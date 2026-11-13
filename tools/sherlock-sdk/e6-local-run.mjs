// E6: SherlockRun with a hand-made wrap verifier key installed on acceptRun, deployed and called on a LOCAL
// Midnight devnet (compose project e6: node 2.1.101 on 127.0.0.1:9954, indexer 4.4.101 on :8098, proof
// server 9.0.1 on :6340; network id "undeployed"). Port of XC-9's xc9-local-run.mjs (with the fixes of the
// Preview port pv-rehearsal-run.mjs, but not its network). It never talks to a public network.
//
// The wrap key's statement has exactly 2 + TARGET public inputs (default TARGET = 1,024 transcript words,
// 1,026 in all). acceptRun's own transcript is shorter, so each call's guaranteed transcript gets one
// trailing Noop { n = TARGET - words } (or its trailing Noop grows by that much). The JS ledger's
// Transaction.prove adds the Noop's gas (noop_constant + noop_coeff_arg * n) to the declared compute_time,
// exactly as for builder-inserted Noops (ledger prove.rs:253-370), so the runner does not touch gas.
//
// Usage: node e6-local-run.mjs PHASE --vk KEY [--bundle DIR] [--out DIR] [--build DIR] [--sdk DIR] [--target N]
//   PHASE    deploy | dump | call | dry-run
//   KEY      the wrap verifier key: raw MidnightVK bytes or the tagged midnight:verifier-key[v7]: form
//   bundle   run values (bundle.json: stmt, prog_d, init_d, fin_d, steps, result); default the XC-9 bundle
//   out      run directory (default ~/.cache/e6-int/run); build: compiled sherlock-run.compact (Compact 0.36.0)
//   sdk      npm root with the ledger 9.1.0.1 stack and testkit-js 5.0.0 (default ~/.cache/xc9/sdk; not modified)
// Environment:
//   E6_PROVER         prover command: {"binding_input": LE hex, "preimage": hex} on stdin, tagged proof hex on stdout
//   E6_VALIDATE_CALL  native proof-verifying ledger 9 validator (crates/mori-recursion examples/validate_call)
//   E6_DUMMY_PROOF    (dump, dry-run) raw or tagged proof used to finish the tx for the shape checks
//                     (default ~/.cache/e5-wrap/out/lfr-real-k21/wrap.proof)
//   E6_NODE_PORT / E6_INDEXER_PORT / E6_PROOF_PORT  (default 9954 / 8098 / 6340)
// Phases:
//   deploy   MAIN (prog_d, init_d) and OTHER-PROGRAM (prog_d + 1), both with KEY on acceptRun; the indexed
//            state is read back and the installed key compared byte for byte.
//   dump     builds the padded valid call to MAIN, runs E6_PROVER (the dump stub e6-dump-prover.mjs is meant
//            here), then finishes the tx with the dummy proof for the shape checks: JS ledger wellFormed
//            (contract proofs off, limits on) + apply, and validate_call (proofs on: the expected verdict is
//            the proof error and nothing earlier). Submits nothing.
//   call     A (fresh prove, native check, balance, submit, readback), then R1, R2, R3 (XC-9 section 5):
//            R1 result-differs: result r + 1 carrying A's proof (made for r and for another binding): refused.
//            R2 tampered-proof: the saved unbound A with one proof bit flipped, balanced fresh: refused.
//            R3 other-program: a second fresh proof of the run sent to OTHER-PROGRAM (prog_d + 1): refused
//               by the guaranteed transcript's read of program_d.
//            Pre-flight before any prove: MAIN must be on this chain and the run not yet in `runs`; an A that
//            already landed is only read back. Resumes by itself: a proven unbound A (or R3) saved after its
//            prove, never refused, with > 5 min of intent TTL left, is reused instead of proving again. A
//            refused A (native check or node) is marked and the next run proves again (old files moved aside).
//            Done refusals are skipped. Exit 0 = verdict pass (A in, R1..R3 refused, R3 at the program_d read),
//            4 = ran to the end but a verdict failed, 1 = stopped (error). E6_REHEARSE=1 (dummy-prover harness
//            rehearsal only) lets the run continue past a refused A and always exits 0 at the end.
//   dry-run  no devnet: the padding, the stub and the JS shape check against a locally constructed contract.
// The genesis wallet seed is read inside this process from testkit-js (LocalTestEnvironment, index 1) and is
// never written, logged or printed.
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync, mkdirSync, existsSync, symlinkSync, readlinkSync, renameSync} from 'node:fs';
import {resolve, join} from 'node:path';
import {homedir} from 'node:os';
import {pathToFileURL} from 'node:url';
import {spawn} from 'node:child_process';
import {createHash} from 'node:crypto';
import {installedKey, resultWindow, readWindow, callEffects} from './run-layout.mjs';
import {decodePreimage, frameVerifierKey, frameProof, padTranscript, paddedCheckResult, assertNormalized,
  bigToLeHex, leHexToBig} from './e6-padding.mjs';

// ---- arguments
const argv = process.argv.slice(2), phase = argv.shift();
const PHASES = ['deploy', 'dump', 'call', 'dry-run'];
assert.ok(PHASES.includes(phase), `usage: e6-local-run.mjs ${PHASES.join('|')} --vk KEY [--bundle DIR] [--out DIR] [--build DIR] [--sdk DIR] [--target N]`);
const opt = {};
for (let i = 0; i < argv.length; i += 2) { assert.match(argv[i], /^--(vk|bundle|out|build|sdk|target)$/, `unknown option ${argv[i]}`); opt[argv[i].slice(2)] = argv[i + 1]; }
const H = homedir();
assert.ok(opt.vk, '--vk KEY (the wrap verifier key)');
const vkPath = resolve(opt.vk);
const bundleDir = resolve(opt.bundle ?? join(H, '.cache/k21-measure/out-k21-20261111T193356Z'));
const output = resolve(opt.out ?? join(H, '.cache/e6-int/run'));
const build = resolve(opt.build ?? join(H, '.cache/e6-int/build'));
const sdkRoot = resolve(opt.sdk ?? join(H, '.cache/xc9/sdk'));
const TARGET = Number(opt.target ?? 1024);
assert.ok(Number.isInteger(TARGET) && TARGET > 0);
mkdirSync(output, {recursive: true});
const OFFLINE = phase === 'dry-run';
const NET = 'undeployed';
const port = (k, d) => Number(process.env[k] ?? d);
const EP = {indexer: `http://127.0.0.1:${port('E6_INDEXER_PORT', 8098)}/api/v4/graphql`,
  indexerWS: `ws://127.0.0.1:${port('E6_INDEXER_PORT', 8098)}/api/v4/graphql/ws`,
  node: `http://127.0.0.1:${port('E6_NODE_PORT', 9954)}`, nodeWS: `ws://127.0.0.1:${port('E6_NODE_PORT', 9954)}`,
  proofServer: `http://127.0.0.1:${port('E6_PROOF_PORT', 6340)}`};
const NODE_CONTAINER = process.env.E6_NODE_CONTAINER ?? 'e6-node-1';
const sha = b => createHash('sha256').update(b).digest('hex');
const json = x => JSON.stringify(x, (_, v) => typeof v === 'bigint' ? v.toString() : v, 2) + '\n';
const statePath = join(output, OFFLINE ? 'e6-dry-run.json' : 'e6-state.json');
const st = existsSync(statePath) ? JSON.parse(readFileSync(statePath)) : {};
const save = () => writeFileSync(statePath, json(st));
const log = (...a) => console.log(`[${new Date().toISOString()}]`, ...a);
const sleep = ms => new Promise(r => setTimeout(r, ms));
let secretSeed = null; // genesis seed, in memory only
const scrub = s => secretSeed ? String(s).split(secretSeed).join('<seed>') : String(s);
const fail = e => { console.error('e6-local-run failed:', scrub(e?.stack ?? e?.message ?? e)); process.exit(1); };
process.on('unhandledRejection', fail); process.on('uncaughtException', fail);

// ---- SDK: ESM-only packages through shim modules in ~/.cache/e6-int/shims (node_modules -> SDK root).
const shimDir = join(H, '.cache/e6-int/shims'); mkdirSync(shimDir, {recursive: true});
const nm = join(shimDir, 'node_modules');
if (!existsSync(nm)) symlinkSync(join(sdkRoot, 'node_modules'), nm);
assert.equal(resolve(shimDir, readlinkSync(nm)), join(sdkRoot, 'node_modules'), `${nm} points at another SDK`);
const load = async name => {
  const file = join(shimDir, sha(name) + '.mjs');
  if (!existsSync(file)) writeFileSync(file, `import * as m from ${JSON.stringify(name)}; export default m;\n`);
  return (await import(pathToFileURL(file).href)).default;
};
const ledger = await load('@midnight-ntwrk/midnight-js-protocol/v9');
const rt = await load('@midnight-ntwrk/compact-runtime');
if (!existsSync(join(build, 'node_modules'))) symlinkSync(join(sdkRoot, 'node_modules'), join(build, 'node_modules'));
if (!existsSync(join(build, 'package.json'))) writeFileSync(join(build, 'package.json'), '{"type":"module"}\n');
const mod = await import(pathToFileURL(join(build, 'contract/index.js')).href);
const ir = readFileSync(join(build, 'zkir/acceptRun.bzkir'));
const proverKey = readFileSync(join(build, 'keys/acceptRun.prover'));

// ---- the wrap key (framed for the ledger) and the run values
const vkFramed = frameVerifierKey(readFileSync(vkPath));
const vkRaw = new Uint8Array(vkFramed.tagged);
st.key = {path: vkPath, input: vkFramed.input, taggedBytes: vkRaw.length, taggedSHA256: sha(vkRaw),
  midnightVkBytes: vkFramed.body.length, midnightVkSHA256: sha(vkFramed.body), header: 'midnight:verifier-key[v7]: + SCALE length'};
if (st.deploy?.main?.key) assert.equal(st.deploy.main.key.taggedSHA256, st.key.taggedSHA256, 'the deployed key differs from --vk; use a fresh --out');
const bundle = JSON.parse(readFileSync(join(bundleDir, 'bundle.json')));
assert.equal(bundle.schema, 'sherlock-gp-bundle/1');
const le = bigToLeHex, fromLe = leHexToBig;
const be16 = v => new Uint8Array(Buffer.from(v.toString(16).padStart(32, '0'), 'hex'));
const run = {stmt: fromLe(bundle.stmt), p: fromLe(bundle.prog_d), i: fromLe(bundle.init_d), fin: fromLe(bundle.fin_d),
  n: BigInt(bundle.steps), r: BigInt(bundle.result)};
st.run ??= {bundle: bundleDir, prog_d: bundle.prog_d, init_d: bundle.init_d, stmt: bundle.stmt, steps: bundle.steps, result: bundle.result,
  source: 'XC-9 bundle values (bundle.json), as XC-9 did'};
const R0 = 1n, LOC = 'sherlock/acceptRun';
// Intent TTL: E6_TTL_S seconds (default 3,600; the ledger's global_ttl is 1,209,600 s). The E6 call uses 4 h because one
// L, L_Fr, wrap chain takes 20 to 50 min on this host (MEASURED L_Fr prove 2,589 s under load).
const TTL_S = Number(process.env.E6_TTL_S ?? 3600);
if (!(TTL_S > 0 && TTL_S < 1_209_600)) throw new Error('E6_TTL_S out of range');
const ttlOf = () => new Date(Date.now() + TTL_S * 1000);
const view = cs => mod.ledger(rt.ContractState.deserialize(cs.serialize()).data);
const constructed = async (prog, init) => {
  const initial = await new mod.Contract({}).initialState(rt.createConstructorContext({}, '00'.repeat(32)), prog, init, be16(R0));
  const state = ledger.ContractState.deserialize(initial.currentContractState.serialize());
  const op = new ledger.ContractOperation(); op.verifierKey = vkRaw;
  state.setOperation('acceptRun', op);
  assert.deepEqual(state.operations().map(String), ['acceptRun']);
  return state;
};
const child = (cmd, input, logName, env = {}) => new Promise((ok, reject) => {
  const p = spawn('/bin/sh', ['-c', cmd], {stdio: ['pipe', 'pipe', 'pipe'], env: {...process.env, ...env}});
  let out = '', err = '';
  p.stdout.on('data', b => { out += b; }); p.stderr.on('data', b => { err += b; }); p.on('error', reject);
  p.on('close', exit => { writeFileSync(join(output, logName), err); exit === 0 ? ok(out) : reject(Object.assign(Error(`child failed (exit ${exit}): ${logName}`), {exit})); });
  p.stdin.end(input);
});

// ---- network (not in dry-run)
let graphql, rpc, parameters;
if (!OFFLINE) {
  graphql = async query => {
    const r = await fetch(EP.indexer, {method: 'POST', headers: {'content-type': 'application/json'}, body: JSON.stringify({query})});
    assert.ok(r.ok, `indexer ${r.status}`); const body = await r.json();
    assert.ok(!body.errors, 'indexer query failed: ' + JSON.stringify(body.errors)); return body.data;
  };
  rpc = async (method, params = []) => (await (await fetch(EP.node, {method: 'POST', headers: {'content-type': 'application/json'},
    body: JSON.stringify({id: 1, jsonrpc: '2.0', method, params})})).json()).result;
  const chain = {genesisHash: await rpc('chain_getBlockHash', [0]), nodeVersion: await rpc('system_version'),
    specVersion: (await rpc('state_getRuntimeVersion')).specVersion, chain: await rpc('system_chain')};
  assert.equal(chain.chain, 'undeployed1', 'local dev chain only');
  st.chain ??= chain; assert.equal(st.chain.genesisHash, chain.genesisHash, 'same local chain as the deploy');
  st.proofServerVersion = (await (await fetch(EP.proofServer + '/version')).text()).trim();
  assert.ok(st.proofServerVersion.startsWith('9.0.'), 'proof server 9.0.x');
  const {block} = await graphql('{ block { height ledgerParameters } }');
  parameters = ledger.LedgerParameters.deserialize(Buffer.from(block.ledgerParameters, 'hex'));
  st.ledgerParameters = {atHeight: block.height, sha256: sha(Buffer.from(block.ledgerParameters, 'hex'))};
} else parameters = ledger.LedgerParameters.initialParameters();
save();

// ---- the padded acceptRun call
// Build the call prototype twice from the same circuit run and randomness: unpadded (to read its preimage,
// word count and the proof server's skip list) and padded (the one that is proven and sent).
const proto = (target, guaranteed, pd, rand) => new ledger.ContractCallPrototype(target, 'acceptRun', OPERATION, guaranteed, undefined,
  pd.privateTranscriptOutputs, pd.input, pd.output, rand, LOC);
let OPERATION;
class Sentinel extends Error {}
const checkWithServer = async raw => {
  if (OFFLINE && process.env.E6_DRY_CHECK !== 'server') return null; // dry-run: no proof server
  const r = await fetch(EP.proofServer + '/check', {method: 'POST', headers: {'content-type': 'application/octet-stream'},
    body: ledger.createCheckPayload(raw, ir)});
  if (!r.ok) assert.fail(`proof server /check ${r.status}: ${await r.text().catch(() => '')}`);
  return ledger.parseCheckResult(new Uint8Array(await r.arrayBuffer()));
};
const callTx = async (target, against, args, tag) => {
  const now = new Date();
  const context = rt.createCircuitContext({circuitId: 'acceptRun', contractAddress: target, coinPublicKeyOrZswapState: '00'.repeat(32),
    contractState: rt.ContractState.deserialize(against.serialize()), privateState: {}, time: Math.floor(now.getTime() / 1000)});
  const result = await new mod.Contract({}).provableCircuits.acceptRun(context, args.stmt, args.p, args.i, args.n, args.fin, be16(args.r));
  const pd = result.context.callProofDataTrace[0];
  const qc = new ledger.QueryContext(against.data, target);
  qc.block = context.callContext.currentQueryContext.block;
  const [guaranteed, fallible] = ledger.partitionTranscripts([new ledger.PreTranscript(qc, pd.publicTranscript)], parameters)[0];
  assert.ok(guaranteed && !fallible, 'acceptRun must be entirely guaranteed');
  OPERATION = against.operation('acceptRun');
  const rand = ledger.communicationCommitmentRandomness();
  const ttl = ttlOf();
  // 1) unpadded: capture the preimage and the check result, stop before proving.
  let unRaw, skips;
  const unTx = ledger.Transaction.fromParts(NET, undefined, undefined, ledger.Intent.new(ttl).addCall(proto(target, guaranteed, pd, rand)));
  await unTx.prove({
    check: async raw => { unRaw = new Uint8Array(raw); skips = await checkWithServer(raw); return skips ?? Array(guaranteed.program.length).fill(undefined); },
    lookupKey: async () => ({proverKey, verifierKey: vkRaw, ir}),
    prove: async () => { throw new Sentinel('unpadded preimage captured'); },
  }, parameters.transactionCostModel.runtimeCostModel).then(() => assert.fail('unpadded prove must stop'), e => {
    if (!(e instanceof Sentinel) && !String(e?.message ?? e).includes('unpadded preimage captured')) throw e; });
  const un = decodePreimage(unRaw);
  assert.equal(un.privateTranscript.length, 0);
  const words = un.publicTranscriptInputs.length;
  // cross-check the word count with the SDK's own preimage builder on the same program
  const words2 = decodePreimage(ledger.proofDataIntoSerializedPreimage(pd.input, pd.output, guaranteed.program, pd.privateTranscriptOutputs, LOC)).publicTranscriptInputs.length;
  assert.equal(words2, words, 'transcript word count from two preimage builders');
  const pad = padTranscript(guaranteed, words, TARGET);
  const checkResult = paddedCheckResult(skips ?? Array(guaranteed.program.length).fill(undefined), pad.opsAdded);
  const tx = ledger.Transaction.fromParts(NET, undefined, undefined, ledger.Intent.new(ttl).addCall(proto(target, pad.transcript, pd, rand)));
  const info = {tag, target, builtAt: now.toISOString(), ttl: ttl.toISOString(), opsBefore: guaranteed.program.length,
    transcriptWordsBefore: words, noop: {mode: pad.mode, pad: pad.pad, n: pad.noopN}, opsAfter: pad.transcript.program.length,
    lastOpBefore: guaranteed.program.at(-1), checkSkips: skips ? skips.map(s => s ?? null) : 'not asked (dry-run)',
    gasBeforePadding: guaranteed.gas, unpaddedPreimageBytes: unRaw.length};
  return {tx, info, checkResult};
};
const callOf = tx => [...tx.intents.values()].flatMap(i => i.actions).find(a => a instanceof ledger.ContractCall);
const noopGas = n => { try { return ledger.runProgram(new ledger.VmStack(), [{noop: {n}}], parameters.transactionCostModel.runtimeCostModel).gasCost; }
  catch (e) { return 'n/a: ' + e.message; } };
// prove: `fixed` = tagged proof bytes to use as they are; otherwise the prover command `cmd`.
const proveCall = async (built, {fixed, cmd, tag, env}) => {
  let n = 0, proof, proveSeconds = null, proverError = null;
  lastProverExit = null;
  const provider = {
    check: async (_raw, location) => { assert.equal(location, LOC); return built.checkResult; },
    lookupKey: async () => ({proverKey, verifierKey: vkRaw, ir}),
    prove: async (raw, location, binding) => {
      assert.equal(++n, 1); assert.equal(location, LOC);
      const opening = JSON.stringify({binding_input: le(binding), preimage: Buffer.from(raw).toString('hex')});
      writeFileSync(join(output, `${tag}.binding-input`), le(binding) + '\n');
      if (fixed) { proof = fixed; return proof; }
      const t = performance.now();
      try {
        const hex = (await child(cmd, opening, `prover-${tag}.log`, env)).trim();
        proveSeconds = (performance.now() - t) / 1000;
        assert.match(hex, /^([0-9a-f]{2})+$/, 'prover stdout: tagged proof hex');
        proof = new Uint8Array(Buffer.from(hex, 'hex'));
        assert.ok(Buffer.from(proof).toString('latin1').startsWith('midnight:proof['), 'tagged proof');
        writeFileSync(join(output, `${tag}.tagged-proof`), proof);
        return proof;
      } catch (e) { proveSeconds = (performance.now() - t) / 1000; proverError = e; lastProverExit = e.exit ?? null; throw e; }
    },
  };
  const unbound = await tx_prove(built.tx, provider);
  const call = callOf(unbound);
  assertNormalized(call.guaranteedTranscript.program);
  const gasAfter = call.guaranteedTranscript.gas;
  const noopN = built.info.noop.n;
  built.info.gasAfterProve = gasAfter;
  built.info.computeTimeAddedByProve = gasAfter.computeTime - built.info.gasBeforePadding.computeTime;
  built.info.noopGasFromVm = noopN ? noopGas(noopN) : null;
  built.info.programAfterProveOps = call.guaranteedTranscript.program.length;
  built.info.lastOpAfterProve = call.guaranteedTranscript.program.at(-1);
  return {unbound, proof, proveSeconds, proverError};
};
let lastProverExit = null;
const tx_prove = (tx, provider) => tx.prove(provider, parameters.transactionCostModel.runtimeCostModel);

// Local ledger with the deploy applied and the contract's state set to `against`.
const localLedger = (deployAction, against) => {
  let local = ledger.LedgerState.blank(NET); local.parameters = parameters;
  const setup = ledger.Transaction.fromParts(NET, undefined, undefined, ledger.Intent.new(ttlOf()).addDeploy(deployAction)).eraseProofs();
  const lax = Object.assign(new ledger.WellFormedStrictness(), {enforceBalancing: false, verifyNativeProofs: false,
    verifyContractProofs: false, enforceLimits: false, verifySignatures: false});
  const secs = BigInt(Math.floor(Date.now() / 1000));
  const block = {secondsSinceEpoch: secs, secondsSinceEpochErr: 30, parentBlockHash: '00'.repeat(32), lastBlockTime: secs - 6n};
  const [boot, res] = local.apply(setup.wellFormed(local, lax, new Date()), new ledger.TransactionContext(local, block));
  assert.equal(res.type, 'success');
  return {local: boot.updateIndex(deployAction.address, against.data, against.balance), block, secs};
};
const bindUnbound = unbound => ledger.Transaction.deserialize('signature', 'proof', 'pre-binding', unbound.serialize()).bind();
// JS ledger (same Rust well_formed/apply, contract proofs off): shape, normalization, limits, gas and transcript.
const jsShape = (deployAction, against, unbound) => {
  const {local, block} = localLedger(deployAction, against);
  const strict = Object.assign(new ledger.WellFormedStrictness(), {enforceBalancing: false, verifyNativeProofs: false,
    verifyContractProofs: false, enforceLimits: true, verifySignatures: true});
  try {
    const verified = bindUnbound(unbound).wellFormed(local, strict, new Date());
    const [, res] = local.apply(verified, new ledger.TransactionContext(local, block));
    return {wellFormed: true, applied: res.type, error: res.type === 'success' ? null : String(res.error ?? res.type)};
  } catch (e) { return {wellFormed: false, error: String(e.message ?? e)}; }
};
// Native proof-verifying ledger 9 (validate_call): the exact refusal stage and the ledger's public inputs.
const validateCallBin = process.env.E6_VALIDATE_CALL ? resolve(process.env.E6_VALIDATE_CALL) : null;
const nativeValidate = async (deployAction, against, unbound, tag) => {
  if (!validateCallBin) return {skipped: 'E6_VALIDATE_CALL not set'};
  const {local, secs} = localLedger(deployAction, against);
  const r = JSON.parse(await child(validateCallBin, JSON.stringify({state: Buffer.from(local.serialize()).toString('hex'),
    transaction: Buffer.from(bindUnbound(unbound).serialize()).toString('hex'), block_seconds: Number(secs)}), `native-${tag}.log`));
  const pis = r.contractCalls?.[0]?.publicInputs ?? [];
  return {accepted: r.contractProofsSignaturesLimitsAccepted, applied: r.applied, failure: r.contractProfileFailure ?? null,
    allOnFailure: r.allOnFailure ?? null, publicInputs: pis.length, publicInputsLeHex: pis};
};
const deployOf = name => {
  const depTx = ledger.Transaction.deserialize('signature', 'proof', 'binding', readFileSync(join(output, `deploy-${name}.tx`)));
  const deploys = [...depTx.intents.values()].flatMap(i => i.actions).filter(a => a instanceof ledger.ContractDeploy);
  assert.equal(deploys.length, 1); return deploys[0];
};
const proofBody = proof => {
  const p = Buffer.from(proof), tag = p.toString('latin1').match(/^midnight:proof\[v\d+\]:/);
  const b = p[tag[0].length], prefix = [1, 2, 4, (b >> 2) + 5][b & 3];
  return {taggedBytes: p.length, header: tag[0], bodyBytes: p.length - tag[0].length - prefix, bodySHA256: sha(p.subarray(tag[0].length + prefix))};
};
const flipProof = (bytes, proof) => {
  const raw = Buffer.from(bytes), tail = Buffer.from(proof).subarray(-64), at = raw.indexOf(tail);
  assert.ok(at > 0 && raw.indexOf(tail, at + 1) < 0, 'proof bytes located once in the transaction');
  raw[at + 16] ^= 1;
  return {raw, flippedOffset: at + 16};
};

// ---- dump / dry-run: padded call, stub prover, then shape checks with a dummy proof.
if (phase === 'dump' || phase === 'dry-run') {
  let deployAction, against, target;
  if (OFFLINE) {
    against = await constructed(run.p, run.i);
    deployAction = new ledger.ContractDeploy(against); target = deployAction.address;
  } else {
    assert.ok(st.deploy?.main?.blockHeight, 'run the deploy phase first');
    against = ledger.ContractState.deserialize(readFileSync(join(output, 'deploy-main-state.bin')));
    deployAction = deployOf('main'); target = st.deploy.main.address; assert.equal(deployAction.address, target);
  }
  const rec = st[phase] = {at: new Date().toISOString(), target, targetWords: TARGET};
  const built = await callTx(target, against, run, phase);
  rec.call = built.info; save();
  log(`padded call built: ${built.info.transcriptWordsBefore} transcript words, ${built.info.opsBefore} ops, noop ${json(built.info.noop).trim()}`);
  // stub prover (or any E6_PROVER); a non-zero exit is expected from the dump stub
  const dumpFile = process.env.E6_DUMP ?? join(output, phase === 'dump' ? 'call-vector-dump.json' : 'dry-run-call-vector.json');
  if (process.env.E6_PROVER) {
    const p = await proveCall(built, {cmd: process.env.E6_PROVER, tag: `${phase}-prover`, env: {E6_DUMP: dumpFile}}).catch(e => ({proverError: e}));
    rec.prover = {cmd: process.env.E6_PROVER, exit: p.proverError ? lastProverExit : 0, error: p.proverError ? String(p.proverError.message) : null};
    if (existsSync(dumpFile)) {
      const words = JSON.parse(readFileSync(dumpFile)).words;
      rec.dump = {file: dumpFile, total: words.length, lane0: words[0], lane1: words[1],
        trailingZeros: (() => { let k = 0; for (let i = words.length - 1; i >= 2 && words[i] === '0'; i--) k++; return k; })()};
    }
    save();
  }
  // finish the same tx with the dummy proof: shape checks
  const dummy = frameProof(readFileSync(process.env.E6_DUMMY_PROOF ?? join(H, '.cache/e5-wrap/out/lfr-real-k21/wrap.proof')));
  const fin = await proveCall(built, {fixed: new Uint8Array(dummy.tagged), tag: `${phase}-dummy`});
  rec.call = built.info;
  writeFileSync(join(output, `${phase}-dummy-unbound.tx`), fin.unbound.serialize());
  rec.jsShape = jsShape(deployAction, against, fin.unbound);
  const nat = await nativeValidate(deployAction, against, fin.unbound, phase);
  rec.native = {...nat, publicInputsLeHex: undefined};
  if (nat.publicInputsLeHex && rec.dump) {
    const words = JSON.parse(readFileSync(dumpFile)).words;
    const nativeDec = nat.publicInputsLeHex.map(h => fromLe(h).toString());
    const diff = nativeDec.map((v, i) => v === words[i] ? null : i).filter(i => i !== null);
    rec.dumpEqualsLedgerPublicInputs = {equal: diff.length === 0 && nativeDec.length === words.length, lanesDiffering: diff.slice(0, 10),
      ledgerLength: nativeDec.length, dumpLength: words.length};
  }
  rec.shapeVerdict = {jsLedgerAcceptsShapeAndApplies: rec.jsShape.wellFormed && rec.jsShape.applied === 'success',
    nativeFailsOnlyAtProof: nat.skipped ? null : (!nat.accepted && /proof/i.test(nat.failure ?? '') && !/normali|exceed|public input/i.test(nat.failure ?? '')),
    nativePublicInputs: nat.publicInputs ?? null};
  save(); log(`${phase}: ${json(rec)}`);
  process.exit(0);
}

// ---- wallet (deploy, call): the local dev-preset genesis wallet; the seed stays in this process.
const {WalletTransaction, ProtocolVersion, DefaultForkSchedule} = await load('@midnightntwrk/wallet-sdk');
process.chdir(join(H, '.cache/e6-int')); // testkit's logger writes ./logs/tests; keep it out of the worktree
const testkit = await load('@midnight-ntwrk/testkit-js');
const {PolkadotNodeClient, makeConfig} = await load('@midnightntwrk/wallet-sdk-node-client');
const {SerializedTransaction} = await load('@midnightntwrk/wallet-sdk-abstractions');
const Rx = await load('rxjs');
const {WebSocket} = await load('ws'); globalThis.WebSocket ??= WebSocket;
testkit.logger.level = 'warn';
{
  const seeds = new testkit.LocalTestEnvironment(testkit.logger).genesisMintWalletSeed;
  assert.ok(Array.isArray(seeds) && /^[0-9a-f]{64}$/.test(seeds[1] ?? '') && seeds[1].endsWith('1'), 'testkit dev-preset genesis seed (index 1)');
  secretSeed = seeds[1];
}
const openWallet = async () => {
  const env = {walletNetworkId: NET, networkId: NET, ...EP, faucet: undefined};
  const {wallet, seeds, keystore} = await testkit.FluentWalletBuilder.forEnvironment(env)
    .withDustOptions({ledgerParams: ledger.LedgerParameters.initialParameters(), additionalFeeOverhead: 1_000n, feeBlocksMargin: 5})
    .withSeed(secretSeed).buildWithoutStarting();
  await wallet.start(seeds);
  const s = await Rx.firstValueFrom(wallet.state().pipe(
    Rx.filter(s => [s.shielded, s.unshielded, s.dust].every(w => w.progress?.isStrictlyComplete?.()) && s.dust.availableCoins.length > 0),
    Rx.timeout({first: 600_000})));
  log(`wallet synced: dust coins ${s.dust.availableCoins.length}, dust balance ${s.dust.balance(new Date())}`);
  const active = async () => (await Rx.firstValueFrom(wallet.state())).activeProtocolVersion;
  const unwrap = async handle => {
    const r = WalletTransaction.unwrapWithin(handle, ProtocolVersion.epochOf(await active(), DefaultForkSchedule.v9));
    if (r._tag === 'Left') throw r.left; return r.right;
  };
  const finalize = async recipe => {
    const signed = await wallet.signRecipe(recipe, p => keystore.signDataAsync(p));
    const handle = await wallet.finalizeRecipe(signed);
    return {handle, tx: await unwrap(handle)};
  };
  return {wallet, finalize,
    dustBalance: async () => (await Rx.firstValueFrom(wallet.state())).dust.balance(new Date()),
    balanceUnproven: async tx => finalize(await wallet.balanceUnprovenTransaction(WalletTransaction.adopt('Unproven', tx, await active()), {ttl: ttlOf()})),
    balanceUnbound: async tx => finalize(await wallet.balanceUnboundTransaction(WalletTransaction.adopt('Unbound', tx, await active()), {ttl: ttlOf()}))};
};
// Raw node submission: every accept/refuse verdict is the node's (pool validation), with its log lines.
const node = await PolkadotNodeClient.init(makeConfig({nodeURL: new URL(EP.nodeWS)}));
const nodeLogSince = since => new Promise(ok => {
  const p = spawn('docker', ['logs', '--since', since, NODE_CONTAINER], {stdio: ['ignore', 'pipe', 'pipe']});
  let o = ''; p.stdout.on('data', b => { o += b; }); p.stderr.on('data', b => { o += b; }); p.on('close', () => ok(o));
});
st.submissions ??= {};
const submit = async (bytes, tag, label) => {
  const t = performance.now(), startedAt = new Date().toISOString();
  writeFileSync(join(output, `submitted-${tag}.tx`), bytes);
  st.submissions[tag] = {label, startedAt, verdict: 'sending', txBytes: bytes.length, txSHA256: sha(bytes)}; save();
  let r;
  try {
    const ev = await Promise.race([node.sendMidnightTransactionAndWait(SerializedTransaction.of(new Uint8Array(bytes)), 'InBlock'),
      new Promise((_, no) => setTimeout(() => no(Error('E6 timeout: no InBlock event within 150 s')), 150_000))]);
    r = {verdict: 'included', txHash: ev.txHash, blockHash: ev.blockHash, blockHeight: ev.blockHeight};
  } catch (e) {
    const chainOf = []; for (let x = e; x; x = x.cause) chainOf.push(scrub(x.message ?? x));
    r = {verdict: chainOf[0].startsWith('E6 timeout') ? 'not-included-timeout' : 'refused-by-node', error: chainOf};
  }
  Object.assign(st.submissions[tag], r, {seconds: (performance.now() - t) / 1000});
  await sleep(2000);
  const logs = await nodeLogSince(startedAt);
  writeFileSync(join(output, `node-log-${tag}.txt`), logs);
  st.submissions[tag].nodeLogLines = logs.split('\n').filter(l => /error|invalid|reject|fail|malformed|verif|mismatch/i.test(l)).slice(0, 20);
  save(); log(`submit ${tag}: ${json(st.submissions[tag])}`);
  return st.submissions[tag];
};
const waitAction = async (address, typename, {skipHash, ms = 180_000} = {}) => {
  const t = Date.now();
  for (;;) {
    const d = await graphql(`{contractAction(address:"${address}"){__typename state
      transaction{hash id block{height hash timestamp} ... on RegularTransaction{fee transactionResult{status segments{id success}}}}
      ... on ContractCall{entryPoint}}}`).catch(() => ({}));
    const a = d.contractAction;
    if (a && (!typename || a.__typename === typename) && a.transaction.hash !== skipHash) return a;
    assert.ok(Date.now() - t < ms, `indexer has no ${typename ?? 'action'} for ${address}`);
    await sleep(2000);
  }
};

if (phase === 'deploy') {
  const w = await openWallet();
  st.deploy ??= {};
  for (const [name, prog] of [['main', run.p], ['other-program', run.p + 1n]]) {
    if (st.deploy[name]?.blockHeight) { log(`deploy ${name} done`); continue; }
    const t0 = performance.now();
    let address, bytes, sub;
    if (st.deploy[name]?.submit?.verdict === 'included') { // resume: submitted, readback pending
      bytes = readFileSync(join(output, `deploy-${name}.tx`)); address = st.deploy[name].address; sub = st.deploy[name].submit;
    } else {
      const d = new ledger.ContractDeploy(await constructed(prog, run.i)); address = d.address;
      const tx = ledger.Transaction.fromParts(NET, undefined, undefined, ledger.Intent.new(ttlOf()).addDeploy(d));
      const fin = await w.balanceUnproven(tx);
      bytes = fin.tx.serialize();
      writeFileSync(join(output, `deploy-${name}.tx`), bytes);
      sub = await submit(bytes, `deploy-${name}`, `deploy ${name}`);
      st.deploy[name] = {address, submit: sub, programD: le(prog), initD: le(run.i)}; save();
    }
    assert.equal(sub.verdict, 'included', `deploy ${name}`);
    const action = await waitAction(address, 'ContractDeploy');
    const indexedState = Buffer.from(action.state, 'hex');
    writeFileSync(join(output, `deploy-${name}-state.bin`), indexedState);
    const cs = ledger.ContractState.deserialize(indexedState);
    const installed = Buffer.from(cs.operation('acceptRun').verifierKey);
    assert.ok(installed.equals(Buffer.from(vkRaw)), 'indexed installed acceptRun key equals the framed --vk bytes');
    const v = view(cs);
    assert.equal(v.program_d, prog); assert.equal(v.init_d, run.i); assert.equal(v.steps, 0n); assert.ok(v.runs.isEmpty());
    assert.deepEqual(Buffer.from(v.result), Buffer.from(be16(R0)));
    st.deploy[name] = {...st.deploy[name], txHash: sub.txHash, nodeBlockHeight: sub.blockHeight, blockHash: sub.blockHash,
      blockHeight: action.transaction.block.height, indexedTxHash: action.transaction.hash, txId: action.transaction.id,
      status: action.transaction.transactionResult?.status, paidFees: action.transaction.fee, txBytes: bytes.length,
      stateBytes: indexedState.length, stateSHA256: sha(indexedState), key: {...st.key, installedEqualsFramed: true,
        installedSHA256: sha(installed)},
      installedKeyLayout: (() => { try { return installedKey(indexedState, vkRaw); } catch (e) { return {error: e.message}; } })(),
      wallSeconds: (performance.now() - t0) / 1000};
    save(); log(`deploy ${name}: ${json(st.deploy[name])}`);
  }
  await w.wallet.stop(); await node.close(); process.exit(0);
}

// ---- call phase: A, then R1, R2, R3.
const proverCmd = process.env.E6_PROVER; assert.ok(proverCmd, 'E6_PROVER');
const main = st.deploy?.main, other = st.deploy?.['other-program'];
assert.ok(main?.blockHeight && other?.blockHeight, 'run the deploy phase first');
const preMain = ledger.ContractState.deserialize(readFileSync(join(output, 'deploy-main-state.bin')));
const otherPre = ledger.ContractState.deserialize(readFileSync(join(output, 'deploy-other-program-state.bin')));
const deployMain = deployOf('main'), deployOther = deployOf('other-program');
st.calls ??= {};
let w = null;
const wallet = async () => (w ??= await openWallet());
const REHEARSE = process.env.E6_REHEARSE === '1'; // harness rehearsal with a dummy prover only
const ttlLeftMin = iso => (Date.parse(iso) - Date.now()) / 60_000;
// A saved unbound tx is resumable only when it was saved after its prove finished, was never refused, and its
// intent TTL has more than 5 min left.
const savedUnbound = (file, info) => existsSync(join(output, file)) && info?.unboundSavedAt && !info.refused && info.ttl && ttlLeftMin(info.ttl) > 5
  ? ledger.Transaction.deserialize('signature', 'proof', 'pre-binding', readFileSync(join(output, file))) : null;
// Move earlier artifacts aside before a fresh prove, so a stale unbound tx or proof is never picked up.
const archive = (...files) => { const sfx = new Date().toISOString().replace(/[:.]/g, ''); for (const f of files)
  if (existsSync(join(output, f))) renameSync(join(output, f), join(output, `${f}.old-${sfx}`)); };
const latestAction = async address => (await graphql(`{contractAction(address:"${address}"){__typename state transaction{hash}}}`)).contractAction;
const markRefused = (why) => { st.calls.valid = {...(st.calls.valid ?? {}), refused: why}; (st.calls.refusedA ??= []).push({why, at: new Date().toISOString()}); save(); };

// A: pre-flight on MAIN, fresh proof (saved unbound at once), native check, balance, submit, readback.
if (!st.calls.accept?.readback && !st.calls.accept?.rehearsalRefused) {
  // Pre-flight before any prove: MAIN must exist on this chain; if a ContractCall already landed, read it back.
  let latest = await latestAction(main.address);
  assert.ok(latest, `MAIN ${main.address} is not on this chain (devnet restarted?); redeploy into a fresh --out`);
  if (latest.__typename !== 'ContractCall' && ['sending', 'included'].includes(st.submissions.A?.verdict)) {
    log(`A was ${st.submissions.A.verdict}; waiting up to 60 s for the indexer`);
    for (let k = 0; k < 30 && latest.__typename !== 'ContractCall'; k++) { await sleep(2000); latest = await latestAction(main.address); }
    if (latest.__typename !== 'ContractCall') markRefused(`A ${st.submissions.A.verdict} but not indexed`);
  }
  if (latest.__typename !== 'ContractCall') {
    assert.ok(!view(ledger.ContractState.deserialize(Buffer.from(latest.state, 'hex'))).runs.member(run.stmt), 'run already accepted');
    let unbound = savedUnbound('valid-unbound.tx', st.calls.valid);
    if (unbound) log(`resumed the proven unbound A (TTL ${ttlLeftMin(st.calls.valid.ttl).toFixed(1)} min left)`);
    else {
      archive('valid-unbound.tx', 'valid.tagged-proof', 'call-valid.tx');
      log('proving A (the valid call)');
      const built = await callTx(main.address, preMain, run, 'valid');
      st.calls.valid = built.info; save();
      const p = await proveCall(built, {cmd: proverCmd, tag: 'valid'});
      unbound = p.unbound;
      writeFileSync(join(output, 'valid-unbound.tx'), unbound.serialize());
      st.calls.valid = {...built.info, proof: proofBody(p.proof), proveSecondsEndToEnd: p.proveSeconds, unboundSavedAt: new Date().toISOString()};
      save();
    }
    st.calls.nativeValid = {...(await nativeValidate(deployMain, preMain, unbound, 'valid')), publicInputsLeHex: undefined};
    st.calls.jsShapeValid = jsShape(deployMain, preMain, unbound); save();
    if (!st.calls.nativeValid.skipped && st.calls.nativeValid.accepted !== true && !REHEARSE) {
      markRefused(`native: ${st.calls.nativeValid.failure}`);
      assert.fail(`A refused by the native ledger (${st.calls.nativeValid.failure}); nothing submitted; a rerun proves again`);
    }
    const ww = await wallet();
    const fin = await ww.balanceUnbound(unbound);
    st.calls.walletFeeEstimate = await ww.wallet.calculateTransactionFee(fin.handle).then(String).catch(e => 'n/a: ' + e.message);
    const bytes = fin.tx.serialize();
    writeFileSync(join(output, 'call-valid.tx'), bytes);
    const sub = await submit(bytes, 'A', 'A valid acceptRun');
    if (sub.verdict !== 'included') {
      await ww.wallet.revert(fin.handle, 'E6 A not included').catch(e => log('revert A: ' + e.message));
      markRefused(`node: ${sub.verdict}`);
      if (REHEARSE) { st.calls.accept = {readback: false, rehearsalRefused: sub.verdict}; save(); }
      else assert.fail(`A not included (${sub.verdict}); see node-log-A.txt; a rerun proves again`);
    }
  }
}
if (st.calls.accept?.rehearsalRefused) log('rehearsal: A refused as expected with a dummy proof; continuing to R1..R3');
else if (!st.calls.accept?.readback) {
  const action = await waitAction(main.address, 'ContractCall');
  assert.equal(action.entryPoint, 'acceptRun');
  const post = Buffer.from(action.state, 'hex');
  writeFileSync(join(output, 'post-main-state.bin'), post);
  const cs = ledger.ContractState.deserialize(post), v = view(cs);
  const deployState = readFileSync(join(output, 'deploy-main-state.bin'));
  const layout = resultWindow(deployState, post, R0, run.r);
  const goodBytes = readFileSync(join(output, 'call-valid.tx'));
  const goodTx = ledger.Transaction.deserialize('signature', 'proof', 'binding', goodBytes);
  st.calls.accept = {readback: true, indexedTxHash: action.transaction.hash, txId: action.transaction.id,
    blockHeight: action.transaction.block.height, status: action.transaction.transactionResult?.status,
    segments: action.transaction.transactionResult?.segments, paidFees: action.transaction.fee, txBytes: goodBytes.length,
    ledgerFeeOfBytes: goodTx.fees(parameters),
    resultHex: Buffer.from(v.result).toString('hex'), steps: v.steps, stmtInRuns: v.runs.member(run.stmt),
    keyUnchanged: Buffer.from(cs.operation('acceptRun').verifierKey).equals(Buffer.from(vkRaw)),
    layout, readWindowDeploy: readWindow(deployState, layout), readWindowPost: readWindow(post, layout),
    effects: callEffects(callOf(goodTx)), postStateBytes: post.length, postStateSHA256: sha(post)};
  assert.deepEqual(Buffer.from(v.result), Buffer.from(be16(run.r))); assert.equal(v.steps, run.n); assert.ok(v.runs.member(run.stmt));
  save(); log(`A: ${json(st.calls.accept)}`);
}
const ww = await wallet();
await sleep(15_000); await ww.wallet.waitForSyncedState();

// One refusal: native check, wallet balancing, submit; a refusal reverts the wallet's booked DUST.
const refusal = async (tag, label, unbound, deployAction, against, address) => {
  const rec = st.calls[tag] = {...(st.calls[tag] ?? {}), label};
  rec.native = {...(await nativeValidate(deployAction, against, unbound, tag)), publicInputsLeHex: undefined}; save();
  let fin;
  try { fin = await ww.balanceUnbound(unbound); }
  catch (e) { rec.wallet = {passed: false, error: scrub(e?.message ?? e)}; rec.done = true; save(); return rec; }
  const sub = await submit(fin.tx.serialize(), tag, label);
  rec.submit = sub;
  if (sub.verdict !== 'included') await ww.wallet.revert(fin.handle, 'E6 negative control').catch(e => log(`revert ${tag}: ${e.message}`));
  else rec.indexed = await waitAction(address, 'ContractCall', {skipHash: st.calls.accept.indexedTxHash, ms: 60_000}).catch(e => String(e.message));
  rec.done = true; save(); log(`${tag}: ${json(rec)}`);
  return rec;
};
const validProof = () => new Uint8Array(readFileSync(join(output, 'valid.tagged-proof')));
// R1 result-differs: result r + 1, carrying A's proof (made for r and for A's binding), against the deploy state.
if (!st.calls.r1?.done) {
  const built = await callTx(main.address, preMain, {...run, r: run.r + 1n}, 'r1');
  const bad = await proveCall(built, {fixed: validProof(), tag: 'r1'});
  st.calls.r1 = {call: built.info};
  await refusal('r1', 'R1 result-differs (result r + 1, reused proof of r)', bad.unbound, deployMain, preMain, main.address);
}
// R2 tampered-proof: the saved unbound A with one proof bit flipped, balanced fresh (A's own bytes are spent).
if (!st.calls.r2?.done) {
  const {raw, flippedOffset} = flipProof(readFileSync(join(output, 'valid-unbound.tx')), validProof());
  const tampered = ledger.Transaction.deserialize('signature', 'proof', 'pre-binding', raw);
  st.calls.r2 = {flippedOffsetInUnbound: flippedOffset};
  await refusal('r2', 'R2 tampered-proof (one proof bit flipped)', tampered, deployMain, preMain, main.address);
}
// R3 other-program: a second fresh proof of the run (prog_d), sent to OTHER-PROGRAM (prog_d + 1). The call is
// computed against MAIN's deploy state, so its guaranteed transcript reads program_d = prog_d.
if (!st.calls.r3?.done) {
  let unbound = savedUnbound('r3-unbound.tx', st.calls.r3?.call);
  if (unbound) log('resumed the proven unbound R3');
  else {
    archive('r3-unbound.tx', 'r3.tagged-proof');
    log('proving R3 (the other-program call)');
    const built = await callTx(other.address, preMain, run, 'r3');
    st.calls.r3 = {call: built.info}; save();
    const p = await proveCall(built, {cmd: proverCmd, tag: 'r3'});
    unbound = p.unbound;
    writeFileSync(join(output, 'r3-unbound.tx'), unbound.serialize());
    st.calls.r3 = {call: {...built.info, unboundSavedAt: new Date().toISOString()}, proof: proofBody(p.proof), proveSecondsEndToEnd: p.proveSeconds}; save();
  }
  await refusal('r3', 'R3 other-program (valid proof of prog_d; the contract holds prog_d + 1)', unbound, deployOther, otherPre, other.address);
  const after = view(ledger.ContractState.deserialize(Buffer.from((await waitAction(other.address)).state, 'hex')));
  st.calls.r3.otherProgramAfter = {result: Buffer.from(after.result).toString('hex'), stmtInRuns: after.runs.member(run.stmt), steps: after.steps};
  save();
}
// Verdict (XC-9 section 5): A included and read back; R1, R2 refused by the node; R3 refused by the node at the
// transcript (native: proof accepted, not applied; node log: the program_d read mismatch).
const sub = t => st.submissions[t] ?? {};
const r3Log = (sub('r3').nodeLogLines ?? []).join('\n');
st.verdict = {A: st.calls.accept?.readback === true && st.calls.accept?.stmtInRuns === true,
  R1: sub('r1').verdict === 'refused-by-node', R2: sub('r2').verdict === 'refused-by-node',
  R3: sub('r3').verdict === 'refused-by-node' && st.calls.r3?.native?.accepted === true && st.calls.r3?.native?.applied === false
    && /mismatch/i.test(r3Log), rehearsal: REHEARSE};
st.verdict.pass = st.verdict.A && st.verdict.R1 && st.verdict.R2 && st.verdict.R3;
st.done = new Date().toISOString(); save();
log(`verdict: ${json(st.verdict)}`);
await ww.wallet.stop(); await node.close();
process.exit(st.verdict.pass || REHEARSE ? 0 : 4);
