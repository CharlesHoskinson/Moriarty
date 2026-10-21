#!/usr/bin/env python3
"""Bounded pure-stdlib verification probe for Sherlock/Moriarty benchmark suite revision03."""

from __future__ import annotations

import copy
import hashlib
import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path("/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/sherlock-moriarty-benchmark-suite-20261021")
CAND03 = ROOT / "integration03" / "candidate"
CAND02 = ROOT / "integration02" / "candidate"
MAP03 = ROOT / "integration03" / "CANDIDATE-MAP.json"
OUT03 = ROOT / "audits03" / "gemini-2"

for p in (
    CAND03,
    CAND03 / "models" / "sherlock" / "benchmarks",
    CAND03 / "models" / "sherlock" / "benchmarks" / "workloads",
    CAND03 / "models" / "sherlock" / "qa",
):
    sys.path.insert(0, str(p))

import acceptance  # noqa: E402
import check as qa_check  # noqa: E402
import generate  # noqa: E402
import metrics  # noqa: E402
from test_acceptance import Fixture  # noqa: E402


def sha256_file(path: Path) -> tuple[str, int]:
    data = path.read_bytes()
    return hashlib.sha256(data).hexdigest(), len(data)


def main() -> None:
    results: dict[str, object] = {}

    # 1. Verify all 45 candidate files against CANDIDATE-MAP.json and integration02/candidate
    cmap = json.loads(MAP03.read_text(encoding="utf-8"))
    files = cmap["sourceFiles"]
    assert len(files) == 45, f"Expected 45 files, got {len(files)}"

    changed_vs_02: list[str] = []
    unchanged_vs_02: list[str] = []
    pin_checks: dict[str, dict[str, object]] = {}

    for entry in files:
        rel = entry["target"]
        p03 = CAND03 / rel
        p02 = CAND02 / rel
        h03, b03 = sha256_file(p03)
        h02, b02 = sha256_file(p02)
        assert h03 == entry["sha256"], f"Hash mismatch for {rel}: {h03} != {entry['sha256']}"
        assert b03 == entry["bytes"], f"Bytes mismatch for {rel}: {b03} != {entry['bytes']}"
        assert h02 == entry["revision02SHA256"], f"Rev02 hash mismatch for {rel}: {h02} != {entry['revision02SHA256']}"
        if entry["revision03"] is not None:
            changed_vs_02.append(rel)
            assert h03 != h02, f"Expected changed hash vs rev02 for {rel}"
            assert h03 == entry["revision03"]["sha256"]
            assert b03 == entry["revision03"]["bytes"]
        else:
            unchanged_vs_02.append(rel)
            assert h03 == h02 and b03 == b02, f"Expected unchanged vs rev02 for {rel}"
        pin_checks[rel] = {
            "sha256": h03,
            "bytes": b03,
            "revision02SHA256": h02,
            "changed_in_revision03": entry["revision03"] is not None,
            "lane": entry["revision03"]["lane"] if entry["revision03"] else "unchanged",
        }

    assert len(changed_vs_02) == 14
    assert len(unchanged_vs_02) == 31
    results["candidate_map_check"] = {
        "total_files": len(files),
        "changed_in_revision03_count": len(changed_vs_02),
        "unchanged_in_revision03_count": len(unchanged_vs_02),
        "changed_files": changed_vs_02,
        "changed_pins": {k: pin_checks[k] for k in changed_vs_02},
        "unchanged_core_pins": {
            k: pin_checks[k]
            for k in (
                "models/sherlock/benchmarks/workloads/generate.py",
                "models/sherlock/benchmarks/workloads/catalog.json",
                "models/sherlock/benchmarks/workloads/SOURCE-PROVENANCE.json",
                "models/sherlock/qa/check.py",
            )
        },
    }

    # 2. Verify traceability.json sourcePins match CAND03 files
    trace_path = CAND03 / "openspec" / "changes" / "sherlock-moriarty-benchmarks" / "traceability.json"
    trace = json.loads(trace_path.read_text(encoding="utf-8"))
    assert len(trace["sourcePins"]) == 44
    for meta in trace["sourcePins"]:
        rel = meta["path"]
        h, _ = sha256_file(CAND03 / rel)
        assert h == meta["sha256"], f"traceability sha256 mismatch on {rel}"
    results["traceability_source_pins_verified"] = len(trace["sourcePins"])

    # 3. Verify workload generator, acceptance.assess, and qa/check.py benchmark_completion_report
    gen_ok = generate.validate_catalog(
        CAND03 / "models" / "sherlock" / "benchmarks" / "workloads",
    )
    assert gen_ok["case_count"] == 7, f"validate_catalog failed: {gen_ok}"

    with tempfile.TemporaryDirectory() as tmp:
        tdir = Path(tmp)
        fx = Fixture(tdir / "retained")
        for name, value in (("policy", fx.policy), ("samples", fx.samples), ("current", fx.current)):
            (fx.root / f"{name}.json").write_text(json.dumps(value), encoding="utf-8")
        paths = [fx.root / "policy.json", fx.root / "samples.json", fx.root / "current.json", fx.root]
        dw = qa_check.benchmark_completion_report(*paths, repo=CAND03)
        assert dw["complete"] is True, f"benchmark_completion_report failed: {dw}"

        # Probe R02-F02 / REV02-F03: mutating any frozen catalog scalar in oracle effects must fail
        bad_fx = Fixture(tdir / "bad_oracle", stage="full-rollup-series")
        wname = bad_fx.policy["cases"][0]["workload_id"]
        exp_path = bad_fx.root / wname / "expected.json"
        exp_doc = json.loads(exp_path.read_text(encoding="utf-8"))
        exp_doc["transition_states"][1]["fuel"] = 99
        exp_path.write_text(json.dumps(exp_doc), encoding="utf-8")
        new_digest = acceptance.canonical_digest(exp_doc)
        bad_fx.current["cases"][wname]["expected_effects_sha256"] = new_digest
        s0 = bad_fx.samples[0]
        s0["effects"]["expected_sha256"] = new_digest
        s0["effects"]["observed_sha256"] = new_digest
        for role in ("prove", "local_verify", "deployment", "inclusion", "readback"):
            bad_fx.replace_receipt(s0, s0["extensions"]["acceptance"]["receipt_roles"][role], lambda d: d.__setitem__("effects", exp_doc))
        bad_res = bad_fx.assess()
        assert bad_res["complete"] is False
        combined_findings = str(bad_res["findings"]) + str(bad_res["cases"])
        assert "catalog" in combined_findings or "transition" in combined_findings or "pre_state_sha256" in combined_findings or "post_state_sha256" in combined_findings, combined_findings

        # Probe REV02-F02: mutating a node's operation_id without matching node receipt fails
        bad_node_fx = Fixture(tdir / "bad_node")
        bad_node_fx.samples[0]["extensions"]["acceptance"]["proof_nodes"][0]["operation_id"] = "tampered-op"
        bad_node_res = bad_node_fx.assess()
        assert bad_node_res["complete"] is False
        assert any("operation_id" in str(c["findings"]) for c in bad_node_res["cases"])

    results["suite_validation"] = {
        "workload_case_count": gen_ok["case_count"],
        "doctorwatson_complete_on_synthetic_fixture": True,
        "oracle_scalar_tamper_rejected": True,
        "node_operation_id_tamper_rejected": True,
    }

    # 4. Probe R02-F01, R02-F05, REV02-F05 on models.sherlock.benchmarks.metrics
    with tempfile.TemporaryDirectory() as tmp:
        tdir = Path(tmp)
        fx = Fixture(tdir / "metrics_probe", stage="host-compiler-coverage")
        s_base = copy.deepcopy(fx.samples[0])
        s_base["metrics"] = {}
        s_base["receipts"] = []

        raw_ok = b'{"exit_code":0,"result":{"verifies":true},"negative_control":{"verifies":false},"metrics":{"sec":2.0,"other_sec":2.2}}'
        (tdir / "r.json").write_bytes(raw_ok)
        sha_ok = hashlib.sha256(raw_ok).hexdigest()

        binding_base = {
            "path": "r.json",
            "sha256": sha_ok,
            "terminal": {"pointer": "/result/verifies", "success_values": [True], "censored_values": []},
            "fields": [
                {
                    "metric": "prove.wall_seconds",
                    "pointer": "/metrics/sec",
                    "unit": "seconds",
                    "scope": "subprocess",
                    "timer_resolution_seconds": 0.001,
                }
            ],
        }

        s_sec = metrics.collect_sample(copy.deepcopy(s_base), [copy.deepcopy(binding_base)], tdir)
        assert s_sec["status"] == "success"

        s_rescaled = copy.deepcopy(s_sec)
        s_rescaled["sample_id"] = "s1-rescaled"
        s_rescaled["metrics"]["prove.wall_seconds"]["source"]["scale"] = 1000.0

        k_sec = metrics.metric_observation_key(s_sec["metrics"]["prove.wall_seconds"])
        k_rescaled = metrics.metric_observation_key(s_rescaled["metrics"]["prove.wall_seconds"])
        assert k_sec == k_rescaled, "Same receipt+pointer with different scale must share metric_observation_key"
        assert metrics.metric_observation_identity(s_sec["metrics"]["prove.wall_seconds"]) != metrics.metric_observation_identity(s_rescaled["metrics"]["prove.wall_seconds"])

        binding_other = copy.deepcopy(binding_base)
        binding_other["fields"][0]["pointer"] = "/metrics/other_sec"
        s_other_base = copy.deepcopy(s_base)
        s_other_base["sample_id"] = "s2"
        s_other = metrics.collect_sample(s_other_base, [binding_other], tdir)
        k_other = metrics.metric_observation_key(s_other["metrics"]["prove.wall_seconds"])
        assert k_sec != k_other, "Distinct pointers in the same receipt file must have distinct metric_observation_key"

        # Summarize + compare_baseline when a retained duplicate is present alongside 2 unique observations
        raw_b1 = b'{"exit_code":0,"result":{"verifies":true},"metrics":{"sec":1.9}}'
        raw_b2 = b'{"exit_code":0,"result":{"verifies":true},"metrics":{"sec":2.1}}'
        (tdir / "b1.json").write_bytes(raw_b1)
        (tdir / "b2.json").write_bytes(raw_b2)
        b1_bind = copy.deepcopy(binding_base)
        b1_bind.update(path="b1.json", sha256=hashlib.sha256(raw_b1).hexdigest())
        b2_bind = copy.deepcopy(binding_base)
        b2_bind.update(path="b2.json", sha256=hashlib.sha256(raw_b2).hexdigest())
        sb1 = copy.deepcopy(s_base); sb1["sample_id"] = "old-1"
        sb2 = copy.deepcopy(s_base); sb2["sample_id"] = "old-2"
        old1 = metrics.collect_sample(sb1, [b1_bind], tdir)
        old2 = metrics.collect_sample(sb2, [b2_bind], tdir)

        summary_with_dup = metrics.summarize([s_sec, s_rescaled, s_other])
        mstat = summary_with_dup["groups"][0]["metrics"]["prove.wall_seconds"]
        assert (mstat["n"], mstat["duplicate_observations"]) == (2, 1)
        cmp_res = metrics.compare_baseline(
            metrics.summarize([old1, old2]),
            summary_with_dup,
            {"prove.wall_seconds": {"statistic": "median", "relative_increase": 0.2, "absolute_increase": 0.5, "min_samples": 2}},
        )
        assert cmp_res["status"] == "within_declared_tolerance", f"Retained duplicate should not poison compare_baseline when unique n>=min_samples: {cmp_res}"

        # R02-F05: numeric 0 or False rejected in success_values; false at selected pointer marks failure
        bad_num_bind = copy.deepcopy(binding_base)
        bad_num_bind["terminal"] = {"pointer": "/exit_code", "success_values": [0], "censored_values": []}
        try:
            metrics.collect_sample(copy.deepcopy(s_base), [bad_num_bind], tdir)
            raise AssertionError("Expected ValueError for numeric success_values=[0]")
        except ValueError as exc:
            assert "semantic" in str(exc) or "boolean" in str(exc)

        false_ptr_bind = copy.deepcopy(binding_base)
        false_ptr_bind["terminal"] = {"pointer": "/negative_control/verifies", "success_values": [True], "censored_values": []}
        s_fail = metrics.collect_sample(copy.deepcopy(s_base), [false_ptr_bind], tdir)
        assert s_fail["status"] == "failure"

    results["r02_f01_f05_rev02_f05_probes"] = "passed"

    # 5. Run the revision03 unit tests with proper PYTHONPATH
    env = os.environ.copy()
    env["PYTHONPATH"] = ":".join(
        str(p)
        for p in (
            CAND03,
            CAND03 / "models" / "sherlock" / "benchmarks",
            CAND03 / "models" / "sherlock" / "benchmarks" / "workloads",
            CAND03 / "models" / "sherlock" / "qa",
        )
    )
    suite = subprocess.run(
        [
            sys.executable,
            "-m",
            "unittest",
            "models/sherlock/benchmarks/tests/test_metrics.py",
            "models/sherlock/benchmarks/workloads/tests/test_workloads.py",
            "models/sherlock/benchmarks/test_acceptance.py",
            "models/sherlock/qa/test_benchmark_completion.py",
        ],
        cwd=str(CAND03),
        env=env,
        capture_output=True,
        text=True,
        timeout=25,
        check=True,
    )
    results["unittests_stderr_tail"] = suite.stderr.strip().splitlines()[-3:]

    out_file = OUT03 / "probe_results.json"
    out_file.write_text(json.dumps(results, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print("PROBE_OK", json.dumps(results["suite_validation"]))


if __name__ == "__main__":
    main()
