import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
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
for(const kind of ['loan','swap'])test(`${kind} exact custody arguments and hints pass every comparison before next call`,async()=>{
 const h=harness(kind),seen=[];h.options.verifyStage=(stage,x)=>{seen.push(stage);assert.equal(h.calls.at(-1).stage,stage);assert.equal(x.state.observed,stage);return {status:'PASS'};};
 const out=await runLocalFinancialCase(h.options);const r=h.options.roles,p=Uint8Array.from(Buffer.from(bindings[kind].programDigest,'hex')),n=new Uint8Array(32).fill(3),t=1700000000n;
 assert.deepEqual(h.calls[0].options.args,[r.firstSecret,r.secondSecret,{bytes:Uint8Array.from(Buffer.from(r.firstAddress,'hex'))},{bytes:Uint8Array.from(Buffer.from(r.secondAddress,'hex'))},p,n]);
 const expected=kind==='loan'?[
 ['initialize',[r.firstSecret,p,n,2n,t]],
 ['accrue',[r.firstSecret,p,n,0n,2n,t,{h0:product(5000000000n,8n,40000000000n),h1:product(40000000000n,31n,1240000000000n),h2:product(100n,365n,36500n),h3:{q:33972602n,r:27000n,product:product(36500n,33972602n,1239999973000n)}}]],
 ['settle',[r.firstSecret,p,n,1n,2n,0n,533972602n,t,{unused:0n}]]
 ]:[
 ['initialize',[r.secondSecret,p,n,3n,t]],
 ['swap',[r.firstSecret,p,n,0n,4n,4n,0n,1n,10000n,19700n,t,{h0:product(10000n,997n,9970000n),h1:product(9970000n,2000000n,19940000000000n),h2:product(1000000n,1000n,1000000000n),h3:{q:19743n,r:162290000n,product:product(1009970000n,19743n,19939837710000n)}}]],
 ['close',[r.secondSecret,p,n,1n,3n,t,{unused:0n}]]];
 assert.deepEqual(h.calls.slice(1).map(x=>[x.stage,x.options.args]),expected);assert.deepEqual(seen,['deploy',...expected.map(x=>x[0])]);assert.equal(out.status,'PASS');assert.deepEqual(out.cleanup,complete);assert.deepEqual(out.transactionIds,seen);assert.ok(!JSON.stringify(out).includes('NEVER_PUBLISH'));assert.equal(h.counts().cleaned,1);
});
for(const result of [false,undefined,{status:'Rejected'},{status:'FAIL'}])test(`returned comparator failure ${JSON.stringify(result)} stops before next SDK call`,async()=>{
 const h=harness();h.options.verifyStage=()=>result;
 await assert.rejects(runLocalFinancialCase(h.options),e=>{assert.match(e.message,/FINANCIAL_COMPARISON_REQUIRED_PASS/);assert.deepEqual(e.publicResult.transactionIds,['deploy']);return true;});assert.equal(h.calls.length,1);assert.equal(h.counts().cleaned,1);
});
test('thrown comparator error retains observed ID and cleans up',async()=>{
 const h=harness();h.options.verifyStage=()=>{throw Error('COMPARISON_FAILED');};await assert.rejects(runLocalFinancialCase(h.options),e=>{assert.equal(e.message,'COMPARISON_FAILED');assert.deepEqual(e.publicResult.transactionIds,['deploy']);return true;});assert.equal(h.calls.length,1);assert.equal(h.counts().cleaned,1);
});
for(const mutate of [h=>h.options.network='preview',h=>h.binding.network.networkId='preview',h=>h.binding.sdkNetworkId='preview',h=>h.binding.payerAddress='ff'.repeat(32),h=>h.binding.network.node='https://rpc.preview.midnight.network',h=>h.options.roles.firstSecret=new Uint8Array(1)])test('preflight mismatch prevents deployment but cleans up owned provider',async()=>{
 const h=harness();mutate(h);await assert.rejects(runLocalFinancialCase(h.options),e=>{assert.equal(e.publicResult.status,'FAILED');return true;});assert.equal(h.calls.length,0);assert.equal(h.counts().cleaned,1);
});
test('network identity changed after deploy blocks the next SDK stage',async()=>{
 const h=harness();h.options.verifyStage=()=>{h.binding.sdkNetworkId='preview';return {status:'PASS'};};await assert.rejects(runLocalFinancialCase(h.options),/LOCAL_EXECUTION_BINDING/);assert.equal(h.calls.length,1);
});
for(const cleanup of [{walletStopped:false,pendingOperations:0,containmentComplete:false},{walletStopped:true,pendingOperations:1,containmentComplete:false}])test('incomplete cleanup cannot yield driver PASS',async()=>{
 const h=harness();h.options.providers.cleanup=async()=>cleanup;await assert.rejects(runLocalFinancialCase(h.options),e=>{assert.equal(e.message,'DRIVER_CLEANUP_INCOMPLETE');assert.equal(e.publicResult.status,'INCOMPLETE');assert.deepEqual(e.publicResult.cleanup,cleanup);assert.equal(e.publicResult.transactionIds.length,4);return true;});
});
test('cleanup rejection preserves the primary failure and provider-known identifiers',async()=>{
 const h=harness();h.options.sdk.deployContract=async()=>{h.ids.push('late-public-id');throw Error('SUBMIT_UNKNOWN');};h.options.providers.cleanup=async()=>{throw Error('PRIVATE_CLEANUP_CANARY');};await assert.rejects(runLocalFinancialCase(h.options),e=>{assert.equal(e.message,'SUBMIT_UNKNOWN');assert.deepEqual(e.publicResult.transactionIds,['late-public-id']);assert.equal(e.publicResult.cleanup.containmentComplete,false);assert.ok(!JSON.stringify(e.publicResult).includes('PRIVATE_CLEANUP_CANARY'));return true;});
});

test('observation failure keeps SDK ID and never invokes comparator or next call',async()=>{
 const h=harness();let compared=0;h.options.observe=async()=>{throw Error('NONCANONICAL_BLOCK');};h.options.verifyStage=()=>{compared++;return {status:'PASS'};};
 await assert.rejects(runLocalFinancialCase(h.options),e=>{assert.equal(e.message,'NONCANONICAL_BLOCK');assert.deepEqual(e.publicResult.transactionIds,['deploy']);return true;});assert.equal(compared,0);assert.equal(h.calls.length,1);assert.equal(h.counts().cleaned,1);
});
