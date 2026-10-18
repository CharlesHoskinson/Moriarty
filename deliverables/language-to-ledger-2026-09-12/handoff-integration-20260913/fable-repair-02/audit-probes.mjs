import {EventEmitter} from 'node:events';
import {mkdtempSync,existsSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
const root='/home/charl/Moriarty-wt-moriarty-ll-successor-20260913b-implement-handoff-fable-repair';
const {runExitRetainingExecutor,runProverBoundedExecutor}=await import(pathToFileURL('/tmp/moriarty-handoff-fable-r2-freeze/executor-caller.mjs'));
const {createInvocationSocket,createRuntimeDirectory}=await import(pathToFileURL('/tmp/moriarty-handoff-fable-r2-freeze/invocation-handoff.mjs'));
const sleep=ms=>new Promise(r=>setTimeout(r,ms)),never=()=>new Promise(()=>{}),clock=Date.now;
const boot='01234567-89ab-4def-8123-456789abcdef',h='a'.repeat(64);
function fixture(){
 const child=new EventEmitter();child.pid=123;child.stdout=new EventEmitter();child.stderr=new EventEmitter();
 const server=new EventEmitter();server.listening=true;server.close=cb=>{server.listening=false;cb?.();};
 const f={child,server,kills:0,spawns:0};
 f.options={command:'/synthetic',args:[],cwd:'/tmp',outerTimeoutMs:20,abandonAfterKillMs:20,readDurableResult:async()=>{throw Error('absent');},dependencies:{spawn:()=>{f.spawns++;return child;},monotonicNow:()=>1n,killGroup:()=>{f.kills++;}},handoff:{parentDir:'/tmp',launcherPid:123,launcherStartTicks:1n,allocationId:'a',actionId:'a',candidateHash:h,runnerDigest:h,chargeId:'c',reservationId:'r',storeIdentity:'s',executionContextSha256:h,projectionSha256:h,correspondenceSha256:h,authoritySha256:h,blockDeadlineUtc:9999999999999,blockDeadlineMonotonicNs:null,appendInvocationEvent:async()=>{},confirmCurrentState:()=>{},getPeerUid:()=>process.getuid(),getPeerPid:()=>123,setupTimeoutMs:20,readBootId:()=>boot,readProcStat:pid=>({pid,ppid:1,startTicks:1n}),createRuntimeDirectory:async()=>'/tmp/fake',createInvocationSocket:async()=>({server,socketPath:'/tmp/fake/x'}),generateNonce:()=>Buffer.alloc(32,1),removeRuntime:async()=>{},appendDenialEvent:async()=>{}}};return f;
}
const observations=[];
function record(probe,data){const row={probe,...data};observations.push(row);console.log(JSON.stringify(row));}
// Wall clock rollback during pre-spawn I/O extends timers installed afterward.
{
 const f=fixture();f.options.handoff.readBootId=async()=>{Date.now=()=>clock()-300;return boot;};f.options.handoff.appendInvocationEvent=never;
 const start=clock();try{const result=await runExitRetainingExecutor(f.options);record('clock-rollback',{elapsedMs:clock()-start,configuredTotalMs:40,kills:f.kills,status:result.disposition.status});}finally{Date.now=clock;}
}
// Settled setup late in abandonment still starts a child after outer expiration.
{
 const f=fixture();let spawnAt;f.options.outerTimeoutMs=20;f.options.abandonAfterKillMs=100;f.options.handoff.readBootId=async()=>{await sleep(65);return boot;};f.options.dependencies.spawn=()=>{f.spawns++;spawnAt=clock();queueMicrotask(()=>f.child.emit('close',0,null));return f.child;};
 const start=clock();try{await runExitRetainingExecutor(f.options);record('startup-after-outer',{unexpectedSuccess:true});}catch(error){record('startup-after-outer',{error:error.message,phase:error.phase,elapsedMs:clock()-start,spawns:f.spawns,kills:f.kills});}
}
// A slow read after setup timeout, followed by slow cleanup, resets both bounds.
{
 const f=fixture();f.options.handoff.appendInvocationEvent=never;f.options.readDurableResult=never;f.options.handoff.removeRuntime=never;f.options.dependencies.spawn=()=>{queueMicrotask(()=>f.child.emit('close',0,null));return f.child;};
 const start=clock(),result=await runExitRetainingExecutor(f.options);record('fresh-completion-bounds',{elapsedMs:clock()-start,configuredTotalMs:40,resultReadFailed:result.diagnostics.resultReadFailed,cleanup:result.diagnostics.handoff.runtimeCleanup.status});
}
// Capture real owned socket, then throw from the next phase, before Object.assign.
{
 const dir=mkdtempSync(join(tmpdir(),'fable-nonce-probe-'));let actual;
 const f=fixture();Object.assign(f.options.handoff,{parentDir:dir,createRuntimeDirectory,createInvocationSocket:async options=>(actual=await createInvocationSocket(options)),generateNonce:()=>{throw Error('nonce source unavailable');},removeRuntime:undefined});
 try{await runExitRetainingExecutor(f.options);}catch(error){record('nonce-exception-loses-socket',{error:error.message,reportedCleanup:error.runtimeCleanup,socketListening:actual.server.listening,socketExists:existsSync(actual.socketPath)});}finally{if(actual.server.listening)await new Promise(r=>actual.server.close(r));rmSync(dir,{recursive:true,force:true});}
}
// A new allocation returned after the setup deadline is not assigned or cleaned.
{
 const dir=mkdtempSync(join(tmpdir(),'fable-late-dir-probe-'));let actual;
 const f=fixture();Object.assign(f.options.handoff,{parentDir:dir,createRuntimeDirectory:async options=>{await sleep(80);actual=await createRuntimeDirectory(options);return actual;},removeRuntime:undefined});
 try{await runExitRetainingExecutor(f.options);}catch(error){await sleep(100);record('late-directory',{error:error.message,phase:error.phase,outstanding:error.outstanding,reportedCleanup:error.runtimeCleanup??null,lateDirectoryExists:existsSync(actual),spawns:f.spawns});}finally{rmSync(dir,{recursive:true,force:true});}
}
// Early child error after handshake activation leaves the protocol timer live.
{
 const f=fixture();f.options.outerTimeoutMs=200;f.options.abandonAfterKillMs=20;f.options.handoff.setupTimeoutMs=100;let denials=0;
 f.options.handoff.appendDenialEvent=async()=>{denials++;};f.options.dependencies.spawn=()=>{setTimeout(()=>f.child.emit('error',Error('child-error')),15);return f.child;};
 try{await runExitRetainingExecutor(f.options);}catch(error){const atReturn=denials,listenersAtReturn=f.server.listenerCount('connection');await sleep(120);record('late-denial-after-error',{error:error.message,cleanup:error.runtimeCleanup.status,listenersAtReturn,denialsAtReturn:atReturn,denialsLater:denials});}
}
// Existing pure latest-start check runs before the awaited boot-id phase.
{
 const f=fixture();let mono=0n,spawnMono;
 f.options.dependencies.monotonicNow=()=>mono;f.options.handoff.readBootId=async()=>{mono=120000000000n;return boot;};f.options.dependencies.spawn=()=>{spawnMono=mono;queueMicrotask(()=>f.child.emit('close',0,null));return f.child;};
 try{await runProverBoundedExecutor({...f.options,writeControlRecord:async()=>{},bootId:boot,timeNamespaceInode:1n,invocationDigest:h});record('latest-start-check-before-boot',{unexpectedSuccess:true});}catch(error){record('latest-start-check-before-boot',{error:error.message,spawns:f.spawns,spawnMono:String(spawnMono)});}
}

const {runHandoffServer}=await import('file:///tmp/moriarty-handoff-fable-r2-freeze/invocation-handoff.mjs');
const {performance}=await import('node:perf_hooks');
function gate(){let release;const promise=new Promise(r=>release=r);return {promise,release};}
function socket(){const s=new EventEmitter();s.destroyed=false;s.writes=[];s.pause=()=>{};s.resume=()=>{};s.write=(line,done)=>{s.writes.push({line,destroyed:s.destroyed});queueMicrotask(done);};s.end=()=>{};s.destroy=()=>{if(s.destroyed)return;s.destroyed=true;s.emit('close');};return s;}
function protocol(server,signal,buildResponse){return {server,signal,timeoutMs:100,expectedNonce:Buffer.alloc(32,1),expectedAuthoritySha256:h,expectedBootId:boot,expectedUid:process.getuid(),invocationPayloadSha256:h,ancestry:{launcherPid:123,launcherStartTicks:1n,readProcStat:pid=>({pid,ppid:1,startTicks:1n}),maxDepth:4},readBootId:()=>boot,getPeerUid:()=>process.getuid(),getPeerPid:()=>123,confirmCurrentState:()=>{},buildResponse,recordDenialEvent:async()=>{}};}
// Abort at the last awaited phase, beyond the state-confirmation regression.
{
 const f=fixture(),s=socket(),controller=new AbortController(),started=gate(),response=gate();
 const p=runHandoffServer(protocol(f.server,controller.signal,()=>{started.release();return response.promise;}));
 f.server.emit('connection',s);s.emit('data',Buffer.from(JSON.stringify({nonce:'01'.repeat(32),authoritySha256:h})+'\n'));
 await started.promise;controller.abort();const result=await p;const atReturn=s.writes.length;
 response.release({payload:{ok:true},payloadSha256:h});await sleep(5);
 record('abort-during-build-response',{status:result.status,writesAtReturn:atReturn,writesAfterReturn:s.writes.length,wroteOnDestroyedSocket:s.writes.some(w=>w.destroyed),listeners:f.server.listenerCount('connection')});
}
// A replay denial already in flight must remain visible on executor return.
{
 const f=fixture(),first=socket(),second=socket(),answered=gate(),denialStarted=gate(),denialDone=gate();let pending=0;
 f.options.outerTimeoutMs=100;f.options.abandonAfterKillMs=30;
 first.write=(line,done)=>{first.writes.push({line,destroyed:first.destroyed});queueMicrotask(done);answered.release();};
 f.options.handoff.appendDenialEvent=async()=>{pending++;denialStarted.release();await denialDone.promise;pending--;};
 const running=runExitRetainingExecutor(f.options);await sleep(5);
 f.server.emit('connection',first);first.emit('data',Buffer.from(JSON.stringify({nonce:'01'.repeat(32),authoritySha256:h})+'\n'));
 await answered.promise;await sleep(1);f.server.emit('connection',second);second.emit('data',Buffer.from('replay'));
 await denialStarted.promise;f.child.emit('close',0,null);const result=await running;
 record('pending-replay-denial-hidden',{pendingDenialWritesAtReturn:pending,handoff:result.diagnostics.handoff});
 denialDone.release();await sleep(5);record('pending-replay-denial-late-settlement',{pendingDenialWrites:pending,secondWrites:second.writes.length});
}
// Force actual socket allocation before deadline, but delay delivery to owner.
{
 const dir=mkdtempSync(join(tmpdir(),'fable-late-socket2-'));let actual;
 const f=fixture();Object.assign(f.options.handoff,{parentDir:dir,createRuntimeDirectory,createInvocationSocket:async options=>{actual=await createInvocationSocket(options);await sleep(70);return actual;},removeRuntime:undefined});
 try{await runExitRetainingExecutor(f.options);}catch(error){const atReturn={listening:actual.server.listening,exists:existsSync(actual.socketPath),cleanup:error.runtimeCleanup.status};const late=await error.lateAllocation.settled;record('late-real-socket-allocation',{error:error.message,atReturn,lateCleanup:late.status,listeningAfter:actual.server.listening,socketExistsAfter:existsSync(actual.socketPath),directoryExistsAfter:existsSync(late.runtimeDir),spawns:f.spawns});}finally{if(actual?.server.listening)await new Promise(r=>actual.server.close(r));rmSync(dir,{recursive:true,force:true});}
}
// Independent timing assertions: 10 ms scheduling margin for a 40 ms run.
{
 const rows=[];
 for(let i=0;i<3;i++){
  const f=fixture();f.options.handoff.setupTimeoutMs=3;f.options.readDurableResult=never;f.options.handoff.removeRuntime=never;f.options.dependencies.spawn=()=>{queueMicrotask(()=>f.child.emit('close',0,null));return f.child;};
  const started=performance.now(),result=await runExitRetainingExecutor(f.options),elapsedMs=performance.now()-started;
  rows.push({elapsedMs,configuredTotalMs:40,allowanceMs:10,withinBound:elapsedMs<=50,readStarted:!result.diagnostics.resultReadSkipped,readFailed:result.diagnostics.resultReadFailed,cleanup:result.diagnostics.handoff.runtimeCleanup.status});
 }
 record('blocking-read-and-cleanup-shared-bound',{rows});
 if(rows.some(r=>!r.withinBound))process.exitCode=1;
}
const {default:assert}=await import('node:assert/strict');
const observed=name=>observations.find(x=>x.probe===name);
assert(observed('clock-rollback').elapsedMs<=50);
assert.equal(observed('startup-after-outer').spawns,0);
assert(observed('fresh-completion-bounds').elapsedMs<=50);
assert.equal(observed('nonce-exception-loses-socket').socketExists,false);
assert.equal(observed('late-directory').lateDirectoryExists,false);
assert.equal(observed('late-denial-after-error').denialsLater,0);
assert.equal(observed('latest-start-check-before-boot').spawns,0);
assert.equal(observed('late-real-socket-allocation').lateCleanup,'removed');
// These assertions establish the two reported counterexamples, not success.
assert.equal(observed('abort-during-build-response').writesAtReturn,0);
assert.equal(observed('abort-during-build-response').writesAfterReturn,1);
assert.equal(observed('pending-replay-denial-hidden').pendingDenialWritesAtReturn,1);
assert.equal(observed('pending-replay-denial-hidden').handoff.outstandingWork,false);
