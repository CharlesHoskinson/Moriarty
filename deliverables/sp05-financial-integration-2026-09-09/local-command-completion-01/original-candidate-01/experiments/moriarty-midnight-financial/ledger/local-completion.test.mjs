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
 const calls=[],compared=[],watched=[];let stopped=false,closed=false;const priorCount=0,priorDust=0n;let charges=priorDust,gross=0n;
 const network={networkId:'undeployed',node:'http://127.0.0.1:19944',indexer:'http://127.0.0.1:18088/api/v4/graphql',indexerWS:'ws://127.0.0.1:18088/api/v4/graphql/ws',proofServer:'http://127.0.0.1:16300'};
 const binding={network,payerAddress:first,sdkNetworkId:'undeployed'};
 const submit=stage=>{calls.push(stage);charges+=BigInt(receiptByStage[stage].transaction.dustFee);gross+=BigInt(receiptByStage[stage].transaction.grossByAsset[color]??0);return receiptByStage[stage].txId;};
 const sdkCalls={deployContract:async()=>({deployTxData:{public:{contractAddress:address,txId:submit('deploy')}}}),submitCallTx:async(_p,o)=>({public:{txId:submit(o.circuitId)}})};
 const options={kind:'loan',sourceTestOnly:true,walletContext:{wallet:{async stop(){closed=true;}},unshieldedKeystore:{getPublicKey:()=>publicKey},shieldedSecretKeys:{coinPublicKey:'synthetic-coin',encryptionPublicKey:'synthetic-encryption'}},networkConfig:network,roles,networkTag,deploymentSigningKey:'synthetic-signing-key-not-used',build:{receiptPath:'/synthetic/public-build.json',receiptSha256:'11'.repeat(32),sourceManifestHash:'22'.repeat(32)},privateStateConfig:{},limits:{allocationId:'composed-negative-only',reservationStatePath:'/synthetic/no-file-created',deadlineMs:Date.now()+10000,submissions:4,dustFee:2000000000000000n,grossByLogicalAsset:{USD_TEST_ASSET:20000000000n}},expectedProtocolVersion:1000000,onEvent:()=>{},now:()=>1700000000n,onStage:async s=>{compared.push(s.stage);if(bindingMutation)bindingMutation(s.stage,binding);return {status:'RECORDED',stage:s.stage,txId:s.txId};}};
 options.adapters={loadAssets:async()=>({compiledContract:{sourceOnly:true},decodeState:decodedState,zkConfigPath:'/synthetic/no-proving-assets',assertFresh(){},cleanup(){}}),loadContractsSdk:async()=>sdkCalls,loadProviderSdk:async()=>({getNetworkId:()=> 'undeployed',NodeZkConfigProvider:class{}}),loadNativeRuntime:async()=>({ledger,runtime}),prepareDeployment:async()=>({public:{contractAddress:address},driverSdk:sdkCalls}),initializeReservations:()=>{},createProviders:async()=>({publicDataProvider:{},execute:async(_label,fn)=>fn(),stop(){stopped=true;},getExecutionBinding:()=>binding,getState:()=>({reservedSubmissions:priorCount+calls.length,reservedDustFee:charges,reservedGrossByAsset:gross?{[color]:gross}:{},identifiers:calls.flatMap(s=>receiptByStage[s].transaction.identifiers)}),cleanup:async()=>{closed=true;return {walletStopped:true,pendingOperations:0,containmentComplete:true};}}),createComparator:createFinancialComparator,driver:runLocalFinancialCase,fetch:()=>{throw Error('NO_NETWORK');},observe:async o=>{
  const stage=o.circuitId,r=receiptByStage[stage];
  const row=x=>({owner:MidnightBech32m.encode('undeployed',new UnshieldedAddress(Buffer.from(x.owner,'hex'))).toString(),tokenType:x.type,value:BigInt(x.value),intentHash:x.intentHash});
  const data={tx:ledger.Transaction.deserialize('signature','proof','binding',raws[stage]),txId:r.txId,txHash:r.transaction.transactionHash,identifiers:r.transaction.identifiers,status:'SucceedEntirely',protocolVersion:r.protocolVersion,blockHash:r.blockHash,blockHeight:r.blockHeight,fees:{paidFees:r.fees.indexerReported.paid,estimatedFees:r.fees.indexerReported.estimated},unshielded:{created:r.transaction.outputs.map(row),spent:r.transaction.inputs.map(row)}};
  if(wireMutation)wireMutation(stage,data);
  const provider={watchForTxData:async id=>{watched.push(stage);assert.equal(id,r.txId);return data;},queryContractState:async()=>new stateRuntime.ContractState(),queryUnshieldedBalances:async()=>Object.entries(r.contractBalances).map(([tokenType,value])=>({tokenType,balance:BigInt(value)}))};
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

import {localFinancialExitCode,validateLocalFinancialCompletion} from './local-completion.mjs';
const originals={providers:await import('./providers.mjs'),assets:await import('./proven-assets.mjs'),prepare:await import('./prepare-deployment.mjs'),receipt:await import('./receipt.mjs')};
// Test-runner module interception only: no production adapters or source status
// are passed to the real integration entry. SDK/transport operations are inert.
async function completedFixture(t,{pending=0,stopFailure=false,observationMutation,retentionFailure=false,dustFeeCap,containmentComplete=false,loaderFailure=false,lateCleanup=false}={}){
 const f=fixture({observationMutation}),deps=f.options.adapters,directory=mkdtempSync(join(tmpdir(),'moriarty-local-exit-'));
 t.after(()=>rmSync(directory,{recursive:true,force:true}));
 if(dustFeeCap!==undefined)f.options.limits.dustFee=dustFeeCap;
 const load=deps.loadAssets;deps.loadAssets=async()=>{const loaded=await load();return {...loaded,cleanup:()=>{loaded.cleanup();if(lateCleanup)t.mock.method(Date,'now',()=>f.options.limits.deadlineMs+1);if(loaderFailure)throw Error('CONTROLLED_LOADER_CLEANUP');}};};
 const create=deps.createProviders;
 deps.createProviders=async o=>{const p=await create(o);return {...p,cleanup:async()=>{await p.cleanup();if(stopFailure)throw Error('CONTROLLED_WALLET_STOP_FAILED');return {walletStopped:true,pendingOperations:pending,containmentComplete};}};};
 t.mock.module(new URL('./providers.mjs',import.meta.url).href,{namedExports:{...originals.providers,loadFinancialSdk:deps.loadProviderSdk,initializeFinancialReservations:deps.initializeReservations,createFinancialProviders:deps.createProviders}});
 t.mock.module(new URL('./proven-assets.mjs',import.meta.url).href,{namedExports:{...originals.assets,loadProvenFinancialContract:deps.loadAssets}});
 t.mock.module(new URL('./prepare-deployment.mjs',import.meta.url).href,{namedExports:{...originals.prepare,loadFinancialContractsSdk:deps.loadContractsSdk,prepareFinancialDeployment:deps.prepareDeployment}});
 t.mock.module(new URL('./receipt.mjs',import.meta.url).href,{namedExports:{...originals.receipt,observeFinalizedStage:deps.observe}});
 t.mock.method(globalThis,'fetch',deps.fetch);
 delete f.options.adapters;delete f.options.sourceTestOnly;
 f.options.onStage=s=>{if(retentionFailure)throw Error('CONTROLLED_RETENTION_FAILURE');writeFileSync(join(directory,'stage-'+s.stage+'.json'),JSON.stringify(s));return {status:'RECORDED',stage:s.stage,txId:s.txId};};
 const entry=await import('./integrate-local.mjs?exit-test='+Math.random());
 let result,error;try{result=await entry.integrateLocalFinancialCase(f.options);}catch(e){error=e;result=e.publicIntegrationResult;}
 return {...f,result,error,directory};
}
test('controlled production driver/integration/writer reaches CLI zero selector with external containment still false',async t=>{
 const f=await completedFixture(t);assert.equal(f.error,undefined);assert.equal(f.result.status,'FINANCIAL_COMPLETE');assert.equal(f.result.driver.status,'FINANCIAL_COMPLETE');
 retainPublicIntegrationResult(f.directory,f.result);const stored=JSON.parse(readFileSync(join(f.directory,'integration-result.json')));
 assert.equal(localFinancialExitCode(stored,f.options.limits.deadlineMs),0);assert.equal(stored.cleanup.containmentComplete,false);assert.equal(stored.driver.cleanup.containmentComplete,false);
 for(const k of ['networkAcceptance','proofAcceptance','financialAcceptance'])assert.equal(stored[k],false);
 assert.equal(stored.sourceTestOnly,false);assert.deepEqual(f.calls,stages);assert.equal(stored.comparisons.length,4);
});
for(const control of [{pending:1},{stopFailure:true},{retentionFailure:true},{loaderFailure:true},{lateCleanup:true},{observationMutation:(s,o)=>{if(s==='initialize')o.receipt.transaction.outputs[0].owner=second;}}])test('failed/pending production path never receives CLI zero',async t=>{
 const f=await completedFixture(t,control);assert.ok(f.error);assert.throws(()=>localFinancialExitCode(f.result,f.options.limits.deadlineMs));assert.notEqual(f.result.status,'FINANCIAL_COMPLETE');
});
test('completed result validator rejects stale status, missing stages/IDs, unknown work and false containment claims',async t=>{
 const f=await completedFixture(t);assert.equal(f.error,undefined);
 for(const mutate of [r=>r.financialComparison.networkAcceptance=true,r=>{r.comparisons[0].proofAcceptance='false';r.financialComparison.stages=r.comparisons;},r=>Object.defineProperty(r.cleanup,'pendingOperations',{get(){throw Error('ACCESSOR_EXECUTED');}}),r=>r.status='FAILED',r=>r.status='INCOMPLETE',r=>r.sourceTestOnly=true,r=>r.setupPendingOperations=1,r=>r.driver.stages.pop(),r=>delete r.driver.stages[0].finalizedHeight,r=>r.financialComparison.stages.pop(),r=>r.driver.transactionIds[0]='bad',r=>r.cleanup.containmentComplete=true,r=>r.driver.status='INCOMPLETE',r=>r.driver.operationalState={unavailable:true},r=>r.driver.failure={phase:'observe',stage:'settle',code:'UNCLASSIFIED_DRIVER_FAILURE'}]){
  const r=structuredClone(f.result);mutate(r);assert.throws(()=>localFinancialExitCode(r,f.options.limits.deadlineMs));
 }
 assert.throws(()=>localFinancialExitCode(f.result,Date.now()),/DEADLINE/);
});

test('admitted cumulative excess fee prevents financial completion and CLI zero',async t=>{
 const total=stages.reduce((sum,s)=>sum+BigInt(receiptByStage[s].transaction.dustFee),0n);
 const f=await completedFixture(t,{dustFeeCap:total-1n});assert.equal(f.error.message,'NATIVE_FEE_CAP_EXCEEDED');assert.equal(f.result.driver.failure.code,'NATIVE_FEE_CAP_EXCEEDED');assert.throws(()=>localFinancialExitCode(f.result,f.options.limits.deadlineMs));
});
test('successful contained result retains PASS; source-only results never become command success',async t=>{
 const f=await completedFixture(t,{containmentComplete:true});assert.equal(f.result.status,'PASS');assert.equal(localFinancialExitCode(f.result,f.options.limits.deadlineMs),0);
 const r=structuredClone(f.result);r.status='SOURCE_TEST_ONLY';r.sourceTestOnly=true;assert.throws(()=>localFinancialExitCode(r,f.options.limits.deadlineMs));
});
test('failed durable result retention prevents the CLI exit selector from running',async t=>{
 const f=await completedFixture(t);writeFileSync(join(f.directory,'integration-result.json'),'original');let selected=false;
 assert.throws(()=>{retainPublicIntegrationResult(f.directory,f.result);selected=true;localFinancialExitCode(f.result,f.options.limits.deadlineMs);});assert.equal(selected,false);assert.equal(readFileSync(join(f.directory,'integration-result.json'),'utf8'),'original');
});
import fs from 'node:fs';import {syncBuiltinESMExports} from 'node:module';
test('final durable result directory fsync crossing the absolute deadline cannot yield CLI zero',async t=>{
 const f=await completedFixture(t);let clock=Date.now();f.options.limits.deadlineMs=clock+1000;const fsync=fs.fsyncSync;
 t.mock.method(Date,'now',()=>clock);t.mock.method(fs,'fsyncSync',fd=>{const r=fsync(fd);if(fs.fstatSync(fd).isDirectory())clock=f.options.limits.deadlineMs+1;return r;});syncBuiltinESMExports();t.after(()=>{t.mock.restoreAll();syncBuiltinESMExports();});
 retainPublicIntegrationResult(f.directory,f.result);assert.throws(()=>localFinancialExitCode(f.result,f.options.limits.deadlineMs),{message:'LOCAL_COMPLETION_DEADLINE'});assert.ok(readFileSync(join(f.directory,'integration-result.json')).length>0);
});
