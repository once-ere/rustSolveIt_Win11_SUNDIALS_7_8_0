# posim ↔ JupyterLab bridge

Lets JupyterLab (or any Jupyter front end) drive the pure-Rust
simulator: notebook cells are posim command-language lines, executed by
the `posim` binary through its `--machine` JSON protocol.

```
JupyterLab ⇄ (ZMQ) ⇄ posim_kernel (Python wrapper kernel)
                        ⇄ (stdin/stdout JSONL) ⇄ posim --machine (Rust)
```

The Rust side stays zero-dependency; the Python side is a thin
~100-line wrapper kernel built on `ipykernel`.

## Setup (Windows 11, PowerShell)

```powershell
# 1. build the simulator (from the repository root)
cd ..
cargo build --release
cd jupyter

# 2. python env for the kernel (any env with ipykernel works)
uv venv .venv
uv pip install -p .venv\Scripts\python.exe ipykernel jupyterlab

# 3. register the kernelspec (user-wide)
.venv\Scripts\jupyter.exe kernelspec install --user kernelspec --name posim
#    (edit kernelspec/kernel.json "argv" to point at your python if it
#     is not `python` on PATH; set POSIM_BIN if the binary is elsewhere)

# 4. go
$env:PYTHONPATH = "$PWD"
.venv\Scripts\jupyter.exe lab
```

(On Linux/macOS the venv paths are `.venv/bin/python` and
`.venv/bin/jupyter`, and the last step is
`PYTHONPATH=$PWD .venv/bin/jupyter lab`.)

Pick the **posim (physical_object)** tile in the launcher. Each cell is
one or more command lines (`HELP` lists them); shift-enter executes,
and JupyterLab supplies cell history/editing:

```
new sphere { mass = 2, radius = 0.5, velocity = [1, 0, 0] }
set system.gravity = [0, -9.81, 0]
step 1
get obj0.position
```

## Tests (no JupyterLab needed)

- `python test_protocol.py` — drives `posim --machine` directly over
  the JSONL protocol (stdlib only).
- `.venv\Scripts\python.exe test_kernel.py` — full kernel test over the
  Jupyter ZMQ messaging protocol via `jupyter_client` (what JupyterLab
  itself uses), including an analytic-solution check. It needs
  `jupyter_client` in the venv:
  `uv pip install -p .venv\Scripts\python.exe ipykernel jupyter_client`.

Machine-mode ops (one JSON doc per line):
`{"op":"exec","code":"..."}`, `{"op":"get","path":"obj0.position"}`,
`{"op":"set","path":"obj0.mass","value":2.5}`, `{"op":"state"}`,
`{"op":"events"}` (drain queued scene-window events),
`{"op":"help"}`, `{"op":"quit"}`.
