# SolveIt_Notebooks_for_rust — the notebook encyclopedia

**147 Jupyter notebooks** — 145/147 logged green so far on this Windows 11
machine (Intel Core Ultra 9 275HX, rustc/cargo 1.91.1 for
`x86_64-pc-windows-msvc`, Python 3.14.5); the two Mercury copies are being
re-executed after the UTF-8 driver fix and their verdicts are pending (see
`_tools/` below). The set: the **128 posim-family copies**
(38 SolveIt worked examples, 59 dynamics-and-Routh notebooks, 12 collision
walkthroughs, 13 mechanism-video notebooks, 6 compiled Rust examples), the
**17 rebound / reboundx N-body notebooks**, and the **2 planet-Mercury
notebooks** — organized into seven major topics, each in its own tagged
subfolder with a companion `player_<tag>.py`, plus the encyclopedia itself:

- `ENCYCLOPEDIA.pdf` / `ENCYCLOPEDIA.md` / `ENCYCLOPEDIA.html` — one entry
  per notebook: description, execution verdict, GUI image, and every
  pointer (source, paired scene, movie, database, player). All three were
  rebuilt on this machine by `_tools/encyclo.py` (the PDF is printed by
  headless Google Chrome) — the committed files are the builder's **dry
  run** from before the Mercury re-execution: the header line says
  "146/147 green" and `mercury_tidal_locking` reads "Executed: FAIL (4
  code cells)" (the stale verdict of the interrupted first copy run),
  while `mercury_test2_jupiter_gr` reads "PASS (17 code cells)" from the
  macOS-era `exec_mercury2.log` that arrived with the encyclopedia. The
  final rebuild after the Mercury verdicts replaces all three.
- `encyclopedia/images/` — **124 captured GUI screenshots**, one per
  GUI-family entry (38 solveit + 59 dynamic + 12 collision + 13 video +
  2 mercury); the 6 `rust_*` and the 17 rebound notebooks have textual
  verdicts and no browser GUI by design. Captured here by
  `_tools/shoot.py --workers 4` through headless Google Chrome: `124
  captured, 0 failed`, smallest image 18,039 bytes (`_tools/shoot.log`).
- `_tools/` — the deterministic pipeline: `gen.py` (copies + header cells
  + players), `run_copy.py` (family-aware executor), `shoot.py` (GUI
  capture), `encyclo.py` (encyclopedia builder), `verify.py` (six
  mechanical gates), `manifest.json` (the 147 entries), and the execution
  logs the encyclopedia's verdicts come from — `exec_posim_0..3.log`
  (the 128 posim-family copies: 38/38, 59/59, 25/25, 6/6 ok),
  `exec_rebound.log` (17/17 ok), `exec_mercury.log` (the
  `mercury_tidal_locking` copy) and `exec_mercury2.log` (the
  `mercury_test2_jupiter_gr` copy) — both 0 bytes while their re-runs are
  in progress, since `run_copy.py` prints its verdict at the end — plus
  `shoot.log` and the macOS-era leftover `exec_rebound_retry.log` (one
  verdict, `shearing_sheet` ok, agreeing with `exec_rebound.log`).
  `encyclo.py` reads `exec_*.log` in filename order and the last verdict
  seen for a tag wins, so a stale or foreign log that sorts later
  overrides a fresh one; delete or rename such logs before the final
  build. The `chunk_*.txt` files are the four-way split of the
  copy list that the parallel run on this machine used.
- `verify.py` on this tree (dry run, `logs/_bg_encyclo_dry.log`):
  `ALL VERIFY GATES PASS (147 notebooks)`; the final pass after the
  Mercury re-runs is pending.

Every notebook's **first cell** is its encyclopedia header (a markdown
cell tagged `encyclopedia-header`) with five mandatory sections:
`## 1. Execute this notebook`, `## 2. Display the browser GUI`,
`## 3. Movies`, `## 4. Database / data access`, and `## 5. The full
player` — the exact commands to execute it, display its browser GUI,
create/access its movies, access its database/data — and the full player
script inline, byte-identical to the shipped `player_<tag>.py` (gate V3
checks that identity). The header commands are PowerShell; every one also
runs unchanged in Git Bash except `Start-Process <file>` (use
`start <file>` there) and `$env:NAME = "value"` (use `export NAME=value`).

## Folder map

```
SolveIt_Notebooks_for_rust/
  01_planet_mercury_tidal_locking/    2 notebooks   (family: mercury)
  02_solveit_worked_examples/        38 notebooks   (family: solveit)
  03_dynamics_and_routh/             59 notebooks   (family: dynamic)
  04_collisions/                     12 notebooks   (family: collision)
  05_mechanism_videos/               13 notebooks   (family: video)
  06_rust_compiled_examples/          6 notebooks   (family: rust)
  07_nbody_rebound_rust/             17 notebooks   (family: rebound)
    <topic>/<tag>/<tag>.ipynb          the executed copy (header cell first)
    <topic>/<tag>/player_<tag>.py      its companion player
    <topic>/<tag>/datasets/            the SQLite dataset (+ plots page) a
                                       posim-family copy writes when run
  ENCYCLOPEDIA.md / .html / .pdf
  encyclopedia/images/<tag>.png       124 GUI captures
  _tools/                             gen.py run_copy.py shoot.py encyclo.py
                                      verify.py manifest.json exec_*.log
                                      shoot.log chunk_*.txt
```

Canonical sources (one edition of each notebook; duplicates elsewhere are
deliberately excluded): the 128 under the engine's `notebooks/`, the 17
under `rebound_rust/notebooks/`, and the 2 under
`planet_Mercury/notebook/`.

## Two layouts, one rule

The paths in the header cells and players resolve automatically in both
places this tree lives (`gen.py`, `run_copy.py`, `encyclo.py` and every
player probe for the engine folder under either platform edition's name,
`rustSolveIt_Win11_SUNDIALS_7_8_0` or
`rustSolveIt_macos-silicon_SUNDIALS_7_8_0`, and fall back to the clone
root):

1. **The workspace layout** — a folder containing
   `SolveIt_Notebooks_for_rust/`, `planet_Mercury/`, and the engine
   repository directory `rustSolveIt_Win11_SUNDIALS_7_8_0/`. Header
   commands generated in this layout run verbatim from that workspace
   root; the engine steps are wrapped as
   `cd rustSolveIt_Win11_SUNDIALS_7_8_0; <command>; cd ..` and engine
   paths carry the `rustSolveIt_Win11_SUNDIALS_7_8_0/` prefix.
2. **A fresh clone of the repository** — here the clone root IS the engine
   (it contains `posim/`, `notebooks/`, `planet_Mercury/`, and this
   folder). Run every command **from the clone root**; the one-time engine
   build is simply `cargo build --release -p posim` and every command that
   starts with `SolveIt_Notebooks_for_rust/...` runs verbatim.

The committed copies were generated and executed on this machine in the
clone layout, so their header cells say "from the **repository root** (the
cloned `rustSolveIt_Win11_SUNDIALS_7_8_0` folder)". To rewrite the headers
for the other layout without losing the executed outputs, run `gen.py`
with `--refresh` from that layout (see below).

## Reproduce everything

Prerequisites, all measured on this machine: the Rust toolchain (rustc /
cargo 1.91.1, MSVC linker from Visual Studio 2026 Build Tools; the root
`.cargo/config.toml` pins `-C target-feature=+fma`, so the CPU must have
FMA), Python 3 on `PATH` as `python` (3.14.5 here — the tools are
standard-library Python and write every file UTF-8 with LF endings
themselves, so the Windows cp1252 default encoding does not matter), and
Google Chrome (`C:\Program Files\Google\Chrome\Application\chrome.exe`;
Microsoft Edge is probed as a fallback) for GUI capture and the PDF. The
rebound family additionally needs `nbformat`, `nbclient` and `ipykernel`
(5.10.4 / 0.10.2 / 7.1.0 here, with `jupyter_client` 8.6.3) and
`matplotlib` for its `shearing_sheet` notebook — either in
`planet_Mercury\notebook\.venv` (`run_copy.py` uses that venv's
`Scripts\python.exe` when it exists) or in the `python` that runs
`run_copy.py`. No WSL2 compiler or linker is involved anywhere; the only C
compiled for this pipeline is the SQLite amalgamation inside `dataset_tools`
(MSVC `cl.exe` through the `cc` crate, found automatically).

```powershell
# from the repository root (Windows 11, PowerShell)

# one-time builds
cargo build --release -p posim                          # target\release\posim.exe
cd dataset_tools; cargo build --release; cd ..          # target\release\solveit_db.exe (SQLite dataset sidecar)
cd planet_Mercury/mercury_rs; cargo build --release; cd ../..   # planet_Mercury\mercury_rs\target\release\mercury_rs.exe
cd reboundx_rust; cargo build --release; cd ..          # the rebound family's engine
python -m pip install --user nbformat nbclient ipykernel matplotlib   # rebound family only

# regenerate the 147 copies, header cells, players and manifest.json
python SolveIt_Notebooks_for_rust/_tools/gen.py            # fresh, unexecuted copies from the canonical sources
python SolveIt_Notebooks_for_rust/_tools/gen.py --refresh  # rebuild headers + players INSIDE the executed copies, keeping every output

# execute one notebook copy (writes its real outputs back into it) — any copy's path
python SolveIt_Notebooks_for_rust/_tools/run_copy.py SolveIt_Notebooks_for_rust/02_solveit_worked_examples/solveit_01_elastic_head_on/solveit_01_elastic_head_on.ipynb

# execute every copy in one go, logging where encyclo.py reads verdicts (_tools/exec_*.log)
$copies = Get-ChildItem SolveIt_Notebooks_for_rust -Recurse -Filter *.ipynb |
          Where-Object FullName -notmatch 'ipynb_checkpoints' | ForEach-Object FullName
python SolveIt_Notebooks_for_rust/_tools/run_copy.py $copies 2>&1 | Tee-Object SolveIt_Notebooks_for_rust/_tools/exec_all.log

# per-notebook player: re-run, open GUI, jump to the key event, capture, export
# (shown for solveit_01_elastic_head_on; every copy has its own player_<tag>.py beside it)
$p = "SolveIt_Notebooks_for_rust/02_solveit_worked_examples/solveit_01_elastic_head_on/player_solveit_01_elastic_head_on.py"
python $p run
python $p gui
python $p jump
python $p capture out.png
python $p data exported_data

# rebuild the whole encyclopedia
python SolveIt_Notebooks_for_rust/_tools/shoot.py --workers 4 2>&1 | Tee-Object SolveIt_Notebooks_for_rust/_tools/shoot.log
python SolveIt_Notebooks_for_rust/_tools/encyclo.py
python SolveIt_Notebooks_for_rust/_tools/verify.py
```

(Git Bash: the same lines, except the all-copies step is
`python SolveIt_Notebooks_for_rust/_tools/run_copy.py $(find SolveIt_Notebooks_for_rust -name '*.ipynb' -not -path '*ipynb_checkpoints*') 2>&1 | tee SolveIt_Notebooks_for_rust/_tools/exec_all.log`
and `Tee-Object` becomes `tee`.)

What each step does and what to expect:

- `run_copy.py` executes every non-interactive code cell of a copy in one
  shared namespace (the engine's own miniature-kernel discipline), and
  writes the outputs back **only if every cell succeeded**. It sets the
  environment itself: posim families get `POSIM_BIN` (pointing at
  `target\release\posim.exe`) and `POSIM_NO_BROWSER=1`; the Mercury pair
  gets `MERCURY_NO_BROWSER=1` and runs in `planet_Mercury\notebook`; the
  rebound family runs through `nbclient` with the kernel's working
  directory set to `rebound_rust\notebooks`. It accepts any number of
  copies in one call and prints one `ok  <copy>  (N cells)` or
  `FAIL <copy>  (N cells)` line each, then `<n> ok, <m> failed`; exit 0
  only if every copy passed. On this machine: 38/38, 59/59, 25/25, 6/6
  (posim families), 17/17 (rebound), and the Mercury pair. `encyclo.py`
  reads every `_tools/exec_*.log` in filename order and the last verdict
  line seen for a tag wins — so log a re-run under a name that sorts after
  the stale one, or delete the stale log.
- The player modes: `run` re-executes the copy through `run_copy.py`;
  `gui` opens the browser GUI (posim families: the paired `.posim` scene
  is replayed into a fresh `posim.exe --machine` child and its live scene
  window opens; video family: the live `gui/<name>/server.py`; Mercury:
  the baked player page); `jump` opens it jumped to the key event
  (Mercury: the 3:2 capture moment via the page's own `bt-cap` button);
  `capture out.png` renders the GUI headlessly with Chrome/Edge
  (`--headless=new --window-size=1500,950 --screenshot`); `data outdir`
  exports every SQLite table to CSV plus the executed transcript (and
  copies the movie page where one exists). The player never computes
  physics — it drives and displays the recorded, verified artifacts.
- `shoot.py` calls each GUI-family player's `capture` in parallel and
  records the image path in `manifest.json` (`image: null` for rust and
  rebound entries). Expected: `124 captured, 0 failed`.
- `encyclo.py` reads `manifest.json`, the executed copies (for each
  notebook's own opening paragraph), the exec logs (verdicts) and the
  images; writes `ENCYCLOPEDIA.md`, `ENCYCLOPEDIA.html` and, through
  headless Chrome (`--headless=new --print-to-pdf --no-pdf-header-footer`),
  `ENCYCLOPEDIA.pdf`; prints the three sizes. Appendix C of the
  encyclopedia repeats these reproduction commands in whichever layout it
  was built in.
- `verify.py` runs the six read-only gates: V1 the manifest has 147
  entries with topic counts 2/38/59/12/13/6/17; V2 every copy exists,
  parses, and its first cell is the tagged header with all five sections;
  V3 every player exists, compiles, and equals the header-embedded script;
  V4 every code cell of every copy carries an execution count
  (interactive-tagged cells exempt); V5 every GUI-family entry has a PNG
  above 10,000 bytes and rust/rebound have none; V6 `ENCYCLOPEDIA.md`
  names every tag exactly once. Expected: `ALL VERIFY GATES PASS (147
  notebooks)`.

Interactive execution of any copy (Shift+Enter through the cells, Python 3
(ipykernel) kernel) needs only JupyterLab:

```powershell
python -m pip install --user jupyterlab    # once
python -m jupyterlab SolveIt_Notebooks_for_rust/02_solveit_worked_examples/solveit_01_elastic_head_on/solveit_01_elastic_head_on.ipynb   # any copy
```

(`python -m jupyterlab` rather than `jupyter lab`: after a `--user`
install the `jupyter` launcher is usually not on PATH on Windows.)

(The Mercury pair's header uses a venv instead:
`python -m venv planet_Mercury/notebook/.venv`, then
`planet_Mercury/notebook/.venv/Scripts/pip install jupyterlab`, then
`../../../planet_Mercury/notebook/.venv/Scripts/jupyter lab <tag>.ipynb`
from the copy's folder.) The posim-family notebooks find
`target\release\posim.exe` themselves; if needed, set
`$env:POSIM_BIN = "$PWD/target/release/posim.exe"` (Git Bash:
`export POSIM_BIN="$PWD/target/release/posim.exe"`) before starting
JupyterLab.

## Windows notes

- Binaries carry `.exe`: the tools probe `posim.exe`, `solveit_db.exe`
  and `mercury_rs.exe` before the extensionless names; everything else
  is platform-neutral Python.
- The three rebound notebooks whose prose describes a libm measurement
  (`libm_diff`, `bs_pow_diff`, `integrators_test`) carry an editorial
  note in the encyclopedia: the run embedded in each copy is this
  repository's own Windows 11 / MSVC (UCRT libm) run, and the verdict
  cell at the end of the copy is the measurement of record here (on this
  machine only `pow` differs from the C reference, by exactly 1 ULP in
  60 of 200,000 samples; the macOS edition recorded no differences with
  Apple libm).
- A SQLite `-wal` / `-shm` pair beside a dataset is transient
  write-ahead-log state (the sidecar checkpoints on close), never a
  dataset; the repository ignores them.
- Screen recording, where a header suggests it: Win+Alt+R starts the Xbox
  Game Bar recorder (or use the Snipping Tool's Record mode).

## Regenerable artifacts: what is tracked and what is not

Every posim-family copy folder carries the `datasets/` the notebook wrote
on this machine (its SQLite dataset `nb_<tag>.db`, or the dataset named in
the paired script, and the `_plots.html` page); these are tracked. Two
notebooks additionally bake large browser pages beside themselves when
executed — `dynamic_double_slit/double_slit.html` (51,088,049 bytes) and
`dynamic_tunneling/scatter.html` (860,345 bytes). Both are tracked in the
repository as well (they arrived tracked from the macOS edition, whose
`scatter.html` is byte-identical to the one baked here; `double_slit.html`
differs, the pinned last-digit divergence of the `double_slit` quantum
notebook on this platform's libm). One run of each notebook recreates
them, so nothing depends on the committed copies.

Not committed: the Mercury notebooks' SQLite databases and CSV runs under
`planet_Mercury\data\` (ignored; regenerated by one execution, documented
in `planet_Mercury/INSTRUCTIONS_FOR_TEACHERS_AND_STUDENTS.md`) — their
baked player pages under `planet_Mercury\gui\` are tracked; the
`planet_Mercury\notebook\.venv` folder; SQLite `-wal`/`-shm` side files;
the `__pycache__` folders the players leave behind; and JupyterLab's
`.ipynb_checkpoints`.
