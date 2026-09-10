import test from 'node:test';
import assert from 'node:assert/strict';
import * as driver from './run-local.mjs';
const preview={networkId:'preview',node:'https://rpc.preview.midnight.network',indexer:'https://indexer.preview.midnight.network/api/v4/graphql',indexerWS:'wss://indexer.preview.midnight.network/api/v4/graphql/ws',proofServer:'http://127.0.0.1:16300'};
const local={networkId:'undeployed',node:'http://127.0.0.1:19944',indexer:'http://127.0.0.1:18088/api/v4/graphql',indexerWS:'ws://127.0.0.1:18088/api/v4/graphql/ws',proofServer:'http://127.0.0.1:16300'};
function fixture(kind='loan',network='preview'){
 const calls=[],events=[],ids=[];let cleaned=0,stopped=0;
 const roles={firstSecret:new Uint8Array(32).fill(1),secondSecret:new Uint8Array(32).fill(2),firstAddress:'01'.repeat(32),secondAddress:'02'.repeat(32)};
 const binding={network:{...(network==='preview'?preview:local)},sdkNetworkId:network,payerAddress:roles.firstAddress};
 const options={kind,network,roles,networkTag:'03'.repeat(32),compiledContract:{},now:()=>1700000000n,
 providers:{getExecutionBinding:()=>binding,execute:async(stage,op)=>{events.push(stage);return op();},stop:()=>stopped++,cleanup:async()=>{cleaned++;return {walletStopped:true,pendingOperations:0,containmentComplete:true};},getState:()=>({identifiers:[...ids],reservedSubmissions:ids.length,reservedDustFee:123n,reservedGrossByAsset:{['04'.repeat(32)]:456n}})},
 sdk:{deployContract:async(_p,o)=>{calls.push({stage:'deploy',options:o});ids.push('deploy');return {deployTxData:{public:{contractAddress:'ab'.repeat(32),txId:'deploy'},private:{secret:'PRIVATE_CANARY'}}};},submitCallTx:async(_p,o)=>{calls.push({stage:o.circuitId,options:o});ids.push(o.circuitId);return {public:{txId:o.circuitId},private:{secret:'PRIVATE_CANARY'}};}},
 observe:async o=>{assert.equal(o.network,network==='preview'?'preview':undefined);return {receipt:{txId:o.txId},state:{stage:o.circuitId}};},verifyStage:(stage,o)=>{assert.equal(o.state.stage,stage);return {status:'PASS'};}};
 return {options,binding,calls,events,counts:()=>({cleaned,stopped})};
}
for(const kind of ['loan','swap'])test('Preview '+kind+' reuses exact local custody arguments and guarded compare-before-next-call order',async()=>{
 assert.equal(typeof driver.runPreviewFinancialCase,'function');
 const p=fixture(kind),l=fixture(kind,'undeployed');
 const result=await driver.runPreviewFinancialCase(p.options);await driver.runLocalFinancialCase(l.options);
 assert.deepEqual(p.calls,l.calls);assert.deepEqual(p.events,l.events);
 assert.deepEqual(p.calls.map(x=>x.stage),kind==='loan'?['deploy','initialize','accrue','settle']:['deploy','initialize','swap','close']);
 assert.equal(result.schema,'moriarty.preview-financial-run/1');assert.equal(result.status,'PASS');
 assert.equal(result.operationalState.reservedSubmissions,4);assert.equal(result.operationalState.reservedDustFee,'123');
 assert.deepEqual(result.operationalState.reservedGrossByAsset,{['04'.repeat(32)]:'456'});
 assert.equal(result.stages.length,4);assert.equal(p.counts().cleaned,1);assert.ok(!JSON.stringify(result).includes('PRIVATE_CANARY'));assert.match(result.scope,/not mandatory PCD acceptance/);
});
for(const [label,change,code] of [
 ['wrong entry network',f=>f.options.network='undeployed','PREVIEW_DRIVER_REQUIRES_PREVIEW_NETWORK'],
 ['wrong SDK network',f=>f.binding.sdkNetworkId='undeployed','PREVIEW_EXECUTION_BINDING_MISMATCH'],
 ['wrong payer',f=>f.binding.payerAddress='ff'.repeat(32),'PREVIEW_EXECUTION_BINDING_MISMATCH'],
 ['wrong node',f=>f.binding.network.node='https://rpc.mainnet.midnight.network','PREVIEW_EXECUTION_BINDING_ENDPOINT'],
 ['wrong indexer',f=>f.binding.network.indexer='https://evil.test/api/v4/graphql','PREVIEW_EXECUTION_BINDING_ENDPOINT'],
 ['wrong WS',f=>f.binding.network.indexerWS='ws://indexer.preview.midnight.network/api/v4/graphql/ws','PREVIEW_EXECUTION_BINDING_ENDPOINT'],
 ['remote prover',f=>f.binding.network.proofServer='http://evil.test','PREVIEW_EXECUTION_BINDING_ENDPOINT'],
 ...['existingDeployment','initializedLoan','initializedSwap'].map(key=>[key,f=>f.options[key]={},'PREVIEW_LOCAL_RECOVERY_FORBIDDEN'])
])test('Preview rejects '+label+' before any SDK operation and cleans up',async()=>{
 assert.equal(typeof driver.runPreviewFinancialCase,'function');const f=fixture();change(f);
 await assert.rejects(driver.runPreviewFinancialCase(f.options),e=>{assert.equal(e.message,code);assert.equal(e.publicResult.status,'FAILED');return true;});assert.equal(f.calls.length,0);assert.deepEqual(f.counts(),{cleaned:1,stopped:1});
});
test('Preview rejects stable-target binding mutation before the next submission',async()=>{
 assert.equal(typeof driver.runPreviewFinancialCase,'function');const f=fixture();
 f.options.verifyStage=()=>{f.binding.newAllocation='unexpected-change';return {status:'PASS'};};
 await assert.rejects(driver.runPreviewFinancialCase(f.options),{message:'PREVIEW_EXECUTION_BINDING_CHANGED'});assert.equal(f.calls.length,1);assert.equal(f.counts().cleaned,1);
});
for(const phase of ['budget','observe','compare'])test('Preview '+phase+' failure prevents next SDK call and retains known identifiers',async()=>{
 assert.equal(typeof driver.runPreviewFinancialCase,'function');const f=fixture();
 if(phase==='budget')f.options.providers.execute=async(stage,op)=>{if(stage==='initialize')throw Error('BUDGET_STOP');return op();};
 if(phase==='observe')f.options.observe=async()=>{throw Error('NONCANONICAL_BLOCK');};
 if(phase==='compare')f.options.verifyStage=()=>({status:'FAIL'});
 await assert.rejects(driver.runPreviewFinancialCase(f.options),e=>{assert.deepEqual(e.publicResult.transactionIds,['deploy']);assert.equal(e.publicResult.status,'FAILED');return true;});assert.equal(f.calls.length,1);assert.equal(f.counts().cleaned,1);
});
test('Preview incomplete cleanup cannot produce PASS',async()=>{
 assert.equal(typeof driver.runPreviewFinancialCase,'function');const f=fixture();f.options.providers.cleanup=async()=>({walletStopped:true,pendingOperations:1,containmentComplete:false});
 await assert.rejects(driver.runPreviewFinancialCase(f.options),e=>{assert.equal(e.message,'DRIVER_CLEANUP_INCOMPLETE');assert.equal(e.publicResult.status,'INCOMPLETE');assert.equal(e.publicResult.stages.length,4);return true;});
});
