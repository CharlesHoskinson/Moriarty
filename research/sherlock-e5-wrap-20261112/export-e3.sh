#!/bin/bash
# Build lfr_e3 (openvm e5-wrap on E3 2f09caf); decode cases first (no prove), then the all-entries L_Fr proof.
set -euxo pipefail
cd $HOME/.cache/m7-wrap/src/openvm-e5
cp Cargo.lock $HOME/.cache/e5-wrap/openvm-lock.before
cargo build --release --offline -p sherlock-wrap-export --example lfr_e3 2>&1 | grep -v Compiling | tail -20
diff <(grep -A1 '^name = ' $HOME/.cache/e5-wrap/openvm-lock.before) <(grep -A1 '^name = ' Cargo.lock) || true
$CARGO_TARGET_DIR/release/examples/lfr_e3 0
RAYON_NUM_THREADS=20 $CARGO_TARGET_DIR/release/examples/lfr_e3 1
ls -la $HOME/.cache/e5-wrap/fixtures
