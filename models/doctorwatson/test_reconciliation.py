"""Append-only reconciliation: applicable clears, wrong attempt cannot."""
import hashlib
import json
from pathlib import Path
import tempfile
import unittest

import delivery
import fixture_support as fx
import reconciliation as rec


class ReconciliationTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(dir=Path(__file__).parent)
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.task = fx.task(self.root, "build", "moriarty-sdk-build")
        self.plan_path = fx.plan_file(self.root, [self.task])
        self.plan = delivery.load_plan(self.plan_path)
        self.fp = delivery.fingerprint(self.plan, "build")

    def _unknown_result(self):
        artifacts = []
        for role in delivery.required_roles("moriarty-sdk-build"):
            raw = b"fixture\n"
            if role == "observation":
                raw = fx.encoded({
                    "schema": "doctorwatson.moriarty-sdk-build-observation/1",
                    "task": "build", "attempt": self.task["attempt"],
                    "fingerprint": self.fp, "built": True,
                    "test_bodies_executed": False})
            if role == "resource-receipt":
                raw = fx.encoded({
                    "schema": "doctorwatson.cleanup-observation/1",
                    "task": "build", "attempt": self.task["attempt"],
                    "fingerprint": self.fp, "cleanup": "complete",
                    "restoration": "complete"})
            path = fx.write(self.root / ("art-" + role), raw)
            artifacts.append({"role": role, **fx.pin(path)})
        return {"task": "build", "attempt": self.task["attempt"],
                "fingerprint": self.fp, "adapter": "moriarty-sdk-build",
                "terminal": "unknown-submission", "exit_code": None,
                "signal": None, "cleanup": "complete",
                "submission": "unknown", "artifacts": artifacts,
                "review": None, "adoption": None}

    def _record(self, original, attempt=None, evidence_name="evidence.json",
                resolution="process-not-started", outcome="not-started",
                exit_code=None, signal=None,
                evidence_token=None):
        digest = rec.result_digest(delivery.load_result(original))
        evidence = fx.write(self.root / evidence_name, fx.encoded({
            "schema": "doctorwatson.process-reconciliation-evidence/2",
            "task": "build", "attempt": self.task["attempt"],
            "fingerprint": self.fp, "preexec_sha256": "a" * 64,
            "original_result_sha256": evidence_token or digest,
            "process_outcome": outcome, "exit_code": exit_code,
            "signal": signal, "cleanup": "complete",
            "restoration": "complete"}))
        return {"schema": "doctorwatson.reconciliation/1", "task": "build",
                "attempt": attempt or self.task["attempt"],
                "fingerprint": self.fp, "adapter": "moriarty-sdk-build",
                "original_result_sha256": digest,
                "resolution": resolution,
                "evidence": [{"role": "process-evidence",
                              **fx.pin(evidence)}]}

    def _assess(self, original, records, new_attempt=True):
        if new_attempt:
            self.task["attempt"] = "build-002"
            # New plan attempt over unchanged operation identity: the
            # reconciled original is historical, history is preserved.
            fx.plan_file(self.root, [self.task])
            self.plan = delivery.load_plan(self.plan_path)
        results = (delivery.load_result(original),)
        return rec.assess_reconciled(self.plan, results, tuple(records))

    def test_unknown_without_reconciliation_blocks(self):
        report = self._assess(self._unknown_result(), [])
        self.assertEqual(report["tasks"][0]["state"], "unknown-reconcile")
        self.assertEqual(report["ready_for_root_selection"], [])

    def test_applicable_reconciliation_clears_only_its_debt(self):
        original = self._unknown_result()
        record = self._record(original)
        report = self._assess(original, [record])
        row = report["tasks"][0]
        self.assertEqual(row["state"], "ready")
        self.assertEqual(report["ready_for_root_selection"], ["build"])
        self.assertEqual(len(row["history"]), 1)
        self.assertEqual(row["history"][0]["terminal"],
                         "unknown-submission")
        self.assertEqual(row["reconciliations"][0]["resolution"],
                         "process-not-started")

    def test_wrong_attempt_cannot_clear(self):
        original = self._unknown_result()
        record = self._record(original, attempt="build-999")
        report = self._assess(original, [record])
        self.assertEqual(report["tasks"][0]["state"], "unknown-reconcile")

    def test_generic_clear_flag_is_not_a_record(self):
        doc = {"schema": "doctorwatson.reconciliations/1",
               "records": [{"schema": "doctorwatson.reconciliation/1",
                            "task": "build", "attempt": "build-001",
                            "fingerprint": self.fp,
                            "adapter": "moriarty-sdk-build",
                            "original_result_sha256": "a" * 64,
                            "resolution": "process-not-started",
                            "evidence": [], "clearDebt": True}]}
        path = self.root / "recs.json"
        path.write_bytes(fx.encoded(doc))
        with self.assertRaises(rec.ReconciliationError):
            rec.load_reconciliations(path)


    def test_consumed_same_fingerprint_retry_suppressed(self):
        original = self._unknown_result()
        record = self._record(original, resolution="process-exited",
                              outcome="exited", exit_code=101)
        report = self._assess(original, [record])  # new attempt, same fp
        row = report["tasks"][0]
        self.assertEqual(row["state"], "stale")
        self.assertFalse(row["selection_candidate"])
        self.assertEqual(report["ready_for_root_selection"], [])
        self.assertTrue(any("same-fingerprint retry suppressed" in b
                            for b in row["blockers"]))

    def test_changed_fingerprint_after_genuine_discharge_ready(self):
        original = self._unknown_result()
        record = self._record(original, resolution="process-exited",
                              outcome="exited", exit_code=101)
        self.task["attempt"] = "build-002"
        self.task["identity"]["oracle_sha256"] = "d" * 64  # genuine change
        fx.plan_file(self.root, [self.task])
        self.plan = delivery.load_plan(self.plan_path)
        self.assertNotEqual(delivery.fingerprint(self.plan, "build"),
                            self.fp)
        results = (delivery.load_result(original),)
        report = rec.assess_reconciled(self.plan, results, (record,))
        self.assertEqual(report["tasks"][0]["state"], "ready")
        self.assertEqual(report["ready_for_root_selection"], ["build"])

    def test_wrong_evidence_token_cannot_clear(self):
        original = self._unknown_result()
        record = self._record(original, evidence_token="0" * 64)
        report = self._assess(original, [record])
        row = report["tasks"][0]
        self.assertEqual(row["state"], "unknown-reconcile")
        self.assertIn("invalid", row["reconciliations"][0])

    def test_current_attempt_unresolved_never_cleared(self):
        original = self._unknown_result()
        record = self._record(original)
        report = self._assess(original, [record], new_attempt=False)
        self.assertEqual(report["ready_for_root_selection"], [])
        self.assertFalse(report["tasks"][0]["selection_candidate"])


class ReviewARegressionTests(unittest.TestCase):
    """D1/D2/D3 regressions from the rec04 Review-A probes (P1-P3)."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(dir=Path(__file__).parent)
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.task = fx.task(self.root, "build", "moriarty-sdk-build")
        self.plan_path = fx.plan_file(self.root, [self.task])
        self.plan = delivery.load_plan(self.plan_path)
        self.fp = delivery.fingerprint(self.plan, "build")

    def _result(self, attempt, terminal, exit_code, cleanup, submission,
                signal=None, artifacts="full"):
        arts = []
        if artifacts == "full":
            for role in delivery.required_roles("moriarty-sdk-build"):
                raw = b"fixture\n"
                if role == "observation":
                    raw = fx.encoded({
                        "schema": "doctorwatson.moriarty-sdk-build-"
                                  "observation/1",
                        "task": "build", "attempt": attempt,
                        "fingerprint": self.fp, "built": True,
                        "test_bodies_executed": False})
                if role == "resource-receipt":
                    raw = fx.encoded({
                        "schema": "doctorwatson.cleanup-observation/1",
                        "task": "build", "attempt": attempt,
                        "fingerprint": self.fp, "cleanup": "complete",
                        "restoration": "complete"})
                path = fx.write(self.root / (attempt + "-" + role), raw)
                arts.append({"role": role, **fx.pin(path)})
        return {"task": "build", "attempt": attempt,
                "fingerprint": self.fp, "adapter": "moriarty-sdk-build",
                "terminal": terminal, "exit_code": exit_code,
                "signal": signal, "cleanup": cleanup,
                "submission": submission, "artifacts": arts,
                "review": None, "adoption": None}

    def _record(self, original, resolution, outcome, exit_code, name,
                signal=None):
        digest = rec.result_digest(delivery.load_result(original))
        evidence = fx.write(self.root / name, fx.encoded({
            "schema": "doctorwatson.process-reconciliation-evidence/2",
            "task": "build", "attempt": original["attempt"],
            "fingerprint": self.fp, "preexec_sha256": "a" * 64,
            "original_result_sha256": digest,
            "process_outcome": outcome, "exit_code": exit_code,
            "signal": signal, "cleanup": "complete",
            "restoration": "complete"}))
        return {"schema": "doctorwatson.reconciliation/1",
                "task": "build", "attempt": original["attempt"],
                "fingerprint": self.fp, "adapter": "moriarty-sdk-build",
                "original_result_sha256": digest,
                "resolution": resolution,
                "evidence": [{"role": "process-evidence",
                              **fx.pin(evidence)}]}

    def _newer(self, changed=False, attempt="build-002"):
        task = dict(self.task)
        task["attempt"] = attempt
        if changed:
            task["identity"] = dict(task["identity"],
                                    oracle_sha256="d" * 64)
        fx.plan_file(self.root, [task])
        return delivery.load_plan(self.plan_path)

    def _fold(self, originals, records, changed=False):
        plan = self._newer(changed=changed)
        results = tuple(delivery.load_result(o) for o in originals)
        return rec.assess_reconciled(plan, results, tuple(records))

    def test_failed_original_not_started_cannot_free_same_fp(self):
        original = self._result("build-001", "failed", 101, "owed",
                                "not-applicable")
        record = self._record(original, "process-not-started",
                              "not-started", None, "ev-d1a.json")
        report = self._fold([original], [record])
        row = report["tasks"][0]
        self.assertNotEqual(row["state"], "ready")
        self.assertEqual(report["ready_for_root_selection"], [])
        self.assertIn("invalid", row["reconciliations"][0])

    def test_exit_mismatch_evidence_cannot_discharge(self):
        original = self._result("build-001", "failed", 3, "owed",
                                "not-applicable")
        record = self._record(original, "process-exited", "exited", 0,
                              "ev-d1b.json")
        report = self._fold([original], [record], changed=True)
        row = report["tasks"][0]
        self.assertNotEqual(row["state"], "ready")
        self.assertEqual(report["ready_for_root_selection"], [])
        self.assertIn("invalid", row["reconciliations"][0])

    def test_matching_recorded_exit_discharges_changed_fp(self):
        original = self._result("build-001", "failed", 3, "owed",
                                "not-applicable")
        record = self._record(original, "process-exited", "exited", 3,
                              "ev-d1c.json")
        report = self._fold([original], [record], changed=True)
        self.assertEqual(report["tasks"][0]["state"], "ready")
        self.assertEqual(report["ready_for_root_selection"], ["build"])

    def test_timed_out_signal_original_mismatches_refused(self):
        original = self._result("build-001", "timed-out", None, "owed",
                                "not-applicable", signal=9)
        not_started = self._record(original, "process-not-started",
                                   "not-started", None, "ev-d1d.json")
        report = self._fold([original], [not_started])
        self.assertEqual(report["ready_for_root_selection"], [])
        wrong_signal = self._record(original, "process-exited",
                                    "exited", None, "ev-d1e.json",
                                    signal=15)
        report = self._fold([original], [wrong_signal], changed=True)
        self.assertEqual(report["ready_for_root_selection"], [])
        self.assertIn("invalid",
                      report["tasks"][0]["reconciliations"][0])

    def test_refused_no_start_subject_still_allowed(self):
        original = self._result("build-001", "refused", None, "owed",
                                "not-submitted")
        record = self._record(original, "process-not-started",
                              "not-started", None, "ev-d1f.json")
        report = self._fold([original], [record])
        self.assertEqual(report["tasks"][0]["state"], "ready")
        self.assertEqual(report["ready_for_root_selection"], ["build"])

    def _sibling_debt_case(self):
        unknown = self._result("build-001", "unknown-submission", None,
                               "complete", "unknown")
        sibling = self._result("build-000", "failed", 1, "complete",
                               "not-applicable", artifacts="none")
        return unknown, sibling

    def test_sibling_receipt_debt_not_erased_by_one_record(self):
        unknown, sibling = self._sibling_debt_case()
        record = self._record(unknown, "process-exited", "exited", 101,
                              "ev-d2a.json")
        report = self._fold([unknown, sibling], [record], changed=True)
        row = report["tasks"][0]
        self.assertEqual(row["state"], "cleanup-debt")
        self.assertIn(rec.DEBT_BLOCKER, row["blockers"])
        self.assertEqual(report["ready_for_root_selection"], [])

    def test_sibling_receipt_debt_reconcilable_per_attempt(self):
        unknown, sibling = self._sibling_debt_case()
        records = [self._record(unknown, "process-exited", "exited", 101,
                                "ev-d2b.json"),
                   self._record(sibling, "process-exited", "exited", 1,
                                "ev-d2c.json")]
        report = self._fold([unknown, sibling], records, changed=True)
        self.assertEqual(report["tasks"][0]["state"], "ready")
        self.assertEqual(report["ready_for_root_selection"], ["build"])

    def test_pinned_bytes_are_parsed_not_transient_pathname_bytes(self):
        history = self.root / "history.json"
        history.write_bytes(fx.encoded(
            {"schema": "doctorwatson.delivery-history/1",
             "results": []}))
        original_bytes = self.plan_path.read_bytes()
        transient = fx.task(self.root, "build", "moriarty-sdk-build",
                            attempt="build-999")
        real_parse = delivery.parse_plan_bytes
        seen = []

        def swap_and_parse(raw):
            seen.append(raw)
            self.plan_path.write_bytes(fx.encoded(
                {"schema": "doctorwatson.delivery-plan/1",
                 "integrator": "Root", "tasks": [transient]}))
            try:
                return real_parse(raw)
            finally:
                self.plan_path.write_bytes(original_bytes)

        import unittest.mock as mock
        with mock.patch.object(delivery, "parse_plan_bytes",
                               swap_and_parse):
            readiness = rec.load_readiness(self.plan_path, history)
        self.assertEqual(seen, [original_bytes])
        self.assertEqual(readiness["plan"].by_id["build"].attempt,
                         "build-001")
        self.assertEqual(
            readiness["identity"]["plan"]["sha256"],
            hashlib.sha256(original_bytes).hexdigest())
        self.assertEqual(
            readiness["identity"]["plan"]["path"],
            str(self.plan_path.resolve()))

    def test_parse_bytes_entrypoints_are_strict(self):
        plan = delivery.parse_plan_bytes(self.plan_path.read_bytes())
        self.assertEqual(delivery.fingerprint(plan, "build"), self.fp)
        self.assertEqual(delivery.parse_history_bytes(fx.encoded(
            {"schema": "doctorwatson.delivery-history/1",
             "results": []})), ())
        self.assertEqual(rec.parse_reconciliations_bytes(fx.encoded(
            {"schema": "doctorwatson.reconciliations/1",
             "records": []})), ())
        with self.assertRaises(delivery.SchemaError):
            delivery.parse_plan_bytes(b'{"schema": "doctorwatson.del')
        with self.assertRaises(rec.ReconciliationError):
            rec.parse_reconciliations_bytes(b'{"schema": "x"}')


if __name__ == "__main__":
    unittest.main()
