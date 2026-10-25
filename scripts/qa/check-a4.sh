#!/usr/bin/env bash
# A4: terminal proof over A3, accepted under the exact installed stagenet key.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-4}"
export RAYON_NUM_THREADS="${RAYON_NUM_THREADS:-4}"
export MORIARTY_PARENT_SRS="${MORIARTY_PARENT_SRS:-$PWD/target/sherlock-srs/bls_midnight_2p20}"
python3 scripts/qa/fetch-sherlock-srs.py "$MORIARTY_PARENT_SRS"
export MORIARTY_A3_PARENT_DIR="$(python3 - <<'PY'
from pathlib import Path
files = list(Path('target/sherlock-a3').glob('run-*/artifacts/parent/w.proof'))
if not files:
    raise SystemExit('A4 requires the public artifacts from scripts/doctorwatson check A3')
print(max(files, key=lambda p: p.stat().st_mtime_ns).parent.resolve())
PY
)"
mkdir -p target/sherlock-a4
run_dir="$(mktemp -d "$PWD/target/sherlock-a4/run-XXXXXXXX")"
export MORIARTY_DEPLOYED_CONTRACT_STATE="$run_dir/contract-state.bin"
python3 scripts/qa/fetch-sherlock-contract.py "$MORIARTY_DEPLOYED_CONTRACT_STATE"
export MORIARTY_TERMINAL_OUTPUT_DIR="$run_dir/artifacts"
exec cargo test --release --locked -p mori-recursion --features sherlock-ledger \
  --test sherlock_terminal -- --include-ignored --nocapture --test-threads=1
