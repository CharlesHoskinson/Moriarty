import test,{afterEach} from 'node:test';
import assert from 'node:assert/strict';
import {EventEmitter} from 'node:events';
import {constants as osConstants} from 'node:os';
import {existsSync,mkdtempSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import net from 'node:net';
import {decodeProverControlRecord,computeProverLifetimeBounds} from './prover-lifetime.mjs';
import {classifyRawExit,runExitRetainingExecutor,runProverBoundedExecutor} from './executor-caller.mjs';
import {HANDOFF_ENV_NONCE,HANDOFF_ENV_SOCKET,createInvocationSocket,createRuntimeDirectory} from './invocation-handoff.mjs';

const command=process.execPath,cwd='/synthetic/moriarty-midnight-financial';
const args=['ledger/launch-local.mjs','--run','--plan','/synthetic/private/plan.json','--sha256','ab'.repeat(32)];
const durableResult=(containmentComplete=true)=>({schema:'moriarty.loan-process-result/1',status:'PASS',cleanup:{containmentComplete}});
const handoffHandles=[];afterEach(()=>{for(const handle of handoffHandles.splice(0)){if(typeof handle.destroy==='function'){if(!handle.destroyed)handle.destroy();}else clearTimeout(handle);}});

function fakeChild({status=0,signal=null,stdout=[],stderr=[]}={}){
 const child=new EventEmitter();child.pid=4321;child.stdout=new EventEmitter();child.stderr=new EventEmitter();
 child.finish=()=>{for(const value of stdout)child.stdout.emit('data',Buffer.from(value));for(const value of stderr)child.stderr.emit('data',Buffer.from(value));child.emit('close',status,signal);};
 return child;
}
function options(overrides={}){
 const child=overrides.child??fakeChild(),calls=[];
 const value={command,args:[...args],cwd,outerTimeoutMs:1000,readDurableResult:async()=>durableResult(),dependencies:{
  spawn:(actualCommand,actualArgs,actualOptions)=>{calls.push({actualCommand,actualArgs,actualOptions});queueMicrotask(()=>child.finish());return child;},
  monotonicNow:()=>987n,killGroup:()=>assert.fail('unexpected process-group kill')
 }};
 delete overrides.child;for(const [key,entry] of Object.entries(overrides))value[key]=entry;
 return {value,child,calls};
}

test('raw close classification gives signals precedence and preserves unknown',()=>{
 assert.deepEqual(classifyRawExit({status:0,signal:'SIGKILL'}),{kind:'signal',code:osConstants.signals.SIGKILL??null});
 assert.deepEqual(classifyRawExit({status:7,signal:null}),{kind:'exit',code:7});
 assert.deepEqual(classifyRawExit({status:null,signal:null}),{kind:'unknown',code:null});
 assert.throws(()=>classifyRawExit({status:0,signal:null,extra:true}),{message:'EXECUTOR_RAW_EXIT_FIELDS'});
});

test('actual argv protocol is handed to exactly one detached piped child without mutation',async()=>{
 const {value,calls}=options();const result=await runExitRetainingExecutor(value);
 assert.deepEqual(calls,[{actualCommand:command,actualArgs:args,actualOptions:{cwd,detached:true,stdio:['ignore','pipe','pipe'],env:{}}}]);
 assert.equal(result.outerStartMonotonicNs,987n);assert.deepEqual(result.rawExit,{kind:'exit',code:0});
 assert.deepEqual(result.disposition,{status:'PROCESS_UNKNOWN',exitCode:3,failureCode:'EVIDENCE_WRITE_FAILED'});
 assert.deepEqual(result.diagnostics,{stdout:'',stderr:'',deadlineExceeded:false,resultReadFailed:false,resultReadSkipped:false,killErrorClass:null,handoff:null});
 assert.equal(Object.isFrozen(result),true);assert.equal(Object.isFrozen(result.rawExit),true);assert.equal(Object.isFrozen(result.disposition),true);assert.equal(Object.isFrozen(result.diagnostics),true);
});

test('explicit child environment is passed verbatim without parent-environment merging',async()=>{
 const env={MORIARTY_TEST_ONLY:'closed-value'};const {value,calls}=options({env});
 await runExitRetainingExecutor(value);
 assert.equal(calls[0].actualOptions.env,env);assert.deepEqual(calls[0].actualOptions.env,{MORIARTY_TEST_ONLY:'closed-value'});
});

test('child zero and success-looking stdout never replace missing durable evidence',async()=>{
 const child=fakeChild({stdout:['{"status":"PASS","containmentComplete":true}\n']});let reads=0;
 const {value}=options({child,readDurableResult:async()=>{reads++;throw Error('missing');}});const result=await runExitRetainingExecutor(value);
 assert.equal(reads,1);assert.deepEqual(result.rawExit,{kind:'exit',code:0});
 assert.deepEqual(result.disposition,{status:'PROCESS_UNKNOWN',exitCode:3,failureCode:'EVIDENCE_WRITE_FAILED'});
 assert.equal(result.diagnostics.stdout,'{"status":"PASS","containmentComplete":true}\n');assert.equal(result.diagnostics.resultReadFailed,true);
});

test('durable completion without independently observed containment remains unknown',async()=>{
 const {value}=options({readDurableResult:async()=>durableResult(false)});const result=await runExitRetainingExecutor(value);
 assert.deepEqual(result.disposition,{status:'PROCESS_UNKNOWN',exitCode:3,failureCode:'EVIDENCE_WRITE_FAILED'});
});

test('schema-invalid or exit-inconsistent durable results cannot establish containment',async()=>{
 for(const durable of [
  {...durableResult(),schema:'moriarty.local-financial-integration/1'},
  {...durableResult(),status:'FAIL'}
 ]){
  const {value}=options({readDurableResult:async()=>durable});const result=await runExitRetainingExecutor(value);
  assert.equal(result.diagnostics.resultReadFailed,true);
  assert.deepEqual(result.disposition,{status:'PROCESS_UNKNOWN',exitCode:3,failureCode:'EVIDENCE_WRITE_FAILED'});
 }
});

test('retained nonzero and signal exits remain failed even when durable result reading fails',async()=>{
 for(const [child,rawExit,failureCode] of [
  [fakeChild({status:4}),{kind:'exit',code:4},'MAIN_EXIT_NONZERO'],
  [fakeChild({status:null,signal:'SIGTERM'}),{kind:'signal',code:osConstants.signals.SIGTERM??null},'MAIN_SIGNAL']
 ]){
  const {value}=options({child,readDurableResult:async()=>{throw Error('missing');}});const result=await runExitRetainingExecutor(value);
  assert.deepEqual(result.rawExit,rawExit);assert.deepEqual(result.disposition,{status:'PROCESS_FAILED',exitCode:1,failureCode});
 }
});

test('unknown close observation is never promoted by a valid-looking durable result',async()=>{
 const {value}=options({child:fakeChild({status:null,signal:null})});const result=await runExitRetainingExecutor(value);
 assert.deepEqual(result.rawExit,{kind:'unknown',code:null});assert.deepEqual(result.disposition,{status:'PROCESS_UNKNOWN',exitCode:3,failureCode:'MAIN_EXIT_UNAVAILABLE'});
});

test('combined diagnostics stop at exactly 1048576 bytes without changing disposition',async()=>{
 const child=fakeChild({stdout:['a'.repeat(1048570),'b'.repeat(20)],stderr:['not-appended']});const {value}=options({child});const result=await runExitRetainingExecutor(value);
 assert.equal(Buffer.byteLength(result.diagnostics.stdout),1048576);assert.equal(result.diagnostics.stdout.endsWith('b'.repeat(6)),true);assert.equal(result.diagnostics.stderr,'');
 assert.deepEqual(result.disposition,{status:'PROCESS_UNKNOWN',exitCode:3,failureCode:'EVIDENCE_WRITE_FAILED'});
});

test('outer timeout retains its close signal but classifies the caller kill as unknown',async()=>{
 const child=fakeChild({status:null,signal:'SIGKILL'});let kills=0;
 const {value}=options({child,outerTimeoutMs:10,dependencies:{spawn:()=>child,monotonicNow:()=>5n,killGroup:(pid,signal)=>{kills++;assert.equal(pid,4321);assert.equal(signal,'SIGKILL');queueMicrotask(()=>child.finish());}}});
 const result=await runExitRetainingExecutor(value);assert.equal(kills,1);assert.equal(result.diagnostics.deadlineExceeded,true);
 assert.deepEqual(result.rawExit,{kind:'signal',code:osConstants.signals.SIGKILL??null});assert.deepEqual(result.disposition,{status:'PROCESS_UNKNOWN',exitCode:3,failureCode:'MAIN_EXIT_UNAVAILABLE'});
 await new Promise(resolve=>setTimeout(resolve,20));assert.equal(kills,1);
});

test('a child that never closes after a failed deadline kill settles at the abandonment bound',async()=>{
 const child=fakeChild();let kills=0;
 const {value}=options({child,outerTimeoutMs:5,abandonAfterKillMs:15,dependencies:{spawn:()=>child,monotonicNow:()=>5n,killGroup:()=>{kills++;const error=Error('unreapable');error.name='SyntheticKillError';throw error;}}});
 let watchdog;try{
  const result=await Promise.race([
   runExitRetainingExecutor(value),
   new Promise((_,reject)=>{watchdog=setTimeout(()=>reject(Error('executor did not settle')),200);watchdog.unref();})
  ]);
  assert.equal(kills,1);assert.deepEqual(result.rawExit,{kind:'unknown',code:null});
  assert.deepEqual(result.disposition,{status:'PROCESS_UNKNOWN',exitCode:3,failureCode:'MAIN_EXIT_UNAVAILABLE'});
  assert.equal(result.diagnostics.deadlineExceeded,true);assert.equal(result.diagnostics.killErrorClass,'SyntheticKillError');
 }finally{clearTimeout(watchdog);}
});

test('a failed kill request does not hide a later retained natural main failure',async()=>{
 const child=fakeChild({status:7});
 const {value}=options({child,outerTimeoutMs:5,abandonAfterKillMs:50,dependencies:{spawn:()=>child,monotonicNow:()=>5n,killGroup:()=>{setTimeout(()=>child.finish(),5);const error=Error('kill denied');error.name='KillDenied';throw error;}}});
 const result=await runExitRetainingExecutor(value);
 assert.deepEqual(result.rawExit,{kind:'exit',code:7});assert.deepEqual(result.disposition,{status:'PROCESS_FAILED',exitCode:1,failureCode:'MAIN_EXIT_NONZERO'});
 assert.equal(result.diagnostics.deadlineExceeded,true);assert.equal(result.diagnostics.killErrorClass,'KillDenied');
});

test('child error rejects with the original spawn error',async()=>{
 const child=fakeChild(),expected=Error('spawn transport failed');child.finish=()=>child.emit('error',expected);
 const {value}=options({child});await assert.rejects(runExitRetainingExecutor(value),error=>error===expected);
});

test('malformed close payload rejects instead of escaping the promise',async()=>{
 const child=fakeChild({status:null,signal:9});const {value}=options({child});
 await assert.rejects(runExitRetainingExecutor(value),{message:'EXECUTOR_RAW_EXIT_SIGNAL'});
});

test('closed invalid options refuse before monotonic sampling or spawn',async()=>{
 for(const mutate of [x=>x.extra=true,x=>x.cwd='relative',x=>x.outerTimeoutMs=0,x=>x.args='bad',x=>x.readDurableResult=null,x=>x.env=null,x=>x.env={BAD:1},x=>x.abandonAfterKillMs=0,x=>x.dependencies.extra=()=>{}]){
  let samples=0,spawns=0;const {value}=options();value.dependencies.monotonicNow=()=>{samples++;return 1n;};value.dependencies.spawn=()=>{spawns++;return fakeChild();};mutate(value);
  await assert.rejects(runExitRetainingExecutor(value),error=>error.message.startsWith('EXECUTOR_'));assert.equal(samples,0);assert.equal(spawns,0);
 }
});

test('argv accessors and hidden array fields reject without invoking attacker code',async()=>{
 for(const kind of ['accessor','hidden']){
  const {value}=options();let reads=0;
  if(kind==='accessor')Object.defineProperty(value.args,'0',{enumerable:true,get(){reads++;return process.execPath;}});
  else Object.defineProperty(value.args,'hidden',{value:'not argv'});
  await assert.rejects(runExitRetainingExecutor(value),{message:'EXECUTOR_ARGS'});assert.equal(reads,0);
 }
});

const bootId='01234567-89ab-4def-8123-456789abcdef',invocationDigest='cd'.repeat(32);
function boundedOptions({times=[100n,101n],writeControlRecord,requireControlRecord=true,readDurableResult=async()=>durableResult()}={}){
 const events=[],child=fakeChild(),samples=[...times];
 return {events,child,value:{command,args:[...args],cwd,outerTimeoutMs:1000,readDurableResult,requireControlRecord,writeControlRecord,
  bootId,timeNamespaceInode:77n,invocationDigest,dependencies:{
   monotonicNow:()=>{events.push('clock');return samples.shift();},
   spawn:()=>{events.push('spawn');queueMicrotask(()=>child.finish());return child;},killGroup:()=>assert.fail('unexpected process-group kill')
  }}};
}

test('prover control bytes are durably written before spawn and bounds use the one outer start',async()=>{
 let writes=0,written;const fixture=boundedOptions({writeControlRecord:async bytes=>{fixture.events.push('write');writes++;written=Buffer.from(bytes);}});
 const result=await runProverBoundedExecutor(fixture.value);
 assert.deepEqual(fixture.events,['clock','write','clock','spawn']);assert.equal(writes,1);
 assert.deepEqual(result.proverLifetimeBounds,computeProverLifetimeBounds(100n));assert.equal(result.outerStartMonotonicNs,100n);
 assert.deepEqual(decodeProverControlRecord(written),{bootId,timeNamespaceInode:77n,...result.proverLifetimeBounds,invocationDigest});
});

test('post-start durable-result loss cannot recompute or widen the established prover bounds',async()=>{
 let reads=0;const fixture=boundedOptions({times:[900n,901n],writeControlRecord:()=>{},readDurableResult:async()=>{reads++;throw Error('post-start receipt lost');}});
 const result=await runProverBoundedExecutor(fixture.value);
 assert.equal(reads,1);assert.deepEqual(result.proverLifetimeBounds,computeProverLifetimeBounds(900n));assert.equal(result.outerStartMonotonicNs,900n);
 assert.equal(fixture.events.filter(x=>x==='clock').length,2);assert.equal(result.disposition.status,'PROCESS_UNKNOWN');
});

test('required control persistence failure prevents every child spawn',async()=>{
 const fixture=boundedOptions({writeControlRecord:async()=>{fixture.events.push('write');throw Error('fsync failed');}});
 await assert.rejects(runProverBoundedExecutor(fixture.value),{message:'fsync failed'});assert.deepEqual(fixture.events,['clock','write']);
});

test('legacy optional-control flag cannot permit spawn without a writer',async()=>{
 const fixture=boundedOptions({requireControlRecord:false});delete fixture.value.writeControlRecord;delete fixture.value.bootId;delete fixture.value.timeNamespaceInode;delete fixture.value.invocationDigest;
 await assert.rejects(runProverBoundedExecutor(fixture.value),{message:'EXECUTOR_PROVER_CONTROL_REQUIRED'});assert.deepEqual(fixture.events,[]);
});

test('legacy optional-control flag cannot swallow a failed control write or spawn',async()=>{
 const fixture=boundedOptions({requireControlRecord:false,writeControlRecord:async()=>{fixture.events.push('write');throw Error('optional fsync failed');}});
 await assert.rejects(runProverBoundedExecutor(fixture.value),{message:'optional fsync failed'});assert.deepEqual(fixture.events,['clock','write']);
});

test('entry at the inclusive latest-start boundary refuses after control persistence and before spawn',async()=>{
 const latest=BigInt(120)*1000000000n,fixture=boundedOptions({times:[0n,latest],writeControlRecord:async()=>fixture.events.push('write')});
 await assert.rejects(runProverBoundedExecutor(fixture.value),{message:'PROVER_LIFETIME_LATE_ENTRY'});assert.deepEqual(fixture.events,['clock','write','clock']);
});

const handoffDigest=character=>character.repeat(64),handoffBootId='01234567-89ab-4def-8123-456789abcdef';
function handoff(parentDir,overrides={}){return {parentDir,launcherPid:400,launcherStartTicks:70n,allocationId:'allocation',actionId:'action',candidateHash:handoffDigest('a'),runnerDigest:handoffDigest('b'),chargeId:'charge',reservationId:'reservation',storeIdentity:'store',executionContextSha256:handoffDigest('c'),projectionSha256:handoffDigest('d'),correspondenceSha256:handoffDigest('e'),authoritySha256:handoffDigest('f'),blockDeadlineUtc:9999999999999,blockDeadlineMonotonicNs:null,appendInvocationEvent:async()=>{},confirmCurrentState:async()=>{},readBootId:()=>handoffBootId,readProcStat:pid=>pid===4321?{pid,ppid:400,startTicks:60n}:pid===500?{pid,ppid:400,startTicks:80n}:{pid,ppid:1,startTicks:70n},getPeerUid:()=>process.getuid(),getPeerPid:()=>500,setupTimeoutMs:500,...overrides};}

test('configured handoff passes exactly its two-entry environment and appends before answering',async()=>{const root=mkdtempSync(join(tmpdir(),'moriarty-executor-handoff-')),events=[],child=fakeChild();let client,response,event,connectTimer;
 try{const fixture=options({child,readDurableResult:async()=>durableResult(),handoff:handoff(root,{appendInvocationEvent:async value=>{events.push('append-start');await new Promise(resolve=>setImmediate(resolve));event=value;events.push('append-done');},confirmCurrentState:async()=>events.push('confirm')})});fixture.value.dependencies.spawn=(actualCommand,actualArgs,actualOptions)=>{fixture.calls.push({actualCommand,actualArgs,actualOptions});assert.deepEqual(Reflect.ownKeys(actualOptions.env),[HANDOFF_ENV_SOCKET,HANDOFF_ENV_NONCE]);assert.match(actualOptions.env[HANDOFF_ENV_SOCKET],/\/handoff-[0-9a-f]{32}\/invocation\.sock$/);assert.match(actualOptions.env[HANDOFF_ENV_NONCE],/^[0-9a-f]{64}$/);connectTimer=setTimeout(()=>{client=net.connect(actualOptions.env[HANDOFF_ENV_SOCKET]);handoffHandles.push(client);client.on('error',()=>{});let text='';client.once('connect',()=>{try{client.write(JSON.stringify({nonce:actualOptions.env[HANDOFF_ENV_NONCE],authoritySha256:handoffDigest('f')})+'\n',()=>{});}catch{}});client.on('data',chunk=>{text+=chunk;if(text.includes('\n')){events.push('response');response=JSON.parse(text.slice(0,text.indexOf('\n')));child.finish();}});},10);handoffHandles.push(connectTimer);return child;};const result=await runExitRetainingExecutor(fixture.value);assert.deepEqual(events.slice(0,4),['append-start','append-done','confirm','response']);assert.equal(event.payload.parentPid,4321);assert.equal(event.payload.parentStartTicks,60n);assert.equal(event.payload.outerStartMonotonic,987n);assert.equal(event.payload.outerDeadlineMonotonic,1800000000987n);assert.equal(event.payload.nonceSha256===fixture.calls[0].actualOptions.env[HANDOFF_ENV_NONCE],false);assert.equal(response.payloadSha256,event.payloadSha256);assert.deepEqual(result.rawExit,{kind:'exit',code:0});assert.equal(result.diagnostics.handoff.setup,'settled');assert.equal(result.diagnostics.handoff.handshake,'ok');assert.equal(result.diagnostics.handoff.appendConfirmed,true);assert.equal(result.diagnostics.handoff.outstandingWork,false);assert.equal(result.diagnostics.handoff.runtimeCleanup.status,'removed');assert.equal(existsSync(result.diagnostics.handoff.runtimeCleanup.runtimeDir),false);}
 finally{clearTimeout(connectTimer);client?.destroy();rmSync(root,{recursive:true,force:true});}
});

test('immediate child connection survives a slow invocation append and the executor returns',async()=>{const root=mkdtempSync(join(tmpdir(),'moriarty-executor-handoff-race-')),events=[],child=fakeChild();let client,watchdog;
 try{const fixture=options({child,readDurableResult:async()=>durableResult(),handoff:handoff(root,{appendInvocationEvent:async()=>{events.push('append-start');await new Promise(resolve=>setTimeout(resolve,60));events.push('append-done');},confirmCurrentState:async()=>events.push('confirm'),setupTimeoutMs:300})});fixture.value.dependencies.spawn=(_command,_args,actualOptions)=>{client=net.connect(actualOptions.env[HANDOFF_ENV_SOCKET]);handoffHandles.push(client);client.on('error',()=>{});client.once('connect',()=>client.write(JSON.stringify({nonce:actualOptions.env[HANDOFF_ENV_NONCE],authoritySha256:handoffDigest('f')})+'\n'));client.on('data',chunk=>{events.push('response');assert.equal(JSON.parse(chunk.toString()).payloadSha256.length,64);child.finish();});return child;};const result=await Promise.race([runExitRetainingExecutor(fixture.value),new Promise((_,reject)=>{watchdog=setTimeout(()=>reject(Error('immediate handoff did not return')),750);})]);assert.deepEqual(events,['append-start','append-done','confirm','response']);assert.deepEqual(result.rawExit,{kind:'exit',code:0});}
 finally{clearTimeout(watchdog);client?.destroy();rmSync(root,{recursive:true,force:true});}
});

test('handoff server close has a bounded fallback when close never calls back',async()=>{const child=fakeChild(),server=new EventEmitter();server.listening=true;server.close=()=>{server.listening=false;};const fixture=options({child,handoff:handoff('/tmp/fake-evidence',{createRuntimeDirectory:async()=>'/tmp/fake-evidence/handoff-'+handoffDigest('b').slice(0,32),createInvocationSocket:async()=>({server,socketPath:'/tmp/fake-evidence/handoff-'+handoffDigest('b').slice(0,32)+'/invocation.sock'}),generateNonce:()=>Buffer.alloc(32,1),removeRuntime:undefined,setupTimeoutMs:10})});fixture.value.dependencies.spawn=()=>{queueMicrotask(()=>child.finish());return child;};let watchdog;try{const result=await Promise.race([runExitRetainingExecutor(fixture.value),new Promise((_,reject)=>{watchdog=setTimeout(()=>reject(Error('handoff close did not return')),600);})]);assert.deepEqual(result.rawExit,{kind:'exit',code:0});assert.equal(result.disposition.status,'PROCESS_UNKNOWN');}finally{clearTimeout(watchdog);}});

test('failed invocation append yields existing evidence-write failure and attaches no handshake listener',async()=>{const child=fakeChild(),server=new EventEmitter();server.listening=true;let connectionListeners=0,spawns=0;server.on=((original=>function(event,...rest){if(event==='connection')connectionListeners++;return original.call(this,event,...rest);})(server.on));server.close=callback=>{server.listening=false;callback?.();};const fixture=options({child,handoff:handoff('/tmp/fake-evidence',{createRuntimeDirectory:async()=>'/tmp/fake-evidence/handoff-'+handoffDigest('a').slice(0,32),createInvocationSocket:async()=>({server,socketPath:'/tmp/fake-evidence/handoff-'+handoffDigest('a').slice(0,32)+'/invocation.sock'}),generateNonce:()=>Buffer.alloc(32,1),appendInvocationEvent:async()=>{throw Error('fsync failed');},removeRuntime:async()=>{}})});fixture.value.dependencies.spawn=()=>{spawns++;queueMicrotask(()=>child.finish());return child;};const result=await runExitRetainingExecutor(fixture.value);assert.equal(spawns,1);assert.equal(connectionListeners,0);assert.deepEqual(result.disposition,{status:'PROCESS_UNKNOWN',exitCode:3,failureCode:'EVIDENCE_WRITE_FAILED'});});

test('explicit environment conflicts with handoff before monotonic sampling or spawn',async()=>{let samples=0,spawns=0;const fixture=options({env:{EXTRA:'forbidden'},handoff:handoff('/tmp/evidence')});fixture.value.dependencies.monotonicNow=()=>{samples++;return 1n;};fixture.value.dependencies.spawn=()=>{spawns++;return fakeChild();};await assert.rejects(runExitRetainingExecutor(fixture.value),{message:'EXECUTOR_HANDOFF_ENV_CONFLICT'});assert.equal(samples,0);assert.equal(spawns,0);});

// GPT-6 high H1/H3 regressions (2026-09-13, second round): one monotonic
// schedule, outer-bounded setup, budgeted completion and owned cleanup.
//
// Timing tests assert the configured limit plus SCHEDULING_ALLOWANCE_MS. The
// allowance covers libuv timer granularity (1 ms) and event-loop latency under
// the test runner; observed slip on this host is single-digit milliseconds and
// the independent audit replicated the bounds within 10 ms. 25 ms is well above
// that slip and below the 40 ms outer bound, the smallest limit it guards.
const OUTER_MS=40,ABANDON_MS=40,TOTAL_MS=OUTER_MS+ABANDON_MS,SCHEDULING_ALLOWANCE_MS=25,LATE_SETTLE_MS=40;
const monotonic=()=>performance.now();
const gate=()=>{let release;const promise=new Promise(resolve=>{release=resolve;});return {promise,release};};
const never=()=>new Promise(()=>{});
const sleep=ms=>new Promise(resolve=>setTimeout(resolve,ms));
const rejection=work=>work.then(()=>assert.fail('resolved instead of rejecting'),error=>error);
const assertWithin=(elapsed,limit,label)=>assert(elapsed<=limit+SCHEDULING_ALLOWANCE_MS,`${label}: ${elapsed.toFixed(1)}ms exceeds ${limit}ms + ${SCHEDULING_ALLOWANCE_MS}ms allowance`);
function fakeServer(){const server=new EventEmitter();server.listening=true;server.connectionRegistrations=0;const original=server.addListener;server.addListener=function(event,...rest){if(event==='connection')server.connectionRegistrations++;return original.call(this,event,...rest);};server.on=server.addListener;server.close=callback=>{server.listening=false;callback?.();};return server;}
function fakeSocket(){const socket=new EventEmitter();socket.destroyed=false;socket.writes=[];socket.pause=()=>{};socket.resume=()=>{};socket.write=(line,done)=>{socket.writes.push(String(line));if(done)queueMicrotask(done);return true;};socket.end=()=>{};socket.destroy=()=>{if(socket.destroyed)return;socket.destroyed=true;socket.emit('close');};return socket;}
const helloLine=()=>JSON.stringify({nonce:'01'.repeat(32),authoritySha256:handoffDigest('f')})+'\n';
const fakeRuntimeDir='/tmp/fake-evidence/handoff-'+handoffDigest('c').slice(0,32);
function stallFixture({server=fakeServer(),child=fakeChild(),removed=[],handoffOverrides={},...runOverrides}={}){
 const fixture=options({child,outerTimeoutMs:OUTER_MS,abandonAfterKillMs:ABANDON_MS,readDurableResult:async()=>{throw Error('absent');},handoff:handoff('/tmp/fake-evidence',{createRuntimeDirectory:async()=>fakeRuntimeDir,createInvocationSocket:async()=>({server,socketPath:fakeRuntimeDir+'/invocation.sock'}),generateNonce:()=>Buffer.alloc(32,1),removeRuntime:async context=>{removed.push(context.runtimeDir);},appendDenialEvent:async()=>{},readProcStat:pid=>pid===4321||pid===500?{pid,ppid:400,startTicks:80n}:{pid,ppid:1,startTicks:70n},...handoffOverrides}),...runOverrides});
 fixture.kills=0;fixture.spawns=0;fixture.value.dependencies.spawn=()=>{fixture.spawns++;return child;};fixture.value.dependencies.killGroup=()=>{fixture.kills++;};fixture.server=server;fixture.removed=removed;return fixture;
}
const timed=async work=>{const started=monotonic();const value=await work;return {value,elapsed:monotonic()-started};};

test('H1 reproducer: a never-resolving invocation append returns within the configured outer plus abandonment bound',async()=>{
 const append=gate(),queued=fakeSocket();const fixture=stallFixture({handoffOverrides:{appendInvocationEvent:()=>append.promise}});
 fixture.value.dependencies.spawn=()=>{fixture.spawns++;fixture.server.emit('connection',queued);return fixture.child;};
 const {value:result,elapsed}=await timed(runExitRetainingExecutor(fixture.value));
 assertWithin(elapsed,TOTAL_MS,'stalled append');assert.equal(fixture.kills,1);
 assert.deepEqual(result.rawExit,{kind:'unknown',code:null});assert.deepEqual(result.disposition,{status:'PROCESS_UNKNOWN',exitCode:3,failureCode:'EVIDENCE_WRITE_FAILED'});
 assert.equal(result.diagnostics.deadlineExceeded,true);assert.equal(result.diagnostics.resultReadSkipped,false);
 const h=result.diagnostics.handoff;assert.equal(h.setup,'pending');assert.equal(h.phase,'append');assert.equal(h.appendConfirmed,false);assert.equal(h.handshake,null);assert.equal(h.outstandingWork,true);assert.equal(h.runtimeCleanup.status,'removed');assert.deepEqual(fixture.removed,[fakeRuntimeDir]);
 assert.equal(queued.destroyed,true);const registrationsAtReturn=fixture.server.connectionRegistrations;
 append.release();await sleep(LATE_SETTLE_MS);
 assert.equal(fixture.server.connectionRegistrations,registrationsAtReturn,'late append must not open the handshake on the cleaned-up server');
 assert.deepEqual(queued.writes,[]);assert.equal(fixture.server.listenerCount('connection'),0);
});

test('H1 prover path: a never-resolving invocation append returns within the same bound with the established prover bounds retained',async()=>{
 const append=gate();const fixture=stallFixture({handoffOverrides:{appendInvocationEvent:()=>append.promise}});
 const value={...fixture.value,writeControlRecord:async()=>{},bootId,timeNamespaceInode:77n,invocationDigest};
 const {value:result,elapsed}=await timed(runProverBoundedExecutor(value));
 assertWithin(elapsed,TOTAL_MS,'stalled append (prover)');
 assert.deepEqual(result.proverLifetimeBounds,computeProverLifetimeBounds(987n));assert.deepEqual(result.disposition,{status:'PROCESS_UNKNOWN',exitCode:3,failureCode:'EVIDENCE_WRITE_FAILED'});
 assert.equal(result.diagnostics.handoff.setup,'pending');assert.equal(result.diagnostics.handoff.outstandingWork,true);assert.equal(result.diagnostics.handoff.runtimeCleanup.status,'removed');
 append.release();await sleep(LATE_SETTLE_MS);assert.equal(fixture.server.listenerCount('connection'),0);
});

test('H1a: a wall-clock rollback during setup does not extend the monotonic schedule',async()=>{
 const realNow=Date.now;const fixture=stallFixture({handoffOverrides:{readBootId:async()=>{Date.now=()=>realNow()-300;return handoffBootId;},appendInvocationEvent:never}});
 try{const {value:result,elapsed}=await timed(runExitRetainingExecutor(fixture.value));assertWithin(elapsed,TOTAL_MS,'wall rollback');assert.equal(fixture.kills,1);assert.equal(result.disposition.status,'PROCESS_UNKNOWN');}
 finally{Date.now=realNow;}
});

test('H1b: a stalled durable read and a stalled remover share the one final bound and are reported unresolved',async()=>{
 const fixture=stallFixture({readDurableResult:never,handoffOverrides:{appendInvocationEvent:never,removeRuntime:never}});
 fixture.value.dependencies.spawn=()=>{fixture.spawns++;queueMicrotask(()=>fixture.child.finish());return fixture.child;};
 const {value:result,elapsed}=await timed(runExitRetainingExecutor(fixture.value));
 assertWithin(elapsed,TOTAL_MS,'shared final bound');
 assert.equal(fixture.kills,0);assert.deepEqual(result.rawExit,{kind:'exit',code:0});assert.deepEqual(result.disposition,{status:'PROCESS_UNKNOWN',exitCode:3,failureCode:'EVIDENCE_WRITE_FAILED'});
 assert.equal(result.diagnostics.resultReadFailed,true);assert.equal(result.diagnostics.resultReadSkipped,true,'no budget remained, so the optional read must not start');
 assert.equal(result.diagnostics.handoff.setup,'pending');assert.equal(result.diagnostics.handoff.runtimeCleanup.status,'unresolved');
});

test('H1b: a slow durable read that outlives its budget is unresolved but leaves the reserved cleanup margin intact',async()=>{
 const removals=[];const fixture=stallFixture({readDurableResult:never,handoffOverrides:{appendInvocationEvent:async()=>{},setupTimeoutMs:5,removeRuntime:async context=>{removals.push(context.runtimeDir);}}});
 fixture.value.dependencies.spawn=()=>{fixture.spawns++;queueMicrotask(()=>fixture.child.finish());return fixture.child;};
 const started=monotonic(),result=await runExitRetainingExecutor(fixture.value),elapsed=monotonic()-started;
 assertWithin(elapsed,TOTAL_MS,'slow read');assert.equal(result.diagnostics.resultReadFailed,true);assert.equal(result.diagnostics.resultReadSkipped,false);
 assert.equal(result.diagnostics.handoff.runtimeCleanup.status,'removed');assert.equal(removals.length,1);
});

test('H1c: a slow boot-id read that crosses the outer deadline rejects without any spawn',async()=>{
 const fixture=stallFixture({abandonAfterKillMs:100,handoffOverrides:{readBootId:async()=>{await sleep(OUTER_MS+45);return handoffBootId;}}});
 const {value:error,elapsed}=await timed(rejection(runExitRetainingExecutor(fixture.value)));
 assertWithin(elapsed,OUTER_MS,'slow boot');assert.equal(error.message,'EXECUTOR_SETUP_TIMEOUT');assert.equal(error.phase,'boot-id');assert.equal(error.outstanding,true);
 assert.equal(fixture.spawns,0);assert.equal(fixture.kills,0);assert.equal(error.runtimeCleanup.status,'removed');assert.deepEqual(fixture.removed,[fakeRuntimeDir]);
 await sleep(60);assert.equal(fixture.spawns,0,'the late boot-id fulfilment must not spawn');
});

test('H1c: the outer deadline is rechecked immediately before spawn after a blocking final phase',async()=>{
 const fixture=stallFixture({handoffOverrides:{readBootId:()=>{const until=monotonic()+OUTER_MS+5;while(monotonic()<until);return handoffBootId;}}});
 const error=await rejection(runExitRetainingExecutor(fixture.value));
 assert.equal(error.message,'EXECUTOR_LATE_START');assert.equal(error.phase,'spawn');assert.equal(error.outstanding,false);assert.equal(fixture.spawns,0);assert.equal(error.runtimeCleanup.status,'removed');
});

test('L1: the prover latest-start check runs after the awaited boot-id, immediately before spawn',async()=>{
 let mono=0n;const fixture=stallFixture({handoffOverrides:{readBootId:async()=>{mono=120n*1000000000n;return handoffBootId;}}});fixture.value.dependencies.monotonicNow=()=>mono;
 const error=await rejection(runProverBoundedExecutor({...fixture.value,writeControlRecord:async()=>{},bootId,timeNamespaceInode:77n,invocationDigest}));
 assert.equal(error.message,'PROVER_LIFETIME_LATE_ENTRY');assert.equal(fixture.spawns,0);assert.equal(error.runtimeCleanup.status,'removed');
});

test('a child that exits while the invocation append is still stalled returns within the bound without inventing persisted evidence',async()=>{
 const append=gate();const fixture=stallFixture({handoffOverrides:{appendInvocationEvent:()=>append.promise}});
 fixture.value.dependencies.spawn=()=>{fixture.spawns++;queueMicrotask(()=>fixture.child.finish());return fixture.child;};
 const {value:result,elapsed}=await timed(runExitRetainingExecutor(fixture.value));
 assertWithin(elapsed,TOTAL_MS,'early exit, stalled append');assert.equal(fixture.kills,0);assert.deepEqual(result.rawExit,{kind:'exit',code:0});assert.deepEqual(result.disposition,{status:'PROCESS_UNKNOWN',exitCode:3,failureCode:'EVIDENCE_WRITE_FAILED'});
 assert.equal(result.diagnostics.handoff.setup,'pending');assert.equal(result.diagnostics.handoff.appendConfirmed,false);append.release();
});

for(const [label,handoffOverrides] of [
 ['stalled current-state confirmation',{confirmCurrentState:never}],
 ['stalled denial append after a reservation mismatch',{confirmCurrentState:async()=>{throw Error('HANDOFF_RESERVATION_MISMATCH');},appendDenialEvent:never}]
])test(`${label} returns STARTUP_INVALID within the bound and never answers the closed socket`,async()=>{
 const socket=fakeSocket();socket.resume=()=>queueMicrotask(()=>socket.emit('data',Buffer.from(helloLine())));
 const fixture=stallFixture({handoffOverrides:{...handoffOverrides,setupTimeoutMs:60000}});
 fixture.value.dependencies.spawn=()=>{fixture.spawns++;fixture.server.emit('connection',socket);return fixture.child;};
 const {value:result,elapsed}=await timed(runExitRetainingExecutor(fixture.value));
 assertWithin(elapsed,TOTAL_MS,label);assert.equal(fixture.kills,1);assert.deepEqual(result.disposition,{status:'PROCESS_UNKNOWN',exitCode:3,failureCode:'STARTUP_INVALID'});
 const h=result.diagnostics.handoff;assert.equal(h.appendConfirmed,true);assert(h.setup==='pending'&&h.phase==='handshake'&&h.handshake==='cancelled'||h.setup==='settled'&&h.handshake==='denied'&&h.denialCode==='HANDOFF_SETUP_TIMEOUT'||h.setup==='settled'&&h.handshake==='cancelled',JSON.stringify(h));
 assert.equal(socket.destroyed,true);await sleep(LATE_SETTLE_MS);
 assert.deepEqual(socket.writes.filter(line=>line.includes('payloadSha256')),[]);assert.equal(h.runtimeCleanup.status,'removed');assert.equal(fixture.server.listenerCount('connection'),0);
});

test('H1d: a child error after handshake activation disposes the protocol so no later denial record starts',async()=>{
 let denials=0;const fixture=stallFixture({outerTimeoutMs:200,abandonAfterKillMs:40,handoffOverrides:{setupTimeoutMs:100,appendDenialEvent:async()=>{denials++;}}});
 fixture.value.dependencies.spawn=()=>{fixture.spawns++;setTimeout(()=>fixture.child.emit('error',Error('child-error')),15);return fixture.child;};
 const error=await rejection(runExitRetainingExecutor(fixture.value));
 assert.equal(error.message,'child-error');assert.equal(error.runtimeCleanup.status,'removed');
 const listenersAtReturn=fixture.server.listenerCount('connection'),denialsAtReturn=denials;await sleep(120);
 assert.equal(listenersAtReturn,0);assert.equal(denialsAtReturn,0);assert.equal(denials,0,'the protocol setup timer must not start a denial append after disposal');
});

for(const [phase,handoffOverrides,expectRuntime] of [
 ['runtime-directory',{createRuntimeDirectory:never},false],
 ['invocation-socket',{createInvocationSocket:never},true],
 ['boot-id',{readBootId:never},true]
])test(`stalled pre-spawn ${phase} rejects at the outer bound without a spawn and removes any owned runtime`,async()=>{
 const fixture=stallFixture({handoffOverrides});
 const {value:error,elapsed}=await timed(rejection(runExitRetainingExecutor(fixture.value)));
 assertWithin(elapsed,OUTER_MS,`stalled ${phase}`);assert.equal(error.message,'EXECUTOR_SETUP_TIMEOUT');assert.equal(error.phase,phase);assert.equal(error.outstanding,true);assert.equal(fixture.spawns,0);
 if(expectRuntime){assert.equal(error.runtimeCleanup.status,'removed');assert.equal(fixture.removed.length,1);}else{assert.equal(error.runtimeCleanup,undefined);assert.equal(fixture.removed.length,0);}
 if(phase!=='boot-id'){assert.equal(error.lateAllocation.phase,phase);assert.equal(typeof error.lateAllocation.settled.then,'function');}else assert.equal(error.lateAllocation,undefined);
});

test('stalled prover control write rejects at the outer bound before any spawn',async()=>{
 const fixture=boundedOptions({writeControlRecord:()=>{fixture.events.push('write');return never();}});fixture.value.outerTimeoutMs=OUTER_MS;fixture.value.abandonAfterKillMs=ABANDON_MS;
 const {value:error,elapsed}=await timed(rejection(runProverBoundedExecutor(fixture.value)));
 assertWithin(elapsed,OUTER_MS,'stalled control write');assert.equal(error.message,'EXECUTOR_SETUP_TIMEOUT');assert.equal(error.phase,'control-record');assert.equal(error.outstanding,true);assert.deepEqual(fixture.events,['clock','write']);
});

test('H3b: a runtime directory created after the outer bound is owned and removed without advancing setup',async()=>{
 const root=mkdtempSync(join(tmpdir(),'moriarty-executor-late-dir-'));let actual;
 try{
  const fixture=stallFixture({handoffOverrides:{parentDir:root,createRuntimeDirectory:async value=>{await sleep(OUTER_MS+40);actual=await createRuntimeDirectory(value);return actual;},removeRuntime:undefined}});
  const error=await rejection(runExitRetainingExecutor(fixture.value));
  assert.equal(error.message,'EXECUTOR_SETUP_TIMEOUT');assert.equal(error.phase,'runtime-directory');assert.equal(error.runtimeCleanup,undefined);assert.equal(error.lateAllocation.phase,'runtime-directory');
  const late=await error.lateAllocation.settled;
  assert.equal(late.status,'removed');assert.equal(late.runtimeDir,actual);assert.equal(existsSync(actual),false);assert.equal(fixture.spawns,0);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test('H3b: a socket created after the outer bound is closed and unlinked without advancing setup',async()=>{
 const root=mkdtempSync(join(tmpdir(),'moriarty-executor-late-socket-'));let actual;
 try{
  const fixture=stallFixture({handoffOverrides:{parentDir:root,createRuntimeDirectory,createInvocationSocket:async value=>{await sleep(OUTER_MS+40);actual=await createInvocationSocket(value);return actual;},removeRuntime:undefined}});
  const error=await rejection(runExitRetainingExecutor(fixture.value));
  assert.equal(error.message,'EXECUTOR_SETUP_TIMEOUT');assert.equal(error.phase,'invocation-socket');assert.equal(error.runtimeCleanup.status,'removed');assert.equal(error.lateAllocation.phase,'invocation-socket');
  const late=await error.lateAllocation.settled;
  if(late.status==='rejected'){assert.equal(actual,undefined,'directory removal made the late listen fail, so nothing was acquired');}
  else{assert.equal(late.status,'removed');assert.equal(actual.server.listening,false);assert.equal(existsSync(actual.socketPath),false);}
  assert.equal(existsSync(error.runtimeCleanup.runtimeDir),false);assert.equal(fixture.spawns,0);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test('H3a: a nonce failure after a real socket is acquired closes the server and removes the owned paths',async()=>{
 const root=mkdtempSync(join(tmpdir(),'moriarty-executor-nonce-'));let actual;
 try{
  const fixture=stallFixture({handoffOverrides:{parentDir:root,createRuntimeDirectory,createInvocationSocket:async value=>(actual=await createInvocationSocket(value)),generateNonce:()=>{throw Error('nonce source unavailable');},removeRuntime:undefined}});
  const error=await rejection(runExitRetainingExecutor(fixture.value));
  assert.equal(error.message,'nonce source unavailable');assert.equal(fixture.spawns,0);
  assert.equal(error.runtimeCleanup.status,'removed');assert.equal(error.runtimeCleanup.socketPath,actual.socketPath);
  assert.equal(actual.server.listening,false);assert.equal(existsSync(actual.socketPath),false);assert.equal(existsSync(error.runtimeCleanup.runtimeDir),false);
 }finally{if(actual?.server.listening)await new Promise(resolve=>actual.server.close(resolve));rmSync(root,{recursive:true,force:true});}
});

test('H3: spawn failure and child error after runtime allocation preserve the original error and remove the owned runtime',async()=>{
 for(const kind of ['spawn-throws','child-error']){
  const expected=Error(kind);const fixture=stallFixture();
  if(kind==='spawn-throws')fixture.value.dependencies.spawn=()=>{throw expected;};else{fixture.child.finish=()=>fixture.child.emit('error',expected);fixture.value.dependencies.spawn=()=>{queueMicrotask(()=>fixture.child.finish());return fixture.child;};}
  const error=await rejection(runExitRetainingExecutor(fixture.value));
  assert.equal(error,expected);assert.equal(error.runtimeCleanup.status,'removed');assert.deepEqual(fixture.removed,[fakeRuntimeDir]);assert.equal(fixture.server.listenerCount('connection'),0);
 }
});

test('H3: a failed runtime removal is reported in the result instead of being swallowed',async()=>{
 const failing=stallFixture({handoffOverrides:{setupTimeoutMs:5,removeRuntime:async()=>{const error=Error('rmdir denied');error.name='RemoveFailed';throw error;}}});failing.value.dependencies.spawn=()=>{queueMicrotask(()=>failing.child.finish());return failing.child;};
 const failed=await runExitRetainingExecutor(failing.value);
 assert.deepEqual(failed.diagnostics.handoff.runtimeCleanup,{status:'failed',errorClass:'RemoveFailed',errorMessage:'rmdir denied',runtimeDir:fakeRuntimeDir,socketPath:fakeRuntimeDir+'/invocation.sock'});
});

test('H3: a spawn failure whose runtime removal also fails keeps the original error and reports the cleanup failure',async()=>{
 const expected=Error('spawn denied');const fixture=stallFixture({handoffOverrides:{removeRuntime:async()=>{throw Error('unlink denied');}}});fixture.value.dependencies.spawn=()=>{throw expected;};
 const error=await rejection(runExitRetainingExecutor(fixture.value));
 assert.equal(error,expected);assert.equal(error.runtimeCleanup.status,'failed');assert.equal(error.runtimeCleanup.errorMessage,'unlink denied');
});

// R2 H1d regressions (2026-09-13): outstanding protocol persistence is owned
// and reported through executor return, and late fulfilment writes nothing.
const trackedSocket=()=>{const socket=fakeSocket();socket.writes=[];socket.write=(line,done)=>{socket.writes.push({line:String(line),destroyed:socket.destroyed});if(done)queueMicrotask(done);return true;};return socket;};
test('R2: a replay denial record still in flight after a successful handshake is reported outstanding at return and never written after release',async()=>{
 const first=trackedSocket(),second=trackedSocket();let releaseDenial,denials=0;const denialStarted=gate(),answered=gate();
 first.write=(line,done)=>{first.writes.push({line:String(line),destroyed:first.destroyed});if(done)queueMicrotask(done);answered.release();return true;};
 const fixture=stallFixture({outerTimeoutMs:200,abandonAfterKillMs:40,handoffOverrides:{setupTimeoutMs:150,appendDenialEvent:()=>{denials++;denialStarted.release();return new Promise(resolve=>{releaseDenial=resolve;});}}});
 fixture.value.dependencies.spawn=()=>{fixture.spawns++;return fixture.child;};
 const running=runExitRetainingExecutor(fixture.value);await sleep(5);
 fixture.server.emit('connection',first);first.emit('data',Buffer.from(helloLine()));await answered.promise;await sleep(1);
 fixture.server.emit('connection',second);second.emit('data',Buffer.from('replay'));await denialStarted.promise;
 fixture.child.finish();const result=await running;
 assert.deepEqual(result.rawExit,{kind:'exit',code:0});const h=result.diagnostics.handoff;
 assert.equal(h.setup,'settled');assert.equal(h.handshake,'ok');assert.equal(h.pendingProtocolWork,1);assert.equal(h.outstandingWork,true,'the in-flight replay denial is outstanding at return');assert.equal(h.runtimeCleanup.status,'removed');
 assert.equal(second.destroyed,true);assert.equal(denials,1);
 releaseDenial();await sleep(LATE_SETTLE_MS);
 assert.deepEqual(second.writes,[],'late record settlement must not write a denial to the closed replay socket');assert.equal(denials,1);
});
test('R2: a child error while a denial record is in flight reports that record outstanding on the rejection and writes nothing after release',async()=>{
 const bad=trackedSocket();let releaseDenial;const denialStarted=gate();
 const fixture=stallFixture({outerTimeoutMs:200,abandonAfterKillMs:40,handoffOverrides:{setupTimeoutMs:150,appendDenialEvent:()=>{denialStarted.release();return new Promise(resolve=>{releaseDenial=resolve;});}}});
 fixture.value.dependencies.spawn=()=>{fixture.spawns++;return fixture.child;};
 const running=rejection(runExitRetainingExecutor(fixture.value));await sleep(5);
 fixture.server.emit('connection',bad);bad.emit('data',Buffer.from(JSON.stringify({nonce:'02'.repeat(32),authoritySha256:handoffDigest('f')})+'\n'));await denialStarted.promise;
 fixture.child.emit('error',Error('child-error'));const error=await running;
 assert.equal(error.message,'child-error');assert.equal(error.runtimeCleanup.status,'removed');
 assert.deepEqual(error.outstandingWork,{setupPending:true,setupPhase:'handshake',pendingProtocolWork:1});assert.equal(bad.destroyed,true);
 releaseDenial();await sleep(LATE_SETTLE_MS);assert.deepEqual(bad.writes,[]);assert.equal(fixture.server.listenerCount('connection'),0);
});
test('R2: a completed handshake with no in-flight protocol work reports zero pending work',async()=>{
 const first=trackedSocket();const answered=gate();first.write=(line,done)=>{first.writes.push({line:String(line),destroyed:first.destroyed});if(done)queueMicrotask(done);answered.release();return true;};
 const fixture=stallFixture({outerTimeoutMs:200,abandonAfterKillMs:40,handoffOverrides:{setupTimeoutMs:150}});fixture.value.dependencies.spawn=()=>{fixture.spawns++;return fixture.child;};
 const running=runExitRetainingExecutor(fixture.value);await sleep(5);fixture.server.emit('connection',first);first.emit('data',Buffer.from(helloLine()));await answered.promise;await sleep(2);
 fixture.child.finish();const result=await running;assert.equal(result.diagnostics.handoff.pendingProtocolWork,0);assert.equal(result.diagnostics.handoff.outstandingWork,false);assert.equal(first.writes.length,1);
});
