#!/bin/bash
# Build the T10 SherlockLedger (sherlock-ledger-t10.compact, the contract T10 and T17 measured; the batched
# T20 form is sherlock-ledger.compact, built by t20-build.sh) (with prover/verifier keys, deployable: the T10a runner reads keys/ and zkir/) and the
# test controls (--skip-zk) with compactc 0.36.0, then write the zkir lane map of every entry.
# Usage: sherlock-ledger-build.sh [OUTDIR]   (default ~/.cache/t9/contract/build)
#   OUTDIR/sherlock-ledger     tools/sherlock-sdk/sherlock-ledger-t10.compact (keys)
#   OUTDIR/control-two-maps    controls/ledger-control-two-maps.compact (--skip-zk)
#   OUTDIR/control-shape       controls/ledger-control-shape.compact (--skip-zk)
#   OUTDIR/sherlock-run        sherlock-run.compact (--skip-zk; the privacy-scan control, file untouched)
#   OUTDIR/lanes-zkir.json     lanes.py over OUTDIR/sherlock-ledger/zkir/*.zkir
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
REPO=$(cd "$HERE/../.." && pwd)
OUT=${1:-$HOME/.cache/t9/contract/build}
COMPACTC=${COMPACTC:-$HOME/.local/bin/compactc}
"$COMPACTC" --version | grep -q '^0.36.0 ' || { echo "compactc 0.36.0 required" >&2; exit 1; }
mkdir -p "$OUT"
"$COMPACTC" "$HERE/sherlock-ledger-t10.compact" "$OUT/sherlock-ledger"
"$COMPACTC" --skip-zk "$HERE/controls/ledger-control-two-maps.compact" "$OUT/control-two-maps"
"$COMPACTC" --skip-zk "$HERE/controls/ledger-control-shape.compact" "$OUT/control-shape"
"$COMPACTC" --skip-zk "$HERE/sherlock-run.compact" "$OUT/sherlock-run"
python3 "$REPO/research/sherlock-enc-fp3-20261112/fp3-closure/lanes.py" "$OUT"/sherlock-ledger/zkir/*.zkir > "$OUT/lanes-zkir.json"
sha256sum "$HERE/sherlock-ledger-t10.compact" "$HERE"/controls/*.compact "$HERE/sherlock-run.compact" > "$OUT/sources.sha256"
echo "built into $OUT"
