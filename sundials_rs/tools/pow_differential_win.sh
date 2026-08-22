#!/usr/bin/env bash
# pow_differential_win.sh [domain|random|all]
#
# Windows-native driver for the deterministic-`pow` differential test.
#
# PLATFORM SCOPE — Windows 11 on Intel/AMD x86-64, run from Git Bash / MSYS2.
#
# Two oracles are involved, and the distinction is the whole point of this
# script:
#
#   1. the *reference* oracle — glibc's `pow`, the routine that generated the
#      upstream SUNDIALS reference `.out` files. It cannot be produced by a
#      Windows toolchain, so it is generated inside a WSL2 Linux guest (which
#      is a real glibc/x86-64 userspace) and then consumed by the *natively
#      built, natively run* Windows test binary. That crossing is the
#      measurement: the Rust routine compiled by x86_64-pc-windows-msvc must
#      reproduce glibc's bit pattern exactly.
#
#   2. the *host* oracle — the Microsoft UCRT `pow` that `f64::powf` would
#      resolve to on this target. It is generated here with clang. It is not
#      a gate; it is the evidence for why the deterministic routine exists on
#      Windows at all — see logs/pow_differential_win.log for the tally.
#
# Usage:
#   tools/pow_differential_win.sh all
#
# Environment:
#   SUNDIALS_WSL_DISTRO  WSL distribution used for the glibc oracle
#                        (default: Ubuntu-24.04). Set to "-" to skip the
#                        glibc side and measure UCRT only.
#   CC_WIN               C compiler for the UCRT oracle (default: clang).
#
# Corpus sizes: 5,900,000 (domain) and 20,000,000 (random) pairs, one
# uint64_t each -> 47 MB and 160 MB per oracle. They land in logs/, which is
# gitignored; nothing here is committed except the log.
#
# SPDX-License-Identifier: BSD-3-Clause

set -u
cd "$(dirname "$0")/.."
WS_ROOT="$(pwd)"
LOGS="$WS_ROOT/logs"
mkdir -p "$LOGS"
LOG="$LOGS/pow_differential_win.log"
WHICH="${1:-all}"
CC_WIN="${CC_WIN:-clang}"
DISTRO="${SUNDIALS_WSL_DISTRO:-Ubuntu-24.04}"

say() { printf '%s\n' "$*" | tee -a "$LOG"; }

: >"$LOG"
say "== pow differential (Windows native) =="
say "date        : $(date -u '+%Y-%m-%dT%H:%M:%SZ')"
say "host        : $(uname -s) $(uname -m) / $(cmd.exe /c ver 2>/dev/null | tr -d '\r' | tail -2 | head -1)"
say "rustc       : $(rustc -vV | tr '\n' ' ')"
say "cc (UCRT)   : $($CC_WIN --version 2>/dev/null | head -1)"
say ""

# ---------------------------------------------------------------- UCRT side
say "-- host oracle: Microsoft UCRT pow (clang, x86_64-pc-windows-msvc) --"
if ! command -v "$CC_WIN" >/dev/null 2>&1; then
  say "SKIP: $CC_WIN not on PATH"
else
  "$CC_WIN" -O2 -o "$LOGS/pow_oracle_ucrt.exe" "$WS_ROOT/tools/pow_oracle.c" \
    2>&1 | tee -a "$LOG"
  if [ -x "$LOGS/pow_oracle_ucrt.exe" ]; then
    for c in domain random; do
      case "$WHICH" in all) ;; "$c") ;; *) continue ;; esac
      say "generating UCRT $c corpus ..."
      "$LOGS/pow_oracle_ucrt.exe" "$c" >"$LOGS/pow_ucrt_$c.bin"
      say "  $(wc -c <"$LOGS/pow_ucrt_$c.bin") bytes"
    done
  fi
fi
say ""

# --------------------------------------------------------------- glibc side
say "-- reference oracle: glibc pow (WSL guest '$DISTRO') --"
if [ "$DISTRO" = "-" ]; then
  say "SKIP: glibc oracle disabled (SUNDIALS_WSL_DISTRO=-)"
elif ! command -v wsl.exe >/dev/null 2>&1; then
  say "SKIP: wsl.exe not on PATH"
else
  # Copy the oracle source into the guest, build with the guest cc, run there,
  # and write the bit-stream back onto the Windows filesystem via /mnt/c.
  GUEST_DIR="/tmp/sundials_pow_oracle"
  WIN_LOGS_GUEST="$(printf '%s' "$LOGS" | sed -e 's|^/\([a-zA-Z]\)/|/mnt/\1/|')"
  wsl.exe -d "$DISTRO" -- bash -lc "
    set -e
    mkdir -p $GUEST_DIR
    cp '$WIN_LOGS_GUEST/../tools/pow_oracle.c' $GUEST_DIR/
    cc -O2 -o $GUEST_DIR/pow_oracle $GUEST_DIR/pow_oracle.c -lm
    echo \"guest: \$(uname -srm) / \$(ldd --version | head -1) / \$(cc --version | head -1)\"
  " 2>&1 | tee -a "$LOG"
  for c in domain random; do
    case "$WHICH" in all) ;; "$c") ;; *) continue ;; esac
    say "generating glibc $c corpus ..."
    wsl.exe -d "$DISTRO" -- bash -lc \
      "$GUEST_DIR/pow_oracle $c > '$WIN_LOGS_GUEST/pow_glibc_$c.bin'" \
      2>&1 | tee -a "$LOG"
    say "  $(wc -c <"$LOGS/pow_glibc_$c.bin" 2>/dev/null || echo 0) bytes"
  done
fi
say ""

# ------------------------------------------------------------------ measure
say "-- differential: deterministic pow (native Windows build) vs oracles --"
export SUNDIALS_POW_ORACLE_DOMAIN="$LOGS/pow_glibc_domain.bin"
export SUNDIALS_POW_ORACLE_RANDOM="$LOGS/pow_glibc_random.bin"
export SUNDIALS_POW_ORACLE_UCRT="$LOGS/pow_ucrt_domain.bin"
[ -f "$SUNDIALS_POW_ORACLE_DOMAIN" ] || unset SUNDIALS_POW_ORACLE_DOMAIN
[ -f "$SUNDIALS_POW_ORACLE_RANDOM" ] || unset SUNDIALS_POW_ORACLE_RANDOM
[ -f "$SUNDIALS_POW_ORACLE_UCRT" ]   || unset SUNDIALS_POW_ORACLE_UCRT

cargo test --release -p sundials_core --lib pow_ -- --nocapture --test-threads=1 \
  2>&1 | tee -a "$LOG"

say ""
say "Done. Read logs/pow_differential_win.log"
