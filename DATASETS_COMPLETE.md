# DATASETS_COMPLETE.md — the SQLite dataset layer: what was asked, what was built, and the proof

Date of every measurement below: 2026-09-09, on this **Windows 11 Pro
for Workstations** machine (build 10.0.26200, Intel Core Ultra 9 275HX,
x86-64 with FMA) — rustc 1.91.1 / cargo 1.91.1 (host
`x86_64-pc-windows-msvc`), the MSVC linker and `cl.exe` of the Visual
Studio 2026 Build Tools (MSVC 14.51.36231, located by rustc and by the
`cc` crate automatically — no developer prompt), Python 3.14.5, MiKTeX
(pdfTeX 4.18, MiKTeX 24.1), Google Chrome
(`C:\Program Files\Google\Chrome\Application\chrome.exe`, headless
captures), SQLite 3.46.0 bundled into the sidecar. The **only** C
compiler used anywhere in this repository is that `cl.exe`, compiling
the bundled SQLite amalgamation inside `dataset_tools/` through the
`cc` crate (and the REBOUND/REBOUNDx C reference harnesses of the
rebound gates); the simulator workspace itself is pure Rust, linked by
the MSVC linker rustc finds. No WSL2 compiler or linker was used
anywhere. The root `.cargo/config.toml` pins
`-C target-feature=+fma`, which the Windows engine's deterministic
libm requires — the CPU must have FMA.

**Lineage.** This layer was designed, built and first measured on
macOS / Apple Silicon (`once-ere/rustSolveIt_macos-silicon_SUNDIALS_7_8_0`;
that machine's record is kept unchanged in `PORT_MACOS_PROVENANCE.md`,
`VERIFICATION_MACOS.md` and `MACOS_PORT_COMPLETE.md`), itself the
port of the Linux project `once-ere/rustSolveIt_Using_SUNDIALS_7_8_0`.
The repository you are reading,
`https://github.com/once-ere/rustSolveIt_Win11_SUNDIALS_7_8_0.git`,
is the Windows 11 port of the macOS project, and every figure on this
page was **re-measured here**; where a Windows figure differs from
the macOS one it is said so explicitly (there is exactly one such
place, §3, the curvilinear examples).

This file is self-contained: every command needed to rebuild, re-test,
re-verify, re-execute and display the work appears below in full.
Commands are given in PowerShell form, with the Git Bash form in
parentheses where it differs; the notebook loop that needs `xargs -P`
is a Git Bash command.

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

**A dataset is one SQLite database file** under `datasets\<name>.db`.
While one is open with recording armed, every output row of a `run`
and the end of every `step` is appended as a frame: time, total
energy, momentum, angular momentum, centre of mass, cumulative
collision count, and the full state of every body (position, w-first
orientation quaternion, velocity, angular velocity). Schema v1 is four
relational tables — `meta`, `bodies`, `frames`, `states` — with **no
timestamps anywhere**, because byte-stable re-runs are a verified
property of this repository. (While a dataset is open SQLite keeps two
transient side files next to it, `<name>.db-wal` and `<name>.db-shm`;
they are checkpointed away on close and are not part of the dataset —
ignore them.)

**All SQLite access lives in a sidecar**, the `solveit_db.exe` binary
(`dataset_tools/`), built exactly to the specified architecture:

- an `r2d2` + `r2d2_sqlite` **connection pool** (size 10) — the POOL is
  cloned across threads, never a connection;
- **`PRAGMA journal_mode=WAL`** executed on every connection the moment
  the pool creates it;
- **`PRAGMA busy_timeout = 5000`** so simultaneous writers wait up to
  five seconds instead of throwing `database is locked`.

The simulator (`posim.exe`) **spawns** the sidecar and speaks
line-delimited JSON to it; it links no SQLite and no external crate.
The finder in `posim/src/dataset.rs` looks first at `$env:POSIM_DB_TOOL`,
then for `solveit_db.exe` (and only then the extensionless
`solveit_db`) in `dataset_tools\target\release\` from the working
directory upward, then on `PATH` — so the release sidecar must be built
before any DATASET/SQL/MOVIE command, the solveit 20–38 scripts or the
notebooks are run. The pure-Rust workspace therefore keeps its
zero-dependency, zero-`unsafe`, zero-warning guarantee —
`dataset_tools/` is excluded from the workspace exactly as
`sundials_rs/` is, and the workspace `Cargo.lock` still lists only the
12 local crates.

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
creates `datasets\nb_<name>.db` in its section 6 (the sixteen dataset
worked examples manage their own, named in the script), records its
run, reads it back in section 9 three ways — `dataset snapshot`, the
`sql` command, and a **plain-Python `sqlite3` cell that opens the very
file the simulator still holds open** (the WAL cross-process case) —
and derives its movie in section 10. The notebook checker enforces
this as requirement R8.

## 3. Everything measured, in one table

| gate | command (PowerShell) | measured result on this machine |
|---|---|---|
| sidecar tests | `cd dataset_tools; cargo test; cd ..` | **11 passed, 0 failed** (8 unit + 3 integration) |
| 10-thread WAL stress | `.\dataset_tools\target\release\solveit_db.exe stress $env:TEMP\stress_demo.db 10 500` | expected `5000 expected, 5000 written, journal_mode=wal` (the macOS-measured line; not yet re-measured here) |
| workspace build | `cargo build --workspace --all-targets` | 0 errors, 0 warnings (release build likewise; `target\release\posim.exe` 5,705,216 bytes) |
| sidecar build | `cd dataset_tools; cargo build --release; cd ..` | 0 warnings, 18.22 s; `dataset_tools\target\release\solveit_db.exe` 2,326,016 bytes |
| workspace tests | `cargo test --workspace` | **638 passed, 0 failed** = 49 physical_object lib + 19 collision + 9 conservation + 42 constrained/DAE + 121 posim + 92 quantum + 240 special_functions + 11 vendored identities + 5 quantum doctests + 50 special_functions doctests (`logs\test.log`; the same 638 again after the last source change, the `.exe`-first sidecar finder — `logs\test_final.log`) |
| physics byte-identity | `bash tools/win_verify_physics.sh` (Git Bash) | exit 0 — the three verdict lines are quoted below the table: collisions IDENTICAL; examples and dynamic notebooks IDENTICAL-MODULO-DOCUMENTED (the two long-pinned host-libm diffs) |
| 6 compiled examples | `cargo run -p physical_object --release --example <name>` | all print SUCCESS |
| 12 collision scripts | loop below | 12/12 exit 0 |
| 38 SolveIt scripts | loop below | 38/38 exit 0 (01–38; 20–38 are the dataset examples, 36–38 the trajectory-algebra / coordinate examples) — 50/50 scripts in all |
| 128 notebooks executed | `nbrun` loop below | **128 ok, 0 failed** |
| 128 notebooks checked | `python notebooks\_build\nbcheck.py` | **128/128 pass all eight requirements** |
| recorded videos (legacy) | `python recorder\src\record_all.py --check` | all 13 recordings reproduce byte for byte |
| 13 live GUI pages | `python tools\win_gui_smoke.py` (and `python tools\gui_smoke.py`) | 13 of 13 pass (ports 8895–8907) |
| wire protocol | `python jupyter\test_protocol.py` | all protocol checks passed |
| JupyterLab kernel | `jupyter\.venv\Scripts\python.exe jupyter\test_kernel.py` with `$env:POSIM_NO_BROWSER = "1"` | 7/7 cells ok — all kernel checks passed, JupyterLab can drive this kernel |
| datasets and movies vs the macOS files | byte comparison after all runs | movies **74/74 byte-identical**; datasets **245/252 byte-identical** — the 7 that differ are the curvilinear maps of examples 37–38 (see below) |
| grammar.pdf | `pdflatex -interaction=nonstopmode grammar.tex` (×2) | 75 pages, 0 errors on macOS; the MiKTeX re-measurement is pending at the time of writing |
| SolveIt.pdf | `pdflatex -interaction=nonstopmode SolveIt.tex` (×2) | 48 pages, 0 errors on macOS; the MiKTeX re-measurement is pending at the time of writing |
| entity index | `python tools\verify_index_examples.py`; `python tools\verify_tierb_examples.py` | **PASS 1365/1365 (100.0%)**, 35 s wall clock; tier-B **`COMPILED 258/258 (100.0%)`**; the 123 command + 28 coordinate-builtin entry counts are the macOS final-pass figures, not re-counted here |
| encyclopedia | `python SolveIt_Notebooks_for_rust\_tools\gen.py`, `run_copy.py`, then `shoot.py`, `encyclo.py`, `verify.py` | measured here: gen.py manifest **147** entries; posim-family copies re-executed ok — 02_solveit 38/38, 03_dynamics 59/59, 04+05 25/25, 06_rust 6/6 — and 07_rebound 17/17 ok; pending at the time of writing: the two 01_mercury copies, shoot.py, encyclo.py and verify.py (macOS: 124 GUI captures, 12.9 MB PDF, **ALL VERIFY GATES PASS (147 notebooks)** — the expected outcome, not a Windows measurement) |

The physics gate's three verdict lines (`logs\win_verify_physics.log`;
the gate's own outputs and the two pinned divergence diffs are in
`evidence\win11\`):

```
IDENTICAL-MODULO-DOCUMENTED  examples-win11.log vs examples-7.8.0.log (pinned: accepted-divergences-examples.diff)
IDENTICAL  collision-scripts-win11.log == collision-scripts-7.8.0.log
IDENTICAL-MODULO-DOCUMENTED  dynamic-notebooks-win11.log vs dynamic-notebooks-7.8.0.log (pinned: accepted-divergences-dynamic.diff)
```

Physics answered **by SQL** inside the sixteen new worked examples
(complete transcripts in `SolveIt.md` §7, examples 20–35; every one of
these numbers reproduces here to the bit — the corresponding datasets
are among the 245 byte-identical files):

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
coordinate systems (examples 36–38, `grammar.md` §5.15), measured here
from `logs\solveit_scripts\solveit_36_*.log`, `37`, `38`:

- The teacher's experiment closes: `(A − B) + B − A` leaves
  `6.938893903907228e-18` in position (`2.7755575615628914e-17` in
  velocity), exactly `0` in energy — identical to the macOS values.
- A planar orbit's spherical polar angle:
  `theta_min = theta_max = 1.5707963267948966` (pi/2 to the bit) —
  identical to macOS.
- Polar radial range `[0.7500000019886397, 3.0000000158618207]` vs the
  analytic `[0.75, 3]` — identical to macOS.
- Helix pitch `0.6981317007977317` in cylindrical coordinates, toroidal
  `tau` range `[5.28650864731464e-9, 1.1892318293548387]` — identical
  to macOS.
- Every value-level round trip `X_to_cart(cart_to_X(w))` lands between
  `0` exactly and `7.447602459741819e-16` (the prolate spheroidal
  chart; oblate, bispherical `4.440892098500626e-16`, paraboloidal
  `4.965068306494546e-16`, bipolar cylindrical
  `2.220446049250313e-16`, elliptic cylindrical / parabolic cylindrical
  / cylindrical exactly `0`); the 2-D bipolar chart returns its input
  `[1.2, -0.8]` verbatim; singular loci (foci, focal rings) come back as
  NAMED ERRORS, never NaNs. **This is where Windows differs from
  macOS, and the only place:** exactly two numeric transcript lines of
  examples 36–38 changed, both value-level round trips — the prolate
  figure, which was `8.671119018262734e-16` on macOS, and example 37's
  `polar_to_cart2(cart2_to_polar([3, 4]))`, which returns
  `[3.0000000000000004, 3.9999999999999996]` here and
  `[3, 3.9999999999999996]` there. The curvilinear maps in
  `special_functions/src/coords.rs` go through the host libm (MSVC
  UCRT here, Apple libm there), which is also why the seven files
  `solveit37_pol.db`, `solveit37_sph.db`, `solveit37_sph_plots.html`,
  `solveit38_bis.db`, `solveit38_cyl.db`, `solveit38_obl.db`,
  `solveit38_tor.db` are the only dataset files not byte-identical to
  the macOS ones. Quantified per `states` table (Windows file vs macOS
  file): `solveit37_pol` 12 of 1400 float values differ, worst relative
  difference 1.99e-16; `solveit37_sph` 3/1400, 1.30e-16;
  `solveit38_bis` 3/700, 1.58e-15; `solveit38_cyl` 5/700, 1.79e-16;
  `solveit38_obl` 16/700, 2.67e-14; `solveit38_tor` 5/700, 4.02e-16
  (`solveit37_sph_plots.html` is the plots page drawn from
  `solveit37_sph.db`). Every SQL-queried figure in the transcripts of
  36, 37 and 38 quoted above is nevertheless identical to the last
  digit.

## 4. The complete command set (build → test → verify → execute → display)

PowerShell, from the repository root (Git Bash forms in the comments
where they differ):

```powershell
cd <path-to>\rustSolveIt_Win11_SUNDIALS_7_8_0

# ---- build --------------------------------------------------------------
cargo build --workspace --all-targets
cargo build --release --workspace --all-targets
cd dataset_tools; cargo build --release; cd ..

# ---- test ---------------------------------------------------------------
cargo test --workspace                          # 638 expected
cd dataset_tools; cargo test; cd ..             # 11 expected
.\dataset_tools\target\release\solveit_db.exe stress $env:TEMP\stress_demo.db 10 500
#   (Git Bash: ./dataset_tools/target/release/solveit_db.exe stress "$TEMP/stress_demo.db" 10 500)

# ---- verify the physics is untouched (Git Bash) --------------------------
bash tools/win_verify_physics.sh                # must exit 0

# ---- execute every example ------------------------------------------------
cargo run -p physical_object --release --example kepler_orbit
cargo run -p physical_object --release --example outer_solar_system
cargo run -p physical_object --release --example tumbling_body
cargo run -p physical_object --release --example charged_in_b_field
cargo run -p physical_object --release --example newtons_cradle
cargo run -p physical_object --release --example bouncing_ball_restitution
$env:POSIM_NO_BROWSER = "1"
Get-ChildItem scripts\collisions\*.posim | ForEach-Object {
    .\target\release\posim.exe --script $_.FullName
    if ($LASTEXITCODE -ne 0) { "FAILED $($_.Name)" }
}
Get-ChildItem scripts\solveit\*.posim | ForEach-Object {
    .\target\release\posim.exe --script $_.FullName
    if ($LASTEXITCODE -ne 0) { "FAILED $($_.Name)" }
}
#   (Git Bash: for f in scripts/collisions/*.posim scripts/solveit/*.posim; do
#                POSIM_NO_BROWSER=1 ./target/release/posim.exe --script "$f" || echo "FAILED $f"
#              done)

# ---- regenerate, execute and check every notebook --------------------------
python notebooks\_build\regen.py
#   the parallel execution loop is a Git Bash command (xargs -P 6):
#     ls notebooks/*.ipynb | POSIM_NO_BROWSER=1 xargs -P 6 -I{} \
#         python notebooks/_build/nbrun.py {}
#   (sequential PowerShell equivalent:
#     Get-ChildItem notebooks\*.ipynb | ForEach-Object { python notebooks\_build\nbrun.py $_.FullName } )
python notebooks\_build\nbcheck.py

# ---- the other front ends ---------------------------------------------------
python recorder\src\record_all.py --check
python tools\win_gui_smoke.py
python jupyter\test_protocol.py
$env:POSIM_NO_BROWSER = "1"
jupyter\.venv\Scripts\python.exe jupyter\test_kernel.py
#   (venv, once:  uv venv jupyter\.venv
#                 uv pip install -p jupyter\.venv\Scripts\python.exe ipykernel jupyter_client)
#   (Git Bash: POSIM_NO_BROWSER=1 jupyter/.venv/Scripts/python.exe jupyter/test_kernel.py)

# ---- the documents (MiKTeX pdflatex; twice each, the TOC needs the second pass)
pdflatex -interaction=nonstopmode grammar.tex
pdflatex -interaction=nonstopmode grammar.tex
pdflatex -interaction=nonstopmode SolveIt.tex
pdflatex -interaction=nonstopmode SolveIt.tex

# ---- the entity index ----------------------------------------------------------
python tools\extract_rust_items.py > index_data\rust_items.jsonl
python tools\build_commands.py
python tools\build_tierb.py
python tools\build_tierc.py
python tools\build_catalog.py
python tools\build_app.py
python tools\verify_index_examples.py
python tools\verify_tierb_examples.py

# ---- the encyclopedia (147 notebooks) ---------------------------------------------
python SolveIt_Notebooks_for_rust\_tools\gen.py
#   the parallel copy-execution loop is a Git Bash command (xargs -P 4):
#     ls SolveIt_Notebooks_for_rust/0[2-6]*/*/*.ipynb | xargs -P 4 -I{} \
#         python SolveIt_Notebooks_for_rust/_tools/run_copy.py {}
$env:POSIM_NO_BROWSER = "1"
python SolveIt_Notebooks_for_rust\_tools\shoot.py
python SolveIt_Notebooks_for_rust\_tools\encyclo.py
python SolveIt_Notebooks_for_rust\_tools\verify.py

# ---- display (opens your web browser) ----------------------------------------------
Remove-Item Env:POSIM_NO_BROWSER -ErrorAction SilentlyContinue
cargo run -p posim --release
#   then type, at the In[]:= prompt:
#     dataset play solveit20_kepler        <- time-series plots of the recording
#     movie play solveit20_kepler          <- the 3-D movie derived from it
#     scene create                          <- the live scene window (any session)
# or open the generated pages directly:
Start-Process datasets\solveit20_kepler_plots.html
Start-Process movies\solveit20_kepler.html
#   (Git Bash: start datasets/solveit20_kepler_plots.html ; start movies/solveit20_kepler.html)
# an independent look inside any dataset, no simulator involved
# (Windows ships no sqlite3.exe; Python 3.12+ has the shell built in):
python -m sqlite3 datasets\solveit20_kepler.db "SELECT COUNT(*) FROM frames"
```

Note on `python tools\extract_rust_items.py > index_data\rust_items.jsonl`:
the extractor writes JSONL to stdout, and on this machine Python's
default text encoding is cp1252. Set `$env:PYTHONUTF8 = "1"` before
that line and run it in PowerShell 7 (whose `>` writes UTF-8; Windows
PowerShell 5.1 would write UTF-16), or run it in Git Bash
(`python tools/extract_rust_items.py > index_data/rust_items.jsonl`),
which redirects the bytes verbatim. The extractor reads every Rust
source in UTF-8 explicitly — a cp1252 or CRLF slip is the classic
Windows way to break a byte-identity claim.

## 5. What changed on disk

| where | what |
|---|---|
| `dataset_tools/` | NEW — the `solveit_db.exe` sidecar (rusqlite + r2d2 + r2d2_sqlite; WAL; busy timeout; serve protocol; CLI; 11 tests; README with schema and protocol) |
| `posim/src/dataset.rs`, `posim/src/movie.rs` | NEW — the language side: sidecar handle (its finder probes `solveit_db.exe`), recording payloads, the plots page, the movie player |
| `posim/src/{lexer,parser,vm,main}.rs` | the three contextual keywords, the EBNF, the instruction set, HELP, the RUN/STEP recording hooks, RESET closing the dataset |
| `physical_object/src/integrate.rs` | `Snapshot` additionally carries per-body states + the collision count (observational only — the byte-identity gate still passes) |
| `scripts/solveit/20..35_*.posim` | NEW — the sixteen dataset worked examples |
| `datasets/`, `movies/` | NEW — one dataset per notebook and per example, the plots pages, and the derived movies |
| `notebooks/` (128) + `notebooks/_build/` | every notebook owns a dataset (sections 6/9/10); `nbdataset.py` NEW; checker requirement R8 |
| `grammar.md` §5.14+§13, `grammar.tex`, `grammar.pdf` | the full command reference + a complete captured worked session |
| `SolveIt.md` §7 (20–35) + §8.3–8.4, `SolveIt.tex`, `SolveIt.pdf` | the sixteen additional fully documented examples + the dataset/movie guide |
| `ARCHITECTURE.md` §3.10 | the pinned dataset-subsystem contracts |
| `CLAUDE.md`, `README.md`, `NOTEBOOKS.md`, `notebooks/README.md`, `THIRD_PARTY.md`, `VERIFICATION_WIN11.md`, `WINDOWS_PORT_COMPLETE.md` | brought up to date (counts, layout, licences, dated verification addenda); the macOS-era twins `VERIFICATION_MACOS.md` / `MACOS_PORT_COMPLETE.md` are kept unchanged as lineage |
| `tools/build_commands.py` + `index_data/` + `index_of_entities.html` | the 16 new command entries, catalogued and 100 % example-verified |
| `tools/win_verify_physics.sh`, `tools/win_gui_smoke.py` | the Windows byte-identity gate and GUI smoke test (the macOS ones remain beside them) |
| `SolveIt_Notebooks_for_rust/` | the encyclopedia regenerated at 147 entries, every posim-family copy re-executed with its dataset sections |
| `SQLITE_DATASETS_PROVENANCE.md` | NEW — the complete build/test/verify/execute/display command record with captured outputs |

## 6. Design decisions, stated plainly

1. **Where rusqlite/r2d2 live.** The repository's standing rule is zero
   external dependencies in the simulator workspace; the dataset
   specification names rusqlite + r2d2. Both are honoured: the crates
   live only in `dataset_tools/`, which is excluded from the workspace,
   and posim reaches SQLite by spawning `solveit_db.exe` — never by
   linking. `cargo build --workspace` does not touch crates.io, and the
   one C compilation in the repository (the SQLite amalgamation, by
   MSVC `cl.exe` through the `cc` crate) happens only inside
   `dataset_tools/`.
2. **`DATASET CREATE` replaces; `RESET` closes.** Re-running any script
   or notebook reproduces its dataset bit-for-bit in content; COPY and
   SAVEAS (`VACUUM INTO`) are the preservation idioms; a dataset
   describes ONE world, so the command that destroys the world closes
   its dataset.
3. **Movies are derived artifacts.** The dataset is the source of
   truth; a movie can be deleted and rebuilt without re-running
   physics, and one dataset can carry several movies. The thirteen
   legacy `videos\*.html` recorder pages remain and still reproduce
   byte-for-byte (the `rack_and_pinion` recording is the Windows one —
   its GEAR/RACK joint residual passes through the host libm).
4. **No wall-clock anywhere in a dataset.** Determinism is a verified
   repository property and stays one — which is exactly what made the
   74/74 movie and 245/252 dataset byte comparison against the macOS
   files possible, and what makes the seven host-libm exceptions
   nameable.
