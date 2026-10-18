"""Pre-edit freeze of every unchanged input this resume reuses (stdlib only, read-only
outside the resume directory). Written BEFORE any harness assertion is edited.

Pins: the frozen independent reviewer packet (all REVIEW-MANIFEST entries), the r2
overlay (all R2-MANIFEST entries), the reviewer/run reports, and the inherited
public-partial records of the terminated job (financial-expectation freeze, addendum-3
freeze and files, parent pins, delta, the unfinished r3 harness). Each file is hashed
now; the record is created exclusively (O_EXCL) and never rewritten.
Usage: python3 -I tools/freeze_inputs.py records/PRE-EDIT-FREEZE.json
"""
import datetime, hashlib, json, os, resource, sys

resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))

BASE = "/home/charl/moriarty-eutxo-validation-2026-10-12"
OWN = BASE + "/current-core-groth16-evidence-repair-resume-source-20261015"
PKT = BASE + "/current-core-groth16-compiler-independent-source-review-20261015"
R2 = BASE + "/current-core-groth16-compiler-repair-independent-review-20261015/plumbing-r2"
OLD = BASE + "/current-core-groth16-evidence-repair-source-20261015"

MANIFESTS = {
    "packet_review_manifest": (PKT + "/REVIEW-MANIFEST.json", "757ed0bfc5804eec8bac3f1db063b1239ead0cdbeccbc5fa3ec3160a1a6ab151"),
    "r2_manifest": (R2 + "/R2-MANIFEST.json", "5a256c766908ff521b6a6cf483d1b109d17aa2d931a63598ab39b183cc2b580b"),
}
NAMED = {
    PKT + "/HELDOUT-FREEZE.json": "7a3b55f4716981f73315ecd11fabeb8d9e6d4251f5655e264f1636b68f683619",
    PKT + "/heldout/HELDOUT-VECTORS.json": "b72cec5a94219da8bd0959138430c0c6fb366712236280c064071e4ea87c2f1f",
    PKT + "/HOSTILE-CLAIMS-ADDENDUM.json": "bdc444016f9edefe5cd252e176c5ed302eaba2e0b3e5806ff5c68094e610e078",
    PKT + "/heldout-addendum-2/HELDOUT-ADDENDUM-2.json": "66b91c46806f2bad97e7b46c1026d9c724b27e40516ba01f10ec87acbc12f33f",
    BASE + "/current-core-groth16-independent-result-review-20261015/REPORT.md": "06e316bcf575c9258c292c29c84712ab2be7ff21be735add8c236f0be47d6ece",
    BASE + "/current-core-groth16-independent-result-review-20261015/proposals/FIX-PROPOSALS.md": "d7d3e40278a6e4a3a721003d0f6afd072f41ffa11d4bd254161f6e0a5fa2f4b7",
    BASE + "/current-core-groth16-env-corrected-offline-execution-20261015/REPORT.md": "18afb040ff7b961543f5b8dc478faa42044a4e4129f4f6dfed58349d1ac84cc1",
    BASE + "/current-core-groth16-env-corrected-offline-execution-20261015/tools/run_pathb.py": "c9cd72377f463b28cf2da791364b3f419018e5ff7ba560656983c207b28b28e1",
    BASE + "/current-core-groth16-env-corrected-offline-execution-20261015/tools/analyze.py": "8000f8bd186bb2860c7c0983c09dda265a85ae58b93b1bd338c3ca39001c72da",
    OLD + "/FINANCIAL-EXPECTATIONS-FREEZE.json": "79709ca75966cd9839f3928eaaa04cc69e4e6ec97851d4332d7ed9710c4be74b",
    OLD + "/ADDENDUM-3-FREEZE.json": "f75f9d9ad693ecfc7522249ad9c10e5740e178b01525a369de2ae491c47d86e2",
    OLD + "/review-overlay/heldout-addendum-3/HELDOUT-ADDENDUM-3.json": "5e03d91b802a8a15098d8f7812c867f0d9aa4066936bcb73c2314a80b1345124",
    OLD + "/review-overlay/heldout-addendum-3/HW-TRACEABILITY.json": "251a898c140e5de87beda6c0fd1e4ed67c5828c139df1bb38144afb17a99bba3",
    OLD + "/PARENT-PINS.json": "11a348a5cc24a0cb10d3fac8b7425dc0ffee932fcbc7827e2f9381af65be45d6",
    OLD + "/DELTA.json": "e2d8dd470f6bd4990f08dc6d318431f442ccb0ad93d2c95b1ca9ce8055ae3987",
    OLD + "/review-overlay/proposals/heldout-harness-r3/tests/heldout.rs": "0dd8b1f566b9d69069bcd2928d681e8f2fc4ee343cbc5b6e65be8b1577dc6ff1",
    OWN + "/records/PARTIAL-PIN-VERIFICATION.json": None,
    OWN + "/records/PARENT-PIN-REVERIFY.json": None,
}


def h(p):
    b = open(p, "rb").read()
    return len(b), hashlib.sha256(b).hexdigest()


def main():
    dst = os.path.abspath(sys.argv[1])
    assert dst.startswith(OWN + "/records/")
    rec = {"format": "evidence-repair-resume-pre-edit-freeze/1",
           "author": "claude-opus-5-5 (resume source author; pins only, no expectation written)",
           "frozen_at_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
           "rule": "Every unchanged independent packet, r2 overlay, report and inherited partial record hashed now, before any harness assertion of this resume is written. A STOP means no edit may follow.",
           "manifests": {}, "named": {}, "stops": []}
    for k, (mp, want) in MANIFESTS.items():
        n, d = h(mp)
        doc = json.load(open(mp))
        root = os.path.dirname(mp)
        rows, bad = [], []
        for e in doc["files"]:
            fn, fd = h(os.path.join(root, e["path"]))
            rows.append({"path": e["path"], "sha256": fd, "bytes": fn})
            if fd != e["sha256"]:
                bad.append(e["path"])
        rec["manifests"][k] = {"path": mp, "sha256": d, "equal": d == want, "entries": len(rows), "mismatch": bad, "files": rows}
        if d != want or bad:
            rec["stops"].append(k)
    for p, want in NAMED.items():
        n, d = h(p)
        rec["named"][p] = {"sha256": d, "bytes": n, "equal": None if want is None else d == want}
        if want is not None and d != want:
            rec["stops"].append(p)
    rec["verdict"] = "PASS" if not rec["stops"] else "STOP"
    body = json.dumps(rec, indent=1, sort_keys=True).encode() + b"\n"
    fd = os.open(dst, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
    with os.fdopen(fd, "wb") as f:
        f.write(body)
    print(rec["verdict"], rec["stops"], {k: v["entries"] for k, v in rec["manifests"].items()}, len(rec["named"]), hashlib.sha256(body).hexdigest())


if __name__ == "__main__":
    main()
