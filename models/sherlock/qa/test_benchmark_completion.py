"""Actual existing Python checker/CLI, with synthetic receipts only."""
import contextlib
import importlib.util
import io
import json
from pathlib import Path
import shutil
import sys
import tempfile
import unittest

HERE=Path(__file__).resolve().parent
REPO=HERE.parents[2]
spec=importlib.util.spec_from_file_location("benchmark_completion_qa",HERE/"check.py")
qa=importlib.util.module_from_spec(spec);spec.loader.exec_module(qa)
sys.path.insert(0,str(HERE.parent/"benchmarks"))
from test_acceptance import Fixture


class DoctorWatsonBenchmarkTests(unittest.TestCase):
    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory();self.addCleanup(self.tmp.cleanup)
        self.root=Path(self.tmp.name)
        self.fixture=Fixture(self.root/"retained")
        for name,value in (("policy",self.fixture.policy),("samples",self.fixture.samples),("current",self.fixture.current)):
            (self.fixture.root/(name+".json")).write_text(json.dumps(value))
        self.paths=[self.fixture.root/"policy.json",self.fixture.root/"samples.json",self.fixture.root/"current.json",self.fixture.root]

    def source_copy(self):
        target=self.root/"selected-checkout";trace_path=Path("openspec/changes/sherlock-moriarty-benchmarks/traceability.json")
        trace=json.loads((REPO/trace_path).read_text())
        for pin in trace["sourcePins"]:
            path=target/pin["path"];path.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(REPO/pin["path"],path)
        path=target/trace_path;path.parent.mkdir(parents=True,exist_ok=True);path.write_text(json.dumps(trace))
        return target,path,trace

    def test_partial_selection_refused_before_corpus_reads(self):
        with self.assertRaisesRegex(ValueError,"benchmark.*together"):
            qa.run(Path("/no-repo"),benchmark_policy=Path("policy.json"))

    def test_real_catalog_cli_completes_then_fails_after_receipt_drift(self):
        args=["--repo",str(REPO)]
        for flag,path in zip(("policy","samples","current","receipt-root"),self.paths):args += ["--benchmark-"+flag,str(path)]
        stream=io.StringIO()
        with contextlib.redirect_stdout(stream):code=qa.main(args)
        self.assertEqual(code,0)
        report=json.loads(stream.getvalue());self.assertTrue(report["benchmark_completion"]["complete"])
        inputs=report["benchmark_completion"]["inputs"]
        for key in ("acceptance_source","metrics_source","sample_schema_source","catalog_source","workload_validator_source"):
            self.assertEqual(len(inputs[key]["sha256"]),64)
        ref=self.fixture.samples[0]["receipts"][-1]
        (self.fixture.root/ref["path"]).write_text("{}")
        stream=io.StringIO()
        with contextlib.redirect_stdout(stream):code=qa.main(args)
        self.assertEqual(code,1);self.assertFalse(json.loads(stream.getvalue())["benchmark_completion"]["complete"])

    def test_selected_repo_loads_its_own_acceptance_module(self):
        repo,trace_path,trace=self.source_copy()
        path=repo/"models/sherlock/benchmarks/acceptance.py"
        text=path.read_text().replace('report["inputs"] = {name:', 'report["selected_checkout_marker"] = "selected-repo"\n    report["inputs"] = {name:')
        path.write_text(text)
        for pin in trace["sourcePins"]:
            if pin["path"]==str(path.relative_to(repo)):pin["sha256"]=qa.digest(path.read_bytes())
        trace_path.write_text(json.dumps(trace))
        report=qa.benchmark_completion_report(*self.paths,repo=repo)
        self.assertTrue(report["complete"]);self.assertEqual(report["selected_checkout_marker"],"selected-repo")

    def test_cross_lane_traceability_source_edit_invalidates_selection(self):
        repo,_,_=self.source_copy()
        with (repo/"models/sherlock/benchmarks/metrics.py").open("a") as stream:stream.write("\n# changed candidate\n")
        with self.assertRaisesRegex(ValueError,"source changed"):
            qa.benchmark_completion_report(*self.paths,repo=repo)

    def test_missing_referenced_receipt_is_cli_input_error_two(self):
        ref=self.fixture.samples[0]["receipts"][-1];(self.fixture.root/ref["path"]).unlink()
        args=["--repo",str(REPO)]
        for flag,path in zip(("policy","samples","current","receipt-root"),self.paths):args += ["--benchmark-"+flag,str(path)]
        with contextlib.redirect_stderr(io.StringIO()),self.assertRaises(SystemExit) as error:
            qa.main(args)
        self.assertEqual(error.exception.code,2)

    def test_cli_exposes_fresh_current_inputs(self):
        stream=io.StringIO()
        with contextlib.redirect_stdout(stream),self.assertRaises(SystemExit) as error:qa.main(["--help"])
        self.assertEqual(error.exception.code,0)
        for flag in ("--benchmark-policy","--benchmark-samples","--benchmark-current","--benchmark-receipt-root","--benchmark-baseline"):
            self.assertIn(flag,stream.getvalue())


if __name__=="__main__":unittest.main()
