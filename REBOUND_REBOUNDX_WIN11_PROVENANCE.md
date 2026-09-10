# REBOUND_REBOUNDX_WIN11_PROVENANCE — the N-body ports, verified on this Windows 11 PC

This page records how the two pure-Rust astronomy libraries in this
repository — `rebound_rust/` and `reboundx_rust/` — got here, and every
command used to build, test, verify and run them on **Windows 11**, with
the measured result of each. It is self-contained: every command you need
appears on this page. All commands are PowerShell (the Git Bash form is
given in parentheses where it differs) and start from the repository root,
`rustSolveIt_Win11_SUNDIALS_7_8_0`.

The measured numbers on this page come from two records made on this
machine on 2026-09-09: `rebound_rust/logs/win11_c_reference_gates_20260909.md`
(the C-reference gates — 542 lines, every command and every number, with
the twelve `rebound_rust/logs/win11_cref_*_20260909.log` transcripts beside
it) and the build/test/clippy/notebook logs
`rebound_rust/logs/win11_{build,test,clippy,notebooks}_20260909.log` and
`logs/reboundx_{build,test,clippy}.log`. Where this page quotes a number,
it is that record's number.

The macOS / Apple Silicon edition of this page, from the macOS repository
this one was ported from, is kept verbatim as `REBOUND_REBOUNDX_MACOS_PROVENANCE.md`
(lineage; its numbers were measured on a Mac, not here). The Windows-side
companions of this page are `PORT_WIN11_PROVENANCE.md`,
`VERIFICATION_WIN11.md` and `WINDOWS_PORT_COMPLETE.md`.

## 1. What these two folders are

- **`rebound_rust/`** (crate `rebound_rs` 5.1.1) — a pure-Rust, line-for-line
  translation of [REBOUND](https://github.com/hannorein/rebound), the
  open-source N-body gravity simulator by Hanno Rein and collaborators
  (version 5.1.1, commit `dad5f97806ecbb408dcaff728851c64e67f9f6eb`).
  28 source modules plus `lib.rs` (29 `.rs` files), 13 example programs, 394 automated tests, and one
  Jupyter notebook per example in `rebound_rust/notebooks/`.
- **`reboundx_rust/`** (crate `reboundx_rs` 5.1.0) — a pure-Rust translation
  of [REBOUNDx](https://github.com/dtamayo/reboundx) by Dan Tamayo, Hanno
  Rein and collaborators, which adds extra physics (general relativity,
  tides, spin, migration, radiation forces…) to a REBOUND simulation.
  33 source modules plus `lib.rs` (34 `.rs` files), 4 example programs, 137 automated tests; its
  notebooks live beside the REBOUND ones.

Both are **zero `unsafe`, zero external dependencies (std only), zero
build warnings**, keep the exact C function names, and are
**GPL-3.0-or-later** (each folder carries its own LICENSE — note this
differs from the BSD-3-Clause license of the physics simulator in the
rest of this repository; the folders are independent works that live
together here).

They were first produced and verified bit-for-bit on Windows 11, as the
physics acceptance test of the first rustSolveIt Windows port; the macOS
repository then re-verified them against Apple clang on an Apple Silicon
Mac. This repository — the Windows 11 port of that macOS repository —
carries them **verified again on Windows 11, on this machine, against the
original C compiled with Microsoft's compiler here**: the same
experiments, re-run today.

The master document — written for a reader who has never programmed, with
the complete story and every command — is `rebound_rust/rebound_rust.md`
(also typeset as `rebound_rust/rebound_rust.pdf`). It has stayed the
Windows edition throughout (see section 3). You do not need it to
reproduce the results below; everything needed is on this page.

## 2. Machine and toolchain (as measured, 2026-09-09)

| item | value |
|---|---|
| machine | Intel Core Ultra 9 275HX, x86-64 |
| OS | Windows 11 Pro for Workstations, build 10.0.26200 |
| C compiler (C references and harnesses only) | MSVC `cl.exe` — banner `Microsoft (R) C/C++ Optimizing Compiler Version 19.51.36256 for x64`, from the MSVC 14.51.36231 toolset of **Visual Studio 2026 Build Tools**, `C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Tools\MSVC\14.51.36231\bin\Hostx64\x64\cl.exe` |
| linker | `Microsoft (R) Incremental Linker Version 14.51.36256.0` (same toolset) |
| compiler environment script | `C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat`, always invoked through `cmd /c '"...vcvars64.bat" >nul && ...'` from PowerShell |
| make (REBOUND's own library build) | GNU Make 3.81, `C:\Program Files (x86)\GnuWin32\bin\make.exe` |
| Rust | `rustc 1.91.1 (ed61e7d7e 2025-11-07)`, `cargo 1.91.1`, host `x86_64-pc-windows-msvc`; the root `.cargo/config.toml` pins `-C target-feature=+fma` (the CPU must have FMA — this one does) |
| Python (notebooks only) | 3.14.5 (default text encoding cp1252), nbformat 5.10.4, nbclient 0.10.2, jupyter_client 8.6.3, ipykernel 7.1.0 |
| shell | PowerShell 7.6.5 for the measurements; the 63-configuration sweep script ran under Windows PowerShell as its documentation says |
| REBOUND reference | github.com/hannorein/rebound, commit `dad5f97806ecbb408dcaff728851c64e67f9f6eb` (`git describe --tags`: `5.1.1-4-gdad5f978`, `reb_version_str = "5.1.1"`) |
| REBOUNDx reference | 5.1.0 — an unpacked `reboundx-main.zip` archive, **not a git checkout**, so no commit id can be reported; version confirmed by `version.txt`, `src/core.c` (`rebx_version_str = "5.1.0"`) and `changelog.md` |

Three things to say plainly about the toolchain:

- **No WSL2 compiler or linker was used anywhere.** Everything on this page
  ran natively on Windows. Also installed on this machine but **not used**:
  Visual Studio 2026 Professional/Enterprise, Visual Studio 2022, and the
  Intel oneAPI 2025.3/2026.1 compilers.
- **The only C compiler used anywhere in this repository is that one MSVC
  `cl.exe`.** It compiles two things: the bundled SQLite amalgamation inside
  `dataset_tools/` (through Rust's `cc` crate, found automatically — no
  developer prompt), and the REBOUND/REBOUNDx **C reference** libraries and
  the sixteen comparison harnesses on this page (explicitly, through
  `vcvars64.bat`). The simulator workspace itself, and both Rust crates on
  this page, are pure Rust; the MSVC linker is located by `rustc` on its own.
- Every `vcvars64.bat` invocation on this machine first prints
  `'vswhere.exe' is not recognized as an internal or external command`. It
  is noise from the developer-prompt script and has no effect: `cl`, `lib`
  and `link` are found, and every build below exited 0.

The Rust build of both crates touches no network (std-only crates), so the
C compiler is needed **only** to reproduce the comparison gates of
section 6, never to use the libraries.

## 3. What changed in this port's rebound folders

This repository's `rebound_rust/` and `reboundx_rust/` are the union of
the first Windows port's folders (the Windows story is current here) and
the material the macOS repository added on top of them. Nothing changed in
any executable line of Rust in this port. The complete list:

1. **The macOS rustdoc backtick fixes were taken** — doc-comment text
   only, in 7 files: `rebound_rust/src/binarydata.rs`, `rebound_rust/src/server.rs`,
   `reboundx_rust/src/lense_thirring.rs`, `tides_constant_time_lag.rs`,
   `tides_dynamical.rs`, `tides_spin.rs`, `yarkovsky_effect.rs` (11 lines
   in all: bare `<placeholder>` and `particles[0]`-style spans wrapped in
   backticks so that newer rustdocs accept them under the crates'
   `deny(warnings)`). No executable line moved; both test suites are green
   here afterwards (section 4).
2. **The platform-neutral notebook generator was taken** —
   `rebound_rust/notebooks/make_notebooks.py` no longer hard-codes the
   `.exe` suffix (it now appends `.exe` only when `os.name == "nt"`, at
   five sites), so the same generator produces working notebooks on
   Windows and on macOS/Linux.
3. **`tides_spin_pseudo.ipynb` is now generated.** The first Windows
   folder had defined but never carried it; **every one of the 17 examples
   now has exactly one companion notebook**, and all 17 were regenerated
   and executed on this machine (section 7).
4. **The macOS shim and the POSIX sweep are kept as cross-platform
   extras**, unused on Windows: `rebound_rust/porttest/macos_shim/rand_r_glibc.c`
   (upstream REBOUND vendors glibc's `rand_r` only under `#ifdef _WIN32`,
   so on macOS the C reference needs this shim to reproduce the random
   initial conditions — on Windows the vendored code is compiled in and
   no shim is needed) and `rebound_rust/porttest/run_integrator_matrix.sh`,
   the POSIX twin of the PowerShell sweep `run_integrator_matrix.ps1`
   that this page uses. The macOS `.gitignore` additions
   (`notebooks/.venv/`, `porttest/macos_shim/*.o`) were taken too.
5. **The Windows documentation is kept**: `rebound_rust/rebound_rust.md`
   / `.tex` / `.pdf`, both READMEs, and the three acceptance write-ups
   (`rebound_rust/shearing_sheet_port_test.md`,
   `rebound_rust/reboundx_port_test.md`, `reboundx_rust/reboundx_port_test.md`)
   are the Windows editions — they describe MSVC `/fp:precise`, the MSVC
   `pow` difference and the MSVC variable-length-array shim as the current
   story, which on this machine they are. The macOS re-platforming of the
   same documents is lineage, recorded by `REBOUND_REBOUNDX_MACOS_PROVENANCE.md`.
6. **The Windows porttest dumps are kept**: the five committed dump files
   that differed between the two editions (`rebound_rust/porttest/archive_state_c.txt`,
   `derivatives_c.txt`, `derivatives_rust.txt`, `frequency_c.txt`,
   `frequency_rust.txt`) are the Windows ones, and today's runs reproduced
   their committed content (section 6). The three C harness sources the
   macOS edition touched were identical modulo CR bytes and are unchanged.
7. **The 16 macOS arm64 Mach-O harness binaries** the macOS repository had
   committed under `rebound_rust/porttest/` and `reboundx_rust/porttest/`
   (extensionless names such as `porttest/libm_diff`) were untracked and
   ignored here: they are build products, rebuilt from their `.c` sources
   by the commands in section 5.
8. **This session's records were added** under `rebound_rust/logs/`: the
   gates log `win11_c_reference_gates_20260909.md`, its twelve
   `win11_cref_*_20260909.log` transcripts, and `win11_build`, `win11_test`,
   `win11_clippy`, `win11_notebooks` (17 files; the REBOUNDx build/test/clippy
   logs are `logs/reboundx_*.log` at the repository root).
9. The upstream C trees are **not** committed: the root `.gitignore`
   reserves `/rebound/` and `/reboundx/` at the repository root for the
   local reference trees used by section 5. The comparison `.bin` archives,
   the harness `.exe`/`.obj`/`.lib` files and the multi-megabyte dump files
   are gitignored for the same reason (every one is regenerated by the
   commands on this page).

## 4. Build and test (commands + measured results)

Open PowerShell in the repository root and run:

```powershell
Set-Location rebound_rust
cargo build --release --all-targets
cargo test  --release
cargo clippy --release --all-targets
cargo doc   --no-deps
Set-Location ..\reboundx_rust
cargo build --release --all-targets
cargo test  --release
cargo clippy --release --all-targets
cargo doc   --no-deps
Set-Location ..
```

(Git Bash: `cd rebound_rust`, `cd ../reboundx_rust`, `cd ..`; the cargo
lines are identical.)

To keep a record as this port did:

```powershell
Set-Location rebound_rust
cargo build --release --all-targets 2>&1 | Tee-Object logs\win11_build_20260909.log
cargo test  --release               2>&1 | Tee-Object logs\win11_test_20260909.log
cargo clippy --release --all-targets 2>&1 | Tee-Object logs\win11_clippy_20260909.log
Set-Location ..
```

(Git Bash: `2>&1 | tee logs/win11_build_20260909.log`.)

Measured on this machine (`rebound_rust/logs/win11_{build,test,clippy}_20260909.log`,
`logs/reboundx_{build,test,clippy}.log`):

| crate | build | tests | clippy |
|---|---|---|---|
| `rebound_rs` | `Finished release` in 4.41 s, **0 warnings** | **394 passed, 0 failed** = 67 (core_io) + 32 (gravity_tree_collision) + 39 (integrators_adaptive) + 72 (integrators_hybrid) + 55 (integrators_symplectic) + 70 (tools) + 59 (transforms_rotations_derivatives) | **0 warnings** |
| `reboundx_rs` | `Finished release` in 3.47 s, **0 warnings** | **137 passed, 0 failed** = 34 (core_params) + 32 (forces_conservative) + 28 (forces_dissipative) + 17 (interpolation_and_spin) + 26 (operators_and_integrators); 2 doctests ignored | **0 warnings** |

Every example binary the notebooks and gates use is built by the first
line of each block (`--all-targets`); they land in
`rebound_rust\target\release\examples\*.exe` (13 programs) and
`reboundx_rust\target\release\examples\*.exe` (4 programs).

## 5. The C references, built on this machine

This section is needed only to reproduce section 6. It builds the original
C libraries with MSVC and the sixteen small C "harness" programs that dump
raw IEEE-754 bit patterns for comparison with their Rust twins.

**Compile C only from PowerShell, through `cmd /c '...'` exactly as written.**
Git Bash's `cmd //c` can fail silently and leave a stale `.exe` behind, so
that you unknowingly test an old build; the first Windows port lost real
time to this. Everything else on this page works from either shell.

Define the repository path once per terminal (every block below uses it):

```powershell
$repo = (Get-Location).Path      # run from the repository root
```

### 5a. Get the sources

```powershell
git clone https://github.com/hannorein/rebound.git rebound\rebound
Set-Location rebound\rebound
git checkout dad5f97806ecbb408dcaff728851c64e67f9f6eb
Set-Location ..\..
git clone https://github.com/dtamayo/reboundx.git reboundx
```

(Git Bash: `rebound/rebound`, `cd rebound/rebound`, `cd ../..`.) Both
folders are gitignored. On this machine they were directory junctions to
sibling folders holding the same trees (`rebound` at the commit above;
`reboundx` an unpacked `reboundx-main.zip` of version 5.1.0 — see
section 2 — so a fresh clone gives you a commit the record cannot name;
its `version.txt` should still read `5.1.0` for the comparison to be the
same one).

### 5b. REBOUND: `librebound.dll`, its import library, and the static library

REBOUND's own makefile builds the DLL. GnuWin32 must be put on `PATH`
**before** `vcvars64.bat` runs, exactly as written (Windows expands
`%PATH%` when it reads the line; the other order silently discards what
`vcvars64.bat` set up and you get "cl is not recognized"):

```powershell
Set-Location "$repo\rebound\rebound\examples\shearing_sheet"
cmd /c 'set PATH=C:\Program Files (x86)\GnuWin32\bin;%PATH% && "C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat" && make'
```

The transcript of that build on this machine is
`rebound_rust/logs/win11_cref_rebound_make_20260909.log`; it records every
per-file command, all of the form

```
cl -c /DBUILDINGLIBREBOUND /D_GNU_SOURCE /D_CRT_SECURE_NO_WARNINGS /D_CRT_NONSTDC_NO_WARNINGS /Ox /fp:precise -DSERVER -DGITHASH=dad5f97806ecbb408dcaff728851c64e67f9f6eb /Fo:rebound.obj rebound.c
```

followed by the `cl 19.51.36256` banner, then the `link` into
`librebound.dll` (794,112 bytes) and `librebound.lib` (65,930 bytes), 31
objects in all. `/Ox` optimises; `/fp:precise` forbids the compiler from
re-associating floating-point arithmetic or fusing multiply-adds, which is
what makes a bit-exact comparison possible.

The harnesses that call REBOUND's *internal* (non-exported) functions
need a static library of the same 31 objects. Build it and copy it beside
both harness folders:

```powershell
Set-Location "$repo\rebound\rebound\src"
cmd /c '"C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul && lib /nologo /OUT:librebound_static.lib *.obj'
Copy-Item librebound_static.lib "$repo\rebound_rust\porttest\librebound_static.lib" -Force
Copy-Item librebound_static.lib "$repo\reboundx_rust\porttest\librebound_static.lib" -Force
Copy-Item librebound.lib, librebound.dll "$repo\rebound_rust\porttest\" -Force
```

Measured: exit 0; `librebound_static.lib` 885,762 bytes, 31 objects
archived. The DLL must sit beside the finished harness `.exe` files for
them to start, which is what the last line arranges.

### 5c. REBOUNDx: the library, and the MSVC variable-length-array shim

```powershell
Set-Location "$repo\reboundx\src"
cmd /c '"C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul && cl /nologo /c /I"..\..\rebound\rebound\src" /I"." /D_GNU_SOURCE /D_CRT_SECURE_NO_WARNINGS /D_CRT_NONSTDC_NO_WARNINGS /DLIBREBOUNDX /Ox /fp:precise *.c'
```

This **is expected to end with `cl` exit code 2**: 31 of the 33 sources
compile, and exactly two fail — `gr_full.c` (lines 63 and 185) and
`interpolation.c` (line 67), each with `error C2057: expected constant
expression`, `C2466` and `C2133`. They use C99 variable-length arrays
(`double a_const[N][3]`, `double u[n]`), which Microsoft's C compiler has
never supported. Today's run produced exactly those nine error lines and
nothing else.

The fix lives in `reboundx_rust/porttest/msvc_shim/` — copies of those two
files that take the same arrays from the heap (`malloc`/`free`) instead of
the stack: same element type, same indices, same values, same order; only
where the memory comes from changes, and the upstream tree is never
edited. Compile the two shims, drop their objects into the upstream
object set, and archive:

```powershell
Set-Location "$repo\reboundx_rust\porttest\msvc_shim"
cmd /c '"C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul && cl /nologo /c /I"..\..\..\rebound\rebound\src" /I"..\..\..\reboundx\src" /D_GNU_SOURCE /D_CRT_SECURE_NO_WARNINGS /D_CRT_NONSTDC_NO_WARNINGS /DLIBREBOUNDX /Ox /fp:precise gr_full.c interpolation.c'
Copy-Item gr_full.obj, interpolation.obj "$repo\reboundx\src\" -Force
Set-Location "$repo\reboundx\src"
cmd /c '"C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul && lib /nologo /OUT:libreboundx.lib *.obj'
Copy-Item libreboundx.lib "$repo\reboundx_rust\porttest\libreboundx.lib" -Force
```

Measured: shims exit 0 (`gr_full.obj` 24,129 bytes, `interpolation.obj`
6,474 bytes); 33 objects; `libreboundx.lib` 334,960 bytes, exit 0.
Transcript: `rebound_rust/logs/win11_cref_reboundx_build_20260909.log`.
The shim affects the **C reference build only**; the Rust port uses `Vec`
and has no such problem.

### 5d. The sixteen comparison harnesses

Eleven for REBOUND. Nine link against the import library
`librebound.lib`; `libm_diff.c` and `bs_pow_diff.c` are pure maths-library
probes and link against nothing; `problem_test.c` (the C side of the
shearing sheet) is named `rebound_test.exe`:

```powershell
Set-Location "$repo\rebound_rust\porttest"
foreach ($f in 'addfmt_test','archive_test','derivatives_test','frequency_test','integrators_test','kepler_rectilinear_c','movetocom_var_c','movetocom_var_test') {
  cmd /c "`"C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat`" >nul && cl /nologo /I`"..\..\rebound\rebound\src`" /D_GNU_SOURCE /D_CRT_SECURE_NO_WARNINGS /D_CRT_NONSTDC_NO_WARNINGS /Ox /fp:precise $f.c librebound.lib /Fe:$f.exe"
}
cmd /c '"C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul && cl /nologo /I"..\..\rebound\rebound\src" /D_GNU_SOURCE /D_CRT_SECURE_NO_WARNINGS /D_CRT_NONSTDC_NO_WARNINGS /Ox /fp:precise problem_test.c librebound.lib /Fe:rebound_test.exe'
cmd /c '"C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul && cl /nologo /Ox /fp:precise libm_diff.c /Fe:libm_diff.exe'
cmd /c '"C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul && cl /nologo /Ox /fp:precise bs_pow_diff.c /Fe:bs_pow_diff.exe'
```

Measured (transcript `rebound_rust/logs/win11_cref_harness_build_20260909.log`):
all eleven built; one warning in total, `movetocom_var_test.c(28): warning
C4477` (a `%zu` given an `int`). If you see `unresolved external symbol
__imp_reb_...`, you linked against `librebound_static.lib` by mistake:
REBOUND's public functions are declared for DLL import, so the import
library is the one to use here.

Five for REBOUNDx, linked against the two static libraries (the shim
objects are already inside `libreboundx.lib`):

```powershell
Set-Location "$repo\reboundx_rust\porttest"
foreach ($f in 'tides_spin_pseudo_c','tides_spin_kozai_c','tides_spin_migration_c','rebx_binary_roundtrip_c','rebx_binary_read_c') {
  cmd /c "`"C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat`" >nul && cl /nologo /I`"..\..\rebound\rebound\src`" /I`"..\..\reboundx\src`" /D_GNU_SOURCE /D_CRT_SECURE_NO_WARNINGS /D_CRT_NONSTDC_NO_WARNINGS /Ox /fp:precise $f.c libreboundx.lib librebound_static.lib /Fe:$f.exe"
}
Set-Location $repo
```

Measured: all five exit 0. The linker prints `LNK4217`/`LNK4286`
"imported and statically defined" warnings — 67, 68, 66, 57 and 59 of
them respectively; they are expected and harmless (the REBOUNDx objects
were compiled expecting REBOUND in a DLL and are being given the static
library). The three `tides_spin_*_c.c` harnesses also print
`warning C4133 'function': incompatible types - from 'void **' to
'rebx_node **'` (8, 8 and 16 times) from the stock examples' own
`rebx_set_param_*` calls.

What was reused versus rebuilt today, for the record: the DLL, its import
library, the 31 REBOUND objects and the eleven REBOUND harnesses had been
built earlier the same day (08:01–08:02) by a previous session whose logs
prove the flags above, and were reused (SHA-256 of the porttest copies
equal to the `rebound\rebound\src` originals); `librebound_static.lib`, all
31 REBOUNDx objects, the two shim objects, `libreboundx.lib` and the five
REBOUNDx harnesses were rebuilt fresh at 16:50–16:52 because no log proved
their flags. The Rust example binaries were those of the 07:40 build in
`win11_build_20260909.log`; the notebooks' own `cargo build` cells reported
them up to date.

## 6. The verification record (every result measured here, 2026-09-09 16:48–16:54)

The method throughout: run the identical experiment in the MSVC-compiled C
and in Rust, dump every value as raw IEEE-754 bits, and compare byte for
byte. A gate passes only if **every bit of every value** matches. Every
comparison below was made with `Compare-Object (Get-Content <c>) (Get-Content <rust>)`
(prints nothing when the files agree) plus `Get-FileHash -Algorithm SHA256`
on both files. (Git Bash: `cmp <c> <rust>` and `sha256sum <c> <rust>`.)

| gate | measured on this machine | verdict |
|---|---|---|
| libm differential, 200,000 samples × 9 functions | sin, cos, tan, atan2, sqrt, fmod, exp, log: **0 differences**; `pow(x, -0.234)`: **60 of 200,000 differ (0.0300 %), every one exactly 1 ULP** | reproduced — the Windows `pow` difference, §6.1 |
| `pow` in the BS step-size-controller shapes, 200,000 samples | **56 of 200,000 differ (0.0280 %), all exactly 1 ULP**; 0 of the 8 constants differ | reproduced |
| integrator matrix, 63 configurations × 500 steps | **63 of 63 bit-identical**, 2.3 s | reproduced |
| ias15 default run, 1000 adaptive steps | 7/7 lines identical, SHA equal | reproduced |
| shearing sheet, 400 steps: 1,482 particles, 102,533 collisions | initial and final dumps byte-identical; final SHA-256 **`75BDAAB7109F125192F56AEB0CCDCAC554AF88A165FE11075EC5A871178521F0`** on both sides; C 2.230 s, Rust 1.700 s | reproduced |
| Simulationarchive round trips, whfast-usafe and ias15 | C→Rust, Rust→C and write-vs-write: **all 6 comparisons 0 differing lines**, SHA equal | reproduced |
| web server blob loaded by C | `served.bin` 3,448 bytes; C-loaded state identical to Rust's dump, 6/6 lines, SHA equal | reproduced |
| orbital derivatives, 65 functions × 2 configurations | **130/130 lines identical** (C file CRLF, Rust LF; byte-identical after CRLF→LF) | reproduced |
| frequency analysis MFT / FMFT / FMFT2 | 30/30 lines identical, SHA equal | reproduced |
| `add_fmt` and built-in datasets | 12/12 lines identical, SHA equal | reproduced |
| `movetocom_var` (MEGNO / variational move-to-com) | 5/5 lines identical, SHA equal | reproduced |
| `movetocom_var_test` (stdout probe) | all 5 raw-bit fields identical; text differs only in C `%e` exponent style (`e+00`) vs Rust (`e0`) | reproduced |
| `kepler_rectilinear` (11 cases) | all 11 raw-bit `AFTER` lines identical; 6/11 full stdouts identical, the 5 `vy=0` cases differ only in the header's exponent style | reproduced |
| REBOUNDx `tides_spin` pseudo / kozai / migration, short and long | **6 of 6 bit-identical** | reproduced |
| REBOUNDx binary files | Rust self round trip 25/25; Rust reads the C file 25/25 with byte-identical re-serialization; files 6,392 bytes, 6,366 identical, the 26 differing bytes (offsets 37–62) are the git-hash header field; C readback of both files identical 28/28 lines | reproduced |
| the 17 notebooks | **`All 17 notebooks executed with no errors.`** | done, §7 |

**Nothing failed.** The gates log lists, in its §7, everything that did not
come out *literally* as the guide words it; all of it is cosmetic and it is
repeated in §6.11 below.

Every block in this section assumes `$repo` from section 5 and that the
Rust examples were built by section 4.

### 6.1 The maths library, and the one known difference: `pow`

```powershell
Set-Location "$repo\rebound_rust\porttest"
.\libm_diff.exe
..\target\release\examples\libm_diff.exe
Get-FileHash libm_c.txt, libm_rust.txt -Algorithm SHA256 | Format-List Path, Hash
```

Measured: both dumps 30,600,000 bytes, 200,000 lines, 9 bit patterns per
line (`sin cos tan atan2 pow(xp,-0.234) sqrt fmod exp log`); C dump
0.243 s, Rust 0.065 s. The two files **differ** (their hashes are
`099663BC…` and `89D206AD…`), and the whole difference is the `pow`
column: 60 lines out of 200,000 (0.0300 %), each by exactly 1 ULP — the
first five at samples 4209, 7132, 7669, 12245 and 13496, e.g. sample 4209
C `3fd6473aac8c764d`, Rust `3fd6473aac8c764c`. All eight other functions
are bit-identical on all 200,000 samples.

To count the differences per function and their ULP sizes yourself, save
the following as `$env:TEMP\libm_ulp.py` and run `python $env:TEMP\libm_ulp.py`
inside `porttest` (Git Bash: `python "$TEMP/libm_ulp.py"`). It also reads
the `bs_pow` dumps of §6.2 if they are present:

```python
import os
names = "sin cos tan atan2 pow(xp,-0.234) sqrt fmod exp log".split()

def ordered(h):                      # 64-bit pattern -> sign-ordered integer
    v = int(h, 16)
    return v if v < 1 << 63 else (1 << 63) - v

diff = {n: {} for n in names}
with open("libm_c.txt") as fc, open("libm_rust.txt") as fr:
    for lc, lr in zip(fc, fr):
        for n, a, b in zip(names, lc.split(), lr.split()):
            if a != b:
                u = abs(ordered(a) - ordered(b))
                diff[n][u] = diff[n].get(u, 0) + 1
for n in names:
    k = sum(diff[n].values())
    print(f"{n:16s} differ={k:6d} ({100*k/200000:.4f}%)  ULP distribution={diff[n] or 'none'}")

if os.path.exists("bs_pow_c.txt"):
    def parse(path):
        d = {}
        with open(path) as fh:
            for line in fh:
                t = line.split()
                if t:
                    d[tuple(t[:-1])] = t[-1]
        return d
    c, r = parse("bs_pow_c.txt"), parse("bs_pow_rust.txt")
    ulp, perk, pmis = {}, {}, 0
    for key, a in c.items():
        b = r[key]
        if a != b:
            if key[0] == "P":
                pmis += 1
                continue
            u = abs(ordered(a) - ordered(b))
            ulp[u] = ulp.get(u, 0) + 1
            perk[int(key[1])] = perk.get(int(key[1]), 0) + 1
    n = sum(1 for k in c if k[0] == "E")
    print(f"bs_pow: mismatches={sum(ulp.values())} of {n} E samples; P-constant mismatches={pmis}")
    print("bs_pow: ULP distribution=", ulp, " per k=", dict(sorted(perk.items())))
```

Why this is the expected Windows result, and why it does not undermine the
gates: Rust ships its own `pow` while the C calls Microsoft's UCRT `pow`;
both are correct to within what the C standard requires and simply round a
handful of cases differently. Every other function the two languages
share on `x86_64-pc-windows-msvc` — including `exp` and `log` — is
bit-identical, which is why the Bridges bounce law in the shearing sheet
(`0.32 * pow(v, -0.234)`) is written as `exp(-0.234 * log(v))` on **both**
sides of that test, and why REBOUND's per-timestep arithmetic matches bit
for bit everywhere. `cbrt`, which the guide's prose also names, is not a
column of this harness and was not measured today. The guide's table
says "≤ 2 ULP" for the general sweep; **the measured maximum on this
machine is 1 ULP**. On the Mac, by contrast, the macOS lineage page
records `pow` as bit-identical too (both languages call Apple's libm
there) — a difference between the two *platforms'* C libraries, not
between the ports.

### 6.2 `pow` in exactly the shapes the BS step-size controller uses

```powershell
Set-Location "$repo\rebound_rust\porttest"
.\bs_pow_diff.exe
..\target\release\examples\bs_pow_diff.exe
python $env:TEMP\libm_ulp.py
```

Measured: both dumps 5,311,288 bytes, 200,008 lines (8 `P` constant lines
`pow(0.02, 1/(2k+1))` plus 200,000 `E` samples `pow(error/0.65, 1/(2k+1))`,
k = 1..8). **56 of 200,000 E samples differ (0.0280 %), all exactly 1 ULP;
0 of the 8 P constants differ**; per k: {1: 5, 2: 8, 3: 8, 4: 3, 5: 7,
6: 8, 7: 7, 8: 10}. First mismatch: `E 1 9857` C `3f9cab338c1d039b`,
Rust `3f9cab338c1d039a`. This is the same function, the same rate and the
same magnitude as §6.1, measured on the arguments that matter to the BS
integrator's step-size choice.

### 6.3 The integrator matrix — 63 configurations × 500 steps

A fixed three-body problem (star of mass 1; planet 10⁻³ at x = 1.6,
vy = 0.5; moon 10⁻⁷ at x = 1.7, vy = 0.6, z = 0.01, vz = 0.001), G = 1,
dt = 0.01, 500 steps, final state dumped as raw bits, once per
configuration in each language, then compared. The sweep script knows
every configuration name:

```powershell
Set-Location "$repo\rebound_rust\porttest"
powershell -NoProfile -ExecutionPolicy Bypass -File .\run_integrator_matrix.ps1 500
```

(Git Bash on Windows: `powershell -NoProfile -ExecutionPolicy Bypass -File ./run_integrator_matrix.ps1 500`;
the POSIX twin `bash run_integrator_matrix.sh 500` exists for macOS/Linux
and was not used here.) `-NoProfile` only keeps your PowerShell profile's
output out of the transcript; the script is unchanged.

Measured: all 63 lines read `identical`; tally
**`63 of 63 configurations bit-identical.` / `ALL CONFIGURATIONS BIT-IDENTICAL`**;
exit 0; 2.3 s wall. The configurations: `none`; `ias15`; `leapfrog` orders
2, 4, 6, 8; `whfast` default, `c11`, `c17`, `dh`, `whds`, `bary`, `mk`,
`comp`, `lazy`, `usafe`; `saba` default, 1, 2, 3, 4, `cm2`, `cl2`, `104`,
`864`, `h844`, `h864`, `h1064`, `usafe`; `janus` default, 2, 4, 8, 10;
`eos` default, the nine diagonals `0-0`…`8-8`, `2-7`, `5-8`, `usafe`;
`mercurius` default, `usafe`, `c4`, `c5`, `inf`, `hill01`; `bs` default,
`tight`, `loose`, `maxdt`; `trace` default, `pbs`, `ias15`, `hill1`,
`perinone`, `eta001`. Transcript: `rebound_rust/logs/win11_cref_integrator_matrix_20260909.log`.

To check one configuration by hand (`Compare-Object` printing nothing
means it matched):

```powershell
.\integrators_test.exe whfast 2 500
..\target\release\examples\integrators_test.exe whfast 2 500
Compare-Object (Get-Content state_c_final.txt) (Get-Content state_rust_final.txt)
```

**Every harness in `porttest\` writes to the same two filenames**,
`state_c_final.txt` and `state_rust_final.txt`, so each run overwrites the
last one's dumps; that is why the shearing-sheet pair (§6.5) was run
*after* the sweep, so that its dumps are the ones left on disk.

### 6.4 ias15, the default 1000-step adaptive run

```powershell
.\integrators_test.exe
..\target\release\examples\integrators_test.exe
Compare-Object (Get-Content state_c_final.txt) (Get-Content state_rust_final.txt)
Get-FileHash state_c_final.txt, state_rust_final.txt -Algorithm SHA256 | Format-List Path, Hash
```

Measured: C prints `ias15 done: t=9.12795929765748610e+01 steps=1000`,
Rust `ias15 done: t=9.127959297657486e1 steps=1000`; the dumps are 7/7
lines identical, 403 bytes each, SHA equal — `t 4056d1e4d9f0a524`,
`dt 3fa3537087ef2705`, `steps_done 1000`, and three particle lines. (The
committed `ref_c_ias15.txt` / `ref_rust_ias15.txt` hold the *archive*
ias15 write dump of §6.6, not this one — see §6.11.)

### 6.5 The shearing sheet — the acceptance test

The stock Saturn's-rings example: SEI integrator, octree gravity, tree
collision search, shear-periodic boundary, hard-sphere collisions with the
Bridges bounce law, `rand_r` initial conditions with seed 42.

```powershell
Set-Location "$repo\rebound_rust\porttest"
.\rebound_test.exe 400
..\target\release\examples\shearing_sheet_test.exe 400
Compare-Object (Get-Content state_c_final.txt) (Get-Content state_rust_final.txt)
Compare-Object (Get-Content state_c_init.txt)  (Get-Content state_rust_init.txt)
Get-FileHash state_c_final.txt, state_rust_final.txt, state_c_init.txt, state_rust_init.txt -Algorithm SHA256 | Format-List Path, Hash
```

Measured: both programs print `Toomre wavelength: 61.009899 | N after
init: 1482 | final: t=1.91217633050229269e+04 steps=400 collisions=102533`;
C 2.230 s, Rust 1.700 s. Final dumps 208,019 bytes, 1,488 lines, **both
SHA-256 `75BDAAB7109F125192F56AEB0CCDCAC554AF88A165FE11075EC5A871178521F0`**;
initial dumps 207,859 bytes, 1,483 lines, both
`82B4013994FFF67315AA126CAA3E993B45A66CD42028C1D00558D7F92EAC86F1`;
0 differing lines in either pair. Header of the final dump: `N 1482 |
t 40d2ac70d9fd4f96 | steps_done 400 | collisions_log_n 102533 |
rand_seed 2089691152`. Transcript: `rebound_rust/logs/win11_cref_shearing_sheet_20260909.log`.

(The macOS lineage page reports 102,478 collisions and SHA
`418c864d…` for the same 400 steps: on a Mac both languages call Apple's
libm, here both call the UCRT, and the two platforms' results differ from
*each other* while C and Rust agree with each other on each. The gate is
same-platform C versus Rust; that is what "bit-identical" claims.)

### 6.6 Simulationarchive — cross-language round trips

One implementation runs 3 × 100 steps, saving a snapshot after each 100.
The *other* loads snapshot 1 (the 200-step state), continues 100 more
steps, and must land on the writer's 300-step state bit-exactly. Both
directions, two integrators:

```powershell
Set-Location "$repo\rebound_rust\porttest"
foreach ($integ in 'whfast-usafe','ias15') {
  .\archive_test.exe $integ write
  Copy-Item archive_state_c.txt "$env:TEMP\arch_${integ}_c_write.txt"
  ..\target\release\examples\archive_test.exe $integ continue        # Rust continues archive_c_<integ>.bin
  "$integ  C write vs Rust continue:"; Compare-Object (Get-Content "$env:TEMP\arch_${integ}_c_write.txt") (Get-Content archive_state_rust.txt)
  ..\target\release\examples\archive_test.exe $integ write
  Copy-Item archive_state_rust.txt "$env:TEMP\arch_${integ}_rust_write.txt"
  .\archive_test.exe $integ continue                                 # C continues archive_rust_<integ>.bin
  "$integ  Rust write vs C continue:"; Compare-Object (Get-Content "$env:TEMP\arch_${integ}_rust_write.txt") (Get-Content archive_state_c.txt)
  "$integ  C write vs Rust write:";    Compare-Object (Get-Content "$env:TEMP\arch_${integ}_c_write.txt") (Get-Content "$env:TEMP\arch_${integ}_rust_write.txt")
}
```

Measured: **all six comparisons 0 differing lines, SHA equal** —
whfast-usafe dumps 375 bytes (6 lines), ias15 dumps 368 bytes; C prints
`write done: t=3.000000e+00`, Rust `write done: t=2.9999999999999583e0`
(the same number, `%e` versus `{:e}`), ias15 `t=2.802771e+01` /
`t=2.8027708766369308e1`. The whfast-usafe write dump on both sides begins
`t 4007ffffffffffa2`, `dt 3f847ae147ae147b`; the ias15 one
`t 403c0717ebf55e62`, `dt 3fa26ccb18d75140`. The archive files themselves
(`archive_c_whfast-usafe.bin` 5,108 bytes, `archive_rust_whfast-usafe.bin`
5,174, `archive_c_ias15.bin` 15,832, `archive_rust_ias15.bin` 15,865) are
gitignored: the C writes struct padding uncleared and its incremental
blobs carry wall-clock fields, so their lengths vary run to run (the C
ias15 file was 15,799 bytes in the 08:04 run) while the state they restore
does not. Transcript: `rebound_rust/logs/win11_cref_archive_20260909.log`.

### 6.7 The web server

The Rust example pauses a 100-step simulation and serves it over HTTP; the
blob is fetched and loaded by the C build:

```powershell
Set-Location "$repo\rebound_rust\porttest"
$srv = Start-Process ..\target\release\examples\server_test.exe -WorkingDirectory (Get-Location).Path -PassThru -NoNewWindow -RedirectStandardOutput "$env:TEMP\server_stdout.txt"
Start-Sleep -Seconds 2
curl.exe -s http://localhost:12873/simulation --output served.bin
curl.exe -s http://localhost:12873/keyboard/81          # key code 81 = 'Q' (quit): the server shuts down; prints ok.
.\archive_test.exe whfast load served.bin
Compare-Object (Get-Content archive_state_c.txt) (Get-Content server_state_rust.txt)
Get-Content "$env:TEMP\server_stdout.txt"
```

(Git Bash: `../target/release/examples/server_test.exe > "$TEMP/server_stdout.txt" &`,
`sleep 2`, then the same `curl.exe` lines and `./archive_test.exe whfast load served.bin`.)

Measured: `served.bin` 3,448 bytes, header `REBOUND Binary File. Version: 5.`,
SHA-256 `47368E55E6C4E933044B8A3C114220D963448A926E1DC13084B92091CB1403A2`;
C prints `load done: t=1.000000e+00`; `archive_state_c.txt` versus
`server_state_rust.txt` **6/6 lines identical**, 369 bytes each, SHA equal
(`t 3ff0000000000003`, `dt 3f847ae147ae147b`); the server printed
`REBOUND Webserver listening on http://localhost:12873 (not secure) ...`,
`Server shutting down...`, `server_test done` and exited 0. Transcripts:
`rebound_rust/logs/win11_cref_server_20260909.log`, `win11_cref_server_stdout_20260909.log`.

### 6.8 Derivatives, frequency analysis, `add_fmt`, and the two defect probes

```powershell
Set-Location "$repo\rebound_rust\porttest"
.\derivatives_test.exe ; ..\target\release\examples\derivatives_test.exe
Compare-Object (Get-Content derivatives_c.txt) (Get-Content derivatives_rust.txt)
.\frequency_test.exe   ; ..\target\release\examples\frequency_test.exe
Compare-Object (Get-Content frequency_c.txt) (Get-Content frequency_rust.txt)
.\addfmt_test.exe      ; ..\target\release\examples\addfmt_test.exe
Compare-Object (Get-Content addfmt_c.txt) (Get-Content addfmt_rust.txt)
.\movetocom_var_c.exe  ; ..\target\release\examples\movetocom_var.exe
Compare-Object (Get-Content movetocom_var_c.txt) (Get-Content movetocom_var_rust.txt)
.\movetocom_var_test.exe > "$env:TEMP\mvt_c.txt" ; ..\target\release\examples\movetocom_var_test.exe > "$env:TEMP\mvt_rust.txt"
Compare-Object (Get-Content "$env:TEMP\mvt_c.txt") (Get-Content "$env:TEMP\mvt_rust.txt")
.\kepler_rectilinear_c.exe 1e-12 1 2.0 ; ..\target\release\examples\kepler_rectilinear.exe 1e-12 1 2.0
```

(Git Bash: replace ` ; ` by ` && `, `Compare-Object (Get-Content a) (Get-Content b)` by `cmp a b`, and `> "$env:TEMP\..."` by `> "$TEMP/..."`.)

Measured (transcript `rebound_rust/logs/win11_cref_pairs_20260909.log`):

- **derivatives** — both print `derivatives_c.txt written (65 functions x 2 configs)`
  (Rust: the `_rust` file); **130/130 lines, 0 differing**. The C file is
  17,028 bytes and the Rust file 16,898: the C harness opens its file in
  text mode, so on Windows it gets CRLF line ends (130 CR bytes); after
  CRLF→LF the two are byte-identical (`True`). `git status` shows
  `derivatives_c.txt` modified for that reason only
  (`git diff --ignore-cr-at-eol` is empty).
- **frequency** — 30/30 lines, 0 differing, 546 bytes each, SHA
  `352C0162E64102ADAA22CAB6F43B7AE55211AA45F7FDEB6ACE87AE5440BBC018`
  on both.
- **addfmt** — `addfmt done N=12` on both; 12/12 lines, 0 differing,
  1,454 bytes, SHA `7551686E6E484F1376FD36518D6732A7FE51094A7C40F3E50E40D7880E72846C`.
- **movetocom_var** (the defect-3 probe: a Sun+Jupiter MEGNO run) — both
  print `com.m 3ff003e8f2d96218 com.x 3f7351b0cb646c7f`; 5/5 lines,
  0 differing, 436 bytes, SHA `07DBBC89144DC335C0D487BE94634A3DAD1F3449A830DC8098C121177436E7D3`.
- **movetocom_var_test** — 8 lines each side; `Compare-Object` reports 14
  entries (7 lines × 2 sides) because C prints `%e` exponents as
  `e+00`/`e-03` and Rust as `e0`/`e-3`; after rewriting only that
  notation the outputs are identical, and the five raw-bit fields agree
  exactly: `3ff003e8f543dbc4 3f753880595c6070 3f2649e164cc8800 bfcd1838694a61b4 bfc6cd3aa79f4e0b`.
- **kepler_rectilinear** (the defect-1 probe) — 11 cases: the notebook's
  `1e-12 1 2.0` and vy ∈ {0, 1e-12} × dt ∈ {0.1, 0.5, 2.0, 5.0, −2.0} at
  n = 20 (run each as `.\kepler_rectilinear_c.exe <vy> <n> <dt>` and the
  Rust twin with the same three arguments). **All 11 `AFTER:` raw-bit
  lines identical**; full stdout identical in 6/11 — the five `vy=0` cases
  differ only in the `BEFORE:` header (`vy=0.00000000000000000e+00` in C
  versus `e0` in Rust). For instance `vy=1e-12 n=20 dt=2.0` gives
  `x=405de2243c66f45a y=3dc5be612de023a0 vx=4007a61d075c3ff2 vy=3d715ac1eb46a876`
  on both sides. The `1e-12 1 2.0` case returned normally in Rust — the
  pre-fix hang did not recur.

### 6.9 REBOUNDx: the `tides_spin` acceptance tests

Three of REBOUNDx's own shipped examples in both languages, short
(notebook-argument) and long (full default) runs. Send the C program's
error stream to `$null`: REBOUNDx prints a warning on *every* timestep.
The long runs go first so that the short dumps are the ones left on disk
for the notebooks:

```powershell
Set-Location "$repo\reboundx_rust\porttest"
foreach ($t in @(@('pseudo','628.3185307179586'),@('kozai','100000.0'),@('migration','628.3185307179586'),
                 @('pseudo','62.83185307179586'),@('kozai','1000.0'),@('migration','62.83185307179586'))) {
    $n = $t[0]
    & ".\tides_spin_${n}_c.exe" $t[1] 2>$null | Out-Null
    & "..\target\release\examples\tides_spin_$n.exe" $t[1] 2>$null | Out-Null
    $d = Compare-Object (Get-Content "state_${n}_c.txt") (Get-Content "state_${n}_rust.txt")
    "$n $($t[1]) : $(if ($d) { 'MISMATCH' } else { 'BIT-IDENTICAL' })"
}
```

(Git Bash, the same six runs — `pseudo` and `migration` take 628.3185307179586 then 62.83185307179586, `kozai` takes 100000.0 then 1000.0:
`for n in pseudo migration; do for t in 628.3185307179586 62.83185307179586; do ./tides_spin_${n}_c.exe $t 2>/dev/null; ../target/release/examples/tides_spin_$n.exe $t 2>/dev/null; cmp state_${n}_c.txt state_${n}_rust.txt && echo "$n $t BIT-IDENTICAL"; done; done; for t in 100000.0 1000.0; do ./tides_spin_kozai_c.exe $t 2>/dev/null; ../target/release/examples/tides_spin_kozai.exe $t 2>/dev/null; cmp state_kozai_c.txt state_kozai_rust.txt && echo "kozai $t BIT-IDENTICAL"; done`.)

Measured — **6 of 6 BIT-IDENTICAL**, every row 0 differing lines and SHA
equal (transcript `rebound_rust/logs/win11_cref_reboundx_tides_20260909.log`):

| run | tmax | C wall | Rust wall | dump |
|---|---|---|---|---|
| pseudo (long) | 628.3185307179586 | 2.46 s | 0.86 s | 8 lines / 471 bytes |
| kozai (long) | 100000.0 | 1.38 s | 1.61 s | 10 / 590 |
| migration (long) | 628.3185307179586 | 5.67 s | 1.76 s | 10 / 663 |
| pseudo (short) | 62.83185307179586 | 0.26 s | 0.10 s | 8 / 471 |
| kozai (short) | 1000.0 | 0.03 s | 0.03 s | 10 / 590 |
| migration (short) | 62.83185307179586 | 0.52 s | 0.19 s | 10 / 663 |

Every position, velocity, mass **and spin vector** matched to all 64 bits
(the short kozai dump, for instance, carries `dt 3fdfb0353c9af096` and
`Omega 1 3f55ea2802e8a6fe 4076773dd448392d 404fb27709cf85c7` on both
sides). The full kozai run is the strongest: IAS15 chooses its own step
size from the REBOUNDx forces, so matching bit for bit at t = 100,000
means both programs took the identical sequence of thousands of adaptive
steps. The long-run dumps were kept as `state_*_c_long.txt` /
`state_*_rust_long.txt`. Timings are indicative (other notebook jobs were
running on the machine); the C times are far below the 83.6 s / 163.9 s the
older port-test write-up quotes because that run had the per-step warning
printing switched on.

### 6.10 REBOUNDx binary files — C writes, Rust reads, and the reverse

Both libraries build the same configuration (forces `gr_potential` and
`central_force`; operators `modify_mass` and `drift` over three operator
steps; parameters of every supported type) and write it out:

```powershell
Set-Location "$repo\reboundx_rust\porttest"
..\target\release\examples\rebx_binary_roundtrip.exe                       # Rust writes rebx_binary_roundtrip.bin and reads it back
.\rebx_binary_roundtrip_c.exe rebx_c_reference.bin                         # C writes rebx_c_reference.bin
$a=[IO.File]::ReadAllBytes("$pwd\rebx_binary_roundtrip.bin"); $b=[IO.File]::ReadAllBytes("$pwd\rebx_c_reference.bin")
"rust=$($a.Length)  c=$($b.Length)"
$d=0; for($i=0;$i -lt $a.Length;$i++){ if($a[$i] -ne $b[$i]){ $d++ } }; "differing bytes: $d"
..\target\release\examples\rebx_binary_roundtrip.exe rebx_c_reference.bin  # Rust reads the C file, re-serializes to rebx_binary_roundtrip2.bin
.\rebx_binary_read_c.exe rebx_binary_roundtrip.bin 2>$null | Out-File -Encoding ascii readback_from_rust.txt
.\rebx_binary_read_c.exe rebx_c_reference.bin      2>$null | Out-File -Encoding ascii readback_from_c.txt
Compare-Object (Get-Content readback_from_rust.txt) (Get-Content readback_from_c.txt)
```

(Git Bash: `cmp -l rebx_binary_roundtrip.bin rebx_c_reference.bin | wc -l` counts the differing bytes; `2>/dev/null > readback_from_rust.txt`; `cmp readback_from_rust.txt readback_from_c.txt`.)

Measured (transcript `rebound_rust/logs/win11_cref_rebx_binary_20260909.log`):

1. Rust writes 6,392 bytes and reads them back: `25 / 25 checks passed`,
   `ROUND TRIP PASSED` (107 registered parameters, both forces, both
   operators, every value and order).
2. C writes 6,392 bytes.
3. **Differing bytes: 26; identical: 6,366.** The 26 are offsets 37–62 —
   the git-hash header field: C writes the ASCII text
   `notavailable00000000000000` (no git information at compile time), Rust
   writes 26 zero bytes. No simulation data is involved.
4. Rust reads the C file: `reading back rebx_c_reference.bin instead
   (foreign binary)`, **`25 / 25 checks passed`**, `re-serializing the
   reloaded state is byte-identical (6392 bytes)`; the re-serialization
   `rebx_binary_roundtrip2.bin` equals Rust's own file byte for byte (both
   SHA-256 `E3E84D8A973D78E8986B9958F3A732AB820EA6D042D1F3DE2B7429A83D16A4C5`)
   and differs from the C file (`F2B6C521A9BAE402CB5CC64CF6DAD22C04FEAF3D43FFC08FF78C0BA36B2551C6`)
   in the same 26 header bytes only.
5. C reads both files and dumps everything it recovers:
   `readback_from_rust.txt` versus `readback_from_c.txt` **28/28 lines,
   0 differing**, SHA `1C94D1CE8964217E7AE4DFF34BB0E9E4691F10B0A8D595682C04AA2AC92FB59C`
   on both — including `gr.c 40c3a8a8f5f13410`, `p1.tau_mass 4636b0a8e891ffff`,
   the negative integer `p1.primary -12345`, the three `p1.Omega`
   components, `p1.force gr_potential` and the parameter order.

(The macOS lineage page quotes 10,784-byte files for this test; the 6,392
bytes above are what both the MSVC-compiled C and the Rust wrote on this
machine, and they agree with each other outside the 26-byte hash field.)

### 6.11 What did not come out literally as the guide words it

Nothing failed and nothing was left unreproduced; these are the gates
log's own §7 items, all cosmetic:

1. **No REBOUNDx commit id** can be reported (the reference tree is an
   unpacked archive; version 5.1.0 is confirmed from three files).
2. **`derivatives_c.txt` has CRLF** line ends on Windows (text-mode
   `fopen`); content identical after CRLF→LF; the notebooks' `compare()`
   normalises CRLF.
3. **Exponent style** in the two stdout-only probes (`e+00` in C, `e0` in
   Rust); every raw-bit field matches.
4. **`libm_diff.ipynb`'s "functions that differ" line** tallies differing
   *lines* keyed by their first token (the `sin` bit pattern), not
   functions — it prints a 60-entry dictionary; the per-function
   measurement (only `pow`, 60 times) is §6.1. Not changed: the session
   was measurement only.
5. **`ref_c_ias15.txt` / `ref_rust_ias15.txt`** in the repository hold the
   *archive* ias15 write dump (`t 403c0717ebf55e62`, 368 bytes, SHA
   `E0B628A0…`), not the `integrators_test.exe` default-run dump the
   guide's §19 step 6 appears to describe; this run reproduced that
   committed content bit-exactly and left the files holding it; the
   default-run comparison is §6.4.
6. **`.bin` archive sizes vary** between runs (wall-clock fields in the
   blobs); restored states do not.
7. **`-NoProfile`** was added to the documented sweep invocation
   (transcript hygiene only).
8. **Timings** were taken with unrelated notebook jobs running; treat them
   as indicative (they match the guide within noise).
9. **`cbrt`** is named in the guide's prose but is not a column of the
   harness, so it was not measured.

## 7. The notebooks — every example has one

Seventeen notebooks in `rebound_rust/notebooks/`, one per example of both
crates (13 REBOUND + 4 REBOUNDx, including `tides_spin_pseudo.ipynb`,
new in this repository). Each builds its example with cargo, runs it, and
decodes or compares the raw-bit dumps. Install the three notebook
packages once (the measured versions: nbformat 5.10.4, nbclient 0.10.2,
ipykernel 7.1.0; one notebook also plots with matplotlib), then regenerate
and execute:

```powershell
python -m pip install --user nbformat nbclient ipykernel matplotlib
Set-Location rebound_rust\notebooks
python make_notebooks.py                                   # regenerates all 17 (ends "17 notebooks written to <this folder>")
$env:PYTHONIOENCODING = "utf-8"
python run_notebooks.py 2>&1 | Tee-Object run_all.log      # executes all 17 in fresh kernels, writes outputs back in place
Set-Location ..\..
```

(Git Bash: `cd rebound_rust/notebooks`, `PYTHONIOENCODING=utf-8 python run_notebooks.py 2>&1 | tee run_all.log`, `cd ../..`
— this is the exact form the record was made with.) `PYTHONIOENCODING`
matters on Windows: Python's default text encoding here is cp1252; the
runner's own strings are ASCII, but on a failure it prints the nbclient
error message (`msg[:110]`), which relays notebook output and tracebacks
that can carry non-ASCII characters.

Measured twice today: first `rebound_rust/logs/win11_notebooks_20260909.log`
(before the C dumps existed), then `rebound_rust/notebooks/run_all.log`
(16:53:30–16:54:17, after them) — both end
**`All 17 notebooks executed with no errors.`** The second run's per-notebook
lines: addfmt_test 1.9 s, archive_test 1.5 s, bs_pow_diff 1.7 s,
derivatives_test 1.6 s, frequency_test 1.5 s, integrators_test 4.1 s,
kepler_rectilinear 1.5 s, libm_diff 2.0 s, movetocom_var 4.1 s,
movetocom_var_test 4.1 s, rebx_binary_roundtrip 1.7 s, server_test 7.7 s,
shearing_sheet 3.9 s, shearing_sheet_test 3.3 s, tides_spin_kozai 1.8 s,
tides_spin_migration 1.7 s, tides_spin_pseudo 1.7 s — every one `ok`. The
raw log also carries a zmq `RuntimeWarning: Proactor event loop…` line
before the first notebook and one libzmq `Assertion failed: Connection
reset by peer [10054]` line while the `tides_spin_migration` kernel shut
down; both are kernel-transport noise, and no cell in any notebook has an
error output.

Final cells of the comparing notebooks, as executed here:

- `libm_diff.ipynb`: `compared 200000 samples` / `functions that differ: {…60 entries…}` (see §6.11 item 4);
- `bs_pow_diff.ipynb`: `samples: 200008   mismatches: 56  (0.0280%)` / `ULP distribution: {1: 56}` (200,008 because it counts the 8 `P` lines as samples);
- `integrators_test.ipynb` (after `integrators_test.exe whfast 2 500`): the Rust dump `t 4013ffffffffffa1`, `dt 3f847ae147ae147b`, `steps_done 500` — the C comparison for whfast is the matrix row of §6.3;
- `shearing_sheet_test.ipynb`: `Rust SHA-256: 75BDAAB7…` / `C    SHA-256: 75BDAAB7…` / **`BIT-IDENTICAL: C and Rust agree on every bit`**;
- `movetocom_var.ipynb`, `tides_spin_pseudo.ipynb`, `tides_spin_kozai.ipynb`, `tides_spin_migration.ipynb`: each ends **`BIT-IDENTICAL: C and Rust agree on every bit`**;
- `server_test.ipynb`: `/simulation returned 3448 bytes`, `header: b'REBOUND Binary File. Version: 5.'`, `server exited cleanly`;
- `archive_test.ipynb`: `archive_rust_ias15.bin: 15865 bytes`, `archive_rust_whfast-usafe.bin: 5174 bytes` plus the whfast-usafe dump;
- `frequency_test.ipynb`: decoded frequencies `[0.3, 0.55, 0.11]` for FMFT/FMFT2 and `[0.299996, 0.550001, 0.11]` for MFT;
- `derivatives_test.ipynb` and `addfmt_test.ipynb` end with an empty last cell by design (their `head` cell is guarded by `OUTFILE = None`).

To read them in your browser:

```powershell
python -m pip install --user jupyterlab
python -m jupyterlab rebound_rust\notebooks\    # (`jupyter lab …` only if the launcher of a --user install is on your PATH)
```

(Git Bash: `jupyter lab rebound_rust/notebooks/`.)

## 8. The encyclopedia's rebound copies, and its three libm editorial notes

`SolveIt_Notebooks_for_rust/` (the notebook encyclopedia) carries an
executed copy of each of the 17 rebound notebooks under
`07_nbody_rebound_rust/<name>/<name>.ipynb`, each with a PowerShell header
cell prepended. The executor is family-aware: it runs the rebound copies
through nbclient with the kernel's working directory set to
`rebound_rust/notebooks`, so their relative paths resolve. To execute the
17 rebound copies and log the verdicts where the encyclopedia builder
reads them:

```powershell
$copies = (Get-ChildItem SolveIt_Notebooks_for_rust\07_nbody_rebound_rust -Recurse -Filter *.ipynb |
           Where-Object FullName -notmatch 'ipynb_checkpoints').FullName
python SolveIt_Notebooks_for_rust\_tools\run_copy.py $copies 2>&1 | Tee-Object SolveIt_Notebooks_for_rust\_tools\exec_rebound.log
```

(Git Bash: `python SolveIt_Notebooks_for_rust/_tools/run_copy.py $(find SolveIt_Notebooks_for_rust/07_nbody_rebound_rust -name '*.ipynb' -not -path '*ipynb_checkpoints*') 2>&1 | tee SolveIt_Notebooks_for_rust/_tools/exec_rebound.log`.)

Measured on this machine: **17/17 ok**
(`SolveIt_Notebooks_for_rust/_tools/exec_rebound.log` — one
`ok   ...\<name>.ipynb  (N cells)` verdict line per copy, N = 2 to 4 code
cells, and `1 ok, 0 failed` after each). The encyclopedia's screenshot
pass takes no image for them by design (the rebound entries have textual
verdicts, no browser GUI). The complete encyclopedia pipeline — copies,
players, screenshots, the builder and its six verify gates — with its own
measured results is in `SolveIt_Notebooks_for_rust/README.md`.

Three of the seventeen notebooks describe, in their introductory prose, a
libm measurement that is platform-specific — and the executed run
embedded in the encyclopedia's copy is *this repository's own Windows
run*, which on the Mac would have come out differently. The encyclopedia
builder (`_tools/encyclo.py`, `EDITORIAL_NOTES`) therefore prints one
italic note under each of those three entries. Verbatim:

- **`libm_diff`** — *Encyclopedia note: the executed run embedded in THIS
  copy is this repository's own Windows 11 / MSVC (UCRT libm) run; its
  final cell lists exactly which functions differ from the C reference on
  this platform (the macOS edition of this encyclopedia recorded
  'functions that differ: NONE' with Apple libm).*
- **`bs_pow_diff`** — *Encyclopedia note: the executed run embedded in THIS
  copy is this repository's own Windows 11 / MSVC (UCRT) measurement of
  the pow difference; its final cell is the ULP distribution of record
  here (the macOS edition recorded 'ULP distribution: none').*
- **`integrators_test`** — *Encyclopedia note: the bit-identical-vs-MSVC
  claim in the intro is exactly what THIS copy's embedded Windows 11 run
  verifies; the verdict cell at the end of the copy is this machine's own.*

How they relate to the gates above: the first two notes point at the
Windows `pow` difference of §6.1 and §6.2 (60/200,000 and 56/200,000,
all 1 ULP, versus "NONE"/"none" on the Mac, where both languages call the
same Apple libm), and the third at the 63/63 matrix of §6.3, whose
whfast row is what that notebook's embedded run shows. The notes exist so
that a reader of the encyclopedia is never left believing the macOS
sentence in the notebook's own intro is what the copy in front of them
measured.

## 9. Run any example directly

```powershell
Set-Location rebound_rust
cargo run --release --example integrators_test -- whfast 2 500
cargo run --release --example shearing_sheet_test -- 400
Set-Location ..\reboundx_rust
cargo run --release --example tides_spin_pseudo -- 62.83185307179586
Set-Location ..
```

(Git Bash: `cd rebound_rust`, `cd ../reboundx_rust`, `cd ..`.) 13 REBOUND
examples, 4 REBOUNDx examples; each has the same name as its notebook.
The examples that compare against C look for the C dumps in `porttest\`
and say so when they are absent (`C reference dump not present; run the
C harness ...`); build them with section 5 if you want the comparison.

## 10. Credit and licensing

REBOUND is © Hanno Rein and collaborators; REBOUNDx is © Dan Tamayo,
Hanno Rein and collaborators; both GPL-3.0-or-later, as are these
translations. **All of the science belongs to the original authors** —
cite **Rein & Liu 2012 (A&A 537, A128)** for any REBOUND use and
**Tamayo, Rein, Shi & Hernandez 2019 (MNRAS 491, 2885)** for any
REBOUNDx use, plus the per-feature papers listed in
`rebound_rust/README.md`. Nothing from these ports has been or will be
submitted to the upstream projects (their contribution policy asks that
no AI-generated code be submitted, and this port honours it); report
problems with the port here, not upstream.
