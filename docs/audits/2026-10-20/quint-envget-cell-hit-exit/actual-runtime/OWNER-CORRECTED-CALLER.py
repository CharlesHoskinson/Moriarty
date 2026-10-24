from pathlib import Path
import json,hashlib,os,runpy,sys
R=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017')
B=R/'dedicated-quint-20261018/batch41-EnvGetCell-EnvGetExit-owner-positive-20261020'
A=B/'actual-before-runtime01'
V=A/'review-preparation02/actual-MuseWonderHIGH-source-API-runtime-review-01'
load=lambda p:json.loads(Path(p).read_bytes())
def fsync_file(p):
 with Path(p).open('rb')as f:os.fsync(f.fileno())
def fsync_dir(p):
 fd=os.open(p,os.O_DIRECTORY|os.O_RDONLY);os.fsync(fd);os.close(fd)
assert len(sys.argv)==2 and sys.argv[1] in ('01','02'), 'exactly one explicit Root selection: 01 or 02'
sel=int(sys.argv[1])-1
prop=A/'VALIDATION-PROPOSAL.json'
assert hashlib.sha256(prop.read_bytes()).hexdigest()=='295df1ff3a78cb1621a3c5e3b793484cb21447b2d38e080b1083fb79537900b1'
j=load(prop);assert len(j['operations'])==2
helper=Path(j['captureHelperUnchanged']['path'])
assert hashlib.sha256(helper.read_bytes()).hexdigest()=='713119d9400f82d559967e4296313cec8cde7125dc91a74152500577ced00626'
api=runpy.run_path(str(helper),run_name='root_B41_run_itf_capture_only')
pin,check=api['pin'],api['check']
rv=load(V/'RESULT.json')
assert rv['actualExit']==0 and rv['terminal']['terminal']=='completed' and rv['finalSHA']=='4fab8edea69b0af32735c4ed5cbfda5eb78791fd04637c5637ccb167a44cc450'
for x in [j['installedTreePins'],j['declaredDependencyResolution'],j['sourceFreeze'],j['newPublicSourceFreeze'],j['actualBeforeSource'],j['releaseConditions'],j['actualInputFormat']]:check(x)
t=load(j['installedTreePins']['path']);flat=t['files']+[t['node'],t['cli']]+j['sourceFiles']
assert len(flat)==4682 and len(t['symlinks'])==3 and j['pinArithmetic']=={'packageFiles':4670,'NodeCLI':2,'sourceFiles':10,'symlinks':3,'totalPrePost':4685}
op=j['operations'][sel]
assert op['wallSeconds']==60 and op['keptBytesPerStream']==4194304 and op['maxStdoutBytes']==4194304 and op['maxStderrBytes']==4194304
assert op['argv'][1]=='--max-old-space-size=512'
C=A/'root-actual-before-runtime-captures01'
itfdir=A/'proposed-itf'
controls=[pin(prop),pin(A/'TEN-PUBLIC-RUNTIME-SOURCE-FREEZE.json'),pin(V/'FINAL-RESPONSE.txt'),pin(V/'RESULT.json'),pin(helper),pin(__file__),pin(j['actualBeforeSource']['path']),pin(j['releaseConditions']['path']),pin(j['actualInputFormat']['path']),pin(j['newPublicSourceFreeze']['path'])]
def checks():
 for x in controls:check(x)
 a=api['static_check'](flat)
 a['symlinks']=[{'path':x['path'],'match':Path(x['path']).is_symlink() and os.readlink(x['path'])==x['target'] and str(Path(x['path']).resolve(strict=True))==x['resolved'] and Path(x['path']).is_dir()==x['isDirectory']}for x in t['symlinks']]
 e=list((A/'source').iterdir())
 a['exactTenRegularEntries']=sorted(p.name for p in e)==sorted(j['sourceExactEntries']) and all(p.is_file() and not p.is_symlink() for p in e)
 a['rawChecks']=len(flat)+len(t['symlinks'])
 a['all_match']=a['all_match'] and all(x['match'] for x in a['symlinks']) and a['exactTenRegularEntries'] and a['rawChecks']==4685
 return a
def durable(p,x):
 assert not Path(p).exists();api['save'](p,x);fsync_file(p);fsync_dir(Path(p).parent)
if sel==0:
 assert op['id']=='01-newActualBeforeRuntimeFrontend' and op['expected']=={'exit':0,'stdoutBytes':0,'stderrBytes':0}
 assert not C.exists() and not itfdir.exists()
else:
 assert op['id']=='02-oneActualBeforeWindowComputation' and op['itfTotalBytesCap']==67108864
 assert not itfdir.exists(), 'fresh proposed-itf required; no replacement/overwrite'
 prev=C/'01-newActualBeforeRuntimeFrontend'/'ROOT-ACCEPTANCE.json'
 assert prev.is_file() and not prev.is_symlink()
 pa=load(prev);assert pa['accepted'] is True and pa['operation']=='01-newActualBeforeRuntimeFrontend'
 controls.append(pin(prev))
before=checks();assert before['all_match']
C.mkdir(exist_ok=(sel==1))
if sel==1:fsync_dir(A)
phase=C/op['id'];phase.mkdir(exist_ok=False)
durable(phase/'ROOT-SELECTION.json',{'selected':True,'operation':op,'singleExplicitRootSelection':True,'review':pin(V/'FINAL-RESPONSE.txt'),'reviewResult':pin(V/'RESULT.json'),'caller':pin(__file__),'noAutomaticNextDispatch':True,'noRetry':True})
durable(phase/'ROOT-PRECHECK.json',before)
if sel==1:
 itfdir.mkdir(exist_ok=False);fsync_dir(A)
raw_durable=False
try:
 a=api['capture'](op,phase)
 durable(phase/'RESULT.json',a)
 # Freeze every raw byte before any assertion, parsing or classification.
 for n in ('COMMAND.json','stdout.bin','stderr.bin'):
  p=phase/n
  if p.is_file() and not p.is_symlink():fsync_file(p)
 fsync_dir(phase)
 itfs=[]
 if sel==1 and itfdir.is_dir():
  for p in list(itfdir.iterdir()):
   if p.is_file() and not p.is_symlink():fsync_file(p)
  fsync_dir(itfdir);itfs=[pin(p) for p in sorted(itfdir.iterdir(),key=lambda q:q.name) if p.is_file() and not p.is_symlink()]
  if itfs:durable(phase/'ITF-RAW-PIN.json',{'files':itfs,'itfTotalBytesCap':67108864,'frozenBeforeParse':True})
 fsync_dir(phase);fsync_dir(C);raw_durable=True
 after=checks();durable(phase/'ROOT-POSTCHECK.json',after)
 if sel==1:
  entries=list(itfdir.iterdir());assert len(entries)==1, 'exactly one ITF output required'
  itf=entries[0];assert itf.is_file() and not itf.is_symlink() and itf.name.startswith('actual_owner_window_') and itf.name.endswith('.itf.json')
  assert 0<itf.stat().st_size<=op['itfTotalBytesCap'] and itfs==[pin(itf)]
 assert a['actual_exit']==0 and a['controller_failure'] is None and after['all_match']
 assert len(a['streams'])==2 and all(s['full_stream_retained'] and s['bytes']==s['bytes_seen']==s['bytes_retained'] for s in a['streams'])
 if sel==0:
  assert [s['bytes'] for s in a['streams']]==[0,0]
  assert (phase/'stdout.bin').read_bytes()==b'' and (phase/'stderr.bin').read_bytes()==b''
  result={'accepted':True,'operation':op['id'],'actual':a,'checksEachPrePost':4685,'exactTenRegularSources':True,'frontendOnly':True,'runNotDispatched':True}
 else:
  assert (phase/'stderr.bin').read_bytes()==b'', 'run stderr diagnostics'
  assert not any(t in (phase/'stdout.bin').read_bytes() for t in (b'QNT0',b'DEBUG',b'error:')), 'run stdout diagnostics'
  assert b'diagnostic' not in Path(itfs[0]['path']).read_bytes().lower(), 'ITF diagnostics'
  raw=itfs[0];trace=json.loads(Path(raw['path']).read_bytes())
  assert trace['vars']==['machine'], 'exactly sole machine variable'
  states=trace['states'];assert len(states)==2, 'exactly two complete ITF states'
  ms=[]
  for s in states:
   assert set(s)-{'#meta'}=={'machine'}, 'no hidden model state'
   m=s['machine'];assert set(m)=={'input_before','computed_window','computed_once'};ms.append(m)
  tag=lambda v:v.get('tag') if isinstance(v,dict) else v
  val=lambda v:v.get('value') if isinstance(v,dict) else None
  assert ms[0]['computed_once'] is False and tag(ms[0]['computed_window'])=='Q41NoWindow' and 'input_before' in ms[0]
  assert ms[1]['computed_once'] is True and tag(ms[1]['computed_window'])=='Q41ComputedWindow'
  w=val(ms[1]['computed_window']);assert isinstance(w,dict) and set(w)=={'before','after_cell','after_exit','events'}
  assert isinstance(w['after_cell'],dict) and isinstance(w['after_exit'],dict)
  assert ms[1]['input_before']==ms[0]['input_before'] and w['before']==ms[1]['input_before']
  ev=w['events'];assert isinstance(ev,list) and len(ev)==2 and all(isinstance(e,dict) and len(e)==28 for e in ev), 'ordered 2 full 28-field events'
  result={'accepted':True,'operation':op['id'],'actual':a,'checksEachPrePost':4685,'exactTenRegularSources':True,'itfRaw':raw,'itfStates':2,'vars':['machine'],'wholeWindowRetainedDerived':True,'witnessDerivation':'complete raw ITF state plus frozen q41ActualOwnerWindowRuntime wholeWindowRetained definition','printedWitnessCounterClaimed':False,'rawStreamTruncationClaimed':False,'exit0AloneNotSufficient':True,'noExpectedAfterRustComparison':True,'scope':'raw newly computed Quint output collection/qualification only'}
 durable(phase/'ROOT-ACCEPTANCE.json',result)
 print(json.dumps({'accepted':True,'id':op['id'],'seconds':a['elapsed_seconds'],'streams':[x['bytes'] for x in a['streams']],'checks':4685}),flush=True)
except BaseException as e:
 if phase.exists() and not (phase/'FAILURE.json').exists():
  durable(phase/'FAILURE.json',{'status':'FAIL_STOP_NO_RETRY','error':type(e).__name__+': '+str(e),'rawPreserved':raw_durable})
 raise
