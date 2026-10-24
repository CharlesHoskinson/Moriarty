"""Requirement evidence memoization is local to one checker call."""
from pathlib import Path
import json
import os
import tempfile
import unittest
from unittest import mock

import test_requirement_evidence as fixture

qa = fixture.qa


class RequirementDigestCacheTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.repo = Path(temporary.name)
        original = self.repo / 'openspec/changes/sherlock-universal-interpreter-design/specs/universal-interpreter/spec.md'
        refinement = self.repo / 'openspec/changes/sherlock-completion-convergence/specs/sherlock-completion/spec.md'
        for path, prefix, count in ((original, 'UIREQ', 35), (refinement, 'SCON', 25)):
            path.parent.mkdir(parents=True, exist_ok=True)
            sections = []
            for index in range(1, count + 1):
                scenarios = 1 if prefix == 'UIREQ' else (3 if index <= 18 else 2)
                sections.append(f'### Requirement: {prefix}-{index:03} synthetic requirement\n' +
                                ''.join(f'#### Scenario: synthetic {number}\n' for number in range(scenarios)))
            path.write_text('\n'.join(sections))
        trace = self.repo / 'openspec/changes/sherlock-completion-convergence/traceability.json'
        trace.write_text(json.dumps({'original35': [{'id': f'UIREQ-{i:03}'} for i in range(1, 36)],
                                     'refinementFeatures': [{'id': f'SCON-{i:03}'} for i in range(1, 26)]}))
        self.f = fixture.RequirementEvidenceTests()
        with mock.patch.object(fixture, 'REPO', self.repo):
            self.f.setUp()
        self.addCleanup(self.f.doCleanups)

    def report(self):
        self.f.path.write_text(json.dumps(self.f.ledger))
        return qa.requirement_evidence_report(self.repo, self.f.path)

    def test_repeated_reference_hashes_once_but_next_call_observes_same_size_edit(self):
        f = self.f
        raw = f.source.read_bytes()
        stamp = f.source.stat()
        read_bytes = Path.read_bytes
        reads = []
        def tracked_read(path):
            if path == f.source:
                reads.append(path)
            return read_bytes(path)
        with mock.patch.object(qa, 'digest', wraps=qa.digest) as digest, \
                mock.patch.object(Path, 'read_bytes', tracked_read):
            first = self.report()
            self.assertEqual(sum(c.args[0] == raw for c in digest.call_args_list), 1)
            self.assertEqual(len(reads), 1)
            changed = b'X' + raw[1:]
            f.source.write_bytes(changed)
            os.utime(f.source, ns=(stamp.st_atime_ns, stamp.st_mtime_ns))
            second = self.report()
            self.assertEqual(sum(c.args[0] == changed for c in digest.call_args_list), 1)
            self.assertEqual(len(reads), 2)
        self.assertEqual(first['status'], 'current')
        self.assertNotEqual(second['status'], 'current')
        self.assertEqual(second['requirements'][0]['integrity'], 'stale')
        self.assertEqual(second['summary']['original35']['current_owner_declared_closed'], 0)

    def test_different_claimed_digests_compare_against_one_actual_digest(self):
        f = self.f
        f.ledger['requirements'][1]['currentness']['sourcePins'][0]['sha256'] = 'f'*64
        report = self.report()
        self.assertEqual(report['requirements'][0]['integrity'], 'current')
        self.assertEqual(report['requirements'][1]['integrity'], 'stale')


if __name__ == '__main__':
    unittest.main()
