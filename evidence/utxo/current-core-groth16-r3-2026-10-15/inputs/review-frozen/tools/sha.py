#!/usr/bin/env python3
"""Reviewer-owned: print sha256 and size of given files. Usage: python3 -I sha.py FILE..."""
import resource, sys, hashlib
resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))
for p in sys.argv[1:]:
    try:
        b = open(p, 'rb').read()
        print(f"{hashlib.sha256(b).hexdigest()}  {len(b)}  {p}")
    except OSError as e:
        print(f"ERROR {type(e).__name__}  {p}")
