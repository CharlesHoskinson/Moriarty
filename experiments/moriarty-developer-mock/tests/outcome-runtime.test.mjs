import test from 'node:test';
import assert from 'node:assert/strict';
import { createOutcomeDemo } from '../dist/language/outcome-runtime.js';
import { signIntent } from '../src/language/outcome.ts';
import { generateLocalKey } from '../src/language/claims.ts';

async function prepared(kind='swap') {
  const {runtime,intent}=await createOutcomeDemo(kind);
  const keys=await generateLocalKey();const signed=await signIntent(intent,keys);
  const trust={domain:intent.domain,principal:intent.principal,publicKey:signed.publicKey};
  return {runtime,intent,keys,signed,trust,plan:await runtime.propose(signed.intentHash)};
}

test('one signed outcome permits two actual pool plans without a new signature',async()=>{
 const x=await prepared();const other=await x.runtime.propose(x.signed.intentHash,1);
 const a=await x.runtime.preview(x.signed,x.plan,x.trust),b=await x.runtime.preview(x.signed,other,x.trust);
 assert.equal(a.outcome,'checked',a.message);assert.equal(b.outcome,'checked',b.message);
 assert.notEqual(a.receipt.planHash,b.receipt.planHash);assert.equal(a.receipt.intentHash,b.receipt.intentHash);
 assert.notEqual(a.receipt.netCredits[0].amount,b.receipt.netCredits[0].amount);
 assert.equal(x.runtime.snapshot().consumed,0);
 assert.equal((await x.runtime.verifyRealAcceptance(x.signed,x.plan,x.trust)).outcome,'unavailable');
 assert.equal(x.runtime.snapshot().consumed,0);
});

test('actual loan settlement uses outcome authority and discharges its existing dues',async()=>{
 const x=await prepared('loan');const done=await x.runtime.simulate(x.signed,x.plan,x.trust);
 assert.equal(done.outcome,'simulated',done.message);assert.equal(done.receipt.dueUpdates.length,2);
 const state=x.runtime.snapshot().agreements[0].state;
 assert.equal(state.values.principalDue,'0');assert.equal(state.values.interestDue,'0');
 assert.equal(state.values.principalPaid,'500000000');assert.equal(state.values.interestPaid,'33972602');
 assert.equal(done.receipt.status,'simulated-complete');assert.equal(done.receipt.proofStatus,'unavailable');
});

test('two concurrent proposals consume a nonce once and commit balances atomically',async()=>{
 const x=await prepared();const other=await x.runtime.propose(x.signed.intentHash,1);
 const rs=await Promise.all([x.runtime.simulate(x.signed,x.plan,x.trust),x.runtime.simulate(x.signed,other,x.trust)]);
 assert.equal(rs.filter(r=>r.outcome==='simulated').length,1);assert.equal(x.runtime.snapshot().consumed,1);
 const before=JSON.stringify(x.runtime.snapshot());assert.equal((await x.runtime.simulate(x.signed,x.plan,x.trust)).outcome,'rejected');assert.equal(JSON.stringify(x.runtime.snapshot()),before);
});

test('same principal/domain/nonce cannot be reused by re-signing a changed intent',async()=>{
 const x=await prepared();assert.equal((await x.runtime.simulate(x.signed,x.plan,x.trust)).outcome,'simulated');
 const changed=structuredClone(x.intent);changed.goals[0].minCredit='1';
 const signed=await signIntent(changed,x.keys);assert.notEqual(signed.intentHash,x.signed.intentHash);
 const p=await x.runtime.propose(signed.intentHash);const before=JSON.stringify(x.runtime.snapshot());
 assert.equal((await x.runtime.simulate(signed,p,x.trust)).outcome,'rejected');assert.equal(JSON.stringify(x.runtime.snapshot()),before);
});

test('expiry, early validity and backwards local time cannot grant simulation authority',async()=>{
 const x=await prepared();x.runtime.advanceTime(x.intent.validity.expiresAt);
 const before=JSON.stringify(x.runtime.snapshot());assert.equal((await x.runtime.simulate(x.signed,x.plan,x.trust)).outcome,'rejected');
 assert.equal(x.runtime.advanceTime('99').outcome,'rejected');assert.equal(JSON.stringify(x.runtime.snapshot()),before);
 const y=await prepared();const future=structuredClone(y.intent);future.validity.notBefore='101';const signed=await signIntent(future,y.keys);
 assert.equal((await y.runtime.preview(signed,await y.runtime.propose(signed.intentHash),y.trust)).outcome,'rejected');
});

test('tampered plan, stale shared state and narrower authority reject without mutation',async()=>{
 const x=await prepared();const before=JSON.stringify(x.runtime.snapshot());
 for(const mutate of [p=>p.steps[0].programHash='00'.repeat(32),p=>p.steps[0].predecessorHash='00'.repeat(32),p=>p.steps[0].action.args.recipient='Mallory',p=>p.steps.push(p.steps[0]),p=>p.after={accepted:true}]){
  const p=structuredClone(x.plan);mutate(p);assert.equal((await x.runtime.simulate(x.signed,p,x.trust)).outcome,'rejected');assert.equal(JSON.stringify(x.runtime.snapshot()),before);
 }
 const strict=structuredClone(x.intent);strict.authority[0].maxDebit='9999';const signed=await signIntent(strict,x.keys);
 assert.equal((await x.runtime.preview(signed,await x.runtime.propose(signed.intentHash),x.trust)).outcome,'rejected');
 const otherIntent=structuredClone(x.intent);otherIntent.nonce='1';const otherSigned=await signIntent(otherIntent,x.keys);
 const staleOther=await x.runtime.propose(otherSigned.intentHash,1);
 assert.equal((await x.runtime.simulate(x.signed,x.plan,x.trust)).outcome,'simulated');
 assert.equal((await x.runtime.simulate(otherSigned,staleOther,x.trust)).outcome,'rejected');
 assert.equal(x.runtime.snapshot().consumed,1);
});

test('fee debits count towards gross authority and net goal and wrong-domain assets reject',async()=>{
 const x=await prepared();const i=structuredClone(x.intent);const out=i.goals[0].asset;
 i.authority.push({asset:out,maxDebit:'100',recipients:['fees']});i.fees.push({asset:out,maxFee:'100'});i.goals[0].minCredit='19743';
 const signed=await signIntent(i,x.keys);const plan=await x.runtime.propose(signed.intentHash);
 plan.fees.push({kind:'Fee',asset:out,from:i.principal,to:'fees',amount:'1'});
 assert.equal((await x.runtime.preview(signed,plan,x.trust)).outcome,'rejected');
 const loose=structuredClone(i);loose.goals[0].minCredit='19742';const signed2=await signIntent(loose,x.keys);plan.intentHash=signed2.intentHash;
 assert.equal((await x.runtime.preview(signed2,plan,x.trust)).outcome,'checked');
 const bad=structuredClone(plan);bad.fees[0].asset.domain='other-domain';assert.equal((await x.runtime.preview(signed2,bad,x.trust)).outcome,'rejected');
});

test('proposal rejects non-string hashes without invoking caller coercion',async()=>{
 const x=await prepared();let called=false;
 await assert.rejects(x.runtime.propose({toString(){called=true;return x.signed.intentHash;}}));
 assert.equal(called,false);
});

test('async checking snapshots caller inputs and returned evidence cannot mutate the world',async()=>{
 const x=await prepared();const operation=x.runtime.simulate(x.signed,x.plan,x.trust);
 x.signed.intent.authority[0].maxDebit='0';x.plan.steps[0].agreementId='Mallory';x.trust.publicKey='bad';
 const result=await operation;assert.equal(result.outcome,'simulated',result.message);
 const expected=JSON.stringify(x.runtime.snapshot());
 result.receipt.effects[0].amount='0';result.receipt.dueUpdates.push({kind:'Forged',fields:{}});
 const snapshot=x.runtime.snapshot();snapshot.balances[0].amount='0';snapshot.agreements[0].state.values.reserveA='0';
 assert.equal(JSON.stringify(x.runtime.snapshot()),expected);
});

test('clock change during async verification rejects without financial or nonce mutation',async()=>{
 const x=await prepared();const operation=x.runtime.simulate(x.signed,x.plan,x.trust);
 assert.equal(x.runtime.advanceTime('101').outcome,'advanced');
 const expected=JSON.stringify(x.runtime.snapshot());const result=await operation;
 assert.equal(result.outcome,'rejected');assert.equal(result.code,'StaleState');
 assert.equal(JSON.stringify(x.runtime.snapshot()),expected);assert.equal(x.runtime.snapshot().consumed,0);
});

test('malformed accessors and wrong trust cannot execute or mutate financial state',async()=>{
 const x=await prepared();const expected=JSON.stringify(x.runtime.snapshot());let invoked=false;
 const p=structuredClone(x.plan);Object.defineProperty(p,'fees',{enumerable:true,get(){invoked=true;return [];}});
 assert.equal((await x.runtime.simulate(x.signed,p,x.trust)).outcome,'rejected');assert.equal(invoked,false);
 for(const trust of [{...x.trust,principal:'Mallory'},{...x.trust,domain:{...x.trust.domain,network:'other'}}])
  assert.equal((await x.runtime.simulate(x.signed,x.plan,trust)).outcome,'rejected');
 assert.equal(JSON.stringify(x.runtime.snapshot()),expected);
});
