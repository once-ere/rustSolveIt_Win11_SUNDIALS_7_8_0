# PLANET_MERCURY_PROVENANCE.md — the planet_Mercury sub-project import

This document records, in the repository's provenance convention, what the
top-level `planet_Mercury/` directory is, where it came from, how it was
verified, and every deviation made along the way. It is self-contained.

**How to read this page (Windows 11 edition, 2026-09-09).** Sections 1–4 and 6
are the import record as it was written on the macOS import machine
(Apple Silicon, 2026-08-30), verbatim apart from bracketed or parenthesised
editorial notes that point to §7: their measurements are macOS ones and are kept as
lineage. Section 5 gives the complete Windows 11 reproduction commands, and
section 7 is the dated Windows 11 re-verification of the same sub-project on
this machine — its figures recorded side by side with the macOS ones, with the
cause of every difference. This repository is
https://github.com/once-ere/rustSolveIt_Win11_SUNDIALS_7_8_0.git, ported from
once-ere/rustSolveIt_macos-silicon_SUNDIALS_7_8_0 (macOS / Apple Silicon),
itself ported from once-ere/rustSolveIt_Using_SUNDIALS_7_8_0 (Linux).

## 1. What was added

`planet_Mercury/` — a complete, verified teaching-and-research sub-project that
simulates the historical tidal despinning and 3:2 spin-orbit resonant capture of
planet Mercury as a two-body Sun–Mercury system (point-mass Sun; Mercury as an
extended, deformable, triaxial body), using THIS repository's vendored pure-Rust
SUNDIALS 7.8.0 CVODE as the only integrator. Contents:

- `mercury_rs/` — pure-Rust compute crate (BSD-3-Clause; `#![forbid(unsafe_code)]`,
  `#![deny(warnings)]`, zero external dependencies; path-depends on
  `../../sundials_rs/crates/{sundials_core,cvode_rs}`). Five-state ODE
  y = [a, e, M, θ, Ω]: Kepler's law, the Hut (1981) constant-time-lag tidal
  torque with its orbital back-reaction, and the Goldreich–Peale triaxial
  "handle" torque. CVODE BDF + Newton + dense at rtol 1e-12,
  atol [1e-3, 1e-6, 1e-10, 1e-10, 1e-14], max step 864000 s, with exact
  root-finding on spin-ratio crossings. 15 analytic-expectation unit tests
  [editorial note: 15 at the test-1 import; 19 after test 2 — §6, §7].
- `mercury_crosscheck/` — small GPL-3.0-or-later crate (links this repository's
  GPL `rebound_rust`/`reboundx_rust` ports, exactly like those imports
  themselves): independently reproduces the tidal despin rate with the
  REBOUNDx `tides_spin` (Eggleton/Lu) formulation — agreement 0.25%, after
  documenting that REBOUNDx's "constant time lag" is exactly HALF of Hut's τ
  (σ = 4τG/(3R⁵k₂); derived by hand in the source).
- `notebook/` — the executed Jupyter deliverable `mercury_tidal_locking.ipynb`
  (Python 3 ipykernel, standard-library-only cells driving the Rust binary by
  subprocess, per this repository's own notebook convention) plus its
  deterministic builder, batch runner, and structure auditor.
- `gui/mercury_orbit.html` + `gui/bake_page.py` — a self-contained browser
  display page in this repository's recorded-player style (all data embedded,
  vanilla JS + canvas, zero network requests): orbit animation with the spin
  "handle," a spin/orbit-ratio dial settling on exactly 1.5, libration,
  period, and angular-momentum-ledger plots, play/scrub controls.
- `plan/` (eight planning documents), `SOURCE_SPECIFICATION.md` (verbatim copy
  of the consolidated physics specification), and
  `INSTRUCTIONS_FOR_TEACHERS_AND_STUDENTS.md` (complete self-contained guide).
- Deliberately NOT committed: `planet_Mercury/data/` (~267 MB of CSV/SQLite the
  notebook regenerates deterministically, bit-for-bit, on one run) and build
  `target/` dirs. No `.gitignore` files were added anywhere in the sub-project.

## 2. Where the physics came from

The model implements *"Mercury 3:2 Spin-Orbit Resonant Capture Provenance
Specification"* (Patrick Nash, 2026-08-24), as consolidated and corrected in
`planet_Mercury/SOURCE_SPECIFICATION.md` (2026-08-25): Hut (1981) CTL tides,
Goldreich & Peale (1966) capture mechanics, spec constants verbatim
(k₂ = 0.12, τ = 100 s, (B−A)/C = 1e-4, C = 0.34mR², a₀ = 5.790905e10 m,
e₀ = 0.20563, Ω₀ = 1.5e-4 rad/s). The specification's four inherited equation
errata (E1–E4, documented in that file) are corrected; the corrected trio
provably conserves C·Ω + m·n·a²·√(1−e²) exactly in the secular part (verified
symbolically during planning and enforced by unit test).

## 3. Verification (all measured on this machine, Apple Silicon, 2026-08-30)

- Engine untouched: after the one-line root `Cargo.toml` change (appending
  `"planet_Mercury"` to the workspace `exclude` list; prior copy in
  `.backups/2026-08-30/`), `cargo metadata` shows the original four workspace
  packages only; `cargo build --workspace --all-targets` warning-free; all 622
  workspace tests green; `bash tools/macos_verify_physics.sh` exit 0.
- Crate gates: `cargo build --release` warning-free and `cargo test --release`
  15/15 in BOTH the workspace original and this in-repo mirror; the mirror's
  run-E seam validation reproduces the original bit-for-bit (despin-rate
  agreement between the secular and full models: 0.12%).
- Science results (the executed notebook's own gauntlet asserts all of these):
  spec-literal run A: Ω/n falls only 181.4 → 178.9 in 10 Myr — the spec's own
  constants imply ~4.7 Gyr of braking (Finding F1), motivating the documented
  1000× time compression of the movie runs; canonical capture into 3:2 at
  4.6499 Myr movie-time; 64-branch phase sweep: 10/64 = 15.6% ± 4.5% captured
  (Goldreich–Peale's simple estimate ~7%); settled-era mean spin ratio
  1.5000000498; final periods 87.968 d / 58.642 d / 175.91 d vs observed
  87.969 / 58.646 / 175.938 (instantaneous samples carry the residual
  ~2e-4 libration wiggle; the time-averaged 2/3 lock holds to 3.3e-8);
  libration swing decays 6.22 → 0.15 rad across the notebook's 12 bins;
  angular-momentum ledger: pre-capture drift ≤ 1e-9 budget, locked era checked
  against the model's own predicted secular leak (the spec's handle torque has
  no orbital back-reaction — see deviation DEV-7 below).
- Notebook: executed end-to-end headless twice; the two executed files are
  byte-identical (determinism gate); the structure auditor passes all rules.
- Display page: baked twice byte-identically; zero network references; loads
  with zero console errors; capture-jump, scrub, HUD readouts and all four
  plots verified in a real browser.

## 4. Deviations register (every one documented and justified)

Approved-plan decisions: D1 build-then-mirror layout (this import); D2 time
compression S = 1000 (spec's 10-Myr window vs its own ~4.7-Gyr braking
timescale); D3 staged integration (the handle torque's ~6-hour wiggle would
need ~1e11 steps; it averages to zero far from resonance, switched on at
Ω/n = 2.2, seam-validated); D4 64-branch phase sweep (capture is genuinely
probabilistic); D5 notebook/display technology per this repository's own
conventions; D6 no `.gitignore` files added.

Build-time findings (full detail in `planet_Mercury/plan/07_PROVENANCE_AND_DEVIATIONS.md`):
DEV-1 the seam checkpoint was moved from Ω/n = 3.0 — itself the 3:1 resonance,
which the validation run caught — to the non-resonant 2.7; DEV-2 run D's
libration period is checked with the exact large-amplitude pendulum formula
(fresh capture librates near the separatrix); DEV-3 sweep branches and the
canonical continuation use numerically identical solver paths (capture is
bit-level path-sensitive); DEV-4 the 2/3-lock statement is checked on the
time-averaged spin (a locked planet librates forever); DEV-5 the REBOUNDx τ
convention is half of Hut's (mapping documented in mercury_crosscheck);
DEV-6 regenerable data not committed; DEV-7 in lock the handle torque's mean
is nonzero by necessity, so the model leaks total angular momentum secularly
(~2e-10 per compressed Myr) — the ledger check is split into the two eras
honestly, and the f64 quantization that freezes the locked orbit's recorded
drift is documented; DEV-8 two reviewer notes documented-not-changed (θ's
magnitude-degraded absolute tolerance; the γ-unwrap margin at the dense
trigger), each harmless as configured and pinned with comments; DEV-9 a
review-driven manifest-clearing hardening briefly deleted a sibling run's
manifest on read access — caught by the end-to-end pipeline and fixed by
splitting read-safe `run_dir` from owning-run `fresh_run_dir`.

## 5. Reproduction (Windows 11 — complete commands)

Everything below is typed into **PowerShell** from a clone of this repository;
where the Git Bash form differs it follows in parentheses. Prerequisites: the
Rust toolchain from https://rustup.rs (on Windows the installer,
`rustup-init.exe`, checks for the Visual Studio C++ Build Tools and offers to
install them when they are missing — accept; they supply the MSVC linker that
`rustc` locates by itself, no "Developer Command Prompt" is ever needed), `git`
(https://git-scm.com/download/win), Python 3 from https://www.python.org
(tick "Add python.exe to PATH"; the command is `python`, not `python3`), and an
x86-64 CPU with FMA — the root `.cargo/config.toml` pins
`-C target-feature=+fma`, which the Windows engine's deterministic libm
requires, and cargo applies it to the sub-project's crates too because it finds
that file by walking up from `planet_Mercury\mercury_rs`. No WSL2 is involved.

**Get the repository and build + test the compute crate** (a few minutes;
`2>&1 | Tee-Object <log>` shows the output and saves it — Git Bash:
`2>&1 | tee <log>`):

```powershell
git clone https://github.com/once-ere/rustSolveIt_Win11_SUNDIALS_7_8_0.git
cd rustSolveIt_Win11_SUNDIALS_7_8_0
cd planet_Mercury\mercury_rs
cargo build --release 2>&1 | Tee-Object ..\..\logs\mercury_rs_build.log
cargo test --release 2>&1 | Tee-Object ..\..\logs\mercury_rs_test.log
cargo run --release -- print-config
cd ..\..
```

(Git Bash: `cd planet_Mercury/mercury_rs`,
`cargo build --release 2>&1 | tee ../../logs/mercury_rs_build.log`,
`cargo test --release 2>&1 | tee ../../logs/mercury_rs_test.log`,
`cargo run --release -- print-config`, `cd ../..`.) Expect zero warnings and
the line `test result: ok. 19 passed; 0 failed` (the `Doc-tests mercury_rs`
harness reporting `0 passed; 0 failed` is a normal empty harness); the binary
is `planet_Mercury\mercury_rs\target\release\mercury_rs.exe`.

**Independent cross-check** (GPL crate; links the repository's REBOUND /
REBOUNDx ports):

```powershell
cd planet_Mercury\mercury_crosscheck
cargo run --release 2>&1 | Tee-Object ..\..\logs\mercury_crosscheck.log
cd ..\..
```

(This port's record of the same run is `logs\crosscheck_stress_launcher.log`,
§7.2; `logs\mercury_crosscheck.log` is the name your own run will create.)

(Git Bash: `cargo run --release 2>&1 | tee ../../logs/mercury_crosscheck.log`.)
Expect the lines `relative difference = 2.511e-3`,
`PASS - crosscheck.despin_rate: two independent formulations agree within 2%`
and `SUCCESS`; an upstream REBOUNDx notice about a 4.5.0 tidal-potential fix
also prints and is expected.

**Both notebooks, headless** (no browser; fails loudly on any error; the first
notebook is the long one — budget about an hour):

```powershell
cd planet_Mercury\notebook
$env:MERCURY_NO_BROWSER = "1"
python run_notebook.py mercury_tidal_locking.ipynb
python check_notebook.py mercury_tidal_locking.ipynb
python run_notebook.py mercury_test2_jupiter_gr.ipynb
python check_notebook.py mercury_test2_jupiter_gr.ipynb
Remove-Item Env:MERCURY_NO_BROWSER
cd ..\..
```

(Git Bash: `cd planet_Mercury/notebook`,
`MERCURY_NO_BROWSER=1 python run_notebook.py mercury_tidal_locking.ipynb`,
`python check_notebook.py mercury_tidal_locking.ipynb`,
`MERCURY_NO_BROWSER=1 python run_notebook.py mercury_test2_jupiter_gr.ipynb`,
`python check_notebook.py mercury_test2_jupiter_gr.ipynb`, `cd ../..`.)
Expect `ok mercury_tidal_locking.ipynb (18 cells)`, `1 ok, 0 failed`,
`ok mercury_tidal_locking.ipynb: all structure rules pass`, and
`ok mercury_test2_jupiter_gr.ipynb (17 cells)`, `1 ok, 0 failed`,
`ok mercury_test2_jupiter_gr.ipynb: all structure rules pass`. One
top-to-bottom run regenerates every dataset (`planet_Mercury\data\runs\`), both
databases (`planet_Mercury\data\mercury_orbit.sqlite3`,
`planet_Mercury\data\mercury_test2.sqlite3`), every check, and both display
pages, deterministically; `data\` is regenerable and gitignored (DEV-6).

**Open the display pages** (the notebooks open them themselves when
`MERCURY_NO_BROWSER` is not set):

```powershell
Start-Process planet_Mercury\gui\mercury_orbit.html
Start-Process planet_Mercury\gui\mercury_test2.html
```

(Git Bash: `start planet_Mercury/gui/mercury_orbit.html`,
`start planet_Mercury/gui/mercury_test2.html`.) To re-bake a page alone after
the data exist: `cd planet_Mercury; python gui\bake_page.py; python gui\bake_page2.py; cd ..`
(Git Bash: `cd planet_Mercury && python gui/bake_page.py && python gui/bake_page2.py && cd ..`).

**Interactive JupyterLab** (optional viewer; the notebooks' code cells use only
Python's standard library; the venv is never "activated", its programs are
called by path, so no PowerShell execution-policy change is needed):

```powershell
python -m venv planet_Mercury\notebook\.venv
planet_Mercury\notebook\.venv\Scripts\pip.exe install jupyterlab
cd planet_Mercury\notebook
.venv\Scripts\jupyter.exe lab mercury_tidal_locking.ipynb
```

(Git Bash: `python -m venv planet_Mercury/notebook/.venv`,
`planet_Mercury/notebook/.venv/Scripts/pip.exe install jupyterlab`,
`cd planet_Mercury/notebook`, `.venv/Scripts/jupyter.exe lab mercury_tidal_locking.ipynb`.)
The step-by-step classroom walkthrough, with troubleshooting, is
`planet_Mercury/INSTRUCTIONS_FOR_TEACHERS_AND_STUDENTS.md` (it repeats every
command above).

## 6. TEST 2 — Jupiter + Einstein (second commit, same day)

Commissioned by the instruction "now run your second test: add Jupiter and
Einstein's GR correction". Implemented as a separate six-state module
(`planet_Mercury/mercury_rs/src/test2.rs`, state y = [a, e, M, ϖ, θ, Ω]) plus
five new CLI runs (`t2-gr-check`, `t2-jupiter-check`, `t2-movie`, `t2-sweep`,
`t2-final`), leaving every test-1 code path and number untouched.

**Physics added.** (a) General relativity's apsidal precession
dϖ/dt = 3nGM☉/(c²a(1−e²)) — 42.98″/century at Mercury's orbit; (b) Jupiter's
secular forcing in Laplace–Lagrange form (fixed Jupiter, e_J = 0.0489,
ϖ_J = 0): de/dt += A12·e_J·sin(ϖ−ϖ_J), dϖ/dt += A11 + A12(e_J/e)cos(ϖ−ϖ_J),
with Laplace coefficients from a deterministic 4096-point trapezoid rule,
unit-tested against the textbook power series to 1e-10. The handle torque's
argument becomes 2(θ−f−ϖ) and the resonance angle γ₂ = 2θ−3M−2ϖ (re-anchoring
leaves γ₂ exactly invariant — unit-tested). Only the tidal strength carries the
documented 1000× movie compression; the GR and Jupiter rates are real.

**Measured verification (macOS import machine, Apple Silicon, 2026-08-30 — the Windows 11 figures are in §7).** Gate A: GR alone
integrates to 42.983″/cy vs 42.983 analytic (rel. diff 7×10⁻¹³). Gate B:
Jupiter alone gives forced-eccentricity amplitude 4.5438×10⁻³ and period
808.8 kyr, both equal to the Laplace–Lagrange predictions. Full chain:
restart root at Ω/n = 1.6 to 5×10⁻¹² (t = 4.4612 Myr movie time); sweep 2/16
phase branches captured (canonical branch 1, capture t = 4.6779 Myr);
canonical continuation to 10 Myr — settled mean spin ratio **1.5000004545**
vs predicted 1.5 + ϖ̇/n = **1.5000003777** (GR 42.98 + Jupiter 160.24″/cy):
the lock sits 4.5×10⁻⁷ above exactly 3/2 — **it follows the precessing
ellipse, not the stars**. Perihelion advanced 54.57 rad = 203.21″/cy vs
203.22 predicted; e oscillated 0.19846–0.20563 through braking, capture, and
lock; final P_orb 87.9678 d. Test 1's angular-momentum ledger is deliberately
absent in test 2: the Laplace–Lagrange terms exchange angular momentum with
Jupiter, which the model does not track — checking a two-body conservation
law there would be checking the wrong law (stated in the program's output,
the notebook, and the plan documents).

**Artifacts.** Second notebook `planet_Mercury/notebook/mercury_test2_jupiter_gr.ipynb`
(authored by `build_notebook2.py`, executed for real, byte-determinism proven
by double execution), its own database schema (`data/mercury_test2.sqlite3`
with a `run_extra` provenance table and a `pomega_rad` sample column — the
database itself is regenerable and not committed, per DEV-6), display page
baker `planet_Mercury/gui/bake_page2.py` → `gui/mercury_test2.html`, four new
analytic unit tests (19 total), and TEST-2 addenda in
`planet_Mercury/plan/07_PROVENANCE_AND_DEVIATIONS.md` (Part 6) and
`planet_Mercury/INSTRUCTIONS_FOR_TEACHERS_AND_STUDENTS.md` (Part G).

## 7. Windows 11 re-verification (2026-09-09)

Sections 1–4 and 6 above are the macOS-era import record and are unchanged;
this section records the re-run of the whole sub-project on this Windows 11
machine after the macOS project was ported onto the Windows engine. The
vendored engine here is once-ere/SUNDIALS_7_8_Rust_port_for_Windows11; it
differs from the Linux/macOS engine only in backend module sets and its public
API is identical, so `mercury_rs/Cargo.toml`'s two path dependencies
(`../../sundials_rs/crates/{sundials_core,cvode_rs}`) needed no change. Every
number in this section was measured on this machine and is backed by a file
under `logs/` (named per item); the macOS figures beside them are the values
embedded in the committed macOS executions of the two notebooks.

### 7.1 Machine and toolchain (measured)

- Windows 11 Pro for Workstations, build 10.0.26200; Intel Core Ultra 9 275HX.
- rustc 1.91.1 (ed61e7d7e 2025-11-07) / cargo 1.91.1, host
  x86_64-pc-windows-msvc; the root `.cargo/config.toml` pins
  `-C target-feature=+fma` (the Windows engine's deterministic libm requires
  FMA), and cargo applies it to the sub-project's crates because it finds
  that file by walking up from `planet_Mercury\mercury_rs`.
- MSVC linker: Visual Studio 2026 Build Tools, MSVC 14.51.36231, located by
  `rustc` automatically (no developer prompt). `mercury_rs` and
  `mercury_crosscheck` are pure Rust — no C compiler takes part in building or
  running them. (The only C compiled anywhere in this repository is (a) the
  bundled SQLite amalgamation inside `dataset_tools`, built through the `cc`
  crate, and (b) the REBOUND/REBOUNDx C reference harnesses of the rebound
  gates — both with MSVC toolset 14.51.36231, `cl` banner 19.51.36256.) No
  WSL2 compiler or linker was used anywhere.
- Python 3.14.5 (default text encoding cp1252 — see 7.4); the notebooks'
  code cells need nothing outside the standard library.

### 7.2 Engine and crate gates (measured)

- Engine untouched by the sub-project: `cargo test --workspace` 638 passed,
  0 failed (`logs/test_final.log`; the macOS record in §3 says 622 — the
  workspace has grown since); physics byte-identity gate
  `bash tools/win_verify_physics.sh` exit 0 (`logs/win_verify_physics.log`).
- `mercury_rs`: `cargo build --release` 0 warnings
  (`logs/mercury_rs_build.log`); `cargo test --release` **19 passed, 0
  failed** (`logs/mercury_rs_test.log`); `target\release\mercury_rs.exe`
  1,104,384 bytes.
- `mercury_crosscheck`: `cargo build --release` 0 warnings; the run printed
  (`logs/crosscheck_stress_launcher.log`):

  ```
    spin ratio 100 x n, e = 0.20563, 200 orbits with IAS15 + tides_spin
    measured  dOmega/dt = -3.652928e-18 rad/s^2  (REBOUNDx tides_spin)
    predicted dOmega/dt = -3.643780e-18 rad/s^2  (Hut 1981 secular formula)
    relative difference = 2.511e-3
  PASS - crosscheck.despin_rate: two independent formulations agree within 2%
  SUCCESS
  ```

  — 0.25 %, the same agreement §1 records for the macOS import.

### 7.3 Notebooks (measured)

Both notebooks were rebuilt by `build_notebook.py` / `build_notebook2.py`,
executed headless by `run_notebook.py` under `MERCURY_NO_BROWSER=1`, and
audited by `check_notebook.py` (`logs/mercury_notebook1.log`,
`logs/mercury_notebook2.log`): `mercury_tidal_locking.ipynb` — `ok (18
cells)`, `1 ok, 0 failed`, `all structure rules pass`;
`mercury_test2_jupiter_gr.ipynb` — `ok (17 cells)`, `1 ok, 0 failed`, `all
structure rules pass`. Every assertion of both verification gauntlets passes
(test 1: `all_run_verdicts: 6/6 runs ended SUCCESS`; test 2: `5/5`), the
display pages bake byte-identically twice, and the numbers are deterministic
run to run on this machine. The executed notebooks carry the lines below as
embedded outputs; the regenerated `data/` is not committed (DEV-6).

**Test 1 — `mercury_tidal_locking.ipynb`, Windows 11 beside the committed
macOS run** (identical lines are marked "same"):

| Quantity | Windows 11 (this machine) | macOS (committed run) |
|---|---|---|
| Run A: final Ω/n after 10 Myr (Finding F1) | 178.9260 | same |
| Run A: full-model proof segment, Ω/n | 181.446450 | 181.446443 |
| Run B: stage handover | t = 3.7281 Myr | same |
| Run B: root-found restart Ω/n (target 1.6) | 1.600000000000e+00 | 1.599999999963e+00 |
| Run B: restart time | 4.4315 Myr | 4.4316 Myr |
| Run B: samples recorded | 10762 rows | 10763 rows |
| Run C: 64-branch capture fraction | 5/64 = 0.0781 ± 0.034 (68 %) | 10/64 = 0.1562 ± 0.045 |
| Canonical capture time (movie time) | 4.6501 Myr | 4.6499 Myr |
| Settled-era mean Ω/n | 1.5000000368 | 1.5000000498 |
| Final P_orb / P_rot / solar day (observed 87.969 / 58.646 / 175.938 d) | 87.9678 / 58.6512 / 175.9896 d | 87.9678 / 58.6419 / 175.9061 d |
| Two-thirds lock: settled-era mean P_rot/P_orb − 2/3, relative | 8.8204e-09 | 1.6579e-08 |
| Residual-libration wiggle of the final instantaneous sample | 1.0236e-04 | 5.5741e-05 |
| Ledger, pre-capture: crate check max drift (budget 1e-9) | 7.0764e-11 | 6.2675e-11 |
| Ledger, pre-capture: notebook total-L drift (budget 1e-9) | 4.029e-10 | 3.947e-10 |
| Ledger, locked era: drift vs the model's predicted leak | 8.821e-12 vs 1.134e-09 (crate: 8.7732e-12 vs 1.1343e-09) | 9.156e-12 vs 1.134e-09 (crate: 9.1007e-12 vs 1.1344e-09) |
| Libration swing, notebook's 12 bins | 6.232 → 0.140 rad | 6.224 → 0.152 rad |
| Libration swing, crate's early → late bin | 4.5684 → 0.0581 rad | 4.5822 → 0.0604 rad |
| Run D: guaranteed capture (e = 0.285) | 765.2 kyr | 762.6 kyr |
| Run D: libration period vs pendulum-exact | 27.46 yr vs 29.13 yr at amplitude 2.723 rad (rel diff 5.730e-02) | 27.52 yr vs 29.25 yr at 2.728 rad (5.897e-02) |
| Run E: seam dΩ/dt, secular vs full | −5.310106e-20 vs −5.304468e-20 rad/s² (rel diff 1.062e-03) | −5.310106e-20 vs −5.303729e-20 (1.201e-03) |
| Run E: end-state \|a_S − a_R\| / \|e_S − e_R\| | 1.393 m / 5.069e-12 | same |
| Canonical run: CVODE steps / re-anchorings | 711041787 / 109388 | 711084473 / 109388 |
| Verdicts | all PASS, 6/6 runs SUCCESS | all PASS, 6/6 runs SUCCESS |

**Test 2 — `mercury_test2_jupiter_gr.ipynb`:**

| Quantity | Windows 11 (this machine) | macOS (committed run) |
|---|---|---|
| Gate A: GR precession, measured vs analytic | 42.983 vs 42.983 ″/century (rel diff 7.193e-13) | same |
| Gate B: forced-eccentricity amplitude (e 0.19654 .. 0.20563) | 4.5438e-03 = LL prediction | same |
| Gate B: secular period | 808.8 kyr = LL prediction | same |
| Movie run: stage handover | t = 3.7616 Myr | same |
| Restart root Ω/n (target 1.6) and time | 1.600000000000e+00 at 4.4615 Myr | 1.599999999995e+00 at 4.4612 Myr |
| 16-branch sweep | 5/16 captured = 0.3125 | 2/16 = 0.1250 |
| Canonical capture time | 4.6781 Myr | 4.6779 Myr |
| Settled mean Ω/n vs predicted 1.5 + ϖ̇/n = 1.5000003777 | 1.5000004081 (\|diff\| 3.05e-08; 4.081e-07 above 3/2; 43221 samples) | 1.5000004545 (7.69e-08; 4.545e-07; 43222 samples) |
| Perihelion advance over the run | 54.56 rad = 203.21 ″/century vs predicted 203.22 | 54.57 rad = 203.21 vs 203.22 |
| e range in the canonical run (swing amplitude) | 0.19846 .. 0.20563 (3.584e-03) | same |
| Final P_orb / P_rot / solar day | 87.9678 / 58.6464 / 175.9463 d | 87.9678 / 58.6447 / 175.9314 d |
| CVODE steps / re-anchorings | 687068258 / 108305 | 687207958 / 108503 |
| Verdicts | all PASS, 5/5 runs SUCCESS | all PASS, 5/5 runs SUCCESS |

### 7.4 Why the two columns differ, and why every gate still passes

`mercury_rs` evaluates its tidal and triaxial torques — the right-hand side
CVODE is handed — through the host C runtime's libm: Microsoft's UCRT on this
machine, Apple's libm on the import machine. The two libraries round some
transcendental results differently in the last bit, so the right-hand side
differs at the 1e-16 level between the platforms. CVODE itself runs on the
vendored deterministic engine and is not the source of any difference. Where
the model is smooth those last-bit differences stay small (the run-A ratio,
the GR rate and its 7.193e-13 residual, the Laplace–Lagrange amplitude and
period, the secular-stage seam value −5.310106e-20 and the 109388
re-anchorings are identical on both columns); near the 3:2 separatrix the
model is chaotic and, as the notebook's own text and deviation DEV-3 state,
capture is bit-level path-sensitive — so the phase-sweep tallies differ
(5/64 vs 10/64 in test 1; 5/16 vs 2/16 in test 2; the notebook prints the
64-flip / 16-flip error bars with them) and the canonical branch's capture
time, settled ratio and final periods differ in their fourth to sixth
significant digit. Every assertion of both gauntlets passes on both platforms
because each one tests a physical expectation with its own tolerance, not a
platform's last digits; the platform-specific numbers are deterministic run
to run on each machine (the determinism gate — byte-identical repeat
executions — holds here as it did there).

### 7.5 What the port changed in the sub-project

Only platform plumbing and prose; no physics, constants, tolerances or CLI
runs changed. The notebook driver cell probes `mercury_rs.exe` before the
extensionless name (the `MERCURY_BIN` override is still consulted first) and
decodes the program's output as UTF-8 (`mercury_rs` prints UTF-8 em-dashes;
Windows Python's default text decoding is cp1252, which garbled them); the
setup prose in both notebooks and both display-page bakers is written for
PowerShell; every builder writes its files with LF line endings. The
pre-modification macOS originals of every edited file are kept under
`.backups/2026-09-09/planet_Mercury/` (gitignored). The sub-project still
carries no `.gitignore` of its own (D6): `planet_Mercury/data/` and
`planet_Mercury/notebook/.venv/` are ignored from the repository root.
