"""Offline preparation expectations. Synthetic funding is never ledger evidence."""
import copy
import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

MODULE = Path(__file__).with_name("prepare_preview_c2.py")
REPO = Path(__file__).resolve().parents[2]
ADDRESS = "addr_test1vzt2n53u2vlr8ggxz8xan6ywrw8m55umfwvrgmzh3wmcmkcfxunjc"
FUNDING = "11" * 32 + "#0"
COLLATERAL = "22" * 32 + "#1"


class OfflineC2Preparation(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.module = None
        if MODULE.exists():
            spec = importlib.util.spec_from_file_location("prepare_preview_c2", MODULE)
            cls.module = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(cls.module)

    def setUp(self):
        self.request = {
            "funding_ref": FUNDING, "collateral_ref": COLLATERAL,
            "change_address": ADDRESS, "claim_lovelace": 3000000,
            "fee_lovelace": 700000, "fee_cap_lovelace": 800000,
            "min_claim_lovelace": 2500000, "min_change_lovelace": 1000000,
            "max_collateral_loss_lovelace": 5000000,
            "max_claim_loss_lovelace": 5000000,
            "execution_units": {"memory": 300000, "steps": 2500000000},
            "execution_units_source": "synthetic planning budget; not measured",
            "fee_source": "synthetic explicit planning amount; not measured",
            "invalid_before": 1000, "invalid_hereafter": 2000,
        }
        self.observation = {
            "format": "cardano-preview-observation/1",
            "network": {"name": "preview", "magic": 2}, "walletAddress": ADDRESS,
            "utxos": [
                {"txHash": "11" * 32, "index": 0, "address": ADDRESS,
                 "lovelace": 20000000, "assets": [], "available": True},
                {"txHash": "22" * 32, "index": 1, "address": ADDRESS,
                 "lovelace": 5000000, "assets": [], "available": True},
            ],
            "protocolParameters": {
                "collateralPercentage": 150, "maxCollateralInputs": 3,
                "maxTxExecutionUnits": {"memory": 17500000, "steps": 10000000000},
                "costModels": {"PlutusV3": [1, 2, 3]},
                "sourceBodySha256": "ab" * 32,
            },
            "provenance": {"evidenceClass": "synthetic-test"},
        }

    def plan(self, output_dir="prepared"):
        self.assertIsNotNone(self.module, "offline C2 builder preparation is missing")
        return self.module.prepare_plan(REPO, self.request, self.observation, output_dir)

    def test_issue_redeemer_and_existing_datum_feed_real_build_argv(self):
        plan = self.plan()
        self.assertTrue(plan["ready_for_build_argv"], plan["blockers"])
        argv = plan["build_raw_argv"]
        self.assertEqual(argv[:4], ["cardano-cli", "conway", "transaction", "build-raw"])
        self.assertNotIn("--mint-redeemer-value", argv)
        self.assertEqual(argv[argv.index("--mint-redeemer-file") + 1], "prepared/c2-issue.redeemer.json")
        self.assertEqual(argv[argv.index("--mint-execution-units") + 1], "(300000,2500000000)")
        redeemer = json.loads(plan["files"]["c2-issue.redeemer.json"]["content"])
        self.assertEqual(redeemer["constructor"], 0)
        self.assertEqual([len(bytes.fromhex(x["bytes"])) for x in redeemer["fields"]], [488, 192])
        script = json.loads(plan["files"]["c2.plutus"]["content"])
        self.assertEqual(script["type"], "PlutusScriptV3")
        self.assertEqual(len(bytes.fromhex(script["cborHex"])), 2770)
        datum = json.loads(plan["files"]["claim.inline-datum.json"]["content"])
        self.assertEqual(datum["fields"][5], {"bytes": "55" * 28})
        self.assertEqual(datum["fields"][-1], {"int": 1893456000000})

    def test_expected_effects_freeze_exact_change_fee_and_collateral(self):
        plan = self.plan()
        effects = plan["expected_effects"]
        self.assertEqual(effects["success"]["funding_input_lovelace"], 20000000)
        self.assertEqual([o["lovelace"] for o in effects["success"]["outputs"]], [3000000, 16300000])
        self.assertEqual(effects["success"]["fee_lovelace"], 700000)
        self.assertEqual(effects["success"]["collateral_consumed_lovelace"], 0)
        self.assertEqual(effects["phase2_failure"]["maximum_collateral_debit_lovelace"], 5000000)
        self.assertFalse(plan["execution_observed"])
        self.assertFalse(plan["financial_settlement_claimed"])
        self.assertFalse(plan["costs_measured"])
        self.assertEqual(effects["claim_custody"]["status"], "unresolved")
        self.assertIn("claimant", effects["claim_custody"]["reason"])

    def test_inline_json_serializes_to_the_exact_existing_envelope_cbor(self):
        plan = self.plan()
        source = MODULE.with_name("host_codec.py")
        spec = importlib.util.spec_from_file_location("public_host_codec", source)
        codec = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(codec)

        def host_data(node):
            if "int" in node:
                return ("int", node["int"])
            if "bytes" in node:
                return ("bytes", bytes.fromhex(node["bytes"]))
            return ("constr", node["constructor"], [host_data(n) for n in node["fields"]])

        data = json.loads(plan["files"]["claim.inline-datum.json"]["content"])
        self.assertEqual(codec.encode_data(host_data(data)).hex(),
                         plan["files"]["claim.inline-datum.cbor"]["content"])

    def test_same_input_cannot_fund_and_collateralize_the_plan(self):
        self.request["collateral_ref"] = FUNDING
        plan = self.plan()
        self.assertFalse(plan["ready_for_build_argv"])
        self.assertIsNone(plan["build_raw_argv"])
        self.assertIn("FUNDING_COLLATERAL_NOT_DISTINCT", plan["blockers"])

    def test_one_available_utxo_reports_split_prerequisite(self):
        self.observation["utxos"] = self.observation["utxos"][:1]
        self.observation["utxos"][0]["lovelace"] = 10000000000
        self.request.pop("collateral_ref")
        plan = self.plan()
        self.assertFalse(plan["ready_for_build_argv"])
        self.assertTrue(plan["funding_preparation"]["required"])
        self.assertIn("FUNDING_COLLATERAL_SPLIT_REQUIRED", plan["blockers"])

    def test_missing_fee_or_budget_is_never_filled_from_k10_costs(self):
        for key in ("fee_lovelace", "execution_units"):
            with self.subTest(key=key):
                old = self.request.pop(key)
                plan = self.plan()
                self.assertFalse(plan["ready_for_build_argv"])
                self.assertIsNone(plan["build_raw_argv"])
                self.request[key] = old

    def test_explicit_preview_and_positive_collateral_parameters_are_required(self):
        self.observation["network"] = {"name": "preprod", "magic": 1}
        self.assertIn("EXPLICIT_PREVIEW_NETWORK_REQUIRED", self.plan()["blockers"])
        self.observation["network"] = {"name": "preview", "magic": 2}
        self.observation["protocolParameters"]["collateralPercentage"] = 0
        self.assertIn("PLUTUS_V3_PROTOCOL_PARAMETERS_REQUIRED", self.plan()["blockers"])

    def test_protocol_override_cannot_change_observed_cost_model_and_keeps_source_binding(self):
        self.request["protocol_parameters"] = copy.deepcopy(self.observation["protocolParameters"])
        self.request["protocol_parameters"]["costModels"]["PlutusV3"] = [9, 9, 9]
        plan = self.plan()
        self.assertIn("PROTOCOL_PARAMETERS_OVERRIDE_MISMATCH", plan["blockers"])
        self.assertIsNone(plan["build_raw_argv"])
        self.request.pop("protocol_parameters")
        plan = self.plan()
        self.assertEqual(plan["protocol_source_body_sha256"], "ab" * 32)
        self.assertEqual(plan["expected_effects"]["protocol_source_body_sha256"], "ab" * 32)

    def test_explicit_caps_and_minimums_are_enforced(self):
        for key, value, code in (
            ("fee_cap_lovelace", 600000, "FEE_CAP_EXCEEDED"),
            ("max_collateral_loss_lovelace", 4000000, "COLLATERAL_LOSS_CAP_EXCEEDED"),
            ("claim_lovelace", 2000000, "CLAIM_OUTPUT_BELOW_SUPPLIED_MINIMUM"),
            ("max_claim_loss_lovelace", 2000000, "CLAIM_LOSS_CAP_EXCEEDED"),
        ):
            with self.subTest(key=key):
                original = self.request[key]
                self.request[key] = value
                self.assertIn(code, self.plan()["blockers"])
                self.request[key] = original

    def test_plan_is_deterministic_and_emits_only_public_offline_files(self):
        request, observation = copy.deepcopy(self.request), copy.deepcopy(self.observation)
        with tempfile.TemporaryDirectory() as directory:
            plan = self.plan(directory)
            self.assertEqual(plan, self.plan(directory))
            self.module.emit_plan(plan, Path(directory))
            files = list(Path(directory).iterdir())
            self.assertEqual(set(p.name for p in files), set(plan["files"]) | {"PLAN.json", "EXPECTED-EFFECTS.json"})
            self.assertEqual(json.loads((Path(directory) / "EXPECTED-EFFECTS.json").read_text()), plan["expected_effects"])
            for name, artifact in plan["files"].items():
                self.assertEqual(hashlib.sha256((Path(directory) / name).read_bytes()).hexdigest(), artifact["sha256"])
            with self.assertRaises(FileExistsError):
                self.module.emit_plan(plan, Path(directory))
        self.assertEqual(self.request, request)
        self.assertEqual(self.observation, observation)

    def split(self, output_dir="prepared-split", **overrides):
        self.assertIsNotNone(self.module, "offline funding split preparation is missing")
        self.assertTrue(hasattr(self.module, "prepare_funding_split_plan"), "funding split implementation is missing")
        request = {
            "funding_ref": FUNDING, "funding_lovelace": 10000000,
            "collateral_lovelace": 5000000, "fee_lovelace": 700000,
            "fee_cap_lovelace": 800000, "min_funding_lovelace": 1000000,
            "min_collateral_lovelace": 1000000, "min_change_lovelace": 1000000,
            "invalid_before": 1000, "invalid_hereafter": 2000,
        }
        request.update(overrides)
        return self.module.prepare_funding_split_plan(request, self.observation, output_dir)

    def test_cli_word64_amount_and_slot_overflow_is_refused(self):
        self.request["invalid_hereafter"] = 1 << 64
        self.assertFalse(self.plan()["ready_for_build_argv"])
        self.assertFalse(self.split(invalid_hereafter=1 << 64)["ready_for_build_argv"])
        self.observation["utxos"][0]["lovelace"] = str(1 << 64)
        self.assertFalse(self.split()["ready_for_build_argv"])

    def test_funding_split_builds_three_exact_selected_wallet_outputs(self):
        plan = self.split()
        self.assertTrue(plan["ready_for_build_argv"], plan["blockers"])
        argv = plan["build_raw_argv"]
        self.assertEqual(argv[:4], ["cardano-cli", "conway", "transaction", "build-raw"])
        self.assertEqual(argv.count("--tx-in"), 1)
        self.assertEqual(argv.count("--tx-out"), 3)
        self.assertFalse(any(("mint" in item or "script" in item or "datum" in item or "sign" in item)
                             for item in argv if item.startswith("--")))
        outputs = plan["expected_effects"]["success"]["outputs"]
        self.assertEqual([o["role"] for o in outputs], ["funding", "collateral", "change"])
        self.assertEqual([o["lovelace"] for o in outputs], [10000000, 5000000, 4300000])
        self.assertEqual({o["address"] for o in outputs}, {ADDRESS})
        self.assertEqual(sum(o["lovelace"] for o in outputs) + 700000, 20000000)
        self.assertFalse(plan["execution_observed"])
        self.assertFalse(plan["costs_measured"])

    def test_funding_split_rejects_assets_and_insufficient_remainder(self):
        self.observation["utxos"][0]["assets"] = [{"policyId": "33" * 28, "assetName": "aa", "quantity": 1}]
        self.assertIn("FUNDING_INPUT_MUST_BE_ADA_ONLY", self.split()["blockers"])
        self.observation["utxos"][0]["assets"] = []
        self.observation["utxos"][0]["lovelace"] = 15000000
        plan = self.split()
        self.assertIn("CHANGE_OUTPUT_BELOW_SUPPLIED_MINIMUM", plan["blockers"])
        self.assertIsNone(plan["build_raw_argv"])

    def test_funding_split_needs_no_proof_or_script_files(self):
        from unittest.mock import patch
        with tempfile.TemporaryDirectory() as directory:
            with patch.object(self.module, "_handoff", side_effect=AssertionError("split must not inspect proof artifacts")):
                plan = self.split(directory)
                self.module.emit_plan(plan, Path(directory))
            self.assertEqual(plan["files"], {})
            self.assertEqual({p.name for p in Path(directory).iterdir()}, {"EXPECTED-EFFECTS.json", "PLAN.json"})


if __name__ == "__main__":
    unittest.main()
