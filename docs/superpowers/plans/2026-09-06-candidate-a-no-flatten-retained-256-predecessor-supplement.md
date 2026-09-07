# Retained256: finite A4 predecessor supplement

Status: source-only proposal. No body below has been executed. This supplement extends only the root slot writer and the support files it supplies to the unchanged retained256 metadata driver. It grants no parser or native invocation by itself.

The original plan remains byte-for-byte unchanged at `docs/superpowers/plans/2026-09-06-candidate-a-no-flatten-retained-256.md`, 42073 bytes, SHA256 `b6179e3f1b6404fc5acb8417435f5f65bbc5cf7adff39d7e22427b33aadb48c5`. Its design remains SHA256 `dbd209064ac9066925ceb3c469b4648e90f2f6292af943518beb67d7a10784f2`. The original plan explicitly requires a separately source-reviewed predecessor extension if native work intervenes before its slot. Seven A4 mocks and four tiny native controls have now produced intake receipts; the one full A4 diagnostic must also finish and receive independent preservation review and root admission before this extension can be adopted. At drafting, the full result is a future prerequisite, not an observed result.

ROOT is `/home/charl/Moriarty/.worktrees/s01-audit-start`; S is ROOT + `/.superpowers/sdd`; A4 is S + `/a4-run-phase-diagnostic-20260906`; NEW is S + `/a5-no-flatten-retained-256-20260906`. The A4 plan is `docs/superpowers/plans/2026-09-06-candidate-a-run-phase-diagnostic.md`, SHA256 `a7aadad2de0ff8b5af81c8b49a1a876fa2739408b629a17e953ccf2be975d8f3`.

The exact new order is:

1. `mock-identity`, `mock-open`, `mock-write`, `mock-partial`, `mock-records`, `mock-bytes`, `mock-classify`.
2. `success-direct`, `success-observed`, `violation-direct`, `violation-observed`.
3. `full`.

No additional slot is inferred from a directory scan. The full slot's original predecessor inventory is reused; the full result is then added because that result cannot appear in its own prelaunch slot. All old retained256 A5 predecessor requirements remain. The eleven controls require their admitted expected exits (0, six times74, 0,0,1,1), preserved expected-negative eligibility, successful pair comparisons and unforced cleanup. Full requires authentic terminal/transport agreement, complete cleanup and preservation admission. Full may have failed, timed out or lacked an authenticated inner receipt; its actual fields remain unchanged. Full need not have exit0, an eligible RH002 receipt or a phase location. Missing full cleanup or admission closes the gate.

## Finite review and execution order

A nonauthor reviews this supplement after the full evidence exists and writes S + `/a5-no-flatten-retained-256-predecessor-independent-review-20260906.md`, beginning `PASS\n` only for this source and its exact selected predecessor closure. The reviewer checks A4's actual `reviews/full.md` and `admissions/full.json`, not merely the full transport exit. Root reads both reviews and all selected decisions. Root then runs the first metadata body below once, preserving its exact argv and actual returned response in NEW/data-transport after the original plan's initialization. It creates an exclusive S + `/a5-no-flatten-retained-256-predecessor-root-adoption-20260906.json`. This is a predecessor-extension adoption only. It neither replaces the original retained256 plan/design adoption nor authorizes another data invocation.

Next, root captures and personally inspects fresh process and memory observations using the original retained256 commands. Root executes the second body below **instead of the original plan's one slot-writer body**, under the original dispatch-derived clean environment, pinned `/home/charl/Moriarty/.venv/bin/python -B -`, optimization0 and ROOT cwd. It writes only the original NEW/root-slot.json, exclusively. Root preserves exact command and response, including any failure, outside the observations or archives they describe. No failed attempt is silently retried.

All subsequent original retained256 steps remain exact: before, dispatch, prelaunch, one120-second owned parser, after, preserve, independent review and root admission. The original source-review/adoption gates still apply. The original bounded-parser block SHA256 `4b49ad1de7e7c0969991dc1f34c98c8695a88dd28ae61ea76d980dae6a646c68`, metadata-driver block SHA256 `bb20261c8805d2ea6d5fbd98ae1cdc7a1cf58889cd5985501bb2065605d9a648` and constructed NF005 body SHA256 `5e43888ccff0ce5a0ea8f04e3a20513a8505db675ab27f6005da818e8416cf3a` are unchanged. The256MiB input bound,8GiB address-space bound,90/95-second CPU bound,16MiB file bound and original RH002 cleanup are unchanged. No compiler output is parsed by either metadata body.

If more native or memory-intensive data work intervenes after this final A4 predecessor, stop for another explicit extension; this recipe is not an open-ended scheduler. Actual HEAD is observed by the original dispatch flow; no historical HEAD is substituted for it.

## Exact root extension adoption

This binds actual future files only after they exist; there are no placeholder hashes. The root decision fixes the complete full-admission file map by hashing that original admission, and fixes its actual independent review separately. Slot-time selection must match those original maps. Review transport and the root's actual adoption-command responses remain originals, not reconstructed here.

```python
import sys
if sys.flags.optimize:raise RuntimeError('optimization zero')
import hashlib,json
from pathlib import Path
R=Path('/home/charl/Moriarty/.worktrees/s01-audit-start');S=R/'.superpowers/sdd';M=S/'a4-run-phase-diagnostic-20260906'
P=R/'docs/superpowers/plans/2026-09-06-candidate-a-no-flatten-retained-256.md'
Q=R/'docs/superpowers/plans/2026-09-06-candidate-a-no-flatten-retained-256-predecessor-supplement.md'
V=S/'a5-no-flatten-retained-256-predecessor-independent-review-20260906.md'
def need(ok,msg):
 if not ok:raise RuntimeError(msg)
def pin(p):
 need(p.is_file() and not p.is_symlink() and p.stat().st_size<=16*1024**2,'small regular metadata')
 b=p.read_bytes();return {'path':str(p),'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest()}
def read(p):pin(p);return json.loads(p.read_text())
need(pin(P)['sha256']=='b6179e3f1b6404fc5acb8417435f5f65bbc5cf7adff39d7e22427b33aadb48c5','original retained256 plan')
need(V.read_text().startswith('PASS\n'),'actual final predecessor source review')
full=read(M/'admissions/full.json');review=M/'reviews/full.md'
need(full['verdict']=='PASS one run-phase diagnostic preservation only' and full['slots']==['full'],'actual full root preservation admission')
need(full['review']==pin(review) and review.read_text().startswith('PASS\n'),'actual full independent preservation review')
need(full['compilerAcceptance'] is False and full['canonicalExportAcceptance'] is False and full['H1']=='unresolved','limited full scope')
value={'decision':'adopt exact A4 predecessor extension only','plan':pin(P),'supplement':pin(Q),'review':pin(V),
       'fullAdmission':pin(M/'admissions/full.json'),'fullReview':pin(review),'fullSlot':pin(M/'slots/full.json'),
       'controlsAdmission':pin(M/'admissions/controls.json'),'mocksAdmission':pin(M/'admissions/mocks.json'),
       'newNativeInvocations':0,'additionalDataInvocations':0}
p=S/'a5-no-flatten-retained-256-predecessor-root-adoption-20260906.json'
with p.open('x') as f:json.dump(value,f,indent=2,sort_keys=True);f.write('\n')
print(json.dumps({'adoption':pin(p),'parserExecuted':False}))
```

## Exact amended slot writer

This performs a finite metadata intake, not another A4 full audit. It checks every selected small original against authentic A4 admission maps, and all rows in the latest full slot's two predecessor arrays against their stored hashes. The A4 root admissions and their independent reviews remain responsible for large source/runtime/raw-artifact validation; those large inputs are not copied or rehashed here. All full-slot prerequisite rows are included without truncation, and conflicting pins or duplicate paths within a declared list fail. The512-row ceiling on either slot array is a fail-closed metadata bound, not permission to accept512 new invocations.

The selected new support consists of this supplement/review/adoption and A4 plan, the full slot and every file in its original bindings/reviewedPrerequisites, three admissions and three reviews, two pair receipts, and for each exact twelve slots its intake, dispatch JSON, RH002 terminal and five sidecars, and complete original transport command/preflight/response sequence/terminal. The selected metadata is checked against the admission file maps where those maps establish its original identity. The full slot and earlier prerequisites are separately rooted in their original pins and the extension adoption. No recursive addition of large `files` or `originalArtifacts` trees is performed.

```python
import sys
if sys.flags.optimize:raise RuntimeError('optimization zero')
import datetime,hashlib,json,os,re
from pathlib import Path
R=Path('/home/charl/Moriarty/.worktrees/s01-audit-start');S=R/'.superpowers/sdd';O=S/'a5-no-flatten-diagnostic-20260906';N=S/'a5-no-flatten-retained-256-20260906'
M=S/'a4-run-phase-diagnostic-20260906'
P=R/'docs/superpowers/plans/2026-09-06-candidate-a-no-flatten-retained-256.md'
Q=R/'docs/superpowers/plans/2026-09-06-candidate-a-no-flatten-retained-256-predecessor-supplement.md'
V=S/'a5-no-flatten-retained-256-predecessor-independent-review-20260906.md'
E=S/'a5-no-flatten-retained-256-predecessor-root-adoption-20260906.json'
AP=R/'docs/superpowers/plans/2026-09-06-candidate-a-run-phase-diagnostic.md'
MOCKS=['mock-'+x for x in ['identity','open','write','partial','records','bytes','classify']]
TINY=['success-direct','success-observed','violation-direct','violation-observed'];ORDER=MOCKS+TINY+['full']
def need(ok,msg):
 if not ok:raise RuntimeError(msg)
def read(p):
 p=Path(p);need(p.is_file() and not p.is_symlink() and p.stat().st_size<=16*1024**2,'bounded regular metadata')
 return json.loads(p.read_text())
def pin(p):
 p=Path(p);need(p.is_absolute() and p.is_relative_to(R) and p.is_file() and not p.is_symlink(),'regular ROOT metadata')
 need(p.stat().st_size<=16*1024**2,'small support only')
 b=p.read_bytes();return {'path':str(p),'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest()}
extra={}
def retain(p,row=None):
 got=pin(p)
 if row is not None:
  need(row['path']==got['path'] and row['sha256']==got['sha256'],'original metadata binding')
  if 'bytes' in row:need(row['bytes']==got['bytes'],'original metadata size')
 need(got['path'] not in extra or extra[got['path']]==got,'conflicting selected metadata')
 extra[got['path']]=got;return got
def absent(n):
 need(type(n) is int and n>0,'owned positive PGID')
 try:os.killpg(n,0)
 except ProcessLookupError:return
 raise RuntimeError('predecessor group remains')
def pinmap(rows):
 need(isinstance(rows,list),'original pin list');out={}
 for row in rows:
  need(row['path'] not in out,'duplicate original pin');out[row['path']]=row
 return out
ad=read(E);retain(E)
need(ad['decision']=='adopt exact A4 predecessor extension only' and ad['newNativeInvocations']==0 and ad['additionalDataInvocations']==0,'root extension only')
for key,path in [('plan',P),('supplement',Q),('review',V),('fullAdmission',M/'admissions/full.json'),('fullReview',M/'reviews/full.md'),('fullSlot',M/'slots/full.json'),('controlsAdmission',M/'admissions/controls.json'),('mocksAdmission',M/'admissions/mocks.json')]:retain(path,ad[key])
need(extra[str(P)]['sha256']=='b6179e3f1b6404fc5acb8417435f5f65bbc5cf7adff39d7e22427b33aadb48c5','unchanged retained256 plan')
need(retain(AP)['sha256']=='a7aadad2de0ff8b5af81c8b49a1a876fa2739408b629a17e953ccf2be975d8f3','exact A4 plan')
need(V.read_text().startswith('PASS\n'),'actual final extension review')
admissions={};maps={}
for mode,slots,verdict in [('mocks',MOCKS,'PASS seven run-profile mocks only'),('controls',MOCKS+TINY,'PASS seven mocks and four tiny run controls only'),('full',['full'],'PASS one run-phase diagnostic preservation only')]:
 a=read(M/'admissions'/(mode+'.json'));admissions[mode]=a;maps[mode]=pinmap(a['files'])
 need(a['slots']==slots and a['verdict']==verdict,'exact original admission scope')
 need(a['fullDispatchAuthorized'] is False and a['compilerAcceptance'] is False and a['canonicalExportAcceptance'] is False and a['H1']=='unresolved','original limited admission')
 rp=M/'reviews'/(mode+'.md');retain(rp,a['review']);need(rp.read_text().startswith('PASS\n'),'actual independent review')
combined={}
for entries in maps.values():
 for path,row in entries.items():
  need(path not in combined or combined[path]==row,'admission-map conflict');combined[path]=row
def admitted(p):
 p=Path(p);need(str(p) in combined,'selected original absent from admission maps');return retain(p,combined[str(p)])
# Bind the successive root decisions to the earlier admission originals.
for p,mode in [(M/'admissions/mocks.json','controls'),(M/'admissions/controls.json','full')]:
 need(str(p) in maps[mode],'prior admission explicitly retained');retain(p,maps[mode][str(p)])
fullslot=read(M/'slots/full.json')
need(fullslot['slot']=='full' and fullslot['authorized'] is True and fullslot['schedulerExclusive'] is True,'actual final A4 root slot')
slotrows={}
for key in ['bindings','reviewedPrerequisites']:
 rows=fullslot[key];need(len(rows)<=512,'bounded original predecessor inventory')
 for path,row in pinmap(rows).items():
  need(path not in slotrows or slotrows[path]==row,'full-slot pin conflict');slotrows[path]=row;retain(Path(path),row)
need(str(M/'admissions/controls.json') in slotrows,'full dispatch bound actual controls admission')
newpgids=list(fullslot['ownedPgids'])
for n in newpgids:absent(n)
for slot in ORDER:
 ip=M/'intake'/(slot+'.json');admitted(ip);a=read(ip)
 need(a['slot']==slot and a['preservation'] is True and a['compilerAcceptance'] is False and a['canonicalExportAcceptance'] is False and a['H1']=='unresolved','preserved exact slot')
 tp=M/'outer'/slot/'terminal.json';xp=M/'transport'/slot/'terminal.json'
 retain(tp,a['outerTerminal']);retain(xp,a['transportTerminal']);admitted(tp);admitted(xp)
 t=read(tp);tr=read(xp)
 need(t['cleanup']['complete'] is True and t['cleanup']['errors']==[],'complete actual owned cleanup')
 pgid=t['owned_pgid']
 if pgid is None:need(slot=='full' and t['launch_error'] is not None,'no group only authenticated full launch failure')
 else:absent(pgid);newpgids.append(pgid)
 need(type(tr['actualOuterExit']) is int and tr['responses'] and tr['responses'][-1]['exit_code']==tr['actualOuterExit']==a['actualOuterExit']==t['return_code'],'actual outer exit equality')
 need(a['actualTimeWrappedExit']==t['actual_exit'],'actual wrapped exit preserved')
 td=xp.parent;responsepaths=sorted(td.glob('response-*.json'))
 need([p.name for p in responsepaths]==['response-%03d.json'%i for i in range(len(responsepaths))],'contiguous original response names')
 need([read(p) for p in responsepaths]==tr['responses'],'actual complete tool responses')
 for p in [td/'command.json',td/'preflight.json',*responsepaths]:admitted(p)
 command=read(td/'command.json')
 need(command['ok'] is True and command['slot']==slot and command['args']==tr['args'],'original launch argv')
 dp=M/'dispatch'/(slot+'.json');retain(dp,command['dispatch']);admitted(dp)
 expectedsidecars={str(tp.parent/x) for x in ['launch.json','resources.txt','runner.py','stdout.bin','stderr.bin']}
 need(set(t['sidecar_sha256'])==expectedsidecars,'exact RH002 sidecar identities')
 for path,h in t['sidecar_sha256'].items():retain(Path(path),{'path':path,'sha256':h});admitted(Path(path))
 if slot!='full':
  need(str(tp) in slotrows and str(xp) in slotrows,'all eleven predecessor terminal pairs in full slot')
  need(pgid in fullslot['ownedPgids'],'all eleven owned groups in full slot')
  expected=0 if slot=='mock-identity' or slot.startswith('success-') else 74 if slot.startswith('mock-') else 1
  need(a['expectedResultPass'] is True and a['validationErrors']==[] and a['innerReceiptAuthenticated'] is True,'admitted expected control result')
  need(a['actualCompilerOrMockExit']==a['actualTimeWrappedExit']==a['actualOuterExit']==expected,'exact expected control exit')
  need(t['eligible'] is (expected==0) and t['timed_out'] is False and t['cleanup']['forced'] is False and t['cleanup']['signals']==[],'honest control eligibility and cleanup')
 else:
  if not a['innerReceiptAuthenticated']:need(a['actualCompilerOrMockExit'] is None,'unknown full native exit remains unknown')
  for key,value in admissions['full']['newFullDiagnostic'].items():need(value==a.get(key),'actual full result unchanged')
for kind in ['success','violation']:
 p=M/'pairs'/(kind+'.json');admitted(p);pair=read(p)
 need(pair['pair']==kind and pair['slots']==[kind+'-direct',kind+'-observed'] and pair['ok'] is True,'exact admitted pair')
 need(pair['allOtherITFValuesEqual'] is True and pair['allOtherStdoutBytesEqual'] is True and pair['stderrBytesEqual'] is True,'admitted comparison scope')
# Original A5 predecessor gate and fresh scheduling requirements remain.
t=read(O/'full-outer/terminal.json');tr=read(O/'transport/full/terminal.json');oldslot=read(S/'a5-no-flatten-slot-full-20260906.json')
if t['cleanup']!={'complete':True,'errors':[],'forced':False,'signals':[]} or t['return_code']!=tr['actualOuterExit'] or tr['responses'][-1]['exit_code']!=0:raise RuntimeError('completed original full')
pgids=sorted(set([t['owned_pgid'],*oldslot['ownedPgids'],*newpgids]))
for pgid in pgids:absent(pgid)
ps=N/'observations/processes.txt';mem=N/'observations/meminfo.txt'
available=re.findall(r'^MemAvailable:\s+([0-9]+) kB$',mem.read_text(),re.M)
if len(available)!=1 or int(available[0])<12*1024*1024 or ps.stat().st_size==0:raise RuntimeError('fresh root-inspected scheduling metadata')
files=[O/'full-outer/terminal.json',O/'transport/full/terminal.json',O/'root-admission-full.json',S/'a5-no-flatten-slot-full-20260906.json',*[Path(r['path']) for r in oldslot['terminalBindings']],ps,mem,*[Path(p) for p in extra]]
value={'authorizedDataInvocations':1,'nativeCompilerAuthorized':False,'schedulerExclusive':True,'ownedPgids':pgids,'files':[pin(p) for p in sorted(set(files))],'memAvailableKiB':int(available[0]),'observedAt':datetime.datetime.now(datetime.timezone.utc).isoformat()}
p=N/'root-slot.json'
with p.open('x') as f:json.dump(value,f,indent=2,sort_keys=True);f.write('\n')
print(json.dumps({'slot':pin(p),'parserExecuted':False,'a4Predecessors':ORDER,'selectedA4SupportFiles':len(extra)}))
```

## Support archive, transport and limits

The original metadata driver forms `support_paths` from its unchanged known closure and every `root-slot.json.files` path. Therefore all new small originals above enter NEW/before/source.tar.gz and its exact index **before** dispatch. The later dispatch uses its original separate one-member archive/seal; no dispatch or later parser transport is inserted into a prior source archive. This supplement, its final review and root adoption exist before the slot and enter its support array, which prevents a hash cycle. The A4 full slot is historical, not mutated to include the full result.

The original before/after support checks detect later selected-file changes; they do not independently reopen all A4 runtime archives. A4 reviews and admissions are preserved evidence for that separate work. Root and nonauthor must inspect their exact source/runtime/cleanup scope, including any failed full diagnostics or missing inner receipt, before admitting this extension. The selected original terminal receipts establish process termination; the fresh PGID and process observations establish the scheduling check at this decision, not continuous monitoring or a guarantee about unrelated processes.

Retain the actual extension adoption/slot tool command and each response in NEW/data-transport using the existing args/responses interface; save returned failures as failures. The original parser transport routine remains unchanged and preserves every actual response before polling again. A lost response is disclosed, never reconstructed. Neither a failed original128MiB intake nor a failed A4 native outcome becomes successful because these predecessors have terminated. This supplement changes no structural predicate, native command, H1 status, acceptance flag, resource ceiling, parser count or retry rule.

Source review may compile the two Python bodies and compare source-block hashes without executing them. Full evidence review and root adoption remain required before the amended slot writer can be used. No materialization, native call, compiler-data parse, runtime edit or commit is part of authoring this supplement.
