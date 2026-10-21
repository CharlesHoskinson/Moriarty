"""Local synthetic provider fixtures; no fetch, wallet or ledger access."""
import copy
from datetime import datetime, timezone
import hashlib
import json
import unittest

import cardano_preview_observation as adapter
from cardano_preview_readiness import SELECTED_ADDRESS


NOW = datetime(2026, 10, 21, 1, 21, tzinfo=timezone.utc)
REF = "a" * 64 + "#0"


def captures():
    records = {
        "tip": [{"hash": "b" * 64, "epoch_no": 1457, "era": "Conway",
                 "abs_slot": 125889569, "block_height": 4778263,
                 "block_time": int(datetime(2026, 10, 21, 1, 19, 29, tzinfo=timezone.utc).timestamp())}],
        "address-utxos": [{"tx_hash": "a" * 64, "tx_index": 0, "address": SELECTED_ADDRESS,
                           "value": "10000000000", "asset_list": [], "is_spent": False,
                           "epoch_no": 1452, "block_height": 4763754, "block_time": 1792162704}],
        "epoch-params": [{"epoch_no": 1457, "era": "Conway", "min_fee_a": 44,
                          "min_fee_b": 155381, "collateral_percent": 150,
                          "max_collateral_inputs": 3, "protocol_major": 11, "protocol_minor": 0,
                          "max_tx_size": 16384, "max_tx_ex_mem": 17500000,
                          "max_tx_ex_steps": 10000000000, "price_mem": 0.0577,
                          "price_step": 0.0000721, "coins_per_utxo_size": "4310",
                          "cost_models": {"PlutusV3": [1, -900, 2]}}],
    }
    result = {}
    for name, payload in records.items():
        body = json.dumps(payload, separators=(",", ":")).encode()
        endpoint = adapter.ENDPOINTS[name]
        result[name] = {"body": body, "receipt": {
            "observedAt": "2026-10-21T01:19:44Z", "completedAt": "2026-10-21T01:20:18Z",
            "network": "preview", "networkMagic": 2,
            "method": "POST" if name == "address-utxos" else "GET",
            "url": endpoint, "finalURL": endpoint, "httpStatus": 200,
            "bodySHA256": hashlib.sha256(body).hexdigest(), "bytes": len(body),
            "fetcher": "Scrapling0.4.15 Fetcher",
        }}
    return result


def replace_body(bundle, name, payload):
    body = json.dumps(payload, separators=(",", ":")).encode()
    bundle[name]["body"] = body
    bundle[name]["receipt"]["bodySHA256"] = hashlib.sha256(body).hexdigest()
    bundle[name]["receipt"]["bytes"] = len(body)


def handoff():
    vk, proof = b"\x01" * 488, b"\x02" * 192
    return {
        "format": "doctorwatson-utxo-cardano-proof-handoff/1", "ready_for_payload_encoding": True,
        "identity": {"proof_sha256": hashlib.sha256(proof).hexdigest(),
                     "vk_digest_blake2b256": hashlib.blake2b(vk, digest_size=32).hexdigest(),
                     "script_sha256": "c" * 64, "script_hash": "d" * 56,
                     "statement_digest_blake2b256": "e" * 64},
        "statement": {"public_inputs": {"x0": "123", "x1": "456"}},
        "observations": {"K10": {"positive_cases": 1, "evidence_kind": "historical-local-record"}},
        "artifacts": {"K10": {"matches": True, "sha256": "f" * 64}},
        "builder_payload": {"plutus_version": "v3", "policy_id_hex": "d" * 56,
                            "redeemer_data": {"constructor": 0, "fields": [
                                {"bytes": vk.hex()}, {"bytes": proof.hex()}]},
                            "inline_datum_cbor_hex": "d87980"},
    }


class NormalizePreviewObservation(unittest.TestCase):
    def normalize(self, bundle=None, **kwargs):
        return adapter.normalize_preview_observation(
            captures() if bundle is None else bundle, now=NOW, **kwargs)

    def test_normalizes_current_provider_snapshot_without_inventing_selections(self):
        result = self.normalize()
        self.assertTrue(result["valid"], result["blockers"])
        value = result["observation"]
        self.assertEqual(value["network"], {"name": "preview", "magic": 2})
        self.assertEqual(value["utxos"][0]["lovelace"], 10_000_000_000)
        self.assertEqual(value["utxos"][0]["txHash"], "a" * 64)
        self.assertEqual(value["protocolParameters"]["epoch"], 1457)
        self.assertEqual(value["protocolParameters"]["maxTxExecutionUnits"]["memory"], 17500000)
        self.assertNotIn("transaction", value)
        self.assertFalse(result["readiness"]["readyForPreparation"])
        self.assertTrue(result["fundingPreparation"]["required"])
        self.assertIn("existing", result["fundingPreparation"]["nextStep"])
        self.assertIn("self-transfer", result["fundingPreparation"]["nextStep"])

    def test_body_or_receipt_digest_change_withholds_observation(self):
        for change in ("body", "receipt"):
            with self.subTest(change=change):
                bundle = captures()
                if change == "body":
                    bundle["tip"]["body"] += b" "
                else:
                    bundle["tip"]["receipt"]["bodySHA256"] = "0" * 64
                result = self.normalize(bundle)
                self.assertFalse(result["valid"])
                self.assertIsNone(result["observation"])
                self.assertEqual(result["blockers"][0]["code"], "capture-body-mismatch")

    def test_wrong_provider_magic_or_redirect_withholds_observation(self):
        for field, value in (("url", "https://preprod.koios.rest/api/v1/tip"),
                             ("networkMagic", 1), ("networkMagic", True),
                             ("finalURL", "https://example.invalid/api/v1/tip")):
            with self.subTest(field=field, value=value):
                bundle = captures()
                bundle["tip"]["receipt"][field] = value
                self.assertFalse(self.normalize(bundle)["valid"])

    def test_stale_or_future_receipt_does_not_become_current(self):
        for field, value in (("observedAt", "2026-10-20T01:19:44Z"),
                             ("completedAt", "2026-10-21T01:22:00Z")):
            bundle = captures()
            bundle["tip"]["receipt"][field] = value
            result = self.normalize(bundle)
            self.assertEqual(result["blockers"][0]["code"], "capture-not-current")

    def test_protocol_epoch_must_match_tip_not_utxo_creation_epoch(self):
        bundle = captures()
        params = json.loads(bundle["epoch-params"]["body"])
        params[0]["epoch_no"] = 1452
        replace_body(bundle, "epoch-params", params)
        result = self.normalize(bundle)
        self.assertFalse(result["valid"])
        self.assertEqual(result["blockers"][0]["code"], "protocol-tip-mismatch")

    def test_historical_handoff_artifacts_are_reused_without_new_proof(self):
        result = self.normalize(handoff=handoff())
        self.assertTrue(result["valid"])
        value = result["observation"]
        self.assertEqual(value["proof"]["kind"], "groth16")
        self.assertTrue(value["proof"]["verification"]["verified"])
        self.assertEqual(value["proof"]["verification"]["evidenceClass"], "historical-local-record")
        self.assertEqual(value["proof"]["sha256"], handoff()["identity"]["proof_sha256"])
        self.assertTrue(value["payload"]["datumSha256"])
        codes = {row["code"] for row in result["readiness"]["blockers"]}
        self.assertNotIn("groth16-artifact-missing", codes)
        self.assertIn("collateral-input-unavailable", codes)

    def test_one_utxo_cannot_satisfy_both_selected_roles(self):
        transaction = {"fundingInputRefs": [REF], "collateralInputRefs": [REF],
                       "feeLovelace": 500000, "outputLovelace": 5000000,
                       "minimumChangeLovelace": 1000000, "requiredCollateralLovelace": 1500000}
        result = self.normalize(handoff=handoff(), transaction=transaction)
        self.assertTrue(result["valid"])
        self.assertFalse(result["readiness"]["readyForPreparation"])
        self.assertIn("input-roles-overlap", {row["code"] for row in result["readiness"]["blockers"]})

    def test_conversion_does_not_mutate_inputs_or_copy_unknown_fields(self):
        bundle = captures()
        before = copy.deepcopy(bundle)
        plan = {"fundingInputRefs": [REF], "ignoredPrivateField": "DO-NOT-COPY"}
        result = self.normalize(bundle, transaction=plan)
        self.assertEqual(bundle, before)
        self.assertNotIn("DO-NOT-COPY", str(result))

    def test_malformed_selection_values_are_missing_not_copied(self):
        result = self.normalize(transaction={"feeLovelace": {"private": "DO-NOT-COPY"},
                                             "fundingInputRefs": [{"private": "DO-NOT-COPY"}]})
        self.assertTrue(result["valid"])
        self.assertNotIn("DO-NOT-COPY", str(result))
        self.assertIn("transaction-budget-missing", {row["code"] for row in result["readiness"]["blockers"]})

    def test_malformed_handoff_preserves_snapshot_but_withholds_artifact_claims(self):
        source = handoff()
        source["builder_payload"]["inline_datum_cbor_hex"] = None
        result = self.normalize(handoff=source)
        self.assertTrue(result["valid"])
        self.assertNotIn("proof", result["observation"])
        self.assertEqual(result["artifactIssues"][0]["code"], "handoff-payload-mismatch")

    def test_invalid_json_or_missing_capture_returns_named_gap(self):
        bundle = captures()
        del bundle["tip"]
        result = self.normalize(bundle)
        self.assertFalse(result["valid"])
        self.assertEqual(result["blockers"][0]["code"], "capture-missing")
        bundle = captures()
        body = b"[{\"hash\":NaN}]"
        bundle["tip"]["body"] = body
        bundle["tip"]["receipt"].update(bodySHA256=hashlib.sha256(body).hexdigest(), bytes=len(body))
        self.assertEqual(self.normalize(bundle)["blockers"][0]["code"], "capture-json-malformed")


if __name__ == "__main__":
    unittest.main()
