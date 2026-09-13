import sys,pathlib,json,copy,time,subprocess
ROOT=pathlib.Path('/home/charl/Moriarty-wt-moriarty-release-afk-20260913-implement-production-executor');sys.path.insert(0,str(ROOT/'plugins/moriarty-dev/tests/loan_executor'))
import harness
from harness import LoanFixture,ExecutorRun
from moriarty_dev import loan_executor
from moriarty_dev.loan_exit_retention.exit_retention import CommandResult
OUT=pathlib.Path('/tmp/moriarty-production-grok-r1-independent-probes');rows=[]
p=OUT/'malformed-startup';p.mkdir();fx=LoanFixture(root=p).build();r=ExecutorRun(fx);deps=r.dependencies();original=deps['run_command']
def run(argv,bound):
 if 'show' in argv and 'ActiveEnterTimestampMonotonic' in argv[-1]:return CommandResult(argv,0,'not-a-property\n','')
 return original(argv,bound)
deps['run_command']=run
try:
 result=loan_executor.execute_once(fx.loaded_plan,fx.loaded_admission,deps);rows.append({'name':'malformed_startup','exception':None,'result':result})
except Exception as e:rows.append({'name':'malformed_startup','exception':type(e).__name__,'message':str(e),'root':str(p),'external':str(fx.external),'resultExists':fx.loaded_plan.result_path.exists(),'serviceCalls':fx.services.calls()})
base=json.loads((OUT/'normal'/harness.IMM/'financial-plan.json').read_text()) if (OUT/'normal'/harness.IMM/'financial-plan.json').exists() else copy.deepcopy(fx.loaded_plan.financial)
base['limits']['deadlineMs']=int(time.time()*1000)+1800000
variants={}
for label,change in [('normal',lambda p:None),('wrong_first_role',lambda p:p['roles'].update(firstAddress='e'*64)),('overlap_directories',lambda p:p.update(outputDirectory=p['wallet']['stateDirectory'])),('uint128_overflow',lambda p:p['limits'].update(dustFee=str(2**128)))]:
 v=copy.deepcopy(base);change(v);variants[label]=v
(OUT/'native-plans.json').write_text(json.dumps(variants))
script=OUT/'native-probe.mjs';script.write_text("import {readFileSync} from 'node:fs';\nimport {validatePreviewLaunchPlan} from "+json.dumps((ROOT/'experiments/moriarty-midnight-financial/ledger/preview-launch.mjs').as_uri())+";\nconst plans=JSON.parse(readFileSync("+json.dumps(str(OUT/'native-plans.json'))+",'utf8'));\nfor(const [name,p] of Object.entries(plans)){try{validatePreviewLaunchPlan(p);console.log(JSON.stringify({name,accepted:true}));}catch(e){console.log(JSON.stringify({name,accepted:false,error:e.message}));}}\n")
run=subprocess.run(['node',str(script)],capture_output=True,text=True,timeout=15);rows.append({'name':'native_plan_consumer','command':['node',str(script)],'exit':run.returncode,'stdout':run.stdout,'stderr':run.stderr})
(OUT/'supplement-results.json').write_text(json.dumps(rows,indent=2));print(json.dumps(rows,indent=2))
