#!/bin/bash
# One lever: the I3 to I5 tests (lfr-real native included) and the lfr-real cost model at k = 21.
set -euxo pipefail
cd $HOME/Projects/Moriarty-wt/tested/tools/sherlock-wrap
cargo test --release --offline --test i3_part1 --test i4_part2 --test i4b_stacked --test i5_whir -- --include-ignored --nocapture --test-threads=1 2>&1 | grep -E 'E5I|test result|panicked|error|FAILED' | tail -40
cargo build --release --offline --bin wrap 2>&1 | grep -v Compiling | tail -3
$CARGO_TARGET_DIR/release/wrap lfr-real $HOME/.cache/e5-wrap/out/lever $K cost
