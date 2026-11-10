#!/usr/bin/env bash
# Lead re-run of the chipleaf-pairrow-m3 lane models. Every quint call runs in heavy.slice.
# Usage: qa/gemini-quint/chipleaf-pairrow-m3/rerun.sh OUTDIR
# Prints one line per command: exit code, verdict tag ([ok] / [violation] / other), command.
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
for f in pairrow.qnt pairrow_test.qnt m3seg.qnt m3seg_test.qnt; do go typecheck "$f"; done
go test --main pairrow_test pairrow_test.qnt
go test --main m3segTest m3seg_test.qnt
P=pairrow.qnt
for inv in invRefine reachAcceptedBadTamper reachHonestFullChain reachHonestDisabledSlot; do
  go run "$P" --main pairrowSound --invariant "$inv" "${RUN[@]}"
done
for m in P1_frame_gate_dropped P2_frame_gate_enabled_only P3_chain_link_dropped P4_flag_replica_unlinked \
  P5_lane_replica_unlinked P6_routed_operand_unlinked P7_post_d_wrong_slot P8_digest_omits_register \
  P9_scratch_read enable_not_prefix; do
  go run "$P" --main "pairrow_$m" --invariant invRefine "${RUN[@]}"
done
M=m3seg.qnt
for inv in invRefine reachAcceptedBadTamper reachHonestWithPadding reachCrossLeafPageRead; do
  go run "$M" --main m3segSound --invariant "$inv" "${RUN[@]}"
done
go run "$M" --main m3segSoundC8 --invariant invRefine "${RUN[@]}"
for m in M1_padding_nonzero M2_append_skips M3_padding_read M4_close_len_running M5_base_unaligned \
  M6_no_empty_slot_check M7_page_serves_segment M7_alone; do
  go run "$M" --main "m3seg_$m" --invariant invRefine "${RUN[@]}"
done
