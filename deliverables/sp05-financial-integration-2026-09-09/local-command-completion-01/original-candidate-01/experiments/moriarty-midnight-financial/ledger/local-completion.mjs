/** Fresh local four-stage completion only. No persistence or outer containment claim. */
import {isDeepStrictEqual} from 'node:util';
const check=(ok,code)=>{if(!ok)throw Error(code);};
const decimal=x=>typeof x==='string'&&/^(0|[1-9][0-9]*)$/.test(x);
const exact=(o,keys)=>check(o&&Object.keys(o).sort().join(',')===keys.split(',').sort().join(','),'LOCAL_COMPLETION_FIELDS');
function data(value){
 let count=0;const visit=v=>{check(++count<=20000,'LOCAL_COMPLETION_BOUND');if(v===null||v===undefined||['string','boolean','number'].includes(typeof v))return;check(v&&typeof v==='object'&&(Array.isArray(v)||Object.getPrototypeOf(v)===Object.prototype),'LOCAL_COMPLETION_DATA');for(const [k,d] of Object.entries(Object.getOwnPropertyDescriptors(v))){check(Object.hasOwn(d,'value'),'LOCAL_COMPLETION_ACCESSOR');visit(d.value);}check(Object.getOwnPropertySymbols(v).length===0,'LOCAL_COMPLETION_DATA');};visit(value);return value;
}
function completed(r){
 check(['PASS','FINANCIAL_COMPLETE'].includes(r.status)&&r.sourceTestOnly===false&&!Object.hasOwn(r,'failureCode')&&r.phase==='driver'&&r.setupPendingOperations===0,'LOCAL_COMPLETION_REQUIRED');
 check(r.cleanup.containmentComplete===(r.status==='PASS'),'LOCAL_COMPLETION_CONTAINMENT');
 const d=r.driver,f=r.financialComparison,order=r.kind==='loan'?['deploy','initialize','accrue','settle']:['deploy','initialize','swap','close'];
 check(d&&f&&d.status===r.status&&!Object.hasOwn(d,'failure')&&f.status==='PASS'&&f.kind===r.kind&&f.contractAddress===d.contractAddress,'LOCAL_COMPLETION_REQUIRED');
 for(const c of [r.cleanup,d.cleanup])check(c.walletStopped===true&&c.pendingOperations===0,'LOCAL_COMPLETION_REQUIRED');
 check(isDeepStrictEqual(r.cleanup,d.cleanup)&&Array.isArray(d.stages)&&d.stages.length===4&&Array.isArray(r.comparisons)&&r.comparisons.length===4&&isDeepStrictEqual(f.stages,r.comparisons),'LOCAL_COMPLETION_REQUIRED');
 const ids=new Set();
 for(const [i,stage] of order.entries()){
  const receipt=d.stages[i],comparison=r.comparisons[i];
  exact(receipt,'schema,txId,contractAddress,circuitId,transaction,blockHash,blockHeight,finalizedHead,finalizedHeight,protocolVersion,indexerIdentifiers,fees,contractBalances,acceptance');
  exact(receipt.transaction,'schema,rawSha256,transactionHash,identifiers,inputs,outputs,actions,grossByAsset,dustFee,proofVerified,ledgerAccepted');
  check(receipt.schema==='moriarty.finalized-financial-stage/1'&&receipt.acceptance==='uncertified-I2-observation'&&Number.isSafeInteger(receipt.blockHeight)&&Number.isSafeInteger(receipt.finalizedHeight)&&receipt.finalizedHeight>=receipt.blockHeight&&/^0x[0-9a-f]{64}$/.test(receipt.finalizedHead)&&Number.isSafeInteger(receipt.protocolVersion)&&receipt.transaction.schema==='moriarty.native-financial-transaction/1','LOCAL_COMPLETION_REQUIRED');
  check(receipt.circuitId===stage&&receipt.contractAddress===d.contractAddress&&comparison.status==='PASS'&&comparison.kind===r.kind&&comparison.stage===stage&&comparison.txId===receipt.txId&&comparison.contractAddress===d.contractAddress&&comparison.blockHash===receipt.blockHash&&comparison.blockHeight===receipt.blockHeight,'LOCAL_COMPLETION_REQUIRED');
  check(receipt.transaction?.identifiers?.includes(receipt.txId)&&receipt.transaction.proofVerified===false&&receipt.transaction.ledgerAccepted===false,'LOCAL_COMPLETION_REQUIRED');
  for(const id of receipt.transaction.identifiers){check(!ids.has(id),'LOCAL_COMPLETION_REQUIRED');ids.add(id);}
 }
 check(Array.isArray(d.transactionIds)&&d.transactionIds.length===ids.size&&new Set(d.transactionIds).size===ids.size&&d.transactionIds.every(id=>ids.has(id)),'LOCAL_COMPLETION_REQUIRED');
 check(d.operationalState?.reservedSubmissions===4&&decimal(d.operationalState.reservedDustFee)&&d.operationalState.unavailable===undefined,'LOCAL_COMPLETION_REQUIRED');
 return r;
}
export function validateLocalFinancialCompletion(result){
 const r=data(result);check(r?.schema==='moriarty.local-financial-integration/1'&&r.driver?.schema==='moriarty.local-financial-run/1'&&['loan','swap'].includes(r.kind)&&r.adverse===undefined,'LOCAL_COMPLETION_REQUIRED');
 for(const flag of ['networkAcceptance','proofAcceptance','financialAcceptance'])check(r[flag]===false,'LOCAL_COMPLETION_ACCEPTANCE');
 for(const entry of [r.financialComparison,...(r.comparisons??[])])check(entry?.networkAcceptance===false&&entry?.proofAcceptance===false,'LOCAL_COMPLETION_ACCEPTANCE');
 const out=completed(r);const hex=x=>typeof x==='string'&&/^[a-f0-9]{64}$/.test(x),id=x=>typeof x==='string'&&/^(?:[a-f0-9]{64}|[a-f0-9]{66})$/.test(x);
 check(hex(r.driver.contractAddress)&&r.driver.kind===r.kind&&hex(r.build?.receiptSha256)&&hex(r.build?.sourceManifestHash),'LOCAL_COMPLETION_BINDING');
 let fees=0n;
 for(const [i,receipt] of r.driver.stages.entries()){
  check(hex(receipt.blockHash)&&hex(receipt.transaction.rawSha256)&&receipt.transaction.rawSha256===receipt.transaction.transactionHash&&receipt.transaction.identifiers.every(id)&&receipt.indexerIdentifiers.every(id)&&id(receipt.txId),'LOCAL_COMPLETION_BINDING');
  check(decimal(receipt.transaction.dustFee)&&r.comparisons[i].nativeFee?.asset==='DUST'&&r.comparisons[i].nativeFee?.unit==='SPECK'&&r.comparisons[i].nativeFee.amount===receipt.transaction.dustFee,'LOCAL_COMPLETION_FEE');fees+=BigInt(receipt.transaction.dustFee);
 }
 check(fees.toString()===r.driver.operationalState.reservedDustFee,'LOCAL_COMPLETION_FEE');return out;
}
export function localFinancialExitCode(result,deadlineMs){check(Number.isSafeInteger(deadlineMs)&&Date.now()<deadlineMs,'LOCAL_COMPLETION_DEADLINE');validateLocalFinancialCompletion(result);check(Date.now()<deadlineMs,'LOCAL_COMPLETION_DEADLINE');return 0;}
