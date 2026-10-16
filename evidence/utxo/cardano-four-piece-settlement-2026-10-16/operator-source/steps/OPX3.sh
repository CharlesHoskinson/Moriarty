# Four-piece VS harness-repair local measurement, step OPX3 (aiken build, unapplied blueprint). PROPOSED SOURCE; never executed by its author.
# Run exactly once, in EXECUTION-PROPOSAL.json launch order, only after a fresh source/effect review of these exact bytes,
# a second substantive position on the operator clauses and the root's exact authorization:
#   env -i PATH=/usr/bin:/bin HOME=/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-four-way-vs-harness-repair-local-measurement-20261016/empty-home TZ=UTC /usr/bin/bash --noprofile --norc <this file> < /dev/null
# This script always exits 0. The step result is $T/logs/OPX3.exit (absent = failed; 200 = not started, aggregate
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
J="$T/journal/OPX3.txt"
printf 'start %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" > "$J"
# OPX3 (gate OPX2a = 0; cap 300 s, kill-after 30 s, reserve 136 s). Launch-order position OPX-ORD: the build, chain, labels and
# sizes run BEFORE the long test step, so the size measurement this split exists for cannot be starved by test time.
# aiken build with attempt 2's trace settings; sha of plutus.json; read-only copy a2/plutus.unapplied.json before any apply.
if ok OPX2a; then
run_step "$T/logs" OPX3 300 30 136 'cd "$T/run" && test ! -e plutus.json && test -f aiken.lock && if test -x /usr/bin/time; then TW="/usr/bin/time -v -o $T/receipts/OPX3-time.txt"; else TW=""; echo "OPX3: /usr/bin/time absent; peak RSS not recorded"; fi &&
$TW unshare --user --map-root-user --net -- env -i PATH=/usr/bin:/bin HOME="$T/empty-home" XDG_CACHE_HOME="$T/empty-xdg-cache" XDG_CONFIG_HOME="$T/empty-xdg-config" XDG_DATA_HOME="$T/empty-xdg-data" TMPDIR="$T/tmp" NO_COLOR=1 "$AIKEN" build --trace-filter all --trace-level silent < /dev/null &&
sha256sum plutus.json | tee "$T/receipts/plutus-unapplied-sha256.txt" &&
test ! -e "$T/a2/plutus.unapplied.json" && cp plutus.json "$T/a2/plutus.unapplied.json" && cmp plutus.json "$T/a2/plutus.unapplied.json" && chmod 0444 "$T/a2/plutus.unapplied.json"'
else skip_step "$T/logs" OPX3 "OPX2a not 0"; fi
printf 'end %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" >> "$J"
times >> "$J"
