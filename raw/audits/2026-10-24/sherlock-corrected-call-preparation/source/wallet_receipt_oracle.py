"""Bounded public safe outcome inspection, no SDK/native/network imports.
Caller must establish owned cgroup/child cleanup first. Private complete streams
are authenticated but neither their bytes nor their hash manifests are returned.
"""
from pathlib import Path
import hashlib,json,re,math,os,stat
HEX=re.compile(r'[0-9a-f]+')
SHA=re.compile(r'[0-9a-f]{64}')
def unique(pairs):
 out={}
 for k,v in pairs:assert k not in out;out[k]=v
 return out
def read(p,maximum):
 p=Path(p);assert not any(q.is_symlink() for q in [p,*p.parents]);s=p.stat();assert stat.S_ISREG(s.st_mode) and s.st_size<=maximum;b=p.read_bytes();t=p.stat();assert (s.st_dev,s.st_ino,s.st_size,s.st_mtime_ns,s.st_mode)==(t.st_dev,t.st_ino,t.st_size,t.st_mtime_ns,t.st_mode) and len(b)==s.st_size;return b
def parse(b):
 def refuse(x):raise ValueError('nonfinite')
 return json.loads(b,object_pairs_hook=unique,parse_constant=refuse)
def load(p,maximum=1048576):return parse(read(p,maximum))
def norm(s):
 assert type(s) is str;v=s[2:] if s.startswith('0x') else s;assert HEX.fullmatch(v) and len(v)%2==0;return v
# Exact C3 writer keys (submit-once recordConsumerSuccess); no call_proof_profile alias. Literals are accept-call-profile.mjs.
ACCEPT_KEYS={'call_proof','call_proof_record_sha256'}
CALL_PROOF_FIELDS={'schema','status','call_context','capture_public','proving_model','sealed_call','consumer_check','limitation'}
CALL_PROOF_SCHEMA='genuine-bound-d-sealed-call-proof-safe/4';CALL_PROOF_STATUS='ACTUAL_GENUINE_SEALED_CALL_PRESERVED_AFTER_DUST_AND_NATIVE_VERIFY'
CALL_PROOF_CHECK='Pinned C2 safe/4 bytes equal the in-process intent, context and A evidence and carry the bound capture_public identity, which equals capture-metadata.json re-read at the same root and validated by the tested closed PUBLIC type against the driver expected object; value pre_state/stmt/address repeat the driver expressions that built the intent, not an independent readback'
CALL_PROOF_LIMITATION='Core ledger offline public_inputs absent; native/adapter verify covers the adapter vector under the pinned decider key; real ledger validation and on-chain readback remain mandatory; capture_public equality pins published PUBLIC bytes/type only, not independent cryptographic verification or finality; the C2 derive/publish stage chain is attested only by route.goGate before signed exposure'
def same(a,b):return json.dumps(a,sort_keys=True)==json.dumps(b,sort_keys=True)
def check_call_proof(safe,proof,binding):
 # Values come from the adopted binding or fixed writer literals. The C2 safe/4 copy is a consistency check only, never the authority.
 p=safe['call_proof'];r=safe['call_proof_record_sha256'];assert type(p) is dict and set(p)==CALL_PROOF_FIELDS and type(r) is str and SHA.fullmatch(r),'C3 profile keys'
 assert p['schema']==CALL_PROOF_SCHEMA and p['status']==CALL_PROOF_STATUS and p['consumer_check']==CALL_PROOF_CHECK and p['limitation']==CALL_PROOF_LIMITATION,'C3 profile literals'
 ctx=p['call_context'];a=binding['address'];assert type(a) is str and norm(a)==a and type(ctx) is dict and set(ctx)=={'schema','network','address','entry_point','deployment_record_sha256'} and all(type(v) is str for v in ctx.values()),'C3 context keys'
 assert ctx['schema']=='sherlock-sealed-call-context/1' and ctx['network']=='stagenet' and ctx['entry_point']=='acceptValidity' and ctx['address']==a and ctx['deployment_record_sha256']==binding['deploymentRecord']['sha256'] and SHA.fullmatch(ctx['deployment_record_sha256']),'C3 context'
 cp=p['capture_public'];want=binding['capturePublicIdentity'];assert type(cp) is dict and list(cp)==['schema','bytes','sha256'] and cp['schema']=='sherlock-capture-public/1' and type(cp['bytes']) is int and 0<cp['bytes']<=65536 and type(cp['sha256']) is str and SHA.fullmatch(cp['sha256']) and type(want) is dict and list(want)==list(cp) and same(cp,want),'C3 capture public identity'
 m=p['proving_model'];assert type(m) is dict and set(m)=={'transaction_cost_model_sha256','parameters_sha256','parameters_block'} and all(type(m[k]) is str and SHA.fullmatch(m[k]) for k in ['transaction_cost_model_sha256','parameters_sha256']),'C3 proving model'
 blk=m['parameters_block'];assert (type(blk) is int and blk>=0) or (type(blk) is str and re.fullmatch('0|[1-9][0-9]*',blk)),'C3 parameters block'
 s=p['sealed_call'];assert type(s) is dict and set(s)=={'intent_sha256','proof_sha256','binding_sha256'} and all(type(v) is str and SHA.fullmatch(v) for v in s.values()),'C3 sealed call'
 assert type(proof) is dict and proof.get('schema')==CALL_PROOF_SCHEMA and proof.get('status')==CALL_PROOF_STATUS and all(same(proof.get(k),p[k]) for k in ['call_context','capture_public','proving_model','sealed_call']),'C3 profile differs from C2 safe/4'

def fee_data_projection(cfg):
 # Runtime-only read of the selected own provider log. D route events may contain
 # private preimage hashes/bindings; only fixed public fields leave this function.
 q=cfg['feeProvingDataQualification'];assert q['remoteBytes']=='UNPINNED_DYNAMIC_HTTPS'
 root=Path(cfg['privateRuntimeRoot']);pattern=re.compile('sherlock-'+cfg['action']+r'-[0-9]+')
 candidates=[p for p in root.iterdir() if pattern.fullmatch(p.name)]
 assert len(candidates)==1 and candidates[0].is_dir() and not candidates[0].is_symlink()
 rows=load(candidates[0]/'prover/proving-events.json',q['eventReadset']['maxBytes']);assert type(rows) is list and 1<=len(rows)<=q['eventReadset']['maxEvents']
 degrees=set();routes={'midnight/dust/spend':dict(checks=0,proofs=0)}
 if cfg['action']=='accept':routes['sherlock/acceptValidity']=dict(checks=0,proofs=0)
 for e in rows:
  assert type(e) is dict and type(e.get('observed_at')) is str and e['phase'] in ['params','check','prove']
  if e['phase']=='params':
   assert set(e)=={'phase','k','observed_at'} and type(e['k']) is int and 0<=e['k']<=18;degrees.add(e['k']);continue
  route=e['key_location'];assert type(route) is str and route in routes
  expected='wasm' if route=='midnight/dust/spend' else ('hand-key' if e['phase']=='prove' else 'native (compiled IR check)')
  assert e['backend']==expected and type(e['preimage_bytes']) is int and e['preimage_bytes']>=0
  assert type(e['duration_ms']) in (int,float) and math.isfinite(e['duration_ms']) and e['duration_ms']>=0
  if e['phase']=='prove':assert type(e['proof_bytes']) is int and e['proof_bytes']>0
  routes[route]['proofs' if e['phase']=='prove' else 'checks']+=1
 assert routes['midnight/dust/spend']['proofs']>=1
 if cfg['action']=='accept':assert routes['sherlock/acceptValidity']['proofs']==1
 return dict(schema='sherlock-fee-data-safe/1',remote_bytes='UNPINNED_DYNAMIC_HTTPS',declared_initial_key_URLs=q['initialKeyURLs'],declared_initial_params_URL_pattern=q['initialParamsURLPattern'],URLs_are_network_egress_allowlist=False,final_redirect_destinations_and_remote_hashes='NOT_INDEPENDENTLY_CAPTURED',per_response_byte_cap=None,per_request_deadline_ms=None,provider_pre_IO_degree_guard='k>18 only',observed_params_degrees=sorted(degrees),degree_check_scope='Successful receiving check; not nonnegative/integer enforcement before I/O',routes=[dict(route=k,**v) for k,v in sorted(routes.items())],event_count=len(rows),local_cache_policy='Degrees5..18 exact pinned public bytes and0..4absence are pre/post receiving predicates',bounds='Unchanged selected whole SDK/native wall, CPU, memory and AS limits; no remote availability/runtime/memory-fit guarantee',acceptance='Actual genuine proof/node admission/canonical finalized financial receipt/full contracteffects and owned close remain required')

def inspect_actual_call(cfg,binding):
 phase=Path(cfg['capturePhase']);leaf=load(phase/'RESULT.json');assert leaf['id']==cfg['operation']['id'] and type(leaf['actual_exit']) is int and leaf['actual_exit']==0 and leaf['controller_failure'] is None
 assert type(leaf['elapsed_seconds']) in (int,float) and math.isfinite(leaf['elapsed_seconds']) and 0<=leaf['elapsed_seconds']<=cfg['operation']['wallSeconds']
 rows=leaf['streams'];assert type(rows) is list and len(rows)==2 and {r['path'] for r in rows}=={str(phase/'stdout.bin'),str(phase/'stderr.bin')}
 for row in rows:
  assert set(row)=={'path','bytes','sha256','bytes_seen','bytes_retained','full_stream_retained'} and row['full_stream_retained'] is True
  assert all(type(row[k]) is int and 0<=row[k]<=4194304 for k in ['bytes','bytes_seen','bytes_retained']) and row['bytes']==row['bytes_seen']==row['bytes_retained']
  b=read(row['path'],4194304);assert len(b)==row['bytes'] and hashlib.sha256(b).hexdigest()==row['sha256']
 safe=load(Path(cfg['privateRuntimeRoot'])/'safe-result.json');required={'schema','network','status','action','address','transaction_hash','identifier','signed_bytes_may_have_been_exposed','own_submit_may_have_started','receipt_accepted','receipt','full_effects_readback','same_genuine_call_proof_verified','owned_submission_closed','wallet_closed','transactions_released','checkpoint_written','no_resend','no_local_revert','pre_state_sha256','post_state_sha256'}
 # Accept adds exactly the two C3 writer keys; deploy keyset unchanged.
 assert type(safe) is dict and set(safe)==(required|ACCEPT_KEYS if cfg['action']=='accept' else required) and safe['schema']=='sherlock-original-once-safe/1' and safe['network']=='stagenet' and safe['status']=='FINALIZED_FULL_EFFECTS_CLOSED' and safe['action']==cfg['action']
 assert cfg['action'] in ['deploy','accept']
 if cfg['action']=='accept':assert norm(safe['address'])==norm(binding['address'])
 assert norm(safe['address']) and SHA.fullmatch(norm(safe['transaction_hash'])) and norm(safe['identifier'])
 for key in ['signed_bytes_may_have_been_exposed','own_submit_may_have_started','receipt_accepted','full_effects_readback','owned_submission_closed','wallet_closed','transactions_released','no_resend','no_local_revert']:assert safe[key] is True,key
 assert safe['checkpoint_written'] is False and SHA.fullmatch(safe['post_state_sha256'])
 if cfg['action']=='accept':assert safe['same_genuine_call_proof_verified'] is True and SHA.fullmatch(safe['pre_state_sha256'])
 else:assert safe['same_genuine_call_proof_verified'] is False and safe['pre_state_sha256'] is None
 receipt=safe['receipt'];assert type(receipt) is dict and set(receipt)=={'hash','block','status','segments','fee_speck','finalized','finalized_height','bytes_match','canonical','full_receipt_guard','physical_intents','physical_contract_calls','logical_guaranteed_segment','signed_dust_authority_speck','fee_with_margin_speck'}
 assert type(receipt['block']) is dict and set(receipt['block'])=={'height','hash','timestamp'}
 assert SHA.fullmatch(norm(receipt['block']['hash'])) and type(receipt['finalized_height']) is int and receipt['finalized_height']>=0
 assert receipt['status']=='SUCCESS' and receipt['segments'] is None
 for key in ['finalized','bytes_match','canonical','full_receipt_guard']:assert receipt[key] is True,key
 assert norm(receipt['hash'])==norm(safe['transaction_hash'])
 assert receipt['physical_intents']==[1,2] and all(type(v) is int for v in receipt['physical_intents']) and type(receipt['physical_contract_calls']) is int and receipt['physical_contract_calls']==(1 if cfg['action']=='accept' else 0) and type(receipt['logical_guaranteed_segment']) is int and receipt['logical_guaranteed_segment']==0
 for k in ['fee_speck','signed_dust_authority_speck','fee_with_margin_speck']:assert type(receipt[k]) is str and re.fullmatch(r'[1-9][0-9]*',receipt[k])
 assert int(receipt['fee_speck'])<=int(receipt['signed_dust_authority_speck']) and int(receipt['fee_speck'])<=int(receipt['fee_with_margin_speck'])
 if cfg['action']=='accept':
  # One read: the bytes parsed are the bytes hashed against call_proof_record_sha256.
  raw=read(Path(cfg['outputRoot'])/'safe-result.json',1048576);proof=parse(raw);assert type(proof) is dict and type(safe['call_proof_record_sha256']) is str and hashlib.sha256(raw).hexdigest()==safe['call_proof_record_sha256'],'C3 record sha'
  assert proof['submission_intent']['address']==safe['address'] and proof['submission_intent']['pre_state_sha256']==safe['pre_state_sha256'] and proof['independent_native_verify'] is True and proof['production_adapter_verify'] is True
  check_call_proof(safe,proof,binding)
 return dict(safe,post_submission_wallet_dust='UNOBSERVED',post_submission_wallet_pending='UNOBSERVED',fee_proving_data=fee_data_projection(cfg))
