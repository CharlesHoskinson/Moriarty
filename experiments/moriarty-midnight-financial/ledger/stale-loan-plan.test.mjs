import test from 'node:test';
import assert from 'node:assert/strict';
import {join} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {PINNED_NM} from './providers.mjs';
const api=await import('./stale-loan-plan.mjs').catch(()=>({}));
const D=fileURLToPath(new URL('../../../deliverables/sp05-financial-integration-2026-09-09/',import.meta.url));
const fixed={
  "schema": "moriarty.existing-stale-local-loan/1",
  "contractAddress": "ba4c808859fc2e4ee6d3d19fa0d812bb9a9c9eb0527161fb91315213bc24a713",
  "networkTag": "e72f7a21a0397844563b4206f887b779ffa0d937c2d1b2339441faa1f08b9846",
  "expectedProtocolVersion": 1000000,
  "buildReceiptSha256": "51ee2d4d60216464a9ace67966ba0ab253699844187aeb5652b46dc6e9ca5bf7",
  "privateStateDirectory": "/home/charl/.local/state/moriarty/sp05-local-loan-recovery-20260910-03/contract-state",
  "initializedStateFile": join(D,"local-recovery-03/indexed-initialize-state.bin"),
  "initializedStateSha256": "1f0724d2afe55e1c2aa22580f1bf9f0eed1a30e74b7a04ff78fe622a1e6fb30f",
  "settledStateSha256": "552d58ff2918332665b179b37a70907c4492a02b5c56d7db05dce5e427e573e0",
  "minimumCurrentBlockHeight": 20415,
  "deployTransactionFile": join(D,"local-execution-04/run-public/public-transactions/0af6f3ed8960b1a7d1d5e8c204d9e133255e784dd2c5fad1f160c5212d02bd3c.bin"),
  "deployTransactionHash": "0af6f3ed8960b1a7d1d5e8c204d9e133255e784dd2c5fad1f160c5212d02bd3c",
  "deployTxId": "00959c51e7d62ee9160bf1396ce0ab52f26757a7c5adec669cb083d5a8787d1de9",
  "deployIdentifiers": [
    "00b6140ece5793d57c801512e8e7c9e2ec2687e19b1c48a1f56167f2cd1dfc9e66",
    "00959c51e7d62ee9160bf1396ce0ab52f26757a7c5adec669cb083d5a8787d1de9"
  ],
  "initializeTransactionFile": join(D,"local-recovery-03/run-public/public-transactions/1f64634de2761fc0f140dbe7784a0cbf7005f226799e94d9878932378bc731e6.bin"),
  "initializeTransactionHash": "1f64634de2761fc0f140dbe7784a0cbf7005f226799e94d9878932378bc731e6",
  "initializeTxId": "006466368995501afc82b36cb38dac6f1cee531565eb75b70db6f3af5af36d9b46",
  "initializeIdentifiers": [
    "004b2aa75cdb10a080c3560a0af50ab17ef6e2e2294e1609a9e087f18498dc1d05",
    "006466368995501afc82b36cb38dac6f1cee531565eb75b70db6f3af5af36d9b46"
  ],
  "accrueTransactionFile": join(D,"local-continuation-02/run-public/public-transactions/473b363b745c4be4864e93a04d475a6f343ff7e56a8f179d86060716ba9a5480.bin"),
  "accrueTransactionHash": "473b363b745c4be4864e93a04d475a6f343ff7e56a8f179d86060716ba9a5480",
  "accrueTxId": "001ac8252beed46b047b6032710893182db342ce85e195403517738bf47e7248e7",
  "accrueIdentifiers": [
    "00f8303ec839a493912ae386b4019cefba92c36e3bfc5a6fea05658a9139f37435",
    "001ac8252beed46b047b6032710893182db342ce85e195403517738bf47e7248e7"
  ],
  "settleTransactionFile": join(D,"local-continuation-02/run-public/public-transactions/c201557431820f032743b0fc99c815b59131fa8bde6e298c466009189bce396e.bin"),
  "settleTransactionHash": "c201557431820f032743b0fc99c815b59131fa8bde6e298c466009189bce396e",
  "settleTxId": "00f40afa7fb4eb3e3daca7b5604ba783e7c1c214151766974848f40daedb474114",
  "settleIdentifiers": [
    "00f9b1159e9e600e04a8ec40a24f1e5e4f364b7ca34c72609a3ff28440ee153a6c",
    "00f40afa7fb4eb3e3daca7b5604ba783e7c1c214151766974848f40daedb474114"
  ],
  "deployResultFile": join(D,"local-execution-04/attempt-result.json"),
  "deployResultSha256": "bf1d456714ea134ad7e320849b3841088b2a2a352435d75ced65854c48031f54",
  "initializeResultFile": join(D,"local-recovery-03/attempt-result.json"),
  "initializeResultSha256": "858bea821333f092e42afacbf84386a0400ea6f1b350a76c6a23d66754e70aca",
  "continuationResultFile": join(D,"local-continuation-02/attempt-result.json"),
  "continuationResultSha256": "822232e8147b0c26e5012eef7a55b1cd926091f400dc52c2c059929ee8a5143f",
  "continuationIntegrationFile": join(D,"local-continuation-02/run-public/integration-result.json"),
  "continuationIntegrationSha256": "5a237ead68191acd494d2b93d6d5ed0b2e6a54c67ab44f2f9f59e2c83bb3ad80",
  "continuationReviewedResultFile": join(D,"local-continuation-02/reviewed-result.json"),
  "continuationReviewedResultSha256": "306e10064ee26742085366f628b14cd6d7f70264e6e8a7b037922362903fcf82",
  "settledProbeFile": join(D,"local-finalized-state-02/probe-result.json"),
  "settledProbeSha256": "8e651e0fc93055a7500aabf9fe874be1a93937ebe622a8451bbe963d973ec220"
};
function plan(){return {schema:'moriarty.local-financial-launch/1',kind:'loan',
 build:{receiptPath:'/home/charl/.local/state/moriarty/sp05-full-build-20260909-01/loan-output/build/build-receipt.json',receiptSha256:fixed.buildReceiptSha256,sourceManifestHash:'a29775a104dde9dbc38fdcbfdbc25a31db63beec84e09440f73441a27e9422e6'},
 networkConfig:{networkId:'undeployed',node:'http://127.0.0.1:19944',indexer:'http://127.0.0.1:18088/api/v4/graphql',indexerWS:'ws://127.0.0.1:18088/api/v4/graphql/ws',proofServer:'http://127.0.0.1:16300'},
 wallet:{seedFile:'/home/charl/.local/share/moriarty/test-wallets/local-undeployed.seed',stateDirectory:'/home/charl/.local/share/moriarty/test-wallets/hello-world-dedicated-v2',expectedAddress:'mn_addr_undeployed1n2w7v4y79630m5u40rpm6tn0qvnm83vptu9vqcwzqppam7pfrr9sa6q9r9'},
 roles:{firstAddress:'9a9de6549e2ea2fdd39578c3bd2e6f0327b3c5815f0ac061c20043ddf82918cb',secondAddress:'c1d1141a7f08931d16f3fe4cec1c57d66ab2d11d04e4ab7abb61121ecad5e61e',secretsFile:'/home/charl/.local/state/moriarty/sp05-local-loan-20260910-03/roles.json'},
 privateState:{directory:fixed.privateStateDirectory,passwordFile:'/home/charl/.local/state/moriarty/sp05-local-loan-recovery-20260910-03/password'},networkTag:fixed.networkTag,expectedProtocolVersion:1000000,
 limits:{allocationId:'sp05-stale-loan-source-test-01',submissions:1,deadlineMs:Date.now()+60000,dustFee:'1000000000000000',grossByLogicalAsset:{USD_TEST_ASSET:'0'}},outputDirectory:'/synthetic-stale-loan/run',
 existingStaleLoan:{...structuredClone(fixed),snapshotDirectory:'/synthetic-stale-loan/snapshot',inspectionDirectory:'/synthetic-stale-loan/inspection'}};}
function validate(p){assert.equal(typeof api.validateStaleLoanPlan,'function');return api.validateStaleLoanPlan(p.existingStaleLoan,p);}
test('fixed stale plan binds settled state and a single zero-financial-debit attempt without reading private paths',()=>{const p=plan(),before=structuredClone(p),d=validate(p);assert.deepEqual(d,p.existingStaleLoan);assert.notEqual(d,p.existingStaleLoan);assert.deepEqual(p,before);assert.equal(d.settledStateSha256,'552d58ff2918332665b179b37a70907c4492a02b5c56d7db05dce5e427e573e0');});
test('stale mode rejects every alternative mode and unknown field',()=>{for(const mutate of [p=>p.existingInitializedLoan={},p=>p.existingInitializedSwap={},p=>p.existingDeployment={},p=>p.staleSettledLoan={},p=>p.kind='swap',p=>p.adapters={},p=>p.existingStaleLoan.cursor='accrue',p=>p.existingStaleLoan.schema='moriarty.existing-initialized-local-loan/1']){const p=plan();mutate(p);assert.throws(()=>validate(p),/STALE_PLAN/);}});
test('all pinned public identity and history fields reject mutation',()=>{for(const key of Object.keys(fixed)){const p=plan();p.existingStaleLoan[key]=Array.isArray(fixed[key])?[]:typeof fixed[key]==='number'?0:'replacement';assert.throws(()=>validate(p),/STALE_PLAN/,key);}});
test('preserves wallet roles build and original private namespace identities',()=>{for(const group of ['wallet','roles','build','privateState'])for(const key of Object.keys(plan()[group])){const p=plan();p[group][key]='replacement';assert.throws(()=>validate(p),/STALE_PLAN/,group+'.'+key);}});
test('caps exactly one new submission and zero USD with bounded canonical DUST/deadline',()=>{for(const mutate of [p=>p.limits.submissions=0,p=>p.limits.submissions=2,p=>p.limits.dustFee='0',p=>p.limits.dustFee='1000000000000001',p=>p.limits.dustFee='01',p=>p.limits.dustFee=1n,p=>p.limits.grossByLogicalAsset.USD_TEST_ASSET='1',p=>p.limits.grossByLogicalAsset.USD_TEST_ASSET='00',p=>p.limits.grossByLogicalAsset.A='0',p=>p.limits.deadlineMs=Date.now()-1,p=>p.limits.deadlineMs=Date.now()+3600001,p=>p.limits.deadlineMs=Infinity,p=>p.limits.reservationStatePath='/old',p=>p.limits.allocationId='sp05-local-loan-continuation-02',p=>p.limits.allocationId='sp05-local-swap-continuation-01',p=>p.limits.allocationId='sp05-local-loan-recovery-03',p=>p.limits.allocationId='sp05-local-loan-03']){const p=plan();mutate(p);assert.throws(()=>validate(p),/STALE_PLAN/);}});
test('fresh destinations are canonical disjoint paths outside source stores and old campaigns',()=>{for(const dest of ['outputDirectory','snapshotDirectory','inspectionDirectory'])for(const value of ['/', 'relative','/tmp/../escape','/home/charl/.local/state/moriarty/sp05-local-loan-continuation-20260910-02/next','/home/charl/.local/state/moriarty/sp05-local-swap-20260910-01/next',plan().privateState.directory,plan().wallet.stateDirectory+'/new',plan().build.receiptPath+'/new',D+'/new']){const p=plan();if(dest==='outputDirectory')p[dest]=value;else p.existingStaleLoan[dest]=value;assert.throws(()=>validate(p),/STALE_PLAN/,dest+':'+value);}for(const mutate of [p=>p.existingStaleLoan.snapshotDirectory=p.outputDirectory,p=>p.existingStaleLoan.inspectionDirectory=p.outputDirectory+'/child',p=>p.outputDirectory='/synthetic-stale-loan']){const p=plan();mutate(p);assert.throws(()=>validate(p),/STALE_PLAN/);}});
test('endpoint network and pin overrides reject before any reader',()=>{for(const mutate of [p=>p.networkConfig.networkId='preview',p=>p.networkConfig.node='http://remote.invalid',p=>p.networkConfig.node='http://127.0.0.1:1',p=>p.networkConfig.node='http://127.0.0.1:19944/#fragment',p=>p.networkTag='11'.repeat(32),p=>p.expectedProtocolVersion=8]){const p=plan();mutate(p);assert.throws(()=>validate(p),/STALE_PLAN/);}});
test('nested getters symbols nonplain objects and array accessors never execute',()=>{let reads=0;for(const mutate of [p=>Object.defineProperty(p.limits,'dustFee',{enumerable:true,get(){reads++;return '1';}}),p=>Object.defineProperty(p.existingStaleLoan.deployIdentifiers,'0',{get(){reads++;return 'x';}}),p=>p.roles[Symbol('hidden')]=1,p=>Object.setPrototypeOf(p.wallet,{extra:true}),p=>p.existingStaleLoan.deployIdentifiers.extra='x']){const p=plan();mutate(p);assert.throws(()=>validate(p),/STALE_PLAN/);}assert.equal(reads,0);});
test('public reader rejects private redirects before invoking native decoder',()=>{assert.equal(typeof api.readStaleLoanInputs,'function');let reads=0;const ledger={get Transaction(){reads++;throw Error('NO_NATIVE');}};for(const key of ['deployTransactionFile','settledProbeFile','initializedStateFile']){const p=plan();p.existingStaleLoan[key]=p.wallet.seedFile;assert.throws(()=>api.readStaleLoanInputs(p.existingStaleLoan,ledger),/STALE_PLAN/);}assert.equal(reads,0);});
test('public metadata is immutable and reader verifies four native histories and settled source only',async()=>{assert.equal(typeof api.readStaleLoanInputs,'function');const ledger=await import(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs')).href);const p=plan(),r=api.readStaleLoanInputs(p.existingStaleLoan,ledger);assert.deepEqual(r.history.map(h=>h.stage),['deploy','initialize','accrue','settle']);assert.equal(r.settledSource.stateSha256,fixed.settledStateSha256);assert.equal(r.freshCurrentStateEstablished,false);assert.equal(r.executionAuthorized,false);assert.equal(r.history[3].transaction.inputs.length,1);assert.ok(Object.isFrozen(api.STALE_LOAN));assert.ok(Object.isFrozen(api.STALE_LOAN.deployIdentifiers));});
