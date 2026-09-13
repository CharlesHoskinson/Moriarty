import test from 'node:test';
import assert from 'node:assert/strict';
import {EventEmitter} from 'node:events';
import {constants as osConstants} from 'node:os';
import {decodeProverControlRecord,computeProverLifetimeBounds} from './prover-lifetime.mjs';
import {classifyRawExit,runExitRetainingExecutor,runProverBoundedExecutor} from './executor-caller.mjs';

const command=process.execPath,cwd='/synthetic/moriarty-midnight-financial';
const args=['ledger/launch-local.mjs','--run','--plan','/synthetic/private/plan.json','--sha256','ab'.repeat(32)];
const durableResult=(containmentComplete=true)=>({schema:'moriarty.loan-process-result/1',status:'PASS',cleanup:{containmentComplete}});

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
 assert.deepEqual(result.diagnostics,{stdout:'',stderr:'',deadlineExceeded:false,resultReadFailed:false,killErrorClass:null});
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
