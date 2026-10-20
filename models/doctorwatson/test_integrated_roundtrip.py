"""One actual integrated ordinary fixture per adapter family.

Plan is created before outputs exist and is never mutated; the exact
consumer fingerprint flows through selection -> mock existing outer
guard boundary -> producer-shaped raw files -> fixed collector ->
consumer Result -> delivery.assess readiness. No manual fingerprint
patching and no synthetic observation producer: normalized observations
are derived by the collectors from raw files only.
"""
import hashlib
import json
from pathlib import Path
import tempfile
import unittest

import delivery
import fixture_support as fx
import run_selected as rs


def raw_json(value):
    # Producer JSON preserves cell insertion order (no sort).
    return (json.dumps(value, separators=(",", ":"), allow_nan=False)
            + "\n").encode()


class IntegratedCompileTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(dir=Path(__file__).parent)
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)

    def _setup_plan_selection(self, disposition="active"):
        root = self.root
        task = fx.task(root, "build", "moriarty-sdk-build",
                       disposition=disposition)
        plan_path = fx.plan_file(root, [task])
        plan = delivery.load_plan(plan_path)
        fp = delivery.fingerprint(plan, "build")
        outputs = fx.guard_packet(root)  # paths declared; packet removed below
        # Plan and selection are fixed before outputs exist: remove the
        # packet/target trees, keep only their declared paths.
        import shutil
        shutil.rmtree(root / "packet")
        shutil.rmtree(root / "target")
        guard = fx.write(root / "bounded_sdk_compile.py",
                         b"# fixture selected guard\n")
        config = fx.write(root / "FINAL.json", b"{}\n")
        sel = {"schema": "doctorwatson.execution-selection/1",
               "task": "build", "adapter": "moriarty-sdk-build",
               "fingerprint": fp,
               "outerSequence": "bounded-sdk-compile-cached-current-f/1",
               "guard": fx.pin(guard), "config": fx.pin(config),
               "outputs": outputs}
        sel_path = root / "selection.json"
        sel_path.write_bytes(fx.encoded(sel))
        return task, plan_path, sel_path, fp, outputs

    def _mock_guard(self, task, fp, code=0):
        root = self.root
        calls = []

        def runner(argv):
            calls.append(list(argv))
            fx.guard_packet(root)
            fx.cleanup_receipt(root / "packet", "cleanup.json",
                               task["id"], task["attempt"], fp)
            return code
        runner.calls = calls
        return runner

    def test_roundtrip_plan_to_readiness(self):
        task, plan_path, sel_path, fp, outputs = self._setup_plan_selection()
        self.assertFalse(Path(outputs[0]["path"]).exists())
        before = plan_path.read_bytes()
        attempt = self.root / "attempt-1"
        runner = self._mock_guard(task, fp)
        code, record = rs.run_selected_once(
            "build", str(plan_path), str(sel_path), str(attempt),
            runner=runner)
        self.assertEqual(code, rs.EXIT_OK)
        self.assertEqual(len(runner.calls), 1)
        self.assertIn("--cached-current-f", runner.calls[0])
        self.assertEqual(plan_path.read_bytes(), before)
        preexec = json.loads((attempt / "PREEXEC.json").read_bytes())
        self.assertEqual(preexec["fingerprint"], fp)
        result = json.loads((attempt / "consumer-result.json").read_bytes())
        loaded = delivery.load_result(result)
        self.assertEqual(loaded.fingerprint, fp)
        history = self.root / "history.json"
        history.write_bytes(fx.encoded(
            {"schema": "doctorwatson.delivery-history/1",
             "results": [result]}))
        report = delivery.inspect_delivery(plan_path, history)
        row = report["tasks"][0]
        self.assertEqual(row["state"], "observed")
        self.assertEqual(row["observation"]["built"], True)
        self.assertEqual(row["observation"]["test_bodies_executed"], False)
        sidecar = json.loads((attempt / "collector.json").read_bytes())
        self.assertEqual(sidecar["rawReceipt"]["sha256"],
                         hashlib.sha256(
                             (attempt / "result.json").read_bytes()
                         ).hexdigest())

    def test_bridge_and_consumer_fingerprint_identical_and_drift_invalidates(self):
        task, plan_path, sel_path, fp, _ = self._setup_plan_selection()
        resolved = rs.resolve_task(str(plan_path), "build")
        self.assertEqual(resolved["fingerprint"], fp)
        # Input drift after selection: consumer fingerprint changes and
        # the stale selection blocks with zero calls.
        plan = json.loads(plan_path.read_bytes())
        plan["tasks"][0]["identity"]["oracle_sha256"] = "d" * 64
        plan_path.write_bytes(fx.encoded(plan))
        runner = self._mock_guard(task, fp)
        with self.assertRaises(rs.Blocked):
            rs.run_selected_once("build", str(plan_path), str(sel_path),
                                 str(self.root / "attempt-2"), runner=runner)
        self.assertEqual(runner.calls, [])

    def test_blocked_disposition_zero_calls(self):
        task, plan_path, sel_path, fp, _ = self._setup_plan_selection(
            disposition="blocked")
        runner = self._mock_guard(task, fp)
        with self.assertRaises(rs.Blocked):
            rs.run_selected_once("build", str(plan_path), str(sel_path),
                                 str(self.root / "a"), runner=runner)
        self.assertEqual(runner.calls, [])

    def test_exit_zero_with_tampered_f_outputs_is_failed(self):
        task, plan_path, sel_path, fp, _ = self._setup_plan_selection()
        root = self.root

        def runner(argv):
            fx.guard_packet(root)
            fx.cleanup_receipt(root / "packet", "cleanup.json",
                               task["id"], task["attempt"], fp)
            elf = root / "target/debug/deps/mori_backend-deadbeef01"
            elf.write_bytes(b"tampered elf\n")
            return 0
        code, record = rs.run_selected_once(
            "build", str(plan_path), str(sel_path),
            str(root / "attempt-x"), runner=runner)
        self.assertEqual(code, rs.EXIT_FAILED)
        self.assertFalse((root / "attempt-x/consumer-result.json").exists())

    def test_signal_maps_to_consumer_signal_field(self):
        self.assertEqual(rs._terminal_fields(-9), ("failed", None, 9))
        self.assertEqual(rs._terminal_fields(101), ("failed", 101, None))


class IntegratedSettleTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(dir=Path(__file__).parent)
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.inputs = fx.settle_inputs()
        self.task = fx.task(self.root, "settle",
                            "sherlock-anchor-settle-readback",
                            inputs=self.inputs)
        self.plan_path = fx.plan_file(self.root, [self.task])
        self.plan = delivery.load_plan(self.plan_path)
        self.fp = delivery.fingerprint(self.plan, "settle")
        self.resolved = {"id": "settle",
                         "adapter": "sherlock-anchor-settle-readback",
                         "attempt": self.task["attempt"],
                         "fingerprint": self.fp, "inputs": self.inputs}

    def _raw(self, settle_value=None, post_bytes=b"post state bytes\n"):
        root = self.root
        value = settle_value or fx.settle_result()
        value["post_state_sha256"] = hashlib.sha256(post_bytes).hexdigest()
        source = fx.write(root / "settle-result-public.json", raw_json(value))
        post = fx.write(root / "post-state.bin", post_bytes)
        fx.cleanup_receipt(root, "cleanup.json", "settle",
                           self.task["attempt"], self.fp)
        return [{"role": "stdout", "path": str(fx.write(root / "out", b"ok\n"))},
                {"role": "stderr", "path": str(fx.write(root / "err", b""))},
                {"role": "resource-receipt",
                 "path": str(root / "cleanup.json")},
                {"role": "source-result", "path": str(source)},
                {"role": "post-state", "path": str(post)}]

    def test_settle_roundtrip_to_observed(self):
        attempt = self.root / "attempt"
        attempt.mkdir()
        result = rs.collect_result(
            self.resolved, self._raw(), 0, attempt,
            {"path": "raw", "bytes": 0, "sha256": "0" * 64})
        history = self.root / "history.json"
        history.write_bytes(fx.encoded(
            {"schema": "doctorwatson.delivery-history/1",
             "results": [result]}))
        row = delivery.inspect_delivery(self.plan_path, history)["tasks"][0]
        self.assertEqual(row["state"], "observed")
        self.assertEqual(row["observation"]["expected_root"],
                         self.inputs["expected_root"])

    def test_wrong_root_and_non_json_fail_collection(self):
        for name, mutate in (
                ("wrong-root", lambda v: v.update(
                    expected_root=str(fx.HUGE + 123456))),
                ("bad-baseline", lambda v: v["settle_assertion_baseline"]
                 ["observation"].update(validity_size=8))):
            with self.subTest(name=name):
                value = fx.settle_result()
                mutate(value)
                attempt = self.root / ("att-" + name)
                attempt.mkdir()
                with self.assertRaises(rs.CollectError):
                    rs.collect_result(self.resolved, self._raw(value), 0,
                                      attempt, {"path": "r", "bytes": 0,
                                                "sha256": "0" * 64})
        attempt = self.root / "att-nonjson"
        attempt.mkdir()
        raw = self._raw()
        Path(raw[3]["path"]).write_bytes(b"not JSON and no observations")
        with self.assertRaises(rs.CollectError):
            rs.collect_result(self.resolved, raw, 0, attempt,
                              {"path": "r", "bytes": 0, "sha256": "0" * 64})


class IntegratedHandkeyTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(dir=Path(__file__).parent)
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        blob = b"public opaque bytes\n"
        sha = hashlib.sha256(blob).hexdigest()
        self.inputs = {k: sha for k in
                       ("key_sha256", "proof_sha256", "statement_sha256")}
        self.task = fx.task(self.root, "verify", "handkey-verify-pis",
                            inputs=self.inputs)
        self.plan_path = fx.plan_file(self.root, [self.task])
        self.fp = delivery.fingerprint(delivery.load_plan(self.plan_path),
                                        "verify")
        self.resolved = {"id": "verify", "adapter": "handkey-verify-pis",
                         "attempt": self.task["attempt"],
                         "fingerprint": self.fp, "inputs": self.inputs}
        self.blob = blob

    def _raw(self, verifies):
        root = self.root
        fx.cleanup_receipt(root, "cleanup.json", "verify",
                           self.task["attempt"], self.fp)
        out = fx.write(root / "stdout", fx.encoded(
            {"verifies": verifies, "verify_s": 0.25}))
        roles = [{"role": "stdout", "path": str(out)},
                 {"role": "stderr", "path": str(fx.write(root / "err", b""))},
                 {"role": "resource-receipt",
                  "path": str(root / "cleanup.json")}]
        for role in ("key", "proof", "statement"):
            roles.append({"role": role,
                          "path": str(fx.write(root / role, self.blob))})
        return roles

    def _state(self, verifies, name):
        attempt = self.root / name
        attempt.mkdir()
        result = rs.collect_result(self.resolved, self._raw(verifies), 0,
                                   attempt, {"path": "r", "bytes": 0,
                                             "sha256": "0" * 64})
        history = self.root / ("history-" + name + ".json")
        history.write_bytes(fx.encoded(
            {"schema": "doctorwatson.delivery-history/1",
             "results": [result]}))
        return delivery.inspect_delivery(self.plan_path, history)["tasks"][0]

    def test_true_stdout_observed_false_stdout_failed(self):
        self.assertEqual(self._state(True, "att-true")["state"], "observed")
        row = self._state(False, "att-false")
        self.assertNotEqual(row["state"], "observed")


if __name__ == "__main__":
    unittest.main()
