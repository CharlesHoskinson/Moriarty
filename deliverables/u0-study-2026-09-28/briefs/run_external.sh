#!/usr/bin/env bash
D=/home/charl/Moriarty/deliverables/u0-study-2026-09-28; R=/home/charl/Moriarty
run_sol() { L=$1; P=$(cat $D/briefs/common.md $D/briefs/$L.md); s=$(date +%s)
  echo "$P" | timeout 3600 codex exec -m gpt-6-sol -c model_reasoning_effort="xhigh" -s read-only --skip-git-repo-check -C $R -o $D/gpt6sol-$L.md - > $D/logs/gpt6sol-$L.log 2>&1
  rc=$?; if [ ! -s $D/gpt6sol-$L.md ] && grep -qi "reasoning_effort\|unsupported\|invalid" $D/logs/gpt6sol-$L.log; then
    echo "$P" | timeout 3600 codex exec -m gpt-6-sol -c model_reasoning_effort="high" -s read-only --skip-git-repo-check -C $R -o $D/gpt6sol-$L.md - >> $D/logs/gpt6sol-$L.log 2>&1; rc=$?; echo "(fell back to high)" >> $D/logs/gpt6sol-$L.log; fi
  echo "gpt6sol-$L rc=$rc secs=$(( $(date +%s)-s ))"; }
run_grok() { L=$1; cat $D/briefs/common.md $D/briefs/$L.md > $D/briefs/grok-$L.prompt; s=$(date +%s)
  timeout 3600 grok -m grok-4.7 --reasoning-effort xhigh --sandbox read-only --cwd $R --max-turns 200 --disable-web-search --output-format plain --prompt-file $D/briefs/grok-$L.prompt > $D/grok47-$L.md 2> $D/logs/grok47-$L.log
  echo "grok47-$L rc=$? secs=$(( $(date +%s)-s ))"; }
mkdir -p $D/logs
for L in L1 L2 L3; do run_sol $L & run_grok $L & done; wait; echo ALL_EXTERNAL_DONE
