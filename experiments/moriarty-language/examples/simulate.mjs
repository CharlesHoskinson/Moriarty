import {readFileSync} from 'node:fs';
import {createSimulator, evaluate} from '../src/evaluate.ts';

// This example supplies simulated authentication. It never signs, proves,
// submits, or consumes a state. Production acceptance needs the MC04/05 backend.
const text=value=>({tag:'Text',value});
const uint=value=>({tag:'UInt128',value:String(value)});
const amount=(value,unit)=>({tag:'Amount',unit,value:String(value)});
const named=(name,value)=>({name,value});
const bounds=readFileSync(new URL('../spec/bounds.json',import.meta.url));
const actors=['borrower','lender','pool','provider','trader'];
const records=[];

for(const name of ['loan','swap']) {
  const source=readFileSync(new URL(`../spec/examples/${name}.moriarty`,import.meta.url));
  const sim=createSimulator(source,bounds);
  const genesis=sim.makeGenesis({
    domain:{network:'simulation',deployment:'developer-example'},
    instanceId:`${name}-example`,
    principalBindings:actors.map(actor=>({actor,principal:actor})),
    observationBindings:[{name:'now',provider:'example-clock',authenticationPolicy:'simulation-only'}]
  });
  let state=sim.initialState(genesis);
  const calls=name==='loan' ? [
    ['accrue','borrower',[]],
    ['settle','borrower',[named('settlement_asset',text('USD_TEST_ASSET')),named('amount_due',amount(533972602,'USD_micro'))]]
  ] : [
    ['swap','trader',[named('recipient',text('trader')),named('asset_in',text('ASSET_A')),named('asset_out',text('ASSET_B')),named('amount_in',amount(10000,'AssetA_quantum')),named('min_out',amount(19743,'AssetB_quantum'))]],
    ['close','provider',[]]
  ];
  const steps=[];
  for(const [actionName,actor,args] of calls) {
    const action={schemaVersion:'moriarty-action/1',name:actionName,arguments:[named('actor',text(actor)),...args]};
    const common={
      beforeStateHash:state.stateHash,domain:genesis.body.domain,genesisHash:genesis.genesisHash,
      instanceId:genesis.body.instanceId,nonce:`${name}-${state.body.revision}`,predecessors:[state.stateHash],
      principal:actor,program:sim.program,requiredClaimRoot:genesis.body.requiredClaimRoot,
      requiredClaims:sim.bound.manifest.requiredClaims,validity:{notBefore:'0',notAfterExclusive:'2000000000'}
    };
    const signature={algorithm:'simulation-only',bytes:'',keyId:actor};
    const authority=name==='loan' ? {
      domain:'MORIARTY-SIGN-bounded-atomic/1',schemaVersion:'moriarty-authority/1',tag:'ExactPlan',signature,
      statement:{...common,action,exactEffects:[],exactWrites:[],mode:'ExactPlan',schemaVersion:'moriarty-exact-plan/1'}
    } : {
      domain:'MORIARTY-OUTCOME-bounded-atomic/1',schemaVersion:'moriarty-authority/1',tag:'IntentRefinement',signature,
      statement:{...common,allowedActions:[actionName],mode:'IntentRefinement',schemaVersion:'moriarty-outcome-intent/1',
        grossDebitCaps:sim.bound.manifest.settlementBindings.map(b=>({actor,asset:b.asset,maximumLedgerAmount:actionName==='swap'?'10000':'2000000'})).sort((a,b)=>a.asset<b.asset?-1:1),
        minimumNetCredits:actionName==='swap'?[{actor,asset:'ASSET_B',minimumLedgerAmount:'19743'}]:[],
        permittedCalls:[],permittedRecipients:actors}
    };
    const input={action,authority,genesis,program:sim.program,state,schemaVersion:'moriarty-evaluation/1',
      observations:{schemaVersion:'moriarty-observations/1',observations:[{name:'now',provider:'example-clock',evidenceDigest:'0'.repeat(64),value:uint(1)}]},
      checks:{authenticatedPrincipal:actor,genesisValid:true,nonceFresh:true,observationsAuthentic:true,predecessorSetValid:true,signatureValid:true,stateCurrentAndUnconsumed:true}
    };
    if(authority.tag==='ExactPlan') {
      const proposal=sim.simulate(input,{unsignedExactPlan:true});
      if(proposal.kind!=='Simulation')throw Error(JSON.stringify(proposal));
      authority.statement.exactWrites=proposal.candidate.body.writes.map(({field,value})=>({field,value}));
      authority.statement.exactEffects=proposal.candidate.body.effects.map(effect=>({effect}));
    }
    const result=sim.simulate(input);
    if(result.kind!=='Simulation')throw Error(JSON.stringify(result));
    const missingProof=await evaluate(sim.bound,input);
    if(missingProof.outcome!=='Rejected'||missingProof.diagnostics[0].code!=='PROOF_INVALID')throw Error('Acceptance did not fail closed');
    const tampered=structuredClone(input);
    if(name==='loan')tampered.authority.statement.exactWrites[0].value.value='0';
    else if(actionName==='swap')tampered.action.arguments.at(-1).value.value='19744';
    else tampered.action.arguments[0].value=text('trader');
    const rejected=sim.simulate(tampered);
    if(rejected.outcome!=='Rejected')throw Error('Adverse example did not reject');
    steps.push({action:actionName,authorityMode:authority.tag,input,result,adverseResult:rejected,acceptanceWithoutProof:missingProof});
    state=result.candidate.body.after;
  }
  records.push({name,programHash:sim.bound.programHash,steps});
}
if(process.argv.includes('--json'))console.log(JSON.stringify({scope:'local-simulation-only',records},null,2));
else for(const record of records) {
  console.log(`${record.name}: ${record.programHash}`);
  for(const step of record.steps) {
    const b=step.result.candidate.body;
    console.log(`  ${step.action} (${step.authorityMode}): ${b.writes.length} writes, ${b.effects.length} effects; episode ${b.after.body.episodeStatus}, agreement ${b.after.body.agreementStatus}`);
    for(const effect of b.effects)console.log(`    ${effect.kind}: ${effect.amount.value} ${effect.amount.unit}`);
    console.log(`    adverse input: ${step.adverseResult.diagnostics[0].code}; acceptance without proof: ${step.acceptanceWithoutProof.diagnostics[0].code}`);
  }
}
