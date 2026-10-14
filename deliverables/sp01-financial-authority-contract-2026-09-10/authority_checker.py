"""Proposed full38 authority relation; never financial execution or authentication."""
import copy, hashlib, json, re
from pathlib import Path
D=Path(__file__).resolve().parent
SCHEMA=json.loads((D/'inputs/financial-schema.json').read_text()); S=SCHEMA['sigma']
MAPPING=json.loads((D/'operation-authority-map.json').read_text())
MAP={r['operation']:r for r in MAPPING['operations']}
RIGHTS={row['right'] for row in MAP.values()}|set('AssignBeneficiary AssignCreditor AssignDebtor AssignObligor BackingChange CapitalizeConsent CashDebit ClaimCarry ClaimCreate ClaimDischarge ClaimIncrease ClaimPay DebtCreate DebtTerms FeeDebit GenesisFunding GenesisWork SupplyChange TransformClaim'.split())
U128=(1<<128)-1
class Reject(Exception):
 def __init__(self,code):self.code=code

def need(p,c):
 if not p:raise Reject(c)
def canonical(x):
 try:
  def scan(v,depth=0):
   need(depth<=64,'BOUND')
   if type(v)==dict:
    need(all(type(k)==str for k in v),'SHAPE')
    return 1+sum(scan(k,depth+1)+scan(y,depth+1) for k,y in v.items())
   if type(v)==list:return 1+sum(scan(y,depth+1) for y in v)
   need(type(v) in (str,bool) or v is None,'SHAPE')
   if type(v)==str:need(len(v.encode('utf8'))<=65536,'BOUND')
   return 1
  need(scan(x)<=16384,'BOUND')
  out=json.dumps(x,sort_keys=True,separators=(',',':'),ensure_ascii=False,allow_nan=False).encode('utf8')
  need(len(out)<=262144,'BOUND');return out
 except (UnicodeError,TypeError,ValueError,RecursionError):raise Reject('SHAPE')
def digest(x):return hashlib.sha256(canonical(x)).hexdigest()
def domain_hash(kind,x):return digest(['MoriartyFinancialAuthority01',kind,x])
def exact(x,keys):need(type(x)==dict and set(x)==set(keys.split()),'SHAPE')
def text(x):need(type(x)==str and re.fullmatch(r'[A-Za-z][A-Za-z0-9_]{0,63}',x)!=None,'IDENTIFIER');return x
def h(x):need(type(x)==str and re.fullmatch('[0-9a-f]{64}',x)!=None,'HASH');return x
def uint(x,bits=128):
 need(type(x)==str and re.fullmatch(r'0|[1-9][0-9]*',x)!=None,'AMOUNT_TYPE')
 n=int(x);need(n<(1<<bits),'AMOUNT_RANGE');return n
def seq(x,cap=128):need(type(x)==list and len(x)<=cap,'BOUND');return x
def unique(xs,code='DUPLICATE'):need(len(xs)==len(set(xs)),code)
def name(x):return text(x['name'])
def typed(t,v):
 k=t[0]
 if k=='Record':
  fields=S['recordTypes'][t[1]];exact(v,' '.join(fields))
  for f in sorted(fields):typed(fields[f],v[f])
  if t[1]=='Hash':h(v['hex'])
  elif t[1].endswith('Id') and t[1]!='ReceiptId':
   if set(v)=={'name'}:text(v['name'])
 elif k=='Variant':
  exact(v,'tag value');need(type(v['tag'])==str and v['tag'] in S['variantTypes'][t[1]],'TYPE');typed(S['variantTypes'][t[1]][v['tag']],v['value'])
 elif k=='Enum':need(type(v)==str and v in S['enumTypes'][t[1]],'TYPE')
 elif k=='Option':
  seq(v,1)
  for q in v:typed(t[1],q)
 elif k=='Collection':
  seq(v,int(t[2]))
  for q in v:typed(t[1],q)
 elif k=='Bool':need(type(v)==bool,'TYPE')
 elif k=='Text':need(type(v)==str,'TYPE');canonical(v)
 elif k in ['UInt64','UInt128','UInt256']:uint(v,int(k[4:]))
 elif k in ['Amount','Shares','Price']:uint(v)
 elif k in ['SInt128','Rate','NetAmount','SignedAmount']:
  need(type(v)==str and re.fullmatch(r'0|-?[1-9][0-9]*',v)!=None,'TYPE');n=int(v)
  bounds={'NetAmount':(-U128,U128),'SignedAmount':(-(1<<255),(1<<255)-1)}.get(k,(-(1<<127),(1<<127)-1))
  need(bounds[0]<=n<=bounds[1],'AMOUNT_RANGE')
 else:raise Reject('UNSUPPORTED_TYPE')
def financial_bound(value):
 def nodes(x):return 1+sum(nodes(v) for v in x.values()) if type(x)==dict else 1+sum(nodes(v) for v in x) if type(x)==list else 1
 need(len(canonical(value))<=65536 and nodes(value)<=4096,'FINANCIAL_BOUND')
def argument_identity(t,value):
 # Fixed type-directed identity projection; numeric magnitudes remain signed in the full transition.
 k=t[0]
 if k=='Record':return {f:argument_identity(ft,value[f]) for f,ft in S['recordTypes'][t[1]].items()}
 if k=='Variant':return {'tag':value['tag'],'value':argument_identity(S['variantTypes'][t[1]][value['tag']],value['value'])}
 if k in ['Option','Collection']:return [argument_identity(t[1],v) for v in value]
 if k in ['UInt64','UInt128','UInt256','SInt128','Amount','Shares','Rate','Price','NetAmount','SignedAmount']:return ['Numeric',t]
 return value

def resource(q):
 t=S['variantTypes']['FinancialQuantity'][q['tag']]
 return {'kind':t[0] if t[0]=='Shares' else 'Asset','id':t[1]}
def quantity(q):
 v=q['value'];return uint(v['quanta'] if type(v)==dict else v)
def duty_resource(o):
 need(bool(o['resource'])!=bool(o['nominal']),'CLAIM_UNIT')
 if o['nominal']:return {'kind':'Nominal','id':name(o['nominal'][0])}
 return resource_id(o['resource'][0])
def resource_id(r):
 if r['kind']=='Asset':need(len(r['asset'])==1 and r['vault']==[],'RESOURCE');return {'kind':'Asset','id':name(r['asset'][0])}
 need(r['kind']=='Share' and len(r['vault'])==1 and r['asset']==[],'RESOURCE');return {'kind':'Shares','id':name(r['vault'][0])}
def identity(o,tag):
 ignored={'Duty':{'original','remaining','discharged','status','creationHash'},'Debt':{'principal','interest','status','lastAccrualEnd'},'Encumbrance':{'quantity','tombstone'},'Supply':{'issued','burned'},'VaultDefinition':{'shareSupply'}}.get(tag,set())
 return digest({k:v for k,v in o.items() if k not in ignored})
def lookup(state,collection,key):
 found=[x for x in state[collection] if x.get('id',{}).get('name')==key]
 need(len(found)==1,'REFERENCE');return found[0]
def authority_principal(profile,key):
 xs=[a for a in profile['authorities'] if name(a['id'])==key];need(len(xs)==1,'AUTHORITY_REFERENCE');return name(xs[0]['principal'])
def check_profile(p):
 exact(p,'version schemaHash mappingHash domain network deployment authorities policies paths')
 need(p['version']=='FinancialAuthorityProfile01' and p['schemaHash']==digest(S) and p['mappingHash']==digest(MAPPING),'PROFILE_BINDING')
 for f in ['domain','network','deployment']:text(p[f])
 for a in seq(p['authorities']):
  typed(['Record','AuthorityDefinition'],a)
  need(a['kind']=='OutcomeIntent' and a['exactPlanHash']==[],'LEGACY_EXACT_PLAN')
 for a in seq(p['policies']):typed(['Record','PolicyDefinition'],a)
 unique([name(a['id']) for a in p['authorities']]);unique([name(a['id']) for a in p['policies']])
 need(len(p['paths'])==38,'POLICY_PATH')
 for row in p['paths']:
  exact(row,'operation policies');need(row['operation'] in MAP,'OPERATION');seq(row['policies'],16);unique(row['policies'])
  for x in row['policies']:text(x);need(any(name(y['id'])==x for y in p['policies']),'POLICY_PATH')
 need({x['operation'] for x in p['paths']}==set(MAP),'POLICY_PATH')

def policy_path(t,p,op):
 ids=next(x['policies'] for x in p['paths'] if x['operation']==op)
 need(t['policy'] in ids,'POLICY_PATH')
 # Walk the actual typed operands/effects, not unrelated registry assets.
 touched=set()
 def visit(typ,value):
  k=typ[0]
  if k=='Record':
   if typ[1]=='AssetId':touched.add(('Asset',name(value)))
   if typ[1]=='VaultId':touched.add(('Shares',name(value)))
   for f,ft in S['recordTypes'][typ[1]].items():visit(ft,value[f])
  elif k=='Variant':visit(S['variantTypes'][typ[1]][value['tag']],value['value'])
  elif k in ['Option','Collection']:
   for x in value:visit(typ[1],x)
  elif k in ['Amount','NetAmount','SignedAmount']:touched.add(('Asset',typ[1]))
  elif k=='Shares':touched.add(('Shares',typ[1]))
  elif k=='Price':touched.update([('Asset',typ[1]),('Asset',typ[2])])
 for desc,effect in zip(t['operations'],t['effects']):
  visit(['Record',MAP[desc['operation']]['argumentType']],desc['arguments']);visit(['Record','OperationEffect'],effect)
 required=set()
 for kind,key in touched:
  collection='assets' if kind=='Asset' else 'vaults'
  objects=[x for state in [t['before'],t['after']] for x in state[collection] if name(x['id'])==key]
  need(bool(objects),'RESOURCE_REFERENCE')
  required.update(name(x['policy']) for x in objects)
 need(required<=set(ids),'POLICY_PATH')
 rows=[]
 for key in ids:
  policy=next(x for x in p['policies'] if name(x['id'])==key)
  need(not policy['revoked'] and uint(policy['notBefore'],64)<=uint(t['now'],64)<uint(policy['expiresAt'],64),'POLICY_LIFETIME')
  rows.append({'policy':key,'hash':digest(policy)})
 return domain_hash('policy-path',rows)

def required_authorities(t,p):
 check_profile(p);canonical(t)
 exact(t,'version schemaHash profileHash domain network deployment sourceHash program policy action actor now mode reason predecessor operations effects before after')
 need(t['version']=='FinancialAuthorityTransition01' and t['schemaHash']==digest(S) and t['profileHash']==digest(p),'PROFILE_BINDING')
 for f in ['domain','network','deployment']:need(t[f]==p[f],'DOMAIN')
 for f in ['sourceHash','predecessor']:h(t[f])
 for f in ['program','policy','action','actor']:text(t[f])
 uint(t['now'],64);need(t['mode'] in ['Ordinary','Recovery'],'GRANT_MODE')
 exact(t['reason'],'code statement evidence')
 need(t['reason']['code'] in ['Ordinary','Default','Liquidation','Migration','Revocation','Correction'],'REASON')
 need(type(t['reason']['statement'])==str and 0<len(t['reason']['statement'].encode())<=512,'REASON')
 for evidence in seq(t['reason']['evidence'],16):h(evidence)
 for state in [t['before'],t['after']]:
  typed(['Record','FinancialState'],state);financial_bound(state)
  for coll in ['parties','assets','nominals','debts','duties','encumbrances','instances','positions','requests','messages','events','rewards','vaults']:
   unique([name(x['id']) for x in state[coll]])
  for duty in state['duties']:
   need(uint(duty['original'])==uint(duty['remaining'])+uint(duty['discharged']),'CLAIM_ACCOUNTING')
   need((duty['status']=='Discharged')==(uint(duty['remaining'])==0),'CLAIM_STATUS')
   duty_resource(duty)
 seq(t['operations'],38);need(0<len(t['operations'])==len(t['effects']),'EFFECT_COUNT')
 parties={name(x['id']) for st in [t['before'],t['after']] for x in st['parties']}
 need(t['actor'] in parties,'PARTY')
 working=copy.deepcopy(t['before']);atoms=[]
 collmap={'Asset':'assets','Debt':'debts','Duty':'duties','Encumbrance':'encumbrances','EventClaim':'events','Instance':'instances','Message':'messages','Nominal':'nominals','Party':'parties','Position':'positions','Request':'requests','Reward':'rewards','Vault':'vaults'}
 for idx,(desc,effect) in enumerate(zip(t['operations'],t['effects'])):
  exact(desc,'operation arguments');op=desc['operation'];need(type(op)==str and op in MAP,'OPERATION')
  args=desc['arguments'];typed(['Record',MAP[op]['argumentType']],args);typed(['Record','OperationEffect'],effect)
  financial_bound(args);financial_bound(effect)
  need(effect['operation']==op and effect['argumentsHash']['hex']==digest(args),'EFFECT_BINDING')
  need(effect['origin']['operationIndex']==str(idx) and effect['origin']['sourceHash']['hex']==t['sourceHash'] and effect['origin']['action']==t['action'] and uint(effect['origin']['end'],64)>=uint(effect['origin']['start'],64),'ORIGIN_BINDING')
  path=policy_path(t,p,op)
  def add(right,party,counterparty,namespace,obj,unit,n,before,after):
   need(party in parties and (counterparty=='' or counterparty in parties),'PARTY')
   need(type(n)==int and 0<=n<=U128,'AMOUNT_RANGE')
   target={'right':right,'party':party,'counterparty':counterparty,'namespace':namespace,'object':obj,'unit':unit,'beforeIdentity':before,'afterIdentity':after,'policyPath':path}
   atoms.append({'operationIndex':str(idx),'operation':op,'target':target,'units':str(n)})
  selector=MAP[op]['principalSelector'].split('.')
  if selector[0]=='args':
   value=args
   for key in selector[1:]:value=value[key]
   principal=name(value)
  elif selector[0]=='context':principal=t['actor']
  elif selector[0]=='authority':principal=authority_principal(p,name(args['authority']))
  else:principal=name(lookup(working,selector[0],name(args[selector[1]]))[selector[2]])
  # Intent scope preserves all typed roles/IDs/rules while allowing bounded numerical variation.
  # Exact occurrences independently commit the full argument values.
  ah=digest(argument_identity(['Record',MAP[op]['argumentType']],args))
  add(MAP[op]['right'],principal,'','Operation',op,{'kind':'Count','id':''},1,ah,ah)
  for mov in effect['movements']:
   shape={k:mov[k] for k in ['source','recipient']};shape['resource']=resource(mov['quantity']);mh=digest(shape)
   source=name(mov['source']);recipient=name(mov['recipient']);n=quantity(mov['quantity'])
   need(n>0,'AMOUNT_RANGE')
   add('CashDebit',source,recipient,'Movement',source,resource(mov['quantity']),n,mh,mh)
   if mov['fee']:add('FeeDebit',source,recipient,'Movement',source,resource(mov['quantity']),n,mh,mh)
  before_operation=copy.deepcopy(working)
  seen=set()
  for change in effect['objects']:
   ns=change['namespace'];tag=change['after']['tag'];new=change['after']['value'];old=change['before'][0]['value'] if change['before'] else None
   expected_tag={'Asset':'AssetDefinition','Nominal':'NominalDefinition','Party':'PartyDefinition','Vault':'VaultDefinition','Counter':'AuthorityCounter'}.get(ns,ns)
   need(tag==expected_tag,'OBJECT_TYPE')
   key=(ns,change['id']);need(key not in seen,'DUPLICATE_OBJECT');seen.add(key)
   need(bool(change['before'])==bool(change['beforeHash']),'OBJECT_HASH')
   need(change['afterHash']['hex']==digest(new) and (old is None or change['beforeHash'][0]['hex']==digest(old)),'OBJECT_HASH')
   need(not change['before'] or change['before'][0]['tag']==tag,'OBJECT_TYPE')
   # Id-bearing records are reconciled here; other complete financial frame/delta checks belong to the bound financial judgment.
   if ns in collmap:
    collection=collmap[ns];expected={'Asset':'AssetDefinition','Nominal':'NominalDefinition','Party':'PartyDefinition','Vault':'VaultDefinition'}.get(ns,ns)
    need(tag==expected and name(new['id'])==change['id'],'OBJECT_TYPE')
    found=[o for o in working[collection] if name(o['id'])==change['id']]
    need((old is None and not found) or (len(found)==1 and found[0]==old),'PREIMAGE')
    if found:working[collection][working[collection].index(found[0])]=copy.deepcopy(new)
    else:working[collection].append(copy.deepcopy(new))
   b='' if old is None else identity(old,tag);a=identity(new,tag);obj=change['id']
   if tag=='Duty':
    unit=duty_resource(new);ob=name(new['obligor']);ben=name(new['beneficiary'])
    if old is None:
     need(new['discharged']=='0' and new['original']==new['remaining'],'CLAIM_CREATION')
     add('ClaimCreate',ob,ben,'Duty',obj,unit,uint(new['original']),b,a)
    else:
     need(new['creationHash']==old['creationHash'],'CLAIM_ORIGIN')
     inc=uint(new['original'])-uint(old['original']);paid=uint(new['discharged'])-uint(old['discharged'])
     need(inc>=0 and paid>=0,'CLAIM_HISTORY')
     if inc:add('ClaimIncrease',name(old['obligor']),name(old['beneficiary']),'Duty',obj,unit,inc,b,a)
     if paid:
      entries=[x for x in effect['claimMap'] if name(x['source'])==obj]
      if entries:
       need(sum(uint(x['quantity']) for x in entries)==paid,'CLAIM_MAP_QUANTITY')
       for entry in entries:add({'Pay':'ClaimPay','Carry':'ClaimCarry','AuthorizedDischarge':'ClaimDischarge'}[entry['disposition']],name(old['beneficiary']),name(old['obligor']),'Duty',obj,duty_resource(old),uint(entry['quantity']),b,a)
      else:
       right='ClaimPay' if op in ['DebtRepay','EventClaimSettle','RewardClaim','RequestClaim','RequestCancel','MessageReceive','MessageRefund'] else 'ClaimDischarge'
       add(right,name(old['beneficiary']),name(old['obligor']),'Duty',obj,duty_resource(old),paid,b,a)
     for field,right in [('beneficiary','AssignBeneficiary'),('obligor','AssignObligor')]:
      if old[field]!=new[field]:
       for party,other in [(name(old[field]),name(new[field])),(name(new[field]),name(old[field]))]:add(right,party,other,'Duty',obj,unit,uint(old['remaining']),b,a)
     structural=['resource','nominal','priority','conditionRule','backing','kind']
     if any(old[k]!=new[k] for k in structural):
      for party,other in [(name(old['obligor']),name(old['beneficiary'])),(name(old['beneficiary']),name(old['obligor']))]:add('TransformClaim',party,other,'Duty',obj,unit,uint(old['remaining']),b,a)
   elif tag=='Debt':
    unit={'kind':'Nominal','id':new['principal']['tag']}
    if old is None:add('DebtCreate',name(new['debtor']),name(new['creditor']),'Debt',obj,unit,quantity(new['principal']),b,a)
    else:
     need(new['liabilityAuthority']==old['liabilityAuthority'],'LIABILITY_ORIGIN')
     if any(old[k]!=new[k] for k in ['controller','accrualRule','allocationRule','conversionRule','negativeAccrual','priority']):
      for party,other in [(name(old['debtor']),name(old['creditor'])),(name(old['creditor']),name(old['debtor']))]:add('DebtTerms',party,other,'Debt',obj,unit,quantity(old['principal'])+quantity(old['interest']),b,a)
     need(new['principal']['tag']==old['principal']['tag'] and new['interest']['tag']==old['interest']['tag'],'DEBT_UNIT')
     delta=quantity(new['principal'])-quantity(old['principal'])
     if delta>0:
      add('Capitalize',name(old['debtor']),name(old['creditor']),'Debt',obj,unit,delta,b,a)
      add('CapitalizeConsent',name(old['creditor']),name(old['debtor']),'Debt',obj,unit,delta,b,a)
     for field,right in [('creditor','AssignCreditor'),('debtor','AssignDebtor')]:
      if old[field]!=new[field]:
       for party,other in [(name(old[field]),name(new[field])),(name(new[field]),name(old[field]))]:add(right,party,other,'Debt',obj,unit,quantity(old['principal'])+quantity(old['interest']),b,a)
   elif tag=='Encumbrance':
    partiesNeeded=[(name(new['owner']),name(new['beneficiary']))] if old is None else [(name(old['owner']),name(old['beneficiary'])),(name(old['beneficiary']),name(old['owner']))]
    for party,other in partiesNeeded:add('BackingChange',party,other,'Encumbrance',obj,resource_id(new['resource']),uint(new['quantity']) if old is None else max(uint(old['quantity']),uint(new['quantity'])),b,a)
   elif tag in ['Supply','VaultDefinition']:
    unit=resource_id(new['resource']) if tag=='Supply' else {'kind':'Shares','id':name(new['id'])}
    owner=next((name(x['issuer']) for x in t['after']['assets'] if name(x['id'])==unit['id']),None) if unit['kind']=='Asset' else name(lookup(t['after'],'vaults',unit['id'])['account'])
    if tag=='Supply':n=abs(uint(new['issued'])-(uint(old['issued']) if old else 0))+abs(uint(new['burned'])-(uint(old['burned']) if old else 0))
    else:n=abs(uint(new['shareSupply'])-(uint(old['shareSupply']) if old else 0))
    if n:need(owner is not None,'ISSUER');add('SupplyChange',owner,'',ns,obj,unit,n,b,a)
  for entry in effect['claimMap']:
   source=lookup(before_operation,'duties',name(entry['source']))
   need(uint(entry['quantity'])>0,'CLAIM_MAP_QUANTITY')
   if entry['disposition']=='Carry':
    need(len(entry['successor'])==1 and len(entry['destination'])==1,'CLAIM_MAP_SHAPE')
    successor=entry['successor'][0]
    need(successor==lookup(working,'duties',name(entry['destination'][0])),'CLAIM_MAP_BINDING')
    need(name(successor['id'])==name(entry['destination'][0]) and successor['remaining']==entry['quantity'] and successor['original']==entry['quantity'] and successor['discharged']=='0','CLAIM_MAP_QUANTITY')
    need(not any(name(x['id'])==name(successor['id']) for x in before_operation['duties']),'CLAIM_SUCCESSOR_FRESH')
    b=identity(source,'Duty');a=identity(successor,'Duty');unit=duty_resource(source);obj=name(source['id'])+'To'+name(successor['id']);n=uint(entry['quantity'])
    for field,right in [('beneficiary','AssignBeneficiary'),('obligor','AssignObligor')]:
     if source[field]!=successor[field]:
      for party,other in [(name(source[field]),name(successor[field])),(name(successor[field]),name(source[field]))]:add(right,party,other,'Duty',obj,unit,n,b,a)
    if any(source[k]!=successor[k] for k in ['resource','nominal','priority','conditionRule','backing','kind']):
     for party,other in [(name(source['obligor']),name(source['beneficiary'])),(name(source['beneficiary']),name(source['obligor']))]:add('TransformClaim',party,other,'Duty',obj,unit,n,b,a)
  if op=='Genesis':
   wi=domain_hash('work-funding',[name(args['instance']),t['program']]);add('GenesisWork',principal,'','Instance',name(args['instance']),{'kind':'Count','id':''},uint(args['initialWork'],64),wi,wi)
   for inp in args['funding']:
    ih=digest(inp);add('GenesisFunding',name(inp['owner']),'','Funding',inp['id']['hex'],resource(inp['quantity']),quantity(inp['quantity']),ih,ih)
  # Every changed Id record is reconstructed, including untouched duties and tombstones.
 for ns,collection in collmap.items():need(working[collection]==t['after'][collection],'FRAME')
 need(len(atoms)<=128,'AUTHORITY_BOUND');return atoms

GRANT_KEYS='id authority principal delegate domain network deployment profileHash programs policies actions operations mode notBefore expiresAt target lifetimeCap kind occurrences reasonHash'
TARGET_KEYS='right party counterparty namespace object unit beforeIdentity afterIdentity policyPath'
def check_grant(g,p):
 exact(g,GRANT_KEYS)
 for f in ['id','authority','principal','delegate','domain','network','deployment']:text(g[f])
 need(g['domain']==p['domain'] and g['network']==p['network'] and g['deployment']==p['deployment'] and g['profileHash']==digest(p),'GRANT_DOMAIN')
 for f in ['programs','policies','actions','operations']:
  seq(g[f],38);unique(g[f]);need(bool(g[f]),'GRANT_SCOPE')
  for x in g[f]:text(x)
 need(all(x in MAP for x in g['operations']),'OPERATION')
 parents=[a for a in p['authorities'] if name(a['id'])==g['authority']];need(len(parents)==1,'AUTHORITY_REFERENCE');parent=parents[0]
 need(g['principal']==name(parent['principal']) and g['delegate']==(name(parent['delegate'][0]) if parent['delegate'] else name(parent['principal'])),'AUTHORITY_DELEGATION')
 allowed=parent['recoveryOperations'] if g['mode']=='Recovery' else parent['operations']
 need(set(g['policies'])<={name(parent['policy'])},'PARENT_POLICY')
 need(set(g['operations'])<=set(allowed) and set(g['actions'])<=set(parent['actions']) and set(g['programs'])<={name(x) for x in parent['programs']},'PARENT_SCOPE')
 need(uint(parent['notBefore'],64)<=uint(g['notBefore'],64)<uint(g['expiresAt'],64)<=uint(parent['expiresAt'],64),'PARENT_LIFETIME')
 need(g['target']['counterparty']=='' or g['target']['counterparty'] in {name(x) for x in parent['recipients']},'PARENT_RECIPIENT')
 need(g['mode'] in ['Ordinary','Recovery'],'GRANT_MODE')
 need(uint(g['notBefore'],64)<uint(g['expiresAt'],64),'GRANT_LIFETIME');uint(g['lifetimeCap']);h(g['reasonHash'])
 exact(g['target'],TARGET_KEYS);z=g['target'];need(z['party']==g['principal'],'GRANT_PRINCIPAL')
 for f in ['right','party','namespace']:text(z[f])
 need(z['right'] in RIGHTS and z['namespace'] in S['enumTypes']['ObjectNamespace']+['Operation','Movement','Funding'],'TARGET_VOCABULARY')
 if z['counterparty']:text(z['counterparty'])
 need(type(z['object'])==str and len(z['object'])<=128,'GRANT_SCOPE')
 exact(z['unit'],'kind id');need(z['unit']['kind'] in ['Count','Asset','Shares','Nominal'],'GRANT_UNIT')
 if z['unit']['kind']=='Count':need(z['unit']['id']=='','GRANT_UNIT')
 else:text(z['unit']['id'])
 for f in ['beforeIdentity','afterIdentity']:
  if z[f]!='':h(z[f])
 h(z['policyPath']);need(g['kind'] in ['ExactPlan','OutcomeIntent'],'GRANT_KIND')
 seq(g['occurrences']);unique([o['id'] for o in g['occurrences']])
 for o in g['occurrences']:
  exact(o,'id predecessor transitionHash');text(o['id']);h(o['predecessor']);h(o['transitionHash'])
 need((g['kind']=='ExactPlan' and len(g['occurrences'])>0) or (g['kind']=='OutcomeIntent' and g['occurrences']==[]),'OCCURRENCE_SHAPE')

def parent_budget_state(state,profile):
 out={}
 for a in profile['authorities']:
  aid=name(a['id']);party=name(a['principal'])
  for b in a['budgets']:
   unit=resource_id(b['resource']);key=aid+'|'+digest(unit)
   need(key not in out,'BUDGET_DUPLICATE')
   rows=[r for r in state['counters'] if name(r['authority'])==aid and name(r['party'])==party and resource_id(r['resource'])==unit]
   need(len(rows)<=1,'COUNTER_DUPLICATE');row=rows[0] if rows else None
   if row:need(row['grossCap']==b['grossCap'] and row['feeCap']==b['feeCap'],'COUNTER_CAP_BINDING')
   out[key]={'gross':uint(row['gross']) if row else 0,'fee':uint(row['fees']) if row else 0,'grossCap':uint(b['grossCap']),'feeCap':uint(b['feeCap'])}
  for b in a['nominalBudgets']:
   unit={'kind':'Nominal','id':name(b['denomination'])};key=aid+'|'+digest(unit);need(key not in out,'BUDGET_DUPLICATE')
   rows=[r for r in state['nominalCounters'] if name(r['authority'])==aid and r['denomination']==b['denomination']];need(len(rows)<=1,'COUNTER_DUPLICATE');row=rows[0] if rows else None
   if row:need(row['creationCap']==b['creationCap'],'COUNTER_CAP_BINDING')
   out[key]={'created':uint(row['created']) if row else 0,'createdCap':uint(b['creationCap'])}
 for r in out.values():
  for k in ['gross','fee','created']:
   if k in r:need(r[k]<=r[k+'Cap'],'PARENT_CAP')
 return out

def validate(document,context):
 """Pure judgment. Context hash sets are externally established premises, not proof flags."""
 try:
  canonical(document);canonical(context)
  exact(document,'version profile grants seed steps')
  need(document['version']=='FinancialAuthorityHistory01','VERSION')
  p=document['profile'];check_profile(p);grants=seq(document['grants']);unique([g['id'] for g in grants])
  for g in grants:check_grant(g,p)
  exact(context,'version now authenticatedGrants authenticatedSeeds financialTransitions currentHeads spentHeads spentOccurrences revokedGrants revokedPolicies')
  need(context['version']=='FinancialAuthorityPremises01','VERSION');now=uint(context['now'],64)
  for f in ['authenticatedGrants','authenticatedSeeds','financialTransitions','currentHeads','spentHeads','revokedGrants','revokedPolicies']:
   seq(context[f]);unique(context[f]);
   for value in context[f]:h(value)
  seq(context['spentOccurrences']);unique(context['spentOccurrences'])
  for x in context['spentOccurrences']:h(x)
  seed=document['seed'];exact(seed,'originHash head state grantSetHash usage occurrences')
  h(seed['originHash']);h(seed['head']);typed(['Record','FinancialState'],seed['state'])
  need(seed['grantSetHash']==domain_hash('grants',grants),'GRANT_SET')
  need(domain_hash('seed',seed) in context['authenticatedSeeds'],'SEED_AUTHENTICATION')
  need(seed['head'] in context['currentHeads'] and seed['head'] not in context['spentHeads'],'SEED_CURRENTNESS')
  exact(seed['usage'],' '.join(g['id'] for g in grants))
  usage={g['id']:uint(seed['usage'][g['id']]) for g in grants}
  parent_usage=parent_budget_state(seed['state'],p)
  need(all(usage[g['id']]<=uint(g['lifetimeCap']) for g in grants),'LIFETIME_CAP')
  seq(seed['occurrences']);unique(seed['occurrences'])
  for x in seed['occurrences']:h(x)
  spent=set(seed['occurrences'])|set(context['spentOccurrences'])
  current=seed['head'];state=seed['state'];heads=set(context['spentHeads']);seq(document['steps'],64)
  for step in document['steps']:
   exact(step,'transition mode uses occurrences');t=step['transition'];rows=required_authorities(t,p)
   need(t['before']==state and t['predecessor']==current and current not in heads,'CURRENTNESS')
   need(t['now']==context['now'],'TIME_CONTEXT');th=domain_hash('transition',t)
   need(th in context['financialTransitions'],'FINANCIAL_JUDGMENT')
   need(step['mode'] in ['Ordinary','Recovery'] and step['mode']==t['mode'],'GRANT_MODE')
   for policy in p['policies']:need(digest(policy) not in context['revokedPolicies'],'POLICY_REVOKED')
   seq(step['uses']);need(len(step['uses'])==len(rows),'AUTHORITY_COVERAGE')
   selected=set(step['uses']);need(all(type(x)==str for x in step['uses']),'SHAPE')
   exact(step['occurrences'],' '.join(selected))
   byid={g['id']:g for g in grants}
   checked=set()
   for row,gid in zip(rows,step['uses']):
    need(gid in byid,'GRANT_MISSING');g=byid[gid];gh=domain_hash('grant',g)
    need(gh in context['authenticatedGrants'] and gh not in context['revokedGrants'],'GRANT_AUTHENTICATION')
    need(g['delegate']==t['actor'],'ACTOR')
    need(g['reasonHash']==domain_hash('reason',t['reason']),'REASON_BINDING')
    need(g['mode']==step['mode'] and t['program'] in g['programs'] and t['policy'] in g['policies'] and t['action'] in g['actions'] and row['operation'] in g['operations'],'GRANT_SCOPE')
    need(uint(g['notBefore'],64)<=now<uint(g['expiresAt'],64),'GRANT_LIFETIME')
    need(g['target']==row['target'],'TARGET_'+row['target']['right'])
    if gid not in checked:
     occurrence=step['occurrences'][gid]
     if g['kind']=='OutcomeIntent':need(occurrence=='','OCCURRENCE_SHAPE')
     else:
      key=domain_hash('occurrence',[gh,occurrence]);need(key not in spent,'OCCURRENCE_CONSUMED')
      matches=[o for o in g['occurrences'] if o['id']==occurrence and o['predecessor']==current and o['transitionHash']==th]
      need(len(matches)==1,'OCCURRENCE_BINDING');spent.add(key)
     checked.add(gid)
    usage[gid]+=uint(row['units']);need(usage[gid]<=uint(g['lifetimeCap']) and usage[gid]<=U128,'LIFETIME_CAP')
    metric={'CashDebit':'gross','FeeDebit':'fee','DebtCreate':'created','Capitalize':'created'}.get(row['target']['right'])
    if metric and row['target']['unit']['kind']!='Count':
     key=g['authority']+'|'+digest(row['target']['unit']);need(key in parent_usage and metric in parent_usage[key],'PARENT_BUDGET')
     parent_usage[key][metric]+=uint(row['units']);need(parent_usage[key][metric]<=parent_usage[key][metric+'Cap'] and parent_usage[key][metric]<=U128,'PARENT_CAP')
   need(parent_budget_state(t['after'],p)==parent_usage,'COUNTER_EFFECT_BINDING')
   heads.add(current);current=domain_hash('head',[current,th]);need(current not in heads and current not in context['spentHeads'],'OUTPUT_CURRENTNESS');state=t['after']
  need(current not in context['spentHeads'],'OUTPUT_CURRENTNESS')
  return {'status':'AUTHORITY_VALID_UNDER_PREMISES','head':current,'usage':{k:str(v) for k,v in usage.items()},'consumedOccurrences':sorted(spent),'consumedPredecessors':sorted(heads-set(context['spentHeads'])),'cryptographicAuthentication':False,'financialExecutionEstablished':False,'ledgerAcceptance':False}
 except Reject as e:return {'status':'REJECTED','code':e.code}
 except (KeyError,TypeError,ValueError,UnicodeError,OverflowError,RecursionError):return {'status':'REJECTED','code':'SHAPE'}

def decode_document(raw):
 """Strict public source encoding; repeated JSON keys and numeric JSON tokens reject."""
 try:
  need(type(raw)==bytes and len(raw)<=262144,'BOUND')
  def pairs(rows):
   out={}
   for key,value in rows:need(key not in out,'DUPLICATE_KEY');out[key]=value
   return out
  def number(_):raise Reject('NUMERIC_ENCODING')
  value=json.loads(raw.decode('utf8'),object_pairs_hook=pairs,parse_int=number,parse_float=number,parse_constant=number)
  canonical(value);return value
 except Reject:raise
 except (UnicodeError,ValueError,TypeError,RecursionError):raise Reject('ENCODING')

def signing_document(grant,profile):
 check_profile(profile);check_grant(grant,profile)
 # Complete bytes, not a lossy label/summary. Authentication signs this domain-separated payload.
 return canonical(['MoriartyFinancialAuthority01','grant',grant])


def signing_bundle(grant,profile):
 """The display carries the complete referenced parent limits as well as signed bytes."""
 raw=signing_document(grant,profile)
 return {'canonicalSigningDocument':raw.decode('utf8'),'signingHash':hashlib.sha256(raw).hexdigest(),'referencedProfile':copy.deepcopy(profile),'referencedProfileHash':digest(profile)}
