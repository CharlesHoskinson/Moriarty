#!/usr/bin/env python3
"""Reviewer-owned read-only verification of the candidate's pin/manifest files.
Usage: python3 -I verify_pins.py CANDIDATE_DIR CHECKOUT_DIR OUT_JSON"""
import resource, sys, os, json, hashlib
resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))
cand, checkout, out = sys.argv[1:4]
res = {}


def sha(p):
    with open(p, "rb") as f:
        b = f.read()
    return hashlib.sha256(b).hexdigest(), len(b)


# 1. Core closure pins vs live checkout, plus the closure digest rule.
pins = json.load(open(os.path.join(cand, "CORE-CLOSURE-PINS.json")))
mism, ok = [], 0
for f in pins["files"]:
    p = os.path.join(checkout, f["path"])
    if not os.path.exists(p):
        mism.append({"path": f["path"], "issue": "missing"})
        continue
    h, n = sha(p)
    if (h, n) != (f["sha256"], f["bytes"]):
        mism.append({"path": f["path"], "pinned": f["sha256"], "live": h})
    else:
        ok += 1
compact = json.dumps(sorted(({"path": f["path"], "sha256": f["sha256"], "bytes": f["bytes"]} for f in pins["files"]),
                            key=lambda x: x["path"]), sort_keys=True, separators=(",", ":")).encode()
res["core_closure"] = {"files": len(pins["files"]), "live_match": ok, "mismatches": mism,
                       "recomputed_closure_sha256": hashlib.sha256(compact).hexdigest(),
                       "claimed_closure_sha256": pins["closure_sha256"]}
# completeness of the closure rule: every file under the 4 crates except excluded dirs/README
expected = set()
for c in pins["crates"]:
    base = os.path.join(checkout, "crates", c)
    for dp, dns, fns in os.walk(base):
        rel = os.path.relpath(dp, base)
        top = rel.split(os.sep)[0]
        if top in ("tests", "examples", "benches", "target"):
            continue
        for fn in fns:
            r = os.path.relpath(os.path.join(dp, fn), checkout)
            if fn == "README.md" and dp == base:
                continue
            expected.add(r)
expected |= {"Cargo.toml", "rust-toolchain.toml"}
pinned = {f["path"] for f in pins["files"]}
res["core_closure"]["rule_missing_from_pins"] = sorted(expected - pinned)
res["core_closure"]["pins_outside_rule"] = sorted(pinned - expected)

# 2. Build manifest and source manifest vs candidate bytes.
for name in ("BUILD-MANIFEST.json", "SOURCE-MANIFEST.json"):
    m = json.load(open(os.path.join(cand, name)))
    files = m.get("files", [])
    bad = []
    for f in files:
        p = os.path.join(cand, f["path"])
        if not os.path.exists(p):
            bad.append({"path": f["path"], "issue": "missing"})
            continue
        h, n = sha(p)
        if h != f["sha256"] or ("bytes" in f and n != f["bytes"]):
            bad.append({"path": f["path"], "listed": f["sha256"], "actual": h})
    res[name] = {"sha256": sha(os.path.join(cand, name))[0], "entries": len(files), "mismatches": bad,
                 "keys": sorted(m.keys())}
# source files present but not in BUILD-MANIFEST
src = os.path.join(cand, "source", "mori-r1cs")
on_disk = set()
for dp, dns, fns in os.walk(src):
    for fn in fns:
        on_disk.add(os.path.relpath(os.path.join(dp, fn), cand))
bm = {f["path"] for f in json.load(open(os.path.join(cand, "BUILD-MANIFEST.json")))["files"]}
res["BUILD-MANIFEST.json"]["src_or_cargo_not_listed"] = sorted(p for p in on_disk if (p.endswith("Cargo.toml") or "/src/" in p) and p not in bm)
sm = {f["path"] for f in json.load(open(os.path.join(cand, "SOURCE-MANIFEST.json"))).get("files", [])}
all_files = set()
for dp, dns, fns in os.walk(cand):
    for fn in fns:
        all_files.add(os.path.relpath(os.path.join(dp, fn), cand))
res["SOURCE-MANIFEST.json"]["files_not_listed"] = sorted(all_files - sm)
with open(out, "w") as f:
    json.dump(res, f, indent=2, sort_keys=True)
    f.write("\n")
print(json.dumps({k: {kk: vv for kk, vv in v.items() if kk not in ("mismatches",)} | {"mismatch_count": len(v.get("mismatches", []))} for k, v in res.items()}, indent=1))
