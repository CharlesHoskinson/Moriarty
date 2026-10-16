"""Normal offline data tests; no model, compiler, SDK, network or native calls.

Literal comparison expectations do not import an evaluator. Historical artifact
and readback files are parser fixtures only, not independently derived semantic
oracles and not current execution evidence. Native/hostile tests belong to Opus.
"""

import hashlib
import json
from pathlib import Path
import unittest

import offline


HERE = Path(__file__).resolve().parent
REPO = HERE.parents[2]
FIXTURES = HERE / "fixtures"
BACKEND = REPO / "crates/mori-backend/tests/support/task4-native"
HISTORICAL = REPO / "deliverables/native-call-relation-2026-10-12/public-ordinary-checks-02/payment/prepared/readback.json"
PURE_SHA256 = "6370ced2898896a914ee8191c44f2937b98bb9f232cb7c15a5a8569aa9cb0540"
HISTORICAL_SHA256 = "e167ada5e15d0e30201d35d874aaf5577718c21cf3a611b757b56d0583e31ea4"


def literal(name):
    return json.loads((FIXTURES / name).read_text())


def payment_inventory(target=None, **kwargs):
    return offline.prepare(
        artifact_path=BACKEND / "independent-payment.artifact.json",
        target_path=target or BACKEND / "independent-payment.target.json",
        scenario_path=FIXTURES / "payment-scenario.json",
        source_path=BACKEND / "independent-payment.mori",
        expected_path=HISTORICAL,
        expected_sha256=HISTORICAL_SHA256,
        action="settle", application_scope="financial", **kwargs)


class NormalReadbackComparison(unittest.TestCase):
    def test_two_literal_normal_outputs_have_the_frozen_first_difference(self):
        result = offline.compare(
            expected_path=FIXTURES / "pure-expected.json", expected_sha256=PURE_SHA256,
            observed_path=FIXTURES / "pure-observed-eight.json")
        self.assertEqual(result["first_difference"], literal("pure-first-difference.json"))
        self.assertEqual(result["scope"], "supplied-json-comparison-only")

    def test_dictionary_order_is_not_execution_order(self):
        left = {"name": "answer", "value": {"type": "UInt", "value": "7"}}
        right = {"value": {"value": "7", "type": "UInt"}, "name": "answer"}
        self.assertIsNone(offline.first_difference(left, right))

    def test_ordered_effects_are_not_sorted_or_netted(self):
        left = [{"kind": "Transfer", "amount": "10"}, {"kind": "Fee", "amount": "2"}]
        right = [{"kind": "Fee", "amount": "2"}, {"kind": "Transfer", "amount": "10"}]
        self.assertEqual(offline.first_difference(left, right), {
            "path": "/0/amount", "kind": "value", "expected": "10", "observed": "2"})

    def test_same_terminal_value_retains_prefix_difference(self):
        left = {"steps": [{"snapshot": {"count": "3"}}, {"snapshot": {"count": "7"}}], "post": "7"}
        right = {"steps": [{"snapshot": {"count": "4"}}, {"snapshot": {"count": "7"}}], "post": "7"}
        self.assertEqual(offline.first_difference(left, right), {
            "path": "/steps/0/snapshot/count", "kind": "value", "expected": "3", "observed": "4"})

    def test_extension_accounting_is_compared_without_projection(self):
        left = {"gross_debit": "12", "fees": "2", "net_receipts": "8", "residual_duties": ["5"]}
        right = {"gross_debit": "12", "fees": "2", "net_receipts": "8", "residual_duties": ["4"]}
        self.assertEqual(offline.first_difference(left, right), {
            "path": "/residual_duties/0", "kind": "value", "expected": "5", "observed": "4"})

    def test_preserved_state_is_compared(self):
        left = {"state": {"affected": "7", "preserved": True}}
        right = {"state": {"affected": "7", "preserved": False}}
        self.assertEqual(offline.first_difference(left, right), {
            "path": "/state/preserved", "kind": "value", "expected": True, "observed": False})

    def test_present_zero_and_absent_duty_have_distinct_reports(self):
        self.assertEqual(offline.first_difference({"zero": "0"}, {}), {
            "path": "/zero", "kind": "member", "expected": "0", "observed": None,
            "expected_present": True, "observed_present": False})

    def test_extra_observation_is_not_discarded(self):
        self.assertEqual(offline.first_difference({"fee": "2"}, {"fee": "2", "refund": "1"}), {
            "path": "/refund", "kind": "member", "expected": None, "observed": "1",
            "expected_present": False, "observed_present": True})

    def test_longer_prefix_retains_the_full_arrays(self):
        self.assertEqual(offline.first_difference(["begin"], ["begin", "return"]), {
            "path": "", "kind": "array-length", "expected": ["begin"],
            "observed": ["begin", "return"]})

    def test_json_pointer_retains_arbitrary_state_names(self):
        self.assertEqual(offline.first_difference({"a/b~c": "7"}, {"a/b~c": "8"})["path"], "/a~1b~0c")

    def test_existing_complete_readback_is_accepted_as_data(self):
        ref, value = offline.frozen_readback(HISTORICAL, HISTORICAL_SHA256)
        offline.readback_shape(value)
        self.assertEqual(ref["sha256"], HISTORICAL_SHA256)
        self.assertEqual(value["execution"]["post"]["state"]["paid"], {"type": "UInt", "value": "70"})
        self.assertEqual(value["evidence"]["ledger_apply"], "not-performed")
        self.assertEqual(value["execution"]["post"]["liabilities"][-1]["amount"], "0")


class NormalInputInventory(unittest.TestCase):
    def test_exact_existing_artifact_and_target_are_retained(self):
        result = payment_inventory()
        supplied = json.loads((BACKEND / "independent-payment.artifact.json").read_text())
        self.assertEqual(result["artifact"], supplied)
        self.assertEqual(result["target"], supplied["target"])
        self.assertIsNone(result["artifact_target_difference"])
        self.assertEqual(result["missing_target_fields"], [])
        # An unchanged local target is never relabelled as a stagenet identity.
        self.assertEqual(result["target"]["network"], "local")
        self.assertEqual(result["requested_target"], "Midnight stagenet")

    def test_selected_public_and_effect_order_remain_the_compiler_order(self):
        result = payment_inventory()
        circuit = result["artifact"]["circuits"][result["selected_circuit_index"]]
        self.assertEqual(circuit["action"], "settle")
        self.assertEqual([b["name"] for b in circuit["public_inputs"][:3]],
                         ["program_target_action.0", "program_target_action.1", "paid"])
        self.assertEqual([e["kind"] for e in circuit["effect_frame"]],
                         ["transfer", "fee", "liability", "state", "emit"])
        self.assertTrue(circuit["liability_frame"])
        self.assertTrue(circuit["authorization_bindings"])
        self.assertTrue(circuit["private_inputs"])

    def test_unsupplied_target_keeps_missing_fields(self):
        result = payment_inventory(target=FIXTURES / "target-unsupplied.json")
        self.assertEqual(result["target"], {})
        self.assertEqual(result["missing_target_fields"], [
            "profile", "network", "anchor", "ledger_revision", "zkir_version",
            "max_instructions", "max_artifact_bytes"])
        self.assertIsNotNone(result["artifact_target_difference"])

    def test_expected_file_is_bound_by_the_supplied_digest(self):
        result = payment_inventory()
        self.assertEqual(result["files"]["expected_readback"]["sha256"], HISTORICAL_SHA256)
        self.assertEqual(result["expected_readback"], json.loads(HISTORICAL.read_text()))
        self.assertEqual(result["expected_readback"]["preserved_frame"]["scope"],
                         "unchanged-supplied-ledger-not-an-applied-successor")
        artifact_bytes = (BACKEND / "independent-payment.artifact.json").read_bytes()
        self.assertEqual(result["files"]["artifact"]["sha256"], hashlib.sha256(artifact_bytes).hexdigest())
        self.assertEqual(result["files"]["artifact"]["bytes"], len(artifact_bytes))

    def test_gaps_survive_preparation_with_a_supplied_local_target(self):
        result = payment_inventory()
        self.assertEqual(result["purpose"], "offline-run-input-inventory")
        self.assertTrue(any("not a mori-ledger" in gap for gap in result["gaps"]))
        self.assertTrue(any("Gross debit" in gap for gap in result["gaps"]))
        self.assertTrue(any("Network fees remain unobserved" in gap for gap in result["gaps"]))


if __name__ == "__main__":
    unittest.main()
