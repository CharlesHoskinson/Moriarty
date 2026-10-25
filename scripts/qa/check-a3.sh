#!/usr/bin/env bash
# A3: prove the two-child recursive parent at k20 using the exact public A2 bundle.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-4}"
export RAYON_NUM_THREADS="${RAYON_NUM_THREADS:-4}"
export MORIARTY_TEST_PARAMS_DIR="${MORIARTY_TEST_PARAMS_DIR:-$HOME/.cache/midnight/zk-params}"
if [[ -z "${MORIARTY_A2_LINK_DIR:-}" ]]; then
  MORIARTY_A2_LINK_DIR="$(python3 - <<'PY'
from pathlib import Path
files = list(Path('target/sherlock-a2').glob('run-*/children/link.json'))
if not files:
    raise SystemExit('A3 requires the public artifacts from scripts/doctorwatson check A2')
print(max(files, key=lambda p: p.stat().st_mtime_ns).parent.resolve())
PY
)"
fi
export MORIARTY_A2_LINK_DIR
export MORIARTY_PARENT_SRS="${MORIARTY_PARENT_SRS:-$PWD/target/sherlock-srs/bls_midnight_2p20}"
python3 scripts/qa/fetch-sherlock-srs.py "$MORIARTY_PARENT_SRS"
mkdir -p target/sherlock-a3
run_dir="$(mktemp -d "$PWD/target/sherlock-a3/run-XXXXXXXX")"
export MORIARTY_PARENT_OUTPUT_DIR="$run_dir/artifacts"
exec cargo test --release --locked -p mori-recursion --features sherlock-recursion \
  --test sherlock_parent -- --include-ignored --nocapture --test-threads=1
