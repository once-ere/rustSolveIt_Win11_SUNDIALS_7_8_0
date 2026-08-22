#!/usr/bin/env python3
"""provenance_manifest.py — checksum every input, binary and output.

    python tools/provenance_manifest.py

Writes:
    c-results/provenance/20-input-sources.sha256      the .c files compiled
    c-results/provenance/21-binaries.sha256           the .exe files produced
    c-results/provenance/22-outputs.sha256            captured C outputs
    rust-results/provenance/20-input-sources.sha256   the .rs files compiled
    rust-results/provenance/21-binaries.sha256        the .exe files produced
    rust-results/provenance/22-outputs.sha256         captured Rust outputs
    differences/provenance/20-references.sha256       the shipped .out files
    differences/provenance/21-source-of-each-artefact.txt  written separately;
                                                      records that the
                                                      examples were compiled
                                                      from this repository

Every line is `<sha256>  <path>`, the format `sha256sum -c` reads, so a reader
can verify the whole set with one command.

SPDX-License-Identifier: BSD-3-Clause
"""

import hashlib
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
UPSTREAM = Path(r"C:\Users\nsh\Developer\sundials-7.8.0")
C_BIN = ROOT / "logs" / "c-build" / "bin"
RUST_BIN = ROOT / "target" / "release" / "examples"

SERIAL = ["cvode/serial", "cvodes/serial", "kinsol/serial",
          "ida/serial", "idas/serial", "arkode/C_serial"]
CRATE_OF = {"cvode/serial": "cvode_rs", "cvodes/serial": "cvodes_rs",
            "kinsol/serial": "kinsol_rs", "ida/serial": "ida_rs",
            "idas/serial": "idas_rs", "arkode/C_serial": "arkode_rs"}


def sha(p: Path) -> str:
    h = hashlib.sha256()
    with open(p, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def manifest(paths, out: Path, base: Path):
    out.parent.mkdir(parents=True, exist_ok=True)
    lines = []
    for p in sorted(paths):
        if p.is_file():
            lines.append(f"{sha(p)}  {p.relative_to(base).as_posix()}")
    out.write_text("\n".join(lines) + "\n", encoding="utf-8", newline="\n")
    print(f"{out.relative_to(ROOT).as_posix():52s} {len(lines):4d} files")
    return len(lines)


def main():
    # ---- inputs actually compiled -----------------------------------------
    # Every C example in THIS repository, all 29 directories — these are the
    # files that were compiled (see 30-build-each-example.json).
    c_src = list((ROOT / "examples").rglob("*.c"))
    manifest(c_src, ROOT / "c-results/provenance/20-input-sources.sha256", ROOT)

    rs_src = [p for d in SERIAL
              for p in (ROOT / "crates" / CRATE_OF[d] / "examples").glob("*.rs")]
    manifest(rs_src, ROOT / "rust-results/provenance/20-input-sources.sha256", ROOT)

    # ---- binaries ----------------------------------------------------------
    manifest(list(C_BIN.glob("*.exe")),
             ROOT / "c-results/provenance/21-binaries.sha256", C_BIN)
    manifest(list(RUST_BIN.glob("*.exe")),
             ROOT / "rust-results/provenance/21-binaries.sha256", RUST_BIN)

    # ---- captured outputs --------------------------------------------------
    manifest(list((ROOT / "c-results/outputs").glob("*.out")),
             ROOT / "c-results/provenance/22-outputs.sha256", ROOT / "c-results/outputs")
    manifest(list((ROOT / "rust-results/outputs").glob("*.out")),
             ROOT / "rust-results/provenance/22-outputs.sha256", ROOT / "rust-results/outputs")

    # ---- the shipped reference outputs ------------------------------------
    refs = list((ROOT / "examples").rglob("*.out"))
    manifest(refs, ROOT / "differences/provenance/20-references.sha256", ROOT / "examples")

    # The examples are compiled from this repository's own copy; that is
    # established by the recorded cl.exe command lines, not by comparing the
    # copy to its original. See differences/provenance/21-source-of-each-artefact.txt
    return 0


if __name__ == "__main__":
    sys.exit(main())
