# dataset_tools — the SQLite dataset sidecar (`solveit_db`)

Every example and every Jupyter notebook in this repository records its
simulation into a **dataset**: one SQLite database file under
`datasets/`. This package builds the `solveit_db.exe` binary that owns
all SQLite access. It is a **sidecar**: the simulator (`posim.exe`)
spawns it as a child process and speaks line-delimited JSON over
stdin/stdout — the simulator itself never links SQLite, so the pure-Rust
workspace keeps its zero-external-dependency, zero-`unsafe` guarantee.

## The concurrency architecture (why WAL + a pool + a busy timeout)

SQLite is an embedded database: without configuration, one writer locks
out everyone else and a second thread gets `database is locked`. This
sidecar therefore follows three rules, in `src/db.rs`:

1. **No raw connection ever crosses a thread.** Every connection comes
   from an `r2d2::Pool` built on `r2d2_sqlite::SqliteConnectionManager`
   (`make_pool`, pool size 10). The POOL is cloned into threads, never
   a connection.
2. **WAL journal mode is enabled at startup.** The pool's `with_init`
   hook runs `PRAGMA journal_mode=WAL;` on every connection as it is
   created. In WAL mode readers never block the writer and the writer
   never blocks readers — which is also what lets a Jupyter notebook's
   Python `sqlite3` reader watch a dataset **while posim is still
   appending to it**.
3. **A busy timeout absorbs simultaneous writes.**
   `PRAGMA busy_timeout = 5000;` makes a connection that finds the
   database locked wait up to five seconds instead of failing.

The proof is executable. PowerShell, from the repository root:

```powershell
cd dataset_tools
cargo test
cd ..
```

runs, among others, `ten_threads_through_one_pool_lose_nothing`
(10 threads × 200 rows through one shared pool, every row must land)
and `two_pools_on_one_file_interleave_writes_and_reads` (two pools on
one file — the cross-process picture — with a concurrent reader). On
Windows 11 that is 8 unit + 3 integration = 11 passed, 0 failed
(`logs/dataset_tools_test.log`). And:

```powershell
cd dataset_tools
cargo build --release
.\target\release\solveit_db.exe stress $env:TEMP\stress_demo.db 10 500
cd ..
```

(Git Bash: `./target/release/solveit_db.exe stress /tmp/stress_demo.db
10 500`) prints `stress: 10 threads x 500 rows = 5000 expected, 5000
written, journal_mode=wal`.

## Building

PowerShell, from the repository root:

```powershell
cd dataset_tools; cargo build --release; cd ..
```

(Git Bash: `cd dataset_tools && cargo build --release && cd ..`.)

The binary lands at `dataset_tools\target\release\solveit_db.exe`
(2,326,016 bytes; the release build finished in 18.22 s with
0 warnings on the reference machine). This package intentionally lives
OUTSIDE the simulator workspace (the root `Cargo.toml` excludes it)
because it depends on three crates.io packages, exactly as specified
for this feature: `rusqlite` 0.32.1 (with the `bundled` SQLite
amalgamation — no system libsqlite3 is used), `r2d2`, and `r2d2_sqlite`
0.25.0. `Cargo.lock` is committed, so the first build resolves the same
versions everywhere; that first build needs network access to
crates.io. The simulator workspace's own `Cargo.lock` still lists only
local crates.

**This is the only C compiled anywhere in the repository.** The
bundled SQLite 3.46.0 amalgamation is compiled by MSVC `cl.exe`
(Visual Studio 2026 Build Tools, MSVC 14.51.36231, at
`C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Tools\MSVC\14.51.36231\bin\Hostx64\x64\cl.exe`),
which the `cc` crate locates by itself — no developer command prompt,
no `vcvars`, and no WSL2 compiler or linker. The simulator workspace is
pure Rust and never invokes a C compiler; rustc finds the MSVC linker
on its own. The root `.cargo/config.toml` pins `-C target-feature=+fma`
for the simulator; the sidecar does not need it, but building it from
inside `dataset_tools\` (as above) keeps the two configurations apart.

**How posim finds the binary.** `posim.exe` looks at `$env:POSIM_DB_TOOL`
first, then walks up from the current directory probing
`dataset_tools\target\release\solveit_db.exe` (the `.exe` name before
the extensionless one — `DB_TOOL_NAMES` in `posim/src/dataset.rs`),
then searches `PATH`. So the build above is all a checkout needs; a
dataset command issued before it reports the exact build command. If
the binary lives elsewhere:

```powershell
$env:POSIM_DB_TOOL = "C:\path\to\solveit_db.exe"
```

(Git Bash: `export POSIM_DB_TOOL=/c/path/to/solveit_db.exe`.)

**Side files.** While a dataset is open, SQLite keeps `<name>.db-wal`
and `<name>.db-shm` next to it. They are transient: `close`,
`checkpoint` and posim's `DATASET CLOSE` fold the WAL back into the
`.db` and remove them. Ignore them; they are never part of a result and
never compared.

## Command reference

PowerShell, from inside `dataset_tools\` (from the repository root,
prefix `dataset_tools\` to the path; Git Bash uses
`./target/release/solveit_db.exe` with the same arguments):

```powershell
.\target\release\solveit_db.exe create <db> [--force]        # new dataset, schema v1
.\target\release\solveit_db.exe exec <db> "<sql...>"          # any SQLite statement(s), table output
.\target\release\solveit_db.exe query <db> "<sql...>"         # any SQLite statement(s), JSON output
.\target\release\solveit_db.exe info <db>                     # counts + journal mode (no timestamps)
.\target\release\solveit_db.exe list <dir>                    # dataset names in a folder
.\target\release\solveit_db.exe copy <src> <dst>              # VACUUM INTO snapshot copy
.\target\release\solveit_db.exe delete <db>                   # remove db + -wal + -shm
.\target\release\solveit_db.exe checkpoint <db>               # flush the WAL into the .db
.\target\release\solveit_db.exe serve <db>                    # JSON-over-stdio server (what posim spawns)
.\target\release\solveit_db.exe stress <db> <threads> <rows>  # multithreaded pool/WAL demonstration
```

`exec`/`query` accept the COMPLETE SQLite command language — DDL, DML,
SELECT with window functions and CTEs, PRAGMA, ATTACH, VACUUM — because
the SQL string is handed to SQLite verbatim. In PowerShell put the SQL
in double quotes as shown; a literal `$` inside it must be written
`` `$ ``. Python's own shell is another way in:
`python -m sqlite3 datasets\<name>.db "SELECT COUNT(*) FROM frames"`.

## Dataset schema (version 1)

| table  | columns | one row per |
|--------|---------|-------------|
| `meta`   | `key TEXT PRIMARY KEY, value TEXT` | metadata pair (`schema_version`, `source`, `title`) |
| `bodies` | `body INTEGER PRIMARY KEY, name TEXT, shape TEXT, mass REAL, wall INTEGER, geometry TEXT` | simulated body; `geometry` is a JSON text of the shape parameters |
| `frames` | `frame INTEGER PRIMARY KEY, t REAL, energy REAL, px py pz REAL, lx ly lz REAL, comx comy comz REAL, ncollisions INTEGER` | recorded instant (conserved quantities) |
| `states` | `frame INTEGER, body INTEGER, x y z REAL, qw qx qy qz REAL, vx vy vz REAL, wx wy wz REAL, PRIMARY KEY (frame, body)` | body state at a frame; quaternion is **w-first**, like everywhere in this repository |

No column holds a wall-clock timestamp: byte-stable, replayable output
is a verified property of this repository, and a timestamp would
silently break it.

## The serve protocol (what posim speaks)

One JSON object per line on stdin, one reply per line on stdout:

```
{"id":1,"op":"sql","sql":"SELECT COUNT(*) FROM frames"}
{"columns":["COUNT(*)"],"changes":0,"id":1,"ok":true,"rows":[[2]]}

{"id":2,"op":"append","bodies":[{"body":0,"name":"ball","shape":"sphere","mass":2.0,"wall":false,"geometry":"{\"radius\":0.5}"}],
 "frames":[{"t":0.0,"e":-1.0,"p":[0,0,0],"l":[0,0,0],"c":[0,0,0],"n":0,"s":[[0, 0,0,0, 1,0,0,0, 0,0,0, 0,0,0]]}]}
{"first_frame":0,"frames":1,"id":2,"ok":true}

{"id":3,"op":"info"}       {"id":4,"op":"checkpoint"}       {"id":5,"op":"close"}
```

A state row is `[body, x,y,z, qw,qx,qy,qz, vx,vy,vz, wx,wy,wz]`. The
server runs four worker threads, each drawing connections from the one
shared pool; `close` checkpoints the WAL and exits, leaving a single
tidy `.db` file.
