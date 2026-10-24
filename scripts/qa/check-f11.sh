#!/usr/bin/env bash
# F11: nothing waits on a reviewer. No agentbus wait verb or --wait flag outside qa/tiers/t3.sh,
# `bus review --t3` is called only from t3.sh, and the one timed wait is bounded (3600 s, chunks <= 240 s).
set -euo pipefail
root="$(git rev-parse --show-toplevel)"
cd "$root"
status=0
scan=(models/doctorwatson/factory/*.py scripts/doctorwatson scripts/doctorwatson.sh scripts/sherlock-next scripts/qa qa/tiers .githooks)
existing=()
for p in "${scan[@]}"; do [[ -e $p ]] && existing+=("$p"); done
hits="$(grep -rnE -e '--wait\b' -e 'agentbus +wait\b' -e '\["wait"' -e '"wait", *"--' "${existing[@]}" 2>/dev/null \
  | grep -v -e '^qa/tiers/t3\.sh:' -e '^scripts/qa/check-f11\.sh:' -e '/tests/' || true)"
if [[ -n "$hits" ]]; then
  echo "wait outside qa/tiers/t3.sh:"; echo "$hits"; status=1
fi
callers="$(grep -rnE 'bus +review.*--t3' scripts/doctorwatson scripts/sherlock-next scripts/qa qa/tiers .githooks 2>/dev/null \
  | grep -v -e '^qa/tiers/t3\.sh:' -e '^scripts/qa/check-f11\.sh:' || true)"
if [[ -n "$callers" ]]; then
  echo "bus review --t3 called outside qa/tiers/t3.sh:"; echo "$callers"; status=1
fi
grep -q 'bus review.*--t3' qa/tiers/t3.sh || { echo "qa/tiers/t3.sh does not route its review through doctorwatson bus review --t3"; status=1; }
[[ $status -eq 0 ]] || exit $status
cd models/doctorwatson
exec python3 -m unittest \
  factory.tests.test_bus.InProcess.test_wait_plan_is_bounded \
  factory.tests.test_bus.BusCommands.test_review_without_t3_returns_at_once \
  factory.tests.test_bus.BusCommands.test_t3_review_gives_up_and_proceeds
