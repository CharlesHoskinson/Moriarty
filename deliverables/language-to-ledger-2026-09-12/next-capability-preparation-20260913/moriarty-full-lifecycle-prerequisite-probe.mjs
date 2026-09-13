import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import assert from 'node:assert/strict';
import {createFinancialAgreementSourceV5} from '/home/charl/Moriarty/experiments/moriarty-language/src/successor/financial-agreement-source-v5.ts';
import {createFinancialAgreementSourceV6} from '/home/charl/Moriarty/experiments/moriarty-language/src/successor/financial-agreement-source-v6.ts';
import {canonical} from '/home/charl/Moriarty/experiments/moriarty-language/src/successor/expression-wire-v1.ts';
const base='/home/charl/Moriarty/experiments/moriarty-language/';
const read=f=>readFileSync(base+'spec/successor/examples/'+f,'utf8');
const hash=x=>createHash('sha256').update(x).digest('hex');
const out=[];
for(const fee of [null,'0','1']) {
 const source=read(fee===null?'loan-lifecycle.mori':'loan-lifecycle-fee.mori');
 const state=JSON.parse(read('loan-lifecycle.state.json'));
 if(fee!==null){state.schemaVersion='moriarty-financial-lifecycle-state/2';state.authority={networkTag:hash('isolated local fixture network'),programDigest:hash('isolated local fixture program'),debtor:{party:'Borrower',capability:hash('fixture borrower')},lender:{party:'Lender',capability:hash('fixture lender')},outcomeIntent:{grossCaps:[{actor:'Borrower',asset:'Cash',maximumLedgerAmount:'110'},{actor:'Lender',asset:'Cash',maximumLedgerAmount:'100'}],minimumNetCredits:[]}};}
 const api=fee===null?createFinancialAgreementSourceV5():createFinancialAgreementSourceV6();
 let predecessor={post:{paid:'0',phase:'0'},financialPost:state};
 const rows=[];
 for(const [action,args] of [['originate',{originationId:'O1',transferId:'D1',...(fee!==null?{feeId:'F1',feeAmount:fee}:{})}],['accrue',{accrualId:'A1',periodIndex:'1',observedTime:'1060'}],['repay',{allocationId:'R1',nominal:'30',transferId:'P1'}],['settle',{allocationId:'R2',transferId:'P2'}]]) {
  const before=structuredClone(predecessor),financialJSON=JSON.stringify(predecessor.financialPost);
  const snapshot=canonical({Pre:predecessor.post,Args:args,Obs:{},workInitial:predecessor.financialPost.work.remaining});
  const result=api.evaluate(source,action,snapshot,financialJSON);
  assert.deepEqual(predecessor,before);
  rows.push({action,snapshot:JSON.parse(snapshot),financialPre:before.financialPost,result});
  if(result.status!=='FundedExpressionPrepared'){assert(!Object.hasOwn(result,'financialPost'));assert(!Object.hasOwn(result,'effects'));break;}
  predecessor=result;
 }
 out.push({scope:'isolated-local-evaluator-no-authentication',fee,sourceSha256:hash(source),rows});
 console.log(JSON.stringify({fee,rows:rows.map(r=>({action:r.action,status:r.result.status,code:r.result.code,workUsed:r.result.workUsed,work:r.result.financialPost?.work,balances:r.result.financialPost?.balances,allowances:r.result.financialPost?.allowances}))}));
}
assert.equal(out[0].rows.length,4);assert(out[0].rows.every(r=>r.result.status==='FundedExpressionPrepared'));
assert.equal(out[1].rows.length,4);assert(out[1].rows.every(r=>r.result.status==='FundedExpressionPrepared'));
assert.equal(out[2].rows.length,2);assert.equal(out[2].rows[0].result.status,'FundedExpressionPrepared');assert.equal(out[2].rows[1].result.status,'Rejected');
writeFileSync('/tmp/moriarty-full-lifecycle-prerequisite-probe-results.json',JSON.stringify(out,null,2)+'\n');
