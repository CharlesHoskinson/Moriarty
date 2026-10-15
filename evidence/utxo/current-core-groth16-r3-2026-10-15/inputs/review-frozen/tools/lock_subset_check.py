#!/usr/bin/env python3
"""PROPOSED E1v tool (reviewer-authored, stdlib only, read-only).
STOP predicate for a generated Cargo.lock: every registry package (name, version,
checksum) must appear in LOCK-UNION-ANALYSIS.json; every non-registry package must be
one of the allowed local path packages; no git sources; no midnight-* crates.
Exit 0 = pass, 3 = STOP. Usage: python3 -I lock_subset_check.py CARGO_LOCK LOCK_UNION_JSON [ALLOWED_LOCAL ...]"""
import resource, sys, json, tomllib, hashlib
resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))
lock_path, union_path = sys.argv[1:3]
allowed_local = set(sys.argv[3:]) or {"mori-r1cs", "mori-r1cs-heldout", "mori-core", "mori-lexer", "mori-diagnostics", "mori-span"}
raw = open(lock_path, "rb").read()
lock = tomllib.loads(raw.decode())
union = {(p["name"], p["version"]): p.get("checksum") for p in json.load(open(union_path))["packages"]
         if (p.get("source") or "").startswith("registry+")}
problems = []
for p in lock.get("package", []):
    src = p.get("source")
    if src is None:
        if p["name"] not in allowed_local:
            problems.append(f"unexpected local package {p['name']}")
        continue
    if src.startswith("git+") or "midnight" in p["name"]:
        problems.append(f"forbidden source/package {p['name']} {src}")
        continue
    k = (p["name"], p["version"])
    if k not in union:
        problems.append(f"outside union {p['name']}@{p['version']}")
    elif union[k] != p.get("checksum"):
        problems.append(f"checksum differs {p['name']}@{p['version']}")
print(json.dumps({"lock": lock_path, "lock_sha256": hashlib.sha256(raw).hexdigest(), "packages": len(lock.get("package", [])),
                  "problems": problems, "verdict": "PASS" if not problems else "STOP"}, indent=1))
sys.exit(0 if not problems else 3)
