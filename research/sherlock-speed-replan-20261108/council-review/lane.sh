#!/usr/bin/env bash
# lane.sh <codex|agy|grok|muse> <smoke|review>
# Runs one council lane inside heavy-run (8G cap). Scratch under ~/.cache/council-2.14.
set -uo pipefail
unset OPENAI_API_KEY CODEX_API_KEY XAI_API_KEY
lane="$1"; mode="$2"
S=/home/hoskinson/.cache/council-2.14
B="$S/bundle"
O="$S/out/$lane-$mode"
mkdir -p "$O"
if [ "$mode" = smoke ]; then
  P="What is 1+1? Reply with exactly: 2"; T=300; WD="$S/smoke-wd"; mkdir -p "$WD"
else
  P="$(cat "$S/PROMPT.md")"; T=5400; WD="$B"
fi
cd "$WD"
start=$(date -u +%FT%TZ)
case "$lane" in
  codex)
    heavy-run --mem 8G --name "council-$lane" -- timeout "$T" \
      codex exec -m gpt-6.1-sol -c model_reasoning_effort=high -s read-only --ephemeral \
      --skip-git-repo-check -C "$WD" -o "$O/answer.md" "$P" < /dev/null > "$O/lane.log" 2>&1
    rc=$? ;;
  agy)
    heavy-run --mem 8G --name "council-$lane" -- timeout "$T" \
      agy --model gemini-4-argon-high --disable-slash-commands \
      --settings '{"permissions":{"deny":["command(*)","write_file(*)","execute_url(*)","mcp(*)"]}}' \
      --output-format json --print-timeout 85m --print="$P" < /dev/null > "$O/lane.json" 2> "$O/lane.err"
    rc=$?
    jq -r '.response // empty' "$O/lane.json" > "$O/answer.md" 2>/dev/null ;;
  grok)
    if [ "$mode" = smoke ]; then pf="$WD/smoke-prompt.md"; printf '%s\n' "$P" > "$pf"; else pf="$S/PROMPT.md"; fi
    heavy-run --mem 8G --name "council-$lane" -- timeout "$T" \
      grok -m grok-4.7 --reasoning-effort xhigh --prompt-file "$pf" --cwd "$WD" \
      --output-format json --no-subagents --disable-web-search --max-turns 80 --no-auto-update \
      < /dev/null > "$O/lane.json" 2> "$O/lane.err"
    rc=$?
    jq -r '.text // empty' "$O/lane.json" > "$O/answer.md" 2>/dev/null ;;
  muse)
    LW="$S/muse-ws"
    if [ "$mode" = review ]; then rm -rf "$LW"; mkdir -p "$LW"; cp -r "$B"/. "$LW/"; cp "$S/PROMPT.md" "$LW/.council-prompt.md"; pf="$LW/.council-prompt.md";
    else mkdir -p "$LW"; printf '%s\n' "$P" > "$LW/.smoke.md"; pf="$LW/.smoke.md"; fi
    MUSE_NO_AUTO_UPDATE=1 heavy-run --mem 8G --name "council-$lane" -- timeout "$T" \
      muse exec --json --model muse-wonder-full-1.0 --effort high \
      --workspace "$LW" --approval-mode never --contributor-mode off \
      --sandbox-network restricted --prompt-file "$pf" \
      < /dev/null > "$O/events.jsonl" 2> "$O/lane.err"
    rc=$?
    python3 -I - "$O/events.jsonl" > "$O/answer.md" <<'PY'
import json, sys
for line in open(sys.argv[1]):
    try: e = json.loads(line)
    except Exception: continue
    if e.get("payload_type") == "run.terminal.completed":
        print(e["payload"].get("text") or "")
PY
    ;;
esac
end=$(date -u +%FT%TZ)
echo "lane=$lane mode=$mode rc=$rc start=$start end=$end" | tee "$O/status.txt"
