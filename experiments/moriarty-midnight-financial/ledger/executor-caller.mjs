/** One-shot child caller with raw exit retention before post-close work. Stdout
 * and stderr are bounded diagnostics only; durable validated facts decide success.
 */
import {spawn as nodeSpawn} from 'node:child_process';
import {constants as osConstants} from 'node:os';
import {isAbsolute,resolve} from 'node:path';
import {beforeDeadline} from './receipt.mjs';
import {classifyDisposition} from './fault-matrix.mjs';
import {computeProverLifetimeBounds,encodeProverControlRecord,validateWrapperEntry} from './prover-lifetime.mjs';

const DIAGNOSTIC_MAX_BYTES=1048576;
const RESULT_READ_GRACE_MS=1000;
const ABANDON_AFTER_KILL_MS=1000;
const RUN_FIELDS=['command','args','cwd','outerTimeoutMs','readDurableResult'];
const RUN_OPTIONAL_FIELDS=['dependencies','env','abandonAfterKillMs'];
const DEPENDENCY_FIELDS=['spawn','monotonicNow','killGroup'];
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
function validateRunOptions(value){
 exactKeys(value,RUN_FIELDS,RUN_OPTIONAL_FIELDS,'EXECUTOR_FIELDS');
 check(typeof value.command==='string'&&value.command.length>0&&!value.command.includes('\0'),'EXECUTOR_COMMAND');
 stringArray(value.args);
 check(typeof value.cwd==='string'&&isAbsolute(value.cwd)&&resolve(value.cwd)===value.cwd&&!value.cwd.includes('\0'),'EXECUTOR_CWD');
 check(Number.isSafeInteger(value.outerTimeoutMs)&&value.outerTimeoutMs>0,'EXECUTOR_TIMEOUT');
 check(typeof value.readDurableResult==='function','EXECUTOR_RESULT_READER');
 if(value.env!==undefined)stringMap(value.env);
 if(value.abandonAfterKillMs!==undefined)check(Number.isSafeInteger(value.abandonAfterKillMs)&&value.abandonAfterKillMs>0,'EXECUTOR_ABANDON_TIMEOUT');
 dependencies(value.dependencies);return value;
}

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

async function runPrepared(options,outerStartMonotonicNs,preSpawn){
 const deps=dependencies(options.dependencies);if(preSpawn)preSpawn();
 // Pass a closed environment, never implicit parent inheritance. The caller may
 // populate the out-of-scope invocation handoff variables through this field.
 const child=deps.spawn(options.command,[...options.args],{cwd:options.cwd,detached:true,stdio:['ignore','pipe','pipe'],env:options.env??{}});
 check(child&&Number.isSafeInteger(child.pid)&&child.pid>0&&typeof child.once==='function'&&typeof child.stdout?.on==='function'&&typeof child.stderr?.on==='function','EXECUTOR_CHILD');
 return new Promise((resolveResult,reject)=>{
  const chunks={stdout:[],stderr:[]};let diagnosticBytes=0,diagnosticsFull=false,deadlineExceeded=false,killRequested=false,killErrorClass=null,settled=false,timer,abandonTimer;
  const append=(which,data)=>{
   if(diagnosticsFull)return;const bytes=Buffer.isBuffer(data)?data:Buffer.from(data),remaining=DIAGNOSTIC_MAX_BYTES-diagnosticBytes;
   if(bytes.length>=remaining){if(remaining>0)chunks[which].push(bytes.subarray(0,remaining));diagnosticBytes=DIAGNOSTIC_MAX_BYTES;diagnosticsFull=true;return;}
   chunks[which].push(bytes);diagnosticBytes+=bytes.length;
  };
  const clearTimers=()=>{clearTimeout(timer);clearTimeout(abandonTimer);};
  const complete=async(rawExit,matrixRawExit,readResult=true)=>{
   let durable,resultReadFailed=false,contained=false;
   if(readResult){
    try{durable=await beforeDeadline(options.readDurableResult,Date.now()+RESULT_READ_GRACE_MS);contained=containment(durable,rawExit);}
    catch{resultReadFailed=true;}
   }
   const matrix={
    refused:null,startupValid:true,startupWriteFailed:false,evidencePreexists:false,invocationIdMismatch:false,rawExit:matrixRawExit,
    terminalEvidencePersisted:false,stopReturnCode:null,stopErrorClass:null,stopReceiptPersisted:false,
    containmentComplete:contained,timerCancelReturnCode:null,timerCancelReceiptPersisted:false,
    deadlineExceeded,parentLost:false,resultWriteFailed:false
   };
   const disposition=Object.freeze(classifyDisposition(matrix));
   const diagnostics=Object.freeze({stdout:Buffer.concat(chunks.stdout).toString(),stderr:Buffer.concat(chunks.stderr).toString(),deadlineExceeded,resultReadFailed,killErrorClass});
   return Object.freeze({outerStartMonotonicNs,rawExit,disposition,diagnostics});
  };
  child.stdout.on('data',data=>append('stdout',data));child.stderr.on('data',data=>append('stderr',data));
  timer=setTimeout(()=>{
   deadlineExceeded=true;
   try{deps.killGroup(child.pid,'SIGKILL');killRequested=true;}catch(error){killErrorClass=typeof error?.name==='string'?error.name:'Error';}
   if(settled)return;
   abandonTimer=setTimeout(()=>{
    if(settled)return;settled=true;clearTimers();const rawExit=Object.freeze({kind:'unknown',code:null});
    void complete(rawExit,rawExit,false).then(resolveResult,reject);
   },options.abandonAfterKillMs??ABANDON_AFTER_KILL_MS);
  },options.outerTimeoutMs);
  child.once('error',error=>{if(settled)return;settled=true;clearTimers();reject(error);});
  child.once('close',(status,signal)=>{
   try{
    const rawExit=Object.freeze(classifyRawExit({status,signal}));
    if(settled)return;settled=true;clearTimers();
    // Once this caller requested SIGKILL, a later close cannot establish whether
    // the reported terminal state was natural or self-inflicted. Keep the real
    // raw exit in the result, but classify the main observation as unavailable.
    const matrixRawExit=killRequested?Object.freeze({kind:'unknown',code:null}):rawExit;
    void complete(rawExit,matrixRawExit).then(resolveResult,reject);
   }catch(error){if(settled)return;settled=true;clearTimers();reject(error);}
  });
 });
}

export async function runExitRetainingExecutor(options){
 validateRunOptions(options);const deps=dependencies(options.dependencies);
 const outerStartMonotonicNs=deps.monotonicNow();check(typeof outerStartMonotonicNs==='bigint'&&outerStartMonotonicNs>=0n,'EXECUTOR_MONOTONIC');
 return runPrepared(options,outerStartMonotonicNs);
}

/** Establish and persist the immutable control bytes before spawn. The legacy
 * requireControlRecord flag remains accepted but can no longer disable the write.
 * bootId, timeNamespaceInode and invocationDigest supply the pure record fields.
 */
export async function runProverBoundedExecutor(options){
 exactKeys(options,RUN_FIELDS,BOUNDED_FIELDS.filter(key=>!RUN_FIELDS.includes(key)),'EXECUTOR_PROVER_FIELDS');
 const requireControlRecord=options.requireControlRecord??true;
 check(typeof requireControlRecord==='boolean','EXECUTOR_PROVER_CONTROL_REQUIRED');
 check(options.writeControlRecord===undefined||typeof options.writeControlRecord==='function','EXECUTOR_PROVER_CONTROL_WRITER');
 const runOptions=Object.fromEntries([...RUN_FIELDS,...RUN_OPTIONAL_FIELDS].filter(key=>Object.hasOwn(options,key)).map(key=>[key,options[key]]));validateRunOptions(runOptions);
 check(typeof options.writeControlRecord==='function','EXECUTOR_PROVER_CONTROL_REQUIRED');
 const deps=dependencies(runOptions.dependencies),outerStartMonotonicNs=deps.monotonicNow();
 const proverLifetimeBounds=Object.freeze(computeProverLifetimeBounds(outerStartMonotonicNs));
 const record=encodeProverControlRecord({bootId:options.bootId,timeNamespaceInode:options.timeNamespaceInode,...proverLifetimeBounds,invocationDigest:options.invocationDigest});
 await options.writeControlRecord(record);
 const result=await runPrepared(runOptions,outerStartMonotonicNs,()=>validateWrapperEntry(deps.monotonicNow(),proverLifetimeBounds.latestStartMonotonicNs));
 return Object.freeze({proverLifetimeBounds,...result});
}
