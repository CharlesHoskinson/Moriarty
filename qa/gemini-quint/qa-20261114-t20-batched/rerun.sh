#!/usr/bin/env bash
# Lead re-run of a qa-20261114 lane. Usage: rerun.sh LANEDIR OUTDIR CMDFILE
# CMDFILE: one quint command per line (as in REPORT.md "## Commands", with or without heavy-run prefix).
# Prints: exit code, tag ([ok] / [violation] / N passing; M failed / other), command.
set -u
cd "$1"; out="$2"; mkdir -p "$out"; n=0
while IFS= read -r line; do
  [ -z "$line" ] && continue
  cmd="${line#heavy-run --mem 4G -- }"
  n=$((n+1)); log="$out/$(printf '%02d' $n).log"
  heavy-run --mem 4G -- bash -c "$cmd" >"$log" 2>&1; rc=$?
  tag=other
  grep -q '\[violation\]' "$log" && tag='[violation]'
  grep -q '\[ok\]' "$log" && tag='[ok]'
  grep -Eq '[0-9]+ passing' "$log" && tag="$(grep -Eo '[0-9]+ passing' "$log" | tail -1)"
  grep -Eq '[0-9]+ failed' "$log" && tag="$tag; $(grep -Eo '[0-9]+ failed' "$log" | tail -1)"
  [ "$tag" = other ] && echo "$cmd" | grep -q typecheck && [ $rc = 0 ] && tag=typechecked
  echo "exit=$rc $tag :: $cmd"
done < "$3"
