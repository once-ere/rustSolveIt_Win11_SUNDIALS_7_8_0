# rust-results — the pure-Rust port, built and run with cargo

What the **Rust** implementation does on the same machine, in the same
session, over the same 199 variants.

## Provenance

| item | value |
|---|---|
| generated | `2026-08-11T12:34:51Z` |
| repository commit | `bc4e2b3` |
| operating system | Microsoft Windows 11 Pro for Workstations 10.0.26200.0 |
| CPU | Intel(R) Core(TM) Ultra 9 275HX |
| C compiler | Microsoft (R) C/C++ Optimizing Compiler Version 19.51.36246 for x64 |
| CMake | cmake version 4.1.2 |
| Rust | rustc 1.91.1 (ed61e7d7e 2025-11-07) / cargo 1.91.1 (ea2d97820 2025-10-10) |
| upstream sources | SUNDIALS 7.8.0, `examples/` as copied into this repository |

| file | what it lets you check |
|---|---|
| [`provenance/00-environment.txt`](provenance/00-environment.txt) | host, UTC start/finish, `rustc -vV` and `cargo -V` with full paths, release profile, warning/error counts |
| [`provenance/01-build-cmd.txt`](provenance/01-build-cmd.txt) | the literal cargo command lines |
| [`provenance/02-build-out.txt`](provenance/02-build-out.txt) | `cargo build -v`: **every `rustc` invocation as executed** (123 lines) |
| [`provenance/03-cargo-config.txt`](provenance/03-cargo-config.txt) | `.cargo/config.toml` verbatim — the source of `-C target-feature=+fma` |
| [`provenance/04-Cargo.lock.txt`](provenance/04-Cargo.lock.txt) | the resolved dependency set: 7 workspace crates, nothing external |
| [`provenance/20-input-sources.sha256`](provenance/20-input-sources.sha256) | SHA-256 of every example source compiled |
| [`provenance/21-binaries.sha256`](provenance/21-binaries.sha256) | SHA-256 of every binary produced |
| [`provenance/22-outputs.sha256`](provenance/22-outputs.sha256) | SHA-256 of every captured output |

### The build command, verbatim

```text
cargo clean
cargo build --release --workspace --examples -v
```

### What rustc was actually told, for the core library

Quoted from `02-build-out.txt` (line breaks added for reading):

```text
C:\Users\nsh\.rustup\toolchains\stable-x86_64-pc-windows-msvc\bin\rustc.exe
  --crate-name sundials_core
  --edition=2021 'crates\sundials_core\src\lib.rs'
  --error-format=json
  --json=diagnostic-rendered-ansi,artifacts,future-incompat
  --crate-type lib
  --emit=dep-info,metadata,link
  -C opt-level=3
  -C embed-bitcode=no
  --check-cfg 'cfg(docsrs,test)'
  --check-cfg 'cfg(feature, values())'
  -C metadata=21b2f8dad5df29c2
  -C extra-filename=-4c97891d540723de
  --out-dir 'C:\Users\nsh\Developer\github\SUNDIALS_7_8_Rust_port_for_Windows11\target\release\deps'
  -C strip=debuginfo -L 'dependency=C:\Users\nsh\Developer\github\SUNDIALS_7_8_Rust_port_for_Windows11\target\release\deps'
  -C target-feature=+fma
```

`-C target-feature=+fma` is not typed on the command line — it comes from
`.cargo/config.toml`, so it applies to every compilation in this workspace.
Run the check in [`../VERIFY.md`](../VERIFY.md) §3 to count how many `rustc`
invocations carried it.

Raw captured output per variant is in [`outputs/`](outputs/); the run harness
is [`tools/example_matrix.py`](../tools/example_matrix.py).

## What "ported" means here

The examples were not written for this exercise: they are part of the port and
were translated line by line from the same C programs this comparison builds,
one Rust file per C file, keeping the C function names, constants and output
formatting. `crates/<solver>_rs/examples/<name>.rs` corresponds to
`examples/<solver>/serial/<name>.c`. All **108** in-scope serial programs are
ported and build clean; `cargo build --release --workspace --examples`
produces 0 warnings.

The port is `std`-only: no `unsafe`, no FFI, no external crates. It does not
call the host libm — `crates/sundials_core/src/sundials_libm/` implements
`exp`, `log`, `expm1`, `log1p`, `sin`, `cos`, `atan`, `asin`, `acos`, `sinh`,
`cosh` and `acosh`, and `sundials_math.rs` implements `pow`, each measured
bit-identical to glibc 2.39 over 8,000,000 inputs per routine. That is the
single most important fact for reading [`../differences/`](../differences/):
**the Rust reproduces glibc's libm, the C build links the Microsoft UCRT's.**

## Results

### Scope — every C example in the repository was attempted

There is no pre-selected subset here. All **180 `.c` files** under
`examples/` were compiled one at a time, and all **258 (example, argv)
variants** declared by the `CMakeLists.txt` files of all **29 example
directories** were run. Where a program did not build, the compiler's own
error is recorded per file in
[`../c-results/provenance/31-build-each-example.txt`](../c-results/provenance/31-build-each-example.txt)
— nothing is excluded by assumption.

The C++ (46), Fortran (51) and CUDA (7) sources are not covered: they are not
C, and this project is a C-to-Rust port.

| outcome | variants |
|---|---:|
| variants whose example is ported to Rust | **179** |
| variants with no Rust counterpart | **59** |
| ran to completion (exit 0) | **179** |
| of those, printed a solver error anyway | **0** |
| non-zero exit or timeout | **59** |
| excluded (KLU / SuperLU_MT) | **20** |
| total | 258 |

Against the reference outputs shipped with SUNDIALS 7.8.0:

| Rust vs shipped `.out` | variants |
|---|---:|
| byte-identical | **153** |
| whitespace-only difference | **15** |
| content difference | **11** |

## Files

| file | contents |
|---|---|
| [`RESULTS.md`](RESULTS.md) | every variant: exit status, output size, agreement with the shipped reference |
| [`EXCLUSIONS.md`](EXCLUSIONS.md) | every example not ported, with the reason |
| [`outputs/`](outputs/) | raw captured stdout+stderr, one file per variant |
| [`provenance/`](provenance/) | build environment, literal command lines, compiler invocations, checksums |
| [`../VERIFY.md`](../VERIFY.md) | how to check every claim here yourself |
