#!/usr/bin/env bash
# One-shot before/after timing of all465 (dependency opt-level change).
# Run ONLY via: scripts/doctorwatson heavy --id WF3-MEASURE --budget 3h --hypothesis ... -- scripts/qa/measure-all465.sh
# before = HEAD~ Cargo.toml (cabacadd9), after = this tree. Build and test time reported separately.
set -euo pipefail
export CARGO_BUILD_JOBS=8
here="$(git rev-parse --show-toplevel)"
T=all465_independent_raw_mutants_reject_both_binding_modes
base="$(mktemp -d)"; trap 'git -C "$here" worktree remove --force "$base/before" 2>/dev/null || true' EXIT
git -C "$here" worktree add --detach "$base/before" "${BEFORE_REV:-cabacadd9}" >/dev/null
run() { # label dir
  local label=$1 dir=$2 t0 t1 t2
  cd "$dir"; export CARGO_TARGET_DIR="${MEASURE_TARGET_BASE:-$HOME/.cache/wf3-measure}/$label"
  t0=$(date +%s); cargo test -p mori-backend --lib --no-run >/dev/null 2>&1
  t1=$(date +%s); cargo test -p mori-backend --lib "$T" >/tmp/wf3-$label.out 2>&1 || true
  t2=$(date +%s)
  echo "MEASURE $label build=$((t1-t0))s test=$((t2-t1))s $(grep -E '^test result' /tmp/wf3-$label.out | head -1)"
}
run after "$here"
run before "$base/before"
