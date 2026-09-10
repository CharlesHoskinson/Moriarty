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
test('private input reader and CLI reject a FIFO without waiting for a writer',()=>{
 const dir=mkdtempSync(join(tmpdir(),'moriarty-launch-fifo-'));const fifo=join(dir,'not-a-regular-file');
 try{
  assert.equal(spawnSync('mkfifo',['-m','600',fifo]).status,0);
  const moduleUrl=new URL('./launch-local.mjs',import.meta.url).href;
  const script=`import {readPrivateLaunchFile} from ${JSON.stringify(moduleUrl)};try{readPrivateLaunchFile(process.argv[1]);process.exitCode=2;}catch(e){process.exitCode=e.message==='LAUNCH_PRIVATE_FILE'?0:3;}`;
  const reader=spawnSync(process.execPath,['--input-type=module','-e',script,fifo],{encoding:'utf8',timeout:2000,killSignal:'SIGKILL'});
  assert.equal(reader.error,undefined,'reader must reject without blocking on FIFO open');assert.equal(reader.status,0);
  const cli=spawnSync(process.execPath,[new URL('./launch-local.mjs',import.meta.url).pathname,'--run','--plan',fifo,'--sha256',h],{encoding:'utf8',timeout:2000,killSignal:'SIGKILL'});
  assert.equal(cli.error,undefined,'CLI must reject before its plan-dependent timer exists');assert.equal(cli.status,1);assert.equal(cli.stdout,'');
  assert.equal(cli.stderr,'Local financial launch failed; inspect retained public run records.\n');
 }finally{rmSync(dir,{recursive:true,force:true});}
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

test('storage password preflight applies actual SDK policy before wallet work and emits only a closed error',async()=>{
 assert.equal(typeof api.readLocalStoragePassword,'function');
 const dir=mkdtempSync(join(tmpdir(),'moriarty-password-preflight-')),file=join(dir,'password');
 try{
  for(const password of ['1a9f'.repeat(16),'Ab1!'.repeat(4)+'abcd','Ab1!'.repeat(4)+'zzzz','too-short']){
   writeFileSync(file,password+'\n',{mode:0o600});
   await assert.rejects(api.readLocalStoragePassword(file),e=>e.message==='LAUNCH_PRIVATE_PASSWORD'&&!e.message.includes(password));
  }
  const accepted='N7!qL2@vR9#sH4$wT6%y';writeFileSync(file,accepted+'\n',{mode:0o600});
  assert.equal(await api.readLocalStoragePassword(file),accepted);
 }finally{rmSync(dir,{recursive:true,force:true});}
});

function recoveryLaunchPlan(){
 const p=plan(),old='/home/charl/.local/state/moriarty/sp05-local-loan-20260910-03';
 p.build.receiptSha256='51ee2d4d60216464a9ace67966ba0ab253699844187aeb5652b46dc6e9ca5bf7';p.networkTag='e72f7a21a0397844563b4206f887b779ffa0d937c2d1b2339441faa1f08b9846';p.limits.submissions=3;
 p.wallet={seedFile:'/home/charl/.local/share/moriarty/test-wallets/local-undeployed.seed',stateDirectory:'/home/charl/.local/share/moriarty/test-wallets/hello-world-dedicated-v2',expectedAddress:'mn_addr_undeployed1n2w7v4y79630m5u40rpm6tn0qvnm83vptu9vqcwzqppam7pfrr9sa6q9r9'};
 p.roles={firstAddress:'9a9de6549e2ea2fdd39578c3bd2e6f0327b3c5815f0ac061c20043ddf82918cb',secondAddress:'c1d1141a7f08931d16f3fe4cec1c57d66ab2d11d04e4ab7abb61121ecad5e61e',secretsFile:old+'/roles.json'};
 p.existingDeployment={schema:'moriarty.existing-local-loan/1',transactionFile:'/public/native.bin',transactionHash:'0af6f3ed8960b1a7d1d5e8c204d9e133255e784dd2c5fad1f160c5212d02bd3c',identifiers:['00b6140ece5793d57c801512e8e7c9e2ec2687e19b1c48a1f56167f2cd1dfc9e66','00959c51e7d62ee9160bf1396ce0ab52f26757a7c5adec669cb083d5a8787d1de9'],txId:'00959c51e7d62ee9160bf1396ce0ab52f26757a7c5adec669cb083d5a8787d1de9',contractAddress:'ba4c808859fc2e4ee6d3d19fa0d812bb9a9c9eb0527161fb91315213bc24a713',buildReceiptSha256:p.build.receiptSha256,networkTag:p.networkTag,expectedProtocolVersion:p.expectedProtocolVersion,sourceAllocationId:'sp05-local-loan-03',sourceResultFile:'/public/result.json',sourceResultSha256:'bf1d456714ea134ad7e320849b3841088b2a2a352435d75ced65854c48031f54',sourcePrivateStateDirectory:old+'/contract-state',inspectionDirectory:'/private/inspection',destinationDirectory:p.privateState.directory};
 return p;
}
test('closed recovery launch keeps original wallet and roles and permits exactly three submissions',()=>{
 const p=recoveryLaunchPlan(),copy=api.validateLocalLaunchPlan(p);assert.deepEqual(copy,p);assert.notEqual(copy,p);
 for(const mutate of [p=>p.kind='swap',p=>p.limits.submissions=4,p=>p.wallet.seedFile='/other/seed',p=>p.wallet.stateDirectory='/other/snapshots',p=>p.wallet.expectedAddress='mn_addr_undeployed1other',p=>p.roles.firstAddress=h,p=>p.roles.secondAddress=h,p=>p.existingDeployment.inspectionDirectory=p.wallet.stateDirectory+'/copy',p=>p.existingDeployment.inspectionDirectory=p.outputDirectory,p=>p.privateState.passwordFile=p.wallet.stateDirectory+'/password',p=>p.existingDeployment.privateState={},p=>p.existingDeployment.txId=h]){const x=recoveryLaunchPlan();mutate(x);assert.throws(()=>api.validateLocalLaunchPlan(x));}
 const ordinary=plan();ordinary.limits.submissions=3;assert.throws(()=>api.validateLocalLaunchPlan(ordinary));
});

test('launcher rejects malformed diagnostic fields and accessors before durable retention',()=>{
 const directory=mkdtempSync(join(tmpdir(),'moriarty-diagnostic-validation-'));let getterCalls=0;
 const valid=()=>({phase:'observe',stage:'initialize',code:'NOT_FINALIZED'});
 const base=()=>({schema:'moriarty.local-financial-integration/1',status:'FAILED',kind:'loan',sourceTestOnly:false,networkAcceptance:false,proofAcceptance:false,financialAcceptance:false,build:undefined,assetBindings:undefined,phase:'driver',driver:{schema:'moriarty.local-financial-run/1',status:'FAILED',kind:'loan',contractAddress:h,stages:[],transactionIds:[h],cleanup:{walletStopped:true},operationalState:{},scope:'fixture',failure:valid()},cleanup:{walletStopped:true},setupPendingOperations:0,comparisons:[],financialComparison:undefined,scope:'fixture'});
 const changes=[
  d=>d.failure.code='PRIVATE_VALUE',d=>d.failure.phase='PRIVATE_VALUE',d=>d.failure.stage='PRIVATE_VALUE',
  d=>d.failure.private='PRIVATE_VALUE',d=>d.failure.code={secret:'PRIVATE_VALUE'},d=>d.failure=null,
  d=>d.status='PASS',d=>Object.defineProperty(d,'failure',{enumerable:true,get(){getterCalls++;return valid();}}),
  d=>Object.defineProperty(d.failure,'code',{enumerable:true,get(){getterCalls++;return 'NOT_FINALIZED';}}),
  d=>Object.defineProperty(d.failure,'secret',{value:'PRIVATE_VALUE'}),d=>d.failure[Symbol('secret')]='PRIVATE_VALUE',
  d=>Object.setPrototypeOf(d.failure,{secret:'PRIVATE_VALUE'})
 ];
 try{for(const change of changes){const result=base();change(result.driver);assert.throws(()=>api.retainPublicIntegrationResult(directory,result),e=>!e.message.includes('PRIVATE_VALUE'));assert.deepEqual(readdirSync(directory),[]);}assert.equal(getterCalls,0);}
 finally{rmSync(directory,{recursive:true,force:true});}
});

function initializedLaunchPlan(){
 const p=recoveryLaunchPlan(),d=p.existingDeployment,root=new URL('../../../deliverables/sp05-financial-integration-2026-09-09/',import.meta.url).pathname;delete p.existingDeployment;
 const store='/home/charl/.local/state/moriarty/sp05-local-loan-recovery-20260910-03';
 p.build.receiptPath='/home/charl/.local/state/moriarty/sp05-full-build-20260909-01/loan-output/build/build-receipt.json';p.build.sourceManifestHash='a29775a104dde9dbc38fdcbfdbc25a31db63beec84e09440f73441a27e9422e6';p.limits.submissions=2;p.privateState={directory:store+'/contract-state',passwordFile:store+'/password'};
 p.existingInitializedLoan={schema:'moriarty.existing-initialized-local-loan/1',deployTransactionFile:root+'local-execution-04/run-public/public-transactions/'+d.transactionHash+'.bin',deployTransactionHash:d.transactionHash,deployIdentifiers:d.identifiers,deployTxId:d.txId,deploySourceResultFile:root+'local-execution-04/attempt-result.json',deploySourceResultSha256:d.sourceResultSha256,deploySourceAllocationId:d.sourceAllocationId,initializeTransactionFile:root+'local-recovery-03/run-public/public-transactions/1f64634de2761fc0f140dbe7784a0cbf7005f226799e94d9878932378bc731e6.bin',initializeTransactionHash:'1f64634de2761fc0f140dbe7784a0cbf7005f226799e94d9878932378bc731e6',initializeIdentifiers:['004b2aa75cdb10a080c3560a0af50ab17ef6e2e2294e1609a9e087f18498dc1d05','006466368995501afc82b36cb38dac6f1cee531565eb75b70db6f3af5af36d9b46'],initializeTxId:'006466368995501afc82b36cb38dac6f1cee531565eb75b70db6f3af5af36d9b46',initializeSourceResultFile:root+'local-recovery-03/attempt-result.json',initializeSourceResultSha256:'858bea821333f092e42afacbf84386a0400ea6f1b350a76c6a23d66754e70aca',initializeSourceAllocationId:'sp05-local-loan-recovery-03',contractAddress:d.contractAddress,initializedStateSha256:'1f0724d2afe55e1c2aa22580f1bf9f0eed1a30e74b7a04ff78fe622a1e6fb30f',buildReceiptSha256:d.buildReceiptSha256,networkTag:d.networkTag,expectedProtocolVersion:d.expectedProtocolVersion,privateStateDirectory:p.privateState.directory,snapshotDirectory:'/private/initialized-snapshot',inspectionDirectory:'/synthetic-new-continuation/inspection'};return p;
}
test('initialized launch admits exactly two calls against the fixed existing store without private access',()=>{const p=initializedLaunchPlan(),copy=api.validateLocalLaunchPlan(p);assert.deepEqual(copy,p);assert.notEqual(copy,p);});
test('initialized launch rejects combined modes, replacement private state, identity and history changes',()=>{for(const mutate of [p=>p.existingDeployment=recoveryLaunchPlan().existingDeployment,p=>p.limits.submissions=3,p=>p.limits.submissions=4,p=>p.privateState.passwordFile='/other/password',p=>p.privateState.directory='/other/store',p=>p.roles.secretsFile='/other/roles',p=>p.wallet.seedFile='/other/seed',p=>p.existingInitializedLoan.initializedStateSha256=h,p=>p.existingInitializedLoan.snapshotDirectory=p.outputDirectory,p=>p.existingInitializedLoan.cursor='settle']){const p=initializedLaunchPlan();mutate(p);assert.throws(()=>api.validateLocalLaunchPlan(p));}});
test('public integration failure code is retained only as an allowlisted own data field on failure',()=>{
 const dir=mkdtempSync(join(tmpdir(),'moriarty-integration-code-'));let reads=0;
 const base=()=>Object.assign(Object.fromEntries('schema,status,kind,sourceTestOnly,networkAcceptance,proofAcceptance,financialAcceptance,build,assetBindings,phase,driver,cleanup,setupPendingOperations,comparisons,financialComparison,scope'.split(',').map(k=>[k,undefined])),{schema:'moriarty.local-financial-integration/1',status:'FAILED',sourceTestOnly:false,networkAcceptance:false,proofAcceptance:false,financialAcceptance:false,failureCode:'RECOVERY_STORE_METADATA'});
 try{const valid=base();api.retainPublicIntegrationResult(dir,valid);assert.equal(JSON.parse(readFileSync(join(dir,'integration-result.json'))).failureCode,valid.failureCode);
 for(const mutate of [r=>r.failureCode='PRIVATE_VALUE',r=>r.status='PASS',r=>Object.defineProperty(r,'failureCode',{enumerable:true,get(){reads++;return 'PRIVATE_VALUE';}})]){const r=base();mutate(r);assert.throws(()=>api.retainPublicIntegrationResult(dir,r),e=>!e.message.includes('PRIVATE_VALUE'));}assert.equal(reads,0);
 }finally{rmSync(dir,{recursive:true,force:true});}
});
