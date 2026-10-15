#!/usr/bin/env python3
"""Mechanical manifests for the review-repair successor (stdlib only; never imports the
origin's tools/manifest.py or any candidate script).

Modes:
  build  ROOT ORIGIN   rewrite ROOT/BUILD-MANIFEST.json: Cargo.toml + every file under
                       src/ of source/mori-r1cs, same format and rule as the origin, with
                       vs_origin_r2 = unchanged | changed | new per file. The origin bytes
                       are kept at history/origin-r2/BUILD-MANIFEST.json.
  delta  ROOT ORIGIN   write ROOT/DELTA.json and ROOT/DIFFS/*.diff (created exclusively):
                       every successor file compared with the origin SOURCE-MANIFEST.
  source ROOT          rewrite ROOT/SOURCE-MANIFEST.json over every file except the
                       harness-owned files, __pycache__, and the manifest and its sidecar;
                       write ROOT/SOURCE-MANIFEST.json.sha256.

Usage: python3 -I manifests.py MODE ROOT [ORIGIN]
"""
import difflib
import hashlib
import json
import os
import resource
import sys
from pathlib import Path

resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))

HARNESS = {"PROMPT.md", "run-author.py", "PROCESS.json", "process.stderr", "provider-result.private.json", "FINAL-RESPONSE.md"}
SIDECAR = "SOURCE-MANIFEST.json.sha256"


def sha(b):
    return hashlib.sha256(b).hexdigest()


def pin(root, p):
    data = p.read_bytes()
    return {"path": p.relative_to(root).as_posix(), "sha256": sha(data), "bytes": len(data)}


def replace(path, record):
    tmp = path.with_name(path.name + ".tmp")
    with tmp.open("x") as f:
        json.dump(record, f, indent=2)
        f.write("\n")
    os.replace(tmp, path)


def origin_pins(origin):
    return {f["path"]: f["sha256"] for f in json.loads((origin / "SOURCE-MANIFEST.json").read_text())["files"]}


def build(root, origin):
    crate = root / "source" / "mori-r1cs"
    files = [pin(root, crate / "Cargo.toml")] + [pin(root, p) for p in sorted((crate / "src").rglob("*")) if p.is_file()]
    prior = origin_pins(origin)
    for f in files:
        f["vs_origin_r2"] = "new" if f["path"] not in prior else ("unchanged" if prior[f["path"]] == f["sha256"] else "changed")
    replace(root / "BUILD-MANIFEST.json", {
        "format": "mori-r1cs-build-manifest/1",
        "rule": "Cargo.toml and every file under src/ of source/mori-r1cs. SHA-256 of this file is CompilerBuild.compiler_source_manifest_sha256. Tests, fixtures and documents are excluded so that they do not change the relation identity.",
        "origin_build_manifest_sha256": sha((origin / "BUILD-MANIFEST.json").read_bytes()),
        "files": files,
    })
    print(json.dumps({"files": len(files), "sha256": sha((root / "BUILD-MANIFEST.json").read_bytes()),
                      "changed": [f["path"] for f in files if f["vs_origin_r2"] != "unchanged"]}, indent=1))


def walk(root):
    for p in sorted(root.rglob("*")):
        if not p.is_file():
            continue
        rel = p.relative_to(root).as_posix()
        if rel in HARNESS or "__pycache__" in rel or rel in ("SOURCE-MANIFEST.json", SIDECAR) or rel.endswith(".tmp"):
            continue
        yield p, rel


def delta(root, origin):
    prior = origin_pins(origin)
    diffs = root / "DIFFS"
    diffs.mkdir()
    rows, seen = [], set()
    for p, rel in walk(root):
        if rel.startswith("DIFFS/") or rel == "DELTA.json":
            continue
        seen.add(rel)
        now = sha(p.read_bytes())
        if rel not in prior:
            rows.append({"path": rel, "status": "added", "sha256": now})
            continue
        if prior[rel] == now:
            rows.append({"path": rel, "status": "unchanged", "sha256": now})
            continue
        a = (origin / rel).read_text(encoding="utf-8").splitlines(keepends=True)
        b = p.read_text(encoding="utf-8").splitlines(keepends=True)
        text = "".join(difflib.unified_diff(a, b, fromfile=f"origin-r2/{rel}", tofile=f"successor/{rel}", n=3))
        name = rel.replace("/", "__") + ".diff"
        (diffs / name).write_text(text, encoding="utf-8")
        rows.append({"path": rel, "status": "changed", "origin_sha256": prior[rel], "sha256": now, "diff": f"DIFFS/{name}",
                     "diff_sha256": sha((diffs / name).read_bytes()),
                     "lines_removed": sum(1 for l in text.splitlines() if l.startswith("-") and not l.startswith("---")),
                     "lines_added": sum(1 for l in text.splitlines() if l.startswith("+") and not l.startswith("+++"))})
    missing = sorted(set(prior) - seen)
    record = {"format": "repair-delta/1", "origin": str(origin),
              "origin_source_manifest_sha256": sha((origin / "SOURCE-MANIFEST.json").read_bytes()),
              "rule": "Every successor file (except harness-owned, DELTA.json, DIFFS/ and the successor manifest) compared by SHA-256 with the origin SOURCE-MANIFEST. Changed text files have a unified diff against the origin bytes.",
              "counts": {s: sum(1 for r in rows if r["status"] == s) for s in ("unchanged", "changed", "added")},
              "origin_files_missing_in_successor": missing, "files": rows}
    with (root / "DELTA.json").open("x") as f:
        json.dump(record, f, indent=2)
        f.write("\n")
    print(json.dumps({"counts": record["counts"], "missing": missing,
                      "changed": [r["path"] for r in rows if r["status"] == "changed"]}, indent=1))


def source(root):
    files = [pin(root, p) for p, _ in walk(root)]
    replace(root / "SOURCE-MANIFEST.json", {
        "format": "mori-r1cs-source-manifest/1",
        "rule": "Every file of the successor directory except the harness-owned PROMPT.md, run-author.py, PROCESS.json, process.stderr, FINAL-RESPONSE.md, the private provider-result.private.json (never opened), __pycache__, this manifest and its sidecar SOURCE-MANIFEST.json.sha256.",
        "build_manifest_sha256": sha((root / "BUILD-MANIFEST.json").read_bytes()),
        "files": files,
    })
    digest = sha((root / "SOURCE-MANIFEST.json").read_bytes())
    (root / SIDECAR).write_text(digest + "  SOURCE-MANIFEST.json\n")
    print(json.dumps({"files": len(files), "sha256": digest}, indent=1))


def main():
    mode, root = sys.argv[1], Path(sys.argv[2]).resolve()
    if mode == "build":
        build(root, Path(sys.argv[3]).resolve())
    elif mode == "delta":
        delta(root, Path(sys.argv[3]).resolve())
    elif mode == "source":
        source(root)
    else:
        raise SystemExit(f"unknown mode {mode}")


if __name__ == "__main__":
    main()
