#!/bin/bash
# I9: build and run the whole-verifier tape test on the real L proof (a systemd user unit: over 30 minutes).
set -uo pipefail
R=$HOME/.cache/e2-lfr/run.sh; W=$HOME/.cache/m7-wrap/src/openvm-tested
$R i9-clippy 32G $W cargo clippy --release --locked --message-format short -p sherlock-lfr --all-targets -- -D warnings || exit 1
$R i9-test 56G $W env RAYON_NUM_THREADS=20 cargo test --release --locked -p sherlock-lfr --test i9_whir -- --nocapture --include-ignored
