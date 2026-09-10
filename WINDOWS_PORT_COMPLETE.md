# WINDOWS_PORT_COMPLETE — summary of the finished port

**rustSolveIt_Win11_SUNDIALS_7_8_0 is complete and verified on Windows
11.** This page summarizes what was delivered and carries the complete
set of commands to reproduce every result — you do not need any other
document to run anything named here. Every number on this page was
measured on the Windows 11 machine described below on 2026-09-09; the
run logs are committed under `logs\` and `rebound_rust\logs\`.

This is the second Windows port. The first (2026-08-22..26) ported the
Linux release; this one ports the **entire macOS / Apple Silicon
project** — `once-ere/rustSolveIt_macos-silicon_SUNDIALS_7_8_0` at
commit 5360157 (31 commits, 5054 tracked files): the SQLite dataset
layer and its sidecar, SolveIt examples 20–38, 128 notebooks, the
`planet_Mercury` sub-project, the 147-notebook encyclopedia, and the
REBOUND / REBOUNDx N-body ports with their C-reference gates — onto the
Windows engine, on Windows 11.

## What this project is

A pure-Rust physics simulator for Windows 11 (x86-64): a notebook
command language (`posim`) over a rigid-body/particle physics library
(`physical_object`), with quantum-mechanics, special-function and
SQLite dataset / SQL / movie command families, a live browser scene
window, 13 live GUI web pages, 13 recorded browser videos, 128 executed
Jupyter notebooks (one per example, each owning a SQLite dataset and a
derived movie), a 147-notebook encyclopedia, the `planet_Mercury`
tidal-locking sub-project, and pure-Rust translations of REBOUND 5.1.1
and REBOUNDx 5.1.0. All numerical integration is done by a **vendored
pure-Rust translation of SUNDIALS 7.8.0** (CVODE, CVODES, IDA, IDAS,
KINSOL, ARKODE) — `once-ere/SUNDIALS_7_8_Rust_port_for_Windows11`,
which differs from the Linux/macOS engine only in its backend module
sets (public API identical; `diff -rq --exclude=.git --exclude=target
--exclude=logs` against a fresh clone of that repository is silent) —
whose pure-Rust glibc-translated math library makes the physics
**byte-identical** to the Linux release's recorded evidence. The
simulator workspace has zero `unsafe`, zero crates.io dependencies,
zero warnings, zero network access at build time.

Lineage: ported from
`once-ere/rustSolveIt_macos-silicon_SUNDIALS_7_8_0` (macOS / Apple
Silicon), itself ported from `once-ere/rustSolveIt_Using_SUNDIALS_7_8_0`
(Linux). Repository:
<https://github.com/once-ere/rustSolveIt_Win11_SUNDIALS_7_8_0.git>.

### Machine and toolchain (measured)

| item | value |
|---|---|
| OS | Windows 11 Pro for Workstations, build 10.0.26200 |
| CPU | Intel Core Ultra 9 275HX (`platform.processor`: Intel64 Family 6 Model 198 Stepping 2); has FMA |
| Rust | rustc 1.91.1 (ed61e7d7e 2025-11-07) / cargo 1.91.1, host `x86_64-pc-windows-msvc` |
| FMA | the root `.cargo\config.toml` pins `-C target-feature=+fma` (required by the Windows engine's deterministic libm) |
| linker / C compiler | Visual Studio 2026 Build Tools, MSVC 14.51.36231 (`C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Tools\MSVC\14.51.36231\bin\Hostx64\x64\cl.exe`), located by rustc and by the `cc` crate automatically — no developer prompt |
| also installed, NOT used | VS 2026 Professional/Enterprise, VS 2022 Professional/Enterprise, Intel oneAPI 2025.3 / 2026.1 |
| Python | 3.14.5 (cp1252 default encoding), nbformat 5.10.4, nbclient 0.10.2, jupyter_client 8.6.3, ipykernel 7.1.0; uv 0.9.10 |
| LaTeX | MiKTeX-pdfTeX 4.18 (MiKTeX 24.1) |
| Chrome | `C:\Program Files\Google\Chrome\Application\chrome.exe` (headless screenshots and the encyclopedia PDF) |

**No WSL2 compiler or linker was used anywhere.** The simulator
workspace is pure Rust; rustc locates the MSVC linker itself. The
**only C compiler used anywhere is MSVC `cl.exe`**, in exactly two
places: compiling the bundled SQLite amalgamation inside
`dataset_tools` through the `cc` crate (the `solveit_db.exe` sidecar,
which lives outside the workspace), and compiling the REBOUND /
REBOUNDx C reference harnesses for the rebound gates (there invoked
explicitly through `vcvars64.bat`, cl 19.51.36256).

## The scoreboard (every number measured on this machine, 2026-09-09)

| gate | result | log |
|---|---|---|
| `cargo build --workspace --all-targets` | 0 errors, **0 warnings** | `logs\build.log` |
| `cargo build --release --workspace --all-targets` | **0 warnings**; `target\release\posim.exe` 5,705,216 bytes | `logs\build_release.log` |
| `cargo test --workspace` | **638 passed, 0 failed** (49 physical_object lib + 19 collision + 9 conservation + 42 constrained/DAE + 121 posim + 92 quantum + 240 special_functions + 11 vendored identities + 5 quantum doctests + 50 special_functions doctests); re-run after the `dataset.rs` finder fix and the `coords.rs` / `physical_object\src` clippy rewrites: 638 passed (`logs\test_final.log`; see "What is and is not claimed" for the later `dataset.rs` edit) | `logs\test.log`, `logs\test_final.log` |
| sidecar `cd dataset_tools; cargo build --release` | rusqlite 0.32.1 + r2d2_sqlite 0.25.0, bundled SQLite compiled by MSVC cl via the `cc` crate; finished in 18.22 s, 0 warnings; `solveit_db.exe` 2,326,016 bytes | `logs\dataset_tools_build.log` |
| sidecar `cargo test` | **11 passed, 0 failed** (8 unit + 3 integration) | `logs\dataset_tools_test.log` |
| sidecar stress demo | `stress: 10 threads x 500 rows = 5000 expected, 5000 written, journal_mode=wal` | `logs\crosscheck_stress_launcher.log` |
| 6 self-checking physics examples | 6/6 **SUCCESS**; `examples-win11.log` IDENTICAL-MODULO-DOCUMENTED to the Linux evidence (the pinned diff is one blank line) | `logs\win_verify_physics.log`, `evidence\win11\` |
| 12 collision scripts | combined output **IDENTICAL** to the Linux evidence | `logs\win_verify_physics.log` |
| 59 dynamic notebooks | IDENTICAL-MODULO-DOCUMENTED: all but the quantum pair (`double_slit`, `tunneling`) byte-identical to Linux; those two differ in last printed digits only, pinned in `evidence\win11\accepted-divergences-dynamic.diff` | `logs\win_verify_physics.log` |
| physics byte-identity gate `bash tools/win_verify_physics.sh` | **exit 0** | `logs\win_verify_physics.log` |
| 38 SolveIt + 12 collision scripts, headless | **50/50 exit 0** | `logs\solveit_and_collision_scripts.log`, `logs\solveit_scripts\` |
| 128 Jupyter notebooks | regen → execute → check: **128 ok, 0 failed**; **128/128 pass all eight requirements** | `logs\notebooks_run.log` |
| datasets and movies vs the macOS files | movies **74/74 byte-identical**; datasets **245/252 byte-identical** (the 7 that differ are the curvilinear-coordinate datasets of SolveIt 37–38 — see "What is and is not claimed") | measured 2026-09-09 |
| 13 recorded browser videos | `record_all.py --check`: **all 13 reproduce byte for byte** (`rack_and_pinion` is the Windows recording); recorder tests 17 OK + 9 OK | `logs\videos_check.log` |
| 13 live GUI servers | `tools\win_gui_smoke.py` **13 of 13 pass**; `tools\gui_smoke.py` 13 of 13 pass (ports 8895–8907) | `logs\gui_smoke.log` |
| Jupyter wire protocol + kernel | all protocol checks passed; **7/7 kernel cells ok**, "JupyterLab can drive this kernel" | `logs\jupyter_tests.log` |
| entity index verifiers | **PASS 1365/1365** posim/machine examples (35 s wall clock); **COMPILED 258/258** Rust snippets | `logs\verify_index_examples.log`, `logs\verify_tierb_examples.log` |
| documentation PDFs (MiKTeX, two passes each, zero errors) | `SolveIt.pdf` 49 pages (693,477 bytes); `grammar.pdf` 74 pages (970,144 bytes); `scene_info.pdf` 15 pages (388,864 bytes); `physical_object_simulator.pdf` 19 pages (523,088 bytes); `collision_detection.pdf` unchanged (its `.tex` was not edited) | `logs\pdflatex_*_pass2.log` |
| `planet_Mercury\mercury_rs` | build 0 warnings; **19 passed**; `mercury_rs.exe` 1,104,384 bytes | `logs\mercury_rs_build.log`, `logs\mercury_rs_test.log` |
| `planet_Mercury\mercury_crosscheck` | build 0 warnings; measured dΩ/dt = −3.652928e-18 vs Hut (1981) −3.643780e-18 rad/s², rel. diff 2.511e-3, **PASS / SUCCESS** | `logs\crosscheck_stress_launcher.log` |
| Mercury notebooks | `mercury_tidal_locking.ipynb` ok (18 cells), 1 ok 0 failed, all structure rules pass, gauntlet 6/6 runs SUCCESS; `mercury_test2_jupiter_gr.ipynb` ok (17 cells), 1 ok 0 failed, all structure rules pass, 5/5 runs SUCCESS | `logs\mercury_notebook1.log`, `logs\mercury_notebook2.log` |
| encyclopedia (`SolveIt_Notebooks_for_rust`) | manifest 147 entries; copies executed: 02_solveit 38/38, 03_dynamics 59/59, 04+05 25/25, 06_rust 6/6, 07_rebound 17/17, 01_mercury 2/2 after the UTF-8 driver fix (`exec_mercury.log`: 18 cells ok; `exec_mercury2.log`: 17 cells ok) — 147/147; `shoot.py`: **124 captured, 0 failed** (smallest PNG 18,039 bytes); final `encyclo.py`: `ENCYCLOPEDIA.md` 162,628 chars, `.html` 187,017 chars, `.pdf` 1,364,723 bytes, header "147/147 green"; `verify.py`: "ALL VERIFY GATES PASS (147 notebooks)" | `SolveIt_Notebooks_for_rust\_tools\exec_*.log`, `shoot.log`; the committed `ENCYCLOPEDIA.*` (commit `5d87e8d`) |
| `rebound_rust` (`rebound_rs` 5.1.1) | build 0 warnings; `cargo test --release` **394 passed** (67+32+39+72+55+70+59); clippy 0 warnings | `rebound_rust\logs\win11_{build,test,clippy}_20260909.log` |
| `reboundx_rust` (`reboundx_rs` 5.1.0) | build 0 warnings; **137 passed** (34+32+28+17+26; 2 doctests ignored); clippy 0 warnings | `logs\reboundx_*.log` |
| 17 rebound notebooks | regenerated and executed: "All 17 notebooks executed with no errors" (twice: first pass, and again after the C dumps existed) | `rebound_rust\logs\win11_notebooks_20260909.log`, `rebound_rust\notebooks\run_all.log` |
| rebound C-reference gates on Windows (MSVC cl 19.51.36256, `/Ox /fp:precise`) | integrator matrix **63 of 63 bit-identical**; shearing sheet 1482 particles / 102,533 collisions, final SHA-256 equal (`75BDAAB7…521F0`); Simulationarchive 6/6 round trips identical; derivatives 130/130, frequency 30/30, addfmt 12/12, movetocom_var 5/5, kepler_rectilinear 11/11; REBOUNDx `tides_spin` **6/6 bit-identical**; rebx binary 25/25 + 25/25, C readback 28/28; libm differential: 8 of 9 functions 0 differences, `pow` 60/200,000 (0.0300 %, all exactly 1 ULP); `bs_pow` 56/200,000 (0.0280 %, all 1 ULP) | `rebound_rust\logs\win11_c_reference_gates_20260909.md` (542 lines, every command and number) |

### What the port changed

The macOS project was merged onto the Windows repository with the
pristine Linux import as the common ancestor (LF-normalization commit
f1f598d, 219 files; merge commit d6f6605 resolving 796 conflicts by a
recorded policy; tooling commit f8557de; notebooks commit 03b3986).
The Windows engine (`sundials_rs\`) was kept — it is the
Windows-verified one and the byte-identity gate is its proof. One
Rust source line of behaviour changed for Windows: `find_db_tool` in
`posim\src\dataset.rs` now probes `solveit_db.exe` before the
extensionless name (otherwise every DATASET command reported the
sidecar "not built"). The Python tooling of the new material
(encyclopedia `_tools\`, `planet_Mercury` notebook builders/runner,
`notebooks\_build\nbdataset.py`) learned to find `posim.exe` /
`mercury_rs.exe` / `.venv\Scripts\python.exe` / Windows Chrome, to
read and write UTF-8 on a cp1252 platform, and to write LF.
`jupyter\kernelspec\kernel.json` starts with `python` (Windows has no
`python3` launcher). Clippy 1.91.1 lints in code inherited unchanged
from the Linux/macOS lineage (7 files: `special_functions\src\coords.rs`,
`physical_object\src\{constrain,equilibrium,integrate,sensitivity}.rs`,
`posim\src\dataset.rs` — the `dataset_map` copy loop became
`copy_from_slice` — and one line of `physical_object\tests\dae.rs`)
were fixed with behaviour-preserving rewrites, two
`#[allow(clippy::too_many_arguments)]` waivers and one
`#[allow(clippy::type_complexity)]`. The 16 macOS
arm64 harness binaries the macOS repository had committed under
`rebound_rust\porttest` and `reboundx_rust\porttest` were untracked
(build products, rebuilt from their `.c` sources). Not one physics
formula, solver call, tolerance or heuristic moved — the byte-identity
gate, the 74/74 movie and 245/252 dataset byte-compares and the 638
tests are what prove it (the gate, the dataset byte-compare and the
tests were all re-run after the last of those rewrites: gate exit 0 on
the final binary, `solveit20_kepler.db` byte-identical, 638 passed, 0 failed inside
the certifier's run — see "What is and is not claimed").

## Complete commands

All from the repository root (the folder that contains `Cargo.toml`
and `posim\`), in **PowerShell** unless marked Git Bash; where the Git
Bash form differs it follows in parentheses. You need Rust (`rustup`
from <https://rustup.rs>, default `x86_64-pc-windows-msvc` toolchain),
the Visual Studio Build Tools with the "Desktop development with C++"
workload (for the MSVC linker and for `cl.exe`, which compiles the
sidecar's bundled SQLite and, optionally, the rebound C reference gates),
Python 3 (`python`, not `python3`), and for the optional parts `uv`,
MiKTeX and Google Chrome. The CPU must have FMA.

Clone:

```powershell
git clone https://github.com/once-ere/rustSolveIt_Win11_SUNDIALS_7_8_0.git
cd rustSolveIt_Win11_SUNDIALS_7_8_0
```

### Build and test

```powershell
cargo build --workspace --all-targets
cargo build --release --workspace --all-targets
cargo test --workspace
```

Expect `0 warnings` from both builds and 638 tests passed. To keep a
log: `cargo test --workspace 2>&1 | Tee-Object logs\test.log` (Git
Bash: `cargo test --workspace 2>&1 | tee logs/test.log`).

### The SQLite sidecar (`solveit_db.exe`)

Build it once — every DATASET command, every dataset example (SolveIt
20–38) and every notebook needs it; this is the one step that compiles
C (the bundled SQLite amalgamation, by MSVC `cl.exe` through the `cc`
crate) and the one build that fetches from crates.io:

```powershell
cd dataset_tools; cargo build --release; cd ..
```

(Git Bash: `cd dataset_tools && cargo build --release && cd ..`.) The
binary lands at `dataset_tools\target\release\solveit_db.exe`;
`posim.exe` finds it by itself (`$env:POSIM_DB_TOOL` first, then that
path walking up from the current directory, then `PATH`). Its tests
and the multithreaded WAL demonstration:

```powershell
cd dataset_tools
cargo test
.\target\release\solveit_db.exe stress $env:TEMP\stress_demo.db 10 500
cd ..
```

(Git Bash: `./target/release/solveit_db.exe stress /tmp/stress_demo.db
10 500`.) Expect `11 passed` and `stress: 10 threads x 500 rows = 5000
expected, 5000 written, journal_mode=wal`. To look inside any dataset
without the sidecar:

```powershell
python -m sqlite3 datasets\nb_solveit_01_elastic_head_on.db "SELECT COUNT(*) FROM frames"
```

`<name>.db-wal` / `<name>.db-shm` files beside a dataset are transient
SQLite side files (a still-open or killed writer); they are gitignored,
never part of a result and never compared.

### The interactive notebook

Type `HELP`; `scene create` opens the live browser window with
Start/Pause/Reverse/Reset and live E, P, L readouts:

```powershell
cargo run
```

### The six self-checking physics examples

```powershell
cargo run -p physical_object --release --example kepler_orbit
cargo run -p physical_object --release --example outer_solar_system
cargo run -p physical_object --release --example tumbling_body
cargo run -p physical_object --release --example charged_in_b_field
cargo run -p physical_object --release --example newtons_cradle
cargo run -p physical_object --release --example bouncing_ball_restitution
```

Each ends with a `SUCCESS:` line.

### Any example script

The 12 collision scripts `scripts\collisions\01_head_on_exchange.posim`
… `12_two_dumbbells.posim`, the 38 SolveIt scripts
`scripts\solveit\01_elastic_head_on.posim` …
`38_dataset_curvilinear_tour.posim` (20–38 need the sidecar built), and
the 59 dynamic notebooks `dynamic_notebooks\*.posim` all run the same
way. Set `$env:POSIM_NO_BROWSER = "1"` first for headless (Git Bash:
prefix the command with `POSIM_NO_BROWSER=1`):

```powershell
cargo run -p posim --release -- --script scripts/collisions/01_head_on_exchange.posim
```

or, with the release binary already built:

```powershell
.\target\release\posim.exe --script scripts\solveit\20_dataset_kepler_orbit.posim
```

All 38 SolveIt and 12 collision scripts headless, printing each exit
code (all 50 are 0):

```powershell
$env:POSIM_NO_BROWSER = "1"
Get-ChildItem scripts\solveit\*.posim, scripts\collisions\*.posim | ForEach-Object {
    .\target\release\posim.exe --script $_.FullName > $null
    "$LASTEXITCODE $($_.FullName)"
}
```

(Git Bash: `for f in scripts/solveit/*.posim scripts/collisions/*.posim;
do POSIM_NO_BROWSER=1 target/release/posim.exe --script "$f" >/dev/null;
echo "$? $f"; done`.) Per-script transcripts from this machine are in
`logs\solveit_scripts\`.

### A dynamic notebook, interactively with its live scene window

```powershell
tools\posim_notebook.cmd kepler_orbit
tools\posim_notebook.cmd routh_p1_hodograph_circle
tools\posim_notebook.cmd --list
```

(Git Bash: `tools/posim_notebook kepler_orbit`, the bash original.)
The launcher resolves the bare name against
`dynamic_notebooks\<name>.posim`, loads every cell, opens the scene
window and leaves you at an `In[]` prompt; press Start in the window or
type `scene start`. Extra arguments pass through to posim.

### The physics byte-identity gate (Git Bash)

Re-runs the six examples, the 12 collision scripts and the 59 dynamic
notebooks in the Linux evidence's concatenated-log formats, writes
`evidence\win11\*-win11.log`, and diffs them against
`evidence\port-7.8.0\`; needs `cargo build --release --workspace
--all-targets` first:

```bash
bash tools/win_verify_physics.sh
```

Expect `IDENTICAL-MODULO-DOCUMENTED` (examples), `IDENTICAL` (collision
scripts), `IDENTICAL-MODULO-DOCUMENTED` (dynamic notebooks) and exit 0;
the pinned diffs are `evidence\win11\accepted-divergences-examples.diff`
and `evidence\win11\accepted-divergences-dynamic.diff`.

### The 128 Jupyter notebooks — regenerate, execute, check

```powershell
cargo build --release -p posim
cd dataset_tools; cargo build --release; cd ..
python notebooks\_build\regen.py
$env:POSIM_NO_BROWSER = "1"
# PowerShell 7 (pwsh) only — Windows PowerShell 5.1 has no -Parallel; use the sequential loop:
#   Get-ChildItem notebooks\*.ipynb | ForEach-Object { python notebooks\_build\nbrun.py $_.FullName }
Get-ChildItem notebooks\*.ipynb | ForEach-Object -Parallel {
    python notebooks/_build/nbrun.py $_.FullName
} -ThrottleLimit 6
python notebooks\_build\nbcheck.py
```

The same batch in Git Bash (the form the verification ran):

```bash
cargo build --release -p posim
cd dataset_tools && cargo build --release && cd ..
python notebooks/_build/regen.py
ls notebooks/*.ipynb | POSIM_NO_BROWSER=1 xargs -P 6 -I{} python notebooks/_build/nbrun.py {}
python notebooks/_build/nbcheck.py
```

Expect `128 ok, 0 failed` and `128/128 notebooks pass all eight
requirements`. Every run rewrites `datasets\nb_<name>.db`, its
`_plots.html` page and `movies\nb_<name>.html`.

### Open the notebooks in JupyterLab

```powershell
python -m pip install --user jupyterlab
python -m jupyterlab notebooks\
```

(`jupyter lab notebooks/` does the same when the `jupyter` launcher is
on your PATH; Git Bash accepts the same lines with forward slashes.)
The notebooks are plain Python 3 notebooks that start `posim.exe` in
machine mode themselves.

### The wire protocol and the JupyterLab wrapper kernel

```powershell
cargo build --release
python jupyter\test_protocol.py
uv venv jupyter\.venv
uv pip install -p jupyter\.venv\Scripts\python.exe ipykernel jupyter_client jupyterlab
$env:POSIM_NO_BROWSER = "1"
jupyter\.venv\Scripts\python.exe jupyter\test_kernel.py
```

(Git Bash: `uv pip install -p jupyter/.venv/Scripts/python.exe …` and
`POSIM_NO_BROWSER=1 jupyter/.venv/Scripts/python.exe
jupyter/test_kernel.py`.) Expect `all protocol checks passed` and `all
kernel checks passed — JupyterLab can drive this kernel`. To use the
kernel from JupyterLab (pick the **posim (physical_object)** tile in
the launcher):

```powershell
cd jupyter
# kernelspec\kernel.json's argv starts with `python` (the PATH interpreter), and
# posim_kernel imports ipykernel — so that interpreter needs it too
# (alternative: edit argv[0] in kernelspec\kernel.json to this venv's .venv\Scripts\python.exe first):
python -m pip install --user ipykernel
.venv\Scripts\jupyter.exe kernelspec install --user kernelspec --name posim
$env:PYTHONPATH = "$PWD"
.venv\Scripts\jupyter.exe lab
cd ..
```

(Git Bash: `PYTHONPATH=$PWD .venv/Scripts/jupyter.exe lab`.)

### The 13 live GUI pages

One standard-library Python server per scene on a fixed port
(`piston_crankshaft` 8895, `rack_and_pinion` 8896, `gyroscope_gimbal`
8897, `cardan_compass` 8898, `universal_joint` 8899, `spinning_top`
8900, `ball_joint_chain` 8901, `cardan_gear` 8902,
`rod_pendulum_chain` 8903, `double_pendulum_hinges` 8904,
`tumbling_racket` 8905, `kepler_ellipse` 8906, `box_of_shapes` 8907).
The server prints its URL and never opens a browser itself:

```powershell
cargo build --release -p posim
python gui\kepler_ellipse\server.py
Start-Process http://127.0.0.1:8906/
```

(Git Bash: `python gui/kepler_ellipse/server.py`, then `start
http://127.0.0.1:8906/`.) Stop it with Ctrl+C. The automated pass over
all 13 (page served, live state, Start advances t, Stop, Reset returns
to t = 0):

```powershell
python tools\win_gui_smoke.py
python tools\gui_smoke.py
```

Both print `13 of 13 GUIs pass`.

### The recorded browser videos — watch, re-record, verify

```powershell
Start-Process videos\kepler_ellipse.html
python recorder\src\record_all.py
python recorder\src\record_all.py --check
python recorder\src\record_all.py --only kepler_ellipse
python recorder\tests\test_units.py
python recorder\tests\test_end_to_end.py
```

(Git Bash: `start videos/kepler_ellipse.html`; the rest with forward
slashes.) `--check` writes nothing and prints `all 13 recordings
reproduce byte for byte`; the tests print `Ran 17 tests … OK` and
`Ran 9 tests … OK`. Record a new video from any scene script:

```powershell
cargo build --release -p posim
python recorder\src\record_video.py videos\scenes\kepler_ellipse.posim `
    -o $env:TEMP\mine.html --frames 360 --dt 0.02 --title "Kepler orbit, e = 0.6"
Start-Process $env:TEMP\mine.html
```

(Git Bash: `python recorder/src/record_video.py
videos/scenes/kepler_ellipse.posim -o /tmp/mine.html --frames 360 --dt
0.02 --title "Kepler orbit, e = 0.6"`, then `start /tmp/mine.html`.)
To record your own screen instead, Win+Alt+R (Xbox Game Bar).

### The entity index and its verifiers

```powershell
Start-Process index_of_entities.html
python tools\verify_index_examples.py
python tools\verify_tierb_examples.py
```

(Git Bash: `start index_of_entities.html`.) Keep `catalog-c.js` beside
the page. Expect `PASS 1365/1365` and `COMPILED 258/258`. To rebuild
the index from the sources:

```powershell
$env:PYTHONUTF8 = "1"
python tools\extract_rust_items.py > index_data\rust_items.jsonl    # PowerShell 7 (pwsh) or Git Bash: Windows PowerShell 5.1 writes `>` as UTF-16, which the index tools do not read
python tools\build_commands.py; python tools\build_tierb.py; python tools\build_tierc.py
python tools\build_catalog.py; python tools\build_app.py
```

(Git Bash: `&&` between the build steps; the first line redirects the
bytes verbatim there.)

### The documentation PDFs

MiKTeX `pdflatex`, each twice — the table of contents needs the second
pass:

```powershell
pdflatex -interaction=nonstopmode SolveIt.tex
pdflatex -interaction=nonstopmode SolveIt.tex
pdflatex -interaction=nonstopmode grammar.tex
pdflatex -interaction=nonstopmode grammar.tex
pdflatex -interaction=nonstopmode scene_info.tex
pdflatex -interaction=nonstopmode scene_info.tex
pdflatex -interaction=nonstopmode physical_object_simulator.tex
pdflatex -interaction=nonstopmode physical_object_simulator.tex
pdflatex -interaction=nonstopmode collision_detection.tex
pdflatex -interaction=nonstopmode collision_detection.tex
```

`SolveIt.md` is the source of truth for `SolveIt.tex` and `grammar.md`
for `grammar.tex`.

### planet_Mercury — build, test, cross-check, notebooks

The compute crate and its 19 analytic-expectation tests
(`mercury_rs.exe` lands at
`planet_Mercury\mercury_rs\target\release\mercury_rs.exe`):

```powershell
cd planet_Mercury\mercury_rs
cargo build --release
cargo test --release
cargo run --release -- print-config
cd ..\..
```

(Git Bash: `cd planet_Mercury/mercury_rs && cargo build --release &&
cargo test --release && cargo run --release -- print-config && cd
../..`.) The independent REBOUNDx `tides_spin` cross-check (an upstream
REBOUNDx notice about a 4.5.0 tidal-potential fix also prints —
expected, not an error):

```powershell
cd planet_Mercury\mercury_crosscheck
cargo run --release
cd ..\..
```

Expect `PASS - crosscheck.despin_rate` and `SUCCESS`. The two notebooks
— build deterministically, execute headless (test 1 takes about an
hour and regenerates `planet_Mercury\data\`, which is gitignored), audit:

```powershell
cd planet_Mercury\notebook
python build_notebook.py
python build_notebook2.py
$env:MERCURY_NO_BROWSER = "1"
python run_notebook.py mercury_tidal_locking.ipynb
python run_notebook.py mercury_test2_jupiter_gr.ipynb
python check_notebook.py mercury_tidal_locking.ipynb
python check_notebook.py mercury_test2_jupiter_gr.ipynb
cd ..\..
```

(Git Bash: `MERCURY_NO_BROWSER=1 python run_notebook.py
mercury_tidal_locking.ipynb`.) Expect `1 ok, 0 failed` and `all
structure rules pass` for each. The display pages
`planet_Mercury\gui\mercury_orbit.html` and `mercury_test2.html` are
baked by the notebooks (`python gui\bake_page.py` / `bake_page2.py`
from `planet_Mercury\` rebakes them from the databases). To open the
notebooks interactively:

```powershell
python -m venv planet_Mercury\notebook\.venv
planet_Mercury\notebook\.venv\Scripts\pip install jupyterlab
planet_Mercury\notebook\.venv\Scripts\jupyter lab planet_Mercury\notebook\mercury_tidal_locking.ipynb
```

### The encyclopedia pipeline (`SolveIt_Notebooks_for_rust`)

147 notebook copies (128 engine + 17 rebound + 2 Mercury) in seven
tagged topic folders, each with a header cell and a `player_<tag>.py`.
Prerequisites: the builds above (`posim.exe`, `solveit_db.exe`,
`mercury_rs.exe`, and `cd reboundx_rust; cargo build --release; cd ..`
for the rebound family), Google Chrome for the screenshots and the
PDF, and — for the rebound family only — `jupyterlab` (plus
`matplotlib`) in `planet_Mercury\notebook\.venv` (the system `python`
with nbformat/nbclient/ipykernel is used when that venv is absent).

```powershell
python SolveIt_Notebooks_for_rust\_tools\gen.py
$env:POSIM_NO_BROWSER = "1"
Get-ChildItem SolveIt_Notebooks_for_rust\0*\*\*.ipynb | ForEach-Object {
    python SolveIt_Notebooks_for_rust\_tools\run_copy.py $_.FullName
}
python SolveIt_Notebooks_for_rust\_tools\shoot.py --workers 4
python SolveIt_Notebooks_for_rust\_tools\encyclo.py
python SolveIt_Notebooks_for_rust\_tools\verify.py
```

(Git Bash: `for f in SolveIt_Notebooks_for_rust/0*/*/*.ipynb; do
POSIM_NO_BROWSER=1 python SolveIt_Notebooks_for_rust/_tools/run_copy.py
"$f"; done`, the rest with forward slashes.) `gen.py --only PAT` and
`shoot.py --only PAT` restrict to tags containing `PAT`. `run_copy.py`
prints `ok`/`FAIL` per copy; `shoot.py` prints `124 captured, 0 failed`
(the 6 `rust_*` and 17 rebound entries have no browser GUI by design);
`encyclo.py` writes `ENCYCLOPEDIA.md`, `.html` and `.pdf` (headless
Chrome print); `verify.py` prints `ALL VERIFY GATES PASS (147
notebooks)`. One notebook copy's own player:

```powershell
python SolveIt_Notebooks_for_rust\02_solveit_worked_examples\solveit_01_elastic_head_on\player_solveit_01_elastic_head_on.py gui
python SolveIt_Notebooks_for_rust\02_solveit_worked_examples\solveit_01_elastic_head_on\player_solveit_01_elastic_head_on.py capture out.png
```

### REBOUND / REBOUNDx — build, test, notebooks, C gates

Each crate is its own Cargo project (both GPL-3.0-or-later; excluded
from the simulator workspace):

```powershell
cd rebound_rust
cargo build --release
cargo test --release
cargo clippy --release --all-targets
cd ..\reboundx_rust
cargo build --release
cargo test --release
cargo clippy --release --all-targets
cd ..
```

Expect 394 and 137 passed, 0 warnings from every build and clippy. Any
example (`rebound_rust\examples\*.rs`, `reboundx_rust\examples\*.rs`):

```powershell
cd rebound_rust
cargo run --release --example integrators_test -- whfast 2 500
cd ..
```

The 17 notebooks — regenerate from the example programs, then execute
every one in a fresh kernel (needs `nbformat`, `nbclient`, `ipykernel`
in the `python` you run; run from inside the folder, the notebooks use
the current directory):

```powershell
cd rebound_rust\notebooks
python make_notebooks.py
$env:PYTHONIOENCODING = "utf-8"    # nbclient failure messages can carry non-ASCII; the default here is cp1252 (Git Bash: PYTHONIOENCODING=utf-8 python run_notebooks.py)
python run_notebooks.py
cd ..\..
```

Expect `All 17 notebooks executed with no errors.` (`python
run_notebooks.py shearing_sheet` runs only matching names; `--timeout
3600` raises the per-cell limit.) The C-reference gates — compiling the
original REBOUND (dad5f97) and REBOUNDx 5.1.0 with MSVC `cl.exe`
through `vcvars64.bat`, then comparing raw-bit dumps with the Rust
examples — are a longer procedure; every command and every measured
number is in
`rebound_rust\logs\win11_c_reference_gates_20260909.md`, and the guide
`rebound_rust\rebound_rust.md` §15–§16 explains each gate.

### The clean-source certifier (Git Bash)

```bash
bash scripts/certify_clean.sh
```

Runs `cargo clippy --workspace --all-targets` under the crates'
`deny(warnings)`, the release build, the 638 tests, the StageNbks
notebooks and the licence-hygiene checks; exit 0 only if every gate
passes.

## Where the deliverables live

- `grammar.md` / `grammar.tex` / `grammar.pdf` — the complete command
  language: lexer, full EBNF, type system, every command (the SQLite
  dataset, SQL and movie families in §5.14), the stack machine, the
  SUNDIALS 7.8.0 engine, the browser videos, and 18 fully worked
  examples.
- `SolveIt.md` / `SolveIt.tex` / `SolveIt.pdf` — the complete solution
  guide for a first-time reader, with 38 fully documented worked
  examples (scripts in `scripts\solveit\`; 20–38 are the dataset / SQL
  / movie / curvilinear-coordinate examples).
- `scene_info.md/.tex/.pdf`, `physical_object_simulator.md/.tex/.pdf`,
  `collision_detection.md/.tex/.pdf` — the scene window, the
  predecessor user guide, the collision science reference.
- `ARCHITECTURE.md` — module responsibilities and pinned cross-module
  contracts. `CLAUDE.md` — working rules for contributors and agents,
  including the Windows traps.
- `PORT_WIN11_PROVENANCE.md` — this port's full provenance: machine,
  compiler choice, the complete change list, and every verification
  command with its measured result. `VERIFICATION_WIN11.md` — the
  per-suite verification matrix and every documented divergence.
  `REBOUND_REBOUNDX_WIN11_PROVENANCE.md` — the rebound ports' Windows
  provenance. The macOS editions of these pages
  (`PORT_MACOS_PROVENANCE.md`, `VERIFICATION_MACOS.md`,
  `MACOS_PORT_COMPLETE.md`, `REBOUND_REBOUNDX_MACOS_PROVENANCE.md`) are
  kept verbatim as lineage under a banner.
- `SQLITE_DATASETS_PROVENANCE.md`, `DATASETS_COMPLETE.md`,
  `dataset_tools\README.md` — the dataset layer and the sidecar;
  `datasets\` (252 files: one `.db` per notebook and dataset example,
  plus `_plots.html` pages) and `movies\` (74 derived movies).
- `REBOUND_PROVENANCE_*.md` — one self-contained provenance page per
  rebound-physics (restitution) notebook.
- `notebooks\` — the 128 executed Jupyter notebooks (one per example)
  and their generator in `notebooks\_build\`; `gui\` — the 13 live GUI
  pages; `videos\` — the 13 recorded browser videos and their scene
  scripts; `dynamic_notebooks\` — the 59 dynamic notebooks
  (`MANIFEST.md`, `README.md`); `jupyter\` — the wrapper kernel;
  `recorder\` — the video recorder; `index_of_entities.html` +
  `catalog-c.js` + `index_data\` — the entity index.
- `evidence\win11\` — the Windows gate logs and the two pinned
  divergence diffs; `evidence\port-7.8.0\` — the Linux reference logs;
  `evidence\macos\` — the macOS gate logs (lineage). `logs\` — every
  run log cited on this page.
- `planet_Mercury\` — `mercury_rs\`, `mercury_crosscheck\`,
  `notebook\` (the two executed notebooks and their tools), `gui\`
  (the two baked display pages), `plan\`,
  `INSTRUCTIONS_FOR_TEACHERS_AND_STUDENTS.md`; `PLANET_MERCURY_PROVENANCE.md`
  at the root. `planet_Mercury\data\` is regenerated, not committed.
- `SolveIt_Notebooks_for_rust\` — the encyclopedia: `ENCYCLOPEDIA.md`
  / `.html` / `.pdf`, seven topic folders of notebook copies with
  players, `encyclopedia\images\` (124 PNGs), `_tools\` (the pipeline
  and its execution logs), `README.md`.
- `rebound_rust\` (`rebound_rs`, `rebound_rust.md` / `.pdf` — the
  master guide — `notebooks\`, `examples\`, `porttest\`, `logs\`) and
  `reboundx_rust\` (`reboundx_rs`, `examples\`, `porttest\`).
- `sundials_rs\` — the vendored Windows engine (read-only; upstream
  any change to `once-ere/SUNDIALS_7_8_Rust_port_for_Windows11`).

## What is and is not claimed

- **Every number on this page was measured on Windows 11 on this
  machine.** No macOS or Linux figure is presented as a Windows
  measurement; where a macOS value is mentioned it is labelled as
  such. The macOS project's measurements remain in the four lineage
  pages, verbatim, under their banner.
- **The physics is byte-identical to the Linux release wherever the
  vendored engine's own libm does the arithmetic.** The three
  exception classes, all of which pass through the *host* libm (UCRT
  here, Apple libm on macOS, glibc on Linux), are: (1) the two
  quantum notebooks `double_slit` and `tunneling` — last printed digits
  only, pinned in `evidence\win11\accepted-divergences-dynamic.diff`,
  unchanged since the first Windows port; (2) the GEAR/RACK joint
  residual — `videos\rack_and_pinion.html` is the Windows recording,
  and it reproduces byte for byte here; (3) new in this port, the
  fourteen curvilinear coordinate maps of
  `special_functions\src\coords.rs` (SolveIt 37–38): six of the 252
  datasets differ from the macOS files in a handful of values —
  `solveit37_pol` 12 of 1400 float values, worst relative 1.99e-16;
  `solveit37_sph` 3/1400, 1.30e-16; `solveit38_bis` 3/700, 1.58e-15;
  `solveit38_cyl` 5/700, 1.79e-16; `solveit38_obl` 16/700, 2.67e-14;
  `solveit38_tor` 5/700, 4.02e-16 (plus `solveit37_sph_plots.html`,
  derived from one of them) — and two transcript lines of examples
  37/38 print different last digits (example 37 `Out[21]=
  [3.0000000000000004, 3.9999999999999996]`, example 38 `Out[22]=
  0.0000000000000007447602459741819`; every other numeric line of
  36–38 is verbatim identical). `SolveIt.md` carries the Windows
  values. The `planet_Mercury` gauntlets belong to the same class:
  `mercury_rs` evaluates its tidal and triaxial torques with the host
  libm while CVODE runs on the vendored engine, so the Windows
  figures differ from the committed macOS ones in their trailing
  digits and in the bit-level path-sensitive 64-branch capture tally
  (Windows 5/64 = 0.0781 ± 0.034 vs macOS 10/64; the notebook's own
  text states that 64-flip error bars apply); every assertion of both
  gauntlets passes on both platforms, and the Windows numbers are
  deterministic run to run on this machine.
- **The rebound ports are bit-identical to the MSVC-compiled C
  originals on every gate, with one characterised difference:** `pow`,
  where Rust's implementation and the UCRT disagree on 60 of 200,000
  samples (0.0300 %), each by exactly 1 ULP; `sin`, `cos`, `tan`,
  `atan2`, `sqrt`, `fmod`, `exp`, `log` showed 0 differences. The
  `libm_diff` notebook's final cell tallies 60 differing *lines* by
  their first bit pattern; per function the difference is `pow` only.
  The REBOUNDx tree is an unpacked 5.1.0 archive with no commit id
  (`version.txt`); the REBOUND reference is commit dad5f97
  (5.1.1-4-gdad5f978). Part of the C reference build (librebound.dll
  / .lib, 31 objects, the 11 REBOUND harness executables, the Rust
  example binaries) was reused from an earlier session on the same
  machine with flags proven by its make/harness logs; the REBOUNDx
  library, its 31 objects, the two MSVC VLA shim objects and the 5
  REBOUNDx harnesses were rebuilt fresh.
- **Closed after this page was first written** (recorded in the logs,
  then claimed): the final `scripts\certify_clean.sh` run after the
  clippy rewrites and the live-test-count sentence fixes —
  **every gate PASS in the final run (exit 0)** (`logs\certify_clean.log`; the preliminary run,
  `logs\certify_clean_prelim.log`, had passed every gate except clippy
  and the five stale "568/622 tests" sentences, both since fixed) — and
  the re-run of the gate, the dataset byte-compare and the tests after
  the last of the behaviour-preserving clippy rewrites (commit
  `a5082b2`, 7 files): the gate on the final release binary
  (`logs\win_verify_physics.log`, 18:04) exit 0 with the same three
  verdicts, `datasets\solveit20_kepler.db` byte-identical when re-written
  by that binary, and 638 passed, 0 failed in the certifier's own `cargo test
  --workspace`. Earlier records: `logs\test_final.log` (16:53, 638
  passed) and `logs\clippy_workspace.log` (16:56, 0 errors). No
  `sundials_rs`, `rebound_rust\src` or `reboundx_rust\src` file was
  edited by this port.
- **What was not measured here at all:** run-time performance (the
  few timings quoted — 18.22 s sidecar build, 35 s index verification,
  the shearing-sheet and tides_spin wall-clock seconds — are
  incidental, single runs); anything on a CPU without FMA (the build
  requires it); any compiler other than the MSVC toolset named above.
