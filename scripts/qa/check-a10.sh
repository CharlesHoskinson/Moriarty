#!/usr/bin/env bash
# A10: public, submitter-independent readback of the stagenet acceptValidity call. Read-only.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-4}"
statement="$(python3 - <<'PY'
from pathlib import Path
files = list(Path('.').glob('target/sherlock-a4/run-*/artifacts/terminal.public.json'))
if not files:
    raise SystemExit('A10 requires the public A4 statement: target/sherlock-a4/run-*/artifacts/terminal.public.json')
print(max(files, key=lambda p: p.stat().st_mtime_ns).resolve())
PY
)"
cargo build --release --locked -p mori-recursion --features sherlock-ledger-validation --example validate_call
mkdir -p target/sherlock-a10
run_dir="$(mktemp -d "$PWD/target/sherlock-a10/run-XXXXXXXX")"
exec node tools/sherlock-sdk/readback-call.mjs "${MORIARTY_SDK_ROOT:-$HOME/.midnight/stagenet-wallet}" \
  "${MORIARTY_ANCHOR_BUILD:-$PWD/target/a4-anchor/deploy-run3/build}" "$statement" "$run_dir"
