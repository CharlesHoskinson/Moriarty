#!/bin/bash
# I6: the full wrap of one fixture: cost model, MockProver, keygen, blinded proof, then the stock ledger check.
# usage: i6.sh <fixture> <k> [mock 0|1]
set -euxo pipefail
F=$1; K=$2; MOCK=${3:-1}
OUT=$HOME/.cache/e5-wrap/out/$F-k$K
cd $HOME/Projects/Moriarty-wt/tested/tools/sherlock-wrap
cargo build --release --offline --bin wrap 2>&1 | grep -v Compiling | tail -20
time $CARGO_TARGET_DIR/release/wrap $F $OUT $K $MOCK
cd ledger-check
cargo build --release --offline 2>&1 | tail -3
$CARGO_TARGET_DIR/release/sherlock-wrap-ledger-check $OUT
