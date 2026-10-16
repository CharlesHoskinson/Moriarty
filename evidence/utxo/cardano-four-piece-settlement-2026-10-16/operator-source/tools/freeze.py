#!/usr/bin/env python3
"""Digest record of task subtrees (O''). NOT RUN BY ITS AUTHOR.

Usage: python3 -I freeze.py OUT_JSON LABEL ROOT SUBDIR [SUBDIR ...]

Walks each ROOT/SUBDIR without following links and writes OUT_JSON with mode "x" (never
overwrites): every regular file as {path relative to ROOT, bytes, sha256}, sorted, and every
non-regular entry (symlink, fifo, socket, device) under "nonRegular". It changes no mode and
deletes nothing; read-only modes are applied by the calling step. When OUT_JSON lies inside a
walked subtree (OPX7f records receipts/ into receipts/task-final.json) it is excluded from its own
record and named in "excludedSelf". Exit 0 when written; 1 when a subdir is missing (nothing
written); 2 on usage.
"""
import datetime
import hashlib
import json
import os
import stat
import sys


def sha256_of(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def walk(root, sub, skip):
    files, other = [], []
    base = os.path.join(root, sub)
    for d, dirs, names in os.walk(base, followlinks=False):
        for n in sorted(dirs):
            p = os.path.join(d, n)
            if not stat.S_ISDIR(os.lstat(p).st_mode):
                other.append(os.path.relpath(p, root))
        for n in names:
            p = os.path.join(d, n)
            if os.path.abspath(p) == skip:
                continue
            st = os.lstat(p)
            rel = os.path.relpath(p, root)
            if stat.S_ISREG(st.st_mode):
                files.append({"path": rel, "bytes": st.st_size, "sha256": sha256_of(p)})
            else:
                other.append(rel)
    return files, other


def main():
    if len(sys.argv) < 5:
        sys.stderr.write("usage: freeze.py OUT_JSON LABEL ROOT SUBDIR [SUBDIR ...]\n")
        sys.exit(2)
    out, label, root, subs = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4:]
    out_abs = os.path.abspath(out)
    excluded = None
    for s in subs:
        base = os.path.abspath(os.path.join(root, s))
        if not os.path.isdir(base) or os.path.islink(base):
            sys.stderr.write("missing or non-directory subtree: %s\n" % s)
            sys.exit(1)
        if out_abs.startswith(base + os.sep):
            excluded = os.path.relpath(out_abs, os.path.abspath(root))
    files, other = [], []
    for s in subs:
        f, o = walk(root, s, out_abs)
        files.extend(f)
        other.extend(o)
    files.sort(key=lambda r: r["path"])
    doc = {
        "schema": "four-way-vs-operator/freeze-receipt/1",
        "label": label,
        "root": root,
        "subtrees": subs,
        "utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "fileCount": len(files),
        "files": files,
        "nonRegular": sorted(other),
        "excludedSelf": excluded,
    }
    with open(out, "x", encoding="utf-8") as f:
        json.dump(doc, f, indent=1)
        f.write("\n")
    print("%s: %d files, %d non-regular" % (label, len(files), len(other)))
    sys.exit(0)


if __name__ == "__main__":
    main()
