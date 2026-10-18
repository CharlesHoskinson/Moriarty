"""After the freeze: copy the ORIGINAL v1.1 checker source (14 files) into scratch/oldv11-src/mcpcat/ so that no
import ever touches the old directory. Each byte is checked against the reviewed hardening SOURCE-MANIFEST.json.
Refuses to overwrite; refuses to run before pack/FREEZE.json exists.

Usage: python3 -I -B tools/copy_checker.py
"""
import datetime
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OLD = Path("/home/charl/moriarty-eutxo-validation-2026-10-12/cardano-catalogue-v11-hardening-20261015")


def main():
    if not (ROOT / "pack" / "FREEZE.json").exists():
        raise SystemExit("freeze first")
    man = json.loads((OLD / "SOURCE-MANIFEST.json").read_bytes())
    want = {r["path"]: r for r in man["files"] if r["path"].startswith("candidate/src/mcpcat/")}
    dst = ROOT / "scratch" / "oldv11-src" / "mcpcat"
    if dst.exists():
        raise SystemExit(f"{dst} exists")
    dst.mkdir(parents=True)
    rec = {"copiedAt": datetime.datetime.now(datetime.timezone.utc).isoformat(),
           "sourceManifestSha256": hashlib.sha256((OLD / "SOURCE-MANIFEST.json").read_bytes()).hexdigest(), "files": {}}
    srcs = sorted((OLD / "candidate/src/mcpcat").glob("*.py"))
    if sorted(f"candidate/src/mcpcat/{p.name}" for p in srcs) != sorted(want):
        raise SystemExit("file set differs from SOURCE-MANIFEST")
    for p in srcs:
        b = p.read_bytes()
        h = hashlib.sha256(b).hexdigest()
        w = want[f"candidate/src/mcpcat/{p.name}"]
        if h != w["sha256"] or len(b) != w["bytes"]:
            raise SystemExit(f"{p} differs from SOURCE-MANIFEST")
        (dst / p.name).write_bytes(b)
        if hashlib.sha256((dst / p.name).read_bytes()).hexdigest() != h:
            raise SystemExit("copy mismatch")
        rec["files"][p.name] = {"sha256": h, "bytes": len(b)}
    (ROOT / "scratch" / "COPY-RECORD.json").write_text(json.dumps(rec, indent=1) + "\n")
    print(json.dumps({"files": len(rec["files"]), "allMatchSourceManifest": True}))


if __name__ == "__main__":
    main()
