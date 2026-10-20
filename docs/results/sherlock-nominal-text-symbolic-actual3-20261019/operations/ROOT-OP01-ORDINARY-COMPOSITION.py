from pathlib import Path
import json,hashlib,runpy,os,re,sys
sys.dont_write_bytecode=True
Q=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/nominal-text-passive-symbolic-comparator-20261019/revision03-split-Opus-candidate')
def pin(p):
 p=Path(p);b=p.read_bytes();return dict(path=str(p),bytes=len(b),sha256=hashlib.sha256(b).hexdigest())
def durable(p,v):
 with Path(p).open('xb') as f:
  f.write((json.dumps(v,indent=2)+'\n').encode());f.flush();os.fsync(f.fileno())
def sync(p):
 fd=os.open(str(p),os.O_RDONLY);os.fsync(fd);os.close(fd)
planp=Q/'PROPOSED-THREE-OPERATIONS.json'
assert pin(planp)['sha256']=='6b593428751bd7a574ae6e516dcf13186faebfa37e8310d8566433d3e4e857ee'
plan=json.loads(planp.read_bytes());hp=plan['helper'];assert pin(hp['path'])==hp
h=runpy.run_path(hp['path'],run_name='root_existing_ordinary_functions_only')
op=plan['operations'][0];assert op['id']=='01-synthetic-only'
rows=json.loads((Q/'STATIC-INPUT-PINS.json').read_bytes())['files']+op['requiredInputs']+[pin(__file__),pin(planp)]
pre=h['static_check'](rows);assert pre['all_match']
qa=Q/'ordinary-QA';tmp=qa/'tmp';assert tmp.is_dir() and not list(tmp.iterdir())
phase=qa/'actual-three-01'/'01-synthetic-only';phase.mkdir(parents=True,exist_ok=False)
durable(phase/'ROOT-SELECTION.json',dict(status='ROOT_SELECTED_ONE_REVIEWED_ORDINARY_SYNTHETIC_OPERATION',caller=pin(__file__),review=pin(Q/'review-delivery-revision02/actual-Gemini4ArgonHIGH-fallback-source-review-01/FINAL-RESPONSE.txt'),operation=op['id'],noExpectedOrActualCaptureBodyRead=True,noRetry=True))
durable(phase/'PRE-PIN-CHECK.json',pre)
result=h['capture'](op,phase)
durable(phase/'RESULT.json',result)
for name in ['COMMAND.json','stdout.bin','stderr.bin']:sync(phase/name)
sync(phase)
post=h['static_check'](rows);durable(phase/'POST-PIN-CHECK.json',post);sync(phase)
try:
 assert result['actual_exit']==0 and result['controller_failure'] is None
 assert pre['all_match'] and post['all_match']
 assert len(result['streams'])==2
 for r in result['streams']:
  assert pin(r['path'])=={k:r[k] for k in ['path','bytes','sha256']}
  assert r['full_stream_retained'] and r['bytes_seen']==r['bytes_retained']
 assert (phase/'stdout.bin').read_bytes()==b''
 expected=op['expected']['exactPassingNames']
 stderr=(phase/'stderr.bin').read_text()
 lines=stderr.splitlines()
 exact=[f"{name.rsplit('.',1)[1]} ({name}) ... ok" for name in expected]
 assert lines[:23]==exact, 'verbose test envelope differs'
 assert len(lines)==28 and lines[23]=='' and lines[24]=='-'*70
 assert re.fullmatch(r'Ran 23 tests in [0-9]+(?:\.[0-9]+)?s',lines[25])
 assert lines[26:]==['','OK']
 durable(phase/'ROOT-OUTCOME.json',dict(status='PASS_EXACT23_SYNTHETIC_TESTS_ONLY',elapsedSeconds=result['elapsed_seconds'],source=pin(Q/'source/symbolic_compare.py'),tests=pin(Q/'source/test_symbolic_compare.py'),pins=len(rows),noRealProjectionOrExpectedRead=True))
 print(json.dumps(json.loads((phase/'ROOT-OUTCOME.json').read_bytes())))
except BaseException as e:
 durable(phase/'ACCEPTANCE-FAILURE.json',dict(status='FAIL_STOP_NO_RETRY',error=type(e).__name__+': '+str(e)))
 raise
