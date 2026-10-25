// Maintainer-only SDK/native call preparation. No wallet or transaction submission.
// Usage: node check-local-call.mjs SDK_ROOT BUILD KEYED_BUILD STATE PARENT KEY OUTPUT [--prove|--wrong-statement]
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync, mkdirSync} from 'node:fs';
import {createRequire} from 'node:module';
import {resolve, join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {spawn} from 'node:child_process';
const [sdkRoot, build, keyedBuild, statePath, parent, key, output, mode] = process.argv.slice(2);
assert.ok(output && (mode === undefined || ['--prove', '--wrong-statement'].includes(mode)));
const proving = mode === '--prove';
const require = createRequire(resolve(sdkRoot, 'package.json'));
const load = name => import(pathToFileURL(require.resolve(name)).href);
const ledger = await load('@midnight-ntwrk/midnight-js-protocol/ledger');
const rt = await load('@midnight-ntwrk/compact-runtime');
const mod = await import(pathToFileURL(resolve(build, 'contract/index.js')).href);
const le = x => Buffer.from(x.toString(16).padStart(64, '0'), 'hex').reverse().toString('hex');
const fromLe = x => BigInt('0x' + Buffer.from(x, 'hex').reverse().toString('hex'));
const state = ledger.ContractState.deserialize(readFileSync(statePath));
const address = new ledger.ContractDeploy(state).address;
const template = JSON.parse(readFileSync('crates/mori-recursion/tests/fixtures/accept-template.json'));
const preparedPublic = JSON.parse(readFileSync(join(key, '../terminal.public.json')));
const stmt = fromLe(preparedPublic[10]);
const now = new Date(), ttl = new Date(now.getTime() + 3600_000);
const parameters = ledger.LedgerParameters.initialParameters();
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
try {
  await tx.prove(provider, parameters.transactionCostModel.runtimeCostModel);
  console.log('PASS native D proof returned to SDK; ledger acceptance remains a separate check');
} catch (error) {
  if (callbacks === 1 && preparedComplete) console.log('PASS real SDK call vector checked by native decider adapter; no proof or submission');
  else throw error;
}
