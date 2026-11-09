#!/usr/bin/env bash
# Build the 2.14 council review bundle (copies of the inputs). Scratch only; not committed.
set -euo pipefail
R=/home/hoskinson/Projects/Moriarty/.claude/worktrees/agent-a371509e1f28f47db
M=/home/hoskinson/Projects/Moriarty
B=/home/hoskinson/.cache/council-2.14/bundle
C="$R/openspec/changes/sherlock-vm-consensus-20261107"
rm -rf "$B"
mkdir -p "$B"/design "$B"/model "$B"/lean "$B"/research "$B"/qa/ivc "$B"/qa/c6 "$B"/pilot "$B"/code
cp "$C/design.md" "$C/requirements.md" "$C/assurance.md" "$B/design/"
sed -n '30,45p;60,70p;80,86p' "$C/tasks.md" > "$B/design/tasks-excerpt.md"
cp "$R/models/sherlock/quint/aggregation.qnt" "$R/models/sherlock/quint/aggregation_test.qnt" "$R/scripts/qa/check-f20.sh" "$B/model/"
cp "$M"/qa/sherlock/traces/20261108T*-F20.log "$B/model/"
cp "$R/clean/fork/Clean/Sherlock/Aggregation.lean" "$R/clean/fork/Clean/Sherlock/AggregationAxioms.lean" "$R/scripts/qa/check-lean-aggregation.sh" "$B/lean/"
cp "$M"/qa/sherlock/traces/20261108T*-F21.log "$B/lean/"
cp "$R"/research/sherlock-speed-replan-20261108/{tree,prover,order}.md "$B/research/"
cp "$R"/research/skeleton-coding-20261108/{builds,testing,review}.md "$B/research/"
cp "$R"/qa/gemini-quint/ivc/* "$B/qa/ivc/"
cp "$R"/qa/gemini-quint/c6/* "$B/qa/c6/"
cp "$R"/research/skeleton-coding-20261108/{pilot-smoke,pilot-amendment-1,pilot-protocol}.md "$B/pilot/"
cp "$R/crates/mori-recursion/src/ivc.rs" "$B/code/"
ls -R "$B"
