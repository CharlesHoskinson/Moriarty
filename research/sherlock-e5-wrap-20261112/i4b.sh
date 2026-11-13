#!/bin/bash
set -euxo pipefail
cd $HOME/Projects/Moriarty-wt/e5-wrap/tools/sherlock-wrap
cargo test --release --offline --test i3_part1 --test i4_part2 --test i4b_stacked -- --include-ignored --nocapture --test-threads=1 2>&1 | grep -v Compiling | tail -80
python3 -I scripts/lock-check.py Cargo.lock
