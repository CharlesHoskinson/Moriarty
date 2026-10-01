import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import * as runtime from '@midnight-ntwrk/compact-runtime';
import {Contract,pureCircuits,ledger} from './output/contract/index.js';
const p=JSON.parse(readFileSync(new URL('./projection.json',import.meta.url))),msg=Uint8Array.from(Buffer.from(p.framed.signing_message_hex,'hex'));
const sig={r:BigInt('0x'+p.artifact.signatureHex.slice(0,64)),s:BigInt('0x'+p.artifact.signatureHex.slice(64))},pk={x:BigInt(p.publicKey.x),y:BigInt(p.publicKey.y),identity:false},n=0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141n;
const results=[];
function positive(name,fn){try{fn();results.push({name,status:'PASS'});}catch(e){results.push({name,status:'FAIL',error:e.message});throw e;}}
function negative(name,fn){assert.throws(fn);results.push({name,status:'PASS'});}
positive('actual beta production prefixed frame signature/digest',()=>assert.equal(Buffer.from(pureCircuits.verifyOwner(msg,sig,pk)).toString('hex'),createHash('sha256').update(msg).digest('hex')));
negative('high S full256-bit canonical refusal',()=>pureCircuits.verifyOwner(msg,{r:sig.r,s:n-sig.s},pk));
negative('zero r',()=>pureCircuits.verifyOwner(msg,{r:0n,s:sig.s},pk));
negative('zero s',()=>pureCircuits.verifyOwner(msg,{r:sig.r,s:0n},pk));
negative('out of range s',()=>pureCircuits.verifyOwner(msg,{r:sig.r,s:n},pk));
negative('identity',()=>pureCircuits.verifyOwner(msg,sig,{x:0n,y:0n,identity:true}));
const changed=msg.slice();changed.at(-1);changed[changed.length-1]^=1;negative('changed exact source frame',()=>pureCircuits.verifyOwner(changed,sig,pk));
negative('opposite SEC1 parity',()=>pureCircuits.verifyOwner(msg,sig,{x:pk.x,y:0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2fn-pk.y,identity:false}));
const bytes=h=>Uint8Array.from(Buffer.from(h,'hex')),color=bytes('a1'.repeat(32)),recipient={bytes:bytes('02'.repeat(32))},fee={bytes:bytes('03'.repeat(32))},address='04'.repeat(32),coin='00'.repeat(32);
async function state(funding=10000n,allowance=10000n,work=10n,round=1n){const contract=new Contract({});const init=await contract.initialState(runtime.createConstructorContext({},coin),pk,color,recipient,fee,funding,allowance,work,round);init.currentContractState.balance=new Map([[{tag:'unshielded',raw:'a1'.repeat(32)},funding]]);return {contract,init};}
function context(init){return runtime.createCircuitContext({circuitId:'pay',contractAddress:address,coinPublicKeyOrZswapState:coin,contractState:init.currentContractState,privateState:{},time:1,parentBlockHash:'00'.repeat(32)});}
const {contract,init}=await state(),original=Buffer.from(init.currentContractState.serialize()).toString('hex');
const outcome=await contract.circuits.pay(context(init),msg,sig),after=ledger(outcome.context.callContext.currentQueryContext.state);
assert.equal(after.ownerBalance,8990n);assert.equal(after.recipientBalance,1000n);assert.equal(after.feeBalance,10n);assert.equal(after.allowanceRemaining,8990n);assert.equal(after.allowanceSpent,1010n);assert.equal(after.workRemaining,9n);assert.equal(after.workSpent,1n);assert.equal(after.usedNonce,true);assert.equal(after.revision,1n);assert.notEqual(Buffer.from(after.head).toString('hex'),p.beforeHead);
assert.equal(Buffer.from(init.currentContractState.serialize()).toString('hex'),original,'original native context snapshot mutated');
results.push({name:'complete Core5 numerical delta with single-use local state',status:'PASS'});
for(const [name,funding,allowance,work,round] of [['underfunded',1009n,10000n,10n,1n],['allowance',10000n,1009n,10n,1n],['work',10000n,10000n,0n,1n],['window',10000n,10000n,10n,11n]]){
 const s=await state(funding,allowance,work,round),before=Buffer.from(s.init.currentContractState.serialize()).toString('hex');await assert.rejects(s.contract.circuits.pay(context(s.init),msg,sig));assert.equal(Buffer.from(s.init.currentContractState.serialize()).toString('hex'),before);results.push({name:name+' refuses without source snapshot mutation',status:'PASS'});
}
await assert.rejects(contract.circuits.pay(runtime.createCircuitContext({circuitId:'pay',contractAddress:address,coinPublicKeyOrZswapState:coin,contractState:outcome.context.callContext.currentQueryContext.state,privateState:{},time:1}),msg,sig));results.push({name:'replay refuses current post-state',status:'PASS'});
const half=n/2n;
for(const value of [1n,(1n<<128n)-1n,1n<<128n,half])positive('full256 lowS boundary '+value,()=>pureCircuits.signatureIsCanonical({r:1n,s:value}));
for(const value of [half+1n,n-1n,n])negative('full256 highS boundary '+value,()=>pureCircuits.signatureIsCanonical({r:1n,s:value}));
for(const [name,c,r,f]of [['wrong color',bytes('b1'.repeat(32)),recipient,fee],['swapped destination',color,fee,recipient],['aliased destination',color,recipient,recipient]]){
 await assert.rejects(new Contract({}).initialState(runtime.createConstructorContext({},coin),pk,c,r,f,10000n,10000n,10n,1n));results.push({name,status:'PASS'});
}
const metadata=JSON.parse(readFileSync(new URL('./output/compiler/contract-info.json',import.meta.url)));
function faultedContractState(initial,name,value){
 const copy=runtime.ContractState.deserialize(initial.serialize()),field=metadata.ledger.find(x=>x.name===name),path=field.index;
 const type=new runtime.CompactTypeUnsignedInteger((1n<<128n)-1n,16),replacement=runtime.StateValue.newCell({alignment:type.alignment(),value:type.toValue(value)});
 function replace(state,indices){if(!indices.length)return replacement;const rows=state.asArray();assert.ok(rows);let next=runtime.StateValue.newArray();for(let i=0;i<rows.length;i++)next=next.arrayPush(i===indices[0]?replace(rows[i],indices.slice(1)):rows[i]);return next;}
 copy.data=new runtime.ChargedState(replace(copy.data.state,path));return copy;
}
for(const name of ['recipientBalance','feeBalance','allowanceSpent','workSpent']){
 const c=faultedContractState(init.currentContractState,name,(1n<<128n)-1n),before=Buffer.from(c.serialize()).toString('hex');await assert.rejects(contract.circuits.pay(context({currentContractState:c}),msg,sig));assert.equal(Buffer.from(c.serialize()).toString('hex'),before);results.push({name:name+' overflow retains original faulted snapshot',status:'PASS'});
}
const claims=outcome.context.callContext.currentQueryContext.effects;
const spendRows=[...claims.claimedUnshieldedSpends].map(([[asset,destination],amount])=>[asset.tag,asset.raw,destination.tag,destination.address,amount]).sort((a,b)=>a[3].localeCompare(b[3]));
assert.deepEqual(spendRows,[['unshielded','a1'.repeat(32),'user','02'.repeat(32),1000n],['unshielded','a1'.repeat(32),'user','03'.repeat(32),10n]]);
assert.deepEqual([...claims.unshieldedOutputs],[[{tag:'unshielded',raw:'a1'.repeat(32)},1010n]]);assert.equal(claims.unshieldedInputs.size,0);assert.equal(claims.unshieldedMints.size,0);assert.equal(claims.claimedShieldedSpends.length,0);assert.equal(claims.claimedShieldedReceives.length,0);results.push({name:'complete exact native escrow effect claims without extra input/mint/shielded effect',status:'PASS'});
const preparedRace=await Promise.all([contract.circuits.pay(context(init),msg,sig),contract.circuits.pay(context(init),msg,sig)]);
assert.equal(preparedRace.length,2);results.push({name:'same snapshot produces two runtime candidates: durable ledger race remains unverified',status:'OBSERVED_GAP'});
const effectFields=['unshieldedOutputs' ,'unshieldedInputs','claimedUnshieldedSpends','unshieldedMints'];
const contextKeys=Object.keys(outcome.context);const effects=outcome.context.callContext.currentQueryContext.effects;
const json=(v)=>JSON.parse(JSON.stringify(v,(_,x)=>typeof x==='bigint'?x.toString():x instanceof Map?[...x]:x));
writeFileSync(new URL('./runtime-result.json',import.meta.url),JSON.stringify({scope:'local ledger9 source/runtime only, no proof/ledger/Preview',results,post:json(after),contextKeys,effects:effects?Object.fromEntries(effectFields.map(k=>[k,json(effects[k])])):null},null,2));console.log(JSON.stringify({results,contextKeys,effectsFound:!!effects}));
