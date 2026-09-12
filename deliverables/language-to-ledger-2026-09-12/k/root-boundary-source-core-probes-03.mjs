import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {pathToFileURL} from 'node:url';
import path from 'node:path';
const root=path.resolve(process.argv[2]);
const imp=p=>import(pathToFileURL(path.join(root,'experiments/moriarty-language',p)).href);
const {createFinancialAgreementSourceV5}=await imp('src/successor/financial-agreement-source-v5.ts');
const {createFinancialExpressionContractV4}=await imp('src/successor/financial-expression-v4.ts');
const {canonical}=await imp('src/successor/expression-wire-v1.ts');
const read=p=>readFileSync(path.join(root,'experiments/moriarty-language/spec/successor/examples',p),'utf8');
const source=read('loan-lifecycle.mori');
const oracle=JSON.parse(readFileSync(new URL('root-complete-oracle-01.json',import.meta.url),'utf8'));
const initial={post:JSON.parse(read('loan-lifecycle.snapshots.json')).Pre,financialPost:JSON.parse(read('loan-lifecycle.state.json'))};
const language=createFinancialAgreementSourceV5();
const args=[{transferId:'D1',originationId:'O1'},{accrualId:'A1',periodIndex:'1',observedTime:'1060'},{transferId:'P1',allocationId:'R1',nominal:'30'},{transferId:'P2',allocationId:'R2'}];
const costs=[99,65,88,86]; const prefixes=[36,10,35,33]; const kernelCosts=[2,1,2,2];
const records=[];
function evaluate(src,index,pre,fin,aa=args[index]) {
 const e=language.elaborate(src); assert.equal(e.judgmentResult,'SourceElaborated');
 const stage=oracle.stages[index], item=e.actions.find(x=>x.action===stage.action);
 const snap={Pre:pre,Args:aa,Obs:{},workInitial:fin.work.remaining};
 const before=JSON.stringify({pre,fin,aa});
 const sourceResult=language.evaluate(src,stage.action,canonical(snap),JSON.stringify(fin));
 const coreResult=createFinancialExpressionContractV4(canonical(item.schema),JSON.stringify(fin)).evaluate(canonical({contract:e.contract,source:src,core:item.core,...snap}));
 assert.deepEqual(sourceResult,coreResult);assert.equal(JSON.stringify({pre,fin,aa}),before);
 if(sourceResult.status==='Rejected') assert.deepEqual(Object.keys(sourceResult).filter(x=>['post','financialPost','effects','workRemaining'].includes(x)),[]);
 return sourceResult;
}
for(let i=0;i<4;i++) {
 const predecessor=i===0?initial:oracle.stages[i-1].expected;
 for(const [mode,work] of [['exact',costs[i]],['suffix-minus-one',costs[i]-1],['kernel-short',prefixes[i]+kernelCosts[i]-1],['suffix-start',prefixes[i]+kernelCosts[i]]]) {
  const fin=structuredClone(predecessor.financialPost);fin.work.remaining=String(work);
  const result=evaluate(source,i,predecessor.post,fin);
  if(mode==='exact') { const expected=structuredClone(oracle.stages[i].expected);expected.financialPost.work.remaining='0';expected.workRemaining='0';assert.deepEqual(result,expected); }
  else {assert.equal(result.status,'Rejected');assert.equal(result.code,mode==='kernel-short'?'INSUFFICIENT_WORK':'WORK_EXHAUSTED');if(mode==='kernel-short')assert.equal(result.actionIndex,null);else assert.equal(result.workUsed,String(work));}
  records.push({id:oracle.stages[i].action+'-'+mode,preWork:fin.work,result});
 }
}
// The failure is in the last settlement ensure after both protected operations.
const badSource=source.replace('ensures post_balance<Cash>("Lender") == amount<Cash>(110);','ensures post_balance<Cash>("Lender") == amount<Cash>(111);');
assert.notEqual(badSource,source);
const pre=oracle.stages[2].expected;
const failed=evaluate(badSource,3,pre.post,pre.financialPost);
assert.equal(failed.status,'Rejected');assert.equal(failed.code,'ENSURES_FAILED');assert.equal(failed.workUsed,'86');
records.push({id:'settlement-final-ensure-rollback',result:failed});
// A failed financial transfer wins over that false suffix and leaves the predecessor intact.
const lacking=structuredClone(pre.financialPost);lacking.balances.find(x=>x.party==='Borrower'&&x.asset==='Cash').amount='79';
const kernelFailed=evaluate(badSource,3,pre.post,lacking);
assert.equal(kernelFailed.status,'Rejected');assert.equal(kernelFailed.code,'INSUFFICIENT_BALANCE');assert.equal(kernelFailed.actionIndex,0);
records.push({id:'funding-before-final-ensure',result:kernelFailed});
console.log(JSON.stringify({scope:'Actual Source5/Core4 boundary probes with independent literal work costs; alternate valid work budgets are isolated fixtures, not normal lifecycle history; no K agreement claim',kInvocations:0,actualChecks:records.length,records},null,2));
