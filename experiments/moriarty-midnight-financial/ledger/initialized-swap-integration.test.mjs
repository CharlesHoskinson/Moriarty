import test from 'node:test';
import assert from 'node:assert/strict';
import {integrateLocalFinancialCase, createLocalRpc} from './integrate-local.mjs';
import {runLocalFinancialCase} from './run-local.mjs';
const first='11'.repeat(32), second='22'.repeat(32), address='33'.repeat(32), color='44'.repeat(32);
function fixture() {
  const calls=[], summaries=[];
  const network={networkId:'undeployed',node:'http://127.0.0.1:9944',indexer:'http://127.0.0.1:8088/graphql',indexerWS:'ws://127.0.0.1:8088/ws',proofServer:'http://127.0.0.1:6300'};
  const options={kind:'swap',walletContext:{wallet:{async stop(){calls.push('wallet-stop');}},unshieldedKeystore:{getPublicKey:()=> 'inert-key'},shieldedSecretKeys:{coinPublicKey:'coin',encryptionPublicKey:'enc'}},networkConfig:network,
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
  options.adapters=adapters;return {options,adapters,calls,summaries,getLimits:()=>limits,sdk,driverSdk};
}
import {join} from 'node:path';
import {EXISTING_SWAP,INITIALIZED_SWAP,inspectInitializedSwapBytes} from './continue-initialized-swap.mjs';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {readFileSync} from 'node:fs';
import {PINNED_NM} from './providers.mjs';
import {decodeNativeFinancialTransaction} from './receipt.mjs';
import {validateInitializedSwapPlan,readInitializedSwapInputs} from './continue-swap-plan.mjs';
import {validateLocalLaunchPlan} from './launch-local.mjs';
function closedContinuationPlan(){const D=fileURLToPath(new URL('../../../deliverables/sp05-financial-integration-2026-09-09/',import.meta.url)),original='/home/charl/.local/state/moriarty/sp05-local-loan-20260910-03',existing='/home/charl/.local/state/moriarty/sp05-local-swap-20260910-01';const p={schema:'moriarty.local-financial-launch/1',kind:'swap',build:{receiptPath:'/home/charl/.local/state/moriarty/sp05-full-build-20260909-01/swap-output/build/build-receipt.json',receiptSha256:INITIALIZED_SWAP.buildReceiptSha256,sourceManifestHash:'a29775a104dde9dbc38fdcbfdbc25a31db63beec84e09440f73441a27e9422e6'},networkConfig:{networkId:'undeployed',node:'http://127.0.0.1:19944',indexer:'http://127.0.0.1:18088/api/v4/graphql',indexerWS:'ws://127.0.0.1:18088/api/v4/graphql/ws',proofServer:'http://127.0.0.1:16300'},wallet:{seedFile:'/home/charl/.local/share/moriarty/test-wallets/local-undeployed.seed',stateDirectory:'/home/charl/.local/share/moriarty/test-wallets/hello-world-dedicated-v2',expectedAddress:'mn_addr_undeployed1n2w7v4y79630m5u40rpm6tn0qvnm83vptu9vqcwzqppam7pfrr9sa6q9r9'},roles:{firstAddress:'9a9de6549e2ea2fdd39578c3bd2e6f0327b3c5815f0ac061c20043ddf82918cb',secondAddress:'c1d1141a7f08931d16f3fe4cec1c57d66ab2d11d04e4ab7abb61121ecad5e61e',secretsFile:original+'/roles.json'},privateState:{directory:existing+'/contract-state',passwordFile:'/home/charl/.local/state/moriarty/sp05-local-loan-recovery-20260910-03/password'},networkTag:'e72f7a21a0397844563b4206f887b779ffa0d937c2d1b2339441faa1f08b9846',expectedProtocolVersion:1000000,limits:{allocationId:'synthetic-continuation-01',submissions:2,deadlineMs:Date.now()+60000,dustFee:'100',grossByLogicalAsset:{ASSET_A:'100000',ASSET_B:'0'}},outputDirectory:'/synthetic-new-continuation/run'};
p.existingInitializedSwap={schema:'moriarty.existing-initialized-local-swap/1',deployTransactionFile:join(D,'local-swap-01/run-public/public-transactions',EXISTING_SWAP.transactionHash+'.bin'),deployTransactionHash:EXISTING_SWAP.transactionHash,deployIdentifiers:[...EXISTING_SWAP.identifiers],deployTxId:EXISTING_SWAP.txId,deploySourceResultFile:join(D,'local-swap-01/attempt-result.json'),deploySourceResultSha256:'20b287e40dff3e5a0b84da59fdb1e944362d772206cbb5ff7d4586b0de63ad61',deploySourceAllocationId:'sp05-local-swap-01',initializeTransactionFile:join(D,'local-swap-01/run-public/public-transactions',INITIALIZED_SWAP.transactionHash+'.bin'),initializeTransactionHash:INITIALIZED_SWAP.transactionHash,initializeIdentifiers:[...INITIALIZED_SWAP.identifiers],initializeTxId:INITIALIZED_SWAP.txId,initializeSourceResultFile:join(D,'local-swap-01/attempt-result.json'),initializeSourceResultSha256:'20b287e40dff3e5a0b84da59fdb1e944362d772206cbb5ff7d4586b0de63ad61',initializeSourceAllocationId:'sp05-local-swap-01',contractAddress:INITIALIZED_SWAP.contractAddress,initializedStateSha256:INITIALIZED_SWAP.initializedStateSha256,buildReceiptSha256:p.build.receiptSha256,networkTag:p.networkTag,expectedProtocolVersion:p.expectedProtocolVersion,privateStateDirectory:p.privateState.directory,snapshotDirectory:'/synthetic-new-continuation/snapshot',inspectionDirectory:'/synthetic-new-continuation/inspection'};return p;}

const ledger=await import(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs')));
const nativeBytes=readFileSync(new URL('../../../deliverables/sp05-financial-integration-2026-09-09/local-swap-01/run-public/public-transactions/'+INITIALIZED_SWAP.transactionHash+'.bin',import.meta.url));
const mint=inspectInitializedSwapBytes(nativeBytes,ledger).mintedOutput,assetB='e7d969726b0884ada7bb477283143911dc4df1b18da25a776a433bfaac8646d4',swapBindings=JSON.parse(readFileSync(new URL('../custody/bindings.json',import.meta.url))).swap;
const initializeReceipt={schema:'moriarty.finalized-financial-stage/1',circuitId:'initialize',acceptance:'uncertified-I2-observation',protocolVersion:1000000,contractAddress:INITIALIZED_SWAP.contractAddress,txId:INITIALIZED_SWAP.txId,blockHash:'ab'.repeat(32),blockHeight:20393,finalizedHead:'0x'+'ab'.repeat(32),finalizedHeight:20394,transaction:decodeNativeFinancialTransaction(nativeBytes,ledger)};
function syncedFixture(){const progress=()=>({isConnected:true,isStrictlyComplete:()=>true});return {shielded:{progress:progress()},unshielded:{progress:progress(),availableCoins:[{utxo:{owner:'mn_addr_undeployed1n2w7v4y79630m5u40rpm6tn0qvnm83vptu9vqcwzqppam7pfrr9sa6q9r9',type:mint.type,value:BigInt(mint.value),intentHash:mint.intentHash,outputNo:mint.outputNo}}],pendingCoins:[]},dust:{progress:progress(),state:{pendingDust:[]}}};}
// Transport/comparator/provider adapters synthetic; real retained native initialize output crosses wallet predicate.
function continuationFixture(){
 const f=fixture(),o=f.options,p=closedContinuationPlan();o.continuationPlan=p;
 o.build={...p.build};o.networkConfig={...p.networkConfig};o.networkTag=p.networkTag;o.roles.firstAddress=p.roles.firstAddress;o.roles.secondAddress=p.roles.secondAddress;
 o.limits={...p.limits,dustFee:BigInt(p.limits.dustFee),grossByLogicalAsset:{ASSET_A:100000n,ASSET_B:0n},reservationStatePath:join(p.outputDirectory,'reservations.json')};
 o.privateStateConfig={midnightDbName:p.privateState.directory,privateStateStoreName:'sp05-swap',privateStoragePasswordProvider:()=> 'PRIVATE_TEST_PASSWORD'};
 o.walletContext.unshieldedKeystore.getBech32Address=()=>({toString:()=>p.wallet.expectedAddress});o.walletContext.wallet.waitForSyncedState=async()=>{f.calls.push('continuation-sync');return syncedFixture();};
 const state={syntheticFullState:true},binding={syntheticHistory:true};
 const checked={status:'INITIALIZED_PUBLIC_STATE_VERIFIED',deploymentObservation:{receipt:{txId:EXISTING_SWAP.txId}},initializeObservation:{receipt:initializeReceipt},initializeContractState:state,binding,tip:{timestampMs:1700000000000}};
 o.continuationAdapters={
  publicCheck:async q=>{assert.deepEqual(q,p);f.calls.push('continuation-public');return checked;},
  checkWallet:x=>{assert.equal(x.binding,binding);assert.equal(x.dustCap,100n);f.calls.push('continuation-wallet');return true;},
  preserveStore:async x=>{assert.deepEqual(x,{sourceDirectory:p.privateState.directory,snapshotDirectory:p.existingInitializedSwap.snapshotDirectory,inspectionDirectory:p.existingInitializedSwap.inspectionDirectory,accountId:p.wallet.expectedAddress});f.calls.push('continuation-preserve');return {status:'PRESERVED'};},
  checkPrivate:async x=>{assert.equal(x.contractState,state);assert.equal(x.signingKey,o.deploymentSigningKey);f.calls.push('continuation-private');return {status:'PRIVATE_STATE_CHECKED'};},
 };
 f.adapters.prepareDeployment=()=>{throw Error('PREPARE_FORBIDDEN');};
 f.adapters.loadContractsSdk=async()=>new Proxy({submitCallTx:async(_provider,input)=>{assert.ok(['swap','close'].includes(input.circuitId));assert.equal(input.contractAddress,INITIALIZED_SWAP.contractAddress);f.calls.push(input.circuitId);return {public:{txId:'inert-'+input.circuitId}};}},{get(target,k){if(['deployContract','createUnprovenDeployTx','findDeployedContract'].includes(k))throw Error('DEPLOY_API_FORBIDDEN');return target[k];}});
 f.adapters.loadNativeRuntime=async()=>({ledger:{addressFromKey:()=>p.roles.firstAddress},runtime:{rawTokenType:(_domain,a)=>{assert.equal(a,INITIALIZED_SWAP.contractAddress);f.calls.push('derive-color');return Buffer.from(_domain).toString('hex')===swapBindings.assetADomain?mint.type:assetB;}}});
 f.adapters.initializeReservations=x=>{assert.equal(x.limits.submissions,2);assert.equal(x.limits.grossByAsset[mint.type],100000n);assert.equal(x.limits.grossByAsset[assetB],0n);f.calls.push('initialize-allocation');};
 const create=f.adapters.createProviders;f.adapters.createProviders=async x=>({...await create(x),privateStateProvider:{synthetic:true},getExecutionBinding:()=>({network:p.networkConfig,payerAddress:p.roles.firstAddress,sdkNetworkId:'undeployed'}),cleanup:async()=>{f.calls.push('provider-cleanup');return {walletStopped:true,pendingOperations:0,containmentComplete:true};}});
 return f;
}
test('initialized composition replays history once before store access and only submits swap and close',async()=>{
 const f=continuationFixture(),out=await integrateLocalFinancialCase(f.options);assert.equal(out.status,'SOURCE_TEST_ONLY');
 assert.deepEqual(f.summaries.map(s=>s.stage),['deploy','initialize','swap','close']);
 for(const stage of ['deploy','initialize']){assert.equal(f.calls.filter(c=>c==='compare:'+stage).length,1);assert.equal(f.calls.filter(c=>c==='record:'+stage).length,1);assert.ok(!f.calls.includes('observe:'+stage));assert.ok(f.calls.indexOf('record:'+stage)<f.calls.indexOf('continuation-preserve'));}
 assert.ok(f.calls.indexOf('continuation-preserve')<f.calls.indexOf('providers'));assert.ok(f.calls.indexOf('continuation-private')<f.calls.indexOf('swap'));
 assert.ok(!f.calls.includes('prepare')&&!f.calls.includes('deploy')&&!f.calls.includes('initialize'));assert.equal(f.calls.filter(c=>c==='swap').length,1);assert.equal(f.calls.filter(c=>c==='close').length,1);
 assert.equal(out.networkAcceptance,false);assert.ok(!JSON.stringify(out).includes('PRIVATE_'));
});
for(const gate of ['publicCheck','checkWallet','preserveStore','checkPrivate'])test(`initialized ${gate} failure stops calls and cleans owned resources`,async()=>{
 const f=continuationFixture();f.options.continuationAdapters[gate]=()=>{throw Error('SYNTHETIC_CONTINUATION_GATE');};await assert.rejects(integrateLocalFinancialCase(f.options),/SYNTHETIC_CONTINUATION_GATE/);assert.ok(!f.calls.includes('swap')&&!f.calls.includes('close'));assert.ok(f.calls.includes('wallet-stop')||f.calls.includes('provider-cleanup'));
});
for(const stage of ['deploy','initialize'])test(`failed initialized historical ${stage} comparison prevents snapshot and providers`,async()=>{
 const f=continuationFixture(),create=f.adapters.createComparator;f.adapters.createComparator=async()=>{const c=await create();return {...c,verifyStage:(s,o)=>s===stage?{status:'FAIL'}:c.verifyStage(s,o)};};
 await assert.rejects(integrateLocalFinancialCase(f.options),/FINANCIAL_COMPARISON_REQUIRED_PASS/);assert.ok(!f.calls.includes('continuation-preserve')&&!f.calls.includes('providers')&&!f.calls.includes('swap'));
});
for(const [field,mutate] of [['store',f=>f.options.privateStateConfig.midnightDbName='/other'],['limits',f=>f.options.limits.submissions=3],['role',f=>f.options.roles.secondAddress='aa'.repeat(32)],['build',f=>f.options.build.sourceManifestHash='aa'.repeat(32)]])test(`initialized actual ${field} mismatch stops before setup`,async()=>{
 const f=continuationFixture();mutate(f);await assert.rejects(integrateLocalFinancialCase(f.options),/CONTINUATION_INTEGRATION/);assert.deepEqual(f.calls,['wallet-stop']);
});
test('production rejects continuation adapters; source fixture requires every gate',async()=>{
 const f=continuationFixture();delete f.options.sourceTestOnly;delete f.options.adapters;await assert.rejects(integrateLocalFinancialCase(f.options),/CONTINUATION_ADAPTERS_REQUIRE_SOURCE_TEST/);assert.deepEqual(f.calls,['wallet-stop']);
 for(const key of ['publicCheck','checkWallet','preserveStore','checkPrivate']){const g=continuationFixture();delete g.options.continuationAdapters[key];await assert.rejects(integrateLocalFinancialCase(g.options),/COMPLETE_INERT_CONTINUATION_ADAPTERS_REQUIRED/);assert.deepEqual(g.calls,['wallet-stop']);}
});

for(const [gate,code] of [['publicCheck','CONTINUATION_PUBLIC_RESULT'],['preserveStore','CONTINUATION_STORE_PRESERVATION'],['checkPrivate','CONTINUATION_PRIVATE_RESULT']])test(`initialized ${gate} requires its closed success result`,async()=>{
 const f=continuationFixture();f.options.continuationAdapters[gate]=async()=>({status:'FAILED'});await assert.rejects(integrateLocalFinancialCase(f.options),new RegExp(code));assert.ok(!f.calls.includes('swap'));
});


test('closed swap plan reads both actual public histories from same source result',()=>{const p=closedContinuationPlan();validateInitializedSwapPlan(p.existingInitializedSwap,p);const b=readInitializedSwapInputs(p.existingInitializedSwap,ledger);assert.equal(b.deployBinding.transactionHash,EXISTING_SWAP.transactionHash);assert.equal(b.initializeBinding.transactionHash,INITIALIZED_SWAP.transactionHash);assert.doesNotThrow(()=>validateLocalLaunchPlan(p));});
for(const change of [p=>p.limits.allocationId='sp05-local-swap-01',p=>p.existingInitializedSwap.extra=true,p=>p.existingInitializedSwap.initializeSourceResultSha256='00'.repeat(32),p=>p.existingInitializedLoan={},p=>p.existingDeployment={}])test('closed swap plan rejects consumed allocation and altered/exclusive descriptor',()=>{const p=closedContinuationPlan();change(p);assert.throws(()=>validateLocalLaunchPlan(p));});
test('composed swap plan getters rejected without invocation before setup',async()=>{for(const key of ['kind','existingInitializedSwap']){const f=continuationFixture();let calls=0;Object.defineProperty(f.options.continuationPlan,key,{get(){calls++;throw Error('GETTER_TRAP');}});await assert.rejects(integrateLocalFinancialCase(f.options),/CONTINUATION_INTEGRATION_PLAN/);assert.equal(calls,0);assert.deepEqual(f.calls,['wallet-stop']);}});
test('missing initialized mint stops before preservation and new dispatch',async()=>{const f=continuationFixture();f.options.walletContext.wallet.waitForSyncedState=async()=>{const s=syncedFixture();s.unshielded.availableCoins=[];return s;};await assert.rejects(integrateLocalFinancialCase(f.options),/SWAP_WALLET/);assert.ok(!f.calls.includes('continuation-preserve')&&!f.calls.includes('swap')&&!f.calls.includes('close'));});
for(const [gate,code] of [['publicCheck','SWAP_CONTINUATION_DEPLOY_STATE'],['checkPrivate','SWAP_CONTINUATION_PRIVATE_READ'],['checkWallet','SWAP_CONTINUATION_WALLET_HISTORY']])test('retains closed swap diagnostic '+code,async()=>{const f=continuationFixture();f.options.continuationAdapters[gate]=()=>{throw Error(code);};await assert.rejects(integrateLocalFinancialCase(f.options),e=>{assert.equal(e.publicIntegrationResult.failureCode,code);return true;});});
