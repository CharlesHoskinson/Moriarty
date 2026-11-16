#!/usr/bin/env bash
# Tasks 1.3 + 2.4 dense memory Quint run: denseMemory.qnt (the C1 model, moved) and
# denseStark.qnt (the STARK kit's HeapReadAir and SegAir/FlushAir). Every quint call runs in
# heavy.slice, 4 at a time. Reads commands.tsv (n, expected exit, command); prints n,
# match/MISMATCH, exit, tag, command.
set -u
cd "$(dirname "$0")/../../.."
export OUT="${1:?outdir}"; mkdir -p "$OUT"
one() {
  local n="$1" exp="$2" cmd="$3" log rc tag ok
  log="$OUT/$(printf '%02d' "$n").log"
  eval "heavy-run --mem 4G -- $cmd" >"$log" 2>&1; rc=$?
  tag=other
  grep -q '\[violation\]' "$log" && tag='[violation]'
  grep -q '\[ok\]' "$log" && tag='[ok]'
  grep -q 'passing' "$log" && tag="$(grep -Eo '[0-9]+ passing' "$log" | tail -1)"
  ok=MISMATCH; [ "$rc" = "$exp" ] && ok=match
  echo "$(printf '%02d' "$n") $ok exit=$rc expected=$exp $tag :: $cmd"
}
export -f one
while IFS=$'\t' read -r n exp cmd; do
  while [ "$(jobs -rp | wc -l)" -ge 4 ]; do wait -n; done
  one "$n" "$exp" "$cmd" &
done < qa/gemini-quint/dense-mem/commands.tsv
wait
