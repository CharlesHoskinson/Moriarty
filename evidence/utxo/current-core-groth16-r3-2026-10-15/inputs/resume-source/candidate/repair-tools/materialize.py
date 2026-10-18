#!/usr/bin/env python3
"""Materialize the origin r2 candidate's manifested public bytes into this successor.

Stdlib only; reads the origin, writes only under DEST. Every origin file listed in the
origin SOURCE-MANIFEST.json is copied to the same relative path, with its SHA-256
checked against the manifest before and after the copy. Files named in --history (the
ones this repair edits or supersedes) and the origin SOURCE-MANIFEST.json are also kept
verbatim under history/origin-r2/<path>. Refuses to overwrite any existing file.

Usage: python3 -I materialize.py ORIGIN DEST OUT_COPY_PINS --history PATH...
"""
import datetime
import hashlib
import json
import os
import resource
import sys

resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))

origin, dest, out = sys.argv[1:4]
assert sys.argv[4] == "--history"
history = sys.argv[5:]


def sha(data):
    return hashlib.sha256(data).hexdigest()


def write_new(path, data):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "xb") as f:
        f.write(data)


manifest_bytes = open(os.path.join(origin, "SOURCE-MANIFEST.json"), "rb").read()
manifest = json.loads(manifest_bytes)
listed = {f["path"]: f for f in manifest["files"]}
for h in history:
    if h not in listed and h != "SOURCE-MANIFEST.json":
        sys.exit(f"history path not in origin manifest: {h}")

rows = []
for f in manifest["files"]:
    data = open(os.path.join(origin, f["path"]), "rb").read()
    if sha(data) != f["sha256"] or len(data) != f["bytes"]:
        sys.exit(f"origin mismatch before copy: {f['path']}")
    target = os.path.join(dest, f["path"])
    write_new(target, data)
    copied = open(target, "rb").read()
    row = {"path": f["path"], "origin_manifest_sha256": f["sha256"], "observed_origin_sha256": sha(data),
           "dest_sha256_after_copy": sha(copied), "bytes": len(copied), "match": sha(copied) == f["sha256"]}
    if f["path"] in history:
        hpath = os.path.join(dest, "history", "origin-r2", f["path"])
        write_new(hpath, data)
        row["history_copy"] = os.path.relpath(hpath, dest)
        row["history_copy_sha256"] = sha(open(hpath, "rb").read())
    rows.append(row)

hman = os.path.join(dest, "history", "origin-r2", "SOURCE-MANIFEST.json")
write_new(hman, manifest_bytes)

record = {
    "format": "repair-copy-pins/1",
    "copied_at_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
    "tool": "repair-tools/materialize.py (repair author, stdlib only; imports no candidate script)",
    "origin": origin,
    "origin_source_manifest_sha256": sha(manifest_bytes),
    "origin_source_manifest_history_copy": os.path.relpath(hman, dest),
    "rule": "Each origin SOURCE-MANIFEST file was byte-copied to the same relative path. Files later edited or superseded by this repair also have a verbatim copy under history/origin-r2/. Root files may differ from these pins after the repair; DELTA.json lists every such change.",
    "files": rows,
    "all_match": all(r["match"] for r in rows),
    "count": len(rows),
    "history_paths": sorted(history) + ["SOURCE-MANIFEST.json"],
}
with open(out, "x") as fh:
    json.dump(record, fh, indent=2)
    fh.write("\n")
print(json.dumps({"count": len(rows), "all_match": record["all_match"], "history": len(history) + 1}, indent=1))
