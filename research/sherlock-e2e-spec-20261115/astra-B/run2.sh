#!/bin/bash
cd ~/Projects/Moriarty-wt/sherlock-e2e-spec
R=research/sherlock-e2e-spec-20261115/astra-B
for i in 1 2 3 4 5 6; do
  timeout 3600 codex exec resume 01a1e017-5d8b-7381-943f-784aed2e270d -m gpt-6-astra -c model_reasoning_effort=high -c sandbox_mode="workspace-write" \
    -o $R/phase2.md "$(cat $R/PROMPT2.md)" < /dev/null > $R/phase2.log 2>&1
  rc=$?
  if grep -qi "at capacity" $R/phase2.log && [ ! -s $R/REVIEW.md ]; then sleep 120; continue; fi
  break
done
echo "rc=$rc" >> $R/phase2.log
