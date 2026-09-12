import assert from 'node:assert/strict';
import {loadLoanLifecycle,evaluateLoanLifecycleAction} from '/home/charl/Moriarty/.worktrees/loan-lifecycle/experiments/moriarty-language/examples/loan-lifecycle.mjs';
const {language,source,stateText}=loadLoanLifecycle();
const stages=[['originate',{originationId:'O1',transferId:'D1'},99,36,2],['accrue',{accrualId:'A1',periodIndex:'1',observedTime:'1060'},65,10,1],['repay',{allocationId:'R1',transferId:'P1',nominal:'30'},88,35,2],['settle',{allocationId:'R2',transferId:'P2'},86,33,2]];
let pred={post:{phase:'41',paid:'7'},financialPost:JSON.parse(stateText)};
let checks=0;
for (const [i,[action,args,W,prefix,N]] of stages.entries()) {
 const before=JSON.stringify(pred);
 let actualStrings;
 const observed={evaluate(...values){actualStrings=values;return language.evaluate(...values);}};
 const good=evaluateLoanLifecycleAction(observed,source,action,pred,args);
 assert.equal(good.result.status,'FundedExpressionPrepared');
 assert.deepEqual(JSON.parse(actualStrings[2]).Pre,pred.post);
 assert.deepEqual(JSON.parse(actualStrings[3]),pred.financialPost);
 assert.equal(JSON.parse(actualStrings[2]).workInitial,pred.financialPost.work.remaining);
 assert.deepEqual(good.result.post,{phase:String(42+i),paid:String([7,7,37,117][i])});
 assert.equal(JSON.stringify(pred),before);
 for(const [budget,code] of [[W-1,'WORK_EXHAUSTED'],[prefix+N-1,'INSUFFICIENT_WORK'],[prefix+N,'WORK_EXHAUSTED']]) {
   const fixture=structuredClone(pred);fixture.financialPost.work.remaining=String(budget);
   const original=JSON.stringify(fixture);
   const failed=evaluateLoanLifecycleAction(language,source,action,fixture,args).result;
   assert.equal(failed.code,code);
   for(const k of ['post','financialPost','effects','descriptors','workRemaining','continueSuffix'])assert.equal(Object.hasOwn(failed,k),false);
   assert.deepEqual(evaluateLoanLifecycleAction(language,source,action,fixture,args).result,failed);
   assert.equal(JSON.stringify(fixture),original);
   assert.deepEqual(evaluateLoanLifecycleAction(language,source,action,pred,args).result,good.result);
   checks++;
 }
 pred=good.result;
}
assert.deepEqual(pred.financialPost.work,{remaining:'174',spent:'355',closureReserve:'16'});
console.log(JSON.stringify({passed:true,actualHelperCallsCaptured:4,phaseAndPaidOffsetStages:4,deterministicWorkFailuresAndOriginalPredecessorRetries:checks,finalWork:pred.financialPost.work}));
