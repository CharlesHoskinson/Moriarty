"""Admission tests retain bad financial inputs for K to reject, without executing K."""
import copy
import json
from pathlib import Path
import unittest
import tempfile
from unittest.mock import patch
import codec
import run

HERE=Path(__file__).resolve().parent

def base():return json.loads((HERE/'fixtures/cases.json').read_text())[0]['input']

class NumericTests(unittest.TestCase):
    def test_all_conversion_modes_and_allocation_are_encoded(self):
        for rule in ['AccrualFirst','PrincipalFirst','ProRata']:
            for mode in ['none','floor','ceil']:
                p=base();o=p['state']['obligations'][0]
                o['allocationRule']=rule;o['conversion']={'mantissa':'3','scale':'1','rounding':mode}
                admitted=codec.admit(json.dumps(p))
                self.assertEqual(admitted,p)
                self.assertIn('conversion(3, 1, '+json.dumps(mode)+')',codec.encode(admitted))

    def test_bad_conversion_is_not_a_host_success_filter(self):
        for m,s in [('0','0'),('1','19'),('1',str(codec.MAX))]:
            p=base();p['state']['obligations'][0]['conversion'].update(mantissa=m,scale=s)
            self.assertEqual(codec.admit(json.dumps(p)),p)
        p=base();p['actions']=p['actions'][:1]
        p['state']['obligations'][0].update(principal='0',accrued='0',outstanding='0',status='Settled',allocationRule='ProRata')
        self.assertEqual(codec.admit(json.dumps(p)),p)

    def test_numeric_allocation_preserves_default_run_limits(self):
        self.assertEqual(run.SUITE_LIMITS['initial'],(16,512))
        self.assertEqual(run.SUITE_LIMITS['branches'],(16,512))
        self.assertEqual(run.SUITE_LIMITS['transfer-only'],(16,512))
        self.assertEqual(run.SUITE_LIMITS['numeric'],(64,1512))

    def test_exhausted_attempt_limits_reject_without_dispatch_or_refund(self):
        for suite,count in [('initial',16),('branches',16),('transfer-only',16),('numeric',64)]:
            with tempfile.TemporaryDirectory() as temp:
                binding=Path(temp)/'binding.json'
                original=json.dumps({'sources':{},'artifacts':{},'krunInvocations':count})
                binding.write_text(original)
                with patch.object(run,'SUITE',suite), patch.object(run,'BINDING',binding), patch.object(run,'sources',return_value={}), patch.object(run,'artifacts',return_value={}), patch.object(run,'toolchain',return_value={}), patch.object(run,'command') as dispatch:
                    with self.assertRaisesRegex(run.RunnerError,'KRUN_LIMIT'):run.evaluate({},'exhausted')
                    dispatch.assert_not_called()
                    self.assertEqual(binding.read_text(),original)

    def test_other_action_shapes_and_large_states_still_unsupported(self):
        p=base();p['actions']=p['actions']*2
        with self.assertRaisesRegex(codec.CodecError,'UNSUPPORTED_PROJECTION'):codec.admit(json.dumps(p))
        p=base();p['state']['balances'].append(copy.deepcopy(p['state']['balances'][0]))
        with self.assertRaisesRegex(codec.CodecError,'UNSUPPORTED_PROJECTION'):codec.admit(json.dumps(p))

if __name__=='__main__':unittest.main()
