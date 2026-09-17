/** Fixed retained-swap rejection probe. No proving, wallet, private-store or network API.
 * This checks input object preservation at call preparation, not ledger rollback.
 */
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {serialize} from 'node:v8';
import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {PINNED_NM} from './providers.mjs';
const sha=b=>createHash('sha256').update(b).digest('hex');
const bytes=s=>Uint8Array.from(Buffer.from(s,'hex'));
const STATE_SHA='5a1dcdb726f25a724f89d0185af0fb8bb863b37e094fce8fc7dc1e75caf2b92c';
const BUILD_SHA='3789da217a36cecd5f7603cbbaead32671418da36ecc5c2d20b1709b9577f362';
const SOURCE_SHA='a29775a104dde9dbc38fdcbfdbc25a31db63beec84e09440f73441a27e9422e6';
const PINS={
 'midnight-js-contracts/dist/index.mjs':'9c8079430513e7b459d52fc6d22ab5dede22f86073a4b50dcbdf35de99bb4072',
 'midnight-js-protocol/dist/ledger.mjs':'4db121cdee21bb88cecca5dc1433c6d30cc12c1ac1f4c5e40a606440e08ac840',
 'midnight-js-protocol/dist/compact-runtime.mjs':'24e261044bcf1b12452393d31a0d1965335397e03a32b288822ffd06fb1c012b',
};
async function pinned(name){const path=join(PINNED_NM,'@midnight-ntwrk',name);if(sha(readFileSync(path))!==PINS[name])throw Error('COMPILED_REJECTION_SDK_PIN');return import(pathToFileURL(path).href);}
export async function checkCompiledSwapRejection(options){
 if(!options||Object.getPrototypeOf(options)!==Object.prototype||Reflect.ownKeys(options).sort().join(',')!=='mutation,receiptPath'||Object.values(Object.getOwnPropertyDescriptors(options)).some(d=>!Object.hasOwn(d,'value'))||typeof options.receiptPath!=='string'||!['revision','program','network'].includes(options.mutation))throw Error('COMPILED_REJECTION_OPTIONS');
 const {receiptPath,mutation}=options;
 const retained=readFileSync(new URL('../../../deliverables/sp05-financial-integration-2026-09-09/swap-initialize-diagnosis-01/indexed-initialize-state.bin',import.meta.url));
 if(retained.length!==8150||sha(retained)!==STATE_SHA)throw Error('COMPILED_REJECTION_STATE_PIN');
 const sdk=await pinned('midnight-js-contracts/dist/index.mjs');
 const runtime=await pinned('midnight-js-protocol/dist/compact-runtime.mjs');
 const ledger=await pinned('midnight-js-protocol/dist/ledger.mjs');
 const state=runtime.ContractState.deserialize(retained),privateState={},beforePrivate=serialize(privateState);
 const {loadProvenFinancialContract}=await import('./proven-assets.mjs');
 const loaded=await loadProvenFinancialContract({case:'swap',receiptPath,receiptSha256:BUILD_SHA,sourceManifestHash:SOURCE_SHA});
 let error,returned=false,loaderClosed=false,zkConfigCalls=0;
 const forbidden=async()=>{zkConfigCalls++;throw Error('COMPILED_REJECTION_ZK_CONFIG_ACCESSED');};
 // This provider IS connected to the SDK call. Other provider interfaces are
 // absent from this API; no disconnected counters purport to observe them.
 const zk={getVerifierKey:forbidden,getProverKey:forbidden,getZKIR:forbidden};
 try{
  try{
   await sdk.createUnprovenCallTxFromInitialStates(zk,{
    compiledContract:loaded.compiledContract,contractAddress:'8824d69c9058f322b4f6da7e7cd8d49f3235db5fbe3d6080d25f239243812261',
    coinPublicKey:'00'.repeat(32),initialContractState:state,initialPrivateState:privateState,initialZswapChainState:undefined,
    ledgerParameters:ledger.LedgerParameters.initialParameters(),circuitId:'close',
    // Synthetic role bytes only; these guards precede role-capability checks.
    args:[new Uint8Array(32).fill(7),bytes(mutation==='program'?'00'.repeat(32):'b00a55b8da611d23c08461a11853151aa50ec7ee8150f9d16afbc66d62e018fa'),bytes(mutation==='network'?'00'.repeat(32):'e72f7a21a0397844563b4206f887b779ffa0d937c2d1b2339441faa1f08b9846'),mutation==='revision'?99n:0n,3n,1789040000n,{unused:0n}],
   },'00'.repeat(32));returned=true;
  }catch(e){error=e;}
  if(!Buffer.from(state.serialize()).equals(retained))throw Error('COMPILED_REJECTION_STATE_MUTATED');
  if(!serialize(privateState).equals(beforePrivate))throw Error('COMPILED_REJECTION_PRIVATE_STATE_MUTATED');
  if(zkConfigCalls!==0)throw Error('COMPILED_REJECTION_ZK_CONFIG_ACCESSED');
  if(returned)throw Error('COMPILED_REJECTION_UNEXPECTED_SUCCESS');
  const code=mutation.toUpperCase()+'_MISMATCH',message='failed assert: '+code;
  if(error?.name!=='Error'||error.message!==message||error.cause?._tag!=='ContractRuntimeError'||error.cause?.cause?.name!=='CompactError'||error.cause.cause.message!==message)throw Error('COMPILED_REJECTION_UNEXPECTED_ERROR');
 }finally{
  try{loaderClosed=loaded.cleanup()?.loaderHooksRemoved===true;}catch{throw Error('COMPILED_REJECTION_CLEANUP');}
  if(!loaderClosed)throw Error('COMPILED_REJECTION_CLEANUP');
 }
 return Object.freeze({status:'REJECTED_BEFORE_TRANSACTION',boundary:'createUnprovenCallTxFromInitialStates',code:mutation.toUpperCase()+'_MISMATCH',stateSha256:STATE_SHA,buildReceiptSha256:BUILD_SHA,stateUnchanged:true,privateStateUnchanged:true,zkConfigCalls,loaderClosed,currentLedgerStateChecked:false});
}
