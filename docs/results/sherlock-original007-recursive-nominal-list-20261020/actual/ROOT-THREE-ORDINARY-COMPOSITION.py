from pathlib import Path
import json,gzip,hashlib,runpy,subprocess,os,stat,re
R=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017')
B=R/'UIREQ007-recursive-list-next-acceptance-20261020/consumer-candidate-02'
V=B/'fresh-focused-source-and-operation-review/actual-MuseWonderHIGH-corrected-consumer-source-threeops-review-01'
load=lambda p:json.loads(Path(p).read_bytes())
j=load(B/'PROPOSED-THREE-OPERATIONS.json')
assert hashlib.sha256((B/'PROPOSED-THREE-OPERATIONS.json').read_bytes()).hexdigest()=='67b09be057e9db8f33fd0d55321366e3a8956cd64d5049c0170198a6b184f835'
helper=Path(j['reuse']['capturer']['path'])
assert hashlib.sha256(helper.read_bytes()).hexdigest()=='713119d9400f82d559967e4296313cec8cde7125dc91a74152500577ced00626'
api=runpy.run_path(str(helper),run_name='root_original007_existing_ordinary_capture_only');pin=api['pin'];check=api['check']
rv=load(V/'RESULT.json');assert rv['actualExit']==0 and rv['terminal']['terminal']=='completed'and rv['finalSHA']=='48affa1ec9be12ccbfeade105c372a122fbae13594768649b9d1eef31334591e'
assert '## Exact three operations: APPROVE each, Root-selected only'in(V/'FINAL-RESPONSE.txt').read_text()
flat={}
def walk(x):
 if isinstance(x,dict):
  if {'path','bytes','sha256'}<=x.keys():
   row={k:x[k]for k in ['path','bytes','sha256']};assert row['path']not in flat or flat[row['path']]==row;flat[row['path']]=row
  for v in x.values():walk(v)
 elif isinstance(x,list):
  for v in x:walk(v)
walk(j['source']);walk(j['reuse']);extra={}
for k in ['tool_extern_inventory','static_prepost_inputs','directory_membership','source_equality']:
 p=Path(j['reuse'][k]['path']);check(j['reuse'][k]);d=json.loads(gzip.decompress(p.read_bytes())if p.suffix=='.gz'else p.read_bytes());walk(d);extra[k]=d
for p in [B/'SOURCE-FREEZE.json',B/'CANDIDATE-MANIFEST.json',B/'PROPOSED-THREE-OPERATIONS.json',V/'FINAL-RESPONSE.txt',V/'RESULT.json',V/'COMMAND.json',Path(__file__)]:walk(pin(p))
rows=sorted(flat.values(),key=lambda x:x['path'])
assert api['static_check'](rows)['all_match']
source=Path(j['source'][0]['path']).parent
assert sorted(str(p)for p in source.iterdir())==sorted(x['path']for x in j['source'])
def membership():
 for d in extra['directory_membership']['directories']:
  root=Path(d['directory']);assert sorted(p.name for p in root.iterdir())==sorted(x['name']for x in d['entries'])
  for x in d['entries']:
   p=root/x['name'];s=p.lstat()
   assert p.is_symlink()==x['symlink']and p.is_file()==x['regularFile']and s.st_size==x['size']and oct(stat.S_IMODE(s.st_mode))==x['mode']
membership()
P=B/'prospective';C=B/'root-actual-three-01'
assert not P.exists()and not P.is_symlink()and not C.exists()
ops=j['operations'];assert len(ops)==3 and all(op['keptBytesPerStream']==131072 for op in ops)
C.mkdir()
def durable(p,x):
 assert not p.exists();api['save'](p,x)
 with p.open('rb')as f:os.fsync(f.fileno())
 fd=os.open(p.parent,os.O_DIRECTORY|os.O_RDONLY);os.fsync(fd);os.close(fd)
durable(C/'ROOT-SELECTION.json',{'selected':True,'sourceAndFullReviewRootRead':True,'review':pin(V/'FINAL-RESPONSE.txt'),'proposal':pin(B/'PROPOSED-THREE-OPERATIONS.json'),'composition':pin(__file__),'conditions1to7Retained':True,'noCoreRebuild':True,'ordinarySdkFreeOnly':True,'serialFailStopNoRetry':True})
durable(C/'FLAT-INPUT-PINS.json',{'members':rows})
(P/'build').mkdir(parents=True)
(P/'outputs').mkdir()
for op in ops:Path(op['environment']['TMPDIR']).mkdir(parents=True)
durable(C/'DIRECTORY-STAGING.json',{'created':[str(P/'build'),str(P/'outputs')]+[x['environment']['TMPDIR']for x in ops],'finalConsumerOutputAbsent':not Path(j['future_outputs']['consumer_output_directory']).exists()})
generated=[];accepted=[];current=None
try:
 for op in ops:
  current=op['id'];membership()
  assert api['static_check'](rows+generated)['all_match']
  assert sorted(str(p)for p in source.iterdir())==sorted(x['path']for x in j['source'])
  phase=C/current;phase.mkdir()
  durable(phase/'ROOT-PRECHECK.json',{'static':api['static_check'](rows+generated),'libraryDirectoryMembership':True,'sourceMembership':True})
  a=api['capture'](op,phase);durable(phase/'RESULT.json',a)
  for n in ['COMMAND.json','RESULT.json','stdout.bin','stderr.bin']:
   with(phase/n).open('rb')as f:os.fsync(f.fileno())
  fd=os.open(phase,os.O_DIRECTORY|os.O_RDONLY);os.fsync(fd);os.close(fd)
  outdir=Path(j['future_outputs']['consumer_output_directory'])
  actualFiles=[pin(p)for p in sorted(outdir.rglob('*'))if p.is_file()]if outdir.exists()else[]
  for x in actualFiles:
   with Path(x['path']).open('rb')as f:os.fsync(f.fileno())
  durable(phase/'ACTUAL-OUTPUTS-FROZEN-BEFORE-CLASSIFICATION.json',{'files':actualFiles,'actualStreams':a['streams']})
  post=api['static_check'](rows+generated);membership()
  durable(phase/'ROOT-POSTCHECK.json',{'static':post,'libraryMembership':True,'rawDurableBeforeClassification':True})
  assert a['actual_exit']==0 and a['controller_failure']is None and post['all_match']
  assert len(a['streams'])==2 and all(x['full_stream_retained']and x['bytes']==x['bytes_seen']==x['bytes_retained']for x in a['streams'])
  stdout=(phase/'stdout.bin').read_text();stderr=(phase/'stderr.bin').read_text()
  if current=='01-external-consumer-compile':
   deps=api['dep_inputs'](j['future_outputs']['dep_info'],{str(Path(x['path']).resolve(strict=True)):x for x in j['source']})
   assert len(deps)==6 and{str(Path(x['path']).resolve())for x in deps}=={str(Path(x['path']).resolve())for x in j['source']}
   generated=[pin(j['future_outputs'][k])for k in ['binary','dep_info']]
   assert{str(p)for p in(P/'build').iterdir()}=={x['path']for x in generated}
   durable(phase/'ACTUAL-GENERATED-AND-DEP-INFO.json',{'products':generated,'depInputs':deps,'exactSix':True,'envDeps':0})
  elif current=='02-list-one':
   assert stdout.strip()==j['expected_list']['test_names'][0]+': test' and stderr==''
  else:
   assert 'test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;'in stdout and stderr==''
   assert{Path(x['path']).name for x in actualFiles}==set(j['future_outputs']['raw_observation_files'])
   assert load(outdir/'checks.json')['passed']is True and(outdir/'errors.jsonl').stat().st_size==0
   manifest=load(outdir/'output-manifest.json')
   assert {x['path']:x['bytes']for x in manifest['files']}=={Path(x['path']).name:x['bytes']for x in actualFiles if Path(x['path']).name!='output-manifest.json'}
  accepted.append({'id':current,'accepted':True,'actual':a,'generated':generated,'outputs':actualFiles})
  durable(phase/'ACCEPTED.json',accepted[-1]);print(json.dumps({'id':current,'accepted':True,'elapsedSeconds':a['elapsed_seconds'],'outputs':len(actualFiles),'streamBytes':[x['bytes']for x in a['streams']]}),flush=True)
 out={'status':'ACTUAL_ORIGINAL007_ONE_RECURSIVE_NOMINAL_LIST_PASS','accepted':True,'phases':accepted,'staticPins':len(rows),'normalCoreOnly':'187be9d','result':load(outdir/'checks.json'),'scope':'One source-aware prefrozen named expected value and actual whole runtime records, not independent full trace/clock golden, additive007/008/binder/formal/native/target/fullEARS closure'}
 durable(C/'OUTCOME.json',out);print(json.dumps({'status':out['status'],'result':out['result']}),flush=True)
except BaseException as e:
 durable(C/'FAILURE.json',{'status':'FAIL_STOP_NO_RETRY','current':current,'error':type(e).__name__+': '+str(e),'accepted':[x['id']for x in accepted]})
 raise
