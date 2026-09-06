/** Trusted local financial adapters and one-shot simulation. Never a ledger or PCD verifier. */
import { evaluate, type Action, type CoreState, type Effect } from './core.ts';
import { elaborate, exampleSource, defaultAction, type ElaboratedBundle } from './packages.ts';
import { canonical, hashCanonical } from './claims.ts';
import { checkIntentEffects, type IntentPolicy } from './policy.ts';
import { assetKey, readIntent, verifyIntent, checkOutcomeTrace,
  type AssetId, type AssetEffect, type Balance, type IntentIR, type OutcomeDomain,
  type PlanIR, type SignedIntent, type TrustContext } from './outcome.ts';

type Rejected = {outcome:'rejected';code:string;message:string};
type Entry = {bundle:ElaboratedBundle;state:CoreState;programHash:string};
type Mapping = {field:string;asset:AssetId;account:string};
export type LocalOutcomeReceipt = {
  kind:'LocalOutcomeReceipt';status:'eligible-local-simulation'|'simulated-complete';
  intentHash:string;planHash:string;beforeHash:string;afterHash:string;
  effects:AssetEffect[];dueUpdates:Effect[];
  grossDebits:{asset:AssetId;amount:string}[];netCredits:{asset:AssetId;account:string;amount:string}[];
  authorityStatus:'unconsumed'|'one-shot-consumed';proofStatus:'unavailable';
};
type Inspected = {outcome:'checked';receipt:LocalOutcomeReceipt;epoch:number;nonceKey:string;
  nextBalances:Balance[];entryIndex:number;nextState:CoreState};
const DOMAIN:OutcomeDomain={network:'local-demo',deployment:'browser-outcome-r2b'};
const MAX=(1n<<128n)-1n;
const clone=<T>(x:T):T=>JSON.parse(canonical(x)) as T;
const fail=(code:string,message:string):Rejected=>({outcome:'rejected',code,message});
const caught=(e:unknown):Rejected=>fail('InvalidPlan',e instanceof Error?e.message:'Invalid local plan');
function uint(x:unknown):bigint {
 if(typeof x!=='string'||x.length>39||!/^(0|[1-9][0-9]*)$/.test(x))throw new Error('Invalid UInt128');
 const n=BigInt(x);if(n>MAX)throw new Error('UInt128 overflow');return n;
}
function keys(x:unknown,expected:string[]):void {
 if(!x||typeof x!=='object'||Array.isArray(x)||Object.keys(x).sort().join('|')!==[...expected].sort().join('|'))throw new Error('Unknown or missing fields');
}
const asset=(reference:string):AssetId=>({domain:DOMAIN.network,issuer:'demo',reference,kind:'token'});
const ASSETS:Record<string,AssetId>={'demo:A':asset('A'),'demo:B':asset('B'),'demo:USD6':asset('USD6')};
function resolveAsset(name:string):AssetId {if(!Object.hasOwn(ASSETS,name))throw new Error('Unregistered asset');return clone(ASSETS[name]);}
const balanceKey=(a:AssetId,account:string)=>canonical({asset:a,account});
function mapping(entry:Entry):Mapping[] {
 const t=entry.bundle.source.terms;
 if(entry.bundle.source.package==='Actus.LAM.FirstPeriod')return [
  {field:'borrowerCash',asset:resolveAsset(t.asset),account:t.borrower},
  {field:'lenderCash',asset:resolveAsset(t.asset),account:t.lender}];
 return [
  {field:'reserveA',asset:resolveAsset(t.assetA),account:t.instance},{field:'reserveB',asset:resolveAsset(t.assetB),account:t.instance},
  {field:'traderA',asset:resolveAsset(t.assetA),account:t.trader},{field:'traderB',asset:resolveAsset(t.assetB),account:t.trader},
  {field:'providerA',asset:resolveAsset(t.assetA),account:t.provider},{field:'providerB',asset:resolveAsset(t.assetB),account:t.provider}];
}
function accountPolicy(entry:Entry,before:CoreState,action:Action):IntentPolicy {
 const t=entry.bundle.source.terms;
 const accounting=Object.fromEntries(mapping(entry).map(m=>[m.field,{asset:m.asset.reference==='USD6'?'demo:USD6':`demo:${m.asset.reference}`,account:m.account}]));
 if(entry.bundle.source.package==='Exchange.ConstantProduct')return {profile:'swap',accounting,
  transfers:[{asset:t.assetA,from:t.trader,to:t.instance,maxAmount:action.args.amountIn},{asset:t.assetB,from:t.instance,to:t.trader,maxAmount:before.values.reserveB}],
  minimumCredits:[],fees:[],dues:[],allowedWrites:['reserveA','reserveB','traderA','traderB']};
 const total=(uint(before.values.principalDue)+uint(before.values.interestDue)).toString();
 return {profile:'loan',accounting,transfers:[{asset:t.asset,from:t.borrower,to:t.lender,maxAmount:total}],minimumCredits:[],fees:[],
  dues:(['principal','interest'] as const).map(bucket=>({kind:'DueSettled',bucket,dueId:`${t.instance}:${bucket}`,debtor:t.borrower,creditor:t.lender,
   denomination:t.denomination,asset:t.asset,maxAmount:before.values[bucket+'Due']})),
  allowedWrites:['borrowerCash','lenderCash','principalDue','interestDue','principalPaid','interestPaid','cursor','closed']};
}
function projectBalances(before:Balance[],effects:AssetEffect[]):Balance[] {
 const balances=clone(before);const seen=new Map(balances.map((b,i)=>[balanceKey(b.asset,b.account),i]));
 for(const effect of effects){
  keys(effect,['kind','asset','from','to','amount']);assetKey(effect.asset);
  if(!['Transfer','Fee'].includes(effect.kind)||typeof effect.from!=='string'||typeof effect.to!=='string')throw new Error('Invalid asset effect');
  const amount=uint(effect.amount),from=seen.get(balanceKey(effect.asset,effect.from)),to=seen.get(balanceKey(effect.asset,effect.to));
  if(from===undefined||to===undefined||from===to)throw new Error('Unregistered or identical balance endpoints');
  const debit=uint(balances[from].amount),credit=uint(balances[to].amount);
  if(debit<amount||credit+amount>MAX)throw new Error('Insufficient prefix balance or overflow');
  balances[from].amount=(debit-amount).toString();balances[to].amount=(credit+amount).toString();
 }
 return balances;
}

export class OutcomeRuntime {
 #entries:Entry[];#balances:Balance[];#used=new Set<string>();#epoch=0;#now='100';
 private constructor(entries:Entry[]) {
  this.#entries=entries;
  const balances=new Map<string,Balance>();
  for(const entry of entries)for(const m of mapping(entry)){
   const value=entry.state.values[m.field];uint(value);const key=balanceKey(m.asset,m.account);
   if(balances.has(key)&&balances.get(key)!.amount!==value)throw new Error('Conflicting initial shared balances');
   balances.set(key,{asset:m.asset,account:m.account,amount:value});
  }
  // A declared demo fee recipient is present for each supported asset. No asset is invented by a plan.
  for(const a of Object.values(ASSETS))balances.set(balanceKey(a,'fees'),{asset:clone(a),account:'fees',amount:'0'});
  this.#balances=[...balances.values()];if(this.#balances.length>32)throw new Error('Balance capacity exceeded');
 }
 static async create(kind:'swap'|'loan'):Promise<{runtime:OutcomeRuntime;intent:IntentIR}> {
  if(kind!=='swap'&&kind!=='loan')throw new Error('Unsupported example');
  const sources=[JSON.parse(exampleSource(kind))];
  if(kind==='swap'){const other=clone(sources[0]);other.terms.instance='pool:alternate';other.terms.reserveA='2000000';other.terms.reserveB='4000000';sources.push(other);}
  const entries:Entry[]=[];
  for(const source of sources){
   const b=elaborate(source);if(b.outcome==='rejected')throw new Error(b.message);
   let state=b.initialState;
   if(kind==='loan'){const accrued=evaluate(b.program,state,defaultAction(b,state));if(accrued.outcome!=='evaluated')throw new Error(accrued.message);state=accrued.after;}
   entries.push({bundle:b,state:clone(state),programHash:await hashCanonical(b.program)});
  }
  const runtime=new OutcomeRuntime(entries),first=entries[0],t=first.bundle.source.terms;
  const total=kind==='loan'?(uint(first.state.values.principalDue)+uint(first.state.values.interestDue)).toString():'10000';
  const intent:IntentIR={version:'moriarty-intent/1',domain:clone(DOMAIN),principal:kind==='swap'?t.trader:t.borrower,nonce:'0',
   validity:{notBefore:'100',expiresAt:'200'},lifecycle:'atomic',
   authority:[{asset:resolveAsset(kind==='swap'?t.assetA:t.asset),maxDebit:total,recipients:kind==='swap'?entries.map(e=>e.state.instance):[t.lender]}],
   goals:[{asset:resolveAsset(kind==='swap'?t.assetB:t.asset),account:kind==='swap'?t.trader:t.lender,minCredit:kind==='swap'?'19700':total}],
   fees:[],agreements:entries.map(e=>({id:e.state.instance,programHash:e.programHash})),
   requiredClaims:['ContractInvariant','IntentRefinement','TransitionValidity','HistoryCompliance'],extensions:[]};
  return {runtime,intent:readIntent(intent)};
 }
 #state(entry:Entry,balances=this.#balances):CoreState {
  const state=clone(entry.state);const table=new Map(balances.map(b=>[balanceKey(b.asset,b.account),b.amount]));
  for(const m of mapping(entry))state.values[m.field]=table.get(balanceKey(m.asset,m.account))!;
  return state;
 }
 #financial(balances=this.#balances,changedIndex=-1,changedState?:CoreState) {
  return {balances:clone(balances),agreements:this.#entries.map((e,i)=>({id:e.state.instance,programHash:e.programHash,
   state:this.#state(i===changedIndex?{...e,state:changedState!}:e,balances)}))};
 }
 snapshot():{now:string;balances:Balance[];consumed:number;agreements:{id:string;programHash:string;state:CoreState}[]} {
  return {now:this.#now,...this.#financial(),consumed:this.#used.size};
 }
 advanceTime(now:string):{outcome:'advanced'}|Rejected {
  try{const time=uint(now);if(time<uint(this.#now))return fail('ClockBackwards','The local clock cannot move backwards');
   if(now!==this.#now){this.#now=now;this.#epoch++;}return {outcome:'advanced'};
  }catch(e){return caught(e);}
 }
 async propose(intentHash:string,route=0):Promise<PlanIR> {
  if(typeof intentHash!=='string'||!/^[0-9a-f]{64}$/.test(intentHash)||!Number.isInteger(route)||route<0||route>=this.#entries.length)throw new Error('Invalid intent hash or route');
  const entry=this.#entries[route],state=this.#state(entry),action=defaultAction(entry.bundle,state);
  return {version:'moriarty-plan/1',intentHash,steps:[{agreementId:state.instance,programHash:entry.programHash,predecessorHash:await hashCanonical(state),action}],fees:[]};
 }
 async #inspect(signedValue:unknown,planValue:unknown,trustValue:TrustContext):Promise<Inspected|Rejected> {
  try {
   const input=clone({signed:signedValue,plan:planValue,trust:trustValue});
   const signed=input.signed as SignedIntent,plan=input.plan as PlanIR,trust=input.trust;
   const epoch=this.#epoch,now=this.#now;
   if(canonical(trust.domain)!==canonical(DOMAIN))return fail('WrongDomain','Trust domain differs from this runtime');
   const signature=await verifyIntent(signed,trust,now);if(signature.outcome!=='checked')return signature;
   const intent=readIntent(signed.intent);
   const nonceKey=canonical({domain:intent.domain,principal:intent.principal,nonce:intent.nonce});
   if(this.#used.has(nonceKey))return fail('NonceConsumed','This principal/domain/nonce was already consumed');
   if(this.#used.size>=128)return fail('NonceCapacity','The local nonce table is full; records cannot be evicted');
   if(epoch!==this.#epoch)return fail('StaleState','Local state changed during signature checking');
   keys(plan,['version','intentHash','steps','fees']);
   if(plan.version!=='moriarty-plan/1'||plan.intentHash!==signed.intentHash||!Array.isArray(plan.steps)||plan.steps.length!==1||!Array.isArray(plan.fees)||plan.fees.length>4)throw new Error('Unsupported or unbound plan');
   const step=plan.steps[0];keys(step,['agreementId','programHash','predecessorHash','action']);
   const index=this.#entries.findIndex(e=>e.state.instance===step.agreementId),entry=this.#entries[index];
   if(!entry||entry.programHash!==step.programHash||!intent.agreements.some(a=>a.id===step.agreementId&&a.programHash===step.programHash))return fail('UnregisteredProgram','Agreement program is not registered and signed');
   const before=this.#state(entry),predecessorHash=await hashCanonical(before);
   if(predecessorHash!==step.predecessorHash)return fail('StalePredecessor','Proposed predecessor differs from current financial state');
   const expectedAction=entry.bundle.source.package==='Exchange.ConstantProduct'?'swap':'settle';
   if(step.action?.name!==expectedAction||step.action?.args?.actor!==intent.principal)return fail('WrongAction','Only the principal swap or full loan settlement is supported');
   const evaluated=evaluate(entry.bundle.program,before,step.action);
   if(evaluated.outcome!=='evaluated')return evaluated;
   const projection=checkIntentEffects(before,evaluated.after,evaluated.effects,accountPolicy(entry,before,step.action));
   if(projection.outcome!=='checked')return projection;
   const effects:AssetEffect[]=[];const dueUpdates:Effect[]=[];
   for(const effect of evaluated.effects){
    if(effect.kind==='Transfer')effects.push({kind:'Transfer',asset:resolveAsset(effect.fields.asset),from:effect.fields.from,to:effect.fields.to,amount:effect.fields.amount});
    else if(effect.kind==='DueSettled')dueUpdates.push(clone(effect));
    else throw new Error('Unsupported financial effect');
   }
   for(const fee of plan.fees){if(fee.kind!=='Fee'||fee.from!==intent.principal)throw new Error('Only explicit principal fees are supported');effects.push(clone(fee));}
   if(effects.length>16)throw new Error('Effect capacity exceeded');
   const nextBalances=projectBalances(this.#balances,effects);
   const outcome=checkOutcomeTrace(intent,this.#balances,effects,nextBalances);
   if(outcome.outcome!=='checked')return outcome;
   const financialBefore=this.#financial(),financialAfter=this.#financial(nextBalances,index,evaluated.after);
   const receipt:LocalOutcomeReceipt={kind:'LocalOutcomeReceipt',status:'eligible-local-simulation',intentHash:signed.intentHash,
    planHash:await hashCanonical({domain:'MORIARTY-OUTCOME-PLAN-v1',plan}),
    beforeHash:await hashCanonical(financialBefore),afterHash:await hashCanonical(financialAfter),effects,dueUpdates,
    grossDebits:outcome.grossDebits,netCredits:outcome.netCredits,authorityStatus:'unconsumed',proofStatus:'unavailable'};
   if(epoch!==this.#epoch)return fail('StaleState','Local state or clock changed during checking');
   return {outcome:'checked',receipt,epoch,nonceKey,nextBalances,entryIndex:index,nextState:evaluated.after};
  }catch(e){return caught(e);}
 }
 async preview(signed:unknown,plan:unknown,trust:TrustContext):Promise<{outcome:'checked';receipt:LocalOutcomeReceipt}|Rejected> {
  const result=await this.#inspect(signed,plan,trust);
  return result.outcome==='checked'?{outcome:'checked',receipt:clone(result.receipt)}:result;
 }
 async simulate(signed:unknown,plan:unknown,trust:TrustContext):Promise<{outcome:'simulated';receipt:LocalOutcomeReceipt}|Rejected> {
  const result=await this.#inspect(signed,plan,trust);if(result.outcome!=='checked')return result;
  if(result.epoch!==this.#epoch||this.#used.has(result.nonceKey))return fail('StaleState','Another operation consumed or changed the local state');
  // No await or caller callback between the final check and joint state/nonce update.
  this.#entries[result.entryIndex].state=result.nextState;this.#balances=result.nextBalances;
  this.#used.add(result.nonceKey);this.#epoch++;
  return {outcome:'simulated',receipt:{...result.receipt,status:'simulated-complete',authorityStatus:'one-shot-consumed'}};
 }
 async verifyRealAcceptance(signed:unknown,plan:unknown,trust:TrustContext):Promise<Rejected|{outcome:'unavailable';missing:string[]}> {
  const result=await this.#inspect(signed,plan,trust);if(result.outcome!=='checked')return result;
  return {outcome:'unavailable',missing:['ContractInvariant','IntentRefinement','TransitionValidity','HistoryCompliance']};
 }
}
export const createOutcomeDemo=(kind:'swap'|'loan')=>OutcomeRuntime.create(kind);
