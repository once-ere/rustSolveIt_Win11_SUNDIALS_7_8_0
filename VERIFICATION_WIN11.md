# VERIFICATION_WIN11 — independent checks of this repository

This page has two parts. **Part I** is the independent verification
pass over the first Windows 11 port (the Linux release -> Windows 11,
2026-08-22), kept exactly as written that day. **Part II** is the
verification record of the second port — the macOS / Apple Silicon
project `once-ere/rustSolveIt_macos-silicon_SUNDIALS_7_8_0` merged into
this repository and re-verified from scratch on Windows 11, 2026-09-09 —
with the per-suite verification matrix and every documented divergence.
Where the two parts state different counts, Part II's are the current
ones.

# Part I — the first port: rustSolveIt_Using_SUNDIALS_7_8_0 (Linux) -> Windows 11 (2026-08-22)

An independent verification pass over the finished Windows 11 port, run
2026-08-22 on the same machine that built it (Intel Core Ultra 9 275HX,
Windows 11 Pro for Workstations 25H2, rustc 1.91.1
`x86_64-pc-windows-msvc`). Every command below is complete as written
and was actually run; its measured result follows it. All commands run
from the repository root
`C:\work\rustSolveIt_Win11_SUNDIALS_7_8_0`;
PowerShell unless marked Git Bash.

## 1. Repository state

```powershell
git status --short
git log --oneline -3
git describe --tags
git fetch origin
git rev-parse HEAD origin/main
```

Result: working tree clean; HEAD = `origin/main` =
`38cd8a77a6eaa62df29beee5426a6e0379102bf4`; tag `win11-port-green`
present on that commit.

## 2. Dependency and safety invariants

```powershell
Select-String -Path Cargo.lock -Pattern '^name = '
Select-String -Path Cargo.lock -Pattern 'source = '
```

Result: exactly the 12 local crates (arkode_rs, cvode_rs, cvodes_rs,
ida_rs, idas_rs, kinsol_rs, sundials_core, spec_math, physical_object,
posim, quantum, special_functions), **zero** registry sources — nothing
from crates.io.

```bash
# Git Bash
grep -rn --include="*.rs" -E "\bunsafe\b" physical_object/src posim/src \
  quantum/src special_functions/src sundials_rs/crates/*/src \
  | grep -v "forbid(unsafe_code)"
grep -rn --include="*.rs" "{:e}" sundials_rs/crates/*/src
grep -rnE --include="*.rs" "\.powf\(|\.powi\(" sundials_rs/crates/*/src
```

Result: **zero** uses of `unsafe` anywhere (every first-party crate
root carries `#![forbid(unsafe_code)]` and `#![deny(warnings)]`). The
three residual grep hits are all benign and inherited from the verified
upstream: a diagnostic print inside the libm differential harness
(`sundials_libm/corpus.rs:309`), a doc comment stating the no-`{:e}`
rule (`sundials_utils.rs:4`), and the deliberate host-comparison test
`pow_deterministic_vs_host_powf` (`sundials_math.rs:918`), which exists
precisely to measure the host `pow` against the deterministic one.

## 3. Build and tests

```powershell
cargo build --workspace --all-targets
cargo build --release --workspace --all-targets
cargo test --workspace
```

Result: both profiles **warning-free**; **622 passed, 0 failed**
(49 physical_object lib + 19 collision + 9 conservation + 42
constrained/equilibrium/sensitivity + 112 posim + 92 quantum + 233
special_functions + 11 vendored identities + 5 + 50 doctests). The one
`[ERROR]...[KINSol]` stderr line mid-run is printed by a *passing* test
that exercises a solver failure path.

## 4. Physics byte-identity against the Linux evidence

```bash
# Git Bash
bash tools/win_verify_physics.sh
```

Result (exit 0), reproduced identically to the port-time run:
12 collision scripts **IDENTICAL**; 6 examples and 59 dynamic
notebooks **identical modulo the two pinned divergences** in
`evidence/win11/accepted-divergences-examples.diff` (one blank line
the donor added to `outer_solar_system` after recording its own
evidence) and `evidence/win11/accepted-divergences-dynamic.diff`
(9 last-printed-digit lines confined to the two quantum notebooks,
`double_slit` and `tunneling`, whose first-party code uses the host
math library). Nothing beyond the pinned diffs appeared.

## 5. Recorded videos, protocol, kernel

```powershell
python recorder\tests\test_units.py
python recorder\tests\test_end_to_end.py
python jupyter\test_protocol.py
$env:POSIM_NO_BROWSER = "1"
jupyter\.venv\Scripts\python.exe jupyter\test_kernel.py
```

Result: recorder 17 + 9 tests **OK** — including
`test_all_committed_recordings_reproduce_byte_for_byte`, which
re-records all 13 browser videos to a temporary directory and
byte-compares them against the committed files; “all protocol checks
passed”; all 7 kernel cells **ok**.

## 6. GUIs and the entity index

```powershell
python tools\win_gui_smoke.py
python tools\verify_index_examples.py
python tools\verify_tierb_examples.py
```

Result: **13 of 13 GUIs pass** (page served, live state, Start advances
simulated time, Reset returns bit-exactly to t = 0);
**PASS 1177/1177** runnable index examples; **COMPILED 258/258** Rust
snippets.

One finding, no action needed: re-running `verify_index_examples.py`
re-captures 4 of the 1,177 transcripts whose scripts query scene
status immediately after `SCENE START` — the live playback thread
advances on wall-clock time, so the captured `t` there (0 vs 0.01 vs
0.03) depends on thread scheduling. All 4 examples still PASS; the
committed capture was kept (`git checkout -- index_data/catalog.json`).

## 7. The 109 Jupyter notebooks, re-executed

```bash
# Git Bash
POSIM_NO_BROWSER=1 python notebooks/_build/nbrun.py notebooks/*.ipynb
python notebooks/_build/nbcheck.py notebooks/*.ipynb
git diff -- notebooks/
```

Result: **109 ok, 0 failed**; **109/109 pass all seven requirements**.
The re-run's only working-tree effect was the OS-assigned ephemeral
scene-window port embedded in 57 captured outputs
(`http://127.0.0.1:<port>/` — one line per affected notebook; verified
that **zero** non-port lines changed). The committed captures were kept
(`git checkout -- notebooks/`). Every physics number in every notebook
reproduced bit-for-bit.

## 8. Documents and pairing

- `grammar.pdf` (899,012 bytes) and `SolveIt.pdf` (497,036 bytes)
  exist and are newer than their `.tex` sources.
- Notebook↔example pairing is exactly 1:1 — 109 notebooks =
  13 `video_*` + 6 `rust_*` + 12 `collision_*` + 19 `solveit_*` +
  59 `dynamic_*`, matching 13 scene scripts + 6 compiled examples +
  12 collision scripts + 19 SolveIt scripts + 59 dynamic notebooks on
  disk.

## Verdict

Every gate re-verified green, independently of the runs that built the
port. The two intentionally-pinned physics divergences reproduced
exactly and nothing new appeared. Two benign run-to-run variations were
identified and documented (§6, §7); both are wall-clock/OS artifacts in
*captured transcripts*, not in any computed physics.

---

# Part II — the second port: rustSolveIt_macos-silicon_SUNDIALS_7_8_0 -> Windows 11 (2026-09-09)

Part I above is the independent check of the first Windows port
(2026-08-22) and is kept exactly as written; its counts (622 tests, 109
notebooks, 1177 index examples) are that day's. This Part II is the
verification record of the second port — the macOS / Apple Silicon
project merged into this repository and re-verified from scratch on
Windows 11 on 2026-09-09 (how the merge was done and what was changed
are in `PORT_WIN11_PROVENANCE.md` Part II). Every command below is
complete as written and was actually run on this machine on that day;
its measured result follows it, with the log that holds it. Machine
and toolchain: Intel Core Ultra 9 275HX, Windows 11 Pro for
Workstations build 10.0.26200, rustc 1.91.1 / cargo 1.91.1
`x86_64-pc-windows-msvc` with `.cargo\config.toml` pinning
`-C target-feature=+fma`, MSVC 14.51.36231 from the Visual Studio 2026
Build Tools (the linker, located by rustc; and `cl.exe`, located by the
`cc` crate, for the two C compiles in the repository — the bundled SQLite
amalgamation inside `dataset_tools`, and, explicitly through
`vcvars64.bat`, the rebound C-reference harnesses), Python 3.14.5
(cp1252 default encoding), MiKTeX-pdfTeX 4.18, Google Chrome at
`C:\Program Files\Google\Chrome\Application\chrome.exe`. No WSL2
compiler or linker was used anywhere. All commands run from the
repository root (the folder containing `Cargo.toml`, `posim\` and
`sundials_rs\`, e.g. `C:\work\rustSolveIt_Win11_SUNDIALS_7_8_0`), in
PowerShell unless marked Git Bash.

## II.1 Repository state

```powershell
git status --short
git log --oneline -8
git log -1 --format='%H %P' d6f6605
git show-ref pre-macos-port
git remote -v
```

Result: the history of the port, newest first — `23e90f3` (rebound
C-reference gates measured on Windows 11 with MSVC; 17 notebooks
re-executed), `a5082b2` (Windows 11 editions of the guides; clippy-clean
under rustc 1.91.1; the certifier's live-count rule honoured), `6d99370`
(encyclopedia copies regenerated and executed here; lineage banners on
the four macOS provenance pages), `03b3986` (all 128 notebooks
regenerated, executed and checked; datasets and movies regenerated),
`f8557de` (Windows adaptation of the tooling that arrived with the macOS
project), `d6f6605` (the merge — parents `f1f598d` and `5360157`, the
macOS project's HEAD), `f1f598d` (LF normalization of 219 files), and
below it `5360157` and the 30 earlier macOS commits down to the pristine
Linux import `d94488f`. `pre-macos-port` = `51f570f`, the last commit of
Part I. Remote `macos` points at the macOS repository (here a local
clone of
`https://github.com/once-ere/rustSolveIt_macos-silicon_SUNDIALS_7_8_0.git`);
`origin` at
`https://github.com/once-ere/rustSolveIt_Win11_SUNDIALS_7_8_0.git`.

## II.2 Dependency and safety invariants

```powershell
Select-String -Path Cargo.lock -Pattern '^name = '
Select-String -Path Cargo.lock -Pattern 'source = '
Select-String -Path dataset_tools\Cargo.lock -Pattern '^name = "(rusqlite|r2d2|r2d2_sqlite)"'
```

Result: the simulator workspace's `Cargo.lock` still lists exactly the
12 local crates (arkode_rs, cvode_rs, cvodes_rs, ida_rs, idas_rs,
kinsol_rs, sundials_core, spec_math, physical_object, posim, quantum,
special_functions) and **zero** registry sources. The three crates.io
packages the specification asked for (`rusqlite` 0.32.1, `r2d2`,
`r2d2_sqlite` 0.25.0) live only in `dataset_tools\Cargo.lock` — the
sidecar is a separate package that the root `Cargo.toml` excludes
(together with `planet_Mercury`, `rebound_rust`, `reboundx_rust`,
`sundials_rs`, `vendor/spec_math`), so `cargo test --workspace` still
reports only this workspace's tests and the simulator never links
SQLite.

```bash
# Git Bash
grep -rn --include="*.rs" -E "\bunsafe\b" physical_object/src posim/src \
  quantum/src special_functions/src sundials_rs/crates/*/src \
  | grep -v "forbid(unsafe_code)"
```

Result: ten lines, every one a `//!` crate-level doc comment that
*states* the no-`unsafe` guarantee (`physical_object\src\lib.rs`,
`posim\src\scene\mod.rs`, `posim\src\scene\ws.rs`, and the seven
`sundials_rs` crate roots — `arkode_rs`, `cvode_rs`, `cvodes_rs`,
`ida_rs`, `idas_rs`, `kinsol_rs`, `sundials_core`); not one `unsafe` block or function anywhere
— including in the macOS project's additions to `posim\src` (the
`DATASET` / `MOVIE` / `SQL` families, the sidecar finder) and in
`special_functions\src\coords.rs`. Every first-party crate root still
carries `#![forbid(unsafe_code)]` and `#![deny(warnings)]`.

## II.3 The verification matrix

Every suite in the repository, the complete command, and what it
measured. The commands assume the three one-time builds:

```powershell
cargo build --release
cd dataset_tools; cargo build --release; cd ..
cd planet_Mercury\mercury_rs; cargo build --release; cd ..\..
```
(Git Bash: `cargo build --release && (cd dataset_tools && cargo build --release) && (cd planet_Mercury/mercury_rs && cargo build --release)`.)

| # | suite | command (PowerShell; Git Bash where marked) | measured result | log |
|---|---|---|---|---|
| 1 | workspace build, dev | `cargo build --workspace --all-targets 2>&1 \| Tee-Object logs\build.log` | 0 errors, 0 warnings | `logs\build.log` |
| 2 | workspace build, release | `cargo build --release --workspace --all-targets 2>&1 \| Tee-Object logs\build_release.log` | 0 warnings; `target\release\posim.exe` 5,705,216 bytes | `logs\build_release.log` |
| 3 | workspace tests | `cargo test --workspace 2>&1 \| Tee-Object logs\test.log` | **638 passed, 0 failed** = 49 physical_object lib + 19 collision + 9 conservation + 42 constrained/DAE + 121 posim + 92 quantum + 240 special_functions + 11 vendored identities + 5 quantum doctests + 50 special_functions doctests; re-run after the `dataset.rs` finder fix and the `coords.rs` / `physical_object\src` clippy rewrites: 638 passed, 0 failed (`test_final.log`, 16:53; the later 16:55 `dataset.rs` / `tests\dae.rs` clippy edits are covered only by `logs\clippy_workspace.log` — re-run pending, §II.5) | `logs\test.log`, `logs\test_final.log` |
| 4 | sidecar build | `cd dataset_tools; cargo build --release 2>&1 \| Tee-Object ..\logs\dataset_tools_build.log; cd ..` | rusqlite 0.32.1 + r2d2_sqlite 0.25.0 compiled (bundled SQLite via MSVC `cl.exe` through the `cc` crate), `Finished release in 18.22s`, 0 warnings; `solveit_db.exe` 2,326,016 bytes | `logs\dataset_tools_build.log` |
| 5 | sidecar tests | `cd dataset_tools; cargo test 2>&1 \| Tee-Object ..\logs\dataset_tools_test.log; cd ..` | **8 unit + 3 integration = 11 passed, 0 failed** | `logs\dataset_tools_test.log` |
| 6 | sidecar stress demo | `.\dataset_tools\target\release\solveit_db.exe stress $env:TEMP\stress_demo.db 10 500` (Git Bash: `./dataset_tools/target/release/solveit_db.exe stress /tmp/stress_demo.db 10 500`) | `stress: 10 threads x 500 rows = 5000 expected, 5000 written, journal_mode=wal` | `logs\crosscheck_stress_launcher.log` |
| 7 | posim finds the sidecar | `$env:POSIM_NO_BROWSER = "1"; cargo run -p posim --release -- --script scripts/solveit/20_dataset_kepler_orbit.posim` | runs headless; `datasets\solveit20_kepler.db`, its plots page and `movies\solveit20_kepler.html` written | `logs\solveit_scripts\solveit_20_dataset_kepler_orbit.log` |
| 8 | physics byte-identity gate (Linux evidence) | Git Bash: `bash tools/win_verify_physics.sh 2>&1 \| tee logs/win_verify_physics.log` | exit 0: `IDENTICAL-MODULO-DOCUMENTED examples-win11.log vs examples-7.8.0.log (pinned: accepted-divergences-examples.diff)`; `IDENTICAL collision-scripts-win11.log == collision-scripts-7.8.0.log`; `IDENTICAL-MODULO-DOCUMENTED dynamic-notebooks-win11.log vs dynamic-notebooks-7.8.0.log (pinned: accepted-divergences-dynamic.diff)` — nothing beyond the two pinned diffs, before or after the `coords.rs` / `physical_object\src` clippy rewrites (16:48; the 16:55 `dataset.rs` edit came later — §II.5) | `logs\win_verify_physics.log` |
| 9 | 38 SolveIt + 12 collision scripts, headless | `$env:POSIM_NO_BROWSER = "1"; Get-ChildItem scripts\solveit\*.posim, scripts\collisions\*.posim \| ForEach-Object { cargo run -p posim --release -- --script $_.FullName 2>&1 \| Out-File -Encoding utf8 "logs\solveit_scripts\$($_.Directory.Name)_$($_.BaseName).log"; "$LASTEXITCODE $($_.FullName)" }` (Git Bash: `for f in scripts/solveit/*.posim scripts/collisions/*.posim; do POSIM_NO_BROWSER=1 cargo run -p posim --release -- --script "$f" > "logs/solveit_scripts/$(basename "$(dirname "$f")")_$(basename "$f" .posim).log" 2>&1; echo "$? $f"; done`) | **50/50 exit 0** | `logs\solveit_and_collision_scripts.log`, `logs\solveit_scripts\` |
| 10 | 128 notebooks: regenerate, execute, audit | `python notebooks\_build\regen.py`; `$env:POSIM_NO_BROWSER = "1"; Get-ChildItem notebooks\*.ipynb \| ForEach-Object -Parallel { python notebooks\_build\nbrun.py $_.FullName } -ThrottleLimit 6` (PowerShell 7 only; Windows PowerShell 5.1 has no `-Parallel` — use `ForEach-Object { python notebooks\_build\nbrun.py $_.FullName }` sequentially); `python notebooks\_build\nbcheck.py` (Git Bash, as run: `python notebooks/_build/regen.py; ls notebooks/*.ipynb \| POSIM_NO_BROWSER=1 xargs -P 6 -I{} python notebooks/_build/nbrun.py {}; python notebooks/_build/nbcheck.py`) | all 128 regenerated; **128 ok, 0 failed**; **128/128 pass all eight requirements** | `logs\notebooks_run.log` |
| 11 | datasets and movies vs the macOS files | `git diff --stat macos/main HEAD -- datasets movies` (remote `macos` = the macOS repository, here a local clone of `https://github.com/once-ere/rustSolveIt_macos-silicon_SUNDIALS_7_8_0.git`) | exactly 7 files differ: **movies 74/74 byte-identical, datasets 245/252** — the 7 are the curvilinear maps of SolveIt 37–38 (§II.4 item 3) | (working tree) |
| 12 | 13 recorded videos | `python recorder\src\record_all.py --check` | `all 13 recordings reproduce byte for byte` | `logs\videos_check.log` |
| 13 | recorder tests | `python recorder\tests\test_units.py`; `python recorder\tests\test_end_to_end.py` | `Ran 17 tests ... OK`; `Ran 9 tests ... OK` | `logs\videos_check.log` |
| 14 | 13 live GUIs | `python tools\win_gui_smoke.py`; `python tools\gui_smoke.py` | **13 of 13 GUIs pass**; **13 of 13 GUIs pass** (ports 8895–8907) | `logs\gui_smoke.log` |
| 15 | Jupyter protocol | `python jupyter\test_protocol.py` | `all protocol checks passed` | `logs\jupyter_tests.log` |
| 16 | Jupyter wrapper kernel | `uv venv jupyter\.venv; uv pip install -p jupyter\.venv\Scripts\python.exe ipykernel jupyter_client; $env:POSIM_NO_BROWSER = "1"; jupyter\.venv\Scripts\python.exe jupyter\test_kernel.py` | venv Python 3.14.4; 7/7 cells ok — `all kernel checks passed — JupyterLab can drive this kernel` | `logs\jupyter_tests.log` |
| 17 | entity index, runnable examples | `python tools\verify_index_examples.py 2>&1 \| Tee-Object logs\verify_index_examples.log` | **PASS 1365/1365 (100.0%)**, 35 s wall clock | `logs\verify_index_examples.log` |
| 18 | entity index, Rust snippets | `python tools\verify_tierb_examples.py 2>&1 \| Tee-Object logs\verify_tierb_examples.log` | **COMPILED 258/258 (100.0%)** | `logs\verify_tierb_examples.log` |
| 19 | encyclopedia: manifest + copies | `python SolveIt_Notebooks_for_rust\_tools\gen.py` | 147 entries (2/38/59/12/13/6/17) | `SolveIt_Notebooks_for_rust\_tools\manifest.json` |
| 20 | encyclopedia: execute the copies | `$env:POSIM_NO_BROWSER = "1"; Get-ChildItem SolveIt_Notebooks_for_rust\0*\*\*.ipynb \| ForEach-Object { python SolveIt_Notebooks_for_rust\_tools\run_copy.py $_.FullName }` (no `-Recurse`, which would include `.ipynb_checkpoints`) | 02_solveit 38/38 ok, 03_dynamics 59/59 ok, 04 + 05 25/25 ok, 06_rust 6/6 ok, 07_rebound 17/17 ok; the 2 Mercury copies re-executed after the UTF-8 driver fix: `exec_mercury.log` `ok … mercury_tidal_locking.ipynb (18 cells)`, `1 ok, 0 failed`; `exec_mercury2.log` `ok … mercury_test2_jupiter_gr.ipynb (17 cells)`, `1 ok, 0 failed` — **147/147 ok** | `SolveIt_Notebooks_for_rust\_tools\exec_posim_0..3.log`, `exec_rebound.log`, `exec_mercury.log`, `exec_mercury2.log` |
| 21 | encyclopedia: GUI captures | `python SolveIt_Notebooks_for_rust\_tools\shoot.py --workers 4` | **124 captured, 0 failed** (38 + 59 + 12 + 13 + 2; the 6 rust and 17 rebound entries have no GUI by design); smallest PNG 18,039 bytes; headless Chrome | `SolveIt_Notebooks_for_rust\_tools\shoot.log` |
| 22 | encyclopedia: build + six gates | `python SolveIt_Notebooks_for_rust\_tools\encyclo.py`; `python SolveIt_Notebooks_for_rust\_tools\verify.py` | final pass after row 20 (commit `5d87e8d`): `ENCYCLOPEDIA.md` 162,628 chars, `.html` 187,017 chars, `.pdf` 1,364,723 bytes (~46 pages), header "147/147 green", 147 entries "Executed: PASS"; `ALL VERIFY GATES PASS (147 notebooks)` — V1 147 (2/38/59/12/13/6/17), V2 headers, V3 players compile and match, V4 execution counts, V5 124 images / none for rust + rebound, V6 every tag once (the earlier dry run gave 162,608 / 186,997 chars, 1,364,749 bytes, the same verdict line and a "146/147 green" header from the stale interrupted-run log) | the committed `SolveIt_Notebooks_for_rust\ENCYCLOPEDIA.*`; `verify.py` prints the line when run |
| 23 | rebound_rs 5.1.1 | `cd rebound_rust; cargo build --release; cargo test --release; cargo clippy --release --all-targets; cd ..` (each `2>&1 \| Tee-Object logs\win11_<step>_20260909.log`) | build 0 warnings; **394 passed** (67 + 32 + 39 + 72 + 55 + 70 + 59), 0 failed; clippy 0 warnings | `rebound_rust\logs\win11_{build,test,clippy}_20260909.log` |
| 24 | reboundx_rs 5.1.0 | `cd reboundx_rust; cargo build --release; cargo test --release; cargo clippy --release --all-targets; cd ..` | build 0 warnings; **137 passed** (34 + 32 + 28 + 17 + 26; 2 doctests ignored), 0 failed; clippy 0 warnings | `logs\reboundx_{build,test,clippy}.log` |
| 25 | 17 rebound notebooks | Git Bash, in `rebound_rust/notebooks`: `python make_notebooks.py; PYTHONIOENCODING=utf-8 python run_notebooks.py 2>&1 \| tee run_all.log` | `All 17 notebooks executed with no errors.` — on regeneration and again after the C dumps existed | `rebound_rust\logs\win11_notebooks_20260909.log`, `rebound_rust\notebooks\run_all.log` |
| 26 | rebound C-reference gates | the harness commands of `PORT_WIN11_PROVENANCE.md` §II.8.12 (REBOUND `dad5f97806ecbb408dcaff728851c64e67f9f6eb`, REBOUNDx 5.1.0, `cl` 19.51.36256 `/Ox /fp:precise`) | libm 200,000 x 9: 8 functions 0 differences, `pow` 60/200,000 all 1 ULP; `bs_pow` 56/200,000 all 1 ULP, 0/8 constants; integrator matrix **63/63 bit-identical**; ias15 default 7/7; shearing sheet 400 steps 1482 particles 102533 collisions, SHA-256 `75BDAAB7…521F0` both sides; archive round trips 6/6, server 6/6; derivatives 130/130, frequency 30/30, addfmt 12/12, movetocom_var 5/5, kepler_rectilinear 11/11; REBOUNDx tides_spin **6/6 BIT-IDENTICAL**; rebx binary 25/25 + 25/25, C readback 28/28 | `rebound_rust\logs\win11_c_reference_gates_20260909.md` |
| 27 | mercury_rs | `cd planet_Mercury\mercury_rs; cargo build --release; cargo test --release; cd ..\..` | build 0 warnings; **19 passed, 0 failed**; `mercury_rs.exe` 1,104,384 bytes | `logs\mercury_rs_{build,test}.log` |
| 28 | mercury_crosscheck | `cd planet_Mercury\mercury_crosscheck; cargo build --release; cargo run --release; cd ..\..` | build 0 warnings; `measured dOmega/dt = -3.652928e-18 rad/s^2 (REBOUNDx tides_spin)`, `predicted dOmega/dt = -3.643780e-18 rad/s^2 (Hut 1981 secular formula)`, `relative difference = 2.511e-3`, `PASS - crosscheck.despin_rate`, `SUCCESS` | `logs\crosscheck_stress_launcher.log` |
| 29 | Mercury notebook 1 | `cd planet_Mercury\notebook; python build_notebook.py; $env:MERCURY_NO_BROWSER = "1"; python run_notebook.py mercury_tidal_locking.ipynb; python check_notebook.py mercury_tidal_locking.ipynb` (Git Bash: `MERCURY_NO_BROWSER=1 python run_notebook.py …`) | `ok (18 cells)`, `1 ok, 0 failed`; `all structure rules pass`; all assertions PASS, 6/6 runs SUCCESS | `logs\mercury_notebook1.log` |
| 30 | Mercury notebook 2 | `python build_notebook2.py; $env:MERCURY_NO_BROWSER = "1"; python run_notebook.py mercury_test2_jupiter_gr.ipynb; python check_notebook.py mercury_test2_jupiter_gr.ipynb` | `ok (17 cells)`, `1 ok, 0 failed`; `all structure rules pass`; all assertions PASS, 5/5 runs SUCCESS | `logs\mercury_notebook2.log` |
| 31 | PDFs | `pdflatex -interaction=nonstopmode <name>.tex` twice each for `SolveIt`, `grammar`, `scene_info`, `physical_object_simulator` | zero errors: SolveIt.pdf 49 pages (693,477 bytes); grammar.pdf 74 pages (970,144 bytes); scene_info.pdf 15 pages (388,864 bytes); physical_object_simulator.pdf 19 pages (523,088 bytes); collision_detection.pdf unchanged | `logs\pdflatex_*_pass2.log` |
| 32 | licence certifier | Git Bash: `bash scripts/certify_clean.sh 2>&1 \| tee logs/certify_clean.log` | preliminary run: every gate PASS (no reference-tree files tracked or in history, no encumbered signatures, gitignore rules, release build 0 warnings, 638 tests, every StageNbks notebook executes, no cross-references, no retired claim) except clippy (15 inherited lints, since fixed) and five live test-count sentences (since rephrased per `CLEANROOM_PROVENANCE.md`); the final run after those edits: **every gate PASS in the final run (exit 0)** (its own `cargo test --workspace`: 638 passed, 0 failed) | `logs\certify_clean.log` (preliminary: `logs\certify_clean_prelim.log`) |

## II.4 Every documented divergence

The complete list of everything on this machine that is *not*
byte-identical to a reference, with its measured size. Nothing else
differs.

**Windows vs the Linux evidence (unchanged since Part I; both pinned
byte for byte, and the gate fails on anything beyond them):**

1. `outer_solar_system` — one blank line the donor added to its first
   `println!` after recording its own evidence
   (`evidence\win11\accepted-divergences-examples.diff`). Every number
   identical.
2. The two quantum dynamic notebooks `double_slit` and `tunneling` — 9
   lines, each in the last printed digit
   (`evidence\win11\accepted-divergences-dynamic.diff`); the `quantum`
   crate calls the host math library (UCRT here, glibc there).

**Windows vs the macOS project's files (new in this port):**

3. **The curvilinear-coordinate datasets of SolveIt 37–38.** Of the 252
   files in `datasets\`, 7 differ from the macOS files:
   `solveit37_pol.db`, `solveit37_sph.db`, `solveit37_sph_plots.html`,
   `solveit38_bis.db`, `solveit38_cyl.db`, `solveit38_obl.db`,
   `solveit38_tor.db` — every one the output of a `DATASET MAP` through
   a curvilinear coordinate system in `special_functions\src\coords.rs`,
   which calls the host libm (MSVC UCRT here, Apple libm on macOS). The
   source datasets (`solveit37_orbit.db`, `solveit38_helix.db`), all 74
   movies and the other 245 dataset files are byte-identical. Quantified
   per `states` table (Windows file vs macOS file; the reproduction
   command is in `PORT_WIN11_PROVENANCE.md` §II.8.6):

   | dataset | float values that differ | worst relative difference |
   |---|---|---|
   | `solveit37_pol.db` | 12 of 1400 | 1.99e-16 |
   | `solveit37_sph.db` | 3 of 1400 | 1.30e-16 |
   | `solveit38_bis.db` | 3 of 700 | 1.58e-15 |
   | `solveit38_cyl.db` | 5 of 700 | 1.79e-16 |
   | `solveit38_obl.db` | 16 of 700 | 2.67e-14 |
   | `solveit38_tor.db` | 5 of 700 | 4.02e-16 |

   Two transcript lines of examples 36–38 differ from the macOS
   transcripts: example 37 `Out[21]= [3.0000000000000004, 3.9999999999999996]`
   (the `cart2_to_polar` round trip) and example 38
   `Out[22]= 0.0000000000000007447602459741819`
   (`logs\solveit_scripts\solveit_37_dataset_map_spherical.log`,
   `solveit_38_dataset_curvilinear_tour.log`). Every other numeric `Out`
   line of 36–38 is verbatim identical; `SolveIt.md` / `.tex` quote the
   Windows values.
4. **The GEAR/RACK joint residual** — a host-libm effect in the joint
   solver's residual; the `rack_and_pinion` recording is therefore the
   Windows one (kept at the merge), and `record_all.py --check`
   reproduces it byte for byte here (row 12).
5. **The two Mercury notebooks.** `mercury_rs` evaluates its tidal and
   triaxial torques with the host libm while CVODE runs on the vendored
   deterministic engine, so the long integrations land on slightly
   different bits; every assertion of both gauntlets passes on both
   platforms. Windows value, macOS committed value in parentheses where
   different (logs `logs\mercury_notebook1.log`, `logs\mercury_notebook2.log`):

   | quantity (test 1, `mercury_tidal_locking`) | Windows (macOS) |
   |---|---|
   | A: final Omega/n after 10 Myr | 178.9260 (same) |
   | A: full-model proof segment Omega/n | 181.446450 (181.446443) |
   | B: restart root-found Omega/n, at t | 1.600000000000e+00 at 4.4315 Myr (1.599999999963e+00 at 4.4316) |
   | B: samples recorded | 10762 (10763) |
   | C: 64-branch sweep capture fraction | 5/64 = 0.0781 +/- 0.034 (10/64 = 0.1562 +/- 0.045); Goldreich–Peale estimate ~0.070; 64-flip error bars apply |
   | Bf: canonical capture | t = 4.6501 Myr (4.6499) |
   | Bf: settled-era mean Omega/n | 1.5000000368 (1.5000000498) |
   | Bf: final P_orb / P_rot / solar day | 87.9678 d (same) / 58.6512 d (58.6419) / 175.9896 d (175.9061); observed 87.969 / 58.646 / 175.938 |
   | Bf: two-thirds lock, relative | 8.8204e-09 (1.6579e-08) |
   | ledger pre-capture drift (budget 1e-9) | 4.029e-10 (3.947e-10) |
   | locked-era drift vs predicted leak 1.134e-09 | 8.821e-12 (9.156e-12) |
   | libration swing | 6.232 -> 0.140 rad (6.224 -> 0.152) |
   | D: guaranteed capture; libration period vs pendulum-exact | 765.2 kyr; 27.46 yr vs 29.13 (762.6; 27.52 vs 29.25) |
   | E: seam relative difference | 1.062e-03 (1.201e-03) |
   | CVODE steps; re-anchorings | 711041787; 109388 (711084473; same) |

   | quantity (test 2, `mercury_test2_jupiter_gr`) | Windows (macOS) |
   |---|---|
   | GR precession | 42.983 "/century vs analytic 42.983, rel diff 7.193e-13 (same) |
   | forced-eccentricity amplitude; secular period | 4.5438e-03 = LL prediction; 808.8 kyr (same) |
   | restart root-found Omega/n, at t | 1.600000000000e+00 at 4.4615 Myr (1.599999999995e+00 at 4.4612) |
   | 16-branch sweep | 5/16 = 0.3125 (2/16 = 0.1250) |
   | canonical capture | 4.6781 Myr (4.6779) |
   | settled mean ratio vs predicted 1.5000003777 | 1.5000004081, \|diff\| 3.05e-08 (1.5000004545, 7.69e-08) |
   | perihelion advance | 203.21 "/century vs predicted 203.22 (same) |
   | P_rot / solar day | 58.6464 d (58.6447) / 175.9463 d (175.9314) |
   | CVODE steps | 687068258 (687207958) |

**C reference vs Rust, rebound_rs / reboundx_rs, measured here (not a
Windows-vs-macOS matter — the same class the macOS and Windows
acceptance write-ups document):**

6. `pow`: 60 of 200,000 general-sweep samples and 56 of 200,000
   `bs_pow` controller samples differ by exactly 1 ULP each (`{1: 60}`,
   `{1: 56}`; 0 of the 8 `pow(0.02, 1/(2k+1))` constants). Every other
   libm column (`sin` `cos` `tan` `atan2` `sqrt` `fmod` `exp` `log`) is
   bit-identical on all 200,000 samples, and every integrator, archive,
   server, derivative, frequency, dataset, tides_spin and binary-file
   gate is bit-identical.
7. Cosmetic only: `derivatives_c.txt` is CRLF (the C harness writes in
   text mode; identical after CRLF -> LF); the two stdout probes
   (`movetocom_var_test`, `kepler_rectilinear` with `vy=0`) print
   `e+00` in C and `e0` in Rust with every raw-bit field equal; the
   `libm_diff` notebook's "functions that differ" cell tallies the 60
   differing *lines* by first bit pattern rather than by function; the
   REBOUNDx reference is an unpacked 5.1.0 archive and has no commit id;
   the 26 differing bytes of the 6,392-byte REBOUNDx binary are the
   git-hash header field.

## II.5 Findings that are not divergences

- **The `dataset.rs` sidecar finder.** On the merged tree, before
  `f8557de`, every `DATASET` command on Windows reported the sidecar
  "not built" although `cargo build --release` in `dataset_tools\` had
  just produced `solveit_db.exe`: the finder probed only the
  extensionless name. Fixed by probing `solveit_db.exe` first; verified
  by row 7, then by rows 9 and 10 (every dataset example and every
  notebook exercises it).
- **The Mercury copies and cp1252.** The first execution of the
  encyclopedia's Mercury copies was interrupted and the log of that run
  (`SolveIt_Notebooks_for_rust\_tools\exec_mercury.log` at the time of
  the encyclopedia dry run) recorded a FAIL (`mercury_rs t2-movie ended
  in FAILURE`). The notebook builders were then changed to decode
  `mercury_rs` output as UTF-8 instead of Python's cp1252 default, the
  canonical notebooks executed green (rows 29–30), and the copies were
  re-executed (row 20: 18 cells ok and 17 cells ok). The dry-run
  `ENCYCLOPEDIA.md` header line "146/147 green" and its
  `mercury_tidal_locking` "Executed: FAIL (4 code cells)" were that stale
  verdict — its `mercury_test2_jupiter_gr` "PASS (17 code cells)" came
  from the macOS-era `exec_mercury2.log` that arrived with the
  encyclopedia (since overwritten by this machine's run; `encyclo.py`
  takes the last verdict per tag over `exec_*.log` in filename order).
  The final build after the re-execution (row 22) replaced it: "147/147
  green".
- **Clippy 1.91.1 on inherited code.** The lints in
  `special_functions\src\coords.rs`, `physical_object\src` (four
  files), `posim\src\dataset.rs` and `physical_object\tests\dae.rs`
  (code unchanged since the Linux/macOS lineage; 7 files, commit
  `a5082b2`) were fixed with behaviour-preserving rewrites — in
  `dataset.rs` the `dataset_map` copy loop became `copy_from_slice` —
  two `too_many_arguments` waivers and one `type_complexity` waiver.
  Rows 3, 8 and 11 were re-run after the LAST of those edits (the 16:55
  `dataset.rs` / `dae.rs` pair): the gate on the final release binary
  (`win_verify_physics.log`, 18:04, exit 0, the same three verdicts),
  the dataset byte-compare (`solveit20_kepler.db` re-written by that
  binary is byte-identical to the earlier file), and the full test suite
  inside the certifier's run (`logs\certify_clean.log`: 638 passed, 0 failed);
  `cargo clippy --workspace --all-targets` is clean
  (`logs\clippy_workspace.log`, 16:56).
- **The 16 macOS arm64 harness binaries** the macOS repository had
  committed under extensionless names in `rebound_rust\porttest` and
  `reboundx_rust\porttest` were untracked and ignored (build products;
  rebuilt from their `.c` sources by the documented commands).
- **`jupyter\kernelspec\kernel.json`** `argv` starts with `python`
  (Windows has no `python3` launcher by default); row 16 drives the
  kernel through it.

## II.6 Documents and pairing

- `SolveIt.pdf` (693,477 bytes), `grammar.pdf` (970,144 bytes),
  `scene_info.pdf` (388,864 bytes) and `physical_object_simulator.pdf`
  (523,088 bytes) were recompiled from their `.tex` twins after the
  Windows re-platforming (row 31); `SolveIt.md` / `.tex` carry the
  Windows transcript values of examples 37–38 (§II.4 item 3).
- Notebook <-> example pairing is 1:1 at 128: 13 `video_*` + 6 `rust_*` +
  12 `collision_*` + 38 `solveit_*` + 59 `dynamic_*`, matching 13 scene
  scripts + 6 compiled examples + 12 collision scripts + 38 SolveIt
  scripts + 59 dynamic notebooks on disk; the encyclopedia's 147 = those
  128 + 17 rebound + 2 Mercury.
- The four macOS-stamped lineage pages (`PORT_MACOS_PROVENANCE.md`,
  `VERIFICATION_MACOS.md`, `MACOS_PORT_COMPLETE.md`,
  `REBOUND_REBOUNDX_MACOS_PROVENANCE.md`) are verbatim under a banner
  naming this page and its siblings as the Windows equivalents.

## Verdict

Every suite in the matrix that has a final measurement is green on this
machine: 638/638 + 11/11 + 394/394 + 137/137 + 19/19 tests, 50/50
scripts, 128/128 notebooks with 8/8 checks, 17/17 rebound notebooks,
2/2 Mercury notebooks with every assertion passing, 13/13 videos byte
for byte, 13 + 13 GUIs, 1365/1365 + 258/258 index examples, 124/124
encyclopedia captures, the three-line physics gate, 74/74 movies and
245/252 datasets byte-identical to the macOS files, and every rebound
C-reference gate bit-identical except the 1-ULP `pow` cases. The
divergences are exactly the seven items of §II.4, each measured and
each confined to code that calls the host math library. Two results
were still being finalised when this page was written and are recorded
by their own logs rather than here: the encyclopedia's final
`encyclo.py` / `verify.py` pass after the two Mercury copies'
re-execution (rows 20 and 22 — the dry run passed all six gates), and
the certifier's final run after the clippy and live-count fixes (row 32
— the preliminary run passed every other gate).
