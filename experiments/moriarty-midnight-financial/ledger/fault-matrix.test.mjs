import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {classifyDisposition} from './fault-matrix.mjs';

const success=()=>({
 refused:null,startupValid:true,startupWriteFailed:false,evidencePreexists:false,invocationIdMismatch:false,
 rawExit:{kind:'exit',code:0},terminalEvidencePersisted:true,stopReturnCode:0,stopErrorClass:null,
 stopReceiptPersisted:true,containmentComplete:true,timerCancelReturnCode:0,timerCancelReceiptPersisted:true,
 deadlineExceeded:false,parentLost:false,resultWriteFailed:false
});
const expected=(status,exitCode,failureCode)=>({status,exitCode,failureCode});
const classify=change=>{const value=success();change(value);return classifyDisposition(value);};

test('row: complete zero terminal evidence and containment succeeds',()=>{
 assert.deepEqual(classifyDisposition(success()),expected('PROCESS_SUCCESS',0,null));
});
test('row: malformed startup is unknown',()=>{
 assert.deepEqual(classify(x=>x.startupValid=false),expected('PROCESS_UNKNOWN',3,'STARTUP_INVALID'));
});
test('row: unpersisted startup reports the specific write failure',()=>{
 assert.deepEqual(classify(x=>{x.startupValid=false;x.startupWriteFailed=true;}),expected('PROCESS_UNKNOWN',3,'EVIDENCE_WRITE_FAILED'));
});
test('row: invocation identity mismatch precedes an apparent main failure',()=>{
 assert.deepEqual(classify(x=>{x.invocationIdMismatch=true;x.rawExit={kind:'exit',code:9};}),expected('PROCESS_UNKNOWN',3,'MAIN_OBSERVATION_INVALID'));
});
test('row: preexisting evidence is unknown and not reused',()=>{
 assert.deepEqual(classify(x=>x.evidencePreexists=true),expected('PROCESS_UNKNOWN',3,'EVIDENCE_PREEXISTS'));
});
test('row: retained nonzero main exit is failed',()=>{
 assert.deepEqual(classify(x=>x.rawExit={kind:'exit',code:7}),expected('PROCESS_FAILED',1,'MAIN_EXIT_NONZERO'));
});
test('row: retained main signal is failed',()=>{
 assert.deepEqual(classify(x=>x.rawExit={kind:'signal',code:9}),expected('PROCESS_FAILED',1,'MAIN_SIGNAL'));
});
test('row: unavailable main exit is unknown before evidence gates',()=>{
 assert.deepEqual(classify(x=>x.rawExit={kind:'unknown',code:null}),expected('PROCESS_UNKNOWN',3,'MAIN_EXIT_UNAVAILABLE'));
});
test('row: absent raw main-exit observation is unknown even when every other fact says success',()=>{
 // GPT-6 H2 reproducer: rawExit:null with otherwise complete success evidence.
 assert.deepEqual(classify(x=>x.rawExit=null),expected('PROCESS_UNKNOWN',3,'MAIN_EXIT_UNAVAILABLE'));
});
test('row: only a retained exact zero exit reaches success',()=>{
 for(const rawExit of [null,{kind:'unknown',code:null}])assert.equal(classify(x=>x.rawExit=rawExit).status,'PROCESS_UNKNOWN');
 for(const rawExit of [{kind:'exit',code:1},{kind:'exit',code:-1},{kind:'signal',code:9},{kind:'signal',code:null}])assert.equal(classify(x=>x.rawExit=rawExit).status,'PROCESS_FAILED');
 assert.deepEqual(classify(x=>x.rawExit={kind:'exit',code:0}),expected('PROCESS_SUCCESS',0,null));
});
test('row: absent raw observation keeps refusal, startup and evidence precedence',()=>{
 assert.deepEqual(classify(x=>{x.rawExit=null;x.refused={code:'CONTAINMENT_UNSUPPORTED'};}),expected('REFUSED',2,'CONTAINMENT_UNSUPPORTED'));
 assert.deepEqual(classify(x=>{x.rawExit=null;x.startupValid=false;}),expected('PROCESS_UNKNOWN',3,'STARTUP_INVALID'));
 assert.deepEqual(classify(x=>{x.rawExit=null;x.startupValid=false;x.startupWriteFailed=true;}),expected('PROCESS_UNKNOWN',3,'EVIDENCE_WRITE_FAILED'));
 assert.deepEqual(classify(x=>{x.rawExit=null;x.evidencePreexists=true;}),expected('PROCESS_UNKNOWN',3,'EVIDENCE_PREEXISTS'));
 assert.deepEqual(classify(x=>{x.rawExit=null;x.invocationIdMismatch=true;}),expected('PROCESS_UNKNOWN',3,'MAIN_OBSERVATION_INVALID'));
 assert.deepEqual(classify(x=>{x.rawExit=null;x.deadlineExceeded=true;x.parentLost=true;x.terminalEvidencePersisted=false;}),expected('PROCESS_UNKNOWN',3,'MAIN_EXIT_UNAVAILABLE'));
});
test('row: known main failure stays failed despite every later cleanup limitation',()=>{
 assert.deepEqual(classify(x=>{x.rawExit={kind:'exit',code:1};x.terminalEvidencePersisted=false;x.stopReturnCode=null;x.stopErrorClass='TIMEOUT';x.stopReceiptPersisted=false;x.deadlineExceeded=true;x.parentLost=true;x.containmentComplete=false;x.timerCancelReturnCode=null;x.timerCancelReceiptPersisted=false;x.resultWriteFailed=true;}),expected('PROCESS_FAILED',1,'MAIN_EXIT_NONZERO'));
});
test('row: terminal evidence write or fsync failure is unknown',()=>{
 assert.deepEqual(classify(x=>x.terminalEvidencePersisted=false),expected('PROCESS_UNKNOWN',3,'EVIDENCE_WRITE_FAILED'));
});
test('row: stop nonzero is unknown',()=>{
 assert.deepEqual(classify(x=>x.stopReturnCode=4),expected('PROCESS_UNKNOWN',3,'STOP_FAILED'));
});
test('row: stop exception or timeout is unknown',()=>{
 assert.deepEqual(classify(x=>x.stopErrorClass='TimeoutError'),expected('PROCESS_UNKNOWN',3,'STOP_FAILED'));
});
test('row: successful stop with missing receipt is unknown',()=>{
 assert.deepEqual(classify(x=>x.stopReceiptPersisted=false),expected('PROCESS_UNKNOWN',3,'STOP_RECEIPT_FAILED'));
});
test('row: deadline crossing is unknown',()=>{
 assert.deepEqual(classify(x=>x.deadlineExceeded=true),expected('PROCESS_UNKNOWN',3,'DEADLINE_EXCEEDED'));
});
test('row: parent loss is unknown',()=>{
 assert.deepEqual(classify(x=>x.parentLost=true),expected('PROCESS_UNKNOWN',3,'PARENT_LOST'));
});
test('row: incomplete containment is unknown',()=>{
 assert.deepEqual(classify(x=>x.containmentComplete=false),expected('PROCESS_UNKNOWN',3,'CONTAINMENT_UNRESOLVED'));
});
test('row: timer cancellation nonzero is unknown',()=>{
 assert.deepEqual(classify(x=>x.timerCancelReturnCode=1),expected('PROCESS_UNKNOWN',3,'TIMER_CANCEL_FAILED'));
});
test('row: missing timer cancellation receipt is unknown',()=>{
 assert.deepEqual(classify(x=>x.timerCancelReceiptPersisted=false),expected('PROCESS_UNKNOWN',3,'TIMER_CANCEL_FAILED'));
});
test('row: final result write or fsync failure is unknown',()=>{
 assert.deepEqual(classify(x=>x.resultWriteFailed=true),expected('PROCESS_UNKNOWN',3,'RESULT_WRITE_FAILED'));
});
test('row: pre-start refusal preserves the validator code',()=>{
 assert.deepEqual(classify(x=>{x.refused={code:'CONTAINMENT_UNSUPPORTED'};x.startupValid=false;}),expected('REFUSED',2,'CONTAINMENT_UNSUPPORTED'));
});

test('precedence follows the closed table from top to bottom',()=>{
 assert.deepEqual(classify(x=>{x.startupValid=false;x.invocationIdMismatch=true;x.rawExit={kind:'signal',code:9};}),expected('PROCESS_UNKNOWN',3,'STARTUP_INVALID'));
 assert.deepEqual(classify(x=>{x.evidencePreexists=true;x.invocationIdMismatch=true;}),expected('PROCESS_UNKNOWN',3,'MAIN_OBSERVATION_INVALID'));
 assert.deepEqual(classify(x=>{x.deadlineExceeded=true;x.parentLost=true;}),expected('PROCESS_UNKNOWN',3,'DEADLINE_EXCEEDED'));
});

test('malformed or open inputs are rejected instead of defaulted',()=>{
 const cases=[
  null,{...success(),extra:true},{...success(),startupValid:1},{...success(),refused:{}},
  {...success(),rawExit:{kind:'other',code:null}},{...success(),stopReturnCode:'0'},
  {...success(),stopErrorClass:7},{...success(),rawExit:{kind:'exit',code:0,extra:true}}
 ];
 for(const value of cases)assert.throws(()=>classifyDisposition(value),error=>error.message.startsWith('FAULT_MATRIX_'));
});

test('safety cleanup is outside the pure classifier and cannot manufacture success evidence',()=>{
 const source=readFileSync(new URL('./fault-matrix.mjs',import.meta.url),'utf8');
 assert.doesNotMatch(source,/node:(?:fs|child_process)|process\.kill|\bspawn\s*\(/);
 const unresolved=success();unresolved.terminalEvidencePersisted=false;
 assert.deepEqual(classifyDisposition(unresolved),expected('PROCESS_UNKNOWN',3,'EVIDENCE_WRITE_FAILED'));
});
