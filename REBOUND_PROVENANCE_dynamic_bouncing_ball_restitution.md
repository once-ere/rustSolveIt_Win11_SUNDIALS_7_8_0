# REBOUND_PROVENANCE — notebooks/dynamic_bouncing_ball_restitution.ipynb

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
this notebook's physics is byte-identical on all three.

## What this notebook is

The Jupyter notebook paired with the dynamic notebook script
`dynamic_notebooks/bouncing_ball_restitution.posim`. A ball (radius
0.5, mass 1) falls from rest at y = 5 onto a static floor slab under
uniform gravity g = 10, with restitution e = 0.8. The centre drops
h = 4.5 to its rest height 0.5, so the first impact lands at
`t = sqrt(2h/g) = sqrt(0.9) ≈ 0.9487`; the **rebound** speed is 0.8 of
the impact speed, so the next apex is `0.5 + e²·4.5 = 3.38`, and each
later apex is lower by the same geometric factor. The Jupyter notebook
drives the simulator over its JSON machine protocol, derives the
closed forms in its own text, and records the real outputs.

## Measured on this machine

- Time of first impact: `0.9486832980505138` — the analytic
  `sqrt(0.9)` to the last digit.
- Rebound apex of the centre: `3.38` — exactly `0.5 + e²·4.5`.
- Initial energy 50 (pure potential, m·g·y), dropping at every impact.
- The script's transcript is **byte-identical** to the Linux release's
  recorded evidence (`evidence/win11/dynamic-notebooks-win11.log`
  against `evidence/port-7.8.0/dynamic-notebooks-7.8.0.log`; the
  gate's only pinned divergences are last-digit lines in the two
  quantum notebooks, not this one) and therefore to the macOS
  edition's. Notebook execution: ok, all cells; checker: passes all
  eight requirements (128 of 128 notebooks in the full run,
  `logs/notebooks_run.log`); the SQLite dataset it records is
  byte-identical to the macOS file.

## Complete commands

Build the simulator and the dataset sidecar, once. The root
`.cargo/config.toml` pins `-C target-feature=+fma`, which the Windows
engine's deterministic libm requires (the CPU must support FMA). The
sidecar `target\release\solveit_db.exe` is needed because every
notebook records a SQLite dataset — posim spawns it, never links it:

```powershell
cargo build --release -p posim
cd dataset_tools; cargo build --release; cd ..
```

(Git Bash: `cargo build --release -p posim` then
`cd dataset_tools && cargo build --release && cd ..`.) The simulator
is `target\release\posim.exe`.

Run the paired script headlessly and read its transcript:

```powershell
$env:POSIM_NO_BROWSER = "1"
cargo run -p posim --release -- --script dynamic_notebooks/bouncing_ball_restitution.posim
```

(Git Bash: `POSIM_NO_BROWSER=1 cargo run -p posim --release -- --script
dynamic_notebooks/bouncing_ball_restitution.posim`. The built binary
runs the same way: `target\release\posim.exe --script
dynamic_notebooks\bouncing_ball_restitution.posim`.)

Execute the Jupyter notebook headlessly and write its outputs back
into it, then check it (stdlib-Python tools vendored here — nothing to
install beyond Python 3; they find `target\release\posim.exe` built
above, or the path in `$env:POSIM_BIN`):

```powershell
$env:POSIM_NO_BROWSER = "1"
python notebooks\_build\nbrun.py notebooks\dynamic_bouncing_ball_restitution.ipynb
python notebooks\_build\nbcheck.py notebooks\dynamic_bouncing_ball_restitution.ipynb
```

(Git Bash: `POSIM_NO_BROWSER=1 python notebooks/_build/nbrun.py
notebooks/dynamic_bouncing_ball_restitution.ipynb` then
`python notebooks/_build/nbcheck.py
notebooks/dynamic_bouncing_ball_restitution.ipynb`.)

Display the notebook in a web browser with JupyterLab (it opens the
page in your default browser itself; if no tab appears, run
`Start-Process <the URL it prints>` — Git Bash: `start <URL>`):

```powershell
python -m pip install --user jupyterlab
jupyter lab notebooks\dynamic_bouncing_ball_restitution.ipynb
```

(Git Bash: `jupyter lab notebooks/dynamic_bouncing_ball_restitution.ipynb`.)

If PowerShell reports that `jupyter` is not recognized, the `--user`
install put its launcher in a per-user `Scripts` folder that is not on
`PATH` by default (here
`C:\Users\<you>\AppData\Roaming\Python\Python314\Scripts` for
Python 3.14); the same server starts as a module without touching
`PATH`:

```powershell
python -m jupyterlab notebooks\dynamic_bouncing_ball_restitution.ipynb
```

(Git Bash: `python -m jupyterlab notebooks/dynamic_bouncing_ball_restitution.ipynb`.)

Display the rebound live in this project's browser GUI — the scene
window opens in your default browser (posim launches it through
`cmd /C start`) with Start/Pause/Reverse/Reset and live E, P, L
readouts; press Start (or type `SCENE START`) and watch the ball climb
to just below 3.4, then geometrically lower each bounce; press C for
the golden action–reaction contact arrows:

```powershell
tools\posim_notebook.cmd bouncing_ball_restitution
```

(Git Bash: `tools/posim_notebook bouncing_ball_restitution`. Both are
the launcher form of `cargo run -p posim --release -- --notebook
dynamic_notebooks/bouncing_ball_restitution.posim`. If the window does
not appear, open the address the notebook prints:
`Start-Process http://127.0.0.1:<port>/` — Git Bash:
`start http://127.0.0.1:<port>/`.)

Verify byte-identity of the whole 59-script dynamic-notebook suite
against the Linux evidence (this script is one of them; the gate also
re-runs the 6 compiled examples and the 12 collision scripts; exit 0 =
pass — measured here: `IDENTICAL-MODULO-DOCUMENTED` for the dynamic
notebooks, `IDENTICAL` for the collision scripts, `GATE_EXIT=0`,
`logs/win_verify_physics.log`). The gate is a bash script, run in Git
Bash:

```bash
bash tools/win_verify_physics.sh
```

(From PowerShell: `& "C:\Program Files\Git\bin\bash.exe" tools/win_verify_physics.sh`.)
