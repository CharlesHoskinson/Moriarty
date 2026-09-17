import {EXISTING_SWAP,INITIALIZED_SWAP,inspectExistingSwapBytes,inspectInitializedSwapBytes,assertInitializedSwapState} from './continue-initialized-swap.mjs';
/** Recovery predicates for retained local histories and one Preview deployment.
 * Public checks authorize no dispatch; explicit private restoration requires a
 * constructor-verified result and never constructs or submits a deployment.
 */
import {createHash} from 'node:crypto';
import {readFileSync,openSync,closeSync,fstatSync,lstatSync,constants} from 'node:fs';
import {join,isAbsolute,resolve,dirname} from 'node:path';
import {pathToFileURL} from 'node:url';
import {isDeepStrictEqual} from 'node:util';
import {PINNED_NM} from './providers.mjs';
import {decodeNativeFinancialTransaction} from './receipt.mjs';
import {inspectInitializedLoanBytes,assertInitializedLoanState} from './continue-initialized-loan.mjs';
const check=(ok,code)=>{if(!ok)throw Error(code);};
const sha=b=>createHash('sha256').update(b).digest('hex');
export const EXISTING_LOAN=Object.freeze({
 transactionHash:'0af6f3ed8960b1a7d1d5e8c204d9e133255e784dd2c5fad1f160c5212d02bd3c',
 contractAddress:'ba4c808859fc2e4ee6d3d19fa0d812bb9a9c9eb0527161fb91315213bc24a713',
 txId:'00959c51e7d62ee9160bf1396ce0ab52f26757a7c5adec669cb083d5a8787d1de9',
 identifiers:Object.freeze(['00b6140ece5793d57c801512e8e7c9e2ec2687e19b1c48a1f56167f2cd1dfc9e66','00959c51e7d62ee9160bf1396ce0ab52f26757a7c5adec669cb083d5a8787d1de9']),
 initialStateSha256:'42aac4043fd65583cc2aa8ef87216f313c3da84c85d50f9db333b59725337cf7',
 buildReceiptSha256:'51ee2d4d60216464a9ace67966ba0ab253699844187aeb5652b46dc6e9ca5bf7',
 blockHeight:20313,blockHash:'1acc3f7bc138076c0d74295c770375884dd99f121574be14960333e36d5195ed',
 allocationId:'sp05-local-loan-03',
});
export function assertExistingLoanState(state,ledger){
 check(state&&typeof state.serialize==='function','RECOVERY_STATE_TYPE');
 const raw=Buffer.from(state.serialize());
 check(raw.length<=16*1024*1024&&sha(raw)===EXISTING_LOAN.initialStateSha256,'RECOVERY_STATE_MISMATCH');
 const native=ledger.ContractState.deserialize(raw);
 check(Buffer.from(native.serialize()).equals(raw),'RECOVERY_STATE_CANONICAL');
 return true;
}
export function inspectExistingLoanBytes(raw,ledger){
 check(raw instanceof Uint8Array&&raw.length>0&&raw.length<=16*1024*1024&&sha(raw)===EXISTING_LOAN.transactionHash,'RECOVERY_NATIVE_HASH');
 const decoded=decodeNativeFinancialTransaction(raw,ledger),tx=ledger.Transaction.deserialize('signature','proof','binding',raw);
 check(decoded.transactionHash===EXISTING_LOAN.transactionHash&&isDeepStrictEqual(decoded.identifiers,EXISTING_LOAN.identifiers),'RECOVERY_NATIVE_IDENTIFIERS');
 check(decoded.actions.length===1&&decoded.actions[0].kind==='deploy'&&decoded.actions[0].segment===1&&decoded.actions[0].address===EXISTING_LOAN.contractAddress,'RECOVERY_NATIVE_ACTION');
 const action=[...tx.intents.values()].flatMap(i=>i.actions)[0];check(action instanceof ledger.ContractDeploy,'RECOVERY_NATIVE_DEPLOY');assertExistingLoanState(action.initialState,ledger);
 const oldDustNullifiers=[...tx.intents.values()].flatMap(i=>i.dustActions?.spends??[]).map(s=>s.oldNullifier);
 check(oldDustNullifiers.length>0&&oldDustNullifiers.every(n=>typeof n==='bigint'),'RECOVERY_DUST_INPUTS');
 return Object.freeze({...EXISTING_LOAN,oldDustNullifiers:Object.freeze(oldDustNullifiers),spentUnshieldedInputs:Object.freeze(decoded.inputs.map(({intentHash,outputNo})=>Object.freeze({intentHash,outputNo})))});
}
/** Check the complete root Level namespace on a copied DB, never just one sublevel. */
export function assertMetadataOnlyEntries(entries,accountId){
 check(typeof accountId==='string'&&accountId.length>0&&Array.isArray(entries)&&entries.length===1,'RECOVERY_STORE_METADATA_ONLY');
 const [key,value]=entries[0];const expected='!sp05-loan:'+sha(Buffer.from(accountId)).slice(0,32)+'!__midnight_encryption_metadata__';
 check(typeof key==='string'&&key===expected&&typeof value==='string'&&value.length<=256,'RECOVERY_STORE_NAMESPACE');
 let metadata;try{metadata=JSON.parse(value);}catch{throw Error('RECOVERY_STORE_METADATA');}
 check(metadata&&Object.keys(metadata).sort().join(',')==='salt,version'&&metadata.version===1&&typeof metadata.salt==='string'&&/^[a-f0-9]{64}$/.test(metadata.salt),'RECOVERY_STORE_METADATA');
 return true;
}
export function assertRecoveryWallet({synced,binding,timestampMs,dustCap}){
 check(binding?.transactionHash===EXISTING_LOAN.transactionHash&&Array.isArray(binding.oldDustNullifiers)&&binding.oldDustNullifiers.length>0&&Array.isArray(binding.spentUnshieldedInputs),'RECOVERY_WALLET_BINDING');
 return assertHistoryWallet({synced,binding,timestampMs,dustCap});
}
export function assertSwapHistoryWallet({synced,binding,timestampMs,dustCap}){
 check(binding?.deployment?.transactionHash===EXISTING_SWAP.transactionHash&&binding?.initialize?.transactionHash===INITIALIZED_SWAP.transactionHash&&Array.isArray(binding.oldDustNullifiers)&&binding.oldDustNullifiers.length>0&&Array.isArray(binding.spentUnshieldedInputs),'RECOVERY_WALLET_BINDING');
 return assertHistoryWallet({synced,binding,timestampMs,dustCap});
}
function assertHistoryWallet({synced,binding,timestampMs,dustCap}){
 const now=Date.now();check(Number.isSafeInteger(timestampMs)&&timestampMs<=now&&now-timestampMs<=60000&&typeof dustCap==='bigint'&&dustCap>0n,'RECOVERY_WALLET_TIME_CAP');
 for(const kind of ['shielded','unshielded','dust']){const p=synced?.[kind]?.progress;check(p?.isConnected===true&&typeof p.isStrictlyComplete==='function'&&p.isStrictlyComplete()===true,'RECOVERY_WALLET_SYNC');}
 const u=synced.unshielded,d=synced.dust;check(Array.isArray(u.availableCoins)&&Array.isArray(u.pendingCoins)&&Array.isArray(d.state?.pendingDust)&&d.state.pendingDust.length===0,'RECOVERY_WALLET_PENDING');
 const coins=[...u.availableCoins,...u.pendingCoins].map(c=>{
  const coin=c?.utxo;check(coin&&typeof coin.intentHash==='string'&&/^[a-f0-9]{64}$/.test(coin.intentHash)&&Number.isSafeInteger(coin.outputNo)&&coin.outputNo>=0,'RECOVERY_WALLET_COIN');return coin;
 });
 for(const spent of binding.spentUnshieldedInputs)check(!coins.some(c=>c.intentHash===spent.intentHash&&c.outputNo===spent.outputNo),'RECOVERY_WALLET_SPENT_INPUT');
 check(typeof d.state.state?.findUtxoByNullifier==='function','RECOVERY_WALLET_DUST_CODEC');
 for(const old of binding.oldDustNullifiers)check(d.state.state.findUtxoByNullifier(old)===undefined,'RECOVERY_WALLET_SPENT_DUST');
 check(Array.isArray(d.availableCoins)&&d.availableCoins.length>0&&typeof d.balance==='function','RECOVERY_WALLET_REGISTERED_DUST');
 const balance=d.balance(new Date(timestampMs));check(typeof balance==='bigint'&&balance>=dustCap,'RECOVERY_WALLET_DUST_CAP');return true;
}
/** Re-execute only the unchanged constructor. No deployment object or transaction is made. */
export async function reconstructExistingLoan(options){return reconstructLoan(options,EXISTING_LOAN);}
async function reconstructLoan({compiledContract,zkConfigProvider,coinPublicKey,signingKey,args,ledger},binding){
 check(Array.isArray(args)&&args.length===6&&typeof signingKey==='string'&&signingKey.length>0,'RECOVERY_CONSTRUCTOR_INPUTS');
 const pins=[['@midnight-ntwrk/midnight-js-protocol/dist/compact-js.mjs','7c34e5ac44b1406080f1cbf9f514c1d887b7bc947327a3f83031d9794caa85b7'],['@midnight-ntwrk/midnight-js-types/dist/index.mjs','465600b3e07a1779ec1f418c550ecf7535d5ad32afe4e145452e3fc2b08ca94d']];
 for(const [file,digest] of pins)check(sha(readFileSync(join(PINNED_NM,file)))===digest,'RECOVERY_CONSTRUCTOR_SDK_PIN');
 const {ContractExecutable}=await import(pathToFileURL(join(PINNED_NM,pins[0][0])).href),{makeContractExecutableRuntime,exitResultOrError}=await import(pathToFileURL(join(PINNED_NM,pins[1][0])).href);
 const runtime=makeContractExecutableRuntime(zkConfigProvider,{coinPublicKey,signingKey});
 const result=exitResultOrError(await runtime.runPromiseExit(ContractExecutable.make(compiledContract).initialize({},...args)));
 assertBoundLoanState(result.public.contractState,ledger,binding);
 const p=result.private,z=p.zswapLocalState;
 check(isDeepStrictEqual(p.privateState,{})&&p.signingKey===signingKey,'RECOVERY_CONSTRUCTOR_PRIVATE_RESULT');
 check(z&&Reflect.ownKeys(z).length===4&&Object.keys(z).sort().join(',')==='coinPublicKey,currentIndex,inputs,outputs'&&Array.isArray(z.inputs)&&z.inputs.length===0&&Array.isArray(z.outputs)&&z.outputs.length===0&&z.currentIndex===0n&&z.coinPublicKey===coinPublicKey,'RECOVERY_CONSTRUCTOR_ZSWAP');
 const state=ledger.ContractState.deserialize(result.public.contractState.serialize()),a=state.maintenanceAuthority;
 check(a.threshold===1&&a.counter===0n&&a.committee.length===1&&a.committee[0]===ledger.signatureVerifyingKey(signingKey),'RECOVERY_CONSTRUCTOR_AUTHORITY');
 const verified=Object.freeze({privateState:p.privateState,signingKey:p.signingKey});verifiedPrivateResults.add(verified);privateBindings.set(verified,binding);return verified;
}

function checkPublicRecoveryTip(tip,readCurrentTip){
 check(typeof readCurrentTip==='function','RECOVERY_INDEXER_READER');
 check(tip?.status==='READY'&&typeof tip.hash==='string'&&/^[a-f0-9]{64}$/.test(tip.hash)&&Number.isSafeInteger(tip.height)&&tip.height>=0&&Number.isSafeInteger(tip.finalizedHeight)&&Math.abs(tip.height-tip.finalizedHeight)<=2,'RECOVERY_TIP_FINALITY');
 check(Number.isSafeInteger(tip.timestampMs)&&tip.timestampMs<=Date.now()&&Date.now()-tip.timestampMs<=60000,'RECOVERY_TIP_TIME');
}
/** Internal fixed-loan snapshot sampler. Only canonical forward movement retries. */
async function stableLoanSnapshot({rpc,deadlineMs,tip,readCurrentTip,finalityHeight,historyHeight,checkStates}){
 const {beforeDeadline}=await import('./receipt.mjs');
 let queryDeadline=deadlineMs;const wait=fn=>beforeDeadline(fn,queryDeadline);
 const indexedTip=async()=>{
  const t=Object.freeze({...await wait(()=>readCurrentTip(queryDeadline))});
  check(t.status==='READY'&&typeof t.hash==='string'&&/^[a-f0-9]{64}$/.test(t.hash)&&Number.isSafeInteger(t.height)&&t.height>=0&&t.finalizedHeight===t.height&&t.finalizedHash==='0x'+t.hash,'RECOVERY_INDEXER_FINALITY');
  check(Number.isSafeInteger(t.timestampMs)&&t.timestampMs<=Date.now()&&Date.now()-t.timestampMs<=60000,'RECOVERY_INDEXER_TIME');
  return t;
 };
 // The historical watch runs once. Only well-formed forward movement may
 // trigger another public snapshot; state/identity/provider failures are fatal.
 check(await wait(()=>rpc('chain_getBlockHash',[tip.height]))==='0x'+tip.hash,'RECOVERY_TIP_CANONICAL');
 const startedAt=Date.now();queryDeadline=Math.min(deadlineMs,startedAt+60000);
 const readRpc=(method,params)=>wait(()=>rpc(method,params,queryDeadline));
 const retry=Object.freeze({});let lastMovement,samples=0,finalityFloor=finalityHeight;
 const moved=async(phase,from,to)=>{
  check(to.height>from.height,'RECOVERY_INDEXER_FINALITY');
  check(await readRpc('chain_getBlockHash',[from.height])===from.hash,'RECOVERY_TIP_CANONICAL');
  finalityFloor=Math.max(finalityFloor,to.height);
  lastMovement=Object.freeze({phase,fromHash:from.hash,fromHeight:from.height,toHash:to.hash,toHeight:to.height});
  throw retry;
 };
 const nodeTip=async()=>{
  const hash=await readRpc('chain_getFinalizedHead',[]);
  check(typeof hash==='string'&&/^0x[a-f0-9]{64}$/.test(hash),'RECOVERY_CURRENT_HEAD');
  const header=await readRpc('chain_getHeader',[hash]);
  check(typeof header?.number==='string'&&/^0x[a-f0-9]+$/i.test(header.number),'RECOVERY_CURRENT_HEADER');
  const height=Number(BigInt(header.number));check(Number.isSafeInteger(height)&&height>=0,'RECOVERY_CURRENT_HEIGHT');
  return {hash,height};
 };
 for(;samples<6&&Date.now()<queryDeadline;){
  samples++;
  try{
   const indexed=await indexedTip(),anchor={hash:'0x'+indexed.hash,height:indexed.height};
   check(indexed.height>=finalityFloor,'RECOVERY_FINALITY_REGRESSION');
   const {hash:head,height}=await nodeTip();
   check(height>=finalityHeight&&height>=tip.height&&height-tip.height<=2,'RECOVERY_CURRENT_HEIGHT');
   if(anchor.hash!==head||anchor.height!==height)await moved('before-state',anchor,{hash:head,height});
   // A block filter selects an action IN that block, not state as of the block.
   const stateEvidence=await checkStates(wait);
   const after=await indexedTip();
   if(after.hash!==indexed.hash||after.height!==indexed.height)await moved('after-state',anchor,{hash:'0x'+after.hash,height:after.height});
   const end=await nodeTip();
   if(end.hash!==head||end.height!==height)await moved('final-node',anchor,end);
   check(tip.height>=historyHeight&&Date.now()-tip.timestampMs<=60000,'RECOVERY_CURRENT_STATE_TIME');
   return Object.freeze({stateBlock:Object.freeze({hash:head,height}),snapshotSamples:samples,stateEvidence});
  }catch(error){if(error!==retry)throw error;}
  if(samples<6&&Date.now()<queryDeadline)await wait(()=>new Promise(resolve=>setTimeout(resolve,Math.min(1000,Math.max(1,queryDeadline-Date.now()-1)))));
 }
 const error=Error('RECOVERY_SNAPSHOT_UNSTABLE');
 error.recoveryObservation=Object.freeze({samples,elapsedMs:Date.now()-startedAt,reason:Date.now()>=queryDeadline?'deadline':'sample-limit',...lastMovement});
 throw error;
}

/** Public constructor-only gate; behavior remains distinct from initialized continuation. */
export async function verifyExistingLoanPublic({raw,ledger,provider,rpc,decodeState,deadlineMs,expectedProtocolVersion,tip,readCurrentTip}){
 const {observeFinalizedStage}=await import('./receipt.mjs');
 const binding=inspectExistingLoanBytes(raw,ledger);checkPublicRecoveryTip(tip,readCurrentTip);
 const observation=await observeFinalizedStage({provider,rpc,ledger,txId:binding.txId,contractAddress:binding.contractAddress,circuitId:'deploy',decodeState,deadlineMs,expectedProtocolVersion});
 const r=observation.receipt;
 check(r.transaction.transactionHash===binding.transactionHash&&r.transaction.rawSha256===binding.transactionHash&&r.blockHash===EXISTING_LOAN.blockHash&&r.blockHeight===EXISTING_LOAN.blockHeight,'RECOVERY_DEPLOY_RECEIPT');
 const {stateBlock,snapshotSamples}=await stableLoanSnapshot({rpc,deadlineMs,tip,readCurrentTip,finalityHeight:r.finalizedHeight,historyHeight:r.blockHeight,checkStates:async wait=>{
  const [historical,current]=await Promise.all([wait(()=>provider.queryContractState(binding.contractAddress,{type:'blockHash',blockHash:r.blockHash})),wait(()=>provider.queryContractState(binding.contractAddress))]);
  assertExistingLoanState(historical,ledger);assertExistingLoanState(current,ledger);
 }});
 return Object.freeze({binding,observation,tip:Object.freeze({...tip}),stateBlock,snapshotSamples,status:'PUBLIC_STATE_VERIFIED',scope:'Read-only native/history/latest-state checks in one stable indexed/finalized snapshot; bounded public sampling, no private recovery or financial acceptance'});
}

const INITIALIZE_BLOCK=Object.freeze({height:20363,hash:'7f61e4c7225c456400e852f3648cf7fcad958bb05ed84002441782764093c94f'});
/** Public-only gate for exactly the retained deploy and initialize history.
 * Both watches run once; latest and historical full states are checked in the
 * bounded stable snapshot. The result grants no wallet/private access or dispatch.
 */
export async function verifyInitializedLoanPublic({raw,rawInitialize,ledger,provider,rpc,decodeState,deadlineMs,expectedProtocolVersion,tip,readCurrentTip}){
 const {observeFinalizedStage}=await import('./receipt.mjs');
 const deployment=inspectExistingLoanBytes(raw,ledger),initialize=inspectInitializedLoanBytes(rawInitialize,ledger);
 check(expectedProtocolVersion===1000000,'INITIALIZED_PROTOCOL');checkPublicRecoveryTip(tip,readCurrentTip);
 const common={provider,rpc,ledger,contractAddress:deployment.contractAddress,decodeState,deadlineMs,expectedProtocolVersion};
 const deploymentObservation=await observeFinalizedStage({...common,txId:deployment.txId,circuitId:'deploy'});
 const d=deploymentObservation.receipt;
 check(d.transaction.transactionHash===deployment.transactionHash&&d.transaction.rawSha256===deployment.transactionHash&&d.blockHash===EXISTING_LOAN.blockHash&&d.blockHeight===EXISTING_LOAN.blockHeight,'RECOVERY_DEPLOY_RECEIPT');
 const initializeObservation=await observeFinalizedStage({...common,txId:initialize.txId,circuitId:'initialize'});
 const i=initializeObservation.receipt;
 check(i.transaction.transactionHash===initialize.transactionHash&&i.transaction.rawSha256===initialize.transactionHash&&i.blockHash===INITIALIZE_BLOCK.hash&&i.blockHeight===INITIALIZE_BLOCK.height,'INITIALIZED_DEPLOYMENT_HISTORY');
 const {stateBlock,snapshotSamples,stateEvidence:initializeContractState}=await stableLoanSnapshot({rpc,deadlineMs,tip,readCurrentTip,finalityHeight:Math.max(d.finalizedHeight,i.finalizedHeight),historyHeight:i.blockHeight,checkStates:async wait=>{
  const [historicalDeploy,historicalInitialize,current]=await Promise.all([wait(()=>provider.queryContractState(deployment.contractAddress,{type:'blockHash',blockHash:d.blockHash})),wait(()=>provider.queryContractState(deployment.contractAddress,{type:'blockHash',blockHash:i.blockHash})),wait(()=>provider.queryContractState(deployment.contractAddress))]);
  assertExistingLoanState(historicalDeploy,ledger);assertInitializedLoanState(historicalInitialize,ledger);assertInitializedLoanState(current,ledger);
  // Copy the verified historical bytes instead of retaining a mutable provider object.
  return ledger.ContractState.deserialize(historicalInitialize.serialize());
 }});
 const binding=Object.freeze({transactionHash:deployment.transactionHash,contractAddress:deployment.contractAddress,deployment,initialize,spentUnshieldedInputs:Object.freeze([...deployment.spentUnshieldedInputs,...initialize.spentUnshieldedInputs]),oldDustNullifiers:Object.freeze([...deployment.oldDustNullifiers,...initialize.oldDustNullifiers]),mintedOutput:initialize.mintedOutput});
 return Object.freeze({status:'INITIALIZED_PUBLIC_STATE_VERIFIED',dispatchAuthorized:false,deploymentObservation,initializeObservation,initializeContractState,binding,tip:Object.freeze({...tip}),stateBlock,snapshotSamples,scope:'Read-only exact deploy/initialize history and unchanged initialized state in a stable indexed/finalized snapshot; no private access or dispatch authorization'});
}

export async function verifyInitializedSwapPublic({raw,rawInitialize,ledger,provider,rpc,decodeState,deadlineMs,expectedProtocolVersion,tip,readCurrentTip}){
 const {observeFinalizedStage}=await import('./receipt.mjs');
 const deployment=inspectExistingSwapBytes(raw,ledger),initialize=inspectInitializedSwapBytes(rawInitialize,ledger);
 check(expectedProtocolVersion===1000000,'INITIALIZED_PROTOCOL');checkPublicRecoveryTip(tip,readCurrentTip);
 const common={provider,rpc,ledger,contractAddress:deployment.contractAddress,decodeState,deadlineMs,expectedProtocolVersion};
 const deploymentObservation=await observeFinalizedStage({...common,txId:deployment.txId,circuitId:'deploy'});
 const d=deploymentObservation.receipt;
 check(d.transaction.transactionHash===deployment.transactionHash&&d.transaction.rawSha256===deployment.transactionHash&&d.blockHash==='75f27b677c24a38c1804935dd909e71d1b7e2719b920758f485e79de3b127202'&&d.blockHeight===20389,'RECOVERY_DEPLOY_RECEIPT');
 const initializeObservation=await observeFinalizedStage({...common,txId:initialize.txId,circuitId:'initialize'});
 const i=initializeObservation.receipt;
 check(i.transaction.transactionHash===initialize.transactionHash&&i.transaction.rawSha256===initialize.transactionHash&&i.blockHash==='f268da29bb3e6e81f8a97bae0eb53938f3a146fce677deb5c7ae18b8c5335386'&&i.blockHeight===20393,'INITIALIZED_DEPLOYMENT_HISTORY');
 const {stateBlock,snapshotSamples,stateEvidence:initializeContractState}=await stableLoanSnapshot({rpc,deadlineMs,tip,readCurrentTip,finalityHeight:Math.max(d.finalizedHeight,i.finalizedHeight),historyHeight:i.blockHeight,checkStates:async wait=>{
  const [historicalDeploy,historicalInitialize,current]=await Promise.all([wait(()=>provider.queryContractState(deployment.contractAddress,{type:'blockHash',blockHash:d.blockHash})),wait(()=>provider.queryContractState(deployment.contractAddress,{type:'blockHash',blockHash:i.blockHash})),wait(()=>provider.queryContractState(deployment.contractAddress))]);
  check(historicalDeploy&&typeof historicalDeploy.serialize==='function'&&sha(historicalDeploy.serialize())===EXISTING_SWAP.initialStateSha256,'SWAP_CONTINUATION_DEPLOY_STATE');assertInitializedSwapState(historicalInitialize,ledger);assertInitializedSwapState(current,ledger);
  // Copy the verified historical bytes instead of retaining a mutable provider object.
  return ledger.ContractState.deserialize(historicalInitialize.serialize());
 }});
 const binding=Object.freeze({transactionHash:deployment.transactionHash,contractAddress:deployment.contractAddress,deployment,initialize,spentUnshieldedInputs:Object.freeze([...deployment.spentUnshieldedInputs,...initialize.spentUnshieldedInputs]),oldDustNullifiers:Object.freeze([...deployment.oldDustNullifiers,...initialize.oldDustNullifiers]),mintedOutput:initialize.mintedOutput});
 return Object.freeze({status:'INITIALIZED_PUBLIC_STATE_VERIFIED',dispatchAuthorized:false,deploymentObservation,initializeObservation,initializeContractState,binding,tip:Object.freeze({...tip}),stateBlock,snapshotSamples,scope:'Read-only exact deploy/initialize history and unchanged initialized state in a stable indexed/finalized snapshot; no private access or dispatch authorization'});
}

const verifiedPrivateResults=new WeakSet(),consumedPrivateResults=new WeakSet(),privateBindings=new WeakMap();
/** Only a constructor-verified, single-use result can reach private persistence. */
export async function restoreExistingLoanPrivate(provider,result){
 check(!verifiedPrivateResults.has(result)||privateBindings.get(result)===EXISTING_LOAN,'RECOVERY_RESTORE_BINDING');return restoreLoanPrivate(provider,result);
}
async function restoreLoanPrivate(provider,result){
 check(result&&verifiedPrivateResults.has(result)&&!consumedPrivateResults.has(result)&&isDeepStrictEqual(result.privateState,{}),'RECOVERY_RESTORE_UNVERIFIED');
 for(const name of ['setContractAddress','set','setSigningKey','get','getSigningKey'])check(typeof provider?.[name]==='function','RECOVERY_RESTORE_PROVIDER');
 consumedPrivateResults.add(result);const binding=privateBindings.get(result);
 provider.setContractAddress(binding.contractAddress);
 await provider.set('sp05-loan',result.privateState);
 await provider.setSigningKey(binding.contractAddress,result.signingKey);
 // The pinned provider opens a distinct Level handle per operation. Serialize
 // readbacks so both sublevels never contend for the same database lock.
 const state=await provider.get('sp05-loan');
 const key=await provider.getSigningKey(binding.contractAddress);
 check(isDeepStrictEqual(state,result.privateState)&&key===result.signingKey,'RECOVERY_RESTORE_ROUNDTRIP');
 return Object.freeze({status:'RESTORED',contractAddress:binding.contractAddress,txId:binding.txId});
}

const ORIGINAL_PRIVATE_BASE='/home/charl/.local/state/moriarty/sp05-local-loan-20260910-03';
const ORIGINAL_RESULT_SHA256='bf1d456714ea134ad7e320849b3841088b2a2a352435d75ced65854c48031f54';
const NETWORK_TAG='e72f7a21a0397844563b4206f887b779ffa0d937c2d1b2339441faa1f08b9846';
/** Closed recovery plan for this known deployment; no caller-provided private state. */
export function validateExistingLoanPlan(recovery,plan){
 const fields='schema,transactionFile,transactionHash,identifiers,txId,contractAddress,buildReceiptSha256,networkTag,expectedProtocolVersion,sourceAllocationId,sourceResultFile,sourceResultSha256,sourcePrivateStateDirectory,inspectionDirectory,destinationDirectory'.split(',').sort();
 check(recovery&&Object.getPrototypeOf(recovery)===Object.prototype&&Reflect.ownKeys(recovery).length===fields.length&&Object.keys(recovery).sort().join(',')===fields.join(',')&&Object.values(Object.getOwnPropertyDescriptors(recovery)).every(d=>Object.hasOwn(d,'value')),'RECOVERY_PLAN_FIELDS');
 check(recovery.schema==='moriarty.existing-local-loan/1'&&plan.kind==='loan'&&plan.limits.submissions===3,'RECOVERY_PLAN_KIND_LIMIT');
 for(const key of ['transactionHash','identifiers','txId','contractAddress','buildReceiptSha256'])check(isDeepStrictEqual(recovery[key],EXISTING_LOAN[key]),'RECOVERY_PLAN_BINDING');
 check(plan.build.receiptSha256===EXISTING_LOAN.buildReceiptSha256&&recovery.networkTag===NETWORK_TAG&&plan.networkTag===NETWORK_TAG&&recovery.expectedProtocolVersion===1000000&&plan.expectedProtocolVersion===1000000,'RECOVERY_PLAN_NETWORK_BUILD');
 check(recovery.sourceAllocationId===EXISTING_LOAN.allocationId&&recovery.sourceResultSha256===ORIGINAL_RESULT_SHA256&&plan.limits.allocationId!==EXISTING_LOAN.allocationId,'RECOVERY_PLAN_ALLOCATION');
 for(const key of ['transactionFile','sourceResultFile','sourcePrivateStateDirectory','inspectionDirectory','destinationDirectory'])check(typeof recovery[key]==='string'&&isAbsolute(recovery[key])&&resolve(recovery[key])===recovery[key],'RECOVERY_PLAN_PATH');
 check(recovery.sourcePrivateStateDirectory===ORIGINAL_PRIVATE_BASE+'/contract-state'&&plan.roles.secretsFile===ORIGINAL_PRIVATE_BASE+'/roles.json'&&recovery.destinationDirectory===plan.privateState.directory,'RECOVERY_PLAN_PRIVATE_BINDING');
 const roots=[recovery.sourcePrivateStateDirectory,recovery.inspectionDirectory,recovery.destinationDirectory];
 check(roots.every((a,i)=>roots.every((b,j)=>i===j||(a!==b&&!a.startsWith(b+'/')))),'RECOVERY_PLAN_OVERLAP');
 for(const path of [recovery.inspectionDirectory,recovery.destinationDirectory,plan.privateState.passwordFile,plan.outputDirectory])check(path!==ORIGINAL_PRIVATE_BASE&&!path.startsWith(ORIGINAL_PRIVATE_BASE+'/'),'RECOVERY_PLAN_PRESERVE_ORIGINAL');
 return structuredClone(recovery);
}
function boundedPublicFile(path,max){
 for(let p=path;;p=dirname(p)){check(!lstatSync(p).isSymbolicLink(),'RECOVERY_PUBLIC_SYMLINK');if(dirname(p)===p)break;}
 let fd;try{fd=openSync(path,constants.O_RDONLY|constants.O_NOFOLLOW|constants.O_NONBLOCK);const s=fstatSync(fd);check(s.isFile()&&s.size>0&&s.size<=max,'RECOVERY_PUBLIC_FILE');return readFileSync(fd);}finally{if(fd!==undefined)closeSync(fd);}
}
export function readExistingLoanInputs(recovery,ledger){
 const raw=boundedPublicFile(recovery.transactionFile,16*1024*1024);const binding=inspectExistingLoanBytes(raw,ledger);
 const result=boundedPublicFile(recovery.sourceResultFile,128*1024);check(sha(result)===ORIGINAL_RESULT_SHA256,'RECOVERY_SOURCE_RESULT');
 const old=JSON.parse(result);check(old.allocationId===EXISTING_LOAN.allocationId&&old.transactionHash===binding.transactionHash&&old.contractAddress===binding.contractAddress&&old.reservedSubmissions===1&&old.reservedDustFee==='300000000000001'&&old.financialStagesCompleted===0,'RECOVERY_SOURCE_ALLOCATION');
 return {raw,binding};
}

// One retained Preview deployment only. Public native bytes remain bound even
// though the original submission result was ambiguous. This is not a general
// recovery cursor and cannot resume an initialized/modified contract.
export const PREVIEW_LOAN_RECOVERY=Object.freeze({
 transactionHash:'ecd8468a01cf85896b2bc53f08c6621029659d31f127b6dc50c7f9eb71b9d400',
 txId:'0074727c89072a920c3fe28bc0be689b0da73a7027fbf0ae734b6a62f743bb293a',
 identifiers:Object.freeze(['0001ec58451698a6d3816ef48038be1c5aae635f8e148eb2eed75a2e2f632f1f4c','0074727c89072a920c3fe28bc0be689b0da73a7027fbf0ae734b6a62f743bb293a']),
 originalDirectory:'/home/charl/.local/state/moriarty/preview-loan-20260917-01',
 contractAddress:'8034dffa6ce124cf34135799969d34831da5ce44fd287a3bead2fa3ce293a341',initialStateSha256:'a7c4b42737a5d6090e5767f5e59e06912c70f75b5f99536903310bf2ce0a1189',
 blockHeight:908628,blockHash:'e3f51b9a3d1518980506e9cb5efbbb8487c8acc9989590628cfb5b7330a4f8a4',
 historicalDustFee:'300000000000001',conservativeReservedDustFee:'600000000000002',remainingDustFee:'1399999999999998',
});
const previewBindings=new WeakSet(),previewCapabilities=new WeakSet();
function assertBoundLoanState(state,ledger,binding){
 check(state&&typeof state.serialize==='function','RECOVERY_STATE_TYPE');
 const raw=Buffer.from(state.serialize());
 check(raw.length<=16*1024*1024&&sha(raw)===binding.initialStateSha256,'RECOVERY_STATE_MISMATCH');
 check(Buffer.from(ledger.ContractState.deserialize(raw).serialize()).equals(raw),'RECOVERY_STATE_CANONICAL');return true;
}
export function inspectPreviewLoanBytes(raw,ledger){
 const fixed=PREVIEW_LOAN_RECOVERY;
 check(raw instanceof Uint8Array&&raw.length>0&&raw.length<=16*1024*1024&&sha(raw)===fixed.transactionHash,'RECOVERY_NATIVE_HASH');
 const decoded=decodeNativeFinancialTransaction(raw,ledger),tx=ledger.Transaction.deserialize('signature','proof','binding',raw);
 check(decoded.transactionHash===fixed.transactionHash&&isDeepStrictEqual(decoded.identifiers,fixed.identifiers),'RECOVERY_NATIVE_IDENTIFIERS');
 check(decoded.actions.length===1&&decoded.actions[0].kind==='deploy'&&decoded.actions[0].segment===1,'RECOVERY_NATIVE_ACTION');
 check(decoded.dustFee===fixed.historicalDustFee&&decoded.inputs.length===0&&decoded.outputs.length===0,'PREVIEW_RECOVERY_FEE');
 const action=[...tx.intents.values()].flatMap(i=>i.actions)[0];check(action instanceof ledger.ContractDeploy,'RECOVERY_NATIVE_DEPLOY');
 const oldDustNullifiers=[...tx.intents.values()].flatMap(i=>i.dustActions?.spends??[]).map(s=>s.oldNullifier);
 check(oldDustNullifiers.length>0&&oldDustNullifiers.every(n=>typeof n==='bigint'),'RECOVERY_DUST_INPUTS');
 const binding=Object.freeze({transactionHash:fixed.transactionHash,txId:fixed.txId,identifiers:fixed.identifiers,contractAddress:action.address,initialStateSha256:sha(action.initialState.serialize()),oldDustNullifiers:Object.freeze(oldDustNullifiers),spentUnshieldedInputs:Object.freeze([])});
 check(binding.contractAddress===fixed.contractAddress&&binding.initialStateSha256===fixed.initialStateSha256,'PREVIEW_RECOVERY_NATIVE_STATE');assertBoundLoanState(action.initialState,ledger,binding);previewBindings.add(binding);return binding;
}
function originalPreviewPlan(){
 const raw=boundedPublicFile(new URL('../../../deliverables/preview-loan-2026-09-17/actual-run01/plan.json',import.meta.url).pathname,128*1024);
 check(sha(raw)==='c74f1ff8768833181c704ec968c04d7c750cada9756c7a0da9707b2246427ec4','PREVIEW_RECOVERY_SOURCE_PLAN');return JSON.parse(raw);
}
export function validatePreviewRecoveryPlan(d,p){
 const fields=['schema','transactionFile','inspectionDirectory'];
 check(d&&Object.getPrototypeOf(d)===Object.prototype&&Reflect.ownKeys(d).length===fields.length&&Object.keys(d).sort().join(',')===fields.sort().join(',')&&Object.values(Object.getOwnPropertyDescriptors(d)).every(x=>Object.hasOwn(x,'value')),'PREVIEW_RECOVERY_FIELDS');
 check(d.schema==='moriarty.preview-deployment-recovery/1'&&p.kind==='loan'&&p.limits.submissions===3,'PREVIEW_RECOVERY_MODE');
 const original=originalPreviewPlan(),fixed=PREVIEW_LOAN_RECOVERY;
 for(const key of ['kind','build','networkConfig','wallet','roles','networkTag','expectedProtocolVersion'])check(isDeepStrictEqual(p[key],original[key]),'PREVIEW_RECOVERY_BINDING');
 check(d.transactionFile===join(fixed.originalDirectory,'run/public-transactions',fixed.transactionHash+'.bin'),'PREVIEW_RECOVERY_NATIVE_PATH');
 check(p.privateState.passwordFile===original.privateState.passwordFile,'PREVIEW_RECOVERY_PASSWORD');
 check(p.limits.allocationId!==original.limits.allocationId&&p.limits.dustFee===fixed.remainingDustFee&&isDeepStrictEqual(p.limits.grossByLogicalAsset,original.limits.grossByLogicalAsset),'PREVIEW_RECOVERY_BUDGET');
 const destinations=[p.outputDirectory,p.privateState.directory,d.inspectionDirectory];
 for(const path of destinations)check(typeof path==='string'&&isAbsolute(path)&&resolve(path)===path&&path!=='/','PREVIEW_RECOVERY_PATH');
 const overlaps=(a,b)=>a===b||a.startsWith(b+'/')||b.startsWith(a+'/');
 for(let i=0;i<destinations.length;i++){
  for(let j=i+1;j<destinations.length;j++)check(!overlaps(destinations[i],destinations[j]),'PREVIEW_RECOVERY_OVERLAP');
  for(const source of [fixed.originalDirectory,original.wallet.stateDirectory,dirname(original.wallet.seedFile)])check(!overlaps(destinations[i],source),'PREVIEW_RECOVERY_PRESERVE');
 }
 return structuredClone(d);
}
export function readPreviewRecoveryInputs(d,p,ledger){
 validatePreviewRecoveryPlan(d,p);const fixed=PREVIEW_LOAN_RECOVERY;
 // Inspect both immutable public receipts and the original live stop record.
 for(const [path,digest] of [
  [new URL('../../../deliverables/preview-loan-2026-09-17/actual-run01/reservations.json',import.meta.url).pathname,'3f499953f87bed0a76c5f683e23b52cd94b33c622c55086dad0c6e7c69b79143'],
  [join(fixed.originalDirectory,'run/reservations.json'),'3f499953f87bed0a76c5f683e23b52cd94b33c622c55086dad0c6e7c69b79143'],
  [new URL('../../../deliverables/preview-loan-2026-09-17/actual-run01/integration-result.json',import.meta.url).pathname,'05456d0bc0ffe9e2e99c637904d407f6ae315e500698a091bda9434b9ba3f25b'],
 ])check(sha(boundedPublicFile(path,128*1024))===digest,'PREVIEW_RECOVERY_HISTORY');
 const raw=boundedPublicFile(d.transactionFile,16*1024*1024);return {raw,binding:inspectPreviewLoanBytes(raw,ledger)};
}
export async function verifyPreviewLoanPublic({binding,ledger,provider,rpc,decodeState,deadlineMs,expectedProtocolVersion,networkTag,tip,readCurrentTip}){
 check(previewBindings.has(binding),'PREVIEW_RECOVERY_BINDING');
 check(expectedProtocolVersion===1000000&&networkTag===originalPreviewPlan().networkTag,'PREVIEW_RECOVERY_NETWORK');
 const {observeFinalizedStage,beforeDeadline}=await import('./receipt.mjs');
 check(await beforeDeadline(()=>rpc('chain_getBlockHash',[0]),deadlineMs)==='0x'+networkTag,'PREVIEW_GENESIS_MISMATCH');
 checkPublicRecoveryTip(tip,readCurrentTip);
 const observation=await observeFinalizedStage({provider,rpc,ledger,txId:binding.txId,contractAddress:binding.contractAddress,circuitId:'deploy',decodeState,deadlineMs,expectedProtocolVersion,network:'preview'});
 const r=observation.receipt;
 check(r.transaction.transactionHash===binding.transactionHash&&r.transaction.rawSha256===binding.transactionHash&&r.transaction.dustFee===PREVIEW_LOAN_RECOVERY.historicalDustFee&&r.blockHeight===PREVIEW_LOAN_RECOVERY.blockHeight&&r.blockHash===PREVIEW_LOAN_RECOVERY.blockHash,'PREVIEW_RECOVERY_RECEIPT');
 const {stateBlock,snapshotSamples}=await stableLoanSnapshot({rpc,deadlineMs,tip,readCurrentTip,finalityHeight:r.finalizedHeight,historyHeight:r.blockHeight,checkStates:async wait=>{
  const historical=await wait(()=>provider.queryContractState(binding.contractAddress,{type:'blockHash',blockHash:r.blockHash}));assertBoundLoanState(historical,ledger,binding);
  const current=await wait(()=>provider.queryContractState(binding.contractAddress));assertBoundLoanState(current,ledger,binding);
 }});
 return Object.freeze({binding,observation,tip:Object.freeze({...tip}),stateBlock,snapshotSamples});
}
export function assertPreviewRecoveryWallet(options){check(previewBindings.has(options.binding),'PREVIEW_RECOVERY_BINDING');check(options.synced?.unshielded?.pendingCoins?.length===0,'RECOVERY_WALLET_PENDING');return assertHistoryWallet(options);}
export async function reconstructPreviewLoan(options,binding){check(previewBindings.has(binding),'PREVIEW_RECOVERY_BINDING');return reconstructLoan(options,binding);}
export async function restorePreviewLoanPrivate(provider,result){check(previewBindings.has(privateBindings.get(result)),'PREVIEW_RECOVERY_BINDING');const restored=await restoreLoanPrivate(provider,result);const capability=Object.freeze({contractAddress:restored.contractAddress,txId:restored.txId,identifiers:PREVIEW_LOAN_RECOVERY.identifiers});previewCapabilities.add(capability);return capability;}
export function assertPreviewRecoveryCapability(value){check(previewCapabilities.has(value),'PREVIEW_RECOVERY_CAPABILITY');previewCapabilities.delete(value);return value;}
