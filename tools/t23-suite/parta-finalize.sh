#!/bin/bash
# T23 Part A finalize (systemd user unit): wait until parta.sh logs PARTA DONE (or stops), then write
# PREVIEW-20261115.md, copy the rerun's public receipts beside it and commit those paths on branch t23-suite.
# No push. The pre-commit tier runs; a failed run is retried (flaky timing tests under load), never skipped.
set -uo pipefail
here=$(cd "$(dirname "$0")" && pwd); wt=$(cd "$here/../.." && pwd)
A=$HOME/.cache/t23/partA; LOG=$HOME/.cache/t23/logs/partA.log
until grep -qE "PARTA DONE|; stop|in flight; stop" $LOG || ! systemctl --user is-active -q t23-partA; do sleep 60; done
cd $wt || exit 1
[ "$(git rev-parse --abbrev-ref HEAD)" = t23-suite ] || { echo "not on t23-suite"; exit 1; }
D=mycroft/tests/compact-suite/preview-20261115
mkdir -p $D
python3 -I $here/parta_report.py mycroft/tests/compact-suite/PREVIEW-20261115.md || exit 1
# public receipts only (addresses, tx hashes, blocks, fees, transcript words); balances are wallet totals only
rsync -a --include='*/' --include='*.json' --exclude='*' $A/suite/preview/ $D/
cp $A/probe.json $A/diff.json $D/
cp $A/suite/PREVIEW.md $D/report-PREVIEW.md 2>/dev/null || true
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
exit 1
