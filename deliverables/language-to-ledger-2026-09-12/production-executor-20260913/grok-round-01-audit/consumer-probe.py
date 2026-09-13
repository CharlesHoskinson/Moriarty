import sys,pathlib,json,subprocess,copy,time
ROOT=pathlib.Path('/home/charl/Moriarty-wt-moriarty-release-afk-20260913-implement-production-executor');sys.path.insert(0,str(ROOT/'plugins/moriarty-dev/tests/loan_executor'))
import harness
from harness import LoanFixture,write_json
from moriarty_dev import store,records,policy
OUT=pathlib.Path('/tmp/moriarty-production-grok-r1-independent-probes');rows=[]
def emit(name,**v):
 rows.append(dict(name=name,**v));(OUT/'consumer-results.json').write_text(json.dumps(rows,indent=2));print(json.dumps(rows[-1]),flush=True)
for label,fragment in [('cli-control',False),('cli-fragment',True),('cli-history-override',False)]:
 p=OUT/label;p.mkdir();fx=LoanFixture(root=p).build()
 if fragment:
  driver=fx.root/'fragment-driver.py';driver.write_text('''import sys,time,json
from pathlib import Path
sys.path.insert(0,%r)
from moriarty_dev import loan_executor
original=loan_executor.ParentChannel.send
def send(self,phase,detail=None):
 if phase!='contained': return original(self,phase,detail)
 raw=(json.dumps({'schema':loan_executor.PROGRESS_SCHEMA,'phase':phase,'detail':detail})+'\\n').encode()
 self._sock.sendall(raw[:15]);time.sleep(0.15);self._sock.sendall(raw[15:]);return True
loan_executor.ParentChannel.send=send
raise SystemExit(loan_executor.main(sys.argv[1:],dependencies=loan_executor.production_dependencies(Path.cwd(),service_table=%r)))
'''%(str(harness.SCRIPTS),fx.service_table))
  fx.write_runner(argv=[fx.python,'fragment-driver.py','--plan',fx.plan_rel,'--sha256',fx.plan_sha,'--admission',fx.admission_rel])
 command=[fx.python,str(harness.SCRIPTS/'moriarty_dev/cli.py'),'--repo',str(fx.root),'run','--action',fx.action['id'],'--json']
 if label=='cli-history-override':
  for i in range(2):store.record_event(fx.db_path,str(fx.root),fx.action['requirement'],fx.action['capability'],fx.action['candidate'],fx.action['id'],'defect_failure',{'findingId':'NEW-'+str(i)})
  snap=records.load_snapshot(str(fx.root),fx.action['id'],history_reader=store.make_history_reader(fx.db_path));emit('actual_history_policy_control',decision=policy.assess(snap,fx.action),sameDefectFailures=snap['sameDefectFailures'])
  driver=fx.root/'history-driver.py';driver.write_text('''import sys
from pathlib import Path
sys.path.insert(0,%r)
from moriarty_dev import cli
cli.cmd_run(Path(%r),Path(%r),lambda lineage:{'sameDefectFailures':0,'adminCycles':0,'adminSeconds':0,'primaryActive':False,'reproducerVerified':False,'approachChanged':False},%r,True)
'''%(str(harness.SCRIPTS),str(fx.root),str(fx.db_path),fx.action['id']))
  command=[fx.python,str(driver)]
 try:
  run=subprocess.run(command,capture_output=True,text=True,timeout=45)
  (p/'stdout.txt').write_text(run.stdout);(p/'stderr.txt').write_text(run.stderr)
  try:result=json.loads(run.stdout)
  except Exception:result={'stdout':run.stdout,'stderr':run.stderr}
  emit(label,command=command,root=str(p),external=str(fx.external),returncode=run.returncode,result=result,serviceCalls=len(fx.services.calls()))
 except subprocess.TimeoutExpired as e:
  emit(label,command=command,root=str(p),external=str(fx.external),timeout=45)
