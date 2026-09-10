# CLAUDE.md — rustSolveIt on pure-Rust SUNDIALS 7.8.0 (Windows 11)

Pure-Rust physics simulator, the **Windows 11 / x86-64** port of
`once-ere/rustSolveIt_macos-silicon_SUNDIALS_7_8_0` (macOS / Apple
Silicon), itself the port of `once-ere/rustSolveIt_Using_SUNDIALS_7_8_0`
(Linux). Measured on Windows 11 Pro for Workstations, build 10.0.26200,
rustc 1.91.1 / cargo 1.91.1, `x86_64-pc-windows-msvc`; the MSVC linker
(Visual Studio 2026 Build Tools, MSVC 14.51.36231) is located by rustc
itself — no developer prompt needed; no WSL2 compiler or linker is used
anywhere. The **only** numerical-integration backend is the vendored
`sundials_rs/` workspace (pure-Rust **SUNDIALS 7.8.0**) — no exceptions.
Treat `sundials_rs/` as a **read-only vendored library**: it is the
crate tree of `once-ere/SUNDIALS_7_8_Rust_port_for_Windows11` (its
`.git`, `target/` and bulk `logs/` corpora excluded; it differs from the
Linux/macOS engine only in its backend module sets — the public API is
identical) and that identity is a verifiable property — never edit it
in place; upstream it instead. Its deterministic `sundials_libm` needs
real FMA instructions: `.cargo/config.toml` at THIS root pins
`-C target-feature=+fma` — do not remove it, and remember cargo config
does not flow down from the vendored workspace's own `.cargo/`. The CPU
must have FMA (Intel Haswell 2013+, AMD Piledriver 2012+).

```bash
# the vendored-tree identity claim, reproducible in Git Bash
git clone https://github.com/once-ere/SUNDIALS_7_8_Rust_port_for_Windows11.git
diff -rq --exclude=.git --exclude=target --exclude=logs \
     SUNDIALS_7_8_Rust_port_for_Windows11 sundials_rs      # silent
```

What the Windows port changed, and the evidence that the physics did
not move (byte-identity gates against the Linux evidence), is in
`PORT_WIN11_PROVENANCE.md`; per-suite verification on this machine:
`VERIFICATION_WIN11.md`; the closing statement: `WINDOWS_PORT_COMPLETE.md`.
The 7.7.0→7.8.0 engine upgrade is `PORT_7.8.0_PROVENANCE.md`; the
original export's provenance — including which donor sources were
deliberately not carried over — is in `EXPORT_PROVENANCE.md` and
`PLAN.md`.

**Lineage.** This repository was produced by porting the macOS project
onto the Windows engine and re-platforming its documentation. The macOS
project's provenance pages — `PORT_MACOS_PROVENANCE.md`,
`VERIFICATION_MACOS.md`, `MACOS_PORT_COMPLETE.md`,
`REBOUND_REBOUNDX_MACOS_PROVENANCE.md` — are kept **verbatim** under
lineage banners; `planet_Mercury/plan/*` is kept verbatim as the
macOS-era import record, and `PLANET_MERCURY_PROVENANCE.md` keeps its
§1–§4 import record macOS-era while its dated §7 "Windows 11
re-verification" section carries this machine's figures side by side
with the macOS ones (the plan pages' numbers are macOS/Linux
measurements, never Windows ones)
(the older engine-lineage pages `PORT_7.8.0_PROVENANCE.md`,
`EXPORT_PROVENANCE.md`, `CLEANROOM_PROVENANCE.md` and
`REFINE_PROVENANCE.md` are likewise untouched). Every
Windows number lives in `PORT_WIN11_PROVENANCE.md`,
`VERIFICATION_WIN11.md`, `WINDOWS_PORT_COMPLETE.md` and the logs under
`logs/` and `evidence/win11/`. Never edit a lineage page.

## Commands (PowerShell; Git Bash equivalents in parentheses)

Run everything from the repository root. `logs\` is where this port
keeps its captured runs; `Tee-Object` is PowerShell's `tee` (Git Bash:
`2>&1 | tee logs/<name>.log`). Open any page with
`Start-Process <file>` (Git Bash: `start <file>`).

- Build: `cargo build --workspace --all-targets 2>&1 | Tee-Object logs\build.log`
  (0 errors, 0 warnings); release:
  `cargo build --release --workspace --all-targets 2>&1 | Tee-Object logs\build_release.log`
  → `target\release\posim.exe` (the examples land in
  `target\release\examples\*.exe`)
- Tests: `cargo test --workspace 2>&1 | Tee-Object logs\test.log`
  (**638 expected**: 49 physical_object lib + 19 collision +
  9 conservation + 42 constrained/DAE + 121 posim + 92 quantum +
  240 special_functions (incl. the 7 coordinate-map suites) +
  11 vendored identities + 5 quantum doctests +
  50 special_functions doctests)
- Notebook: `cargo run` (type `HELP`); batch: `cargo run -p posim -- --script <f>`
- Dynamic notebook (loads a file, opens its scene window, stays
  interactive): `cargo run -p posim --release -- --notebook
  dynamic_notebooks\<name>.posim`, or `tools\posim_notebook.cmd <name>`
  (`tools/posim_notebook` from Git Bash; `--list` names all 59) — see
  `dynamic_notebooks/README.md`
- Scene window: type `SCENE CREATE` in the notebook (opens a browser
  page via `cmd /C start`; `SCENE START/PAUSE/REVERSE/RESET`,
  arrows/drag/wheel in the window). Headless runs:
  `$env:POSIM_NO_BROWSER = "1"` suppresses the browser launch (Git
  Bash: `POSIM_NO_BROWSER=1 <command>`).
- Self-checking physics examples:
  `cargo run -p physical_object --release --example
  {kepler_orbit|outer_solar_system|tumbling_body|charged_in_b_field|newtons_cradle|bouncing_ball_restitution}`
  (each prints SUCCESS/FAILURE and exits nonzero on failure)
- Collision example scripts: `cargo run -p posim -- --script
  scripts\collisions\NN_name.posim` (01–12; documented with captured
  output in `collision_detection.md` §9)
- The thirty-eight SolveIt examples: `cargo run -p posim --release --
  --script scripts\solveit\NN_name.posim` (01–38; documented with
  captured output in `SolveIt.md` §7; 20–38 are the dataset/SQL/movie
  examples — 36–38 add DATASET DIFF/SUM/MAP and the fourteen
  curvilinear coordinate systems — and need the sidecar built first).
  All 38 + the 12 collision scripts headless, one log each (measured
  50/50 exit 0, `logs\solveit_and_collision_scripts.log`, per-script
  logs in `logs\solveit_scripts\`):

  ```powershell
  $env:POSIM_NO_BROWSER = "1"
  New-Item -ItemType Directory -Force logs\solveit_scripts | Out-Null
  Get-ChildItem scripts\solveit\*.posim | ForEach-Object {
      .\target\release\posim.exe --script $_.FullName 2>&1 |
          Tee-Object "logs\solveit_scripts\solveit_$($_.BaseName).log" }
  Get-ChildItem scripts\collisions\*.posim | ForEach-Object {
      .\target\release\posim.exe --script $_.FullName 2>&1 |
          Tee-Object "logs\solveit_scripts\collisions_$($_.BaseName).log" }
  ```

  (Git Bash: `for f in scripts/solveit/*.posim; do POSIM_NO_BROWSER=1
  target/release/posim.exe --script "$f" 2>&1 | tee
  "logs/solveit_scripts/solveit_$(basename "$f" .posim).log"; done`)
- Dataset sidecar (REQUIRED for DATASET/SQL/MOVIE commands, the
  solveit 20–38 scripts and all notebooks):
  `cd dataset_tools; cargo build --release; cd ..` (Git Bash:
  `cd dataset_tools && cargo build --release && cd ..`). The first
  build fetches rusqlite/r2d2/r2d2_sqlite from crates.io (`Cargo.lock`
  pins rusqlite 0.32.1, r2d2_sqlite 0.25.0) and compiles the bundled
  SQLite amalgamation with MSVC `cl.exe` through the `cc` crate — the
  only C the simulator's own build ever compiles (the simulator
  workspace is pure Rust; the sole other C built here is the
  REBOUND/REBOUNDx reference harnesses the rebound gates compile, also
  with `cl.exe`). Produces `dataset_tools\target\release\solveit_db.exe`.
  Its own tests: `cd dataset_tools; cargo test; cd ..` (11 = 8 unit +
  3 integration: WAL + busy-timeout, 10-thread pool stress, two-pool
  cross-process interleave, serve-protocol round trip;
  `logs\dataset_tools_test.log`). Demo:
  `.\dataset_tools\target\release\solveit_db.exe stress $env:TEMP\s.db 10 500`
  (Git Bash: `dataset_tools/target/release/solveit_db.exe stress /tmp/s.db 10 500`).
  Read any dataset from outside posim with Python's bundled SQLite
  shell: `python -m sqlite3 datasets\<name>.db "SELECT COUNT(*) FROM frames"`
- Datasets/movies from the language: `DATASET CREATE <name>` (arms
  recording), `RUN`/`STEP` record frames, `SQL "<any SQLite>"`,
  `DATASET PLAY` (plots page in the browser), `MOVIE CREATE <dataset>`
  + `MOVIE PLAY <name>` (3-D player from the recording). Files land in
  `datasets\` and `movies\`; `$env:POSIM_NO_BROWSER = "1"` suppresses
  browsers. Full reference: `grammar.md` §5.14; worked session: §13
- Browser video: `cargo build --release -p posim` then
  `python recorder\src\record_video.py videos\scenes\<x>.posim -o videos\<x>.html
  --frames N --dt DT --title "..." --caption "..."`; byte-identity of
  all 13 committed recordings: `python recorder\src\record_all.py --check`
  (all 13 reproduce byte for byte here, `logs\videos_check.log`);
  recorder tests: `python recorder\tests\test_units.py` (17) and
  `python recorder\tests\test_end_to_end.py` (9)
- The Windows physics byte-identity gate (Git Bash; needs the release
  build above): `bash tools/win_verify_physics.sh 2>&1 | tee
  logs/win_verify_physics.log` — re-runs the 6 examples, 12 collision
  scripts and 59 dynamic notebooks, writes
  `evidence/win11/{examples,collision-scripts,dynamic-notebooks}-win11.log`
  and diffs against `evidence/port-7.8.0/`; only the divergences pinned
  in `evidence/win11/accepted-divergences-*.diff` are allowed (measured:
  exit 0 — examples IDENTICAL-MODULO-DOCUMENTED, collision scripts
  IDENTICAL, dynamic notebooks IDENTICAL-MODULO-DOCUMENTED)
- GUI smoke test: `python tools\win_gui_smoke.py` (drives all 13
  `gui\*\server.py` over their HTTP API on ports 8895–8907; 13 of 13
  pass); `python tools\gui_smoke.py` is the POSIX twin and also passes
  13 of 13 here (`logs\gui_smoke.log`). One GUI by hand:
  `python gui\<name>\server.py`, then open the printed URL
- Wire protocol test: `python jupyter\test_protocol.py` (needs
  `cargo build --release` first — it prefers `target\release\posim.exe`,
  so a stale release binary silently shadows your debug build)
- Kernel test: `$env:POSIM_NO_BROWSER = "1";
  jupyter\.venv\Scripts\python.exe jupyter\test_kernel.py`
  (needs `uv venv jupyter\.venv` then `uv pip install -p
  jupyter\.venv\Scripts\python.exe ipykernel jupyter_client`; 7/7 cells
  ok, `logs\jupyter_tests.log`)
- Notebooks (the 128 in `notebooks\`; sidecar required): regenerate
  `python notebooks\_build\regen.py`; execute all 128 headless

  ```powershell
  $env:POSIM_NO_BROWSER = "1"
  Get-ChildItem notebooks\*.ipynb | ForEach-Object { python notebooks\_build\nbrun.py $_.FullName }
  ```

  (Git Bash, six at a time: `ls notebooks/*.ipynb | POSIM_NO_BROWSER=1
  xargs -P 6 -I{} python notebooks/_build/nbrun.py {}`); check
  `python notebooks\_build\nbcheck.py` (no arguments = all 128; 128/128
  must pass all eight requirements, `logs\notebooks_run.log`)
- Index of entities: `python tools\verify_index_examples.py`
  (PASS 1365/1365, `logs\verify_index_examples.log`);
  `python tools\verify_tierb_examples.py` compiles the Tier-B doc
  snippets (regenerates the gitignored scratch file
  `posim\examples\_tierb_compile_check.rs` on every run)
- The REBOUND / REBOUNDx ports (separate workspaces, see Layout):
  `cd rebound_rust; cargo test --release; cd ..` (394 tests) and
  `cd reboundx_rust; cargo test --release; cd ..` (137 tests, 2 doctests
  ignored); both clean under `cargo clippy --release --all-targets`
  (`rebound_rust\logs\win11_{build,test,clippy}_20260909.log`,
  `logs\reboundx_*.log`). Their 17 notebooks:
  `python rebound_rust\notebooks\make_notebooks.py` then
  `$env:PYTHONIOENCODING = "utf-8"; python rebound_rust\notebooks\run_notebooks.py`
  (Git Bash: `PYTHONIOENCODING=utf-8 python …`; needs nbformat +
  nbclient + ipykernel; "All 17 notebooks executed with no errors",
  `rebound_rust\logs\win11_notebooks_20260909.log`)
- planet_Mercury (separate crates, see Layout):
  `cd planet_Mercury\mercury_rs; cargo build --release 2>&1 | Tee-Object ..\..\logs\mercury_rs_build.log;
  cargo test --release 2>&1 | Tee-Object ..\..\logs\mercury_rs_test.log; cd ..\..`
  (0 warnings; 19 passed; `target\release\mercury_rs.exe`);
  `cargo run --release -- print-config` prints the spec constants. The
  GPL cross-check crate:
  `cd planet_Mercury\mercury_crosscheck; cargo run --release; cd ..\..`
  (links the rebound ports; prints the despin-rate agreement and
  SUCCESS — an upstream REBOUNDx notice about a 4.5.0 tidal-potential
  fix also prints and is expected). The two notebooks, headless:

  ```powershell
  cd planet_Mercury\notebook
  $env:MERCURY_NO_BROWSER = "1"
  python run_notebook.py mercury_tidal_locking.ipynb
  python check_notebook.py mercury_tidal_locking.ipynb
  python run_notebook.py mercury_test2_jupiter_gr.ipynb
  python check_notebook.py mercury_test2_jupiter_gr.ipynb
  cd ..\..
  ```

  (Git Bash: `MERCURY_NO_BROWSER=1 python run_notebook.py
  mercury_tidal_locking.ipynb`). They regenerate `planet_Mercury\data\`
  (~270 MB, gitignored) and the player page; re-bake it by hand with
  `cd planet_Mercury; python gui\bake_page.py; cd ..`, open it with
  `Start-Process planet_Mercury\gui\mercury_orbit.html`. Optional
  JupyterLab: `python -m venv planet_Mercury\notebook\.venv;
  planet_Mercury\notebook\.venv\Scripts\pip.exe install jupyterlab`
- The notebook encyclopedia (`SolveIt_Notebooks_for_rust\`, 147 copies
  = 128 notebooks + 17 rebound + 2 Mercury; needs the posim release
  build, the sidecar and `mercury_rs`; Chrome at
  `C:\Program Files\Google\Chrome\Application\chrome.exe` for the
  captures and the PDF):
  `python SolveIt_Notebooks_for_rust\_tools\gen.py` (copies + header
  cells + players) →
  `python SolveIt_Notebooks_for_rust\_tools\run_copy.py SolveIt_Notebooks_for_rust\<topic>\<tag>\<tag>.ipynb`
  (one or many; family-aware executor) →
  `python SolveIt_Notebooks_for_rust\_tools\shoot.py` (GUI PNGs) →
  `python SolveIt_Notebooks_for_rust\_tools\encyclo.py`
  (ENCYCLOPEDIA.md/.html/.pdf) →
  `python SolveIt_Notebooks_for_rust\_tools\verify.py` (six mechanical
  gates; exit 0 only if all pass). Per-notebook player:
  `python SolveIt_Notebooks_for_rust\<topic>\<tag>\player_<tag>.py {gui|jump|capture out.png|data exported}`
- Docs: `pdflatex -interaction=nonstopmode <name>.tex` (MiKTeX; run
  **twice** — the table of contents needs the second pass) for
  `SolveIt`, `grammar`, `physical_object_simulator`, `scene_info`,
  `collision_detection`. Keep `.md` and `.tex` in sync — **the `.md` is
  the source of truth.**

## Layout

- `physical_object/` — library: `constrain` (rigid rods + the constraint
  Jacobian), `equilibrium` (KINSOL rest states), `sensitivity` (CVODES /
  IDAS derivatives), `linalg` (Vec3/Mat3/Quat + skew/outer),
  `boundary` (enum + `Sdf` trait; **every round shape is symmetric
  about its local z axis**), `physical_object` (the union struct,
  get/set), `system` (collection + 13N pack/unpack), `integrate`
  (**all** time integration: CVODE Adams/BDF + ARKODE SPRK, with
  collision event rootfinding armed when collidable pairs exist),
  `collide` (contact geometry + impulse response; conventions pinned in
  ARCHITECTURE §3.8 — normal points i→j, separation > 0 apart and IS
  the root function).
- `posim/` — binary: `lexer` → `parser` (EBNF in its header) → `vm`
  (stack machine) → `notebook` (REPL) / `machine` (JSONL) / `scene`
  (graphical window: `mod.rs` server+playback, `ws.rs` hand-rolled
  SHA-1/base64/RFC 6455, `scene.html` embedded page); `qm`/`qm2`/`qm3`
  and `special` are the quantum and special-function command families;
  `dataset` (DATASET/SQL families + the plots page + the solveit_db
  handle) and `movie` (dataset-derived 3-D player pages) are the SQLite
  recording layer (ARCHITECTURE §3.10).
- `dataset_tools/` — the `solveit_db` SQLite sidecar: the ONE package
  with external crates (rusqlite bundled + r2d2 + r2d2_sqlite, per the
  feature's specification), EXCLUDED from the workspace like
  `sundials_rs`. WAL journal mode + busy_timeout=5000 on every pooled
  connection; posim spawns it, never links it. On Windows the binary is
  `solveit_db.exe` and posim's finder (`posim/src/dataset.rs`,
  `find_db_tool`) probes `$env:POSIM_DB_TOOL`, then
  `dataset_tools\target\release\solveit_db.exe` (before the
  extensionless name) from the working directory upward, then `PATH`.
- `datasets/`, `movies/` — the recorded SQLite datasets (one per
  notebook and per dataset example, plus `_plots.html` pages) and the
  movies derived from them. After all runs here: movies 74/74
  byte-identical to the macOS files; datasets 245/252 — the 7 that
  differ are the curvilinear-coordinate maps of SolveIt 37–38 (see
  Windows platform notes).
- `quantum/`, `special_functions/`, `vendor/spec_math/` — the physics
  and mathematics libraries the language reaches
  (`special_functions/src/coords.rs` holds the fourteen curvilinear
  coordinate maps).
- `jupyter/` — Python wrapper kernel (outside the Rust constraints);
  a reader thread streams async `{"event":...}` lines to the notebook;
  `.venv/` and `.kernels/` are gitignored scratch.
- `tools/` — the index pipeline, the example verifiers, the Windows
  gates (`win_verify_physics.sh`, `win_gui_smoke.py`; `gui_smoke.py`
  and `macos_verify_physics.sh` are their macOS twins, kept for
  lineage), the `posim_notebook.cmd` launcher, and the video recorder
  in `recorder/` (also outside the Rust constraints).
- `videos/`, `videos/scenes/` — recorded browser videos and the posim
  scripts that produced them.
- `gui/` — thirteen live GUI web pages, one per recorded video scene:
  a stdlib-Python server owning a `posim --machine` child plus a
  vanilla-JS canvas page with live physics readouts (`gui/README.md`).
- `notebooks/` — one executed Jupyter notebook per example, 128 in all
  (13 video scenes, 6 Rust examples, 12 collision scripts, 38 SolveIt
  scripts, 59 dynamic notebooks). **Every notebook owns a SQLite
  dataset** (section 6 creates it, section 9 reads it back with SQL and
  plain-Python sqlite3, section 10 derives the movie); nbcheck enforces
  this as requirement R8. Python notebooks driving
  `posim --machine`; stand-alone by design — every explanation is
  repeated in full in every notebook. Machinery and re-verification
  loop in `notebooks/_build/` (see `notebooks/README.md`).
- `SolveIt_Notebooks_for_rust/` — the notebook encyclopedia: the 147
  notebook copies in seven tagged topic folders, each with its
  `player_<tag>.py`, the `_tools/` pipeline (`gen.py`, `run_copy.py`,
  `shoot.py`, `encyclo.py`, `verify.py`, `manifest.json` and the
  execution logs), and `ENCYCLOPEDIA.md/.html/.pdf` with one entry per
  notebook. Its tools resolve the engine root automatically in both a
  fresh clone (this root) and the two-folder workspace layout.
- `rebound_rust/`, `reboundx_rust/` — two more pure-Rust ports carried
  here: `rebound_rs` 5.1.1 (REBOUND) and `reboundx_rs` 5.1.0 (REBOUNDx).
  **Not dependencies of the simulator** and **excluded from this
  workspace** (`Cargo.toml` `exclude`), so `cargo test --workspace` is
  still exactly the 638 tests above. They sit side by side at this root
  because `reboundx_rs` depends on `rebound_rs` through the relative
  path `../rebound_rust`, and because that layout is what their own
  documentation's absolute paths describe — a clone of this repository
  reproduces it. Build and test them separately (394 and 137 tests,
  commands above). `rebound_rust/` is a copy of the
  `once-ere/rebound_rust` repository's tracked tree — treat it as
  read-only here and push changes there. Full provenance of both ports,
  every command, and usage instructions: `rebound_rust/rebound_rust.md`
  (typeset as `rebound_rust.pdf`); the macOS lineage page is
  `REBOUND_REBOUNDX_MACOS_PROVENANCE.md`. The upstream C reference
  trees (`rebound/`, `reboundx/`, GPL) are gitignored, reference-only;
  the C harnesses the rebound gates compile are the ONLY other C built
  here (MSVC `cl.exe`, like the sidecar's SQLite).
- `planet_Mercury/` — the tidal-despinning / 3:2 spin-orbit capture
  sub-project, EXCLUDED from the workspace: `mercury_rs/` (pure Rust,
  zero external deps, path-depends on the vendored `sundials_core` +
  `cvode_rs`; the five-state Hut + Goldreich–Peale model), the GPL
  `mercury_crosscheck/` (links `rebound_rs`/`reboundx_rs`), `notebook/`
  (`mercury_tidal_locking.ipynb`, `mercury_test2_jupiter_gr.ipynb`, their
  deterministic builder/runner/auditor), `gui/` (`bake_page.py` → the
  self-contained `mercury_orbit.html` player; `bake_page2.py` →
  `mercury_test2.html`), `plan/` and
  `SOURCE_SPECIFICATION.md` (lineage, verbatim), and
  `INSTRUCTIONS_FOR_TEACHERS_AND_STUDENTS.md`. `data/` (~270 MB,
  regenerated by one notebook run) and `notebook/.venv/` are
  gitignored from the root `.gitignore` — the sub-project carries none.
  Import provenance: `PLANET_MERCURY_PROVENANCE.md` (its §1–§4 import
  record stays macOS-era — the §3 measurements are Apple Silicon ones —
  and its dated §7 "Windows 11 re-verification" section records this
  machine's figures beside them).
- `evidence/port-7.8.0/` — the logs behind every claim in
  `PORT_7.8.0_PROVENANCE.md` (the Linux reference outputs the Windows
  gate diffs against); `evidence/win11/` — this port's gate outputs and
  the two pinned accepted-divergence diffs; `evidence/macos/` — the
  macOS port's equivalents, kept for lineage.
- `logs/` — the captured Windows runs behind `VERIFICATION_WIN11.md`
  (build, test, sidecar, scripts, notebooks, videos, GUI, Jupyter,
  index, gates).
- `ARCHITECTURE.md` — pinned cross-module contracts (state layout,
  solver driving §3.7, setter invariants, wire protocol, scene
  subsystem, collision subsystem, the video recorder §7, the dataset
  layer §3.10). **Read it before touching anything cross-module; update
  it when a contract moves.**
- `SolveIt.md`/`.pdf` — the complete solution guide (38 worked
  examples); `grammar.md`/`.pdf` — the command-language spec (18 more);
  `scene_info.md`/`.pdf`; `collision_detection.md`/`.pdf`;
  `physical_object_simulator.md`/`.pdf` (the predecessor guide, 14
  more).
- `.backups/` — pre-modification file backups (gitignored). Back up
  before modifying; this repo's git history is the real undo.

## Hard rules

1. **Sundials-only integration.** All stepping goes through
   `physical_object/src/integrate.rs` calling `cvode_rs`/`arkode_rs`.
   Never add a hand-rolled Euler/Verlet/RK stepper anywhere — including
   in examples, docs, tooling (`recorder/` drives `step`;
   it does not integrate), and the scene playback thread (it calls
   `integrate::step`; reverse is snapshot replay from the history ring,
   never negative-dt integration).
2. **Zero `unsafe`, zero external dependencies, zero warnings.** Every
   crate root carries `#![forbid(unsafe_code)]`, `#![deny(warnings)]`,
   and allows `non_snake_case`, `non_camel_case_types`,
   `non_upper_case_globals`. `Cargo.lock` must list **only local
   crates**; today: `arkode_rs 7.8.0`, `cvode_rs 7.8.0`,
   `cvodes_rs 7.8.0`, `ida_rs 7.8.0`, `idas_rs 7.8.0`,
   `kinsol_rs 7.8.0`, `sundials_core 7.8.0`, `physical_object 0.1.0`,
   `posim 0.1.0`, `quantum 0.1.0`, `spec_math 0.1.6`,
   `special_functions 0.1.0`. Any entry with a registry source means
   you broke the rule. This applies to networking too: the scene
   server's HTTP/WebSocket/SHA-1/base64 are hand-rolled on `std::net`
   in `posim/src/scene/ws.rs`, `scene.html` uses vanilla JS + canvas,
   and the recorded video pages fetch nothing. The ONE deliberate
   exception lives outside the workspace: `dataset_tools/` (see
   Layout) — excluded in the root Cargo.toml, so `cargo build
   --workspace` still never touches crates.io and the workspace
   `Cargo.lock` still lists only local crates. posim reaches SQLite
   exclusively by SPAWNING `solveit_db.exe`.
3. **Fidelity to donor physics.** Formulas ported from the legacy types
   (softened gravity, Laplace vector incl. its `r = 0` guard, inertia
   formulas, kinetic energy, SDF central-difference normal) keep their
   arithmetic order — floating point is not associative.
4. **All state access through get/set.** The VM, machine mode, the
   scene subsystem, and any new front end reach `physical_object`
   fields only via the setters (they enforce the coupled invariants:
   mass↔inverse, inertia↔inverse, unit quaternions,
   momentum-canonical velocity).
5. **Missing sundials symbols are reported, not invented.** If an API
   you need is absent from `sundials_rs/`, stop and say exactly which
   symbol is missing, naming the C original's file (`src/` or
   `include/` of the upstream **SUNDIALS 7.8.0** release, at
   `C:\Users\youruser\Developer\sundials-7.8.0\` — that reference tree is not
   vendored here). Do not reimplement solver numerics locally. The same
   applies to a `None` from a 7.8.0 constructor or from
   `N_VGetArrayPointer`: turn it into a named `Err`, never an `unwrap`.
6. **Quaternions are w-first everywhere** (packing, VM literals, JSON,
   scene frame messages, video frames, docs). The 13-per-object state
   layout in `system.rs` is a pinned contract (ARCHITECTURE §3.2).
7. Public error style: `Result<_, String>` with actionable messages
   naming the field/column/feature. No panics in library code paths;
   scene threads turn bad window input into error events, never crashes.
8. **The struct name is `physical_object`** (lower-case, by
   specification). Import as
   `use physical_object::physical_object::physical_object;`; in files
   that import it, prefix other crate paths with `::`. Do not rename it
   and do not try to re-export it at the crate root (namespace clash).
9. **The scene evolves a copy.** The playback thread owns a synced
   clone of the system (`SCENE CREATE`/`SCENE REFRESH`/`RESET` sync
   it); notebook `STEP`/`RUN` never move the window and window playback
   never moves the notebook. Do not "fix" this by sharing the system —
   it is the isolation that keeps the VM lock-free (ARCHITECTURE §3.7).

## Working with the 7.8.0 API

Read ARCHITECTURE §3.3 before touching `integrate.rs`. The four things
that will bite you:

- **Handles are shared, not mutable.** `CVodeMem` and `ARKodeMem` are
  `Rc<RefCell<…>>`; every entry point takes `&`, never `&mut`. If you
  reach for `&mut`, you are reading 7.7.0-era code.
- **Vector data comes with a borrow guard.**
  `N_VGetArrayPointer` returns a `RefMut`. Holding it across a solver
  call on the same vector panics. Use `with_data` / `with_data_mut`,
  which take the guard, do the work, and drop it.
- **Constructors return `Option`, destructors take ownership.**
  `SUNContext_Create(SUN_COMM_NULL, &mut opt) -> SUNErrCode`;
  `CVodeFree(&mut Some(mem))`; `SPRKStepCreate(f1, f2, …)` takes the
  two right-hand sides **by value**, not `Option`.
- **`fmt_e(x, prec)` is `%.*e`; `fmt_ew(x, width, prec)` is `%*.*e`.**
  7.7.0's three-argument `fmt_e` split in two. Never use Rust's `{:e}` —
  its exponent form differs from C's and breaks byte-identity with the
  reference outputs.

## Windows platform notes

- **The physics is byte-identical to the Linux evidence** because the
  Windows engine's `sundials_libm` is host-independent (given `+fma`,
  see the header). Two documented exceptions pass through the **host
  libm** (the UCRT here, Apple libm on macOS, glibc on Linux) and may
  differ in the last digit: the quantum crate (`double_slit`/`tunneling`
  notebooks, pinned in `evidence/win11/accepted-divergences-dynamic.diff`)
  and the GEAR/RACK joint residual `g = sin(qθᵢ + pθⱼ)` in
  `physical_object/src/constrain.rs` (which is why
  `videos/rack_and_pinion.html` is a Windows recording; all 13
  committed recordings reproduce byte for byte here,
  `python recorder\src\record_all.py --check`).
- **A third exception, new to this port:** the curvilinear coordinate
  maps in `special_functions/src/coords.rs` call the host libm's
  transcendentals (`sin`/`cos`/`atan2`/`acos`/`sinh`/`cosh`/`acosh`/
  `asinh`/`hypot`/`ln`; `sqrt` is correctly rounded everywhere and is
  not a source). The seven dataset files they produce —
  `solveit37_pol.db`, `solveit37_sph.db`, `solveit37_sph_plots.html`,
  `solveit38_bis.db`, `solveit38_cyl.db`, `solveit38_obl.db`,
  `solveit38_tor.db` — differ from the macOS files in the last digits;
  every other dataset (245/252) and every movie (74/74) is
  byte-identical to the macOS output. Measured per `states` table
  (differing float values / total, worst relative difference):
  `solveit37_pol` 12/1400, 1.99e-16; `solveit37_sph` 3/1400, 1.30e-16;
  `solveit38_bis` 3/700, 1.58e-15; `solveit38_cyl` 5/700, 1.79e-16;
  `solveit38_obl` 16/700, 2.67e-14; `solveit38_tor` 5/700, 4.02e-16.
  The Windows transcripts of examples 36–38 are
  `logs\solveit_scripts\solveit_36_*.log`, `solveit_37_*.log`,
  `solveit_38_*.log`; exactly two printed lines differ from the
  macOS-quoted ones — example 37 `Out[21]= [3.0000000000000004,
  3.9999999999999996]` (the cart2→polar round trip) and example 38
  `Out[22]= 0.0000000000000007447602459741819` — while every other
  numeric `Out` line of 36–38, including the anchors quoted in
  `SolveIt.md` §7 (the closure residual `6.938893903907228e-18`,
  `theta = 1.5707963267948966`), is verbatim identical. When a page
  quotes a 36–38 number, paste it from those logs.
- The Windows byte-identity gate is `bash tools/win_verify_physics.sh`
  (Git Bash); the GUI smoke test is `python tools\win_gui_smoke.py`
  (`python tools\gui_smoke.py` also passes here). Divergences beyond
  the pinned diffs fail the gate.
- The upstream C reference tree lives at
  `C:\Users\youruser\Developer\sundials-7.8.0\` on this machine.

## Workflow

- **Make backups before modifying files** (copy into
  `.backups\<date>\`), then edit.
- After EVERY build/test/run command: `2>&1 | Tee-Object <log>` (Git
  Bash: `2>&1 | tee <log>`), then read the log before editing. Never
  re-run a command that returned no visible output. ≤2 attempts per
  failing command, then switch strategy.
- Commit after every coherent file group; keep
  `cargo build --workspace --all-targets` warning-free and
  `cargo test --workspace` green at every commit (**638 tests**:
  49 physical_object lib + 19 collision + 9 conservation +
  42 constrained/DAE + 121 posim + 92 quantum + 240 special_functions
  (incl. the 7 coordinate-map suites) + 11 vendored identities +
  55 doctests). The sidecar adds its own 11
  (`cd dataset_tools; cargo test; cd ..`).
- New solver features need: a unit or conservation test with an
  **analytic** expectation (not a golden-output snapshot), a grammar
  hook if user-facing (lexer keyword → parser production → VM
  instruction → `HELP_TEXT` → `grammar.md`/`.tex`), and an
  ARCHITECTURE.md update if a contract changed.
- When touching the language, update **all four** in lockstep: the EBNF
  comment in `parser.rs`, `HELP_TEXT` in `vm.rs`, `grammar.md`, and
  `grammar.tex` (then recompile the PDF). The DATASET/MOVIE/SQL family has
  a two-directional test for exactly this:
  `every_dataset_and_movie_subcommand_is_documented_in_lockstep`. Scene-visible behavior also
  updates `scene_info.md`/`.tex`; user-visible behavior also updates
  `SolveIt.md`/`.tex`.
- Scene argument parsing is **term-level** (so `scene rotate 15 -5` is
  two arguments); if you change it, `parser::scene_command` and the
  grammar docs move together.
- **Documentation numbers are measured, not remembered.** Every figure
  quoted in `SolveIt.md`, `grammar.md` and
  `PORT_7.8.0_PROVENANCE.md` came out of a run captured in
  `evidence/port-7.8.0/`; every Windows figure in
  `PORT_WIN11_PROVENANCE.md`, `VERIFICATION_WIN11.md` and
  `WINDOWS_PORT_COMPLETE.md` came out of a run captured in `logs/` or
  `evidence/win11/`. If you change something that moves a number,
  re-run and re-paste; do not adjust the prose to fit.

## The constrained / equilibrium / sensitivity paths

Read ARCHITECTURE §3.9 before touching `constrain.rs`, `equilibrium.rs`
or `sensitivity.rs`. Four things that already cost a day:

- **The GGL projection is mass-weighted: `q̇ = v - M⁻¹Jᵀμ`.** Dropping
  `M⁻¹` is dimensionally wrong the moment a joint grips orientation
  (`J_ω` carries the attachment arm), and *invisible* for rods, where it
  is only a rescaling of μ. This cost a day; do not undo it.
- **`IDACalcIC` cannot find the multipliers and is not called.** The
  index-2 residual is satisfied by free fall with λ = 0, so there is
  nothing for it to solve. `seed_multipliers` solves `g̈ = 0` directly.
- **`g = |d| - L`, not `|d|² - L²`.** The squared form is better
  behaved algebraically and *worse* numerically: its gradient scales
  with `L`, and the index-2 corrector stops converging for `L ≠ 1`. Do
  not "simplify" it back.
- **GGL index-2, not index-1.** Both `g` and `ġ` are carried as
  algebraic equations. Dropping `μ` gives an index-1 system whose `g`
  drifts quadratically — and nothing fails loudly when it does.
- **Do not call `IDACalcIC` when the ICs are already consistent.** A
  bare `CONSTRAIN` guarantees they are, and `seed_multipliers` supplies
  the exact tension; asking IDA to re-derive it at trajectory tolerance
  fails with `IDA_CONV_FAIL`.
- **The sensitivity parameter vector is shared (`Rc<RefCell<Vec<f64>>>`),
  and the RHS must read every value out of it.** Capture a copy and the
  difference-quotient sensitivities come back as zeros, silently.

## Windows traps that have already cost time (this port)

- **Python's default text encoding here is cp1252, not UTF-8** (checked:
  Python 3.14.5, `locale.getpreferredencoding()` = cp1252). Every
  `open()`, `read_text()`, `write_text()` and every `subprocess` pipe
  with `text=True` that touches posim I/O or repository files must say
  `encoding="utf-8"` explicitly — posim prints UTF-8 em-dashes, and one
  unmarked pipe turns them into `â€”` (or writes a snippet file rustc
  rejects as invalid UTF-8). All first-party tooling already complies;
  keep new code compliant.
- **Text-mode writes translate `\n` to `\r\n`.** Anything byte-compared
  (recorded videos, notebook JSON, evidence logs) is written with
  `newline="\n"`; `.gitattributes` sets `* -text` so git never converts
  eols either. Do not remove either half.
- **The binary is `posim.exe`.** Every finder probes
  `target\release\posim.exe` before the extensionless name; keep that
  order in new tooling.
- **The sidecar is `solveit_db.exe`, and posim's finder knows it.**
  `find_db_tool` in `posim/src/dataset.rs` probes
  `dataset_tools\target\release\solveit_db.exe` before `solveit_db`
  (after `$env:POSIM_DB_TOOL`, before `PATH`). Without that order every
  DATASET command reported the sidecar "not built" on a checkout where
  cargo had just built it. Keep the `.exe`-first order in any new
  finder, exactly as the Python tooling already does for `posim.exe`.
- **SQLite `-wal`/`-shm` side files are transient.** The sidecar runs
  every dataset in WAL mode and checkpoints on close; a
  `<name>.db-wal`/`<name>.db-shm` pair left beside a dataset (a killed
  posim, a still-open notebook) is not data and is gitignored
  (`*.db-wal`, `*.db-shm`, `*.sqlite3-wal`, `*.sqlite3-shm`). Never
  commit one, never byte-compare a `.db` while its `-wal` exists —
  `DATASET CLOSE` (or `solveit_db.exe checkpoint <db>`) first.
- **posim string literals process `\` escapes** (grammar lexer:
  `\"`, `\\`, `\n`, anything else keeps the following character), so a
  Windows path pasted into a quoted literal loses its separators —
  `"C:\Users\x"` becomes `C:Usersx`. Use forward slashes (Windows
  accepts them) or doubled backslashes; the qm3 tests do exactly this.
- **A cwd inside a `TemporaryDirectory` blocks its cleanup**
  (WinError 32). `os.chdir` back before the context manager exits.
- **`cargo test` shell-reply races**: `jupyter/test_kernel.py` matches
  shell replies to their execute request by parent `msg_id` — on
  Windows `wait_for_ready` can leave a stale reply queued and shift
  every status by one. Keep the matching loop.

## Traps that have already cost time

- **`RUN` takes a duration, not an absolute time.** `run 1.7` then
  `run 2.89` lands you at t = 4.59.
- **`system.g_constant` defaults to 1.** Bodies rattling in a box also
  attract each other, and the softened force at `softening = 1e-6` is
  nearly singular when surfaces touch. The same box scenario conserves
  energy to 3e-16 with `G = 0` and drifts 3.2 % with `G = 1`. **A
  conservation claim carries its system settings with it** — the
  `free_system` helper in `physical_object/tests/collision.rs` sets
  `G = 0` for exactly this reason.
- **A magnetic moment tensor's third column is what a B along z can
  grip.** `[[0, 0.5, 0], [-0.5, 0, 0], [0, 0, 0]]` looks like a
  reasonable antisymmetric tensor and produces exactly zero torque.
- **`BALL`, `HINGE`, `UNIVERSAL`, `GEAR`, `RACK` and `PRISMATIC` are CONTEXTUAL keywords** — commands
  only at the start of a line. `ball` is exactly what a physics user
  calls a sphere, and `new sphere as ball` / `get ball.mass` were already
  in the tests and docs; reserving it outright broke them.
- **A joint constrains velocity, so starting velocities are PROJECTED**
  (`project_initial_velocities`). A body turning about an offset pivot
  must have its centre moving; a caller who sets ω and leaves v at zero
  is off the manifold, and IDA then fails on the first step at every
  tolerance. Reported via `RunReport::initial_velocity_projected`.
  A rod has `J_ω = 0` and never needed it — which is why this hid.
- **Orientation joints carry a tolerance floor** of `rtol = 1e-6`
  (`ROT_JOINT_RTOL_FLOOR`): the index-2 accuracy ceiling is real, and
  measured sharp across twelve pendulums.
- **A constrained system refuses every method but `METHOD IDA`**, and a
  fully-free system has no isolated equilibrium at all (translate it and
  nothing changes) — `EQUILIBRIUM` says so and tells you to pin a body.
- **Quantum discretisation error converges; a bug does not.** Before
  calling a QM disagreement a defect, refine the grid and check the
  ratio: the square-barrier transmission at E = 2 goes 0.069368 →
  0.070328 → 0.070570 toward the analytic 0.070651 as the grid goes
  4k → 16k → 64k.

## Verified physical facts to protect (regression anchors)

Under 7.8.0 every one of these is **byte-identical** to its 7.7.0 value;
that is the substance of `PORT_7.8.0_PROVENANCE.md` §3.

- Outer solar system matches the donor `solar_system` example: Pluto
  `x = 31.78592516  y = 38.63618957  z = 3.19279415` at t = 500,000
  days, energy drift `7.835809e-07`, `12581` internal steps.
- Kepler e = 0.6 conserves E/L/Laplace < 1e-6
  (`|dA|/|A| = 1.131858e-07`); gyroradius = mv/(qB) to 1e-4; tumbling
  body conserves L **exactly** (`|dL|/|L| = 0.000000e+00`).
- Kepler's third law: `T²/a³ = 0.039478417604357434` for a = 1, 2, 3, 4,
  identical to the last bit.
- Ball dropped on a plate: TOI `0.8944271909999157` against analytic
  `sqrt(0.8) = 0.8944271909999159` — 1.24e-16 relative.
- Mixed shapes in a rigid BOX 4 with G = 0: `|dE|/E = 3.4e-16` through
  36 collision events.
- Scene reverse replays history to **exactly** t = 0 (bit-identical
  snapshots, `playback_forward_then_reverse_restores_state`).
- Zero-collidable-pair systems are **bit-identical** with COLLIDE ON vs
  OFF (structural invariance — protect it).
- The tilted-torus fit: an axis-aligned torus of outer radius 2 exactly
  inscribes BOX 4; tilted to axis (1,1,1)/√3 its per-axis extent is
  1.5·√(2/3) + 0.5 ≈ 1.7247, clearing every wall
  (`tilted_torus_fits_a_4x4_box_where_flat_does_not`).
- A ball rattling in a rigid BOX conserves E with the walls
  **bit-identically at rest**
  (`ball_in_a_rigid_box_conserves_energy_and_walls_never_move`).
- A point threads the torus hole while the fat ball bounces with
  TOI = (3−√1.45)/4
  (`point_threads_the_torus_hole_but_a_fat_ball_bounces`).
- The BOX grammar family (BOX `<size>`/OFF/status, `system.box`,
  `[wall: static, inverse_mass=0]` LIST tags, machine
  `box`/`wall`/`inverse_mass`, scene init `box` + wall flags)
  round-trips end-to-end (`box_family_and_infinite_mass_walls`,
  `state_reports_box_walls_and_inverse_mass`,
  `box_shapes_and_wall_flags_reach_the_init_message`); and `BOX <size>`
  after a wall deletion removes the surviving tracked slabs before
  building the new box — no orphan leak
  (`box_recreate_after_wall_deletion_leaks_nothing`).
- Parallel round shapes touch side-on at the exact lateral gap — the
  radial rejection axes make cylinder-cylinder side contacts exact
  (`parallel_cylinders_side_contact_is_exact`).
- A ball released FROM REST above a thin plate is caught at the
  analytic TOI to 1e-6 — the anti-tunneling cap is acceleration-aware
  (`ball_released_from_rest_does_not_tunnel_the_thin_plate`).
- **A known limitation, pinned deliberately:** two disks with PARALLEL
  planes have separation |dz|, which touches zero without a sign
  change, so face-on disk-disk crossings are invisible to
  downward-crossing rootfinding
  (`parallel_disk_disk_separation_is_the_documented_limitation`; tilt
  one disk or model a thin cylinder).
- NEW is transactional (a failing initializer or final validation
  leaves no ghost object) and the torus inner/outer pair is genuinely
  order-independent, horn torus `inner_radius = 0` valid
  (`torus_pair_is_order_independent_and_new_is_transactional`).
- Two tumbling dumbbells colliding off-center conserve E, P **and L
  (about the origin)** to 1e-8 through real CVODE events — the
  part-wise exact narrow phase puts the impulse pair at one shared
  contact point
  (`colliding_dumbbells_conserve_energy_momentum_and_angular_momentum`).
- Scene Reset (toolbar button / window `reset` / `SCENE RESET` — one
  primitive) restores the playback's initial state **bit-identically**,
  clears history and the step counter, returns the mode to Stopped, and
  Start re-runs from the beginning
  (`reset_restores_the_initial_state_and_start_reruns`).
- The `create_dumbell` flow — define/call/members/shorthands/renumber/
  errors/redefine/LET-defaults/ghost-free failing calls — round-trips
  end-to-end (`def_call_named_objects_and_dumbbell_members`); the
  directed supports are exact for BOTH dumbbell ends (asymmetric wall
  gaps exact, the light end pokes farther when m2 > m1, ball-vs-pole
  and ball-vs-rod contacts exact —
  `dumbbell_wall_gaps_and_ball_contacts_are_exact`,
  `dumbbell_constructor_com_sdf_and_supports`); part masses whose SUM
  overflows f64 are refused, and a fat-rod dumbbell reports honest
  side-on supports — rank 1 with footprint = the rod half-length
  (`fat_rod_dumbbell_reports_honest_support_ranks`).
- The dumbbell composite inertia is exact: with a massless rod and two
  2 kg spheres of radius 0.3 at z = ±1, the tensor is
  `[[4.144,0,0],[0,4.144,0],[0,0,0.144]]`.
- DEF is hardened: parameters cannot be reserved words, builtins,
  pi/tau or duplicates; defaults are expressions only (a command like
  `reset` is refused at definition time); a `}` inside a `#` comment
  does not close the body; `NEW ... AS n` errors when `n` is bound to a
  number; and a function that runs NEW inside another NEW's initializer
  stashes/restores the outer NEW context
  (`def_hardening_reserved_words_duplicates_defaults_and_nesting`).
- Contact records arriving with a notebook system never reach the scene
  playback copy — cleared at create, refresh-sync and reset
  (`stale_notebook_contacts_never_reach_the_playback_copy`).

## Verified UI facts to protect

Re-check with a headless-Chrome CDP session after touching
`scene.html`: arrow keys translate the view right/left (and up/down),
left-drag orbits yaw+pitch, mouse wheel and `+`/`-` zoom, Space toggles
start/pause, toolbar Start/Pause/Stop/Reverse work, the statusbar shows
mode/t/dt/E/bodies/contacts/camera/fps, the Contacts button and `C`
toggle the golden contact-normal arrows and the frame protocol carries
the exact analytic normal/point/impulse for a head-on impact, the
dashed `#5d84a8` interior box wireframe draws when a BOX exists (**the
six wall slabs are never drawn as bodies**), and torus (outer/inner
equators + tube rings + 4 cross-sections), disk (rim + 2 diameters) and
cylinder (2 rims + 4 side lines) wireframes render quaternion-rotated so
spin is visible; the permanent toolbar Reset button (`bt-reset`)
re-initializes the playback and Start re-runs from the beginning; the
labeled conserved-quantities readout (`hud`: E, P and L with components
and magnitudes) updates live and reads identically before and after a
dumbbell impact; entity labels show the registered user names
(`dumbell0`) instead of `objN`; and dumbbells render as one rigid body —
two shaded spheres at their rotated COM offsets joined by the rod's four
silhouette lines.

The recorded video player (`recorder/src/record_video.py`) follows the same
rules: wall slabs excluded from both the draw list and the camera
auto-fit, round shapes rotated about their local z axis, contact arrows
along the exact analytic normal scaled by impulse.
