//! Concurrency tests: the reason WAL mode + busy_timeout + the r2d2
//! pool exist. Ten writer threads share one pool; two separate pools
//! on the same file emulate two PROCESSES (posim's serve child and a
//! notebook's Python reader); and the serve binary is driven over its
//! real stdio protocol.

use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use solveit_db::db;

fn temp_db(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join("solveit_db_it");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join(name);
    for suffix in ["", "-wal", "-shm"] {
        let mut p = path.as_os_str().to_owned();
        p.push(suffix);
        let _ = std::fs::remove_file(PathBuf::from(p));
    }
    path
}

/// Ten threads, one shared pool, simultaneous inserts. With WAL and a
/// 5 s busy timeout every row must land; without them SQLite would
/// throw "database is locked" under this load.
#[test]
fn ten_threads_through_one_pool_lose_nothing() {
    let path = temp_db("ten_threads.db");
    let pool = db::create(&path, false).unwrap();
    {
        let conn = pool.get().unwrap();
        db::exec_sql(&conn, "CREATE TABLE hits(thread INTEGER, k INTEGER)").unwrap();
    }
    const THREADS: usize = 10;
    const ROWS: usize = 200;
    let mut handles = Vec::new();
    for tid in 0..THREADS {
        let pool = pool.clone();
        handles.push(std::thread::spawn(move || {
            for k in 0..ROWS {
                let conn = pool.get().expect("pool connection");
                conn.execute(
                    "INSERT INTO hits(thread, k) VALUES (?1, ?2)",
                    rusqlite::params![tid as i64, k as i64],
                )
                .expect("insert must not hit `database is locked`");
            }
        }));
    }
    for h in handles {
        h.join().expect("no thread may panic");
    }
    let conn = pool.get().unwrap();
    let n: i64 = conn.query_row("SELECT COUNT(*) FROM hits", [], |r| r.get(0)).unwrap();
    assert_eq!(n as usize, THREADS * ROWS);
    let mode: String = conn.query_row("PRAGMA journal_mode;", [], |r| r.get(0)).unwrap();
    assert_eq!(mode.to_lowercase(), "wal");
}

/// Two pools on one file — the cross-process picture. Writers on both
/// pools interleave while a reader on pool B watches the count grow.
/// WAL is exactly the mode in which the reader never blocks either
/// writer.
#[test]
fn two_pools_on_one_file_interleave_writes_and_reads() {
    let path = temp_db("two_pools.db");
    let pool_a = db::create(&path, false).unwrap();
    let pool_b = db::open(&path).unwrap();
    {
        let conn = pool_a.get().unwrap();
        db::exec_sql(&conn, "CREATE TABLE hits(side TEXT, k INTEGER)").unwrap();
    }
    const ROWS: usize = 300;
    let writer = |pool: db::Pool, side: &'static str| {
        std::thread::spawn(move || {
            for k in 0..ROWS {
                let conn = pool.get().expect("pool connection");
                conn.execute(
                    "INSERT INTO hits(side, k) VALUES (?1, ?2)",
                    rusqlite::params![side, k as i64],
                )
                .expect("cross-pool insert must survive contention");
            }
        })
    };
    let wa = writer(pool_a.clone(), "a");
    let wb = writer(pool_b.clone(), "b");
    /* Concurrent reader: must never error, and counts only grow. */
    let reader = {
        let pool = pool_b.clone();
        std::thread::spawn(move || {
            let mut last = 0i64;
            for _ in 0..50 {
                let conn = pool.get().expect("reader connection");
                let n: i64 =
                    conn.query_row("SELECT COUNT(*) FROM hits", [], |r| r.get(0)).unwrap();
                assert!(n >= last, "count went backwards: {n} < {last}");
                last = n;
                std::thread::yield_now();
            }
        })
    };
    wa.join().unwrap();
    wb.join().unwrap();
    reader.join().unwrap();
    let conn = pool_a.get().unwrap();
    let n: i64 = conn.query_row("SELECT COUNT(*) FROM hits", [], |r| r.get(0)).unwrap();
    assert_eq!(n as usize, 2 * ROWS);
}

/// Drives the real `solveit_db serve` binary over stdin/stdout exactly
/// as posim does: create via serve, append two frame batches, query
/// them back, close. Uses the binary cargo built for this test run.
#[test]
fn serve_protocol_round_trips_append_and_sql() {
    let path = temp_db("serve_rt.db");
    let exe = env!("CARGO_BIN_EXE_solveit_db");
    let mut child = Command::new(exe)
        .arg("serve")
        .arg(&path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn solveit_db serve");
    let mut stdin = child.stdin.take().unwrap();
    let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();
    let mut rpc = |req: &str| -> String {
        writeln!(stdin, "{req}").unwrap();
        stdin.flush().unwrap();
        lines.next().expect("a reply line").expect("readable reply")
    };

    let r = rpc(r#"{"id":1,"op":"append","bodies":[{"body":0,"name":"ball","shape":"sphere","mass":2.0,"wall":false,"geometry":"{\"radius\":0.5}"}],"frames":[{"t":0.0,"e":-1.0,"p":[0,0,0],"l":[0,0,0],"c":[0,0,0],"n":0,"s":[[0,0,0,0,1,0,0,0,0,0,0,0,0,0]]}]}"#);
    assert!(r.contains("\"ok\":true"), "append 1: {r}");
    assert!(r.contains("\"first_frame\":0"), "append 1: {r}");

    let r = rpc(r#"{"id":2,"op":"append","frames":[{"t":0.5,"e":-1.0,"p":[0,0,0],"l":[0,0,0],"c":[0,0,0],"n":0,"s":[[0,0.5,0,0,1,0,0,0,0,0,0,0,0,0]]}]}"#);
    assert!(r.contains("\"first_frame\":1"), "append 2: {r}");

    let r = rpc(r#"{"id":3,"op":"sql","sql":"SELECT COUNT(*) AS n FROM frames"}"#);
    assert!(r.contains("\"rows\":[[2]]"), "count: {r}");

    let r = rpc(r#"{"id":4,"op":"sql","sql":"SELECT b.name, s.x FROM states s JOIN bodies b ON b.body = s.body ORDER BY s.frame"}"#);
    assert!(r.contains("\"rows\":[[\"ball\",0.0],[\"ball\",0.5]]"), "join: {r}");

    let r = rpc(r#"{"id":5,"op":"sql","sql":"SELECT * FROM missing_table"}"#);
    assert!(r.contains("\"ok\":false"), "error path: {r}");

    let r = rpc(r#"{"id":6,"op":"close"}"#);
    assert!(r.contains("\"ok\":true"), "close: {r}");
    drop(stdin);
    let status = child.wait().expect("serve exit");
    assert!(status.success(), "serve exited with {status:?}");

    /* After close the WAL has been checkpointed: the .db alone holds
     * both frames (a fresh reader sees them with no -wal file). */
    let pool = db::open(&path).unwrap();
    let conn = pool.get().unwrap();
    let n: i64 = conn.query_row("SELECT COUNT(*) FROM frames", [], |r| r.get(0)).unwrap();
    assert_eq!(n, 2);
}
