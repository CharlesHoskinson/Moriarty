import {EventEmitter} from 'node:events';
import {mkdtempSync,existsSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
const root='/home/charl/Moriarty-wt-moriarty-ll-successor-20260913b-implement-handoff-fable-repair';
const {runExitRetainingExecutor,runProverBoundedExecutor}=await import(pathToFileURL('/tmp/moriarty-handoff-fable-r3-freeze/executor-caller.mjs'));
const {createInvocationSocket,createRuntimeDirectory}=await import(pathToFileURL('/tmp/moriarty-handoff-fable-r3-freeze/invocation-handoff.mjs'));
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

const {runHandoffServer}=await import('file:///tmp/moriarty-handoff-fable-r3-freeze/invocation-handoff.mjs');
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
// R3 corrected predicates: the R2 counterexamples must no longer reproduce.
assert.equal(observed('abort-during-build-response').writesAtReturn,0);
assert.equal(observed('abort-during-build-response').writesAfterReturn,0);
assert.equal(observed('abort-during-build-response').wroteOnDestroyedSocket,false);
assert.equal(observed('pending-replay-denial-hidden').pendingDenialWritesAtReturn,1);
assert.equal(observed('pending-replay-denial-hidden').handoff.outstandingWork,true);
assert.equal(observed('pending-replay-denial-hidden').handoff.pendingProtocolWork,1);
assert.equal(observed('pending-replay-denial-late-settlement').pendingDenialWrites,0);
assert.equal(observed('pending-replay-denial-late-settlement').secondWrites,0);

// Fresh R3 variants: final awaited response can fulfil or reject after timeout
// or abort. None may cause a response write or a second denial.
for(const stop of ['abort','timeout'])for(const late of ['fulfil','reject']){
 const f=fixture(),s=socket(),controller=new AbortController(),started=gate();let release,reject,denials=0,pending=0;
 const build=new Promise((r,j)=>{release=r;reject=j;});
 const options={...protocol(f.server,controller.signal,()=>{started.release();return build;}),timeoutMs:20,recordDenialEvent:async()=>{denials++;},persistence:{begin:()=>pending++,settle:()=>pending--}};
 const start=performance.now(),p=runHandoffServer(options);f.server.emit('connection',s);s.emit('data',Buffer.from(JSON.stringify({nonce:'01'.repeat(32),authoritySha256:h})+'\n'));
 await started.promise;if(stop==='abort')controller.abort();const result=await p,elapsedMs=performance.now()-start,denialsAtReturn=denials;
 if(late==='fulfil')release({payload:{accepted:true},payloadSha256:h});else reject(Error('late response rejection'));
 await sleep(30);controller.abort();
 const row={probe:`final-await-${stop}-${late}`,status:result.status,elapsedMs,writes:s.writes.length,denialsAtReturn,denialsAfter:denials,pending,listeners:f.server.listenerCount('connection')};record(row.probe,row);
 assert.equal(row.writes,0);assert.equal(row.denialsAfter,stop==='abort'?0:1);assert.equal(row.pending,0);assert.equal(row.listeners,0);assert(elapsedMs<=30);
}

// A response write already issued but without callback remains owned across
// executor return. Destruction alone does not manufacture callback completion.
{
 const f=fixture(),s=socket(),answered=gate();let callback,context;
 f.options.outerTimeoutMs=100;f.options.abandonAfterKillMs=30;
 f.options.handoff.removeRuntime=async c=>{context=c;};
 s.write=(line,done)=>{s.writes.push({line,destroyed:s.destroyed});callback=done;answered.release();};
 const running=runExitRetainingExecutor(f.options);await sleep(5);f.server.emit('connection',s);s.emit('data',Buffer.from(JSON.stringify({nonce:'01'.repeat(32),authoritySha256:h})+'\n'));await answered.promise;
 f.child.emit('close',0,null);const result=await running;
 const atReturn={pending:result.diagnostics.handoff.pendingProtocolWork,outstanding:result.diagnostics.handoff.outstandingWork,destroyed:s.destroyed};
 callback();await sleep(5);
 record('pending-response-write-at-return',{atReturn,livePendingAfter:context.pendingProtocolWork,snapshotPendingAfter:result.diagnostics.handoff.pendingProtocolWork,writes:s.writes.length});
 assert.equal(atReturn.pending,1);assert.equal(atReturn.outstanding,true);assert.equal(atReturn.destroyed,true);assert.equal(context.pendingProtocolWork,0);assert.equal(s.writes.length,1);
}

// Count several concurrent replay records, then both successful and failed late
// settlement. This observes the executor-owned count through remover injection.
for(const rejects of [false,true]){
 const f=fixture(),first=socket(),replays=[socket(),socket()],answered=gate(),bothStarted=gate();let context,pendingRecords=0;
 const completions=[];f.options.outerTimeoutMs=100;f.options.abandonAfterKillMs=30;
 f.options.handoff.removeRuntime=async c=>{context=c;};
 first.write=(line,done)=>{first.writes.push({line,destroyed:first.destroyed});queueMicrotask(done);answered.release();};
 f.options.handoff.appendDenialEvent=()=>{pendingRecords++;if(pendingRecords===2)bothStarted.release();return new Promise((resolve,reject)=>completions.push(()=>{pendingRecords--;rejects?reject(Error('late fsync failure')):resolve();}));};
 const running=runExitRetainingExecutor(f.options);await sleep(5);f.server.emit('connection',first);first.emit('data',Buffer.from(JSON.stringify({nonce:'01'.repeat(32),authoritySha256:h})+'\n'));await answered.promise;await sleep(1);
 for(const s of replays){f.server.emit('connection',s);s.emit('data',Buffer.from('replay'));}await bothStarted.promise;
 f.child.emit('close',0,null);const result=await running;const atReturn={pendingRecords,pending:result.diagnostics.handoff.pendingProtocolWork,outstanding:result.diagnostics.handoff.outstandingWork};
 for(const done of completions)done();await sleep(30);
 record(`multiple-replays-late-${rejects?'reject':'fulfil'}`,{atReturn,pendingRecordsAfter:pendingRecords,livePendingAfter:context.pendingProtocolWork,writesAfter:replays.map(s=>s.writes.length),destroyed:replays.every(s=>s.destroyed),listeners:f.server.listenerCount('connection')});
 assert.equal(atReturn.pending,2);assert.equal(atReturn.pendingRecords,2);assert.equal(atReturn.outstanding,true);assert.equal(context.pendingProtocolWork,0);assert.equal(pendingRecords,0);assert(replays.every(s=>s.destroyed&&s.writes.length===0));
}

// Child rejection also carries the still-running denial; abort must not erase it.
{
 const f=fixture(),s=socket(),began=gate(),done=gate();let context;
 f.options.outerTimeoutMs=100;f.options.abandonAfterKillMs=30;
 f.options.handoff.removeRuntime=async c=>{context=c;};f.options.handoff.appendDenialEvent=()=>{began.release();return done.promise;};
 const running=runExitRetainingExecutor(f.options).then(()=>assert.fail('expected rejection'),error=>error);await sleep(5);
 f.server.emit('connection',s);s.emit('data',Buffer.from(JSON.stringify({nonce:'02'.repeat(32),authoritySha256:h})+'\n'));await began.promise;f.child.emit('error',Error('child transport failure'));const error=await running;
 const atReturn=error.outstandingWork;done.release();await sleep(30);
 record('child-error-pending-denial',{error:error.message,atReturn,livePendingAfter:context.pendingProtocolWork,writes:s.writes.length,destroyed:s.destroyed});
 assert.deepEqual(atReturn,{setupPending:true,setupPhase:'handshake',pendingProtocolWork:1});assert.equal(context.pendingProtocolWork,0);assert.equal(s.writes.length,0);
}

// H2 preservation: absent/null observation cannot borrow complete success facts.
{
 const {classifyDisposition}=await import('file:///tmp/moriarty-handoff-fable-r3-freeze/fault-matrix.mjs');
 const facts={refused:null,startupValid:true,startupWriteFailed:false,evidencePreexists:false,invocationIdMismatch:false,rawExit:null,terminalEvidencePersisted:true,stopReturnCode:0,stopErrorClass:null,stopReceiptPersisted:true,containmentComplete:true,timerCancelReturnCode:0,timerCancelReceiptPersisted:true,deadlineExceeded:false,parentLost:false,resultWriteFailed:false};
 const nullResult=classifyDisposition(facts),unknownResult=classifyDisposition({...facts,rawExit:{kind:'unknown',code:null}});
 assert.equal(nullResult.status,'PROCESS_UNKNOWN');assert.equal(unknownResult.status,'PROCESS_UNKNOWN');
 const missing={...facts};delete missing.rawExit;assert.throws(()=>classifyDisposition(missing),{message:'FAULT_MATRIX_FIELDS'});
 record('h2-preserved',{nullResult,unknownResult,missingField:'FAULT_MATRIX_FIELDS'});
}
record('assertions-complete',{pass:true,observationCount:observations.length});
