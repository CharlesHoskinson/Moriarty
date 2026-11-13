#!/bin/bash
set -euxo pipefail
cd $HOME/Projects/Moriarty-wt/tested/tools/sherlock-wrap
cargo test --release --offline --test i1_transcript --test i2_merkle --test i3_part1 -- --include-ignored --nocapture --test-threads=1 2>&1 | grep -v Compiling | tail -60
python3 -I scripts/lock-check.py Cargo.lock
