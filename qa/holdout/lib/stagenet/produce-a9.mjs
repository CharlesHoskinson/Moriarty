// Evaluator-owned A9 evidence producer: stagenet rejection attempts on the pinned Sherlock verifier.
// Usage (repo root): node qa/holdout/lib/stagenet/produce-a9.mjs SDK_ROOT BUILD KEYED PARENT KEY OUTPUT [--submit]
// Without --submit: no wallet; builds the s-b wrong-key call through the proof server and records the
// native proof-verifying prediction. With --submit (run under `doctorwatson heavy`): one wallet session
//   1. twin T: a fresh actual-bound decider proof for acceptValidity(stmt) (stmt already accepted by A8), balanced;
//   2. s-a mutant M: the exact balanced bytes of T with one bit of its contract proof flipped;
//   3. s-b mutant B: acceptValidity(fresh stmt) proven under the compiled acceptValidity key, not the installed decider key;
//   order: build, balance and submit B (must be refused; frees its dust), then prove and balance T, submit M
//   (must be refused), then T (positive control, must finalize),
//   then the identical bytes of T again (s-e transaction replay; must be refused).
// The wallet comes only from tools/sherlock-sdk/stagenet-session.mjs; no seed, key or transaction body is
// written or logged. Receipts hold hashes, offsets and sanitized refusal text only.
// Native validate_call is a predictor of the refusal class; the node and indexer are the oracle.
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync, mkdirSync, existsSync} from 'node:fs';
import {resolve, join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {createHash} from 'node:crypto';
import {spawn} from 'node:child_process';
import {sdkLoader, endpoints, openStagenetSession} from '../../../../tools/sherlock-sdk/stagenet-session.mjs';

const [sdkRoot, build, keyed, parent, key, outputArg, mode] = process.argv.slice(2);
assert.ok(outputArg, 'usage: produce-a9.mjs SDK_ROOT BUILD KEYED PARENT KEY OUTPUT [--submit]');
assert.ok(mode === undefined || mode === '--submit' || mode === '--resubmit');
const submit = mode === '--submit', resubmit = mode === '--resubmit';
const output = resolve(outputArg); mkdirSync(output, {recursive: true, mode: 0o700});
// --resubmit OUTPUT names a finished run: resubmit the twin's indexed bytes after the node pool's temporary ban.
if (resubmit) assert.ok(existsSync(join(output, 'receipt.json')) && !existsSync(join(output, 'resubmit.json')), 'resubmit needs a finished run without resubmit.json');
else assert.ok(!existsSync(join(output, 'receipt.json')), 'evidence already recorded in this output directory');
const caseOf = id => JSON.parse(readFileSync(`qa/holdout/cases/rejections/${id}/case.json`));
const FLIP = Object.freeze((({proofOffset, mask}) => ({proofOffset, mask}))(caseOf('s-a-tampered-inner-proof').mutation));
const STMT_B_TAG = caseOf('s-b-mismatched-verifying-key').mutation.stmtTag;
assert.ok(Number.isInteger(FLIP.proofOffset) && [1, 2, 4, 8, 16, 32, 64, 128].includes(FLIP.mask) && typeof STMT_B_TAG === 'string');
const sha = b => createHash('sha256').update(b).digest('hex');
const le = x => Buffer.from(x.toString(16).padStart(64, '0'), 'hex').reverse().toString('hex');
const fromLe = s => BigInt('0x' + Buffer.from(s, 'hex').reverse().toString('hex'));
const json = x => JSON.stringify(x, (_, v) => typeof v === 'bigint' ? v.toString() : v, 2) + '\n';
const log = m => console.log(`[a9] ${m}`);
const load = sdkLoader(sdkRoot, join(output, 'sdk-imports'));
log(`output ${output}`);
const pins = JSON.parse(readFileSync('crates/mori-recursion/tests/fixtures/stagenet-verifier.json'));
assert.equal(sha(readFileSync(join(parent, 'w.vk'))), pins.parent_vk_sha256);
assert.equal(sha(readFileSync(join(key, 'decider.vk'))), pins.vk_sha256);
assert.equal(sha(readFileSync(join(key, 'decider.relation'))), pins.relation_sha256);
assert.equal((await (await fetch(endpoints.proof + '/version')).text()).trim(), '9.0.0-rc.5');
const {setNetworkId} = await load('@midnight-ntwrk/midnight-js-network-id'); setNetworkId('stagenet');
const {WebSocket} = await load('ws'); globalThis.WebSocket = WebSocket;
const ledger = await load('@midnight-ntwrk/midnight-js-protocol/ledger');
const {CompiledContract} = await load('@midnight-ntwrk/midnight-js-protocol/compact-js');
const {createVerifierKey} = await load('@midnight-ntwrk/midnight-js-types');
const {createUnprovenCallTx} = await load('@midnight-ntwrk/midnight-js-contracts');
const {indexerPublicDataProvider} = await load('@midnight-ntwrk/midnight-js-indexer-public-data-provider');
const {NodeZkConfigProvider} = await load('@midnight-ntwrk/midnight-js-node-zk-config-provider');
const {httpClientProvingProvider} = await load('@midnight-ntwrk/midnight-js-http-client-proof-provider');
const mod = await import(pathToFileURL(resolve(build, 'contract/index.js')).href);
const publicDataProvider = indexerPublicDataProvider(endpoints.indexer, endpoints.indexerWs);
const graphql = async query => {
  const r = await fetch(endpoints.indexer, {method: 'POST', headers: {'content-type': 'application/json'}, body: JSON.stringify({query})});
  assert.ok(r.ok); const body = await r.json(); assert.ok(!body.errors, 'indexer query failed'); return body.data;
};
const child = (command, argv, input, env, name) => new Promise((ok, reject) => {
  const p = spawn(command, argv, {env, stdio: ['pipe', 'pipe', 'pipe']}); let out = '', err = '';
  p.stdout.on('data', b => { out += b; }); p.stderr.on('data', b => { err += b; }); p.on('error', reject);
  p.on('close', code => { writeFileSync(join(output, name), err); code === 0 ? ok(out) : reject(Error('child failed: ' + name)); });
  p.stdin.end(input);
});
// Refusal text only: tags and messages along the Effect cause chain (including symbol-keyed causes),
// long hex runs (transaction bodies) removed. Never serializes whole error objects.
const refusal = error => {
  const out = [], seen = new Set();
  const visit = (e, depth) => {
    if (!e || typeof e !== 'object' || seen.has(e) || depth > 8 || out.length >= 16) return;
    seen.add(e);
    const tag = e._tag ?? e.name, msg = typeof e.message === 'string' ? e.message : null;
    if (tag || msg) out.push({tag: String(tag ?? ''), message: String(msg ?? '').replace(/[0-9a-fA-F]{129,}/g, '<hex>').slice(0, 600)});
    for (const k of ['cause', 'error', 'failure', 'reason', 'left'])
      if (k in e) visit(e[k], depth + 1);
    for (const k of Object.getOwnPropertySymbols(e)) if (/Cause/.test(String(k))) visit(e[k], depth + 1);
  };
  visit(error, 0);
  if (!out.length) out.push({tag: typeof error, message: String(error).slice(0, 600)});
  return out;
};

class DeciderKeys extends NodeZkConfigProvider {           // the installed decider key, as the A8 adapter
  async getVerifierKey(name) {
    return name === 'acceptValidity' ? createVerifierKey(readFileSync(join(key, 'acceptValidity.verifier'))) : super.getVerifierKey(name);
  }
}
const deciderKeys = new DeciderKeys(resolve(keyed));
const compiledKeys = new NodeZkConfigProvider(resolve(keyed));   // the compiler's own acceptValidity key (s-b)
const compiledContract = CompiledContract.make('SherlockAnchor', mod.Contract).pipe(
  CompiledContract.withVacantWitnesses, CompiledContract.withCompiledFileAssets(resolve(build)));
const stmt = fromLe(JSON.parse(readFileSync(join(key, '../terminal.public.json')))[10]);
const FR = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001n;
const stmtB = BigInt('0x' + sha(STMT_B_TAG)) % FR;

// One pinned snapshot for assertions, native validation and the indexed no-change check.
const [, currentState, parameters] = await publicDataProvider.queryZSwapAndContractState(pins.address);
const preStateSHA256 = sha(currentState.serialize());
const before = mod.ledger(currentState.data);
assert.ok(before.validity_ok.member(stmt), 'twin statement must already be accepted (A8)');
assert.ok(!before.settled.member(stmt));
assert.ok(!before.validity_ok.member(stmtB) && !before.settled.member(stmtB), 's-b statement must be fresh');
assert.equal(sha(await deciderKeys.getVerifierKey('acceptValidity')), sha(currentState.operation('acceptValidity').verifierKey));
const compiledVk = await compiledKeys.getVerifierKey('acceptValidity');
assert.notEqual(sha(compiledVk), sha(currentState.operation('acceptValidity').verifierKey), 's-b key must differ from the installed key');
if (!resubmit) writeFileSync(join(output, 'pre-state.bin'), currentState.serialize());

let deployState;
const nativeValidate = async (tx, name) => {
  if (!deployState) {
    const deployed = JSON.parse(readFileSync('target/a4-anchor/deploy-run3/receipt.json'));
    assert.equal(deployed.address, pins.address);
    const data = await graphql(`{transactions(offset:{identifier:"${deployed.txId}"}){raw}}`);
    const dtx = ledger.Transaction.deserialize('signature', 'proof', 'binding', Buffer.from(data.transactions[0].raw, 'hex'));
    const deps = [...dtx.intents.values()].flatMap(i => i.actions).filter(a => a instanceof ledger.ContractDeploy);
    assert.equal(deps.length, 1); assert.equal(deps[0].address, pins.address);
    deployState = deps[0];
  }
  let local = ledger.LedgerState.blank('stagenet'); local.parameters = parameters;
  const setup = ledger.Transaction.fromParts('stagenet', undefined, undefined, ledger.Intent.new(new Date(Date.now() + 3600_000)).addDeploy(deployState)).eraseProofs();
  const lax = Object.assign(new ledger.WellFormedStrictness(), {enforceBalancing: false, verifyNativeProofs: false, verifyContractProofs: false, enforceLimits: false, verifySignatures: false});
  const secs = BigInt(Math.floor(Date.now() / 1000)), block = {secondsSinceEpoch: secs, secondsSinceEpochErr: 30, parentBlockHash: '00'.repeat(32), lastBlockTime: secs - 6n};
  const [boot, result] = local.apply(setup.wellFormed(local, lax, new Date()), new ledger.TransactionContext(local, block));
  assert.equal(result.type, 'success');
  const st = ledger.ContractState.deserialize(currentState.serialize());
  local = boot.updateIndex(pins.address, st.data, st.balance);
  const report = JSON.parse(await child(resolve('target/release/examples/validate_call'), [],
    JSON.stringify({state: Buffer.from(local.serialize()).toString('hex'), transaction: Buffer.from(tx.serialize()).toString('hex'), block_seconds: Number(secs)}), process.env, name + '-stderr.log'));
  assert.equal(report.proofVerification, 'native/proof-verifying');
  const {contractProofsSignaturesLimitsAccepted, contractProfileFailure, applied, allOn, allOnFailure, transactionSHA256} = report;
  const kept = {contractProofsSignaturesLimitsAccepted, contractProfileFailure, applied, allOn, allOnFailure, transactionSHA256,
    calls: report.contractCalls.map(c => ({address: c.address, entryPoint: c.entryPoint, proofSHA256: c.proofSHA256, stmt: c.publicInputs?.[10] ?? null}))};
  writeFileSync(join(output, name + '.json'), json(kept));
  return kept;
};
const isProofRefusal = r => !r.contractProofsSignaturesLimitsAccepted && /failed to verify proof/i.test(r.contractProfileFailure ?? '');

let session;
const dummyWallet = {getCoinPublicKey: () => '00'.repeat(32), getEncryptionPublicKey: () => '00'.repeat(32)};
const providers = (zkConfigProvider, proofProvider) => ({publicDataProvider, zkConfigProvider, proofProvider,
  walletProvider: session?.walletProvider ?? dummyWallet, ...(session ? {privateStateProvider: session.privateStateProvider} : {}),
  midnightProvider: {submitTx: async () => { throw Error('evaluator submits explicitly'); }}});
const callOptions = s => ({compiledContract, contractAddress: pins.address, circuitId: 'acceptValidity', args: [s]});
const costModel = () => parameters.transactionCostModel.runtimeCostModel;

// s-b: an honest proof server proof under the compiled acceptValidity key.
const buildWrongKey = async () => {
  // The SDK resolver refuses a key whose hash differs from the deployed one (a client-side guard).
  // The bare circuit name selects the compiled bundle, so the node, not the client, decides.
  const base = httpClientProvingProvider(endpoints.proof, compiledKeys, {timeout: 15 * 60_000});
  const prover = {check: raw => base.check(raw, 'acceptValidity'), lookupKey: () => base.lookupKey('acceptValidity'),
    prove: (raw, _loc, binding) => base.prove(raw, 'acceptValidity', binding)};
  const unproven = await createUnprovenCallTx(providers(compiledKeys, null), callOptions(stmtB));
  const proven = await unproven.private.unprovenTx.prove(prover, costModel());
  return proven;
};
// Twin: the A8 native decider prover, unchanged invocation.
let twinProofHash, twinProofBytes;
const buildTwin = async () => {
  let calls = 0;
  const provider = {
    check: (raw, loc) => httpClientProvingProvider(endpoints.proof, deciderKeys).check(raw, loc),
    lookupKey: loc => httpClientProvingProvider(endpoints.proof, deciderKeys).lookupKey(loc),
    prove: async (raw, location, binding) => {
      assert.equal(++calls, 1, 'one contract proof per transaction');
      assert.equal(location, `contract:${pins.address}/acceptValidity?vk=${sha(currentState.operation('acceptValidity').verifierKey)}`);
      const env = {...process.env, CARGO_BUILD_JOBS: '4', RAYON_NUM_THREADS: '4',
        MORIARTY_A3_PARENT_DIR: resolve(parent), MORIARTY_TERMINAL_CALL_STDIN: '1',
        MORIARTY_DEPLOYED_CONTRACT_STATE: join(output, 'pre-state.bin'),
        MORIARTY_PARENT_SRS: resolve('target/sherlock-srs/bls_midnight_2p20'),
        MORIARTY_TERMINAL_KEY_DIR: resolve(key), MORIARTY_TERMINAL_OUTPUT_DIR: join(output, 'proof')};
      await child('cargo', ['test', '--release', '--locked', '-p', 'mori-recursion', '--features', 'sherlock-ledger',
        '--test', 'sherlock_terminal', 'terminal_proof_matches_the_installed_stagenet_verifier', '--', '--exact',
        '--include-ignored', '--nocapture', '--test-threads=1'],
        JSON.stringify({binding_input: le(binding), preimage: Buffer.from(raw).toString('hex')}), env, 'native-prover-stderr.log');
      const pub = JSON.parse(readFileSync(join(output, 'proof/terminal.public.json')));
      assert.equal(pub[0], le(binding)); assert.equal(pub[10], le(stmt));
      twinProofBytes = readFileSync(join(output, 'proof/terminal.proof')); twinProofHash = sha(twinProofBytes);
      return readFileSync(join(output, 'proof/terminal.tagged-proof'));
    },
  };
  const unproven = await createUnprovenCallTx(providers(deciderKeys, null), callOptions(stmt));
  return unproven.private.unprovenTx.prove(provider, costModel());
};
const flipProof = finalTx => {
  const bytes = Buffer.from(finalTx.serialize());
  const at = bytes.indexOf(twinProofBytes);
  assert.ok(at > 0 && bytes.indexOf(twinProofBytes, at + 1) === -1, 'twin proof must occur exactly once in the transaction');
  const txOffset = at + FLIP.proofOffset; assert.ok(FLIP.proofOffset < twinProofBytes.length);
  const mutated = Buffer.from(bytes); mutated[txOffset] ^= FLIP.mask;
  return {tx: ledger.Transaction.deserialize('signature', 'proof', 'binding', mutated),
    flip: {proofStart: at, proofLength: twinProofBytes.length, proofOffset: FLIP.proofOffset, txOffset, mask: FLIP.mask}};
};
const unboundForCheck = proven => ledger.Transaction.deserialize('signature', 'proof', 'pre-binding', proven.serialize()).bind();
const attempt = async (name, tx) => {
  const hash = sha(Buffer.from(tx.serialize()));
  const at = new Date().toISOString();
  try {
    await session.walletProvider.submitTx(tx);
    log(`${name}: node ACCEPTED ${hash}`);
    return {name, txSHA256: hash, at, nodeOutcome: 'accepted'};
  } catch (error) {
    log(`${name}: node refused ${hash}`);
    return {name, txSHA256: hash, at, nodeOutcome: 'refused', refusal: refusal(error)};
  }
};
const latestStateSHA = async () => sha((await publicDataProvider.queryContractState(pins.address)).serialize());
const waitIndexed = async hash => {
  for (let i = 0; i < 60; i++) {
    const d = await graphql(`{transactions(offset:{hash:"${hash}"}){hash block{height hash} ... on RegularTransaction{transactionResult{status}}}}`);
    if (d.transactions.length) return d.transactions;
    await new Promise(r => setTimeout(r, 5000));
  }
  return [];
};

const receipt = {schema: 'sherlock-a9-stagenet-rejections/1', address: pins.address, network: 'stagenet', submitted: submit,
  preStateSHA256, stmt: le(stmt), stmtB: le(stmtB), stmtBTag: STMT_B_TAG,
  installedAcceptVkSHA256: sha(currentState.operation('acceptValidity').verifierKey), compiledAcceptVkSHA256: sha(compiledVk)};
try {
  if (resubmit) {
    const done = JSON.parse(readFileSync(join(output, 'receipt.json')));
    const twinAttempt = done.attempts.find(a => a.name === 'twin');
    assert.equal(twinAttempt?.nodeOutcome, 'accepted');
    const waited = (Date.now() - Date.parse(twinAttempt.at)) / 60_000;
    assert.ok(waited >= 35, `node pool ban window: wait ${Math.ceil(35 - waited)} more minutes`);
    const data = await graphql(`{transactions(offset:{hash:"${twinAttempt.txSHA256}"}){raw}}`);
    assert.equal(data.transactions.length, 1);
    const raw = Buffer.from(data.transactions[0].raw, 'hex'); assert.equal(sha(raw), twinAttempt.txSHA256);
    session = await openStagenetSession(load, join(output, 'session-3'));
    log('wallet synced for resubmission');
    const a = await attempt('s-e-identical-bytes-after-ban', ledger.Transaction.deserialize('signature', 'proof', 'binding', raw));
    writeFileSync(join(output, 'resubmit.json'), json({schema: 'sherlock-a9-resubmit/1', twinSHA256: twinAttempt.txSHA256,
      minutesAfterTwin: waited, attempt: a, finishedAt: new Date().toISOString()}), {flag: 'wx'});
    console.log(json({resubmit: [a.name, a.nodeOutcome, a.refusal?.at(-1)?.message]}));
  } else if (!submit) {
    const B = await buildWrongKey();
    receipt.prediction = {sB: await nativeValidate(unboundForCheck(B), 'native-sB-prebalance')};
    assert.ok(isProofRefusal(receipt.prediction.sB), 's-b prediction: the native validator must refuse the wrong-key proof');
    writeFileSync(join(output, 'preparation.json'), json(receipt));
    log('PASS preparation: s-b wrong-key call built and natively refused; no wallet, no submission');
  } else {
    session = await openStagenetSession(load, output);
    log('wallet synced');
    receipt.prediction = {}; receipt.attempts = [];
    // s-b first: cheap to build, and its refusal (wallet revert) releases its dust before the twin is balanced.
    const B0 = await buildWrongKey();
    log('s-b built; balancing');
    const B = await session.walletProvider.balanceTx(B0);
    receipt.prediction.sB = await nativeValidate(B, 'native-sB-final');
    assert.ok(isProofRefusal(receipt.prediction.sB), 'sB prediction must be a contract proof refusal');
    assert.equal(await latestStateSHA(), preStateSHA256, 'contract state changed since the pinned snapshot');
    receipt.attempts.push(await attempt('s-b', B));
    // A refused transaction keeps its dust locked locally until the dust grace period; a fresh session
    // re-syncs from chain, where the s-b dust spend never landed.
    await session.close(); session = undefined;
    session = await openStagenetSession(load, join(output, 'session-2'));
    log('wallet re-synced for the twin');
    const T0 = await buildTwin();
    const pre = await nativeValidate(unboundForCheck(T0), 'native-twin-prebalance');
    assert.ok(pre.contractProofsSignaturesLimitsAccepted && pre.applied, 'twin control must be accepted and applied natively before balancing');
    log('twin proven; balancing');
    const T = await session.walletProvider.balanceTx(T0);
    const {tx: M, flip} = flipProof(T);
    receipt.prediction.twin = await nativeValidate(T, 'native-twin-final');
    receipt.prediction.sA = await nativeValidate(M, 'native-sA-final');
    assert.ok(isProofRefusal(receipt.prediction.sA), 'sA prediction must be a contract proof refusal');
    assert.ok(!/proof|signature/i.test(receipt.prediction.twin.contractProfileFailure ?? ''), 'twin final must not fail on proofs or signatures');
    receipt.flip = flip; receipt.twinProofSHA256 = twinProofHash;
    assert.equal(await latestStateSHA(), preStateSHA256, 'contract state changed since the pinned snapshot');
    receipt.attempts.push(await attempt('s-a', M));
    receipt.stateAfterMutantsSHA256 = await latestStateSHA();
    receipt.attempts.push(await attempt('twin', T));
    receipt.twinIndexed = await waitIndexed(sha(Buffer.from(T.serialize())));
    receipt.attempts.push(await attempt('s-e-identical-bytes', T));
    receipt.finishedAt = new Date().toISOString();
  }
} finally {
  if (session) await session.close();
  if (submit && receipt.attempts) writeFileSync(join(output, 'receipt.json'), json(receipt), {flag: 'wx'});
}
if (submit) console.log(json({attempts: receipt.attempts?.map(a => [a.name, a.nodeOutcome, a.txSHA256])}));
