"""Bounded disjoint financial history specification checker; no cryptography."""
import copy,hashlib,importlib.util,json
from pathlib import Path
from jsonschema import Draft202012Validator
HERE=Path(__file__).parent
spec=importlib.util.spec_from_file_location('context03',HERE/'context-checker-05.py');C=importlib.util.module_from_spec(spec);spec.loader.exec_module(C)
PROFILE={'format':'branch-profile-5','contextProfileHash':C.digest('profile',{'profile':C.PROFILE,'schema':C.schema_for(C.PROFILE)}),'splitCharge':1,'joinCharge':1,'maxNodes':32,'maxChildren':8,'maxBytes':524288}
ACCOUNTS=C.accounts(C.PROFILE)
need=C.need;Reject=C.Reject

def digest(kind,value):return hashlib.sha256(('MORIARTY-SIGNING-BRANCH-EXAMPLE/5/'+kind).encode()+b'\0'+C.canonical(value)).hexdigest()
def profile_hash():return digest('profile',{'parameters':PROFILE,'schema':json.loads((HERE/'branch-schema-05.json').read_text())})
def refkey(ref):return (ref['node'],ref['output'])
def zero_debt():return {k:'0' for k in ['created','accrued','repaid','writtenOff','credited','outstanding']}
def root_output(seed):
 state=seed['steps'][-1]['post'] if seed['steps'] else seed['initial'];a=seed['authority']
 return {'state':copy.deepcopy(state),'grossRemaining':{k:str(int(a['grossCaps'][k])-int(state['gross'][k])) for k in ACCOUNTS},'feeRemaining':{k:str(int(a['feeCaps'][k])-int(state['fees'][k])) for k in ACCOUNTS},'ownedAccounts':list(ACCOUNTS),'ownsDebt':True,'retiredOrdinary':'0','retiredRecovery':'0'}
def record_check(out):
 C.check_generated(out['state'])
 need(len(set(out['ownedAccounts']))==len(out['ownedAccounts']) and set(out['ownedAccounts'])<=set(ACCOUNTS),'OWNERSHIP')
 for k in ACCOUNTS:
  need(all(0<=int(out[f][k])<=C.MAX for f in ['grossRemaining','feeRemaining']),'AUTHORITY_DOMAIN')
  if k not in out['ownedAccounts']:need(int(out['state']['balances'][k])==0 and int(out['grossRemaining'][k])==0 and int(out['feeRemaining'][k])==0,'OWNERSHIP')
 if not out['ownsDebt']:need(out['state']['debt']==zero_debt(),'DEBT_OWNER')
 for d in out['state']['duties']:
  k=d['debtor']+'.'+d['asset'];need(k in out['ownedAccounts'],'BACKING_OWNER')
 for k in ACCOUNTS:
  backing=sum(int(d['amount']) for d in out['state']['duties'] if d['debtor']+'.'+d['asset']==k)
  need(backing<=int(out['state']['balances'][k]) and backing<=int(out['grossRemaining'][k]),'DUTY_BACKING')
 for k in ['retiredOrdinary','retiredRecovery']:need(0<=int(out[k])<=C.MAX,'RETIRED_DOMAIN')
def split(parent,body):
 parts=body['partitions'];owned=[k for p in parts for k in p['accounts']]
 need(sorted(owned)==sorted(parent['ownedAccounts']) and len(set(owned))==len(owned),'OWNERSHIP_PARTITION')
 need(sum(p['ownsDebt'] for p in parts)==int(parent['ownsDebt']),'DEBT_PARTITION')
 ro=int(body['retireOrdinary']);rr=int(body['retireRecovery'])
 need(sum(int(p['ordinary']) for p in parts)+ro==int(parent['state']['ordinary'])-PROFILE['splitCharge'] and sum(int(p['recovery']) for p in parts)+rr==int(parent['state']['recovery']),'WORK_PARTITION')
 result=[]
 for index,p in enumerate(parts):
  out=copy.deepcopy(parent);out['ownedAccounts']=sorted(p['accounts']);out['ownsDebt']=p['ownsDebt'];state=out['state'];state['ordinary']=p['ordinary'];state['recovery']=p['recovery']
  for k in ACCOUNTS:
   if k not in p['accounts']:state['balances'][k]='0';out['grossRemaining'][k]='0';out['feeRemaining'][k]='0'
  if not p['ownsDebt']:state['debt']=zero_debt()
  state['duties']=[d for d in state['duties'] if d['debtor']+'.'+d['asset'] in p['accounts']]
  # Retired history has one structural owner, avoiding duplicated inherited totals.
  out['retiredOrdinary']=str(int(parent['retiredOrdinary'])+ro) if index==0 else '0';out['retiredRecovery']=str(int(parent['retiredRecovery'])+rr) if index==0 else '0'
  record_check(out);result.append(out)
 return result
def action(parent,plan,authority):
 C.display_arguments(plan)
 owned=set(parent['ownedAccounts'])
 for op in plan['operations']:
  if op['op'] in ['Transfer','Fee']:need({op['from']+'.'+op['asset'],op['to']+'.'+op['asset']}<=owned,'ACTION_OWNERSHIP')
  elif op['op'] in ['CreateDebt','Accrue','Repay']:need(parent['ownsDebt'],'DEBT_OWNER')
  elif op['op']=='Cancel':need({C.PROFILE['goalPayer']+'.'+C.PROFILE['goalAsset'],C.PROFILE['goalRecipient']+'.'+C.PROFILE['goalAsset']}<=owned,'BACKING_OWNER')
 auth=copy.deepcopy(authority)
 for k in ACCOUNTS:
  auth['grossCaps'][k]=str(int(parent['state']['gross'][k])+int(parent['grossRemaining'][k]));auth['feeCaps'][k]=str(int(parent['state']['fees'][k])+int(parent['feeRemaining'][k]))
 post,o,r=C.derive(parent['state'],plan,auth);out=copy.deepcopy(parent);out['state']=post
 for k in ACCOUNTS:
  out['grossRemaining'][k]=str(int(parent['grossRemaining'][k])-(int(post['gross'][k])-int(parent['state']['gross'][k])))
  out['feeRemaining'][k]=str(int(parent['feeRemaining'][k])-(int(post['fees'][k])-int(parent['state']['fees'][k])))
 record_check(out);return [out],o,r

def join(children,prefix):
 owned=[k for x in children for k in x['ownedAccounts']];need(sorted(owned)==sorted(prefix['ownedAccounts']) and len(set(owned))==len(owned),'JOIN_OWNERSHIP')
 need(sum(x['ownsDebt'] for x in children)==int(prefix['ownsDebt']),'JOIN_DEBT_OWNER')
 need(len({x['state']['version'] for x in children})==1 and len({x['state']['status'] for x in children})==1,'JOIN_STATE_COMPATIBILITY')
 out=copy.deepcopy(prefix);state=out['state'];out['ownedAccounts']=sorted(owned);out['ownsDebt']=any(x['ownsDebt'] for x in children)
 for f in ['gross','fees','receipts']:
  for k in ACCOUNTS:
   base=int(prefix['state'][f][k]);deltas=[int(x['state'][f][k])-base for x in children];need(all(v>=0 for v in deltas),'PREFIX_COUNTER');state[f][k]=str(base+sum(deltas))
 for k in ACCOUNTS:
  state['balances'][k]=str(sum(int(x['state']['balances'][k]) for x in children))
  for f in ['grossRemaining','feeRemaining']:out[f][k]=str(sum(int(x[f][k]) for x in children))
 for k in state['debt']:state['debt'][k]=str(sum(int(x['state']['debt'][k]) for x in children))
 state['duties']=[copy.deepcopy(d) for x in children for d in x['state']['duties']];need(len({d['id'] for d in state['duties']})==len(state['duties']),'JOIN_DUTY_ID')
 ordinary=sum(int(x['state']['ordinary']) for x in children)-PROFILE['joinCharge'];need(ordinary>=0,'WORK_EXHAUSTED');state['ordinary']=str(ordinary);state['recovery']=str(sum(int(x['state']['recovery']) for x in children))
 state['status']=children[0]['state']['status'];state['version']=children[0]['state']['version']
 for f in ['retiredOrdinary','retiredRecovery']:out[f]=str(sum(int(x[f]) for x in children))
 record_check(out);return [out]

def validate(graph,context):
 try:
  need(len(C.canonical(graph))<=PROFILE['maxBytes'],'GRAPH_BOUND');schema=json.loads((HERE/'branch-schema-05.json').read_text());need(not list(Draft202012Validator(schema).iter_errors(graph)),'GRAPH_SCHEMA')
  need(type(context) is dict and set(context)=={'base','consumedOutputs','compositionAuthorityHash'},'CONTEXT_SHAPE');need(type(context['consumedOutputs']) is set,'CONTEXT_SHAPE')
  seed=graph['seed'];checked=C.validate(seed,context['base']);need(checked['status']=='SOURCE_CONTEXT_VALID','SEED_'+checked.get('firstPredicate','UNKNOWN'))
  # A verified linear history does not imply that its final resource is still available.
  need(checked['head'] not in context['base']['consumed'],'SEED_CURRENTNESS')
  need(not checked['terminal'] or not graph['nodes'],'TERMINAL_CONTINUATION');occurrences=set(checked['consumedOccurrences'])
  origin=seed['authorityHash'];profile=profile_hash();need(graph['profileHash']==profile,'GRAPH_PROFILE');root=digest('root',seed)
  auth=graph['compositionAuthorization'];need(auth['originHash']==origin and auth['profileHash']==profile and digest('composition-authority',auth)==context['compositionAuthorityHash'],'COMPOSITION_AUTHORITY')
  locations={(root,'0'):'root'};live={(root,'0'):(root_output(seed),[])};splits={};seen={root};record_check(live[(root,'0')][0])
  for node_index,node in enumerate(graph['nodes']):
   body={k:v for k,v in node.items() if k!='id'};need(node['id']==digest('node',body),'NODE_HASH');need(node['id'] not in seen,'NODE_DUPLICATE');need(node['origin']==origin and node['profileHash']==profile,'IMMUTABLE_ORIGIN')
   refs=[refkey(r) for r in node['predecessors']];need(len(set(refs))==len(refs),'PREDECESSOR_DUPLICATE');need(all(r in live and r not in context['consumedOutputs'] for r in refs),'CURRENTNESS')
   selected=[live[r] for r in refs]
   need(int(seed['authority']['notBefore'])<=int(node['now'])<int(seed['authority']['notAfter']),'ACTIVATION')
   need(node['kind']=='Action' or node['occurrenceId']=='','NONACTION_OCCURRENCE')
   if node['kind']=='Split':
    need(len(selected)==1,'SPLIT_PREDECESSOR');parent,path=selected[0];outputs=split(parent,node['body']);o=PROFILE['splitCharge'];r=0;splits[node['id']]={'prefix':copy.deepcopy(parent),'path':path,'count':len(outputs)};paths=[path+[(node['id'],i)] for i in range(len(outputs))]
   elif node['kind']=='Action':
    need(len(selected)==1,'ACTION_PREDECESSOR');parent,path=selected[0];C.consume_occurrence(seed['authority'],node['occurrenceId'],C.digest('state',parent['state']),C.digest('plan',node['body']),C.digest('state',node['outputs'][0]['state']),False,occurrences,locations[refs[0]],digest('output',parent));outputs,o,r=action(parent,node['body'],seed['authority']);paths=[path]
   else:
    need(all(path for _,path in selected),'JOIN_LINEAGE');group=selected[0][1][-1][0];need(all(path[-1][0]==group for _,path in selected),'JOIN_LINEAGE');split_record=splits[group];need(sorted(path[-1][1] for _,path in selected)==list(range(split_record['count'])),'JOIN_SIBLINGS');outputs=join([out for out,_ in selected],split_record['prefix']);o=PROFILE['joinCharge'];r=0;paths=[split_record['path']]
   need(int(node['ordinaryCharge'])==o and int(node['recoveryCharge'])==r,'DERIVED_CHARGE');need(outputs==node['outputs'],'OUTPUT_RELATION')
   for ref in refs:del live[ref]
   for i,(out,path) in enumerate(zip(outputs,paths)):
    live[(node['id'],str(i))]=(out,path);locations[(node['id'],str(i))]='graph:'+str(node_index)+':'+str(i)
   seen.add(node['id'])
  expected=sorted([{'node':node,'output':output} for node,output in live],key=lambda x:(x['node'],int(x['output'])))
  # Every resource reported current must also be available in the supplied context.
  need(all(ref not in context['consumedOutputs'] for ref in live),'CURRENTNESS')
  need(graph['current']==expected,'CURRENT_OUTPUTS')
  return {'status':'SOURCE_GRAPH_VALID','current':expected,'signatureVerified':False,'ledgerAccepted':False}
 except Reject as e:return {'status':'REJECTED','firstPredicate':e.code,'signatureVerified':False,'ledgerAccepted':False}
 except (TypeError,ValueError,UnicodeError,RecursionError,KeyError,IndexError):return {'status':'REJECTED','firstPredicate':'INPUT_SHAPE','signatureVerified':False,'ledgerAccepted':False}
