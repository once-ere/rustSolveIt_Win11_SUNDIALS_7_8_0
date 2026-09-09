//! `solveit_db` — the SQLite dataset sidecar binary for rustSolveIt.
//!
//! The simulator (`posim`) spawns this binary; it never links SQLite
//! itself. See Cargo.toml for why this package lives outside the
//! pure-Rust simulator workspace.
//!
//! Usage:
//!   solveit_db create <db> [--force]        create a dataset (schema v1)
//!   solveit_db exec <db> <sql...>           run any SQLite statement(s)
//!   solveit_db query <db> <sql...>          run SQL, print JSON result
//!   solveit_db info <db>                    row counts + journal mode
//!   solveit_db list <dir>                   list *.db datasets in a folder
//!   solveit_db copy <src> <dst>             VACUUM INTO snapshot copy
//!   solveit_db delete <db>                  remove db + -wal + -shm
//!   solveit_db checkpoint <db>              flush WAL into the main file
//!   solveit_db serve <db>                   JSON-over-stdio server (posim)
//!   solveit_db stress <db> <threads> <rows> multithreaded pool/WAL demo

#![deny(warnings)]

use std::path::{Path, PathBuf};

use solveit_db::json::Json;
use solveit_db::{db, json, serve};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = match run(&args) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("solveit_db: {e}");
            1
        }
    };
    std::process::exit(code);
}

fn run(args: &[String]) -> Result<(), String> {
    let cmd = args.first().map(String::as_str).unwrap_or("");
    match cmd {
        "create" => {
            let path = want_path(args, 1, "create <db> [--force]")?;
            let force = args.iter().any(|a| a == "--force");
            db::create(&path, force)?;
            println!("created {} (journal_mode=wal, schema v{})", path.display(), db::SCHEMA_VERSION);
            Ok(())
        }
        "exec" => {
            let path = want_path(args, 1, "exec <db> <sql...>")?;
            let sql = rest_sql(args, 2, "exec <db> <sql...>")?;
            let pool = db::open(&path)?;
            let conn = pool.get().map_err(|e| format!("pool: {e}"))?;
            let r = db::exec_sql(&conn, &sql)?;
            if r.columns.is_empty() {
                println!("changes: {}", r.changes);
            } else {
                println!("{}", r.columns.join(" | "));
                for row in &r.rows {
                    let cells: Vec<String> = row.iter().map(cell_text).collect();
                    println!("{}", cells.join(" | "));
                }
            }
            Ok(())
        }
        "query" => {
            let path = want_path(args, 1, "query <db> <sql...>")?;
            let sql = rest_sql(args, 2, "query <db> <sql...>")?;
            let pool = db::open(&path)?;
            let conn = pool.get().map_err(|e| format!("pool: {e}"))?;
            let r = db::exec_sql(&conn, &sql)?;
            let mut obj = std::collections::BTreeMap::new();
            obj.insert(
                "columns".to_string(),
                Json::Arr(r.columns.into_iter().map(Json::Str).collect()),
            );
            obj.insert("rows".to_string(), Json::Arr(r.rows.into_iter().map(Json::Arr).collect()));
            obj.insert("changes".to_string(), Json::Int(r.changes as i64));
            println!("{}", json::emit(&Json::Obj(obj)));
            Ok(())
        }
        "info" => {
            let path = want_path(args, 1, "info <db>")?;
            let pool = db::open(&path)?;
            let conn = pool.get().map_err(|e| format!("pool: {e}"))?;
            let i = db::info(&conn)?;
            println!("journal_mode   = {}", i.journal_mode);
            println!("schema_version = {}", i.schema_version);
            println!("bodies         = {}", i.bodies);
            println!("frames         = {}", i.frames);
            println!("states         = {}", i.states);
            match (i.t_first, i.t_last) {
                (Some(a), Some(b)) => println!("t range        = [{a:?}, {b:?}]"),
                _ => println!("t range        = (empty)"),
            }
            Ok(())
        }
        "list" => {
            let dir = want_path(args, 1, "list <dir>")?;
            let mut names: Vec<String> = std::fs::read_dir(&dir)
                .map_err(|e| format!("list {}: {e}", dir.display()))?
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| {
                    let p = entry.path();
                    if p.extension().and_then(|s| s.to_str()) == Some("db") {
                        p.file_stem().and_then(|s| s.to_str()).map(String::from)
                    } else {
                        None
                    }
                })
                .collect();
            names.sort();
            for n in names {
                println!("{n}");
            }
            Ok(())
        }
        "copy" => {
            let src = want_path(args, 1, "copy <src> <dst>")?;
            let dst = want_path(args, 2, "copy <src> <dst>")?;
            db::copy(&src, &dst)?;
            println!("copied {} -> {}", src.display(), dst.display());
            Ok(())
        }
        "delete" => {
            let path = want_path(args, 1, "delete <db>")?;
            db::delete(&path)?;
            println!("deleted {}", path.display());
            Ok(())
        }
        "checkpoint" => {
            let path = want_path(args, 1, "checkpoint <db>")?;
            let pool = db::open(&path)?;
            let conn = pool.get().map_err(|e| format!("pool: {e}"))?;
            db::checkpoint(&conn)?;
            println!("checkpointed {}", path.display());
            Ok(())
        }
        "serve" => {
            let path = want_path(args, 1, "serve <db>")?;
            serve::serve(&path)
        }
        "stress" => {
            let path = want_path(args, 1, "stress <db> <threads> <rows>")?;
            let threads: usize = want_num(args, 2, "stress <db> <threads> <rows>")?;
            let rows: usize = want_num(args, 3, "stress <db> <threads> <rows>")?;
            stress(&path, threads, rows)
        }
        "" | "help" | "--help" | "-h" => {
            print!("{}", USAGE);
            Ok(())
        }
        other => Err(format!("unknown command \"{other}\"\n{USAGE}")),
    }
}

const USAGE: &str = "usage:\n  solveit_db create <db> [--force]\n  solveit_db exec <db> <sql...>\n  solveit_db query <db> <sql...>\n  solveit_db info <db>\n  solveit_db list <dir>\n  solveit_db copy <src> <dst>\n  solveit_db delete <db>\n  solveit_db checkpoint <db>\n  solveit_db serve <db>\n  solveit_db stress <db> <threads> <rows>\n";

fn want_path(args: &[String], i: usize, usage: &str) -> Result<PathBuf, String> {
    args.get(i)
        .filter(|a| !a.starts_with("--"))
        .map(PathBuf::from)
        .ok_or_else(|| format!("usage: solveit_db {usage}"))
}

fn want_num(args: &[String], i: usize, usage: &str) -> Result<usize, String> {
    args.get(i)
        .and_then(|a| a.parse().ok())
        .ok_or_else(|| format!("usage: solveit_db {usage}"))
}

fn rest_sql(args: &[String], from: usize, usage: &str) -> Result<String, String> {
    if args.len() <= from {
        return Err(format!("usage: solveit_db {usage}"));
    }
    Ok(args[from..].join(" "))
}

fn cell_text(v: &Json) -> String {
    match v {
        Json::Null => "NULL".to_string(),
        Json::Bool(b) => b.to_string(),
        Json::Int(i) => i.to_string(),
        Json::Num(n) => format!("{n:?}"),
        Json::Str(s) => s.clone(),
        other => json::emit(other),
    }
}

/// The multithreaded demonstration the project owner asked for: N
/// threads, each holding pooled connections, all appending to the SAME
/// database file at once. WAL + busy_timeout means every insert lands;
/// the final count proves it.
fn stress(path: &Path, threads: usize, rows: usize) -> Result<(), String> {
    let pool = if path.is_file() { db::open(path)? } else { db::create(path, false)? };
    {
        let conn = pool.get().map_err(|e| format!("pool: {e}"))?;
        db::exec_sql(
            &conn,
            "CREATE TABLE IF NOT EXISTS stress(thread INTEGER, k INTEGER, x REAL); \
             DELETE FROM stress;",
        )?;
    }
    let mut handles = Vec::with_capacity(threads);
    for tid in 0..threads {
        let pool = pool.clone(); // the POOL crosses the thread, never a connection
        handles.push(std::thread::spawn(move || -> Result<(), String> {
            for k in 0..rows {
                let conn = pool.get().map_err(|e| format!("pool: {e}"))?;
                conn.execute(
                    "INSERT INTO stress(thread, k, x) VALUES (?1, ?2, ?3)",
                    rusqlite::params![tid as i64, k as i64, (tid * rows + k) as f64],
                )
                .map_err(|e| format!("insert (thread {tid}, row {k}): {e}"))?;
            }
            Ok(())
        }));
    }
    let mut failures = Vec::new();
    for (tid, h) in handles.into_iter().enumerate() {
        match h.join() {
            Ok(Ok(())) => {}
            Ok(Err(e)) => failures.push(e),
            Err(_) => failures.push(format!("thread {tid} panicked")),
        }
    }
    if !failures.is_empty() {
        return Err(failures.join("; "));
    }
    let conn = pool.get().map_err(|e| format!("pool: {e}"))?;
    let n: i64 = conn
        .query_row("SELECT COUNT(*) FROM stress", [], |r| r.get(0))
        .map_err(|e| format!("count: {e}"))?;
    let mode: String = conn
        .query_row("PRAGMA journal_mode;", [], |r| r.get(0))
        .map_err(|e| format!("pragma: {e}"))?;
    println!(
        "stress: {threads} threads x {rows} rows = {} expected, {n} written, journal_mode={mode}",
        threads * rows
    );
    if n as usize != threads * rows {
        return Err(format!("row loss: expected {} got {n}", threads * rows));
    }
    Ok(())
}
