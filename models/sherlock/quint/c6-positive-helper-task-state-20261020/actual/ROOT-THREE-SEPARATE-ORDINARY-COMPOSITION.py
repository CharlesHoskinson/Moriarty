from pathlib import Path
import json,hashlib,os,runpy,sys,re,stat
R=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017')
B=R/'dedicated-quint-20261018/batch39-positive-helper-task-state-20261020'
V=B/'review-preparation01/single-complete-review01/actual-MuseWonderHIGH-QuintB39CompleteHelperTaskStateSourceApiOrdinaryMuseReview20261020'
load=lambda p:json.loads(Path(p).read_bytes())
j=load(B/'VALIDATION-PROPOSAL.pending-root-binding.json');assert hashlib.sha256((B/'VALIDATION-PROPOSAL.pending-root-binding.json').read_bytes()).hexdigest()=='8ef35ad0862eff60121dd3091baabeecc61973efcbcba32a521872811f769358'
helper=Path(j['captureHelperUnchanged']['path']);assert hashlib.sha256(helper.read_bytes()).hexdigest()=='713119d9400f82d559967e4296313cec8cde7125dc91a74152500577ced00626'
api=runpy.run_path(str(helper),run_name='root_B39_existing_ordinary_capture_only');pin=api['pin'];check=api['check']
rv=load(V/'RESULT.json');assert rv['actualExit']==0 and rv['terminal']['terminal']=='completed'and rv['finalSHA']=='f7f0f155c5675ad04d4daca00d70b199972431b7f360c590e4a78acb90ec5efa'
for x in [j['installedTreePins'],j['declaredDependencyResolution'],j['sourceFreeze']]:check(x)
t=load(j['installedTreePins']['path']);flat=t['files']+[t['node'],t['cli']]+j['sourceFiles']
assert len(flat)==4681 and len(t['symlinks'])==3
idx=int(sys.argv[1]);assert idx in [0,1,2]
op=j['operations'][idx];C=B/'root-actual-separate-operations01'
if idx==0:C.mkdir(exist_ok=False)
else:
 for prior in j['operations'][:idx]:assert load(C/prior['id']/'ROOT-ACCEPTANCE.json')['accepted']is True
def durable(p,x):
 assert not p.exists();api['save'](p,x)
 with p.open('rb')as f:os.fsync(f.fileno())
 fd=os.open(p.parent,os.O_DIRECTORY|os.O_RDONLY);os.fsync(fd);os.close(fd)
controls=[pin(B/'VALIDATION-PROPOSAL.pending-root-binding.json'),pin(B/'AUTHOR-SOURCE-FREEZE.json'),pin(B/'ROOT-OPAQUE-NINTH-FIXTURE-BINDING-01.json'),pin(V/'FINAL-RESPONSE.txt'),pin(V/'RESULT.json'),pin(helper),pin(__file__)]
def checks():
 for x in controls:check(x)
 a=api['static_check'](flat)
 a['symlinks']=[{'path':x['path'],'match':Path(x['path']).is_symlink()and os.readlink(x['path'])==x['target']and str(Path(x['path']).resolve(strict=True))==x['resolved']and Path(x['path']).is_dir()==x['isDirectory']}for x in t['symlinks']]
 entries=list((B/'source').iterdir())
 a['exactNineRegularEntries']=sorted(p.name for p in entries)==sorted(j['sourceExactEntries'])and all(p.is_file()and not p.is_symlink()for p in entries)
 a['fixtureMode0400']=stat.S_IMODE(Path(j['independentFixture']['path']).stat().st_mode)==0o400
 a['rawChecks']=len(flat)+len(t['symlinks'])
 a['all_match']=a['all_match']and all(x['match']for x in a['symlinks'])and a['exactNineRegularEntries']and a['fixtureMode0400']and a['rawChecks']==4684
 return a
before=checks();assert before['all_match']
phase=C/op['id'];assert not phase.exists()
durable(C/('ROOT-SELECTION-0'+str(idx+1)+'.json'),{'selected':True,'operation':op,'separateExplicitRootSelection':True,'review':pin(V/'FINAL-RESPONSE.txt'),'composition':pin(__file__),'RootPrivateBodySemanticReads':0,'conditionAllPriorPass':idx>0,'noAutomaticNextDispatch':True})
phase.mkdir()
durable(phase/'ROOT-PRECHECK.json',before)
a=api['capture'](op,phase);durable(phase/'RESULT.json',a)
for n in ['COMMAND.json','RESULT.json','stdout.bin','stderr.bin']:
 with(phase/n).open('rb')as f:os.fsync(f.fileno())
fd=os.open(phase,os.O_DIRECTORY|os.O_RDONLY);os.fsync(fd);os.close(fd)
after=checks();durable(phase/'ROOT-POSTCHECK.json',after)
try:
 assert a['actual_exit']==0 and a['controller_failure']is None and after['all_match']
 assert len(a['streams'])==2 and all(s['full_stream_retained']and s['bytes']==s['bytes_seen']==s['bytes_retained']for s in a['streams'])
 out=(phase/'stdout.bin').read_text();err=(phase/'stderr.bin').read_text();assert err==''
 if idx==0:assert out==''
 else:
  names=re.findall(r'^\s*ok (\w+) passed 1 test\(s\)\s*$',out,re.M)
  assert names==op['expected']['passingNames']
  assert len(re.findall(r'^\s*ok ',out,re.M))==1
  assert len(re.findall(r'^\s*1 passing \(\d+ms\)\s*$',out,re.M))==1
  assert not re.search(r'failed|ignored|pending|QNT|DEBUG|error:|truncat',out,re.I)
  other=[x.strip()for x in out.splitlines()if x.strip()and not re.fullmatch(r'ok \w+ passed 1 test\(s\)',x.strip())and not re.fullmatch(r'1 passing \(\d+ms\)',x.strip())]
  assert other==['c6PositiveStateTest']
 result={'accepted':True,'operation':op['id'],'actual':a,'checksEachPrePost':4684,'exactNineRegularSources':True,'fixtureOpaque':True,'scope':'Finite positive full HELPER/RESULT/STAGED/TASK and STATE41 representation with external canonical-image/host-fit/cap-success/limit premises only; no malformed/refusal/auth/admission/StepInv/native/refinement/EARS closure'}
 durable(phase/'ROOT-ACCEPTANCE.json',result);print(json.dumps({'accepted':True,'id':op['id'],'seconds':a['elapsed_seconds'],'streams':[x['bytes']for x in a['streams']],'checks':4684}),flush=True)
except BaseException as e:
 durable(phase/'FAILURE.json',{'status':'FAIL_STOP_NO_RETRY','error':type(e).__name__+': '+str(e),'rawPreserved':True});raise
