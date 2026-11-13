#!/bin/bash
# I0: build the sherlock-lfr fixture example and test, make the L fixture, check it from disk.
set -uo pipefail
R=$HOME/.cache/e2-lfr/run.sh; W=$HOME/.cache/m7-wrap/src/openvm-tested
export RUSTFLAGS="-C target-cpu=native"
$R i0-build 48G $W cargo build --release --locked --message-format short -p sherlock-lfr --example l_fixture --tests || exit 1
if [ ! -s $HOME/.cache/e2-lfr/fixture/l_fixture.json ]; then
  $R i0-run 60G $W env RAYON_NUM_THREADS=20 $HOME/.cache/e2-lfr/target/release/examples/l_fixture || exit 1
fi
$R i0-test 24G $W cargo test --release --locked -p sherlock-lfr --test i0_fixture -- --nocapture --include-ignored
