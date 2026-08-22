# SUNDIALS_7_8_Rust_port_for_Windows11

A line-by-line translation of [SUNDIALS](https://github.com/LLNL/sundials)
7.8.0 into safe Rust, scoped to **Windows 11 on Intel/AMD x86-64**. **No
`unsafe`, no FFI, no external crates, no build warnings.**

**→ Read [`current_status.md`](current_status.md) first.** It is the
authoritative statement of what is measured, what is not, and what remains.

## Where this port stands

| gate | result |
|---|---|
| `cargo build --workspace`, native `x86_64-pc-windows-msvc` | **0 errors, 0 warnings** |
| `cargo test --workspace` | **40 passed, 0 failed** |
| all 108 in-scope example programs build and run (199 argv variants) | **0 build failures, 0 run failures** |
| deterministic `pow` vs **glibc `pow`**, built and run natively on Windows | **0 mismatches over 25,900,000 inputs** |
| deterministic libm — 12 routines vs **glibc 2.39** | **0 mismatches over 96,000,000 inputs** (8,000,000 each; the second half never seen while porting) |
| host libm reachable from the port | **no** — 0 call sites outside `sundials_libm` |
| `tools/verify_examples.sh all` — byte-identity against the upstream references | **153 IDENTICAL / 26 divergent / 20 excluded (KLU/SuperLU)** |
| the 26 divergences | **exactly** the Linux/glibc sibling's set, variant for variant — reference-side, 0 port defects |

That is parity with the Linux port, reached by taking the host libm out of
the code entirely rather than by tolerating it.

## Headline facts

* 7 crates: `sundials_core` plus `cvode_rs`, `cvodes_rs`, `kinsol_rs`,
  `ida_rs`, `idas_rs`, `arkode_rs`. Solver crates depend on the core, never
  on each other.
* 141 modules, one per upstream C file, keeping the exact C function names,
  constants and return-flag conventions (`CV_SUCCESS = 0`; negative fatal,
  positive recoverable), plus a 9-module deterministic libm.
* Serial only. No MPI, GPU, KLU, SuperLU, LAPACK, Fortran or XBraid backends.
* The crate tree is **inherited unchanged** from the sibling port
  [`SUNDIALS_7_8_Rust_port_for_AppleSilicon_macos`](https://github.com/once-ere/SUNDIALS_7_8_Rust_port_for_AppleSilicon_macos),
  where the 141 modules were translated from the C sources. No solver code
  is re-derived here. The work in this repository is target-platform work.

## Quick start

```bash
cargo build --workspace
cargo run -p cvode_rs --example cvRoberts_dns
```

**Requires an FMA-capable x86-64 CPU** (Intel Haswell 2012+, AMD Piledriver
2011+). `.cargo/config.toml` pins `-C target-feature=+fma` so that
`f64::mul_add` compiles to an instruction rather than a call into the C
runtime — without it the port would still depend on the host's `fma`. glibc
itself only selects the FMA build of these routines when the CPU reports FMA,
so bit-exactness with the reference outputs was never available without one.

The routines also assume the **default floating-point environment**
(round-to-nearest-even, no flush-to-zero). glibc's `sin`, `cos` and `atan`
force round-to-nearest for the duration of the call; safe Rust cannot reach
MXCSR, so this port inherits the ambient mode instead. Nothing in SUNDIALS or
Rust changes it — but a linked C dependency could, and no test here could
detect that.

```rust
use cvode_rs::prelude::*;
```

The verification harness is a POSIX `bash` script — run it from **Git Bash**
or MSYS2, not `cmd.exe` or PowerShell — and it needs the read-only upstream
SUNDIALS 7.8.0 C tree, which this workspace does *not* sit inside:

```bash
SUNDIALS_C_TREE=/c/Users/nsh/Developer/sundials-7.8.0 tools/verify_examples.sh all
```

## Platform scope

**Every numerical result claimed here was established on Windows 11 Pro for
Workstations 10.0.26200 (25H2), x86-64, `ucrtbase.dll` 10.0.26100.8521,
rustc 1.91.1, target `x86_64-pc-windows-msvc`.** The Rust sources are
portable — `std` only, no `unsafe`, no FFI, no `cfg(target_os)` or
`cfg(target_arch)` anywhere in the tree — so they compile and unit-test on
any target Rust supports. What does not travel is the numerical evidence.

### Why the host libm had to go

The upstream reference `.out` files were generated on a **glibc** host, and
Rust's `f64::sin`, `f64::exp`, … are documented as having *unspecified
precision* and forward to the **host** libm — on this target, the Microsoft
UCRT. `tools/libm_fingerprint_win.sh` builds the same Rust probe natively on
Windows and inside a WSL2 glibc guest and hashes 1,000,000 results per
function. The verdict:

| function | Windows UCRT vs glibc 2.39 |
|---|---|
| `sqrt` | **same** — IEEE-754 specifies it |
| `sin`, `cos`, `tan`, `exp`, `ln`, `log10`, `asin`, `acos`, `atan`, `sinh`, `cosh`, `acosh`, `tanh` | **all differ** |
| `powf` — the host routine this port deliberately does *not* call | **differs** |

Inside an adaptive integrator a one-ulp difference forks the step-size
trajectory and therefore the printed output. With the host libm in the path
this port stood at **125 IDENTICAL**; the 54 divergent variants were a strict
superset of the Linux sibling's 26, and every one of the 28 extra evaluated at
least one differing function.

So the host libm was removed. `crates/sundials_core/src/sundials_libm/` now
implements `exp`, `log`, `expm1`, `log1p`, `sin`, `cos`, `atan`, `asin`,
`acos`, `sinh`, `cosh` and `acosh` in pure Rust, each a translation of the
implementation glibc 2.39 selects on x86-64, and each measured against a real
glibc oracle: **0 mismatches over 8,000,000 inputs per routine.** All 218 call
sites were rewired, and the gate moved to **153 IDENTICAL** — the Linux
result, on the Linux set.

The detail that makes it work: glibc's x86-64 build ifunc-dispatches `exp`,
`log`, `pow`, `sin`, `cos`, `atan`, `asin`, `acos`, `expm1` and `log1p` to an
FMA rebuild of the generic source, so those fuse `a*b + c` into one `fma` and
the Rust must use `f64::mul_add` in exactly the same places; `sinh`, `cosh`
and `acosh` have no FMA variant and must not fuse. Rust never contracts on its
own, which is what makes that reproducible.

### `pow`, which was done first

`crates/sundials_core/src/sundials_math.rs` contains `pow_glibc`, a
pure-Rust port of the ARM optimized-routines / musl `pow` (MIT, © 2018 Arm
Limited) — the same algorithm glibc ≥ 2.28 ships as
`sysdeps/ieee754/dbl-64/e_pow.c`, and on x86-64 the same FMA-contracted
build glibc ifunc-dispatches to as `__ieee754_pow_fma`. `SUNRpowerR` routes
through it instead of `f64::powf`. It predates `sundials_libm` and stays
where it is; the newer module follows the same reasoning for the other
twelve routines.

On Windows that substitution is load-bearing, and both halves of the claim
are measured natively:

* **vs glibc:** `tools/pow_differential_win.sh` builds the oracle inside a
  WSL2 Linux guest (real glibc, real x86-64) and feeds it to a test binary
  compiled by `x86_64-pc-windows-msvc` and run natively. **0 mismatches over
  5,900,000 domain inputs and 0 over 20,000,000 unrestricted finite
  inputs.**
* **vs the host:** `pow_deterministic_vs_host_powf` compares it in process
  against the UCRT `pow`. **4,926 of those 5,900,000 domain inputs — 1 in
  1,198 — round differently, always by 1 ulp.** Every one of those is a
  digit the port would have got wrong had it called `f64::powf`.

The libm is not the only host-C-library dependence that had to go:
`ark_analytic_lsrk_domeigest`, `ark_brusselator_lsrk_domeigest` and
`ark_brusselator_lsrk_externaldomeigest` reproduce the BSD/glibc `rand()`
TYPE_3 additive-feedback generator in Rust, sequence for sequence, because
those examples feed pseudo-random vectors into a dominant-eigenvalue
estimator and the draws are output-observable. See [`NOTICE`](NOTICE).

### What the 26 remaining divergences are

They are the Linux port's 26, variant for variant, and that port root-caused
every one of them against a pristine C build on glibc: **Rust == pristine C in
all 26 cases, so the references are stale and the port is not wrong
anywhere.** 15 are whitespace-only (`SUN_TABLE_WIDTH` column drift — every
printed *value* identical); the other 11 are two LAPACK→native substitutions,
two upstream `.out` anomalies, five references with trailing whitespace
stripped, and two missing a final blank line the source prints
unconditionally.

### Licence — the deterministic libm changes it

Six of the twelve routines are translations of glibc, which is
**LGPL-2.1-or-later**; bit-exactness *is* the requirement and the IBM
Accurate Mathematical Library tables are copied data, so there is no route to
the same bits that avoids it. `exp` and `log` are MIT (musl / ARM
optimized-routines) and the SUNDIALS translation stays BSD-3-Clause. The
encumbered code is confined to `crates/sundials_core/src/sundials_libm/`,
each file carries its `SPDX-License-Identifier`, and dropping it costs
byte-identity, not correctness. See [`NOTICE`](NOTICE).

## Sibling ports

Each platform is a separate repository, never conditional compilation
inside one tree.

| repository | target | gate |
|---|---|---|
| [`…_for_AppleSilicon_macos`](https://github.com/once-ere/SUNDIALS_7_8_Rust_port_for_AppleSilicon_macos) | macOS / arm64 / Apple libm | 127 / 52 / 20 |
| [`…_for_Linux`](https://github.com/once-ere/SUNDIALS_7_8_Rust_port_for_Linux) | Linux / x86-64 / glibc 2.36–2.41 | 153 / 26 / 20 |
| **this one** | Windows 11 / x86-64, host libm replaced | **153 / 26 / 20** |

## Head-to-head against the C, built here with MSVC

The upstream C examples were built on this machine with **Visual Studio 18
Professional** and run beside the Rust port over the same 179 comparable
variants. Full write-up in [`differences/`](differences/):

| | variants |
|---|---:|
| C and Rust byte-identical | **131** |
| C and Rust differ | **48** |
| — Rust matches the shipped reference, C does not | **41** |
| — C matches the shipped reference, Rust does not | **0** |
| — neither matches (stale references) | 7 |
| outputs containing a solver error — Rust | **0** |
| outputs containing a solver error — C | **1** (`cvsDiurnal_FSA_kry -sensi sim t` exhausts `mxstep`) |

Against the references shipped with SUNDIALS 7.8.0, the Rust port is
byte-identical on **153** of 179 and the MSVC C build on **112** — because
the references were generated on glibc, and this port reproduces glibc's libm
while a C binary built here links the Microsoft UCRT's. Re-running both sides
from scratch reproduced all 358 outputs bit for bit.

## Documentation

| file | contents |
|---|---|
| [`current_status.md`](current_status.md) | **start here** — measured state, deficiencies, what remains |
| [`c-results/`](c-results/) | the C built with Visual Studio 18 Professional: build configuration, per-variant results, raw outputs |
| [`rust-results/`](rust-results/) | the Rust port under `cargo`: per-variant results, raw outputs |
| [`differences/`](differences/) | the two side by side, each against the shipped references, with root causes |
| [`sundials.md`](sundials.md) | public guide — crate map, worked example, C-to-Rust API conventions |
| [`ARCHITECTURE.md`](ARCHITECTURE.md) | handle model, locked porting patterns, numbered deviation classes |
| [`VERIFICATION.md`](VERIFICATION.md) | per-variant matrix: Windows results, then the inherited Linux and macOS evidence |
| [`PROGRESS.md`](PROGRESS.md) | per-file port status |
| [`STATUS.md`](STATUS.md) | what is done, what remains, how to resume |
| [`POW_FMA_EXACTNESS.md`](POW_FMA_EXACTNESS.md) | how far the deterministic `pow` is bit-exact, and the limits of that claim |
| [`evidence/windows-x86_64-ucrt/`](evidence/windows-x86_64-ucrt/) | raw artefacts behind every number above |
| [`crates/sundials_core/src/sundials_libm.rs`](crates/sundials_core/src/sundials_libm.rs) | the deterministic libm: why it exists, what glibc runs on x86-64, provenance per routine |

## Licence

Derivative work of SUNDIALS, **BSD-3-Clause**, Copyright © 2002–2026
Lawrence Livermore National Security, Southern Methodist University,
University of Maryland Baltimore County and the SUNDIALS contributors.

**This tree is mixed-licence.** The deterministic `pow` in
`crates/sundials_core/src/sundials_math.rs`, and `sundials_libm/exp.rs` and
`log.rs`, are pure-Rust ports of ARM optimized-routines code taken via musl,
**MIT**, Copyright © 2018 Arm Limited. The other six modules of
`sundials_libm` — `expm1`, `log1p`, `sincos`, `atan`, `asincos`,
`hyperbolic` — are translations of **glibc 2.39** and are therefore
**LGPL-2.1-or-later**, Copyright © The Free Software Foundation, with
portions from the IBM Accurate Mathematical Library, Copyright ©
International Business Machines Corp.

BSD-3-Clause and MIT are both LGPL-compatible, so the combined work is
distributable — but a binary linking those six modules must satisfy
LGPL-2.1-or-later. They are confined to one directory, each carries its own
`SPDX-License-Identifier`, and removing them costs byte-identical output,
not correctness. `NOTICE` opens with the full position.

Not an LLNL product; not endorsed by the SUNDIALS project. See `sundials.md`
§8 and `NOTICE`.
