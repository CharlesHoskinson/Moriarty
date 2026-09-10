"""Document structure controls, not an executable language semantic test suite."""
import copy
import importlib.util
import json
from pathlib import Path
import unittest

HERE = Path(__file__).resolve().parent

class ContractTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.validator_path = HERE / 'check-expression-contract.py'
        if cls.validator_path.exists():
            spec = importlib.util.spec_from_file_location('contract_checker', cls.validator_path)
            cls.module = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(cls.module)
        else:
            cls.module = None

    def inputs(self):
        self.assertIsNotNone(self.module, 'missing focused structural contract validator')
        return (json.loads((HERE/'expression-signatures.json').read_text()),
                json.loads((HERE/'expression-cases.json').read_text()))

    def test_complete_contract(self):
        c, cases = self.inputs()
        self.assertEqual([], self.module.validate(c, cases))

    def test_missing_constructor(self):
        c, cases = self.inputs(); c['constructors'].pop()
        self.assertIn('constructor-set', self.module.validate(c, cases))

    def test_duplicate_constructor(self):
        c, cases = self.inputs(); c['constructors'].append(copy.deepcopy(c['constructors'][0]))
        self.assertIn('duplicate-constructor', self.module.validate(c, cases))

    def test_dangling_type(self):
        c, cases = self.inputs(); c['constructors'][0]['result'] = 'AbsentType'
        self.assertIn('dangling-type:LitUInt', self.module.validate(c, cases))

    def test_dangling_operand(self):
        c, cases = self.inputs(); c['constructors'][0]['evaluatedOperands'] = ['absent']
        self.assertIn('dangling-operand:LitUInt', self.module.validate(c, cases))

    def test_missing_case(self):
        c, cases = self.inputs(); cases['cases'] = [x for x in cases['cases'] if x['constructor'] != 'Ensure']
        self.assertIn('case-pair:Ensure', self.module.validate(c, cases))

    def test_dangling_rule(self):
        c, cases = self.inputs(); c['constructors'][0]['rule'] = 'ABSENT'
        self.assertIn('dangling-rule:LitUInt', self.module.validate(c, cases))

    def test_dangling_source(self):
        c, cases = self.inputs(); c['constructors'][0]['sources'] = ['ABSENT']
        self.assertIn('dangling-source:LitUInt', self.module.validate(c, cases))

    def test_unknown_case_constructor(self):
        c, cases = self.inputs(); cases['cases'][0]['constructor'] = 'Invented'
        self.assertIn('unknown-case-constructor', self.module.validate(c, cases))

    def test_duplicate_case_identity(self):
        c, cases = self.inputs(); cases['cases'].append(copy.deepcopy(cases['cases'][0]))
        self.assertIn('duplicate-case', self.module.validate(c, cases))

    def test_strict_version_rejected(self):
        c, cases = self.inputs(); c['schemaVersion'] = 'moriarty-expression-contract/0'
        self.assertIn('claim-scope', self.module.validate(c, cases))

    def test_eager_boolean_metadata_rejected(self):
        c, cases = self.inputs()
        row = next(r for r in c['constructors'] if r['constructor'] == 'And')
        row['evaluatedOperands'] = ['left', 'right']
        self.assertIn('boolean-evaluation-metadata:And', self.module.validate(c, cases))

    def test_skipped_static_operand_rejected(self):
        c, cases = self.inputs()
        row = next(r for r in c['constructors'] if r['constructor'] == 'Or')
        row['staticOperands'] = ['left']
        self.assertIn('boolean-evaluation-metadata:Or', self.module.validate(c, cases))

    def test_wrong_selection_predicate_rejected(self):
        c, cases = self.inputs()
        row = next(r for r in c['constructors'] if r['constructor'] == 'Or')
        row['conditionalEvaluatedOperands'][0]['when'] = 'left=true'
        self.assertIn('boolean-evaluation-metadata:Or', self.module.validate(c, cases))

    def test_reused_strict_positive_id_rejected(self):
        c, cases = self.inputs()
        row = next(r for r in c['constructors'] if r['constructor'] == 'And')
        row['caseIds'][0] = 'And-positive'
        self.assertIn('case-reference:And', self.module.validate(c, cases))

    def supplement(self):
        return json.loads((HERE/'boolean-cases.json').read_text())

    def test_complete_boolean_fixture_structure(self):
        c, _ = self.inputs()
        self.assertEqual([], self.module.validate_booleans(c, self.supplement()))

    def test_missing_truth_row_rejected(self):
        c, _ = self.inputs(); b = self.supplement()
        b['cases'] = [r for r in b['cases'] if r['id'] != 'BC1-truth-and-true-true']
        self.assertIn('boolean-truth-coverage', self.module.validate_booleans(c, b))

    def test_unentered_path_not_a_charge(self):
        c, _ = self.inputs(); b = self.supplement()
        row = next(r for r in b['cases'] if r['id'] == 'BC1-work2-select-and')
        row['enteredNodePaths'].append([1])
        self.assertTrue(any('specified-entry-accounting' in e for e in self.module.validate_booleans(c, b)))

    def test_rebased_selected_span_rejected(self):
        c, _ = self.inputs(); b = self.supplement()
        row = next(r for r in b['cases'] if r['id'] == 'BC1-source-selected-work-exhaustion')
        row['expected']['span'] = row['core']['span']
        self.assertTrue(any('rejection-span-provenance' in e for e in self.module.validate_booleans(c, b)))

    def test_metadata_checks_do_not_evaluate_truth(self):
        c, _ = self.inputs(); b = self.supplement()
        row = next(r for r in b['cases'] if r['id'] == 'BC1-truth-and-true-true')
        row['expected']['value'] = False
        self.assertEqual([], self.module.validate_booleans(c, b),
                         'This checker is structural; independent review must catch wrong semantic values.')

if __name__ == '__main__':
    unittest.main(verbosity=2)
