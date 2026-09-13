Task 5.2 cannot close against the current contract. The corrected helpers pass 86 focused tests, but full source readiness is withheld.

Candidate HEAD: `5107a6ffb45cfc498076c8fedcffe3f0345cfa6b`. Full SHA-256 manifest and historical comparison are in `/tmp/moriarty-handoff-gpt6-high-20260913.json`.

The race correction, parent boot denial and stronger ordering test are present. Preserve these as partial work; they do not establish the missing production consumer or independent prover lifetime.

| Required boundary | Result |
| --- | --- |
| Concrete one-shot runner invocation handoff | partial-helper-only; not met |
| Prover lifetime established before daemon startup | not met |
| Identity-safe external containment and deterministic fault disposition | partial-helper-only; not met |

Findings:

- **H1 (high; helper-source-readiness)** — Persistence waits can still prevent executor return after its outer and abandonment bounds. `experiments/moriarty-midnight-financial/ledger/executor-caller.mjs:107`. complete() unconditionally awaits handoffSetup; appendInvocationEvent is awaited without a deadline. An injected never-resolving append with outerTimeoutMs=5 and abandonAfterKillMs=5 remained pending after 60ms despite one kill request. Releasing the gate permitted completion. writeControlRecord is also awaited before runPrepared installs any timer. Denial persistence has the same unbounded await. Repair: Bound setup, control/event/denial persistence and completion against the same absolute deadline; cancellation must prevent late response/work after timeout. For real blocking fsync use a separately supervised persistence worker as the spec requires. Add hanging append/control/denial cases; retain unresolved facts.

- **H2 (high; helper-source-readiness)** — The pure selector can declare success without any raw main-exit observation. `experiments/moriarty-midnight-financial/ledger/fault-matrix.mjs:21`. Read-only call with rawExit:null and otherwise success facts returned {status:PROCESS_SUCCESS,exitCode:0,failureCode:null}. Null passes validation and skips both known-failure and unknown branches. Existing caller does not currently pass null, so this is a latent helper API hole, not an observed live success. Repair: Require rawExit.kind=exit and code=0 for the success path; classify unavailable/null observation as unknown. Add a missing raw-exit regression with otherwise successful evidence.

- **C1 (blocking; task-5.2-closure)** — There is no concrete production runner-to-executor handoff or authoritative result consumer. `plugins/moriarty-dev/scripts/moriarty_dev/runner.py:138`. runner.execute launches generic Popen without env/socket/store-event/result-path logic. cmd_run lines 457-464 still maps ordinary exit zero to finished. Repository source search found no non-test consumer of runExitRetainingExecutor/runProverBoundedExecutor. Required loan_executor.py and installed loan_exit_retention package are absent. JS confirmCurrentState/getPeerUid/getPeerPid are injected; reservation/stale-event tests throw the desired codes directly. Launcher ancestry uses caller-supplied launcherPid/startTicks, not an enforced binding to the actual spawned process. Parent-side boot denial exists, but required child-side boot/digest/deadline/socket/current-state validation does not. Repair: Implement and integrate the specified loan executor, existing runner/cmd_run/store consumers and actual child exchange. Keep production authentication predicates real, inject only transports. Bind the actual spawned launcher and current store/master/wallet claim. Run isolated actual runner-to-script/fake-launcher/store tests and strong-launcher environment transport test.

- **C2 (blocking; task-5.2-closure)** — Lifetime arithmetic and control serialization do not implement the required independent static PID 1 lifetime boundary. `experiments/moriarty-midnight-financial/ledger/prover-lifetime.mjs:1`. Required plugins/moriarty-dev/scripts/moriarty_dev/prover_lifetime.c is absent. JS computes timestamps, invokes a caller writer, checks parent-side entry and spawns the requested command. No static wrapper, actual wrapper entry parser/clock/no-follow validation, pinned argv, descendant supervision/reap, private PID namespace or pre-start Docker configuration validation is implemented by this candidate. The seven prover tests exercise pure arithmetic and strings only; no wrapper is compiled. Repair: Implement the specified small static wrapper and offline controlled-child tests, then connect durable authenticated control and immutable-container validation in the actual executor. Live Docker remains separately gated; no live start is necessary for source tests.

- **C3 (blocking; task-5.2-closure)** — The offline matrix does not execute collector, persistence and identity-safe cleanup; the JS caller cannot produce process success from genuine evidence. `experiments/moriarty-midnight-financial/ledger/executor-caller.mjs:87`. containment() expects PASS/FAIL under a schema whose required values are PROCESS_SUCCESS/PROCESS_FAILED/PROCESS_UNKNOWN/REFUSED. Every caller matrix hardcodes terminalEvidencePersisted=false, stop receipts=false and timer receipts=false. Thus normal zero always becomes EVIDENCE_WRITE_FAILED; parentLost/containment/timer outcomes cannot be reached there. No collector/cleanup/timer/result/history path supplies the missing facts, and no installed loan_cleanup.py exists. Tests assert helper dispositions using hand-written booleans and simulated child exits, not actual installed full-path faults. Repair: Connect actual raw-main collector, exclusive durable writers, identity-safe cleanup observations and final result validator; pass only measured facts into the selector. Run each fault-table row through the installed parser/main/runner path using isolated fixtures and fake external commands.

- **H3 (medium; helper-source-readiness)** — Runtime cleanup failures and early setup/spawn errors do not retain their ownership limitations. `experiments/moriarty-midnight-financial/ledger/executor-caller.mjs:128`. Completion swallows every removeRuntime error. Runtime/socket creation occurs before the child promise; failures after allocation or child error reject without guaranteed owned-runtime cleanup. Bounded server close improves the measured race but does not make all failure paths retain/remove their own runtime resources. Repair: Use an outer try/finally covering allocation through spawn/completion, preserve original failures, and report unresolved owned socket/directory cleanup explicitly instead of an empty catch.

Historical reports: the Opus verdict is WARNING against older bytes, not approval. The correction report accurately discloses reservation/store pass-through. Its F2/F3 labels refer to Opus F3/F5 respectively; they do not dispose of Opus provenance, ancestry, consumer and remaining coverage findings. Both protected source digests match. Only the focused 86-test result was independently rerun here.

No dispatch, K, Docker, live network, accounting change or financial acceptance was performed. The actual static wrapper, child consumer, current-state authentication and full fault orchestration remain absent.

Exact narrow-defect reproducers (run from the candidate checkout; no real child/service/filesystem transports):

H1:

```sh
node --input-type=module <<'JS'
import {EventEmitter} from 'node:events';
import {runExitRetainingExecutor} from './experiments/moriarty-midnight-financial/ledger/executor-caller.mjs';
const h='a'.repeat(64),boot='01234567-89ab-4def-8123-456789abcdef';
const server=new EventEmitter();server.listening=true;
const child=new EventEmitter();child.pid=123;child.stdout=new EventEmitter();child.stderr=new EventEmitter();
let release,kills=0;const gate=new Promise(r=>release=r);
const execution=runExitRetainingExecutor({command:'/synthetic',args:[],cwd:'/tmp',outerTimeoutMs:5,abandonAfterKillMs:5,readDurableResult:async()=>{throw Error('absent');},dependencies:{spawn:()=>child,monotonicNow:()=>1n,killGroup:()=>{kills++;}},handoff:{parentDir:'/tmp',launcherPid:123,launcherStartTicks:1n,allocationId:'a',actionId:'a',candidateHash:h,runnerDigest:h,chargeId:'c',reservationId:'r',storeIdentity:'s',executionContextSha256:h,projectionSha256:h,correspondenceSha256:h,authoritySha256:h,blockDeadlineUtc:9999999999999,blockDeadlineMonotonicNs:null,appendInvocationEvent:()=>gate,confirmCurrentState:()=>{},getPeerUid:()=>process.getuid(),getPeerPid:()=>123,setupTimeoutMs:5,readBootId:()=>boot,readProcStat:pid=>({pid,ppid:1,startTicks:1n}),createRuntimeDirectory:async()=>'/tmp/fake',createInvocationSocket:async()=>({server,socketPath:'/tmp/fake/x'}),generateNonce:()=>Buffer.alloc(32,1),removeRuntime:()=>{},appendDenialEvent:()=>{}}});
const state=await Promise.race([execution.then(()=> 'returned'),new Promise(r=>setTimeout(()=>r('still pending after 60ms'),60))]);
console.log(JSON.stringify({state,kills,outerTimeoutMs:5,abandonAfterKillMs:5}));release();await execution;
JS
```

H2:

```sh
node --input-type=module <<'JS'
import {classifyDisposition} from './experiments/moriarty-midnight-financial/ledger/fault-matrix.mjs';
const facts={refused:null,startupValid:true,startupWriteFailed:false,evidencePreexists:false,invocationIdMismatch:false,rawExit:null,terminalEvidencePersisted:true,stopReturnCode:0,stopErrorClass:null,stopReceiptPersisted:true,containmentComplete:true,timerCancelReturnCode:0,timerCancelReceiptPersisted:true,deadlineExceeded:false,parentLost:false,resultWriteFailed:false};
console.log(JSON.stringify(classifyDisposition(facts)));
JS
```
