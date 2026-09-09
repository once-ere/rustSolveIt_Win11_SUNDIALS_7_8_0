# REBOUND_PROVENANCE — notebooks/rust_bouncing_ball_restitution.ipynb

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

The Jupyter notebook paired with the compiled, self-checking Rust
example `physical_object/examples/bouncing_ball_restitution.rs`. A
ball (radius 0.5, mass 1) is dropped so its centre falls h = 4.5 onto
a static floor under gravity g = 10, with coefficient of restitution
e = 0.8. The physics of a **rebound**: the impact must land at the
analytic time of impact `t = sqrt(2h/g) = sqrt(0.9)`, and the rebound
apex must be `e² × h` above the rest height, because the impulse
response scales the separation speed by exactly e. The example checks
both against closed forms and prints SUCCESS or FAILURE; the notebook
compiles and runs that example, asserts SUCCESS, and reproduces the
same physics interactively over the simulator's JSON machine protocol.

## Measured on this machine

```
Bouncing ball: h = 4.5, g = 10, e = 0.8
  impact time  : 0.9486832980505138 (analytic 0.9486832980505138)
  impact normal: [-0, 1, -0] (floor → ball)
  rebound apex : 3.37997191386479 (analytic 3.380000000000001)
SUCCESS: rebound apex = e^2 x drop height, impact at the exact TOI
```

The impact time matches the analytic value **to the last digit**, and
this output is **byte-identical** to the Linux release's recorded
evidence (`evidence/win11/examples-win11.log` against
`evidence/port-7.8.0/examples-7.8.0.log`; the examples suite's only
pinned divergence is one blank line in `outer_solar_system`, not this
example) and therefore to the macOS edition's. Notebook execution: ok,
all cells; checker: passes all eight requirements (128 of 128
notebooks in the full run, `logs/notebooks_run.log`); the SQLite
dataset it records is byte-identical to the macOS file.

## Complete commands

Build the simulator, the examples and the dataset sidecar, once. The
root `.cargo/config.toml` pins `-C target-feature=+fma`, which the
Windows engine's deterministic libm requires (the CPU must support
FMA). The sidecar `target\release\solveit_db.exe` is needed because
every notebook records a SQLite dataset — posim spawns it, never links
it:

```powershell
cargo build --release --workspace --all-targets
cd dataset_tools; cargo build --release; cd ..
```

(Git Bash: `cargo build --release --workspace --all-targets` then
`cd dataset_tools && cargo build --release && cd ..`.) The simulator is
`target\release\posim.exe`; the compiled example is
`target\release\examples\bouncing_ball_restitution.exe`.

Run the self-checking example directly (prints the block quoted above,
exits 0 on SUCCESS and nonzero on FAILURE):

```powershell
cargo run -p physical_object --release --example bouncing_ball_restitution
```

(Identical in Git Bash. The built binary runs the same way:
`target\release\examples\bouncing_ball_restitution.exe`.)

Execute the notebook headlessly and write its outputs back into it,
then check it (both are stdlib-Python tools vendored in this
repository — nothing to install beyond Python 3; they find
`target\release\posim.exe` built above, or the path in
`$env:POSIM_BIN`):

```powershell
$env:POSIM_NO_BROWSER = "1"
python notebooks\_build\nbrun.py notebooks\rust_bouncing_ball_restitution.ipynb
python notebooks\_build\nbcheck.py notebooks\rust_bouncing_ball_restitution.ipynb
```

(Git Bash: `POSIM_NO_BROWSER=1 python notebooks/_build/nbrun.py
notebooks/rust_bouncing_ball_restitution.ipynb` then
`python notebooks/_build/nbcheck.py
notebooks/rust_bouncing_ball_restitution.ipynb`.)

Display the notebook in a web browser with JupyterLab (it opens the
page in your default browser itself; if no tab appears, run
`Start-Process <the URL it prints>` — Git Bash: `start <URL>`):

```powershell
python -m pip install --user jupyterlab
jupyter lab notebooks\rust_bouncing_ball_restitution.ipynb
```

(Git Bash: `jupyter lab notebooks/rust_bouncing_ball_restitution.ipynb`.)

If PowerShell reports that `jupyter` is not recognized, the `--user`
install put its launcher in a per-user `Scripts` folder that is not on
`PATH` by default (here
`C:\Users\<you>\AppData\Roaming\Python\Python314\Scripts` for
Python 3.14); the same server starts as a module without touching
`PATH`:

```powershell
python -m jupyterlab notebooks\rust_bouncing_ball_restitution.ipynb
```

(Git Bash: `python -m jupyterlab notebooks/rust_bouncing_ball_restitution.ipynb`.)

Display the same rebound live in this project's browser GUI — the
scene window opens in your default browser (posim launches it through
`cmd /C start`) with Start/Pause/Reverse/Reset and live E, P, L
readouts (the dynamic-notebook twin of this example; press Start in
the window):

```powershell
tools\posim_notebook.cmd bouncing_ball_restitution
```

(Git Bash: `tools/posim_notebook bouncing_ball_restitution`. Both are
the launcher form of `cargo run -p posim --release -- --notebook
dynamic_notebooks/bouncing_ball_restitution.posim`. If the window does
not appear, open the address the notebook prints:
`Start-Process http://127.0.0.1:<port>/` — Git Bash:
`start http://127.0.0.1:<port>/`.)

Verify byte-identity of the whole examples suite against the Linux
evidence (this example is one of the six it covers; the gate also
re-runs the 12 collision scripts and the 59 dynamic notebooks; exit 0 =
pass — measured here: `IDENTICAL-MODULO-DOCUMENTED` for the examples,
`IDENTICAL` for the collision scripts, `GATE_EXIT=0`,
`logs/win_verify_physics.log`). The gate is a bash script, run in Git
Bash:

```bash
bash tools/win_verify_physics.sh
```

(From PowerShell: `& "C:\Program Files\Git\bin\bash.exe" tools/win_verify_physics.sh`.)
