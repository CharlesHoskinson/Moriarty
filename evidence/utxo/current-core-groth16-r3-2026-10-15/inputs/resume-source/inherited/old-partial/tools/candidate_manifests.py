#!/usr/bin/env python3
"""Manifests for the evidence-repair successor candidate (stdlib only; imports no
candidate, parent or operator script).

Modes:
  build   rewrite candidate/BUILD-MANIFEST.json in the UNCHANGED parent format
          mori-r1cs-build-manifest/1 (P-2: no format redesign). Annotation fields keep
          their parent meaning: origin_build_manifest_sha256 is the origin-r2
          BUILD-MANIFEST digest and vs_origin_r2 compares each file with the origin-r2
          SOURCE-MANIFEST pins kept at candidate/history/origin-r2/.
  source  rewrite candidate/SOURCE-MANIFEST.json (+ sidecar) over every candidate file
          except the manifest and its sidecar.
  delta   write DELTA.json and DIFFS/*.diff in this directory (created exclusively):
          every candidate file compared with the parent SOURCE-MANIFEST a6e17efd...
  digest  print the successor compiler_build_digest predicted by stdlib arithmetic
          (SHA-256 of compact sorted-key JSON of CompilerBuild), with the parent value
          reproduced as a method control. The actual value is whatever the reviewed
          harness prints in an authorized run.

Usage: python3 -I tools/candidate_manifests.py MODE
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

BASE = Path("/home/charl/moriarty-eutxo-validation-2026-10-12")
HERE = BASE / "current-core-groth16-evidence-repair-source-20261015"
CAND = HERE / "candidate"
PARENT = BASE / "current-core-groth16-compiler-review-repair-source-20261015"
PARENT_SOURCE_MANIFEST = "a6e17efd0a47194c70ac6e4467efce2d47f9254dc47a8e596f6add2b92aef97e"
PARENT_BUILD_MANIFEST = "4e3ba2e2796b425598b9b2abc022c4d6782bf032044aa53e124af4946c543b47"
ORIGIN_R2_SOURCE_MANIFEST = "063beb9553fc8720e2ef485672643eabccfa10a918629fa5764efefacf973602"
ORIGIN_R2_BUILD_MANIFEST = "2c943b511bb6cb912889728ed2c5de90d4257939ded53b1d626b998edd3beb21"
CORE_CLOSURE = "f47318a26a4df0477e89962a9de0e9b48f12d01d0e9b4a8f5e74603beaa235ed"
PARENT_COMPILER_BUILD_DIGEST_OBSERVED = "a6911ddd6b3883dedfa4c155e4924d98fc645d41d72806d4d6bcd0c398366d44"
SIDECAR = "SOURCE-MANIFEST.json.sha256"


def sha(b):
    return hashlib.sha256(b).hexdigest()


def compact(obj):
    return json.dumps(obj, sort_keys=True, separators=(",", ":")).encode()


def pin(root, p):
    data = p.read_bytes()
    return {"path": p.relative_to(root).as_posix(), "sha256": sha(data), "bytes": len(data)}


def replace(path, record):
    tmp = path.with_name(path.name + ".tmp")
    with tmp.open("x") as f:
        json.dump(record, f, indent=2)
        f.write("\n")
    os.replace(tmp, path)


def build():
    origin_sm = CAND / "history" / "origin-r2" / "SOURCE-MANIFEST.json"
    origin_bm = CAND / "history" / "origin-r2" / "BUILD-MANIFEST.json"
    if sha(origin_sm.read_bytes()) != ORIGIN_R2_SOURCE_MANIFEST or sha(origin_bm.read_bytes()) != ORIGIN_R2_BUILD_MANIFEST:
        sys.exit("STOP: origin-r2 history bytes differ from their pins")
    prior = {f["path"]: f["sha256"] for f in json.loads(origin_sm.read_text())["files"]}
    crate = CAND / "source" / "mori-r1cs"
    files = [pin(CAND, crate / "Cargo.toml")] + [pin(CAND, p) for p in sorted((crate / "src").rglob("*")) if p.is_file()]
    for f in files:
        f["vs_origin_r2"] = "new" if f["path"] not in prior else ("unchanged" if prior[f["path"]] == f["sha256"] else "changed")
    parent = json.loads((PARENT / "BUILD-MANIFEST.json").read_text())
    record = {"format": parent["format"], "rule": parent["rule"], "origin_build_manifest_sha256": sha(origin_bm.read_bytes()), "files": files}
    replace(CAND / "BUILD-MANIFEST.json", record)
    pf = {f["path"]: f for f in parent["files"]}
    print(json.dumps({"files": len(files), "sha256": sha((CAND / "BUILD-MANIFEST.json").read_bytes()),
                      "vs_parent_4e3_changed": [f["path"] for f in files if pf.get(f["path"], {}).get("sha256") != f["sha256"]],
                      "keys_equal_to_parent": sorted(record) == sorted(parent)}, indent=1))


def walk():
    for p in sorted(CAND.rglob("*")):
        if not p.is_file():
            continue
        rel = p.relative_to(CAND).as_posix()
        if rel in ("SOURCE-MANIFEST.json", SIDECAR) or "__pycache__" in rel or rel.endswith(".tmp"):
            continue
        yield p, rel


def source():
    files = [pin(CAND, p) for p, _ in walk()]
    replace(CAND / "SOURCE-MANIFEST.json", {
        "format": "mori-r1cs-source-manifest/1",
        "rule": "Every file of the evidence-repair successor candidate directory (candidate/) except this manifest and its sidecar SOURCE-MANIFEST.json.sha256. The successor directory's own records, tools, harness overlay and harness-owned files live outside candidate/ and are not listed.",
        "build_manifest_sha256": sha((CAND / "BUILD-MANIFEST.json").read_bytes()),
        "files": files,
    })
    digest = sha((CAND / "SOURCE-MANIFEST.json").read_bytes())
    (CAND / SIDECAR).write_text(digest + "  SOURCE-MANIFEST.json\n")
    print(json.dumps({"files": len(files), "sha256": digest}, indent=1))


def delta():
    pm = (PARENT / "SOURCE-MANIFEST.json").read_bytes()
    if sha(pm) != PARENT_SOURCE_MANIFEST:
        sys.exit("STOP: parent SOURCE-MANIFEST differs")
    prior = {f["path"]: f["sha256"] for f in json.loads(pm)["files"]}
    diffs = HERE / "DIFFS"
    diffs.mkdir()
    rows, seen = [], set()
    for p, rel in walk():
        seen.add(rel)
        now = sha(p.read_bytes())
        if rel not in prior:
            rows.append({"path": rel, "status": "added", "sha256": now})
            continue
        if prior[rel] == now:
            rows.append({"path": rel, "status": "unchanged", "sha256": now})
            continue
        a = (PARENT / rel).read_text(encoding="utf-8").splitlines(keepends=True)
        b = p.read_text(encoding="utf-8").splitlines(keepends=True)
        text = "".join(difflib.unified_diff(a, b, fromfile=f"parent-a6e17efd/{rel}", tofile=f"successor/{rel}", n=3))
        name = rel.replace("/", "__") + ".diff"
        (diffs / name).write_text(text, encoding="utf-8")
        rows.append({"path": rel, "status": "changed", "parent_sha256": prior[rel], "sha256": now, "diff": f"DIFFS/{name}",
                     "diff_sha256": sha((diffs / name).read_bytes()),
                     "lines_removed": sum(1 for l in text.splitlines() if l.startswith("-") and not l.startswith("---")),
                     "lines_added": sum(1 for l in text.splitlines() if l.startswith("+") and not l.startswith("+++"))})
    missing = sorted(set(prior) - seen)
    record = {"format": "evidence-repair-delta/1", "parent": str(PARENT), "parent_source_manifest_sha256": PARENT_SOURCE_MANIFEST,
              "rule": "Every successor candidate file (except its SOURCE-MANIFEST and sidecar) compared by SHA-256 with the parent SOURCE-MANIFEST. Changed text files have a unified diff against the parent bytes.",
              "counts": {s: sum(1 for r in rows if r["status"] == s) for s in ("unchanged", "changed", "added")},
              "parent_files_missing_in_successor": missing, "files": rows}
    with (HERE / "DELTA.json").open("x") as f:
        json.dump(record, f, indent=2)
        f.write("\n")
    print(json.dumps({"counts": record["counts"], "missing": missing, "changed": [r["path"] for r in rows if r["status"] == "changed"]}, indent=1))


def digest():
    bm = sha((CAND / "BUILD-MANIFEST.json").read_bytes())
    control = sha(compact({"compiler_source_manifest_sha256": PARENT_BUILD_MANIFEST, "core_closure_sha256": CORE_CLOSURE}))
    successor = sha(compact({"compiler_source_manifest_sha256": bm, "core_closure_sha256": CORE_CLOSURE}))
    print(json.dumps({"build_manifest_sha256": bm, "core_closure_sha256": CORE_CLOSURE,
                      "method_control_parent": control, "method_control_equal_observed": control == PARENT_COMPILER_BUILD_DIGEST_OBSERVED,
                      "successor_compiler_build_digest_predicted": successor}, indent=1))


if __name__ == "__main__":
    {"build": build, "source": source, "delta": delta, "digest": digest}[sys.argv[1]]()
