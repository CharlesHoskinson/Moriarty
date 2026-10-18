#!/usr/bin/env python3
"""Reviewer-owned read-only check that the local cargo sparse-index cache has an
entry for every registry package in LOCK-UNION-ANALYSIS.json, and that the entry
lists the locked version. Reads files only; never runs cargo.
Usage: python3 -I index_cache_check.py LOCK_UNION_JSON INDEX_CACHE_DIR"""
import resource, sys, os, json
resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))
d = json.load(open(sys.argv[1]))
base = sys.argv[2]


def rel(name):
    n = name.lower()
    if len(n) == 1:
        return os.path.join("1", n)
    if len(n) == 2:
        return os.path.join("2", n)
    if len(n) == 3:
        return os.path.join("3", n[0], n)
    return os.path.join(n[:2], n[2:4], n)


missing, no_version, ok = [], [], 0
for p in d["packages"]:
    if not (p.get("source") or "").startswith("registry+"):
        continue
    path = os.path.join(base, rel(p["name"]))
    if not os.path.isfile(path):
        missing.append(p["name"])
        continue
    blob = open(path, "rb").read()
    if ('"vers":"%s"' % p["version"]).encode() not in blob:
        no_version.append(f"{p['name']}@{p['version']}")
    else:
        ok += 1
print(json.dumps({"index_cache_dir": base, "checked_ok": ok, "missing_entries": missing, "entry_without_locked_version": no_version}, indent=1))
