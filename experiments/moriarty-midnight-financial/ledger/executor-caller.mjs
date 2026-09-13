/** One-shot child caller with raw exit retention before post-close work. Stdout
 * and stderr are bounded diagnostics only; durable validated facts decide success.
 *
 * One monotonic schedule governs every wait in a run. Pre-spawn setup must
 * finish by the outer deadline, and the outer deadline plus the prover
 * latest-start bound are rechecked immediately before spawn. After the outer
 * deadline only return and cleanup work remains: setup waits, the optional
 * durable-result read and runtime cleanup draw on the remaining abandonment
 * budget, with a reserved cleanup margin. A wait that misses its bound is
 * reported as outstanding; the underlying I/O is never cancelled, a blocking
 * write is not made preemptible, and no persisted evidence is assumed.
 */
import {spawn as nodeSpawn} from 'node:child_process';
import {readFileSync} from 'node:fs';
import {unlink,rmdir} from 'node:fs/promises';
import {constants as osConstants} from 'node:os';
import {isAbsolute,resolve} from 'node:path';
import {performance} from 'node:perf_hooks';
import {classifyDisposition} from './fault-matrix.mjs';
import {computeProverLifetimeBounds,encodeProverControlRecord,validateWrapperEntry} from './prover-lifetime.mjs';
import {HANDOFF_ANCESTRY_MAX_DEPTH,HANDOFF_OUTER_SECONDS,buildInvocationEnv,buildInvocationEventPayload,classifyEofAfterStartup,createInvocationSocket,createRuntimeDirectory,generateNonce,invocationPayloadSha256,nonceHex,nonceSha256Hex,readProcStatReal,runHandoffServer} from './invocation-handoff.mjs';

const DIAGNOSTIC_MAX_BYTES=1048576;
const RESULT_READ_GRACE_MS=1000;
const ABANDON_AFTER_KILL_MS=1000;
const HANDOFF_SERVER_CLOSE_GRACE_MS=250;
const CLEANUP_RESERVE_MAX_MS=250;
const RUN_FIELDS=['command','args','cwd','outerTimeoutMs','readDurableResult'];
const RUN_OPTIONAL_FIELDS=['dependencies','env','abandonAfterKillMs','handoff'];
const DEPENDENCY_FIELDS=['spawn','monotonicNow','killGroup'];
const HANDOFF_FIELDS=['parentDir','launcherPid','launcherStartTicks','allocationId','actionId','candidateHash','runnerDigest','chargeId','reservationId','storeIdentity','executionContextSha256','projectionSha256','correspondenceSha256','authoritySha256','blockDeadlineUtc','blockDeadlineMonotonicNs','appendInvocationEvent','confirmCurrentState','getPeerUid','getPeerPid','setupTimeoutMs'];
const HANDOFF_OPTIONAL_FIELDS=['readBootId','readProcStat','createRuntimeDirectory','createInvocationSocket','generateNonce','removeRuntime','wait','appendDenialEvent'];
const BOUNDED_FIELDS=[...RUN_FIELDS,...RUN_OPTIONAL_FIELDS,'requireControlRecord','writeControlRecord','bootId','timeNamespaceInode','invocationDigest'];
const check=(ok,code)=>{if(!ok)throw Error(code);};
const plain=value=>value!==null&&typeof value==='object'&&Object.getPrototypeOf(value)===Object.prototype;
function fields(value,code){
 check(plain(value),code);const descriptors=Object.getOwnPropertyDescriptors(value);
 check(Reflect.ownKeys(descriptors).length===Object.keys(descriptors).length&&Object.values(descriptors).every(d=>Object.hasOwn(d,'value')&&d.enumerable),code);return Object.keys(descriptors);
}
function exactKeys(value,required,optional,code){
 const actual=fields(value,code);check(required.every(key=>actual.includes(key))&&actual.every(key=>required.includes(key)||optional.includes(key)),code);
}
function stringArray(value){
 check(Array.isArray(value),'EXECUTOR_ARGS');const descriptors=Object.getOwnPropertyDescriptors(value),keys=Reflect.ownKeys(descriptors),expected=[...value.keys()].map(String);
 check(keys.length===expected.length+1&&keys.at(-1)==='length'&&expected.every((key,index)=>keys[index]===key&&Object.hasOwn(descriptors[key],'value')&&descriptors[key].enumerable&&typeof descriptors[key].value==='string'&&!descriptors[key].value.includes('\0')),'EXECUTOR_ARGS');
}
function stringMap(value){
 const keys=fields(value,'EXECUTOR_ENV');
 for(const key of keys)check(key.length>0&&!key.includes('\0')&&!key.includes('=')&&typeof value[key]==='string'&&!value[key].includes('\0'),'EXECUTOR_ENV');
}
function dependencies(value){
 if(value===undefined)return {spawn:nodeSpawn,monotonicNow:process.hrtime.bigint,killGroup:(pid,signal)=>process.kill(-pid,signal)};
 const actual=fields(value,'EXECUTOR_DEPENDENCIES');check(actual.every(key=>DEPENDENCY_FIELDS.includes(key)),'EXECUTOR_DEPENDENCIES');
 for(const key of actual)check(typeof value[key]==='function','EXECUTOR_DEPENDENCIES');
 return {spawn:value.spawn??nodeSpawn,monotonicNow:value.monotonicNow??process.hrtime.bigint,killGroup:value.killGroup??((pid,signal)=>process.kill(-pid,signal))};
}
function validateHandoff(value){
 exactKeys(value,HANDOFF_FIELDS,HANDOFF_OPTIONAL_FIELDS,'EXECUTOR_HANDOFF_FIELDS');
 check(typeof value.parentDir==='string'&&isAbsolute(value.parentDir)&&resolve(value.parentDir)===value.parentDir&&!value.parentDir.includes('\0'),'EXECUTOR_HANDOFF_PARENT_DIR');
 check(Number.isSafeInteger(value.launcherPid)&&value.launcherPid>0&&typeof value.launcherStartTicks==='bigint'&&value.launcherStartTicks>=0n,'EXECUTOR_HANDOFF_LAUNCHER');
 for(const key of ['allocationId','actionId','chargeId','reservationId','storeIdentity'])check(typeof value[key]==='string'&&value[key].length>0&&!value[key].includes('\0')&&!value[key].includes('\n'),'EXECUTOR_HANDOFF_IDENTITY');
 for(const key of ['candidateHash','runnerDigest','executionContextSha256','projectionSha256','correspondenceSha256','authoritySha256'])check(typeof value[key]==='string'&&/^[0-9a-f]{64}$/.test(value[key]),'EXECUTOR_HANDOFF_DIGEST');
 check(Number.isSafeInteger(value.blockDeadlineUtc)&&value.blockDeadlineUtc>=0,'EXECUTOR_HANDOFF_BLOCK_DEADLINE');check(value.blockDeadlineMonotonicNs===null||typeof value.blockDeadlineMonotonicNs==='bigint'&&value.blockDeadlineMonotonicNs>=0n,'EXECUTOR_HANDOFF_BLOCK_DEADLINE');
 for(const key of ['appendInvocationEvent','confirmCurrentState','getPeerUid','getPeerPid'])check(typeof value[key]==='function','EXECUTOR_HANDOFF_DEPENDENCY');
 for(const key of HANDOFF_OPTIONAL_FIELDS)if(value[key]!==undefined)check(typeof value[key]==='function','EXECUTOR_HANDOFF_DEPENDENCY');
 check(Number.isSafeInteger(value.setupTimeoutMs)&&value.setupTimeoutMs>0,'EXECUTOR_HANDOFF_TIMEOUT');return value;
}
function validateRunOptions(value){
 exactKeys(value,RUN_FIELDS,RUN_OPTIONAL_FIELDS,'EXECUTOR_FIELDS');
 check(typeof value.command==='string'&&value.command.length>0&&!value.command.includes('\0'),'EXECUTOR_COMMAND');
 stringArray(value.args);
 check(typeof value.cwd==='string'&&isAbsolute(value.cwd)&&resolve(value.cwd)===value.cwd&&!value.cwd.includes('\0'),'EXECUTOR_CWD');
 check(Number.isSafeInteger(value.outerTimeoutMs)&&value.outerTimeoutMs>0,'EXECUTOR_TIMEOUT');
 check(typeof value.readDurableResult==='function','EXECUTOR_RESULT_READER');
 if(value.env!==undefined)stringMap(value.env);
 if(value.handoff!==undefined){check(value.env===undefined,'EXECUTOR_HANDOFF_ENV_CONFLICT');validateHandoff(value.handoff);}
 if(value.abandonAfterKillMs!==undefined)check(Number.isSafeInteger(value.abandonAfterKillMs)&&value.abandonAfterKillMs>0,'EXECUTOR_ABANDON_TIMEOUT');
 dependencies(value.dependencies);return value;
}

const realBootId=()=>readFileSync('/proc/sys/kernel/random/boot_id','utf8').trim();
const realWait=ms=>new Promise(resolveWait=>setTimeout(resolveWait,ms));
const errorClassOf=error=>typeof error?.name==='string'?error.name:'Error';
const errorMessageOf=error=>typeof error?.message==='string'?error.message:String(error);
const invoke=fn=>{try{return Promise.resolve(fn());}catch(error){return Promise.reject(error);}};
const attach=(target,key,value)=>{if(target!==null&&typeof target==='object')try{Object.defineProperty(target,key,{value,enumerable:false,configurable:true,writable:true});}catch{}};

/** One monotonic schedule per run, in milliseconds of performance.now(), which
 * is immune to wall-clock changes. outerMs ends execution and all fresh setup
 * work; settleMs ends setup waits and the optional result read; finalMs ends
 * cleanup and is the return bound. The gap between settleMs and finalMs is the
 * reserved cleanup margin. Wall time is never used for enforcement.
 */
function createDeadline(options){
 const startMs=performance.now(),abandonMs=options.abandonAfterKillMs??ABANDON_AFTER_KILL_MS,outerMs=startMs+options.outerTimeoutMs,finalMs=outerMs+abandonMs,cleanupReserveMs=Math.min(CLEANUP_RESERVE_MAX_MS,Math.floor(abandonMs/2)),settleMs=finalMs-cleanupReserveMs;
 return Object.freeze({startMs,outerMs,settleMs,finalMs,cleanupReserveMs,now:()=>performance.now(),remaining:until=>Math.max(0,until-performance.now())});
}
/** Wait for work until a monotonic instant. Resolves {settled:true,value},
 * {settled:true,error} or {settled:false}. The work is never cancelled; an
 * unsettled outcome means the promise is still outstanding.
 */
function waitUntil(work,untilMs){
 return new Promise(resolveWait=>{
  let done=false;const finish=outcome=>{if(done)return;done=true;clearTimeout(timer);resolveWait(outcome);};
  const timer=setTimeout(()=>finish({settled:false}),Math.max(0,untilMs-performance.now()));
  Promise.resolve(work).then(value=>finish({settled:true,value}),error=>finish({settled:true,error}));
 });
}
function setupTimeoutError(phase){const error=Error('EXECUTOR_SETUP_TIMEOUT');error.phase=phase;error.outstanding=true;return error;}
/** Await one pre-spawn setup phase against the outer deadline. A stalled phase
 * rejects with EXECUTOR_SETUP_TIMEOUT naming itself; a failed phase rethrows its
 * original error. onLate, when given, receives a value that fulfils after the
 * timeout so an acquired resource is cleaned instead of leaked; it never
 * advances the cancelled pipeline. The late outcome is exposed on the error as
 * lateAllocation.settled, which resolves only if the allocation ever fulfils.
 */
async function boundedPhase(phase,work,deadline,onLate=null){
 const outcome=await waitUntil(work,deadline.outerMs);
 if(!outcome.settled){
  const error=setupTimeoutError(phase);
  if(onLate){const settledLate=Promise.resolve(work).then(value=>onLate(value),()=>Object.freeze({status:'rejected'}));attach(error,'lateAllocation',Object.freeze({phase,settled:settledLate}));}
  throw error;
 }
 if(Object.hasOwn(outcome,'error'))throw outcome.error;
 return outcome.value;
}

async function parentStat(handoff,pid){const readProcStat=handoff.readProcStat??readProcStatReal;for(let attempt=0;attempt<5;attempt++){const stat=await readProcStat(pid);if(stat!==null){check(plain(stat)&&stat.pid===pid&&Number.isSafeInteger(stat.ppid)&&stat.ppid>=0&&typeof stat.startTicks==='bigint'&&stat.startTicks>=0n,'EXECUTOR_HANDOFF_PARENT_STAT');return stat;}if(attempt<4)await (handoff.wait??realWait)(10);}throw Error('HANDOFF_ANCESTRY_PID_GONE');}
function runtimeContext(runtimeDir){return {runtimeDir,server:null,socketPath:null,nonce:null,sockets:new Set(),pending:[],queueConnection:null,trackConnection:null,abort:new AbortController(),cancelled:false,cleanup:null,pendingProtocolWork:0,setup:null};}
/** Protocol persistence bookkeeping: counts denial records, denial writes and
 * response writes the protocol has started and not yet settled. The count is a
 * report, never a cancellation; late settlement decrements it after return.
 */
const protocolPersistence=context=>Object.freeze({begin:()=>{context.pendingProtocolWork++;},settle:()=>{context.pendingProtocolWork--;}});
const outstandingWorkOf=context=>context===null?null:Object.freeze({setupPending:context.setup?.state==='pending',setupPhase:context.setup?.phase??null,pendingProtocolWork:context.pendingProtocolWork});
async function removeHandoffRuntime(context){
 if(context.server){
  await new Promise(resolveClose=>{if(!context.server.listening){resolveClose();return;}let done=false;const finish=()=>{if(done)return;done=true;clearTimeout(closeTimer);resolveClose();},closeTimer=setTimeout(()=>{try{context.server.closeAllConnections?.();}catch{}finish();},HANDOFF_SERVER_CLOSE_GRACE_MS);try{context.server.close(finish);}catch{finish();}});
 }
 if(context.socketPath!==null){try{await unlink(context.socketPath);}catch(error){if(error?.code!=='ENOENT')throw error;}}
 try{await rmdir(context.runtimeDir);}catch(error){if(error?.code!=='ENOENT')throw error;}
}
const cleanupOutcome=(status,context,error=null)=>Object.freeze({status,errorClass:error===null?null:errorClassOf(error),errorMessage:error===null?null:errorMessageOf(error),runtimeDir:context.runtimeDir,socketPath:context.socketPath});
/** Dispose this run's protocol and remove only its socket, server and directory,
 * bounded by the remaining cleanup budget. Order: abort the handshake protocol so
 * no new denial record or response starts, detach the executor's listeners,
 * destroy tracked sockets, then run the remover. The outcome is reported, never
 * swallowed: 'removed', 'failed' with the error, or 'unresolved' when removal was
 * still outstanding at the bound.
 */
async function cleanupHandoffRuntime(context,handoff,untilMs){
 if(context.cleanup!==null)return context.cleanup;
 context.cancelled=true;context.abort.abort();
 if(context.server){if(context.queueConnection)context.server.removeListener('connection',context.queueConnection);if(context.trackConnection)context.server.removeListener('connection',context.trackConnection);}
 for(const socket of context.pending.splice(0))if(!socket.destroyed)socket.destroy();
 for(const socket of context.sockets)if(!socket.destroyed)socket.destroy();
 const outcome=await waitUntil(invoke(()=>(handoff.removeRuntime??removeHandoffRuntime)(context)),untilMs);
 context.cleanup=!outcome.settled?cleanupOutcome('unresolved',context):Object.hasOwn(outcome,'error')?cleanupOutcome('failed',context,outcome.error):cleanupOutcome('removed',context);
 return context.cleanup;
}
/** Late-fulfilled allocations are owned and removed here without any further
 * setup work; the remover is the same one the run would have used.
 */
async function cleanupLateAllocation(context,handoff){
 context.cancelled=true;context.abort.abort();
 try{await (handoff.removeRuntime??removeHandoffRuntime)(context);return cleanupOutcome('removed',context);}catch(error){return cleanupOutcome('failed',context,error);}
}
function queueHandoffConnections(context){const forget=socket=>context.sockets.delete(socket);context.queueConnection=socket=>{context.sockets.add(socket);socket.once('close',()=>forget(socket));socket.pause();context.pending.push(socket);};context.trackConnection=socket=>{context.sockets.add(socket);socket.once('close',()=>forget(socket));};context.server.addListener('connection',context.queueConnection);}
function activateHandoffServer(context,options){context.server.removeListener('connection',context.queueConnection);const result=runHandoffServer(options);context.server.addListener('connection',context.trackConnection);for(const socket of context.pending.splice(0)){if(socket.destroyed)continue;socket.resume();context.server.emit('connection',socket);}return result;}

export function classifyRawExit(value){
 exactKeys(value,['status','signal'],[],'EXECUTOR_RAW_EXIT_FIELDS');
 check(value.status===null||Number.isSafeInteger(value.status),'EXECUTOR_RAW_EXIT_STATUS');
 check(value.signal===null||typeof value.signal==='string','EXECUTOR_RAW_EXIT_SIGNAL');
 if(value.signal!==null)return {kind:'signal',code:osConstants.signals[value.signal]??null};
 if(typeof value.status==='number')return {kind:'exit',code:value.status};
 return {kind:'unknown',code:null};
}

function containment(result,rawExit){
 if(!plain(result))throw Error('EXECUTOR_DURABLE_RESULT');
 if(result.schema!=='moriarty.loan-process-result/1')throw Error('EXECUTOR_DURABLE_RESULT_SCHEMA');
 const observedStatus=rawExit.kind==='exit'&&rawExit.code===0?'PASS':rawExit.kind==='exit'||rawExit.kind==='signal'?'FAIL':null;
 if(!['PASS','FAIL'].includes(result.status)||result.status!==observedStatus)throw Error('EXECUTOR_DURABLE_RESULT_STATUS');
 if(typeof result.containmentComplete==='boolean')return result.containmentComplete;
 if(plain(result.cleanup)&&typeof result.cleanup.containmentComplete==='boolean')return result.cleanup.containmentComplete;
 return false;
}

/** Diagnostic record of the handoff setup and runtime cleanup. setup is
 * 'settled', 'failed' or 'pending'. pendingProtocolWork counts denial records
 * and denial/response writes the protocol started and had not settled at
 * return. outstandingWork is true when either the setup or any such protocol
 * work was still running, uncancelled, at return.
 */
function handoffDiagnostics(context,setup){
 if(context===null)return null;
 return Object.freeze({setup:setup.state,phase:setup.phase,appendConfirmed:setup.appendConfirmed,handshake:setup.handshake,denialCode:setup.denialCode,errorClass:setup.errorClass,errorMessage:setup.errorMessage,pendingProtocolWork:context.pendingProtocolWork,outstandingWork:setup.state==='pending'||context.pendingProtocolWork>0,runtimeCleanup:context.cleanup});
}

async function runPrepared(options,outerStartMonotonicNs,preSpawn,deadline){
 const deps=dependencies(options.dependencies);let handoffContext=null,childEnv=options.env??{};
 try{
  if(options.handoff){
   const handoff=options.handoff,makeDirectory=handoff.createRuntimeDirectory??createRuntimeDirectory,makeSocket=handoff.createInvocationSocket??createInvocationSocket,makeNonce=handoff.generateNonce??generateNonce;
   const runtimeDir=await boundedPhase('runtime-directory',invoke(()=>makeDirectory({parentDir:handoff.parentDir})),deadline,lateDir=>cleanupLateAllocation(runtimeContext(lateDir),handoff));
   // The directory is owned from this point; every later failure path reports its removal.
   handoffContext=runtimeContext(runtimeDir);
   const created=await boundedPhase('invocation-socket',invoke(()=>makeSocket({runtimeDir})),deadline,late=>{const context=runtimeContext(runtimeDir);context.server=late?.server??null;context.socketPath=late?.socketPath??null;return cleanupLateAllocation(context,handoff);});
   // Acquired server and socket path are owned before any further fallible work.
   handoffContext.server=created.server;handoffContext.socketPath=created.socketPath;
   queueHandoffConnections(handoffContext);
   handoffContext.nonce=makeNonce();
   childEnv=buildInvocationEnv({socketPath:created.socketPath,nonceHex:nonceHex(handoffContext.nonce)});
  }
  const bootId=options.handoff?await boundedPhase('boot-id',invoke(options.handoff.readBootId??realBootId),deadline):null;
  // Rechecked immediately before spawn, after every awaited setup phase: the
  // outer execution deadline, then the prover latest-start bound. Abandonment
  // is a return and cleanup allowance, never an allowance for fresh work.
  if(deadline.now()>=deadline.outerMs){const error=Error('EXECUTOR_LATE_START');error.phase='spawn';error.outstanding=false;throw error;}
  if(preSpawn)preSpawn();
  // The legacy path remains a caller-supplied closed environment. A configured
  // handoff replaces it with exactly the socket path and raw nonce variables.
  const child=deps.spawn(options.command,[...options.args],{cwd:options.cwd,detached:true,stdio:['ignore','pipe','pipe'],env:childEnv});
  check(child&&Number.isSafeInteger(child.pid)&&child.pid>0&&typeof child.once==='function'&&typeof child.stdout?.on==='function'&&typeof child.stderr?.on==='function','EXECUTOR_CHILD');
  return await new Promise((resolveResult,reject)=>{
   const chunks={stdout:[],stderr:[]};let diagnosticBytes=0,diagnosticsFull=false,deadlineExceeded=false,killRequested=false,killErrorClass=null,settled=false,timer,abandonTimer,startupValid=true,startupWriteFailed=false,parentLost=false,knownMainFailure=false;
   const setup={state:'settled',phase:null,appendConfirmed:false,handshake:null,denialCode:null,errorClass:null,errorMessage:null};if(handoffContext)handoffContext.setup=setup;
   // Fresh setup work stops at cancellation or at the outer deadline.
   const cancelled=()=>handoffContext.cancelled||deadline.now()>=deadline.outerMs;
   const handoffSetup=options.handoff?(async()=>{
    setup.state='pending';setup.phase='parent-stat';
    try{
     const stat=await parentStat(options.handoff,child.pid);if(cancelled())return null;
     const uncapped=outerStartMonotonicNs+BigInt(HANDOFF_OUTER_SECONDS)*1000000000n,outerDeadlineMonotonic=options.handoff.blockDeadlineMonotonicNs===null?uncapped:options.handoff.blockDeadlineMonotonicNs<uncapped?options.handoff.blockDeadlineMonotonicNs:uncapped;
     const payload=buildInvocationEventPayload({allocationId:options.handoff.allocationId,actionId:options.handoff.actionId,candidateHash:options.handoff.candidateHash,runnerDigest:options.handoff.runnerDigest,chargeId:options.handoff.chargeId,reservationId:options.handoff.reservationId,storeIdentity:options.handoff.storeIdentity,executionContextSha256:options.handoff.executionContextSha256,projectionSha256:options.handoff.projectionSha256,correspondenceSha256:options.handoff.correspondenceSha256,authoritySha256:options.handoff.authoritySha256,bootId,outerStartMonotonic:outerStartMonotonicNs,outerDeadlineMonotonic,blockDeadlineUtc:options.handoff.blockDeadlineUtc,parentPid:child.pid,parentStartTicks:stat.startTicks,launcherPid:options.handoff.launcherPid,launcherStartTicks:options.handoff.launcherStartTicks,nonceSha256:nonceSha256Hex(handoffContext.nonce)}),payloadSha256=invocationPayloadSha256(payload);
     setup.phase='append';
     await options.handoff.appendInvocationEvent(Object.freeze({payload,payloadSha256}));
     // A late append after cancellation or the outer deadline must not open the
     // handshake on a cleaned-up server or answer a socket this run no longer owns.
     if(cancelled())return null;setup.appendConfirmed=true;
     setup.phase='handshake';
     const result=await activateHandoffServer(handoffContext,{server:handoffContext.server,timeoutMs:Math.max(1,Math.min(options.handoff.setupTimeoutMs,Math.floor(deadline.remaining(deadline.outerMs)))),expectedNonce:handoffContext.nonce,expectedAuthoritySha256:options.handoff.authoritySha256,expectedBootId:bootId,expectedUid:process.getuid(),invocationPayloadSha256:payloadSha256,ancestry:{launcherPid:options.handoff.launcherPid,launcherStartTicks:options.handoff.launcherStartTicks,readProcStat:options.handoff.readProcStat??readProcStatReal,maxDepth:HANDOFF_ANCESTRY_MAX_DEPTH},readBootId:options.handoff.readBootId??realBootId,getPeerUid:options.handoff.getPeerUid,getPeerPid:options.handoff.getPeerPid,confirmCurrentState:options.handoff.confirmCurrentState,buildResponse:()=>Object.freeze({payload,payloadSha256}),recordDenialEvent:options.handoff.appendDenialEvent??options.handoff.appendInvocationEvent,signal:handoffContext.abort.signal,persistence:protocolPersistence(handoffContext)});
     if(result.status==='HANDSHAKE_ABORTED'||handoffContext.cancelled){if(result.status==='HANDSHAKE_OK'&&!result.socket.destroyed)result.socket.destroy();setup.handshake='cancelled';return null;}
     if(result.status==='HANDSHAKE_DENIED'){startupValid=false;setup.handshake='denied';setup.denialCode=result.code;}
     else{setup.handshake='ok';let classified=false;const eof=()=>{if(classified)return;classified=true;parentLost=classifyEofAfterStartup({retainedKnownMainFailure:knownMainFailure}).parentLost;};result.socket.once('end',eof);result.socket.once('close',eof);}
     setup.state='settled';setup.phase='complete';return result;
    }catch(error){
     if(handoffContext.cancelled)return null;
     startupValid=false;startupWriteFailed=true;setup.state='failed';setup.errorClass=errorClassOf(error);setup.errorMessage=errorMessageOf(error);return null;
    }
   })():Promise.resolve(null);
   const append=(which,data)=>{
    if(diagnosticsFull)return;const bytes=Buffer.isBuffer(data)?data:Buffer.from(data),remaining=DIAGNOSTIC_MAX_BYTES-diagnosticBytes;
    if(bytes.length>=remaining){if(remaining>0)chunks[which].push(bytes.subarray(0,remaining));diagnosticBytes=DIAGNOSTIC_MAX_BYTES;diagnosticsFull=true;return;}
    chunks[which].push(bytes);diagnosticBytes+=bytes.length;
   };
   const clearTimers=()=>{clearTimeout(timer);clearTimeout(abandonTimer);};
   const complete=async(rawExit,matrixRawExit,readResult=true)=>{
    // Setup is awaited only until settleMs. A setup still pending there is
    // cancelled for this run and reported as outstanding work; it never counts as
    // persisted evidence or a valid startup.
    const setupOutcome=await waitUntil(handoffSetup,deadline.settleMs);
    if(!setupOutcome.settled){if(handoffContext)handoffContext.cancelled=true;setup.state='pending';setup.handshake=setup.phase==='handshake'?'cancelled':null;startupValid=false;startupWriteFailed=setup.phase==='append';}
    else if(handoffContext&&setup.state==='pending'){setup.state='settled';startupValid=false;}
    knownMainFailure=rawExit.kind==='exit'&&rawExit.code!==0||rawExit.kind==='signal';let resultReadFailed=false,resultReadSkipped=false,contained=false;
    if(readResult){
     // The optional durable read draws only on the remaining settle budget and
     // is not started once that budget is exhausted.
     const budget=Math.min(RESULT_READ_GRACE_MS,deadline.remaining(deadline.settleMs));
     // Timers have millisecond granularity and may fire up to one millisecond
     // early against performance.now(); a budget under two milliseconds cannot
     // start a bounded observation and is treated as exhausted.
     if(budget<2){resultReadFailed=true;resultReadSkipped=true;}
     else{const read=await waitUntil(invoke(options.readDurableResult),deadline.now()+budget);if(!read.settled||Object.hasOwn(read,'error'))resultReadFailed=true;else try{contained=containment(read.value,rawExit);}catch{resultReadFailed=true;}}
    }
    const matrix={
     refused:null,startupValid,startupWriteFailed,evidencePreexists:false,invocationIdMismatch:false,rawExit:matrixRawExit,
     terminalEvidencePersisted:false,stopReturnCode:null,stopErrorClass:null,stopReceiptPersisted:false,
     containmentComplete:contained,timerCancelReturnCode:null,timerCancelReceiptPersisted:false,
     deadlineExceeded,parentLost,resultWriteFailed:false
    };
    const disposition=Object.freeze(classifyDisposition(matrix));
    if(handoffContext)await cleanupHandoffRuntime(handoffContext,options.handoff,deadline.finalMs);
    const diagnostics=Object.freeze({stdout:Buffer.concat(chunks.stdout).toString(),stderr:Buffer.concat(chunks.stderr).toString(),deadlineExceeded,resultReadFailed,resultReadSkipped,killErrorClass,handoff:handoffDiagnostics(handoffContext,setup)});
    return Object.freeze({outerStartMonotonicNs,rawExit,disposition,diagnostics});
   };
   child.stdout.on('data',data=>append('stdout',data));child.stderr.on('data',data=>append('stderr',data));
   timer=setTimeout(()=>{
    deadlineExceeded=true;
    try{deps.killGroup(child.pid,'SIGKILL');killRequested=true;}catch(error){killErrorClass=errorClassOf(error);}
    if(settled)return;
    // Abandonment fires at settleMs so the reserved margin remains for cleanup.
    abandonTimer=setTimeout(()=>{
     if(settled)return;settled=true;clearTimers();const rawExit=Object.freeze({kind:'unknown',code:null});
     void complete(rawExit,rawExit,false).then(resolveResult,reject);
    },deadline.remaining(deadline.settleMs));
   },deadline.remaining(deadline.outerMs));
   child.once('error',error=>{if(settled)return;settled=true;clearTimers();reject(error);});
   child.once('close',(status,signal)=>{
    try{
     const rawExit=Object.freeze(classifyRawExit({status,signal}));
     knownMainFailure=rawExit.kind==='exit'&&rawExit.code!==0||rawExit.kind==='signal';
     if(settled)return;settled=true;clearTimers();
     // Once this caller requested SIGKILL, a later close cannot establish whether
     // the reported terminal state was natural or self-inflicted. Keep the real
     // raw exit in the result, but classify the main observation as unavailable.
     const matrixRawExit=killRequested?Object.freeze({kind:'unknown',code:null}):rawExit;
     void complete(rawExit,matrixRawExit).then(resolveResult,reject);
    }catch(error){if(settled)return;settled=true;clearTimers();reject(error);}
   });
  });
 }catch(error){
  // Preserve the original failure; attach the owned-runtime cleanup outcome so a
  // leaked or unresolved socket/directory is never hidden behind the rejection.
  if(handoffContext!==null){attach(error,'runtimeCleanup',await cleanupHandoffRuntime(handoffContext,options.handoff,deadline.finalMs));attach(error,'outstandingWork',outstandingWorkOf(handoffContext));}
  throw error;
 }
}

export async function runExitRetainingExecutor(options){
 validateRunOptions(options);const deps=dependencies(options.dependencies);
 const outerStartMonotonicNs=deps.monotonicNow();check(typeof outerStartMonotonicNs==='bigint'&&outerStartMonotonicNs>=0n,'EXECUTOR_MONOTONIC');
 return runPrepared(options,outerStartMonotonicNs,undefined,createDeadline(options));
}

/** Establish and persist the immutable control bytes before spawn. The legacy
 * requireControlRecord flag remains accepted but can no longer disable the write.
 * bootId, timeNamespaceInode and invocationDigest supply the pure record fields.
 * The control write shares the run's outer deadline; a stalled write rejects
 * with EXECUTOR_SETUP_TIMEOUT before any spawn and is reported as outstanding.
 * The latest-start check runs immediately before spawn, after every await.
 */
export async function runProverBoundedExecutor(options){
 exactKeys(options,RUN_FIELDS,BOUNDED_FIELDS.filter(key=>!RUN_FIELDS.includes(key)),'EXECUTOR_PROVER_FIELDS');
 const requireControlRecord=options.requireControlRecord??true;
 check(typeof requireControlRecord==='boolean','EXECUTOR_PROVER_CONTROL_REQUIRED');
 check(options.writeControlRecord===undefined||typeof options.writeControlRecord==='function','EXECUTOR_PROVER_CONTROL_WRITER');
 const runOptions=Object.fromEntries([...RUN_FIELDS,...RUN_OPTIONAL_FIELDS].filter(key=>Object.hasOwn(options,key)).map(key=>[key,options[key]]));validateRunOptions(runOptions);
 check(typeof options.writeControlRecord==='function','EXECUTOR_PROVER_CONTROL_REQUIRED');
 const deps=dependencies(runOptions.dependencies),outerStartMonotonicNs=deps.monotonicNow(),deadline=createDeadline(runOptions);
 const proverLifetimeBounds=Object.freeze(computeProverLifetimeBounds(outerStartMonotonicNs));
 const record=encodeProverControlRecord({bootId:options.bootId,timeNamespaceInode:options.timeNamespaceInode,...proverLifetimeBounds,invocationDigest:options.invocationDigest});
 await boundedPhase('control-record',invoke(()=>options.writeControlRecord(record)),deadline);
 const result=await runPrepared(runOptions,outerStartMonotonicNs,()=>validateWrapperEntry(deps.monotonicNow(),proverLifetimeBounds.latestStartMonotonicNs),deadline);
 return Object.freeze({proverLifetimeBounds,...result});
}
