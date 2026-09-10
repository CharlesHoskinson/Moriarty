import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {pathToFileURL} from 'node:url';
import {registerHooks} from 'node:module';
const R='/home/charl/Moriarty/.worktrees/sp05-local-completion';
const D=R+'/deliverables/sp05-financial-integration-2026-09-09/local-command-execution-proposal-01/loan', O=D+'/historical-readback-01';
const j=p=>JSON.parse(readFileSync(p)), sha=x=>createHash('sha256').update(x).digest('hex');
const pub=x=>typeof x==='bigint'?x.toString():x instanceof Uint8Array?Buffer.from(x).toString('hex'):Array.isArray(x)?x.map(pub):x&&typeof x==='object'?Object.fromEntries(Object.entries(x).map(([k,v])=>[k,pub(v)])):x;
const candidate=j(O+'/actual-result-candidate-01.json');
const verifyPins=()=>{assert.equal(sha(readFileSync(O+'/actual-result-candidate-01.json')),'395d7b8563cb5575ffeea848817b80cc3dc23bf08e9b8631cbaa377dc8b48f07');assert.equal(Object.keys(candidate.files).length,228);for(const [p,h] of Object.entries(candidate.files))assert.equal(sha(readFileSync(R+'/'+p)),h,p);return 228;};
const pinsBefore=verifyPins();
const plan=j(D+'/../loan-public-plan-draft.json'),build=j(plan.build.receiptPath),e=j(D+'/financial-expectations-used-01.json').cases.loan;
assert.equal(sha(readFileSync(plan.build.receiptPath)),plan.build.receiptSha256);
assert.equal(build.sourceManifestHash,plan.build.sourceManifestHash);
assert.equal(sha(readFileSync(D+'/financial-expectations-used-01.json')),j(D+'/expectation-source-binding-01.json').sha256);
const moduleBytes=readFileSync(build.contractModulePath),modulePin=build.artifacts.find(x=>x.path==='contract/index.js');assert.equal(sha(moduleBytes),modulePin.sha256);
const {PINNED_NM}=await import(pathToFileURL(R+'/experiments/moriarty-midnight-financial/ledger/providers.mjs'));
const ledger=await import(pathToFileURL(PINNED_NM+'/@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs'));
const runtime=await import(pathToFileURL(PINNED_NM+'/@midnight-ntwrk/midnight-js-protocol/dist/compact-runtime.mjs'));
const {decodeNativeFinancialTransaction}=await import(pathToFileURL(R+'/experiments/moriarty-midnight-financial/ledger/receipt.mjs'));
// Import only the checked generated public decoder; never read role preimages,
// private wallet files, proving keys, or run compiler/prover/contract circuits.
const moduleUrl=pathToFileURL(build.contractModulePath).href;
const hooks=registerHooks({resolve(s,c,n){if(c.parentURL?.startsWith(moduleUrl)){assert.equal(s,'@midnight-ntwrk/compact-runtime');return {url:pathToFileURL(PINNED_NM+'/@midnight-ntwrk/compact-runtime/dist/index.js').href,shortCircuit:true};}return n(s,c);},load(u,c,n){if(u.startsWith(moduleUrl))return {format:'module',source:moduleBytes,shortCircuit:true};return n(u,c);}});
let generated;try{generated=await import(moduleUrl+'?independentActualResult=01');}finally{hooks.deregister();}
const p=j(O+'/probe-result.json'),result=j(D+'/actual-run/integration-result.json');
assert.equal(sha(readFileSync(O+'/probe-result.json')),'ff890eea9d2a4d12fbada60dbfc437ef9730cc6216b8cde94d567a385fb638e9');
const address=result.driver.stages[0].contractAddress;
const color=runtime.rawTokenType(Buffer.from(e.bindingParameters.assets.USD_TEST_ASSET.domainHex,'hex'),address);
const report={status:'PASS',pinsBefore,buildReceiptSha256:plan.build.receiptSha256,compiledModuleSha256:sha(moduleBytes),scope:'Offline independent checks of retained native RPC bytes, compiled public fields, original native transactions, and exact arithmetic; no proof verification or authenticated state proof',states:[],transactions:[]};
const nativeTransactions=[];
for(const stage of result.driver.stages){
 const raw=readFileSync(D+'/actual-run/public-transactions/'+stage.transaction.rawSha256+'.bin');
 const decoded=decodeNativeFinancialTransaction(raw,ledger);assert.deepEqual(decoded,stage.transaction);assert(decoded.identifiers.includes(stage.txId));assert.deepEqual([...decoded.identifiers].sort(),[...stage.indexerIdentifiers].sort());
 assert.equal(decoded.actions.length,1);assert.equal(decoded.actions[0].address,address);assert.equal(decoded.actions[0].kind,stage.circuitId==='deploy'?'deploy':'call');if(stage.circuitId!=='deploy')assert.equal(decoded.actions[0].entryPoint,stage.circuitId);
 assert.equal(decoded.proofVerified,false);assert.equal(decoded.ledgerAccepted,false);
 nativeTransactions.push(decoded);report.transactions.push({stage:stage.circuitId,rawSha256:sha(raw),bytes:raw.length,identifiers:decoded.identifiers,inputs:decoded.inputs,outputs:decoded.outputs,nativeEffects:decoded.actions[0].transcripts??[],nativeDustDebitSpeck:decoded.dustFee});
}
const initialTx=ledger.Transaction.deserialize('signature','proof','binding',readFileSync(D+'/actual-run/public-transactions/'+nativeTransactions[0].rawSha256+'.bin'));
const initialAction=[...initialTx.intents.values()].flatMap(x=>x.actions)[0];assert.equal(initialAction.address,address);
const initialHex=Buffer.from(initialAction.initialState.serialize()).toString('hex');initialTx.free();
const decodedStates=[];
assert.equal(p.states.length,4);assert.equal(p.responses.length,17);assert.equal(p.queries,17);assert(p.totalQueries<=77&&p.elapsedMs<=60000);assert.equal(p.submissions,0);assert.equal(p.proofs,0);
let resultBytes=0;for(const r of p.responses){const raw=Buffer.from(JSON.stringify(r.result));assert.equal(sha(raw),r.resultJsonSha256);assert.equal(raw.length,r.resultJsonBytes);assert(raw.length<=65536);resultBytes+=raw.length;}assert.equal(resultBytes,p.resultBytes);
const response=(i,m,params)=>{assert.equal(p.responses[i].method,m);assert.deepEqual(p.responses[i].params,params);return p.responses[i].result;};
assert.equal(response(0,'chain_getBlockHash',[0]),'0x'+plan.networkTag);
assert.equal(response(1,'chain_getFinalizedHead',[]),p.finalizedHash);assert.equal(Number(BigInt(response(2,'chain_getHeader',[p.finalizedHash]).number)),p.finalizedHeight);assert.equal(p.finalizedHeight,20488);assert.equal(response(3,'chain_getBlockHash',[p.finalizedHeight]),p.finalizedHash);assert.equal(response(16,'chain_getBlockHash',[p.finalizedHeight]),p.finalizedHash);
for(let i=0;i<4;i++){
 const s=p.states[i],name=e.stageOrder[i],stage=result.driver.stages[i],summary=j(D+'/actual-run/stage-'+name+'.json'),expect=e.stages[name];assert.equal(s.stage,name);assert.equal(s.blockHeight,[20455,20459,20463,20467][i]);assert.equal(s.blockHeight,stage.blockHeight);assert.equal(s.blockHash,'0x'+stage.blockHash);assert(s.blockHeight<=p.finalizedHeight);assert.equal(stage.finalizedHead,s.blockHash);assert.equal(stage.finalizedHeight,s.blockHeight);
 assert.equal(response(4+3*i,'chain_getBlockHash',[s.blockHeight]),s.blockHash);assert.equal(response(6+3*i,'chain_getBlockHash',[s.blockHeight]),s.blockHash);
 const hex=response(5+3*i,'midnight_contractState',[address,s.blockHash]);assert.equal(hex,s.serializedStateHex);const raw=Buffer.from(hex,'hex'),native=runtime.ContractState.deserialize(raw);
 try{
  assert(Buffer.from(native.serialize()).equals(raw));assert.equal(sha(raw),s.stateSha256);assert.equal(raw.length,s.nativeBytes);
  const state=pub(generated.ledger(native.data));assert.deepEqual(state,summary.publicState);assert.deepEqual(state,s.state);decodedStates.push(state);
  for(const k of ['kernelState','remaining','revision','initialized'])assert.deepEqual(state[k],expect[k]);for(const [k,v] of Object.entries(expect.lastResults))assert.deepEqual(state[k],v);
  assert.equal(state.programDigest,e.bindingParameters.programDigest);assert.equal(state.networkTag,plan.networkTag);assert.equal(state.borrowerAddress.bytes,plan.roles.firstAddress);assert.equal(state.lenderAddress.bytes,plan.roles.secondAddress);assert.notEqual(state.borrowerAddress.bytes,state.lenderAddress.bytes);
  assert.equal(state.usdDomain,e.bindingParameters.assets.USD_TEST_ASSET.domainHex);assert.equal(state.usdColor,i===0?'0'.repeat(64):color);
  for(const k of ['borrowerCapability','lenderCapability']){assert.match(state[k],/^[a-f0-9]{64}$/);assert.equal(state[k],decodedStates[0][k]);}
  const balances={};for(const [t,v] of native.balance){assert.equal(t.tag,'unshielded');assert.equal(t.raw,color);assert.equal(v,0n);balances[t.raw]=v.toString();}assert.deepEqual(balances,s.balances);for(const k of new Set([...Object.keys(balances),...Object.keys(summary.contractBalances)]))assert.equal(BigInt(balances[k]??0),BigInt(summary.contractBalances[k]??0));
  assert.deepEqual([...native.operations()].sort(),['accrue','initialize','settle']);const keys=[];for(const op of ['accrue','initialize','settle']){const key=readFileSync(build.assetsPath+'/keys/'+op+'.verifier'),pin=build.artifacts.find(x=>x.path==='keys/'+op+'.verifier');assert.equal(sha(key),pin.sha256);const operation=native.operation(op);try{assert(Buffer.from(operation.verifierKey).equals(key));keys.push({operation:op,sha256:sha(key)});}finally{operation.free();}}
  if(i===0)assert.equal(hex,initialHex);assert(Buffer.from(native.serialize()).equals(raw));
  // Independent financial effects and net cash comparisons, not summary PASS.
  const tx=nativeTransactions[i],effects={unshieldedMints:{},unshieldedInputs:{},unshieldedOutputs:{},claimedUnshieldedSpends:[]};
  for(const t of tx.actions[0].transcripts??[]){for(const k of ['unshieldedMints','unshieldedInputs','unshieldedOutputs'])for(const [asset,n] of Object.entries(t.effects[k]))effects[k][asset]=(BigInt(effects[k][asset]??0)+BigInt(n)).toString();effects.claimedUnshieldedSpends.push(...t.effects.claimedUnshieldedSpends);}
  assert.deepEqual(effects,summary.nativeEffects);
  for(const k of ['unshieldedMints','unshieldedInputs','unshieldedOutputs']){const asset=k==='unshieldedMints'?state.usdDomain:color;assert.equal(BigInt(effects[k][asset]??0),BigInt(expect.nativeEffects[k].USD_TEST_ASSET));assert(Object.keys(effects[k]).every(x=>x===asset));}
  assert.deepEqual(effects.claimedUnshieldedSpends,expect.nativeEffects.claimedUnshieldedSpends.map(x=>({type:color,recipientKind:x.recipientKind,recipient:x.recipientRole==='borrower'?plan.roles.firstAddress:plan.roles.secondAddress,amount:x.amount})));
  const deltas={borrower:0n,lender:0n};for(const [direction,rows] of [[-1n,tx.inputs],[1n,tx.outputs]])for(const row of rows){assert.equal(row.type,color);const role=row.owner===plan.roles.firstAddress?'borrower':row.owner===plan.roles.secondAddress?'lender':null;assert(role);deltas[role]+=direction*BigInt(row.value);}
  for(const role of ['borrower','lender'])assert.equal(deltas[role].toString(),expect.participantNetDeltas[role].USD_TEST_ASSET);
  assert.equal(deltas.borrower+deltas.lender,BigInt(expect.mintAmounts.USD_TEST_ASSET));
  report.states.push({stage:name,height:s.blockHeight,blockHash:s.blockHash,stateSha256:sha(raw),nativeBytes:raw.length,compiledFieldCount:Object.keys(state).length,allFieldsEqual:true,kernelState:state.kernelState,remaining:state.remaining,revision:state.revision,balances,verifierKeys:keys,participantNetDeltas:pub(deltas)});
 }finally{native.free();}
}
const [deploy,init,accrue,settle]=nativeTransactions;assert.equal(deploy.inputs.length+deploy.outputs.length+accrue.inputs.length+accrue.outputs.length,0);assert.equal(init.outputs.length,1);assert.equal(settle.inputs.length,1);assert.equal(settle.outputs.length,2);const input=settle.inputs[0],origin=init.outputs[0];for(const key of ['owner','type','value','intentHash'])assert.equal(input[key],origin[key]);assert.equal(input.outputNo,origin.offerIndex);
const principal=500000000n,interest=5000000000n*8n*31n/(100n*365n),payment=principal+interest,change=20000000000n-payment;
assert.equal(interest,33972602n);assert.equal(BigInt(input.value),20000000000n);assert.equal(BigInt(settle.outputs[0].value),payment);assert.equal(settle.outputs[0].owner,plan.roles.secondAddress);assert.equal(BigInt(settle.outputs[1].value),change);assert.equal(settle.outputs[1].owner,plan.roles.firstAddress);assert.equal(payment+change,BigInt(input.value));
assert.equal(decodedStates[3].kernelState.f0,(5000000000n-principal).toString());assert.equal(decodedStates[3].kernelState.f3,principal.toString());assert.equal(decodedStates[3].kernelState.f4,interest.toString());assert.equal(decodedStates[3].kernelState.f6,payment.toString());assert.equal(decodedStates[3].kernelState.f5,change.toString());assert.equal(decodedStates[3].kernelState.f8,'1');
const totalDust=nativeTransactions.reduce((n,x)=>n+BigInt(x.dustFee),0n);assert.equal(totalDust,1200000000000004n);assert.equal(candidate.priorAccounting.reservedDustSpeck,(8100000000000027n+totalDust).toString());assert.equal(candidate.priorAccounting.reservedSubmissions,27+4);
report.arithmetic=pub({interestNumerator:5000000000n*8n*31n,interestDenominator:36500n,interest,principal,payment,borrowerChange:change,residualPrincipal:4500000000n,totalNativeDustDebitSpeck:totalDust});report.finality={finalizedHeight:p.finalizedHeight,finalizedHash:p.finalizedHash,canonicalChecksBeforeAfter:true,queries:p.queries,totalQueries:p.totalQueries,elapsedMs:p.elapsedMs,authenticatedStateProof:false};report.pinsAfter=verifyPins();
writeFileSync(O+'/gpt6-independent-offline-evidence-01.json',JSON.stringify(report,null,2)+'\n',{flag:'wx'});
console.log(JSON.stringify({status:'PASS',states:4,nativeTransactions:4,checkedInputSignatures:1,verifierKeysCompared:12,pinsBefore,pinsAfter:report.pinsAfter,arithmetic:report.arithmetic}));
