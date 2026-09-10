/** One retained initialized swap only. No explicit store creation or writes,
 * dispatch, service, finality or acceptance authority. Passed provider reads may
 * have SDK side effects; caller first preserves the existing store and verifies
 * history/current public state.
 */
import {createHash} from 'node:crypto';
import {isDeepStrictEqual} from 'node:util';
import {decodeNativeFinancialTransaction} from './receipt.mjs';
import {projectUnshieldedContractBalanceMap} from './contract-balances.mjs';
import {assertSwapInitializedWallet} from './swap-wallet.mjs';
const check=(ok,label)=>{if(!ok)throw Error('SWAP_CONTINUATION_'+label);};
const sha=raw=>createHash('sha256').update(raw).digest('hex');
export const INITIALIZED_SWAP=Object.freeze({"transactionHash":"3fec717c8d31e6da9b9ace76c9f209bdbda11fe9d27f5d870c6edf428d2be7b4","txId":"00d081a157b4485e2df800852750a60ef2d7071f7c347490b775a4c6595d78d42c","contractAddress":"8824d69c9058f322b4f6da7e7cd8d49f3235db5fbe3d6080d25f239243812261","initializedStateSha256":"5a1dcdb726f25a724f89d0185af0fb8bb863b37e094fce8fc7dc1e75caf2b92c","buildReceiptSha256":"3789da217a36cecd5f7603cbbaead32671418da36ecc5c2d20b1709b9577f362","sourceManifestHash":"a29775a104dde9dbc38fdcbfdbc25a31db63beec84e09440f73441a27e9422e6","networkTag":"e72f7a21a0397844563b4206f887b779ffa0d937c2d1b2339441faa1f08b9846","expectedProtocolVersion":1000000,"traderAddress":"9a9de6549e2ea2fdd39578c3bd2e6f0327b3c5815f0ac061c20043ddf82918cb","providerAddress":"c1d1141a7f08931d16f3fe4cec1c57d66ab2d11d04e4ab7abb61121ecad5e61e",identifiers:Object.freeze(["0003173e5329ad680ae188223b53711fd36c915ea48480551297983668d1abdb37", "00d081a157b4485e2df800852750a60ef2d7071f7c347490b775a4c6595d78d42c"])});
export const EXISTING_SWAP=Object.freeze({"transactionHash":"31677c341ab6900f144b5205c6be51dbaf81a06607307e7864be378a4472f91d","txId":"009936facb7e8454084dca61c325da89ffb5f7b4d6f4c54770872b456574eb0da6","contractAddress":"8824d69c9058f322b4f6da7e7cd8d49f3235db5fbe3d6080d25f239243812261","initialStateSha256":"c76e8b3604c83b2f19d6dec2c7ee8b96b06fa7f159374a3847b36537e754ffec",identifiers:Object.freeze(["000ac9a488b2781aeb373d798f3531b669504f826e69c125833ae6069935fcae8d", "009936facb7e8454084dca61c325da89ffb5f7b4d6f4c54770872b456574eb0da6"])});
const MINTED_OUTPUT=Object.freeze({"segment": 36489, "section": "fallible", "owner": "9a9de6549e2ea2fdd39578c3bd2e6f0327b3c5815f0ac061c20043ddf82918cb", "type": "5475695f8ccb85c05055a4ffef06bbbff208c58b07cf5c61052d25143729d166", "value": "100000", "intentHash": "cc0511b9583d85ccea1e63ba024a04a6f0dd1dce86aa0f83e92f8917b42e3548", "outputNo": 0});
const BALANCES=Object.freeze({"e7d969726b0884ada7bb477283143911dc4df1b18da25a776a433bfaac8646d4": "2000000", "5475695f8ccb85c05055a4ffef06bbbff208c58b07cf5c61052d25143729d166": "1000000"});
const AUTHORITY='5702bb28fce06bea68b4fb2dad29ca6694c4021cbc1bf3f04d995cbb885815d2';
function inspect(raw,ledger,identity,kind){
 check(raw instanceof Uint8Array&&raw.length>0&&raw.length<=16*1024*1024&&sha(raw)===identity.transactionHash,'NATIVE_HASH');
 const decoded=decodeNativeFinancialTransaction(raw,ledger),tx=ledger.Transaction.deserialize('signature','proof','binding',raw);
 check(decoded.transactionHash===identity.transactionHash&&isDeepStrictEqual(decoded.identifiers,identity.identifiers),'NATIVE_IDENTIFIERS');
 const a=decoded.actions[0];check(decoded.actions.length===1&&a.address===identity.contractAddress&&a.kind===kind&&a.segment===(kind==='deploy'?1:36489)&&(kind==='deploy'||a.entryPoint==='initialize'),'NATIVE_ACTION');
 check(decoded.inputs.length===0,'NATIVE_INPUTS');
 const oldDustNullifiers=[...tx.intents.values()].flatMap(i=>i.dustActions?.spends??[]).map(s=>s.oldNullifier);check(oldDustNullifiers.length>0&&oldDustNullifiers.every(n=>typeof n==='bigint'),'NATIVE_DUST');
 return {decoded,tx,binding:{...identity,oldDustNullifiers:Object.freeze(oldDustNullifiers),spentUnshieldedInputs:Object.freeze(decoded.inputs.map(x=>Object.freeze({...x}))),proofVerified:false,ledgerAccepted:false}};
}
export function inspectExistingSwapBytes(raw,ledger){
 const {decoded,tx,binding}=inspect(raw,ledger,EXISTING_SWAP,'deploy');check(decoded.outputs.length===0,'NATIVE_OUTPUT');
 const a=[...tx.intents.values()].flatMap(i=>i.actions)[0];check(a instanceof ledger.ContractDeploy&&sha(a.initialState.serialize())===EXISTING_SWAP.initialStateSha256,'NATIVE_DEPLOY_STATE');return Object.freeze(binding);
}
export function inspectInitializedSwapBytes(raw,ledger){
 const {decoded,binding}=inspect(raw,ledger,INITIALIZED_SWAP,'call');check(decoded.outputs.length===1,'NATIVE_OUTPUT');const {offerIndex,...row}=decoded.outputs[0],mintedOutput={...row,outputNo:offerIndex};check(isDeepStrictEqual(mintedOutput,MINTED_OUTPUT),'NATIVE_OUTPUT');return Object.freeze({...binding,mintedOutput:Object.freeze(mintedOutput)});
}
/** SDK and ledger state producers differ; serialize explicitly before native inspection. */
export function assertInitializedSwapState(state,ledger){
 check(state&&typeof state.serialize==='function','STATE_TYPE');const raw=Buffer.from(state.serialize());check(raw.length===8150&&sha(raw)===INITIALIZED_SWAP.initializedStateSha256,'STATE_MISMATCH');
 const native=ledger.ContractState.deserialize(raw);check(Buffer.from(native.serialize()).equals(raw),'STATE_CANONICAL');check(isDeepStrictEqual(projectUnshieldedContractBalanceMap(native.balance),BALANCES),'STATE_BALANCES');
 const a=native.maintenanceAuthority;check(a.threshold===1&&a.counter===0n&&isDeepStrictEqual(a.committee,[AUTHORITY]),'STATE_AUTHORITY');return true;
}
/** Authenticated existing-state readback, after public and preserved-store checks. */
export async function verifyInitializedSwapPrivate({provider,signingKey,contractState,ledger}){
 assertInitializedSwapState(contractState,ledger);let key;try{key=ledger.signatureVerifyingKey(signingKey);}catch{throw Error('SWAP_CONTINUATION_SIGNING_AUTHORITY');}check(key===AUTHORITY,'SIGNING_AUTHORITY');
 let setAddress,get,getKey;try{setAddress=provider?.setContractAddress;get=provider?.get;getKey=provider?.getSigningKey;}catch{throw Error('SWAP_CONTINUATION_PRIVATE_PROVIDER');}check([setAddress,get,getKey].every(f=>typeof f==='function'),'PRIVATE_PROVIDER');
 let value;try{await setAddress.call(provider,INITIALIZED_SWAP.contractAddress);value=await get.call(provider,'sp05-swap');}catch{throw Error('SWAP_CONTINUATION_PRIVATE_READ');}
 check(value!==null&&typeof value==='object'&&Object.getPrototypeOf(value)===Object.prototype&&Reflect.ownKeys(value).length===0,'PRIVATE_STATE');
 let stored;try{stored=await getKey.call(provider,INITIALIZED_SWAP.contractAddress);}catch{throw Error('SWAP_CONTINUATION_PRIVATE_READ');}check(stored===signingKey,'PRIVATE_KEY');
 return Object.freeze({status:'PRIVATE_STATE_CHECKED',contractAddress:INITIALIZED_SWAP.contractAddress,txId:INITIALIZED_SWAP.txId,dispatchAuthorized:false});
}
/** Receipt must come from the current finalized observer and durable comparison.
 * Reuse the ordinary mint availability gate, but bind this exact retained mint.
 */
export function assertInitializedSwapMintedOutput({receipt,roles,assetBindings,synced}){
 check(roles?.firstAddress===INITIALIZED_SWAP.traderAddress&&roles?.secondAddress===INITIALIZED_SWAP.providerAddress&&assetBindings?.ASSET_A===MINTED_OUTPUT.type&&assetBindings?.ASSET_B==='e7d969726b0884ada7bb477283143911dc4df1b18da25a776a433bfaac8646d4','WALLET_BINDING');
 check(receipt?.txId===INITIALIZED_SWAP.txId&&receipt.contractAddress===INITIALIZED_SWAP.contractAddress&&receipt.transaction?.transactionHash===INITIALIZED_SWAP.transactionHash,'WALLET_HISTORY');
 check(receipt.transaction.outputs?.length===1,'NATIVE_OUTPUT');const {offerIndex,...row}=receipt.transaction.outputs[0];check(isDeepStrictEqual({...row,outputNo:offerIndex},MINTED_OUTPUT),'NATIVE_OUTPUT');
 return assertSwapInitializedWallet({receipt,roles,assetBindings,synced});
}
