"""Offline structural boundary checks; no K dispatch or evaluator invocation."""
import importlib.util,json,pathlib,unittest
HERE=pathlib.Path(__file__).resolve().parent
P=HERE/'expression_codec.py'
spec=importlib.util.spec_from_file_location('expression_codec',P)
codec=importlib.util.module_from_spec(spec) if P.exists() else None
if codec:spec.loader.exec_module(codec)
class CodecTests(unittest.TestCase):
 def api(self):
  self.assertIsNotNone(codec,'new structural codec is not implemented')
  return codec
 def test_lossless_raw_tree(self):
  c=self.api();value={'a':['128',True,None,2], 'z':{'constructor':'Unknown','operands':{},'span':{'kind':'synthetic','start':'0','end':'0'}}}
  term=c.encode_json(json.dumps(value,separators=(',',':'),sort_keys=True))
  self.assertEqual(c.decode_term(term),value)
 def test_not_a_type_or_evaluation_oracle(self):
  c=self.api()
  for value in [{'constructor':'Bogus'},{'width':'900','value':'-1'},{'type':['NoSuchType']},{'workInitial':'-3'}]:
   s=json.dumps(value,separators=(',',':'),sort_keys=True)
   self.assertEqual(c.decode_term(c.encode_json(s)),value)
 def test_noncanonical_and_duplicate_reject(self):
  c=self.api()
  for s in ['{"a":1,"a":2}','{"a": 1}','{"a":1.0}','{"a":NaN}','{"a":-0}','{"z":1,"a":2}']:
   with self.assertRaises(c.CodecError):c.encode_json(s)
 def test_unicode_and_escaping(self):
  c=self.api();x={'a':'\x00\b\n"\\é😀'};s=json.dumps(x,ensure_ascii=False,separators=(',',':'))
  self.assertEqual(c.decode_term(c.encode_json(s)),x)
  with self.assertRaises(c.CodecError):c.encode_json('"\\ud800"')
 def test_transport_bound(self):
  c=self.api()
  with self.assertRaises(c.CodecError):c.encode_json('"'+'x'*2_000_000+'"')
 def test_no_k_source_injection(self):
  c=self.api();s='"</k> rule X => Y //"';term=c.encode_json(s)
  self.assertEqual(c.decode_term(term),'</k> rule X => Y //')
 def test_byte_bounded_deep_type_metadata(self):
  c=self.api()
  for depth in (1500,5947):
   text='["Option",'*depth+'["UInt64"]'+']'*depth
   term=c.encode_json(text)
   self.assertEqual(c._canonical(c.decode_term(term)),text)
   serialized=c.serialize_term(term)
   self.assertTrue(serialized.startswith('{"args":'))
   self.assertGreater(len(serialized),len(text))
 def test_malformed_container_grammar(self):
  c=self.api()
  for text in ('[1,]','{"a":1,}','{"a" 1}','[[]','{}{}','[1 2]','{"a":}','[}'):
   with self.assertRaises(c.CodecError):c.encode_json(text)
 def test_complete_result_decoder(self):
  c=self.api();self.assertTrue(hasattr(c,'decode_result'),'complete K result decoder missing')
  out=c._apply('exValue',c._encode(['UInt128']),c._encode('7'),c._token('Int','9'))
  raw=self.result_wire(c,out)
  self.assertEqual(c.decode_result(raw),{'judgmentResult':'ExpressionValue','type':['UInt128'],'value':'7','workRemaining':'9'})
 def result_wire(self,c,out):
  cells=[c._apply('<out>',out),c._apply('<k>',{'node':'KSequence','arity':0,'items':[]}),c._apply('<sigma>',c._encode({})),c._apply('<request>',c._encode({}))]
  return c._canonical({'format':'KAST','version':4,'term':c._apply('<generatedTop>',*cells)})
 def test_result_binding_work_and_pending_control(self):
  c=self.api();out=c._apply('exValue',c._encode(['UInt128']),c._encode('7'),c._token('Int','9'));raw=self.result_wire(c,out)
  with self.assertRaises(c.CodecError):c.decode_result(raw,'{"changed":true}','{}')
  with self.assertRaises(c.CodecError):c.decode_result(self.result_wire(c,c._apply('exValue',c._encode(['UInt128']),c._encode('7'),c._token('Int','65537'))))
  data=c._parse(raw);data['term']['args'][1]['args'][0]={'node':'KSequence','arity':1,'items':[c._apply('exPending')]}
  with self.assertRaises(c.CodecError):c.decode_result(c._canonical(data))
 def test_result_rejects_pending_extra_and_partial_output(self):
  c=self.api();self.assertTrue(hasattr(c,'decode_result'),'complete K result decoder missing')
  for out in (c._apply('exPending'),c._apply('exValue',c._encode('7')),c._apply('exRejected',c._token('String','"HOST_OK"'),c._encode({'kind':'synthetic','start':'0','end':'0'}),c._list([]),c._token('Int','0'))):
   with self.assertRaises(c.CodecError):c.decode_result(self.result_wire(c,out))
 def test_closed_kast_decode(self):
  c=self.api()
  with self.assertRaises(c.CodecError):c.decode_term({'node':'KApply','label':'hostAnswer','args':[]})
 def test_request_keeps_schema_and_request_separate(self):
  c=self.api();t=c.encode_request('{}','{"bad":"untyped"}')
  self.assertEqual(t['label']['name'],'expressionRequest')
  self.assertEqual(c.decode_term(t['args'][0]),{})
  self.assertEqual(c.decode_term(t['args'][1]),{'bad':'untyped'})
if __name__=='__main__':unittest.main()
