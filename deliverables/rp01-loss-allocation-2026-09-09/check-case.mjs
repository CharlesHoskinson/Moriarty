import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {isDeepStrictEqual} from 'node:util';
import {fileURLToPath} from 'node:url';
const CASE=new URL('./case.json',import.meta.url),ROOT=new URL('../../',import.meta.url);

const MAX=(1n<<128n)-1n;
const actors=['Pool','Borrower','Buyer','FeeCollector','Custodian'],assets=['Cash','Collateral'];
function need(ok,code){if(!ok)throw Error(code);}
function n(x){need(typeof x==='string'&&/^(0|[1-9][0-9]*)$/.test(x),'UINT128');const v=BigInt(x);need(v<=MAX,'UINT128');return v;}
function put(x,key,v){need(v>=0n&&v<=MAX,'UINT128');x[key]=v.toString();}
function exact(a,b,code){need(isDeepStrictEqual(a,b),code);}
function leaves(v,p=''){
 if(Array.isArray(v))return v.length?v.flatMap((x,i)=>leaves(x,p+'/'+i)):[[p,[]]];
 if(v!==null&&typeof v==='object')return Object.entries(v).flatMap(([k,x])=>leaves(x,p+'/'+k));
 return [[p,v]];
}
function sum(xs){return xs.reduce((a,b)=>{const v=a+b;need(v<=MAX,'UINT128');return v;},0n);}
function total(s,asset){return sum(actors.map(p=>n(s.assets[asset][p])));}
function conserveAssets(s,totals){for(const asset of assets)need(total(s,asset)===totals[asset],'ASSET_CONSERVATION');}
function recalculate(s,policy){
 const d=s.duties[0],a=s.accounts;put(a,'grossReceivable',n(d.outstanding));
 put(a,'carryingReceivable',n(a.grossReceivable)-n(a.impairmentAllowance));
 put(a,'nav',n(s.assets.Cash.Pool)+n(a.carryingReceivable));
 put(a,'netLoss',n(a.impairmentExpense)+n(a.feeExpense)-n(a.recoveryGain));
 for(const c of s.claims){const product=n(a.nav)*n(c.shares),loss=n(a.netLoss)*n(c.shares),supply=n(a.sharesOutstanding);need(product<=MAX&&loss<=MAX&&supply>0n&&product%supply===0n&&loss%supply===0n,'SHARE_ROUNDING');put(c,'bookValue',product/supply);put(c,'allocatedNetLoss',loss/supply);}
 need(n(policy.initialPoolNav)-n(a.nav)===n(a.netLoss),'LOSS_CONSERVATION');
}
function invariants(s,c){
 const b=c.bounds;need(Object.keys(s.assets).length===2&&actors.every(p=>assets.every(a=>Object.hasOwn(s.assets[a],p))),'ASSET_SHAPE');
 need(assets.reduce((k,a)=>k+Object.keys(s.assets[a]).length,0)<=Number(n(b.assetRows))&&s.duties.length<=Number(n(b.duties))&&s.claims.length<=Number(n(b.claims))&&Object.keys(s.authority).length<=Number(n(b.grants)),'CAPACITY');
 need(s.duties.length===1&&s.claims.length===2,'RETAIN_RECORDS');
 const d=s.duties[0],a=s.accounts;need(d.id==='Loan1'&&d.debtor==='Borrower'&&d.creditor==='Pool'&&d.denomination==='Cash'&&d.legalRecourse==='Retained','DUTY_IDENTITY');
 need(n(d.principal)+n(d.accrued)===n(d.outstanding)&&n(d.accrued)===0n,'DUTY');
 // This oracle is a fixed partial-recovery case, not the full debt lifecycle.
 need(n(d.outstanding)>0n&&['Performing','Defaulted'].includes(d.status),'PARTIAL_RECOVERY_SCOPE');
 need(n(d.collateralUnits)===n(s.assets.Collateral.Custodian),'COLLATERAL_DUTY');
 need(n(s.history.forgivenNominal)===0n&&n(s.history.nominalCreated)===0n&&n(d.outstanding)+n(s.history.fundedNominalRecovery)===n(c.policy.initialDebt),'NO_DEBT_ERASURE');
 for(const [key,cap] of [['usedTransferIds','usedTransfers'],['usedAllocationIds','usedAllocations'],['usedObservationIds','observations']])need(s.history[key].length<=Number(n(b[cap]))&&new Set(s.history[key]).size===s.history[key].length,'CAPACITY');
 for(const grant of Object.values(s.authority))need(n(grant.remaining)+n(grant.spent)===n(grant.initial)&&grant.version===c.policy.version&&grant.dutyId==='Loan1','AUTHORITY');
 need(s.work.owner==='Servicer'&&n(s.work.remaining)+n(s.work.spent)===n(b.initialOrdinaryWork)&&n(s.work.closureReserve)===n(b.closureReserve),'WORK');
 exact(s.continuation,{owner:'Servicer',status:'Open',debtId:'Loan1',nominalCreationRemaining:'0',forgivenessRemaining:'0',policyVersion:c.policy.version},'CONTINUATION');
 need(n(a.impairmentAllowance)===n(a.impairmentExpense)-n(a.recoveryGain)&&n(a.impairmentAllowance)<=n(d.outstanding),'IMPAIRMENT_ACCOUNT');
 need(n(a.sharesOutstanding)===n(c.policy.initialShareSupply)&&sum(s.claims.map(x=>n(x.shares)))===n(a.sharesOutstanding),'SHARE_SUPPLY');
 const calculated=structuredClone(s);recalculate(calculated,c.policy);exact(calculated.accounts,s.accounts,'ACCOUNTS');exact(calculated.claims,s.claims,'CLAIM_LOSS_ALLOCATION');
}
function actorEffects(events){return assets.flatMap(asset=>actors.map(actor=>{
 const relevant=events.filter(e=>e.kind==='Transfer'&&e.asset===asset);
 const grossDebit=sum(relevant.filter(e=>e.from===actor).map(e=>n(e.amount))),grossCredit=sum(relevant.filter(e=>e.to===actor).map(e=>n(e.amount))),fees=sum(relevant.filter(e=>e.from===actor&&['LiquidationFee','RecoveryFee'].includes(e.purpose)).map(e=>n(e.amount))),net=grossCredit-grossDebit;
 return {actor,asset,grossDebit:grossDebit.toString(),grossCredit:grossCredit.toString(),feesPaid:fees.toString(),refund:'0',netChange:net.toString(),netCredit:(net>0n?net:0n).toString(),netDebit:(net<0n?-net:0n).toString()};
}));}
export function checkCase(c){
 need(c.schema==='moriarty.fixed-partial-loss-recovery-challenge/1'&&c.protocolConformance===false&&c.languageImplementation===false,'SCOPE');
 const b=c.bounds;need(Buffer.byteLength(JSON.stringify(c),'utf8')<=Number(n(b.sidecarUtf8Bytes)),'CAPACITY');
 need(c.steps.length===3&&c.steps.length<=Number(n(b.steps))&&Object.keys(c.observations).length<=Number(n(b.observations)),'CAPACITY');
 need(Object.keys(c.representationDisposition.mandatoryPredicates).length<=Number(n(b.claimCount))&&c.steps.length<=Number(n(b.dependencyCount)),'CAPACITY');
 need(n(c.policy.debtCreationLimit)===0n&&n(c.policy.forgivenessLimit)===0n,'AUTHORITY');
 for(const source of c.sources){const raw=readFileSync(new URL(source.path,ROOT));need(createHash('sha256').update(raw).digest('hex')===source.sha256,'SOURCE_PIN');}
 let current=structuredClone(c.initial),verification=BigInt(c.sources.length);const totals=Object.fromEntries(assets.map(a=>[a,total(current,a)]));invariants(current,c);verification+=4n;
 for(const step of c.steps){
  exact(step.before,current,'PRE_STATE');const s=structuredClone(current),d=s.duties[0],a=s.accounts;
  need(Array.isArray(step.events)&&step.events.length>0&&step.events.length<=Number(n(b.eventsPerStep)),'CAPACITY');
  // remaining excludes the separately conserved closureReserve.
  need(n(s.work.remaining)>=BigInt(step.events.length),'WORK');const now=n(step.now);
  for(const id of step.observations){const o=c.observations[id];need(o&&o.version===c.policy.version&&n(o.observedAt)<=now&&now<=n(o.validUntil)&&!s.history.usedObservationIds.includes(id),'OBSERVATION');
   if(id==='DefaultObs')need(o.kind==='DefaultNotice'&&o.sequence==='1'&&o.authority==='Governor'&&o.dutyId===d.id&&o.borrower===d.debtor&&n(o.dueAt)<now,'OBSERVATION');
   else if(id==='BidObs')need(o.kind==='ExecutableBid'&&o.sequence==='2'&&o.authority==='Buyer'&&o.from==='Buyer'&&o.to==='Pool'&&o.collateralRecipient==='Buyer'&&o.collateralAsset==='Collateral'&&o.cashAsset==='Cash'&&n(o.collateralUnits)===n(c.policy.collateralUnits)&&n(o.cashAmount)===n(c.policy.saleCash),'OBSERVATION');
   else throw Error('OBSERVATION');s.history.usedObservationIds.push(id);
  }
  const funding={},allocations={},reversed=new Set();let collateralDelivered=0n,saleCash=0n,saleDischarged=0n;
  for(const e of step.events){
   need(['MarkDefault','Impair','Transfer','Repay','ReverseImpairment'].includes(e.kind),'UNKNOWN_EFFECT');
   const grant=s.authority[e.grant];need(grant&&e.actor===grant.holder&&e.kind===grant.scope&&e.version===c.policy.version&&e.version===grant.version&&now<=n(grant.expiresAt),'AUTHORITY');
   const amount=e.kind==='MarkDefault'?1n:n(e.amount);need(amount>0n,'ZERO_AMOUNT');
   // Financial preconditions are checked before grant consumption, which is tentative.
   if(e.kind==='MarkDefault'){need(e.dutyId===d.id&&d.status==='Performing'&&step.observations.includes(e.observationId)&&e.observationId==='DefaultObs','OBSERVATION');d.status='Defaulted';}
   if(e.kind==='Impair'){need(e.dutyId===d.id&&d.status==='Defaulted'&&amount===n(d.outstanding)-n(c.policy.saleCash)&&n(a.impairmentAllowance)===0n,'IMPAIRMENT');put(a,'impairmentAllowance',amount);put(a,'impairmentExpense',n(a.impairmentExpense)+amount);}
   if(e.kind==='Transfer'){
    need(assets.includes(e.asset)&&actors.includes(e.from)&&actors.includes(e.to)&&e.from!==e.to&&e.actor===e.from&&grant.asset===e.asset&&(grant.recipient===null?e.grant==='BorrowerGrant'&&['Pool','FeeCollector'].includes(e.to):grant.recipient===e.to),'AUTHORITY');
    need(!s.history.usedTransferIds.includes(e.id),'REPLAY');need(n(s.assets[e.asset][e.from])>=amount,'BALANCE');
    if(e.asset==='Collateral'){need(e.purpose==='CollateralSale'&&step.observations.includes('BidObs')&&e.from==='Custodian'&&e.to==='Buyer','SALE_DELIVERY');collateralDelivered+=amount;put(d,'collateralUnits',n(d.collateralUnits)-amount);}
    if(e.purpose==='SaleProceeds'){need(step.observations.includes('BidObs')&&e.asset==='Cash'&&e.from==='Buyer'&&e.to==='Pool','SALE_CASH');saleCash+=amount;}
    if(e.purpose==='LiquidationFee'){need(e.asset==='Cash'&&e.from==='Pool'&&e.to==='FeeCollector'&&amount===n(c.policy.liquidationFee),'FEE');put(a,'feeExpense',n(a.feeExpense)+amount);}
    if(e.purpose==='RecoveryFee')need(e.asset==='Cash'&&e.from==='Borrower'&&e.to==='FeeCollector'&&amount===n(c.policy.laterFee),'FEE');
    if(e.purpose==='VoluntaryRecovery')need(e.asset==='Cash'&&e.from==='Borrower'&&e.to==='Pool'&&amount===n(c.policy.laterRecovery),'FUNDING');
    need(['CollateralSale','SaleProceeds','LiquidationFee','RecoveryFee','VoluntaryRecovery'].includes(e.purpose),'TRANSFER_PURPOSE');
    put(s.assets[e.asset],e.from,n(s.assets[e.asset][e.from])-amount);put(s.assets[e.asset],e.to,n(s.assets[e.asset][e.to])+amount);
    s.history.usedTransferIds.push(e.id);funding[e.id]={asset:e.asset,to:e.to,remaining:amount,purpose:e.purpose};
   }
   if(e.kind==='Repay'){
    need(grant.asset===d.denomination&&grant.recipient===d.creditor&&grant.dutyId===d.id,'AUTHORITY');
    const f=funding[e.transferId];need(f&&f.asset===grant.asset&&f.to===grant.recipient&&f.remaining>=amount&&e.dutyId===d.id&&amount<=n(d.outstanding),'FUNDING');need(!s.history.usedAllocationIds.includes(e.allocationId),'REPLAY');
    f.remaining-=amount;put(d,'principal',n(d.principal)-amount);put(d,'outstanding',n(d.outstanding)-amount);put(s.history,'fundedNominalRecovery',n(s.history.fundedNominalRecovery)+amount);s.history.usedAllocationIds.push(e.allocationId);allocations[e.allocationId]=amount;if(f.purpose==='SaleProceeds')saleDischarged+=amount;
   }
   if(e.kind==='ReverseImpairment'){
    need(e.dutyId===d.id&&allocations[e.allocationId]===amount&&!reversed.has(e.allocationId)&&amount<=n(a.impairmentAllowance),'REVERSAL_FUNDING');put(a,'impairmentAllowance',n(a.impairmentAllowance)-amount);put(a,'recoveryGain',n(a.recoveryGain)+amount);reversed.add(e.allocationId);
   }
   need(n(grant.remaining)>=amount,'AUTHORITY');put(grant,'remaining',n(grant.remaining)-amount);put(grant,'spent',n(grant.spent)+amount);verification++;
  }
  if(step.observations.includes('BidObs'))need(collateralDelivered===n(c.policy.collateralUnits)&&saleCash===n(c.policy.saleCash)&&saleDischarged===saleCash,'SALE_DELIVERY');
  need(n(step.workCost)===BigInt(step.events.length),'WORK_COST');put(s.work,'remaining',n(s.work.remaining)-n(step.workCost));put(s.work,'spent',n(s.work.spent)+n(step.workCost));
  recalculate(s,c.policy);invariants(s,c);verification+=4n;
  conserveAssets(s,totals);
  exact(s,step.after,'POST_STATE');exact(actorEffects(step.events),step.actorEffects,'ACTOR_EFFECTS');
  const beforeLeaves=Object.fromEntries(leaves(current)),afterLeaves=Object.fromEntries(leaves(s));
  const writes=[...new Set([...Object.keys(beforeLeaves),...Object.keys(afterLeaves)])].filter(k=>!isDeepStrictEqual(beforeLeaves[k],afterLeaves[k])).sort();
  exact(step.footprint.readPaths,Object.keys(beforeLeaves).sort(),'FOOTPRINT');exact(step.footprint.writePaths,writes,'FOOTPRINT');exact(step.footprint.observationIds,step.observations,'FOOTPRINT');
  need(step.footprint.readPaths.length<=Number(n(b.readPaths))&&writes.length<=Number(n(b.writePaths)),'CAPACITY');current=s;
 }
 need(verification<=n(b.verificationWork),'VERIFICATION_WORK');
 return {status:'PASS',caseId:c.id,stepsChecked:String(c.steps.length),fundedRecovery:current.history.fundedNominalRecovery,residualDebt:current.duties[0].outstanding,remainingImpairment:current.accounts.impairmentAllowance,allocatedNetLoss:current.accounts.netLoss,poolCash:current.assets.Cash.Pool,ordinaryWorkRemaining:current.work.remaining,closureReserve:current.work.closureReserve,logicalVerificationUnits:verification.toString(),networkAcceptance:false,languageAcceptance:false};
}

function patch(value,patches){const out=structuredClone(value);for(const p of patches){const parts=p.path.slice(1).split('/');let owner=out;for(const key of parts.slice(0,-1))owner=owner[key];const key=parts.at(-1);if(p.op==='remove'){if(Array.isArray(owner))owner.splice(Number(key),1);else delete owner[key];}else owner[key]=structuredClone(p.value);}return out;}
function runUnchanged(value,operation){
 const before=structuredClone(value);
 try{return operation(value);}finally{exact(value,before,'INPUT_MUTATED');}
}
function expectRejection(value,operation,code,id){
 let error;try{runUnchanged(value,operation);}catch(e){error=e;}
 if(!error||error.message!==code)throw Error(`CONTROL_${id}: ${error?.message??'ACCEPTED'}`);
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 try{
  const input=JSON.parse(readFileSync(process.argv[2]??CASE,'utf8'));
  const result=runUnchanged(input,checkCase);
  for(const m of input.mutations)expectRejection(patch(input,m.patches),checkCase,m.reject,m.id);
  for(const control of input.positiveControls){const actual=runUnchanged(patch(input,control.patches),checkCase);for(const [key,value] of Object.entries(control.expected))exact(actual[key],value,'POSITIVE_CONTROL');}
  for(const control of input.invariantControls){
   need(control.kind==='asset-conservation','CONTROL_KIND');let boundary=input;for(const key of control.statePath.slice(1).split('/'))boundary=boundary[key];
   const totals=Object.fromEntries(assets.map(asset=>[asset,total(input.initial,asset)]));
   expectRejection(patch(boundary,control.patches),value=>conserveAssets(value,totals),control.reject,control.id);
  }
  for(const control of input.scopeControls)expectRejection(patch(input,control.patches),checkCase,control.reject,control.id);
  console.log(JSON.stringify({...result,mutationsRejected:String(input.mutations.length),directInvariantControls:String(input.invariantControls.length),positiveBoundaryControls:String(input.positiveControls.length),scopeExclusions:String(input.scopeControls.length),inputImmutability:'checked on success and every rejection',scope:'Fixed three-step illustrative partial-recovery JSON oracle only; full funded closure remains required separate work; no language, ledger, proof or protocol acceptance'}));
 }catch(error){console.error(error.message);process.exitCode=1;}
}
