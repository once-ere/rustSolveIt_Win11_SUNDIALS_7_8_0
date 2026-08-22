# SUNDIALS_7_8_Rust_port_for_Windows11 — workspace rules

Pure-Rust port of SUNDIALS 7.8.0. The upstream C tree is **read-only** and
lives *outside* this workspace, at `C:\Users\nsh\Developer\sundials-7.8.0`;
the harness scripts take its location from `$SUNDIALS_C_TREE`. (The sibling
ports keep the workspace inside that tree — this one does not, because it is
its own repository under `.../github/`.) This workspace is its own git repo;
git is the undo mechanism.

Read `current_status.md` first — it is the resume anchor.

## Target platform — binding on every rule below

**This port is scoped to Windows 11 on Intel/AMD x86-64**, measured on
Windows 11 Pro for Workstations 10.0.26200 (25H2) / `ucrtbase.dll`
10.0.26100.8521 / rustc 1.91.1 / target `x86_64-pc-windows-msvc`. The Rust
sources are portable (`std` only, no `cfg(target_os)`/`cfg(target_arch)`)
and build and unit-test anywhere, but every *numerical* claim — the
199-variant gate, each per-variant classification in `VERIFICATION.md`, and
the `pow` differential — is a Windows-on-x86-64 result.

**This platform does not get its libm for free, and that shapes every rule
below.** The upstream reference `.out` files were generated on a glibc host,
while `f64`'s `sin`, `cos`, `exp`, `ln`, ... are documented as having
unspecified precision and forward to the host libm — here the Microsoft UCRT,
which `tools/libm_fingerprint_win.sh` shows disagrees with glibc on **every**
one of them; only `sqrt` matches. With the host libm in the path the gate read
125 / 54 / 20 against the Linux sibling's 153 / 26 / 20 on identical Rust.

So the host libm is gone. `crates/sundials_core/src/sundials_libm/` carries
`exp`, `log`, `expm1`, `log1p`, `sin`, `cos`, `atan`, `asin`, `acos`, `sinh`,
`cosh` and `acosh`, each a translation of what glibc 2.39 runs on x86-64 and
each measured **0 mismatches over 8,000,000 inputs** against a real glibc
oracle; `pow` was done earlier and lives in `sundials_math.rs`. The gate now
reads **153 IDENTICAL / 26 divergent / 20 excluded**, and the 26 are exactly
the Linux port's set. Rules that follow:

* **Never call a host libm method** — `.sin()`, `.cos()`, `.exp()`, `.ln()`,
  `.powf()`, `.sinh()` and friends — anywhere in `crates/*/src` or
  `crates/*/examples`. A grep for them outside `sundials_libm/` must stay
  empty. `sqrt`, `mul_add`, `abs`, `copysign`, `floor`, `ceil`, `round` and
  `trunc` are IEEE-754 specified and portable — do not list them as
  host-dependent and do not replace them.
* **New call sites go through `sundials_libm`.** `tools/route_libm_calls.py`
  does the rewrite mechanically and is idempotent.
* **A routine the port does not yet have** (`tan`, `log10`, `tanh`, `atan2`)
  must be ported into `sundials_libm` with its own differential, not taken
  from the host.
* **The FMA-contraction split is load-bearing.** glibc's x86-64 build
  ifunc-dispatches `exp`, `log`, `pow`, `sin`, `cos`, `atan`, `asin`, `acos`,
  `expm1` and `log1p` to an FMA rebuild (`-mfma -mavx2 -ffp-contract=fast`),
  so those fuse `a*b + c`; `sinh`, `cosh` and `acosh` have no FMA variant and
  must not. Use `f64::mul_add` exactly where the FMA build fuses.

* **Two preconditions the module cannot enforce, and must not be quietly
  dropped from the docs.** (a) `f64::mul_add` lowers to `llvm.fma.f64`, and
  on the SSE2 msvc baseline that is a *call into `ucrtbase`*, not an
  instruction — `.cargo/config.toml` pins `-C target-feature=+fma` to fix
  that, and a downstream crate must do the same. (b) glibc's `sin`, `cos` and
  `atan` wrap themselves in `SET_RESTORE_ROUND (FE_TONEAREST)`, which is
  **not** an x86-64 no-op; safe Rust cannot reach MXCSR, so the default
  floating-point environment is a precondition. See the Preconditions section
  of `sundials_libm`.
* **`SUNDIALS_LIBM_ORACLE_STRICT` when you mean to measure.** Without it a
  missing oracle makes all twelve differentials pass having compared nothing.
  The harness sets it; set it by hand if you run `cargo test` directly.

Never present this port as byte-identical to all 199 references, and never
close a divergence by tuning an example or widening `noise_filter()`.

Any statement added to any document in this repo that asserts a verification
result must carry that scope explicitly. Ports for other platforms are
separate repositories (see the siblings
`SUNDIALS_7_8_Rust_port_for_AppleSilicon_macos`, from which this
workspace's crate tree is inherited unchanged, and
`SUNDIALS_7_8_Rust_port_for_Linux`), never conditional compilation inside
this one.

## Hard rules

1. **Fidelity first.** Line-by-line faithful translation: control flow,
   constants, tolerances, heuristics, error/return codes, and argument
   lists (names, order, meaning) match the parent C function exactly.
   Preserve arithmetic order — acceptance is byte-identical printed output.
2. Zero `unsafe`, zero FFI, zero external crates (std only), zero warnings
   from `cargo build --workspace`.
3. Never stub a missing symbol — its definition is under `../src/` or
   `../include/`; port it into `sundials_core`.
4. Public API keeps exact C names and return-flag conventions
   (`CV_SUCCESS = 0`; negative = fatal, positive = recoverable). Crate
   roots carry `#![allow(non_snake_case, non_camel_case_types,
   non_upper_case_globals)]`.
5. All float output goes through
   `sundials_core::sundials_utils::{fmt_e, fmt_f, fmt_g}` — never `{:e}`.
6. C buffer aliasing (e.g. CVODE `cv_y` / user `yout`): copy back at
   **every** return path, including early-error and rootfinding exits.
   All of CVODE(S), IDA(S), ARKODE do this.
7. Once a crate's examples verify green they stay green — the cumulative
   regression gate runs `tools/verify_examples.sh` for all crates ported
   so far at every phase gate.

## Module layout

- Module = C file base name + `.rs` (`cvodes_nls_stg1.c` →
  `crates/cvodes_rs/src/cvodes_nls_stg1.rs`; `arkode_impl.h` →
  `arkode_impl.rs`). Public `include/` headers fold into the matching
  module.
- Solver crates re-export every shared `sundials_core` module at root and
  provide a flat prelude so examples can `use cvode_rs::*;`.
- One `[[example]]` entry per translated example; example name = C base
  name.
- `user_data` is `Option<Box<dyn Any>>`; callbacks are plain `fn`
  pointers. Aliasing vector ops get in-place methods; free functions
  (`N_VLinearSum`) serve distinct operands.

## Workflow

- Commit after every ported file (or small coherent group); tag phase
  gates (`phase2-cvode-green`, …).
- After EVERY build/test/run: `… 2>&1 | tee <log>` then **Read the log**
  before the next edit. Never re-run a command that produced no output.
- Max two attempts per failing command, then switch strategy.
- Read each in-scope C file exactly once, at translation time, completely.
  Never read excluded paths (GPU/MPI/KLU/LAPACK/Fortran/xbraid trees).
- Update `PROGRESS.md` (per-file status: todo | ported | building |
  committed) and `VERIFICATION.md` (per-variant status) as units land.
- Resume after context loss from this file + `PROGRESS.md` + `git log` —
  do not re-explore the tree.

## Verification

Run the harness from **Git Bash / MSYS2** — it is POSIX `bash` and will not
run under `cmd.exe` or PowerShell — and give it the upstream C tree:

```bash
SUNDIALS_C_TREE=/c/Users/nsh/Developer/sundials-7.8.0 tools/verify_examples.sh all
```

`tools/verify_examples.sh [crate|all|list]` parses the upstream CMakeLists
tuples (199 variants), builds release examples, runs each variant with exact
argv, diffs against `$SUNDIALS_C_TREE/examples/...` references
(noise-filtered symmetrically), and writes `logs/summary.txt`. Read only the
summary; open individual diffs only for non-IDENTICAL lines. CLI-option
variants use bare `<solverid>.<key>` tokens (no leading dashes); the parser
prefix-matches literally.

Current Windows/x86-64 gate: **153 IDENTICAL / 26 divergent / 20 excluded**,
0 build failures, 0 run failures — the Linux sibling's numbers, on exactly its
set of variants. **A divergence is a port defect only when Rust != pristine C
on the same host**, and that comparison was made on Linux, not here. Until
someone builds upstream SUNDIALS on Windows with cmake + MSVC/clang-cl and
re-runs the three-way comparison, say "0 port defects identified", never "0
port defects proven". `current_status.md` §7 item 1 is that job.

`tools/classify_diffs.sh` is the second pass — it re-diffs the non-IDENTICAL
variants under `tr -s ' '` and `diff -w` so a whitespace-only divergence
(stale `SUN_TABLE_WIDTH` 28 -> 29 references) can be told from a content one.
Currently 15 of the 26 are whitespace-only, matching Linux element for
element. Never widen `noise_filter()` to swallow last-ulp drift, and never
tune an example to match a reference.

`tools/libm_fingerprint_win.sh` builds `tools/libm_probe.rs` natively and
inside a WSL2 glibc guest and diffs the two fingerprints (FNV-1a over 1M
results per function). Run it after any toolchain or OS update: it is what
turns "the output moved" into "this function moved", and it *predicts*
which variants are at risk before the gate is run.

`tools/pow_differential_win.sh [domain|random|all]` builds
`tools/pow_oracle.c` inside the WSL2 guest with the guest `cc` — the glibc
reference — and runs the `pow_glibc_vs_native_oracle_*` tests in a natively
built Windows binary against it. `tools/libm_differential_win.sh [fn|all] [n]`
does the same for the twelve `sundials_libm` routines via
`tools/libm_oracle.c`. Re-run both after **any** change to a deterministic
routine: the example gate is blind to that class of defect
(POW_FMA_EXACTNESS.md §6). Keep each corpus generator byte-for-byte in step
with its C twin — `pow_corpus` with `pow_oracle.c`, `sundials_libm/corpus.rs`
with `libm_oracle.c`; the libm harness hashes the arguments and asserts on
drift, the pow one does not. For an out-of-sample run, generate a longer
corpus into a separate directory:
`SUNDIALS_ORACLE_OUT="$PWD/logs/oracle8m" tools/libm_differential_win.sh all 8000000`.
`pow_deterministic_vs_host_powf` needs no oracle and reports how far the host
UCRT `pow` is from the deterministic one.

The Linux sibling's `tools/{pow_differential,glibc_sweep,gate_in_container,
pristine_c_build,compare_pristine_c,compare_lapack_substituted,
wsl_sync_build}.sh` and `tools/libm_probe.c` are carried along unchanged.
They are Linux-side tools: they run in a Linux guest, they are what produced
`evidence/linux-x86_64-glibc239/`, and they are the templates for open item
1. Do not present their output as a Windows result.
Invoke it as `wsl.exe -d Ubuntu-24.04 -- bash tools/wsl_sync_build.sh <step>`
— do **not** pass `$PATH` inside a `bash -c` string, the interop layer
pre-expands it and the Windows paths containing `(x86)` break bash parsing.
