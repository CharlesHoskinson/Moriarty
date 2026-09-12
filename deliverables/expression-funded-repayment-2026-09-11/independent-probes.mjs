import assert from 'node:assert/strict';
import {pathToFileURL} from 'node:url';
const root = process.argv[2];
const {createFundedFinancialExpressionSourceV1} = await import(pathToFileURL(`${root}/experiments/moriarty-language/src/successor/funded-expression-source-v1.ts`));
const {createFinancialExpressionSourceV1} = await import(pathToFileURL(`${root}/experiments/moriarty-language/src/successor/financial-expression-source-v1.ts`));
const {canonical} = await import(pathToFileURL(`${root}/experiments/moriarty-language/src/successor/expression-wire-v1.ts`));
const schema = {units:['DebtUnit'],assets:['Cash'],vaults:[],parties:['Lender','Payer'],recordTypes:{
  TransferFields:{id:['Text'],from:['Text'],to:['Text'],settlementAsset:['Text'],transferAmount:['Amount','Cash']},
  RepayFields:{allocationId:['Text'],transferId:['Text'],obligationId:['Text'],payer:['Text'],nominalAmount:['Quantity',[['DebtUnit','1']],'0']}
},enumTypes:{},variantTypes:{},fields:{paid:{type:['UInt128'],writeClass:'ordinary'}},args:{first:['Quantity',[['DebtUnit','1']],'0'],second:['Quantity',[['DebtUnit','1']],'0']},observations:{},operations:{Repay:'RepayFields',Transfer:'TransferFields'}};
const source=`profile "moriarty-financial-expression-source/1";
agreement RepaymentProbe { action pay(first: Quantity<Units<DebtUnit,1>,0>, second: Quantity<Units<DebtUnit,1>,0>) {
 let nominal = first + second;
 requires is_negative(nominal) == false;
 let cash = amount<Cash>(magnitude(nominal));
 next.paid = pre.paid + quanta(cash);
 emit Transfer { id: "PayOne", from: "Payer", to: "Lender", settlementAsset: "Cash", transferAmount: cash };
 emit Repay { allocationId: "AllocOne", transferId: "PayOne", obligationId: "Loan", payer: "Payer", nominalAmount: nominal };
 ensures post.paid == pre.paid + quanta(cash);
} }`;
const state=()=>({balances:[{party:'Payer',asset:'Cash',amount:'100'},{party:'Lender',asset:'Cash',amount:'0'},{party:'Reserve',asset:'Other',amount:'91'}],allowances:[{party:'Payer',asset:'Cash',remaining:'100',spent:'5'}],obligations:[{id:'Loan',debtor:'Payer',creditor:'Lender',denomination:'DebtUnit',settlementAsset:'Cash',principal:'100',accrued:'0',outstanding:'100',allocationRule:'AccrualFirst',conversion:{mantissa:'1',scale:'0',rounding:'none'},status:'Outstanding'}],usedTransferIds:['Historical'],usedAllocationIds:['HistoricalAlloc'],work:{remaining:'500',spent:'19',closureReserve:'16'}});
const snapshot=(first='10',second='20',work='500',paid='0')=>({Pre:{paid},Args:{first,second},Obs:{},workInitial:work});
const api=createFundedFinancialExpressionSourceV1(canonical(schema));
const pure=createFinancialExpressionSourceV1(canonical(schema));
let count=0;
function evaluate(s,ss,st,language=api){return language.evaluate(s,canonical(ss),JSON.stringify(st));}
function reject(s,ss,st,language=api){const result=evaluate(s,ss,st,language);assert.equal(result.status,'Rejected',JSON.stringify(result));for(const k of ['post','financialPost','effects','descriptors'])assert.equal(Object.hasOwn(result,k),false, k);count++;return result;}
for(const [first,second,interest] of [['10','20','0'],['3','4','10']]){
 const st=state();st.obligations[0].accrued=interest;st.obligations[0].outstanding=String(100+Number(interest));
 const ss=snapshot(first,second), before=JSON.stringify(st), result=evaluate(source,ss,st);
 assert.equal(result.status,'FundedExpressionPrepared',JSON.stringify(result));
 const payment=Number(first)+Number(second), dischargedInterest=Math.min(payment,Number(interest)), dischargedPrincipal=payment-dischargedInterest;
 const p=pure.evaluate(source,canonical(ss));assert.equal(p.status,'ExpressionPrepared');
 const E=500-Number(p.workRemaining), expected=structuredClone(st);
 expected.balances[0].amount=String(100-payment);expected.balances[1].amount=String(payment);
 expected.allowances[0].remaining=String(100-payment);expected.allowances[0].spent=String(5+payment);
 Object.assign(expected.obligations[0],{principal:String(100-dischargedPrincipal),accrued:String(Number(interest)-dischargedInterest),outstanding:String(100+Number(interest)-payment)});
 expected.usedTransferIds.push('PayOne');expected.usedAllocationIds.push('AllocOne');expected.work.remaining=String(500-E-2);expected.work.spent=String(19+E+2);
 assert.deepEqual(result.financialPost,expected);assert.deepEqual(result.post,{paid:String(payment)});assert.equal(result.workRemaining,expected.work.remaining);
 assert.deepEqual(result.effects,[{kind:'Transfer',id:'PayOne',from:'Payer',to:'Lender',asset:'Cash',amount:String(payment)},{kind:'Repayment',allocationId:'AllocOne',transferId:'PayOne',obligationId:'Loan',payer:'Payer',creditor:'Lender',denomination:'DebtUnit',settlementAsset:'Cash',nominalAmount:String(payment),settlementAmount:String(payment),principalDischarged:String(dischargedPrincipal),accruedDischarged:String(dischargedInterest),remainingOutstanding:expected.obligations[0].outstanding}]);
 assert.equal(JSON.stringify(st),before);count++;
 const repeat=snapshot(first,second,result.workRemaining,String(payment));reject(source,repeat,result.financialPost);
 const fresh=source.replaceAll('PayOne','PayTwo').replaceAll('AllocOne','AllocTwo');const continued=evaluate(fresh,repeat,result.financialPost);assert.equal(continued.status,'FundedExpressionPrepared',JSON.stringify(continued));assert.equal(continued.financialPost.allowances[0].spent,String(5+2*payment));assert.equal(continued.financialPost.usedTransferIds.length,3);assert.equal(continued.financialPost.work.closureReserve,'16');count++;
 const exact=state();exact.work.remaining=String(E+2);const exactResult=evaluate(source,snapshot(first,second,String(E+2)),exact);assert.equal(exactResult.status,'FundedExpressionPrepared',JSON.stringify(exactResult));assert.equal(exactResult.workRemaining,'0');assert.equal(exactResult.financialPost.work.closureReserve,'16');count++;
 exact.work.remaining=String(E+1);reject(source,snapshot(first,second,String(E+1)),exact);
}
reject(source.replace('ensures post.paid == pre.paid + quanta(cash);','ensures false;'),snapshot(),state());
reject(source,snapshot('-11','1'),state());
const noFunds=state();noFunds.allowances[0].remaining='29';reject(source,snapshot(),noFunds);
const mismatch=state();mismatch.obligations[0].denomination='OtherDebt';reject(source,snapshot(),mismatch);
reject(source,snapshot('10','20','501'),state());
const wrongAsset=source.replace('settlementAsset: "Cash"','settlementAsset: "Other"');reject(wrongAsset,snapshot(),state());
const missingTransfer=source.replace(/ emit Transfer[^\n]+\n/,'');reject(missingTransfer,snapshot(),state());
const overflow=state();overflow.balances[1].amount=String(2n**128n-1n);reject(source,snapshot(),overflow);
const forged=structuredClone(schema);forged.recordTypes.RepayFields.nominalAmount=['UInt128'];reject(source,snapshot(),state(),createFundedFinancialExpressionSourceV1(canonical(forged)));
console.log(JSON.stringify({passed:count,scope:'Independent full-financial-result, continuation, work-boundary, and rejection probes',actualModels:{author:'separately recorded Grok receipt',probeAuthor:'root GPT-6 Astra'}},null,2));
