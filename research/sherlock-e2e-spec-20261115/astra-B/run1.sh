#!/bin/bash
cd ~/Projects/Moriarty-wt/sherlock-e2e-spec
R=research/sherlock-e2e-spec-20261115/astra-B
for i in 1 2 3 4 5 6; do
  timeout 3600 codex exec -m gpt-6-astra -c model_reasoning_effort=high -s workspace-write \
    -C ~/Projects/Moriarty-wt/sherlock-e2e-spec -o $R/phase1.md \
    "$(cat $R/PROMPT1.md)" < /dev/null > $R/phase1.log 2>&1
  rc=$?
  if grep -qi "at capacity" $R/phase1.log && [ ! -s $R/DRAFT.md ]; then sleep 120; continue; fi
  break
done
echo "rc=$rc" >> $R/phase1.log
