import {createLocalRpc} from './financial-rpc.mjs';
export {createLocalRpc};
import {validateStaleLoanPlan,readStaleLoanInputs} from './stale-loan-plan.mjs';
import {validateInitializedSwapPlan,readInitializedSwapInputs} from './continue-swap-plan.mjs';
import {EXISTING_SWAP,INITIALIZED_SWAP,verifyInitializedSwapPrivate,assertInitializedSwapMintedOutput} from './continue-initialized-swap.mjs';
import {verifyInitializedSwapPublic,assertSwapHistoryWallet} from './recover-deployment.mjs';
import {preserveInitializedSwapStore} from './recover-store.mjs';
import {CONTRACT_BALANCE_FAILURE_CODES} from './contract-balances.mjs';
/** Production composition for the fixed local I2 case. No wallet creation or CLI dispatch.
 * Supplying existing handles transfers cleanup responsibility, including preflight failures.
 * Execution still requires a separately reviewed launch with outer process containment.
 */
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {isDeepStrictEqual} from 'node:util';
import {loadProvenFinancialContract} from './proven-assets.mjs';
import {loadFinancialContractsSdk,prepareFinancialDeployment} from './prepare-deployment.mjs';
import {PINNED_NM,loadFinancialSdk,initializeFinancialReservations,createFinancialProviders} from './providers.mjs';
import {observeFinalizedStage,beforeDeadline} from './receipt.mjs';
import {createFinancialComparator} from './financial-comparison.mjs';
import {runLocalFinancialCase} from './run-local.mjs';
import {EXISTING_LOAN,validateExistingLoanPlan,readExistingLoanInputs,verifyExistingLoanPublic,verifyInitializedLoanPublic,reconstructExistingLoan,assertRecoveryWallet,restoreExistingLoanPrivate} from './recover-deployment.mjs';
import {inspectFailedLoanStore,assertEmptyRecoveryStore,preserveInitializedLoanStore} from './recover-store.mjs';
import {validateInitializedLoanPlan,readInitializedLoanInputs} from './continue-loan-plan.mjs';
import {INITIALIZED_LOAN,verifyInitializedLoanPrivate,assertInitializedLoanMintedOutput} from './continue-initialized-loan.mjs';
import {assertSwapInitializedWallet,SWAP_WALLET_FAILURE_CODES} from './swap-wallet.mjs';

const requireThat=(condition,message)=>{if(!condition)throw Error(message);};

// These are the original, already admitted allocations for the fixed recovery
// histories. No allowance is created from an observed debit or transferred to
// new calls. The existing public recovery gates separately bind the exact txs.
const historicalFeeSources={
 'local-execution-04':['b51f125bfb3769131a994d21938d466e95562bb77951c2a3a13ad210daedadbc','resource-proposal-02.json','ee0915b4c44e6fd45a753776e42c6356e65ab0ff34db410a07b3f216db50b292'],
 'local-recovery-03':['771104d9703efe4a3154e46a20c7d8dab2e78eb38061be50f4e23d85b66a85d3','resource-proposal-01.json','4c37ee8953ed9faf150497370a9729aa01091db33e5e33d0dfefa57d283ca50e'],
 'local-continuation-02':['75d3412bda16c2dd6ebcd9014915fb0f39d6fde6b32c5df66338e97f5d45148a','resource-proposal-01.json','efb14dd15da433393e022fed265890642700800b223b8a2557019e0d12fb9acd'],
 'local-swap-01':['65899327525fbfb4bc412ab9962e559a37644c10bbd7d58211c46342bc042dca','resource-proposal-01.json','6c9fb3df909cb0432e2463df6433e2d5678d8070da401b30287f0ec54b2295fe']
};
function historicalFeeAllocation(name,stages){
 const [admissionHash,proposalFile,proposalHash]=historicalFeeSources[name];
 const root=new URL('../../../deliverables/sp05-financial-integration-2026-09-09/'+name+'/',import.meta.url);
 const read=(file,digest)=>{const raw=readFileSync(new URL(file,root));requireThat(createHash('sha256').update(raw).digest('hex')===digest,'HISTORICAL_FEE_ADMISSION_PIN');return JSON.parse(raw);};
 const admission=read('execution-admission.json',admissionHash),proposal=read(proposalFile,proposalHash);
 requireThat(admission.status==='ADMITTED'&&admission.proposalSha256===proposalHash&&proposal.dustFeeSpeck==='2000000000000000','HISTORICAL_FEE_ADMISSION');
 return {stages,dustFeeCap:BigInt(proposal.dustFeeSpeck)};
}

const publicIntegrationFailureCodes=new Set([
 'NATIVE_FEE_CAP_REQUIRED','NATIVE_FEE_CAP_EXCEEDED','HISTORICAL_FEE_ADMISSION_PIN','HISTORICAL_FEE_ADMISSION',
 'COMPLETE_INERT_STALE_ADAPTERS_REQUIRED','INTEGRATION_STALE_ADAPTERS_REQUIRE_SOURCE_TEST','STALE_ADVERSE_RESULT','STALE_HANDLE_CLEANUP','STALE_HISTORY_ORDER','STALE_INTEGRATION_BINDING','STALE_INTEGRATION_EXCLUSIVE','STALE_INTEGRATION_LIMITS','STALE_INTEGRATION_PLAN_REQUIRED','STALE_INTEGRATION_RETENTION','STALE_INTEGRATION_ROLES','STALE_INTEGRATION_STORE','STALE_PREPARATION_LATE','STALE_PUBLIC_CURRENT','STALE_PUBLIC_GENESIS','STALE_PUBLIC_HISTORY','STALE_PUBLIC_RESULT','STALE_STORE_PRESERVATION',
 'SWAP_CONTINUATION_DEPLOY_STATE','SWAP_CONTINUATION_NATIVE_HASH','SWAP_CONTINUATION_NATIVE_IDENTIFIERS','SWAP_CONTINUATION_NATIVE_ACTION','SWAP_CONTINUATION_NATIVE_INPUTS','SWAP_CONTINUATION_NATIVE_DUST','SWAP_CONTINUATION_NATIVE_OUTPUT','SWAP_CONTINUATION_NATIVE_DEPLOY_STATE','SWAP_CONTINUATION_STATE_TYPE','SWAP_CONTINUATION_STATE_MISMATCH','SWAP_CONTINUATION_STATE_CANONICAL','SWAP_CONTINUATION_STATE_BALANCES','SWAP_CONTINUATION_STATE_AUTHORITY','SWAP_CONTINUATION_SIGNING_AUTHORITY','SWAP_CONTINUATION_PRIVATE_PROVIDER','SWAP_CONTINUATION_PRIVATE_READ','SWAP_CONTINUATION_PRIVATE_STATE','SWAP_CONTINUATION_PRIVATE_KEY','SWAP_CONTINUATION_WALLET_BINDING','SWAP_CONTINUATION_WALLET_HISTORY','CONTINUATION_INTEGRATION_PLAN','INVALID_INITIALIZED_SWAP',
 ...SWAP_WALLET_FAILURE_CODES,...CONTRACT_BALANCE_FAILURE_CODES,'AMOUNT_CONTRACT_BALANCES',
 'INTEGRATION_ADAPTERS_REQUIRE_SOURCE_TEST','INTEGRATION_CONTINUATION_ADAPTERS_REQUIRE_SOURCE_TEST','CONTINUATION_PUBLIC_RESULT','CONTINUATION_STORE_PRESERVATION','CONTINUATION_PRIVATE_RESULT',
 'INITIALIZED_PRIVATE_PROVIDER','INITIALIZED_PRIVATE_READ','INITIALIZED_PRIVATE_STATE','INITIALIZED_PRIVATE_KEY','INITIALIZED_SIGNING_AUTHORITY','INITIALIZED_STATE_TYPE','INITIALIZED_STATE_MISMATCH','INITIALIZED_STATE_CANONICAL','INITIALIZED_WALLET_BINDING','INITIALIZED_WALLET_COINS','INITIALIZED_WALLET_AVAILABLE','INITIALIZED_WALLET_OUTPUT','INITIALIZED_WALLET_HISTORY',
 'RECOVERY_WALLET_BINDING','RECOVERY_WALLET_TIME_CAP','RECOVERY_WALLET_SYNC','RECOVERY_WALLET_PENDING','RECOVERY_WALLET_COIN','RECOVERY_WALLET_SPENT_INPUT','RECOVERY_WALLET_DUST_CODEC','RECOVERY_WALLET_SPENT_DUST','RECOVERY_WALLET_REGISTERED_DUST','RECOVERY_WALLET_DUST_CAP',
 'RECOVERY_STORE_INITIALIZED_NAMESPACE','RECOVERY_STORE_INITIALIZED_CIPHERTEXT','RECOVERY_STORE_METADATA','RECOVERY_STORE_DESTINATION_EXISTS','RECOVERY_STORE_COPY_CHANGED','RECOVERY_STORE_ACCOUNT','RECOVERY_STORE_ABSOLUTE_PATH','RECOVERY_STORE_OVERLAP','RECOVERY_STORE_LEVEL_PATH_PIN','RECOVERY_STORE_LEVEL_PIN','RECOVERY_STORE_FILE_CHANGED','RECOVERY_STORE_DIRECTORY_PRIVATE','RECOVERY_STORE_FILE_PRIVATE','RECOVERY_STORE_DIRECTORY_SYMLINK','RECOVERY_STORE_BYTE_BOUND','RECOVERY_STORE_FILE_BOUND','RECOVERY_STORE_EMPTY','RECOVERY_STORE_SOURCE_CHANGED','RECOVERY_STORE_SNAPSHOT_CHANGED','RECOVERY_STORE_PRESERVATION_FAILED','RECOVERY_STORE_ENTRY_BOUND','RECOVERY_STORE_ENTRY_ENCODING','RECOVERY_STORE_CLOSE_FAILED',
 'NOT_FINALIZED','FINALITY_REGRESSION','FINALITY_CANONICAL_MISMATCH','NONCANONICAL_FINALIZED_BLOCK','NONCANONICAL_BLOCK','INDEXED_INPUTS_MISMATCH','INDEXED_OUTPUTS_MISMATCH','RPC_DEADLINE','OBSERVATION_TIMEOUT_UNKNOWN','FINANCIAL_COMPARISON_REQUIRED_PASS','STAGE_RETENTION_REQUIRED'
]);
export function validatePublicIntegrationFailureCode(code){requireThat(code==='UNCLASSIFIED_INTEGRATION_FAILURE'||publicIntegrationFailureCodes.has(code),'INVALID_PUBLIC_INTEGRATION_FAILURE');return code;}
function integrationFailureCode(error){const message=Object.getOwnPropertyDescriptor(error,'message');return message&&Object.hasOwn(message,'value')&&publicIntegrationFailureCodes.has(message.value)?message.value:'UNCLASSIFIED_INTEGRATION_FAILURE';}
const hash=bytes=>createHash('sha256').update(bytes).digest('hex');
const bytes=hex=>{requireThat(typeof hex==='string'&&/^[a-f0-9]{64}$/.test(hex),'INTEGRATION_PUBLIC_BYTES');return Uint8Array.from(Buffer.from(hex,'hex'));};
const PINS={
  "@midnight-ntwrk/midnight-js-protocol": {
    "version": "4.1.1",
    "packageSha256": "bdfe30f046627f364fd057fdf4dd756655fb4a4309722b64bb3e46484bf9effd",
    "entries": {
      "dist/ledger.mjs": "4db121cdee21bb88cecca5dc1433c6d30cc12c1ac1f4c5e40a606440e08ac840"
    }
  },
  "@midnight-ntwrk/ledger-v8": {
    "version": "8.1.0",
    "packageSha256": "52266d4e4ffa3dcd46faed72c1ffc0c7f4dd08bf01004068f8912d285b6b0536",
    "entries": {
      "midnight_ledger_wasm_fs.js": "125acbc6327afcf5540e70fe2a42de58af909695fc284a0e663e20a3666d296c",
      "midnight_ledger_wasm_bg.js": "0fcb6f30ec4c0f894917d516db627d78de5630377bf7918bac08466b894cb716",
      "midnight_ledger_wasm_bg.wasm": "88ff7c7c47c30138ee8f3a0c8bcbf12dbdcdeba591edebfb955c30d732ea7638"
    }
  },
  "@midnight-ntwrk/compact-runtime": {
    "version": "0.16.0",
    "packageSha256": "ac4f818510afca0d17758b4c38af613f7b1a489aec96633548d0d361683f124c",
    "entries": {
      "dist/index.js": "c55a8ab3e7533b3fa6e27b66ab742c783d912d5a00d6ecc23d9f6142ddb0ecde"
    }
  }
};
async function loadNativeRuntime(){
  for(const [name,pin] of Object.entries(PINS)){
    const root=join(PINNED_NM,name);
    requireThat(hash(readFileSync(join(root,'package.json')))===pin.packageSha256,'INTEGRATION_RUNTIME_PACKAGE_PIN');
    requireThat(JSON.parse(readFileSync(join(root,'package.json'),'utf8')).version===pin.version,'INTEGRATION_RUNTIME_VERSION');
    for(const [entry,digest] of Object.entries(pin.entries))requireThat(hash(readFileSync(join(root,entry)))===digest,'INTEGRATION_RUNTIME_ENTRY_PIN');
  }
  const ledger=await import(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs')).href);
  const runtime=await import(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/compact-runtime/dist/index.js')).href);
  return {ledger,runtime};
}
function localEndpoint(value,protocol){
  const url=new URL(value);
  requireThat(url.protocol===protocol&&['127.0.0.1','localhost','[::1]'].includes(url.hostname)&&!url.username&&!url.password&&!url.hash,'INTEGRATION_LOCAL_ENDPOINT');
  return url.href;
}
function validDeadline(value){requireThat(Number.isSafeInteger(value)&&value>Date.now(),'INTEGRATION_DEADLINE');}
const realDependencies={loadAssets:loadProvenFinancialContract,loadContractsSdk:loadFinancialContractsSdk,loadProviderSdk:loadFinancialSdk,loadNativeRuntime,prepareDeployment:prepareFinancialDeployment,initializeReservations:initializeFinancialReservations,createProviders:createFinancialProviders,createComparator:createFinancialComparator,observe:observeFinalizedStage,driver:runLocalFinancialCase,fetch:globalThis.fetch};
/** Fixed read-only public gate. No seed, wallet, new private files or store writes.
 * The launcher invokes this before reading wallet material; integration repeats it.
 */
export async function preflightLocalRecovery(plan){
  const recovery=validateExistingLoanPlan(plan.existingDeployment,plan),deadlineMs=plan.limits.deadlineMs;
  validDeadline(deadlineMs);let loaded;
  try{
    const {ledger}=await loadNativeRuntime(),sdk=await loadFinancialSdk();
    requireThat(plan.networkConfig.networkId==='undeployed','RECOVERY_PUBLIC_NETWORK');sdk.setNetworkId('undeployed');
    for(const [key,protocol] of [['node','http:'],['indexer','http:'],['indexerWS','ws:']])localEndpoint(plan.networkConfig[key],protocol);
    const {raw}=readExistingLoanInputs(recovery,ledger);
    loaded=await loadProvenFinancialContract({case:'loan',...plan.build});
    const rpc=(method,params,requestDeadline=deadlineMs)=>createLocalRpc({node:plan.networkConfig.node,deadlineMs:Math.min(deadlineMs,requestDeadline)})(method,params);
    requireThat(await rpc('chain_getBlockHash',[0])==='0x'+recovery.networkTag,'RECOVERY_PUBLIC_GENESIS');
    const {waitForLocalTip}=await import('./local-tip.mjs');
    const readCurrentTip=(requestDeadline=deadlineMs)=>waitForLocalTip({node:plan.networkConfig.node,indexer:plan.networkConfig.indexer,deadlineMs:Math.min(deadlineMs,requestDeadline),exactFinality:true});
    const tip=await readCurrentTip();
    const provider=sdk.indexerPublicDataProvider(plan.networkConfig.indexer,plan.networkConfig.indexerWS);
    const result=await verifyExistingLoanPublic({raw,ledger,provider,rpc,decodeState:loaded.decodeState,deadlineMs,expectedProtocolVersion:plan.expectedProtocolVersion,tip,readCurrentTip});
    loaded.assertFresh();validDeadline(deadlineMs);return result;
  }finally{if(loaded)await loaded.cleanup();}
}
/** Same public-only preflight for the one already initialized loan; no private reads. */
export async function preflightLocalInitializedLoan(plan){
 const continuation=validateInitializedLoanPlan(plan.existingInitializedLoan,plan),deadlineMs=plan.limits.deadlineMs;validDeadline(deadlineMs);let loaded;
 try{
  const {ledger}=await loadNativeRuntime(),sdk=await loadFinancialSdk();
  requireThat(plan.networkConfig.networkId==='undeployed','INITIALIZED_PUBLIC_NETWORK');sdk.setNetworkId('undeployed');
  for(const [key,protocol] of [['node','http:'],['indexer','http:'],['indexerWS','ws:']])localEndpoint(plan.networkConfig[key],protocol);
  const {deployRaw,initializeRaw}=readInitializedLoanInputs(continuation,ledger);
  loaded=await loadProvenFinancialContract({case:'loan',...plan.build});
  const rpc=(method,params,requestDeadline=deadlineMs)=>createLocalRpc({node:plan.networkConfig.node,deadlineMs:Math.min(deadlineMs,requestDeadline)})(method,params);
  requireThat(await rpc('chain_getBlockHash',[0])==='0x'+continuation.networkTag,'INITIALIZED_PUBLIC_GENESIS');
  const {waitForLocalTip}=await import('./local-tip.mjs');
  const readCurrentTip=(requestDeadline=deadlineMs)=>waitForLocalTip({node:plan.networkConfig.node,indexer:plan.networkConfig.indexer,deadlineMs:Math.min(deadlineMs,requestDeadline),exactFinality:true});
  const tip=await readCurrentTip(),provider=sdk.indexerPublicDataProvider(plan.networkConfig.indexer,plan.networkConfig.indexerWS);
  const result=await verifyInitializedLoanPublic({raw:deployRaw,rawInitialize:initializeRaw,ledger,provider,rpc,decodeState:loaded.decodeState,deadlineMs,expectedProtocolVersion:plan.expectedProtocolVersion,tip,readCurrentTip});
  loaded.assertFresh();validDeadline(deadlineMs);return result;
 }finally{if(loaded)await loaded.cleanup();}
}
export async function preflightLocalInitializedSwap(plan){
 const continuation=validateInitializedSwapPlan(plan.existingInitializedSwap,plan),deadlineMs=plan.limits.deadlineMs;validDeadline(deadlineMs);let loaded;
 try{
  const {ledger}=await loadNativeRuntime(),sdk=await loadFinancialSdk();
  requireThat(plan.networkConfig.networkId==='undeployed','INITIALIZED_PUBLIC_NETWORK');sdk.setNetworkId('undeployed');
  for(const [key,protocol] of [['node','http:'],['indexer','http:'],['indexerWS','ws:']])localEndpoint(plan.networkConfig[key],protocol);
  const {deployRaw,initializeRaw}=readInitializedSwapInputs(continuation,ledger);
  loaded=await loadProvenFinancialContract({case:'swap',...plan.build});
  const rpc=(method,params,requestDeadline=deadlineMs)=>createLocalRpc({node:plan.networkConfig.node,deadlineMs:Math.min(deadlineMs,requestDeadline)})(method,params);
  requireThat(await rpc('chain_getBlockHash',[0])==='0x'+continuation.networkTag,'INITIALIZED_PUBLIC_GENESIS');
  const {waitForLocalTip}=await import('./local-tip.mjs');
  const readCurrentTip=(requestDeadline=deadlineMs)=>waitForLocalTip({node:plan.networkConfig.node,indexer:plan.networkConfig.indexer,deadlineMs:Math.min(deadlineMs,requestDeadline),exactFinality:true});
  const tip=await readCurrentTip(),provider=sdk.indexerPublicDataProvider(plan.networkConfig.indexer,plan.networkConfig.indexerWS);
  const result=await verifyInitializedSwapPublic({raw:deployRaw,rawInitialize:initializeRaw,ledger,provider,rpc,decodeState:loaded.decodeState,deadlineMs,expectedProtocolVersion:plan.expectedProtocolVersion,tip,readCurrentTip});
  loaded.assertFresh();validDeadline(deadlineMs);return result;
 }finally{if(loaded)await loaded.cleanup();}
}
/** Fixed public-only settled-loan gate; no old positive calls are submitted. */
export async function preflightLocalStaleLoan(plan){
 const {captureFinalizedFinancialState}=await import('./finalized-financial-state.mjs');
 const d=validateStaleLoanPlan(plan.existingStaleLoan,plan),deadlineMs=plan.limits.deadlineMs;validDeadline(deadlineMs);let loaded;
 const within=async fn=>{validDeadline(deadlineMs);const result=await beforeDeadline(fn,deadlineMs);validDeadline(deadlineMs);return result;};
 try{
  const {ledger}=await within(loadNativeRuntime),sdk=await within(loadFinancialSdk);sdk.setNetworkId('undeployed');
  const inputs=readStaleLoanInputs(d,ledger);loaded=await within(()=>loadProvenFinancialContract({case:'loan',...plan.build}));
  const rpc=(method,params,requestDeadline=deadlineMs)=>createLocalRpc({node:plan.networkConfig.node,deadlineMs:Math.min(deadlineMs,requestDeadline)})(method,params);
  requireThat(await within(()=>rpc('chain_getBlockHash',[0]))==='0x'+d.networkTag,'STALE_PUBLIC_GENESIS');
  const {waitForLocalTip}=await import('./local-tip.mjs');let tip=await within(()=>waitForLocalTip({node:plan.networkConfig.node,indexer:plan.networkConfig.indexer,deadlineMs,exactFinality:true}));
  const provider=sdk.indexerPublicDataProvider(plan.networkConfig.indexer,plan.networkConfig.indexerWS),history=[],oldDustNullifiers=[],spentUnshieldedInputs=[];
  for(const entry of inputs.history){
   const observation=await within(()=>observeFinalizedStage({provider,rpc,ledger,txId:entry.txId,contractAddress:d.contractAddress,circuitId:entry.stage,decodeState:loaded.decodeState,deadlineMs,expectedProtocolVersion:d.expectedProtocolVersion}));
   requireThat(observation.receipt.transaction.rawSha256===entry.transaction.rawSha256&&observation.receipt.transaction.transactionHash===entry.transaction.transactionHash,'STALE_PUBLIC_HISTORY');
   const tx=ledger.Transaction.deserialize('signature','proof','binding',entry.raw);
   oldDustNullifiers.push(...[...tx.intents.values()].flatMap(i=>i.dustActions?.spends??[]).map(x=>x.oldNullifier));
   spentUnshieldedInputs.push(...entry.transaction.inputs.map(({intentHash,outputNo})=>({intentHash,outputNo})));
   history.push({stage:entry.stage,observation});
  }
  const current=await within(()=>captureFinalizedFinancialState({contractAddress:d.contractAddress,rpc,loadedContract:loaded,deadlineMs}));
  requireThat(current.stateSha256===d.settledStateSha256&&current.blockHeight>=d.minimumCurrentBlockHeight,'STALE_PUBLIC_CURRENT');
  tip=await within(()=>waitForLocalTip({node:plan.networkConfig.node,indexer:plan.networkConfig.indexer,deadlineMs,exactFinality:true}));
  loaded.assertFresh();validDeadline(deadlineMs);
  return {status:'SETTLED_LOAN_PUBLIC_VERIFIED',history,current,tip,binding:{transactionHash:EXISTING_LOAN.transactionHash,oldDustNullifiers,spentUnshieldedInputs},scope:'Exact canonical historical native observations and fresh settled state; no submission authority'};
 }finally{if(loaded)loaded.cleanup();validDeadline(deadlineMs);}
}
const realStaleDependencies={publicCheck:preflightLocalStaleLoan,preserveStore:preserveInitializedLoanStore,checkWallet:assertRecoveryWallet,prepare:async o=>(await import('./stale-loan-rejection.mjs')).prepareStaleLoanAccrue(o),submit:async(h,o)=>(await import('./stale-loan-rejection.mjs')).submitPreparedStaleLoan(h,o),close:async h=>(await import('./stale-loan-rejection.mjs')).closeStaleLoanPreparation(h)};
const realContinuationDependencies={publicCheck:preflightLocalInitializedLoan,preserveStore:preserveInitializedLoanStore,checkPrivate:verifyInitializedLoanPrivate,
 checkWallet({synced,binding,timestampMs,dustCap}){
  requireThat(binding?.deployment?.transactionHash===EXISTING_LOAN.transactionHash&&binding?.initialize?.transactionHash===INITIALIZED_LOAN.transactionHash,'INITIALIZED_WALLET_HISTORY');
  assertRecoveryWallet({synced,binding:{...binding.deployment,oldDustNullifiers:binding.oldDustNullifiers,spentUnshieldedInputs:binding.spentUnshieldedInputs},timestampMs,dustCap});
  return assertInitializedLoanMintedOutput({synced,binding:binding.initialize});
 }};
const realSwapContinuationDependencies={publicCheck:preflightLocalInitializedSwap,preserveStore:preserveInitializedSwapStore,checkPrivate:verifyInitializedSwapPrivate,checkWallet:assertSwapHistoryWallet};
const realRecoveryDependencies={publicCheck:preflightLocalRecovery,reconstruct:reconstructExistingLoan,inspectStore:inspectFailedLoanStore,checkDestination:assertEmptyRecoveryStore,restore:restoreExistingLoanPrivate,checkWallet:assertRecoveryWallet};
async function stopWallet(wallet){
  if(typeof wallet?.stop!=='function')return {walletStopped:false,pendingOperations:null,containmentComplete:false};
  let timer;try{await Promise.race([Promise.resolve().then(()=>wallet.stop()),new Promise((_,reject)=>{timer=setTimeout(()=>reject(Error('WALLET_CLEANUP_TIMEOUT')),5000);})]);return {walletStopped:true,pendingOperations:0,containmentComplete:false};}
  catch{return {walletStopped:false,pendingOperations:null,containmentComplete:false};}finally{clearTimeout(timer);}
}
/**
 * limits.grossByLogicalAsset uses USD_TEST_ASSET (loan), ASSET_A/ASSET_B (swap).
 * onStage must durably retain the closed comparison and return {status:'RECORDED',stage,txId}.
 * Explicit complete inert adapters are permitted only with sourceTestOnly:true;
 * those invocations can never return execution/network/proof acceptance.
 */
export async function integrateLocalFinancialCase(options){
  let loaded,providers,driverStarted=false,driverResult,financialComparison,failure,cleanup,phase='preflight',adverse,staleHandle,closeStaleHandle;
  const summaries=[],pending=new Set();
  const wallet=options?.walletContext?.wallet;
  let sourceTestOnly=false,limits,assetBindings,buildBinding;
  try{
    requireThat(options&&typeof wallet?.stop==='function','EXISTING_WALLET_CLEANUP_REQUIRED');
    sourceTestOnly=options.sourceTestOnly===true;
    requireThat(options.adapters===undefined||sourceTestOnly,'INTEGRATION_ADAPTERS_REQUIRE_SOURCE_TEST');
    requireThat(!sourceTestOnly||options.adapters,'COMPLETE_INERT_ADAPTERS_REQUIRED');
    const deps=sourceTestOnly?options.adapters:realDependencies;
    for(const key of Object.keys(realDependencies))requireThat(typeof deps[key]==='function','COMPLETE_INERT_ADAPTERS_REQUIRED');
    requireThat(options.recoveryAdapters===undefined||sourceTestOnly,'INTEGRATION_RECOVERY_ADAPTERS_REQUIRE_SOURCE_TEST');
    requireThat(options.existingDeployment===undefined&&options.initializedLoan===undefined&&options.initializedSwap===undefined&&options.existingStaleLoan===undefined,'INTEGRATION_USE_CLOSED_RECOVERY_PLAN');
    requireThat(options.continuationAdapters===undefined||sourceTestOnly,'INTEGRATION_CONTINUATION_ADAPTERS_REQUIRE_SOURCE_TEST');
    requireThat(options.recoveryPlan===undefined||options.continuationPlan===undefined,'INTEGRATION_EXCLUSIVE_CONTINUATION');
    requireThat(options.staleLoanAdapters===undefined||sourceTestOnly,'INTEGRATION_STALE_ADAPTERS_REQUIRE_SOURCE_TEST');
    if(options.staleLoanPlan!==undefined)validateStaleLoanPlan(options.staleLoanPlan.existingStaleLoan,options.staleLoanPlan);
    const stalePlan=options.staleLoanPlan===undefined?undefined:structuredClone(options.staleLoanPlan);
    const stale=stalePlan?validateStaleLoanPlan(stalePlan.existingStaleLoan,stalePlan):undefined;
    requireThat(!stale||(options.recoveryPlan===undefined&&options.continuationPlan===undefined),'STALE_INTEGRATION_EXCLUSIVE');
    const staleDeps=sourceTestOnly?options.staleLoanAdapters:realStaleDependencies;
    if(stale)for(const key of Object.keys(realStaleDependencies))requireThat(typeof staleDeps?.[key]==='function','COMPLETE_INERT_STALE_ADAPTERS_REQUIRED');
    else requireThat(options.staleLoanAdapters===undefined,'STALE_INTEGRATION_PLAN_REQUIRED');
    if(stale)closeStaleHandle=staleDeps.close;
    const cp=options.continuationPlan;
    requireThat(cp===undefined||(cp!==null&&typeof cp==='object'&&Object.getPrototypeOf(cp)===Object.prototype),'CONTINUATION_INTEGRATION_PLAN');
    const kd=cp===undefined?undefined:Object.getOwnPropertyDescriptor(cp,'kind');
    requireThat(cp===undefined||(kd&&Object.hasOwn(kd,'value')),'CONTINUATION_INTEGRATION_PLAN');
    const swapContinuation=kd?.value==='swap';
    const continuationValidator=swapContinuation?validateInitializedSwapPlan:validateInitializedLoanPlan;
    const continuationField=swapContinuation?'existingInitializedSwap':'existingInitializedLoan';
    const historyDeploy=swapContinuation?EXISTING_SWAP:EXISTING_LOAN,historyInitialize=swapContinuation?INITIALIZED_SWAP:INITIALIZED_LOAN;
    if(cp!==undefined){const dd=Object.getOwnPropertyDescriptor(cp,continuationField);requireThat(dd&&Object.hasOwn(dd,'value'),'CONTINUATION_INTEGRATION_PLAN');continuationValidator(dd.value,cp);}
    const continuationPlan=options.continuationPlan===undefined?undefined:structuredClone(options.continuationPlan);
    const continuation=continuationPlan===undefined?undefined:continuationValidator(continuationPlan[continuationField],continuationPlan);
    const continuationDeps=sourceTestOnly?options.continuationAdapters:(swapContinuation?realSwapContinuationDependencies:realContinuationDependencies);
    if(continuation)for(const key of Object.keys(realContinuationDependencies))requireThat(typeof continuationDeps?.[key]==='function','COMPLETE_INERT_CONTINUATION_ADAPTERS_REQUIRED');
    else requireThat(options.continuationAdapters===undefined,'INTEGRATION_CONTINUATION_PLAN_REQUIRED');
    if(options.recoveryPlan!==undefined)validateExistingLoanPlan(options.recoveryPlan.existingDeployment,options.recoveryPlan);
    const recoveryPlan=options.recoveryPlan===undefined?undefined:structuredClone(options.recoveryPlan);
    const recovery=recoveryPlan===undefined?undefined:validateExistingLoanPlan(recoveryPlan.existingDeployment,recoveryPlan);
    const recoveryDeps=sourceTestOnly?options.recoveryAdapters:realRecoveryDependencies;
    if(recovery)for(const key of Object.keys(realRecoveryDependencies))requireThat(typeof recoveryDeps?.[key]==='function','COMPLETE_INERT_RECOVERY_ADAPTERS_REQUIRED');
    else requireThat(options.recoveryAdapters===undefined,'INTEGRATION_RECOVERY_PLAN_REQUIRED');
    const {kind}=options;requireThat(kind==='loan'||kind==='swap','INTEGRATION_KIND');
    requireThat(options.build&&Object.keys(options.build).sort().join(',')==='receiptPath,receiptSha256,sourceManifestHash','BUILD_RECEIPT_BINDING_REQUIRED');
    bytes(options.build.receiptSha256);bytes(options.build.sourceManifestHash);
    buildBinding=Object.freeze({...options.build});
    requireThat(Number.isSafeInteger(options.expectedProtocolVersion)&&options.expectedProtocolVersion>=0,'EXPECTED_PROTOCOL_VERSION_REQUIRED');
    requireThat(typeof options.onStage==='function'&&typeof options.onEvent==='function'&&typeof options.now==='function','REVIEWED_STAGE_EVENT_TIME_CALLBACKS_REQUIRED');
    const expectedProtocolVersion=options.expectedProtocolVersion,onStage=options.onStage,onEvent=options.onEvent,now=options.now;
    const network=Object.freeze({...options.networkConfig});requireThat(network.networkId==='undeployed','INTEGRATION_LOCAL_NETWORK');
    for(const [key,protocol] of [['node','http:'],['indexer','http:'],['indexerWS','ws:'],['proofServer','http:']])localEndpoint(network[key],protocol);
    requireThat(options.roles&&Object.keys(options.roles).sort().join(',')==='firstAddress,firstSecret,secondAddress,secondSecret','ROLE_BINDING_FIELDS');
    const roles={...options.roles,firstSecret:Uint8Array.from(options.roles?.firstSecret??[]),secondSecret:Uint8Array.from(options.roles?.secondSecret??[])};
    requireThat(roles.firstSecret.length===32&&roles.secondSecret.length===32,'ROLE_SECRETS_REQUIRED');
    bytes(roles.firstAddress);bytes(roles.secondAddress);requireThat(roles.firstAddress!==roles.secondAddress,'DISTINCT_PARTICIPANTS_REQUIRED');bytes(options.networkTag);
    requireThat(options.deploymentSigningKey!==undefined,'EXISTING_DEPLOYMENT_SIGNING_KEY_REQUIRED');
    const logicalNames=kind==='loan'?['USD_TEST_ASSET']:['ASSET_A','ASSET_B'];
    limits={...options.limits};const logical={...limits.grossByLogicalAsset};
    requireThat(Object.keys(logical).sort().join('|')===logicalNames.sort().join('|'),'LOGICAL_ASSET_LIMITS');
    for(const cap of Object.values(logical))requireThat(typeof cap==='bigint'&&cap>=0n,'LOGICAL_ASSET_CAP');
    requireThat(!Object.hasOwn(limits,'grossByAsset'),'ALREADY_BOUND_ASSET_LIMITS_FORBIDDEN');
    validDeadline(limits.deadlineMs);
    requireThat(Number.isSafeInteger(limits.submissions)&&limits.submissions>0&&typeof limits.dustFee==='bigint'&&limits.dustFee>=0n,'FINANCIAL_LIMITS');
    const checkDeadline=()=>validDeadline(limits.deadlineMs);
    if(recovery){
      requireThat(kind===recoveryPlan.kind&&isDeepStrictEqual(buildBinding,recoveryPlan.build)&&isDeepStrictEqual(network,recoveryPlan.networkConfig)&&options.networkTag===recoveryPlan.networkTag&&expectedProtocolVersion===recoveryPlan.expectedProtocolVersion,'RECOVERY_INTEGRATION_BINDING');
      requireThat(roles.firstAddress===recoveryPlan.roles.firstAddress&&roles.secondAddress===recoveryPlan.roles.secondAddress&&options.walletContext.unshieldedKeystore.getBech32Address().toString()===recoveryPlan.wallet.expectedAddress,'RECOVERY_INTEGRATION_ROLES');
      requireThat(options.privateStateConfig&&Object.keys(options.privateStateConfig).sort().join(',')==='midnightDbName,privateStateStoreName,privateStoragePasswordProvider'&&options.privateStateConfig.midnightDbName===recovery.destinationDirectory&&options.privateStateConfig.privateStateStoreName==='sp05-loan','RECOVERY_INTEGRATION_STORE');
      requireThat(limits.allocationId===recoveryPlan.limits.allocationId&&limits.deadlineMs===recoveryPlan.limits.deadlineMs&&limits.submissions===3&&limits.dustFee===BigInt(recoveryPlan.limits.dustFee)&&logical.USD_TEST_ASSET===BigInt(recoveryPlan.limits.grossByLogicalAsset.USD_TEST_ASSET)&&limits.reservationStatePath===join(recoveryPlan.outputDirectory,'reservations.json'),'RECOVERY_INTEGRATION_LIMITS');
    }
    if(continuation){
      requireThat(kind===continuationPlan.kind&&isDeepStrictEqual(buildBinding,continuationPlan.build)&&isDeepStrictEqual(network,continuationPlan.networkConfig)&&options.networkTag===continuationPlan.networkTag&&expectedProtocolVersion===continuationPlan.expectedProtocolVersion,'CONTINUATION_INTEGRATION_BINDING');
      requireThat(roles.firstAddress===continuationPlan.roles.firstAddress&&roles.secondAddress===continuationPlan.roles.secondAddress&&options.walletContext.unshieldedKeystore.getBech32Address().toString()===continuationPlan.wallet.expectedAddress,'CONTINUATION_INTEGRATION_ROLES');
      requireThat(options.privateStateConfig&&Object.keys(options.privateStateConfig).sort().join(',')==='midnightDbName,privateStateStoreName,privateStoragePasswordProvider'&&options.privateStateConfig.midnightDbName===continuation.privateStateDirectory&&options.privateStateConfig.privateStateStoreName==='sp05-'+kind,'CONTINUATION_INTEGRATION_STORE');
      requireThat(limits.allocationId===continuationPlan.limits.allocationId&&limits.deadlineMs===continuationPlan.limits.deadlineMs&&limits.submissions===2&&limits.dustFee===BigInt(continuationPlan.limits.dustFee)&&Object.keys(logical).every(k=>logical[k]===BigInt(continuationPlan.limits.grossByLogicalAsset[k]))&&limits.reservationStatePath===join(continuationPlan.outputDirectory,'reservations.json'),'CONTINUATION_INTEGRATION_LIMITS');
    }
    if(stale){
      requireThat(kind==='loan'&&isDeepStrictEqual(buildBinding,stalePlan.build)&&isDeepStrictEqual(network,stalePlan.networkConfig)&&options.networkTag===stalePlan.networkTag&&expectedProtocolVersion===stalePlan.expectedProtocolVersion,'STALE_INTEGRATION_BINDING');
      requireThat(roles.firstAddress===stalePlan.roles.firstAddress&&roles.secondAddress===stalePlan.roles.secondAddress&&options.walletContext.unshieldedKeystore.getBech32Address().toString()===stalePlan.wallet.expectedAddress,'STALE_INTEGRATION_ROLES');
      requireThat(options.privateStateConfig&&Object.keys(options.privateStateConfig).sort().join(',')==='midnightDbName,privateStateStoreName,privateStoragePasswordProvider'&&options.privateStateConfig.midnightDbName===stale.privateStateDirectory&&options.privateStateConfig.privateStateStoreName==='sp05-loan','STALE_INTEGRATION_STORE');
      requireThat(limits.allocationId===stalePlan.limits.allocationId&&limits.deadlineMs===stalePlan.limits.deadlineMs&&limits.submissions===1&&limits.dustFee===BigInt(stalePlan.limits.dustFee)&&logical.USD_TEST_ASSET===0n&&limits.reservationStatePath===join(stalePlan.outputDirectory,'reservations.json'),'STALE_INTEGRATION_LIMITS');
      requireThat(typeof options.retainAdverseCandidate==='function'&&typeof options.retainAdverseOutcome==='function','STALE_INTEGRATION_RETENTION');
    }
    async function within(label,fn){
      checkDeadline();let timer;const operation=Promise.resolve().then(()=>{checkDeadline();return fn();});pending.add(operation);operation.then(()=>pending.delete(operation),()=>pending.delete(operation));
      try{const value=await Promise.race([operation,new Promise((_,reject)=>{timer=setTimeout(()=>reject(Error('INTEGRATION_DEADLINE_'+label)),Math.min(limits.deadlineMs-Date.now(),2147483647));})]);checkDeadline();return value;}finally{clearTimeout(timer);}
    }
    const sdk=await within('runtime',()=>deps.loadProviderSdk()),{ledger,runtime}=await within('native',()=>deps.loadNativeRuntime());
    const payer=ledger.addressFromKey(options.walletContext.unshieldedKeystore.getPublicKey());
    const checkBinding=()=>{checkDeadline();requireThat(sdk.getNetworkId()==='undeployed'&&ledger.addressFromKey(options.walletContext.unshieldedKeystore.getPublicKey())===roles.firstAddress,'INTEGRATION_WALLET_NETWORK_BINDING');};
    checkBinding();requireThat(payer===roles.firstAddress,'INTEGRATION_PAYER');
    phase='load-assets';loaded=await within('assets',()=>deps.loadAssets({case:kind,...buildBinding}));
    const assertFresh=()=>{checkBinding();loaded.assertFresh();};assertFresh();
    const sourceBindings=JSON.parse(readFileSync(new URL('../custody/bindings.json',import.meta.url),'utf8'))[kind];
    const program=bytes(sourceBindings.programDigest),networkTag=options.networkTag;
    const FreshZkProvider=class extends sdk.NodeZkConfigProvider{constructor(path){super(path);return new Proxy(this,{get(target,key){const value=Reflect.get(target,key,target);if(typeof value!=='function')return value;return(...args)=>{assertFresh();const result=value.apply(target,args);return result?.then?result.then(value=>{assertFresh();return value;}):result;};}});}};
    const publicWalletProvider={getCoinPublicKey(){assertFresh();return options.walletContext.shieldedSecretKeys.coinPublicKey;},getEncryptionPublicKey(){assertFresh();return options.walletContext.shieldedSecretKeys.encryptionPublicKey;}};
    const contractsSdk=await within('contracts-sdk',()=>deps.loadContractsSdk());
    const constructorArgs=[roles.firstSecret,roles.secondSecret,{bytes:bytes(roles.firstAddress)},{bytes:bytes(roles.secondAddress)},program,bytes(networkTag)];
    const historicalFeeAllocations=stale?[
      historicalFeeAllocation('local-execution-04',['deploy']),historicalFeeAllocation('local-recovery-03',['initialize']),historicalFeeAllocation('local-continuation-02',['accrue','settle'])
    ]:continuation?(swapContinuation?[historicalFeeAllocation('local-swap-01',['deploy','initialize'])]:[
      historicalFeeAllocation('local-execution-04',['deploy']),historicalFeeAllocation('local-recovery-03',['initialize'])
    ]):recovery?[historicalFeeAllocation('local-execution-04',['deploy'])]:[];
    let prepared,verifiedPrivate,checkedContinuation,checkedStale,comparator;
    const historicalSummaries=new Map();
    const compareAndRetain=async(stage,observation)=>{const summary=comparator.verifyStage(stage,observation);requireThat(summary?.status==='PASS','FINANCIAL_COMPARISON_REQUIRED_PASS');const publicSummary=structuredClone(summary);const stored=await onStage(structuredClone(publicSummary));requireThat(stored?.status==='RECORDED'&&stored.stage===stage&&stored.txId===summary.txId,'STAGE_RETENTION_REQUIRED');summaries.push(publicSummary);return summary;};
    if(stale){
      phase='stale-public';checkedStale=await within('stale-public',()=>staleDeps.publicCheck(stalePlan));
      requireThat(checkedStale?.status==='SETTLED_LOAN_PUBLIC_VERIFIED'&&checkedStale.history?.length===4,'STALE_PUBLIC_RESULT');
      phase='stale-history';comparator=await within('comparator',()=>deps.createComparator({kind,roles,networkTag,expectedProtocolVersion,dustFeeCap:limits.dustFee,historicalFeeAllocations}));
      for(const [i,stage] of ['deploy','initialize','accrue','settle'].entries()){const row=checkedStale.history[i];requireThat(row.stage===stage,'STALE_HISTORY_ORDER');await within('stale-history-'+stage,()=>compareAndRetain(stage,row.observation));}
      financialComparison=comparator.finish();
      phase='stale-wallet';const synced=await within('stale-sync',()=>wallet.waitForSyncedState());
      staleDeps.checkWallet({synced,binding:checkedStale.binding,timestampMs:checkedStale.tip.timestampMs,dustCap:limits.dustFee});checkDeadline();
      phase='stale-preserve';const preserved=await within('stale-preserve',()=>staleDeps.preserveStore({sourceDirectory:stale.privateStateDirectory,snapshotDirectory:stale.snapshotDirectory,inspectionDirectory:stale.inspectionDirectory,accountId:options.walletContext.unshieldedKeystore.getBech32Address().toString()}));requireThat(preserved?.status==='PRESERVED','STALE_STORE_PRESERVATION');
    }else if(continuation){
      phase='continuation-public';checkedContinuation=await within('continuation-public',()=>continuationDeps.publicCheck(continuationPlan));
      requireThat(checkedContinuation?.status==='INITIALIZED_PUBLIC_STATE_VERIFIED'&&checkedContinuation.deploymentObservation?.receipt?.txId===historyDeploy.txId&&checkedContinuation.initializeObservation?.receipt?.txId===historyInitialize.txId,'CONTINUATION_PUBLIC_RESULT');
      phase='continuation-wallet';const synced=await within('continuation-sync',()=>wallet.waitForSyncedState());
      continuationDeps.checkWallet({synced,binding:checkedContinuation.binding,timestampMs:checkedContinuation.tip.timestampMs,dustCap:limits.dustFee});checkDeadline();
      phase='continuation-history';comparator=await within('comparator',()=>deps.createComparator({kind,roles,networkTag,expectedProtocolVersion,dustFeeCap:limits.dustFee,historicalFeeAllocations}));
      for(const [stage,observation] of [['deploy',checkedContinuation.deploymentObservation],['initialize',checkedContinuation.initializeObservation]])historicalSummaries.set(stage,await within('history-'+stage,()=>compareAndRetain(stage,observation)));
      if(swapContinuation){phase='continuation-mint';assertInitializedSwapMintedOutput({receipt:checkedContinuation.initializeObservation.receipt,roles,assetBindings:{ASSET_A:runtime.rawTokenType(bytes(sourceBindings.assetADomain),historyInitialize.contractAddress),ASSET_B:runtime.rawTokenType(bytes(sourceBindings.assetBDomain),historyInitialize.contractAddress)},synced});checkDeadline();}
      phase='continuation-preserve';const preserved=await within('continuation-preserve',()=>continuationDeps.preserveStore({sourceDirectory:continuation.privateStateDirectory,snapshotDirectory:continuation.snapshotDirectory,inspectionDirectory:continuation.inspectionDirectory,accountId:options.walletContext.unshieldedKeystore.getBech32Address().toString()}));requireThat(preserved?.status==='PRESERVED','CONTINUATION_STORE_PRESERVATION');
    }else if(recovery){
      phase='recovery-public';const checked=await within('recovery-public',()=>recoveryDeps.publicCheck(recoveryPlan));
      phase='recovery-wallet';const synced=await within('recovery-sync',()=>wallet.waitForSyncedState());
      recoveryDeps.checkWallet({synced,binding:checked.binding,timestampMs:checked.tip.timestampMs,dustCap:limits.dustFee});checkDeadline();
      phase='recovery-constructor';verifiedPrivate=await within('recovery-constructor',()=>recoveryDeps.reconstruct({compiledContract:loaded.compiledContract,zkConfigProvider:new FreshZkProvider(loaded.zkConfigPath),coinPublicKey:publicWalletProvider.getCoinPublicKey(),signingKey:options.deploymentSigningKey,args:constructorArgs,ledger}));
      phase='recovery-store';await within('recovery-store',()=>recoveryDeps.inspectStore({sourceDirectory:recovery.sourcePrivateStateDirectory,inspectionDirectory:recovery.inspectionDirectory,accountId:options.walletContext.unshieldedKeystore.getBech32Address().toString()}));
    }else{
      phase='prepare-deployment';
      prepared=await within('prepare',()=>deps.prepareDeployment({deploymentOptions:{compiledContract:loaded.compiledContract,privateStateId:`sp05-${kind}`,initialPrivateState:{},args:constructorArgs},publicWalletProvider,zkConfigProvider:new FreshZkProvider(loaded.zkConfigPath),signingKey:options.deploymentSigningKey,ledger,sdk:contractsSdk}));
    }
    const contractAddress=stale?stale.contractAddress:continuation?historyInitialize.contractAddress:recovery?EXISTING_LOAN.contractAddress:prepared.public.contractAddress;bytes(contractAddress);assertFresh();
    const domains=kind==='loan'?{USD_TEST_ASSET:sourceBindings.usdDomain}:{ASSET_A:sourceBindings.assetADomain,ASSET_B:sourceBindings.assetBDomain};
    const grossByAsset={};assetBindings={};
    for(const name of Object.keys(domains)){const color=runtime.rawTokenType(bytes(domains[name]),contractAddress);bytes(color);requireThat(!Object.hasOwn(grossByAsset,color),'ASSET_COLOR_COLLISION');grossByAsset[color]=logical[name];assetBindings[name]=color;}
    delete limits.grossByLogicalAsset;limits=Object.freeze({...limits,grossByAsset:Object.freeze(grossByAsset)});
    const providerOptions={walletContext:options.walletContext,networkConfig:network,zkConfigPath:loaded.zkConfigPath,privateStateConfig:options.privateStateConfig,limits,ledger,sdk:{...sdk,NodeZkConfigProvider:FreshZkProvider},onEvent};
    if(recovery){phase='recovery-destination';recoveryDeps.checkDestination(recovery.destinationDirectory);checkDeadline();}
    phase='allocate';await within('allocate',()=>deps.initializeReservations(providerOptions));
    phase='providers';providers=await within('providers',async()=>{const p=await deps.createProviders(providerOptions);if(Date.now()>=limits.deadlineMs){await p.cleanup();throw Error('INTEGRATION_LATE_PROVIDER');}return p;});
    if(continuation){phase='continuation-private';const checked=await within('continuation-private',()=>continuationDeps.checkPrivate({provider:providers.privateStateProvider,signingKey:options.deploymentSigningKey,contractState:checkedContinuation.initializeContractState,ledger}));requireThat(checked?.status==='PRIVATE_STATE_CHECKED','CONTINUATION_PRIVATE_RESULT');}
    if(recovery){phase='recovery-restore';await within('recovery-restore',()=>recoveryDeps.restore(providers.privateStateProvider,verifiedPrivate));}
    const rpc=(method,params,requestDeadline=limits.deadlineMs)=>createLocalRpc({node:network.node,deadlineMs:Math.min(limits.deadlineMs,requestDeadline),fetchImpl:deps.fetch})(method,params);
    comparator??=await within('comparator',()=>deps.createComparator({kind,roles,networkTag,expectedProtocolVersion,dustFeeCap:limits.dustFee,historicalFeeAllocations}));
    if(stale){
      phase='stale-prepare';staleHandle=await within('stale-prepare',async()=>{const handle=await staleDeps.prepare({providers,rpc,receiptPath:buildBinding.receiptPath,borrowerSecret:roles.firstSecret,signingKey:options.deploymentSigningKey,deadlineMs:limits.deadlineMs,nowSeconds:now()});if(Date.now()>=limits.deadlineMs){await staleDeps.close(handle);throw Error('STALE_PREPARATION_LATE');}return handle;});
      phase='stale-submit';const handle=staleHandle;staleHandle=undefined;
      adverse=await within('stale-submit',()=>staleDeps.submit(handle,{retainCandidate:options.retainAdverseCandidate,retainSubmissionOutcome:options.retainAdverseOutcome}));
      requireThat(adverse&&['NODE_REJECTION_FINANCIAL_NONMUTATION','OUTCOME_UNKNOWN','UNEXPECTED_SUBMISSION_SUCCESS','INCOMPLETE'].includes(adverse.status),'STALE_ADVERSE_RESULT');
      const state=providers.getState();adverse={...adverse,reservations:{reservedSubmissions:state.reservedSubmissions,reservedDustFee:state.reservedDustFee,reservedGrossByAsset:state.reservedGrossByAsset,identifiers:state.identifiers,pendingOperations:state.pendingOperations}};
    }else{
    const freshDriverSdk=(recovery||continuation)?{submitCallTx(...args){assertFresh();return contractsSdk.submitCallTx(...args);}}:{deployContract(...args){assertFresh();return prepared.driverSdk.deployContract(...args);},submitCallTx(...args){assertFresh();return prepared.driverSdk.submitCallTx(...args);}};
    phase='driver';driverStarted=true;
    let driverFailure;
    try{
      driverResult=await deps.driver({kind,network:'undeployed',providers,compiledContract:loaded.compiledContract,roles,networkTag,now,sdk:freshDriverSdk,
        ...(recovery?{existingDeployment:{contractAddress,txId:EXISTING_LOAN.txId}}:{}),
        ...(continuation?{[swapContinuation?'initializedSwap':'initializedLoan']:{contractAddress,deployTxId:historyDeploy.txId,initializeTxId:historyInitialize.txId}}:{}),
        observe:({circuitId,txId,contractAddress:observedAddress})=>{assertFresh();requireThat(observedAddress===contractAddress,'PREPARED_OBSERVED_ADDRESS_MISMATCH');if(continuation&&['deploy','initialize'].includes(circuitId)){const observation=circuitId==='deploy'?checkedContinuation.deploymentObservation:checkedContinuation.initializeObservation;requireThat(observation.receipt.txId===txId,'CONTINUATION_HISTORY_ID');return observation;}return deps.observe({provider:providers.publicDataProvider,rpc,ledger,circuitId,txId,contractAddress:observedAddress,decodeState:loaded.decodeState,deadlineMs:limits.deadlineMs,expectedProtocolVersion});},
        verifyStage:async(stage,observation)=>{
          if(continuation&&historicalSummaries.has(stage)){requireThat(observation===(stage==='deploy'?checkedContinuation.deploymentObservation:checkedContinuation.initializeObservation),'CONTINUATION_HISTORY_OBSERVATION');return historicalSummaries.get(stage);}
          const summary=await compareAndRetain(stage,observation);
          if(kind==='swap'&&stage==='initialize'){
            const synced=await within('swap-wallet-sync',()=>wallet.waitForSyncedState());
            assertSwapInitializedWallet({receipt:observation.receipt,roles,assetBindings,synced});checkDeadline();
          }
          return summary;
        }});
    }catch(error){driverFailure=error;driverResult=error.publicResult;throw error;}
    finally{if(summaries.length===4){try{financialComparison=comparator.finish();}catch(error){if(!driverFailure)throw error;}}}
    driverResult={...driverResult,assetBindings};
    }
  }catch(error){failure=error instanceof Error?error:Error('INTEGRATION_FAILED');}
  finally{
    if(staleHandle){try{await closeStaleHandle(staleHandle);}catch{failure??=Error('STALE_HANDLE_CLEANUP');}}
    if(!driverStarted || !driverResult?.cleanup){
      if(providers){try{cleanup=await providers.cleanup();}catch{cleanup=await stopWallet(wallet);}}
      else cleanup=await stopWallet(wallet);
    }else cleanup=driverResult?.cleanup;
    if(loaded){try{await loaded.cleanup();}catch{failure??=Error('ASSET_LOADER_CLEANUP_FAILED');}}
  }
  if(driverResult?.status==='FINANCIAL_COMPLETE'&&!failure){try{requireThat(financialComparison?.status==='PASS'&&summaries.length===4&&pending.size===0&&cleanup?.walletStopped===true&&cleanup?.pendingOperations===0,'INTEGRATION_COMPLETION_REQUIRED');requireThat(Number.isSafeInteger(limits.deadlineMs)&&Date.now()<limits.deadlineMs,'INTEGRATION_DEADLINE');}catch(error){failure=error;}}
  const adverseVerified=adverse?.status==='NODE_REJECTION_FINANCIAL_NONMUTATION'&&adverse.financialNonmutationEstablished===true&&adverse.rejection?.nodeRejectionEstablished===true&&adverse.equality?.status==='FINANCIAL_STATE_UNCHANGED'&&adverse.before?.stateSha256===adverse.after?.stateSha256&&adverse.before?.serializedStateHex===adverse.after?.serializedStateHex&&adverse.after?.blockHeight>adverse.terminalBarrier?.blockHeight&&adverse.terminalBarrier?.blockHeight>=adverse.before?.blockHeight&&adverse.reservations?.pendingOperations===0&&pending.size===0&&cleanup?.pendingOperations===0;
  const publicResult={schema:'moriarty.local-financial-integration/1',status:failure?(['FAILED','INCOMPLETE'].includes(driverResult?.status)?driverResult.status:'FAILED'):sourceTestOnly?'SOURCE_TEST_ONLY':adverse?(adverseVerified&&cleanup?.containmentComplete===true?'PASS':'INCOMPLETE'):driverResult.status,kind:['loan','swap'].includes(options?.kind)?options.kind:null,sourceTestOnly,networkAcceptance:false,proofAcceptance:false,financialAcceptance:false,
    build:buildBinding?{receiptSha256:buildBinding.receiptSha256,sourceManifestHash:buildBinding.sourceManifestHash}:undefined,assetBindings,phase,driver:driverResult,...(adverse?{adverse}:{}),cleanup,setupPendingOperations:pending.size,comparisons:summaries,financialComparison,...(failure?{failureCode:integrationFailureCode(failure)}:{}),
    scope:'Fixed I2 composition; source adapters and incomplete containment never establish financial network acceptance'};
  if(failure){failure.publicIntegrationResult=publicResult;throw failure;}
  return publicResult;
}
