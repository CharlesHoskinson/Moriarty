import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync,mkdtempSync,rmSync,writeFileSync,symlinkSync} from 'node:fs';
import {join} from 'node:path';
import {tmpdir} from 'node:os';
import {createRequire} from 'node:module';
import {pathToFileURL,fileURLToPath} from 'node:url';
import {PINNED_NM} from './providers.mjs';
const api=await import('./recover-deployment.mjs').catch(()=>({}));
const ledger=await import(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs')).href);
const raw=readFileSync(new URL('../../../deliverables/sp05-financial-integration-2026-09-09/local-execution-04/run-public/public-transactions/0af6f3ed8960b1a7d1d5e8c204d9e133255e784dd2c5fad1f160c5212d02bd3c.bin',import.meta.url));
const native=()=>ledger.Transaction.deserialize('signature','proof','binding',raw);
const initial=()=>[...native().intents.values()].flatMap(i=>i.actions)[0].initialState;
test('recovery binds actual retained loan bytes, every identifier and original state without constructing a deployment',()=>{
 assert.equal(typeof api.inspectExistingLoanBytes,'function');
 const b=api.inspectExistingLoanBytes(raw,ledger);assert.equal(b.contractAddress,'ba4c808859fc2e4ee6d3d19fa0d812bb9a9c9eb0527161fb91315213bc24a713');assert.equal(b.txId,'00959c51e7d62ee9160bf1396ce0ab52f26757a7c5adec669cb083d5a8787d1de9');assert.equal(b.identifiers.length,2);assert.ok(b.oldDustNullifiers.length>0);assert.equal(b.spentUnshieldedInputs.length,0);
 const changed=Buffer.from(raw);changed[changed.length-1]^=1;assert.throws(()=>api.inspectExistingLoanBytes(changed,ledger),/RECOVERY_NATIVE_HASH/);
});
test('complete native state comparison rejects authority, operation-set and balance changes',()=>{
 assert.equal(typeof api.assertExistingLoanState,'function');assert.doesNotThrow(()=>api.assertExistingLoanState(initial(),ledger));
 const authority=initial();authority.maintenanceAuthority=new ledger.ContractMaintenanceAuthority(authority.maintenanceAuthority.committee,1,1n);assert.throws(()=>api.assertExistingLoanState(authority,ledger),/RECOVERY_STATE/);
 const balance=initial();balance.balance=new Map([[{tag:'unshielded',raw:'22'.repeat(32)},1n]]);assert.throws(()=>api.assertExistingLoanState(balance,ledger),/RECOVERY_STATE/);
 const extra=initial();extra.setOperation('unexpected',extra.operation('initialize'));assert.throws(()=>api.assertExistingLoanState(extra,ledger),/RECOVERY_STATE/);
});
function synced(){const progress=()=>({isConnected:true,isStrictlyComplete:()=>true});return {shielded:{progress:progress()},unshielded:{progress:progress(),availableCoins:[],pendingCoins:[]},dust:{progress:progress(),availableCoins:[{}],balance:()=>1000n,state:{state:{findUtxoByNullifier:()=>undefined},pendingDust:[]}}};}
test('recovery wallet gate requires strict sync, no pending or spent dust, and the full remaining cap',()=>{
 assert.equal(typeof api.assertRecoveryWallet,'function');const binding=api.inspectExistingLoanBytes(raw,ledger),now=Date.now();assert.doesNotThrow(()=>api.assertRecoveryWallet({synced:synced(),binding,timestampMs:now,dustCap:1000n}));
 for(const mutate of [s=>s.shielded.progress.isConnected=false,s=>s.unshielded.progress.isStrictlyComplete=()=>false,s=>s.dust.state.pendingDust=[{nullifier:0n}],s=>s.dust.state.state.findUtxoByNullifier=()=>({}),s=>s.dust.balance=()=>999n,s=>s.dust.availableCoins=[]]){const s=synced();mutate(s);assert.throws(()=>api.assertRecoveryWallet({synced:s,binding,timestampMs:now,dustCap:1000n}));}
 assert.throws(()=>api.assertRecoveryWallet({synced:synced(),binding,timestampMs:now-61000,dustCap:1000n}));
});
test('metadata gate checks entire real Level namespace and rejects additional state without leaking values',async()=>{
 assert.equal(typeof api.assertMetadataOnlyEntries,'function');const require=createRequire(join(PINNED_NM,'../package.json'));const {Level}=require('level');const {levelPrivateStateProvider}=await import(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/midnight-js-level-private-state-provider/dist/index.mjs')).href);
 const dir=mkdtempSync(join(tmpdir(),'moriarty-recovery-store-')),accountId='synthetic-account';
 try{const p=levelPrivateStateProvider({midnightDbName:dir,privateStateStoreName:'sp05-loan',accountId,privateStoragePasswordProvider:()=> 'a1'.repeat(32)});p.setContractAddress('aa'.repeat(32));await assert.rejects(p.set('sp05-loan',{}),e=>e.reason==='insufficient_classes');const db=new Level(dir);let entries;try{await db.open();entries=await db.iterator().all();}finally{await db.close();}
  assert.equal(api.assertMetadataOnlyEntries(entries,accountId),true);
  for(const bad of [[],[...entries,['unrelated','PRIVATE_VALUE']],[[entries[0][0],'{"salt":"bad","version":1}']],[[entries[0][0],JSON.stringify({salt:'11'.repeat(32),version:1,extra:true})]]])assert.throws(()=>api.assertMetadataOnlyEntries(bad,accountId),e=>!e.message.includes('PRIVATE_VALUE'));
  assert.throws(()=>api.assertMetadataOnlyEntries(entries,'wrong-account'));
 }finally{rmSync(dir,{recursive:true,force:true});}
});

function publicFixture(){
 const tx=native(),h='1acc3f7bc138076c0d74295c770375884dd99f121574be14960333e36d5195ed',txId='00959c51e7d62ee9160bf1396ce0ab52f26757a7c5adec669cb083d5a8787d1de9';
 const data={tx,status:'SucceedEntirely',txId,txHash:tx.transactionHash(),identifiers:[...tx.identifiers()],blockHeight:20313,blockHash:h,protocolVersion:1000000,fees:{paidFees:'0',estimatedFees:'0'},unshielded:{created:[],spent:[]}};
 const provider={async watchForTxData(){return data;},async queryContractState(){return initial();},async queryUnshieldedBalances(){return [];}};
 const rpc=async(method)=>method==='chain_getHeader'?{number:'0x4f59'}:'0x'+h;
 const options={raw,ledger,provider,rpc,decodeState:()=>({syntheticStateOnly:true}),deadlineMs:Date.now()+20000,expectedProtocolVersion:1000000,tip:{status:'READY',hash:h,finalizedHash:'0x'+h,height:20313,finalizedHeight:20313,timestampMs:Date.now()}};
 return {options,data};
}
test('public recovery gate verifies native receipt, canonical finality and whole current state through read-only interfaces',async()=>{
 const {options}=publicFixture();const r=await api.verifyExistingLoanPublic(options);assert.equal(r.status,'PUBLIC_STATE_VERIFIED');assert.equal(r.observation.receipt.circuitId,'deploy');assert.equal(r.binding.txId,options.provider?api.EXISTING_LOAN.txId:'');
});
test('public recovery gate stops on nonfinality, changed current state, receipt mismatch or stale tip',async()=>{
 for(const change of [f=>f.options.tip.timestampMs-=61000,f=>f.options.tip.finalizedHeight--,f=>f.data.status='FailEntirely',f=>f.data.blockHeight++,f=>f.options.rpc=async method=>method==='chain_getHeader'?{number:'0x1'}:'0x'+api.EXISTING_LOAN.blockHash,f=>f.options.provider.queryContractState=async()=>{const s=initial();s.maintenanceAuthority=new ledger.ContractMaintenanceAuthority(s.maintenanceAuthority.committee,1,1n);return s;}]){const f=publicFixture();change(f);await assert.rejects(api.verifyExistingLoanPublic(f.options));}
});

function recoveryPlanFixture(){
 const source=fileURLToPath(new URL('../../../deliverables/sp05-financial-integration-2026-09-09/local-execution-04/',import.meta.url));
 const original='/home/charl/.local/state/moriarty/sp05-local-loan-20260910-03',networkTag='e72f7a21a0397844563b4206f887b779ffa0d937c2d1b2339441faa1f08b9846';
 const recovery={schema:'moriarty.existing-local-loan/1',transactionFile:join(source,'run-public/public-transactions',api.EXISTING_LOAN.transactionHash+'.bin'),transactionHash:api.EXISTING_LOAN.transactionHash,identifiers:[...api.EXISTING_LOAN.identifiers],txId:api.EXISTING_LOAN.txId,contractAddress:api.EXISTING_LOAN.contractAddress,buildReceiptSha256:api.EXISTING_LOAN.buildReceiptSha256,networkTag,expectedProtocolVersion:1000000,sourceAllocationId:'sp05-local-loan-03',sourceResultFile:join(source,'attempt-result.json'),sourceResultSha256:'bf1d456714ea134ad7e320849b3841088b2a2a352435d75ced65854c48031f54',sourcePrivateStateDirectory:original+'/contract-state',inspectionDirectory:'/tmp/moriarty-synthetic-inspection',destinationDirectory:'/tmp/moriarty-synthetic-destination'};
 const plan={kind:'loan',limits:{submissions:3,allocationId:'synthetic-recovery'},build:{receiptSha256:recovery.buildReceiptSha256},networkTag,expectedProtocolVersion:1000000,roles:{secretsFile:original+'/roles.json'},privateState:{directory:recovery.destinationDirectory,passwordFile:'/tmp/moriarty-synthetic-password'},outputDirectory:'/tmp/moriarty-synthetic-output'};
 return {recovery,plan};
}
test('closed known recovery plan returns an independent identity and reads only retained public inputs',()=>{
 const {recovery,plan}=recoveryPlanFixture(),before=structuredClone(recovery);
 const validated=api.validateExistingLoanPlan(recovery,plan);assert.deepEqual(validated,before);assert.notEqual(validated,recovery);assert.notEqual(validated.identifiers,recovery.identifiers);
 const inputs=api.readExistingLoanInputs(validated,ledger);assert.ok(inputs.raw.equals(raw));assert.equal(inputs.binding.contractAddress,api.EXISTING_LOAN.contractAddress);assert.deepEqual(recovery,before);
});
for(const [name,mutate,code] of [
 ['unknown field',f=>f.recovery.privateState={},'FIELDS'],
 ['symbol field',f=>f.recovery[Symbol('hidden')]=true,'FIELDS'],
 ['hidden field',f=>Object.defineProperty(f.recovery,'hidden',{value:true}),'FIELDS'],
 ['prototype',f=>Object.setPrototypeOf(f.recovery,{extra:true}),'FIELDS'],
 ['getter',f=>Object.defineProperty(f.recovery,'txId',{get(){throw Error('GETTER_MUST_NOT_RUN');}}),'FIELDS'],
 ['swap',f=>f.plan.kind='swap','KIND_LIMIT'],
 ['four submissions',f=>f.plan.limits.submissions=4,'KIND_LIMIT'],
 ['source allocation',f=>f.recovery.sourceAllocationId='other','ALLOCATION'],
 ['reused allocation',f=>f.plan.limits.allocationId=f.recovery.sourceAllocationId,'ALLOCATION'],
 ['source digest',f=>f.recovery.sourceResultSha256='11'.repeat(32),'ALLOCATION'],
 ['address',f=>f.recovery.contractAddress='11'.repeat(32),'BINDING'],
 ['identifier order',f=>f.recovery.identifiers.reverse(),'BINDING'],
 ['hash instead of identifier',f=>f.recovery.txId=f.recovery.transactionHash,'BINDING'],
 ['build',f=>f.plan.build.receiptSha256='11'.repeat(32),'NETWORK_BUILD'],
 ['network',f=>f.recovery.networkTag='11'.repeat(32),'NETWORK_BUILD'],
 ['protocol',f=>f.plan.expectedProtocolVersion++,'NETWORK_BUILD'],
 ['relative path',f=>f.recovery.transactionFile='relative.bin','PATH'],
 ['parent path',f=>f.recovery.sourceResultFile='/tmp/foo/../result.json','PATH'],
 ['wrong source store',f=>f.recovery.sourcePrivateStateDirectory='/tmp/wrong','PRIVATE_BINDING'],
 ['wrong roles',f=>f.plan.roles.secretsFile='/tmp/roles.json','PRIVATE_BINDING'],
 ['wrong destination',f=>f.plan.privateState.directory='/tmp/other','PRIVATE_BINDING'],
 ['same inspection',f=>f.recovery.inspectionDirectory=f.recovery.destinationDirectory,'OVERLAP'],
 ['nested inspection',f=>f.recovery.inspectionDirectory=f.recovery.destinationDirectory+'/child','OVERLAP'],
 ['parent inspection',f=>f.recovery.inspectionDirectory='/tmp','OVERLAP'],
 ['old password',f=>f.plan.privateState.passwordFile='/home/charl/.local/state/moriarty/sp05-local-loan-20260910-03/password','PRESERVE_ORIGINAL'],
 ['old output',f=>f.plan.outputDirectory='/home/charl/.local/state/moriarty/sp05-local-loan-20260910-03','PRESERVE_ORIGINAL']
])test(`closed recovery plan rejects ${name}`,()=>{
 const f=recoveryPlanFixture();mutate(f);assert.throws(()=>api.validateExistingLoanPlan(f.recovery,f.plan),new RegExp('RECOVERY_PLAN_'+code));
});
test('public input reads reject tampered bytes, source result and symlink paths',()=>{
 const dir=mkdtempSync(join(tmpdir(),'moriarty-recovery-public-'));
 try{
  const {recovery}=recoveryPlanFixture(),result=readFileSync(recovery.sourceResultFile),txPath=join(dir,'tx.bin'),resultPath=join(dir,'result.json');
  writeFileSync(txPath,raw);writeFileSync(resultPath,result);const local={...recovery,transactionFile:txPath,sourceResultFile:resultPath};
  assert.ok(api.readExistingLoanInputs(local,ledger).raw.equals(raw));
  const tampered=Buffer.from(raw);tampered[0]^=1;writeFileSync(txPath,tampered);assert.throws(()=>api.readExistingLoanInputs(local,ledger),/RECOVERY_NATIVE_HASH/);writeFileSync(txPath,raw);
  for(const mutate of [x=>x.allocationId='other',x=>x.reservedSubmissions=0,x=>x.reservedDustFee='0',x=>x.financialStagesCompleted=1]){const changed=JSON.parse(result);mutate(changed);writeFileSync(resultPath,JSON.stringify(changed));assert.throws(()=>api.readExistingLoanInputs(local,ledger),/RECOVERY_SOURCE_RESULT/);}
  writeFileSync(resultPath,result);symlinkSync(txPath,join(dir,'tx-link'));assert.throws(()=>api.readExistingLoanInputs({...local,transactionFile:join(dir,'tx-link')},ledger),/RECOVERY_PUBLIC_SYMLINK/);
  symlinkSync(dir,join(dir,'parent-link'));assert.throws(()=>api.readExistingLoanInputs({...local,sourceResultFile:join(dir,'parent-link/result.json')},ledger),/RECOVERY_PUBLIC_SYMLINK/);
  writeFileSync(resultPath,'');assert.throws(()=>api.readExistingLoanInputs(local,ledger),/RECOVERY_PUBLIC_FILE/);
 }finally{rmSync(dir,{recursive:true,force:true});}
});
test('unminted private restore results fail before any provider access',async()=>{
 let accessed=0;const provider=new Proxy({},{get(){accessed++;throw Error('PROVIDER_MUST_NOT_BE_ACCESSED');}});
 for(const result of [undefined,null,{},Object.freeze({privateState:{},signingKey:'synthetic-key'}),{status:'RESTORED',contractAddress:api.EXISTING_LOAN.contractAddress,txId:api.EXISTING_LOAN.txId}])await assert.rejects(api.restoreExistingLoanPrivate(provider,result),/RECOVERY_RESTORE_UNVERIFIED/);
 assert.equal(accessed,0);
});
