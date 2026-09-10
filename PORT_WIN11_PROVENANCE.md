# PORT_WIN11_PROVENANCE — the Windows 11 ports, and the evidence they changed no physics

This page has two parts. **Part I** is the complete record of the
first Windows 11 port — the Linux release
`rustSolveIt_Using_SUNDIALS_7_8_0/version-7.8.0` -> Windows 11,
2026-08-22 — kept exactly as written that day, measurements included.
**Part II** is the record of the second port — the macOS / Apple
Silicon project `once-ere/rustSolveIt_macos-silicon_SUNDIALS_7_8_0`
merged into this repository, adapted, and re-verified from scratch on
Windows 11, 2026-09-09. The repository is
https://github.com/once-ere/rustSolveIt_Win11_SUNDIALS_7_8_0.git. Its
lineage: ported from `once-ere/rustSolveIt_macos-silicon_SUNDIALS_7_8_0`
(macOS / Apple Silicon), itself ported from
`once-ere/rustSolveIt_Using_SUNDIALS_7_8_0` (Linux); the vendored engine
is `once-ere/SUNDIALS_7_8_Rust_port_for_Windows11`, which differs from
the Linux/macOS engine only in backend module sets (public API
identical; `diff -rq --exclude=.git --exclude=target --exclude=logs`
against a fresh clone of it is silent). Where the two parts state
different counts (622 vs 638 tests, 109 vs 128 notebooks, 1177 vs 1365
index examples, 19 vs 38 SolveIt scripts), Part II's are the current
ones.

# Part I — the first port: rustSolveIt_Using_SUNDIALS_7_8_0 (Linux) -> Windows 11 (2026-08-22)

This document records, completely and self-containedly, how
**rustSolveIt_Win11_SUNDIALS_7_8_0** was produced, on what machine, with
which compiler, what was changed, and the **exact, complete commands**
that build, test, verify, execute and display every example in this
repository. Every command below was actually run on this machine on
2026-08-22 and its result is stated next to it. You do not need any
other document to reproduce any result on this page.

## 0. The machine and the toolchain

| item | value (measured, not assumed) |
|---|---|
| computer | Intel(R) Core(TM) Ultra 9 275HX (2.70 GHz), 192 GB RAM |
| OS | Windows 11 Pro for Workstations, version 25H2, build 10.0.26200 |
| Rust | rustc 1.91.1 / cargo 1.91.1, default host `x86_64-pc-windows-msvc` |
| linker | the Microsoft linker from the installed Visual Studio 2026 MSVC toolset, located automatically by rustc — no developer prompt, no manual configuration |
| Python | 3.14.5 (default text encoding **cp1252** — the source of a whole class of porting fixes, §3) |
| Package tool | `uv` (only for the optional Jupyter virtual environment; the Rust build touches no network) |
| LaTeX | MiKTeX-pdfTeX 4.18 (MiKTeX 24.1) |

**Which compiler, and why.** This machine carries Visual Studio
Professional/Enterprise/Build Tools 2026, Visual Studio 2022, and the
Intel oneAPI compilers. The project is **pure Rust with zero C/C++/
Fortran code and zero FFI**, so no C compiler compiles a single line of
it; the only piece of Visual Studio used is its **linker**, which
`rustc`'s `x86_64-pc-windows-msvc` target locates itself (it picks the
newest installed MSVC toolset — here Visual Studio 2026). The Intel
oneAPI compilers and every WSL2 compiler were **not used at all**.
Everything below ran natively on Windows — nothing in this port was
built, run, or verified under WSL2.

One hardware requirement is inherited from the engine: the CPU must
have the FMA instruction (Intel 2013+, AMD 2012+ — this machine does).
`.cargo/config.toml` at the repository root pins
`-C target-feature=+fma` because the engine's deterministic math
library uses `f64::mul_add` exactly where glibc's FMA build fuses, and
without the flag the MSVC target's SSE2 baseline would turn each fused
operation into a call into `ucrtbase.dll`.

## 1. What this repository was made from

Two inputs, both already present on this machine:

1. **`rustSolveIt_Using_SUNDIALS_7_8_0/version-7.8.0`** — the Linux
   release of the pure-Rust physics simulator (the notebook language,
   the `physical_object` library, the quantum and special-function
   crates, the scene window, the GUI servers, the recorder, the 109
   Jupyter notebooks, the documentation). Everything except its
   `sundials_rs/` was copied verbatim as the starting point.
2. **`SUNDIALS_7_8_Rust_port_for_Windows11`** — the Windows 11 port of
   the pure-Rust SUNDIALS 7.8.0 translation, verified on this exact
   machine (its gate: 153 byte-IDENTICAL / 26 documented-divergent /
   20 excluded example variants against the upstream references, the
   Linux port's numbers on exactly its variants). Its crate tree,
   examples, docs and tools were vendored as this repository's
   `sundials_rs/` (excluding its `.git/`, `target/`, and the 1.35 GB
   of oracle corpora under `logs/`).

The two sundials trees differ only in backend module sets: the Windows
port adds `sundials_libm/` (pure-Rust translations of the glibc
`exp`, `log`, `expm1`, `log1p`, `sin`, `cos`, `atan`, `asin`, `acos`,
`sinh`, `cosh`, `acosh`, each measured 0 mismatches over millions of
inputs against a real glibc oracle) and `nvector_openmp`, and drops the
Linux tree's KLU/sparse-LU stubs. The public solver API is identical —
the first-party code compiled against it **unchanged, warning-free, on
the first build**.

## 2. What was changed (the complete list)

**Rust sources — 1 file.**

- `posim/src/qm3.rs` (tests only): the two QM3 animation tests embed a
  temp-directory path inside a posim string literal; posim string
  literals process `\` escapes, so a Windows `C:\Users\...` path lost
  its separators. The tests now embed the path with forward slashes
  (Windows accepts them). No non-test Rust line was touched anywhere.

**Build configuration — 2 files.**

- `.cargo/config.toml` (new): pins `-C target-feature=+fma` (see §0).
- `.gitattributes` (new): `* -text` — git end-of-line conversion is off
  repository-wide, because recorded videos, evidence logs and reference
  outputs are byte-compared by the harnesses.

**Python tooling — the `posim.exe` name.** Windows builds the binary as
`posim.exe`; every finder now probes it before the extensionless name:
the 13 `gui/*/server.py`, `jupyter/posim_kernel/kernel.py`,
`jupyter/test_protocol.py`, `recorder/src/record_video.py`,
`tools/verify_index_examples.py`, `tools/extract_transcripts.py`, and
the notebook driver template in `notebooks/_build/nbtext.py`.

**Python tooling — UTF-8 and line endings.** Python's default text
encoding on this host is cp1252 and text-mode writes translate `\n` to
`\r\n`; both silently corrupt UTF-8 output and break byte-identity. All
first-party tooling now passes `encoding="utf-8"` on every subprocess
pipe that speaks to posim and on every repository file read/write, and
`newline="\n"` on every byte-compared artifact (recorded videos,
notebook JSON, spec JSON, the Tier-B snippet file).

**Windows-only test fixes — 2 files.**

- `recorder/tests/test_units.py`: restore the working directory before a
  `TemporaryDirectory` cleans up (a cwd inside it is WinError 32).
- `jupyter/test_kernel.py`: match each shell reply to its execute
  request by parent `msg_id` (a startup race left a stale reply queued,
  shifting every subsequent status by one).

**Notebook machinery paths — 2 files + 110 specs.**

- `notebooks/_build/regen.py` emits repo-root-relative POSIX
  `pairs_with` paths (this repository IS the workspace; the donor's
  `version-7.8.0/` nesting is gone), `notebooks/_build/nbcheck.py`
  resolves them against the correct root, and all 110 spec files were
  cleansed of the stale prefix.

**New tools — 3 files.**

- `tools/posim_notebook.cmd` — the cmd.exe twin of the bash
  `tools/posim_notebook` dynamic-notebook launcher (CRLF-encoded;
  cmd.exe mis-parses LF batch files).
- `tools/win_verify_physics.sh` — the Windows physics byte-identity
  gate (§4 below).
- `tools/win_gui_smoke.py` — drives all 13 GUI servers over their
  documented HTTP API (§7 below).

**Documentation.** `README.md`, `CLAUDE.md`, `ARCHITECTURE.md`,
`SolveIt.md`/`.tex`, `grammar.md`/`.tex`, `jupyter/README.md`,
`dynamic_notebooks/README.md` and the notebook boilerplate
(`notebooks/_build/nbtext.py`) were rewritten for Windows 11 —
PowerShell command forms, the `.exe` binary name, the new repository
URL, the vendored Windows engine, and the measured 622-test gate — and
`grammar.pdf` + `SolveIt.pdf` were recompiled from the updated `.tex`.

**Everything else** — every physics formula, every solver call, every
tolerance, every heuristic, the entire vendored engine — is byte-for-
byte what the donors shipped.

## 3. Build and test — commands and results

Run everything from the repository root
`C:\work\rustSolveIt_Win11_SUNDIALS_7_8_0`.
PowerShell forms are given; each was run exactly as written.

```powershell
cargo build --workspace --all-targets 2>&1 | Tee-Object logs\build.log
```
Result: **zero errors, zero warnings**, all 12 crates (7 vendored
sundials + spec_math + physical_object, posim, quantum,
special_functions).

```powershell
cargo build --release --workspace --all-targets 2>&1 | Tee-Object logs\build_release.log
```
Result: clean; produces `target\release\posim.exe` and the six example
binaries under `target\release\examples\`.

```powershell
cargo test --workspace 2>&1 | Tee-Object logs\test.log
```
Result: **622 passed, 0 failed** — 49 physical_object lib + 19
collision + 9 conservation + 42 constrained/equilibrium/sensitivity +
112 posim + 92 quantum + 233 special_functions + 11 vendored
identities + 5 quantum doctests + 50 special_functions doctests.
(One `[ERROR]...[KINSol]` line appears on stderr mid-run; it is printed
*by a passing test* that exercises a solver failure path.)

## 4. The physics byte-identity gate — commands and results

The Linux release recorded its physics outputs in
`evidence/port-7.8.0/`. The Windows gate re-runs the same three suites
in the same concatenated-log formats and diffs:

```bash
# Git Bash, from the repository root
bash tools/win_verify_physics.sh 2>&1 | tee logs/win_verify_physics.log
```

Result (exit 0):

| suite | verdict |
|---|---|
| 12 collision scripts | **IDENTICAL** — byte-for-byte |
| 6 self-checking examples | identical except **one blank line**: the donor added a leading `\n` to `outer_solar_system`'s first `println!` *after* recording its evidence, so the donor's own current source cannot reproduce its evidence on Linux either; every number is identical |
| 59 dynamic notebooks | 57 **IDENTICAL**; the two quantum notebooks (`double_slit`, `tunneling`) differ on 9 lines, each in the **last printed digit** (e.g. norm drift `6.393e-13` vs `6.375e-13`) — the `quantum` crate calls the host math library (UCRT here, glibc there); the classical-physics engine routes through the vendored deterministic `sundials_libm` and does not drift |

Both accepted divergences are pinned byte-for-byte in
`evidence/win11/accepted-divergences-examples.diff` and
`evidence/win11/accepted-divergences-dynamic.diff`; the gate fails on
anything beyond them. Both diverging notebooks were re-run twice and
are **deterministic run-to-run** on this machine.

Regression anchors reproduced exactly (from `logs\example_*.log`):
Pluto at t = 500,000 days after 12,581 internal steps with energy
drift `7.835809e-07`; Kepler e = 0.6 with `|dA|/|A| = 1.131858e-07`;
tumbling body `|dL|/|L| = 0.000000e+00`.

## 5. Running every example (the complete commands)

The six self-checking physics examples (each prints SUCCESS/FAILURE and
exits nonzero on failure) — all six printed **SUCCESS**:

```powershell
cargo run -p physical_object --release --example kepler_orbit
cargo run -p physical_object --release --example outer_solar_system
cargo run -p physical_object --release --example tumbling_body
cargo run -p physical_object --release --example charged_in_b_field
cargo run -p physical_object --release --example newtons_cradle
cargo run -p physical_object --release --example bouncing_ball_restitution
```

The twelve collision scripts — all exit 0 (headless form shown; drop
the first line to let `SCENE CREATE` scripts open a browser window):

```powershell
$env:POSIM_NO_BROWSER = "1"
cargo run -p posim --release -- --script scripts/collisions/01_head_on_exchange.posim
cargo run -p posim --release -- --script scripts/collisions/02_unequal_masses.posim
cargo run -p posim --release -- --script scripts/collisions/03_restitution_ladder.posim
cargo run -p posim --release -- --script scripts/collisions/04_newtons_cradle.posim
cargo run -p posim --release -- --script scripts/collisions/05_billiard_break.posim
cargo run -p posim --release -- --script scripts/collisions/06_spin_up.posim
cargo run -p posim --release -- --script scripts/collisions/07_thin_wall_toi.posim
cargo run -p posim --release -- --script scripts/collisions/08_colliding_binary.posim
cargo run -p posim --release -- --script scripts/collisions/09_spinning_target.posim
cargo run -p posim --release -- --script scripts/collisions/10_billiard_box.posim
cargo run -p posim --release -- --script scripts/collisions/11_box_of_shapes.posim
cargo run -p posim --release -- --script scripts/collisions/12_two_dumbbells.posim
```

The nineteen SolveIt worked-example scripts — all exit 0:

```powershell
$env:POSIM_NO_BROWSER = "1"
cargo run -p posim --release -- --script scripts/solveit/01_elastic_head_on.posim
cargo run -p posim --release -- --script scripts/solveit/02_keplers_third_law.posim
cargo run -p posim --release -- --script scripts/solveit/03_three_conics.posim
cargo run -p posim --release -- --script scripts/solveit/04_restitution_ladder.posim
cargo run -p posim --release -- --script scripts/solveit/05_cyclotron_bdf.posim
cargo run -p posim --release -- --script scripts/solveit/06_symplectic_vs_adaptive.posim
cargo run -p posim --release -- --script scripts/solveit/07_dzhanibekov.posim
cargo run -p posim --release -- --script scripts/solveit/08_magnetic_torque.posim
cargo run -p posim --release -- --script scripts/solveit/09_newtons_cradle.posim
cargo run -p posim --release -- --script scripts/solveit/10_no_tunnelling.posim
cargo run -p posim --release -- --script scripts/solveit/11_lagrange_l4.posim
cargo run -p posim --release -- --script scripts/solveit/12_dumbbell_inertia.posim
cargo run -p posim --release -- --script scripts/solveit/13_tilted_torus.posim
cargo run -p posim --release -- --script scripts/solveit/14_particle_in_a_box.posim
cargo run -p posim --release -- --script scripts/solveit/15_tunnelling.posim
cargo run -p posim --release -- --script scripts/solveit/16_special_functions.posim
cargo run -p posim --release -- --script scripts/solveit/17_pendulum_dae.posim
cargo run -p posim --release -- --script scripts/solveit/18_equilibrium_and_sensitivity.posim
cargo run -p posim --release -- --script scripts/solveit/19_hinged_door.posim
```

The fifty-nine dynamic notebooks run the same way
(`cargo run -p posim --release -- --script dynamic_notebooks/<name>.posim`
for the 59 files of `dynamic_notebooks\*.posim`), and interactively —
loading the file, opening its live scene window in your browser, and
leaving you at the `In[]:=` prompt — with:

```powershell
tools\posim_notebook.cmd kepler_orbit
```

(any bare name from `tools\posim_notebook.cmd --list`; press **Start**
in the window that opens, or type `scene start`).

## 6. The Jupyter notebooks — commands and results

Every example above has exactly one Jupyter notebook in `notebooks/`
(109 in all: 13 video scenes + 6 compiled Rust examples + 12 collision
scripts + 19 SolveIt scripts + 59 dynamic notebooks), each a
stand-alone Python notebook that starts `posim --machine` and drives it
over JSON Lines. The full regenerate–execute–check loop, as run:

```powershell
$env:POSIM_NO_BROWSER = "1"
python notebooks\_build\regen.py
```
Result: all 110 specs rebuilt, all 109 notebooks rendered.

```bash
# Git Bash (the * glob); from PowerShell list the files explicitly
POSIM_NO_BROWSER=1 python notebooks/_build/nbrun.py notebooks/*.ipynb
```
Result: **109 ok, 0 failed** — every code cell of every notebook
executed against the live simulator, outputs written back.

```bash
python notebooks/_build/nbcheck.py notebooks/*.ipynb
```
Result: **109/109 notebooks pass all seven requirements** (launch
instructions, no cross-references, explanation before every code cell,
naming, save dialog, physics derivation sections, valid nbformat-4
pairing with a real example file).

To open them yourself:

```powershell
python -m pip install --user jupyterlab
jupyter lab notebooks/
```

(Footnote added in the Windows 11 edition, the record above kept as
written: after a `--user` install the `jupyter` launcher is usually not
on PATH, so `python -m jupyterlab notebooks\` is the form that works
everywhere.)

The wrapper-kernel path (JupyterLab driving the simulator in its own
language) and the wire protocol were verified with:

```powershell
cargo build --release
python jupyter\test_protocol.py
uv venv jupyter\.venv
uv pip install -p jupyter\.venv\Scripts\python.exe ipykernel jupyter_client
$env:POSIM_NO_BROWSER = "1"
jupyter\.venv\Scripts\python.exe jupyter\test_kernel.py
```
Results: “all protocol checks passed”; all 7 kernel cells **ok** —
“JupyterLab can drive this kernel”.

## 7. The browser displays — commands and results

**Live GUI pages** (one per recorded scene; stdlib-Python server owning
a `posim --machine` child + vanilla-JS canvas page):

```powershell
cargo build --release -p posim
python gui\kepler_ellipse\server.py     # then open http://127.0.0.1:8906/
```

Each of the 13 servers has its fixed port (piston_crankshaft 8895,
rack_and_pinion 8896, gyroscope_gimbal 8897, cardan_compass 8898,
universal_joint 8899, spinning_top 8900, ball_joint_chain 8901,
cardan_gear 8902, rod_pendulum_chain 8903, double_pendulum_hinges 8904,
tumbling_racket 8905, kepler_ellipse 8906, box_of_shapes 8907). The
automated pass over all thirteen:

```powershell
python tools\win_gui_smoke.py
```
Result: **13 of 13 GUIs pass** — page served with its canvas,
`/api/state` live, Start advances simulated time (a real engine
stepping), Stop and Reset return bit-exactly to t = 0.

**The live scene window** from any notebook session:

```powershell
cargo run
```
then type `new sphere { mass = 1, radius = 0.5 }` and `scene create` —
a browser window opens (via `cmd /C start`) with Start/Pause/Reverse/
Reset, orbit/zoom gestures and a live conserved-quantities readout.

**Recorded browser videos** (open offline, no server):

```powershell
start videos\kepler_ellipse.html
```

(likewise `tumbling_racket`, `box_of_shapes`, `double_pendulum_hinges`,
`universal_joint`, `ball_joint_chain`, `rod_pendulum_chain`,
`spinning_top`, `gyroscope_gimbal`, `cardan_compass`, `cardan_gear`,
`rack_and_pinion`, `piston_crankshaft`). All 13 were **re-recorded on
this machine** and verified:

```powershell
python recorder\src\record_all.py            # re-record all 13
python recorder\src\record_all.py --check    # byte-compare against committed
python recorder\tests\test_units.py          # 17 tests OK
python recorder\tests\test_end_to_end.py     # 9 tests OK
```
Result: “all 13 recordings reproduce byte for byte”. Before adopting
the Windows-written files, one recording was diffed against the
committed Linux original end-of-line-insensitively and found
**identical modulo CRLF** — the Windows recorder reproduces the Linux
frames exactly.

**The index of every named entity** (6,309 entries, with runnable
examples):

```powershell
start index_of_entities.html
python tools\verify_index_examples.py     # PASS 1177/1177 (100.0%)
python tools\verify_tierb_examples.py     # COMPILED 258/258 (100.0%)
```

## 8. The documents — commands and results

```powershell
pdflatex -interaction=nonstopmode grammar.tex
pdflatex -interaction=nonstopmode grammar.tex
pdflatex -interaction=nonstopmode SolveIt.tex
pdflatex -interaction=nonstopmode SolveIt.tex
```
(Each file twice — the table of contents needs the second pass.)
Result: `grammar.pdf` and `SolveIt.pdf` rebuilt without errors.
`grammar.md`/`grammar.pdf` document the full command language — lexer,
EBNF, type system, every command, the stack machine, the engine, the
browser videos — with **18 fully worked examples**; `SolveIt.md`/
`SolveIt.pdf` is the complete first-time-reader solution guide with
**16 more fully documented worked examples** (scripts 01–16 above; 17–19
extend the set). Both `.md` files are the source of truth for their
`.tex` twins.

## 9. What is and is not claimed

- **Claimed**: on this machine, this repository builds warning-free,
  passes 622/622 tests, runs all 6 + 12 + 19 + 59 examples and all 109
  notebooks green, serves all 13 GUIs, reproduces all 13 videos byte-
  for-byte, and reproduces the Linux physics evidence byte-for-byte
  except the two pinned divergences of §4 — one a donor-side stale
  blank line, one a last-digit host-libm effect confined to the two
  quantum notebooks.
- **Not claimed**: byte-identity of the *quantum* outputs across
  platforms (the `quantum` crate uses the host math library by design —
  its tests are analytic, and all 92 pass), or any verification result
  on any platform other than Windows 11 x86-64 as configured in §0.

---

# Part II — the second port: rustSolveIt_macos-silicon_SUNDIALS_7_8_0 -> Windows 11 (2026-09-09)

Part I above is the record of the first Windows port (the Linux
release -> Windows 11, 2026-08-22) and is kept exactly as written; its
numbers (622 tests, 109 notebooks, 1177 index examples, 19 SolveIt
scripts) are the numbers of that day. This Part II records the second
port: the macOS / Apple Silicon project
**once-ere/rustSolveIt_macos-silicon_SUNDIALS_7_8_0** — itself a port
of the same Linux release, grown by 31 commits into a much larger
project — was merged into this Windows 11 repository, adapted, and
re-verified from scratch on Windows 11 on 2026-09-09. Every command
below is complete as written, was actually run on this machine on that
day, and its measured result is stated next to it. Where a result is
the same as a macOS or Linux number, that is stated; where it differs,
both values are given and the cause is named. Run everything from the
repository root (the folder that contains `Cargo.toml`, `posim\` and
`sundials_rs\`; for example `C:\work\rustSolveIt_Win11_SUNDIALS_7_8_0`),
in PowerShell unless a block is marked Git Bash.

## II.0 The machine and the toolchain

| item | value (measured on this machine, not assumed) |
|---|---|
| computer | Intel Core Ultra 9 275HX (`python -c "import platform; print(platform.processor())"` prints `Intel64 Family 6 Model 198 Stepping 2`) |
| OS | Windows 11 Pro for Workstations, build 10.0.26200 |
| Rust | rustc 1.91.1 (ed61e7d7e 2025-11-07) / cargo 1.91.1, default host `x86_64-pc-windows-msvc` |
| FMA | `.cargo\config.toml` at the repository root pins `-C target-feature=+fma` (required by the Windows engine's deterministic math library; the CPU must have FMA — this one does) |
| MSVC toolset | Visual Studio 2026 Build Tools, MSVC 14.51.36231: `C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Tools\MSVC\14.51.36231\bin\Hostx64\x64\cl.exe`, located by rustc (the linker) and by the `cc` crate (the one C compile, below) automatically — no developer prompt, no `vcvars`, no manual configuration |
| also installed, NOT used | Visual Studio 2026 Professional/Enterprise, Visual Studio 2022 Professional/Enterprise, Intel oneAPI 2025.3 / 2026.1 |
| WSL2 | no WSL2 compiler or linker was used anywhere — everything ran natively on Windows |
| Python | 3.14.5 (default text encoding **cp1252**); nbformat 5.10.4, nbclient 0.10.2, jupyter_client 8.6.3, ipykernel 7.1.0 |
| uv | 0.9.10 (only for the optional `jupyter\.venv`; the Mercury notebook venv is made with `python -m venv`) |
| LaTeX | MiKTeX-pdfTeX 4.18 (MiKTeX 24.1) |
| Chrome | `C:\Program Files\Google\Chrome\Application\chrome.exe` (headless GUI capture and the encyclopedia PDF) |
| C toolchain for the rebound C-reference gates only | the same Build Tools through `vcvars64.bat`: `cl` banner `Microsoft (R) C/C++ Optimizing Compiler Version 19.51.36256 for x64`, `link` 14.51.36256.0; GNU Make 3.81 (GnuWin32) |

**Which compiler compiles what.** The simulator workspace (`posim`,
`physical_object`, `quantum`, `special_functions`, the vendored
`sundials_rs` engine) is pure Rust with zero C, zero FFI and zero
crates.io dependencies; the only piece of Visual Studio it uses is the
**linker**, which rustc's `x86_64-pc-windows-msvc` target finds by
itself. Exactly **one** C compile happens anywhere in the simulator
build: the SQLite dataset sidecar in `dataset_tools\` (a separate
package, excluded from the workspace) depends on `rusqlite` with the
*bundled* SQLite amalgamation, and the `cc` crate compiles that
amalgamation with the MSVC `cl.exe` above, which it locates through
vswhere on its own. The rebound C-reference gates (§II.8.12) compile
the upstream REBOUND / REBOUNDx C sources and their harnesses with the
same `cl.exe`, explicitly, through `vcvars64.bat`. No other C compiler
— not Intel oneAPI, not anything under WSL2 — touched anything.

## II.1 What this repository was made from

Three inputs, all present on this machine:

1. **`rustSolveIt_Win11_SUNDIALS_7_8_0`** — this repository as Part I
   left it (`origin/main` at `51f570f` before this work): the Linux
   release ported to Windows 11 with the Windows engine vendored.
2. **`rustSolveIt_macos-silicon_SUNDIALS_7_8_0`** — the macOS / Apple
   Silicon project (`once-ere/rustSolveIt_macos-silicon_SUNDIALS_7_8_0`,
   HEAD `5360157` = `origin/main`, 31 commits, 5054 tracked files),
   received as an unzipped workspace. Only the nested repository is the
   deliverable; the sibling reference trees in that workspace (the
   Apple Silicon engine, the Linux release, an outer copy of
   `planet_Mercury` with its 267 MB `data\`, an outer
   `SolveIt_Notebooks_for_rust`, prompt text files, loose Mercury
   html/ipynb files) are workspace scratch — the outer `planet_Mercury`
   differs from the nested one only in Cargo path dependencies, the
   loose html/ipynb are earlier editions of the nested files, and the
   prompt files were never part of the macOS repository.
3. **`SUNDIALS_7_8_Rust_port_for_Windows11`** — the Windows engine.
   This repository's `sundials_rs\` is its crate tree (verified before
   the merge: `diff -rq --exclude=.git --exclude=target --exclude=logs`
   against a fresh clone of it is silent).

**What the macOS project added** on top of the Linux release (the
content this port carries over):

- **The SQLite dataset layer**: `dataset_tools\` (the `solveit_db`
  sidecar — `rusqlite` + `r2d2` + `r2d2_sqlite`, WAL journal mode, a
  connection pool, a busy timeout) and the `DATASET` / `MOVIE` / `SQL`
  command families in posim, which drive the sidecar as a child
  process over JSON Lines so the simulator itself never links SQLite.
  Every example and every notebook now records its run into a dataset
  under `datasets\` and derives a browser movie under `movies\`.
- **SolveIt worked examples 20–38** (`scripts\solveit\20_*` … `38_*`):
  sixteen dataset / movie / SQL examples (20–35), the teacher-diff
  example (36), and the curvilinear-coordinate examples (37–38: fourteen
  coordinate systems and the `DIFF` / `SUM` / `MAP` trajectory algebra
  in `special_functions\src\coords.rs`).
- **128 Jupyter notebooks** in `notebooks\` (109 -> 128; every notebook
  now owns a SQLite dataset and has a simulator-independent `sqlite3`
  read-back cell), with eight audit requirements instead of seven.
- **The notebook encyclopedia** `SolveIt_Notebooks_for_rust\`: 147
  entries (the 128 notebooks + the 17 rebound notebooks + the 2 Mercury
  notebooks), each a copy with a header cell and a companion
  `player_<tag>.py`, one captured GUI image per browser-family entry,
  and the built `ENCYCLOPEDIA.md` / `.html` / `.pdf` with six
  mechanical verification gates.
- **`planet_Mercury\`**: the Mercury 3:2 spin-orbit capture project —
  `mercury_rs` (pure-Rust CVODE compute crate), `mercury_crosscheck`
  (independent REBOUNDx `tides_spin` cross-check), two executed
  notebooks (`mercury_tidal_locking`, `mercury_test2_jupiter_gr` —
  Jupiter + general relativity) and two baked browser pages.
- **The rebound ports' macOS layer** on `rebound_rust\` /
  `reboundx_rust\` (79 files): macOS-edition documentation, porttest
  dumps from macOS runs, a macOS `rand_r` shim, a POSIX integrator-matrix
  sweep, rustdoc backtick fixes in 7 `.rs` files (11 doc-comment lines;
  10 `.rs` files were resolved "theirs" at the merge, the other three
  carrying no doc-text change),
  a platform-neutral notebook generator, 17 executed notebooks including
  the new `tides_spin_pseudo`.
- **The entity index** grown to 1365 runnable examples (the `DATASET` /
  `SQL` / `MOVIE` families cataloged), and the documentation for all of
  the above.

## II.2 Lineage facts established by diffing (before anything was merged)

- The macOS repository's root commit `d94488f` is a **pristine import**
  of the Linux release `rustSolveIt_Using_SUNDIALS_7_8_0/version-7.8.0`.
  This repository's root commit `98501f4` is that same tree with
  `sundials_rs\` swapped for the Windows engine and `.cargo\config.toml`
  (`+fma`) added: 1840 identical blobs; the only non-engine differences
  are `.cargo\config.toml` and `Cargo.lock`.
- The macOS `sundials_rs\` is byte-identical (crates) to the Linux
  engine; the Windows `sundials_rs\` differs from both only in backend
  module sets (Windows adds `sundials_libm\` and `nvector_openmp`, drops
  the KLU/sparse-LU stubs). The public solver API is identical.
- The macOS repository's `rebound_rust` / `reboundx_rust` import
  (commit `3a52eab`) is byte-identical modulo CR bytes to this
  repository's (`once-ere/rebound_rust` @ `ec01f60`): 190/190 files.
- 34 `dynamic_notebooks\routh_*.posim` were changed by both ports on
  the same LAUNCH comment line (Windows: "run from the repository
  root"; macOS: a placeholder path).
- The 17 `evidence\port-7.8.0\*.log` differ between the two only by the
  username-placeholder spelling, and were CRLF in the Linux donor
  already.

## II.3 How the merge was done

The problem: both repositories descend from the same Linux tree, but
git did not know it (two unrelated root commits), and this repository's
text files were CRLF (checked out that way in Part I with `* -text`)
while the macOS files were LF — so a naive merge would have conflicted
on thousands of byte-identical files. The method, in the order it ran:

1. **Safety branch.** `pre-macos-port` was created at `51f570f`, the
   last commit of Part I. It still exists; `git show pre-macos-port:<path>`
   shows any file exactly as the first Windows port left it.
2. **Line-ending normalization, commit `f1f598d`.** 219 tracked text
   files outside `sundials_rs\`, `logs\`, `evidence\` and `*.cmd` were
   converted CRLF -> LF (cmd.exe needs CRLF batch files; the vendored
   engine and the run records were left alone) so that byte-identical
   content would merge silently.
3. **The graft.** The macOS repository was fetched as remote `macos`.
   `git replace --graft 98501f4 d94488f` told git, temporarily, that
   this repository's root commit has the pristine Linux import as its
   parent — so the two histories now share a common ancestor and git
   can do an ordinary **three-way merge** (base = the Linux release,
   ours = Windows, theirs = macOS) instead of treating every file as
   added on both sides. Then
   `git merge --no-ff --no-commit -X no-renames macos/main`, which
   left **796 conflicts**.
4. **Resolution by category, commit `d6f6605`.** Every conflict fell
   into one of these classes and was resolved by the rule for its class:

   | files | rule |
   |---|---|
   | `sundials_rs\` — 539 modify/delete + 3 docs | **ours** (the Windows engine stays; §II.5) |
   | `notebooks\*.ipynb` (109), `notebooks\_build\specs` (34), `index_data` (2) | **theirs**, then regenerated on this machine (§II.8.5, §II.8.11) |
   | `notebooks\_build\nbtext.py`, `regen.py` | Windows text kept, macOS machinery merged by hand |
   | `dynamic_notebooks\routh_*.posim` (34) | **ours** (the Windows LAUNCH line) |
   | `evidence\port-7.8.0\*.log` (17) | the LF/macOS copies (identical modulo CR and the placeholder) |
   | `videos\rack_and_pinion.html` | **ours** (the Windows recording; reproduces byte for byte here, §II.8.8) |
   | rebound: `rebound_rust.md`/`.tex`/`.pdf`, both READMEs, three acceptance write-ups (8 doc files), the five Windows porttest dumps | **ours** (§II.5) |
   | rebound: the 10 doc-only `.rs` (7 of them with doc-text changes, 11 lines in all — §II.5), `.gitignore`, `make_notebooks.py`, the 17 notebooks; 3 C harness files (identical modulo CR) | **theirs** |
   | `README.md`, `CLAUDE.md`, `ARCHITECTURE.md`, `SolveIt.md`, `grammar.md` | **theirs**, then re-platformed to Windows 11 in place |
   | root `Cargo.toml` | union of the two `exclude` lists (adds `planet_Mercury`, `dataset_tools`) |

   The graft ref was deleted after the merge (`git replace -d 98501f4`);
   the merge commit records both parents (`f1f598d` and `5360157`), so
   `git log` shows the complete macOS history inside this repository.

The command sequence, for the record (it needs the macOS repository
reachable as a remote; the macOS project's public URL is given):

```powershell
git branch pre-macos-port 51f570f
# commit f1f598d: CRLF -> LF on the 219 files named above
git remote add macos https://github.com/once-ere/rustSolveIt_macos-silicon_SUNDIALS_7_8_0.git
git fetch macos
git replace --graft 98501f4 d94488f
git merge --no-ff --no-commit -X no-renames macos/main    # 796 conflicts
# ... resolve by the table above, git add, then:
git commit                                                  # d6f6605
git replace -d 98501f4
```

Later commits of the port: `f8557de` (the Windows adaptation, §II.4),
`03b3986` (all 128 notebooks regenerated, executed and checked here;
the 17 rebound notebooks re-executed; datasets and movies regenerated),
`6d99370` (the encyclopedia copies regenerated with Windows headers and
executed here; lineage banners on the four macOS provenance pages),
`a5082b2` (Windows 11 editions of the guides; clippy-clean under rustc
1.91.1; the certifier's live-count rule honoured; the 16 macOS harness
binaries untracked), `23e90f3` (the rebound C-reference gates measured
on Windows 11 with MSVC; all 17 notebooks re-executed against the dumps).

## II.4 What was changed for Windows (the complete list)

Pre-modification originals of every edited file were saved under
`.backups\2026-09-09\` (gitignored); the macOS originals also remain in
git history (`git show macos/main:<path>` or `git show 5360157:<path>`).

**Rust sources — 1 file, 1 function.**

- `posim\src\dataset.rs`: `find_db_tool` (the sidecar finder) now
  probes `dataset_tools\target\release\solveit_db.exe` **before** the
  extensionless `solveit_db` (`DB_TOOL_NAMES`, `cfg!(windows)`), after
  `$env:POSIM_DB_TOOL` and before `PATH`. Without it every `DATASET`
  command on Windows reported the sidecar "not built" on a checkout
  where cargo had just built it. No physics line was touched.

**Clippy under rustc 1.91.1 — 7 files, behaviour-preserving.** The
repository's certifier runs `cargo clippy --workspace --all-targets`
under the crates' `#![deny(warnings)]`. clippy 1.91.1 flagged 2 lints in
`special_functions\src\coords.rs` (`nonminimal_bool`;
`too_many_arguments` 8/7), 13 in `physical_object`
(`needless_range_loop` x10 in `integrate.rs` / `sensitivity.rs` /
`equilibrium.rs`, `manual_memcpy` x1 in `sensitivity.rs`,
`too_many_arguments` 9/7 for `constrain.rs` `rack_state`), plus
`manual_memcpy` and `type_complexity` in `posim\src\dataset.rs` and one
line in `physical_object\tests\dae.rs` (`… * -1.0` → `-(…)`) — code that
arrived unchanged from the Linux/macOS lineage, built there with newer
toolchains. Fixed with iterator forms, slice `fill` /
`copy_from_slice` (in `dataset.rs` the `dataset_map` copy loop
`cols[3..13] -> row[4..14]` became `copy_from_slice`), two
`#[allow(clippy::too_many_arguments)]` waivers and one
`#[allow(clippy::type_complexity)]` (commit `a5082b2`: 7 source files,
42 insertions, 40 deletions). Everything was re-run after the LAST of
those edits (the 16:55 `dataset.rs` / `tests\dae.rs` pair): the physics
gate on the final release binary (`logs\win_verify_physics.log`, 18:04
— examples IDENTICAL-MODULO-DOCUMENTED, collision scripts IDENTICAL,
dynamic notebooks IDENTICAL-MODULO-DOCUMENTED, exit 0), the dataset
byte-comparison (`scripts\solveit\20_dataset_kepler_orbit.posim`
re-run on that binary: `datasets\solveit20_kepler.db` byte-identical to
the file the earlier binary wrote), `cargo clippy --workspace
--all-targets` (`logs\clippy_workspace.log`, 16:56, 0 errors) and the
full test suite inside the certifier's own run (`logs\certify_clean.log`,
18:0x: 638 passed, 0 failed). Nothing numerical moved.

**The encyclopedia tooling —
`SolveIt_Notebooks_for_rust\_tools\{gen,run_copy,encyclo,shoot}.py`.**
Engine detection under either edition's folder name
(`rustSolveIt_Win11_SUNDIALS_7_8_0` or
`rustSolveIt_macos-silicon_SUNDIALS_7_8_0` beside the tree) or the
clone root; `posim.exe` / venv `Scripts\python.exe` / Windows Chrome
(Edge fallback) probing; PowerShell header cells and Appendix C;
Windows editorial notes for the three libm notebooks; LF writes; POSIX
paths in headers and the manifest (commit `6d99370`); a verdict parser
tolerant of either path separator.

**The Mercury tooling — `planet_Mercury\notebook\{build_notebook,
build_notebook2,run_notebook}.py`, `planet_Mercury\gui\{bake_page,
bake_page2}.py`.** `mercury_rs.exe` probing, Windows setup prose, LF
writes; the notebooks' driver cell decodes `mercury_rs` output as UTF-8
(Python's cp1252 default otherwise mangles the program's `Ω`/`—`
characters).

**`notebooks\_build\nbdataset.py`.** The sidecar build command in the
notebook prose is in PowerShell-safe form
(`cd dataset_tools; cargo build --release; cd ..`).

**`jupyter\kernelspec\kernel.json`.** `argv` now starts with `python`
(Windows has no `python3` launcher by default).

**`.gitignore`.** Added: `/planet_Mercury/data/` (~270 MB of regenerable
CSV/SQLite), `/planet_Mercury/notebook/.venv/`, the SQLite side files
`*.db-wal` `*.db-shm` `*.sqlite3-wal` `*.sqlite3-shm`, the
encyclopedia's `.ipynb_checkpoints`, and the 16 extensionless
`rebound_rust\porttest\*` / `reboundx_rust\porttest\*` names under which
the macOS repository had committed its arm64 Mach-O harness binaries
(build products, useless on any other platform; untracked in `a5082b2`
and rebuilt from their `.c` sources by the documented commands).

**Lineage banners.** The four macOS-stamped provenance pages —
`PORT_MACOS_PROVENANCE.md`, `VERIFICATION_MACOS.md`,
`MACOS_PORT_COMPLETE.md`, `REBOUND_REBOUNDX_MACOS_PROVENANCE.md` — keep
their text verbatim under a one-paragraph banner saying that every
number on them was measured on the Apple Silicon machine and naming
the Windows equivalents (`PORT_WIN11_PROVENANCE.md`,
`VERIFICATION_WIN11.md`, `WINDOWS_PORT_COMPLETE.md`,
`REBOUND_REBOUNDX_WIN11_PROVENANCE.md`). Every other platform-stamped
document was re-platformed in place to Windows 11 — PowerShell forms,
`posim.exe` / `solveit_db.exe` / `mercury_rs.exe`, `python`, the
Windows-measured numbers — and the `.md` -> `.tex` twins (`SolveIt`,
`grammar`, `scene_info`, `physical_object_simulator`) were kept in sync
and recompiled (§II.8.14).

**The rebound documentation decision.** `rebound_rust\rebound_rust.md`
/ `.tex` / `.pdf`, both READMEs and the three acceptance write-ups stay
the **Windows** edition. The macOS edition had re-platformed the same
Windows story to macOS; here the Windows story is current and the
macOS one is lineage, recorded verbatim by
`REBOUND_REBOUNDX_MACOS_PROVENANCE.md`.

**Everything else** — every physics formula, every solver call, every
tolerance, the entire vendored engine, the macOS project's Rust
sources apart from the one finder function and the clippy rewrites —
is what the donors shipped.

## II.5 Decisions a reader should know

- **The Windows engine stays.** The macOS project's engine was the
  Linux one; it is not carried. Nothing is lost: the two differ only in
  backend module sets, the public API is identical, and the physics
  byte-identity gate (§II.8.3) is the proof that the Windows engine
  reproduces the recorded evidence.
- **Host-libm exception classes.** Three, all confined to first-party
  code that calls the host math library (UCRT here, glibc on Linux,
  Apple libm on macOS) instead of the vendored deterministic
  `sundials_libm`: (1) the two quantum notebooks (`double_slit`,
  `tunneling`) — last printed digit, pinned since Part I; (2) the
  GEAR/RACK joint residual — the `rack_and_pinion` video is therefore
  the Windows recording, and it reproduces byte for byte here;
  (3) **new in this port**: the fourteen curvilinear coordinate maps in
  `special_functions\src\coords.rs` (SolveIt 37–38) — six datasets
  differ from the macOS files in a handful of values at 1e-16..1e-14
  relative, and two transcript lines of examples 37/38 print different
  last digits (quantified in §II.8.6). Everything else — 245/252
  datasets, 74/74 movies, all 50 scripts, the gate — is byte-identical
  to the macOS files.
- **Regenerable data is not committed**: `planet_Mercury\data\`, the
  encyclopedia and Mercury venvs, SQLite `-wal`/`-shm` files. The run
  logs in `logs\` and `rebound_rust\logs\` ARE committed (the precedent
  of Part I), with the build-machine username swept to the standing
  placeholder `youruser`.

## II.6 Windows traps (all of Part I's still apply; two are new)

Part I's list stands: Python's cp1252 default encoding (`encoding="utf-8"`
on every file and pipe), text-mode `\n` -> `\r\n` (`newline="\n"` on
every byte-compared artifact), `posim.exe` probed before the
extensionless name, backslash escapes inside posim string literals (use
forward slashes), a cwd inside a `TemporaryDirectory` blocking its
cleanup, and the kernel test's reply matching by parent `msg_id`. New in
this port: **the sidecar finder probes `solveit_db.exe`** (§II.4), and
**SQLite `-wal`/`-shm` side files are transient** — the sidecar
checkpoints on close; a pair left beside a dataset is not data, is
gitignored, and a `.db` must never be byte-compared while its `-wal`
exists (`DATASET CLOSE` or `solveit_db.exe checkpoint <db>` first).

## II.7 Building — commands and results

```powershell
cargo build --workspace --all-targets 2>&1 | Tee-Object logs\build.log
```
Result: **0 errors, 0 warnings** (`logs\build.log`).

```powershell
cargo build --release --workspace --all-targets 2>&1 | Tee-Object logs\build_release.log
```
Result: **0 warnings** (`logs\build_release.log`);
`target\release\posim.exe` is 5,705,216 bytes.

```powershell
cd dataset_tools; cargo build --release 2>&1 | Tee-Object ..\logs\dataset_tools_build.log; cd ..
```
(Git Bash: `cd dataset_tools && cargo build --release 2>&1 | tee ../logs/dataset_tools_build.log && cd ..`.)
Result: `rusqlite v0.32.1` and `r2d2_sqlite v0.25.0` compiled — the
bundled SQLite amalgamation compiled by MSVC `cl.exe` through the `cc`
crate — `Finished release ... in 18.22s`, **0 warnings**;
`dataset_tools\target\release\solveit_db.exe` is 2,326,016 bytes. The
first build needs network access to crates.io (its own `Cargo.lock` is
committed, so the versions are fixed); the simulator workspace's
`Cargo.lock` still lists exactly the 12 local crates and **zero**
registry sources.

## II.8 Verification — every command and its measured result

### II.8.1 The workspace tests

```powershell
cargo test --workspace 2>&1 | Tee-Object logs\test.log
```
Result: **638 passed, 0 failed** = 49 `physical_object` lib + 19
collision + 9 conservation + 42 constrained/DAE + 121 `posim` + 92
`quantum` + 240 `special_functions` + 11 vendored identities + 5
`quantum` doctests + 50 `special_functions` doctests (`logs\test.log`).
Re-run after the `dataset.rs` finder fix and after the `coords.rs` /
`physical_object\src` clippy rewrites: **638 passed, 0 failed**
(`logs\test_final.log`, 16:53); and once more, after the last 16:55
`dataset.rs` / `tests\dae.rs` clippy edits, by the certifier's own run:
**638 passed, 0 failed** (`logs\certify_clean.log`, §II.8.15). (The 16 tests beyond Part I's 622 are 9 `posim` and 7
`special_functions` tests that arrived with the macOS project.)

### II.8.2 The sidecar

```powershell
cd dataset_tools; cargo test 2>&1 | Tee-Object ..\logs\dataset_tools_test.log; cd ..
```
Result: **8 unit + 3 integration = 11 passed, 0 failed**
(`logs\dataset_tools_test.log`) — including
`ten_threads_through_one_pool_lose_nothing` and
`two_pools_on_one_file_interleave_writes_and_reads`.

```powershell
.\dataset_tools\target\release\solveit_db.exe stress $env:TEMP\stress_demo.db 10 500
```
(Git Bash: `./dataset_tools/target/release/solveit_db.exe stress /tmp/stress_demo.db 10 500`.)
Result: `stress: 10 threads x 500 rows = 5000 expected, 5000 written, journal_mode=wal`
(`logs\crosscheck_stress_launcher.log`).

That posim finds the sidecar (the `dataset.rs` fix) was checked with
the first dataset example, headless:

```powershell
$env:POSIM_NO_BROWSER = "1"
cargo run -p posim --release -- --script scripts/solveit/20_dataset_kepler_orbit.posim
```
Result: runs to completion; `datasets\solveit20_kepler.db`,
`datasets\solveit20_kepler_plots.html` and `movies\solveit20_kepler.html`
written.

### II.8.3 The physics byte-identity gate (Linux evidence)

```bash
# Git Bash, from the repository root
bash tools/win_verify_physics.sh 2>&1 | tee logs/win_verify_physics.log
```
Result (exit 0), the same three verdict lines as Part I:

```
IDENTICAL-MODULO-DOCUMENTED  examples-win11.log vs examples-7.8.0.log (pinned: accepted-divergences-examples.diff)
IDENTICAL  collision-scripts-win11.log == collision-scripts-7.8.0.log
IDENTICAL-MODULO-DOCUMENTED  dynamic-notebooks-win11.log vs dynamic-notebooks-7.8.0.log (pinned: accepted-divergences-dynamic.diff)
```
Nothing beyond the two pinned diffs of Part I §4 appeared — before or
after the `coords.rs` / `physical_object\src` clippy rewrites
(`logs\win_verify_physics.log`, 16:48; the 16:55 `dataset.rs` edit came
later and the gate does not exercise `dataset_map` — see §II.5).

### II.8.4 All 50 scripts, headless

The 38 SolveIt worked-example scripts and the 12 collision scripts:

```powershell
$env:POSIM_NO_BROWSER = "1"
Get-ChildItem scripts\solveit\*.posim, scripts\collisions\*.posim | ForEach-Object {
    cargo run -p posim --release -- --script $_.FullName 2>&1 | Out-File -Encoding utf8 "logs\solveit_scripts\$($_.Directory.Name)_$($_.BaseName).log"
    "$LASTEXITCODE $($_.FullName)"
} | Tee-Object logs\solveit_and_collision_scripts.log
```
(Git Bash:
`for f in scripts/solveit/*.posim scripts/collisions/*.posim; do POSIM_NO_BROWSER=1 cargo run -p posim --release -- --script "$f" > "logs/solveit_scripts/$(basename "$(dirname "$f")")_$(basename "$f" .posim).log" 2>&1; echo "$? $f"; done | tee logs/solveit_and_collision_scripts.log`.)
Result: **50/50 exit 0** (`logs\solveit_and_collision_scripts.log`;
per-script transcripts in `logs\solveit_scripts\`). The two transcript
lines of examples 37–38 that differ from the macOS transcripts are in
§II.8.6.

### II.8.5 The 128 Jupyter notebooks

```powershell
cargo build --release -p posim
cd dataset_tools; cargo build --release; cd ..
python notebooks\_build\regen.py
$env:POSIM_NO_BROWSER = "1"
# PowerShell 7 (pwsh) only — Windows PowerShell 5.1 has no -Parallel; use the sequential loop:
#   Get-ChildItem notebooks\*.ipynb | ForEach-Object { python notebooks\_build\nbrun.py $_.FullName }
Get-ChildItem notebooks\*.ipynb | ForEach-Object -Parallel {
    python notebooks\_build\nbrun.py $_.FullName
} -ThrottleLimit 6
python notebooks\_build\nbcheck.py
```
The Git Bash form actually run (`logs\notebooks_run.log`):

```bash
python notebooks/_build/regen.py
ls notebooks/*.ipynb | POSIM_NO_BROWSER=1 xargs -P 6 -I{} python notebooks/_build/nbrun.py {}
python notebooks/_build/nbcheck.py
```
Result: all 128 regenerated; **128 ok, 0 failed**; **128/128 notebooks
pass all eight requirements** (`logs\notebooks_run.log`).

### II.8.6 Datasets and movies against the macOS files

After all of the above had run (the 50 scripts and the 128 notebooks),
the regenerated `datasets\` and `movies\` were byte-compared with the
files the macOS machine produced. With the macOS repository fetched as
remote `macos` (§II.3):

```powershell
git diff --stat macos/main HEAD -- datasets movies
```
Result: exactly **7 files** differ — `datasets\solveit37_pol.db`,
`solveit37_sph.db`, `solveit37_sph_plots.html`, `solveit38_bis.db`,
`solveit38_cyl.db`, `solveit38_obl.db`, `solveit38_tor.db`; i.e.
**movies 74/74 byte-identical, datasets 245/252 byte-identical**. Every
one of the seven is the output of a `DATASET MAP` through a curvilinear
coordinate system (`special_functions\src\coords.rs`, host libm). The
source datasets they were mapped from (`solveit37_orbit.db`,
`solveit38_helix.db`) are byte-identical.

Quantified per `states` table, Windows file against macOS file:

| dataset | float values that differ | worst relative difference |
|---|---|---|
| `solveit37_pol.db` | 12 of 1400 | 1.99e-16 |
| `solveit37_sph.db` | 3 of 1400 | 1.30e-16 |
| `solveit38_bis.db` | 3 of 700 | 1.58e-15 |
| `solveit38_cyl.db` | 5 of 700 | 1.79e-16 |
| `solveit38_obl.db` | 16 of 700 | 2.67e-14 |
| `solveit38_tor.db` | 5 of 700 | 4.02e-16 |

(`solveit37_sph_plots.html` is the plots page drawn from
`solveit37_sph.db`.) To reproduce the count yourself, check the macOS
files out beside the repository and compare the tables with Python's
standard library:

```powershell
git worktree add ..\macos-files macos/main
python -c "import sqlite3,sys; a=sqlite3.connect(sys.argv[1]); b=sqlite3.connect(sys.argv[2]); ra=a.execute('SELECT * FROM states ORDER BY frame,body').fetchall(); rb=b.execute('SELECT * FROM states ORDER BY frame,body').fetchall(); n=0; worst=0.0; total=0; [(total:=total+1, (n:=n+1, worst:=max(worst, abs(x-y)/max(abs(x),abs(y),1e-300))) if x!=y else None) for p,q in zip(ra,rb) for x,y in zip(p,q) if isinstance(x,float)]; print(len(ra),'rows;',n,'of',total,'float values differ; worst relative',worst)" datasets\solveit37_pol.db ..\macos-files\datasets\solveit37_pol.db
git worktree remove ..\macos-files
```
(Git Bash: the same with `../macos-files` and forward slashes.)

The two transcript lines of examples 36–38 that differ from the
macOS-quoted ones (`logs\solveit_scripts\solveit_37_dataset_map_spherical.log`,
`solveit_38_dataset_curvilinear_tour.log`): example 37
`Out[21]= [3.0000000000000004, 3.9999999999999996]` (the
`cart2_to_polar` round trip) and example 38
`Out[22]= 0.0000000000000007447602459741819`. Every other numeric `Out`
line of examples 36–38 is verbatim identical to the macOS transcript;
`SolveIt.md` / `.tex` carry the Windows values.

### II.8.7 The notebook encyclopedia

```powershell
cargo build --release -p posim
cd dataset_tools; cargo build --release; cd ..
cd planet_Mercury\mercury_rs; cargo build --release; cd ..\..
python SolveIt_Notebooks_for_rust\_tools\gen.py
```
Result: manifest of **147 entries** (2 Mercury + 38 SolveIt + 59
dynamics/Routh + 12 collisions + 13 mechanism videos + 6 compiled Rust
examples + 17 rebound), copies and `player_<tag>.py` written.

Executing the copies (one call per notebook; the batch was split into
chunks and run in parallel):

```powershell
$env:POSIM_NO_BROWSER = "1"
Get-ChildItem SolveIt_Notebooks_for_rust\0*\*\*.ipynb | ForEach-Object {
    python SolveIt_Notebooks_for_rust\_tools\run_copy.py $_.FullName
}
```
(No `-Recurse`: it would also feed `.ipynb_checkpoints` copies to the
runner.) Result: `02_solveit` 38/38 ok, `03_dynamics` 59/59 ok, `04` +
`05` 25/25 ok, `06_rust` 6/6 ok, `07_rebound` 17/17 ok
(`SolveIt_Notebooks_for_rust\_tools\exec_posim_0.log` … `_3.log`,
`exec_rebound.log`). The two `01_mercury` copies were re-executed after
the UTF-8 driver fix (§II.4): `SolveIt_Notebooks_for_rust\_tools\exec_mercury.log`
records `ok … mercury_tidal_locking.ipynb (18 cells)` / `1 ok, 0 failed`
and `exec_mercury2.log` records `ok … mercury_test2_jupiter_gr.ipynb (17
cells)` / `1 ok, 0 failed` — **147/147 copies ok**. `encyclo.py` reads
`_tools\exec_*.log` in filename order and the last verdict per tag wins;
that is why the earlier dry-run header line of `ENCYCLOPEDIA.md` said
"146/147 green": at dry-run time `exec_mercury.log` still held the FAIL
of the interrupted first copy run (`mercury_tidal_locking`), while the
`mercury_test2_jupiter_gr` "PASS" came from the macOS-era
`exec_mercury2.log` that arrived with the encyclopedia (both files now
hold this machine's verdicts). The macOS-era `exec_rebound_retry.log`
is also still present; its one verdict (`shearing_sheet` ok, 4 cells)
agrees with `exec_rebound.log`.

```powershell
python SolveIt_Notebooks_for_rust\_tools\shoot.py --workers 4
```
Result: **124 captured, 0 failed** — one PNG per GUI-family entry
(38 SolveIt + 59 dynamic + 12 collision + 13 video + 2 Mercury = 124;
the 6 `rust_*` and 17 rebound entries have no browser GUI by design),
smallest image 18,039 bytes; headless Google Chrome at
`C:\Program Files\Google\Chrome\Application\chrome.exe`
(`SolveIt_Notebooks_for_rust\_tools\shoot.log`).

```powershell
python SolveIt_Notebooks_for_rust\_tools\encyclo.py
python SolveIt_Notebooks_for_rust\_tools\verify.py
```
Result of the final pass (after the Mercury copies' re-execution;
committed as `5d87e8d`): `ENCYCLOPEDIA.md` 162,628 characters,
`ENCYCLOPEDIA.html` 187,017 characters, `ENCYCLOPEDIA.pdf` 1,364,723
bytes (about 46 pages, headless-Chrome print), header line "147/147
green", all 147 entries "Executed: PASS"; `verify.py`: **`ALL VERIFY
GATES PASS (147 notebooks)`** — V1 147 entries with topic counts
2/38/59/12/13/6/17, V2 every header cell present, V3 every player
compiles and matches its embedded copy, V4 execution counts present, V5
124 images and none for the rust/rebound families, V6 every tag exactly
once. (The builder's earlier dry run, before the Mercury re-execution,
had given 162,608 / 186,997 characters and 1,364,749 bytes with the
same `verify.py` line but a "146/147 green" header, for the reason
given above.)

### II.8.8 The 13 recorded videos and the recorder tests

```powershell
python recorder\src\record_all.py --check
python recorder\tests\test_units.py
python recorder\tests\test_end_to_end.py
```
Result: **`all 13 recordings reproduce byte for byte`** (sizes in bytes:
kepler_ellipse 142106, tumbling_racket 118201, box_of_shapes 287976,
double_pendulum_hinges 284436, universal_joint 205556, ball_joint_chain
373656, rod_pendulum_chain 310608, spinning_top 190450, gyroscope_gimbal
333644, cardan_compass 324423, cardan_gear 279671, rack_and_pinion
170255, piston_crankshaft 316882); `Ran 17 tests ... OK`; `Ran 9 tests
... OK` (`logs\videos_check.log`). `rack_and_pinion.html` is the Windows
recording kept at the merge (§II.3); it reproduces here byte for byte.

### II.8.9 The 13 live GUI pages

```powershell
cargo build --release -p posim
python tools\win_gui_smoke.py
python tools\gui_smoke.py
```
Result: **13 of 13 GUIs pass** and **13 of 13 GUIs pass** (ports
8895–8907: piston_crankshaft 8895, rack_and_pinion 8896,
gyroscope_gimbal 8897, cardan_compass 8898, universal_joint 8899,
spinning_top 8900, ball_joint_chain 8901, cardan_gear 8902,
rod_pendulum_chain 8903, double_pendulum_hinges 8904, tumbling_racket
8905, kepler_ellipse 8906, box_of_shapes 8907) — page served with its
canvas, `/api/state` live, Start advances simulated time, Reset returns
to t = 0 (`logs\gui_smoke.log`). To open one yourself:
`python gui\kepler_ellipse\server.py` then
`Start-Process http://127.0.0.1:8906/` (Git Bash: `start http://127.0.0.1:8906/`).

### II.8.10 The Jupyter wire protocol and the wrapper kernel

```powershell
cargo build --release
python jupyter\test_protocol.py
uv venv jupyter\.venv
uv pip install -p jupyter\.venv\Scripts\python.exe ipykernel jupyter_client
$env:POSIM_NO_BROWSER = "1"
jupyter\.venv\Scripts\python.exe jupyter\test_kernel.py
```
Result: `all protocol checks passed`; the venv is Python 3.14.4; all 7
kernel cells `ok` — `all kernel checks passed — JupyterLab can drive
this kernel` (`logs\jupyter_tests.log`).

### II.8.11 The entity index

```powershell
python tools\verify_index_examples.py 2>&1 | Tee-Object logs\verify_index_examples.log
python tools\verify_tierb_examples.py 2>&1 | Tee-Object logs\verify_tierb_examples.log
```
Result: **PASS 1365/1365 (100.0%)** in 35 s wall clock (Part I: 1177 —
the macOS project cataloged the `DATASET` / `SQL` / `MOVIE` families);
**COMPILED 258/258 (100.0%)**. Open the index with
`Start-Process index_of_entities.html` (Git Bash: `start index_of_entities.html`).

### II.8.12 rebound_rs 5.1.1 and reboundx_rs 5.1.0

**Build, tests, clippy** (each is its own workspace):

```powershell
cd rebound_rust
cargo build --release --all-targets 2>&1 | Tee-Object logs\win11_build_20260909.log   # --all-targets: the example .exe files the gates run
cargo test --release 2>&1 | Tee-Object logs\win11_test_20260909.log
cargo clippy --release --all-targets 2>&1 | Tee-Object logs\win11_clippy_20260909.log
cd ..\reboundx_rust
cargo build --release --all-targets 2>&1 | Tee-Object ..\logs\reboundx_build.log
cargo test --release 2>&1 | Tee-Object ..\logs\reboundx_test.log
cargo clippy --release --all-targets 2>&1 | Tee-Object ..\logs\reboundx_clippy.log
cd ..
```
Result: rebound_rs build 0 warnings; **394 passed** (67 + 32 + 39 + 72 +
55 + 70 + 59), 0 failed; clippy 0 warnings
(`rebound_rust\logs\win11_{build,test,clippy}_20260909.log`).
reboundx_rs build 0 warnings; **137 passed** (34 + 32 + 28 + 17 + 26;
2 doctests ignored), 0 failed; clippy 0 warnings
(`logs\reboundx_{build,test,clippy}.log`).

**The 17 notebooks** (install once into the `python` that runs them:
`python -m pip install --user nbformat nbclient ipykernel matplotlib`):

```bash
# Git Bash, from rebound_rust/notebooks
python make_notebooks.py
PYTHONIOENCODING=utf-8 python run_notebooks.py 2>&1 | tee run_all.log
```
Result: **`All 17 notebooks executed with no errors.`** — first on
regeneration (`rebound_rust\logs\win11_notebooks_20260909.log`) and
again after the C dumps below existed (`rebound_rust\notebooks\run_all.log`).
Final cells: `libm_diff` "compared 200000 samples / functions that
differ: {…60 entries…}" (the notebook tallies the 60 differing *lines*
by their first bit pattern; per function the difference is `pow` only);
`bs_pow_diff` "samples: 200008   mismatches: 56  (0.0280%) / ULP
distribution: {1: 56}"; `integrators_test` prints the Rust dump
(`t 4013ffffffffffa1`, `dt 3f847ae147ae147b`, `steps_done 500`);
`shearing_sheet_test`, `movetocom_var`, `tides_spin_pseudo` / `_kozai` /
`_migration` end `BIT-IDENTICAL: C and Rust agree on every bit`.

**The C-reference gates** — every command, number and file hash is in
`rebound_rust\logs\win11_c_reference_gates_20260909.md` (542 lines,
measured 16:48–16:54 on 2026-09-09); this is the summary. The
references: REBOUND at commit `dad5f97806ecbb408dcaff728851c64e67f9f6eb`
(`5.1.1-4-gdad5f978`) and REBOUNDx 5.1.0 (an unpacked archive —
`version.txt` — so no commit id), mounted as the gitignored folders
`rebound\rebound` and `reboundx` at the repository root:

```powershell
git clone https://github.com/hannorein/rebound.git rebound\rebound
cd rebound\rebound; git checkout dad5f97806ecbb408dcaff728851c64e67f9f6eb; cd ..\..
git clone https://github.com/dtamayo/reboundx.git reboundx
```
Build the C REBOUND library (GnuWin32 Make must be on `PATH` *before*
`vcvars64.bat` runs, exactly as written; compile C only from PowerShell
through `cmd /c`):

```powershell
$repo = (Get-Location).Path
cd rebound\rebound\examples\shearing_sheet
cmd /c 'set PATH=C:\Program Files (x86)\GnuWin32\bin;%PATH% && "C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat" && make'
cd ..\..\src
cmd /c '"C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul && lib /nologo /OUT:librebound_static.lib *.obj'
Copy-Item librebound.dll, librebound.lib, librebound_static.lib "$repo\rebound_rust\porttest\" -Force
Copy-Item librebound_static.lib "$repo\reboundx_rust\porttest\" -Force
cd $repo
```
(Every file compiles as `cl -c /DBUILDINGLIBREBOUND /D_GNU_SOURCE
/D_CRT_SECURE_NO_WARNINGS /D_CRT_NONSTDC_NO_WARNINGS /Ox /fp:precise
-DSERVER -DGITHASH=dad5f97… ` — flags `/Ox /fp:precise`; 31 objects,
`librebound.dll` 794,112 bytes.) Build the eleven REBOUND harnesses (in
`rebound_rust\porttest`; `libm_diff.c` and `bs_pow_diff.c` take no
library, `problem_test.c` produces `rebound_test.exe`):

```powershell
cd rebound_rust\porttest
foreach ($n in 'addfmt_test','archive_test','derivatives_test','frequency_test','integrators_test','kepler_rectilinear_c','movetocom_var_c','movetocom_var_test') {
  cmd /c "`"C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat`" >nul && cl /nologo /I`"..\..\rebound\rebound\src`" /D_GNU_SOURCE /D_CRT_SECURE_NO_WARNINGS /D_CRT_NONSTDC_NO_WARNINGS /Ox /fp:precise $n.c librebound.lib /Fe:$n.exe"
}
cmd /c '"C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul && cl /nologo /Ox /fp:precise libm_diff.c /Fe:libm_diff.exe'
cmd /c '"C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul && cl /nologo /Ox /fp:precise bs_pow_diff.c /Fe:bs_pow_diff.exe'
cmd /c '"C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul && cl /nologo /I"..\..\rebound\rebound\src" /D_GNU_SOURCE /D_CRT_SECURE_NO_WARNINGS /D_CRT_NONSTDC_NO_WARNINGS /Ox /fp:precise problem_test.c librebound.lib /Fe:rebound_test.exe'
cd ..\..
```
Build REBOUNDx (MSVC rejects the two variable-length arrays in
`gr_full.c` and `interpolation.c`; the documented shim in
`reboundx_rust\porttest\msvc_shim\` replaces exactly those two files)
and its five harnesses:

```powershell
$repo = (Get-Location).Path
cd reboundx\src
cmd /c '"C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul && cl /nologo /c /I"..\..\rebound\rebound\src" /I"." /D_GNU_SOURCE /D_CRT_SECURE_NO_WARNINGS /D_CRT_NONSTDC_NO_WARNINGS /DLIBREBOUNDX /Ox /fp:precise *.c'
cd $repo\reboundx_rust\porttest\msvc_shim
cmd /c '"C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul && cl /nologo /c /I"..\..\..\rebound\rebound\src" /I"..\..\..\reboundx\src" /D_GNU_SOURCE /D_CRT_SECURE_NO_WARNINGS /D_CRT_NONSTDC_NO_WARNINGS /DLIBREBOUNDX /Ox /fp:precise gr_full.c interpolation.c'
Copy-Item gr_full.obj, interpolation.obj "$repo\reboundx\src\" -Force
cd $repo\reboundx\src
cmd /c '"C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul && lib /nologo /OUT:libreboundx.lib *.obj'
Copy-Item libreboundx.lib "$repo\reboundx_rust\porttest\" -Force
cd $repo\reboundx_rust\porttest
foreach ($n in 'tides_spin_pseudo_c','tides_spin_kozai_c','tides_spin_migration_c','rebx_binary_roundtrip_c','rebx_binary_read_c') {
  cmd /c "`"C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat`" >nul && cl /nologo /I`"..\..\rebound\rebound\src`" /I`"..\..\reboundx\src`" /D_GNU_SOURCE /D_CRT_SECURE_NO_WARNINGS /D_CRT_NONSTDC_NO_WARNINGS /Ox /fp:precise $n.c libreboundx.lib librebound_static.lib /Fe:$n.exe"
}
cd $repo
```
(The first `cl` exits 2 with exactly the nine VLA errors from
`gr_full.c(63)`, `gr_full.c(185)` and `interpolation.c(67)` and produces
the other 31 objects; that is the expected, documented outcome.)

Measure (in `rebound_rust\porttest`; every comparison is
`Compare-Object (Get-Content <c>) (Get-Content <rust>)` plus
`Get-FileHash -Algorithm SHA256` on both files):

```powershell
cd rebound_rust\porttest
.\libm_diff.exe;            ..\target\release\examples\libm_diff.exe
.\bs_pow_diff.exe;          ..\target\release\examples\bs_pow_diff.exe
.\derivatives_test.exe;     ..\target\release\examples\derivatives_test.exe
.\frequency_test.exe;       ..\target\release\examples\frequency_test.exe
.\addfmt_test.exe;          ..\target\release\examples\addfmt_test.exe
.\movetocom_var_c.exe;      ..\target\release\examples\movetocom_var.exe
.\movetocom_var_test.exe;   ..\target\release\examples\movetocom_var_test.exe
.\kepler_rectilinear_c.exe 1e-12 1 2.0;  ..\target\release\examples\kepler_rectilinear.exe 1e-12 1 2.0   # and vy in {0,1e-12} x dt in {0.1,0.5,2.0,5.0,-2.0} at n = 20
powershell -NoProfile -ExecutionPolicy Bypass -File .\run_integrator_matrix.ps1 500
.\integrators_test.exe;     ..\target\release\examples\integrators_test.exe
Start-Process ..\target\release\examples\server_test.exe -WorkingDirectory (Get-Location).Path -NoNewWindow -RedirectStandardOutput server_stdout.txt
Start-Sleep -Seconds 2
curl.exe -s http://localhost:12873/simulation --output served.bin
curl.exe -s http://localhost:12873/keyboard/81
.\archive_test.exe whfast load served.bin      # then Compare-Object archive_state_c.txt server_state_rust.txt
foreach ($integ in 'whfast-usafe','ias15') {
  .\archive_test.exe $integ write;                              Copy-Item archive_state_c.txt   "arch_${integ}_c_write.txt"
  ..\target\release\examples\archive_test.exe $integ continue   # Rust continues archive_c_<integ>.bin
  ..\target\release\examples\archive_test.exe $integ write;     Copy-Item archive_state_rust.txt "arch_${integ}_rust_write.txt"
  .\archive_test.exe $integ continue                            # C continues archive_rust_<integ>.bin
}
.\rebound_test.exe 400;     ..\target\release\examples\shearing_sheet_test.exe 400
Get-FileHash state_c_final.txt, state_rust_final.txt, state_c_init.txt, state_rust_init.txt -Algorithm SHA256
cd ..\..\reboundx_rust\porttest
foreach ($t in @(@('pseudo','628.3185307179586'),@('kozai','100000.0'),@('migration','628.3185307179586'),@('pseudo','62.83185307179586'),@('kozai','1000.0'),@('migration','62.83185307179586'))) {
  $n = $t[0]
  & ".\tides_spin_${n}_c.exe" $t[1] 2>$null
  & "..\target\release\examples\tides_spin_$n.exe" $t[1] 2>$null
  Compare-Object (Get-Content "state_${n}_c.txt") (Get-Content "state_${n}_rust.txt")
}
..\target\release\examples\rebx_binary_roundtrip.exe
.\rebx_binary_roundtrip_c.exe rebx_c_reference.bin
$a=[IO.File]::ReadAllBytes("$pwd\rebx_binary_roundtrip.bin"); $b=[IO.File]::ReadAllBytes("$pwd\rebx_c_reference.bin")
$d=0; for($i=0;$i -lt $a.Length;$i++){ if($a[$i] -ne $b[$i]){ $d++ } }; "differing bytes: $d"
..\target\release\examples\rebx_binary_roundtrip.exe rebx_c_reference.bin
.\rebx_binary_read_c.exe rebx_binary_roundtrip.bin 2>$null | Out-File -Encoding ascii readback_from_rust.txt
.\rebx_binary_read_c.exe rebx_c_reference.bin      2>$null | Out-File -Encoding ascii readback_from_c.txt
Compare-Object (Get-Content readback_from_rust.txt) (Get-Content readback_from_c.txt)
cd ..\..
```

Measured results (all from `rebound_rust\logs\win11_c_reference_gates_20260909.md`):

| gate | measured on this machine |
|---|---|
| libm differential, 200,000 samples x 9 functions | `sin` `cos` `tan` `atan2` `sqrt` `fmod` `exp` `log`: **0 differences**; `pow(x, -0.234)`: **60 of 200,000** (0.0300 %), ULP distribution `{1: 60}` — every one exactly 1 ULP |
| `bs_pow` differential, 200,000 samples | **56 of 200,000** E samples (0.0280 %), `{1: 56}`; 0 of the 8 `P` constants differ; per k `{1:5, 2:8, 3:8, 4:3, 5:7, 6:8, 7:7, 8:10}` |
| integrator matrix, 63 configurations x 500 steps | **63 of 63 bit-identical** (2.3 s) |
| ias15 default run, 1000 adaptive steps | 7/7 dump lines identical, SHA-256 equal |
| shearing sheet, 400 steps | 1482 particles, 102533 collisions; final SHA-256 both `75BDAAB7109F125192F56AEB0CCDCAC554AF88A165FE11075EC5A871178521F0`, initial both `82B4013994FFF67315AA126CAA3E993B45A66CD42028C1D00558D7F92EAC86F1`; C 2.230 s, Rust 1.700 s |
| Simulationarchive round trips (whfast-usafe, ias15) | C write / Rust continue, Rust write / C continue, write vs write — **all 6 comparisons 0 differing lines**, SHA equal |
| web server | `served.bin` 3448 bytes; C-loaded state identical to the Rust dump, 6/6 lines |
| derivatives / frequency / addfmt / movetocom_var | **130/130** (the C file is CRLF; identical after CRLF -> LF), **30/30**, **12/12**, **5/5** lines identical, SHA equal |
| `movetocom_var_test` (stdout) | all 5 raw-bit fields identical; text differs only in `e+00` (C `%e`) vs `e0` (Rust) |
| `kepler_rectilinear`, 11 cases | 11/11 raw-bit `AFTER` lines identical |
| REBOUNDx `tides_spin` pseudo / kozai / migration, short and long | **6/6 BIT-IDENTICAL** (long runs: pseudo C 2.46 s / Rust 0.86 s; kozai 100000 C 1.38 s / Rust 1.61 s; migration C 5.67 s / Rust 1.76 s) |
| REBOUNDx binary | 25/25 self round trip; 25/25 Rust reading the C file with byte-identical re-serialization; files 6392 bytes, 6366 identical, the 26 differing bytes at offsets 37–62 are the git-hash header field; C readback of the Rust file 28/28 lines identical |

Reused from the earlier (interrupted) session of the same day, with
flags proven by its make/harness logs: `librebound.dll` / `.lib` + the
31 objects, the 11 REBOUND harness `.exe`, the Rust example binaries.
Rebuilt fresh in the measurement session: `librebound_static.lib`, all
31 REBOUNDx objects, the two shim objects, `libreboundx.lib`, the 5
REBOUNDx harnesses.

### II.8.13 planet_Mercury

```powershell
cd planet_Mercury\mercury_rs
cargo build --release 2>&1 | Tee-Object ..\..\logs\mercury_rs_build.log
cargo test --release 2>&1 | Tee-Object ..\..\logs\mercury_rs_test.log
cd ..\..
```
Result: build 0 warnings; **19 passed, 0 failed** (`tests\physics.rs`;
`logs\mercury_rs_{build,test}.log`); `planet_Mercury\mercury_rs\target\release\mercury_rs.exe`
is 1,104,384 bytes.

```powershell
cd planet_Mercury\mercury_crosscheck
cargo build --release
cargo run --release
cd ..\..
```
Result: build 0 warnings; the run prints (an upstream REBOUNDx notice
about the 4.5.0 tidal-potential fix also prints — expected):

```
spin ratio 100 x n, e = 0.20563, 200 orbits with IAS15 + tides_spin
measured  dOmega/dt = -3.652928e-18 rad/s^2  (REBOUNDx tides_spin)
predicted dOmega/dt = -3.643780e-18 rad/s^2  (Hut 1981 secular formula)
relative difference = 2.511e-3
PASS - crosscheck.despin_rate: two independent formulations agree within 2%
SUCCESS
```
(`logs\crosscheck_stress_launcher.log`.)

The two notebooks — built deterministically, executed headless, audited:

```powershell
cd planet_Mercury\notebook
python build_notebook.py
python build_notebook2.py
$env:MERCURY_NO_BROWSER = "1"
python run_notebook.py mercury_tidal_locking.ipynb
python check_notebook.py mercury_tidal_locking.ipynb
python run_notebook.py mercury_test2_jupiter_gr.ipynb
python check_notebook.py mercury_test2_jupiter_gr.ipynb
cd ..\..
```
(Git Bash: `MERCURY_NO_BROWSER=1 python run_notebook.py mercury_tidal_locking.ipynb`, etc.)
Result: `mercury_tidal_locking.ipynb` **ok (18 cells), 1 ok, 0 failed;
all structure rules pass**; `mercury_test2_jupiter_gr.ipynb` **ok (17
cells), 1 ok, 0 failed; all structure rules pass**
(`logs\mercury_notebook1.log`, `logs\mercury_notebook2.log`). The runs
are long (test 1 integrates 711,041,787 CVODE steps) and rebuild
`planet_Mercury\data\` (gitignored). The numbers are deterministic run
to run on this machine.

**Windows figures against the committed macOS run** (every assertion
of both gauntlets passes on both platforms; macOS value in parentheses
where it differs; cause: `mercury_rs` evaluates its tidal and triaxial
torques with the host libm — UCRT here, Apple libm there — while CVODE
itself runs on the vendored deterministic engine):

Test 1, `mercury_tidal_locking` — all PASS, 6/6 runs SUCCESS:
`A.final_ratio` 178.9260 after 10 Myr (same); full-model proof segment
Omega/n = 181.446450 (181.446443); restart root-found
1.600000000000e+00 at t = 4.4315 Myr (1.599999999963e+00 at 4.4316);
samples 10762 (10763); 64-branch sweep capture fraction 5/64 = 0.0781
+/- 0.034 (macOS 10/64 = 0.1562 +/- 0.045; the Goldreich–Peale estimate
is ~0.070, and the notebook's own text says capture is bit-level
path-sensitive with 64-flip error bars); canonical capture at t = 4.6501
Myr (4.6499); settled-era mean Omega/n = 1.5000000368 (1.5000000498);
final P_orb 87.9678 d (same), P_rot 58.6512 d (58.6419), solar day
175.9896 d (175.9061) vs observed 87.969 / 58.646 / 175.938; two-thirds
lock 8.8204e-09 (1.6579e-08); ledger pre-capture 4.029e-10 (3.947e-10)
against a 1e-9 budget; locked-era drift 8.821e-12 (9.156e-12) vs the
predicted leak 1.134e-09; libration swing 6.232 -> 0.140 rad (6.224 ->
0.152); run D guaranteed capture at 765.2 kyr (762.6), libration period
27.46 yr vs pendulum-exact 29.13 (27.52 / 29.25); run E seam relative
difference 1.062e-03 (1.201e-03); 711041787 CVODE steps (711084473),
109388 re-anchorings (same).

Test 2, `mercury_test2_jupiter_gr` — all PASS, 5/5 runs SUCCESS: GR
precession 42.983 "/century vs analytic 42.983 (rel diff 7.193e-13,
same); forced-eccentricity amplitude 4.5438e-03 = the Laplace–Lagrange
prediction (same); secular period 808.8 kyr (same); restart
1.600000000000e+00 at 4.4615 Myr (1.599999999995e+00 at 4.4612);
16-branch sweep 5/16 captured = 0.3125 (macOS 2/16 = 0.1250); canonical
capture at 4.6781 Myr (4.6779); settled mean ratio 1.5000004081 vs
predicted 1.5000003777, |diff| 3.05e-08 (1.5000004545, 7.69e-08);
perihelion advance 203.21 "/century vs predicted 203.22 (same); P_rot
58.6464 d (58.6447), solar day 175.9463 d (175.9314); 687068258 CVODE
steps (687207958).

### II.8.14 The documents

```powershell
pdflatex -interaction=nonstopmode SolveIt.tex
pdflatex -interaction=nonstopmode SolveIt.tex
pdflatex -interaction=nonstopmode grammar.tex
pdflatex -interaction=nonstopmode grammar.tex
pdflatex -interaction=nonstopmode scene_info.tex
pdflatex -interaction=nonstopmode scene_info.tex
pdflatex -interaction=nonstopmode physical_object_simulator.tex
pdflatex -interaction=nonstopmode physical_object_simulator.tex
```
(Each twice — the table of contents needs the second pass.) Result,
MiKTeX-pdfTeX 4.18, zero errors: `SolveIt.pdf` 49 pages (693,477
bytes); `grammar.pdf` 74 pages (970,144 bytes); `scene_info.pdf` 15
pages (388,864 bytes); `physical_object_simulator.pdf` 19 pages
(523,088 bytes) (`logs\pdflatex_*_pass2.log`). `collision_detection.pdf`
is unchanged (its `.tex` was not edited). `ENCYCLOPEDIA.pdf` is printed
by headless Chrome from `encyclo.py` (§II.8.7).

### II.8.15 The licence certifier

```bash
# Git Bash, from the repository root
bash scripts/certify_clean.sh 2>&1 | tee logs/certify_clean.log
```
The final run (`logs\certify_clean.log`, after every source and
documentation edit of this port): **every gate PASS in the final run (exit 0)**. The preliminary run
before the fixes of §II.4 (`logs\certify_clean_prelim.log`): every gate
**PASS** — no reference-tree files tracked or
reachable in history, no encumbered-code signatures, the gitignore
rules, the release build with 0 warnings, all 638 tests, every
StageNbks notebook executes, no cross-references, no retired claim —
except two: clippy (the 15 lints above, since fixed) and the live
test-count rule (five pages stated a count the tree does not have:
`CLEANROOM_PROVENANCE.md`, `EXPORT_PROVENANCE.md`, `PROJECT_STATUS.md`,
`SPECIAL_FUNCTIONS_PROVENANCE.md` said 568; `README.md` said 622; the
tree has 638). Per `CLEANROOM_PROVENANCE.md`'s own rule, the four
lineage pages' "568 passed, workspace-wide" sentences were rephrased to
dated records ("568 passed at the time of this record"),
`EXPORT_PROVENANCE.md`'s reproduce block now says "expect 638 passed
(568 at the 2026-07-26 export)", and `README.md` states the live 638.
Running the certifier after those edits is the last step of the port;
its final line is the verdict.

## II.9 What is and is not claimed

- **Claimed**: on this machine, this repository builds warning-free
  (workspace, sidecar, `rebound_rs`, `reboundx_rs`, `mercury_rs`,
  `mercury_crosscheck`); passes 638/638 workspace tests, 11/11 sidecar
  tests, 394/394 + 137/137 rebound tests and 19/19 Mercury tests; runs
  all 50 scripts, all 128 notebooks (8/8 checks), the 17 rebound
  notebooks and the 2 Mercury notebooks green; captures 124 encyclopedia
  images with 0 failures; serves 13/13 GUIs twice over; reproduces all
  13 videos byte for byte; verifies 1365/1365 index examples and
  258/258 snippets; reproduces the Linux physics evidence modulo the two
  pinned divergences of Part I; reproduces the macOS project's 74
  movies and 245 of its 252 datasets byte for byte, the remaining 7
  differing by at most 2.67e-14 relative in individual mapped
  coordinates; and reproduces every rebound C-reference gate
  bit-identically except the documented 1-ULP `pow` cases (60/200,000
  and 56/200,000).
- **Not claimed**: byte-identity across platforms of anything that
  calls the host math library (the quantum crate, the GEAR/RACK joint
  residual, the curvilinear coordinate maps, `mercury_rs`'s torques —
  all listed with their measured sizes above), or any verification
  result on any platform other than Windows 11 x86-64 as configured in
  §II.0.
