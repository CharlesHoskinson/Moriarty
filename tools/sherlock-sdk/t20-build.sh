#!/bin/bash
# T20: build the batched SherlockLedger (with keys: the consumer test installs the compiled keys of the read
# entries, as a deployment would), the example claim consumer and its controls (--skip-zk), into OUTDIR.
# Usage: t20-build.sh [OUTDIR]   (default ~/.cache/t20/build; never the T9/T10/T17 build dirs)
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
OUT=${1:-$HOME/.cache/t20/build}
COMPACTC=${COMPACTC:-$HOME/.local/bin/compactc}
"$COMPACTC" --version | grep -q '^0.36.0 ' || { echo "compactc 0.36.0 required" >&2; exit 1; }
mkdir -p "$OUT"
"$COMPACTC" "$HERE/sherlock-ledger.compact" "$OUT/sherlock-ledger"
"$COMPACTC" --skip-zk "$HERE/sherlock-claim-consumer.compact" "$OUT/claim-consumer"
for c in "$HERE"/controls/consumer-control-*.compact; do
  "$COMPACTC" --skip-zk "$c" "$OUT/$(basename "$c" .compact)"
done
sha256sum "$HERE/sherlock-ledger.compact" "$HERE/sherlock-claim-consumer.compact" "$HERE"/controls/consumer-control-*.compact > "$OUT/sources.sha256"
echo "built into $OUT"
