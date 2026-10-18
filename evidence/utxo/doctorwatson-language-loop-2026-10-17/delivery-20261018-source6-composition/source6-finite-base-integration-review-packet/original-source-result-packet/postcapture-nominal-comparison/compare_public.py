"""Compare immutable public captures to immutable independent facts. No producer/runtime imports."""
import pathlib, json, hashlib, re, collections, datetime
R=pathlib.Path('/home/charl/moriarty-c2-opus55-restored-20261017')
N=R/'source6-nominal-mapping-independent-oracles'
A=R/'source6-public-consumer-check-cycle/node-observations'
O=pathlib.Path(__file__).resolve().parent
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
assert sha(N/'FREEZE.sha256')=='bdd550f9b32b94919e67caa3d444ef0f5ea583903cbc85ec38513f1451bd6107'
for line in (N/'FREEZE.sha256').read_text().splitlines():
 digest,name=line.split('  ',1)
 assert sha(N/name)==digest, name
E=json.loads((N/'REQUIRED-NOMINAL-AND-FINANCIAL-FIELDS.json').read_text())
C=json.loads((N/'INPUT-FIELD-COVERAGE.json').read_text())
F=json.loads((N/'FINANCIAL-PRE-POST.json').read_text())
RUN=json.loads((A/'run.json').read_text())
assert RUN['financialFreezeSha256']=='64a6b4a6e454bbc6bc2cf3582581e81e3dd5fd379f9ed3fc89bbfc8051180269'
MISSING=object()
def pointer(obj,p):
 try:
  for k in p.strip('/').split('/') if p else []:obj=obj[int(k)] if isinstance(obj,list) else obj[k]
  return obj
 except (KeyError,IndexError,TypeError,ValueError):return MISSING
def get(j,p):
 v=pointer(j,p)
 if v is MISSING:raise KeyError(p)
 return v
def canonical_action(x):
 # Wire Transfer plus explicit fee operands is the frozen TransferLiteralFee constructor.
 if x.get('kind')!='Transfer':return x
 return {'constructor':'TransferLiteralFee','owner':x.get('from'),'recipient':x.get('to'),'feeRecipient':x.get('feeTo'),'value':x.get('value'),'fee':x.get('fee')}
def flat_action(x):
 if x.get('kind')!='Transfer':return x
 return {'constructor':'TransferLiteralFee','owner':x.get('signer'),'recipient':x.get('recipient'),'feeRecipient':x.get('feeRecipient'),'value':x.get('amount'),'fee':x.get('fee')}
def effects(x,source=False):
 out=[]
 for e in x:
  k=e['kind']
  if k in ('Debit','Credit'):args=[e['account'],e['amount']]
  elif k=='UseAllowance':args=[e['owner'],e['amount']]
  elif k=='UseReplay':args=[e['key'] if source else json.loads(e['key'])]
  elif k=='AdvanceHead':args=[e['predecessor'],e['successor']]
  else:out.append(e);continue
  if source:out.append({'production':{'Debit':'debit','Credit':'credit','UseAllowance':'useAllowance','UseReplay':'useReplay','AdvanceHead':'advanceHead'}[k],'operands':args})
  else:out.append({'constructor':k,'arguments':args})
 return out
def financial(s,intent):
 replay=s['consumedReplay'];components=[intent['domain'],intent['signer'],intent['nonce']]
 return {'domain':s['domain'],'asset':s['asset'],'head':s['head'],'balances':s['balances'],'allowance':s['allowances'][0] if len(s['allowances'])==1 else s['allowances'],'obligation':'absent in closed transfer input' if s['obligations']==[] else s['obligations'],'positive_residual_duties':s['obligations'],'replay':{'key_components':components if not replay else (json.loads(replay[0]) if len(replay)==1 else replay),'status':'unused' if not replay else 'consumed'}}
all_target=[];all_core=[];all_coverage=[];all_supp=[];observed_inputs={}
def record(case,group,key,expected,checks,reason=None):
 result={'case':case,'field':key,'expected_frozen_ref':f'{N}/REQUIRED-NOMINAL-AND-FINANCIAL-FIELDS.json#/cases/{case}/{group}/{key}','expected':expected,'observations':checks}
 if reason:result.update(status='unasserted',reason=reason)
 elif not checks:result.update(status='missing',reason='No mapped observation')
 elif any(c.get('missing') for c in checks):result['status']='missing'
 elif all(c['value']==expected for c in checks):result['status']='matched'
 else:result['status']='mismatch'
 return result
def obs(path,value,adapter='identity'):return {'observed_path':path,'adapter':adapter,'value':value}
for case,ce in E['cases'].items():
 j=json.loads((A/case/'actual-consumer.json').read_text());process=json.loads((A/case/'process.json').read_text())
 text=(A/case/'stdout.source6').read_text();lines=text.splitlines();ast=j['lowered']['ast'];intent=j['lowered']['intent'];state=j['lowered']['state'];candidate=j['outcome']['candidate']
 for p in [A/'run.json',A/case/'actual-consumer.json',A/case/'stdout.source6',A/case/'stderr.txt',A/case/'process.json']:observed_inputs[str(p)]=sha(p)
 b=str(A/case/'actual-consumer.json')+'#';sr=str(A/case/'stdout.source6')
 def jo(p,fn=lambda x:x,adapter='identity'):
  v=pointer(j,p)
  return {'observed_path':b+p,'adapter':adapter,'missing':True} if v is MISSING else obs(b+p,fn(v),adapter)
 def source(pattern,convert=lambda m:m.group(1)):
  hits=[(i+1,re.fullmatch(pattern,l)) for i,l in enumerate(lines)]
  hits=[(i,m) for i,m in hits if m]
  if len(hits)!=1:return {'observed_path':sr+'#regex:'+pattern,'missing':True,'matches':len(hits)}
  line,m=hits[0];return obs(sr+':'+str(line),convert(m),'finite line-token extraction; no language parser/runtime')
 def multi(pattern,convert):
  hits=[(i+1,re.fullmatch(pattern,l)) for i,l in enumerate(lines)];hits=[(i,m) for i,m in hits if m]
  return obs(sr+'#lines:'+','.join(str(i) for i,m in hits),[convert(m) for i,m in hits],'ordered line-token extraction; no sorting')
 # Each target-production descriptor is mapped independently to raw tokens AND available parsed AST.
 mapping={
 'profile_raw_token':(r'profile ("[^"]+");','/lowered/ast/profile',lambda x:json.dumps(x)),
 'domain_identifier':(r'domain (\w+);','/lowered/ast/domain',lambda x:x),
 'settlement_identifier':(r'settlement (\w+) scale \d+;','/lowered/ast/settlement/asset',lambda x:x),
 'settlement_scale_integer_token':(r'settlement \w+ scale (\d+);','/lowered/ast/settlement/scale',lambda x:x),
 'selected_action_identifier':(r'selected (\w+) source_hash "[^"]+" digest "[^"]+";','/lowered/ast/selected/actionId',lambda x:x),
 'selected_source_hash_string':(r'selected \w+ source_hash "([^"]+)" digest "[^"]+";','/lowered/ast/selected/sourceHash',lambda x:x),
 'selected_digest_string':(r'selected \w+ source_hash "[^"]+" digest "([^"]+)";','/lowered/ast/selected/policyDigest',lambda x:x),
 'intent_signer_identifier':(r'signer (\w+) key "[^"]+";','/lowered/ast/intent/signer',lambda x:x),
 'intent_key_string':(r'signer \w+ key "([^"]+)";','/lowered/ast/intent/keyRef',lambda x:x),
 'intent_nonce_string':(r'nonce "([^"]+)";','/lowered/ast/intent/nonce',lambda x:x),
 'intent_pre_head_string':(r'pre_head "([^"]+)";','/lowered/ast/intent/preHead',lambda x:x),
 'authenticated_head_string':(r'head "([^"]+)";','/lowered/ast/authenticated/head',lambda x:x),
 'authenticated_predecessor_string':(r'predecessor "([^"]+)";','/lowered/ast/authenticated/predecessor',lambda x:x),
 'authenticated_round_integer_token':(r'round (\d+);','/lowered/ast/authenticated/round',lambda x:x),
 'authenticated_work_remaining_integer_token':(r'work_remaining (\d+);','/lowered/ast/authenticated/workRemaining',lambda x:x),
 'authenticated_work_spent_integer_token':(r'work_spent (\d+);','/lowered/ast/authenticated/workSpent',lambda x:x),
 'authenticated_replay_token':(r'replay (\w+);','/lowered/ast/authenticated/replay',lambda x:x),
 'authenticated_allowance_owner_identifier':(r'allowance (\w+) remaining \d+ spent \d+;','/lowered/ast/authenticated/allowance/owner',lambda x:x),
 'authenticated_allowance_remaining_integer_token':(r'allowance \w+ remaining (\d+) spent \d+;','/lowered/ast/authenticated/allowance/remaining',lambda x:x),
 'authenticated_allowance_spent_integer_token':(r'allowance \w+ remaining \d+ spent (\d+);','/lowered/ast/authenticated/allowance/spent',lambda x:x),
 'submitted_post_head_string':(r'post_head "([^"]+)";','/lowered/ast/submitted/postHead',lambda x:x)}
 for key,wire in [('gross_cap','grossCap'),('fee_cap','feeCap'),('net_floor','netFloor'),('failure','failure'),('observations','observations'),('disclosures','disclosures'),('retained_effects','retainedEffects'),('retained_duties','retainedDuties'),('delegation','delegation'),('recovery','recovery')]:
  mapping['intent_'+key+('_integer_token' if key.endswith('cap') or key=='net_floor' else '_token')]=(key+r' (\w+);','/lowered/ast/intent/'+wire,lambda x:x)
 target_results={}
 for key,expected in ce['target_production_values'].items():
  if key in mapping:
   pattern,path,fn=mapping[key];checks=[source(pattern),jo(path,fn,'JSON quote adapter' if key=='profile_raw_token' else 'identity')]
  elif key=='authenticated_balances_ordered':checks=[multi(r'balance (\w+) (\d+);',lambda m:{'account':m[1],'amount':m[2]}),jo('/lowered/ast/authenticated/balances')]
  elif key=='intent_valid_integer_tokens':checks=[source(r'valid (\d+)\.\.(\d+);',lambda m:[m[1],m[2]]),obs(b+'/lowered/ast/intent/{notBefore,notAfter}',[ast['intent']['notBefore'],ast['intent']['notAfter']],'ordered pair')]
  elif key in ('signed_action','submitted_action'):
   path='/lowered/ast/'+('intent/signedAction' if key=='signed_action' else 'submitted/action')
   prefix='signed_action' if key=='signed_action' else 'submit'
   pattern=prefix+r' transfer from (\w+) to (\w+) fee_to (\w+) value (\d+) fee (\d+);'
   checks=[source(pattern,lambda m:{'constructor':'TransferLiteralFee','owner':m[1],'recipient':m[2],'feeRecipient':m[3],'value':m[4],'fee':m[5]}),jo(path,canonical_action,'Transfer fields -> frozen constructor/operand facts')]
  elif key=='authenticated_obligation_presence':
   checks=[obs(sr+'#authenticated block', 'absent' if not any(re.match(r'\s*obligation\b',l) for l in lines) else 'present','absence of obligation production'),jo('/lowered/state/obligations',lambda x:'absent' if x==[] else x,'empty obligations -> absent')]
  elif key=='submitted_effect_productions_ordered':
   eff=[];refs=[];inside=False
   for i,l in enumerate(lines):
    if l=='effects {':inside=True;continue
    if inside and l=='}':break
    if inside:
     refs.append(i+1)
     p=l[:-1].split();name={'use_allowance':'useAllowance','use_replay':'useReplay','advance_head':'advanceHead'}.get(p[0],p[0]);eff.append({'production':name,'operands':[json.loads(v) if v.startswith('"') else v for v in p[1:]]})
   checks=[obs(sr+'#lines:'+','.join(map(str,refs)),eff,'production spelling snake_case -> frozen descriptive camelcase; ordered operands'),jo('/lowered/ast/submitted/effects',lambda x:effects(x,True),'Effect objects -> production/operands, asset separately checked')]
  else:raise AssertionError('Unmapped target '+key)
  rec=record(case,'target_production_values',key,expected,checks);target_results[key]=rec;all_target.append(rec)
 # Core5 facts are descriptive; fields absent from Core DTO use AST read claims only, explicitly referenced.
 corepaths={'assetScaleReadClaim':'/lowered/ast/settlement/scale','currentRoundReadClaim':'/lowered/state/round','domainId':'/lowered/intent/domain','feeCap':'/lowered/intent/feeCap','grossCap':'/lowered/intent/grossCap','keyRefClaim':'/lowered/intent/keyRef','netFloor':'/lowered/intent/netFloor','nonceClaim':'/lowered/intent/nonce','preHeadClaim':'/lowered/intent/preHead','predecessorReadClaim':'/lowered/ast/authenticated/predecessor','programId':'/lowered/intent/programId','selectedPolicyDigestClaim':'/lowered/intent/policyDigest','selectedSourceHashClaim':'/lowered/intent/sourceHash','settlementAssetId':'/lowered/intent/asset','signer':'/lowered/intent/signer','validRoundHi':'/lowered/intent/notAfter','validRoundLo':'/lowered/intent/notBefore'}
 coreres={}
 for key,expected in ce['Core5_semantic_constraints'].items():
  reason=None
  if key in corepaths:checks=[jo(corepaths[key])]
  elif key=='actionAndSignedAction':checks=[jo('/lowered/ast/intent/signedAction',canonical_action,'Transfer -> constructor facts'),jo('/lowered/ast/submitted/action',canonical_action,'Transfer -> constructor facts'),jo('/lowered/intent',flat_action,'flat Transfer intent -> constructor facts')]
  elif key=='orderedEffects':checks=[jo('/lowered/submittedEffects',effects,'ordered Effect -> constructor/arguments; replay key JSON decoded'),jo('/outcome/candidate/effects',effects,'same ordered adapter; no sorting')]
  elif key=='pre':checks=[obs(b+'/lowered/state + /lowered/intent',financial(state,intent),'finite financial frame projection; replay JSON component adapter')]
  elif key=='postFinancialIfLocalS0Admitted':checks=[obs(b+'/outcome/candidate/candidatePost + /lowered/intent',financial(candidate['candidatePost'],intent),'finite financial frame projection; ignores only unasserted work counters')]
  elif key=='failurePolicy':checks=[jo('/lowered/ast/intent/failure',lambda x:{'success_only':'SuccessOnly'}.get(x,x),'frozen token -> descriptive enum')]
  elif key=='emptyObservationsDisclosuresRetainedEffectsRetainedDuties':checks=[obs(b+'/lowered/ast/intent/{observations,disclosures,retainedEffects,retainedDuties}',all(ast['intent'][k]=='empty' for k in ['observations','disclosures','retainedEffects','retainedDuties']),'conjunction of four exact empty tokens')]
  elif key=='noDelegationNoRecovery':checks=[obs(b+'/lowered/ast/intent/{delegation,recovery}',all(ast['intent'][k]=='none' for k in ['delegation','recovery']),'conjunction of two exact none tokens')]
  elif key=='replayKeyComponents':checks=[obs(b+'/lowered/intent/{domain,signer,nonce}',[intent['domain'],intent['signer'],intent['nonce']],'ordered composite-key components')]
  elif key=='workReadClaims':checks=[obs(b+'/lowered/state/{workRemaining,workSpent}',{'remaining':state['workRemaining'],'spent':state['workSpent']},'opening counter projection')]
  elif key=='postWork':checks=[jo('/outcome/candidate/candidatePost/workRemaining'),jo('/outcome/candidate/candidatePost/workSpent')];reason='Frozen UNASSERTED: actual post counters recorded only, never treated as expected values.'
  elif key=='qualification':checks=[jo('/outcome/status'),jo('/outcome/candidate/status')];reason='Frozen qualification is a scope/premise limitation, not a DTO value. PreparedUnqualified is separately reported; no trust/admission inference.'
  else:raise AssertionError('Unmapped core '+key)
  rec=record(case,'Core5_semantic_constraints',key,expected,checks,reason);all_core.append(rec);coreres[key]=rec
 # All 124 original coverage rows: either resolve to fixed semantic constraints or retain unresolved metadata.
 for idx,row in enumerate(C['cases'][case]['rows']):
  f=row['input_field'];keys=[];ck=[];reason=None
  if f=='profile':keys=['profile_raw_token']
  elif f=='agreement':reason='Exact beta->Source6 agreement-token rule is frozen UNASSERTED (NM01); observed name does not resolve that rule.'
  elif f in ['domain.alias_in_source','domain.chain','domain.network','asset.USD.alias','asset.USD.representation','asset.USD.symbol']:reason='Frozen coverage states no separate target production for this metadata; no preservation or synthesized absence claim inferred.'
  elif f=='domain.id' or f.endswith('.domain') or f=='asset.USD.domain_alias':keys=['domain_identifier']
  elif f.startswith('account.'):
   role=f.split('.')[1];keys=[{'Buyer':'intent_signer_identifier','Seller':'signed_action','Treasury':'signed_action'}[role]]
  elif f=='asset.USD.id':keys=['settlement_identifier']
  elif f=='asset.USD.scale':keys=['settlement_scale_integer_token']
  elif f in ['const.base','const.supplement']:reason='Source6 has resolved operation value only; individual source constant/intermediate cells have no independent target field. Final price is compared separately.'
  elif f.startswith('const.'):keys=['signed_action','submitted_action'];ck=['actionAndSignedAction']
  elif f in ['intent.name','action.pay.uses']:keys=['selected_action_identifier'];ck=['programId']
  elif f=='intent.asset':keys=['settlement_identifier']
  elif f.startswith('intent.operation.transfer.'):keys=['signed_action','submitted_action'];ck=['actionAndSignedAction']
  elif f.startswith('intent.'):
   tail=f[7:]; mp={'signer':'intent_signer_identifier','key':'intent_key_string','nonce':'intent_nonce_string','pre_head':'intent_pre_head_string','source_hash':'selected_source_hash_string','policy_digest':'selected_digest_string','valid.from':'intent_valid_integer_tokens','valid.to':'intent_valid_integer_tokens','gross_cap':'intent_gross_cap_integer_token','fee_cap':'intent_fee_cap_integer_token','net_floor':'intent_net_floor_integer_token','failure':'intent_failure_token','observations':'intent_observations_token','disclosures':'intent_disclosures_token','retained_effects':'intent_retained_effects_token','retained_duties':'intent_retained_duties_token','delegation':'intent_delegation_token','recovery':'intent_recovery_token'};keys=[mp[tail]]
  elif f.startswith('scenario.'):
   tail=f[9:]
   if tail in ['profile','kind']:reason='Input-only profile/local-stipulation marker has no separate Source6/Core output field. Recorded process success is not a byte-preservation claim.'
   else:
    mp={'domain':'domain_identifier','asset':'settlement_identifier','head':'authenticated_head_string','predecessor':'authenticated_predecessor_string','round':'authenticated_round_integer_token','balances':'authenticated_balances_ordered','allowance':'authenticated_allowance_owner_identifier','replay':'authenticated_replay_token','work_remaining':'authenticated_work_remaining_integer_token','work_spent':'authenticated_work_spent_integer_token','post_head':'submitted_post_head_string'};keys=[mp[tail]]
    if tail=='allowance':keys+=['authenticated_allowance_remaining_integer_token','authenticated_allowance_spent_integer_token']
  else:raise AssertionError('Unmapped coverage '+f)
  refs=[target_results[k] for k in keys]+[coreres[k] for k in ck]
  statuses=[r['status'] for r in refs]
  status='unasserted' if reason else ('missing' if not refs or 'missing' in statuses else 'mismatch' if 'mismatch' in statuses else 'matched')
  all_coverage.append({'case':case,'index':idx,'input_field':f,'frozen_coverage_ref':f'{N}/INPUT-FIELD-COVERAGE.json#/cases/{case}/rows/{idx}','frozen_input_value':row['value'],'frozen_resolution':row['resolution'],'status':status,'reason':reason,'fixed_expected_refs':[r['expected_frozen_ref'] for r in refs],'observed_paths':[c['observed_path'] for r in refs for c in r['observations']],'adapter':'Resolve only according to frozen coverage destination and required nominal constraints; source aliases are not compared literally to economic IDs.'})
 # Additional representation/frame evidence, not a substitute denominator or new financial oracle.
 for path in ['/lowered/submittedEffects','/outcome/candidate/effects']:
  v=get(j,path)
  checks=[e.get('asset')==ce['Core5_semantic_constraints']['settlementAssetId'] for e in v if e.get('kind') in ('Debit','Credit')]
  all_supp.append({'case':case,'field':'Debit/Credit exact asset fields','expected_ref':f'{N}/REQUIRED-NOMINAL-AND-FINANCIAL-FIELDS.json#/cases/{case}/Core5_semantic_constraints/settlementAssetId','observed_path':b+path,'status':'matched' if checks and all(checks) else 'mismatch'})
 all_supp += [{'case':case,'field':'outcome AST equals directly lowered AST','observed_paths':[b+'/lowered/ast',b+'/outcome/ast'],'status':'matched' if j['outcome']['ast']==ast else 'mismatch'}, {'case':case,'field':'public process exit and quiet stderr','observed_paths':[str(A/case/'process.json')+'#/status',str(A/case/'stderr.txt')],'status':'matched' if process['status']==0 and (A/case/'stderr.txt').read_bytes()==b'' else 'mismatch','observed_process_status':process['status']}, {'case':case,'field':'single final LF, literal stdout retained as artifact not whole golden','observed_path':sr,'status':'matched' if text.endswith('\n') and not text.endswith('\n\n') else 'mismatch'}]
def counts(rows):return {k:sum(r['status']==k for r in rows) for k in ['matched','unasserted','missing','mismatch']}
def put(n,x):(O/n).write_text(json.dumps(x,indent=2)+'\n')
assert len(all_coverage)==124
put('COMPARISON-124-ROWS.json',{'denominator':'original INPUT-FIELD-COVERAGE rows only; no wider language coverage percentage','counts':counts(all_coverage),'rows':all_coverage})
put('DECLARED-TARGET-AND-CORE-COMPARISON.json',{'target_production_counts':counts(all_target),'Core5_constraint_counts':counts(all_core),'target_production_rows':all_target,'Core5_rows':all_core,'supplemental_checks':all_supp})
summary={'status':'postcapture mechanical comparison against preexisting frozen expectations','created_at_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'nominal_freeze_sha256':sha(N/'FREEZE.sha256'),'required_fields_sha256':sha(N/'REQUIRED-NOMINAL-AND-FINANCIAL-FIELDS.json'),'financial_source_packet_freeze_sha256':RUN['financialFreezeSha256'],'counts_124':counts(all_coverage),'per_case_124':{case:counts([r for r in all_coverage if r['case']==case]) for case in E['cases']},'declared_target_counts':counts(all_target),'declared_Core5_counts':counts(all_core),'supplemental_counts':counts(all_supp),'observed_outcomes':{case:json.loads((A/case/'actual-consumer.json').read_text())['outcome']['status'] for case in E['cases']},'observed_input_sha256':observed_inputs,'no_runtime_execution':True,'limits':['This is a nonauthor postcapture mechanical comparison, not new independent oracle derivation or an audit of execution/trust adequacy.','The 124 coverage rows include metadata without a target production; unasserted rows remain explicit and do not count as matches.','Core source claims such as scale/predecessor/failure/none/empty can be compared in AST only; AST equality does not establish binding or authentication.','Qualification and post-work remain unasserted. Exact Source6 agreement-name mapping, whole wire bytes, richer metadata/origin mapping, native/proof/ledger and parent acceptance remain unresolved.','Financial projections ignore work counters by frozen scope only; arrays retain original order. Replay key representation JSON is decoded, not reordered.','No original oracle files were edited, no expected value was taken from actual output, and no producer/lowerer/runtime/Cargo/SDK/Git/W/I/J/K write was invoked.']}
put('SUMMARY.json',summary)
(O/'ADAPTERS.md').write_text('Expected values come only from frozen REQUIRED-NOMINAL-AND-FINANCIAL-FIELDS.json. The 124-row input coverage table references those fixed values and original row references. Observations come only from named public captures. compare_public.py contains the entire comparison, with no repository/module/runtime imports.\n\nRaw Source6 is extracted with anchored finite line patterns, preserving ordered balance/effect rows; this is passive artifact extraction rather than execution of a language parser/lowerer. JSON AST properties use explicit paths. Transfer wire objects map from/from-to/feeTo/value/fee into frozen TransferLiteralFee constructor facts; the flat lowered intent uses signer/recipient/feeRecipient/amount/fee. Effect wire objects map to descriptive constructor/arguments without sorting, including JSON decoding of composite replay keys. AST effect productions use the nonce token, while lowered/candidate effects retain the full composite replay key. Debit/Credit asset fields are checked separately against the frozen settlement asset.\n\nThe financial projection retains exact domain/asset/head/balances/allowance/obligation/replay data. An empty obligation array maps to the frozen descriptive absent marker. Empty consumedReplay is unused for the domain/signer/nonce from the same observed intent; a single consumed replay JSON string is decoded to components. No successful work policy is inferred. AST metadata read claims are distinguished from locally bound Core fields.\n\nUnasserted coverage rows: exact agreement name rule; source metadata with no output production; Workshop base/supplement intermediate constants not independently exposed; scenario input profile and local-stipulation kind absent from output schema. Individual final constants map only to the frozen resolved submitted/signed action values, not source-origin provenance. Overall recorded outcomes remain PreparedUnqualified.\n')
inv={p.name:sha(p) for p in sorted(O.iterdir()) if p.is_file() and p.name not in ['INVENTORY.json','FREEZE.json']}
put('INVENTORY.json',inv)
put('FREEZE.json',{'status':'COMPLETE POSTCAPTURE MECHANICAL COMPARISON','at_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'inventory_sha256':sha(O/'INVENTORY.json'),'original_nominal_freeze_sha256':sha(N/'FREEZE.sha256'),'comparison_is_new_oracle_derivation':False})
print(json.dumps({k:summary[k] for k in ['counts_124','per_case_124','declared_target_counts','declared_Core5_counts','supplemental_counts','observed_outcomes']},indent=2))
print('inventory_sha256='+sha(O/'INVENTORY.json'))
print('freeze_sha256='+sha(O/'FREEZE.json'))
