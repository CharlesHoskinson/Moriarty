/** Pure closed fault disposition. Classification performs no cleanup or I/O;
 * callers must supply only facts they independently established.
 */

const FIELDS='refused,startupValid,startupWriteFailed,evidencePreexists,invocationIdMismatch,rawExit,terminalEvidencePersisted,stopReturnCode,stopErrorClass,stopReceiptPersisted,containmentComplete,timerCancelReturnCode,timerCancelReceiptPersisted,deadlineExceeded,parentLost,resultWriteFailed';
const check=(ok,code)=>{if(!ok)throw Error(code);};
const plain=value=>value!==null&&typeof value==='object'&&Object.getPrototypeOf(value)===Object.prototype;
function exact(value,keys,code='FAULT_MATRIX_FIELDS'){
 check(plain(value),code);const descriptors=Object.getOwnPropertyDescriptors(value);
 check(Reflect.ownKeys(descriptors).length===Object.keys(descriptors).length&&Object.values(descriptors).every(d=>Object.hasOwn(d,'value')&&d.enumerable)&&Object.keys(descriptors).sort().join('|')===keys.split(',').sort().join('|'),code);
}
const integerOrNull=value=>value===null||Number.isSafeInteger(value);
const result=(status,exitCode,failureCode)=>({status,exitCode,failureCode});

export function classifyDisposition(value){
 exact(value,FIELDS);
 for(const key of ['startupValid','startupWriteFailed','evidencePreexists','invocationIdMismatch','terminalEvidencePersisted','stopReceiptPersisted','containmentComplete','timerCancelReceiptPersisted','deadlineExceeded','parentLost','resultWriteFailed'])check(typeof value[key]==='boolean','FAULT_MATRIX_TYPES');
 check(integerOrNull(value.stopReturnCode)&&integerOrNull(value.timerCancelReturnCode),'FAULT_MATRIX_TYPES');
 check(value.stopErrorClass===null||typeof value.stopErrorClass==='string','FAULT_MATRIX_TYPES');
 if(value.refused!==null){exact(value.refused,'code','FAULT_MATRIX_REFUSED');check(typeof value.refused.code==='string'&&value.refused.code.length>0,'FAULT_MATRIX_REFUSED');}
 if(value.rawExit!==null){
  exact(value.rawExit,'kind,code','FAULT_MATRIX_RAW_EXIT');check(['exit','signal','unknown'].includes(value.rawExit.kind)&&integerOrNull(value.rawExit.code),'FAULT_MATRIX_RAW_EXIT');
  check(value.rawExit.kind==='exit'?value.rawExit.code!==null:value.rawExit.kind==='unknown'?value.rawExit.code===null:true,'FAULT_MATRIX_RAW_EXIT');
 }
 if(value.refused!==null)return result('REFUSED',2,value.refused.code);
 if(!value.startupValid)return result('PROCESS_UNKNOWN',3,value.startupWriteFailed?'EVIDENCE_WRITE_FAILED':'STARTUP_INVALID');
 if(value.evidencePreexists||value.invocationIdMismatch)return result('PROCESS_UNKNOWN',3,value.invocationIdMismatch?'MAIN_OBSERVATION_INVALID':'EVIDENCE_PREEXISTS');
 const knownMainFailure=value.rawExit!==null&&((value.rawExit.kind==='exit'&&value.rawExit.code!==0)||value.rawExit.kind==='signal');
 if(knownMainFailure)return result('PROCESS_FAILED',1,value.rawExit.kind==='signal'?'MAIN_SIGNAL':'MAIN_EXIT_NONZERO');
 // A null observation means no raw main exit was collected at all. It is never
 // an implicit zero: only a retained {kind:'exit',code:0} can reach success.
 if(value.rawExit===null||value.rawExit.kind==='unknown')return result('PROCESS_UNKNOWN',3,'MAIN_EXIT_UNAVAILABLE');
 check(value.rawExit.kind==='exit'&&value.rawExit.code===0,'FAULT_MATRIX_RAW_EXIT');
 if(!value.terminalEvidencePersisted)return result('PROCESS_UNKNOWN',3,'EVIDENCE_WRITE_FAILED');
 if(value.stopReturnCode!==0||value.stopErrorClass!==null)return result('PROCESS_UNKNOWN',3,'STOP_FAILED');
 if(!value.stopReceiptPersisted)return result('PROCESS_UNKNOWN',3,'STOP_RECEIPT_FAILED');
 if(value.deadlineExceeded||value.parentLost)return result('PROCESS_UNKNOWN',3,value.deadlineExceeded?'DEADLINE_EXCEEDED':'PARENT_LOST');
 if(!value.containmentComplete)return result('PROCESS_UNKNOWN',3,'CONTAINMENT_UNRESOLVED');
 if(value.timerCancelReturnCode!==0||!value.timerCancelReceiptPersisted)return result('PROCESS_UNKNOWN',3,'TIMER_CANCEL_FAILED');
 if(value.resultWriteFailed)return result('PROCESS_UNKNOWN',3,'RESULT_WRITE_FAILED');
 return result('PROCESS_SUCCESS',0,null);
}
