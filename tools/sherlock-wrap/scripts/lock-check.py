#!/usr/bin/env python3
"""Every (name, version) in the given lock must be in one of the reference locks
(openvm, backend, Moriarty root, tools/gpu-prover). Prints the extras; exit 1 if any."""
import os, re, sys
H = os.path.expanduser("~")
REFS = [f"{H}/.cache/m7-wrap/src/openvm-tested/Cargo.lock",
        f"{H}/Projects/Moriarty-wt/tested/research/sherlock-e1-fr-backend-20261112/Cargo.lock.e1",
        f"{H}/Projects/Moriarty-wt/tested/Cargo.lock",
        f"{H}/Projects/Moriarty-wt/tested/tools/gpu-prover/Cargo.lock"]
def pk(p):
    s = open(p).read()
    return set(re.findall(r'name = "([^"]+)"\nversion = "([^"]+)"', s))
ref = set().union(*(pk(p) for p in REFS if os.path.exists(p)))
local = {"sherlock-wrap", "sherlock-wrap-ledger-check"}
extra = sorted(x for x in pk(sys.argv[1]) if x not in ref and x[0] not in local)
print("extra:", extra)
sys.exit(1 if extra else 0)
