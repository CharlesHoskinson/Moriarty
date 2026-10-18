# Four-piece VS harness-repair local measurement, step OPX2x (extraction, exact comparison and host-CEK comparison). PROPOSED SOURCE; never executed by its author.
# Run exactly once, in EXECUTION-PROPOSAL.json launch order, only after a fresh source/effect review of these exact bytes,
# a second substantive position on the operator clauses and the root's exact authorization:
#   env -i PATH=/usr/bin:/bin HOME=/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-four-way-vs-harness-repair-local-measurement-20261016/empty-home TZ=UTC /usr/bin/bash --noprofile --norc <this file> < /dev/null
# This script always exits 0. The step result is $T/logs/OPX2x.exit (absent = failed; 200 = not started, aggregate
# budget; 201 = not started, gate). OPX2x.exit 0 = the report parsed (the verdict is receipts/OPX2-compare.json
# "classification"); 3 = no parseable report (classify from logs/OPX2.log); 2 = a pin differs.
umask 022
cd /
export BASE=/home/charl/moriarty-eutxo-validation-2026-10-12
export OWN="$BASE/current-cardano-four-way-vs-harness-repair-operator-materialized-20261016"
export PINSTORE="$BASE/current-cardano-four-way-vs-harness-repair-root-records-20261016"
export M="$BASE/current-cardano-four-way-vs-harness-repair-materialized-20261016"
export PT="$BASE/current-cardano-sharded-settlement-parser-repair-local-measurement-20261016"
export R1T="$BASE/current-cardano-p3-r1-aiken-local-execution-20261015"
export PIN="$BASE/aiken-stdlib-v220-source-20261015"
export DATAONLY="$BASE/aiken-fixed-compiler-release-discovery-20261015/v1.1.24-data-only"
export T="$BASE/current-cardano-four-way-vs-harness-repair-local-measurement-20261016"
export AIKEN="$T/bin/aiken"
export PY=/usr/bin/python3.14
# Aggregate cap: 1,800 s over the summed wall time of all step processes; OPX0 is charged 70 s and OPX1 5 s.
B_TOTAL=1800
B_FIXED=75
run_step() { # LOGDIR ID CAP KILL RESERVE CMD
  if test -f "$T/journal/elapsed.txt"; then E=$(awk '{s+=$2} END{print s+0}' "$T/journal/elapsed.txt"); else E=100000; fi
  case "$E" in ''|*[!0-9]*) E=100000;; esac
  U=$(( B_FIXED + E ))
  W=$(( B_TOTAL - U - $5 - $4 ))
  if test "$W" -gt "$3"; then W=$3; fi
  S0=$(date +%s)
  if test "$W" -lt 5; then
    echo "BUDGET: aggregate $B_TOTAL s, used $U s, reserve $5 s; $2 not started" | tee "$1/$2.log"; echo 200 > "$1/$2.exit"
  else
    printf 'used %s wall %s kill-after %s reserve %s\n' "$U" "$W" "$4" "$5" > "$T/journal/$2.budget"
    nice -n 10 timeout --kill-after="$4" "$W" prlimit --as=34359738368 --core=0 -- \
      bash -o pipefail -c "{ $6
} 2>&1 | tee \"$1/$2.log\""; echo $? > "$1/$2.exit"
  fi
  echo "$2 $(( $(date +%s) - S0 + 1 ))" >> "$T/journal/elapsed.txt"
}
skip_step() { # LOGDIR ID REASON
  echo "GATE: $3; $2 not started" | tee "$1/$2.log"; echo 201 > "$1/$2.exit"
  echo "$2 1" >> "$T/journal/elapsed.txt"
}
ok() { for s in "$@"; do test "$(cat "$T/logs/$s.exit" 2>/dev/null)" = 0 || return 1; done; }
J="$T/journal/OPX2x.txt"
printf 'start %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" > "$J"
# OPX2x (cap 30 s, kill-after 10 s, reserve 136 s) if logs/OPX2.log exists from a started OPX2: read-only extraction of
# module/title/status/on_failure/mem/cpu, seed and summary to receipts/OPX2-tests.tsv; exact comparison with the frozen
# 147-name index (EXPECTATIONS.json shard4TestModule + shard4CostModule, pinned 80cf1601...); PC4-1..PC4-4 against
# attempt 2's raw shc_* rows (OP2-tests.tsv, pinned 2771106b..., not adopted) into receipts/OPX2-compare.json.
E2="$(cat "$T/logs/OPX2.exit" 2>/dev/null)"
if test -f "$T/logs/OPX2.log" && test -n "$E2" && test "$E2" != 200 && test "$E2" != 201; then
run_step "$T/logs" OPX2x 30 10 136 'cd / && "$PY" -I "$OWN/tools/test_compare4.py" "$T/logs/OPX2.log" "$T/receipts/OPX2-tests.tsv" "$T/receipts/OPX2-compare.json" "$M/EXPECTATIONS.json" "$PT/receipts/OP2-tests.tsv"; RC=$?;
if test -f "$T/receipts/OPX2-tests.tsv"; then wc -l < "$T/receipts/OPX2-tests.tsv"; cut -f1,3,4 "$T/receipts/OPX2-tests.tsv" | LC_ALL=C sort | uniq -c; fi; exit $RC'
else skip_step "$T/logs" OPX2x "no log from a started OPX2"; fi
printf 'end %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" >> "$J"
times >> "$J"
