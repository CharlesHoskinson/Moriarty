// Maintainer-only SDK/native call preparation. No wallet or transaction submission.
// Usage: node rollup-local-call.mjs SDK_ROOT BUILD KEYED_BUILD STATE B3_DIR KEY OUTPUT [--prove|--wrong-statement]
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync, mkdirSync} from 'node:fs';
import {createRequire} from 'node:module';
import {resolve, join, dirname} from 'node:path';
import {pathToFileURL} from 'node:url';
import {spawn} from 'node:child_process';
import {createHash} from 'node:crypto';
import {anchorView, bindBatch, assertPredecessor, assertSettled} from './rollup-statement.mjs';
import {checkCompiledCall} from './rollup-compiled-check.mjs';
const [sdkRoot, build, keyedBuild, statePath, parent, key, output, mode] = process.argv.slice(2);
assert.ok(output && (mode === undefined || ['--prove', '--wrong-statement'].includes(mode)));
const proving = mode === '--prove';
const require = createRequire(resolve(sdkRoot, 'package.json'));
const load = name => import(pathToFileURL(require.resolve(name)).href);
const ledger = await load('@midnight-ntwrk/midnight-js-protocol/ledger');
const rt = await load('@midnight-ntwrk/compact-runtime');
const mod = await import(pathToFileURL(resolve(build, 'contract/index.js')).href);
const le = x => Buffer.from(x.toString(16).padStart(64, '0'), 'hex').reverse().toString('hex');
const state = ledger.ContractState.deserialize(readFileSync(statePath));
const deployment = new ledger.ContractDeploy(state);
const address = deployment.address;
const template = JSON.parse(readFileSync('crates/mori-recursion/tests/fixtures/rollup-template.json'));
const manifest = JSON.parse(readFileSync(join(key, 'manifest.json')));
assert.equal(manifest.schema, 'sherlock-b4-preparation/1');
const batch = bindBatch(JSON.parse(readFileSync(join(parent, 'leaf.instance.json'))),
  JSON.parse(readFileSync(join(parent, 'parent.instance.json'))), manifest, rt);
const stmt = batch.statement;
assertPredecessor(anchorView(rt, mod, state), batch);
const now = new Date(), ttl = new Date(now.getTime() + 3600_000);
const parameters = ledger.LedgerParameters.initialParameters();
// Install the exact keys and constructor state in an isolated local ledger. This bootstrap
// waives fees; it is setup, not evidence that a contract call is valid.
const blockContext = {secondsSinceEpoch: BigInt(Math.floor(now.getTime() / 1000)),
  secondsSinceEpochErr: 30, parentBlockHash: '00'.repeat(32),
  lastBlockTime: BigInt(Math.floor(now.getTime() / 1000)) - 6n};
const strictness = flags => Object.assign(new ledger.WellFormedStrictness(), flags);
const waived = {enforceBalancing: false, verifyNativeProofs: false, verifyContractProofs: false,
  enforceLimits: false, verifySignatures: false};
let local = ledger.LedgerState.blank('stagenet');
local.parameters = parameters;
const setup = ledger.Transaction.fromParts('stagenet', undefined, undefined,
  ledger.Intent.new(ttl).addDeploy(deployment)).eraseProofs();
const [installed, setupResult] = local.apply(setup.wellFormed(local, strictness(waived), now),
  new ledger.TransactionContext(local, blockContext));
assert.equal(setupResult.type, 'success', 'local deployment bootstrap');
local = installed;
assert.deepEqual(local.index(address).operation('acceptValidity').verifierKey,
  state.operation('acceptValidity').verifierKey);
const runtimeState = rt.ContractState.deserialize(state.serialize());
const context = rt.createCircuitContext('acceptValidity', address, '00'.repeat(32), runtimeState, {},
  undefined, undefined, undefined, now);
const result = await new mod.Contract({}).provableCircuits.acceptValidity(context, stmt + (mode === '--wrong-statement' ? 1n : 0n));
const pd = result.context.callProofDataTrace[0];
const nativeContext = new ledger.QueryContext(state.data, address);
nativeContext.block = context.callContext.currentQueryContext.block;
const [guaranteed, fallible] = ledger.partitionTranscripts([new ledger.PreTranscript(nativeContext, pd.publicTranscript)], parameters)[0];
assert.ok(guaranteed && !fallible, 'acceptValidity must be entirely guaranteed');
const randomness = ledger.communicationCommitmentRandomness();
const prototype = new ledger.ContractCallPrototype(address, 'acceptValidity', state.operation('acceptValidity'),
  guaranteed, undefined, pd.privateTranscriptOutputs, pd.input, pd.output, randomness, 'sherlock/rollup/acceptValidity');
const tx = ledger.Transaction.fromParts('stagenet', undefined, undefined, ledger.Intent.new(ttl).addCall(prototype));
mkdirSync(output, {recursive: true});
let callbacks = 0, preparedComplete = false;
const prepared = new Error('PREPARED_WITHOUT_PROVING');
const provider = {
  check: async (raw, location) => {
    assert.equal(location, 'sherlock/rollup/acceptValidity');
    return checkCompiledCall(raw, location, {ir: join(keyedBuild, 'zkir/acceptValidity.bzkir'),
      prover: join(keyedBuild, 'keys/acceptValidity.prover'), verifier: join(key, 'acceptValidity.verifier'), output});
  },
  lookupKey: async location => {
    assert.equal(location, 'sherlock/rollup/acceptValidity');
    return {proverKey: readFileSync(join(keyedBuild, 'keys/acceptValidity.prover')),
      verifierKey: state.operation('acceptValidity').verifierKey,
      ir: readFileSync(join(keyedBuild, 'zkir/acceptValidity.bzkir'))};
  },
  prove: async (raw, location, binding) => {
    assert.equal(++callbacks, 1); assert.equal(location, 'sherlock/rollup/acceptValidity');
    assert.equal(typeof binding, 'bigint');
    // Only this pipe carries the preimage, including its private opening. Never persist raw or stdin.
    const input = JSON.stringify({binding_input: le(binding), preimage: Buffer.from(raw).toString('hex')});
    const args = [proving ? 'prove' : 'check', resolve(parent),
      resolve('crates/mori-recursion/tests/fixtures/rollup-template.json'), resolve(key),
      resolve(output, 'proof'), manifest.instance_id,
      resolve(process.env.MORIARTY_PARENT_SRS ?? 'target/sherlock-srs/bls_midnight_2p20')];
    const env = {...process.env, RAYON_NUM_THREADS: '4'};
    await new Promise((ok, reject) => {
      const child = spawn('python3', ['-B', 'scripts/qa/measure-rollup.py',
        join(output, 'native-proof.measurement.json'), '--',
        resolve(process.env.ROLLUP_CALL_BINARY ?? 'target/release/examples/rollup_call'), ...args],
        {env, stdio: ['pipe', 'pipe', 'pipe']});
      let logs = '';
      child.stdout.on('data', b => { logs += b; }); child.stderr.on('data', b => { logs += b; });
      child.on('error', reject);
      child.on('close', exit => {
        writeFileSync(join(output, 'native-check.log'), logs);
        if (exit === 0) ok(); else reject(Error('native call check failed; see native-check.log'));
      });
      child.stdin.end(input);
    });
    if (!proving) {
      writeFileSync(join(output, 'preparation.json'), JSON.stringify({address, bindingInput: le(binding),
        stmt: le(stmt), templateSlots: template.slots.length, nativeVectorChecked: true,
        privateOpeningPersisted: false, proofGenerated: false, submitted: false}, null, 2) + '\n');
      preparedComplete = true;
      throw prepared;
    }
    const publicInputs = JSON.parse(readFileSync(join(output, 'proof/terminal.public.json')));
    assert.equal(publicInputs[0], le(binding)); assert.equal(publicInputs[11], le(stmt));
    assert.equal(publicInputs[25], le(stmt));
    return readFileSync(join(output, 'proof/terminal.tagged-proof'));
  },
};
const nativeCheck = async (proven, pre = local, log = 'native-ledger.log') => {
  const request = JSON.stringify({state: Buffer.from(pre.serialize()).toString('hex'),
    transaction: Buffer.from(proven.serialize()).toString('hex'),
    block_seconds: Number(blockContext.secondsSinceEpoch)});
  return new Promise((ok, reject) => {
    const child = spawn(resolve(process.env.SHERLOCK_NATIVE_VALIDATOR ?? 'target/release/examples/validate_call'), [], {stdio: ['pipe','pipe','pipe']});
    let stdout = '', stderr = '';
    child.stdout.on('data', b => { stdout += b; }); child.stderr.on('data', b => { stderr += b; });
    child.on('error', reject);
    child.on('close', exit => {
      writeFileSync(join(output, log), stderr);
      if (exit !== 0) reject(Error('native ledger input/validation failed; see native-ledger.log'));
      else { try { ok(JSON.parse(stdout)); } catch (error) { reject(error); } }
    });
    child.stdin.end(request);
  });
};
const contractChecks = {enforceBalancing: false, verifyNativeProofs: false,
  verifyContractProofs: true, enforceLimits: true, verifySignatures: true};
try {
  const proven = (await tx.prove(provider, parameters.transactionCostModel.runtimeCostModel)).bind();
  const native = await nativeCheck(proven);
  assert.equal(native.contractProofsSignaturesLimitsAccepted, true, native.contractProfileFailure);
  assert.equal(native.applied, true);
  const verified = proven.wellFormed(local, strictness(contractChecks), now);
  const [post, outcome] = local.apply(verified, new ledger.TransactionContext(local, blockContext));
  assert.equal(outcome.type, 'success');
  const before = anchorView(rt, mod, state), after = anchorView(rt, mod, post.index(address));
  assert.ok(after.validity_ok.member(stmt)); assert.ok(after.settled.isEmpty());
  for (const field of ['instance_id', 'root', 'height', 'batch_no', 'dep_consumed', 'dep_acc',
    'last_in_root', 'last_fee_sum', 'last_wd_sum_usd', 'last_wd_sum_night',
    'last_dep_sum_usd', 'last_dep_sum_night']) assert.equal(after[field], before[field]);
  const postHash = createHash('sha256').update(post.serialize()).digest('hex');
  assert.equal(postHash, native.postStateSHA256, 'native/WASM complete post-state correspondence');
  // Rehearse the compiled settlement too, before the first network transaction.
  // The outer local-preparation command already owns the heavy slot.
  const accepted = post.index(address);
  const settleContext = rt.createCircuitContext('settleValidity', address, '00'.repeat(32),
    rt.ContractState.deserialize(accepted.serialize()), {}, undefined, undefined, undefined, now);
  const settledCall = await new mod.Contract({}).provableCircuits.settleValidity(settleContext, batch.batch, batch.fields);
  const settleData = settledCall.context.callProofDataTrace[0];
  const settleQuery = new ledger.QueryContext(accepted.data, address);
  settleQuery.block = settleContext.callContext.currentQueryContext.block;
  const [settleTranscript, settleFallible] = ledger.partitionTranscripts(
    [new ledger.PreTranscript(settleQuery, settleData.publicTranscript)], parameters)[0];
  assert.ok(settleTranscript && !settleFallible);
  const settleLocation = `contract:${address}/settleValidity?vk=${createHash('sha256').update(accepted.operation('settleValidity').verifierKey).digest('hex')}`;
  const settlePrototype = new ledger.ContractCallPrototype(address, 'settleValidity', accepted.operation('settleValidity'),
    settleTranscript, undefined, settleData.privateTranscriptOutputs, settleData.input, settleData.output,
    ledger.communicationCommitmentRandomness(), settleLocation);
  const settleTx = ledger.Transaction.fromParts('stagenet', undefined, undefined, ledger.Intent.new(ttl).addCall(settlePrototype));
  const settleProvider = {
    check: (raw, location) => checkCompiledCall(raw, location, {ir: join(keyedBuild, 'zkir/settleValidity.bzkir'),
      prover: join(keyedBuild, 'keys/settleValidity.prover'), verifier: join(keyedBuild, 'keys/settleValidity.verifier'), output}),
    lookupKey: async () => ({proverKey: readFileSync(join(keyedBuild, 'keys/settleValidity.prover')),
      verifierKey: accepted.operation('settleValidity').verifierKey,
      ir: readFileSync(join(keyedBuild, 'zkir/settleValidity.bzkir'))}),
    prove: async (raw, location, binding) => {
      assert.equal(location, settleLocation);
      const proof = resolve(output, 'settle.tagged-proof');
      await new Promise((ok, reject) => {
        const child = spawn(process.execPath, ['tools/sherlock-sdk/rollup-proof-compiled.mjs', resolve(sdkRoot),
          resolve(dirname(statePath), 'preparation.json'), location, proof], {stdio: ['pipe', 'pipe', 'pipe']});
        let logs = '';
        child.stdout.on('data', b => { logs += b; }); child.stderr.on('data', b => { logs += b; });
        child.on('error', reject);
        child.on('close', code => {
          writeFileSync(join(output, 'settle-prover.log'), logs);
          code === 0 ? ok() : reject(Error('compiled settlement worker failed'));
        });
        child.stdin.end(JSON.stringify({binding: String(binding), preimage: Buffer.from(raw).toString('hex')}));
      });
      return readFileSync(proof);
    },
  };
  const settledTx = (await settleTx.prove(settleProvider, parameters.transactionCostModel.runtimeCostModel)).bind();
  const settleNative = await nativeCheck(settledTx, post, 'settle-native-ledger.log');
  assert.equal(settleNative.contractProofsSignaturesLimitsAccepted, true);
  assert.equal(settleNative.applied, true);
  const [settledState, settledOutcome] = post.apply(settledTx.wellFormed(post, strictness(contractChecks), now),
    new ledger.TransactionContext(post, blockContext));
  assert.equal(settledOutcome.type, 'success');
  assert.equal(createHash('sha256').update(settledState.serialize()).digest('hex'), settleNative.postStateSHA256);
  assertSettled(anchorView(rt, mod, settledState.index(address)), batch);
  const result = {schema: 'sherlock-rollup-local-call/1', address, contractChecks,
    proofVerification: native.proofVerification,
    contractProofAccepted: true, applied: true, validityStatementPresent: true,
    rootHeightBatchUnchanged: true, nativePostStateSHA256: postHash,
    settlementProofAccepted: true, settlementApplied: true, completeSettledEffectsMatch: true,
    nativeSettledPostStateSHA256: settleNative.postStateSHA256,
    allOn: native.allOn, allOnFailure: native.allOnFailure, submitted: false};
  writeFileSync(join(output, 'ledger-result.json'), JSON.stringify(result, null, 2) + '\n');
  console.log(JSON.stringify(result));
} catch (error) {
  if (callbacks === 1 && preparedComplete) console.log('PASS real SDK call vector checked by native decider adapter; no proof or submission');
  else throw error;
}
