// Independent public rejection-interface controls, authored by GPT-6.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {pathToFileURL} from 'node:url';
const root=process.argv[2];
const {prepareRepayment:run}=await import(pathToFileURL(`${root}/experiments/moriarty-language/src/successor/repayment.ts`));
const source=readFileSync(`${root}/experiments/moriarty-language/spec/successor/examples/funded-repayment.json`,'utf8');
const base=()=>JSON.parse(source);
const results=[];
function check(code,index,input){
 const actual=run(input);
 assert.deepEqual(actual,{status:'Rejected',code,actionIndex:index});
 assert.deepEqual(run(input),actual);
 results.push({code,actionIndex:index});
}
for(const [code,input] of [
 ['INPUT_NOT_STRING',null],['INPUT_UTF16_LENGTH',' '.repeat(65537)],
 ['INPUT_UTF8_LENGTH','é'.repeat(32769)],['INPUT_LONE_SURROGATE','\ud800'],
 ['INPUT_JSON','{'],['INPUT_COMPACT',source+'\n'],
 ['INPUT_ENCODING','['.repeat(15000)+'0'+']'.repeat(15000)]
])check(code,null,input);
for(const [code,index,mutate] of [
 ['SCHEMA',null,x=>delete x.state],
 ['UNKNOWN_FIELD',null,x=>x.extra=true],
 ['INVALID_IDENTIFIER',null,x=>x.actions[0].from='Payer\n'],
 ['INVALID_AMOUNT',null,x=>x.actions[0].amount='01'],
 ['CAPACITY',null,x=>x.state.usedTransferIds=Array.from({length:129},(_,i)=>`T${i}`)],
 ['INVARIANT',null,x=>x.state.obligations[0].outstanding='101'],
 ['DUPLICATE',null,x=>x.state.balances.push({...x.state.balances[0]})],
 ['UNKNOWN_ACTION',null,x=>x.actions[0].kind='Forgive'],
 ['INSUFFICIENT_WORK',null,x=>x.state.work.remaining='1'],
 ['ZERO_AMOUNT',0,x=>x.actions[0].amount='0'],
 ['SELF_TRANSFER',0,x=>x.actions[0].to=x.actions[0].from],
 ['DUPLICATE',0,x=>x.state.usedTransferIds=[x.actions[0].id]],
 ['MISSING_BALANCE',0,x=>x.state.balances=[]],
 ['INSUFFICIENT_BALANCE',0,x=>x.state.balances[0].amount='1'],
 ['MISSING_ALLOWANCE',0,x=>x.state.allowances=[]],
 ['INSUFFICIENT_ALLOWANCE',0,x=>x.state.allowances[0].remaining='1'],
 ['OVERFLOW',0,x=>x.state.balances.push({party:x.actions[0].to,asset:x.actions[0].asset,amount:'340282366920938463463374607431768211455'})],
 ['MISSING_OBLIGATION',1,x=>x.state.obligations=[]],
 ['NOT_OUTSTANDING',1,x=>Object.assign(x.state.obligations[0],{principal:'0',accrued:'0',outstanding:'0',status:'Settled'})],
 ['EXCEEDS_OUTSTANDING',1,x=>x.actions[1].nominalAmount='101'],
 ['TRANSFER_NOT_IN_STEP',0,x=>x.actions.shift()],
 ['TRANSFER_MISMATCH',1,x=>x.actions[1].payer='Other'],
 ['INSUFFICIENT_UNALLOCATED',1,x=>x.actions[0].amount='1'],
 ['INEXACT_CONVERSION',1,x=>Object.assign(x.state.obligations[0].conversion,{mantissa:'1',scale:'2',rounding:'none'})],
 ['DUST',1,x=>Object.assign(x.state.obligations[0].conversion,{mantissa:'1',scale:'2',rounding:'floor'})],
]){
 const x=base();
 // Make receiver-overflow control independent of whether the fixture includes a receiver.
 if(code==='OVERFLOW')x.state.balances=x.state.balances.filter(b=>b.party!==x.actions[0].to);
 mutate(x);check(code,index,JSON.stringify(x));
}
console.log(JSON.stringify({scope:'Stable public rejection codes and action indexes; no unreachable defensive-code coverage claim',cases:results.length,pass:true,results},null,2));
