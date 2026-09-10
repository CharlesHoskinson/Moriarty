import test from 'node:test';
import assert from 'node:assert/strict';
import {integrateLocalFinancialCase, createLocalRpc} from './integrate-local.mjs';
import {runLocalFinancialCase} from './run-local.mjs';
const first='11'.repeat(32), second='22'.repeat(32), address='33'.repeat(32), color='44'.repeat(32);
function fixture() {
  const calls=[], summaries=[];
  const network={networkId:'undeployed',node:'http://127.0.0.1:9944',indexer:'http://127.0.0.1:8088/graphql',indexerWS:'ws://127.0.0.1:8088/ws',proofServer:'http://127.0.0.1:6300'};
  const options={kind:'loan',walletContext:{wallet:{async stop(){calls.push('wallet-stop');}},unshieldedKeystore:{getPublicKey:()=> 'inert-key'},shieldedSecretKeys:{coinPublicKey:'coin',encryptionPublicKey:'enc'}},networkConfig:network,
    roles:{firstSecret:new Uint8Array(32).fill(1),secondSecret:new Uint8Array(32).fill(2),firstAddress:first,secondAddress:second},networkTag:'55'.repeat(32),deploymentSigningKey:'PRIVATE-DEPLOYMENT-CANARY',
    build:{receiptPath:'/inert/build-receipt.json',receiptSha256:'66'.repeat(32),sourceManifestHash:'77'.repeat(32)},privateStateConfig:{},limits:{allocationId:'inert-allocation',reservationStatePath:'/inert/allocation.json',deadlineMs:Date.now()+10000,submissions:4,dustFee:100n,grossByLogicalAsset:{USD_TEST_ASSET:20n}},expectedProtocolVersion:1000000,now:()=>1700000000n,
    onEvent:event=>calls.push('event:'+event.kind),onStage:async summary=>{summaries.push(summary);calls.push('record:'+summary.stage);return {status:'RECORDED',stage:summary.stage,txId:summary.txId};},sourceTestOnly:true};
  const driverSdk={deployContract:async(_p,o)=>{calls.push('deploy');assert.equal(o.args.length,6);return {deployTxData:{public:{contractAddress:address,txId:'inert-deploy'}}};},submitCallTx:async(_p,o)=>{calls.push(o.circuitId);return {public:{txId:'inert-'+o.circuitId}};}};
  const sdk={getNetworkId:()=> 'undeployed',NodeZkConfigProvider:class {async getVerifierKey(){calls.push('verifier-key');return {};}}};
  let limits, cleaned=false;
  const adapters={
    loadAssets:async()=>{calls.push('load-assets');return {compiledContract:{inert:true},decodeState:x=>x,zkConfigPath:'/inert/assets',assertFresh(){assert.equal(cleaned,false);calls.push('fresh');},cleanup(){cleaned=true;calls.push('asset-cleanup');}};},
    loadContractsSdk:async()=>({inert:true}),loadProviderSdk:async()=>sdk,
    loadNativeRuntime:async()=>({ledger:{addressFromKey:()=>first},runtime:{rawTokenType:(_domain,a)=>{assert.equal(a,address);calls.push('derive-color');return color;}}}),
    prepareDeployment:async o=>{calls.push('prepare');await o.zkConfigProvider.getVerifierKey('initialize');return {public:{contractAddress:address},driverSdk};},
    initializeReservations:o=>{calls.push('initialize-allocation');limits=o.limits;assert.equal(limits.grossByAsset[color],20n);assert.equal('grossByLogicalAsset' in limits,false);},
    createProviders:async()=>{calls.push('providers');return {publicDataProvider:{},async execute(_label,fn){return fn();},stop(){calls.push('provider-stop');},getState:()=>({reservedSubmissions:4,reservedDustFee:10n,reservedGrossByAsset:{[color]:10n},identifiers:[]}),getExecutionBinding:()=>({network,payerAddress:first,sdkNetworkId:'undeployed'}),cleanup:async()=>{calls.push('provider-cleanup');return {walletStopped:true,pendingOperations:0,containmentComplete:false};}};},
    observe:async o=>{calls.push('observe:'+o.circuitId);return {receipt:{circuitId:o.circuitId,txId:o.txId},state:{}};},
    createComparator:async()=>({verifyStage(stage,observation){calls.push('compare:'+stage);return {status:'PASS',stage,txId:observation.receipt.txId,publicState:{observed:true},networkAcceptance:false,proofAcceptance:false};},finish:()=>({status:'PASS',networkAcceptance:false,proofAcceptance:false})}),
    driver:runLocalFinancialCase,fetch:()=>{throw Error('network forbidden in source fixture');},
  };
  options.adapters=adapters;return {options,adapters,calls,summaries,getLimits:()=>limits,sdk,driverSdk};
}

import {STALE_LOAN as fixed} from './stale-loan-plan.mjs';
import {validateLocalLaunchPlan,retainPublicAdverseCandidate,retainPublicAdverseOutcome,retainPublicIntegrationResult} from './launch-local.mjs';
function plan(){return {schema:'moriarty.local-financial-launch/1',kind:'loan',
 build:{receiptPath:'/home/charl/.local/state/moriarty/sp05-full-build-20260909-01/loan-output/build/build-receipt.json',receiptSha256:fixed.buildReceiptSha256,sourceManifestHash:'a29775a104dde9dbc38fdcbfdbc25a31db63beec84e09440f73441a27e9422e6'},
 networkConfig:{networkId:'undeployed',node:'http://127.0.0.1:19944',indexer:'http://127.0.0.1:18088/api/v4/graphql',indexerWS:'ws://127.0.0.1:18088/api/v4/graphql/ws',proofServer:'http://127.0.0.1:16300'},
 wallet:{seedFile:'/home/charl/.local/share/moriarty/test-wallets/local-undeployed.seed',stateDirectory:'/home/charl/.local/share/moriarty/test-wallets/hello-world-dedicated-v2',expectedAddress:'mn_addr_undeployed1n2w7v4y79630m5u40rpm6tn0qvnm83vptu9vqcwzqppam7pfrr9sa6q9r9'},
 roles:{firstAddress:'9a9de6549e2ea2fdd39578c3bd2e6f0327b3c5815f0ac061c20043ddf82918cb',secondAddress:'c1d1141a7f08931d16f3fe4cec1c57d66ab2d11d04e4ab7abb61121ecad5e61e',secretsFile:'/home/charl/.local/state/moriarty/sp05-local-loan-20260910-03/roles.json'},
 privateState:{directory:fixed.privateStateDirectory,passwordFile:'/home/charl/.local/state/moriarty/sp05-local-loan-recovery-20260910-03/password'},networkTag:fixed.networkTag,expectedProtocolVersion:1000000,
 limits:{allocationId:'sp05-stale-loan-source-test-01',submissions:1,deadlineMs:Date.now()+60000,dustFee:'1000000000000000',grossByLogicalAsset:{USD_TEST_ASSET:'0'}},outputDirectory:'/synthetic-stale-loan/run',
 existingStaleLoan:{...structuredClone(fixed),snapshotDirectory:'/synthetic-stale-loan/snapshot',inspectionDirectory:'/synthetic-stale-loan/inspection'}};}

function staleFixture(){
 const f=fixture(),p=plan(),o=f.options;o.staleLoanPlan=p;o.build=p.build;o.networkConfig=p.networkConfig;o.networkTag=p.networkTag;o.roles={...o.roles,firstAddress:p.roles.firstAddress,secondAddress:p.roles.secondAddress};
 o.privateStateConfig={midnightDbName:p.privateState.directory,privateStateStoreName:'sp05-loan',privateStoragePasswordProvider:()=> 'PRIVATE-PASSWORD-CANARY'};
 o.limits={...p.limits,dustFee:BigInt(p.limits.dustFee),grossByLogicalAsset:{USD_TEST_ASSET:0n},reservationStatePath:p.outputDirectory+'/reservations.json'};
 o.walletContext.unshieldedKeystore.getBech32Address=()=>({toString:()=>p.wallet.expectedAddress});o.walletContext.wallet.waitForSyncedState=async()=>{f.calls.push('wallet-sync');return {syncedFixture:true};};
 f.adapters.loadNativeRuntime=async()=>({ledger:{addressFromKey:()=>p.roles.firstAddress},runtime:{rawTokenType:()=>color}});
 f.adapters.initializeReservations=x=>{f.calls.push('allocation');assert.equal(x.limits.submissions,1);assert.equal(x.limits.grossByAsset[color],0n);};
 f.adapters.prepareDeployment=()=>{throw Error('OLD_DEPLOY_FORBIDDEN');};f.adapters.driver=()=>{throw Error('OLD_DRIVER_FORBIDDEN');};
 o.retainAdverseCandidate=async()=>{};o.retainAdverseOutcome=async()=>{};
 o.staleLoanAdapters={close:()=>{f.calls.push('stale-close');},publicCheck:async()=>{f.calls.push('stale-public');return {status:'SETTLED_LOAN_PUBLIC_VERIFIED',history:['deploy','initialize','accrue','settle'].map(stage=>({stage,observation:{receipt:{txId:stage}}})),binding:{},tip:{timestampMs:Date.now()}};},checkWallet:()=>{f.calls.push('wallet-check');},preserveStore:async x=>{f.calls.push('preserve');assert.equal(x.sourceDirectory,p.privateState.directory);return {status:'PRESERVED'};},prepare:async x=>{f.calls.push('stale-prepare');assert.equal(x.providers.getState().reservedSubmissions,4);return {opaque:true};},submit:async(h,callbacks)=>{f.calls.push('stale-submit');assert.equal(h.opaque,true);assert.equal(callbacks.retainCandidate,o.retainAdverseCandidate);return {status:'NODE_REJECTION_FINANCIAL_NONMUTATION',financialNonmutationEstablished:true};}};
 return {...f,p};
}
test('actual launch plan selects single stale mode and rejects old combined modes',()=>{const p=plan();assert.deepEqual(validateLocalLaunchPlan(p),p);p.existingInitializedLoan={};assert.throws(()=>validateLocalLaunchPlan(p),/MUTUALLY_EXCLUSIVE/);});
test('actual integration dispatch orders history, wallet, preservation, provider and only stale submission',async()=>{
 const f=staleFixture(),r=await integrateLocalFinancialCase(f.options);assert.equal(r.sourceTestOnly,true);assert.equal(r.status,'SOURCE_TEST_ONLY');assert.equal(r.adverse.status,'NODE_REJECTION_FINANCIAL_NONMUTATION');assert.equal(r.comparisons.length,4);
 for(const [a,b] of [['stale-public','compare:deploy'],['compare:settle','wallet-check'],['wallet-check','preserve'],['preserve','allocation'],['allocation','providers'],['providers','stale-prepare'],['stale-prepare','stale-submit'],['stale-submit','provider-cleanup']])assert.ok(f.calls.indexOf(a)>=0&&f.calls.indexOf(a)<f.calls.indexOf(b),a+' before '+b);
 assert.equal(f.calls.filter(x=>x==='stale-submit').length,1);assert.equal(r.networkAcceptance,false);
});
for(const gate of ['publicCheck','checkWallet','preserveStore','prepare','submit'])test(gate+' stops later actions and always cleans existing wallet/providers',async()=>{const f=staleFixture();f.options.staleLoanAdapters[gate]=()=>{throw Error('SYNTHETIC_GATE');};await assert.rejects(integrateLocalFinancialCase(f.options));assert.ok(f.calls.includes('wallet-stop')||f.calls.includes('provider-cleanup'));if(!['submit'].includes(gate))assert.equal(f.calls.includes('stale-submit'),false);});
test('production options refuse source adapters and a second submission',async()=>{const f=staleFixture();f.options.sourceTestOnly=false;await assert.rejects(integrateLocalFinancialCase(f.options),/ADAPTERS_REQUIRE_SOURCE_TEST/);const g=staleFixture();g.options.limits.submissions=2;await assert.rejects(integrateLocalFinancialCase(g.options),/STALE_INTEGRATION_LIMITS/);assert.equal(g.calls.includes('providers'),false);});
import {mkdtempSync,readFileSync,rmSync,chmodSync} from 'node:fs';import {tmpdir} from 'node:os';import {join} from 'node:path';import {createHash} from 'node:crypto';
test('durable public candidate/outcome/result writer preserves binary, bigint state and rejects reuse/accessors',async()=>{
 const dir=mkdtempSync(join(tmpdir(),'moriarty-adverse-writer-'));chmodSync(dir,0o700);
 try{
  const raw=readFileSync(new URL('../../../deliverables/sp05-financial-integration-2026-09-09/local-continuation-02/run-public/public-transactions/473b363b745c4be4864e93a04d475a6f343ff7e56a8f179d86060716ba9a5480.bin',import.meta.url)),digest=createHash('sha256').update(raw).digest('hex');
  const {PINNED_NM}=await import('./providers.mjs'),{pathToFileURL}=await import('node:url'),ledger=await import(pathToFileURL(PINNED_NM+'/@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs').href),{decodeNativeFinancialTransaction}=await import('./receipt.mjs');const transaction=decodeNativeFinancialTransaction(raw,ledger);const badTransaction=structuredClone(transaction);badTransaction.actions[0].private={secret:'PRIVATE-CANARY'};await assert.rejects(retainPublicAdverseCandidate(dir,{raw,transaction:badTransaction}),/LAUNCH_ADVERSE_CANDIDATE_BINDING/);
  assert.deepEqual(await retainPublicAdverseCandidate(dir,{raw,transaction}),{status:'RECORDED',rawSha256:digest});assert.deepEqual(readFileSync(join(dir,'adverse-candidate.bin')),Buffer.from(raw));
  await assert.rejects(retainPublicAdverseCandidate(dir,{raw,transaction}),/EEXIST/);
  const outcome={candidateRawSha256:digest,status:'NODE_REJECTED',phase:'submit',code:'NODE_TRANSACTION_INVALID',nodeRejectionEstablished:true,stalePredicateEstablished:false};assert.equal(retainPublicAdverseOutcome(dir,outcome).code,outcome.code);
  let invoked=false;const bad={...outcome};Object.defineProperty(bad,'code',{get(){invoked=true;return 'NODE_TRANSACTION_INVALID';},enumerable:true});assert.throws(()=>retainPublicAdverseOutcome(dir,bad),/LAUNCH_ADVERSE_OUTCOME/);assert.equal(invoked,false);
  const f=staleFixture(),result=await integrateLocalFinancialCase(f.options);result.sourceTestOnly=false;result.status='INCOMPLETE';result.financialComparison=undefined;const before=JSON.parse(readFileSync(new URL('../../../deliverables/sp05-financial-integration-2026-09-09/local-finalized-state-02/probe-result.json',import.meta.url))).snapshots[0];before.state.revision=2n;result.adverse={status:'OUTCOME_UNKNOWN',financialNonmutationEstablished:false,before};
  const privateFailure=structuredClone(result);privateFailure.adverse.failure={status:'OUTCOME_UNKNOWN',phase:'submit',code:'UNCLASSIFIED_SUBMIT_FAILURE',cause:{secret:'PRIVATE-CANARY'}};assert.throws(()=>retainPublicIntegrationResult(dir,privateFailure),/LAUNCH_ADVERSE_DIAGNOSTIC/);
  const badFee=structuredClone(result);badFee.adverse.feeAccounting={candidateDustFeeSpeck:'7',paidFeeSpeck:{secret:'PRIVATE-CANARY'},paidStatus:'UNKNOWN',reservationReleasePerformed:false};assert.throws(()=>retainPublicIntegrationResult(dir,badFee),/LAUNCH_ADVERSE_FEES/);
  retainPublicIntegrationResult(dir,result);const saved=JSON.parse(readFileSync(join(dir,'integration-result.json')));assert.equal(saved.adverse.before.state.revision,'2');assert.deepEqual(saved.adverse.before.balances,before.balances);assert.equal(saved.financialAcceptance,false);
 }finally{rmSync(dir,{recursive:true,force:true});}
});
for(const mode of ['preflight-fails','existing-output'])test('actual launcher '+mode+' stops before every private read',async t=>{
 const p=plan(),fs=await import('node:fs'),integration=await import('./integrate-local.mjs'),assets=await import('./proven-assets.mjs');let privateReads=0,preflights=0,mkdirs=0;
 const {syncBuiltinESMExports}=await import('node:module');const read=fs.readFileSync;
 t.mock.method(fs.default,'lstatSync',()=>({isSymbolicLink:()=>false,isDirectory:()=>true,mode:0o40700,uid:process.getuid()}));
 t.mock.method(fs.default,'readFileSync',(path,...args)=>{const value=String(path);if([p.roles.secretsFile,p.privateState.passwordFile,p.wallet.seedFile].includes(value)||value.startsWith(p.wallet.stateDirectory+'/.midnight-wallet-state')){privateReads++;throw Error('PRIVATE_READ_FORBIDDEN');}return read(path,...args);});
 t.mock.method(fs.default,'mkdirSync',path=>{mkdirs++;assert.equal(path,p.outputDirectory);throw Object.assign(Error('EXCLUSIVE_OUTPUT_EXISTS'),{code:'EEXIST'});});syncBuiltinESMExports();t.after(()=>{t.mock.restoreAll();syncBuiltinESMExports();});
 t.mock.module(new URL('./proven-assets.mjs',import.meta.url).href,{namedExports:{...assets,inspectFinancialBuild:async()=>({publicOnly:true})}});
 t.mock.module(new URL('./integrate-local.mjs',import.meta.url).href,{namedExports:{...integration,preflightLocalStaleLoan:async q=>{preflights++;assert.deepEqual(q,p);if(mode==='preflight-fails')throw Error('PUBLIC_PREFLIGHT_REJECTED');return {status:'SETTLED_LOAN_PUBLIC_VERIFIED'};}}});
 const launch=await import('./launch-local.mjs?before-private-'+mode);
 await assert.rejects(launch.launchLocalFinancialCase(p),e=>e.message===(mode==='preflight-fails'?'PUBLIC_PREFLIGHT_REJECTED':'EXCLUSIVE_OUTPUT_EXISTS'));assert.equal(preflights,1);assert.equal(privateReads,0);assert.equal(mkdirs,mode==='existing-output'?1:0);
});
