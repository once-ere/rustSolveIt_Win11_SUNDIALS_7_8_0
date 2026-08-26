# c-results — SUNDIALS 7.8.0 C examples built with Visual Studio 18 Professional

What the **C** implementation does on this machine. Built from the upstream
SUNDIALS 7.8.0 sources with MSVC, out of source; the upstream tree is never
written to.

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

Build script: [`tools/build_c_examples.cmd`](../tools/build_c_examples.cmd)
(library + examples) and
[`tools/build_c_lapack_substituted.cmd`](../tools/build_c_lapack_substituted.cmd)
(the four `*L` examples, see below). Run harness:
[`tools/example_matrix.py`](../tools/example_matrix.py). Raw captured stdout
for every variant is in [`outputs/`](outputs/), one file per variant, named
exactly like the reference file it corresponds to.

## Provenance — the recorded chain from source to result

Every command line below is **quoted from the file that recorded it**, not
restated. [`../VERIFY.md`](../VERIFY.md) is a step-by-step guide to checking
all of it yourself.

| file | what it lets you check |
|---|---|
| [`provenance/00-environment.txt`](provenance/00-environment.txt) | host, UTC start/finish, full path and version banner of `cl.exe`, `link.exe`, `cmake`, `ninja`; the MSVC toolset and Windows SDK/UCRT chosen by `vcvars64.bat`; the complete `INCLUDE` and `LIB` paths |
| [`provenance/01-configure-cmd.txt`](provenance/01-configure-cmd.txt) | the literal `cmake` configure command line |
| [`provenance/02-configure-out.txt`](provenance/02-configure-out.txt) | everything CMake printed |
| [`provenance/03-CMakeCache.txt`](provenance/03-CMakeCache.txt) | every option CMake resolved, including defaults |
| [`provenance/04-compile_commands.json`](provenance/04-compile_commands.json) | **the exact `cl.exe` line for each of the 245 translation units** |
| [`provenance/05-build-cmd.txt`](provenance/05-build-cmd.txt) | the literal build command line |
| [`provenance/06-build-out.txt`](provenance/06-build-out.txt) | `ninja -v`: every compile *and link* as executed (388 lines) |
| [`provenance/10-lapacksub-cmd.txt`](provenance/10-lapacksub-cmd.txt) | for each `*L` example, every line that differs from upstream, and the `cl.exe` line used |
| [`provenance/11-lapacksub-out.txt`](provenance/11-lapacksub-out.txt) | compiler/linker output for those four |
| [`provenance/20-input-sources.sha256`](provenance/20-input-sources.sha256) | SHA-256 of every C source compiled |
| [`provenance/21-binaries.sha256`](provenance/21-binaries.sha256) | SHA-256 of every binary produced |
| [`provenance/22-outputs.sha256`](provenance/22-outputs.sha256) | SHA-256 of every captured output |

### The configure command, verbatim

```text
cmake -G Ninja -S "C:\Users\youruser\Developer\sundials-7.8.0" -B "C:\Users\youruser\Developer\github\SUNDIALS_7_8_Rust_port_for_Windows11\tools\..\logs\c-build" ^
  -DCMAKE_BUILD_TYPE=Release ^
  -DCMAKE_C_COMPILER=cl ^
  -DCMAKE_EXPORT_COMPILE_COMMANDS=ON ^
  -DBUILD_SHARED_LIBS=OFF ^
  -DBUILD_STATIC_LIBS=ON ^
  -DEXAMPLES_ENABLE_C=ON ^
  -DEXAMPLES_ENABLE_CXX=OFF ^
  -DEXAMPLES_INSTALL=OFF ^
  -DBUILD_TESTING=OFF ^
  -DSUNDIALS_INDEX_SIZE=64 ^
  -DSUNDIALS_PRECISION=double ^
  -DENABLE_LAPACK=OFF -DENABLE_KLU=OFF -DENABLE_SUPERLUMT=OFF ^
  -DENABLE_SUPERLUDIST=OFF -DENABLE_MPI=OFF -DENABLE_OPENMP=ON ^
  -DENABLE_PTHREAD=OFF -DENABLE_HYPRE=OFF -DENABLE_PETSC=OFF ^
  -DENABLE_TRILINOS=OFF -DENABLE_CUDA=OFF -DENABLE_HIP=OFF ^
  -DENABLE_SYCL=OFF -DENABLE_RAJA=OFF -DENABLE_KOKKOS=OFF ^
  -DENABLE_GINKGO=OFF -DENABLE_XBRAID=OFF -DENABLE_CALIPER=OFF ^
  -DENABLE_ADIAK=OFF -DBUILD_FORTRAN_MODULE_INTERFACE=OFF
```

run from the environment established by
`"C:\Program Files\Microsoft Visual Studio\18\Professional\VC\Auxiliary\Build\vcvars64.bat"`,
then built with

```text
cmake --build "C:\Users\youruser\Developer\github\SUNDIALS_7_8_Rust_port_for_Windows11\tools\..\logs\c-build" --parallel -- -v
```

### What the compiler was actually told, for one file

`cvRoberts_dns.c`, quoted from `04-compile_commands.json` (line breaks added
for reading; the recorded command is one line):

```text
C:\PROGRA~1\MICROS~3\18\PROFES~1\VC\Tools\MSVC\1451~1.362\bin\Hostx64\x64\cl.exe  /nologo -DSUNDIALS_STATIC_DEFINE -D_CRT_SECURE_NO_WARNINGS
  -IC:\Users\youruser\Developer\sundials-7.8.0\include
  -IC:\Users\youruser\Developer\github\SUNDIALS_7_8_Rust_port_for_Windows11\logs\c-build\include
  -IC:\Users\youruser\Developer\sundials-7.8.0\src\sundials
  -IC:\Users\youruser\Developer\github\SUNDIALS_7_8_Rust_port_for_Windows11\logs\c-build\src\sundials
  /DWIN32
  /D_WINDOWS /O2 /Ob2
  /DNDEBUG -MD /Foexamples\cvode\serial\CMakeFiles\cvRoberts_dns.dir\cvRoberts_dns.c.obj /Fdexamples\cvode\serial\CMakeFiles\cvRoberts_dns.dir\ /FS
  -c C:\Users\youruser\Developer\sundials-7.8.0\examples\cvode\serial\cvRoberts_dns.c
```

So: `/O2 /Ob2` optimisation, `/DNDEBUG`, `-MD` (dynamic UCRT), 64-bit indices
and double precision as configured. Nothing is inferred here — read the JSON.

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
| C source files attempted | **180** |
| of those, built | **114** |
| ran to completion (exit 0) | **189** |
| of those, printed a solver error anyway | **1** |
| non-zero exit or timeout | **49** |
| excluded (KLU / SuperLU_MT) | **20** |
| total | 258 |

Against the reference outputs shipped with SUNDIALS 7.8.0:

| C vs shipped `.out` | variants |
|---|---:|
| byte-identical | **112** |
| whitespace-only difference | **13** |
| content difference | **54** |

Those shipped references were generated on a **glibc** host. This build links
the Microsoft UCRT, whose `sin`, `cos`, `exp`, `log`, `asin`, `acos`, `atan`,
`sinh`, `cosh`, `acosh` and `pow` all differ from glibc's in the last ulp
(measured: [`../evidence/windows-x86_64-ucrt/libm_fingerprint.txt`](../evidence/windows-x86_64-ucrt/libm_fingerprint.txt)).
That is the dominant reason this column is not 199, and it is a property of
the *platform*, not of the C code. See [`../differences/ANALYSIS.md`](../differences/ANALYSIS.md).

## The four `*L` examples

`cvRoberts_dnsL`, `cvAdvDiff_bndL`, `cvsRoberts_dnsL` and `cvsAdvDiff_bndL`
call a LAPACK linear solver, and there is no LAPACK here, so the main build
skips them. They are built separately with exactly the substitution the Rust
port also makes — `sunlinsol_lapackdense.h` → `sunlinsol_dense.h`,
`SUNLinSol_LapackDense` → `SUNLinSol_Dense`, and the band equivalents — so
that both sides run the same algorithm and the comparison stays honest. This
is recorded per variant in [`RESULTS.md`](RESULTS.md).

## Files

| file | contents |
|---|---|
| [`RESULTS.md`](RESULTS.md) | every variant: exit status, output size, agreement with the shipped reference |
| [`EXCLUSIONS.md`](EXCLUSIONS.md) | every example not built, with the reason |
| [`outputs/`](outputs/) | raw captured stdout+stderr, one file per variant |
| [`provenance/`](provenance/) | build environment, literal command lines, compiler invocations, checksums |
| [`../VERIFY.md`](../VERIFY.md) | how to check every claim here yourself |
