"""Normal data-carrier QA only; independent K lane expectations remain opaque."""
from pathlib import Path
import sys
import unittest
sys.path.insert(0,str(Path(__file__).resolve().parents[1]/'source'))
import root_data as m

class DataTests(unittest.TestCase):
    def setUp(self):self.doc=m.base.loads(m.base.checked(m.INPUT_PIN))
    def test_complete_input_KJSON_inverse(self):
        for row in self.doc['cases']:
            term=m.from_tree(row['root']);actual=m.decode_document(m.envelope(term),'CTRoot')
            self.assertTrue(m.base.same(m.base.tree(actual),row['root']))
    def test_full_17_case_source_query_inverse(self):
        before=dict(m.base.codec.REG)
        term,wire=m.carrier(self.doc)
        self.assertEqual(m.base.codec.REG,before)
        self.assertEqual(len(self.doc['cases']),17)
        self.assertEqual(term.name,'ctObserveAll')
        self.assertEqual(wire['term']['label']['name'],m.LABELS['ctObserveAll'])
    def test_distinct_optional_sorts(self):
        self.assertEqual(m.DATA_REG['ctEnter'][1][0],'CTFunction')
        self.assertEqual(m.DATA_REG['ctEnvSome'][0],'CTEnv')
        self.assertEqual(m.DATA_REG['ctFunctionSome'][0],'CTFunction')
        self.assertNotEqual(m.LABELS['ctEnvSome'],m.LABELS['ctFunctionSome'])
    def test_all67_actual_compiled_label_members(self):
        data=m.base.loads(m.base.checked(m.LABEL_MAP_PIN));forward,reverse=m.validated_labels()
        self.assertEqual(len(forward),67);self.assertEqual(len(reverse),67)
        lines=m.base.checked(data['definition']).decode('utf-8').splitlines()
        for row in data['constructors']:
            self.assertEqual(lines[row['declaration_line']-1],row['complete_symbol_declaration'])
            self.assertEqual(m.kore_label(row['compiled_klabel']),row['kore_symbol'])
    def test_actual_decoder_registry_is_data_only(self):
        self.assertEqual(set(m.base.codec.REG),set(m.DATA_REG))
        self.assertNotIn('ctObserveAll',m.base.codec.REG)
        self.assertNotIn('ctDecodeRoot',m.base.codec.REG)
    def test_wide_native_integers_retained(self):
        root=self.doc['cases'][0]['root'];term=m.from_tree(root)
        self.assertTrue(m.base.same(m.base.tree(term),root))
        self.assertIs(type(root['arguments'][5]['value']),int)
        self.assertGreater(root['arguments'][5]['value'],2**64)

    @staticmethod
    def deep_lanes(leaf):
        # Loop-built independent synthetic typed carriers; no actual/golden input.
        tail={'sort':'CTLanes','constructor':'ctNoLanes','arguments':[]}
        for i in range(600):
            head=dict(leaf) if i == 0 else {'sort':'Int','value':i}
            tail={'sort':'CTLanes','constructor':'ctLane','arguments':[head,tail]}
        return tail
    def test_whole_deep_equal_independent_trees(self):
        left=self.deep_lanes({'sort':'Int','value':7})
        right=self.deep_lanes({'sort':'Int','value':7})
        self.assertIsNot(left,right)
        self.assertTrue(m.same_whole(left,right))
    def test_whole_deepest_scalar_difference(self):
        left=self.deep_lanes({'sort':'Int','value':7})
        right=self.deep_lanes({'sort':'Int','value':8})
        self.assertFalse(m.same_whole(left,right))
    def test_whole_deepest_bool_int_kind_difference(self):
        left=self.deep_lanes({'sort':'Bool','value':True})
        right=self.deep_lanes({'sort':'Int','value':1})
        self.assertFalse(m.same_whole(left,right))
        # Also isolate the native scalar-kind rule from the sort-name difference.
        self.assertFalse(m.same_whole(self.deep_lanes({'sort':'Int','value':True}),right))

if __name__=='__main__':unittest.main()
