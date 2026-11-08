"""Unit tests for scripts/qa/check-traceability.py (F14, SWF-002)."""
import contextlib
import io
from pathlib import Path
import shutil
import tempfile
import unittest
from test_profile_identity import ROOT, load

FIXTURES = Path(__file__).resolve().parent / 'fixtures' / 'traceability'
CHANGE = 'openspec/changes/demo'


class TraceabilityTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mod = load('check-traceability')

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name) / 'repo'
        shutil.copytree(FIXTURES / 'good', self.root)

    def tearDown(self):
        self.tmp.cleanup()

    def errors(self):
        return self.mod.check(self.root, CHANGE)[0]

    def edit(self, rel, old, new):
        path = self.root / rel
        text = path.read_text()
        self.assertIn(old, text)
        path.write_text(text.replace(old, new))

    def run_main(self, *args):
        out = io.StringIO()
        with contextlib.redirect_stdout(out):
            rc = self.mod.main(['--root', str(self.root), '--change', CHANGE, *args])
        return rc, out.getvalue()

    def test_good_fixture_passes(self):
        self.assertEqual(self.errors(), [])
        rc, out = self.run_main()
        self.assertEqual(rc, 0, out)
        self.assertIn('F14 PASS', out)

    def test_missing_id_fixture_fails(self):
        shutil.copy(FIXTURES / 'missing-id-traceability.md', self.root / CHANGE / 'traceability.md')
        self.assertIn('DEM-002: missing from traceability.md', self.errors())
        rc, out = self.run_main()
        self.assertEqual(rc, 1)
        self.assertIn('F14 FAIL', out)

    def test_requirement_only_in_requirements(self):
        self.edit(f'{CHANGE}/requirements.md', '| DEM-002 |', '| DEM-002 | E | x |\n| DEM-003 |')
        errors = self.errors()
        self.assertIn('DEM-003: in requirements.md but in no specs/*/spec.md', errors)
        self.assertIn('DEM-003: missing from traceability.md', errors)

    def test_requirement_only_in_spec(self):
        self.edit(f'{CHANGE}/specs/cap/spec.md', '### Requirement: DEM-002', '### Requirement: DEM-009')
        errors = self.errors()
        self.assertTrue(any(e.startswith('DEM-009: in openspec/changes/demo/specs/cap/spec.md') for e in errors), errors)
        self.assertIn('DEM-002: in requirements.md but in no specs/*/spec.md', errors)

    def test_unknown_id_in_traceability(self):
        self.edit(f'{CHANGE}/traceability.md', '| DEM-002 | 1.2 |', '| DEM-002 | 1.2 | F2 | M1 |\n| DEM-007 | 1.2 |')
        self.assertIn('traceability.md: unknown requirement ID DEM-007', self.errors())

    def test_unknown_predicate_in_traceability_is_a_warning(self):
        self.edit(f'{CHANGE}/traceability.md', '| F2, B1 |', '| F2, B1, A9m |')
        errors, warnings = self.mod.check(self.root, CHANGE)
        self.assertEqual(errors, [])
        self.assertEqual(len(warnings), 1)
        self.assertIn('A9m', warnings[0])

    def test_dangling_source_anchor(self):
        self.edit('qa/sherlock/predicates.yaml', 'requirements.md#dem-001', 'requirements.md#dem-005')
        self.assertIn('F1: source anchor does not resolve: openspec/changes/demo/requirements.md#dem-005', self.errors())

    def test_heading_anchor_must_name_a_requirement(self):
        self.edit('qa/sherlock/predicates.yaml', 'spec.md#requirement-dem-002-second-thing', 'spec.md#added-requirements')
        self.assertIn('F2: source anchor names no requirement of the change: '
                      'openspec/changes/demo/specs/cap/spec.md#added-requirements', self.errors())

    def test_line_anchor_out_of_range(self):
        self.edit('qa/sherlock/predicates.yaml', 'tasks.md#L3', 'tasks.md#L99')
        self.assertTrue(any('B1: source line anchor out of range' in e for e in self.errors()))

    def test_missing_source_file(self):
        self.edit('qa/sherlock/predicates.yaml', 'openspec/changes/demo/tasks.md#L3', 'nowhere.md#L1')
        self.assertIn('B1: source file missing: nowhere.md#L1', self.errors())

    def test_active_row_needs_source(self):
        self.edit('qa/sherlock/predicates.yaml', '    source: "openspec/changes/demo/tasks.md#L3"\n', '')
        self.assertIn('B1: no source: anchor', self.errors())

    def test_retired_row_keeps_historical_check(self):
        self.assertFalse(any(e.startswith('A1:') for e in self.errors()))
        self.edit('qa/sherlock/predicates.yaml', '    retired: "superseded by F1"\n', '')
        errors = self.errors()
        self.assertIn('A1: no source: anchor', errors)
        self.assertIn('A1: check names a missing path: scripts/qa/check-gone.sh', errors)

    def test_missing_check_script(self):
        self.edit('qa/sherlock/predicates.yaml', 'scripts/qa/check-f1.sh', 'scripts/qa/check-f9.sh')
        self.assertIn('F1: check names a missing path: scripts/qa/check-f9.sh', self.errors())

    def test_unimplemented_names_other_row(self):
        self.edit('qa/sherlock/predicates.yaml', 'unimplemented.sh F2', 'unimplemented.sh F3')
        self.assertIn("F2: unimplemented.sh names 'F3', not F2", self.errors())

    def test_task_names_unknown_requirement(self):
        self.edit(f'{CHANGE}/tasks.md', '[DEM-001] -> F1', '[DEM-004] -> F1')
        self.assertIn('tasks.md 1.1: unknown requirement ID DEM-004', self.errors())

    def test_task_range_is_expanded(self):
        self.edit(f'{CHANGE}/tasks.md', 'DEM-001..002', 'DEM-001..003')
        self.assertIn('tasks.md 1.2: unknown requirement ID DEM-003', self.errors())

    def test_task_names_unknown_predicate(self):
        self.edit(f'{CHANGE}/tasks.md', '-> F2, B1', '-> F2, B7')
        self.assertIn('tasks.md 1.2: unknown predicate row B7', self.errors())

    def test_render_adds_status_column_and_is_idempotent(self):
        status = self.root / 'qa/sherlock/status.json'
        status.write_text('{"predicates": {"F1": {"passes": true, "exit": 0}, "F2": {"passes": false, "exit": 2}}}')
        first = self.mod.render(self.root, CHANGE)
        self.assertIn('| DEM-001 | 1.1 | F1 | F1 pass | M1 |', first)
        self.assertIn('| DEM-002 | 1.2 | F2, B1 | F2 unimplemented; B1 not run, optional | M1 |', first)
        self.assertIn('| XYZ-001 | DEM-001 |', first)
        (self.root / CHANGE / 'traceability.md').write_text(first)
        self.assertEqual(self.mod.render(self.root, CHANGE), first)
        self.assertEqual(self.errors(), [])

    def test_repository_passes(self):
        errors, _ = self.mod.check(ROOT)
        self.assertEqual(errors, [])


if __name__ == '__main__':
    unittest.main()
