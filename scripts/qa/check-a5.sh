#!/usr/bin/env bash
# Verify retained public A2-A4 files in a separate, read-only stock-verifier process.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-4}"
export RAYON_NUM_THREADS="${RAYON_NUM_THREADS:-4}"
mapfile -t artifacts < <(python3 - <<'PY'
from pathlib import Path
for pattern in ['target/sherlock-a2/run-*/children/link.json',
                'target/sherlock-a3/run-*/artifacts/parent/w.proof',
                'target/sherlock-a4/run-*/artifacts/terminal.proof']:
    files = list(Path('.').glob(pattern))
    if not files:
        raise SystemExit('A5 requires completed public artifacts: ' + pattern)
    print(max(files, key=lambda p: p.stat().st_mtime_ns).parent.resolve())
PY
)
[[ ${#artifacts[@]} == 3 ]] || exit 1
mkdir -p target/sherlock-a5
run_dir="$(mktemp -d "$PWD/target/sherlock-a5/run-XXXXXXXX")"
python3 scripts/qa/fetch-sherlock-contract.py "$run_dir/contract-state.bin"
exec cargo run --release --locked -p mori-recursion --features sherlock-ledger \
  --example verify_chain -- "${artifacts[@]}" "$run_dir/contract-state.bin" \
  "${MORIARTY_PARENT_SRS:-$PWD/target/sherlock-srs/bls_midnight_2p20}" \
  crates/mori-recursion/tests/fixtures/stagenet-verifier.json
