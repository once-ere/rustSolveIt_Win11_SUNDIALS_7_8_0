# REBOUND_PROVENANCE — notebooks/solveit_04_restitution_ladder.ipynb

Rebound physics, verified on **Windows 11 / x86-64 (Windows 11 Pro for
Workstations, build 10.0.26200; Intel Core Ultra 9 275HX; rustc 1.91.1,
`x86_64-pc-windows-msvc`, MSVC linker from the Visual Studio 2026 Build
Tools 14.51.36231; the vendored Windows engine
`once-ere/SUNDIALS_7_8_Rust_port_for_Windows11`)**, 2026-09-09. This
page is self-contained: every command needed to test, verify, execute
and display this notebook appears on this page, run from the
repository root in PowerShell (the Git Bash form follows in
parentheses where it differs). The repository is
`once-ere/rustSolveIt_Win11_SUNDIALS_7_8_0`, ported from the macOS /
Apple Silicon edition
(`once-ere/rustSolveIt_macos-silicon_SUNDIALS_7_8_0`), itself ported
from the Linux original (`once-ere/rustSolveIt_Using_SUNDIALS_7_8_0`);
this example's physics is byte-identical on all three.

## What this notebook is

The Jupyter notebook paired with the SolveIt worked example
`scripts/solveit/04_restitution_ladder.posim` (worked example 4 of the
SolveIt solution guide, one of its 38 scripts). A ball with
restitution e = 0.7 is dropped so its centre starts at 5.5 (radius
0.5) and falls 5.0 to the floor's top face under g = 10. Each
**rebound** reaches `e^(2n)` of the drop height on bounce n — the
restitution ladder:

- bounce 1 at t = 1.0, rebound speed 7.00, apex at t = 1.7 —
  height above floor `5·e² = 2.45`, centre at 2.95
- bounce 2 at t = 2.4, rebound speed 4.90, apex at t = 2.89 —
  height `5·e⁴ = 1.2005`, centre at 1.7005
- bounce 3 at t = 3.38, rebound speed 3.43, apex at t = 3.723 —
  height `5·e⁶`, centre at 1.088245

The notebook derives the ladder, runs the three bounces, and compares
each measured apex against its closed form.

## Measured on this machine

| bounce | measured centre apex | analytic centre apex |
|---|---|---|
| 1 | `2.949999999869974` | `2.9499999999999997` |
| 2 | `1.7004999998062575` | `1.7004999999999997` |
| 3 | `1.0882449997748824` | `1.0882449999999997` |

Three collisions counted; every apex agrees with its closed form to
~1e-10 (the apex is sampled on the output grid, which is what limits
it — the impact times themselves are solver rootfinding events). The
script's transcript (`logs/solveit_scripts/solveit_04_restitution_ladder.log`)
is **byte-identical** to the Linux release's recorded evidence — its
section of `evidence/port-7.8.0/solveit-examples.log`, the record that
covers SolveIt scripts 01–16 — and therefore to the macOS edition's.
All 38 SolveIt scripts and all 12 collision scripts ran headlessly
here, 50 of 50 exit 0 (`logs/solveit_and_collision_scripts.log`).
Notebook execution: ok, all cells; checker: passes all eight
requirements (128 of 128 notebooks in the full run,
`logs/notebooks_run.log`); the SQLite dataset it records is
byte-identical to the macOS file.

## Complete commands

Build the simulator and the dataset sidecar, once. The root
`.cargo/config.toml` pins `-C target-feature=+fma`, which the Windows
engine's deterministic libm requires (the CPU must support FMA). The
sidecar `target\release\solveit_db.exe` is needed because every
notebook records a SQLite dataset, and because SolveIt scripts 20–38
are the dataset/SQL/movie examples — posim spawns it, never links it:

```powershell
cargo build --release -p posim
cd dataset_tools; cargo build --release; cd ..
```

(Git Bash: `cargo build --release -p posim` then
`cd dataset_tools && cargo build --release && cd ..`.) The simulator
is `target\release\posim.exe`.

Run the paired worked-example script headlessly and read its
transcript (also documented, with captured output, as example 4 in the
SolveIt guide):

```powershell
$env:POSIM_NO_BROWSER = "1"
cargo run -p posim --release -- --script scripts/solveit/04_restitution_ladder.posim
```

(Git Bash: `POSIM_NO_BROWSER=1 cargo run -p posim --release -- --script
scripts/solveit/04_restitution_ladder.posim`. The built binary runs
the same way: `target\release\posim.exe --script
scripts\solveit\04_restitution_ladder.posim`.)

Execute the Jupyter notebook headlessly and write its outputs back
into it, then check it (stdlib-Python tools vendored here — nothing to
install beyond Python 3; they find `target\release\posim.exe` built
above, or the path in `$env:POSIM_BIN`):

```powershell
$env:POSIM_NO_BROWSER = "1"
python notebooks\_build\nbrun.py notebooks\solveit_04_restitution_ladder.ipynb
python notebooks\_build\nbcheck.py notebooks\solveit_04_restitution_ladder.ipynb
```

(Git Bash: `POSIM_NO_BROWSER=1 python notebooks/_build/nbrun.py
notebooks/solveit_04_restitution_ladder.ipynb` then
`python notebooks/_build/nbcheck.py
notebooks/solveit_04_restitution_ladder.ipynb`.)

Display the notebook in a web browser with JupyterLab (it opens the
page in your default browser itself; if no tab appears, run
`Start-Process <the URL it prints>` — Git Bash: `start <URL>`):

```powershell
python -m pip install --user jupyterlab
jupyter lab notebooks\solveit_04_restitution_ladder.ipynb
```

(Git Bash: `jupyter lab notebooks/solveit_04_restitution_ladder.ipynb`.)

If PowerShell reports that `jupyter` is not recognized, the `--user`
install put its launcher in a per-user `Scripts` folder that is not on
`PATH` by default (here
`C:\Users\<you>\AppData\Roaming\Python\Python314\Scripts` for
Python 3.14); the same server starts as a module without touching
`PATH`:

```powershell
python -m jupyterlab notebooks\solveit_04_restitution_ladder.ipynb
```

(Git Bash: `python -m jupyterlab notebooks/solveit_04_restitution_ladder.ipynb`.)

Display a bouncing-ball rebound live in this project's browser GUI —
the scene window opens in your default browser (posim launches it
through `cmd /C start`) with Start/Pause/Reverse/Reset and live E, P,
L readouts (the dynamic-notebook variant of the same physics; press
Start in the window):

```powershell
tools\posim_notebook.cmd bouncing_ball_restitution
```

(Git Bash: `tools/posim_notebook bouncing_ball_restitution`. Both are
the launcher form of `cargo run -p posim --release -- --notebook
dynamic_notebooks/bouncing_ball_restitution.posim`. If the window does
not appear, open the address the notebook prints:
`Start-Process http://127.0.0.1:<port>/` — Git Bash:
`start http://127.0.0.1:<port>/`.)

Verify the SolveIt scripts — run all thirty-eight and confirm each
exits 0 (measured here: 38 of 38; scripts 20–38 need the sidecar built
above):

```powershell
$env:POSIM_NO_BROWSER = "1"
Get-ChildItem scripts\solveit\*.posim | ForEach-Object { & .\target\release\posim.exe --script $_.FullName; if ($LASTEXITCODE -ne 0) { "FAILED $($_.Name)" } }
```

(Git Bash: `for f in scripts/solveit/*.posim; do POSIM_NO_BROWSER=1
target/release/posim.exe --script "$f" || echo "FAILED $f"; done`.)

Verify this script's byte-identity against the Linux record directly
(Git Bash; the record's sections end with the harness's `rc=` line, so
the run appends its own; silence followed by `IDENTICAL` = pass):

```bash
diff --strip-trailing-cr \
  <(awk '/^#* scripts\/solveit\/04_restitution_ladder.posim$/{p=1;next} /^#####/{p=0} p' evidence/port-7.8.0/solveit-examples.log) \
  <(POSIM_NO_BROWSER=1 target/release/posim.exe --script scripts/solveit/04_restitution_ladder.posim; echo "rc=$?") \
  && echo IDENTICAL
```
