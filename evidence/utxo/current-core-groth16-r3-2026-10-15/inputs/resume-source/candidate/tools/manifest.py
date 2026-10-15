"""Hash manifests for the mori-r1cs r2 task directory (read and hash only).

Modes (each output is created exclusively and never overwritten):
  build  <root> <out>         Cargo.toml + src/** of source/mori-r1cs (compiler build identity)
  source <root> <out>         every task file except harness-owned and private files
  pins   <root> <head> <out>  re-verify PRIOR-SOURCE-PINS.json against the live checkout

Usage: python3 -I manifest.py <mode> ...
"""
import hashlib
import json
import sys
from pathlib import Path

HARNESS = {"PROMPT.md", "run-author.py", "PROCESS.json", "process.stderr", "provider-result.private.json", "FINAL-RESPONSE.md"}


def pin(root, path):
    data = path.read_bytes()
    return {"path": path.relative_to(root).as_posix(), "sha256": hashlib.sha256(data).hexdigest(), "bytes": len(data)}


def write(out, record):
    with Path(out).open("x") as f:
        json.dump(record, f, indent=2)
        f.write("\n")


def prior_copy(root):
    copy = json.loads((root / "ROOT-PARTIAL-SOURCE-COPY.json").read_text())
    return {Path(f["destination"]).resolve().relative_to(root).as_posix(): f["sha256"] for f in copy["copiedFiles"]}


def build(root, out):
    crate = root / "source" / "mori-r1cs"
    files = [pin(root, crate / "Cargo.toml")] + [pin(root, p) for p in sorted((crate / "src").rglob("*")) if p.is_file()]
    prior = prior_copy(root)
    for f in files:
        f["vs_prior_partial"] = "new" if f["path"] not in prior else ("unchanged" if prior[f["path"]] == f["sha256"] else "changed")
    write(out, {
        "format": "mori-r1cs-build-manifest/1",
        "rule": "Cargo.toml and every file under src/ of source/mori-r1cs. SHA-256 of this file is CompilerBuild.compiler_source_manifest_sha256. Tests, fixtures and documents are excluded so that they do not change the relation identity.",
        "files": files,
    })
    print(json.dumps({"files": len(files), "sha256": hashlib.sha256(Path(out).read_bytes()).hexdigest(), "status": {f["path"]: f["vs_prior_partial"] for f in files}}, indent=1))


def source(root, out):
    out_rel = Path(out).resolve().relative_to(root).as_posix()
    files = []
    for p in sorted(root.rglob("*")):
        if not p.is_file():
            continue
        rel = p.relative_to(root).as_posix()
        if rel in HARNESS or rel == out_rel or "__pycache__" in rel:
            continue
        files.append(pin(root, p))
    prior = prior_copy(root)
    for f in files:
        if f["path"] in prior:
            f["vs_prior_partial"] = "unchanged" if prior[f["path"]] == f["sha256"] else "changed"
    write(out, {
        "format": "mori-r1cs-source-manifest/1",
        "rule": "Every file of the task directory except the harness-owned PROMPT.md, run-author.py, PROCESS.json, process.stderr, FINAL-RESPONSE.md and the private provider-result.private.json (never opened).",
        "build_manifest_sha256": hashlib.sha256((root / "BUILD-MANIFEST.json").read_bytes()).hexdigest(),
        "files": files,
    })
    print(json.dumps({"files": len(files), "sha256": hashlib.sha256(Path(out).read_bytes()).hexdigest()}, indent=1))


def pins(root, head, out):
    prior = json.loads((root / "PRIOR-SOURCE-PINS.json").read_text())
    tree = Path(prior["worktree"])
    rows = []
    for f in prior["files"]:
        p = tree / f["path"]
        now = hashlib.sha256(p.read_bytes()).hexdigest() if p.exists() else None
        rows.append({**f, "observed_sha256": now, "matches": now == f["sha256"]})
    write(out, {
        "format": "mori-r1cs-source-pins/2",
        "worktree": str(tree),
        "head": head,
        "head_source": "read from the worktree's git metadata files (HEAD ref file), no git command",
        "parent": prior["parent"],
        "prior_pins_sha256": hashlib.sha256((root / "PRIOR-SOURCE-PINS.json").read_bytes()).hexdigest(),
        "all_match": all(r["matches"] for r in rows),
        "note": "crates/mori-core is unmodified at HEAD; uncommitted changes in that checkout are under crates/mori-cardano (staged codec), pinned by digest only. The Core dependency closure is pinned separately in CORE-CLOSURE-PINS.json.",
        "files": rows,
    })
    print(json.dumps({"files": len(rows), "all_match": all(r["matches"] for r in rows)}, indent=1))


def main():
    mode = sys.argv[1]
    root = Path(sys.argv[2]).resolve()
    if mode == "build":
        build(root, sys.argv[3])
    elif mode == "source":
        source(root, sys.argv[3])
    elif mode == "pins":
        pins(root, sys.argv[3], sys.argv[4])
    else:
        raise SystemExit(f"unknown mode {mode}")


if __name__ == "__main__":
    main()
