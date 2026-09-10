import test from 'node:test';
import assert from 'node:assert/strict';
import {mkdtempSync,writeFileSync,chmodSync,symlinkSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {spawnSync} from 'node:child_process';
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
