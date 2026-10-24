# Known-public synthetic contract tests; expected results are explicit literals, never module output.
import hashlib,json,unittest
import capture_public as C
h=lambda c:c*64;SENT='PRIVATE-SENTINEL-preimage';P=int('73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001',16)
P6T='P6 NONCOMPOSABLE: under the hand-written Sherlock decider key acceptValidity is noncomposable. No contract may claim or rely on a call to it. Accepted relation: ops = the captured template constants with the stmt slot = the DeciderRelation stmt of a parent-verified W; the binding input is a public input; the communication commitment at slot 1 is constrained in circuit to Poseidon(comm_rand, stmt) for the compiled acceptValidity shape (one Native input, no outputs), where comm_rand is private witness only and is never a public input, template constant, logged value or record field. No equivalence to the compiled acceptValidity operation is claimed.'
EFF='all nine Effects collections empty (native post-state and every partitioned transcript)'
SRC_LEGEND='Whitelist from actual complete quiescent original MP2 capture via pinned full oracle; ordered public build/source/constructor metadata retained; private capture/preimage/manifest products stripped.'
PUB_LEGEND='Built only from authenticated expected ctor/source/build/guards/template inputs and fixed constants; no raw capture, preimage, manifest, hash, path or extra is an input.'
FILES9=['contract/index.js','zkir/acceptValidity.zkir','zkir/acceptValidity.bzkir','keys/acceptValidity.prover','keys/acceptValidity.verifier','zkir/settleValidity.zkir','zkir/settleValidity.bzkir','keys/settleValidity.prover','keys/settleValidity.verifier']
CK=['id','keys','a102_lo','a102_hi','a80_lo','a80_hi','prog','r0','h0','b0']
FILES=['EXPECTED-PUBLIC-INPUTS.BEFORE-PROOF.json','batch-0.proof','batch-0.public.json','batch-0.stats.json','capture-metadata.json','provenance.json','safe-result.json','tagged-decider.vk','tagging.json','wrapped-decider.proof']
def put(v,keys,x):
 o=v
 for k in keys[:-1]:o=o[k]
 o[keys[-1]]=x;return v
def values():return {k:str(i+1) for i,k in enumerate(CK)}
def build():return {f:dict(bytes=100+i,sha256=h('456789abc'[i])) for i,f in enumerate(FILES9)}
def expected():return dict(schema='sherlock-capture-public-expected/1',templateSha256=h('3'),ctor=dict(schema='sherlock-anchor-ctor/1',values=values()),source=dict(bytes=11,sha256=h('1')),guards=dict(bytes=22,sha256=h('2')),build=build(),settleVerifierSha256=h('c'),taggedVerifierSha256=h('d'))
def source(extras=True):
 s=dict(schema='sherlock-template-capture/1',route='sherlock/acceptValidity',status='CAPTURED_CROSS_CHECKED',template=dict(sha256=h('3'),provisional=False,n=15,stmt_index=10),ctor=dict(schema='sherlock-anchor-ctor/1',values=values()),frozen=dict(source=dict(bytes=11,sha256=h('1')),build=build(),guards=dict(bytes=22,sha256=h('2'))),keys=dict(compiled_acceptValidity_verifier=dict(sha256=h('8')),settleValidity_verifier=dict(sha256=h('c'))),p6=P6T,captures=[dict(effects=EFF),dict(effects=EFF)],projection=SRC_LEGEND)
 if extras:
  for k in (('template','path'),('ctor','values','private_rand'),('frozen','source','path'),('frozen','build',FILES9[0],'path'),('keys','compiled_acceptValidity_verifier','path')):put(s,k,SENT)
  put(s,('captures',0,'preimage_sha256'),h('e'));s['rawCapture']=dict(path=SENT,sha256=h('f'))
 return s
def public():
 s=source(False);s['schema']='sherlock-capture-public/1';s['projection']=PUB_LEGEND;return s
pub=lambda v:(json.dumps(v,separators=(',',':'))+'\n').encode()
src=lambda v:(json.dumps(v,indent=2)+'\n').encode()
PRIVATE=((('rawCapturePath',),'SHAPE'),(('frozen','manifest'),'SHAPE'),(('template','path'),'TEMPLATE'),(('ctor','values','private_rand'),'CTOR'),(('frozen','source','path'),'SOURCE'),(('frozen','build',FILES9[0],'path'),'BUILD'),(('frozen','guards','path'),'GUARDS'),(('keys','compiled_acceptValidity_verifier','path'),'KEYS'),(('captures',0,'preimage_sha256'),'EFFECTS'))
MISMATCH=((('schema',),'x','SCHEMA'),(('projection',),'x','SCHEMA'),(('template','stmt_index'),11,'TEMPLATE'),(('template','n'),True,'TEMPLATE'),(('template','provisional'),True,'TEMPLATE'),(('ctor','values','id'),'999','CTOR'),(('frozen','source','sha256'),h('0'),'SOURCE'),(('frozen','build',FILES9[2],'bytes'),1,'BUILD'),(('frozen','guards','bytes'),23,'GUARDS'),(('keys','settleValidity_verifier','sha256'),h('b'),'KEYS'),(('p6',),P6T[:17],'P6'),(('captures',1,'effects'),'x','EFFECTS'))
BAD_EXPECTED=((('extra',),1),(('ctor','values','id'),'01'),(('ctor','values','id'),str(P)),(('ctor','values','id'),'-1'),(('settleVerifierSha256',),h('b')),(('taggedVerifierSha256',),h('8')),(('source','bytes'),0),(('source','bytes'),True),(('build',FILES9[0],'bytes'),(256<<20)+1),(('templateSha256',),h('A')))

class CapturePublicContract(unittest.TestCase):
 def refuse(self,code,fn,*a):
  with self.assertRaises(C.CapturePublicError) as cm:fn(*a)
  e=cm.exception;self.assertEqual(e.args,('ERR_CPUB_'+code,));self.assertIsNone(e.__cause__);self.assertIsNone(e.__context__);self.assertIn(e.args[0],C.CODES)
 def refuse_publish(self,code,candidate,exp=None):
  calls=[];self.refuse(code,C.publish_validated,candidate,expected() if exp is None else exp,calls.append);self.assertEqual(calls,[])
 def test_shared_cross_language_golden_identity(self):
  top='52435875175126190479447740508185965837690552500527637822603658699938581184512';self.assertEqual(str(P-1),top)
  for b0,n,d in (('10',2974,'48b10f20716b13b64235793fbb33dcc1013a0e3e2b6662db474f6c29b9035791'),(top,3049,'6bf33a4bf374b221b525b7ac6c1f17a52cf042c94e03564acccb409588909cd6')):
   with self.subTest(b0=b0):
    e=put(expected(),('ctor','values','b0'),b0);r=C.derive_public(src(put(source(),('ctor','values','b0'),b0)),e);w=pub(put(public(),('ctor','values','b0'),b0))
    self.assertEqual((len(r['canonical']),hashlib.sha256(r['canonical']).hexdigest(),r['sha256']),(n,d,d))
    self.assertEqual(C.public_identity(e),dict(schema='sherlock-capture-public/1',bytes=n,sha256=d))
    self.assertEqual((len(w),hashlib.sha256(w).hexdigest()),(n,d))
 def test_derive_drops_private_projection_extras(self):
  r=C.derive_public(src(source()),expected());want=pub(public())
  self.assertEqual((r['schema'],r['canonical'],r['sha256']),('sherlock-capture-public/1',want,hashlib.sha256(want).hexdigest()))
  for s in (SENT,h('e'),h('f'),'private_rand'):self.assertNotIn(s.encode(),r['canonical'])
 def test_publish_roundtrip_identity(self):
  want=pub(public());calls=[];i=C.publish_validated(want,expected(),calls.append)
  self.assertEqual(i,dict(schema='sherlock-capture-public/1',bytes=len(want),sha256=hashlib.sha256(want).hexdigest()));self.assertEqual(calls,[want])
  self.assertEqual(C.public_identity(expected()),i);self.assertEqual(C.validate_public(C.derive_public(src(source()),expected())['canonical'],expected()),i)
 def test_old_raw_projection_publication_refused(self):
  self.refuse_publish('SCHEMA',pub(source(False)));self.refuse_publish('SCHEMA',src(source(False)));self.refuse_publish('SHAPE',pub(source()))
 def test_public_private_keys_rejected_before_sink(self):
  for k,c in PRIVATE:
   with self.subTest(k):self.refuse_publish(c,pub(put(public(),k,SENT)))
 def test_mismatch_codes_public_and_projection(self):
  for k,x,c in MISMATCH:
   with self.subTest(k):self.refuse_publish(c,pub(put(public(),k,x)));self.refuse(c,C.derive_public,src(put(source(),k,x)),expected())
  self.refuse('EFFECTS',C.derive_public,src(put(source(),('captures',),[dict(effects=EFF)]*3)),expected())
  self.refuse('BUILD',C.derive_public,src(put(source(),('frozen','build',FILES9[8]),None)),expected())
 def test_expected_schema_range_and_key_roles(self):
  for k,x in BAD_EXPECTED:
   with self.subTest(k):self.refuse_publish('EXPECTED',pub(public()),put(expected(),k,x))
  e=expected();e['build']=dict(reversed(list(e['build'].items())));self.refuse_publish('EXPECTED',pub(public()),e)
  self.refuse('EXPECTED',C.public_identity,object())
  top=str(P-1);self.assertEqual(C.validate_public(pub(put(public(),('ctor','values','b0'),top)),put(expected(),('ctor','values','b0'),top))['schema'],'sherlock-capture-public/1')
 def test_json_duplicate_nonfinite_noncanonical_bounds(self):
  w=pub(public())
  for b,c in ((w[:-2]+b',"p6":"x"}\n','JSON'),(w.replace(b'"n":15',b'"n":NaN'),'JSON'),(w.replace(b'"n":15',b'"n":1e999'),'JSON'),(w.replace(b'"n":15',b'"n":15.0'),'TEMPLATE'),(src(public()),'NONCANONICAL'),(w[:-1],'NONCANONICAL'),(w.replace(b'NONCOMPOSABLE',b'NONCOMPOSABL\xc3\x89'),'JSON'),(b' '*((1<<16)+1),'JSON'),(b'','JSON'),(w.decode(),'JSON')):
   with self.subTest(b[:40]):self.refuse_publish(c,b)
  self.refuse('SOURCE_FORM',C.derive_public,pub(source()),expected())
  self.refuse('SOURCE_FORM',C.derive_public,src(source())[:-3]+b',\n  "p6": "x"\n}\n',expected())
 def test_ten_file_interface(self):
  self.assertEqual(C.PUBLIC_FILES,tuple(FILES));self.assertTrue(C.assert_public_membership(FILES[::-1]))
  for bad in ([n.replace(FILES[0],'expected-before-proof.json') for n in FILES],FILES+['capture.json'],FILES[1:],FILES[1:]+[FILES[1]],None):
   with self.subTest(bad):self.refuse('MEMBERSHIP',C.assert_public_membership,bad)
 def test_bind_receipt_root_address_network_closure_binding(self):
  R='/run/public-proof';A='ab'*32;S=h('4')
  prods=lambda root=R:[dict(path=root+'/'+n,bytes=1,sha256=h(c)) for n,c in zip(FILES,'0123456789')]
  ok=lambda:dict(schema='sherlock-original-once-safe/1',action='accept',status='FINALIZED_FULL_EFFECTS_CLOSED',receipt_accepted=True,same_genuine_call_proof_verified=True,call_proof_record_sha256=h('6'),network='stagenet',address=A,transaction_hash='tx-1',identifier='id-1',full_effects_readback=True,owned_submission_closed=True,wallet_closed=True,transactions_released=True)
  drop=lambda k:{n:v for n,v in ok().items() if n!=k}
  want=dict(status='PUBLIC_ROOT_BOUND_TO_FINALIZED_OUTCOME_METADATA_ONLY',safeResultSHA256=h('6'),capturePublicSHA256=S)
  self.assertEqual(C.bind_receipt(ok(),prods(),S,R,A),want)
  for x in ('0x'+A,A.upper(),'0x'+A.upper()):
   with self.subTest(address=x):self.assertEqual(C.bind_receipt(put(ok(),('address',),x),prods(),S,R,A),want)
  for k,x,c in (('status','UNACCEPTED','DIRTY_ROOT'),('action','deploy','DIRTY_ROOT'),('same_genuine_call_proof_verified',False,'DIRTY_ROOT'),('receipt_accepted',1,'DIRTY_ROOT'),('network','mainnet','DIRTY_ROOT'),('network','Stagenet','DIRTY_ROOT'),('transaction_hash','','DIRTY_ROOT'),('transaction_hash',1,'DIRTY_ROOT'),('identifier','','DIRTY_ROOT'),('identifier',None,'DIRTY_ROOT'),('call_proof_record_sha256',h('5'),'RECEIPT_BINDING'),('address','cd'*32,'RECEIPT_BINDING'),('address','0X'+A,'RECEIPT_BINDING'),('address',A+'0','RECEIPT_BINDING'),('address','0x','RECEIPT_BINDING'),('address',int(A,16),'RECEIPT_BINDING')):
   with self.subTest(outcome=(k,x)):self.refuse(c,C.bind_receipt,put(ok(),(k,),x),prods(),S,R,A)
  for k in ('full_effects_readback','owned_submission_closed','wallet_closed','transactions_released'):
   for x in (False,1,'True',None):
    with self.subTest(closure=(k,x)):self.refuse('DIRTY_ROOT',C.bind_receipt,put(ok(),(k,),x),prods(),S,R,A)
  for k in ('network','transaction_hash','identifier','full_effects_readback','owned_submission_closed','wallet_closed','transactions_released','address'):
   with self.subTest(missing=k):self.refuse('RECEIPT_BINDING' if k=='address' else 'DIRTY_ROOT',C.bind_receipt,drop(k),prods(),S,R,A)
  for root in ('run/public-proof','/run/public-proof/','/run/./public-proof','/run/x/../public-proof','//run/public-proof','/run//public-proof','/','',' /run'):
   with self.subTest(root=root):self.refuse('EXPECTED',C.bind_receipt,ok(),prods(root),S,root,A)
  self.refuse('EXPECTED',C.bind_receipt,ok(),prods(),S,None,A)
  for x in ('0x'+A,A.upper(),A+'0','',None,'zz'):
   with self.subTest(expected_address=x):self.refuse('EXPECTED',C.bind_receipt,ok(),prods(),S,R,x)
  self.refuse('EXPECTED',C.bind_receipt,ok(),prods(),'x',R,A)
  for i,path in ((3,'/run/other/'+FILES[3]),(0,R+'//'+FILES[0]),(0,FILES[0]),(0,R+'/sub/'+FILES[0])):
   p=prods();p[i]['path']=path
   with self.subTest(path=path):self.refuse('MEMBERSHIP',C.bind_receipt,ok(),p,S,R,A)
  self.refuse('MEMBERSHIP',C.bind_receipt,ok(),prods('/run/other'),S,R,A);self.refuse('MEMBERSHIP',C.bind_receipt,ok(),prods()[1:],S,R,A)
  self.refuse('RECEIPT_BINDING',C.bind_receipt,ok(),prods(),h('3'),R,A);self.refuse('DIRTY_ROOT',C.bind_receipt,None,prods(),S,R,A)
 def test_constants_equal_independent_literals(self):
  self.assertEqual((C.P6,C.EFFECTS_EMPTY,C.SOURCE_LEGEND,C.PUBLIC_LEGEND,list(C.BUILD_FILES),list(C.CTOR_KEYS)),(P6T,EFF,SRC_LEGEND,PUB_LEGEND,FILES9,CK))
if __name__=='__main__':unittest.main()
