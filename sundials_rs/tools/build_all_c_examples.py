#!/usr/bin/env python3
"""build_all_c_examples.py — compile EVERY C example in ./examples with MSVC,
one file at a time, recording the literal command line and the real outcome.

    python tools/build_all_c_examples.py

This does not use CMake to choose what to build. It attempts all 180 `.c`
files found under this repository's own `examples/` directory — the copy the
user placed here — so that "this example does not build" is a *measured*
result with a compiler error attached, never an assumption.

The SUNDIALS *library* is still built by CMake from the upstream source tree,
because only the examples were copied into this repository; `src/` and
`include/` are not here. That is stated in the generated documents.

Writes:
    c-results/provenance/30-build-each-example.json   one record per .c file:
        the literal cl.exe command, exit code, and full compiler/linker output
    c-results/provenance/31-build-each-example.txt    the same, readable
    logs/c-build/bin/<name>.exe                       the binaries

SPDX-License-Identifier: BSD-3-Clause
"""

import json
import os
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
EXAMPLES = ROOT / "examples"
UPSTREAM = Path(r"C:\Users\nsh\Developer\sundials-7.8.0")
BUILD = ROOT / "logs" / "c-build"
BIN = BUILD / "bin"
OBJ = BUILD / "objs"
PROV = ROOT / "c-results" / "provenance"
VCVARS = Path(r"C:\Program Files\Microsoft Visual Studio\18\Professional\VC\Auxiliary\Build\vcvars64.bat")

# Extra compiler flags per example directory. Everything else uses the base
# flags; a directory needing a backend that is not installed still gets
# attempted, and the compiler's own error is what gets recorded.
EXTRA_FLAGS = {
    "arkode/C_openmp": ["/openmp"],
    "cvode/C_openmp": ["/openmp"],
    "cvodes/C_openmp": ["/openmp"],
    "ida/C_openmp": ["/openmp"],
    "idas/C_openmp": ["/openmp"],
    "kinsol/C_openmp": ["/openmp"],
    "arkode/C_openmpdev": ["/openmp"],
    "cvode/C_openmpdev": ["/openmp"],
}

BASE_FLAGS = [
    "/nologo", "/O2", "/Ob2", "/DNDEBUG", "/MD",
    "/DWIN32", "/D_WINDOWS",
    "/DSUNDIALS_STATIC_DEFINE", "/D_CRT_SECURE_NO_WARNINGS",
]

# Exactly one solver library per example. Linking all of them at once fails
# with LNK2005: CVODES re-defines CVODE's symbols (CVodeGetJac and friends),
# and IDAS re-defines IDA's, because each is a superset built from shared
# sources. The solver is chosen by the top-level directory the example is in.
SOLVER_LIB = {
    "cvode": "sundials_cvode_static.lib",
    "cvodes": "sundials_cvodes_static.lib",
    "ida": "sundials_ida_static.lib",
    "idas": "sundials_idas_static.lib",
    "kinsol": "sundials_kinsol_static.lib",
    "arkode": "sundials_arkode_static.lib",
}

# Vector implementations are additive and carry no conflicting symbols.
SUPPORT_LIBS = [
    "sundials_core_static.lib",
    "sundials_nvecserial_static.lib",
    "sundials_nvecmanyvector_static.lib",
    "sundials_sunmatrixdense_static.lib",
    "sundials_sunmatrixband_static.lib",
    "sundials_sunmatrixsparse_static.lib",
    "sundials_sunlinsoldense_static.lib",
    "sundials_sunlinsolband_static.lib",
    "sundials_sunlinsolpcg_static.lib",
    "sundials_sunlinsolspbcgs_static.lib",
    "sundials_sunlinsolspfgmr_static.lib",
    "sundials_sunlinsolspgmr_static.lib",
    "sundials_sunlinsolsptfqmr_static.lib",
    "sundials_sunnonlinsolnewton_static.lib",
    "sundials_sunnonlinsolfixedpoint_static.lib",
    "sundials_sunnonlinsolauto_static.lib",
    "sundials_sundomeigestpower_static.lib",
]


def libs_for(d, available):
    """Libraries to link for an example in directory `d`."""
    solver = SOLVER_LIB[d.split("/")[0]]
    want = [solver] + SUPPORT_LIBS
    if d.endswith("openmp") or d.endswith("openmpdev"):
        want.append("sundials_nvecopenmp_static.lib")
    return [l for l in want if l in available]


def vcvars_env():
    """Environment as vcvars64.bat leaves it."""
    out = subprocess.run(
        f'call "{VCVARS}" >nul && set',
        shell=True, capture_output=True, text=True, errors="replace")
    env = {}
    for line in out.stdout.splitlines():
        if "=" in line:
            k, v = line.split("=", 1)
            env[k] = v
    if "INCLUDE" not in env:
        print("vcvars64.bat did not set INCLUDE; aborting", file=sys.stderr)
        sys.exit(1)
    return env


def find_cl(env):
    """Absolute path to cl.exe. CreateProcess resolves the executable against
    the *parent* PATH, not the child environment's, so the child env alone is
    not enough."""
    for d in env.get("PATH", "").split(os.pathsep):
        c = Path(d) / "cl.exe"
        if c.is_file():
            return str(c)
    print("cl.exe not found on the vcvars PATH; aborting", file=sys.stderr)
    sys.exit(1)


def main():
    env = vcvars_env()
    CL = find_cl(env)
    OBJ.mkdir(parents=True, exist_ok=True)
    BIN.mkdir(parents=True, exist_ok=True)
    PROV.mkdir(parents=True, exist_ok=True)

    available = {p.name for p in BIN.glob('*.lib')}
    libs = sorted(available)
    if not libs:
        print(f"No .lib in {BIN}. Run tools\\build_c_examples.cmd first.", file=sys.stderr)
        return 1

    srcs = sorted(EXAMPLES.rglob("*.c"))
    print(f"attempting {len(srcs)} C files from {EXAMPLES}")
    records = []
    for i, src in enumerate(srcs, 1):
        rel = src.relative_to(EXAMPLES).as_posix()
        d = rel.rsplit("/", 1)[0]
        name = src.stem
        cmd = [CL] + BASE_FLAGS + EXTRA_FLAGS.get(d, []) + [
            f"/I{src.parent}",                     # companion .h files
            f"/I{UPSTREAM / 'include'}",           # public SUNDIALS headers
            f"/I{BUILD / 'include'}",              # generated sundials_config.h
            str(src),
            f"/Fo{OBJ / (name + '.obj')}",
            f"/Fe{BIN / (name + '.exe')}",
            "/link", f"/LIBPATH:{BIN}",
        ] + libs_for(d, available)
        p = subprocess.run(cmd, capture_output=True, text=True, errors="replace",
                           env=env, cwd=str(OBJ))
        ok = p.returncode == 0 and (BIN / (name + ".exe")).exists()
        records.append(dict(
            file=rel, dir=d, name=name, ok=ok, returncode=p.returncode,
            command=subprocess.list2cmdline(cmd),
            output=(p.stdout + p.stderr).strip(),
        ))
        if i % 20 == 0:
            print(f"  {i}/{len(srcs)}  ok so far: {sum(1 for r in records if r['ok'])}")

    (PROV / "30-build-each-example.json").write_text(
        json.dumps(records, indent=1), encoding="utf-8", newline="\n")

    lines = []
    for r in records:
        lines += [
            "=" * 78,
            f"{r['file']}   {'BUILT' if r['ok'] else 'FAILED (exit %d)' % r['returncode']}",
            "-" * 78,
            "command:",
            "  " + r["command"],
            "",
            "compiler/linker output:",
        ]
        lines += ["  " + l for l in (r["output"] or "(none)").splitlines()]
        lines.append("")
    (PROV / "31-build-each-example.txt").write_text(
        "\n".join(lines) + "\n", encoding="utf-8", newline="\n")

    built = sum(1 for r in records if r["ok"])
    print(f"\nbuilt {built} of {len(records)}")
    by_dir = {}
    for r in records:
        s = by_dir.setdefault(r["dir"], [0, 0])
        s[0] += 1
        s[1] += r["ok"]
    for d in sorted(by_dir):
        n, k = by_dir[d]
        print(f"  {d:34s} {k:3d}/{n:<3d}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
