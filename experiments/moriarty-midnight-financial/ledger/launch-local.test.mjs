import test from 'node:test';
import assert from 'node:assert/strict';
import {mkdtempSync,writeFileSync,readFileSync,readdirSync,chmodSync,symlinkSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {spawnSync} from 'node:child_process';
import {pathToFileURL} from 'node:url';
import {PINNED_NM} from './providers.mjs';
const api=await import('./launch-local.mjs').catch(()=>({}));
const h='11'.repeat(32);
function plan(){return {schema:'moriarty.local-financial-launch/1',kind:'loan',build:{receiptPath:'/private/build-receipt.json',receiptSha256:h,sourceManifestHash:h},networkConfig:{networkId:'undeployed',node:'http://127.0.0.1:19944',indexer:'http://127.0.0.1:18088/api/v4/graphql',indexerWS:'ws://127.0.0.1:18088/api/v4/graphql/ws',proofServer:'http://127.0.0.1:16300'},wallet:{seedFile:'/private/local.seed',stateDirectory:'/private/existing',expectedAddress:'mn_addr_undeployed1example'},roles:{firstAddress:h,secondAddress:'22'.repeat(32),secretsFile:'/private/roles.json'},privateState:{directory:'/private/db',passwordFile:'/private/password'},networkTag:h,expectedProtocolVersion:1000000,limits:{allocationId:'local-01',deadlineMs:Date.now()+60000,submissions:4,dustFee:'123',grossByLogicalAsset:{USD_TEST_ASSET:'1000'}},outputDirectory:'/private/run-01'};}
test('closed local plan validates without opening secret paths',()=>{assert.equal(typeof api.validateLocalLaunchPlan,'function');assert.equal(api.validateLocalLaunchPlan(plan()).kind,'loan');});
test('rejects remote endpoints, Preview, embedded secrets, unknown fields and unbounded limits',()=>{
 assert.equal(typeof api.validateLocalLaunchPlan,'function');
 for(const mutate of [p=>p.networkConfig.node='http://example.com',p=>p.networkConfig.networkId='preview',p=>p.wallet.seed='SECRET',p=>p.roles.firstSecret='SECRET',p=>p.adapters={},p=>p.limits.submissions=5,p=>p.limits.deadlineMs=Date.now()+7200000,p=>p.limits.dustFee='-1',p=>p.privateState.directory=p.outputDirectory,p=>p.roles.secondAddress=p.roles.firstAddress]){const p=plan();mutate(p);assert.throws(()=>api.validateLocalLaunchPlan(p));}
});
test('private input reader rejects permissive files and symlinks without leaking contents',()=>{
 assert.equal(typeof api.readPrivateLaunchFile,'function');const root=mkdtempSync(join(tmpdir(),'moriarty-launch-'));const file=join(root,'secret');
 try{writeFileSync(file,'DO_NOT_EMIT',{mode:0o600});assert.equal(api.readPrivateLaunchFile(file).toString(),'DO_NOT_EMIT');chmodSync(file,0o644);assert.throws(()=>api.readPrivateLaunchFile(file),e=>!e.message.includes('DO_NOT_EMIT'));chmodSync(file,0o600);symlinkSync(file,join(root,'link'));assert.throws(()=>api.readPrivateLaunchFile(join(root,'link')));}finally{rmSync(root,{recursive:true,force:true});}
});
test('event projection drops private fields and rejects unexpected event kinds',()=>{
 assert.equal(typeof api.publicLaunchEvent,'function');
 const event=api.publicLaunchEvent({kind:'submitted',txId:h,identifiers:[h],transactionHash:h,private:'DO_NOT_EMIT'});
 assert.deepEqual(event,{kind:'submitted',txId:h,identifiers:[h],transactionHash:h});
 assert.throws(()=>api.publicLaunchEvent({kind:'private',secret:'DO_NOT_EMIT'}));
});
test('actual pinned launch SDK files satisfy source inspection without wallet construction',async()=>{
 assert.equal(typeof api.inspectLocalLaunchRuntime,'function');const result=await api.inspectLocalLaunchRuntime();assert.equal(result.status,'SOURCE_INSPECTED');assert.equal(result.walletStarted,false);assert.ok(result.files>10);
});
test('CLI requires an explicit run, plan path and exact digest without echoing arguments',()=>{
 const child=spawnSync(process.execPath,['experiments/moriarty-midnight-financial/ledger/launch-local.mjs','--seed','DO_NOT_EMIT'],{encoding:'utf8'});
 assert.equal(child.status,1);assert.equal(child.stdout,'');assert.ok(!child.stderr.includes('DO_NOT_EMIT'));
});
test('strict saved-state envelope rejects absent, null and incompatible restore data',()=>{
 assert.equal(typeof api.decodeSavedWalletEnvelope,'function');
 assert.equal(api.decodeSavedWalletEnvelope({version:1,state:'opaque-saved-state'}),'opaque-saved-state');
 for(const x of [{version:1},{version:1,state:null},{version:2,state:'x'},{version:1,state:'x',fallback:true}])assert.throws(()=>api.decodeSavedWalletEnvelope(x));
});
test('real pinned API exports compose without constructing a wallet or key',async()=>{
 assert.equal(typeof api.inspectLocalLaunchApi,'function');assert.deepEqual(await api.inspectLocalLaunchApi(),{status:'API_INSPECTED',walletStarted:false,keysCreated:false,networkRequests:0});
});
test('public submission capture persists actual historical native bytes before inert submission and retains failure evidence',async()=>{
 assert.equal(typeof api.retainPublicSubmissions,'function');
 const ledger=await import(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs')).href);
 const raw=readFileSync(new URL('./fixtures/historical-dust/transaction.bin',import.meta.url));
 const tx=ledger.Transaction.deserialize('signature','proof','binding',raw);
 const dir=mkdtempSync(join(tmpdir(),'moriarty-public-submission-'));let calls=0;
 const wallet={marker:1,async submitTransaction(actual){calls++;assert.equal(this,wallet);assert.deepEqual(Buffer.from(actual.serialize()),raw);const metadata=JSON.parse(readFileSync(join(dir,tx.transactionHash()+'.json')));assert.equal(metadata.transactionHash,tx.transactionHash());assert.deepEqual(metadata.identifiers,[...tx.identifiers()]);assert.deepEqual(readFileSync(join(dir,metadata.file)),raw);throw Error('INERT_SUBMIT_FAILURE');},stop(){assert.equal(this,wallet);return this.marker;}};
 try{const retained=api.retainPublicSubmissions({wallet,ledger,directory:dir});assert.equal(retained.stop(),1);await assert.rejects(retained.submitTransaction(tx),/INERT_SUBMIT_FAILURE/);assert.equal(calls,1);assert.equal(readdirSync(dir).length,2);
  await assert.rejects(retained.submitTransaction(tx));assert.equal(calls,1,'exclusive existing capture blocks repeat submission');
 }finally{rmSync(dir,{recursive:true,force:true});}
});
test('submission evidence storage failure prevents calling even an inert submitter',async()=>{
 assert.equal(typeof api.retainPublicSubmissions,'function');const ledger=await import(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs')).href);
 const tx=ledger.Transaction.deserialize('signature','proof','binding',readFileSync(new URL('./fixtures/historical-dust/transaction.bin',import.meta.url)));const dir=mkdtempSync(join(tmpdir(),'moriarty-capture-failure-'));let calls=0;
 try{writeFileSync(join(dir,tx.transactionHash()+'.json'),'occupied',{mode:0o600});const wallet={async submitTransaction(){calls++;}};const retained=api.retainPublicSubmissions({wallet,ledger,directory:dir});await assert.rejects(retained.submitTransaction(tx));assert.equal(calls,0);assert.ok(readdirSync(dir).some(x=>x.endsWith('.bin')),'public bytes retained despite metadata storage failure');}finally{rmSync(dir,{recursive:true,force:true});}
});
test('complete reviewed public integration result preserves failed driver receipts, cleanup and finished comparison',()=>{
 assert.equal(typeof api.retainPublicIntegrationResult,'function');const dir=mkdtempSync(join(tmpdir(),'moriarty-integration-result-'));
 const result={schema:'moriarty.local-financial-integration/1',status:'INCOMPLETE',kind:'loan',sourceTestOnly:false,networkAcceptance:false,proofAcceptance:false,financialAcceptance:false,build:{receiptSha256:h,sourceManifestHash:h},assetBindings:{USD_TEST_ASSET:h},phase:'driver',driver:{schema:'moriarty.local-financial-run/1',status:'INCOMPLETE',kind:'loan',contractAddress:h,stages:[{schema:'moriarty.finalized-financial-stage/1',txId:h}],transactionIds:[h],cleanup:{walletStopped:true,pendingOperations:0,containmentComplete:false},operationalState:{reservedSubmissions:4},scope:'public'},cleanup:{walletStopped:true,pendingOperations:0,containmentComplete:false},setupPendingOperations:0,comparisons:[{status:'PASS',stage:'settle'}],financialComparison:{status:'PASS',kind:'loan',contractAddress:h,expectationsSha256:h,stages:[{status:'PASS',stage:'settle'}],scope:'public',networkAcceptance:false,proofAcceptance:false},scope:'public'};
 try{api.retainPublicIntegrationResult(dir,result);assert.deepEqual(JSON.parse(readFileSync(join(dir,'integration-result.json'))),result);assert.throws(()=>api.retainPublicIntegrationResult(dir,{...result,private:'DO_NOT_EMIT'}));assert.throws(()=>api.retainPublicIntegrationResult(dir,new Error('DO_NOT_EMIT')));
  const successDir=mkdtempSync(join(dir,'success-'));const success=structuredClone(result);success.status='PASS';success.driver.status='PASS';api.retainPublicIntegrationResult(successDir,success);assert.deepEqual(JSON.parse(readFileSync(join(successDir,'integration-result.json'))),success);
 }finally{rmSync(dir,{recursive:true,force:true});}
});
