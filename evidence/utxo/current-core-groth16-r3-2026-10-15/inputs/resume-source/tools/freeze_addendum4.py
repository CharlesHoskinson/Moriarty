"""Freeze addendum 4 (AUTH) before any r3 harness assertion of this resume is edited.

Stdlib only. Parses both JSON files (syntax check), checks every HW-01..26 id appears
exactly once in HW-TRACEABILITY-2 in frozen order, hashes both files and the inherited
addendum-3 files, and writes records/ADDENDUM-4-FREEZE.json exclusively.
"""
import datetime, hashlib, json, os, resource, sys

resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))

OWN = "/home/charl/moriarty-eutxo-validation-2026-10-12/current-core-groth16-evidence-repair-resume-source-20261015"
VEC = "/home/charl/moriarty-eutxo-validation-2026-10-12/current-core-groth16-compiler-independent-source-review-20261015/heldout/HELDOUT-VECTORS.json"
FILES = ["review-overlay/heldout-addendum-4/HELDOUT-ADDENDUM-4.json", "review-overlay/heldout-addendum-4/HW-TRACEABILITY-2.json",
         "review-overlay/heldout-addendum-3/HELDOUT-ADDENDUM-3.json", "review-overlay/heldout-addendum-3/HW-TRACEABILITY.json"]


def main():
    rec = {"format": "evidence-repair-addendum-4-freeze/1", "author": "claude-opus-5-5 (resume source author; AUTH)",
           "frozen_at_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
           "rule": "Written immediately after addendum 4 and HW-TRACEABILITY-2, before the r3 harness assertions of this resume were edited.",
           "files": {}, "checks": {}, "stops": []}
    for rel in FILES:
        b = open(os.path.join(OWN, rel), "rb").read()
        json.loads(b)
        rec["files"][rel] = {"sha256": hashlib.sha256(b).hexdigest(), "bytes": len(b)}
    frozen = [h["id"] for h in json.load(open(VEC))["hostile_witness_proposals"]]
    trace = json.load(open(os.path.join(OWN, FILES[1])))
    ids = [e["id"] for e in trace["entries"]]
    rec["checks"]["hw_ids_equal_frozen_order"] = ids == frozen
    classes = set(trace["classes"])
    rec["checks"]["classes_known"] = all(e["class"] in classes for e in trace["entries"])
    rec["checks"]["refs_well_formed"] = all(
        all(set(r) == {"table", "row"} for r in e["refs"]) and (e["refs"] or e["reason"]) for e in trace["entries"])
    for k, v in rec["checks"].items():
        if not v:
            rec["stops"].append(k)
    rec["verdict"] = "PASS" if not rec["stops"] else "STOP"
    out = os.path.join(OWN, "records/ADDENDUM-4-FREEZE.json")
    fd = os.open(out, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
    with os.fdopen(fd, "w") as fh:
        json.dump(rec, fh, indent=1, sort_keys=True)
        fh.write("\n")
    print(rec["verdict"], rec["checks"], {k: v["sha256"][:12] for k, v in rec["files"].items()})


if __name__ == "__main__":
    main()
