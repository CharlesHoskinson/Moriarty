#!/usr/bin/env python3
"""Linear-time certificate checker seed for Phase 3.3.

Walks gen-ir-v2ten, tags declaration names by P-keyword rules, writes a
certificate summary JSON + validates invariants:
  - every file has at least one tagged node OR is allowed-empty
  - tag set subset of {Led, Prop, Cmp, Post, OTHER}
Exit 0 on success.
"""
from __future__ import annotations

import json
import os
import sys
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
IR = Path(os.environ["GEN_IR"]) if os.environ.get("GEN_IR") \
     else ROOT / "research/positive-program/sigma/gen-ir-v2ten"
OUT = ROOT / "research/positive-program/sigma/GATE-3.3-CERT-RESULT.json"

RULES = {
    "Led": [
        "credit", "debit", "transfer", "mint", "burn", "balance", "supply",
        "move", "deposit", "withdraw", "reallocate", "shares", "collateral",
    ],
    "Prop": ["muldiv", "sharesfrom", "assetsfrom", "ceildiv", "geometric", "prorata"],
    "Cmp": [
        "healthy", "canliquidate", "require", "guard", "safe", "ishealthy",
        "invariant", "canborrow", "canwithdraw",
    ],
    "Post": [
        "price", "oracle", "shock", "post", "index", "phase", "status",
        "confirm", "approve", "attest",
    ],
}


def tag_name(name: str) -> str:
    n = name.lower()
    best, score = "OTHER", 0
    for fam, kws in RULES.items():
        s = sum(1 for k in kws if k.lower() in n)
        if s > score:
            best, score = fam, s
    return best if score else "OTHER"


def collect_names(d) -> list[str]:
    names: list[str] = []

    def walk(o):
        if isinstance(o, dict):
            if isinstance(o.get("name"), str):
                names.append(o["name"])
            for v in o.values():
                walk(v)
        elif isinstance(o, list):
            for i in o:
                walk(i)

    walk(d)
    return names


def main() -> int:
    if not IR.is_dir():
        # A missing corpus is a blocked check, not a failed certificate.
        print("BLOCKED - missing IR", IR, file=sys.stderr)
        return 3
    files = {}
    total = Counter()
    for fp in sorted(IR.glob("*.json")):
        names = collect_names(json.loads(fp.read_text()))
        tags = Counter(tag_name(n) for n in names)
        files[fp.name] = dict(tags)
        total.update(tags)
    # Invariants
    allowed = {"Led", "Prop", "Cmp", "Post", "OTHER"}
    for fn, tags in files.items():
        for k in tags:
            if k not in allowed:
                print("FAIL: bad tag", k, "in", fn, file=sys.stderr)
                return 1
        if sum(tags.values()) == 0:
            print("FAIL: empty file", fn, file=sys.stderr)
            return 1
    # Zero certificates examined is not a passing certificate check.
    # Guard sits BEFORE the write so an empty IR leaves no PASS artefact.
    if not files:
        print("BLOCKED - no *.json under", IR, "; nothing was certified",
              file=sys.stderr)
        return 3
    result = {
        "status": "PASS",
        "ir": str(IR),
        "files": len(files),
        "totals": dict(total),
        "per_file": files,
        "note": "keyword certificate seed; not signature-level",
    }
    OUT.write_text(json.dumps(result, indent=2) + "\n")
    print("PASS", len(files), "files; totals", dict(total))
    print("wrote", OUT)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
