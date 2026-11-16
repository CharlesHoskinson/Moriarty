#!/bin/bash
cd ~/Projects/Moriarty-wt/sherlock-e2e-spec
D=research/sherlock-e2e-spec-20261115/astra-A
for i in 1 2 3 4 5 6; do
  timeout 3600 codex exec resume 01a1e017-6dff-76a0-9807-21c8d69d7b1f -c model_reasoning_effort=high \
    "$(cat $D/PROMPT2.md)" < /dev/null > $D/phase2.log 2>&1
  rc=$?
  if grep -qi "at capacity" $D/phase2.log && [ ! -s $D/REVIEW.md ]; then sleep 120; continue; fi
  break
done
tail -c 1500 $D/phase2.log > $D/phase2.md
echo "rc=$rc"
