# K5 operator, step H5: non-gating records of Q5, K5a, K5b and K5M (runs if any of their logs exists). PROPOSED SOURCE; never executed by its author.
# Run exactly once, after K5g (or after the first stop of K5):
#   env -i PATH=/usr/bin:/bin HOME=/nonexistent LC_ALL=C TZ=UTC /usr/bin/bash --noprofile --norc <this file> < /dev/null
# This script always exits 0. The step result is the content of $T/logs/H5.exit. It decides nothing.
umask 022
cd /
export LC_ALL=C
export BASE=/home/charl/moriarty-eutxo-validation-2026-10-12
export M5="$BASE/current-cardano-k2-k5-setup-operator-materialized-20261016"
export OWN="$M5/operator/request"
export RR5="$BASE/current-cardano-k2-k5-root-records-20261016"
export M="$BASE/current-cardano-k2-k4h-hostile-materialized-20261016"
export C="$BASE/current-cardano-k2-g1-materialized-candidate-20261015"
export RR="$BASE/current-cardano-k2-ordinary-root-records-20261015"
export TOLD="$BASE/current-cardano-k2-ordinary-local-execution-20261015"
export TK="$BASE/current-cardano-k2-k4h-hostile-local-execution-20261016"
export FREEZE="$BASE/claim-utxo-abi-v1-aiken-placement-candidate-20261015/tools/freeze.py"
export T="$BASE/current-cardano-k2-k5-single-party-test-setup-20261016"
export W="$T/w"
run_step() { nice -n 10 timeout --kill-after=60 "$3" prlimit --as=8589934592 --cpu="$5" --core=0 -- \
  bash -o pipefail -c "{ $4
} 2>&1 | tee \"$1/$2.log\""; echo $? > "$1/$2.exit"; }
ID=H5
once() { if test -e "$T/logs/$ID.log" || test -e "$T/logs/$ID.exit" || test -e "$T/logs/$ID.skipped"; then echo "REFUSED: step $ID already ran or was skipped; one attempt only"; exit 0; fi; }
test -f "$T/logs/Q5.log" || test -f "$T/logs/K5a.log" || test -f "$T/logs/K5b.log" || test -f "$T/logs/K5M.log" || exit 0
once
J="$T/journal/$ID.txt"
printf 'start %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" > "$J"
# H5 reads host-written logs and receipts only (never w/): (1) the in-view GNU time report lines of Q5, K5a, K5b and
# K5M (K-6); (2) lines that may classify a death or refusal, copied for the result reviewer, with getrandom and
# allocation signs kept verbatim (clause K5-5 precedence is the reviewer's, never this step's); (3) the K5M stdout
# summary and the K5g receipt, if any. Classifies nothing; the full logs govern. Original failures and resource
# counters are retained unchanged.
run_step "$T/logs" "$ID" 120 'for S in Q5 K5a K5b K5M; do if test -f "$T/logs/$S.log"; then
{ grep -E "Command being timed|User time [(]seconds[)]|System time [(]seconds[)]|Percent of CPU|Elapsed [(]wall clock[)]|Maximum resident set size|Exit status|Command terminated by signal|Command exited with non-zero status" "$T/logs/$S.log" > "$T/receipts/$S-inview-time.txt" || true; } &&
{ grep -E "process didn.t exit successfully|memory allocation of|out of memory|Cannot allocate memory|memory exhausted|getrandom|signal: [0-9]+|terminated with signal|Command terminated by signal|SIG(KILL|XCPU|ABRT|SEGV|BUS|ILL)|panicked at|^refused: |^usage: |^PROBE-FAIL:" "$T/logs/$S.log" > "$T/receipts/$S-abnormal.txt" || true; } &&
echo "record: $S in-view time report:" && cat "$T/receipts/$S-inview-time.txt" &&
echo "record: $S abnormal_or_refusal_lines=$(wc -l < "$T/receipts/$S-abnormal.txt")"; else echo "record: no $S log"; fi || exit 1; done &&
{ if test -f "$T/receipts/K5M-summary.json"; then echo "record: K5M stdout summary:"; cat "$T/receipts/K5M-summary.json"; else echo "record: no K5M summary"; fi; } &&
{ if test -f "$T/receipts/K5g.json"; then echo "record: K5g receipt:"; cat "$T/receipts/K5g.json"; else echo "record: no K5g receipt"; fi; }' 60
printf 'end %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" >> "$J"
times >> "$J"
