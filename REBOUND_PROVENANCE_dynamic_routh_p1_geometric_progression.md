# REBOUND_PROVENANCE — notebooks/dynamic_routh_p1_geometric_progression.ipynb

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

The Jupyter notebook paired with
`dynamic_notebooks/routh_p1_geometric_progression.posim` — E. J.
Routh, *A Treatise on Dynamics of a Particle* (Cambridge, 1898),
Part I, Art. 88: perfectly elastic balls in a line, each impinging
directly on the next; if their masses form a geometric progression of
common ratio 2, the velocities after impact form a geometric
progression. Four spheres of masses 1, 2, 4, 8 (restitution 1, no
gravity) realize the theorem: each impact passes 2/3 of the incoming
speed to the struck ball and **rebounds** the striker backwards, so
the struck-ball speeds run 2/3, (2/3)², (2/3)³ of the original — the
geometric progression Routh asks for. The notebook derives the impact
equations of Arts. 83–84 in full and checks the measured ratios.

## Measured on this machine

- Final velocity of the last (mass-8) ball:
  `0.2962962962962963 = 8/27` — the measured ratio against the exact
  value prints `1`.
- Rebound of the third ball after it strikes the fourth:
  `-0.14814814814814814` (exactly −4/27 — it bounces backwards).
- `3 collision(s)` through the chain; total momentum `[1, 0, 0]`
  conserved exactly; total energy `0.49999999999999994` against the
  initial 0.5; at tight tolerance the run reports
  `|dE/E| = 1.110e-16` — one machine epsilon.
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
cargo run -p posim --release -- --script dynamic_notebooks/routh_p1_geometric_progression.posim
```

(Git Bash: `POSIM_NO_BROWSER=1 cargo run -p posim --release -- --script
dynamic_notebooks/routh_p1_geometric_progression.posim`. The built
binary runs the same way: `target\release\posim.exe --script
dynamic_notebooks\routh_p1_geometric_progression.posim`.)

Execute the Jupyter notebook headlessly and write its outputs back
into it, then check it (stdlib-Python tools vendored here — nothing to
install beyond Python 3; they find `target\release\posim.exe` built
above, or the path in `$env:POSIM_BIN`):

```powershell
$env:POSIM_NO_BROWSER = "1"
python notebooks\_build\nbrun.py notebooks\dynamic_routh_p1_geometric_progression.ipynb
python notebooks\_build\nbcheck.py notebooks\dynamic_routh_p1_geometric_progression.ipynb
```

(Git Bash: `POSIM_NO_BROWSER=1 python notebooks/_build/nbrun.py
notebooks/dynamic_routh_p1_geometric_progression.ipynb` then
`python notebooks/_build/nbcheck.py
notebooks/dynamic_routh_p1_geometric_progression.ipynb`.)

Display the notebook in a web browser with JupyterLab (it opens the
page in your default browser itself; if no tab appears, run
`Start-Process <the URL it prints>` — Git Bash: `start <URL>`):

```powershell
python -m pip install --user jupyterlab
jupyter lab notebooks\dynamic_routh_p1_geometric_progression.ipynb
```

(Git Bash: `jupyter lab notebooks/dynamic_routh_p1_geometric_progression.ipynb`.)

If PowerShell reports that `jupyter` is not recognized, the `--user`
install put its launcher in a per-user `Scripts` folder that is not on
`PATH` by default (here
`C:\Users\<you>\AppData\Roaming\Python\Python314\Scripts` for
Python 3.14); the same server starts as a module without touching
`PATH`:

```powershell
python -m jupyterlab notebooks\dynamic_routh_p1_geometric_progression.ipynb
```

(Git Bash: `python -m jupyterlab notebooks/dynamic_routh_p1_geometric_progression.ipynb`.)

Display the rebound chain live in this project's browser GUI — the
scene window opens in your default browser (posim launches it through
`cmd /C start`); press Start (or type `SCENE START`) to watch the
impulse run down the chain of doubling masses; press C for the golden
contact arrows at each impact:

```powershell
tools\posim_notebook.cmd routh_p1_geometric_progression
```

(Git Bash: `tools/posim_notebook routh_p1_geometric_progression`. Both
are the launcher form of `cargo run -p posim --release -- --notebook
dynamic_notebooks/routh_p1_geometric_progression.posim`. If the window
does not appear, open the address the notebook prints:
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
