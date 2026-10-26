// G9: local ledger-9 accept/refuse check for SherlockRun.acceptRun. No wallet, network or submission.
// Usage: node check-local-run.mjs SDK_ROOT BUILD VALIDATE_CALL OUTPUT [--bundle GP_BUNDLE PROVER_CMD]
//   BUILD          compiled tools/sherlock-sdk/sherlock-run.compact (compact 0.34.0, --feature-zkir-v3)
//   VALIDATE_CALL  target/release/examples/validate_call (native ledger 9, proof-verifying)
//   default        the compiled acceptRun key, proved by the local proof server, with fixed test values. This
//                  checks the ledger side of the binding (transcript, reads, writes, replay) only.
//   --bundle       a gp_fold_and_key bundle (bundle.json, key/acceptRun.verifier): its run-decider key is the
//                  installed acceptRun key, its run values are the call, and PROVER_CMD proves: it reads
//                  {"binding_input": hex32le, "preimage": hex} on stdin and writes the tagged proof as hex on
//                  stdout (tools/sherlock-sdk/gp-call-prover.sh). The preimage holds the private communication
//                  opening: it is only piped, never written.
//   SHERLOCK_RUN_TEMPLATE  optional template JSON; the real public-input vector must fit it slot by slot.
// The installed SDK JS WASM does not verify proofs cryptographically; every accept/refuse verdict here
// comes from VALIDATE_CALL. The local ledger holds three deployments with the same key: MAIN (program_d,
// init_d of the run), OTHER-PROGRAM (program_d + 1) and OTHER-INIT (init_d + 1). Cases:
//   accept        a fresh acceptRun on MAIN is proof-verified and applied: result = r (BE u128), steps = n,
//                 stmt in runs
//   flipped-proof the accepted transaction with one byte of its proof flipped: refused by proof verification
//   reused-proof  the valid proof attached to a call with result r + 1: refused by proof verification
//   other-program a valid proof of the run sent to OTHER-PROGRAM: the ledger's read of program_d refuses it
//   other-init    the same on OTHER-INIT, for init_d
//   replay        a fresh valid proof of the same stmt, made against the pre-run state and applied after the
//                 run: the ledger's runs.member read refuses it
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync, mkdirSync, existsSync, symlinkSync} from 'node:fs';
import {createRequire} from 'node:module';
import {resolve, join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {spawn} from 'node:child_process';
import {createHash} from 'node:crypto';
import {installedKey, resultWindow, readWindow, callEffects} from './run-layout.mjs';

const argv = process.argv.slice(2);
const at = argv.indexOf('--bundle');
const [sdkRoot, buildArg, validateCall, outputArg] = argv.slice(0, at < 0 ? argv.length : at);
assert.ok(outputArg, 'usage: check-local-run.mjs SDK_ROOT BUILD VALIDATE_CALL OUTPUT [--bundle GP_BUNDLE PROVER_CMD]');
assert.ok(at < 0 || argv.length === at + 3, '--bundle GP_BUNDLE PROVER_CMD');
const bundleDir = at < 0 ? undefined : resolve(argv[at + 1]), proverCmd = at < 0 ? undefined : argv[at + 2];
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
const vkRaw = readFileSync(bundleDir ? join(bundleDir, 'key/acceptRun.verifier') : join(build, 'keys/acceptRun.verifier'));
assert.ok(vkRaw.subarray(0, 22).toString().startsWith('midnight:verifier-key['), 'tagged ledger verifier key');
assert.equal((await (await fetch('http://localhost:6300/version')).text()).trim(), '9.0.0-rc.5');
const le = x => Buffer.from(x.toString(16).padStart(64, '0'), 'hex').reverse().toString('hex');
const fromLe = h => { assert.match(h, /^[0-9a-f]{64}$/); return BigInt('0x' + Buffer.from(h, 'hex').reverse().toString('hex')); };
const be16 = v => new Uint8Array(Buffer.from(v.toString(16).padStart(32, '0'), 'hex'));
const LOC = 'sherlock/acceptRun', R0 = 1n;

let bundle, run;
if (bundleDir) {
  bundle = JSON.parse(readFileSync(join(bundleDir, 'bundle.json')));
  assert.equal(bundle.schema, 'sherlock-gp-bundle/1');
  assert.equal(sha(vkRaw), bundle.acceptRun_verifier_sha256, 'bundle acceptRun key');
  run = {stmt: fromLe(bundle.stmt), p: fromLe(bundle.prog_d), i: fromLe(bundle.init_d), fin: fromLe(bundle.fin_d),
    n: BigInt(bundle.steps), r: BigInt(bundle.result)};
  assert.equal(bundle.result_be16_hex, Buffer.from(be16(run.r)).toString('hex'));
} else run = {stmt: 0x4242n, p: 0x1111n, i: 0x1212n, n: 7n, fin: 0x9999n, r: 6n};
assert.notEqual(run.r, R0, 'the run result must differ from the initial value');

const now = new Date(), ttl = new Date(now.getTime() + 3600_000);
const blockContext = {secondsSinceEpoch: BigInt(Math.floor(now.getTime() / 1000)), secondsSinceEpochErr: 30,
  parentBlockHash: '00'.repeat(32), lastBlockTime: BigInt(Math.floor(now.getTime() / 1000)) - 6n};
const parameters = ledger.LedgerParameters.initialParameters();
const strictness = flags => Object.assign(new ledger.WellFormedStrictness(), flags);

// Constructor state with the installed acceptRun key, as the deployment holds it.
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
const mainState = await constructed(run.p, run.i);
const deploys = {main: new ledger.ContractDeploy(mainState),
  'other-program': new ledger.ContractDeploy(await constructed(run.p + 1n, run.i)),
  'other-init': new ledger.ContractDeploy(await constructed(run.p, run.i + 1n))};
const address = deploys.main.address;
// Isolated local ledger with the three deployments. The bootstrap waives fees and proofs (a deploy has
// none); it is setup, not evidence about any call.
let pre = ledger.LedgerState.blank('stagenet');
pre.parameters = parameters;
for (const d of Object.values(deploys)) {
  const setup = ledger.Transaction.fromParts('stagenet', undefined, undefined, ledger.Intent.new(ttl).addDeploy(d)).eraseProofs();
  const waived = {enforceBalancing: false, verifyNativeProofs: false, verifyContractProofs: false,
    enforceLimits: false, verifySignatures: false};
  const [installed, res] = pre.apply(setup.wellFormed(pre, strictness(waived), now), new ledger.TransactionContext(pre, blockContext));
  assert.equal(res.type, 'success', 'local deployment bootstrap');
  pre = installed;
}
for (const d of Object.values(deploys))
  assert.equal(sha(pre.index(d.address).operation('acceptRun').verifierKey), sha(vkRaw), 'installed acceptRun key');

// One acceptRun call to `target`, computed against `against` (the contract data the circuit reads).
const callTx = async (target, against, args) => {
  const runtimeState = rt.ContractState.deserialize(against.serialize());
  const context = rt.createCircuitContext('acceptRun', target, '00'.repeat(32), runtimeState, {},
    undefined, undefined, undefined, now);
  const result = await new mod.Contract({}).provableCircuits.acceptRun(context, args.stmt, args.p, args.i,
    args.n, args.fin, be16(args.r));
  const pd = result.context.callProofDataTrace[0];
  const qc = new ledger.QueryContext(against.data, target);
  qc.block = context.callContext.currentQueryContext.block;
  const [guaranteed, fallible] = ledger.partitionTranscripts([new ledger.PreTranscript(qc, pd.publicTranscript)], parameters)[0];
  assert.ok(guaranteed && !fallible, 'acceptRun must be entirely guaranteed');
  const proto = new ledger.ContractCallPrototype(target, 'acceptRun', mainState.operation('acceptRun'),
    guaranteed, undefined, pd.privateTranscriptOutputs, pd.input, pd.output,
    ledger.communicationCommitmentRandomness(), LOC);
  return ledger.Transaction.fromParts('stagenet', undefined, undefined, ledger.Intent.new(ttl).addCall(proto));
};
let proofs = 0;
const proveRaw = async (raw, binding) => {
  proofs++;
  if (!bundleDir) {
    const payload = ledger.createProvingPayload(raw, binding, {proverKey, verifierKey: vkRaw, ir});
    const response = await fetch('http://localhost:6300/prove', {method: 'POST',
      headers: {'content-type': 'application/octet-stream'}, body: payload});
    assert.ok(response.ok, `proof server /prove ${response.status}`);
    return new Uint8Array(await response.arrayBuffer());
  }
  return new Promise((ok, reject) => {
    const child = spawn('/bin/sh', ['-c', proverCmd], {stdio: ['pipe', 'pipe', 'pipe']});
    let stdout = '', stderr = '';
    child.stdout.on('data', b => { stdout += b; }); child.stderr.on('data', b => { stderr += b; });
    child.on('error', reject);
    child.on('close', exit => {
      writeFileSync(join(output, `prover-${proofs}.log`), stderr);
      if (exit !== 0) return reject(Error(`prover failed; see prover-${proofs}.log`));
      const hex = stdout.trim();
      if (!/^([0-9a-f]{2})+$/.test(hex)) return reject(Error('prover stdout is not hex'));
      ok(new Uint8Array(Buffer.from(hex, 'hex')));
    });
    child.stdin.end(JSON.stringify({binding_input: le(binding), preimage: Buffer.from(raw).toString('hex')}));
  });
};
// Prove a call; `fixed` replaces the proof without proving (the reused-proof case).
const prove = async (tx, fixed) => {
  let n = 0, proof;
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
      proof = fixed ?? await proveRaw(raw, binding);
      return proof;
    },
  };
  const proven = (await tx.prove(provider, parameters.transactionCostModel.runtimeCostModel)).bind();
  return {tx: proven, proof};
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
const refused = async (name, state, tx, expectStage) => {
  const native = await nativeCheck(state, tx, name);
  assert.equal(native.applied, false, `${name} must be refused`);
  const stage = native.contractProofsSignaturesLimitsAccepted ? 'transcript application' : 'proof/contract check';
  assert.equal(stage, expectStage, `${name} refusal stage`);
  cases[name] = {verdict: 'refused', stage, failure: native.contractProfileFailure ??
    (native.contractProofsSignaturesLimitsAccepted ? 'apply result not Success' : null),
    target: native.contractCalls?.[0]?.address, publicInputs: native.contractCalls?.[0]?.publicInputs?.length ?? null};
  console.log(`PASS ${name}: refused at ${stage}`);
};

// accept
const valid = await prove(await callTx(address, mainState, run));
const native = await nativeCheck(pre, valid.tx, 'accept');
assert.equal(native.contractProofsSignaturesLimitsAccepted, true, native.contractProfileFailure);
assert.equal(native.applied, true, 'valid acceptRun applied');
const contractChecks = {enforceBalancing: false, verifyNativeProofs: false, verifyContractProofs: true,
  enforceLimits: true, verifySignatures: true};
const [post, outcome] = pre.apply(valid.tx.wellFormed(pre, strictness(contractChecks), now),
  new ledger.TransactionContext(pre, blockContext));
assert.equal(outcome.type, 'success');
assert.equal(sha(post.serialize()), native.postStateSHA256, 'native/WASM post-state correspondence');
const before = view(pre.index(address)), after = view(post.index(address));
assert.deepEqual(Buffer.from(before.result), Buffer.from(be16(R0)));
assert.deepEqual(Buffer.from(after.result), Buffer.from(be16(run.r)));
assert.equal(after.steps, run.n); assert.ok(after.runs.member(run.stmt));
assert.equal(after.program_d, run.p); assert.equal(after.init_d, run.i);
assert.equal(sha(post.index(address).operation('acceptRun').verifierKey), sha(vkRaw), 'key unchanged by the call');
cases.accept = {verdict: 'accepted', resultBefore: Buffer.from(before.result).toString('hex'),
  resultAfter: Buffer.from(after.result).toString('hex'), steps: String(after.steps),
  proofSHA256: native.contractCalls[0].proofSHA256,
  publicInputs: native.contractCalls[0].publicInputs.length, postStateSHA256: native.postStateSHA256};
if (process.env.SHERLOCK_RUN_TEMPLATE) {
  const pis = native.contractCalls[0].publicInputs;
  const {slots} = JSON.parse(readFileSync(process.env.SHERLOCK_RUN_TEMPLATE));
  const values = [run.p, run.i, run.fin, run.n, BigInt('0x' + Buffer.from(be16(run.r)).reverse().toString('hex'))];
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
  const effects = callEffects([...valid.tx.intents.values()].flatMap(i => i.actions)[0]);
  assert.ok(effects.empty, JSON.stringify(effects.sizes));
  Object.assign(cases.accept, {effects: effects.sizes, installedKey: key, layout});
  console.log(`PASS layout: u128-be window at ${layout.offset}; key body once, ${key.gapBytes} bytes after the entry name`);
}
console.log(`PASS accept: result ${cases.accept.resultBefore} -> ${cases.accept.resultAfter}, applied`);

// refusals
{
  const raw = Buffer.from(valid.tx.serialize());
  const tail = Buffer.from(valid.proof).subarray(-64);
  const atProof = raw.indexOf(tail);
  assert.ok(atProof > 0 && raw.indexOf(tail, atProof + 1) < 0, 'proof bytes located once in the transaction');
  raw[atProof + 16] ^= 1;
  const flipped = ledger.Transaction.deserialize('signature', 'proof', 'binding', raw);
  await refused('flipped-proof', pre, flipped, 'proof/contract check');
}
await refused('reused-proof', pre, (await prove(await callTx(address, mainState, {...run, r: run.r + 1n}), valid.proof)).tx,
  'proof/contract check');
await refused('other-program', pre, (await prove(await callTx(deploys['other-program'].address, mainState, run))).tx,
  'transcript application');
await refused('other-init', pre, (await prove(await callTx(deploys['other-init'].address, mainState, run))).tx,
  'transcript application');
await refused('replay', post, (await prove(await callTx(address, mainState, run))).tx, 'transcript application');

const record = {schema: 'sherlock-run-local-ledger-check/2', submitted: false,
  prover: bundleDir ? 'run decider (gp_call_prove)' : 'compiled acceptRun key via local proof server 9.0.0-rc.5',
  bindsDeciderRelation: !!bundleDir, proofsMade: proofs,
  note: bundleDir ? undefined : 'the compiled key only proves the transcript; the result/program binding to a proved run is the decider (G7)',
  bundle: bundleDir ? {dir: bundleDir, sha256: sha(readFileSync(join(bundleDir, 'bundle.json'))), standin: bundle.standin,
    steps: bundle.steps, result: bundle.result, stmt: bundle.stmt} : undefined,
  installedVerifier: {bytes: vkRaw.length, sha256: sha(vkRaw)},
  build: {acceptRunProverSHA256: sha(proverKey), acceptRunBzkirSHA256: sha(ir)},
  validateCallSHA256: sha(readFileSync(resolve(validateCall))),
  addresses: Object.fromEntries(Object.entries(deploys).map(([k, d]) => [k, d.address])), cases};
writeFileSync(join(output, 'ledger-check.json'), JSON.stringify(record, null, 2) + '\n');
console.log(JSON.stringify({proofsMade: proofs, cases: Object.fromEntries(Object.entries(cases).map(([k, v]) => [k, v.verdict]))}));
process.exit(0);
