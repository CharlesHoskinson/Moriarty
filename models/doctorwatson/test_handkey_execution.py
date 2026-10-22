"""Actual harmless subprocess protocol tests; no native verifier or proof claim."""
import json
import os
from pathlib import Path
import sys
import tempfile
import unittest
from unittest import mock

import delivery
import fixture_support as fx
import run_selected as rs


class HandkeyExecutionTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.output = self.root / "output"
        self.output.mkdir()
        self.history = fx.write(self.root / "history.json", fx.encoded({
            "schema": "doctorwatson.delivery-history/1", "results": []}))
        self.tool = self.root / "fake-handkey"
        self.key = fx.write(self.root / "key", b"fake key")
        self.proof = fx.write(self.root / "proof", b"fake tagged proof")
        self.statement = fx.write(self.root / "pis.json", b'{"pis":[]}')
        self.descriptor = fx.write(self.root / "output-root.json", fx.encoded({
            "schema": "doctorwatson.handkey-output-root/1", "path": str(self.output)}))
        self.marker = self.root / "launched"
        self.configure()

    def configure(self, body='print(\'{"verifies":true,"verify_s":0.01}\')'):
        self.tool.write_text(f"#!{sys.executable}\nimport json, os, pathlib, sys, time\n"
                             f"pathlib.Path({str(self.marker)!r}).write_text(json.dumps(sys.argv))\n"
                             + body + "\n")
        self.tool.chmod(0o700)
        pins = {name: fx.pin(getattr(self, name)) for name in ("key", "proof", "statement")}
        self.task = fx.task(self.root, "verify", "handkey-verify-pis", inputs={
            name + "_sha256": pin["sha256"] for name, pin in pins.items()})
        self.task["identity"]["source"] += list(pins.values())
        self.task["identity"]["tools"].append(fx.pin(self.tool))
        self.task["identity"]["config"].append(fx.pin(self.descriptor))
        self.plan = fx.plan_file(self.root, [self.task])
        self.fp = delivery.fingerprint(delivery.load_plan(self.plan), "verify")
        self.selection_value = {"schema": rs.HANDKEY_SELECTION_SCHEMA,
                                "task": "verify", "adapter": "handkey-verify-pis",
                                "attempt": self.task["attempt"], "fingerprint": self.fp,
                                "tool": fx.pin(self.tool), "outputRoot": fx.pin(self.descriptor), **pins}
        self.selection = fx.write(self.root / "selection.json", fx.encoded(self.selection_value))

    def go(self, name="attempt"):
        return rs.run_selected_once("verify", str(self.plan), str(self.selection),
                                    str(self.output / name), history_path=str(self.history))

    def test_real_subprocess_fixed_argv_and_consumer_roundtrip(self):
        code, record = self.go()
        self.assertEqual(code, rs.EXIT_OK, record)
        self.assertEqual(json.loads(self.marker.read_text()), [str(self.tool), "verify-pis",
                         str(self.key), str(self.proof), str(self.statement)])
        self.assertEqual(record["cleanupState"], "complete")
        observed = record["observation"]
        self.assertTrue(observed["subreaperDrainECHILD"])
        self.assertTrue(observed["groupAbsentObserved"])
        self.assertGreaterEqual(observed["usage"]["userSeconds"], 0)
        result = json.loads((self.output / "attempt/consumer-result.json").read_text())
        loaded = delivery.load_result(result)
        report = delivery.assess(delivery.load_plan(self.plan), [loaded])
        self.assertEqual(report["tasks"][0]["state"], "observed")
        self.assertFalse(report["tasks"][0]["acceptance_recorded"])
        preexec = json.loads((self.output / "attempt/PREEXEC.json").read_text())
        self.assertEqual(preexec["stage"]["profile"], rs.HANDKEY_PROFILE)
        self.assertEqual(preexec["stage"]["env"], {"PATH": "/usr/bin:/bin", "LANG": "C", "LC_ALL": "C"})

    def test_false_empty_extra_malformed_nonfinite_and_nonzero_fail(self):
        bodies = ['print(\'{"verifies":false,"verify_s":0.01}\')', 'pass',
                  'print(\'{"verifies":true,"verify_s":0.01,"extra":1}\')',
                  'print("not JSON")', 'print(\'{"verifies":true,"verify_s":NaN}\')',
                  'print(\'{"verifies":true,"verify_s":0.01}\'); sys.exit(7)']
        for index, body in enumerate(bodies):
            with self.subTest(body=body):
                self.configure(body)
                # Separate authoritative history namespace for independent cases.
                self.history = fx.write(self.root / f"history-{index}.json", fx.encoded({
                    "schema": "doctorwatson.delivery-history/1", "results": []}))
                code, record = self.go(f"attempt-{index}")
                self.assertEqual(code, rs.EXIT_FAILED, record)
                self.assertNotEqual(record["terminal"], "succeeded")

    def test_wrong_task_attempt_fingerprint_tuple_and_unbound_tool_zero_child(self):
        variants = [("task", "wrong"), ("attempt", "old"), ("fingerprint", "0" * 64),
                    ("proof", fx.pin(self.key)), ("tool", fx.pin(self.key))]
        for field, value in variants:
            with self.subTest(field=field):
                sel = {**self.selection_value, field: value}
                self.selection.write_bytes(fx.encoded(sel))
                with self.assertRaises((rs.Blocked, rs.BadInput)):
                    self.go()
                self.assertFalse(self.marker.exists())
        self.assertFalse((self.output / "attempt").exists())

    def test_arbitrary_fields_refused(self):
        self.selection_value["argv"] = ["evil"]
        self.selection.write_bytes(fx.encoded(self.selection_value))
        with self.assertRaises(rs.BadInput):
            self.go()
        self.assertFalse(self.marker.exists())

    def test_changed_input_before_use_zero_child(self):
        original = rs._recheck_at_use
        def changed(*args):
            self.proof.write_bytes(b"changed")
            return original(*args)
        with mock.patch.object(rs, "_recheck_at_use", side_effect=changed):
            with self.assertRaises(rs.Blocked):
                self.go()
        self.assertFalse(self.marker.exists())

    def test_changed_public_input_during_child_fails(self):
        self.configure(f"pathlib.Path({str(self.proof)!r}).write_bytes(b'changed')\n"
                       'print(\'{"verifies":true,"verify_s":0.01}\')')
        code, record = self.go()
        self.assertEqual(code, rs.EXIT_FAILED, record)
        self.assertIn("pin mismatch", record["collectorDetail"])

    def test_duplicate_attempt_even_different_output_directory_zero_second_child(self):
        self.assertEqual(self.go()[0], rs.EXIT_OK)
        self.marker.unlink()
        with self.assertRaises(rs.Blocked):
            self.go("different-attempt-directory")
        self.assertFalse(self.marker.exists())

    def test_setsid_grandchild_is_reaped_before_complete(self):
        self.configure('''pid = os.fork()
if pid == 0:
    os.setsid()
    os.close(1)
    os.close(2)
    time.sleep(15)
    os._exit(0)
print('{"verifies":true,"verify_s":0.01}')''')
        code, record = self.go()
        self.assertEqual(code, rs.EXIT_OK, record)
        self.assertTrue(record["observation"]["subreaperDrainECHILD"])
        self.assertGreaterEqual(len(record["observation"]["adoptedChildren"]), 1)
        for child in record["observation"]["adoptedChildren"]:
            self.assertFalse(Path(f'/proc/{child["pid"]}').exists())

    def test_timeout_drain_observed_terminal_failed_no_retry(self):
        self.configure('time.sleep(10)')
        with mock.patch.dict(rs.HANDKEY_PROFILE, wallSeconds=0.05):
            code, record = self.go()
        self.assertEqual(code, rs.EXIT_FAILED, record)
        self.assertEqual(record["observation"]["reason"], "wall-timeout")
        self.assertTrue(record["observation"]["subreaperDrainECHILD"])
        self.assertEqual(record["cleanupState"], "complete")

    def test_unproved_drain_remains_unknown(self):
        observed = {"returncode": 0, "reason": "unproved drain", "cleanup": "unknown"}
        with mock.patch.object(rs, "_capture_handkey", return_value=observed):
            code, record = self.go()
        self.assertEqual(code, rs.EXIT_UNKNOWN, record)
        self.assertFalse((self.output / "attempt/consumer-result.json").exists())

    def test_prctl_failure_before_actual_workload(self):
        with mock.patch.object(rs, "_enable_handkey_subreaper", side_effect=OSError("fake prctl failure")):
            code, record = self.go()
        self.assertEqual(code, rs.EXIT_UNKNOWN, record)
        self.assertFalse(self.marker.exists())
        self.assertIn("prctl failure", record["observation"]["reason"])

    def test_stdout_limit_records_actual_overflow_and_fails(self):
        self.configure("sys.stdout.write('x' * (4 * 1024**2 + 1))")
        code, record = self.go()
        self.assertEqual(code, rs.EXIT_FAILED, record)
        self.assertEqual(record["observation"]["reason"], "stream-limit")
        self.assertEqual((self.output / "attempt/stdout").stat().st_size, 4 * 1024**2)

    def test_output_root_bytes_actually_parsed_must_match_pin(self):
        original = rs._read_raw
        def swapped(path, what, *args):
            if what == "handkey output root":
                return fx.encoded({"schema": "doctorwatson.handkey-output-root/1", "path": "/tmp"})
            return original(path, what, *args)
        with mock.patch.object(rs, "_read_raw", side_effect=swapped):
            with self.assertRaisesRegex(rs.Blocked, "changed before parsing"):
                self.go()
        self.assertFalse(self.marker.exists())

    def test_output_open_failure_happens_before_workload(self):
        original = rs._capture_handkey
        def occupied(stage):
            Path(stage["stdout"]).write_bytes(b"occupied")
            return original(stage)
        with mock.patch.object(rs, "_capture_handkey", side_effect=occupied):
            code, record = self.go()
        self.assertEqual(code, rs.EXIT_UNKNOWN, record)
        self.assertFalse(self.marker.exists())
        self.assertIn("not launched", record["observation"]["reason"])

    def test_wait4_retries_eintr_without_inventing_terminal(self):
        with mock.patch.object(rs.os, "wait4", side_effect=[InterruptedError(), (123, 0, None)]) as wait:
            self.assertEqual(rs._wait4_retry(-1, 0x40000001), (123, 0, None))
        self.assertEqual(wait.call_count, 2)


if __name__ == "__main__":
    unittest.main()
