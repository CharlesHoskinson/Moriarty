// Maintainer-only SDK/native call preparation. No wallet or transaction submission.
// Usage: node check-local-call.mjs SDK_ROOT BUILD KEYED_BUILD STATE PARENT KEY OUTPUT [--prove|--wrong-statement|--reject-offline-proof]
// WARNING: the installed SDK JS WASM wellFormed does not verify proofs cryptographically (ledger-wasm is
// built without proof-verifying); every acceptance claim here comes from the native validate_call example.
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync, mkdirSync} from 'node:fs';
import {createRequire} from 'node:module';
import {resolve, join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {spawn} from 'node:child_process';
import {createHash} from 'node:crypto';
const [sdkRoot, build, keyedBuild, statePath, parent, key, output, mode] = process.argv.slice(2);
assert.ok(output && (mode === undefined || ['--prove', '--wrong-statement', '--reject-offline-proof'].includes(mode)));
const proving = mode === '--prove';
const require = createRequire(resolve(sdkRoot, 'package.json'));
const load = name => import(pathToFileURL(require.resolve(name)).href);
assert.equal((await (await fetch('http://localhost:6300/version')).text()).trim(), '9.0.0-rc.5');
const ledger = await load('@midnight-ntwrk/midnight-js-protocol/ledger');
const rt = await load('@midnight-ntwrk/compact-runtime');
const mod = await import(pathToFileURL(resolve(build, 'contract/index.js')).href);
const le = x => Buffer.from(x.toString(16).padStart(64, '0'), 'hex').reverse().toString('hex');
const fromLe = x => BigInt('0x' + Buffer.from(x, 'hex').reverse().toString('hex'));
const state = ledger.ContractState.deserialize(readFileSync(statePath));
const deployment = new ledger.ContractDeploy(state);
const address = deployment.address;
const template = JSON.parse(readFileSync('crates/mori-recursion/tests/fixtures/accept-template.json'));
const preparedPublic = JSON.parse(readFileSync(join(key, '../terminal.public.json')));
const stmt = fromLe(preparedPublic[10]);
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
  guaranteed, undefined, pd.privateTranscriptOutputs, pd.input, pd.output, randomness, 'sherlock/acceptValidity');
const tx = ledger.Transaction.fromParts('stagenet', undefined, undefined, ledger.Intent.new(ttl).addCall(prototype));
mkdirSync(output, {recursive: true});
let callbacks = 0, preparedComplete = false;
const prepared = new Error('PREPARED_WITHOUT_PROVING');
const provider = {
  check: async (raw, location) => {
    assert.equal(location, 'sherlock/acceptValidity');
    const payload = ledger.createCheckPayload(raw, readFileSync(join(keyedBuild, 'zkir/acceptValidity.bzkir')));
    const response = await fetch('http://localhost:6300/check', {method: 'POST',
      headers: {'content-type': 'application/octet-stream'}, body: payload});
    assert.ok(response.ok, 'compiled call check');
    return ledger.parseCheckResult(new Uint8Array(await response.arrayBuffer()));
  },
  lookupKey: async location => {
    assert.equal(location, 'sherlock/acceptValidity');
    return {proverKey: readFileSync(join(keyedBuild, 'keys/acceptValidity.prover')),
      verifierKey: state.operation('acceptValidity').verifierKey,
      ir: readFileSync(join(keyedBuild, 'zkir/acceptValidity.bzkir'))};
  },
  prove: async (raw, location, binding) => {
    assert.equal(++callbacks, 1); assert.equal(location, 'sherlock/acceptValidity');
    assert.equal(typeof binding, 'bigint');
    // Only this pipe carries the preimage, including its private opening. Never persist raw or stdin.
    const input = JSON.stringify({binding_input: le(binding), preimage: Buffer.from(raw).toString('hex')});
    const args = ['test', '--release', '--locked', '-p', 'mori-recursion', '--features', 'sherlock-ledger',
      '--test', 'sherlock_terminal', proving ? 'terminal_proof_matches_the_installed_stagenet_verifier'
        : 'sdk_call_vector_matches_the_decider_template', '--', '--exact', '--include-ignored', '--nocapture', '--test-threads=1'];
    const env = {...process.env, CARGO_BUILD_JOBS: '4', RAYON_NUM_THREADS: '4',
      MORIARTY_A3_PARENT_DIR: resolve(parent), MORIARTY_TERMINAL_CALL_STDIN: '1',
      MORIARTY_DEPLOYED_CONTRACT_STATE: resolve(statePath),
      MORIARTY_PARENT_SRS: resolve('target/sherlock-srs/bls_midnight_2p20'),
      MORIARTY_TERMINAL_KEY_DIR: resolve(key), MORIARTY_TERMINAL_OUTPUT_DIR: resolve(output, 'proof')};
    await new Promise((ok, reject) => {
      const child = spawn('cargo', args, {env, stdio: ['pipe', 'pipe', 'pipe']});
      let logs = '';
      child.stdout.on('data', b => { logs += b; }); child.stderr.on('data', b => { logs += b; });
      child.on('error', reject);
      child.on('close', exit => {
        writeFileSync(join(output, 'native-check.log'), logs);
        if (exit === 0) ok(); else reject(Error('native call check failed; see native-check.log'));
      });
      child.stdin.end(input);
    });
    if (mode === '--reject-offline-proof') return readFileSync(join(key, '../terminal.tagged-proof'));
    if (!proving) {
      writeFileSync(join(output, 'preparation.json'), JSON.stringify({address, bindingInput: le(binding),
        stmt: le(stmt), templateSlots: template.slots.length, nativeVectorChecked: true,
        privateOpeningPersisted: false, proofGenerated: false, submitted: false}, null, 2) + '\n');
      preparedComplete = true;
      throw prepared;
    }
    const publicInputs = JSON.parse(readFileSync(join(output, 'proof/terminal.public.json')));
    assert.equal(publicInputs[0], le(binding)); assert.equal(publicInputs[10], le(stmt));
    return readFileSync(join(output, 'proof/terminal.tagged-proof'));
  },
};
const nativeCheck = async proven => {
  const request = JSON.stringify({state: Buffer.from(local.serialize()).toString('hex'),
    transaction: Buffer.from(proven.serialize()).toString('hex'),
    block_seconds: Number(blockContext.secondsSinceEpoch)});
  return new Promise((ok, reject) => {
    const child = spawn(resolve('target/release/examples/validate_call'), [], {stdio: ['pipe','pipe','pipe']});
    let stdout = '', stderr = '';
    child.stdout.on('data', b => { stdout += b; }); child.stderr.on('data', b => { stderr += b; });
    child.on('error', reject);
    child.on('close', exit => {
      writeFileSync(join(output, 'native-ledger.log'), stderr);
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
  if (mode === '--reject-offline-proof') {
    const wasmAccepted = !!proven.wellFormed(local, strictness(contractChecks), now);
    const native = await nativeCheck(proven);
    assert.equal(native.contractProofsSignaturesLimitsAccepted, false);
    assert.equal(native.applied, false);
    assert.match(native.contractProfileFailure, /proof/i);
    writeFileSync(join(output, 'negative-ledger-result.json'), JSON.stringify({wasmAccepted, native}, null, 2) + '\n');
    console.log('PASS native ledger refuses reused offline proof; installed JS WASM accepted it without cryptographic verification');
  } else {
    const native = await nativeCheck(proven);
    assert.equal(native.contractProofsSignaturesLimitsAccepted, true, native.contractProfileFailure);
    assert.equal(native.applied, true);
    const verified = proven.wellFormed(local, strictness(contractChecks), now);
    const [post, outcome] = local.apply(verified, new ledger.TransactionContext(local, blockContext));
    assert.equal(outcome.type, 'success');
    const view = s => mod.ledger(rt.ContractState.deserialize(s.serialize()).data);
    const before = view(state), after = view(post.index(address));
    assert.ok(after.validity_ok.member(stmt)); assert.ok(after.settled.isEmpty());
    for (const field of ['root', 'height', 'batch_no']) assert.equal(after[field], before[field]);
    const postHash = createHash('sha256').update(post.serialize()).digest('hex');
    assert.equal(postHash, native.postStateSHA256, 'native/WASM complete post-state correspondence');
    const result = {schema: 'sherlock-local-contract-call/1', address, contractChecks,
      proofVerification: native.proofVerification,
      contractProofAccepted: true, applied: true, validityStatementPresent: true,
      rootHeightBatchUnchanged: true, nativePostStateSHA256: postHash,
      allOn: native.allOn, allOnFailure: native.allOnFailure, submitted: false};
    writeFileSync(join(output, 'ledger-result.json'), JSON.stringify(result, null, 2) + '\n');
    console.log(JSON.stringify(result));
  }
} catch (error) {
  if (callbacks === 1 && preparedComplete) console.log('PASS real SDK call vector checked by native decider adapter; no proof or submission');
  else throw error;
}
