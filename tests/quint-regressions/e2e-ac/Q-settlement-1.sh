#!/usr/bin/env bash
# Q-settlement-1 regression (text against code). Correct behaviour: the adopted LDG-001 text must not forbid
# separate ledgers while the T23 driver deploys one SherlockLedger per program (SLC-005, owner item O8).
# Pinned at check time: sherlock-e2e-spec@c154662c4, t20-batched@5dc1be2ee (LDG-001 "as amended"),
# t23-post-t25@cf7a86473 tools/t23-suite/t23.py:349 ("one SherlockLedger per program").
# FAILS while LDG-001 still says "Two contracts with separate ledgers SHALL NOT be used" with no per-program scope.
set -u
W=/home/hoskinson/Projects/Moriarty-wt
SPEC=openspec/changes/sherlock-modular-leaf-20261111/specs/sherlock-ledger-contract/spec.md
DRV=$W/t23-post-t25/tools/t23-suite/t23.py
grep -q 'one SherlockLedger per program' "$DRV" || { echo "SKIP: driver no longer deploys per program"; exit 0; }
rc=0
for t in sherlock-e2e-spec t20-batched; do
  f=$W/$t/$SPEC
  line=$(grep -m1 'Two contracts with separate ledgers SHALL NOT be used' "$f" || true)
  if [ -n "$line" ] && ! grep -qiE 'per program|for each program|one ledger per program|O8|SLC-005' <<<"$line"; then
    echo "FAIL $t: LDG-001 forbids separate ledgers, but t23.py deploys one SherlockLedger per program"; rc=1
  else echo "PASS $t"; fi
done
exit $rc
