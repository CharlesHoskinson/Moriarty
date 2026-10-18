"""Optional SYNTHETIC FORMAT tests, never actual model capture evidence.

Authored after the decoder was frozen and after the author saw the actual ITF
shape. These literal encoding specimens are not a reserve execution oracle.
"""
import copy
import unittest

from decode_reserve_itf import decode_trace, effect, first_difference, observation

SYNTHETIC_RECORD = {
    "c": {
        "specimen": {"tag": "Reserve", "value": {"#tup": []}},
        "balances": {"#map": [["synthetic_b", {"#bigint": "2"}],
                                ["synthetic_a", {"#bigint": "1"}]]},
        "programState": {"#map": [["synthetic_state", {"#bigint": "7"}]]},
        "dutyPresent": True,
        "dutyAmount": {"#bigint": "8"},
        "site": {"#bigint": "2"},
        "captures": [{"#bigint": "3"}, {"#bigint": "4"}],
        "committed": {"#bigint": "9"},
    },
    "last": {
        "status": {"tag": "Waiting", "value": {"#tup": []}},
        "attempted": {"#bigint": "10"},
        "advanced": True,
        "hasSegment": True,
        "newContinuation": True,
        "effects": [{"tag": "StateValue", "value": {
            "name": "synthetic_state", "before": {"#bigint": "7"},
            "after": {"#bigint": "8"}}}],
        "sourceSteps": [{"#bigint": "1"}, {"#bigint": "2"}],
        "errorSteps": [],
    },
    "attemptedTotal": {"#bigint": "50"},
}
SYNTHETIC_COMMON = {
    "c": {"specimen": "Reserve", "balances": {"synthetic_b": 2, "synthetic_a": 1},
          "programState": {"synthetic_state": 7}, "dutyPresent": True,
          "dutyAmount": 8, "site": 2, "captures": [3, 4], "committed": 9},
    "last": {"status": "Waiting", "attempted": 10, "advanced": True,
             "hasSegment": True, "newContinuation": True,
             "effects": [{"tag": "StateValue", "name": "synthetic_state", "before": 7, "after": 8}],
             "sourceSteps": [1, 2], "errorSteps": []},
    "attemptedTotal": 50,
}


class NormalSyntheticFormatTests(unittest.TestCase):
    def test_literal_full_record_retains_all_shared_fields(self):
        self.assertEqual(observation(SYNTHETIC_RECORD, "synthetic"), SYNTHETIC_COMMON)

    def test_literal_resume_effect_payload_forms(self):
        specimens = [
            ({"tag": "StateValue", "value": {"name": "x", "before": {"#bigint": "7"},
                "after": {"#bigint": "8"}}},
             {"tag": "StateValue", "name": "x", "before": 7, "after": 8}),
            ({"tag": "TransferAmount", "value": {"#bigint": "12"}},
             {"tag": "TransferAmount", "amount": 12}),
            ({"tag": "DutyValue", "value": {"before": {"#bigint": "14"},
                "after": {"#bigint": "13"}}},
             {"tag": "DutyValue", "before": 14, "after": 13}),
            ({"tag": "FeeAmount", "value": {"#bigint": "6"}},
             {"tag": "FeeAmount", "amount": 6}),
            ({"tag": "EmitQuote", "value": {"#bigint": "15"}},
             {"tag": "EmitQuote", "quote": 15}),
        ]
        for raw, expected in specimens:
            with self.subTest(constructor=raw["tag"]):
                self.assertEqual(effect(raw, "synthetic-effect"), expected)

    def test_five_synthetic_records_keep_capture_order_and_exact_variable_name(self):
        variable = "synthetic_namespace::observation"
        states = []
        expected = []
        for i in range(5):
            raw = copy.deepcopy(SYNTHETIC_RECORD)
            raw["attemptedTotal"] = {"#bigint": str(50 + i)}
            states.append({"#meta": {"index": i}, variable: raw})
            row = copy.deepcopy(SYNTHETIC_COMMON)
            row["attemptedTotal"] = 50 + i
            expected.append(row)
        trace = {"#meta": {"format": "ITF", "status": "passed",
                          "description": "SYNTHETIC format specimen, not model output"},
                 "vars": [variable], "states": states}
        self.assertEqual(decode_trace(trace, variable), expected)

    def test_common_json_comparison_retains_scalar_type_and_array_order(self):
        self.assertIsNotNone(first_difference({"flag": True}, {"flag": 1}))
        self.assertIsNotNone(first_difference([1, 2], [2, 1]))
        self.assertIsNone(first_difference({"b": 2, "a": 1}, {"a": 1, "b": 2}))


if __name__ == "__main__":
    unittest.main()
