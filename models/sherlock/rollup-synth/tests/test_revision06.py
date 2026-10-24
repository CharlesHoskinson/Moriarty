"""Focused discovery/JSON overflow regressions; no old test cohort execution."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from common import Error, parse_json

CODE = Path(__file__).resolve().parents[1]

class Revision06Tests(unittest.TestCase):
    def test_real_discovery_with_environment_cut_unset(self):
        environment = dict(os.environ)
        environment.pop("FSYN_DESIGN_CUT", None)
        code = """import json, pathlib, unittest, sys
loader = unittest.TestLoader()
suite = loader.discover('tests')
assert not loader.errors, loader.errors
actual = sys.modules['test_revision05'].CUT
expected = pathlib.Path.cwd().parents[2] / 'openspec/changes/sherlock-synthetic-rollup-data/design-cut.json'
assert actual == expected, (actual, expected)
print(json.dumps({'discovered': suite.countTestCases(), 'environment_cut_unset': True}))
"""
        result = subprocess.run([sys.executable, "-B", "-c", code], cwd=CODE, env=environment,
                                capture_output=True, text=True, timeout=10)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stderr, "")
        self.assertEqual(json.loads(result.stdout)["discovered"], 51)
    def test_real_cli_overflow_json_refuses_two_without_traceback(self):
        cut = Path(os.environ.get("FSYN_DESIGN_CUT", str(CODE.parents[2] / "openspec/changes/sherlock-synthetic-rollup-data/design-cut.json")))
        original = cut.read_bytes()
        with tempfile.TemporaryDirectory() as tmp:
            for overflow in ("1e999", "-1e999"):
                with self.subTest(overflow=overflow):
                    doc = json.loads(original)
                    row = next(iter(doc["groups"].values()))[0]
                    row["review_overflow_extra"] = "OVERFLOW_PLACEHOLDER"
                    text = json.dumps(doc).replace('"OVERFLOW_PLACEHOLDER"', overflow)
                    malformed = Path(tmp) / "overflow-cut.json"
                    malformed.write_text(text)
                    result = subprocess.run([sys.executable, "-B", str(CODE / "fsyn.py"), "cut", "check", "--cut", str(malformed)],
                                            cwd=CODE, capture_output=True, text=True, timeout=10)
                    self.assertEqual(result.returncode, 2, result.stderr)
                    self.assertEqual(result.stdout, "")
                    self.assertNotIn("Traceback", result.stderr)
                    self.assertEqual(json.loads(result.stderr)["status"], "refused")
        self.assertEqual(cut.read_bytes(), original)
    def test_finite_exponent_preserved_and_nested_overflow_rejected(self):
        self.assertEqual(parse_json(b'{"finite":1e100}')["finite"], 1e100)
        with self.assertRaises(Error): parse_json(b'{"nested":[{"value":1e999}]}')

if __name__ == "__main__": unittest.main()
