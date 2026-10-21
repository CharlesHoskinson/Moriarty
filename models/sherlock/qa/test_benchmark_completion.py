"""Existing checker integration; pure files and synthetic labels only."""
import contextlib
import importlib.util
import io
import json
from pathlib import Path
import sys
import tempfile
import unittest

HERE=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location("benchmark_completion_qa",HERE/"check.py")
qa=importlib.util.module_from_spec(spec); spec.loader.exec_module(qa)
sys.path.insert(0,str(HERE.parent/"benchmarks"))
import test_acceptance as fixture_module

class DoctorWatsonBenchmarkTests(unittest.TestCase):
    def test_partial_selection_is_refused_before_corpus_reads(self):
        with self.assertRaisesRegex(ValueError,"benchmark.*together"):
            qa.run(Path("/no-repo"),benchmark_policy=Path("policy.json"))

    def test_missing_inputs_fail_explicit_selection(self):
        with self.assertRaises((OSError,ValueError)):
            qa.benchmark_completion_report(Path("/none"),Path("/none"),Path("/none"),Path("/none"))

    def test_recomputes_source_and_receipt_evidence_each_call(self):
        fixture=fixture_module.AcceptanceTests(); fixture.setUp()
        self.addCleanup(fixture.doCleanups)
        root=fixture.root
        for name,value in (("policy",fixture.policy),("samples",[fixture.sample]),("current",fixture.current)):
            (root/(name+".json")).write_text(json.dumps(value))
        args=[root/"policy.json",root/"samples.json",root/"current.json",root]
        self.assertTrue(qa.benchmark_completion_report(*args)["complete"])
        (root/"readback.json").write_text("{}")
        self.assertFalse(qa.benchmark_completion_report(*args)["complete"])

    def test_actual_checker_cli_returns_mismatch_after_receipt_drift(self):
        fixture=fixture_module.AcceptanceTests();fixture.setUp();self.addCleanup(fixture.doCleanups)
        root=fixture.root
        for name,value in (("policy",fixture.policy),("samples",[fixture.sample]),("current",fixture.current)):
            (root/(name+".json")).write_text(json.dumps(value))
        args=["--repo",str(HERE.parents[2]),"--benchmark-policy",str(root/"policy.json"),
              "--benchmark-samples",str(root/"samples.json"),"--benchmark-current",str(root/"current.json"),
              "--benchmark-receipt-root",str(root)]
        stream=io.StringIO()
        with contextlib.redirect_stdout(stream): code=qa.main(args)
        self.assertEqual(code,0)
        self.assertTrue(json.loads(stream.getvalue())["benchmark_completion"]["complete"])
        (root/"readback.json").write_text("{}")
        stream=io.StringIO()
        with contextlib.redirect_stdout(stream): code=qa.main(args)
        self.assertEqual(code,1)
        result=json.loads(stream.getvalue())
        self.assertEqual(result["status"],"mismatch")
        self.assertFalse(result["benchmark_completion"]["complete"])

    def test_cli_exposes_current_input_instead_of_cached_report(self):
        stream=io.StringIO()
        with contextlib.redirect_stdout(stream), self.assertRaises(SystemExit) as exit:
            qa.main(["--help"])
        self.assertEqual(exit.exception.code,0)
        for flag in ["--benchmark-policy","--benchmark-samples","--benchmark-current","--benchmark-receipt-root","--benchmark-baseline"]:
            self.assertIn(flag,stream.getvalue())

if __name__=="__main__": unittest.main()
