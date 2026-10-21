"""Pure parser fixtures only: labels never establish an actual stagenet run."""
import copy
import hashlib
import json
from pathlib import Path
import tempfile
import unittest

from acceptance import assess, canonical_digest

H = hashlib.sha256(b"fixture source").hexdigest()
I = {"source_sha256": H, "compiler_sha256": H, "ledger_sha256": H,
     "network": "stagenet-fixture", "circuit_sha256": H, "profile": "fixture/1"}
LAYERS = ["child-0", "child-1", "parent", "decider"]

class AcceptanceTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        effects = {"ordered_effects": [{"credit": 3}], "preserved": {"reserve": 4},
                   "fees_dust": 2, "residual_duties": ["owed"]}
        self.current = {"schema": "sherlock-benchmark-current/1", "identity": I,
                        "target_id": "target-fixture", "anchor_id": "anchor-fixture",
                        "identity_pins": {key: {"path": "identity.bin", "algorithm": "sha256"} for key in I if key.endswith("sha256")},
                        "cases": {"sherlock-linked-two-step": {
                            "workload_digest_sha256": H,
                            "expected_effects_sha256": canonical_digest(effects),
                            "expected_effects_independence": "independent", "expected_effects_scope": "complete-deployed-effects",
                            "workload_pin": {"path": "workload.bin", "algorithm": "sha256"},
                            "expected_effects_pin": {"path": "effects.json", "algorithm": "canonical-json-sha256"}}}}
        self.case = {"workload_id": "sherlock-linked-two-step", "kind": "sherlock-recursive-interpreter",
                     "workload_digest_sha256": H, "required_metrics": ["prove.wall_seconds"],
                     "required_layers": LAYERS, "min_successful_samples": 1,
                     "max_failures": 0, "max_censored": 0}
        self.policy = {"schema": "sherlock-benchmark-acceptance-policy/1", "cases": [self.case],
                       "receipt_fields": {}, "limits": {}, "profile": "fixture/1", "network": "stagenet-fixture",
                       "regression": {"mode": "initial_baseline", "future_tolerances": "must be explicitly selected before comparison"}}
        ext = dict(target_id="target-fixture", anchor_id="anchor-fixture", deployment_id="deploy-fixture",
                   call_transaction_id="call-fixture", proof_sha256=hashlib.sha256(b"fixture proof").hexdigest(),
                   statement_sha256=H, receipt_roles={})
        self.sample = {"schema": "sherlock-benchmark-sample/1", "sample_id": "fixture-1",
                       "workload_id": "sherlock-linked-two-step", "workload_digest_sha256": H,
                       "evidence_class": "stagenet", "status": "success", "identity": I,
                       "cache_state": "cold", "metrics": {}, "layers": {}, "receipts": [],
                       "effects": {"expected_sha256": canonical_digest(effects),
                                   "observed_sha256": canonical_digest(effects), "match": True},
                       "events": {k: f"2026-10-21T00:00:0{n}Z" for n,k in enumerate(
                           ["proof_ready", "submission", "ack", "inclusion", "readback"])},
                       "extensions": {"acceptance": ext}}
        (self.root/"identity.bin").write_bytes(b"fixture source")
        (self.root/"workload.bin").write_bytes(b"fixture source")
        (self.root/"effects.json").write_text(json.dumps(effects))
        self.put("proof.bin", b"fixture proof")
        ext["proof_artifact_receipt"] = self.sample["receipts"][-1]["sha256"]
        facts = dict(I, workload_digest_sha256=H, target_id=ext["target_id"], anchor_id=ext["anchor_id"],
                     deployment_id=ext["deployment_id"], call_transaction_id=ext["call_transaction_id"],
                     proof_sha256=ext["proof_sha256"], statement_sha256=H,
                     success=True, effects=effects, seconds=2, layer_seconds={layer: 2 for layer in LAYERS})
        facts.update(self.sample["events"])
        for role in ["prove", "local_verify", "deployment", "inclusion", "readback"]:
            self.put(role + ".json", json.dumps(facts).encode())
            ext["receipt_roles"][role] = self.sample["receipts"][-1]["sha256"]
            self.policy["receipt_fields"][role] = {key: "/"+key for key in facts if key != "seconds"}
        sha = ext["receipt_roles"]["prove"]
        m = {"value": 2, "unit": "seconds", "status": "measured",
             "source": {"receipt_sha256": sha, "pointer": "/seconds"}, "reason": None}
        self.sample["metrics"]["prove.wall_seconds"] = m
        self.sample["layers"] = {layer: {"prove.wall_seconds": dict(copy.deepcopy(m),source={"receipt_sha256":sha,"pointer":"/layer_seconds/"+layer})} for layer in LAYERS}

    def put(self, name, raw):
        (self.root/name).write_bytes(raw)
        self.sample["receipts"].append({"path": name,"sha256": hashlib.sha256(raw).hexdigest(),"schema": None})

    def result(self, samples=None, baseline=None):
        return assess(samples or [self.sample], self.policy, self.current, self.root, baseline or [])

    def test_complete_fixture_requires_every_receipt_and_layer(self):
        self.assertTrue(self.result()["complete"])
        self.assertFalse(self.result()["performance_target_satisfied"])
        self.assertIn("owner", self.result()["scope"])

    def test_current_artifact_edit_invalidates_before_adoption(self):
        (self.root/"identity.bin").write_bytes(b"changed source")
        self.assertFalse(self.result()["complete"])

    def test_layer_cost_cannot_reuse_whole_run_timer(self):
        self.sample["layers"]["decider"]["prove.wall_seconds"]["source"] = copy.deepcopy(self.sample["metrics"]["prove.wall_seconds"]["source"])
        self.assertFalse(self.result()["complete"])

    def test_wrong_limit_profile_cannot_qualify(self):
        self.policy["profile"] = "fenrir-unrelated/1"
        self.assertFalse(self.result()["complete"])

    def test_host_and_model_cannot_close_stagenet(self):
        for label in ["host", "model", "native", "synthetic"]:
            self.sample["evidence_class"] = label
            self.assertFalse(self.result()["complete"])

    def test_current_source_or_target_drift_invalidates(self):
        self.current["identity"] = dict(I, source_sha256="b"*64)
        self.assertFalse(self.result()["complete"])
        self.current["identity"] = I
        self.current["anchor_id"] = "changed"
        self.assertFalse(self.result()["complete"])

    def test_timestamp_labels_must_match_observed_receipts(self):
        self.sample["events"]["readback"]="2026-10-21T00:00:09Z"
        self.assertFalse(self.result()["complete"])

    def test_receipt_edit_invalidates_even_with_unchanged_sample(self):
        (self.root/"readback.json").write_text("{}")
        self.assertFalse(self.result()["complete"])

    def test_fabricated_metric_value_or_unit_cannot_pass(self):
        self.sample["metrics"]["prove.wall_seconds"]["value"] = 0
        self.assertFalse(self.result()["complete"])

    def test_missing_layer_and_unavailable_required_metric_incomplete(self):
        self.sample["layers"].pop("decider")
        self.assertFalse(self.result()["complete"])
        self.sample["metrics"]["prove.wall_seconds"].update(value=None,status="unavailable",source=None,reason="not exposed")
        self.assertFalse(self.result()["complete"])

    def test_entire_effect_including_residual_duty_is_required(self):
        self.current["cases"]["sherlock-linked-two-step"]["expected_effects_sha256"] = H
        self.assertFalse(self.result()["complete"])

    def test_correlated_expected_effects_cannot_qualify(self):
        self.current["cases"]["sherlock-linked-two-step"]["expected_effects_independence"] = "correlated"
        self.assertFalse(self.result()["complete"])

    def test_proof_artifact_must_exist_and_match(self):
        (self.root/"proof.bin").write_bytes(b"")
        self.assertFalse(self.result()["complete"])

    def test_repeats_cannot_reuse_same_transaction(self):
        self.case["min_successful_samples"] = 2
        other = copy.deepcopy(self.sample); other["sample_id"] = "fixture-2"
        self.assertFalse(self.result([self.sample,other])["complete"])

    def test_failure_and_censored_counts_remain_and_block_by_declared_policy(self):
        for status in ["failure", "censored"]:
            other=copy.deepcopy(self.sample); other.update(sample_id=status,status=status)
            result=self.result([self.sample,other])
            self.assertFalse(result["complete"])
            self.assertEqual(result["cases"][0]["counts"][status],1)

    def test_exit_zero_with_false_verifier_verdict_cannot_pass(self):
        path=self.root/"local_verify.json"
        receipt=json.loads(path.read_text()); receipt["verifies"]=False
        raw=json.dumps(receipt).encode(); path.write_bytes(raw)
        sha=hashlib.sha256(raw).hexdigest()
        for ref in self.sample["receipts"]:
            if ref["path"]=="local_verify.json": ref["sha256"]=sha
        self.sample["extensions"]["acceptance"]["receipt_roles"]["local_verify"]=sha
        self.policy["receipt_fields"]["local_verify"]["success"]="/verifies"
        self.assertFalse(self.result()["complete"])

    def test_repeated_prove_receipt_cannot_supply_new_observation(self):
        self.case["min_successful_samples"]=2
        other=copy.deepcopy(self.sample);other["sample_id"]="second"
        other["extensions"]["acceptance"]["call_transaction_id"]="another-call"
        result=self.result([self.sample,other])
        self.assertFalse(result["complete"])
        self.assertTrue(any("reus" in f["reason"] for f in result["cases"][0]["findings"]))

    def test_initial_baseline_needs_explicit_future_policy(self):
        self.policy["regression"].pop("future_tolerances")
        self.assertFalse(self.result()["complete"])

    def prior_baseline(self):
        baseline=copy.deepcopy(self.sample)
        baseline["sample_id"]="prior-fixture"
        baseline["extensions"]["acceptance"]["call_transaction_id"]="prior-call"
        mapping={}
        for ref in baseline["receipts"]:
            if not ref["path"].endswith(".json"): continue
            value=json.loads((self.root/ref["path"]).read_text())
            value["call_transaction_id"]="prior-call";value["observation_id"]="prior"
            raw=json.dumps(value).encode();old=ref["sha256"];new=hashlib.sha256(raw).hexdigest()
            ref["path"]="prior-"+ref["path"];ref["sha256"]=new
            (self.root/ref["path"]).write_bytes(raw);mapping[old]=new
        for role,sha in baseline["extensions"]["acceptance"]["receipt_roles"].items():
            baseline["extensions"]["acceptance"]["receipt_roles"][role]=mapping[sha]
        for metrics in [baseline["metrics"],*baseline["layers"].values()]:
            for metric in metrics.values():
                source=metric.get("source")
                if source:source["receipt_sha256"]=mapping.get(source["receipt_sha256"],source["receipt_sha256"])
        return baseline

    def test_compare_requires_comparable_baseline_and_explicit_tolerances(self):
        self.policy["regression"]={"mode":"compare","tolerances":{"prove.wall_seconds":{"max_ratio":1.1}}}
        self.assertFalse(self.result()["complete"])
        self.assertFalse(self.result(baseline=[copy.deepcopy(self.sample)])["complete"])
        self.assertTrue(self.result(baseline=[self.prior_baseline()])["complete"])
        baseline=copy.deepcopy(self.sample); baseline["identity"]=dict(I,network="other")
        self.assertFalse(self.result(baseline=[baseline])["complete"])

    def test_declared_limit_is_applied_to_whole_and_each_layer(self):
        self.policy["limits"]={"prove.wall_seconds":{"max":1,"unit":"seconds"}}
        self.assertFalse(self.result()["complete"])

    def test_empty_required_cases_cannot_succeed(self):
        self.policy["cases"]=[]
        self.assertFalse(self.result()["complete"])

    def test_missing_role_semantic_binding_cannot_succeed(self):
        self.policy["receipt_fields"]["inclusion"].pop("proof_sha256")
        self.assertFalse(self.result()["complete"])

if __name__ == "__main__":
    unittest.main()
