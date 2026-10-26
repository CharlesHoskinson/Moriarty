# A2 runner snapshot
> Source: file:///home/charl/code/Moriarty-perf/scripts/qa/check-a2.sh
> Collected: 2026-10-26
> Published: Unknown

SOURCE — Read-only snapshot; not an execution result.

#!/usr/bin/env bash
# A2: compile, prove and independently reconstruct/verify linked A102 -> A80 children.
# Invoke through DoctorWatson, which owns the heavy lock and resource measurements.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-4}"
export MORIARTY_TEST_PARAMS_DIR="${MORIARTY_TEST_PARAMS_DIR:-$HOME/.cache/midnight/zk-params}"
mkdir -p target/sherlock-a2
run_dir="$(mktemp -d "$PWD/target/sherlock-a2/run-XXXXXXXX")"
export MORIARTY_LINK_OUTPUT_DIR="$run_dir/children"
exec cargo test --release --locked -p mori-backend --features sherlock-real-proofs \
  --test sherlock_linked_children -- --include-ignored --nocapture --test-threads=1
