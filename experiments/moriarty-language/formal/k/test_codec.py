import importlib.util
import json
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
def named(kind,name): return {'node':kind,'name':name,'params':[]}
try:
    import codec
except ModuleNotFoundError:
    codec = None

class CodecTests(unittest.TestCase):
    def test_codec_exists(self):
        self.assertIsNotNone(codec, 'bounded K codec must exist')

    @unittest.skipIf(codec is None, 'codec not yet implemented')
    def test_admission_preserves_financial_failures(self):
        cases = json.loads((HERE/'fixtures/cases.json').read_text())
        for case in cases:
            packet = codec.admit(json.dumps(case['input']))
            self.assertIn('packet(', codec.encode(packet))
        base = cases[0]['input']
        for field in ['payer','transferId','obligationId']:
            p = json.loads(json.dumps(base)); p['actions'][1][field] = 'Wrong'
            codec.admit(json.dumps(p))
        for field in ['outstanding','principal']:
            p = json.loads(json.dumps(base)); p['state']['obligations'][0][field] = '0'
            codec.admit(json.dumps(p))

    @unittest.skipIf(codec is None, 'codec not yet implemented')
    def test_schema_and_projection_are_separate(self):
        p=json.loads((HERE/'fixtures/cases.json').read_text())[0]['input']
        p['actions']=p['actions']*2
        with self.assertRaisesRegex(codec.CodecError,'UNSUPPORTED_PROJECTION'):
            codec.admit(json.dumps(p))
        p['extra']=True
        with self.assertRaisesRegex(codec.CodecError,'MALFORMED_INPUT'):
            codec.admit(json.dumps(p))

    @unittest.skipIf(codec is None, 'codec not yet implemented')
    def test_output_is_bound_and_closed(self):
        p=codec.admit(json.dumps(json.loads((HERE/'fixtures/cases.json').read_text())[0]['input']))
        digest=codec.digest(p)
        def token(sort,value): return {'node':'KToken','sort':named('KSort',sort),'token':value}
        out={'node':'KApply','label':named('KLabel','rejected'),'arity':3,'args':[token('String',json.dumps(digest)),token('String','"ZERO_AMOUNT"'),token('Int','1')]}
        def wire(o): return json.dumps({'format':'KAST','version':4,'term':{'node':'KApply','label':named('KLabel','<out>'),'arity':1,'args':[o]}})
        self.assertEqual(codec.decode(wire(out),p),{'status':'Rejected','code':'ZERO_AMOUNT','actionIndex':1})
        for mutation in ['digest','arity','label','index']:
            bad=json.loads(json.dumps(out))
            if mutation=='digest': bad['args'][0]['token']='"wrong"'
            elif mutation=='arity': bad['arity']=2
            elif mutation=='label': bad['label']='prepared'
            else: bad['args'][2]['token']='99'
            with self.assertRaises(codec.CodecError): codec.decode(wire(bad),p)
        with self.assertRaises(codec.CodecError): codec.decode(wire(out)+'{}',p)
        with self.assertRaises(codec.CodecError): codec.decode(wire(out).replace('"version": 4','"version": 999, "version": 4'),p)
        with self.assertRaises(codec.CodecError): codec.decode(wire(out).replace('"arity": 1','"arity": true',1),p)
        with self.assertRaises(codec.CodecError): codec.decode(wire(out).replace('"version": 4','"version": 4.0'),p)

    @unittest.skipIf(codec is None, 'codec not yet implemented')
    def test_complete_independent_positive_decoding(self):
        for case in json.loads((HERE/'fixtures/cases.json').read_text())[:6]:
            p=codec.admit(json.dumps(case['input']));e=case['expected'];s=e['post'];o=s['obligations'][0];r=e['effects'][1]
            values=[codec.digest(p),s['balances'][0]['amount'],s['balances'][1]['amount'],'0',s['allowances'][0]['remaining'],s['allowances'][0]['spent'],s['work']['remaining'],s['work']['spent'],o['principal'],o['accrued'],o['outstanding'],o['status'],r['settlementAmount'],r['principalDischarged'],r['accruedDischarged'],'1']
            args=[{'node':'KToken','sort':named('KSort','String' if i in [0,11] else 'Int'),'token':json.dumps(v) if i in [0,11] else v} for i,v in enumerate(values)]
            def app(label,children):return {'node':'KApply','label':named('KLabel',label),'arity':len(children),'args':children}
            out=app('prepared',args)
            term=app('<generatedTop>',[app('<k>',[{'node':'KSequence','arity':0,'items':[]}]),app('<out>',[out]),app('<generatedCounter>',[{'node':'KToken','sort':named('KSort','Int'),'token':'0'}])])
            raw=json.dumps({'format':'KAST','version':4,'term':term})
            self.assertEqual(codec.decode(raw,p),e)
            for wrong in ['0','-1']:
                args[15]['token']=wrong
                with self.assertRaises(codec.CodecError):codec.decode(json.dumps({'format':'KAST','version':4,'term':term}),p)
            args[15]['token']='1'
            args[1]['token']='340282366920938463463374607431768211456'
            with self.assertRaises(codec.CodecError):codec.decode(json.dumps({'format':'KAST','version':4,'term':term}),p)

    def test_observed_pinned_kast_v4(self):
        case=json.loads((HERE/'fixtures/cases.json').read_text())[0]
        packet=codec.admit(json.dumps(case['input']))
        raw=(HERE/'fixtures/observed-kast-v4-principal-partial.json').read_text()
        self.assertEqual(codec.decode(raw,packet),case['expected'])
        for kind in ['old-version','float-version','label-kind','label-param','sort-kind','sort-name','sort-param']:
            data=json.loads(raw)
            if kind=='old-version':data['version']=3
            elif kind=='float-version':data['version']=4.0
            elif kind=='label-kind':data['term']['label']['node']='KSort'
            elif kind=='label-param':data['term']['label']['params']=['bad']
            else:
                sort=data['term']['args'][1]['args'][0]['args'][1]['sort']
                if kind=='sort-kind':sort['node']='KLabel'
                elif kind=='sort-name':sort['name']='String'
                else:sort['params']=['bad']
            with self.assertRaises(codec.CodecError):codec.decode(json.dumps(data),packet)

if __name__=='__main__': unittest.main()
