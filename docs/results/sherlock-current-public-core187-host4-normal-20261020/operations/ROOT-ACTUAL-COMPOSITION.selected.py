from pathlib import Path
import json,hashlib,runpy,os,stat
R=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017')
B=R/'current-public-Core187-Host4-normal-qualification-proposal-20261020'
V=B/'review-delivery-revision02/actual-MuseWonderHIGH-current187-normal-integration-review-01'
load=lambda p:json.loads(Path(p).read_text())
j=load(B/'EXACT-ONE-PROPOSED-OPERATION.json')
assert hashlib.sha256((B/'EXACT-ONE-PROPOSED-OPERATION.json').read_bytes()).hexdigest()=='b1a365a77cc4ef4cf91c9deb1972c483acac3af47b008e299dd122420e038df4'
op=j['operations'][0];h=j['captureHelper']
assert hashlib.sha256(Path(h['path']).read_bytes()).hexdigest()=='713119d9400f82d559967e4296313cec8cde7125dc91a74152500577ced00626'
api=runpy.run_path(h['path'],run_name='root_current187_existing_capture_only')
pin=api['pin'];check=api['check']
rv=load(V/'RESULT.json');assert rv['actualExit']==0 and rv['terminal']['terminal']=='completed'and rv['finalSHA']=='f10c2eecde68726f07ce8f34212bd147b3efa6b08f57604069d73b28c8805fdd'
assert 'ELIGIBLE for Root selection' in(V/'FINAL-RESPONSE.txt').read_text()
flat=load(B/'STATIC-PREPOST-PINS.json')['files'];assert len(flat)==632
for row in load(B/'HANDOFF.json').values():
 if isinstance(row,dict)and{'path','bytes','sha256'}<=row.keys():check(row)
pre=api['static_check'](flat);assert pre['all_match']
def inventory():
 ld=load(B/'EXACT-L-DIRECTORY-MEMBERSHIP.json');ans=[]
 for dr in ld['directories']:
  path=Path(dr['directory']);assert sorted(p.name for p in path.iterdir())==sorted(r['name']for r in dr['entries'])
  for r in dr['entries']:
   p=path/r['name'];s=p.lstat()
   assert stat.S_ISLNK(s.st_mode)==r['symlink']and stat.S_ISREG(s.st_mode)==r['regularFile']
   assert oct(stat.S_IMODE(s.st_mode))==r['mode']and s.st_size==r['size']
   if r['symlink']:assert os.readlink(p)==r['linkTarget']
   if 'qualifiedLibrary'in r:check(r['qualifiedLibrary'])
  ans.append({'directory':str(path),'count':len(dr['entries']),'exact':True})
 return ans
beforeInv=inventory();W=Path(op['cwd']);C=B/'actual-one-01';phase=C/'01'
for p in [W,C]:
 assert not p.exists()and not p.is_symlink()
 assert not any(x.is_symlink()for x in p.parents)
for p in op['outputs']:assert not Path(p).exists()
assert op['wallSeconds']==30 and op['keptBytesPerStream']==131072
W.mkdir();(W/'build').mkdir();Path(op['environment']['TMPDIR']).mkdir(parents=True)
C.mkdir();phase.mkdir()
def durable(p,x):
 assert not p.exists();api['save'](p,x)
 with p.open('rb')as f:os.fsync(f.fileno())
 fd=os.open(p.parent,os.O_DIRECTORY|os.O_RDONLY);os.fsync(fd);os.close(fd)
durable(C/'ROOT-SELECTION.json',{'selected':True,'review':pin(V/'FINAL-RESPONSE.txt'),'RootReadFullReview':True,'source':pin(B/'SOURCE-COMPOSITION.json'),'proposal':pin(B/'EXACT-ONE-PROPOSED-OPERATION.json'),'composition':pin(__file__),'oneNewNormalRlibOnly':True,'oldMetadataTestConsumerReplays':0})
durable(C/'PRECHECK.json',{'static':pre,'LDirectories':beforeInv})
a=api['capture'](op,phase);durable(phase/'RESULT.json',a)
for n in ['RESULT.json','COMMAND.json','stdout.bin','stderr.bin']:
 with(phase/n).open('rb')as f:os.fsync(f.fileno())
fd=os.open(phase,os.O_DIRECTORY|os.O_RDONLY);os.fsync(fd);os.close(fd)
post=api['static_check'](flat);afterInv=inventory()
durable(C/'POSTCHECK.json',{'static':post,'LDirectories':afterInv})
ok=a['actual_exit']==0 and a['controller_failure']is None and post['all_match']and all(s['full_stream_retained']and s['bytes']==s['bytes_seen']==s['bytes_retained']for s in a['streams'])and(phase/'stdout.bin').stat().st_size==0
products=[];deps=[]
if ok:
 for p in op['outputs']:assert Path(p).is_file()and not Path(p).is_symlink()and Path(p).stat().st_size>0
 products=[pin(p)for p in op['outputs']]
 durable(C/'ACTUAL-GENERATED-PINS.json',{'members':products})
 expected=load(B/'EXPECTED-NORMAL-DEP-MEMBERSHIP.json')['files']
 allowed={str(Path(x['path']).resolve(strict=True)):x for x in expected}
 assert len(allowed)==len(expected)==115
 deps=api['dep_inputs'](op['outputs'][1],allowed)
 assert len(deps)==115 and{str(Path(x['path']).resolve(strict=True))for x in deps}==set(allowed)
 assert any(x['path'].endswith('/c6_phys_host.rs')for x in deps)
 durable(C/'ACTUAL-EXACT115-NORMAL-DEPS.json',{'files':deps,'exactMembership':True,'noCfgTest':True,'newHost4Present':True})
 for row in products:check(row)
out={'status':'ACTUAL_CURRENT187_HOST4_NORMAL_RLIB_PASS'if ok else'ACTUAL_CURRENT187_NORMAL_RLIB_FAIL_STOP_NO_RETRY','accepted':ok,'actual':a,'sourceMembers':187,'staticPins':632,'normalDependencies':len(deps),'generated':products,'stderr':'Full retained diagnostics; inspect before warning classification','scope':'Current187 full normal source/API/codegen integration only; no Host4runtime, externalABI, SDKnative, proof, target, EARS acceptance','replay':False}
durable(C/'ROOT-OUTCOME.json',out);print(json.dumps(out))
