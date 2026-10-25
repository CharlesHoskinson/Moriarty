"""Mutation tests for the UIREQ-024 optimization record check (no cargo, no git writes)."""
import copy
import hashlib
import json
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parent))
import optimization_record as opt  # noqa: E402

REPO = Path(__file__).resolve().parents[3]
TRACE = ("running 2 tests\ntest a_positive ... ok\ntest b_rejects ... ok\n"
         "test result: ok. 2 passed; 0 failed; 0 ignored\n")
SHA = 'a' * 40
TEST_SRC = "#[test]\nfn a_positive() {}\n#[test]\nfn b_rejects() {}\n"
CMD = 'cargo test -p mori-core --test t -- --test-threads=1'


class FakeRepo:
    def __init__(self, files, runs, ancestor=True, unchanged=True):
        self.files, self._runs = files, runs
        self.ancestor, self.unchanged = ancestor, unchanged

    def read(self, rel):
        return self.files.get(rel)

    def runs(self):
        return self._runs

    def git(self, *args):
        ok = self.ancestor if args[0] == 'merge-base' else self.unchanged
        return type('R', (), {'returncode': 0 if ok else 1})()


def fixture():
    record = json.loads((REPO / opt.DEFAULT_RECORD).read_text())
    e1 = record['evidence'][0]
    e1.update(commit=SHA, run='R1', trace='qa/sherlock/traces/R1.log',
              trace_sha256=hashlib.sha256(TRACE.encode()).hexdigest(), command=CMD,
              file='crates/x/tests/t.rs', tests=['a_positive', 'b_rejects'])
    files = {record['candidate']['module']: 'pub fn check_preservation() {}',
             e1['trace']: TRACE, e1['file']: TEST_SRC}
    runs = [dict(run='R1', exit=0, head=SHA, cmd=CMD, trace=e1['trace'])]
    return record, files, runs


class RecordCheckTests(unittest.TestCase):
    def run_check(self, record, files, runs, **kw):
        return opt.check(record, FakeRepo(files, runs, **kw))

    def test_positive_record_verifies(self):
        rep = self.run_check(*fixture())
        self.assertEqual(rep['status'], 'predicate-stated-and-evidence-verified')
        self.assertEqual([c['observation'] for c in rep['clauses']], list(opt.CLAUSES))
        self.assertEqual(rep['acceptance'], 'none')

    def assert_rejected(self, mutate, **kw):
        record, files, runs = fixture()
        mutate(record, files, runs)
        with self.assertRaises((opt.RecordError, opt.Malformed)):
            self.run_check(record, files, runs, **kw)

    def test_rejects_missing_clause(self):
        for name in opt.CLAUSES:
            self.assert_rejected(lambda r, f, x, n=name: r.__setitem__(
                'clauses', [c for c in r['clauses'] if c['observation'] != n]))

    def test_rejects_evidence_gap_overlap(self):
        self.assert_rejected(lambda r, f, x: r['unresolved_gaps'].append(
            dict(id='E1', clauses=['values'], statement='s')))

    def test_rejects_clause_without_predicate(self):
        self.assert_rejected(lambda r, f, x: r['clauses'][0].__setitem__('predicate', ''))

    def test_rejects_edited_trace(self):
        self.assert_rejected(lambda r, f, x: f.__setitem__('qa/sherlock/traces/R1.log', TRACE + 'x'))

    def test_rejects_failed_run(self):
        self.assert_rejected(lambda r, f, x: x[0].__setitem__('exit', 1))

    def test_rejects_no_run_command(self):
        def m(r, f, x):
            r['evidence'][0]['command'] = x[0]['cmd'] = 'cargo test --no-run'
        self.assert_rejected(m)

    def test_rejects_test_not_in_trace(self):
        self.assert_rejected(lambda r, f, x: r['evidence'][0]['tests'].append('c_missing'))

    def test_rejects_changed_files_or_foreign_commit(self):
        self.assert_rejected(lambda r, f, x: None, unchanged=False)
        self.assert_rejected(lambda r, f, x: None, ancestor=False)

    def test_rejects_unknown_cost_without_gap(self):
        self.assert_rejected(lambda r, f, x: r['costs'][-1].pop('gap'))

    def test_rejects_wrong_candidate_kind(self):
        self.assert_rejected(lambda r, f, x: r['candidate'].__setitem__('kind', 'cache'))

    def test_malformed_format(self):
        record, files, runs = fixture()
        record = copy.deepcopy(record)
        record['format'] = 'x'
        with self.assertRaises(opt.Malformed):
            self.run_check(record, files, runs)


if __name__ == '__main__':
    unittest.main()
