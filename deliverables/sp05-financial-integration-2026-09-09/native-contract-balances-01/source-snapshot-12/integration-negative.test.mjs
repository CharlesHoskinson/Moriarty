/** Composed SOURCE-ONLY regressions. Transport/SDK submission and reservation
 * counters are controlled fixtures, never live calls or real allocations.
 * Native transaction bytes and financial public states come from retained loan
 * evidence. Role capability commitments alone are replaced with commitments to
 * synthetic test secrets; no original private roles or state are accessed.
 * ContractState containers have controlled empty balances; decoded financial
 * fields are reconstructed from retained summaries, not decoded from those containers.
 */
import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync,mkdtempSync,rmSync,writeFileSync,openSync,fsyncSync,closeSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {PINNED_NM} from './providers.mjs';
import {integrateLocalFinancialCase} from './integrate-local.mjs';
import {runLocalFinancialCase} from './run-local.mjs';
import {observeFinalizedStage} from './receipt.mjs';
import {createFinancialComparator} from './financial-comparison.mjs';
import {retainPublicIntegrationResult} from './launch-local.mjs';
const ledger=await import(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs')).href);
const runtime=await import(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/compact-runtime/dist/index.js')).href);
const {MidnightBech32m,UnshieldedAddress}=await import(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/wallet-sdk-address-format/dist/index.js')).href);
const D=new URL('../../../deliverables/sp05-financial-integration-2026-09-09/',import.meta.url);
const read=path=>readFileSync(new URL(path,D));
const retained=JSON.parse(read('local-continuation-02/run-public/integration-result.json'));
const stages=['deploy','initialize','accrue','settle'];
const summaries=Object.fromEntries(stages.map(s=>[s,JSON.parse(read(`local-continuation-02/run-public/stage-${s}.json`))]));
const receiptByStage=Object.fromEntries(retained.driver.stages.map(r=>[r.circuitId,r]));
const raws={deploy:read('local-execution-04/run-public/public-transactions/'+receiptByStage.deploy.transaction.transactionHash+'.bin'),initialize:read('local-recovery-03/run-public/public-transactions/'+receiptByStage.initialize.transaction.transactionHash+'.bin'),...Object.fromEntries(['accrue','settle'].map(s=>[s,read('local-continuation-02/run-public/public-transactions/'+receiptByStage[s].transaction.transactionHash+'.bin')]))};
const bytes=s=>Uint8Array.from(Buffer.from(s,'hex'));
const address=receiptByStage.deploy.contractAddress,first=summaries.deploy.publicState.borrowerAddress.bytes,second=summaries.deploy.publicState.lenderAddress.bytes,networkTag=summaries.deploy.publicState.networkTag;
const color=summaries.initialize.publicState.usdColor;
const publicKey=[...ledger.Transaction.deserialize('signature','proof','binding',raws.settle).intents.values()].flatMap(i=>[i.guaranteedUnshieldedOffer,i.fallibleUnshieldedOffer]).filter(Boolean).flatMap(o=>o.inputs)[0].owner;
const roles={firstAddress:first,secondAddress:second,firstSecret:new Uint8Array(32).fill(17),secondSecret:new Uint8Array(32).fill(18)};
function decodedState(stage){
 const p=summaries[stage].publicState;
 const scalars=x=>typeof x==='string'?BigInt(x):Object.fromEntries(Object.entries(x).map(([k,v])=>[k,scalars(v)]));
 const s={...p,programDigest:bytes(p.programDigest),networkTag:bytes(p.networkTag),initialized:p.initialized,remaining:BigInt(p.remaining),revision:BigInt(p.revision),kernelState:scalars(p.kernelState),usdDomain:bytes(p.usdDomain),usdColor:bytes(p.usdColor),lastAccrue:scalars(p.lastAccrue),lastSettle:scalars(p.lastSettle)};
 for(const [role,secret] of [['borrower',roles.firstSecret],['lender',roles.secondSecret]]){s[role+'Address']={bytes:bytes(p[role+'Address'].bytes)};s[role+'Capability']=runtime.persistentHash(new runtime.CompactTypeVector(4,new runtime.CompactTypeBytes(32)),[Uint8Array.from(Buffer.from(`moriarty:sp05:loan:${role}`.padEnd(32,'\0'))),bytes(networkTag),bytes(p.programDigest),secret]);}
 return s;
}
function fixture({wireMutation,observationMutation,bindingMutation}={}){
 const calls=[],compared=[],watched=[];let stopped=false,closed=false;const priorCount=6,priorDust=1800000000000006n;let charges=priorDust,gross=0n;
 const network={networkId:'undeployed',node:'http://127.0.0.1:19944',indexer:'http://127.0.0.1:18088/api/v4/graphql',indexerWS:'ws://127.0.0.1:18088/api/v4/graphql/ws',proofServer:'http://127.0.0.1:16300'};
 const binding={network,payerAddress:first,sdkNetworkId:'undeployed'};
 const submit=stage=>{calls.push(stage);charges+=BigInt(receiptByStage[stage].transaction.dustFee);gross+=BigInt(receiptByStage[stage].transaction.grossByAsset[color]??0);return receiptByStage[stage].txId;};
 const sdkCalls={deployContract:async()=>({deployTxData:{public:{contractAddress:address,txId:submit('deploy')}}}),submitCallTx:async(_p,o)=>({public:{txId:submit(o.circuitId)}})};
 const options={kind:'loan',sourceTestOnly:true,walletContext:{wallet:{async stop(){closed=true;}},unshieldedKeystore:{getPublicKey:()=>publicKey},shieldedSecretKeys:{coinPublicKey:'synthetic-coin',encryptionPublicKey:'synthetic-encryption'}},networkConfig:network,roles,networkTag,deploymentSigningKey:'synthetic-signing-key-not-used',build:{receiptPath:'/synthetic/public-build.json',receiptSha256:'11'.repeat(32),sourceManifestHash:'22'.repeat(32)},privateStateConfig:{},limits:{allocationId:'composed-negative-only',reservationStatePath:'/synthetic/no-file-created',deadlineMs:Date.now()+10000,submissions:4,dustFee:2000000000000000n,grossByLogicalAsset:{USD_TEST_ASSET:20000000000n}},expectedProtocolVersion:1000000,onEvent:()=>{},now:()=>1700000000n,onStage:async s=>{compared.push(s.stage);if(bindingMutation)bindingMutation(s.stage,binding);return {status:'RECORDED',stage:s.stage,txId:s.txId};}};
 options.adapters={loadAssets:async()=>({compiledContract:{sourceOnly:true},decodeState:decodedState,zkConfigPath:'/synthetic/no-proving-assets',assertFresh(){},cleanup(){}}),loadContractsSdk:async()=>sdkCalls,loadProviderSdk:async()=>({getNetworkId:()=> 'undeployed',NodeZkConfigProvider:class{}}),loadNativeRuntime:async()=>({ledger,runtime}),prepareDeployment:async()=>({public:{contractAddress:address},driverSdk:sdkCalls}),initializeReservations:()=>{},createProviders:async()=>({publicDataProvider:{},execute:async(_label,fn)=>fn(),stop(){stopped=true;},getExecutionBinding:()=>binding,getState:()=>({reservedSubmissions:priorCount+calls.length,reservedDustFee:charges,reservedGrossByAsset:gross?{[color]:gross}:{},identifiers:calls.map(s=>receiptByStage[s].txId)}),cleanup:async()=>{closed=true;return {walletStopped:true,pendingOperations:0,containmentComplete:true};}}),createComparator:createFinancialComparator,driver:runLocalFinancialCase,fetch:()=>{throw Error('NO_NETWORK');},observe:async o=>{
  const stage=o.circuitId,r=receiptByStage[stage];
  const row=x=>({owner:MidnightBech32m.encode('undeployed',new UnshieldedAddress(Buffer.from(x.owner,'hex'))).toString(),tokenType:x.type,value:BigInt(x.value),intentHash:x.intentHash});
  const data={tx:ledger.Transaction.deserialize('signature','proof','binding',raws[stage]),txId:r.txId,txHash:r.transaction.transactionHash,identifiers:r.transaction.identifiers,status:'SucceedEntirely',protocolVersion:r.protocolVersion,blockHash:r.blockHash,blockHeight:r.blockHeight,fees:{paidFees:r.fees.indexerReported.paid,estimatedFees:r.fees.indexerReported.estimated},unshielded:{created:r.transaction.outputs.map(row),spent:r.transaction.inputs.map(row)}};
  if(wireMutation)wireMutation(stage,data);
  const provider={watchForTxData:async id=>{watched.push(stage);assert.equal(id,r.txId);return data;},queryContractState:async()=>new ledger.ContractState(),queryUnshieldedBalances:async()=>Object.entries(r.contractBalances).map(([tokenType,value])=>({tokenType,balance:BigInt(value)}))};
  const rpc=async(method,args)=>method==='chain_getHeader'?{number:'0x'+r.blockHeight.toString(16)}:method==='chain_getBlockHash'?'0x'+r.blockHash:'0x'+r.blockHash;
  const observed=await observeFinalizedStage({...o,provider,rpc,decodeState:()=>o.decodeState(stage)});if(observationMutation)observationMutation(stage,observed);return observed;
 }};
 return {options,calls,compared,watched,state:()=>({stopped,closed,charges,gross,priorCount,priorDust})};
}
async function runAndRetain(f){
 const dir=mkdtempSync(join(tmpdir(),'moriarty-composed-public-'));let result,error;
 try{
  // Real durable stage writes are confined to synthetic public scratch.
  const record=f.options.onStage;f.options.onStage=async s=>{const file=join(dir,'stage-'+s.stage+'.json');writeFileSync(file,JSON.stringify(s));const fd=openSync(file,'r');try{fsyncSync(fd);}finally{closeSync(fd);}return record(s);};
  try{result=await integrateLocalFinancialCase(f.options);}catch(e){error=e;result=e.publicIntegrationResult;}
  assert.throws(()=>retainPublicIntegrationResult(dir,result),/LAUNCH_PUBLIC_RESULT/,'production retention must reject source-only evidence');
  assert.ok(result, error?.message);writeFileSync(join(dir,'integration-result.json'),JSON.stringify(result));const fd=openSync(join(dir,'integration-result.json'),'r');try{fsyncSync(fd);}finally{closeSync(fd);}const parentFd=openSync(dir,'r');try{fsyncSync(parentFd);}finally{closeSync(parentFd);}const retainedResult=JSON.parse(readFileSync(join(dir,'integration-result.json')));
  for(const stage of f.compared)assert.equal(JSON.parse(readFileSync(join(dir,'stage-'+stage+'.json'))).status,'PASS');
  return {result:retainedResult,error};
 }finally{rmSync(dir,{recursive:true,force:true});}
}
test('control composes actual driver, observer/native decoder and comparator with source-only durable retention',async()=>{const f=fixture(),{result,error}=await runAndRetain(f);assert.equal(error?.message,undefined);assert.equal(result.status,'SOURCE_TEST_ONLY');assert.equal(result.financialComparison.status,'PASS');assert.deepEqual(f.calls,stages);assert.deepEqual(f.watched,stages);assert.deepEqual(f.compared,stages);assert.equal(result.financialAcceptance,false);});
for(const c of [
 {name:'wrong payer binding after durable deploy',options:{bindingMutation:(s,b)=>{if(s==='deploy')b.payerAddress=second;}},code:'LOCAL_EXECUTION_BINDING_MISMATCH',calls:['deploy'],compared:['deploy']},
 {name:'wrong indexed recipient in initialize',options:{wireMutation:(s,d)=>{if(s==='initialize')d.unshielded.created[0].owner=MidnightBech32m.encode('undeployed',new UnshieldedAddress(Buffer.from(second,'hex'))).toString();}},code:'INDEXED_OUTPUTS_MISMATCH',calls:['deploy','initialize'],compared:['deploy']},
 {name:'deleted wire fee fields',options:{wireMutation:(s,d)=>{if(s==='deploy')delete d.fees;}},code:'INVALID_PAID_FEES',calls:['deploy'],compared:[]},
 {name:'deleted observed fee fields',options:{observationMutation:(s,o)=>{if(s==='deploy')delete o.receipt.fees;}},code:'FIELDS_RECEIPT',calls:['deploy'],compared:[]},
 {name:'understated native debit after actual observer',options:{observationMutation:(s,o)=>{if(s==='deploy')o.receipt.fees.nativeDebit.amount='1';}},code:'NATIVE_FEE_MISMATCH',calls:['deploy'],compared:[]},
 {name:'wrong native payer in settle observation',options:{observationMutation:(s,o)=>{if(s==='settle')o.receipt.transaction.inputs[0].owner=second;}},code:'UNAUTHORIZED_PAYER',calls:stages,compared:['deploy','initialize','accrue']}
])test(`composed negative: ${c.name} stops and preserves charged public failure evidence`,async()=>{
 const f=fixture(c.options),{result,error}=await runAndRetain(f);assert.ok(error);assert.equal(error.message,c.code);assert.equal(result.status,'FAILED');assert.deepEqual(f.calls,c.calls);assert.deepEqual(f.compared,c.compared);assert.equal(result.driver.transactionIds.length,c.calls.length);assert.equal(result.driver.failure.phase,c.code==='LOCAL_EXECUTION_BINDING_MISMATCH'?'call':['INDEXED_OUTPUTS_MISMATCH','INVALID_PAID_FEES'].includes(c.code)?'observe':'compare');assert.equal(result.driver.operationalState.reservedSubmissions,6+c.calls.length);const expectedCharge=1800000000000006n+c.calls.reduce((sum,stage)=>sum+BigInt(receiptByStage[stage].transaction.dustFee),0n);assert.equal(result.driver.operationalState.reservedDustFee,String(expectedCharge));assert.ok(f.state().charges>f.state().priorDust);assert.deepEqual(result.driver.operationalState.reservedGrossByAsset,c.calls.includes('settle')?{[color]:'20000000000'}:{});assert.equal(f.state().stopped,true);assert.equal(f.state().closed,true);assert.equal(result.financialAcceptance,false);assert.ok(result.driver.failure&&typeof result.failureCode==='string');assert.equal(result.sourceTestOnly,true);
});
