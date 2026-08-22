# PORT_WIN11_PROVENANCE — the Windows 11 port, and the evidence it changed no physics

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
`C:\Users\nsh\Developer\github\rustSolveIt_Win11_SUNDIALS_7_8_0\rustSolveIt_Win11_SUNDIALS_7_8_0`.
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
