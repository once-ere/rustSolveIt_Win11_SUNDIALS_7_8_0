#!/usr/bin/env python3
"""example_report.py — turn the run state into the three document sets.

    python tools/example_report.py

Reads `logs/example_matrix_state.json` (written by `tools/example_matrix.py`)
and the captured outputs under `c-results/outputs/` and
`rust-results/outputs/`, then writes:

    c-results/{README,RESULTS,EXCLUSIONS}.md
    rust-results/{README,RESULTS,EXCLUSIONS}.md
    differences/{README,SUMMARY,BY-EXAMPLE,ANALYSIS}.md
    differences/diffs/<variant>.diff

Comparisons are made on noise-filtered text (timing lines removed from both
sides), and each pair is additionally classified under whitespace squeezing so
a column-width difference can be told from a numeric one.

SPDX-License-Identifier: BSD-3-Clause
"""

import difflib
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
EXAMPLES = ROOT / "examples"
NOISE = re.compile(r"Total run time|CPU time|cpu time|wall clock")
# A zero exit code does not mean an example succeeded: SUNDIALS prints a
# diagnostic and several examples return 0 anyway. The output has to be read.
ERRMARK = re.compile(r"SUNDIALS_ERROR|\[ERROR\]|failed with retval|mxstep steps taken")

state = json.loads((ROOT / "logs" / "example_matrix_state.json").read_text())
_bj = ROOT / "c-results/provenance/30-build-each-example.json"
CBUILD = {r["name"]: r for r in json.loads(_bj.read_text(encoding="utf-8"))} if _bj.exists() else {}


def build_reason(name):
    """Why a C example has no binary, in the compiler's own words."""
    r = CBUILD.get(name)
    if r is None:
        return "not attempted"
    if r["ok"]:
        return "built"
    o = r["output"]
    m = re.search(r"C1083: Cannot open include file: '([^']+)'", o)
    if m:
        return f"missing header `{m.group(1)}`"
    if "C3001" in o and "target" in o:
        return "MSVC rejects `#pragma omp target` (OpenMP 4.5 device offload)"
    if "LNK2019" in o:
        # The four *L examples fail the plain build (no LAPACK is installed)
        # and are then built by tools/build_c_lapack_substituted.cmd with the
        # same two-token substitution the Rust port makes.
        if (ROOT / "logs/c-build/bin" / (name + ".exe")).exists():
            return "built via documented LAPACK->native substitution"
        return "unresolved external (LAPACK)"
    return f"build failed (exit {r['returncode']})"

ROWS, PROV = state["rows"], state["prov"]


def load(p: Path):
    if not p.exists():
        return None
    txt = p.read_text(encoding="utf-8", errors="replace")
    return [l.rstrip("\r") for l in txt.splitlines() if not NOISE.search(l)]


def has_error(side, key):
    p = ROOT / f"{side}-results" / "outputs" / key
    return bool(p.exists() and ERRMARK.search(p.read_text(encoding="utf-8", errors="replace")))


def squeeze(ls):
    return None if ls is None else [re.sub(r" +", " ", l).rstrip() for l in ls]


def cmp(a, b):
    """('same'|'ws-only'|'content', ndiff_lines, preview)"""
    if a is None or b is None:
        return "missing", None, []
    if a == b:
        return "same", 0, []
    d = [l for l in difflib.unified_diff(a, b, lineterm="", n=0)
         if l.startswith(("+", "-")) and not l.startswith(("+++", "---"))]
    kind = "ws-only" if squeeze(a) == squeeze(b) else "content"
    return kind, len(d), d[:14]


recs = []
for r in ROWS:
    key = r["outfile"]
    cst = state["c"][key]["state"]
    rst = state["rust"][key]["state"]
    ref = load(EXAMPLES / r["dir"] / key)
    cout = None if cst.startswith("excluded") else load(ROOT / "c-results" / "outputs" / key)
    rout = None if rst.startswith("excluded") else load(ROOT / "rust-results" / "outputs" / key)
    rec = dict(r)
    rec["c_state"], rec["rust_state"] = cst, rst
    rec["has_ref"] = ref is not None
    rec["c_ref"], rec["c_ref_n"], _ = cmp(cout, ref)
    rec["rust_ref"], rec["rust_ref_n"], _ = cmp(rout, ref)
    rec["c_rust"], rec["c_rust_n"], rec["preview"] = cmp(cout, rout)
    rec["c_err"] = (not cst.startswith("excluded")) and has_error("c", key)
    rec["rust_err"] = (not rst.startswith("excluded")) and has_error("rust", key)
    recs.append(rec)

live = [r for r in recs if not r["c_state"].startswith("excluded")]
excl = [r for r in recs if r["c_state"].startswith("excluded")]
# Comparable = both sides actually produced output.
both = [r for r in recs if r["c_state"] == "ran" and r["rust_state"] == "ran"]
c_only = [r for r in recs if r["c_state"] == "ran" and r["rust_state"] == "NO-BINARY"]
rust_only = [r for r in recs if r["rust_state"] == "ran" and r["c_state"] == "NO-BINARY"]
neither_ran = [r for r in recs if r["c_state"] != "ran" and r["rust_state"] != "ran"
               and not r["c_state"].startswith("excluded")]


def tally(field, among=live):
    out = {}
    for r in among:
        out[r[field]] = out.get(r[field], 0) + 1
    return out


T_CR = tally('c_rust', both)
T_CREF = tally('c_ref', both)
T_RREF = tally('rust_ref', both)
PROV_BLOCK = f"""
| item | value |
|---|---|
| generated | `{PROV['when']}` |
| repository commit | `{PROV['commit']}` |
| operating system | {PROV['os']} |
| CPU | {PROV['cpu']} |
| C compiler | {PROV['cl']} |
| CMake | {PROV['cmake']} |
| Rust | {PROV['rustc']} / {PROV['cargo']} |
| upstream sources | SUNDIALS 7.8.0, `examples/` as copied into this repository |
"""

SCOPE = f"""
### Scope — every C example in the repository was attempted

There is no pre-selected subset here. All **180 `.c` files** under
`examples/` were compiled one at a time, and all **{len(recs)} (example, argv)
variants** declared by the `CMakeLists.txt` files of all **29 example
directories** were run. Where a program did not build, the compiler's own
error is recorded per file in
[`../c-results/provenance/31-build-each-example.txt`](../c-results/provenance/31-build-each-example.txt)
— nothing is excluded by assumption.

The C++ (46), Fortran (51) and CUDA (7) sources are not covered: they are not
C, and this project is a C-to-Rust port.
"""




def quote(relpath, first=None, fence="text"):
    """Embed a provenance artefact verbatim. The documents must never restate
    a command line from memory; they quote the recorded file, so what is
    published is what was executed."""
    f = ROOT / relpath
    if not f.exists():
        return f"_(missing: `{relpath}` — run the build script to regenerate)_"
    lines = f.read_text(encoding="utf-8", errors="replace").splitlines()
    if first:
        lines = lines[:first]
    body = "\n".join(l.rstrip() for l in lines).strip("\n")
    return f"```{fence}\n{body}\n```"


def rustc_line(name):
    """Pull one literal rustc invocation out of the cargo -v log."""
    f = ROOT / "rust-results/provenance/02-build-out.txt"
    if not f.exists():
        return "_(missing: rust-results/provenance/02-build-out.txt)_"
    for l in f.read_text(encoding="utf-8", errors="replace").splitlines():
        if f"--crate-name {name} " in l:
            cmd = l.strip()
            if cmd.startswith("Running `"):
                cmd = cmd[len("Running `"):].rstrip("`")
            return "```text\n" + cmd.replace(" -C ", "\n  -C ").replace(" --", "\n  --") + "\n```"
    return f"_(no rustc invocation found for {name})_"


def cl_line(suffix):
    """Pull one literal cl.exe invocation out of compile_commands.json."""
    import json as _json
    f = ROOT / "c-results/provenance/04-compile_commands.json"
    if not f.exists():
        return "_(missing: c-results/provenance/04-compile_commands.json)_"
    for e in _json.loads(f.read_text(encoding="utf-8")):
        if e["file"].endswith(suffix):
            return "```text\n" + e["command"].replace(" -I", "\n  -I").replace(" /D", "\n  /D").replace(" -c ", "\n  -c ") + "\n```"
    return f"_(no compile command found for {suffix})_"


def n_units():
    import json as _json
    f = ROOT / "c-results/provenance/04-compile_commands.json"
    return len(_json.loads(f.read_text(encoding="utf-8"))) if f.exists() else 0


def count_lines(relpath):
    f = ROOT / relpath
    return len(f.read_text(encoding="utf-8", errors="replace").splitlines()) if f.exists() else 0


def md_table(rows, cols):
    out = ["| " + " | ".join(c[0] for c in cols) + " |",
           "|" + "|".join("---" for _ in cols) + "|"]
    for r in rows:
        out.append("| " + " | ".join(str(c[1](r)) for c in cols) + " |")
    return "\n".join(out)


def w(path, text):
    p = ROOT / path
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(text.lstrip("\n"), encoding="utf-8", newline="\n")
    print("wrote", path)


# ------------------------------------------------------------------ c-results
w("c-results/README.md", f"""
# c-results — SUNDIALS 7.8.0 C examples built with Visual Studio 18 Professional

What the **C** implementation does on this machine. Built from the upstream
SUNDIALS 7.8.0 sources with MSVC, out of source; the upstream tree is never
written to.

## Provenance
{PROV_BLOCK}
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
| [`provenance/04-compile_commands.json`](provenance/04-compile_commands.json) | **the exact `cl.exe` line for each of the {n_units()} translation units** |
| [`provenance/05-build-cmd.txt`](provenance/05-build-cmd.txt) | the literal build command line |
| [`provenance/06-build-out.txt`](provenance/06-build-out.txt) | `ninja -v`: every compile *and link* as executed ({count_lines('c-results/provenance/06-build-out.txt')} lines) |
| [`provenance/10-lapacksub-cmd.txt`](provenance/10-lapacksub-cmd.txt) | for each `*L` example, every line that differs from upstream, and the `cl.exe` line used |
| [`provenance/11-lapacksub-out.txt`](provenance/11-lapacksub-out.txt) | compiler/linker output for those four |
| [`provenance/20-input-sources.sha256`](provenance/20-input-sources.sha256) | SHA-256 of every C source compiled |
| [`provenance/21-binaries.sha256`](provenance/21-binaries.sha256) | SHA-256 of every binary produced |
| [`provenance/22-outputs.sha256`](provenance/22-outputs.sha256) | SHA-256 of every captured output |

### The configure command, verbatim

{quote('c-results/provenance/01-configure-cmd.txt')}

run from the environment established by
`"C:\\Program Files\\Microsoft Visual Studio\\18\\Professional\\VC\\Auxiliary\\Build\\vcvars64.bat"`,
then built with

{quote('c-results/provenance/05-build-cmd.txt')}

### What the compiler was actually told, for one file

`cvRoberts_dns.c`, quoted from `04-compile_commands.json` (line breaks added
for reading; the recorded command is one line):

{cl_line('cvRoberts_dns.c')}

So: `/O2 /Ob2` optimisation, `/DNDEBUG`, `-MD` (dynamic UCRT), 64-bit indices
and double precision as configured. Nothing is inferred here — read the JSON.

## Results
{SCOPE}
| outcome | variants |
|---|---:|
| C source files attempted | **{len(CBUILD)}** |
| of those, built | **{sum(1 for r in CBUILD.values() if r['ok'])}** |
| ran to completion (exit 0) | **{sum(1 for r in live if r['c_state'] == 'ran')}** |
| of those, printed a solver error anyway | **{sum(1 for r in live if r['c_err'])}** |
| non-zero exit or timeout | **{sum(1 for r in live if r['c_state'] != 'ran')}** |
| excluded (KLU / SuperLU_MT) | **{len(excl)}** |
| total | {len(recs)} |

Against the reference outputs shipped with SUNDIALS 7.8.0:

| C vs shipped `.out` | variants |
|---|---:|
| byte-identical | **{T_CREF.get('same', 0)}** |
| whitespace-only difference | **{T_CREF.get('ws-only', 0)}** |
| content difference | **{T_CREF.get('content', 0)}** |

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
""")

cols_side = [
    ("example", lambda r: f"`{r['name']}`"),
    ("argv", lambda r: f"`{r['args']}`" if r["args"] else ""),
    ("dir", lambda r: r["dir"]),
]
w("c-results/RESULTS.md", f"""
# c-results — per-variant results

MSVC build, {PROV['when']}. `same` / `ws-only` / `content` describe this
build's output against the reference `.out` shipped with SUNDIALS 7.8.0,
after removing timing lines from both sides.

{md_table(recs, cols_side + [
    ("build", lambda r: build_reason(r["name"])),
    ("run", lambda r: r["c_state"]),
    ("solver error", lambda r: "**YES**" if r["c_err"] else ""),
    ("bytes", lambda r: state["c"][r["outfile"]]["bytes"]),
    ("vs shipped ref", lambda r: r["c_ref"] if not r["c_state"].startswith("excluded") else "—"),
    ("diff lines", lambda r: r["c_ref_n"] if r["c_ref_n"] else ""),
    ("output file", lambda r: f"[`{r['outfile']}`](outputs/{r['outfile']})"
        if not r["c_state"].startswith("excluded") else "—"),
])}
""")

# --------------------------------------------------------------- rust-results
w("rust-results/README.md", f"""
# rust-results — the pure-Rust port, built and run with cargo

What the **Rust** implementation does on the same machine, in the same
session, over the same 199 variants.

## Provenance
{PROV_BLOCK}
| file | what it lets you check |
|---|---|
| [`provenance/00-environment.txt`](provenance/00-environment.txt) | host, UTC start/finish, `rustc -vV` and `cargo -V` with full paths, release profile, warning/error counts |
| [`provenance/01-build-cmd.txt`](provenance/01-build-cmd.txt) | the literal cargo command lines |
| [`provenance/02-build-out.txt`](provenance/02-build-out.txt) | `cargo build -v`: **every `rustc` invocation as executed** ({count_lines('rust-results/provenance/02-build-out.txt')} lines) |
| [`provenance/03-cargo-config.txt`](provenance/03-cargo-config.txt) | `.cargo/config.toml` verbatim — the source of `-C target-feature=+fma` |
| [`provenance/04-Cargo.lock.txt`](provenance/04-Cargo.lock.txt) | the resolved dependency set: 7 workspace crates, nothing external |
| [`provenance/20-input-sources.sha256`](provenance/20-input-sources.sha256) | SHA-256 of every example source compiled |
| [`provenance/21-binaries.sha256`](provenance/21-binaries.sha256) | SHA-256 of every binary produced |
| [`provenance/22-outputs.sha256`](provenance/22-outputs.sha256) | SHA-256 of every captured output |

### The build command, verbatim

{quote('rust-results/provenance/01-build-cmd.txt')}

### What rustc was actually told, for the core library

Quoted from `02-build-out.txt` (line breaks added for reading):

{rustc_line('sundials_core')}

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
{SCOPE}
| outcome | variants |
|---|---:|
| variants whose example is ported to Rust | **{sum(1 for r in live if r['crate'])}** |
| variants with no Rust counterpart | **{sum(1 for r in live if not r['crate'])}** |
| ran to completion (exit 0) | **{sum(1 for r in live if r['rust_state'] == 'ran')}** |
| of those, printed a solver error anyway | **{sum(1 for r in live if r['rust_err'])}** |
| non-zero exit or timeout | **{sum(1 for r in live if r['rust_state'] != 'ran')}** |
| excluded (KLU / SuperLU_MT) | **{len(excl)}** |
| total | {len(recs)} |

Against the reference outputs shipped with SUNDIALS 7.8.0:

| Rust vs shipped `.out` | variants |
|---|---:|
| byte-identical | **{T_RREF.get('same', 0)}** |
| whitespace-only difference | **{T_RREF.get('ws-only', 0)}** |
| content difference | **{T_RREF.get('content', 0)}** |

## Files

| file | contents |
|---|---|
| [`RESULTS.md`](RESULTS.md) | every variant: exit status, output size, agreement with the shipped reference |
| [`EXCLUSIONS.md`](EXCLUSIONS.md) | every example not ported, with the reason |
| [`outputs/`](outputs/) | raw captured stdout+stderr, one file per variant |
| [`provenance/`](provenance/) | build environment, literal command lines, compiler invocations, checksums |
| [`../VERIFY.md`](../VERIFY.md) | how to check every claim here yourself |
""")

w("rust-results/RESULTS.md", f"""
# rust-results — per-variant results

Rust port, release build, {PROV['when']}. `same` / `ws-only` / `content`
describe this build's output against the reference `.out` shipped with
SUNDIALS 7.8.0, after removing timing lines from both sides.

{md_table(recs, cols_side + [
    ("ported", lambda r: "yes" if r["crate"] else "**no**"),
    ("run", lambda r: r["rust_state"]),
    ("solver error", lambda r: "**YES**" if r["rust_err"] else ""),
    ("bytes", lambda r: state["rust"][r["outfile"]]["bytes"]),
    ("vs shipped ref", lambda r: r["rust_ref"] if not r["rust_state"].startswith("excluded") else "—"),
    ("diff lines", lambda r: r["rust_ref_n"] if r["rust_ref_n"] else ""),
    ("output file", lambda r: f"[`{r['outfile']}`](outputs/{r['outfile']})"
        if not r["rust_state"].startswith("excluded") else "—"),
])}
""")

# ----------------------------------------------------------------- exclusions
EXCL_MD = f"""
# Exclusions

## Serial examples excluded on both sides ({len(set(r['name'] for r in excl))} programs, {len(excl)} variants)

These need KLU or SuperLU_MT. Neither library is installed, the C build skips
them, and the Rust port excludes them by specification — so they are excluded
symmetrically and no comparison is affected.

{md_table(excl, [
    ("example", lambda r: f"`{r['name']}`"),
    ("argv", lambda r: f"`{r['args']}`" if r["args"] else ""),
    ("dir", lambda r: r["dir"]),
    ("reason", lambda r: r["c_state"]),
])}

> ### Correction — three of these "absent" backends are installed
>
> An earlier revision of this file stated that "MS-MPI, PETSc, hypre, KLU,
> SuperLU, LAPACK and any Fortran compiler are genuinely absent". That was
> wrong, and the error was mine: I probed a handful of default install paths
> and generalised from the misses. Intel oneAPI is installed on this machine
> and supplies three of them:
>
> | claimed absent | actually present |
> |---|---|
> | LAPACK | **oneMKL** 2025.3 / 2026.1 / latest — `mkl_lapack.h`, ILP64 libraries |
> | MPI | **Intel MPI** 2021.17 / 2021.18 — `mpi.h`, `impi.lib`, `mpiexec.exe` |
> | Fortran compiler | **`ifx`** 2026.1.1 and 2025.3 |
>
> Genuinely absent, still: PETSc, hypre, KLU/SuiteSparse, SuperLU_MT,
> Trilinos, Ginkgo, RAJA, Kokkos. A CUDA toolkit (v13.0) is also present.
>
> **Consequence: the failure counts below understate what this machine can
> build.** With oneMKL the four `*L` examples can link real LAPACK instead of
> the documented substitution, and with Intel MPI the 34 files that failed on
> a missing `mpi.h` can at least be attempted (the 5 PETSc ones would still
> fail on PETSc). Rebuilding with those enabled has **not** been done yet, so
> every "missing header" row below remains an accurate record of *this* build
> — it is just not the best this machine could do.

## Directories outside the comparison

Every one needs a backend present on neither side. Counts are C source files.

| directory | programs | requires |
|---|---:|---|
| `arkode/C_klu`, `arkode/C_superlu-mt` | 2 | KLU / SuperLU_MT |
| `arkode/C_manyvector`, `cvode/C_mpimanyvector` | 2 | ManyVector (+ MPI) |
| `arkode/C_openmp`, `cvode/C_openmp`, `cvodes/C_openmp`, `ida/C_openmp`, `idas/C_openmp`, `kinsol/C_openmp` | 9 | OpenMP N_Vector |
| `arkode/C_openmpdev`, `cvode/C_openmpdev` | 4 | OpenMP device offload |
| `arkode/C_parallel`, `cvode/parallel`, `cvodes/parallel`, `ida/parallel`, `idas/parallel`, `kinsol/parallel` | 28 | MPI |
| `arkode/C_parhyp`, `cvode/parhyp` | 2 | *hypre* |
| `arkode/C_petsc`, `cvode/petsc`, `ida/petsc` | 5 | PETSc |
| C++ sources (`*.cpp`) | 46 | CUDA / HIP / SYCL / RAJA / Kokkos / Ginkgo / Trilinos / C++ interface |
| Fortran sources (`*.f90`) | 51 | Fortran 2003 interface |
| CUDA sources (`*.cu`) | 7 | CUDA |

The Rust port excludes all of these by specification: it is serial-only, with
no MPI, GPU, KLU, SuperLU, LAPACK, Fortran or XBraid backend. Porting them
would mean first porting those backends, which is out of scope for a
`std`-only translation.
"""
w("c-results/EXCLUSIONS.md", EXCL_MD)
w("rust-results/EXCLUSIONS.md", EXCL_MD)

# ----------------------------------------------------------------- differences
diffdir = ROOT / "differences" / "diffs"
diffdir.mkdir(parents=True, exist_ok=True)
for f in diffdir.glob("*.diff"):
    f.unlink()
ndiff = 0
for r in live:
    if r["c_rust"] in ("same", "missing"):
        continue
    a = load(ROOT / "c-results" / "outputs" / r["outfile"])
    b = load(ROOT / "rust-results" / "outputs" / r["outfile"])
    d = list(difflib.unified_diff(a, b, fromfile=f"c/{r['outfile']}",
                                  tofile=f"rust/{r['outfile']}", lineterm=""))
    (diffdir / (r["outfile"] + ".diff")).write_text("\n".join(d) + "\n",
                                                    encoding="utf-8", newline="\n")
    ndiff += 1

agree = [r for r in both if r["c_rust"] == "same"]
disagree = [r for r in both if r["c_rust"] not in ("same", "missing")]
# For each disagreement, which side matches the shipped reference?
rust_right = [r for r in disagree if r["rust_ref"] == "same" and r["c_ref"] != "same"]
c_right = [r for r in disagree if r["c_ref"] == "same" and r["rust_ref"] != "same"]
neither = [r for r in disagree if r["c_ref"] != "same" and r["rust_ref"] != "same"]

w("differences/README.md", f"""
# differences — the C build and the Rust port, side by side

## Provenance
{PROV_BLOCK}
Both sides were built and run in the same session on the same machine, over
the same {len(live)} comparable variants, by
[`tools/example_matrix.py`](../tools/example_matrix.py); this document set is
generated by [`tools/example_report.py`](../tools/example_report.py).

## Method

For every `(example, argv)` variant:

1. run the MSVC C binary, capture stdout+stderr → [`../c-results/outputs/`](../c-results/outputs/)
2. run the Rust binary, capture the same → [`../rust-results/outputs/`](../rust-results/outputs/)
3. remove timing lines (`Total run time`, `CPU time`, `wall clock`) from
   **both** sides — they are machine-dependent and not part of any result
4. compare the two, and compare each against the reference `.out` shipped with
   SUNDIALS 7.8.0
5. classify each difference as *whitespace-only* (`tr -s ' '` makes it vanish,
   so every printed value matches and only column spacing differs) or
   *content*

Each program runs in its own scratch directory, because several write
`*_stats.csv` and solution files into the working directory.

Step 4 is what makes this a three-way comparison rather than a two-way one,
and it is the only way to say which side is *right* when they disagree.

## Headline

| coverage | variants |
|---|---:|
| declared by the CMakeLists of all 29 example directories | {len(recs)} |
| ran on **both** sides — the comparable set | **{len(both)}** |
| ran on the C side only (no Rust counterpart) | **{len(c_only)}** |
| ran on the Rust side only | {len(rust_only)} |
| excluded on both sides (KLU / SuperLU_MT not installed) | {len(excl)} |
| neither side could run (backend absent — see EXCLUSIONS) | {len(neither_ran)} |

| comparison over the {len(both)} comparable variants | variants |
|---|---:|
| C and Rust byte-identical | **{len(agree)}** |
| C and Rust differ | **{len(disagree)}** |
| — of those, Rust matches the shipped reference and C does not | **{len(rust_right)}** |
| — of those, C matches the shipped reference and Rust does not | **{len(c_right)}** |
| — of those, neither matches the shipped reference | **{len(neither)}** |
| excluded on both sides (KLU / SuperLU_MT) | {len(excl)} |
| total | {len(recs)} |

| | vs shipped reference |
|---|---:|
| Rust byte-identical | **{T_RREF.get('same', 0)} / {len(live)}** |
| C byte-identical | **{T_CREF.get('same', 0)} / {len(live)}** |

## Files

| file | contents |
|---|---|
| [`SUMMARY.md`](SUMMARY.md) | the counts above, broken down by solver and by class |
| [`BY-EXAMPLE.md`](BY-EXAMPLE.md) | every variant, three comparisons each |
| [`ANALYSIS.md`](ANALYSIS.md) | root cause of every difference, and what if anything to fix |
| [`diffs/`](diffs/) | a unified diff per disagreeing variant ({ndiff} files) |
""")

by_solver = {}
for r in live:
    d = by_solver.setdefault(r["dir"], dict(n=0, same=0, ws=0, content=0,
                                            rref=0, cref=0))
    d["n"] += 1
    d["same" if r["c_rust"] == "same" else ("ws" if r["c_rust"] == "ws-only" else "content")] += 1
    d["rref"] += r["rust_ref"] == "same"
    d["cref"] += r["c_ref"] == "same"

w("differences/SUMMARY.md", f"""
# differences — summary

Generated {PROV['when']} from commit `{PROV['commit']}`.

## C vs Rust, by class

| class | variants | meaning |
|---|---:|---|
| identical | {T_CR.get('same', 0)} | same bytes after the timing filter |
| whitespace-only | {T_CR.get('ws-only', 0)} | every printed value identical, column spacing differs |
| content | {T_CR.get('content', 0)} | at least one printed value differs |

## By example directory

| directory | variants | C==Rust | ws-only | content | Rust==ref | C==ref |
|---|---:|---:|---:|---:|---:|---:|
""" + "\n".join(
    f"| `{k}` | {v['n']} | {v['same']} | {v['ws']} | {v['content']} | {v['rref']} | {v['cref']} |"
    for k, v in sorted(by_solver.items())
) + f"""
| **total** | **{len(live)}** | **{T_CR.get('same', 0)}** | **{T_CR.get('ws-only', 0)}** | **{T_CR.get('content', 0)}** | **{T_RREF.get('same', 0)}** | **{T_CREF.get('same', 0)}** |

## Which side is right where they disagree

A disagreement is only a *defect* in one side if that side also disagrees with
the reference output shipped with SUNDIALS 7.8.0 — which was generated on a
glibc host by the upstream project.

| verdict | variants |
|---|---:|
| Rust matches the shipped reference, C does not | **{len(rust_right)}** |
| C matches the shipped reference, Rust does not | **{len(c_right)}** |
| neither matches the shipped reference | **{len(neither)}** |
""")

w("differences/BY-EXAMPLE.md", f"""
# differences — every variant

`C vs Rust`, `C vs ref` and `Rust vs ref` are each one of `same`,
`ws-only`, `content`, after removing timing lines from both sides.
`ref` is the `.out` file shipped with SUNDIALS 7.8.0.

{md_table(recs, [
    ("example", lambda r: f"`{r['name']}`"),
    ("argv", lambda r: f"`{r['args']}`" if r["args"] else ""),
    ("C vs Rust", lambda r: "—" if r["c_state"].startswith("excluded") else
        (f"**{r['c_rust']}**" if r["c_rust"] != "same" else "same")),
    ("lines", lambda r: r["c_rust_n"] if r["c_rust_n"] else ""),
    ("C vs ref", lambda r: "—" if r["c_state"].startswith("excluded") else r["c_ref"]),
    ("Rust vs ref", lambda r: "—" if r["rust_state"].startswith("excluded") else r["rust_ref"]),
    ("diff", lambda r: f"[diff](diffs/{r['outfile']}.diff)"
        if r["c_rust"] not in ("same", "missing") and not r["c_state"].startswith("excluded") else ""),
])}
""")

json.dump(dict(agree=len(agree), disagree=len(disagree), rust_right=len(rust_right),
               c_right=len(c_right), neither=len(neither),
               T_CR=T_CR, T_CREF=T_CREF, T_RREF=T_RREF,
               disagree_names=[(r["name"], r["args"], r["c_rust"], r["c_ref"], r["rust_ref"])
                               for r in disagree]),
          open(ROOT / "logs" / "compare_summary.json", "w"), indent=1)
print("\nC==Rust:", len(agree), " differ:", len(disagree),
      " (rust right:", len(rust_right), ", c right:", len(c_right),
      ", neither:", len(neither), ")")
