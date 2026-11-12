#!/bin/bash
# I0: build, native tests, smoke (MockProver, keygen, blinded proof), ledger check.
set -euxo pipefail
W=$HOME/Projects/Moriarty-wt/e5-wrap/tools/sherlock-wrap
cd $W
cargo test --release --offline --test i0_native 2>&1 | tail -30
python3 -I scripts/lock-check.py Cargo.lock
cargo build --release --offline --bin smoke 2>&1 | tail -5
$CARGO_TARGET_DIR/release/smoke $HOME/.cache/e5-wrap/smoke-k${K:-15} ${K:-15} 1 ${POW:-12}
cd ledger-check
cargo build --release --offline 2>&1 | tail -5
python3 -I ../scripts/lock-check.py Cargo.lock
$CARGO_TARGET_DIR/release/sherlock-wrap-ledger-check $HOME/.cache/e5-wrap/smoke-k${K:-15}
cd $HOME/Projects/Moriarty-wt/e5-wrap/tools/sherlock-wrap
cargo test --release --offline --test i1_transcript --test i2_merkle -- --include-ignored --nocapture --test-threads=1 2>&1 | grep -v Compiling | tail -40
