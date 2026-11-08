#!/usr/bin/env bash
# B2: the in-repository I1 oracle replays the exact B1 FSYN fixture and rejects
# semantic mutations (SSRD-015, SSRD-030). Reads only tracked repository files.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
unset FRTF_V3_ROOT FSYN_SOURCE_MAP FSYN_DESIGN_CUT FSYN_ARCHIVE_ROOT
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-4}"
cargo build --release --locked -p mori-recursion --features sherlock-rollup --example i1_replay
binary="${CARGO_TARGET_DIR:-$PWD/target}/release/examples/i1_replay"
mkdir -p target/sherlock-b2
run_dir="$(mktemp -d "$PWD/target/sherlock-b2/run-XXXXXXXX")"
python3 -B scripts/qa/verify-fsyn-replay.py --binary "$binary" --out "$run_dir/replay"
