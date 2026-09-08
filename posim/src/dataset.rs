//! Datasets: every simulation can record itself into a SQLite database
//! under `datasets/<name>.db`, and every Jupyter notebook in this
//! repository creates one.
//!
//! posim itself contains NO SQLite code — the pure-Rust workspace keeps
//! its zero-external-dependency guarantee. All database work happens in
//! the `solveit_db` sidecar binary (see `dataset_tools/`), which posim
//! spawns and speaks to in line-delimited JSON. The sidecar follows the
//! specified concurrency architecture: an r2d2 connection pool shared
//! across threads, WAL journal mode enabled at startup, and a 5-second
//! busy timeout — so posim can keep appending frames while a notebook's
//! Python `sqlite3` reader watches the same file.
//!
//! The command family this module implements (see the parser's EBNF):
//!
//!   DATASET CREATE <name>     new dataset (replaces any existing one),
//!                             opened with recording armed
//!   DATASET OPEN <name>       open an existing dataset, recording armed
//!   DATASET CLOSE             checkpoint the WAL and close
//!   DATASET RECORD [ON|OFF]   arm/disarm auto-recording; bare = status
//!   DATASET SNAPSHOT          record the current state as one frame
//!   DATASET LIST              list datasets/
//!   DATASET INFO [<name>]     row counts (open dataset if no name)
//!   DATASET COPY <src> <dst>  snapshot-copy a dataset
//!   DATASET SAVEAS <name>     snapshot the OPEN dataset under a new name
//!   DATASET DELETE <name>     remove a dataset
//!   DATASET PLAY [<name>]     plot the dataset in a browser window
//!   SQL "<statements>"        run ANY SQLite statement(s) on the open
//!                             dataset — the full SQLite grammar
//!
//! `RESET` closes the open dataset (the world it was recording is gone).

use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use crate::machine::{self, Json};
use crate::vm::SimState;
use ::physical_object::boundary::Boundary;
use ::physical_object::integrate::Snapshot;

/// Where datasets live, relative to the working directory (exactly like
/// `videos/` and `scripts/`).
pub const DATASET_DIR: &str = "datasets";

/// Every DATASET subcommand the parser implements. The lockstep test
/// below checks each is documented in the EBNF, HELP_TEXT, grammar.md
/// and grammar.tex — and, conversely, that the EBNF promises nothing
/// the parser lacks (the QM2 phantom-`ISO` lesson).
pub const DATASET_SUBCOMMANDS: &[&str] = &[
    "create", "open", "close", "record", "snapshot", "list", "info", "copy", "saveas",
    "delete", "play",
];

/// Every MOVIE subcommand the parser implements.
pub const MOVIE_SUBCOMMANDS: &[&str] = &["create", "delete", "list", "play"];

/// Dataset and movie names are file stems: letters, digits, `_`, `-`.
pub fn valid_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("the name must not be empty".to_string());
    }
    if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
        return Err(format!(
            "invalid name `{name}`: use letters, digits, `_` and `-` only"
        ));
    }
    Ok(())
}

pub fn db_path(name: &str) -> PathBuf {
    Path::new(DATASET_DIR).join(format!("{name}.db"))
}

/// Locates the `solveit_db` sidecar binary: `$POSIM_DB_TOOL`, then
/// `dataset_tools/target/release/solveit_db` from the working directory
/// upward, then `$PATH`. The error tells the user exactly how to build
/// it — the sidecar is a separate package precisely so the simulator
/// workspace stays dependency-free.
pub fn find_db_tool() -> Result<PathBuf, String> {
    if let Some(p) = std::env::var_os("POSIM_DB_TOOL") {
        let p = PathBuf::from(p);
        if p.is_file() {
            return Ok(p);
        }
        return Err(format!("POSIM_DB_TOOL is set but not a file: {}", p.display()));
    }
    if let Ok(mut dir) = std::env::current_dir() {
        loop {
            let cand = dir.join("dataset_tools/target/release/solveit_db");
            if cand.is_file() {
                return Ok(cand);
            }
            if !dir.pop() {
                break;
            }
        }
    }
    if let Some(paths) = std::env::var_os("PATH") {
        for dir in std::env::split_paths(&paths) {
            let cand = dir.join("solveit_db");
            if cand.is_file() {
                return Ok(cand);
            }
        }
    }
    Err("the dataset sidecar `solveit_db` is not built.\n\
         Build it once, from the repository root:\n\
             cd dataset_tools && cargo build --release\n\
         or set POSIM_DB_TOOL to the full path of the binary."
        .to_string())
}

/// Opens `path` in the default browser unless `$POSIM_NO_BROWSER` is
/// set (the same suppression switch the scene window uses). Returns
/// whether a browser was actually asked for.
pub fn open_in_browser(path: &Path) -> bool {
    if std::env::var_os("POSIM_NO_BROWSER").is_some() {
        return false;
    }
    let target = path.to_string_lossy().into_owned();
    let launched = if cfg!(target_os = "macos") {
        Command::new("open").arg(&target).spawn()
    } else if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "start", "", &target]).spawn()
    } else {
        Command::new("xdg-open").arg(&target).spawn()
    };
    launched.is_ok()
}

/// A live connection to one dataset: a `solveit_db serve` child holding
/// the r2d2 pool, spoken to in line-delimited JSON.
pub struct DatasetHandle {
    pub name: String,
    child: Child,
    stdin: ChildStdin,
    reader: BufReader<ChildStdout>,
    next_id: u64,
    /// Last `BOX` size written into meta, to avoid rewriting it on
    /// every append.
    last_box: Option<Option<f64>>,
}

impl DatasetHandle {
    fn spawn(name: &str, path: &Path) -> Result<Self, String> {
        let tool = find_db_tool()?;
        let mut child = Command::new(&tool)
            .arg("serve")
            .arg(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| format!("cannot start {}: {e}", tool.display()))?;
        let stdin = child.stdin.take().ok_or("no stdin pipe to solveit_db")?;
        let stdout = child.stdout.take().ok_or("no stdout pipe from solveit_db")?;
        Ok(Self {
            name: name.to_string(),
            child,
            stdin,
            reader: BufReader::new(stdout),
            next_id: 1,
            last_box: None,
        })
    }

    /// Creates `datasets/<name>.db` FRESH — an existing dataset of the
    /// same name is replaced, so re-running a script or notebook always
    /// reproduces the same dataset (use DATASET COPY / SAVEAS first to
    /// keep an old run).
    pub fn create(name: &str) -> Result<Self, String> {
        valid_name(name)?;
        let path = db_path(name);
        if path.exists() {
            delete_files(&path)?;
        }
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("cannot create {}: {e}", parent.display()))?;
        }
        let mut h = Self::spawn(name, &path)?;
        /* Stamp provenance (no wall-clock values — outputs must be
         * byte-stable across runs). */
        let title = name.replace('\'', "''");
        h.sql(&format!(
            "INSERT OR REPLACE INTO meta(key, value) VALUES ('title', '{title}')"
        ))?;
        Ok(h)
    }

    /// Opens an existing dataset.
    pub fn open(name: &str) -> Result<Self, String> {
        valid_name(name)?;
        let path = db_path(name);
        if !path.is_file() {
            return Err(format!(
                "no such dataset `{name}` ({}). DATASET LIST shows what exists.",
                path.display()
            ));
        }
        Self::spawn(name, &path)
    }

    fn rpc(&mut self, mut req: std::collections::BTreeMap<String, Json>) -> Result<Json, String> {
        let id = self.next_id;
        self.next_id += 1;
        req.insert("id".to_string(), Json::Num(id as f64));
        let line = machine::to_string(&Json::Obj(req));
        writeln!(self.stdin, "{line}").map_err(|e| format!("solveit_db went away: {e}"))?;
        self.stdin.flush().map_err(|e| format!("solveit_db went away: {e}"))?;
        loop {
            let mut reply = String::new();
            let n = self
                .reader
                .read_line(&mut reply)
                .map_err(|e| format!("solveit_db went away: {e}"))?;
            if n == 0 {
                return Err("solveit_db closed the connection".to_string());
            }
            let v = machine::parse(reply.trim())
                .map_err(|e| format!("bad reply from solveit_db: {e}"))?;
            let matches = match v.get("id") {
                Some(Json::Num(rid)) => *rid == id as f64,
                _ => false,
            };
            if !matches {
                continue;
            }
            if let Some(Json::Bool(true)) = v.get("ok") {
                return Ok(v);
            }
            let msg = v
                .get("error")
                .and_then(Json::as_str)
                .unwrap_or("unknown error")
                .to_string();
            return Err(msg);
        }
    }

    /// Runs arbitrary SQL. Returns (columns, rows, changed-row count).
    pub fn sql(&mut self, sql: &str) -> Result<(Vec<String>, Vec<Vec<Json>>, i64), String> {
        let mut req = std::collections::BTreeMap::new();
        req.insert("op".to_string(), Json::Str("sql".to_string()));
        req.insert("sql".to_string(), Json::Str(sql.to_string()));
        let v = self.rpc(req)?;
        let columns = match v.get("columns") {
            Some(Json::Arr(a)) => a
                .iter()
                .map(|c| c.as_str().unwrap_or("").to_string())
                .collect(),
            _ => Vec::new(),
        };
        let rows = match v.get("rows") {
            Some(Json::Arr(a)) => a
                .iter()
                .map(|r| match r {
                    Json::Arr(cells) => cells.clone(),
                    other => vec![other.clone()],
                })
                .collect(),
            _ => Vec::new(),
        };
        let changes = match v.get("changes") {
            Some(Json::Num(n)) => *n as i64,
            _ => 0,
        };
        Ok((columns, rows, changes))
    }

    /// Appends one batch of frames (bodies re-synced first). Also keeps
    /// the `box` meta key current so a movie can draw the bounding box.
    pub fn append(&mut self, state: &SimState, frames: Vec<Json>) -> Result<(i64, i64), String> {
        if self.last_box != Some(state.box_size) {
            let boxsql = match state.box_size {
                Some(size) => format!(
                    "INSERT OR REPLACE INTO meta(key, value) VALUES ('box', '{size}')"
                ),
                None => "DELETE FROM meta WHERE key = 'box'".to_string(),
            };
            self.sql(&boxsql)?;
            self.last_box = Some(state.box_size);
        }
        let mut req = std::collections::BTreeMap::new();
        req.insert("op".to_string(), Json::Str("append".to_string()));
        req.insert("bodies".to_string(), bodies_payload(state));
        req.insert("frames".to_string(), Json::Arr(frames));
        let v = self.rpc(req)?;
        let first = match v.get("first_frame") {
            Some(Json::Num(n)) => *n as i64,
            _ => 0,
        };
        let n = match v.get("frames") {
            Some(Json::Num(n)) => *n as i64,
            _ => 0,
        };
        Ok((first, n))
    }

    /// Row counts for messages: (bodies, frames, states).
    pub fn counts(&mut self) -> Result<(i64, i64, i64), String> {
        let mut req = std::collections::BTreeMap::new();
        req.insert("op".to_string(), Json::Str("info".to_string()));
        let v = self.rpc(req)?;
        let get = |k: &str| match v.get(k) {
            Some(Json::Num(n)) => *n as i64,
            _ => 0,
        };
        Ok((get("bodies"), get("frames"), get("states")))
    }

    /// Checkpoints the WAL and shuts the child down.
    pub fn close(mut self) -> Result<String, String> {
        let mut req = std::collections::BTreeMap::new();
        req.insert("op".to_string(), Json::Str("close".to_string()));
        let _ = self.rpc(req);
        let _ = self.child.wait();
        let name = self.name.clone();
        /* Drop must not close twice: forget self after the clean
         * shutdown (all owned pipes/child were already waited on). */
        std::mem::forget(self);
        Ok(format!("dataset {name} closed"))
    }
}

impl Drop for DatasetHandle {
    fn drop(&mut self) {
        /* Best-effort shutdown so no solveit_db child outlives posim. */
        let _ = writeln!(self.stdin, "{{\"id\":0,\"op\":\"close\"}}");
        let _ = self.stdin.flush();
        let _ = self.child.wait();
    }
}

fn delete_files(path: &Path) -> Result<(), String> {
    std::fs::remove_file(path).map_err(|e| format!("delete {}: {e}", path.display()))?;
    for suffix in ["-wal", "-shm"] {
        let mut side = path.as_os_str().to_owned();
        side.push(suffix);
        let side = PathBuf::from(side);
        if side.exists() {
            std::fs::remove_file(&side).map_err(|e| format!("delete {}: {e}", side.display()))?;
        }
    }
    Ok(())
}

/// Deletes `datasets/<name>.db` (refusing while it is the open one is
/// the VM's job — it knows what is open).
pub fn delete(name: &str) -> Result<String, String> {
    valid_name(name)?;
    let path = db_path(name);
    if !path.exists() {
        return Err(format!("no such dataset `{name}` ({})", path.display()));
    }
    delete_files(&path)?;
    Ok(format!("dataset {name} deleted"))
}

/// Copies `src` to `dst` via the sidecar's VACUUM INTO (compact,
/// checkpointed, safe against a live writer).
pub fn copy(src: &str, dst: &str) -> Result<String, String> {
    valid_name(src)?;
    valid_name(dst)?;
    let from = db_path(src);
    let to = db_path(dst);
    if !from.is_file() {
        return Err(format!("no such dataset `{src}` ({})", from.display()));
    }
    if to.exists() {
        return Err(format!("dataset `{dst}` already exists ({})", to.display()));
    }
    let tool = find_db_tool()?;
    let out = Command::new(&tool)
        .arg("copy")
        .arg(&from)
        .arg(&to)
        .output()
        .map_err(|e| format!("cannot start {}: {e}", tool.display()))?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
    }
    Ok(format!("dataset {src} copied to {dst}"))
}

/// Names of every dataset under `datasets/`, sorted.
pub fn list() -> Result<Vec<String>, String> {
    let dir = Path::new(DATASET_DIR);
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut names: Vec<String> = std::fs::read_dir(dir)
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
    Ok(names)
}

/// One-shot query against a dataset ON DISK (no open handle needed —
/// WAL guarantees a reader sees every committed transaction even while
/// a serve child holds the file). Returns (columns, rows).
pub fn query_file(db: &Path, sql: &str) -> Result<(Vec<String>, Vec<Vec<Json>>), String> {
    let tool = find_db_tool()?;
    let out = Command::new(&tool)
        .arg("query")
        .arg(db)
        .arg(sql)
        .output()
        .map_err(|e| format!("cannot start {}: {e}", tool.display()))?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let v = machine::parse(text.trim()).map_err(|e| format!("bad query output: {e}"))?;
    let columns = match v.get("columns") {
        Some(Json::Arr(a)) => a.iter().map(|c| c.as_str().unwrap_or("").to_string()).collect(),
        _ => Vec::new(),
    };
    let rows = match v.get("rows") {
        Some(Json::Arr(a)) => a
            .iter()
            .map(|r| match r {
                Json::Arr(cells) => cells.clone(),
                other => vec![other.clone()],
            })
            .collect(),
        _ => Vec::new(),
    };
    Ok((columns, rows))
}

pub fn jnum(j: &Json) -> f64 {
    match j {
        Json::Num(n) => *n,
        _ => f64::NAN,
    }
}

// ---------------------------------------------------------------------
// Recording payloads
// ---------------------------------------------------------------------

fn geometry_json(b: &Boundary) -> (String, String) {
    match b {
        Boundary::Point => ("point".to_string(), "{}".to_string()),
        Boundary::Sphere { radius } => {
            ("sphere".to_string(), format!("{{\"radius\":{radius}}}"))
        }
        Boundary::Cuboid { half_extents } => (
            "cuboid".to_string(),
            format!(
                "{{\"half_extents\":[{},{},{}]}}",
                half_extents[0], half_extents[1], half_extents[2]
            ),
        ),
        Boundary::Torus { ring_radius, tube_radius } => (
            "torus".to_string(),
            format!("{{\"ring_radius\":{ring_radius},\"tube_radius\":{tube_radius}}}"),
        ),
        Boundary::Disk { radius } => ("disk".to_string(), format!("{{\"radius\":{radius}}}")),
        Boundary::Cylinder { radius, half_height } => (
            "cylinder".to_string(),
            format!("{{\"radius\":{radius},\"half_height\":{half_height}}}"),
        ),
        Boundary::Dumbbell { r1, r2, rod_radius, z1, z2, .. } => (
            "dumbbell".to_string(),
            format!(
                "{{\"r1\":{r1},\"r2\":{r2},\"rod_radius\":{rod_radius},\"z1\":{z1},\"z2\":{z2}}}"
            ),
        ),
    }
}

/// The bodies table payload: every current object, keyed by its STABLE
/// id (`get_id()`, not the index — DEL renumbers indices but never
/// reuses ids), with the user-registered name when one exists.
pub fn bodies_payload(state: &SimState) -> Json {
    let mut rows = Vec::new();
    for (i, o) in state.system.objects.iter().enumerate() {
        let mut m = std::collections::BTreeMap::new();
        let name = state
            .names
            .iter()
            .find(|(_, oi)| **oi == i)
            .map(|(n, _)| n.clone())
            .unwrap_or_else(|| format!("obj{i}"));
        let (shape, geometry) = geometry_json(&o.get_boundary());
        m.insert("body".to_string(), Json::Num(o.get_id() as f64));
        m.insert("name".to_string(), Json::Str(name));
        m.insert("shape".to_string(), Json::Str(shape));
        m.insert("mass".to_string(), Json::Num(o.get_mass()));
        m.insert("wall".to_string(), Json::Bool(state.wall_indices.contains(&i)));
        m.insert("geometry".to_string(), Json::Str(geometry));
        rows.push(Json::Obj(m));
    }
    Json::Arr(rows)
}

fn state_row(
    id: usize,
    p: ::physical_object::linalg::Vec3,
    q: ::physical_object::linalg::Quat,
    v: ::physical_object::linalg::Vec3,
    w: ::physical_object::linalg::Vec3,
) -> Json {
    Json::Arr(vec![
        Json::Num(id as f64),
        Json::Num(p.x),
        Json::Num(p.y),
        Json::Num(p.z),
        Json::Num(q.w),
        Json::Num(q.x),
        Json::Num(q.y),
        Json::Num(q.z),
        Json::Num(v.x),
        Json::Num(v.y),
        Json::Num(v.z),
        Json::Num(w.x),
        Json::Num(w.y),
        Json::Num(w.z),
    ])
}

fn frame_obj(
    t: f64,
    energy: f64,
    p: ::physical_object::linalg::Vec3,
    l: ::physical_object::linalg::Vec3,
    com: ::physical_object::linalg::Vec3,
    ncollisions: u64,
    states: Vec<Json>,
) -> Json {
    let vec3 = |v: ::physical_object::linalg::Vec3| {
        Json::Arr(vec![Json::Num(v.x), Json::Num(v.y), Json::Num(v.z)])
    };
    let mut m = std::collections::BTreeMap::new();
    m.insert("t".to_string(), Json::Num(t));
    m.insert("e".to_string(), Json::Num(energy));
    m.insert("p".to_string(), vec3(p));
    m.insert("l".to_string(), vec3(l));
    m.insert("c".to_string(), vec3(com));
    m.insert("n".to_string(), Json::Num(ncollisions as f64));
    m.insert("s".to_string(), Json::Arr(states));
    Json::Obj(m)
}

/// A frame captured from the LIVE system (DATASET SNAPSHOT, and the
/// frame recorded after STEP).
pub fn frame_from_system(state: &SimState) -> Json {
    use ::physical_object::linalg::Vec3;
    let s = &state.system;
    let states = s
        .objects
        .iter()
        .map(|o| {
            state_row(
                o.get_id(),
                o.get_position(),
                o.get_orientation(),
                o.get_velocity(),
                o.angular_velocity(),
            )
        })
        .collect();
    frame_obj(
        s.time,
        s.total_energy(),
        s.total_momentum(),
        s.total_angular_momentum(Vec3::zeros()),
        s.center_of_mass(),
        s.collision_count,
        states,
    )
}

/// A frame from an integration [`Snapshot`] (the per-output captures a
/// RUN makes on its way to t_end).
pub fn frame_from_snapshot(snap: &Snapshot) -> Json {
    let states = snap
        .body_states
        .iter()
        .map(|b| state_row(b.id, b.position, b.orientation, b.velocity, b.angular_velocity))
        .collect();
    frame_obj(
        snap.t,
        snap.energy,
        snap.total_momentum,
        snap.total_angular_momentum,
        snap.center_of_mass,
        snap.collision_count,
        states,
    )
}

// ---------------------------------------------------------------------
// DATASET PLAY — time-series plots in a browser window
// ---------------------------------------------------------------------

/// Builds `datasets/<name>_plots.html` (self-contained: vanilla JS +
/// canvas, zero external fetches) and opens it in the browser. Returns
/// the status message.
pub fn play(name: &str) -> Result<String, String> {
    valid_name(name)?;
    let db = db_path(name);
    if !db.is_file() {
        return Err(format!("no such dataset `{name}` ({})", db.display()));
    }
    let (_, frames) = query_file(
        &db,
        "SELECT t, energy, px, py, pz, lx, ly, lz FROM frames ORDER BY frame",
    )?;
    if frames.is_empty() {
        return Err(format!(
            "dataset `{name}` has no frames yet — RUN or STEP with recording on, \
             or DATASET SNAPSHOT, then try again"
        ));
    }
    let (_, bodies) = query_file(&db, "SELECT body, name, wall FROM bodies ORDER BY body")?;
    let (_, states) = query_file(
        &db,
        "SELECT s.body, f.t, s.x, s.y, s.z FROM states s \
         JOIN frames f ON f.frame = s.frame ORDER BY s.body, s.frame",
    )?;

    let mut data = String::from("{\"name\":\"");
    data.push_str(name);
    data.push_str("\",\"frames\":[");
    for (i, r) in frames.iter().enumerate() {
        if i > 0 {
            data.push(',');
        }
        data.push_str(&format!(
            "[{},{},{},{},{},{},{},{}]",
            js_num(&r[0]),
            js_num(&r[1]),
            js_num(&r[2]),
            js_num(&r[3]),
            js_num(&r[4]),
            js_num(&r[5]),
            js_num(&r[6]),
            js_num(&r[7]),
        ));
    }
    data.push_str("],\"bodies\":[");
    for (i, r) in bodies.iter().enumerate() {
        if i > 0 {
            data.push(',');
        }
        let bname = r[1].as_str().unwrap_or("?").replace('"', "");
        data.push_str(&format!(
            "{{\"id\":{},\"name\":\"{}\",\"wall\":{}}}",
            js_num(&r[0]),
            bname,
            if jnum(&r[2]) != 0.0 { "true" } else { "false" }
        ));
    }
    data.push_str("],\"paths\":[");
    for (i, r) in states.iter().enumerate() {
        if i > 0 {
            data.push(',');
        }
        data.push_str(&format!(
            "[{},{},{},{},{}]",
            js_num(&r[0]),
            js_num(&r[1]),
            js_num(&r[2]),
            js_num(&r[3]),
            js_num(&r[4]),
        ));
    }
    data.push_str("]}");

    let page = PLOTS_PAGE.replace("__TITLE__", name).replace("__DATA__", &data);
    let out = Path::new(DATASET_DIR).join(format!("{name}_plots.html"));
    std::fs::write(&out, page).map_err(|e| format!("write {}: {e}", out.display()))?;
    let opened = open_in_browser(&out);
    Ok(format!(
        "dataset plots written to {} ({})",
        out.display(),
        if opened { "browser opened" } else { "browser suppressed" }
    ))
}

fn js_num(j: &Json) -> String {
    let v = jnum(j);
    if v.is_finite() {
        format!("{v:?}")
    } else {
        "null".to_string()
    }
}

/// The DATASET PLAY page. Self-contained by construction: no CDN, no
/// fetch, no WebSocket — the data is baked in as `__DATA__`.
const PLOTS_PAGE: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>dataset __TITLE__ — plots</title>
<style>
  body { margin: 0; background: #0e1116; color: #cdd6e4;
         font: 13px/1.5 -apple-system, "Segoe UI", Roboto, sans-serif; }
  h1 { font-size: 16px; margin: 14px 18px 4px; font-weight: 600; }
  h1 code { color: #7fb4e6; }
  p.sub { margin: 0 18px 10px; color: #7d8797; }
  .grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(430px, 1fr));
          gap: 12px; padding: 0 14px 18px; }
  .card { background: #151a22; border: 1px solid #232b37; border-radius: 8px; padding: 8px; }
  .card h2 { font-size: 12.5px; font-weight: 600; margin: 2px 4px 6px; color: #9fb0c6; }
  canvas { width: 100%; height: 240px; display: block; }
</style>
</head>
<body>
<h1>dataset <code>__TITLE__</code></h1>
<p class="sub">Recorded by posim into SQLite (WAL). Every panel is drawn from the
frames / states tables of <code>datasets/__TITLE__.db</code>.</p>
<div class="grid" id="grid"></div>
<script>
"use strict";
const DATA = __DATA__;
const HUES = ["#6db1e0","#e0a56d","#8fd08f","#d98fb8","#b9a5e8","#e8d06d","#7fd6c9","#e08383"];

function card(title) {
  const div = document.createElement("div");
  div.className = "card";
  const h = document.createElement("h2");
  h.textContent = title;
  const c = document.createElement("canvas");
  div.appendChild(h); div.appendChild(c);
  document.getElementById("grid").appendChild(div);
  return c;
}

/* A minimal line chart: autoscaled axes, ticks, legend. */
function chart(canvas, series, xlabel) {
  const dpr = window.devicePixelRatio || 1;
  const W = canvas.clientWidth * dpr, H = 240 * dpr;
  canvas.width = W; canvas.height = H;
  const g = canvas.getContext("2d");
  const L = 58 * dpr, R = 10 * dpr, T = 10 * dpr, B = 30 * dpr;
  let xmin = Infinity, xmax = -Infinity, ymin = Infinity, ymax = -Infinity;
  for (const s of series) for (let i = 0; i < s.x.length; i++) {
    const x = s.x[i], y = s.y[i];
    if (x === null || y === null) continue;
    if (x < xmin) xmin = x; if (x > xmax) xmax = x;
    if (y < ymin) ymin = y; if (y > ymax) ymax = y;
  }
  if (!isFinite(xmin)) { xmin = 0; xmax = 1; }
  if (!isFinite(ymin)) { ymin = 0; ymax = 1; }
  if (xmax === xmin) { xmax = xmin + 1; }
  if (ymax === ymin) { const c0 = ymin; ymin = c0 - (Math.abs(c0) || 1) * 1e-3; ymax = c0 + (Math.abs(c0) || 1) * 1e-3; }
  const pad = (ymax - ymin) * 0.06; ymin -= pad; ymax += pad;
  const px = x => L + (x - xmin) / (xmax - xmin) * (W - L - R);
  const py = y => H - B - (y - ymin) / (ymax - ymin) * (H - T - B);
  g.fillStyle = "#151a22"; g.fillRect(0, 0, W, H);
  g.strokeStyle = "#26303e"; g.fillStyle = "#7d8797";
  g.lineWidth = dpr; g.font = `${11 * dpr}px sans-serif`;
  const fmt = v => { const a = Math.abs(v);
    return (a !== 0 && (a < 1e-3 || a >= 1e5)) ? v.toExponential(2) : +v.toPrecision(6) + ""; };
  for (let k = 0; k <= 4; k++) {
    const y = ymin + (ymax - ymin) * k / 4;
    g.beginPath(); g.moveTo(L, py(y)); g.lineTo(W - R, py(y)); g.stroke();
    g.textAlign = "right"; g.fillText(fmt(y), L - 5 * dpr, py(y) + 4 * dpr);
    const x = xmin + (xmax - xmin) * k / 4;
    g.textAlign = "center"; g.fillText(fmt(x), px(x), H - B + 16 * dpr);
  }
  g.textAlign = "center";
  g.fillText(xlabel, (L + W - R) / 2, H - 4 * dpr);
  series.forEach((s, si) => {
    g.strokeStyle = s.color || HUES[si % HUES.length];
    g.lineWidth = 1.6 * dpr;
    g.beginPath();
    let started = false;
    for (let i = 0; i < s.x.length; i++) {
      if (s.x[i] === null || s.y[i] === null) { started = false; continue; }
      const X = px(s.x[i]), Y = py(s.y[i]);
      if (!started) { g.moveTo(X, Y); started = true; } else { g.lineTo(X, Y); }
    }
    g.stroke();
  });
  let lx = L + 8 * dpr;
  series.forEach((s, si) => {
    if (!s.label) return;
    g.fillStyle = s.color || HUES[si % HUES.length];
    g.fillRect(lx, T + 2 * dpr, 10 * dpr, 3 * dpr);
    g.textAlign = "left"; g.fillStyle = "#9fb0c6";
    g.fillText(s.label, lx + 14 * dpr, T + 8 * dpr);
    lx += (14 + 8 * s.label.length) * dpr;
  });
}

const F = DATA.frames;                 /* [t, E, px,py,pz, lx,ly,lz] */
const ts = F.map(f => f[0]);
const mag = (a,b,c) => (a===null||b===null||c===null) ? null : Math.hypot(a,b,c);

chart(card("total energy E(t)"),
      [{ x: ts, y: F.map(f => f[1]), label: "E", color: HUES[0] }], "t");
chart(card("momentum and angular momentum magnitudes"),
      [{ x: ts, y: F.map(f => mag(f[2],f[3],f[4])), label: "|P|", color: HUES[1] },
       { x: ts, y: F.map(f => mag(f[5],f[6],f[7])), label: "|L|", color: HUES[2] }], "t");

/* Per-body position components — one card per (non-wall) body. */
const byBody = new Map();
for (const [id, t, x, y, z] of DATA.paths) {
  if (!byBody.has(id)) byBody.set(id, { t: [], x: [], y: [], z: [] });
  const s = byBody.get(id);
  s.t.push(t); s.x.push(x); s.y.push(y); s.z.push(z);
}
const walls = new Set(DATA.bodies.filter(b => b.wall).map(b => b.id));
const nameOf = id => (DATA.bodies.find(b => b.id === id) || { name: "body " + id }).name;
let shown = 0;
for (const [id, s] of byBody) {
  if (walls.has(id)) continue;
  if (++shown > 8) break;             /* keep the page readable */
  chart(card(`position of ${nameOf(id)} — x, y, z over t`),
        [{ x: s.t, y: s.x, label: "x", color: HUES[0] },
         { x: s.t, y: s.y, label: "y", color: HUES[2] },
         { x: s.t, y: s.z, label: "z", color: HUES[1] }], "t");
}

/* Top view: y against x for every non-wall body. */
/* `top` is a reserved global (window.top), so: topview. */
const topview = [];
let ci = 0;
for (const [id, s] of byBody) {
  if (walls.has(id)) continue;
  topview.push({ x: s.x, y: s.y, label: nameOf(id), color: HUES[ci++ % HUES.length] });
}
if (topview.length) chart(card("top view — y against x"), topview, "x");
</script>
</body>
</html>
"##;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_validated_strictly() {
        assert!(valid_name("kepler_e06").is_ok());
        assert!(valid_name("a-b-2").is_ok());
        assert!(valid_name("").is_err());
        assert!(valid_name("../evil").is_err());
        assert!(valid_name("a b").is_err());
        assert!(valid_name("x/y").is_err());
    }

    #[test]
    fn db_paths_stay_inside_the_dataset_dir() {
        assert_eq!(db_path("kepler"), Path::new("datasets").join("kepler.db"));
    }

    /// Every DATASET / MOVIE subcommand must appear in the parser's
    /// EBNF, in `HELP_TEXT`, and in both grammar documents — and the
    /// EBNF must promise nothing the parser does not implement. Same
    /// two-directional discipline as
    /// `every_qm2_subcommand_is_documented_in_lockstep`.
    #[test]
    fn every_dataset_and_movie_subcommand_is_documented_in_lockstep() {
        let prep = |s: &str| s.replace('\\', "").to_ascii_uppercase();
        let help = prep(crate::vm::HELP_TEXT);
        let all = prep(include_str!("parser.rs"));
        /* Anchor on the PRODUCTIONS (`datasetcmd :=`), not the first
         * mention — the `command` production's alternatives line quotes
         * "DATASET" datasetcmd earlier in the file. */
        let ds_from = all.find("DATASETCMD :=").expect("parser.rs must declare datasetcmd :=");
        let ds_to = all[ds_from..]
            .find("MOVIECMD :=")
            .map_or(all.len(), |k| ds_from + k);
        let ds_ebnf = all[ds_from..ds_to].to_string();
        let mv_from = ds_to;
        let mv_to = all[mv_from..].find("NAME     :=").map_or(all.len(), |k| mv_from + k);
        let mv_ebnf = all[mv_from..mv_to].to_string();
        let md = prep(include_str!("../../grammar.md"));
        let tex = prep(include_str!("../../grammar.tex"));

        let mut missing = Vec::new();
        let mut check_family =
            |family: &str, subs: &[&str], ebnf: &str, help: &str, md: &str, tex: &str| {
                for w in subs {
                    let up = w.to_ascii_uppercase();
                    let needle = format!("{family} {up}");
                    let ebnf_needle = format!("\"{up}\"");
                    for (what, hay) in [
                        ("HELP_TEXT", help),
                        ("parser.rs EBNF", ebnf),
                        ("grammar.md", md),
                        ("grammar.tex", tex),
                    ] {
                        let want: &str =
                            if what == "parser.rs EBNF" { &ebnf_needle } else { &needle };
                        if !hay.contains(want) {
                            missing.push(format!("{want} is missing from {what}"));
                        }
                    }
                }
            };
        check_family("DATASET", crate::dataset::DATASET_SUBCOMMANDS, &ds_ebnf, &help, &md, &tex);
        check_family("MOVIE", crate::dataset::MOVIE_SUBCOMMANDS, &mv_ebnf, &help, &md, &tex);
        /* SQL is a one-production family of its own. */
        for (what, hay) in
            [("HELP_TEXT", &help), ("grammar.md", &md), ("grammar.tex", &tex)]
        {
            if !hay.contains("SQL") {
                missing.push(format!("SQL is missing from {what}"));
            }
        }

        /* Converse: every quoted ALL-CAPS word in the two EBNF blocks
         * must be a declared subcommand or a declared argument word. */
        const ARGUMENT_WORDS: &[&str] = &["ON", "OFF", "AS"];
        let mut phantom = Vec::new();
        for (name, ebnf, subs) in [
            ("datasetcmd", &ds_ebnf, crate::dataset::DATASET_SUBCOMMANDS),
            ("moviecmd", &mv_ebnf, crate::dataset::MOVIE_SUBCOMMANDS),
        ] {
            let mut rest = ebnf.as_str();
            while let Some(i) = rest.find('"') {
                rest = &rest[i + 1..];
                let Some(j) = rest.find('"') else { break };
                let word = &rest[..j];
                rest = &rest[j + 1..];
                if word.is_empty() || !word.chars().all(|c| c.is_ascii_uppercase() || c == '_')
                {
                    continue;
                }
                let known = subs.iter().any(|s| s.eq_ignore_ascii_case(word))
                    || ARGUMENT_WORDS.contains(&word);
                if !known {
                    phantom.push(format!(
                        "the {name} EBNF quotes `{word}`, which is neither a declared \
                         subcommand nor a declared argument word"
                    ));
                }
            }
        }
        assert!(
            missing.is_empty() && phantom.is_empty(),
            "documentation lockstep broken:\n  {}\n  {}",
            missing.join("\n  "),
            phantom.join("\n  ")
        );
    }

    #[test]
    fn the_plots_page_is_self_contained() {
        /* No external fetches, ever — same rule as scene.html and the
         * recorded video pages. */
        for banned in ["http://", "https://", "fetch(", "XMLHttpRequest", "WebSocket", "import "] {
            assert!(
                !PLOTS_PAGE.contains(banned),
                "plots page must not contain `{banned}`"
            );
        }
        assert!(PLOTS_PAGE.contains("__DATA__") && PLOTS_PAGE.contains("__TITLE__"));
    }
}
