"""Focused tests for the fixed sherlock-settle-result/1 decoder."""
import json
import unittest

import fixture_support as fx
import settle_observation as so


def raw(value):
    return (json.dumps(value, separators=(",", ":")) + "\n").encode()


class SettleObservationTests(unittest.TestCase):
    def decode(self, value, inputs=None):
        return so.decode_sherlock_settle_result(
            raw(value), inputs or fx.settle_inputs())

    def test_pre8_baseline7_post7_succeeds(self):
        out = self.decode(fx.settle_result())
        self.assertEqual(out["phase"], "SETTLED_FINALIZED_READBACK_OK")

    def test_recorded_baseline_is_recomputed_not_trusted(self):
        value = fx.settle_result()
        value["settle_assertion_baseline"]["observation"]["cells"]["root"] = "1"
        with self.assertRaises(so.SettleDecodeError):
            self.decode(value)

    def test_wrong_independent_root_height_and_tx_fail(self):
        cases = []
        v = fx.settle_result(); cases.append(("task-root", v,
            fx.settle_inputs(root_value=str(fx.HUGE + 555))))
        v = fx.settle_result()
        v["post_observation"]["cells"]["height"] = "1001"
        v["post_cells"] = dict(v["post_observation"]["cells"])
        cases.append(("height", v, None))
        v = fx.settle_result()
        v["action_check"]["tx_hash"] = "cc" * 32
        cases.append(("tx", v, None))
        v = fx.settle_result()
        v["pre_observation"]["validity_has_stmt"] = 1
        cases.append(("bool-type", v, None))
        v = fx.settle_result()
        v["post_observation"]["cells"]["root"] = "042"
        v["post_cells"] = dict(v["post_observation"]["cells"])
        cases.append(("noncanonical-decimal", v, None))
        v = fx.settle_result()
        # Producer-test serialization stub: never emitted by the actual
        # assertReadbackAction return; must be refused.
        v["action_check"] = {"operation": "settleValidity",
                             "observed_tx": v["settle_tx_hash"]}
        cases.append(("producer-test-stub-shape", v, None))
        v = fx.settle_result()
        v["receipt"]["status"] = "FAILURE"
        cases.append(("receipt-not-success", v, None))
        v = fx.settle_result()
        v["receipt_check"]["receipt_hash"] = "cc" * 32
        cases.append(("receipt-check-tx", v, None))
        for name, value, inputs in cases:
            with self.subTest(name=name):
                with self.assertRaises(so.SettleDecodeError):
                    self.decode(value, inputs)


if __name__ == "__main__":
    unittest.main()
