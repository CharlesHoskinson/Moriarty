"""Verify public partial pins (manifest + RECOVERY) against current bytes. Read-only, stdlib only.

Writes one JSON record (argv[1]) inside the resume task directory. Hashes only
the pinned public files; unpinned files in the old tree are listed by name only
(no content read/hash). Private/provider/stderr names are skipped entirely.
"""
import hashlib, json, os, sys

BASE = "/home/charl/moriarty-eutxo-validation-2026-10-12"
MAN = BASE + "/opus-session-limit-recovery-20261015/PUBLIC-PARTIAL-TREE-MANIFEST.json"
REC = BASE + "/opus-session-limit-recovery-20261015/RECOVERY.json"
TASK = "current-core-groth16-evidence-repair-source-20261015"
OLD = BASE + "/" + TASK
OWN = BASE + "/current-core-groth16-evidence-repair-resume-source-20261015"
SKIP_MARKERS = ("private", "stderr", "reasoning", "session")


def h(p):
    b = open(p, "rb").read()
    return len(b), hashlib.sha256(b).hexdigest()


def main():
    dst = os.path.abspath(sys.argv[1])
    assert dst.startswith(OWN + "/"), dst
    man = json.load(open(MAN))
    rec = json.load(open(REC))
    out = {"manifest_path": MAN, "manifest_sha256": h(MAN)[1], "rows": [], "unpinned_present_names_only": []}
    pinned = set()
    for f in man["files"]:
        if f["task"] != TASK:
            continue
        pinned.add(f["path"])
        rel = f["path"][len(OLD) + 1:]
        if not os.path.isfile(f["path"]):
            out["rows"].append({"path": rel, "status": "MISSING"})
            continue
        n, d = h(f["path"])
        ok = (n, d) == (f["bytes"], f["sha256"])
        out["rows"].append({"path": rel, "bytes": n, "sha256": d, "status": "MATCH" if ok else "MISMATCH"})
    for j in rec["jobs"]:
        if j["task"] != TASK:
            continue
        for p in j["publicPartialPins"]:
            pinned.add(p["path"])
            for key in ("path", "snapshot"):
                n, d = h(p[key])
                ok = (n, d) == (p["bytes"], p["sha256"])
                out["rows"].append({"path": p[key], "pin_source": "RECOVERY.json:" + key, "bytes": n, "sha256": d,
                                    "status": "MATCH" if ok else "MISMATCH"})
    for root, dirs, files in os.walk(OLD):
        for fn in files:
            p = os.path.join(root, fn)
            if any(m in fn.lower() for m in SKIP_MARKERS):
                continue
            if p not in pinned:
                out["unpinned_present_names_only"].append(p[len(OLD) + 1:])
    out["unpinned_present_names_only"].sort()
    summary = {}
    for r in out["rows"]:
        summary[r["status"]] = summary.get(r["status"], 0) + 1
    out["summary"] = summary
    with open(dst, "w") as fh:
        json.dump(out, fh, indent=1)
        fh.write("\n")
    print(summary, "unpinned:", len(out["unpinned_present_names_only"]))
    for r in out["rows"]:
        if r["status"] != "MATCH":
            print(r)
    for n in out["unpinned_present_names_only"]:
        print("UNPINNED", n)


if __name__ == "__main__":
    main()
