import sys,json
from pathlib import Path
w=Path("/home/charl/Moriarty/.worktrees/moriarty-dev-plugin-grok")
sys.path[:0]=[str(w/"plugins/moriarty-dev/scripts"),str(w/"plugins/moriarty-dev/tests")]
import test_records as t
rows=[]
for name,patch in [("positive",{}),("exhausted-by-planning",{"planning_charge_seconds":20000}),("negative-planning",{"planning_charge_seconds":-1}),("unknown-budget-key",{"inventedAuthority":True}),("no-master-or-package-cap",{"master_limit_seconds":None,"package_limit_seconds":None})]:
 c=t.GenuineRegisters(); c.setUp()
 try:
  c._install_loan_family([t.loan_action("review","commands.json#driver")],integer_budget=True)
  p=c.root/t.BUDGET_REL; d=t.load_json(p); d.update(patch); t.dump_json(p,d)
  snap=t.load_snapshot(str(c.root),"sp01-loan-review",history_reader=t.verified_history())
  decision=t.assess(snap,t.read_actions(str(c.root))[0])
  rows.append({"case":name,"expectedAllow":name=="positive","actualAllow":decision["allow"],"resourceAdmitted":snap["resourceAdmitted"],"missingEvidence":snap["missingEvidence"]})
 finally: c.doCleanups()
print(json.dumps(rows,indent=2))
