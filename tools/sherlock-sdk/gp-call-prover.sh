#!/bin/sh
# Prover command for check-local-run.mjs --bundle and call-run.mjs (SHERLOCK_RUN_PROVER): proves one
# acceptRun call with the run decider of a GP bundle (sherlock_gp gp_call_prove) and prints the tagged
# proof as hex. stdin is the SDK opening {"binding_input", "preimage"}; it is passed straight to the test
# and never written. Holds heavy.lock for the prove only (Sherlock owner priority).
# Usage: gp-call-prover.sh GP_BUNDLE CALL_OUT_PARENT   (run from the repository root)
#   MORIARTY_PARENT_SRS  the pinned 2^20 SRS (default ~/repos/Moriarty-rollup/target/sherlock-srs/bls_midnight_2p20)
set -eu
bundle=$(realpath "$1"); parent=$(realpath "$2")
mkdir -p "$parent"
out=$(mktemp -u "$parent/call-XXXXXX")
srs=${MORIARTY_PARENT_SRS:-$HOME/repos/Moriarty-rollup/target/sherlock-srs/bls_midnight_2p20}
flock -w 3600 "$HOME/.cache/doctorwatson/heavy.lock" \
  env CARGO_BUILD_JOBS=4 RAYON_NUM_THREADS=8 MORIARTY_PARENT_SRS="$srs" MORIARTY_GP_DIR="$bundle" MORIARTY_GP_CALL_OUT="$out" \
  memcap 48G cargo test --release --locked -p mori-recursion --features sherlock-ledger --test sherlock_gp gp_call_prove \
  -- --exact --include-ignored --nocapture --test-threads=1 >&2
test -s "$out/call.tagged-proof"
od -An -v -tx1 "$out/call.tagged-proof" | tr -d ' \n'
