/** Fixed stale initialized-loan preparation. No admission, provider startup or retry.
 * Caller must preserve the existing store BEFORE constructing financial providers,
 * reconcile all four historical transactions and synchronize wallet/spent DUST.
 * The returned handle is privacy-sensitive and is never a public receipt.
 */
import {readFileSync} from 'node:fs';import {createHash} from 'node:crypto';import {serialize} from 'node:v8';import {isDeepStrictEqual} from 'node:util';import {pathToFileURL} from 'node:url';
import {PINNED_NM} from './providers.mjs';import {loadProvenFinancialContract} from './proven-assets.mjs';
import {captureFinalizedFinancialState} from './finalized-financial-state.mjs';import {beforeDeadline,decodeNativeFinancialTransaction} from './receipt.mjs';
import {verifyInitializedLoanPrivate} from './continue-initialized-loan.mjs';
const ADDRESS='ba4c808859fc2e4ee6d3d19fa0d812bb9a9c9eb0527161fb91315213bc24a713';
const CURRENT='552d58ff2918332665b179b37a70907c4492a02b5c56d7db05dce5e427e573e0',INITIAL='1f0724d2afe55e1c2aa22580f1bf9f0eed1a30e74b7a04ff78fe622a1e6fb30f';
const hash=b=>createHash('sha256').update(b).digest('hex'),bytes=s=>Uint8Array.from(Buffer.from(s,'hex'));
const requireThat=(ok,code)=>{if(!ok)throw Error('STALE_LOAN_'+code);};
const handles=new WeakMap();
const effect=await import(pathToFileURL(PINNED_NM+'/effect/dist/esm/index.js').href);
const {TransactionInvalidError,SubmissionError}=await import(pathToFileURL(PINNED_NM+'/@midnight-ntwrk/wallet-sdk-node-client/dist/effect/NodeClientError.js').href);
const {SubmissionError:WalletSubmissionError}=await import(pathToFileURL(PINNED_NM+'/@midnight-ntwrk/wallet-sdk-capabilities/dist/submission/submissionService.js').href);
const {default:RpcError}=await import(pathToFileURL(PINNED_NM+'/@polkadot/rpc-provider/coder/error.js').href);
async function dependencies(){
 const path=PINNED_NM+'/@midnight-ntwrk/midnight-js-contracts/dist/index.mjs';requireThat(hash(readFileSync(path))==='9c8079430513e7b459d52fc6d22ab5dede22f86073a4b50dcbdf35de99bb4072','SDK_PIN');
 return {sdk:await import(pathToFileURL(path).href),runtime:await import(pathToFileURL(PINNED_NM+'/@midnight-ntwrk/midnight-js-protocol/dist/compact-runtime.mjs').href),ledger:await import(pathToFileURL(PINNED_NM+'/@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs').href)};
}
function checkSnapshot(s){requireThat(s?.contractAddress===ADDRESS&&s.source==='midnight_contractState-at-explicit-finalized-block'&&Number.isSafeInteger(s.blockHeight)&&s.blockHeight>=20415&&/^0x[a-f0-9]{64}$/.test(s.blockHash),'SNAPSHOT');requireThat(typeof s.serializedStateHex==='string'&&hash(Buffer.from(s.serializedStateHex,'hex'))===CURRENT&&s.stateSha256===CURRENT,'CURRENT_STATE');}
/** A later anchor is necessary, but caller still must establish that it covers the
 * completed submission outcome. This function alone never accepts a rejection.
 */
export function compareStaleLoanSnapshots(before,after){checkSnapshot(before);checkSnapshot(after);requireThat(after.blockHeight>before.blockHeight&&after.blockHash!==before.blockHash,'AFTER_ANCHOR');requireThat(before.serializedStateHex===after.serializedStateHex&&isDeepStrictEqual(before.balances,after.balances)&&isDeepStrictEqual(before.state,after.state),'FINANCIAL_MUTATION');return Object.freeze({status:'FINANCIAL_STATE_UNCHANGED',beforeHash:before.blockHash,afterHash:after.blockHash,stateSha256:CURRENT,nodeRejectionEstablished:false,scope:'Full serialized contract state and decoded financial fields/balances equality; no fees or rejection acceptance inferred'});}
export function classifyStaleLoanFailure(error,phase,submittedBytes){
 requireThat(['prepare','prove','balance','submit','observe'].includes(phase),'PHASE');
 // SDK Effect.runPromise may wrap node failures. Do not recognize names, text or
 // generic SubmissionError as a ledger rejection, or serialize its txData.
 let failure=error;if(effect.Runtime.isFiberFailure(error)){const cause=error[effect.Runtime.FiberFailureCauseId];failure=effect.Cause.isFailType(cause)?effect.Option.getOrUndefined(effect.Cause.failureOption(cause)):undefined;}
 // The default submission service wraps exactly one node-client failure.
 if(failure instanceof WalletSubmissionError)failure=failure.cause;
 if(phase==='submit'&&failure instanceof TransactionInvalidError&&submittedBytes instanceof Uint8Array&&submittedBytes.length>0&&failure.txData instanceof Uint8Array&&Buffer.from(failure.txData).equals(Buffer.from(submittedBytes)))return Object.freeze({status:'NODE_REJECTED',phase,code:'NODE_TRANSACTION_INVALID',nodeRejectionEstablished:true,stalePredicateEstablished:false});
 const rpcCode=phase==='submit'&&failure instanceof SubmissionError&&failure.txData instanceof Uint8Array&&submittedBytes instanceof Uint8Array&&Buffer.from(failure.txData).equals(Buffer.from(submittedBytes))&&failure.cause instanceof RpcError&&failure.cause.code===1010?1010:undefined;
 const codes=['OBSERVATION_TIMEOUT_UNKNOWN','RPC_DEADLINE','STALE_LOAN_DEADLINE'];
 return Object.freeze({status:phase==='submit'||phase==='observe'?'OUTCOME_UNKNOWN':'REJECTED_BEFORE_SUBMISSION',phase,...(rpcCode===undefined?{}:{rpcCode}),code:codes.includes(error?.message)?error.message:'UNCLASSIFIED_'+phase.toUpperCase()+'_FAILURE',nodeRejectionEstablished:false});
}
export async function prepareStaleLoanAccrue({providers,rpc,receiptPath,borrowerSecret,signingKey,deadlineMs,nowSeconds}){
 requireThat(Number.isSafeInteger(deadlineMs)&&deadlineMs>Date.now(),'DEADLINE');requireThat(borrowerSecret instanceof Uint8Array&&borrowerSecret.length===32&&typeof nowSeconds==='bigint'&&nowSeconds>=0n&&nowSeconds<2n**64n,'ARGUMENTS');
 const within=async(f,limit=deadlineMs)=>{const stop=Math.min(limit,deadlineMs);requireThat(Date.now()<stop,'DEADLINE');const value=await beforeDeadline(f,stop);requireThat(Date.now()<stop,'DEADLINE');return value;};
 const {sdk,runtime,ledger}=await dependencies();let loaded;
 try{
  loaded=await within(()=>loadProvenFinancialContract({case:'loan',receiptPath,receiptSha256:'51ee2d4d60216464a9ace67966ba0ab253699844187aeb5652b46dc6e9ca5bf7',sourceManifestHash:'a29775a104dde9dbc38fdcbfdbc25a31db63beec84e09440f73441a27e9422e6'}));
  const before=await within(()=>captureFinalizedFinancialState({contractAddress:ADDRESS,rpc,loadedContract:loaded,deadlineMs}));checkSnapshot(before);
  const publicStates=await within(()=>providers.publicDataProvider.queryZSwapAndContractState(ADDRESS));
  requireThat(Array.isArray(publicStates)&&publicStates.length===3,'PUBLIC_STATES');const [zswap,current,parameters]=publicStates;
  requireThat(zswap instanceof ledger.ZswapChainState&&parameters instanceof ledger.LedgerParameters&&current instanceof runtime.ContractState&&Buffer.from(current.serialize()).toString('hex')===before.serializedStateHex,'PUBLIC_STATES');
  const raw=readFileSync(new URL('../../../deliverables/sp05-financial-integration-2026-09-09/local-recovery-03/indexed-initialize-state.bin',import.meta.url));requireThat(hash(raw)===INITIAL,'INITIAL_STATE_PIN');
  const initial=runtime.ContractState.deserialize(raw),privateState={};
  try{
   requireThat((await within(()=>verifyInitializedLoanPrivate({provider:providers.privateStateProvider,signingKey,contractState:initial,ledger})))?.status==='PRIVATE_STATE_CHECKED','PRIVATE_STATE');
   const B=1n<<64n,mul=(a,b)=>({aLo:a%B,aHi:a/B,bLo:b%B,bHi:b/B,lo:(a%B)*(b%B)%B,carry:(a%B)*(b%B)/B}),div=(n,d)=>({q:n/d,r:n%d,product:mul(d,n/d)});
   const privateBefore=serialize(privateState);loaded.assertFresh();
   const prepared=await within(()=>sdk.createUnprovenCallTxFromInitialStates(providers.zkConfigProvider,{compiledContract:loaded.compiledContract,contractAddress:ADDRESS,coinPublicKey:providers.walletProvider.getCoinPublicKey(),initialContractState:initial,initialPrivateState:privateState,initialZswapChainState:zswap,ledgerParameters:parameters,circuitId:'accrue',args:[borrowerSecret,bytes('95b46e39a9039e19063bb3d618128aec6cbd9ee656b6e635b3587e7f3f5235b2'),bytes('e72f7a21a0397844563b4206f887b779ffa0d937c2d1b2339441faa1f08b9846'),0n,2n,nowSeconds,{h0:mul(5000000000n,8n),h1:mul(40000000000n,31n),h2:mul(100n,365n),h3:div(1240000000000n,36500n)}]},providers.walletProvider.getEncryptionPublicKey()));
   requireThat(Buffer.from(initial.serialize()).equals(raw)&&serialize(privateState).equals(privateBefore),'INPUT_MUTATION');
   const handle=Object.freeze({status:'PREPARED_STALE_ACCRUE',before,privacySensitive:true});handles.set(handle,{prepared,providers,loaded,rpc,deadlineMs,within,sdk,ledger,used:false});loaded=undefined;return handle;
  }finally{initial.free();}
 }finally{loaded?.cleanup();}
}
/** Exactly one attempt through existing proof, balance and issued-ticket submit
 * providers. Caller owns provider/store preservation, wallet gates, reservations
 * and cleanup. retainCandidate must durably retain PUBLIC finalized bytes only.
 */
export async function submitPreparedStaleLoan(handle,{retainCandidate,retainSubmissionOutcome}){
 const h=handles.get(handle);requireThat(h&&!h.used&&typeof retainCandidate==='function'&&typeof retainSubmissionOutcome==='function','HANDLE');h.used=true;handles.delete(handle);
 let phase='prove',raw,candidate,rejection,providerSubmissionAttempted=false;
 try{
  h.loaded.assertFresh();
  const proven=await h.within(()=>h.providers.proofProvider.proveTx(h.prepared.private.unprovenTx));
  phase='balance';const finalized=await h.within(()=>h.providers.walletProvider.balanceTx(proven));
  raw=Buffer.from(finalized.serialize());candidate=decodeNativeFinancialTransaction(raw,h.ledger);
  const a=candidate.actions[0];requireThat(candidate.actions.length===1&&a.kind==='call'&&a.address===ADDRESS&&a.entryPoint==='accrue'&&candidate.inputs.length===0&&candidate.outputs.length===0&&a.transcripts.length===1&&a.transcripts[0].section==='guaranteed','CANDIDATE');
  requireThat(Object.keys(a.transcripts[0].effects).sort().join(',')==='claimedUnshieldedSpends,unshieldedInputs,unshieldedMints,unshieldedOutputs'&&Object.values(a.transcripts[0].effects).every(v=>Array.isArray(v)?v.length===0:Object.keys(v).length===0),'FINANCIAL_EFFECTS');
  const retained=await h.within(()=>retainCandidate({raw:Uint8Array.from(raw),transaction:structuredClone(candidate)}));requireThat(retained?.status==='RECORDED'&&retained.rawSha256===candidate.rawSha256,'CANDIDATE_RETENTION');
  requireThat(Buffer.from(finalized.serialize()).equals(raw),'CANDIDATE_MUTATION');
  phase='submit';
  try{await h.within(()=>{providerSubmissionAttempted=true;return h.providers.midnightProvider.submitTx(finalized);});return {status:'UNEXPECTED_SUBMISSION_SUCCESS',candidate,financialNonmutationEstablished:false};}
  catch(error){rejection=classifyStaleLoanFailure(error,phase,raw);}
  const outcome={candidateRawSha256:candidate.rawSha256,...rejection};
  const recorded=await h.within(()=>retainSubmissionOutcome(outcome));requireThat(recorded?.status==='RECORDED'&&recorded.candidateRawSha256===candidate.rawSha256&&recorded.code===rejection.code,'OUTCOME_RETENTION');
  phase='observe';
  // Terminal outcome is already retained before this barrier is sampled.
  const barrierHash=await h.within(()=>h.rpc('chain_getFinalizedHead',[],h.deadlineMs));requireThat(/^0x[a-f0-9]{64}$/.test(barrierHash),'BARRIER');
  const header=await h.within(()=>h.rpc('chain_getHeader',[barrierHash],h.deadlineMs));requireThat(/^0x[0-9a-f]{1,16}$/i.test(header?.number),'BARRIER');const barrier=Number(BigInt(header.number));requireThat(Number.isSafeInteger(barrier)&&barrier>=handle.before.blockHeight,'BARRIER');
  let advanced=false;const observationStop=Math.min(h.deadlineMs,Date.now()+60000);
  for(let sample=0;sample<60&&Date.now()<observationStop;sample++){
   const head=await h.within(()=>h.rpc('chain_getFinalizedHead',[],observationStop),observationStop);requireThat(/^0x[a-f0-9]{64}$/.test(head),'BARRIER');
   if(head!==barrierHash){const next=await h.within(()=>h.rpc('chain_getHeader',[head],observationStop),observationStop);requireThat(/^0x[0-9a-f]{1,16}$/i.test(next?.number),'BARRIER');const height=Number(BigInt(next.number));requireThat(Number.isSafeInteger(height)&&height>=barrier,'BARRIER');if(height>barrier){advanced=true;break;}}
   await beforeDeadline(()=>new Promise(resolve=>setTimeout(resolve,Math.min(1000,observationStop-Date.now()))),observationStop);
  }
  requireThat(advanced,'AFTER_SUBMISSION_ANCHOR');
  const after=await h.within(()=>captureFinalizedFinancialState({contractAddress:ADDRESS,rpc:h.rpc,loadedContract:h.loaded,deadlineMs:h.deadlineMs}));
  requireThat(after.blockHeight>barrier,'AFTER_SUBMISSION_ANCHOR');
  const equality=compareStaleLoanSnapshots(handle.before,after);
  return {status:rejection.nodeRejectionEstablished?'NODE_REJECTION_FINANCIAL_NONMUTATION':'OUTCOME_UNKNOWN',rejection,candidate,before:handle.before,terminalBarrier:{blockHash:barrierHash,blockHeight:barrier},after,equality,financialNonmutationEstablished:rejection.nodeRejectionEstablished,feeAccounting:{candidateDustFeeSpeck:candidate.dustFee,paidFeeSpeck:null,paidStatus:'UNKNOWN',reservationReleasePerformed:false},scope:'Retained node invalid status and full contract financial nonmutation only; invalid predicate detail and wallet fees remain separate'};
 }catch(error){return {status:'INCOMPLETE',failure:classifyStaleLoanFailure(error,phase),providerSubmissionAttempted,...(candidate?{candidate}:{}),...(rejection?{rejection}:{}),financialNonmutationEstablished:false};}
 finally{requireThat(h.loaded.cleanup()?.loaderHooksRemoved===true,'CLEANUP');requireThat(Date.now()<h.deadlineMs,'DEADLINE');}
}
export function closeStaleLoanPreparation(handle){const h=handles.get(handle);requireThat(h&&!h.used,'HANDLE');h.used=true;handles.delete(handle);requireThat(h.loaded.cleanup()?.loaderHooksRemoved===true,'CLEANUP');return {loaderClosed:true};}
