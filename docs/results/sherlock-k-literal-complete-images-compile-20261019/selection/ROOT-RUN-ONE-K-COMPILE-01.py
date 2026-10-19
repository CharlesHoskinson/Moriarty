from pathlib import Path
import json,hashlib,os,runpy
D=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/k-literal-complete-images-counterpart-revision02-source-corrections-20261019')
C=json.loads((D/'root-one-compile-composition/ROOT-ONE-COMPILE-COMPOSITION.json').read_bytes())
def pin(p):
 p=Path(p);b=p.read_bytes();return {'path':str(p),'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest()}
def verify(r):
 a=pin(r['path']);return a['bytes']==r['bytes'] and a['sha256']==r['sha256'] and ('resolved_path' not in r or str(Path(r['path']).resolve(strict=True))==r['resolved_path'])
def save(p,v):
 p=Path(p);data=(json.dumps(v,indent=2)+'\n').encode()
 with p.open('xb') as f:f.write(data);f.flush();os.fsync(f.fileno())
for key in ['plan','actualFreshReview','N2SourcePins','toolEntriesAndJarPins','existing281ClosurePins']:assert verify(C[key]),key
for r in C['callback'].values():
 if isinstance(r,dict):assert verify(r)
S=json.loads(Path(C['N2SourcePins']['path']).read_bytes())
T=json.loads(Path(C['toolEntriesAndJarPins']['path']).read_bytes())
K=json.loads(Path(C['existing281ClosurePins']['path']).read_bytes())
assert len(T['entries'])==4 and len(T['classpath_jar_membership'])==156 and len(K['files'])==281
def check():
 rows=S['files']+T['entries']+K['files']+T['classpath_jar_membership']
 result={'rows':[{'path':r['path'],'match':verify(r)} for r in rows]}
 result['sourceDirectoryMatch']=sorted(x.name for x in Path(S['exactDirectory']).iterdir())==S['exactMembers']
 jars=T['classpath_jar_membership'];parent=Path(jars[0]['path']).parent
 result['jarMembershipMatch']=sorted(str(x) for x in parent.iterdir() if x.suffix.lower()=='.jar')==sorted(x['path'] for x in jars)
 result['allMatch']=all(x['match'] for x in result['rows']) and result['sourceDirectoryMatch'] and result['jarMembershipMatch']
 return result
phase=C['exactPhase'];cwd=Path(phase['cwd']);out=Path(phase['products'][0]).parent
assert not cwd.exists() and not out.exists()
before=check()
assert before['allMatch']
cwd.mkdir(mode=0o700);(cwd/'home').mkdir(mode=0o700);(cwd/'tmp').mkdir(mode=0o700)
save(cwd/'ROOT-PRECHECK.json',before);save(cwd/'COMMAND.json',phase)
api=runpy.run_path(C['callback']['fullsource']['path'],run_name='root_k_compile_only')
record={'id':phase['id'],'command':phase['argv'],'cwd':phase['cwd'],'environment':phase['environment']}
api['launch'](record,phase['argv'],dict(phase['environment']),phase['cwd'],phase['environment']['SHERLOCK_K_ATTEMPT_TOKEN'],536870912)
save(cwd/'RESULT.json',record)
after=check();save(cwd/'ROOT-POSTCHECK.json',after)
streams=[pin(cwd/x) for x in ['stdout','stderr']];save(cwd/'STREAM-PINS.json',streams)
inventory=[]
if out.exists():
 for p in sorted(out.rglob('*')):
  assert not p.is_symlink(),p
  inventory.append({'path':str(p),'kind':'directory' if p.is_dir() else 'file',**({} if p.is_dir() else pin(p))})
save(cwd/'OUTPUT-INVENTORY.json',inventory)
assert after['allMatch']
assert record['returncode']==0 and not record['timed_out'] and not record['remaining'] and not record['containment_breach'],record
assert Path(phase['products'][0]).is_file() and Path(phase['products'][0]).stat().st_size>0
save(cwd/'ROOT-OUTCOME.json',{'status':'PASS_FRESH_DEFINITION_GENERATION_ONLY','record':record,'definition':pin(phase['products'][0]),'outputFileCount':sum(x['kind']=='file' for x in inventory),'streams':streams,'scope':'Exact new K source syntax/definition generation only. No runtime, SDK consumer, context, oracle, refinement, native proof, stagenet or EARS.'})
print(json.dumps({'status':'PASS_FRESH_DEFINITION_GENERATION_ONLY','elapsed':record['elapsed_seconds'],'definition':pin(phase['products'][0]),'streams':streams}),flush=True)
