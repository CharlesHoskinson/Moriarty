"""Local finalization tests: temporary Git repos, no product/native evidence."""
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile
import time
import unittest
from unittest.mock import patch
import cleanup


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


class CleanupTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name) / 'repo'
        self.root.mkdir()
        self.git('init', '-b', 'main')
        self.git('config', 'user.email', 'test@example.invalid')
        self.git('config', 'user.name', 'Test')
        (self.root / 'source.py').write_text('source\n')
        (self.root / '.gitignore').write_text('ignored\n')
        self.git('add', '.')
        self.git('commit', '-m', 'initial')
        self.tip = self.git('rev-parse', 'HEAD')
        self.git('branch', 'done')
        self.wt = Path(self.tmp.name) / 'done'
        self.git('worktree', 'add', str(self.wt), 'done')
        (self.root / 'graphify-out').mkdir()
        self.graph = self.root / 'graphify-out' / 'graph.json'
        self.graph.write_text('{"nodes":["source.py"]}\n')
        (self.root / 'wiki-llm').mkdir()
        self.wiki = self.root / 'wiki-llm' / 'cycles.md'
        self.wiki.write_text('Existing prefix\n')
        self.receipt = self.root / 'graphify-out' / 'cycle.json'
        self.sources = [{'path': 'source.py', 'sha256': digest(self.root / 'source.py')}]
        self.receipt.write_text(json.dumps({
            'schema': 'doctorwatson.graph-refresh/1', 'cycle_id': 'cycle-1',
            'affected_sources': self.sources, 'coverage': ['source.py'],
            'artifacts': [{'path': 'graphify-out/graph.json', 'sha256': digest(self.graph)}]}))
        self.request = {
            'schema': 'doctorwatson.cleanup-request/1', 'cycle_id': 'cycle-1',
            'affected_sources': self.sources,
            'graph_receipt': {'path': 'graphify-out/cycle.json', 'sha256': digest(self.receipt)},
            'wiki_path': 'wiki-llm/cycles.md', 'outcome': 'Host changes checked.',
            'lessons': ['Retain unknown work.'],
            'retained_roots': [{'ref': 'refs/heads/main', 'tip': self.tip}],
            'active_branches': [], 'active_worktrees': [],
            'inventory': {'captured_at': time.time(), 'repo_common_dir': str(self.root / '.git'),
                          'head': self.tip, 'open_prs_complete': True, 'leases_known': True,
                          'open_prs': [], 'lease_worktrees': []},
            'candidates': [{'branch': 'done', 'tip': self.tip,
                            'worktree': str(self.wt), 'head': self.tip, 'owner': 'Root', 'inactive': True}]}
        self.report = self.root / 'cleanup-report.json'

    def git(self, *args, cwd=None):
        p = subprocess.run(['git', '-C', str(cwd or self.root), *args],
                           text=True, capture_output=True, check=True)
        return p.stdout.strip()

    def run_cleanup(self, apply=True):
        return cleanup.finalize(self.root, self.request, self.report, apply=apply)

    def branch_exists(self):
        return subprocess.run(['git', '-C', str(self.root), 'show-ref', '--verify',
                               '--quiet', 'refs/heads/done']).returncode == 0

    def test_inspect_never_mutates_or_claims_complete(self):
        report = self.run_cleanup(False)
        self.assertEqual(report['status'], 'planned')
        self.assertEqual(report['dispositions'][0]['status'], 'eligible')
        self.assertTrue(self.branch_exists())
        self.assertEqual(self.wiki.read_text(), 'Existing prefix\n')

    def test_inactive_merged_clean_worktree_and_branch_retired(self):
        report = self.run_cleanup()
        self.assertEqual(report['status'], 'complete')
        self.assertFalse(self.branch_exists())
        self.assertFalse(self.wt.exists())
        self.assertEqual(report['dispositions'][0]['status'], 'deleted')
        self.assertTrue(self.wiki.read_text().startswith('Existing prefix\n'))
        self.assertEqual([p['name'] for p in report['phases']], ['graph', 'wiki', 'retirement', 'final'])

    def test_unmerged_unique_work_retained(self):
        (self.wt / 'new').write_text('unique')
        self.git('add', 'new', cwd=self.wt)
        self.git('commit', '-m', 'unique', cwd=self.wt)
        self.request['candidates'][0]['tip'] = self.request['candidates'][0]['head'] = self.git('rev-parse', 'HEAD', cwd=self.wt)
        report = self.run_cleanup()
        self.assertEqual(report['dispositions'][0]['status'], 'retained')
        self.assertIn('not reachable', report['dispositions'][0]['reason'])
        self.assertTrue(self.branch_exists())

    def test_dirty_untracked_ignored_each_protect_worktree(self):
        for name in ('source.py', 'untracked', 'ignored'):
            with self.subTest(name=name):
                path = self.wt / name
                original = path.read_bytes() if path.exists() else None
                path.write_text('new dirty bytes')
                report = self.run_cleanup()
                self.assertEqual(report['dispositions'][0]['status'], 'retained')
                self.assertIn('pristine', report['dispositions'][0]['reason'])
                self.assertTrue(self.wt.exists())
                path.unlink() if original is None else path.write_bytes(original)

    def test_active_lease_and_pr_branches_protected(self):
        for field, value in (
            ('active_branches', ['done']), ('active_worktrees', [str(self.wt)]),
        ):
            with self.subTest(field=field):
                self.request[field] = value
                self.assertEqual(self.run_cleanup()['dispositions'][0]['status'], 'retained')
                self.request[field] = []
        self.request['inventory']['lease_worktrees'] = [str(self.wt)]
        self.assertEqual(self.run_cleanup()['dispositions'][0]['status'], 'retained')
        self.request['inventory']['lease_worktrees'] = []
        self.request['inventory']['open_prs'] = [{'head': 'done', 'base': 'main'}]
        self.assertEqual(self.run_cleanup()['dispositions'][0]['status'], 'retained')
        self.assertTrue(self.branch_exists())

    def test_moved_tips_or_roots_refused(self):
        self.request['candidates'][0]['tip'] = '0' * 40
        self.assertEqual(self.run_cleanup()['dispositions'][0]['status'], 'blocked')
        self.request['candidates'][0]['tip'] = self.tip
        self.request['retained_roots'][0]['tip'] = '0' * 40
        report = self.run_cleanup()
        self.assertEqual(report['status'], 'incomplete')
        self.assertTrue(self.branch_exists())

    def test_graph_or_wiki_failure_prevents_deletion(self):
        self.graph.write_text('stale graph')
        report = self.run_cleanup()
        self.assertEqual(report['status'], 'incomplete')
        self.assertEqual(report['phases'][0]['status'], 'blocked')
        self.assertTrue(self.branch_exists())
        self.graph.write_text('{"nodes":["source.py"]}\n')
        self.wiki.unlink()
        self.wiki.mkdir()
        report = self.run_cleanup()
        self.assertEqual(report['status'], 'incomplete')
        self.assertTrue(self.branch_exists())

    def test_wiki_idempotent_and_changed_cycle_rejected(self):
        self.request['active_branches'] = ['done']
        self.run_cleanup()
        once = self.wiki.read_bytes()
        self.run_cleanup()
        self.assertEqual(self.wiki.read_bytes(), once)
        self.request['outcome'] = 'Different outcome'
        self.assertEqual(self.run_cleanup()['status'], 'incomplete')
        self.assertEqual(self.wiki.read_bytes(), once)

    def test_unknown_or_stale_inventory_prevents_retirement(self):
        for field, value in [('open_prs_complete', False), ('leases_known', False),
                             ('captured_at', time.time() - 901)]:
            original = self.request['inventory'][field]
            self.request['inventory'][field] = value
            report = self.run_cleanup()
            self.assertEqual(report['status'], 'incomplete')
            self.assertTrue(self.branch_exists())
            self.request['inventory'][field] = original

    def test_path_traversal_and_symlink_refused(self):
        self.request['wiki_path'] = '../outside.md'
        self.assertEqual(self.run_cleanup()['status'], 'incomplete')
        self.assertTrue(self.branch_exists())
        self.request['wiki_path'] = 'wiki-llm/link.md'
        (self.root / 'wiki-llm' / 'link.md').symlink_to(self.wiki)
        self.assertEqual(self.run_cleanup()['status'], 'incomplete')

    def test_source_change_after_wiki_blocks_retirement(self):
        original = cleanup._wiki
        def change_source(*args):
            result = original(*args)
            (self.root / 'source.py').write_text('changed during cycle')
            return result
        with patch.object(cleanup, '_wiki', side_effect=change_source):
            report = self.run_cleanup()
        self.assertEqual(report['status'], 'incomplete')
        self.assertTrue(self.branch_exists())
        self.assertTrue(self.wt.exists())

    def test_hardlinked_wiki_source_refused(self):
        self.wiki.unlink()
        self.wiki.hardlink_to(self.root / 'source.py')
        self.assertEqual(self.run_cleanup()['status'], 'incomplete')
        self.assertEqual((self.root / 'source.py').read_text(), 'source\n')
        self.assertTrue(self.branch_exists())

    def test_remote_tracking_retained_root_supports_merged_retirement(self):
        self.git('update-ref', 'refs/remotes/origin/sherlock', self.tip)
        self.request['retained_roots'] = [{'ref': 'refs/remotes/origin/sherlock', 'tip': self.tip}]
        self.assertEqual(self.run_cleanup()['status'], 'complete')
        self.assertFalse(self.branch_exists())
        self.assertEqual(self.git('rev-parse', 'refs/remotes/origin/sherlock'), self.tip)

    def test_root_revision_expression_refused_before_wiki_mutation(self):
        self.request['retained_roots'] = [{'ref': 'main~0', 'tip': self.tip}]
        self.assertEqual(self.run_cleanup()['status'], 'incomplete')
        self.assertEqual(self.wiki.read_text(), 'Existing prefix\n')
        self.assertTrue(self.branch_exists())

    def test_embedded_repository_metadata_is_preserved(self):
        nested = self.wt / 'nested'
        nested.mkdir()
        (nested / 'file.txt').write_text('tracked outer file\n')
        self.git('add', 'nested/file.txt', cwd=self.wt)
        self.git('commit', '-m', 'tracked nested directory', cwd=self.wt)
        self.git('merge', '--ff-only', 'done')
        tip = self.git('rev-parse', 'HEAD')
        self.request['inventory']['head'] = tip
        self.request['retained_roots'][0]['tip'] = tip
        self.request['candidates'][0]['tip'] = tip
        self.request['candidates'][0]['head'] = tip
        self.git('init', str(nested))
        private = nested / '.git' / 'private-fixture-data'
        private.write_text('untracked nested Git metadata\n')
        report = self.run_cleanup()
        self.assertTrue(private.exists())
        self.assertTrue(self.branch_exists())
        self.assertIn('embedded repository', report['dispositions'][0]['reason'])

    def test_inaccessible_worktree_scan_blocks_retirement(self):
        with patch.object(cleanup.os, 'walk', side_effect=PermissionError('metadata scan denied')):
            report = self.run_cleanup()
        self.assertEqual(report['status'], 'incomplete')
        self.assertTrue(self.branch_exists())
        self.assertTrue(self.wt.exists())

    def test_refresh_inventory_keeps_retirement_evidence(self):
        self.assertEqual(self.run_cleanup()['status'], 'complete')
        self.request['inventory']['captured_at'] = time.time()
        report = self.run_cleanup()
        self.assertEqual(report['status'], 'complete')
        self.assertTrue(report['dispositions'][0]['previously_deleted'])

    def test_inspect_preserves_apply_retirement_evidence(self):
        self.assertEqual(self.run_cleanup()['status'], 'complete')
        first = json.loads(self.report.read_text())
        self.run_cleanup(False)
        inspected = json.loads(self.report.read_text())
        self.assertEqual(first['retirement_receipts'], inspected['retirement_receipts'])
        self.assertEqual(self.run_cleanup()['status'], 'complete')

    def test_refreshed_pr_and_lease_inventory_is_rechecked_on_resume(self):
        self.assertEqual(self.run_cleanup()['status'], 'complete')
        self.request['inventory']['captured_at'] = time.time()
        self.request['inventory']['open_prs'] = [{'head': 'done', 'base': 'main'}]
        report = self.run_cleanup()
        self.assertIn('open-PR', report['dispositions'][0]['reason'])
        self.assertFalse(report['dispositions'][0].get('previously_deleted', False))
        self.request['inventory']['open_prs_complete'] = False
        self.assertEqual(self.run_cleanup()['status'], 'incomplete')

    def test_successful_apply_repeat_uses_exact_prior_retirement(self):
        first = self.run_cleanup()
        self.assertEqual(first['status'], 'complete')
        once = self.wiki.read_bytes()
        second = self.run_cleanup()
        self.assertEqual(second['status'], 'complete')
        self.assertIn('already retired', second['dispositions'][0]['reason'])
        self.assertEqual(self.wiki.read_bytes(), once)
        self.assertEqual(self.run_cleanup()['status'], 'complete')

    def test_recreated_different_branch_tip_never_reuses_prior_retirement(self):
        self.run_cleanup()
        self.git('checkout', '-b', 'new')
        (self.root / 'newfile').write_text('unique new bytes')
        self.git('add', 'newfile')
        self.git('commit', '-m', 'new unique')
        new_tip = self.git('rev-parse', 'HEAD')
        self.git('checkout', 'main')
        self.git('branch', 'done', new_tip)
        report = self.run_cleanup()
        self.assertEqual(report['status'], 'incomplete')
        self.assertEqual(self.git('rev-parse', 'done'), new_tip)

    def test_literal_cycle_marker_in_prose_remains_idempotent(self):
        self.request['active_branches'] = ['done']
        self.request['outcome'] = 'Prose <!-- doctorwatson-cycle:cycle-1: example'
        self.assertEqual(self.run_cleanup()['status'], 'complete')
        once = self.wiki.read_bytes()
        self.assertEqual(self.run_cleanup()['status'], 'complete')
        self.assertEqual(self.wiki.read_bytes(), once)

    def test_unknown_owner_cannot_attest_inactivity(self):
        self.request['candidates'][0].pop('owner')
        report = self.run_cleanup()
        self.assertEqual(report['dispositions'][0]['status'], 'blocked')
        self.assertEqual(report['status'], 'incomplete')
        self.assertTrue(self.branch_exists())

    def test_concurrent_apply_lock_refused_before_mutation(self):
        import fcntl
        with (self.root / '.git' / 'doctorwatson-cleanup.lock').open('w') as stream:
            fcntl.flock(stream, fcntl.LOCK_EX | fcntl.LOCK_NB)
            report = self.run_cleanup()
        self.assertEqual(report['status'], 'incomplete')
        self.assertIn('cleanup lock', report['error'])
        self.assertFalse(self.report.exists())
        self.assertEqual(self.wiki.read_text(), 'Existing prefix\n')
        self.assertTrue(self.branch_exists())

    def test_absent_lease_does_not_prove_candidate_inactive(self):
        self.request['candidates'][0].pop('inactive')
        report = self.run_cleanup()
        self.assertEqual(report['dispositions'][0]['status'], 'blocked')
        self.assertEqual(report['status'], 'incomplete')
        self.assertIn('inactivity', report['dispositions'][0]['reason'])
        self.assertTrue(self.branch_exists())

    def test_report_cannot_overwrite_graph_output(self):
        self.report = self.graph
        original = self.graph.read_bytes()
        self.assertEqual(self.run_cleanup()['status'], 'incomplete')
        self.assertEqual(self.graph.read_bytes(), original)
        self.assertTrue(self.branch_exists())

    def test_atomic_ref_delete_refuses_moved_root(self):
        self.git('worktree', 'remove', str(self.wt))
        self.request['candidates'][0].pop('worktree')
        self.request['retained_roots'][0]['tip'] = '0' * 40
        with self.assertRaises(cleanup.CleanupError):
            cleanup._delete_ref(self.root, self.request, self.request['candidates'][0])
        self.assertTrue(self.branch_exists())

    def test_atomic_ref_delete_refuses_moved_candidate(self):
        self.git('worktree', 'remove', str(self.wt))
        self.request['candidates'][0].pop('worktree')
        self.request['candidates'][0]['tip'] = '0' * 40
        with self.assertRaises(cleanup.CleanupError):
            cleanup._delete_ref(self.root, self.request, self.request['candidates'][0])
        self.assertTrue(self.branch_exists())

    def test_partial_worktree_removal_failure_is_reported(self):
        def refuse(*args):
            raise cleanup.CleanupError('exact ref transaction failed')
        with patch.object(cleanup, '_delete_ref', side_effect=refuse):
            report = self.run_cleanup()
        self.assertEqual(report['status'], 'incomplete')
        row = report['dispositions'][0]
        self.assertTrue(row['worktree_removed'])
        self.assertFalse(row['branch_deleted'])
        self.assertIn('exact ref transaction failed', row['reason'])
        self.assertTrue(self.branch_exists())

    def test_report_failure_cannot_claim_completion_or_delete(self):
        with patch.object(cleanup, '_save_report', side_effect=OSError('disk full')):
            with self.assertRaises(OSError):
                self.run_cleanup()
        self.assertTrue(self.branch_exists())
        self.assertEqual(self.wiki.read_text(), 'Existing prefix\n')

    def test_report_failure_after_graph_keeps_persisted_status_incomplete(self):
        original = cleanup._save_report
        calls = 0
        def fail_later(path, report):
            nonlocal calls
            calls += 1
            if calls > 1:
                raise OSError('disk full')
            original(path, report)
        with patch.object(cleanup, '_save_report', side_effect=fail_later):
            with self.assertRaises(OSError):
                self.run_cleanup()
        self.assertEqual(json.loads(self.report.read_text())['status'], 'incomplete')
        self.assertTrue(self.branch_exists())


if __name__ == '__main__':
    unittest.main()
