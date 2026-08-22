#!/usr/bin/env bash
# libm_fingerprint_win.sh
#
# Build tools/libm_probe.rs twice — natively on Windows and inside a WSL2
# Linux guest — and diff the two fingerprints. The same Rust source, the
# same corpus, the same rustc family: the only thing that varies is the
# libm behind `f64`'s unspecified-precision methods. Whatever differs is
# exactly what stands between this port and byte-identical output against
# the upstream reference `.out` files, which were generated on glibc.
#
# Writes logs/libm_fingerprint.txt.
#
# Environment:
#   SUNDIALS_WSL_DISTRO   guest to compare against (default Ubuntu-24.04)
#
# SPDX-License-Identifier: BSD-3-Clause

set -u
cd "$(dirname "$0")/.."
WS_ROOT="$(pwd)"
LOGS="$WS_ROOT/logs"
mkdir -p "$LOGS"
OUT="$LOGS/libm_fingerprint.txt"
DISTRO="${SUNDIALS_WSL_DISTRO:-Ubuntu-24.04}"

: >"$OUT"
{
  echo "== host libm fingerprint =="
  echo "date : $(date -u '+%Y-%m-%dT%H:%M:%SZ')"
  echo
} | tee -a "$OUT"

echo "-- Windows (x86_64-pc-windows-msvc, Microsoft UCRT) --" | tee -a "$OUT"
rustc -O "$WS_ROOT/tools/libm_probe.rs" -o "$LOGS/libm_probe.exe" 2>&1 | tee -a "$OUT"
"$LOGS/libm_probe.exe" >"$LOGS/fp_windows.txt" 2>&1
cat "$LOGS/fp_windows.txt" | tee -a "$OUT"
echo | tee -a "$OUT"

echo "-- WSL guest '$DISTRO' (x86_64-unknown-linux-gnu, glibc) --" | tee -a "$OUT"
GUEST_TOOLS="$(printf '%s' "$WS_ROOT/tools" | sed -e 's|^/\([a-zA-Z]\)/|/mnt/\1/|')"
wsl.exe -d "$DISTRO" -- bash -lc "
  set -e
  cp '$GUEST_TOOLS/libm_probe.rs' /tmp/libm_probe.rs
  rustc -O /tmp/libm_probe.rs -o /tmp/libm_probe
  echo \"# guest: \$(uname -srm) / \$(ldd --version | head -1) / \$(rustc -V)\"
  /tmp/libm_probe
" 2>/dev/null | tr -d '\r' | grep -E '^(#|[a-z]+ +[0-9a-f]{16})' >"$LOGS/fp_linux.txt"
cat "$LOGS/fp_linux.txt" | tee -a "$OUT"
echo | tee -a "$OUT"

echo "-- verdict: functions whose results differ between the two hosts --" | tee -a "$OUT"
join -j1 \
  <(grep -vE '^#' "$LOGS/fp_windows.txt" | sort) \
  <(grep -vE '^#' "$LOGS/fp_linux.txt" | sort) \
  | while read -r fn a b; do
      if [ "$a" = "$b" ]; then
        printf '%-8s same\n' "$fn"
      else
        printf '%-8s DIFFERS   windows=%s  glibc=%s\n' "$fn" "$a" "$b"
      fi
    done | tee -a "$OUT"

echo | tee -a "$OUT"
echo "Done. Read logs/libm_fingerprint.txt" | tee -a "$OUT"
