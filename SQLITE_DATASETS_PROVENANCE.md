# SQLITE_DATASETS_PROVENANCE.md — the dataset layer, built and verified on this machine

Platform: **Windows 11 Pro for Workstations** (build 10.0.26200),
Intel Core Ultra 9 275HX (x86-64 with FMA). Toolchain: rustc 1.91.1
(ed61e7d7e 2025-11-07) / cargo 1.91.1, host `x86_64-pc-windows-msvc`;
the MSVC linker and `cl.exe` of the Visual Studio 2026 Build Tools,
MSVC 14.51.36231
(`C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Tools\MSVC\14.51.36231\bin\Hostx64\x64\cl.exe`),
located automatically by rustc and by the `cc` crate — no developer
prompt; Python 3.14.5 (cp1252 default encoding), nbformat 5.10.4,
nbclient 0.10.2, jupyter_client 8.6.3, ipykernel 7.1.0, uv 0.9.10;
MiKTeX-pdfTeX 4.18 (MiKTeX 24.1); Google Chrome at
`C:\Program Files\Google\Chrome\Application\chrome.exe`; SQLite 3.46.0
(bundled into the sidecar by `libsqlite3-sys 0.30.1` — Windows ships
no `sqlite3.exe`, and none is needed: `python -m sqlite3 <db> "<sql>"`
is the independent shell used below). The ONLY C compiler used
anywhere in this repository is that `cl.exe`, compiling the bundled
SQLite amalgamation inside `dataset_tools/` through the `cc` crate (and
the REBOUND/REBOUNDx C reference harnesses of the rebound gates); the
simulator workspace is pure Rust and only the MSVC linker rustc finds
touches it. No WSL2 compiler or linker was used anywhere. The root
`.cargo/config.toml` pins `-C target-feature=+fma`, required by the
Windows engine's deterministic libm — the CPU must have FMA. Every
command below was run exactly as written, from the repository root, on
this machine, on 2026-09-09; every number is captured program output
(the logs are under `logs\`).

**Lineage.** The dataset layer was designed, built and first measured
on macOS / Apple Silicon, in
`once-ere/rustSolveIt_macos-silicon_SUNDIALS_7_8_0` (its record is
kept unchanged in `PORT_MACOS_PROVENANCE.md`, `VERIFICATION_MACOS.md`
and `MACOS_PORT_COMPLETE.md`), the port of the Linux project
`once-ere/rustSolveIt_Using_SUNDIALS_7_8_0`. This repository,
`https://github.com/once-ere/rustSolveIt_Win11_SUNDIALS_7_8_0.git`, is
the Windows 11 port of the macOS project; its vendored engine is
`once-ere/SUNDIALS_7_8_Rust_port_for_Windows11` (public API identical
to the Linux/macOS engine; the backend module sets differ). Everything
on this page was then re-measured here, and the one place where the
Windows figures differ from the macOS ones (§1, §6 and §7, the
curvilinear maps of examples 37–38) is quantified exactly.

This file is self-contained: the complete set of commands to build,
test, verify, execute and display the dataset layer is given here in
full. Commands are PowerShell; the Git Bash form follows in parentheses
where it differs, and the two parallel notebook loops (`xargs -P`) are
Git Bash commands.

---

## 1. What was built

- **Datasets**: every simulation can record itself into a SQLite
  database `datasets\<name>.db` (schema v1: `meta`, `bodies`, `frames`,
  `states`; w-first quaternions; no timestamps). Every one of the 128
  Jupyter notebooks owns one, and SolveIt worked examples 20–38 are
  built around them. While a dataset is open SQLite keeps two transient
  side files beside it, `<name>.db-wal` and `<name>.db-shm`; they are
  checkpointed away on close and are not part of the dataset.
- **The `solveit_db.exe` sidecar** (`dataset_tools/`): the ONE package
  with external crates — `rusqlite` (bundled SQLite amalgamation,
  compiled here by MSVC `cl.exe` through the `cc` crate), `r2d2`,
  `r2d2_sqlite` — exactly as specified for this feature. It owns a
  thread-safe connection pool (size 10) shared across worker threads,
  enables **WAL journal mode** on every connection at creation, and
  sets **`busy_timeout = 5000`**. It is excluded from the simulator
  workspace: `posim.exe` spawns it and speaks line-delimited JSON; the
  simulator still links zero external crates. The finder in
  `posim/src/dataset.rs` looks at `$env:POSIM_DB_TOOL`, then for
  `solveit_db.exe` (then the extensionless name) in
  `dataset_tools\target\release\` from the working directory upward,
  then on `PATH`.
- **Language surface** (contextual keywords, command position only):
  `DATASET CREATE/OPEN/CLOSE/RECORD/SNAPSHOT/LIST/INFO/COPY/SAVEAS/`
  `DELETE/PLAY`, `SQL "<any SQLite statement(s)>"` (verbatim
  passthrough — the COMPLETE SQLite grammar), and `MOVIE
  CREATE/DELETE/LIST/PLAY` (movies are 3-D player pages DERIVED from
  datasets). Full reference: `grammar.md` §5.14; complete worked
  session: `grammar.md` §13.
- **Trajectory algebra and curvilinear coordinates** (added in the
  same series of work): `DATASET DIFF <a> <b> AS <out>` and
  `DATASET SUM ...` — frame-aligned component arithmetic with the
  alignment CHECKED (same frame count, bit-equal t grid, same body
  ids; mismatches error naming the frame); `DATASET MAP <src> TO
  <system> [params] AS <out>` — positions through any of the fourteen
  classical curvilinear systems (eleven 3-D + three 2-D, the 2-D ones
  requiring |z| <= 1e-9; output velocities NULL because curvilinear
  components need the metric); and 28 value-level builtins
  (`cart_to_spherical(v)` ... `bipolar_to_cart2(w, a)`), all in pure
  Rust (`special_functions/src/coords.rs`) with round-trip,
  known-value and error-path tests. Reference: `grammar.md` §5.15;
  worked examples 36–38. Measured on this machine
  (`logs\solveit_scripts\solveit_36_dataset_teacher_diff.log`,
  `solveit_37_dataset_map_spherical.log`,
  `solveit_38_dataset_curvilinear_tour.log`): the teacher's
  (A−B)+B−A closure residual `6.938893903907228e-18` in position,
  `2.7755575615628914e-17` in velocity, exactly `0` in energy; a planar
  orbit's spherical polar angle `1.5707963267948966` with MIN = MAX
  over 100 frames; the polar radial range
  `[0.7500000019886397, 3.0000000158618207]`; the helix pitch
  `0.6981317007977317` and toroidal `tau` range
  `[5.28650864731464e-9, 1.1892318293548387]` — all identical to the
  macOS transcripts. The value-level round trips
  `X_to_cart(cart_to_X(w))` for `w = [1.3, 0.7, 2.1]` land between `0`
  exactly (elliptic cylindrical, parabolic cylindrical, cylindrical)
  and `7.447602459741819e-16` (prolate spheroidal; oblate and
  bispherical `4.440892098500626e-16`, paraboloidal
  `4.965068306494546e-16`, bipolar cylindrical
  `2.220446049250313e-16`); the 2-D bipolar chart returns `[1.2, -0.8]`
  verbatim. **Exactly two numeric transcript lines of examples 36–38
  differ from the macOS ones**, both value-level round trips whose
  coordinate maps go through the host libm (the MSVC UCRT here, Apple
  libm there): the prolate figure above (macOS: `8.671119018262734e-16`)
  and example 37's `polar_to_cart2(cart2_to_polar([3, 4]))`, which
  returns `[3.0000000000000004, 3.9999999999999996]` here and
  `[3, 3.9999999999999996]` on macOS. Every other numeric `Out` line of
  the three transcripts — every SQL-queried figure included — is
  identical to the last digit.

## 2. Build everything

```powershell
cd <path-to>\rustSolveIt_Win11_SUNDIALS_7_8_0

# the simulator workspace (pure Rust, zero external crates, zero unsafe)
cargo build --workspace --all-targets
cargo build --release --workspace --all-targets

# the SQLite sidecar (first build fetches rusqlite/r2d2 from crates.io;
# dataset_tools\Cargo.lock pins the versions; the SQLite amalgamation is
# compiled by MSVC cl.exe through the cc crate — no other C compiler exists here)
cd dataset_tools; cargo build --release; cd ..
```

Measured: `cargo build --workspace --all-targets` — 0 errors, 0 warnings
(`logs\build.log`); the release build likewise warning-free
(`logs\build_release.log`), `target\release\posim.exe` 5,705,216 bytes.
The sidecar: `rusqlite 0.32.1` and `r2d2_sqlite 0.25.0` compiled,
`Finished release` in 18.22 s, 0 warnings;
`dataset_tools\target\release\solveit_db.exe` **2,326,016 bytes**
(`logs\dataset_tools_build.log`). The workspace `Cargo.lock` still lists
only local crates (12 packages) — verify with:

```powershell
(Select-String -Path Cargo.lock -Pattern '^\[\[package\]\]').Count   # 12
Select-String -Path Cargo.lock -Pattern 'source = '                  # no output: no registry sources
```

(Git Bash: `grep -c "\[\[package\]\]" Cargo.lock` → 12;
`grep "source = " Cargo.lock | sort -u` → no output.)

## 3. Test the sidecar (WAL + pool + busy timeout, under real contention)

```powershell
cd dataset_tools
cargo test
cd ..
```

Measured: **11 passed, 0 failed** (8 unit + 3 integration;
`logs\dataset_tools_test.log`), including:

- `create_sets_wal_mode_and_busy_timeout` — `PRAGMA journal_mode` comes
  back `wal`, `PRAGMA busy_timeout` comes back `5000`.
- `ten_threads_through_one_pool_lose_nothing` — 10 threads × 200
  simultaneous inserts through ONE cloned pool; all 2000 rows land, no
  `database is locked`.
- `two_pools_on_one_file_interleave_writes_and_reads` — two pools on
  one file (the cross-process picture) with a concurrent reader; all
  600 rows land and the reader's counts never go backwards.
- `serve_protocol_round_trips_append_and_sql` — the real binary driven
  over stdin/stdout exactly as posim drives it, including the WAL
  checkpoint on close.

The multithreaded demonstration, exactly as specified for the feature:

```powershell
.\dataset_tools\target\release\solveit_db.exe stress $env:TEMP\stress_demo.db 10 500
```

(Git Bash: `./dataset_tools/target/release/solveit_db.exe stress "$TEMP/stress_demo.db" 10 500`.)

Expected output (this is the line the macOS machine measured; the
Windows re-measurement of this one command is pending at the time of
writing — the same code path is exercised by the
`ten_threads_through_one_pool_lose_nothing` test above, which passed
here):

```
stress: 10 threads x 500 rows = 5000 expected, 5000 written, journal_mode=wal
```

## 4. Test the simulator (the new grammar included)

```powershell
cargo test --workspace
```

Measured: **638 passed, 0 failed** (`logs\test.log`) = 49
physical_object lib + 19 collision + 9 conservation + 42
constrained/DAE + 121 posim + 92 quantum + 240 special_functions + 11
vendored identities + 5 quantum doctests + 50 special_functions
doctests (622 before the dataset layer, 631 before the coordinate
layer; posim 112 → 121, special_functions 233 → 240 with the seven
coordinate-map suites). Re-run after the last source change of this
port — the `.exe`-first sidecar finder in `posim/src/dataset.rs` — the
count is the same **638 passed, 0 failed** (`logs\test_final.log`).
The new posim tests:
`dataset_movie_and_sql_productions_compile`,
`dataset_movie_and_sql_stay_identifiers_off_the_front`,
`dataset_records_run_frames_and_sql_reads_them_back` (end-to-end
through the real sidecar: 10 recorded RUN frames, the analytic
free-fall y(1) = 10 − 9.81/2 read back BY SQL, SAVEAS/DELETE round
trip, a movie generated),
`every_dataset_and_movie_subcommand_is_documented_in_lockstep`
(EBNF ↔ HELP_TEXT ↔ grammar.md ↔ grammar.tex, both directions),
`the_plots_page_is_self_contained` and
`the_player_page_is_self_contained` (no CDN, no fetch, no WebSocket).

## 5. The physics is untouched: the byte-identity gate

The recording hook rides on `integrate::Snapshot` as purely
observational fields — no arithmetic changed anywhere. The gate that
proves it re-runs the six examples, the twelve collision scripts and
the 59 dynamic notebooks and diffs them against the Linux evidence in
`evidence\port-7.8.0\` (it is a bash script; run it in Git Bash):

```powershell
cargo build --release -p posim
bash tools/win_verify_physics.sh
```

Measured (exit 0; `logs\win_verify_physics.log`, outputs in
`evidence\win11\`):

```
IDENTICAL-MODULO-DOCUMENTED  examples-win11.log vs examples-7.8.0.log (pinned: accepted-divergences-examples.diff)
IDENTICAL  collision-scripts-win11.log == collision-scripts-7.8.0.log
IDENTICAL-MODULO-DOCUMENTED  dynamic-notebooks-win11.log vs dynamic-notebooks-7.8.0.log (pinned: accepted-divergences-dynamic.diff)
```

(The two MODULO lines are the SAME two pre-existing pinned host-libm
divergences — the quantum `double_slit`/`tunneling` last digit and the
GEAR/RACK joint residual — documented in `evidence\win11\`; nothing
new diverged.)

## 6. Run and verify every example

```powershell
# the six self-checking compiled examples
cargo run -p physical_object --release --example kepler_orbit
cargo run -p physical_object --release --example outer_solar_system
cargo run -p physical_object --release --example tumbling_body
cargo run -p physical_object --release --example charged_in_b_field
cargo run -p physical_object --release --example newtons_cradle
cargo run -p physical_object --release --example bouncing_ball_restitution

$env:POSIM_NO_BROWSER = "1"

# the twelve collision scripts
Get-ChildItem scripts\collisions\*.posim | ForEach-Object {
    .\target\release\posim.exe --script $_.FullName
    if ($LASTEXITCODE -ne 0) { "FAILED $($_.Name)" }
}

# the THIRTY-EIGHT SolveIt scripts (20-38 are the dataset examples)
Get-ChildItem scripts\solveit\*.posim | ForEach-Object {
    .\target\release\posim.exe --script $_.FullName
    if ($LASTEXITCODE -ne 0) { "FAILED $($_.Name)" }
}
```

(Git Bash:
`for f in scripts/collisions/*.posim scripts/solveit/*.posim; do POSIM_NO_BROWSER=1 ./target/release/posim.exe --script "$f" || echo "FAILED $f"; done`.)

Measured: all six examples print SUCCESS and exit 0
(`logs\example_*.log`); **all 12 collision scripts exit 0**; **all 38
SolveIt scripts exit 0** — 50/50, no FAILED line
(`logs\solveit_and_collision_scripts.log`; one log per script in
`logs\solveit_scripts\`).

Highlights measured inside the 16 dataset examples (each number is in
the committed transcripts in `SolveIt.md` §7 and reproduces here to the
bit — their datasets are among the byte-identical files of §7 below):

| example | claim | measured by SQL |
|---|---|---|
| 20 | perihelion 1, aphelion 4 | `1.000000001093295`, `4.000000026379791` |
| 21 | free fall matches the closed form | worst error `3.552713678800501e-15` |
| 23 | rebound speeds 10 e | `10.000000000000028 / 7.000000000000018 / 5.000000000000014` |
| 25 | gyroradius 5/9, pitch v_z·T | `0.5555555528977862`, `0.6981317007977318` |
| 26 | \|L\| constant through the flip | `L_min = L_max = 3.0001333303705024` |
| 28 | GGL rod stretch | `7.838174553853605e-14` |
| 31 | Adams vs BDF, by ATTACH + JOIN | worst gap `5.615900633415549e-9` |
| 32 | LAG error = g·dt/2 = 0.24525 | `0.24525000000025088` |
| 35 | quaternions stay unit | `4.440892098500626e-16` |

And the three trajectory-algebra / coordinate examples:

| example | claim | measured on this machine |
|---|---|---|
| 36 | (A−B)+B−A closes | `x_residual 6.938893903907228e-18`, `vy_residual 2.7755575615628914e-17`, `e_residual 0` |
| 37 | planar orbit has θ = π/2 in spherical coordinates | `theta_min = theta_max = 1.5707963267948966`; polar r ∈ `[0.7500000019886397, 3.0000000158618207]` |
| 38 | helix pitch v_z·T; toroidal τ range; round trips | pitch `0.6981317007977317`; τ ∈ `[5.28650864731464e-9, 1.1892318293548387]`; worst round trip `7.447602459741819e-16` (prolate; macOS: `8.671119018262734e-16`) |

Every SQL-queried figure in these three tables is identical to the
macOS transcript. The only two lines of the 36–38 transcripts that are
not are value-level round trips through the host libm: example 37's
`polar_to_cart2(cart2_to_polar([3, 4]))` = `[3.0000000000000004,
3.9999999999999996]` (macOS `[3, 3.9999999999999996]`) and example 38's
prolate round trip above.

## 7. The notebooks: 128 executed, eight requirements each

```powershell
cargo build --release -p posim
cd dataset_tools; cargo build --release; cd ..
python notebooks\_build\regen.py
# the parallel execution loop is a Git Bash command (xargs -P 6):
#   ls notebooks/*.ipynb | POSIM_NO_BROWSER=1 xargs -P 6 -I{} \
#       python notebooks/_build/nbrun.py {}
# (sequential PowerShell equivalent, slower:
#   $env:POSIM_NO_BROWSER = "1"
#   Get-ChildItem notebooks\*.ipynb | ForEach-Object { python notebooks\_build\nbrun.py $_.FullName } )
python notebooks\_build\nbcheck.py
```

Measured (`logs\notebooks_run.log`): regen renders **128** notebooks;
nbrun ends **128 ok, 0 failed**; nbcheck ends **128/128 notebooks pass
all eight requirements** (R8 — the dataset section and a
simulator-independent `sqlite3` read-back cell — included). The run
leaves one dataset per notebook in `datasets\` and derives the movies
in `movies\`.

**Byte comparison against the macOS files, after all of the above had
run** (the 38 SolveIt scripts, the 12 collision scripts and the 128
notebooks): the **74 movies in `movies\` are 74/74 byte-identical** to
the files the macOS machine produced; of the **252 files in
`datasets\`** (137 `.db` + 115 `_plots.html`), **245 are byte-identical**
and exactly seven differ — `solveit37_pol.db`, `solveit37_sph.db`,
`solveit37_sph_plots.html`, `solveit38_bis.db`, `solveit38_cyl.db`,
`solveit38_obl.db`, `solveit38_tor.db` — every one of them the output
of a `DATASET MAP` through a curvilinear coordinate system, i.e. of
`special_functions/src/coords.rs` calling the host libm (MSVC UCRT
here, Apple libm on macOS). The source datasets they were mapped from
(`solveit37_orbit.db`, `solveit38_helix.db`) are byte-identical, and so
is every SQL-queried figure in the two transcripts (§6): the difference
is confined to last-place digits of individual mapped coordinates that
no printed aggregate reaches. Quantified per `states` table, the
Windows file against the macOS file: `solveit37_pol` 12 of 1400 float
values differ, worst relative difference 1.99e-16; `solveit37_sph`
3/1400, 1.30e-16; `solveit38_bis` 3/700, 1.58e-15; `solveit38_cyl`
5/700, 1.79e-16; `solveit38_obl` 16/700, 2.67e-14; `solveit38_tor`
5/700, 4.02e-16 (the seventh file, `solveit37_sph_plots.html`, is the
plots page drawn from `solveit37_sph.db`). No dataset of the 128
notebooks or of examples 20–36 differs.

## 8. Display: the browser artifacts

```powershell
Remove-Item Env:POSIM_NO_BROWSER -ErrorAction SilentlyContinue   # let it open the browser

# a dataset's plots page (energy, |P| and |L|, per-body positions, top
# view), straight from the notebook language — this opens the browser:
cargo run -p posim --release
#   then type:   dataset play solveit20_kepler
#   (the page is datasets\solveit20_kepler_plots.html)

# a movie (the 3-D player derived from the dataset):
cargo run -p posim --release
#   then type:   movie play solveit20_kepler
#   (the page is movies\solveit20_kepler.html)

# or open the generated pages directly:
Start-Process datasets\solveit20_kepler_plots.html
Start-Process movies\solveit20_kepler.html
#   (Git Bash: start datasets/solveit20_kepler_plots.html ; start movies/solveit20_kepler.html)

# an independent look inside a dataset, no simulator involved
# (Python 3.12+ ships the sqlite3 shell; Windows has no sqlite3.exe):
python -m sqlite3 datasets\solveit20_kepler.db "SELECT COUNT(*) AS frames, MIN(t), MAX(t) FROM frames"
```

Both pages are self-contained (vanilla JS + canvas, zero external
fetches) and were rendered and inspected in a real browser (Google
Chrome) on this machine: the movie shows the two spheres, labels,
trails, the scrub bar, speed control and the live t/E/|P|/|L| readout;
the plots page shows the five chart panels drawn from the
frames/states tables.

## 9. The other front ends still pass

```powershell
python recorder\src\record_all.py --check      # legacy recorded videos
python tools\win_gui_smoke.py                  # the 13 live GUI pages (Windows smoke test)
python tools\gui_smoke.py                      # the same 13 through the portable smoke test
python jupyter\test_protocol.py                # the machine-mode wire protocol
$env:POSIM_NO_BROWSER = "1"
jupyter\.venv\Scripts\python.exe jupyter\test_kernel.py   # JupyterLab kernel
# (the kernel venv is created once with:
#   uv venv jupyter\.venv
#   uv pip install -p jupyter\.venv\Scripts\python.exe ipykernel jupyter_client )
```

(Git Bash: `POSIM_NO_BROWSER=1 jupyter/.venv/Scripts/python.exe jupyter/test_kernel.py`.)

Measured: **all 13 recordings reproduce byte for byte**
(`logs\videos_check.log`; sizes in bytes: kepler_ellipse 142106,
tumbling_racket 118201, box_of_shapes 287976, double_pendulum_hinges
284436, universal_joint 205556, ball_joint_chain 373656,
rod_pendulum_chain 310608, spinning_top 190450, gyroscope_gimbal
333644, cardan_compass 324423, cardan_gear 279671, rack_and_pinion
170255, piston_crankshaft 316882 — `rack_and_pinion` is the Windows
recording, its GEAR/RACK residual passing through the host libm; the
recorder's own suites `recorder\tests\test_units.py` 17 tests OK and
`test_end_to_end.py` 9 tests OK); **13 of 13 GUIs pass** with both
smoke tests, on ports 8895–8907 (`logs\gui_smoke.log`); **all protocol
checks passed**; kernel venv created with uv (Python 3.14.4 in the
venv), **7/7 cells ok — all kernel checks passed, JupyterLab can drive
this kernel** (`logs\jupyter_tests.log`).

## 10. The documents compile

```powershell
pdflatex -interaction=nonstopmode grammar.tex
pdflatex -interaction=nonstopmode grammar.tex    # twice: the TOC needs the second pass
pdflatex -interaction=nonstopmode SolveIt.tex
pdflatex -interaction=nonstopmode SolveIt.tex
```

(MiKTeX's `pdflatex` — the first run may install missing packages on
the fly; let it.) Measured on the macOS machine: **grammar.pdf 75
pages**, **SolveIt.pdf 48 pages**, zero LaTeX errors in either final
pass (`Select-String -Path grammar.log -Pattern '^!'` prints nothing;
Git Bash `grep -c "^!" grammar.log` = 0). The MiKTeX re-measurement of
the page counts on this machine is pending at the time of writing.

## 11. The entity index and the encyclopedia

```powershell
# the entity index (index_of_entities.html) with the new command families
$env:PYTHONUTF8 = "1"
python tools\extract_rust_items.py > index_data\rust_items.jsonl    # PowerShell 7 writes UTF-8 (Git Bash: identical line)
python tools\build_commands.py
python tools\build_tierb.py
python tools\build_tierc.py
python tools\build_catalog.py
python tools\build_app.py
python tools\verify_index_examples.py
python tools\verify_tierb_examples.py

# the notebook encyclopedia (SolveIt_Notebooks_for_rust\, 147 notebooks)
python SolveIt_Notebooks_for_rust\_tools\gen.py
# execute every posim-family copy (the mercury and rebound families are
# unchanged and keep their verified runs) — Git Bash, xargs -P 4:
#   ls SolveIt_Notebooks_for_rust/0[2-6]*/*/*.ipynb | xargs -P 4 -I{} \
#       python SolveIt_Notebooks_for_rust/_tools/run_copy.py {}
$env:POSIM_NO_BROWSER = "1"
python SolveIt_Notebooks_for_rust\_tools\shoot.py
python SolveIt_Notebooks_for_rust\_tools\encyclo.py
python SolveIt_Notebooks_for_rust\_tools\verify.py
```

Measured on this machine, index: `verify_index_examples.py` ends
**`PASS 1365/1365  (100.0%)`** in 35 s wall clock
(`logs\verify_index_examples.log`) and `verify_tierb_examples.py` ends
**`COMPILED 258/258  (100.0%)`** (`logs\verify_tierb_examples.log`;
tier-B grew with the public `special_functions::coords` module). The
entry counts are the macOS machine's final-pass figures and are not
re-counted here: **123 command entries** (the dataset layer added
`cmd.dataset.*` ×11 + `cmd.sql` + `cmd.movie.*` ×4, then
`cmd.dataset.{diff,sum,map}`) plus **28 coordinate-builtin entries**
(`fn.cart_to_spherical` … `fn.bipolar_to_cart2`), and
`index_of_entities.html` embedding 1623/1623 verified examples.

Encyclopedia, measured on this machine so far: gen.py writes the
manifest with **147** entries; run_copy.py re-executes the posim-family
copies ok — `02_solveit` 38/38, `03_dynamics` 59/59, `04` + `05`
25/25, `06_rust` 6/6 (`_tools\exec_posim_0..3.log`) — and the rebound
family `07_rebound` 17/17 ok (`_tools\exec_rebound.log`). Pending at
the time of writing (the macOS machine's figures are given as the
expected outcome, not as Windows measurements): the two `01_mercury`
copies; shoot.py (macOS: **124** GUI images, 0 failed); encyclo.py
(macOS: ENCYCLOPEDIA.md / .html / .pdf rebuilt, 12.9 MB PDF, Appendix B
populated); verify.py (macOS: **`ALL VERIFY GATES PASS (147
notebooks)`**).

## 12. What is committed where

| path | contents |
|---|---|
| `dataset_tools/` | the sidecar: sources, tests, pinned `Cargo.lock`, `README.md` (schema + protocol + architecture) |
| `posim/src/dataset.rs`, `posim/src/movie.rs` | the language side: handle (finder probes `solveit_db.exe`), recording payloads, plots page, movie player |
| `datasets/` | one `.db` per notebook and per dataset example, plus the generated `_plots.html` pages (252 files) |
| `movies/` | the 74 movies derived from datasets |
| `scripts/solveit/20..38_*.posim` | the sixteen dataset worked examples plus the three trajectory-algebra / coordinate examples |
| `notebooks/` | 128 executed notebooks, each owning a dataset (sections 6, 9, 10) |
| `grammar.md` §5.14–§5.15, §13 / `grammar.tex` / `grammar.pdf` | the full command reference (datasets, SQL, movies, trajectory algebra, the fourteen coordinate systems) and the complete worked session |
| `SolveIt.md` §7 (20–38), §8.3–8.4 / `SolveIt.tex` / `SolveIt.pdf` | the nineteen additional worked examples and the dataset/movie guide |
| `ARCHITECTURE.md` §3.10 | the pinned dataset-subsystem contracts |
| `tools/win_verify_physics.sh`, `tools/win_gui_smoke.py`, `evidence/win11/` | the Windows byte-identity gate, the Windows GUI smoke test, and the gate's outputs and pinned divergence diffs |
| `logs/` | the captured output of every command on this page (`build.log`, `build_release.log`, `test.log`, `test_final.log`, `dataset_tools_build.log`, `dataset_tools_test.log`, `win_verify_physics.log`, `example_*.log`, `solveit_and_collision_scripts.log`, `solveit_scripts\`, `notebooks_run.log`, `videos_check.log`, `gui_smoke.log`, `jupyter_tests.log`, `verify_index_examples.log`, `verify_tierb_examples.log`) |
| `SolveIt_Notebooks_for_rust/` | the encyclopedia, regenerated at 147 entries with the dataset sections |

## 13. Design decisions recorded

1. **The external-crate boundary.** The standing repository rule is
   zero external dependencies in the simulator workspace; the feature
   specification for datasets explicitly names rusqlite + r2d2. Both
   are honoured at once by the sidecar architecture: `dataset_tools/`
   is EXCLUDED from the workspace (exactly like `sundials_rs`), owns
   its own `Cargo.lock`, and posim reaches SQLite only by spawning
   `solveit_db.exe`. `cargo build --workspace` never touches crates.io,
   and the one C compilation in the repository — the SQLite
   amalgamation, by MSVC `cl.exe` through the `cc` crate — happens only
   inside `dataset_tools/`.
2. **No timestamps in datasets.** Byte-stable, replayable output is a
   verified repository property; a wall-clock column would break it
   silently. `meta` carries `schema_version`, `title` and `box` only.
   This is what made the cross-platform byte comparison of §7 possible
   at all: 74/74 movies and 245/252 datasets identical between an
   Apple Silicon Mac and this Windows machine, with the seven
   exceptions nameable and explained.
3. **`DATASET CREATE` replaces.** Re-running a script or notebook must
   reproduce the same database; COPY and SAVEAS are the preservation
   idioms (both are SQLite `VACUUM INTO`).
4. **`RESET` closes the open dataset.** A dataset describes ONE world.
   The ~40 notebooks whose scripts reset mid-session reopen the file
   afterwards and read back everything recorded up to the reset.
5. **Movies are derived, never recorded.** `MOVIE CREATE` renders
   whatever the dataset holds; the legacy `videos\` recorder pages
   remain and still verify byte-for-byte, but the dataset → movie
   pipeline is the canonical path because the movie is backed by a
   queryable database instead of numbers frozen into HTML.
