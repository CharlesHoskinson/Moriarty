"""Deterministic public-observation checks; no provider or wallet access."""
import copy
from datetime import datetime, timezone
import unittest

import cardano_preview_readiness as preview


NOW = datetime(2026, 10, 20, 12, tzinfo=timezone.utc)


def observation():
    """Synthetic fixture, deliberately distinct from a current provider result."""
    return {
        "format": "cardano-preview-observation/1",
        "network": {"name": "preview", "magic": 2},
        "walletAddress": preview.SELECTED_ADDRESS,
        "provenance": {"source": "synthetic fixture", "evidenceClass": "fixture",
                       "observedAt": "2026-10-20T11:59:00Z", "maxAgeSeconds": 300},
        "chainTip": {"blockHash": "a" * 64, "slot": 100},
        "protocolParameters": {"txFeePerByte": 44, "txFeeFixed": 155381,
                               "collateralPercentage": 150, "maxCollateralInputs": 3},
        "utxos": [
            {"txHash": "b" * 64, "index": 0, "address": preview.SELECTED_ADDRESS,
             "lovelace": 9_000_000, "assets": [], "available": True},
            {"txHash": "c" * 64, "index": 1, "address": preview.SELECTED_ADDRESS,
             "lovelace": 3_000_000, "assets": [], "available": True},
        ],
        "transaction": {"fundingInputRefs": ["b" * 64 + "#0"],
                        "collateralInputRefs": ["c" * 64 + "#1"],
                        "feeLovelace": 500_000, "outputLovelace": 5_000_000,
                        "minimumChangeLovelace": 1_000_000,
                        "requiredCollateralLovelace": 1_500_000},
        "validator": {"sha256": "d" * 64, "scriptHash": "e" * 56,
                      "language": "PlutusV3", "verificationKeySha256": "f" * 64},
        "proof": {"kind": "groth16", "sha256": "1" * 64,
                  "verificationKeySha256": "f" * 64,
                  "publicInputsSha256": "2" * 64,
                  "verification": {"verified": True, "verifier": "fixture-only verifier",
                                   "proofSha256": "1" * 64,
                                   "verificationKeySha256": "f" * 64,
                                   "publicInputsSha256": "2" * 64}},
        "payload": {"validatorSha256": "d" * 64, "proofSha256": "1" * 64,
                    "verificationKeySha256": "f" * 64,
                    "publicInputsSha256": "2" * 64,
                    "datumSha256": "3" * 64, "redeemerSha256": "4" * 64},
    }


class PreviewReadinessTests(unittest.TestCase):
    def report(self, value=None):
        return preview.evaluate_cardano_preview_readiness(value, now=NOW)

    def codes(self, report):
        return {row["code"] for row in report["blockers"]}

    def test_no_observation_reports_exact_network_and_actionable_gap(self):
        result = self.report()
        self.assertFalse(result["readyForPreparation"])
        self.assertEqual({"name": "preview", "magic": 2}, result["network"])
        self.assertIn("observation-missing", self.codes(result))
        self.assertIn("magic 2", result["nextStep"])

    def test_fixture_checks_prerequisites_without_claiming_live_submission(self):
        source = observation()
        before = copy.deepcopy(source)
        result = self.report(source)
        self.assertTrue(result["prerequisitesSatisfied"])
        self.assertFalse(result["readyForPreparation"])
        self.assertEqual("not_assessed", result["submissionReadiness"])
        self.assertIn("fixture-only", self.codes(result))
        self.assertEqual(source, before)

    def test_public_observation_can_be_ready_for_existing_preparation_route(self):
        source = observation()
        source["provenance"]["evidenceClass"] = "public-provider-observation"
        result = self.report(source)
        self.assertTrue(result["readyForPreparation"])
        self.assertEqual("not_assessed", result["submissionReadiness"])
        self.assertEqual([], result["blockers"])
        self.assertIn("cardano-cli", result["nextStep"])
        self.assertIn("Koios", result["nextStep"])
        self.assertFalse(result["targetAcceptance"])

    def test_testnet_address_and_historical_funding_do_not_select_preview(self):
        source = observation()
        source["network"] = {"id": 0}
        source["historicFundingLovelace"] = 10_000_000_000
        source["utxos"] = []
        result = self.report(source)
        self.assertIn("preview-network-required", self.codes(result))
        self.assertIn("funding-input-unavailable", self.codes(result))

    def test_stale_and_future_observations_do_not_count_as_current(self):
        for observed in ("2026-10-18T12:00:00Z", "2026-10-20T12:00:01Z"):
            with self.subTest(observed=observed):
                source = observation()
                source["provenance"]["observedAt"] = observed
                self.assertIn("observation-not-current", self.codes(self.report(source)))

    def test_fee_and_collateral_are_separate_explicit_requirements(self):
        source = observation()
        source["transaction"]["feeLovelace"] = 4_000_000
        source["transaction"]["requiredCollateralLovelace"] = 3_000_001
        codes = self.codes(self.report(source))
        self.assertIn("funding-insufficient", codes)
        self.assertIn("collateral-insufficient", codes)

    def test_missing_proof_and_mismatched_payload_are_named_gaps(self):
        source = observation()
        source["proof"]["verification"]["verified"] = False
        source["payload"]["publicInputsSha256"] = "0" * 64
        codes = self.codes(self.report(source))
        self.assertIn("proof-verification-missing", codes)
        self.assertIn("payload-binding-mismatch", codes)

    def test_missing_budget_is_unknown_instead_of_zero_cost(self):
        source = observation()
        del source["transaction"]["feeLovelace"]
        self.assertIn("transaction-budget-missing", self.codes(self.report(source)))

    def test_current_protocol_parameters_are_required(self):
        source = observation()
        del source["protocolParameters"]
        self.assertIn("protocol-parameters-missing", self.codes(self.report(source)))

    def test_duplicate_utxo_references_do_not_create_funding(self):
        source = observation()
        duplicate = copy.deepcopy(source["utxos"][0])
        duplicate["available"] = False
        source["utxos"].insert(0, duplicate)
        self.assertIn("utxo-snapshot-malformed", self.codes(self.report(source)))

    def test_selected_refs_must_exist_and_be_disjoint(self):
        source = observation()
        source["transaction"]["collateralInputRefs"] = source["transaction"]["fundingInputRefs"]
        self.assertIn("input-roles-overlap", self.codes(self.report(source)))
        source["transaction"]["fundingInputRefs"] = ["0" * 64 + "#0"]
        self.assertIn("funding-input-unavailable", self.codes(self.report(source)))

    def test_non_object_and_malformed_sections_return_diagnostics(self):
        for value in ([], "", {"network": None}, {"utxos": [None]}, {"proof": []}):
            with self.subTest(value=value):
                result = self.report(value)
                self.assertFalse(result["readyForPreparation"])
                self.assertTrue(result["blockers"])

    def test_output_does_not_echo_unknown_input_fields(self):
        source = observation()
        source["ignoredPrivateField"] = "DO-NOT-RETURN-THIS"
        self.assertNotIn("DO-NOT-RETURN-THIS", str(self.report(source)))


if __name__ == "__main__":
    unittest.main()
