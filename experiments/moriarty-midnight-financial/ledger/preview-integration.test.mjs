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
const api=await import('./integrate-preview.mjs').catch(()=>({}));
import {runPreviewFinancialCase} from './run-local.mjs';
import {observeFinalizedStage} from './receipt.mjs';
import {createFinancialComparator} from './financial-comparison.mjs';

const ledger=await import(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs')).href);
const stateRuntime=await import(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/midnight-js-protocol/dist/compact-runtime.mjs')).href);
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
 const network={networkId:'preview',node:'https://rpc.preview.midnight.network',indexer:'https://indexer.preview.midnight.network/api/v4/graphql',indexerWS:'wss://indexer.preview.midnight.network/api/v4/graphql/ws',proofServer:'http://127.0.0.1:16300'};
 const binding={network,payerAddress:first,sdkNetworkId:'preview'};
 const submit=stage=>{calls.push(stage);charges+=BigInt(receiptByStage[stage].transaction.dustFee);gross+=BigInt(receiptByStage[stage].transaction.grossByAsset[color]??0);return receiptByStage[stage].txId;};
 const sdkCalls={deployContract:async()=>({deployTxData:{public:{contractAddress:address,txId:submit('deploy')}}}),submitCallTx:async(_p,o)=>({public:{txId:submit(o.circuitId)}})};
 const options={kind:'loan',sourceTestOnly:true,walletContext:{wallet:{async stop(){closed=true;}},unshieldedKeystore:{getPublicKey:()=>publicKey,getBech32Address:()=>MidnightBech32m.encode('preview',new UnshieldedAddress(Buffer.from(first,'hex')))},shieldedSecretKeys:{coinPublicKey:'synthetic-coin',encryptionPublicKey:'synthetic-encryption'}},networkConfig:network,roles,networkTag,deploymentSigningKey:'synthetic-signing-key-not-used',build:{receiptPath:'/synthetic/public-build.json',receiptSha256:'11'.repeat(32),sourceManifestHash:'22'.repeat(32)},privateStateConfig:{},limits:{allocationId:'composed-negative-only',reservationStatePath:'/synthetic/no-file-created',deadlineMs:Date.now()+10000,submissions:4,dustFee:2000000000000000n,grossByLogicalAsset:{USD_TEST_ASSET:20000000000n}},expectedProtocolVersion:1000000,onEvent:()=>{},now:()=>1700000000n,onStage:async s=>{compared.push(s.stage);if(bindingMutation)bindingMutation(s.stage,binding);return {status:'RECORDED',stage:s.stage,txId:s.txId};}};
 options.adapters={loadPublicIdentity:()=>({address:MidnightBech32m.encode('preview',new UnshieldedAddress(Buffer.from(first,'hex'))).toString()}),loadAssets:async()=>({compiledContract:{sourceOnly:true},decodeState:decodedState,zkConfigPath:'/synthetic/no-proving-assets',assertFresh(){},cleanup(){}}),loadContractsSdk:async()=>sdkCalls,loadProviderSdk:async()=>({getNetworkId:()=> 'preview',NodeZkConfigProvider:class{}}),loadNativeRuntime:async()=>({ledger,runtime}),prepareDeployment:async()=>({public:{contractAddress:address},driverSdk:sdkCalls}),initializeReservations:()=>{},createProviders:async()=>({publicDataProvider:{},execute:async(_label,fn)=>fn(),stop(){stopped=true;},getExecutionBinding:()=>binding,getState:()=>({reservedSubmissions:priorCount+calls.length,reservedDustFee:charges,reservedGrossByAsset:gross?{[color]:gross}:{},identifiers:calls.map(s=>receiptByStage[s].txId)}),cleanup:async()=>{closed=true;return {walletStopped:true,pendingOperations:0,containmentComplete:true};}}),createComparator:createFinancialComparator,driver:runPreviewFinancialCase,fetch:async(_url,o)=>{const r=JSON.parse(o.body);assert.equal(r.method,'chain_getBlockHash');assert.deepEqual(r.params,[0]);return Response.json({jsonrpc:'2.0',id:r.id,result:'0x'+networkTag});},observe:async o=>{
  const stage=o.circuitId,r=receiptByStage[stage];
  const row=x=>({owner:MidnightBech32m.encode('preview',new UnshieldedAddress(Buffer.from(x.owner,'hex'))).toString(),tokenType:x.type,value:BigInt(x.value),intentHash:x.intentHash});
  const data={tx:ledger.Transaction.deserialize('signature','proof','binding',raws[stage]),txId:r.txId,txHash:r.transaction.transactionHash,identifiers:r.transaction.identifiers,status:'SucceedEntirely',protocolVersion:r.protocolVersion,blockHash:r.blockHash,blockHeight:r.blockHeight,fees:{paidFees:r.fees.indexerReported.paid,estimatedFees:r.fees.indexerReported.estimated},unshielded:{created:r.transaction.outputs.map(row),spent:r.transaction.inputs.map(row)}};
  if(wireMutation)wireMutation(stage,data);
  const provider={watchForTxData:async id=>{watched.push(stage);assert.equal(id,r.txId);return data;},queryContractState:async()=>new stateRuntime.ContractState(),queryUnshieldedBalances:async()=>Object.entries(r.contractBalances).map(([tokenType,value])=>({tokenType,balance:BigInt(value)}))};
  const rpc=async(method,args)=>method==='chain_getHeader'?{number:'0x'+r.blockHeight.toString(16)}:method==='chain_getBlockHash'?'0x'+r.blockHash:'0x'+r.blockHash;
  const observed=await observeFinalizedStage({...o,provider,rpc,decodeState:()=>o.decodeState(stage)});if(observationMutation)observationMutation(stage,observed);return observed;
 }};
 return {options,calls,compared,watched,state:()=>({stopped,closed,charges,gross,priorCount,priorDust})};
}

async function run(f){assert.equal(typeof api.integratePreviewFinancialCase,'function');return api.integratePreviewFinancialCase(f.options);}
test('Preview composition connects actual stage driver, native observer and independent comparator',async()=>{
 const f=fixture(),result=await run(f);assert.equal(result.schema,'moriarty.preview-financial-integration/1');assert.equal(result.status,'SOURCE_TEST_ONLY');assert.equal(result.financialComparison.status,'PASS');assert.deepEqual(f.calls,stages);assert.deepEqual(f.compared,stages);assert.deepEqual(f.watched,stages);assert.equal(result.financialAcceptance,false);assert.equal(f.state().closed,true);
});
for(const [name,change,code] of [
 ['wrong target',f=>f.options.networkConfig.node='https://evil.test','PREVIEW_NETWORK_CONFIG'],
 ['wrong genesis',f=>f.options.networkTag='ff'.repeat(32),'PREVIEW_GENESIS_MISMATCH'],
 ['wrong original wallet',f=>f.options.adapters.loadPublicIdentity=()=>({address:MidnightBech32m.encode('preview',new UnshieldedAddress(Buffer.from(second,'hex'))).toString()}),'PREVIEW_WALLET_IDENTITY'],
 ['injected production dependency',f=>f.options.sourceTestOnly=false,'INTEGRATION_ADAPTERS_REQUIRE_SOURCE_TEST'],
 ['retention failed',f=>f.options.onStage=()=>({status:'FAILED'}),'STAGE_RETENTION_REQUIRED']
])test('Preview composition stops '+name+' and owns cleanup',async()=>{
 const f=fixture();change(f);await assert.rejects(run(f),e=>{assert.equal(e.message,code);if(code.startsWith('PREVIEW_'))assert.equal(e.publicIntegrationResult.failureCode,code);assert.equal(e.publicIntegrationResult.financialAcceptance,false);return true;});assert.ok(f.calls.length<=1);assert.equal(f.state().closed,true);
});
test('actual Preview owner decoding rejects a wrong initialize recipient before next call',async()=>{
 const f=fixture({wireMutation:(s,d)=>{if(s==='initialize')d.unshielded.created[0].owner=MidnightBech32m.encode('preview',new UnshieldedAddress(Buffer.from(second,'hex'))).toString();}});
 await assert.rejects(run(f),{message:'INDEXED_OUTPUTS_MISMATCH'});assert.deepEqual(f.calls,['deploy','initialize']);assert.equal(f.state().closed,true);
});
test('actual independent comparator rejects understated observed native fee',async()=>{
 const f=fixture({observationMutation:(s,o)=>{if(s==='deploy')o.receipt.fees.nativeDebit.amount='1';}});
 await assert.rejects(run(f),{message:'NATIVE_FEE_MISMATCH'});assert.deepEqual(f.calls,['deploy']);assert.equal(f.state().closed,true);
});

test('Preview composition uses real exclusive reservations and guarded provider lifecycle without invoking wallet transaction APIs',async()=>{
 const {initializeFinancialReservations,createFinancialProviders}=await import('./providers.mjs');
 const dir=mkdtempSync(join(tmpdir(),'moriarty-preview-provider-')),f=fixture();
 try{
  f.options.limits.reservationStatePath=join(dir,'reservation.json');
  f.options.privateStateConfig={midnightDbName:join(dir,'inert-store'),privateStateStoreName:'sp05-loan',privateStoragePasswordProvider:()=> 'SYNTHETIC-NOT-USED'};
  f.options.walletContext.dustSecretKey='synthetic';
  for(const method of ['balanceUnboundTransaction','signRecipe','finalizeRecipe','submitTransaction'])f.options.walletContext.wallet[method]=()=>{throw Error('WALLET_API_FORBIDDEN_IN_FIXTURE');};
  const prior=f.options.adapters.loadProviderSdk;
  f.options.adapters.loadProviderSdk=async()=>({...await prior(),levelPrivateStateProvider:()=>({}),indexerPublicDataProvider:()=>({}),httpClientProofProvider:()=>({})});
  f.options.adapters.initializeReservations=initializeFinancialReservations;
  f.options.adapters.createProviders=createFinancialProviders;
  const r=await run(f);assert.equal(r.status,'SOURCE_TEST_ONLY');assert.equal(r.driver.status,'FINANCIAL_COMPLETE');assert.equal(r.driver.operationalState.reservedSubmissions,0);assert.equal(r.financialComparison.status,'PASS');assert.equal(r.cleanup.containmentComplete,false);
  assert.ok(readFileSync(f.options.limits.reservationStatePath).length>0);
 }finally{rmSync(dir,{recursive:true,force:true});}
});
test('Preview setup deadline rejects a late public response before build/provider construction',async t=>{
 const f=fixture();let clock=Date.now();f.options.limits.deadlineMs=clock+1000;t.mock.method(Date,'now',()=>clock);
 f.options.adapters.fetch=async(_u,o)=>{await Promise.resolve();clock+=1001;const r=JSON.parse(o.body);return Response.json({jsonrpc:'2.0',id:r.id,result:'0x'+networkTag});};
 await assert.rejects(run(f),{message:'RPC_DEADLINE'});assert.deepEqual(f.calls,[]);assert.equal(f.state().closed,true);
});
test('Preview integration cannot report completion after loader cleanup exceeds deadline',async t=>{
 const f=fixture();let clock=Date.now();f.options.limits.deadlineMs=clock+1000;t.mock.method(Date,'now',()=>clock);
 const original=f.options.adapters.loadAssets;f.options.adapters.loadAssets=async()=>({...await original(),cleanup(){clock+=1001;}});
 await assert.rejects(run(f),{message:'INTEGRATION_DEADLINE'});
});

test('Preview actual observer/comparator composition rejects cumulative fees above its admitted cap',async()=>{
 const total=stages.reduce((sum,s)=>sum+BigInt(receiptByStage[s].transaction.dustFee),0n);
 const f=fixture();f.options.limits.dustFee=total-1n;
 await assert.rejects(run(f),e=>{assert.equal(e.message,'NATIVE_FEE_CAP_EXCEEDED');assert.equal(e.publicIntegrationResult.failureCode,'NATIVE_FEE_CAP_EXCEEDED');assert.equal(e.publicIntegrationResult.driver.failure.code,'NATIVE_FEE_CAP_EXCEEDED');return true;});
 assert.deepEqual(f.compared,stages.slice(0,3));assert.equal(f.state().closed,true);
 const exact=fixture();exact.options.limits.dustFee=total;assert.equal((await run(exact)).financialComparison.status,'PASS');
});
