#!/bin/bash
# Build the exporter (openvm worktree e5-wrap) and write the fixtures; then I1 tests.
set -euxo pipefail
cd $HOME/.cache/m7-wrap/src/openvm-tested
cp Cargo.lock $HOME/.cache/e5-wrap/openvm-lock.before
cargo build --release --offline -p sherlock-wrap-export --example fixtures 2>&1 | tail -5
diff <(grep -A1 '^name = ' $HOME/.cache/e5-wrap/openvm-lock.before) <(grep -A1 '^name = ' Cargo.lock) || true
RAYON_NUM_THREADS=16 $CARGO_TARGET_DIR/release/examples/fixtures ${WHICH:-all}
ls -la $HOME/.cache/e5-wrap/fixtures
