"""Pure planning fixtures; none is a Preview observation or evaluator result."""
import copy
from contextlib import redirect_stdout
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import tempfile
import unittest


MODULE = Path(__file__).with_name("preview_evaluation_plan.py")
ADDRESS = "addr_test1vzt2n53u2vlr8ggxz8xan6ywrw8m55umfwvrgmzh3wmcmkcfxunjc"
FUNDING, COLLATERAL = "11" * 32 + "#0", "22" * 32 + "#1"


def bound(name, value, captured=False):
    text = json.dumps(value, sort_keys=True) + "\n"
    row = {"path": "/public/" + name, "text": text,
           "sha256": hashlib.sha256(text.encode()).hexdigest()}
    if captured:
        row["capture"] = {
            "network": "preview", "network_magic": 2,
            "receipt_sha256": "ab" * 32,
            "source": "https://preview.example.invalid/UNIT-ONLY",
            "observed_at": "2026-10-20T00:00:00Z",
        }
    return row


class PreviewEvaluationPlanTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        spec = importlib.util.spec_from_file_location("preview_evaluation_plan", MODULE)
        cls.module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(cls.module)

    def setUp(self):
        self.script = bound("c2.plutus", {"type": "PlutusScriptV3", "cborHex": "4100"})
        self.params = bound("protocol-parameters.json", {"costModels": {"PlutusV3": [1]}})
        self.plan = {
            "format": "doctorwatson-utxo-preview-c2-offline-plan/1",
            "network": "cardano-preview", "network_magic": 2,
            "ready_for_build_argv": True, "identity": {"script_sha256": "cd" * 32},
            "protocol_source_body_sha256": "bc" * 32,
            "files": {name: {"sha256": row["sha256"]} for name, row in
                      (("c2.plutus", self.script), ("protocol-parameters.json", self.params))},
            "build_raw_argv": ["cardano-cli", "conway", "transaction", "build-raw",
                               "--tx-in", FUNDING, "--tx-in-collateral", COLLATERAL,
                               "--mint-script-file", "/public/c2.plutus",
                               "--protocol-params-file", "/public/protocol-parameters.json",
                               "--out-file", "/public/unsigned.body"],
            "expected_effects": {"success": {
                "ordinary_inputs": [FUNDING], "funding_input_lovelace": 20_000_000,
                "collateral_ref_retained": COLLATERAL,
            }, "phase2_failure": {"collateral_input_lovelace": 5_000_000,
                                   "maximum_collateral_debit_lovelace": 5_000_000}},
        }
        self.bindings = {
            "format": "cardano-preview-offline-evaluation-bindings/1",
            "cli": {"path": "/tools/cardano-cli", "sha256": "ef" * 32},
            "body": bound("unsigned.body", {"type": "Tx ConwayEra", "cborHex": "84a0a0f5f6"}),
            "script": self.script, "protocol_parameters": self.params,
            "era_history": bound("era-history.json", {"type": "EraHistory", "cborHex": "80"}, True),
            "system_start": bound("shelley-genesis.json", {
                "networkMagic": 2, "systemStart": "2022-10-25T00:00:00Z"}, True),
            "utxo": bound("utxo.json", {
                FUNDING: {"address": ADDRESS, "value": {"lovelace": 20_000_000}},
                COLLATERAL: {"address": ADDRESS, "value": {"lovelace": 5_000_000}},
            }, True),
            "protocol_capture": {"network": "preview", "network_magic": 2,
                                 "receipt_sha256": "aa" * 32, "source": "UNIT-ONLY",
                                 "source_body_sha256": "bc" * 32,
                                 "observed_at": "2026-10-20T00:00:00Z"},
        }

    def prepare(self):
        return self.module.prepare_evaluation_plan(self.plan, self.bindings, "/new/cost.json")

    def test_exact_offline_argv_and_no_execution_claim(self):
        result = self.prepare()
        self.assertTrue(result["ready_for_evaluation_argv"], result["blockers"])
        self.assertEqual(result["argv"], [
            "/tools/cardano-cli", "conway", "transaction", "calculate-plutus-script-cost", "offline",
            "--start-time-utc", "2022-10-25T00:00:00Z",
            "--era-history-file", "/public/era-history.json", "--utxo-file", "/public/utxo.json",
            "--protocol-params-file", "/public/protocol-parameters.json",
            "--tx-file", "/public/unsigned.body", "--out-file", "/new/cost.json"])
        self.assertFalse(result["execution_observed"])
        self.assertFalse(result["ledger_acceptance_claimed"])
        self.assertFalse(result["body_semantics_decoded"])

    def test_missing_actual_era_history_is_not_replaced_with_fixture(self):
        del self.bindings["era_history"]
        result = self.prepare()
        self.assertFalse(result["ready_for_evaluation_argv"])
        self.assertIn("ERA_HISTORY_BINDING_REQUIRED", result["blockers"])

    def test_offline_refuses_body_only_labels_and_missing_or_invalid_full_tx_hex(self):
        for value in ({"type": "TxBodyConway", "cborHex": "80"},
                      {"type": "TxBody ConwayEra", "cborHex": "a0"},
                      {"type": "Tx ConwayEra", "cborHex": ""},
                      {"type": "Tx ConwayEra", "cborHex": "0"},
                      {"type": "Tx ConwayEra", "cborHex": "zz"},
                      {"type": "Tx ConwayEra", "cborHex": 80},
                      {"type": "Tx ConwayEra"}):
            with self.subTest(value=value):
                self.bindings["body"] = bound("unsigned.body", value)
                result = self.prepare()
                self.assertFalse(result["ready_for_evaluation_argv"])
                self.assertIn("BODY_FULL_CONWAY_TRANSACTION_ENVELOPE_REQUIRED", result["blockers"])
                self.assertIsNone(result["argv"])

    def test_raw_byte_drift_and_foreign_builder_script_are_refused(self):
        self.bindings["body"]["text"] += " "
        self.assertIn("BODY_SHA256_MISMATCH", self.prepare()["blockers"])
        self.bindings["body"] = bound("unsigned.body", {"type": "Tx ConwayEra", "cborHex": "84a0a0f5f6"})
        self.bindings["script"] = bound("c2.plutus", {"type": "PlutusScriptV3", "cborHex": "4101"})
        self.assertIn("SCRIPT_BUILDER_IDENTITY_MISMATCH", self.prepare()["blockers"])

    def test_network_receipt_and_complete_selected_utxo_values_required(self):
        self.bindings["era_history"]["capture"]["network_magic"] = 1
        self.assertIn("ERA_HISTORY_PREVIEW_CAPTURE_REQUIRED", self.prepare()["blockers"])
        self.bindings["era_history"]["capture"]["network_magic"] = 2
        self.bindings["utxo"] = bound("utxo.json", {
            FUNDING: {"address": ADDRESS, "value": {"lovelace": 20_000_000}}}, True)
        self.assertIn("SELECTED_COLLATERAL_UTXO_MISMATCH", self.prepare()["blockers"])

    def test_original_protocol_capture_must_match_builder_source(self):
        self.bindings["protocol_capture"]["source_body_sha256"] = "ff" * 32
        self.assertIn("PROTOCOL_CAPTURE_BUILDER_IDENTITY_MISMATCH", self.prepare()["blockers"])

    def actual_planner_case(self):
        """Call the real planner; fixture roles are synthetic, never live UTxOs."""
        path = MODULE.with_name("test_prepare_preview_c2.py")
        spec = importlib.util.spec_from_file_location("planner_integration_fixture", path)
        fixture = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(fixture)
        fixture.OfflineC2Preparation.setUpClass()
        case = fixture.OfflineC2Preparation()
        case.setUp()
        case.request["fee_lovelace"] = 700_001
        plan = case.plan("/public")
        self.assertTrue(plan["ready_for_build_argv"], plan["blockers"])
        bindings = copy.deepcopy(self.bindings)
        for role, filename in (("script", "c2.plutus"),
                               ("protocol_parameters", "protocol-parameters.json")):
            row = plan["files"][filename]
            bindings[role] = {"path": "/public/" + filename,
                              "text": row["content"], "sha256": row["sha256"]}
        bindings["protocol_capture"]["source_body_sha256"] = plan["protocol_source_body_sha256"]
        return plan, bindings

    def test_actual_planner_collateral_input_is_not_net_failure_debit(self):
        # Independent root oracle frozen before reproduction: SHA99517dbf9e86635b...
        plan, bindings = self.actual_planner_case()
        before = copy.deepcopy((plan, bindings))
        failure = plan["expected_effects"]["phase2_failure"]
        self.assertEqual(failure["collateral_input_lovelace"], 5_000_000)
        self.assertEqual(failure["maximum_collateral_debit_lovelace"], 1_050_002)
        self.assertEqual(failure["total_collateral_lovelace"], 1_050_002)
        self.assertEqual(failure["collateral_return"]["lovelace"], 3_949_998)
        result = self.module.prepare_evaluation_plan(plan, bindings, "/new/cost.json")
        self.assertTrue(result["ready_for_evaluation_argv"], result["blockers"])
        self.assertEqual((plan, bindings), before)
        self.assertFalse(result["execution_observed"])
        self.assertFalse(result["costs_measured"])

    def test_actual_planner_missing_or_mismatched_full_collateral_input_is_refused(self):
        plan, bindings = self.actual_planner_case()
        del plan["expected_effects"]["phase2_failure"]["collateral_input_lovelace"]
        result = self.module.prepare_evaluation_plan(plan, bindings, "/new/cost.json")
        self.assertFalse(result["ready_for_evaluation_argv"])
        self.assertIn("BUILDER_ARGUMENTS_INVALID", result["blockers"])
        plan, bindings = self.actual_planner_case()
        plan["expected_effects"]["phase2_failure"]["collateral_input_lovelace"] = 1_050_002
        result = self.module.prepare_evaluation_plan(plan, bindings, "/new/cost.json")
        self.assertFalse(result["ready_for_evaluation_argv"])
        self.assertIn("SELECTED_COLLATERAL_UTXO_MISMATCH", result["blockers"])

    def test_duplicate_flags_output_collision_and_input_immutability(self):
        before = copy.deepcopy((self.plan, self.bindings))
        self.prepare()
        self.assertEqual((self.plan, self.bindings), before)
        self.plan["build_raw_argv"] += ["--out-file", "/other"]
        self.assertIn("BUILDER_ARGUMENTS_INVALID", self.prepare()["blockers"])
        self.plan = before[0]
        result = self.module.prepare_evaluation_plan(self.plan, self.bindings, "/public/utxo.json")
        self.assertIn("OUTPUT_REUSES_INPUT_PATH", result["blockers"])

    def materialized_main_case(self, root):
        """Write ordinary test DATA; the CLI-labelled file is never executed."""
        plan, bindings = copy.deepcopy((self.plan, self.bindings))
        for role in ("body", "script", "protocol_parameters", "utxo", "era_history", "system_start"):
            row = bindings[role]
            row["path"] = str(root / Path(row["path"]).name)
            Path(row["path"]).write_bytes(row["text"].encode())
        cli = root / "cardano-cli"
        cli.write_bytes(b"UNIT DATA ONLY, NEVER EXECUTE\n")
        bindings["cli"] = {"path": str(cli), "sha256": hashlib.sha256(cli.read_bytes()).hexdigest()}
        plan["build_raw_argv"] = [str(root / item.removeprefix("/public/"))
                                  if item.startswith("/public/") else item
                                  for item in plan["build_raw_argv"]]
        builder_file, bindings_file = root / "builder.json", root / "bindings.json"
        builder_file.write_text(json.dumps(plan))
        bindings_file.write_text(json.dumps(bindings))
        return ["--builder-plan", str(builder_file), "--bindings", str(bindings_file),
                "--cost-out", str(root / "cost.json")]

    def test_main_binds_actual_file_bytes_and_only_emits_offline_argv(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            args = self.materialized_main_case(root)
            before = {p: p.read_bytes() for p in root.iterdir()}
            expected_argv = [str(root / "cardano-cli"), "conway", "transaction",
                             "calculate-plutus-script-cost", "offline", "--start-time-utc",
                             "2022-10-25T00:00:00Z", "--era-history-file", str(root / "era-history.json"),
                             "--utxo-file", str(root / "utxo.json"), "--protocol-params-file",
                             str(root / "protocol-parameters.json"), "--tx-file", str(root / "unsigned.body"),
                             "--out-file", str(root / "cost.json")]
            output = io.StringIO()
            with redirect_stdout(output):
                code = self.module.main(args)
            result = json.loads(output.getvalue())
            self.assertEqual(code, 0)
            self.assertEqual(result["argv"], expected_argv)
            self.assertTrue(result["materialized_inputs_verified"])
            self.assertFalse(result["execution_observed"])
            self.assertFalse((root / "cost.json").exists())
            self.assertEqual({p: p.read_bytes() for p in root.iterdir()}, before)

    def test_main_refuses_materialized_byte_drift(self):
        for name, blocker in (("unsigned.body", "BODY_MATERIALIZED_FILE_MISMATCH"),
                              ("cardano-cli", "CLI_MATERIALIZED_FILE_MISMATCH")):
            with self.subTest(name=name), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                args = self.materialized_main_case(root)
                path = root / name
                path.write_bytes(path.read_bytes() + b" ")
                output = io.StringIO()
                with redirect_stdout(output):
                    code = self.module.main(args)
                result = json.loads(output.getvalue())
                self.assertEqual(code, 1)
                self.assertIn(blocker, result["blockers"])
                self.assertFalse(result["ready_for_evaluation_argv"])
                self.assertFalse(result["materialized_inputs_verified"])
                self.assertIsNone(result["argv"])
                self.assertFalse((root / "cost.json").exists())

    def test_main_refuses_existing_output_without_overwriting(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            args = self.materialized_main_case(root)
            output_file = root / "cost.json"
            expected_bytes = b"EXISTING ORDINARY DATA\n"
            output_file.write_bytes(expected_bytes)
            output = io.StringIO()
            with redirect_stdout(output):
                code = self.module.main(args)
            result = json.loads(output.getvalue())
            self.assertEqual(code, 1)
            self.assertIn("OUTPUT_ALREADY_EXISTS", result["blockers"])
            self.assertIsNone(result["argv"])
            self.assertEqual(output_file.read_bytes(), expected_bytes)

    def test_remote_actual_planner_preserves_frozen_request_bytes_without_dispatch(self):
        plan, _ = self.actual_planner_case()
        transaction = bound("unsigned.body", {"type": "Tx ConwayEra", "cborHex": "84A0a0F5f6"})
        expected_request = b'{"id":"unit-actual-planner","jsonrpc":"2.0","method":"evaluateTransaction","params":{"transaction":{"cbor":"84A0a0F5f6"}}}'
        before = copy.deepcopy((plan, transaction))
        result = self.module.prepare_remote_evaluation_request(
            plan, transaction, transaction["sha256"], "unit-actual-planner")
        self.assertTrue(result["ready_for_http_request"], result["blockers"])
        self.assertEqual(json.dumps(result["request"], sort_keys=True, separators=(",", ":")).encode(), expected_request)
        self.assertEqual((plan, transaction), before)
        self.assertEqual(result["transaction_envelope_sha256"], transaction["sha256"])
        self.assertIsNone(result["ledger_transaction_body_hash"])
        self.assertFalse(result["execution_admitted"])
        self.assertFalse(result["execution_observed"])
        self.assertFalse(result["costs_measured"])

    def test_remote_request_preserves_full_transaction_cbor_without_dispatch(self):
        # Shape-only fixture, never a live transaction or evaluator result.
        transaction = bound("unsigned.body", {"type": "Tx ConwayEra", "cborHex": "84a0a0f5f6"})
        result = self.module.prepare_remote_evaluation_request(
            self.plan, transaction, transaction["sha256"], "unit-c2")
        self.assertTrue(result["ready_for_http_request"], result["blockers"])
        self.assertEqual(result["request"], {"jsonrpc": "2.0", "id": "unit-c2",
                         "method": "evaluateTransaction",
                         "params": {"transaction": {"cbor": "84a0a0f5f6"}}})
        self.assertEqual(result["endpoint"], "https://preview.koios.rest/api/v1/ogmios")
        self.assertEqual(result["whole_transaction_cbor_sha256"],
                         hashlib.sha256(bytes.fromhex("84a0a0f5f6")).hexdigest())
        self.assertEqual(result["transaction_envelope_sha256"], transaction["sha256"])
        self.assertIsNone(result["ledger_transaction_body_hash"])
        self.assertFalse(result["execution_observed"])
        self.assertFalse(result["transaction_contents_verified"])
        self.assertFalse(result["execution_admitted"])

    def test_remote_refuses_body_only_envelope_without_relabelling(self):
        transaction = bound("unsigned.body", {"type": "TxBody ConwayEra", "cborHex": "a0"})
        result = self.module.prepare_remote_evaluation_request(
            self.plan, transaction, transaction["sha256"])
        self.assertFalse(result["ready_for_http_request"])
        self.assertIn("FULL_CONWAY_TRANSACTION_ENVELOPE_REQUIRED", result["blockers"])
        self.assertIsNone(result["request"])

    def test_remote_requires_exact_encoded_output_path_and_byte_digest(self):
        transaction = bound("other.body", {"type": "Tx ConwayEra", "cborHex": "84a0a0f5f6"})
        result = self.module.prepare_remote_evaluation_request(
            self.plan, transaction, transaction["sha256"])
        self.assertIn("TRANSACTION_BUILDER_PATH_MISMATCH", result["blockers"])
        transaction["path"] = "/public/unsigned.body"
        result = self.module.prepare_remote_evaluation_request(self.plan, transaction, "00" * 32)
        self.assertIn("TRANSACTION_ENVELOPE_SHA256_MISMATCH", result["blockers"])

    def test_remote_has_no_additional_utxo_or_mutation_and_refuses_bad_hex(self):
        transaction = bound("unsigned.body", {"type": "Tx ConwayEra", "cborHex": "84a0a0f5f6"})
        before = copy.deepcopy((self.plan, transaction))
        result = self.module.prepare_remote_evaluation_request(self.plan, transaction, transaction["sha256"])
        self.assertEqual((self.plan, transaction), before)
        self.assertEqual(set(result["request"]["params"]), {"transaction"})
        transaction = bound("unsigned.body", {"type": "Tx ConwayEra", "cborHex": "0"})
        result = self.module.prepare_remote_evaluation_request(self.plan, transaction, transaction["sha256"])
        self.assertIn("FULL_CONWAY_TRANSACTION_ENVELOPE_REQUIRED", result["blockers"])


if __name__ == "__main__":
    unittest.main()
