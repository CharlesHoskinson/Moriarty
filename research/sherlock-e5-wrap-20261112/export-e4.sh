#!/bin/bash
# Build the lfr_real and lfr_e4 examples; write lfr-real.json from the E4 CPU-baseline proof of the I10 tape (no new prove).
set -euxo pipefail
cd $HOME/.cache/m7-wrap/src/openvm-tested
cp Cargo.lock $HOME/.cache/e5-wrap/openvm-lock.before
cargo build --release --offline -p sherlock-wrap-export --example lfr_real --example lfr_e4 2>&1 | grep -v Compiling | tail -20
diff <(grep -A1 '^name = ' $HOME/.cache/e5-wrap/openvm-lock.before) <(grep -A1 '^name = ' Cargo.lock) || true
RAYON_NUM_THREADS=8 $CARGO_TARGET_DIR/release/examples/lfr_e4 $HOME/.cache/e4-gpu/pk.bitcode $HOME/.cache/e4-gpu/proofs/cpu-base1.bin
ls -la $HOME/.cache/e5-wrap/fixtures
