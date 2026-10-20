"""True CLI/executor roundtrip for reconciled readiness (mock guard).

Drives the real run_selected.py CLI (and the shell --execute/--inspect
routes) through a mock of the existing bounded-sdk-compile guard
boundary. Unknown history blocks; correct original terminal + cleanup
evidence enables a separately selected qualifying newer attempt; every
wrong-token/digest/cleanup/fingerprint/cancelled/tampered variant
leaves zero dispatch (the mock guard's dispatch counter never moves).
Fingerprints come only from delivery.fingerprint; no accepted booleans
are supplied anywhere.
"""
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest
from unittest import mock

import delivery
import fixture_support as fx
import reconciliation as rec
import run_selected as rs

MODELS = Path(__file__).resolve().parent
REPO = MODELS.parents[1]


class ExecutionReconciliationCliTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(dir=MODELS)
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name).resolve()
        self.task1 = fx.task(self.root, "build", "moriarty-sdk-build")
        self.plan_path = fx.plan_file(self.root, [self.task1])
        self.fp1 = delivery.fingerprint(
            delivery.load_plan(self.plan_path), "build")
        self.original = self._unknown_result()
        self.history_path = self.root / "history.json"
        self._write_history(self.original)
        self.outputs = fx.guard_packet(self.root)
        shutil.rmtree(self.root / "packet")
        shutil.rmtree(self.root / "target")
        self.config = fx.write(self.root / "cfg.json", b"{}\n")
        self.sel_path = self.root / "selection.json"
        self.recs_path = self.root / "reconciliations.json"
        self.counter = self.root / "dispatch-count.txt"

    def _unknown_result(self):
        artifacts = []
        for role in delivery.required_roles("moriarty-sdk-build"):
            raw = b"fixture\n"
            if role == "observation":
                raw = fx.encoded({
                    "schema": "doctorwatson.moriarty-sdk-build-observation/1",
                    "task": "build", "attempt": self.task1["attempt"],
                    "fingerprint": self.fp1, "built": True,
                    "test_bodies_executed": False})
            if role == "resource-receipt":
                raw = fx.encoded({
                    "schema": "doctorwatson.cleanup-observation/1",
                    "task": "build", "attempt": self.task1["attempt"],
                    "fingerprint": self.fp1, "cleanup": "unknown",
                    "restoration": "unknown"})
            path = fx.write(self.root / ("art-" + role), raw)
            artifacts.append({"role": role, **fx.pin(path)})
        return {"task": "build", "attempt": self.task1["attempt"],
                "fingerprint": self.fp1, "adapter": "moriarty-sdk-build",
                "terminal": "unknown-submission", "exit_code": None,
                "signal": None, "cleanup": "unknown",
                "submission": "unknown", "artifacts": artifacts,
                "review": None, "adoption": None}

    def _write_history(self, original):
        self.history_path.write_bytes(fx.encoded(
            {"schema": "doctorwatson.delivery-history/1",
             "results": [original]}))

    def _newer_plan(self, changed=True, disposition="active",
                    attempt="build-002"):
        task = fx.task(self.root, "build", "moriarty-sdk-build",
                       attempt=attempt, disposition=disposition)
        if changed:
            task["identity"]["oracle_sha256"] = "d" * 64
        fx.plan_file(self.root, [task])
        self.task2 = task
        self.fp2 = delivery.fingerprint(
            delivery.load_plan(self.plan_path), "build")
        guard_src = (
            "import sys\n"
            f"sys.path.insert(0, {str(MODELS)!r})\n"
            "from pathlib import Path\n"
            "import fixture_support as fx\n"
            f"root = Path({str(self.root)!r})\n"
            f"counter = Path({str(self.counter)!r})\n"
            "prev = counter.read_text() if counter.exists() else ''\n"
            "counter.write_text(prev + 'x\\n')\n"
            "fx.guard_packet(root)\n"
            f"fx.cleanup_receipt(root / 'packet', 'cleanup.json', 'build',"
            f" {attempt!r}, {self.fp2!r})\n")
        self.guard = fx.write(self.root / "bounded_sdk_compile.py",
                              guard_src.encode())
        self._write_selection(self.fp2)
        return task

    def _write_selection(self, fingerprint):
        sel = {"schema": "doctorwatson.execution-selection/1",
               "task": "build", "adapter": "moriarty-sdk-build",
               "fingerprint": fingerprint,
               "outerSequence": "bounded-sdk-compile-cached-current-f/1",
               "guard": fx.pin(self.guard), "config": fx.pin(self.config),
               "outputs": self.outputs}
        self.sel_path.write_bytes(fx.encoded(sel))

    def _write_recs(self, resolution="process-exited", outcome="exited",
                    exit_code=101, token=None, rec_digest=None,
                    rec_fingerprint=None, cleanup="complete"):
        digest = rec.result_digest(delivery.load_result(self.original))
        evidence = fx.write(self.root / "evidence.json", fx.encoded({
            "schema": "doctorwatson.process-reconciliation-evidence/2",
            "task": "build", "attempt": self.task1["attempt"],
            "fingerprint": self.fp1, "preexec_sha256": "a" * 64,
            "original_result_sha256": token or digest,
            "process_outcome": outcome, "exit_code": exit_code,
            "signal": None, "cleanup": cleanup,
            "restoration": cleanup}))
        record = {"schema": "doctorwatson.reconciliation/1",
                  "task": "build", "attempt": self.task1["attempt"],
                  "fingerprint": rec_fingerprint or self.fp1,
                  "adapter": "moriarty-sdk-build",
                  "original_result_sha256": rec_digest or digest,
                  "resolution": resolution,
                  "evidence": [{"role": "process-evidence",
                                **fx.pin(evidence)}]}
        self.recs_path.write_bytes(fx.encoded(
            {"schema": "doctorwatson.reconciliations/1",
             "records": [record]}))
        return evidence

    def _run_cli(self, attempt="exec-att", history=True, recs=True):
        argv = [sys.executable, str(MODELS / "run_selected.py"),
                "--execute", "build", "--plan", str(self.plan_path),
                "--selection", str(self.sel_path),
                "--attempt", str(self.root / attempt)]
        if history:
            argv += ["--history", str(self.history_path)]
        if recs:
            argv += ["--reconciliations", str(self.recs_path)]
        return subprocess.run(argv, cwd=MODELS, capture_output=True,
                              text=True)

    def _inspect(self):
        argv = [sys.executable, str(MODELS / "reconciliation.py"),
                "--plan", str(self.plan_path), "--results",
                str(self.history_path), "--reconciliations",
                str(self.recs_path)]
        done = subprocess.run(argv, cwd=MODELS, capture_output=True,
                              text=True)
        self.assertEqual(done.returncode, 0, done.stderr)
        return json.loads(done.stdout)

    def _dispatches(self):
        if not self.counter.exists():
            return 0
        return self.counter.read_text().count("x")

    def _assert_zero_dispatch(self, done, attempt="exec-att",
                              codes=(2, 3)):
        self.assertIn(done.returncode, codes, done.stderr)
        self.assertEqual(self._dispatches(), 0)

    def test_positive_cli_roundtrip_and_identity_matches_inspect(self):
        self._newer_plan(changed=True)
        self._write_recs()
        history_before = self.history_path.read_bytes()
        report = self._inspect()
        self.assertEqual(report["ready_for_root_selection"], ["build"])
        done = self._run_cli()
        self.assertEqual(done.returncode, 0, done.stderr)
        self.assertEqual(self._dispatches(), 1)
        attempt = self.root / "exec-att"
        preexec = json.loads((attempt / "PREEXEC.json").read_bytes())
        self.assertEqual(preexec["readinessIdentity"],
                         report["readiness_identity"])
        self.assertEqual(preexec["fingerprint"], self.fp2)
        self.assertNotEqual(self.fp2, self.fp1)
        delivery.load_result(json.loads(
            (attempt / "consumer-result.json").read_bytes()))
        self.assertEqual(self.history_path.read_bytes(), history_before)

    def test_positive_shell_roundtrip(self):
        self._newer_plan(changed=True)
        self._write_recs()
        shell = str(REPO / "scripts/doctorwatson.sh")
        done = subprocess.run(
            [shell, "--inspect", "--plan", str(self.plan_path),
             "--results", str(self.history_path), "--reconciliations",
             str(self.recs_path), "--next"],
            cwd=REPO, capture_output=True, text=True)
        self.assertEqual(done.returncode, 0, done.stderr)
        self.assertEqual(json.loads(done.stdout)
                         ["ready_for_root_selection"], ["build"])
        done = subprocess.run(
            [shell, "--execute", "build", "--plan", str(self.plan_path),
             "--selection", str(self.sel_path), "--attempt",
             str(self.root / "shell-att"), "--history",
             str(self.history_path), "--reconciliations",
             str(self.recs_path)],
            cwd=REPO, capture_output=True, text=True)
        self.assertEqual(done.returncode, 0, done.stderr)
        self.assertEqual(self._dispatches(), 1)

    def test_unknown_history_without_reconciliations_blocks(self):
        self._newer_plan(changed=True)
        done = self._run_cli(recs=False)
        self._assert_zero_dispatch(done, codes=(3,))

    def test_wrong_evidence_token_zero_dispatch(self):
        self._newer_plan(changed=True)
        self._write_recs(token="0" * 64)
        self._assert_zero_dispatch(self._run_cli(), codes=(3,))

    def test_old_result_digest_zero_dispatch(self):
        self._newer_plan(changed=True)
        stale = dict(self.original, cleanup="complete")
        self._write_recs(
            rec_digest=rec.result_digest(delivery.load_result(stale)))
        self._assert_zero_dispatch(self._run_cli(), codes=(3,))

    def test_missing_cleanup_zero_dispatch(self):
        self._newer_plan(changed=True)
        self._write_recs(cleanup="owed")
        self._assert_zero_dispatch(self._run_cli(), codes=(3,))

    def test_missing_evidence_file_zero_dispatch(self):
        self._newer_plan(changed=True)
        evidence = self._write_recs()
        evidence.unlink()
        self._assert_zero_dispatch(self._run_cli(), codes=(3,))

    def test_wrong_selection_fingerprint_zero_dispatch(self):
        self._newer_plan(changed=True)
        self._write_recs()
        self._write_selection(self.fp1)
        self._assert_zero_dispatch(self._run_cli(), codes=(3,))

    def test_wrong_record_fingerprint_zero_dispatch(self):
        self._newer_plan(changed=True)
        self._write_recs(rec_fingerprint="0" * 64)
        self._assert_zero_dispatch(self._run_cli(), codes=(3,))

    def test_cancelled_task_zero_dispatch(self):
        self._newer_plan(changed=True, disposition="cancelled")
        self._write_recs()
        self._assert_zero_dispatch(self._run_cli(), codes=(3,))

    def test_current_attempt_unresolved_zero_dispatch(self):
        self._newer_plan(changed=True, attempt="build-001")
        self._write_recs()
        self._assert_zero_dispatch(self._run_cli(), codes=(3,))

    def test_same_fingerprint_consumed_retry_zero_dispatch(self):
        self._newer_plan(changed=False)
        self.assertEqual(self.fp2, self.fp1)
        self._write_recs()
        self._assert_zero_dispatch(self._run_cli(), codes=(3,))

    def test_tampered_history_zero_dispatch(self):
        self._newer_plan(changed=True)
        self._write_recs()
        self._write_history(dict(self.original, cleanup="complete"))
        self._assert_zero_dispatch(self._run_cli(), codes=(3,))

    def test_tampered_reconciliations_zero_dispatch(self):
        self._newer_plan(changed=True)
        self._write_recs()
        doc = json.loads(self.recs_path.read_bytes())
        doc["records"][0]["resolution"] = "process-not-started"
        self.recs_path.write_bytes(fx.encoded(doc))
        self._assert_zero_dispatch(self._run_cli(), codes=(3,))

    def test_truncated_reconciliations_bad_input_zero_dispatch(self):
        self._newer_plan(changed=True)
        self.recs_path.write_bytes(b'{"schema": "doctorwatson.rec')
        self._assert_zero_dispatch(self._run_cli(), codes=(2,))

    def test_reconciliations_without_history_bad_input(self):
        self._newer_plan(changed=True)
        self._write_recs()
        self._assert_zero_dispatch(self._run_cli(history=False),
                                   codes=(2,))

    def test_known_failed_original_not_started_zero_dispatch(self):
        self.original = dict(self.original, terminal="failed",
                             exit_code=101, cleanup="owed",
                             submission="not-applicable")
        self._write_history(self.original)
        self._newer_plan(changed=False)
        self.assertEqual(self.fp2, self.fp1)
        self._write_recs(resolution="process-not-started",
                         outcome="not-started", exit_code=None)
        self._assert_zero_dispatch(self._run_cli(), codes=(3,))

    def test_exit_mismatch_zero_dispatch(self):
        self.original = dict(self.original, terminal="failed",
                             exit_code=3, cleanup="owed",
                             submission="not-applicable")
        self._write_history(self.original)
        self._newer_plan(changed=True)
        self._write_recs(exit_code=0)
        self._assert_zero_dispatch(self._run_cli(), codes=(3,))

    def test_sibling_receipt_debt_zero_dispatch(self):
        sibling = {"task": "build", "attempt": "build-000",
                   "fingerprint": self.fp1,
                   "adapter": "moriarty-sdk-build",
                   "terminal": "failed", "exit_code": 1, "signal": None,
                   "cleanup": "complete", "submission": "not-applicable",
                   "artifacts": [], "review": None, "adoption": None}
        self.history_path.write_bytes(fx.encoded(
            {"schema": "doctorwatson.delivery-history/1",
             "results": [self.original, sibling]}))
        self._newer_plan(changed=True)
        self._write_recs()
        self._assert_zero_dispatch(self._run_cli(), codes=(3,))


class ActualUseRecheckTests(unittest.TestCase):
    def test_identity_drift_after_first_resolve_zero_dispatch(self):
        case = ExecutionReconciliationCliTests(
            "test_positive_cli_roundtrip_and_identity_matches_inspect")
        case.setUp()
        self.addCleanup(case.tmp.cleanup)
        case._newer_plan(changed=True)
        case._write_recs()
        calls = []
        real = rs.resolve_selection
        state = {"n": 0}

        def drifting(path, task):
            out = real(path, task)
            state["n"] += 1
            if state["n"] == 1:
                case._write_history(
                    dict(case.original, cleanup="complete"))
            return out

        with mock.patch.object(rs, "resolve_selection", drifting):
            with self.assertRaises((rs.Blocked, rs.BadInput)):
                rs.run_selected_once(
                    "build", str(case.plan_path), str(case.sel_path),
                    str(case.root / "drift-att"),
                    runner=lambda argv: calls.append(argv) or 0,
                    history_path=str(case.history_path),
                    reconciliations_path=str(case.recs_path))
        self.assertEqual(calls, [])


class DeletedSentinelTests(unittest.TestCase):
    def test_deleted_attempt_dir_does_not_allow_same_attempt(self):
        case = ExecutionReconciliationCliTests(
            "test_positive_cli_roundtrip_and_identity_matches_inspect")
        case.setUp()
        self.addCleanup(case.tmp.cleanup)
        # Original plan/selection for build-001 with a plain guard file.
        guard = fx.write(case.root / "guard.py", b"# guard\n")
        case.guard = guard
        case._write_selection(case.fp1)
        calls = []

        def unknown_runner(argv):
            calls.append(list(argv))
            raise TimeoutError("unknown")

        code, _ = rs.run_selected_once(
            "build", str(case.plan_path), str(case.sel_path),
            str(case.root / "att"), runner=unknown_runner)
        self.assertEqual(code, rs.EXIT_UNKNOWN)
        shutil.rmtree(case.root / "att")  # sentinel deleted by operator
        calls2 = []
        with self.assertRaises(rs.Blocked):
            rs.run_selected_once(
                "build", str(case.plan_path), str(case.sel_path),
                str(case.root / "att"),
                runner=lambda argv: calls2.append(argv) or 0,
                history_path=str(case.history_path))
        self.assertEqual(calls2, [])


if __name__ == "__main__":
    unittest.main()
