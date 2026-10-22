"""Per-assessment reuse preserves exact fingerprints and fresh dispatch checks."""
from collections import Counter
import hashlib
import json
from pathlib import Path
import unittest
from unittest import mock

import delivery
import reconciliation
import test_delivery as fixture
import test_reconciliation as reconciliation_fixture
import fixture_support


def legacy_fingerprint(plan, name):
    """Independent transcription of the original recursive fingerprint format."""
    task = plan.by_id[name]
    payload = {'task': task.id, 'adapter': task.adapter,
               'identity': json.loads(task.identity_json), 'inputs': task.inputs,
               'owner': task.owner, 'eligible_reviewers': task.eligible_reviewers,
               'integrator': plan.integrator, 'dependencies': [
                   {'task': dep, 'stage': stage,
                    'fingerprint': legacy_fingerprint(plan, dep),
                    'attempt': plan.by_id[dep].attempt} for dep, stage in task.dependencies]}
    return hashlib.sha256(json.dumps(payload, sort_keys=True, separators=(',', ':'),
                                    allow_nan=False).encode()).hexdigest()


class AssessmentCacheTests(unittest.TestCase):
    def setUp(self):
        self.f = fixture.DeliveryTests()
        self.f.setUp()
        self.addCleanup(self.f.doCleanups)

    def diamond(self):
        f = self.f
        f.plan['tasks'] += [
            f.task('left', dependencies=[{'task': 'build', 'stage': 'observed'}]),
            f.task('right', dependencies=[{'task': 'build', 'stage': 'observed'}]),
            f.task('leaf', dependencies=[{'task': 'left', 'stage': 'observed'},
                                         {'task': 'right', 'stage': 'observed'}]),
            f.task('independent')]
        # Exercise a non-topological input order, which the plan already supports.
        f.plan['tasks'].reverse()
        return delivery.load_plan(f.save()[0])

    def test_assessment_hashes_each_task_once_with_legacy_equivalent_bytes(self):
        plan = self.diamond()
        with mock.patch.object(delivery, '_digest', wraps=delivery._digest) as digest:
            report = delivery.assess(plan, ())
        self.assertEqual(digest.call_count, len(plan.tasks))
        for row in report['tasks']:
            self.assertEqual(row['fingerprint'], legacy_fingerprint(plan, row['id']))

    def test_shared_dependency_change_stales_descendants_and_preserves_independent(self):
        f = self.f
        plan = self.diamond()
        for name in plan.order:
            f.success(next(t for t in f.plan['tasks'] if t['id'] == name))
        before = {row['id']: row for row in f.inspect()['tasks']}
        shared = next(t for t in f.plan['tasks'] if t['id'] == 'build')
        shared['identity'] = {**shared['identity'], 'config': [f.file('edited-config', b'changed')]}
        after = {row['id']: row for row in f.inspect()['tasks']}
        for name in ('build', 'left', 'right', 'leaf'):
            self.assertNotEqual(before[name]['fingerprint'], after[name]['fingerprint'])
            self.assertEqual(after[name]['state'], 'stale')
        self.assertEqual(after['independent']['fingerprint'], before['independent']['fingerprint'])
        self.assertEqual(after['independent']['state'], 'observed')

    def test_repeated_inputs_are_read_once_and_next_assessment_reads_fresh(self):
        f = self.f
        f.plan['tasks'].append(f.task('independent'))
        plan = delivery.load_plan(f.save()[0])
        with mock.patch.object(delivery, '_read', wraps=delivery._read) as read:
            first = delivery.assess(plan, ())
            self.assertEqual(read.call_count, 3)
            Path(f.source['path']).write_bytes(b'changed source\n')
            second = delivery.assess(plan, ())
            self.assertEqual(read.call_count, 6)
        self.assertEqual(first['ready_for_dispatch'], ['build', 'independent'])
        self.assertEqual(second['ready_for_dispatch'], [])
        self.assertTrue(all(row['state'] == 'blocked-input' for row in second['tasks']))

    def test_cache_keys_include_pin_and_parse_mode_but_not_role(self):
        f = self.f
        # This config is checked once as an opaque input and once as JSON
        # cleanup evidence; changing only the role must reuse the opaque read.
        result = f.success()
        receipt = next(a for a in result['artifacts'] if a['role'] == 'resource-receipt')
        f.plan['tasks'][0]['identity']['config'].append({k: receipt[k] for k in ('path', 'bytes', 'sha256')})
        # The result is now historical, so this only tests opaque+JSON modes.
        plan = delivery.load_plan(f.save()[0])
        raw_result = delivery.load_result(result)
        with mock.patch.object(delivery, '_read', wraps=delivery._read) as read:
            delivery.assess(plan, (raw_result,))
        modes = [(call.args[0].path, call.kwargs.get('public_json', False)) for call in read.call_args_list]
        self.assertEqual(Counter(modes)[(receipt['path'], False)], 1)
        self.assertEqual(Counter(modes)[(receipt['path'], True)], 1)
        # Conflicting pins to the same path must never inherit a successful read.
        f.plan['tasks'].append(f.task('different-pin'))
        f.plan['tasks'][-1]['identity'] = {**f.plan['tasks'][-1]['identity'],
            'source': [{**f.source, 'sha256': 'f'*64}]}
        report = f.inspect()
        self.assertEqual(report['tasks'][-1]['state'], 'blocked-input')

    def test_input_and_artifact_roles_share_exact_opaque_read(self):
        f = self.f
        result = f.success()
        stdout = next(a for a in result['artifacts'] if a['role'] == 'stdout')
        stdout.update(f.config)
        plan = delivery.load_plan(f.save()[0])
        with mock.patch.object(delivery, '_read', wraps=delivery._read) as read:
            report = delivery.assess(plan, (delivery.load_result(result),))
        self.assertEqual(sum(c.args[0].path == f.config['path'] for c in read.call_args_list), 1)
        self.assertEqual(report['tasks'][0]['state'], 'observed')

    def test_reconciliation_reuses_same_attempt_cleanup_reads_in_one_call(self):
        f = reconciliation_fixture.ReconciliationTests()
        f.setUp()
        self.addCleanup(f.doCleanups)
        original = f._unknown_result()
        record = f._record(original)
        f.task['attempt'] = 'build-002'
        f.plan = delivery.load_plan(fixture_support.plan_file(f.root, [f.task]))
        result = delivery.load_result(original)
        receipt = next(a for a in result.artifacts if a.role == 'resource-receipt')
        with mock.patch.object(delivery, '_read', wraps=delivery._read) as read:
            report = reconciliation.assess_reconciled(f.plan, (result,), (record,))
        self.assertEqual(sum(c.args[0].path == receipt.path for c in read.call_args_list), 1)
        self.assertEqual(report['ready_for_dispatch'], ['build'])

    def test_independent_readiness_load_detects_intervening_artifact_edit(self):
        f = self.f
        result = f.success(adopt=True)
        paths = f.save()
        before = reconciliation.load_readiness(*paths)['report']['tasks'][0]
        self.assertTrue(before['acceptance_recorded'])
        Path(result['artifacts'][1]['path']).write_bytes(b'edited artifact')
        after = reconciliation.load_readiness(*paths)['report']['tasks'][0]
        self.assertEqual(after['state'], 'artifact-invalid')
        self.assertFalse(after['acceptance_recorded'])
        self.assertFalse(after['ready_for_dispatch'])


if __name__ == '__main__':
    unittest.main()
