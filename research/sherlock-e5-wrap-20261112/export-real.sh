#!/bin/bash
# Build the lfr_real example (openvm worktree e5-wrap, on top of E2 c4bd312) and write lfr-real.json.
set -euxo pipefail
cd $HOME/.cache/m7-wrap/src/openvm-tested
cp Cargo.lock $HOME/.cache/e5-wrap/openvm-lock.before
cargo build --release --offline -p sherlock-wrap-export --example lfr_real 2>&1 | grep -v Compiling | tail -20
diff <(grep -A1 '^name = ' $HOME/.cache/e5-wrap/openvm-lock.before) <(grep -A1 '^name = ' Cargo.lock) || true
RAYON_NUM_THREADS=16 $CARGO_TARGET_DIR/release/examples/lfr_real
ls -la $HOME/.cache/e5-wrap/fixtures
