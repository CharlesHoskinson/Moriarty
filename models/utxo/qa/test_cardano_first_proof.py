"""Synthetic selector tests: these records are never ledger evidence."""

import copy
import importlib
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parent))
selector = importlib.import_module("cardano_first_proof")


class FirstProofSelection(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.repo = Path(self.temp.name)
        source = self.repo / "openspec/changes/utxo-doctorwatson/current-evidence.json"
        source.parent.mkdir(parents=True)
        source.write_text(json.dumps({"format": "doctorwatson-utxo.evidence/1", "rows": [
            {"id": rid, "heading": rid + " existing obligation", "acceptance": {"status": "open"}}
            for rid in ("CS04", "CS06", "CS10", "EU-CON-012", "EU-CON-014", "ALL-LANGUAGE")
        ]}), encoding="utf-8")
        self.subject = {
            "proof_export_sha256": "a" * 64,
            "vk_digest_blake2b256": "b" * 64,
            "statement_digest_blake2b256": "c" * 64,
            "script_hash": "d" * 56,
        }
        self.handoff = {"ready_for_payload_encoding": True, "identity": self.subject,
                        "artifacts": {}, "observations": {"K10": "historical-local"},
                        "blockers": [], "next_step": "Use the exact existing proof and C2."}
        self.start_patch("_inspect_handoff", return_value=self.handoff)
        self.start_patch("_graph_context", return_value={"status": "STALE", "coverage": "partial"})
        self.start_patch("_preview_readiness", return_value={
            "readyForPreparation": True, "nextStep": "Build the exact Preview transaction.",
            "targetAcceptance": False, "blockers": []})

    def start_patch(self, name, **kwargs):
        mock = patch.object(selector, name, **kwargs)
        result = mock.start()
        self.addCleanup(mock.stop)
        return result

    def observations(self):
        predicates = {
            "preview-transaction-build": ("current-local", "transaction-built"),
            "budgeted-evaluation": ("current-local", "budget-fit"),
            "preview-submit": ("preview-chain", "submitted"),
            "preview-readback": ("preview-chain", "accepted-utxo-observed"),
            "tampered-proof-refusal": ("preview-evaluation", "tampered-proof-rejected"),
            "settlement-readback": ("preview-chain", "settlement-effects-confirmed"),
        }
        observations = {"stages": {key: {
            "scope": scope, "result": result, "subject": copy.deepcopy(self.subject),
            "evidence_refs": ["public/qualified-owner-record.json"],
            "network": "cardano-preview", "network_magic": 2,
            "body_hash": "e" * 64, "tx_id": "e" * 64,
            "accepted_utxo": "e" * 64 + "#0", "block_hash": "f" * 64,
        } for key, (scope, result) in predicates.items()}}
        observations["stages"]["tampered-proof-refusal"]["control_body_hash"] = "e" * 64
        observations["stages"]["settlement-readback"].update(
            effects_evidence_refs=["public/complete-independent-effects-comparison.json"],
            confirmation_evidence_refs=["public/named-chain-confirmation.json"])
        return observations

    def test_reuses_historical_tuple_and_selects_real_builder(self):
        result = selector.build_cardano_status(self.repo)
        self.assertEqual(result["selected_action"]["id"], "preview-transaction-build")
        self.assertEqual(result["stages"][0]["status"], "artifact-matched-historical-local")
        self.assertEqual(result["acceptance"]["preview_chain"]["status"], "open")
        self.assertEqual(result["acceptance"]["current_local"]["status"], "open")
        self.assertEqual(result["graph_context"]["status"], "STALE")
        self.assertTrue(result["wider_exits_open"])
        self.assertEqual(result["requirements"]["ALL-LANGUAGE"]["acceptance"], {"status": "open"})

    def test_missing_handoff_selects_public_artifact_inspection(self):
        self.handoff.update(ready_for_payload_encoding=False, blockers=["C2 hash mismatch"])
        result = selector.build_cardano_status(self.repo)
        self.assertEqual(result["selected_action"]["id"], "proof-handoff")
        self.assertIn("C2 hash mismatch", result["stages"][0]["missing"])

    def test_stage_order_requires_each_concrete_observation(self):
        observations = self.observations()
        supplied = {"stages": {}}
        for stage_id, record in observations["stages"].items():
            self.assertEqual(selector.build_cardano_status(self.repo, supplied)["selected_action"]["id"], stage_id)
            supplied["stages"][stage_id] = record
        result = selector.build_cardano_status(self.repo, supplied)
        self.assertIsNone(result["selected_action"])
        self.assertEqual(result["acceptance"]["preview_chain"]["status"], "reported-observed")
        self.assertTrue(result["wider_exits_open"])

    def test_exit_zero_and_artifacts_never_establish_chain_acceptance(self):
        observations = {"stages": {key: {"exit_code": 0, "artifact_present": True}
                                    for key in self.observations()["stages"]}}
        result = selector.build_cardano_status(self.repo, observations)
        self.assertEqual(result["selected_action"]["id"], "preview-transaction-build")
        self.assertEqual(result["acceptance"]["preview_chain"]["status"], "open")

    def test_historical_scope_and_wrong_proof_cannot_advance(self):
        for change in ({"scope": "historical-local"}, {"subject": dict(self.subject, script_hash="0" * 56)}):
            with self.subTest(change=change):
                observations = self.observations()
                observations["stages"]["preview-transaction-build"].update(change)
                result = selector.build_cardano_status(self.repo, observations)
                self.assertEqual(result["selected_action"]["id"], "preview-transaction-build")

    def test_wrong_network_and_changed_body_do_not_reuse_local_evaluation(self):
        for change in ({"network": "cardano-mainnet"}, {"body_hash": "0" * 64}):
            with self.subTest(change=change):
                observations = self.observations()
                observations["stages"]["preview-submit"].update(change)
                result = selector.build_cardano_status(self.repo, observations)
                self.assertEqual(result["selected_action"]["id"], "preview-submit")
                self.assertEqual(result["acceptance"]["preview_chain"]["status"], "open")

    def test_submit_ack_cannot_replace_accepted_utxo_or_settlement_readback(self):
        observations = self.observations()
        observations["stages"]["preview-readback"].pop("accepted_utxo")
        result = selector.build_cardano_status(self.repo, observations)
        self.assertEqual(result["selected_action"]["id"], "preview-readback")
        observations = self.observations()
        observations["stages"].pop("settlement-readback")
        self.assertEqual(selector.build_cardano_status(self.repo, observations)["selected_action"]["id"], "settlement-readback")

    def test_missing_evidence_refs_never_promote_a_report(self):
        observations = self.observations()
        observations["stages"]["budgeted-evaluation"]["evidence_refs"] = []
        self.assertEqual(selector.build_cardano_status(self.repo, observations)["selected_action"]["id"], "budgeted-evaluation")

    def test_later_settlement_transaction_links_back_to_claim_utxo(self):
        observations = self.observations()
        settlement = observations["stages"]["settlement-readback"]
        settlement.update(body_hash="1" * 64, tx_id="1" * 64,
                          accepted_utxo="1" * 64 + "#1",
                          claim_tx_id="e" * 64,
                          consumed_claim_utxo="e" * 64 + "#0")
        result = selector.build_cardano_status(self.repo, observations)
        self.assertIsNone(result["selected_action"])
        settlement["consumed_claim_utxo"] = "0" * 64 + "#0"
        result = selector.build_cardano_status(self.repo, observations)
        self.assertEqual(result["selected_action"]["id"], "settlement-readback")

    def test_tampered_attempt_can_differ_from_its_positive_control(self):
        observations = self.observations()
        record = observations["stages"]["tampered-proof-refusal"]
        record["body_hash"] = "2" * 64
        self.assertIsNone(selector.build_cardano_status(self.repo, observations)["selected_action"])
        record.pop("control_body_hash")
        result = selector.build_cardano_status(self.repo, observations)
        self.assertEqual(result["selected_action"]["id"], "tampered-proof-refusal")

    def test_claim_utxo_report_does_not_replace_settlement_effects_evidence(self):
        observations = self.observations()
        observations["stages"]["settlement-readback"].pop("effects_evidence_refs")
        result = selector.build_cardano_status(self.repo, observations)
        self.assertEqual(result["selected_action"]["id"], "settlement-readback")

    def test_funding_readiness_remains_a_concrete_builder_dependency(self):
        self.start_patch("_preview_readiness", return_value={
            "readyForPreparation": False, "nextStep": "Observe funded public Preview outrefs.",
            "blockers": [{"code": "funding-missing", "detail": "No funded outref observation."}]})
        result = selector.build_cardano_status(self.repo, self.observations())
        self.assertEqual(result["selected_action"]["id"], "preview-transaction-build")
        self.assertIn("Observe funded public Preview outrefs.", result["selected_action"]["evidence_required"])

    def test_existing_single_utxo_selects_collateral_preparation_under_builder(self):
        observations = {"preview": {"fundingPreparation": {
            "required": True, "reason": "single-utxo-no-distinct-collateral",
            "nextStep": "Reuse the existing selected wallet UTxO: prepare a bounded self-transfer that creates distinct funding and collateral outputs, then observe settled outrefs before building the proof transaction.",
        }}}
        result = selector.build_cardano_status(self.repo, observations)
        action = result["selected_action"]
        self.assertEqual(action["id"], "preview-transaction-build")
        self.assertIn("existing selected-wallet UTxO", action["description"])
        self.assertIn("collateral", action["description"])
        self.assertIn("self-transfer", " ".join(action["evidence_required"]))
        self.assertEqual(result["funding_preparation"]["reason"], "single-utxo-no-distinct-collateral")

    def test_complete_caller_reports_never_authenticate_financial_settlement(self):
        result = selector.build_cardano_status(self.repo, self.observations())
        self.assertIsNone(result["selected_action"])
        self.assertEqual(result["acceptance"]["preview_chain"]["status"], "reported-observed")
        self.assertFalse(result["acceptance"]["preview_chain"]["authenticated"])
        self.assertEqual(result["acceptance"]["financial_settlement"]["status"], "open")
        self.assertFalse(result["target_acceptance"])

    def test_read_only_and_input_not_mutated(self):
        observations = self.observations()
        before = copy.deepcopy(observations)
        files_before = {str(p): p.read_bytes() for p in self.repo.rglob("*") if p.is_file()}
        with patch("subprocess.Popen", side_effect=AssertionError("no process")), \
             patch("socket.socket", side_effect=AssertionError("no network")):
            selector.build_cardano_status(self.repo, observations)
        self.assertEqual(observations, before)
        self.assertEqual(files_before, {str(p): p.read_bytes() for p in self.repo.rglob("*") if p.is_file()})

    def test_malformed_canonical_evidence_is_reported(self):
        source = self.repo / "openspec/changes/utxo-doctorwatson/current-evidence.json"
        source.write_text("[]", encoding="utf-8")
        with self.assertRaises(ValueError):
            selector.build_cardano_status(self.repo)


if __name__ == "__main__":
    unittest.main()
