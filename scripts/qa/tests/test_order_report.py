import importlib.util
import tempfile
import unittest
from pathlib import Path

SCRIPT = Path(__file__).resolve().parents[1] / "order-report.py"
spec = importlib.util.spec_from_file_location("order_report", SCRIPT)
report = importlib.util.module_from_spec(spec)
spec.loader.exec_module(report)


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
        for invalid in ('', lcov.replace('1,1','1,0'), lcov.replace('kernel.ts','other.ts'),
                        lcov.replace('2,0','1,0')):
            with self.subTest(invalid=invalid), self.assertRaises(ValueError):
                report.coverage(invalid, {source})

    def test_positive_exit_alone_cannot_prove_a_mutation_was_caught(self):
        expected = report.MUTATION_TEST
        for code, detail in [(0, "assertion"), (1, "import failure"), (127, expected)]:
            with self.subTest(code=code), self.assertRaises(ValueError):
                report.mutation_outcome(code, detail)

    def test_real_amount_regression_fails_original_boundary_assertion(self):
        with tempfile.TemporaryDirectory() as tmp:
            result = report.probe_mutation(Path(tmp))
        self.assertEqual(result['failedTests'], [report.MUTATION_TEST])
        self.assertNotEqual(result['beforeSha256'], result['afterSha256'])
        self.assertEqual(result['exit'], 1)
