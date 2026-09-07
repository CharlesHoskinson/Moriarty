# Candidate A unused-helper compilation-view execution plan

**Goal:** Test whether symmetric omission of13 unused swap declarations makes the unchanged funding pilot's default flattened compilation feasible, then preserve the original paired-size/offline/H1 gates.
**Architecture:** Fresh24-file copied view; derive existing prepare.py and record.py by counted changes. Add one finite offline caller using the unchanged record(). No compiler/runtime patch or primary-model edit.
**Toolchain:** Existing pinned Quint0.32.0, Rust evaluator, Apalache0.56.1, Java25, Node24 and Python; original30 derivation pins.

Status: source-only exact plan. Design: docs/superpowers/specs/2026-09-06-candidate-a-unused-helper-compilation-view-design.md. Root and nonauthor must review design, plan and actual derived sources before preparation/native dispatch. This document is a proposed finite remedy, not evidence that compilation will succeed.

ROOT=/home/charl/Moriarty/.worktrees/s01-audit-start. CONTROL=ROOT/.superpowers/sdd/a5-factoring-receipts/pilot-unused-helper-view. Every output below is fresh and exclusive. The old pilot-compilation-view and all failed/default/no-flatten/retained-data attempts remain immutable. The only model writes are CONTROL/view's24 new files. The old30 source freeze,24 original closure and two old AuthorityKey-import additions remain exact.

The omitted names are preparePairS, pipelineS, fundingRouteS, routeS, noPendingS, financialTerminalS, nonCommitS, PrefixS, executeListS, prefixS, expectedFinalLedgerS, staleScenarioS and staleRouteS. Exactly3,835 bytes per swap copy are deleted at independently pinned original spans. Only CommandS/evidenceS/canCommandS/applyCommandS/safetyS remain as swap declarations. Every other retained byte and import route stays unchanged.

## UV001: Source derivation and preparation

Use the following source-only derivation after design/plan adoption. It prints the exact three proposed source files as JSON without materializing or importing them. Preserve its actual tool result, create exactly those bytes with apply_patch under absent CONTROL, and independently compare each SHA/byte string back to this derivation before preparation. Nothing is extracted from an executable archive.

```python
import ast,hashlib,json
from pathlib import Path
R=Path('/home/charl/Moriarty/.worktrees/s01-audit-start')
P=R/'docs/superpowers/plans/2026-09-06-candidate-a-unused-helper-compilation-view.md'
O=R/'.superpowers/sdd/a5-factoring-receipts/pilot-compilation-view'
N=R/'.superpowers/sdd/a5-factoring-receipts/pilot-unused-helper-view'
def replace(s,a,b,n=1):
    if s.count(a)!=n:raise RuntimeError('counted source delta '+a)
    return s.replace(a,b)
def block(label):
    import re
    f=chr(96)*3
    matches=re.findall(r'^<!-- '+label+r' -->\n'+f+r'python\n(.*?)^'+f+'$',P.read_text(),re.M|re.S)
    if len(matches)!=1:raise RuntimeError('one exact plan source '+label)
    return matches[0]
prepare=(O/'prepare.py').read_text();record=(O/'record.py').read_text()
assert hashlib.sha256(prepare.encode()).hexdigest()=='6521f780d2c6a07e16501bfaab9810262bd0c73a82e6dbc36d7fb0ad64a1fc8f'
assert hashlib.sha256(record.encode()).hexdigest()=='03e456a65942714082ce16e66725d0db16d830576e33e4ef164a046213748511'
prepare=replace(prepare,"PLAN = ROOT / 'docs/superpowers/plans/2026-09-06-candidate-a-pilot-compilation-view.md'","PLAN = ROOT / 'docs/superpowers/plans/2026-09-06-candidate-a-unused-helper-compilation-view.md'")
prepare=replace(prepare,'import sys\n','import sys\nassert not sys.flags.optimize\n')
prepare=replace(prepare,"if __name__ == '__main__':\n",block('prepare-extension')+"\nif __name__ == '__main__':\n")
record=replace(record,'sha, write, validate\n','sha, write, validate, stage_authority\n')
record=replace(record,'assert sys.dont_write_bytecode and sys.pycache_prefix','assert not sys.flags.optimize and sys.dont_write_bytecode and sys.pycache_prefix')
record=replace(record,'manifest = validate()\n','authority = stage_authority(stage)\nmanifest = validate()\n')
record=replace(record,"extras = [ROOT / name for name in d['sources']] + [dispatch_path]","extras = [ROOT / name for name in d['sources']] + [dispatch_path] + authority")
record=replace(record,"extras += [ROOT / row['path'] for row in manifest['references']]","extras += [ROOT / row['path'] for row in manifest['references'] if not row['path'].endswith('.tar.gz')]")
record=replace(record,"extras += sorted(p for p in parent.iterdir() if p.is_file())","extras += sorted(p for p in parent.iterdir() if p.is_file() and not p.name.endswith(('.tar.gz','.qnt.json')))")
record=replace(record,"record('pilot-compilation-view/' + stage","record('pilot-unused-helper-view/' + stage")
record=replace(record,"'two direct AuthorityKey imports only; actualDispatchBase='","'two direct AuthorityKey imports plus symmetric13-unused-helper copied omission; actualDispatchBase='")
record=replace(record,"        declarations = {item['name']: item for item in modules[0]['declarations']}\n","        raw_declarations = modules[0]['declarations']\n        declarations = {item['name']: item for item in raw_declarations}\n        checks['distinctGeneratedDeclarationNames'] = len(raw_declarations) == len(declarations)\n")
files={'prepare.py':prepare,'record.py':record,'offline.py':block('offline-source')}
for name,source in files.items():ast.parse(source)
assert not N.exists()
print(json.dumps({'target':str(N),'files':files,'sha256':{n:hashlib.sha256(s.encode()).hexdigest() for n,s in files.items()},'materialized':False},indent=2))
```

Exact preparation extension inserted before the existing __main__ block:
<!-- prepare-extension -->
```python
# Finite copied-source omission; every original remains immutable.
INVENTORY = ROOT / '.superpowers/sdd/a5-post-noflat-source-opportunities-20260906.json'
assert sha(INVENTORY) == '8c6b3f1f91da2ff9784e461b305710386c19be272315ed1f8b8a3c74acbbed7a'
OPPORTUNITY = json.loads(INVENTORY.read_text())
GROUPS = {g['module']: g for g in OPPORTUNITY['candidateGroups']}
assert len(GROUPS) == 2
OMITTED = ['preparePairS','pipelineS','fundingRouteS','routeS','noPendingS','financialTerminalS','nonCommitS','PrefixS','executeListS','prefixS','expectedFinalLedgerS','staleScenarioS','staleRouteS']
RETAINED = {'CommandS','evidenceS','canCommandS','applyCommandS','safetyS'}
_base_changed = changed
_base_expected_manifest = expected_manifest
_base_validate = validate
def removed_spans(relative, data):
    group = GROUPS[relative]
    assert group['candidateBytes'] == 3835 and group['totalDeclarations'] == 18
    assert set(group['retainedDeclarations']) == RETAINED
    assert [r['name'] for r in group['removedCandidates']] == OMITTED
    lines = data.splitlines(keepends=True)
    spans = []
    for row in group['removedCandidates']:
        start = sum(map(len, lines[:row['firstLine']-1]))
        end = start + row['bytes']
        chunk = data[start:end]
        assert hashlib.sha256(chunk).hexdigest() == row['sha256']
        assert re.match(rb'  (?:pure (?:def|val)|type) '+row['name'].encode()+rb'\b', chunk)
        spans.append((start,end,chunk))
    assert all(a[1] <= b[0] for a,b in zip(spans,spans[1:]))
    return spans
def changed(name, data):
    relative = 'specs/quint/s02/' + name
    expected_pin = OPPORTUNITY['pins'][str(BASE/name)]
    assert sha(BASE/name) == expected_pin['sha256'] and len(data) == expected_pin['bytes']
    if relative not in GROUPS:
        return _base_changed(name,data)
    spans = removed_spans(relative,data)
    cursor=0;pieces=[]
    for start,end,chunk in spans:
        pieces.append(data[cursor:start]);cursor=end
    pieces.append(data[cursor:]);result=b''.join(pieces)
    assert len(data)-len(result)==3835
    restored=bytearray(result)
    for start,end,chunk in spans:restored[start:start]=chunk
    assert bytes(restored)==data
    assert {m[1].decode() for m in re.finditer(rb'^  (?:pure (?:def|val)|type) (\w+)',result,re.M)}==RETAINED
    return result
def expected_manifest():
    result=_base_expected_manifest()
    result['schema']='moriarty.a5-unused-helper-compilation-view/v1'
    result['omissionInventory']={'path':str(INVENTORY.relative_to(ROOT)),'sha256':sha(INVENTORY)}
    result['omissions']=OPPORTUNITY['candidateGroups']
    return result
def validate():
    manifest=_base_validate()
    groups=[]
    for relative in GROUPS:
        p=ROOT/relative;groups.append([chunk for start,end,chunk in removed_spans(relative,p.read_bytes())])
    assert groups[0]==groups[1]
    for row in manifest['files']:
        data=(ROOT/row['view']).read_bytes()
        assert not any(re.search(rb'\b'+name.encode()+rb'\b',data) for name in OMITTED)
        name=str((ROOT/row['original']).relative_to(BASE))
        assert imports(ROOT/row['view'])==(([ADDITIONS[name].split(' from "')[1].split('"')[0]] if name in ADDITIONS else [])+row['originalImports'])
    assert sum(row['originalBytes']-row['viewBytes'] for row in manifest['files'] if row['original'] in GROUPS)==7670
    return manifest

STAGE_ORDER=['typecheck','prefix-test','sample-original','sample-factored','compile-original','compile-factored','check-original','check-factored','compile-negative','check-negative']
def stage_authority(stage):
    assert stage in STAGE_ORDER
    p=CONTROL/('allow-'+stage+'.json')
    a=json.loads(p.read_text())
    assert a['stage']==stage and a['nativeInvocations']==1 and a['dispatchSha256']==sha(CONTROL/'dispatch.json')
    assert a['schedulerExclusive'] is True and a['sourceFreezeHeld'] is True
    for row in a['files']:
        assert sha(ROOT/row['path'])==row['sha256']
    for pgid in a['ownedPgids']:
        assert type(pgid) is int and pgid>0
        try:__import__('os').killpg(pgid,0)
        except ProcessLookupError:continue
        raise RuntimeError('previous owned process group remains')
    previous=STAGE_ORDER[:STAGE_ORDER.index(stage)]
    assert a['predecessors']==previous
    for name in previous:
        out=CONTROL/name
        value=json.loads((out/'view-validation.json').read_text())
        assert (out/'outer-command.json').is_file()
        if name=='check-original' and not value['ok']:
            assert value['preservation'] is True and a['baselineResourceFailureAdmitted'] is True
        else:assert value['ok'] is True
    assert not (CONTROL/stage).exists()
    return [p]+[ROOT/row['path'] for row in a['files'] if not row['path'].endswith(('.tar.gz','.qnt.json'))]
```

The exact new offline caller:
<!-- offline-source -->
```python
import json,re,subprocess,sys
from pathlib import Path
from prepare import ROOT,CONTROL,VIEW,PLAN,BOOTSTRAP,HELPER_SHA,sha,write,validate,stage_authority
assert not sys.flags.optimize and sys.dont_write_bytecode and sys.pycache_prefix
sys.path.insert(0,str(ROOT))
from scripts.run_s02_candidate_a_factoring_pilot import record,QUINT,AP
assert len(sys.argv)==2 and sys.argv[1] in ['check-original','check-factored','compile-negative','check-negative']
stage=sys.argv[1];authority=stage_authority(stage)
d=json.loads((CONTROL/'dispatch.json').read_text())
assert d['runtimeBootstrapBase']==BOOTSTRAP and d['runtimeHelperSha256']==HELPER_SHA
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip()==d['actualDispatchBase']
assert d['planSha256']==sha(PLAN)
assert all(sha(ROOT/name)==digest for name,digest in d['sources'].items())
manifest=validate();assert sha(CONTROL/'view-manifest.json')==d['manifestSha256']
negative=stage.endswith('-negative');compiling=stage=='compile-negative'
prop='neverPrepared' if negative else 'pilotSafety'
stem='candidate_a_funding_pilot'+('' if stage=='check-original' else '_f')
base=VIEW/'specs/quint/s02/factored_verification'
extras=[ROOT/name for name in d['sources']]+[CONTROL/'dispatch.json']+authority
extras += [ROOT/row['path'] for row in manifest['references'] if not row['path'].endswith('.tar.gz')]
if compiling:
    argv=[str(QUINT),'compile',str(base/(stem+'.qnt')),'--main='+stem,'--target=json','--invariant=neverPrepared','--verbosity=0']
else:
    source=CONTROL/('compile-negative' if negative else 'compile-original' if stage=='check-original' else 'compile-factored')
    previous=json.loads((source/'result.json').read_text())
    assert previous['exitCode']==0 and previous['timedOut'] is False and previous['sourceAndToolsUnchanged'] is True and 'measurementError' not in previous
    assert json.loads((source/'view-validation.json').read_text())['ok'] is True
    metadata=json.loads((source/'input.json').read_text())
    assert all(sha(row['path'])==row['sha256'] for row in metadata['pins'])
    inp=source/'input.qnt.json';assert sha(inp)==previous['stdoutSha256']
    extras += [inp,source/'input.json',source/'result.json',source/'view-validation.json',source/'outer-command.json']
    argv=[str(AP),'--out-dir='+str(CONTROL/stage/'apalache'),'check','--init=q::init','--next=q::step','--inv=q::inv','--length=5','--no-deadlock',str(inp)]
automatic={Path(row['path']) for row in json.loads((ROOT/'.superpowers/sdd/a5-factoring-receipts/compile-original-pilotSafety/input.json').read_text())['pins']}
extras=tuple(p for p in dict.fromkeys(extras) if p not in automatic)
code=record('pilot-unused-helper-view/'+stage,argv,900 if compiling else 600,4096,prop,5,compile_output=compiling,extra=extras,before_dispatch_base=BOOTSTRAP,
    domain='Unchanged four-route/22-prefix Candidate A pilot; symmetric13-unused-swap-helper copied omission; actualDispatchBase='+d['actualDispatchBase'])
out=CONTROL/stage;result=json.loads((out/'result.json').read_text())
preservation=result['runtimeUnchanged'] is True and result['sourceAndToolsUnchanged'] is True
try:preservation=preservation and validate()==manifest
except (AssertionError,OSError,ValueError):preservation=False
expected=12 if stage=='check-negative' else 0
checks={'expectedNativeExit':code==expected,'noTimeout':result['timedOut'] is False,'stablePins':preservation}
if compiling:
    checks['emptyCompileStderr']=(out/'stderr.txt').stat().st_size==0
    checks['noMeasurementError']='measurementError' not in result
    try:
        document=json.loads((out/'input.qnt.json').read_text());modules=document['modules']
        decls=modules[0]['declarations'];names=[x['name'] for x in decls]
        checks['generatedMain']=len(modules)==1 and modules[0]['name']==document['main']==stem
        checks['distinctNames']=len(names)==len(set(names))
        checks['generatedBindings']=all(x in names for x in ['init','step','neverPrepared','q::init','q::step','q::inv'])
        checks['generatedMeasurements']=result['generated']['bytes']==(out/'input.qnt.json').stat().st_size>0
    except (ValueError,KeyError,IndexError,TypeError):checks['validGeneratedJSON']=False
ok=all(checks.values())
write(out/'view-validation.json',{'ok':ok,'preservation':preservation,'checks':checks,'actualCommandExitCode':code,'actualDispatchBase':d['actualDispatchBase'],
    'dispatchSha256':sha(CONTROL/'dispatch.json'),'property':prop,'parentArgv':sys.orig_argv,'parentBytecodeDisabled':sys.dont_write_bytecode,'parentCachePrefix':sys.pycache_prefix,
    'classification':'Mechanical process/input gate only. Root must inspect NoError/states0-5 or genuine state1 counterexample before semantic admission; H1 not automatically concluded.'})
raise SystemExit(0 if ok else 1)
```

## UV002: Exact environment, preparation and freeze

Every Python command (including derivation, prepare, freeze, admission and handoff) must run with optimization0 and -B. Use this exact sanitized prefix for parent commands; it also ensures the helper's import-time which java/node resolves the admitted executables. Existing record() still sets its original child heap/JVM/cache controls.

```sh
env -u PYTHONPATH -u PYTHONHOME -u PYTHONSTARTUP -u PYTHONINSPECT -u PYTHONOPTIMIZE -u PYTEST_ADDOPTS -u PYTEST_PLUGINS -u NODE_PATH -u NODE_OPTIONS -u NODE_COMPILE_CACHE -u LD_PRELOAD -u LD_LIBRARY_PATH -u JAVA_TOOL_OPTIONS -u _JAVA_OPTIONS -u JDK_JAVA_OPTIONS -u FORCE_COLOR -u NO_COLOR PYTHONOPTIMIZE=0 PYTHONNOUSERSITE=1 PYTHONDONTWRITEBYTECODE=1 LC_ALL=C PATH=/usr/lib/jvm/java-25-openjdk-amd64/bin:/home/charl/.foreman/tools/fnm/node-versions/v24.18.1/installation/bin:/home/charl/.npm-global/bin:/usr/bin:/bin JVM_ARGS=-Xmx4096m 'JVM_GC_ARGS=-XX:+UseG1GC -XX:G1PeriodicGCInterval=600000 -XX:+G1PeriodicGCInvokesConcurrent' APALACHE_JAR=/home/charl/.quint/apalache-dist-0.56.1/apalache/lib/apalache.jar
```

Append /home/charl/Moriarty/.venv/bin/python -B to this prefix and the exact arguments below. Prefix tokens and Python/code arguments must be joined with shlex.join or shell single-quote escaping. Workdir is ROOT. Do not inherit a persistent shell's environment.

Run prepare.py once with -X pycache_prefix=CONTROL/prepare-parent-cache. Preserve actual preparation args and all returned tool responses as CONTROL/prepare-outer-command.json. Expected:24 files, two added imports, unchanged30 original pins, exact26 omissions totaling7,670 bytes, manifest validation true. Source checks include restoration of every deleted span, exact18→5 declaration inventory per copy, paired deleted-body byte equality and no removed identifier in any transformed file. The native typecheck below is a separate actual name/type-binding gate.

Root reviews actual diffs, complete manifest, source pins and all import destinations before freezing. Root settles the current A4 implementation first, records current HEAD, and holds those source bytes through the entire finite stage series. No concurrent source mutation or experimental commit during a dispatched stage; if the base changes between phases, stop for reviewed source/dispatch correction, never silently rewrite dispatch. Historical bootstrap900bb2051225b4a3d99bf422c3b2e5e386e3e7bc remains historical, not the actual source HEAD.

After source review root writes exclusive .superpowers/sdd/a5-unused-helper-view-root-adoption-20260906.json with decision="adopt finite unused-helper view preparation", nativeStages=10, and plan/design/review fields each {path relative to ROOT,sha256} binding the completed actual files. The review is .superpowers/sdd/a5-unused-helper-view-independent-review-20260906.md and begins PASS followed by newline only on actual acceptance. No native stage follows from preparation alone; each concrete root allow below remains required.

Run this exact freeze source under the sanitized Python prefix and -X pycache_prefix=CONTROL/freeze-parent-cache. Preserve the actual argv/response as dispatch-outer-command.json. It creates no native process.

```python
import json,subprocess,sys
from pathlib import Path
assert not sys.flags.optimize and sys.dont_write_bytecode
root=Path('/home/charl/Moriarty/.worktrees/s01-audit-start')
c=root/'.superpowers/sdd/a5-factoring-receipts/pilot-unused-helper-view'
sys.path.insert(0,str(c))
from prepare import PLAN,BOOTSTRAP,HELPER_SHA,INVENTORY,sha,write,validate
design=root/'docs/superpowers/specs/2026-09-06-candidate-a-unused-helper-compilation-view-design.md'
review=root/'.superpowers/sdd/a5-unused-helper-view-independent-review-20260906.md'
adoption=root/'.superpowers/sdd/a5-unused-helper-view-root-adoption-20260906.json'
brief=root/'.superpowers/sdd/a5-post-noflat-next-route-source-brief-20260906.md'
old=root/'.superpowers/sdd/a5-factoring-receipts/pilot-compilation-view'
a=json.loads(adoption.read_text())
assert a['decision']=='adopt finite unused-helper view preparation' and a['nativeStages']==10
for field,p in [('plan',PLAN),('design',design),('review',review)]:
    assert a[field]=={'path':str(p.relative_to(root)),'sha256':sha(p)}
assert review.read_text().startswith('PASS\n')
assert sha(brief)=='d9503e7e7751bc178cd788a759be1bb8e12eb242846c6ccf7d76538d004744ed'
manifest=validate()
paths=[c/'prepare.py',c/'record.py',c/'offline.py',PLAN,c/'view-manifest.json',design,review,adoption,INVENTORY,brief,old/'prepare.py',old/'record.py']
paths += [root/row['view'] for row in manifest['files']]
assert sha(root/'scripts/run_s02_candidate_a_factoring_pilot.py')==HELPER_SHA
d={'schema':'moriarty.a5-unused-helper-view-dispatch/v1','actualDispatchBase':subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip(),
   'runtimeBootstrapBase':BOOTSTRAP,'runtimeHelperSha256':HELPER_SHA,'planSha256':sha(PLAN),'manifestSha256':sha(c/'view-manifest.json'),
   'sources':{str(p.relative_to(root)):sha(p) for p in paths}}
write(c/'dispatch.json',d)
print(json.dumps(d,indent=2))
```

## UV003: One root slot and original transport for each stage

Before every launch root captures a fresh ps -eo pid,ppid,pgid,sid,stat,lstart,args output and retains its actual tool response. Root inspects that original and the latest completed A4/A5 native/data terminals. No other heavy/native operation may overlap the stage. Write CONTROL/schedule-STAGE.json from that actual inspection, with exactly these fields:
stage; rootInspected=true; schedulerExclusive=true; sourceFreezeHeld=true; dispatchSha256; processListing={path relative to ROOT,sha256}; terminalBindings=[{path relative to ROOT,sha256}] for all latest actual predecessors; ownedPgids as the authentic observed predecessor PGIDs; baselineResourceFailureAdmitted=false except where the independently inspected original checker ended in a genuine resource failure.

Do not invent unobserved PGIDs. The existing helper does not itself publish a PGID receipt; preserve root's actual process observations and disclose any observation gap. A known group must be absent; root's fresh full listing must show no surviving native/helper work. The stage's actual exec/write_stdin responses remain authoritative for its wrapper exit. This limited reuse does not claim RH002 process-monitor guarantees.

Under the same sanitized prefix, set SV_STAGE to the exact stage and run this finite allow writer before launching. This is root's concrete stage decision; a worker cannot self-grant it. The first stage has no predecessors in this new series; its schedule still binds latest A4/A5 terminals. A later stage binds every earlier actual outcome. On actual source/runtime movement, preparation failure, native failure, or failed gate stop; only the explicit original-checker resource outcome exception below can proceed.

```python
import json,os,sys
from pathlib import Path
R=Path('/home/charl/Moriarty/.worktrees/s01-audit-start');C=R/'.superpowers/sdd/a5-factoring-receipts/pilot-unused-helper-view'
assert not sys.flags.optimize and sys.dont_write_bytecode
sys.path.insert(0,str(C))
from prepare import STAGE_ORDER,sha,write,validate
stage=os.environ['SV_STAGE'];assert stage in STAGE_ORDER
d=json.loads((C/'dispatch.json').read_text());validate()
previous=STAGE_ORDER[:STAGE_ORDER.index(stage)]
# Root supplies this original scheduling observation after personally inspecting ps.
scheduling=json.loads((C/('schedule-'+stage+'.json')).read_text())
assert scheduling['rootInspected'] is True and scheduling['schedulerExclusive'] is True
assert scheduling['sourceFreezeHeld'] is True and scheduling['stage']==stage
assert scheduling['dispatchSha256']==sha(C/'dispatch.json')
files=[C/('schedule-'+stage+'.json'),R/scheduling['processListing']['path']]
assert sha(files[-1])==scheduling['processListing']['sha256']
for row in scheduling['terminalBindings']:
    assert sha(R/row['path'])==row['sha256'];files.append(R/row['path'])
for name in previous:
    out=C/name;v=json.loads((out/'view-validation.json').read_text());outer=json.loads((out/'outer-command.json').read_text())
    assert type(outer['responses'][-1].get('exit_code')) is int and outer['actualOuterExit']==outer['responses'][-1]['exit_code']
    if name=='check-original' and not v['ok']:
        assert v['preservation'] is True and scheduling['baselineResourceFailureAdmitted'] is True
    else:assert v['ok'] is True and outer['actualOuterExit']==0
    files += [p for p in out.iterdir() if p.is_file() and not p.name.endswith(('.tar.gz','.qnt.json'))]
value={'stage':stage,'nativeInvocations':1,'dispatchSha256':sha(C/'dispatch.json'),'schedulerExclusive':True,'sourceFreezeHeld':True,
    'ownedPgids':scheduling['ownedPgids'],'predecessors':previous,'baselineResourceFailureAdmitted':scheduling['baselineResourceFailureAdmitted'],
    'files':[{'path':str(p.relative_to(R)),'sha256':sha(p)} for p in sorted(set(files))]}
write(C/('allow-'+stage+'.json'),value)
print(json.dumps({'allowSha256':sha(C/('allow-'+stage+'.json')),'stage':stage,'nativeExecuted':False}))
```

For each stage append to the sanitized prefix:
 /home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=CONTROL/STAGE-parent-cache CONTROL/SCRIPT STAGE
where CONTROL is the literal absolute path defined above, and SCRIPT/STAGE are exactly one row below. Each row is a separate tools.exec_command with ROOT cwd, yield_time_ms1000 and max_output_tokens2500; no shell loop or chained commands.

| Stage | Script | Exact underlying native work | Bound |
|---|---|---|---|
| typecheck | record.py | quint typecheck VIEW/specs/quint/s02/factored_verification/candidate_a_funding_pilot_test.qnt |1200s,4096MiB|
| prefix-test | record.py | quint test same test root --backend=rust --seed=42 --match allPilotPrefixesTest |1200s,4096MiB|
| sample-original | record.py | unchanged original pilot --backend=rust --seed=42 --max-samples=100 --max-steps=5 --invariant=pilotSafety --witnesses prepared signed proposed verified committed rejected afterCompleted beforeCompleted --verbosity=1 |1200s,4096MiB|
| sample-factored | record.py | same exact sample on candidate_a_funding_pilot_f.qnt |1200s,4096MiB|
| compile-original | record.py | quint compile original pilot --main=candidate_a_funding_pilot --target=json --invariant=pilotSafety --verbosity=0 |900s,4096MiB|
| compile-factored | record.py | same compile, factored entry/main candidate_a_funding_pilot_f |900s,4096MiB|
| check-original | offline.py | apalache-mc --out-dir=CONTROL/check-original/apalache check --init=q::init --next=q::step --inv=q::inv --length=5 --no-deadlock CONTROL/compile-original/input.qnt.json |600s,4096MiB|
| check-factored | offline.py | same check with check-factored output and compile-factored/input.qnt.json |600s,4096MiB|
| compile-negative | offline.py | identical factored compile source/main, --invariant=neverPrepared |900s,4096MiB|
| check-negative | offline.py | same check with check-negative output and compile-negative/input.qnt.json |600s,4096MiB|

Quint argv is converted by unchanged record() to the admitted explicit Node executable and resolved CLI. No --flatten=false flag is present. The record() wall interval covers its native /usr/bin/time child; runtime/source checks and Python JSON measurement are outside that interval, as in the original interface. Do not report it as an end-to-end memory/wall limit.

Preserve every original response before the next poll, using write_stdin only on the exact returned session. Keep an exclusive CONTROL/transport-STAGE/response-NNN.json sequence plus command.json; after actual terminal write CONTROL/STAGE/outer-command.json={args,responses,actualOuterExit}. The actual integer exit is copied from the final tool response, never guessed from result.json or helper stdout. Create CONTROL/transport-STAGE before the native launch and retain all responses there; it remains canonical transport, with outer-command.json copying the same exact response list after terminal. Native launch failure with absent stage directory remains in that canonical transport; root inventories missing originals. Failed save or lost terminal triggers root takeover with partial originals retained, never relaunch.

There is one preparation cache, one freeze cache, ten uniquely named STAGE-parent-cache paths, and the helper's separate fresh-python-cache per stage. All must be unused. Do not create or change caches in an old control. Shared runtime/Python archives remain external and pinned. Per-stage source archives retain the full new view and small source/metadata closure; earlier source archives are not nested again. The checking command includes its exact compiled JSON as an input pin/archive member, as required by unchanged record().

## UV004: Gate semantics and final evidence

After typecheck, require child/actual wrapper0, empty stderr, stable complete original/view/runtime pins. After prefix-test require exactly one passing allPilotPrefixesTest, whose unchanged source quantifies all22 full-state prefix pairs with safety/guard/enabled-update equality. After each sample inspect the original eight witness rows: all positive out of exactly100. Root admits all four before any compilation.

Compile-original must succeed and produce genuine JSON before compile-factored is allowed. For each inspect exact generated main, unique declarations, init/step/pilotSafety/q::init/q::step/q::inv, and the actual selected expressions and preserved state/history/guard structure. Names alone are insufficient. Retain complete raw JSON and byte/object/let/app measurements plus raw GNU time resources. Factored bytes must be strictly smaller than original. Failed compile/invalid JSON/non-smaller result closes this experiment; no checker is authorized. Valid but non-smaller compilation is still a real successful compile and a failed size gate. Root records its actual paired-input review before allowing check-original.

The original checker is an actual baseline. Original Task3 explicitly allows its genuine resource failure to remain a failed baseline before one factored check; only root can set baselineResourceFailureAdmitted after examining actual exit/timeouts/resources and stable pins. Name/type/backend-input error, movement, or unexplained failure stops. No baseline success is fabricated.

The factored checker must have actual exit0, terminal NoError, actual exploration of states0–5 and the intended pilotSafety property. Root reads the retained checker/SMT/log artifacts; offline.py's process gate alone does not establish these facts. Any failure before exploration or incomplete log stops. Only after that semantic intake may root allow compile-negative, then its actual compiled q::inv must select neverPrepared from the identical factored view. Only then allow check-negative. Require actual exit12 and a genuine state1 counterexample showing preparation, not an init-only failure or malformed input. The negative property is already in the original pilot; no source mutation implements this control.

On first failure or after the final stage, root makes a finite index of every present CONTROL file (including all source/runner archives, raw generated inputs, checker outputs and actual transport), hashes it, and records absent stages explicitly. Runtime payloads are referenced by their existing manifests and not duplicated. The index itself is excluded from its own member set; any later index-command outer receipt is retained separately. Do not modify original files to fill missing metadata. If hashing/indexing fails, preserve that actual command/partial index and let root inventory missing data; no native retry follows.

Independent final review checks whole archive membership against input pins, complete unchanged original30/new24 closures, literal deletion/restoration, both adapter imports, all actual exits and resource limitations, actual paired size and positive/negative checker semantics. Preserve the helper's inherited fairness/deadlock/classification prose verbatim and explain where a line pertains only to checker stages. Report compilation feasibility, preservation, baseline outcome, factored outcome, negative outcome and H1 separately. A successful pilot supports only the existing live-factor H1 for this symmetrically prepared pilot; it does not close full A5.

## UV005: Source-only verification before handoff

During authoring/review only AST-parse the derived Python strings and compute the finite omission as bytes. Do not import prepare/record/offline or execute a plan block. Verify exact base source hashes, all replacement counts, the26 span hashes, paired3,835-byte equality, complete restored originals,24 original pins and absence of all13 identifiers after transformation. Preserve the actual static tool argv/result. No test, compiler, solver, runtime verifier or large compiler-output parse is part of this source review.
