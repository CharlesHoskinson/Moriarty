/** Fixed Preview composition. Existing wallet handles transfer cleanup ownership.
 * No wallet creation, recovery, CLI or admission. Source adapters never establish acceptance.
 */
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {loadProvenFinancialContract} from './proven-assets.mjs';
import {loadFinancialContractsSdk,prepareFinancialDeployment} from './prepare-deployment.mjs';
import {PINNED_NM,loadFinancialSdk,initializeFinancialReservations,createFinancialProviders} from './providers.mjs';
import {observeFinalizedStage} from './receipt.mjs';
import {createFinancialComparator} from './financial-comparison.mjs';
import {runPreviewFinancialCase} from './run-local.mjs';
import {createPreviewRpc} from './financial-rpc.mjs';
import {decodePreviewIndexedOwner} from './indexed-owner.mjs';
import {assertSwapInitializedWallet} from './swap-wallet.mjs';
import {validatePublicIntegrationFailureCode} from './integrate-local.mjs';
const requireThat=(ok,code)=>{if(!ok)throw Error(code);};
const hash=bytes=>createHash('sha256').update(bytes).digest('hex');
const bytes=hex=>{requireThat(typeof hex==='string'&&/^[a-f0-9]{64}$/.test(hex),'INTEGRATION_PUBLIC_BYTES');return Uint8Array.from(Buffer.from(hex,'hex'));};
function validDeadline(value){requireThat(Number.isSafeInteger(value)&&value>Date.now(),'INTEGRATION_DEADLINE');}
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
function loadPublicIdentity(){
 const raw=readFileSync(new URL('../../../evidence/midnight-preview-2026-09-07/wallet-public.json',import.meta.url));
 requireThat(hash(raw)==='01a31bab9f9920f1385f3fef92fee7638b477f77875390613ebb19e1a786fc98','PREVIEW_PUBLIC_IDENTITY_PIN');
 const value=JSON.parse(raw);requireThat(value.network==='preview','PREVIEW_WALLET_IDENTITY');return {address:value.address};
}
const realDependencies={loadAssets:loadProvenFinancialContract,loadContractsSdk:loadFinancialContractsSdk,loadProviderSdk:loadFinancialSdk,loadNativeRuntime,loadPublicIdentity,prepareDeployment:prepareFinancialDeployment,initializeReservations:initializeFinancialReservations,createProviders:createFinancialProviders,createComparator:createFinancialComparator,observe:observeFinalizedStage,driver:runPreviewFinancialCase,fetch:globalThis.fetch};
const previewFailureCodes=new Set(['PREVIEW_NETWORK_CONFIG','PREVIEW_PUBLIC_IDENTITY_PIN','PREVIEW_WALLET_IDENTITY','PREVIEW_GENESIS_MISMATCH','PREVIEW_RECOVERY_FORBIDDEN','PREVIEW_INTEGRATION_INCOMPLETE','PREVIEW_INTEGRATION_FAILURE','INTEGRATION_DEADLINE']);
function integrationFailureCode(error){const code=Object.getOwnPropertyDescriptor(error,'message')?.value;if(previewFailureCodes.has(code))return code;try{return validatePublicIntegrationFailureCode(code);}catch{return 'UNCLASSIFIED_INTEGRATION_FAILURE';}}
async function stopWallet(wallet){
  if(typeof wallet?.stop!=='function')return {walletStopped:false,pendingOperations:null,containmentComplete:false};
  let timer;try{await Promise.race([Promise.resolve().then(()=>wallet.stop()),new Promise((_,reject)=>{timer=setTimeout(()=>reject(Error('WALLET_CLEANUP_TIMEOUT')),5000);})]);return {walletStopped:true,pendingOperations:0,containmentComplete:false};}
  catch{return {walletStopped:false,pendingOperations:null,containmentComplete:false};}finally{clearTimeout(timer);}
}
export async function integratePreviewFinancialCase(options){
 let loaded,providers,driverStarted=false,driverResult,financialComparison,failure,cleanup,phase='preflight',limits,assetBindings,buildBinding;
 const summaries=[],pending=new Set(),wallet=options?.walletContext?.wallet;let sourceTestOnly=false;
 try{
  requireThat(typeof wallet?.stop==='function','EXISTING_WALLET_CLEANUP_REQUIRED');
  sourceTestOnly=options.sourceTestOnly===true;
  requireThat(options.adapters===undefined||sourceTestOnly,'INTEGRATION_ADAPTERS_REQUIRE_SOURCE_TEST');
  const deps=sourceTestOnly?options.adapters:realDependencies;
  for(const key of Object.keys(realDependencies))requireThat(typeof deps?.[key]==='function','COMPLETE_INERT_ADAPTERS_REQUIRED');
  for(const key of ['recoveryPlan','continuationPlan','existingDeployment','initializedLoan','initializedSwap'])requireThat(options[key]===undefined,'PREVIEW_RECOVERY_FORBIDDEN');
  const {kind}=options;requireThat(kind==='loan'||kind==='swap','INTEGRATION_KIND');
  requireThat(options.build&&Object.keys(options.build).sort().join(',')==='receiptPath,receiptSha256,sourceManifestHash','BUILD_RECEIPT_BINDING_REQUIRED');
  bytes(options.build.receiptSha256);bytes(options.build.sourceManifestHash);buildBinding=Object.freeze({...options.build});
  const expectedProtocolVersion=options.expectedProtocolVersion;requireThat(Number.isSafeInteger(expectedProtocolVersion)&&expectedProtocolVersion>=0,'EXPECTED_PROTOCOL_VERSION_REQUIRED');
  const {onStage,onEvent,now}=options;requireThat([onStage,onEvent,now].every(x=>typeof x==='function'),'REVIEWED_STAGE_EVENT_TIME_CALLBACKS_REQUIRED');
  const network=Object.freeze({...options.networkConfig});
  requireThat(network.networkId==='preview'&&['https://rpc.preview.midnight.network','https://rpc.preview.midnight.network/'].includes(network.node)&&network.indexer==='https://indexer.preview.midnight.network/api/v4/graphql'&&network.indexerWS==='wss://indexer.preview.midnight.network/api/v4/graphql/ws','PREVIEW_NETWORK_CONFIG');
  const proof=new URL(network.proofServer);requireThat(proof.protocol==='http:'&&['127.0.0.1','localhost','[::1]'].includes(proof.hostname)&&!proof.username&&!proof.password&&!proof.hash,'PREVIEW_NETWORK_CONFIG');
  requireThat(options.roles&&Object.keys(options.roles).sort().join(',')==='firstAddress,firstSecret,secondAddress,secondSecret','ROLE_BINDING_FIELDS');
  requireThat(options.roles.firstSecret instanceof Uint8Array&&options.roles.secondSecret instanceof Uint8Array,'ROLE_SECRETS_REQUIRED');
  const roles={...options.roles,firstSecret:Uint8Array.from(options.roles.firstSecret),secondSecret:Uint8Array.from(options.roles.secondSecret)};
  requireThat(roles.firstSecret.length===32&&roles.secondSecret.length===32,'ROLE_SECRETS_REQUIRED');bytes(roles.firstAddress);bytes(roles.secondAddress);requireThat(roles.firstAddress!==roles.secondAddress,'DISTINCT_PARTICIPANTS_REQUIRED');
  const networkTag=options.networkTag;bytes(networkTag);requireThat(options.deploymentSigningKey!==undefined,'EXISTING_DEPLOYMENT_SIGNING_KEY_REQUIRED');
  limits={...options.limits};validDeadline(limits.deadlineMs);requireThat(limits.submissions===4&&typeof limits.dustFee==='bigint'&&limits.dustFee>=0n,'FINANCIAL_LIMITS');
  const logical={...limits.grossByLogicalAsset},names=kind==='loan'?['USD_TEST_ASSET']:['ASSET_A','ASSET_B'];
  requireThat(Object.keys(logical).sort().join('|')===names.sort().join('|'),'LOGICAL_ASSET_LIMITS');for(const cap of Object.values(logical))requireThat(typeof cap==='bigint'&&cap>=0n,'LOGICAL_ASSET_CAP');requireThat(!Object.hasOwn(limits,'grossByAsset'),'ALREADY_BOUND_ASSET_LIMITS_FORBIDDEN');
  async function within(label,fn){
   validDeadline(limits.deadlineMs);let timer;const op=Promise.resolve().then(()=>{validDeadline(limits.deadlineMs);return fn();});pending.add(op);op.then(()=>pending.delete(op),()=>pending.delete(op));
   try{const value=await Promise.race([op,new Promise((_,reject)=>{timer=setTimeout(()=>reject(Error('INTEGRATION_DEADLINE_'+label)),Math.min(limits.deadlineMs-Date.now(),2147483647));})]);validDeadline(limits.deadlineMs);return value;}finally{clearTimeout(timer);}
  }
  const rpc=(method,params,requestDeadline=limits.deadlineMs)=>{validDeadline(requestDeadline);return createPreviewRpc({node:network.node,deadlineMs:Math.min(limits.deadlineMs,requestDeadline),fetchImpl:deps.fetch})(method,params);};
  phase='public-identity';const identity=await within('identity',()=>deps.loadPublicIdentity());
  requireThat(await within('genesis',()=>rpc('chain_getBlockHash',[0]))==='0x'+networkTag,'PREVIEW_GENESIS_MISMATCH');
  const sdk=await within('runtime',()=>deps.loadProviderSdk()),{ledger,runtime}=await within('native',()=>deps.loadNativeRuntime());
  const checkBinding=()=>{validDeadline(limits.deadlineMs);requireThat(sdk.getNetworkId()==='preview'&&options.walletContext.unshieldedKeystore.getBech32Address().toString()===identity.address&&decodePreviewIndexedOwner(identity.address)===roles.firstAddress&&ledger.addressFromKey(options.walletContext.unshieldedKeystore.getPublicKey())===roles.firstAddress,'PREVIEW_WALLET_IDENTITY');};checkBinding();
  phase='load-assets';loaded=await within('assets',async()=>{const a=await deps.loadAssets({case:kind,...buildBinding});if(Date.now()>=limits.deadlineMs){await a.cleanup();throw Error('INTEGRATION_LATE_ASSETS');}return a;});
  const assertFresh=()=>{checkBinding();loaded.assertFresh();};assertFresh();
  const sourceBindings=JSON.parse(readFileSync(new URL('../custody/bindings.json',import.meta.url)))[kind],program=bytes(sourceBindings.programDigest);
  const FreshZkProvider=class extends sdk.NodeZkConfigProvider{constructor(path){super(path);return new Proxy(this,{get(target,key){const value=Reflect.get(target,key,target);if(typeof value!=='function')return value;return(...args)=>{assertFresh();const result=value.apply(target,args);return result?.then?result.then(value=>{assertFresh();return value;}):result;};}});}};
  const publicWalletProvider={getCoinPublicKey(){assertFresh();return options.walletContext.shieldedSecretKeys.coinPublicKey;},getEncryptionPublicKey(){assertFresh();return options.walletContext.shieldedSecretKeys.encryptionPublicKey;}};
  const contractsSdk=await within('contracts-sdk',()=>deps.loadContractsSdk());
  phase='prepare-deployment';const prepared=await within('prepare',()=>deps.prepareDeployment({deploymentOptions:{compiledContract:loaded.compiledContract,privateStateId:`sp05-${kind}`,initialPrivateState:{},args:[roles.firstSecret,roles.secondSecret,{bytes:bytes(roles.firstAddress)},{bytes:bytes(roles.secondAddress)},program,bytes(networkTag)]},publicWalletProvider,zkConfigProvider:new FreshZkProvider(loaded.zkConfigPath),signingKey:options.deploymentSigningKey,ledger,sdk:contractsSdk}));
  const contractAddress=prepared.public.contractAddress;bytes(contractAddress);assertFresh();
  const domains=kind==='loan'?{USD_TEST_ASSET:sourceBindings.usdDomain}:{ASSET_A:sourceBindings.assetADomain,ASSET_B:sourceBindings.assetBDomain},grossByAsset={};assetBindings={};
  for(const [name,domain] of Object.entries(domains)){const color=runtime.rawTokenType(bytes(domain),contractAddress);bytes(color);requireThat(!Object.hasOwn(grossByAsset,color),'ASSET_COLOR_COLLISION');grossByAsset[color]=logical[name];assetBindings[name]=color;}
  delete limits.grossByLogicalAsset;limits=Object.freeze({...limits,grossByAsset:Object.freeze(grossByAsset)});
  const providerOptions={walletContext:options.walletContext,networkConfig:network,zkConfigPath:loaded.zkConfigPath,privateStateConfig:options.privateStateConfig,limits,ledger,sdk:{...sdk,NodeZkConfigProvider:FreshZkProvider},onEvent};
  phase='allocate';await within('allocate',()=>deps.initializeReservations(providerOptions));
  phase='providers';providers=await within('providers',async()=>{const p=await deps.createProviders(providerOptions);if(Date.now()>=limits.deadlineMs){await p.cleanup();throw Error('INTEGRATION_LATE_PROVIDER');}return p;});
  const comparator=await within('comparator',()=>deps.createComparator({kind,roles,networkTag,expectedProtocolVersion}));
  const freshSdk={deployContract(...args){assertFresh();return prepared.driverSdk.deployContract(...args);},submitCallTx(...args){assertFresh();return prepared.driverSdk.submitCallTx(...args);}};
  phase='driver';driverStarted=true;let driverFailure;
  try{driverResult=await deps.driver({kind,network:'preview',providers,compiledContract:loaded.compiledContract,roles,networkTag,now,sdk:freshSdk,
   observe:({circuitId,txId,contractAddress:observedAddress,network:ownerNetwork})=>{assertFresh();requireThat(observedAddress===contractAddress&&ownerNetwork==='preview','PREPARED_OBSERVED_ADDRESS_MISMATCH');return deps.observe({provider:providers.publicDataProvider,rpc,ledger,circuitId,txId,contractAddress,network:'preview',decodeState:loaded.decodeState,deadlineMs:limits.deadlineMs,expectedProtocolVersion});},
   verifyStage:async(stage,observation)=>{const summary=comparator.verifyStage(stage,observation);requireThat(summary?.status==='PASS','FINANCIAL_COMPARISON_REQUIRED_PASS');const stored=await onStage(structuredClone(summary));requireThat(stored?.status==='RECORDED'&&stored.stage===stage&&stored.txId===summary.txId,'STAGE_RETENTION_REQUIRED');summaries.push(structuredClone(summary));
    if(kind==='swap'&&stage==='initialize'){const synced=await within('swap-sync',()=>wallet.waitForSyncedState());assertSwapInitializedWallet({receipt:observation.receipt,roles,assetBindings,synced,network:'preview'});validDeadline(limits.deadlineMs);}return summary;}
  });}catch(error){driverFailure=error;driverResult=error.publicResult;throw error;}
  finally{if(summaries.length===4){try{financialComparison=comparator.finish();}catch(error){if(!driverFailure)throw error;}}}
  requireThat(driverResult?.status==='PASS'&&financialComparison?.status==='PASS'&&pending.size===0,'PREVIEW_INTEGRATION_INCOMPLETE');
 }catch(error){failure=error instanceof Error?error:Error('PREVIEW_INTEGRATION_FAILURE');}
 finally{
  if(!driverStarted||!driverResult?.cleanup){if(providers){try{cleanup=await providers.cleanup();}catch{cleanup=await stopWallet(wallet);}}else cleanup=await stopWallet(wallet);}else cleanup=driverResult.cleanup;
  if(loaded){try{await loaded.cleanup();}catch{failure??=Error('ASSET_LOADER_CLEANUP_FAILED');}}
 }
 if(!failure){try{validDeadline(limits.deadlineMs);requireThat(pending.size===0&&cleanup?.walletStopped===true&&cleanup.pendingOperations===0&&cleanup.containmentComplete===true,'PREVIEW_INTEGRATION_INCOMPLETE');}catch(error){failure=error;}}
 const result={schema:'moriarty.preview-financial-integration/1',status:failure?'FAILED':sourceTestOnly?'SOURCE_TEST_ONLY':'PASS',kind:options?.kind,sourceTestOnly,networkAcceptance:false,proofAcceptance:false,financialAcceptance:false,build:buildBinding?{receiptSha256:buildBinding.receiptSha256,sourceManifestHash:buildBinding.sourceManifestHash}:undefined,assetBindings,phase,driver:driverResult,cleanup,setupPendingOperations:pending.size,comparisons:summaries,financialComparison,...(failure?{failureCode:integrationFailureCode(failure)}:{}),scope:'Fixed Preview composition; only independently reviewed actual evidence establishes financial acceptance'};
 if(failure){failure.publicIntegrationResult=result;throw failure;}return result;
}
