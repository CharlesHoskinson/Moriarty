#!/bin/bash
# T23 Part A runner (systemd user unit): wait for the Preview indexer, then rerun the T18 (a) suite natively on
# Preview into ~/.cache/t23/partA (never over the 2026-11-14 receipts). Resumable: preview.mjs skips finished calls
# and refuses to resubmit a call that was in flight. Before each program the indexer gate runs again and the heavy
# slot is taken for that program only. The wallet phrase is read in-process by preview.mjs (OP-PV-WALLET) and is
# never printed, logged or written.
# usage: parta.sh [IDS...]   (default: every program of the full build)
set -uo pipefail
here=$(cd "$(dirname "$0")" && pwd)
A=$HOME/.cache/t23/partA; RUN=$A/suite
export MYC_PREVIEW_HOME=$A/home PREVIEW_PROOF_URL=${PREVIEW_PROOF_URL:-http://127.0.0.1:6342}
gate() {
  local waited=0
  until python3 -I $here/parta.py gate; do
    [ $waited -ge ${T23_GATE_MAX_S:-21600} ] && { echo "# $(date -Is) indexer not ready after $waited s; stop"; exit 2; }
    sleep 300; waited=$((waited + 300))
  done
}
gate
python3 -I $here/preview_probe.py $HOME/Projects/Moriarty-wt/mycroft/mycroft/tests/compact-suite/preview $A/probe.json || exit 1
python3 -I $here/parta.py setup || exit 1
echo -n "PLANNED "; python3 -I $here/parta.py plan | tail -1 || exit 1
curl -sf -m5 $PREVIEW_PROOF_URL/version >/dev/null || { echo "proof server $PREVIEW_PROOF_URL down"; exit 1; }
cd $RUN || exit 1
ids=("$@"); [ ${#ids[@]} -eq 0 ] && mapfile -t ids < <(python3 -I -c 'import json;c=json.load(open("build/full/compile.json"));print("\n".join(sorted(k for k,v in c.items() if v.get("status")=="ok")))')
[ -e preview/balance-before.json ] || node tools/preview/preview.mjs balance before || exit 1
for id in "${ids[@]}"; do
  gate
  echo "# $(date -Is) run $id"
  $HOME/.local/bin/heavy-slot --mem 6G --name t23-partA-$id -- node tools/preview/preview.mjs run "$id" || echo "# $(date -Is) $id exit $?"
  if python3 -I -c 'import json,sys;sys.exit(0 if json.load(open(sys.argv[1])).get("pending") else 1)' $MYC_PREVIEW_HOME/state/$id.json 2>/dev/null; then
    echo "# $(date -Is) $id has a call in flight; stop for a manual check (no automatic resubmit)"; exit 3
  fi
done
gate
node tools/preview/compare.mjs
node tools/preview/preview.mjs balance after
python3 -I tools/preview/report.py
echo -n "ACTUAL PREVIEW TX: "; python3 -I $here/parta.py count
python3 -I $here/parta_diff.py $HOME/Projects/Moriarty-wt/mycroft/mycroft/tests/compact-suite/PREVIEW.md $RUN/PREVIEW.md $A/diff.json $HOME/.cache/mycroft-preview/state $MYC_PREVIEW_HOME/state
echo "# $(date -Is) PARTA DONE"
