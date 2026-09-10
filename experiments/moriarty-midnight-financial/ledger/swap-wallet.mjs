/** Fixed swap initialize wallet gate. Caller must supply the actual
 * observeFinalizedStage receipt AFTER a durable PASS initialize comparison,
 * then await wallet.waitForSyncedState under the existing absolute deadline.
 * This predicate does not establish receipt provenance, finality or proofs.
 * It performs no wallet/store/network operations and grants no dispatch authority.
 */
import {decodeLocalIndexedOwner} from './indexed-owner.mjs';
export const SWAP_WALLET_FAILURE_CODES=Object.freeze(['ROLES','ASSETS','RECEIPT','HISTORY','NATIVE_IDENTITY','NATIVE_ACTION','NATIVE_MINT','NATIVE_OUTPUT','SYNC','PENDING','COIN_SHAPE','AVAILABLE','OWNER','OUTPUT'].map(x=>'SWAP_WALLET_'+x));
const check=(ok,label)=>{if(!ok)throw Error('SWAP_WALLET_'+label);};
const hex=x=>typeof x==='string'&&/^[a-f0-9]{64}$/.test(x);
const height=x=>Number.isSafeInteger(x)&&x>=0;
export function assertSwapInitializedWallet({receipt,roles,assetBindings,synced}){
 check(hex(roles?.firstAddress)&&hex(roles?.secondAddress)&&roles.firstAddress!==roles.secondAddress,'ROLES');
 check(hex(assetBindings?.ASSET_A)&&hex(assetBindings?.ASSET_B)&&assetBindings.ASSET_A!==assetBindings.ASSET_B,'ASSETS');
 check(receipt?.schema==='moriarty.finalized-financial-stage/1'&&receipt.circuitId==='initialize'&&receipt.acceptance==='uncertified-I2-observation'&&receipt.protocolVersion===1000000,'RECEIPT');
 check(hex(receipt.contractAddress)&&![roles.firstAddress,roles.secondAddress].includes(receipt.contractAddress)&&hex(receipt.blockHash)&&typeof receipt.finalizedHead==='string'&&/^0x[a-f0-9]{64}$/.test(receipt.finalizedHead)&&height(receipt.blockHeight)&&height(receipt.finalizedHeight)&&receipt.finalizedHeight>=receipt.blockHeight,'HISTORY');
 const tx=receipt.transaction;
 check(tx&&hex(tx.transactionHash)&&tx.rawSha256===tx.transactionHash&&Array.isArray(tx.identifiers)&&typeof receipt.txId==='string'&&/^[a-f0-9]{66}$/.test(receipt.txId)&&tx.identifiers.includes(receipt.txId)&&tx.proofVerified===false&&tx.ledgerAccepted===false,'NATIVE_IDENTITY');
 check(Array.isArray(tx.actions)&&tx.actions.length===1&&tx.actions[0]?.kind==='call'&&tx.actions[0].entryPoint==='initialize'&&tx.actions[0].address===receipt.contractAddress&&Number.isInteger(tx.actions[0].segment)&&tx.actions[0].segment>=0&&tx.actions[0].segment<=65535,'NATIVE_ACTION');
 check(Array.isArray(tx.inputs)&&tx.inputs.length===0&&Array.isArray(tx.outputs)&&tx.outputs.length===1,'NATIVE_MINT');
 const o=tx.outputs[0];
 check(o&&Object.keys(o).sort().join(',')==='intentHash,offerIndex,owner,section,segment,type,value'&&o.segment===tx.actions[0].segment&&o.section==='guaranteed'&&o.offerIndex===0&&o.owner===roles.firstAddress&&o.type===assetBindings.ASSET_A&&o.value==='100000'&&hex(o.intentHash),'NATIVE_OUTPUT');
 for(const kind of ['shielded','unshielded','dust']){const p=synced?.[kind]?.progress;check(p?.isConnected===true&&typeof p.isStrictlyComplete==='function'&&p.isStrictlyComplete()===true,'SYNC');}
 const u=synced.unshielded;check(Array.isArray(u.availableCoins)&&Array.isArray(u.pendingCoins)&&u.pendingCoins.length===0&&Array.isArray(synced.dust.state?.pendingDust)&&synced.dust.state.pendingDust.length===0,'PENDING');
 const coins=u.availableCoins.map(c=>{check(c?.utxo&&hex(c.utxo.intentHash)&&height(c.utxo.outputNo),'COIN_SHAPE');return c.utxo;});
 const found=coins.filter(c=>c.intentHash===o.intentHash&&c.outputNo===o.offerIndex);check(found.length===1,'AVAILABLE');
 const coin=found[0];let owner;try{owner=decodeLocalIndexedOwner(coin.owner);}catch{throw Error('SWAP_WALLET_OWNER');}
 check(owner===o.owner&&coin.type===o.type&&typeof coin.value==='bigint'&&coin.value===100000n,'OUTPUT');
 const mintedOutput=Object.freeze({segment:o.segment,section:o.section,owner:o.owner,type:o.type,value:o.value,intentHash:o.intentHash,outputNo:o.offerIndex});
 return Object.freeze({status:'SWAP_INITIALIZED_WALLET_VERIFIED',mintedOutput,dispatchAuthorized:false});
}
