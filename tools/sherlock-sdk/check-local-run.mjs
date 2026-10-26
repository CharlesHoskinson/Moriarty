// G9: local ledger-9 accept/refuse check for SherlockRun.acceptRun. No wallet, network or submission.
// Usage: node check-local-run.mjs SDK_ROOT BUILD VALIDATE_CALL OUTPUT [--external CMD VERIFIER]
//   BUILD          compiled tools/sherlock-sdk/sherlock-run.compact (compact 0.34.0, --feature-zkir-v3)
//   VALIDATE_CALL  target/release/examples/validate_call (native ledger 9, proof-verifying)
//   default prover the compiled acceptRun key through the local proof server (/prove). This checks the
//                  ledger side of the binding (transcript, reads, writes, replay), not the decider relation.
//   --external     install VERIFIER (tagged ledger VerifierKey, e.g. the run-decider key) as acceptRun's key
//                  and prove with CMD: it reads {"binding_input": hex32le, "preimage": hex} on stdin and
//                  writes the tagged proof as hex on stdout. The preimage holds the private communication
//                  opening: it is only piped, never written.
// The installed SDK JS WASM does not verify proofs cryptographically; every accept/refuse verdict here
// comes from VALIDATE_CALL. Cases:
//   accept        a fresh acceptRun against the deployed state is proof-verified and applied; the post state
//                 has result = r (BE u128), steps = n and stmt in runs
//   flipped-proof one proof byte flipped: refused by proof verification
//   reused-proof  the valid proof attached to a call with another result r': refused
//   other-program a proof made against a state whose program_d differs (a proof of another program): the
//                 ledger's read of program_d refuses it
//   other-init    the same for init_d
//   replay        a fresh proof of the same stmt made against the pre-run state, applied after the run: the
//                 ledger's runs.member read refuses it
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync, mkdirSync, existsSync, symlinkSync} from 'node:fs';
import {createRequire} from 'node:module';
import {resolve, join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {spawn} from 'node:child_process';
import {createHash} from 'node:crypto';
import {installedKey, resultWindow, readWindow, callEffects} from './run-layout.mjs';

const argv = process.argv.slice(2);
const ext = argv.indexOf('--external');
const [sdkRoot, buildArg, validateCall, outputArg] = argv.slice(0, ext < 0 ? argv.length : ext);
assert.ok(outputArg, 'usage: check-local-run.mjs SDK_ROOT BUILD VALIDATE_CALL OUTPUT [--external CMD VERIFIER]');
const external = ext < 0 ? undefined : {cmd: argv[ext + 1], vk: argv[ext + 2]};
assert.ok(ext < 0 || (external.cmd && external.vk && argv.length === ext + 3), '--external CMD VERIFIER');
const build = resolve(buildArg), output = resolve(outputArg);
mkdirSync(output, {recursive: true});
const sha = b => createHash('sha256').update(b).digest('hex');
const require = createRequire(resolve(sdkRoot, 'package.json'));
const load = name => import(pathToFileURL(require.resolve(name)).href);
const ledger = await load('@midnight-ntwrk/midnight-js-protocol/ledger');
const rt = await load('@midnight-ntwrk/compact-runtime');
if (!existsSync(join(build, 'node_modules'))) symlinkSync(resolve(sdkRoot, 'node_modules'), join(build, 'node_modules'));
if (!existsSync(join(build, 'package.json'))) writeFileSync(join(build, 'package.json'), '{"type":"module"}\n');
const mod = await import(pathToFileURL(join(build, 'contract/index.js')).href);
const proverKey = readFileSync(join(build, 'keys/acceptRun.prover'));
const ir = readFileSync(join(build, 'zkir/acceptRun.bzkir'));
const vkRaw = readFileSync(external ? external.vk : join(build, 'keys/acceptRun.verifier'));
assert.ok(vkRaw.subarray(0, 22).toString().startsWith('midnight:verifier-key['), 'tagged ledger verifier key');
assert.equal((await (await fetch('http://localhost:6300/version')).text()).trim(), '9.0.0-rc.5');
const le = x => Buffer.from(x.toString(16).padStart(64, '0'), 'hex').reverse().toString('hex');
const be16 = v => new Uint8Array(Buffer.from(v.toString(16).padStart(32, '0'), 'hex'));
const LOC = 'sherlock/acceptRun';

// Deterministic, nonzero test values; with --external, real values come with the decider proof (G11).
const PROG = 0x1111n, INIT = 0x1212n, R0 = 1n;
const run = {stmt: 0x4242n, p: PROG, i: INIT, n: 7n, fin: 0x9999n, r: 6n};

const now = new Date(), ttl = new Date(now.getTime() + 3600_000);
const blockContext = {secondsSinceEpoch: BigInt(Math.floor(now.getTime() / 1000)), secondsSinceEpochErr: 30,
  parentBlockHash: '00'.repeat(32), lastBlockTime: BigInt(Math.floor(now.getTime() / 1000)) - 6n};
const parameters = ledger.LedgerParameters.initialParameters();
const strictness = flags => Object.assign(new ledger.WellFormedStrictness(), flags);

// Constructor state with the installed acceptRun key, as the deployment would hold it.
const constructed = async (prog, init) => {
  const initial = await new mod.Contract({}).initialState(rt.createConstructorContext({}, '00'.repeat(32)),
    prog, init, be16(R0));
  const state = ledger.ContractState.deserialize(initial.currentContractState.serialize());
  const op = new ledger.ContractOperation();
  op.verifierKey = vkRaw;
  state.setOperation('acceptRun', op);
  assert.deepEqual(state.operations().map(String), ['acceptRun']);
  return state;
};
const deployState = await constructed(PROG, INIT);
const deployment = new ledger.ContractDeploy(deployState);
const address = deployment.address;
// Isolated local ledger with that deployment. The bootstrap waives fees and proofs (a deploy has none);
// it is setup, not evidence about any call.
let pre = ledger.LedgerState.blank('stagenet');
pre.parameters = parameters;
{
  const setup = ledger.Transaction.fromParts('stagenet', undefined, undefined,
    ledger.Intent.new(ttl).addDeploy(deployment)).eraseProofs();
  const waived = {enforceBalancing: false, verifyNativeProofs: false, verifyContractProofs: false,
    enforceLimits: false, verifySignatures: false};
  const [installed, res] = pre.apply(setup.wellFormed(pre, strictness(waived), now),
    new ledger.TransactionContext(pre, blockContext));
  assert.equal(res.type, 'success', 'local deployment bootstrap');
  pre = installed;
}
assert.equal(sha(pre.index(address).operation('acceptRun').verifierKey), sha(vkRaw), 'installed acceptRun key');

// One acceptRun call computed against `against` (a ContractState whose data the circuit reads).
const callTx = async (against, args) => {
  const runtimeState = rt.ContractState.deserialize(against.serialize());
  const context = rt.createCircuitContext('acceptRun', address, '00'.repeat(32), runtimeState, {},
    undefined, undefined, undefined, now);
  const result = await new mod.Contract({}).provableCircuits.acceptRun(context, args.stmt, args.p, args.i,
    args.n, args.fin, be16(args.r));
  const pd = result.context.callProofDataTrace[0];
  const qc = new ledger.QueryContext(against.data, address);
  qc.block = context.callContext.currentQueryContext.block;
  const [guaranteed, fallible] = ledger.partitionTranscripts([new ledger.PreTranscript(qc, pd.publicTranscript)],
    parameters)[0];
  assert.ok(guaranteed && !fallible, 'acceptRun must be entirely guaranteed');
  const proto = new ledger.ContractCallPrototype(address, 'acceptRun', deployState.operation('acceptRun'),
    guaranteed, undefined, pd.privateTranscriptOutputs, pd.input, pd.output,
    ledger.communicationCommitmentRandomness(), LOC);
  return ledger.Transaction.fromParts('stagenet', undefined, undefined, ledger.Intent.new(ttl).addCall(proto));
};
const proveRaw = async (raw, binding) => {
  if (!external) {
    const payload = ledger.createProvingPayload(raw, binding, {proverKey, verifierKey: vkRaw, ir});
    const response = await fetch('http://localhost:6300/prove', {method: 'POST',
      headers: {'content-type': 'application/octet-stream'}, body: payload});
    assert.ok(response.ok, `proof server /prove ${response.status}`);
    return new Uint8Array(await response.arrayBuffer());
  }
  return new Promise((ok, reject) => {
    const child = spawn('/bin/sh', ['-c', external.cmd], {stdio: ['pipe', 'pipe', 'pipe']});
    let stdout = '', stderr = '';
    child.stdout.on('data', b => { stdout += b; }); child.stderr.on('data', b => { stderr += b; });
    child.on('error', reject);
    child.on('close', exit => {
      writeFileSync(join(output, 'external-prover.log'), stderr);
      if (exit !== 0) return reject(Error('external prover failed; see external-prover.log'));
      ok(new Uint8Array(Buffer.from(stdout.trim(), 'hex')));
    });
    child.stdin.end(JSON.stringify({binding_input: le(binding), preimage: Buffer.from(raw).toString('hex')}));
  });
};
// Prove a call; `override(proof)` may replace the proof bytes (refusal cases).
const prove = async (tx, override = p => p) => {
  let n = 0;
  const provider = {
    check: async (raw, location) => {
      assert.equal(location, LOC);
      const response = await fetch('http://localhost:6300/check', {method: 'POST',
        headers: {'content-type': 'application/octet-stream'}, body: ledger.createCheckPayload(raw, ir)});
      assert.ok(response.ok, `proof server /check ${response.status}`);
      return ledger.parseCheckResult(new Uint8Array(await response.arrayBuffer()));
    },
    lookupKey: async () => ({proverKey, verifierKey: vkRaw, ir}),
    prove: async (raw, location, binding) => {
      assert.equal(++n, 1); assert.equal(location, LOC); assert.equal(typeof binding, 'bigint');
      return override(await proveRaw(raw, binding));
    },
  };
  return (await tx.prove(provider, parameters.transactionCostModel.runtimeCostModel)).bind();
};
const nativeCheck = (state, tx, name) => new Promise((ok, reject) => {
  const request = JSON.stringify({state: Buffer.from(state.serialize()).toString('hex'),
    transaction: Buffer.from(tx.serialize()).toString('hex'), block_seconds: Number(blockContext.secondsSinceEpoch)});
  const child = spawn(resolve(validateCall), [], {stdio: ['pipe', 'pipe', 'pipe']});
  let stdout = '', stderr = '';
  child.stdout.on('data', b => { stdout += b; }); child.stderr.on('data', b => { stderr += b; });
  child.on('error', reject);
  child.on('close', exit => {
    writeFileSync(join(output, `native-${name}.log`), stderr);
    if (exit !== 0) return reject(Error(`validate_call failed on ${name}; see native-${name}.log`));
    try { ok(JSON.parse(stdout)); } catch (error) { reject(error); }
  });
  child.stdin.end(request);
});
const view = s => mod.ledger(rt.ContractState.deserialize(s.serialize()).data);
const cases = {};
const refused = async (name, state, tx) => {
  const native = await nativeCheck(state, tx, name);
  assert.equal(native.applied, false, `${name} must be refused`);
  const stage = native.contractProofsSignaturesLimitsAccepted ? 'transcript application' : 'proof/contract check';
  cases[name] = {verdict: 'refused', stage, failure: native.contractProfileFailure ?? (native.contractProofsSignaturesLimitsAccepted ? "apply result not Success" : null),
    publicInputs: native.contractCalls?.[0]?.publicInputs?.length ?? null};
  console.log(`PASS ${name}: refused at ${stage}`);
};

// accept
const validTx = await prove(await callTx(deployState, run));
const native = await nativeCheck(pre, validTx, 'accept');
assert.equal(native.contractProofsSignaturesLimitsAccepted, true, native.contractProfileFailure);
assert.equal(native.applied, true, 'valid acceptRun applied');
const contractChecks = {enforceBalancing: false, verifyNativeProofs: false, verifyContractProofs: true,
  enforceLimits: true, verifySignatures: true};
const [post, outcome] = pre.apply(validTx.wellFormed(pre, strictness(contractChecks), now),
  new ledger.TransactionContext(pre, blockContext));
assert.equal(outcome.type, 'success');
assert.equal(sha(post.serialize()), native.postStateSHA256, 'native/WASM post-state correspondence');
const before = view(pre.index(address)), after = view(post.index(address));
assert.deepEqual(Buffer.from(before.result), Buffer.from(be16(R0)));
assert.deepEqual(Buffer.from(after.result), Buffer.from(be16(run.r)));
assert.equal(after.steps, run.n); assert.ok(after.runs.member(run.stmt));
assert.equal(after.program_d, PROG); assert.equal(after.init_d, INIT);
assert.equal(sha(post.index(address).operation('acceptRun').verifierKey), sha(vkRaw), 'key unchanged by the call');
cases.accept = {verdict: 'accepted', resultBefore: Buffer.from(before.result).toString('hex'),
  resultAfter: Buffer.from(after.result).toString('hex'), steps: String(after.steps),
  publicInputs: native.contractCalls?.[0]?.publicInputs?.length ?? null, postStateSHA256: native.postStateSHA256};
// Optional: the real ledger public-input vector must fit the decider transcript template slot by slot.
if (process.env.SHERLOCK_RUN_TEMPLATE) {
  const pis = native.contractCalls[0].publicInputs;
  const {slots} = JSON.parse(readFileSync(process.env.SHERLOCK_RUN_TEMPLATE));
  const values = [run.p, run.i, run.fin, run.n, BigInt('0x' + (Buffer.from(be16(run.r)).reverse().toString('hex')))];
  assert.equal(pis.length, slots.length, 'template length');
  slots.forEach((slot, k) => {
    if (slot.kind === 'const') assert.equal(pis[k], slot.value, `const slot ${k}`);
    else if (slot.kind === 'value') assert.equal(pis[k], le(values[Number(slot.value)]), `value slot ${k}`);
    else if (slot.kind === 'stmt') assert.equal(pis[k], le(run.stmt), `stmt slot ${k}`);
  });
  cases.accept.templateMatched = {path: process.env.SHERLOCK_RUN_TEMPLATE, slots: slots.length,
    sha256: sha(readFileSync(process.env.SHERLOCK_RUN_TEMPLATE))};
  console.log(`PASS template: ${slots.length} ledger public inputs fit const/stmt/value slots`);
}
// E7/E9 byte analysis on the local deploy and post states (the same code reads the indexed bytes in G11).
{
  const d = pre.index(address).serialize(), q = post.index(address).serialize();
  const key = installedKey(d, vkRaw); assert.equal(installedKey(q, vkRaw).bodySHA256, key.bodySHA256);
  const layout = resultWindow(d, q, R0, run.r);
  assert.equal(readWindow(d, layout), R0); assert.equal(readWindow(q, layout), run.r);
  const action = [...validTx.intents.values()].flatMap(i => i.actions)[0];
  const effects = callEffects(action); assert.ok(effects.empty, JSON.stringify(effects.sizes));
  cases.accept.effects = effects.sizes;
  cases.accept.installedKey = key; cases.accept.layout = layout;
  console.log(`PASS layout: u128-be window at ${layout.offset}; key body once, ${key.gapBytes} bytes after the entry name`);
}
console.log(`PASS accept: result ${cases.accept.resultBefore} -> ${cases.accept.resultAfter}, applied`);

// refusals
await refused('flipped-proof', pre, await prove(await callTx(deployState, run), p => {
  const q = Uint8Array.from(p); q[q.length - 40] ^= 1; return q; }));
const validProof = [];
await prove(await callTx(deployState, run), p => { validProof.push(p); return p; });
await refused('reused-proof', pre, await prove(await callTx(deployState, {...run, r: 7n}), () => validProof[0]));
await refused('other-program', pre, await prove(await callTx(await constructed(PROG + 1n, INIT), {...run, p: PROG + 1n})));
await refused('other-init', pre, await prove(await callTx(await constructed(PROG, INIT + 1n), {...run, i: INIT + 1n})));
await refused('replay', post, await prove(await callTx(deployState, run)));

const record = {schema: 'sherlock-run-local-ledger-check/1', submitted: false,
  prover: external ? 'external' : 'compiled acceptRun key via local proof server 9.0.0-rc.5',
  bindsDeciderRelation: !!external,
  note: external ? undefined : 'the compiled key only proves the transcript; the result/program binding to a proved run is the decider (G7)',
  installedVerifier: {bytes: vkRaw.length, sha256: sha(vkRaw)},
  build: {acceptRunProverSHA256: sha(proverKey), acceptRunBzkirSHA256: sha(ir)},
  validateCallSHA256: sha(readFileSync(resolve(validateCall))), address, cases};
writeFileSync(join(output, 'ledger-check.json'), JSON.stringify(record, null, 2) + '\n');
console.log(JSON.stringify({cases: Object.fromEntries(Object.entries(cases).map(([k, v]) => [k, v.verdict]))}));
process.exit(0);
