import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {createFinancialAgreementSourceV6} from '../src/successor/financial-agreement-source-v6.ts';
import {createFinancialAgreementSourceV5} from '../src/successor/financial-agreement-source-v5.ts';
import {admitFinancialLifecycleStateJSON, prepareFinancialLifecycle, LIFECYCLE_STATE_VERSION, LIFECYCLE_VERSION} from '../src/successor/financial-lifecycle-v2.ts';
import {admitFinancialLifecycleStateJSON as admitLegacy} from '../src/successor/financial-lifecycle.ts';
import {checkOutcomeIntent} from '../src/successor/financial-outcome-intent.ts';
import {canonical} from '../src/successor/expression-wire-v1.ts';
const fixture = f => readFileSync(new URL('../spec/successor/examples/'+f,import.meta.url),'utf8');
// Public test commitments are computed from labeled test inputs, never deployment identities.
const hash = s => createHash('sha256').update(s).digest('hex');
const authority = () => ({networkTag:hash('test network'),programDigest:hash('test program'),
 debtor:{party:'Borrower',capability:hash('test debtor')},lender:{party:'Lender',capability:hash('test lender')},
 outcomeIntent:{grossCaps:[{actor:'Borrower',asset:'Cash',maximumLedgerAmount:'110'},{actor:'Lender',asset:'Cash',maximumLedgerAmount:'100'}],minimumNetCredits:[]}});
const state = () => ({...JSON.parse(fixture('loan-lifecycle.state.json')),schemaVersion:LIFECYCLE_STATE_VERSION,authority:authority()});
const fee = (amount='1') => ({kind:'Fee',id:'Fee1',from:'Borrower',to:'Lender',asset:'Cash',amount});
const run = (actions,s=state())=>prepareFinancialLifecycle(JSON.stringify({schemaVersion:LIFECYCLE_VERSION,state:s,actions}));
const cap=(actor,maximumLedgerAmount)=>({actor,asset:'Cash',maximumLedgerAmount});
const goal=(actor,minimumLedgerAmount)=>({actor,asset:'Cash',minimumLedgerAmount});
const transfer=(from,to,amount)=>({kind:'Transfer',id:'T',from,to,asset:'Cash',amount});

test('/2 authority is required and preserved, /1 remains accepted',()=>{
 const s=state(); assert.equal(admitFinancialLifecycleStateJSON(JSON.stringify(s)).ok,true);
 delete s.authority; assert.equal(admitFinancialLifecycleStateJSON(JSON.stringify(s)).ok,false);
 assert.equal(admitLegacy(fixture('loan-lifecycle.state.json')).ok,true);
 const r=run([fee()]); assert.equal(r.status,'Prepared'); assert.deepEqual(r.post.authority,authority());
});
test('authority rejects malformed commitments and identical role parties',()=>{
 const s=state();s.authority.lender.capability='missing';assert.equal(admitFinancialLifecycleStateJSON(JSON.stringify(s)).ok,false);
 s.authority=authority();s.authority.lender.party='Borrower';assert.equal(admitFinancialLifecycleStateJSON(JSON.stringify(s)).ok,false);
});
test('Fee stays distinct and debits balance, allowance and work',()=>{
 const r=run([fee()]);assert.equal(r.status,'Prepared');assert.equal(r.effects[0].kind,'Fee');
 assert.equal(r.post.balances.find(x=>x.party==='Borrower').amount,'9');
 assert.equal(r.post.allowances.find(x=>x.party==='Borrower').spent,'1');
 assert.equal(BigInt(r.post.work.remaining),BigInt(state().work.remaining)-1n);
 assert.deepEqual(r.post.obligations,[]);
});
test('Fee cannot fund an Originate',()=>{
 const source=fixture('loan-lifecycle-fee.mori').replace('emit Transfer {','emit Fee {');
 const r=createFinancialAgreementSourceV6().evaluate(source,'originate',canonical({...JSON.parse(fixture('loan-lifecycle.snapshots.json')),Args:{transferId:'D1',originationId:'O1',feeId:'F1',feeAmount:'0'}}),JSON.stringify(state()));
 assert.equal(r.status,'Rejected'); assert.equal(r.code,'TRANSFER_NOT_IN_STEP');
});
test('gross debit cannot be erased by a refund',()=>{
 const effects=[transfer('Borrower','Lender','100'),transfer('Lender','Borrower','100')];
 assert.equal(checkOutcomeIntent(effects,{grossCaps:[cap('Borrower','99'),cap('Lender','100')],minimumNetCredits:[]}).code,'INTENT_DEBIT_CAP');
});
test('zero Fee without cap rejects before kernel',()=>{
 const s=state();s.authority.outcomeIntent.grossCaps=[];
 assert.equal(run([fee('0')],s).code,'INTENT_DEBIT_UNCAPPED');
});
test('net goal includes Fee and passes exactly at boundary',()=>{
 const e=[transfer('Lender','Borrower','110'),transfer('Borrower','Lender','100'),fee()];
 const i={grossCaps:[cap('Borrower','101'),cap('Lender','110')],minimumNetCredits:[goal('Borrower','10')]};
 assert.equal(checkOutcomeIntent(e,i).code,'INTENT_NET_GOAL');i.minimumNetCredits[0].minimumLedgerAmount='9';assert.equal(checkOutcomeIntent(e,i).ok,true);
});
test('self fee counts both sides and missing sums are zero',()=>{
 const e=[{...fee('7'),to:'Borrower'}];const i={grossCaps:[cap('Borrower','7')],minimumNetCredits:[goal('Borrower','0')]};
 assert.equal(checkOutcomeIntent(e,i).ok,true);i.minimumNetCredits[0].minimumLedgerAmount='1';assert.equal(checkOutcomeIntent(e,i).code,'INTENT_NET_GOAL');
 assert.equal(checkOutcomeIntent([],{grossCaps:[],minimumNetCredits:[goal('Borrower','0')]}).ok,true);
});
test('checked sum and debit plus minimum overflow reject',()=>{
 const max=((1n<<128n)-1n).toString();const i={grossCaps:[cap('Borrower',max)],minimumNetCredits:[goal('Borrower','1')]};
 assert.equal(checkOutcomeIntent([fee(max)],i).code,'INTENT_ARITHMETIC_OVERFLOW');
 assert.equal(checkOutcomeIntent([fee(max),fee()],{...i,minimumNetCredits:[]}).code,'INTENT_ARITHMETIC_OVERFLOW');
});
test('assets are independent and nonmonetary effects do not count',()=>{
 const e=[{...transfer('Lender','Borrower','100'),asset:'DUST'},{kind:'DueSettled',from:'Lender',to:'Borrower',asset:'Cash',amount:'100'}];
 const i={grossCaps:[{actor:'Lender',asset:'DUST',maximumLedgerAmount:'100'}],minimumNetCredits:[goal('Borrower','1')]};
 assert.equal(checkOutcomeIntent(e,i).code,'INTENT_NET_GOAL');
});
test('/6 public evaluator lowers Fee to Core /5 and checks the net goal atomically',()=>{
 const api=createFinancialAgreementSourceV6(),source=fixture('loan-lifecycle-fee.mori');
 const c=api.elaborate(source);assert.equal(c.contract,'moriarty-financial-expression-contract/5');
 assert.ok(c.actions.find(a=>a.action==='chargeFee').core.statements.some(s=>s.constructor==='Emit'&&s.operands.operation==='Fee'));
 const s=state(); const snap=canonical({Pre:{phase:'0',paid:'0'},Args:{feeId:'Fee1',feeAmount:'1'},Obs:{},workInitial:s.work.remaining});
 s.authority.outcomeIntent.minimumNetCredits=[goal('Borrower','0')];
 const r=api.evaluate(source,'chargeFee',snap,JSON.stringify(s));assert.equal(r.code,'INTENT_NET_GOAL');assert.equal('financialPost' in r,false);
 s.authority.outcomeIntent.minimumNetCredits=[];assert.equal(api.evaluate(source,'chargeFee',snap,JSON.stringify(s)).status,'FundedExpressionPrepared');
 assert.equal(createFinancialAgreementSourceV5().check(source).code,'PROFILE_MISMATCH');
});

import {exactLedgerAmount} from '../src/successor/financial-outcome-intent.ts';
import {parseFinancialAgreementSourceV6,formatFinancialAgreementSourceV6} from '../src/successor/financial-agreement-source-v6-frontend.ts';
test('exact quantum conversion rejects nondivisibility and overflow',()=>{
 assert.equal(exactLedgerAmount('100','1','1'),'100');
 assert.equal(exactLedgerAmount('10','1','2'),'5');
 assert.throws(()=>exactLedgerAmount('1','1','2'),/QUANTUM_INEXACT/);
 assert.throws(()=>exactLedgerAmount(((1n<<128n)-1n).toString(),'2','1'),/OVERFLOW/);
});
test('malformed effects cannot disappear from the outcome check',()=>{
 for(const e of [null,{kind:'Fée'}])assert.equal(checkOutcomeIntent([e],{grossCaps:[],minimumNetCredits:[]}).code,'INTENT_EFFECT_SCHEMA');
});
test('authority role substitution rejects before publishing debt',()=>{
 const s=state();s.authority.lender.party='Impostor';s.authority.outcomeIntent.grossCaps[1].actor='Impostor';
 const source=fixture('loan-lifecycle-fee.mori');
 const snap=canonical({...JSON.parse(fixture('loan-lifecycle.snapshots.json')),Args:{transferId:'D1',originationId:'O1',feeId:'F1',feeAmount:'0'}});
 const r=createFinancialAgreementSourceV6().evaluate(source,'originate',snap,JSON.stringify(s));
 assert.equal(r.status,'Rejected');assert.equal('financialPost' in r,false);
});
test('/6 parser preserves Unicode spans and profile-looking comments',()=>{
 const s='/* é moriarty-financial-agreement-source/5 */\n'+fixture('loan-lifecycle-fee.mori');
 const p=parseFinancialAgreementSourceV6(s);assert.equal(p.profile.value,'moriarty-financial-agreement-source/6');
 assert.equal(createFinancialAgreementSourceV6().check(formatFinancialAgreementSourceV6(s)).judgmentResult,'SourceChecked');
});
test('/6 rejects /1 state, duplicate Fee IDs and unknown debit actors',()=>{
 const r=createFinancialAgreementSourceV6().evaluate(fixture('loan-lifecycle-fee.mori'),'chargeFee',canonical({Pre:{phase:'0',paid:'0'},Args:{feeId:'F',feeAmount:'0'},Obs:{},workInitial:'512'}),fixture('loan-lifecycle.state.json'));
 assert.equal(r.status,'Rejected');assert.equal(run([fee(),fee()]).code,'DUPLICATE');
 assert.equal(run([{...fee(),from:'Other'}]).status,'Rejected');
});
test('self Fee preserves balance and still consumes gross allowance',()=>{
 const r=run([{...fee('7'),to:'Borrower'}]);assert.equal(r.status,'Prepared');
 assert.equal(r.post.balances.find(b=>b.party==='Borrower').amount,'10');
 assert.equal(r.post.allowances.find(b=>b.party==='Borrower').spent,'7');
});
