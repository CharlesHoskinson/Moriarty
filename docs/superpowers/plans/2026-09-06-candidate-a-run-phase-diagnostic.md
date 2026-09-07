# Candidate A public run-phase diagnostic implementation plan

> **For agentic workers:** Use superpowers:executing-plans for this finite plan after independent review and root dispatch. The present assignment is source-only: no materialization, native call, helper execution or commit occurs while writing this document.

**Goal:** Preserve one newly observed case014 public run interval, after fresh mocks and four tiny direct/observed runs prove this specific observation route.

**Architecture:** A diagnostic-local five-export observer and strict launcher wrap the real CLI. Unchanged RH002 surrounds the unchanged integrated recorder. Root-owned data-only commands supply extra source/runtime endpoints and acyclic dispatch archives; there is no new recorder or generic harness.

**Tech stack:** Pinned Python3.13, Node, Quint/Rust runtime, existing RH002 and integrated recorder, standard-library source/archive/JSON inspection.

## Global constraints and finite inventory

ROOT is `/home/charl/Moriarty/.worktrees/s01-audit-start`; META is ROOT + `/.superpowers/sdd/a4-run-phase-diagnostic-20260906`; PLAN is this document's absolute path. Pinned design is `e969fc1004d1cfb4e9928604d52acc6c863c996bad58284b0ce76cc02c794e4a`, independent design review `1caa1a7c4334ffb2c9212358d4606a2d3225e6da85d917dfd2e684a0b46d8d03`, root design adoption `2d570855596f17454f038a94091f1db294a86b5071a041773a4c001583555453`.

The only ordered execution slots are `mock-identity`, `mock-open`, `mock-write`, `mock-partial`, `mock-records`, `mock-bytes`, `mock-classify`, `success-direct`, `success-observed`, `violation-direct`, `violation-observed`, `full`. Each runs once; unexpected results stop before dependent slots. Expected mock74 and tiny violation1 are control results, not positive RH002 eligibility. Mocks use Node `--max-old-space-size=128 --unhandled-rejections=strict` with RH00215seconds. Four tiny slots use RH002120seconds and no native heap/verbosity override. Full uses RH002900seconds and no native heap/verbosity override. All bounds include integrated setup, four version probes, snapshots and command execution. No retry, canonical export replacement,044,all78,final115,H1,solver or Council conclusion follows.

Do not edit any old observer/helper, model or runtime. Source endpoints bind the entire original121-source map (104 Quint files plus17 Python sources) and8102 runtime pins/four trees. Actual preparation/dispatch/inner HEADs are recorded separately from original014 source08e426c and historical producer386bf0ae/runtime900bb205. Hash stability, not a hardcoded current HEAD, controls source acceptance. Root serializes all native work with A5 and grants one exact slot at a time only after prior owned groups are terminal and absent.

## Common parent environment: required for every tool invocation

Apply this complete literal prefix before **every** parent Python invocation in this plan: materialization, before/after endpoints, dispatch construction, preflight, intake, pair comparison, admission, and transport mkdir/save. RP003 embeds the same prefix in its native `shellCommand`; the transport code embeds it for its own Python writes. A shell export in an earlier tool call does not satisfy this requirement. Append only the slot/mode variables specified by the relevant finite task, then the pinned Python executable, its existing flags, and the exact body. This prefix acts before Python startup and before shared-helper import-time Java/Node discovery; RH002's later child sanitization cannot do that retroactively.

```sh
env -u PYTHONPATH -u PYTHONHOME -u PYTHONSTARTUP -u PYTHONINSPECT -u PYTHONOPTIMIZE -u PYTEST_ADDOPTS -u PYTEST_PLUGINS -u NODE_PATH -u NODE_OPTIONS -u NODE_COMPILE_CACHE -u LD_PRELOAD -u LD_LIBRARY_PATH -u JAVA_TOOL_OPTIONS -u _JAVA_OPTIONS -u JDK_JAVA_OPTIONS -u FORCE_COLOR -u NO_COLOR PYTHONOPTIMIZE=0 PYTHONNOUSERSITE=1 PYTHONDONTWRITEBYTECODE=1 LC_ALL=C PATH=/usr/lib/jvm/java-25-openjdk-amd64/bin:/home/charl/.foreman/tools/fnm/node-versions/v24.18.1/installation/bin:/home/charl/.npm-global/bin:/usr/bin:/bin JVM_ARGS=-Xmx4096m JVM_GC_ARGS='-XX:+UseG1GC -XX:G1PeriodicGCInterval=600000 -XX:+G1PeriodicGCInvokesConcurrent' APALACHE_JAR=/home/charl/.quint/apalache-dist-0.56.1/apalache/lib/apalache.jar
```

The printed line is a command prefix, not a standalone execution or a persistent shell setting. For example the endpoint command appends `RP_SLOT=S RP_WHEN=before /home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=META/cache/S-before -` with the one authorized literal S and the absolute META path. Use the same prefix again for its after call. All other shorthand Python command descriptions below inherit this requirement. The fixed PATH selects the admitted Java25, Node24 and Quint launcher. JVM settings reproduce the existing admitted baseline; no Java solver runs. Neither NODE_OPTIONS nor a Node heap argument is added to tiny/full. Removing FORCE_COLOR and NO_COLOR leaves the pinned non-TTY rendering decision in control; no color/verbosity argument or output normalization is added. Existing mock128MiB arguments remain unchanged.

The original plan/review are preserved at `.superpowers/sdd/a4-run-phase-plan-original-20260906.md` (SHA256 b6701382d8ce19e1bb2419a65ec30ae642cee908bed80f5db2015bc666de2980) and `.superpowers/sdd/a4-run-phase-plan-independent-review-original-20260906.md` (SHA256 cc627e5fbd16893494488bf21fdca2ba0e53d00c63667e9b21d263ffecc40885). This correction requires a new independent plan review/root adoption before materialization; the original PASS did not cover this parent-environment requirement.

## Task RP001: Exact new local sources, source review and source-only materialization

Create only these five new executable/model source files after independent plan adoption: META/observer.cjs, META/launch.cjs, META/controls.cjs, META/fixtures/success.qnt, META/fixtures/violation.qnt. Retain the three closed A5 predecessor files by original hash in the support inventory. Observer mechanics are byte-identical to the predecessor except the fixed PHASES line; control admission does not transfer automatically. The observer callbacks remain non-forwarding and return the original Promise.

<!-- file: observer.cjs -->
```javascript
'use strict';
const fs = require('node:fs');
const either = require('/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/node_modules/@sweet-monads/either/cjs/index.js');
const prototype = Object.getPrototypeOf(either.right(null));
const PHASES = Object.freeze(['load', 'parse', 'typecheck', 'runSimulator', 'outputResult']);
function fail() {
  try { fs.writeSync(2, 'phase-observer: diagnostic failure\n'); } catch (_) {}
  process.exit(74);
}
function outcome(value) {
  try {
    if (Object.getPrototypeOf(value) !== prototype) fail();
    const d = Object.getOwnPropertyDescriptor(value, 'type');
    if (!d || !Object.hasOwn(d, 'value') || !['Left', 'Right'].includes(d.value)) fail();
    return d.value;
  } catch (_) { fail(); }
}
function createTrace(path, io = fs, clock = process.hrtime.bigint) {
  let fd;
  try { fd = io.openSync(path, 'wx', 0o600); } catch (_) { fail(); }
  let sequence = 0, bytes = 0, previous = -1n;
  return function emit(phase, event, result) {
    try {
      if (!PHASES.includes(phase) || !['enter', 'return', 'resolve', 'throw', 'reject'].includes(event)) fail();
      if (!['stage', 'Left', 'Right', 'undefined', 'thrown', 'rejected'].includes(result)) fail();
      const ns = clock();
      if (typeof ns !== 'bigint' || ns < 0n || ns < previous) fail();
      const raw = Buffer.from(JSON.stringify({seq: sequence + 1, phase, event, outcome: result, ns: String(ns)}) + '\n');
      if (sequence >= 32 || bytes + raw.length > 16384) fail();
      if (io.writeSync(fd, raw) !== raw.length) fail();
      sequence += 1; bytes += raw.length; previous = ns;
    } catch (_) { fail(); }
  };
}
function install(commands, emit) {
  const safeEmit = (...args) => { try { emit(...args); } catch (_) { fail(); } };
  try {
  for (const phase of PHASES) {
    const descriptor = Object.getOwnPropertyDescriptor(commands, phase);
    if (!descriptor || typeof descriptor.value !== 'function' || !descriptor.writable) fail();
    const original = descriptor.value;
    const observed = function (...args) {
      safeEmit(phase, 'enter', phase === 'outputResult' ? outcome(args[0]) : 'stage');
      let value;
      try { value = Reflect.apply(original, this, args); }
      catch (error) { safeEmit(phase, 'throw', 'thrown'); throw error; }
      if (value instanceof Promise) {
        // Return the original promise. Both observing callbacks return undefined;
        // neither forwards rejection into the otherwise unused observing promise.
        try {
          Promise.prototype.then.call(value,
            result => { safeEmit(phase, 'resolve', outcome(result)); },
            error => { safeEmit(phase, 'reject', 'rejected'); });
        } catch (_) { fail(); }
      } else {
        safeEmit(phase, 'return', phase === 'outputResult' && value === undefined ? 'undefined' : outcome(value));
      }
      return value;
    };
    Object.defineProperty(commands, phase, {...descriptor, value: observed});
  }
  } catch (_) { fail(); }
}
module.exports = {PHASES, fail, createTrace, install};
```

<!-- file: launch.cjs -->
```javascript
'use strict';
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const ROOT = '/home/charl/Moriarty/.worktrees/s01-audit-start';
const META = ROOT + '/.superpowers/sdd/a4-run-phase-diagnostic-20260906';
const NODE = '/home/charl/.foreman/tools/fnm/node-versions/v24.18.1/installation/bin/node';
const CLI = '/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cli.js';
let hooks;
try { hooks = require('./observer.cjs'); }
catch (_) { try { fs.writeSync(2,'phase-observer: diagnostic failure\n'); } catch (_) {} process.exit(74); }
const {fail, createTrace, install} = hooks;
function hashFile(filename) {
  const fd=fs.openSync(filename,'r'), h=crypto.createHash('sha256'), b=Buffer.alloc(1048576);
  try { let n; while((n=fs.readSync(fd,b,0,b.length,null))>0) h.update(b.subarray(0,n)); }
  finally { fs.closeSync(fd); }
  return h.digest('hex');
}
const launched=process.argv.slice();
let invocation;
try {
  if(launched.length!==3 || launched[0]!==NODE || launched[1]!==META+'/launch.cjs' ||
     process.execPath!==NODE || process.execArgv.length!==0 || process.cwd()!==ROOT) fail();
  const slot=path.basename(launched[2],'.json');
  if(!['success-observed','violation-observed','full'].includes(slot) ||
     launched[2]!==META+'/invocations/'+slot+'.json') fail();
  if(hashFile(NODE)!=='f3432a45b03b2da0d270095fdd8813dc34cbea73f5fc8b18c7a384b7cf9b333a' ||
     hashFile(CLI)!=='ac12595b1cb7253feec93c79417615c6eb20fc3b6a3df35c5e3530b24e90a501') fail();
  const raw=fs.readFileSync(launched[2]);
  if(raw.length>16384) fail();
  invocation=JSON.parse(raw.toString('utf8'));
  if(JSON.stringify(Object.keys(invocation).sort())!==JSON.stringify(['argvReceipt','effectiveArgv','trace'])) fail();
  const entry=slot==='full' ? 'specs/quint/s02/candidate_a_integrated_case_014.qnt' :
    '.superpowers/sdd/a4-run-phase-diagnostic-20260906/fixtures/'+slot.split('-')[0]+'.qnt';
  const out='.superpowers/sdd/a4-producer-receipts/a4-run-phase-'+slot+'-20260906/diagnostic.itf.json';
  const expected=[NODE,CLI,'run',entry,'--backend=rust','--seed=42','--max-samples=1','--n-traces=1',
    '--max-steps=27','--invariants','noDiagnosticA4','sourceInvariantA4','--witnesses','completeA4','--out-itf',out];
  const observation=META+'/observations/'+slot;
  if(JSON.stringify(invocation.effectiveArgv)!==JSON.stringify(expected) ||
     invocation.trace!==observation+'/trace.jsonl' || invocation.argvReceipt!==observation+'/argv.json') fail();
  const receipt=Buffer.from(JSON.stringify({actualLaunchArgv:launched,effectiveArgv:expected,
    execPath:process.execPath,execArgv:process.execArgv,cwd:process.cwd(),
    invocationSha256:crypto.createHash('sha256').update(raw).digest('hex')})+'\n');
  if(receipt.length>16384) fail();
  const fd=fs.openSync(invocation.argvReceipt,'wx',0o600);
  try { if(fs.writeSync(fd,receipt)!==receipt.length) fail(); } finally { fs.closeSync(fd); }
} catch (_) { fail(); }
const emit=createTrace(invocation.trace);
try {
  const commands=require('/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cliCommands.js');
  install(commands,emit);
  process.argv=invocation.effectiveArgv.slice();
  require(CLI);
} catch (_) { fail(); }
```

<!-- file: controls.cjs -->
```javascript
'use strict';
const assert=require('node:assert/strict');
const fs=require('node:fs');
const path=require('node:path');
const {PHASES,createTrace,install}=require('./observer.cjs');
const {left,right}=require('/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/node_modules/@sweet-monads/either/cjs/index.js');
const [kind,directory]=process.argv.slice(2);
assert.equal(process.argv.length,4);
assert.deepEqual(PHASES,['load','parse','typecheck','runSimulator','outputResult']);
const trace=path.join(directory,'trace.jsonl');
function memoryTrace() {
  const rows=[];
  const emit=createTrace('memory',{openSync:()=>9,writeSync:(fd,raw)=>{rows.push(JSON.parse(raw.toString()));return raw.length;}},()=>BigInt(rows.length));
  return {rows,emit};
}
function commands(fn) {return Object.fromEntries(PHASES.map(p=>[p,fn]));}
async function identity() {
  const report=[];
  for(const phase of PHASES) for(const mode of ['sync-right','sync-left','resolve','reject','throw']) {
    const {emit,rows}=memoryTrace(), receiver={}, argument={mutable:0}, extra={};
    const input=phase==='outputResult'?right(argument):argument;
    const result=mode==='sync-left'?left(argument):right(argument), error=new Error('identity sentinel');
    let calls=0,promise;
    const exports=commands(function(...args){
      calls++;assert.equal(this,receiver);assert.equal(args.length,2);assert.equal(args[0],input);assert.equal(args[1],extra);
      argument.mutable++;
      if(mode==='throw')throw error;
      if(mode==='resolve')return promise=Promise.resolve(result);
      if(mode==='reject')return promise=Promise.reject(error);
      return result;
    });
    install(exports,emit);
    if(mode==='throw')assert.throws(()=>exports[phase].call(receiver,input,extra),e=>e===error);
    else {
      const returned=exports[phase].call(receiver,input,extra);
      if(mode==='resolve'||mode==='reject'){
        assert.equal(returned,promise);
        if(mode==='resolve')assert.equal(await returned,result);
        else await assert.rejects(returned,e=>e===error);
      } else assert.equal(returned,result);
    }
    assert.equal(calls,1);assert.equal(argument.mutable,1);assert.equal(rows.length,2);
    assert.deepEqual([rows[0].phase,rows[0].event,rows[0].outcome],[phase,'enter',phase==='outputResult'?'Right':'stage']);
    assert.deepEqual([rows[1].phase,rows[1].event,rows[1].outcome],[phase,({resolve:'resolve',reject:'reject',throw:'throw'})[mode]||'return',({'sync-left':'Left',reject:'rejected',throw:'thrown'})[mode]||'Right']);
    report.push({phase,mode,calls,mutations:argument.mutable,rows});
  }
  {
    const {emit,rows}=memoryTrace();const exports=commands(()=>undefined);install(exports,emit);
    assert.equal(exports.outputResult(left(null)),undefined);
    assert.deepEqual(rows.map(r=>[r.phase,r.event,r.outcome]),[['outputResult','enter','Left'],['outputResult','return','undefined']]);
    report.push({mode:'output-undefined',rows});
  }
  for(const broken of [null,'parse','runSimulator']) {
    const {emit,rows}=memoryTrace(), called=[], argumentsSeen=[], state={};
    const exports=Object.fromEntries(PHASES.map(phase=>[phase,function(arg){
      called.push(phase);argumentsSeen.push(arg);
      if(phase==='outputResult')return undefined;
      return Promise.resolve(phase===broken?left(arg):right(arg));
    }]));
    install(exports,emit);
    await exports.load(state).then(r=>r.asyncChain(exports.parse)).then(r=>r.asyncChain(exports.typecheck))
      .then(r=>r.asyncChain(exports.runSimulator)).then(exports.outputResult);
    const wanted=broken==='parse'?['load','parse','outputResult']:PHASES;
    assert.deepEqual(called,wanted);assert.deepEqual(rows.filter(r=>r.event==='enter').map(r=>r.phase),wanted);
    assert.ok(argumentsSeen.slice(0,-1).every(x=>x===state));assert.equal(argumentsSeen.at(-1).value,state);
    assert.equal(rows.at(-2).outcome,broken?'Left':'Right');assert.equal(rows.at(-1).outcome,'undefined');
    report.push({mode:broken?'left-'+broken:'right-chain',calls:called,rows});
  }
  // One full turn under strict rejection handling proves the non-forwarding
  // observation handlers did not create an unused rejected continuation.
  await new Promise(resolve=>setImmediate(resolve));
  assert.equal(report.length,29);
  process.stdout.write(JSON.stringify({ok:true,profile:PHASES,controls:report})+'\n');
}
if(kind==='identity')identity().catch(error=>{process.stderr.write(String(error)+'\n');process.exitCode=1;});
else {
  let io=fs,clock=process.hrtime.bigint;
  if(kind==='open')fs.mkdirSync(trace);
  if(kind==='write')io={openSync:fs.openSync,writeSync:()=>{throw new Error('injected write');}};
  if(kind==='partial')io={openSync:fs.openSync,writeSync:(fd,raw)=>fs.writeSync(fd,raw.subarray(0,raw.length-1))};
  if(kind==='bytes')clock=()=>BigInt('1'.repeat(17000));
  const emit=createTrace(trace,io,clock);
  if(kind==='records')for(let i=0;i<33;i++)emit('runSimulator','enter','stage');
  else if(kind==='classify') {
    const exports=commands(()=>{fs.writeFileSync(path.join(directory,'original-called.txt'),'1\n',{flag:'wx'});return {};});
    install(exports,emit);exports.runSimulator({});
  } else if(['open','write','partial','bytes'].includes(kind)) {
    const exports=commands(()=>{fs.writeFileSync(path.join(directory,'original-called.txt'),'1\n',{flag:'wx'});return right(null);});
    install(exports,emit);exports.runSimulator({});
  } else throw new Error('unknown control');
  throw new Error('fault failed to stop observer');
}
```

<!-- file: fixtures/success.qnt -->
```quint
module success {
  var x: int
  action init = x' = 0
  action step = x' = x + 1
  val noDiagnosticA4 = true
  val sourceInvariantA4 = x >= 0
  val completeA4 = x == 27
}
```

<!-- file: fixtures/violation.qnt -->
```quint
module violation {
  var x: int
  action init = x' = 0
  action step = x' = x + 1
  val noDiagnosticA4 = true
  val sourceInvariantA4 = x < 1
  val completeA4 = x == 0
}
```

The fixtures intentionally have no imports. Both use the exact run argument shape and selected names. Success traverses28 states x=0..27 and reaches its completion witness. Violation traverses x=0,1: initial invariants hold and the second selected invariant fails after the real transition. Its witness is true initially; do not infer a post-violation evaluation order. All state and printed witness predicates are fixed below and must hold in both members of each pair. Pinned graphics.js uses80 columns for redirected stdout; its grouped State label and one-field record render on the same line as `[State i] { x: i }`, with two trailing newlines. Pinned prettierimp.js flattens each optional line to one space at that width. No generic state-label layout is assumed.

## Exact immutable support manifests

The following29 support originals close every existing extra source/data read. The original014 receipt supplies the exact121 source map and8102 runtime map; these large inventories are referenced, not copied into this plan. Its hash is fixed below. Reject duplicate/conflicting paths when combining maps. Large shared/Python archives stay at their admitted original paths and are verified by unchanged runtime interfaces.

<!-- support-pins -->
```json
{
  "/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cli.js": "ac12595b1cb7253feec93c79417615c6eb20fc3b6a3df35c5e3530b24e90a501",
  "/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cliCommands.js": "b18672d656782aefde254fe1021dc13fdffdcf8f4ad8709e3126fb5fa293362a",
  "/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cliHelpers.js": "4ad6b6364d2a5d07389f2518aa80013bfd01dbdbab6201b53fd6b042eb8a91dc",
  "/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cliReporting.js": "823889ffe67fa0132f51f2a40d40bb663d8b6280c4c1077dfe9dfb77be9ba66e",
  "/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/graphics.js": "f4a38ffdac81d1d42803bbca445142aeced61babcabe022a1871584cfcfdef2b",
  "/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/itf.js": "5085766014116354c152a8a242aad39ae8fb0c46b6bb3b447dbd484240d34495",
  "/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/jsonHelper.js": "1b99d8d050c2ed2803869b886ca2f003727e01e9e5353e8111d95bafee300a4d",
  "/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/prettierimp.js": "e96313294f47a452a59b3a3e10ceb0ca1a37a51c9f265a7f292063eb878afe31",
  "/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/rust/commandWrapper.js": "1f26d3f1c31529572b5be32276d2d94c19a3d0298dbf444891802f744e337e1c",
  "/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/simulation.js": "05ae64c6819dfa5b340cdcd5343f7eea397949835f7a9db42d915d2e0c895e62",
  "/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/verbosity.js": "3a0a9857b248b3e0d3a14e70ea04bf78f6e4cee57370bce5eafe6c8c29024423",
  "/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/node_modules/@sweet-monads/either/cjs/index.js": "4b789da6fbd347942c20977ccc140524dc8297179a3cd1d96a5b36327ad30757",
  "/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a4-checker-task1-receipts/python-environment.json": "cd004057c4067bf7cc536d2cec5038add88d0852c1a7de5030e402ff2204f9c4",
  "/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a4-largest-native-pilot-014/terminal.json": "6efe155209200a679b0739d0ae475e82be84a9cdc3a8bcac1dfea49cf2f10f5d",
  "/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a4-pilot014-failure-root-admission-20260906.json": "f0676a056c57464478954ecc9ae622acfee14102bfcec8079b6900584dc3f758",
  "/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a4-pilot014-next-diagnostic-source-proposal-20260906.md": "4258fbfc7cb9b32574f4cda14ad44829dd323dbb50150abf3b8d171cc9cf495a",
  "/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a4-pilot014-transport-20260906/terminal.json": "5473054609de2ed5cea11c0af9c724231944bcc17b47219d8196dc7c5d7f1653",
  "/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a4-producer-dispatch.json": "cc8ab817598c84a4d44c6464d9ce341d4ea7959461370634488c995858dc9147",
  "/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a4-producer-receipts/case-014-export/receipt.json": "8909e53ae4e66cfa2e9866f5aee3edcbb11792ae56eb4cc6e4c6984524ba3c06",
  "/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a4-run-phase-diagnostic-design-independent-review-20260906.md": "1caa1a7c4334ffb2c9212358d4606a2d3225e6da85d917dfd2e684a0b46d8d03",
  "/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a4-run-phase-diagnostic-design-root-adoption-20260906.json": "2d570855596f17454f038a94091f1db294a86b5071a041773a4c001583555453",
  "/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-compiler-phase-diagnostic-20260906/controls.cjs": "00499f501018da5786f1e692f0de2fc681adead0777aa4c96e15713a34b0aee0",
  "/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-compiler-phase-diagnostic-20260906/launch.cjs": "f14dc497792d6e440efa4ec974cf838e884adb0945d51c8c1844ed43578af06b",
  "/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-compiler-phase-diagnostic-20260906/observer.cjs": "59f706006a73c795c303d84e7c1e3368acd1cf8a7547bfb0d46821f0d69fbd49",
  "/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-factoring-receipts/dispatch.json": "393a8aa904bcd9473670ac48934b3dca4ed611ca9b9accbfa94f15bb4798ac2f",
  "/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-factoring-receipts/tool-store/manifest.json": "fa3e9838c66480557ed6d928a2d159ebc074d05f67f15ef08ab20388d9c49e0b",
  "/home/charl/Moriarty/.worktrees/s01-audit-start/docs/superpowers/specs/2026-09-06-candidate-a-run-phase-diagnostic-design.md": "e969fc1004d1cfb4e9928604d52acc6c863c996bad58284b0ce76cc02c794e4a",
  "/home/charl/Moriarty/.worktrees/s01-audit-start/evidence/s02-candidate-a-completion/a4/native-resources/runner.py": "d8973d3541269d1be2ce524f2482e5f5b858f15f7d9117e3fe6d5cd2171d660d",
  "/home/charl/Moriarty/.worktrees/s01-audit-start/scripts/run_s02_candidate_a_factoring_pilot.py": "816c3ad79dc3a67ca9a03729af56d74188c161a7546b9c43e821c222a4bce7f2"
}
```

## Root decisions and exclusive ownership

Independent plan review lives at ROOT + `/.superpowers/sdd/a4-run-phase-plan-independent-review-20260906.md`. Root records actual adoption at the sibling `a4-run-phase-plan-root-adoption-20260906.json` with exact fields `{plan:{path,sha256},review:{path,sha256},decision:"adopt A4 run-phase exact plan"}`. Only the root may create that decision after a real PASS review. Root's already-granted source-only design adoption is not native permission.

META initially does not exist. RP001 materialization below creates only the five exact source files, three immutable invocation JSON files, materialization.json and parent/empty observation directories. It creates no inner stage or RH002 outer. Inner stage for slot S is ROOT + `/.superpowers/sdd/a4-producer-receipts/a4-run-phase-S-20260906`; its entire tree is owned by the unchanged integrated recorder. Outer is META + `/outer/S`; RH002 alone creates and owns it. Parent cache is META + `/cache/S-parent`, endpoint prefixes `/cache/S-before` and `/cache/S-after`; all use -B and are unused prefix paths. Observation files for observed/full are META + `/observations/S/{argv.json,trace.jsonl}`, written exclusively by the launcher/sink. Mock control artifacts are META + `/mock-artifacts/KIND/`; only that control writes the folder after root creates it empty.

Root owns META/slots/S.json, endpoints/S-before/ and S-after/, dispatch/S.json, dispatch/S-source.tar.gz, dispatch/S-seal.json, intake/S.json, pairs/success.json and pairs/violation.json, admissions/{mocks,controls,full}.json and transport/. No result is written into a completed source archive or RH002-owned directory. Reviewer files under META/reviews/{sources,mocks,controls,full}.md remain distinct original decisions. After source materialization the source reviewer must produce a real `PASS\n` sources.md before any mock is authorized.

A slot decision is root-authored and pins the actual plan adoption, source review, every current predecessor terminal/outer receipt, root's original process listing, and any required control admission. Schema: `{slot:S,authorized:true,schedulerExclusive:true,bindings:[{path,sha256}],ownedPgids:[positive integers],reviewedPrerequisites:[{path,sha256}],createdAt:UTC}`. Root inspects `ps -eo pid,ppid,pgid,sid,stat,lstart,args` and preserves its exact actual response before setting schedulerExclusive. Include all actual A4/A5 predecessors current at dispatch; after transport/host loss root must inspect process ancestry and ownership, not infer disappearance. Every listed group is independently checked absent by the preparation and launch gates. No other native work may intervene in the granted slot. A lost/uncertain slot is not reused; no compiler retry is authorized.

Required prerequisite decisions are exact: first mock needs sources PASS and no live native work; each next mock needs the prior mock's admitted expected-result intake; success-direct needs root mocks admission covering all seven; success-observed needs success-direct intake; violation-direct needs the completed success pair; violation-observed needs violation-direct intake; full needs separate root controls admission covering all seven mocks, four tiny terminals and both pair checks, plus independent controls PASS. Root controls admission is not itself the full dispatch. Root separately creates slots/full.json and performs RP003.

## Task RP002: Source-only materialization and per-slot endpoint command

- [ ] After real plan adoption, run this source-only command once under pinned Python `-B`, optimization0. It imports no project/helper and runs no Node/compiler. It writes exactly five source files and three data invocations. Preserve its actual tool args/response; no content is generated by a compiler.

```python
import sys
if sys.flags.optimize!=0 or not sys.dont_write_bytecode:raise RuntimeError('optimization0 and -B required')
import hashlib,json,re
from pathlib import Path
R=Path('/home/charl/Moriarty/.worktrees/s01-audit-start');M=R/'.superpowers/sdd/a4-run-phase-diagnostic-20260906'
P=R/'docs/superpowers/plans/2026-09-06-candidate-a-run-phase-diagnostic.md'
def require(ok,msg):
    if not ok:raise RuntimeError(msg)
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def put(p,obj):
    with p.open('x') as stream:json.dump(obj,stream,indent=2,sort_keys=True);stream.write('\n')
a=json.loads((R/'.superpowers/sdd/a4-run-phase-plan-root-adoption-20260906.json').read_text())
require(a['decision']=='adopt A4 run-phase exact plan' and a['plan']['path']==str(P) and a['plan']['sha256']==sha(P),'actual root plan adoption')
require(sha(a['review']['path'])==a['review']['sha256'] and Path(a['review']['path']).read_text().startswith('PASS\n'),'independent plan PASS')
blocks=re.findall(r'^<!-- file: ([^ ]+) -->\n```(?:javascript|quint)\n(.*?)^```$',P.read_text(),re.S|re.M)
require(len(blocks)==5 and {n for n,_ in blocks}=={'observer.cjs','launch.cjs','controls.cjs','fixtures/success.qnt','fixtures/violation.qnt'},'exact five source blocks')
M.mkdir(exist_ok=False)
for name in ['fixtures','invocations','observations','mock-artifacts','cache','outer','slots','endpoints','dispatch','intake','pairs','admissions','reviews','transport']:(M/name).mkdir()
files=[]
for name,source in blocks:
    p=M/name
    with p.open('xb') as stream:stream.write(source.encode())
    files.append({'path':str(p),'bytes':p.stat().st_size,'sha256':sha(p)})
N='/home/charl/.foreman/tools/fnm/node-versions/v24.18.1/installation/bin/node';C='/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cli.js'
for slot in ['success-observed','violation-observed','full']:
    (M/'observations'/slot).mkdir()
    entry='specs/quint/s02/candidate_a_integrated_case_014.qnt' if slot=='full' else '.superpowers/sdd/a4-run-phase-diagnostic-20260906/fixtures/'+slot.split('-')[0]+'.qnt'
    out='.superpowers/sdd/a4-producer-receipts/a4-run-phase-'+slot+'-20260906/diagnostic.itf.json'
    argv=[N,C,'run',entry,'--backend=rust','--seed=42','--max-samples=1','--n-traces=1','--max-steps=27','--invariants','noDiagnosticA4','sourceInvariantA4','--witnesses','completeA4','--out-itf',out]
    p=M/'invocations'/(slot+'.json');put(p,{'effectiveArgv':argv,'argvReceipt':str(M/'observations'/slot/'argv.json'),'trace':str(M/'observations'/slot/'trace.jsonl')})
    files.append({'path':str(p),'bytes':p.stat().st_size,'sha256':sha(p)})
put(M/'materialization.json',{'plan':a['plan'],'files':files,'sourceFiles':5,'dataInvocations':3,'nativeExecuted':False})
print(json.dumps({'ok':True,'materializationSha256':sha(M/'materialization.json'),'sources':5,'invocations':3,'nativeExecuted':False}))
```

- [ ] Source reviewer checks exact bytes against the five plan blocks and all invocation arrays before granting sources PASS. No closed observer or helper changes. Compare the observer predecessor after replacing only its PHASES line; Promise callback bodies and original-return statement must match. Check the launcher restores argv before require(CLI), never calls stages directly and has no global handler.
- [ ] For every authorized S, execute the following inline data command once with `RP_SLOT=S RP_WHEN=before`, then after its authentic terminal and cleanup once with `RP_SLOT=S RP_WHEN=after`. Use `PYTHONOPTIMIZE=0 PYTHON -B -X pycache_prefix=META/cache/S-WHEN -` with the exact body below. These are finite data calls, not an installed helper/recorder. It calls only the unchanged integrated snapshot/runtime interfaces, never main/tool or old A5 record. Shared helper import performs its existing `which java` lookup; no version probe/compiler is invoked by this data command. The integrated **native** invocation later performs its four original version probes inside RH002's bound.

```python
import sys
if sys.flags.optimize!=0 or not sys.dont_write_bytecode:raise RuntimeError('optimization0 and -B required')
import datetime,hashlib,importlib.util,io,json,os,re,subprocess,tarfile
from pathlib import Path
R=Path('/home/charl/Moriarty/.worktrees/s01-audit-start');M=R/'.superpowers/sdd/a4-run-phase-diagnostic-20260906'
P=R/'docs/superpowers/plans/2026-09-06-candidate-a-run-phase-diagnostic.md'
ORDER=['mock-identity','mock-open','mock-write','mock-partial','mock-records','mock-bytes','mock-classify','success-direct','success-observed','violation-direct','violation-observed','full']
s=os.environ['RP_SLOT'];when=os.environ['RP_WHEN']
if s not in ORDER or when not in ('before','after'):raise RuntimeError('finite slot/endpoint only')
if Path(sys.pycache_prefix or '')!=M/'cache'/(s+'-'+when):raise RuntimeError('exact endpoint prefix')
def require(ok,msg):
    if not ok:raise RuntimeError(msg)
def read(p):return json.loads(Path(p).read_text())
def hashfile(p):
    h=hashlib.sha256()
    with Path(p).open('rb') as stream:
        while b:=stream.read(1048576):h.update(b)
    return h.hexdigest()
def pin(p):
    p=Path(p);return {'path':str(p),'bytes':p.stat().st_size,'sha256':hashfile(p)}
def check(row):require(hashfile(row['path'])==row['sha256'],'changed pin:'+row['path'])
def put(p,value):
    with p.open('x') as stream:json.dump(value,stream,indent=2,sort_keys=True);stream.write('\n')
def absent(n):
    require(type(n) is int and n>0,'positive PGID')
    try:os.killpg(n,0)
    except ProcessLookupError:return
    raise RuntimeError('owned group exists')
adoption=R/'.superpowers/sdd/a4-run-phase-plan-root-adoption-20260906.json';a=read(adoption)
require(a['decision']=='adopt A4 run-phase exact plan' and a['plan']['path']==str(P),'root plan decision')
for row in [a['plan'],a['review']]:check(row)
require(Path(a['review']['path']).read_text().startswith('PASS\n'),'independent plan review')
slot_path=M/'slots'/(s+'.json');slot=read(slot_path)
require(slot['slot']==s and slot['authorized'] is True and slot['schedulerExclusive'] is True,'root exclusive native slot')
require(slot['bindings'] and slot['ownedPgids'],'complete predecessor listing')
for row in slot['bindings']+slot['reviewedPrerequisites']:check(row)
for n in slot['ownedPgids']:absent(n)
inner=R/('.superpowers/sdd/a4-producer-receipts/a4-run-phase-'+s+'-20260906');outer=M/'outer'/s
if when=='before':require(not inner.exists() and not outer.exists(),'unused native stages')
else:
    terminal=read(outer/'terminal.json');transport=read(M/'transport'/s/'terminal.json')
    require(type(transport['responses'][-1].get('exit_code')) is int,'actual outer terminal')
    require(terminal['cleanup']['complete'] is True and not terminal['cleanup']['errors'],'complete original group cleanup')
    if terminal['owned_pgid'] is not None:absent(terminal['owned_pgid'])
    else:require(terminal['launch_error'] is not None,'missing PID only on genuine launch failure')
# No artifact/source/runtime final hashing before the ownership gates above.
D=M/'endpoints'/(s+'-'+when);D.mkdir(exist_ok=False)
out={'slot':s,'endpoint':when,'createdAt':datetime.datetime.now(datetime.timezone.utc).isoformat(),
     'actualHead':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),'ok':False,'errors':[]}
try:
    support=json.loads(P.read_text().split('<!-- support-pins -->\n```json\n',1)[1].split('\n```',1)[0]);require(len(support)==29,'support inventory')
    for path,h in support.items():check({'path':path,'sha256':h})
    old=read(R/'.superpowers/sdd/a4-producer-receipts/case-014-export/receipt.json')
    require(old['sources_before']==old['sources_after'] and len(old['sources_before'])==121,'authentic original121 closure')
    expected={str(R/n):h for n,h in old['sources_before'].items()}
    for path,h in support.items():require(path not in expected or expected[path]==h,'conflicting source/support pin');expected[path]=h
    materialization=read(M/'materialization.json');require(materialization['plan']==a['plan'],'materialized exact adopted plan')
    blocks=re.findall(r'^<!-- file: ([^ ]+) -->\n```(?:javascript|quint)\n(.*?)^```$',P.read_text(),re.S|re.M)
    require(len(blocks)==5,'exact materialized source count')
    for name,source in blocks:require((M/name).read_bytes()==source.encode(),'exact materialized source bytes')
    require(len(materialization['files'])==8 and len({r['path'] for r in materialization['files']})==8,'five sources and three invocation data files')
    require({r['path'] for r in materialization['files']}=={str(M/n) for n,_ in blocks}|{str(M/'invocations'/(x+'.json')) for x in ['success-observed','violation-observed','full']},'exact source/invocation inventory')
    for row in materialization['files']:check(row);expected[row['path']]=row['sha256']
    require((M/'reviews/sources.md').read_text().startswith('PASS\n'),'independent source review')
    extra=[P,adoption,Path(a['review']['path']),M/'materialization.json',M/'reviews/sources.md',slot_path]
    extra += [Path(r['path']) for r in slot['bindings']+slot['reviewedPrerequisites']]
    for p in extra:
        h=hashfile(p);require(str(p) not in expected or expected[str(p)]==h,'conflicting extra input');expected[str(p)]=h
    if when=='after':
        previous=read(M/'endpoints'/(s+'-before')/'endpoint.json')
        require(previous['ok'] is True and set(previous['sourcePins'])==set(expected),'complete same before source set')
        expected=previous['sourcePins']
    for path,h in expected.items():check({'path':path,'sha256':h})
    spec=importlib.util.spec_from_file_location('existing_integrated_for_run_phase',R/'scripts/record_s02_candidate_a_integrated.py')
    m=importlib.util.module_from_spec(spec);sys.path.insert(0,str(R));spec.loader.exec_module(m)
    sources,missing=m.snapshot(D/'integrated')
    require(sources==old['sources_before'] and missing==[],'exact121 current snapshot')
    try:
        producer,shared,manifest,expected_runtime,helper,verified=m.runtime_expected()
        observed=m.runtime_observed(manifest,expected_runtime)
        require(len(expected_runtime)==8102 and observed==old['runtime_before']==old['runtime_after'],'complete unchanged runtime map/four trees')
        require(observed=={'files':expected_runtime,'treeMembers':manifest['treeMembers']},'runtime exact membership')
        require(producer['beforeDispatchCommit']=='386bf0ae10f767e051b414a7231be105cc4b0f71' and shared['beforeDispatchCommit']=='900bb2051225b4a3d99bf422c3b2e5e386e3e7bc','historical bases')
        out['runtime']={'producer':producer,'shared':shared,'manifest':manifest,'expected':expected_runtime,'observed':observed,'verified':verified}
    except BaseException as error:
        out['runtimeError']={'type':type(error).__name__,'message':str(error)}
        if when=='after':out['observedDespiteVerifierFailure']=m.runtime_observed(previous['runtime']['manifest'],previous['runtime']['expected'])
        raise
    if when=='after':require(out['runtime']==previous['runtime'],'identical root runtime endpoints')
    rows=[];archive=D/'source.tar.gz'
    with tarfile.open(archive,'x:gz') as tar:
        for name,h in sorted(expected.items()):
            p=Path(name);require(p.is_file() and not p.is_symlink(),'regular source original');b=p.read_bytes()
            require(hashlib.sha256(b).hexdigest()==h,'source changed during archive')
            member=tarfile.TarInfo(name.lstrip('/'));member.size=len(b);member.mode=0o644;tar.addfile(member,io.BytesIO(b))
            rows.append({'path':name,'archivePath':member.name,'bytes':len(b),'sha256':h})
    with tarfile.open(archive,'r:gz') as tar:
        members=tar.getmembers();by_name={r['archivePath']:r for r in rows}
        require(len(members)==len(rows) and {x.name for x in members}==set(by_name) and all(x.isfile() for x in members),'exact source archive membership')
        for member in members:require(hashlib.sha256(tar.extractfile(member).read()).hexdigest()==by_name[member.name]['sha256'],'source archive bytes')
    for path,h in expected.items():check({'path':path,'sha256':h})
    out.update(ok=True,sourcePins=expected,integrated121=sources,sourceArchive=pin(archive),sourceMembers=rows,slotDecision=pin(slot_path))
except BaseException as error:out['errors'].append({'type':type(error).__name__,'message':str(error)})
put(D/'endpoint.json',out)
print(json.dumps({'ok':out['ok'],'endpoint':pin(D/'endpoint.json'),'nativeExecuted':False}))
raise SystemExit(0 if out['ok'] else 2)
```

Actual before archives are produced before each native dispatch. A failed endpoint stays partial and blocks dispatch/admission. On timeout the independent after endpoint can close runtime/source provenance without inventing an inner after snapshot or inner receipt. Missing/partial original receipts remain missing/partial. Endpoint equality establishes endpoint stability only. Each data tool's actual argv, response and exit is preserved outside endpoint directories; those transport files are not silently inserted into finalized archives.

## Task RP003: Root dispatch, acyclic seal and one original tool invocation

- [ ] Root alone runs this exact data-only command with RP_SLOT=S after the successful before endpoint and actual prerequisite reviews. It creates one dispatch and a separate one-member dispatch archive. Primary source archive → before endpoint → dispatch → dispatch archive/seal → native launch is acyclic. No prior archive is rewritten. Use pinned Python -B with optimization0.

```python
import sys
if sys.flags.optimize!=0:raise RuntimeError('optimization0 required')
import datetime,hashlib,io,json,os,shlex,subprocess,tarfile
from pathlib import Path
R=Path('/home/charl/Moriarty/.worktrees/s01-audit-start');M=R/'.superpowers/sdd/a4-run-phase-diagnostic-20260906'
ORDER=['mock-identity','mock-open','mock-write','mock-partial','mock-records','mock-bytes','mock-classify','success-direct','success-observed','violation-direct','violation-observed','full']
s=os.environ['RP_SLOT'];i=ORDER.index(s)
def require(ok,msg):
    if not ok:raise RuntimeError(msg)
def read(p):return json.loads(Path(p).read_text())
def pin(p):
    p=Path(p);h=hashlib.sha256()
    with p.open('rb') as stream:
        while b:=stream.read(1048576):h.update(b)
    return {'path':str(p),'bytes':p.stat().st_size,'sha256':h.hexdigest()}
def put(p,v):
    with p.open('x') as stream:json.dump(v,stream,indent=2,sort_keys=True);stream.write('\n')
bp=M/'endpoints'/(s+'-before')/'endpoint.json';before=read(bp)
require(before['ok'] is True,'complete before endpoint')
for p,h in before['sourcePins'].items():require(pin(p)['sha256']==h,'source input changed')
def bound_read(p):
    require(str(p) in before['sourcePins'] and pin(p)['sha256']==before['sourcePins'][str(p)],'new prerequisite absent from before archive')
    return read(p)
slot=bound_read(M/'slots'/(s+'.json'))
require(slot['authorized'] is True and slot['schedulerExclusive'] is True,'separate actual root authorization')
for pgid in slot['ownedPgids']:
    try:os.killpg(pgid,0)
    except ProcessLookupError:continue
    raise RuntimeError('prior group remains')
if i>0:require(bound_read(M/'intake'/(ORDER[i-1]+'.json'))['expectedResultPass'] is True,'prior finite control passed')
if s=='success-direct':require(bound_read(M/'admissions/mocks.json')['verdict']=='PASS seven run-profile mocks only','independent mocks gate')
if s=='violation-direct':require(bound_read(M/'pairs/success.json')['ok'] is True,'successful native pair gate')
if s=='full':
    admission=bound_read(M/'admissions/controls.json')
    require(admission['verdict']=='PASS seven mocks and four tiny run controls only' and admission['fullDispatchAuthorized'] is False,'separate controls gate')
    for row in admission['files']:require(pin(row['path'])==row,'control admission original changed')
P='/home/charl/Moriarty/.venv/bin/python';N='/home/charl/.foreman/tools/fnm/node-versions/v24.18.1/installation/bin/node';C='/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/cli.js'
inner=R/('.superpowers/sdd/a4-producer-receipts/a4-run-phase-'+s+'-20260906');outer=M/'outer'/s
require(not inner.exists() and not outer.exists() and not (M/'cache'/(s+'-parent')).exists(),'exclusive stages/cache')
if s.startswith('mock-'):
    kind=s[5:];folder=M/'mock-artifacts'/kind;folder.mkdir(exist_ok=False)
    command=[N,'--max-old-space-size=128','--unhandled-rejections=strict',str(M/'controls.cjs'),kind,str(folder)]
    effective=None;wall=15;expected=0 if kind=='identity' else 74
else:
    entry='specs/quint/s02/candidate_a_integrated_case_014.qnt' if s=='full' else '.superpowers/sdd/a4-run-phase-diagnostic-20260906/fixtures/'+s.split('-')[0]+'.qnt'
    output='.superpowers/sdd/a4-producer-receipts/a4-run-phase-'+s+'-20260906/diagnostic.itf.json'
    effective=[N,C,'run',entry,'--backend=rust','--seed=42','--max-samples=1','--n-traces=1','--max-steps=27','--invariants','noDiagnosticA4','sourceInvariantA4','--witnesses','completeA4','--out-itf',output]
    if s=='full':
        original=bound_read(R/'.superpowers/sdd/a4-producer-receipts/case-014-export/receipt.json')['executed_command']
        require(effective[:-1]==original[:-1] and effective[-1]!=original[-1],'sole full CLI change is fresh ITF destination')
    command=effective
    if s.endswith('-observed') or s=='full':
        invocation=M/'invocations'/(s+'.json');spec=bound_read(invocation)
        require(spec=={'effectiveArgv':effective,'argvReceipt':str(M/'observations'/s/'argv.json'),'trace':str(M/'observations'/s/'trace.jsonl')},'exact immutable invocation')
        require(not any((M/'observations'/s).iterdir()),'unused observation paths')
        command=[N,str(M/'launch.cjs'),str(invocation)]
    wall=900 if s=='full' else 120;expected=None if s=='full' else (1 if s.startswith('violation') else 0)
inside=[P,'-B','-X','pycache_prefix='+str(inner/'python-cache'),str(R/'scripts/record_s02_candidate_a_integrated.py'),'--stage','a4-run-phase-'+s+'-20260906','--',*command]
argv=[P,'-B','-X','pycache_prefix='+str(M/'cache'/(s+'-parent')),str(R/'evidence/s02-candidate-a-completion/a4/native-resources/runner.py'),'--receipt-dir',str(outer),'--inner-receipt',str(inner/'receipt.json'),'--wall-seconds',str(wall),'--cwd',str(R),'--',*inside]
parent_removed=['PYTHONPATH','PYTHONHOME','PYTHONSTARTUP','PYTHONINSPECT','PYTHONOPTIMIZE','PYTEST_ADDOPTS','PYTEST_PLUGINS','NODE_PATH','NODE_OPTIONS','NODE_COMPILE_CACHE','LD_PRELOAD','LD_LIBRARY_PATH','JAVA_TOOL_OPTIONS','_JAVA_OPTIONS','JDK_JAVA_OPTIONS','FORCE_COLOR','NO_COLOR']
parent_fixed={'PYTHONOPTIMIZE':'0','PYTHONNOUSERSITE':'1','PYTHONDONTWRITEBYTECODE':'1','LC_ALL':'C','PATH':'/usr/lib/jvm/java-25-openjdk-amd64/bin:'+str(Path(N).parent)+':/home/charl/.npm-global/bin:/usr/bin:/bin','JVM_ARGS':'-Xmx4096m','JVM_GC_ARGS':'-XX:+UseG1GC -XX:G1PeriodicGCInterval=600000 -XX:+G1PeriodicGCInvokesConcurrent','APALACHE_JAR':'/home/charl/.quint/apalache-dist-0.56.1/apalache/lib/apalache.jar'}
parent_prefix=['env',*[part for key in parent_removed for part in ['-u',key]],*[k+'='+v for k,v in parent_fixed.items()]]
dispatch={'slot':s,'authorized':True,'invocations':1,'createdAt':datetime.datetime.now(datetime.timezone.utc).isoformat(),
          'actualDispatchHead':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),
          'beforeEndpoint':pin(bp),'sourceArchive':before['sourceArchive'],'slotDecision':before['slotDecision'],
          'command':command,'effectiveArgv':effective,'innerArgv':inside,'rh002Argv':argv,'inner':str(inner),'outer':str(outer),
          'wallSeconds':wall,'expectedControlExit':expected,'cwd':str(R),'parentRemoved':parent_removed,'parentFixed':parent_fixed,'shellCommand':shlex.join([*parent_prefix,*argv]),
          'scope':'diagnostic only; inherited Node handling unchanged through RH002/integrated; no canonical acceptance'}
path=M/'dispatch'/(s+'.json');put(path,dispatch);raw=path.read_bytes();archive=M/'dispatch'/(s+'-source.tar.gz')
with tarfile.open(archive,'x:gz') as tar:
    member=tarfile.TarInfo(str(path).lstrip('/'));member.size=len(raw);member.mode=0o644;tar.addfile(member,io.BytesIO(raw))
with tarfile.open(archive,'r:gz') as tar:
    ms=tar.getmembers();require(len(ms)==1 and ms[0].isfile() and ms[0].name==str(path).lstrip('/') and tar.extractfile(ms[0]).read()==raw,'exact dispatch archive')
seal={'dispatch':pin(path),'archive':pin(archive),'beforeEndpoint':pin(bp),'sourceArchive':before['sourceArchive']}
put(M/'dispatch'/(s+'-seal.json'),seal)
print(json.dumps({'ok':True,'dispatch':seal['dispatch'],'seal':pin(M/'dispatch'/(s+'-seal.json'))}))
```

RH002 starts the time-wrapped integrated recorder in its new session. Integrated subprocess.run inherits that group, and the launcher loads the real CLI in the same Node process. The Rust child also inherits the group. RH002's fixed environment clears inherited NODE_/PYTHON/PYTEST/LD_ keys as in failed014; integrated sanitization stays unchanged. No NODE_OPTIONS/heap/verbosity change is introduced for tiny/full. Mock128MiB is an explicit isolated control argument, not a claimed native default or RSS limit. The actual RH002 removed/fixed environment fields, parent argv, cache flags, selected tool paths and unchanged helper source are audited from original launch/inner receipts. Require a clean non-TTY parent with FORCE_COLOR absent; do not introduce a color/verbosity argument or silently strip arbitrary terminal escapes.

- [ ] Immediately before dispatch, root rechecks all before source pins, seal hashes, unchanged runtime scheduling assumption, empty native destinations and every slot-owned PGID absent. The exact source-only preflight is the read/check portion of RP003 through the prerequisite gates, **without executing its exclusive writes a second time**: reviewers may use the following complete read-only gate instead. It produces the exact exec args and stores no fake terminal.

```python
import sys
if sys.flags.optimize!=0:raise RuntimeError('optimization0 required')
import hashlib,json,os
from pathlib import Path
R=Path('/home/charl/Moriarty/.worktrees/s01-audit-start');M=R/'.superpowers/sdd/a4-run-phase-diagnostic-20260906';s=os.environ['RP_SLOT']
if s not in ['mock-identity','mock-open','mock-write','mock-partial','mock-records','mock-bytes','mock-classify','success-direct','success-observed','violation-direct','violation-observed','full']:raise RuntimeError('finite slot only')
def read(p):return json.loads(Path(p).read_text())
def check(row):
    h=hashlib.sha256()
    with Path(row['path']).open('rb') as stream:
        while b:=stream.read(1048576):h.update(b)
    if h.hexdigest()!=row['sha256']:raise RuntimeError('prelaunch pin mismatch')
seal=read(M/'dispatch'/(s+'-seal.json'))
for row in seal.values():check(row)
d=read(seal['dispatch']['path']);before=read(seal['beforeEndpoint']['path'])
if d['slot']!=s or d['authorized'] is not True or d['invocations']!=1 or before['ok'] is not True:raise RuntimeError('one exact authorized invocation')
for p,h in before['sourcePins'].items():check({'path':p,'sha256':h})
slot=read(d['slotDecision']['path'])
for pgid in slot['ownedPgids']:
    try:os.killpg(pgid,0)
    except ProcessLookupError:continue
    raise RuntimeError('prior writer group exists')
if Path(d['inner']).exists() or Path(d['outer']).exists() or (M/'transport'/s).exists():raise RuntimeError('exclusive invocation already used')
if 'FORCE_COLOR' in os.environ:raise RuntimeError('unexpected color override; root source review required')
print(json.dumps({'ok':True,'slot':s,'dispatch':seal['dispatch'],'args':{'cmd':d['shellCommand'],'workdir':str(R),'yield_time_ms':1000,'max_output_tokens':2500}}))
```

The functions.exec that runs this preflight stores its **actual returned tool object** with `store('run-phase-preflight-'+S, response)` and preserves that response outside finalized endpoints. Then execute this finite orchestrator body with the literal slot S from the ordered list. No native retry or source file is created by the orchestrator. Each actual native response is flushed before the next poll. Root uses yielded cells for progress, never kills a monitor cell to harvest buffered output.

```javascript
const slot='mock-identity'; // The exact separately root-authorized slot; replace only with the next admitted literal.
const original=load('run-phase-preflight-'+slot);
if(!original || original.exit_code!==0)throw Error('actual successful preflight required');
const launch=JSON.parse(original.output);
if(launch.slot!==slot || launch.ok!==true)throw Error('preflight identity');
const root='/home/charl/Moriarty/.worktrees/s01-audit-start';
const folder=root+'/.superpowers/sdd/a4-run-phase-diagnostic-20260906/transport/'+slot;
const q=s=>"'"+String(s).replaceAll("'","'\\''")+"'";
const python='/home/charl/Moriarty/.venv/bin/python';
const parentRemoved=['PYTHONPATH','PYTHONHOME','PYTHONSTARTUP','PYTHONINSPECT','PYTHONOPTIMIZE','PYTEST_ADDOPTS','PYTEST_PLUGINS','NODE_PATH','NODE_OPTIONS','NODE_COMPILE_CACHE','LD_PRELOAD','LD_LIBRARY_PATH','JAVA_TOOL_OPTIONS','_JAVA_OPTIONS','JDK_JAVA_OPTIONS','FORCE_COLOR','NO_COLOR'];
const parentFixed=['PYTHONOPTIMIZE=0','PYTHONNOUSERSITE=1','PYTHONDONTWRITEBYTECODE=1','LC_ALL=C','PATH=/usr/lib/jvm/java-25-openjdk-amd64/bin:/home/charl/.foreman/tools/fnm/node-versions/v24.18.1/installation/bin:/home/charl/.npm-global/bin:/usr/bin:/bin','JVM_ARGS=-Xmx4096m','JVM_GC_ARGS=-XX:+UseG1GC -XX:G1PeriodicGCInterval=600000 -XX:+G1PeriodicGCInvokesConcurrent','APALACHE_JAR=/home/charl/.quint/apalache-dist-0.56.1/apalache/lib/apalache.jar'];
const dataPrefix=['env',...parentRemoved.flatMap(k=>['-u',k]),...parentFixed,python,'-B'].map(q).join(' ');
const make=await tools.exec_command({cmd:dataPrefix+' -c '+q('from pathlib import Path; import sys; Path(sys.argv[1]).mkdir(exist_ok=False)')+' '+q(folder),workdir:root,max_output_tokens:1000});
text(make);if(make.exit_code!==0)throw Error('transport creation failed; no native launched');
async function save(name,value){
 const body='from pathlib import Path; import sys; f=Path(sys.argv[1]).open("x"); f.write(sys.argv[2]); f.close()';
 const response=await tools.exec_command({cmd:dataPrefix+' -c '+q(body)+' '+q(folder+'/'+name)+' '+q(JSON.stringify(value,null,2)+'\n'),workdir:root,max_output_tokens:1000});
 if(response.exit_code!==0){text({saveFailure:response,unsavedOriginal:value});throw Error('root takeover required; do not relaunch');}
}
await save('preflight.json',original);await save('command.json',launch);
let response=await tools.exec_command(launch.args);const responses=[response];await save('response-000.json',response);
while(response.session_id!==undefined && response.exit_code===undefined){
 response=await tools.write_stdin({session_id:response.session_id,chars:'',yield_time_ms:10000,max_output_tokens:2500});
 responses.push(response);await save('response-'+String(responses.length-1).padStart(3,'0')+'.json',response);
 if(responses.length%6===0)notify({slot,returnedResponses:responses.length,terminal:false});
}
if(!Number.isInteger(response.exit_code))throw Error('unknown actual terminal; root takeover');
await save('terminal.json',{args:launch.args,responses,actualOuterExit:response.exit_code,scope:'original tool transport, not diagnostic acceptance'});
text(response);
```

On save/transport/host failure preserve the actual returned error and unsaved original in the live tool output, leave missing metadata missing, and require root takeover. RH002 has no parent signal handler and exposes its PGID only in terminal.json. Never infer cleanup from no output or from sidecar hashes it may have emitted. Root inspects process ancestry, PIDs/PGIDs/session ownership and resolves writers before after hashing. No new cleanup hook, global handler, process supervisor or native retry is introduced.

## Task RP004: Authenticate originals before interpreting exits or phase data

- [ ] After each actual outer terminal and settled group, run its after endpoint. Then run this inline pinned-Python -B data intake with RP_SLOT=S. All hashes stream; stdout/ITF content parsing has a128MiB ceiling, native phase/argv files16KiB and32 phase rows, mock identity output1MiB. Larger originals remain intact and receive artifact pins but no bounded content admission. No native/semantic checker or archived helper code executes. The only reused parser is unchanged live RH002.parse_resource after its source pin is verified.

```python
import sys
if sys.flags.optimize!=0:raise RuntimeError('optimization0 required')
import hashlib,importlib.util,json,math,os,re
from pathlib import Path
R=Path('/home/charl/Moriarty/.worktrees/s01-audit-start');M=R/'.superpowers/sdd/a4-run-phase-diagnostic-20260906';s=os.environ['RP_SLOT']
PHASES=['load','parse','typecheck','runSimulator','outputResult'];LIMIT=128*1024*1024

def require(ok,msg):
    if not ok:raise RuntimeError(msg)
def read(p):return json.loads(Path(p).read_text())
def pin(p):
    p=Path(p);require(p.is_file() and not p.is_symlink(),'regular original')
    h=hashlib.sha256();size=0
    with p.open('rb') as stream:
        while b:=stream.read(1048576):h.update(b);size+=len(b)
    return {'path':str(p),'bytes':size,'sha256':h.hexdigest()}
def check(row):require(pin(row['path'])==row,'exact original pin')
def require_hash(path,h):require(pin(path)['sha256']==h,'original hash mismatch')
def integer(x,n):require(type(x) is int and x==n,'authentic integer result')
def absent(n):
    require(type(n) is int and n>0,'positive PGID')
    try:os.killpg(n,0)
    except ProcessLookupError:return
    raise RuntimeError('live or unresolved writer')
def unique(pairs):
    d={}
    for k,v in pairs:require(k not in d,'duplicate JSON key');d[k]=v
    return d
def bad(x):raise ValueError('non-JSON constant '+x)
def finite(x):
    n=float(x);require(math.isfinite(n),'nonfinite JSON number');return n
def bounded_bytes(p,limit):
    with Path(p).open('rb') as stream:b=stream.read(limit+1)
    require(len(b)<=limit,'content exceeds reviewed bound');return b
def strict(p,limit=LIMIT):return json.loads(bounded_bytes(p,limit).decode('utf-8'),object_pairs_hook=unique,parse_constant=bad,parse_float=finite)
def put(p,obj):
    with p.open('x') as stream:json.dump(obj,stream,indent=2,sort_keys=True);stream.write('\n')
d=read(M/'dispatch'/(s+'.json'));inner=Path(d['inner']);outer=Path(d['outer']);td=M/'transport'/s
terminal=read(outer/'terminal.json');transport=read(td/'terminal.json')
require(terminal['cleanup']['complete'] is True and terminal['cleanup']['errors']==[],'complete owned cleanup')
if terminal['owned_pgid'] is not None:absent(terminal['owned_pgid'])
else:require(terminal['launch_error'] is not None,'no PID only on real launch failure')
require(type(transport['responses'][-1].get('exit_code')) is int,'actual outer terminal required')
# First artifact hashes occur only after settled ownership.
integer(transport['actualOuterExit'],transport['responses'][-1]['exit_code']);integer(terminal['return_code'],transport['actualOuterExit'])
responses=sorted(td.glob('response-*.json'))
require([p.name for p in responses]==['response-%03d.json'%i for i in range(len(responses))] and [read(p) for p in responses]==transport['responses'],'all actual returned responses')
for row in transport['responses'][:-1]:require('exit_code' not in row and row['session_id']==transport['responses'][0]['session_id'],'original same-session nonterminal')
require(transport['args']==read(td/'command.json')['args'] and transport['args']['cmd']==d['shellCommand'] and transport['args']['workdir']==str(R),'exact dispatch/transport')
before=read(M/'endpoints'/(s+'-before')/'endpoint.json');after=read(M/'endpoints'/(s+'-after')/'endpoint.json')
require(before['ok'] is True and after['ok'] is True and before['sourcePins']==after['sourcePins'] and before['runtime']==after['runtime'],'complete frozen before/after endpoints')
for ep in [before,after]:check(ep['sourceArchive'])
for row in read(M/'dispatch'/(s+'-seal.json')).values():check(row)
launch=read(outer/'launch.json')
require(launch['child_argv']==d['innerArgv'] and launch['original_parent_argv']==d['rh002Argv'] and launch['wrapper_argv']==d['rh002Argv'][4:],'exact capture argv')
require(launch['argv']==['/usr/bin/time','-v','-o',str(outer/'resources.txt'),*d['innerArgv']] and launch['cwd']==str(R) and launch['inner_receipt']==str(inner/'receipt.json'),'measured argv/cwd/inner')
require(launch['wall_seconds']==terminal['wall_seconds']==d['wallSeconds'] and launch['grace_seconds']==launch['cleanup_seconds']==5,'fixed bounds')
require(launch['fixed_environment']=={'PYTHONNOUSERSITE':'1','PYTHONDONTWRITEBYTECODE':'1','NODE_DISABLE_COMPILE_CACHE':'1','LC_ALL':'C'},'unchanged RH002 environment')
require(launch['source_runtime_before']==terminal['source_runtime_before']==terminal['source_runtime_after'] and terminal['source_runtime_stable'] is True,'RH002 source/runtime pins')
for p,h in terminal['sidecar_sha256'].items():require_hash(p,h)
require(terminal['inner_receipt_complete'] is False and terminal['inner_receipt_independent_validation_required'] is True,'RH002 does not validate inner body')
extra=M/'mock-artifacts'/s[5:] if s.startswith('mock-') else M/'observations'/s
artifacts=[];directories=[]
for base in [inner,outer,extra]:
    if not base.exists():continue
    for p in sorted(base.rglob('*')):
        require(not p.is_symlink(),'unexpected original artifact link')
        if p.is_file():artifacts.append(pin(p))
        elif p.is_dir():directories.append(str(p))
        else:raise RuntimeError('nonregular original artifact')
out={'slot':s,'preservation':True,'expectedResultPass':False,'originalArtifacts':artifacts,'originalDirectories':directories,
     'actualOuterExit':transport['actualOuterExit'],'actualTimeWrappedExit':terminal['actual_exit'],'actualCompilerOrMockExit':None,
     'innerReceiptAuthenticated':False,'innerReceiptPresent':(inner/'receipt.json').is_file(),'validationErrors':[],
     'phaseLocationAvailable':False,'compilerAcceptance':False,'canonicalExportAcceptance':False,'H1':'unresolved',
     'beforeEndpoint':pin(M/'endpoints'/(s+'-before')/'endpoint.json'),'afterEndpoint':pin(M/'endpoints'/(s+'-after')/'endpoint.json'),
     'dispatchSeal':pin(M/'dispatch'/(s+'-seal.json')),'outerTerminal':pin(outer/'terminal.json'),'transportTerminal':pin(td/'terminal.json')}
receipt=None
try:
    require((inner/'receipt.json').is_file() and terminal['inner_receipt_present'] is True,'inner receipt missing')
    require_hash(inner/'receipt.json',terminal['inner_receipt_sha256'])
    candidate=strict(inner/'receipt.json')
    require(candidate['command']==candidate['executed_command']==d['command'] and candidate['cwd']==str(R),'independent inner command/cwd')
    require(candidate['source_stable'] is True and candidate['runtime_stable'] is True and candidate['sources_before']==candidate['sources_after']==before['integrated121'],'independent121/source stable body')
    require(candidate['not_yet_created_before']==candidate['not_yet_created_after']==[],'no omitted121 inputs')
    for when in ['before','after']:
        closure=read(inner/when/'closure.json');require(closure=={'sources':before['integrated121'],'not_yet_created':[]},'exact inner closure')
        copied=inner/when/'source';files={str(p.relative_to(copied)):p for p in copied.rglob('*') if p.is_file()}
        require(set(files)==set(before['integrated121']),'exact121 copied files')
        for name,h in before['integrated121'].items():require_hash(files[name],h)
    require(candidate['runtime_before']==candidate['runtime_after']==before['runtime']['observed'] and candidate['shared_runtime_before']==candidate['shared_runtime_after']==before['runtime']['verified'],'independent complete runtime body')
    require(candidate['beforeDispatchCommit']=='386bf0ae10f767e051b414a7231be105cc4b0f71','historical producer base')
    require(candidate['python_cache_prefix']==str(inner/'python-cache') and candidate['python_bytecode_writes'] is False,'exact inner cache/no writes')
    require(all(re.fullmatch(r'[0-9a-f]{40}',candidate[k]) for k in ['sourceCommit','sourceCommitAfter']),'actual separate HEAD fields')
    require(type(candidate['exit_code']) is int and type(candidate['recorder_exit_code']) is int,'authentic inner integer exits')
    require(candidate['recorder_exit_code']==(128-candidate['exit_code'] if candidate['exit_code']<0 else candidate['exit_code']),'original recorder exit normalization')
    require_hash(inner/'stdout.bin',candidate['stdout_sha256']);require_hash(inner/'stderr.bin',candidate['stderr_sha256'])
    actual_itf={p.name:pin(p)['sha256'] for p in inner.glob('*.itf.json')}
    require(candidate['artifacts']==actual_itf,'unchanged actual ITF glob pins')
    versions={r['name']:r for r in before['runtime']['manifest']['versions']}
    require(set(candidate['tools'])=={'node','quint','rust','python'},'four original setup probes')
    for name,tool in candidate['tools'].items():
        ref=versions[name];require(tool['version_exit']==ref['exitCode'] and hashlib.sha256(tool['version_stdout'].encode()).hexdigest()==ref['stdoutSha256'] and hashlib.sha256(tool['version_stderr'].encode()).hexdigest()==ref['stderrSha256'],'admitted actual probe results')
        require(before['runtime']['expected'][tool['path']]==tool['sha256'],'selected actual executable')
    # Only now may an exit field be credited, including negative/nonzero outcomes.
    receipt=candidate;out.update(innerReceiptAuthenticated=True,actualCompilerOrMockExit=receipt['exit_code'],
                                innerSourceCommit=receipt['sourceCommit'],innerSourceCommitAfter=receipt['sourceCommitAfter'])
except (OSError,ValueError,RuntimeError,KeyError,TypeError,RecursionError) as error:
    out['innerReceiptValidationError']={'type':type(error).__name__,'message':str(error)}
# No fallback copies exit_code from a merely present/partial/unvalidated JSON body.
try:
    require(receipt is not None,'control/success needs authenticated inner receipt')
    require(not terminal['timed_out'] and not terminal['cleanup']['forced'] and terminal['launch_error'] is None and terminal['monitor_error'] is None,'normal unforced control lifecycle')
    integer(terminal['actual_exit'],receipt['recorder_exit_code']);integer(terminal['return_code'],receipt['recorder_exit_code'])
    require(terminal['resource_complete'] is True and terminal['resource'] is not None,'complete resource receipt')
    rh_path=R/'evidence/s02-candidate-a-completion/a4/native-resources/runner.py';require_hash(rh_path,'d8973d3541269d1be2ce524f2482e5f5b858f15f7d9117e3fe6d5cd2171d660d')
    spec=importlib.util.spec_from_file_location('existing_run_phase_resource_parser',rh_path);rh=importlib.util.module_from_spec(spec);spec.loader.exec_module(rh)
    require(rh.parse_resource(outer/'resources.txt',d['innerArgv'])==terminal['resource'],'independent original resources')
    require(terminal['resource']['exit_status']==receipt['recorder_exit_code'],'resource/actual exit binding')
    require((outer/'stderr.bin').read_bytes()==b'','clean integrated recorder stderr')
    summary=read(outer/'stdout.bin')
    require(summary=={'stage':str(inner),'exit_code':receipt['exit_code'],'source_stable':True,'runtime_stable':True,'recorder_exit_code':receipt['recorder_exit_code'],'artifacts':receipt['artifacts']},'original inner recorder summary')
    if s!='full':integer(receipt['exit_code'],d['expectedControlExit'])
    else:integer(receipt['exit_code'],0)
    # RH002 eligible is false for genuine expected nonzero controls; never relabel it.
    require(terminal['eligible'] is (receipt['exit_code']==0),'honest positive/expected-negative eligibility')
    if s.startswith('mock-'):
        kind=s[5:];require(receipt['artifacts']=={},'no mock ITF')
        stderr=(inner/'stderr.bin').read_bytes();stdout=bounded_bytes(inner/'stdout.bin',1048576)
        if kind=='identity':
            report=json.loads(stdout,object_pairs_hook=unique,parse_constant=bad)
            require(stderr==b'' and report['ok'] is True and report['profile']==PHASES and len(report['controls'])==29,'full run-profile identity report')
            modes=['sync-right','sync-left','resolve','reject','throw']
            for i,row in enumerate(report['controls'][:25]):
                phase=PHASES[i//5];mode=modes[i%5]
                require(row['phase']==phase and row['mode']==mode and row['calls']==row['mutations']==1,'receiver/argument/mutation identity')
                wanted=[(phase,'enter','Right' if phase=='outputResult' else 'stage'),(phase,{'resolve':'resolve','reject':'reject','throw':'throw'}.get(mode,'return'),{'sync-left':'Left','reject':'rejected','throw':'thrown'}.get(mode,'Right'))]
                require([(r['phase'],r['event'],r['outcome']) for r in row['rows']]==wanted,'identity exact observation')
            require(report['controls'][25]['mode']=='output-undefined' and [(r['event'],r['outcome']) for r in report['controls'][25]['rows']]==[('enter','Left'),('return','undefined')],'outputResult undefined identity')
            for row,mode,calls in zip(report['controls'][26:],['right-chain','left-parse','left-runSimulator'],[PHASES,['load','parse','outputResult'],PHASES]):
                require(row['mode']==mode and row['calls']==calls and [r['phase'] for r in row['rows'] if r['event']=='enter']==calls,'genuine asyncChain route')
                require(row['rows'][-2]['outcome']==('Right' if mode=='right-chain' else 'Left') and row['rows'][-1]['outcome']=='undefined','chain ending')
            out['identityChecks']=29
        else:
            require(stdout==b'' and stderr==b'phase-observer: diagnostic failure\n','authentic expected observer74')
            called=extra/'original-called.txt';require(called.is_file()==(kind=='classify'),'original call presence')
            if kind=='classify':require(called.read_bytes()==b'1\n','one original classify call')
            if kind=='open':require((extra/'trace.jsonl').is_dir(),'exclusive open fault')
            if kind in ['write','bytes']:require((extra/'trace.jsonl').read_bytes()==b'','zero-byte fault trace')
            if kind=='partial':require((extra/'trace.jsonl').read_bytes() and not (extra/'trace.jsonl').read_bytes().endswith(b'\n'),'partial trace retained')
            if kind in ['records','classify']:
                rows=[json.loads(b,object_pairs_hook=unique) for b in bounded_bytes(extra/'trace.jsonl',16384).splitlines()]
                require(len(rows)==(32 if kind=='records' else 1),'fault record bound')
                for i,row in enumerate(rows,1):require(row['seq']==i and (row['phase'],row['event'],row['outcome'])==('runSimulator','enter','stage'),'fault event identities')
        out['expectedResultPass']=True
    elif s!='full':
        itf=strict(inner/'diagnostic.itf.json');kind=s.split('-')[0];n=28 if kind=='success' else 2
        require(receipt['artifacts']=={'diagnostic.itf.json':pin(inner/'diagnostic.itf.json')['sha256']},'sole diagnostic ITF')
        require(set(itf)=={'#meta','vars','states'} and itf['vars']==['x'],'exact actual toItf top fields/variables')
        require(itf['states']==[{'#meta':{'index':i},'x':{'#bigint':str(i)}} for i in range(n)],'complete deterministic original state sequence')
        meta=itf['#meta'];require(set(meta)=={'format','format-description','source','status','description','timestamp'},'exact actual addItfHeader fields')
        require(meta['format']=='ITF' and meta['format-description']=='https://apalache-mc.org/docs/adr/015adr-trace.html' and meta['source']==d['effectiveArgv'][3] and meta['status']==('ok' if kind=='success' else 'violation'),'source/status header')
        require(type(meta['timestamp']) is int and receipt['started_ns']//1000000<=meta['timestamp']<=receipt['ended_ns']//1000000,'authentic generated timestamp interval')
        require(type(meta['description']) is str and re.fullmatch(r'Created by Quint on (Mon|Tue|Wed|Thu|Fri|Sat|Sun) [A-Z][a-z]{2} [0-9]{2} [0-9]{4} [0-9]{2}:[0-9]{2}:[0-9]{2} GMT[+-][0-9]{4} \([^\n]+\)',meta['description']),'specific generated Date description')
        stdout=bounded_bytes(inner/'stdout.bin',1048576).decode('utf-8');stderr=bounded_bytes(inner/'stderr.bin',1048576)
        require('\x1b' not in stdout and '\r' not in stdout,'plain captured non-TTY output')
        require(stderr==(b'' if kind=='success' else b'error: Invariant violated\n'),'exact real CLI error stream')
        require([(int(i),int(x)) for i,x in re.findall(r'^\[State ([0-9]+)\] \{ x: ([0-9]+) \}$',stdout,re.M)]==[(i,i) for i in range(n)],'complete actual80-column printed state inventory')
        require(stdout.startswith('An example execution:\n\n'+''.join('[State %d] { x: %d }\n\n'%(i,i) for i in range(n))),'actual graphics/prettier complete state prefix')
        status_pattern=r'^\[ok\] No violation found \([0-9]+ms at (?:[0-9]+|Infinity) traces/second\)\.$' if kind=='success' else r'^\[violation\] Found an issue \([0-9]+ms at (?:[0-9]+|Infinity) traces/second\)\.$'
        require(len(re.findall(status_pattern,stdout,re.M))==1,'exact result/duration line')
        require('completeA4 was witnessed in 1 trace(s) out of 1 explored (100.00%)\n' in stdout,'one selected witness reported from a real trace')
        if kind=='success':require('Trace length statistics: max=28, min=28, average=28.00\n' in stdout,'success trace statistics')
        else:require('  ❌ sourceInvariantA4\n' in stdout and '  ❌ noDiagnosticA4\n' not in stdout,'actual second-invariant violation')
        require((itf['states'][-1]['x']['#bigint']=='27') if kind=='success' else (itf['states'][0]['x']['#bigint']=='0' and itf['states'][1]['x']['#bigint']=='1'),'success completion / real violating transition')
        out.update(expectedResultPass=True,itf=pin(inner/'diagnostic.itf.json'),states=n,witnessReported=1)
    else:
        require('diagnostic.itf.json' in receipt['artifacts'],'full success diagnostic artifact')
        # Content is diagnostic only: no carrier, correspondence or canonical export check.
        full_itf=strict(inner/'diagnostic.itf.json')
        require(type(full_itf) is dict and type(full_itf.get('states')) is list and full_itf.get('#meta',{}).get('status')=='ok','bounded full diagnostic ITF')
        out['fullBoundedITF']=pin(inner/'diagnostic.itf.json')
        out['expectedResultPass']=True
except (OSError,ValueError,RuntimeError,KeyError,TypeError,RecursionError) as error:out['validationErrors'].append({'type':type(error).__name__,'message':str(error)})

# Phase interpretation is separate from native success, including timeout/OOM.
if s.endswith('-observed') or s=='full':
    try:
        invocation=M/'invocations'/(s+'.json');specification=read(invocation);argv=strict(extra/'argv.json',16384)
        require(argv=={'actualLaunchArgv':d['command'],'effectiveArgv':d['effectiveArgv'],'execPath':d['effectiveArgv'][0],'execArgv':[],'cwd':str(R),'invocationSha256':pin(invocation)['sha256']},'actual restored CLI argv receipt')
        require(specification=={'effectiveArgv':d['effectiveArgv'],'argvReceipt':str(extra/'argv.json'),'trace':str(extra/'trace.jsonl')},'immutable observed invocation')
        raw=bounded_bytes(extra/'trace.jsonl',16384);require(raw and raw.endswith(b'\n'),'nonempty complete raw phase JSONL')
        rows=[json.loads(line,object_pairs_hook=unique,parse_constant=bad) for line in raw.splitlines()]
        require(1<=len(rows)<=32,'phase record bound')
        previous=-1;expected_phase='load';waiting=False;finished=False;last_completed=None;last_entered=None;pending_outcome=None
        for i,row in enumerate(rows,1):
            require(set(row)=={'seq','phase','event','outcome','ns'} and type(row['seq']) is int and row['seq']==i,'exact raw phase schema/sequence')
            require(type(row['ns']) is str and re.fullmatch(r'[0-9]+',row['ns']) and int(row['ns'])>=previous,'nonnegative monotonic clock');previous=int(row['ns'])
            require(not finished,'no invented phase after terminal observation')
            if not waiting:
                require(row['phase']==expected_phase and row['event']=='enter' and row['outcome']==(pending_outcome if expected_phase=='outputResult' else 'stage'),'actual next public entry')
                last_entered=expected_phase;waiting=True
                if expected_phase=='outputResult':finished=True
            else:
                require(row['phase']==expected_phase and ((row['event']=='resolve' and row['outcome'] in ['Left','Right']) or (row['event']=='reject' and row['outcome']=='rejected')),'actual async public settlement')
                last_completed={'phase':row['phase'],'event':row['event'],'outcome':row['outcome']};waiting=False
                if row['event']=='reject':finished=True
                elif row['outcome']=='Left':expected_phase='outputResult';pending_outcome='Left'
                elif expected_phase=='runSimulator':expected_phase='outputResult';pending_outcome='Right'
                else:expected_phase=PHASES[PHASES.index(expected_phase)+1]
        stderr_marker=False
        if (inner/'stderr.bin').is_file():
            with (inner/'stderr.bin').open('rb') as stream:
                tail=b''
                while chunk:=stream.read(1048576):
                    combined=tail+chunk
                    if b'phase-observer: diagnostic failure\n' in combined:stderr_marker=True
                    tail=combined[-64:]
        observer_failure=out['actualCompilerOrMockExit']==74 or stderr_marker
        out.update(traceValid=True,phaseRows=rows,observerFailureKnown=observer_failure,innerExitKnown=out['innerReceiptAuthenticated'])
        if not observer_failure:
            out.update(phaseLocationAvailable=True,lastEntered=last_entered,lastCompleted=last_completed,
                       typecheckResolvedRight=any((r['phase'],r['event'],r['outcome'])==('typecheck','resolve','Right') for r in rows),
                       runSimulatorEntered=any((r['phase'],r['event'])==('runSimulator','enter') for r in rows),
                       intervalMeaning='observed public interval includes observer I/O and Promise scheduling; no historical/internal localization')
        if s!='full':
            wanted=[]
            for phase in PHASES[:-1]:wanted += [(phase,'enter','stage'),(phase,'resolve','Left' if phase=='runSimulator' and s.startswith('violation') else 'Right')]
            wanted += [('outputResult','enter','Left' if s.startswith('violation') else 'Right')]
            require([(r['phase'],r['event'],r['outcome']) for r in rows]==wanted and not observer_failure,'exact tiny real Right/Left five-profile route')
    except (OSError,ValueError,RuntimeError,KeyError,TypeError,RecursionError) as error:
        out['phaseLocationAvailable']=False;out['traceValidationError']={'type':type(error).__name__,'message':str(error)}
        out['expectedResultPass']=False
# Direct tiny calls must have no observation files; the pair closes exact differences.
if s.endswith('-direct'):require(not extra.exists(),'direct call has no observer output')
put(M/'intake'/(s+'.json'),out)
print(json.dumps({'slot':s,'intake':pin(M/'intake'/(s+'.json')),'preservation':True,'expectedResultPass':out['expectedResultPass'],'phaseLocationAvailable':out['phaseLocationAvailable'],'canonicalExportAcceptance':False}))
raise SystemExit(0 if (s=='full' or out['expectedResultPass']) else 2)
```

Expected native full failure can finish the data-preservation command with0 while expectedResultPass is false; the three original exits remain separately recorded. Mock/tiny unexpected predicates produce actual intake2 and stop the route. A malformed/missing phase file or known observer74 supplies no phase location. A valid partial trace on a timeout can retain the last observed public entries even when the inner exit is unknown; the report records that unknown explicitly and makes no observer-success or compiler-success claim. An incomplete runSimulator contains expression resolution, evaluator discovery, request serialization, Rust execution, parsing/conversion and ITF handling; none is distinguished by entry alone.

## Task RP005: Exact direct/observed pair comparison

- [ ] After success-observed intake, run this data-only command with `RP_PAIR=success`. After violation-observed intake, run it once with `RP_PAIR=violation`. Use `/home/charl/Moriarty/.venv/bin/python -B -` with optimization0 and ROOT cwd. These checks parse existing originals; they execute no Quint, JavaScript, helper or archived code. Any nonzero result stops before the next native slot. Preserve the actual data command and terminal response separately under META/transport; do not rewrite an intake or its originals.

The allowed ITF differences are exactly `#meta.description` and `#meta.timestamp`, each already constrained by RP004's actual serializer schema and authentic native interval. The stdout allowance is exactly the elapsed-millisecond/rate text inside the one status line emitted by runSimulator. The status words, punctuation, every other byte, all state fields and all other header fields must match. Neither source paths nor arbitrary metadata fields are dropped. The two fresh --out-itf destinations differ by their fixed slot names; source input, seed, selections and every other effective CLI argument are identical.

```python
import sys
if sys.flags.optimize!=0:raise RuntimeError('optimization0 required')
import copy,hashlib,json,math,os,re
from pathlib import Path
R=Path('/home/charl/Moriarty/.worktrees/s01-audit-start');M=R/'.superpowers/sdd/a4-run-phase-diagnostic-20260906'
kind=os.environ['RP_PAIR']
if kind not in ['success','violation']:raise RuntimeError('two finite pairs only')
def require(ok,msg):
    if not ok:raise RuntimeError(msg)
def pin(p):
    p=Path(p);require(p.is_file() and not p.is_symlink(),'regular original');h=hashlib.sha256();size=0
    with p.open('rb') as f:
        while b:=f.read(1048576):h.update(b);size+=len(b)
    return {'path':str(p),'bytes':size,'sha256':h.hexdigest()}
def check(row):require(pin(row['path'])==row,'exact retained original')
def unique(pairs):
    d={}
    for k,v in pairs:require(k not in d,'duplicate key');d[k]=v
    return d
def bad(x):raise ValueError('non-JSON constant '+x)
def finite(x):
    n=float(x);require(math.isfinite(n),'finite number');return n
def raw(p,n):
    with Path(p).open('rb') as f:b=f.read(n+1)
    require(len(b)<=n,'reviewed content bound');return b
def read(p,n=128*1024*1024):return json.loads(raw(p,n).decode('utf-8'),object_pairs_hook=unique,parse_constant=bad,parse_float=finite)
slots=[kind+'-direct',kind+'-observed'];intakes=[];dispatches=[];itfs=[];stdout=[];stderr=[];files=[];times=[];endpoint_core=[]
for slot in slots:
    ip=M/'intake'/(slot+'.json');a=read(ip);d=read(M/'dispatch'/(slot+'.json'));inner=Path(d['inner'])
    require(a['slot']==slot and a['preservation'] is True and a['expectedResultPass'] is True and a['innerReceiptAuthenticated'] is True,'both original intakes admitted expected result')
    expected=0 if kind=='success' else 1
    require(type(a['actualCompilerOrMockExit']) is int and a['actualCompilerOrMockExit']==a['actualTimeWrappedExit']==a['actualOuterExit']==expected,'authentic expected three exits')
    require(a['validationErrors']==[] and a['compilerAcceptance'] is False and a['canonicalExportAcceptance'] is False and a['H1']=='unresolved','no diagnostic promotion')
    for row in a['originalArtifacts']+[a[k] for k in ['beforeEndpoint','afterEndpoint','dispatchSeal','outerTerminal','transportTerminal']]:check(row)
    terminal=read(a['outerTerminal']['path']);require(terminal['cleanup']['complete'] is True and terminal['cleanup']['forced'] is False and terminal['cleanup']['errors']==[] and terminal['timed_out'] is False,'settled unforced tiny group')
    pgid=terminal['owned_pgid'];require(type(pgid) is int and pgid>0,'owned tiny group')
    try:os.killpg(pgid,0)
    except ProcessLookupError:pass
    else:raise RuntimeError('tiny writer remains')
    expected_names={str(p) for base in [inner,Path(d['outer']),M/'observations'/slot] if base.exists() for p in base.rglob('*') if p.is_file()}
    require(expected_names=={r['path'] for r in a['originalArtifacts']} and len(expected_names)==len(a['originalArtifacts']),'no extra or omitted tiny original')
    for before_after in ['beforeEndpoint','afterEndpoint']:
        ep=read(a[before_after]['path']);require(ep['ok'] is True,'complete endpoint');check(ep['sourceArchive'])
    ep=read(a['beforeEndpoint']['path']);materialized=read(M/'materialization.json')
    require(all(ep['sourcePins'].get(row['path'])==row['sha256'] for row in materialized['files']),'all eight materialized sources/data frozen')
    endpoint_core.append({'integrated121':ep['integrated121'],'runtime':ep['runtime'],'materialized':materialized['files']})
    effective=d['effectiveArgv'];require(len(effective)==16 and effective[-2]=='--out-itf' and effective[-1]==str(inner.relative_to(R)/'diagnostic.itf.json'),'exact fresh output argument')
    require(effective[3]==str((M/'fixtures'/(kind+'.qnt')).relative_to(R)),'same original tiny input')
    if slot.endswith('-direct'):
        require(d['command']==effective and not (M/'observations'/slot).exists() and a['phaseLocationAvailable'] is False,'real direct CLI route')
    else:
        require(d['command']==[effective[0],str(M/'launch.cjs'),str(M/'invocations'/(slot+'.json'))],'real observed launcher route')
        require(a['traceValid'] is True and a['phaseLocationAvailable'] is True and a['observerFailureKnown'] is False and len(a['phaseRows'])==9,'complete observed run profile')
    itf=read(inner/'diagnostic.itf.json');check(a['itf'])
    require(set(itf)=={'#meta','vars','states'} and set(itf['#meta'])=={'format','format-description','source','status','description','timestamp'},'no ignored serializer field')
    n=28 if kind=='success' else 2
    require(itf['vars']==['x'] and itf['states']==[{'#meta':{'index':i},'x':{'#bigint':str(i)}} for i in range(n)],'complete real states')
    times.append({k:itf['#meta'][k] for k in ['description','timestamp']})
    normalized=copy.deepcopy(itf)
    del normalized['#meta']['description'];del normalized['#meta']['timestamp']
    itfs.append(normalized);stdout.append(raw(inner/'stdout.bin',1048576).decode('utf-8'));stderr.append(raw(inner/'stderr.bin',1048576))
    files += [pin(ip),pin(M/'dispatch'/(slot+'.json')),*a['originalArtifacts'],*[a[k] for k in ['beforeEndpoint','afterEndpoint','dispatchSeal','outerTerminal','transportTerminal']]]
    intakes.append(a);dispatches.append(d)
require(endpoint_core[0]==endpoint_core[1],'same original121/runtime/materialized inputs across pair')
require(dispatches[0]['effectiveArgv'][:-1]==dispatches[1]['effectiveArgv'][:-1] and dispatches[0]['effectiveArgv'][-1]!=dispatches[1]['effectiveArgv'][-1],'sole effective argument difference is fresh destination')
require(itfs[0]==itfs[1],'every other parsed ITF value identical')
prefix=r'\[ok\] No violation found' if kind=='success' else r'\[violation\] Found an issue'
pattern=re.compile(r'^('+prefix+r' \()([0-9]+ms at (?:[0-9]+|Infinity) traces/second)(\)\.)$',re.M)
normalized_stdout=[];durations=[]
for text in stdout:
    matches=list(pattern.finditer(text));require(len(matches)==1,'one exact status timing substring')
    match=matches[0];durations.append(match.group(2));normalized_stdout.append(text[:match.start(2)]+'<runSimulator-duration-and-rate>'+text[match.end(2):])
require(normalized_stdout[0]==normalized_stdout[1] and stderr[0]==stderr[1],'every remaining stdout/stderr byte identical')
by_path={}
for row in files:
    require(row['path'] not in by_path or by_path[row['path']]==row,'conflicting original');by_path[row['path']]=row
result={'pair':kind,'ok':True,'slots':slots,'originalFiles':[by_path[p] for p in sorted(by_path)],'states':28 if kind=='success' else 2,
        'nativeExits':[a['actualCompilerOrMockExit'] for a in intakes],'witnessReported':[a['witnessReported'] for a in intakes],
        'allowedDifferences':{'itfHeaderTimeFields':times,'singleStdoutDurationAndRate':durations,'outItfDestinations':[d['effectiveArgv'][-1] for d in dispatches]},
        'allOtherITFValuesEqual':True,'allOtherStdoutBytesEqual':True,'stderrBytesEqual':True,
        'canonicalExportAcceptance':False,'compilerAcceptance':False,'H1':'unresolved'}
p=M/'pairs'/(kind+'.json')
with p.open('x') as f:json.dump(result,f,indent=2,sort_keys=True);f.write('\n')
print(json.dumps({'pair':kind,'ok':True,'receipt':pin(p),'nativeExecuted':False}))
```

## Task RP006: Independent control admission and final diagnostic preservation

- [ ] A nonauthor reviews the actual seven mock intakes and original receipts before writing META/reviews/mocks.md beginning `PASS\n`. Expected exits are identity0 and six separate observer74 exits, with RH002 eligibility preserved honestly. Root may then create admissions/mocks.json using the exact data-only body below and `RP_ADMIT=mocks`.
- [ ] A nonauthor reviews the four actual tiny terminals, both complete pair comparisons, source/runtime endpoints, and the actual seven mock results before writing META/reviews/controls.md beginning `PASS\n`. Root may then run the same body with `RP_ADMIT=controls`. This produces no native dispatch. Only a later independent slot/full.json authorizes RP003's single900-second call.
- [ ] After full's authentic terminal, cleanup, after endpoint and RP004 preservation, a nonauthor reviews original failures as well as any successful data and writes META/reviews/full.md beginning `PASS\n` only for preservation. Root runs `RP_ADMIT=full` to preserve that narrow conclusion. A valid partial trace can establish only the last observed public interval in this new execution. Missing/invalid trace or diagnostic74 means unavailable phase location. An unavailable/unauthenticated inner exit remains JSON null and is never inferred from the RH002 exit.

For each admission, use pinned Python -B optimization0 with ROOT cwd. This finite code validates original memberships and archive bytes without importing any archived source. It binds the reviewer file as an original; root must first verify that the named file is the actual independent review, not an author-generated verdict. Any failed data command and its partial files stay retained; stop instead of rewriting/retrying an admission.

```python
import sys
if sys.flags.optimize!=0:raise RuntimeError('optimization0 required')
import datetime,hashlib,json,os,tarfile
from pathlib import Path
R=Path('/home/charl/Moriarty/.worktrees/s01-audit-start');M=R/'.superpowers/sdd/a4-run-phase-diagnostic-20260906'
mode=os.environ['RP_ADMIT'];MOCKS=['mock-'+x for x in ['identity','open','write','partial','records','bytes','classify']]
TINY=['success-direct','success-observed','violation-direct','violation-observed']
if mode not in ['mocks','controls','full']:raise RuntimeError('three finite admissions only')
slots=MOCKS if mode=='mocks' else MOCKS+TINY if mode=='controls' else ['full']
def require(ok,msg):
    if not ok:raise RuntimeError(msg)
def read(p):return json.loads(Path(p).read_text())
def pin(p):
    p=Path(p);require(p.is_file() and not p.is_symlink(),'regular original');h=hashlib.sha256();size=0
    with p.open('rb') as f:
        while b:=f.read(1048576):h.update(b);size+=len(b)
    return {'path':str(p),'bytes':size,'sha256':h.hexdigest()}
files={}
def check(row):
    require(pin(row['path'])==row,'exact original bytes')
    require(row['path'] not in files or files[row['path']]==row,'conflicting original pin');files[row['path']]=row
def retain(p):row=pin(p);check(row);return row
def absent(n):
    require(type(n) is int and n>0,'owned positive PGID')
    try:os.killpg(n,0)
    except ProcessLookupError:return
    raise RuntimeError('unsettled writer')
def source_archive(ep):
    require(ep['ok'] is True and len(ep['integrated121'])==121,'complete original121 endpoint')
    for p,h in ep['integrated121'].items():require(ep['sourcePins'].get(str(R/p))==h,'complete121 in extra archive')
    rows=ep['sourceMembers'];require(len(rows)==len(ep['sourcePins']) and len({r['archivePath'] for r in rows})==len(rows),'exact source archive index count')
    require({r['path']:r['sha256'] for r in rows}==ep['sourcePins'],'exact source membership/hash map')
    check(ep['sourceArchive']);by_name={r['archivePath']:r for r in rows}
    with tarfile.open(ep['sourceArchive']['path'],'r:gz') as tar:
        members=tar.getmembers();require(len(members)==len(rows) and {m.name for m in members}==set(by_name),'exact original archive members')
        for member in members:
            row=by_name[member.name];require(member.isfile() and member.name==row['path'].lstrip('/') and member.size==row['bytes'],'regular exact archived source')
            h=hashlib.sha256();size=0
            with tar.extractfile(member) as stream:
                while b:=stream.read(1048576):h.update(b);size+=len(b)
            require(size==row['bytes'] and h.hexdigest()==row['sha256'],'exact archived source bytes')
    runtime=ep['runtime'];require(len(runtime['expected'])==8102 and runtime['observed']=={'files':runtime['expected'],'treeMembers':runtime['manifest']['treeMembers']},'full runtime maps/four tree inventories')
    # Do not re-execute the verifier or archived code at this admission stage.
    # Before/after endpoint originals retain the actual unchanged live verifier results.
review=M/'reviews'/(mode+'.md');require(review.read_text().startswith('PASS\n'),'actual independent preservation/control review');retain(review)
intakes=[]
for slot in slots:
    ip=M/'intake'/(slot+'.json');a=read(ip);require(a['slot']==slot and a['preservation'] is True,'exact preserved slot')
    # Settle ownership before reading/hash-binding the original artifact trees.
    t=read(a['outerTerminal']['path']);require(t['cleanup']['complete'] is True and t['cleanup']['errors']==[],'complete authentic cleanup')
    if t['owned_pgid'] is not None:absent(t['owned_pgid'])
    else:require(t['launch_error'] is not None,'no PGID only for launch failure')
    retain(ip)
    for row in [a[k] for k in ['beforeEndpoint','afterEndpoint','dispatchSeal','outerTerminal','transportTerminal']]:check(row)
    transport=read(a['transportTerminal']['path']);require(type(transport['actualOuterExit']) is int and transport['actualOuterExit']==transport['responses'][-1]['exit_code']==a['actualOuterExit']==t['return_code'],'authentic actual outer terminal')
    td=Path(a['transportTerminal']['path']).parent;responses=sorted(td.glob('response-*.json'))
    require([p.name for p in responses]==['response-%03d.json'%i for i in range(len(responses))] and [read(p) for p in responses]==transport['responses'],'complete original response sequence')
    for p in sorted(td.rglob('*')):
        require(not p.is_symlink(),'no transport alias')
        if p.is_file():retain(p)
    d=read(M/'dispatch'/(slot+'.json'));seal=read(a['dispatchSeal']['path'])
    for row in seal.values():check(row)
    with tarfile.open(seal['archive']['path'],'r:gz') as tar:
        members=tar.getmembers();require(len(members)==1 and members[0].isfile() and members[0].name==seal['dispatch']['path'].lstrip('/'),'exact separate dispatch archive')
        require(tar.extractfile(members[0]).read()==Path(seal['dispatch']['path']).read_bytes(),'acyclic dispatch archive bytes')
    require(transport['args']['cmd']==d['shellCommand'] and transport['args']['workdir']==str(R),'dispatch actual command')
    before=read(a['beforeEndpoint']['path']);after=read(a['afterEndpoint']['path'])
    source_archive(before);source_archive(after)
    require(before['sourcePins']==after['sourcePins'] and before['runtime']==after['runtime'],'same complete source/runtime endpoints')
    bases=[Path(d['inner']),Path(d['outer']),M/'mock-artifacts'/slot[5:] if slot.startswith('mock-') else M/'observations'/slot]
    actual_files=[];actual_dirs=[]
    for base in bases:
        if not base.exists():continue
        for p in sorted(base.rglob('*')):
            require(not p.is_symlink(),'no original alias')
            if p.is_file():actual_files.append(str(p))
            elif p.is_dir():actual_dirs.append(str(p))
            else:raise RuntimeError('unexpected original type')
    require(actual_files==[r['path'] for r in a['originalArtifacts']] and actual_dirs==a['originalDirectories'],'exact all original file/directory inventory')
    for row in a['originalArtifacts']:check(row)
    require(a['canonicalExportAcceptance'] is False and a['compilerAcceptance'] is False and a['H1']=='unresolved','explicit no promotion')
    if mode!='full':
        expected=0 if slot=='mock-identity' or slot.startswith('success') else 74 if slot.startswith('mock-') else 1
        require(a['expectedResultPass'] is True and a['innerReceiptAuthenticated'] is True and a['validationErrors']==[],'actual expected control intake')
        require(type(a['actualCompilerOrMockExit']) is int and a['actualCompilerOrMockExit']==a['actualTimeWrappedExit']==a['actualOuterExit']==expected,'authentic expected control exits')
        require(t['eligible'] is (expected==0) and not t['timed_out'] and not t['cleanup']['forced'],'honest expected-negative eligibility')
        if slot=='mock-identity':require(a['identityChecks']==29,'all identity/chain cases')
    else:
        require(a['actualTimeWrappedExit']==t['actual_exit'],'actual time-wrapped outcome even failed')
        if not a['innerReceiptAuthenticated']:require(a['actualCompilerOrMockExit'] is None,'unknown inner exit stays unknown')
        if a['phaseLocationAvailable']:
            require(a['traceValid'] is True and a['observerFailureKnown'] is False and 1<=len(a['phaseRows'])<=32,'bounded available public interval')
        else:require(not a.get('lastEntered') and not a.get('lastCompleted'),'no location claimed from invalid observer evidence')
    intakes.append(a)
if mode=='controls':
    for kind in ['success','violation']:
        p=M/'pairs'/(kind+'.json');pair=read(p)
        require(pair['pair']==kind and pair['ok'] is True and pair['slots']==[kind+'-direct',kind+'-observed'],'both exact pairs')
        require(pair['allOtherITFValuesEqual'] is True and pair['allOtherStdoutBytesEqual'] is True and pair['stderrBytesEqual'] is True,'exact allowed comparison differences')
        for row in pair['originalFiles']:check(row)
        retain(p)
    prior=read(M/'admissions/mocks.json');require(prior['verdict']=='PASS seven run-profile mocks only','earlier independent mocks admission');retain(M/'admissions/mocks.json')
if mode=='full':
    prior=read(M/'admissions/controls.json');require(prior['verdict']=='PASS seven mocks and four tiny run controls only' and prior['fullDispatchAuthorized'] is False,'controls were separately admitted');retain(M/'admissions/controls.json')
    require(read(M/'slots/full.json')['authorized'] is True,'separate actual full slot decision');retain(M/'slots/full.json')
verdict={'mocks':'PASS seven run-profile mocks only','controls':'PASS seven mocks and four tiny run controls only','full':'PASS one run-phase diagnostic preservation only'}[mode]
result={'verdict':verdict,'slots':slots,'files':[files[p] for p in sorted(files)],'review':pin(review),'createdAt':datetime.datetime.now(datetime.timezone.utc).isoformat(),
        'fullDispatchAuthorized':False,'canonicalExportAcceptance':False,'compilerAcceptance':False,'H1':'unresolved',
        'historical014Phase':'unknown','original014SourceCommit':'08e426c7163880b9312f1f1f029a4dde9d0e7593',
        'limitations':['new observed execution only; includes observer I/O and Promise scheduling','endpoint equality is endpoint evidence, not continuous monitoring','no canonical native_command receipt; no all78/final115/solver/Council/H1 promotion']}
if mode=='full':
    a=intakes[0];result['newFullDiagnostic']={k:a.get(k) for k in ['expectedResultPass','actualCompilerOrMockExit','actualTimeWrappedExit','actualOuterExit','innerReceiptAuthenticated','innerReceiptPresent','phaseLocationAvailable','traceValid','observerFailureKnown','lastEntered','lastCompleted','typecheckResolvedRight','runSimulatorEntered','innerReceiptValidationError','validationErrors','traceValidationError']}
p=M/'admissions'/(mode+'.json')
with p.open('x') as f:json.dump(result,f,indent=2,sort_keys=True);f.write('\n')
print(json.dumps({'admission':pin(p),'verdict':verdict,'nativeExecuted':False,'fullDispatchAuthorized':False}))
```

## Stop rules, evidence retention and handoff

- [ ] Preserve every actual preparation/materialization/endpoint/dispatch/preflight/intake/pair/admission command argument and returned tool response as a fresh transport original outside the finalized directories it describes. Failures and partial files are originals too. The dispatch's native polling routine retains each actual returned native response before another poll. Never reconstruct missing transport, infer success from output, or restart after lost transport. Root takes over an uncertain live group; no final artifact hashing or admission happens until ownership is settled.
- [ ] Stop on any failed source/runtime pin, missing source archive, native slot uncertainty, unexpected mock/tiny result, pair difference outside the two enumerated time allowances, or independent review rejection. No full call occurs unless all seven mocks and four tiny calls have authentic expected exits, both pairs pass and root separately authorizes the one full slot. A failed native control consumes that finite slot; it requires a new reviewed continuation design, not a rerun of this plan.
- [ ] After the full terminal, preserve success, OOM, signal, timeout or observer failure under its actual exit identities. Report the new last entered/completed public interval only when the raw bounded trace validates and no observer failure is known. A missing original inner receipt leaves native exit unknown even if the actual outer result is known. Missing/uncertain cleanup blocks endpoint hashing/intake/admission entirely; do not convert root takeover into an original RH002 terminal.
- [ ] Root requests an independent final review of source/runtime archives, all originals, actual transport and the narrow admission. Later durable packaging is a separate preservation task; do not silently change this plan's frozen inputs or the original014 stage. No commit is part of the source-only writing/materialization/native procedure. Root may commit reviewed plan/evidence separately when its active source freeze permits it.

Completion of this finite plan means the diagnostic and its limitations are preserved and reviewed. It does not repair canonical014, authorize044, complete78 exports or final115, localize the historical OOM, prove a model invariant or settle H1. An observed unfinished runSimulator remains an interval containing expression resolution, evaluator discovery, serialization, Rust execution and output conversion; deeper localization requires its own design and root decision.
