import test from 'node:test';
import assert from 'node:assert/strict';
import * as m from '../src/language/outcome.ts';

const A={domain:'local',issuer:'demo',reference:'A',kind:'token'};
const B={domain:'local',issuer:'demo',reference:'B',kind:'token'};
const claims=['ContractInvariant','IntentRefinement','TransitionValidity','HistoryCompliance'];
function intent(){return {version:'moriarty-intent/1',domain:{network:'local',deployment:'r2b'},principal:'trader',nonce:'7',validity:{notBefore:'10',expiresAt:'20'},lifecycle:'atomic',authority:[{asset:{...A},maxDebit:'10',recipients:['pool','fee']}],goals:[{asset:{...B},account:'trader',minCredit:'3'}],fees:[{asset:{...A},maxFee:'1'}],agreements:[{id:'pool-one',programHash:'11'.repeat(32)}],requiredClaims:[...claims],extensions:[]};}
function trace(effects, traderB='3'){return {before:[{asset:A,account:'trader',amount:'20'},{asset:A,account:'pool',amount:'0'},{asset:A,account:'fee',amount:'0'},{asset:B,account:'trader',amount:'0'},{asset:B,account:'pool',amount:'20'}],effects,after:[{asset:A,account:'trader',amount:'9'},{asset:A,account:'pool',amount:'10'},{asset:A,account:'fee',amount:'1'},{asset:B,account:'trader',amount:traderB},{asset:B,account:'pool',amount:String(20n-BigInt(traderB))}]};}

test('exports the outcome intent API and rejects noncanonical semantic shapes',async()=>{
 assert.equal(m.readIntent(intent()).nonce,'7');
 for(const mutate of [x=>x.extensions.push('future'),x=>x.requiredClaims[1]='IntentEffects',x=>x.extra='x',x=>x.authority[0].recipients.push('pool'),x=>x.authority.push(structuredClone(x.authority[0])),x=>x.goals.push(structuredClone(x.goals[0])),x=>x.fees.push(structuredClone(x.fees[0])),x=>x.agreements.push(structuredClone(x.agreements[0])),x=>x.authority[0].asset.kind='coin',x=>x.nonce='07']){const bad=intent();mutate(bad);assert.throws(()=>m.readIntent(bad));}
 const tickerAlias=intent();tickerAlias.goals[0].asset={...B,domain:'other'};assert.notEqual(m.assetKey(B),m.assetKey(tickerAlias.goals[0].asset));
 const huge=intent();huge.authority=Array.from({length:9},(_,i)=>({asset:{...A,reference:`A${i}`},maxDebit:'1',recipients:['pool']}));assert.throws(()=>m.readIntent(huge));
 let calls=0;const bad=intent();Object.defineProperty(bad,'principal',{enumerable:true,get(){calls++;return 'trader';}});assert.throws(()=>m.readIntent(bad));assert.equal(calls,0);
});

test('signature trusts caller domain, principal and key and expires exclusively',async()=>{
 const keys=await crypto.subtle.generateKey({name:'Ed25519'},false,['sign','verify']);const signed=await m.signIntent(intent(),keys);
 const trust={domain:{network:'local',deployment:'r2b'},principal:'trader',publicKey:signed.publicKey};
 assert.deepEqual(await m.verifyIntent(signed,trust,'10'),{outcome:'checked'});
 assert.equal((await m.verifyIntent(signed,trust,'20')).outcome,'rejected');
 assert.equal((await m.verifyIntent(signed,{...trust,principal:'mallory'},'10')).outcome,'rejected');
 assert.equal((await m.verifyIntent(signed,{...trust,domain:{...trust.domain,deployment:'other'}},'10')).outcome,'rejected');
 const other=await m.signIntent(intent(),await crypto.subtle.generateKey({name:'Ed25519'},false,['sign','verify']));assert.equal((await m.verifyIntent(signed,{...trust,publicKey:other.publicKey},'10')).outcome,'rejected');
 for(const mutate of [x=>x.intent.requiredClaims[0]='Changed',x=>x.intentHash='00'.repeat(32),x=>x.signature='00'.repeat(64),x=>x.kind='Other',x=>x.extra='x']){const bad=structuredClone(signed);mutate(bad);assert.equal((await m.verifyIntent(bad,trust,'10')).outcome,'rejected');}
});

test('summary explains every signed meaning field and changes with dynamic values',()=>{
 const summary=m.renderIntentSummary(intent());for(const value of ['Outcome intent moriarty-intent/1','Domain: local / r2b','Principal: trader','Nonce: 7 (one-shot)','Valid: 10 inclusive, 20 exclusive','Lifecycle: atomic; unused authority expires on commit','Asset: domain=local, issuer=demo, reference=A, kind=token','Aggregate gross debit cap: 10','Recipients: pool, fee','Net credit goal: account=trader, minimum=3','Fee cap: 1 (included in gross debit budget)','Agreement: id=pool-one','Program hash: '+('11'.repeat(32)),'Mandatory claims: '+claims.join(', '),'Unsupported semantic extensions: none'])assert.ok(summary.includes(value),value);
 const changed=intent();changed.principal='alice';changed.authority[0].maxDebit='9';const changedSummary=m.renderIntentSummary(changed);assert.ok(changedSummary.includes('Principal: alice'));assert.ok(changedSummary.includes('Aggregate gross debit cap: 9'));assert.notEqual(changedSummary,summary);
 const bad=intent();bad.extensions=['opaque'];assert.throws(()=>m.renderIntentSummary(bad));
});

test('the first profile rejects duplicate agreement identifiers even with different programs',()=>{
 const bad=intent();bad.agreements.push({id:'pool-one',programHash:'22'.repeat(32)});assert.throws(()=>m.readIntent(bad));
});

test('trace checker enforces aggregate gross debit despite refunds and across recipients',()=>{
 const effects=[{kind:'Transfer',asset:A,from:'trader',to:'pool',amount:'6'},{kind:'Transfer',asset:A,from:'trader',to:'fee',amount:'5'},{kind:'Transfer',asset:A,from:'pool',to:'trader',amount:'1'},{kind:'Transfer',asset:B,from:'pool',to:'trader',amount:'3'}];
 const t=trace(effects);t.after[0].amount='10';t.after[1].amount='5';t.after[2].amount='5';const result=m.checkOutcomeTrace(intent(),t.before,t.effects,t.after);assert.equal(result.outcome,'rejected');assert.equal(result.code,'GrossDebitExceeded');
});

test('trace checker uses net credits including fees and checks prefix/completeness',()=>{
 const i=intent();i.fees=[{asset:B,maxFee:'1'}];i.authority.push({asset:B,maxDebit:'1',recipients:['fee']});
 const effects=[{kind:'Transfer',asset:A,from:'trader',to:'pool',amount:'10'},{kind:'Transfer',asset:B,from:'pool',to:'trader',amount:'3'},{kind:'Fee',asset:B,from:'trader',to:'fee',amount:'1'}];
 const before=[{asset:A,account:'trader',amount:'20'},{asset:A,account:'pool',amount:'0'},{asset:B,account:'pool',amount:'20'},{asset:B,account:'trader',amount:'0'},{asset:B,account:'fee',amount:'0'}];
 const after=[{asset:A,account:'trader',amount:'10'},{asset:A,account:'pool',amount:'10'},{asset:B,account:'pool',amount:'17'},{asset:B,account:'trader',amount:'2'},{asset:B,account:'fee',amount:'1'}];
 assert.equal(m.checkOutcomeTrace(i,before,effects,after).outcome,'rejected');
 const prefix=[{kind:'Transfer',asset:B,from:'trader',to:'fee',amount:'1'},{kind:'Transfer',asset:B,from:'pool',to:'trader',amount:'4'}];assert.equal(m.checkOutcomeTrace(i,before,prefix,after).outcome,'rejected');
});

test('trace checker accepts complete independently-accounted positive trace',()=>{
 const effects=[{kind:'Transfer',asset:A,from:'trader',to:'pool',amount:'10'},{kind:'Fee',asset:A,from:'trader',to:'fee',amount:'1'},{kind:'Transfer',asset:B,from:'pool',to:'trader',amount:'3'}];
 const t=trace(effects);const i=intent();i.authority[0].maxDebit='11';const result=m.checkOutcomeTrace(i,t.before,effects,t.after);assert.equal(result.outcome,'checked',result.message);assert.deepEqual(result.grossDebits,[{asset:A,amount:'11'}]);assert.deepEqual(result.netCredits,[{asset:B,account:'trader',amount:'3'}]);
});

test('trace accounting uses the full structured asset identity',()=>{
 const effects=[{kind:'Transfer',asset:{...A,domain:'other'},from:'trader',to:'pool',amount:'10'},{kind:'Transfer',asset:B,from:'pool',to:'trader',amount:'3'}];
 const t=trace(effects);assert.equal(m.checkOutcomeTrace(intent(),t.before,effects,t.after).outcome,'rejected');
});

test('a missing goal balance cell is an explicit incomplete-accounting rejection',()=>{
 const i=intent();const t=trace([],'0');t.before=t.before.filter(x=>!(x.account==='trader'&&x.asset.reference==='B'));t.after=t.after.filter(x=>!(x.account==='trader'&&x.asset.reference==='B'));
 const result=m.checkOutcomeTrace(i,t.before,[],t.after);assert.equal(result.outcome,'rejected');assert.equal(result.code,'IncompleteAccounting');
});
