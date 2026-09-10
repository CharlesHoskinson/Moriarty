/** Existing-wallet local entry point. Actual execution requires separate admission
 * and outer process containment. Importing this module never reads private files,
 * constructs a wallet, imports SDK code, or contacts a service.
 */
import {readFileSync,openSync,closeSync,fstatSync,lstatSync,writeFileSync,fsyncSync,mkdirSync,constants} from 'node:fs';
import {createHash} from 'node:crypto';
import {join,dirname,isAbsolute,resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
import {PINNED_NM} from './providers.mjs';
import {validatePublicDriverFailure} from './run-local.mjs';
import {inspectFinancialBuild} from './proven-assets.mjs';
import {integrateLocalFinancialCase,preflightLocalRecovery} from './integrate-local.mjs';
import {waitForLocalTip,guardLocalWallet} from './local-tip.mjs';
import {validateExistingLoanPlan} from './recover-deployment.mjs';

const check=(ok,code)=>{if(!ok)throw Error(code);};
const hash=b=>createHash('sha256').update(b).digest('hex');
const hex=x=>typeof x==='string'&&/^[a-f0-9]{64}$/.test(x);
const decimal=x=>typeof x==='string'&&/^(0|[1-9][0-9]*)$/.test(x)&&x.length<=39&&BigInt(x)<1n<<128n;
const exact=(o,keys)=>check(o&&Object.getPrototypeOf(o)===Object.prototype&&Object.keys(o).sort().join('|')===keys.split(',').sort().join('|'),'LAUNCH_FIELDS');
const absolute=p=>check(typeof p==='string'&&isAbsolute(p)&&resolve(p)===p,'LAUNCH_PATH');
const childKinds=['shielded','unshielded','dust'];
const PIN_MANIFEST_SHA256='4fa41776e0fce393bf7c6ee19acd824bec0b9459ed4a93616cef35904e548195';
function runtimePins(){const raw=readFileSync(new URL('./launch-runtime-pins.json',import.meta.url));check(hash(raw)===PIN_MANIFEST_SHA256,'LAUNCH_PIN_MANIFEST');return JSON.parse(raw);}
export function validateLocalLaunchPlan(p){
 const recovery=Object.hasOwn(p??{},'existingDeployment');
 exact(p,'schema,kind,build,networkConfig,wallet,roles,privateState,networkTag,expectedProtocolVersion,limits,outputDirectory'+(recovery?',existingDeployment':''));
 check(p.schema==='moriarty.local-financial-launch/1'&&['loan','swap'].includes(p.kind),'LAUNCH_SCHEMA');
 exact(p.build,'receiptPath,receiptSha256,sourceManifestHash');absolute(p.build.receiptPath);check(hex(p.build.receiptSha256)&&hex(p.build.sourceManifestHash),'LAUNCH_BUILD_HASH');
 exact(p.networkConfig,'networkId,node,indexer,indexerWS,proofServer');check(p.networkConfig.networkId==='undeployed','LAUNCH_NETWORK');
 for(const key of ['node','indexer','indexerWS','proofServer']){const u=new URL(p.networkConfig[key]);check(u.protocol===(key==='indexerWS'?'ws:':'http:')&&['127.0.0.1','[::1]'].includes(u.hostname)&&!u.username&&!u.password&&!u.hash,'LAUNCH_LOOPBACK');}
 exact(p.wallet,'seedFile,stateDirectory,expectedAddress');absolute(p.wallet.seedFile);absolute(p.wallet.stateDirectory);check(typeof p.wallet.expectedAddress==='string'&&/^mn_addr_undeployed1[a-z0-9]+$/.test(p.wallet.expectedAddress),'LAUNCH_WALLET_ADDRESS');
 exact(p.roles,'firstAddress,secondAddress,secretsFile');absolute(p.roles.secretsFile);check(hex(p.roles.firstAddress)&&hex(p.roles.secondAddress)&&p.roles.firstAddress!==p.roles.secondAddress,'LAUNCH_ROLES');
 exact(p.privateState,'directory,passwordFile');absolute(p.privateState.directory);absolute(p.privateState.passwordFile);absolute(p.outputDirectory);
 const paths=[p.outputDirectory,p.privateState.directory,p.wallet.stateDirectory];check(new Set(paths).size===3&&paths.every((a,i)=>paths.every((b,j)=>i===j||!a.startsWith(b+'/'))),'LAUNCH_DISTINCT_DIRECTORIES');
 check(hex(p.networkTag)&&Number.isSafeInteger(p.expectedProtocolVersion)&&p.expectedProtocolVersion>=0,'LAUNCH_BINDINGS');
 exact(p.limits,'allocationId,deadlineMs,submissions,dustFee,grossByLogicalAsset');check(typeof p.limits.allocationId==='string'&&/^[a-zA-Z0-9_-]{1,80}$/.test(p.limits.allocationId),'LAUNCH_ALLOCATION');
 check(Number.isSafeInteger(p.limits.deadlineMs)&&p.limits.deadlineMs>Date.now()&&p.limits.deadlineMs<=Date.now()+3600000&&p.limits.submissions===(recovery?3:4)&&decimal(p.limits.dustFee),'LAUNCH_LIMITS');
 exact(p.limits.grossByLogicalAsset,p.kind==='loan'?'USD_TEST_ASSET':'ASSET_A,ASSET_B');check(Object.values(p.limits.grossByLogicalAsset).every(decimal),'LAUNCH_ASSET_LIMITS');
 if(recovery){
  validateExistingLoanPlan(p.existingDeployment,p);
  check(p.wallet.seedFile==='/home/charl/.local/share/moriarty/test-wallets/local-undeployed.seed'&&p.wallet.stateDirectory==='/home/charl/.local/share/moriarty/test-wallets/hello-world-dedicated-v2'&&p.wallet.expectedAddress==='mn_addr_undeployed1n2w7v4y79630m5u40rpm6tn0qvnm83vptu9vqcwzqppam7pfrr9sa6q9r9','LAUNCH_RECOVERY_ORIGINAL_WALLET');
  check(p.roles.firstAddress==='9a9de6549e2ea2fdd39578c3bd2e6f0327b3c5815f0ac061c20043ddf82918cb'&&p.roles.secondAddress==='c1d1141a7f08931d16f3fe4cec1c57d66ab2d11d04e4ab7abb61121ecad5e61e','LAUNCH_RECOVERY_ORIGINAL_ROLES');
  const roots=[p.outputDirectory,p.privateState.directory,p.wallet.stateDirectory,p.existingDeployment.inspectionDirectory];
  check(roots.every((a,i)=>roots.every((b,j)=>i===j||(a!==b&&!a.startsWith(b+'/')))),'LAUNCH_RECOVERY_DIRECTORY_OVERLAP');
  check(!p.privateState.passwordFile.startsWith(p.wallet.stateDirectory+'/'),'LAUNCH_RECOVERY_PRESERVE_WALLET');
 }
 return structuredClone(p);
}
function safeAncestors(p){for(let d=p;;d=dirname(d)){check(!lstatSync(d).isSymbolicLink(),'LAUNCH_SYMLINK');if(dirname(d)===d)break;}}
export function readPrivateLaunchFile(p){
 absolute(p);safeAncestors(p);let fd;
 // Nonblocking open lets fstat reject a FIFO even when it has no writer.
 try{fd=openSync(p,constants.O_RDONLY|constants.O_NOFOLLOW|constants.O_NONBLOCK);const s=fstatSync(fd);check(s.isFile()&&(s.mode&0o077)===0&&s.uid===process.getuid()&&s.size>0&&s.size<=32*1024*1024,'LAUNCH_PRIVATE_FILE');return readFileSync(fd);}
 finally{if(fd!==undefined)closeSync(fd);}
}
/** Apply the installed private-store policy before wallet restore or submission.
 * Pin the actual validator, never mirror its rules or expose its error text.
 */
export async function readLocalStoragePassword(file){
 const password=readPrivateLaunchFile(file).toString('utf8').trim();
 const entry=join(PINNED_NM,'@midnight-ntwrk/midnight-js-utils/dist/index.mjs');
 check(hash(readFileSync(entry))==='812b7644d7280797bd68bbf72b3c3bd64015158404ca5af29b8ecaec95f2c5de','LAUNCH_PASSWORD_POLICY_PIN');
 const {validatePassword}=await import(pathToFileURL(entry).href);
 try{validatePassword(password);}catch{throw Error('LAUNCH_PRIVATE_PASSWORD');}
 return password;
}
export function decodeSavedWalletEnvelope(wrapper){exact(wrapper,'version,state');check(wrapper.version===1&&typeof wrapper.state==='string'&&wrapper.state.length>0,'LAUNCH_SAVED_STATE');return wrapper.state;}
function privateJson(p){try{return JSON.parse(readPrivateLaunchFile(p));}catch{throw Error('LAUNCH_PRIVATE_JSON');}}
function privateDirectory(p){absolute(p);safeAncestors(p);const s=lstatSync(p);check(s.isDirectory()&&(s.mode&0o077)===0&&s.uid===process.getuid(),'LAUNCH_PRIVATE_DIRECTORY');}
function syncDirectory(p){const d=openSync(p,constants.O_RDONLY);try{fsyncSync(d);}finally{closeSync(d);}}
function durableBytes(p,bytes){const fd=openSync(p,constants.O_WRONLY|constants.O_CREAT|constants.O_EXCL|constants.O_NOFOLLOW,0o600);try{writeFileSync(fd,bytes);fsyncSync(fd);}finally{closeSync(fd);}syncDirectory(dirname(p));}
function durableFile(p,value){durableBytes(p,JSON.stringify(value)+'\n');}
/** Called only at the wallet submission boundary, after provider reservations.
 * Retains native finalized public payloads, never recipes or SDK private objects.
 */
export function retainPublicSubmissions({wallet,ledger,directory}){
 privateDirectory(directory);check(typeof wallet?.submitTransaction==='function','LAUNCH_SUBMIT_API');
 const submit=wallet.submitTransaction.bind(wallet);
 return new Proxy(wallet,{get(target,key){
  if(key==='submitTransaction')return async tx=>{
   check(tx instanceof ledger.Transaction&&Object.getPrototypeOf(tx)===ledger.Transaction.prototype,'LAUNCH_NATIVE_TRANSACTION');
   const raw=Buffer.from(tx.serialize());check(raw.length>0&&raw.length<=16*1024*1024,'LAUNCH_PUBLIC_TRANSACTION_SIZE');
   const canonical=ledger.Transaction.deserialize('signature','proof','binding',raw);
   check(Buffer.from(canonical.serialize()).equals(raw),'LAUNCH_NONCANONICAL_TRANSACTION');
   const transactionHash=canonical.transactionHash(),identifiers=[...canonical.identifiers()];check(hex(transactionHash)&&identifiers.length>0&&identifiers.length<=128&&identifiers.every(x=>typeof x==='string'&&/^[a-f0-9]{1,256}$/.test(x)),'LAUNCH_TRANSACTION_IDENTITY');
   const file=transactionHash+'.bin';
   durableBytes(join(directory,file),raw);
   durableFile(join(directory,transactionHash+'.json'),{schema:'moriarty.public-submission-bytes/1',transactionHash,identifiers,rawSha256:hash(raw),bytes:raw.length,file,scope:'Finalized public native bytes retained before wallet submission; submission and ledger acceptance unknown'});
   return submit(canonical);
  };
  const value=Reflect.get(target,key,target);return typeof value==='function'?value.bind(target):value;
 }});
}
/** Only the actual reviewed integration producer reaches this writer in the CLI.
 * That producer already closes nested observations/comparisons. This is retention,
 * not a sanitizer for caller-supplied SDK errors or private results.
 */
export function retainPublicIntegrationResult(directory,result){
 exact(result,'schema,status,kind,sourceTestOnly,networkAcceptance,proofAcceptance,financialAcceptance,build,assetBindings,phase,driver,cleanup,setupPendingOperations,comparisons,financialComparison,scope');
 check(result.schema==='moriarty.local-financial-integration/1'&&['PASS','FAILED','INCOMPLETE'].includes(result.status)&&result.sourceTestOnly===false&&result.networkAcceptance===false&&result.proofAcceptance===false&&result.financialAcceptance===false,'LAUNCH_PUBLIC_RESULT');
 if(result.driver!==undefined){
  const d=result.driver,diagnostic=Object.getOwnPropertyDescriptor(d,'failure');
  exact(d,'schema,status,kind,contractAddress,stages,transactionIds,cleanup,operationalState,scope'+(Object.hasOwn(d,'assetBindings')?',assetBindings':'')+(diagnostic?',failure':''));
  check(d.schema==='moriarty.local-financial-run/1','LAUNCH_PUBLIC_DRIVER');
  if(diagnostic){check(Object.hasOwn(diagnostic,'value')&&d.status==='FAILED','LAUNCH_PUBLIC_DRIVER_FAILURE');validatePublicDriverFailure(diagnostic.value);}
 }
 if(result.financialComparison!==undefined)exact(result.financialComparison,'status,kind,contractAddress,expectationsSha256,stages,scope,networkAcceptance,proofAcceptance');
 durableFile(join(directory,'integration-result.json'),structuredClone(result));
}
export function publicLaunchEvent(e){
 check(e&&['submitted','stopped'].includes(e.kind),'LAUNCH_EVENT_KIND');
 check(Array.isArray(e.identifiers)&&e.identifiers.length<=128&&e.identifiers.every(x=>typeof x==='string'&&/^[a-f0-9]{1,256}$/.test(x)),'LAUNCH_EVENT_IDS');
 if(e.kind==='submitted'){check(typeof e.txId==='string'&&e.identifiers.includes(e.txId)&&hex(e.transactionHash),'LAUNCH_EVENT_TX');return {kind:e.kind,txId:e.txId,identifiers:[...e.identifiers],transactionHash:e.transactionHash};}
 check(Number.isSafeInteger(e.pendingOperations)&&e.pendingOperations>=0&&e.reservationRetained===true,'LAUNCH_EVENT_STOP');
 return {kind:e.kind,identifiers:[...e.identifiers],reservationRetained:true,pendingOperations:e.pendingOperations};
}
export async function inspectLocalLaunchRuntime(){
 const pins=runtimePins();let files=0;
 for(const [name,pin] of Object.entries(pins))for(const [entry,digest] of Object.entries(pin.files)){check(hash(readFileSync(join(PINNED_NM,name,entry)))===digest,'LAUNCH_SDK_PIN');files++;}
 return {status:'SOURCE_INSPECTED',files,walletStarted:false};
}
async function runtime(){
 await inspectLocalLaunchRuntime();const pins=runtimePins();
 const load=name=>import(pathToFileURL(join(PINNED_NM,name,pins[name].entry)).href);
 const [sdk,ledger,network,rx,ws]=await Promise.all(['@midnight-ntwrk/wallet-sdk','@midnight-ntwrk/midnight-js-protocol','@midnight-ntwrk/midnight-js-network-id','rxjs','ws'].map(load));
 globalThis.WebSocket=ws.WebSocket;return {sdk,ledger,network,rx};
}
/** Imports exact SDK code but creates no wallet, key, proof or network request. */
export async function inspectLocalLaunchApi(){
 const {sdk,ledger,network,rx}=await runtime();
 check(['ShieldedWallet','UnshieldedWallet','DustWallet','createKeystore','NoOpTransactionHistoryStorage'].every(k=>typeof sdk[k]==='function')&&typeof sdk.WalletFacade.init==='function'&&typeof sdk.HDWallet.fromSeed==='function'&&typeof ledger.signingKeyFromBip340==='function'&&typeof network.setNetworkId==='function'&&typeof rx.firstValueFrom==='function','LAUNCH_SDK_API');
 return {status:'API_INSPECTED',walletStarted:false,keysCreated:false,networkRequests:0};
}
// No caller-supplied SDK/runtime adapters: only the fixed inspected SDK reaches execution.
export async function launchLocalFinancialCase(plan){
 const p=validateLocalLaunchPlan(plan);process.umask(0o077);
 privateDirectory(p.wallet.stateDirectory);privateDirectory(dirname(p.outputDirectory));privateDirectory(p.privateState.directory);
 // Inspect real proving assets before opening private wallet material.
 await inspectFinancialBuild({case:p.kind,...p.build});await inspectLocalLaunchRuntime();
 // Repeat the operational read-only gate inside the launcher, before seed/HD or writes.
 if(p.existingDeployment)await preflightLocalRecovery(p);
 const roleData=privateJson(p.roles.secretsFile);exact(roleData,'firstSecret,secondSecret');check(hex(roleData.firstSecret)&&hex(roleData.secondSecret),'LAUNCH_ROLE_SECRET');
 const password=await readLocalStoragePassword(p.privateState.passwordFile);
 const saved={};for(const kind of childKinds)saved[kind]=decodeSavedWalletEnvelope(privateJson(join(p.wallet.stateDirectory,'.midnight-wallet-state/undeployed',kind+'.json')));
 const seed=readPrivateLaunchFile(p.wallet.seedFile);let seedHex=seed.toString('utf8').trim();check(/^[a-f0-9]{64}$/.test(seedHex),'LAUNCH_EXISTING_SEED');
 mkdirSync(p.outputDirectory,{mode:0o700}); // Exclusive run directory, never resume or retry automatically.
 syncDirectory(dirname(p.outputDirectory));
 durableFile(join(p.outputDirectory,'plan.json'),p);
 let sequence=0,wallet,ownershipTransferred=false,phase='restore';const children=[];
 const append=value=>durableFile(join(p.outputDirectory,`${String(++sequence).padStart(3,'0')}.json`),value);
 const deadline=p.limits.deadlineMs;
 const within=async fn=>{check(Date.now()<deadline,'LAUNCH_DEADLINE');let t;try{return await Promise.race([Promise.resolve().then(fn),new Promise((_,reject)=>{t=setTimeout(()=>reject(Error('LAUNCH_DEADLINE')),Math.max(1,deadline-Date.now()));})]);}finally{clearTimeout(t);}};
 try{
  const {sdk,ledger,network,rx}=await within(runtime);network.setNetworkId('undeployed');
  const tipOptions={node:p.networkConfig.node,indexer:p.networkConfig.indexer,deadlineMs:deadline};
  phase='tip-readiness';append(await within(()=>waitForLocalTip(tipOptions)));phase='restore';
  const hd=sdk.HDWallet.fromSeed(Buffer.from(seedHex,'hex'));seed.fill(0);seedHex=undefined;check(hd.type==='seedOk','LAUNCH_SEED_DERIVATION');
  let keys;try{const result=hd.hdWallet.selectAccount(0).selectRoles([sdk.Roles.Zswap,sdk.Roles.NightExternal,sdk.Roles.Dust]).deriveKeysAt(0);check(result.type==='keysDerived','LAUNCH_SEED_DERIVATION');keys=result.keys;}finally{hd.hdWallet.clear();}
  const shieldedSecretKeys=ledger.ZswapSecretKeys.fromSeed(keys[sdk.Roles.Zswap]),dustSecretKey=ledger.DustSecretKey.fromSeed(keys[sdk.Roles.Dust]),unshieldedKeystore=sdk.createKeystore(keys[sdk.Roles.NightExternal],'undeployed');
  check(unshieldedKeystore.getBech32Address().asString()===p.wallet.expectedAddress&&unshieldedKeystore.getAddress()===p.roles.firstAddress,'LAUNCH_EXISTING_IDENTITY');
  const signingBytes=unshieldedKeystore.getSecretKey();let deploymentSigningKey;try{deploymentSigningKey=ledger.signingKeyFromBip340(signingBytes);}finally{signingBytes.fill(0);}
  check(ledger.signatureVerifyingKey(deploymentSigningKey)===unshieldedKeystore.getPublicKey(),'LAUNCH_SIGNING_IDENTITY');
  const restore=(factory,kind)=>async c=>{const child=await factory(c).restore(saved[kind]);children.push(child);if(Date.now()>=deadline){await child.stop();throw Error('LAUNCH_DEADLINE');}return child;};
  wallet=await within(()=>sdk.WalletFacade.init({configuration:{networkId:'undeployed',indexerClientConnection:{indexerHttpUrl:p.networkConfig.indexer,indexerWsUrl:p.networkConfig.indexerWS},provingServerUrl:new URL(p.networkConfig.proofServer),relayURL:new URL(p.networkConfig.node.replace(/^http/,'ws')),txHistoryStorage:new sdk.NoOpTransactionHistoryStorage(),costParameters:{additionalFeeOverhead:300000000000000n,feeBlocksMargin:5}},shielded:restore(sdk.ShieldedWallet,'shielded'),unshielded:restore(sdk.UnshieldedWallet,'unshielded'),dust:restore(sdk.DustWallet,'dust')}).then(async value=>{if(Date.now()>=deadline){await value.stop();throw Error('LAUNCH_DEADLINE');}return value;}));
  const restored=await within(()=>rx.firstValueFrom(wallet.unshielded.state));
  check(restored.state.networkId==='undeployed'&&restored.state.publicKey.publicKey===unshieldedKeystore.getPublicKey()&&restored.state.publicKey.addressHex===p.roles.firstAddress&&restored.state.publicKey.address===p.wallet.expectedAddress,'LAUNCH_RESTORED_IDENTITY');
  phase='sync';await within(()=>wallet.start(shieldedSecretKeys,dustSecretKey));const synced=await within(()=>wallet.waitForSyncedState());check(synced.isSynced===true&&synced.dust.availableCoins.length>0,'LAUNCH_REGISTERED_DUST_REQUIRED');
 const submissionDirectory=join(p.outputDirectory,'public-transactions');mkdirSync(submissionDirectory,{mode:0o700});syncDirectory(p.outputDirectory);
  const guardedWallet=guardLocalWallet({wallet,...tipOptions,onTip:append});
  const retainedWallet=retainPublicSubmissions({wallet:guardedWallet,ledger,directory:submissionDirectory});
  phase='integration';ownershipTransferred=true;
  const options={kind:p.kind,build:p.build,walletContext:{wallet:retainedWallet,shieldedSecretKeys,dustSecretKey,unshieldedKeystore},deploymentSigningKey,networkConfig:p.networkConfig,roles:{firstAddress:p.roles.firstAddress,secondAddress:p.roles.secondAddress,firstSecret:Uint8Array.from(Buffer.from(roleData.firstSecret,'hex')),secondSecret:Uint8Array.from(Buffer.from(roleData.secondSecret,'hex'))},networkTag:p.networkTag,expectedProtocolVersion:p.expectedProtocolVersion,privateStateConfig:{midnightDbName:p.privateState.directory,privateStateStoreName:'sp05-'+p.kind,privateStoragePasswordProvider:()=>password},limits:{...p.limits,dustFee:BigInt(p.limits.dustFee),grossByLogicalAsset:Object.fromEntries(Object.entries(p.limits.grossByLogicalAsset).map(([k,v])=>[k,BigInt(v)])),reservationStatePath:join(p.outputDirectory,'reservations.json')},now:()=>BigInt(Math.floor(Date.now()/1000)),onEvent:e=>append(publicLaunchEvent(e)),onStage:summary=>{check(summary?.status==='PASS'&&['deploy','initialize','accrue','settle','swap','close'].includes(summary.stage),'LAUNCH_STAGE');durableFile(join(p.outputDirectory,'stage-'+summary.stage+'.json'),summary);return {status:'RECORDED',stage:summary.stage,txId:summary.txId};}};
  let result;
  if(p.existingDeployment)options.recoveryPlan=p;
  try{result=await integrateLocalFinancialCase(options);}
  catch(error){if(error.publicIntegrationResult!==undefined)retainPublicIntegrationResult(p.outputDirectory,error.publicIntegrationResult);throw Error('LOCAL_FINANCIAL_RUN_INCOMPLETE');}
  retainPublicIntegrationResult(p.outputDirectory,result);return result;
 }catch{append({kind:'failure',phase,networkAcceptance:false,proofAcceptance:false,financialAcceptance:false});throw Error('LOCAL_FINANCIAL_LAUNCH_FAILED');}
 finally{seed.fill(0);if(!ownershipTransferred){let t;try{await Promise.race([Promise.allSettled((wallet?[wallet]:children).map(w=>w.stop())),new Promise((_,reject)=>{t=setTimeout(()=>reject(Error('STOP_TIMEOUT')),5000);})]);}catch{/* Outer process containment remains required. */}finally{clearTimeout(t);}}}
}
if(process.argv[1]&&pathToFileURL(resolve(process.argv[1])).href===import.meta.url){
 let hardTimer;
 try{const a=process.argv.slice(2);check(a.length===5&&a[0]==='--run'&&a[1]==='--plan'&&a[3]==='--sha256'&&hex(a[4]),'LAUNCH_USAGE');const raw=readPrivateLaunchFile(a[2]);check(hash(raw)===a[4],'LAUNCH_PLAN_HASH');const plan=validateLocalLaunchPlan(JSON.parse(raw));hardTimer=setTimeout(()=>process.exit(124),Math.max(1,plan.limits.deadlineMs-Date.now()+6000));const result=await launchLocalFinancialCase(plan);process.stdout.write(JSON.stringify({status:result.status,networkAcceptance:false,proofAcceptance:false,financialAcceptance:false})+'\n');process.exitCode=result.status==='PASS'?0:2;}
 catch{process.stderr.write('Local financial launch failed; inspect retained public run records.\n');process.exitCode=1;}
 finally{clearTimeout(hardTimer);}
 process.exit(process.exitCode??0);
}
