#!/usr/bin/env bash
# X12 lead runs: every quint command through q.sh (heavy-run in heavy.slice, no heavy lock, timeout 900).
# Usage: rerun.sh OUTDIR [SAMPLES] [STEPS]
set -u
cd "$(dirname "$0")"
out="${1:?outdir}"; S="${2:-20000}"; K="${3:-10}"
mkdir -p "$out"; : > "$out/summary.txt"
SEED=0x20261112; n=0
go() {
  n=$((n + 1))
  local log="$out/$(printf '%02d' $n).log"
  ./q.sh "x12-$n" "$@" >"$log" 2>&1
  local rc=$? tag=other
  grep -q '\[violation\]' "$log" && tag='[violation]'
  grep -q '\[ok\]' "$log" && tag='[ok]'
  grep -q 'passing' "$log" && tag="$(grep -Eo '[0-9]+ passing' "$log" | tail -1)"
  grep -Eq '[0-9]+ failed' "$log" && tag="$tag; $(grep -Eo '[0-9]+ failed' "$log" | tail -1)"
  echo "exit=$rc $tag :: quint $*" | tee -a "$out/summary.txt"
}
M=claimConsumption.qnt; T=claimConsumption_test.qnt; MATCH='(pos_gp2|neg_|claim_|honest_|probe_)'
go typecheck "$M"; go typecheck "$T"
for m in claimConsumption_test t_noTerminal t_ignoreTag t_headOfJoin t_subUnrelated t_subUnrelated_sound t_linkAnyStatus \
         t_noValueTie t_valueMismatch t_nstepsForged t_plainSealNonEmpty t_noChainCheck t_premiseInChain t_accumSorted \
         t_accumSet t_coverSkipsLater t_linkNoCover t_sealDepsEmpty; do
  go test --match "$MATCH" --main "$m" "$T"
done
RUN=(--max-samples "$S" --max-steps "$K" --seed "$SEED")
go run "$M" --main cc_sound --invariant soundAll "${RUN[@]}"
for w in reachBConsumesA reachJoin reachDescend reachChainSeal; do go run "$M" --main cc_sound --invariant "$w" "${RUN[@]}"; done
for mu in noTerminal ignoreTag headOfJoin subUnrelated linkAnyStatus noValueTie valueMismatch nstepsForged plainSealNonEmpty \
          noChainCheck premiseInChain accumSorted accumSet coverSkipsLater linkNoCover sealDepsEmpty; do
  go run "$M" --main "cc_$mu" --invariant soundAll "${RUN[@]}"
done
