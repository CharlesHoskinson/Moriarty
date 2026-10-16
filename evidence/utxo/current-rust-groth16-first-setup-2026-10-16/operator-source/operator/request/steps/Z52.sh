# K5 operator, step Z52: freeze (ALWAYS, last, once K5 started). PROPOSED SOURCE; never executed by its author.
# Run exactly once, last, after Z51:
#   env -i PATH=/usr/bin:/bin HOME=/nonexistent LC_ALL=C TZ=UTC /usr/bin/bash --noprofile --norc <this file> < /dev/null
# This script always exits 0. The step result is the content of $T/Z52.exit.
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
test -e "$T/logs/P1.log" || test -e "$T/logs/P1.skipped" || exit 0
test ! -e "$T/Z52.exit" || { echo "REFUSED: Z52 already ran; one attempt only"; exit 0; }
J="$T/journal/Z52.txt"
printf 'start %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" > "$J"
# Z52: freeze every host-written K5 log and receipt with the pinned freeze.py; journal/ is excluded because Z52 is still
# writing it. Never freezes, reads or publishes w/, w/bin, w/keys, any proving key, cache or binary.
cd / && /usr/bin/python3 -I "$FREEZE" "$T/receipts/stage-K5-final.json" "K5 single-party test setup logs and receipts" "$T" logs receipts; echo $? > "$T/Z52.exit"
printf 'end %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" >> "$J"
times >> "$J"
