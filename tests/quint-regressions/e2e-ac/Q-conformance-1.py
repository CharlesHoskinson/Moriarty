#!/usr/bin/env python3
"""Q-conformance-1 regression (ECS-003, ECS-012).
Pinned: Moriarty worktree t23-suite @ 2594f2b95, tools/t23-suite/t23.py table() (line 575).
Asserts the CORRECT behaviour: the suite-table mismatch line never reports more mismatches
than calls it says it compared ("n of m" with n <= m), or reports compile-stage mismatches
apart from post-proof ones. One compile-stage mismatch (compare stage never ran, ECS-003
scenario) and no proved call must not print "Mismatch: 1 of 0 compared after a proof".
Exit 0 = correct, 1 = defect present."""
import importlib.util, pathlib, re, subprocess, sys
sys.dont_write_bytecode = True
T23 = pathlib.Path.home() / "Projects/Moriarty-wt/t23-suite/tools/t23-suite/t23.py"
PIN = "2594f2b95"
head = subprocess.run(["git", "-C", str(T23.parent), "rev-parse", "--short=9", "HEAD"], capture_output=True, text=True).stdout.strip()
print(f"t23-suite HEAD {head} (pinned {PIN})")
sys.path.insert(0, str(T23.parent))
spec = importlib.util.spec_from_file_location("t23", T23); m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
row = {"id": "t01-tiny", "label": "get-set", "circuit": "set", "native_outcome": "ok", "native_preview": None,
       "native_preview_pass": False, "compile": "different", "prove": None, "settle": None, "compare": None,
       "precheck": None, "result": "mismatch", "reason": "compile stage: different"}  # as classify() t23.py:494
meta = {"mode": "dry", "started": "-", "finished": "-", "prover": "stub", "settle": "dry", "preview_tx_sent": 0,
        "native_preview": "-", "compiler": {"branch": "-", "head": "0" * 9, "dirty_paths": 0, "tree_sha256": "0" * 16}, "build": {}}
tot, md = m.table([row], meta)
line = next((l for l in md.splitlines() if l.startswith("Mismatch:")), "")
print(line or "(no Mismatch: line)")
g = re.match(r"Mismatch: (\d+) of (\d+)", line)
if g and int(g.group(1)) > int(g.group(2)):
    print("FAIL: n > m; the compile-stage mismatch is counted against the post-proof denominator"); sys.exit(1)
print("PASS"); sys.exit(0)
