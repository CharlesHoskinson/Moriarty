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
const counters=()=>({submission:10n,grossSpend:1000n,reservedSubmission:0n,reservedGross:0n,dustFee:1000n,reservedDustFee:0n});
async function fund(f, recipe={type:'UNPROVEN_TRANSACTION',transaction:f.tx}, c=counters(), final, allowance={submission:1n}){
 const calls=[],events=[];
 const p=await createFinancialProviders({walletContext:{unshieldedKeystore:{signData:d=>l.signData(f.sk,d)},facade:{balanceUnboundTransaction:async()=>{calls.push('balance');return recipe;},signRecipe:async()=>{calls.push('sign');return recipe;},finalizeRecipe:async()=>{calls.push('inert-finalize');if(!final)throw Error('LOCAL_FINALIZE_BOUNDARY');return final;},submitTransaction:async()=>{calls.push('inert-submit');return 'LOCAL_SYNTHETIC_ID';}}},networkConfig:{},privateStateLocation:'/unused',provenAssetManifest:{},resourceCounters:c,onSubmission:()=>{},eventSink:e=>events.push(e),adapters:{CompiledContract:{},NodeZkConfigProvider:{},httpClientProofProvider:{},indexerPublicDataProvider:{},levelPrivateStateProvider:{}}});
 let error,result;try{result=await p.fundUnshielded({tx:f.tx,payer:{vk:f.vk},ttl,allowance,tokenType:f.token});}catch(e){error=e.message;}return {calls,error,counters:{...c},events,result:result?.submittedId};
}
await record('native no-dust public path unchanged bind inert submit',async()=>{const f=fixture();return await fund(f,undefined,undefined,f.tx.bind());});
await record('persistent scalar counters accept different native token units',async()=>{const a=fixture(),b=fixture({amount:200n}),c=counters();return {distinctTokenTypes:a.token!==b.token,first:await fund(a,undefined,c),second:await fund(b,undefined,c)};});
await record('plain diagnostic Maps authorize public funding and inert submit',async()=>{const f=fixture();const shaped={intents:f.tx.intents};return await fund(f,{type:'UNPROVEN_TRANSACTION',transaction:shaped},undefined,shaped);});
await record('unknown recipe type admitted public native funding',async()=>{const f=fixture();return await fund(f,{type:'UNKNOWN_TYPE',baseTransaction:f.tx},undefined,f.tx.bind());});
await record('malformed budget side effects precede validation',async()=>{const f=fixture(),c=counters();c.reservedGross=-1n;return await fund(f,undefined,c);});
function addDust(f,spends=[],registrations=[]){const ints=f.tx.intents,i=ints.get(1);i.dustActions=new l.DustActions('signature','pre-proof',new Date('2026-09-08T00:00:00Z'),spends,registrations);i.guaranteedUnshieldedOffer=i.guaranteedUnshieldedOffer.addSignatures([l.signData(f.sk,i.signatureData(1))]);ints.set(1,i);f.tx.intents=ints;return f;}
await record('native registration invalid signature accepted with cap',async()=>{const f=fixture();addDust(f,[],[new l.DustRegistration('signature',f.vk,undefined,900n,l.signData(f.sk,new Uint8Array([1])))]);return {signatureGetter:f.tx.intents.get(1).dustActions.registrations[0].signature??null,...await fund(f,undefined,undefined,f.tx.bind())};});
await record('native registration exceeds remaining fee authority',async()=>{const f=fixture();addDust(f,[],[new l.DustRegistration('signature',f.vk,undefined,900n,l.signData(f.sk,new Uint8Array([1])))]);const c=counters();c.reservedDustFee=999n;return await fund(f,undefined,c);});
let nativeDust;
await record('construct native DustSpend synthetic ledger state',()=>{
 const sk=l.DustSecretKey.fromBigint(1n),ctime=new Date('2026-09-08T00:00:00Z'),now=new Date('2026-09-08T01:00:00Z'),nonce=l.sampleIntentHash();
 const gen={value:1000000n,owner:sk.publicKey,nonce,dtime:undefined};
 const qdo={initialValue:1000000n,owner:sk.publicKey,nonce:l.dustNonce(nonce,0n,sk),seq:0,ctime,backingNight:nonce,mtIndex:0n};
 let state=new l.DustLocalState(new l.DustParameters(1000n,1n,3600n));state=state.insertGenerationInfo(0n,gen,nonce).insertCommitment(0n,qdo,true).addUtxo(l.dustNullifier(qdo,sk),qdo);
 const [next,spend]=state.spend(sk,qdo,7n,now);nativeDust=spend;return {nativeInstance:spend instanceof l.DustSpend,vFee:spend.vFee,serializedBytes:spend.serialize().length,scope:'Synthetic local Merkle state only, no ledger validation or real proof'};
});
if(nativeDust){
 await record('native DustSpend public path fee reservation and bind comparison',async()=>{const f=addDust(fixture(),[nativeDust]);return {inspection:inspectSignedRecipeGross({transaction:f.tx},{tokenType:f.token,ledger:l}),...await fund(f,undefined,undefined,f.tx.bind())};});
 await record('native DustSpend inadequate fee cap rejects atomically',async()=>{const f=addDust(fixture(),[nativeDust]),c=counters();c.dustFee=6n;return await fund(f,undefined,c);});
 await record('native DustSpend changed material effect across bind rejects',()=>{const f=addDust(fixture(),[nativeDust]),a=clone(f.tx),ints=a.intents,i=ints.get(1);i.dustActions=new l.DustActions('signature','pre-proof',new Date('2026-09-08T00:00:01Z'),[nativeDust],[]);ints.set(1,i);a.intents=ints;return result(a.bind(),{transaction:f.tx});});
}
writeFileSync(new URL('./gpt6-independent-results.json',import.meta.url),JSON.stringify({scope:'Independent GPT-6 Astra review. Pure native objects, synthetic signatures/UTXO/Merkle state, bind/merge/erase only. All finalize/submit calls are inert adapters.',rows},(_,v)=>typeof v==='bigint'?v.toString():v,2)+'\n');console.log(JSON.stringify(rows,(_,v)=>typeof v==='bigint'?v.toString():v,2));
