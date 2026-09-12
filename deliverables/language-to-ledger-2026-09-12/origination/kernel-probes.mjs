import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {pathToFileURL,fileURLToPath} from 'node:url';
const root=process.argv[2],here=path.dirname(fileURLToPath(import.meta.url));
const {prepareFinancialLifecycle:prepare,admitFinancialLifecycleStateJSON:admit}=await import(pathToFileURL(path.join(root,'experiments/moriarty-language/src/successor/financial-lifecycle.ts')).href);
const oracle=JSON.parse(fs.readFileSync(path.join(here,'../design-review/origination-accrual-expectations.json'),'utf8'));
const bounds=JSON.parse(fs.readFileSync(path.join(here,'result-bound-fixtures.json'),'utf8'));
const clone=structuredClone,J=JSON.stringify,V='moriarty-financial-lifecycle/1';
const results=[];
function test(name,fn){try{fn();results.push({name,pass:true});}catch(e){results.push({name,pass:false,error:e.message});}}
function run(state,actions){return prepare(J({schemaVersion:V,state,actions}));}
function rejected(r,code,index){assert.deepEqual(r,{status:'Rejected',code,actionIndex:index});}
function admitted(state){const a=admit(J(state));assert.equal(a.ok,true,J(a));assert.deepEqual(a.value,state);}
for(const t of oracle.kernelTrace)test('complete independent trace: '+t.name,()=>{
 const text=J({schemaVersion:V,state:t.pre,actions:t.actions});const r=prepare(text);
 assert.deepEqual(r,{status:'Prepared',schemaVersion:V,post:t.post,effects:t.effects});
 admitted(r.post);assert.equal(J({schemaVersion:V,state:t.pre,actions:t.actions}),text);
 r.post.work.spent='0';r.effects.length=0;
 assert.deepEqual(prepare(text),{status:'Prepared',schemaVersion:V,post:t.post,effects:t.effects});
});
for(const c of oracle.negativeKernelCases)test('independent rejection: '+c.name,()=>{
 const text=J({schemaVersion:V,state:c.pre,actions:c.actions});
 assert.deepEqual(prepare(text),c.expected);assert.deepEqual(prepare(text),c.expected);
});
for(const c of oracle.additionalNumericOracles)test('numeric '+J(c),()=>{
 // A well-formed local projection for arithmetic admission, not authenticated history.
 const s=clone(oracle.kernelTrace[0].post),o=s.obligations[0];
 for(const k of ['principal','outstanding','initialPrincipal','liabilityIncurred'])o[k]=c.principal;
 o.nominalLiabilityCap='10000';o.accrualTerms.numerator=c.numerator;o.accrualTerms.denominator=c.denominator;o.accrualTerms.rounding=c.rounding;
 admitted(s);const r=run(s,oracle.kernelTrace[1].actions);
 if(c.expectedCode){rejected(r,c.expectedCode,0);return;}
 assert.equal(r.status,'Prepared',J(r));admitted(r.post);
 assert.equal(r.effects[0].kind,'Accrual');assert.equal(r.effects[0].interestAmount,c.interestAmount);
 assert.equal(r.post.obligations[0].outstanding,String(BigInt(c.principal)+BigInt(c.interestAmount)));
 assert.equal(r.post.obligations[0].liabilityIncurred,r.post.obligations[0].outstanding);
 assert.equal(r.post.obligations[0].lastAccruedPeriod,'1');assert.equal(r.post.obligations[0].nextAccrualAt,'1120');
 assert.deepEqual(r.post.usedAccrualIds,['A1']);assert.deepEqual(r.post.balances,s.balances);assert.deepEqual(r.post.allowances,s.allowances);
 assert.deepEqual(r.post.work,{remaining:'253',spent:'20',closureReserve:'16'});
});
for(const c of bounds.cases)test('result byte bound: '+c.name,()=>{
 assert.ok(Buffer.byteLength(J(c.input))<=65536);admitted(c.input.state);
 const r=prepare(J(c.input));
 if(c.expectedStatus==='Rejected'){assert.deepEqual(r,c.expectedRejection);return;}
 assert.equal(r.status,'Prepared',J(r));assert.deepEqual(r.post,c.mathematicalPost);admitted(r.post);
 assert.equal(Buffer.byteLength(J(r.post)),65536);
});
test('next UInt64 boundary overflow precedes lifetime cap',()=>{
 const s=clone(oracle.kernelTrace[0].post),o=s.obligations[0];
 o.nominalLiabilityCap='100';o.accrualTerms.firstPeriodStart=String(2n**64n-2n);o.accrualTerms.periodSeconds='1';o.nextAccrualAt=String(2n**64n-1n);
 admitted(s);const a=clone(oracle.kernelTrace[1].actions[0]);a.observedTime=o.nextAccrualAt;
 rejected(run(s,[a]),'OVERFLOW',0);
});
test('reused accrual identity precedes wrong period and early time',()=>{
 const s=clone(oracle.kernelTrace[1].post),a=clone(oracle.kernelTrace[1].actions[0]);a.periodIndex='9';a.observedTime='0';
 rejected(run(s,[a]),'DUPLICATE',0);
});
test('fresh skipped period precedes early time',()=>{
 const s=clone(oracle.kernelTrace[0].post),a=clone(oracle.kernelTrace[1].actions[0]);a.periodIndex='9';a.observedTime='0';
 rejected(run(s,[a]),'PERIOD_SEQUENCE',0);
});
test('all-action shape admission precedes missing current funding',()=>{
 const t=oracle.kernelTrace[0],a=clone(t.actions[1]),later={...clone(t.actions[1]),unknown:'x'};
 const r=run(t.pre,[a,later]);assert.equal(r.status,'Rejected');assert.equal(r.actionIndex,null);assert.notEqual(r.code,'TRANSFER_NOT_IN_STEP');
 for(const k of ['post','effects','financialPost'])assert.equal(Object.hasOwn(r,k),false);
});
function reverseFundingCase(){
 const pre=clone(oracle.kernelTrace[0].pre),o=clone(oracle.kernelTrace[0].post.obligations[0]);
 Object.assign(o,{id:'ReverseLoan',debtor:'Lender',creditor:'Borrower',originationId:'OldOrigin',originationTransferId:'OldDisbursement',principal:'50',initialPrincipal:'50',outstanding:'50',liabilityIncurred:'50',nominalLiabilityCap:'50'});
 pre.obligations=[o];pre.usedTransferIds=['OldDisbursement'];pre.usedOriginationIds=['OldOrigin'];admitted(pre);
 const [transfer,originate]=clone(oracle.kernelTrace[0].actions);
 const repay={kind:'Repay',allocationId:'PartialOldDebt',transferId:'D1',obligationId:'ReverseLoan',payer:'Lender',nominalAmount:'30'};
 return {pre,transfer,originate,repay};
}
test('partial reverse-role repayment prevents sharing disbursement with origin',()=>{
 const {pre,transfer,originate,repay}=reverseFundingCase();
 rejected(run(pre,[transfer,repay,originate]),'TRANSFER_ALREADY_ALLOCATED',2);
 const r=run(pre,[transfer,repay]);assert.equal(r.status,'Prepared',J(r));
 assert.equal(r.post.obligations[0].outstanding,'20');admitted(r.post);
});
test('origination consumes funding before later reverse-role repayment',()=>{
 const {pre,transfer,originate,repay}=reverseFundingCase();
 rejected(run(pre,[transfer,originate,repay]),'INSUFFICIENT_UNALLOCATED',2);
});
test('zero-interest period still exhausts full accrual history',()=>{
 const s=clone(oracle.kernelTrace[0].post),o=s.obligations[0];
 o.accrualTerms.numerator='0';o.lastAccruedPeriod='128';o.nextAccrualAt='8740';
 s.usedAccrualIds=Array.from({length:128},(_,i)=>'A'+String(i+1));admitted(s);
 rejected(run(s,[{kind:'Accrue',accrualId:'NewPeriod',obligationId:'Loan1',periodIndex:'129',observedTime:'8740'}]),'CAPACITY',0);
});
test('cursor-history equation enforced on unrelated retained state',()=>{
 const s=clone(oracle.kernelTrace[0].post);s.obligations[0].lastAccruedPeriod='1';s.obligations[0].nextAccrualAt='1120';
 const a=admit(J(s));assert.equal(a.ok,false);assert.equal(a.result.code,'INVARIANT');assert.equal(a.result.actionIndex,null);
});
test('initial principal cannot be restored past admitted original principal',()=>{
 const s=clone(oracle.kernelTrace[2].post);s.obligations[0].principal='101';s.obligations[0].outstanding='101';
 const a=admit(J(s));assert.equal(a.ok,false);assert.equal(a.result.code,'INVARIANT');
});
test('valid prototype-like identity is a financial ID',()=>{
 const s=clone(oracle.kernelTrace[0].post);s.obligations[0].id='constructor';admitted(s);
 const a=clone(oracle.kernelTrace[1].actions[0]);a.obligationId='constructor';
 const r=run(s,[a]);assert.equal(r.status,'Prepared',J(r));assert.equal(r.post.obligations[0].outstanding,'110');admitted(r.post);
});
for(const [rule,principal,accrued,dP,dA] of [['AccrualFirst','80','0','20','10'],['PrincipalFirst','70','10','30','0'],['ProRata','73','7','27','3']])test('partial repayment retains lifetime liability under '+rule,()=>{
 const s=clone(oracle.kernelTrace[1].post);s.obligations[0].allocationRule=rule;
 const expected=clone(oracle.kernelTrace[2].post);Object.assign(expected.obligations[0],{allocationRule:rule,principal,accrued});
 const effects=clone(oracle.kernelTrace[2].effects);effects[1].principalDischarged=dP;effects[1].accruedDischarged=dA;
 const r=run(s,oracle.kernelTrace[2].actions);
 assert.deepEqual(r,{status:'Prepared',schemaVersion:V,post:expected,effects});admitted(r.post);
 const next={kind:'Accrue',accrualId:'A2',obligationId:'Loan1',periodIndex:'2',observedTime:'1120'};
 rejected(run(r.post,[next]),'LIABILITY_CAP_EXCEEDED',0);
 const settled=run(r.post,oracle.kernelTrace[3].actions);assert.equal(settled.status,'Prepared',J(settled));admitted(settled.post);
 const o=settled.post.obligations[0];assert.equal(o.status,'Settled');assert.equal(o.principal,'0');assert.equal(o.accrued,'0');assert.equal(o.outstanding,'0');
 assert.equal(o.initialPrincipal,'100');assert.equal(o.liabilityIncurred,'110');assert.equal(o.nominalLiabilityCap,'110');assert.equal(o.lastAccruedPeriod,'1');
 assert.deepEqual(settled.post.work,{remaining:'249',spent:'24',closureReserve:'16'});
});
console.log(J({passed:results.filter(x=>x.pass).length,failed:results.filter(x=>!x.pass).length,results}));
if(results.some(x=>!x.pass))process.exitCode=1;
