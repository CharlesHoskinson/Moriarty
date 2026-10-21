"""Public JSON/hash/source inspections only; no proof or ledger execution."""
import hashlib
import importlib.util
import json
from pathlib import Path
import shutil
import tempfile
import unittest


MODULE = Path(__file__).with_name("cardano_proof_handoff.py")
REPO = Path(__file__).resolve().parents[3]


class PublicProofHandoff(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        if MODULE.exists():
            spec = importlib.util.spec_from_file_location("cardano_proof_handoff", MODULE)
            cls.module = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(cls.module)
        else:
            cls.module = None

    def inspect(self, repo=REPO):
        self.assertIsNotNone(self.module, "public proof handoff implementation is missing")
        return self.module.inspect_handoff(repo)

    def test_existing_genuine_public_artifacts_are_ready_for_encoding(self):
        result = self.inspect()
        self.assertTrue(result["ready_for_payload_encoding"], result["blockers"])
        self.assertEqual(result["blockers"], [])
        self.assertEqual(result["identity"]["vk_digest_blake2b256"],
                         "d19d99a4ede6bf6ed8ee241dfc4c14f2abf8d73f15a58367f4a38a127cd613bd")
        self.assertEqual(result["identity"]["script_hash"],
                         "6a4b67e7d46dbada1b4f49dd324018324b60b13bebbeb6f33d9d59c1")

    def test_builder_fields_use_existing_c2_issue_abi_and_public_bytes(self):
        result = self.inspect()
        payload = result["builder_payload"]
        redeemer = payload["redeemer_data"]
        self.assertEqual(redeemer["constructor"], 0)
        vk, proof = [bytes.fromhex(field["bytes"]) for field in redeemer["fields"]]
        self.assertEqual((len(vk), len(proof)), (488, 192))
        self.assertEqual(hashlib.sha256(proof).hexdigest(),
                         "ac7c60f332f77b8db6368ac27da91ed7385378add91c64f32529843a4a75a172")
        self.assertEqual(hashlib.blake2b(bytes.fromhex(payload["inline_datum_cbor_hex"]),
                                       digest_size=32).hexdigest(), payload["token_name_hex"])
        self.assertEqual(payload["validator_title"], "c2.c2.mint")
        self.assertTrue(payload["context_fixture_path"].endswith("K10-T1-positive-genuine-issue.uplc.txt"))

    def test_relation_statement_and_lengths_remain_explicit(self):
        result = self.inspect()
        self.assertIn("required_lengths", result)
        self.assertEqual(result["required_lengths"], {
            "verification_key_bytes": 488, "proof_bytes": 192,
            "statement_bytes": 721, "compiled_script_bytes": 2770,
        })
        self.assertEqual(result["relation"]["semantics"], "moriarty-core/1")
        self.assertEqual(result["relation"]["action"], "partial")
        self.assertEqual(result["statement"]["digest_blake2b256"],
                         "c9ef8ea00a7f7b71fc1e049bd956dc8a068e635e31306aca88210711282384cc")
        self.assertEqual(result["statement"]["public_inputs"]["x1"],
                         "268418678883876601707409168117995592842")

    def test_historical_results_do_not_become_authenticated_ledger_claims(self):
        result = self.inspect()
        self.assertEqual(set(result["observations"]), {"K6", "K7", "K8", "K9", "K10"})
        self.assertEqual(result["observations"]["K10"]["positive_cases"], 1)
        self.assertEqual(result["observations"]["K10"]["negative_cases"], 6)
        self.assertEqual(result["scope"]["trust_profile"], "single-party-test-setup/1")
        self.assertEqual(result["scope"]["setup_class"], "Class0")
        self.assertFalse(result["scope"]["authenticated_ledger_context"])
        self.assertFalse(result["scope"]["preview_settlement_observed"])
        self.assertIn("fabricated", result["scope"]["context_description"])
        self.assertIn("builder", result["next_step"].lower())
        self.assertNotIn("setup", result["next_step"].lower())

    def fixture_copy(self):
        temporary = tempfile.TemporaryDirectory(prefix="public-proof-handoff-")
        self.addCleanup(temporary.cleanup)
        root = Path(temporary.name)
        result = self.inspect()
        for artifact in result["artifacts"].values():
            path = artifact["path"]
            (root / path).parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(REPO / path, root / path)
        return root, result

    def test_missing_proof_withholds_builder_payload(self):
        root, reference = self.fixture_copy()
        (root / reference["artifacts"]["proof_export"]["path"]).unlink()
        result = self.inspect(root)
        self.assertFalse(result["ready_for_payload_encoding"])
        self.assertIsNone(result["builder_payload"])
        self.assertTrue(any("proof_export" in item for item in result["blockers"]))

    def test_changed_public_vk_withholds_builder_payload(self):
        root, reference = self.fixture_copy()
        setup = root / reference["artifacts"]["setup_record"]["path"]
        record = json.loads(setup.read_text())
        record["verifyingKeyHex"] = "00" * 488
        setup.write_text(json.dumps(record))
        result = self.inspect(root)
        self.assertFalse(result["ready_for_payload_encoding"])
        self.assertIsNone(result["builder_payload"])
        self.assertTrue(any("setup_record" in item and "SHA-256" in item for item in result["blockers"]))

    def test_absent_checkout_is_a_json_readable_gap(self):
        with tempfile.TemporaryDirectory() as directory:
            result = self.inspect(Path(directory))
        self.assertFalse(result["available"])
        self.assertFalse(result["ready_for_payload_encoding"])
        self.assertIsNone(result["builder_payload"])
        json.dumps(result)


if __name__ == "__main__":
    unittest.main()
