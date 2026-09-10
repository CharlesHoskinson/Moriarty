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

const oldLoan={contractAddress:'ba4c808859fc2e4ee6d3d19fa0d812bb9a9c9eb0527161fb91315213bc24a713',transactionHash:'0af6f3ed8960b1a7d1d5e8c204d9e133255e784dd2c5fad1f160c5212d02bd3c',txId:'00959c51e7d62ee9160bf1396ce0ab52f26757a7c5adec669cb083d5a8787d1de9',identifiers:['00b6140ece5793d57c801512e8e7c9e2ec2687e19b1c48a1f56167f2cd1dfc9e66','00959c51e7d62ee9160bf1396ce0ab52f26757a7c5adec669cb083d5a8787d1de9'],buildReceiptSha256:'51ee2d4d60216464a9ace67966ba0ab253699844187aeb5652b46dc6e9ca5bf7'};
function recoveryFixture(){
 const f=fixture(),o=f.options,original='/home/charl/.local/state/moriarty/sp05-local-loan-20260910-03',networkTag='e72f7a21a0397844563b4206f887b779ffa0d937c2d1b2339441faa1f08b9846';
 o.networkTag=networkTag;o.build.receiptSha256=oldLoan.buildReceiptSha256;o.limits.submissions=3;o.limits.reservationStatePath='/inert/recovery-output/reservations.json';
 o.privateStateConfig={midnightDbName:'/inert/recovery-destination',privateStateStoreName:'sp05-loan',privateStoragePasswordProvider:()=> 'synthetic-password'};
 o.walletContext.unshieldedKeystore.getBech32Address=()=>({toString:()=> 'synthetic-wallet'});
 o.walletContext.wallet.waitForSyncedState=async()=>{f.calls.push('recovery-sync');return {syntheticSync:true};};
 const recovery={schema:'moriarty.existing-local-loan/1',transactionFile:'/inert/original.bin',...oldLoan,networkTag,expectedProtocolVersion:1000000,sourceAllocationId:'sp05-local-loan-03',sourceResultFile:'/inert/original-result.json',sourceResultSha256:'bf1d456714ea134ad7e320849b3841088b2a2a352435d75ced65854c48031f54',sourcePrivateStateDirectory:original+'/contract-state',inspectionDirectory:'/inert/recovery-inspection',destinationDirectory:o.privateStateConfig.midnightDbName};
 o.recoveryPlan={kind:'loan',networkTag,networkConfig:{...o.networkConfig},expectedProtocolVersion:1000000,build:{...o.build},roles:{firstAddress:first,secondAddress:second,secretsFile:original+'/roles.json'},wallet:{expectedAddress:'synthetic-wallet'},limits:{...o.limits,dustFee:'100',grossByLogicalAsset:{USD_TEST_ASSET:'20'}},privateState:{directory:o.privateStateConfig.midnightDbName,passwordFile:'/inert/new-password'},outputDirectory:'/inert/recovery-output',existingDeployment:recovery};
 const privateResult={syntheticConstructorResult:true},privateProvider={syntheticPrivateProvider:true};
 o.recoveryAdapters={
  checkDestination:path=>{f.calls.push('recovery-destination');assert.equal(path,o.privateStateConfig.midnightDbName);},
  publicCheck:async plan=>{f.calls.push('recovery-public');assert.deepEqual(plan,o.recoveryPlan);return {binding:{syntheticBinding:true},tip:{timestampMs:1700000000000}};},
  checkWallet:input=>{f.calls.push('recovery-wallet');assert.deepEqual(input,{synced:{syntheticSync:true},binding:{syntheticBinding:true},timestampMs:1700000000000,dustCap:100n});},
  reconstruct:async input=>{f.calls.push('recovery-constructor');assert.equal(input.args.length,6);assert.equal(input.signingKey,o.deploymentSigningKey);return privateResult;},
  inspectStore:async input=>{f.calls.push('recovery-store');assert.deepEqual(input,{sourceDirectory:original+'/contract-state',inspectionDirectory:recovery.inspectionDirectory,accountId:'synthetic-wallet'});},
  restore:async(provider,result)=>{f.calls.push('recovery-restore');assert.equal(provider,privateProvider);assert.equal(result,privateResult);return {status:'RESTORED'};},
 };
 f.adapters.prepareDeployment=()=>{throw Error('PREPARE_FORBIDDEN');};
 f.adapters.loadContractsSdk=async()=>new Proxy({submitCallTx:async(_p,input)=>{f.calls.push(input.circuitId);assert.equal(input.contractAddress,oldLoan.contractAddress);return {public:{txId:'inert-'+input.circuitId}};}},{get(target,key){if(['deployContract','createUnprovenDeployTx','findDeployedContract'].includes(key))throw Error('DEPLOY_API_FORBIDDEN');return target[key];}});
 f.adapters.loadNativeRuntime=async()=>({ledger:{addressFromKey:()=>first},runtime:{rawTokenType:(_domain,a)=>{assert.equal(a,oldLoan.contractAddress);f.calls.push('derive-color');return color;}}});
 const create=f.adapters.createProviders;
 f.adapters.createProviders=async input=>({...await create(input),privateStateProvider:privateProvider,getState:()=>({reservedSubmissions:3,reservedDustFee:10n,reservedGrossByAsset:{[color]:10n},identifiers:[]}),cleanup:async()=>{f.calls.push('provider-cleanup');return {walletStopped:true,pendingOperations:0,containmentComplete:true};}});
 f.adapters.driver=async input=>{f.calls.push('driver');assert.deepEqual(input.existingDeployment,{contractAddress:oldLoan.contractAddress,txId:oldLoan.txId});assert.ok(f.calls.includes('recovery-restore'));return runLocalFinancialCase(input);};
 return f;
}
test('recovery composition restores before real driver, observes historical deploy once and submits only three calls',async()=>{
 const f=recoveryFixture(),out=await integrateLocalFinancialCase(f.options);
 assert.equal(out.status,'SOURCE_TEST_ONLY');assert.equal(out.financialAcceptance,false);assert.equal(out.networkAcceptance,false);assert.equal(out.proofAcceptance,false);
 const order=['recovery-public','recovery-sync','recovery-wallet','recovery-constructor','recovery-store','derive-color','recovery-destination','initialize-allocation','providers','recovery-restore','driver','observe:deploy','compare:deploy','record:deploy','initialize','observe:initialize','compare:initialize','record:initialize','accrue','observe:accrue','compare:accrue','record:accrue','settle','observe:settle','compare:settle','record:settle','provider-cleanup','asset-cleanup'];
 assert.deepEqual(f.calls.filter(x=>x!=='fresh'&&x!=='load-assets'),order);
 assert.deepEqual(out.driver.transactionIds,[oldLoan.txId,'inert-initialize','inert-accrue','inert-settle']);assert.equal(out.driver.operationalState.reservedSubmissions,3);assert.equal(out.comparisons.length,4);assert.equal(out.driver.contractAddress,oldLoan.contractAddress);assert.deepEqual(out.assetBindings,{USD_TEST_ASSET:color});
 assert.ok(!JSON.stringify(out).includes('PRIVATE-DEPLOYMENT-CANARY'));
});
for(const [gate,phase] of [['publicCheck','recovery-public'],['checkWallet','recovery-wallet'],['reconstruct','recovery-constructor'],['inspectStore','recovery-store'],['restore','recovery-restore']])test(`recovery ${gate} failure stops before driver and cleans owned resources`,async()=>{
 const f=recoveryFixture();f.options.recoveryAdapters[gate]=()=>{throw Error('SYNTHETIC_GATE_FAILURE');};
 await assert.rejects(integrateLocalFinancialCase(f.options),e=>{assert.equal(e.message,'SYNTHETIC_GATE_FAILURE');assert.equal(e.publicIntegrationResult.phase,phase);assert.equal(e.publicIntegrationResult.status,'FAILED');assert.equal(e.publicIntegrationResult.comparisons.length,0);return true;});
 assert.ok(!f.calls.includes('driver'));assert.ok(!f.calls.includes('initialize'));assert.ok(!f.calls.includes('observe:deploy'));assert.equal(f.calls.filter(x=>x===(gate==='restore'?'provider-cleanup':'wallet-stop')).length,1);assert.equal(f.calls.filter(x=>x==='asset-cleanup').length,1);
 if(gate!=='restore')assert.ok(!f.calls.includes('initialize-allocation'));
});
test('recovery wallet synchronization rejection prevents constructor, store and driver',async()=>{
 const f=recoveryFixture();f.options.walletContext.wallet.waitForSyncedState=async()=>{throw Error('SYNC_FAILURE');};await assert.rejects(integrateLocalFinancialCase(f.options),/SYNC_FAILURE/);
 for(const stage of ['recovery-constructor','recovery-store','driver','initialize-allocation'])assert.ok(!f.calls.includes(stage));assert.ok(f.calls.includes('wallet-stop'));assert.ok(f.calls.includes('asset-cleanup'));
});
for(const [name,mutate,code] of [
 ['allocation',f=>f.options.limits.allocationId='changed','LIMITS'],
 ['submissions',f=>f.options.limits.submissions=4,'LIMITS'],
 ['dust',f=>f.options.limits.dustFee=101n,'LIMITS'],
 ['gross cap',f=>f.options.limits.grossByLogicalAsset.USD_TEST_ASSET=21n,'LIMITS'],
 ['deadline',f=>f.options.limits.deadlineMs++,'LIMITS'],
 ['journal',f=>f.options.limits.reservationStatePath='/inert/other.json','LIMITS'],
 ['store directory',f=>f.options.privateStateConfig.midnightDbName='/inert/other','STORE'],
 ['store namespace',f=>f.options.privateStateConfig.privateStateStoreName='other','STORE'],
 ['role',f=>f.options.roles.secondAddress='99'.repeat(32),'ROLES'],
 ['wallet',f=>f.options.walletContext.unshieldedKeystore.getBech32Address=()=>({toString:()=> 'changed'}),'ROLES'],
 ['network tag',f=>f.options.networkTag='99'.repeat(32),'BINDING'],
 ['build',f=>f.options.build.sourceManifestHash='99'.repeat(32),'BINDING'],
])test(`recovery rejects actual ${name} differing from closed plan before recovery work`,async()=>{
 const f=recoveryFixture();mutate(f);await assert.rejects(integrateLocalFinancialCase(f.options),new RegExp('RECOVERY_INTEGRATION_'+code));assert.deepEqual(f.calls,['wallet-stop']);
});
test('recovery plan rejects changed deployment identity and unchecked identity shortcut',async()=>{
 const f=recoveryFixture();f.options.recoveryPlan.existingDeployment.contractAddress=address;await assert.rejects(integrateLocalFinancialCase(f.options),/RECOVERY_PLAN_BINDING/);assert.deepEqual(f.calls,['wallet-stop']);
 const shortcut=fixture();shortcut.options.existingDeployment={contractAddress:oldLoan.contractAddress,txId:oldLoan.txId};await assert.rejects(integrateLocalFinancialCase(shortcut.options),/INTEGRATION_USE_CLOSED_RECOVERY_PLAN/);assert.deepEqual(shortcut.calls,['wallet-stop']);
});
test('production integration rejects recovery adapters before loading runtime or touching private state',async()=>{
 const f=recoveryFixture();delete f.options.sourceTestOnly;delete f.options.adapters;
 await assert.rejects(integrateLocalFinancialCase(f.options),/INTEGRATION_RECOVERY_ADAPTERS_REQUIRE_SOURCE_TEST/);assert.deepEqual(f.calls,['wallet-stop']);
});
test('source recovery requires all recovery adapters',async()=>{
 for(const missing of ['publicCheck','reconstruct','inspectStore','restore','checkWallet','checkDestination']){const f=recoveryFixture();delete f.options.recoveryAdapters[missing];await assert.rejects(integrateLocalFinancialCase(f.options),/COMPLETE_INERT_RECOVERY_ADAPTERS_REQUIRED/);assert.deepEqual(f.calls,['wallet-stop']);}
});

test('nonempty recovery destination stops before provider creation and driver',async()=>{
 const f=recoveryFixture();f.options.recoveryAdapters.checkDestination=()=>{throw Error('DESTINATION_NOT_EMPTY');};
 await assert.rejects(integrateLocalFinancialCase(f.options),/DESTINATION_NOT_EMPTY/);
 assert.ok(!f.calls.includes('initialize-allocation'));assert.ok(!f.calls.includes('providers'));assert.ok(!f.calls.includes('driver'));assert.ok(f.calls.includes('wallet-stop'));assert.ok(f.calls.includes('asset-cleanup'));
});

test('recovery rejects deployment identity getter before cloning or executing it',async()=>{
 const f=recoveryFixture();let accessed=0;
 Object.defineProperty(f.options.recoveryPlan.existingDeployment,'txId',{enumerable:true,get(){accessed++;return oldLoan.txId;}});
 await assert.rejects(integrateLocalFinancialCase(f.options),/RECOVERY_PLAN_FIELDS/);assert.equal(accessed,0);assert.deepEqual(f.calls,['wallet-stop']);
});
