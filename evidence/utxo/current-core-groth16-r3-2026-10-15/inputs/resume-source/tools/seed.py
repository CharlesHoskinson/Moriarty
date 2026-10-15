"""Seed this resume's own candidate/ and review-overlay/ from the frozen public partial.

Stdlib only. Source list = PUBLIC-PARTIAL-TREE-MANIFEST.json (fd38e491...) entries of the
terminated task; every source byte is hashed against its manifest pin before copy and
the copy is hashed again. Destinations must not exist (O_EXCL). The terminated job's
top-level records/tools/diffs are copied under inherited/old-partial/ as history.
Nothing is written outside the resume directory; no source file is modified.
Usage: python3 -I tools/seed.py records/SEED-RECORD.json
"""
import datetime, hashlib, json, os, resource, sys

resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))

BASE = "/home/charl/moriarty-eutxo-validation-2026-10-12"
OWN = BASE + "/current-core-groth16-evidence-repair-resume-source-20261015"
OLD = BASE + "/current-core-groth16-evidence-repair-source-20261015"
MAN = BASE + "/opus-session-limit-recovery-20261015/PUBLIC-PARTIAL-TREE-MANIFEST.json"
MAN_SHA = "fd38e491ce4881bf298c82f79095bbba81884adabf61ba23f2f08463554d9aee"
REC = BASE + "/opus-session-limit-recovery-20261015/RECOVERY.json"


def sha(b):
    return hashlib.sha256(b).hexdigest()


def main():
    dst_rec = os.path.abspath(sys.argv[1])
    assert dst_rec.startswith(OWN + "/records/")
    mb = open(MAN, "rb").read()
    assert sha(mb) == MAN_SHA, "manifest changed"
    man = json.loads(mb)
    plan = []
    for f in man["files"]:
        if not f["path"].startswith(OLD + "/"):
            continue
        rel = f["path"][len(OLD) + 1:]
        if rel.startswith("candidate/") or rel.startswith("review-overlay/"):
            dest = rel
        else:
            dest = "inherited/old-partial/" + rel
        plan.append((f["path"], dest, f["sha256"], f["bytes"]))
    for j in json.load(open(REC))["jobs"]:
        if j["task"] != "current-core-groth16-evidence-repair-source-20261015":
            continue
        for p in j["publicPartialPins"]:
            rel = p["path"][len(OLD) + 1:]
            dest = "inherited/old-partial/" + rel
            if dest not in [d for _, d, _, _ in plan]:
                plan.append((p["path"], dest, p["sha256"], p["bytes"]))
    rows, stops = [], []
    for src, dest, want, size in plan:
        b = open(src, "rb").read()
        if sha(b) != want or len(b) != size or os.path.islink(src):
            stops.append("source differs from pin: " + src)
            continue
        out = os.path.join(OWN, dest)
        os.makedirs(os.path.dirname(out), exist_ok=True)
        fd = os.open(out, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
        with os.fdopen(fd, "wb") as fh:
            fh.write(b)
        after = sha(open(out, "rb").read())
        if after != want:
            stops.append("post-copy differs: " + dest)
        rows.append({"dest": dest, "source": src, "sha256": want, "bytes": size})
    counts = {}
    for r in rows:
        top = r["dest"].split("/")[0] if not r["dest"].startswith("inherited/") else "inherited"
        counts[top] = counts.get(top, 0) + 1
    rec = {"format": "evidence-repair-resume-seed/1", "at_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
           "manifest": MAN, "manifest_sha256": MAN_SHA, "counts": counts, "stops": stops,
           "rule": "Byte-identical copies of the terminated job's pinned public partial files. candidate/ and review-overlay/ seed this resume's own candidate (UNACCEPTED partial work, preserved before additions). Everything else is history under inherited/old-partial/.",
           "files": rows, "verdict": "PASS" if not stops else "STOP"}
    fd = os.open(dst_rec, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
    with os.fdopen(fd, "w") as fh:
        json.dump(rec, fh, indent=1, sort_keys=True)
        fh.write("\n")
    print(rec["verdict"], counts, stops[:3])


if __name__ == "__main__":
    main()
