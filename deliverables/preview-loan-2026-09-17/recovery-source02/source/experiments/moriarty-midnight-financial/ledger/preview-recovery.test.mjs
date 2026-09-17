/** Offline recovery regressions: actual published native deployment bytes,
 * controlled chain/wallet/constructor callbacks; no private files or network. */
import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync,mkdtempSync,mkdirSync,rmSync,writeFileSync,readdirSync} from 'node:fs';
import {join} from 'node:path';
import {tmpdir} from 'node:os';
import {pathToFileURL} from 'node:url';
import {PINNED_NM} from './providers.mjs';
import * as recovery from './recover-deployment.mjs';
import {validatePreviewLaunchPlan} from './preview-launch.mjs';
import {inspectUnwrittenLoanStore} from './recover-store.mjs';
import {integratePreviewFinancialCase} from './integrate-preview.mjs';
import {runPreviewFinancialCase} from './run-local.mjs';
const fixed=recovery.PREVIEW_LOAN_RECOVERY;
const directory=new URL('../../../deliverables/preview-loan-2026-09-17/actual-run01/',import.meta.url);
const raw=readFileSync(new URL('public-transactions/'+fixed.transactionHash+'.bin',directory));
const ledger=await import(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs')).href);
const stateRuntime=await import(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/midnight-js-protocol/dist/compact-runtime.mjs')).href);
const native=()=>ledger.Transaction.deserialize('signature','proof','binding',raw);
const initial=()=>stateRuntime.ContractState.deserialize([...native().intents.values()].flatMap(i=>i.actions)[0].initialState.serialize());
function plan(){
 const p=JSON.parse(readFileSync(new URL('plan.json',directory)));
 p.outputDirectory='/tmp/moriarty-preview-recovery-test/run';p.privateState.directory='/tmp/moriarty-preview-recovery-test/contract';
 p.limits={...p.limits,allocationId:'sp05-preview-loan-recovery-test',submissions:3,dustFee:fixed.remainingDustFee,deadlineMs:Date.now()+30000};
 p.deploymentRecovery={schema:'moriarty.preview-deployment-recovery/1',transactionFile:join(fixed.originalDirectory,'run/public-transactions',fixed.transactionHash+'.bin'),inspectionDirectory:'/tmp/moriarty-preview-recovery-test/inspection'};
 p.submissionTransport='preview-ws';return p;
}
function publicFixture(){
 const p=plan(),binding=recovery.inspectPreviewLoanBytes(raw,ledger),height=fixed.blockHeight,hash=fixed.blockHash;
 const data={tx:native(),status:'SucceedEntirely',txId:fixed.txId,txHash:fixed.transactionHash,identifiers:[...fixed.identifiers],blockHeight:height,blockHash:hash,protocolVersion:1000000,fees:{paidFees:'0',estimatedFees:'0'},unshielded:{created:[],spent:[]}};
 const provider={watchForTxData:async()=>data,queryContractState:async()=>initial()};
 const rpc=async(method,args)=>method==='chain_getHeader'?{number:'0x'+height.toString(16)}:method==='chain_getBlockHash'&&args[0]===0?'0x'+p.networkTag:'0x'+hash;
 const tip={status:'READY',hash,height,finalizedHash:'0x'+hash,finalizedHeight:height,timestampMs:Date.now()};
 const options={binding,ledger,provider,rpc,decodeState:()=>({sourceTestOnly:true}),deadlineMs:p.limits.deadlineMs,expectedProtocolVersion:1000000,networkTag:p.networkTag,tip,readCurrentTip:async()=>({...tip})};
 return {options,data};
}
test('closed Preview recovery admits exactly original build, wallet, network and remaining conservative budget',()=>{
 const p=plan();assert.deepEqual(validatePreviewLaunchPlan(p),p);
 assert.equal(BigInt(p.limits.dustFee)+BigInt(fixed.conservativeReservedDustFee),2000000000000000n);
 for(const mutate of [p=>p.wallet.expectedAddress='changed',p=>p.networkTag='00'.repeat(32),p=>p.build.receiptSha256='00'.repeat(32),p=>p.build.sourceManifestHash='00'.repeat(32),p=>p.roles.firstAddress='00'.repeat(32),p=>p.roles.secretsFile='/tmp/substitute',p=>p.limits.dustFee='1400000000000000',p=>p.limits.submissions=4,p=>p.limits.allocationId='sp05-preview-loan-20260917-01',p=>p.privateState.directory=fixed.originalDirectory+'/contract-state',p=>p.deploymentRecovery.transactionFile='/tmp/substitute',p=>p.deploymentRecovery.progress=1]){
  const p=plan();mutate(p);assert.throws(()=>validatePreviewLaunchPlan(p));
 }
});
test('actual published native bytes bind fee, address, whole constructor state and every identifier',()=>{
 const b=recovery.inspectPreviewLoanBytes(raw,ledger);assert.equal(b.contractAddress,fixed.contractAddress);assert.equal(b.initialStateSha256,fixed.initialStateSha256);assert.deepEqual(b.identifiers,fixed.identifiers);assert.ok(b.oldDustNullifiers.length>0);
 const bad=Buffer.from(raw);bad[bad.length-1]^=1;assert.throws(()=>recovery.inspectPreviewLoanBytes(bad,ledger),/RECOVERY_NATIVE_HASH/);
 assert.throws(()=>recovery.assertPreviewRecoveryCapability({contractAddress:b.contractAddress,txId:b.txId}),/CAPABILITY/);
});
test('actual finalized deployment public gate accepts unchanged full native state',async()=>{
 const f=publicFixture(),r=await recovery.verifyPreviewLoanPublic(f.options);assert.equal(r.observation.receipt.transaction.dustFee,fixed.historicalDustFee);assert.equal(r.stateBlock.height,fixed.blockHeight);
});
for(const [name,change] of [
 ['genesis',f=>{const rpc=f.options.rpc;f.options.rpc=(m,a)=>m==='chain_getBlockHash'&&a[0]===0?'0x'+'00'.repeat(32):rpc(m,a);}],
 ['failed acceptance',f=>f.data.status='FailEntirely'],
 ['wrong transaction id',f=>f.data.txId='00'.repeat(33)],
 ['different history block',f=>f.data.blockHeight--],
 ['mutated current constructor state',f=>{f.options.provider.queryContractState=async(_a,c)=>{const s=initial();if(!c)s.balance=new Map([[{tag:'unshielded',raw:'22'.repeat(32)},1n]]);return s;};}],
 ['advanced authority',f=>{f.options.provider.queryContractState=async()=>{const s=initial();s.maintenanceAuthority=new stateRuntime.ContractMaintenanceAuthority(s.maintenanceAuthority.committee,1,1n);return s;};}],
 ['stale tip',f=>f.options.tip.timestampMs-=61000],
 ['forged binding',f=>f.options.binding={...f.options.binding}],
])test('Preview recovery rejects '+name+' before private restoration',async()=>{const f=publicFixture();change(f);await assert.rejects(recovery.verifyPreviewLoanPublic(f.options));});
test('empty failed constructor store remains untouched and nonempty unrelated store rejects',async()=>{
 const root=mkdtempSync(join(tmpdir(),'preview-empty-store-'));try{
  const source=join(root,'source');mkdirSync(source,{mode:0o700});
  const r=await inspectUnwrittenLoanStore({sourceDirectory:source,inspectionDirectory:join(root,'inspection'),accountId:'source-test'});assert.equal(r.status,'EMPTY');assert.deepEqual(readdirSync(source),[]);
  writeFileSync(join(source,'unrelated'),'private-canary',{mode:0o600});await assert.rejects(inspectUnwrittenLoanStore({sourceDirectory:source,inspectionDirectory:join(root,'second'),accountId:'source-test'}));assert.deepEqual(readdirSync(source),['unrelated']);
 }finally{rmSync(root,{recursive:true,force:true});}
});
// Controlled constructor runtime only. Native state comparisons remain actual;
// synthetic key verification isolates constructor/restoration/driver sequencing.
async function constructorMock(t,mutation){
 const signingKey='SYNTHETIC-NOT-A-KEY',coinPublicKey='SYNTHETIC-PUBLIC';
 let result;
 t.mock.module(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/midnight-js-protocol/dist/compact-js.mjs')).href,{namedExports:{ContractExecutable:{make:()=>({initialize:()=>{result={public:{contractState:initial()},private:{privateState:{},signingKey,zswapLocalState:{coinPublicKey,currentIndex:0n,inputs:[],outputs:[]}}};mutation?.(result);return result;}})}}});
 t.mock.module(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/midnight-js-types/dist/index.mjs')).href,{namedExports:{makeContractExecutableRuntime:()=>({runPromiseExit:async x=>x}),exitResultOrError:x=>x}});
 const testLedger={...ledger,signatureVerifyingKey:()=>initial().maintenanceAuthority.committee[0]};
 return {signingKey,coinPublicKey,ledger:testLedger,compiledContract:{sourceTestOnly:true},zkConfigProvider:{},args:[1,2,3,4,5,6]};
}
test('constructor mismatch cannot restore private state or mint a recovery capability',async t=>{
 const o=await constructorMock(t,r=>{r.public.contractState.balance=new Map([[{tag:'unshielded',raw:'11'.repeat(32)},1n]]);});
 await assert.rejects(recovery.reconstructPreviewLoan(o,recovery.inspectPreviewLoanBytes(raw,ledger)),/RECOVERY_STATE_MISMATCH/);
 let calls=0;await assert.rejects(recovery.restorePreviewLoanPrivate({set(){calls++;}},{privateState:{},signingKey:'forged'}));assert.equal(calls,0);
});
function walletState(){const progress=()=>({isConnected:true,isStrictlyComplete:()=>true});return {shielded:{progress:progress()},unshielded:{progress:progress(),availableCoins:[],pendingCoins:[]},dust:{progress:progress(),availableCoins:[{}],balance:()=>BigInt(fixed.remainingDustFee),state:{state:{findUtxoByNullifier:()=>undefined},pendingDust:[]}}};}
test('recovery wallet requires consumed deployment DUST and empty pending state',()=>{
 const binding=recovery.inspectPreviewLoanBytes(raw,ledger),input={binding,timestampMs:Date.now(),dustCap:BigInt(fixed.remainingDustFee)};
 assert.equal(recovery.assertPreviewRecoveryWallet({...input,synced:walletState()}),true);
 for(const mutate of [s=>s.dust.state.state.findUtxoByNullifier=()=>({}),s=>s.unshielded.pendingCoins=[{}],s=>s.dust.balance=()=>0n,s=>s.shielded.progress.isConnected=false]){const synced=walletState();mutate(synced);assert.throws(()=>recovery.assertPreviewRecoveryWallet({...input,synced}));}
});
async function composition(t){
 const p=plan(),events=[],ctor=await constructorMock(t),binding=recovery.inspectPreviewLoanBytes(raw,ledger),address=fixed.contractAddress,color='77'.repeat(32),first=p.roles.firstAddress;
 const observed={receipt:{txId:fixed.txId,circuitId:'deploy'},state:{}};
 const wallet={stop:async()=>events.push('wallet-stop'),waitForSyncedState:async()=>walletState()};
 const store={setContractAddress(a){assert.equal(a,address);events.push('restore-address');},async set(_id,v){this.state=v;events.push('restore-state');},async setSigningKey(_a,k){this.key=k;events.push('restore-key');},async get(){return this.state;},async getSigningKey(){return this.key;}};
 const options={kind:'loan',sourceTestOnly:true,deploymentRecoveryPlan:p,build:p.build,publicIdentity:p.wallet.publicIdentity,networkConfig:p.networkConfig,networkTag:p.networkTag,expectedProtocolVersion:p.expectedProtocolVersion,deploymentSigningKey:ctor.signingKey,roles:{firstAddress:first,secondAddress:p.roles.secondAddress,firstSecret:new Uint8Array(32),secondSecret:new Uint8Array(32).fill(1)},privateStateConfig:{midnightDbName:p.privateState.directory,privateStateStoreName:'sp05-loan',privateStoragePasswordProvider:()=> 'SYNTHETIC'},walletContext:{wallet,unshieldedKeystore:{getPublicKey:()=> 'SYNTHETIC-PUBLIC',getBech32Address:()=>p.wallet.expectedAddress},shieldedSecretKeys:{coinPublicKey:ctor.coinPublicKey,encryptionPublicKey:'SYNTHETIC'}},limits:{...p.limits,dustFee:BigInt(p.limits.dustFee),grossByLogicalAsset:{USD_TEST_ASSET:20000000000n},reservationStatePath:join(p.outputDirectory,'reservations.json')},now:()=>1700000000n,onEvent:()=>{},onStage:async s=>{events.push('record:'+s.stage);return {status:'RECORDED',stage:s.stage,txId:s.txId};}};
 const sdk={getNetworkId:()=> 'preview',NodeZkConfigProvider:class{},indexerPublicDataProvider:()=>({})};
 options.adapters={loadPublicIdentity:()=>({address:p.wallet.expectedAddress}),loadAssets:async()=>({compiledContract:ctor.compiledContract,decodeState:x=>x,zkConfigPath:'/synthetic/no-assets',assertFresh(){},cleanup:async()=>events.push('asset-cleanup')}),loadContractsSdk:async()=>({deployContract:()=>{throw Error('REDEPLOY_FORBIDDEN');},submitCallTx:async(_p,o)=>{events.push('call:'+o.circuitId);return {public:{txId:o.circuitId}};}}),loadProviderSdk:async()=>sdk,loadNativeRuntime:async()=>({ledger:{...ctor.ledger,addressFromKey:()=>first},runtime:{rawTokenType:()=>color}}),prepareDeployment:()=>{throw Error('PREPARE_DEPLOYMENT_FORBIDDEN');},initializeReservations:async o=>{events.push('allocate');assert.equal(o.limits.submissions,3);assert.equal(o.limits.dustFee,BigInt(fixed.remainingDustFee));},createProviders:async()=>({privateStateProvider:store,publicDataProvider:{},execute:async(_l,fn)=>fn(),getExecutionBinding:()=>({network:p.networkConfig,sdkNetworkId:'preview',payerAddress:first}),stop:()=>{},getState:()=>({reservedSubmissions:3,reservedDustFee:3n,reservedGrossByAsset:{},identifiers:[]}),cleanup:async()=>{events.push('provider-cleanup');return {walletStopped:true,pendingOperations:0,containmentComplete:false};}}),createComparator:async o=>{assert.deepEqual(o.historicalFeeAllocations,[{stages:['deploy'],dustFeeCap:BigInt(fixed.historicalDustFee)}]);assert.equal(o.dustFeeCap,BigInt(fixed.remainingDustFee));return {verifyStage:(stage,o)=>({status:'PASS',stage,txId:o.receipt.txId}),finish:()=>({status:'PASS'})};},observe:async o=>({receipt:{txId:o.txId,circuitId:o.circuitId},state:{}}),driver:runPreviewFinancialCase,fetch:async(_u,o)=>{const q=JSON.parse(o.body);return Response.json({jsonrpc:'2.0',id:q.id,result:'0x'+p.networkTag});}};
 options.recoveryAdapters={readInputs:()=>({binding}),verifyPublic:async()=>{events.push('public-current');return {binding,observation:observed,tip:{timestampMs:Date.now()}};},readTip:async()=>({}),checkWallet:recovery.assertPreviewRecoveryWallet,reconstruct:(o,b)=>recovery.reconstructPreviewLoan(o,b),inspectStore:async()=>{events.push('inspect-store');return {status:'EMPTY'};},checkDestination:()=>events.push('empty-destination'),restore:recovery.restorePreviewLoanPrivate};
 return {options,events};
}
test('callable Preview recovery restores verified constructor and executes only three calls after fresh deployment comparison',async t=>{
 const f=await composition(t),r=await integratePreviewFinancialCase(f.options);assert.equal(r.status,'SOURCE_TEST_ONLY');assert.equal(r.financialComparison.status,'PASS');
 assert.deepEqual(f.events.filter(x=>x.startsWith('call:')),['call:initialize','call:accrue','call:settle']);assert.equal(f.events.filter(x=>x==='public-current').length,2);
 assert.ok(f.events.indexOf('public-current')<f.events.indexOf('restore-state'));assert.ok(f.events.lastIndexOf('public-current')>f.events.indexOf('restore-key'));assert.ok(f.events.indexOf('record:deploy')<f.events.indexOf('call:initialize'));
 assert.deepEqual(r.driver.transactionIds,[...fixed.identifiers,'initialize','accrue','settle']);assert.deepEqual(r.driver.deploymentRecovery,{transactionHash:fixed.transactionHash,reservedSubmissions:2,reservedDustFee:fixed.conservativeReservedDustFee});
});
for(const gate of ['loadAssets','artifact-freshness','verifyPublic','reconstruct','inspectStore','restore','genesis','fee'])test('recovery '+gate+' failure cannot reach a financial call',async t=>{
 const f=await composition(t);
 if(gate==='loadAssets')f.options.adapters.loadAssets=()=>{throw Error('SOURCE_MANIFEST_CHANGED');};
 else if(gate==='artifact-freshness'){const load=f.options.adapters.loadAssets;f.options.adapters.loadAssets=async()=>({...await load(),assertFresh(){throw Error('ARTIFACT_CHANGED');}});}
 else if(gate==='genesis')f.options.adapters.fetch=async(_u,o)=>Response.json({jsonrpc:'2.0',id:JSON.parse(o.body).id,result:'0x'+'ff'.repeat(32)});
 else if(gate==='fee')f.options.limits.dustFee++;
 else f.options.recoveryAdapters[gate]=()=>{throw Error('CONTROLLED_GATE_FAILURE');};
 await assert.rejects(integratePreviewFinancialCase(f.options));assert.deepEqual(f.events.filter(x=>x.startsWith('call:')),[]);assert.ok(f.events.includes('wallet-stop')||f.events.includes('provider-cleanup'));
});

test('verified private restoration and the resulting driver capability are each single-use',async t=>{
 const o=await constructorMock(t),b=recovery.inspectPreviewLoanBytes(raw,ledger),result=await recovery.reconstructPreviewLoan(o,b);let state,key,writes=0;
 const provider={setContractAddress(){},async set(_id,v){state=v;writes++;},async setSigningKey(_a,k){key=k;writes++;},async get(){return state;},async getSigningKey(){return key;}};
 const capability=await recovery.restorePreviewLoanPrivate(provider,result);assert.equal(writes,2);assert.equal(recovery.assertPreviewRecoveryCapability(capability),capability);
 assert.throws(()=>recovery.assertPreviewRecoveryCapability(capability),/CAPABILITY/);await assert.rejects(recovery.restorePreviewLoanPrivate(provider,result),/UNVERIFIED/);assert.equal(writes,2);
});

// Public-result consumer regression. Later-stage data are retained LOCAL fixture
// projections rebound for schema testing only; this is not execution evidence.
import {validatePreviewFinancialCompletion,retainPreviewIntegrationResult} from './preview-launch.mjs';
import {previewFinancialExitCode} from './preview-bootstrap.mjs';
function completionFixture(){
 const r=JSON.parse(readFileSync(new URL('../../../deliverables/sp05-financial-integration-2026-09-09/local-continuation-02/run-public/integration-result.json',import.meta.url)));
 // Supply the Preview integration envelope used by its actual durable consumer.
 const output={schema:'moriarty.preview-financial-integration/1',status:'FINANCIAL_COMPLETE',kind:'loan',sourceTestOnly:false,networkAcceptance:false,proofAcceptance:false,financialAcceptance:false,build:plan().build,assetBindings:r.assetBindings,phase:'driver',driver:r.driver,cleanup:{walletStopped:true,pendingOperations:0,containmentComplete:false},setupPendingOperations:0,comparisons:r.comparisons,financialComparison:r.financialComparison,scope:'Fixed Preview composition; only independently reviewed actual evidence establishes financial acceptance'};
 delete output.build.receiptPath;output.driver.schema='moriarty.preview-financial-run/1';output.driver.status='FINANCIAL_COMPLETE';output.driver.cleanup=structuredClone(output.cleanup);output.driver.contractAddress=fixed.contractAddress;
 output.driver.deploymentRecovery={transactionHash:fixed.transactionHash,reservedSubmissions:2,reservedDustFee:fixed.conservativeReservedDustFee};output.driver.operationalState.reservedSubmissions=3;output.driver.operationalState.reservedDustFee='900000000000003';
 const deployment=output.driver.stages[0];Object.assign(deployment,{contractAddress:fixed.contractAddress,txId:fixed.txId,blockHash:fixed.blockHash,blockHeight:fixed.blockHeight,finalizedHeight:fixed.blockHeight,finalizedHead:'0x'+fixed.blockHash});
 Object.assign(deployment.transaction,{transactionHash:fixed.transactionHash,rawSha256:fixed.transactionHash,identifiers:[...fixed.identifiers],dustFee:fixed.historicalDustFee});deployment.indexerIdentifiers=[...fixed.identifiers];
 for(let i=0;i<4;i++){const receipt=output.driver.stages[i];receipt.contractAddress=fixed.contractAddress;Object.assign(output.comparisons[i],{contractAddress:fixed.contractAddress,txId:receipt.txId,blockHash:receipt.blockHash,blockHeight:receipt.blockHeight});}
 output.financialComparison.contractAddress=fixed.contractAddress;output.financialComparison.stages=structuredClone(output.comparisons);output.driver.transactionIds=output.driver.stages.flatMap(s=>s.transaction.identifiers);return output;
}
test('recovery full receipt result survives durable writer and CLI completion with all historical identifiers',()=>{
 const r=completionFixture(),directory=mkdtempSync(join(tmpdir(),'preview-recovery-completion-'));try{
  validatePreviewFinancialCompletion(r);retainPreviewIntegrationResult(directory,r);const stored=JSON.parse(readFileSync(join(directory,'integration-result.json')));assert.equal(previewFinancialExitCode(stored,Date.now()+1000),0);
  for(const mutate of [r=>r.driver.transactionIds.shift(),r=>delete r.driver.deploymentRecovery,r=>r.driver.deploymentRecovery.transactionHash='00'.repeat(32),r=>r.driver.deploymentRecovery.reservedDustFee='300000000000001',r=>r.driver.operationalState.reservedSubmissions=4,r=>r.driver.operationalState.reservedDustFee='1400000000000000',r=>r.driver.stages[0].transaction.dustFee='1',r=>r.driver.stages[0].transaction.rawSha256='00'.repeat(32)]){const bad=structuredClone(r);mutate(bad);assert.throws(()=>validatePreviewFinancialCompletion(bad));}
 }finally{rmSync(directory,{recursive:true,force:true});}
});

for(const value of [null,false,''])test('malformed explicit recovery '+JSON.stringify(value)+' cannot fall back to fresh deployment',async t=>{
  const f=await composition(t);f.options.deploymentRecoveryPlan=value;f.options.limits.submissions=4;
  let prepared=false;f.options.adapters.prepareDeployment=()=>{prepared=true;throw Error('REDEPLOY_FORBIDDEN');};
  await assert.rejects(integratePreviewFinancialCase(f.options));assert.equal(prepared,false);assert.deepEqual(f.events.filter(x=>x.startsWith('call:')),[]);
});
