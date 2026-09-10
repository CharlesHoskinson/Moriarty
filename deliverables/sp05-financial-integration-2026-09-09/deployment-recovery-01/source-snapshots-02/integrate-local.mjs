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
import {observeFinalizedStage} from './receipt.mjs';
import {createFinancialComparator} from './financial-comparison.mjs';
import {runLocalFinancialCase} from './run-local.mjs';
import {EXISTING_LOAN,validateExistingLoanPlan,readExistingLoanInputs,verifyExistingLoanPublic,reconstructExistingLoan,assertRecoveryWallet,restoreExistingLoanPrivate} from './recover-deployment.mjs';
import {inspectFailedLoanStore,assertEmptyRecoveryStore} from './recover-store.mjs';

const requireThat=(condition,message)=>{if(!condition)throw Error(message);};
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
/** Read-only local JSON-RPC: one request, no redirects/retries, 64KiB body ceiling. */
export function createLocalRpc({node,deadlineMs,fetchImpl=globalThis.fetch}){
  const endpoint=localEndpoint(node,'http:');validDeadline(deadlineMs);requireThat(typeof fetchImpl==='function','RPC_FETCH_REQUIRED');
  let nextId=0;
  return async(method,params)=>{
    requireThat(['chain_getFinalizedHead','chain_getHeader','chain_getBlockHash'].includes(method)&&Array.isArray(params),'RPC_METHOD');
    requireThat(method==='chain_getFinalizedHead'?params.length===0:method==='chain_getHeader'?params.length===1&&typeof params[0]==='string'&&/^0x[a-f0-9]{64}$/i.test(params[0]):params.length===1&&Number.isSafeInteger(params[0])&&params[0]>=0,'RPC_PARAMS');
    validDeadline(deadlineMs);const id=++nextId,controller=new AbortController();let timer;
    const operation=(async()=>{
      const response=await fetchImpl(endpoint,{method:'POST',redirect:'error',headers:{'content-type':'application/json'},body:JSON.stringify({jsonrpc:'2.0',id,method,params}),signal:controller.signal});
      requireThat(response.ok===true&&response.body,'RPC_HTTP_STATUS');
      const reader=response.body.getReader(),chunks=[];let size=0;
      try{while(true){const {done,value}=await reader.read();if(done)break;size+=value.byteLength;if(size>65536){controller.abort();throw Error('RPC_BODY_LIMIT');}chunks.push(Buffer.from(value));}}
      finally{reader.releaseLock();}
      const body=JSON.parse(Buffer.concat(chunks).toString('utf8'));
      requireThat(body?.jsonrpc==='2.0'&&body.id===id&&Object.hasOwn(body,'result')&&!Object.hasOwn(body,'error'),'RPC_RESPONSE');
      return body.result;
    })();
    try{return await Promise.race([operation,new Promise((_,reject)=>{timer=setTimeout(()=>{controller.abort();reject(Error('RPC_DEADLINE'));},Math.min(deadlineMs-Date.now(),2147483647));})]);}
    finally{clearTimeout(timer);controller.abort();}
  };
}
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
    const rpc=createLocalRpc({node:plan.networkConfig.node,deadlineMs});
    requireThat(await rpc('chain_getBlockHash',[0])==='0x'+recovery.networkTag,'RECOVERY_PUBLIC_GENESIS');
    const {waitForLocalTip}=await import('./local-tip.mjs');
    const tip=await waitForLocalTip({node:plan.networkConfig.node,indexer:plan.networkConfig.indexer,deadlineMs});
    const provider=sdk.indexerPublicDataProvider(plan.networkConfig.indexer,plan.networkConfig.indexerWS);
    const result=await verifyExistingLoanPublic({raw,ledger,provider,rpc,decodeState:loaded.decodeState,deadlineMs,expectedProtocolVersion:plan.expectedProtocolVersion,tip});
    loaded.assertFresh();validDeadline(deadlineMs);return result;
  }finally{if(loaded)await loaded.cleanup();}
}
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
  let loaded,providers,driverStarted=false,driverResult,financialComparison,failure,cleanup,phase='preflight';
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
    requireThat(options.existingDeployment===undefined,'INTEGRATION_USE_CLOSED_RECOVERY_PLAN');
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
    let prepared,verifiedPrivate;
    if(recovery){
      phase='recovery-public';const checked=await within('recovery-public',()=>recoveryDeps.publicCheck(recoveryPlan));
      phase='recovery-wallet';const synced=await within('recovery-sync',()=>wallet.waitForSyncedState());
      recoveryDeps.checkWallet({synced,binding:checked.binding,timestampMs:checked.tip.timestampMs,dustCap:limits.dustFee});checkDeadline();
      phase='recovery-constructor';verifiedPrivate=await within('recovery-constructor',()=>recoveryDeps.reconstruct({compiledContract:loaded.compiledContract,zkConfigProvider:new FreshZkProvider(loaded.zkConfigPath),coinPublicKey:publicWalletProvider.getCoinPublicKey(),signingKey:options.deploymentSigningKey,args:constructorArgs,ledger}));
      phase='recovery-store';await within('recovery-store',()=>recoveryDeps.inspectStore({sourceDirectory:recovery.sourcePrivateStateDirectory,inspectionDirectory:recovery.inspectionDirectory,accountId:options.walletContext.unshieldedKeystore.getBech32Address().toString()}));
    }else{
      phase='prepare-deployment';
      prepared=await within('prepare',()=>deps.prepareDeployment({deploymentOptions:{compiledContract:loaded.compiledContract,privateStateId:`sp05-${kind}`,initialPrivateState:{},args:constructorArgs},publicWalletProvider,zkConfigProvider:new FreshZkProvider(loaded.zkConfigPath),signingKey:options.deploymentSigningKey,ledger,sdk:contractsSdk}));
    }
    const contractAddress=recovery?EXISTING_LOAN.contractAddress:prepared.public.contractAddress;bytes(contractAddress);assertFresh();
    const domains=kind==='loan'?{USD_TEST_ASSET:sourceBindings.usdDomain}:{ASSET_A:sourceBindings.assetADomain,ASSET_B:sourceBindings.assetBDomain};
    const grossByAsset={};assetBindings={};
    for(const name of Object.keys(domains)){const color=runtime.rawTokenType(bytes(domains[name]),contractAddress);bytes(color);requireThat(!Object.hasOwn(grossByAsset,color),'ASSET_COLOR_COLLISION');grossByAsset[color]=logical[name];assetBindings[name]=color;}
    delete limits.grossByLogicalAsset;limits=Object.freeze({...limits,grossByAsset:Object.freeze(grossByAsset)});
    const providerOptions={walletContext:options.walletContext,networkConfig:network,zkConfigPath:loaded.zkConfigPath,privateStateConfig:options.privateStateConfig,limits,ledger,sdk:{...sdk,NodeZkConfigProvider:FreshZkProvider},onEvent};
    phase='allocate';await within('allocate',()=>deps.initializeReservations(providerOptions));
    if(recovery){recoveryDeps.checkDestination(recovery.destinationDirectory);checkDeadline();}
    phase='providers';providers=await within('providers',async()=>{const p=await deps.createProviders(providerOptions);if(Date.now()>=limits.deadlineMs){await p.cleanup();throw Error('INTEGRATION_LATE_PROVIDER');}return p;});
    if(recovery){phase='recovery-restore';await within('recovery-restore',()=>recoveryDeps.restore(providers.privateStateProvider,verifiedPrivate));}
    const rpc=createLocalRpc({node:network.node,deadlineMs:limits.deadlineMs,fetchImpl:deps.fetch});
    const comparator=await within('comparator',()=>deps.createComparator({kind,roles,networkTag,expectedProtocolVersion}));
    const freshDriverSdk=recovery?{submitCallTx(...args){assertFresh();return contractsSdk.submitCallTx(...args);}}:{deployContract(...args){assertFresh();return prepared.driverSdk.deployContract(...args);},submitCallTx(...args){assertFresh();return prepared.driverSdk.submitCallTx(...args);}};
    phase='driver';driverStarted=true;
    let driverFailure;
    try{
      driverResult=await deps.driver({kind,network:'undeployed',providers,compiledContract:loaded.compiledContract,roles,networkTag,now,sdk:freshDriverSdk,
        ...(recovery?{existingDeployment:{contractAddress,txId:EXISTING_LOAN.txId}}:{}),
        observe:({circuitId,txId,contractAddress:observedAddress})=>{assertFresh();requireThat(observedAddress===contractAddress,'PREPARED_OBSERVED_ADDRESS_MISMATCH');return deps.observe({provider:providers.publicDataProvider,rpc,ledger,circuitId,txId,contractAddress:observedAddress,decodeState:loaded.decodeState,deadlineMs:limits.deadlineMs,expectedProtocolVersion});},
        verifyStage:async(stage,observation)=>{const summary=comparator.verifyStage(stage,observation);requireThat(summary?.status==='PASS','FINANCIAL_COMPARISON_REQUIRED_PASS');const publicSummary=structuredClone(summary);const stored=await onStage(structuredClone(publicSummary));requireThat(stored?.status==='RECORDED'&&stored.stage===stage&&stored.txId===summary.txId,'STAGE_RETENTION_REQUIRED');summaries.push(publicSummary);return summary;}});
    }catch(error){driverFailure=error;driverResult=error.publicResult;throw error;}
    finally{if(summaries.length===4){try{financialComparison=comparator.finish();}catch(error){if(!driverFailure)throw error;}}}
    driverResult={...driverResult,assetBindings};
  }catch(error){failure=error instanceof Error?error:Error('INTEGRATION_FAILED');}
  finally{
    if(!driverStarted || !driverResult?.cleanup){
      if(providers){try{cleanup=await providers.cleanup();}catch{cleanup=await stopWallet(wallet);}}
      else cleanup=await stopWallet(wallet);
    }else cleanup=driverResult?.cleanup;
    if(loaded){try{await loaded.cleanup();}catch{failure??=Error('ASSET_LOADER_CLEANUP_FAILED');}}
  }
  const publicResult={schema:'moriarty.local-financial-integration/1',status:failure?(driverResult?.status??'FAILED'):sourceTestOnly?'SOURCE_TEST_ONLY':driverResult.status,kind:['loan','swap'].includes(options?.kind)?options.kind:null,sourceTestOnly,networkAcceptance:false,proofAcceptance:false,financialAcceptance:false,
    build:buildBinding?{receiptSha256:buildBinding.receiptSha256,sourceManifestHash:buildBinding.sourceManifestHash}:undefined,assetBindings,phase,driver:driverResult,cleanup,setupPendingOperations:pending.size,comparisons:summaries,financialComparison,
    scope:'Fixed I2 composition; source adapters and incomplete containment never establish financial network acceptance'};
  if(failure){failure.publicIntegrationResult=publicResult;throw failure;}
  return publicResult;
}
