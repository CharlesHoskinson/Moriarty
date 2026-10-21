"""Deterministic full-CLI capture comparisons; no native CLI or provider call."""
import copy
from datetime import datetime, timezone
import hashlib
import json
import unittest

import preview_protocol_parameters as protocol


NOW = datetime(2026, 10, 21, 1, 38, tzinfo=timezone.utc)


def parameters():
    return {
        "protocolVersion": {"major": 11, "minor": 0},
        "txFeePerByte": 44, "txFeeFixed": 155381, "utxoCostPerByte": 4310,
        "maxTxSize": 16384, "maxTxExecutionUnits": {"memory": 17500000, "steps": 10000000000},
        "collateralPercentage": 150, "maxCollateralInputs": 3,
        "executionUnitPrices": {"priceMemory": 0.0577, "priceSteps": 0.0000721},
        "costModels": {"PlutusV1": [1, 2], "PlutusV2": [3, 4], "PlutusV3": [1, -900, 2]},
        "maxBlockBodySize": 90112, "maxBlockHeaderSize": 1100,
        "maxBlockExecutionUnits": {"memory": 77500000, "steps": 20000000000},
        "dRepVotingThresholds": {"motionNoConfidence": 0.67},
        "committeeMinSize": 3,
    }


def observation():
    return {"format": "cardano-preview-observation/1", "network": {"name": "preview", "magic": 2},
            "protocolParameters": {"protocolMajor": 11, "protocolMinor": 0,
                                   "txFeePerByte": 44, "txFeeFixed": 155381, "coinsPerUtxoByte": 4310,
                                   "maxTxSize": 16384,
                                   "maxTxExecutionUnits": {"memory": 17500000, "steps": 10000000000},
                                   "collateralPercentage": 150, "maxCollateralInputs": 3,
                                   "executionUnitPrices": {"memory": "0.057700", "steps": "7.21e-5"},
                                   "costModels": {"PlutusV3": [1, -900, 2]},
                                   "sourceBodySha256": "a" * 64}}


def capture(body=None):
    if body is None:
        # Formatting and trailing newline must survive intact for file emission.
        body = (json.dumps(parameters(), indent=3) + "\n").encode()
    return {"body": body, "receipt": {
        "observedAt": "2026-10-21T01:36:55Z", "completedAt": "2026-10-21T01:37:06Z",
        "network": "preview", "networkMagic": 2, "httpStatus": 200, "method": "GET",
        "url": protocol.ENDPOINT, "finalURL": protocol.ENDPOINT,
        "bodySHA256": hashlib.sha256(body).hexdigest(), "bytes": len(body),
        "fetcher": "Scrapling0.4.15 Fetcher",
    }}


class FullCliProtocolParameters(unittest.TestCase):
    def inspect(self, source=None, observed=None, **kwargs):
        return protocol.inspect_cli_protocol_parameters(
            capture() if source is None else source,
            observation() if observed is None else observed, now=NOW, **kwargs)

    def codes(self, result):
        return {item["code"] for item in result["blockers"]}

    def test_exact_full_body_is_retained_without_json_roundtrip(self):
        source = capture()
        before = copy.deepcopy(source)
        result = self.inspect(source)
        self.assertTrue(result["valid"], result["blockers"])
        self.assertEqual(result["raw_bytes"], source["body"])
        self.assertEqual(result["raw_text"].encode(), source["body"])
        self.assertEqual(result["body_sha256"], source["receipt"]["bodySHA256"])
        self.assertIn('"PlutusV1"', result["raw_text"])
        self.assertIn('"dRepVotingThresholds"', result["raw_text"])
        self.assertFalse(result["cli_decoder_executed"])
        self.assertFalse(result["atomic_chain_point"])
        self.assertEqual(source, before)

    def test_decimal_equivalent_execution_prices_match_exactly(self):
        result = self.inspect()
        self.assertTrue(result["valid"])
        prices = [item for item in result["comparisons"] if item["field"].startswith("executionUnitPrices")]
        self.assertEqual(len(prices), 2)
        self.assertTrue(all(item["matches"] for item in prices))

    def test_drift_in_every_required_parameter_group_is_reported(self):
        for path in (("protocolVersion", "major"), ("txFeeFixed",), ("txFeePerByte",),
                     ("utxoCostPerByte",), ("maxTxSize",), ("collateralPercentage",),
                     ("maxCollateralInputs",), ("maxTxExecutionUnits", "steps"),
                     ("maxTxExecutionUnits", "memory"), ("costModels", "PlutusV3"),
                     ("executionUnitPrices", "priceMemory"), ("executionUnitPrices", "priceSteps")):
            with self.subTest(path=path):
                values = parameters()
                target = values
                for name in path[:-1]:
                    target = target[name]
                target[path[-1]] = [9, -900, 2] if path[-1] == "PlutusV3" else target[path[-1]] + 1
                result = self.inspect(capture(json.dumps(values).encode()))
                self.assertFalse(result["valid"])
                self.assertIn("parameter-mismatch", self.codes(result))

    def test_wrong_provider_magic_or_http_status_rejects_capture(self):
        for field, value in (("url", "https://preprod.koios.rest/api/v1/cli_protocol_params"),
                             ("finalURL", "https://example.invalid/cli_protocol_params"),
                             ("networkMagic", 1), ("networkMagic", True), ("httpStatus", 500)):
            with self.subTest(field=field, value=value):
                source = capture()
                source["receipt"][field] = value
                result = self.inspect(source)
                self.assertIn("capture-source-mismatch", self.codes(result))
                self.assertIsNone(result["raw_bytes"])

    def test_stale_future_or_unordered_receipt_times_are_named(self):
        for field, value in (("observedAt", "2026-10-20T01:36:55Z"),
                             ("completedAt", "2026-10-21T01:39:00Z"),
                             ("observedAt", "2026-10-21T01:37:07Z")):
            source = capture()
            source["receipt"][field] = value
            self.assertIn("capture-not-current", self.codes(self.inspect(source)))

    def test_body_bytes_and_receipt_hash_are_compared_before_parsing(self):
        source = capture()
        source["body"] += b" "
        self.assertIn("capture-body-mismatch", self.codes(self.inspect(source)))
        source = capture()
        source["receipt"]["bodySHA256"] = "0" * 64
        self.assertIn("capture-body-mismatch", self.codes(self.inspect(source)))

    def test_object_duplicate_member_and_nonfinite_json_are_rejected(self):
        for body in (b"[]", b'{"txFeeFixed":1,"txFeeFixed":2}', b'{"txFeeFixed":NaN}'):
            self.assertIn("capture-json-malformed", self.codes(self.inspect(capture(body))))

    def test_numeric_bools_and_word64_overflow_are_not_coerced(self):
        for value in (True, -1, 2**64):
            for path in (("txFeePerByte",), ("protocolVersion", "major"), ("maxTxExecutionUnits", "steps")):
                with self.subTest(value=value, path=path):
                    values = parameters()
                    target = values
                    for name in path[:-1]:
                        target = target[name]
                    target[path[-1]] = value
                    self.assertIn("parameter-value-invalid", self.codes(self.inspect(capture(json.dumps(values).encode()))))
        observed = observation()
        observed["protocolParameters"]["executionUnitPrices"]["memory"] = True
        self.assertIn("parameter-value-invalid", self.codes(self.inspect(observed=observed)))

    def test_missing_normalized_field_does_not_become_agreement(self):
        observed = observation()
        del observed["protocolParameters"]["protocolMinor"]
        self.assertIn("parameter-value-invalid", self.codes(self.inspect(observed=observed)))

    def test_normalized_preview_identity_and_source_pin_are_required(self):
        observed = observation()
        observed["network"]["magic"] = 1
        self.assertIn("normalized-observation-invalid", self.codes(self.inspect(observed=observed)))
        observed = observation()
        del observed["protocolParameters"]["sourceBodySha256"]
        self.assertIn("normalized-observation-invalid", self.codes(self.inspect(observed=observed)))


if __name__ == "__main__":
    unittest.main()
