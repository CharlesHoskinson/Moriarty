import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import {pathToFileURL} from 'node:url';
const root=process.argv[2],base=path.join(root,'experiments/moriarty-language');
const load=async p=>import(pathToFileURL(path.join(base,p)).href);
const {canonical}=await load('src/successor/expression-wire-v1.ts');
const {createFinancialAgreementSourceV4}=await load('src/successor/financial-agreement-source-v4.ts');
const {createFinancialExpressionContractV3}=await load('src/successor/financial-expression-v3.ts');
const {createFinancialExpressionContractV2}=await load('src/successor/financial-expression-v2.ts');
const {admitRepaymentStateJSON}=await load('src/successor/repayment.ts');
const src=fs.readFileSync(path.join(base,'spec/successor/examples/financial-state-payment.mori'),'utf8').replace('agreement-source/3','agreement-source/4');
const state=JSON.parse(fs.readFileSync(path.join(base,'spec/successor/examples/financial-state-payment.state.json'),'utf8'));
const snapshots=JSON.parse(fs.readFileSync(path.join(base,'spec/successor/examples/financial-state-payment.snapshots.json'),'utf8'));
snapshots.Args={transferId:'CoreT1',allocationId:'CoreA1'};
const api=createFinancialAgreementSourceV4(),artifact=api.elaborate(src);
assert.equal(artifact.judgmentResult,'SourceElaborated');
const action=artifact.actions.find(a=>a.action==='repay_remaining');
const schema=canonical(action.schema),stateText=JSON.stringify(state),contract='moriarty-financial-expression-contract/3';
const req={contract,source:src,core:action.core,...snapshots};
const factory=()=>createFinancialExpressionContractV3(schema,stateText);
const cases=[];
function test(name,fn){try{fn();cases.push({name,pass:true});}catch(e){cases.push({name,pass:false,error:e.message});}}
function reject(r,code){assert.equal(r.status,'Rejected');assert.equal(r.code,code);for(const p of ['post','financialPost','descriptors','effects','continuation'])assert.equal(Object.hasOwn(r,p),false);}
const SYN={kind:'synthetic',start:'0',end:'0'};
function exactSynthetic(r,code){assert.deepEqual(r,{status:'Rejected',code,span:SYN,nodePath:[],workUsed:'0'});}
function node(constructor,operands){return {constructor,operands,span:SYN};}
const literal=node('LitUInt',{width:'128',value:'1'});
const read=node('ReadPostBalance',{asset:'Cash',identity:node('LitText',{value:'Payer'})});
test('Core and source complete funded results match',()=>assert.deepEqual(factory().evaluate(canonical(req)),api.evaluate(src,'repay_remaining',canonical(snapshots),stateText)));
test('Core check contextless',()=>assert.equal(createFinancialExpressionContractV3(schema).check(canonical(req)).judgmentResult,'ExpressionChecked'));
test('missing context exact envelope',()=>exactSynthetic(createFinancialExpressionContractV3(schema).evaluate(canonical(req)),'FINANCIAL_CONTEXT_REQUIRED'));
test('standalone typed expression action-required',()=>exactSynthetic(factory().evaluate(canonical({...req,core:literal})),'TYPE_ACTION_REQUIRED'));
test('standalone expression remains checkable',()=>assert.equal(factory().check(canonical({...req,core:literal})).judgmentResult,'ExpressionChecked'));
test('post expression outside ensures static rejection',()=>reject(factory().check(canonical({...req,core:read})),'TYPE_POST_SCOPE'));
test('invalid Core precedes missing context',()=>reject(createFinancialExpressionContractV3(schema).evaluate(canonical({...req,core:node('Unknown',{})})),'TYPE_CONSTRUCTOR'));
for(const name of ['financialPost','resume','callback'])test('closed request rejects '+name,()=>reject(factory().evaluate(canonical({...req,[name]:{}})),'INPUT_SCHEMA'));
test('no context object accepted',()=>reject(createFinancialExpressionContractV3(schema,state).evaluate(canonical(req)),'INPUT_SCHEMA'));
test('no request object accepted',()=>reject(factory().evaluate(req),'INPUT_SCHEMA'));
test('schema object rejects without coercion',()=>{let result;try{result=createFinancialExpressionContractV3(action.schema,stateText).evaluate(canonical(req));}catch(e){result={status:'Rejected',code:e.code};}reject(result,'INPUT_SCHEMA');});
test('mutated result cannot influence next evaluation',()=>{const core=factory();const first=core.evaluate(canonical(req));assert.equal(first.status,'FundedExpressionPrepared');const snapshot=structuredClone(first);first.financialPost.obligations[0].outstanding='999';first.effects.length=0;assert.deepEqual(core.evaluate(canonical(req)),snapshot);});
test('mutated elaboration cannot influence source reevaluation',()=>{const expected=api.evaluate(src,'repay_remaining',canonical(snapshots),stateText);artifact.actions.length=0;assert.deepEqual(api.evaluate(src,'repay_remaining',canonical(snapshots),stateText),expected);});
for(const kind of ['balance','allowance','obligation','work'])test('state admission kernel envelope '+kind,()=>{
 const s=structuredClone(state);
 if(kind==='balance')s.balances[0].amount='-1';
 if(kind==='allowance')s.allowances.push(structuredClone(s.allowances[0]));
 if(kind==='obligation')s.obligations[0].outstanding='99';
 if(kind==='work')s.work.extra='0';
 const str=JSON.stringify(s),a=admitRepaymentStateJSON(str);assert.equal(a.ok,false);
 assert.deepEqual(createFinancialExpressionContractV3(schema,str).evaluate(canonical(req)),a.result);
});
for(const name of ['ReadPostOutstanding','ReadPostPrincipal','ReadPostAccrued','ReadPostBalance','ReadPostAllowanceRemaining','ReadPostAllowanceSpent'])test('old Core rejects '+name,()=>{
 const isUnit=['ReadPostOutstanding','ReadPostPrincipal','ReadPostAccrued'].includes(name);
 const n=node(name,{[isUnit?'unit':'asset']:'Cash',identity:node('LitText',{value:isUnit?'Due100':'Payer'})});
 reject(createFinancialExpressionContractV2(schema,stateText).check(canonical({...req,contract:'moriarty-financial-expression-contract/2',core:n})),'TYPE_CONSTRUCTOR');
});
console.log(JSON.stringify({passed:cases.filter(c=>c.pass).length,failed:cases.filter(c=>!c.pass).length,cases},null,2));
if(cases.some(c=>!c.pass))process.exitCode=1;
