import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync,mkdtempSync,mkdirSync,writeFileSync,symlinkSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join,resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
import {spawnSync} from 'node:child_process';
import {lowerCompact,prepareCompactCall,decodeCompactResult,compactSnapshotHarness} from '../src/lower-compact.ts';
import {createSimulator} from '../src/evaluate.ts';
const bounds=readFileSync(new URL('../spec/bounds.json',import.meta.url));
const sources=Object.fromEntries(['loan','swap'].map(name=>[name,readFileSync(new URL(`../spec/examples/${name}.moriarty`,import.meta.url))]));
const runtime=process.env.MORIARTY_RUNTIME_NODE_MODULES??'/home/charl/Moriarty/.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world/node_modules';
const root=mkdtempSync(join(tmpdir(),'moriarty-mapping-'));
process.on('exit',()=>rmSync(root,{recursive:true,force:true}));
function compileArtifact(name,mapping){const source=join(root,name+'.compact'),target=join(root,name);writeFileSync(source,mapping.source);const r=spawnSync('compact',['compile','--skip-zk','--compact-path',resolve(new URL('../compact/',import.meta.url).pathname),source,target],{encoding:'utf8',timeout:90000});assert.equal(r.status,0,r.stdout+r.stderr);symlinkSync(runtime,join(target,'node_modules'),'dir');return import(pathToFileURL(join(target,'contract/index.js')).href);}
const str=value=>({tag:'Text',value}),uint=value=>({tag:'UInt128',value:String(value)}),amount=(value,unit)=>({tag:'Amount',value:String(value),unit});
const named=(name,value)=>({name,value});
const swapArgs=min=>[named('actor',str('trader')),named('recipient',str('trader')),named('asset_in',str('ASSET_A')),named('asset_out',str('ASSET_B')),named('amount_in',amount(10000,'AssetA_quantum')),named('min_out',amount(min,'AssetB_quantum'))];
const settleArgs=[named('actor',str('borrower')),named('settlement_asset',str('USD_TEST_ASSET')),named('amount_due',amount(533972602,'USD_micro'))];
const obs=[named('now',uint(1))];
const calls={loan:[['accrue',[named('actor',str('borrower'))]],['settle',settleArgs]],swap:[['swap',swapArgs(0)],['close',[named('actor',str('provider'))]]]};
function simulationInput(sim,genesis,state,name,args){const actor=args.find(x=>x.name==='actor').value.value;return {action:{arguments:args,name,schemaVersion:'moriarty-action/1'},authority:{domain:'MORIARTY-OUTCOME-bounded-atomic/1',schemaVersion:'moriarty-authority/1',signature:{algorithm:'simulation-only',bytes:'',keyId:actor},tag:'IntentRefinement',statement:{allowedActions:sim.bound.manifest.core.actions.map(a=>a.name),grossDebitCaps:sim.bound.manifest.settlementBindings.map(b=>({actor,asset:b.asset,maximumLedgerAmount:String((1n<<128n)-1n)})).sort((a,b)=>a.asset.localeCompare(b.asset)),minimumNetCredits:[],beforeStateHash:state.stateHash,domain:genesis.body.domain,genesisHash:genesis.genesisHash,instanceId:'mapping',mode:'IntentRefinement',nonce:state.body.revision,permittedCalls:[],permittedRecipients:['borrower','lender','pool','provider','trader'],predecessors:[state.stateHash],principal:actor,program:sim.program,requiredClaimRoot:genesis.body.requiredClaimRoot,requiredClaims:sim.bound.manifest.requiredClaims,schemaVersion:'moriarty-outcome-intent/1',validity:{notBefore:'0',notAfterExclusive:'2000000000'}}},checks:{authenticatedPrincipal:actor,genesisValid:true,nonceFresh:true,observationsAuthentic:true,predecessorSetValid:true,signatureValid:true,stateCurrentAndUnconsumed:true},genesis,observations:{observations:[{name:'now',provider:'clock',evidenceDigest:'0'.repeat(64),value:uint(1)}],schemaVersion:'moriarty-observations/1'},program:sim.program,schemaVersion:'moriarty-evaluation/1',state};}

test('generic mapper compiles full loan and swap functions and matches all state/effect operands for four transitions',async()=>{
 for(const name of ['loan','swap']){
  const mapping=lowerCompact(sources[name],bounds),artifact=await compileArtifact(name,mapping),sim=createSimulator(sources[name],bounds);
  const genesis=sim.makeGenesis({domain:{network:'simulation',deployment:'mapping'},instanceId:'mapping',principalBindings:['borrower','lender','pool','provider','trader'].map(actor=>({actor,principal:actor})),observationBindings:[{name:'now',provider:'clock',authenticationPolicy:'external'}]});
  let state=sim.initialState(genesis);
  for(const [action,args] of calls[name]){
   const call=prepareCompactCall(mapping,action,state.body.values,args,obs,state.body.remaining,state.body.revision);
   const output=artifact.pureCircuits[call.circuit](...call.arguments),decoded=decodeCompactResult(mapping,action,output);
   const simulation=sim.simulate(simulationInput(sim,genesis,state,action,args));assert.equal(simulation.kind,'Simulation',JSON.stringify(simulation));
   assert.deepEqual(decoded.values,simulation.candidate.body.after.body.values);
   assert.equal(decoded.remaining,simulation.candidate.body.after.body.remaining);assert.equal(decoded.revision,simulation.candidate.body.after.body.revision);
   const operands=simulation.candidate.body.effects.map(({settlement,...effect})=>effect);assert.deepEqual(decoded.effects,operands);
   state=simulation.candidate.body.after;
  }
 }
});

test('every supplied arithmetic hint is constrained; minOut, actor, state order and lifecycle changes reject',async()=>{
 for(const name of ['loan','swap']){
  const mapping=lowerCompact(sources[name],bounds),artifact=await compileArtifact(name+'-negative',mapping),[action,args]=calls[name][0];
  const call=prepareCompactCall(mapping,action,mapping.bound.manifest.initialState,args,obs,mapping.bound.manifest.lifetime,'0');
  const run=arguments_=>artifact.pureCircuits[call.circuit](...arguments_);
  for(const h of mapping.metadata.actions[0].hints){
   const paths=h.kind==='Mul'?['aLo','aHi','bLo','bHi','lo','carry'].map(k=>[k]):[['q'],['r'],...['aLo','aHi','bLo','bHi','lo','carry'].map(k=>['product',k])];
   for(const path of paths){const changed=structuredClone(call.arguments);let value=changed[5][h.field];for(const k of path.slice(0,-1))value=value[k];value[path.at(-1)]+=1n;assert.throws(()=>run(changed),`${name} ${h.nodeId} ${path.join('.')}`);}
  }
  const actorField=mapping.metadata.actions[0].arguments.find(x=>x.name==='actor').field;
  const badActor=structuredClone(call.arguments);badActor[1][actorField]=BigInt(mapping.metadata.textTable.find(x=>x.text===(name==='loan'?'lender':'provider')).id);assert.throws(()=>run(badActor));
  const exhausted=structuredClone(call.arguments);exhausted[3]=0n;exhausted[4]=BigInt(mapping.bound.manifest.lifetime);assert.throws(()=>run(exhausted));
  const reset=structuredClone(call.arguments);reset[4]=1n;assert.throws(()=>run(reset));
  const expired=structuredClone(call.arguments);expired[2].o0=2000000000n;assert.throws(()=>run(expired));
  const reordered=structuredClone(call.arguments);[reordered[0].f0,reordered[0].f1]=[reordered[0].f1,reordered[0].f0];assert.throws(()=>run(reordered));
  if(name==='swap'){const badMin=structuredClone(call.arguments);badMin[1][mapping.metadata.actions[0].arguments.find(x=>x.name==='min_out').field]=19744n;assert.throws(()=>run(badMin));const reserved=structuredClone(call.arguments);reserved[3]=1n;reserved[4]=7n;assert.throws(()=>run(reserved));}
 }
});
const small=body=>`agreement Generic profile "moriarty-bounded-atomic/1" { lifetime 2; horizon 2000000000; state closed: UInt128 = uint(0); observation now: UInt128; status episode closed_when closed == uint(1); status agreement no_remaining_notional; action run(actor: Text) { guard arg.actor == text("123"), "actor"; ${body} } }`;
test('unsupported text and short-circuit forms fail closed, while numeric-looking Text remains nominal',()=>{
 const source=small('let t = text("é"); let u = text("😀"); let n = uint(123);');const a=lowerCompact(source,bounds),b=lowerCompact(source.replace('agreement Generic','agreement Renamed'),bounds);
 assert.deepEqual(a.metadata.textTable.map(x=>x.text),['123','é','😀']);assert.notEqual(a.metadata.programHash,b.metadata.programHash);assert.deepEqual(a.metadata.actions,b.metadata.actions);
 for(const [src,code] of [[small('').replace('guard arg.actor == text("123"), "actor";',''),'MAPPING_TEXT_ARGUMENT_UNGUARDED'],[small('let b = true or false;'),'MAPPING_SHORT_CIRCUIT_UNSUPPORTED'],[small('').replace('state closed:', 'state note: Text = text("x"); state closed:'),'MAPPING_PERSISTENT_TEXT_UNSUPPORTED'],[small('').replace('observation now:', 'observation word: Text; observation now:'),'MAPPING_OBSERVATION_TEXT_UNSUPPORTED']])assert.throws(()=>lowerCompact(src,bounds),e=>e.code===code);
});
test('dynamic compiled overflow is rejected before a later fitting division',async()=>{
 const src=small('let result = floor_div(arg.n * uint(2),uint(2)); set closed = result;').replace('actor: Text)','actor: Text, n: UInt128)');
 const mapping=lowerCompact(src,bounds),artifact=await compileArtifact('overflow',mapping),args=[named('actor',str('123')),named('n',uint((1n<<128n)-1n))];
 const call=prepareCompactCall(mapping,'run',mapping.bound.manifest.initialState,args,obs,'2','0');assert.throws(()=>artifact.pureCircuits.transition0(...call.arguments));
 const good=prepareCompactCall(mapping,'run',mapping.bound.manifest.initialState,[named('actor',str('123')),named('n',uint(7))],obs,'2','0');assert.equal(artifact.pureCircuits.transition0(...good.arguments).after.f0,7n);
});
test('source map covers every Core expression and numeric unit metadata survives lowering',()=>{
 for(const name of ['loan','swap']){const mapping=lowerCompact(sources[name],bounds);const expressions=mapping.bound.manifest.core.actions.reduce((n,a)=>n+Number(a.resourceCounts.expressionNodes),0);assert.equal(mapping.metadata.sourceMap.length,expressions);assert.equal(new Set(mapping.metadata.sourceMap.map(x=>x.coreNodeId)).size,expressions);for(const entry of mapping.metadata.sourceMap){assert.equal(entry.sourceRef.sourceHash,mapping.metadata.sourceHash);assert.match(mapping.source.split('\n')[Number(entry.generatedLine)-1],/const e[0-9]+:/);}if(name==='swap')assert.ok(mapping.metadata.sourceMap.some(x=>x.type.tag==='Quantity'&&x.unitVector.length===2));}
});
test('test-only snapshot harness compiles ZKIR and records complete kernel result without mutating failed input state',async()=>{
 const runtimeModule=await import(pathToFileURL(join(runtime,'@midnight-ntwrk/compact-runtime/dist/index.js')).href);
 for(const name of ['loan','swap']){
  const mapping=lowerCompact(sources[name],bounds);writeFileSync(join(root,name+'-snapshot-kernel.compact'),mapping.source);
  const wrapper={...mapping,source:compactSnapshotHarness(mapping,name+'-snapshot-kernel')},artifact=await compileArtifact(name+'-snapshot',wrapper);
  const contract=new artifact.Contract({}),zero='00'.repeat(32),initial=contract.initialState(runtimeModule.createConstructorContext({},zero));
  const context=()=>runtimeModule.createCircuitContext(runtimeModule.dummyContractAddress(),zero,initial.currentContractState,{});
  const [action,args]=calls[name][0],call=prepareCompactCall(mapping,action,mapping.bound.manifest.initialState,args,obs,mapping.bound.manifest.lifetime,'0');
  const output=contract.circuits.record0(context(),...call.arguments);const snapshot=artifact.ledger(output.context.currentQueryContext.state).snapshot0;
  assert.deepEqual(snapshot,artifact.pureCircuits.transition0(...call.arguments));
  const changed=structuredClone(call.arguments),h=mapping.metadata.actions[0].hints[0];changed[5][h.field].aLo+=1n;assert.throws(()=>contract.circuits.record0(context(),...changed));
  assert.equal(artifact.ledger(initial.currentContractState.data).snapshot0.remaining,0n);
  const zkir=readFileSync(join(root,name+'-snapshot','zkir','record0.zkir'),'utf8');assert.ok(zkir.length>1000);
 }
});
