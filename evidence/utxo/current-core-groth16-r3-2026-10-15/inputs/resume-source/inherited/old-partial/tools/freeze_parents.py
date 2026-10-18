#!/usr/bin/env python3
"""Freeze the immutable parent pins of this evidence repair (stdlib only).

Read-only on every parent. Opens only (a) explicitly named public files and (b) files
listed by an explicitly named, digest-pinned manifest. Never opens, stats or hashes a
path containing "private". Writes exactly one new file, PARENT-PINS.json, in this
directory (created exclusively; an existing file is a STOP).

Usage: python3 -I tools/freeze_parents.py
"""
import hashlib
import json
import os
import resource
import sys
from datetime import datetime, timezone

resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))

BASE = "/home/charl/moriarty-eutxo-validation-2026-10-12"
HERE = os.path.join(BASE, "current-core-groth16-evidence-repair-source-20261015")
CORE_CHECKOUT = "/home/charl/Moriarty/.worktrees/utxo-pr54-currentness-20261015"
PARENT = os.path.join(BASE, "current-core-groth16-compiler-review-repair-source-20261015")
PACKET = os.path.join(BASE, "current-core-groth16-compiler-independent-source-review-20261015")
REPAIR_REVIEW = os.path.join(BASE, "current-core-groth16-compiler-repair-independent-review-20261015")
R2 = os.path.join(REPAIR_REVIEW, "plumbing-r2")
RUN = os.path.join(BASE, "current-core-groth16-env-corrected-offline-execution-20261015")
RESULT_REVIEW = os.path.join(BASE, "current-core-groth16-independent-result-review-20261015")

# Digests stated by the user or by the reviewed records; each is compared with bytes.
EXPECT = {
    "parent_source_manifest": (os.path.join(PARENT, "SOURCE-MANIFEST.json"), "a6e17efd0a47194c70ac6e4467efce2d47f9254dc47a8e596f6add2b92aef97e"),
    "parent_build_manifest": (os.path.join(PARENT, "BUILD-MANIFEST.json"), "4e3ba2e2796b425598b9b2abc022c4d6782bf032044aa53e124af4946c543b47"),
    "packet_review_manifest": (os.path.join(PACKET, "REVIEW-MANIFEST.json"), "757ed0bfc5804eec8bac3f1db063b1239ead0cdbeccbc5fa3ec3160a1a6ab151"),
    "packet_hostile_addendum": (os.path.join(PACKET, "HOSTILE-CLAIMS-ADDENDUM.json"), "bdc444016f9edefe5cd252e176c5ed302eaba2e0b3e5806ff5c68094e610e078"),
    "packet_addendum_2": (os.path.join(PACKET, "heldout-addendum-2", "HELDOUT-ADDENDUM-2.json"), "66b91c46806f2bad97e7b46c1026d9c724b27e40516ba01f10ec87acbc12f33f"),
    "r2_manifest": (os.path.join(R2, "R2-MANIFEST.json"), "5a256c766908ff521b6a6cf483d1b109d17aa2d931a63598ab39b183cc2b580b"),
    "run_report": (os.path.join(RUN, "REPORT.md"), "18afb040ff7b961543f5b8dc478faa42044a4e4129f4f6dfed58349d1ac84cc1"),
    "result_review_report": (os.path.join(RESULT_REVIEW, "REPORT.md"), "06e316bcf575c9258c292c29c84712ab2be7ff21be735add8c236f0be47d6ece"),
    "result_review_fix_proposals": (os.path.join(RESULT_REVIEW, "proposals", "FIX-PROPOSALS.md"), "d7d3e40278a6e4a3a721003d0f6afd072f41ffa11d4bd254161f6e0a5fa2f4b7"),
}
CORE_CLOSURE_EXPECTED = "f47318a26a4df0477e89962a9de0e9b48f12d01d0e9b4a8f5e74603beaa235ed"
COMPILER_BUILD_DIGEST_OBSERVED = "a6911ddd6b3883dedfa4c155e4924d98fc645d41d72806d4d6bcd0c398366d44"

# Additional named public files pinned by bytes (no expectation; recorded).
NAMED = [
    os.path.join(PACKET, "HELDOUT-FREEZE.json"),
    os.path.join(PACKET, "heldout", "HELDOUT-VECTORS.json"),
    os.path.join(REPAIR_REVIEW, "OUTPUT-MANIFEST.json"),
    os.path.join(REPAIR_REVIEW, "R2-PLUMBING-EXECUTION-REQUEST.json"),
    os.path.join(REPAIR_REVIEW, "R2-HARNESS.diff"),
    os.path.join(RUN, "OUTPUT-MANIFEST.json"),
    os.path.join(RUN, "FREEZE.json"),
    os.path.join(RUN, "ANALYSIS.json"),
    os.path.join(RUN, "ENV-R2-EXACT-PROPOSAL.md"),
    os.path.join(RUN, "run", "RESULT.json"),
    os.path.join(RUN, "run", "E4.stdout"),
    os.path.join(RUN, "run", "E5-r2.stdout"),
    os.path.join(RUN, "run", "E0r2.stdout"),
    os.path.join(RUN, "tools", "run_pathb.py"),
    os.path.join(RUN, "tools", "analyze.py"),
    os.path.join(RESULT_REVIEW, "REVIEW-MANIFEST.json"),
    os.path.join(RESULT_REVIEW, "RECOMPUTE.json"),
    os.path.join(RESULT_REVIEW, "RECONCILE.json"),
]


def sha(b):
    return hashlib.sha256(b).hexdigest()


def read(p):
    if "private" in p:
        raise SystemExit(f"refusing a private path: {p}")
    with open(p, "rb") as f:
        return f.read()


def compact(obj):
    return json.dumps(obj, sort_keys=True, separators=(",", ":")).encode()


def entries(manifest):
    """(path, sha256) pairs from the manifest formats used in this lineage."""
    files = manifest.get("files") if isinstance(manifest, dict) else None
    if isinstance(files, list):
        return [(f["path"], f["sha256"]) for f in files]
    if isinstance(files, dict):
        return [(p, v["sha256"]) for p, v in files.items()]
    if isinstance(manifest, dict) and all(isinstance(v, dict) and "sha256" in v for v in manifest.values()):
        return [(p, v["sha256"]) for p, v in manifest.items()]
    raise SystemExit("unknown manifest format")


def verify_manifest(root, manifest_path):
    manifest = json.loads(read(manifest_path))
    rows = entries(manifest)
    bad, skipped = [], []
    for rel, digest in rows:
        if "private" in rel:
            skipped.append(rel)
            continue
        p = os.path.join(root, rel)
        if not os.path.isfile(p) or os.path.islink(p):
            bad.append({"path": rel, "problem": "missing or not a regular file"})
        elif sha(read(p)) != digest:
            bad.append({"path": rel, "problem": "sha256 differs"})
    return {"manifest": manifest_path, "manifest_sha256": sha(read(manifest_path)), "entries": len(rows),
            "verified_equal": len(rows) - len(bad) - len(skipped), "mismatches": bad, "private_names_skipped": skipped}


def main():
    out_path = os.path.join(HERE, "PARENT-PINS.json")
    stops = []
    rec = {"format": "evidence-repair-parent-pins/1",
           "author": "claude-opus-5-5 (evidence-repair source author)",
           "frozen_at_utc": datetime.now(timezone.utc).isoformat(),
           "rule": "Frozen before any successor source, harness or expectation byte is edited. Every digest below is of bytes read now; the stated digests are compared, never copied.",
           "expected": {}, "named": {}, "manifests": {}}
    for key, (p, digest) in EXPECT.items():
        got = sha(read(p))
        rec["expected"][key] = {"path": p, "sha256": got, "stated": digest, "equal": got == digest}
        if got != digest:
            stops.append(f"{key}: {got} != stated {digest}")
    for p in NAMED:
        b = read(p)
        rec["named"][os.path.relpath(p, BASE)] = {"sha256": sha(b), "bytes": len(b)}
    rec["manifests"]["parent_source_manifest"] = verify_manifest(PARENT, os.path.join(PARENT, "SOURCE-MANIFEST.json"))
    rec["manifests"]["parent_build_manifest"] = verify_manifest(PARENT, os.path.join(PARENT, "BUILD-MANIFEST.json"))
    rec["manifests"]["packet_review_manifest"] = verify_manifest(PACKET, os.path.join(PACKET, "REVIEW-MANIFEST.json"))
    rec["manifests"]["r2_manifest"] = verify_manifest(R2, os.path.join(R2, "R2-MANIFEST.json"))
    rec["manifests"]["run_output_manifest"] = verify_manifest(RUN, os.path.join(RUN, "OUTPUT-MANIFEST.json"))
    rec["manifests"]["repair_review_output_manifest"] = verify_manifest(REPAIR_REVIEW, os.path.join(REPAIR_REVIEW, "OUTPUT-MANIFEST.json"))
    rec["manifests"]["result_review_manifest"] = verify_manifest(RESULT_REVIEW, os.path.join(RESULT_REVIEW, "REVIEW-MANIFEST.json"))
    for k, m in rec["manifests"].items():
        if m["mismatches"]:
            stops.append(f"{k}: {len(m['mismatches'])} mismatches")

    # Core closure recomputed from the live currentness checkout by the pinned rule.
    pins_path = os.path.join(PARENT, "CORE-CLOSURE-PINS.json")
    pins = json.loads(read(pins_path))
    rows = []
    for f in sorted(pins["files"], key=lambda f: f["path"]):
        b = read(os.path.join(CORE_CHECKOUT, f["path"]))
        rows.append({"path": f["path"], "sha256": sha(b), "bytes": len(b)})
    closure = sha(compact(rows))
    rec["core_closure"] = {"checkout": CORE_CHECKOUT, "pins": pins_path, "pins_sha256": sha(read(pins_path)),
                           "files": len(rows), "recomputed": closure, "pinned": pins["closure_sha256"],
                           "expected": CORE_CLOSURE_EXPECTED, "equal": closure == CORE_CLOSURE_EXPECTED == pins["closure_sha256"],
                           "rows_differing_from_pins": [r["path"] for r, f in zip(rows, sorted(pins["files"], key=lambda f: f["path"])) if r["sha256"] != f["sha256"]]}
    if not rec["core_closure"]["equal"]:
        stops.append("Core closure differs")

    # compiler_build_digest method check: SHA-256 of compact sorted-key JSON of
    # CompilerBuild (Digest32 serialises as lowercase hex), as the result review stated.
    method = sha(compact({"compiler_source_manifest_sha256": EXPECT["parent_build_manifest"][1], "core_closure_sha256": CORE_CLOSURE_EXPECTED}))
    rec["compiler_build_digest_method_check"] = {"recomputed_for_parent": method, "observed_in_actual_run": COMPILER_BUILD_DIGEST_OBSERVED,
                                                 "equal": method == COMPILER_BUILD_DIGEST_OBSERVED}
    if method != COMPILER_BUILD_DIGEST_OBSERVED:
        stops.append("compiler_build_digest method does not reproduce the observed parent value")

    rec["stops"] = stops
    rec["verdict"] = "PASS" if not stops else "STOP"
    with open(out_path, "x") as f:
        json.dump(rec, f, indent=1, sort_keys=True)
        f.write("\n")
    print(json.dumps({"verdict": rec["verdict"], "stops": stops,
                      "manifests": {k: (m["entries"], m["verified_equal"], len(m["private_names_skipped"])) for k, m in rec["manifests"].items()},
                      "core_closure": (rec["core_closure"]["files"], rec["core_closure"]["equal"]),
                      "build_digest_method": rec["compiler_build_digest_method_check"]["equal"],
                      "sha256": sha(read(out_path))}, indent=1))
    sys.exit(0 if not stops else 3)


if __name__ == "__main__":
    main()
