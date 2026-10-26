import unittest

import export_plans
import residualize


class ExportPlans(unittest.TestCase):
    def test_separate_dispositions_and_no_witness_from_json(self):
        r = export_plans.prepare('constraint-json-basic')
        d = r['disposition']
        self.assertEqual(d['witnessProgramExport']['status'], 'Unsupported')
        self.assertFalse(r['claims']['constraintJsonSuppliesWitnessComputation'])
        self.assertNotEqual(r['constraintPlan']['planId'], r['witnessPlan']['planId'])

    def test_native_lookup_interaction_unsupported(self):
        r = export_plans.prepare('structured-witness-basic')
        feats = {u['feature'] for u in r['disposition']['witnessProgramExport']['unsupported']}
        self.assertTrue({'native-witness-closure', 'lookup', 'interaction'} <= feats)
        self.assertEqual(r['tooling_status'], 'Unsupported')

    def test_midnight_not_claimed(self):
        r = export_plans.prepare('midnight-zkir')
        self.assertEqual(r['claims']['midnightZkirExporter'], 'not-implemented')
        self.assertEqual(r['disposition']['constraintExport']['status'], 'Unsupported')


class Residualize(unittest.TestCase):
    def test_recursion_stays_runtime_call(self):
        for ir in residualize.report()['realizations']:
            for b in ir['blocks']:
                if b['kind'] == 'runtime-call':
                    self.assertTrue(b['backEdge'] and not b['unrolled'])

    def test_both_branch_successors_and_dynamic(self):
        ir = residualize.specialize('empty')
        cond = [b for b in ir['blocks'] if b['kind'] == 'dynamic-branch'][0]
        self.assertEqual(cond['targets'], ['R_then', 'R_else'])
        self.assertIn('fuel-work', ir['dynamicInputs'])

    def test_limit_falls_back(self):
        ir = residualize.specialize('empty', limit=2)
        self.assertTrue(ir['genericFallbacks'])

    def test_family_keeps_dynamic_call_path(self):
        ir = residualize.specialize('family', family='arith')
        fetch = [b for b in ir['blocks'] if b['rule'] == 'R_fetch'][0]
        self.assertEqual(fetch['targets'], ['R_cond', 'R_call'])
        self.assertEqual(fetch['genericTargets'], ['R_call'])
        self.assertTrue(ir['genericFallbacks'])

    def test_masks_share_rule_map(self):
        a = residualize.specialize('empty')['sourceOccurrenceMap']
        b = residualize.specialize('image', image='img-sum')['sourceOccurrenceMap']
        self.assertEqual(a, b)


if __name__ == '__main__':
    unittest.main()
