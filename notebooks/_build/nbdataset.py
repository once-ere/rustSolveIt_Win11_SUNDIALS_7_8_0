"""The dataset sections (SQLite recording) shared by every notebook.

Invariant prose and code templates, exactly like nbtext.py, but for the
dataset / SQL / movie machinery. `__DATASET__` is substituted by
nbbuild.py with the notebook's own dataset name.
"""

DATASET_MD = r"""## 6. This notebook's dataset

Every notebook in this repository owns a **dataset**: one SQLite
database file, created fresh each time the notebook runs, that records
the simulation as it happens. This notebook's dataset is
`datasets/__DATASET__.db`, and the cell below creates it.

What "recording" means here: from the moment the dataset is created,
every output row of a `run` and the end of every `step` is appended as
one **frame** — the time, the total energy, the momentum and
angular-momentum components, the centre of mass, the cumulative
collision count, and the full position / orientation (w-first
quaternion) / velocity / angular-velocity state of every body. The
schema has four tables: `meta` (key/value pairs), `bodies` (one row per
body: name, shape, mass, geometry), `frames` (one row per recorded
instant) and `states` (one row per body per frame).

The simulator itself contains no SQLite code at all. It drives a small
sidecar program, `solveit_db` (built once with
`cd dataset_tools && cargo build --release` from the repository root),
which owns a thread-safe connection pool (`r2d2`), switches the
database into **WAL journal mode** at startup, and sets a five-second
**busy timeout** — the standard recipe that lets several readers and a
writer share one SQLite file without ever seeing `database is locked`.
WAL matters to *this notebook* concretely: a later cell reads the
database with Python's own `sqlite3` module **while the simulator still
holds it open**, which is exactly the cross-process concurrency WAL
provides.

Creating a dataset arms recording automatically (`dataset record off`
would pause it). If a dataset of this name already exists from an
earlier run of this notebook, it is replaced, so a re-run always
reproduces the same database.
"""

DATASET_SELF_MD = r"""## 6. This notebook's dataset

Every notebook in this repository owns a **dataset**: one SQLite
database file that records the simulation as it happens. This
notebook's paired example script manages its dataset itself — you will
see the `dataset create` command among the steps below, followed by SQL
queries that interrogate the recorded frames, so the recording is part
of the worked example rather than something added around it. The
dataset lands in `datasets/__DATASET__.db`.

What "recording" means here: from the moment a dataset is created,
every output row of a `run` and the end of every `step` is appended as
one **frame** — time, total energy, momentum and angular-momentum
components, centre of mass, cumulative collision count, and the full
position / orientation (w-first quaternion) / velocity /
angular-velocity state of every body. The schema has four tables:
`meta`, `bodies`, `frames` and `states`.

The simulator itself contains no SQLite code. It drives the
`solveit_db` sidecar (built once with
`cd dataset_tools && cargo build --release` from the repository root),
which owns a thread-safe `r2d2` connection pool, switches the file into
**WAL journal mode** at startup and sets a five-second **busy
timeout** — the standard recipe that lets several readers and a writer
share one SQLite file. A later cell in this notebook reads the same
database with Python's own `sqlite3` module, which is exactly the
cross-process concurrency WAL exists to provide.
"""

DATASET_CREATE_CODE = 'sim.do("dataset create __DATASET__")'

READBACK_MD = r"""## 9. Reading the dataset back

The run above has been recorded. Three different readers now open the
same SQLite file, which is the point of the exercise:

1. `dataset snapshot` appends one more frame — the state exactly as it
   stands now — so even a notebook that only stepped a little has
   something in the table.
2. The `sql` command sends **any SQLite statement** to the open
   dataset, verbatim: `SELECT` with joins, window functions and CTEs,
   `INSERT`, `UPDATE`, `CREATE VIEW`, `ATTACH`, `PRAGMA` — the whole
   SQLite grammar. Here it counts the frames and summarises the time
   range.
3. `dataset play` renders the recorded time series — energy, momentum
   magnitudes, per-body positions, a top view — into a self-contained
   page `datasets/__DATASET___plots.html` and opens it in your web
   browser (suppressed in batch runs, where no browser is available).
"""

READBACK_SQL_CODE = "\n".join([
    'sim.do("dataset snapshot")',
    'sim.do(\'sql "SELECT COUNT(*) AS frames, MIN(t) AS t_first, MAX(t) AS t_last FROM frames"\')',
    'sim.do(\'sql "SELECT COUNT(*) AS body_state_rows FROM states"\')',
    'sim.do("dataset play")',
])

READBACK_SELF_MD = r"""## 9. Reading the dataset back

The paired script above already interrogated its own dataset with SQL
as part of the worked example. This section adds the one reader the
script cannot show: a completely separate process. The cell below is
plain Python using only the standard library's `sqlite3` module — no
simulator involved — opening the very same database file. Anything a
spreadsheet, a plotting library or another program can do with a SQLite
file, it can do with this dataset.
"""

PY_READBACK_MD = r"""The cell below is the third reader: plain Python, using only the
standard library's `sqlite3` module — no simulator involved. It opens
the very database the simulator is still holding open (WAL mode is what
makes that safe), prints the row counts and the last recorded frame,
and closes. Anything a spreadsheet, a plotting library or another
program can do with a SQLite file, it can do with this dataset.
"""

PY_READBACK_CODE = "\n".join([
    "import sqlite3",
    "",
    'con = sqlite3.connect("datasets/__DATASET__.db")',
    "try:",
    "    tables = {}",
    '    for table in ("meta", "bodies", "frames", "states"):',
    '        tables[table] = con.execute(f"SELECT COUNT(*) FROM {table}").fetchone()[0]',
    '    print("row counts:", tables)',
    "    last = con.execute(",
    '        "SELECT frame, t, energy, ncollisions FROM frames ORDER BY frame DESC LIMIT 1"',
    "    ).fetchone()",
    "    if last is not None:",
    '        print(f"last frame: frame={last[0]} t={last[1]} energy={last[2]} collisions={last[3]}")',
    "    for row in con.execute(",
    '        "SELECT b.name, b.shape, COUNT(s.frame) FROM bodies b "',
    '        "LEFT JOIN states s ON s.body = b.body GROUP BY b.body ORDER BY b.body"',
    "    ):",
    '        print(f"body {row[0]!r} ({row[1]}): {row[2]} recorded states")',
    "finally:",
    "    con.close()",
])

MOVIE_MD = r"""## 10. The movie derived from this dataset

A **movie** in this repository is not a screen recording — it is a
self-contained 3-D player page generated FROM a dataset: `movie create
<dataset>` reads the `bodies`, `frames` and `states` tables and writes
`movies/<name>.html`, which animates the recorded rigid bodies with a
scrub bar, play/pause, speed control, an orbiting camera and a live
readout of t, E, |P| and |L|. Because the dataset is the source of
truth, a movie can be deleted and rebuilt at any time without
re-running any physics, and one dataset can carry several movies.

The cell below asks the database how much it recorded, and builds and
plays the movie only if there is something to animate — a dataset with
no moving bodies (for example a purely quantum-mechanical notebook,
whose wavefunction lives on a grid rather than in rigid-body states)
gets an honest explanation instead of an error.
"""

MOVIE_GUARD_CODE = "\n".join([
    "frames = int(sim.do('sql \"SELECT COUNT(*) FROM frames\"', quiet=True).splitlines()[-1])",
    "state_rows = int(sim.do('sql \"SELECT COUNT(*) FROM states\"', quiet=True).splitlines()[-1])",
    "if frames >= 2 and state_rows > 0:",
    '    print(sim.do("movie create __DATASET__", quiet=True))',
    '    print(sim.do("movie play __DATASET__", quiet=True))',
    "else:",
    '    print(f"this dataset holds {frames} frame(s) and {state_rows} body-state row(s);")',
    '    print("a movie needs at least 2 frames of at least one body, so none is made here.")',
])

MOVIE_SELF_MD = r"""## 10. The movie derived from this dataset

A **movie** in this repository is not a screen recording — it is a
self-contained 3-D player page generated FROM a dataset: `movie create
<dataset>` reads the `bodies`, `frames` and `states` tables and writes
`movies/<name>.html`, which animates the recorded rigid bodies with a
scrub bar, play/pause, speed control, an orbiting camera and a live
readout of t, E, |P| and |L|. This notebook's paired script issues its
own dataset and movie commands among the steps above, so that work is
part of the worked example itself. Because the dataset is the source of
truth, those movies can be deleted and rebuilt at any time — for
example `movie create __DATASET__` — without re-running any physics.
"""

CLOSE_DATASET_AND_SIM_CODE = "\n".join([
    'sim.do("dataset close")',
    "sim.close()",
])


READBACK_RESET_MD = r"""## 9. Reading the dataset back

One of the steps above was a `reset` — the command that clears the
world and starts a fresh scenario. A dataset describes ONE world, so
the `reset` checkpointed and closed it; everything recorded up to that
moment is safely on disk in `datasets/__DATASET__.db`, and the frames
that follow a reset belong to a different world and are deliberately
not mixed into the same tables.

The cell below reopens the file and interrogates it: `dataset open`
attaches it again, the `sql` command (which accepts ANY SQLite
statement, verbatim — the whole SQLite grammar) counts what was
recorded, and `dataset play` renders the recorded time series into a
self-contained page `datasets/__DATASET___plots.html`, opened in your
web browser (suppressed in batch runs). If nothing had been recorded
before the reset, a snapshot of the current world is taken first so
the plots have at least one frame to show.
"""

READBACK_REOPEN_CODE = "\n".join([
    'print(sim.do("dataset open __DATASET__", quiet=True))',
    "frames = int(sim.do('sql \"SELECT COUNT(*) FROM frames\"', quiet=True).splitlines()[-1])",
    "if frames == 0:",
    '    sim.do("dataset snapshot")',
    'sim.do(\'sql "SELECT COUNT(*) AS frames, MIN(t) AS t_first, MAX(t) AS t_last FROM frames"\')',
    'sim.do(\'sql "SELECT COUNT(*) AS body_state_rows FROM states"\')',
    'sim.do("dataset play")',
])

CLOSE_GUARDED_CODE = "\n".join([
    'if not sim.do("dataset record", quiet=True).startswith("no dataset"):',
    '    sim.do("dataset close")',
    "sim.close()",
])
