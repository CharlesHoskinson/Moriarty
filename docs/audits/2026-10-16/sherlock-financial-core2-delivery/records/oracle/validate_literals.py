"""Data-only validation of independent literal transcription; no evaluation."""
from pathlib import Path
import hashlib,json
R=Path(__file__).resolve().parent
W=Path('/home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016')
def read(p):return json.loads(p.read_bytes())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def canonical(d):return hashlib.sha256(json.dumps(d,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()).hexdigest()
freeze=read(R/'NORMATIVE-FREEZE.json')
for f in freeze['files']:
 assert sha(R/'normative-source'/f['path'])==f['sha256']==sha(W/f['path'])
inputs=read(R/'inputs.json');expected=read(R/'expected.json');accounting=read(R/'accounting.json')
segments=[];invocations=0;false_count=0;cases=[]
for inp,exp,acc in zip(inputs['cases'],expected['cases'],accounting['cases']):
 assert inp['case_id']==exp['case_id']==acc['case_id']
 p=exp['program_ir'];plan=exp['stage_plan'];pa=plan['actions'][0];work=exp['work_schedule'];action=p['actions'][0]
 assert sha(R/inp['source'])==exp['source_sha256']
 assert exp['program_hash']==canonical(p)==plan['program_hash']
 assert exp['plan_hash']==canonical(plan)
 assert p['language']==inputs['source_profile']=='moriarty/2'
 assert p['semantics']==inputs['semantics']=='moriarty-core/2'
 assert action['name']==inp['action']==pa['action']
 assert len(action['statements'])==pa['statement_count']==len(work['statement_costs_on_execution'])
 assert all(int(v)>=2 for v in work['statement_costs_on_execution'])
 assert work['successful_segment_costs']==pa['segment_cost'] and work['committed_before']==pa['committed_before'] and work['plan_total']==pa['plan_total']==inp['start_cap']
 assert sum(map(int,pa['segment_cost']))==int(pa['plan_total'])
 running=0
 for before,cost in zip(pa['committed_before'],pa['segment_cost']):
  assert int(before)==running;running+=int(cost)
 for i,e in enumerate(p['expressions']):
  refs=([e['lhs'],e['rhs']] if e['kind']=='Binary' else [e['value']] if e['kind']=='Not' else [])
  assert all(type(j) is int and 0<=j<i for j in refs)
 for s in action['statements']:
  roots=[s[k] for k in ['value','predicate','amount','guard'] if k in s]
  assert len(roots)==1 and 0<=roots[0]<len(p['expressions'])
 assert [s['ordinal'] for s in pa['sites']]==[i for i,s in enumerate(action['statements']) if s['kind']=='Await']
 for site in pa['sites']:
  stmt=action['statements'][site['ordinal']]
  assert site['label']==stmt['label']
  assert site['binders']==stmt['binders']
 initial=read(R/inp['scenario'])
 assert len(initial['inputs'])==len(action['inputs'])
 assert set(initial['inputs'])=={i['name'] for i in action['inputs']}
 seq=inp['sequence'];obs=exp['invocations'];assert len(seq)==len(obs)
 outcomes={};these_segments=[];statement_indices=[]
 for call,ob in zip(seq,obs):
  invocations+=1
  assert (call['id'],call['operation'])==(ob['id'],ob['operation'])
  if call['operation']=='resume':
   assert call['continuation_from']==ob['continuation_from']
   expected_input=outcomes[call['continuation_from']]['Waiting']['continuation']
   assert ob['expected']['input']==expected_input
   assert ob['expected']['event']==read(R/call['event'])
   assert ob['cli_action'] is call['cli_action'] is None and ob['cli_cap'] is call['cli_cap'] is None
   out=ob['expected']['outcome']
  else:out=ob['expected']
  assert len(out)==1
  kind,body=next(iter(out.items()));assert kind in ['Waiting','Complete']
  outcomes[call['id']]=out
  if kind=='Waiting':
   con=body['continuation'];assert con['program_hash']==exp['program_hash'] and con['plan_hash']==exp['plan_hash']
   assert con['action']==inp['action'] and con['format']=='moriarty-continuation/1'
   assert con['logical']['profile']==work['profile'] and con['logical']['cap']==inp['start_cap']
   site=pa['sites'][con['site']-1]
   assert [v['name'] for v in con['captures']]==[c['name'] for c in site['captures']]
   assert [v['value']['type'] for v in con['captures']]==[c['ty'] for c in site['captures']]
   assert con['logical']['committed']==pa['committed_before'][con['site']]
   assert body['residual']==[d for d in con['boundary']['liabilities'] if int(d['amount'])>0]
   if body['advanced'] is False:
    false_count+=1;assert body['segment'] is None
    assert con==ob['expected']['input']
    assert body['attempted']==next(x['attempted'] for x in work['false_guard_attempts'] if x['site']==con['site'])
    continue
  seg=body['segment'];ex=seg['execution'];segments.append(seg);these_segments.append(seg)
  assert seg['action']==ex['action']==inp['action'] and ex['semantics']==p['semantics']
  if len(these_segments)==1:
   assert seg['frame_pre']=={'site':0,'captures':[]}
   assert ex['pre']['balances']==initial['balances'] and ex['pre']['liabilities']==initial['liabilities']
  else:
   assert ex['pre']==ob['expected']['input']['boundary']
   assert seg['frame_pre']=={'site':ob['expected']['input']['site'],'captures':ob['expected']['input']['captures']}
  if kind=='Waiting':
   assert ex['post']==con['boundary']
   assert seg['frame_post']=={'site':con['site'],'captures':con['captures']}
  else:
   assert seg['frame_post']=={'site':0,'captures':[]}
   assert body['committed']==pa['plan_total']
   assert body['residual']==[d for d in ex['post']['liabilities'] if int(d['amount'])>0]
  assert body['attempted']==pa['segment_cost'][len(these_segments)-1]
  assert ex['effects']==[f for st in ex['steps'] for f in st['effects']]
  assert ex['steps'][-1]['snapshot']==ex['post']
  for st in ex['steps']:
   statement_indices.append(st['index'])
   assert st['statement']==action['statements'][st['index']]
   assert st['value']['type'] in ['Bool','UInt']
   snap=st['snapshot']
   assert set(snap['state'])=={s['name'] for s in p['states']}
   keys=[(b['asset']['network'],b['asset']['issuer'],b['asset']['reference'],b['asset']['scale'],b['account']) for b in snap['balances']]
   assert keys==sorted(keys) and len(set(keys))==len(keys)
   assert [d['name'] for d in snap['liabilities']]==sorted(d['name'] for d in snap['liabilities'])
   assert all(int(b['amount'])>=0 for b in snap['balances'])
  for snap in [ex['pre'],ex['post']]+[s['snapshot'] for s in ex['steps']]:
   # Preserved per-asset supply in these transfers/fees only (no mint/burn).
   for assetdecl in p['assets']:
    id=assetdecl['identity']
    assert sum(int(b['amount']) for b in snap['balances'] if b['asset']==id)==sum(int(b['amount']) for b in initial['balances'] if b['asset']==id)
 assert statement_indices==list(range(len(action['statements'])))
 for annotations,seg in zip(acc['published_segments'],these_segments):
  ex=seg['execution'];flows=[f for f in ex['effects'] if f['kind'] in ['Transfer','Fee']]
  for row in annotations['gross_debits']:
   assert int(row['amount'])==sum(int(f['amount']) for f in flows if f['asset']==row['asset'] and f['from']==row['account'])
  for row in annotations['fee_debits']:
   assert int(row['amount'])==sum(int(f['amount']) for f in flows if f['kind']=='Fee' and f['asset']==row['asset'] and f['from']==row['account'])
  for k,kind in [('transfer_receipts','Transfer'),('fee_receipts','Fee')]:
   for row in annotations[k]:assert int(row['amount'])==sum(int(f['amount']) for f in flows if f['kind']==kind and f['asset']==row['asset'] and f['to']==row['account'])
  for row in annotations['duty_evolution']:
   assert int(row['opening'])+int(row['created_or_accrued'])-int(row['discharged'])==int(row['closing'])
 for row in acc['cumulative']['duty_evolution']:
  assert int(row['opening'])+int(row['created_or_accrued'])-int(row['discharged'])==int(row['closing'])
 cases.append({'case_id':inp['case_id'],'program_hash':exp['program_hash'],'plan_hash':exp['plan_hash'],'successful_segment_costs':pa['segment_cost'],'plan_total':pa['plan_total'],'invocations':len(obs),'published_segments':len(these_segments),'steps':len(statement_indices),'effects':sum(len(s['execution']['effects']) for s in these_segments)})
assert invocations==8 and false_count==3 and len(segments)==5
assert sum(len(s['execution']['steps']) for s in segments)==29
assert sum(len(s['execution']['effects']) for s in segments)==22
# Independently written sums of statement-root work + next-await entry charges.
w1=expected['cases'][0]['work_schedule']['statement_costs_on_execution']
assert sum(map(int,w1[:6]))+1==17 and sum(map(int,w1[6:]))==18
w2=expected['cases'][1]['work_schedule']['statement_costs_on_execution']
assert sum(map(int,w2[:5]))+1==15 and sum(map(int,w2[5:9]))+1==15 and sum(map(int,w2[9:]))==28
print(json.dumps({'status':'pass','scope':'literal data structure/source pins/hash/accounting consistency only; no parser/evaluator/subject execution','normative_files':len(freeze['files']),'cases':cases,'invocations':8,'false_ordinary_observations':3,'published_segments':5,'steps':29,'effects':22,'native_authority_used':0,'acceptance':'none'},indent=2))
