#!/usr/bin/env python3
"""Reviewer-owned read-only regex search over a directory of source files.
Usage: python3 -I grepsrc.py ROOT REGEX [CONTEXT] [GLOB_SUFFIX]"""
import resource, sys, os, re
resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))
root, pat = sys.argv[1], re.compile(sys.argv[2])
ctx = int(sys.argv[3]) if len(sys.argv) > 3 else 0
suffix = sys.argv[4] if len(sys.argv) > 4 else ".rs"
paths = [root] if os.path.isfile(root) else [os.path.join(dp, f) for dp, _, fs in os.walk(root) for f in fs if f.endswith(suffix)]
for p in sorted(paths):
    try:
        lines = open(p, encoding="utf-8", errors="replace").read().split("\n")
    except OSError:
        continue
    for i, line in enumerate(lines):
        if pat.search(line):
            lo, hi = max(0, i - ctx), min(len(lines), i + ctx + 1)
            for j in range(lo, hi):
                print(f"{os.path.relpath(p, root) if p != root else os.path.basename(p)}:{j+1}:{lines[j]}")
            if ctx:
                print("--")
