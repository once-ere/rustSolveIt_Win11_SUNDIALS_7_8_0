#!/usr/bin/env bash
# win_verify_physics.sh — the Windows 11 byte-identity gate for the physics.
#
# Re-runs, on Windows, exactly the three suites whose Linux outputs are
# recorded in evidence/port-7.8.0/, in the same concatenated-log formats,
# and diffs the results:
#
#   examples-7.8.0.log            6 self-checking physics examples
#   collision-scripts-7.8.0.log   12 collision scripts
#   dynamic-notebooks-7.8.0.log   59 dynamic notebooks
#
# Normalisations (the same ones the originals carry):
#   * the OS-assigned scene port  ->  <port>
#   * the current directory       ->  <cwd>
# Line endings are compared with --strip-trailing-cr because the vendored
# evidence files were checked out through git eol conversion.
#
# Run from the repository root, in Git Bash:
#   bash tools/win_verify_physics.sh
#
# Writes evidence/win11/{examples,collision-scripts,dynamic-notebooks}-win11.log
# and prints IDENTICAL or DIFFERS per suite. Exit 0 only if all three match.
set -u
cd "$(dirname "$0")/.."
ROOT="$PWD"
BIN="$ROOT/target/release/posim.exe"
OUT="$ROOT/evidence/win11"
REF="$ROOT/evidence/port-7.8.0"
mkdir -p "$OUT"
export POSIM_NO_BROWSER=1

[ -x "$BIN" ] || { echo "build first: cargo build --release -p posim" >&2; exit 2; }

normalize () {
    # scene ports vary per run; cwd is machine-specific
    sed -e 's#127\.0\.0\.1:[0-9][0-9]*#127.0.0.1:<port>#g' \
        -e "s#$(printf '%s' "$ROOT" | sed 's/[.[\*^$/]/\\&/g')#<cwd>#g"
}

fail=0

# The reference logs were concatenated on a glibc host whose locale
# collation ordered the section globs; rather than imitate that locale,
# take the section order straight out of each reference log.
sections () { grep '^#####* ' "$1" | tr -d '\r' | sed 's/^#* //'; }

# ---- 1. the six examples --------------------------------------------------
: > "$OUT/examples-win11.log"
sections "$REF/examples-7.8.0.log" | while read -r ex; do
    echo "############ $ex" >> "$OUT/examples-win11.log"
    "$ROOT/target/release/examples/$ex.exe" >> "$OUT/examples-win11.log" 2>&1
    echo "exit=$?" >> "$OUT/examples-win11.log"
done

# ---- 2. the twelve collision scripts -------------------------------------
: > "$OUT/collision-scripts-win11.log"
for f in scripts/collisions/*.posim; do
    echo "##### $f" >> "$OUT/collision-scripts-win11.log"
    "$BIN" --script "$f" >> "$OUT/collision-scripts-win11.log" 2>&1 \
        || { echo "script $f FAILED" >&2; fail=1; }
done

# ---- 3. the fifty-nine dynamic notebooks ----------------------------------
{ echo "<cwd>"; } > "$OUT/dynamic-notebooks-win11.log"
sections "$REF/dynamic-notebooks-7.8.0.log" | while read -r f; do
    echo "##### $f" >> "$OUT/dynamic-notebooks-win11.log"
    "$BIN" --script "$f" 2>&1 | normalize >> "$OUT/dynamic-notebooks-win11.log"
    echo "rc=${PIPESTATUS[0]}" >> "$OUT/dynamic-notebooks-win11.log"
done

# ---- compare --------------------------------------------------------------
# Two documented divergences from the Linux evidence are ACCEPTED, pinned
# byte-for-byte in evidence/win11/accepted-divergences-*.diff:
#
#   * examples: one inserted blank line after the outer_solar_system
#     header. The donor added a leading "\n" to that example's first
#     println! AFTER its evidence log was recorded — the donor's own
#     current source cannot reproduce its evidence on Linux either.
#     Every number is identical.
#   * dynamic notebooks: 9 lines, all in double_slit and tunneling —
#     the QUANTUM notebooks — differing in the LAST printed digit
#     (norm drifts at 1e-13, one 16th significant digit). The quantum
#     crate calls the host libm (UCRT here, glibc there); the physics
#     engine itself routes through the vendored deterministic libm and
#     is byte-identical, as the other 57 notebooks and all 12 collision
#     scripts show.
#
# Anything beyond those pinned diffs is a regression and fails the gate.
for pair in "examples-win11.log:examples-7.8.0.log:accepted-divergences-examples.diff" \
            "collision-scripts-win11.log:collision-scripts-7.8.0.log:" \
            "dynamic-notebooks-win11.log:dynamic-notebooks-7.8.0.log:accepted-divergences-dynamic.diff"; do
    IFS=: read -r got want accepted <<< "$pair"
    d="$(diff --strip-trailing-cr "$REF/$want" "$OUT/$got")"
    if [ -z "$d" ]; then
        echo "IDENTICAL  $got == $want"
    elif [ -n "$accepted" ] && [ "$d" = "$(cat "$OUT/$accepted" | tr -d '\r')" ]; then
        echo "IDENTICAL-MODULO-DOCUMENTED  $got vs $want (pinned: $accepted)"
    else
        echo "DIFFERS    $got vs $want (beyond the pinned divergences)"
        printf '%s\n' "$d" | head -20
        fail=1
    fi
done
exit $fail
