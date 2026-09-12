import assert from 'node:assert/strict';
import fs from 'node:fs';
import {source,originationPair} from './source-fixture.mjs';
const root='/home/charl/Moriarty/.worktrees/loan-origination-accrual';
const {createFinancialAgreementSourceV5}=await import(root+'/experiments/moriarty-language/src/successor/financial-agreement-source-v5.ts');
const {prepareFinancialLifecycle:kernel,admitFinancialLifecycleStateJSON:admit}=await import(root+'/experiments/moriarty-language/src/successor/financial-lifecycle.ts');
const {canonical:C}=await import(root+'/experiments/moriarty-language/src/successor/expression-wire-v1.ts');
const oracle=JSON.parse(fs.readFileSync(new URL('../design-review/origination-accrual-expectations.json',import.meta.url)));
const api=createFinancialAgreementSourceV5(),J=JSON.stringify,clone=structuredClone,results=[];
function test(name,f){f();results.push({name,pass:true});}
const snap={Pre:{phase:'0'},Args:{transferId:'D1',originationId:'O1'},Obs:{},workInitial:'256'};
function ev(src=source,s=oracle.kernelTrace[0].pre,ss=snap,action='originate'){return api.evaluate(src,action,C(ss),J(s));}
function rej(r,code){assert.equal(r.status,'Rejected');assert.equal(r.code,code);for(const k of ['post','financialPost','effects'])assert.equal(k in r,false);}
test('same-call explicit originate then accrue costs62 and retains immutable PRE',()=>{
 const accr='emit Accrue { accrualId: "A1", obligationId: "Loan1", periodIndex: u64(1), observedTime: u64(1060) };';
 const src=source.replace(originationPair(),originationPair()+accr).replace('post_outstanding<Cash>("Loan1") == nominal','post_outstanding<Cash>("Loan1") == quantity<Units<Cash,1>,0>(110)');
 const r=ev(src);assert.equal(r.status,'FundedExpressionPrepared',J(r));assert.equal(r.workRemaining,'194');assert.equal(r.financialPost.work.spent,'79');assert.equal(r.financialPost.obligations[0].liabilityIncurred,'110');assert.deepEqual(r.effects.map(x=>x.kind),['Transfer','Origination','Accrual']);assert.equal(admit(J(r.financialPost)).ok,true);
 rej(ev(src.replace('let nominal = quantity','let absent = outstanding<Cash>("Loan1"); let nominal = quantity')),'MISSING_OBLIGATION');
});
test('zero interest source event costs21 and records cursor and nonce',()=>{
 const src=source.replace('numerator: 1, denominator: 10','numerator: 0, denominator: 10').replace('post_outstanding<Cash>("Loan1") == quantity<Units<Cash,1>,0>(110)','post_outstanding<Cash>("Loan1") == quantity<Units<Cash,1>,0>(100)');
 const o=ev(src);const ss={Pre:o.post,Args:{accrualId:'A1',periodIndex:'1',observedTime:'1060'},Obs:{},workInitial:o.workRemaining};
 const r=ev(src,o.financialPost,ss,'accrue');assert.equal(r.status,'FundedExpressionPrepared',J(r));assert.equal(r.workRemaining,'180');assert.equal(r.effects[0].interestAmount,'0');assert.equal(r.financialPost.obligations[0].lastAccruedPeriod,'1');assert.deepEqual(r.financialPost.usedAccrualIds,['A1']);
 rej(ev(src,r.financialPost,{...ss,Pre:r.post,workInitial:r.workRemaining},'accrue'),'DUPLICATE');
});
test('negative nominal cap does not pass signed Quantity adapter',()=>rej(ev(source.replace('nominalLiabilityCap: quantity<Units<Cash,1>,0>(110)','nominalLiabilityCap: quantity<Units<Cash,1>,0>(-1)')),'NOMINAL_RANGE'));
test('effects terms and financialPost terms do not alias',()=>{const r=ev();r.effects[1].accrualTerms.numerator='999';r.effects[1].conversion.mantissa='999';assert.equal(r.financialPost.obligations[0].accrualTerms.numerator,'1');assert.equal(r.financialPost.obligations[0].conversion.mantissa,'1');assert.equal(ev().effects[1].accrualTerms.numerator,'1');});
test('overflow precedes history capacity on fully admitted state',()=>{
 const s=clone(oracle.kernelTrace[0].post),o=s.obligations[0];o.lastAccruedPeriod='128';o.nextAccrualAt='8740';o.accrualTerms.numerator=String(2n**128n-1n);s.usedAccrualIds=Array.from({length:128},(_,i)=>'A'+i);assert.equal(admit(J(s)).ok,true);
 assert.deepEqual(kernel(J({schemaVersion:'moriarty-financial-lifecycle/1',state:s,actions:[{kind:'Accrue',accrualId:'New',obligationId:'Loan1',periodIndex:'129',observedTime:'8740'}]})),{status:'Rejected',code:'OVERFLOW',actionIndex:0});
});
test('input primitive object cannot invoke coercion',()=>{let called=false;const x={toString(){called=true;return J(oracle.kernelTrace[0].pre)}};rej(api.evaluate(source,'originate',C(snap),x),'INPUT_SCHEMA');assert.equal(called,false);});
console.log(J({passed:results.length,results}));
