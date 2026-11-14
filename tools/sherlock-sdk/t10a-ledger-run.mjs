// T10a: SherlockLedger (sherlock-ledger.compact) with the universal wrap verifier key installed on all three entries
// (acceptRun, acceptCollection, openClaim), deployed and called on a LOCAL Midnight devnet (compose project e6: node
// on 127.0.0.1:9954, indexer :8098, proof server :6340; network id "undeployed"). Forked from e6-local-run.mjs, with
// its safety rules: the chain must be `undeployed1`, the genesis seed stays in this process (scrubbed from every
// message), state is resumable, the intent TTL comes from T10A_TTL_S (default 14,400 s, owner ruling O6). It never
// talks to a public network.
//
// The wrap key's statement has 2 + TARGET public inputs (default TARGET = 1,024 transcript words, 1,026 in all).
// acceptRun's own transcript has 40 words (42 lanes), so the call's guaranteed transcript gets one trailing
// Noop { n = TARGET - 40 }; Transaction.prove adds the Noop's gas to the declared compute_time (e6-padding.mjs).
//
// Usage: node t10a-ledger-run.mjs PHASE --vk KEY [--args FILE] [--forbid FILE] [--out DIR] [--build DIR] [--sdk DIR] [--target N]
//   PHASE    deploy | dump | call | dry-run
//   KEY      the wrap verifier key: raw MidnightVK bytes or the tagged midnight:verifier-key[v7]: form
//   args     JSON {c_p, input_pub, outcome_pub, anchor?, reg_pub?}: decimal or 0x-hex Fr strings. anchor defaults
//            to enc1.C_P (LEAD pick), reg_pub to VALUES.txt coll.reg_pub; c_p, input_pub, outcome_pub default to
//            VALUES.txt coll.run[0] (dump, call, dry-run need them; deploy needs only anchor and reg_pub)
//   forbid   JSON list of decimal Fr words (or {label, value}) that must not appear in the included call vector or
//            the indexed state (prog_d words, pack(prog_d), n_steps); default: none (the scan still runs its control)
//   forbid-small  T10: JSON list of {label, value} words below 2^16 (n_steps, h_hi limbs, r_P words). A small word
//            can equal a template constant of the contract, so each hit on the call vector is judged by its lane: it
//            passes only on a lane whose word is a constant of the compiled acceptRun (contract/lanes.json of T9),
//            i.e. a lane that no run can change
//   out      run directory (default ~/.cache/t9/devnet); build: sherlock-ledger-build.sh output
//            (default ~/.cache/t9/contract/build/sherlock-ledger, compiled with keys)
//   sdk      npm root with the ledger 9.1.0.1 stack and testkit-js (default ~/.cache/xc9/sdk; not modified)
// Environment:
//   T10A_PROVER        prover command. stdin: {"binding_input": "<32-byte LE hex>", "preimage": "<hex of the tagged
//                      ProofPreimage>"}; stdout: the tagged proof as hex (midnight:proof[v5]: + SCALE length + body),
//                      exit 0. The call vector it must prove is [binding_input, comm] ++ the preimage's public
//                      transcript inputs (e6-padding.mjs callVector). Default for dump and dry-run: the dump stub
//                      e6-dump-prover.mjs (writes <out>/vector.json, exits 3). Required for call.
//   T10A_VALIDATE_CALL or E6_VALIDATE_CALL   native proof-verifying ledger 9 validator (mori-recursion validate_call)
//   T10A_DUMMY_PROOF   (dump, dry-run) proof used to finish the tx for the shape checks
//                      (default ~/.cache/e5-wrap/out/lfr-real-k21/wrap.proof)
//   T10A_TTL_S         intent TTL in seconds (default 14,400)
//   T10A_R3_FRESH=1    R3 proves the replay afresh with T10A_PROVER (default: A's proof with the fresh binding)
//   E6_NODE_PORT / E6_INDEXER_PORT / E6_PROOF_PORT / E6_NODE_CONTAINER  (default 9954 / 8098 / 6340 / e6-node-1)
// Phases:
//   deploy   one SherlockLedger, constructor (anchor, reg_pub), KEY on all three entries; the indexed state is read
//            back and each installed key compared byte for byte with the framed --vk.
//   dump     builds the padded acceptRun call for (c_p, input_pub, outcome_pub) against the deployed state, runs the
//            prover (the dump stub by default) so <out>/vector.json holds the 1,026-word call vector, locates
//            claim_pub, outcome_pub and the anchor in it, then finishes the tx with the dummy proof for the JS shape
//            check (and validate_call, if given: expected verdict is the proof error, with the ledger's public
//            inputs compared to vector.json). Submits nothing.
//   call     A: pre-flight (contract on this chain, claim not in runs), fresh prove, native check, balance, submit,
//               inclusion, indexer readback of runs[claim_pub] = outcome_pub, then the privacy scan of A's call
//               vector, A's included transcript and the indexed state against --forbid, with control words that
//               are present (claim_pub, outcome_pub, anchor) to show the scanner fires.
//            R1 outcome-differs: outcome_pub + 1 carrying A's proof: refused.
//            R2 tampered-proof: the saved unbound A with one proof bit flipped: refused.
//            R4 (T10, with T10A_R4_ARGS and T10A_R4_PROOF): the acceptRun arguments of program B with B's own
//               wrap proof (tagged midnight:proof bytes) against this ledger, whose anchor is C_L of program A: refused.
//            R3 replay: the same claim again after A landed (fresh binding; A's proof, or a fresh one with
//               T10A_R3_FRESH=1): refused. The local circuit run against the post-A state is recorded too (it
//               refuses with "claim already settled": the contract-level evidence when the node refuses at the proof).
//            Resumes like e6-local-run.mjs: a proven unbound A saved after its prove, never refused, with > 5 min of
//            TTL left is reused; done refusals are skipped. Exit 0 = verdict pass, 4 = a verdict failed, 1 = error.
//   dry-run  no devnet: the padding, the stub, the JS shape check and the scan against a locally constructed contract.
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync, mkdirSync, existsSync, symlinkSync, readlinkSync, renameSync} from 'node:fs';
import {resolve, join, dirname} from 'node:path';
import {homedir} from 'node:os';
import {pathToFileURL, fileURLToPath} from 'node:url';
import {spawn} from 'node:child_process';
import {createHash} from 'node:crypto';
import {decodePreimage, callVector, frameVerifierKey, frameProof, padTranscript, paddedCheckResult, assertNormalized,
  bigToLeHex, leHexToBig, FR_MODULUS} from './e6-padding.mjs';
import {nestedWords, contractStateWords, scan, surfaceSizes} from './sherlock-ledger-scan.mjs';

const here = dirname(fileURLToPath(import.meta.url));
const ENTRIES = ['acceptCollection', 'acceptRun', 'openClaim'];
// ---- arguments
const argv = process.argv.slice(2), phase = argv.shift();
const PHASES = ['deploy', 'dump', 'call', 'dry-run'];
const USAGE = `usage: t10a-ledger-run.mjs ${PHASES.join('|')} --vk KEY [--args FILE] [--forbid FILE] [--forbid-small FILE] [--out DIR] [--build DIR] [--sdk DIR] [--target N]`;
assert.ok(PHASES.includes(phase), USAGE);
const opt = {};
for (let i = 0; i < argv.length; i += 2) { assert.match(argv[i], /^--(vk|args|forbid|forbid-small|out|build|sdk|target)$/, `unknown option ${argv[i]}; ${USAGE}`); opt[argv[i].slice(2)] = argv[i + 1]; }
const H = homedir();
assert.ok(opt.vk, '--vk KEY (the wrap verifier key)');
const vkPath = resolve(opt.vk);
const output = resolve(opt.out ?? join(H, '.cache/t9/devnet'));
const build = resolve(opt.build ?? join(H, '.cache/t9/contract/build/sherlock-ledger'));
const sdkRoot = resolve(opt.sdk ?? join(H, '.cache/xc9/sdk'));
const TARGET = Number(opt.target ?? 1024);
assert.ok(Number.isInteger(TARGET) && TARGET > 0);
mkdirSync(output, {recursive: true});
const OFFLINE = phase === 'dry-run';
// T10: T10_NET=preview runs the same phases on Midnight Preview (owner ruling 2026-11-14), guarded by the pinned
// Preview genesis and the pinned wallet address (OP-PV-WALLET); otherwise the local devnet only.
const PREVIEW = process.env.T10_NET === 'preview';
const NET = PREVIEW ? 'preview' : 'undeployed';
const PREVIEW_GENESIS = '0x3c096de209e06a1a8c52be7bda109ee8c891a42288c0a9db5495f9616dd13796';
const PREVIEW_WALLET = 'mn_addr_preview1qdqmf7mv8n673fm0r868vyxzkfcnjepmsnyayhxc54vqvyg4l87s5dwena';
const port = (k, d) => Number(process.env[k] ?? d);
const EP = PREVIEW ? {indexer: 'https://indexer.preview.midnight.network/api/v4/graphql',
  indexerWS: 'wss://indexer.preview.midnight.network/api/v4/graphql/ws',
  node: 'https://rpc.preview.midnight.network', nodeWS: 'wss://rpc.preview.midnight.network',
  proofServer: `http://127.0.0.1:${port('E6_PROOF_PORT', 6340)}`}
  : {indexer: `http://127.0.0.1:${port('E6_INDEXER_PORT', 8098)}/api/v4/graphql`,
  indexerWS: `ws://127.0.0.1:${port('E6_INDEXER_PORT', 8098)}/api/v4/graphql/ws`,
  node: `http://127.0.0.1:${port('E6_NODE_PORT', 9954)}`, nodeWS: `ws://127.0.0.1:${port('E6_NODE_PORT', 9954)}`,
  proofServer: `http://127.0.0.1:${port('E6_PROOF_PORT', 6340)}`};
const NODE_CONTAINER = process.env.E6_NODE_CONTAINER ?? 'e6-node-1';
const sha = b => createHash('sha256').update(b).digest('hex');
const json = x => JSON.stringify(x, (_, v) => typeof v === 'bigint' ? v.toString() : v, 2) + '\n';
const statePath = join(output, OFFLINE ? 't10a-dry-run.json' : 't10a-state.json');
const st = existsSync(statePath) ? JSON.parse(readFileSync(statePath)) : {};
const save = () => writeFileSync(statePath, json(st));
const log = (...a) => console.log(`[${new Date().toISOString()}]`, ...a);
const sleep = ms => new Promise(r => setTimeout(r, ms));
let secretSeed = null; // genesis seed (devnet) or the Preview phrase, in memory only
let secretHex = null; // the Preview master seed hex, in memory only
const scrub = s => { let o = String(s); for (const x of [secretSeed, secretHex]) if (x) o = o.split(x).join('<seed>'); return o; };
const fail = e => { console.error('t10a-ledger-run failed:', scrub(e?.stack ?? e?.message ?? e)); process.exit(1); };
process.on('unhandledRejection', fail); process.on('uncaughtException', fail);

// ---- SDK: ESM-only packages through shim modules in ~/.cache/t9/contract/shims (node_modules -> SDK root).
const shimDir = join(H, '.cache/t9/contract/shims'); mkdirSync(shimDir, {recursive: true});
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
const buildRoot = dirname(build);
if (!existsSync(join(buildRoot, 'node_modules'))) symlinkSync(join(sdkRoot, 'node_modules'), join(buildRoot, 'node_modules'));
if (!existsSync(join(buildRoot, 'package.json'))) writeFileSync(join(buildRoot, 'package.json'), '{"type":"module"}\n');
const mod = await import(pathToFileURL(join(build, 'contract/index.js')).href);
const ir = readFileSync(join(build, 'zkir/acceptRun.bzkir'));
const proverKey = readFileSync(join(build, 'keys/acceptRun.prover'));

// ---- the wrap key (framed for the ledger) and the call values
const vkFramed = frameVerifierKey(readFileSync(vkPath));
const vkRaw = new Uint8Array(vkFramed.tagged);
st.key = {path: vkPath, input: vkFramed.input, taggedBytes: vkRaw.length, taggedSHA256: sha(vkRaw),
  midnightVkBytes: vkFramed.body.length, midnightVkSHA256: sha(vkFramed.body), header: 'midnight:verifier-key[v7]: + SCALE length',
  installedOn: ENTRIES};
if (st.deploy?.key) assert.equal(st.deploy.key.taggedSHA256, st.key.taggedSHA256, 'the deployed key differs from --vk; use a fresh --out');
const fr = (x, name) => { const v = BigInt(x); assert.ok(v >= 0n && v < FR_MODULUS, `${name}: not a canonical Fr`); return v; };
const VALUES = Object.fromEntries(readFileSync(join(here, '../../research/sherlock-enc-fp3-20261112/fp3-closure/VALUES.txt'), 'utf8')
  .split('\n').filter(l => l && !l.startsWith('#')).map(l => l.split(' = ')).map(([k, v]) => [k, v]));
const argsIn = opt.args ? JSON.parse(readFileSync(resolve(opt.args), 'utf8')) : {};
const A_ARGS = {
  anchor: fr(argsIn.anchor ?? '0x65f6874b0be5aaa5a42b2e3b6b94ee9adf10dad60d7eff1211c65e3331ff2994', 'anchor'),
  reg_pub: fr(argsIn.reg_pub ?? VALUES['coll.reg_pub'], 'reg_pub'),
  c_p: fr(argsIn.c_p ?? VALUES['coll.run[0].c_p'], 'c_p'),
  input_pub: fr(argsIn.input_pub ?? VALUES['coll.run[0].input_pub'], 'input_pub'),
  outcome_pub: fr(argsIn.outcome_pub ?? VALUES['coll.run[0].outcome_pub'], 'outcome_pub')};
assert.notEqual(A_ARGS.anchor, 0n, 'zero anchor');
const claimPub = rt.transientHash(new rt.CompactTypeVector(3, rt.CompactTypeField), [A_ARGS.c_p, A_ARGS.input_pub, A_ARGS.outcome_pub]);
const argsRecord = {file: opt.args ? resolve(opt.args) : 'defaults (VALUES.txt coll.run[0], anchor enc1.C_P)',
  ...Object.fromEntries(Object.entries(A_ARGS).map(([k, v]) => [k, v.toString()])), claim_pub: claimPub.toString()};
if (phase !== 'deploy') {
  if (st.args && phase === 'call') for (const k of ['c_p', 'input_pub', 'outcome_pub'])
    if (st.calls?.valid?.unboundSavedAt) assert.equal(st.args[k], argsRecord[k], `--args ${k} differs from the proven A; use a fresh --out`);
  st.args = argsRecord;
}
if (st.deploy?.constructorArgs) {
  assert.equal(st.deploy.constructorArgs.anchor, A_ARGS.anchor.toString(), 'anchor differs from the deployed one');
  assert.equal(st.deploy.constructorArgs.reg_pub, A_ARGS.reg_pub.toString(), 'reg_pub differs from the deployed one');
}
const forbid = opt.forbid ? JSON.parse(readFileSync(resolve(opt.forbid), 'utf8')).map((x, i) => typeof x === 'object'
  ? {label: x.label ?? `forbid[${i}]`, value: fr(x.value, `forbid[${i}]`)} : {label: `forbid[${i}]`, value: fr(x, `forbid[${i}]`)}) : [];
const forbidSmall = opt['forbid-small'] ? JSON.parse(readFileSync(resolve(opt['forbid-small']), 'utf8'))
  .map((x, i) => ({label: x.label ?? `small[${i}]`, value: BigInt(x.value)})).filter(x => x.value !== 0n) : [];
const lanesFile = join(here, '../../research/sherlock-t9-contract-20261113/contract/lanes.json');
const templateWords = existsSync(lanesFile) ? new Map(JSON.parse(readFileSync(lanesFile, 'utf8')).entries.acceptRun.words
  .filter(w => String(w.what).startsWith('const')).map(w => [w.lane, BigInt(w.word)])) : new Map();
// T10: the small words on the call vector, each hit judged by its lane (a compiled template constant or not).
const smallScan = vec => {
  const rows = forbidSmall.map(({label, value}) => {
    const lanes = vec.map((x, i) => x === value ? i : -1).filter(i => i >= 0);
    const offTemplate = lanes.filter(l => templateWords.get(l) !== value);
    return {label, value: value.toString(), lanes, offTemplate};
  });
  return {templateLanes: templateWords.size, rows, pass: templateWords.size > 0 && rows.every(r => r.offTemplate.length === 0)};
};
const controls = () => [{label: 'control claim_pub', value: claimPub}, {label: 'control outcome_pub', value: A_ARGS.outcome_pub},
  {label: 'control anchor', value: A_ARGS.anchor}];
const LOC = 'sherlock/acceptRun';
// dry-run checks against LedgerParameters.initialParameters(), whose TTL bound is below 4 h: 3,600 s default there.
const TTL_S = Number(process.env.T10A_TTL_S ?? (OFFLINE ? 3600 : 14_400));
if (!(TTL_S > 0 && TTL_S < 1_209_600)) throw new Error('T10A_TTL_S out of range');
const ttlOf = () => new Date(Date.now() + TTL_S * 1000);
const view = cs => mod.ledger(rt.ContractState.deserialize(cs.serialize()).data);
const constructed = async (anchor, reg) => {
  const initial = await new mod.Contract({}).initialState(rt.createConstructorContext({}, '00'.repeat(32)), anchor, reg);
  const state = ledger.ContractState.deserialize(initial.currentContractState.serialize());
  for (const e of ENTRIES) { const op = new ledger.ContractOperation(); op.verifierKey = vkRaw; state.setOperation(e, op); }
  assert.deepEqual(state.operations().map(String).sort(), ENTRIES);
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
  if (PREVIEW) assert.equal(chain.genesisHash, PREVIEW_GENESIS, 'refuse: the RPC genesis is not the pinned Preview genesis');
  else assert.equal(chain.chain, 'undeployed1', 'local dev chain only');
  st.chain ??= chain; assert.equal(st.chain.genesisHash, chain.genesisHash, 'same local chain as the deploy');
  st.proofServerVersion = (await (await fetch(EP.proofServer + '/version')).text()).trim();
  assert.ok(st.proofServerVersion.startsWith('9.0.'), 'proof server 9.0.x');
  const {block} = await graphql('{ block { height ledgerParameters } }');
  parameters = ledger.LedgerParameters.deserialize(Buffer.from(block.ledgerParameters, 'hex'));
  st.ledgerParameters = {atHeight: block.height, sha256: sha(Buffer.from(block.ledgerParameters, 'hex'))};
} else parameters = ledger.LedgerParameters.initialParameters();
save();

// ---- the padded acceptRun call (as e6-local-run.mjs: built twice from the same circuit run and randomness)
const proto = (target, guaranteed, pd, rand) => new ledger.ContractCallPrototype(target, 'acceptRun', OPERATION, guaranteed, undefined,
  pd.privateTranscriptOutputs, pd.input, pd.output, rand, LOC);
let OPERATION;
class Sentinel extends Error {}
const checkWithServer = async raw => {
  if (OFFLINE && process.env.T10A_DRY_CHECK !== 'server') return null; // dry-run: no proof server
  const r = await fetch(EP.proofServer + '/check', {method: 'POST', headers: {'content-type': 'application/octet-stream'},
    body: ledger.createCheckPayload(raw, ir)});
  if (!r.ok) assert.fail(`proof server /check ${r.status}: ${await r.text().catch(() => '')}`);
  return ledger.parseCheckResult(new Uint8Array(await r.arrayBuffer()));
};
const circuitRun = async (target, against, args) => {
  const context = rt.createCircuitContext({circuitId: 'acceptRun', contractAddress: target, coinPublicKeyOrZswapState: '00'.repeat(32),
    contractState: rt.ContractState.deserialize(against.serialize()), privateState: {}, time: Math.floor(Date.now() / 1000)});
  const result = await new mod.Contract({}).provableCircuits.acceptRun(context, args.c_p, args.input_pub, args.outcome_pub);
  return {context, result};
};
const callTx = async (target, against, args, tag) => {
  const now = new Date();
  const {context, result} = await circuitRun(target, against, args);
  const pd = result.context.callProofDataTrace[0];
  const qc = new ledger.QueryContext(against.data, target);
  qc.block = context.callContext.currentQueryContext.block;
  const [guaranteed, fallible] = ledger.partitionTranscripts([new ledger.PreTranscript(qc, pd.publicTranscript)], parameters)[0];
  assert.ok(guaranteed && !fallible, 'acceptRun must be entirely guaranteed');
  OPERATION = against.operation('acceptRun');
  const rand = ledger.communicationCommitmentRandomness();
  const ttl = ttlOf();
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
  const words2 = decodePreimage(ledger.proofDataIntoSerializedPreimage(pd.input, pd.output, guaranteed.program, pd.privateTranscriptOutputs, LOC)).publicTranscriptInputs.length;
  assert.equal(words2, words, 'transcript word count from two preimage builders');
  const pad = padTranscript(guaranteed, words, TARGET);
  const checkResult = paddedCheckResult(skips ?? Array(guaranteed.program.length).fill(undefined), pad.opsAdded);
  const tx = ledger.Transaction.fromParts(NET, undefined, undefined, ledger.Intent.new(ttl).addCall(proto(target, pad.transcript, pd, rand)));
  const info = {tag, target, builtAt: now.toISOString(), ttl: ttl.toISOString(), opsBefore: guaranteed.program.length,
    transcriptWordsBefore: words, noop: {mode: pad.mode, pad: pad.pad, n: pad.noopN}, opsAfter: pad.transcript.program.length,
    lastOpBefore: guaranteed.program.at(-1), checkSkips: skips ? skips.map(s => s ?? null) : 'not asked (dry-run)',
    gasBeforePadding: guaranteed.gas, unpaddedPreimageBytes: unRaw.length};
  return {tx, info, checkResult, unpaddedWords: un.publicTranscriptInputs, pd};
};
const callOf = tx => [...tx.intents.values()].flatMap(i => i.actions).find(a => a instanceof ledger.ContractCall);
const noopGas = n => { try { return ledger.runProgram(new ledger.VmStack(), [{noop: {n}}], parameters.transactionCostModel.runtimeCostModel).gasCost; }
  catch (e) { return 'n/a: ' + e.message; } };
let lastProverExit = null;
// prove: `fixed` = tagged proof bytes to use as they are; otherwise the prover command `cmd`. The call vector the
// proof is made for ([binding_input, comm] ++ transcript inputs) is written to <tag>.call-vector.json either way.
const proveCall = async (built, {fixed, cmd, tag, env}) => {
  let n = 0, proof, proveSeconds = null, proverError = null;
  lastProverExit = null;
  const provider = {
    check: async (_raw, location) => { assert.equal(location, LOC); return built.checkResult; },
    lookupKey: async () => ({proverKey, verifierKey: vkRaw, ir}),
    prove: async (raw, location, binding) => {
      assert.equal(++n, 1); assert.equal(location, LOC);
      const opening = JSON.stringify({binding_input: bigToLeHex(binding), preimage: Buffer.from(raw).toString('hex')});
      writeFileSync(join(output, `${tag}.binding-input`), bigToLeHex(binding) + '\n');
      const vec = callVector(decodePreimage(raw), bigToLeHex(binding));
      writeFileSync(join(output, `${tag}.call-vector.json`), JSON.stringify({words: vec.map(String)}) + '\n');
      built.vector = vec;
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
  const unbound = await built.tx.prove(provider, parameters.transactionCostModel.runtimeCostModel);
  const call = callOf(unbound);
  assertNormalized(call.guaranteedTranscript.program);
  const gasAfter = call.guaranteedTranscript.gas;
  built.info.gasAfterProve = gasAfter;
  built.info.computeTimeAddedByProve = gasAfter.computeTime - built.info.gasBeforePadding.computeTime;
  built.info.noopGasFromVm = built.info.noop.n ? noopGas(built.info.noop.n) : null;
  built.info.programAfterProveOps = call.guaranteedTranscript.program.length;
  built.info.lastOpAfterProve = call.guaranteedTranscript.program.at(-1);
  return {unbound, proof, proveSeconds, proverError};
};

// Local ledger with the deploy applied and the contract's state set to `against`.
const localLedger = (deployAction, against) => {
  let local = ledger.LedgerState.blank(NET); local.parameters = parameters;
  const setup = ledger.Transaction.fromParts(NET, undefined, undefined, ledger.Intent.new(new Date(Date.now() + 600_000)).addDeploy(deployAction)).eraseProofs(); // local setup only
  const lax = Object.assign(new ledger.WellFormedStrictness(), {enforceBalancing: false, verifyNativeProofs: false,
    verifyContractProofs: false, enforceLimits: false, verifySignatures: false});
  const secs = BigInt(Math.floor(Date.now() / 1000));
  const block = {secondsSinceEpoch: secs, secondsSinceEpochErr: 30, parentBlockHash: '00'.repeat(32), lastBlockTime: secs - 6n};
  const [boot, res] = local.apply(setup.wellFormed(local, lax, new Date()), new ledger.TransactionContext(local, block));
  assert.equal(res.type, 'success');
  return {local: boot.updateIndex(deployAction.address, against.data, against.balance), block, secs};
};
const bindUnbound = unbound => ledger.Transaction.deserialize('signature', 'proof', 'pre-binding', unbound.serialize()).bind();
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
const validateEnv = process.env.T10A_VALIDATE_CALL ?? process.env.E6_VALIDATE_CALL;
const validateCallBin = validateEnv ? resolve(validateEnv) : null;
const nativeValidate = async (deployAction, against, unbound, tag) => {
  if (!validateCallBin) return {skipped: 'T10A_VALIDATE_CALL / E6_VALIDATE_CALL not set'};
  const {local, secs} = localLedger(deployAction, against);
  const r = JSON.parse(await child(validateCallBin, JSON.stringify({state: Buffer.from(local.serialize()).toString('hex'),
    transaction: Buffer.from(bindUnbound(unbound).serialize()).toString('hex'), block_seconds: Number(secs)}), `native-${tag}.log`));
  const pis = r.contractCalls?.[0]?.publicInputs ?? [];
  return {accepted: r.contractProofsSignaturesLimitsAccepted, applied: r.applied, failure: r.contractProfileFailure ?? null,
    allOnFailure: r.allOnFailure ?? null, publicInputs: pis.length, publicInputsLeHex: pis};
};
const vectorEqualsNative = (vec, nat) => {
  if (!nat.publicInputsLeHex?.length) return null;
  const nativeDec = nat.publicInputsLeHex.map(h => leHexToBig(h));
  const diff = nativeDec.map((v, i) => v === vec[i] ? null : i).filter(i => i !== null);
  return {equal: diff.length === 0 && nativeDec.length === vec.length, lanesDiffering: diff.slice(0, 10), ledgerLength: nativeDec.length, vectorLength: vec.length};
};
const deployOf = () => {
  const depTx = ledger.Transaction.deserialize('signature', 'proof', 'binding', readFileSync(join(output, 'deploy-ledger.tx')));
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
const occurrences = (hay, needle) => { let n = 0; for (let i = hay.indexOf(needle); i >= 0; i = hay.indexOf(needle, i + 1)) n++; return n; };
// Lanes of the call vector that hold each value, plus the layout summary.
const vectorLayout = vec => {
  const lanesOf = v => vec.map((w, i) => w === v ? i : null).filter(i => i !== null);
  let zeros = 0; for (let i = vec.length - 1; i >= 2 && vec[i] === 0n; i--) zeros++;
  return {total: vec.length, claim_pub: lanesOf(claimPub), outcome_pub: lanesOf(A_ARGS.outcome_pub), anchor: lanesOf(A_ARGS.anchor),
    c_p: lanesOf(A_ARGS.c_p), input_pub: lanesOf(A_ARGS.input_pub), trailingZeroWords: zeros, lastNonZeroLane: vec.length - 1 - zeros};
};
// The privacy scan: --forbid words must be absent, the control words present, on each surface.
// The scanner must fire on EACH surface named in requireIn (a dead walker on one surface is not hidden by another).
const privacyScan = (surfaces, requireIn = Object.keys(surfaces)) => {
  const rows = scan(surfaces, forbid), ctl = scan(surfaces, controls());
  const missing = ctl.flatMap(r => requireIn.filter(sf => !r.foundIn.includes(sf)).map(sf => `${r.label} not in ${sf}`));
  return {surfaces: surfaceSizes(surfaces), forbidden: rows, forbiddenFound: rows.filter(r => r.found).map(r => r.label),
    controls: ctl, controlsRequiredIn: requireIn, controlsMissing: missing, scannerFires: missing.length === 0,
    pass: rows.every(r => !r.found) && missing.length === 0};
};
const dummyProofPath = process.env.T10A_DUMMY_PROOF ?? join(H, '.cache/e5-wrap/out/lfr-real-k21/wrap.proof');

// ---- dump / dry-run: padded call, stub prover, then shape checks with a dummy proof.
if (phase === 'dump' || phase === 'dry-run') {
  assert.ok(existsSync(dummyProofPath), `dummy proof ${dummyProofPath} missing; set T10A_DUMMY_PROOF`);
  let deployAction, against, target;
  if (OFFLINE) {
    against = await constructed(A_ARGS.anchor, A_ARGS.reg_pub);
    deployAction = new ledger.ContractDeploy(against); target = deployAction.address;
  } else {
    assert.ok(st.deploy?.blockHeight, 'run the deploy phase first');
    against = ledger.ContractState.deserialize(readFileSync(join(output, 'deploy-ledger-state.bin')));
    deployAction = deployOf(); target = st.deploy.address; assert.equal(deployAction.address, target);
  }
  const rec = st[phase] = {at: new Date().toISOString(), target, targetWords: TARGET, args: argsRecord};
  const built = await callTx(target, against, A_ARGS, phase);
  rec.call = built.info; save();
  log(`padded call built: ${built.info.transcriptWordsBefore} transcript words, ${built.info.opsBefore} ops, noop ${json(built.info.noop).trim()}`);
  const dumpFile = join(output, phase === 'dump' ? 'vector.json' : 'dry-run-vector.json');
  const cmd = process.env.T10A_PROVER ?? `node ${JSON.stringify(join(here, 'e6-dump-prover.mjs'))}`;
  const p = await proveCall(built, {cmd, tag: `${phase}-prover`, env: {E6_DUMP: dumpFile, E6_EXPECT_WORDS: String(TARGET + 2)}}).catch(e => ({proverError: e}));
  rec.prover = {cmd, exit: p.proverError ? lastProverExit : 0, error: p.proverError ? String(p.proverError.message) : null};
  if (existsSync(dumpFile)) {
    const words = JSON.parse(readFileSync(dumpFile)).words.map(BigInt);
    assert.deepEqual(words, built.vector, 'the stub vector equals the runner-decoded vector');
    assert.deepEqual(words.slice(2, 2 + built.unpaddedWords.length), built.unpaddedWords, 'lanes 2.. are the unpadded transcript words');
    rec.dump = {file: dumpFile, ...vectorLayout(words), lane0: words[0].toString(), lane1: words[1].toString(),
      unpaddedTranscriptWords: built.unpaddedWords.length, lanes: words.slice(0, 2 + built.unpaddedWords.length + 4).map(String)};
    rec.scan = privacyScan({callVector: words, transcriptOps: nestedWords(built.pd.publicTranscript), preState: contractStateWords(against)},
      ['callVector', 'transcriptOps']); // the pre-call state holds the anchor only
  }
  save();
  const dummy = frameProof(readFileSync(dummyProofPath));
  const fin = await proveCall(built, {fixed: new Uint8Array(dummy.tagged), tag: `${phase}-dummy`});
  rec.call = built.info;
  writeFileSync(join(output, `${phase}-dummy-unbound.tx`), fin.unbound.serialize());
  rec.jsShape = jsShape(deployAction, against, fin.unbound);
  const nat = await nativeValidate(deployAction, against, fin.unbound, phase);
  rec.native = {...nat, publicInputsLeHex: undefined};
  if (rec.dump) {
    const words = JSON.parse(readFileSync(dumpFile)).words.map(BigInt);
    const dummyVec = JSON.parse(readFileSync(join(output, `${phase}-dummy.call-vector.json`))).words.map(BigInt);
    rec.dummyVectorDiffersOnlyAt = dummyVec.map((v, i) => v === words[i] ? null : i).filter(i => i !== null); // same tx: expect []
    rec.dumpEqualsLedgerPublicInputs = vectorEqualsNative(words, nat);
  }
  // Offline rehearsal of the post-A path of `call`: the state after this call (circuit run), its readback and the
  // scan of a populated ledger-v9 state and ledger-v9 transcript ops; then the replay against that state with
  // proofs off (the ledger's own replay refusal, the evidence R3 records next to the node verdict).
  {
    const {result: cr} = await circuitRun(target, against, A_ARGS);
    const postRt = rt.ContractState.deserialize(against.serialize());
    postRt.data = cr.context.callContext.currentQueryContext.state;
    const postL = ledger.ContractState.deserialize(postRt.serialize());
    const v = view(postL);
    rec.postA = {claimInRuns: v.runs.member(claimPub), runsLookupEqualsOutcome: v.runs.member(claimPub) && v.runs.lookup(claimPub) === A_ARGS.outcome_pub,
      scan: privacyScan({callVector: built.vector, includedTranscriptOps: nestedWords(callOf(fin.unbound).guaranteedTranscript.program),
        indexedState: contractStateWords(postL)}),
      replayJsShapeAgainstPostA: jsShape(deployAction, postL, fin.unbound),
      replayLocalCircuit: await circuitRun(target, postL, A_ARGS).then(() => ({refused: false}), e => ({refused: true, error: String(e.message ?? e)}))};
    save();
  }
  rec.shapeVerdict = {jsLedgerAcceptsShapeAndApplies: rec.jsShape.wellFormed && rec.jsShape.applied === 'success',
    nativeFailsOnlyAtProof: nat.skipped ? null : (!nat.accepted && /proof/i.test(nat.failure ?? '') && !/normali|exceed|public input/i.test(nat.failure ?? '')),
    nativePublicInputs: nat.publicInputs ?? null, scanPass: rec.scan?.pass ?? null, postAScanPass: rec.postA.scan.pass,
    postAReadback: rec.postA.runsLookupEqualsOutcome, replayRefusedWithProofsOff: rec.postA.replayJsShapeAgainstPostA.applied !== 'success'
      && rec.postA.replayLocalCircuit.refused};
  save(); log(`${phase}: ${json({...rec, dump: rec.dump && {...rec.dump, lanes: `${rec.dump.lanes.length} lanes (see state)`}})}`);
  process.exit(0);
}

// ---- wallet (deploy, call): the local dev-preset genesis wallet; the seed stays in this process.
const {WalletTransaction, ProtocolVersion, DefaultForkSchedule} = await load('@midnightntwrk/wallet-sdk');
mkdirSync(join(H, '.cache/t9'), {recursive: true});
process.chdir(join(H, '.cache/t9')); // testkit's logger writes ./logs/tests; keep it out of the worktree
const testkit = await load('@midnight-ntwrk/testkit-js');
const {PolkadotNodeClient, makeConfig} = await load('@midnightntwrk/wallet-sdk-node-client');
const {SerializedTransaction} = await load('@midnightntwrk/wallet-sdk-abstractions');
const Rx = await load('rxjs');
const {WebSocket} = await load('ws'); globalThis.WebSocket ??= WebSocket;
testkit.logger.level = 'warn';
if (PREVIEW) {
  // The OP-PV-WALLET wallet, found as preview-session.mjs finds it: the 0600 phrase file of the wallet directory.
  const dir = process.env.MORIARTY_PREVIEW_WALLET_DIR || join(H, '.local/share/moriarty/wallets/preview');
  secretSeed = readFileSync(join(dir, 'seed'), 'utf8').trim();
  const bip39 = await load('@scure/bip39');
  secretHex = Buffer.from(bip39.mnemonicToSeedSync(secretSeed)).toString('hex');
} else {
  const seeds = new testkit.LocalTestEnvironment(testkit.logger).genesisMintWalletSeed;
  assert.ok(Array.isArray(seeds) && /^[0-9a-f]{64}$/.test(seeds[1] ?? '') && seeds[1].endsWith('1'), 'testkit dev-preset genesis seed (index 1)');
  secretSeed = seeds[1];
}
const openWallet = async () => {
  const env = {walletNetworkId: NET, networkId: NET, ...EP, faucet: undefined};
  const {wallet, seeds, keystore} = await testkit.FluentWalletBuilder.forEnvironment(env)
    .withDustOptions({ledgerParams: ledger.LedgerParameters.initialParameters(), additionalFeeOverhead: 1_000n, feeBlocksMargin: 5})
    .withSeed(PREVIEW ? secretHex : secretSeed).buildWithoutStarting();
  if (PREVIEW) assert.equal(keystore.getBech32Address().toString(), PREVIEW_WALLET, 'refuse: not the OP-PV-WALLET wallet');
  await wallet.start(seeds);
  const s = await Rx.firstValueFrom(wallet.state().pipe(
    Rx.filter(s => [s.shielded, s.unshielded, s.dust].every(w => w.progress?.isStrictlyComplete?.()) && s.dust.availableCoins.length > 0),
    Rx.timeout({first: PREVIEW ? 30 * 60_000 : 600_000})));
  log(`wallet synced: dust coins ${s.dust.availableCoins.length}, dust balance ${s.dust.balance(new Date())}`);
  (st.walletSyncs ??= []).push({at: new Date().toISOString(), dustCoins: s.dust.availableCoins.length, dustBalance: String(s.dust.balance(new Date()))}); save();
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
    balanceUnproven: async tx => finalize(await wallet.balanceUnprovenTransaction(WalletTransaction.adopt('Unproven', tx, await active()), {ttl: ttlOf()})),
    balanceUnbound: async tx => finalize(await wallet.balanceUnboundTransaction(WalletTransaction.adopt('Unbound', tx, await active()), {ttl: ttlOf()}))};
};
const node = await PolkadotNodeClient.init(makeConfig({nodeURL: new URL(EP.nodeWS)}));
const nodeLogSince = since => PREVIEW ? Promise.resolve('(Preview: no node container log)') : new Promise(ok => {
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
      new Promise((_, no) => setTimeout(() => no(Error('T10A timeout: no InBlock event within 150 s')), 150_000))]);
    r = {verdict: 'included', txHash: ev.txHash, blockHash: ev.blockHash, blockHeight: ev.blockHeight};
  } catch (e) {
    const chainOf = []; for (let x = e; x; x = x.cause) chainOf.push(scrub(x.message ?? x));
    r = {verdict: chainOf[0].startsWith('T10A timeout') ? 'not-included-timeout' : 'refused-by-node', error: chainOf};
  }
  Object.assign(st.submissions[tag], r, {seconds: (performance.now() - t) / 1000});
  await sleep(2000);
  const logs = scrub(await nodeLogSince(startedAt));
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
  if (st.deploy?.blockHeight) { log('deploy done'); await w.wallet.stop(); await node.close(); process.exit(0); }
  const t0 = performance.now();
  let address, bytes, sub;
  if (st.deploy?.submit?.verdict === 'included') { // resume: submitted, readback pending
    bytes = readFileSync(join(output, 'deploy-ledger.tx')); address = st.deploy.address; sub = st.deploy.submit;
  } else {
    const d = new ledger.ContractDeploy(await constructed(A_ARGS.anchor, A_ARGS.reg_pub)); address = d.address;
    const tx = ledger.Transaction.fromParts(NET, undefined, undefined, ledger.Intent.new(ttlOf()).addDeploy(d));
    const fin = await w.balanceUnproven(tx);
    bytes = fin.tx.serialize();
    writeFileSync(join(output, 'deploy-ledger.tx'), bytes);
    sub = await submit(bytes, 'deploy-ledger', 'deploy SherlockLedger');
    st.deploy = {address, submit: sub, constructorArgs: {anchor: A_ARGS.anchor.toString(), reg_pub: A_ARGS.reg_pub.toString()}}; save();
  }
  assert.equal(sub.verdict, 'included', 'deploy');
  const action = await waitAction(address, 'ContractDeploy');
  const indexedState = Buffer.from(action.state, 'hex');
  writeFileSync(join(output, 'deploy-ledger-state.bin'), indexedState);
  const cs = ledger.ContractState.deserialize(indexedState);
  assert.deepEqual(cs.operations().map(String).sort(), ENTRIES, 'three entries installed');
  const installed = {};
  for (const e of ENTRIES) {
    const k = Buffer.from(cs.operation(e).verifierKey);
    assert.ok(k.equals(Buffer.from(vkRaw)), `indexed installed ${e} key equals the framed --vk bytes`);
    installed[e] = {equalsFramed: true, bytes: k.length, sha256: sha(k)};
  }
  const v = view(cs);
  assert.equal(v.anchor, A_ARGS.anchor); assert.equal(v.reg_root, A_ARGS.reg_pub);
  assert.ok(v.runs.isEmpty()); assert.ok(v.verified_roots.isEmpty());
  st.deploy = {...st.deploy, txHash: sub.txHash, nodeBlockHeight: sub.blockHeight, blockHash: sub.blockHash,
    blockHeight: action.transaction.block.height, indexedTxHash: action.transaction.hash, txId: action.transaction.id,
    status: action.transaction.transactionResult?.status, paidFees: action.transaction.fee, txBytes: bytes.length,
    stateBytes: indexedState.length, stateSHA256: sha(indexedState), key: {...st.key, installed,
      keyBodyOccurrencesInState: occurrences(indexedState, vkFramed.body)},
    readback: {anchor: v.anchor.toString(), reg_root: v.reg_root.toString(), runsEmpty: true, verifiedRootsEmpty: true},
    wallSeconds: (performance.now() - t0) / 1000};
  save(); log(`deploy: ${json(st.deploy)}`);
  await w.wallet.stop(); await node.close(); process.exit(0);
}

// ---- call phase: A, then R1, R2, R3.
const proverCmd = process.env.T10A_PROVER; assert.ok(proverCmd, 'T10A_PROVER (the wrap prover command)');
assert.ok(opt.args, 'call needs --args FILE (c_p, input_pub, outcome_pub of the proved run); no defaults before a prove');
assert.ok(st.deploy?.blockHeight, 'run the deploy phase first');
const addr = st.deploy.address;
const preState = ledger.ContractState.deserialize(readFileSync(join(output, 'deploy-ledger-state.bin')));
const deployAction = deployOf();
st.calls ??= {};
let w = null;
const wallet = async () => (w ??= await openWallet());
const ttlLeftMin = iso => (Date.parse(iso) - Date.now()) / 60_000;
const savedUnbound = (file, info) => existsSync(join(output, file)) && info?.unboundSavedAt && !info.refused && info.ttl && ttlLeftMin(info.ttl) > 5
  ? ledger.Transaction.deserialize('signature', 'proof', 'pre-binding', readFileSync(join(output, file))) : null;
const archive = (...files) => { const sfx = new Date().toISOString().replace(/[:.]/g, ''); for (const f of files)
  if (existsSync(join(output, f))) renameSync(join(output, f), join(output, `${f}.old-${sfx}`)); };
const latestAction = async address => (await graphql(`{contractAction(address:"${address}"){__typename state transaction{hash}}}`)).contractAction;
const markRefused = why => { st.calls.valid = {...(st.calls.valid ?? {}), refused: why}; (st.calls.refusedA ??= []).push({why, at: new Date().toISOString()}); save(); };

if (!st.calls.accept?.readback) {
  let latest = await latestAction(addr);
  assert.ok(latest, `SherlockLedger ${addr} is not on this chain (devnet restarted?); redeploy into a fresh --out`);
  if (latest.__typename !== 'ContractCall' && ['sending', 'included'].includes(st.submissions.A?.verdict)) {
    log(`A was ${st.submissions.A.verdict}; waiting up to 60 s for the indexer`);
    for (let k = 0; k < 30 && latest.__typename !== 'ContractCall'; k++) { await sleep(2000); latest = await latestAction(addr); }
    if (latest.__typename !== 'ContractCall') markRefused(`A ${st.submissions.A.verdict} but not indexed`);
  }
  if (latest.__typename !== 'ContractCall') {
    assert.ok(!view(ledger.ContractState.deserialize(Buffer.from(latest.state, 'hex'))).runs.member(claimPub), 'claim already settled on chain');
    let unbound = savedUnbound('valid-unbound.tx', st.calls.valid);
    if (unbound) log(`resumed the proven unbound A (TTL ${ttlLeftMin(st.calls.valid.ttl).toFixed(1)} min left)`);
    else {
      archive('valid-unbound.tx', 'valid.tagged-proof', 'call-valid.tx', 'valid.call-vector.json');
      log('proving A (the valid call)');
      const built = await callTx(addr, preState, A_ARGS, 'valid');
      st.calls.valid = built.info; save();
      const p = await proveCall(built, {cmd: proverCmd, tag: 'valid'});
      unbound = p.unbound;
      writeFileSync(join(output, 'valid-unbound.tx'), unbound.serialize());
      st.calls.valid = {...built.info, proof: proofBody(p.proof), proveSecondsEndToEnd: p.proveSeconds, unboundSavedAt: new Date().toISOString(),
        vectorLayout: vectorLayout(built.vector)};
      save();
    }
    const nat = await nativeValidate(deployAction, preState, unbound, 'valid');
    const vec = JSON.parse(readFileSync(join(output, 'valid.call-vector.json'))).words.map(BigInt);
    st.calls.nativeValid = {...nat, publicInputsLeHex: undefined, vectorEqualsLedgerPublicInputs: vectorEqualsNative(vec, nat)};
    st.calls.jsShapeValid = jsShape(deployAction, preState, unbound); save();
    if (!nat.skipped && nat.accepted !== true) {
      markRefused(`native: ${nat.failure}`);
      assert.fail(`A refused by the native ledger (${nat.failure}); nothing submitted; a rerun proves again`);
    }
    const ww = await wallet();
    const fin = await ww.balanceUnbound(unbound);
    const bytes = fin.tx.serialize();
    writeFileSync(join(output, 'call-valid.tx'), bytes);
    const sub = await submit(bytes, 'A', 'A valid acceptRun');
    if (sub.verdict !== 'included') {
      await ww.wallet.revert(fin.handle, 'T10A A not included').catch(e => log('revert A: ' + e.message));
      markRefused(`node: ${sub.verdict}`);
      assert.fail(`A not included (${sub.verdict}); see node-log-A.txt; a rerun proves again`);
    }
  }
  const action = await waitAction(addr, 'ContractCall');
  assert.equal(action.entryPoint, 'acceptRun');
  const post = Buffer.from(action.state, 'hex');
  writeFileSync(join(output, 'post-ledger-state.bin'), post);
  const cs = ledger.ContractState.deserialize(post), v = view(cs);
  const goodBytes = readFileSync(join(output, 'call-valid.tx'));
  const goodTx = ledger.Transaction.deserialize('signature', 'proof', 'binding', goodBytes);
  const goodCall = callOf(goodTx);
  const vec = JSON.parse(readFileSync(join(output, 'valid.call-vector.json'))).words.map(BigInt);
  // The included call is the proven one: same guaranteed program as the saved unbound A.
  const unboundA = ledger.Transaction.deserialize('signature', 'proof', 'pre-binding', readFileSync(join(output, 'valid-unbound.tx')));
  const progJson = c => json(c.guaranteedTranscript.program);
  const samePrograms = progJson(goodCall) === progJson(callOf(unboundA));
  const keysUnchanged = ENTRIES.every(e => Buffer.from(cs.operation(e).verifierKey).equals(Buffer.from(vkRaw)));
  st.calls.accept = {readback: true, indexedTxHash: action.transaction.hash, txId: action.transaction.id,
    blockHeight: action.transaction.block.height, status: action.transaction.transactionResult?.status,
    segments: action.transaction.transactionResult?.segments, paidFees: action.transaction.fee, txBytes: goodBytes.length,
    ledgerFeeOfBytes: goodTx.fees(parameters), claimPub: claimPub.toString(), claimInRuns: v.runs.member(claimPub),
    runsLookup: v.runs.member(claimPub) ? v.runs.lookup(claimPub).toString() : null, runsSize: v.runs.size(),
    anchorUnchanged: v.anchor === A_ARGS.anchor, keysUnchanged, includedProgramEqualsProven: samePrograms,
    postStateBytes: post.length, postStateSHA256: sha(post), vectorLayout: vectorLayout(vec),
    scan: privacyScan({callVector: vec, includedTranscriptOps: nestedWords(goodCall.guaranteedTranscript.program), indexedState: contractStateWords(cs)}),
    smallScan: smallScan(vec)};
  assert.ok(v.runs.member(claimPub) && v.runs.lookup(claimPub) === A_ARGS.outcome_pub, 'runs[claim_pub] = outcome_pub');
  save(); log(`A: ${json(st.calls.accept)}`);
}
const ww = await wallet();
await sleep(15_000); await ww.wallet.waitForSyncedState();
const postState = ledger.ContractState.deserialize(readFileSync(join(output, 'post-ledger-state.bin')));

const refusal = async (tag, label, unbound, against) => {
  const rec = st.calls[tag] = {...(st.calls[tag] ?? {}), label};
  rec.native = {...(await nativeValidate(deployAction, against, unbound, tag)), publicInputsLeHex: undefined}; save();
  let fin;
  try { fin = await ww.balanceUnbound(unbound); }
  catch (e) { rec.wallet = {passed: false, error: scrub(e?.message ?? e)}; rec.done = true; save(); return rec; }
  const sub = await submit(fin.tx.serialize(), tag, label);
  rec.submit = sub;
  if (sub.verdict !== 'included') await ww.wallet.revert(fin.handle, 'T10A negative control').catch(e => log(`revert ${tag}: ${e.message}`));
  else rec.indexed = await waitAction(addr, 'ContractCall', {skipHash: st.calls.accept.indexedTxHash, ms: 60_000}).catch(e => String(e.message));
  rec.done = true; save(); log(`${tag}: ${json(rec)}`);
  return rec;
};
const validProof = () => new Uint8Array(readFileSync(join(output, 'valid.tagged-proof')));
// R1 outcome-differs: outcome_pub + 1 carrying A's proof, against the deploy state.
if (!st.calls.r1?.done) {
  const built = await callTx(addr, preState, {...A_ARGS, outcome_pub: (A_ARGS.outcome_pub + 1n) % FR_MODULUS}, 'r1');
  const bad = await proveCall(built, {fixed: validProof(), tag: 'r1'});
  st.calls.r1 = {call: built.info, vectorDiffersFromA: (() => { const a = JSON.parse(readFileSync(join(output, 'valid.call-vector.json'))).words;
    return built.vector.map((x, i) => String(x) === a[i] ? null : i).filter(i => i !== null); })()};
  await refusal('r1', 'R1 outcome-differs (outcome_pub + 1, reused proof of A)', bad.unbound, preState);
}
// R2 tampered-proof: the saved unbound A with one proof bit flipped.
if (!st.calls.r2?.done) {
  const {raw, flippedOffset} = flipProof(readFileSync(join(output, 'valid-unbound.tx')), validProof());
  const tampered = ledger.Transaction.deserialize('signature', 'proof', 'pre-binding', raw);
  st.calls.r2 = {flippedOffsetInUnbound: flippedOffset};
  await refusal('r2', 'R2 tampered-proof (one proof bit flipped)', tampered, preState);
}
// R4 (T10) program B against program A's anchor: B's acceptRun arguments, B's own wrap proof, this ledger (C_L of A).
const r4Args = process.env.T10A_R4_ARGS, r4Proof = process.env.T10A_R4_PROOF;
if (r4Args && r4Proof && !st.calls.r4?.done) {
  const b = JSON.parse(readFileSync(resolve(r4Args), 'utf8'));
  const bArgs = {...A_ARGS, c_p: fr(b.c_p, 'R4 c_p'), input_pub: fr(b.input_pub, 'R4 input_pub'), outcome_pub: fr(b.outcome_pub, 'R4 outcome_pub')};
  const built = await callTx(addr, preState, bArgs, 'r4');
  const bad = await proveCall(built, {fixed: new Uint8Array(readFileSync(resolve(r4Proof))), tag: 'r4'});
  st.calls.r4 = {call: built.info, programBAnchor: b.anchor, ledgerAnchor: A_ARGS.anchor.toString(), proofFile: resolve(r4Proof)};
  await refusal('r4', 'R4 program B (its args and its own wrap proof) against program A\'s anchor', bad.unbound, preState);
}
// R3 replay: the same claim after A landed. The circuit against the post-A state refuses locally (recorded); the
// transaction is built against the deploy state (the transcript claims runs.member(claim_pub) = false), with a
// fresh binding, so the node sees a call whose transcript no longer matches the chain.
if (!st.calls.r3?.done) {
  const local = await circuitRun(addr, postState, A_ARGS).then(() => ({refused: false}), e => ({refused: true, error: String(e.message ?? e)}));
  const fresh = process.env.T10A_R3_FRESH === '1';
  let unbound = fresh ? savedUnbound('r3-unbound.tx', st.calls.r3?.call) : null;
  if (!unbound) {
    archive('r3-unbound.tx', 'r3.tagged-proof');
    const built = await callTx(addr, preState, A_ARGS, 'r3');
    const p = await proveCall(built, fresh ? {cmd: proverCmd, tag: 'r3'} : {fixed: validProof(), tag: 'r3'});
    unbound = p.unbound;
    writeFileSync(join(output, 'r3-unbound.tx'), unbound.serialize());
    st.calls.r3 = {call: {...built.info, unboundSavedAt: new Date().toISOString()}, proofSource: fresh ? 'fresh (T10A_R3_FRESH=1)' : 'A proof, fresh binding'};
    save();
  }
  st.calls.r3 = {...st.calls.r3, localCircuitAgainstPostA: local, jsShapeAgainstPostA: jsShape(deployAction, postState, unbound)};
  await refusal('r3', `R3 replay of A after it landed (${st.calls.r3.proofSource})`, unbound, postState);
  const after = view(ledger.ContractState.deserialize(Buffer.from((await waitAction(addr)).state, 'hex')));
  st.calls.r3.afterRunsSize = after.runs.size(); save();
}
const sub = t => st.submissions[t] ?? {};
st.verdict = {A: st.calls.accept?.readback === true && st.calls.accept?.claimInRuns === true,
  scan: st.calls.accept?.scan?.pass === true,
  R1: sub('r1').verdict === 'refused-by-node', R2: sub('r2').verdict === 'refused-by-node',
  R3: sub('r3').verdict === 'refused-by-node' && st.calls.r3?.localCircuitAgainstPostA?.refused === true
    && st.calls.r3?.jsShapeAgainstPostA?.applied !== 'success',
  R4: (r4Args && r4Proof) ? sub('r4').verdict === 'refused-by-node' : 'not run',
  smallScan: st.calls.accept?.smallScan ? st.calls.accept.smallScan.pass : 'not run',
  R3stage: st.calls.r3?.native?.skipped ? 'native not run' : st.calls.r3?.native?.accepted ? 'proof accepted, refused at apply (replay rule)' : `proof stage: ${st.calls.r3?.native?.failure}`};
st.verdict.pass = st.verdict.A && st.verdict.scan && st.verdict.R1 && st.verdict.R2 && st.verdict.R3
  && st.verdict.R4 !== false && st.verdict.smallScan !== false;
st.done = new Date().toISOString(); save();
log(`verdict: ${json(st.verdict)}`);
await ww.wallet.stop(); await node.close();
process.exit(st.verdict.pass ? 0 : 4);
