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

if __name__ == '__main__':
    unittest.main(verbosity=2)
