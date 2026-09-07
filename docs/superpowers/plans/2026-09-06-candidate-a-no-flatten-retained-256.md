# Candidate A retained256 data continuation plan

Status: source-only exact plan. No code block in this plan is executed while authoring or reviewing it. Use the writing-plans/executing-plans workflow only within the separate root authority described here. No executable helper is materialized and no compiler is rerun.

Design: `docs/superpowers/specs/2026-09-06-candidate-a-no-flatten-retained-256-design.md`, SHA256 `dbd209064ac9066925ceb3c469b4648e90f2f6292af943518beb67d7a10784f2`.

ROOT is `/home/charl/Moriarty/.worktrees/s01-audit-start`; S is ROOT + `/.superpowers/sdd`; OLD is S + `/a5-no-flatten-diagnostic-20260906`; NEW is S + `/a5-no-flatten-retained-256-20260906`. The only parsed compiler input remains S + `/a4-producer-receipts/a5-noflat-full-20260906/stdout.bin`,199868611 bytes, SHA256 `ebd75615ca4f89ba5cb47c7a535206e990ef9162d53530b35f7c3a95b691cb8d`. Exact old source/index/admission pins are in the code below, not inferred from current HEAD.

Every Python command, including initial root adoption and slot recording, uses the original full dispatch's exact parentRemoved/parentFixed environment values, then the pinned `/home/charl/Moriarty/.venv/bin/python -B`. The dispatch JSON is the immutable original SHA256 `878870cf1c9ea1cc3dcbe64e37c002011081e93a7adc494c4e2565fd09d98fba`. Construct the prefix as `['env', *[x for key in parentRemoved for x in ['-u',key]], *[key+'='+value for key,value in parentFixed.items()]]`; append the finite step assignment and actual Python argv, then use shlex.join. Do not inherit an unchecked Python startup/optimization environment. No version probe or compiler is used to construct that prefix.

## Decisions, path ownership and finite order

A nonauthor reviews this plan and its design at S + `/a5-no-flatten-retained-256-independent-review-20260906.md`, beginning `PASS\n` only on actual acceptance. Root records an exclusive S + `/a5-no-flatten-retained-256-root-adoption-20260906.json` with exactly `{decision:"adopt retained256 preparation only",plan:{path,bytes,sha256},design:{path,bytes,sha256},review:{path,bytes,sha256},dataInvocations:1,nativeCompilerAuthorized:false}`. Root creates this from the three actual completed files after reading the review. This decision authorizes preparation only; root's later slot and dispatch authorize the one parser.

The new result is NEW + `/retained-intake.json`. The original OLD/full-intake.json and OLD/root-admission-full.json remain failed structural gates. NEW/outer is created only by unchanged RH002. NEW/parser-limits.json is created only by the data child. Parent/child pycache prefixes NEW/parent-cache and NEW/child-cache must be unused; Python uses -B. NEW/before and NEW/after contain root-owned input observations, not copies of the raw compiler output. NEW/transport owns every actual parser response. NEW/data-transport owns actual preparation/check command receipts, including failures. Root owns NEW/root-slot.json, dispatch.json, dispatch-source.tar.gz, dispatch-seal.json, preservation.json and final-admission.json. Source support stays in NEW/before/source.tar.gz and its exact member index. Nothing is added to a completed before/dispatch archive.

Exactly these steps are permitted, in order:

1. Root adoption, then `NF256_STEP=initialize` using the metadata driver below.
2. Root captures its actual process listing and memory metadata in the new observations directory, inspects them and creates root-slot.json using the exact slot recipe below. At least12GiB MemAvailable and no overlapping native/memory-intensive parser work are required. If later native work intervenes, stop for an explicit source-reviewed predecessor extension.
3. `NF256_STEP=before`, then `NF256_STEP=dispatch`, then `NF256_STEP=prelaunch`. Each must have actual exit0; retain its real args/response before continuing.
4. One tools.exec_command of the exact prelaunch-returned native-free RH002 command. Save every returned response before polling again. No timeout, resource failure or transport failure permits a retry.
5. After authentic terminal and settled owned group only: `NF256_STEP=after`, then `NF256_STEP=preserve`. These commands never parse compiler stdout. A parser result or missing result is retained under authentic exits.
6. Nonauthor result review at NEW/independent-review.md, then root `NF256_STEP=admit`. A failed data invocation can receive failed preservation, never a structural success. Original128-MiB failure and all previous native limits remain unchanged.

The fixed data limits are256MiB input,8GiB virtual address space,90/95s CPU,16MiB per-file writes, core0 and120s outer wall. No parser retry or further ceiling increase belongs to this plan. No parsing by a reviewer outside this owned resource budget is implied.

## Exact source-only root adoption and slot recipes

After actual independent review, root runs this inline metadata command under `/home/charl/Moriarty/.venv/bin/python -B -`, optimization0. It creates no parser or native process:

```python
import sys
if sys.flags.optimize:raise RuntimeError('optimization zero')
import hashlib,json
from pathlib import Path
R=Path('/home/charl/Moriarty/.worktrees/s01-audit-start');S=R/'.superpowers/sdd'
P=R/'docs/superpowers/plans/2026-09-06-candidate-a-no-flatten-retained-256.md';G=R/'docs/superpowers/specs/2026-09-06-candidate-a-no-flatten-retained-256-design.md';V=S/'a5-no-flatten-retained-256-independent-review-20260906.md'
def pin(p):return {'path':str(p),'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
if not V.read_text().startswith('PASS\n'):raise RuntimeError('real independent plan/design review')
if pin(G)['sha256']!='dbd209064ac9066925ceb3c469b4648e90f2f6292af943518beb67d7a10784f2':raise RuntimeError('design pin')
value={'decision':'adopt retained256 preparation only','plan':pin(P),'design':pin(G),'review':pin(V),'dataInvocations':1,'nativeCompilerAuthorized':False}
p=S/'a5-no-flatten-retained-256-root-adoption-20260906.json'
with p.open('x') as f:json.dump(value,f,indent=2,sort_keys=True);f.write('\n')
print(json.dumps({'adoption':pin(p),'parserExecuted':False}))
```

After initialize, root runs these exact observation commands with ROOT cwd, preserves their actual tool responses outside the observations, and personally inspects the process listing:

```bash
bash -c 'set -C; exec ps -eo pid,ppid,pgid,sid,stat,lstart,args > /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-no-flatten-retained-256-20260906/observations/processes.txt'
```

```bash
bash -c 'set -C; exec cat /proc/meminfo > /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-no-flatten-retained-256-20260906/observations/meminfo.txt'
```

Then root executes this one slot writer, under the same pinned Python -B optimization0. It binds the completed original full terminal and all predecessor PGIDs from its root slot. The writer is a root decision after process inspection, not an automatic worker grant:

```python
import sys
if sys.flags.optimize:raise RuntimeError('optimization zero')
import datetime,hashlib,json,os,re
from pathlib import Path
R=Path('/home/charl/Moriarty/.worktrees/s01-audit-start');S=R/'.superpowers/sdd';O=S/'a5-no-flatten-diagnostic-20260906';N=S/'a5-no-flatten-retained-256-20260906'
def read(p):return json.loads(p.read_text())
def pin(p):return {'path':str(p),'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
t=read(O/'full-outer/terminal.json');tr=read(O/'transport/full/terminal.json');oldslot=read(S/'a5-no-flatten-slot-full-20260906.json')
if t['cleanup']!={'complete':True,'errors':[],'forced':False,'signals':[]} or t['return_code']!=tr['actualOuterExit'] or tr['responses'][-1]['exit_code']!=0:raise RuntimeError('completed original full')
pgids=sorted(set([t['owned_pgid'],*oldslot['ownedPgids']]))
for pgid in pgids:
 if type(pgid) is not int or pgid<=0:raise RuntimeError('positive PGID')
 try:os.killpg(pgid,0)
 except ProcessLookupError:continue
 raise RuntimeError('predecessor group remains')
ps=N/'observations/processes.txt';mem=N/'observations/meminfo.txt'
available=re.findall(r'^MemAvailable:\s+([0-9]+) kB$',mem.read_text(),re.M)
if len(available)!=1 or int(available[0])<12*1024*1024 or ps.stat().st_size==0:raise RuntimeError('fresh root-inspected scheduling metadata')
files=[O/'full-outer/terminal.json',O/'transport/full/terminal.json',O/'root-admission-full.json',S/'a5-no-flatten-slot-full-20260906.json',*[Path(r['path']) for r in oldslot['terminalBindings']],ps,mem]
value={'authorizedDataInvocations':1,'nativeCompilerAuthorized':False,'schedulerExclusive':True,'ownedPgids':pgids,'files':[pin(p) for p in sorted(set(files))],'memAvailableKiB':int(available[0]),'observedAt':datetime.datetime.now(datetime.timezone.utc).isoformat()}
p=N/'root-slot.json'
with p.open('x') as f:json.dump(value,f,indent=2,sort_keys=True);f.write('\n')
print(json.dumps({'slot':pin(p),'parserExecuted':False}))
```

## Exact bounded parser source, extracted in memory

This block is the only new data-child source. It is not a new recorder. Its three counted replacement sites change the128→256 constant, the matching error label, and the result-write block (which inserts retained provenance and selects the fresh result path). Every other original full NF005 byte and predicate is unchanged. Memory/resource failure is not converted into a passing result.

The constructed full NF005 body is211 lines, SHA256 `5e43888ccff0ce5a0ea8f04e3a20513a8505db675ab27f6005da818e8416cf3a`. Both metadata and bounded-parser construction must independently reproduce it from the original210-line body; no hand-edited replacement body is admitted.

<!-- bounded-parser -->
```python
import sys
if sys.flags.optimize or not sys.dont_write_bytecode:raise RuntimeError('optimization zero and -B required')
import ast,hashlib,json,os,re,resource
from pathlib import Path
R=Path('/home/charl/Moriarty/.worktrees/s01-audit-start');S=R/'.superpowers/sdd';N=S/'a5-no-flatten-retained-256-20260906'
P=R/'docs/superpowers/plans/2026-09-06-candidate-a-no-flatten-diagnostic.md'
def need(ok,msg):
 if not ok:raise RuntimeError(msg)
need(os.environ.get('NF_MODE')=='full','original full predicates only')
need(sys.pycache_prefix==str(N/'child-cache'),'exact unused data child cache')
settings=[('addressSpace',resource.RLIMIT_AS,(8*1024**3,8*1024**3)),('cpuSeconds',resource.RLIMIT_CPU,(90,95)),('fileSize',resource.RLIMIT_FSIZE,(16*1024**2,16*1024**2)),('coreSize',resource.RLIMIT_CORE,(0,0))]
for name,key,value in settings:resource.setrlimit(key,value);need(resource.getrlimit(key)==value,'exact OS resource limit '+name)
def pin(p):
 p=Path(p);need(p.is_file() and not p.is_symlink(),'regular original');h=hashlib.sha256();size=0
 with p.open('rb') as f:
  while b:=f.read(1048576):h.update(b);size+=len(b)
 return {'path':str(p),'bytes':size,'sha256':h.hexdigest()}
def read(p):
 p=Path(p);need(p.stat().st_size<=16*1024**2,'bounded metadata');return json.loads(p.read_text())
def check(row):need(pin(row['path'])==row,'exact pinned parser support')
d=read(N/'dispatch.json');before=read(N/'before/inputs.json')
need(d['dataInvocations']==1 and d['nativeCompilerAuthorized'] is False,'actual one-data dispatch')
for row in d['support']:check(row)
check(d['pythonExecutable']);need(Path(sys.executable).resolve()==Path(d['pythonExecutable']['path']),'actual selected pinned parser interpreter')
need(pin(P)['sha256']=='79325d985b5f58a62a5f490b0ac51f60c873afcb36775be3d95e372d144b2260','original plan')
original=re.findall(r'^```python\n(.*?)^```$',P.read_text(),re.S|re.M)
need(len(original)==7 and hashlib.sha256(original[3].encode()).hexdigest()=='6801bed3dc7b9cad0a0ee495f45512a6eea7d9168ef2f088db73d9901ce8004a','exact original NF005')
body=original[3]
replacements=[('LIMIT=128*1024*1024','LIMIT=256*1024*1024'),('compiler JSON exceeds128 MiB intake bound','compiler JSON exceeds256 MiB intake bound'),("path=M/(mode+'-intake.json')","result['retainedDataContinuation']=retained_provenance\npath=Path('/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-no-flatten-retained-256-20260906/retained-intake.json')")]
for old,new in replacements:need(body.count(old)==1,'unique reviewed substitution');body=body.replace(old,new,1)
need(hashlib.sha256(body.encode()).hexdigest()==d['constructedBodySha256'],'exact constructed data body')
need(pin(S/'a4-producer-receipts/a5-noflat-full-20260906/stdout.bin')==d['compilerOutput'],'same complete retained input')
need(d['compilerOutput']['bytes']==199868611 and d['compilerOutput']['bytes']<=256*1024**2,'fixed reviewed input size')
limits={'pid':os.getpid(),'pgid':os.getpgrp(),'sid':os.getsid(0),'executable':sys.executable,'origArgv':sys.orig_argv,'optimize':sys.flags.optimize,'dontWriteBytecode':sys.dont_write_bytecode,'pycachePrefix':sys.pycache_prefix,'limits':{name:list(resource.getrlimit(key)) for name,key,value in settings},'compilerOutput':d['compilerOutput'],'constructedBodySha256':d['constructedBodySha256'],'nativeCompilerExecuted':False}
lp=N/'parser-limits.json'
with lp.open('x') as f:json.dump(limits,f,indent=2,sort_keys=True);f.write('\n')
provenance={'originalFailedIntake':d['originalFailedIntake'],'originalFailedAdmission':d['originalFailedAdmission'],'originalPlanSha256':pin(P)['sha256'],'dispatch':pin(N/'dispatch.json'),'limitsReceipt':pin(lp),'inputLimitBytes':256*1024**2,'originalInputLimitBytes':128*1024**2,'constructedBodySha256':d['constructedBodySha256'],'nativeRerun':False,'originalFailedGateRetained':True,'scope':'separate bounded structural inspection of one retained altered-command output; no H1/compiler/solver acceptance'}
ast.parse(body)
try:
 exec(compile(body,'<retained-original-full-NF005-256>','exec'),{'__name__':'__main__','retained_provenance':provenance})
finally:
 for row in d['support']:check(row)
 need(pin(Path(d['compilerOutput']['path']))==d['compilerOutput'],'retained compiler bytes unchanged after data attempt')
```

If MemoryError, CPU signal, timeout or other interruption prevents writing a complete intake, the absence/partial file remains an original failed data result. RH002's actual time-wrapped exit and return_code are distinct; do not infer a Python exit from return_code2/124 alone. No catch fabricates a valid data receipt. The final support/hash checks cannot convert a failed child into success; a failed final check must block adoption even if an earlier intake file says capability true.

## Root metadata driver: exact source/input closure and dispatch

This finite data-only block implements initialize/before/dispatch/prelaunch/after/preserve/admit. Execute it from this plan in memory with `NF256_STEP` set to exactly the intended step, under pinned Python -B optimization0 and ROOT cwd. It never calls the bounded parser directly, never JSON-loads compiler stdout and never imports a project/helper. It writes only its step-owned fresh paths. Its original index pins1,090 originals and8,102 runtime files; all are streamed, including the retained raw output and external runtime archives. No large file is copied into the new source archive.

<!-- metadata-driver -->
```python
import sys
if sys.flags.optimize or not sys.dont_write_bytecode:raise RuntimeError('optimization zero and -B required')
import ast,datetime,hashlib,io,json,os,re,shlex,tarfile
from pathlib import Path
R=Path('/home/charl/Moriarty/.worktrees/s01-audit-start');S=R/'.superpowers/sdd';N=S/'a5-no-flatten-retained-256-20260906';O=S/'a5-no-flatten-diagnostic-20260906'
P=R/'docs/superpowers/plans/2026-09-06-candidate-a-no-flatten-retained-256.md';G=R/'docs/superpowers/specs/2026-09-06-candidate-a-no-flatten-retained-256-design.md';A=S/'a5-no-flatten-retained-256-root-adoption-20260906.json'
OLDPLAN=R/'docs/superpowers/plans/2026-09-06-candidate-a-no-flatten-diagnostic.md';IX=S/'a5-no-flatten-full-independent-index-20260906.json';RH=R/'evidence/s02-candidate-a-completion/a4/native-resources/runner.py';PY='/home/charl/Moriarty/.venv/bin/python'
step=os.environ['NF256_STEP']
def need(ok,msg):
 if not ok:raise RuntimeError(msg)
need(step in ['initialize','before','dispatch','prelaunch','after','preserve','admit'],'finite metadata step')
def pin(p):
 p=Path(p);need(p.is_file() and not p.is_symlink(),'regular original '+str(p));h=hashlib.sha256();size=0
 with p.open('rb') as f:
  while b:=f.read(1048576):h.update(b);size+=len(b)
 return {'path':str(p),'bytes':size,'sha256':h.hexdigest()}
def read(p):
 p=Path(p);need(p.stat().st_size<=16*1024**2,'metadata bound; never parse compiler stdout');return json.loads(p.read_text())
def check(row):need(pin(row['path'])==row,'changed original '+row['path'])
def put(p,v):
 with p.open('x') as f:json.dump(v,f,indent=2,sort_keys=True);f.write('\n')
def absent(n):
 need(type(n) is int and n>0,'owned positive PGID')
 try:os.killpg(n,0)
 except ProcessLookupError:return
 raise RuntimeError('owned group remains')
ad=read(A);need(ad['decision']=='adopt retained256 preparation only' and ad['dataInvocations']==1 and ad['nativeCompilerAuthorized'] is False,'separate root preparation adoption')
for row in [ad['plan'],ad['design'],ad['review']]:check(row)
need(ad['plan']==pin(P) and ad['design']==pin(G) and Path(ad['review']['path']).read_text().startswith('PASS\n'),'actual adopted plan/design/review')
known={G:'dbd209064ac9066925ceb3c469b4648e90f2f6292af943518beb67d7a10784f2',OLDPLAN:'79325d985b5f58a62a5f490b0ac51f60c873afcb36775be3d95e372d144b2260',IX:'c5050c72c3b5f41267fb8102fcc2e66624d5adbbc007d24050948875801875c4',O/'full-intake.json':'66a4b45f3681592d755c4f85e20a055aa60c142a9f67c6958e86dd1301405f46',O/'root-admission-full.json':'a1e566d6737e18ea6c47249fa7c3bb8ead2c5becd64c16455a47c33b91f2f6b8',S/'a5-no-flatten-full-independent-review-20260906.md':'c61259897e4aef3f17b16781c913f6d67de6de6eeaaf7c3aad72b99f6f75d0f3',S/'a5-no-flatten-full-independent-tool-receipt-20260906.json':'bd7cb7fc77494853434b6761ad57a3a78b07ff3fd0cd1022f7d43f3ad2246c38',RH:'d8973d3541269d1be2ce524f2482e5f5b858f15f7d9117e3fe6d5cd2171d660d'}
for p,h in known.items():need(pin(p)['sha256']==h,'frozen support')
old=read(O/'full-intake.json');failed=read(O/'root-admission-full.json');index=read(IX)
need(old['capabilityOrFullSuccess'] is False and old['validationErrors']==[{'message':'compiler JSON exceeds128 MiB intake bound','type':'RuntimeError'}] and failed['verdict']=='PASS failed diagnostic preservation only' and failed['capabilityOrFullSuccess'] is False,'original failed gate stays false')
need(len(index['originals'])==1090 and len(index['runtimeFiles'])==8102 and index['compilerJSONParsed'] is False,'independent finite original inventory')
native_receipt=read(S/'a4-producer-receipts/a5-noflat-full-20260906/receipt.json');python_ref=native_receipt['tools']['python']
need(Path(PY).resolve()==Path(sys.executable).resolve()==Path(python_ref['path']),'same selected admitted Python executable')
python_executable=pin(Path(PY).resolve());need(python_executable==index['runtimeFiles'][python_ref['path']] and python_executable['sha256']==python_ref['sha256'],'actual interpreter byte pin')
need(pin(Path('/usr/bin/time'))==index['originals']['/usr/bin/time'] and pin(Path('/usr/bin/time'))['sha256']=='34ba90f8199989d88f2f02d40ddcee60be27a7f361db9fb7c100578cc8d7bf1e','unchanged admitted GNU time')
raw=index['compilerOutput'];need(raw['bytes']==199868611 and raw['sha256']=='ebd75615ca4f89ba5cb47c7a535206e990ef9162d53530b35f7c3a95b691cb8d','exact retained output')
if step=='initialize':
 N.mkdir(exist_ok=False)
 for name in ['observations','data-transport']:(N/name).mkdir()
 put(N/'initialization.json',{'adoption':pin(A),'originalIndex':pin(IX),'nativeCompilerExecuted':False})
 print(json.dumps({'ok':True,'initialized':pin(N/'initialization.json')}));raise SystemExit(0)
slot=read(N/'root-slot.json');need(slot['authorizedDataInvocations']==1 and slot['nativeCompilerAuthorized'] is False and slot['schedulerExclusive'] is True,'root data slot')
for row in slot['files']:check(row)
for n in slot['ownedPgids']:absent(n)
support_paths=set(known)|{P,A,Path(ad['review']['path']),N/'root-slot.json',N/'initialization.json'}|{Path(r['path']) for r in slot['files']}
dt=S/'a5-no-flatten-full-supplement-transport-20260906'
support_paths|={dt/(name+'.json') for name in ['admit-command','admit-response-000','admit-terminal']}
admit_tool=read(dt/'admit-terminal.json');need(admit_tool['actualExit']==admit_tool['responses'][-1]['exit_code']==0 and admit_tool['responses'][-1]['chunk_id']=='49554c','actual original full admission terminal')
need(json.loads(admit_tool['responses'][-1]['output'])['admission']==pin(O/'root-admission-full.json'),'actual original failed admission binding')
support=[pin(p) for p in sorted(support_paths)]
def input_inventory():
 inputs={}
 for rows in [index['originals'],index['runtimeFiles']]:
  for p,row in rows.items():need(p==row['path'] and (p not in inputs or inputs[p]==row),'unique original/runtime pin');inputs[p]=row
 for row in support:
  need(row['path'] not in inputs or inputs[row['path']]==row,'support overlap');inputs[row['path']]=row
 for row in inputs.values():check(row)
 manifest=read(O/'full-before/endpoint.json')['runtime']['manifest'];trees={}
 for root,expected in manifest['treeMembers'].items():
  paths=[]
  for p in Path(root).rglob('*'):
   need(not(p.is_symlink() and p.is_dir()),'no runtime directory alias')
   if p.is_file():paths.append(str(p.resolve()))
  trees[root]=sorted(set(paths));need(trees[root]==expected,'exact runtime tree membership')
 return {'files':{p:inputs[p] for p in sorted(inputs)},'treeMembers':trees,'compilerOutput':raw,'support':support}
if step in ['before','after']:
 if step=='before':
  need(all(not (N/n).exists() for n in ['before','after','outer','retained-intake.json','parser-limits.json','dispatch.json','dispatch-source.tar.gz','dispatch-seal.json','transport','parent-cache','child-cache']),'fresh finite data paths')
 else:
  t=read(N/'outer/terminal.json');tr=read(N/'transport/terminal.json')
  need(type(tr['responses'][-1].get('exit_code')) is int and t['return_code']==tr['actualOuterExit']==tr['responses'][-1]['exit_code'],'authentic data outer terminal')
  need(t['cleanup']['complete'] is True and t['cleanup']['errors']==[],'complete data cleanup')
  if t['owned_pgid'] is not None:absent(t['owned_pgid'])
  else:need(t['launch_error'] is not None,'no PID only on actual launch failure')
 dest=N/step;dest.mkdir(exist_ok=False);inventory=input_inventory()
 if step=='after':need(inventory==read(N/'before/inputs.json'),'identical before/after source/runtime/input observations')
 put(dest/'inputs.json',inventory)
 if step=='before':
  rows=[]
  with tarfile.open(dest/'source.tar.gz','x:gz') as tar:
   for row in support:
    check(row);b=Path(row['path']).read_bytes();need(len(b)<=16*1024**2,'small source/support only');name=row['path'].lstrip('/');member=tarfile.TarInfo(name);member.size=len(b);member.mode=0o644;tar.addfile(member,io.BytesIO(b));rows.append({**row,'archivePath':name})
  put(dest/'source-index.json',{'archive':pin(dest/'source.tar.gz'),'members':rows,'bulkInputPolicy':'old originals/runtime/output external and bound by inputs.json; not copied'})
 print(json.dumps({'ok':True,'step':step,'inputs':pin(dest/'inputs.json'),'parserExecuted':False}));raise SystemExit(0)
before=read(N/'before/inputs.json');need(before['support']==support,'frozen parser support')
for row in support:check(row)
sourceindex=read(N/'before/source-index.json');check(sourceindex['archive']);need(sourceindex['members']==[{**row,'archivePath':row['path'].lstrip('/')} for row in support],'exact source archive member index')
with tarfile.open(sourceindex['archive']['path'],'r:gz') as tar:
 ms=tar.getmembers();need(len(ms)==len(support) and {m.name for m in ms}=={r['archivePath'] for r in sourceindex['members']} and all(m.isfile() for m in ms),'exact archived support membership')
 by={r['archivePath']:r for r in sourceindex['members']}
 for m in ms:need(hashlib.sha256(tar.extractfile(m).read()).hexdigest()==by[m.name]['sha256'],'archived support bytes')
def constructed():
 bodies=re.findall(r'^```python\n(.*?)^```$',OLDPLAN.read_text(),re.S|re.M);need(len(bodies)==7,'original blocks');body=bodies[3]
 need(hashlib.sha256(body.encode()).hexdigest()=='6801bed3dc7b9cad0a0ee495f45512a6eea7d9168ef2f088db73d9901ce8004a','original full NF005 body')
 for old,new in [('LIMIT=128*1024*1024','LIMIT=256*1024*1024'),('compiler JSON exceeds128 MiB intake bound','compiler JSON exceeds256 MiB intake bound'),("path=M/(mode+'-intake.json')","result['retainedDataContinuation']=retained_provenance\npath=Path('/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-no-flatten-retained-256-20260906/retained-intake.json')")]:
  need(body.count(old)==1,'exact reviewed substitution');body=body.replace(old,new,1)
 ast.parse(body);return hashlib.sha256(body.encode()).hexdigest()
if step=='dispatch':
 need(all(not (N/n).exists() for n in ['dispatch.json','dispatch-source.tar.gz','dispatch-seal.json','outer','retained-intake.json','parser-limits.json','parent-cache','child-cache','transport']),'unused data dispatch')
 bootstrap='from pathlib import Path\nimport hashlib,re\np=Path('+repr(str(P))+')\ns=p.read_bytes()\nif hashlib.sha256(s).hexdigest()!='+repr(pin(P)['sha256'])+':raise RuntimeError("frozen parser plan")\nb=re.findall(r"^<!-- bounded-parser -->\\n```python\\n(.*?)^```$",s.decode(),re.S|re.M)\nif len(b)!=1:raise RuntimeError("one bounded parser source")\nexec(compile(b[0],"<bounded-retained-parser>","exec"))\n'
 ast.parse(bootstrap)
 child=[PY,'-B','-X','pycache_prefix='+str(N/'child-cache'),'-c',bootstrap]
 parent=[PY,'-B','-X','pycache_prefix='+str(N/'parent-cache'),str(RH),'--receipt-dir',str(N/'outer'),'--inner-receipt',str(N/'retained-intake.json'),'--wall-seconds','120','--cwd',str(R),'--',*child]
 original=read(O/'root-dispatch-full.json');env=['env',*[x for key in original['parentRemoved'] for x in ['-u',key]],*[k+'='+v for k,v in original['parentFixed'].items()],'NF_MODE=full']
 dispatch={'dataInvocations':1,'nativeCompilerAuthorized':False,'support':support,'pythonExecutable':python_executable,'beforeInputs':pin(N/'before/inputs.json'),'sourceArchive':sourceindex['archive'],'sourceIndex':pin(N/'before/source-index.json'),'rootSlot':pin(N/'root-slot.json'),'constructedBodySha256':constructed(),'compilerOutput':raw,'originalFailedIntake':pin(O/'full-intake.json'),'originalFailedAdmission':pin(O/'root-admission-full.json'),'childArgv':child,'rh002Argv':parent,'shellCommand':shlex.join(env+parent),'wallSeconds':120,'limits':{'addressSpace':[8*1024**3]*2,'cpuSeconds':[90,95],'fileSize':[16*1024**2]*2,'coreSize':[0,0]},'inputLimitBytes':256*1024**2}
 put(N/'dispatch.json',dispatch);b=(N/'dispatch.json').read_bytes()
 with tarfile.open(N/'dispatch-source.tar.gz','x:gz') as tar:
  item=tarfile.TarInfo(str(N/'dispatch.json').lstrip('/'));item.size=len(b);item.mode=0o644;tar.addfile(item,io.BytesIO(b))
 put(N/'dispatch-seal.json',{k:pin(N/path) for k,path in [('dispatch','dispatch.json'),('archive','dispatch-source.tar.gz'),('beforeInputs','before/inputs.json'),('sourceIndex','before/source-index.json')]})
 print(json.dumps({'ok':True,'dispatch':pin(N/'dispatch.json'),'seal':pin(N/'dispatch-seal.json')}));raise SystemExit(0)
d=read(N/'dispatch.json');seal=read(N/'dispatch-seal.json')
for row in seal.values():check(row)
need(d['support']==support and d['constructedBodySha256']==constructed() and d['compilerOutput']==raw and d['wallSeconds']==120,'exact data dispatch')
with tarfile.open(seal['archive']['path'],'r:gz') as tar:
 ms=tar.getmembers();need(len(ms)==1 and ms[0].isfile() and ms[0].name==str(N/'dispatch.json').lstrip('/') and tar.extractfile(ms[0]).read()==(N/'dispatch.json').read_bytes(),'exact acyclic dispatch archive')
if step=='prelaunch':
 need(all(not (N/n).exists() for n in ['outer','retained-intake.json','parser-limits.json','transport','parent-cache','child-cache']),'one unused parser invocation')
 check(raw)
 available=re.findall(r'^MemAvailable:\s+([0-9]+) kB$',Path('/proc/meminfo').read_text(),re.M);need(len(available)==1 and int(available[0])>=12*1024*1024,'current data memory scheduling gate')
 print(json.dumps({'ok':True,'memAvailableKiB':int(available[0]),'dispatch':pin(N/'dispatch.json'),'args':{'cmd':d['shellCommand'],'workdir':str(R),'yield_time_ms':1000,'max_output_tokens':2500}}));raise SystemExit(0)
# Preserve/admit only after authentic new data terminal and settled ownership.
t=read(N/'outer/terminal.json');tr=read(N/'transport/terminal.json')
need(type(tr['responses'][-1].get('exit_code')) is int and tr['actualOuterExit']==tr['responses'][-1]['exit_code']==t['return_code'],'actual parser outer terminal')
need(t['cleanup']['complete'] is True and t['cleanup']['errors']==[],'complete owned data cleanup')
if t['owned_pgid'] is not None:absent(t['owned_pgid'])
else:need(t['launch_error'] is not None,'genuine launch failure')
need(read(N/'after/inputs.json')==before,'complete unchanged original source/runtime/input observations')
responses=sorted((N/'transport').glob('response-*.json'));need([p.name for p in responses]==['response-%03d.json'%i for i in range(len(responses))] and [read(p) for p in responses]==tr['responses'],'all actual data responses')
for row in tr['responses'][:-1]:need('exit_code' not in row and row['session_id']==tr['responses'][0]['session_id'],'same original data session')
pre=read(N/'transport/prelaunch.json');need(pre['exit_code']==0 and json.loads(pre['output'])['args']==tr['args'] and tr['args']==read(N/'transport/command.json')['args'] and tr['args']['cmd']==d['shellCommand'],'actual prelaunch/data command')
launch=read(N/'outer/launch.json');need(launch['child_argv']==d['childArgv'] and launch['original_parent_argv']==d['rh002Argv'] and launch['wrapper_argv']==d['rh002Argv'][4:] and launch['argv']==['/usr/bin/time','-v','-o',str(N/'outer/resources.txt'),*d['childArgv']],'actual data argv')
need(launch['inner_receipt']==str(N/'retained-intake.json') and launch['cwd']==str(R) and launch['wall_seconds']==t['wall_seconds']==120 and launch['cleanup_seconds']==launch['grace_seconds']==5,'fixed data ownership/bounds')
need(launch['source_runtime_before']==t['source_runtime_before']==t['source_runtime_after'] and t['source_runtime_stable'] is True,'data capture helper/runtime unchanged')
for p,h in t['sidecar_sha256'].items():need(pin(p)['sha256']==h,'actual data sidecar bytes')
artifacts=[pin(p) for base in [N/'outer',N/'transport',N/'after'] for p in sorted(base.rglob('*')) if p.is_file()]
for p in [N/'parser-limits.json',N/'retained-intake.json']:
 if p.exists():artifacts.append(pin(p))
result={'preservation':True,'structuralPredicatesPassed':False,'actualDataOuterExit':tr['actualOuterExit'],'actualTimeWrappedExit':t['actual_exit'],'resultPresent':(N/'retained-intake.json').is_file(),'validationErrors':[],'artifacts':artifacts,'originalFailedGateRetained':True,'compilerAcceptance':False,'H1':'unresolved','nativeRerun':False,'beforeInputs':pin(N/'before/inputs.json'),'afterInputs':pin(N/'after/inputs.json'),'dispatchSeal':pin(N/'dispatch-seal.json')}
try:
 need(t['eligible'] is True and t['timed_out'] is False and t['cleanup']['forced'] is False and t['actual_exit']==t['return_code']==tr['actualOuterExit']==0,'positive original data process/resource eligibility')
 need(t['inner_receipt_present'] is True and t['inner_receipt_complete'] is False and t['inner_receipt_independent_validation_required'] is True and t['resource_complete'] is True and t['resource']['exit_status']==0 and t['resource']['diagnostics']==[],'genuine separate data result required')
 limits=read(N/'parser-limits.json');need(limits['limits']==d['limits'] and limits['pgid']==limits['sid']==t['owned_pgid'] and limits['optimize']==0 and limits['dontWriteBytecode'] is True and limits['pycachePrefix']==str(N/'child-cache') and limits['origArgv']==d['childArgv'],'actual preparse resource limits/flags/group')
 fresh=read(N/'retained-intake.json');need(pin(N/'retained-intake.json')['sha256']==t['inner_receipt_sha256'],'authentic actual data result hash')
 need(fresh['capabilityOrFullSuccess'] is True and fresh['validationErrors']==[] and fresh['compilerAcceptance'] is False and fresh['H1']=='unresolved','all original full structural predicates passed')
 changed={k for k in set(fresh)|set(old) if fresh.get(k)!=old.get(k)}
 need(changed=={'capabilityOrFullSuccess','validationErrors','compilerJSON','serializedMain','originalImports','declarations','generatedSelections','retainedDataContinuation'},'only genuine new structural fields and provenance differ')
 need(fresh['compilerJSON']==raw and fresh['serializedMain']=='candidate_a_funding_pilot' and fresh['originalImports']==13 and fresh['declarations']==40 and fresh['generatedSelections']==['q::init','q::step','q::inv'],'exact recorded full structure')
 prov=fresh['retainedDataContinuation'];need(prov['dispatch']==pin(N/'dispatch.json') and prov['limitsReceipt']==pin(N/'parser-limits.json') and prov['constructedBodySha256']==d['constructedBodySha256'] and prov['nativeRerun'] is False and prov['originalFailedGateRetained'] is True and prov['inputLimitBytes']==256*1024**2 and prov['originalInputLimitBytes']==128*1024**2,'separate bounded retained provenance')
 need((N/'outer/stderr.bin').stat().st_size==0,'clean successful Python data stderr')
 need(read(N/'outer/stdout.bin')=={'preservation':True,'capabilityOrFullSuccess':True,'intake':pin(N/'retained-intake.json'),'mode':'full','H1':'unresolved','compilerAcceptance':False},'authentic original NF005 data summary')
 result['structuralPredicatesPassed']=True
except (OSError,ValueError,RuntimeError,KeyError,TypeError) as error:result['validationErrors'].append({'type':type(error).__name__,'message':str(error)})
if step=='preserve':
 put(N/'preservation.json',result);print(json.dumps({'preservation':pin(N/'preservation.json'),'structuralPredicatesPassed':result['structuralPredicatesPassed'],'nativeCompilerExecuted':False}));raise SystemExit(0)
need(step=='admit' and result==read(N/'preservation.json'),'exact already reviewed preservation')
review=N/'independent-review.md';need(review.read_text().startswith('PASS\n'),'real nonauthor result review')
put(N/'final-admission.json',{'verdict':'PASS retained altered-command structure only' if result['structuralPredicatesPassed'] else 'PASS failed bounded-data preservation only','review':pin(review),'preservation':pin(N/'preservation.json'),'retainedStructureAdmitted':result['structuralPredicatesPassed'],'originalFailedGateRetained':True,'compilerAcceptance':False,'H1':'unresolved','nativeRerunAuthorized':False,'dataRetryAuthorized':False})
print(json.dumps({'admission':pin(N/'final-admission.json'),'nativeCompilerExecuted':False}))
```

The final report's structuralPredicatesPassed flag is a candidate for independent/root data admission, not compiler acceptance. The independent reviewer checks the new raw resources.txt against original RH002 fields and reads every actual data response, resource-limit receipt, derived body/source pins, before/after hashes and genuine new result. For failure, record the actual time-wrapped process exit and outer normalization separately; a missing result supplies no completed structural predicate. Keep all failure stdout/stderr and partial outputs. A further raw JSON parse by a reviewer needs its own explicit bounded decision.

## Exact command extraction and original transport

For metadata steps use a sanitized parent environment identical to original OLD/root-dispatch-full.json's parentRemoved/parentFixed, then `NF256_STEP=STEP /home/charl/Moriarty/.venv/bin/python -B -c BOOTSTRAP`. BOOTSTRAP is exactly this fixed source. It reads the actual root decision's expected plan pin before execution; use shlex.join or structured argv to preserve newlines:

```python
from pathlib import Path
import hashlib,json,re
p=Path('/home/charl/Moriarty/.worktrees/s01-audit-start/docs/superpowers/plans/2026-09-06-candidate-a-no-flatten-retained-256.md')
a=json.loads(Path('/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-no-flatten-retained-256-root-adoption-20260906.json').read_text())
if a['decision']!='adopt retained256 preparation only' or a['plan']['path']!=str(p):raise RuntimeError('actual root preparation adoption')
s=p.read_bytes()
if hashlib.sha256(s).hexdigest()!=a['plan']['sha256']:raise RuntimeError('adopted plan changed')
blocks=re.findall(r'^<!-- metadata-driver -->\n```python\n(.*?)^```$',s.decode(),re.S|re.M)
if len(blocks)!=1:raise RuntimeError('one finite metadata driver')
exec(compile(blocks[0],'<retained256-metadata>','exec'))
```

The expected digest comes from the explicit actual root decision, never from an arbitrary current plan. Root preserves actual argv/response for each data command under NEW/data-transport/STEP-{command,response-NNN,terminal}.json; before initialize, preserve the actual adoption/initialize tools at the sibling S/a5-no-flatten-retained-256-initial-tools-20260906.json, created exclusively after both actual terminals. During any yielded initialization preserve each actual response in a separate exclusive sibling progress file before another poll; do not wait until completion to retain a long-running original response. No successful response or missing terminal is reconstructed. Actual preparation source, reviewed source and terminal evidence remain distinct.

Store the prelaunch command's exact returned tool object with `store('retained256-prelaunch', response)`. Then run exactly this one finite native-free transport body. It invokes unchanged RH002 around the bounded Python child, not a compiler:

```javascript
const pre=load('retained256-prelaunch');
if(!pre || pre.exit_code!==0)throw Error('actual successful prelaunch required');
const launch=JSON.parse(pre.output),root='/home/charl/Moriarty/.worktrees/s01-audit-start';
if(launch.ok!==true)throw Error('prelaunch failure');
const folder=root+'/.superpowers/sdd/a5-no-flatten-retained-256-20260906/transport';
const q=s=>"'"+String(s).replaceAll("'","'\\''")+"'";
const py='/home/charl/Moriarty/.venv/bin/python';
const removed=['PYTHONPATH','PYTHONHOME','PYTHONSTARTUP','PYTHONINSPECT','PYTHONOPTIMIZE','PYTEST_ADDOPTS','PYTEST_PLUGINS','NODE_PATH','NODE_OPTIONS','NODE_COMPILE_CACHE','LD_PRELOAD','LD_LIBRARY_PATH','JAVA_TOOL_OPTIONS','_JAVA_OPTIONS','JDK_JAVA_OPTIONS'];
const fixed=['PYTHONOPTIMIZE=0','PYTHONNOUSERSITE=1','PYTHONDONTWRITEBYTECODE=1','LC_ALL=C','PATH=/usr/lib/jvm/java-25-openjdk-amd64/bin:/home/charl/.foreman/tools/fnm/node-versions/v24.18.1/installation/bin:/home/charl/.npm-global/bin:/usr/bin:/bin','JVM_ARGS=-Xmx4096m','JVM_GC_ARGS=-XX:+UseG1GC -XX:G1PeriodicGCInterval=600000 -XX:+G1PeriodicGCInvokesConcurrent','APALACHE_JAR=/home/charl/.quint/apalache-dist-0.56.1/apalache/lib/apalache.jar'];
const prefix=['env',...removed.flatMap(k=>['-u',k]),...fixed,py,'-B'].map(q).join(' ');
const make=await tools.exec_command({cmd:prefix+' -c '+q('from pathlib import Path; import sys; Path(sys.argv[1]).mkdir(exist_ok=False)')+' '+q(folder),workdir:root,max_output_tokens:1000});
text(make);if(make.exit_code!==0)throw Error('transport creation failed; no parser launch');
async function save(name,value){
 const body='from pathlib import Path; import sys; f=Path(sys.argv[1]).open("x"); f.write(sys.argv[2]); f.close()';
 const r=await tools.exec_command({cmd:prefix+' -c '+q(body)+' '+q(folder+'/'+name)+' '+q(JSON.stringify(value,null,2)+'\n'),workdir:root,max_output_tokens:1000});
 if(r.exit_code!==0){text({saveFailure:r,unsavedOriginal:value});throw Error('root takeover, no relaunch');}
}
await save('prelaunch.json',pre);await save('command.json',launch);
let r=await tools.exec_command(launch.args);const responses=[r];await save('response-000.json',r);
while(r.session_id!==undefined && r.exit_code===undefined){
 r=await tools.write_stdin({session_id:r.session_id,chars:'',yield_time_ms:10000,max_output_tokens:2500});responses.push(r);
 await save('response-'+String(responses.length-1).padStart(3,'0')+'.json',r);
 if(responses.length%6===0)notify({returnedResponses:responses.length,terminal:false});
}
if(!Number.isInteger(r.exit_code))throw Error('unknown actual terminal; root takeover');
await save('terminal.json',{args:launch.args,responses,actualOuterExit:r.exit_code,scope:'original bounded-data RH002 tool transport, not native compiler acceptance'});
text(r);
```

Save failures expose the actual returned error and unsaved original to root. RH002 has no parent signal handler and exposes its PGID in terminal.json only at completion. Do not terminate a monitor or parent for progress. Root resolves any uncertain descendants after transport/host loss before after hashing; no inferred cleanup or parser retry follows.

## Static review and stop conditions

Source-only review checks the exact design and original pin tables, original210-line NF005 hash, counted substitutions/body equality, all resource settings, genuine new result path, unchanged original full predicates, source/input archive ordering and actual transport handling. Syntax-compile Python blocks only; do not import helpers or execute a driver/parser. The input is still not parsed at this review stage.

Stop before parser dispatch on failed support/index/runtime/raw pin, root review/slot failure, insufficient current scheduling memory, changed source, used destination or uncertain prior group. Stop the route after any parser attempt, whether it succeeds, exceeds address-space/CPU/wall budget, fails JSON/schema checks or loses transport. Preserve failure as failure. Successful data predicates require separate nonauthor/root admission and leave old128-MiB gate false.

No native compiler run, source/model/helper/runtime edit, solver/correspondence/H1/causal conclusion, cleanup redesign or commit is included. The only intended new knowledge is whether one exact retained stream meets the already reviewed altered-command structural predicates under an explicit finite data budget.
