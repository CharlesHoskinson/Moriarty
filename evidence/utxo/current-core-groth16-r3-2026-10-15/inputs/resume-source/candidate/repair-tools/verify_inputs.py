#!/usr/bin/env python3
"""Repair-author input verification (stdlib only, read and hash only; imports no candidate
or reviewer script). Writes one JSON record, created exclusively.

Usage: python3 -I verify_inputs.py OUT_JSON
"""
import glob
import hashlib
import json
import os
import resource
import sys

resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))

TASK = "/home/charl/moriarty-eutxo-validation-2026-10-12"
ORIGIN = TASK + "/current-core-groth16-compiler-source-r2-20261015"
REVIEW = TASK + "/current-core-groth16-compiler-independent-source-review-20261015"
ABI_V6 = TASK + "/claim-utxo-abi-v1-independent-source-review-r2-20261015"
CHECKOUT = "/home/charl/Moriarty/.worktrees/utxo-pr54-currentness-20261015"
TOOLCHAIN = "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu"


def sha_file(p):
    h = hashlib.sha256()
    with open(p, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def compact(obj):
    return json.dumps(obj, sort_keys=True, separators=(",", ":")).encode()


def check_list(root, files):
    bad = []
    for f in files:
        p = os.path.join(root, f["path"])
        got = sha_file(p) if os.path.isfile(p) else None
        if got != f["sha256"]:
            bad.append({"path": f["path"], "expected": f["sha256"], "observed": got})
    return bad


out = {"format": "repair-input-verification/1", "tool": "repair-tools/verify_inputs.py"}

# 1. Review report and origin manifests.
out["review_report_sha256"] = sha_file(REVIEW + "/REPORT.md")
out["review_report_expected"] = "a0d1725fc6ad473f3aa9b94409c7894961c536de9a852f8d2ec4b6ccbe69455f"
out["origin_source_manifest_sha256"] = sha_file(ORIGIN + "/SOURCE-MANIFEST.json")
out["origin_source_manifest_expected"] = "063beb9553fc8720e2ef485672643eabccfa10a918629fa5764efefacf973602"
sm = json.load(open(ORIGIN + "/SOURCE-MANIFEST.json"))
out["origin_source_manifest_entries"] = len(sm["files"])
out["origin_source_manifest_mismatches"] = check_list(ORIGIN, sm["files"])
bm = json.load(open(ORIGIN + "/BUILD-MANIFEST.json"))
out["origin_build_manifest_sha256"] = sha_file(ORIGIN + "/BUILD-MANIFEST.json")
out["origin_build_manifest_entries"] = len(bm["files"])
out["origin_build_manifest_mismatches"] = check_list(ORIGIN, bm["files"])

# 2. Core closure against the live checkout; HEAD read from git metadata files only.
pins = json.load(open(ORIGIN + "/CORE-CLOSURE-PINS.json"))
recomputed = hashlib.sha256(compact(sorted(pins["files"], key=lambda f: f["path"]))).hexdigest()
gitdir = open(CHECKOUT + "/.git").read().split("gitdir:", 1)[1].strip()
head = open(os.path.join(gitdir, "HEAD")).read().strip()
if head.startswith("ref:"):
    ref = head.split(" ", 1)[1]
    refp = os.path.join(gitdir, ref)
    if not os.path.exists(refp):
        common = open(os.path.join(gitdir, "commondir")).read().strip()
        refp = os.path.normpath(os.path.join(gitdir, common, ref))
    head = open(refp).read().strip() if os.path.exists(refp) else "unresolved:" + ref
out["core_closure"] = {
    "recorded_closure_sha256": pins["closure_sha256"],
    "recomputed_closure_sha256": recomputed,
    "files": len(pins["files"]),
    "live_mismatches": check_list(CHECKOUT, pins["files"]),
    "recorded_head": pins["head"],
    "live_head_from_git_metadata": head,
}

# 3. Frozen reviewer artefacts (used unchanged) and the review manifest.
rm = json.load(open(REVIEW + "/REVIEW-MANIFEST.json"))
out["review_manifest_sha256"] = sha_file(REVIEW + "/REVIEW-MANIFEST.json")
out["review_manifest_entries"] = len(rm["files"])
out["review_manifest_mismatches"] = check_list(REVIEW, rm["files"])
freeze = json.load(open(REVIEW + "/HELDOUT-FREEZE.json"))
out["heldout_freeze"] = {
    "recorded_aggregate": freeze["aggregate_sha256"],
    "recomputed_aggregate": hashlib.sha256(compact(freeze["files"])).hexdigest(),
    "file_mismatches": check_list(REVIEW, freeze["files"]),
}
vec = json.load(open(REVIEW + "/heldout/HELDOUT-VECTORS.json"))
vectors = vec.get("vectors", [])
out["heldout_counts"] = {
    "vectors": len(vectors),
    "accepted": sum(1 for v in vectors if v.get("expected", {}).get("accepted") is True),
    "rejected": sum(1 for v in vectors if v.get("expected", {}).get("accepted") is False),
    "top_level_keys": sorted(vec.keys()),
    "list_lengths": {k: len(v) for k, v in vec.items() if isinstance(v, list)},
}
out["post_freeze_addenda"] = {
    "HOSTILE-CLAIMS-ADDENDUM.json": sha_file(REVIEW + "/HOSTILE-CLAIMS-ADDENDUM.json"),
    "heldout-addendum-2/HELDOUT-ADDENDUM-2.json": sha_file(REVIEW + "/heldout-addendum-2/HELDOUT-ADDENDUM-2.json"),
}
out["review_corrected_request_sha256"] = sha_file(REVIEW + "/CORRECTED-EXECUTION-REQUEST.json")
out["harness_heldout_rs_sha256"] = sha_file(REVIEW + "/proposals/heldout-harness/tests/heldout.rs")
out["abi_v6_report_sha256"] = sha_file(ABI_V6 + "/REPORT.md")

# 4. Lock union (data only).
union = json.load(open(REVIEW + "/LOCK-UNION-ANALYSIS.json"))
reg = [p for p in union["packages"] if (p.get("source") or "").startswith("registry+")]
out["lock_union"] = {
    "packages": len(union["packages"]),
    "registry_packages": len(reg),
    "registry_with_checksum": sum(1 for p in reg if p.get("checksum")),
    "crate_sha_matches_lock": sum(1 for p in reg if p.get("crate_sha256_matches_lock")),
    "non_registry": sorted(p["name"] for p in union["packages"] if not (p.get("source") or "").startswith("registry+")),
    "registry_subset_sha256": hashlib.sha256(compact(sorted(
        [{"name": p["name"], "version": p["version"], "checksum": p["checksum"]} for p in reg],
        key=lambda r: (r["name"], r["version"])))).hexdigest(),
    "registry_subset_rule": "sha256 of compact sorted-key JSON of [{name, version, checksum}] sorted by (name, version)",
}

# 5. Toolchain bytes (hashed as data, never executed) and config/selection facts.
tc = {"root": TOOLCHAIN, "exists": os.path.isdir(TOOLCHAIN)}
for name in ("bin/cargo", "bin/rustc"):
    p = os.path.join(TOOLCHAIN, name)
    tc[name] = {"sha256": sha_file(p), "bytes": os.path.getsize(p)} if os.path.isfile(p) else None
drivers = sorted(glob.glob(TOOLCHAIN + "/lib/librustc_driver-*.so"))
tc["librustc_driver"] = [{"path": os.path.relpath(d, TOOLCHAIN), "sha256": sha_file(d), "bytes": os.path.getsize(d)} for d in drivers]
settings = "/home/charl/.rustup/settings.toml"
tc["rustup_settings_toml"] = open(settings).read() if os.path.isfile(settings) else None
tc["cargo_config_files_present"] = [p for p in ("/home/charl/.cargo/config", "/home/charl/.cargo/config.toml") if os.path.exists(p)]
tc["rust_toolchain_files_on_task_ancestors"] = [
    p for d in ("/home/charl/moriarty-eutxo-validation-2026-10-12", "/home/charl", "/home", "/tmp", "/")
    for p in (os.path.join(d, "rust-toolchain.toml"), os.path.join(d, "rust-toolchain")) if os.path.exists(p)]
tc["note"] = "Binaries hashed as bytes only; no toolchain binary was executed (no -V, no --help)."
out["toolchain"] = tc

with open(sys.argv[1], "x") as f:
    json.dump(out, f, indent=2, sort_keys=True)
    f.write("\n")
summary = {k: out[k] for k in ("review_report_sha256", "origin_source_manifest_sha256", "origin_source_manifest_mismatches",
                                "origin_build_manifest_mismatches", "review_manifest_mismatches")}
summary["core_live_mismatches"] = len(out["core_closure"]["live_mismatches"])
summary["core_heads"] = (out["core_closure"]["recorded_head"], out["core_closure"]["live_head_from_git_metadata"])
summary["closure_recomputed_ok"] = recomputed == pins["closure_sha256"]
summary["freeze_ok"] = out["heldout_freeze"]["recomputed_aggregate"] == freeze["aggregate_sha256"] and not out["heldout_freeze"]["file_mismatches"]
summary["heldout_counts"] = out["heldout_counts"]
summary["lock_union"] = {k: out["lock_union"][k] for k in ("packages", "registry_packages", "crate_sha_matches_lock")}
summary["toolchain_bins"] = {k: tc[k] for k in ("bin/cargo", "bin/rustc")}
summary["drivers"] = len(tc["librustc_driver"])
summary["abi_v6"] = out["abi_v6_report_sha256"]
print(json.dumps(summary, indent=1))
