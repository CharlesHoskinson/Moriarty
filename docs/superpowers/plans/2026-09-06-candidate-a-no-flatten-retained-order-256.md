# Candidate A retained-order256 counted-delta supplement

**Goal:** Inspect the same retained 199,868,611-byte no-flatten output once with the corrected declaration-order predicate, within the previously adopted data limits.
**Architecture:** Derive the two immutable retained256 plan blocks in memory. Apply the four exact NF005 substitutions and finite metadata/path/provenance changes below. Reuse RH002 and original transport.
**Toolchain:** Pinned Python -B, optimization zero; existing RH002; no compiler, solver, helper import or project test.

Status: source-only exact supplement; no block is executed during authoring/review. ROOT is /home/charl/Moriarty/.worktrees/s01-audit-start. NEW is ROOT/.superpowers/sdd/a5-no-flatten-retained-order-256-20260906. The new plan is this file; all original plans, receipts, source copies and archives remain immutable.

This supersedes ONLY the proposal's instruction to release the A4 source freeze before parsing. Root MUST retain that freeze through the corrected data invocation, settled cleanup, and successful unchanged-source after snapshot. Only root may then release it. The original proposal7164402e and independent REQUEST_CHANGES14639be7 stay unchanged and are pinned below. Their one freeze-wording objection is resolved here; the reviewer accepted the ordering/four-substitution/envelope source reasoning. No A4 native or source work may intervene between the new slot and after. If any does, stop.

The hypothesis is a malformed validator ordering assumption, not a changed compiler or model. Original128-MiB and failed256 gates stay failed. Old256's actual537f3c/0 admission is pinned64198426. Its raw GNU-time record exists but RH002 could not parse the multiline command; no metrics are invented. The new one-physical-line bootstrap changes only command transport, never RH002. Limits remain input256MiB, address space8GiB, CPU90/95s, file writes16MiB, core0, RH002120s wall/5s grace/5s cleanup, and12GiB MemAvailable. Only one corrected parse is authorized after actual review/root adoption. Failure closes the slot without retry. Structural success is not compiler acceptance or H1 evidence.

## 1. Review, root adoption and command environment

Nonauthor review: ROOT/.superpowers/sdd/a5-no-flatten-retained-order-256-independent-review-20260906.md, beginning PASS followed by newline only on acceptance. After root reads it, create the following exclusive adoption under the same sanitized pinned Python environment described below. This is the existing preparation/slot/dispatch authority, not another approval hierarchy.

```python
import sys
if sys.flags.optimize or not sys.dont_write_bytecode:raise RuntimeError('optimization zero and -B')
import hashlib,json
from pathlib import Path
R=Path('/home/charl/Moriarty/.worktrees/s01-audit-start');S=R/'.superpowers/sdd'
P=R/'docs/superpowers/plans/2026-09-06-candidate-a-no-flatten-retained-order-256.md';V=S/'a5-no-flatten-retained-order-256-independent-review-20260906.md'
def pin(p):return {'path':str(p),'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
if not V.read_text().startswith('PASS\n'):raise RuntimeError('accepted actual independent review')
with (S/'a5-no-flatten-retained-order-256-root-adoption-20260906.json').open('x') as f:
 json.dump({'decision':'adopt retained-order256 preparation only','plan':pin(P),'review':pin(V),'dataInvocations':1,'nativeCompilerAuthorized':False},f,indent=2,sort_keys=True);f.write('\n')
print(json.dumps({'adoption':pin(S/'a5-no-flatten-retained-order-256-root-adoption-20260906.json'),'parserExecuted':False}))
```

Every parent/data/transport Python uses the exact original full dispatch parentRemoved/parentFixed environment, SHA878870cf1c9ea1cc3dcbe64e37c002011081e93a7adc494c4e2565fd09d98fba, as required by the immutable retained256 plan. Construct argv with env, each -u/key pair, each fixed key=value, NF256_STEP=<one literal step below>, then /home/charl/Moriarty/.venv/bin/python -B -c and the exact bootstrap below. Use shlex.join, never string interpolation of unquoted code. Workdir is ROOT. Preserve actual arguments and all responses using the original retained256 data-transport rules, with the fresh NEW path. Before initialization exists use exclusive sibling a5-no-flatten-retained-order-256-initial-tools-20260906.json and exclusive progress files. A failed save invokes root takeover; retain unsaved actual responses in the transcript, no synthetic originals or replay.

Exact metadata bootstrap (NF256_STEP is one of initialize, slot, before, dispatch, prelaunch, after, preserve, admit):
```python
from pathlib import Path
import hashlib,json,re
p=Path('/home/charl/Moriarty/.worktrees/s01-audit-start/docs/superpowers/plans/2026-09-06-candidate-a-no-flatten-retained-order-256.md')
a=json.loads(Path('/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-no-flatten-retained-order-256-root-adoption-20260906.json').read_text())
s=p.read_bytes()
if a['decision']!='adopt retained-order256 preparation only' or a['plan']['path']!=str(p) or hashlib.sha256(s).hexdigest()!=a['plan']['sha256']:raise RuntimeError('actual adopted correction')
b=re.findall(r'^<!-- correction-driver -->\n```python\n(.*?)^```$',s.decode(),re.S|re.M)
if len(b)!=1:raise RuntimeError('one correction driver')
exec(compile(b[0],'<retained-order-metadata>','exec'))
```

## 2. Exact counted-delta driver

The two base blocks are pinned before every derivation. The original NF005 is independently reconstructed by parent and child and must hash d4d949fc3ba7d5798b7a5e6885e7e1e8a56abbc54389a8b2b7afee06148a0902. This block is source only until adoption.

<!-- correction-driver -->
```python
import sys
if sys.flags.optimize or not sys.dont_write_bytecode:raise RuntimeError('optimization zero and -B')
import ast,datetime,hashlib,json,os,re
from pathlib import Path
C_R=Path('/home/charl/Moriarty/.worktrees/s01-audit-start');C_S=C_R/'.superpowers/sdd'
C_P=C_R/'docs/superpowers/plans/2026-09-06-candidate-a-no-flatten-retained-order-256.md'
C_A=C_S/'a5-no-flatten-retained-order-256-root-adoption-20260906.json'
C_B=C_R/'docs/superpowers/plans/2026-09-06-candidate-a-no-flatten-retained-256.md'
C_OLD=C_S/'a5-no-flatten-retained-256-20260906';C_N=C_S/'a5-no-flatten-retained-order-256-20260906'
def C_need(v,m):
 if not v:raise RuntimeError(m)
def C_pin(p):
 p=Path(p);C_need(p.is_file() and not p.is_symlink() and p.stat().st_size<=16*1024**2,'small regular support')
 b=p.read_bytes();return {'path':str(p),'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest()}
def C_read(p):C_pin(p);return json.loads(Path(p).read_text())
def C_check(r):C_need(C_pin(r['path'])==r,'exact support')
def C_put(p,v):
 with Path(p).open('x') as f:json.dump(v,f,indent=2,sort_keys=True);f.write('\n')
def C_block(p,label):
 z=re.findall(r'^<!-- '+re.escape(label)+r' -->\n'+chr(96)*3+r'python\n(.*?)^'+chr(96)*3+r'$',Path(p).read_text(),re.S|re.M)
 C_need(len(z)==1,'unique source block '+label);return z[0]
def C_replace(s,a,b,n=1):
 C_need(s.count(a)==n,'counted delta '+a[:90]);return s.replace(a,b)
C_ad=C_read(C_A)
C_need(C_ad['decision']=='adopt retained-order256 preparation only' and C_ad['dataInvocations']==1 and C_ad['nativeCompilerAuthorized'] is False,'root source adoption')
for C_row in [C_ad['plan'],C_ad['review']]:C_check(C_row)
C_need(C_ad['plan']==C_pin(C_P) and Path(C_ad['review']['path']).read_text().startswith('PASS\n'),'exact reviewed supplement')
C_known={C_B:'b6179e3f1b6404fc5acb8417435f5f65bbc5cf7adff39d7e22427b33aadb48c5',C_S/'a5-retained-order-256-predicate-correction-proposal-20260906.md':'7164402e0e042ec8f16e3dc688b7c114a54a3869b323e60dc7cdc8543eb71359',C_S/'a5-retained-order-256-predicate-independent-review-20260906.md':'14639be715451267ac74feda6da778f271d0ba85b5fb4e6a13d974d9a6c07e70',C_S/'a5-retained-order-256-source-read-receipts-20260906.json':'a95e5ed7ff9663197c6712c9532b64792c24633add496dab536cdee2dcf3c966',C_OLD/'final-admission.json':'6419842633fffbe66294c0c2069116df4e456460dfd0bf7baba56722cb807816',C_OLD/'retained-intake.json':'3ee52e89dfa5f7c282ea653e12317ff604e050a18336b9ee5f7ce8de1dc16bc9',C_OLD/'preservation.json':'f94d9e659f6498602db3b2b0aada6e5bc713f8728874792d649501a88f453275'}
for C_path,C_sha in C_known.items():C_need(C_pin(C_path)['sha256']==C_sha,'frozen predecessor/proposal')
C_fa=C_read(C_OLD/'final-admission.json')
C_need(C_fa['retainedStructureAdmitted'] is False and C_fa['compilerAcceptance'] is False and C_fa['H1']=='unresolved','failed predecessor remains failed')
for C_row in [C_fa['review'],C_fa['preservation']]:C_check(C_row)
C_tool=C_read(C_OLD/'data-transport/admit-terminal.json')
C_need(C_tool['actualExit']==C_tool['responses'][-1]['exit_code']==0 and C_tool['responses'][-1]['chunk_id']=='537f3c' and json.loads(C_tool['responses'][-1]['output'])['admission']==C_pin(C_OLD/'final-admission.json'),'authentic failed256 admission')
C_support=set(C_known)|{C_P,C_A,Path(C_ad['review']['path']),Path(C_fa['review']['path'])}
C_support|={C_OLD/'data-transport'/name for name in ['admit-command.json','admit-response-000.json','admit-terminal.json']}
C_oldpres=C_read(C_OLD/'preservation.json')
C_support|={Path(r['path']) for r in C_oldpres['artifacts']}
C_support|={C_OLD/name for name in ['root-slot.json','dispatch.json','dispatch-seal.json','before/source-index.json','before/inputs.json','parser-limits.json']}
for C_row in C_oldpres['artifacts']:C_check(C_row)
C_step=os.environ['NF256_STEP']
if C_step=='slot':
 C_t=C_read(C_OLD/'outer/terminal.json');C_tr=C_read(C_OLD/'transport/terminal.json');C_slot=C_read(C_OLD/'root-slot.json')
 C_need(C_t['return_code']==C_t['actual_exit']==C_tr['actualOuterExit']==C_tr['responses'][-1]['exit_code']==2 and C_t['owned_pgid']==3674297 and C_t['cleanup']=={'complete':True,'errors':[],'forced':False,'signals':[]} and C_t['timed_out'] is False,'actual failed256 clean predecessor')
 C_pgids=sorted(set(C_slot['ownedPgids']+[C_t['owned_pgid']]))
 for C_pgid in C_pgids:
  C_need(type(C_pgid) is int and C_pgid>0,'positive owned PGID')
  try:os.killpg(C_pgid,0)
  except ProcessLookupError:continue
  raise RuntimeError('predecessor group remains')
 for C_row in C_slot['files']:C_check(C_row)
 C_ps=C_N/'observations/processes.txt';C_mem=C_N/'observations/meminfo.txt'
 C_av=re.findall(r'^MemAvailable:\s+([0-9]+) kB$',C_mem.read_text(),re.M)
 C_need(len(C_av)==1 and int(C_av[0])>=12*1024*1024 and C_ps.stat().st_size>0,'fresh root-inspected exclusive slot')
 C_files=C_support|{Path(r['path']) for r in C_slot['files']}|{C_ps,C_mem}
 C_put(C_N/'root-slot.json',{'authorizedDataInvocations':1,'nativeCompilerAuthorized':False,'schedulerExclusive':True,'ownedPgids':C_pgids,'files':[C_pin(p) for p in sorted(C_files)],'memAvailableKiB':int(C_av[0]),'observedAt':datetime.datetime.now(datetime.timezone.utc).isoformat(),'sourceFreezeThroughAfter':True,'priorFailed256':C_pin(C_OLD/'final-admission.json')})
 print(json.dumps({'slot':C_pin(C_N/'root-slot.json'),'parserExecuted':False}));raise SystemExit(0)
C_insert="    if mode=='full':\n        require(len(ds)==40,'all forty original/generated declarations')\n        generated=ds[-3:]\n        require([(x.get('kind'),x.get('name'),x.get('qualifier')) for x in generated]==[('def','q::init','action'),('def','q::step','action'),('def','q::inv','val')],'three appended selections retain original order')\n        original_declarations=ds[:-3]\n        require(len(original_declarations)==37 and all(x['id']<module['id'] for x in original_declarations) and all(x['id']>module['id'] for x in generated),'original parse IDs precede module ID; generated IDs follow')\n        require([x['id'] for x in original_declarations[:13]]==[1,10,11,12,13,2,3,4,5,6,7,8,9],'exact retained topological import order')\n        source_order=sorted(original_declarations,key=lambda x:x['id'])\n        declaration_order_correction={'method':'numeric original parse IDs; appended selections unchanged','originalOrderIds':[x['id'] for x in original_declarations],'sourceOrderIds':[x['id'] for x in source_order],'generatedOrderIds':[x['id'] for x in generated],'declarationsPreserved':40}\n        ds=source_order+generated\n"
C_anchor="    require(len({x['id'] for x in ds})==len(ds),'distinct top-level declaration IDs')\n"
C_replacements=[('LIMIT=128*1024*1024','LIMIT=256*1024*1024'),('compiler JSON exceeds128 MiB intake bound','compiler JSON exceeds256 MiB intake bound'),(C_anchor,C_anchor+C_insert),("path=M/(mode+'-intake.json')","result['retainedDataContinuation']=retained_provenance\nresult['declarationOrderCorrection']=locals().get('declaration_order_correction')\npath=Path("+repr(str(C_N/'retained-intake.json'))+")")]
C_expected='d4d949fc3ba7d5798b7a5e6885e7e1e8a56abbc54389a8b2b7afee06148a0902'
def C_constructed():
 z=re.findall(r'^'+chr(96)*3+r'python\n(.*?)^'+chr(96)*3+r'$',(C_R/'docs/superpowers/plans/2026-09-06-candidate-a-no-flatten-diagnostic.md').read_text(),re.S|re.M)
 C_need(len(z)==7 and hashlib.sha256(z[3].encode()).hexdigest()=='6801bed3dc7b9cad0a0ee495f45512a6eea7d9168ef2f088db73d9901ce8004a','original NF005')
 b=z[3]
 for a,v in C_replacements:b=C_replace(b,a,v)
 ast.parse(b);C_need(hashlib.sha256(b.encode()).hexdigest()==C_expected,'exact corrected NF005');return C_expected
def C_bootstrap():
 b="from pathlib import Path\nimport hashlib,re,os\np=Path("+repr(str(C_P))+")\ns=p.read_bytes()\nif hashlib.sha256(s).hexdigest()!="+repr(C_pin(C_P)['sha256'])+":raise RuntimeError('frozen correction supplement')\nb=re.findall(r'^<!-- correction-driver -->\\n"+chr(96)*3+"python\\n(.*?)^"+chr(96)*3+"$',s.decode(),re.S|re.M)\nif len(b)!=1:raise RuntimeError('one correction source')\nos.environ['NF256_STEP']='child'\nexec(compile(b[0],'<retained-order-controller>','exec'))\n"
 e="exec(compile("+repr(b)+",'<retained-order-envelope>','exec'))"
 C_need('\n' not in e and '\r' not in e,'one physical line');ast.parse(e);return e
C_child=C_block(C_B,'bounded-parser');C_meta=C_block(C_B,'metadata-driver')
C_need(hashlib.sha256(C_child.encode()).hexdigest()=='4b49ad1de7e7c0969991dc1f34c98c8695a88dd28ae61ea76d980dae6a646c68' and hashlib.sha256(C_meta.encode()).hexdigest()=='bb20261c8805d2ea6d5fbd98ae1cdc7a1cf58889cd5985501bb2065605d9a648','two immutable base bodies')
C_child=C_replace(C_child,'a5-no-flatten-retained-256-20260906','a5-no-flatten-retained-order-256-20260906',2)
C_meta=C_replace(C_meta,'a5-no-flatten-retained-256-20260906','a5-no-flatten-retained-order-256-20260906',2)
C_line=[x for x in C_child.splitlines(True) if x.startswith('replacements=')]
C_need(len(C_line)==1,'one old replacement assignment');C_child=C_replace(C_child,C_line[0],'replacements=C_replacements\n')
C_child=C_replace(C_child,"provenance={'originalFailedIntake':","provenance={'priorFailed256':C_pin(C_OLD/'final-admission.json'),'originalFailedIntake':")
C_start=C_meta.index('def constructed():\n');C_end=C_meta.index("if step=='dispatch':\n",C_start)
C_meta=C_replace(C_meta,C_meta[C_start:C_end],'def constructed():return C_constructed()\n')
C_line=[x for x in C_meta.splitlines(True) if x.startswith(' bootstrap=')]
C_need(len(C_line)==1,'one old child bootstrap');C_meta=C_replace(C_meta,C_line[0],' bootstrap=C_bootstrap()\n')
C_meta=C_replace(C_meta,'support=[pin(p) for p in sorted(support_paths)]','support_paths|=C_support\nsupport=[pin(p) for p in sorted(support_paths)]')
C_meta=C_replace(C_meta,"dispatch={'dataInvocations':1,","dispatch={'priorFailed256':C_pin(C_OLD/'final-admission.json'),'dataInvocations':1,")
C_meta=C_replace(C_meta,"'generatedSelections','retainedDataContinuation'},","'generatedSelections','retainedDataContinuation','declarationOrderCorrection'},")
C_final=""" corr=fresh['declarationOrderCorrection'];oi=corr['originalOrderIds'];si=corr['sourceOrderIds'];gi=corr['generatedOrderIds']
 need(set(corr)=={'method','originalOrderIds','sourceOrderIds','generatedOrderIds','declarationsPreserved'} and corr['method']=='numeric original parse IDs; appended selections unchanged' and corr['declarationsPreserved']==40,'exact correction metadata')
 need(len(oi)==len(si)==37 and len(gi)==3 and all(type(x) is int for x in oi+si+gi) and len(set(oi+gi))==40 and si==sorted(oi) and oi[:13]==[1,10,11,12,13,2,3,4,5,6,7,8,9] and max(oi)<min(gi),'complete forty-ID original partition')
 need(prov['priorFailed256']==d['priorFailed256']==C_pin(C_OLD/'final-admission.json'),'failed256 predecessor provenance')
"""
C_meta=C_replace(C_meta," result['structuralPredicatesPassed']=True\n",C_final+" result['structuralPredicatesPassed']=True\n")
C_constructed();ast.parse(C_child);ast.parse(C_meta)
C_need(C_step in ['initialize','before','dispatch','prelaunch','after','preserve','admit','child'],'finite corrected step')
exec(compile(C_child if C_step=='child' else C_meta,'<derived-retained-order-'+C_step+'>','exec'),globals())
```

## 3. Finite execution and unchanged transport

Run initialize once. Preserve its actual terminal. Capture and inspect these two fresh observations, recording both actual tool responses:

```bash
bash -c 'set -C; exec ps -eo pid,ppid,pgid,sid,stat,lstart,args > /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-no-flatten-retained-order-256-20260906/observations/processes.txt'
```

```bash
bash -c 'set -C; exec cat /proc/meminfo > /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-no-flatten-retained-order-256-20260906/observations/meminfo.txt'
```

Then run slot, before, dispatch, prelaunch sequentially, each only after previous actual exit0 and saved original response. Slot reuses the admitted failed256 predecessor's finite file pins and all owned PGIDs, adding3674297 and its authentic terminal/failed admission. Before retains the unchanged1090 originals/8102 runtime inventory checks and exact tree membership; new support adds only finite small metadata, not prior source archives or the raw199MB/runtime payload. Only the base before support archive and one-dispatch archive are made. No additional archive layer.

Store prelaunch's actual returned object with store('retained-order256-prelaunch', response). Execute the original retained256 plan's sole JavaScript transport block after precisely these substitutions (each count1):
- load('retained256-prelaunch') -> load('retained-order256-prelaunch')
- the directory token a5-no-flatten-retained-256-20260906 -> a5-no-flatten-retained-order-256-20260906.

All other transport bytes, environment, response-save ordering, poll bounds and failure takeover remain exact. This calls the exact prelaunch-returned RH002 command once. The generated child -c has zero physical LF/CR; its decoded envelope executes the same derived bounded parser, with the same pinned executable, flags and limits. No second parser invocation belongs to this plan.

After terminal, retain every original response and inspect cleanup. Run after while the A4 source freeze is still held; it must reproduce all original pins and runtime memberships. Only after that successful snapshot may root release A4 source work. Run preserve. Obtain NEW/independent-review.md and run admit exactly as in the base plan. The unchanged base preserves failure results and distinguishes time-wrapped exit from outer normalization, missing/partial intake and resource failures. If after/preserve/admit fails, retain actual partial originals and tool failures; root inventories missing data. Never reconstruct a terminal or retry the parse.

The original failed256 after/input/support files are immutable prerequisites. New result adds declarationOrderCorrection with the exact original/source/generated ID partitions, while the original full NF005 ordered import inventory, all40 declarations, type/schema/unique-ID checks, PilotState and annotations, and exact generated selections remain unchanged. Numeric ordering applies only to the37 original declarations in a local comparison list; document bytes/object are not rewritten, and appended selections stay in original order. This does not claim full pilot operator-body equivalence beyond the existing NF005 predicates.

Independent intake checks the exact derived source hashd4d949fc, actual corrected arguments/bootstrap, all raw transport/resources and actual exits, prior failure pins, complete before/after observations, and new correction metadata. No extra unbounded JSON parse is implied. On success only retained altered-command structure can be admitted; original default-flatten compilation, paired-size gate, offline checker, and H1 remain open.

