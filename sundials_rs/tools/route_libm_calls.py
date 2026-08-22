#!/usr/bin/env python3
"""route_libm_calls.py — move every host-libm call in the port onto
`sundials_libm`.

The port evaluates transcendentals through `f64` methods, which Rust
documents as having unspecified precision and forwards to the host libm. On
Windows that is the Microsoft UCRT, which is not the libm the upstream
SUNDIALS reference outputs were generated with. This script rewrites those
call sites to the deterministic, glibc-equivalent routines in
`sundials_core::sundials_libm`:

    (0.2 * yup).exp()   ->   sundials_libm::exp(0.2 * yup)
    t.atan()            ->   sundials_libm::atan(t)
    con_errors[i].ln()  ->   sundials_libm::log(con_errors[i])

Turning `x.f()` into `f(x)` also moves the Rust closer to the C it was
translated from, which writes `exp(0.2*yup)`.

The receiver is found by scanning backwards from the `.` over balanced
parentheses and brackets, method/field chains and paths, so arbitrary
expressions are handled. Matches inside string literals, char literals and
comments are skipped.

    tools/route_libm_calls.py            # report only
    tools/route_libm_calls.py --apply    # rewrite in place

SPDX-License-Identifier: BSD-3-Clause
"""

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

# f64 method -> sundials_libm function. sqrt/mul_add/abs/copysign/floor/ceil/
# round/trunc are IEEE-754 specified and identical on every target; they are
# deliberately absent.
MAP = {
    "sin": "sin",
    "cos": "cos",
    "exp": "exp",
    "ln": "log",
    "asin": "asin",
    "acos": "acos",
    "atan": "atan",
    "sinh": "sinh",
    "cosh": "cosh",
    "acosh": "acosh",
    "exp_m1": "expm1",
    "ln_1p": "log1p",
}

CALL = re.compile(r"\.\s*(" + "|".join(MAP) + r")\s*\(\s*\)")

IDENT = re.compile(r"[A-Za-z0-9_]")


def masked(src: str) -> str:
    """Copy of `src` with strings, chars and comments blanked out, so regex
    matches inside them are not mistaken for code."""
    out = list(src)
    i, n = 0, len(src)
    while i < n:
        c = src[i]
        if c == '"':
            j = i + 1
            while j < n:
                if src[j] == "\\":
                    j += 2
                    continue
                if src[j] == '"':
                    break
                j += 1
            for k in range(i, min(j + 1, n)):
                out[k] = " "
            i = j + 1
        elif c == "'":
            # char literal or lifetime; only blank a real char literal
            j = i + 1
            if j < n and src[j] == "\\":
                j += 2
            else:
                j += 1
            if j < n and src[j] == "'":
                for k in range(i, j + 1):
                    out[k] = " "
                i = j + 1
            else:
                i += 1
        elif c == "/" and i + 1 < n and src[i + 1] == "/":
            j = src.find("\n", i)
            j = n if j < 0 else j
            for k in range(i, j):
                out[k] = " "
            i = j
        elif c == "/" and i + 1 < n and src[i + 1] == "*":
            j = src.find("*/", i + 2)
            j = n if j < 0 else j + 2
            for k in range(i, j):
                out[k] = " "
            i = j
        else:
            i += 1
    return "".join(out)


def receiver_start(s: str, dot: int) -> int:
    """Index of the first character of the receiver expression ending at `dot`."""
    i = dot - 1
    while i >= 0 and s[i] in " \t\r\n":
        i -= 1
    if i < 0:
        raise ValueError("no receiver")
    while True:
        c = s[i]
        if c in ")]":
            close, opener = c, "(" if c == ")" else "["
            depth = 0
            while i >= 0:
                if s[i] in ")]":
                    depth += 1
                elif s[i] in "([":
                    depth -= 1
                    if depth == 0:
                        break
                i -= 1
            if i < 0:
                raise ValueError("unbalanced " + close + opener)
            i -= 1
            j = i
            while j >= 0 and s[j] in " \t":
                j -= 1
            # a call or index whose base is an identifier/path: keep going
            if j >= 0 and (IDENT.match(s[j]) or s[j] == ">"):
                i = j
                continue
            return i + 1
        if IDENT.match(c):
            while i >= 0 and IDENT.match(s[i]):
                i -= 1
            if i >= 0 and s[i] == "." and i - 1 >= 0 and (IDENT.match(s[i - 1]) or s[i - 1] in ")]"):
                i -= 1          # field access, method chain, or float literal
                continue
            if i - 1 >= 0 and s[i] == ":" and s[i - 1] == ":":
                i -= 2          # path segment
                continue
            return i + 1
        return i + 1


def strip_outer_parens(e: str) -> str:
    """`((y - ONE) * z)` -> `(y - ONE) * z`, but `(a) * (b)` unchanged: the
    call site supplies its own parentheses, and a redundant pair is a
    `unused_parens` warning, which this project does not allow."""
    while len(e) >= 2 and e[0] == "(" and e[-1] == ")":
        depth = 0
        for k, c in enumerate(e):
            if c == "(":
                depth += 1
            elif c == ")":
                depth -= 1
                if depth == 0 and k != len(e) - 1:
                    return e
        e = e[1:-1].strip()
    return e


def ensure_import(src: str, crate: str) -> str:
    """Examples reach the module through their solver crate. Some glob-import
    the crate root, some name every item; add the explicit `use` either way."""
    line = f"use {crate}::sundials_libm;"
    if line in src:
        return src
    nl = chr(10)
    uses = list(re.finditer(r"^use [^" + nl + r"]*;$", src, re.M))
    if uses:
        at = uses[-1].end()
        return src[:at] + nl + line + src[at:]
    lines = src.split(nl)
    for i, l in enumerate(lines):
        s = l.strip()
        if s and not s.startswith("//") and not s.startswith("#!"):
            lines.insert(i, line + nl)
            return nl.join(lines)
    return src + nl + line + nl


def rewrite(src: str, path: Path, report: list) -> str:
    changed = True
    while changed:
        changed = False
        m = CALL.search(masked(src))
        if not m:
            break
        method = m.group(1)
        target = MAP[method]
        start = receiver_start(src, m.start())
        recv = src[start:m.start()].strip()
        recv = strip_outer_parens(recv)
        src = src[:start] + f"sundials_libm::{target}({recv})" + src[m.end():]
        report.append(f"{path}: {recv[:48]}.{method}()  ->  sundials_libm::{target}(…)")
        changed = True
    return src


def main() -> int:
    apply = "--apply" in sys.argv
    files = []
    for crate in sorted((ROOT / "crates").iterdir()):
        for sub in ("src", "examples"):
            d = crate / sub
            if d.is_dir():
                files += sorted(d.rglob("*.rs"))

    report, touched = [], 0
    for f in files:
        # sundials_libm is the implementation; it must keep using whatever it
        # uses, and sundials_math's pow is already host-independent.
        if "sundials_libm" in str(f):
            continue
        src = f.read_text(encoding="utf-8")
        if not CALL.search(masked(src)):
            continue
        new = rewrite(src, f.relative_to(ROOT), report)
        parts = f.relative_to(ROOT).parts
        if parts[2] == "examples":
            new = ensure_import(new, parts[1])
        elif parts[1] == "sundials_core":
            new = new.replace("sundials_libm::", "crate::sundials_libm::")
            new = new.replace("crate::crate::", "crate::")
        else:
            new = new.replace("sundials_libm::", "crate::sundials_libm::")
            new = new.replace("crate::crate::", "crate::")
        if new != src:
            touched += 1
            if apply:
                f.write_text(new, encoding="utf-8", newline="\n")
    for line in report:
        print(line)
    print(f"\n{len(report)} call sites in {touched} files"
          f"{' — REWRITTEN' if apply else ' — report only, pass --apply'}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
