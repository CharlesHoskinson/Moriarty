import assert from 'node:assert/strict';
import {writeFileSync} from 'node:fs';
import {importPinned,recheckFinalizedAgainstRecipe as compare,validateSignedRecipe,inspectSignedRecipeGross,createFinancialProviders} from './candidate/experiments/moriarty-midnight-financial/ledger/providers.mjs';
const l=await importPinned('@midnight-ntwrk/ledger-v8');
const rows=[]; const ttl=new Date('2030-01-01T00:00:00Z');
const result=(a,r)=>{try{return {accepted:compare(a,r).ok};}catch(e){return {accepted:false,rejection:e.message};}};
async function record(name,fn){try{rows.push({name,...await fn()});}catch(e){rows.push({name,constructionError:e.message});}}
const clone=t=>l.Transaction.deserialize('signature','pre-proof','pre-binding',t.serialize());
function fixture({segment=1,amount=100n,token=l.sampleRawTokenType(),sk=l.sampleSigningKey(),action}={}){
 const vk=l.signatureVerifyingKey(sk),owner=l.addressFromKey(vk);let i=l.Intent.new(ttl);
 if(action)i.actions=[action];
 i.guaranteedUnshieldedOffer=l.UnshieldedOffer.new([{owner:vk,type:token,value:amount,intentHash:l.sampleIntentHash(),outputNo:0}],[{owner,type:token,value:amount-1n}],[]);
 let tx=l.Transaction.fromParts('undeployed').addIntent({tag:'specific',value:segment},i);let ints=tx.intents;i=ints.get(segment);i.guaranteedUnshieldedOffer=i.guaranteedUnshieldedOffer.addSignatures([l.signData(sk,i.signatureData(segment))]);ints.set(segment,i);tx.intents=ints;
 assert.equal(validateSignedRecipe({baseTransaction:tx},{vk},{ledger:l,tokenType:token}).ok,true);return {tx,sk,vk,token};
}
for(const change of ['unchanged','counter','authority'])await record('native maintenance '+change+' across bind',()=>{
 const address=l.sampleContractAddress();const authority=new l.ContractMaintenanceAuthority([l.signatureVerifyingKey(l.sampleSigningKey())],1);
 const action=new l.MaintenanceUpdate(address,[new l.ReplaceAuthority(authority)],1n);
 const {tx}=fixture({action});const c=clone(tx);
 if(change!=='unchanged'){const ints=c.intents,i=ints.get(1);i.actions=[new l.MaintenanceUpdate(address,[new l.ReplaceAuthority(change==='authority'?new l.ContractMaintenanceAuthority([],0):authority)],change==='counter'?2n:1n)];ints.set(1,i);c.intents=ints;}
 const bound=c.bind();return {serializedBeforeBindingChanged:!Buffer.from(c.serialize()).equals(Buffer.from(tx.serialize())),signatureDataEqual:Buffer.from(c.intents.get(1).signatureData(1)).equals(Buffer.from(tx.intents.get(1).signatureData(1))),eraseProofsBytesEqual:Buffer.from(tx.eraseProofs().serialize()).equals(Buffer.from(bound.eraseProofs().serialize())),samePhase:result(c,{baseTransaction:tx}),...result(bound,{baseTransaction:tx})};
});
function transcript(){return {gas:{readTime:0n,computeTime:0n,bytesWritten:0n,bytesDeleted:0n},effects:{claimedNullifiers:[],claimedShieldedReceives:[],claimedShieldedSpends:[],claimedContractCalls:[],shieldedMints:new Map(),unshieldedMints:new Map(),unshieldedInputs:new Map(),unshieldedOutputs:new Map(),claimedUnshieldedSpends:new Map()},program:[]};}
for(const change of ['unchanged','mint','gas'])await record('native contract transcript '+change+' across bind',()=>{
 const address=l.sampleContractAddress(),rand=l.communicationCommitmentRandomness(),op=new l.ContractOperation(),empty={value:[],alignment:[]};
 function call(t){return new l.ContractCallPrototype(address,'entry',op,t,undefined,[],empty,empty,rand,'inert-key');}
 let i=l.Intent.new(ttl).addCall(call(transcript()));const action=i.actions[0];const {tx}=fixture({action});const c=clone(tx);
 if(change!=='unchanged'){const t=transcript();if(change==='mint')t.effects.unshieldedMints.set('01'.repeat(32),7n);else t.gas.computeTime=1n;const ci=l.Intent.new(ttl).addCall(call(t));const ints=c.intents,ii=ints.get(1);ii.actions=ci.actions;ints.set(1,ii);c.intents=ints;}
 return {serializedBeforeBindingChanged:!Buffer.from(c.serialize()).equals(Buffer.from(tx.serialize())),eraseProofsBytesEqual:Buffer.from(tx.eraseProofs().serialize()).equals(Buffer.from(c.bind().eraseProofs().serialize())),samePhase:result(c,{baseTransaction:tx}),...result(c.bind(),{baseTransaction:tx})};
});
async function funding(recipe,f,{allowance={submission:1n},cap=1000n,reservedGross=0n,submissionCap=2n}={}){
 const counters={submission:submissionCap,grossSpend:cap,reservedSubmission:0n,reservedGross};const calls=[];if(cap===null)delete counters.grossSpend;
 const p=await createFinancialProviders({walletContext:{unshieldedKeystore:{signData:d=>l.signData(f.sk,d)},facade:{balanceUnboundTransaction:async()=>recipe,signRecipe:async()=>recipe,finalizeRecipe:async()=>{calls.push('inert-finalize');throw Error('LOCAL_FINALIZE_BOUNDARY');},submitTransaction:async()=>{throw Error('UNREACHABLE');}}},networkConfig:{},privateStateLocation:'/unused',provenAssetManifest:{},resourceCounters:counters,onSubmission:()=>{},eventSink:()=>{},adapters:{CompiledContract:{},NodeZkConfigProvider:{},httpClientProofProvider:{},indexerPublicDataProvider:{},levelPrivateStateProvider:{}}});let error;try{await p.fundUnshielded({tx:f.tx,payer:{vk:f.vk},ttl,allowance,tokenType:f.token});}catch(e){error=e.message;}return {calls,error,counters};
}
for(const [name,args] of [['positive',{}],['zero-submission',{allowance:{submission:0n}}],['negative-existing-reservation',{reservedGross:-1000n,cap:50n}],['missing-gross-counter',{cap:null}],['boolean-declaration',{allowance:{submission:true,grossSpend:true}}]])await record('funding '+name,async()=>{const f=fixture();return await funding({baseTransaction:f.tx},f,args);});
await record('native dust registration fee allowance omitted from gross',async()=>{
 const f=fixture();const ints=f.tx.intents,i=ints.get(1);
 i.dustActions=new l.DustActions('signature','pre-proof',new Date('2026-09-08T00:00:00Z'),[],[new l.DustRegistration('signature',f.vk,undefined,900n,l.signData(f.sk,new Uint8Array([1])))]);
 i.guaranteedUnshieldedOffer=i.guaranteedUnshieldedOffer.addSignatures([l.signData(f.sk,i.signatureData(1))]);ints.set(1,i);f.tx.intents=ints;
 const recipe={baseTransaction:f.tx};validateSignedRecipe(recipe,{vk:f.vk},{ledger:l,tokenType:f.token});return {gross:inspectSignedRecipeGross(recipe,{tokenType:f.token}),...await funding(recipe,f,{cap:100n})};
});
await record('all parts guaranteed offer actual gross',()=>{const f=fixture(),b=fixture({segment:2,sk:f.sk,token:f.token,amount:200n});return inspectSignedRecipeGross({baseTransaction:f.tx,balancingTransaction:b.tx},{tokenType:f.token});});
await record('native eraseProofs unchanged disjoint merge bind normalization',()=>{const a=fixture(),b=fixture({segment:2,sk:a.sk,token:a.token});const merged=a.tx.merge(b.tx);return {eraseProofsBytesEqual:Buffer.from(merged.eraseProofs().serialize()).equals(Buffer.from(merged.bind().eraseProofs().serialize()))};});
writeFileSync(new URL('./gpt6-independent-results.json',import.meta.url),JSON.stringify({scope:'Native synthetic objects and signatures, native bind only; inert finalization boundary; no proof/wallet/facade native finalization/network/submission.',rows},(_,v)=>typeof v==='bigint'?v.toString():v,2)+'\n');console.log(JSON.stringify(rows,(_,v)=>typeof v==='bigint'?v.toString():v,2));
