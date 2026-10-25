#!/usr/bin/env bash
# Evaluator-owned A9 evidence run (opens the stagenet wallet, submits). Run only through the heavy slot:
#   DW_ROLE=evaluator scripts/doctorwatson heavy --id A9produce -- qa/holdout/lib/stagenet/produce-a9.sh
# Writes target/sherlock-a9/run-*/receipt.json, which `qa/holdout/run.sh rejections-stagenet` checks read-only.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
newest() { python3 - "$1" <<'PY'
from pathlib import Path
import sys
paths = list(Path('.').glob(sys.argv[1]))
if not paths:
    raise SystemExit('A9 evidence requires ' + sys.argv[1])
print(max(paths, key=lambda p: p.stat().st_mtime_ns).resolve())
PY
}
parent="${MORIARTY_A3_PARENT_DIR:-$(newest 'target/sherlock-a3/run-*/artifacts/parent')}"
key="${MORIARTY_TERMINAL_KEY_DIR:-$(newest 'target/sherlock-a4/run-*/artifacts/key')}"
cargo build --release --locked -p mori-recursion --features sherlock-ledger-validation --example validate_call
mkdir -p target/sherlock-a9
run_dir="$(mktemp -d "$PWD/target/sherlock-a9/run-XXXXXXXX")"
exec node qa/holdout/lib/stagenet/produce-a9.mjs "${MORIARTY_SDK_ROOT:-$HOME/.midnight/stagenet-wallet}" \
  "${MORIARTY_ANCHOR_BUILD:-$PWD/target/a4-anchor/deploy-run3/build}" "${MORIARTY_ANCHOR_KEYED:-$PWD/target/a4-anchor/keyed}" \
  "$parent" "$key" "$run_dir" --submit
