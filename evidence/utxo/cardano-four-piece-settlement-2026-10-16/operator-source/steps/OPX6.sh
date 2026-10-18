# Four-piece VS harness-repair local measurement, step OPX6 (detailed compiled sizes and size predictions). PROPOSED SOURCE; never executed by its author.
# Run exactly once, in EXECUTION-PROPOSAL.json launch order, only after a fresh source/effect review of these exact bytes,
# a second substantive position on the operator clauses and the root's exact authorization:
#   env -i PATH=/usr/bin:/bin HOME=/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-four-way-vs-harness-repair-local-measurement-20261016/empty-home TZ=UTC /usr/bin/bash --noprofile --norc <this file> < /dev/null
# This script always exits 0. The step result is $T/logs/OPX6.exit (absent = failed; 200 = not started, aggregate
# budget; 201 = not started, gate).
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
J="$T/journal/OPX6.txt"
printf 'start %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" > "$J"
# OPX6 (gate OPX3 and OPX3p both 0; cap 30 s, kill-after 10 s, reserve 136 s; non-gating): unapplied and, only if OPX4 exited
# 0, B9-applied compiledCode byte length per validator (c2, vi, vs1, vs2, vs3, vs4, s2p); per-script verdict against the
# 14,336 B working target (not a qualified node parameter); PS4-1..PS4-4 and the size halves of PS4-5/PS4-6; comparison
# with attempt 2's raw sizes (pinned). NOT a deploy/reference-script/settle transaction size, NOT ExUnits, NOT fees.
if ok OPX3 OPX3p; then
run_step "$T/logs" OPX6 30 10 136 'cd / && if test "$(cat "$T/logs/OPX4.exit" 2>/dev/null)" = 0; then APP="$T/a2/b9-applied.json"; else APP=""; echo "OPX6: OPX4 not 0; applied sizes not reported"; fi &&
"$PY" -I "$OWN/tools/size_report4.py" "$T/receipts/sizes.json" "$PT/receipts/sizes.json" "$T/a2/plutus.unapplied.json" $APP && cat "$T/receipts/sizes.json"'
else skip_step "$T/logs" OPX6 "OPX3 or OPX3p not 0"; fi
printf 'end %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" >> "$J"
times >> "$J"
