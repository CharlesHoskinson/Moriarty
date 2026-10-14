from pathlib import Path
from datetime import datetime, timezone
import subprocess,time,json,hashlib,re
repo=Path('/home/charl/Moriarty/.worktrees/language-static-compile-20261014')
out=Path(__file__).parent
model=repo/'specs/quint/fee_pool_component.qnt'
tests=repo/'specs/quint/fee_pool_component_test.qnt'
witnesses=re.findall(r'^  val (w[A-Z]\w*)\s*=',model.read_text(),re.M)
checks=[('model-typecheck',['quint','typecheck',str(model)]),('test-typecheck',['quint','typecheck',str(tests)]),('deterministic-tests',['quint','test',str(tests),'--main','fee_pool_component_test','--backend','typescript','--seed','20261014','--max-samples','1','--match','.*Test$']),('sample20x24',['quint','run',str(model),'--main','fee_pool_component','--backend','typescript','--seed','20261014','--max-samples','20','--max-steps','24','--invariant','allInvariants','--verbosity','1','--witnesses',*witnesses])]
for name,argv in checks:
 started=datetime.now(timezone.utc).isoformat();t=time.monotonic()
 locked=['flock','/tmp/moriarty-language-wave1-build.lock','timeout','120s',*argv]
 p=subprocess.run(locked,cwd=repo,text=True,capture_output=True)
 (out/(name+'.stdout.log')).write_text(p.stdout)
 (out/(name+'.stderr.log')).write_text(p.stderr)
 record={'argv':locked,'cwd':str(repo),'started_utc':started,'elapsed_seconds':time.monotonic()-t,'exit':p.returncode,'source_sha256':{str(f.relative_to(repo)):hashlib.sha256(f.read_bytes()).hexdigest() for f in [model,tests]},'native_authority':0}
 (out/(name+'.json')).write_text(json.dumps(record,indent=2)+'\n')
 print(name,json.dumps({'exit':p.returncode,'elapsed_seconds':round(record['elapsed_seconds'],3)}),flush=True)
 print(p.stdout,flush=True)
 if p.stderr: print(p.stderr,flush=True)
 if p.returncode: break
