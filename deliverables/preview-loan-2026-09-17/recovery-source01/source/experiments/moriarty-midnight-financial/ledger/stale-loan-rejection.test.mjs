import test from 'node:test';import assert from 'node:assert/strict';import {readFileSync} from 'node:fs';
const api=await import('./stale-loan-rejection.mjs').catch(e=>{if(e.code==='ERR_MODULE_NOT_FOUND')return {};throw e;});
const observed=JSON.parse(readFileSync(new URL('../../../deliverables/sp05-financial-integration-2026-09-09/local-finalized-state-02/probe-result.json',import.meta.url))).snapshots[0];
const fixture=()=>structuredClone(observed);
test('full native state including zero balance entry must remain unchanged at later anchor',()=>{assert.equal(typeof api.compareStaleLoanSnapshots,'function');const before=fixture(),after=fixture();after.blockHeight++;after.blockHash='0x'+'ab'.repeat(32);assert.equal(api.compareStaleLoanSnapshots(before,after).status,'FINANCIAL_STATE_UNCHANGED');});
for(const [name,mutate,code] of [['same anchor',x=>{},'STALE_LOAN_AFTER_ANCHOR'],['missing zero balance',x=>{x.balances={};},'STALE_LOAN_FINANCIAL_MUTATION'],['debt mutation',x=>{x.state.kernelState.f0='0';},'STALE_LOAN_FINANCIAL_MUTATION'],['native mutation',x=>{x.serializedStateHex+='00';},'STALE_LOAN_CURRENT_STATE']])test(name+' is not adverse nonmutation evidence',()=>{const before=fixture(),after=fixture();if(name!=='same anchor'){after.blockHeight++;after.blockHash='0x'+'ab'.repeat(32);}mutate(after);assert.throws(()=>api.compareStaleLoanSnapshots(before,after),e=>e.message===code);});
test('generic submission and timeout failures cannot establish node rejection',()=>{assert.equal(typeof api.classifyStaleLoanFailure,'function');for(const e of [Error('Transaction submission failed'),Error('deadline'),new TypeError('fetch failed')])assert.equal(api.classifyStaleLoanFailure(e,'submit').nodeRejectionEstablished,false);});
const {PINNED_NM}=await import('./providers.mjs');const {pathToFileURL}=await import('node:url');
const url=p=>pathToFileURL(PINNED_NM+'/'+p).href;
const sdkUrl=url('@midnight-ntwrk/midnight-js-contracts/dist/index.mjs'),realSdk=await import(sdkUrl);
const runtime=await import(url('@midnight-ntwrk/midnight-js-protocol/dist/compact-runtime.mjs')),ledger=await import(url('@midnight-ntwrk/midnight-js-protocol/dist/ledger.mjs'));
const effect=await import(url('effect/dist/esm/index.js'));const errors=await import(url('@midnight-ntwrk/wallet-sdk-node-client/dist/effect/NodeClientError.js'));
test('actual Effect FiberFailure invalid status is bound to exact submitted bytes',async()=>{
 const raw=new Uint8Array([1,2,3]);let error;try{await effect.Effect.runPromise(effect.Effect.fail(new errors.TransactionInvalidError({message:'public node invalid',txData:raw})));}catch(e){error=e;}
 assert.equal(api.classifyStaleLoanFailure(error,'submit',raw).nodeRejectionEstablished,true);
 assert.equal(api.classifyStaleLoanFailure(error,'submit',new Uint8Array([4])).nodeRejectionEstablished,false);
 assert.equal(api.classifyStaleLoanFailure(error,'prove',raw).nodeRejectionEstablished,false);
 assert.equal(api.classifyStaleLoanFailure({_tag:'TransactionInvalidError',txData:raw},'submit',raw).nodeRejectionEstablished,false);
});
test('real SDK/generated accrue prepares with synthetic capability only; actual state/parameters interfaces connected',async t=>{
 let clockOffset=0,currentMode='';const realNow=Date.now;t.mock.method(Date,'now',()=>realNow()+clockOffset);
 const loaderUrl=new URL('./proven-assets.mjs',import.meta.url).href,realLoader=await import(loaderUrl);
 t.mock.module(loaderUrl,{namedExports:{loadProvenFinancialContract:async o=>{const loaded=await realLoader.loadProvenFinancialContract(o);return {...loaded,cleanup(){const result=loaded.cleanup();if(currentMode==='cleanupLate')clockOffset+=90000;return result;}};}}});
 const receiptPath=process.env.MORIARTY_LOAN_BUILD_RECEIPT;assert.ok(receiptPath,'MORIARTY_LOAN_BUILD_RECEIPT is mandatory; no rebuild/skip');
 const {loadFinancialSdk}=await import('./providers.mjs');const sdkProviders=await loadFinancialSdk();sdkProviders.setNetworkId('undeployed');
 const secret=new Uint8Array(32).fill(7);const b=s=>Uint8Array.from(Buffer.from(s,'hex'));
 const commitment=runtime.persistentHash(new runtime.CompactTypeVector(4,new runtime.CompactTypeBytes(32)),[b(Buffer.from('moriarty:sp05:loan:borrower'.padEnd(32,'\0')).toString('hex')),b('e72f7a21a0397844563b4206f887b779ffa0d937c2d1b2339441faa1f08b9846'),b('95b46e39a9039e19063bb3d618128aec6cbd9ee656b6e635b3587e7f3f5235b2'),secret]);
 let sdkCalls=0,queryCalls=0;
 t.mock.module(new URL('./continue-initialized-loan.mjs',import.meta.url).href,{namedExports:{verifyInitializedLoanPrivate:async()=>({status:'PRIVATE_STATE_CHECKED'})}});
 t.mock.module(sdkUrl,{namedExports:{...realSdk,createUnprovenCallTxFromInitialStates:async(zk,options,enc)=>{
  sdkCalls++;assert.equal(options.ledgerParameters,parameters);assert.equal(options.initialZswapChainState,zswap);assert.equal(options.circuitId,'accrue');assert.equal(options.args[3],0n);
  const hex=Buffer.from(options.initialContractState.serialize()).toString('hex'),original='01a11721cbbf9e0c1b73e66f8f1b7c5c7971b6446e1ca6563eb11f0b8c99a81a';assert.equal(hex.split(original).length,2);
  const synthetic=runtime.ContractState.deserialize(Buffer.from(hex.replace(original,Buffer.from(commitment).toString('hex')),'hex'));
  try{return await realSdk.createUnprovenCallTxFromInitialStates(zk,{...options,initialContractState:synthetic},enc);}finally{synthetic.free();}
 }}});
 const zswap=new ledger.ZswapChainState(),parameters=ledger.LedgerParameters.initialParameters();
 const native=()=>runtime.ContractState.deserialize(Buffer.from(observed.serializedStateHex,'hex'));
 const rpc=async(m,a)=>m==='chain_getFinalizedHead'?observed.blockHash:m==='chain_getHeader'?{number:'0x'+observed.blockHeight.toString(16)}:m==='chain_getBlockHash'?observed.blockHash:observed.serializedStateHex;
 const receipt=JSON.parse(readFileSync(receiptPath));
 const providers={publicDataProvider:{queryZSwapAndContractState:async()=>{queryCalls++;return[zswap,native(),parameters];}},privateStateProvider:{},zkConfigProvider:new sdkProviders.NodeZkConfigProvider(receipt.assetsPath),walletProvider:{getCoinPublicKey:()=> '00'.repeat(32),getEncryptionPublicKey:()=> '00'.repeat(32)}};
 const connected=await import('./stale-loan-rejection.mjs?synthetic-capability');
 const handle=await connected.prepareStaleLoanAccrue({providers,rpc,receiptPath,borrowerSecret:secret,signingKey:'synthetic-not-read',deadlineMs:Date.now()+89000,nowSeconds:BigInt(Math.floor(Date.now()/1000))});
 assert.equal(handle.status,'PREPARED_STALE_ACCRUE');assert.equal(sdkCalls,1);assert.equal(queryCalls,1);assert.deepEqual(connected.closeStaleLoanPreparation(handle),{loaderClosed:true});assert.throws(()=>connected.closeStaleLoanPreparation(handle),/HANDLE/);
 const historical=readFileSync(new URL('../../../deliverables/sp05-financial-integration-2026-09-09/local-continuation-02/run-public/public-transactions/473b363b745c4be4864e93a04d475a6f343ff7e56a8f179d86060716ba9a5480.bin',import.meta.url));
 // Controlled connected proof/balance boundaries return a retained PUBLIC native
 // transaction. This is not a fresh proof, issued ticket or operational replay.
 for(const mode of ['invalid','unknown','success','retention','outcomeRetention','latePoll','cleanupLate']){
  currentMode=mode;clockOffset=0;let advanced=false,terminalReads=0;const calls=[];const finalized=ledger.Transaction.deserialize('signature','proof','binding',historical);
  const observedRpc=async(m,a)=>{if(advanced&&m==='chain_getFinalizedHead'){terminalReads++;if(mode==='latePoll'&&terminalReads===2)clockOffset+=61000;}const head=advanced&&terminalReads>1?'0x'+'ab'.repeat(32):observed.blockHash;if(m==='chain_getFinalizedHead')return head;if(m==='chain_getHeader')return {number:'0x'+(observed.blockHeight+(advanced&&terminalReads>1?1:0)).toString(16)};if(m==='chain_getBlockHash')return head;return observed.serializedStateHex;};
  providers.proofProvider={proveTx:async tx=>{assert.ok(tx instanceof ledger.Transaction);calls.push('prove');return tx;}};
  providers.walletProvider.balanceTx=async()=>{calls.push('balance');return finalized;};
  providers.midnightProvider={submitTx:async tx=>{assert.equal(tx,finalized);calls.push('submit');advanced=true;if(['invalid','outcomeRetention','latePoll','cleanupLate'].includes(mode))return effect.Effect.runPromise(effect.Effect.fail(new errors.TransactionInvalidError({message:'node invalid',txData:historical})));if(mode==='unknown')throw Error('transport lost');return tx.identifiers()[0];}};
  const prepared=await connected.prepareStaleLoanAccrue({providers,rpc:observedRpc,receiptPath,borrowerSecret:secret,signingKey:'synthetic-not-read',deadlineMs:Date.now()+89000,nowSeconds:BigInt(Math.floor(Date.now()/1000))});
  const operation=connected.submitPreparedStaleLoan(prepared,{retainSubmissionOutcome:async outcome=>({...outcome,status:mode==='outcomeRetention'?'FAILED':'RECORDED'}),retainCandidate:async value=>{calls.push('retain');assert.deepEqual(Buffer.from(value.raw),historical);return {status:'RECORDED',rawSha256:mode==='retention'?'00'.repeat(32):value.transaction.rawSha256};}});
  if(mode==='cleanupLate'){await assert.rejects(operation,e=>e.message==='STALE_LOAN_DEADLINE');clockOffset=0;continue;}
  const out=await operation;
  assert.deepEqual(calls,mode==='retention'?['prove','balance','retain']:['prove','balance','retain','submit']);
  assert.equal(out.status,{invalid:'NODE_REJECTION_FINANCIAL_NONMUTATION',unknown:'OUTCOME_UNKNOWN',success:'UNEXPECTED_SUBMISSION_SUCCESS',retention:'INCOMPLETE',outcomeRetention:'INCOMPLETE',latePoll:'INCOMPLETE'}[mode]);
  assert.equal(out.financialNonmutationEstablished,mode==='invalid');if(mode==='latePoll')assert.equal(out.failure.code,'STALE_LOAN_DEADLINE');clockOffset=0;
  await assert.rejects(connected.submitPreparedStaleLoan(prepared,{retainCandidate:async()=>{}}),/HANDLE/);
 }

});
