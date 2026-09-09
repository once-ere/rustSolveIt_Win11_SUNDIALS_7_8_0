//! `solveit_db serve <db>` — the long-running dataset server.
//!
//! posim spawns one of these per open dataset and speaks line-delimited
//! JSON over stdin/stdout. The server is genuinely multi-threaded: a
//! reader thread accepts requests and worker threads execute them, each
//! worker drawing its own connection from the SHARED r2d2 pool (the
//! pool is cloned into every worker — exactly the architecture the
//! project owner specified). WAL mode means a worker running a long
//! SELECT never blocks the worker that is appending frames, and the
//! 5-second busy timeout absorbs simultaneous write attempts.
//!
//! While the server holds the dataset open, OTHER PROCESSES (the
//! Jupyter notebook's Python `sqlite3` reader, the `sqlite3` CLI) can
//! read the same file concurrently — that is WAL's cross-process
//! guarantee, and the notebooks rely on it.
//!
//! Requests:  {"id":1,"op":"sql","sql":"SELECT ..."}
//!            {"id":2,"op":"append","bodies":[...],"frames":[...]}
//!            {"id":3,"op":"info"}
//!            {"id":4,"op":"checkpoint"}
//!            {"id":5,"op":"close"}
//! Replies:   {"id":1,"ok":true, ...}  or  {"id":1,"ok":false,"error":"..."}

use std::collections::BTreeMap;
use std::io::{BufRead, Write};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};

use crate::db::{self, BodyRow, FrameRow, Pool};
use crate::json::{self, Json};

const WORKERS: usize = 4;

pub fn serve(path: &Path) -> Result<(), String> {
    let pool = if path.is_file() { db::open(path)? } else { db::create(path, false)? };

    let stdout: Arc<Mutex<std::io::Stdout>> = Arc::new(Mutex::new(std::io::stdout()));
    let shutdown = Arc::new(AtomicBool::new(false));
    let (tx, rx) = mpsc::channel::<Json>();
    let rx = Arc::new(Mutex::new(rx));

    let mut workers = Vec::with_capacity(WORKERS);
    for _ in 0..WORKERS {
        /* The pool is cloned into each worker thread: connections are
         * never shared, the POOL is. */
        let pool = pool.clone();
        let stdout = Arc::clone(&stdout);
        let shutdown = Arc::clone(&shutdown);
        let rx = Arc::clone(&rx);
        workers.push(std::thread::spawn(move || loop {
            let job = {
                let guard = rx.lock().expect("request queue lock");
                guard.recv()
            };
            let Ok(req) = job else { break };
            let reply = handle(&pool, &req, &shutdown);
            let line = json::emit(&reply);
            let mut out = stdout.lock().expect("stdout lock");
            let _ = writeln!(out, "{line}");
            let _ = out.flush();
        }));
    }

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.map_err(|e| format!("stdin: {e}"))?;
        if line.trim().is_empty() {
            continue;
        }
        match json::parse(&line) {
            Ok(req) => {
                let is_close = req.get("op").and_then(Json::as_str) == Some("close");
                if tx.send(req).is_err() {
                    break;
                }
                if is_close {
                    break;
                }
            }
            Err(e) => {
                let mut obj = BTreeMap::new();
                obj.insert("ok".to_string(), Json::Bool(false));
                obj.insert("error".to_string(), Json::Str(format!("bad JSON: {e}")));
                let line = json::emit(&Json::Obj(obj));
                let mut out = stdout.lock().expect("stdout lock");
                let _ = writeln!(out, "{line}");
                let _ = out.flush();
            }
        }
        if shutdown.load(Ordering::SeqCst) {
            break;
        }
    }

    /* Drop the sender so idle workers see a closed channel and exit;
     * queued work (including the close reply) still completes. */
    drop(tx);
    for w in workers {
        let _ = w.join();
    }
    /* Leave the file as one tidy .db: flush and truncate the WAL. */
    if let Ok(conn) = pool.get() {
        let _ = db::checkpoint(&conn);
    }
    Ok(())
}

fn handle(pool: &Pool, req: &Json, shutdown: &AtomicBool) -> Json {
    let id = req.get("id").cloned().unwrap_or(Json::Null);
    let op = req.get("op").and_then(Json::as_str).unwrap_or("");
    let body = match op {
        "sql" => op_sql(pool, req),
        "append" => op_append(pool, req),
        "info" => op_info(pool),
        "checkpoint" => pool
            .get()
            .map_err(|e| format!("pool: {e}"))
            .and_then(|c| db::checkpoint(&c))
            .map(|()| BTreeMap::new()),
        "close" => {
            shutdown.store(true, Ordering::SeqCst);
            Ok(BTreeMap::new())
        }
        other => Err(format!("unknown op \"{other}\"")),
    };
    let mut obj = match body {
        Ok(map) => {
            let mut m = map;
            m.insert("ok".to_string(), Json::Bool(true));
            m
        }
        Err(e) => {
            let mut m = BTreeMap::new();
            m.insert("ok".to_string(), Json::Bool(false));
            m.insert("error".to_string(), Json::Str(e));
            m
        }
    };
    obj.insert("id".to_string(), id);
    Json::Obj(obj)
}

fn op_sql(pool: &Pool, req: &Json) -> Result<BTreeMap<String, Json>, String> {
    let sql = req
        .get("sql")
        .and_then(Json::as_str)
        .ok_or_else(|| "sql op needs a \"sql\" string".to_string())?;
    let conn = pool.get().map_err(|e| format!("pool: {e}"))?;
    let r = db::exec_sql(&conn, sql)?;
    let mut m = BTreeMap::new();
    m.insert(
        "columns".to_string(),
        Json::Arr(r.columns.into_iter().map(Json::Str).collect()),
    );
    m.insert(
        "rows".to_string(),
        Json::Arr(r.rows.into_iter().map(Json::Arr).collect()),
    );
    m.insert("changes".to_string(), Json::Int(r.changes as i64));
    Ok(m)
}

fn op_info(pool: &Pool) -> Result<BTreeMap<String, Json>, String> {
    let conn = pool.get().map_err(|e| format!("pool: {e}"))?;
    let i = db::info(&conn)?;
    let mut m = BTreeMap::new();
    m.insert("journal_mode".to_string(), Json::Str(i.journal_mode));
    m.insert("schema_version".to_string(), Json::Str(i.schema_version));
    m.insert("bodies".to_string(), Json::Int(i.bodies));
    m.insert("frames".to_string(), Json::Int(i.frames));
    m.insert("states".to_string(), Json::Int(i.states));
    m.insert("t_first".to_string(), i.t_first.map(Json::Num).unwrap_or(Json::Null));
    m.insert("t_last".to_string(), i.t_last.map(Json::Num).unwrap_or(Json::Null));
    Ok(m)
}

fn op_append(pool: &Pool, req: &Json) -> Result<BTreeMap<String, Json>, String> {
    let bodies = parse_bodies(req.get("bodies").and_then(Json::as_arr).unwrap_or(&[]))?;
    let frames = parse_frames(
        req.get("frames")
            .and_then(Json::as_arr)
            .ok_or_else(|| "append needs a \"frames\" array".to_string())?,
    )?;
    let mut conn = pool.get().map_err(|e| format!("pool: {e}"))?;
    let (first, n) = db::append(&mut conn, &bodies, &frames)?;
    let mut m = BTreeMap::new();
    m.insert("first_frame".to_string(), Json::Int(first));
    m.insert("frames".to_string(), Json::Int(n as i64));
    Ok(m)
}

fn parse_bodies(items: &[Json]) -> Result<Vec<BodyRow>, String> {
    items
        .iter()
        .map(|b| {
            Ok(BodyRow {
                /* posim's hand-rolled JSON emits every number as a
                 * double (`0.0`), so accept any numeric form here. */
                body: b
                    .get("body")
                    .and_then(Json::as_f64)
                    .map(|v| v as i64)
                    .ok_or_else(|| "body needs a numeric \"body\"".to_string())?,
                name: b.get("name").and_then(Json::as_str).unwrap_or("").to_string(),
                shape: b.get("shape").and_then(Json::as_str).unwrap_or("").to_string(),
                mass: b.get("mass").and_then(Json::as_f64).unwrap_or(0.0),
                wall: b.get("wall").and_then(Json::as_f64).unwrap_or(0.0) != 0.0
                    || matches!(b.get("wall"), Some(Json::Bool(true))),
                geometry: b.get("geometry").and_then(Json::as_str).unwrap_or("{}").to_string(),
            })
        })
        .collect()
}

fn vec3(v: Option<&Json>) -> [f64; 3] {
    let mut out = [0.0; 3];
    if let Some(arr) = v.and_then(Json::as_arr) {
        for (i, x) in arr.iter().take(3).enumerate() {
            out[i] = x.as_f64().unwrap_or(0.0);
        }
    }
    out
}

fn parse_frames(items: &[Json]) -> Result<Vec<FrameRow>, String> {
    items
        .iter()
        .map(|f| {
            let states = f
                .get("s")
                .and_then(Json::as_arr)
                .unwrap_or(&[])
                .iter()
                .map(|row| {
                    let arr = row
                        .as_arr()
                        .ok_or_else(|| "state row must be an array".to_string())?;
                    if arr.len() != 14 {
                        return Err(format!(
                            "state row needs 14 numbers (body + 13 state), got {}",
                            arr.len()
                        ));
                    }
                    let mut out = [0.0f64; 14];
                    for (i, x) in arr.iter().enumerate() {
                        out[i] = x
                            .as_f64()
                            .ok_or_else(|| "state row must be numbers".to_string())?;
                    }
                    Ok(out)
                })
                .collect::<Result<Vec<_>, String>>()?;
            Ok(FrameRow {
                t: f.get("t")
                    .and_then(Json::as_f64)
                    .ok_or_else(|| "frame needs a numeric \"t\"".to_string())?,
                energy: f.get("e").and_then(Json::as_f64).unwrap_or(0.0),
                p: vec3(f.get("p")),
                l: vec3(f.get("l")),
                com: vec3(f.get("c")),
                ncollisions: f.get("n").and_then(Json::as_f64).unwrap_or(0.0) as i64,
                states,
            })
        })
        .collect()
}
