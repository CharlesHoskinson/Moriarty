import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
import {createFundedFinancialExpressionSourceV1 as funded} from '/home/charl/Moriarty/.worktrees/expression-funded-repayment/experiments/moriarty-language/src/successor/funded-expression-source-v1.ts';
import {createFinancialExpressionSourceV1 as pure} from '/home/charl/Moriarty/.worktrees/expression-funded-repayment/experiments/moriarty-language/src/successor/financial-expression-source-v1.ts';
import {canonical} from '/home/charl/Moriarty/.worktrees/expression-funded-repayment/experiments/moriarty-language/src/successor/expression-wire-v1.ts';
const root='/home/charl/Moriarty/.worktrees/expression-funded-repayment/experiments/moriarty-language/';
const out='/home/charl/Moriarty/deliverables/expression-funded-repayment-2026-09-11/audit-01/';
const stem=root+'spec/successor/examples/expression-funded-payment';
const source=readFileSync(stem+'.mori','utf8'), schema=JSON.parse(readFileSync(stem+'.schema.json','utf8'));
const snapshot=JSON.parse(readFileSync(stem+'.snapshots.json','utf8')), state=JSON.parse(readFileSync(stem+'.state.json','utf8'));
const results=[]; const clone=x=>structuredClone(x);
function run(name,modify=()=>{},expected='Rejected',verify=()=>{}){const c={schema:clone(schema),snapshot:clone(snapshot),state:clone(state),source};modify(c);const result=funded(canonical(c.schema)).evaluate(c.source,canonical(c.snapshot),typeof c.state==='string'?c.state:JSON.stringify(c.state));assert.equal(result.status,expected,`${name}: ${JSON.stringify(result)}`);if(expected==='Rejected')for(const k of ['post','financialPost','effects','descriptors'])assert.equal(k in result,false,name);verify(result);results.push({name,status:result.status,code:result.code??null});return result;}
const base=run('computed 10+20 reaches both actions',()=>{},'FundedExpressionPrepared',r=>{assert.equal(r.effects[0].amount,'30');assert.equal(r.effects[1].nominalAmount,'30');assert.equal(r.financialPost.obligations[0].principal,'70');});
run('same source 4+3 pays interest first',c=>{c.snapshot.Args.first='4';c.snapshot.Args.second='3';Object.assign(c.state.obligations[0],{accrued:'10',outstanding:'110'});},'FundedExpressionPrepared',r=>{assert.equal(r.effects[0].amount,'7');assert.equal(r.effects[1].nominalAmount,'7');assert.equal(r.financialPost.obligations[0].principal,'100');assert.equal(r.financialPost.obligations[0].accrued,'3');});
const E=100n-BigInt(pure(canonical(schema)).evaluate(source,canonical(snapshot)).workRemaining),cost=E+2n;
run('exact E+N with uint128 total boundary and separate reserve',c=>{c.snapshot.workInitial=String(cost);c.state.work={remaining:String(cost),spent:String((1n<<128n)-1n-cost-16n),closureReserve:'16'};},'FundedExpressionPrepared',r=>{assert.equal(r.workRemaining,'0');assert.equal(r.financialPost.work.spent,String((1n<<128n)-17n));assert.equal(r.financialPost.work.closureReserve,'16');});
for(const n of [0n,E-1n,E,E+1n])run('insufficient remaining '+n,c=>{c.snapshot.workInitial=String(n);c.state.work.remaining=String(n);c.state.work.closureReserve='10000';});
run('carried total overflow cannot be normalized away',c=>{c.state.work.spent=String((1n<<128n)-100n);c.state.work.closureReserve='0';});
run('spent above uint128',c=>{c.state.work.spent=String(1n<<128n);});
run('work mismatch',c=>{c.snapshot.workInitial='99';});
run('work above expression ceiling',c=>{c.snapshot.workInitial='65537';c.state.work.remaining='65537';});
for(const [name,modify] of [
 ['unknown state key',c=>c.state.extra=true],
 ['unknown work key',c=>c.state.work.extra=true],
 ['numeric spent',c=>c.state.work.spent=0],
 ['noncanonical spent',c=>c.state.work.spent='00'],
 ['missing reserve',c=>delete c.state.work.closureReserve],
 ['missing obligations',c=>delete c.state.obligations],
 ['malformed obligation',c=>c.state.obligations=[null]],
 ['obligation invariant mismatch',c=>c.state.obligations[0].outstanding='99'],
 ['duplicate balance',c=>c.state.balances.push(clone(c.state.balances[0]))],
 ['allowance total overflow',c=>c.state.allowances[0].spent=String((1n<<128n)-1n)],
 ['denomination mismatch',c=>c.state.obligations[0].denomination='USD'],
 ['creditor mismatch',c=>c.state.obligations[0].creditor='Other'],
 ['asset mismatch',c=>c.state.obligations[0].settlementAsset='USD'],
 ['allocation tombstone reuse',c=>c.state.usedAllocationIds=['Alloc1']],
 ['transfer tombstone reuse',c=>c.state.usedTransferIds=['T1']],
 ['malformed JSON',c=>c.state='{'],
 ['nonrecord JSON',c=>c.state='[]'],
 ['deep JSON',c=>c.state='['.repeat(10000)+'0'+']'.repeat(10000)],
 ['negative computed nominal',c=>{c.snapshot.Args.first='-30';c.snapshot.Args.second='20';}],
 ['quantity add overflow',c=>{c.snapshot.Args.first=String((1n<<127n)-1n);c.snapshot.Args.second='1';}],
 ['ensure fails after effects',c=>c.source=c.source.replace('ensures post.paid == pre.paid + payment;','ensures false;')],
 ['financial schema field even unread',c=>{c.schema.fields.hidden={type:['UInt128'],writeClass:'financial'};c.snapshot.Pre.hidden='1';}],
 ['unsupported binding even unused',c=>c.schema.operations.Notice='TransferFields'],
 ['missing Transfer binding',c=>delete c.schema.operations.Transfer],
 ['extra transfer field',c=>c.schema.recordTypes.TransferFields.extra=['Text']],
 ['wrong nominal type',c=>c.schema.recordTypes.RepayFields.nominalAmount=['UInt128']],
 ['empty actions',c=>c.source='profile "moriarty-financial-expression-source/1"; agreement Test { action pay(first: Quantity<Units<Cash,1>,0>, second: Quantity<Units<Cash,1>,0>, transferId: Text, allocationId: Text) { next.paid = 1; } }'],
 ['unfunded converted nominal',c=>c.state.obligations[0].conversion.mantissa='2']
])run(name,modify);
run('carry full unrelated projection and second-step identities',c=>{c.state=clone(base.financialPost);c.snapshot.Pre=clone(base.post);c.snapshot.workInitial=c.state.work.remaining;c.snapshot.Args={allocationId:'Alloc2',transferId:'T2',first:'4',second:'3'};c.state.balances.push({party:'Other',asset:'USD',amount:'91'});c.state.allowances.push({party:'Other',asset:'USD',remaining:'2',spent:'89'});c.state.obligations.push({...clone(c.state.obligations[0]),id:'OtherDebt',denomination:'USD',conversion:{mantissa:'2',scale:'1',rounding:'ceil'}});},'FundedExpressionPrepared',r=>{assert.equal(r.financialPost.obligations[0].principal,'63');assert.equal(r.financialPost.obligations[1].principal,'70');assert.equal(r.financialPost.obligations[1].conversion.rounding,'ceil');assert.equal(r.financialPost.balances[2].amount,'91');assert.equal(r.financialPost.allowances[1].spent,'89');assert.deepEqual(r.financialPost.usedTransferIds,['T1','T2']);assert.deepEqual(r.financialPost.usedAllocationIds,['Alloc1','Alloc2']);assert.equal(r.financialPost.work.spent,String(cost*2n));});
for(const [name,st,expected] of [['cli normal',JSON.stringify(state),0],['cli malformed state','{}',1],['cli unknown state key',JSON.stringify({...state,extra:1}),1],['cli invalid UTF8',Buffer.from([255]),1]]){const f=out+name.replaceAll(' ','-')+'.json';writeFileSync(f,st);const r=spawnSync(process.execPath,[root+'src/cli.ts','simulate','--profile','moriarty-financial-expression-source/1','--schema',stem+'.schema.json','--snapshots',stem+'.snapshots.json','--repayment-state',f,stem+'.mori'],{encoding:'utf8',timeout:5000});assert.equal(r.error,undefined);assert.equal(r.status,expected,r.stderr);if(expected===0){assert.equal(r.stderr,'');assert.equal(JSON.parse(r.stdout).result.effects[1].nominalAmount,'30');}else{assert.equal(r.stdout,'');assert.ok(JSON.parse(r.stderr).status.endsWith('Rejected'));}results.push({name,exitCode:r.status});}
writeFileSync(out+'probes.json',JSON.stringify({expressionReductions:String(E),kernelActions:2,cases:results.length,results},null,2)+'\n');console.log(JSON.stringify({cases:results.length,expressionReductions:String(E),outcome:'PASS'}));
