"""Pure/mock tests for run_selected; injected stage runners only."""
import json
from pathlib import Path
import tempfile
import unittest

import delivery
import fixture_support as fx
import run_selected as rs


class RunSelectedTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(dir=Path(__file__).parent)
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.empty_history = fx.write(self.root / "empty-history.json", fx.encoded(
            {"schema": "doctorwatson.delivery-history/1", "results": []}))
        self.task = fx.task(self.root, "build", "moriarty-sdk-build")
        self.plan_path = fx.plan_file(self.root, [self.task])
        self.fp = delivery.fingerprint(
            delivery.load_plan(self.plan_path), "build")
        self.outer = fx.outer_fixture(self.root)
        self.guard = self.outer["guard"]
        self.config = self.outer["config_path"]
        self.sel_path = self.root / "selection.json"
        self.write_selection()

    def write_selection(self, fingerprint=None,
                        adapter="moriarty-sdk-build"):
        sel = fx.selection_dict(self.outer, fingerprint or self.fp,
                                 adapter=adapter)
        self.sel_path.write_bytes(fx.encoded(sel))

    def runner(self, failures=None):
        return fx.outer_runner(self.outer, failures=failures)

    def go(self, runner, attempt="attempt-1"):
        return rs.run_selected_once("build", str(self.plan_path),
                                    str(self.sel_path),
                                    str(self.root / attempt), runner=runner,
            history_path=str(self.empty_history))

    def test_valid_dispatch_exactly_once_consumer_result_loads(self):
        runner = self.runner()
        code, record = self.go(runner)
        self.assertEqual(code, rs.EXIT_OK)
        self.assertEqual(runner.calls.count("native"), 1)
        self.assertEqual(record["terminal"], "succeeded")
        self.assertEqual(record["cleanupState"], "complete")
        result = json.loads(
            (self.root / "attempt-1/consumer-result.json").read_bytes())
        delivery.load_result(result)
        self.assertEqual(result["cleanup"], "complete")
        self.assertIsNone(result["review"])
        self.assertIsNone(result["adoption"])

    def test_missing_selection_zero_calls(self):
        self.sel_path.unlink()
        runner = self.runner()
        with self.assertRaises((rs.BadInput, rs.CollectError)):
            self.go(runner)
        self.assertEqual(runner.calls, [])

    def test_stale_fingerprint_zero_calls(self):
        self.write_selection(fingerprint="0" * 64)
        runner = self.runner()
        with self.assertRaises(rs.Blocked):
            self.go(runner)
        self.assertEqual(runner.calls, [])

    def test_guard_pin_mismatch_zero_calls(self):
        self.guard.write_bytes(b"# tampered\n")
        runner = self.runner()
        with self.assertRaises(rs.Blocked):
            self.go(runner)
        self.assertEqual(runner.calls, [])

    def test_selection_v1_never_selected(self):
        sel = json.loads(self.sel_path.read_bytes())
        sel["schema"] = "doctorwatson.execution-selection/1"
        del sel["outer"]
        self.sel_path.write_bytes(fx.encoded(sel))
        runner = self.runner()
        with self.assertRaises(rs.BadInput):
            self.go(runner)
        self.assertEqual(runner.calls, [])

    def test_task_cannot_supply_argv(self):
        plan = json.loads(self.plan_path.read_bytes())
        plan["tasks"][0]["argv"] = ["cargo"]
        self.plan_path.write_bytes(fx.encoded(plan))
        runner = self.runner()
        with self.assertRaises(rs.BadInput):
            self.go(runner)
        self.assertEqual(runner.calls, [])

    def test_selection_cannot_supply_argv_or_hooks(self):
        sel = json.loads(self.sel_path.read_bytes())
        sel["outer"]["argv"] = ["evil"]
        self.sel_path.write_bytes(fx.encoded(sel))
        runner = self.runner()
        with self.assertRaises(rs.BadInput):
            self.go(runner)
        self.assertEqual(runner.calls, [])

    def test_unmet_dependency_zero_calls(self):
        child = fx.task(self.root, "child", "moriarty-sdk-build",
                        deps=[{"task": "build", "stage": "observed"}])
        fx.plan_file(self.root, [self.task, child])
        with self.assertRaises(rs.Blocked):
            rs.resolve_task(str(self.plan_path), "child")

    def test_consumed_attempt_never_retried(self):
        (self.root / "attempt-1").mkdir()
        runner = self.runner()
        with self.assertRaises(rs.Blocked):
            self.go(runner)
        self.assertEqual(runner.calls, [])

    def test_unlaunchable_adapter_blocked(self):
        resolved = {"id": "build", "adapter": "sherlock-rollup-benchmark",
                    "fingerprint": self.fp}
        self.write_selection(adapter="sherlock-rollup-benchmark")
        with self.assertRaises(rs.Blocked):
            rs.resolve_selection(str(self.sel_path), resolved)

    def test_nonzero_exit_failed_one_call(self):
        runner = self.runner(failures={"native": 101})
        code, record = self.go(runner)
        self.assertEqual((code, record["terminal"]),
                         (rs.EXIT_FAILED, "failed"))
        self.assertEqual(runner.calls.count("native"), 1)

    def test_exit_zero_missing_outputs_failed(self):
        runner = fx.outer_runner(self.outer, write_outputs=False)
        code, _ = self.go(runner)
        self.assertEqual(code, rs.EXIT_FAILED)

    def test_runner_exception_unknown_and_consumed(self):
        runner = self.runner(failures={"native": TimeoutError("t")})
        code, record = self.go(runner)
        self.assertEqual(code, rs.EXIT_UNKNOWN)
        self.assertEqual(record["terminal"], "unknown-submission")
        self.assertEqual(record["unknownScope"], "local-process")
        self.assertNotEqual(record["cleanupState"], "complete-or-fake")
        runner2 = self.runner()
        with self.assertRaises(rs.Blocked):
            self.go(runner2)
        self.assertEqual(runner2.calls, [])

    def test_no_shell_one_subprocess_site(self):
        src = (Path(__file__).parent / "run_selected.py").read_text()
        self.assertNotIn("shell=True", src)
        self.assertEqual(src.count("subprocess.run"), 1)
        self.assertIn("reconciliation.load_readiness", src)
        self.assertIn("delivery.fingerprint", src)
        rec_src = (Path(__file__).parent / "reconciliation.py").read_text()
        self.assertIn("delivery.parse_plan_bytes", rec_src)
        self.assertIn("delivery.parse_history_bytes", rec_src)
        self.assertIn("parse_reconciliations_bytes", rec_src)
        self.assertIn("delivery.assess", rec_src)


class SettleDecoderTests(unittest.TestCase):
    def test_decoder_direct(self):
        import settle_observation as so
        value = fx.settle_result()
        raw = (json.dumps(value, separators=(",", ":")) + "\n").encode()
        out = so.decode_sherlock_settle_result(raw, fx.settle_inputs())
        self.assertTrue(out["accepted"] and out["settled"])
        value["post_observation"]["validity_size"] = 8
        raw = (json.dumps(value, separators=(",", ":")) + "\n").encode()
        with self.assertRaises(so.SettleDecodeError):
            so.decode_sherlock_settle_result(raw, fx.settle_inputs())
        with self.assertRaises(so.SettleDecodeError):
            so.decode_sherlock_settle_result(b"not json",
                                             fx.settle_inputs())
        old = fx.settle_result()
        del old["observation_schema"]
        raw = (json.dumps(old, separators=(",", ":")) + "\n").encode()
        with self.assertRaises(so.SettleDecodeError):
            so.decode_sherlock_settle_result(raw, fx.settle_inputs())


if __name__ == "__main__":
    unittest.main()
