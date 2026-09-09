# DATASETS_COMPLETE.md — the SQLite dataset layer: what was asked, what was built, and the proof

Date of the work and of every measurement below: 2026-09-08, on this
**Apple Silicon Mac (arm64, macOS, Darwin 25.6.0)** — rustc 1.94.0,
cargo 1.94.0, Python 3 (Homebrew), pdflatex (TeX Live), Google Chrome
(headless captures), SQLite 3.46.0 bundled into the sidecar.

This file is self-contained: every command needed to rebuild, re-test,
re-verify, re-execute and display the work appears below in full.

---

## 1. The correction that was asked for

The earlier state of this repository interpreted "record and show a
run" as **browser movies only** — self-contained HTML player pages with
the numbers frozen into them. That interpretation was rejected. The
instruction was:

> create and employ (for plotting/displaying, for example) 'dataset'(s)
> that are associated to each and every Jupyter notebook

with SQLite as the storage engine (rusqlite + r2d2 connection pool,
WAL journal mode enabled at startup, a busy timeout so concurrent
writers wait instead of failing), a notebook grammar that exposes all
of SQLite plus dataset and movie lifecycle commands, browser plotting
for datasets, and movies **derived from** datasets — with every
document, example, and the encyclopedia brought up to date.

## 2. The solution, in one page

**A dataset is one SQLite database file** under `datasets/<name>.db`.
While one is open with recording armed, every output row of a `run`
and the end of every `step` is appended as a frame: time, total
energy, momentum, angular momentum, centre of mass, cumulative
collision count, and the full state of every body (position, w-first
orientation quaternion, velocity, angular velocity). Schema v1 is four
relational tables — `meta`, `bodies`, `frames`, `states` — with **no
timestamps anywhere**, because byte-stable re-runs are a verified
property of this repository.

**All SQLite access lives in a sidecar**, the `solveit_db` binary
(`dataset_tools/`), built exactly to the specified architecture:

- an `r2d2` + `r2d2_sqlite` **connection pool** (size 10) — the POOL is
  cloned across threads, never a connection;
- **`PRAGMA journal_mode=WAL`** executed on every connection the moment
  the pool creates it;
- **`PRAGMA busy_timeout = 5000`** so simultaneous writers wait up to
  five seconds instead of throwing `database is locked`.

The simulator (`posim`) **spawns** the sidecar and speaks line-delimited
JSON to it; it links no SQLite and no external crate. The pure-Rust
workspace therefore keeps its zero-dependency, zero-`unsafe`,
zero-warning guarantee — `dataset_tools/` is excluded from the
workspace exactly as `sundials_rs/` is, and the workspace `Cargo.lock`
still lists only the 12 local crates.

**The notebook grammar grew three families** (contextual keywords, so
`new sphere as movie` and `let sql = 3` still work):

```
DATASET CREATE <name> | OPEN <name> | CLOSE | RECORD [ON|OFF] |
        SNAPSHOT | LIST | INFO [<name>] | COPY <src> <dst> |
        SAVEAS <name> | DELETE <name> | PLAY [<name>] |
        DIFF <a> <b> AS <out> | SUM <a> <b> AS <out> |
        MAP <src> TO <system> [params] AS <out>
SQL "<any SQLite statement(s)>"
MOVIE CREATE <dataset> [AS <name>] | DELETE <name> | LIST | PLAY <name>
```

plus 28 value-level coordinate builtins (`cart_to_spherical(v)`,
`toroidal_to_cart(q, a)`, `cart2_to_polar(u)`, ...) covering all
fourteen classical curvilinear systems, every one with checked errors
(grammar.md §5.15).

`SQL` passes its string to SQLite **verbatim**, which is what makes the
COMPLETE SQLite command language available from a notebook: SELECT
with joins, subqueries, CTEs and window functions, INSERT, UPDATE,
DELETE, CREATE/ALTER/DROP, ATTACH/DETACH, PRAGMA, VACUUM, EXPLAIN,
transactions. The sidecar registers `sqrt`, `pow` and `hypot` as
deterministic SQL functions. `DATASET PLAY` renders the recording into
a self-contained plots page (energy, |P| and |L|, per-body positions,
top view) and opens it in the web browser; `MOVIE CREATE` derives a
self-contained 3-D player page from the tables (scrub bar, speeds,
orbiting camera, trails, labels, live t/E/|P|/|L| readout) and
`MOVIE PLAY` opens it. Movies are pure derivations: delete and rebuild
them at will, the physics never re-runs.

**Every Jupyter notebook owns a dataset.** Each of the 128 notebooks
creates `datasets/nb_<name>.db` in its section 6 (the sixteen dataset
worked examples manage their own, named in the script), records its
run, reads it back in section 9 three ways — `dataset snapshot`, the
`sql` command, and a **plain-Python `sqlite3` cell that opens the very
file the simulator still holds open** (the WAL cross-process case) —
and derives its movie in section 10. The notebook checker enforces
this as requirement R8.

## 3. Everything measured, in one table

| gate | command | measured result |
|---|---|---|
| sidecar tests | `cd dataset_tools && cargo test` | **11 passed, 0 failed** |
| 10-thread WAL stress | `./dataset_tools/target/release/solveit_db stress /tmp/s.db 10 500` | `5000 expected, 5000 written, journal_mode=wal` |
| workspace build | `cargo build --workspace --all-targets` | warning-free |
| workspace tests | `cargo test --workspace` | **638 passed, 0 failed** (was 622) |
| physics byte-identity | `bash tools/macos_verify_physics.sh` | exit 0 — collisions IDENTICAL; examples and dynamic notebooks identical modulo the two long-pinned host-libm diffs |
| 6 compiled examples | `cargo run -p physical_object --release --example <name>` | all print SUCCESS |
| 12 collision scripts | loop below | all ok |
| 38 SolveIt scripts | loop below | all ok (01–38; 20–38 are the dataset examples, 36–38 the trajectory-algebra / coordinate examples) |
| 128 notebooks executed | `nbrun` loop below | **128 ok, 0 failed** |
| 128 notebooks checked | `python3 notebooks/_build/nbcheck.py` | **128/128 pass all eight requirements** |
| recorded videos (legacy) | `python3 recorder/src/record_all.py --check` | all 13 byte-identical |
| 13 live GUI pages | `python3 tools/gui_smoke.py` | 13 of 13 pass |
| wire protocol | `python3 jupyter/test_protocol.py` | all checks passed |
| JupyterLab kernel | `POSIM_NO_BROWSER=1 jupyter/.venv/bin/python jupyter/test_kernel.py` | all checks passed |
| grammar.pdf | `pdflatex -interaction=nonstopmode grammar.tex` (×2) | 75 pages, 0 errors |
| SolveIt.pdf | `pdflatex -interaction=nonstopmode SolveIt.tex` (×2) | 48 pages, 0 errors |
| entity index | `python3 tools/verify_index_examples.py` | **PASS 1365/1365 (100.0%)**; tier-B `COMPILED 258/258 (100.0%)`; 123 command entries + 28 coordinate-builtin entries |
| encyclopedia | `python3 SolveIt_Notebooks_for_rust/_tools/verify.py` | **ALL VERIFY GATES PASS (147 notebooks)** — every posim-family copy re-executed ok, 124 GUI captures, 12.9 MB PDF rebuilt |

Physics answered **by SQL** inside the sixteen new worked examples
(complete transcripts in `SolveIt.md` §7, examples 20–35):

- Kepler two-body: perihelion `1.000000001093295`, aphelion
  `4.000000026379791` against the analytic 1 and 4 — via a self-join.
- Free fall vs the closed form: worst error `3.552713678800501e-15` —
  via a CTE.
- Restitution ladder e = 1.0 / 0.7 / 0.5: rebound speeds
  `10.000000000000028 / 7.000000000000018 / 5.000000000000014` — via
  GROUP BY.
- Helical cyclotron: gyroradius `0.5555555528977862` (5/9), pitch
  `0.6981317007977318` (v_z·T) — via aggregates.
- Dzhanibekov flip: |L| **bit-identical** (`MIN = MAX =
  3.0001333303705024`), 3 zero-crossings counted — via `LAG(...) OVER`.
- GGL pendulum rod stretch over a full period: `7.838174553853605e-14`.
- Adams vs BDF, two databases, one query: worst gap
  `5.615900633415549e-9` — via `ATTACH` + JOIN.
- Backward-difference error vs recorded velocity:
  `0.24525000000025088` against the theoretical g·dt/2 = `0.24525`.
- Recorded quaternions stay unit to `4.440892098500626e-16`.

And the second wave — trajectory algebra and the fourteen curvilinear
coordinate systems (examples 36–38, `grammar.md` §5.15):

- The teacher's experiment closes: `(A − B) + B − A` leaves
  `6.938893903907228e-18` in position, exactly `0` in energy.
- A planar orbit's spherical polar angle:
  `theta_min = theta_max = 1.5707963267948966` (pi/2 to the bit).
- Polar radial range `[0.7500000019886397, 3.0000000158618207]` vs the
  analytic `[0.75, 3]`.
- Every value-level round trip `X_to_cart(cart_to_X(w))` within
  `2.2e-16`; the 2-D charts return their inputs verbatim; singular
  loci (foci, focal rings) come back as NAMED ERRORS, never NaNs.

## 4. The complete command set (build → test → verify → execute → display)

```bash
cd /path/to/rustSolveIt_macos-silicon_SUNDIALS_7_8_0

# ---- build --------------------------------------------------------------
cargo build --workspace --all-targets
cargo build --release --workspace --all-targets
cd dataset_tools && cargo build --release && cd ..

# ---- test ---------------------------------------------------------------
cargo test --workspace                          # 631 expected
cd dataset_tools && cargo test && cd ..         # 11 expected
./dataset_tools/target/release/solveit_db stress /tmp/stress_demo.db 10 500

# ---- verify the physics is untouched -------------------------------------
bash tools/macos_verify_physics.sh              # must exit 0

# ---- execute every example ------------------------------------------------
cargo run -p physical_object --release --example kepler_orbit
cargo run -p physical_object --release --example outer_solar_system
cargo run -p physical_object --release --example tumbling_body
cargo run -p physical_object --release --example charged_in_b_field
cargo run -p physical_object --release --example newtons_cradle
cargo run -p physical_object --release --example bouncing_ball_restitution
for f in scripts/collisions/*.posim; do
  POSIM_NO_BROWSER=1 ./target/release/posim --script "$f" || echo "FAILED $f"
done
for f in scripts/solveit/*.posim; do
  POSIM_NO_BROWSER=1 ./target/release/posim --script "$f" || echo "FAILED $f"
done

# ---- regenerate, execute and check every notebook --------------------------
python3 notebooks/_build/regen.py
ls notebooks/*.ipynb | POSIM_NO_BROWSER=1 xargs -P 6 -I{} \
    python3 notebooks/_build/nbrun.py {}
python3 notebooks/_build/nbcheck.py

# ---- the other front ends ---------------------------------------------------
python3 recorder/src/record_all.py --check
python3 tools/gui_smoke.py
python3 jupyter/test_protocol.py
POSIM_NO_BROWSER=1 jupyter/.venv/bin/python jupyter/test_kernel.py
#   (venv, once:  uv venv jupyter/.venv
#                 uv pip install -p jupyter/.venv/bin/python ipykernel jupyter_client)

# ---- the documents -----------------------------------------------------------
pdflatex -interaction=nonstopmode grammar.tex
pdflatex -interaction=nonstopmode grammar.tex
pdflatex -interaction=nonstopmode SolveIt.tex
pdflatex -interaction=nonstopmode SolveIt.tex

# ---- the entity index ----------------------------------------------------------
python3 tools/extract_rust_items.py > index_data/rust_items.jsonl
python3 tools/build_commands.py
python3 tools/build_tierb.py
python3 tools/build_tierc.py
python3 tools/build_catalog.py
python3 tools/build_app.py
python3 tools/verify_index_examples.py
python3 tools/verify_tierb_examples.py

# ---- the encyclopedia (144 notebooks) --------------------------------------------
python3 SolveIt_Notebooks_for_rust/_tools/gen.py
ls SolveIt_Notebooks_for_rust/0[2-6]*/*/*.ipynb | xargs -P 4 -I{} \
    python3 SolveIt_Notebooks_for_rust/_tools/run_copy.py {}
POSIM_NO_BROWSER=1 python3 SolveIt_Notebooks_for_rust/_tools/shoot.py
python3 SolveIt_Notebooks_for_rust/_tools/encyclo.py
python3 SolveIt_Notebooks_for_rust/_tools/verify.py

# ---- display (opens your web browser) ----------------------------------------------
cargo run -p posim --release
#   then type, at the In[]:= prompt:
#     dataset play solveit20_kepler        <- time-series plots of the recording
#     movie play solveit20_kepler          <- the 3-D movie derived from it
#     scene create                          <- the live scene window (any session)
# or open the generated pages directly:
open datasets/solveit20_kepler_plots.html
open movies/solveit20_kepler.html
```

## 5. What changed on disk

| where | what |
|---|---|
| `dataset_tools/` | NEW — the `solveit_db` sidecar (rusqlite + r2d2 + r2d2_sqlite; WAL; busy timeout; serve protocol; CLI; 11 tests; README with schema and protocol) |
| `posim/src/dataset.rs`, `posim/src/movie.rs` | NEW — the language side: sidecar handle, recording payloads, the plots page, the movie player |
| `posim/src/{lexer,parser,vm,main}.rs` | the three contextual keywords, the EBNF, the instruction set, HELP, the RUN/STEP recording hooks, RESET closing the dataset |
| `physical_object/src/integrate.rs` | `Snapshot` additionally carries per-body states + the collision count (observational only — the byte-identity gate still passes) |
| `scripts/solveit/20..35_*.posim` | NEW — the sixteen dataset worked examples |
| `datasets/`, `movies/` | NEW — one dataset per notebook and per example, the plots pages, and the derived movies |
| `notebooks/` (125) + `notebooks/_build/` | every notebook owns a dataset (sections 6/9/10); `nbdataset.py` NEW; checker requirement R8 |
| `grammar.md` §5.14+§13, `grammar.tex`, `grammar.pdf` | the full command reference + a complete captured worked session (72 pp) |
| `SolveIt.md` §7 (20–35) + §8.3–8.4, `SolveIt.tex`, `SolveIt.pdf` | the sixteen additional fully documented examples + the dataset/movie guide (44 pp) |
| `ARCHITECTURE.md` §3.10 | the pinned dataset-subsystem contracts |
| `CLAUDE.md`, `README.md`, `NOTEBOOKS.md`, `notebooks/README.md`, `THIRD_PARTY.md`, `VERIFICATION_MACOS.md`, `MACOS_PORT_COMPLETE.md` | brought up to date (counts, layout, licences, dated verification addenda) |
| `tools/build_commands.py` + `index_data/` + `index_of_entities.html` | the 16 new command entries, catalogued and 100 % example-verified |
| `SolveIt_Notebooks_for_rust/` | the encyclopedia regenerated at 144 entries, every posim-family copy re-executed with its dataset sections |
| `SQLITE_DATASETS_PROVENANCE.md` | NEW — the complete build/test/verify/execute/display command record with captured outputs |

## 6. Design decisions, stated plainly

1. **Where rusqlite/r2d2 live.** The repository's standing rule is zero
   external dependencies in the simulator workspace; the dataset
   specification names rusqlite + r2d2. Both are honoured: the crates
   live only in `dataset_tools/`, which is excluded from the workspace,
   and posim reaches SQLite by spawning `solveit_db` — never by
   linking. `cargo build --workspace` does not touch crates.io.
2. **`DATASET CREATE` replaces; `RESET` closes.** Re-running any script
   or notebook reproduces its dataset bit-for-bit in content; COPY and
   SAVEAS (`VACUUM INTO`) are the preservation idioms; a dataset
   describes ONE world, so the command that destroys the world closes
   its dataset.
3. **Movies are derived artifacts.** The dataset is the source of
   truth; a movie can be deleted and rebuilt without re-running
   physics, and one dataset can carry several movies. The thirteen
   legacy `videos/*.html` recorder pages remain and still reproduce
   byte-for-byte.
4. **No wall-clock anywhere in a dataset.** Determinism is a verified
   repository property and stays one.
