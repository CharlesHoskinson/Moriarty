#!/bin/bash
# I9: the ENC wrap-side vectors (Fr vectors on the wrap gadgets; pv binding and lane map through the decoder), and I7 again after the DecodeRelation move.
set -euxo pipefail
cd $HOME/Projects/Moriarty-wt/tested/tools/sherlock-wrap
cargo test --release --offline --test i9_enc --test i7_decode -- --nocapture --test-threads 1 2>&1 | grep -v Compiling
