# WINDOWS_PORT_COMPLETE — summary of the finished port

**rustSolveIt_Win11_SUNDIALS_7_8_0 is complete and verified on Windows
11.** This page summarizes what was delivered and carries the complete
set of commands to reproduce every result — you do not need any other
document to run anything named here.

## What this project is

A pure-Rust physics simulator for Windows 11 (x86-64): a notebook
command language (`posim`) over a rigid-body/particle physics library
(`physical_object`), with quantum-mechanics and special-function
command families, a live browser scene window, 13 live GUI web pages,
13 recorded browser videos, and 109 executed Jupyter notebooks — every
example in the repository has exactly one concomitant Jupyter notebook
that defines, implements, executes and displays it. All numerical
integration is done by a **vendored pure-Rust translation of SUNDIALS
7.8.0** (CVODE, CVODES, IDA, IDAS, KINSOL, ARKODE) — the Windows port
of that translation, whose pure-Rust glibc-translated math library
makes the physics **byte-identical** to the Linux release's recorded
evidence. Zero `unsafe`, zero crates.io dependencies, zero warnings,
zero network access at build time.

It was produced by porting
`rustSolveIt_Using_SUNDIALS_7_8_0/version-7.8.0` (Linux) to Windows 11,
with `SUNDIALS_7_8_Rust_port_for_Windows11` vendored as the engine.
Machine: Intel Core Ultra 9 275HX, 192 GB RAM, Windows 11 Pro for
Workstations 25H2. Toolchain: rustc/cargo 1.91.1,
`x86_64-pc-windows-msvc`, linking with the Microsoft linker from the
installed Visual Studio 2026 MSVC toolset (found by rustc
automatically; the project is pure Rust — no C/C++/Fortran/Intel/WSL2
compiler compiled anything). The CPU's FMA instruction is required and
pinned via `.cargo/config.toml`.

## The scoreboard (every number measured on this machine, 2026-08-22)

| gate | result |
|---|---|
| `cargo build --workspace --all-targets` | 0 errors, **0 warnings** |
| `cargo test --workspace` | **622 passed, 0 failed** (49 lib + 19 collision + 9 conservation + 42 DAE/equilibrium/sensitivity + 112 posim + 92 quantum + 233 special_functions + 11 vendored identities + 55 doctests) |
| 6 self-checking physics examples | 6/6 **SUCCESS** (Pluto anchor, step counts and drifts byte-identical to Linux) |
| 12 collision scripts | 12/12 exit 0; combined output **byte-identical** to the Linux evidence |
| 19 SolveIt worked-example scripts | 19/19 exit 0 |
| 59 dynamic notebooks | 59/59 exit 0; 57 **byte-identical** to Linux, 2 (the quantum pair) differ only in the last printed digit — pinned in `evidence/win11/accepted-divergences-dynamic.diff` |
| 13 recorded browser videos | re-recorded on Windows; `record_all.py --check`: **all 13 reproduce byte for byte** |
| 109 Jupyter notebooks | executed: **109 ok, 0 failed**; checker: **109/109 pass all seven requirements** |
| 13 live GUI servers | **13/13 pass** the automated HTTP smoke test (page, live state, Start advances, Reset returns bit-exactly to t = 0) |
| index of 6,309 entities | **1177/1177** runnable posim/machine examples pass; **258/258** Rust snippets compile |
| Jupyter wire protocol + kernel | all protocol checks passed; all 7 kernel cells ok |
| documentation PDFs | `grammar.pdf` (18 worked examples) and `SolveIt.pdf` (16 worked examples) rebuilt from their `.tex` |

The port changed **one Rust file** (two path-embedding tests), added
two build-configuration files, made the Python tooling find `posim.exe`
and speak UTF-8 on a cp1252 platform, fixed two Windows-only test
harness races, corrected notebook pairing paths for the flattened
layout, and added three tools (`tools\posim_notebook.cmd`,
`tools/win_verify_physics.sh`, `tools\win_gui_smoke.py`). Not one
physics formula, solver call, tolerance or heuristic moved — and the
byte-identity gate is what proves it.

## Complete commands

All from the repository root
`C:\Users\nsh\Developer\github\rustSolveIt_Win11_SUNDIALS_7_8_0\rustSolveIt_Win11_SUNDIALS_7_8_0`,
in PowerShell unless marked Git Bash.

Build and test:

```powershell
cargo build --workspace --all-targets
cargo build --release --workspace --all-targets
cargo test --workspace
```

The interactive notebook (type `HELP`; `scene create` opens the live
browser window with Start/Pause/Reverse/Reset and live E, P, L
readouts):

```powershell
cargo run
```

The six self-checking physics examples:

```powershell
cargo run -p physical_object --release --example kepler_orbit
cargo run -p physical_object --release --example outer_solar_system
cargo run -p physical_object --release --example tumbling_body
cargo run -p physical_object --release --example charged_in_b_field
cargo run -p physical_object --release --example newtons_cradle
cargo run -p physical_object --release --example bouncing_ball_restitution
```

Any example script — the 12 collision scripts
`scripts\collisions\01_head_on_exchange.posim` … `12_two_dumbbells.posim`,
the 19 SolveIt scripts
`scripts\solveit\01_elastic_head_on.posim` … `19_hinged_door.posim`,
and the 59 dynamic notebooks `dynamic_notebooks\*.posim` — runs as
(set `$env:POSIM_NO_BROWSER = "1"` first for headless):

```powershell
cargo run -p posim --release -- --script scripts/collisions/01_head_on_exchange.posim
```

A dynamic notebook, interactively with its live scene window:

```powershell
tools\posim_notebook.cmd kepler_orbit
tools\posim_notebook.cmd --list
```

The physics byte-identity gate against the Linux evidence (Git Bash):

```bash
bash tools/win_verify_physics.sh
```

The Jupyter notebooks — regenerate, execute all 109, check all 109
(first line PowerShell; the globbed lines Git Bash):

```powershell
python notebooks\_build\regen.py
```

```bash
POSIM_NO_BROWSER=1 python notebooks/_build/nbrun.py notebooks/*.ipynb
python notebooks/_build/nbcheck.py notebooks/*.ipynb
```

Open the notebooks in JupyterLab:

```powershell
python -m pip install --user jupyterlab
jupyter lab notebooks/
```

The wire protocol and the JupyterLab wrapper kernel:

```powershell
cargo build --release
python jupyter\test_protocol.py
uv venv jupyter\.venv
uv pip install -p jupyter\.venv\Scripts\python.exe ipykernel jupyter_client
$env:POSIM_NO_BROWSER = "1"
jupyter\.venv\Scripts\python.exe jupyter\test_kernel.py
```

The live GUI pages (one server per scene, fixed ports 8895–8907; open
the printed URL in your browser), and the automated pass over all 13:

```powershell
python gui\kepler_ellipse\server.py
python tools\win_gui_smoke.py
```

The recorded browser videos — watch, re-record, verify byte-identity:

```powershell
start videos\kepler_ellipse.html
python recorder\src\record_all.py
python recorder\src\record_all.py --check
python recorder\tests\test_units.py
python recorder\tests\test_end_to_end.py
```

Record a new video from any scene script:

```powershell
cargo build --release -p posim
python recorder/src/record_video.py videos/scenes/kepler_ellipse.posim `
     -o mine.html --frames 360 --dt 0.02 --title "Kepler orbit, e = 0.6"
```

The entity index and its verifiers:

```powershell
start index_of_entities.html
python tools\verify_index_examples.py
python tools\verify_tierb_examples.py
```

The documentation PDFs (each twice — the table of contents needs the
second pass):

```powershell
pdflatex -interaction=nonstopmode grammar.tex
pdflatex -interaction=nonstopmode grammar.tex
pdflatex -interaction=nonstopmode SolveIt.tex
pdflatex -interaction=nonstopmode SolveIt.tex
```

## Where the deliverables live

- `grammar.md` / `grammar.tex` / `grammar.pdf` — the complete command
  language: lexer, full EBNF, type system, every command, the stack
  machine, the SUNDIALS 7.8.0 engine, the browser videos, and 18 fully
  worked examples.
- `SolveIt.md` / `SolveIt.tex` / `SolveIt.pdf` — the complete solution
  guide for a first-time reader, with 16 fully documented worked
  examples (scripts in `scripts\solveit\`).
- `ARCHITECTURE.md` — module responsibilities and pinned cross-module
  contracts. `CLAUDE.md` — working rules for contributors and agents,
  including the Windows traps.
- `PORT_WIN11_PROVENANCE.md` — this port's full provenance: machine,
  compiler choice, the complete change list, and every verification
  command with its measured result.
- `notebooks\` — the 109 executed Jupyter notebooks (one per example);
  `gui\` — the 13 live GUI pages; `videos\` — the 13 recorded browser
  videos; `evidence\win11\` — the Windows gate logs and the two pinned
  divergence diffs.
