#!/usr/bin/env python3
"""PROPOSED E1hv comparator (repair-author, stdlib only, read-only; needs review before use).

Predicate: for every registry package NAME present in both Cargo.lock files, the set of
(version, checksum) pairs must be identical in both locks. A crate resolved to different
versions or checksums for the candidate and the reviewer harness would make the two
builds disagree about the code under test. Path and git packages are ignored here (the
subset check E1v/E1hv already constrains them).

Exit 0 = SAME, 3 = DIFFER. Usage: python3 -I lock_pair_compare.py LOCK_A LOCK_B
"""
import hashlib
import json
import resource
import sys
import tomllib

resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))


def registry(path):
    raw = open(path, "rb").read()
    out = {}
    for p in tomllib.loads(raw.decode()).get("package", []):
        if (p.get("source") or "").startswith("registry+"):
            out.setdefault(p["name"], set()).add((p["version"], p.get("checksum")))
    return raw, out


raw_a, a = registry(sys.argv[1])
raw_b, b = registry(sys.argv[2])
shared = sorted(set(a) & set(b))
differ = [{"name": n, "a": sorted(a[n]), "b": sorted(b[n])} for n in shared if a[n] != b[n]]
print(json.dumps({"lock_a_sha256": hashlib.sha256(raw_a).hexdigest(), "lock_b_sha256": hashlib.sha256(raw_b).hexdigest(),
                  "shared_names": len(shared), "only_a": sorted(set(a) - set(b)), "only_b": sorted(set(b) - set(a)),
                  "differ": differ, "verdict": "SAME" if not differ else "DIFFER"}, indent=1))
sys.exit(0 if not differ else 3)
