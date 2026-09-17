import json,sys,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
H=ROOT/'experiments/moriarty-language/formal/k'
sys.path.insert(0,str(H))
import run
run.BUILD=H/'.build-lifecycle-v1'
run.DEFINITION=run.BUILD/'lifecycle-v1-kompiled'
run.DEADLINE=time.monotonic()+190
lock=json.loads((H/'expression-toolchain.lock.json').read_text())
assert all(run.sha(Path(n))==v for n,v in lock['files'].items())
files=[*H.glob('lifecycle-*.k'),H/'expression-json.k',H/'lifecycle_codec.py',H/'expression_codec.py',H/'lifecycle-parser.py',H/'expression-toolchain.lock.json']
source={str(p.relative_to(H)):run.sha(p) for p in files}
run.BUILD.mkdir(exist_ok=True)
with (run.BUILD/'compile-attempt.json').open('x') as f:json.dump({'sources':source,'started':time.time(),'attemptId':'lifecycle-compile01','priorCompiles':11,'newCompile':1},f)
try:
 run.command(['/usr/bin/kompile',str(H/'lifecycle-v1.k'),'--main-module','MORIARTY-LIFECYCLE-EXPRESSION-V1','--syntax-module','MORIARTY-LIFECYCLE-EXPRESSION-V1','--backend','llvm','--output-definition',str(run.DEFINITION)],180,'compile')
 assert source=={str(p.relative_to(H)):run.sha(p) for p in files}
 run.write(run.BUILD/'binding.json',{'sources':source,'artifacts':run.artifacts(),'krunInvocations':0})
 print('LifecycleCompiled')
except run.RunnerError as e:print(str(e));sys.exit(2)
