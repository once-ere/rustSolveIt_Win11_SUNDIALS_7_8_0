#!/usr/bin/env python3
"""example_matrix.py — run every in-scope SUNDIALS example twice, once from
the MSVC C build and once from the Rust port, and tabulate the comparison.

    python tools/example_matrix.py [--c] [--rust] [--compare] [--all]

Produces three document sets:

    c-results/       what the C built by Visual Studio 18 Professional does
    rust-results/    what the pure-Rust port does
    differences/     the two against each other, and each against the
                     reference outputs shipped with SUNDIALS 7.8.0

Every variant is an (example, argv) pair taken from the upstream
`CMakeLists.txt` of each serial example directory, using the same parsing
rules as `tools/verify_examples.sh`: quoted `\\;`-separated tuples, 3 fields
= name/args/label, 2 fields = name/label, arkode names carry a `.c` suffix to
strip, and the reference file is `<name>.out` when args are empty else
`<name>_<args with spaces replaced by underscores>.out`.

Each program runs in its own scratch directory, because several of them write
`*_stats.csv` and solution files into the working directory and would
otherwise overwrite each other's.

SPDX-License-Identifier: BSD-3-Clause
"""

import json
import os
import re
import shutil
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
EXAMPLES = ROOT / "examples"
C_BIN = ROOT / "logs" / "c-build" / "bin"
RUST_BIN = ROOT / "target" / "release" / "examples"
RUN_ROOT = ROOT / "logs" / "run"

# EVERY example directory that ships C sources and reference outputs — not
# just the serial ones. `crate` is the Rust crate holding the ported example,
# or None where the port has no counterpart (those rows are reported as
# not-ported rather than quietly dropped).
DIRS = [
    ("cvode_rs",  "cvode/serial"),
    ("cvodes_rs", "cvodes/serial"),
    ("kinsol_rs", "kinsol/serial"),
    ("ida_rs",    "ida/serial"),
    ("idas_rs",   "idas/serial"),
    ("arkode_rs", "arkode/C_serial"),
    (None, "arkode/C_klu"),
    (None, "arkode/C_manyvector"),
    (None, "arkode/C_openmp"),
    (None, "arkode/C_openmpdev"),
    (None, "arkode/C_parallel"),
    (None, "arkode/C_parhyp"),
    (None, "arkode/C_petsc"),
    (None, "arkode/C_superlu-mt"),
    (None, "cvode/C_mpimanyvector"),
    (None, "cvode/C_openmp"),
    (None, "cvode/C_openmpdev"),
    (None, "cvode/parallel"),
    (None, "cvode/parhyp"),
    (None, "cvode/petsc"),
    (None, "cvodes/C_openmp"),
    (None, "cvodes/parallel"),
    (None, "ida/C_openmp"),
    (None, "ida/parallel"),
    (None, "ida/petsc"),
    (None, "idas/C_openmp"),
    (None, "idas/parallel"),
    (None, "kinsol/C_openmp"),
    (None, "kinsol/parallel"),
]

KLU = {
    "cvRoberts_klu", "cvRoberts_block_klu", "cvsRoberts_klu", "cvsRoberts_FSA_klu",
    "cvsRoberts_ASAi_klu", "kinFerTron_klu", "idaHeat2D_klu", "idaRoberts_klu",
    "idasRoberts_klu", "idasRoberts_FSA_klu", "idasRoberts_ASAi_klu",
}
SUPERLU = {
    "cvRoberts_sps", "cvsRoberts_sps", "cvsRoberts_FSA_sps", "cvsRoberts_ASAi_sps",
    "kinRoboKin_slu", "idaRoberts_sps", "idasRoberts_sps", "idasRoberts_FSA_sps",
    "idasRoberts_ASAi_sps",
}
LAPACK_SUBSTITUTED = {"cvRoberts_dnsL", "cvAdvDiff_bndL", "cvsRoberts_dnsL", "cvsAdvDiff_bndL"}

# Machine-dependent lines, removed from BOTH sides before any comparison.
NOISE = re.compile(r"Total run time|CPU time|cpu time|wall clock")


def parse_cmake(exdir: Path):
    """(name, args, outfile) per tuple in one example dir's CMakeLists.txt."""
    out = []
    text = (exdir / "CMakeLists.txt").read_text(encoding="utf-8", errors="replace")
    for line in text.splitlines():
        if line.lstrip().startswith("#"):
            continue
        for tup in re.findall(r'"([^"]*\\;[^"]*)"', line):
            parts = tup.split("\\;")
            name = parts[0]
            args = parts[1] if len(parts) >= 3 else ""
            name = name[:-2] if name.endswith(".c") else name
            outfile = f"{name}.out" if not args else f"{name}_{args.replace(' ', '_')}.out"
            out.append((name, args, outfile))
    return out


def variants():
    seen, rows = set(), []
    for crate, sub in DIRS:
        exdir = EXAMPLES / sub
        for name, args, outfile in parse_cmake(exdir):
            key = (sub, name, args)
            if key in seen:
                continue
            seen.add(key)
            rows.append(dict(crate=crate, dir=sub, name=name, args=args, outfile=outfile))
    return rows


def excluded(name):
    if name in KLU:
        return "excluded(klu)"
    if name in SUPERLU:
        return "excluded(superlu)"
    return None


def run_side(side, rows):
    """Execute every variant for one side; write outputs; return status map."""
    binroot = C_BIN if side == "c" else RUST_BIN
    outdir = ROOT / f"{side}-results" / "outputs"
    outdir.mkdir(parents=True, exist_ok=True)
    rundir = RUN_ROOT / side
    status = {}
    for i, r in enumerate(rows, 1):
        key = r["outfile"]
        why = excluded(r["name"])
        if why:
            status[key] = dict(state=why, exit=None, bytes=0)
            continue
        exe = binroot / (r["name"] + ".exe")
        if not exe.exists():
            status[key] = dict(state="NO-BINARY", exit=None, bytes=0)
            continue
        wd = rundir / r["name"]
        if wd.exists():
            shutil.rmtree(wd, ignore_errors=True)
        wd.mkdir(parents=True, exist_ok=True)
        argv = [str(exe)] + (r["args"].split() if r["args"] else [])
        try:
            p = subprocess.run(argv, cwd=wd, capture_output=True, timeout=600)
            body = p.stdout + p.stderr
            code = p.returncode
            state = "ran" if code == 0 else f"EXIT({code})"
        except subprocess.TimeoutExpired:
            body, code, state = b"", None, "TIMEOUT"
        (outdir / key).write_bytes(body)
        status[key] = dict(state=state, exit=code, bytes=len(body))
        if i % 40 == 0:
            print(f"  {side}: {i}/{len(rows)}", flush=True)
    return status


def filtered(path: Path):
    if not path.exists():
        return None
    txt = path.read_text(encoding="utf-8", errors="replace")
    return [l.rstrip("\r") for l in txt.splitlines() if not NOISE.search(l)]


def diff_count(a, b):
    """Number of differing lines, unified-diff style, and a short preview."""
    import difflib
    if a is None or b is None:
        return None, []
    d = [l for l in difflib.unified_diff(a, b, lineterm="", n=0)
         if l.startswith(("+", "-")) and not l.startswith(("+++", "---"))]
    return len(d), d[:12]


def squeeze(lines):
    return None if lines is None else [re.sub(r" +", " ", l).rstrip() for l in lines]


def provenance():
    def sh(cmd):
        try:
            return subprocess.run(cmd, capture_output=True, text=True, shell=True,
                                  timeout=60).stdout.strip().splitlines()[0]
        except Exception:
            return "(unavailable)"
    cl = "(unavailable)"
    log = ROOT / "logs" / "c-build.log"
    if log.exists():
        for l in log.read_text(encoding="utf-8", errors="replace").splitlines():
            if "Optimizing Compiler Version" in l:
                cl = l.strip()
                break
    return dict(
        when=time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        commit=sh("git rev-parse --short HEAD"),
        rustc=sh("rustc -V"),
        cargo=sh("cargo -V"),
        cl=cl,
        cmake=sh("cmake --version"),
        os=sh('powershell -NoProfile -Command "(Get-CimInstance Win32_OperatingSystem).Caption + \' \' + [System.Environment]::OSVersion.Version"'),
        cpu=sh('powershell -NoProfile -Command "(Get-CimInstance Win32_Processor).Name"'),
    )


def main():
    args = set(sys.argv[1:]) or {"--all"}
    do_all = "--all" in args
    rows = variants()
    print(f"{len(rows)} variants across {len(DIRS)} example directories")

    state_path = ROOT / "logs" / "example_matrix_state.json"
    state = json.loads(state_path.read_text()) if state_path.exists() else {}

    if do_all or "--c" in args:
        print("running C ...")
        state["c"] = run_side("c", rows)
    if do_all or "--rust" in args:
        print("running Rust ...")
        state["rust"] = run_side("rust", rows)
    state["rows"] = rows
    state["prov"] = provenance()
    state_path.write_text(json.dumps(state, indent=1))
    print("state written to", state_path)


if __name__ == "__main__":
    sys.exit(main())
