#!/usr/bin/env bash
# Lead re-run of the chipleaf-hash lane models. Every quint call runs in heavy.slice.
# Usage: qa/gemini-quint/chipleaf-hash/rerun.sh OUTDIR
# Prints one line per command: exit code, verdict tag ([ok] / [violation] / passing / other), command.
# An exit 1 counts as a catch only with a [violation] tag; a parse or type error shows as "other".
set -u
cd "$(dirname "$0")"
out="${1:?outdir}"
mkdir -p "$out"
Q=(heavy-run --mem 4G -- quint)
RUN=(--max-samples 4000 --max-steps 3 --seed 0x20261110)
n=0
go() {
  n=$((n + 1))
  local log="$out/$(printf '%02d' $n).log"
  "${Q[@]}" "$@" >"$log" 2>&1
  local rc=$?
  local tag=other
  grep -q '\[violation\]' "$log" && tag='[violation]'
  grep -q '\[ok\]' "$log" && tag='[ok]'
  grep -q 'passing' "$log" && tag="$(grep -Eo '[0-9]+ passing' "$log")"
  grep -Eq '[0-9]+ failed' "$log" && tag="$tag; $(grep -Eo '[0-9]+ failed' "$log")"
  echo "exit=$rc $tag :: quint $*"
}
for f in hashchip.qnt hashchip_test.qnt; do go typecheck "$f"; done
go test --main hashchip_test hashchip_test.qnt
H=hashchip.qnt
for inv in invRefine reachAcceptedBadTamper reachHashRequestServed reachTailChunk reachChainedSponge; do
  go run "$H" --main hash_sound --invariant "$inv" "${RUN[@]}"
done
for m in skip_absorb_first skip_absorb_chain tail_absorbs_in1 chain_no_prev capacity_advice len_wrong \
  key_unpinned chip_unpinned out_not_copied out_from_nonfinal prov_on_chain_row one_input_sponge \
  round_gate_dropped prov_zero_not_zero; do
  go run "$H" --main "hash_$m" --invariant invRefine "${RUN[@]}"
done
