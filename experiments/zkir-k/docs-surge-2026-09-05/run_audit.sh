#!/usr/bin/env bash
# run_audit.sh VENDOR ROLE NN  — audit chapter NN as ROLE (dev|fm) with grok or codex
set -u
VENDOR=$1; ROLE=$2; NN=$3
ROOT=/home/charl/Moriarty/.worktrees/zkir-k-semantics
D=$ROOT/experiments/zkir-k/docs-surge-2026-09-05
FILE=$(grep -o '^Write `experiments/zkir-k/docs/[^`]*`' $D/briefs/$NN.md | sed 's/.*docs\///; s/`//')
REPORT=experiments/zkir-k/docs-surge-2026-09-05/audits/$NN-$ROLE-$VENDOR.md
P=$D/prompts/$NN-$ROLE-$VENDOR.audit.prompt
{
  echo "You are auditing one chapter of technical documentation inside the repository at $ROOT (your working directory). Relative paths are relative to that root. Chapter: experiments/zkir-k/docs/$FILE. Chapter brief: experiments/zkir-k/docs-surge-2026-09-05/briefs/$NN.md. Write your report to $REPORT and nothing else. Proceed without asking questions."
  echo; echo "=================== AUDIT BRIEF ==================="; cat $D/briefs/AUDIT-COMMON.md
  echo; cat $D/briefs/AUDIT-$ROLE.md
  echo; echo "=================== COMMON DRAFTING BRIEF (the rules the chapter had to follow) =================="; cat $D/briefs/COMMON.md
  echo; echo "=================== CHAPTER BRIEF =================="; cat $D/briefs/$NN.md
  echo; echo "When the report is written, reply with one line: DONE $REPORT <verdict>."
} > $P
cd $ROOT
START=$(date +%s)
case $VENDOR in
  grok)
    GROK_HOME=$HOME/.foreman/credential-profiles/grok-default/homes/grok \
    timeout 3000 grok --prompt-file $P -m grok-4.6 --output-format json --always-approve --no-subagents --disable-web-search --verbatim --max-turns 150 --cwd $ROOT < /dev/null > $D/logs/$NN-$ROLE-grok.json 2> $D/logs/$NN-$ROLE-grok.err; RC=$? ;;
  codex)
    # read-only sandbox cannot write the report: capture the last message instead
    timeout 3000 codex exec -C $ROOT --model gpt-6-astra -c model_reasoning_effort=high --sandbox read-only --skip-git-repo-check --json -o $D/audits/$NN-$ROLE-codex.md < <(sed "s|Write your report to $REPORT and nothing else|Your final message must be the complete report in the format below (it is captured to $REPORT); do not write files|") $P > $D/logs/$NN-$ROLE-codex.events.jsonl 2> $D/logs/$NN-$ROLE-codex.err; RC=$? ;;
esac
echo "$NN $ROLE $VENDOR exit $RC $(( $(date +%s) - START ))s report=$( [ -f $ROOT/$REPORT ] && grep -m1 -o 'Verdict: [A-Z]*' $ROOT/$REPORT || echo missing)" >> $D/logs/audits.status
