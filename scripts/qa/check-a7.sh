#!/usr/bin/env bash
# Prepare genuine public A3 artifacts, then run the unchanged evaluator-owned suite.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-4}"
export RAYON_NUM_THREADS="${RAYON_NUM_THREADS:-4}"
cargo build --release --locked -p mori-recursion --features sherlock-recursion --bin sherlock-verify-parent
mkdir -p target/sherlock-a7
run_dir="$(mktemp -d "$PWD/target/sherlock-a7/run-XXXXXXXX")"
python3 - "$run_dir" <<'PY'
from pathlib import Path
import sys
out = Path(sys.argv[1])
for pattern, name in [('target/sherlock-a3/run-*/artifacts/parent/w.vk', 'vk.bin'),
                      ('target/sherlock-a3/run-*/artifacts/parent/w.proof', 'proof.bin'),
                      ('target/sherlock-a3/run-*/artifacts/parent/w.instance.json', 'instance.json'),
                      ('target/sherlock-a2/run-*/children/a102.vk', 'wrong-valid-child-vk.bin')]:
    paths = list(Path('.').glob(pattern))
    if not paths:
        raise SystemExit('A7 requires genuine public artifacts: ' + pattern)
    (out / name).symlink_to(max(paths, key=lambda p: p.stat().st_mtime_ns).resolve())
PY
export DW_PARENT_FIXTURE_DIR="${DW_PARENT_FIXTURE_DIR:-$run_dir}"
export DW_PARENT_VERIFIER="${DW_PARENT_VERIFIER:-$PWD/target/release/sherlock-verify-parent}"
export DW_SRS14="${DW_SRS14:-${MORIARTY_TEST_PARAMS_DIR:-$HOME/.cache/midnight/zk-params}/bls_midnight_2p14}"
exec qa/holdout/run.sh rejections-local
