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

// Fixture above is retained verbatim from the frozen Preview composition test;
// this consumer test connects its actual producer outputs to durable writers.
import {retainPreviewStage,retainPreviewIntegrationResult} from './preview-launch.mjs';
function output(t){const d=mkdtempSync(join(tmpdir(),'moriarty-preview-writer-composed-'));t.after(()=>rmSync(d,{recursive:true,force:true}));return d;}
test('actual Preview integration producer durably retains all four native/comparison stages and66hex IDs',async t=>{
 const directory=output(t),f=fixture();f.options.onStage=s=>retainPreviewStage(directory,s);
 const produced=await api.integratePreviewFinancialCase(f.options);assert.equal(produced.status,'SOURCE_TEST_ONLY');retainPreviewIntegrationResult(directory,produced);
 const stored=JSON.parse(readFileSync(join(directory,'integration-result.json')));assert.deepEqual(stored,JSON.parse(JSON.stringify(produced)));assert.deepEqual(f.calls,stages);
 for(const stage of stages){const row=JSON.parse(readFileSync(join(directory,'stage-'+stage+'.json')));assert.equal(row.txId.length,66);assert.deepEqual(row,produced.comparisons.find(s=>s.stage===stage));}
 assert.equal(stored.driver.stages.length,4);assert.equal(stored.financialComparison.status,'PASS');assert.equal(stored.networkAcceptance,false);assert.equal(stored.sourceTestOnly,true);
 const publicText=readFileSync(join(directory,'integration-result.json'),'utf8');assert.ok(!publicText.includes('synthetic-signing-key-not-used')&&!publicText.includes('firstSecret')&&!publicText.includes('privateStateConfig'));
});
test('actual Preview preflight failure is retainable without relabeling or inventing absent fields',async t=>{
 const directory=output(t),f=fixture();f.options.networkTag='ff'.repeat(32);let produced;await assert.rejects(api.integratePreviewFinancialCase(f.options),e=>{produced=e.publicIntegrationResult;return true;});
 retainPreviewIntegrationResult(directory,produced);const stored=JSON.parse(readFileSync(join(directory,'integration-result.json')));assert.equal(stored.status,'FAILED');assert.equal(stored.failureCode,'PREVIEW_GENESIS_MISMATCH');assert.equal(stored.driver,undefined);assert.deepEqual(f.calls,[]);
});
test('actual Preview comparison failure retains prior stage and driver IDs without private error message',async t=>{
 const directory=output(t),f=fixture({observationMutation:(stage,o)=>{if(stage==='initialize')o.receipt.fees.nativeDebit.amount='1';}});f.options.onStage=s=>retainPreviewStage(directory,s);let produced;
 await assert.rejects(api.integratePreviewFinancialCase(f.options),e=>{produced=e.publicIntegrationResult;return true;});retainPreviewIntegrationResult(directory,produced);const stored=JSON.parse(readFileSync(join(directory,'integration-result.json')));assert.equal(stored.status,'FAILED');assert.deepEqual(f.calls,['deploy','initialize']);assert.equal(stored.driver.transactionIds.length,2);assert.equal(stored.comparisons.length,1);assert.equal(stored.driver.failure.code,'UNCLASSIFIED_DRIVER_FAILURE');
});
test('durable stage collision through actual integration stops before another SDK call',async t=>{
 const directory=output(t),f=fixture();writeFileSync(join(directory,'stage-deploy.json'),'retained-original');f.options.onStage=s=>retainPreviewStage(directory,s);let produced;
 await assert.rejects(api.integratePreviewFinancialCase(f.options),e=>{produced=e.publicIntegrationResult;return true;});retainPreviewIntegrationResult(directory,produced);assert.deepEqual(f.calls,['deploy']);assert.equal(readFileSync(join(directory,'stage-deploy.json'),'utf8'),'retained-original');assert.equal(f.state().closed,true);
});
