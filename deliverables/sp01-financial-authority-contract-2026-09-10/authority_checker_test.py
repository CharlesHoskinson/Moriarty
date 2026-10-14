"""Independent finite examples: trusted premise sets are explicit test assumptions."""
import copy,hashlib,json,unittest
from pathlib import Path
import authority_checker as C
D=Path(__file__).resolve().parent
SCHEMA=json.loads((D/'inputs/financial-schema.json').read_text())
S=SCHEMA['sigma']
def digest(x):return hashlib.sha256(json.dumps(x,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()).hexdigest()
def ident(x):return {'name':x}
def hx(x):return {'hex':x}
def default(t):
 k=t[0]
 if k=='Record':
  if t[1]=='Hash':return hx('a'*64)
  if t[1].endswith('Id') and t[1]!='ReceiptId':return ident('Example')
  return {n:default(v) for n,v in S['recordTypes'][t[1]].items()}
 if k=='Enum':return S['enumTypes'][t[1]][0]
 if k=='Variant':
  n=next(iter(S['variantTypes'][t[1]]));return {'tag':n,'value':default(S['variantTypes'][t[1]][n])}
 if k in ['Option','Collection']:return []
 if k=='Bool':return False
 if k=='Text':return 'Example'
 return '0'
def state():
 s=default(['Record','FinancialState'])
 s['parties']=[{'id':ident(p),'domain':ident('Local'),'identityCommitment':hx(digest(p))} for p in ['Alice','Bob','Pool','Issuer','Treasury']]
 s['assets']=[]
 for assetName in ['AssetA','AssetB','USD']:
  asset=default(['Record','AssetDefinition']);asset.update(id=ident(assetName),issuer=ident('Issuer'),policy=ident('Policy01'),quantum='1',network=ident('Local'),domain=ident('Domain01'));s['assets'].append(asset)
 s['nominals']=[{'id':ident('NominalUSD'),'quantum':'1','unit':'USD'}]
 return s
def nominal(n):return {'tag':'NominalUSD','value':{'quanta':str(n)}}
def debt(p=10,i=0):
 d=default(['Record','Debt']);d.update(id=ident('Debt01'),debtor=ident('Alice'),creditor=ident('Bob'),controller=ident('Alice'),principal=nominal(p),interest=nominal(i),claim=ident('Claim01'),status='Outstanding',liabilityAuthority=ident('Authority01'));return d
def duty(n=10):
 d=default(['Record','Duty']);d.update(id=ident('Claim01'),obligor=ident('Alice'),beneficiary=ident('Bob'),original=str(n),remaining=str(n),discharged='0',resource=[],nominal=[ident('NominalUSD')],kind='NominalDebt',status='Open');return d
def profile():
 a=default(['Record','AuthorityDefinition']);a.update(id=ident('Authority01'),principal=ident('Alice'),expiresAt='1000',kind='OutcomeIntent',policy=ident('Policy01'),delegate=[ident('Alice')],actions=['Action01'],programs=[ident('Program01')],operations=S['enumTypes']['FinancialOperation'],recoveryOperations=S['enumTypes']['FinancialOperation'],recipients=[ident(x) for x in ['Alice','Bob','Pool','Issuer','Treasury']],nominalBudgets=[{'denomination':ident('NominalUSD'),'creationCap':'100'}])
 a['budgets']=[{'resource':{'kind':'Asset','asset':[ident(asset)],'vault':[]},'grossCap':'100','feeCap':'10','netGoal':[],'eligibleIncomingOperations':[],'eligibleIncomingSources':[]} for asset in ['AssetA','AssetB','USD']]
 authorities=[]
 for party in ['Alice','Bob','Pool','Issuer','Treasury']:
  authority=copy.deepcopy(a);authority['id']=ident('Authority01' if party=='Alice' else 'Authority'+party);authority['principal']=ident(party);authorities.append(authority)
 policy=default(['Record','PolicyDefinition']);policy.update(id=ident('Policy01'),issuer=ident('Alice'),expiresAt='1000')
 return {'version':'FinancialAuthorityProfile01','schemaHash':digest(S),'mappingHash':digest(json.loads((D/'operation-authority-map.json').read_text())), 'domain':'Domain01','network':'Local','deployment':'Deployment01','authorities':authorities,'policies':[policy], 'paths':[{'operation':o,'policies':['Policy01']} for o in S['enumTypes']['FinancialOperation']]}
def transition(op='DebtCreate',before=None,after=None,args=None):
 b=state() if before is None else before
 a=copy.deepcopy(b) if after is None else after
 if op=='DebtCreate' and after is None:
  a['debts']=[debt()];a['duties']=[duty()];a['nominalCounters']=[{'authority':ident('Authority01'),'created':'10','creationCap':'100','denomination':ident('NominalUSD')}]
 if args is None:
  args=default(['Record',S['operations'][op]])
  if 'authority' in args:args['authority']=ident('AuthorityBob' if op=='DebtWriteOff' else 'Authority01')
  if op=='DebtCreate':args.update(debt=debt(),authority=ident('Authority01'),kind='AuthorizedNonCash');args['settlement'].update(asset=ident('AssetA'),denomination=ident('NominalUSD'),numerator='1',denominator='1')
  elif op in ['DebtWriteOff','DebtCapitalize','DebtAccrue','DebtRepay']:args['debt']=ident('Debt01')
  elif op=='Migrate':args.update(instance=ident('Instance01'),authority=ident('Authority01'))
 e=default(['Record','OperationEffect']);e.update(operation=op,argumentsHash=hx(digest(args)));e['origin'].update(operationIndex='0',sourceHash=hx('b'*64),action='Action01',start='0',end='1')
 for coll,tag in [('debts','Debt'),('duties','Duty')]:
  old={x['id']['name']:x for x in b[coll]}
  for obj in a[coll]:
   prev=old.get(obj['id']['name'])
   if prev!=obj:e['objects'].append({'id':obj['id']['name'],'namespace':tag,'before':[] if prev is None else [{'tag':tag,'value':prev}],'beforeHash':[] if prev is None else [hx(digest(prev))],'after':{'tag':tag,'value':obj},'afterHash':hx(digest(obj))})
 return {'version':'FinancialAuthorityTransition01','schemaHash':digest(S),'profileHash':digest(profile()),'domain':'Domain01','network':'Local','deployment':'Deployment01','sourceHash':'b'*64,'program':'Program01','policy':'Policy01','action':'Action01','actor':'Alice','now':'10','mode':'Ordinary','reason':{'code':'Ordinary','statement':'Explicit financial authority fixture','evidence':[]},'predecessor':'c'*64,'operations':[{'operation':op,'arguments':args}],'effects':[e],'before':b,'after':a}
class AuthorityGrammar(unittest.TestCase):
 def test_actual_typed_creation_derives_non_cash_authority(self):
  rows=C.required_authorities(transition(),profile())
  self.assertIn('ClaimCreate',[x['target']['right'] for x in rows])
  creation=next(x for x in rows if x['target']['right']=='ClaimCreate')
  self.assertEqual((creation['target']['party'],creation['target']['counterparty'],creation['units']),('Alice','Bob','10'))

def document(t=None,exact=False,p=None):
 t=transition() if t is None else t;p=profile() if p is None else p;t['profileHash']=digest(p);rows=C.required_authorities(t,p);grants=[]
 for i,row in enumerate(rows):
  grants.append({'id':'Grant'+str(i),'authority':'Authority01' if row['target']['party']=='Alice' else 'Authority'+row['target']['party'],'principal':row['target']['party'],'delegate':'Alice','domain':p['domain'],'network':p['network'],'deployment':p['deployment'],'profileHash':digest(p),'programs':['Program01'],'policies':['Policy01'],'actions':['Action01'],'operations':[row['operation']],'mode':'Ordinary','notBefore':'0','expiresAt':'100','target':copy.deepcopy(row['target']),'lifetimeCap':row['units'],'kind':'ExactPlan' if exact else 'OutcomeIntent','occurrences':[{'id':'Occurrence01','predecessor':t['predecessor'],'transitionHash':C.domain_hash('transition',t)}] if exact else [],'reasonHash':C.domain_hash('reason',t['reason'])})
 seed={'originHash':digest('Origin01'),'head':t['predecessor'],'state':t['before'],'grantSetHash':C.domain_hash('grants',grants),'usage':{g['id']:'0' for g in grants},'occurrences':[]}
 d={'version':'FinancialAuthorityHistory01','profile':p,'grants':grants,'seed':seed,'steps':[{'transition':t,'mode':'Ordinary','uses':[g['id'] for g in grants],'occurrences':{g['id']:'Occurrence01' if exact else '' for g in grants}}]}
 ctx={'version':'FinancialAuthorityPremises01','now':'10','authenticatedGrants':[C.domain_hash('grant',g) for g in grants],'authenticatedSeeds':[C.domain_hash('seed',seed)],'financialTransitions':[C.domain_hash('transition',t)],'currentHeads':[seed['head']],'spentHeads':[],'spentOccurrences':[],'revokedGrants':[],'revokedPolicies':[]}
 return d,ctx

def rebind(d,c):
 d['seed']['grantSetHash']=C.domain_hash('grants',d['grants']);c['authenticatedGrants']=[C.domain_hash('grant',g) for g in d['grants']];c['authenticatedSeeds']=[C.domain_hash('seed',d['seed'])];c['financialTransitions']=[C.domain_hash('transition',s['transition']) for s in d['steps']]

def modified_debt(op,change):
 b=state();b['debts']=[debt(10,2)];b['duties']=[duty(12)];b['nominalCounters']=[{'authority':ident('Authority01'),'created':'10','creationCap':'100','denomination':ident('NominalUSD')}];a=copy.deepcopy(b);change(a)
 if op=='DebtCapitalize':
  a['nominalCounters'][0]['created']='12'
 return transition(op,b,a)

class Histories(unittest.TestCase):
 def accepted(self,d,c):
  result=C.validate(d,c);self.assertEqual(result['status'],'AUTHORITY_VALID_UNDER_PREMISES',result);self.assertFalse(result['ledgerAcceptance']);return result
 def rejected(self,d,c,code):self.assertEqual(C.validate(d,c),{'status':'REJECTED','code':code})
 def test_creation_exact_and_outcome(self):
  for exact in [False,True]:
   d,c=document(exact=exact);r=self.accepted(d,c);self.assertEqual(set(r['usage'].values()),{'1','10'})
 def test_cash_grant_cannot_create_claim(self):
  d,c=document();i=next(i for i,g in enumerate(d['grants']) if g['target']['right']=='ClaimCreate');d['grants'][i]['target']['right']='CashDebit';rebind(d,c)
  self.rejected(d,c,'TARGET_ClaimCreate')
 def test_rehashed_creditor_substitution_rejects_bound_claim(self):
  d,c=document();t=d['steps'][0]['transition'];t['operations'][0]['arguments']['debt']['creditor']=ident('Treasury');t['after']['debts'][0]['creditor']=ident('Treasury');t['after']['duties'][0]['beneficiary']=ident('Treasury')
  replacement=transition(after=t['after'],args=t['operations'][0]['arguments']);d['steps'][0]['transition']=replacement
  rows=C.required_authorities(replacement,profile());d['grants'][0]['target']=rows[0]['target'];d['grants'][1]['target']=rows[1]['target'];rebind(d,c)
  self.rejected(d,c,'TARGET_ClaimCreate')
 def test_writeoff_needs_creditor_discharge_not_payment(self):
  def change(a):a['debts'][0]['principal']=nominal(8);a['duties'][0].update(remaining='10',discharged='2')
  t=modified_debt('DebtWriteOff',change);d,c=document(t);self.accepted(d,c)
  grant=next(g for g in d['grants'] if g['target']['right']=='ClaimDischarge');self.assertEqual(grant['principal'],'Bob');grant['target']['right']='ClaimPay';rebind(d,c);self.rejected(d,c,'TARGET_ClaimDischarge')
 def test_capitalization_requires_both_parties_and_consumes_capacity(self):
  def change(a):a['debts'][0].update(principal=nominal(12),interest=nominal(0))
  d,c=document(modified_debt('DebtCapitalize',change));r=self.accepted(d,c)
  grants=[g for g in d['grants'] if g['target']['namespace']=='Debt' and g['target']['right'] in ['Capitalize','CapitalizeConsent']];self.assertEqual({g['principal'] for g in grants},{'Alice','Bob'})
  grants[0]['lifetimeCap']='1';rebind(d,c);self.rejected(d,c,'LIFETIME_CAP')
 def test_direct_creditor_and_beneficiary_assignment_requires_both_roles(self):
  def change(a):a['debts'][0]['creditor']=ident('Treasury');a['duties'][0]['beneficiary']=ident('Treasury')
  d,c=document(modified_debt('Migrate',change));self.accepted(d,c)
  self.assertEqual({g['principal'] for g in d['grants'] if g['target']['right']=='AssignCreditor'},{'Bob','Treasury'})
  g=next(g for g in d['grants'] if g['target']['right']=='AssignBeneficiary' and g['principal']=='Treasury');c['authenticatedGrants'].remove(C.domain_hash('grant',g));self.rejected(d,c,'GRANT_AUTHENTICATION')
 def test_carry_new_id_cannot_evade_assignment(self):
  b=state();b['duties']=[duty()];a=copy.deepcopy(b);a['duties'][0].update(remaining='0',discharged='10',status='Discharged');successor=duty();successor.update(id=ident('Claim02'),beneficiary=ident('Treasury'));a['duties'].append(successor)
  t=transition('Migrate',b,a);entry=default(['Record','ClaimMapEntry']);entry.update(source=ident('Claim01'),destination=[ident('Claim02')],successor=[successor],quantity='10',disposition='Carry');t['effects'][0]['claimMap']=[entry]
  d,c=document(t);self.accepted(d,c);grants=[g for g in d['grants'] if g['target']['right']=='AssignBeneficiary'];self.assertEqual({g['principal'] for g in grants},{'Bob','Treasury'})
  grants[0]['target']['right']='ClaimCarry';rebind(d,c);self.rejected(d,c,'TARGET_AssignBeneficiary')
 def test_exact_replay_external_and_later_predecessor(self):
  d,c=document(exact=True);g=d['grants'][0];c['spentOccurrences']=[C.domain_hash('occurrence',[C.domain_hash('grant',g),'Occurrence01'])];self.rejected(d,c,'OCCURRENCE_CONSUMED')
  d,c=document(exact=True);d['seed']['head']='d'*64;d['steps'][0]['transition']['predecessor']='d'*64;c['currentHeads']=['d'*64];rebind(d,c);self.rejected(d,c,'OCCURRENCE_BINDING')
 def test_authenticated_seed_usage_cannot_be_reset(self):
  d,c=document();d['seed']['usage']['Grant0']='1';rebind(d,c);self.rejected(d,c,'LIFETIME_CAP')
  d['seed']['usage']['Grant0']='0';self.rejected(d,c,'SEED_AUTHENTICATION')
 def test_consumed_seed_and_final_outputs(self):
  d,c=document();c['spentHeads']=[d['seed']['head']];self.rejected(d,c,'SEED_CURRENTNESS')
  d,c=document();result=self.accepted(d,c);c['spentHeads']=[result['head']];self.rejected(d,c,'OUTPUT_CURRENTNESS')
  d,c=document();d['steps']=[];c['spentHeads']=[d['seed']['head']];self.rejected(d,c,'SEED_CURRENTNESS')
 def test_no_financial_judgment_from_signature(self):
  d,c=document();c['financialTransitions']=[];self.rejected(d,c,'FINANCIAL_JUDGMENT')
 def test_wrong_actor_window_network_mode_operation(self):
  for field,value,code in [('delegate','Bob','AUTHORITY_DELEGATION'),('expiresAt','10','GRANT_LIFETIME'),('mode','Recovery','GRANT_SCOPE'),('operations',['Transfer'],'GRANT_SCOPE')]:
   d,c=document();d['grants'][0][field]=value;rebind(d,c);self.rejected(d,c,code)
  d,c=document();d['steps'][0]['transition']['network']='Preview';rebind(d,c);self.rejected(d,c,'DOMAIN')
 def test_revoked_policy_and_grant(self):
  d,c=document();c['revokedGrants']=[C.domain_hash('grant',d['grants'][0])];self.rejected(d,c,'GRANT_AUTHENTICATION')
  d,c=document();c['revokedPolicies']=[digest(d['profile']['policies'][0])];self.rejected(d,c,'POLICY_REVOKED')
 def test_type_unknown_and_unicode_fail_closed(self):
  for mutation in [lambda d:d.update(allowed=True),lambda d:d['steps'][0]['transition'].update(actor='\ud800'),lambda d:d['steps'][0]['transition']['after']['duties'][0].update(remaining=True)]:
   d,c=document();mutation(d);result=C.validate(d,c);self.assertEqual(result['status'],'REJECTED')
 def test_dropped_duty_and_wrong_preimage_reject(self):
  d,c=document();d['steps'][0]['transition']['after']['duties']=[];rebind(d,c);self.rejected(d,c,'FRAME')
 def test_all38_mapping_exact_and_typed(self):
  self.assertEqual(set(C.MAP),set(S['enumTypes']['FinancialOperation']));self.assertEqual(len(C.MAP),38)
  for op,row in C.MAP.items():self.assertEqual(row['argumentType'],S['operations'][op])



def cash_transition(amount=10,prior=None,gross=0,fee=False,cap=100):
 b=state() if prior is None else copy.deepcopy(prior)
 if prior is None:b['balances']=[{'owner':ident('Alice'),'quantity':{'tag':'AssetA','value':'100'}},{'owner':ident('Bob'),'quantity':{'tag':'AssetA','value':'0'}}]
 a=copy.deepcopy(b);a['balances'][0]['quantity']['value']=str(int(b['balances'][0]['quantity']['value'])-amount);a['balances'][1]['quantity']['value']=str(int(b['balances'][1]['quantity']['value'])+amount)
 r={'kind':'Asset','asset':[ident('AssetA')],'vault':[]}
 a['counters']=[{'authority':ident('Authority01'),'party':ident('Alice'),'resource':r,'gross':str(gross+amount),'fees':str(amount if fee else 0),'eligibleIncoming':'0','grossCap':str(cap),'feeCap':'10','netGoal':[]}]
 op='Fee' if fee else 'Transfer';args=default(['Record',S['operations'][op]]);args.update(authority=ident('Authority01'),source=ident('Alice'),recipient=ident('Bob'),quantity={'tag':'AssetA','value':str(amount)},policy=ident('Policy01'))
 t=transition(op,b,a,args);e=t['effects'][0];receipt={'hash':hx(digest('Receipt01'))}
 # ReceiptId's actual schema uses its one Hash field; preserve that exact field name.
 receipt={next(iter(S['recordTypes']['ReceiptId'])):hx(digest('Receipt01'))}
 e['movements']=[{'source':ident('Alice'),'recipient':ident('Bob'),'quantity':args['quantity'],'fee':fee,'receipt':receipt}]
 e['balances']=[{'owner':ident(owner),'resource':r,'before':b['balances'][i]['quantity']['value'],'after':a['balances'][i]['quantity']['value']} for i,owner in enumerate(['Alice','Bob'])]
 e['counters']=[{'authority':ident('Authority01'),'party':ident('Alice'),'resource':r,'grossBefore':str(gross),'grossAfter':str(gross+amount),'feesBefore':'0','feesAfter':str(amount if fee else 0),'eligibleBefore':'0','eligibleAfter':'0'}]
 return t

class CompleteBudgetControls(unittest.TestCase):
 accepted=Histories.accepted
 rejected=Histories.rejected
 def test_variable_quantity_intent_cumulative_exact_cap(self):
  first=cash_transition(10);d,c=document(first);second=cash_transition(5,first['after'],10);second['predecessor']=C.domain_hash('head',[first['predecessor'],C.domain_hash('transition',first)])
  d['steps'].append({'transition':second,'mode':'Ordinary','uses':d['steps'][0]['uses'].copy(),'occurrences':d['steps'][0]['occurrences'].copy()})
  d['grants'][0]['lifetimeCap']='2';d['grants'][1]['lifetimeCap']='15';rebind(d,c);r=self.accepted(d,c);self.assertEqual(r['usage']['Grant1'],'15')
  d['grants'][1]['lifetimeCap']='14';rebind(d,c);self.rejected(d,c,'LIFETIME_CAP')
 def test_parent_cap_cannot_be_expanded_by_new_grant(self):
  p=profile();p['authorities'][0]['budgets'][0]['grossCap']='9';d,c=document(cash_transition(10,cap=9),p=p)
  self.rejected(d,c,'PARENT_CAP')
 def test_existing_financial_counter_charge_survives_new_grant(self):
  prior=cash_transition(90)['after'];prior['balances'][0]['quantity']['value']='100';t=cash_transition(11,prior,90);d,c=document(t);d['grants'][1]['lifetimeCap']='100';rebind(d,c);self.rejected(d,c,'PARENT_CAP')
 def test_fee_consumes_gross_and_separate_fee_budget(self):
  d,c=document(cash_transition(5,fee=True));r=self.accepted(d,c);self.assertEqual([g['target']['right'] for g in d['grants']],['FeeInstruction','CashDebit','FeeDebit']);self.assertEqual(list(r['usage'].values()),['1','5','5'])
 def test_migration_cannot_reset_persistent_nominal_counter(self):
  def change(a):a['nominalCounters'][0]['created']='0'
  d,c=document(modified_debt('Migrate',change));self.rejected(d,c,'COUNTER_EFFECT_BINDING')
 def test_reason_cannot_be_changed_after_signing(self):
  d,c=document();d['steps'][0]['transition']['reason']['code']='Liquidation';rebind(d,c);self.rejected(d,c,'REASON_BINDING')
 def test_complete_canonical_signing_and_strict_decoder(self):
  d,c=document();g=d['grants'][0];raw=C.signing_document(g,d['profile']);self.assertEqual(hashlib.sha256(raw).hexdigest(),C.domain_hash('grant',g));self.assertEqual(json.loads(raw)[2],g)
  for raw,code in [(b'{"a":"1","a":"2"}','DUPLICATE_KEY'),(b'{"a":1}','NUMERIC_ENCODING'),(b'{"a":"\\ud800"}','SHAPE')]:
   with self.assertRaises(C.Reject) as cm:C.decode_document(raw)
   self.assertEqual(cm.exception.code,code)
 def test_schema_grant_and_target_match_actual_records(self):
  schema=json.loads((D/'authority-schema.json').read_text());d,c=document()
  self.assertEqual(set(schema['$defs']['Grant']['required']),set(d['grants'][0]));self.assertEqual(set(schema['$defs']['Target']['required']),set(d['grants'][0]['target']))
  for definition in schema['$defs'].values():
   if definition.get('type')=='object' and 'properties' in definition:self.assertFalse(definition['additionalProperties'])



class AdditionalBindings(unittest.TestCase):
 rejected=Histories.rejected
 def test_changed_origin_index_rejects_actual_binding(self):
  d,c=document();d['steps'][0]['transition']['effects'][0]['origin']['operationIndex']='1';rebind(d,c);self.rejected(d,c,'ORIGIN_BINDING')
 def test_unknown_right_and_legacy_exact_hash_reject(self):
  d,c=document();d['grants'][0]['target']['right']='ArbitraryHostRule';rebind(d,c);self.rejected(d,c,'TARGET_VOCABULARY')
  d,c=document();d['profile']['authorities'][0]['kind']='ExactPlan';self.rejected(d,c,'LEGACY_EXACT_PLAN')
 def test_signing_bundle_includes_parent_budget_preimages(self):
  d,c=document();bundle=C.signing_bundle(d['grants'][0],d['profile']);self.assertEqual(bundle['referencedProfile'],d['profile']);self.assertEqual(digest(bundle['referencedProfile']),d['grants'][0]['profileHash'])
 def test_right_vocabulary_schema_equals_executable(self):
  schema=json.loads((D/'authority-schema.json').read_text());self.assertEqual(set(schema['$defs']['Target']['properties']['right']['enum']),C.RIGHTS)

if __name__=='__main__':unittest.main()
