import hashlib
import importlib.util
import json
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
REPORT = Path("deliverables/competitor-report")
RUNNER = Path("scripts/qa/order-report.py")
DESIGN = Path("openspec/changes/competitive-landscape-adoption/design.md")


class ReconciliationTests(unittest.TestCase):
    def setUp(self):
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        self.root = Path(directory.name)
        self.base = self.root / REPORT
        shutil.copytree(ROOT / REPORT, self.base)
        for path in (DESIGN, RUNNER, RUNNER.with_name("order_evidence.py"), RUNNER.with_name("order_cost_report.py")):
            (self.root / path).parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / path, self.root / path)
        # The handoff is a snapshot. Give the fixture its own registered replay inputs.
        spec = importlib.util.spec_from_file_location("order_report", self.root / RUNNER)
        self.report = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(self.report)
        for case in self.report.NATIVE:
            receipt = self.report.native_receipt(case)
            record = json.loads(receipt.read_bytes())
            for name in record["inputs"]:
                source = self.root / name
                source.parent.mkdir(parents=True, exist_ok=True)
                source.write_text(name)
                record["inputs"][name] = hashlib.sha256(source.read_bytes()).hexdigest()
            receipt.write_text(json.dumps(record))

    def read(self, name):
        return json.loads((self.base / name).read_bytes())

    def reconcile(self, *options):
        return subprocess.run([sys.executable, str(self.root / RUNNER), "reconcile", *options],
                              cwd=self.root, capture_output=True, text=True)

    def test_finalize_refreshes_complete_rows_before_index_and_check_is_read_only(self):
        path = self.base / "I.2/result.json"
        stale = self.read("I.2/result.json")
        stale["orderBehaviorCoverage"].pop("IV.3", None)
        stale["orderBehaviorCoverage"]["II.1"] = {"tests": ["a stale test"]}
        path.write_text(json.dumps(stale))
        before = path.read_bytes()
        check = self.reconcile("--check")
        self.assertNotEqual(check.returncode, 0)
        self.assertIn("orderBehaviorCoverage", check.stderr)
        self.assertEqual(path.read_bytes(), before)
        final = self.reconcile()
        self.assertEqual(final.returncode, 0, final.stderr)
        baseline = self.read("I.2/result.json")
        self.assertEqual(stale | {"orderBehaviorCoverage": baseline["orderBehaviorCoverage"]}, baseline)
        self.assertEqual(set(baseline["orderBehaviorCoverage"]), set(self.report.NATIVE))
        for case, row in baseline["orderBehaviorCoverage"].items():
            receipt = (self.root / row["receipt"]).read_bytes()
            self.assertEqual(row["sha256"], hashlib.sha256(receipt).hexdigest(), case)
            self.assertEqual(row["tests"], json.loads(receipt)["tests"])
            self.assertEqual(row["scopedCoverage"], json.loads(receipt).get("coverage"))
        index = self.read("index.json")
        self.assertEqual(index["recommendations"]["I.2"]["evidence"][0]["sha256"],
                         hashlib.sha256(path.read_bytes()).hexdigest())
        self.assertEqual(self.reconcile("--check").returncode, 0)
        frozen = path.read_bytes(), (self.base / "index.json").read_bytes()
        self.assertEqual(self.reconcile().returncode, 0)
        self.assertEqual(frozen, (path.read_bytes(), (self.base / "index.json").read_bytes()))
        index["recommendations"].pop("II.1")
        (self.base / "index.json").write_text(json.dumps(index))
        self.assertNotEqual(self.reconcile("--check").returncode, 0)
        self.assertEqual(self.read("index.json"), index)

    def test_baseline_can_run_while_a_later_native_replay_is_out_of_date(self):
        (self.root / "packages/moriarty-orders/test/o8.test.mjs").write_text("changed")
        rows = self.report.order_behavior_coverage(complete=False)
        self.assertEqual(set(rows), set(self.report.NATIVE) - {"IV.3"})
        with self.assertRaisesRegex(ValueError, "IV.3"):
            self.report.order_behavior_coverage(complete=True)

    def test_reconciliation_rejects_changed_native_receipts_and_detached_replay(self):
        for name in ("receipts.json", "strict-replay.json"):
            path = self.base / "IV.3" / name
            original = path.read_bytes()
            with self.subTest(name=name):
                path.write_text("{}")
                final = self.reconcile()
                self.assertNotEqual(final.returncode, 0)
                self.assertIn("IV.3", final.stderr)
            path.write_bytes(original)

    def test_reconciliation_rejects_a_workflow_validation_made_under_other_pins(self):
        # The fixture has its own replay inputs; refresh the receipt references first.
        refresh = self.reconcile()
        self.assertEqual(refresh.returncode, 0, refresh.stderr)
        path = self.base / "III.1/workflow-pins.json"
        path.write_text(json.dumps({**json.loads(path.read_bytes()), "revision": "0" * 40}))
        before = (self.base / "I.2/result.json").read_bytes(), (self.base / "index.json").read_bytes()
        for options in (("--check",), ()):
            with self.subTest(options=options):
                final = self.reconcile(*options)
                self.assertNotEqual(final.returncode, 0)
                self.assertIn("III.1", final.stderr)
                self.assertEqual(before, ((self.base / "I.2/result.json").read_bytes(),
                                          (self.base / "index.json").read_bytes()))

    def test_reconciliation_rejects_wrong_workflow_revision_with_matching_pins_hash(self):
        refresh = self.reconcile()
        self.assertEqual(refresh.returncode, 0, refresh.stderr)
        path = self.base / "III.1/workflow-check.json"
        record = json.loads(path.read_bytes())
        record["upstreamRevision"] = "b" * 40
        path.write_text(json.dumps(record))
        for options in ((), ("--check",)):
            with self.subTest(options=options):
                run = self.reconcile(*options)
                self.assertNotEqual(run.returncode, 0)
                self.assertIn("III.1", run.stderr)

    def test_finalize_refuses_missing_failed_draft_or_unrelated_receipts(self):
        path = self.base / "IV.3/factory-check.json"
        record = json.loads(path.read_bytes())
        before = (self.base / "I.2/result.json").read_bytes(), (self.base / "index.json").read_bytes()
        for changes in (None, {"passed": False}, {"draft": True}, {"requirement": "IV.2"},
                        {"tests": []}, {"inputs": {}}, {"evidenceRevision": "0" * 40}):
            with self.subTest(changes=changes):
                if changes is None:
                    path.unlink()
                else:
                    path.write_text(json.dumps(record | changes))
                final = self.reconcile()
                self.assertNotEqual(final.returncode, 0)
                self.assertIn("IV.3", final.stderr)
                self.assertEqual(before, ((self.base / "I.2/result.json").read_bytes(),
                                          (self.base / "index.json").read_bytes()))
