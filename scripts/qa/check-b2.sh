#!/usr/bin/env bash
# B2: separate Rust oracle replays the exact B1 corpus and rejects semantic mutations.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-4}"
export FENRIR_ROOT="${FENRIR_ROOT:-$HOME/repos/projectfenrir-main}"
export FRTF_V3_ROOT="${FRTF_V3_ROOT:-$FENRIR_ROOT/recursion/rollup-test-format/v3}"
cut="${FSYN_DESIGN_CUT:-$PWD/openspec/changes/sherlock-synthetic-rollup-data/design-cut.json}"
if [[ -z "${FSYN_SOURCE_MAP:-}" ]]; then
  FSYN_SOURCE_MAP="$(python3 -B models/sherlock/rollup-synth/sources.py \
    --cut "$cut" --fenrir-root "$FENRIR_ROOT" --cache "$PWD/target/fsyn-sources")"
fi
export FSYN_SOURCE_MAP
python3 -B -m unittest discover -s scripts/qa/tests -p test_fsyn_replay.py
manifest="$(python3 -B models/sherlock/rollup-synth/replay.py \
  --pins models/sherlock/rollup-synth/replay-cut.json \
  --fenrir-root "$FENRIR_ROOT" --out "$PWD/target/sherlock-b2/source")"
export CARGO_TARGET_DIR="$PWD/target/sherlock-b2/build"
cargo build --release --locked --manifest-path "$manifest"
run_dir="$(mktemp -d "$PWD/target/sherlock-b2/run-XXXXXXXX")"
python3 -B scripts/qa/verify-fsyn-replay.py --binary "$CARGO_TARGET_DIR/release/rtf" \
  --cut "$cut" --golden-root "$FRTF_V3_ROOT" --out "$run_dir/replay"
