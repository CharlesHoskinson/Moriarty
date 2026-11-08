"""Focused discovery/JSON overflow regressions; no old test cohort execution."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from common import Error, parse_json
import golden

CODE = Path(__file__).resolve().parents[1]

class Revision06Tests(unittest.TestCase):
    def test_real_discovery_reads_only_the_in_repository_golden(self):
        environment = {k: v for k, v in os.environ.items() if not k.startswith(("FSYN_", "FRTF_"))}
        code = """import json, pathlib, unittest, sys
loader = unittest.TestLoader()
suite = loader.discover('tests')
assert not loader.errors, loader.errors
actual = sys.modules['test_revision05'].PIN
expected = pathlib.Path.cwd() / 'golden/s0.json'
assert actual == expected, (actual, expected)
print(json.dumps({'discovered': suite.countTestCases()}))
"""
        result = subprocess.run([sys.executable, "-B", "-c", code], cwd=CODE, env=environment,
                                capture_output=True, text=True, timeout=10)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stderr, "")
        self.assertEqual(json.loads(result.stdout)["discovered"], 56)
    def test_overflow_json_pin_refuses_without_traceback(self):
        original = golden.PIN.read_bytes()
        with tempfile.TemporaryDirectory() as tmp:
            for overflow in ("1e999", "-1e999"):
                with self.subTest(overflow=overflow):
                    doc = json.loads(original)
                    doc["review_overflow_extra"] = "OVERFLOW_PLACEHOLDER"
                    malformed = Path(tmp) / "overflow.json"
                    malformed.write_text(json.dumps(doc).replace('"OVERFLOW_PLACEHOLDER"', overflow))
                    with self.assertRaisesRegex(Error, "non-finite"): golden.load(golden.ROOT, malformed)
        self.assertEqual(golden.PIN.read_bytes(), original)
    def test_finite_exponent_preserved_and_nested_overflow_rejected(self):
        self.assertEqual(parse_json(b'{"finite":1e100}')["finite"], 1e100)
        with self.assertRaises(Error): parse_json(b'{"nested":[{"value":1e999}]}')

if __name__ == "__main__": unittest.main()
