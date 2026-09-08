//! The dataset database layer: connection pool, schema, and operations.
//!
//! Architecture, exactly as specified by the project owner:
//!
//! 1. **No raw connection is ever shared across threads.** Every
//!    connection comes out of an [`r2d2::Pool`] built on
//!    [`r2d2_sqlite::SqliteConnectionManager`].
//! 2. **WAL mode is turned on at startup.** The pool's `with_init`
//!    hook runs `PRAGMA journal_mode=WAL` on every connection the
//!    moment it is created, so readers never block the writer and the
//!    writer never blocks readers.
//! 3. **A busy timeout is set.** `PRAGMA busy_timeout = 5000` makes a
//!    connection that finds the database locked wait up to five
//!    seconds for the other writer to finish instead of failing with
//!    "database is locked".
//!
//! The pool is `Clone`; `serve.rs` clones it into worker threads, and
//! the concurrency tests clone it into ten writers at once.

use std::path::{Path, PathBuf};

use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::types::ValueRef;
use rusqlite::Connection;

use crate::json::Json;

pub type Pool = r2d2::Pool<SqliteConnectionManager>;
pub type PooledConn = r2d2::PooledConnection<SqliteConnectionManager>;

/// Run on every pooled connection as it is created. `journal_mode=WAL`
/// is persistent (it is a property of the database file once set), but
/// `busy_timeout` is per-connection, so both belong here.
/// `synchronous=NORMAL` is the documented pairing for WAL: durable
/// against application crashes, one fsync per checkpoint instead of
/// per transaction. `foreign_keys=ON` enforces the frames↔states
/// relationship; SQLite ships with it off for historical reasons.
pub const CONNECTION_PRAGMAS: &str = "PRAGMA journal_mode=WAL;\n\
     PRAGMA busy_timeout = 5000;\n\
     PRAGMA synchronous = NORMAL;\n\
     PRAGMA foreign_keys = ON;";

/// Dataset schema, version 1. Deliberately relational rather than a
/// blob store: `states` joins to `bodies` and `frames`, so plain SQL
/// can answer physics questions (per-body extrema, energy drift,
/// momentum components) without any application code.
///
/// No column anywhere holds a wall-clock time. Byte-stable, replayable
/// output is a property this repository verifies, and a timestamp
/// would silently break it.
pub const SCHEMA: &str = "\
CREATE TABLE IF NOT EXISTS meta(\n\
  key   TEXT PRIMARY KEY,\n\
  value TEXT NOT NULL\n\
) WITHOUT ROWID;\n\
CREATE TABLE IF NOT EXISTS bodies(\n\
  body     INTEGER PRIMARY KEY,\n\
  name     TEXT NOT NULL,\n\
  shape    TEXT NOT NULL,\n\
  mass     REAL NOT NULL,\n\
  wall     INTEGER NOT NULL DEFAULT 0,\n\
  geometry TEXT NOT NULL\n\
);\n\
CREATE TABLE IF NOT EXISTS frames(\n\
  frame       INTEGER PRIMARY KEY,\n\
  t           REAL NOT NULL,\n\
  energy      REAL,\n\
  px REAL, py REAL, pz REAL,\n\
  lx REAL, ly REAL, lz REAL,\n\
  comx REAL, comy REAL, comz REAL,\n\
  ncollisions INTEGER NOT NULL DEFAULT 0\n\
);\n\
CREATE TABLE IF NOT EXISTS states(\n\
  frame INTEGER NOT NULL REFERENCES frames(frame) ON DELETE CASCADE,\n\
  body  INTEGER NOT NULL REFERENCES bodies(body),\n\
  x REAL, y REAL, z REAL,\n\
  qw REAL, qx REAL, qy REAL, qz REAL,\n\
  vx REAL, vy REAL, vz REAL,\n\
  wx REAL, wy REAL, wz REAL,\n\
  PRIMARY KEY(frame, body)\n\
) WITHOUT ROWID;\n\
CREATE INDEX IF NOT EXISTS idx_states_body ON states(body, frame);\n";

pub const SCHEMA_VERSION: &str = "1";

/// Builds the thread-safe pool for `path`. Every connection the pool
/// hands out has already executed [`CONNECTION_PRAGMAS`] and carries
/// the math scalar functions physics queries want (`sqrt`, `pow`,
/// `hypot`) — the bundled SQLite amalgamation is compiled without
/// SQLITE_ENABLE_MATH_FUNCTIONS, so the sidecar registers its own,
/// marked DETERMINISTIC so the query planner may fold them.
pub fn make_pool(path: &Path, max_size: u32) -> Result<Pool, String> {
    use rusqlite::functions::FunctionFlags;
    let flags = FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC;
    let manager = SqliteConnectionManager::file(path).with_init(move |conn| {
        conn.execute_batch(CONNECTION_PRAGMAS)?;
        conn.create_scalar_function("sqrt", 1, flags, |ctx| {
            let x: Option<f64> = ctx.get(0)?;
            Ok(x.map(f64::sqrt))
        })?;
        conn.create_scalar_function("pow", 2, flags, |ctx| {
            let x: Option<f64> = ctx.get(0)?;
            let y: Option<f64> = ctx.get(1)?;
            Ok(match (x, y) {
                (Some(x), Some(y)) => Some(x.powf(y)),
                _ => None,
            })
        })?;
        conn.create_scalar_function("hypot", 2, flags, |ctx| {
            let x: Option<f64> = ctx.get(0)?;
            let y: Option<f64> = ctx.get(1)?;
            Ok(match (x, y) {
                (Some(x), Some(y)) => Some(x.hypot(y)),
                _ => None,
            })
        })?;
        Ok(())
    });
    r2d2::Pool::builder()
        .max_size(max_size)
        .build(manager)
        .map_err(|e| format!("cannot open pool on {}: {e}", path.display()))
}

/// Creates a new dataset database at `path` (parent directories
/// included), initializing the schema and stamping the schema version.
/// Refuses to touch an existing file unless `force` is set.
pub fn create(path: &Path, force: bool) -> Result<Pool, String> {
    if path.exists() {
        if !force {
            return Err(format!(
                "dataset already exists: {} (use force/overwrite to replace it)",
                path.display()
            ));
        }
        delete(path)?;
    }
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("cannot create {}: {e}", parent.display()))?;
        }
    }
    let pool = make_pool(path, 10)?;
    let conn = pool.get().map_err(|e| format!("pool: {e}"))?;
    conn.execute_batch(SCHEMA).map_err(|e| format!("schema: {e}"))?;
    conn.execute(
        "INSERT OR REPLACE INTO meta(key, value) VALUES ('schema_version', ?1)",
        rusqlite::params![SCHEMA_VERSION],
    )
    .map_err(|e| format!("schema version: {e}"))?;
    Ok(pool)
}

/// Opens an existing dataset, verifying it looks like one.
pub fn open(path: &Path) -> Result<Pool, String> {
    if !path.is_file() {
        return Err(format!("no such dataset: {}", path.display()));
    }
    let pool = make_pool(path, 10)?;
    {
        let conn = pool.get().map_err(|e| format!("pool: {e}"))?;
        let n: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN \
                 ('meta','bodies','frames','states')",
                [],
                |r| r.get(0),
            )
            .map_err(|e| format!("not a dataset ({}): {e}", path.display()))?;
        if n != 4 {
            return Err(format!(
                "not a solveit dataset (found {n} of the 4 schema tables): {}",
                path.display()
            ));
        }
    }
    Ok(pool)
}

/// The result of running SQL: the last row-returning statement's
/// columns and rows, plus the total number of rows changed by every
/// non-SELECT statement.
pub struct SqlResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Json>>,
    pub changes: u64,
}

/// Executes arbitrary SQL — one statement or many, separated by `;`.
/// This is the passthrough that gives the notebook language the
/// COMPLETE SQLite command set: DDL, DML, queries, PRAGMA, ATTACH,
/// VACUUM, window functions, CTEs — whatever SQLite accepts.
pub fn exec_sql(conn: &Connection, sql: &str) -> Result<SqlResult, String> {
    let mut result = SqlResult { columns: Vec::new(), rows: Vec::new(), changes: 0 };
    let mut batch = rusqlite::Batch::new(conn, sql);
    loop {
        match batch.next() {
            Ok(Some(mut stmt)) => {
                if stmt.column_count() > 0 {
                    let columns: Vec<String> =
                        stmt.column_names().iter().map(|s| s.to_string()).collect();
                    let ncols = columns.len();
                    let mut rows_out: Vec<Vec<Json>> = Vec::new();
                    let mut rows = stmt.query([]).map_err(|e| format!("SQL: {e}"))?;
                    while let Some(row) = rows.next().map_err(|e| format!("SQL: {e}"))? {
                        let mut out = Vec::with_capacity(ncols);
                        for i in 0..ncols {
                            let v = row.get_ref(i).map_err(|e| format!("SQL: {e}"))?;
                            out.push(value_ref_to_json(v));
                        }
                        rows_out.push(out);
                    }
                    result.columns = columns;
                    result.rows = rows_out;
                } else {
                    /* `conn.changes()` is stale after a DDL statement
                     * (it still reports the previous DML's count), so
                     * measure each statement as a delta of the
                     * monotonic total instead. */
                    let before = conn.total_changes();
                    stmt.raw_execute().map_err(|e| format!("SQL: {e}"))?;
                    result.changes += conn.total_changes().saturating_sub(before);
                }
            }
            Ok(None) => break,
            Err(e) => return Err(format!("SQL: {e}")),
        }
    }
    Ok(result)
}

fn value_ref_to_json(v: ValueRef<'_>) -> Json {
    match v {
        ValueRef::Null => Json::Null,
        ValueRef::Integer(i) => Json::Int(i),
        ValueRef::Real(f) => Json::Num(f),
        ValueRef::Text(t) => Json::Str(String::from_utf8_lossy(t).into_owned()),
        ValueRef::Blob(b) => {
            /* Datasets store no blobs; render one legibly if user SQL
             * makes one anyway. */
            let hex: String = b.iter().map(|x| format!("{x:02x}")).collect();
            Json::Str(format!("x'{hex}'"))
        }
    }
}

/// One body row for `append`.
pub struct BodyRow {
    pub body: i64,
    pub name: String,
    pub shape: String,
    pub mass: f64,
    pub wall: bool,
    pub geometry: String,
}

/// One frame plus the per-body states captured at that instant.
/// `states` entries are `[body, x,y,z, qw,qx,qy,qz, vx,vy,vz, wx,wy,wz]`.
pub struct FrameRow {
    pub t: f64,
    pub energy: f64,
    pub p: [f64; 3],
    pub l: [f64; 3],
    pub com: [f64; 3],
    pub ncollisions: i64,
    pub states: Vec<[f64; 14]>,
}

/// Appends a batch of frames in ONE transaction: frame numbers are
/// allocated from `MAX(frame)+1` inside the transaction, so concurrent
/// appenders (or a crash mid-batch) can never interleave or leave a
/// torn batch. Bodies are upserted first — the bodies table always
/// reflects every body that ever appeared.
pub fn append(
    conn: &mut Connection,
    bodies: &[BodyRow],
    frames: &[FrameRow],
) -> Result<(i64, usize), String> {
    let tx = conn.transaction().map_err(|e| format!("append: {e}"))?;
    {
        let mut upsert = tx
            .prepare(
                "INSERT OR REPLACE INTO bodies(body, name, shape, mass, wall, geometry) \
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            )
            .map_err(|e| format!("append: {e}"))?;
        for b in bodies {
            upsert
                .execute(rusqlite::params![
                    b.body,
                    b.name,
                    b.shape,
                    b.mass,
                    b.wall as i64,
                    b.geometry
                ])
                .map_err(|e| format!("append: {e}"))?;
        }
    }
    let first: i64 = tx
        .query_row("SELECT COALESCE(MAX(frame) + 1, 0) FROM frames", [], |r| r.get(0))
        .map_err(|e| format!("append: {e}"))?;
    {
        let mut ins_frame = tx
            .prepare(
                "INSERT INTO frames(frame, t, energy, px, py, pz, lx, ly, lz, \
                 comx, comy, comz, ncollisions) \
                 VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13)",
            )
            .map_err(|e| format!("append: {e}"))?;
        let mut ins_state = tx
            .prepare(
                "INSERT INTO states(frame, body, x, y, z, qw, qx, qy, qz, \
                 vx, vy, vz, wx, wy, wz) \
                 VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15)",
            )
            .map_err(|e| format!("append: {e}"))?;
        for (k, f) in frames.iter().enumerate() {
            let frame = first + k as i64;
            ins_frame
                .execute(rusqlite::params![
                    frame,
                    f.t,
                    f.energy,
                    f.p[0],
                    f.p[1],
                    f.p[2],
                    f.l[0],
                    f.l[1],
                    f.l[2],
                    f.com[0],
                    f.com[1],
                    f.com[2],
                    f.ncollisions
                ])
                .map_err(|e| format!("append: {e}"))?;
            for s in &f.states {
                ins_state
                    .execute(rusqlite::params![
                        frame, s[0] as i64, s[1], s[2], s[3], s[4], s[5], s[6], s[7], s[8],
                        s[9], s[10], s[11], s[12], s[13]
                    ])
                    .map_err(|e| format!("append: {e}"))?;
            }
        }
    }
    tx.commit().map_err(|e| format!("append: {e}"))?;
    Ok((first, frames.len()))
}

/// Copies a dataset with `VACUUM INTO`: a compact, checkpointed,
/// single-file snapshot, safe to take while the source is open in WAL
/// mode.
pub fn copy(src: &Path, dst: &Path) -> Result<(), String> {
    if dst.exists() {
        return Err(format!("destination already exists: {}", dst.display()));
    }
    if let Some(parent) = dst.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("cannot create {}: {e}", parent.display()))?;
        }
    }
    let pool = open(src)?;
    let conn = pool.get().map_err(|e| format!("pool: {e}"))?;
    let dst_text = dst.to_string_lossy().replace('\'', "''");
    conn.execute_batch(&format!("VACUUM INTO '{dst_text}'"))
        .map_err(|e| format!("copy: {e}"))
}

/// Removes a dataset and its WAL sidecar files.
pub fn delete(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("no such dataset: {}", path.display()));
    }
    std::fs::remove_file(path).map_err(|e| format!("delete {}: {e}", path.display()))?;
    for suffix in ["-wal", "-shm"] {
        let mut side = path.as_os_str().to_owned();
        side.push(suffix);
        let side = PathBuf::from(side);
        if side.exists() {
            std::fs::remove_file(&side)
                .map_err(|e| format!("delete {}: {e}", side.display()))?;
        }
    }
    Ok(())
}

/// Flushes the WAL into the main database file and truncates it.
pub fn checkpoint(conn: &Connection) -> Result<(), String> {
    conn.execute_batch("PRAGMA wal_checkpoint(TRUNCATE);")
        .map_err(|e| format!("checkpoint: {e}"))
}

/// Counts for `info`: deterministic, no timestamps.
pub struct Info {
    pub journal_mode: String,
    pub schema_version: String,
    pub bodies: i64,
    pub frames: i64,
    pub states: i64,
    pub t_first: Option<f64>,
    pub t_last: Option<f64>,
}

pub fn info(conn: &Connection) -> Result<Info, String> {
    let journal_mode: String = conn
        .query_row("PRAGMA journal_mode;", [], |r| r.get(0))
        .map_err(|e| format!("info: {e}"))?;
    let schema_version: String = conn
        .query_row("SELECT value FROM meta WHERE key = 'schema_version'", [], |r| r.get(0))
        .unwrap_or_else(|_| "?".to_string());
    let count = |table: &str| -> Result<i64, String> {
        conn.query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |r| r.get(0))
            .map_err(|e| format!("info: {e}"))
    };
    let (t_first, t_last): (Option<f64>, Option<f64>) = conn
        .query_row("SELECT MIN(t), MAX(t) FROM frames", [], |r| {
            Ok((r.get(0)?, r.get(1)?))
        })
        .map_err(|e| format!("info: {e}"))?;
    Ok(Info {
        journal_mode,
        schema_version,
        bodies: count("bodies")?,
        frames: count("frames")?,
        states: count("states")?,
        t_first,
        t_last,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_db(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join("solveit_db_unit");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join(name);
        let _ = std::fs::remove_file(&path);
        let mut wal = path.as_os_str().to_owned();
        wal.push("-wal");
        let _ = std::fs::remove_file(PathBuf::from(wal));
        path
    }

    #[test]
    fn create_sets_wal_mode_and_busy_timeout() {
        let path = temp_db("wal_mode.db");
        let pool = create(&path, false).unwrap();
        let conn = pool.get().unwrap();
        let mode: String =
            conn.query_row("PRAGMA journal_mode;", [], |r| r.get(0)).unwrap();
        assert_eq!(mode.to_lowercase(), "wal");
        let timeout: i64 =
            conn.query_row("PRAGMA busy_timeout;", [], |r| r.get(0)).unwrap();
        assert_eq!(timeout, 5000);
    }

    #[test]
    fn append_allocates_contiguous_frames_and_info_counts_them() {
        let path = temp_db("append.db");
        let pool = create(&path, false).unwrap();
        let mut conn = pool.get().unwrap();
        let bodies = vec![BodyRow {
            body: 0,
            name: "ball".into(),
            shape: "sphere".into(),
            mass: 2.0,
            wall: false,
            geometry: "{\"radius\":0.5}".into(),
        }];
        let frame = |t: f64| FrameRow {
            t,
            energy: -1.0,
            p: [0.0; 3],
            l: [0.0; 3],
            com: [0.0; 3],
            ncollisions: 0,
            states: vec![[0.0, t, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]],
        };
        let (first, n) = append(&mut conn, &bodies, &[frame(0.0), frame(0.5)]).unwrap();
        assert_eq!((first, n), (0, 2));
        let (first, n) = append(&mut conn, &bodies, &[frame(1.0)]).unwrap();
        assert_eq!((first, n), (2, 1));
        let i = info(&conn).unwrap();
        assert_eq!((i.bodies, i.frames, i.states), (1, 3, 3));
        assert_eq!((i.t_first, i.t_last), (Some(0.0), Some(1.0)));
    }

    #[test]
    fn exec_sql_runs_the_full_sqlite_surface() {
        let path = temp_db("exec.db");
        let pool = create(&path, false).unwrap();
        let conn = pool.get().unwrap();
        /* DDL + DML + query in one passthrough string. */
        let r = exec_sql(
            &conn,
            "CREATE TABLE extra(x REAL); INSERT INTO extra VALUES (1.5), (2.5); \
             SELECT COUNT(*) AS n, SUM(x) AS s FROM extra;",
        )
        .unwrap();
        assert_eq!(r.columns, vec!["n", "s"]);
        assert_eq!(r.rows, vec![vec![Json::Int(2), Json::Num(4.0)]]);
        assert_eq!(r.changes, 2);
        /* A window function — SQLite's modern SQL is all reachable. */
        let r = exec_sql(
            &conn,
            "SELECT x - LAG(x) OVER (ORDER BY x) AS dx FROM extra ORDER BY x;",
        )
        .unwrap();
        assert_eq!(r.rows, vec![vec![Json::Null], vec![Json::Num(1.0)]]);
        /* An error surfaces as Err, not a panic. */
        assert!(exec_sql(&conn, "SELECT * FROM nonexistent").is_err());
    }

    #[test]
    fn copy_and_delete_round_trip() {
        let src = temp_db("copy_src.db");
        let dst = temp_db("copy_dst.db");
        let _ = std::fs::remove_file(&dst);
        let pool = create(&src, false).unwrap();
        {
            let conn = pool.get().unwrap();
            exec_sql(&conn, "INSERT INTO meta(key, value) VALUES ('title', 'x')").unwrap();
        }
        copy(&src, &dst).unwrap();
        let pool2 = open(&dst).unwrap();
        let conn2 = pool2.get().unwrap();
        let title: String = conn2
            .query_row("SELECT value FROM meta WHERE key='title'", [], |r| r.get(0))
            .unwrap();
        assert_eq!(title, "x");
        drop(conn2);
        drop(pool2);
        delete(&dst).unwrap();
        assert!(!dst.exists());
    }
}
