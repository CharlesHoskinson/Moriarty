import sys,pathlib,json,subprocess,time,os,signal
ROOT=pathlib.Path('/home/charl/Moriarty-wt-moriarty-release-afk-20260913-implement-production-executor');sys.path.insert(0,str(ROOT/'plugins/moriarty-dev/tests/loan_executor'))
import harness
from harness import LoanFixture
OUT=pathlib.Path('/tmp/moriarty-production-grok-r1-independent-probes');p=OUT/'true-after-startup';p.mkdir();fx=LoanFixture(root=p).build();marker=p/'durable-startup-barrier.json'
driver=p/'phase-child.py';driver.write_text('''import sys,json
from pathlib import Path
sys.path.insert(0,%r)
from moriarty_dev import loan_executor
original=loan_executor.ParentChannel.send
def send(self,phase,detail=None):
 if phase=='launched':
  result=loan_executor.production_write_exclusive(Path(%r),json.dumps({'phase':phase,'detail':detail}).encode(),timeout_seconds=2)
  assert result.persisted
 return original(self,phase,detail)
loan_executor.ParentChannel.send=send
raise SystemExit(loan_executor.main(sys.argv[1:],dependencies=loan_executor.production_dependencies(Path.cwd(),service_table=%r)))
'''%(str(harness.SCRIPTS),str(marker),fx.service_table))
fx.write_runner(argv=[fx.python,'phase-child.py','--plan',fx.plan_rel,'--sha256',fx.plan_sha,'--admission',fx.admission_rel]);fx.services._save('script.json',{'runningPolls':30})
command=[fx.python,str(harness.SCRIPTS/'moriarty_dev/cli.py'),'--repo',str(p),'run','--action',fx.action['id'],'--json'];a=open(p/'parent-stdout.txt','w');b=open(p/'parent-stderr.txt','w');proc=subprocess.Popen(command,stdout=a,stderr=b)
start=time.monotonic();deadline=start+20
while time.monotonic()<deadline and not marker.exists() and proc.poll() is None:time.sleep(.02)
barrier=json.loads(marker.read_text()) if marker.exists() else None
if proc.poll() is None:os.kill(proc.pid,signal.SIGKILL)
ret=proc.wait(timeout=5);a.close();b.close();resultpath=p/fx.plan['resultPath'];deadline=time.monotonic()+25
while time.monotonic()<deadline and not resultpath.exists():time.sleep(.05)
result=json.loads(resultpath.read_text()) if resultpath.exists() else None
out={'command':command,'root':str(p),'external':str(fx.external),'barrier':barrier,'parentExit':ret,'elapsed':time.monotonic()-start,'result':result,'ownership':json.loads(fx.ownership_path.read_text())};(OUT/'phase-results.json').write_text(json.dumps(out,indent=2));print(json.dumps({'barrier':barrier,'parentExit':ret,'status':result.get('status') if result else None,'failureCode':result.get('failureCode') if result else None,'containmentComplete':result.get('containmentComplete') if result else None}))
