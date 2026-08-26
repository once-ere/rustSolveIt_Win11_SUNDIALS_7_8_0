# current_status.md — SUNDIALS_7_8_Rust_port_for_Windows11

**Read this file first when resuming.** Session of 2026-08-10.

**Status: the port is complete and green on Windows 11 / x86-64. The host libm
has been removed from the code entirely, and the example gate now reads
153 IDENTICAL / 26 divergent / 20 excluded — the same result as the
Linux/glibc sibling, on exactly the same 26 variants.**

| gate | result | verdict |
|---|---|---|
| `cargo build --workspace` (native `x86_64-pc-windows-msvc`) | **0 errors, 0 warnings** | PASS |
| `cargo test --workspace` | **40 passed, 0 failed** | PASS |
| all 108 in-scope example programs build and run | 199 variants, **0 build failures, 0 run failures, 0 missing references** | PASS |
| deterministic `pow` vs **glibc `pow`** | 5,900,000 + 20,000,000 inputs, **0 mismatches** | PASS |
| deterministic libm (12 routines) vs **glibc 2.39**, development corpus | 4,000,000 inputs each — **48,000,000 comparisons, 0 mismatches** | PASS |
| deterministic libm, **out-of-sample** corpus | 8,000,000 inputs each — **96,000,000 comparisons, 0 mismatches**, the second half never seen while porting | PASS |
| every corpus begins with 56 exceptional inputs (NaN, ±inf, ±0, subnormals, domain boundaries, overflow thresholds) | all special-value branches measured | PASS |
| same, with `mul_add` forced back to a call into the host `fma` | 0 mismatches — the two codegen paths agree | PASS |
| host libm reachable from the port | **no** — 0 call sites outside `sundials_libm` | PASS |
| `tools/verify_examples.sh all` (199 reference variants) | **153 IDENTICAL / 26 divergent / 20 excluded (KLU/SuperLU)** | PASS — parity with Linux |
| the 26 divergences | **exactly** the Linux port's set, variant for variant; 15 whitespace-only, 11 content | reference-side |
| port defects | **0 identified** (see §7 item 1 for what would make that "proven" here) | — |

Measured host: Windows 11 Pro for Workstations 10.0.26200.8655 (25H2),
Intel Core Ultra 9 275HX, `ucrtbase.dll` 10.0.26100.8521, rustc/cargo 1.91.1,
target `x86_64-pc-windows-msvc`, harness run from Git Bash (MSYS2 3.6.5).
The glibc reference oracles come from the WSL2 guest `Ubuntu-24.04`
(glibc 2.39-0ubuntu8.7).

Raw artefacts: [`evidence/windows-x86_64-ucrt/`](evidence/windows-x86_64-ucrt/).

> ### Published
>
> `main` is on
> `https://github.com/once-ere/SUNDIALS_7_8_Rust_port_for_Windows11.git`.
> All work lands directly on `main`. The `credential.helper` problem the Linux
> sibling recorded on this machine is gone — both the system and the user
> gitconfig now say `manager`, and `git push` works unattended.

---

## 1. What this project is

A pure-Rust port of SUNDIALS 7.8.0 (LLNL) scoped to **Windows 11 on Intel/AMD
x86-64**. No `unsafe`, no FFI, no external crates, `std` only. Seven crates,
141 SUNDIALS modules plus a 9-module deterministic libm, keeping the exact C
names, constants and return-flag conventions.

Per the task's hard requirement it **reuses the entire crate tree** of the
sibling port `SUNDIALS_7_8_Rust_port_for_AppleSilicon_macos`, where the 141
modules were translated from the C sources. Not one line of solver code was
re-derived. The tree arrived by way of `SUNDIALS_7_8_Rust_port_for_Linux`.
Everything new here is target-platform work.

## 2. The libm question — the whole story

### 2.1 `pow`, the one that was already done

The task set `pow` as the gating item, on the grounds that the deterministic
`pow` inherited from the macOS port is a translation of the **ARM
optimized-routines / musl** implementation.

**The algorithm needed no rewrite; what it needed was a Windows
measurement.** ARM's routine *is* what glibc ≥ 2.28 ships as
`sysdeps/ieee754/dbl-64/e_pow.c`, and on x86-64 glibc ifunc-dispatches to
`__ieee754_pow_fma`, the same source rebuilt with `-mfma -mavx2
-ffp-contract=fast`; the Rust already reproduces that contraction map, and
nothing in it is AArch64-specific. `tools/pow_differential_win.sh` builds the
oracle with the guest `cc` inside WSL2 — real glibc, real x86-64 — and hands
the bit-stream to a binary compiled by `x86_64-pc-windows-msvc` and executed
natively: **0 mismatches over 5,900,000 domain inputs and 0 over 20,000,000
unrestricted finite inputs.**

On Windows that substitution is load-bearing in a way it was not on Linux.
`pow_deterministic_vs_host_powf` measures the gap in process: **4,926 of the
same 5,900,000 domain inputs — 1 in 1,198 — round differently under the
Microsoft UCRT, always by exactly 1 ulp.**

Writing a second, Windows-specific `pow` was considered and rejected: it
would replace a routine *measured* bit-exact against the target with an
unmeasured rewrite. (The C skeleton suggested in the task prompt is the same
ARM algorithm with an `exp(y*log(x))` tail spliced in place of the exact
reconstruction, elided tables, and a fallback to the host `pow` for special
cases — neither pure Rust nor bit-exact, and it would have *reintroduced* the
UCRT dependency this port exists to remove.)

### 2.2 The other ten, which is what actually closed the gap

A bit-exact `pow` alone left the gate at 125/54/20, because `pow` was the
*only* routine taken off the host. `tools/libm_fingerprint_win.sh` builds the
same Rust probe natively and inside the glibc guest and shows the problem
plainly: of the functions the port evaluates, **only `sqrt` agrees with
glibc**; `sin`, `cos`, `tan`, `exp`, `ln`, `log10`, `asin`, `acos`, `atan`,
`sinh`, `cosh`, `acosh`, `tanh` and the host `powf` all differ.

So [`crates/sundials_core/src/sundials_libm/`](crates/sundials_core/src/sundials_libm/)
now carries all of them:

| routine | translated from | licence | corpus | mismatches |
|---|---|---|---|---:|
| `exp` | musl `src/math/exp.c` + `exp_data.c` (ARM optimized-routines) | MIT | 8,000,000 | **0** |
| `log` | musl `src/math/log.c` + `log_data.c` (ARM optimized-routines) | MIT | 8,000,000 | **0** |
| `expm1` | glibc `s_expm1.c` | LGPL-2.1+ | 8,000,000 | **0** |
| `log1p` | glibc `s_log1p.c` | LGPL-2.1+ | 8,000,000 | **0** |
| `sin`, `cos` | glibc `s_sin.c` + `sincostab.c` + `branred.c` + `usncs.h` (IBM Accurate Mathematical Library) | LGPL-2.1+ | 8,000,000 each | **0** |
| `atan` | glibc `s_atan.c` + `atnat.h` + `uatan.tbl` (IBM APML) | LGPL-2.1+ | 8,000,000 | **0** |
| `asin`, `acos` | glibc `e_asin.c` + `uasncs.h` + `asincos.tbl` + `root.tbl` + `powtwo.tbl` (IBM APML) | LGPL-2.1+ | 8,000,000 each | **0** |
| `sinh`, `cosh`, `acosh` | glibc `e_sinh.c`, `e_cosh.c`, `e_acosh.c` | LGPL-2.1+ | 8,000,000 each | **0** |

**The FMA-contraction split is the detail that makes this work.** glibc's
x86-64 build ifunc-dispatches `exp`, `log`, `pow`, `sin`, `cos`, `atan`,
`asin`, `acos`, `expm1` and `log1p` to an FMA rebuild of the generic source
(`-mfma -mavx2 -ffp-contract=fast`), so those fuse `a*b + c` into a single
`fma` — and the Rust must use `f64::mul_add` in exactly the same places.
`sinh`, `cosh` and `acosh` have **no** FMA variant in
`sysdeps/x86_64/fpu/multiarch/`: they are the generic build against the SSE2
x86-64 baseline, so nothing is fused and the Rust must *not* use `mul_add`.
Rust never contracts on its own, which is what makes the distinction
expressible at all.

### 2.3 Wiring

`tools/route_libm_calls.py` rewrote **218 call sites in 45 files** — 9 in
library code (`SUNRexp`; the sensitivity checks in `cvodes.rs` and `idas.rs`;
`SUNRlog`/`SUNRsinh`/`SUNRcosh`/`SUNRacosh` in `arkode_lsrkstep.rs`) and 209
in the examples:

```
(0.2 * yup).exp()   ->  sundials_libm::exp(0.2 * yup)
con_errors[i].ln()  ->  sundials_libm::log(con_errors[i])
```

which also moves the Rust closer to the C it was translated from. After it,

```bash
grep -rE '\.(sin|cos|tan|exp|ln|log10|asin|acos|atan|sinh|cosh|acosh|tanh|powf)\(\)' \
     crates/*/src crates/*/examples | grep -v sundials_libm/
```

returns nothing: **the host libm is no longer reachable from the port.**
`sqrt`, `mul_add`, `abs`, `copysign`, `floor`, `ceil`, `round` and `trunc`
stay — IEEE-754 specifies them and they are identical on every target.

## 3. Verification gate — Windows vs the sibling ports

| | macOS / arm64 | Linux / x86-64 glibc | **Windows 11 / x86-64 (here)** |
|---|---:|---:|---:|
| IDENTICAL | 127 | 153 | **153** |
| divergent | 52 | 26 | **26** |
| excluded (KLU/SuperLU) | 20 | 20 | 20 |
| build failures / run failures | 0 | 0 | **0** |
| total variants | 199 | 199 | 199 |

Before the libm work this port stood at 125/54/20. The 28 extra divergences
and the one variant that had shifted from whitespace-only to content-divergent
(`ark_harmonic_symplectic`) are all closed. What remains is **exactly** the
Linux port's 26, variant for variant — `comm` over the two summaries reports
no difference — and the content/whitespace split matches too: **15
whitespace-only** (`SUN_TABLE_WIDTH` 28 → 29 column drift in references that
predate the change; every printed *value* identical) and **11 content**.

Those 11 are the ones the Linux port root-caused against a pristine C build
on glibc: two LAPACK→native substitutions (`cv[s]Roberts_dnsL`), two upstream
`.out` anomalies (`cv[s]Pendulum_dns`), five references with trailing
whitespace stripped (`cvsKrylovDemo_ls` ×4, `idasAkzoNob_ASAi_dns`), and two
missing a final blank line the source prints unconditionally
(`ark_conserved_exp_entropy_ark 1 1`, `ark_dissipated_exp_entropy 1 1`).
There Rust == pristine C in all 26 cases; **the references are stale, the port
is not wrong anywhere.**

## 4. Licence — read this before redistributing

The tree is now a **mixed-licence work**:

| part | licence |
|---|---|
| the SUNDIALS translation (`crates/*`) | BSD-3-Clause |
| `sundials_math.rs` `pow`, `sundials_libm/{exp,log}.rs` | MIT (ARM optimized-routines, via musl) |
| `sundials_libm/{expm1,log1p,sincos,atan,asincos,hyperbolic}.rs` | **LGPL-2.1-or-later** (glibc) |

Bit-exactness with glibc *is* the requirement, and the IBM Accurate
Mathematical Library tables are copied data, not independently derivable — so
there is no route to the same bits that avoids the LGPL. BSD-3-Clause and MIT
are both LGPL-compatible, so the combined work is distributable, but a binary
linking those six modules must satisfy LGPL-2.1-or-later. The encumbered code
is confined to one directory, each file carries its `SPDX-License-Identifier`,
and dropping it costs byte-identity, not correctness. `NOTICE` opens with the
summary; the upstream C is not committed (`tools/fetch_libm_sources.sh`
fetches it into the gitignored `reference/`).

## 5. Adversarial review, and what it changed

After the routines were measured green, three independent reviewers were run
over `sundials_libm` with one instruction each: find what a differential test
*cannot* catch. They found four things worth acting on, and all four are now
closed. They are recorded here because each one was a real hole in the
evidence, not in the arithmetic.

**1. `mul_add` was calling into the host C runtime.** The module claimed the
host libm was out of the path; it was not, quite. `f64::mul_add` lowers to
`llvm.fma.f64`, and the default `x86_64-pc-windows-msvc` baseline is SSE2,
which has no FMA instruction — so LLVM emitted a call instead. Verified on
this toolchain: `a.mul_add(b, c)` compiles to `jmp fma`, a tail call into
`ucrtbase.dll`; with the feature enabled it compiles to `vfmadd213sd`. That
is value-safe only if the runtime's `fma` is correctly rounded.
*Resolution:* `.cargo/config.toml` now pins `-C target-feature=+fma` for
x86-64, and the whole differential was re-run **both ways** — with the
instruction and with the libcall — giving 0 mismatches in both. So the UCRT's
`fma` is correctly rounded here (as IEEE-754 requires), every earlier
measurement stands, and the property is now enforced by the build rather than
assumed of the host. Consequence: binaries from this workspace require an
FMA-capable CPU (Haswell 2012+ / Piledriver 2011+) — not a new restriction,
since glibc only selects the FMA build of these routines when the CPU reports
FMA, so bit-exactness was never available without it.

**2. glibc's `SET_RESTORE_ROUND` cannot be reproduced, and the code said it
was a no-op.** glibc wraps `sin`, `cos` and `atan` in
`SET_RESTORE_ROUND (FE_TONEAREST)`, which rewrites the MXCSR rounding-control
bits for the duration of the call — so those routines answer in
round-to-nearest even when the caller is in another mode. Two comments in the
port dismissed this as an x86-64 no-op. That was wrong about the C: on
x86-64 `SET_RESTORE_ROUND_53BIT` *is* `SET_RESTORE_ROUND`, and it does touch
MXCSR. *Resolution:* the comments are corrected, and the default
floating-point environment — round-to-nearest-even, no FTZ/DAZ — is now a
stated **precondition** of `sundials_libm`. It cannot be enforced from safe
`std` Rust, and no differential can detect its violation, because the
differential runs in exactly the environment it assumes. SUNDIALS never
changes it and neither does Rust, so it holds in practice.

**3. The differential tests passed vacuously without an oracle.** A missing
stream, a misspelled variable or a renamed file made all twelve report "not
run" and pass — so a green `cargo test` was not, on its own, evidence that
anything had been compared. *Resolution:* `SUNDIALS_LIBM_ORACLE_STRICT` turns
a missing oracle into a hard failure, and `tools/libm_differential_win.sh`
sets it. A run that was meant to measure can no longer report success having
measured nothing. (Verified: it panics.)

**4. The corpus never produced NaN, ±inf, or out-of-domain arguments.** Every
generator filtered to finite in-domain values, so the special-value branches
of all twelve routines — `exp(-inf)`, `log(0)`, `asin(2)`, `acosh(0.5)`, the
overflow and underflow tails, the ±0 sign paths — were unmeasured despite the
4,000,000-input headline. *Resolution:* both the C oracle and its Rust twin
now begin every corpus with **56 shared exceptional values** (NaN, ±inf, ±0,
±DBL_MAX, ±DBL_MIN, largest and smallest subnormals, ±1 and 1±1ulp, the exp
and sinh/cosh overflow thresholds, the tiny-argument cutoffs, π/2 multiples
and Payne–Hanek boundaries). One shared table for all twelve, because a value
out of domain for one is in domain for another. All oracles were regenerated
— every input hash changed — and every routine is still 0 mismatches.

## 6. Deficiencies

None outstanding against the task. For the record, the two limits that remain
are limits of *evidence*, not of the port, and both are in §6.

Two things are deliberately **not** claimed:

1. **Exhaustiveness.** 96,000,000 measured inputs across 12 routines is strong
   evidence, not proof; the input space is 2^64 per routine.
2. **Other glibc versions.** The oracle is glibc 2.39. The Linux sibling
   measured that `sin`/`cos`/`atan`/`asin`/`acos`/`exp`/`log` are stable
   across glibc 2.36–2.41 but that 2.44 changed `sinh`, `cosh` and `acosh`.
   This port now carries its own 2.39-equivalent versions of all of them, so
   it is *insulated* from that drift — which also means that on a host whose
   glibc differs from 2.39 the port reproduces 2.39, deliberately.
3. **The default floating-point environment.** Round-to-nearest-even and no
   FTZ/DAZ are preconditions, not guarantees — see §5 item 2.
4. **An FMA-capable CPU**, which the build now requires — see §5 item 1.

## 7. Open items (not blocking)

1. ~~**Pristine C build on Windows.**~~ **Done** — see
   [`differences/`](differences/). Upstream SUNDIALS 7.8.0 was built here with
   cmake + MSVC (Visual Studio 18 Professional, `cl` 19.51.36246, 363 targets,
   0 errors) and all 179 comparable variants were run three ways: Rust, that C
   build, and the shipped reference.

   The result is **not** the Linux port's "Rust == pristine C", and it cannot
   be: the two binaries link different C libraries by construction — the Rust
   reproduces glibc's libm, an MSVC binary links the UCRT's. What it does
   establish is the thing that matters for a defect hunt: over 179 variants
   there is **no case where the native C reproduces the upstream reference and
   the Rust does not** (0 of 48 disagreements), while there are 41 where the
   Rust does and the C does not. The Rust is byte-identical to the references
   on 153 variants against the C build's 112, and the only solver error
   produced by either side is on the C side (`cvsDiurnal_FSA_kry -sensi sim t`
   exhausts `mxstep` under the UCRT's `exp`; the Rust reproduces the reference
   exactly). Both sides re-ran bit-for-bit identically.

   A same-libm comparison — building the C against a glibc-equivalent libm —
   would still be a distinct and stronger experiment, and is not done.
2. **Bare-metal glibc oracle.** Every oracle came from a WSL2 guest. That is a
   real glibc/x86-64 userspace and the arithmetic cannot differ, but an oracle
   from a physical Linux box would remove the last question.
3. **Windows on ARM.** Out of scope and untested. The Rust carries no
   `cfg(target_arch)`, so it would compile; every numerical claim would have
   to be re-measured. The `sundials_libm` routines are host-independent by
   construction, so the expectation is that they carry — but expectation is
   not measurement.
4. **`tan`, `log10`, `tanh`, `atan2`** are not ported: nothing in the library
   or the examples calls them. If a future example does, it must be added to
   `sundials_libm` rather than taken from the host.

## 8. How to reproduce, from a clean checkout

On Windows 11 x86-64 with rustup, Git Bash, and (for the oracles) a WSL2 Linux
guest:

```bash
cargo build --workspace                 # 0 warnings
cargo test  --workspace                 # 40 passed
tools/pow_differential_win.sh all       # pow: 0 mismatches / 25.9M inputs
tools/libm_differential_win.sh all      # 12 routines: 0 mismatches / 48M inputs
tools/libm_fingerprint_win.sh           # which host functions differ from glibc
```

For the out-of-sample run, generate a longer corpus into a separate directory:

```bash
SUNDIALS_ORACLE_OUT="$PWD/logs/oracle8m" tools/libm_differential_win.sh all 8000000
```

The example gate additionally needs the read-only upstream SUNDIALS 7.8.0 C
tree; unlike the sibling ports this workspace does not live inside it, so name
it explicitly:

```bash
SUNDIALS_C_TREE=/c/Users/youruser/Developer/sundials-7.8.0 tools/verify_examples.sh all
SUNDIALS_C_TREE=/c/Users/youruser/Developer/sundials-7.8.0 tools/classify_diffs.sh
```

Then read `logs/summary.txt` and `logs/classify_diffs.txt`.

## 9. Provenance

* **Upstream:** SUNDIALS 7.8.0, LLNL, BSD-3-Clause. Read-only reference at
  `C:\Users\youruser\Developer\sundials-7.8.0` on the machine this was built on.
* **Crate tree:** inherited wholesale from
  `SUNDIALS_7_8_Rust_port_for_AppleSilicon_macos` (BSD-3-Clause), by way of
  `SUNDIALS_7_8_Rust_port_for_Linux`. `ARCHITECTURE.md`, `PROGRESS.md` and
  Parts B and C of `VERIFICATION.md` come from there and describe the
  translation, which is platform-independent.
* **Deterministic `pow`:** ARM optimized-routines via musl, MIT, © 2018 Arm
  Limited. See `NOTICE` and `POW_FMA_EXACTNESS.md`.
* **Deterministic libm:** musl (MIT) for `exp`/`log`; glibc 2.39
  (LGPL-2.1-or-later) for the other six modules. See `NOTICE` §
  "Deterministic libm".
* **`rand()` reproduction** in three `ark_*_lsrk_domeigest` examples: the
  BSD/glibc TYPE_3 additive-feedback generator, reimplemented in Rust because
  the draws are output-observable. See `NOTICE`.
* **New in this repository:** `crates/sundials_core/src/sundials_libm/`,
  `tools/{libm_oracle.c, libm_differential_win.sh, libm_probe.rs,
  libm_fingerprint_win.sh, fetch_libm_sources.sh, route_libm_calls.py,
  pow_differential_win.sh}`, the `$SUNDIALS_C_TREE` support in the two harness
  scripts, `evidence/windows-x86_64-ucrt/`, this file, and the Windows scoping
  of every document.
