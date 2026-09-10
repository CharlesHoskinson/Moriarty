/** Read-only node state at an explicit finalized anchor. The node RPC remains
 * trusted: this observation is not an authenticated state proof or rollback test.
 */
import {createHash} from 'node:crypto';
import {pathToFileURL} from 'node:url';
import {beforeDeadline} from './receipt.mjs';
import {extractNativeContractBalances} from './contract-balances.mjs';
import {PINNED_NM} from './providers.mjs';
// contract-balances checks this exact protocol/runtime entry and package pin.
const {ContractState}=await import(pathToFileURL(PINNED_NM+'/@midnight-ntwrk/midnight-js-protocol/dist/compact-runtime.mjs').href);
const check=(ok,code)=>{if(!ok)throw Error('FINALIZED_STATE_'+code);};
/** Caller supplies the existing bounded RPC and verified build loader. Neither
 * is started or closed here. RPC must admit midnight_contractState(address, at).
 */
export async function captureFinalizedFinancialState({contractAddress,rpc,loadedContract,deadlineMs}){
 check(typeof contractAddress==='string'&&/^[a-f0-9]{64}$/.test(contractAddress),'ADDRESS');
 check(typeof rpc==='function'&&typeof loadedContract?.decodeState==='function','INTERFACES');
 if(!Number.isSafeInteger(deadlineMs)||deadlineMs<=Date.now())throw Error('DEADLINE_EXPIRED');
 const stop=Math.min(deadlineMs,Date.now()+60000);
 const inTime=()=>{if(Date.now()>=stop)throw Error('OBSERVATION_TIMEOUT_UNKNOWN');};
 const call=async(method,params)=>{
  inTime();
  const result=await beforeDeadline(()=>rpc(method,params,stop),stop);
  // Timers cannot interrupt synchronous work; a resolved RPC can still be late.
  inTime();return result;
 };
 const blockHash=await call('chain_getFinalizedHead',[]);
 check(typeof blockHash==='string'&&/^0x[a-f0-9]{64}$/.test(blockHash),'HEAD');
 const header=await call('chain_getHeader',[blockHash]);
 check(typeof header?.number==='string'&&/^0x[0-9a-f]+$/i.test(header.number)&&header.number.length<=18,'HEADER');
 const blockHeight=Number(BigInt(header.number));check(Number.isSafeInteger(blockHeight)&&blockHeight>=0,'HEIGHT');
 check(await call('chain_getBlockHash',[blockHeight])===blockHash,'NONCANONICAL');
 // Mandatory second argument prevents the RPC's implicit best-block fallback.
 const serializedStateHex=await call('midnight_contractState',[contractAddress,blockHash]);
 check(typeof serializedStateHex==='string'&&serializedStateHex.length>0&&serializedStateHex.length<=2097152&&/^(?:[a-f0-9]{2})+$/.test(serializedStateHex),'ENCODING');
 const raw=Buffer.from(serializedStateHex,'hex');let native,snapshot;
 try{native=ContractState.deserialize(raw);if(!Buffer.from(native.serialize()).equals(raw))throw Error('roundtrip');}
 catch{native?.free();throw Error('FINALIZED_STATE_NATIVE');}
 try{
  const balances=extractNativeContractBalances({state:native});
  const decoded=loadedContract.decodeState(native.data);
  check(decoded&&Object.getPrototypeOf(decoded)===Object.prototype&&Object.keys(decoded).length>0,'DECODE');
  // Detach every public field from the generated decoder's getter view.
  const state=structuredClone(decoded);
  check(Buffer.from(native.serialize()).equals(raw),'DECODE_MUTATED_NATIVE');
  check(await call('chain_getBlockHash',[blockHeight])===blockHash,'NONCANONICAL');
  snapshot=Object.freeze({schema:'moriarty.finalized-financial-state/1',contractAddress,blockHash,blockHeight,serializedStateHex,stateSha256:createHash('sha256').update(raw).digest('hex'),balances,state,source:'midnight_contractState-at-explicit-finalized-block',authenticatedStateProof:false,noInterveningActionsAfterAnchorEstablished:false});
 }finally{native.free();}
 inTime();
 return snapshot;
}
