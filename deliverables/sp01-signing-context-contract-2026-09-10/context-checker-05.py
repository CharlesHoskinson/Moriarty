"""Closed source-example checker. Assumed authorization/current registry, no signatures."""
import copy,hashlib,json,re
from pathlib import Path
from jsonschema import Draft202012Validator
HERE=Path(__file__).parent
SCHEMA=json.loads((HERE/'schema-02.json').read_text())
PROFILE=json.loads((HERE/'profile-05.json').read_text())
ACCOUNTS=tuple(p+'.'+a for p in PROFILE['parties'] for a in PROFILE['assets'])
PAYER=PROFILE['goalPayer']; RECIPIENT=PROFILE['goalRecipient']; GOAL=PROFILE['goalAsset']; GOAL_KEY=RECIPIENT+'.'+GOAL
CREDITOR=PROFILE['creditor']; DEBTOR=PROFILE['debtor']; DEBT_ASSET=PROFILE['debtAsset']
MAX=2**128-1
class Reject(Exception):
 def __init__(self,code):self.code=code;super().__init__(code)
def need(ok,code):
 if not ok:raise Reject(code)
def canonical(value):
 try:return json.dumps(value,sort_keys=True,separators=(',',':'),ensure_ascii=False,allow_nan=False).encode('utf8')
 except (UnicodeError,TypeError,ValueError,RecursionError):raise Reject('INPUT_ENCODING')
def digest(kind,value):return hashlib.sha256(('MORIARTY-SIGNING-CONTEXT-EXAMPLE/5/'+kind).encode()+b'\0'+canonical(value)).hexdigest()
def num(x):return int(x)
def accounts(profile):return tuple(p+'.'+a for p in profile['parties'] for a in profile['assets'])
def empty(profile=PROFILE):return {k:'0' for k in accounts(profile)}
def initial(g,profile=PROFILE):
 s={'balances':empty(profile),'gross':empty(profile),'fees':empty(profile),'receipts':empty(profile),'debt':{k:'0' for k in ('created','accrued','repaid','writtenOff','credited','outstanding')},'ordinary':g['ordinary'],'recovery':g['recovery'],'duties':[],'version':'v1','status':'Active'}
 ids=set()
 for f in g['funding']:
  need(f['id'] not in ids,'FUNDING_ID');ids.add(f['id']);k=f['owner']+'.'+f['asset'];s['balances'][k]=str(num(s['balances'][k])+num(f['amount']))
 check_generated(s)
 return s
def display_arguments(plan,profile=PROFILE):
 need(plan['action'] in profile['actionSignatures'],'ACTION_BODY')
 signature=profile['actionSignatures'][plan['action']];declared={a['name']:a for a in signature};seen=set();result=[]
 for i,a in enumerate(plan['arguments']):
  need(a['name'] not in seen,'ARGUMENT_DUPLICATE');seen.add(a['name'])
  need(a['name'] in declared and a['type']==declared[a['name']]['type'],'ARGUMENT_SIGNATURE')
  for binding in declared[a['name']]['bindings']:
   need(binding['operation']<len(plan['operations']),'ARGUMENT_BINDING');op=plan['operations'][binding['operation']];field=binding['field']
   need(field in op,'ARGUMENT_BINDING');actual=op[field]['literal'] if field=='amount' else op[field]
   need(a['value']==actual,'ARGUMENT_BINDING')
  result.append({'path':f'arguments.{i}.value','value':a['value'],'unit':a['type'],'role':'obligation-identity' if a['type']=='ObligationId' else 'quantity'})
 need(seen==set(declared),'ARGUMENT_SIGNATURE')
 return result
def derive(pre,plan,auth,profile=PROFILE):
 ACCOUNTS=accounts(profile);PAYER=profile['goalPayer'];RECIPIENT=profile['goalRecipient'];GOAL=profile['goalAsset'];GOAL_KEY=RECIPIENT+'.'+GOAL;CREDITOR=profile['creditor'];DEBTOR=profile['debtor'];DEBT_ASSET=profile['debtAsset']
 need(plan['action'] in profile['actionOperations'] and [x['op'] for x in plan['operations']]==profile['actionOperations'][plan['action']],'ACTION_BODY')
 s=copy.deepcopy(pre);ordinary=recovery=0
 for op in plan['operations']:
  kind=op['op'];need(kind in auth['allowedOps'],'OP_AUTHORITY');need(s['status']=='Active' or kind=='Migrate','CANCELLED_ACTION')
  # Fixed example schedule: each operation1, each literal occurrence1,
  # sidecar ceil(bytes/256) once. Not full38 financial price admission.
  charge=profile['operationPrice']+(profile['literalPrice'] if 'amount' in op else 0)
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
   if kind in ('Accrue','Repay'):need(num(d['created'])>0,'DEBT_NOT_CREATED')
   if kind=='Repay':need(sum(num(m['amount']['literal']) for m in plan['operations'] if m['op']=='Transfer' and m['from']==DEBTOR and m['to']==CREDITOR and m['asset']==DEBT_ASSET)==q and sum(m['op']=='Repay' for m in plan['operations'])==1,'REPAYMENT_CASH')
   if kind=='CreateDebt':need(num(d['created'])==0 and sum(num(m['amount']['literal']) for m in plan['operations'] if m['op']=='Transfer' and m['from']==CREDITOR and m['to']==DEBTOR and m['asset']==DEBT_ASSET)==q,'DEBT_FUNDING')
   d[field]=str(num(d[field])+q);out=num(d['created'])+num(d['accrued'])-num(d['repaid'])-num(d['writtenOff'])-num(d['credited']);need(out>=0,'DEBT_DOMAIN');d['outstanding']=str(out)
  elif kind=='Cancel':
   need(auth['cancelAllowed'],'CANCEL_AUTHORITY');need(not s['duties'],'BACKING_UNIQUENESS');net=num(s['receipts'][GOAL_KEY])-num(s['fees'][GOAL_KEY]);deficit=max(0,num(auth['netGoal'])-net)
   need(num(s['balances'][PAYER+'.'+GOAL])>=deficit and num(auth['grossCaps'][PAYER+'.'+GOAL])-num(s['gross'][PAYER+'.'+GOAL])>=deficit,'DUTY_BACKING')
   if deficit:s['duties']=[{'id':op['reservation'],'debtor':PAYER,'creditor':RECIPIENT,'asset':GOAL,'amount':str(deficit)}]
   s['status']='Cancelled'
  elif kind=='Migrate':need(op['newVersion']!=s['version'],'MIGRATION_VERSION');s['version']=op['newVersion']
  check_generated(s)
 ordinary+=(len(bytes.fromhex(plan['sidecarHex']))+profile['sidecarQuantumBytes']-1)//profile['sidecarQuantumBytes']
 need(num(pre['ordinary'])>=ordinary and num(pre['recovery'])>=recovery,'WORK_EXHAUSTED')
 s['ordinary']=str(num(pre['ordinary'])-ordinary);s['recovery']=str(num(pre['recovery'])-recovery)
 for k in ACCOUNTS:need(num(s['gross'][k])<=num(auth['grossCaps'][k]),'GROSS_CAP');need(num(s['fees'][k])<=num(auth['feeCaps'][k]),'FEE_CAP')
 need(num(s['debt']['created'])+num(s['debt']['accrued'])<=num(auth['debtAdditionCap']) and num(s['debt']['outstanding'])<=num(auth['debtOutstandingCap']),'DEBT_CAP')
 check_generated(s)
 return s,ordinary,recovery

def validate(doc,context,profile=PROFILE):
 """Context is trusted caller test-environment, never a field in signed document."""
 try:
  validate_profile(profile);schema=schema_for(profile);GOAL_KEY=profile['goalRecipient']+'.'+profile['goalAsset']
  need(type(context) is dict and set(context)=={'network','deployment','revoked','consumed','consumedOccurrences'},'CONTEXT_SHAPE')
  need(type(context['network']) is str and type(context['deployment']) is str and type(context['revoked']) is bool and type(context['consumed']) is set and type(context['consumedOccurrences']) is set,'CONTEXT_SHAPE')
  need(len(canonical(doc))<=profile['maxDocumentBytes'],'DOCUMENT_BOUND');need(not list(Draft202012Validator(schema).iter_errors(doc)),'SCHEMA')
  def numbers(v):
   if type(v) is dict:
    for x in v.values():numbers(x)
   elif type(v) is list:
    for x in v:numbers(x)
   elif type(v) is str and len(v)<=39 and re.fullmatch(r'[0-9]+',v):need(num(v)<=MAX,'NUMERIC_DOMAIN')
  numbers(doc)
  g=doc['genesis'];a=doc['authority'];need(digest('genesis',g)==doc['genesisHash']==a['genesisHash'],'GENESIS_HASH');need(digest('authority',a)==doc['authorityHash'],'AUTHORITY_HASH')
  need(a['profileHash']==digest('profile',{'profile':profile,'schema':schema}),'PROFILE_BINDING')
  need(a['network']==context['network']==profile['network'] and a['deployment']==context['deployment']==profile['deployment'],'TARGET');need(not context['revoked'],'REVOKED');need(num(a['notBefore'])<num(a['notAfter']),'VALIDITY_WINDOW')
  need(a['mode']!='OutcomeIntent' or not a['exactOccurrences'],'MODE_BINDING')
  pre=initial(g,profile);need(pre==doc['initial'],'GENESIS_FUNDING');need(digest('state',pre)==doc['initialHash'],'INITIAL_HASH')
  head=doc['initialHash'];spent=set(context['consumed']);ids=set();occurrences=set(context['consumedOccurrences']);terminal=False
  need(len({x['id'] for x in a['exactOccurrences']})==len(a['exactOccurrences']),'EXACT_OCCURRENCE_DUPLICATE')
  for step_index,step in enumerate(doc['steps']):
   need(not terminal,'TERMINAL_CONTINUATION')
   need(step['authorityHash']==doc['authorityHash'],'ORIGIN_BINDING');need(digest('plan',step['plan'])==step['planHash'],'PLAN_HASH');need(digest('state',step['post'])==step['stateHash'],'STATE_HASH');display_arguments(step['plan'],profile)
   consume_occurrence(a,step['occurrenceId'],head,step['planHash'],step['stateHash'],step['terminal'],occurrences,'linear:'+str(step_index),head)
   need(num(a['notBefore'])<=num(step['now'])<num(a['notAfter']),'ACTIVATION')
   need(step['predecessor']==head,'PREDECESSOR_BINDING');need(head not in spent and step['id'] not in ids,'CURRENTNESS')
   expected,o,r=derive(pre,step['plan'],a,profile);need(num(step['ordinaryCharge'])==o and num(step['recoveryCharge'])==r,'WORK_DERIVATION')
   need(expected==step['post'],'TRANSITION_FRAME')
   if step['terminal']:
    net=num(expected['receipts'][GOAL_KEY])-num(expected['fees'][GOAL_KEY]);goal=num(a['netGoal']);need(net==goal if a['netMode']=='Exact' else net>=goal,'NET_GOAL');need(not expected['duties'] and num(expected['debt']['outstanding'])==0,'RESIDUAL_DUTY')
   spent.add(head);ids.add(step['id']);head=step['stateHash'];pre=expected;terminal=step['terminal']
  return {'status':'SOURCE_CONTEXT_VALID','head':head,'steps':len(doc['steps']),'terminal':terminal,'consumedOccurrences':sorted(occurrences),'signatureVerified':False,'ledgerAccepted':False}
 except Reject as e:return {'status':'REJECTED','firstPredicate':e.code,'signatureVerified':False,'ledgerAccepted':False}
 except (TypeError,ValueError,UnicodeError,RecursionError,KeyError,IndexError):return {'status':'REJECTED','firstPredicate':'INPUT_SHAPE','signatureVerified':False,'ledgerAccepted':False}

def decode_canonical(raw):
 need(type(raw) is bytes and len(raw)<=PROFILE['maxDocumentBytes'],'DOCUMENT_BOUND')
 def unique(pairs):
  d={}
  for k,v in pairs:need(k not in d,'DUPLICATE_KEY');d[k]=v
  return d
 try:value=json.loads(raw.decode('utf8'),object_pairs_hook=unique)
 except (ValueError,UnicodeError):raise Reject('CANONICAL')
 need(canonical(value)==raw,'CANONICAL');return value

def validate_partition(parent,children,charge,profile=PROFILE):
 """Independent generic after-charge partition; full38 financial merge is absent."""
 try:
  validate_profile(profile);ACCOUNTS=accounts(profile)
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

def validate_profile(p):
 need(type(p) is dict and set(p)==set(PROFILE),'PROFILE_SCHEMA')
 def identity(x):return type(x) is str and re.fullmatch(r'[A-Za-z][A-Za-z0-9_-]{0,63}',x) is not None
 for key,limit in [('parties',16),('assets',8)]:
  xs=p[key];need(type(xs) is list and 1<=len(xs)<=limit and all(identity(x) for x in xs) and len(set(xs))==len(xs),'PROFILE_SCHEMA')
 for key in ['creditor','debtor','goalPayer','goalRecipient']:need(p[key] in p['parties'],'PROFILE_IDENTITY')
 for key in ['debtAsset','goalAsset']:need(p[key] in p['assets'],'PROFILE_IDENTITY')
 need(all(identity(p[k]) for k in ['network','deployment','obligation']),'PROFILE_IDENTITY')
 for key,ceiling in [('maxDocumentBytes',65536),('maxSteps',32),('maxOperations',16),('operationPrice',256),('literalPrice',256),('sidecarQuantumBytes',256)]:need(type(p[key]) is int and 1<=p[key]<=ceiling,'PROFILE_BOUND')
 need(p['grossIncludesFees'] is True,'PROFILE_ACCOUNTING')
 for key in ['status','signatureScope','financialRegistryStatus','expressionScope','schemaFile']:need(type(p[key]) is str and len(p[key])<=256,'PROFILE_SCHEMA')
 need(p['ordinaryOperations']==['Transfer','Fee','CreateDebt','Accrue','Repay','Migrate'] and p['recoveryOperations']==['Cancel'],'PROFILE_OPERATIONS')
 need(type(p['actionOperations']) is dict and type(p['actionSignatures']) is dict and set(p['actionOperations'])==set(p['actionSignatures']) and 1<=len(p['actionOperations'])<=16,'PROFILE_ACTION')
 for name,ops in p['actionOperations'].items():
  need(identity(name) and type(ops) is list and 1<=len(ops)<=p['maxOperations'] and all(op in p['ordinaryOperations']+p['recoveryOperations'] for op in ops),'PROFILE_ACTION')
  args=p['actionSignatures'][name];need(type(args) is list and len(args)<=8,'PROFILE_SIGNATURE');names=set();bound=set()
  for arg in args:
   need(type(arg) is dict and set(arg)=={'name','type','bindings'} and identity(arg['name']) and arg['name'] not in names and arg['type'] in ['UInt128','ObligationId'],'PROFILE_SIGNATURE');names.add(arg['name']);bindings=arg['bindings'];need(type(bindings) is list and 1<=len(bindings)<=16,'PROFILE_SIGNATURE')
   for b in bindings:
    need(type(b) is dict and set(b)=={'operation','field'} and type(b['operation']) is int and 0<=b['operation']<len(ops),'PROFILE_SIGNATURE');pair=(b['operation'],b['field']);need(b['field'] in ['amount','obligation'] and pair not in bound,'PROFILE_SIGNATURE');bound.add(pair)
    need(arg['type']==('UInt128' if b['field']=='amount' else 'ObligationId'),'PROFILE_SIGNATURE')
  expected={(i,'amount') for i,op in enumerate(ops) if op in ['Transfer','Fee','CreateDebt','Accrue','Repay']}|{(i,'obligation') for i,op in enumerate(ops) if op in ['CreateDebt','Accrue','Repay']}
  need(bound==expected,'PROFILE_SIGNATURE')

def schema_for(p):
 validate_profile(p);s=copy.deepcopy(SCHEMA);old_accounts=set(ACCOUNTS)
 def visit(x):
  if type(x) is dict:
   if x.get('enum')==['alice','bob','fees']:x['enum']=p['parties']
   if x.get('enum')==['A','B']:x['enum']=p['assets']
   if type(x.get('properties')) is dict and set(x['properties'])==old_accounts:
    value=next(iter(x['properties'].values()));x['properties']={k:copy.deepcopy(value) for k in accounts(p)};x['required']=list(accounts(p))
   for v in x.values():visit(v)
  elif type(x) is list:
   for v in x:visit(v)
 visit(s)
 authority=s['properties']['authority']['properties'];authority['netGoal']=authority.pop('netGoalB');s['properties']['authority']['required']=['netGoal' if k=='netGoalB' else k for k in s['properties']['authority']['required']];authority['network']={'enum':[p['network']]};authority['deployment']={'enum':[p['deployment']]};authority['principal']={'enum':[p['creditor']]}
 steps=s['properties']['steps'];steps['maxItems']=p['maxSteps'];plan=steps['items']['properties']['plan']['properties'];plan['operations']['maxItems']=p['maxOperations']
 variants=plan['operations']['items']['oneOf']
 for op in variants:
  if 'obligation' in op['properties']:op['properties']['obligation']={'enum':[p['obligation']]}
 args=plan['arguments']['items']['oneOf'];args[0]['properties']['name']={'type':'string','pattern':r'^[A-Za-z][A-Za-z0-9_-]{0,63}(?![\s\S])'};args[1]['properties']['name']=copy.deepcopy(args[0]['properties']['name'])
 for state in [s['properties']['initial'],steps['items']['properties']['post']]:
  duty=state['properties']['duties']['items']['properties'];duty['debtor']={'enum':[p['goalPayer']]};duty['creditor']={'enum':[p['goalRecipient']]};duty['asset']={'enum':[p['goalAsset']]}
 s['$id']='urn:moriarty:signing-context-example:5';s['properties']['format']={'enum':['signing-context-example-5']}
 authority.pop('exactPlans');s['properties']['authority']['required']=['exactOccurrences' if k=='exactPlans' else k for k in s['properties']['authority']['required']]
 h={'type':'string','pattern':r'^[0-9a-f]{64}(?![\s\S])'};name={'type':'string','pattern':r'^[A-Za-z][A-Za-z0-9_-]{0,63}(?![\s\S])'}
 fields={'id':name,'predecessorStateHash':h,'planHash':h,'postStateHash':h,'terminal':{'type':'boolean'},'predecessorLocation':{'type':'string','maxLength':64},'predecessorResourceHash':h}
 authority['exactOccurrences']={'type':'array','maxItems':32,'items':{'type':'object','properties':fields,'required':list(fields),'additionalProperties':False}}
 steps['items']['properties']['occurrenceId']={'type':'string','maxLength':64};steps['items']['required'].append('occurrenceId')
 return s

def check_generated(state):
 for field in ['balances','gross','fees','receipts','debt']:
  need(all(type(v) is str and re.fullmatch(r'0|[1-9][0-9]*',v) and 0<=int(v)<=MAX for v in state[field].values()),'NUMERIC_DOMAIN')
 for field in ['ordinary','recovery']:need(0<=int(state[field])<=MAX,'NUMERIC_DOMAIN')
 for d in state['duties']:need(0<=int(d['amount'])<=MAX,'NUMERIC_DOMAIN')


def consume_occurrence(authority,identifier,predecessor,plan_hash,post_hash,terminal,spent,location,resource_hash):
 if authority['mode']=='OutcomeIntent':need(identifier=='','OUTCOME_OCCURRENCE');return
 need(identifier not in spent,'EXACT_OCCURRENCE_CONSUMED')
 choices=[x for x in authority['exactOccurrences'] if x['id']==identifier]
 need(len(choices)==1,'EXACT_OCCURRENCE')
 expected={'id':identifier,'predecessorStateHash':predecessor,'planHash':plan_hash,'postStateHash':post_hash,'terminal':terminal,'predecessorLocation':location,'predecessorResourceHash':resource_hash}
 need(choices[0]==expected,'EXACT_OCCURRENCE_BINDING');spent.add(identifier)
