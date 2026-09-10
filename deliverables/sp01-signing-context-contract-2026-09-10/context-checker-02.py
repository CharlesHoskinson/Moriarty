"""Closed source-example checker. Assumed authorization/current registry, no signatures."""
import copy,hashlib,json,re
from pathlib import Path
from jsonschema import Draft202012Validator
HERE=Path(__file__).parent
SCHEMA=json.loads((HERE/'schema-02.json').read_text())
PROFILE=json.loads((HERE/'profile-02.json').read_text())
ACCOUNTS=tuple(p+'.'+a for p in PROFILE['parties'] for a in PROFILE['assets'])
PAYER=PROFILE['goalPayer']; RECIPIENT=PROFILE['goalRecipient']; GOAL=PROFILE['goalAsset']; GOAL_KEY=RECIPIENT+'.'+GOAL
CREDITOR=PROFILE['creditor']; DEBTOR=PROFILE['debtor']; DEBT_ASSET=PROFILE['debtAsset']
MAX=2**128-1
class Reject(Exception):
 def __init__(self,code):self.code=code;super().__init__(code)
def need(ok,code):
 if not ok:raise Reject(code)
def canonical(value):return json.dumps(value,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode('utf8')
def digest(kind,value):return hashlib.sha256(('MORIARTY-SIGNING-CONTEXT-EXAMPLE/2/'+kind).encode()+b'\0'+canonical(value)).hexdigest()
def num(x):return int(x)
def empty():return {k:'0' for k in ACCOUNTS}
def initial(g):
 s={'balances':empty(),'gross':empty(),'fees':empty(),'receipts':empty(),'debt':{k:'0' for k in ('created','accrued','repaid','writtenOff','credited','outstanding')},'ordinary':g['ordinary'],'recovery':g['recovery'],'duties':[],'version':'v1','status':'Active'}
 ids=set()
 for f in g['funding']:
  need(f['id'] not in ids,'FUNDING_ID');ids.add(f['id']);k=f['owner']+'.'+f['asset'];s['balances'][k]=str(num(s['balances'][k])+num(f['amount']))
 return s
def display_arguments(plan):
 result=[];seen=set()
 for i,a in enumerate(plan['arguments']):
  need(a['name'] not in seen,'ARGUMENT_DUPLICATE');seen.add(a['name'])
  result.append({'path':f'arguments.{i}.value','value':a['value'],'unit':a['type'],'role':'obligation-identity' if a['name']=='debt_id' else 'quantity'})
 return result
def derive(pre,plan,auth):
 need(plan['action'] in PROFILE['actionOperations'] and [x['op'] for x in plan['operations']]==PROFILE['actionOperations'][plan['action']],'ACTION_BODY')
 s=copy.deepcopy(pre);ordinary=recovery=0
 for op in plan['operations']:
  kind=op['op'];need(kind in auth['allowedOps'],'OP_AUTHORITY');need(s['status']=='Active' or kind=='Migrate','CANCELLED_ACTION')
  # Fixed example schedule: each operation1, each literal occurrence1,
  # sidecar ceil(bytes/256) once. Not full38 financial price admission.
  charge=PROFILE['operationPrice']+(PROFILE['literalPrice'] if 'amount' in op else 0)
  if kind=='Cancel':recovery+=charge
  else:ordinary+=charge
  if kind in ('Transfer','Fee'):
   q=num(op['amount']['literal']);need(q>0 and op['from']!=op['to'],'MOVE_DOMAIN');a=op['from']+'.'+op['asset'];b=op['to']+'.'+op['asset']
   reserved=sum(num(d['amount']) for d in s['duties'] if d['debtor']==op['from'] and d['asset']==op['asset'])
   need(num(s['balances'][a])-reserved>=q,'BALANCE_BACKING')
   s['balances'][a]=str(num(s['balances'][a])-q);s['balances'][b]=str(num(s['balances'][b])+q)
   s['gross'][a]=str(num(s['gross'][a])+q)
   if kind=='Fee':s['fees'][a]=str(num(s['fees'][a])+q)
   else:s['receipts'][b]=str(num(s['receipts'][b])+q)
  elif kind in ('CreateDebt','Accrue','Repay'):
   q=num(op['amount']['literal']);need(q>0,'DEBT_DOMAIN');d=s['debt'];field={'CreateDebt':'created','Accrue':'accrued','Repay':'repaid'}[kind]
   # Repayment requires matching actual cash movement in this exact plan.
   if kind=='Repay':need(sum(num(m['amount']['literal']) for m in plan['operations'] if m['op']=='Transfer' and m['from']==DEBTOR and m['to']==CREDITOR and m['asset']==DEBT_ASSET)==q and sum(m['op']=='Repay' for m in plan['operations'])==1,'REPAYMENT_CASH')
   if kind=='CreateDebt':need(num(d['created'])==0 and sum(num(m['amount']['literal']) for m in plan['operations'] if m['op']=='Transfer' and m['from']==CREDITOR and m['to']==DEBTOR and m['asset']==DEBT_ASSET)==q,'DEBT_FUNDING')
   d[field]=str(num(d[field])+q);out=num(d['created'])+num(d['accrued'])-num(d['repaid'])-num(d['writtenOff'])-num(d['credited']);need(out>=0,'DEBT_DOMAIN');d['outstanding']=str(out)
  elif kind=='Cancel':
   need(auth['cancelAllowed'],'CANCEL_AUTHORITY');need(not s['duties'],'BACKING_UNIQUENESS');net=num(s['receipts'][GOAL_KEY])-num(s['fees'][GOAL_KEY]);deficit=max(0,num(auth['netGoalB'])-net)
   need(num(s['balances'][PAYER+'.'+GOAL])>=deficit and num(auth['grossCaps'][PAYER+'.'+GOAL])-num(s['gross'][PAYER+'.'+GOAL])>=deficit,'DUTY_BACKING')
   if deficit:s['duties']=[{'id':op['reservation'],'debtor':PAYER,'creditor':RECIPIENT,'asset':GOAL,'amount':str(deficit)}]
   s['status']='Cancelled'
  elif kind=='Migrate':need(op['newVersion']!=s['version'],'MIGRATION_VERSION');s['version']=op['newVersion']
 ordinary+=(len(bytes.fromhex(plan['sidecarHex']))+PROFILE['sidecarQuantumBytes']-1)//PROFILE['sidecarQuantumBytes']
 need(num(pre['ordinary'])>=ordinary and num(pre['recovery'])>=recovery,'WORK_EXHAUSTED')
 s['ordinary']=str(num(pre['ordinary'])-ordinary);s['recovery']=str(num(pre['recovery'])-recovery)
 for k in ACCOUNTS:need(num(s['gross'][k])<=num(auth['grossCaps'][k]),'GROSS_CAP');need(num(s['fees'][k])<=num(auth['feeCaps'][k]),'FEE_CAP')
 need(num(s['debt']['created'])+num(s['debt']['accrued'])<=num(auth['debtAdditionCap']) and num(s['debt']['outstanding'])<=num(auth['debtOutstandingCap']),'DEBT_CAP')
 return s,ordinary,recovery

def validate(doc,context):
 """Context is trusted caller test-environment, never a field in signed document."""
 try:
  need(type(context) is dict and set(context)=={'network','deployment','revoked','consumed'},'CONTEXT_SHAPE')
  need(context['network']=='local' and type(context['deployment']) is str and type(context['revoked']) is bool and type(context['consumed']) is set,'CONTEXT_SHAPE')
  need(len(canonical(doc))<=PROFILE['maxDocumentBytes'],'DOCUMENT_BOUND');need(not list(Draft202012Validator(SCHEMA).iter_errors(doc)),'SCHEMA')
  def numbers(v):
   if type(v) is dict:
    for x in v.values():numbers(x)
   elif type(v) is list:
    for x in v:numbers(x)
   elif type(v) is str and len(v)<=39 and re.fullmatch(r'[0-9]+',v):need(num(v)<=MAX,'NUMERIC_DOMAIN')
  numbers(doc)
  g=doc['genesis'];a=doc['authority'];need(digest('genesis',g)==doc['genesisHash']==a['genesisHash'],'GENESIS_HASH');need(digest('authority',a)==doc['authorityHash'],'AUTHORITY_HASH')
  need(a['profileHash']==digest('profile',{'profile':PROFILE,'schema':SCHEMA}),'PROFILE_BINDING')
  need(a['network']==context['network'] and a['deployment']==context['deployment'],'TARGET');need(not context['revoked'],'REVOKED');need(num(a['notBefore'])<num(a['notAfter']),'VALIDITY_WINDOW')
  need(a['mode']!='OutcomeIntent' or not a['exactPlans'],'MODE_BINDING')
  pre=initial(g);need(pre==doc['initial'],'GENESIS_FUNDING');need(digest('state',pre)==doc['initialHash'],'INITIAL_HASH')
  head=doc['initialHash'];spent=set(context['consumed']);ids=set()
  for step in doc['steps']:
   need(step['authorityHash']==doc['authorityHash'],'ORIGIN_BINDING');need(digest('plan',step['plan'])==step['planHash'],'PLAN_HASH');need(digest('state',step['post'])==step['stateHash'],'STATE_HASH');display_arguments(step['plan'])
   need(a['mode']!='ExactPlan' or step['planHash'] in a['exactPlans'],'EXACT_PLAN')
   need(num(a['notBefore'])<=num(step['now'])<num(a['notAfter']),'ACTIVATION')
   need(step['predecessor']==head,'PREDECESSOR_BINDING');need(head not in spent and step['id'] not in ids,'CURRENTNESS')
   expected,o,r=derive(pre,step['plan'],a);need(num(step['ordinaryCharge'])==o and num(step['recoveryCharge'])==r,'WORK_DERIVATION')
   need(expected==step['post'],'TRANSITION_FRAME')
   if step['terminal']:
    net=num(expected['receipts'][GOAL_KEY])-num(expected['fees'][GOAL_KEY]);goal=num(a['netGoalB']);need(net==goal if a['netMode']=='Exact' else net>=goal,'NET_GOAL');need(not expected['duties'] and num(expected['debt']['outstanding'])==0,'RESIDUAL_DUTY')
   spent.add(head);ids.add(step['id']);head=step['stateHash'];pre=expected
  return {'status':'SOURCE_CONTEXT_VALID','head':head,'steps':len(doc['steps']),'signatureVerified':False,'ledgerAccepted':False}
 except Reject as e:return {'status':'REJECTED','firstPredicate':e.code,'signatureVerified':False,'ledgerAccepted':False}

def decode_canonical(raw):
 need(type(raw) is bytes and len(raw)<=PROFILE['maxDocumentBytes'],'DOCUMENT_BOUND')
 def unique(pairs):
  d={}
  for k,v in pairs:need(k not in d,'DUPLICATE_KEY');d[k]=v
  return d
 try:value=json.loads(raw.decode('utf8'),object_pairs_hook=unique)
 except (ValueError,UnicodeError):raise Reject('CANONICAL')
 need(canonical(value)==raw,'CANONICAL');return value

def validate_partition(parent,children,charge):
 """Independent generic after-charge partition; full38 financial merge is absent."""
 try:
  def shape(x):
   need(type(x) is dict and set(x)=={'id','origin','ordinary','recovery','gross','duties'},'PARTITION_SCHEMA')
   need(type(x['id']) is str and type(x['origin']) is str and type(x['ordinary']) is int and type(x['recovery']) is int and 0<=x['ordinary']<=MAX and 0<=x['recovery']<=MAX,'PARTITION_SCHEMA')
   need(type(x['gross']) is dict and set(x['gross'])==set(ACCOUNTS) and all(type(q) is int and 0<=q<=MAX for q in x['gross'].values()),'PARTITION_SCHEMA')
   need(type(x['duties']) is list and len(x['duties'])<=8 and all(type(d) is str for d in x['duties']) and len(set(x['duties']))==len(x['duties']),'PARTITION_SCHEMA')
  shape(parent);need(type(children) is list and 1<=len(children)<=8 and type(charge) is int and 0<=charge<=MAX,'PARTITION_SCHEMA')
  for child in children:shape(child)
  need(len({x['id'] for x in children})==len(children) and parent['id'] not in {x['id'] for x in children},'PARTITION_ID')
  need(all(x['origin']==parent['origin'] for x in children),'PARTITION_ORIGIN')
  need(sum(x['ordinary'] for x in children)==parent['ordinary']-charge and sum(x['recovery'] for x in children)==parent['recovery'],'PARTITION_WORK')
  need(all(sum(x['gross'][k] for x in children)==parent['gross'][k] for k in ACCOUNTS),'PARTITION_GROSS')
  duties=[d for x in children for d in x['duties']];need(sorted(duties)==sorted(parent['duties']),'PARTITION_DUTY')
  return {'status':'SOURCE_PARTITION_VALID'}
 except Reject as e:return {'status':'REJECTED','firstPredicate':e.code}
