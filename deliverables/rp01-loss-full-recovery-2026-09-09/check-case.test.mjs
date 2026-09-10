import {test} from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {checkCase} from './check-case.mjs';
const original=JSON.parse(readFileSync(new URL('./case.json',import.meta.url)));
function run(c){const prior=structuredClone(c);try{return checkCase(c);}finally{assert.deepEqual(c,prior,'success/rejection must leave all inputs unchanged');}}
test('finite sponsor cash funds retained discharged debt with full historical loss reconciliation',()=>{
 const r=run(original); assert.equal(r.status,'PASS');
 assert.equal(r.final.duties.length,1);assert.equal(r.final.duties[0].status,'Discharged');assert.equal(r.final.duties[0].outstanding,'0');
 assert.deepEqual(r.final.assets.Cash,{Pool:'1090',Borrower:'45',Buyer:'0',FeeCollector:'15',Custodian:'0',Sponsor:'0'});
 assert.deepEqual(r.final.accounts,{grossReceivable:'0',impairmentAllowance:'0',carryingReceivable:'0',nav:'1090',sharesOutstanding:'1000',impairmentExpense:'600',feeExpense:'10',recoveryGain:'600',netLoss:'10'});
 assert.deepEqual(r.final.claims.map(c=>[c.bookValue,c.allocatedNetLoss]),[['654','6'],['436','4']]);
 assert.deepEqual(r.final.history.lossAllocations.map(a=>a.netLoss),['600','610','460','10']);
 assert.equal(r.final.history.fundedNominalRecovery,'1000');assert.equal(r.final.authority.BorrowerGrant.remaining,'0');
 assert.equal(r.final.work.remaining,'0');assert.equal(r.final.work.spent,'13');assert.equal(r.final.work.closureReserve,'2');
});
const controls=[
 ['unfunded discharge',c=>c.step.events.shift(),'FUNDING'],
 ['underfunded discharge',c=>c.step.events[0].amount='449','FUNDING'],
 ['overdraw finite sponsor',c=>c.step.events[0].amount='451','BALANCE'],
 ['borrower cannot fund 450',c=>{c.step.events[0].from='Borrower';c.step.events[0].actor='Borrower';c.step.events[0].grant='BorrowerGrant';},'AUTHORITY'],
 ['old repayment grant exhausted',c=>c.step.events[1].grant='RecoveryGrant','AUTHORITY'],
 ['old reversal grant exhausted',c=>c.step.events[2].grant='ReverseGrant','AUTHORITY'],
 ['wrong transfer recipient',c=>c.step.events[0].to='FeeCollector','AUTHORITY'],
 ['collateral is not cash',c=>c.step.events[0].asset='Collateral','AUTHORITY'],
 ['wrong repayment asset grant',c=>c.freshGrants.Recovery450.asset='Collateral','FRESH_AUTHORITY'],
 ['wrong grant issuer',c=>c.freshGrants.Sponsor450.issuer='Borrower','FRESH_AUTHORITY'],
 ['missing fresh grant',c=>delete c.freshGrants.Recovery450,'FRESH_AUTHORITY'],
 ['grant reset name collision',c=>{c.freshGrants.BorrowerGrant=c.freshGrants.Sponsor450;delete c.freshGrants.Sponsor450;},'FRESH_AUTHORITY'],
 ['stale authority',c=>c.step.now='131','AUTHORITY'],
 ['grant not yet issued',c=>c.step.now='120','AUTHORITY'],
 ['wrong predecessor authorization',c=>c.freshGrants.Sponsor450.predecessorCaseSha256='0'.repeat(64),'FRESH_AUTHORITY'],
 ['old transfer reused',c=>c.step.events[0].id='LateCash','REPLAY'],
 ['old allocation reused',c=>c.step.events[1].allocationId='LateAllocation','REPLAY'],
 ['prior transfer cannot fund again',c=>c.step.events[1].transferId='LateCash','FUNDING'],
 ['unfunded reversal',c=>c.step.events[2].allocationId='Absent','REVERSAL_FUNDING'],
 ['over-reversal',c=>c.step.events[2].amount='451','REVERSAL_FUNDING'],
 ['reversal before repayment',c=>[c.step.events[1],c.step.events[2]]=[c.step.events[2],c.step.events[1]],'REVERSAL_FUNDING'],
 ['unfunded explicit discharge effect',c=>c.step.events[0].kind='Discharge','UNKNOWN_EFFECT'],
 ['deleted terminal duty',c=>c.step.after.duties=[],'RETAIN_RECORDS'],
 ['false terminal status',c=>c.step.after.duties[0].status='Defaulted','POST_STATE'],
 ['erase historical expense',c=>c.step.after.accounts.impairmentExpense='0','POST_STATE'],
 ['erase historical loss snapshot',c=>c.step.after.history.lossAllocations.splice(1,1),'POST_STATE'],
 ['retain impairment after payment',c=>c.step.after.accounts.impairmentAllowance='450','POST_STATE'],
 ['erase remaining fee loss',c=>c.step.after.claims[0].allocatedNetLoss='0','POST_STATE'],
 ['recharge old borrower grant',c=>c.step.after.authority.BorrowerGrant.remaining='155','POST_STATE'],
 ['wrong net effect',c=>c.step.actorEffects[0].netChange='449','ACTOR_EFFECTS'],
 ['no finite new work',c=>c.workSource.contribution='0','WORK_SOURCE'],
 ['source cannot supply extra work',c=>c.workSource.initial='0','WORK_SOURCE'],
 ['borrow closure reserve',c=>c.initial.work.closureReserve='1','PRE_STATE'],
 ['too small lifetime budget',c=>c.bounds.ordinaryLifetime='12','CAPACITY'],
 ['too many events',c=>c.step.events.push(structuredClone(c.step.events[2])),'CAPACITY'],
 ['negative transfer amount',c=>c.step.events[0].amount='-1','UINT128'],
 ['noncanonical integer',c=>c.step.events[0].amount='0450','UINT128'],
 ['overflow transfer amount',c=>c.step.events[0].amount=(1n<<128n).toString(),'UINT128'],
 ['zero transfer',c=>c.step.events[0].amount='0','ZERO_AMOUNT'],
 ['unbacked sponsor input',c=>c.fundingSource.balance='449','FUNDING_SOURCE'],
 ['extra unaccounted asset row',c=>c.initial.assets.Cash.Hidden='1','PRE_STATE'],
 ['claim language acceptance',c=>c.languageImplementation=true,'SCOPE'],
 ['change prefix pin',c=>c.prefix.caseSha256='0'.repeat(64),'SOURCE_PIN'],
 ['extend policy implicitly',c=>c.policy.additionalFee='1','POLICY'],
];
for(const [name,mutate,code] of controls)test(name,()=>{const c=structuredClone(original);mutate(c);assert.throws(()=>run(c),e=>e.message===code,code);});
test('authority is valid exactly at inclusive expiry',()=>{const c=structuredClone(original);c.step.now='130';assert.equal(run(c).status,'PASS');});
test('changing expected state cannot select a different computational result',()=>{const c=structuredClone(original);c.step.after.assets.Cash.Pool='999';assert.throws(()=>run(c),/POST_STATE/);});
