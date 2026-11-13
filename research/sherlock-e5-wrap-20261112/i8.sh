#!/bin/bash
# I8: the wrap on the E3 lfr-e3 fixture: native all-samples check, then the k = 21 cost model of the full wrap (E3 map).
set -euxo pipefail
cd $HOME/Projects/Moriarty-wt/tested/tools/sherlock-wrap
cargo test --release --offline --test i5_whir -- --ignored --nocapture --test-threads=1 i5_lfr_e3_native 2>&1 | grep -v Compiling | tail -30
cargo build --release --offline --bin wrap 2>&1 | grep -v Compiling | tail -5
$CARGO_TARGET_DIR/release/wrap lfr-e3 $HOME/.cache/e5-wrap/out/lfr-e3-cost 21 cost || true
