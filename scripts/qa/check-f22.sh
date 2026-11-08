#!/usr/bin/env bash
# F22: tree node row probe (task 2.13, tree.md T0, SRC-017, SRC-011).
# Checks the budgets.json `tree-node-rows` record: rows and k for the three slot-node shapes,
# k consistent with rows, and a verdict that follows the kill rule rows > 0.95 x 2^20.
# The probe itself (crates/mori-recursion/tests/tree_node_rows.rs, ignored, ~150 s debug) reruns with F22_RUN=1.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
probe=crates/mori-recursion/tests/tree_node_rows.rs
[ -f "$probe" ] || { echo "F22: $probe missing"; exit 1; }
if [ "${F22_RUN:-0}" = 1 ]; then
  cargo test -p mori-recursion --features sherlock-recursion --test tree_node_rows --locked -- --ignored --nocapture --test-threads=1
fi
python3 - <<'PY'
import json, math, sys
e = json.load(open("qa/sherlock/budgets.json"))["budgets"].get("tree-node-rows")
if not e: sys.exit("F22: budgets.json has no tree-node-rows entry")
shapes, verdict = e.get("shapes") or {}, e.get("verdict") or {}
kill = math.floor(0.95 * 2**20)
if verdict.get("killRows") != kill: sys.exit(f"F22: killRows {verdict.get('killRows')} != {kill}")
over = []
for name in ("self+self+leaf", "self+self", "self+self+leaf+leaf"):
    s = shapes.get(name)
    if not s or not isinstance(s.get("rows"), int) or not isinstance(s.get("k"), int):
        sys.exit(f"F22: shape {name} lacks integer rows and k")
    if not (2 ** (s["k"] - 1) < s["rows"] <= 2 ** s["k"]):
        sys.exit(f"F22: shape {name}: rows {s['rows']} not in (2^{s['k']-1}, 2^{s['k']}]")
    if s["rows"] > kill: over.append(name)
    print(f"F22: {name}: {s['rows']} rows, k={s['k']}")
# The tree needs the {self, self, leaf} slot node; the kill rule applies to it.
want = "killed" if "self+self+leaf" in over else "kept"
if verdict.get("tree") != want: sys.exit(f"F22: verdict {verdict.get('tree')!r}, kill rule gives {want!r}")
print(f"F22 PASS: tree {want} (kill at > {kill} rows)")
PY
