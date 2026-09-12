import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {source,twoOrigins} from './source-fixture.mjs';
const root=process.argv[2],here=path.dirname(fileURLToPath(import.meta.url));
const load=async p=>import(pathToFileURL(path.join(root,'experiments/moriarty-language',p)).href);
const {createFinancialAgreementSourceV5}=await load('src/successor/financial-agreement-source-v5.ts');
const {createFinancialExpressionContractV4}=await load('src/successor/financial-expression-v4.ts');
const {admitFinancialLifecycleStateJSON:admit}=await load('src/successor/financial-lifecycle.ts');
const {canonical:C}=await load('src/successor/expression-wire-v1.ts');
const oracle=JSON.parse(fs.readFileSync(path.join(here,'../design-review/origination-accrual-expectations.json'),'utf8'));
const bounds=JSON.parse(fs.readFileSync(path.join(here,'result-bound-fixtures.json'),'utf8'));
const clone=structuredClone,J=JSON.stringify,results=[];
function test(name,fn){try{fn();results.push({name,pass:true});}catch(e){results.push({name,pass:false,error:e.message});}}
const api=createFinancialAgreementSourceV5();
const originSnap=(work='256')=>({Pre:{phase:'0'},Args:{transferId:'D1',originationId:'O1'},Obs:{},workInitial:work});
const accrueSnap=(work='201')=>({Pre:{phase:'1'},Args:{accrualId:'A1',periodIndex:'1',observedTime:'1060'},Obs:{},workInitial:work});
function evaluate(src=source,s=oracle.kernelTrace[0].pre,snap=originSnap(),action='originate'){return api.evaluate(src,action,C(snap),J(s));}
function noCandidate(r,code){assert.equal(r.status,'Rejected',J(r));assert.equal(r.code,code);for(const k of ['post','financialPost','effects','descriptors','continueSuffix'])assert.equal(Object.hasOwn(r,k),false);}
function expected(t,phase,remaining,spent){const financialPost=clone(t.post);financialPost.work={remaining,spent,closureReserve:'16'};return {status:'FundedExpressionPrepared',post:{phase},financialPost,effects:t.effects,workRemaining:remaining};}
const originExpected=expected(oracle.kernelTrace[0],'1','201','72');
const accrueExpected=expected(oracle.kernelTrace[1],'2','180','93');
test('independent source declares and checks both actions',()=>{
 const r=api.check(source);assert.equal(r.judgmentResult,'SourceChecked',J(r));
 assert.deepEqual(r.actions,[{action:'originate',staticWorkBound:'53'},{action:'accrue',staticWorkBound:'20'}]);
});
test('source origination exact complete result costs55',()=>assert.deepEqual(evaluate(),originExpected));
test('source accrual chains complete origin output and costs21',()=>{
 const a=evaluate();assert.equal(a.status,'FundedExpressionPrepared',J(a));
 const r=evaluate(source,a.financialPost,accrueSnap(),'accrue');assert.deepEqual(r,accrueExpected);
 const admitted=admit(J(r.financialPost));assert.equal(admitted.ok,true);assert.deepEqual(admitted.value,r.financialPost);
});
test('Core4 and source5 complete result agreement',()=>{
 const a=api.elaborate(source).actions.find(x=>x.action==='originate');
 const core=createFinancialExpressionContractV4(C(a.schema),J(oracle.kernelTrace[0].pre));
 assert.deepEqual(core.evaluate(C({contract:'moriarty-financial-expression-contract/4',source,core:a.core,...originSnap()})),originExpected);
});
test('structural operation binding permits renamed nested records',()=>{
 let renamed=source;
 for(const [a,b] of [['ConversionFields','XConv'],['AccrualTermsFields','XTerms'],['OriginateFields','XOrigin'],['AccrueFields','XAccrual'],['TransferFields','XTransfer'],['RepayFields','XRepay']])renamed=renamed.replaceAll(a,b);
 assert.deepEqual(evaluate(renamed),originExpected);
});
test('exact source spendable55 preserves reserve',()=>{
 const s=clone(oracle.kernelTrace[0].pre);s.work.remaining='55';
 assert.deepEqual(evaluate(source,s,originSnap('55')),expected(oracle.kernelTrace[0],'1','0','72'));
});
test('late source exhaustion54 publishes no originated obligation',()=>{
 const s=clone(oracle.kernelTrace[0].pre);s.work.remaining='54';
 const r=evaluate(source,s,originSnap('54'));noCandidate(r,'WORK_EXHAUSTED');assert.equal(r.workUsed,'54');
});
test('false origin debt postcondition costs49 and rolls back disbursement',()=>{
 const bad=source.replace('post_outstanding<Cash>("Loan1") == nominal','post_outstanding<Cash>("Loan1") == quantity<Units<Cash,1>,0>(101)');
 const r=evaluate(bad);noCandidate(r,'ENSURES_FAILED');assert.equal(r.workUsed,'49');assert.deepEqual(evaluate(),originExpected);
});
test('success outputs are owned across repeat calls',()=>{
 const r=evaluate();r.financialPost.obligations[0].nominalLiabilityCap='999';r.effects.length=0;assert.deepEqual(evaluate(),originExpected);
});
test('unselected invalid post-read scope precedes missing selector',()=>{
 const bad=source.replace('action accrue(accrualId: Text, periodIndex: UInt64, observedTime: UInt64) {','action accrue(accrualId: Text, periodIndex: UInt64, observedTime: UInt64) { let forbidden = post_balance<Cash>("Borrower");');
 noCandidate(evaluate(bad,oracle.kernelTrace[0].pre,originSnap(),'missing'),'TYPE_POST_SCOPE');
});
test('foreign-denomination accrue target rejected before suffix',()=>{
 const s=clone(originExpected.financialPost);s.obligations[0].denomination='Token';assert.equal(admit(J(s)).ok,true);
 const r=evaluate(source,s,accrueSnap(),'accrue');noCandidate(r,'NOMINAL_UNIT');assert.equal(Object.hasOwn(r,'span'),false);
});
for(const trim of [9,10])test('source final debit byte boundary trim'+trim,()=>{
 const s=clone(bounds.cases[0].input.state);s.usedAllocationIds[0]=s.usedAllocationIds[0].slice(0,-trim);s.work.spent='928';
 assert.equal(admit(J(s)).ok,true);
 const r=evaluate(twoOrigins,s);
 if(trim===9){assert.deepEqual(r,{status:'Rejected',code:'RESULT_BOUND',actionIndex:null});return;}
 const post=clone(bounds.cases[0].mathematicalPost);post.usedAllocationIds[0]=post.usedAllocationIds[0].slice(0,-trim);post.work={remaining:'169',spent:'1015',closureReserve:'16'};
 const effects=clone(oracle.kernelTrace[0].effects),second=clone(effects);second[0].id='D2';Object.assign(second[1],{originationId:'O2',transferId:'D2',obligationId:'Loan2'});
 assert.deepEqual(r,{status:'FundedExpressionPrepared',post:{phase:'1'},financialPost:post,effects:[...effects,...second],workRemaining:'169'});
 assert.equal(Buffer.byteLength(C(post)),65536);assert.equal(admit(C(post)).ok,true);
});
console.log(J({passed:results.filter(x=>x.pass).length,failed:results.filter(x=>!x.pass).length,results}));
if(results.some(x=>!x.pass))process.exitCode=1;
