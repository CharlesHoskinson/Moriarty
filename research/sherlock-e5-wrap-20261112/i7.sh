#!/bin/bash
# I7: the E3 map (D-9 decoder): unit costs, native verdicts equal stock on every exported case, gadget decoder rows and MockProver.
set -euxo pipefail
cd $HOME/Projects/Moriarty-wt/tested/tools/sherlock-wrap
cargo test --release --offline --test i7_unit_cost --test i7_decode -- --nocapture --test-threads 1 2>&1 | grep -v Compiling
