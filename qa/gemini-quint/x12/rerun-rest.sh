#!/usr/bin/env bash
# Rest of the X12 mutant runs after full2 was cut at cc_noValueTie (load average 30: [ok] runs took ~850 s each at 20000 samples).
# Same seed and depth, 5000 samples. Plus one deeper reach run for reachChainSeal (20 steps).
set -u
cd "$(dirname "$0")"
out=runs/full2b; mkdir -p "$out"; : > "$out/summary.txt"; SEED=0x20261112; n=0
go() {
  n=$((n + 1)); local log="$out/$(printf '%02d' $n).log"
  ./q.sh "x12b-$n" "$@" >"$log" 2>&1; local rc=$? tag=other
  grep -q '\[violation\]' "$log" && tag='[violation]'; grep -q '\[ok\]' "$log" && tag='[ok]'
  [ $rc = 124 ] && tag=timeout
  echo "exit=$rc $tag :: quint $*" | tee -a "$out/summary.txt"
}
M=claimConsumption.qnt
for mu in valueMismatch nstepsForged plainSealNonEmpty noChainCheck premiseInChain accumSorted accumSet coverSkipsLater linkNoCover sealDepsEmpty; do
  go run "$M" --main "cc_$mu" --invariant soundAll --max-samples 5000 --max-steps 10 --seed "$SEED"
done
go run "$M" --main cc_sound --invariant reachChainSeal --max-samples 5000 --max-steps 20 --seed "$SEED"
