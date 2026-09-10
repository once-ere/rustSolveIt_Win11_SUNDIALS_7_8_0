# Mercury Tidal Locking — Complete Instructions for Teachers and Students

**What this is:** the full, self-contained guide to running (and re-creating) the
`planet_Mercury` project on a Windows 11 PC: a Jupyter notebook that simulates how
the planet Mercury became tidally locked to the Sun in its strange 3:2 spin-orbit
resonance — spinning exactly three times for every two trips around the Sun.
Everything you need is in this one document. Commands are given for **PowerShell**
(the terminal Windows 11 opens when you right-click the Start button and choose
*Terminal*); where the **Git Bash** form differs it follows in parentheses.

Words used below, explained once: **ODE** = ordinary differential equation (a rule
for how fast something changes); **CSV** = comma-separated values (a data table saved
as plain text); **SQL** = Structured Query Language (the standard mini-language for
asking a database questions); **BDF** = backward differentiation formulas (the solver
method used, built for stiff problems); **libration** = the gentle rocking of a locked
angle around its resting point — a librating resonance angle IS the lock.

---

## Part A — What you will have when you follow these instructions

A Jupyter notebook, `notebook/mercury_tidal_locking.ipynb`, that on one top-to-bottom
run:

1. **Explains** (inside the notebook itself, in plain English) how the Sun's tides
   slowly braked Mercury's spin, and why Mercury got caught at exactly 3 spins per
   2 orbits.
2. **Computes** that entire history by running a pure-Rust program (`mercury_rs.exe`)
   that integrates five ODEs with the SUNDIALS 7.8.0 CVODE solver (BDF method, Newton
   iteration, dense linear solver — the rustSolveIt engine's only integration
   backend; here the engine is the Windows port
   once-ere/SUNDIALS_7_8_Rust_port_for_Windows11, vendored in this repository).
   Python never integrates anything.
3. **Stores** every result in a real database file (`data/mercury_orbit.sqlite3`) and
   **retrieves** results from it with seven documented SQL queries.
4. **Checks itself** against the observed Mercury: a year of 87.969 Earth days, a
   rotation of 58.646 Earth days (exactly 2/3 of the year), and a 175.938-day solar
   day (exactly two Mercury years, sunrise to sunrise). The notebook stops with an
   error if any check fails.
5. **Displays** the whole story as one interactive page in your web browser
   (`gui/mercury_orbit.html`): the orbit with a spinning Mercury and its long-axis
   "handle," a spin/orbit-ratio dial that settles on exactly 1.5, the libration plot,
   and the period and angular-momentum histories, with play/scrub controls.

**What the shipped, verified pipeline actually measured on this Windows 11 machine
(2026-09-09)** — your re-run on Windows 11 reproduces these. The project was first
built and measured on a macOS (Apple Silicon) machine; where that record differs it
is given in parentheses, and Part H explains why the two differ and why it does not
matter:

| Result | Value |
|---|---|
| Spec-literal braking after 10 Myr (run A) | spin barely moves: 181.4x → 178.9x the orbital rate (same on macOS) — the real braking takes ~4.7 billion years, so the main runs use a documented 1000x time compression |
| Capture into 3:2 (canonical run) | at 4.6501 million movie-years (≈ 4.65 billion real-equivalent years) (macOS: 4.6499) |
| Measured capture odds (64-branch phase sweep) | 5/64 = 7.8% ± 3.4% per crossing (macOS: 10/64 = 15.6% ± 4.5%; Goldreich–Peale 1966 theory estimates ~7% — same order; the classic formula is approximate, and the capture coin-flip is bit-level path-sensitive, so each platform has its own deterministic tally) |
| Settled spin/orbit ratio (time-averaged) | 1.5000000368 (macOS: 1.5000000498) |
| Final year / rotation / solar day | 87.968 d / 58.651 d / 175.990 d (macOS: 87.968 / 58.642 / 175.906; observed: 87.969 / 58.646 / 175.938; instantaneous samples carry a ~1×10⁻⁴ residual-libration wiggle, while the time-averaged 2/3 lock holds to 8.8×10⁻⁹) |
| Angular-momentum ledger | what the spin lost, the orbit gained: pre-capture total drift 4.0×10⁻¹⁰ (macOS: 3.9×10⁻¹⁰; budget 10⁻⁹, where the model conserves exactly); locked-era drift 8.8×10⁻¹² (macOS: 9.2×10⁻¹²) vs the model's own predicted 1.1×10⁻⁹ secular leak (the spec's handle torque has no orbital back-reaction; the recorded value is further suppressed by 64-bit orbit quantization) |
| Libration decay after capture | the notebook's 12-bin table shows the swing falling 6.23 rad → 0.14 rad over the first 1.2 million years after capture (macOS: 6.22 → 0.15); the crate's own check, spanning the whole locked era, sees 4.57 → 0.06 rad (macOS: 4.58 → 0.06) — the lock settling in |
| Independent cross-check | REBOUNDx-port `tides_spin` despin rate agrees with the Hut formula to 0.25% (relative difference 2.511e-3: measured −3.652928e-18 vs predicted −3.643780e-18 rad/s²; the same agreement as on macOS) — a separate one-command crate outside the notebook: `cd planet_Mercury\mercury_crosscheck; cargo run --release; cd ..\..` (Git Bash: `cd planet_Mercury/mercury_crosscheck && cargo run --release && cd ../..`) prints the agreement, `PASS - crosscheck.despin_rate` and `SUCCESS` (an upstream REBOUNDx notice about a 4.5.0 tidal-potential fix also prints — expected, not an error) |

## Part B — One-time setup (teachers: once per classroom machine)

Requirements: a Windows 11 PC whose x86-64 processor has the FMA instructions
(every recent Intel or AMD desktop/laptop chip does; the engine's deterministic
maths library requires them and the repository's `.cargo/config.toml` switches them
on), the Rust toolchain (`cargo`), `git`, and Python 3. Check all three in
PowerShell:

```powershell
cargo --version
git --version
python --version
```

(Git Bash: the same three commands.) The machine this document was measured on
answered `cargo 1.91.1`, `rustc 1.91.1`, and `Python 3.14.5`.

If any is missing:

- **Rust** — download and run `rustup-init.exe` from https://rustup.rs (no
  administrator rights needed). On Windows it checks for the *Visual Studio C++
  Build Tools* and offers to install them when they are missing: accept. Those
  tools supply the MSVC linker that Rust uses on Windows; `rustc` finds it by
  itself — you never need a "Developer Command Prompt". (If you prefer to install
  them yourself: "Build Tools for Visual Studio", workload *Desktop development
  with C++*. The measured machine used the Visual Studio 2026 Build Tools, MSVC
  14.51.36231.) No C compiler is used to build this project — the Rust crates are
  pure Rust; only the linker is needed.
- **git** — https://git-scm.com/download/win (this also installs Git Bash).
- **Python 3** — https://www.python.org/downloads/windows/; in the installer tick
  **"Add python.exe to PATH"**. The command on Windows is `python` (not
  `python3`). If typing `python` opens the Microsoft Store instead, install from
  python.org as above and open a new terminal.

Close and reopen the terminal after any installation so the new programs are found.

**Step B1 — get the project.** From whatever folder you keep projects in:

```powershell
git clone https://github.com/once-ere/rustSolveIt_Win11_SUNDIALS_7_8_0.git
cd rustSolveIt_Win11_SUNDIALS_7_8_0
```

(Git Bash: identical.) In the clone, `planet_Mercury\` is a top-level folder inside
the repository. The compute crate finds the SUNDIALS solver by a relative path into
the engine that ships with it — specifically, `planet_Mercury\mercury_rs\Cargo.toml`
points two folders up and into `sundials_rs\crates\` (the repository root) — so
keep the folder layout exactly as delivered. (This repository is the Windows 11
port of once-ere/rustSolveIt_macos-silicon_SUNDIALS_7_8_0, itself a port of the
Linux original once-ere/rustSolveIt_Using_SUNDIALS_7_8_0; PowerShell also accepts
forward slashes in every path below.)

**Step B2 — build the compute program** (a few minutes the first time). From the
repository root:

```powershell
cd planet_Mercury\mercury_rs
cargo build --release 2>&1 | Tee-Object $env:TEMP\mercury_build.log
```

(Git Bash: `cd planet_Mercury/mercury_rs` then
`cargo build --release 2>&1 | tee /tmp/mercury_build.log`.) Zero errors and zero
warnings expected; the program is `target\release\mercury_rs.exe`. (Every command
here uses `2>&1 | Tee-Object <logfile>` — Git Bash `2>&1 | tee <logfile>` — so the
full output is both shown and saved — a habit worth teaching.)

**Step B3 — prove the program works before touching the notebook** (still inside
`planet_Mercury\mercury_rs\`):

```powershell
cargo test --release 2>&1 | Tee-Object $env:TEMP\mercury_test.log
cargo run --release -- print-config
cd ..\..
```

(Git Bash: `cargo test --release 2>&1 | tee /tmp/mercury_test.log`,
`cargo run --release -- print-config`, `cd ../..`.) `cargo test` must show a line
reporting **19 passed; 0 failed** (the physics suite; the `Doc-tests mercury_rs`
line saying "0 passed; 0 failed" is a normal empty harness) — every test checks a
pencil-and-paper expectation (Kepler's law, the Hut polynomials, torque signs, exact
angular-momentum closure, the libration frequency, Einstein's 43″/century, the
Laplace coefficients…). `print-config` prints every constant and solver setting;
have students read it aloud.

**Step B4 — install JupyterLab** (the viewer only; the notebook's own code cells use
nothing outside Python's standard library). Keep it in a small private environment
so nothing is installed system-wide — from the repository root:

```powershell
python -m venv planet_Mercury\notebook\.venv
planet_Mercury\notebook\.venv\Scripts\pip.exe install jupyterlab
```

(Git Bash: `python -m venv planet_Mercury/notebook/.venv` then
`planet_Mercury/notebook/.venv/Scripts/pip.exe install jupyterlab`.) One-time,
about a minute. The environment is never "activated" — its programs are called by
their path, so no PowerShell execution-policy change is needed. Deleting the
`.venv` folder uninstalls everything.

## Part C — Running the notebook (students start here)

**Step C1 — open it.** From the repository root:

```powershell
cd planet_Mercury\notebook
.venv\Scripts\jupyter.exe lab mercury_tidal_locking.ipynb
```

(Git Bash: `cd planet_Mercury/notebook` then
`.venv/Scripts/jupyter.exe lab mercury_tidal_locking.ipynb`.) A browser tab opens
with the notebook. If asked to pick a kernel, choose **Python 3 (ipykernel)**.

**Step C2 — run it, top to bottom.** Click the first cell, then press
**Shift+Enter** repeatedly (each press runs one cell and moves on), or use
Run → Run All Cells. The six simulation cells (§6.2–6.7) print progress lines and take minutes
each (budget about an hour for the whole notebook — it integrates millions of years
of physics at 12-digit accuracy, including a 64-branch sweep).

**Step C3 — the finale.** The last computational cell bakes the display page twice,
proves the two bakes byte-identical, and opens `gui\mercury_orbit.html` in your
browser. Press **Space** to play. Watch the spin/orbit dial: it starts near 181 (a
young Mercury spinning every 11.6 hours), falls for millions of years, hesitates at
the 3:2 line — and locks. Use "Jump to capture" and the scrub bar; the libration
plot below shows the resonance angle's circulation turning into a dying rocking —
tidal locking made visible.

Everything is also saved on disk: raw simulation output in
`planet_Mercury\data\runs\`, the database at
`planet_Mercury\data\mercury_orbit.sqlite3`, the page at
`planet_Mercury\gui\mercury_orbit.html`. To open the page again later, from the
repository root:

```powershell
Start-Process planet_Mercury\gui\mercury_orbit.html
```

(Git Bash: `start planet_Mercury/gui/mercury_orbit.html`.) To re-bake it from the
saved data without re-running the notebook: `cd planet_Mercury; python gui\bake_page.py; cd ..`
(Git Bash: `cd planet_Mercury && python gui/bake_page.py && cd ..`).

**Headless / grading re-run** (no browser, fails loudly on any error) — from
`planet_Mercury\notebook\`:

```powershell
$env:MERCURY_NO_BROWSER = "1"
python run_notebook.py mercury_tidal_locking.ipynb
python check_notebook.py mercury_tidal_locking.ipynb
Remove-Item Env:MERCURY_NO_BROWSER
```

(Git Bash: `MERCURY_NO_BROWSER=1 python run_notebook.py mercury_tidal_locking.ipynb`
then `python check_notebook.py mercury_tidal_locking.ipynb`.) Both must end happily:
`ok mercury_tidal_locking.ipynb (18 cells)` and `1 ok, 0 failed` from the runner,
`ok mercury_tidal_locking.ipynb: all structure rules pass` from the auditor — exactly
what this machine printed (the run is recorded in `logs\mercury_notebook1.log` under the repository root).
The runner rewrites the notebook file with the real captured outputs only if every
cell succeeded.

## Part D — What each section of the notebook contains

The cells alternate: a markdown (text) cell explains, then a code cell does exactly
what was explained.

- **§1 What this notebook computes** — the story and the three observed numbers it
  must reproduce.
- **§2 How to run this notebook** — the Shift+Enter instructions (in their
  PowerShell form), repeated inside the notebook so it stands alone.
- **§3 The words used in this notebook** — a one-line glossary (tide, torque,
  resonance, libration, ODE, tolerance, secular…).
- **§4 The physical situation** — the complete model in the notebook's own words:
  point-mass Sun; extended, deformable, lopsided Mercury; the five tracked numbers
  [a, e, M, θ, Ω]; every governing equation including all five Hut eccentricity
  factors; every constant with its meaning; and the honest deviations (the 1000x
  time compression, the two-stage integration, the capture coin-flip) with reasons.
  **§4.3** states the exact solver contract handed to SUNDIALS: CVODE, BDF, Newton,
  dense; relative tolerance 10⁻¹²; absolute tolerances [10⁻³, 10⁻⁶, 10⁻¹⁰, 10⁻¹⁰,
  10⁻¹⁴]; max step 864,000 s.
- **§5 How this notebook talks to the simulator** — the subprocess driver (finds
  `mercury_rs.exe` — or the path in the `MERCURY_BIN` environment variable —
  streams its progress, decoded as UTF-8, and insists on the program's own SUCCESS
  verdict).
- **§6 Building and running the simulation** — one explained cell per run:
  6.1 config echo; 6.2 run A (spec-literal honesty run); 6.3 run B (compressed
  braking to the saved restart at ratio 1.6); 6.4 run C (the 64-branch sweep and the
  measured odds, with error bars — 5/64 on this platform); 6.5 run B-final (the
  first captured branch, continued to the locked present); 6.6 run D (guaranteed
  capture at eccentricity 0.285 — why ovalness is the secret); 6.7 run E (proof the
  two integration stages agree, at the deliberately non-resonant checkpoint 2.7).
- **§7 Storing the results in a database** — creates the five tables (`run`,
  `sample`, `event`, `branch`, `target`) and bulk-loads every CSV; prints row counts
  and a referential-integrity check.
- **§8 Retrieving results from the database** — seven explained SQL queries: the
  present day; the capture time; the libration decay; the orbital-period history;
  the angular-momentum ledger; the measured capture odds; and the traceability query
  (which solver settings produced the canonical history).
- **§9 The verification gauntlet** — every acceptance target asserted in one place;
  one PASS line each, or the notebook stops. On this machine it ends
  `PASS - all_run_verdicts: 6/6 runs ended SUCCESS` and `ALL CHECKS PASSED`.
- **§10 Baking and opening the display page** — deterministic double-bake plus the
  browser launch (skipped under MERCURY_NO_BROWSER=1).
- **§11 What we learned** — the story retold with the measured numbers, plus
  exercises (bigger sweeps; where does guaranteed capture stop?; the excluded
  physics — Jupiter's tugs and Einstein's perihelion correction — is the second
  test, Part G).
- **§12 Name and save this notebook** — an interactive save-dialog cell (batch
  runners skip it).

## Part E — For teachers: how the notebook was authored (make your own!)

1. **One notebook = one complete lesson.** Every explanation lives inside the
   notebook; never "see another file" — restate instead.
2. **Markdown before code, always** — every code cell has a ≥80-character
   explanation cell above it.
3. **Plain Python 3, standard library only** in code cells (`subprocess`, `sqlite3`,
   `csv`, `json`, `pathlib`, `webbrowser`). JupyterLab is only the viewer.
4. **The Rust program does all the physics.** Python runs it, loads, queries,
   checks, displays. New physics goes into `mercury_rs` (Rust), never the notebook.
5. **Real outputs are part of the notebook.** The shipped file was executed
   top-to-bottom by `run_notebook.py`, so every output you see is a genuine capture;
   `check_notebook.py` audits the structure rules mechanically.
6. **Determinism is sacred.** Same inputs → byte-identical outputs: no timestamps,
   fixed number formatting, a recorded phase for the capture coin-flip. The notebook
   is authored by `build_notebook.py` (also deterministic) — editing that builder
   and re-running it (`python build_notebook.py`), then `run_notebook.py`, is the
   full authoring loop for a sibling notebook (a Venus? the Moon?).
7. **Two Windows text rules, both already built in.** Windows Python decodes
   program output as cp1252 by default, which garbles the UTF-8 em-dashes
   `mercury_rs.exe` prints — the driver therefore passes `encoding="utf-8"` to
   `subprocess.Popen`; and Python's text mode would turn `\n` into `\r\n` on
   Windows, so the builder and runner write every file with `newline="\n"` (the
   repository's byte-identity checks depend on LF endings). Keep both if you copy
   the driver.

## Part F — Troubleshooting (verified answers)

| Symptom | Cause and fix |
|---|---|
| `cargo : The term 'cargo' is not recognized` (PowerShell) / `cargo: command not found` (Git Bash) | Install Rust from https://rustup.rs, then close and reopen the terminal |
| `python` opens the Microsoft Store, or `Python was not found` | Install Python 3 from https://www.python.org/downloads/windows/ with "Add python.exe to PATH" ticked; reopen the terminal |
| `error: linker 'link.exe' not found` during `cargo build` | The Visual Studio C++ Build Tools are missing — re-run `rustup-init.exe` and accept their installation, or install "Build Tools for Visual Studio" with the *Desktop development with C++* workload; reopen the terminal |
| `pip install` asks for administrator rights or fails on permissions | Use the Part B4 venv commands exactly (`.venv\Scripts\pip.exe`) |
| `.venv\Scripts\jupyter.exe` is "not recognized" | You are not inside `planet_Mercury\notebook\`, or Part B4 was skipped |
| The driver cell cannot find the binary (`mercury_rs binary not found`) | Run the Part B2 build, or set `$env:MERCURY_BIN = "C:\full\path\to\mercury_rs.exe"` (Git Bash: `export MERCURY_BIN=/c/full/path/to/mercury_rs.exe`) before starting JupyterLab |
| Build error about a missing `sundials_core`/`cvode_rs` path | The relative path from `planet_Mercury\mercury_rs\` to the engine's `sundials_rs\crates\` folder was broken by moving folders — restore the shipped layout |
| A cell raises `RuntimeError: … FAILURE` | The Rust program's self-check failed and its output names the exact check; re-run once, then `cargo test --release` and report |
| Progress lines show `â€”` or `?` where a dash should be | Output is being decoded as cp1252; the shipped driver decodes UTF-8 — if you wrote your own, add `encoding="utf-8"` to `subprocess.Popen` |
| The browser page does not open | Open it with `Start-Process planet_Mercury\gui\mercury_orbit.html` (Git Bash: `start planet_Mercury/gui/mercury_orbit.html`) or double-click the file; under MERCURY_NO_BROWSER=1 no page is expected |
| The notebook takes very long | Expected: about an hour in release mode; if you built without `--release` it can take many hours — rebuild |
| Sweep reports zero captures | Impossible for the shipped, deterministic pipeline on this platform (on Windows 11 it always finds 5/64; the macOS edition found 10/64); when re-creating the pipeline or running on another platform it is a real ~1% possibility — re-run section 6.4 once with `run("sweep", "--branches", "128")` (the documented contingency) |
| Numbers differ from this document in the last digit | They should not on Windows 11 (the pipeline is deterministic on this platform); they DO differ from the macOS values given in parentheses, for the reason in Part H; if a Windows 11 re-run differs from this document after a clean re-run, that IS a reportable finding |

## Part G — Test 2: Jupiter and Einstein (the second notebook)

The folder ships a **second, independent notebook**:
`notebook/mercury_test2_jupiter_gr.ipynb`. It adds the two pieces of physics the
first lesson deliberately left out, and it is just as self-contained — every
explanation is restated inside it.

**What's new in the physics.**

1. **Einstein's correction.** Very close to the Sun, gravity is a whisker
   stronger than Newton's law predicts. The orbit stays an ellipse, but the
   ellipse's long axis slowly swings around — 42.98 arcseconds per century, the
   number that made general relativity famous. The notebook makes the simulator
   reproduce it to better than one part in a thousand with everything else
   switched off (measured here: 42.983 vs 42.983 analytic, relative difference
   7.193e-13 — identical on macOS).
2. **Jupiter's tugs** (in the classic averaged, "secular" form): they turn
   Mercury's ellipse another ~160 arcseconds per century AND make the orbit's
   ovalness breathe up and down by about 0.0045 on a ~809,000-year cycle. Both
   are checked against the pencil-and-paper Laplace–Lagrange values (measured
   here: amplitude 4.5438e-03 and period 808.8 kyr, both equal to the
   predictions — identical on macOS).

**The punchline your students should look for.** With the ellipse itself
turning, a locked Mercury cannot average EXACTLY 1.5 spins per orbit any more —
the lock tracks the moving perihelion, so the settled average is
1.5 + (perihelion turn rate)/(orbital rate) ≈ 1.5000004. The notebook measures
that four-parts-in-ten-million offset and checks it against the prediction:
on this machine the settled mean ratio is **1.5000004081** against the predicted
**1.5000003777** (|difference| 3.05×10⁻⁸; macOS: 1.5000004545, 7.69×10⁻⁸) —
**the lock follows the precessing ellipse, not the stars.** A genuinely
relativistic fingerprint, resolved by a 12-digit integration your class can run.
The other Windows 11 figures of this notebook: restart root at ratio 1.6 found to
1.600000000000e+00 at 4.4615 Myr (macOS: 1.599999999995e+00 at 4.4612); 16-branch
sweep 5/16 captured (macOS: 2/16); canonical capture at 4.6781 Myr (macOS: 4.6779);
perihelion advance 203.21″/century vs predicted 203.22 (same); final year /
rotation / solar day 87.9678 / 58.6464 / 175.9463 d (macOS: 87.9678 / 58.6447 /
175.9314); all checks PASS, 5/5 runs SUCCESS on both.

**How to run it.** Identical to Parts A–C: same build, same JupyterLab
environment, then open `mercury_test2_jupiter_gr.ipynb` and Shift+Enter top to
bottom — or batch, from `planet_Mercury\notebook\`:

```powershell
$env:MERCURY_NO_BROWSER = "1"
python run_notebook.py mercury_test2_jupiter_gr.ipynb
python check_notebook.py mercury_test2_jupiter_gr.ipynb
Remove-Item Env:MERCURY_NO_BROWSER
```

(Git Bash: `MERCURY_NO_BROWSER=1 python run_notebook.py mercury_test2_jupiter_gr.ipynb`
then `python check_notebook.py mercury_test2_jupiter_gr.ipynb`.) Expect
`ok mercury_test2_jupiter_gr.ipynb (17 cells)`, `1 ok, 0 failed` and
`ok mercury_test2_jupiter_gr.ipynb: all structure rules pass` — what this machine
printed (`logs\mercury_notebook2.log` under the repository root). It writes its own database
(`data\mercury_test2.sqlite3`) and bakes its own display page
(`gui\mercury_test2.html`; open it later with
`Start-Process planet_Mercury\gui\mercury_test2.html` — Git Bash:
`start planet_Mercury/gui/mercury_test2.html`; re-bake with
`cd planet_Mercury; python gui\bake_page2.py; cd ..`) — nothing it does touches
the first lesson's files, so the two notebooks can be re-run in any order.

**One honest difference to point out.** The first lesson's angular-momentum
ledger is deliberately ABSENT from test 2: Jupiter's averaged tugs exchange
angular momentum with Jupiter itself, which the model does not track, so a
two-body "books must balance" check would be checking the wrong law. The
notebook (and the simulator's own output) say so — auditing what a model can
and cannot promise is itself the lesson there.

## Part H — This Windows 11 edition versus the macOS record

The project was created and first verified on a macOS / Apple Silicon machine
(2026-08-30) and re-verified on this Windows 11 machine (Windows 11 Pro for
Workstations build 10.0.26200, Intel Core Ultra 9 275HX, rustc/cargo 1.91.1 for
x86_64-pc-windows-msvc, Python 3.14.5) on 2026-09-09. Nothing in the physics, the
constants, the tolerances or the program's runs changed between the two; the only
Windows edits are platform plumbing (the driver looks for `mercury_rs.exe` and
decodes UTF-8; the setup prose is PowerShell; files are written with LF endings).

Yet some measured numbers differ, and a class deserves the reason. `mercury_rs`
computes its tidal and triaxial torques through the maths library of the host
operating system — Microsoft's UCRT on Windows, Apple's libm on macOS — and those
two libraries round a few functions differently in the last binary digit. The
CVODE solver itself is the vendored deterministic engine and is not the source of
any difference. Where the model is smooth, that last digit stays where it is: the
run-A ratio 178.9260, Einstein's 42.983″/century with its 7.193e-13 residual, the
Laplace–Lagrange amplitude and period, and the count of 109388 re-anchorings are
identical on both machines. Near the 3:2 resonance the model is chaotic — the
notebook's own §4 calls the capture a coin-flip decided at the bit level — so the
64-branch sweep tallies 5 captures here and 10 on macOS (5/16 versus 2/16 in the
second notebook), and the canonical branch's capture time, settled ratio and final
periods differ in their fourth to sixth significant digit (4.6501 vs 4.6499 Myr;
1.5000000368 vs 1.5000000498; 58.651 vs 58.642 d). Every check in both notebooks'
verification gauntlets passes on both machines, because each check tests a physical
expectation with its own tolerance rather than a platform's last digits — and on
each machine the numbers are deterministic run to run, which is what "Numbers
differ in the last digit" in Part F relies on. The complete line-by-line
comparison of both notebooks' gauntlet outputs, Windows 11 beside macOS, is in
`PLANET_MERCURY_PROVENANCE.md` §7 (further reading; every command you need is
already on this page).
