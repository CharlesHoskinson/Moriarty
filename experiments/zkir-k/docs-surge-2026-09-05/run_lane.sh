#!/usr/bin/env bash
# run_lane.sh VENDOR NN  — draft chapter NN with grok or codex; logs under docs-surge logs/
set -u
VENDOR=$1; NN=$2
ROOT=/home/charl/Moriarty/.worktrees/zkir-k-semantics
D=$ROOT/experiments/zkir-k/docs-surge-2026-09-05
FILE=$(ls $ROOT/experiments/zkir-k/docs-surge-2026-09-05/briefs/$NN.md >/dev/null && grep -o '^Write `experiments/zkir-k/docs/[^`]*`' $D/briefs/$NN.md | sed 's/.*docs\///; s/`//')
P=$D/prompts/$NN-$VENDOR.prompt
{
  echo "You are drafting one chapter of technical documentation inside the repository at $ROOT (your working directory). All relative paths below are relative to that root. You are authorized to read any file there and to run read-only commands; proceed without asking questions; finish the whole chapter in this session. The only file you may create or modify is experiments/zkir-k/docs/$FILE."
  echo; echo "=================== COMMON BRIEF ==================="; cat $D/briefs/COMMON.md
  echo; echo "=================== CHAPTER BRIEF =================="; cat $D/briefs/$NN.md
  echo; echo "When the chapter is written and verified, reply with one line: DONE experiments/zkir-k/docs/$FILE <word count>."
} > $P
cd $ROOT
START=$(date +%s)
case $VENDOR in
  grok)
    GROK_HOME=$HOME/.foreman/credential-profiles/grok-default/homes/grok \
    timeout 3000 grok --prompt-file $P -m grok-4.6 --output-format json --always-approve --no-subagents --disable-web-search --verbatim --max-turns 150 --cwd $ROOT < /dev/null > $D/logs/$NN-grok.json 2> $D/logs/$NN-grok.err; RC=$? ;;
  codex)
    timeout 3000 codex exec -C $ROOT --model gpt-6-astra -c model_reasoning_effort=high --sandbox workspace-write --skip-git-repo-check --json -o $D/logs/$NN-codex.last.md < $P > $D/logs/$NN-codex.events.jsonl 2> $D/logs/$NN-codex.err; RC=$? ;;
esac
echo "$NN $VENDOR exit $RC $(( $(date +%s) - START ))s file=$( [ -f $ROOT/experiments/zkir-k/docs/$FILE ] && wc -w < $ROOT/experiments/zkir-k/docs/$FILE || echo missing)" >> $D/logs/lanes.status
