#!/usr/bin/env bash
# T13 (2026-11-13): the two mutants that the 5000-sample rest run did not catch, again at 20000 samples
# (the sample count of full2), same seed and depth, through q-slot.sh (heavy-slot), in parallel.
set -u
cd "$(dirname "$0")"
out=runs/full2c; mkdir -p "$out"; SEED=0x20261112
for mu in valueMismatch accumSorted; do
  ( ./q-slot.sh "x12c-$mu" run claimConsumption.qnt --main "cc_$mu" --invariant soundAll --max-samples 20000 --max-steps 10 --seed "$SEED" >"$out/$mu.log" 2>&1
    rc=$?; tag=other; grep -q '\[violation\]' "$out/$mu.log" && tag='[violation]'; grep -q '\[ok\]' "$out/$mu.log" && tag='[ok]'
    echo "exit=$rc $tag :: quint run claimConsumption.qnt --main cc_$mu --invariant soundAll --max-samples 20000 --max-steps 10 --seed $SEED" >"$out/$mu.sum" ) &
done
wait; cat "$out"/*.sum > "$out/summary.txt"; cat "$out/summary.txt"
