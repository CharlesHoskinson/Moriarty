/** Existing Preview wallet bootstrap. No implicit wallet creation or recovery.
 * Actual invocation requires the separately admitted public plan and containment.
 */
import {readFileSync,mkdirSync,lstatSync,renameSync,unlinkSync,existsSync,openSync,writeFileSync,closeSync,fsyncSync,constants} from 'node:fs';
import {join,dirname,resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
import {createHash} from 'node:crypto';
import {validatePreviewLaunchPlan,retainPreviewIntegrationResult,retainPreviewStage,publicPreviewLaunchEvent} from './preview-launch.mjs';
import {readPrivateLaunchFile,readLocalStoragePassword,decodeSavedWalletEnvelope,inspectLocalLaunchRuntime,retainPublicSubmissions} from './launch-local.mjs';
import {inspectFinancialBuild} from './proven-assets.mjs';
import {createPreviewRpc} from './financial-rpc.mjs';
import {integratePreviewFinancialCase} from './integrate-preview.mjs';
import {PINNED_NM} from './providers.mjs';
const check=(ok,code)=>{if(!ok)throw Error(code);};
const hash=x=>createHash('sha256').update(x).digest('hex');
const hex=x=>typeof x==='string'&&/^[a-f0-9]{64}$/.test(x);
const childKinds=['shielded','unshielded','dust'];
const pendingMarker='.moriarty-persistence-pending.json';
function privateDirectory(path){for(let p=path;;p=dirname(p)){check(!lstatSync(p).isSymbolicLink(),'PREVIEW_LAUNCH_SYMLINK');if(dirname(p)===p)break;}const s=lstatSync(path);check(s.isDirectory()&&s.uid===process.getuid()&&(s.mode&0o077)===0,'PREVIEW_LAUNCH_DIRECTORY');}
function syncDirectory(path){const fd=openSync(path,constants.O_RDONLY);try{fsyncSync(fd);}finally{closeSync(fd);}}
function retain(path,value){const fd=openSync(path,constants.O_WRONLY|constants.O_CREAT|constants.O_EXCL|constants.O_NOFOLLOW,0o600);try{writeFileSync(fd,JSON.stringify(value)+'\n');fsyncSync(fd);}finally{closeSync(fd);}syncDirectory(dirname(path));}
function deadlineCheck(deadline){check(Number.isSafeInteger(deadline)&&Date.now()<deadline,'PREVIEW_LAUNCH_DEADLINE');}
async function within(deadline,fn){deadlineCheck(deadline);let timer;try{const result=await Promise.race([Promise.resolve().then(()=>{deadlineCheck(deadline);return fn();}),new Promise((_,reject)=>{timer=setTimeout(()=>reject(Error('PREVIEW_LAUNCH_DEADLINE')),Math.max(1,Math.min(2147483647,deadline-Date.now())));})]);deadlineCheck(deadline);return result;}finally{clearTimeout(timer);}}
/** Public checks only, before any private wallet reads or new private paths. */
export async function preflightPreviewLaunch(plan){
 const p=validatePreviewLaunchPlan(plan),deadline=p.limits.deadlineMs;
 await within(deadline,()=>inspectFinancialBuild({case:p.kind,...p.build}));
 await within(deadline,inspectLocalLaunchRuntime);
 const genesis=await within(deadline,()=>createPreviewRpc({node:p.networkConfig.node,deadlineMs:Math.min(deadline,Date.now()+15000)})('chain_getBlockHash',[0]));
 check(genesis==='0x'+p.networkTag,'PREVIEW_GENESIS_MISMATCH');
 return {status:'PUBLIC_PREFLIGHT_CHECKED',genesis,network:'preview',walletStarted:false,networkAcceptance:false};
}
async function runtime(){
 await inspectLocalLaunchRuntime();
 const raw=readFileSync(new URL('./launch-runtime-pins.json',import.meta.url));check(hash(raw)==='4fa41776e0fce393bf7c6ee19acd824bec0b9459ed4a93616cef35904e548195','PREVIEW_RUNTIME_MANIFEST_PIN');const pins=JSON.parse(raw);
 const load=name=>import(pathToFileURL(join(PINNED_NM,name,pins[name].entry)).href);
 const [sdk,ledger,network,rx,ws]=await Promise.all(['@midnight-ntwrk/wallet-sdk','@midnight-ntwrk/midnight-js-protocol','@midnight-ntwrk/midnight-js-network-id','rxjs','ws'].map(load));
 globalThis.WebSocket=ws.WebSocket;return {sdk,ledger,network,rx};
}
/** Persist the actual SDK serializeState strings in the existing version1 format.
 * Original snapshots stay private in an exclusive backup. A pending marker makes
 * any interrupted three-file replacement fail closed at the next launch.
 */
export async function persistPreviewWalletState({wallet,stateDirectory,allocationId,original,deadlineMs}){
 check(resolve(stateDirectory)===stateDirectory&&typeof allocationId==='string'&&/^[a-zA-Z0-9_-]{1,80}$/.test(allocationId),'PREVIEW_PERSIST_BINDING');
 const parent=join(stateDirectory,'.midnight-wallet-state/preview');privateDirectory(parent);deadlineCheck(deadlineMs);
 check(!existsSync(join(parent,pendingMarker)),'PREVIEW_PERSIST_PENDING');
 const next={};
 for(const kind of childKinds){check(Buffer.isBuffer(original[kind])&&readPrivateLaunchFile(join(parent,kind+'.json')).equals(original[kind]),'PREVIEW_PERSIST_ORIGINAL_CHANGED');const state=await within(deadlineMs,()=>wallet[kind].serializeState());decodeSavedWalletEnvelope({version:1,state});next[kind]=Buffer.from(JSON.stringify({version:1,state})+'\n');check(next[kind].length<=32*1024*1024,'PREVIEW_PERSIST_BOUND');}
 const backup=join(parent,'.moriarty-backup-'+allocationId);mkdirSync(backup,{mode:0o700});syncDirectory(parent);
 const save=(file,raw)=>{const fd=openSync(file,constants.O_WRONLY|constants.O_CREAT|constants.O_EXCL|constants.O_NOFOLLOW,0o600);try{writeFileSync(fd,raw);fsyncSync(fd);}finally{closeSync(fd);}syncDirectory(dirname(file));};
 for(const kind of childKinds){save(join(backup,kind+'.json'),original[kind]);save(join(backup,'next-'+kind+'.json'),next[kind]);}
 deadlineCheck(deadlineMs);retain(join(parent,pendingMarker),{schema:'moriarty.preview-wallet-persistence-pending/1',allocationId});
 for(const kind of childKinds){deadlineCheck(deadlineMs);check(readPrivateLaunchFile(join(parent,kind+'.json')).equals(original[kind]),'PREVIEW_PERSIST_ORIGINAL_CHANGED');renameSync(join(backup,'next-'+kind+'.json'),join(parent,kind+'.json'));}
 for(const kind of childKinds)check(readPrivateLaunchFile(join(parent,kind+'.json')).equals(next[kind]),'PREVIEW_PERSIST_VERIFY');
 syncDirectory(parent);deadlineCheck(deadlineMs);unlinkSync(join(parent,pendingMarker));syncDirectory(parent);deadlineCheck(deadlineMs);
 return {status:'PERSISTED',children:3};
}
export async function launchPreviewFinancialCase(plan){
 const p=validatePreviewLaunchPlan(plan);process.umask(0o077);const deadline=p.limits.deadlineMs;
 const preflight=await preflightPreviewLaunch(p);
 privateDirectory(dirname(p.outputDirectory));privateDirectory(dirname(p.privateState.directory));
 mkdirSync(p.outputDirectory,{mode:0o700});syncDirectory(dirname(p.outputDirectory));
 retain(join(p.outputDirectory,'plan.json'),p);retain(join(p.outputDirectory,'public-preflight.json'),preflight);
 let sequence=0,phase='private-inputs',wallet,ownershipTransferred=false,seed;const children=[];
 const append=value=>retain(join(p.outputDirectory,`${String(++sequence).padStart(3,'0')}.json`),value);
 try{
  // Both directory reservations are exclusive. Existing contract stores are never opened.
  mkdirSync(p.privateState.directory,{mode:0o700});syncDirectory(dirname(p.privateState.directory));
  privateDirectory(p.wallet.stateDirectory);check(!existsSync(join(p.wallet.stateDirectory,'.midnight-wallet-state/preview',pendingMarker)),'PREVIEW_PERSIST_PENDING');deadlineCheck(deadline);
  const roleData=JSON.parse(readPrivateLaunchFile(p.roles.secretsFile));
  check(roleData&&Object.keys(roleData).sort().join(',')==='firstSecret,secondSecret'&&hex(roleData.firstSecret)&&hex(roleData.secondSecret),'PREVIEW_ROLE_SECRETS');
  const password=await within(deadline,()=>readLocalStoragePassword(p.privateState.passwordFile));
  const saved={},original={};for(const kind of childKinds){original[kind]=readPrivateLaunchFile(join(p.wallet.stateDirectory,'.midnight-wallet-state/preview',kind+'.json'));saved[kind]=decodeSavedWalletEnvelope(JSON.parse(original[kind]));}
  seed=readPrivateLaunchFile(p.wallet.seedFile);let seedHex=seed.toString('utf8').trim();check(hex(seedHex),'PREVIEW_EXISTING_SEED');
  phase='restore';const {sdk,ledger,network,rx}=await within(deadline,runtime);network.setNetworkId('preview');
  const hd=sdk.HDWallet.fromSeed(Buffer.from(seedHex,'hex'));seed.fill(0);seedHex=undefined;check(hd.type==='seedOk','PREVIEW_SEED_DERIVATION');
  let keys;try{const result=hd.hdWallet.selectAccount(0).selectRoles([sdk.Roles.Zswap,sdk.Roles.NightExternal,sdk.Roles.Dust]).deriveKeysAt(0);check(result.type==='keysDerived','PREVIEW_SEED_DERIVATION');keys=result.keys;}finally{hd.hdWallet.clear();}
  const shieldedSecretKeys=ledger.ZswapSecretKeys.fromSeed(keys[sdk.Roles.Zswap]),dustSecretKey=ledger.DustSecretKey.fromSeed(keys[sdk.Roles.Dust]),unshieldedKeystore=sdk.createKeystore(keys[sdk.Roles.NightExternal],'preview');
  check(unshieldedKeystore.getBech32Address().asString()===p.wallet.expectedAddress&&unshieldedKeystore.getAddress()===p.roles.firstAddress,'PREVIEW_EXISTING_IDENTITY');
  const signingBytes=unshieldedKeystore.getSecretKey();let deploymentSigningKey;try{deploymentSigningKey=ledger.signingKeyFromBip340(signingBytes);}finally{signingBytes.fill(0);}
  check(ledger.signatureVerifyingKey(deploymentSigningKey)===unshieldedKeystore.getPublicKey(),'PREVIEW_SIGNING_IDENTITY');
  const restore=(factory,kind)=>async config=>{const child=await factory(config).restore(saved[kind]);children.push(child);if(Date.now()>=deadline){await child.stop();throw Error('PREVIEW_LAUNCH_DEADLINE');}return child;};
  wallet=await within(deadline,()=>sdk.WalletFacade.init({configuration:{networkId:'preview',indexerClientConnection:{indexerHttpUrl:p.networkConfig.indexer,indexerWsUrl:p.networkConfig.indexerWS},provingServerUrl:new URL(p.networkConfig.proofServer),relayURL:new URL('wss://rpc.preview.midnight.network'),txHistoryStorage:new sdk.NoOpTransactionHistoryStorage(),costParameters:{additionalFeeOverhead:300000000000000n,feeBlocksMargin:5}},shielded:restore(sdk.ShieldedWallet,'shielded'),unshielded:restore(sdk.UnshieldedWallet,'unshielded'),dust:restore(sdk.DustWallet,'dust')}).then(async value=>{if(Date.now()>=deadline){await value.stop();throw Error('PREVIEW_LAUNCH_DEADLINE');}return value;}));
  const restored=await within(deadline,()=>rx.firstValueFrom(wallet.unshielded.state));
  check(restored.state.networkId==='preview'&&restored.state.publicKey.publicKey===unshieldedKeystore.getPublicKey()&&restored.state.publicKey.addressHex===p.roles.firstAddress&&restored.state.publicKey.address===p.wallet.expectedAddress,'PREVIEW_RESTORED_IDENTITY');
  phase='sync';await within(deadline,()=>wallet.start(shieldedSecretKeys,dustSecretKey));const synced=await within(deadline,()=>wallet.waitForSyncedState());
  check(synced.isSynced===true&&synced.dust.availableCoins.length>0,'PREVIEW_REGISTERED_DUST_REQUIRED');
  for(const kind of childKinds)check(synced[kind]?.progress?.isConnected===true&&synced[kind].progress.isStrictlyComplete()===true,'PREVIEW_SYNC_INCOMPLETE');
  check(Array.isArray(synced.unshielded.pendingCoins)&&synced.unshielded.pendingCoins.length===0&&Array.isArray(synced.dust.state?.pendingDust)&&synced.dust.state.pendingDust.length===0,'PREVIEW_PENDING_WALLET');
  const submissionDirectory=join(p.outputDirectory,'public-transactions');mkdirSync(submissionDirectory,{mode:0o700});syncDirectory(p.outputDirectory);
  let stopping;const persistentWallet=new Proxy(wallet,{get(target,key){if(key==='stop')return()=>stopping??=(async()=>{try{append(await persistPreviewWalletState({wallet,stateDirectory:p.wallet.stateDirectory,allocationId:p.limits.allocationId,original,deadlineMs:Math.min(deadline+5000,Date.now()+5000)}));}finally{await target.stop();}})();const value=Reflect.get(target,key,target);return typeof value==='function'?value.bind(target):value;}});
  const retainedWallet=retainPublicSubmissions({wallet:persistentWallet,ledger,directory:submissionDirectory});
  const options={kind:p.kind,build:p.build,walletContext:{wallet:retainedWallet,shieldedSecretKeys,dustSecretKey,unshieldedKeystore},deploymentSigningKey,networkConfig:p.networkConfig,roles:{firstAddress:p.roles.firstAddress,secondAddress:p.roles.secondAddress,firstSecret:Uint8Array.from(Buffer.from(roleData.firstSecret,'hex')),secondSecret:Uint8Array.from(Buffer.from(roleData.secondSecret,'hex'))},networkTag:p.networkTag,expectedProtocolVersion:p.expectedProtocolVersion,privateStateConfig:{midnightDbName:p.privateState.directory,privateStateStoreName:'sp05-'+p.kind,privateStoragePasswordProvider:()=>password},limits:{...p.limits,dustFee:BigInt(p.limits.dustFee),grossByLogicalAsset:Object.fromEntries(Object.entries(p.limits.grossByLogicalAsset).map(([k,v])=>[k,BigInt(v)])),reservationStatePath:join(p.outputDirectory,'reservations.json')},now:()=>BigInt(Math.floor(Date.now()/1000)),onEvent:e=>append(publicPreviewLaunchEvent(e)),onStage:s=>retainPreviewStage(p.outputDirectory,s)};
  phase='integration';ownershipTransferred=true;let result;
  try{result=await integratePreviewFinancialCase(options);}catch(error){if(error.publicIntegrationResult!==undefined)retainPreviewIntegrationResult(p.outputDirectory,error.publicIntegrationResult);throw Error('PREVIEW_FINANCIAL_RUN_INCOMPLETE');}
  retainPreviewIntegrationResult(p.outputDirectory,result);deadlineCheck(deadline);return result;
 }catch{append({kind:'failure',phase,networkAcceptance:false,proofAcceptance:false,financialAcceptance:false});throw Error('PREVIEW_FINANCIAL_LAUNCH_FAILED');}
 finally{seed?.fill(0);if(!ownershipTransferred){let timer;try{await Promise.race([Promise.allSettled((wallet?[wallet]:children).map(w=>w.stop())),new Promise((_,reject)=>{timer=setTimeout(()=>reject(Error('STOP_TIMEOUT')),5000);})]);}catch{/* Independent outer containment remains required. */}finally{clearTimeout(timer);}}}
}
if(process.argv[1]&&pathToFileURL(resolve(process.argv[1])).href===import.meta.url){
 let timer;
 try{const args=process.argv.slice(2);check(args.length===5&&args[0]==='--run'&&args[1]==='--plan'&&args[3]==='--sha256'&&hex(args[4]),'PREVIEW_LAUNCH_USAGE');const raw=readPrivateLaunchFile(args[2]);check(hash(raw)===args[4],'PREVIEW_PLAN_HASH');const p=validatePreviewLaunchPlan(JSON.parse(raw));timer=setTimeout(()=>process.exit(124),Math.max(1,p.limits.deadlineMs-Date.now()+6000));const r=await launchPreviewFinancialCase(p);process.stdout.write(JSON.stringify({status:r.status,networkAcceptance:false,proofAcceptance:false,financialAcceptance:false})+'\n');process.exitCode=r.status==='PASS'?0:2;}
 catch{process.stderr.write('Preview financial launch failed; inspect retained public run records.\n');process.exitCode=1;}
 finally{clearTimeout(timer);}process.exit(process.exitCode??0);
}
