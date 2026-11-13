#!/bin/bash
set -euxo pipefail
cd $HOME/Projects/Moriarty-wt/e5-wrap/tools/sherlock-wrap
cargo test --release --offline --test i5_whir -- --ignored --nocapture --test-threads=1 i5_lfr_real_native 2>&1 | grep -v Compiling | tail -30
