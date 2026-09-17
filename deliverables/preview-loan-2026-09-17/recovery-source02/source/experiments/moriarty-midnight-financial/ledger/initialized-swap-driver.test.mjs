import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync,mkdtempSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {retainPublicIntegrationResult} from './launch-local.mjs';
import {runLocalFinancialCase} from './run-local.mjs';
const bindings=JSON.parse(readFileSync(new URL('../custody/bindings.json',import.meta.url),'utf8'));
const network={networkId:'undeployed',node:'http://127.0.0.1:19944',indexer:'http://127.0.0.1:18088/api/v4/graphql',indexerWS:'ws://127.0.0.1:18088/api/v4/graphql/ws',proofServer:'http://127.0.0.1:16300'};
const complete={walletStopped:true,pendingOperations:0,containmentComplete:true};
const product=(a,b,lo)=>({aLo:a,aHi:0n,bLo:b,bHi:0n,lo,carry:0n});
function harness(kind='loan') {
 const calls=[],address='ab'.repeat(32),ids=[];
 let cleaned=0,stopped=0;
 const roles={firstSecret:new Uint8Array(32).fill(1),secondSecret:new Uint8Array(32).fill(2),firstAddress:'01'.repeat(32),secondAddress:'02'.repeat(32)};
 const binding={network:{...network},payerAddress:roles.firstAddress,sdkNetworkId:'undeployed'};
 const options={kind,network:'undeployed',compiledContract:{},roles,networkTag:'03'.repeat(32),now:()=>1700000000n,
 providers:{execute:async(_,f)=>f(),stop:()=>stopped++,cleanup:async()=>{cleaned++;return complete;},getState:()=>({identifiers:[...ids],pendingOperations:0,reservedSubmissions:ids.length,reservedDustFee:0n,reservedGrossByAsset:{}}),getExecutionBinding:()=>binding},
 sdk:{deployContract:async(p,o)=>{calls.push({stage:'deploy',options:o});ids.push('deploy');return {deployTxData:{public:{contractAddress:address,txId:'deploy'},private:{canary:'NEVER_PUBLISH'}}};},submitCallTx:async(p,o)=>{calls.push({stage:o.circuitId,options:o});ids.push(o.circuitId);return {public:{txId:o.circuitId},private:{canary:'NEVER_PUBLISH'}};}},
 observe:async x=>({receipt:{txId:x.txId},state:{observed:x.circuitId}}),verifyStage:()=>({status:'PASS'})};
 return {options,calls,binding,ids,counts:()=>({cleaned,stopped})};
}

import {EXISTING_SWAP,INITIALIZED_SWAP} from './continue-initialized-swap.mjs';
// Controlled SDK/providers only; no live or private-state evidence.
function initializedSwapFixture(){const h=harness('swap');h.options.initializedSwap={contractAddress:INITIALIZED_SWAP.contractAddress,deployTxId:EXISTING_SWAP.txId,initializeTxId:INITIALIZED_SWAP.txId};h.options.sdk.deployContract=()=>{throw Error('DEPLOY_TRAP');};return h;}
test('initialized swap compares both histories before exactly swap and close',async()=>{const h=initializedSwapFixture(),seen=[];h.options.verifyStage=stage=>{seen.push(stage);if(['deploy','initialize'].includes(stage))assert.equal(h.calls.length,0);return {status:'PASS'};};const out=await runLocalFinancialCase(h.options);assert.deepEqual(h.calls.map(x=>x.stage),['swap','close']);assert.deepEqual(seen,['deploy','initialize','swap','close']);assert.deepEqual(out.transactionIds.slice(0,2),[EXISTING_SWAP.txId,INITIALIZED_SWAP.txId]);assert.equal(out.stages.length,4);assert.deepEqual(h.calls[0].options.args.slice(3,10),[0n,4n,4n,0n,1n,10000n,19700n]);assert.deepEqual(h.calls[1].options.args.slice(3,5),[1n,3n]);assert.equal(h.counts().cleaned,1);});
for(const stage of ['deploy','initialize'])test(`initialized swap failed ${stage} comparison prevents new submissions`,async()=>{const h=initializedSwapFixture();h.options.verifyStage=s=>({status:s===stage?'FAIL':'PASS'});await assert.rejects(runLocalFinancialCase(h.options),e=>{assert.match(e.message,/FINANCIAL_COMPARISON_REQUIRED_PASS/);assert.equal(e.publicResult.status,'FAILED');return true;});assert.equal(h.calls.length,0);assert.equal(h.counts().cleaned,1);});
test('initialized swap rejects altered, mixed and accessor identities without calling SDK',async()=>{for(const change of [h=>h.options.kind='loan',h=>h.options.initializedSwap.contractAddress='ab'.repeat(32),h=>h.options.initializedSwap.initializeTxId='01'.repeat(33),h=>h.options.initializedSwap.extra=true,h=>h.options.initializedLoan={},h=>h.options.existingDeployment={}]){const h=initializedSwapFixture();change(h);await assert.rejects(runLocalFinancialCase(h.options),/INVALID_INITIALIZED_SWAP/);assert.equal(h.calls.length,0);}const h=initializedSwapFixture();let calls=0;Object.defineProperty(h.options.initializedSwap,'deployTxId',{get(){calls++;throw Error('ACCESSOR_TRAP');}});await assert.rejects(runLocalFinancialCase(h.options),/INVALID_INITIALIZED_SWAP/);assert.equal(calls,0);assert.equal(h.calls.length,0);});
