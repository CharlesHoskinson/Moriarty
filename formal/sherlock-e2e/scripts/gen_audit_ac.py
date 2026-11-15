#!/usr/bin/env python3
"""gen_audit.py CTX NS : write <ctx>/lean/<NS>/Audit.lean with pinned #print axioms of every
theorem in <NS>/*.lean (except Audit), using the messages Lean prints on odin.
Refuses (exit 1) when an axiom outside the allowlist or sorryAx appears."""
import re, subprocess, sys, pathlib
ctx, ns = sys.argv[1], sys.argv[2]
root = pathlib.Path("/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e") / ctx / "lean"
mods = sorted(p for p in (root / ns).glob("*.lean") if p.stem != "Audit")
names = []
for p in mods:
    cur = []
    for line in p.read_text().splitlines():
        m = re.match(r"^\s*namespace (\S+)", line)
        if m: cur.append(m.group(1))
        m = re.match(r"^\s*end (\S+)\s*$", line)
        if m and cur and cur[-1] == m.group(1): cur.pop()
        m = re.match(r"^(?:@\[[^\]]*\]\s*)?(?:private\s+)?theorem\s+(\S+)", line)
        if m: names.append(".".join(cur + [m.group(1)]))
imports = "import SherlockCodec.AuditCmd\n" + "".join(f"import {ns}.{p.stem}\n" for p in mods)
probe = imports + "".join(f"#print axioms {n}\n" for n in names)
(root / ns / "Audit.lean").write_text(probe)
subprocess.run(["rsync", "-a", "--exclude", ".lake", str(root.parent.parent) + "/", "odin:sherlock/formal-e2e/"], check=True)
out = subprocess.run(["ssh", "odin", f"cd ~/sherlock/formal-e2e/{ctx}/lean && PATH=$HOME/.elan/bin:$PATH flock ~/sherlock/formal-e2e/.build.lock nice -n 10 lake build SherlockCodec.AuditCmd {' '.join(ns + '.' + p.stem for p in mods)} >/dev/null 2>&1; PATH=$HOME/.elan/bin:$PATH nice -n 10 lake env lean {ns}/Audit.lean"], capture_output=True, text=True).stdout
msgs = [l.strip() for l in out.splitlines() if re.match(r"^'\S+' (depends on axioms: \[.*\]|does not depend on any axioms)$", l.strip())]
if len(msgs) != len(names):
    print(out[:3000]); sys.exit(f"expected {len(names)} messages, got {len(msgs)}")
allowed = {"propext", "Classical.choice", "Quot.sound"}
bad = []
body = [imports, f"\n/-! # Audit of {ns}\n\n`#print axioms` of every theorem, pinned with `#guard_msgs` (allowlist: propext,\nClassical.choice, Quot.sound), and `#audit_ns {ns}` pinned at `[]`. -/\n\n/-- info: [] -/\n#guard_msgs in\n#audit_ns {ns}\n"]
for n, m in zip(names, msgs):
    ax = re.search(r"\[(.*)\]", m)
    used = set(a.strip() for a in ax.group(1).split(",")) if ax else set()
    if used - allowed:
        bad.append((n, used - allowed))
    body.append(f"\n/-- info: {m} -/\n#guard_msgs in\n#print axioms {n}\n")
(root / ns / "Audit.lean").write_text("".join(body))
print(f"{len(names)} theorems pinned")
if bad:
    print("BAD AXIOMS", bad); sys.exit(1)
