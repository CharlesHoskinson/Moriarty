#!/usr/bin/env python3
"""Freeze repair-author expectation files by hash (stdlib only). Creates OUT exclusively.

Usage: python3 -I freeze.py ROOT OUT STATEMENT PATH...
"""
import datetime
import hashlib
import json
import os
import resource
import sys

resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))

root, out, statement = sys.argv[1:4]
files = []
for rel in sys.argv[4:]:
    data = open(os.path.join(root, rel), "rb").read()
    files.append({"path": rel, "sha256": hashlib.sha256(data).hexdigest(), "bytes": len(data)})
record = {
    "format": "repair-author-freeze/1",
    "frozen_at_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
    "author": "claude-opus-5-5, review-repair source author",
    "statement": statement,
    "files": files,
    "aggregate_sha256": hashlib.sha256(json.dumps(files, sort_keys=True, separators=(",", ":")).encode()).hexdigest(),
    "aggregate_rule": "sha256 of compact sorted-key JSON of the files list",
}
with open(out, "x") as f:
    json.dump(record, f, indent=2)
    f.write("\n")
print(json.dumps({"frozen_at_utc": record["frozen_at_utc"], "aggregate_sha256": record["aggregate_sha256"]}, indent=1))
