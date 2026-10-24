#!/usr/bin/env python3
"""Independent bounded pure-stdlib verification & adversarial probe script for revision03 (audits03/gemini-1)."""
from __future__ import annotations

import copy
import hashlib
import json
import sys
import tempfile
from pathlib import Path

BASE = Path("/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/sherlock-moriarty-benchmark-suite-20261021")
INT03 = BASE / "integration03"
CAND03 = INT03 / "candidate"
INT02 = BASE / "integration02"
CAND02 = INT02 / "candidate"

sys.path.insert(0, str(CAND03))
sys.path.insert(0, str(CAND03 / "models/sherlock/benchmarks"))

import acceptance  # noqa: E402
import metrics  # noqa: E402
import test_acceptance  # noqa: E402


def sha256_file(path: Path) -> tuple[int, str]:
    raw = path.read_bytes()
    return len(raw), hashlib.sha256(raw).hexdigest()


def verify_candidate_and_receipts() -> dict:
    cmap = json.loads((INT03 / "CANDIDATE-MAP.json").read_text())
    source_files = cmap["sourceFiles"]
    assert len(source_files) == 45, f"Expected 45 files, got {len(source_files)}"

    changed_count = 0
    unchanged_count = 0
    mismatches = []
    changed_list = []

    for entry in source_files:
        rel = entry["target"]
        p03 = CAND03 / rel
        size03, digest03 = sha256_file(p03)
        if size03 != entry["bytes"] or digest03 != entry["sha256"]:
            mismatches.append({"target": rel, "expected": entry, "actual_bytes": size03, "actual_sha256": digest03})
        if entry["revision03"] is not None:
            changed_count += 1
            src_after = Path(entry["revision03"]["source"])
            s_size, s_digest = sha256_file(src_after)
            if s_size != size03 or s_digest != digest03:
                mismatches.append({"target": rel, "afterimage_mismatch": True})
            if digest03 == entry["revision02SHA256"]:
                mismatches.append({"target": rel, "expected_change_but_same_as_rev02": True})
            changed_list.append({
                "target": rel,
                "lane": entry["revision03"]["lane"],
                "bytes": size03,
                "sha256": digest03,
                "revision02SHA256": entry["revision02SHA256"],
            })
        else:
            unchanged_count += 1
            if digest03 != entry["revision02SHA256"]:
                mismatches.append({"target": rel, "expected_unchanged_but_differs": True})

    # Verify traceability.json pins (44 files, excluding traceability.json itself)
    trace = json.loads((CAND03 / "openspec/changes/sherlock-moriarty-benchmarks/traceability.json").read_text())
    trace_pins = trace["sourcePins"]
    assert len(trace_pins) == 44, f"Expected 44 traceability pins, got {len(trace_pins)}"
    for item in trace_pins:
        p = CAND03 / item["path"]
        sz, dg = sha256_file(p)
        if dg != item["sha256"]:
            mismatches.append({"traceability_mismatch": item["path"], "expected": item, "actual": (sz, dg)})

    # Verify root-actual-tests01/RESULT.json and 6 raw streams
    res_path = INT03 / "root-actual-tests01/RESULT.json"
    res_bytes, res_sha = sha256_file(res_path)
    res_doc = json.loads(res_path.read_text())
    stream_summaries = []
    total_ran = 0
    for cmd in res_doc["results"]:
        stderr_p = Path(cmd["stderr"]["path"])
        stdout_p = Path(cmd["stdout"]["path"])
        se_bytes, se_sha = sha256_file(stderr_p)
        so_bytes, so_sha = sha256_file(stdout_p)
        if (
            se_sha != cmd["stderr"]["sha256"]
            or se_bytes != cmd["stderr"]["bytes"]
            or so_sha != cmd["stdout"]["sha256"]
            or so_bytes != cmd["stdout"]["bytes"]
        ):
            mismatches.append({"stream_mismatch": cmd["id"]})
        total_ran += cmd["actualTests"]
        stream_summaries.append({
            "id": cmd["id"],
            "ran": cmd["actualTests"],
            "returncode": cmd["exit"],
            "stderr_bytes": se_bytes,
            "stderr_sha256": se_sha,
            "stdout_bytes": so_bytes,
            "stdout_sha256": so_sha,
        })

    # Compute exact digests for Lane A and Lane C artifacts
    lane_artifacts = {}
    for rel in [
        "revision03/sol-metrics/REPORT.md",
        "revision03/sol-metrics/FILE-MAP.json",
        "revision03/sol-metrics/INTERFACE-DELTA.json",
        "revision03/sol-loop/REPORT.md",
        "revision03/sol-loop/AUDIT-RESOLUTION.json",
        "revision03/sol-loop/FILE-MAP.json",
        "integration03/CANDIDATE-MAP.json",
        "integration03/root-actual-tests01/RESULT.json",
    ]:
        sz, dg = sha256_file(BASE / rel)
        lane_artifacts[rel] = {"bytes": sz, "sha256": dg}

    return {
        "total_candidate_files": len(source_files),
        "changed_vs_revision02": changed_count,
        "unchanged_vs_revision02": unchanged_count,
        "mismatches": mismatches,
        "changed_files": changed_list,
        "root_tests_total_ran": total_ran,
        "root_test_streams": stream_summaries,
        "lane_and_root_artifacts": lane_artifacts,
    }


def run_adversarial_probes() -> dict:
    results = {}
    H = lambda raw: hashlib.sha256(raw).hexdigest()

    # Probe 1: R02-F01 / REV02-F01 (Token-duplicate leak & per-profile repetition minima)
    with tempfile.TemporaryDirectory() as tmpdir:
        root = Path(tmpdir)
        f = test_acceptance.Fixture(root / "cur")
        prior = test_acceptance.Fixture(f.root / "prior", label="prior", compiler="old-compiler")
        name = f.samples[0]["workload_id"]
        metric = "compile.frontend.wall_seconds"
        sample = f.samples[0]
        ext = sample["extensions"]["acceptance"]
        f.replace_receipt(sample, ext["observation"], lambda d: d["metrics"].update({metric: 100}))
        sample["metrics"][metric]["value"] = 100
        for i in range(2):
            dup = copy.deepcopy(sample)
            dup["sample_id"] = f"alias{i}"
            de = dup["extensions"]["acceptance"]
            de["observation_id"] = f"alias{i}"
            ref = next(r for r in dup["receipts"] if r["sha256"] == de["observation"]["receipt_sha256"])
            body = json.loads((f.root / ref["path"]).read_text())
            body.update(observation_id=de["observation_id"])
            body["metrics"][metric] = 1
            de["observation"] = f.add(dup, f"alias{i}.json", body)
            for entry in dup["metrics"].values():
                entry["source"]["receipt_sha256"] = de["observation"]["receipt_sha256"]
            dup["metrics"][metric]["value"] = 1
            f.samples.append(dup)
        f.policy["regression"] = {
            "mode": "compare",
            "allowed_changed_subject_fields": ["compiler_sha256"],
            "tolerances": {name: {metric: {"statistic": "median", "relative_increase": 0, "absolute_increase": 0, "min_samples": 1}}},
        }
        baseline = dict(schema="sherlock-benchmark-baseline/2", root_prefix="prior", samples=prior.samples, policy=prior.policy, current=prior.current)
        rep_dup = f.assess(baseline)

        # Split profile minimum check
        f2 = test_acceptance.Fixture(root / "cur2")
        prior2 = test_acceptance.Fixture(f2.root / "prior", label="prior", compiler="old-compiler")
        for fx in (f2, prior2):
            second = test_acceptance.Fixture(fx.root / "second", label=fx.label + "-second", compiler="compiler" if fx is f2 else "old-compiler")
            s2 = second.samples[0]
            e2 = s2["extensions"]["acceptance"]
            s2["cache_state"] = "warm"
            s2["cache_identity_sha256"] = H(b"cache")
            second.replace_receipt(s2, e2["observation"], lambda d: d.update(cache_state="warm", cache_identity_sha256=H(b"cache")))
            fx.policy["observation_fields"]["cache_identity_sha256"] = "/cache_identity_sha256"
            for ref in s2["receipts"]:
                ref["path"] = "second/" + ref["path"]
            fx.samples.append(s2)
            fx.policy["cases"][0]["min_successful_samples"] = 2
        f2.policy["regression"] = {
            "mode": "compare",
            "allowed_changed_subject_fields": ["compiler_sha256"],
            "tolerances": {name: {"layer:decider:prove.wall_seconds": {"statistic": "median", "relative_increase": 0, "absolute_increase": 0, "min_samples": 1}}},
        }
        baseline2 = dict(schema="sherlock-benchmark-baseline/2", root_prefix="prior", samples=prior2.samples, policy=prior2.policy, current=prior2.current)
        rep_split = f2.assess(baseline2)

        results["probe_1_r02_f01_duplicate_and_profile_minima"] = {
            "token_duplicates_excluded_from_valid_and_comparison": (
                rep_dup["complete"] is False
                and rep_dup["cases"][0]["counts"]["success"] == 1
                and rep_dup["cases"][0]["duplicate_observations"] == 2
            ),
            "split_profile_below_case_minimum_rejected": (
                rep_split["complete"] is False
                and "per-profile" in str(rep_split["findings"])
            ),
        }

    # Probe 2: R02-F02 / REV02-F02 (Prover/verifier role aliasing & command execution independence)
    with tempfile.TemporaryDirectory() as tmpdir:
        root = Path(tmpdir)
        # Positive: shared file with distinct JSON pointers /prove and /verify
        f_shared = test_acceptance.Fixture(root / "shared")
        sample = f_shared.samples[0]
        node0 = sample["extensions"]["acceptance"]["proof_nodes"][0]
        p_ref = next(r for r in sample["receipts"] if r["sha256"] == node0["receipt_roles"]["prove"]["receipt_sha256"])
        v_ref = next(r for r in sample["receipts"] if r["sha256"] == node0["receipt_roles"]["local_verify"]["receipt_sha256"])
        p_doc = json.loads((f_shared.root / p_ref["path"]).read_text())
        v_doc = json.loads((f_shared.root / v_ref["path"]).read_text())
        combined = f_shared.add(sample, "combined-child0.json", dict(p_doc, prove=p_doc, verify=v_doc))
        node0["receipt_roles"]["prove"] = dict(receipt_sha256=combined["receipt_sha256"], pointer="/prove")
        node0["receipt_roles"]["local_verify"] = dict(receipt_sha256=combined["receipt_sha256"], pointer="/verify")
        for m_name, m_entry in sample["layers"]["child-0"].items():
            role_ptr = "/verify" if m_name.startswith("local_verify.") else "/prove"
            m_entry["source"]["receipt_sha256"] = combined["receipt_sha256"]
            m_entry["source"]["pointer"] = role_ptr + m_entry["source"]["pointer"]
        rep_shared = f_shared.assess()

        # Negative A: aliased pointer
        f_alias = test_acceptance.Fixture(root / "alias")
        s_alias = f_alias.samples[0]
        n0_a = s_alias["extensions"]["acceptance"]["proof_nodes"][0]
        n0_a["receipt_roles"]["local_verify"] = copy.deepcopy(n0_a["receipt_roles"]["prove"])
        f_alias.policy["command_role_values"]["local_verify"] = ["prove", "verify"]
        rep_alias = f_alias.assess()

        # Negative B: reused execution_id
        f_exec = test_acceptance.Fixture(root / "execid")
        s_exec = f_exec.samples[0]
        n0_e = s_exec["extensions"]["acceptance"]["proof_nodes"][0]
        f_exec.replace_receipt(s_exec, n0_e["receipt_roles"]["local_verify"], lambda d: d.update(execution_id=p_doc["execution_id"]))
        rep_exec = f_exec.assess()

        results["probe_2_r02_f02_command_independence"] = {
            "shared_file_distinct_pointers_accepted": rep_shared["complete"] is True,
            "aliased_pointer_or_overlapping_roles_rejected": rep_alias["complete"] is False,
            "reused_execution_id_rejected": rep_exec["complete"] is False,
        }

    # Probe 3: R02-F03 / REV02-F02 (Multi-node layer aggregation & witness/keygen/proving_key/artifact attribution)
    with tempfile.TemporaryDirectory() as tmpdir:
        root = Path(tmpdir)
        f_full = test_acceptance.Fixture(root / "full", "full-rollup-series")
        s_fenrir2 = next(s for s in f_full.samples if s["workload_id"] == "fenrir-transfer-batches-2")
        # Tamper proving_key.bytes on multi-node leaf layer
        s_fenrir2_bad = copy.deepcopy(s_fenrir2)
        s_fenrir2_bad["layers"]["leaf"]["proving_key.bytes"]["value"] += 1
        rep_pk_bad = acceptance.assess([s_fenrir2_bad if s["workload_id"] == "fenrir-transfer-batches-2" else s for s in f_full.samples], f_full.policy, f_full.current, f_full.root)

        # Unattributed witness/keygen/proving_key/artifact on decider layer
        f_attr = test_acceptance.Fixture(root / "attr")
        s_attr = f_attr.samples[0]
        ext_attr = s_attr["extensions"]["acceptance"]
        unattributed_rejected = True
        for m_name in ("witness.wall_seconds", "keygen.cpu_seconds", "proving_key.bytes", "artifact.bytes"):
            other = copy.deepcopy(s_attr)
            other["layers"]["decider"][m_name]["source"].update(receipt_sha256=ext_attr["proof_nodes"][0]["receipt_roles"]["prove"]["receipt_sha256"])
            if acceptance.assess([other], f_attr.policy, f_attr.current, f_attr.root)["complete"]:
                unattributed_rejected = False

        results["probe_3_r02_f03_layer_aggregation_and_attribution"] = {
            "tampered_multinode_proving_key_bytes_rejected": rep_pk_bad["complete"] is False,
            "cross_layer_unattributed_witness_keygen_pk_artifact_rejected": unattributed_rejected,
        }

    # Probe 4: R02-F04 (Compare mode tolerance scope & full metric coverage)
    with tempfile.TemporaryDirectory() as tmpdir:
        root = Path(tmpdir)
        f = test_acceptance.Fixture(root / "cur")
        prior = test_acceptance.Fixture(f.root / "prior", label="prior", compiler="old-compiler")
        name = f.samples[0]["workload_id"]
        metric = "layer:decider:prove.wall_seconds"
        f.policy["regression"] = {
            "mode": "compare",
            "allowed_changed_subject_fields": ["compiler_sha256"],
            "tolerances": {name: {metric: {"statistic": "median", "relative_increase": 0, "absolute_increase": 0, "min_samples": 1}}},
        }
        baseline = dict(schema="sherlock-benchmark-baseline/2", root_prefix="prior", samples=prior.samples, policy=prior.policy, current=prior.current)
        rep = f.assess(baseline)
        cscope = rep["comparison_scope"][0]
        results["probe_4_r02_f04_comparison_scope"] = {
            "complete": rep["complete"] is True,
            "performance_target_satisfied": rep["performance_target_satisfied"] is False,
            "full_goal_complete": rep["full_goal_complete"] is False,
            "required_measurements_count": len(cscope["required_measurements"]),
            "compared_metrics": cscope["compared_metrics"],
            "measured_without_tolerance_count": len(cscope["measured_without_tolerance"]),
        }

    # Probe 5: R02-F05 / REV02-F05 (collect_sample exit-0 rejection, nested verifier verdict, receipt+pointer key)
    with tempfile.TemporaryDirectory() as tmpdir:
        root = Path(tmpdir)
        raw_ok = json.dumps({
            "exit_code": 0,
            "result": {"verifies": False},
            "negative_control": {"verifies": False},
            "wall_seconds": 1.25,
        }).encode()
        p_rec = root / "rec.json"
        p_rec.write_bytes(raw_ok)
        sha_rec = H(raw_ok)
        base_spec = {
            "schema": "sherlock-benchmark-sample/1",
            "sample_id": "s-probe",
            "workload_id": "synthetic-test",
            "workload_digest_sha256": "a" * 64,
            "evidence_class": "synthetic",
            "status": "success",
            "identity": {
                "source_sha256": "a" * 64,
                "compiler_sha256": None,
                "ledger_sha256": None,
                "network": None,
                "circuit_sha256": None,
                "profile": "test/1",
                "environment_sha256": "a" * 64,
                "toolchain_sha256": "a" * 64,
            },
            "cache_state": "not_applicable",
            "metrics": {},
            "layers": {},
            "receipts": [],
            "effects": {"expected_sha256": None, "observed_sha256": None, "match": None},
            "events": {},
        }
        bindings_exit0 = [{
            "path": "rec.json",
            "sha256": sha_rec,
            "terminal": {"pointer": "/exit_code", "success_values": [0], "censored_values": []},
            "fields": [{"metric": "prove.wall_seconds", "pointer": "/wall_seconds", "unit": "seconds", "scope": "subprocess", "timer_resolution_seconds": 0.001}],
        }]
        exit0_rejected = False
        try:
            metrics.collect_sample(base_spec, bindings_exit0, root)
        except ValueError:
            exit0_rejected = True

        # Nested verifier /result/verifies = False marks sample failure
        bindings_nested_fail = copy.deepcopy(bindings_exit0)
        bindings_nested_fail[0]["terminal"] = {"pointer": "/result/verifies", "success_values": [True], "censored_values": []}
        sample_nested_fail = metrics.collect_sample(base_spec, bindings_nested_fail, root)

        # Nested verifier /result/verifies = True with /negative_control/verifies = False succeeds
        raw_pass = json.dumps({
            "exit_code": 0,
            "result": {"verifies": True},
            "negative_control": {"verifies": False},
            "wall_seconds": 1.25,
        }).encode()
        (root / "rec_pass.json").write_bytes(raw_pass)
        bindings_neg_ctrl = copy.deepcopy(bindings_exit0)
        bindings_neg_ctrl[0]["path"] = "rec_pass.json"
        bindings_neg_ctrl[0]["sha256"] = H(raw_pass)
        bindings_neg_ctrl[0]["terminal"] = {"pointer": "/result/verifies", "success_values": [True], "censored_values": []}
        sample_neg_ctrl = metrics.collect_sample(base_spec, bindings_neg_ctrl, root)

        results["probe_5_r02_f05_collect_and_identity"] = {
            "exit_code_zero_success_predicate_rejected": exit0_rejected,
            "nested_verifier_false_marks_sample_failure": sample_nested_fail["status"] == "failure",
            "unrelated_negative_control_false_permitted_when_selected_verdict_true": sample_neg_ctrl["status"] == "success",
        }

    # Probe 6: REV02-F03 (Catalog scalar correspondence, transition state views, distinct root pointers)
    with tempfile.TemporaryDirectory() as tmpdir:
        root = Path(tmpdir)
        f_full = test_acceptance.Fixture(root / "full", "full-rollup-series")
        # Tamper catalog scalar in expected.json of fenrir-transfer-batches-1
        ctx_f1 = f_full.current["cases"]["fenrir-transfer-batches-1"]
        exp_path = f_full.root / ctx_f1["expected_effects_pin"]["path"]
        exp_doc = json.loads(exp_path.read_text())
        first_scalar_key = next(iter(exp_doc["catalog_scalars"]))
        exp_doc["catalog_scalars"][first_scalar_key] += 1
        exp_path.write_bytes(json.dumps(exp_doc).encode())
        ctx_f1["expected_effects_sha256"] = acceptance.canonical_digest(exp_doc)
        rep_scalar_tamper = f_full.assess()

        # Overlapping root pointers rejected, equal root values at distinct pointers accepted
        f_roots = test_acceptance.Fixture(root / "roots")
        ctx_r = next(iter(f_roots.current["cases"].values()))
        ctx_r["expected_effect_root_pointers"]["ledger_state_root"] = ctx_r["expected_effect_root_pointers"]["post_state_root"]
        rep_root_alias = f_roots.assess()

        results["probe_6_rev02_f03_oracle_and_roots"] = {
            "tampered_catalog_scalar_rejected": rep_scalar_tamper["complete"] is False,
            "aliased_root_pointers_rejected": rep_root_alias["complete"] is False,
        }

    # Probe 7: Check single-member layer_observations behavior on len(members) == 1
    with tempfile.TemporaryDirectory() as tmpdir:
        root = Path(tmpdir)
        f_single = test_acceptance.Fixture(root / "single")
        s_single = f_single.samples[0]
        ext = s_single["extensions"]["acceptance"]
        decider_node = next(n for n in ext["proof_nodes"] if n["layer"] == "decider")
        # Try attaching a layer_observations["decider"] on a 1-member layer without _layer_aggregation
        body = dict(
            s_single["identity"],
            workload_digest_sha256=s_single["workload_digest_sha256"],
            layer="decider",
            nodes=[{k: v for k, v in decider_node.items() if k in {"node_id", "layer", "circuit_sha256", "proof_sha256", "statement_sha256", "verifying_key_sha256", "pre_state_sha256", "post_state_sha256"}}],
            success=True,
            aggregation={"local_verify.wall_seconds": {"mode": "observed_parallel_span", "scope": "all-member-command-span"}},
            spoofed_wall=999,
        )
        binding = f_single.add(s_single, "decider-layer.json", body)
        ext["layer_observations"] = {"decider": binding}
        f_single.policy["layer_receipt_fields"] = {key: "/" + key for key in body if key != "spoofed_wall"}
        f_single.policy["terminal_values"]["layer_observation"] = [True]
        s_single["layers"]["decider"]["local_verify.wall_seconds"]["value"] = 999
        s_single["layers"]["decider"]["local_verify.wall_seconds"]["source"]["receipt_sha256"] = binding["receipt_sha256"]
        s_single["layers"]["decider"]["local_verify.wall_seconds"]["source"]["pointer"] = "/spoofed_wall"
        rep_single = f_single.assess()
        results["probe_7_single_member_layer_observation_behavior"] = {
            "complete_when_single_member_layer_has_layer_observations": rep_single["complete"],
        }

    return results


if __name__ == "__main__":
    summary = {
        "verification": verify_candidate_and_receipts(),
        "probes": run_adversarial_probes(),
    }
    out_path = Path(__file__).parent / "probe-results.json"
    out_path.write_text(json.dumps(summary, indent=2) + "\n")
    print(json.dumps({
        "mismatches": summary["verification"]["mismatches"],
        "changed_vs_revision02": summary["verification"]["changed_vs_revision02"],
        "unchanged_vs_revision02": summary["verification"]["unchanged_vs_revision02"],
        "root_tests_total_ran": summary["verification"]["root_tests_total_ran"],
        "probes": summary["probes"],
    }, indent=2))
