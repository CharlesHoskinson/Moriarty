"""Re-verify the old partial's PARENT-PINS (pinned public 15:53Z) against bytes read now.

Stdlib only, read-only outside the resume task directory. Every manifest listed in
PARENT-PINS is re-read and each of its entries is hashed (names containing
'private' / 'stderr' are skipped by name, never opened). Named files are hashed.
The live Core closure is recomputed by the CORE-CLOSURE-PINS rule. Writes argv[1].
"""
import hashlib, json, os, sys

BASE = "/home/charl/moriarty-eutxo-validation-2026-10-12"
OWN = BASE + "/current-core-groth16-evidence-repair-resume-source-20261015"
PINS = BASE + "/current-core-groth16-evidence-repair-source-20261015/PARENT-PINS.json"
PINS_SHA = "11a348a5cc24a0cb10d3fac8b7425dc0ffee932fcbc7827e2f9381af65be45d6"
CORE_CHECKOUT = "/home/charl/Moriarty/.worktrees/utxo-pr54-currentness-20261015"
SKIP = ("private", "stderr")


def sha_file(p):
    b = open(p, "rb").read()
    return len(b), hashlib.sha256(b).hexdigest()


def canonical(v):
    return json.dumps(v, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()


def manifest_entries(doc):
    """Yield (relpath, sha256, bytes) for the manifest shapes used in this lineage."""
    files = doc.get("files")
    if isinstance(files, list):
        for e in files:
            yield e["path"], e["sha256"], e.get("bytes")
    elif isinstance(files, dict):
        for k, e in files.items():
            if isinstance(e, dict):
                yield k, e["sha256"], e.get("bytes")
            else:
                yield k, e, None
    else:
        for key in ("entries", "outputs"):
            if isinstance(doc.get(key), list):
                for e in doc[key]:
                    yield e["path"], e["sha256"], e.get("bytes")
                return
            if isinstance(doc.get(key), dict):
                for k, e in doc[key].items():
                    if isinstance(e, dict):
                        yield k, e["sha256"], e.get("bytes")
                    else:
                        yield k, e, None
                return
        raise ValueError("unknown manifest shape: keys %s" % sorted(doc))


def main():
    dst = os.path.abspath(sys.argv[1])
    assert dst.startswith(OWN + "/")
    n, d = sha_file(PINS)
    assert d == PINS_SHA, ("PARENT-PINS changed", d)
    pins = json.load(open(PINS))
    out = {"parent_pins": {"path": PINS, "sha256": d, "bytes": n}, "expected": {}, "manifests": {}, "named": {}, "stops": []}
    for k, e in pins["expected"].items():
        n, d = sha_file(e["path"])
        ok = d == e["sha256"]
        out["expected"][k] = {"path": e["path"], "sha256": d, "equal": ok}
        if not ok:
            out["stops"].append("expected " + k)
    for k, m in pins["manifests"].items():
        mp = m["manifest"]
        n, d = sha_file(mp)
        res = {"manifest": mp, "sha256": d, "equal_manifest": d == m["manifest_sha256"], "entries": 0, "verified": 0, "mismatch": [], "skipped_by_name": []}
        root = os.path.dirname(mp)
        doc = json.load(open(mp))
        for rel, sha, size in manifest_entries(doc):
            res["entries"] += 1
            if any(s in rel.lower() for s in SKIP):
                res["skipped_by_name"].append(rel)
                continue
            p = rel if rel.startswith("/") else os.path.join(root, rel)
            if not os.path.isfile(p):
                res["mismatch"].append(rel + " (missing)")
                continue
            bn, bd = sha_file(p)
            if bd != sha or (size is not None and bn != size):
                res["mismatch"].append(rel)
            else:
                res["verified"] += 1
        out["manifests"][k] = res
        if not res["equal_manifest"] or res["mismatch"]:
            out["stops"].append("manifest " + k)
    for rel, e in pins["named"].items():
        n, d = sha_file(os.path.join(BASE, rel))
        ok = (n, d) == (e["bytes"], e["sha256"])
        out["named"][rel] = {"sha256": d, "bytes": n, "equal": ok}
        if not ok:
            out["stops"].append("named " + rel)
    # Live Core closure by the CORE-CLOSURE-PINS rule.
    cp = json.load(open(pins["core_closure"]["pins"]))
    rows, differ = [], []
    for f in cp["files"]:
        n, d = sha_file(os.path.join(CORE_CHECKOUT, f["path"]))
        rows.append({"path": f["path"], "sha256": d, "bytes": n})
        if (n, d) != (f["bytes"], f["sha256"]):
            differ.append(f["path"])
    rows.sort(key=lambda r: r["path"])
    closure = hashlib.sha256(canonical(rows)).hexdigest()
    out["core_closure"] = {"checkout": CORE_CHECKOUT, "files": len(rows), "recomputed": closure,
                           "pinned": cp["closure_sha256"], "equal": closure == cp["closure_sha256"] == "f47318a26a4df0477e89962a9de0e9b48f12d01d0e9b4a8f5e74603beaa235ed",
                           "rows_differing": differ, "rule": cp.get("closure_sha256_rule")}
    if not out["core_closure"]["equal"] or differ:
        out["stops"].append("core closure")
    out["core_closure_pins_rule"] = cp.get("closure_sha256_rule")
    out["verdict"] = "PASS" if not out["stops"] else "STOP"
    with open(dst, "w") as fh:
        json.dump(out, fh, indent=1, sort_keys=True)
        fh.write("\n")
    print(out["verdict"], out["stops"])
    for k, r in out["manifests"].items():
        print(k, r["entries"], r["verified"], len(r["skipped_by_name"]), r["mismatch"][:3])
    print("core closure", out["core_closure"]["files"], out["core_closure"]["recomputed"], out["core_closure"]["equal"])


if __name__ == "__main__":
    main()
