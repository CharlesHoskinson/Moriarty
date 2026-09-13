#!/bin/bash
set -euo pipefail
exec /home/charl/.local/bin/grok --resume 035bb25d-82ea-49e5-a042-bf1c628e0a39 --prompt-file /tmp/moriarty-production-switch-grok-20260913/correct-02-prompt.md --cwd /home/charl/Moriarty-wt-moriarty-release-afk-20260913-implement-production-executor -m grok-4.6 --reasoning-effort high --allow Write --allow Edit --always-approve --no-subagents --disable-web-search --verbatim --output-format streaming-messages-json
