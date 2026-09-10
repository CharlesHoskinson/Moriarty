import test from 'node:test';
import assert from 'node:assert/strict';
import {integrateLocalFinancialCase, createLocalRpc} from './integrate-local.mjs';
import {runLocalFinancialCase} from './run-local.mjs';
const first='11'.repeat(32), second='22'.repeat(32), address='33'.repeat(32), color='44'.repeat(32);
function fixture() {
  const calls=[], summaries=[];
  const network={networkId:'undeployed',node:'http://127.0.0.1:9944',indexer:'http://127.0.0.1:8088/graphql',indexerWS:'ws://127.0.0.1:8088/ws',proofServer:'http://127.0.0.1:6300'};
  const options={kind:'loan',walletContext:{wallet:{async stop(){calls.push('wallet-stop');}},unshieldedKeystore:{getPublicKey:()=> 'inert-key'},shieldedSecretKeys:{coinPublicKey:'coin',encryptionPublicKey:'enc'}},networkConfig:network,
    roles:{firstSecret:new Uint8Array(32).fill(1),secondSecret:new Uint8Array(32).fill(2),firstAddress:first,secondAddress:second},networkTag:'55'.repeat(32),deploymentSigningKey:'PRIVATE-DEPLOYMENT-CANARY',
    build:{receiptPath:'/inert/build-receipt.json',receiptSha256:'66'.repeat(32),sourceManifestHash:'77'.repeat(32)},privateStateConfig:{},limits:{allocationId:'inert-allocation',reservationStatePath:'/inert/allocation.json',deadlineMs:Date.now()+10000,submissions:4,dustFee:100n,grossByLogicalAsset:{USD_TEST_ASSET:20n}},expectedProtocolVersion:1000000,now:()=>1700000000n,
    onEvent:event=>calls.push('event:'+event.kind),onStage:async summary=>{summaries.push(summary);calls.push('record:'+summary.stage);return {status:'RECORDED',stage:summary.stage,txId:summary.txId};},sourceTestOnly:true};
  const driverSdk={deployContract:async(_p,o)=>{calls.push('deploy');assert.equal(o.args.length,6);return {deployTxData:{public:{contractAddress:address,txId:'inert-deploy'}}};},submitCallTx:async(_p,o)=>{calls.push(o.circuitId);return {public:{txId:'inert-'+o.circuitId}};}};
  const sdk={getNetworkId:()=> 'undeployed',NodeZkConfigProvider:class {async getVerifierKey(){calls.push('verifier-key');return {};}}};
  let limits, cleaned=false;
  const adapters={
    loadAssets:async()=>{calls.push('load-assets');return {compiledContract:{inert:true},decodeState:x=>x,zkConfigPath:'/inert/assets',assertFresh(){assert.equal(cleaned,false);calls.push('fresh');},cleanup(){cleaned=true;calls.push('asset-cleanup');}};},
    loadContractsSdk:async()=>({inert:true}),loadProviderSdk:async()=>sdk,
    loadNativeRuntime:async()=>({ledger:{addressFromKey:()=>first},runtime:{rawTokenType:(_domain,a)=>{assert.equal(a,address);calls.push('derive-color');return color;}}}),
    prepareDeployment:async o=>{calls.push('prepare');await o.zkConfigProvider.getVerifierKey('initialize');return {public:{contractAddress:address},driverSdk};},
    initializeReservations:o=>{calls.push('initialize-allocation');limits=o.limits;assert.equal(limits.grossByAsset[color],20n);assert.equal('grossByLogicalAsset' in limits,false);},
    createProviders:async()=>{calls.push('providers');return {publicDataProvider:{},async execute(_label,fn){return fn();},stop(){calls.push('provider-stop');},getState:()=>({reservedSubmissions:4,reservedDustFee:10n,reservedGrossByAsset:{[color]:10n},identifiers:[]}),getExecutionBinding:()=>({network,payerAddress:first,sdkNetworkId:'undeployed'}),cleanup:async()=>{calls.push('provider-cleanup');return {walletStopped:true,pendingOperations:0,containmentComplete:false};}};},
    observe:async o=>{calls.push('observe:'+o.circuitId);return {receipt:{circuitId:o.circuitId,txId:o.txId},state:{}};},
    createComparator:async()=>({verifyStage(stage,observation){calls.push('compare:'+stage);return {status:'PASS',stage,txId:observation.receipt.txId,publicState:{observed:true},networkAcceptance:false,proofAcceptance:false};},finish:()=>({status:'PASS',networkAcceptance:false,proofAcceptance:false})}),
    driver:runLocalFinancialCase,fetch:()=>{throw Error('network forbidden in source fixture');},
  };
  options.adapters=adapters;return {options,adapters,calls,summaries,getLimits:()=>limits,sdk};
}
test('full composition prepares address then binds colors once and records every stage before next call',async()=>{
  const f=fixture();let error;
  try{await integrateLocalFinancialCase(f.options);}catch(e){error=e;}
  assert.equal(error.message,'DRIVER_CLEANUP_INCOMPLETE');
  assert.equal(error.publicIntegrationResult.status,'INCOMPLETE');
  assert.equal(error.publicIntegrationResult.sourceTestOnly,true);
  assert.equal(error.publicIntegrationResult.networkAcceptance,false);
  assert(f.calls.indexOf('prepare')<f.calls.indexOf('derive-color'));
  assert(f.calls.indexOf('derive-color')<f.calls.indexOf('initialize-allocation'));
  assert.equal(f.calls.filter(c=>c==='derive-color').length,1);
  for(const [stage,next] of [['deploy','initialize'],['initialize','accrue'],['accrue','settle']])assert(f.calls.indexOf('record:'+stage)<f.calls.indexOf(next));
  assert.equal(f.summaries.length,4);
  assert(!JSON.stringify(error.publicIntegrationResult).includes('PRIVATE-DEPLOYMENT-CANARY'));
  assert.equal(f.calls.at(-1),'asset-cleanup');
});
for(const [label,mutate] of [
 ['wrong network',f=>{f.options.networkConfig.networkId='preview';}],
 ['wrong SDK network',f=>{f.sdk.getNetworkId=()=> 'preview';}],
 ['wrong payer',f=>{f.options.roles.firstAddress='88'.repeat(32);}],
 ['public endpoint',f=>{f.options.networkConfig.node='https://preview.invalid';}],
 ['missing protocol',f=>{delete f.options.expectedProtocolVersion;}],
 ['unknown logical asset',f=>{f.options.limits.grossByLogicalAsset.EXTRA=1n;}],
])test(label+' rejects before preparation and bounded-cleans supplied wallet',async()=>{
 const f=fixture();mutate(f);await assert.rejects(integrateLocalFinancialCase(f.options));assert(!f.calls.includes('prepare'));assert(f.calls.includes('wallet-stop'));
});
test('provider construction failure preserves failure and cleans wallet/assets',async()=>{
 const f=fixture();f.adapters.createProviders=async()=>{throw Error('INERT_PROVIDER_FAILURE');};let error;
 try{await integrateLocalFinancialCase(f.options);}catch(e){error=e;}
 assert.equal(error.message,'INERT_PROVIDER_FAILURE');assert(f.calls.includes('wallet-stop'));assert(f.calls.includes('asset-cleanup'));assert(!f.calls.includes('deploy'));
});
test('driver failure before its cleanup ownership guard falls back to wallet cleanup',async()=>{
 const f=fixture(),create=f.adapters.createProviders;
 f.adapters.createProviders=async o=>({...await create(o),cleanup:undefined});
 let error;try{await integrateLocalFinancialCase(f.options);}catch(e){error=e;}
 assert.equal(error.message,'OWNED_PROVIDER_CLEANUP_REQUIRED');
 assert.equal(f.calls.filter(c=>c==='wallet-stop').length,1);
 assert.equal(error.publicIntegrationResult.cleanup.walletStopped,true);
 assert.equal(error.publicIntegrationResult.cleanup.containmentComplete,false);
 assert(f.calls.includes('asset-cleanup'));assert(!f.calls.includes('deploy'));
});
test('driver throw without cleanup receipt falls back to owned providers exactly once',async()=>{
 const f=fixture();f.adapters.driver=async()=>{throw Error('EARLY_DRIVER_FAILURE');};
 let error;try{await integrateLocalFinancialCase(f.options);}catch(e){error=e;}
 assert.equal(error.message,'EARLY_DRIVER_FAILURE');
 assert.equal(f.calls.filter(c=>c==='provider-cleanup').length,1);
 assert.equal(error.publicIntegrationResult.cleanup.walletStopped,true);
 assert.equal(error.publicIntegrationResult.cleanup.containmentComplete,false);
 assert(f.calls.includes('asset-cleanup'));assert(!f.calls.includes('deploy'));
});
test('onStage failed retention stops before another transaction',async()=>{
 const f=fixture();f.options.onStage=async()=>({status:'FAILED'});
 await assert.rejects(integrateLocalFinancialCase(f.options),/STAGE_RETENTION/);assert(!f.calls.includes('initialize'));
});
test('source adapters cannot silently switch into production mode',async()=>{
 const f=fixture();delete f.options.sourceTestOnly;await assert.rejects(integrateLocalFinancialCase(f.options),/ADAPTER/);assert(!f.calls.includes('prepare'));
});
test('local RPC uses one bounded nonredirecting request and validates its response identity',async()=>{
 let seen;
 const rpc=createLocalRpc({node:'http://127.0.0.1:9944',deadlineMs:Date.now()+1000,fetchImpl:async(url,options)=>{seen={url,options};const req=JSON.parse(options.body);return new Response(JSON.stringify({jsonrpc:'2.0',id:req.id,result:'0x'+'ab'.repeat(32)}));}});
 assert.equal(await rpc('chain_getFinalizedHead',[]),'0x'+'ab'.repeat(32));
 assert.equal(seen.options.redirect,'error');assert.equal(seen.options.method,'POST');assert(seen.options.signal instanceof AbortSignal);
 await assert.rejects(rpc('author_submitExtrinsic',[]),/RPC_METHOD/);
});
test('local RPC aborts an unresponsive inert request by absolute deadline',async()=>{
 let signal;
 const rpc=createLocalRpc({node:'http://127.0.0.1:9944',deadlineMs:Date.now()+30,fetchImpl:async(_u,o)=>{signal=o.signal;return new Promise(()=>{});}});
 await assert.rejects(rpc('chain_getFinalizedHead',[]),/DEADLINE/);assert.equal(signal.aborted,true);
});
test('swap binds each logical ceiling once and records initialize/swap/close through the actual driver',async()=>{
  const f=fixture();f.options.kind='swap';f.options.limits.grossByLogicalAsset={ASSET_A:20n,ASSET_B:30n};let derived=0;
  f.adapters.loadNativeRuntime=async()=>({ledger:{addressFromKey:()=>first},runtime:{rawTokenType:()=> (++derived===1?color:'99'.repeat(32))}});
  f.adapters.initializeReservations=o=>{assert.deepEqual(o.limits.grossByAsset,{[color]:20n,['99'.repeat(32)]:30n});f.calls.push('initialize-allocation');};
  let error;try{await integrateLocalFinancialCase(f.options);}catch(e){error=e;}
  assert.equal(error.message,'DRIVER_CLEANUP_INCOMPLETE');assert.equal(derived,2);
  assert.deepEqual(f.summaries.map(s=>s.stage),['deploy','initialize','swap','close']);
  assert.deepEqual(error.publicIntegrationResult.assetBindings,{ASSET_A:color,ASSET_B:'99'.repeat(32)});
});
test('asset mutation after a recorded stage stops before the next SDK stage',async()=>{
  const f=fixture(),load=f.adapters.loadAssets;let changed=false;
  f.adapters.loadAssets=async()=>{const loaded=await load();return {...loaded,assertFresh(){if(changed)throw Error('INERT_ASSET_MUTATION');return loaded.assertFresh();}};};
  f.options.onStage=async s=>{changed=true;return {status:'RECORDED',stage:s.stage,txId:s.txId};};
  await assert.rejects(integrateLocalFinancialCase(f.options),/INERT_ASSET_MUTATION/);
  assert(!f.calls.includes('initialize'));assert(f.calls.includes('provider-cleanup'));
});
test('inert contained success is explicitly source-only and never financial acceptance',async()=>{
  const f=fixture(),create=f.adapters.createProviders;
  f.adapters.createProviders=async o=>{const providers=await create(o);return {...providers,cleanup:async()=>({walletStopped:true,pendingOperations:0,containmentComplete:true})};};
  const result=await integrateLocalFinancialCase(f.options);
  assert.equal(result.status,'SOURCE_TEST_ONLY');assert.equal(result.financialAcceptance,false);assert.equal(result.networkAcceptance,false);assert.equal(result.proofAcceptance,false);
});
test('RPC rejects mismatched IDs and oversized bodies without a retry',async()=>{
  for(const body of [JSON.stringify({jsonrpc:'2.0',id:999,result:null}),'x'.repeat(65537)]){
    let calls=0;const rpc=createLocalRpc({node:'http://localhost:9944',deadlineMs:Date.now()+1000,fetchImpl:async()=>{calls++;return new Response(body);}});
    await assert.rejects(rpc('chain_getFinalizedHead',[]),/RPC_RESPONSE|RPC_BODY_LIMIT/);assert.equal(calls,1);
  }
});
test('protocol and reviewed callbacks remain captured after asynchronous preparation starts',async()=>{
  const f=fixture(),prepare=f.adapters.prepareDeployment;let observed=0;
  f.adapters.prepareDeployment=async o=>{
    f.options.expectedProtocolVersion=7;
    f.options.onStage=()=>{throw Error('REPLACED_STAGE_CALLBACK');};
    f.options.now=()=>{throw Error('REPLACED_TIME_CALLBACK');};
    f.options.onEvent=()=>{throw Error('REPLACED_EVENT_CALLBACK');};
    return prepare(o);
  };
  const compare=f.adapters.createComparator;
  f.adapters.createComparator=async o=>{assert.equal(o.expectedProtocolVersion,1000000);return compare(o);};
  const observe=f.adapters.observe;
  f.adapters.observe=async o=>{assert.equal(o.expectedProtocolVersion,1000000);observed++;return observe(o);};
  await assert.rejects(integrateLocalFinancialCase(f.options),/DRIVER_CLEANUP_INCOMPLETE/);
  assert.equal(observed,4);assert.equal(f.summaries.length,4);
});
