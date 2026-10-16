# Four-piece VS harness-repair local measurement, step OPX2 (local CEK tests of the two new modules only). PROPOSED SOURCE; never executed by its author.
# Run exactly once, in EXECUTION-PROPOSAL.json launch order, only after a fresh source/effect review of these exact bytes,
# a second substantive position on the operator clauses and the root's exact authorization:
#   env -i PATH=/usr/bin:/bin HOME=/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-four-way-vs-harness-repair-local-measurement-20261016/empty-home TZ=UTC /usr/bin/bash --noprofile --norc <this file> < /dev/null
# This script always exits 0. The step result is $T/logs/OPX2.exit (absent = failed; 200 = not started, aggregate
# budget; 201 = not started, gate). The root may run this one step as a background task with the identical command text.
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
J="$T/journal/OPX2.txt"
printf 'start %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" > "$J"
# OPX2 (gate OPX2a = 0 and OPX3p = 0; a FAILED build does not block it). Cap: whatever the aggregate ledger leaves after
# reserving 246 s (OPX2p 70 + OPX2x 40 + OPX7 100 + OPX7f 36), kill-after 60 s. Expected: 147 tests (125 + 22, 9 `fail`).
# Test selection (position OPX-FILT). The byte-equal check help says "-m, --match-tests: Only run tests if they match any of
# these strings. You can match a module with `-m aiken/list` or `-m list`." The author's `-m shard4` is NOT used: a bare
# word without "/" or "." is, in aiken's matcher as recalled by this author (source not in the freeze, so unverified), a
# test-NAME substring filter across all modules, and no expected name contains "shard4"; help and recalled source
# disagree. The two slash-qualified module names below select exactly pvc2/shard4_test and pvc2/shard4_cost_test under
# both the help reading and the recalled source reading (OPX0 P5 proved no other module name contains either string).
# Repeated flags with one value each and nothing after them parse whether clap takes one value per flag or many.
# Any membership deviation (other modules, missing names, zero tests) is UNCLASSIFIED(match-filter), never a candidate
# result. The 472 parent tests are NOT re-executed (unchanged modules; attempt 2's observation stays raw history).
# stdout+stderr merged and tee-d unchanged; stdout is a pipe, so the report is JSON. Peak RSS via /usr/bin/time if present.
if ok OPX2a OPX3p; then
run_step "$T/logs" OPX2 1800 60 246 'cd "$T/run" && test -f aiken.lock && if test -x /usr/bin/time; then TW="/usr/bin/time -v -o $T/receipts/OPX2-time.txt"; else TW=""; echo "OPX2: /usr/bin/time absent; peak RSS not recorded"; fi &&
$TW unshare --user --map-root-user --net -- env -i PATH=/usr/bin:/bin HOME="$T/empty-home" XDG_CACHE_HOME="$T/empty-xdg-cache" XDG_CONFIG_HOME="$T/empty-xdg-config" XDG_DATA_HOME="$T/empty-xdg-data" TMPDIR="$T/tmp" NO_COLOR=1 "$AIKEN" check --trace-filter all --trace-level silent --match-tests pvc2/shard4_test --match-tests pvc2/shard4_cost_test < /dev/null'
else skip_step "$T/logs" OPX2 "OPX2a or OPX3p not 0"; fi
printf 'end %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" >> "$J"
times >> "$J"
