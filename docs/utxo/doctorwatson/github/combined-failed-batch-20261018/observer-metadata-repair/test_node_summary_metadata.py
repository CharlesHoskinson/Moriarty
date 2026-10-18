"""Pure metadata tests; captured SPEC is real, TAP/mutations are synthetic.
Never imports or executes the driver module or any product process.
"""
import ast
from pathlib import Path
import re
import sys
import unittest

D = Path(__file__).resolve().parent
SOURCE = Path(sys.argv.pop(1))
tree = ast.parse(SOURCE.read_text())
functions = [n for n in tree.body if isinstance(n, ast.FunctionDef) and n.name == 'node_summary']
if functions:
    namespace = {'re': re}
    exec(compile(ast.Module(body=functions, type_ignores=[]), str(SOURCE), 'exec'), namespace)
    parse = namespace['node_summary']
else:
    # Reproduce the exact old counts expression, without running worker().
    worker = next(n for n in tree.body if isinstance(n, ast.FunctionDef) and n.name == 'worker')
    assignment = next(n for n in ast.walk(worker) if isinstance(n, ast.Assign)
                      and any(isinstance(t, ast.Name) and t.id == 'counts' for t in n.targets))
    expression = compile(ast.Expression(assignment.value), str(SOURCE), 'eval')
    def parse(text):
        return eval(expression, {'re': re, 'text': text})

SPEC = (D / 'actual/node-capped-public-consumer/STDOUT.log').read_text()
EXPECTED = {'tests': 2, 'pass': 2, 'fail': 0}
TAP = 'TAP version 13\n1..2\n# tests 2\n# pass 2\n# fail 0\n'

class MetadataSummary(unittest.TestCase):
    def test_actual_full_two_case_spec_capture(self):
        self.assertEqual(parse(SPEC), EXPECTED)

    def test_synthetic_supported_tap(self):
        self.assertEqual(parse(TAP), EXPECTED)

    def test_missing_and_malformed_counts_are_rejected(self):
        for field, value in EXPECTED.items():
            for prefix, text in [('ℹ', SPEC), ('#', TAP)]:
                line = f'{prefix} {field} {value}\n'
                for replacement in ['', f'{prefix} {field} invalid\n']:
                    with self.subTest(field=field, prefix=prefix, replacement=replacement):
                        with self.assertRaises((ValueError, AssertionError)):
                            parse(text.replace(line, replacement))

    def test_duplicate_conflicting_and_mixed_reporters_are_rejected(self):
        for text, prefix in [(SPEC, 'ℹ'), (TAP, '#')]:
            for field, value in EXPECTED.items():
                for other in [value, value + 1]:
                    with self.subTest(prefix=prefix, field=field, other=other):
                        with self.assertRaises((ValueError, AssertionError)):
                            parse(text + f'{prefix} {field} {other}\n')
        for text in [SPEC + TAP, TAP.replace('# pass', 'ℹ pass'), SPEC + 'ℹ pass invalid\n', TAP + '# fail\n']:
            with self.assertRaises((ValueError, AssertionError)):
                parse(text)

    def test_nonexpected_totals_or_failures_are_rejected(self):
        for prefix, text in [('ℹ', SPEC), ('#', TAP)]:
            for field, value in EXPECTED.items():
                for other in [0, 1, 3]:
                    if other != value:
                        with self.subTest(prefix=prefix, field=field, other=other):
                            with self.assertRaises((ValueError, AssertionError)):
                                parse(text.replace(f'{prefix} {field} {value}\n', f'{prefix} {field} {other}\n'))

if __name__ == '__main__':
    unittest.main()
