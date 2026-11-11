#!/usr/bin/env bash
# Lead re-run of the C1 dense-memory Quint lane. Every quint call runs in heavy.slice.
# Reads commands.tsv (n, expected exit, command); prints exit, expected, verdict tag, command.
set -u
cd "$(dirname "$0")/../../.."
out="${1:?outdir}"; mkdir -p "$out"
while IFS=$'\t' read -r n exp cmd; do
  log="$out/$(printf '%02d' "$n").log"
  eval "heavy-run --mem 4G -- $cmd" >"$log" 2>&1; rc=$?
  tag=other
  grep -q '\[violation\]' "$log" && tag='[violation]'
  grep -q '\[ok\]' "$log" && tag='[ok]'
  grep -q 'passing' "$log" && tag="$(grep -Eo '[0-9]+ passing' "$log" | tail -1)"
  grep -Eq '[0-9]+ failed' "$log" && tag="$tag; $(grep -Eo '[0-9]+ failed' "$log" | tail -1)"
  ok=MISMATCH; [ "$rc" = "$exp" ] && ok=match
  echo "$ok exit=$rc expected=$exp $tag :: $cmd"
done < qa/gemini-quint/c1-dense/commands.tsv
