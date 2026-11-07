#!/usr/bin/env bash
# Launch the three GPT-6 Astra council lanes (Codex, xhigh, network on, bus-writable roots).
set -u
C=/home/hoskinson/Projects/Moriarty/research/sherlock-vm-council-20261107
B=/home/hoskinson/wt/.bindings/sherlock-vm-council-20261107
for a in astra-1 astra-2 astra-3; do
  P="You are council member $a in the Sherlock VM design council. Your identity env file is $B/$a/.agentbus/env; source it in every shell command that calls agentbus (for example: . $B/$a/.agentbus/env && agentbus recv). Read $C/BRIEF.md completely now and follow it exactly: your lens is in its table, your writable folder is $C/$a/. The repository root is /home/hoskinson/Projects/Moriarty and you must not change files outside your folder. You may search the web and you may start helper lanes (for example: codex exec -m gpt-6-luna --ephemeral -s read-only --skip-git-repo-check ... < /dev/null) for bulk reading. Stay active through all four phases: between steps, poll the bus with 'agentbus wait --timeout 240' (never longer) and 'agentbus recv'. Do not end your session until the chair broadcasts DONE or the 2026-11-08 03:00Z cutoff passes; then write $C/$a/final.md and end with the final report the brief asks for."
  printf '%s' "$P" > "$C/_logs/$a.prompt"
  ( cd "$C/$a" && script -qefc "timeout 23400 codex exec -m gpt-6-astra -c model_reasoning_effort=xhigh \
      -c sandbox_workspace_write.network_access=true \
      -c 'sandbox_workspace_write.writable_roots=[\"/home/hoskinson/.cache/agentbus\",\"$B\",\"$C\"]' \
      --skip-git-repo-check -s workspace-write -C $C/$a -o $C/_logs/$a.last \
      \"\$(cat $C/_logs/$a.prompt)\" < /dev/null" /dev/null > "$C/_logs/$a.log" 2>&1 ) &
done
wait
echo "all astra lanes exited"
