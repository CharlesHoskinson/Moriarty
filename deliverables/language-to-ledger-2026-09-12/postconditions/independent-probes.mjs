import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import { pathToFileURL } from 'node:url';
const root=process.argv[2];
if(!root)throw new Error('Pass checkout root');
const base=path.join(root,'experiments/moriarty-language');
const load=async p=>import(pathToFileURL(path.join(base,p)).href);
const {createFinancialAgreementSourceV4}=await load('src/successor/financial-agreement-source-v4.ts');
const {canonical}=await load('src/successor/expression-wire-v1.ts');
const api=createFinancialAgreementSourceV4();
const fixture=fs.readFileSync(path.join(base,'spec/successor/examples/financial-state-payment.mori'),'utf8');
const state0=JSON.parse(fs.readFileSync(path.join(base,'spec/successor/examples/financial-state-payment.state.json'),'utf8'));
const snapshot0=JSON.parse(fs.readFileSync(path.join(base,'spec/successor/examples/financial-state-payment.snapshots.json'),'utf8'));
const extra=[
 'ensures post_outstanding<Cash>("Due100") == quantity<Units<Cash, 1>, 0>(0);',
 'ensures post_principal<Cash>("Due100") == quantity<Units<Cash, 1>, 0>(0);',
 'ensures post_accrued<Cash>("Due100") == quantity<Units<Cash, 1>, 0>(0);',
 'ensures post_balance<Cash>("Payer") == amount<Cash>(0);',
 'ensures post_allowance_remaining<Cash>("Payer") == amount<Cash>(0);',
 'ensures post_allowance_spent<Cash>("Payer") == amount<Cash>(100);',
].join('\n    ');
const source=fixture.replace('agreement-source/3','agreement-source/4').replace(/\n  }\n}\s*$/,'\n    '+extra+'\n  }\n}\n');
assert.notEqual(source,fixture);
const cases=[];
function test(name,fn){try{fn();cases.push({name,pass:true});}catch(e){cases.push({name,pass:false,error:e.message});}}
function evaluate(src=source,work='256',edit=()=>{},action='repay_remaining'){
 const state=structuredClone(state0);state.work.remaining=work;
 const snap=structuredClone(snapshot0);snap.workInitial=work;snap.Args={transferId:'IndependentT1',allocationId:'IndependentA1'};
 edit(state,snap);
 const stateText=JSON.stringify(state),snapText=canonical(snap);
 const result=api.evaluate(src,action,snapText,stateText);
 return {result,state,snap,stateText,snapText};
}
function rejected(r,code){assert.equal(r.status,'Rejected');assert.equal(r.code,code);for(const k of ['post','financialPost','effects','descriptors','continuation'])assert.equal(Object.hasOwn(r,k),false,k);}
function complete(r,remaining,spent){
 assert.equal(r.status,'FundedExpressionPrepared');assert.equal(r.post.paid,'100');
 const s=r.financialPost;const o=s.obligations.find(x=>x.id==='Due100');
 assert.equal(o.principal,'0');assert.equal(o.accrued,'0');assert.equal(o.outstanding,'0');assert.equal(o.status,'Settled');
 assert.equal(s.balances.find(x=>x.party==='Payer'&&x.asset==='Cash').amount,'0');assert.equal(s.balances.find(x=>x.party==='Lender'&&x.asset==='Cash').amount,'100');
 const a=s.allowances.find(x=>x.party==='Payer'&&x.asset==='Cash');assert.equal(a.remaining,'0');assert.equal(a.spent,'100');
 assert.equal(s.work.remaining,remaining);assert.equal(s.work.spent,spent);assert.equal(s.work.closureReserve,'16');
 assert.deepEqual(s.usedTransferIds,['IndependentT1']);assert.deepEqual(s.usedAllocationIds,['IndependentA1']);assert.equal(r.effects.length,2);
 assert.equal(r.workRemaining,remaining);
 for(const key of ['balances','allowances'])assert.deepEqual(s[key].filter(x=>x.asset==='Token'),state0[key].filter(x=>x.asset==='Token'));
}
test('all actions statically check',()=>assert.equal(api.check(source).judgmentResult,'SourceChecked'));
test('complete payment independently costs82',()=>complete(evaluate().result,'174','82'));
test('exact spendable82 reserve separate',()=>complete(evaluate(source,'82').result,'0','82'));
test('existing spent retained',()=>complete(evaluate(source,'82',s=>{s.work.spent='17';}).result,'0','99'));
for(const work of ['81','43','40'])test('work exhaustion '+work,()=>{const r=evaluate(source,work).result;rejected(r,'WORK_EXHAUSTED');assert.equal(r.workUsed,work);});
for(const work of ['42','41'])test('kernel failure before suffix '+work,()=>rejected(evaluate(source,work).result,'INSUFFICIENT_WORK'));
test('first false postcondition rollback cost54',()=>{const r=evaluate(source.replace('post_outstanding<Cash>("Due100") == quantity<Units<Cash, 1>, 0>(0)','post_outstanding<Cash>("Due100") == quantity<Units<Cash, 1>, 0>(1)')).result;rejected(r,'ENSURES_FAILED');assert.equal(r.workUsed,'54');});
test('last false postcondition rollback cost82',()=>{const r=evaluate(source.replace('post_allowance_spent<Cash>("Payer") == amount<Cash>(100)','post_allowance_spent<Cash>("Payer") == amount<Cash>(99)')).result;rejected(r,'ENSURES_FAILED');assert.equal(r.workUsed,'82');});
test('unprefixed ensures reads retain pre',()=>{const src=source.replace('ensures post_outstanding','ensures outstanding<Cash>("Due100") == quantity<Units<Cash, 1>, 0>(100);\n    ensures post_outstanding');assert.equal(evaluate(src).result.status,'FundedExpressionPrepared');});
test('post read cannot escape ensure scope',()=>{const src=source.replace('let nominal = outstanding<Cash>','let nominal = post_outstanding<Cash>');rejected(api.check(src),'TYPE_POST_SCOPE');});
test('missing post identity rejects at runtime',()=>{const src=source.replace('post_balance<Cash>("Payer")','post_balance<Cash>("Missing")');rejected(evaluate(src).result,'MISSING_BALANCE');});
test('post identity newline rejected fully',()=>{const src=source.replace('post_balance<Cash>("Payer")','post_balance<Cash>("Payer\\n")');rejected(evaluate(src).result,'INVALID_IDENTIFIER');});
test('runtime short circuit omits missing post lookup',()=>{const src=source.replace(extra,'ensures true or post_balance<Cash>("Missing") == amount<Cash>(0);');const r=evaluate(src,'52').result;assert.equal(r.status,'FundedExpressionPrepared');assert.equal(r.workRemaining,'0');});
test('deterministic retry leaves input strings intact',()=>{const x=evaluate();assert.deepEqual(api.evaluate(source,'repay_remaining',x.snapText,x.stateText),x.result);assert.equal(JSON.stringify(x.state),x.stateText);});
console.log(JSON.stringify({passed:cases.filter(x=>x.pass).length,failed:cases.filter(x=>!x.pass).length,cases},null,2));
if(cases.some(x=>!x.pass))process.exitCode=1;
