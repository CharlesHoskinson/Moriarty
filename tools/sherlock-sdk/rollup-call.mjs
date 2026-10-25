// Synthetic Sherlock rollup stagenet call. Default is read-only preparation; --submit opens the wallet.
// Usage: node rollup-call.mjs SDK_ROOT BUILD KEYED_BUILD B3_DIR KEY DEPLOY_RECEIPT OUTPUT acceptValidity|settleValidity [--submit]
// WARNING: the installed SDK JS WASM wellFormed does not verify proofs cryptographically; the final
// balanced transaction is checked by the native proof-verifying validate_call before submission.
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync, mkdirSync, existsSync} from 'node:fs';
import {resolve, join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {createHash} from 'node:crypto';
import {spawn} from 'node:child_process';
import {sdkLoader, endpoints, openStagenetSession} from './stagenet-session.mjs';
import {anchorView, bindBatch, assertPredecessor, assertSettled} from './rollup-statement.mjs';
import {pinnedRollupKeys} from './rollup-keys.mjs';
import {checkCompiledCall} from './rollup-compiled-check.mjs';
import {ownedWalletProver} from './owned-wallet-prover.mjs';
import {recordSubmissionRefusal} from './rollup-refusal.mjs';
const [sdkRoot, build, keyed, parent, key, deploymentReceipt, outputArg, circuit, mode] = process.argv.slice(2);
assert.ok(outputArg && ['acceptValidity','settleValidity'].includes(circuit));
assert.ok(mode === undefined || mode === '--submit');
const output = resolve(outputArg); mkdirSync(output, {recursive: true, mode: 0o700});
assert.ok(!existsSync(join(output, 'submitted.json')), 'submission already attempted from this output directory');
const load = sdkLoader(sdkRoot, join(output, 'sdk-imports'));
const sha = b => createHash('sha256').update(b).digest('hex');
const le = x => Buffer.from(x.toString(16).padStart(64, '0'), 'hex').reverse().toString('hex');
const fromLe = s => BigInt('0x' + Buffer.from(s, 'hex').reverse().toString('hex'));
const pins = JSON.parse(readFileSync(deploymentReceipt));
assert.equal(pins.schema,'sherlock-rollup-stagenet-deployment/1');
assert.equal(pins.network,'stagenet'); assert.match(pins.address,/^[0-9a-f]{64}$/);
assert.equal(pins.sourceSHA256,sha(readFileSync(new URL('./rollup-anchor.compact',import.meta.url))));
const manifest=JSON.parse(readFileSync(join(key,'manifest.json')));
assert.equal(manifest.schema,'sherlock-b4-preparation/1');
assert.equal(sha(readFileSync(join(parent,'parent.vk'))),manifest.parent_vk_sha256);
assert.equal(sha(readFileSync(join(key,'decider.vk'))),manifest.vk_sha256);
assert.equal(sha(readFileSync(join(key,'decider.relation'))),manifest.relation_sha256);
assert.equal(sha(readFileSync(join(key,'acceptValidity.verifier'))),manifest.tagged_vk_sha256);
assert.equal(pins.keys.acceptValidity.sha256,manifest.tagged_vk_sha256);
const {setNetworkId} = await load('@midnight-ntwrk/midnight-js-network-id'); setNetworkId('stagenet');
const {WebSocket} = await load('ws'); assert.equal(typeof WebSocket, 'function'); globalThis.WebSocket = WebSocket;
const ledger = await load('@midnight-ntwrk/midnight-js-protocol/ledger');
const rt = await load('@midnight-ntwrk/compact-runtime');
const {CompiledContract} = await load('@midnight-ntwrk/midnight-js-protocol/compact-js');
const types = await load('@midnight-ntwrk/midnight-js-types');
const {SucceedEntirely} = types;
const {parseCoinPublicKeyToHex, ZkArtifactIntegrityError} = await load('@midnight-ntwrk/midnight-js-utils');
const {createUnprovenCallTx, submitTx} = await load('@midnight-ntwrk/midnight-js-contracts');
const {indexerPublicDataProvider} = await load('@midnight-ntwrk/midnight-js-indexer-public-data-provider');
const mod = await import(pathToFileURL(resolve(build, 'contract/index.js')).href);
const publicDataProvider = indexerPublicDataProvider(endpoints.indexer, endpoints.indexerWs);
for (const name of ['acceptValidity', 'settleValidity']) {
  assert.equal(resolve(pins.provingArtifacts[name].prover.path), resolve(keyed, `keys/${name}.prover`));
  assert.equal(resolve(pins.provingArtifacts[name].ir.path), resolve(keyed, `zkir/${name}.bzkir`));
}
const keys = pinnedRollupKeys(types, ZkArtifactIntegrityError, pins);
const compiledContract = CompiledContract.make('SherlockRollupAnchor', mod.Contract).pipe(
  CompiledContract.withVacantWitnesses, CompiledContract.withCompiledFileAssets(resolve(build)));
const bound=bindBatch(JSON.parse(readFileSync(join(parent,'leaf.instance.json'))),
  JSON.parse(readFileSync(join(parent,'parent.instance.json'))),manifest,rt);
const {fields,id,batch,statement:stmt}=bound;
const args=circuit==='acceptValidity'?[stmt]:[batch,fields];
const json = x => JSON.stringify(x, (_, v) => typeof v === 'bigint' ? v.toString() : v, 2) + '\n';
const graphql = async query => {
  const r = await fetch(endpoints.indexer, {method:'POST', headers:{'content-type':'application/json'}, body:JSON.stringify({query})});
  assert.ok(r.ok); const body=await r.json(); assert.ok(!body.errors, 'indexer query failed'); return body.data;
};
const child = (command, argv, input, env, log) => new Promise((ok, reject) => {
  const p=spawn(command,argv,{env,stdio:['pipe','pipe','pipe']}); let stdout='',stderr='';
  p.stdout.on('data',b=>{stdout+=b;});p.stderr.on('data',b=>{stderr+=b;});p.on('error',reject);
  p.stdin.on('error',error=>{if(error.code!=='EPIPE')reject(Error('child input transport failed'));});
  p.on('close',exit=>{writeFileSync(join(output,log),stderr); if(exit===0)ok(stdout);else reject(Error('child failed: '+log));});
  p.stdin.end(input);
});
let session, currentState, parameters, expectedState, nativeProofPublic, nativeProofHash, proofCalls=0;
let submissionStarted, submissionAcknowledged, finalizationObserved, provenFeesSpecks;
const prepared = new Error('PREPARATION_ONLY'); let preparedComplete=false;
const options = {compiledContract, contractAddress:pins.address, circuitId:circuit, args};
const registry = new types.ZKConfigRegistry([keys]);
const proveProvider = {
  check:async(raw,location)=>{
    assert.ok(await registry.resolveKeyLocation(location));
    return checkCompiledCall(raw,location,{ir:pins.provingArtifacts[circuit].ir.path,
      prover:pins.provingArtifacts[circuit].prover.path,verifier:pins.keys[circuit].path,output});
  },
  lookupKey:async location=>{
    const material=await registry.resolveKeyLocation(location);assert.ok(material);
    return {ir:material.zkir,proverKey:material.proverKey,verifierKey:material.verifierKey};
  },
  prove:async(raw,location,binding)=>{
    assert.equal(++proofCalls,1,'one contract proof per transaction');
    assert.equal(location,`contract:${pins.address}/${circuit}?vk=${sha(currentState.operation(circuit).verifierKey)}`);
    if(circuit==='settleValidity') {
      if(!mode){preparedComplete=true;throw prepared;}
      const proof=join(output,'settle.tagged-proof');
      const log=await child(resolve('scripts/doctorwatson'),['heavy','--id','B4-settle-proof',
        '--budget','b4-compiled-settle','--hypothesis','Generate only the compiled settlement proof under heavy; wallet sync and finalization stay outside',
        '--pipe-stdin','--',process.execPath,resolve('tools/sherlock-sdk/rollup-proof-compiled.mjs'),
        resolve(sdkRoot),resolve(deploymentReceipt),location,proof],
        JSON.stringify({binding:String(binding),preimage:Buffer.from(raw).toString('hex')}),
        {...process.env,RAYON_NUM_THREADS:'4'},'settle-prover-stderr.log');
      writeFileSync(join(output,'settle-prover-stdout.log'),log);
      return readFileSync(proof);
    }
    assert.equal(typeof binding,'bigint');
    const measured=['-B','scripts/qa/measure-rollup.py',join(output,'native-proof.measurement.json'),'--',
      resolve(process.env.ROLLUP_CALL_BINARY ?? 'target/release/examples/rollup_call'),
       mode?'prove':'check',resolve(parent),resolve('crates/mori-recursion/tests/fixtures/rollup-template.json'),
       resolve(key),join(output,'proof'),manifest.instance_id,resolve(process.env.MORIARTY_PARENT_SRS ?? 'target/sherlock-srs/bls_midnight_2p20')];
    const command=mode?resolve('scripts/doctorwatson'):'python3';
    const argv=mode?['heavy','--id','B4-bound-decider','--budget','b4-bound-decider',
      '--hypothesis','Generate only the SDK-bound stock D under heavy; wallet sync and finalization stay outside',
      '--pipe-stdin','--','python3',...measured]:measured;
    const log=await child(command,argv,
      JSON.stringify({binding_input:le(binding),preimage:Buffer.from(raw).toString('hex')}),
      {...process.env,RAYON_NUM_THREADS:'4'},'native-prover-stderr.log');
    writeFileSync(join(output,'native-prover-stdout.log'),log);
    if(!mode){preparedComplete=true;throw prepared;}
    nativeProofPublic=JSON.parse(readFileSync(join(output,'proof/terminal.public.json')));
    nativeProofHash=sha(readFileSync(join(output,'proof/terminal.proof')));
    assert.equal(nativeProofPublic[0],le(binding)); assert.equal(nativeProofPublic[11],le(stmt));assert.equal(nativeProofPublic[25],le(stmt));
    return readFileSync(join(output,'proof/terminal.tagged-proof'));
  },
};
const proofProvider={proveTx:tx=>tx.prove(proveProvider,parameters.transactionCostModel.runtimeCostModel)};
try {
  if(mode){
    const guard=await ownedWalletProver({binary:process.env.MORIARTY_WALLET_PROOF_SERVER,
      parameters:process.env.MORIARTY_WALLET_PROOF_PARAMS,output,
      sha256:process.env.MORIARTY_WALLET_PROOF_SERVER_SHA256,purpose:circuit});
    session=await openStagenetSession(load,output,{proofUrl:guard.url,finalize:operation=>guard.run(operation)});
  }
  const walletProvider=session?.walletProvider ?? {getCoinPublicKey:()=> '00'.repeat(32),getEncryptionPublicKey:()=> '00'.repeat(32)};
  const source=await publicDataProvider.queryZSwapAndContractState(pins.address); assert.ok(source);
  [,currentState,parameters]=source;
  const before=anchorView(rt,mod,currentState);
  assertPredecessor(before,bound);
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
      // Bootstrap from the actual public deployment action; never persist or log transaction bodies.
      const deployed=pins;
      assert.equal(deployed.address,pins.address);
      assert.match(deployed.txHash,/^[0-9a-f]{64}$/);
      const data=await graphql(`{transactions(offset:{hash:"${deployed.txHash}"}){hash raw}}`);
      const indexedDeployment=data.transactions.find(tx=>tx.hash===deployed.txHash);assert.ok(indexedDeployment);
      const deploymentTx=ledger.Transaction.deserialize('signature','proof','binding',Buffer.from(indexedDeployment.raw,'hex'));
      const deployments=[...deploymentTx.intents.values()].flatMap(i=>i.actions).filter(a=>a instanceof ledger.ContractDeploy);
      assert.equal(deployments.length,1); assert.equal(deployments[0].address,pins.address);
      let local=ledger.LedgerState.blank('stagenet'); local.parameters=parameters;
      const setup=ledger.Transaction.fromParts('stagenet',undefined,undefined,ledger.Intent.new(new Date(Date.now()+3600_000)).addDeploy(deployments[0])).eraseProofs();
      const lax=Object.assign(new ledger.WellFormedStrictness(),{enforceBalancing:false,verifyNativeProofs:false,verifyContractProofs:false,enforceLimits:false,verifySignatures:false});
      const secs=BigInt(Math.floor(Date.now()/1000)),block={secondsSinceEpoch:secs,secondsSinceEpochErr:30,parentBlockHash:'00'.repeat(32),lastBlockTime:secs-6n};
      const [boot,result]=local.apply(setup.wellFormed(local,lax,new Date()),new ledger.TransactionContext(local,block));assert.equal(result.type,'success');
      const nativeState=ledger.ContractState.deserialize(currentState.serialize());
      local=boot.updateIndex(pins.address,nativeState.data,nativeState.balance);
      const report=JSON.parse(await child(resolve(process.env.SHERLOCK_NATIVE_VALIDATOR ?? 'target/release/examples/validate_call'),[],
        JSON.stringify({state:Buffer.from(local.serialize()).toString('hex'),transaction:Buffer.from(tx.serialize()).toString('hex'),block_seconds:Number(secs)}),process.env,'native-final-stderr.log'));
      assert.equal(report.contractProofsSignaturesLimitsAccepted,true,report.contractProfileFailure);
      assert.equal(report.applied,true,'native execution against the predicted snapshot');
      assert.equal(report.contractCalls.length,1);const call=report.contractCalls[0];
      assert.equal(call.address,pins.address);assert.equal(call.entryPoint,circuit);assert.equal(call.proofVersion,'V3');
      if(circuit==='acceptValidity'){assert.equal(call.proofSHA256,nativeProofHash);assert.deepEqual(call.publicInputs,nativeProofPublic);}
      writeFileSync(join(output,'native-final-check.json'),json(report));
      const latest=await publicDataProvider.queryContractState(pins.address);assert.ok(latest);
      assert.equal(sha(latest.serialize()),sha(currentState.serialize()),'contract state changed since prediction');
      provenFeesSpecks=String(tx.fees(parameters));
      writeFileSync(join(output,'submitted.json'),json({at:new Date().toISOString(),network:'stagenet',address:pins.address,circuit,
        transactionSHA256:sha(tx.serialize())}),{flag:'wx',mode:0o600});
      submissionStarted=performance.now();
      try {
        const submitted=await session.walletProvider.submitTx(tx);
        submissionAcknowledged=performance.now();
        return submitted;
      } catch(error) {
        recordSubmissionRefusal(error,output,circuit);
        throw Error('stagenet call submission failed; private details suppressed');
      }
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
    finalizationObserved=performance.now();
    const receipt={schema:'sherlock-rollup-stagenet-call/1',address:pins.address,circuit,txId:result.txId,txHash:result.txHash,
      blockHeight:result.blockHeight,blockHash:result.blockHash,status:result.status,preStateSHA256:sha(currentState.serialize()),
      expectedPostStateSHA256:sha(expectedState),stmt:le(stmt),network:'stagenet',batch:String(batch),keys:pins.keys,
      blockTimestamp:result.blockTimestamp,fees:result.fees,provenFeesSpecks,
      timing:{submissionAckSeconds:(submissionAcknowledged-submissionStarted)/1000,
        finalizationObservedSeconds:(finalizationObserved-submissionStarted)/1000,
        finalizationScope:'host time from RPC submission start through SDK finalized transaction observation',
        validatorCryptographicVerificationSeconds:null,validatorTimingEvidence:'unavailable'}};
    writeFileSync(join(output,'receipt.json'),json(receipt));
    const readbackStarted=performance.now();
    const indexed=await graphql(`{contractAction(address:"${pins.address}",offset:{transactionOffset:{hash:"${result.txHash}"}}){__typename state transaction{hash block{height hash}} ... on ContractCall{entryPoint}}}`);
    assert.equal(indexed.contractAction.__typename,'ContractCall');assert.equal(indexed.contractAction.transaction.hash,result.txHash);
    const post=Buffer.from(indexed.contractAction.state,'hex');assert.deepEqual(post,expectedState,'complete contract effect readback');
    const after=anchorView(rt,mod,rt.ContractState.deserialize(post));
    if(circuit==='settleValidity')assertSettled(after,bound);
    else {assertPredecessor(after,bound);assert.ok(after.validity_ok.member(stmt));assert.ok(!after.settled.member(stmt));}
    writeFileSync(join(output,'post-state.bin'),post);writeFileSync(join(output,'indexed-call.json'),json(indexed));
    receipt.timing.readbackSeconds=(performance.now()-readbackStarted)/1000;
    receipt.timing.submissionToReadbackSeconds=(performance.now()-submissionStarted)/1000;
    receipt.completeEffectsMatch=true;receipt.postStateSHA256=sha(post);
    receipt.localCryptographicVerification=circuit==='acceptValidity'
      ? JSON.parse(readFileSync(join(output,'proof/proof.json'))).local_verify_seconds : null;
    receipt.localVerificationScope='isolated stock host proof verification; not validator execution time';
    writeFileSync(join(output,'receipt.json'),json(receipt));
    console.log(json(receipt));
  }
} catch {throw Error('rollup call failed; private wallet/provider details suppressed');}
finally {if(session)try{await session.close();}catch{throw Error('wallet shutdown failed; private details suppressed');}}
