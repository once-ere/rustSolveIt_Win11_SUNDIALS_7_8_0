#!/usr/bin/env bash
# fetch_libm_sources.sh — download the C sources this port's deterministic
# libm is translated from.
#
# They are NOT committed. glibc is LGPL-2.1-or-later and this repository is
# BSD-3-Clause; keeping the upstream C out of the tree keeps the licence
# boundary legible, and the ported Rust carries its own per-file provenance
# and licence header. Run this before working on `sundials_libm`, and read
# `NOTICE` § "Deterministic libm" for the licence position.
#
#   tools/fetch_libm_sources.sh          # -> reference/ (gitignored)
#
# Pinned versions:
#   glibc  2.39   — the libm that generated the upstream SUNDIALS reference
#                   outputs, and the one this port reproduces.
#   musl   master — used only for `exp` and `log`, which are the MIT-licensed
#                   ARM optimized-routines implementations that glibc >= 2.28
#                   also ships. Preferring musl's copy keeps those two
#                   routines' provenance MIT, as the already-ported `pow` is.
#
# SPDX-License-Identifier: BSD-3-Clause

set -eu
cd "$(dirname "$0")/.."
REF="$PWD/reference"
GLIBC_TAG="${GLIBC_TAG:-glibc-2.39}"
GLIBC_RAW="https://raw.githubusercontent.com/bminor/glibc/$GLIBC_TAG"
MUSL_RAW="https://raw.githubusercontent.com/kraj/musl/master"

mkdir -p "$REF/glibc-2.39/dbl-64" "$REF/musl"

get() { # get <url> <dest>
  if curl -fsS -o "$2" "$1"; then
    printf '  %-24s %8s bytes\n' "$(basename "$2")" "$(wc -c <"$2")"
  else
    printf '  %-24s FAILED %s\n' "$(basename "$2")" "$1" >&2
    return 1
  fi
}

echo "glibc $GLIBC_TAG — sysdeps/ieee754/dbl-64/"
for f in \
  s_sin.c usncs.h sincostab.c branred.c branred.h mydefs.h dla.h \
  s_atan.c atnat.h uatan.tbl \
  e_asin.c uasncs.h asincos.tbl root.tbl powtwo.tbl \
  e_sinh.c e_cosh.c e_acosh.c s_expm1.c s_log1p.c \
  e_exp.c e_log.c e_exp_data.c e_log_data.c math_config.h
do
  get "$GLIBC_RAW/sysdeps/ieee754/dbl-64/$f" "$REF/glibc-2.39/dbl-64/$f" || true
done

echo "glibc $GLIBC_TAG — x86-64 multiarch (which routines get an FMA build)"
get "$GLIBC_RAW/sysdeps/x86_64/fpu/multiarch/s_sin-fma.c" "$REF/glibc-2.39/s_sin-fma.c" || true
get "$GLIBC_RAW/sysdeps/x86_64/fpu/multiarch/e_asin-fma.c" "$REF/glibc-2.39/e_asin-fma.c" || true
get "$GLIBC_RAW/sysdeps/x86_64/fpu/multiarch/s_atan-fma.c" "$REF/glibc-2.39/s_atan-fma.c" || true
get "$GLIBC_RAW/sysdeps/x86_64/fpu/multiarch/s_expm1-fma.c" "$REF/glibc-2.39/s_expm1-fma.c" || true
get "$GLIBC_RAW/sysdeps/x86_64/fpu/multiarch/s_log1p-fma.c" "$REF/glibc-2.39/s_log1p-fma.c" || true

echo "musl (MIT) — src/math/"
for f in exp.c exp_data.c exp_data.h log.c log_data.c log_data.h libm.h; do
  get "$MUSL_RAW/src/math/$f" "$REF/musl/$f" || true
done
get "$MUSL_RAW/src/internal/libm.h" "$REF/musl/internal_libm.h" || true

echo
echo "Done. Sources in reference/ (gitignored)."
