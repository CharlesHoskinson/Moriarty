/** Draft read-only diagnostic. One actual preflight, no launcher invocation. */
import {readFileSync,openSync,writeFileSync,fsyncSync,closeSync,constants} from 'node:fs';
import {createHash} from 'node:crypto';
const output=new URL('./diagnostic-result.json',import.meta.url);
const plan=JSON.parse(readFileSync(new URL('./public-plan.json',import.meta.url)));
const codes=new Set(JSON.parse(readFileSync(new URL('./local-error-codes.json',import.meta.url))));
const started=Date.now(),deadlineMs=started+89000;let phase='imports',done=false;
const result={schema:'moriarty.actual-public-recovery-diagnostic/1',status:'INCOMPLETE',scope:'Read-only public preflight observation; no private material, wallet, recovery writes, proof or submission',elapsedMs:0};
function retain(exitCode){if(done)return;done=true;result.phase=phase;result.elapsedMs=Date.now()-started;const raw=Buffer.from(JSON.stringify(result,null,2)+'\n');if(raw.length>65536)process.exit(3);const fd=openSync(output,constants.O_WRONLY|constants.O_CREAT|constants.O_EXCL|constants.O_NOFOLLOW,0o600);try{writeFileSync(fd,raw);fsyncSync(fd);}finally{closeSync(fd);}const d=openSync(new URL('.',import.meta.url),constants.O_RDONLY|constants.O_DIRECTORY);try{fsyncSync(d);}finally{closeSync(d);}process.exit(exitCode);}
const hardTimer=setTimeout(()=>{result.errorClass='TimeoutError';result.localCode='DIAGNOSTIC_DEADLINE';retain(2);},89000);
const clip=s=>String(s).replace(/[^\x20-\x7e]/g,' ').slice(0,2048);
const classes=new Set(['Error','TypeError','RangeError','SyntaxError','IndexerQueryError','IndexerSubscriptionError','AbortError','TimeoutError']);
function classify(e){return {errorClass:classes.has(e?.name)?e.name:'UNCLASSIFIED_ERROR',localCode:codes.has(e?.message)?e.message:['ENOENT','EACCES','ELOOP','EISDIR'].includes(e?.code)?'FS_'+e.code:'UNCLASSIFIED_MESSAGE',publicMessage:typeof e?.message==='string'?clip(e.message):typeof e==='string'?clip(e):'NO_ERROR_MESSAGE'};}
async function oneSdkQuery(){
 const request=JSON.parse(readFileSync(new URL('./sdk-query.json',import.meta.url)));const available=deadlineMs-Date.now();if(available<=0)return {status:'DEADLINE_EXHAUSTED'};
 const response=await fetch(plan.networkConfig.indexer,{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify(request),redirect:'error',signal:AbortSignal.timeout(Math.min(5000,available))});
 const reader=response.body.getReader(),chunks=[];let bytes=0,truncated=false;
 try{for(;;){const {done,value}=await reader.read();if(done)break;const left=262144-bytes;if(value.length>left){chunks.push(value.subarray(0,left));bytes+=left;truncated=true;break;}chunks.push(value);bytes+=value.length;}}finally{await reader.cancel();}
 const raw=Buffer.concat(chunks),record={httpStatus:response.status,operationName:'TX_ID_QUERY',bytesRead:bytes,responsePrefixSha256:createHash('sha256').update(raw).digest('hex'),truncated};
 if(truncated)return {...record,status:'RESPONSE_BOUND'};
 let body;try{body=JSON.parse(raw);}catch{return {...record,status:'NON_JSON_RESPONSE'};}
 return {...record,status:'OBSERVED',errors:Array.isArray(body.errors)?body.errors.slice(0,8).map(e=>({message:clip(e.message),...(Array.isArray(e.path)?{path:e.path.slice(0,16).map(x=>typeof x==='number'?x:clip(x))}:{})})):[],transactionCount:Array.isArray(body.data?.transactions)?body.data.transactions.length:null};
}
try{
 const {waitForLocalTip}=await import('../../../experiments/moriarty-midnight-financial/ledger/local-tip.mjs');
 const {validateLocalLaunchPlan}=await import('../../../experiments/moriarty-midnight-financial/ledger/launch-local.mjs');
 const {preflightLocalRecovery}=await import('../../../experiments/moriarty-midnight-financial/ledger/integrate-local.mjs');
 plan.limits.deadlineMs=deadlineMs;validateLocalLaunchPlan(plan);
 phase='existing-tip-readiness';const tip=await waitForLocalTip({node:plan.networkConfig.node,indexer:plan.networkConfig.indexer,deadlineMs,exactFinality:true});
 result.readyTip={hash:tip.hash,height:tip.height,finalizedHash:tip.finalizedHash,finalizedHeight:tip.finalizedHeight,timestampMs:tip.timestampMs};
 phase='actual-preflight';const r=await preflightLocalRecovery(plan);
 result.status='PUBLIC_STATE_VERIFIED';result.publicBinding={transactionHash:r.binding.transactionHash,contractAddress:r.binding.contractAddress,txId:r.binding.txId,finalizedHash:r.stateBlock.hash,finalizedHeight:r.stateBlock.height};
 clearTimeout(hardTimer);retain(0);
}catch(e){
 result.status='FAILED';result.failurePhase=phase;Object.assign(result,classify(e));
 // One exact SDK query reveals server schema errors without repeating preflight.
 if(phase==='actual-preflight'&&(e?.name==='IndexerQueryError'||result.localCode==='UNCLASSIFIED_MESSAGE')&&deadlineMs>Date.now()){
  phase='single-public-sdk-query';try{result.sdkQuery=await oneSdkQuery();}catch(error){result.sdkQuery={status:'FAILED',...classify(error)};}
 }
 clearTimeout(hardTimer);retain(2);
}
