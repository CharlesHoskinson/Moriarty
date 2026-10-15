"""Pin the Moriarty Core source closure linked by mori-r1cs (read-only hashing).

Reads the checkout; never runs git, cargo or Moriarty code. Writes the JSON given as
the third argument (exclusive create).

Usage: python3 -I pin_core_closure.py <moriarty-checkout> <head-commit> <output.json>
"""
import hashlib
import json
import re
import sys
from pathlib import Path

CRATES = ["mori-core", "mori-lexer", "mori-diagnostics", "mori-span"]
ROOT_FILES = ["Cargo.toml", "rust-toolchain.toml"]
EXCLUDED_DIRS = {"tests", "examples", "benches", "snapshots", "target"}


def pin(base, rel):
    data = (base / rel).read_bytes()
    return {"path": rel, "sha256": hashlib.sha256(data).hexdigest(), "bytes": len(data)}


def main():
    base = Path(sys.argv[1]).resolve()
    head = sys.argv[2]
    output = Path(sys.argv[3])
    files = [pin(base, f) for f in ROOT_FILES]
    excluded = []
    includes = []
    for crate in CRATES:
        root = base / "crates" / crate
        for p in sorted(root.rglob("*")):
            if not p.is_file():
                continue
            rel = p.relative_to(base).as_posix()
            parts = p.relative_to(root).parts
            if parts[0] in EXCLUDED_DIRS or parts[0] in {"README.md"}:
                excluded.append(rel)
                continue
            files.append(pin(base, rel))
            if p.suffix == ".rs":
                for m in re.finditer(r"include_(?:str|bytes)!\s*\(\s*\"([^\"]+)\"", p.read_text()):
                    includes.append({"in": rel, "target": m.group(1)})
    files.sort(key=lambda f: f["path"])
    canonical = json.dumps(files, sort_keys=True, separators=(",", ":")).encode()
    record = {
        "format": "mori-r1cs-core-closure-pins/1",
        "checkout": str(base),
        "head": head,
        "crates": CRATES,
        "rule": "workspace root Cargo.toml and rust-toolchain.toml, plus every file of each crate except tests/, examples/, benches/, snapshots/ and README.md (a path dependency builds only the library target).",
        "build_scripts": [f["path"] for f in files if f["path"].endswith("/build.rs")],
        "include_macros": includes,
        "closure_sha256": hashlib.sha256(canonical).hexdigest(),
        "closure_sha256_rule": "SHA-256 of the compact sorted-key JSON array of {path, sha256, bytes} objects sorted by path",
        "files": files,
        "excluded": excluded,
        "registry_dependencies_not_pinned_here": "serde, serde_json, thiserror, sha2, k256, miette, soa-rs and their transitive crates are resolved by a future lock (EXECUTION-REQUEST.json); no lock is invented.",
    }
    with output.open("x") as f:
        json.dump(record, f, indent=2)
        f.write("\n")
    print(json.dumps({"files": len(files), "excluded": len(excluded), "closure_sha256": record["closure_sha256"], "build_scripts": record["build_scripts"], "include_macros": includes}, indent=1))


if __name__ == "__main__":
    main()
