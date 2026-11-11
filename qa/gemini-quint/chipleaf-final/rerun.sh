#!/usr/bin/env bash
# Lead re-run of the chipleaf-final lane models. Every quint call runs in heavy.slice.
# Usage: qa/gemini-quint/chipleaf-final/rerun.sh LANE OUTDIR   (LANE = packsel | range | m3)
# Prints one line per command: exit code, verdict tag ([ok] / [violation] / N passing; M failed / other),
# command. An exit 1 counts as a catch only with a [violation] tag; a parse or type error shows as "other".
set -u
cd "$(dirname "$0")"
lane="${1:?lane}"
out="${2:?outdir}"
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
case $lane in
  packsel)
    M=packsel.qnt
    SOUND=packsel_sound
    INVS=(invRefine reachAcceptedBadTamper reachNarrowMultiSlot reachWideLayout reachStaggeredSharedCol reachStaggeredSplitCols)
    MUTS=(same_code cnt_collides lagrange_skips_zero pack_bound_7 chain_on_last walk_lagrange1 walk_code_shift dec_packed abs_two_codes)
    ;;
  range)
    M=range.qnt
    SOUND=range_sound
    INVS=(${RANGE_INVS:?set RANGE_INVS})
    MUTS=(${RANGE_MUTS:?set RANGE_MUTS})
    ;;
  m3)
    M=m3.qnt
    SOUND=m3_sound
    INVS=(${M3_INVS:?set M3_INVS})
    MUTS=(${M3_MUTS:?set M3_MUTS})
    ;;
esac
T="${M%.qnt}_test.qnt"
for f in "$M" "$T"; do go typecheck "$f"; done
go test --main "${T%.qnt}" "$T"
for inv in "${INVS[@]}"; do go run "$M" --main "$SOUND" --invariant "$inv" "${RUN[@]}"; done
for m in "${MUTS[@]}"; do go run "$M" --main "${M%.qnt}_$m" --invariant invRefine "${RUN[@]}"; done
