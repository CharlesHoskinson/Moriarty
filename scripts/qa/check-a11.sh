#!/usr/bin/env bash
# A11: keygen regression at the current pins (task 2.4; SRC-008, SRC-009, SMN-002).
# Checks the committed manifests in qa/sherlock/manifests/ and the regression record
# a11-regression.json: every SRC-008 field, pins equal to Cargo.lock, the verdict per relation
# against its prior manifest, the regenerated closure leaf -> node -> decider -> contract,
# and the heavy run ids in runs.jsonl.
# A11_RUN=1 also reruns the keygen (VK only; under scripts/doctorwatson heavy, about 20 GiB)
# into a new directory and refuses if any fresh hash differs from the manifests.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
if [ "${A11_RUN:-0}" = 1 ]; then
  out="${A11_OUT:?A11_OUT: a new output directory}"
  export MORIARTY_A11_OUT="$out"
  export MORIARTY_TEST_PARAMS_DIR="${MORIARTY_TEST_PARAMS_DIR:-$HOME/.cache/midnight/zk-params}"
  export MORIARTY_PARENT_SRS="${MORIARTY_PARENT_SRS:-$MORIARTY_TEST_PARAMS_DIR/bls_midnight_2p20}"
  cargo test --release --locked -p mori-backend --test sherlock_c6_child -- \
    --ignored --nocapture --exact a11_c6_digest_leaf_key_identity
  for t in a11_run_closure a11_parent_closure; do
    cargo test --release --locked -p mori-recursion --features sherlock-ledger \
      --test sherlock_keygen_regression -- --ignored --nocapture --exact "$t" --test-threads=1
  done
  python3 scripts/qa/a11-manifests.py compare "$out"
fi
exec python3 scripts/qa/a11-manifests.py check
