import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {isDeepStrictEqual} from 'node:util';
import {fileURLToPath} from 'node:url';
import {checkCase as checkPartial} from '../rp01-loss-allocation-2026-09-09/check-case.mjs';
const ROOT=new URL('../../',import.meta.url);
const PREFIX='deliverables/rp01-loss-allocation-2026-09-09/case.json';
const CASE_HASH='74e3a5506a775c260ed5471a273343bb0a8b840c3f628a7a83b4ca818a2edd93';
const CHECKER_HASH='38c985ce97cb53f4d86fbeb7148f7b6dde3e932b50ad05d98f79b0af169af16b';
const MAX=(1n<<128n)-1n;
const actors=['Pool','Borrower','Buyer','FeeCollector','Custodian','Sponsor'];
const assets=['Cash','Collateral'];
const POLICY={version:'GenericLossV1',extension:'GenericFullRecoveryV1',contribution:'NonrefundableThirdPartyPaymentWithoutSubrogation',additionalCash:'450',additionalFee:'0',additionalOrdinaryWork:'1',debtCreationLimit:'0',forgivenessLimit:'0'};
const LIMITS={uint128Max:MAX.toString(),extensionSteps:'1',eventsPerStep:'3',actors:'6',assetRows:'12',duties:'1',claims:'2',grants:'11',usedTransfers:'6',usedAllocations:'3',lossSnapshots:'4',ordinaryLifetime:'13',closureReserve:'2',sidecarUtf8Bytes:'65536'};
function need(ok,code){if(!ok)throw Error(code);}
function exact(a,b,code){need(isDeepStrictEqual(a,b),code);}
function n(x){need(typeof x==='string'&&/^(0|[1-9][0-9]*)$/.test(x)&&x.length<=39,'UINT128');const v=BigInt(x);need(v<=MAX,'UINT128');return v;}
function put(o,k,v){need(v>=0n&&v<=MAX,'UINT128');o[k]=v.toString();}
function add(a,b){const v=a+b;need(v<=MAX,'UINT128');return v;}
function hash(raw){return createHash('sha256').update(raw).digest('hex');}
function recalculate(s,p){
 const a=s.accounts,d=s.duties[0];put(a,'grossReceivable',n(d.outstanding));
 put(a,'carryingReceivable',n(a.grossReceivable)-n(a.impairmentAllowance));
 put(a,'nav',add(n(s.assets.Cash.Pool),n(a.carryingReceivable)));
 put(a,'netLoss',add(n(a.impairmentExpense),n(a.feeExpense))-n(a.recoveryGain));
 for(const claim of s.claims){
  const supply=n(a.sharesOutstanding),value=n(a.nav)*n(claim.shares),loss=n(a.netLoss)*n(claim.shares);
  need(supply>0n&&value<=MAX&&loss<=MAX&&value%supply===0n&&loss%supply===0n,'SHARE_ROUNDING');
  put(claim,'bookValue',value/supply);put(claim,'allocatedNetLoss',loss/supply);
 }
 need(n(p.initialPoolNav)-n(a.nav)===n(a.netLoss),'LOSS_CONSERVATION');
}
function lossSnapshot(s,id){const a=s.accounts;return {stepId:id,impairmentExpense:a.impairmentExpense,feeExpense:a.feeExpense,recoveryGain:a.recoveryGain,netLoss:a.netLoss,claims:structuredClone(s.claims)};}
// The reviewed checker establishes the fixed prefix's admissibility. This replay
// derives its successor from initial state and events, never from before/after
// expected snapshots. Only checkPartial reads those snapshots for comparison.
function replayPrefix(p){
 checkPartial(p);
 const s=structuredClone(p.initial),history=[];
 for(const step of p.steps){
  const d=s.duties[0],a=s.accounts;
  s.history.usedObservationIds.push(...step.observations);
  for(const e of step.events){
   const quantity=e.kind==='MarkDefault'?1n:n(e.amount),g=s.authority[e.grant];
   switch(e.kind){
    case 'MarkDefault':d.status='Defaulted';break;
    case 'Impair':put(a,'impairmentAllowance',add(n(a.impairmentAllowance),quantity));put(a,'impairmentExpense',add(n(a.impairmentExpense),quantity));break;
    case 'Transfer':
     put(s.assets[e.asset],e.from,n(s.assets[e.asset][e.from])-quantity);put(s.assets[e.asset],e.to,add(n(s.assets[e.asset][e.to]),quantity));
     if(e.asset==='Collateral')put(d,'collateralUnits',n(d.collateralUnits)-quantity);
     if(e.purpose==='LiquidationFee')put(a,'feeExpense',add(n(a.feeExpense),quantity));
     s.history.usedTransferIds.push(e.id);break;
    case 'Repay':put(d,'principal',n(d.principal)-quantity);put(d,'outstanding',n(d.outstanding)-quantity);put(s.history,'fundedNominalRecovery',add(n(s.history.fundedNominalRecovery),quantity));s.history.usedAllocationIds.push(e.allocationId);break;
    case 'ReverseImpairment':put(a,'impairmentAllowance',n(a.impairmentAllowance)-quantity);put(a,'recoveryGain',add(n(a.recoveryGain),quantity));break;
    default:throw Error('PREFIX_EFFECT');
   }
   put(g,'remaining',n(g.remaining)-quantity);put(g,'spent',add(n(g.spent),quantity));
  }
  put(s.work,'remaining',n(s.work.remaining)-BigInt(step.events.length));put(s.work,'spent',add(n(s.work.spent),BigInt(step.events.length)));
  recalculate(s,p.policy);history.push(lossSnapshot(s,step.id));
 }
 s.history.lossAllocations=history;return s;
}
function expectedGrant(issuer,scope){return {issuer,holder:scope==='Transfer'?'Sponsor':'Servicer',scope,initial:'450',remaining:'450',spent:'0',asset:scope==='ReverseImpairment'?null:'Cash',recipient:scope==='ReverseImpairment'?null:'Pool',dutyId:'Loan1',version:'GenericLossV1',issuedAt:'121',expiresAt:'130',predecessorCaseSha256:CASE_HASH};}
function totals(s){return Object.fromEntries(assets.map(asset=>[asset,actors.reduce((acc,actor)=>add(acc,n(s.assets[asset][actor])),0n).toString()]));}
function invariants(s,p){
 need(s.duties.length===1&&s.claims.length===2,'RETAIN_RECORDS');
 const d=s.duties[0],a=s.accounts;
 exact(Object.keys(s.assets).sort(),[...assets].sort(),'ASSET_SHAPE');for(const asset of assets)exact(Object.keys(s.assets[asset]).sort(),[...actors].sort(),'ASSET_SHAPE');
 need(d.id==='Loan1'&&d.debtor==='Borrower'&&d.creditor==='Pool'&&d.denomination==='Cash'&&d.legalRecourse==='Retained','DUTY_IDENTITY');
 need(n(d.principal)===n(d.outstanding)&&n(d.accrued)===0n&&n(d.collateralUnits)===0n,'DUTY');
 need(n(d.outstanding)+n(s.history.fundedNominalRecovery)===n(p.policy.initialDebt)&&s.history.forgivenNominal==='0'&&s.history.nominalCreated==='0','NO_DEBT_ERASURE');
 need(d.status===(n(d.outstanding)===0n?'Discharged':'Defaulted'),'DUTY_STATUS');
 need(s.continuation.status===(n(d.outstanding)===0n?'Discharged':'Open')&&s.continuation.owner==='Servicer'&&s.continuation.debtId===d.id&&s.continuation.nominalCreationRemaining==='0'&&s.continuation.forgivenessRemaining==='0'&&s.continuation.policyVersion===p.policy.version,'CONTINUATION');
 need(n(a.impairmentAllowance)===n(a.impairmentExpense)-n(a.recoveryGain)&&n(a.impairmentAllowance)<=n(d.outstanding),'IMPAIRMENT_ACCOUNT');
 need(s.claims[0].holder==='HolderA'&&s.claims[0].shares==='600'&&s.claims[1].holder==='HolderB'&&s.claims[1].shares==='400'&&a.sharesOutstanding==='1000','SHARE_SUPPLY');
 for(const grant of Object.values(s.authority))need(n(grant.remaining)+n(grant.spent)===n(grant.initial),'AUTHORITY');
 need(n(s.work.remaining)+n(s.work.spent)===13n&&s.work.additionalAllocated==='1'&&s.work.closureReserve==='2'&&s.work.owner==='Servicer','WORK');
 exact(s.work.resourceSource,{owner:'RecoveryWorkSponsor',initial:'1',remaining:'0',spent:'1'},'WORK_SOURCE');
 for(const key of ['usedTransferIds','usedAllocationIds','usedObservationIds'])need(new Set(s.history[key]).size===s.history[key].length,'REPLAY');
 const calculated=structuredClone(s);recalculate(calculated,p.policy);exact(s.accounts,calculated.accounts,'ACCOUNTS');exact(s.claims,calculated.claims,'CLAIM_LOSS_ALLOCATION');
}
function effects(events){return assets.flatMap(asset=>actors.map(actor=>{
 const outgoing=events.filter(e=>e.kind==='Transfer'&&e.asset===asset&&e.from===actor).reduce((a,e)=>add(a,n(e.amount)),0n);
 const incoming=events.filter(e=>e.kind==='Transfer'&&e.asset===asset&&e.to===actor).reduce((a,e)=>add(a,n(e.amount)),0n);
 return {actor,asset,grossDebit:outgoing.toString(),grossCredit:incoming.toString(),feesPaid:'0',refund:'0',netChange:(incoming-outgoing).toString()};
}));}
function leaves(v,p=''){if(Array.isArray(v))return v.length?v.flatMap((x,i)=>leaves(x,`${p}/${i}`)):[[p,[]]];if(v!==null&&typeof v==='object')return Object.entries(v).flatMap(([k,x])=>leaves(x,`${p}/${k}`));return [[p,v]];}
export function checkCase(c){
 need(c.schema==='moriarty.fixed-full-loss-recovery-challenge/1'&&c.id==='RP01-LOSS-FULL-RECOVERY'&&c.protocolConformance===false&&c.languageImplementation===false&&c.networkAcceptance===false,'SCOPE');
 exact(c.policy,POLICY,'POLICY');exact(Object.keys(c.bounds).sort(),Object.keys(LIMITS).sort(),'CAPACITY');
 for(const [key,value] of Object.entries(LIMITS))need(n(c.bounds[key])<=n(value),'CAPACITY');
 need(Buffer.byteLength(JSON.stringify(c))<=Number(n(c.bounds.sidecarUtf8Bytes)),'CAPACITY');
 need(n(c.bounds.uint128Max)===MAX&&n(c.bounds.ordinaryLifetime)===13n&&n(c.bounds.closureReserve)===2n,'CAPACITY');
 exact(c.prefix,{casePath:PREFIX,caseSha256:CASE_HASH,checkerSha256:CHECKER_HASH},'SOURCE_PIN');
 const raw=readFileSync(new URL(PREFIX,ROOT));need(hash(raw)===CASE_HASH&&hash(readFileSync(new URL(PREFIX.replace('case.json','check-case.mjs'),ROOT)))===CHECKER_HASH,'SOURCE_PIN');
 const prefix=JSON.parse(raw);let s=replayPrefix(prefix);
 exact(c.fundingSource,{owner:'Sponsor',asset:'Cash',balance:'450',collateralBalance:'0',kind:'ExternalPreexistingCashAccount'},'FUNDING_SOURCE');
 exact(c.workSource,{owner:'RecoveryWorkSponsor',initial:'1',contribution:'1'},'WORK_SOURCE');
 exact(c.freshGrants,{Sponsor450:expectedGrant('Sponsor','Transfer'),Recovery450:expectedGrant('Pool','Repay'),Reverse450:expectedGrant('Governor','ReverseImpairment')},'FRESH_AUTHORITY');
 s.assets.Cash.Sponsor=c.fundingSource.balance;s.assets.Collateral.Sponsor=c.fundingSource.collateralBalance;
 for(const [id,g] of Object.entries(c.freshGrants)){need(!Object.hasOwn(s.authority,id),'FRESH_AUTHORITY');s.authority[id]=structuredClone(g);}
 put(s.work,'remaining',add(n(s.work.remaining),n(c.workSource.contribution)));s.work.additionalAllocated=c.workSource.contribution;
 s.work.resourceSource={owner:c.workSource.owner,initial:c.workSource.initial,remaining:(n(c.workSource.initial)-n(c.workSource.contribution)).toString(),spent:c.workSource.contribution};
 invariants(s,prefix);exact(c.initial,s,'PRE_STATE');const before=structuredClone(s),initialTotals=totals(s);
 need(n(c.bounds.extensionSteps)>=1n&&n(c.bounds.actors)>=BigInt(actors.length)&&n(c.bounds.assetRows)>=12n&&n(c.bounds.duties)>=1n&&n(c.bounds.claims)>=2n&&n(c.bounds.grants)>=11n,'CAPACITY');
 const step=c.step;need(step.id==='full-funded-recovery'&&Array.isArray(step.events)&&step.events.length>0&&BigInt(step.events.length)<=n(c.bounds.eventsPerStep),'CAPACITY');
 need(n(s.work.remaining)>=BigInt(step.events.length),'WORK');const now=n(step.now),funding=new Map(),allocations=new Map(),reversed=new Set();
 const d=s.duties[0],a=s.accounts;
 for(const e of step.events){
  need(['Transfer','Repay','ReverseImpairment'].includes(e.kind),'UNKNOWN_EFFECT');
  const g=s.authority[e.grant];need(g&&Object.hasOwn(c.freshGrants,e.grant)&&g.holder===e.actor&&g.scope===e.kind&&e.version===g.version&&n(g.issuedAt)<=now&&now<=n(g.expiresAt),'AUTHORITY');
  const amount=n(e.amount);need(amount>0n,'ZERO_AMOUNT');
  if(e.kind==='Transfer'){
   exact(Object.keys(e).sort(),['kind','grant','actor','version','id','asset','from','to','amount','purpose'].sort(),'UNKNOWN_EFFECT');
   need(e.asset===g.asset&&e.from===g.issuer&&e.actor===e.from&&e.to===g.recipient&&e.purpose==='ThirdPartyRecovery','AUTHORITY');
   need(typeof e.id==='string'&&e.id.length>0&&e.id.length<=64&&!s.history.usedTransferIds.includes(e.id),'REPLAY');
   need(n(s.assets[e.asset][e.from])>=amount,'BALANCE');
   put(s.assets[e.asset],e.from,n(s.assets[e.asset][e.from])-amount);put(s.assets[e.asset],e.to,add(n(s.assets[e.asset][e.to]),amount));
   s.history.usedTransferIds.push(e.id);funding.set(e.id,{asset:e.asset,recipient:e.to,remaining:amount});
  }
  if(e.kind==='Repay'){
   exact(Object.keys(e).sort(),['kind','grant','actor','version','allocationId','transferId','dutyId','amount'].sort(),'UNKNOWN_EFFECT');
   need(g.asset===d.denomination&&g.recipient===d.creditor&&e.dutyId===d.id,'AUTHORITY');
   const f=funding.get(e.transferId);need(f&&f.asset===d.denomination&&f.recipient===d.creditor&&f.remaining>=amount&&amount<=n(d.outstanding),'FUNDING');
   need(typeof e.allocationId==='string'&&e.allocationId.length>0&&e.allocationId.length<=64&&!s.history.usedAllocationIds.includes(e.allocationId),'REPLAY');
   f.remaining-=amount;put(d,'principal',n(d.principal)-amount);put(d,'outstanding',n(d.outstanding)-amount);put(s.history,'fundedNominalRecovery',add(n(s.history.fundedNominalRecovery),amount));
   s.history.usedAllocationIds.push(e.allocationId);allocations.set(e.allocationId,amount);if(n(d.outstanding)===0n){d.status='Discharged';s.continuation.status='Discharged';}
  }
  if(e.kind==='ReverseImpairment'){
   exact(Object.keys(e).sort(),['kind','grant','actor','version','dutyId','allocationId','amount'].sort(),'UNKNOWN_EFFECT');
   need(e.dutyId===d.id&&allocations.get(e.allocationId)===amount&&!reversed.has(e.allocationId)&&amount<=n(a.impairmentAllowance),'REVERSAL_FUNDING');
   put(a,'impairmentAllowance',n(a.impairmentAllowance)-amount);put(a,'recoveryGain',add(n(a.recoveryGain),amount));reversed.add(e.allocationId);
  }
  need(n(g.remaining)>=amount,'AUTHORITY');put(g,'remaining',n(g.remaining)-amount);put(g,'spent',add(n(g.spent),amount));
 }
 need(n(step.workCost)===BigInt(step.events.length),'WORK_COST');put(s.work,'remaining',n(s.work.remaining)-n(step.workCost));put(s.work,'spent',add(n(s.work.spent),n(step.workCost)));
 recalculate(s,prefix.policy);s.history.lossAllocations.push(lossSnapshot(s,step.id));invariants(s,prefix);exact(totals(s),initialTotals,'ASSET_CONSERVATION');
 need(s.duties[0].outstanding==='0'&&s.accounts.impairmentAllowance==='0','FULL_RECOVERY_REQUIRED');
 need(BigInt(s.history.usedTransferIds.length)<=n(c.bounds.usedTransfers)&&BigInt(s.history.usedAllocationIds.length)<=n(c.bounds.usedAllocations)&&BigInt(s.history.lossAllocations.length)<=n(c.bounds.lossSnapshots),'CAPACITY');
 need(Array.isArray(step.after.duties)&&step.after.duties.length===1,'RETAIN_RECORDS');exact(s,step.after,'POST_STATE');exact(effects(step.events),step.actorEffects,'ACTOR_EFFECTS');
 const beforeLeaves=Object.fromEntries(leaves(before)),afterLeaves=Object.fromEntries(leaves(s));
 const footprint={readPaths:Object.keys(beforeLeaves).sort(),writePaths:[...new Set([...Object.keys(beforeLeaves),...Object.keys(afterLeaves)])].filter(k=>!isDeepStrictEqual(beforeLeaves[k],afterLeaves[k])).sort()};
 return {status:'PASS',caseId:c.id,scope:'fixed full-recovery JSON design oracle only',final:s,actorEffects:effects(step.events),footprint,totals:initialTotals,languageAcceptance:false,networkAcceptance:false,protocolConformance:false,independentlyAccepted:false};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 try{const path=process.argv[2]??new URL('./case.json',import.meta.url);console.log(JSON.stringify(checkCase(JSON.parse(readFileSync(path))),null,2));}
 catch(error){console.error(error.message);process.exitCode=1;}
}
