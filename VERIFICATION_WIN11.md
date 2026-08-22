# VERIFICATION_WIN11 — independent check of this repository

An independent verification pass over the finished Windows 11 port, run
2026-08-22 on the same machine that built it (Intel Core Ultra 9 275HX,
Windows 11 Pro for Workstations 25H2, rustc 1.91.1
`x86_64-pc-windows-msvc`). Every command below is complete as written
and was actually run; its measured result follows it. All commands run
from the repository root
`C:\Users\nsh\Developer\github\rustSolveIt_Win11_SUNDIALS_7_8_0\rustSolveIt_Win11_SUNDIALS_7_8_0`;
PowerShell unless marked Git Bash.

## 1. Repository state

```powershell
git status --short
git log --oneline -3
git describe --tags
git fetch origin
git rev-parse HEAD origin/main
```

Result: working tree clean; HEAD = `origin/main` =
`38cd8a77a6eaa62df29beee5426a6e0379102bf4`; tag `win11-port-green`
present on that commit.

## 2. Dependency and safety invariants

```powershell
Select-String -Path Cargo.lock -Pattern '^name = '
Select-String -Path Cargo.lock -Pattern 'source = '
```

Result: exactly the 12 local crates (arkode_rs, cvode_rs, cvodes_rs,
ida_rs, idas_rs, kinsol_rs, sundials_core, spec_math, physical_object,
posim, quantum, special_functions), **zero** registry sources — nothing
from crates.io.

```bash
# Git Bash
grep -rn --include="*.rs" -E "\bunsafe\b" physical_object/src posim/src \
  quantum/src special_functions/src sundials_rs/crates/*/src \
  | grep -v "forbid(unsafe_code)"
grep -rn --include="*.rs" "{:e}" sundials_rs/crates/*/src
grep -rnE --include="*.rs" "\.powf\(|\.powi\(" sundials_rs/crates/*/src
```

Result: **zero** uses of `unsafe` anywhere (every first-party crate
root carries `#![forbid(unsafe_code)]` and `#![deny(warnings)]`). The
three residual grep hits are all benign and inherited from the verified
upstream: a diagnostic print inside the libm differential harness
(`sundials_libm/corpus.rs:309`), a doc comment stating the no-`{:e}`
rule (`sundials_utils.rs:4`), and the deliberate host-comparison test
`pow_deterministic_vs_host_powf` (`sundials_math.rs:918`), which exists
precisely to measure the host `pow` against the deterministic one.

## 3. Build and tests

```powershell
cargo build --workspace --all-targets
cargo build --release --workspace --all-targets
cargo test --workspace
```

Result: both profiles **warning-free**; **622 passed, 0 failed**
(49 physical_object lib + 19 collision + 9 conservation + 42
constrained/equilibrium/sensitivity + 112 posim + 92 quantum + 233
special_functions + 11 vendored identities + 5 + 50 doctests). The one
`[ERROR]...[KINSol]` stderr line mid-run is printed by a *passing* test
that exercises a solver failure path.

## 4. Physics byte-identity against the Linux evidence

```bash
# Git Bash
bash tools/win_verify_physics.sh
```

Result (exit 0), reproduced identically to the port-time run:
12 collision scripts **IDENTICAL**; 6 examples and 59 dynamic
notebooks **identical modulo the two pinned divergences** in
`evidence/win11/accepted-divergences-examples.diff` (one blank line
the donor added to `outer_solar_system` after recording its own
evidence) and `evidence/win11/accepted-divergences-dynamic.diff`
(9 last-printed-digit lines confined to the two quantum notebooks,
`double_slit` and `tunneling`, whose first-party code uses the host
math library). Nothing beyond the pinned diffs appeared.

## 5. Recorded videos, protocol, kernel

```powershell
python recorder\tests\test_units.py
python recorder\tests\test_end_to_end.py
python jupyter\test_protocol.py
$env:POSIM_NO_BROWSER = "1"
jupyter\.venv\Scripts\python.exe jupyter\test_kernel.py
```

Result: recorder 17 + 9 tests **OK** — including
`test_all_committed_recordings_reproduce_byte_for_byte`, which
re-records all 13 browser videos to a temporary directory and
byte-compares them against the committed files; “all protocol checks
passed”; all 7 kernel cells **ok**.

## 6. GUIs and the entity index

```powershell
python tools\win_gui_smoke.py
python tools\verify_index_examples.py
python tools\verify_tierb_examples.py
```

Result: **13 of 13 GUIs pass** (page served, live state, Start advances
simulated time, Reset returns bit-exactly to t = 0);
**PASS 1177/1177** runnable index examples; **COMPILED 258/258** Rust
snippets.

One finding, no action needed: re-running `verify_index_examples.py`
re-captures 4 of the 1,177 transcripts whose scripts query scene
status immediately after `SCENE START` — the live playback thread
advances on wall-clock time, so the captured `t` there (0 vs 0.01 vs
0.03) depends on thread scheduling. All 4 examples still PASS; the
committed capture was kept (`git checkout -- index_data/catalog.json`).

## 7. The 109 Jupyter notebooks, re-executed

```bash
# Git Bash
POSIM_NO_BROWSER=1 python notebooks/_build/nbrun.py notebooks/*.ipynb
python notebooks/_build/nbcheck.py notebooks/*.ipynb
git diff -- notebooks/
```

Result: **109 ok, 0 failed**; **109/109 pass all seven requirements**.
The re-run's only working-tree effect was the OS-assigned ephemeral
scene-window port embedded in 57 captured outputs
(`http://127.0.0.1:<port>/` — one line per affected notebook; verified
that **zero** non-port lines changed). The committed captures were kept
(`git checkout -- notebooks/`). Every physics number in every notebook
reproduced bit-for-bit.

## 8. Documents and pairing

- `grammar.pdf` (899,012 bytes) and `SolveIt.pdf` (497,036 bytes)
  exist and are newer than their `.tex` sources.
- Notebook↔example pairing is exactly 1:1 — 109 notebooks =
  13 `video_*` + 6 `rust_*` + 12 `collision_*` + 19 `solveit_*` +
  59 `dynamic_*`, matching 13 scene scripts + 6 compiled examples +
  12 collision scripts + 19 SolveIt scripts + 59 dynamic notebooks on
  disk.

## Verdict

Every gate re-verified green, independently of the runs that built the
port. The two intentionally-pinned physics divergences reproduced
exactly and nothing new appeared. Two benign run-to-run variations were
identified and documented (§6, §7); both are wall-clock/OS artifacts in
*captured transcripts*, not in any computed physics.
