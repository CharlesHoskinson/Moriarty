#!/usr/bin/env bash
# Offline preparation only. Run with doctorwatson heavy; this never submits a transaction.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-4}"
export RAYON_NUM_THREADS="${RAYON_NUM_THREADS:-4}"
export MORIARTY_PARENT_SRS="${MORIARTY_PARENT_SRS:-$PWD/target/sherlock-srs/bls_midnight_2p20}"
python3 scripts/qa/fetch-sherlock-srs.py "$MORIARTY_PARENT_SRS"
if [[ -z "${MORIARTY_A3_PARENT_DIR:-}" ]]; then
  MORIARTY_A3_PARENT_DIR="$(python3 - <<'PY'
from pathlib import Path
files = list(Path('target/sherlock-a3').glob('run-*/artifacts/parent/w.proof'))
if not files:
    raise SystemExit('terminal preparation requires public A3 artifacts')
print(max(files, key=lambda p: p.stat().st_mtime_ns).parent.resolve())
PY
)"
fi
export MORIARTY_A3_PARENT_DIR
mkdir -p target/sherlock-a4
run_dir="$(mktemp -d "$PWD/target/sherlock-a4/prepare-XXXXXXXX")"
export MORIARTY_TERMINAL_OUTPUT_DIR="$run_dir/artifacts"
exec cargo test --release --locked -p mori-recursion --features sherlock-ledger \
  --test sherlock_terminal prepare_terminal_proof_for_deployment \
  -- --exact --include-ignored --nocapture --test-threads=1
