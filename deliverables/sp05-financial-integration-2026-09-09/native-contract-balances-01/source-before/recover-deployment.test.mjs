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
 options.readCurrentTip=async()=>({...options.tip});
 return {options,data};
}
test('public recovery gate verifies native receipt, canonical finality and whole current state through read-only interfaces',async()=>{
 const {options}=publicFixture();const r=await api.verifyExistingLoanPublic(options);assert.equal(r.status,'PUBLIC_STATE_VERIFIED');assert.equal(r.observation.receipt.circuitId,'deploy');assert.equal(r.binding.txId,options.provider?api.EXISTING_LOAN.txId:'');
});
test('public recovery gate stops on nonfinality, changed current state, receipt mismatch or stale tip',async()=>{
 for(const change of [f=>f.options.tip.timestampMs-=61000,f=>f.options.tip.finalizedHeight-=3,f=>f.data.status='FailEntirely',f=>f.data.blockHeight++,f=>f.options.rpc=async method=>method==='chain_getHeader'?{number:'0x1'}:'0x'+api.EXISTING_LOAN.blockHash,f=>f.options.provider.queryContractState=async()=>{const s=initial();s.maintenanceAuthority=new ledger.ContractMaintenanceAuthority(s.maintenanceAuthority.committee,1,1n);return s;}]){const f=publicFixture();change(f);await assert.rejects(api.verifyExistingLoanPublic(f.options));}
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

function movingFinalityFixture({changedCurrent=false,movesDuringQuery=false,lag=0}={}){
 const f=publicFixture(),oldHash=api.EXISTING_LOAN.blockHash,newHash='ab'.repeat(32),laterHash='cd'.repeat(32),height=api.EXISTING_LOAN.blockHeight;
 let headReads=0;const rpcCalls=[],stateHashes=[];
 f.options.tip.finalizedHash=lag?'0x'+newHash:'0x'+oldHash;f.options.tip.finalizedHeight=height+lag;
 f.options.rpc=async(method,args)=>{
  rpcCalls.push({method,args});
  if(method==='chain_getFinalizedHead'){headReads++;return '0x'+(headReads===1?oldHash:headReads===2?newHash:movesDuringQuery?laterHash:newHash);}
  if(method==='chain_getHeader')return {number:'0x'+(args[0]==='0x'+oldHash?height:args[0]==='0x'+newHash?height+1:height+2).toString(16)};
  if(method==='chain_getBlockHash')return '0x'+(args[0]===height?oldHash:args[0]===height+1?newHash:laterHash);
  throw Error('UNEXPECTED_READ_RPC');
 };
 f.options.provider.queryContractState=async(_address,query)=>{
  const hash=query?.blockHash?.replace(/^0x/,'')??'latest';stateHashes.push(hash);const s=initial();
  if(changedCurrent&&hash==='latest')s.maintenanceAuthority=new ledger.ContractMaintenanceAuthority(s.maintenanceAuthority.committee,1,1n);
  return s;
 };
 f.options.readCurrentTip=async()=>({status:'READY',hash:newHash,height:height+1,finalizedHash:'0x'+newHash,finalizedHeight:height+1,timestampMs:Date.now()});
 return {...f,newHash,oldHash,rpcCalls,stateHashes,headReads:()=>headReads};
}
test('recovery detects later finalized state transition after prefetched tip',async()=>{
 const f=movingFinalityFixture({changedCurrent:true});
 await assert.rejects(api.verifyExistingLoanPublic(f.options),/RECOVERY_STATE_MISMATCH/);
 assert.ok(f.stateHashes.includes('latest'),'must query latest state after exact indexed/finalized coverage');
});
test('later finalized head with unchanged constructor state passes after final head recheck',async()=>{
 const f=movingFinalityFixture(),result=await api.verifyExistingLoanPublic(f.options);
 assert.equal(result.status,'PUBLIC_STATE_VERIFIED');assert.deepEqual(result.stateBlock,{hash:'0x'+f.newHash,height:api.EXISTING_LOAN.blockHeight+1});assert.ok(f.stateHashes.includes('latest'));assert.ok(f.headReads()>=3);assert.equal(f.rpcCalls.at(-1).method,'chain_getHeader');
});
test('recovery rejects finalized head movement across current state query',async()=>{
 const f=movingFinalityFixture({movesDuringQuery:true});await assert.rejects(api.verifyExistingLoanPublic(f.options),/RECOVERY_SNAPSHOT_UNSTABLE|RECOVERY_INDEXER_FINALITY|RECOVERY_FINALITY_REGRESSION/);assert.ok(f.stateHashes.includes('latest'));assert.ok(f.headReads()>=3);
});
test('earlier one-block indexed lag must catch up before latest-state observation',async()=>{
 const f=movingFinalityFixture({lag:1}),result=await api.verifyExistingLoanPublic(f.options);assert.equal(result.status,'PUBLIC_STATE_VERIFIED');assert.ok(f.stateHashes.includes('latest'));
});
test('indexed tip ahead of fresh finalized head is rejected',async()=>{
 const f=movingFinalityFixture();f.options.tip.height+=2;f.options.tip.hash='cd'.repeat(32);f.options.tip.finalizedHeight=f.options.tip.height;f.options.tip.finalizedHash='0x'+f.options.tip.hash;
 await assert.rejects(api.verifyExistingLoanPublic(f.options));
});
test('indexed tip outside two-block bound or not canonical is rejected',async()=>{
 for(const mutate of [f=>{f.options.tip.height-=3;},f=>{f.options.tip.hash='ef'.repeat(32);}]){const f=movingFinalityFixture();mutate(f);await assert.rejects(api.verifyExistingLoanPublic(f.options));}
});

// Indexer4.3.3's SQL block filter is equality: an empty later block has no
// contractAction row, while offset:null returns the last recorded state.
function exactBlockStateFixture() {
 const f=publicFixture(),oldHash=api.EXISTING_LOAN.blockHash,newHash='ac'.repeat(32),height=api.EXISTING_LOAN.blockHeight+1;
 const indexed={status:'READY',hash:newHash,height,finalizedHash:'0x'+newHash,finalizedHeight:height,timestampMs:Date.now()};
 const stateReads=[],tipReads=[];
 f.options.rpc=async(method,args)=>method==='chain_getFinalizedHead'?'0x'+newHash:method==='chain_getHeader'?{number:'0x'+height.toString(16)}:'0x'+(args[0]===height?newHash:oldHash);
 f.options.readCurrentTip=async()=>{tipReads.push('read');return {...indexed};};
 f.options.provider.queryContractState=async(_address,config)=>{stateReads.push(config?.blockHash??'latest');return config&&config.blockHash!==oldHash?null:initial();};
 return {...f,indexed,stateReads,tipReads,newHash,height};
}
test('recovery reads latest state when the finalized head has no contract action',async()=>{
 const f=exactBlockStateFixture();const result=await api.verifyExistingLoanPublic(f.options);
 assert.equal(result.status,'PUBLIC_STATE_VERIFIED');assert.deepEqual(result.stateBlock,{hash:'0x'+f.newHash,height:f.height});
 assert.ok(f.stateReads.includes('latest'));assert.ok(!f.stateReads.includes(f.newHash));assert.equal(f.tipReads.length,2);
});
test('latest state is rejected when indexed coverage is behind current finality',async()=>{
 const f=exactBlockStateFixture();f.options.provider.queryContractState=async()=>initial();
 f.options.readCurrentTip=async()=>({...f.indexed,height:f.height-1,hash:api.EXISTING_LOAN.blockHash});
 await assert.rejects(api.verifyExistingLoanPublic(f.options),/RECOVERY_INDEXER_FINALITY/);
});
test('latest state is rejected when indexer coverage changes during observation',async()=>{
 const f=exactBlockStateFixture();f.options.provider.queryContractState=async()=>initial();let reads=0;
 f.options.readCurrentTip=async()=>++reads===1?{...f.indexed}:{...f.indexed,hash:'ed'.repeat(32),finalizedHash:'0x'+'ed'.repeat(32),height:f.height+1,finalizedHeight:f.height+1};
 await assert.rejects(api.verifyExistingLoanPublic(f.options),/RECOVERY_SNAPSHOT_UNSTABLE|RECOVERY_INDEXER_FINALITY|RECOVERY_FINALITY_REGRESSION/);
});


function oneHeadAdvanceFixture(){
 const f=exactBlockStateFixture(),rpc=f.options.rpc;let heads=0;
 f.options.rpc=async(method,args,...rest)=>{
  if(method==='chain_getFinalizedHead'&&++heads===1)return '0x'+api.EXISTING_LOAN.blockHash;
  if(method==='chain_getHeader'&&args[0]==='0x'+api.EXISTING_LOAN.blockHash)return {number:'0x'+api.EXISTING_LOAN.blockHeight.toString(16)};
  return rpc(method,args,...rest);
 };
 return f;
}
test('one forward head advance between indexed sample and node read is sampled again before private recovery',async()=>{
 const f=oneHeadAdvanceFixture();let samples=0,watches=0;
 const watch=f.options.provider.watchForTxData;f.options.provider.watchForTxData=async(...args)=>{watches++;return watch(...args);};
 f.options.readCurrentTip=async()=>{samples++;return samples===1?{...f.options.tip}:{...f.indexed};};
 const result=await api.verifyExistingLoanPublic(f.options);
 assert.equal(result.status,'PUBLIC_STATE_VERIFIED');assert.equal(watches,1);assert.ok(samples>=3);assert.equal(result.stateBlock.height,f.height);
});
test('persistent forward movement is rejected within the existing initial-tip bound',async()=>{
 const f=exactBlockStateFixture();let samples=0,current=f.height;const hash=h=>h===api.EXISTING_LOAN.blockHeight?api.EXISTING_LOAN.blockHash:h.toString(16).padStart(64,'0');
 // The retained two-block initial-tip bound can stop before the six-sample ceiling.
 f.options.readCurrentTip=async()=>{samples++;return {...f.indexed,height:current,finalizedHeight:current,hash:hash(current),finalizedHash:'0x'+hash(current)};};
 f.options.rpc=async(method,args)=>{
  if(method==='chain_getFinalizedHead'){if(samples)current++;return '0x'+hash(current);}
  if(method==='chain_getHeader')return {number:'0x'+(args[0]==='0x'+api.EXISTING_LOAN.blockHash?api.EXISTING_LOAN.blockHeight:Number(BigInt(args[0]))).toString(16)};
  return '0x'+hash(args[0]);
 };
 await assert.rejects(api.verifyExistingLoanPublic(f.options),/RECOVERY_CURRENT_HEIGHT/);assert.ok(samples<=6);assert.ok(!f.stateReads.includes('latest'));
});
test('same-height conflicting coverage is fatal without sampling again',async()=>{
 const f=exactBlockStateFixture();let samples=0;
 f.options.readCurrentTip=async()=>{samples++;return {...f.indexed,hash:'11'.repeat(32),finalizedHash:'0x'+'11'.repeat(32)};};
 await assert.rejects(api.verifyExistingLoanPublic(f.options),/RECOVERY_INDEXER_FINALITY/);assert.equal(samples,1);
});


test('sampling passes its bounded deadline into the reader and does not retry provider failure',async()=>{
 const f=exactBlockStateFixture(),failure=Error('SYNTHETIC_PROVIDER_FAILURE');let calls=0;
 f.options.deadlineMs=Date.now()+120000;
 f.options.readCurrentTip=async stop=>{calls++;assert.ok(stop<=Date.now()+60000);assert.ok(stop<f.options.deadlineMs);throw failure;};
 await assert.rejects(api.verifyExistingLoanPublic(f.options),e=>e===failure);assert.equal(calls,1);
});
test('a finalized snapshot regressing after an observed advance is fatal',async()=>{
 const f=oneHeadAdvanceFixture();let calls=0;f.options.readCurrentTip=async()=>{calls++;return {...f.options.tip};};
 await assert.rejects(api.verifyExistingLoanPublic(f.options),/RECOVERY_FINALITY_REGRESSION/);assert.equal(calls,2);
});

test('actual SDK decoded coin wrappers reject a retained spent input in available or pending coins',async()=>{
 const require=createRequire(join(PINNED_NM,'../package.json'));const {Schema}=require('effect');
 const {WalletSyncUpdateSchema}=await import(pathToFileURL(join(PINNED_NM,'@midnight-ntwrk/wallet-sdk-unshielded-wallet/dist/v1/SyncSchema.js')).href);
 // Synthetic subscription metadata and spent history, actual initialized output identity.
 const wire={type:'UnshieldedTransaction',transaction:{id:35,hash:'1f64634de2761fc0f140dbe7784a0cbf7005f226799e94d9878932378bc731e6',type:'RegularTransaction',protocolVersion:1000000,block:{timestamp:1789027704001},transactionResult:{status:'SUCCESS',segments:null}},createdUtxos:[{value:'20000000000',owner:'mn_addr_undeployed1n2w7v4y79630m5u40rpm6tn0qvnm83vptu9vqcwzqppam7pfrr9sa6q9r9',tokenType:'e92df6339320f55209d4586ce039b6ef05a7be960999fca913006146cb72cdef',intentHash:'4d343d21f150af922dc483b319b87b069a2b2cb8e3f3f64f4bad4485dbf61603',outputIndex:0,ctime:1,registeredForDustGeneration:false}],spentUtxos:[]};
 const coin=Schema.decodeUnknownSync(WalletSyncUpdateSchema)(wire).createdUtxos[0];
 assert.equal(coin.intentHash,undefined);assert.equal(coin.utxo.outputNo,0);assert.equal(coin.utxo.value,20000000000n);
 const original=api.inspectExistingLoanBytes(raw,ledger);assert.equal(original.spentUnshieldedInputs.length,0,'original deployment has no unshielded spend to miss');
 const binding={...original,spentUnshieldedInputs:[{intentHash:coin.utxo.intentHash,outputNo:coin.utxo.outputNo}]};
 for(const list of ['availableCoins','pendingCoins']){const s=synced();s.unshielded[list]=[coin];assert.throws(()=>api.assertRecoveryWallet({synced:s,binding,timestampMs:Date.now(),dustCap:1000n}),/RECOVERY_WALLET_SPENT_INPUT/);}
 const other=synced();other.unshielded.availableCoins=[{...coin,utxo:{...coin.utxo,outputNo:1}}];assert.doesNotThrow(()=>api.assertRecoveryWallet({synced:other,binding,timestampMs:Date.now(),dustCap:1000n}));
});

test('wallet coin identity rejects flattened and malformed wrappers',()=>{
 const binding=api.inspectExistingLoanBytes(raw,ledger);
 for(const coin of [null,{}, {intentHash:'aa'.repeat(32),outputNo:0},{utxo:{intentHash:'bad',outputNo:0}},{utxo:{intentHash:'aa'.repeat(32),outputNo:-1}},{utxo:{intentHash:'aa'.repeat(32),outputNo:0.5}}]){
  const s=synced();s.unshielded.availableCoins=[coin];assert.throws(()=>api.assertRecoveryWallet({synced:s,binding,timestampMs:Date.now(),dustCap:1000n}),/RECOVERY_WALLET_COIN/);
 }
});

const initializedRaw=readFileSync(new URL('../../../deliverables/sp05-financial-integration-2026-09-09/local-recovery-03/run-public/public-transactions/1f64634de2761fc0f140dbe7784a0cbf7005f226799e94d9878932378bc731e6.bin',import.meta.url));
const initializedBytes=readFileSync(new URL('../../../deliverables/sp05-financial-integration-2026-09-09/local-recovery-03/indexed-initialize-state.bin',import.meta.url));
const initializedState=()=>ledger.ContractState.deserialize(initializedBytes);
const initializeId='006466368995501afc82b36cb38dac6f1cee531565eb75b70db6f3af5af36d9b46',initializeHash='1f64634de2761fc0f140dbe7784a0cbf7005f226799e94d9878932378bc731e6',initializeBlock='7f61e4c7225c456400e852f3648cf7fcad958bb05ed84002441782764093c94f';
function initializedPublicFixture(){
 const f=publicFixture(),watches=[],stateReads=[],rpcReads=[],headHash='ab'.repeat(32),headHeight=20364;
 const tx=ledger.Transaction.deserialize('signature','proof','binding',initializedRaw);
 const initializedData={...f.data,tx,txId:initializeId,txHash:initializeHash,identifiers:[...tx.identifiers()],blockHeight:20363,blockHash:initializeBlock,unshielded:{created:[{owner:'mn_addr_undeployed1n2w7v4y79630m5u40rpm6tn0qvnm83vptu9vqcwzqppam7pfrr9sa6q9r9',tokenType:'e92df6339320f55209d4586ce039b6ef05a7be960999fca913006146cb72cdef',value:20000000000n,intentHash:'4d343d21f150af922dc483b319b87b069a2b2cb8e3f3f64f4bad4485dbf61603'}],spent:[]}};
 f.options.rawInitialize=initializedRaw;
 f.options.tip={status:'READY',hash:headHash,height:headHeight,finalizedHash:'0x'+headHash,finalizedHeight:headHeight,timestampMs:Date.now()};
 f.options.readCurrentTip=async()=>({...f.options.tip});
 f.options.provider.watchForTxData=async id=>{watches.push(id);assert.ok([api.EXISTING_LOAN.txId,initializeId].includes(id));return id===initializeId?initializedData:f.data;};
 f.options.provider.queryContractState=async(_address,config)=>{const h=config?.blockHash??'latest';stateReads.push(h);return h===api.EXISTING_LOAN.blockHash?initial():initializedState();};
 f.options.rpc=async(method,args)=>{rpcReads.push({method,args});if(method==='chain_getHeader')return {number:'0x'+headHeight.toString(16)};if(method==='chain_getFinalizedHead')return '0x'+headHash;assert.equal(method,'chain_getBlockHash');return '0x'+(args[0]===20313?api.EXISTING_LOAN.blockHash:args[0]===20363?initializeBlock:headHash);};
 return {...f,initializedData,watches,stateReads,rpcReads,headHash,headHeight};
}
test('initialized public gate observes each historical transaction once and verifies complete initialized snapshot',async()=>{
 assert.equal(typeof api.verifyInitializedLoanPublic,'function');const f=initializedPublicFixture();const result=await api.verifyInitializedLoanPublic(f.options);
 assert.deepEqual(Buffer.from(result.initializeContractState.serialize()),initializedBytes);
 assert.equal(result.status,'INITIALIZED_PUBLIC_STATE_VERIFIED');assert.equal(result.dispatchAuthorized,false);assert.equal(result.deploymentObservation.receipt.circuitId,'deploy');assert.equal(result.initializeObservation.receipt.circuitId,'initialize');
 assert.deepEqual(f.watches,[api.EXISTING_LOAN.txId,initializeId]);assert.deepEqual(result.stateBlock,{hash:'0x'+f.headHash,height:f.headHeight});assert.equal(result.snapshotSamples,1);
 assert.equal(result.binding.initialize.txId,initializeId);assert.equal(result.binding.deployment.txId,api.EXISTING_LOAN.txId);assert.equal(result.binding.contractAddress,api.EXISTING_LOAN.contractAddress);
 assert.deepEqual(result.binding.oldDustNullifiers,[...result.binding.deployment.oldDustNullifiers,...result.binding.initialize.oldDustNullifiers]);assert.deepEqual(result.binding.spentUnshieldedInputs,[]);assert.equal(result.binding.mintedOutput.intentHash,'4d343d21f150af922dc483b319b87b069a2b2cb8e3f3f64f4bad4485dbf61603');
 assert.ok(Object.isFrozen(result)&&Object.isFrozen(result.binding)&&Object.isFrozen(result.binding.oldDustNullifiers));assert.ok(f.stateReads.includes('latest'));assert.ok(f.stateReads.filter(h=>h===initializeBlock).length>=2);
});
for(const [name,change,code] of [
 ['changed initialize bytes',f=>{f.options.rawInitialize=Buffer.from(initializedRaw);f.options.rawInitialize[0]^=1;},'INITIALIZED_NATIVE_HASH'],
 ['wrong initialize identifier',f=>f.initializedData.txId=api.EXISTING_LOAN.txId,'TRANSACTION_ID_MISMATCH'],
 ['wrong initialize hash',f=>f.initializedData.txHash='00'.repeat(32),'TRANSACTION_HASH_MISMATCH'],
 ['wrong origin',f=>f.initializedData.unshielded.created[0].intentHash='00'.repeat(32),'INDEXED_OUTPUTS_MISMATCH'],
 ['failed initialize status',f=>f.initializedData.status='FailEntirely','TRANSACTION_STATUS'],
 ['failed initialize segment',f=>f.initializedData.segmentStatusMap=new Map([[21861,'SegmentFail']]),'SEGMENT_FAILURE'],
 ['wrong initialize block identity',f=>{f.initializedData.blockHeight=20362;f.initializedData.blockHash=f.headHash;},'INITIALIZED_DEPLOYMENT_HISTORY'],
 ['wrong protocol',f=>f.options.expectedProtocolVersion=9,'INITIALIZED_PROTOCOL'],
 ['stale prefetched time',f=>f.options.tip.timestampMs-=61000,'RECOVERY_TIP_TIME'],
 ['not canonical deployment',f=>{const rpc=f.options.rpc;f.options.rpc=(method,args)=>method==='chain_getBlockHash'&&args[0]===20313?'0x'+'00'.repeat(32):rpc(method,args);},'NONCANONICAL_BLOCK'],
 ['not canonical initialize',f=>{const rpc=f.options.rpc;f.options.rpc=(method,args)=>method==='chain_getBlockHash'&&args[0]===20363?'0x'+'00'.repeat(32):rpc(method,args);},'NONCANONICAL_BLOCK'],
])test(`initialized public gate rejects ${name}`,async()=>{
 assert.equal(typeof api.verifyInitializedLoanPublic,'function');const f=initializedPublicFixture();change(f);await assert.rejects(api.verifyInitializedLoanPublic(f.options),new RegExp(code));assert.ok(f.watches.length<=2);
});
for(const target of ['deploy','initialize','latest'])test(`initialized public gate rejects wrong complete ${target} state`,async()=>{
 assert.equal(typeof api.verifyInitializedLoanPublic,'function');const f=initializedPublicFixture(),query=f.options.provider.queryContractState;
 f.options.provider.queryContractState=async(a,config)=>{const match=target==='deploy'?config?.blockHash===api.EXISTING_LOAN.blockHash:target==='initialize'?config?.blockHash===initializeBlock:config===undefined;if(!match)return query(a,config);const state=target==='deploy'?initial():initializedState();state.maintenanceAuthority=new ledger.ContractMaintenanceAuthority(state.maintenanceAuthority.committee,1,1n);return state;};
 await assert.rejects(api.verifyInitializedLoanPublic(f.options),/RECOVERY_STATE_MISMATCH|INITIALIZED_STATE_MISMATCH/);assert.deepEqual(f.watches,[api.EXISTING_LOAN.txId,initializeId]);
});
test('initialized public gate samples forward movement without repeating history watches',async()=>{
 assert.equal(typeof api.verifyInitializedLoanPublic,'function');const f=initializedPublicFixture();let tips=0;const newerHash='cd'.repeat(32);const oldRpc=f.options.rpc;
 f.options.readCurrentTip=async()=>{tips++;return tips===1?{...f.options.tip}:{...f.options.tip,hash:newerHash,height:f.headHeight+1,finalizedHash:'0x'+newerHash,finalizedHeight:f.headHeight+1};};
 f.options.rpc=async(method,args)=>{if(tips&&method==='chain_getFinalizedHead')return '0x'+newerHash;if(tips&&method==='chain_getHeader')return {number:'0x'+(f.headHeight+1).toString(16)};return oldRpc(method,args);};
 const out=await api.verifyInitializedLoanPublic(f.options);assert.equal(out.snapshotSamples,2);assert.deepEqual(f.watches,[api.EXISTING_LOAN.txId,initializeId]);
});
test('initialized public gate stops on current snapshot provider error or same-height reorg',async()=>{
 assert.equal(typeof api.verifyInitializedLoanPublic,'function');const f=initializedPublicFixture(),error=Error('READ_ONLY_PROVIDER_FAILED');f.options.readCurrentTip=async()=>{throw error;};await assert.rejects(api.verifyInitializedLoanPublic(f.options),e=>e===error);assert.deepEqual(f.watches,[api.EXISTING_LOAN.txId,initializeId]);
 const g=initializedPublicFixture();g.options.readCurrentTip=async()=>({...g.options.tip,hash:'ee'.repeat(32),finalizedHash:'0x'+'ee'.repeat(32)});await assert.rejects(api.verifyInitializedLoanPublic(g.options),/RECOVERY_INDEXER_FINALITY/);
});
test('initialized public gate times out rather than treating indexed inclusion as finality',async()=>{
 assert.equal(typeof api.verifyInitializedLoanPublic,'function');const f=initializedPublicFixture();f.options.deadlineMs=Date.now()+180;f.options.rpc=async(method,args)=>method==='chain_getHeader'?{number:'0x'+(20362).toString(16)}:method==='chain_getBlockHash'?'0x'+api.EXISTING_LOAN.blockHash:'0x'+f.headHash;
 await assert.rejects(api.verifyInitializedLoanPublic(f.options),/NOT_FINALIZED|DEADLINE_EXPIRED|OBSERVATION_TIMEOUT_UNKNOWN/);assert.ok(!f.stateReads.includes('latest'));
});
