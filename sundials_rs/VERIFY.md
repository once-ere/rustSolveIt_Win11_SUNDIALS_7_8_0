# VERIFY.md — how to check every claim in this repository yourself

This document assumes you know nothing about this project. It tells you what
was done, what evidence was recorded, where that evidence is, and what to type
to confirm it. Nothing here asks you to take a summary on trust: every number
quoted in `c-results/`, `rust-results/` and `differences/` traces to a file you
can open or a command you can re-run.

If you only do one thing, do **Check 1** — it takes a minute and confirms the
files you have are the files that were measured.

---

## 0. What the evidence consists of

| directory | what it holds |
|---|---|
| `examples/` | the SUNDIALS 7.8.0 C example sources and their expected outputs, copied from the upstream release |
| `c-results/provenance/` | how the C was built: environment, tool versions, every command line, every compiler invocation, checksums |
| `rust-results/provenance/` | the same for the Rust build |
| `differences/provenance/` | checksums of the reference outputs, and proof the sources in this repository are the ones that were compiled |
| `c-results/outputs/`, `rust-results/outputs/` | the raw text each program printed, one file per run |
| `differences/diffs/` | a line-by-line diff for every case where the two disagreed |

"Provenance" here means: for each result, the recorded origin and chain of
custody — which exact input file, transformed by which exact command with which
exact flags, using which exact tool version, producing which exact bytes.

---

## 1. Tools you need, and what you can check without them

| check | needs |
|---|---|
| checksums (Check 1, 2, 5) | nothing beyond PowerShell, which ships with Windows |
| reading the recorded command lines (Check 3) | a text editor |
| re-running the programs (Check 6) | the built binaries, or a rebuild |
| rebuilding the C (Check 7) | Visual Studio 18 Professional, CMake, Ninja |
| rebuilding the Rust (Check 8) | a Rust toolchain (`rustup`) |

Checks 1–5 need no compiler at all.

---

## Check 1 — the files you have are the files that were measured

Every output file was hashed with SHA-256 when it was produced. To confirm your
copy is unaltered, in **PowerShell**, from the top of this repository:

```
Get-Content c-results\provenance\22-outputs.sha256 | ForEach-Object {
  $h,$f = $_ -split '  ',2
  $a = (Get-FileHash "c-results\outputs\$f" -Algorithm SHA256).Hash.ToLower()
  if ($a -ne $h) { "MISMATCH $f" }
}
"checked $((Get-Content c-results\provenance\22-outputs.sha256).Count) files"
```

**Expected:** no `MISMATCH` lines, and `checked 189 files`.

Repeat with `rust-results` in place of `c-results` (179 files there — the
Rust port has no counterpart for the 10 OpenMP/ManyVector examples, see
"What the evidence shows").

On a machine with `sha256sum` (Git Bash, WSL, Linux, macOS) the same check is:

```
cd c-results/outputs && sha256sum -c ../provenance/22-outputs.sha256 | grep -v ': OK$'
```

**Expected:** no output at all.

---

## Check 2 — what was compiled from where

The 180 C example files were compiled **directly from this
repository's own `examples/` directory**. Confirm it from the recorded
command lines:

```
python -c "import json,os;d=json.load(open('c-results/provenance/30-build-each-example.json'));m=os.path.join(os.getcwd(),'examples');print(sum(1 for r in d if m in r['command']),'of',len(d))"
```

**Expected:** `180 of 180` — every compile command names a
path inside this repository.

The SUNDIALS *library* is the one thing that could not come from here: only
the examples were copied in, so `src/` and `include/` are absent and the
library was built from the upstream 7.8.0 release tree. That is recorded in
`c-results/provenance/01-configure-cmd.txt` and `03-CMakeCache.txt`.
`differences/provenance/21-source-of-each-artefact.txt` states the split.

> An earlier revision of this document described a check that compared this
> repository's `examples/` against the upstream tree and reported "370
> identical". That check was circular — the C was being compiled from the
> upstream tree, so confirming a copy matches its original established
> nothing about the build. It has been removed and replaced by the check
> above.

## Check 3 — what the compiler was actually told to do

This is the part that was missing in the first version of these documents, and
it is the part that matters most.

**The C — the examples.** `c-results/provenance/30-build-each-example.json`
has one entry per example source: the literal `cl.exe` command line, the exit
code, and the compiler's full output. This is the record that matters, because
it covers all 180 files including the ones that failed. Pull any file's line
out yourself:

```
python -c "import json;d=json.load(open('c-results/provenance/30-build-each-example.json'));print(next(r['command'] for r in d if r['name']=='cvRoberts_dns'))"
```

The flags every example was compiled with:

```
/nologo /O2 /Ob2 /DNDEBUG /MD /DWIN32 /D_WINDOWS
/DSUNDIALS_STATIC_DEFINE /D_CRT_SECURE_NO_WARNINGS
/I<example's own dir> /I<sundials>\include /I<build>\include
/link /LIBPATH:<build>\bin <one solver lib> <support libs>
```

with `/openmp` added for the `C_openmp` and `C_openmpdev` directories.
`31-build-each-example.txt` is the same record in readable form — command,
then verbatim compiler output — for every one of the 180 files.

**The C — the library.** `c-results/provenance/04-compile_commands.json` is
CMake's record for the 235 library translation units, in the same format.

`c-results/provenance/06-build-out.txt` is the build log with Ninja in verbose
mode, so it also contains every **link** command as executed, not just the
compiles. `c-results/provenance/01-configure-cmd.txt` holds the literal CMake
configure line, and `03-CMakeCache.txt` holds every option CMake resolved —
including the ones that were left at their defaults, which the configure line
does not show.

**The Rust.** `rust-results/provenance/02-build-out.txt` is `cargo build -v`
output, which prints every `rustc` invocation. The library was compiled with:

```
rustc --crate-name sundials_core --edition=2021 crates\sundials_core\src\lib.rs
  --crate-type lib --emit=dep-info,metadata,link
  -C opt-level=3 -C embed-bitcode=no -C strip=debuginfo
  -C target-feature=+fma
```

`-C target-feature=+fma` comes from `.cargo/config.toml`, reproduced verbatim
in `rust-results/provenance/03-cargo-config.txt`. Count how many compilations
carried it:

```
Select-String -Path rust-results\provenance\02-build-out.txt -Pattern 'target-feature=\+fma' | Measure-Object | Select-Object Count
```

**Expected:** 115.

**The four `*L` examples** are a special case, because they call LAPACK and no
LAPACK is installed. `c-results/provenance/10-lapacksub-cmd.txt` shows, for
each of them, *every line that differs* between the upstream source and the
source that was compiled — four lines each — plus the exact `cl.exe` line used.
Read it if you want to satisfy yourself that nothing else was changed.

---

## Check 4 — which tools produced this

```
Get-Content c-results\provenance\00-environment.txt
Get-Content rust-results\provenance\00-environment.txt
```

These record the operating system, the CPU, the UTC start and finish times, the
full path and version banner of `cl.exe` and `link.exe`, the CMake and Ninja
versions, the MSVC toolset and Windows SDK versions selected by `vcvars64.bat`,
and the complete `INCLUDE` and `LIB` search paths. The measured build used:

| | |
|---|---|
| compiler | `cl.exe` 19.51.36246 (MSVC toolset 14.51.36231), x64 |
| Windows SDK / UCRT | 10.0.28000.0 |
| CMake / Ninja | 4.1.2 / 1.13.2 |
| Rust | rustc/cargo 1.91.1, target `x86_64-pc-windows-msvc` |
| host | Windows 11 Pro for Workstations 10.0.26200.8655, Intel Core Ultra 9 275HX |

---

## Check 5 — the reference outputs are unmodified

The comparison judges both implementations against the `.out` files shipped in
the SUNDIALS 7.8.0 release. Their checksums are in
`differences/provenance/20-references.sha256` (199 files). Verify them the same
way as Check 1, against `examples/`.

---

## Check 6 — re-run the programs and get the same answers

The binaries are not committed (they are build products), but if you have built
them, or after doing Check 7 and Check 8:

```
python tools\example_matrix.py --all
python tools\example_report.py
```

This runs all 199 (example, argv) pairs on both sides, each in its own scratch
directory, and rewrites the tables. Then repeat Check 1: the checksums should
be unchanged.

This was done twice from independent builds, including a full `cargo clean`
rebuild, and all 358 output files were bit-for-bit identical both times.

---

## Check 7 — rebuild the C from scratch

Needs Visual Studio 18 Professional, CMake and Ninja.

```
tools\build_c_examples.cmd
tools\build_c_lapack_substituted.cmd
```

The first script deletes and recreates `logs\c-build`, then rewrites every file
in `c-results\provenance\`. Compare your regenerated
`04-compile_commands.json` against the committed one to confirm the flags match.

Then compile every example individually and record each command:

```
python tools\build_all_c_examples.py
tools\build_c_lapack_substituted.cmd
```

Expected: **114 of 180** C files build, and `logs\c-build\bin`
holds 118 `.exe` files (114 plus the four `*L` built by the second
script). The 66 that do not build each have the compiler's own
error recorded in `c-results/provenance/31-build-each-example.txt`; grouped by
cause they are 34 missing `mpi.h`, 12 missing `klu.h`, 10 missing
`slu_mt_ddefs.h`, 4 rejected `#pragma omp target`, 4 LAPACK, 2 missing
`HYPRE.h`.

---

## Check 8 — rebuild the Rust from scratch

Needs a Rust toolchain.

```
bash tools/build_rust_examples.sh
```

This runs `cargo clean` then `cargo build --release --workspace --examples -v`,
rewriting `rust-results/provenance/`. Expected: exit code 0, 0 warnings,
0 errors — the last lines of `rust-results/provenance/00-environment.txt`
report all three.

---

## What the evidence shows

Stated here so you know what you are checking. The reasoning is in
[`differences/ANALYSIS.md`](differences/ANALYSIS.md).

**Coverage.** All 180 C example files in `examples/` were compiled
individually; 114 built. All 258 `(example, argv)` variants
declared by the `CMakeLists.txt` of all 29 example directories were run on
both sides.

| | variants |
|---|---:|
| ran on **both** sides — the comparable set | **179** |
| ran on the C side only — no Rust counterpart exists | **10** |
| excluded on both sides (KLU / SuperLU_MT not installed) | 20 |
| neither side could run (MPI / PETSc / hypre / OpenMP-offload absent) | 49 |

The 10 C-only variants are the 9 `C_openmp` examples and
`ark_brusselator1D_manyvec`. They are **not ported**: the Rust port has no
`nvector_openmp` and no `nvector_manyvector`. That is a real gap, not an
exclusion, and it is listed example by example in
[`rust-results/EXCLUSIONS.md`](rust-results/EXCLUSIONS.md).

**Comparison**, over the 179 variants both sides ran:

| | count |
|---|---:|
| C and Rust print the same thing | 131 |
| they differ | 48 |
| …of those, Rust matches the SUNDIALS reference and the C does not | 41 |
| …of those, the C matches the reference and Rust does not | **0** |
| …of those, neither matches (the reference file is stale) | 7 |

Against the reference outputs shipped with SUNDIALS 7.8.0, the Rust port is
byte-identical on **153** and the MSVC C build on
**112**.

The reason is recorded and measurable: the reference files were generated on a
Linux/glibc machine, and glibc's and Microsoft's maths libraries do not round
identically. A C program built here uses Microsoft's. The Rust port uses
neither — it carries its own implementations of `sin`, `cos`, `exp`, `log` and
the rest, checked bit-for-bit against glibc over 8,000,000 inputs per function
(`current_status.md` §2). So on this platform the Rust reproduces the
published results and a native C build cannot.

## Known limits of this evidence

* 62 of the 180 C examples did not build **in this build**. Each one's
  compiler error is recorded in
  `c-results/provenance/31-build-each-example.txt` — these are measured
  failures. But note the build did not use everything this machine has:
  Intel oneAPI is installed and supplies **oneMKL** (LAPACK), **Intel MPI**
  (`mpi.h`, `mpiexec`) and **`ifx`** (Fortran). An earlier revision of this
  document wrongly listed all three as absent. Rebuilding with them enabled
  would let the 34 `mpi.h` failures and the 4 LAPACK ones be attempted
  properly; that has not been done yet. PETSc, hypre, KLU/SuiteSparse and
  SuperLU_MT are genuinely absent.
* 10 examples build and run on the C side but have **no Rust counterpart**,
  because the port has no `nvector_openmp` or `nvector_manyvector`. Those two
  vector implementations, and then the 10 example translations, are what
  "port every example" still requires.
* The C++ (46), Fortran (51) and CUDA (7) sources are not covered: they are
  not C. Note that a CUDA toolkit (v13.0) *is* installed on this machine — an
  earlier revision of these documents wrongly claimed it was not — but the
  Rust port has no GPU backend, so there would be nothing to compare against.
* Comparisons strip carriage returns from both sides before comparing, because
  the MSVC C build writes CRLF line endings and the Rust port writes LF. That
  is a platform convention, not a numerical result — but it means "identical"
  in these tables means "identical after that one normalisation". The raw
  bytes are committed unmodified so you can confirm this yourself.
* Timing lines (`Total run time`, `CPU time`, `wall clock`) are removed from
  both sides before comparing, because they differ on every run of any program.
* The binaries themselves are not committed. Checksums for them are, in
  `*/provenance/21-binaries.sha256`, so a rebuild can be compared — though note
  that MSVC and rustc do not guarantee byte-identical binaries across rebuilds,
  so a hash mismatch there is not by itself evidence of a problem. The output
  checksums in Check 1 are the meaningful ones.
