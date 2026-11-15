#!/bin/bash
# T23 Part A finalize (systemd user unit): wait until the t23-partA unit ends, give the programs it did not finish one
# retry pass (only when no call is in flight), run the tx-level compare and report.py on the rerun state, then write
# PREVIEW-20261115.md, copy the rerun's public receipts beside it and commit those paths on branch t23-suite.
# No push. The pre-commit tier runs; a failed run is retried (flaky timing tests under load), never skipped.
set -uo pipefail
here=$(cd "$(dirname "$0")" && pwd); wt=$(cd "$here/../.." && pwd)
A=$HOME/.cache/t23/partA; RUN=$A/suite; LOG=$HOME/.cache/t23/logs/partA.log
export MYC_PREVIEW_HOME=$A/home
while systemctl --user is-active -q t23-partA; do sleep 60; done
unfinished() { python3 -I - "$MYC_PREVIEW_HOME/state" <<'EOF'
import json, pathlib, sys
st = pathlib.Path(sys.argv[1])
c = json.loads((pathlib.Path.home() / ".cache/t23/partA/suite/build/full/compile.json").read_text())
for k in sorted(k for k, v in c.items() if v.get("status") == "ok"):
    f = st / f"{k}.json"
    if not (f.exists() and json.loads(f.read_text()).get("finishedAt")):
        print(k)
EOF
}
pending() { grep -l '"pending"' $MYC_PREVIEW_HOME/state/*.json 2>/dev/null | grep -v txcmp; }
left=$(unfinished)
if [ -n "$left" ] && [ -z "$(pending)" ]; then
  echo "# $(date -Is) retry pass: $(echo $left | wc -w) programs"
  (cd $RUN && for id in $left; do
     until python3 -I $here/parta.py gate; do sleep 300; done
     echo "# $(date -Is) run $id (retry pass)"
     $HOME/.local/bin/heavy-slot --mem 6G --name t23-partA-$id -- node tools/preview/preview.mjs run "$id" || echo "# $(date -Is) $id exit $?"
     [ -n "$(pending)" ] && { echo "# $(date -Is) $id has a call in flight; stop"; break; }
   done) >> $LOG 2>&1
fi
cd $RUN || exit 1
until python3 -I $here/parta.py gate; do sleep 300; done
node tools/preview/compare.mjs
[ -e preview/balance-after.json ] || node tools/preview/preview.mjs balance after
python3 -I tools/preview/report.py
cd $wt || exit 1
[ "$(git rev-parse --abbrev-ref HEAD)" = t23-suite ] || { echo "not on t23-suite"; exit 1; }
D=mycroft/tests/compact-suite/preview-20261115
mkdir -p $D
python3 -I $here/parta_report.py mycroft/tests/compact-suite/PREVIEW-20261115.md || exit 1
# public receipts only (addresses, tx hashes, blocks, fees, transcript words); balances are wallet totals only
rsync -a --include='*/' --include='*.json' --exclude='*' $RUN/preview/ $D/
cp $A/probe.json $A/diff.json $D/
if grep -rlE '"(seed|mnemonic|secret)' $D >/dev/null; then echo "secret-like key in receipts; not committing"; exit 1; fi
git add mycroft/tests/compact-suite/PREVIEW-20261115.md $D
s=$(grep -m1 -A7 '^| result | 2026-11-14' mycroft/tests/compact-suite/PREVIEW-20261115.md | tail -5 | tr -s ' |' ' ' | paste -sd';')
for i in 1 2 3; do
  git commit -q -m "T23 Part A: Preview rerun of the Compact suite, results and per-call diff against 2026-11-14

$(grep -m1 '^Status:' mycroft/tests/compact-suite/PREVIEW-20261115.md)
$(grep -m1 '^Actual Preview transactions' mycroft/tests/compact-suite/PREVIEW-20261115.md)
Totals (2026-11-14, 2026-11-15):$s" && { echo "# $(date -Is) FINALIZE COMMITTED $(git rev-parse --short HEAD)"; exit 0; }
  echo "# $(date -Is) commit attempt $i failed; retry"
done
echo "# $(date -Is) FINALIZE NOT COMMITTED (files staged)"; exit 1
