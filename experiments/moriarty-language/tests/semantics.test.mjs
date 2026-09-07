import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createSimulator, evaluate, derive, hash, sealState} from '../src/evaluate.ts';
const bounds=readFileSync(new URL('../spec/bounds.json',import.meta.url));
const source=name=>readFileSync(new URL(`../spec/examples/${name}.mori`,import.meta.url));
const text=value=>({tag:'Text',value});
const uint=value=>({tag:'UInt128',value:String(value)});
const amount=(value,unit)=>({tag:'Amount',value:String(value),unit});
const named=(name,value)=>({name,value});
const checks=principal=>({authenticatedPrincipal:principal,genesisValid:true,nonceFresh:true,observationsAuthentic:true,predecessorSetValid:true,signatureValid:true,stateCurrentAndUnconsumed:true});
function setup(name,src=source(name)) {
 const sim=createSimulator(src,bounds);
 const genesis=sim.makeGenesis({domain:{network:'simulation',deployment:'test'},instanceId:'instance',principalBindings:['borrower','lender','pool','provider','trader'].map(actor=>({actor,principal:actor})),observationBindings:[{name:'now',provider:'clock',authenticationPolicy:'external'}]});
 return {sim,genesis,state:sim.initialState(genesis)};
}
function input(f,name,args,actor='borrower',state=f.state) {
 const action={schemaVersion:'moriarty-action/1',name,arguments:[named('actor',text(actor)),...args]};
 const statement={allowedActions:f.sim.bound.manifest.core.actions.map(a=>a.name),grossDebitCaps:f.sim.bound.manifest.settlementBindings.map(b=>({actor,asset:b.asset,maximumLedgerAmount:'340282366920938463463374607431768211455'})).sort((a,b)=>a.asset.localeCompare(b.asset)),minimumNetCredits:[],beforeStateHash:state.stateHash,domain:f.genesis.body.domain,genesisHash:f.genesis.genesisHash,instanceId:'instance',mode:'IntentRefinement',nonce:'nonce'+state.body.revision,permittedCalls:[],permittedRecipients:['borrower','lender','pool','provider','trader'],predecessors:[state.stateHash],principal:actor,program:f.sim.program,requiredClaimRoot:f.genesis.body.requiredClaimRoot,requiredClaims:f.sim.bound.manifest.requiredClaims,schemaVersion:'moriarty-outcome-intent/1',validity:{notBefore:'0',notAfterExclusive:'2000000000'}};
 return structuredClone({action,authority:{domain:'MORIARTY-OUTCOME-bounded-atomic/1',schemaVersion:'moriarty-authority/1',signature:{algorithm:'simulation-only',bytes:'',keyId:actor},statement,tag:'IntentRefinement'},checks:checks(actor),genesis:f.genesis,observations:{schemaVersion:'moriarty-observations/1',observations:[{name:'now',provider:'clock',evidenceDigest:'0'.repeat(64),value:uint(1)}]},program:f.sim.program,schemaVersion:'moriarty-evaluation/1',state});
}
const swapArgs=min=>[named('recipient',text('trader')),named('asset_in',text('ASSET_A')),named('asset_out',text('ASSET_B')),named('amount_in',amount(10000,'AssetA_quantum')),named('min_out',amount(min,'AssetB_quantum'))];
const settlementArgs=[named('settlement_asset',text('USD_TEST_ASSET')),named('amount_due',amount(533972602,'USD_micro'))];
function candidate(result){assert.equal(result.kind,'Simulation',JSON.stringify(result));return result.candidate;}
function reject(result,code){assert.equal(result.outcome,'Rejected',JSON.stringify(result));assert.equal(result.diagnostics[0].code,code);assert.deepEqual(Object.keys(result).sort(),['diagnostics','outcome','profile','programHash','schemaVersion']);}

test('loan accrual and settlement preserve all fields and obligation tombstones',()=>{
 const f=setup('loan');const accrued=candidate(f.sim.simulate(input(f,'accrue',[])));
 assert.deepEqual(Object.fromEntries(accrued.body.after.body.values.map(x=>[x.name,x.value.value])),{notional:'4500000000',principal_due:'500000000',interest_due:'33972602',principal_paid:'0',interest_paid:'0',borrower_cash:'20000000000',lender_cash:'0',cursor:'1',episode_closed:'0'});
 assert.deepEqual(accrued.body.after.body.obligations.map(o=>[o.dueId,o.amount.value,o.status]),[['lam01:period1:PR','500000000','Outstanding'],['lam01:period1:IP','33972602','Outstanding']]);
 const settled=candidate(f.sim.simulate(input(f,'settle',settlementArgs,'borrower',accrued.body.after)));
 assert.deepEqual(Object.fromEntries(settled.body.after.body.values.map(x=>[x.name,x.value.value])),{notional:'4500000000',principal_due:'0',interest_due:'0',principal_paid:'500000000',interest_paid:'33972602',borrower_cash:'19466027398',lender_cash:'533972602',cursor:'2',episode_closed:'1'});
 assert.equal(settled.body.after.body.episodeStatus,'Closed');assert.equal(settled.body.after.body.agreementStatus,'Outstanding');assert.equal(settled.body.after.body.remainingNotional.amount.value,'4500000000');assert.deepEqual(settled.body.after.body.obligations.map(o=>o.status),['Settled','Settled']);assert.equal(settled.body.after.body.remaining,'0');assert.equal(settled.body.effects[0].settlement.ledgerAmount,'533972602');
});
test('swap and close read state and capture effects in source order',()=>{
 const f=setup('swap');const swapped=candidate(f.sim.simulate(input(f,'swap',swapArgs(0),'trader')));
 assert.deepEqual(swapped.body.after.body.values.map(x=>x.value.value),['1010000','1980257','90000','19743','0','0','0']);
 const closed=candidate(f.sim.simulate(input(f,'close',[],'provider',swapped.body.after)));
 assert.deepEqual(closed.body.after.body.values.map(x=>x.value.value),['0','0','90000','19743','1010000','1980257','1']);assert.deepEqual(closed.body.effects.map(e=>e.amount.value),['1010000','1980257']);assert.equal(closed.body.after.body.agreementStatus,'NoOutstanding');assert.equal(closed.body.after.body.remaining,'6');
});
test('actionHash binds changed min_out even with identical resulting writes and effects',()=>{
 const f=setup('swap');const a=candidate(f.sim.simulate(input(f,'swap',swapArgs(0),'trader'))),b=candidate(f.sim.simulate(input(f,'swap',swapArgs(1),'trader')));assert.deepEqual(a.body.writes,b.body.writes);assert.deepEqual(a.body.effects,b.body.effects);assert.notEqual(a.body.actionHash,b.body.actionHash);assert.notEqual(a.traceHash,b.traceHash);
});
test('missing backend and client boolean claims cannot produce accepted Complete',async()=>{
 const f=setup('loan'),i=input(f,'accrue',[]);reject(await evaluate(f.sim.bound,i),'PROOF_INVALID');reject(await evaluate(f.sim.bound,i,{requiredClaimsValid:true,historyProofValid:true}),'PROOF_INVALID');
});
const minimal=body=>`agreement Arbitrary profile "moriarty-bounded-atomic/1" { lifetime 2; horizon 2000000000; state closed: UInt128 = uint(0); observation now: UInt128; status episode closed_when closed == uint(1); status agreement no_remaining_notional; action run(actor: Text) { ${body} } }`;
function remapState(i,edit){const body=structuredClone(i.state.body);edit(body);i.state=sealState(body);i.authority.statement.beforeStateHash=i.state.stateHash;i.authority.statement.predecessors=[i.state.stateHash];return i;}

test('checked arithmetic rejects overflow BEFORE division, subtraction underflow and zero divisor atomically',()=>{
 for(const [expression,code] of [[`floor_div(uint(${(1n<<128n)-1n}) * uint(2),uint(2))`,'ARITHMETIC_OVERFLOW'],['uint(0)-uint(1)','ARITHMETIC_UNDERFLOW'],['floor_div(uint(1),uint(0))','DIVISION_BY_ZERO']]){
  const f=setup('loan',minimal(`set closed = uint(1); let x = ${expression};`)),i=input(f,'run',[]),before=structuredClone(i);reject(f.sim.simulate(i),code);assert.deepEqual(i,before);
 }
});
test('short circuit execution counts visited expressions and permits Bool and Quantity locals',()=>{
 const f=setup('loan',minimal('let b = false and floor_div(uint(1),uint(0)) == uint(0); guard not b, "b"; let c = true or floor_div(uint(1),uint(0)) == uint(0); guard c, "c"; let q = amount(2,A) * amount(3,B); let n = floor_div(q,amount(2,A)); guard n == amount(3,B), "quantity";').replace('state closed','unit A; unit B; state closed'));
 const c=candidate(f.sim.simulate(input(f,'run',[])));assert.ok(Number(c.body.resourceCounts.expressionNodes)<Number(f.sim.bound.manifest.core.actions[0].resourceCounts.expressionNodes));assert.equal(c.body.resourceCounts.unitComponents,'2');
});
test('typed input units, unknown members, nominal minOut and state tampering fail closed',()=>{
 const f=setup('swap');let i=input(f,'swap',swapArgs(19744),'trader');const before=structuredClone(i);reject(f.sim.simulate(i),'GUARD_FAILED');assert.deepEqual(i,before);
 i=input(f,'swap',swapArgs(0),'trader');i.action.arguments.at(-1).value.unit='AssetA_quantum';reject(f.sim.simulate(i),'INPUT_SCHEMA');
 i=input(f,'swap',swapArgs(0),'trader');i.extra=true;reject(f.sim.simulate(i),'INPUT_SCHEMA');
 i=input(f,'swap',swapArgs(0),'trader');i.state.body.values[0].value.value='9';reject(f.sim.simulate(i),'DIGEST_MISMATCH');
 i=input(f,'swap',swapArgs(0),'trader');i.program.sourceHash='0'.repeat(64);reject(f.sim.simulate(i),'PROGRAM_BINDING');
});
test('principal, provider, external prechecks, expiry, invariant, reserve and exhaustion reject',()=>{
 const f=setup('swap');let i=input(f,'swap',swapArgs(0),'trader');i.checks.authenticatedPrincipal='provider';reject(f.sim.simulate(i),'PRINCIPAL_BINDING');
 i=input(f,'swap',swapArgs(0),'trader');i.observations.observations[0].provider='fake';reject(f.sim.simulate(i),'OBSERVATION_UNAUTHENTICATED');
 for(const [key,code] of [['nonceFresh','NONCE_STALE'],['stateCurrentAndUnconsumed','STATE_NOT_CURRENT'],['signatureValid','SIGNATURE_INVALID'],['genesisValid','GENESIS_UNAUTHENTICATED'],['predecessorSetValid','PREDECESSOR_UNAUTHENTICATED']]){i=input(f,'swap',swapArgs(0),'trader');i.checks[key]=false;reject(f.sim.simulate(i),code);}
 i=input(f,'swap',swapArgs(0),'trader');i.observations.observations[0].value=uint(2000000000);reject(f.sim.simulate(i),'HORIZON_EXPIRED');
 i=remapState(input(f,'swap',swapArgs(0),'trader'),s=>{s.revision='1';s.remaining='8';});reject(f.sim.simulate(i),'LIFETIME_INVARIANT');
 i=remapState(input(f,'swap',swapArgs(0),'trader'),s=>{s.revision='7';s.remaining='1';});reject(f.sim.simulate(i),'GUARD_FAILED');
 i=remapState(input(f,'close',[],'provider'),s=>{s.revision='8';s.remaining='0';});reject(f.sim.simulate(i),'LIFETIME_EXHAUSTED');
 i=remapState(input(f,'swap',swapArgs(0),'trader'),s=>{s.revision='340282366920938463463374607431768211455';s.remaining='1';});reject(f.sim.simulate(i),'LIFETIME_INVARIANT');
});
test('exact plan construction is explicitly unsigned simulation, then exact matching rejects changes',()=>{
 const f=setup('loan'),i=input(f,'accrue',[]);const a=i.authority.statement;
 for(const key of ['allowedActions','grossDebitCaps','minimumNetCredits','permittedCalls','permittedRecipients'])delete a[key];
 Object.assign(a,{mode:'ExactPlan',schemaVersion:'moriarty-exact-plan/1',action:structuredClone(i.action),exactEffects:[],exactWrites:[]});i.authority.tag='ExactPlan';i.authority.domain='MORIARTY-SIGN-bounded-atomic/1';
 reject(f.sim.simulate(i),'EXACT_PLAN_MISMATCH');const proposal=candidate(f.sim.simulate(i,{unsignedExactPlan:true}));
 a.exactWrites=proposal.body.writes.map(({field,value})=>({field,value}));a.exactEffects=proposal.body.effects.map(effect=>({effect}));candidate(f.sim.simulate(i));a.exactEffects[0].effect.amount.value='1';reject(f.sim.simulate(i),'EXACT_PLAN_MISMATCH');
});
test('obligation identity, tombstones, exact amounts and transfer conservation are enforced',()=>{
 const f=setup('loan'),accrued=candidate(f.sim.simulate(input(f,'accrue',[]))).body.after;
 for(const [change,code] of [[s=>s.obligations.pop(),'OBLIGATION_UNKNOWN'],[s=>s.obligations[0].status='Settled','OBLIGATION_ALREADY_SETTLED'],[s=>s.obligations[0].debtor='someone','OBLIGATION_MISMATCH'],[s=>s.obligations[0].amount.value='500000001','OBLIGATION_PARTIAL_UNSUPPORTED'],[s=>s.obligations[0].amount.value='499999999','OBLIGATION_EXCESS']]){const i=remapState(input(f,'settle',settlementArgs,'borrower',accrued),change);reject(f.sim.simulate(i),code);}
 let i=remapState(input(f,'accrue',[],'borrower',accrued),s=>{s.values.find(x=>x.name==='cursor').value=uint(0);s.values.find(x=>x.name==='notional').value=amount(5000000000,'USD_micro');s.remainingNotional.amount=amount(5000000000,'USD_micro');});reject(f.sim.simulate(i),'OBLIGATION_DUPLICATE');
 const altered=source('loan').toString().replace('from: const.borrower, to: const.lender, amount: arg.amount_due','from: const.borrower, to: const.borrower, amount: arg.amount_due');
 const g=setup('loan',altered),state=candidate(g.sim.simulate(input(g,'accrue',[]))).body.after;reject(g.sim.simulate(input(g,'settle',settlementArgs,'borrower',state)),'OBLIGATION_CONSERVATION');
});
function movements(effects,quantum='1'){
 const kinds=[...new Set(effects.map(e=>e.kind))];
 return `agreement Movements profile "moriarty-bounded-atomic/1" { lifetime 2; horizon 2000000000; unit Coin; state closed: UInt128 = uint(0); observation now: UInt128; status episode closed_when closed == uint(1); status agreement no_remaining_notional; settlement coin_binding asset text("COIN") quantum amount(${quantum},Coin); policy p targets ${effects.map((e,j)=>`effect(run,${j},amount)`).join(',')} { unit Coin; derivation "test"; rounding none; remainder "none"; comparison "exact"; proof "movement_claim"; } ${kinds.map(kind=>`effect ${kind} { asset: Text; from: Text; to: Text; amount: Amount; }`).join(' ')} action run(actor: Text) { ${effects.map(e=>`emit ${e.kind} { asset: text("COIN"), from: text("${e.from}"), to: text("${e.to}"), amount: amount(${e.amount},Coin) };`).join(' ')} } }`;
}
test('quantum uses nominal subunits per ledger unit and rejects indivisible transfer or mismatched asset',()=>{
 for(const [n,result] of [['20','2'],['15','SETTLEMENT_NON_DIVISIBLE']]){const f=setup('loan',movements([{kind:'Transfer',from:'borrower',to:'lender',amount:n}],'10'));const r=f.sim.simulate(input(f,'run',[]));if(n==='20'){const e=candidate(r).body.effects[0];assert.deepEqual(e.settlement,{asset:'COIN',binding:'coin_binding',ledgerAmount:result,nominalAmount:amount(n,'Coin'),quantum:amount(10,'Coin'),unit:'Coin'});}else reject(r,result);}
 const f=setup('loan',movements([{kind:'Transfer',from:'borrower',to:'lender',amount:'20'}]).replace('emit Transfer { asset: text("COIN")','emit Transfer { asset: text("BAD")'));reject(f.sim.simulate(input(f,'run',[])),'SETTLEMENT_BINDING_MISSING');
});
test('outcome gross caps include fees and never net refunds; net goals include every debit',()=>{
 const f=setup('loan',movements([{kind:'Transfer',from:'borrower',to:'lender',amount:'100'},{kind:'Transfer',from:'lender',to:'borrower',amount:'99'},{kind:'Fee',from:'borrower',to:'provider',amount:'2'}]));
 let i=input(f,'run',[]);i.authority.statement.grossDebitCaps[0].maximumLedgerAmount='101';reject(f.sim.simulate(i),'INTENT_DEBIT_CAP');
 i=input(f,'run',[]);i.authority.statement.minimumNetCredits=[{actor:'borrower',asset:'COIN',minimumLedgerAmount:'0'}];reject(f.sim.simulate(i),'INTENT_NET_GOAL');
 i=input(f,'run',[]);i.authority.statement.grossDebitCaps=[];reject(f.sim.simulate(i),'INTENT_DEBIT_UNCAPPED');
 i=input(f,'run',[]);i.authority.statement.grossDebitCaps[0].actor='lender';reject(f.sim.simulate(i),'INTENT_ACTOR_SCOPE');
 i=input(f,'run',[]);i.authority.statement.permittedRecipients=['borrower','lender'];reject(f.sim.simulate(i),'INTENT_RECIPIENT_FORBIDDEN');
 i=input(f,'run',[]);i.authority.statement.permittedCalls=[{callee:'other',selector:'call'}];reject(f.sim.simulate(i),'INTENT_CALL_UNSUPPORTED');
 i=input(f,'run',[]);i.authority.statement.minimumNetCredits=[{actor:'borrower',asset:'COIN',minimumLedgerAmount:String((1n<<128n)-1n)}];reject(f.sim.simulate(i),'INTENT_ARITHMETIC_OVERFLOW');
});

test('all four complete financial traces agree with retained independent legacy evaluator after documented renaming',()=>{
 const legacy=JSON.parse(readFileSync(new URL('../../../evidence/moriarty-completion-program-2026-09-07/MC01/profile-04/evaluator/legacy-reference.json',import.meta.url)));
 const names={principalDue:'principal_due',interestDue:'interest_due',principalPaid:'principal_paid',interestPaid:'interest_paid',borrowerCash:'borrower_cash',lenderCash:'lender_cash',reserveA:'reserve_a',reserveB:'reserve_b',traderA:'trader_a',traderB:'trader_b',providerA:'provider_a',providerB:'provider_b'};
 const scalar={'demo:USD6':'USD_TEST_ASSET','demo:A':'ASSET_A','demo:B':'ASSET_B','pool:demo':'pool','USD':'USD_micro','loan:demo:principal':'lam01:period1:PR','loan:demo:interest':'lam01:period1:IP'};
 for(const name of ['loan','swap']){
  const f=setup(name);let state=f.state;
  const calls=name==='loan'?[['accrue',[],'borrower'],['settle',settlementArgs,'borrower']]:[['swap',swapArgs(19743),'trader'],['close',[],'provider']];
  const rename=x=>x==='closed'?(name==='loan'?'episode_closed':'epoch_closed'):(names[x]??x);
  for(let j=0;j<2;j++){
   const [action,args,actor]=calls[j],i=input(f,action,args,actor,state),c=candidate(f.sim.simulate(i)),reference=legacy.cases[name].steps[j].result;
   const values=Object.entries(reference.after.values).map(([field,n])=>({name:rename(field),value:{...f.sim.bound.manifest.initialState.find(x=>x.name===rename(field)).value,value:n}}));
   const obligations=name==='loan'?[{amount:amount(500000000,'USD_micro'),creditor:'lender',debtor:'borrower',denomination:'USD_micro',dueId:'lam01:period1:PR',status:j===0?'Outstanding':'Settled'},{amount:amount(33972602,'USD_micro'),creditor:'lender',debtor:'borrower',denomination:'USD_micro',dueId:'lam01:period1:IP',status:j===0?'Outstanding':'Settled'}]:[];
   assert.deepEqual(c.body.after.body,{agreementStatus:name==='loan'?'Outstanding':'NoOutstanding',episodeStatus:j===0?'Open':'Closed',genesisHash:f.genesis.genesisHash,instanceId:'instance',obligations,profile:'moriarty-bounded-atomic/1',programHash:f.sim.bound.programHash,remaining:reference.after.remaining,remainingNotional:name==='loan'?{tag:'Amount',amount:amount(4500000000,'USD_micro')}:{tag:'NotApplicable'},revision:reference.after.revision,schemaVersion:'moriarty-state-body/1',values});
   assert.deepEqual(c.body.writes,reference.writes.map(field=>{const renamed=rename(field),policy=f.sim.bound.manifest.core.actions[j].instructions.find(x=>x.tag==='Set'&&x.field===renamed).policy;return {field:renamed,policy,value:values.find(x=>x.name===renamed).value};}));
   const expectedEffects=reference.effects.map((e,k)=>{
    const fields=Object.fromEntries(Object.entries(e.fields).map(([key,v])=>[key,scalar[v]??v]));
    const unit=name==='loan'?'USD_micro':fields.asset==='ASSET_A'?'AssetA_quantum':'AssetB_quantum';fields.amount=amount(fields.amount,unit);
    const effect={...fields,kind:e.kind,ordinal:String(k)};
    if(e.kind!=='DueCreated'){const binding=f.sim.bound.manifest.settlementBindings.find(b=>b.unit===unit);effect.settlement={asset:binding.asset,binding:binding.name,ledgerAmount:fields.amount.value,nominalAmount:fields.amount,quantum:amount(1,unit),unit};}
    return effect;
   });
   assert.deepEqual(c.body.effects,expectedEffects);
   assert.deepEqual(c.body.obligationDelta,{created:name==='loan'&&j===0?obligations:[],settled:name==='loan'&&j===1?obligations:[]});
   assert.equal(c.body.after.stateHash,hash('STATE',c.body.after.body));assert.equal(c.body.actionHash,hash('ACTION',i.action));assert.equal(c.traceHash,hash('TRACE',c.body));assert.equal(c.body.observationsHash,hash('OBSERVATIONS',i.observations));assert.equal(c.body.beforeStateHash,state.stateHash);assert.deepEqual(c.body.predecessors,[state.stateHash]);
   assert.equal(c.body.resourceCounts.executedInstructions,String(f.sim.bound.manifest.core.actions[j].instructions.length));assert.equal(c.body.resourceCounts.expressionNodes,f.sim.bound.manifest.core.actions[j].resourceCounts.expressionNodes);state=c.body.after;
  }
 }
});
test('policy provenance cannot discard floor nodes, and committed action provenance resets',()=>{
 const src=movements([{kind:'Transfer',from:'borrower',to:'lender',amount:'20'}]).replace('amount: amount(20,Coin)','amount: floor_div(amount(40,Coin),uint(2))');
 const f=setup('loan',src);reject(f.sim.simulate(input(f,'run',[])),'POLICY_PROVENANCE');
 // Successful loan settlement and swap closure in the complete traces above
 // require provenance reset; their rounding-none policies read prior floor writes.
});
test('rehashed malicious Core and source-map changes cannot substitute for source lowering',()=>{
 const f=setup('loan'),i=input(f,'accrue',[]),bound=structuredClone(f.sim.bound);bound.manifest.name='tamper';bound.programHash=hash('PROGRAM',bound.manifest);reject(derive(bound,i,{source:source('loan'),bounds}),'PROGRAM_ENCODING');
});
test('all binding equalities reject independently even after recomputing affected digest wrappers',()=>{
 const f=setup('loan');
 for(const [change,code] of [[i=>i.genesis.body.initialState[0].value.value='1','GENESIS_INITIAL_STATE_BINDING'],[i=>i.genesis.body.lifetime='3','GENESIS_LIFETIME_BINDING'],[i=>i.genesis.body.horizon='2000000001','GENESIS_HORIZON_BINDING'],[i=>i.genesis.body.bounds.boundsHash='0'.repeat(64),'GENESIS_BOUNDS_BINDING'],[i=>i.state.body.programHash='0'.repeat(64),'STATE_PROGRAM_BINDING'],[i=>i.state.body.genesisHash='0'.repeat(64),'STATE_GENESIS_BINDING'],[i=>i.state.body.instanceId='another','INSTANCE_BINDING'],[i=>i.authority.statement.domain={...i.authority.statement.domain,network:'another'},'AUTHORITY_CONTEXT_BINDING'],[i=>i.authority.statement.requiredClaimRoot='0'.repeat(64),'CLAIM_ROOT_BINDING'],[i=>i.authority.statement.predecessors=[],'PREDECESSOR_BINDING']]){
  const i=input(f,'accrue',[]);change(i);i.genesis.genesisHash=hash('GENESIS',i.genesis.body);i.state.stateHash=hash('STATE',i.state.body);reject(f.sim.simulate(i),code);
 }
});
test('backend authentication is mandatory, replay prechecks cannot be copied, and unavailable proof work cannot commit',async()=>{
 const f=setup('loan'),i=input(f,'accrue',[]);let commits=0;
 const backend={source:source('loan'),bounds,authenticate:async()=>({...checks('borrower'),nonceFresh:false}),verifyAndCommit:async()=>{commits++;throw Error('no proof backend');}};
 reject(await evaluate(f.sim.bound,i,backend),'NONCE_STALE');assert.equal(commits,0);assert.equal(i.checks.nonceFresh,true);
 backend.authenticate=async()=>checks('borrower');reject(await evaluate(f.sim.bound,i,backend),'PROOF_INVALID');assert.equal(commits,1);
});
test('resource count body omits the entire counts member and includes every other value',()=>{
 const f=setup('loan'),c=candidate(f.sim.simulate(input(f,'accrue',[]))),{resourceCounts,...countBody}=c.body;
 let nodes=0,maxDepth=0;function visit(v,depth=0){nodes++;maxDepth=Math.max(maxDepth,depth);if(Array.isArray(v))for(const x of v)visit(x,depth+1);else if(v&&typeof v==='object')for(const x of Object.values(v))visit(x,depth+1);}visit(countBody);
 assert.equal(resourceCounts.canonicalNodes,String(nodes));assert.equal(resourceCounts.canonicalDepth,String(maxDepth));
});

test('same-action creation and settlement preserve immutable event-time delta snapshots',()=>{
 const schemas=source('loan').toString().match(/  effect (?:Transfer|DueCreated|DueSettled) \{[\s\S]*?  \}/g).join('\n');
 const src=`agreement Snapshot profile "moriarty-bounded-atomic/1" {
 lifetime 2; horizon 2000000000; unit USD_micro; state done: UInt128 = uint(0);
 observation now: UInt128;
 settlement usd asset text("USD_TEST_ASSET") quantum amount(1, USD_micro);
 status episode closed_when done == uint(1); status agreement no_remaining_notional;
 policy payment targets effect(cycle, 0, amount), effect(cycle, 1, amount), effect(cycle, 2, amount) {
 unit USD_micro; derivation "fixed"; rounding none; remainder "none"; comparison "exact"; proof "snapshot_example";
 }
 ${schemas}
 action cycle(actor: Text) {
 emit DueCreated { due_id: text("instant"), debtor: arg.actor, creditor: text("lender"), denomination: text("USD_micro"), amount: amount(10, USD_micro) };
 emit Transfer { asset: text("USD_TEST_ASSET"), from: arg.actor, to: text("lender"), amount: amount(10, USD_micro) };
 emit DueSettled { due_id: text("instant"), debtor: arg.actor, creditor: text("lender"), denomination: text("USD_micro"), amount: amount(10, USD_micro), asset: text("USD_TEST_ASSET") };
 }
 }`;
 const f=setup('loan',src);const c=candidate(f.sim.simulate(input(f,'cycle',[])));
 const record={amount:amount(10,'USD_micro'),creditor:'lender',debtor:'borrower',denomination:'USD_micro',dueId:'instant'};
 assert.deepEqual(c.body.obligationDelta,{created:[{...record,status:'Outstanding'}],settled:[{...record,status:'Settled'}]});
 assert.deepEqual(c.body.after.body.obligations,[{...record,status:'Settled'}]);
 assert.equal(f.state.body.obligations.length,0);
});

test('empty action has zero expression maxima and consumes one finite allowance',()=>{
 const src='agreement Empty profile "moriarty-bounded-atomic/1" { lifetime 2; horizon 2000000000; state done: UInt128 = uint(0); observation now: UInt128; status episode closed_when done == uint(1); status agreement no_remaining_notional; action idle(actor: Text) {} }';
 const f=setup('loan',src);assert.deepEqual(f.sim.bound.manifest.core.actions[0].resourceCounts,{effects:'0',expressionDepth:'0',expressionNodes:'0',instructions:'0',locals:'0'});
 const c=candidate(f.sim.simulate(input(f,'idle',[])));
 for(const key of ['effects','executedInstructions','expressionNodes','maximumExpressionDepth','unitComponents'])assert.equal(c.body.resourceCounts[key],'0');
 for(const key of ['canonicalUtf8Bytes','canonicalNodes','canonicalDepth'])assert.ok(BigInt(c.body.resourceCounts[key])>0n);
 assert.equal(c.body.after.body.revision,'1');assert.equal(c.body.after.body.remaining,'1');
});

for(const shape of ['hidden','symbol','prototype','getter','nested-getter','array-getter','array-prototype','proxy'])test(`original runtime ${shape} is rejected without normalization or executing user code`,async()=>{
 const f=setup('loan'),i=input(f,'accrue',[]);let touched=0,authentication=0,commits=0;
 if(shape==='hidden')Object.defineProperty(i,'extra',{value:true});
 if(shape==='symbol')i[Symbol('extra')]=true;
 if(shape==='prototype')Object.setPrototypeOf(i,{extra:true});
 if(shape==='getter'){const action=i.action;Object.defineProperty(i,'action',{get(){touched++;return action;},enumerable:true});}
 if(shape==='nested-getter'){const name=i.action.name;Object.defineProperty(i.action,'name',{get(){touched++;return name;},enumerable:true});}
 if(shape==='array-getter'){const arg=i.action.arguments[0];Object.defineProperty(i.action.arguments,'0',{get(){touched++;return arg;},enumerable:true});}
 if(shape==='array-prototype')Object.setPrototypeOf(i.action.arguments,Object.create(Array.prototype));
 const bad=shape==='proxy'?new Proxy(i,{ownKeys(target){touched++;return Reflect.ownKeys(target);},getOwnPropertyDescriptor(target,key){touched++;return Reflect.getOwnPropertyDescriptor(target,key);},get(target,key,receiver){touched++;return Reflect.get(target,key,receiver);},getPrototypeOf(target){touched++;return Reflect.getPrototypeOf(target);}}):i;
 reject(f.sim.simulate(bad),'INPUT_SCHEMA');assert.equal(touched,0);
 const backend={source:source('loan'),bounds,authenticate:async()=>{authentication++;return checks('borrower');},verifyAndCommit:async()=>{commits++;throw Error('not implemented');}};
 reject(await evaluate(f.sim.bound,bad,backend),'INPUT_SCHEMA');assert.equal(touched,0);assert.equal(authentication,0);assert.equal(commits,0);
});

test('loan settlement before accrual rejects at its exact source guard, adjacent ordered settlement succeeds',()=>{
 const f=setup('loan'),bad=input(f,'settle',settlementArgs),before=structuredClone(bad),rejected=f.sim.simulate(bad);
 reject(rejected,'GUARD_FAILED');assert.equal(rejected.diagnostics[0].message,'dues are not ready');assert.equal(rejected.diagnostics[0].stage,'11');assert.deepEqual(bad,before);
 const accrued=candidate(f.sim.simulate(input(f,'accrue',[])));candidate(f.sim.simulate(input(f,'settle',settlementArgs,'borrower',accrued.body.after)));
});
test('ExactPlan rejects extra and reordered effects next to its exact accepting plan',()=>{
 const f=setup('loan'),i=input(f,'accrue',[]),a=i.authority.statement;
 for(const key of ['allowedActions','grossDebitCaps','minimumNetCredits','permittedCalls','permittedRecipients'])delete a[key];
 Object.assign(a,{mode:'ExactPlan',schemaVersion:'moriarty-exact-plan/1',action:structuredClone(i.action),exactEffects:[],exactWrites:[]});i.authority.tag='ExactPlan';i.authority.domain='MORIARTY-SIGN-bounded-atomic/1';
 const proposal=candidate(f.sim.simulate(i,{unsignedExactPlan:true}));a.exactWrites=proposal.body.writes.map(({field,value})=>({field,value}));a.exactEffects=proposal.body.effects.map(effect=>({effect}));candidate(f.sim.simulate(i));
 const extra=structuredClone(i);extra.authority.statement.exactEffects.push(structuredClone(a.exactEffects[0]));reject(f.sim.simulate(extra),'EXACT_PLAN_MISMATCH');
 const reordered=structuredClone(i);reordered.authority.statement.exactEffects.reverse();reject(f.sim.simulate(reordered),'EXACT_PLAN_MISMATCH');
});
function obligationProgram(count=1,partyLength=1){return `agreement Capacity profile "moriarty-bounded-atomic/1" { lifetime 2; horizon 2000000000; unit C; const party: Text = text("${'p'.repeat(partyLength)}"); state done: UInt128 = uint(0); observation now: UInt128; status episode closed_when done == uint(1); status agreement no_remaining_notional; policy p targets ${Array.from({length:count},(_,j)=>`effect(create,${j},amount)`).join(',')} { unit C; derivation "fixed"; rounding none; remainder "none"; comparison "exact"; proof "capacity_claim"; } effect DueCreated { due_id: Text; debtor: Text; creditor: Text; denomination: Text; amount: Amount; } action create(actor: Text) { ${Array.from({length:count},(_,j)=>`emit DueCreated { due_id: text("new${j}"), debtor: const.party, creditor: const.party, denomination: text("C"), amount: amount(1,C) };`).join(' ')} } }`;}
function withObligations(f,count,padding=1,extra=0){return remapState(input(f,'create',[]),s=>{
 s.revision='1';s.remaining='1';s.agreementStatus=count?'Outstanding':'NoOutstanding';
 s.obligations=Array.from({length:count},(_,j)=>({amount:amount(1,'C'),creditor:'c'.repeat(padding+(j===0?extra:0)),debtor:'d'.repeat(padding),denomination:'C',dueId:'old'+j,status:'Outstanding'}));
});}
test('retained obligation capacity accepts 127 plus one and rejects 128 plus one, including settled identities',()=>{
 const f=setup('loan',obligationProgram());
 const accepted=candidate(f.sim.simulate(withObligations(f,127)));assert.equal(accepted.body.after.body.obligations.length,128);assert.equal(accepted.body.obligationDelta.created.length,1);
 const i=withObligations(f,128),before=structuredClone(i);reject(f.sim.simulate(i),'OBLIGATION_CAPACITY');assert.deepEqual(i,before);
 const tombstones=withObligations(f,128);tombstones.state=sealState({...tombstones.state.body,agreementStatus:'NoOutstanding',obligations:tombstones.state.body.obligations.map(o=>({...o,status:'Settled'}))});tombstones.authority.statement.beforeStateHash=tombstones.state.stateHash;tombstones.authority.statement.predecessors=[tombstones.state.stateHash];reject(f.sim.simulate(tombstones),'OBLIGATION_CAPACITY');
});
test('runtime counts attain the admitted instruction/local/node bounds; the next source size rejects before execution',()=>{
 const expressions=Array.from({length:64},(_,j)=>`let x${j} = ${j<32?'uint(1)+uint(1)':'uint(1)+uint(1)+uint(1)'};`).join(' ');
 const f=setup('loan',minimal(expressions)),c=candidate(f.sim.simulate(input(f,'run',[])));
 assert.equal(c.body.resourceCounts.executedInstructions,'64');assert.equal(c.body.resourceCounts.expressionNodes,'256');assert.equal(f.sim.bound.manifest.core.actions[0].resourceCounts.locals,'64');
 assert.throws(()=>setup('loan',minimal(expressions+' let x64=uint(1);')),e=>e.code==='PROGRAM_BOUNDS');
 assert.throws(()=>setup('loan',minimal(expressions.replace('let x0 = uint(1)+uint(1);','let x0 = uint(1)+uint(1)+uint(1);'))),e=>e.code==='PROGRAM_BOUNDS');
 const depth=setup('loan',minimal('let x = '+Array(16).fill('uint(1)').join('+')+';'));assert.equal(candidate(depth.sim.simulate(input(depth,'run',[]))).body.resourceCounts.maximumExpressionDepth,'16');
 assert.throws(()=>setup('loan',minimal('let x = '+Array(17).fill('uint(1)').join('+')+';')),e=>e.code==='PROGRAM_BOUNDS');
 const effects=setup('loan',obligationProgram(16));assert.equal(candidate(effects.sim.simulate(input(effects,'create',[]))).body.resourceCounts.effects,'16');assert.throws(()=>setup('loan',obligationProgram(17)),e=>e.code==='PROGRAM_BOUNDS');
});
test('registered RESULT_BOUNDS has an adjacent valid-input accepting boundary without custom registry limits',async()=>{
 const {canonicalEncode,checkEncoding}=await import('../src/codec.ts');const limits=JSON.parse(bounds),f=setup('loan',obligationProgram(16,256));
 let low=1,high=200;
 while(low+1<high){const mid=Math.floor((low+high)/2),r=f.sim.simulate(withObligations(f,112,mid));if(r.kind==='Simulation')low=mid;else {assert.ok(['RESULT_BOUNDS','INPUT_BOUNDS'].includes(r.diagnostics[0].code));high=mid;}}
 // Tune a single creditor field to hit the exact complete-wrapper byte boundary.
 let best=0;const base=candidate(f.sim.simulate(withObligations(f,112,low)));const missing=65536-Buffer.byteLength(canonicalEncode(base));assert.ok(missing>=0&&low+missing+1<=256);
 const acceptedInput=withObligations(f,112,low,missing),rejectedInput=withObligations(f,112,low,missing+1);
 checkEncoding(acceptedInput,limits.evaluationEncoding,'input');checkEncoding(rejectedInput,limits.evaluationEncoding,'input');
 const accepted=candidate(f.sim.simulate(acceptedInput));assert.equal(Buffer.byteLength(canonicalEncode(accepted)),65536);assert.equal(accepted.body.effects.length,16);assert.equal(accepted.body.after.body.obligations.length,128);
 const before=structuredClone(rejectedInput);reject(f.sim.simulate(rejectedInput),'RESULT_BOUNDS');assert.deepEqual(rejectedInput,before);
});
test('valid false client checks are replaced, while the admitted backend tuple is deeply immutable',async()=>{
 const f=setup('loan'),i=input(f,'accrue',[]);for(const key of Object.keys(i.checks))if(typeof i.checks[key]==='boolean')i.checks[key]=false;
 let authentication=0,commit=0;const backend={source:source('loan'),bounds,authenticate:async(bound,tuple)=>{authentication++;assert.equal(Object.hasOwn(tuple,'checks'),false);assert.ok(Object.isFrozen(bound)&&Object.isFrozen(bound.manifest.core.actions[0]));assert.ok(Object.isFrozen(tuple)&&Object.isFrozen(tuple.action.arguments[0].value));assert.throws(()=>{tuple.action.name='settle';},TypeError);return checks('borrower');},verifyAndCommit:async()=>{commit++;throw Error('no proof implementation');}};
 reject(await evaluate(f.sim.bound,i,backend),'PROOF_INVALID');assert.equal(authentication,1);assert.equal(commit,1);assert.equal(i.checks.signatureValid,false);
});
