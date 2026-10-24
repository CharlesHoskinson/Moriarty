"""Dispatch facts are additive to the existing qualified candidate view."""
import unittest

import delivery
import test_delivery as fixture
import test_reconciliation as reconciliation_fixture


class CapabilityTests(unittest.TestCase):
    def setUp(self):
        self.f = fixture.DeliveryTests()
        self.f.setUp()
        self.addCleanup(self.f.doCleanups)

    def test_dispatch_view_qualifies_unobserved_preflight(self):
        report = self.f.inspect()
        self.assertEqual(report['ready_for_dispatch'], ['build'])
        self.assertEqual(report['dispatch_preflight'], 'not-observed')
        self.assertEqual(report['dispatch_readiness_scope'],
                         'dependencies-and-adapter-capabilities')
        paths = self.f.save()
        reconciled = reconciliation_fixture.rec.load_readiness(*paths)['report']
        self.assertEqual(reconciled['dispatch_preflight'], 'not-observed')
        self.assertEqual(reconciled['dispatch_readiness_scope'],
                         report['dispatch_readiness_scope'])

    def test_capabilities_are_pure_fresh_facts(self):
        import capabilities
        expected = {
            'moriarty-sdk-build': (True, True),
            'handkey-verify-pis': (True, True),
            'sherlock-anchor-settle-readback': (False, True),
        }
        for adapter in (*delivery.ROLES, 'unknown'):
            with self.subTest(adapter=adapter):
                cap = capabilities.adapter_capability(adapter)
                self.assertEqual((cap['executor_available'], cap['collector_available']),
                                 expected.get(adapter, (False, False)))
                self.assertEqual(cap['blocked_reason'] is None, cap['executor_available'])
                cap['executor_available'] = 'mutated'
                self.assertIsInstance(capabilities.adapter_capability(adapter)['executor_available'], bool)

    def test_unsupported_candidate_is_visible_but_cannot_dispatch(self):
        f = self.f
        f.plan['tasks'] += [f.task('compact', 'compact-anchor-build9'),
                            f.task('tests', 'sdk-selected-tests', inputs={'selectors': ['x']})]
        report = f.inspect()
        self.assertEqual(report['ready_for_root_selection'], ['build', 'compact', 'tests'])
        self.assertEqual(report['ready_for_dispatch'], ['build'])
        for row in report['tasks'][1:]:
            self.assertTrue(row['dependency_ready'])
            self.assertFalse(row['executor_available'])
            self.assertFalse(row['collector_available'])
            self.assertFalse(row['ready_for_dispatch'])
            self.assertTrue(row['dispatch_blockers'])
        self.assertFalse(report['execution_authority'])

    def test_handkey_and_collect_only_settle_are_distinct(self):
        f = self.f
        f.plan['tasks'] = [f.task('verify', 'handkey-verify-pis', inputs={k: 'a'*64 for k in delivery.HANDKEY_PINS}),
                           f.task('settle', 'sherlock-anchor-settle-readback', inputs=f.settlement_inputs())]
        report = f.inspect()
        self.assertEqual(report['ready_for_dispatch'], ['verify'])
        settle = report['tasks'][1]
        self.assertTrue(settle['collector_available'])
        self.assertFalse(settle['executor_available'])

    def test_accepted_outcome_is_not_dispatched_again(self):
        self.f.success(adopt=True)
        row = self.f.row()
        self.assertTrue(row['acceptance_recorded'])
        self.assertFalse(row['dependency_ready'])
        self.assertFalse(row['ready_for_dispatch'])
        self.assertTrue(row['dispatch_blockers'])

    def test_unknown_debt_failure_and_source_outcomes_never_dispatch(self):
        f = self.f
        result = f.success()
        for update in ({'terminal': 'unknown-submission', 'submission': 'unknown'},
                       {'cleanup': 'owed'}, {'terminal': 'failed', 'exit_code': 1}):
            old = result.copy()
            result.update(update)
            with self.subTest(update=update):
                self.assertFalse(f.row()['ready_for_dispatch'])
                self.assertEqual(f.inspect()['ready_for_dispatch'], [])
            result.clear(); result.update(old)
        f.plan['tasks'] = [f.task('source', 'source-integration')]
        f.history['results'].clear()
        f.success()
        self.assertEqual(f.row('source')['state'], 'source-integrated')
        self.assertFalse(f.row('source')['ready_for_dispatch'])

    def test_capability_view_is_refreshed_after_reconciliation(self):
        f = reconciliation_fixture.ReconciliationTests()
        f.setUp()
        self.addCleanup(f.doCleanups)
        original = f._unknown_result()
        record = f._record(original)
        report = f._assess(original, [record])
        self.assertEqual(report['ready_for_root_selection'], ['build'])
        self.assertEqual(report['ready_for_dispatch'], ['build'])
        self.assertTrue(report['tasks'][0]['dependency_ready'])


if __name__ == '__main__':
    unittest.main()
