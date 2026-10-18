# Four-piece VS harness-repair local measurement, step OPX7f (outcome record and final freeze). PROPOSED SOURCE; never executed by its author.
# Run exactly once, LAST, after OPX7.exit exists, only after a fresh source/effect review of these exact bytes, a second
# substantive position on the operator clauses and the root's exact authorization:
#   env -i PATH=/usr/bin:/bin HOME=/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-four-way-vs-harness-repair-local-measurement-20261016/empty-home TZ=UTC /usr/bin/bash --noprofile --norc <this file> < /dev/null
# This script always exits 0. The step result is $T/OPX7f.exit (200 = not started, aggregate budget).
# Not tee-logged (no log may be open while logs/ is frozen); bounded by timeout 30 s + 5 s kill-after and the same
# AS/core/nice limits; its stdout/stderr go to journal/OPX7f.out. journal/ is excluded because this step is still writing it.
# (1) tools/evaluate4.py writes receipts/outcome.json: per-stage labels, every frozen prediction evaluated
#     (TRUE/FALSE/UNEVALUATED), "allCommandsExit0" kept separate from "allPredictionsHeld", lineage and the charged ledger;
# (2) tools/freeze.py records logs, receipts, help and a2 into receipts/task-final.json; (3) those files become read-only.
# The outcome record is written even if it cannot be frozen, and the freeze runs even if the outcome record fails.
umask 022
cd /
export BASE=/home/charl/moriarty-eutxo-validation-2026-10-12
export OWN="$BASE/current-cardano-four-way-vs-harness-repair-operator-materialized-20261016"
export T="$BASE/current-cardano-four-way-vs-harness-repair-local-measurement-20261016"
export PY=/usr/bin/python3.14
B_TOTAL=1800
B_FIXED=75
J="$T/journal/OPX7f.txt"
printf 'start %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" > "$J"
if test -f "$T/journal/elapsed.txt"; then E=$(awk '{s+=$2} END{print s+0}' "$T/journal/elapsed.txt"); else E=100000; fi
case "$E" in ''|*[!0-9]*) E=100000;; esac
U=$(( B_FIXED + E ))
S0=$(date +%s)
if test $(( B_TOTAL - U )) -lt 35; then
  echo "BUDGET: aggregate $B_TOTAL s, used $U s; OPX7f not started" > "$T/journal/OPX7f.out"; echo 200 > "$T/OPX7f.exit"
else
  cd / && nice -n 10 timeout --kill-after=5 30 prlimit --as=34359738368 --core=0 -- bash -o pipefail -c '
"$PY" -I "$OWN/tools/evaluate4.py" "$T" "$T/receipts/outcome.json"; R1=$?;
"$PY" -I "$OWN/tools/freeze.py" "$T/receipts/task-final.json" "all task logs, receipts, raw help and blueprints" "$T" logs receipts help a2; R2=$?;
find "$T/logs" "$T/receipts" "$T/help" "$T/a2" -type f -exec chmod a-w {} +; R3=$?;
echo "evaluate $R1 freeze $R2 chmod $R3"; test "$R1" = 0 && test "$R2" = 0 && test "$R3" = 0' > "$T/journal/OPX7f.out" 2>&1 < /dev/null; echo $? > "$T/OPX7f.exit"
fi
echo "OPX7f $(( $(date +%s) - S0 + 1 ))" >> "$T/journal/elapsed.txt"
printf 'end %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" >> "$J"
times >> "$J"
