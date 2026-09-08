import sys,json
from pathlib import Path
w=Path('/home/charl/Moriarty/.worktrees/moriarty-dev-plugin-grok');sys.path[:0]=[str(w/'plugins/moriarty-dev/scripts'),str(w/'plugins/moriarty-dev/tests')]
import test_records as t
rows=[]
for name in ['positive','missing-charges','wrong-authority','historical-exact','historical-dot-alias']:
 c=t.GenuineRegisters();c.setUp()
 try:
  c._install_funded_loan_review();p=c.root/t.CURRENT_ACCOUNTING_REL;d=t.load_json(p)
  if name=='missing-charges':
   for key in ['charges','reserved','externalPackageCharges']:d.pop(key,None)
  if name=='wrong-authority':d['authority_sha256']='0'*64
  t.dump_json(p,d)
  if name.startswith('historical-'):
   q=c.root/t.BUDGET_REL;t.dump_json(q,d);camp=t.load_json(c.campaign_path);camp['campaigns']['sp01-loan-swap-grok-01']['currentAccounting']['path']=('./' if name.endswith('dot-alias') else '')+t.BUDGET_REL;t.dump_json(c.campaign_path,camp)
  snap=t.load_snapshot(str(c.root),'sp01-loan-review',history_reader=t.verified_history());a=next(x for x in t.read_actions(str(c.root)) if x['id']=='sp01-loan-review');decision=t.assess(snap,a)
  rows.append({'case':name,'allow':decision['allow'],'resourceAdmitted':snap['resourceAdmitted'],'missing':snap['missingEvidence']})
 finally:c.doCleanups()
print(json.dumps(rows,indent=2))
