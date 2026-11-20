import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest


QA = Path(__file__).resolve().parents[1]
REPORT = Path("deliverables/competitor-report/III.1")


class CostRunFailureTests(unittest.TestCase):
    def setUp(self):
        scratch = tempfile.TemporaryDirectory()
        self.addCleanup(scratch.cleanup)
        self.root = Path(scratch.name)
        self.report = self.root / REPORT
        self.sample = self.report / "native/o1-fill/sample-1"
        self.sample.mkdir(parents=True)
        (self.sample / "receipts.json").write_bytes(b"[]\n")
        qa = self.root / "scripts/qa"
        qa.mkdir(parents=True)
        for name in ("order-report.py", "order_cost_report.py", "validate_order_cost_handoff.py"):
            shutil.copyfile(QA / name, qa / name)
        for name in ("recipe.json", "workloads.json", "workflow-pins.json"):
            shutil.copyfile(QA.parents[1] / REPORT / name, self.report / name)
        (self.report / "result.json").write_text('{"passed": true, "totals": {}}')
        (self.report / "assessment-input.json").write_text('{"values": {}}')
        (self.report / "workflow-check.json").write_text('{"validated": {}}')

    def invoke(self, script, argument, **env):
        return subprocess.run([sys.executable, "scripts/qa/" + script, argument],
                              cwd=self.root, capture_output=True, text=True,
                              env={**os.environ, **env}, timeout=30)

    def assert_withheld(self, process, reason):
        self.assertNotEqual(process.returncode, 0)
        self.assertIn(reason, process.stderr)
        result = json.loads((self.report / "result.json").read_bytes())
        self.assertIs(result["passed"], False)
        self.assertIn(reason, result["error"])
        self.assertNotIn("totals", result)
        self.assertNotIn("assessmentInput", result)
        self.assertEqual(result["retainedSamples"], [str(self.sample.relative_to(self.root))])
        self.assertEqual((self.sample / "receipts.json").read_bytes(), b"[]\n")
        self.assertFalse((self.report / "assessment-input.json").exists())
        self.assertFalse((self.report / "workflow-check.json").exists())

    def test_sample_failure_withholds_the_previous_assessment(self):
        proc = self.invoke("order_cost_report.py", "o1-fill")
        self.assert_withheld(proc, "at least two consecutively numbered serial samples")

    def test_malformed_configuration_withholds_the_previous_assessment(self):
        (self.report / "recipe.json").write_text("{")
        proc = self.invoke("order_cost_report.py", "cancel-unfilled")
        self.assert_withheld(proc, "Expecting property name")

    def test_empty_workload_profile_cannot_publish_an_empty_success(self):
        path = self.report / "workloads.json"
        profile = json.loads(path.read_bytes())
        profile["operations"] = []
        path.write_text(json.dumps(profile))
        proc = self.invoke("order_cost_report.py", "o1-fill")
        self.assert_withheld(proc, "pinned workload profile")

    def test_recipe_cannot_skip_the_required_local_measurements(self):
        path = self.report / "recipe.json"
        recipe = json.loads(path.read_bytes())
        recipe["unavailable"].update(dict.fromkeys(recipe["operations"], "skipped"))
        path.write_text(json.dumps(recipe))
        proc = self.invoke("order_cost_report.py", "o1-fill")
        self.assert_withheld(proc, "o1-fill and cancel-unfilled")

    def test_failed_preliminary_measurement_suite_withholds_the_previous_assessment(self):
        subprocess.run(["git", "init", "-q"], cwd=self.root, check=True)
        suite = self.root / "packages/moriarty-orders/test/measurement.test.mjs"
        suite.parent.mkdir(parents=True)
        suite.write_text("import {test} from 'node:test';\ntest('preflight', () => { throw Error('injected failure'); });")
        proc = self.invoke("order-report.py", "III.1", MORIARTY_EVIDENCE_DRAFT="1")
        self.assert_withheld(proc, "measurements exited 1")

    def test_source_revision_failure_withholds_the_previous_assessment(self):
        subprocess.run(["git", "init", "-q"], cwd=self.root, check=True)
        proc = self.invoke("order-report.py", "III.1", MORIARTY_EVIDENCE_DRAFT="0")
        self.assert_withheld(proc, "clean committed tree")

    def test_failed_handoff_validation_removes_the_previous_validation(self):
        proc = self.invoke("validate_order_cost_handoff.py", str(self.root / "missing-upstream"))
        self.assertNotEqual(proc.returncode, 0)
        self.assertIn("missing-upstream", proc.stderr)
        self.assertFalse((self.report / "workflow-check.json").exists())
        self.assertTrue((self.report / "assessment-input.json").is_file())


if __name__ == "__main__":
    unittest.main()
