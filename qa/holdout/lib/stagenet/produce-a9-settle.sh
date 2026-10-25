#!/usr/bin/env bash
# Evaluator-owned A9 settle evidence run (s-c, s-d, s-e iii on D2; s-e iv on D3). Do NOT wrap in doctorwatson heavy:
# the producer takes heavy.lock itself only for the five settle proofs (ACQUIRE/RELEASE on #sherlock) and posts
# WALLET BUSY/FREE around the wallet phase. Check that no other agent holds the wallet (#sherlock) before running.
#   qa/holdout/lib/stagenet/produce-a9-settle.sh            # writes target/sherlock-a9/settle-run-*/{receipt.json,settle.log}
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
parent="${MORIARTY_A3_PARENT_DIR:-$(python3 -c "
from pathlib import Path
p = list(Path('.').glob('target/sherlock-a3/run-*/artifacts/parent'))
assert p, 'A9 settle evidence requires the A3 parent artifacts'
print(max(p, key=lambda x: x.stat().st_mtime_ns).resolve())")}"
cargo build --release --locked -p mori-recursion --features sherlock-ledger-validation --example validate_call
mkdir -p target/sherlock-a9
run_dir="$(mktemp -d "$PWD/target/sherlock-a9/settle-run-XXXXXXXX")"
node qa/holdout/lib/stagenet/produce-a9-settle.mjs "${MORIARTY_SDK_ROOT:-$HOME/.midnight/stagenet-wallet}" "$parent" "$run_dir" --submit \
  2>&1 | tee "$run_dir/settle.log"
exit "${PIPESTATUS[0]}"
