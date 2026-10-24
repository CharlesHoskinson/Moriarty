"""Source-first positive interface tests. UNRUN; no runtime/codec oracle."""
import sys
import unittest
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'source'))
import helper_ports as m

class PortTests(unittest.TestCase):
    def test_all_distinct_option_sorts_preserve_some_zero(self):
        for kind, prefix in [('CHIndex','Index'),('CHResume','Resume'),('CHType','Type'),('CTEnv','Env')]:
            self.assertEqual(m.port_value(kind, {'None': None})['constructor'], ('ct' if kind=='CTEnv' else 'ch')+prefix+'None')
            value=m.port_value(kind, {'Some': 0})
            self.assertEqual(value['constructor'], ('ct' if kind=='CTEnv' else 'ch')+prefix+'Some')
            self.assertEqual(value['arguments'], [{'sort':'Int','value':0}])
    def test_reference_presence_and_const_data_are_separate(self):
        self.assertEqual(m.port_value('CHOptRef',{'None':None})['constructor'],'chRefNone')
        left=m.port_value('CHOptRef',{'Some':{'Const':0}})
        right=m.port_value('CHOptRef',{'Some':{'Data':0}})
        self.assertEqual(left['arguments'][0]['constructor'],'ctConst')
        self.assertEqual(right['arguments'][0]['constructor'],'ctData')
    def test_raw_empty_slice_retains_nonzero_start(self):
        self.assertEqual(m.port_value('CTSlice',{'start':19,'len':0}),
            {'sort':'CTSlice','constructor':'ctSlice','arguments':[{'sort':'Int','value':19},{'sort':'Int','value':0}]})
    def test_fullwidth_common_origin_and_resume_sort(self):
        common=m.port_value('CHCommon',{'site':'RetRight','origin_reference_cursor':2**128-1,'parent_resume':{'Some':0}})
        self.assertEqual(common['arguments'][0]['constructor'],'chSiteRetRight')
        self.assertEqual(common['arguments'][1],{'sort':'Int','value':2**128-1})
        self.assertEqual(common['arguments'][2]['sort'],'CHResume')
    def test_scalar_staged_bool_and_uint_remain_distinct(self):
        a=m.port_value('CHStaged',{'BoolValue':{'b':True}})
        b=m.port_value('CHStaged',{'U128Value':{'n':1}})
        self.assertEqual(a['constructor'],'chStagedBoolValue')
        self.assertIs(type(a['arguments'][0]['value']),bool)
        self.assertEqual(b['constructor'],'chStagedU128Value')
        self.assertIs(type(b['arguments'][0]['value']),int)
    def test_full_source_carrier_and_order_are_retained(self):
        source={'helper_cases':[{'case_id':'H','helper':None}],
                'result_cases':[{'case_id':'R','result':{'EnvMissing':None}}],
                'domain':'UNADMITTED input only'}
        result=m.port_document(source)
        self.assertIs(result['source_input'],source)
        query=result['queries']
        self.assertEqual(query['constructor'],'chHelperQuery')
        self.assertEqual(query['arguments'][0]['value'],'H')
        following=query['arguments'][2]
        self.assertEqual(following['constructor'],'chResultQuery')
        self.assertEqual(following['arguments'][0]['value'],'R')
        self.assertEqual(following['arguments'][1]['arguments'][0]['constructor'],'chResultEnvMissing')

if __name__=='__main__':
    unittest.main()
