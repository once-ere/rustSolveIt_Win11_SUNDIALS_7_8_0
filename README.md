# rustSolveIt — on pure-Rust SUNDIALS 7.8.0, for Windows 11 (x86-64)
This notebook is distributed as free software under the GNU General Public License.They are distributed in the hope that they will  be useful,but WITHOUT ANY WARRANTY;without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.See the GNU GPL for more details.

Please  cite  this  work  and this web page if you use it.

Pure-Rust physics simulator, the **Windows 11 (x86-64)** edition of the
project. Its lineage is two ports deep (Linux → macOS → Windows), on top
of the 7.7.0 → 7.8.0 engine upgrade: this repository was ported from
[`once-ere/rustSolveIt_macos-silicon_SUNDIALS_7_8_0`](https://github.com/once-ere/rustSolveIt_macos-silicon_SUNDIALS_7_8_0)
(macOS / Apple Silicon), which was itself ported from the Linux release
[`once-ere/rustSolveIt_Using_SUNDIALS_7_8_0`](https://github.com/once-ere/rustSolveIt_Using_SUNDIALS_7_8_0)
— which is
[`once-ere/rustSolveIt`](https://github.com/once-ere/rustSolveIt) with
its numerical engine upgraded from a pure-Rust translation of SUNDIALS
**7.7.0** to one of **7.8.0**. The engine vendored here is the **Windows
engine**
[`once-ere/SUNDIALS_7_8_Rust_port_for_Windows11`](https://github.com/once-ere/SUNDIALS_7_8_Rust_port_for_Windows11):
it differs from the Linux/macOS engine only in its backend module sets
(public API identical), and its deterministic pure-Rust math library
(`sundials_libm`), together with the `-C target-feature=+fma` pin in
`.cargo/config.toml`, is what reproduces the Linux physics **byte for
byte** on this machine. The record, measured here on 2026-09-09: the
physics byte-identity gate `bash tools/win_verify_physics.sh` exits 0 —
the twelve collision scripts **IDENTICAL** to the Linux evidence, the six
self-checking examples and the 59 dynamic notebooks
**IDENTICAL-MODULO-DOCUMENTED** (the pinned diffs are one blank line in
the examples log and the last-digit differences of the two quantum
notebooks); **74 of 74 movies** and **245 of 252 datasets** are
byte-identical to the macOS project's committed files; all **13 recorded
videos** reproduce byte for byte. The only places the host math library
(the Windows UCRT here, glibc on Linux, Apple libm on macOS) shows
through are three documented classes: (1) the two quantum notebooks
(`double_slit`, `tunneling`), last printed digit; (2) the `GEAR`/`RACK`
joint residual — `rack_and_pinion.html` is the Windows recording, and it
reproduces byte for byte here; (3) the fourteen curvilinear coordinate
maps of `special_functions/src/coords.rs` used by SolveIt examples 37–38 —
six datasets (`solveit37_pol`, `solveit37_sph`, `solveit38_bis`, `_cyl`,
`_obl`, `_tor`) differ from the macOS files in a handful of values at
1e-16 to 2.7e-14 relative, plus one `_plots.html` page derived from them,
and two transcript lines of examples 37/38 print different last digits.
See [PORT_WIN11_PROVENANCE.md](PORT_WIN11_PROVENANCE.md),
[VERIFICATION_WIN11.md](VERIFICATION_WIN11.md),
[WINDOWS_PORT_COMPLETE.md](WINDOWS_PORT_COMPLETE.md),
[PORT_7.8.0_PROVENANCE.md](PORT_7.8.0_PROVENANCE.md),
[evidence/win11/](evidence/win11) and
[evidence/port-7.8.0/](evidence/port-7.8.0); the macOS and Linux records
are kept verbatim as lineage pages
([PORT_MACOS_PROVENANCE.md](PORT_MACOS_PROVENANCE.md),
[VERIFICATION_MACOS.md](VERIFICATION_MACOS.md)).

rustSolveIt itself is the refined and refactored export of
[`once-ere/rustSimulate`](https://github.com/once-ere/rustSimulate)
(full history preserved; the refinement pass and its evidence are
recorded in [REFINE_PROVENANCE.md](REFINE_PROVENANCE.md)).



Based on a Claude-Fable-5/Opus-5 port of my
2002 'SolveIt' code (c++ v.14, c, fortran, x86 assembly codes)
anything that could not be ported to pure rust has been omitted.
Numerical integration runs through
[`sundials_rs/`](sundials_rs) — a pure-Rust translation of SUNDIALS
7.8.0 vendored in this repository (CVODE Adams/BDF, ARKODE symplectic
SPRK). Zero `unsafe`, zero external crate dependencies, zero warnings.

The repository is **self-contained**: an ordinary clone is all you need
— no submodules, no network access during the build, nothing from
crates.io for the simulator itself.

Open **PowerShell** (Start menu, type `PowerShell`), go to the folder you
keep projects in, and type:

```powershell
git clone https://github.com/once-ere/rustSolveIt_Win11_SUNDIALS_7_8_0.git
cd rustSolveIt_Win11_SUNDIALS_7_8_0
cargo run                 # the notebook REPL (type HELP)
```

(The same three lines work unchanged in Git Bash.) The first build takes
a few minutes; afterwards `cargo run` starts in a second.

What you need:

- **Rust** for the MSVC target: run `rustup-init.exe` from
  <https://rustup.rs> and accept the default `x86_64-pc-windows-msvc`
  toolchain (measured here: rustc 1.91.1 / cargo 1.91.1).
- **Any installed Visual Studio 2022 or 2026** (the free Build Tools are
  enough) — rustc finds its MSVC **linker** by itself, no "developer
  prompt" needed. Its C compiler `cl.exe` is used in exactly two places
  and nowhere else: the SQLite **sidecar** (`dataset_tools`, whose
  bundled SQLite amalgamation the `cc` crate compiles — measured here with
  the Visual Studio 2026 Build Tools, MSVC 14.51.36231) and the optional
  REBOUND / REBOUNDx **C reference gates** (the same Build Tools; compiler
  banner `Version 19.51.36256 for x64`). The simulator workspace itself
  is pure Rust.
- **A CPU with FMA** (every Intel since 2013, every AMD since 2012): the
  vendored engine's deterministic math library requires it, and the root
  `.cargo/config.toml` pins `-C target-feature=+fma`. Do not remove that
  pin.
- **Python 3** (`python`, not `python3` — measured here: 3.14.5) for the
  GUIs, notebooks, recorder and index tools; **Google Chrome** for the
  headless GUI captures and the encyclopedia PDF; **MiKTeX** only if you
  want to rebuild the PDFs; **uv** (or `python -m venv`) only for the
  optional `jupyter\.venv` of the wrapper kernel.
- **PowerShell 7 (`pwsh`) recommended**: Windows PowerShell 5.1 has no
  `ForEach-Object -Parallel`, and its `>` / `Tee-Object` redirections
  write UTF-16, which the index tools do not read. Git Bash is the
  alternative wherever a page shows a Git Bash form.
- **No WSL.** Nothing here uses a WSL2 compiler or linker; Git Bash (which
  comes with Git for Windows) is the only POSIX shell any command below
  assumes, and only where the text says so.

This project is a standalone export of the simulator. The original
Linux export's lineage, byte-identity manifest and verification
transcript are recorded in [EXPORT_PROVENANCE.md](EXPORT_PROVENANCE.md)
(a lineage page); this port's own record is
[PORT_WIN11_PROVENANCE.md](PORT_WIN11_PROVENANCE.md).

Latest release: a programmable notebook and a compound rigid body —
user-defined functions (`DEF name(param = default, ...) { body }`,
every body line syntax-checked at definition, `FUNCS`/`SHOW` to list
and edit), named objects (`NEW ... AS name`, plus `LET` variables and
string literals), and the rigid `DUMBBELL` (two solid spheres plus a
rod as ONE rigid body, exact part-wise collisions conserving E, P and
L through real solver events); the scene window gains a permanent
Reset button (with `SCENE RESET` — bit-identical re-initialization,
Start re-runs) and a live labeled conserved-quantities readout (E, P
and L); **638 passed, 0 failed, workspace-wide on Windows 11**
(`cargo test --workspace`, logs/test_final.log: 49 physical_object lib +
19 collision + 9 conservation + 42 constrained/DAE + 121 posim +
92 quantum + 240 special_functions + 11 vendored identities +
5 quantum doctests + 50 special_functions doctests). Both builds are
warning-free: `cargo build --workspace --all-targets` and
`cargo build --release --workspace --all-targets` (0 errors, 0 warnings;
`target\release\posim.exe` is 5,705,216 bytes).

- `physical_object/` — library: `pub struct physical_object`, the
  unique union of the legacy `PointParticle`, `RigidBody` and
  `RigidBody3D`, with get/set for every field; `PhysicalObjectSystem`;
  the sundials integration drivers; validated examples.
- `posim/` — the simulator front end: lexer → grammar compiler → stack
  machine, a notebook REPL (`In[n]`/`Out[n]` cells), script batch mode,
  and a JSON machine mode. The binary is `target\release\posim.exe`.
- `gui/` — thirteen live GUI web pages, one per recorded video scene:
  each a standard-library Python server driving the engine over the
  machine protocol plus a canvas page with Start/Pause/Reset and live
  readouts verifying that scene's closed forms (`gui/README.md`).
  Measured here: `python tools/win_gui_smoke.py` and
  `python tools/gui_smoke.py` both report **13 of 13 GUIs pass**.
- `notebooks/` — one executed Jupyter notebook per example — 128 in
  all, one per video scene, Rust example, collision script, SolveIt
  script and dynamic notebook. Each is a stand-alone Python notebook
  that starts the simulator in machine mode, explains every command it
  sends, derives the equations of motion and constraint equations,
  carries real outputs, and **owns a SQLite dataset**: the run is
  recorded into `datasets/nb_<name>.db`, read back with SQL and with a
  plain-Python `sqlite3` cell, and a browser movie is derived from it
  (`notebooks/README.md`). Measured here: **128 ok, 0 failed; 128/128
  pass all eight requirements** (logs/notebooks_run.log).
- `datasets/`, `movies/`, `dataset_tools/` — the SQLite recording
  layer: one dataset per notebook and per dataset example (252 files:
  137 `.db` plus 115 `_plots.html` pages), the 74 movies derived from
  them, and the `solveit_db` sidecar that owns all SQLite access
  (rusqlite + r2d2 connection pool, WAL journal mode, 5 s busy timeout).
  The simulator itself still links zero external crates — it spawns the
  sidecar, and its finder (`posim/src/dataset.rs`) probes
  `dataset_tools\target\release\solveit_db.exe` first. Build the sidecar
  once, from the repository root:

  ```powershell
  cd dataset_tools; cargo build --release; cd ..
  ```

  (Git Bash: `cd dataset_tools && cargo build --release && cd ..`.)
  Measured here: 0 warnings, `solveit_db.exe` 2,326,016 bytes,
  `cargo test` inside `dataset_tools` 11 passed (8 unit + 3 integration),
  and the concurrency demo
  `.\dataset_tools\target\release\solveit_db.exe stress $env:TEMP\stress_demo.db 10 500`
  (Git Bash: `dataset_tools/target/release/solveit_db.exe stress /tmp/stress_demo.db 10 500`)
  prints `10 threads x 500 rows = 5000 expected, 5000 written,
  journal_mode=wal`. The `-wal` / `-shm` files that appear beside an
  open database are SQLite's transient side files — ignore them.
- `jupyter/` — JupyterLab wrapper kernel so notebooks can get/set the
  simulator's data (see `jupyter/README.md`). Measured here:
  `python jupyter/test_protocol.py` all protocol checks passed;
  `jupyter\.venv\Scripts\python.exe jupyter/test_kernel.py` 7/7 cells ok
  (that venv is gitignored and created once with `uv venv jupyter\.venv;
  uv pip install -p jupyter\.venv\Scripts\python.exe ipykernel
  jupyter_client`, then `$env:POSIM_NO_BROWSER = "1"` before the test
  line; Git Bash: forward slashes and `POSIM_NO_BROWSER=1 …`).
- `planet_Mercury/` — the Mercury 3:2 spin-orbit sub-project (two Rust
  crates, two notebooks, two browser pages); see
  [its section below](#the-planet_mercury-sub-project).
- `SolveIt_Notebooks_for_rust/` — the notebook encyclopedia: 147
  notebooks in seven topics with a built encyclopedia; see
  [its section below](#the-notebook-encyclopedia-solveit_notebooks_for_rust).
- `rebound_rust/`, `reboundx_rust/` — the REBOUND / REBOUNDx N-body
  ports; see [the next section](#the-rebound--reboundx-n-body-ports).
- `sundials_rs/` — the numerical engine: a pure-Rust, zero-`unsafe`,
  dependency-free translation of SUNDIALS 7.8.0 (the Windows engine),
  vendored here as a self-contained workspace (read-only; upstream any
  changes).
- `PLAN.md` — the integration plan / design record (union mapping,
  grammar, solver mapping, verification results).

## The REBOUND / REBOUNDx N-body ports

This repository also carries two further pure-Rust astronomy libraries,
built with the same discipline (zero `unsafe`, zero dependencies, zero
warnings, C names preserved) and verified bit-for-bit against their
originals compiled with Microsoft's C compiler **on this machine**
(REBOUND at commit `dad5f97`, REBOUNDx 5.1.0; MSVC `cl.exe` from the
Visual Studio 2026 Build Tools, `/Ox /fp:precise`; the only C compiled
anywhere in this repository besides the SQLite sidecar):

- [`rebound_rust/`](rebound_rust) — `rebound_rs` 5.1.1, a translation of
  Hanno Rein et al.'s [REBOUND](https://github.com/hannorein/rebound)
  N-body code: **63 of 63** integrator configurations bit-identical; the
  **1,482-particle shearing sheet** (400 steps, 102,533 collisions)
  byte-identical with equal SHA-256 state dumps; Simulationarchive files
  interchangeable with the C build in all **six** directions (0 differing
  lines); the 65 orbital-derivative functions 130/130 lines identical,
  frequency analysis 30/30; **394 tests** (`cargo test --release`:
  67+32+39+72+55+70+59), clippy 0 warnings; 13 examples — each with its
  own Jupyter notebook.
- [`reboundx_rust/`](reboundx_rust) — `reboundx_rs` 5.1.0, a translation
  of Dan Tamayo et al.'s [REBOUNDx](https://github.com/dtamayo/reboundx)
  extra-physics library: all **six** `tides_spin` acceptance runs
  **BIT-IDENTICAL** (including the 100,000-orbit Kozai run's adaptive
  steps); binary files interchangeable with C-REBOUNDx — 25/25 fields
  both ways, the only differing bytes being the git-hash header field;
  **137 tests** (34+32+28+17+26; 2 doctests ignored), clippy 0 warnings;
  4 examples with notebooks.

The 200,000-sample libm differential that accompanies the gates is the
one honest platform note: `sin`, `cos`, `tan`, `atan2`, `sqrt`, `fmod`,
`exp` and `log` agree with the UCRT on every sample; `pow` differs on
60 of 200,000 samples, every one by exactly 1 ULP (the macOS edition
recorded none against Apple libm). All 17 rebound notebooks executed
here with no errors.

Both folders are GPL-3.0-or-later (their own LICENSE files — unlike the
BSD-3-Clause simulator around them). They are **not** dependencies of the
simulator — they are excluded from this Cargo workspace and build on
their own:

```powershell
cd rebound_rust;  cargo test --release; cd ..
cd reboundx_rust; cargo test --release; cd ..
python -m pip install --user nbformat nbclient ipykernel matplotlib   # once (the executor's packages)
python rebound_rust/notebooks/make_notebooks.py    # regenerate the 17 notebooks
$env:PYTHONIOENCODING = "utf-8"                    # nbclient failure messages can carry non-ASCII; cp1252 is the default here
python rebound_rust/notebooks/run_notebooks.py     # execute them (Git Bash: PYTHONIOENCODING=utf-8 python rebound_rust/notebooks/run_notebooks.py)
```

(Git Bash: `cd rebound_rust && cargo test --release && cd ..`, and the
same for `reboundx_rust`.) The complete guide, written for a reader who
has never programmed, is
[`rebound_rust/rebound_rust.md`](rebound_rust/rebound_rust.md) (also as
a PDF); the Windows port provenance with every command and measured
result is
[`REBOUND_REBOUNDX_WIN11_PROVENANCE.md`](REBOUND_REBOUNDX_WIN11_PROVENANCE.md)
(the raw log of the C-reference gates is
`rebound_rust/logs/win11_c_reference_gates_20260909.md`); the macOS
edition's record is kept verbatim as a lineage page,
[`REBOUND_REBOUNDX_MACOS_PROVENANCE.md`](REBOUND_REBOUNDX_MACOS_PROVENANCE.md).

## Six solver families, four questions

| you type | question | solver |
|---|---|---|
| `STEP` / `RUN` | what happens next? | CVODE / ARKODE |
| `CONSTRAIN` / `GEAR` / `RACK` / `BALL` / `HINGE` / `UNIVERSAL` / `PRISMATIC` + `METHOD IDA` + `RUN` | …with this geometry held **exactly** | IDA |
| `EQUILIBRIUM` | where does it come to **rest**? | KINSOL |
| `SENSITIVITY 3 "gravity.y"` | how much does the answer **depend** on an input? | CVODES / IDAS |

A rod is a geometric fact, not a stiff spring, so `CONSTRAIN` turns the
equations of motion into a differential-algebraic equation. Over a full
pendulum period IDA holds the rod's length to 7.1e-15 (`worst |g| =
7.105427357601002e-15` after 384 solver steps, |dE/E| 6.9e-13) and the
bob closes to within 2.8e-10 of its start
(`logs/solveit_scripts/solveit_17_pendulum_dae.log`). `EQUILIBRIUM` hangs the same pendulum
straight down to 13 digits. `SENSITIVITY` returns `∂y/∂g = T²/2` on free
fall to 1.3 parts in 10⁸, and **exactly zero** for `∂y/∂mass`, because
in uniform gravity there is no dependence to find.

See grammar.md §5.13 for the commands and SolveIt.md examples 17–18 for
the worked cases.

## The planet_Mercury sub-project

[`planet_Mercury/`](planet_Mercury) is a complete teaching-and-research
sub-project that simulates how the planet Mercury was tidally braked by
the Sun and caught in its strange **3:2 spin-orbit resonance** — three
spins for every two trips around the Sun — using this repository's
vendored pure-Rust SUNDIALS 7.8.0 CVODE as the only integrator (BDF +
Newton + dense, rtol 1e-12, exact root-finding on spin-ratio crossings).
It has two crates, two notebooks and two browser pages:

- **`planet_Mercury/mercury_rs/`** (BSD-3-Clause, `#![forbid(unsafe_code)]`,
  zero external dependencies; path-depends on `sundials_rs`) — the compute
  program. Test 1 integrates the five-state system y = [a, e, M, θ, Ω]
  with Hut (1981) constant-time-lag tides and the Goldreich–Peale
  triaxial "handle" torque; test 2 adds Einstein's perihelion precession
  and Jupiter's Laplace–Lagrange secular forcing (six states). The binary
  is `mercury_rs.exe` with the sub-commands `print-config`, `run-a`,
  `run-b`, `sweep [--branches N]`, `run-b-final --branch K`, `run-d`,
  `run-e`, `t2-gr-check`, `t2-jupiter-check`, `t2-movie`,
  `t2-sweep [--branches N]`, `t2-final --branch K`.
- **`planet_Mercury/mercury_crosscheck/`** (GPL-3.0-or-later, because it
  links the GPL `rebound_rust` / `reboundx_rust` ports) — an independent
  one-command check that the REBOUNDx `tides_spin` formulation gives the
  same despin rate as the Hut secular formula `mercury_rs` uses.
- **`planet_Mercury/notebook/`** — the two executed Jupyter deliverables
  `mercury_tidal_locking.ipynb` (test 1) and
  `mercury_test2_jupiter_gr.ipynb` (test 2): standard-library Python
  cells that drive `mercury_rs.exe` by subprocess, load every CSV into a
  SQLite database, answer questions with SQL, run a verification
  gauntlet that stops the notebook on any failed assertion, and bake the
  browser page; with their deterministic builders (`build_notebook.py`,
  `build_notebook2.py`), headless runner (`run_notebook.py`) and
  structure auditor (`check_notebook.py`).
- **`planet_Mercury/gui/`** — `mercury_orbit.html` (test 1) and
  `mercury_test2.html` (test 2): self-contained recorded-player pages
  (all data embedded, vanilla JavaScript + canvas, zero network
  requests) — the orbit with the spinning planet and its long-axis
  handle, a spin/orbit dial that settles on exactly 1.5, the libration,
  period and angular-momentum-ledger plots, play/scrub controls; baked
  by `bake_page.py` / `bake_page2.py`.
- `plan/` (eight planning documents), `SOURCE_SPECIFICATION.md`, and
  `INSTRUCTIONS_FOR_TEACHERS_AND_STUDENTS.md`, the complete self-contained
  guide. `planet_Mercury/data/` (hundreds of MB of CSV and SQLite) is
  regenerated by one notebook run and is deliberately not committed.

**Measured on this machine (2026-09-09).** `mercury_rs`:
`cargo build --release` 0 warnings, `cargo test --release` **19 passed**,
`mercury_rs.exe` 1,104,384 bytes (logs/mercury_rs_build.log,
mercury_rs_test.log). The cross-check (logs/crosscheck_stress_launcher.log)
prints:

```
spin ratio 100 x n, e = 0.20563, 200 orbits with IAS15 + tides_spin
measured  dOmega/dt = -3.652928e-18 rad/s^2  (REBOUNDx tides_spin)
predicted dOmega/dt = -3.643780e-18 rad/s^2  (Hut 1981 secular formula)
relative difference = 2.511e-3
PASS - crosscheck.despin_rate: two independent formulations agree within 2%
SUCCESS
```

(An upstream REBOUNDx notice about its 4.5.0 tidal-potential fix also
prints — expected, not an error.) Both notebooks were built, executed
headless and audited here (logs/mercury_notebook1.log,
mercury_notebook2.log): `mercury_tidal_locking.ipynb` ok (18 executed
cells), all structure rules pass; `mercury_test2_jupiter_gr.ipynb` ok (17
executed cells), all structure rules pass. Their gauntlets, every
assertion PASS:

- Test 1 (6/6 runs SUCCESS): the spec-literal run A leaves Ω/n at
  178.9260 after 10 Myr (the spec's own constants imply ~4.7 Gyr of
  braking, hence the documented 1000× time compression of the movie
  runs); the restart root is found at Ω/n = 1.600000000000e+00 at
  t = 4.4315 Myr; the 64-branch phase sweep captures **5/64 = 0.0781 ±
  0.034** (Goldreich–Peale's simple estimate ~0.070; capture is bit-level
  path-sensitive, and the notebook says so); canonical capture at
  **4.6501 Myr**; settled-era mean Ω/n **1.5000000368**; final periods
  **87.9678 d / 58.6512 d / 175.9896 d** against the observed 87.969 /
  58.646 / 175.938; the time-averaged two-thirds lock holds to 8.8204e-09;
  angular-momentum ledger pre-capture drift 4.029e-10 (budget 1e-9),
  locked-era drift 8.821e-12 against the model's own predicted leak
  1.134e-09; libration swing 6.232 → 0.140 rad; run D's guaranteed
  capture at 765.2 kyr with a 27.46 yr libration period against the
  pendulum-exact 29.13; run E's seam agreement 1.062e-03;
  711,041,787 CVODE steps and 109,388 re-anchorings.
- Test 2 (5/5 runs SUCCESS): GR precession **42.983 ″/century** vs the
  analytic 42.983 (rel. diff 7.193e-13); forced-eccentricity amplitude
  4.5438e-03 and secular period 808.8 kyr, both equal to the
  Laplace–Lagrange predictions; restart at 4.4615 Myr; 16-branch sweep
  5/16 captured; canonical capture at 4.6781 Myr; settled mean ratio
  **1.5000004081** vs the predicted 1.5000003777 (|diff| 3.05e-08);
  perihelion advance 203.21 ″/century vs predicted 203.22; P_rot
  58.6464 d, solar day 175.9463 d; 687,068,258 CVODE steps.

These differ from the macOS project's committed run in the last digits
(for example capture at 4.6501 vs 4.6499 Myr, sweep 5/64 vs 10/64)
because `mercury_rs` evaluates its tidal and triaxial torques with the
host libm (UCRT here, Apple libm there); CVODE itself runs on the
deterministic vendored engine, and every assertion passes on both
platforms. The side-by-side Windows-vs-macOS comparison is §7 ("Windows
11 re-verification") of
[PLANET_MERCURY_PROVENANCE.md](PLANET_MERCURY_PROVENANCE.md), whose
§1–§4 record the sub-project's origin, physics sources and deviations
register; the Windows logs are listed in
[VERIFICATION_WIN11.md](VERIFICATION_WIN11.md).

**Run it yourself.** From the repository root, in PowerShell:

```powershell
# 1. build and test the compute program (a few minutes the first time)
cd planet_Mercury/mercury_rs; cargo build --release; cargo test --release; cd ../..
.\planet_Mercury\mercury_rs\target\release\mercury_rs.exe print-config

# 2. the independent cross-check (builds the rebound ports too)
cd planet_Mercury/mercury_crosscheck; cargo run --release; cd ../..

# 3. a private Python environment with JupyterLab (once, about a minute)
python -m venv planet_Mercury/notebook/.venv
planet_Mercury\notebook\.venv\Scripts\python.exe -m pip install jupyterlab

# 4a. interactive: open the notebook and press Shift+Enter cell by cell
#     (roughly an hour of computing; the last cell opens the browser page)
cd planet_Mercury/notebook
.venv\Scripts\python.exe -m jupyterlab mercury_tidal_locking.ipynb
cd ../..                                  # after closing JupyterLab

# 4b. headless (grading) run instead: execute, then audit the structure
cd planet_Mercury/notebook                # (from the repository root)
$env:MERCURY_NO_BROWSER = "1"
python run_notebook.py mercury_tidal_locking.ipynb
python check_notebook.py mercury_tidal_locking.ipynb
python run_notebook.py mercury_test2_jupiter_gr.ipynb
python check_notebook.py mercury_test2_jupiter_gr.ipynb
cd ../..

# 5. the browser pages (committed, so they open without any run)
Start-Process planet_Mercury\gui\mercury_orbit.html
Start-Process planet_Mercury\gui\mercury_test2.html
# rebuild them from a fresh database:  cd planet_Mercury; python gui/bake_page.py; python gui/bake_page2.py; cd ..
```

Git Bash forms where they differ: join steps with `&&` instead of `;`
(`cd planet_Mercury/mercury_rs && cargo build --release && cargo test
--release && cd ../..`); the venv's programs are
`planet_Mercury/notebook/.venv/Scripts/python.exe` (same `Scripts`
folder, forward slashes); the headless run is
`MERCURY_NO_BROWSER=1 python run_notebook.py mercury_tidal_locking.ipynb`;
open a page with `start planet_Mercury/gui/mercury_orbit.html`. The
runner writes the executed notebook back only if every cell succeeded;
`check_notebook.py` must end with `all structure rules pass`.

## The notebook encyclopedia (SolveIt_Notebooks_for_rust)

[`SolveIt_Notebooks_for_rust/`](SolveIt_Notebooks_for_rust) gathers
**147 notebooks** — every project-authored notebook in this repository,
each copied into its own tagged subfolder beside a companion
`player_<tag>.py` — and builds an encyclopedia out of them
(`ENCYCLOPEDIA.md`, `ENCYCLOPEDIA.html`, `ENCYCLOPEDIA.pdf`: one entry per
notebook with its description, execution verdict, GUI image and every
pointer to source, paired scene, movie, database and player). Seven
topics:

| topic folder | notebooks | what |
|---|---|---|
| `01_planet_mercury_tidal_locking` | 2 | the two Mercury notebooks above |
| `02_solveit_worked_examples` | 38 | SolveIt.md's worked examples 1–38 |
| `03_dynamics_and_routh` | 59 | the dynamic notebooks, the 34 Routh problems among them |
| `04_collisions` | 12 | the collision scripts 01–12 |
| `05_mechanism_videos` | 13 | one per recorded video scene |
| `06_rust_compiled_examples` | 6 | the self-checking Rust examples |
| `07_nbody_rebound_rust` | 17 | the REBOUND / REBOUNDx notebooks |

**The header-cell contract.** The **first cell** of every copy is a
markdown "Encyclopedia header" that makes the notebook self-sufficient
for a reader who has never used the program, in PowerShell from the
repository root: §1 the exact commands to execute it (the one-time engine
and sidecar builds, the batch execution with `run_copy.py`, and the
interactive `jupyter lab` form); §2 how to display its browser GUI; §3 its
movies (or how to screen-record the live scene with Win+Alt+R); §4 how to
open its SQLite database with `python -m sqlite3` and how to export its
data — and the full player script inline, **byte-identical** to the
shipped `player_<tag>.py` (the verifier checks that). Each player answers
`run`, `gui`, `jump` (to the key event), `capture out.png` and
`data <outdir>`.

**The pipeline** (`_tools/`): `gen.py` copies the canonical notebooks,
prepends the header cells and writes the players and `manifest.json`;
`run_copy.py` executes a copy family-aware and writes its real outputs
back; `shoot.py` captures one GUI screenshot per GUI-family entry with
headless Google Chrome; `encyclo.py` builds the three encyclopedia files;
`verify.py` runs six mechanical gates — V1 manifest has 147 entries with
topic counts 2/38/59/12/13/6/17, V2 every copy exists and its first cell
is the tagged header, V3 every player compiles and matches its embedded
copy, V4 every code cell carries an execution count, V5 every GUI-family
entry has a non-trivial PNG (and the rust/rebound entries none), V6
`ENCYCLOPEDIA.md` names every tag exactly once.

**Measured here.** `run_copy.py`: 02_solveit **38/38 ok**, 03_dynamics
**59/59 ok**, 04 + 05 **25/25 ok**, 06_rust **6/6 ok**, 07_rebound
**17/17 ok** (`_tools/exec_posim_0..3.log`, `exec_rebound.log`); the two
Mercury copies, re-executed after the runner learned to decode
`mercury_rs` output as UTF-8 (a Windows cp1252 fix): `_tools/exec_mercury.log`
records `ok … mercury_tidal_locking.ipynb (18 cells)`, `1 ok, 0 failed`
and `_tools/exec_mercury2.log` records `ok …
mercury_test2_jupiter_gr.ipynb (17 cells)`, `1 ok, 0 failed` — **147/147
copies ok** (`encyclo.py` takes the last verdict per tag over
`exec_*.log` in filename order). `shoot.py --workers 4`: **124 captured,
0 failed** (38 solveit + 59 dynamic + 12 collision + 13 video + 2
mercury; the 6 rust and 17 rebound entries have no browser GUI by
design; smallest image 18,039 bytes). The final `encyclo.py` pass
(after the Mercury re-runs; commit `5d87e8d`) produced `ENCYCLOPEDIA.md`
162,628 characters, `ENCYCLOPEDIA.html` 187,017 characters and
`ENCYCLOPEDIA.pdf` 1,364,723 bytes (about 46 pages, printed by headless
Chrome), header line "147/147 green", all 147 entries "Executed: PASS",
and `verify.py` reported **ALL VERIFY GATES PASS (147 notebooks)** (the
earlier dry run, before the Mercury re-runs, had given 162,608 /
186,997 characters and 1,364,749 bytes with the same verdict line but a
"146/147 green" header from the stale interrupted-run log).

**Rebuild everything**, from the repository root in PowerShell:

```powershell
# one-time builds
cargo build --release -p posim
cd dataset_tools; cargo build --release; cd ..
cd planet_Mercury/mercury_rs; cargo build --release; cd ../..

python SolveIt_Notebooks_for_rust/_tools/gen.py         # copies + header cells + players + manifest
python SolveIt_Notebooks_for_rust/_tools/run_copy.py SolveIt_Notebooks_for_rust/02_solveit_worked_examples/solveit_01_elastic_head_on/solveit_01_elastic_head_on.ipynb
                                                        # execute one copy — any copy's path (repeat per notebook)
python SolveIt_Notebooks_for_rust/_tools/shoot.py       # capture every GUI image (headless Chrome)
python SolveIt_Notebooks_for_rust/_tools/encyclo.py     # build ENCYCLOPEDIA.md / .html / .pdf
python SolveIt_Notebooks_for_rust/_tools/verify.py      # the six gates

# one notebook's player
python SolveIt_Notebooks_for_rust/02_solveit_worked_examples/solveit_01_elastic_head_on/player_solveit_01_elastic_head_on.py gui
python SolveIt_Notebooks_for_rust/02_solveit_worked_examples/solveit_01_elastic_head_on/player_solveit_01_elastic_head_on.py capture out.png
```

(Git Bash: the same lines, with `cd dataset_tools && cargo build
--release && cd ..` and `cd planet_Mercury/mercury_rs && cargo build
--release && cd ../..`.) Everything is standard-library Python plus the
pure-Rust engine, except headless capture and the PDF (Google Chrome at
`C:\Program Files\Google\Chrome\Application\chrome.exe`, Edge as the
fallback) and the rebound family, which needs `nbformat`, `nbclient`,
`ipykernel` (and `matplotlib` for its `shearing_sheet` notebook) either
in `planet_Mercury/notebook/.venv` or in the `python` that runs
`run_copy.py`. The reader's page is
[SolveIt_Notebooks_for_rust/README.md](SolveIt_Notebooks_for_rust/README.md);
the encyclopedia itself is
[SolveIt_Notebooks_for_rust/ENCYCLOPEDIA.md](SolveIt_Notebooks_for_rust/ENCYCLOPEDIA.md).

## Documentation

- **[SolveIt.md](SolveIt.md) / [SolveIt.pdf](SolveIt.pdf)** — the
  complete solution guide, written for a reader who has never used the
  program, with **thirty-eight** fully documented worked examples
  (scripts in [`scripts/solveit/`](scripts/solveit); examples 20–38 are
  the SQLite dataset / SQL / movie / curvilinear-coordinate examples).
  All 38 SolveIt scripts and all 12 collision scripts ran headless here:
  50/50 exit 0 (logs/solveit_and_collision_scripts.log).
- **[grammar.md](grammar.md) / [grammar.pdf](grammar.pdf)** — the
  complete command-language and notebook specification: the lexer, the
  full EBNF, the type system, every command (the SQLite dataset, SQL
  and movie families in §5.14, with a complete worked session in §13),
  the stack machine, the
  SUNDIALS 7.8.0 engine, the browser videos, and **eighteen** worked
  examples.
- **Windows 11 port record** —
  [PORT_WIN11_PROVENANCE.md](PORT_WIN11_PROVENANCE.md) (what was done
  and why, both Windows ports), [VERIFICATION_WIN11.md](VERIFICATION_WIN11.md)
  (every gate and its measured result),
  [WINDOWS_PORT_COMPLETE.md](WINDOWS_PORT_COMPLETE.md) (the completion
  summary), and
  [REBOUND_REBOUNDX_WIN11_PROVENANCE.md](REBOUND_REBOUNDX_WIN11_PROVENANCE.md)
  (the N-body ports' C-reference gates on Windows).
- **The dataset layer** — [DATASETS_COMPLETE.md](DATASETS_COMPLETE.md)
  (the SQLite recording layer end to end) and
  [SQLITE_DATASETS_PROVENANCE.md](SQLITE_DATASETS_PROVENANCE.md) (how the
  sidecar and the dataset commands were built and verified).
- [PLANET_MERCURY_PROVENANCE.md](PLANET_MERCURY_PROVENANCE.md) — the
  Mercury sub-project: its import record (physics sources, deviations
  register, both tests; §1–§4 are the macOS-era record) and the dated §7
  Windows 11 re-verification with this machine's figures beside the
  macOS ones.
- The five rebound provenance pages — one per notebook whose bounces are
  verified against closed forms:
  [REBOUND_PROVENANCE_dynamic_bouncing_ball_restitution.md](REBOUND_PROVENANCE_dynamic_bouncing_ball_restitution.md),
  [REBOUND_PROVENANCE_dynamic_routh_p1_geometric_progression.md](REBOUND_PROVENANCE_dynamic_routh_p1_geometric_progression.md),
  [REBOUND_PROVENANCE_dynamic_spinning_target.md](REBOUND_PROVENANCE_dynamic_spinning_target.md),
  [REBOUND_PROVENANCE_rust_bouncing_ball_restitution.md](REBOUND_PROVENANCE_rust_bouncing_ball_restitution.md),
  [REBOUND_PROVENANCE_solveit_04_restitution_ladder.md](REBOUND_PROVENANCE_solveit_04_restitution_ladder.md).
- [SolveIt_Notebooks_for_rust/README.md](SolveIt_Notebooks_for_rust/README.md)
  and [SolveIt_Notebooks_for_rust/ENCYCLOPEDIA.md](SolveIt_Notebooks_for_rust/ENCYCLOPEDIA.md)
  — the notebook encyclopedia.
- [PORT_7.8.0_PROVENANCE.md](PORT_7.8.0_PROVENANCE.md) — the 7.8.0
  upgrade: the API delta call by call, and the evidence that nothing
  else moved.
- [physical_object_simulator.md](physical_object_simulator.md) /
  [physical_object_simulator.pdf](physical_object_simulator.pdf) — the
  predecessor user guide, with fourteen more worked examples.
- [scene_info.md](scene_info.md) / [scene_info.pdf](scene_info.pdf) —
  the graphical scene window: the simulator research survey, the
  protocol, and the UI.
- [collision_detection.md](collision_detection.md) /
  [collision_detection.pdf](collision_detection.pdf) — the collision
  science reference, with documented example scripts in
  `scripts/collisions/` (01–12).
- [ARCHITECTURE.md](ARCHITECTURE.md) — module responsibilities and
  pinned cross-module contracts (solver driving §3.3, the video
  recorder §7).
- [CLAUDE.md](CLAUDE.md) — working rules for contributors and agents,
  including the Windows traps (cp1252, CRLF, `posim.exe` probing order).
- **Lineage pages, kept verbatim from the earlier platforms** (their
  numbers were measured there, not here):
  [PORT_MACOS_PROVENANCE.md](PORT_MACOS_PROVENANCE.md),
  [VERIFICATION_MACOS.md](VERIFICATION_MACOS.md),
  [MACOS_PORT_COMPLETE.md](MACOS_PORT_COMPLETE.md),
  [REBOUND_REBOUNDX_MACOS_PROVENANCE.md](REBOUND_REBOUNDX_MACOS_PROVENANCE.md)
  (macOS / Apple Silicon); [EXPORT_PROVENANCE.md](EXPORT_PROVENANCE.md),
  [CLEANROOM_PROVENANCE.md](CLEANROOM_PROVENANCE.md),
  [REFINE_PROVENANCE.md](REFINE_PROVENANCE.md) and
  [PORT_7.8.0_PROVENANCE.md](PORT_7.8.0_PROVENANCE.md) (Linux).

The PDFs were rebuilt here with MiKTeX (`pdflatex -interaction=nonstopmode
SolveIt.tex`, run twice; likewise `grammar.tex`, `scene_info.tex`,
`physical_object_simulator.tex`): SolveIt.pdf 49 pages, grammar.pdf 74
pages, scene_info.pdf 15 pages, physical_object_simulator.pdf 19 pages,
zero errors.

## Browser videos

Thirteen recorded runs you can open offline — no server, no CDN, nothing
fetched. Scrub, orbit, and read the conserved quantities off whichever
frame you stopped on. Open one with `Start-Process videos\kepler_ellipse.html`
(Git Bash: `start videos/kepler_ellipse.html`), or double-click it.

| video | what to watch | measured over the recording |
|---|---|---|
| [videos/kepler_ellipse.html](videos/kepler_ellipse.html) | the speed swinging between perihelion and aphelion on an `e = 0.6` ellipse | \|dE\|/E = 9.8e-8, \|dL\|/\|L\| = 1.3e-7 |
| [videos/tumbling_racket.html](videos/tumbling_racket.html) | the Dzhanibekov flip: a torque-free cuboid spun about its **intermediate** axis turns over, and over | \|d\|L\|\|/\|L\| = **0 exactly**; \|dE\|/E = 6.4e-9 |
| [videos/box_of_shapes.html](videos/box_of_shapes.html) | a cylinder, a disk and a cuboid rattling in a rigid `BOX 4`; gold arrows are the analytic contact normals, sized by impulse | 36 collision events, \|dE\|/E = 3.4e-16 |
| [videos/double_pendulum_hinges.html](videos/double_pendulum_hinges.html) | **two `HINGE` joints** made into the chaotic double pendulum; gold rings are the joints | the joints hold to \|g\| = 5.6e-8 |
| [videos/universal_joint.html](videos/universal_joint.html) | a **`UNIVERSAL` joint** carrying a driven shaft's rotation to a second shaft; the bend flattens straight and folds back, and the speed across the joint swings with it | the bend stops at cos b = 0.6000004 against a geometric bound of exactly 0.6; joints hold to \|g\| = 4.0e-7 |
| [videos/ball_joint_chain.html](videos/ball_joint_chain.html) | four links on **`BALL` joints**, whirling as they collapse out of the plane they started on | joints hold to \|g\| = 3.3e-9; \|z\| runs from exactly 0 to 1.7147 |
| [videos/rod_pendulum_chain.html](videos/rod_pendulum_chain.html) | four bobs on four **`CONSTRAIN` rods** — one row each, the cheapest linkage there is — going chaotic | run continuously at the default tolerance, \|g\| = 5.4e-15 |
| [videos/spinning_top.html](videos/spinning_top.html) | a top held at its tip by a **`BALL` joint**, precessing under gravity | 1.020440 rad/s vs the exact 1.020408 — 3 parts in 10⁵ |
| [videos/gyroscope_gimbal.html](videos/gyroscope_gimbal.html) | a rotor in **two gimbal rings** on three perpendicular `HINGE` axes; push it one way, it moves another | `L·ŷ` conserved to 1.4e-14 |
| [videos/cardan_compass.html](videos/cardan_compass.html) | the same two rings with a **pendulous** bowl: a ship's compass held level by its own weight | two physical-pendulum periods matched, 1.878587 and 2.307339 s |
| [videos/cardan_gear.html](videos/cardan_gear.html) | **Cardan gears**: a wheel inside a ring of twice its radius rolling on a `GEAR` row, rim point on a straight line | line held to 1.1e-8, against 1.8e-3 with the ratio merely imposed |
| [videos/rack_and_pinion.html](videos/rack_and_pinion.html) | a weight on a **`RACK`** winding up a flywheel, guided by a **`PRISMATIC`** | falls at exactly `g/2`, independent of the pitch radius |
| [videos/piston_crankshaft.html](videos/piston_crankshaft.html) | the **slider-crank**: piston, connecting rod and crankshaft, free-running | follows the exact kinematics to 8.4e-8 |

All thirteen reproduce **byte for byte** on this machine:
`python recorder/src/record_all.py --check` re-records each one and
compares (logs/videos_check.log: `all 13 recordings reproduce byte for
byte`; the recorder's own suites `python recorder/tests/test_units.py`
17 tests OK and `python recorder/tests/test_end_to_end.py` 9 tests OK).

Record your own from any posim script:

```powershell
cargo build --release -p posim
python recorder/src/record_video.py videos/scenes/kepler_ellipse.posim `
     -o mine.html --frames 360 --dt 0.02 --title "..."
```

(Git Bash: the same command on one line, or with `\` instead of the
PowerShell backtick as the line-continuation character.)

Every advance in a recording is a real SUNDIALS step — the recorder
drives `posim --machine` and asks it to `step`. It is a camera, not a
physics engine.

## Some beautiful examples from Routh

The two title pages, transcribed:

> Routh, Part 1:
> A TREATISE ON
> DYNAMICS OF A PARTICLE
> WITH NUMEROUS EXAMPLES
> BY
> EDWARD JOHN ROUTH, Sc.D., LL.D., M.A., F.R.S., &c.,
> HON. FELLOW OF PETERHOUSE, CAMBRIDGE;
> FELLOW OF THE UNIVERSITY OF LONDON.
> CAMBRIDGE
> AT THE UNIVERSITY PRESS
> 1898

> Routh, Part 2:
> THE ADVANCED PART
> OF A TREATISE ON THE
> DYNAMICS OF A SYSTEM OF
> RIGID BODIES.
> BEING PART II. OF A TREATISE ON THE WHOLE
> SUBJECT.
> WITH NUMEROUS EXAMPLES.
> BY
> EDWARD JOHN ROUTH, Sc.D., LL.D., F.R.S., &c.
> HON. FELLOW OF PETERHOUSE, CAMBRIDGE;
> FELLOW OF THE SENATE OF THE UNIVERSITY OF LONDON.
> SIXTH EDITION, REVISED AND ENLARGED.
> London:
> MACMILLAN AND CO., LIMITED
> NEW YORK: THE MACMILLAN COMPANY
> 1905

Thirty-four of Routh's problems are solved here as runnable notebooks —
seventeen from each Part (a first pair, then sixteen more from each book).
Every one derives the closed-form answer in its own header and
then checks it against the integrator, so the numbers below are measured, not
quoted. Launch any of them by name, from the repository root:

```powershell
tools\posim_notebook.cmd routh_p1_hodograph_circle
```

(Git Bash: `tools/posim_notebook routh_p1_hodograph_circle`.) The
launcher builds `posim.exe` if needed and opens the notebook in the
REPL; `tools\posim_notebook.cmd --list` names all 59 dynamic notebooks.

### Part I — *Dynamics of a Particle* (1898)

| notebook | Arts. | what it shows | measured |
|---|---|---|---|
| `routh_p1_sphere_exchange` | 85, 87 | equal elastic spheres exchange velocities; `m = e·m'` stops the striker dead | `v = 0` exactly, momentum 1 |
| `routh_p1_geometric_progression` | 88 | masses in geometric progression give velocities in geometric progression | 1, 2/3, 4/9, 8/27; `\|dE/E\| = 1.1e-16` |
| `routh_p1_oblique_impact` | 89 | equal spheres leave an oblique impact at a right angle | `v₀·v₁ = -8.3e-17` |
| `routh_p1_centre_of_gravity` | 92 | impacts cannot move the centre of gravity | 5.0e-15 from `P/M·t` through 8 impacts |
| `routh_p1_three_projectiles` | 158 | the plane through three projectiles stays parallel to itself | normal fixed to 1e-15 while the triangle grows 4× |
| `routh_p1_parabola_of_safety` | 159–160 | two trajectories reach any point inside the envelope | both arcs within 3.6e-15; every clearance positive |
| `routh_p1_expanding_sphere` | 167 | equal speeds in all directions give an expanding sphere | radii 5, 5, 5 then 10, 10, 10 |
| `routh_p1_escape_velocity` | 312, 335 | the velocity from infinity, and the three conics | `E` = −0.19 / **exactly 0** / +0.21 |
| `routh_p1_equal_periods` | 335 | speed alone fixes the orbit's size, hence its period | four eccentricities 0.87→0.21, one period, all home within 1e-9 |
| `routh_p1_two_trajectories` | 339 | two orbits of one speed reach the same target | 1.3e-6 and 9.0e-7 |
| `routh_p1_kepler_equation` | 342–346 | Kepler's equation and the equation of the centre | radius predicted to 1e-11 |
| `routh_p1_lambert_theorem` | 350–355 | the time depends only on chord, `r₁+r₂` and `a` | two dissimilar ellipses, both 1.3453158122479596 |
| `routh_p1_hodograph_circle` | 394–398 | the hodograph of a Kepler orbit is a circle | radius 1.25 while the speed swings 2 → 0.5 |
| `routh_double_star_period` | 400 | a double star's period depends only on the **sum** of the masses | ratios 1:1, 3:1, 19:1 all close to ~1e-11 |
| `routh_p1_equilateral_three_body` | 407–408, 412 | Lagrange's equilateral solution, and why it cannot hold | home to 3e-10; nudged 0.001 → sides 0.405 / 1.802 / 1.409 |
| `routh_p1_collinear_three_body` | 409–412 | Euler's collinear solution, and its instability | home to 6e-11; nudged → spacings 3.44 / 0.34 |
| `routh_p1_apsidal_symmetry` | 419–420 | an apsidal radius divides the orbit symmetrically | `r(1) = r(2π−1)` to 3e-9; apses at exactly 0.4 and 1.6 |

### Part II — *Advanced Rigid Dynamics* (1905)

| notebook | Arts. | what it shows | measured |
|---|---|---|---|
| `routh_p2_two_quadrics` | 140–142 | the two quadrics whose intersection is the polhode | `G` bit-identical, `E` to 12 digits, `\|ω\|` wanders 3.017 → 3.407 |
| `routh_p2_invariable_line` | 141 | the invariable line is fixed in space; the instantaneous axis is not | `normalize(L)` bit-identical; `normalize(ω)` swings |
| `routh_p2_poinsot_rolling` | 143 | the momental ellipsoid rolls on a fixed plane | `ω·L̂ = 2E/G` constant to 12 digits |
| `routh_p2_thin_rod` | 144 | a thin rod's momental ellipsoid is a circular cylinder | a 50× axial spin carries 0.0075 of `L` |
| `routh_p2_impulsive_couple` | 146 | an impulsive couple tilts the invariable line | `\|L'\| = √(G²+25)`; tilt 21.2785° |
| `routh_p2_fixed_couple` | 148 | a couple whose axis is fixed in space | `L(t) = (0,0,2t)` **exactly** |
| `routh_p2_polhode_quarter_period` | 150a | the quarter period as an elliptic integral | `k² = 0.25` exactly; `ω₃` vanishes at `K/λ` to 6e-12 |
| `routh_rectangle_diagonal` | 150b Ex. 1 | a rectangle spun about one diagonal ends on the other | at `T = 4.285129705594264`, error 1.2e-12 |
| `routh_p2_period_ratio` | 150b Ex. 2 | the ratio of the two periods is independent of the disturbance | sixfold different wobbles, identical period |
| `routh_p2_mean_axis_instability` | 155 | the three principal axes have unequal stability | mean axis reverses; the others stay within 0.005 and 0.018 |
| `routh_p2_spin_stabilisation` | 156 | rapid rotation steadies a body | one kick at n = 1, 3, 9 → 20.171°, 8.326°, 2.854° |
| `routh_p2_rolling_cones` | 157–159 | the body and space cones roll on one another | both half-angles constant to 13 digits |
| `routh_p2_principal_axes_in_space` | 176–179 | `cos α = Aω₁/G` — the invariable line seen from the body | fixed in space, sweeping `(1.5,12.75,0.125)` → `(11.52,−4.90,2.86)` in the body |
| `routh_p2_uniaxal_precession` | 180–183 | steady precession when `A = B` — two periods at once | space 4.683, body 3.491; `\|ω\|` constant |
| `routh_p2_separatrix` | 184–185 | `G² = BT` decides which axis the polhode encircles | +0.3 / 0 / −0.337 pins `ω_x`, nothing, `ω_z` |
| `routh_p2_correlated_bodies` | 192–195 | confocal ellipsoids of gyration | `ω − ω' = kL` exact at correspondence, with the drift measured |
| `routh_p2_sylvester_time` | 196–198 | the clock Poinsot's rolling cannot supply | rate `kG = 1.283854061020956` |

Provenance and the full run record for every notebook — including the other 25
in the directory — are in
[dynamic_notebooks/MANIFEST.md](dynamic_notebooks/MANIFEST.md); the reader's
catalogue is [dynamic_notebooks/README.md](dynamic_notebooks/README.md).

## The Index of Functions

`index_of_entities.html` is a browsable catalog of **every named entity in this
repository** — 6,429 of them in the catalog regenerated here (499 Tier A +
599 Tier B + 5,331 Tier C) — with a definition, the `file:line` where it is
defined, its complete syntax, and examples you can paste into a notebook and
run. Open it directly; it needs no server and fetches nothing.

```powershell
Start-Process index_of_entities.html
```

(Git Bash: `start index_of_entities.html`; or double-click the file.)

Keep `catalog-c.js` beside it: the page loads that second payload on demand
when you open a bucket or search, which is what keeps the main file small
instead of carrying the 5.7 MB of Tier C inline.

| tier | what | entries | examples |
|---|---|---|---|
| A | the notebook surface — commands, keywords, field paths, builtins, types, notebooks, worked examples | 499 | 1,495 examples, of which the **1,365** runnable posim and machine-mode fragments were **all executed** here |
| B | the first-party Rust API — `physical_object`, `special_functions`, `quantum`, `posim` | 599 | **258** snippets, **all compiled** here |
| C | the vendored `sundials_rs` workspace (SUNDIALS 7.8.0: CVODE, CVODES, IDA, IDAS, KINSOL, ARKODE, core) | 5,331 | linked to the shipped example programs that call them |

**Every example is checked, and the page says which kind of check it got:**
a posim fragment is *executed* (`posim --script`, output captured), a
machine-mode fragment is *executed* through the JSONL protocol, a Rust snippet
is *compiled*, and a shell command or a quoted transcript reads "run this
yourself" because it points at a program verified elsewhere rather than one
this page ran. Measured on this machine (2026-09-09):
`python tools/verify_index_examples.py` **PASS 1365/1365 (100.0%)** in 35 s
(logs/verify_index_examples.log) and `python tools/verify_tierb_examples.py`
**COMPILED 258/258 (100.0%)** (logs/verify_tierb_examples.log) — the
catalog's own stamp reads 1,623 of 1,623 verified. (The macOS edition's
"1,141 + 24" and "1,435" arithmetic described a smaller catalog; the
numbers above are the ones counted and run here.)

The documented worked examples carry their real transcripts. Where the
typed half replays, it is **executed** and shown beside the document's own
published output so you can compare; where it cannot — a deliberate refusal
that *is* the lesson, an elided session, or a `SCENE REVERSE` that depends on
wall-clock playback — the transcript is quoted and the reason is stated.

Navigation is A–Z / 0–9 / special-character buckets, with search and kind
filters. The whole thing is operable from the keyboard — press <kbd>?</kbd> for
the map — and has BACK/FORWARD history that agrees with the browser's own
buttons. You can edit, add and hide entries; changes live in a local overlay
you can export and re-import, and the generated catalog is never mutated.

### What it does not claim

The status page inside the app is the authority, and it is blunt:

- **Every Tier-A entry is `complete`** (499 of 499 carry examples). Of the
  599 Tier-B entries, 258 are `complete` (a compiled snippet each), 329 are
  `reference` — mostly in `posim`, which is a binary crate with no lib
  target, so nothing can `use posim::…` and no Rust snippet is possible;
  those link instead to the notebook command each one implements — and 12
  internal items (`JacBlock`, `EquilibriumReport`, `BodySnap`,
  `ParamSensitivity` and its `max_position_sensitivity`,
  `SensitivityReport`, `run`, `DATASET_DIR`, `DatasetHandle::append_raw`,
  `bodies_payload`, `MOVIE_DIR`, `movie_path`) still carry status `stub`
  in the catalog regenerated here.
- **5,331 Tier-C entries carry status `reference`**, not `complete`.
  `sundials_rs` is a faithful translation of a C library whose API is
  `&mut CVodeMem` plus a context, a matrix, a linear solver and callbacks; a
  one-line snippet would misrepresent how any of it is reached. Instead each
  entry links to the shipped example *programs* that actually call it — those
  are diffed byte-for-byte against the upstream C references
  ([sundials_rs/VERIFICATION.md](sundials_rs/VERIFICATION.md)).
- The machine-mode JSON ops carry real executed examples (`posim --machine`
  is as runnable as the notebook), and each shape has a rung that *checks*
  its documented inertia formula against the printed tensor rather than
  restating it.
- Captured output has genuine run-to-run variation normalised — the
  OS-assigned scene port becomes `<port>`, playback counters become
  `<varies>`. Solver step counts are **not** normalised: those are
  deterministic, and they are the anchors the documentation pins.

[index_data/DIVERGENCES.md](index_data/DIVERGENCES.md) records seven places
(D1–D7) where the prose and the code disagreed, each settled by running a
probe rather than by reading; six are marked fixed, and the index carries the
code's behaviour.

### Rebuilding it

```powershell
python tools/extract_rust_items.py > index_data/rust_items.jsonl
python tools/build_commands.py; python tools/build_tierb.py; python tools/build_tierc.py
python tools/build_catalog.py; python tools/build_app.py
python tools/verify_index_examples.py      # runs every posim / machine-mode fragment
python tools/verify_tierb_examples.py      # compiles every Rust snippet
```

(Git Bash: the same lines with `&&` in place of `;`. Run the first line
in PowerShell 7 (`pwsh`) or Git Bash — the older Windows PowerShell 5
writes `>` redirections as UTF-16, which the index tools do not read.)

## Quick start

```powershell
cargo run                 # notebook REPL (type HELP)
cargo test --workspace    # all tests (638 here)
cargo run -p physical_object --release --example kepler_orbit
cargo run -p physical_object --release --example outer_solar_system
cargo run -p physical_object --release --example tumbling_body
cargo run -p physical_object --release --example charged_in_b_field
cargo run -p posim -- --script scripts/collisions/12_two_dumbbells.posim
cargo run -p posim -- --script my_session.posim
cargo run -p posim --release -- --notebook dynamic_notebooks/kepler_orbit.posim
cargo run -p posim -- --machine   # JSON protocol for front ends
```

(Identical in Git Bash.) To run without opening browser windows, set
`$env:POSIM_NO_BROWSER = "1"` first (Git Bash: prefix the command with
`POSIM_NO_BROWSER=1`). To keep a log while you watch, append
`2>&1 | Tee-Object run.log` (Git Bash: `2>&1 | tee run.log`). The
physics byte-identity gate needs Git Bash:
`bash tools/win_verify_physics.sh` (from PowerShell:
`& "C:\Program Files\Git\bin\bash.exe" tools/win_verify_physics.sh`).

Example session:

```
In[1]:= new sphere { mass = 2, radius = 0.5, position = [0, 10, 0], velocity = [1, 0, 0] }
Out[1]= obj0
In[2]:= set system.gravity = [0, -9.81, 0]
In[3]:= step 1
Out[3]= t = 1 (advanced by 1, 12 solver steps)
In[4]:= get obj0.position
Out[4]= [1, 5.095000000000006, 0]
In[5]:= method sprk leapfrog_2_2 0.001
In[6]:= help
```
