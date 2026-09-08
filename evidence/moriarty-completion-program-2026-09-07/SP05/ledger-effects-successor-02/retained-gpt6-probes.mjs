import assert from 'node:assert/strict';
import {writeFileSync} from 'node:fs';
import {importPinned,recheckFinalizedAgainstRecipe as compare,validateSignedRecipe} from './candidate/experiments/moriarty-midnight-financial/ledger/providers.mjs';
const ledger = await importPinned('@midnight-ntwrk/ledger-v8');
const rows=[];
function record(name,fn){try{rows.push({name,...fn()});}catch(e){rows.push({name,error:e.message});}}
const clone=tx=>ledger.Transaction.deserialize('signature','pre-proof','pre-binding',tx.serialize());
function fixture(segment=1){
 const sk=ledger.sampleSigningKey(), vk=ledger.signatureVerifyingKey(sk),owner=ledger.addressFromKey(vk),token=ledger.sampleRawTokenType(),ttl=new Date('2030-01-01T00:00:00Z');
 let intent=ledger.Intent.new(ttl);
 for(const kind of ['guaranteed','fallible'])intent[kind+'UnshieldedOffer']=ledger.UnshieldedOffer.new([{value:100n,owner:vk,type:token,intentHash:ledger.sampleIntentHash(),outputNo:0}],[{value:90n,owner,type:token}],[]);
 let tx=ledger.Transaction.fromParts('undeployed').addIntent({tag:'specific',value:segment},intent);
 const intents=tx.intents; intent=intents.get(segment);
 for(const kind of ['guaranteed','fallible'])intent[kind+'UnshieldedOffer']=intent[kind+'UnshieldedOffer'].addSignatures([ledger.signData(sk,intent.signatureData(segment))]);
 intents.set(segment,intent);tx.intents=intents;
 assert.equal(validateSignedRecipe({baseTransaction:tx},{vk},{ledger,tokenType:token}).ok,true);
 return {tx,vk,token,ttl};
}
function outcome(tx,recipe){try{return {accepted:compare(tx,recipe).ok};}catch(e){return {accepted:false,rejection:e.message};}}
record('unchanged native transaction',()=>{const {tx}=fixture();return outcome(clone(tx),{type:'UNPROVEN_TRANSACTION',transaction:tx});});
for(const kind of ['guaranteed','fallible'])for(const field of ['input.intentHash','input.outputNo','input.type','input.value','input.owner','output.type','output.value','output.owner','signature','omit-input','omit-output'])record(kind+' '+field,()=>{
 const {tx}=fixture(),copy=clone(tx),intents=copy.intents,i=intents.get(1),o=i[kind+'UnshieldedOffer'];let inputs=o.inputs,outputs=o.outputs,sigs=o.signatures;
 const [type,key]=field.split('.');if(type==='input'||type==='output'){const row=type==='input'?inputs[0]:outputs[0];row[key]=key==='intentHash'?ledger.sampleIntentHash():key==='outputNo'?row[key]+1:key==='type'?ledger.sampleRawTokenType():key==='value'?row[key]+1n:type==='input'?ledger.signatureVerifyingKey(ledger.sampleSigningKey()):ledger.sampleUserAddress();}
 if(field==='signature')sigs=[ledger.signData(ledger.sampleSigningKey(),i.signatureData(1))];if(field==='omit-input')inputs=[];if(field==='omit-output')outputs=[];
 i[kind+'UnshieldedOffer']=ledger.UnshieldedOffer.new(inputs,outputs,sigs);intents.set(1,i);copy.intents=intents;
 return outcome(copy,{type:'UNPROVEN_TRANSACTION',transaction:tx});
});
record('ttl mutation',()=>{const {tx}=fixture(),copy=clone(tx),ints=copy.intents,i=ints.get(1);i.ttl=new Date('2031-01-01T00:00:00Z');ints.set(1,i);copy.intents=ints;return outcome(copy,{transaction:tx});});
for(const type of ['UNBOUND_TRANSACTION','FINALIZED_TRANSACTION'])record(type+' native disjoint merge control',()=>{const a=fixture(1).tx,b=fixture(2).tx;const recipe={type,balancingTransaction:b,[type==='UNBOUND_TRANSACTION'?'baseTransaction':'originalTransaction']:a};return outcome(a.merge(b),recipe);});
record('native colliding merge',()=>{const a=fixture().tx,b=fixture().tx;let sdkRejects=false;try{a.merge(b);}catch{sdkRejects=true;}return {sdkRejects,...outcome(a,{type:'UNBOUND_TRANSACTION',baseTransaction:a,balancingTransaction:b})};});
record('changed network transaction commitment',()=>{const {tx}=fixture();const copy=ledger.Transaction.fromParts('preview');copy.intents=tx.intents;return {serializationChanged:!Buffer.from(tx.serialize()).equals(Buffer.from(copy.serialize())),signatureDataEqual:Buffer.from(tx.intents.get(1).signatureData(1)).equals(Buffer.from(copy.intents.get(1).signatureData(1))),...outcome(copy,{transaction:tx})};});
for(const kind of ['guaranteed','fallible'])record('added '+kind+' native Zswap output',()=>{
 const {tx,token}=fixture();const copy=clone(tx);
 const output=ledger.ZswapOutput.newContractOwned(ledger.createShieldedCoinInfo(token,5n),kind==='guaranteed'?undefined:1,ledger.sampleContractAddress());
 const offer=ledger.ZswapOffer.fromOutput(output);
 if(kind==='guaranteed')copy.guaranteedOffer=offer;else copy.fallibleOffer=new Map([[1,offer]]);
 return {serializationChanged:!Buffer.from(tx.serialize()).equals(Buffer.from(copy.serialize())),signatureDataEqual:Buffer.from(tx.intents.get(1).signatureData(1)).equals(Buffer.from(copy.intents.get(1).signatureData(1))),...outcome(copy,{transaction:tx})};
});
for(const kind of ['guaranteed','fallible'])for(const change of ['unchanged','remove','amount','recipient'])record(kind+' Zswap '+change,()=>{
 const {tx,token}=fixture();const contract=ledger.sampleContractAddress();
 const makeOffer=(value,target)=>ledger.ZswapOffer.fromOutput(ledger.ZswapOutput.newContractOwned(ledger.createShieldedCoinInfo(token,value),kind==='guaranteed'?undefined:1,target));
 const set=(target,offer)=>{if(kind==='guaranteed')target.guaranteedOffer=offer;else target.fallibleOffer=offer?new Map([[1,offer]]):undefined;};
 set(tx,makeOffer(5n,contract));const copy=clone(tx);
 if(change==='remove')set(copy,undefined);
 if(change==='amount')set(copy,makeOffer(6n,contract));
 if(change==='recipient')set(copy,makeOffer(5n,ledger.sampleContractAddress()));
 const oldOffer=kind==='guaranteed'?tx.guaranteedOffer:tx.fallibleOffer.get(1),newOffer=kind==='guaranteed'?copy.guaranteedOffer:copy.fallibleOffer?.get(1);
 return {expectedZswapOutputs:oldOffer.outputs.length,actualZswapOutputs:newOffer?.outputs.length??0,expectedDelta:String(oldOffer.deltas.get(token)),actualDelta:String(newOffer?.deltas.get(token)??0n),serializationChanged:!Buffer.from(tx.serialize()).equals(Buffer.from(copy.serialize())),signatureDataEqual:Buffer.from(tx.intents.get(1).signatureData(1)).equals(Buffer.from(copy.intents.get(1).signatureData(1))),...outcome(copy,{transaction:tx})};
});
writeFileSync(new URL('./retained-gpt6-results.json',import.meta.url),JSON.stringify({scope:'Offline native pre-proof/pre-binding synthetic UTXOs; no proof, binding, wallet, finalization, network or submission',rows},null,2)+'\n');console.log(JSON.stringify(rows,null,2));
