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

const recoveredIdentity=()=>({contractAddress:'ab'.repeat(32),txId:'00959c51e7d62ee9160bf1396ce0ab52f26757a7c5adec669cb083d5a8787d1de9'});
function recoveryHarness() {
 const h=harness();h.options.existingDeployment=recoveredIdentity();
 for(const name of ['deployContract','createUnprovenDeployTx','findDeployedContract','prepareFinancialDeployment'])Object.defineProperty(h.options.sdk,name,{get(){throw Error('RECOVERY_DEPLOY_API_FORBIDDEN:'+name);}});
 return h;
}
test('recovered loan observes historical deployment once then submits exactly three calls',async()=>{
 const h=recoveryHarness(),events=[];
 h.options.providers.execute=async(stage,f)=>{events.push(stage);return f();};
 h.options.observe=async x=>{assert.equal(x.contractAddress,h.options.existingDeployment.contractAddress);return {receipt:{txId:x.txId},state:{observed:x.circuitId}};};
 const out=await runLocalFinancialCase(h.options);
 assert.deepEqual(events,['observe:deploy','compare:deploy','initialize','observe:initialize','compare:initialize','accrue','observe:accrue','compare:accrue','settle','observe:settle','compare:settle']);
 assert.deepEqual(h.calls.map(x=>x.stage),['initialize','accrue','settle']);
 assert.ok(h.calls.every(x=>x.options.contractAddress===recoveredIdentity().contractAddress&&x.options.privateStateId==='sp05-loan'));
 assert.equal(out.status,'PASS');assert.equal(out.stages.length,4);assert.deepEqual(out.transactionIds,[recoveredIdentity().txId,'initialize','accrue','settle']);assert.equal(out.operationalState.reservedSubmissions,3);assert.deepEqual(h.counts(),{cleaned:1,stopped:0});
});
for(const value of [null,[],{},'identity',{...recoveredIdentity(),extra:true},{...recoveredIdentity(),contractAddress:'ab'},{...recoveredIdentity(),contractAddress:'AB'.repeat(32)},{...recoveredIdentity(),txId:''},{...recoveredIdentity(),txId:'deploy'},{...recoveredIdentity(),txId:'cd'.repeat(32)},{...recoveredIdentity(),txId:'CD'.repeat(33)},Object.assign(Object.create({extra:true}),recoveredIdentity()),Object.defineProperty(recoveredIdentity(),'hidden',{value:true}),Object.assign(recoveredIdentity(),{[Symbol('extra')]:true})])test('malformed recovery identity fails before observations or SDK calls',async()=>{
 const h=harness();h.options.existingDeployment=value;let observed=0;h.options.observe=()=>{observed++;throw Error('OBSERVATION_FORBIDDEN');};
 await assert.rejects(runLocalFinancialCase(h.options),e=>{assert.equal(e.message,'INVALID_EXISTING_DEPLOYMENT');assert.deepEqual(e.publicResult.transactionIds,[]);return true;});assert.equal(h.calls.length,0);assert.equal(observed,0);assert.deepEqual(h.counts(),{cleaned:1,stopped:1});
});
test('recovery rejects accessors without invoking them',async()=>{
 const h=harness();h.options.existingDeployment=Object.defineProperty(recoveredIdentity(),'txId',{get(){throw Error('GETTER_FORBIDDEN');}});
 await assert.rejects(runLocalFinancialCase(h.options),/INVALID_EXISTING_DEPLOYMENT/);assert.equal(h.calls.length,0);assert.equal(h.counts().cleaned,1);
});
test('swap recovery is rejected before deployment or observation',async()=>{
 const h=harness('swap');h.options.existingDeployment=recoveredIdentity();await assert.rejects(runLocalFinancialCase(h.options),/EXISTING_DEPLOYMENT_REQUIRES_LOAN/);assert.equal(h.calls.length,0);assert.equal(h.counts().cleaned,1);
});
for(const outcome of ['FAIL','throw','observe'])test(`historical recovery ${outcome} stops all calls and retains public identity`,async()=>{
 const h=recoveryHarness();let compared=0;
 h.options.verifyStage=()=>{compared++;if(outcome==='throw')throw Error('HISTORICAL_FAILURE');return {status:'FAIL'};};
 if(outcome==='observe')h.options.observe=()=>{throw Error('HISTORICAL_FAILURE');};
 await assert.rejects(runLocalFinancialCase(h.options),e=>{assert.equal(e.publicResult.status,'FAILED');assert.equal(e.publicResult.contractAddress,recoveredIdentity().contractAddress);assert.deepEqual(e.publicResult.transactionIds,[recoveredIdentity().txId]);assert.deepEqual(e.publicResult.stages,[]);return true;});
 assert.equal(compared,outcome==='observe'?0:1);assert.equal(h.calls.length,0);assert.deepEqual(h.counts(),{cleaned:1,stopped:1});
});
test('recovery identity is captured before asynchronous binding checks',async()=>{
 const h=recoveryHarness(),seen=[];
 h.options.providers.getExecutionBinding=async()=>{h.options.existingDeployment.contractAddress='ef'.repeat(32);h.options.existingDeployment.txId='12'.repeat(32);return h.binding;};
 h.options.observe=async x=>{seen.push(x);return {receipt:{txId:x.txId}};};
 const out=await runLocalFinancialCase(h.options);assert.equal(out.contractAddress,recoveredIdentity().contractAddress);assert.equal(seen[0].txId,recoveredIdentity().txId);assert.ok(h.calls.every(x=>x.options.contractAddress===recoveredIdentity().contractAddress));
});

test('recovered loan cannot pass with incomplete containment',async()=>{
 const h=recoveryHarness();h.options.providers.cleanup=async()=>({walletStopped:true,pendingOperations:1,containmentComplete:false});
 await assert.rejects(runLocalFinancialCase(h.options),e=>{assert.equal(e.message,'DRIVER_CLEANUP_INCOMPLETE');assert.equal(e.publicResult.status,'INCOMPLETE');assert.equal(e.publicResult.stages.length,4);assert.equal(e.publicResult.operationalState.reservedSubmissions,3);return true;});
});


test('public failure retains a known observer code and closed phase without SDK details',async()=>{
 const h=harness();h.options.observe=async()=>{throw Error('NOT_FINALIZED');};
 await assert.rejects(runLocalFinancialCase(h.options),e=>{assert.deepEqual(e.publicResult.failure,{phase:'observe',stage:'deploy',code:'NOT_FINALIZED'});return true;});
});
test('public failure never copies an unknown SDK message or object',async()=>{
 const h=harness();h.options.observe=async()=>{const e=Error('PRIVATE_VALUE_ABC123');e.privateState={secret:'PRIVATE_OTHER'};throw e;};
 await assert.rejects(runLocalFinancialCase(h.options),e=>{assert.deepEqual(e.publicResult.failure,{phase:'observe',stage:'deploy',code:'UNCLASSIFIED_DRIVER_FAILURE'});assert.ok(!JSON.stringify(e.publicResult).includes('PRIVATE_'));return true;});
});

for(const message of ['NOT_FINALIZED','PRIVATE_SDK_FAILURE'])test(`launcher durably retains actual driver failure: ${message}`,async()=>{
 const h=harness(),directory=mkdtempSync(join(tmpdir(),'moriarty-driver-retention-'));
 h.options.observe=async()=>{throw Object.assign(Error(message),{private:{secret:'PRIVATE_CANARY'}});};
 try{
  await assert.rejects(runLocalFinancialCase(h.options),error=>{
   const driver=error.publicResult;
   const result={schema:'moriarty.local-financial-integration/1',status:'FAILED',kind:'loan',sourceTestOnly:false,networkAcceptance:false,proofAcceptance:false,financialAcceptance:false,build:undefined,assetBindings:undefined,phase:'driver',driver,cleanup:driver.cleanup,setupPendingOperations:0,comparisons:[],financialComparison:undefined,scope:'Source-only fixture; no network acceptance'};
   retainPublicIntegrationResult(directory,result);
   const raw=readFileSync(join(directory,'integration-result.json'),'utf8'),saved=JSON.parse(raw);
   assert.deepEqual(saved.driver.failure,{phase:'observe',stage:'deploy',code:message==='NOT_FINALIZED'?message:'UNCLASSIFIED_DRIVER_FAILURE'});
   assert.deepEqual(saved.driver.transactionIds,['deploy']);assert.deepEqual(saved.cleanup,complete);
   assert.ok(!raw.includes('PRIVATE_')&&!raw.includes('NEVER_PUBLISH'));return true;
  });
  assert.equal(h.calls.length,1);assert.deepEqual(h.counts(),{cleaned:1,stopped:1});
 }finally{rmSync(directory,{recursive:true,force:true});}
});

function initializedDriverFixture(){
 const h=harness();h.options.initializedLoan={contractAddress:'ba4c808859fc2e4ee6d3d19fa0d812bb9a9c9eb0527161fb91315213bc24a713',deployTxId:'00959c51e7d62ee9160bf1396ce0ab52f26757a7c5adec669cb083d5a8787d1de9',initializeTxId:'006466368995501afc82b36cb38dac6f1cee531565eb75b70db6f3af5af36d9b46'};
 delete h.options.sdk.deployContract;return h;
}
test('fixed initialized loan compares both histories once then calls only accrue and settle',async()=>{
 const h=initializedDriverFixture(),seen=[];h.options.verifyStage=stage=>{seen.push(stage);return {status:'PASS'};};
 const result=await runLocalFinancialCase(h.options);
 assert.deepEqual(h.calls.map(c=>c.stage),['accrue','settle']);assert.deepEqual(seen,['deploy','initialize','accrue','settle']);assert.equal(result.stages.length,4);
 assert.deepEqual(result.transactionIds.slice(0,2),[h.options.initializedLoan.deployTxId,h.options.initializedLoan.initializeTxId]);
 assert.deepEqual(h.calls[0].options.args.slice(3,5),[0n,2n]);assert.deepEqual(h.calls[1].options.args.slice(3,7),[1n,2n,0n,533972602n]);
});
for(const historical of ['deploy','initialize'])test(`failed historical ${historical} comparison prevents continuation calls`,async()=>{
 const h=initializedDriverFixture();h.options.verifyStage=stage=>({status:stage===historical?'FAIL':'PASS'});
 await assert.rejects(runLocalFinancialCase(h.options),/FINANCIAL_COMPARISON_REQUIRED_PASS/);assert.equal(h.calls.length,0);assert.equal(h.counts().cleaned,1);
});
test('initialized driver rejects combined modes and altered fixed history before SDK calls',async()=>{
 for(const change of [h=>h.options.kind='swap',h=>h.options.existingDeployment={contractAddress:'ab'.repeat(32),txId:'01'.repeat(33)},h=>h.options.initializedLoan.contractAddress='ab'.repeat(32),h=>h.options.initializedLoan.initializeTxId='01'.repeat(33),h=>h.options.initializedLoan.cursor=2]){
  const h=initializedDriverFixture();change(h);await assert.rejects(runLocalFinancialCase(h.options),/INITIALIZED_LOAN/);assert.equal(h.calls.length,0);
 }
});

test('fresh local four-stage completion does not require external containment',async()=>{
 const h=harness();h.options.providers.cleanup=async()=>({walletStopped:true,pendingOperations:0,containmentComplete:false});
 const result=await runLocalFinancialCase(h.options);assert.equal(result.status,'FINANCIAL_COMPLETE');assert.equal(result.cleanup.containmentComplete,false);
});
