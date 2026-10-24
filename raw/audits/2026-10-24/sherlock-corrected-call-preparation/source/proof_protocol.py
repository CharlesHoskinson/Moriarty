"""Public-only sealed-call bound-D product inspection (genuine-bound-d-sealed-call-proof-safe/4 only); no SDK or subprocess.
capture-metadata.json is the closed sherlock-capture-public/1 type, checked by the pinned capture_public module bytes.
Only actual-sherlock-call-intent/2 is accepted; schema1 receipts and the old protocol file stay immutable and never satisfy this phase.
The wallet controller authenticates PRE-PROOF-OPERANDS and its selected submission intent.
inspect_products does not parse the wallet controller mixed terminal streams.
This inspector compares the actual before-proof numeric freeze with final native
products. It does not recompute Poseidon or independently execute verification.
"""
from pathlib import Path
import hashlib,json,math,os,re,stat,types
# Every predicate below is an assert and -O/PYTHONOPTIMIZE strips them: refuse first.
if not __debug__:raise RuntimeError('proof_protocol_v2 requires assertions')

P=int('73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001',16)
HEX=re.compile(r'[0-9a-f]{64}')
# The before-proof PI record uses the frozen PROOF-EXPECTATIONS policy name; capture-metadata.json is the cleared sherlock-capture-public/1 canonical bytes, never the raw B3 member capture.
FILES={'EXPECTED-PUBLIC-INPUTS.BEFORE-PROOF.json','safe-result.json','batch-0.proof','wrapped-decider.proof','batch-0.public.json','batch-0.stats.json','tagged-decider.vk','tagging.json','provenance.json','capture-metadata.json'}
PUBLIC_FIELDS={'public_inputs','binding_input','communication_commitment','stmt','stmt_slot','instance_id','batch_no','w_instance'}
TAG_CHECKS={'raw_processed_roundtrip','tagged_suffix_is_raw','tagged_decode_reencode_equal','ledger_reader_init_reencode_equal'}
PROVENANCE_CHECKS={'decider_vk_processed_roundtrip','decider_relation_roundtrip','template_equal','parent_vk_equal','shape','comm_binding_bound_poseidon_stmt_v1','s_g2_official_srs'}
# Exact sherlock-provenance.rs inspect members (22) and its unclaimed caveat.
PROVENANCE_FIELDS={'schema','decider_vk_raw_sha256','decider_vk_raw_bytes','decider_vk_k','decider_relation_sha256','decider_relation_bytes','template_sha256','template_slots','stmt_index','parent_vk_sha256','w_vk_sha256','dom_stmt','stmt_fields','n_w_instance','nr_cols','comm_binding','comm_opening_constrained','s_g2','srs','checks','not_checked','p6'}
PROVENANCE_NOT_CHECKED='decider.vk is the key of decider.relation: needs re-keygen at the decider degree under the pinned SRS'
SCOPE='Genuine call intent1 proved and sealed before disjoint DUST intent2; same raw native proof verified after merge and provider/ledger proof bytes linked; adapter vector only; real ledger validation remains required'
# Schema2 receipts published provider_link.preimage_sha256 and private native bundle/member paths; they never satisfy this phase.
SAFE_SCHEMA2='genuine-bound-d-sealed-call-proof-safe/2'
SAFE_SCHEMA3='genuine-bound-d-sealed-call-proof-safe/3'
# Schema3 receipts published the raw capture projection, native_tuple.config_sha256 and digests.capture; they never satisfy this phase.
SAFE_SCHEMA4='genuine-bound-d-sealed-call-proof-safe/4'
SAFE_STATUS2='ACTUAL_GENUINE_SEALED_CALL_PRESERVED_AFTER_DUST_AND_NATIVE_VERIFY'
SAFE_FIELDS2={'schema','status','public','proof','wrapped_proof','raw_decider_vk_sha256','tagged_verifier_sha256','k','negative_checks','private_opening_logged','independent_native_verify','production_adapter_verify','post_fee_merge_call_preserved','submission_intent','scope','call_context','proving_model','sealed_call','provider_link','native_tuple'}
SAFE_FIELDS4=SAFE_FIELDS2|{'capture_public'}
CALL_KEY='sherlock/acceptValidity'
CAPTURE_PUBLIC_MODULE_BYTES=11457
CAPTURE_PUBLIC_MODULE_SHA256='14f0d1e6b9b50daeae69c0dd9d796b1c851f74393bbd83f8b692bfc583cd4d05'
CALL_CONTEXT_FIELDS={'schema','network','address','entry_point','deployment_record_sha256'}
PROVING_MODEL_FIELDS={'transaction_cost_model_sha256','parameters_sha256','parameters_block'}
SEALED_CALL_FIELDS={'intent_sha256','proof_sha256','binding_sha256'}
# Corrected B4 observeSealed b4-call-provider-link/2 projected without raw_proof_hex, stored_proof_hex and preimage_sha256 (frozen /private: the B4 preimage digest equality stays in the wrapper's memory).
LINK_HEX={'binding_le','callback_sha256','communication_commitment','stmt','instance_id','batch_no','verifier_sha256','verifier_key_sha256','deployment_record_sha256','transport_sha256','stored_proof_sha256','raw_proof_sha256'}
LINK_FIELDS=LINK_HEX|{'schema','key_location','address','entry_point','verifier_key_tag','stored_variant','transport_form','linked','limitation','transport_limitation','requires'}
B4_LINKED='PROVIDER_RETURN_EQ_ROUTE_CAPTURE; TRANSPORT_BODY_EQ_STORED_BODY; STORED_VARIANT_FROM_RESOLVED_VERIFIER_KEY_TAG; INSTALLED_CODEC_ROUNDTRIPS_STORED; SEALED_AND_FINALIZED_GETTERS_EQ_STORED'
B4_LIMITATION='ADAPTER_VECTOR_ONLY: installed ContractCall exports no public_inputs/binding_input and the ledger vector depends on the parent binding_commitment; only real validateTransaction and on-chain validation check it'
B4_TRANSPORT_LIMITATION='installed ledger.Proof is the tagged ProofVersioned codec (proof-versioned, u8 variant, Proof body); the handkey transport is bare tagged proof[v5], which tx.rs accepts first and no public JS codec decodes. B4 parses it from pinned 9a8777c serialize/proofs.rs source, derives the stored V2/V3 form from the resolved verifier-key tag and round-trips that through installed ledger.Proof. Installed v9 binary correspondence to the rc3/9a8777c sources is unproven until a separately reviewed installed-SDK codec smoke runs'
B4_REQUIRES=['B3 native verify of raw_proof_hex (raw form, distinct from transport and stored forms) with the C-held native vector','real ledger validateTransaction','on-chain validation']
# Corrected B3 verifyAndPublishTuple callback object as projected by publish (bundle and member paths stay private), before PUBLISHED is returned.
TUPLE_FIELDS={'schema','status','stdout','digests','config_sha256','public_context_sha256','bounds','native_verify_runs'}
# Schema4 drops the private-path config_sha256 and the raw capture digest from the public projection.
TUPLE_FIELDS4=TUPLE_FIELDS-{'config_sha256'}
B3_BOUNDS={'bin':512<<20,'key':4<<20,'record':1<<20,'capture':16<<20,'template':1<<20,'proof':1<<20,'stdout':1<<20,'members':12,'bundle':4,'runs':1,'publishes':1}
B3_MEMBERS={'bin':512<<20,'decider.vk':4<<20,'decider.relation':4<<20,'tagged-verifier':4<<20,'template':1<<20,'tagging':1<<20,'provenance':1<<20,'capture':16<<20,'batch-0.proof':1<<20,'batch-0.public.json':1<<20,'batch-0.stats.json':1<<20,'wrapped-proof':1<<20}
PUBLIC_MEMBERS={n:c for n,c in B3_MEMBERS.items() if n!='capture'}
VERIFY_OK=re.compile(r'verify with ledger 2\^14 verifier params, stock MidnightVK::read: true in [0-9]+\.[0-9]{3}s\n')
# serializable.rs tagged_serialize writes 'midnight:'+tag+':'; proofs.rs bare Proof tag proof[v5]; structure.rs ProofVersioned u8 1 (V2) | 2 (V3).
BARE=b'midnight:proof[v5]:';VERSIONED=b'midnight:proof-versioned:'
VARIANTS={'verifier-key[v6]':('V2',1),'verifier-key[v7]':('V3',2)}
BUILD_FILES=['contract/index.js','zkir/acceptValidity.zkir','zkir/acceptValidity.bzkir','keys/acceptValidity.prover','keys/acceptValidity.verifier','zkir/settleValidity.zkir','zkir/settleValidity.bzkir','keys/settleValidity.prover','keys/settleValidity.verifier']
CTOR_KEYS=['id','keys','a102_lo','a102_hi','a80_lo','a80_hi','prog','r0','h0','b0']
OFFICIAL20='1cc62978558fdc1e445cd70cfd9a86ec3c2e2151b6d74811232d37faf9133ff1'

def unique(pairs):
 out={}
 for k,v in pairs:
  assert k not in out,'duplicate JSON member';out[k]=v
 return out

def parse_json(raw):
 def refuse(v):raise ValueError('nonfinite JSON number')
 return json.loads(raw.decode('utf-8'),object_pairs_hook=unique,parse_constant=refuse)

def sha(raw):return hashlib.sha256(raw).hexdigest()

def regular(path,maximum=1048576):
 p=Path(path);assert p.is_absolute() and not any(q.is_symlink() for q in [p,*p.parents]),'symlink or relative file'
 fd=os.open(p,os.O_RDONLY|os.O_NOFOLLOW)
 try:
  s=os.fstat(fd);assert stat.S_ISREG(s.st_mode) and 0<=s.st_size<=maximum,'file shape or bound'
  chunks=[];remaining=s.st_size
  while remaining:
   b=os.read(fd,min(remaining,1048576));assert b,'file shrank';chunks.append(b);remaining-=len(b)
  assert os.read(fd,1)==b'','file grew';t=os.fstat(fd)
  assert (s.st_dev,s.st_ino,s.st_size,s.st_mtime_ns,s.st_mode)==(t.st_dev,t.st_ino,t.st_size,t.st_mtime_ns,t.st_mode),'file changed'
 finally:os.close(fd)
 after=p.lstat();assert (t.st_dev,t.st_ino,t.st_size,t.st_mtime_ns,t.st_mode)==(after.st_dev,after.st_ino,after.st_size,after.st_mtime_ns,after.st_mode),'path changed'
 return b''.join(chunks)

def pinned(row,maximum=1048576):
 assert type(row) is dict and type(row['bytes']) is int and 0<=row['bytes']<=maximum and type(row['sha256']) is str and HEX.fullmatch(row['sha256'])
 b=regular(row['path'],maximum);assert len(b)==row['bytes'] and sha(b)==row['sha256'],'pin mismatch';return b

def field(value):
 assert type(value) is str and HEX.fullmatch(value),'field encoding'
 assert int.from_bytes(bytes.fromhex(value),'little')<P,'noncanonical field'
 return value

def fields(value,n):
 assert type(value) is list and len(value)==n
 return [field(v) for v in value]

def finite(value):return type(value) in (int,float) and math.isfinite(value) and value>=0

def negative_labels(template):
 labels=[]
 for i,s in enumerate(template['slots']):
  kind=s['kind']
  if kind=='const':name='Const(Fq(0x'+bytes.fromhex(field(s['value']))[::-1].hex()+'))'
  else:name={'binding_input':'BindingInput','communication_commitment':'CommunicationCommitment','stmt':'Stmt'}[kind]
  labels.append(('slot'+str(i)+':'+name)[:60])
 return set(labels)|{'short_vector','flipped_proof_byte','stmt_for_batch_no_plus_1'}

def submission_intent(value,expect,template_sha,tagged_sha):
 keys={'schema','network','address','action','physical_calls','pre_state_sha256','deployment_record_sha256','instance_id','batch_no','tagged_verifier_sha256','template_sha256','prove_after_final_balance','balancing_scope'}
 assert type(value) is dict and set(value)==keys,'submission intent shape'
 assert value['schema']=='actual-sherlock-call-intent/2' and value['network']=='stagenet' and value['action']=='sherlock/acceptValidity'
 assert type(value['address']) is str and value['address'].strip(),'submission address'
 assert type(value['physical_calls']) is int and value['physical_calls']==1,'one physical call'
 for k in ('pre_state_sha256','deployment_record_sha256'):assert type(value[k]) is str and HEX.fullmatch(value[k]),k
 assert field(value['instance_id'])==field(expect['instance_id']) and field(value['batch_no'])==expect['batch_no_le']=='0'*64
 assert value['tagged_verifier_sha256']==tagged_sha and value['template_sha256']==template_sha
 assert value['prove_after_final_balance'] is False and type(value['balancing_scope']) is list and value['balancing_scope']==['dust']
 return value

def frozen_public(value,expect,template,tagged_sha,intent):
 assert type(value) is dict and set(value)==PUBLIC_FIELDS|{'schema','verifier_sha256','submission_intent'}
 assert value['submission_intent']==intent
 submission_intent(value['submission_intent'],expect,intent['template_sha256'],tagged_sha)
 assert value['schema']=='genuine-sherlock-pi-before-proof/1' and value['verifier_sha256']==tagged_sha
 pis=fields(value['public_inputs'],15);w=fields(value['w_instance'],28)
 assert w==expect['genuineParentW']['complete28'],'complete parent W differs'
 assert field(value['instance_id'])==field(expect['instance_id']) and field(value['batch_no'])==expect['batch_no_le']=='0'*64
 assert type(value['stmt_slot']) is int and value['stmt_slot']==10
 assert field(value['binding_input'])==pis[0] and field(value['communication_commitment'])==pis[1] and field(value['stmt'])==pis[10]
 assert type(template['slots']) is list and len(template['slots'])==15 and template.get('provisional') is False
 for i,slot in enumerate(template['slots']):
  if i in (0,1,10):assert slot=={'kind':{0:'binding_input',1:'communication_commitment',10:'stmt'}[i]}
  else:assert set(slot)=={'kind','value'} and slot['kind']=='const' and pis[i]==field(slot['value'])
 return {k:value[k] for k in PUBLIC_FIELDS}

def inspect_capture(cfg,leaf):
 assert type(leaf) is dict and leaf['id']==cfg['operation']['id'] and type(leaf['actual_exit']) is int and leaf['actual_exit']==0 and leaf['controller_failure'] is None
 assert finite(leaf['elapsed_seconds']) and leaf['elapsed_seconds']<=cfg['operation']['wallSeconds']
 phase=Path(cfg['capturePhase']);rows=leaf['streams'];assert type(rows) is list and len(rows)==2
 assert all(type(r) is dict for r in rows) and {r['path'] for r in rows}=={str(phase/'stdout.bin'),str(phase/'stderr.bin')}
 streams={};cap=cfg['operation']['keptBytesPerStream'];assert type(cap) is int and 0<cap<=2097152
 for r in rows:
  assert set(r)=={'path','bytes','sha256','bytes_seen','bytes_retained','full_stream_retained'} and r['full_stream_retained'] is True
  assert all(type(r[k]) is int and 0<=r[k]<=cap for k in ('bytes','bytes_seen','bytes_retained')) and r['bytes']==r['bytes_seen']==r['bytes_retained']
  b=pinned(r,cap);streams[Path(r['path']).name]=b
 assert streams['stderr.bin']==b'','unexpected public stderr'
 assert regular(Path(cfg['publicOutputRoot'])/'safe-result.json')==streams['stdout.bin'],'safe terminal bytes differ from retained result'
 return inspect_products(cfg)

# Vec<u8> length: (len as u32) via_scale!(u32,4) -> ScaleBigInt (9a8777c serialize/src/util.rs), same as corrected B3 midnightVecLen; not a fixed LE32 word.
def midnight_vec_len(n):
 assert type(n) is int and 0<=n<=0xffffffff,'length is not a u32'
 if n<1<<6:return bytes([n<<2])
 if n<1<<14:return (n<<2|1).to_bytes(2,'little')
 if n<1<<30:return (n<<2|2).to_bytes(4,'little')
 return b'\x03'+n.to_bytes(4,'little')

def scale_u32(b,i):
 # Strict u32 ScaleBigInt read: the n-byte form must carry exactly 4 bytes and every form must re-encode identically.
 assert 0<=i<len(b),'scale length truncated';f=b[i];end=i+(1,2,4,5)[f&3]
 assert end<=len(b),'scale length truncated';assert f&3!=3 or f==3,'scale length exceeds u32'
 n=int.from_bytes(b[i+1:end],'little') if f&3==3 else int.from_bytes(b[i:end],'little')>>2
 assert midnight_vec_len(n)==b[i:end],'noncanonical scale length';return n,end

def bare_transport_raw(t):
 # tx.rs bare tagged proof[v5] transport: tag, SCALE u32 length, raw proof; consumed exactly.
 assert t.startswith(BARE),'transport is not bare proof[v5]';n,end=scale_u32(t,len(BARE))
 assert 0<n<=1048576 and end+n==len(t),'transport length or trailing bytes';return t[end:]

def hexes(o,keys):return all(type(o[k]) is str and HEX.fullmatch(o[k]) for k in keys)

def public_ident(value):
 assert type(value) is dict and set(value)=={'bytes','sha256'}
 assert type(value['bytes']) is int and value['bytes']>0
 assert type(value['sha256']) is str and HEX.fullmatch(value['sha256'])
 return value

def public_constructor(value):
 assert type(value) is dict and set(value)=={'schema','values'} and value['schema']=='sherlock-anchor-ctor/1'
 assert type(value['values']) is dict and list(value['values'])==CTOR_KEYS
 for name in CTOR_KEYS:
  v=value['values'][name]
  assert type(v) is str and re.fullmatch(r'0|[1-9][0-9]*',v),'constructor canonical decimal'
  assert int(v)<P,'constructor field bound'
 return value

def capture_public_module(row):
 # The executed bytes are the hashed bytes: no sys.path import, so no import-then-hash race. The module is stdlib-only.
 assert type(row) is dict and row['bytes']==CAPTURE_PUBLIC_MODULE_BYTES and row['sha256']==CAPTURE_PUBLIC_MODULE_SHA256,'capture_public module identity'
 src=pinned(row,65536);assert len(src)==CAPTURE_PUBLIC_MODULE_BYTES and sha(src)==CAPTURE_PUBLIC_MODULE_SHA256,'capture_public module identity'
 mod=types.ModuleType('capture_public');exec(compile(src,row['path'],'exec',dont_inherit=True),mod.__dict__);return mod

def inspect_products(cfg):
 # Selected production wallet controller authenticates its mixed streams separately.
 root=Path(cfg['publicOutputRoot']);assert root.is_absolute() and not any(q.is_symlink() for q in [root,*root.parents]) and root.is_dir()
 assert {p.name for p in root.iterdir()}==FILES,'public product membership'
 raw_products={name:regular(root/name) for name in FILES}
 safe=parse_json(raw_products['safe-result.json']);assert type(safe) is dict
 expect=parse_json(pinned(cfg['proofExpectations']));template=parse_json(pinned(cfg['publicTemplate']))
 assert expect['status']=='FROZEN_BEFORE_GENUINE_CALL_AND_NATIVE_PROOF' and type(expect['n']) is int and expect['n']==15 and type(expect['stmt_index']) is int and expect['stmt_index']==10
 assert expect['freeSlots']==[0,1,10] and all(type(i) is int for i in expect['freeSlots']) and expect['batch_no']=='0'
 assert expect['templateSHA256']==cfg['publicTemplate']['sha256'] and expect['genuineParentW']['vkSHA256']==cfg['parentVK_SHA256']
 fields(expect['genuineParentW']['complete28'],28)
 assert set(safe)==SAFE_FIELDS4
 assert safe['schema']==SAFE_SCHEMA4 and safe['status']==SAFE_STATUS2 and safe['post_fee_merge_call_preserved'] is True
 assert type(safe['k']) is int and safe['k']==20 and safe['private_opening_logged'] is False and safe['scope']==SCOPE
 assert safe['independent_native_verify'] is True and safe['production_adapter_verify'] is True
 raw=pinned(cfg['rawDeciderVK']);relation=pinned(cfg['deciderRelation']);tagged=raw_products['tagged-decider.vk'];proof=raw_products['batch-0.proof']
 assert 0<len(raw)<len(tagged)<=50000 and tagged.endswith(raw) and len(proof)>100
 raw_sha=sha(raw);tagged_sha=sha(tagged);proof_sha=sha(proof)
 assert safe['raw_decider_vk_sha256']==raw_sha and safe['tagged_verifier_sha256']==tagged_sha
 assert type(safe['proof']) is dict and set(safe['proof'])=={'bytes','sha256'} and type(safe['proof']['bytes']) is int and safe['proof']=={'bytes':len(proof),'sha256':proof_sha}
 wrapped=raw_products['wrapped-decider.proof'];assert len(wrapped)<=1048576 and wrapped==BARE+midnight_vec_len(len(proof))+proof and bare_transport_raw(wrapped)==proof
 assert type(safe['wrapped_proof']) is dict and set(safe['wrapped_proof'])=={'bytes','sha256'} and type(safe['wrapped_proof']['bytes']) is int and safe['wrapped_proof']=={'bytes':len(wrapped),'sha256':sha(wrapped)}
 intent=submission_intent(cfg['submissionIntent'],expect,cfg['publicTemplate']['sha256'],tagged_sha)
 assert safe['submission_intent']==intent
 submission_intent(safe['submission_intent'],expect,cfg['publicTemplate']['sha256'],tagged_sha)
 before=parse_json(raw_products['EXPECTED-PUBLIC-INPUTS.BEFORE-PROOF.json']);public=frozen_public(before,expect,template,tagged_sha,intent)
 assert safe['public']==before
 # Python bool/int equality is not enough: validate every nested public number/type again.
 frozen_public(safe['public'],expect,template,tagged_sha,intent)
 final=parse_json(raw_products['batch-0.public.json']);assert type(final) is dict and set(final)==PUBLIC_FIELDS|{'layout','encoding'}
 assert final['layout']=='[binding_input, communication_commitment, transcript field encodings] (ledger-9 ContractCall::public_inputs)' and final['encoding']=='32-byte little-endian BLS12-381 scalar hex'
 assert type(final['stmt_slot']) is int and all(final[k]==public[k] for k in PUBLIC_FIELDS),'before/final public vector mismatch'
 stats=parse_json(raw_products['batch-0.stats.json'])
 assert type(stats) is dict and set(stats)=={'kind','k','keygen_pk_s','prove_s','verify_s','total_s','proof_bytes','vk_bytes','peak_rss_kb_after_prove','peak_rss_kb','vk_sha256','proof_sha256','w_vk_sha256','transcript','verify_params','srs','threads','negative_checks'}
 assert stats['kind']=='recl9-decider-prove-call' and type(stats['k']) is int and stats['k']==20
 for k in ('keygen_pk_s','prove_s','verify_s','total_s'):assert finite(stats[k]),k
 for k in ('proof_bytes','vk_bytes','peak_rss_kb_after_prove','peak_rss_kb'):assert type(stats[k]) is int and stats[k]>0,k
 assert stats['proof_bytes']==len(proof) and stats['vk_bytes']==len(raw) and stats['proof_sha256']==proof_sha and stats['vk_sha256']==raw_sha and stats['w_vk_sha256']==cfg['parentVK_SHA256']
 assert stats['transcript']=='Blake2b (blake2b_simd::State)' and stats['verify_params']=="bls_midnight_2p14 verifier params (the ledger's embedded file)"
 assert type(cfg['expectedThreads']) is int and cfg['expectedThreads']==2 and stats['threads']=='2'
 srs=stats['srs'];assert type(srs) is dict and set(srs)=={'file','sha256','official_pin_match'} and srs=={'file':cfg['srs20']['boundPath'],'sha256':OFFICIAL20,'official_pin_match':True} and srs['official_pin_match'] is True and cfg['srs20']['sha256']==OFFICIAL20
 negatives=stats['negative_checks'];labels=negative_labels(template)
 assert len(labels)==18 and type(negatives) is dict and set(negatives)==labels and all(type(v) is str and v=='rejected' for v in negatives.values())
 assert safe['negative_checks']==negatives and type(safe['negative_checks']) is dict
 tagging=parse_json(raw_products['tagging.json']);assert type(tagging) is dict and set(tagging)=={'schema','raw_sha256','raw_bytes','tagged_sha256','tagged_bytes','checks','within_pinned_ledger_vk_limit','reader'}
 assert tagging['schema']=='sherlock-vk-tagging/1' and tagging['raw_sha256']==raw_sha and tagging['tagged_sha256']==tagged_sha
 assert type(tagging['raw_bytes']) is int and tagging['raw_bytes']==len(raw) and type(tagging['tagged_bytes']) is int and tagging['tagged_bytes']==len(tagged)
 assert type(tagging['checks']) is dict and set(tagging['checks'])==TAG_CHECKS and all(v is True for v in tagging['checks'].values()) and tagging['within_pinned_ledger_vk_limit'] is True
 assert tagging['reader']=='midnight-transient-crypto =3.0.0 (midnight-ledger rev 9a8777c4), midnight-proofs =0.8.3'
 assert raw_products['provenance.json']==pinned(cfg['provenance'])
 provenance=parse_json(raw_products['provenance.json'])
 # Typed public capture: pinned module bytes and pinned canonical expected parameters; validate_public runs on the raw bytes, which are parsed only afterwards.
 cp=capture_public_module(cfg['capturePublicModule']);xraw=pinned(cfg['capturePublicExpected']);expected=parse_json(xraw)
 assert cp.canonical_bytes(expected)==xraw and expected['templateSha256']==cfg['publicTemplate']['sha256'] and expected['taggedVerifierSha256']==tagged_sha,'capture public expected authority'
 cid=cp.validate_public(raw_products['capture-metadata.json'],expected);cpub=safe['capture_public']
 assert type(cpub) is dict and list(cpub)==['schema','bytes','sha256'] and type(cpub['bytes']) is int and cid==cp.public_identity(expected)==cpub==cfg['capturePublicIdentity'],'capture public identity'
 assert HEX.fullmatch(cfg['captureProjection']['sha256']) and cid['sha256']!=cfg['captureProjection']['sha256'] and sha(raw_products['capture-metadata.json'])==cid['sha256'],'public metadata equals raw projection pin'
 capture=parse_json(raw_products['capture-metadata.json'])
 assert type(provenance) is dict and set(provenance)==PROVENANCE_FIELDS,'provenance members'
 assert type(provenance['decider_vk_raw_bytes']) is int and provenance['decider_vk_raw_bytes']==len(raw) and type(provenance['decider_relation_bytes']) is int and provenance['decider_relation_bytes']==len(relation),'provenance byte counts'
 assert type(provenance['checks']) is dict and type(provenance['srs']) is dict and set(provenance['srs'])=={'k','sha256','official_pin_match'},'provenance nested shape'
 assert provenance['not_checked']==PROVENANCE_NOT_CHECKED and provenance['p6']==cp.P6,'provenance caveat or P6'
 assert provenance['schema']=='sherlock-decider-provenance/2' and provenance['decider_vk_raw_sha256']==raw_sha and provenance['decider_relation_sha256']==sha(relation)
 assert provenance['template_sha256']==cfg['publicTemplate']['sha256'] and provenance['parent_vk_sha256']==provenance['w_vk_sha256']==cfg['parentVK_SHA256']
 for k,n in [('n_w_instance',28),('nr_cols',4),('template_slots',15),('stmt_index',10),('decider_vk_k',20)]:assert type(provenance[k]) is int and provenance[k]==n
 assert provenance['dom_stmt']==cfg['expectedDomain'] and provenance['stmt_fields']==list(range(16)) and all(type(i) is int for i in provenance['stmt_fields'])
 assert provenance['comm_binding']=='poseidon-rand-stmt-v1' and provenance['comm_opening_constrained'] is True and set(provenance['checks'])==PROVENANCE_CHECKS and all(v is True for v in provenance['checks'].values())
 assert provenance['srs']=={'k':20,'sha256':OFFICIAL20,'official_pin_match':True} and type(provenance['srs']['k']) is int and provenance['srs']['official_pin_match'] is True
 assert type(provenance['s_g2']) is str and re.fullmatch(r'[0-9a-f]{192}',provenance['s_g2'])
 assert set(capture)=={'schema','route','status','template','frozen','keys','p6','captures','projection','ctor'}
 assert public_constructor(capture['ctor'])==public_constructor(cfg['expectedConstructor']),'constructor differs from frozen public inputs'
 assert type(capture['frozen']) is dict and set(capture['frozen'])=={'source','build','guards'}
 assert public_ident(capture['frozen']['source'])==public_ident(cfg['expectedCaptureSource']),'source differs from frozen public input'
 public_ident(capture['frozen']['guards'])
 assert type(capture['frozen']['build']) is dict and list(capture['frozen']['build'])==BUILD_FILES,'capture build order or membership'
 for row in capture['frozen']['build'].values():public_ident(row)
 assert capture['schema']=='sherlock-capture-public/1' and capture['projection']==cp.PUBLIC_LEGEND and capture['route']=='sherlock/acceptValidity' and capture['status']=='CAPTURED_CROSS_CHECKED'
 assert capture['template']['sha256']==cfg['publicTemplate']['sha256'] and capture['template']['provisional'] is False and type(capture['template']['n']) is int and capture['template']['n']==15 and type(capture['template']['stmt_index']) is int and capture['template']['stmt_index']==10
 assert capture['p6']==provenance['p6'] and type(capture['p6']) is str and capture['p6'].startswith('P6 NONCOMPOSABLE:')
 assert type(capture['captures']) is list and len(capture['captures'])==2 and all(e=={'effects':'all nine Effects collections empty (native post-state and every partitioned transcript)'} for e in capture['captures'])
 # Schema3 sealed-call links. Public evidence only: no private preimage/params/wallet is opened, no ledger public_inputs are recomputed.
 ctx=safe['call_context'];want_ctx=cfg['callContext']
 assert type(ctx) is dict and type(want_ctx) is dict and set(ctx)==set(want_ctx)==CALL_CONTEXT_FIELDS and all(type(ctx[k]) is str and ctx[k] for k in CALL_CONTEXT_FIELDS) and ctx==want_ctx,'call context differs from frozen A context'
 assert ctx['network']==intent['network'] and ctx['address']==intent['address'] and ctx['deployment_record_sha256']==intent['deployment_record_sha256'] and hexes(ctx,('deployment_record_sha256',)),'call context differs from submission intent'
 model=safe['proving_model'];want_model=cfg['provingModel']
 assert type(model) is dict and type(want_model) is dict and set(model)==set(want_model)==PROVING_MODEL_FIELDS and hexes(model,('transaction_cost_model_sha256','parameters_sha256')),'proving model shape'
 blk=model['parameters_block'];assert (type(blk) is int and blk>=0) or (type(blk) is str and re.fullmatch(r'0|[1-9][0-9]*',blk)),'parameters block'
 assert all(type(model[k]) is type(want_model[k]) for k in PROVING_MODEL_FIELDS) and model==want_model,'proving model differs from A'
 sealed=safe['sealed_call'];assert type(sealed) is dict and set(sealed)==SEALED_CALL_FIELDS and hexes(sealed,SEALED_CALL_FIELDS),'sealed call shape'
 link=safe['provider_link'];assert type(link) is dict and set(link)==LINK_FIELDS,'provider link projection'
 assert link['schema']=='b4-call-provider-link/2' and link['key_location']==CALL_KEY and hexes(link,LINK_HEX),'provider link identity'
 assert all(link[k]==public[k] for k in ('communication_commitment','stmt','instance_id','batch_no')) and link['binding_le']==public['binding_input'],'provider link public vector'
 assert link['verifier_sha256']==tagged_sha and all(link[k]==ctx[k] for k in ('address','entry_point','deployment_record_sha256')),'provider link deployment/key'
 # Stored ProofVersioned discriminant comes from the authenticated SDK lookupKey tag, never the config tagged decider VK.
 assert type(link['verifier_key_tag']) is str and link['verifier_key_tag'] in VARIANTS,'resolved verifier key tag';variant,disc=VARIANTS[link['verifier_key_tag']]
 stored=VERSIONED+bytes([disc])+midnight_vec_len(len(proof))+proof
 assert link['stored_variant']==variant and link['transport_form']=='bare-proof[v5]' and link['transport_sha256']==sha(wrapped) and link['raw_proof_sha256']==proof_sha,'provider raw/transport link'
 assert len({proof_sha,sha(wrapped),sha(stored)})==3 and link['stored_proof_sha256']==sha(stored)==sealed['proof_sha256'],'stored proof not linked to sealed call'
 assert link['linked']==B4_LINKED and link['limitation']==B4_LIMITATION and link['transport_limitation']==B4_TRANSPORT_LIMITATION and link['requires']==B4_REQUIRES,'provider link obligations'
 nt=safe['native_tuple'];assert type(nt) is dict and set(nt)==TUPLE_FIELDS4 and nt['schema']=='b3-verified-captured-tuple/1' and nt['status']=='CAPTURED_TUPLE_NATIVE_VERIFIED','native tuple is not the pre-publication verified callback'
 assert type(nt['native_verify_runs']) is int and nt['native_verify_runs']==1 and type(nt['stdout']) is str and VERIFY_OK.fullmatch(nt['stdout']) and hexes(nt,('public_context_sha256',)),'native verify run'
 assert type(nt['bounds']) is dict and all(type(v) is int for v in nt['bounds'].values()) and nt['bounds']==B3_BOUNDS,'B3 bounds'
 # Public native_tuple carries no bundle or member paths: set(nt)==TUPLE_FIELDS4 above refuses them and config_sha256.
 dig=nt['digests'];assert type(dig) is dict and set(dig)==set(PUBLIC_MEMBERS),'B3 public members'
 for n,cap in PUBLIC_MEMBERS.items():r=dig[n];assert type(r) is dict and set(r)=={'bytes','sha256'} and type(r['bytes']) is int and 0<=r['bytes']<=cap and hexes(r,('sha256',)),n
 # The raw B3 capture digest stays private; capture-metadata.json is bound above only through validate_public and capture_public.
 for n,b in {'decider.vk':raw,'decider.relation':relation,'tagged-verifier':tagged,'batch-0.proof':proof,'wrapped-proof':wrapped,'batch-0.public.json':raw_products['batch-0.public.json'],'batch-0.stats.json':raw_products['batch-0.stats.json'],'tagging':raw_products['tagging.json'],'provenance':raw_products['provenance.json']}.items():assert dig[n]['bytes']==len(b) and dig[n]['sha256']==sha(b),n+' differs from published product'
 # Key roles stay distinct: B4's SDK resolver key is the capture's compiled acceptValidity build key, never tagged D or settleValidity.
 ck=capture['keys'];assert type(ck) is dict and all(type(ck.get(k)) is dict and type(ck[k].get('sha256')) is str and HEX.fullmatch(ck[k]['sha256']) for k in ('compiled_acceptValidity_verifier','settleValidity_verifier')),'capture key shape'
 accept=ck['compiled_acceptValidity_verifier']['sha256'];settle=ck['settleValidity_verifier']['sha256'];build=capture['frozen']['build']
 assert accept==build['keys/acceptValidity.verifier']['sha256'] and settle==build['keys/settleValidity.verifier']['sha256'] and len({accept,settle,tagged_sha,raw_sha})==4 and type(cfg['settleVerifierSha256']) is str and HEX.fullmatch(cfg['settleVerifierSha256']) and settle==expected['settleVerifierSha256']==cfg['settleVerifierSha256'] and accept==expected['build']['keys/acceptValidity.verifier']['sha256'] and link['verifier_key_sha256']==accept,'resolver key is not the canonical compiled acceptValidity key'
 assert dig['template']['sha256']==cfg['publicTemplate']['sha256'] and dig['template']['bytes']==cfg['publicTemplate']['bytes'],'B3 template'
 # Weak evidence only; C1a owns fsync ordering and no-other-writer. safe-result.json must be the last product written.
 assert (root/'safe-result.json').lstat().st_mtime_ns==max((root/n).lstat().st_mtime_ns for n in FILES),'safe-result.json not written last'
 products=[dict(path=str(root/name),bytes=len(raw_products[name]),sha256=sha(raw_products[name])) for name in sorted(FILES)]
 return dict(report=safe,products=products,beforeProofEqualsFinal=True,submissionIntentBound=True,completeParentWFields=28,publicInputSlots=15,stmtIndex=10,actualNegativeChecks=18,privateOpeningPublished=False,scope='AUTHENTICATED_PUBLIC_RECEIPT_ONLY_ACTUAL_NATIVE_AND_ADAPTER_SUCCESS_FROM_PINNED_DRIVER_NOT_INDEPENDENT_CRYPTO_REVERIFY',statementCheck='Exact numeric callback freeze equals final PI10/stmt/I/B0/W28; Poseidon statement calculation is checked by pinned actual route/native verification, not recomputed by this inspector')
