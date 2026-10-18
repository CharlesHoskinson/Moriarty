#!/usr/bin/env python3
"""Reviewer-owned stdlib listing/hash helper. Reads only; never imports candidate code.
Usage: python3 -I ls.py ROOT [--hash] [--exclude-dir NAME ...]"""
import resource, sys, os, hashlib
resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))
root = sys.argv[1]
do_hash = '--hash' in sys.argv
excl = set()
a = sys.argv[2:]
for i, x in enumerate(a):
    if x == '--exclude-dir':
        excl.add(a[i + 1])
out = []
for dp, dns, fns in os.walk(root, followlinks=False):
    dns[:] = sorted(d for d in dns if d not in excl)
    for fn in sorted(fns):
        p = os.path.join(dp, fn)
        rel = os.path.relpath(p, root)
        if os.path.islink(p):
            out.append((rel, -1, 'SYMLINK->' + os.readlink(p)))
            continue
        st = os.stat(p)
        h = ''
        if do_hash:
            with open(p, 'rb') as f:
                h = hashlib.sha256(f.read()).hexdigest()
        out.append((rel, st.st_size, h))
for rel, sz, h in sorted(out):
    print(f"{sz}\t{h}\t{rel}" if do_hash else f"{sz}\t{rel}")
print(f"# files={len(out)}", file=sys.stderr)
