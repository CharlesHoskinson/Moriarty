/** Closed Preview launch descriptors and durable public evidence only.
 * No wallet construction, private input reads, network dispatch or admission.
 */
import {readFileSync,lstatSync,openSync,writeFileSync,closeSync,fsyncSync,constants} from 'node:fs';
import {createHash} from 'node:crypto';
import {isDeepStrictEqual} from 'node:util';
import {dirname,join,isAbsolute,resolve} from 'node:path';
import {decodePreviewIndexedOwner} from './indexed-owner.mjs';
import {validatePublicDriverFailure} from './run-local.mjs';
import {validatePublicIntegrationFailureCode} from './integrate-local.mjs';
const check=(ok,code)=>{if(!ok)throw Error(code);};
const hash=x=>typeof x==='string'&&/^[a-f0-9]{64}$/.test(x);
const identifier=x=>typeof x==='string'&&/^(?:[a-f0-9]{64}|[a-f0-9]{66})$/.test(x);
const decimal=x=>typeof x==='string'&&/^(0|[1-9][0-9]*)$/.test(x)&&x.length<=39&&BigInt(x)<1n<<128n;
function fields(o){check(o&&Object.getPrototypeOf(o)===Object.prototype,'PREVIEW_FIELDS');const d=Object.getOwnPropertyDescriptors(o);check(Reflect.ownKeys(d).length===Object.keys(d).length&&Object.values(d).every(v=>Object.hasOwn(v,'value')&&v.enumerable),'PREVIEW_FIELDS');return Object.keys(d);}
function exact(o,keys){check(fields(o).sort().join(',')===keys.split(',').sort().join(','),'PREVIEW_FIELDS');}
const absolute=x=>check(typeof x==='string'&&isAbsolute(x)&&resolve(x)===x&&!x.includes('\0'),'PREVIEW_PATH');
const inside=(a,b)=>a===b||a.startsWith(b+'/');
function identity(){const raw=readFileSync(new URL('../../../evidence/midnight-preview-2026-09-07/wallet-public.json',import.meta.url));check(createHash('sha256').update(raw).digest('hex')==='01a31bab9f9920f1385f3fef92fee7638b477f77875390613ebb19e1a786fc98','PREVIEW_PUBLIC_IDENTITY_PIN');return JSON.parse(raw);}
export function validatePreviewLaunchPlan(p){
 exact(p,'schema,kind,build,networkConfig,wallet,roles,privateState,networkTag,expectedProtocolVersion,limits,outputDirectory');check(p.schema==='moriarty.preview-financial-launch/1'&&['loan','swap'].includes(p.kind),'PREVIEW_PLAN_SCHEMA');
 exact(p.build,'receiptPath,receiptSha256,sourceManifestHash');absolute(p.build.receiptPath);check(hash(p.build.receiptSha256)&&hash(p.build.sourceManifestHash),'PREVIEW_BUILD_HASH');
 exact(p.networkConfig,'networkId,node,indexer,indexerWS,proofServer');const n=p.networkConfig;check(n.networkId==='preview'&&['https://rpc.preview.midnight.network','https://rpc.preview.midnight.network/'].includes(n.node)&&n.indexer==='https://indexer.preview.midnight.network/api/v4/graphql'&&n.indexerWS==='wss://indexer.preview.midnight.network/api/v4/graphql/ws','PREVIEW_NETWORK');
 check(typeof n.proofServer==='string','PREVIEW_PROOF_ENDPOINT');const proof=new URL(n.proofServer);check(proof.protocol==='http:'&&['127.0.0.1','localhost','[::1]'].includes(proof.hostname)&&!proof.username&&!proof.password&&!proof.hash&&!proof.search&&proof.pathname==='/','PREVIEW_PROOF_ENDPOINT');
 exact(p.wallet,'seedFile,stateDirectory,expectedAddress');absolute(p.wallet.seedFile);absolute(p.wallet.stateDirectory);const original=identity();check(p.wallet.seedFile===original.seedFile&&p.wallet.expectedAddress===original.address,'PREVIEW_ORIGINAL_WALLET');
 exact(p.roles,'firstAddress,secondAddress,secretsFile');absolute(p.roles.secretsFile);check(hash(p.roles.firstAddress)&&hash(p.roles.secondAddress)&&p.roles.firstAddress===decodePreviewIndexedOwner(original.address)&&p.roles.firstAddress!==p.roles.secondAddress,'PREVIEW_ROLES');
 exact(p.privateState,'directory,passwordFile');absolute(p.privateState.directory);absolute(p.privateState.passwordFile);absolute(p.outputDirectory);
 const dirs=[p.outputDirectory,p.privateState.directory,p.wallet.stateDirectory];check(dirs.every((a,i)=>dirs.every((b,j)=>i===j||!inside(a,b))),'PREVIEW_DIRECTORY_OVERLAP');
 for(const f of [p.wallet.seedFile,p.roles.secretsFile,p.privateState.passwordFile,p.build.receiptPath])check(!inside(f,p.outputDirectory)&&!inside(f,p.privateState.directory),'PREVIEW_INPUT_OUTPUT_OVERLAP');
 check(hash(p.networkTag)&&Number.isSafeInteger(p.expectedProtocolVersion)&&p.expectedProtocolVersion>=0,'PREVIEW_BINDINGS');
 exact(p.limits,'allocationId,deadlineMs,submissions,dustFee,grossByLogicalAsset');const l=p.limits,now=Date.now();check(typeof l.allocationId==='string'&&/^sp05-preview-[a-zA-Z0-9_-]{1,64}$/.test(l.allocationId),'PREVIEW_ALLOCATION');check(Number.isSafeInteger(l.deadlineMs)&&l.deadlineMs>now&&l.deadlineMs<=now+3600000&&l.submissions===4&&decimal(l.dustFee),'PREVIEW_LIMITS');exact(l.grossByLogicalAsset,p.kind==='loan'?'USD_TEST_ASSET':'ASSET_A,ASSET_B');check(Object.values(l.grossByLogicalAsset).every(decimal),'PREVIEW_ASSET_LIMITS');return structuredClone(p);
}
const KEYS=new Set('ASSET_A ASSET_B USD_TEST_ASSET acceptance actions address after amount asset assetADomain assetBDomain assetBindings blockHash blockHeight borrower borrowerAddress borrowerCapability build bytes circuitId claimedUnshieldedSpends cleanup colorA colorB comparisons containmentComplete contractAddress contractBalances driver dustFee effect0 effect1 effect2 effects encoding entryPoint estimated expectationsSha256 f0 f1 f2 f3 f4 f5 f6 f7 f8 failure failureCode fees finalizedHead finalizedHeight financialAcceptance financialComparison grossByAsset identifiers indexerFees indexerIdentifiers indexerReported initialized inputs intentHash kernelState kind lastAccrue lastClose lastSettle lastSwap ledgerAccepted lender lenderAddress lenderCapability nativeDebit nativeDebitRelationship nativeEffects nativeFee networkAcceptance networkTag offerIndex operationalState outputNo outputs owner paid participantNetDeltas pendingOperations phase programDigest proofAcceptance proofVerified protocolVersion provider providerAddress providerCapability publicState rawSha256 receiptSha256 recipient recipientKind remaining reservedDustFee reservedGrossByAsset reservedSubmissions revision schema scope section segment setupPendingOperations sourceManifestHash sourceTestOnly sourceUnitLabel stage stages status trader traderAddress traderCapability transaction transactionHash transactionIds transcripts txId type unit unshieldedInputs unshieldedMints unshieldedOutputs usdColor usdDomain v0 v1 v2 v3 v4 v5 value walletStopped unavailable code'.split(' '));
const SCOPES=new Set(['All four stages match independent fixed loan/swap expectations; external observer, indexer and RPC trust remain; no PCD or proof acceptance','Exact fixed financial trace comparison over supplied finalized observations; no proof or network acceptance','Fixed I2 composition; source adapters and incomplete containment never establish financial network acceptance','Fixed I2 execution and supplied financial comparison; not mandatory PCD acceptance','Fixed Preview composition; only independently reviewed actual evidence establishes financial acceptance']);
const VALUES=new Set('DUST SPECK unresolved uncertified-I2-observation loan swap deploy initialize accrue settle close call user contract guaranteed fallible PASS FINANCIAL_COMPLETE FAILED INCOMPLETE SOURCE_TEST_ONLY preflight allocate driver load-assets prepare-deployment providers public-identity observe compare'.split(' '));
const HASH_FIELDS=new Set('rawSha256 transactionHash contractAddress address bytes owner intentHash programDigest networkTag usdColor usdDomain colorA colorB assetADomain assetBDomain borrowerCapability lenderCapability traderCapability providerCapability receiptSha256 sourceManifestHash expectationsSha256 type'.split(' '));
const ID_FIELDS=new Set(['txId','transactionIds','identifiers','indexerIdentifiers']);
const PREVIEW_FAILURES=new Set(['PREVIEW_NETWORK_CONFIG','PREVIEW_PUBLIC_IDENTITY_PIN','PREVIEW_WALLET_IDENTITY','PREVIEW_GENESIS_MISMATCH','PREVIEW_RECOVERY_FORBIDDEN','PREVIEW_INTEGRATION_INCOMPLETE','PREVIEW_INTEGRATION_FAILURE','INTEGRATION_DEADLINE']);
function failureCode(value){if(PREVIEW_FAILURES.has(value))return value;return validatePublicIntegrationFailureCode(value);}
function publicClone(value){let count=0;const seen=new Set();function clone(v,key='',depth=0,arrayItem=false){check(++count<=50000&&depth<=24,'PREVIEW_PUBLIC_SIZE');
 if(['networkAcceptance','proofAcceptance','financialAcceptance','proofVerified','ledgerAccepted'].includes(key))check(v===false,'PREVIEW_PUBLIC_ACCEPTANCE');
 if(ID_FIELDS.has(key))check(key==='txId'||arrayItem?typeof v==='string':Array.isArray(v),'PREVIEW_PUBLIC_IDENTIFIER');
 if(HASH_FIELDS.has(key))check(typeof v==='string'||key==='contractAddress'&&v===undefined,'PREVIEW_PUBLIC_HASH');
 if(['blockHash','finalizedHead','schema','scope','failureCode','code'].includes(key))check(typeof v==='string','PREVIEW_PUBLIC_STRING');
 if(v===undefined||v===null)return v;if(typeof v==='boolean')return v;if(typeof v==='number'){check(Number.isSafeInteger(v)&&v>=0,'PREVIEW_PUBLIC_NUMBER');return v;}
 if(typeof v==='string'){
  if(key==='scope')check(SCOPES.has(v),'PREVIEW_PUBLIC_SCOPE');
  else if(key==='failureCode')failureCode(v);
  else if(ID_FIELDS.has(key))check(identifier(v),'PREVIEW_PUBLIC_IDENTIFIER');
  else if(HASH_FIELDS.has(key))check(hash(v),'PREVIEW_PUBLIC_HASH');
  else if(key==='blockHash'||key==='finalizedHead')check(/^(?:0x)?[a-f0-9]{64}$/.test(v),'PREVIEW_PUBLIC_HASH');
  else if(key==='schema')check(['moriarty.preview-financial-integration/1','moriarty.preview-financial-run/1','moriarty.finalized-financial-stage/1','moriarty.native-financial-transaction/1'].includes(v),'PREVIEW_PUBLIC_SCHEMA');
  else if(key==='code')validatePublicDriverFailure({phase:'preflight',stage:null,code:v});
  else check(v.length<=128&&(VALUES.has(v)||/^-?(0|[1-9][0-9]*)$/.test(v)||hash(v)),'PREVIEW_PUBLIC_STRING');return v;
 }
 check(typeof v==='object'&&!seen.has(v),'PREVIEW_PUBLIC_OBJECT');seen.add(v);let out;
 if(Array.isArray(v)){check(v.length<=1024&&Reflect.ownKeys(v).length===v.length+1,'PREVIEW_PUBLIC_ARRAY');out=[];for(let i=0;i<v.length;i++){const d=Object.getOwnPropertyDescriptor(v,String(i));check(d&&Object.hasOwn(d,'value')&&d.enumerable,'PREVIEW_FIELDS');out.push(clone(d.value,key,depth+1,true));}}
 else {out={};for(const k of fields(v)){check(KEYS.has(k)||hash(k),'PREVIEW_PUBLIC_KEY');const x=Object.getOwnPropertyDescriptor(v,k).value;if(hash(k))check(decimal(x),'PREVIEW_PUBLIC_ASSET_AMOUNT');out[k]=clone(x,k,depth+1);}}
 seen.delete(v);return out;
 }return clone(value);}
function cleanup(c){exact(c,'walletStopped,pendingOperations,containmentComplete');check(typeof c.walletStopped==='boolean'&&typeof c.containmentComplete==='boolean'&&(c.pendingOperations===null||Number.isSafeInteger(c.pendingOperations)&&c.pendingOperations>=0),'PREVIEW_PUBLIC_CLEANUP');}
function safeDirectory(directory){absolute(directory);for(let p=directory;;p=dirname(p)){const s=lstatSync(p);check(!s.isSymbolicLink(),'PREVIEW_PUBLIC_SYMLINK');if(p===directory)check(s.isDirectory()&&(s.mode&0o077)===0&&s.uid===process.getuid(),'PREVIEW_PUBLIC_DIRECTORY');if(dirname(p)===p)break;}}
function durable(directory,name,value){safeDirectory(directory);const raw=Buffer.from(JSON.stringify(value)+'\n');check(raw.length<=8*1024*1024,'PREVIEW_PUBLIC_SIZE');const fd=openSync(join(directory,name),constants.O_WRONLY|constants.O_CREAT|constants.O_EXCL|constants.O_NOFOLLOW,0o600);try{writeFileSync(fd,raw);fsyncSync(fd);}finally{closeSync(fd);}const d=openSync(directory,constants.O_RDONLY|constants.O_DIRECTORY);try{fsyncSync(d);}finally{closeSync(d);}}
function checkedIntegrationResult(result){
 const keys=fields(result);exact(result,'schema,status,kind,sourceTestOnly,networkAcceptance,proofAcceptance,financialAcceptance,build,assetBindings,phase,driver,cleanup,setupPendingOperations,comparisons,financialComparison,scope'+(keys.includes('failureCode')?',failureCode':''));
 check(result.schema==='moriarty.preview-financial-integration/1'&&['PASS','FINANCIAL_COMPLETE','FAILED','SOURCE_TEST_ONLY'].includes(result.status)&&['loan','swap'].includes(result.kind)&&typeof result.sourceTestOnly==='boolean'&&result.networkAcceptance===false&&result.proofAcceptance===false&&result.financialAcceptance===false,'PREVIEW_PUBLIC_RESULT');check(!['PASS','FINANCIAL_COMPLETE'].includes(result.status)||result.sourceTestOnly===false,'PREVIEW_PUBLIC_SOURCE_ONLY');check(result.status!=='SOURCE_TEST_ONLY'||result.sourceTestOnly===true,'PREVIEW_PUBLIC_SOURCE_ONLY');
 if(keys.includes('failureCode')){check(result.status==='FAILED','PREVIEW_PUBLIC_FAILURE');failureCode(result.failureCode);}cleanup(result.cleanup);
 if(result.build!==undefined){exact(result.build,'receiptSha256,sourceManifestHash');check(hash(result.build.receiptSha256)&&hash(result.build.sourceManifestHash),'PREVIEW_BUILD_HASH');}
 if(result.driver!==undefined){const d=result.driver,dk=fields(d);exact(d,'schema,status,kind,contractAddress,stages,transactionIds,cleanup,operationalState,scope'+(dk.includes('assetBindings')?',assetBindings':'')+(dk.includes('failure')?',failure':''));check(d.schema==='moriarty.preview-financial-run/1'&&d.kind===result.kind&&['PASS','FINANCIAL_COMPLETE','FAILED','INCOMPLETE'].includes(d.status),'PREVIEW_PUBLIC_DRIVER');cleanup(d.cleanup);if(dk.includes('failure'))validatePublicDriverFailure(d.failure);}
 if(result.financialComparison!==undefined)exact(result.financialComparison,'status,kind,contractAddress,expectationsSha256,stages,scope,networkAcceptance,proofAcceptance');
 const out=publicClone(result);if(out.status==='FINANCIAL_COMPLETE')completed(out);return out;
}

// Validate a completed command result, never upgrade historical FAILED/INCOMPLETE
// records. Receipt authentication and financial comparison remain producer duties.
function completed(r){
 check(['PASS','FINANCIAL_COMPLETE'].includes(r.status)&&r.sourceTestOnly===false&&!Object.hasOwn(r,'failureCode')&&r.phase==='driver'&&r.setupPendingOperations===0,'PREVIEW_COMPLETION_REQUIRED');
 check(r.cleanup.containmentComplete===(r.status==='PASS'),'PREVIEW_COMPLETION_CONTAINMENT');
 const d=r.driver,f=r.financialComparison,order=r.kind==='loan'?['deploy','initialize','accrue','settle']:['deploy','initialize','swap','close'];
 check(d&&f&&d.status===r.status&&!Object.hasOwn(d,'failure')&&f.status==='PASS'&&f.kind===r.kind&&f.contractAddress===d.contractAddress,'PREVIEW_COMPLETION_REQUIRED');
 for(const c of [r.cleanup,d.cleanup])check(c.walletStopped===true&&c.pendingOperations===0,'PREVIEW_COMPLETION_REQUIRED');
 check(isDeepStrictEqual(r.cleanup,d.cleanup)&&Array.isArray(d.stages)&&d.stages.length===4&&Array.isArray(r.comparisons)&&r.comparisons.length===4&&isDeepStrictEqual(f.stages,r.comparisons),'PREVIEW_COMPLETION_REQUIRED');
 const ids=new Set();
 for(const [i,stage] of order.entries()){
  const receipt=d.stages[i],comparison=r.comparisons[i];
  check(receipt.circuitId===stage&&receipt.contractAddress===d.contractAddress&&comparison.status==='PASS'&&comparison.kind===r.kind&&comparison.stage===stage&&comparison.txId===receipt.txId&&comparison.contractAddress===d.contractAddress&&comparison.blockHash===receipt.blockHash&&comparison.blockHeight===receipt.blockHeight,'PREVIEW_COMPLETION_REQUIRED');
  check(receipt.transaction?.identifiers?.includes(receipt.txId)&&receipt.transaction.proofVerified===false&&receipt.transaction.ledgerAccepted===false,'PREVIEW_COMPLETION_REQUIRED');
  for(const id of receipt.transaction.identifiers){check(!ids.has(id),'PREVIEW_COMPLETION_REQUIRED');ids.add(id);}
 }
 check(Array.isArray(d.transactionIds)&&d.transactionIds.length===ids.size&&new Set(d.transactionIds).size===ids.size&&d.transactionIds.every(id=>ids.has(id)),'PREVIEW_COMPLETION_REQUIRED');
 check(d.operationalState?.reservedSubmissions===4&&decimal(d.operationalState.reservedDustFee)&&d.operationalState.unavailable===undefined,'PREVIEW_COMPLETION_REQUIRED');
 return r;
}
export function validatePreviewFinancialCompletion(result){return completed(checkedIntegrationResult(result));}
export function retainPreviewIntegrationResult(directory,result){const out=checkedIntegrationResult(result);durable(directory,'integration-result.json',out);return {status:'RECORDED'};}

export function retainPreviewStage(directory,summary){fields(summary);check(summary.status==='PASS'&&['loan','swap'].includes(summary.kind)&&['deploy','initialize',...(summary.kind==='loan'?['accrue','settle']:['swap','close'])].includes(summary.stage)&&identifier(summary.txId)&&summary.networkAcceptance===false&&summary.proofAcceptance===false,'PREVIEW_PUBLIC_STAGE');const out=publicClone(summary);durable(directory,'stage-'+summary.stage+'.json',out);return {status:'RECORDED',stage:summary.stage,txId:summary.txId};}
export function publicPreviewLaunchEvent(event){
 const keys=fields(event);check(['submitted','stopped'].includes(event.kind)&&Array.isArray(event.identifiers)&&event.identifiers.length<=128,'PREVIEW_PUBLIC_EVENT');
 const identifiers=publicClone({identifiers:event.identifiers}).identifiers;check(identifiers.every(identifier),'PREVIEW_PUBLIC_EVENT');
 if(event.kind==='submitted'){exact(event,'kind,txId,identifiers,transactionHash');check(identifier(event.txId)&&identifiers.includes(event.txId)&&hash(event.transactionHash),'PREVIEW_PUBLIC_EVENT');return {kind:event.kind,txId:event.txId,identifiers,transactionHash:event.transactionHash};}
 exact(event,'kind,identifiers,reservationRetained,pendingOperations'+(keys.includes('operation')?',operation':''));check(event.reservationRetained===true&&Number.isSafeInteger(event.pendingOperations)&&event.pendingOperations>=0,'PREVIEW_PUBLIC_EVENT');return {kind:'stopped',identifiers,reservationRetained:true,pendingOperations:event.pendingOperations};
}
