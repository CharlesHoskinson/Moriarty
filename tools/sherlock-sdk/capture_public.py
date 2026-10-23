# Closed PUBLIC capture type sherlock-capture-public/1; byte-identical twin of capture-public.mjs.
# Pure stdlib: no file, SDK, wallet, network, native or private-operand access. The record is built
# ONLY from authenticated expected inputs and fixed constants; a private projection is only compared.
import hashlib,json,math,posixpath,re
PUBLIC_SCHEMA='sherlock-capture-public/1';EXPECTED_SCHEMA='sherlock-capture-public-expected/1';SOURCE_SCHEMA='sherlock-template-capture/1';CTOR_SCHEMA='sherlock-anchor-ctor/1'
PUBLIC_LIMIT=1<<16;SOURCE_LIMIT=1<<20;SMALL_FILE=16<<20;BUILD_FILE=256<<20
BUILD_FILES=('contract/index.js','zkir/acceptValidity.zkir','zkir/acceptValidity.bzkir','keys/acceptValidity.prover','keys/acceptValidity.verifier','zkir/settleValidity.zkir','zkir/settleValidity.bzkir','keys/settleValidity.prover','keys/settleValidity.verifier')
CTOR_KEYS=('id','keys','a102_lo','a102_hi','a80_lo','a80_hi','prog','r0','h0','b0')
P6='P6 NONCOMPOSABLE: under the hand-written Sherlock decider key acceptValidity is noncomposable. No contract may claim or rely on a call to it. Accepted relation: ops = the captured template constants with the stmt slot = the DeciderRelation stmt of a parent-verified W; the binding input is a public input; the communication commitment at slot 1 is constrained in circuit to Poseidon(comm_rand, stmt) for the compiled acceptValidity shape (one Native input, no outputs), where comm_rand is private witness only and is never a public input, template constant, logged value or record field. No equivalence to the compiled acceptValidity operation is claimed.'
EFFECTS_EMPTY='all nine Effects collections empty (native post-state and every partitioned transcript)'
SOURCE_LEGEND='Whitelist from actual complete quiescent original MP2 capture via pinned full oracle; ordered public build/source/constructor metadata retained; private capture/preimage/manifest products stripped.'
PUBLIC_LEGEND='Built only from authenticated expected ctor/source/build/guards/template inputs and fixed constants; no raw capture, preimage, manifest, hash, path or extra is an input.'
PUBLIC_FILES=('EXPECTED-PUBLIC-INPUTS.BEFORE-PROOF.json','batch-0.proof','batch-0.public.json','batch-0.stats.json','capture-metadata.json','provenance.json','safe-result.json','tagged-decider.vk','tagging.json','wrapped-decider.proof')
CODES=frozenset('ERR_CPUB_'+c for c in 'EXPECTED JSON SHAPE SCHEMA TEMPLATE CTOR SOURCE BUILD GUARDS KEYS P6 EFFECTS NONCANONICAL SOURCE_FORM MEMBERSHIP DIRTY_ROOT RECEIPT_BINDING UNCLASSIFIED'.split())
P=int('73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001',16)
HEX=re.compile('[0-9a-f]{64}');DEC=re.compile('0|[1-9][0-9]{0,77}');TEXT=re.compile('[\n -~]*')
ID=('bytes','sha256');ACCEPT='keys/acceptValidity.verifier';SETTLE='keys/settleValidity.verifier'
EXPECTED_KEYS=('schema','templateSha256','ctor','source','guards','build','settleVerifierSha256','taggedVerifierSha256')
SECTIONS=((('schema',),'SCHEMA'),(('route',),'SCHEMA'),(('status',),'SCHEMA'),(('projection',),'SCHEMA'),(('template',),'TEMPLATE'),(('ctor',),'CTOR'),(('frozen','source'),'SOURCE'),(('frozen','build'),'BUILD'),(('frozen','guards'),'GUARDS'),(('keys',),'KEYS'),(('p6',),'P6'),(('captures',),'EFFECTS'))

class CapturePublicError(Exception):
 '''args == (one fixed CODES member,); never data, cause or context.'''

def _req(ok,code):
 if not ok:raise CapturePublicError('ERR_CPUB_'+code)
def _fixed(fn):
 def run(*a):
  try:return fn(*a)
  except CapturePublicError as e:code=e.args[0] if len(e.args)==1 and e.args[0] in CODES else 'ERR_CPUB_UNCLASSIFIED'
  except Exception:code='ERR_CPUB_UNCLASSIFIED'
  raise CapturePublicError(code) from None
 return run
_hex=lambda v:type(v) is str and HEX.fullmatch(v) is not None
_dec=lambda v:type(v) is str and DEC.fullmatch(v) is not None and int(v)<P
_exact=lambda v,keys:type(v) is dict and list(v)==list(keys)
_same=lambda a,b:json.dumps(a)==json.dumps(b)
_pick=lambda o,keys:{k:o[k] for k in keys if k in o} if type(o) is dict else None
_ident=lambda v,m:_exact(v,ID) and type(v['bytes']) is int and 0<v['bytes']<=m and _hex(v['sha256'])
_obj=lambda v:v if type(v) is dict else {}
def _at(o,path):
 for k in path:o=o.get(k) if type(o) is dict else None
 return o
def _finite(s):
 v=float(s)
 if not math.isfinite(v):raise ValueError('nonfinite')
 return v
def _unique(pairs):
 out={}
 for k,v in pairs:
  if k in out:raise ValueError('duplicate')
  out[k]=v
 return out
def _parse(raw,limit,code):
 _req(type(raw) is bytes and 0<len(raw)<=limit,code);text=value=None
 try:text=raw.decode('ascii');value=json.loads(text,object_pairs_hook=_unique,parse_float=_finite,parse_constant=_finite)
 except (ValueError,RecursionError):text=None
 _req(text is not None and TEXT.fullmatch(text) is not None,code)
 return text,value
# Detached snapshot of the independently authenticated expected parameters, never the validated object.
def _expected(value):
 try:e=json.loads(json.dumps(value,allow_nan=False))
 except (TypeError,ValueError,RecursionError):e=None
 _req(_exact(e,EXPECTED_KEYS) and e['schema']==EXPECTED_SCHEMA and all(_hex(e[k]) for k in ('templateSha256','settleVerifierSha256','taggedVerifierSha256')) and _exact(e['ctor'],('schema','values')) and e['ctor']['schema']==CTOR_SCHEMA and _exact(e['ctor']['values'],CTOR_KEYS) and all(_dec(v) for v in e['ctor']['values'].values()) and _ident(e['source'],SMALL_FILE) and _ident(e['guards'],SMALL_FILE) and _exact(e['build'],BUILD_FILES) and all(_ident(r,BUILD_FILE) for r in e['build'].values()),'EXPECTED')
 _req(e['build'][SETTLE]['sha256']==e['settleVerifierSha256'] and len({e['build'][ACCEPT]['sha256'],e['build'][SETTLE]['sha256'],e['taggedVerifierSha256']})==3,'EXPECTED')
 return e
def _build(value):
 e=_expected(value);i=lambda r:dict(bytes=r['bytes'],sha256=r['sha256'])
 return dict(schema=PUBLIC_SCHEMA,route='sherlock/acceptValidity',status='CAPTURED_CROSS_CHECKED',template=dict(sha256=e['templateSha256'],provisional=False,n=15,stmt_index=10),ctor=dict(schema=CTOR_SCHEMA,values=_pick(e['ctor']['values'],CTOR_KEYS)),frozen=dict(source=i(e['source']),build={f:i(e['build'][f]) for f in BUILD_FILES},guards=i(e['guards'])),keys=dict(compiled_acceptValidity_verifier=dict(sha256=e['build'][ACCEPT]['sha256']),settleValidity_verifier=dict(sha256=e['build'][SETTLE]['sha256'])),p6=P6,captures=[dict(effects=EFFECTS_EMPTY),dict(effects=EFFECTS_EMPTY)],projection=PUBLIC_LEGEND)
def canonical_bytes(record):return (json.dumps(record,separators=(',',':'))+'\n').encode('ascii')
def _identity(b):return dict(schema=PUBLIC_SCHEMA,bytes=len(b),sha256=hashlib.sha256(b).hexdigest())
build_public=_fixed(_build)
public_identity=_fixed(lambda value:_identity(canonical_bytes(_build(value))))
def _derive(projection,value):
 # Private projection (pinned projector json.dump(indent=2)+newline) -> public bytes; extras are dropped
 # because nothing is copied: every whitelisted source field must equal the expected-built value.
 pub=_build(value);text,s=_parse(projection,SOURCE_LIMIT,'SOURCE_FORM')
 _req(type(s) is dict and json.dumps(s,indent=2)+'\n'==text,'SOURCE_FORM')
 _req(s.get('schema')==SOURCE_SCHEMA and s.get('route')==pub['route'] and s.get('status')==pub['status'] and s.get('projection')==SOURCE_LEGEND,'SCHEMA')
 _req(_same(_pick(s.get('template'),list(pub['template'])),pub['template']),'TEMPLATE')
 c=_obj(s.get('ctor'));_req(c.get('schema')==CTOR_SCHEMA and _same(_pick(c.get('values'),CTOR_KEYS),pub['ctor']['values']),'CTOR')
 f=_obj(s.get('frozen'));b=_obj(f.get('build'));k=_obj(s.get('keys'))
 _req(_same(_pick(f.get('source'),ID),pub['frozen']['source']),'SOURCE')
 _req(all(_same(_pick(b.get(n),ID),pub['frozen']['build'][n]) for n in BUILD_FILES),'BUILD')
 _req(_same(_pick(f.get('guards'),ID),pub['frozen']['guards']),'GUARDS')
 _req(all(_same(_pick(k.get(n),('sha256',)),pub['keys'][n]) for n in pub['keys']),'KEYS')
 _req(s.get('p6')==P6,'P6')
 cs=s.get('captures');_req(type(cs) is list and len(cs)==2 and all(type(x) is dict and x.get('effects')==EFFECTS_EMPTY for x in cs),'EFFECTS')
 out=canonical_bytes(pub);return dict(schema=PUBLIC_SCHEMA,canonical=out,sha256=hashlib.sha256(out).hexdigest())
derive_public=_fixed(_derive)
def _validate(candidate,value):
 # PUBLIC candidate: closed type, per-section fixed codes, then exact canonical bytes; never drops anything.
 pub=_build(value);want=canonical_bytes(pub);_,v=_parse(candidate,PUBLIC_LIMIT,'JSON')
 _req(_exact(v,list(pub)) and _exact(v['frozen'],list(pub['frozen'])),'SHAPE')
 for path,code in SECTIONS:_req(_same(_at(v,path),_at(pub,path)),code)
 _req(candidate==want,'NONCANONICAL')
 return _identity(want)
validate_public=_fixed(_validate)
def publish_validated(candidate,value,sink):
 # The sink (exclusive durable writer) runs only after validation and receives an immutable copy.
 identity=validate_public(candidate,value)
 if not callable(sink):raise CapturePublicError('ERR_CPUB_UNCLASSIFIED')
 sink(bytes(candidate));return identity
def _membership(names):
 _req(type(names) in (list,tuple) and all(type(n) is str for n in names) and sorted(names)==list(PUBLIC_FILES),'MEMBERSHIP');return True
assert_public_membership=_fixed(_membership)
def _root(v):
 # Absolute normalized printable product directory; '/' and the implementation-defined '//' prefix refused.
 return type(v) is str and re.fullmatch('/[ -~]+',v) is not None and not v.startswith('//') and posixpath.normpath(v)==v
def _address(v):
 # Twin of normalizeContractAddress: one leading '0x' stripped, lowercased, nonempty even-length hex; else None.
 if type(v) is not str:return None
 s=(v[2:] if v.startswith('0x') else v).lower()
 return s if re.fullmatch('[0-9a-f]+',s) is not None and len(s)%2==0 else None
def _bind(outcome,products,public_sha256,root,expected_address):
 # Post-run controller adoption of exactly the ten files directly under one root. Metadata binding only: never
 # financial acceptance, never a nonexecution inference. Any refusal leaves the root DIRTY: not adopted,
 # retained, no resend/revert. expected_address is the frozen callContext address, already normalized.
 _req(_hex(public_sha256) and _root(root) and type(expected_address) is str and _address(expected_address)==expected_address,'EXPECTED')
 _req(type(products) is list and all(type(r) is dict and type(r.get('path')) is str and _hex(r.get('sha256')) for r in products),'MEMBERSHIP')
 by={r['path'].rsplit('/',1)[-1]:r for r in products};_req(len(by)==len(products),'MEMBERSHIP');_membership(list(by))
 _req(all(r['path']==root+'/'+n for n,r in by.items()),'MEMBERSHIP')
 o=_obj(outcome)
 _req(o.get('schema')=='sherlock-original-once-safe/1' and o.get('action')=='accept' and o.get('status')=='FINALIZED_FULL_EFFECTS_CLOSED' and o.get('network')=='stagenet' and o.get('receipt_accepted') is True and o.get('same_genuine_call_proof_verified') is True and all(o.get(k) is True for k in ('full_effects_readback','owned_submission_closed','wallet_closed','transactions_released')) and all(type(o.get(k)) is str and o.get(k)!='' for k in ('transaction_hash','identifier')),'DIRTY_ROOT')
 _req(_address(o.get('address'))==expected_address and o.get('call_proof_record_sha256')==by['safe-result.json']['sha256'] and by['capture-metadata.json']['sha256']==public_sha256,'RECEIPT_BINDING')
 return dict(status='PUBLIC_ROOT_BOUND_TO_FINALIZED_OUTCOME_METADATA_ONLY',safeResultSHA256=o['call_proof_record_sha256'],capturePublicSHA256=public_sha256)
bind_receipt=_fixed(_bind)
