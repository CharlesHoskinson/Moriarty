/** Composed SOURCE-ONLY regressions. Transport/SDK submission and reservation
 * counters are controlled fixtures, never live calls or real allocations.
 * Native transaction bytes and financial public states come from retained swap
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
const retained=JSON.parse(read('local-swap-continuation-01/run-public/integration-result.json'));
const stages=['deploy','initialize','swap','close'];
const summaries=Object.fromEntries(stages.map(s=>[s,JSON.parse(read(`local-swap-continuation-01/run-public/stage-${s}.json`))]));
const receiptByStage=Object.fromEntries(retained.driver.stages.map(r=>[r.circuitId,r]));
const raws={deploy:read('local-swap-01/run-public/public-transactions/'+receiptByStage.deploy.transaction.transactionHash+'.bin'),initialize:read('local-swap-01/run-public/public-transactions/'+receiptByStage.initialize.transaction.transactionHash+'.bin'),...Object.fromEntries(['swap','close'].map(s=>[s,read('local-swap-continuation-01/run-public/public-transactions/'+receiptByStage[s].transaction.transactionHash+'.bin')]))};
const bytes=s=>Uint8Array.from(Buffer.from(s,'hex'));
const address=receiptByStage.deploy.contractAddress,first=summaries.deploy.publicState.traderAddress.bytes,second=summaries.deploy.publicState.providerAddress.bytes,networkTag=summaries.deploy.publicState.networkTag;
const color=summaries.initialize.publicState.colorA;
const publicKey=[...ledger.Transaction.deserialize('signature','proof','binding',raws.swap).intents.values()].flatMap(i=>[i.guaranteedUnshieldedOffer,i.fallibleUnshieldedOffer]).filter(Boolean).flatMap(o=>o.inputs)[0].owner;
const roles={firstAddress:first,secondAddress:second,firstSecret:new Uint8Array(32).fill(17),secondSecret:new Uint8Array(32).fill(18)};
function decodedState(stage){
 const p=summaries[stage].publicState;
 const scalars=x=>typeof x==='string'?BigInt(x):Object.fromEntries(Object.entries(x).map(([k,v])=>[k,scalars(v)]));
 const s={...p,initialized:p.initialized,remaining:BigInt(p.remaining),revision:BigInt(p.revision),kernelState:scalars(p.kernelState),lastSwap:scalars(p.lastSwap),lastClose:scalars(p.lastClose)};
 for(const key of ['programDigest','networkTag','assetADomain','assetBDomain','colorA','colorB'])s[key]=bytes(p[key]);
 for(const [role,secret] of [['trader',roles.firstSecret],['provider',roles.secondSecret]]){s[role+'Address']={bytes:bytes(p[role+'Address'].bytes)};s[role+'Capability']=runtime.persistentHash(new runtime.CompactTypeVector(4,new runtime.CompactTypeBytes(32)),[Uint8Array.from(Buffer.from(`moriarty:sp05:swap:${role}`.padEnd(32,'\0'))),bytes(networkTag),bytes(p.programDigest),secret]);}
 return s;
}

function fixture({wireMutation,observationMutation,bindingMutation}={}){
 const calls=[],compared=[],watched=[];let stopped=false,closed=false;const priorCount=6,priorDust=1800000000000006n;let charges=priorDust,gross=0n;
 const network={networkId:'preview',node:'https://rpc.preview.midnight.network',indexer:'https://indexer.preview.midnight.network/api/v4/graphql',indexerWS:'wss://indexer.preview.midnight.network/api/v4/graphql/ws',proofServer:'http://127.0.0.1:16300'};
 const binding={network,payerAddress:first,sdkNetworkId:'preview'};
 const submit=stage=>{calls.push(stage);charges+=BigInt(receiptByStage[stage].transaction.dustFee);gross+=BigInt(receiptByStage[stage].transaction.grossByAsset[color]??0);return receiptByStage[stage].txId;};
 const sdkCalls={deployContract:async()=>({deployTxData:{public:{contractAddress:address,txId:submit('deploy')}}}),submitCallTx:async(_p,o)=>({public:{txId:submit(o.circuitId)}})};
 const options={kind:'swap',sourceTestOnly:true,walletContext:{wallet:{async stop(){closed=true;},async waitForSyncedState(){const o=receiptByStage.initialize.transaction.outputs[0],progress={isConnected:true,isStrictlyComplete:()=>true};return {shielded:{progress},unshielded:{progress,pendingCoins:[],availableCoins:[{utxo:{owner:MidnightBech32m.encode('preview',new UnshieldedAddress(Buffer.from(o.owner,'hex'))).toString(),type:o.type,value:BigInt(o.value),intentHash:o.intentHash,outputNo:o.offerIndex}}]},dust:{progress,state:{pendingDust:[]}}};}},unshieldedKeystore:{getPublicKey:()=>publicKey,getBech32Address:()=>MidnightBech32m.encode('preview',new UnshieldedAddress(Buffer.from(first,'hex')))},shieldedSecretKeys:{coinPublicKey:'synthetic-coin',encryptionPublicKey:'synthetic-encryption'}},networkConfig:network,roles,networkTag,deploymentSigningKey:'synthetic-signing-key-not-used',build:{receiptPath:'/synthetic/public-build.json',receiptSha256:'11'.repeat(32),sourceManifestHash:'22'.repeat(32)},privateStateConfig:{},limits:{allocationId:'composed-negative-only',reservationStatePath:'/synthetic/no-file-created',deadlineMs:Date.now()+10000,submissions:4,dustFee:2000000000000000n,grossByLogicalAsset:{ASSET_A:100000n,ASSET_B:0n}},expectedProtocolVersion:1000000,onEvent:()=>{},now:()=>1700000000n,onStage:async s=>{compared.push(s.stage);if(bindingMutation)bindingMutation(s.stage,binding);return {status:'RECORDED',stage:s.stage,txId:s.txId};}};
 options.adapters={loadPublicIdentity:()=>({address:MidnightBech32m.encode('preview',new UnshieldedAddress(Buffer.from(first,'hex'))).toString()}),loadAssets:async()=>({compiledContract:{sourceOnly:true},decodeState:decodedState,zkConfigPath:'/synthetic/no-proving-assets',assertFresh(){},cleanup(){}}),loadContractsSdk:async()=>sdkCalls,loadProviderSdk:async()=>({getNetworkId:()=> 'preview',NodeZkConfigProvider:class{}}),loadNativeRuntime:async()=>({ledger,runtime}),prepareDeployment:async()=>({public:{contractAddress:address},driverSdk:sdkCalls}),initializeReservations:()=>{},createProviders:async()=>({publicDataProvider:{},execute:async(_label,fn)=>fn(),stop(){stopped=true;},getExecutionBinding:()=>binding,getState:()=>({reservedSubmissions:priorCount+calls.length,reservedDustFee:charges,reservedGrossByAsset:gross?{[color]:gross}:{},identifiers:calls.map(s=>receiptByStage[s].txId)}),cleanup:async()=>{closed=true;return {walletStopped:true,pendingOperations:0,containmentComplete:true};}}),createComparator:createFinancialComparator,driver:runPreviewFinancialCase,fetch:async(_url,o)=>{const r=JSON.parse(o.body);assert.equal(r.method,'chain_getBlockHash');assert.deepEqual(r.params,[0]);return Response.json({jsonrpc:'2.0',id:r.id,result:'0x'+networkTag});},observe:async o=>{
  const stage=o.circuitId,r=receiptByStage[stage];
  const row=x=>({owner:MidnightBech32m.encode('preview',new UnshieldedAddress(Buffer.from(x.owner,'hex'))).toString(),tokenType:x.type,value:BigInt(x.value),intentHash:x.intentHash});
  const data={tx:ledger.Transaction.deserialize('signature','proof','binding',raws[stage]),txId:r.txId,txHash:r.transaction.transactionHash,identifiers:r.transaction.identifiers,status:'SucceedEntirely',protocolVersion:r.protocolVersion,blockHash:r.blockHash,blockHeight:r.blockHeight,fees:{paidFees:r.fees.indexerReported.paid,estimatedFees:r.fees.indexerReported.estimated},unshielded:{created:r.transaction.outputs.map(row),spent:r.transaction.inputs.map(row)}};
  if(wireMutation)wireMutation(stage,data);
  const provider={watchForTxData:async id=>{watched.push(stage);assert.equal(id,r.txId);return data;},queryContractState:async()=>{const state=new stateRuntime.ContractState();state.balance=new Map(Object.entries(r.contractBalances).map(([raw,v])=>[{tag:'unshielded',raw},BigInt(v)]));return state;},queryUnshieldedBalances:async()=>Object.entries(r.contractBalances).map(([tokenType,value])=>({tokenType,balance:BigInt(value)}))};
  const rpc=async(method,args)=>method==='chain_getHeader'?{number:'0x'+r.blockHeight.toString(16)}:method==='chain_getBlockHash'?'0x'+r.blockHash:'0x'+r.blockHash;
  const observed=await observeFinalizedStage({...o,provider,rpc,decodeState:()=>o.decodeState(stage)});if(observationMutation)observationMutation(stage,observed);return observed;
 }};
 return {options,calls,compared,watched,state:()=>({stopped,closed,charges,gross,priorCount,priorDust})};
}


// Complete controlled swap integration, not a Preview transaction. All native
// transaction bytes are retained actual LOCAL bytes; state capabilities only
// are rebound to synthetic secrets, balances are native test containers.
test('full Preview swap composition traverses native receipts, independent comparator and minted wallet gate',async()=>{
 const f=fixture(),result=await api.integratePreviewFinancialCase(f.options);
 assert.equal(result.status,'SOURCE_TEST_ONLY');assert.equal(result.financialComparison.status,'PASS');assert.deepEqual(f.calls,stages);assert.deepEqual(f.watched,stages);assert.deepEqual(f.compared,stages);assert.equal(result.financialAcceptance,false);assert.equal(f.state().closed,true);
});
test('Preview swap bad close recipient stops actual financial comparator',async()=>{
 const f=fixture({observationMutation:(s,o)=>{if(s==='close')o.receipt.transaction.outputs[0].owner=first;}});
 await assert.rejects(api.integratePreviewFinancialCase(f.options),{message:'PARTICIPANT_NET_DELTA'});assert.equal(f.state().closed,true);
});

test('Preview swap pending minted wallet coin stops before swap submission callback',async()=>{
 const f=fixture(),original=f.options.walletContext.wallet.waitForSyncedState;
 f.options.walletContext.wallet.waitForSyncedState=async()=>{const s=await original();s.unshielded.pendingCoins=[{}];return s;};
 await assert.rejects(api.integratePreviewFinancialCase(f.options),{message:'SWAP_WALLET_PENDING'});assert.deepEqual(f.calls,['deploy','initialize']);assert.equal(f.state().closed,true);
});
