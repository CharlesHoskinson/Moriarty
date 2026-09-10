import copy,importlib.util,json,unittest
from pathlib import Path
O=Path(__file__).parent
spec=importlib.util.spec_from_file_location('checker',O/'context-checker-05.py');c=importlib.util.module_from_spec(spec);spec.loader.exec_module(c)
def fixture():
 g={'instance':'Swap','declared':['balances','debt','duties','status','version','work'],'funding':[{'id':'fundA','owner':'alice','asset':'A','amount':'100'},{'id':'fundB','owner':'alice','asset':'B','amount':'1000'}],'ordinary':'20','recovery':'4'}
 a={'mode':'OutcomeIntent','network':'local','deployment':'d1','principal':'alice','nonce':'n1','genesisHash':c.digest('genesis',g),'profileHash':c.digest('profile',{'profile':c.PROFILE,'schema':c.schema_for(c.PROFILE)}),'notBefore':'100','notAfter':'200','claims':['ContractInvariant','IntentRefinement','TransitionValidity','HistoryCompliance'],'allowedOps':['Transfer','Fee','CreateDebt','Accrue','Repay','Cancel','Migrate'],'exactOccurrences':[],'grossCaps':{k:'100' for k in c.ACCOUNTS},'feeCaps':{k:'5' for k in c.ACCOUNTS},'debtAdditionCap':'110','debtOutstandingCap':'110','netGoal':'20','netMode':'AtLeast','cancelAllowed':True}
 initial={'balances':dict(c.empty(),**{'alice.A':'100','alice.B':'1000'}),'gross':c.empty(),'fees':c.empty(),'receipts':c.empty(),'debt':{k:'0' for k in ('created','accrued','repaid','writtenOff','credited','outstanding')},'ordinary':'20','recovery':'4','duties':[],'version':'v1','status':'Active'}
 post=copy.deepcopy(initial);post['balances']['alice.B']='980';post['balances']['bob.B']='18';post['balances']['fees.B']='2';post['gross']['alice.B']='20';post['gross']['bob.B']='2';post['fees']['bob.B']='2';post['receipts']['bob.B']='20';post['ordinary']='16'
 plan={'action':'fill','arguments':[{'name':'received','type':'UInt128','value':'20'},{'name':'fee','type':'UInt128','value':'2'}],'operations':[{'op':'Transfer','from':'alice','to':'bob','asset':'B','amount':{'literal':'20'}},{'op':'Fee','from':'bob','to':'fees','asset':'B','amount':{'literal':'2'}}],'sidecarHex':''}
 step={'occurrenceId':'','id':'step1','predecessor':c.digest('state',initial),'authorityHash':c.digest('authority',a),'plan':plan,'planHash':c.digest('plan',plan),'now':'150','ordinaryCharge':'4','recoveryCharge':'0','post':post,'stateHash':c.digest('state',post),'terminal':False}
 return {'format':'signing-context-example-5','genesis':g,'genesisHash':c.digest('genesis',g),'authority':a,'authorityHash':c.digest('authority',a),'initial':initial,'initialHash':c.digest('state',initial),'steps':[step]}
def rehash(d):
 d['genesisHash']=c.digest('genesis',d['genesis']);d['authority']['genesisHash']=d['genesisHash'];d['authorityHash']=c.digest('authority',d['authority']);d['initialHash']=c.digest('state',d['initial']);head=d['initialHash']
 for s in d['steps']:
  s.setdefault('occurrenceId','')
  s['authorityHash']=d['authorityHash'];s['predecessor']=head;s['planHash']=c.digest('plan',s['plan']);s['stateHash']=c.digest('state',s['post']);head=s['stateHash']
 return d
def context():return {'network':'local','deployment':'d1','revoked':False,'consumed':set(),'consumedOccurrences':set()}
def cancel(d):
 pre=d['steps'][-1]['post'];post=copy.deepcopy(pre);post['status']='Cancelled';post['recovery']='3';post['duties']=[{'id':'reserve1','debtor':'alice','creditor':'bob','asset':'B','amount':'2'}]
 d['steps'].append({'id':'cancel','predecessor':'0'*64,'authorityHash':'0'*64,'plan':{'action':'cancel','arguments':[],'operations':[{'op':'Cancel','reservation':'reserve1'}],'sidecarHex':''},'planHash':'0'*64,'now':'151','ordinaryCharge':'0','recoveryCharge':'1','post':post,'stateHash':'0'*64,'terminal':False});return rehash(d)
class Check(unittest.TestCase):
 def expect(self,d,code,ctx=None):self.assertEqual(c.validate(d,ctx or context()).get('firstPredicate'),code)
 def test_positive_and_removed_mutation(self):
  d=fixture();self.assertEqual(c.validate(d,context())['status'],'SOURCE_CONTEXT_VALID');d['steps'][0]['ordinaryCharge']='0';self.expect(d,'WORK_DERIVATION');d['steps'][0]['ordinaryCharge']='4';self.assertEqual(c.validate(d,context())['status'],'SOURCE_CONTEXT_VALID')
 def test_reordered_named_binding(self):
  d=fixture();d['steps'][0]['plan']['arguments'].reverse();rehash(d);self.assertEqual(c.validate(d,context())['status'],'SOURCE_CONTEXT_VALID');rows=c.display_arguments(d['steps'][0]['plan']);self.assertEqual(rows[1],{'path':'arguments.1.value','value':'20','unit':'UInt128','role':'quantity'})
 def test_wrong_field_and_type(self):
  for field,value in [('name','qty'),('type','ObligationId')]:
   d=fixture();d['steps'][0]['plan']['arguments'][0][field]=value;self.assertIn(c.validate(rehash(d),context())['firstPredicate'],['SCHEMA','ARGUMENT_SIGNATURE'])
 def test_label_not_accepted(self):
  d=fixture();d['intendedFailureStage']='CURRENTNESS';self.expect(d,'SCHEMA');d.pop('intendedFailureStage');self.assertEqual(c.validate(d,context())['status'],'SOURCE_CONTEXT_VALID')
 def test_funding_actual_relation(self):
  d=fixture();d['genesis']['funding'].pop();self.expect(rehash(d),'GENESIS_FUNDING')
 def test_duplicate_funding(self):
  d=fixture();d['genesis']['funding'].append(copy.deepcopy(d['genesis']['funding'][0]));self.expect(rehash(d),'FUNDING_ID')
 def test_changed_effect_and_full_frame(self):
  d=fixture();d['steps'][0]['plan']['operations'][0]['amount']['literal']='19';d['steps'][0]['plan']['arguments'][0]['value']='19';self.expect(rehash(d),'TRANSITION_FRAME')
  d=fixture();d['steps'][0]['post']['balances']['alice.A']='99';self.expect(rehash(d),'TRANSITION_FRAME')
 def test_caps_derived_even_rehashed(self):
  d=fixture();d['authority']['grossCaps']['alice.B']='19';self.expect(rehash(d),'GROSS_CAP')
  d=fixture();d['authority']['feeCaps']['bob.B']='1';self.expect(rehash(d),'FEE_CAP')
 def test_cancel_backing_positive_and_authority(self):
  d=cancel(fixture());self.assertEqual(c.validate(d,context())['status'],'SOURCE_CONTEXT_VALID');d['authority']['grossCaps']['alice.B']='21';self.expect(rehash(d),'DUTY_BACKING')
 def test_currentness_before_bad_post(self):
  d=cancel(fixture());ctx=context();ctx['consumed'].add(d['steps'][0]['stateHash']);d['steps'][1]['post']['duties'][0]['amount']='3';rehash(d);self.expect(d,'CURRENTNESS',ctx)
 def test_net_goal(self):
  d=fixture();d['steps'][0]['terminal']=True;self.expect(d,'NET_GOAL');d['authority']['netGoal']='18';rehash(d);self.assertEqual(c.validate(d,context())['status'],'SOURCE_CONTEXT_VALID')
 def test_exact_plan_vs_outcome(self):
  d=fixture();a=d['authority'];a['mode']='ExactPlan';a['exactOccurrences']=[{'id':'once','predecessorStateHash':d['initialHash'],'planHash':d['steps'][0]['planHash'],'postStateHash':d['steps'][0]['stateHash'],'terminal':False,'predecessorLocation':'linear:0','predecessorResourceHash':d['initialHash']}];d['steps'][0]['occurrenceId']='once';rehash(d);self.assertEqual(c.validate(d,context())['status'],'SOURCE_CONTEXT_VALID');d['steps'][0]['plan']['arguments'].reverse();self.expect(rehash(d),'EXACT_OCCURRENCE_BINDING')
 def test_expired_and_revoked(self):
  d=fixture();d['steps'][0]['now']='200';self.expect(d,'ACTIVATION');d=fixture();ctx=context();ctx['revoked']=True;self.expect(d,'REVOKED',ctx)
 def test_migration_preserves_context(self):
  d=cancel(fixture());s=copy.deepcopy(d['steps'][-1]);s.update(id='migrate',ordinaryCharge='1',recoveryCharge='0');s['plan']={'action':'migrate','arguments':[],'operations':[{'op':'Migrate','newVersion':'v2'}],'sidecarHex':''};s['post']['version']='v2';s['post']['ordinary']='15';d['steps'].append(s);rehash(d);self.assertEqual(c.validate(d,context())['status'],'SOURCE_CONTEXT_VALID');s['post']['gross']['alice.B']='0';self.expect(rehash(d),'TRANSITION_FRAME')
 def test_sidecar_derived_not_reported(self):
  d=fixture();d['steps'][0]['plan']['sidecarHex']='aa';self.expect(rehash(d),'WORK_DERIVATION')
class MoreChecks(unittest.TestCase):
 def test_canonical_and_duplicate(self):
  d=fixture();self.assertEqual(c.decode_canonical(c.canonical(d)),d)
  for raw in [b'{"x":"1","x":"2"}',b'{ "x":"1"}']:
   with self.assertRaises(c.Reject):c.decode_canonical(raw)
 def test_action_not_arbitrary(self):
  d=fixture();d['steps'][0]['plan']['action']='unknown';self.assertEqual(c.validate(rehash(d),context())['firstPredicate'],'ACTION_BODY')
 def test_partition_independent_expected(self):
  z={k:0 for k in c.ACCOUNTS};p={'id':'p','origin':'origin1','ordinary':12,'recovery':4,'gross':dict(z,**{'alice.A':40}),'duties':['duty50']};l={'id':'l','origin':'origin1','ordinary':6,'recovery':1,'gross':dict(z,**{'alice.A':25}),'duties':['duty50']};r={'id':'r','origin':'origin1','ordinary':4,'recovery':3,'gross':dict(z,**{'alice.A':15}),'duties':[]}
  self.assertEqual(c.validate_partition(p,[l,r],2)['status'],'SOURCE_PARTITION_VALID')
  for field,value,code in [('ordinary',12,'PARTITION_WORK'),('duties',['duty50'],'PARTITION_DUTY'),('id','l','PARTITION_ID'),('origin','newOrigin','PARTITION_ORIGIN')]:
   bad=copy.deepcopy(r);bad[field]=value;self.assertEqual(c.validate_partition(p,[l,bad],2)['firstPredicate'],code)
 def test_funded_debt_history_and_mutation(self):
  d=fixture();s=d['steps'][0];s['plan']={'action':'loan','arguments':[{'name':'loan','type':'ObligationId','value':'Loan01'},{'name':'principal','type':'UInt128','value':'100'}],'operations':[{'op':'Transfer','from':'alice','to':'bob','asset':'A','amount':{'literal':'100'}},{'op':'CreateDebt','obligation':'Loan01','amount':{'literal':'100'}}],'sidecarHex':''};post=copy.deepcopy(d['initial']);post['balances']['alice.A']='0';post['balances']['bob.A']='100';post['gross']['alice.A']='100';post['receipts']['bob.A']='100';post['debt']['created']='100';post['debt']['outstanding']='100';post['ordinary']='16';s['post']=post;rehash(d)
  self.assertEqual(c.validate(d,context())['status'],'SOURCE_CONTEXT_VALID');s['post']['debt']['outstanding']='1';self.assertEqual(c.validate(rehash(d),context())['firstPredicate'],'TRANSITION_FRAME')

class CumulativeChecks(unittest.TestCase):
 def test_second_fill_crosses_cap_with_consistent_state(self):
  d=fixture();s=copy.deepcopy(d['steps'][0]);s['id']='step2';p=s['post'];p['balances']['alice.B']='960';p['balances']['bob.B']='36';p['balances']['fees.B']='4';p['gross']['alice.B']='40';p['gross']['bob.B']='4';p['fees']['bob.B']='4';p['receipts']['bob.B']='40';p['ordinary']='12';d['steps'].append(s);rehash(d)
  self.assertEqual(c.validate(d,context())['status'],'SOURCE_CONTEXT_VALID');d['authority']['grossCaps']['alice.B']='39';self.assertEqual(c.validate(rehash(d),context())['firstPredicate'],'GROSS_CAP')
 def test_recovery_current_predecessor_cannot_reset(self):
  d=cancel(fixture());ctx=context();ctx['consumed'].add(d['initialHash']);self.assertEqual(c.validate(d,ctx)['firstPredicate'],'CURRENTNESS')
 def test_late_domain_newline_not_hash(self):
  d=fixture();d['authority']['grossCaps']['alice.B']='100\n';self.assertEqual(c.validate(d,context())['firstPredicate'],'SCHEMA')

class RepairChecks(unittest.TestCase):
 def test_disconnected_display_rehashed(self):
  d=fixture();d['steps'][0]['plan']['arguments'][0]['value']='99';self.assertEqual(c.validate(rehash(d),context())['firstPredicate'],'ARGUMENT_BINDING')
 def test_malformed_plain_inputs_are_closed(self):
  for mutate in [lambda d:d['authority'].update(nonce='\ud800'),lambda d:d.update(extra=object()),lambda d:d.update(extra=float('nan')),lambda d:d.update(genesis=7),lambda d:d['authority']['grossCaps'].update({'alice.A':str(c.MAX+1)})]:
   d=fixture();mutate(d);r=c.validate(d,context());self.assertEqual(r['status'],'REJECTED');self.assertIn(r['firstPredicate'],['INPUT_ENCODING','SCHEMA','NUMERIC_DOMAIN'])
 def test_funding_sum_overflow_and_exact_boundary(self):
  d=fixture();g=d['genesis'];g['funding']=[{'id':'one','owner':'alice','asset':'A','amount':str(c.MAX)},{'id':'two','owner':'alice','asset':'A','amount':'1'}];self.assertEqual(c.validate(rehash(d),context())['firstPredicate'],'NUMERIC_DOMAIN')
  g['funding'].pop();d['initial']['balances']['alice.A']=str(c.MAX);d['initial']['balances']['alice.B']='0';d['steps']=[];self.assertEqual(c.validate(rehash(d),context())['status'],'SOURCE_CONTEXT_VALID')
 def test_generated_cumulative_overflow(self):
  d=fixture();s=d['initial'];s['gross']['alice.B']=str(c.MAX)
  with self.assertRaises(c.Reject) as caught:c.derive(s,d['steps'][0]['plan'],d['authority'])
  self.assertEqual(caught.exception.code,'NUMERIC_DOMAIN')
 def test_profile_preview_more_parties_and_assets(self):
  profile=copy.deepcopy(c.PROFILE);profile.update(network='preview',deployment='previewExample',parties=['Ada','Ben','Vault','Carol'],assets=['USD','EUR','GOLD'],creditor='Ada',debtor='Ben',goalPayer='Ada',goalRecipient='Ben',goalAsset='EUR',debtAsset='USD',obligation='Debt77')
  # Rewrite an abstract record, never connect to Preview or use a private identity.
  d=fixture();mapping={'alice':'Ada','bob':'Ben','fees':'Vault','A':'USD','B':'EUR','Loan01':'Debt77'}
  def visit(v):
   if type(v) is dict:
    return {('.'.join(mapping.get(part,part) for part in k.split('.')) if '.' in k else k):visit(x) for k,x in v.items()}
   if type(v) is list:return [visit(x) for x in v]
   return mapping.get(v,v) if type(v) is str else v
  d=visit(d);a=d['authority'];a['network']='preview';a['deployment']='previewExample';a['principal']='Ada';a['profileHash']=c.digest('profile',{'profile':profile,'schema':c.schema_for(profile)})
  for row in [d['initial']]+[s['post'] for s in d['steps']]:
   for field in ['balances','gross','fees','receipts']:
    for key in c.accounts(profile):row[field].setdefault(key,'0')
  for field in ['grossCaps','feeCaps']:
   for key in c.accounts(profile):a[field].setdefault(key,'0')
  rehash(d);ctx={'network':'preview','deployment':'previewExample','revoked':False,'consumed':set(),'consumedOccurrences':set()};self.assertEqual(c.validate(d,ctx,profile)['status'],'SOURCE_CONTEXT_VALID');ctx['network']='local';self.assertEqual(c.validate(d,ctx,profile)['firstPredicate'],'TARGET')
 def test_argument_identity_uses_type_not_name(self):
  profile=copy.deepcopy(c.PROFILE);profile['actionSignatures']['loan'][0]['name']='obligation'
  plan={'action':'loan','arguments':[{'name':'principal','type':'UInt128','value':'10'},{'name':'obligation','type':'ObligationId','value':'Loan01'}],'operations':[{'op':'Transfer','from':'alice','to':'bob','asset':'A','amount':{'literal':'10'}},{'op':'CreateDebt','obligation':'Loan01','amount':{'literal':'10'}}],'sidecarHex':''};c.validate_profile(profile);rows=c.display_arguments(plan,profile);self.assertEqual(rows[1]['role'],'obligation-identity');self.assertEqual(rows[1]['value'],'Loan01');plan['operations'][1]['obligation']='Other';
  with self.assertRaises(c.Reject) as caught:c.display_arguments(plan,profile)
  self.assertEqual(caught.exception.code,'ARGUMENT_BINDING')

class ReviewCorrections(unittest.TestCase):
 def exact(self,d):
  d['authority']['mode']='ExactPlan';d['authority']['exactOccurrences']=[]
  for i,s in enumerate(d['steps']):
   s['occurrenceId']='occ'+str(i);d['authority']['exactOccurrences'].append({'id':s['occurrenceId'],'predecessorStateHash':s['predecessor'],'planHash':s['planHash'],'postStateHash':s['stateHash'],'terminal':s['terminal'],'predecessorLocation':'linear:'+str(i),'predecessorResourceHash':s['predecessor']})
  return rehash(d)
 def twice(self):
  d=fixture();s=copy.deepcopy(d['steps'][0]);s['id']='step2';p=s['post'];p['balances']['alice.B']='960';p['balances']['bob.B']='36';p['balances']['fees.B']='4';p['gross']['alice.B']='40';p['gross']['bob.B']='4';p['fees']['bob.B']='4';p['receipts']['bob.B']='40';p['ordinary']='12';d['steps'].append(s);return rehash(d)
 def test_exact_occurrence_once_and_reuse(self):
  d=self.exact(fixture());original=d['authorityHash'];self.assertEqual(c.validate(d,context())['status'],'SOURCE_CONTEXT_VALID');second=self.twice()['steps'][1];second['occurrenceId']='occ0';d['steps'].append(second);rehash(d);self.assertEqual(original,d['authorityHash']);self.assertEqual(c.validate(d,context())['firstPredicate'],'EXACT_OCCURRENCE_CONSUMED')
 def test_explicit_two_occurrences(self):
  d=self.exact(self.twice());self.assertEqual(c.validate(d,context())['status'],'SOURCE_CONTEXT_VALID');d['authority']['exactOccurrences'][1]['predecessorStateHash']=d['initialHash'];self.assertEqual(c.validate(rehash(d),context())['firstPredicate'],'EXACT_OCCURRENCE_BINDING')
 def test_external_consumption(self):
  d=self.exact(fixture());ctx=context();ctx['consumedOccurrences'].add('occ0');self.assertEqual(c.validate(d,ctx)['firstPredicate'],'EXACT_OCCURRENCE_CONSUMED')
 def accrue_step(self,d,pre):
  post=copy.deepcopy(pre);post['debt']['accrued']='10';post['debt']['outstanding']='10';post['ordinary']=str(int(pre['ordinary'])-2)
  return {'occurrenceId':'','id':'accrue','predecessor':'0'*64,'authorityHash':'0'*64,'plan':{'action':'accrue','arguments':[{'name':'loan','type':'ObligationId','value':'Loan01'},{'name':'interest','type':'UInt128','value':'10'}],'operations':[{'op':'Accrue','obligation':'Loan01','amount':{'literal':'10'}}],'sidecarHex':''},'planHash':'0'*64,'stateHash':'0'*64,'post':post,'ordinaryCharge':'2','recoveryCharge':'0','now':'155','terminal':False}
 def test_uncreated_debt_accrual_rejects(self):
  d=fixture();d['steps']=[self.accrue_step(d,d['initial'])];self.assertEqual(c.validate(rehash(d),context())['firstPredicate'],'DEBT_NOT_CREATED')
 def test_terminal_absorbing(self):
  d=fixture();d['authority']['netGoal']='18';d['steps'][0]['terminal']=True;rehash(d);self.assertTrue(c.validate(d,context())['terminal']);d['steps'].append(self.accrue_step(d,d['steps'][0]['post']));self.assertEqual(c.validate(rehash(d),context())['firstPredicate'],'TERMINAL_CONTINUATION')

if __name__=='__main__':unittest.main(verbosity=2)
