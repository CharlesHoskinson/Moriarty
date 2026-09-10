import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {checkCase} from '../rp01-loss-full-recovery-2026-09-09/check-case.mjs';
const read=p=>JSON.parse(readFileSync(new URL(p,import.meta.url)));
const c=read('../rp01-loss-full-recovery-2026-09-09/case.json');
const p=read('../rp01-loss-allocation-2026-09-09/case.json');
const results=[];
function call(x){const before=structuredClone(x);try{return checkCase(x);}finally{assert.deepEqual(x,before);}}
function reject(name,mutate,code){const x=structuredClone(c);mutate(x);assert.throws(()=>call(x),e=>e.message===code,`${name}: expected ${code}`);results.push({name,result:'REJECTED',code,inputUnchanged:true});}
function accept(name,mutate=()=>{}){const x=structuredClone(c);mutate(x);const r=call(x);assert.equal(r.status,'PASS');results.push({name,result:'PASS',inputUnchanged:true});return r;}

// Independent accounting fold starts only from prefix initial facts and events.
// It neither imports a replay helper nor reads any expected state to compute values.
const cash=Object.fromEntries(Object.entries(p.initial.assets.Cash).map(([k,v])=>[k,BigInt(v)]));
const collateral=Object.fromEntries(Object.entries(p.initial.assets.Collateral).map(([k,v])=>[k,BigInt(v)]));
let debt=BigInt(p.initial.duties[0].principal),expense=0n,fees=0n,gain=0n,funded=0n,work=0n;
const losses=[],values=[],prefixGrants=structuredClone(p.initial.authority);
for(const step of p.steps){
 for(const e of step.events){
  const amount=e.kind==='MarkDefault'?1n:BigInt(e.amount);work++;
  const g=prefixGrants[e.grant];g.remaining=String(BigInt(g.remaining)-amount);g.spent=String(BigInt(g.spent)+amount);
  if(e.kind==='Transfer'){const balances=e.asset==='Cash'?cash:collateral;balances[e.from]-=amount;balances[e.to]+=amount;if(e.purpose==='LiquidationFee')fees+=amount;}
  if(e.kind==='Repay'){debt-=amount;funded+=amount;}
  if(e.kind==='Impair')expense+=amount;
  if(e.kind==='ReverseImpairment')gain+=amount;
 }
 losses.push(String(expense+fees-gain));values.push(String(cash.Pool+debt-(expense-gain)));
}
assert.equal(debt,450n);assert.equal(cash.Pool,640n);assert.equal(cash.Borrower,45n);assert.equal(work,10n);
cash.Sponsor=450n;collateral.Sponsor=0n;
const totalCash=Object.values(cash).reduce((a,b)=>a+b,0n);assert.equal(totalCash,1150n);
cash.Sponsor-=450n;cash.Pool+=450n;debt-=450n;funded+=450n;gain+=450n;work+=3n;
const nav=cash.Pool+debt-(expense-gain),loss=expense+fees-gain;
losses.push(String(loss));values.push(String(nav));
const r=accept('baseline agrees with independent accounting fold');
assert.deepEqual(r.final.assets.Cash,Object.fromEntries(Object.entries(cash).map(([k,v])=>[k,String(v)])));
assert.deepEqual(r.final.assets.Collateral,Object.fromEntries(Object.entries(collateral).map(([k,v])=>[k,String(v)])));
assert.deepEqual(r.final.duties,[{...p.initial.duties[0],principal:'0',outstanding:'0',collateralUnits:'0',status:'Discharged'}]);
assert.deepEqual(r.final.accounts,{grossReceivable:String(debt),impairmentAllowance:String(expense-gain),carryingReceivable:'0',nav:String(nav),sharesOutstanding:'1000',impairmentExpense:String(expense),feeExpense:String(fees),recoveryGain:String(gain),netLoss:String(loss)});
assert.deepEqual(r.final.claims,[600n,400n].map((shares,i)=>({holder:['HolderA','HolderB'][i],shares:String(shares),bookValue:String(nav*shares/1000n),allocatedNetLoss:String(loss*shares/1000n)})));
assert.deepEqual(r.final.history.lossAllocations.map(x=>x.netLoss),losses);
assert.deepEqual(r.final.history.lossAllocations.map(x=>String(x.claims.reduce((a,b)=>a+BigInt(b.bookValue),0n))),values);
assert.equal(r.final.history.fundedNominalRecovery,String(funded));
for(const [id,g] of Object.entries(prefixGrants))assert.deepEqual(r.final.authority[id],g);
assert.equal(work,13n);assert.equal(12n+2n+1n,BigInt(r.final.work.spent)+BigInt(r.final.work.remaining)+BigInt(r.final.work.closureReserve));
assert.deepEqual(r.final.work.resourceSource,{owner:'RecoveryWorkSponsor',initial:'1',remaining:'0',spent:'1'});
assert.equal(r.languageAcceptance,false);assert.equal(r.networkAcceptance,false);assert.equal(r.protocolConformance,false);assert.equal(r.independentlyAccepted,false);

for(const id of c.initial.history.usedTransferIds)reject(`each prior transfer cannot replay: ${id}`,x=>x.step.events[0].id=id,'REPLAY');
for(const id of c.initial.history.usedAllocationIds)reject(`each prior allocation cannot replay: ${id}`,x=>x.step.events[1].allocationId=id,'REPLAY');
for(const id of ['__proto__','constructor','toString','missing'])reject(`spoofed unexecuted funding lookup: ${id}`,x=>x.step.events[1].transferId=id,'FUNDING');
for(const id of ['__proto__','constructor','toString','missing'])reject(`spoofed reversal allocation lookup: ${id}`,x=>x.step.events[2].allocationId=id,'REVERSAL_FUNDING');
reject('hidden event authority grant',x=>x.step.events[0].grant='constructor','AUTHORITY');
reject('hidden transfer refund field',x=>x.step.events[0].refund='450','UNKNOWN_EFFECT');
reject('hidden repayment forgiveness field',x=>x.step.events[1].forgive='450','UNKNOWN_EFFECT');
reject('consistent pre/post invented sponsor balance',x=>{x.initial.assets.Cash.Sponsor='900';x.step.after.assets.Cash.Sponsor='450';},'PRE_STATE');
reject('consistent pre/post erased fee collector funds',x=>{x.initial.assets.Cash.FeeCollector='0';x.step.after.assets.Cash.FeeCollector='0';},'PRE_STATE');
reject('expected states cannot erase prior spending',x=>{x.initial.authority.BorrowerGrant.spent='0';x.step.after.authority.BorrowerGrant.spent='0';},'PRE_STATE');
reject('expected states cannot erase old transfer identity',x=>{x.initial.history.usedTransferIds.shift();x.step.after.history.usedTransferIds.shift();},'PRE_STATE');
reject('invent additional hidden grant consistently',x=>{x.initial.authority.Hidden=structuredClone(x.initial.authority.Sponsor450);x.step.after.authority.Hidden=structuredClone(x.initial.authority.Hidden);},'PRE_STATE');
reject('sponsor subrogation is outside fixed policy',x=>x.policy.contribution='PaymentWithSubrogation','POLICY');
reject('new sponsor successor liability',x=>x.step.after.duties.push({...x.step.after.duties[0],id:'SponsorClaim',creditor:'Sponsor',principal:'450',outstanding:'450',status:'Performing'}),'RETAIN_RECORDS');
reject('recourse policy cannot mutate at discharge',x=>x.step.after.duties[0].legalRecourse='ActiveClaim450','POST_STATE');
reject('continuation cannot retain unpaid claim',x=>x.step.after.continuation.status='Open','POST_STATE');
reject('nonzero discharged duty cannot pass expected state',x=>{x.step.after.duties[0].principal='1';x.step.after.duties[0].outstanding='1';},'POST_STATE');
reject('partial 449 payment hits exact share rounding requirement',x=>x.step.events.forEach(e=>e.amount='449'),'SHARE_ROUNDING');
reject('partial 445 payment cannot masquerade as full',x=>x.step.events.forEach(e=>e.amount='445'),'FULL_RECOVERY_REQUIRED');
reject('full cash without impairment reversal',x=>{x.step.events.pop();x.step.workCost='2';},'UINT128');
reject('reserve converted to ordinary work in expected states',x=>{x.initial.work.remaining='4';x.initial.work.closureReserve='1';x.step.after.work.remaining='1';x.step.after.work.closureReserve='1';},'PRE_STATE');
reject('external work counted twice',x=>x.workSource.contribution='2','WORK_SOURCE');
reject('external work source changed owner',x=>x.workSource.owner='Servicer','WORK_SOURCE');
reject('unspent external work after contribution',x=>x.step.after.work.resourceSource.remaining='1','POST_STATE');
reject('undercharged work count',x=>x.step.workCost='2','WORK_COST');
reject('same grant fresh predecessor spoof',x=>{x.freshGrants.Recovery450.predecessorCaseSha256='f'.repeat(64);x.initial.authority.Recovery450=structuredClone(x.freshGrants.Recovery450);},'FRESH_AUTHORITY');
for(const k of ['Sponsor450','Recovery450','Reverse450'])reject(`fresh grant duty bound: ${k}`,x=>x.freshGrants[k].dutyId='Loan2','FRESH_AUTHORITY');
for(const k of ['usedTransfers','usedAllocations','lossSnapshots'])reject(`history cannot exceed reduced ${k} bound`,x=>x.bounds[k]=String(BigInt(x.bounds[k])-1n),'CAPACITY');
reject('event array cannot widen capacity',x=>x.bounds.eventsPerStep='4','CAPACITY');
reject('source composition balance is canonical finite supply',x=>x.fundingSource.balance='0450','FUNDING_SOURCE');
reject('transfer identifier length bounded',x=>x.step.events[0].id='a'.repeat(65),'REPLAY');
reject('empty allocation identifier',x=>x.step.events[1].allocationId='','REPLAY');
reject('event amount must be string',x=>x.step.events[0].amount=450,'UINT128');
reject('false protocol claim rejected',x=>x.protocolConformance=true,'SCOPE');
reject('false network claim rejected',x=>x.networkAcceptance=true,'SCOPE');
accept('issued-at inclusive boundary',x=>x.step.now='121');
accept('expiry inclusive boundary',x=>x.step.now='130');
accept('Map permits genuinely fresh prototype-looking IDs without spoofing',x=>{x.step.events[0].id='__proto__';x.step.events[1].transferId='__proto__';x.step.events[1].allocationId='constructor';x.step.events[2].allocationId='constructor';x.step.after.history.usedTransferIds[5]='__proto__';x.step.after.history.usedAllocationIds[2]='constructor';});
console.log(JSON.stringify({reviewer:'fresh independent GPT-6 Astra',scope:'callable fixed JSON design checker; no compiler, wallet, network or proof work',status:'PASS',node:process.version,counts:{total:results.length,accepted:results.filter(x=>x.result==='PASS').length,rejected:results.filter(x=>x.result==='REJECTED').length},independentArithmetic:{cash:String(totalCash),collateral:'4',funded:String(funded),debt:String(debt),nav:String(nav),loss:String(loss),lossHistory:losses,navHistory:values,ordinarySpent:String(work),totalResourceUniverse:'15'},results},null,2));
