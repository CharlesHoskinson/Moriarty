import copy,importlib.util,json,unittest
from pathlib import Path
O=Path(__file__).parent
s=importlib.util.spec_from_file_location('branch',O/'branch-checker-04.py');B=importlib.util.module_from_spec(s);s.loader.exec_module(B)
s=importlib.util.spec_from_file_location('fixtures03',O/'context-checker-03.test.py');F=importlib.util.module_from_spec(s);s.loader.exec_module(F)
def seal(node):node['id']=B.digest('node',{k:v for k,v in node.items() if k!='id'});return node
def node(g,kind,refs,body,outputs,ordinary,recovery=0):return seal({'id':'0'*64,'kind':kind,'origin':g['seed']['authorityHash'],'profileHash':g['profileHash'],'predecessors':refs,'body':body,'outputs':outputs,'ordinaryCharge':str(ordinary),'recoveryCharge':str(recovery),'now':'155'})
def ref(n,i=0):return {'node':n['id'],'output':str(i)}
def fixture():
 seed=F.fixture();profile=B.profile_hash();g={'format':'branch-source-example-4','profileHash':profile,'compositionAuthorization':{'originHash':seed['authorityHash'],'profileHash':profile,'operators':['Split','Join'],'signatureAbsent':True},'seed':seed,'nodes':[],'current':[]};root=B.root_output(seed);a=copy.deepcopy(root);b=copy.deepcopy(root)
 for out,asset,ordinary,recovery,owns in [(a,'A','7','1',True),(b,'B','7','2',False)]:
  out['ownedAccounts']=[k for k in B.ACCOUNTS if k.endswith('.'+asset)];out['ownsDebt']=owns;out['state']['ordinary']=ordinary;out['state']['recovery']=recovery
  for k in B.ACCOUNTS:
   if not k.endswith('.'+asset):out['state']['balances'][k]='0';out['grossRemaining'][k]='0';out['feeRemaining'][k]='0'
 a['retiredOrdinary']='1';a['retiredRecovery']='1'
 split=node(g,'Split',[{'node':B.digest('root',seed),'output':'0'}],{'partitions':[{'accounts':a['ownedAccounts'],'ownsDebt':True,'ordinary':'7','recovery':'1'},{'accounts':b['ownedAccounts'],'ownsDebt':False,'ordinary':'7','recovery':'2'}],'retireOrdinary':'1','retireRecovery':'1'},[a,b],1)
 pa=copy.deepcopy(a);sa=pa['state'];sa['balances']['alice.A']='90';sa['balances']['bob.A']='10';sa['gross']['alice.A']='10';sa['receipts']['bob.A']='10';sa['debt']['created']='10';sa['debt']['outstanding']='10';sa['ordinary']='3';pa['grossRemaining']['alice.A']='90'
 planA={'action':'loan','arguments':[{'name':'loan','type':'ObligationId','value':'Loan01'},{'name':'principal','type':'UInt128','value':'10'}],'operations':[{'op':'Transfer','from':'alice','to':'bob','asset':'A','amount':{'literal':'10'}},{'op':'CreateDebt','obligation':'Loan01','amount':{'literal':'10'}}],'sidecarHex':''}
 left=node(g,'Action',[ref(split,0)],planA,[pa],4)
 pb=copy.deepcopy(b);sb=pb['state'];sb['balances']['alice.B']='960';sb['balances']['bob.B']='36';sb['balances']['fees.B']='4';sb['gross']['alice.B']='40';sb['gross']['bob.B']='4';sb['fees']['bob.B']='4';sb['receipts']['bob.B']='40';sb['ordinary']='3';pb['grossRemaining']['alice.B']='60';pb['grossRemaining']['bob.B']='96';pb['feeRemaining']['bob.B']='1'
 right=node(g,'Action',[ref(split,1)],copy.deepcopy(seed['steps'][0]['plan']),[pb],4)
 out=copy.deepcopy(root);out['retiredOrdinary']='1';out['retiredRecovery']='1';state=out['state'];state['ordinary']='5';state['recovery']='3';state['debt']=copy.deepcopy(sa['debt'])
 for k in B.ACCOUNTS:
  selected=pa if k.endswith('.A') else pb
  for field in ['balances','gross','fees','receipts']:state[field][k]=selected['state'][field][k]
  for field in ['grossRemaining','feeRemaining']:out[field][k]=selected[field][k]
 join=node(g,'Join',[ref(left),ref(right)],{},[out],1)
 g['nodes']=[split,left,right,join];g['current']=[ref(join)];return g
def context(g):return {'base':F.context(),'consumedOutputs':set(),'compositionAuthorityHash':B.digest('composition-authority',g['compositionAuthorization'])}
def reseal(g):
 mapping={}
 for n in g['nodes']:
  old=n['id']
  for r in n['predecessors']:r['node']=mapping.get(r['node'],r['node'])
  seal(n);mapping[old]=n['id']
 for r in g['current']:r['node']=mapping.get(r['node'],r['node'])
 return g
class Histories(unittest.TestCase):
 def expect(self,g,code,ctx=None):self.assertEqual(B.validate(g,ctx or context(g)).get('firstPredicate'),code)
 def test_actual_split_independent_actions_join(self):
  g=fixture();self.assertEqual(B.validate(g,context(g))['status'],'SOURCE_GRAPH_VALID');out=g['nodes'][-1]['outputs'][0];self.assertEqual(out['state']['gross']['alice.B'],'40');self.assertEqual(out['state']['debt']['outstanding'],'10');self.assertEqual(out['state']['ordinary'],'5');self.assertEqual(out['retiredOrdinary'],'1')
 def test_duplicate_branch(self):
  g=fixture();g['nodes'][-1]['predecessors'][1]=copy.deepcopy(g['nodes'][-1]['predecessors'][0]);self.expect(reseal(g),'PREDECESSOR_DUPLICATE')
 def test_prefix_counted_twice(self):
  g=fixture();g['nodes'][-1]['outputs'][0]['state']['gross']['alice.B']='60';self.expect(reseal(g),'OUTPUT_RELATION')
 def test_consumed_predecessor(self):
  g=fixture();ctx=context(g);ctx['consumedOutputs'].add((g['nodes'][0]['id'],'0'));self.expect(g,'CURRENTNESS',ctx)
 def test_overlapping_account_and_backing(self):
  g=fixture();g['nodes'][0]['body']['partitions'][0]['accounts'].append('alice.B');self.expect(reseal(g),'OWNERSHIP_PARTITION')
 def test_cloned_work_and_caps(self):
  g=fixture();g['nodes'][0]['body']['partitions'][0]['ordinary']='16';self.expect(reseal(g),'WORK_PARTITION')
  g=fixture();g['nodes'][0]['outputs'][1]['grossRemaining']['alice.A']='100';self.expect(reseal(g),'OUTPUT_RELATION')
 def test_origin_and_profile_not_replaceable(self):
  for field in ['origin','profileHash']:
   g=fixture();g['nodes'][2][field]='a'*64;self.expect(reseal(g),'IMMUTABLE_ORIGIN')
 def test_debt_cannot_be_dropped(self):
  g=fixture();g['nodes'][-1]['outputs'][0]['state']['debt']=B.zero_debt();self.expect(reseal(g),'OUTPUT_RELATION')
 def test_retired_work_not_restored(self):
  g=fixture();g['nodes'][-1]['outputs'][0]['state']['ordinary']='6';g['nodes'][-1]['outputs'][0]['retiredOrdinary']='0';self.expect(reseal(g),'OUTPUT_RELATION')
 def test_wrong_charge(self):
  g=fixture();g['nodes'][-1]['ordinaryCharge']='0';self.expect(reseal(g),'DERIVED_CHARGE')
 def test_action_cannot_spend_sibling(self):
  g=fixture();g['nodes'][1]['body']=copy.deepcopy(g['nodes'][2]['body']);self.expect(reseal(g),'ACTION_OWNERSHIP')
 def test_no_implicit_composition_authority(self):
  g=fixture();ctx=context(g);ctx['compositionAuthorityHash']='0'*64;self.expect(g,'COMPOSITION_AUTHORITY',ctx)
 def test_prior_prefix_is_linked_not_supplied(self):
  g=fixture();g['seed']['steps'][0]['post']['gross']['alice.B']='19';F.rehash(g['seed']);self.expect(g,'SEED_TRANSITION_FRAME')
class BackingHistory(unittest.TestCase):
 def test_reserved_duty_survives_split_join(self):
  seed=F.cancel(F.fixture());profile=B.profile_hash();g={'format':'branch-source-example-4','profileHash':profile,'compositionAuthorization':{'originHash':seed['authorityHash'],'profileHash':profile,'operators':['Split','Join'],'signatureAbsent':True},'seed':seed,'nodes':[],'current':[]};root=B.root_output(seed);a=copy.deepcopy(root);b=copy.deepcopy(root)
  for out,asset,owns in [(a,'A',True),(b,'B',False)]:
   out['ownedAccounts']=[k for k in B.ACCOUNTS if k.endswith('.'+asset)];out['ownsDebt']=owns;out['state']['ordinary']='7';out['state']['recovery']='1'
   for k in B.ACCOUNTS:
    if k not in out['ownedAccounts']:out['state']['balances'][k]='0';out['grossRemaining'][k]='0';out['feeRemaining'][k]='0'
   out['state']['duties']=[] if asset=='A' else copy.deepcopy(root['state']['duties'])
  a['retiredOrdinary']='1';a['retiredRecovery']='1'
  sp=node(g,'Split',[{'node':B.digest('root',seed),'output':'0'}],{'partitions':[{'accounts':a['ownedAccounts'],'ownsDebt':True,'ordinary':'7','recovery':'1'},{'accounts':b['ownedAccounts'],'ownsDebt':False,'ordinary':'7','recovery':'1'}],'retireOrdinary':'1','retireRecovery':'1'},[a,b],1)
  joined=copy.deepcopy(root);joined['state']['ordinary']='13';joined['state']['recovery']='2';joined['retiredOrdinary']='1';joined['retiredRecovery']='1';jn=node(g,'Join',[ref(sp,0),ref(sp,1)],{},[joined],1);g['nodes']=[sp,jn];g['current']=[ref(jn)]
  self.assertEqual(B.validate(g,context(g))['status'],'SOURCE_GRAPH_VALID');self.assertEqual(joined['state']['duties'][0]['amount'],'2');jn['outputs'][0]['state']['duties']=[];self.assertEqual(B.validate(reseal(g),context(g))['firstPredicate'],'OUTPUT_RELATION')

if __name__=='__main__':unittest.main(verbosity=2)
