"""Pure/mock tests for run_selected; injected runners only."""
import hashlib
import json
from pathlib import Path
import shutil
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
        self.task = fx.task(self.root, "build", "moriarty-sdk-build")
        self.plan_path = fx.plan_file(self.root, [self.task])
        self.fp = delivery.fingerprint(
            delivery.load_plan(self.plan_path), "build")
        self.outputs = fx.guard_packet(self.root)
        shutil.rmtree(self.root / "packet")
        shutil.rmtree(self.root / "target")
        self.guard = fx.write(self.root / "guard.py", b"# guard\n")
        self.config = fx.write(self.root / "cfg.json", b"{}\n")
        self.sel_path = self.root / "selection.json"
        self.write_selection()

    def write_selection(self, fingerprint=None, outputs=None,
                        adapter="moriarty-sdk-build"):
        sel = {"schema": "doctorwatson.execution-selection/1",
               "task": "build", "adapter": adapter,
               "fingerprint": fingerprint or self.fp,
               "outerSequence": "bounded-sdk-compile-cached-current-f/1",
               "guard": fx.pin(self.guard), "config": fx.pin(self.config),
               "outputs": outputs if outputs is not None else self.outputs}
        self.sel_path.write_bytes(fx.encoded(sel))

    def runner(self, code=0, exc=None):
        calls = []

        def run(argv):
            calls.append(list(argv))
            if exc is not None:
                raise exc
            fx.guard_packet(self.root)
            fx.cleanup_receipt(self.root / "packet", "cleanup.json",
                               "build", self.task["attempt"], self.fp)
            return code
        run.calls = calls
        return run

    def go(self, runner, attempt="attempt-1"):
        return rs.run_selected_once("build", str(self.plan_path),
                                    str(self.sel_path),
                                    str(self.root / attempt), runner=runner)

    def test_valid_dispatch_exactly_once_consumer_result_loads(self):
        runner = self.runner()
        code, record = self.go(runner)
        self.assertEqual(code, rs.EXIT_OK)
        self.assertEqual(len(runner.calls), 1)
        self.assertEqual(record["terminal"], "succeeded")
        result = json.loads(
            (self.root / "attempt-1/consumer-result.json").read_bytes())
        delivery.load_result(result)

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

    def test_task_cannot_supply_argv(self):
        plan = json.loads(self.plan_path.read_bytes())
        plan["tasks"][0]["argv"] = ["cargo"]
        self.plan_path.write_bytes(fx.encoded(plan))
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
        runner = self.runner(code=101)
        code, record = self.go(runner)
        self.assertEqual((code, record["terminal"]),
                         (rs.EXIT_FAILED, "failed"))

    def test_exit_zero_missing_outputs_failed(self):
        def run(argv):
            return 0
        code, _ = self.go(run)
        self.assertEqual(code, rs.EXIT_FAILED)

    def test_runner_exception_unknown_and_consumed(self):
        runner = self.runner(exc=TimeoutError("t"))
        code, record = self.go(runner)
        self.assertEqual(code, rs.EXIT_UNKNOWN)
        self.assertEqual(record["terminal"], "unknown-submission")
        self.assertEqual(record["unknownScope"], "local-process")
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
