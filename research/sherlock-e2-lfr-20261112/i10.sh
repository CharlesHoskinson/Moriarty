#!/bin/bash
# I10, the E2 gate: the whole L verifier tape at the L_Fr parameter set (a systemd user unit),
# then every other sherlock-lfr test on the same tree.
set -uo pipefail
export RUSTUP_AUTO_INSTALL=0
R=$HOME/.cache/e2-lfr/run.sh; W=$HOME/.cache/m7-wrap/src/openvm-org_openvm
$R i10-gate 58G $W env RAYON_NUM_THREADS=20 cargo test --release --locked -p sherlock-lfr --test i10_gate -- --nocapture --include-ignored i10_gate_at_lfr_params || exit 1
$R i10-final 56G $W env RAYON_NUM_THREADS=20 cargo test --release --locked -p sherlock-lfr -- --include-ignored --test-threads=1 --skip i10_gate_at_lfr_params
