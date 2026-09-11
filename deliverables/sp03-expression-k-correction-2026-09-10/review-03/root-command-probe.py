"""Actual harmless child processes exercise production command transport only; no K invocation."""
from pathlib import Path
import sys, tempfile, json, time, hashlib
root=Path(sys.argv[1]);sys.path.insert(0,str(root/'experiments/moriarty-language/formal/k'))
import run
before=hashlib.sha256((run.HERE/'run.py').read_bytes()).hexdigest()
results=[]
for label,stderr_size,expected in [('exact-aggregate',67108862,None),('one-byte-over',67108863,'K_OUTPUT_BOUND'),('stderr-alone-over',67108865,'K_OUTPUT_BOUND')]:
 with tempfile.TemporaryDirectory() as td:
  run.select_suite('expression-v1');run.BUILD=Path(td);run.DEADLINE=time.monotonic()+30
  script=f'import os;os.write(1,b"ok");os.ftruncate(2,{stderr_size})'
  outcome=None
  try:
   value=run.command([sys.executable,'-c',script],5,label)
   assert value=='ok'
  except run.RunnerError as exc: outcome=str(exc)
  assert outcome==expected,(label,outcome,expected)
  receipt=json.loads((Path(td)/(label+'.command.json')).read_text())
  assert receipt['returncode']==0 and receipt['timedOut'] is False
  assert isinstance(receipt['elapsedSeconds'],float) and receipt['elapsedSeconds']>=0
  assert isinstance(receipt['aggregateSeconds'],float) and receipt['aggregateSeconds']>=0
  results.append({'case':label,'stdoutBytes':2,'stderrBytes':stderr_size,'outcome':outcome or 'accepted','receiptRetained':True})
assert before==hashlib.sha256((run.HERE/'run.py').read_bytes()).hexdigest()
print(json.dumps({'status':'PASS','sourceSha256':before,'cases':results,'scope':'Production command() with actual Python child and sparse stderr files, no K/toolchain execution.'},indent=2))
