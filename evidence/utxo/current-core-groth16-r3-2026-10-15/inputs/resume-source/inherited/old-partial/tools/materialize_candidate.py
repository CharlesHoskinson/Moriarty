#!/usr/bin/env python3
"""Copy the pinned parent candidate (every SOURCE-MANIFEST a6e17efd... entry) into
./candidate/ byte-for-byte, verifying each file before and after the copy (stdlib only).

The destination must not exist. Nothing in the parent is written. Harness-owned and
private files are not in the parent SOURCE-MANIFEST and are therefore never opened.

Usage: python3 -I tools/materialize_candidate.py
"""
import hashlib
import json
import os
import resource
import shutil
import sys

resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))

BASE = "/home/charl/moriarty-eutxo-validation-2026-10-12"
HERE = os.path.join(BASE, "current-core-groth16-evidence-repair-source-20261015")
PARENT = os.path.join(BASE, "current-core-groth16-compiler-review-repair-source-20261015")
PARENT_SOURCE_MANIFEST = "a6e17efd0a47194c70ac6e4467efce2d47f9254dc47a8e596f6add2b92aef97e"


def sha(b):
    return hashlib.sha256(b).hexdigest()


def read(p):
    with open(p, "rb") as f:
        return f.read()


def main():
    mb = read(os.path.join(PARENT, "SOURCE-MANIFEST.json"))
    if sha(mb) != PARENT_SOURCE_MANIFEST:
        sys.exit("STOP: parent SOURCE-MANIFEST digest differs")
    manifest = json.loads(mb)
    dest = os.path.join(HERE, "candidate")
    os.mkdir(dest)
    for f in manifest["files"]:
        rel = f["path"]
        if os.path.isabs(rel) or ".." in rel.split("/") or "private" in rel:
            sys.exit(f"STOP: bad path {rel}")
        src = os.path.join(PARENT, rel)
        if sha(read(src)) != f["sha256"]:
            sys.exit(f"STOP: {rel} differs from its pin")
        out = os.path.join(dest, rel)
        os.makedirs(os.path.dirname(out), exist_ok=True)
        shutil.copyfile(src, out)
        if sha(read(out)) != f["sha256"]:
            sys.exit(f"STOP: post-copy mismatch {rel}")
    print(json.dumps({"copied": len(manifest["files"]), "dest": dest}))


if __name__ == "__main__":
    main()
