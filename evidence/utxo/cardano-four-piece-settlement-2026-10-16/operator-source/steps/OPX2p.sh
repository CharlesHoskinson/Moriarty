# Four-piece VS harness-repair local measurement, step OPX2p (hygiene after the test run). PROPOSED SOURCE; never executed by its author.
# Run exactly once, in EXECUTION-PROPOSAL.json launch order, only after a fresh source/effect review of these exact bytes,
# a second substantive position on the operator clauses and the root's exact authorization:
#   env -i PATH=/usr/bin:/bin HOME=/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-four-way-vs-harness-repair-local-measurement-20261016/empty-home TZ=UTC /usr/bin/bash --noprofile --norc <this file> < /dev/null
# This script always exits 0. The step result is $T/logs/OPX2p.exit (absent = failed; 200 = not started, aggregate
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
J="$T/journal/OPX2p.txt"
printf 'start %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" > "$J"
# OPX2p ALWAYS after a started OPX2 (cap 60 s, kill-after 10 s, reserve 176 s = OPX2x 40 + OPX7 100 + OPX7f 36). Same checks
# as OPX3p on the tree after the test run (OPX2 started only if OPX2a was 0, so the lock files are required; plutus.json is
# required unchanged if OPX3 exited 0), then a digest record.
E2="$(cat "$T/logs/OPX2.exit" 2>/dev/null)"
if test -n "$E2" && test "$E2" != 200 && test "$E2" != 201; then
FL="--require-lock"; if ok OPX3; then FL="$FL --plutus $T/receipts/plutus-unapplied-sha256.txt $T/a2/plutus.unapplied.json"; fi
export FL
run_step "$T/logs" OPX2p 60 10 176 'cd / && printf "%s  %s\n" 4257cc8be37cbde5bf7b5dd111e43a1277f45352fb2433ad7a7e647209aa7677 "$AIKEN" | sha256sum -c - && test "$(stat -c %a "$AIKEN")" = 555 &&
for d in empty-home empty-xdg-cache empty-xdg-config empty-xdg-data probe-cwd; do test -d "$T/$d" && test -z "$(find "$T/$d" -mindepth 1 -print -quit)" || { echo "not empty or missing: $d"; exit 1; }; done &&
"$PY" -I "$OWN/tools/tree_check.py" "$T/run" "$OWN/data" "$T/receipts/tree-after-tests.json" after-tests $FL &&
find "$T/tmp" -mindepth 1 | tee "$T/receipts/tmp-after-tests.txt" &&
"$PY" -I "$OWN/tools/freeze.py" "$T/receipts/run-after-tests.json" "run tree after the filtered aiken check" "$T" run'
else skip_step "$T/logs" OPX2p "OPX2 not started"; fi
printf 'end %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" >> "$J"
times >> "$J"
