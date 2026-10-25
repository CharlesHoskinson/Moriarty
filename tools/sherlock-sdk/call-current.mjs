// Current Sherlock stagenet call. Default is read-only preparation; --submit opens the wallet.
// Usage: node call-current.mjs SDK_ROOT BUILD KEYED_BUILD PARENT KEY OUTPUT acceptValidity|settleValidity [--submit]
// WARNING: the installed SDK JS WASM wellFormed does not verify proofs cryptographically; the final
// balanced transaction is checked by the native proof-verifying validate_call before submission.
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync, mkdirSync, existsSync} from 'node:fs';
import {resolve, join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {createHash} from 'node:crypto';
import {spawn} from 'node:child_process';
import {sdkLoader, endpoints, openStagenetSession} from './stagenet-session.mjs';
const [sdkRoot, build, keyed, parent, key, outputArg, circuit, mode] = process.argv.slice(2);
assert.ok(outputArg && ['acceptValidity','settleValidity'].includes(circuit));
assert.ok(mode === undefined || mode === '--submit');
const output = resolve(outputArg); mkdirSync(output, {recursive: true, mode: 0o700});
assert.ok(!existsSync(join(output, 'submitted.json')), 'submission already attempted from this output directory');
const load = sdkLoader(sdkRoot, join(output, 'sdk-imports'));
const sha = b => createHash('sha256').update(b).digest('hex');
const le = x => Buffer.from(x.toString(16).padStart(64, '0'), 'hex').reverse().toString('hex');
const fromLe = s => BigInt('0x' + Buffer.from(s, 'hex').reverse().toString('hex'));
const pins = JSON.parse(readFileSync('crates/mori-recursion/tests/fixtures/stagenet-verifier.json'));
assert.equal(sha(readFileSync(join(parent, 'w.vk'))), pins.parent_vk_sha256);
assert.equal(sha(readFileSync(join(key, 'decider.vk'))), pins.vk_sha256);
assert.equal(sha(readFileSync(join(key, 'decider.relation'))), pins.relation_sha256);
assert.equal((await (await fetch(endpoints.proof + '/version')).text()).trim(), '9.0.0-rc.5');
const {setNetworkId} = await load('@midnight-ntwrk/midnight-js-network-id'); setNetworkId('stagenet');
const {WebSocket} = await load('ws'); assert.equal(typeof WebSocket, 'function'); globalThis.WebSocket = WebSocket;
const ledger = await load('@midnight-ntwrk/midnight-js-protocol/ledger');
const rt = await load('@midnight-ntwrk/compact-runtime');
const {CompiledContract} = await load('@midnight-ntwrk/midnight-js-protocol/compact-js');
const {createVerifierKey, SucceedEntirely} = await load('@midnight-ntwrk/midnight-js-types');
const {parseCoinPublicKeyToHex} = await load('@midnight-ntwrk/midnight-js-utils');
const {createUnprovenCallTx, submitTx} = await load('@midnight-ntwrk/midnight-js-contracts');
const {indexerPublicDataProvider} = await load('@midnight-ntwrk/midnight-js-indexer-public-data-provider');
const {NodeZkConfigProvider} = await load('@midnight-ntwrk/midnight-js-node-zk-config-provider');
const {httpClientProvingProvider} = await load('@midnight-ntwrk/midnight-js-http-client-proof-provider');
const mod = await import(pathToFileURL(resolve(build, 'contract/index.js')).href);
const publicDataProvider = indexerPublicDataProvider(endpoints.indexer, endpoints.indexerWs);
class Keys extends NodeZkConfigProvider {
  async getVerifierKey(name) {
    if (name !== 'acceptValidity') return super.getVerifierKey(name);
    return createVerifierKey(readFileSync(join(key, 'acceptValidity.verifier')));
  }
}
const keys = new Keys(resolve(keyed));
const compiledContract = CompiledContract.make('SherlockAnchor', mod.Contract).pipe(
  CompiledContract.withVacantWitnesses, CompiledContract.withCompiledFileAssets(resolve(build)));
const fields = JSON.parse(readFileSync(join(parent, 'w.instance.json'))).slice(0,16).map(fromLe);
const stmt = fromLe(JSON.parse(readFileSync(join(key, '../terminal.public.json')))[10]);
const id = fromLe('489080d3e57cacab615cd0cde4c87914b47c4ff0ab5857d865ee2474ecd45c00');
const args = circuit === 'acceptValidity' ? [stmt] : [0n, fields];
const json = x => JSON.stringify(x, (_, v) => typeof v === 'bigint' ? v.toString() : v, 2) + '\n';
const graphql = async query => {
  const r = await fetch(endpoints.indexer, {method:'POST', headers:{'content-type':'application/json'}, body:JSON.stringify({query})});
  assert.ok(r.ok); const body=await r.json(); assert.ok(!body.errors, 'indexer query failed'); return body.data;
};
const child = (command, argv, input, env, log) => new Promise((ok, reject) => {
  const p=spawn(command,argv,{env,stdio:['pipe','pipe','pipe']}); let stdout='',stderr='';
  p.stdout.on('data',b=>{stdout+=b;});p.stderr.on('data',b=>{stderr+=b;});p.on('error',reject);
  p.on('close',exit=>{writeFileSync(join(output,log),stderr); if(exit===0)ok(stdout);else reject(Error('child failed: '+log));});
  p.stdin.end(input);
});
let session, currentState, parameters, expectedState, nativeProofPublic, nativeProofHash, proofCalls=0;
const prepared = new Error('PREPARATION_ONLY'); let preparedComplete=false;
const options = {compiledContract, contractAddress:pins.address, circuitId:circuit, args};
const baseProver = httpClientProvingProvider(endpoints.proof,keys,{timeout:15*60_000});
const proveProvider = {
  check:(raw,location)=>baseProver.check(raw,location),
  lookupKey:location=>baseProver.lookupKey(location),
  prove:async(raw,location,binding)=>{
    assert.equal(++proofCalls,1,'one contract proof per transaction');
    assert.equal(location,`contract:${pins.address}/${circuit}?vk=${sha(currentState.operation(circuit).verifierKey)}`);
    if(circuit==='settleValidity') {
      if(!mode){preparedComplete=true;throw prepared;}
      return baseProver.prove(raw,location,binding);
    }
    assert.equal(typeof binding,'bigint');
    const test = mode ? 'terminal_proof_matches_the_installed_stagenet_verifier' : 'sdk_call_vector_matches_the_decider_template';
    const env={...process.env,CARGO_BUILD_JOBS:'4',RAYON_NUM_THREADS:'4',
      MORIARTY_A3_PARENT_DIR:resolve(parent),MORIARTY_TERMINAL_CALL_STDIN:'1',
      MORIARTY_DEPLOYED_CONTRACT_STATE:join(output,'pre-state.bin'),
      MORIARTY_PARENT_SRS:resolve('target/sherlock-srs/bls_midnight_2p20'),
      MORIARTY_TERMINAL_KEY_DIR:resolve(key),MORIARTY_TERMINAL_OUTPUT_DIR:join(output,'proof')};
    const log=await child('cargo',['test','--release','--locked','-p','mori-recursion','--features','sherlock-ledger',
      '--test','sherlock_terminal',test,'--','--exact','--include-ignored','--nocapture','--test-threads=1'],
      JSON.stringify({binding_input:le(binding),preimage:Buffer.from(raw).toString('hex')}),env,'native-prover-stderr.log');
    writeFileSync(join(output,'native-prover-stdout.log'),log);
    if(!mode){preparedComplete=true;throw prepared;}
    nativeProofPublic=JSON.parse(readFileSync(join(output,'proof/terminal.public.json')));
    nativeProofHash=sha(readFileSync(join(output,'proof/terminal.proof')));
    assert.equal(nativeProofPublic[0],le(binding)); assert.equal(nativeProofPublic[10],le(stmt));
    return readFileSync(join(output,'proof/terminal.tagged-proof'));
  },
};
// Native proof-verifying validation over an isolated ledger holding the indexed deployment and the predicted snapshot.
// Bootstrap from the actual public deployment action; never persist or log transaction bodies.
let prebalance;
const nativeValidate=async(tx,log)=>{
  const deployed=JSON.parse(readFileSync('target/a4-anchor/deploy-run3/receipt.json'));
  assert.equal(deployed.address,pins.address);
  const data=await graphql(`{transactions(offset:{identifier:"${deployed.txId}"}){raw}}`);
  const deploymentTx=ledger.Transaction.deserialize('signature','proof','binding',Buffer.from(data.transactions[0].raw,'hex'));
  const deployments=[...deploymentTx.intents.values()].flatMap(i=>i.actions).filter(a=>a instanceof ledger.ContractDeploy);
  assert.equal(deployments.length,1); assert.equal(deployments[0].address,pins.address);
  let local=ledger.LedgerState.blank('stagenet'); local.parameters=parameters;
  const setup=ledger.Transaction.fromParts('stagenet',undefined,undefined,ledger.Intent.new(new Date(Date.now()+3600_000)).addDeploy(deployments[0])).eraseProofs();
  const lax=Object.assign(new ledger.WellFormedStrictness(),{enforceBalancing:false,verifyNativeProofs:false,verifyContractProofs:false,enforceLimits:false,verifySignatures:false});
  const secs=BigInt(Math.floor(Date.now()/1000)),block={secondsSinceEpoch:secs,secondsSinceEpochErr:30,parentBlockHash:'00'.repeat(32),lastBlockTime:secs-6n};
  const [boot,result]=local.apply(setup.wellFormed(local,lax,new Date()),new ledger.TransactionContext(local,block));assert.equal(result.type,'success');
  const ledgerState=ledger.ContractState.deserialize(currentState.serialize()); // compact-runtime -> ledger WASM class
  local=boot.updateIndex(pins.address,ledgerState.data,ledgerState.balance);
  return JSON.parse(await child(resolve('target/release/examples/validate_call'),[],
    JSON.stringify({state:Buffer.from(local.serialize()).toString('hex'),transaction:Buffer.from(tx.serialize()).toString('hex'),block_seconds:Number(secs)}),process.env,log+'-stderr.log'));
};
const checkCall=report=>{
  assert.equal(report.proofVerification,'native/proof-verifying');
  assert.equal(report.contractCalls.length,1);const call=report.contractCalls[0];
  assert.equal(call.address,pins.address);assert.equal(call.entryPoint,circuit);assert.equal(call.proofVersion,'V3');
  if(circuit==='acceptValidity'){assert.equal(call.proofSHA256,nativeProofHash);assert.deepEqual(call.publicInputs,nativeProofPublic);}
};
const proofProvider={proveTx:async tx=>{
  const proven=await tx.prove(proveProvider,parameters.transactionCostModel.runtimeCostModel);
  if(mode){ // strict pre-balance check: contract proof, signatures and limits on; must apply
    prebalance=await nativeValidate(ledger.Transaction.deserialize('signature','proof','pre-binding',proven.serialize()).bind(),'native-prebalance');
    assert.equal(prebalance.contractProofsSignaturesLimitsAccepted,true,prebalance.contractProfileFailure);
    assert.equal(prebalance.applied,true,'pre-balance native execution'); checkCall(prebalance);
    writeFileSync(join(output,'native-prebalance-check.json'),json(prebalance));
  }
  return proven;
}};
try {
  if(mode)session=await openStagenetSession(load,output);
  const walletProvider=session?.walletProvider ?? {getCoinPublicKey:()=> '00'.repeat(32),getEncryptionPublicKey:()=> '00'.repeat(32)};
  const source=await publicDataProvider.queryZSwapAndContractState(pins.address); assert.ok(source);
  [,currentState,parameters]=source;
  const before=mod.ledger(currentState.data);
  for(const [name,value] of Object.entries({instance_id:id,keys_id:fields[0],a102_ctx_lo:fields[1],a102_ctx_hi:fields[2],
    a80_ctx_lo:fields[3],a80_ctx_hi:fields[4],program_c:fields[5],root:fields[6],height:fields[7],batch_no:0n}))assert.equal(before[name],value,name);
  assert.ok(!before.settled.member(stmt));
  assert.equal(before.validity_ok.member(stmt),circuit==='settleValidity');
  for(const name of ['acceptValidity','settleValidity'])assert.equal(sha(await keys.getVerifierKey(name)),sha(currentState.operation(name).verifierKey));
  writeFileSync(join(output,'pre-state.bin'),currentState.serialize());
  // Independent deterministic VM prediction of the complete contract state, including keys/CMA/balance.
  const clone=rt.ContractState.deserialize(currentState.serialize());
  const context=rt.createCircuitContext(circuit,pins.address,parseCoinPublicKeyToHex(walletProvider.getCoinPublicKey(),'stagenet'),clone,{});
  const predicted=await new mod.Contract({}).provableCircuits[circuit](context,...args);
  clone.data=predicted.context.callContext.currentQueryContext.state;
  expectedState=Buffer.from(clone.serialize());
  const providers={publicDataProvider,zkConfigProvider:keys,proofProvider,walletProvider,
    ...(session?{privateStateProvider:session.privateStateProvider}:{}),midnightProvider:{submitTx:async tx=>{
      // Final balanced transaction. A refusal that is about proofs or signatures, or a non-applied call,
      // blocks submission; a refusal caused only by wallet DUST/fee state absent from the blank local ledger
      // is retained as evidence, because the pre-balance call already passed the strict native check.
      const report=await nativeValidate(tx,'native-final');
      if(!report.contractProofsSignaturesLimitsAccepted){
        assert.doesNotMatch(report.contractProfileFailure,/proof|signature/i,report.contractProfileFailure);
        assert.ok(prebalance?.applied,'pre-balance native check must have applied');
      } else assert.equal(report.applied,true,'native execution against the predicted snapshot');
      checkCall(report);
      writeFileSync(join(output,'native-final-check.json'),json(report));
      // The SDK builder fetched its own snapshot; refuse to submit unless the chain still holds the predicted pre-state.
      const latest=await publicDataProvider.queryContractState(pins.address); assert.ok(latest);
      assert.equal(sha(latest.serialize()),sha(currentState.serialize()),'contract state changed since prediction');
      writeFileSync(join(output,'submitted.json'),json({at:new Date().toISOString(),network:'stagenet',address:pins.address,circuit}),{flag:'wx',mode:0o600});
      return session.walletProvider.submitTx(tx);
    }}};
  if(!mode){
    try {const unproven=await createUnprovenCallTx(providers,options);await proofProvider.proveTx(unproven.private.unprovenTx);}
    catch(error){if(!preparedComplete)throw error;}
    assert.ok(preparedComplete);
    writeFileSync(join(output,'preparation.json'),json({address:pins.address,circuit,stmt:le(stmt),
      preStateSHA256:sha(currentState.serialize()),expectedPostStateSHA256:sha(expectedState),nativeVectorChecked:circuit==='acceptValidity',submitted:false}));
    console.log('PASS current stagenet call preparation; no wallet, proof generation or submission');
  } else {
    const unproven=await createUnprovenCallTx(providers,options);
    const result=await submitTx(providers,{unprovenTx:unproven.private.unprovenTx});assert.equal(result.status,SucceedEntirely);
    const receipt={schema:'sherlock-stagenet-call/1',address:pins.address,circuit,txId:result.txId,txHash:result.txHash,
      blockHeight:result.blockHeight,blockHash:result.blockHash,status:result.status,preStateSHA256:sha(currentState.serialize()),
      expectedPostStateSHA256:sha(expectedState),stmt:le(stmt)};
    writeFileSync(join(output,'receipt.json'),json(receipt));
    const indexed=await graphql(`{contractAction(address:"${pins.address}",offset:{transactionOffset:{hash:"${result.txHash}"}}){__typename state transaction{hash block{height hash}} ... on ContractCall{entryPoint}}}`);
    assert.equal(indexed.contractAction.__typename,'ContractCall');assert.equal(indexed.contractAction.transaction.hash,result.txHash);
    const post=Buffer.from(rt.ContractState.deserialize(Buffer.from(indexed.contractAction.state,'hex')).serialize());assert.deepEqual(post,expectedState,'complete contract effect readback');
    writeFileSync(join(output,'post-state.bin'),post);writeFileSync(join(output,'indexed-call.json'),json(indexed));
    console.log(json({...receipt,completeEffectsMatch:true}));
  }
} finally {if(session)await session.close();}
