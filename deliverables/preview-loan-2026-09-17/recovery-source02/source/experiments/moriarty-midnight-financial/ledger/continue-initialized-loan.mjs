/** Predicates for one retained initialized loan. No dispatch, finality check,
 * provider creation, constructor execution, or private-state persistence.
 * Callers must separately verify current chain state and admitted history.
 */
import {createHash} from 'node:crypto';
import {isDeepStrictEqual} from 'node:util';
import {decodeNativeFinancialTransaction} from './receipt.mjs';
import {decodeLocalIndexedOwner} from './indexed-owner.mjs';
const check=(ok,code)=>{if(!ok)throw Error(code);};
const sha=raw=>createHash('sha256').update(raw).digest('hex');
export const INITIALIZED_LOAN=Object.freeze({
 transactionHash:'1f64634de2761fc0f140dbe7784a0cbf7005f226799e94d9878932378bc731e6',
 txId:'006466368995501afc82b36cb38dac6f1cee531565eb75b70db6f3af5af36d9b46',
 identifiers:Object.freeze(['004b2aa75cdb10a080c3560a0af50ab17ef6e2e2294e1609a9e087f18498dc1d05','006466368995501afc82b36cb38dac6f1cee531565eb75b70db6f3af5af36d9b46']),
 contractAddress:'ba4c808859fc2e4ee6d3d19fa0d812bb9a9c9eb0527161fb91315213bc24a713',
 initializedStateSha256:'1f0724d2afe55e1c2aa22580f1bf9f0eed1a30e74b7a04ff78fe622a1e6fb30f',
});
const MINTED_OUTPUT=Object.freeze({segment:21861,section:'guaranteed',outputNo:0,intentHash:'4d343d21f150af922dc483b319b87b069a2b2cb8e3f3f64f4bad4485dbf61603',owner:'9a9de6549e2ea2fdd39578c3bd2e6f0327b3c5815f0ac061c20043ddf82918cb',type:'e92df6339320f55209d4586ce039b6ef05a7be960999fca913006146cb72cdef',value:'20000000000'});
/** Exact native bytes only; outputs/nullifiers support separately checked wallet gates. */
export function inspectInitializedLoanBytes(raw,ledger){
 check(raw instanceof Uint8Array&&raw.length>0&&raw.length<=16*1024*1024&&sha(raw)===INITIALIZED_LOAN.transactionHash,'INITIALIZED_NATIVE_HASH');
 const decoded=decodeNativeFinancialTransaction(raw,ledger);
 check(decoded.transactionHash===INITIALIZED_LOAN.transactionHash&&isDeepStrictEqual(decoded.identifiers,INITIALIZED_LOAN.identifiers),'INITIALIZED_NATIVE_IDENTIFIERS');
 const action=decoded.actions[0];
 check(decoded.actions.length===1&&action.kind==='call'&&action.entryPoint==='initialize'&&action.address===INITIALIZED_LOAN.contractAddress&&action.segment===21861,'INITIALIZED_NATIVE_ACTION');
 check(decoded.outputs.length===1,'INITIALIZED_NATIVE_OUTPUT');
 const {offerIndex,...output}=decoded.outputs[0],mintedOutput={...output,outputNo:offerIndex};
 check(isDeepStrictEqual(mintedOutput,MINTED_OUTPUT),'INITIALIZED_NATIVE_OUTPUT');
 const native=ledger.Transaction.deserialize('signature','proof','binding',raw);
 const oldDustNullifiers=[...native.intents.values()].flatMap(i=>i.dustActions?.spends??[]).map(s=>s.oldNullifier);
 check(oldDustNullifiers.length>0&&oldDustNullifiers.every(n=>typeof n==='bigint'),'INITIALIZED_NATIVE_DUST_INPUTS');
 return Object.freeze({...INITIALIZED_LOAN,mintedOutput:Object.freeze(mintedOutput),oldDustNullifiers:Object.freeze(oldDustNullifiers),spentUnshieldedInputs:Object.freeze(decoded.inputs.map(input=>Object.freeze({...input}))),proofVerified:false,ledgerAccepted:false});
}
/** Require complete native serialized state, not merely initialized/revision flags. */
export function assertInitializedLoanState(state,ledger){
 check(state&&typeof state.serialize==='function','INITIALIZED_STATE_TYPE');
 const raw=Buffer.from(state.serialize());
 check(raw.length<=16*1024*1024&&sha(raw)===INITIALIZED_LOAN.initializedStateSha256,'INITIALIZED_STATE_MISMATCH');
 const native=ledger.ContractState.deserialize(raw);
 check(Buffer.from(native.serialize()).equals(raw),'INITIALIZED_STATE_CANONICAL');
 return true;
}
function emptyPrivateState(value){
 return value!==null&&typeof value==='object'&&Object.getPrototypeOf(value)===Object.prototype&&Reflect.ownKeys(value).length===0;
}
/** Read the existing store only, after the caller's public and preservation gates.
 * setContractAddress selects the provider namespace; no private value is written.
 * Returned evidence is public and carries no authority to dispatch transactions.
 */
export async function verifyInitializedLoanPrivate({provider,signingKey,contractState,ledger}){
 assertInitializedLoanState(contractState,ledger);
 const authority=ledger.ContractState.deserialize(contractState.serialize()).maintenanceAuthority;
 let verifyingKey;
 try{verifyingKey=ledger.signatureVerifyingKey(signingKey);}catch{throw Error('INITIALIZED_SIGNING_AUTHORITY');}
 check(authority.threshold===1&&authority.counter===0n&&authority.committee.length===1&&authority.committee[0]===verifyingKey,'INITIALIZED_SIGNING_AUTHORITY');
 let setAddress,get,getKey;
 try{setAddress=provider?.setContractAddress;get=provider?.get;getKey=provider?.getSigningKey;}catch{throw Error('INITIALIZED_PRIVATE_PROVIDER');}
 check([setAddress,get,getKey].every(fn=>typeof fn==='function'),'INITIALIZED_PRIVATE_PROVIDER');
 let privateState;
 try{await setAddress.call(provider,INITIALIZED_LOAN.contractAddress);privateState=await get.call(provider,'sp05-loan');}catch{throw Error('INITIALIZED_PRIVATE_READ');}
 check(emptyPrivateState(privateState),'INITIALIZED_PRIVATE_STATE');
 let storedKey;
 try{storedKey=await getKey.call(provider,INITIALIZED_LOAN.contractAddress);}catch{throw Error('INITIALIZED_PRIVATE_READ');}
 check(storedKey===signingKey,'INITIALIZED_PRIVATE_KEY');
 return Object.freeze({status:'PRIVATE_STATE_CHECKED',contractAddress:INITIALIZED_LOAN.contractAddress,txId:INITIALIZED_LOAN.txId,dispatchAuthorized:false,scope:'Existing private state/key equality only; no chain finality, current-state or execution authorization'});
}

/** Only the exact initialized output can fund the continuation. Child synchronization,
 * spent-input/DUST absence and reservation caps are separate required wallet gates.
 */
export function assertInitializedLoanMintedOutput({synced,binding}){
 check(binding?.transactionHash===INITIALIZED_LOAN.transactionHash&&isDeepStrictEqual(binding.mintedOutput,MINTED_OUTPUT),'INITIALIZED_WALLET_BINDING');
 const u=synced?.unshielded;check(Array.isArray(u?.availableCoins)&&Array.isArray(u?.pendingCoins),'INITIALIZED_WALLET_COINS');
 const matches=coin=>coin?.utxo?.intentHash===MINTED_OUTPUT.intentHash&&coin.utxo.outputNo===MINTED_OUTPUT.outputNo;
 const available=u.availableCoins.filter(matches);check(available.length===1&&!u.pendingCoins.some(matches),'INITIALIZED_WALLET_AVAILABLE');
 const coin=available[0].utxo;let owner;try{owner=decodeLocalIndexedOwner(coin.owner);}catch{throw Error('INITIALIZED_WALLET_OUTPUT');}check(owner===MINTED_OUTPUT.owner&&coin.type===MINTED_OUTPUT.type&&typeof coin.value==='bigint'&&coin.value===BigInt(MINTED_OUTPUT.value),'INITIALIZED_WALLET_OUTPUT');
 return true;
}
