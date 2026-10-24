#!/usr/bin/env bash
# U016: visible regression tests for the scenario, then holdout suite uireq-016.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
run_cargo() {  # target [filter]; fails when the filter matches zero tests
  local out; out=$(cargo test -p mori-core --test "$@" 2>&1) || { echo "$out" | tail -40; exit 1; }
  echo "$out" | grep -E 'test result' | tail -1
  echo "$out" | grep -Eq 'test result: ok\. [1-9][0-9]* passed' || { echo "no test matched: $*"; exit 1; }
}
run_py() {     # module [dotted test]
  local m=$1; shift; local out
  out=$(cd models/sherlock/qa && python3 -m unittest -v "$m${1:+.$1}" 2>&1) || { echo "$out" | tail -40; exit 1; }
  echo "$out" | tail -3
  echo "$out" | grep -Eq '^Ran [1-9][0-9]* test' || { echo "no test matched: $m $*"; exit 1; }
}
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-6}"
run_cargo sherlock_reference_slice_positive public_technical_cut_resumes_the_same_pending_publication
# The visible tests above are a necessary guard; the holdout scenario is the arbiter.
[ -x qa/holdout/run.sh ] || { echo "UNIMPLEMENTED U016 (qa/holdout/run.sh absent)"; exit 2; }
exec qa/holdout/run.sh uireq-016
