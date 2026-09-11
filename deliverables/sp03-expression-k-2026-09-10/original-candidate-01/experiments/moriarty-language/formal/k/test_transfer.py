"""Offline codec contracts; synthetic KAST does not establish K execution."""
import copy
import json
from pathlib import Path
import unittest
import codec

HERE = Path(__file__).resolve().parent

def packet():
    p = json.loads((HERE/'fixtures/cases.json').read_text())[0]['input']
    p['actions'] = p['actions'][:1]
    return p

def wire(label, values):
    def named(kind, name): return {'node':kind, 'name':name, 'params':[]}
    args = [{'node':'KToken', 'sort':named('KSort', sort),
             'token':json.dumps(value) if sort == 'String' else value}
            for sort, value in values]
    out = {'node':'KApply', 'label':named('KLabel', label), 'arity':len(args), 'args':args}
    return json.dumps({'format':'KAST', 'version':4, 'term':{
        'node':'KApply', 'label':named('KLabel','<out>'), 'arity':1, 'args':[out]}})

class TransferTests(unittest.TestCase):
    def test_transfer_admitted_and_distinct_term(self):
        p = codec.admit(json.dumps(packet()))
        self.assertTrue(codec.encode(p).startswith('transferPacket('))
        self.assertNotIn('repay(', codec.encode(p))

    def test_success_preserves_debt_and_charges_one_work(self):
        p = packet()
        # Original independent principal-partial input: cash/cap100, work100/0.
        values = [('String',codec.digest(p))] + [('Int',v) for v in ['70','30','0','70','30','99','1','1']]
        result = codec.decode(wire('preparedTransfer', values), p)
        expected = copy.deepcopy(p['state'])
        expected['balances'][0]['amount'] = '70'
        expected['balances'][1]['amount'] = '30'
        expected['allowances'][0].update(remaining='70',spent='30')
        expected['work'].update(remaining='99',spent='1')
        expected['usedTransferIds'] = [p['actions'][0]['id']]
        self.assertEqual(result, {'status':'Prepared','schemaVersion':codec.VERSION,
                                 'post':expected,'effects':p['actions']})
        self.assertEqual(p, packet())
        for index, value in [(0,'wrong'),(1,str(2**128)),(3,'1'),(8,'-1')]:
            bad = copy.deepcopy(values); bad[index] = (bad[index][0],value)
            with self.assertRaises(codec.CodecError): codec.decode(wire('preparedTransfer',bad),p)
        both = copy.deepcopy(p)
        both['actions'] = json.loads((HERE/'fixtures/cases.json').read_text())[0]['input']['actions']
        values[0] = ('String',codec.digest(both))
        with self.assertRaises(codec.CodecError): codec.decode(wire('preparedTransfer',values),both)

    def test_transfer_cannot_report_repay_index(self):
        p = packet()
        with self.assertRaises(codec.CodecError):
            codec.decode(wire('rejected',[('String',codec.digest(p)),('String','ZERO_AMOUNT'),('Int','1')]),p)

    def test_other_sequences_still_reject(self):
        p = packet()
        for actions in [[], p['actions']*2]:
            bad = copy.deepcopy(p); bad['actions'] = actions
            with self.assertRaisesRegex(codec.CodecError,'UNSUPPORTED_PROJECTION'):
                codec.admit(json.dumps(bad))

if __name__ == '__main__': unittest.main()
