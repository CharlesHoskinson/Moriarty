"""The executor requires complete authoritative history; inspect remains pure."""
import json
from pathlib import Path
import unittest
from unittest import mock

import delivery
import fixture_support as fx
import run_selected as rs
import test_handkey_execution as handkey


class ExecutionHistoryRequiredTests(unittest.TestCase):
    def setUp(self):
        self.case = handkey.HandkeyExecutionTests()
        self.case.setUp()
        self.addCleanup(self.case.doCleanups)
        self.root = self.case.root

    def test_missing_history_api_zero_child_and_zero_reservation(self):
        with mock.patch.object(rs, "_capture_handkey") as capture:
            with self.assertRaisesRegex(rs.BadInput, "authoritative complete --history"):
                rs.run_selected_once("verify", str(self.case.plan), str(self.case.selection),
                                      str(self.case.output / "attempt"))
        capture.assert_not_called()
        self.assertFalse(self.case.marker.exists())
        self.assertEqual(list(self.case.output.iterdir()), [])

    def test_missing_history_cli_zero_child(self):
        with mock.patch.object(rs, "_capture_handkey") as capture:
            code = rs.main(["--execute", "verify", "--plan", str(self.case.plan),
                            "--selection", str(self.case.selection), "--attempt",
                            str(self.case.output / "attempt")])
        self.assertEqual(code, rs.EXIT_BAD_INPUT)
        capture.assert_not_called()
        self.assertFalse(self.case.marker.exists())

    def test_resolve_inspection_can_explicitly_use_empty_history(self):
        task = rs.resolve_task(str(self.case.plan), "verify", str(self.case.history))
        self.assertTrue(task["readiness"]["selection_candidate"])
        self.assertEqual(list(self.case.output.iterdir()), [])

    def newer_attempt(self):
        self.case.task["attempt"] = "verify-002"
        self.case.task["identity"]["oracle_sha256"] = "d" * 64
        self.case.plan.write_bytes(fx.encoded({"schema": "doctorwatson.delivery-plan/1",
            "integrator": "Integrator", "tasks": [self.case.task]}))
        self.case.selection_value["attempt"] = "verify-002"
        self.case.selection_value["fingerprint"] = delivery.fingerprint(
            delivery.load_plan(self.case.plan), "verify")
        self.case.selection.write_bytes(fx.encoded(self.case.selection_value))

    def test_omitted_original_result_blocks_new_attempt_even_changed_source(self):
        self.assertEqual(self.case.go()[0], rs.EXIT_OK)
        self.newer_attempt()
        self.case.marker.unlink()
        with self.assertRaisesRegex(rs.Blocked, "omitted or changed"):
            self.case.go("attempt2")
        self.assertFalse(self.case.marker.exists())

    def test_retained_exact_success_allows_selected_changed_attempt(self):
        code, record = self.case.go()
        self.assertEqual(code, rs.EXIT_OK)
        result = json.loads(Path(record["historyResult"]["path"]).read_text())
        self.case.history.write_bytes(fx.encoded({"schema": "doctorwatson.delivery-history/1", "results": [result]}))
        self.newer_attempt()
        code, record = self.case.go("attempt2")
        self.assertEqual(code, rs.EXIT_OK, record)

    def test_sibling_unknown_debt_blocks_new_attempt(self):
        with mock.patch.object(rs, "_capture_handkey", return_value={
                "returncode": None, "cleanup": "unknown", "reason": "uncertain descendants"}):
            code, record = self.case.go()
        self.assertEqual(code, rs.EXIT_UNKNOWN)
        original = json.loads(Path(record["historyResult"]["path"]).read_text())
        self.case.history.write_bytes(fx.encoded({"schema": "doctorwatson.delivery-history/1", "results": [original]}))
        self.newer_attempt()
        with mock.patch.object(rs, "_capture_handkey") as capture:
            with self.assertRaisesRegex(rs.Blocked, "unknown-reconcile"):
                self.case.go("attempt2")
        capture.assert_not_called()

    def test_crash_after_preexec_reservation_blocks_new_attempt(self):
        with mock.patch.object(rs, "_capture_handkey", side_effect=KeyboardInterrupt):
            with self.assertRaises(KeyboardInterrupt):
                self.case.go()
        self.newer_attempt()
        with self.assertRaisesRegex(rs.Blocked, "prior reservation incomplete"):
            self.case.go("attempt2")
        self.assertFalse(self.case.marker.exists())

    def test_history_result_write_failure_returns_unknown_and_preserves_reservation(self):
        original = rs._write_handkey_json
        def failing(path, value):
            if path.name == "history-result.json":
                raise OSError("fake terminal write failure")
            return original(path, value)
        with mock.patch.object(rs, "_write_handkey_json", side_effect=failing):
            code, record = self.case.go()
        self.assertEqual(code, rs.EXIT_UNKNOWN, record)
        self.assertEqual(record["unknownScope"], "local-record")
        self.assertEqual(record["observedCollectorTerminal"], "succeeded")
        self.assertEqual(record["cleanupState"], "complete")
        self.assertTrue(self.case.marker.exists())
        self.assertTrue((self.case.output / "attempt/PREEXEC.json").is_file())
        self.assertTrue((self.case.output / "attempt/launched.json").is_file())
        self.case.marker.unlink()
        with self.assertRaises(rs.Blocked):
            self.case.go("different-directory")
        self.assertFalse(self.case.marker.exists())

    def test_final_record_fsync_failure_retains_actual_unknown_debt(self):
        original = rs._fsync_dir
        def failing(path):
            if (path / "result-final.json").exists():
                raise OSError("fake terminal directory fsync failure")
            return original(path)
        with mock.patch.object(rs, "_capture_handkey", return_value={
                "returncode": None, "cleanup": "unknown", "reason": "unproved drain"}), \
                mock.patch.object(rs, "_fsync_dir", side_effect=failing):
            code, record = self.case.go()
        self.assertEqual(code, rs.EXIT_UNKNOWN, record)
        self.assertEqual(record["cleanupState"], "unknown")
        self.assertEqual(record["terminal"], "unknown-submission")
        self.assertIn("fsync failure", record["persistenceDetail"])
        self.assertFalse(self.case.marker.exists())

    def test_failed_verifier_retains_observed_cleanup_for_changed_attempt(self):
        self.case.configure("pass")
        code, record = self.case.go()
        self.assertEqual(code, rs.EXIT_FAILED, record)
        self.assertEqual(record["cleanupState"], "complete")
        result = json.loads(Path(record["historyResult"]["path"]).read_text())
        self.assertEqual([ref["role"] for ref in result["artifacts"]], ["resource-receipt"])
        self.case.history.write_bytes(fx.encoded({"schema": "doctorwatson.delivery-history/1", "results": [result]}))
        self.newer_attempt()
        self.case.marker.unlink()
        # This is a separately selected changed attempt, never automatic retry.
        # Its empty-output operation fails, but original complete cleanup adds no debt.
        code, record = self.case.go("attempt2")
        self.assertEqual(code, rs.EXIT_FAILED, record)
        self.assertTrue(self.case.marker.exists())
        self.assertEqual(record["cleanupState"], "complete")

    def test_reservation_uses_exact_history_already_parsed_by_common_readiness(self):
        code, record = self.case.go()
        self.assertEqual(code, rs.EXIT_OK)
        old = delivery.load_result(json.loads(Path(record["historyResult"]["path"]).read_text()))
        self.newer_attempt()
        self.case.marker.unlink()
        # A transient, different pathname re-read must not satisfy the journal
        # when the actual digest-pinned readiness history omits the original.
        with mock.patch.object(delivery, "load_history", return_value=[old]) as reread:
            with self.assertRaisesRegex(rs.Blocked, "omitted or changed"):
                self.case.go("attempt2")
        reread.assert_not_called()
        self.assertFalse(self.case.marker.exists())


if __name__ == "__main__":
    unittest.main()
