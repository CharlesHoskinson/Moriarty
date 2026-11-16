#!/usr/bin/env bash
# X.3 hash-stack Quint run (lane and lead re-run). Every quint call runs in heavy.slice.
# Reads commands.tsv (n, expected exit, command); prints match/MISMATCH, exit, tag, command.
set -u
cd "$(dirname "$0")/../../.."
out="${1:?outdir}"; mkdir -p "$out"
while IFS=$'\t' read -r n exp cmd; do
  log="$out/$(printf '%02d' "$n").log"
  eval "heavy-run --mem 6G -- $cmd" >"$log" 2>&1; rc=$?
  tag=other
  grep -q '\[violation\]' "$log" && tag='[violation]'
  grep -q '\[ok\]' "$log" && tag='[ok]'
  grep -q 'No example found\|EXITCODE: OK\|outcome.*NoError' "$log" && tag='[verify ok]'
  ok=MISMATCH; [ "$rc" = "$exp" ] && ok=match
  echo "$ok exit=$rc expected=$exp $tag :: $cmd"
done < qa/gemini-quint/x3-hash-stacks/commands.tsv
