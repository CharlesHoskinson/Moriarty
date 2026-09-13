import {EventEmitter} from 'node:events';
import {mkdtempSync,existsSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
const root='/home/charl/Moriarty-wt-moriarty-ll-successor-20260913b-implement-handoff-fable-repair';
const {runExitRetainingExecutor,runProverBoundedExecutor}=await import(pathToFileURL(root+'/experiments/moriarty-midnight-financial/ledger/executor-caller.mjs'));
const {createInvocationSocket,createRuntimeDirectory}=await import(pathToFileURL(root+'/experiments/moriarty-midnight-financial/ledger/invocation-handoff.mjs'));
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
 const start=clock(),result=await runExitRetainingExecutor(f.options);record('startup-after-outer',{spawnAfterMs:spawnAt-start,outerMs:20,kills:f.kills,deadlineExceeded:result.diagnostics.deadlineExceeded});
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
 try{await runExitRetainingExecutor(f.options);}catch(error){record('nonce-exception-loses-socket',{error:error.message,reportedCleanup:error.runtimeCleanup,socketListening:actual.server.listening,socketExists:existsSync(actual.socketPath)});}finally{await new Promise(r=>actual.server.close(r));rmSync(dir,{recursive:true,force:true});}
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
 const result=await runProverBoundedExecutor({...f.options,writeControlRecord:async()=>{},bootId:boot,timeNamespaceInode:1n,invocationDigest:h});record('latest-start-check-before-boot',{spawnMono:String(spawnMono),latestStart:String(result.proverLifetimeBounds.latestStartMonotonicNs),spawnedAtLateBoundary:spawnMono>=result.proverLifetimeBounds.latestStartMonotonicNs});
}
