# REBOUND_PROVENANCE — notebooks/dynamic_spinning_target.ipynb

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
`dynamic_notebooks/spinning_target.posim` — a **rebound off a moving
surface**. A cuboid (m = 2, half-extents 0.6) spins at ω_z = 2; a
sphere (m = 1, r = 0.3) flies in at [2, 0, 0], aimed above centre
(y = 0.4). The surface point the sphere meets is itself moving
(ω × r), so the approach speed at contact is 2.40, not 2 — the
K-matrix's angular terms handle exactly this. The t ≈ 0.97 impact
trades the target's spin for the sphere's rebound, yet the conserved
totals stay pinned through the collision.

## Measured on this machine

- Cuboid's own spin before impact: `I_z = (2/3)(0.36+0.36) = 0.48`,
  so `L_z = 0.96` — printed as `[0, 0, 0.96]`.
- Conserved totals, held through the impact: `L = [0, 0, 0.16]`,
  `P = [2, 0, 0]`, `E = 2.96` (0.96 spin + 2 translational).
- The impact trades spin for rebound: cuboid `L_z` 0.96 → 0.31. The
  dynamic notebook stops at the scene window, so the post-impact value
  is measured by the collision script that runs the identical scenario
  to completion, `scripts/collisions/09_spinning_target.posim`: impact
  at `t = 0.9655899592834437`, approach speed `2.4034935760686995`,
  cuboid `L_z` afterwards `[0, 0, 0.31044030597153427]`
  (`logs/collision_09_spinning_target.log`; the same line of
  `evidence/win11/collision-scripts-win11.log`, which the gate found
  `IDENTICAL` to the Linux record).
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
cargo run -p posim --release -- --script dynamic_notebooks/spinning_target.posim
```

(Git Bash: `POSIM_NO_BROWSER=1 cargo run -p posim --release -- --script
dynamic_notebooks/spinning_target.posim`. The built binary runs the
same way: `target\release\posim.exe --script
dynamic_notebooks\spinning_target.posim`.)

Execute the Jupyter notebook headlessly and write its outputs back
into it, then check it (stdlib-Python tools vendored here — nothing to
install beyond Python 3; they find `target\release\posim.exe` built
above, or the path in `$env:POSIM_BIN`):

```powershell
$env:POSIM_NO_BROWSER = "1"
python notebooks\_build\nbrun.py notebooks\dynamic_spinning_target.ipynb
python notebooks\_build\nbcheck.py notebooks\dynamic_spinning_target.ipynb
```

(Git Bash: `POSIM_NO_BROWSER=1 python notebooks/_build/nbrun.py
notebooks/dynamic_spinning_target.ipynb` then
`python notebooks/_build/nbcheck.py notebooks/dynamic_spinning_target.ipynb`.)

Display the notebook in a web browser with JupyterLab (it opens the
page in your default browser itself; if no tab appears, run
`Start-Process <the URL it prints>` — Git Bash: `start <URL>`):

```powershell
python -m pip install --user jupyterlab
jupyter lab notebooks\dynamic_spinning_target.ipynb
```

(Git Bash: `jupyter lab notebooks/dynamic_spinning_target.ipynb`.)

If PowerShell reports that `jupyter` is not recognized, the `--user`
install put its launcher in a per-user `Scripts` folder that is not on
`PATH` by default (here
`C:\Users\<you>\AppData\Roaming\Python\Python314\Scripts` for
Python 3.14); the same server starts as a module without touching
`PATH`:

```powershell
python -m jupyterlab notebooks\dynamic_spinning_target.ipynb
```

(Git Bash: `python -m jupyterlab notebooks/dynamic_spinning_target.ipynb`.)

Display the rebound live in this project's browser GUI — the scene
window opens in your default browser (posim launches it through
`cmd /C start`); press Start (or type `SCENE START`) to watch the
sphere strike the turning face and rebound while the hud's E, P, L
readouts hold their values through the impact; press C for the golden
contact arrows:

```powershell
tools\posim_notebook.cmd spinning_target
```

(Git Bash: `tools/posim_notebook spinning_target`. Both are the
launcher form of `cargo run -p posim --release -- --notebook
dynamic_notebooks/spinning_target.posim`. If the window does not
appear, open the address the notebook prints:
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
