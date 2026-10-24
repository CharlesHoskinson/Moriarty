#!/usr/bin/env bash
# F8: the k20 parent budget has a recorded measurement and a cap.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
python3 -I - <<'PY'
import json, sys
b = json.load(open("qa/sherlock/budgets.json"))["budgets"].get("a3-parent-k20")
if b is None: print("no a3-parent-k20 budget"); sys.exit(1)
m = b["measured"]
if m.get("wallSeconds") is None or m.get("maxRssBytes") is None or m.get("k") != 20:
    print("a3-parent-k20 not measured:", m); sys.exit(1)
c = b["cap"]
if c.get("wallSeconds") is None or c.get("maxRssBytes") is None:
    print("a3-parent-k20 measured but uncapped:", c); sys.exit(1)
print("a3-parent-k20 measured and capped:", m, c)
PY
