#!/usr/bin/env bash
# libm_differential_win.sh [fn|all] [n]
#
# Windows-native driver for the deterministic-libm differential tests.
#
# Builds tools/libm_oracle.c inside a WSL2 Linux guest with the guest's cc —
# real glibc, real x86-64, the libm that generated the upstream SUNDIALS
# reference outputs — generates one bit-stream per function, and then runs
# the natively built, natively executed Windows test binary against them.
# That crossing is the measurement: the Rust in crates/sundials_core/src/
# sundials_libm/ must reproduce glibc's bits when compiled by
# x86_64-pc-windows-msvc.
#
# Usage:
#   tools/libm_differential_win.sh              # all 12, 4,000,000 inputs each
#   tools/libm_differential_win.sh sin          # one function
#   tools/libm_differential_win.sh sin 500000   # one function, smaller corpus
#   tools/libm_differential_win.sh test         # re-run the tests, reusing
#                                               # whatever streams already exist
#
# Environment:
#   SUNDIALS_WSL_DISTRO   guest to build the oracle in (default Ubuntu-24.04)
#
# Streams land in logs/oracle/ (gitignored): 8 bytes per input, so 32 MB per
# function at the default size.
#
# SPDX-License-Identifier: BSD-3-Clause

set -u
cd "$(dirname "$0")/.."
WS_ROOT="$(pwd)"
LOGS="$WS_ROOT/logs"
# Output/inputs directory; override to keep an out-of-sample corpus
# separate from the one the routines were developed against.
ORACLE_DIR="${SUNDIALS_ORACLE_OUT:-$LOGS/oracle}"
mkdir -p "$ORACLE_DIR"
LOG="$LOGS/libm_differential_win.log"
DISTRO="${SUNDIALS_WSL_DISTRO:-Ubuntu-24.04}"

ALL_FNS="exp log sin cos atan asin acos sinh cosh acosh expm1 log1p"
WHICH="${1:-all}"
N="${2:-4000000}"

say() { printf '%s\n' "$*" | tee -a "$LOG"; }

case "$WHICH" in
  test) FNS="" ;;
  all)  FNS="$ALL_FNS" ;;
  *)    FNS="$WHICH" ;;
esac

: >"$LOG"
say "== deterministic libm differential (Windows native, glibc oracle) =="
say "date  : $(date -u '+%Y-%m-%dT%H:%M:%SZ')"
say "rustc : $(rustc -V)"
say "corpus: $N per function"
say ""

if [ -n "$FNS" ]; then
  if ! command -v wsl.exe >/dev/null 2>&1; then
    say "ERROR: wsl.exe not on PATH; cannot build a glibc oracle." >&2
    exit 2
  fi
  GUEST_DIR="/tmp/sundials_libm_oracle"
  WIN_WS_GUEST="$(printf '%s' "$WS_ROOT" | sed -e 's|^/\([a-zA-Z]\)/|/mnt/\1/|')"
  WIN_ORACLE_GUEST="$(printf '%s' "$ORACLE_DIR" | sed -e 's|^/\([a-zA-Z]\)/|/mnt/\1/|')"
  say "-- building the oracle in WSL guest '$DISTRO' --"
  wsl.exe -d "$DISTRO" -- bash -lc "
    set -e
    mkdir -p $GUEST_DIR
    mkdir -p $GUEST_DIR
    cp '$WIN_WS_GUEST/tools/libm_oracle.c' $GUEST_DIR/
    cc -O2 -o $GUEST_DIR/libm_oracle $GUEST_DIR/libm_oracle.c -lm
    echo \"guest: \$(uname -srm) / \$(ldd --version | head -1) / \$(cc --version | head -1)\"
  " 2>&1 | grep -vE '^(wsl:|\s*$)' | tail -3 | tee -a "$LOG"

  for fn in $FNS; do
    say "generating $fn ..."
    wsl.exe -d "$DISTRO" -- bash -lc \
      "$GUEST_DIR/libm_oracle $fn $N > '$WIN_ORACLE_GUEST/$fn.bin'" \
      2>&1 | grep -E 'results, input hash' | tee -a "$LOG"
  done
  say ""
fi

say "-- differential: sundials_libm (native Windows build) vs glibc --"
# STRICT turns "no oracle -> the test passes without comparing anything" into
# a hard failure. This script exists to measure; a run of it that measured
# nothing must not report success.
SUNDIALS_LIBM_ORACLE_DIR="$ORACLE_DIR" SUNDIALS_LIBM_ORACLE_STRICT=1 \
  cargo test --release -p sundials_core --lib sundials_libm \
  -- --nocapture --test-threads=1 2>&1 | tee -a "$LOG" | grep -vE '^\s*(Compiling|Finished)'

say ""
say "Done. Read logs/libm_differential_win.log"
