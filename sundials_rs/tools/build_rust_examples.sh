#!/usr/bin/env bash
# ===========================================================================
# build_rust_examples.sh — build the Rust port's examples, recording the
# same class of evidence the C build records.
#
#   tools/build_rust_examples.sh
#
# Writes into rust-results/provenance/ :
#   00-environment.txt   host, toolchain versions and paths, the effective
#                        rustflags and where they come from
#   01-build-cmd.txt     the literal cargo command line
#   02-build-out.txt     cargo -v output: every rustc invocation as executed
#   03-cargo-config.txt  .cargo/config.toml verbatim (it sets +fma)
#   04-Cargo.lock.txt    the resolved dependency set (7 workspace crates, no
#                        external dependencies)
#
# A full `cargo clean` is done first, because `cargo -v` only prints the rustc
# command lines for crates it actually compiles.
#
# SPDX-License-Identifier: BSD-3-Clause
# ===========================================================================

set -u
cd "$(dirname "$0")/.."
PROV="$PWD/rust-results/provenance"
mkdir -p "$PROV"
P="$PROV/00-environment.txt"

{
  echo "== how this file was produced =="
  echo "tools/build_rust_examples.sh"
  echo
  echo "== host =="
  uname -srm
  powershell -NoProfile -Command "(Get-CimInstance Win32_OperatingSystem).Caption" 2>/dev/null | tr -d '\r'
  powershell -NoProfile -Command "(Get-CimInstance Win32_Processor).Name" 2>/dev/null | tr -d '\r'
  echo
  echo "== build started (UTC) =="
  date -u '+%Y-%m-%dT%H:%M:%SZ'
  echo
  echo "== rustc =="
  command -v rustc
  rustc -vV
  echo
  echo "== cargo =="
  command -v cargo
  cargo -V
  echo
  echo "== effective rustflags =="
  echo "None are set in the environment; they come from .cargo/config.toml,"
  echo "reproduced in 03-cargo-config.txt. RUSTFLAGS env var is:"
  echo "RUSTFLAGS=${RUSTFLAGS:-<unset>}"
  echo
  echo "== release profile =="
  grep -A8 '\[profile.release\]' Cargo.toml 2>/dev/null || echo "(cargo defaults: opt-level=3, debug=false, overflow-checks=false, lto=false, codegen-units=16, panic=unwind)"
} > "$P"

cp .cargo/config.toml "$PROV/03-cargo-config.txt"
cp Cargo.lock "$PROV/04-Cargo.lock.txt"

echo 'cargo clean' > "$PROV/01-build-cmd.txt"
echo 'cargo build --release --workspace --examples -v' >> "$PROV/01-build-cmd.txt"

cargo clean
cargo build --release --workspace --examples -v > "$PROV/02-build-out.txt" 2>&1
rc=$?

{
  echo
  echo "== build finished (UTC) =="
  date -u '+%Y-%m-%dT%H:%M:%SZ'
  echo "cargo exit code: $rc"
  echo "warnings emitted: $(grep -c '^warning' "$PROV/02-build-out.txt")"
  echo "errors emitted:   $(grep -c '^error' "$PROV/02-build-out.txt")"
} >> "$P"

echo "cargo exit=$rc; provenance in rust-results/provenance/"
exit $rc
