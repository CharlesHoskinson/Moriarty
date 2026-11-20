import importlib.util
import tempfile
import unittest
from pathlib import Path
from unittest import mock

SCRIPT = Path(__file__).resolve().parents[1] / "order-report.py"
spec = importlib.util.spec_from_file_location("order_report", SCRIPT)
report = importlib.util.module_from_spec(spec)
spec.loader.exec_module(report)

ASSERTION = "<failure message=\"Expected values to be strictly equal: '2' !== '1'\">cause: AssertionError [ERR_ASSERTION]</failure>"
CRASH = "<failure message=\"native.serialize is not a function '2' !== '1'\">TypeError</failure>"


def junit(*cases):
    body = "".join(f'<testcase name="{name}" file="/tests/a.mjs">{detail}</testcase>' for name, detail in cases)
    return f"<testsuites>{body}</testsuites>"


class BaselineTests(unittest.TestCase):
    def test_suite_requires_each_file_and_an_executed_test(self):
        xml = '<testsuites><testcase name="one" file="/tests/a.mjs"/></testsuites>'
        self.assertEqual(report.test_inventory(xml, {Path("/tests/a.mjs")}), {"one"})
        for invalid in (xml.replace('a.mjs','b.mjs'), xml.replace('/>','><skipped/></testcase>'),
                        '<testsuites/>', xml.replace('/>','><failure/></testcase>')):
            with self.subTest(invalid=invalid), self.assertRaises(ValueError):
                report.test_inventory(invalid, {Path("/tests/a.mjs")})

    def test_coverage_needs_executed_lines_from_every_scoped_file(self):
        source = Path("src/kernel.ts")
        lcov = 'SF:src/kernel.ts\nDA:1,1\nDA:2,0\nend_of_record\n'
        self.assertEqual(report.coverage(lcov, {source}), {str(source): {"covered": 1, "total": 2}})
        self.assertEqual(report.line_hits(lcov, {source}), {str(source): {1: 1, 2: 0}})
        for invalid in ('', lcov.replace('1,1','1,0'), lcov.replace('kernel.ts','other.ts'),
                        lcov.replace('2,0','1,0')):
            with self.subTest(invalid=invalid), self.assertRaises(ValueError):
                report.coverage(invalid, {source})

    def test_declared_scopes_measure_source_kernel_and_ledger_with_their_suites(self):
        self.assertLessEqual({"source", "kernel", "ledger"}, set(report.SCOPES))
        self.assertEqual(set(report.SCOPES), set(report.SCOPE_SUITES))
        self.assertEqual(set(report.SCOPES), set(report.MUTATIONS))
        self.assertEqual(report.COVERAGE_SUITES, list(report.SCOPE_SUITES.values()))
        self.assertIn(report.LANG / "src/parser.ts", report.SCOPES["source"])
        self.assertIn(report.LANG / "src/checker.ts", report.SCOPES["source"])
        self.assertEqual(report.SCOPES["kernel"], (report.LANG / "src/lower-compact.ts",))
        self.assertEqual(report.SCOPES["ledger"], (Path("packages/moriarty-orders/src/accepted-state.mjs"),))
        self.assertEqual(report.SCOPE_SUITES["ledger"], Path("packages/moriarty-orders/test/acceptance.test.mjs"))
        for scope, mutation in report.MUTATIONS.items():
            with self.subTest(scope=scope):
                self.assertIn(mutation["file"], report.SCOPES[scope])
                for path in (*report.SCOPES[scope], report.SCOPE_SUITES[scope]):
                    self.assertTrue((report.ROOT / path).is_file(), path)

    def test_coverage_requires_a_row_for_every_declared_scope_file(self):
        scoped = {path for paths in report.SCOPES.values() for path in paths}
        lcov = "".join(f"SF:{path}\nDA:1,1\nDA:2,0\nend_of_record\n" for path in sorted(scoped))
        rows = report.coverage(lcov, scoped)
        self.assertEqual(set(rows), {str(path) for path in scoped})
        self.assertIn("packages/moriarty-orders/src/accepted-state.mjs", rows)
        self.assertEqual(rows["experiments/moriarty-language/src/parser.ts"], {"covered": 1, "total": 2})
        without_ledger = lcov.replace("SF:packages/moriarty-orders/src/accepted-state.mjs", "SF:other.mjs")
        with self.assertRaises(ValueError):
            report.coverage(without_ledger, scoped)

    def test_only_the_registered_assertion_proves_a_mutation_was_caught(self):
        mutation = {"tests": ["expected"], "reason": "'2' !== '1'"}
        clean = {"expected", "other"}
        caught = junit(("expected", ASSERTION), ("other", ""))
        self.assertEqual(report.mutation_outcome(1, caught, mutation, clean), ["expected"])
        for label, code, xml, reason in [
            ("not caught", 0, junit(("expected", ""), ("other", "")), "executed assertion failure"),
            ("not executed", 1, "import failure", "executed assertion failure"),
            ("wrong exit", 127, caught, "executed assertion failure"),
            ("wrong test", 1, junit(("expected", ""), ("other", ASSERTION)), "unexpected failing checks"),
            ("extra test", 1, junit(("expected", ASSERTION), ("other", ASSERTION)), "unexpected failing checks"),
            ("crash, not an assertion", 1, junit(("expected", CRASH), ("other", "")), "expected assertion"),
            ("other reason", 1, junit(("expected", ASSERTION.replace("'2'", "'3'")), ("other", "")), "expected assertion"),
            ("skipped test", 1, junit(("expected", ASSERTION), ("other", "<skipped/>")), "unexpected failing checks"),
            ("lost test", 1, junit(("expected", ASSERTION)), "clean test inventory"),
        ]:
            with self.subTest(label), self.assertRaisesRegex(ValueError, reason):
                report.mutation_outcome(code, xml, mutation, clean)

    def test_clean_fixture_must_pass_the_registered_check(self):
        mutation = {"tests": ["expected"], "reason": "x"}
        suite = Path("/tests/a.mjs")
        passing = junit(("expected", ""), ("other", ""))
        self.assertEqual(report.clean_outcome(0, passing, suite, mutation), {"expected", "other"})
        for label, code, xml, reason in [("fails unmutated", 1, junit(("expected", ASSERTION)), "clean fixture fails"),
                                         ("failing case", 0, junit(("expected", ASSERTION)), "failed or skipped"),
                                         ("registered check absent", 0, junit(("other", "")), "lacks the registered check")]:
            with self.subTest(label), self.assertRaisesRegex(ValueError, reason):
                report.clean_outcome(code, xml, suite, mutation)


class ControlledRegressionTests(unittest.TestCase):
    def probe(self, scope, executed=None):
        with tempfile.TemporaryDirectory() as tmp:
            return report.probe_mutation(Path(tmp), scope, executed)

    def test_each_scoped_regression_fails_its_check_and_passes_again_when_clean(self):
        for scope, mutation in report.MUTATIONS.items():
            target = report.ROOT / mutation["file"]
            original = target.read_bytes()
            with self.subTest(scope=scope):
                result = self.probe(scope)
                self.assertEqual(result["failedTests"], sorted(mutation["tests"]))
                self.assertEqual(result["exit"], 1)
                self.assertNotEqual(result["beforeSha256"], result["afterSha256"])
                self.assertEqual(result["clean"]["exit"], 0)
                self.assertEqual(result["clean"]["restoredSha256"], result["beforeSha256"])
                self.assertEqual(result["beforeSha256"], report.sha(original))
                self.assertEqual(target.read_bytes(), original)

    def test_a_mutation_caught_by_another_check_or_not_caught_is_refused(self):
        base = report.MUTATIONS["outcomeIntent"]
        for label, change, reason in [
            ("wrong check", {"tests": ["Fee cannot fund an Originate"]}, "unexpected failing checks"),
            ("wrong reason", {"reason": "'102' !== '100'"}, "expected assertion"),
            ("not caught", {"with": base["replace"].replace("toString()", "toString(10)")}, "executed assertion failure"),
        ]:
            with self.subTest(label), mock.patch.dict(report.MUTATIONS, {"outcomeIntent": {**base, **change}}):
                with self.assertRaisesRegex(ValueError, reason):
                    self.probe("outcomeIntent")

    def test_a_mutation_outside_executed_scope_lines_is_refused(self):
        mutation = report.MUTATIONS["kernel"]
        with self.assertRaisesRegex(ValueError, "not executed by the scoped suites"):
            self.probe("kernel", {str(mutation["file"]): {}})
        moved = {**mutation, "file": report.LANG / "src/evaluate.ts"}
        with mock.patch.dict(report.MUTATIONS, {"kernel": moved}):
            with self.assertRaisesRegex(ValueError, "outside its coverage scope"):
                self.probe("kernel")


if __name__ == "__main__":
    unittest.main()
