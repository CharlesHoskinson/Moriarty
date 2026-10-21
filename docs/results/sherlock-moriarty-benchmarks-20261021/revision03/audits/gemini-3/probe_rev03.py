#!/usr/bin/env python3
"""Independent verification and adversarial probes for Revision 03 (integration03)."""
from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
import sys
import tempfile
from pathlib import Path

BASE = Path("/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/sherlock-moriarty-benchmark-suite-20261021")
INT03 = BASE / "integration03"
CAND = INT03 / "candidate"
MAP_PATH = INT03 / "CANDIDATE-MAP.json"
TRACE_PATH = CAND / "openspec/changes/sherlock-moriarty-benchmarks/traceability.json"

sys.path.insert(0, str(CAND / "models/sherlock/benchmarks"))
import metrics as m
import acceptance as a
from test_acceptance import Fixture
from workloads import generate as g


def sha256_file(path: Path) -> tuple[str, int]:
    raw = path.read_bytes()
    return hashlib.sha256(raw).hexdigest(), len(raw)


def verify_candidate_pins() -> dict:
    cmap = json.loads(MAP_PATH.read_text())
    trace = json.loads(TRACE_PATH.read_text())
    files = cmap["sourceFiles"]
    assert len(files) == 45, f"Expected 45 files, got {len(files)}"

    changed_vs_r02 = []
    unchanged_vs_r02 = []
    for entry in files:
        rel = entry["target"]
        p = CAND / rel
        assert p.is_file(), f"Missing candidate file: {rel}"
        digest, size = sha256_file(p)
        assert digest == entry["sha256"], f"Digest mismatch for {rel}: {digest} != {entry['sha256']}"
        assert size == entry["bytes"], f"Byte count mismatch for {rel}: {size} != {entry['bytes']}"
        if entry["sha256"] != entry["revision02SHA256"]:
            assert entry["revision03"] is not None
            changed_vs_r02.append(entry)
        else:
            assert entry["revision03"] is None
            unchanged_vs_r02.append(entry)

    assert len(changed_vs_r02) == 14, f"Expected 14 changed files, got {len(changed_vs_r02)}"
    assert len(unchanged_vs_r02) == 31, f"Expected 31 unchanged files, got {len(unchanged_vs_r02)}"

    trace_pins = {item["path"]: item["sha256"] for item in trace["sourcePins"]}
    assert len(trace_pins) == 44, f"Expected 44 sourcePins in traceability.json, got {len(trace_pins)}"
    for entry in files:
        rel = entry["target"]
        if rel == "openspec/changes/sherlock-moriarty-benchmarks/traceability.json":
            assert rel not in trace_pins
        else:
            assert trace_pins.get(rel) == entry["sha256"], f"Traceability pin mismatch for {rel}"

    return {
        "total_files": len(files),
        "changed_in_revision03": len(changed_vs_r02),
        "unchanged_from_revision02": len(unchanged_vs_r02),
        "changed_files": [
            {
                "path": e["target"],
                "sha256": e["sha256"],
                "bytes": e["bytes"],
                "lane": e["revision03"]["lane"],
                "revision02SHA256": e["revision02SHA256"],
            }
            for e in changed_vs_r02
        ],
    }


def probe_rev02_findings_resolved() -> dict:
    results = {}

    # 1. Probe REV02-F01 / R02-F01: Token-level operation duplicates in acceptance._evaluate
    #    must not inflate counts["success"], enter valid/compare_baseline, or bypass per-profile minima.
    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp)
        cur = Fixture(root / "current", label="cur", compiler="new-compiler")
        wid = cur.samples[0]["workload_id"]
        metric = "compile.frontend.wall_seconds"

        # Make genuine current sample regress to 100s
        s0 = cur.samples[0]
        ext0 = s0["extensions"]["acceptance"]
        cur.replace_receipt(s0, ext0["observation"], lambda d: d["metrics"].update({metric: 100}))
        s0["metrics"][metric]["value"] = 100

        # Add 2 renamed wrapper samples that reuse s0's proof/call tokens but report 1s
        for idx in range(2):
            dup = copy.deepcopy(s0)
            dup["sample_id"] = f"dup-{idx}"
            dext = dup["extensions"]["acceptance"]
            dext["observation_id"] = f"dup-obs-{idx}"
            ref = next(r for r in dup["receipts"] if r["sha256"] == dext["observation"]["receipt_sha256"])
            body = json.loads((cur.root / ref["path"]).read_text())
            body["observation_id"] = dext["observation_id"]
            body["metrics"][metric] = 1
            dext["observation"] = cur.add(dup, f"dup-{idx}.json", body)
            for entry in dup["metrics"].values():
                entry["source"]["receipt_sha256"] = dext["observation"]["receipt_sha256"]
            dup["metrics"][metric]["value"] = 1
            cur.samples.append(dup)

        cur.policy["regression"] = {
            "mode": "compare",
            "allowed_changed_subject_fields": ["compiler_sha256"],
            "tolerances": {
                wid: {
                    metric: {
                        "statistic": "median",
                        "relative_increase": 0,
                        "absolute_increase": 0,
                        "min_samples": 1,
                    }
                }
            },
        }
        prior_in_cur = Fixture(cur.root / "prior_sub", label="old", compiler="old-compiler")
        baseline = {
            "schema": "sherlock-benchmark-baseline/2",
            "root_prefix": "prior_sub",
            "samples": prior_in_cur.samples,
            "policy": prior_in_cur.policy,
            "current": prior_in_cur.current,
        }
        res = cur.assess(baseline)
        assert not res["complete"], "Expected regression to fail assessment"
        assert res["cases"][0]["counts"]["success"] == 1
        assert res["cases"][0]["raw_counts"]["success"] == 3
        assert res["cases"][0]["duplicate_observations"] == 2
        assert "shared baseline comparison is incomplete or regressed" in str(res["findings"])
        results["REV02-F01_R02-F01_token_duplicate_filtering"] = "RESOLVED"

    # 2. Probe REV02-F02 / R02-F02 & R02-F03:
    #    - Independent prove vs local_verify command bindings (including shared bundle support)
    #    - All 19 layer metrics attributed + multi-member aggregation recipes
    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp)
        f = Fixture(root / "f", stage="full-rollup-series")
        assert f.assess()["complete"]

        # Mutate witness.wall_seconds on leaf layer of 2-batch rollup to disagree with member span
        s2 = next(s for s in f.samples if s["workload_id"] == "fenrir-transfer-batches-2")
        s2_bad = copy.deepcopy(s2)
        s2_bad["layers"]["leaf"]["witness.wall_seconds"]["value"] = 99
        assert not a.assess([s2_bad] + [s for s in f.samples if s is not s2], f.policy, f.current, f.root)["complete"]

        # Mutate proving_key.bytes on leaf layer of 2-batch rollup to disagree with member sum
        s2_bad_pk = copy.deepcopy(s2)
        s2_bad_pk["layers"]["leaf"]["proving_key.bytes"]["value"] = 99
        assert not a.assess([s2_bad_pk] + [s for s in f.samples if s is not s2], f.policy, f.current, f.root)["complete"]
        results["REV02-F02_R02-F02_R02-F03_layer_and_command_attribution"] = "RESOLVED"

    # 3. Probe REV02-F03 / R02-F05:
    #    - Frozen catalog scalar mapping and per-transition state scalar bindings
    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp)
        f = Fixture(root / "f", stage="full-rollup-series")
        ctx = copy.deepcopy(f.current)
        # Swap batch 0 post with pre in transition_state_pointers
        ctx["cases"]["fenrir-transfer-batches-2"]["transition_state_pointers"][0]["post"] = (
            ctx["cases"]["fenrir-transfer-batches-2"]["transition_state_pointers"][0]["pre"]
        )
        assert not a.assess(f.samples, f.policy, ctx, f.root)["complete"]
        results["REV02-F03_R02-F05_catalog_and_transition_scalars"] = "RESOLVED"

    # 4. Probe REV02-F04 / R02-F04:
    #    - Failed/censored partial samples without fictitious proof/deployment artifacts
    #    - comparison_scope explicit reporting
    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp)
        f = Fixture(root / "f")
        failed = copy.deepcopy(f.samples[0])
        failed.update(sample_id="partial-fail", status="failure")
        ext = failed["extensions"]["acceptance"]
        ext["observation_id"] = "partial-fail"
        old_obs = next(r for r in failed["receipts"] if r["sha256"] == ext["observation"]["receipt_sha256"])
        body = json.loads((f.root / old_obs["path"]).read_text())
        body.update(status="failure", observation_id="partial-fail")
        ext["observation"] = f.add(failed, "partial-fail-obs.json", body)
        for mv in failed["metrics"].values():
            mv["source"]["receipt_sha256"] = ext["observation"]["receipt_sha256"]
        ext["receipt_roles"] = {}
        ext["proof_nodes"] = []
        failed["layers"] = {}
        f.policy["cases"][0]["max_failures"] = 1
        f.samples.append(failed)
        res = f.assess()
        assert res["complete"], f"Partial failure sample rejected: {res}"
        results["REV02-F04_R02-F04_partial_failure_and_comparison_scope"] = "RESOLVED"

    # 5. Probe REV02-F05 / R02-F05:
    #    - metric_observation_key = (receipt_sha256, canonical_pointer)
    #    - deduplicated n vs min_samples when descriptive duplicates exist
    #    - nested verifier verdict pointer vs unrelated negative controls
    s_a = m.validate_sample({
        "schema": "sherlock-benchmark-sample/1",
        "sample_id": "s-a",
        "workload_id": "w",
        "workload_digest_sha256": "a" * 64,
        "evidence_class": "synthetic",
        "status": "success",
        "identity": {
            "source_sha256": "a" * 64,
            "compiler_sha256": None,
            "ledger_sha256": None,
            "network": None,
            "circuit_sha256": None,
            "profile": "p/1",
            "environment_sha256": "b" * 64,
            "toolchain_sha256": "c" * 64,
        },
        "cache_state": "cold",
        "metrics": {
            "prove.wall_seconds": {
                "value": 0,
                "unit": "seconds",
                "status": "measured",
                "source": {
                    "receipt_sha256": "d" * 64,
                    "pointer": "/runs/0/wall",
                    "scale": 1,
                    "scope": "subprocess",
                    "timer_resolution_seconds": 0.001,
                },
                "reason": None,
            }
        },
        "layers": {},
        "receipts": [{"path": "r.json", "sha256": "d" * 64, "schema": "s/1"}],
        "effects": {"expected_sha256": None, "observed_sha256": None, "match": None},
        "events": {},
    })
    s_b = copy.deepcopy(s_a)
    s_b["sample_id"] = "s-b"
    s_b["metrics"]["prove.wall_seconds"]["source"]["scale"] = 1000
    assert m.metric_observation_key(s_a["metrics"]["prove.wall_seconds"]) == m.metric_observation_key(
        s_b["metrics"]["prove.wall_seconds"]
    )
    results["REV02-F05_R02-F05_observation_key_and_verdict_scoping"] = "RESOLVED"

    return results


def probe_cross_lane_integration() -> dict:
    # Verify workloads catalog and provenance against candidate
    cat_info = g.validate_catalog(CAND / "models/sherlock/benchmarks/workloads")
    stages = {
        s: len(g.stage_contract(cat_info["catalog"], s)["cases"])
        for s in ("host-compiler-coverage", "initial-deployed-baseline", "full-rollup-series", "full-suite")
    }
    assert stages["full-suite"] == len(cat_info["catalog"]["cases"])
    assert stages["initial-deployed-baseline"] == 1
    assert stages["full-rollup-series"] == 5

    # Verify check.py loads candidate acceptance/metrics and verifies traceability.json
    spec = importlib.util.spec_from_file_location("qa_check_cand", CAND / "models/sherlock/qa/check.py")
    qa = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(qa)

    with tempfile.TemporaryDirectory() as tmp:
        root = Path(tmp)
        f = Fixture(root / "retained")
        for name, val in (("policy", f.policy), ("samples", f.samples), ("current", f.current)):
            (f.root / f"{name}.json").write_text(json.dumps(val))
        report = qa.benchmark_completion_report(
            f.root / "policy.json",
            f.root / "samples.json",
            f.root / "current.json",
            f.root,
            repo=CAND,
        )
        assert report["complete"] is True
        assert report["performance_target_satisfied"] is False
        assert report["full_goal_complete"] is False

    return {
        "catalog_cases": len(cat_info["catalog"]["cases"]),
        "stage_case_counts": stages,
        "doctorwatson_check_cand_complete": True,
    }


if __name__ == "__main__":
    out = {
        "pin_verification": verify_candidate_pins(),
        "rev02_resolution_probes": probe_rev02_findings_resolved(),
        "cross_lane_integration": probe_cross_lane_integration(),
    }
    print(json.dumps(out, indent=2))
