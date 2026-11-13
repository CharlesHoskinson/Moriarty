#!/bin/bash
# Build and run enc_wrap: the ENC pv vector through the stock D-9 decoder (fixture enc-wrap-decode.json).
set -euxo pipefail
cd $HOME/.cache/m7-wrap/src/openvm-e5
cp Cargo.lock $HOME/.cache/e5-wrap/openvm-lock.before
cargo build --release --offline -p sherlock-wrap-export --example enc_wrap 2>&1 | grep -v Compiling | tail -30
diff $HOME/.cache/e5-wrap/openvm-lock.before Cargo.lock
$CARGO_TARGET_DIR/release/examples/enc_wrap
