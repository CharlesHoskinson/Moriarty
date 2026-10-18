#!/usr/bin/env python3
"""PROPOSED E1v+/E1hv lock gate (fresh-reviewer r2 plumbing; stdlib only; read-only; UNREVIEWED).

subset: every package of a generated Cargo.lock is either an explicitly allowed local
        path package (no source) or a crates.io registry package whose exact
        (name, version, checksum) row is in the pinned lock union; the union file's own
        SHA-256 must equal the caller's pin. Any git or other source, any midnight-*
        crate, any missing checksum is a STOP. Records the lock sha256 and its
        registry-subset digest (sha256 of compact sorted-key JSON of
        [{name, version, checksum}] sorted by (name, version)).
pair:   for every registry crate name present in both locks, the sets of
        (version, checksum) are identical (the candidate and the harness build the
        same registry code).

Usage:
  python3 -I lock_gate.py subset LOCK UNION_JSON UNION_SHA256 LOCAL [LOCAL ...]
  python3 -I lock_gate.py pair LOCK_A LOCK_B
Exit 0 = PASS/SAME, 2 = usage, 3 = STOP/DIFFER.
"""
import hashlib
import json
import resource
import sys
import tomllib

resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))

CRATES_IO = "registry+https://github.com/rust-lang/crates.io-index"


def load(path):
    with open(path, "rb") as f:
        raw = f.read()
    return raw, tomllib.loads(raw.decode()).get("package", [])


def subset_digest(rows):
    rows = sorted(rows, key=lambda r: (r["name"], r["version"]))
    return hashlib.sha256(json.dumps(rows, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


def main(argv):
    if len(argv) >= 5 and argv[0] == "subset":
        lock_path, union_path, union_pin, allowed = argv[1], argv[2], argv[3], set(argv[4:])
        union_raw = open(union_path, "rb").read()
        problems = []
        if hashlib.sha256(union_raw).hexdigest() != union_pin:
            problems.append("lock union file differs from its pinned sha256")
        union = {(p["name"], p["version"]): p.get("checksum") for p in json.loads(union_raw)["packages"] if p.get("source") == CRATES_IO}
        raw, pkgs = load(lock_path)
        reg = []
        for p in pkgs:
            src, name, ver = p.get("source"), p.get("name"), p.get("version")
            if src is None:
                if name not in allowed:
                    problems.append(f"local package not allowed: {name}")
                continue
            if src != CRATES_IO:
                problems.append(f"forbidden source {src} for {name}@{ver}")
                continue
            if "midnight" in name:
                problems.append(f"forbidden crate {name}@{ver}")
            ck = p.get("checksum")
            if not ck:
                problems.append(f"missing checksum {name}@{ver}")
            elif (name, ver) not in union:
                problems.append(f"outside the union {name}@{ver}")
            elif union[(name, ver)] != ck:
                problems.append(f"checksum differs {name}@{ver}")
            reg.append({"name": name, "version": ver, "checksum": ck})
        out = {"mode": "subset", "lock": lock_path, "lock_sha256": hashlib.sha256(raw).hexdigest(), "packages": len(pkgs),
               "registry_packages": len(reg), "registry_subset_digest": subset_digest(reg), "allowed_local": sorted(allowed),
               "problems": problems, "verdict": "PASS" if not problems else "STOP"}
        print(json.dumps(out, indent=1, sort_keys=True))
        return 0 if not problems else 3
    if len(argv) == 3 and argv[0] == "pair":
        sides = []
        for path in argv[1:3]:
            raw, pkgs = load(path)
            reg = {}
            for p in pkgs:
                if p.get("source") == CRATES_IO:
                    reg.setdefault(p["name"], set()).add((p["version"], p.get("checksum")))
            sides.append((path, hashlib.sha256(raw).hexdigest(), reg))
        (pa, ha, a), (pb, hb, b) = sides
        shared = sorted(set(a) & set(b))
        differ = [{"name": n, "a": sorted(a[n]), "b": sorted(b[n])} for n in shared if a[n] != b[n]]
        out = {"mode": "pair", "lock_a": pa, "lock_a_sha256": ha, "lock_b": pb, "lock_b_sha256": hb, "shared_names": len(shared),
               "only_a": sorted(set(a) - set(b)), "only_b": sorted(set(b) - set(a)), "differ": differ,
               "verdict": "SAME" if not differ else "DIFFER"}
        print(json.dumps(out, indent=1, sort_keys=True))
        return 0 if not differ else 3
    print(json.dumps({"verdict": "USAGE", "usage": __doc__.split("Usage:")[1].strip()}))
    return 2


sys.exit(main(sys.argv[1:]))
