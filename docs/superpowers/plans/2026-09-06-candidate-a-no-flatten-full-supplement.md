# Candidate A No-Flatten Full Supplement Implementation Plan

> **For agentic workers:** Use superpowers:executing-plans for the reviewed, root-dispatched steps. This document is source-only until root adopts it. Root owns decisions and the one native dispatch; a worker cannot grant itself a slot.

**Goal:** Permit the still-unused full no-flatten observation only after independent review and root admission of the corrected retained tiny data.

**Architecture:** Extract the exact reviewed blocks from the immutable original plan and apply finite, counted substitutions to its full tiny-admission references and support closure. Keep the existing integrated producer recorder, RH002, native command, full intake and cleanup unchanged. Preserve the failed original tiny gate and the separate corrected intake as distinct originals.

**Tech Stack:** Pinned Python standard-library data commands; existing Quint 0.32.0 CLI and Node 24.18.1; existing integrated recorder and RH002; authentic tools.exec_command/write_stdin transport.

## Global constraints and evidence boundary

Repository observation: original plan `docs/superpowers/plans/2026-09-06-candidate-a-no-flatten-diagnostic.md` is 110164 bytes, SHA-256 `79325d985b5f58a62a5f490b0ac51f60c873afcb36775be3d95e372d144b2260`. It is already in actual tiny before/after archives and must not be edited. Its fixed 215-path union, including integrated121, A5 view/original54, alias4 and support38, stays exact.

Experiment observation: original native tiny exited zero. Original `tiny-intake.json`, SHA-256 `2c39b46364d588dbe78f1505a657f5df003051b8a59ad67c411193becd44a988`, failed the incorrect `Concrete=Box[int]` expectation. The separate retained-data correction produced `tiny-retained-type-application-intake.json`, 82939 bytes, SHA-256 `f2d73ee627dd4025b7038149f2a5b99f78abb29a6e55fba44f1f4d8d121ed4aa`, with successful corrected predicates in actual tool result f108b4/0. This result is a prerequisite for review, not full authority.

Source fact: the pinned analyzer resolves type applications before the `--flatten=false` branch. The correction retains `Key` as a type constant while substituting the generic `Box` argument. Diagnosis `a5-no-flatten-tiny-type-application-diagnosis-20260906.md`, SHA-256 `6d8a52ea54b5a70d1e2d6f0bd7bc72aa298c3139fc96af34febe31ed7a5703d0`, contains the exact source pins and correction. Root failed-preservation admission `a5-no-flatten-tiny-failure-root-admission-20260906.json`, SHA-256 `7cdbc33d9556e64e605a7a85b25851d11afba2abaaaed745080602e8bfa72a49`, stays failed-capability-only. No tiny rerun occurs.

Specified-only full observation: one RH002 setup-inclusive 900-second bound and Node `--max-old-space-size=4096`, with the original full CLI `compile`, original copied pilot entry, `--main=candidate_a_funding_pilot --target=json --invariant=pilotSafety --verbosity=0 --flatten=false`. No model, helper, runtime, environment, heap limit, native argv or retry changes. Node old-space is not an RSS limit. Full success or timeout is observation only: H1 remains unresolved, compilerAcceptance remains false, and this output is no solver, A4 export, equivalence, Council or completion substitute.

## Task FS001: Separate corrected capability and supplemental plan decisions

**Paths:** ROOT is `/home/charl/Moriarty/.worktrees/s01-audit-start`; META is `ROOT/.superpowers/sdd/a5-no-flatten-diagnostic-20260906`. All shorter paths in the code are resolved against ROOT, never the shell's current directory.

- [ ] A separate reviewer completes the actual corrected-output review at `.superpowers/sdd/a5-no-flatten-retained-data-independent-review-20260906.md`. It starts `PASS\n` only when the correction and all unchanged NF005 tiny predicates are verified against originals. Preserve the actual review checks and their original tool responses. A request for changes blocks this route.
- [ ] Root, separately from plan adoption, creates `META/root-admission-tiny-retained-type-application.json`. Exact consumed interface: `verdict: "PASS retained tiny capability only"`, `capabilityOrFullSuccess: true`, `fullDispatchAuthorized: false`, `nativeRetryAuthorized: false`, `compilerAcceptance: false`, `H1: "unresolved"`, and `files: [{path,bytes,sha256}, ...]` with unique absolute regular-file paths. Its files must include every known dependency checked below, all 254 corrected-intake originalArtifacts, every file referenced by the original failed-preservation admission, and actual corrected-output review evidence. The review, correction and admission are separate acts. Neither this plan nor its driver creates that admission.
- [ ] A different reviewer reviews this complete supplement at `.superpowers/sdd/a5-no-flatten-full-supplement-independent-review-20260906.md`, beginning `PASS\n` only on acceptance. Root reads the exact result and runs the `adopt` data step below. This writes `.superpowers/sdd/a5-no-flatten-full-supplement-root-adoption-20260906.json` exclusively, pinning the actual plan, review, corrected admission and full prerequisite closure. Future supplemental review/adoption hashes are computed from those real completed files, never fabricated here. Source-only adoption permits full preparation; the separate full slot and dispatch still supply native authority.

Repository observation added during authoring: the separate corrected capability admission is now complete, 86218 bytes, SHA-256 `09eeff84e1f750b4ffae73dc44173aae058a5343f3bf64801be4480ab848e0ac`, with 284 original file pins and actual root b31f8d/0. Its corrected-output review has SHA-256 `82ec3e4aa6aa0ce84c18eb7d2c8df25255076e46f97b67ea6fde96b4bbeef5c4` and embeds its original ac3d90 check. The driver pins both actual results. Supplemental review/adoption and full dispatch remain unperformed.

The original `META/root-admission-tiny.json` remains absent. The original failed intake is never overwritten or read as a successful capability. Stop if this separate interface or evidence closure is unavailable; do not relax the original failure or invent missing receipts.

## Task FS002: One finite data driver, extracted in memory

**Files:** No new executable file. The following block lives only in this plan. It extracts seven original Python blocks by exact anchored fences under the original SHA. Only blocks 0 (NF002), 1 (NF003) and 6 (full slot) change. Blocks 2 (NF004 prelaunch), 3 (NF005 full) and 4 (NF006 full) are byte-identical; block 5's old adoption is not run.

**Interfaces:** Root selects one of `adopt`, `slot`, `before`, `dispatch`, `prelaunch`, `native-source`, `after`, `intake`, `admit`. Each data call is separate, its actual args/result preserved before the next step. The native-source step prints the original NF004 JavaScript with its documented single mode selection set to full; it does not execute it. All execution uses `NF_MODE=full` and optimization zero, with `-B`.

<!-- supplement-data-driver -->
```python
import ast, datetime, hashlib, json, os, re, sys
from pathlib import Path
R=Path('/home/charl/Moriarty/.worktrees/s01-audit-start')
M=R/'.superpowers/sdd/a5-no-flatten-diagnostic-20260906'
S=R/'docs/superpowers/plans/2026-09-06-candidate-a-no-flatten-full-supplement.md'
P=R/'docs/superpowers/plans/2026-09-06-candidate-a-no-flatten-diagnostic.md'
V=R/'.superpowers/sdd/a5-no-flatten-full-supplement-independent-review-20260906.md'
D=R/'.superpowers/sdd/a5-no-flatten-full-supplement-root-adoption-20260906.json'
A=M/'root-admission-tiny-retained-type-application.json'
I=M/'tiny-retained-type-application-intake.json'
F=M/'tiny-intake.json'
FA=R/'.superpowers/sdd/a5-no-flatten-tiny-failure-root-admission-20260906.json'
CV=R/'.superpowers/sdd/a5-no-flatten-retained-data-independent-review-20260906.md'
step=os.environ['NF_SUPPLEMENT_STEP']
def require(ok,message):
    if not ok:raise RuntimeError(message)
require(sys.flags.optimize==0 and sys.dont_write_bytecode,'optimization zero and -B')
require(os.environ['NF_MODE']=='full','full mode only; no tiny rerun')
require(step in ('adopt','slot','before','dispatch','prelaunch','native-source','after','intake','admit'),'finite data step')
def read(p):return json.loads(Path(p).read_bytes())
def pin(p):
    p=Path(p);require(p.is_file() and not p.is_symlink(),'regular original: '+str(p))
    with p.open('rb') as stream:h=hashlib.file_digest(stream,'sha256').hexdigest()
    return {'path':str(p),'bytes':p.stat().st_size,'sha256':h}
def check(row):require(pin(row['path'])==row,'exact path/bytes/hash changed: '+row['path'])
known={
    P:'79325d985b5f58a62a5f490b0ac51f60c873afcb36775be3d95e372d144b2260',
    I:'f2d73ee627dd4025b7038149f2a5b99f78abb29a6e55fba44f1f4d8d121ed4aa',
    F:'2c39b46364d588dbe78f1505a657f5df003051b8a59ad67c411193becd44a988',
    FA:'7cdbc33d9556e64e605a7a85b25851d11afba2abaaaed745080602e8bfa72a49',
    R/'.superpowers/sdd/a5-no-flatten-tiny-type-application-diagnosis-20260906.md':'6d8a52ea54b5a70d1e2d6f0bd7bc72aa298c3139fc96af34febe31ed7a5703d0',
    R/'.superpowers/sdd/a5-no-flatten-tiny-type-application-diagnosis-tool-20260906.json':'743e8b0d64a8a95edcbe32ecbf185d56c704ee4bcc85bf5647882691edd91edb',
    R/'.superpowers/sdd/a5-no-flatten-retained-data-proposal-independent-review-20260906.md':'6b20f3e4fcebc8fa4993cdb1c26b568b0b5a87a8e319ef2d805bc062764be4f8',
    R/'.superpowers/sdd/a5-no-flatten-retained-data-proposal-root-adoption-20260906.json':'16ba5d306515b3fc836765bf4d36a7fc5629d4f9d87e6099cb0bd918f1f7492c',
    R/'.superpowers/sdd/a5-no-flatten-retained-data-intake-tool-20260906.json':'6bea89688f01e25b34ee4448ddc98973e1a84fe04e6150a44c2632eb8a04ac7f',
    CV:'82ec3e4aa6aa0ce84c18eb7d2c8df25255076e46f97b67ea6fde96b4bbeef5c4',
}
for path,h in known.items():require(pin(path)['sha256']==h,'known prerequisite changed')
require(pin(A)=={'path':str(A),'bytes':86218,'sha256':'09eeff84e1f750b4ffae73dc44173aae058a5343f3bf64801be4480ab848e0ac'},'actual separate corrected tiny admission')
require(not (M/'root-admission-tiny.json').exists(),'original tiny admission stays absent')
tiny=read(A);corrected=read(I);failed=read(F);failure=read(FA)
require(tiny['verdict']=='PASS retained tiny capability only' and tiny['capabilityOrFullSuccess'] is True and
        tiny['fullDispatchAuthorized'] is False and tiny['nativeRetryAuthorized'] is False and
        tiny['compilerAcceptance'] is False and tiny['H1']=='unresolved','separate corrected tiny decision')
require(corrected['capabilityOrFullSuccess'] is True and corrected['validationErrors']==[] and
        corrected['preservation'] is True and corrected['sourceEndpointsStable'] is True and
        corrected['runtimeEndpointsStable'] is True and corrected['compilerAcceptance'] is False and
        corrected['H1']=='unresolved','corrected result scope')
require(corrected['retainedDataCorrection']['nativeRerun'] is False and
        corrected['retainedDataCorrection']['fullDispatchAuthorized'] is False,'data-only correction')
require(failed['capabilityOrFullSuccess'] is False and failed['validationErrors']==[
        {'message':'retained Concrete=Box[int]','type':'RuntimeError'}],'original failed gate retained')
require(failure['capabilityAdmitted'] is False and failure['fullDispatchAuthorized'] is False and
        failure['nativeRetryAuthorized'] is False and failure['originalPinsChecked']==746 and
        failure['consumedTinySlot'] is True,'original failed-preservation scope')
require(CV.read_text().startswith('PASS\n') and V.read_text().startswith('PASS\n'),'separate corrected-output and supplemental-plan PASS')
rows=tiny['files'];by_path={row['path']:row for row in rows}
require(len(rows)==len(by_path)==284,'exact unique corrected admission files')
for row in rows:check(row)
required=(set(known)-{P})|{CV}
required.update(M/name for name in ('tiny-before/endpoint.json','tiny-after/endpoint.json',
    'tiny-before/source.tar.gz','tiny-after/source.tar.gz','root-dispatch-tiny.json',
    'tiny-dispatch-source.tar.gz','tiny-dispatch-seal.json','tiny-outer/terminal.json','transport/tiny/terminal.json'))
for row in failure['independentEvidence']+[failure['originalIntake'],failure['outerTerminal'],failure['transportTerminal']]:
    check(row);required.add(Path(row['path']))
require(len(corrected['originalArtifacts'])==254,'exact original tiny artifact set')
for row in corrected['originalArtifacts']:
    require(by_path.get(row['path'])==row,'corrected admission includes each original artifact')
require({str(p) for p in required}<=set(by_path),'complete finite correction/failure control closure')
receipt=read(R/'.superpowers/sdd/a5-no-flatten-retained-data-intake-tool-20260906.json')
require(receipt['result']['exit_code']==0 and receipt['result']['chunk_id']=='f108b4','actual retained-data terminal')
require(json.loads(receipt['result']['output'])['intake']==pin(I),'actual corrected-output binding')
closure={str(p):pin(p) for p in [S,V,A,P,*map(Path,by_path)]}
if step=='adopt':
    require(not D.exists(),'exclusive supplemental root decision')
    value={'decision':'adopt reviewed full no-flatten supplement for preparation only',
           'plan':pin(S),'review':pin(V),'correctedTinyAdmission':pin(A),
           'files':[closure[p] for p in sorted(closure)],'fullPreparationAuthorized':True,
           'nativeDispatchAuthorized':False,'nativeRetryAuthorized':False,'fullInvocations':1,
           'wallSeconds':900,'nodeOldSpaceMiB':4096,'H1':'unresolved','compilerAcceptance':False,
           'createdAt':datetime.datetime.now(datetime.timezone.utc).isoformat()}
    with D.open('x') as stream:json.dump(value,stream,indent=2,sort_keys=True);stream.write('\n')
    print(json.dumps({'ok':True,'adoption':pin(D)}));raise SystemExit(0)
decision=read(D)
require(decision['decision']=='adopt reviewed full no-flatten supplement for preparation only' and
        decision['fullPreparationAuthorized'] is True and decision['nativeDispatchAuthorized'] is False and
        decision['nativeRetryAuthorized'] is False and decision['fullInvocations']==1 and
        decision['wallSeconds']==900 and decision['nodeOldSpaceMiB']==4096,'separate root supplemental decision')
require(decision['plan']==pin(S) and decision['review']==pin(V) and decision['correctedTinyAdmission']==pin(A),'actual adopted pins')
require(decision['files']==[closure[p] for p in sorted(closure)],'exact adopted correction/control closure')
bindings=[*decision['files'],pin(D)]
for row in bindings:check(row)
if step in ('slot','before'):
    unused=[R/'.superpowers/sdd/a4-producer-receipts/a5-noflat-full-20260906',
            *[M/name for name in ('full-outer','full-parent-cache','full-before-cache','full-after-cache',
              'full-before','full-after','root-dispatch-full.json','full-dispatch-source.tar.gz',
              'full-dispatch-seal.json','full-intake.json','root-admission-full.json','transport/full')]]
    if step=='slot':unused.append(R/'.superpowers/sdd/a5-no-flatten-slot-full-20260906.json')
    require(all(not p.exists() and not p.is_symlink() for p in unused),'unused full destinations; no retry')
original=P.read_text()
blocks=re.findall(r'^```python\n(.*?)^```$',original,re.S|re.M)
require(len(blocks)==7,'exact seven original Python blocks')
expected=['04e23d1574097160175f1a4772852a05eabe80d5e9a75995703c2029e4646897',
 'd560d0ed6e9db8a2c9a1148349cb119eb7a30df4a710e1ff4c37960b4a988131',
 '427e25b0638e19e5392abc74c069e6903ae6c15bea0e203f582e98802e792dac',
 '6801bed3dc7b9cad0a0ee495f45512a6eea7d9168ef2f088db73d9901ce8004a',
 '7dc8b07678e2f1428b71a7e0d3c7019a257e6723564650ae9ceb168419791533',
 '97ce235001bb7e6eeac985b755033494e2ada5e3748cdd49ebc83bc4ab6e78a9',
 '160a27d90eecdfeeb392dbf347402f2729affa539175c1d4fb76cf027aed22d9']
require([hashlib.sha256(b.encode()).hexdigest() for b in blocks]==expected,'exact original block bytes')
def replace_once(body,old,new):
    require(body.count(old)==1,'unique explicit substitution: '+old)
    return body.replace(old,new,1)
if step=='native-source':
    native=re.findall(r'^```javascript\n(.*?)^```$',original,re.S|re.M)
    require(len(native)==1,'exact original NF004 JavaScript')
    print(replace_once(native[0],"const mode = 'tiny';","const mode = 'full';"),end='')
    raise SystemExit(0)
index={'slot':6,'before':0,'dispatch':1,'prelaunch':2,'after':0,'intake':3,'admit':4}[step]
body=blocks[index]
if index in (0,1,6):
    body=replace_once(body,"'root-admission-tiny.json'","'root-admission-tiny-retained-type-application.json'")
    body=replace_once(body,"'PASS tiny capability only'","'PASS retained tiny capability only'")
if index==0:
    body=replace_once(body,"        dynamic += [tiny_admission,*[Path(r['path']) for r in tiny['files']]]",
        "        dynamic += [tiny_admission,*[Path(r['path']) for r in tiny['files']]]\n"
        "        dynamic += [Path(r['path']) for r in supplement_bindings]")
if index==1:
    body=replace_once(body,"read(M/'tiny-intake.json')['capabilityOrFullSuccess']",
        "read(M/'tiny-retained-type-application-intake.json')['capabilityOrFullSuccess']")
if step in ('before','after'):
    require(os.environ['NF_WHEN']==step,'matching endpoint environment')
    require(sys.pycache_prefix==str(M/('full-'+step+'-cache')),'exact unused endpoint cache')
ast.parse(body,filename='<reviewed-full-supplement-'+step+'>')
try:
    exec(compile(body,'<reviewed-full-supplement-'+step+'>','exec'),
         {'__name__':'__main__','supplement_bindings':bindings})
finally:
    for row in bindings:check(row)
```

The closure contains originals and actual decision bytes, not a future digest reference. NF002 adds these to the original dynamic sources, verifies conflicts, archives each regular file and then enforces identical full-before/full-after membership and hashes. Its existing full runtime before/after validation remains unchanged. The source archive includes this extraction/substitution code in the pinned supplement itself. The original NF003 separate dispatch archive/seal remains acyclic and binds the full-before endpoint and source archive. Nothing hashes the later full transport into a prelaunch prerequisite.

## Task FS003: Exact root command and ordered gates

Use this exact command template in tools.exec_command with workdir ROOT, max_output_tokens 2500 and yield_time_ms 1000. The only finite selections are `NF_SUPPLEMENT_STEP` from the table below, `NF_WHEN` for the two endpoint calls and the documented cache argument. No source is materialized. The command extracts this plan's single marked driver in memory; its adoption and prerequisite checks run before any original block.

```bash
env -u PYTHONPATH -u PYTHONHOME -u PYTHONSTARTUP -u PYTHONINSPECT -u PYTHONOPTIMIZE -u PYTEST_ADDOPTS -u PYTEST_PLUGINS -u NODE_PATH -u NODE_OPTIONS -u NODE_COMPILE_CACHE -u LD_PRELOAD -u LD_LIBRARY_PATH -u JAVA_TOOL_OPTIONS -u _JAVA_OPTIONS -u JDK_JAVA_OPTIONS PYTHONOPTIMIZE=0 PYTHONNOUSERSITE=1 PYTHONDONTWRITEBYTECODE=1 LC_ALL=C PATH=/usr/lib/jvm/java-25-openjdk-amd64/bin:/home/charl/.foreman/tools/fnm/node-versions/v24.18.1/installation/bin:/home/charl/.npm-global/bin:/usr/bin:/bin JVM_ARGS=-Xmx4096m JVM_GC_ARGS='-XX:+UseG1GC -XX:G1PeriodicGCInterval=600000 -XX:+G1PeriodicGCInvokesConcurrent' APALACHE_JAR=/home/charl/.quint/apalache-dist-0.56.1/apalache/lib/apalache.jar NF_MODE=full NF_SUPPLEMENT_STEP=adopt /home/charl/Moriarty/.venv/bin/python -B -c 'from pathlib import Path; import re; p=Path("/home/charl/Moriarty/.worktrees/s01-audit-start/docs/superpowers/plans/2026-09-06-candidate-a-no-flatten-full-supplement.md"); s=p.read_text().split("<!-- supplement-data-driver -->\n",1)[1]; b=re.findall(r"^```python\n(.*?)^```$",s,re.S|re.M); assert len(b)==1; exec(compile(b[0],str(p),"exec"))'
```

| Order | Exact finite step selection | Required result or ownership |
|---|---|---|
| 1 | `NF_SUPPLEMENT_STEP=adopt` | Root alone after both separate reviews and corrected tiny admission; actual zero and exclusive supplemental adoption. |
| 2 | Root process observation command below | Actual saved listing inspected; all previous native work terminal. No intervening native work through full terminal. |
| 3 | `NF_SUPPLEMENT_STEP=slot` | Actual zero, original full slot with original predecessor checks including tiny's actual RH002/transport terminal and current group absence. |
| 4 | `NF_SUPPLEMENT_STEP=before NF_WHEN=before`; add `-X pycache_prefix=/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-no-flatten-diagnostic-20260906/full-before-cache` immediately after `-B` | Actual zero and endpoint `ok:true`; full 215 fixed union plus closed supplemental sources/runtime archived before dispatch. |
| 5 | `NF_SUPPLEMENT_STEP=dispatch` | Actual zero; original full dispatch argv/900/4096 and two archive bindings/seal exist. Root reads actual dispatch before launch. |
| 6 | `NF_SUPPLEMENT_STEP=native-source` | Data-only zero yields exact original NF004 JavaScript except literal full mode. Root retains the actual extracted source response. |
| 7 | `NF_SUPPLEMENT_STEP=prelaunch` | Actual zero; save exact returned tool object with `store('noflat-prelaunch-full', actualToolResponse)`. The returned args are the sole native arguments. |
| 8 | Execute the exact JavaScript returned by step 6 as functions.exec source | One native launch only. Original incremental transport handling saves every actual response before the next poll. |
| 9 | `NF_SUPPLEMENT_STEP=after NF_WHEN=after`; add `-X pycache_prefix=/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-no-flatten-diagnostic-20260906/full-after-cache` after `-B` | Only after authentic terminal and resolved ownership; actual zero and complete equal endpoints required for preservation. |
| 10 | `NF_SUPPLEMENT_STEP=intake` | Original full NF005 unchanged: zero only on full altered-command predicate success; two records a failed diagnostic. Either consumes the full slot. |
| 11 | Independent original full evidence review | Existing `.superpowers/sdd/a5-no-flatten-full-independent-review-20260906.md`; PASS must state successful altered-command observation or failed preservation accurately. |
| 12 | `NF_SUPPLEMENT_STEP=admit` | Root only; original full NF006 unchanged. No automatic retry or completion. |

Before step 3 root runs this exclusive process-list command with workdir ROOT and preserves the real response. Shell noclobber prevents replacement. Root inspects the listing and the original full slot's complete predecessor set; any intervening native command requires a separately reviewed explicit predecessor extension before proceeding.

```bash
bash -c 'set -C; exec ps -eo pid,ppid,pgid,sid,stat,lstart,args > /home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-no-flatten-slot-full-processes-20260906.txt'
```

Root preserves each data call's exact args and returned objects incrementally at fresh `.superpowers/sdd/a5-no-flatten-full-supplement-transport-20260906/STEP-command.json`, `STEP-response-NNN.json` and `STEP-terminal.json`. STEP is one of the finite data selections or `processes`; NNN is the actual zero-based response sequence. Directory creation and writes are exclusive, and responses are preserved before polling again. Keep adoption, review and data-call transport outside full endpoint directories and all prelaunch source membership. The native transport remains the original `META/transport/full/` implementation, unchanged. Preserve save failures and unsaved original objects in actual tool output; never manufacture a successful response or silently overwrite partial evidence.

## Task FS004: Receipt and stop contracts

- [ ] Before any full preparation root adopts this supplement. Before native dispatch it verifies that all original full inner/outer/cache/endpoint/dispatch/intake/transport/slot paths are unused. A consumed path is a stop, not permission to rename the run and retry. Original tiny before/after and failed/corrected outputs remain immutable.
- [ ] Root holds the native slot from process inspection through authentic full terminal, cleanup and after capture. Original NF004 transport is not interrupted for progress. An unknown terminal, lost parent, transport failure or unresolved descendant ownership invokes the original explicit root takeover; no after hashing or relaunch until ownership is accounted for. RH002's setup-inclusive timeout and existing process-group cleanup stay unchanged.
- [ ] NF005 retains strict UTF-8/duplicate-key/non-finite rejection, 128 MiB JSON acceptance bound, complete required top-level schema, exactly one serialized main, original 13 import identities, the ordered original declaration kind/name/qualifier inventory, the three generated selection bodies, and the exact PilotState field/type structure and pilot annotation, actual inner/outer/time exit distinctions, original RH002 binding and cleanup eligibility. Other original declaration bodies remain preserved in source/compiler originals but are not individually compared by NF005. No full structural predicate is replaced by the tiny correction.
- [ ] Any prelaunch gate failure stops before native. Any full terminal, including timeout, OOM, compiler error or missing/oversized JSON, consumes the one full invocation. Preserve all originals and actual failed data outputs. Continue only read-only preservation/review after a terminal failure; do not require NF005 success to admit honest failed diagnostic preservation.
- [ ] Independent review verifies the original 215 pins, supplemental archive members, all current source/runtime before/after records, archive byte identity, actual dispatch source, full original transport, inner receipt and RH002 originals. It independently validates every full predicate reached and discloses missing evidence. Root NF006 can admit failed preservation while `capabilityOrFullSuccess:false`; it never changes H1 or grants solver reuse.

## Source-only plan verification and handoff

- [ ] Author and non-author verify the original plan SHA and all seven extracted block hashes, AST-parse the driver and each finite substituted block without importing helpers or executing plan bodies, and confirm exact NF004–NF006 full block identity. Check one occurrence for every substitution and complete closure/unused-path/terminal gates.
- [ ] Record the final supplement SHA, exact static-check args/results and review outside frozen native inputs. Root reads the review and separately adopts that exact supplement. No native, corrected-intake rerun, model/runtime/helper edit, executable materialization or commit is part of this source-only authoring task.
