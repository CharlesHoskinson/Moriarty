from pathlib import Path
import json,hashlib,runpy,os,sys,glob
R=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017')
B=R/'k-C6-EnvGetCell-EnvGetExit-owner-positive-20261020/conditional-positive-transition-source01'
V=B/'fresh-source-API-exact1-review-preparation02-full-production-context/actual-MuseWonderHIGH-source-or-review-01'
load=lambda p:json.loads(Path(p).read_bytes())
def pin(p):
 p=Path(p);v=p.read_bytes();return dict(path=str(p),bytes=len(v),sha256=hashlib.sha256(v).hexdigest())
def check(x):
 a=pin(x['path']);assert a['bytes']==x['bytes']and a['sha256']==x['sha256'];return a
proposal=B/'PROPOSED-EXACT1-NEW-ENVGET-COMPILE.json'
assert pin(proposal)['sha256']=='600b0578f1f25bf48431366d7b32b0b7f4edd0ba34654ad2b66fa991e811e80d'
j=load(proposal);assert len(j['phases'])==1;op=j['phases'][0]
rv=load(V/'RESULT.json');assert rv['actualExit']==0 and rv['terminal']['terminal']=='completed'and rv['finalSHA']=='6e71059b8fa82365eafb5d7df82d43177f3c58777a8d1d8f7246436771c6031d'
f=load(B/'root-exact1-compile-composition-preparation01/CURRENT-COMPLETE-PUBLIC-COMPILE-INPUT-DATA.json');tools=load(j['tools']['path']);closure=load(j['K_builtin_closure']['path'])
flat={}
def walk(x):
 if isinstance(x,dict):
  if {'path','bytes','sha256'}<=x.keys():
   row={k:x[k]for k in ['path','bytes','sha256']};assert row['path']not in flat or flat[row['path']]==row;flat[row['path']]=row
  for v in x.values():walk(v)
 elif isinstance(x,list):
  for v in x:walk(v)
for x in [j,f,tools,closure]:walk(x)
for p in [proposal,B/'SOURCE-FREEZE.json',V/'FINAL-RESPONSE.txt',V/'RESULT.json',V/'COMMAND.json',Path(__file__)]:walk(pin(p))
rows=sorted(flat.values(),key=lambda x:x['path'])
def prepost():
 for x in rows:check(x)
 assert sorted(glob.glob(tools['classpath_pattern']))==sorted(x['path']for x in tools['classpath_jar_membership'])
 for x in tools['entries']:assert str(Path(x['path']).resolve(strict=True))==x['resolved_path']
 return {'all_match':True,'rows':len(rows),'classpathJars':len(tools['classpath_jar_membership'])}
before=prepost()
check(j['callback']['fullsource'])
api=runpy.run_path(j['callback']['fullsource']['path'],run_name='root_conditional_ENVGET_F03_launch_only')
assert api['CHILD_SECONDS']==30 and api['SWEEP_SECONDS']==3
actual=B/'actual';assert not actual.exists()
phase=Path(op['cwd']);phase.mkdir(parents=True,mode=0o700)
for p in [phase,Path(op['environment']['HOME']),Path(op['environment']['TMPDIR'])]:
 if not p.exists():p.mkdir(mode=0o700)
 os.chmod(p,0o700)
def durable(p,x):
 assert not p.exists()
 with p.open('xb')as f:f.write((json.dumps(x,indent=2)+'\n').encode());f.flush();os.fsync(f.fileno())
 fd=os.open(p.parent,os.O_DIRECTORY|os.O_RDONLY);os.fsync(fd);os.close(fd)
durable(actual/'ROOT-SELECTION.json',{'selected':True,'RootFullSourceClosureAndReviewRead':True,'exact1CompileOnly':True,'review':pin(V/'FINAL-RESPONSE.txt'),'proposal':pin(proposal),'composition':pin(__file__),'noUnitsOrRuntimeOrExpectedInputs':True,'noRetryNoCapChange':True})
durable(actual/'FLAT-INPUT-PINS.json',{'members':rows});durable(phase/'ROOT-PRECHECK.json',before)
durable(phase/'COMMAND.json',op)
record={}
try:
 api['launch'](record,op['argv'],dict(op['environment']),op['cwd'],op['environment']['SHERLOCK_K_ATTEMPT_TOKEN'],536870912)
 durable(phase/'RESULT.json',record)
 for n in ['stdout','stderr','RESULT.json','COMMAND.json']:
  with(phase/n).open('rb')as z:os.fsync(z.fileno())
 fd=os.open(phase,os.O_DIRECTORY|os.O_RDONLY);os.fsync(fd);os.close(fd)
 post=prepost();durable(phase/'ROOT-POSTCHECK.json',post)
 artifacts=[pin(p)for p in sorted(actual.rglob('*'))if p.is_file()and p not in [actual/'FLAT-INPUT-PINS.json']]
 products=[pin(p)for p in sorted((actual/'envget-positive-kompiled').rglob('*'))if p.is_file()]
 errors=[pin(p)for p in phase.glob('hs_err_pid*.log')]
 durable(actual/'ROOT-ALL-ACTUAL-PRODUCTS-FROZEN.json',{'products':products,'rawFiles':artifacts,'jvmErrorFiles':errors})
 ok=record.get('returncode')==0 and record.get('outcome')=='exited'and record.get('timed_out')is False and record.get('containment_breach')is False and not record.get('remaining')and not errors and all(Path(p).is_file()for p in op['products'])
 durable(actual/'ROOT-COMPILE-RESULT.json',{'cleanProcessAndProducts':ok,'source':pin(B/'SOURCE-FREEZE.json'),'record':record,'products':products,'streams':[pin(phase/'stdout'),pin(phase/'stderr')],'fullStreamsRootReadOwed':True,'warningsUnadjudicated':True,'labelsUnqualified':True,'runtimeUnrun':True,'rows':len(rows)})
 print(json.dumps({'cleanProcessAndProducts':ok,'record':record,'streams':[pin(phase/'stdout'),pin(phase/'stderr')],'products':len(products),'rows':len(rows)}),flush=True)
except BaseException as e:
 if not(phase/'RESULT.json').exists():durable(phase/'RESULT.json',record)
 durable(actual/'ROOT-FAILURE.json',{'status':'FAIL_STOP_NO_RETRY','error':type(e).__name__+': '+str(e),'record':record});raise
