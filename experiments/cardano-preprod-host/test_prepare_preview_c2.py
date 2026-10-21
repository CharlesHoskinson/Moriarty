"""Offline preparation expectations. Synthetic funding is never ledger evidence."""
import copy
from contextlib import chdir, redirect_stderr, redirect_stdout
from datetime import datetime, timezone
import hashlib
import importlib.util
import inspect
import io
import json
from pathlib import Path
import tempfile
import unittest

MODULE = Path(__file__).with_name("prepare_preview_c2.py")
REPO = Path(__file__).resolve().parents[2]
ADDRESS = "addr_test1vzt2n53u2vlr8ggxz8xan6ywrw8m55umfwvrgmzh3wmcmkcfxunjc"
FUNDING = "11" * 32 + "#0"
COLLATERAL = "22" * 32 + "#1"
NOW = datetime(2026, 10, 21, 1, 38, tzinfo=timezone.utc)


def full_cli_fixture():
    """Synthetic full Conway JSON shape; no native decoding or fee claim."""
    return {
        "collateralPercentage": 150, "committeeMaxTermLength": 365, "committeeMinSize": 3,
        "costModels": {name: [1, 2, 3] for name in ("PlutusV1", "PlutusV2", "PlutusV3")},
        "dRepActivity": 31, "dRepDeposit": 500000000,
        "dRepVotingThresholds": {name: 0.67 for name in (
            "committeeNoConfidence", "committeeNormal", "hardForkInitiation", "motionNoConfidence",
            "ppEconomicGroup", "ppGovGroup", "ppNetworkGroup", "ppTechnicalGroup", "treasuryWithdrawal", "updateToConstitution")},
        "executionUnitPrices": {"priceMemory": 0.0577, "priceSteps": 0.0000721},
        "govActionDeposit": 1000000000, "govActionLifetime": 30,
        "maxBlockBodySize": 90112, "maxBlockExecutionUnits": {"memory": 77500000, "steps": 20000000000},
        "maxBlockHeaderSize": 1100, "maxCollateralInputs": 3,
        "maxTxExecutionUnits": {"memory": 17500000, "steps": 10000000000},
        "maxTxSize": 16384, "maxValueSize": 5000, "minFeeRefScriptCostPerByte": 15,
        "minPoolCost": 75000000, "monetaryExpansion": 0.003, "poolPledgeInfluence": 0.3,
        "poolRetireMaxEpoch": 18,
        "poolVotingThresholds": {name: 0.51 for name in (
            "committeeNoConfidence", "committeeNormal", "hardForkInitiation", "motionNoConfidence", "ppSecurityGroup")},
        "protocolVersion": {"major": 11, "minor": 0}, "stakeAddressDeposit": 2000000,
        "stakePoolDeposit": 500000000, "stakePoolTargetNum": 500, "treasuryCut": 0.2,
        "txFeeFixed": 155381, "txFeePerByte": 44, "utxoCostPerByte": 4310,
    }


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
            "min_collateral_return_lovelace": 1000000,
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
                "costModels": full_cli_fixture()["costModels"],
                "sourceBodySha256": "ab" * 32,
                "txFeePerByte": 44, "txFeeFixed": 155381, "maxTxSize": 16384,
                "executionUnitPrices": {"memory": "0.0577", "steps": "0.0000721"},
                "coinsPerUtxoByte": 4310, "protocolMajor": 11, "protocolMinor": 0,
            },
            "provenance": {"evidenceClass": "synthetic-test", "observedAt": "2026-10-21T01:37:00+00:00"},
        }
        # Preserve whitespace/order exactly; emitted bytes must not be canonicalized.
        text = "  " + json.dumps(full_cli_fixture(), indent=1) + "\n\n"
        self.request["cli_protocol_parameters_capture"] = {
            "body_text": text,
            "receipt": {
                "observedAt": "2026-10-21T01:37:00+00:00", "completedAt": "2026-10-21T01:37:01+00:00",
                "httpStatus": 200, "url": "https://preview.koios.rest/api/v1/cli_protocol_params",
                "finalURL": "https://preview.koios.rest/api/v1/cli_protocol_params", "method": "GET",
                "bodySHA256": hashlib.sha256(text.encode()).hexdigest(), "bytes": len(text.encode()),
                "network": "preview", "networkMagic": 2,
                "fetcher": "Scrapling0.4.15 Fetcher",
            },
        }

    def plan(self, output_dir="prepared"):
        self.assertIsNotNone(self.module, "offline C2 builder preparation is missing")
        self.assertIn("now", inspect.signature(self.module.prepare_plan).parameters,
                      "capture freshness needs a deterministic comparison time")
        return self.module.prepare_plan(REPO, self.request, self.observation, output_dir, now=NOW)

    def test_default_and_relative_output_paths_are_absolute_for_both_planners(self):
        with tempfile.TemporaryDirectory() as directory, chdir(directory):
            c2 = self.module.prepare_plan(REPO, self.request, self.observation, now=NOW)
            split = self.module.prepare_funding_split_plan(self.split_request(), self.observation)
            relative = self.plan("nested/public")
            self.assertEqual(c2["output_directory"], str(Path(directory) / "prepared-c2-preview"))
            self.assertEqual(split["output_directory"], str(Path(directory) / "prepared-preview-funding-split"))
            self.assertEqual(relative["output_directory"], str(Path(directory) / "nested/public"))
            for plan in (c2, split, relative):
                self.assertTrue(plan["ready_for_build_argv"], plan["blockers"])
                argv = plan["build_raw_argv"]
                for index, flag in enumerate(argv):
                    if flag in ("--mint-script-file", "--mint-redeemer-file", "--protocol-params-file",
                                "--tx-out-inline-datum-file", "--out-file"):
                        self.assertTrue(Path(argv[index + 1]).is_absolute(), (flag, argv[index + 1]))

    def main_fixture(self, directory, kind="c2-mint"):
        request = copy.deepcopy(self.request) if kind == "c2-mint" else self.split_request()
        observation = copy.deepcopy(self.observation)
        if kind == "c2-mint":
            stamp = datetime.now(timezone.utc).isoformat()
            request["cli_protocol_parameters_capture"]["receipt"].update(observedAt=stamp, completedAt=stamp)
            observation["provenance"]["observedAt"] = stamp
        request_path, observation_path = directory / "request.json", directory / "observation.json"
        request_path.write_text(json.dumps(request), encoding="utf-8")
        observation_path.write_text(json.dumps(observation), encoding="utf-8")
        return ["--kind", kind, "--repo", str(REPO), "--request", str(request_path),
                "--observation", str(observation_path), "--out-dir", "public"]

    def invoke_main(self, args):
        stdout, stderr = io.StringIO(), io.StringIO()
        with redirect_stdout(stdout), redirect_stderr(stderr):
            try:
                status = self.module.main(args)
            except Exception as error:
                self.fail(f"Python main must report ordinary input/emission errors: {error!r}")
        return status, stdout.getvalue(), stderr.getvalue()

    def test_python_main_emits_c2_files_with_absolute_argv_and_exact_parameter_capture(self):
        with tempfile.TemporaryDirectory() as directory, chdir(directory):
            root = Path(directory)
            args = self.main_fixture(root)
            status, output, error = self.invoke_main(args)
            self.assertEqual((status, error), (0, ""))
            plan = json.loads(output)
            self.assertEqual(plan["output_directory"], str(root / "public"))
            self.assertEqual(json.loads((root / "public/PLAN.json").read_text()), plan)
            self.assertEqual(json.loads((root / "public/EXPECTED-EFFECTS.json").read_text()), plan["expected_effects"])
            for name, artifact in plan["files"].items():
                content = (root / "public" / name).read_bytes()
                self.assertEqual(hashlib.sha256(content).hexdigest(), artifact["sha256"])
            capture = json.loads((root / "request.json").read_text())["cli_protocol_parameters_capture"]
            self.assertEqual((root / "public/protocol-parameters.json").read_bytes(), capture["body_text"].encode())
            self.assertEqual(plan["expected_effects"]["success"]["fee_lovelace"], 700000)
            self.assertEqual(plan["expected_effects"]["claim_custody"]["status"], "unresolved")
            self.assertFalse((root / "public/unsigned.body").exists())

    def test_python_main_funding_split_emits_expectations_without_proof_reads(self):
        from unittest.mock import patch
        with tempfile.TemporaryDirectory() as directory, chdir(directory):
            root = Path(directory)
            args = self.main_fixture(root, "funding-split")
            with patch.object(self.module, "_handoff", side_effect=AssertionError("split must not read proofs")):
                status, output, error = self.invoke_main(args)
            self.assertEqual((status, error), (0, ""))
            plan = json.loads(output)
            self.assertEqual({p.name for p in (root / "public").iterdir()}, {"PLAN.json", "EXPECTED-EFFECTS.json"})
            self.assertEqual([o["lovelace"] for o in plan["expected_effects"]["success"]["outputs"]], [10000000, 5000000, 4300000])
            self.assertTrue(Path(plan["build_raw_argv"][-1]).is_absolute())

    def test_python_main_bad_json_and_blocked_inputs_leave_no_output_files(self):
        with tempfile.TemporaryDirectory() as directory, chdir(directory):
            root = Path(directory)
            args = self.main_fixture(root)
            (root / "request.json").write_text("{broken", encoding="utf-8")
            status, output, error = self.invoke_main(args)
            self.assertEqual((status, output), (2, ""))
            self.assertEqual(json.loads(error)["error"]["type"], "JSONDecodeError")
            self.assertFalse((root / "public").exists())
            args = self.main_fixture(root, "funding-split")
            request = json.loads((root / "request.json").read_text())
            request["fee_cap_lovelace"] = 1
            (root / "request.json").write_text(json.dumps(request), encoding="utf-8")
            status, output, error = self.invoke_main(args)
            self.assertEqual((status, error), (1, ""))
            self.assertIn("FEE_CAP_EXCEEDED", json.loads(output)["blockers"])
            self.assertFalse((root / "public").exists())

    def test_python_main_refuses_existing_file_and_dangling_link_without_writes(self):
        for dangling in (False, True):
            with self.subTest(dangling=dangling), tempfile.TemporaryDirectory() as directory, chdir(directory):
                root = Path(directory)
                args = self.main_fixture(root, "funding-split")
                output_dir = root / "public"
                output_dir.mkdir()
                existing = output_dir / "PLAN.json"
                target = root / "dangling-target"
                if dangling:
                    existing.symlink_to(target)
                else:
                    existing.write_bytes(b"retained original\n")
                status, output, error = self.invoke_main(args)
                self.assertEqual((status, output), (2, ""))
                self.assertEqual(json.loads(error)["error"]["type"], "FileExistsError")
                self.assertEqual({p.name for p in output_dir.iterdir()}, {"PLAN.json"})
                self.assertFalse(target.exists())
                if dangling:
                    self.assertTrue(existing.is_symlink())
                else:
                    self.assertEqual(existing.read_bytes(), b"retained original\n")

    def test_protocol_file_preserves_exact_full_capture_bytes_and_receipt_binding(self):
        plan = self.plan()
        self.assertTrue(plan["ready_for_build_argv"], plan["blockers"])
        capture = self.request["cli_protocol_parameters_capture"]
        output = plan["files"]["protocol-parameters.json"]
        self.assertEqual(output["content"].encode(), capture["body_text"].encode())
        self.assertEqual(output["sha256"], capture["receipt"]["bodySHA256"])
        self.assertEqual(plan["cli_protocol_parameters"]["receipt"], capture["receipt"])
        self.assertEqual(plan["expected_effects"]["cli_protocol_parameters"], plan["cli_protocol_parameters"])
        self.assertFalse(plan["cli_protocol_parameters"]["cli_decoder_executed"])

    def test_missing_full_capture_blocks_build_instead_of_serializing_normalized_subset(self):
        self.request.pop("cli_protocol_parameters_capture")
        plan = self.plan()
        self.assertFalse(plan["ready_for_build_argv"])
        self.assertIn("CLI_PROTOCOL_PARAMETERS_CAPTURE_REQUIRED", plan["blockers"])
        self.assertNotIn("protocol-parameters.json", plan["files"])
        self.assertIn("capture", plan["next_step"].lower())

    def test_full_capture_hash_or_normalized_parameter_mismatch_blocks_build(self):
        self.request["cli_protocol_parameters_capture"]["receipt"]["bodySHA256"] = "cd" * 32
        plan = self.plan()
        self.assertFalse(plan["ready_for_build_argv"])
        self.assertTrue(any("CLI_PROTOCOL_PARAMETERS_CAPTURE" in b for b in plan["blockers"]))
        self.setUp()
        self.observation["protocolParameters"]["txFeeFixed"] += 1
        self.assertFalse(self.plan()["ready_for_build_argv"])

    def test_issue_redeemer_and_existing_datum_feed_real_build_argv(self):
        plan = self.plan()
        self.assertTrue(plan["ready_for_build_argv"], plan["blockers"])
        argv = plan["build_raw_argv"]
        self.assertEqual(argv[:4], ["cardano-cli", "conway", "transaction", "build-raw"])
        self.assertNotIn("--mint-redeemer-value", argv)
        self.assertEqual(argv[argv.index("--mint-redeemer-file") + 1], str(Path("prepared/c2-issue.redeemer.json").resolve()))
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
        self.assertEqual(effects["phase2_failure"]["maximum_collateral_debit_lovelace"], 1050000)
        self.assertEqual(effects["phase2_failure"]["collateral_return"]["lovelace"], 3950000)
        self.assertEqual(effects["phase2_failure"]["collateral_return"]["address"], ADDRESS)
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
            ("max_collateral_loss_lovelace", 1000000, "COLLATERAL_LOSS_CAP_EXCEEDED"),
            ("claim_lovelace", 2000000, "CLAIM_OUTPUT_BELOW_SUPPLIED_MINIMUM"),
            ("max_claim_loss_lovelace", 2000000, "CLAIM_LOSS_CAP_EXCEEDED"),
        ):
            with self.subTest(key=key):
                original = self.request[key]
                self.request[key] = value
                self.assertIn(code, self.plan()["blockers"])
                self.request[key] = original

    def test_collateral_total_rounds_up_and_return_is_conserved(self):
        self.request["fee_lovelace"] = 700001
        plan = self.plan()
        self.assertTrue(plan["ready_for_build_argv"], plan["blockers"])
        argv = plan["build_raw_argv"]
        self.assertIn("--tx-total-collateral", argv)
        self.assertEqual(argv[argv.index("--tx-total-collateral") + 1], "1050002")
        self.assertEqual(argv[argv.index("--tx-out-return-collateral") + 1], ADDRESS + "+3949998")
        failure = plan["expected_effects"]["phase2_failure"]
        self.assertEqual(failure["total_collateral_lovelace"] + failure["collateral_return"]["lovelace"], 5000000)

    def test_collateral_return_minimum_and_owner_total_loss_ceiling_are_enforced(self):
        self.request["min_collateral_return_lovelace"] = 4000000
        self.assertIn("COLLATERAL_RETURN_BELOW_SUPPLIED_MINIMUM", self.plan()["blockers"])
        self.request["min_collateral_return_lovelace"] = 1000000
        self.request.update(fee_lovelace=4000000, fee_cap_lovelace=4100000,
                            max_collateral_loss_lovelace=7000000)
        self.observation["utxos"][1]["lovelace"] = 10000000
        self.assertIn("COLLATERAL_LOSS_CAP_EXCEEDED", self.plan()["blockers"])

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

    def split_request(self):
        return {
            "funding_ref": FUNDING, "funding_lovelace": 10000000,
            "collateral_lovelace": 5000000, "fee_lovelace": 700000,
            "fee_cap_lovelace": 800000, "min_funding_lovelace": 1000000,
            "min_collateral_lovelace": 1000000, "min_change_lovelace": 1000000,
            "invalid_before": 1000, "invalid_hereafter": 2000,
        }

    def split(self, output_dir="prepared-split", **overrides):
        self.assertIsNotNone(self.module, "offline funding split preparation is missing")
        self.assertTrue(hasattr(self.module, "prepare_funding_split_plan"), "funding split implementation is missing")
        request = self.split_request()
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
