// SherlockRun.acceptRun on stagenet (gap G11, call half). Default: full local preparation with no wallet:
// the run proof is made and checked by native proof-verifying ledger 9 against the indexed deployment, and
// nothing is submitted. --submit opens the wallet (hold ~/.cache/doctorwatson/wallet.lock around it).
// Usage: node call-run.mjs SDK_ROOT BUILD DEPLOYMENT_RECEIPT GP_BUNDLE OUTPUT [--submit]
//   BUILD               compiled tools/sherlock-sdk/sherlock-run.compact (its prover key/IR serve /check only)
//   DEPLOYMENT_RECEIPT  receipt.json of deploy-run.mjs (address, txId, verifier path/sha, constructor digests)
//   GP_BUNDLE           gp_fold_and_key output: bundle.json {stmt, prog_d, init_d, fin_d, steps, result, ...} and
//                       key/acceptRun.verifier, which must be the deployment's pinned key
//   SHERLOCK_RUN_REFERENCE  the `mori simulate` result of the program; required unless the bundle is a stand-in
//   SHERLOCK_RUN_PROVER command: reads {"binding_input": hex32le, "preimage": hex} on stdin, writes the tagged
//                       acceptRun proof as hex on stdout (tools/sherlock-sdk/gp-call-prover.sh). The preimage holds
//                       the private communication opening; it is only piped, never written.
//   SHERLOCK_VALIDATE_CALL  native validator (default target/release/examples/validate_call)
// Before any wallet access: the deployment's installed acceptRun key must equal the pinned run-decider key
// (read from the indexer), its sealed digests must equal RUN_JSON, and the proven call must be accepted and
// applied by the native validator. After submission the complete post state is read back from the indexer
// and must equal the deterministic prediction.
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync, mkdirSync, existsSync} from 'node:fs';
import {resolve, join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {createHash} from 'node:crypto';
import {spawn} from 'node:child_process';
import {sdkLoader, endpoints, openStagenetSession} from './stagenet-session.mjs';

const argv = process.argv.slice(2);
const submit = argv.includes('--submit');
const pos = argv.filter(a => a !== '--submit');
assert.equal(pos.length, 5, 'usage: call-run.mjs SDK_ROOT BUILD DEPLOYMENT_RECEIPT GP_BUNDLE OUTPUT [--submit]');
const [sdkRoot, buildArg, receiptPath, bundleArg, outputArg] = pos;
const proverCmd = process.env.SHERLOCK_RUN_PROVER;
assert.ok(proverCmd, 'SHERLOCK_RUN_PROVER');
const validateCall = resolve(process.env.SHERLOCK_VALIDATE_CALL ?? 'target/release/examples/validate_call');
const output = resolve(outputArg), build = resolve(buildArg);
mkdirSync(output, {recursive: true, mode: 0o700});
assert.ok(!existsSync(join(output, 'submitted.json')), 'submission already attempted from this output directory');
const sha = b => createHash('sha256').update(b).digest('hex');
const le = x => Buffer.from(x.toString(16).padStart(64, '0'), 'hex').reverse().toString('hex');
const fromLe = h => { assert.match(h, /^[0-9a-f]{64}$/); return BigInt('0x' + Buffer.from(h, 'hex').reverse().toString('hex')); };
const be16 = v => new Uint8Array(Buffer.from(v.toString(16).padStart(32, '0'), 'hex'));
const json = x => JSON.stringify(x, (_, v) => typeof v === 'bigint' ? v.toString() : v, 2) + '\n';

const deployment = JSON.parse(readFileSync(receiptPath));
assert.equal(deployment.schema, 'sherlock-run-stagenet-deployment/1');
assert.equal(deployment.sourceSHA256, sha(readFileSync(new URL('./sherlock-run.compact', import.meta.url))), 'contract source');
const target = deployment.address; assert.match(target, /^[0-9a-f]{64}$/);
const vkRaw = readFileSync(deployment.verifier.path);
assert.equal(sha(vkRaw), deployment.verifier.sha256, 'pinned run-decider key');
const bundleDir = resolve(bundleArg);
const runJson = JSON.parse(readFileSync(join(bundleDir, 'bundle.json')));
assert.equal(runJson.schema, 'sherlock-gp-bundle/1');
assert.equal(sha(readFileSync(join(bundleDir, 'key/acceptRun.verifier'))), deployment.verifier.sha256, 'bundle key is the deployed key');
assert.equal(runJson.acceptRun_verifier_sha256, deployment.verifier.sha256);
const run = {stmt: fromLe(runJson.stmt), p: fromLe(runJson.prog_d), i: fromLe(runJson.init_d),
  fin: fromLe(runJson.fin_d), n: BigInt(runJson.steps), r: BigInt(runJson.result)};
assert.ok(run.r >= 0n && run.r < 1n << 128n && run.n >= 1n && run.n < 1n << 64n);
assert.equal(runJson.result_be16_hex, Buffer.from(be16(run.r)).toString('hex'));
const reference = process.env.SHERLOCK_RUN_REFERENCE;
assert.ok(reference !== undefined || runJson.standin === true, 'SHERLOCK_RUN_REFERENCE (mori simulate result) is required');
if (reference !== undefined) assert.equal(BigInt(reference), run.r, 'decider result equals the mori simulate reference');
assert.deepEqual(deployment.constructor, [runJson.prog_d, runJson.init_d], 'deployment digests equal the run');
assert.equal((await (await fetch(endpoints.proof + '/version')).text()).trim(), '9.0.0-rc.5');

const load = sdkLoader(sdkRoot, join(output, 'sdk-imports'));
const {setNetworkId} = await load('@midnight-ntwrk/midnight-js-network-id'); setNetworkId('stagenet');
const {WebSocket} = await load('ws'); globalThis.WebSocket = WebSocket;
const ledger = await load('@midnight-ntwrk/midnight-js-protocol/ledger');
const rt = await load('@midnight-ntwrk/compact-runtime');
const {CompiledContract} = await load('@midnight-ntwrk/midnight-js-protocol/compact-js');
const {createVerifierKey, SucceedEntirely} = await load('@midnight-ntwrk/midnight-js-types');
const {parseCoinPublicKeyToHex} = await load('@midnight-ntwrk/midnight-js-utils');
const {createUnprovenCallTx, submitTx} = await load('@midnight-ntwrk/midnight-js-contracts');
const {indexerPublicDataProvider} = await load('@midnight-ntwrk/midnight-js-indexer-public-data-provider');
const {NodeZkConfigProvider} = await load('@midnight-ntwrk/midnight-js-node-zk-config-provider');
const {httpClientProvingProvider} = await load('@midnight-ntwrk/midnight-js-http-client-proof-provider');
const mod = await import(pathToFileURL(join(build, 'contract/index.js')).href);
const publicDataProvider = indexerPublicDataProvider(endpoints.indexer, endpoints.indexerWs);
class Keys extends NodeZkConfigProvider {
  async getVerifierKey(name) { assert.equal(name, 'acceptRun'); return createVerifierKey(vkRaw); }
}
const keys = new Keys(build);
const compiledContract = CompiledContract.make('SherlockRun', mod.Contract).pipe(
  CompiledContract.withVacantWitnesses, CompiledContract.withCompiledFileAssets(build));
const args = [run.stmt, run.p, run.i, run.n, run.fin, be16(run.r)];
const graphql = async query => {
  const r = await fetch(endpoints.indexer, {method: 'POST', headers: {'content-type': 'application/json'},
    body: JSON.stringify({query})});
  assert.ok(r.ok); const body = await r.json(); assert.ok(!body.errors, 'indexer query failed'); return body.data;
};
const child = (command, cargs, input, log, shell = false) => new Promise((ok, reject) => {
  const p = shell ? spawn('/bin/sh', ['-c', command], {stdio: ['pipe', 'pipe', 'pipe']})
    : spawn(command, cargs, {stdio: ['pipe', 'pipe', 'pipe']});
  let stdout = '', stderr = '';
  p.stdout.on('data', b => { stdout += b; }); p.stderr.on('data', b => { stderr += b; }); p.on('error', reject);
  p.on('close', exit => { writeFileSync(join(output, log), stderr); exit === 0 ? ok(stdout) : reject(Error('child failed: ' + log)); });
  p.stdin.end(input);
});

let session, currentState, parameters, expectedState, proofHash, proofCalls = 0, prebalance;
const baseProver = httpClientProvingProvider(endpoints.proof, keys, {timeout: 15 * 60_000});
const proveProvider = {
  check: (raw, location) => baseProver.check(raw, location),
  lookupKey: location => baseProver.lookupKey(location),
  prove: async (raw, location, binding) => {
    assert.equal(++proofCalls, 1, 'one contract proof per transaction');
    assert.equal(location, `contract:${target}/acceptRun?vk=${sha(currentState.operation('acceptRun').verifierKey)}`);
    assert.equal(typeof binding, 'bigint');
    const hex = (await child(proverCmd, [], JSON.stringify({binding_input: le(binding),
      preimage: Buffer.from(raw).toString('hex')}), 'run-prover-stderr.log', true)).trim();
    assert.match(hex, /^([0-9a-f]{2})+$/, 'prover output is hex');
    const proof = Buffer.from(hex, 'hex');
    proofHash = sha(proof);
    writeFileSync(join(output, 'acceptRun.tagged-proof'), proof); // public: the proof reveals no opening
    return new Uint8Array(proof);
  },
};
// Native proof-verifying check over a local ledger built from the indexed deployment and the current state.
const nativeValidate = async (tx, log) => {
  const data = await graphql(`{transactions(offset:{identifier:"${deployment.txId}"}){raw}}`);
  const deploymentTx = ledger.Transaction.deserialize('signature', 'proof', 'binding', Buffer.from(data.transactions[0].raw, 'hex'));
  const deploys = [...deploymentTx.intents.values()].flatMap(i => i.actions).filter(a => a instanceof ledger.ContractDeploy);
  assert.equal(deploys.length, 1); assert.equal(deploys[0].address, target);
  let local = ledger.LedgerState.blank('stagenet'); local.parameters = parameters;
  const setup = ledger.Transaction.fromParts('stagenet', undefined, undefined,
    ledger.Intent.new(new Date(Date.now() + 3600_000)).addDeploy(deploys[0])).eraseProofs();
  const lax = Object.assign(new ledger.WellFormedStrictness(), {enforceBalancing: false, verifyNativeProofs: false,
    verifyContractProofs: false, enforceLimits: false, verifySignatures: false});
  const secs = BigInt(Math.floor(Date.now() / 1000));
  const block = {secondsSinceEpoch: secs, secondsSinceEpochErr: 30, parentBlockHash: '00'.repeat(32), lastBlockTime: secs - 6n};
  const [boot, res] = local.apply(setup.wellFormed(local, lax, new Date()), new ledger.TransactionContext(local, block));
  assert.equal(res.type, 'success');
  const ls = ledger.ContractState.deserialize(currentState.serialize());
  local = boot.updateIndex(target, ls.data, ls.balance);
  const report = JSON.parse(await child(validateCall, [], JSON.stringify({state: Buffer.from(local.serialize()).toString('hex'),
    transaction: Buffer.from(tx.serialize()).toString('hex'), block_seconds: Number(secs)}), log + '-stderr.log'));
  assert.equal(report.proofVerification, 'native/proof-verifying');
  assert.equal(report.contractCalls.length, 1);
  const call = report.contractCalls[0];
  assert.equal(call.address, target); assert.equal(call.entryPoint, 'acceptRun'); assert.equal(call.proofVersion, 'V3');
  assert.equal(call.proofSHA256, proofHash);
  assert.ok(call.publicInputs.includes(le(run.stmt)), 'stmt in the public inputs');
  return report;
};
const proofProvider = {proveTx: async tx => {
  const proven = await tx.prove(proveProvider, parameters.transactionCostModel.runtimeCostModel);
  // Strict pre-balance check: contract proof, signatures and limits on; the call must apply.
  prebalance = await nativeValidate(ledger.Transaction.deserialize('signature', 'proof', 'pre-binding', proven.serialize()).bind(), 'native-prebalance');
  assert.equal(prebalance.contractProofsSignaturesLimitsAccepted, true, prebalance.contractProfileFailure);
  assert.equal(prebalance.applied, true, 'pre-balance native execution');
  writeFileSync(join(output, 'native-prebalance-check.json'), json(prebalance));
  return proven;
}};
const prepared = new Error('PREPARATION_ONLY');
try {
  if (submit) session = await openStagenetSession(load, output);
  const walletProvider = session?.walletProvider ?? {getCoinPublicKey: () => '00'.repeat(32), getEncryptionPublicKey: () => '00'.repeat(32)};
  const source = await publicDataProvider.queryZSwapAndContractState(target); assert.ok(source);
  [, currentState, parameters] = source;
  // The installed key is checked locally before anything is proved or spent.
  assert.equal(sha(currentState.operation('acceptRun').verifierKey), sha(vkRaw), 'installed acceptRun key is the run-decider key');
  const before = mod.ledger(currentState.data);
  assert.equal(before.program_d, run.p, 'sealed program_d'); assert.equal(before.init_d, run.i, 'sealed init_d');
  assert.ok(!before.runs.member(run.stmt), 'run not yet accepted');
  assert.notDeepEqual(Buffer.from(before.result), Buffer.from(be16(run.r)), 'post result must differ from the current result');
  writeFileSync(join(output, 'pre-state.bin'), currentState.serialize());
  // Deterministic VM prediction of the complete contract state.
  const clone = rt.ContractState.deserialize(currentState.serialize());
  const context = rt.createCircuitContext('acceptRun', target, parseCoinPublicKeyToHex(walletProvider.getCoinPublicKey(), 'stagenet'), clone, {});
  const predicted = await new mod.Contract({}).provableCircuits.acceptRun(context, ...args);
  clone.data = predicted.context.callContext.currentQueryContext.state;
  expectedState = Buffer.from(clone.serialize());
  const providers = {publicDataProvider, zkConfigProvider: keys, proofProvider, walletProvider,
    ...(session ? {privateStateProvider: session.privateStateProvider} : {}), midnightProvider: {submitTx: async tx => {
      if (!submit) throw prepared;
      // Final balanced transaction: a refusal about proofs or signatures, or a non-applied call, blocks submission;
      // a refusal caused only by wallet DUST/fee state absent from the blank local ledger is kept as evidence,
      // because the pre-balance call already passed the strict native check.
      const report = await nativeValidate(tx, 'native-final');
      if (!report.contractProofsSignaturesLimitsAccepted) {
        assert.doesNotMatch(report.contractProfileFailure, /proof|signature/i, report.contractProfileFailure);
        assert.ok(prebalance?.applied, 'pre-balance native check must have applied');
      } else assert.equal(report.applied, true, 'native execution against the current snapshot');
      writeFileSync(join(output, 'native-final-check.json'), json(report));
      const latest = await publicDataProvider.queryContractState(target); assert.ok(latest);
      assert.equal(sha(latest.serialize()), sha(currentState.serialize()), 'contract state changed since prediction');
      writeFileSync(join(output, 'submitted.json'), json({at: new Date().toISOString(), network: 'stagenet', address: target,
        circuit: 'acceptRun'}), {flag: 'wx', mode: 0o600});
      return session.walletProvider.submitTx(tx);
    }}};
  const unproven = await createUnprovenCallTx(providers, {compiledContract, contractAddress: target, circuitId: 'acceptRun', args});
  if (!submit) {
    await proofProvider.proveTx(unproven.private.unprovenTx);
    writeFileSync(join(output, 'preparation.json'), json({address: target, stmt: runJson.stmt, result: run.r, reference: reference ?? 'none (stand-in bundle)',
      bundleSHA256: sha(readFileSync(join(bundleDir, 'bundle.json'))), installedVerifierSHA256: sha(vkRaw), preStateSHA256: sha(currentState.serialize()),
      expectedPostStateSHA256: sha(expectedState), nativePrebalanceAccepted: true, proofSHA256: proofHash, submitted: false}));
    console.log('PASS acceptRun prepared: installed key, sealed digests and native proof-verifying ledger 9 accept; nothing submitted');
  } else {
    const result = await submitTx(providers, {unprovenTx: unproven.private.unprovenTx});
    assert.equal(result.status, SucceedEntirely);
    const receipt = {schema: 'sherlock-run-stagenet-call/1', address: target, circuit: 'acceptRun', txId: result.txId,
      txHash: result.txHash, blockHeight: result.blockHeight, blockHash: result.blockHash, status: result.status,
      preStateSHA256: sha(currentState.serialize()), expectedPostStateSHA256: sha(expectedState), proofSHA256: proofHash,
      stmt: runJson.stmt, result: run.r, reference: reference ?? 'none (stand-in bundle)',
      bundleSHA256: sha(readFileSync(join(bundleDir, 'bundle.json')))};
    writeFileSync(join(output, 'receipt.json'), json(receipt));
    const indexed = await graphql(`{contractAction(address:"${target}",offset:{transactionOffset:{hash:"${result.txHash}"}}){__typename state transaction{hash block{height hash}} ... on ContractCall{entryPoint}}}`);
    assert.equal(indexed.contractAction.__typename, 'ContractCall'); assert.equal(indexed.contractAction.transaction.hash, result.txHash);
    const post = Buffer.from(rt.ContractState.deserialize(Buffer.from(indexed.contractAction.state, 'hex')).serialize());
    assert.deepEqual(post, expectedState, 'complete contract effect readback');
    writeFileSync(join(output, 'post-state.bin'), post); writeFileSync(join(output, 'indexed-call.json'), json(indexed));
    console.log(json({...receipt, completeEffectsMatch: true}));
  }
} finally { if (session) await session.close(); }
process.exit(0);
