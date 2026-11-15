// T20 compiled-contract tests of the batched SherlockLedger and the example claim consumer (Midnight write
// rule W-2: no per-claim ledger write; a consumer proves membership in an accepted claims_root inside its own
// call and keeps its own nullifier). compactc 0.36.0, compact-runtime 0.21.0 (cross-contract calls resolved
// as in mycroft/tests/compact-suite/tools/drive.mjs: the callee's operations carry its compiled verifier keys).
// Build first: tools/sherlock-sdk/t20-build.sh [DIR]
// Run: T20_BUILD=DIR [SHERLOCK_SDK_ROOT=~/.cache/xc9/sdk] node --test t20-ledger.test.mjs
// Every refusal below is a counterexample attempt on the real compiled code (COUNTEREXAMPLES.md, T20 rows).
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync, mkdirSync, existsSync, symlinkSync} from 'node:fs';
import {resolve, join} from 'node:path';
import {homedir} from 'node:os';
import {pathToFileURL, fileURLToPath} from 'node:url';
import {createHash} from 'node:crypto';
import test from 'node:test';

const here = resolve(fileURLToPath(import.meta.url), '..');
const repo = resolve(here, '../..');
const H = homedir();
const sdk = resolve(process.env.SHERLOCK_SDK_ROOT ?? join(H, '.cache/xc9/sdk'));
const build = resolve(process.env.T20_BUILD ?? join(H, '.cache/t20/build'));
const shimDir = resolve(process.env.T20_SHIM_DIR ?? join(H, '.cache/t20/shims'));
mkdirSync(shimDir, {recursive: true});
if (!existsSync(join(shimDir, 'node_modules'))) symlinkSync(join(sdk, 'node_modules'), join(shimDir, 'node_modules'));
if (!existsSync(join(build, 'node_modules'))) symlinkSync(join(sdk, 'node_modules'), join(build, 'node_modules'));
if (!existsSync(join(build, 'package.json'))) writeFileSync(join(build, 'package.json'), '{"type":"module"}\n');
const load = async name => {
  const file = join(shimDir, createHash('sha256').update(name).digest('hex') + '.mjs');
  if (!existsSync(file)) writeFileSync(file, `import * as m from ${JSON.stringify(name)}; export default m;\n`);
  return (await import(pathToFileURL(file).href)).default;
};
const rt = await load('@midnight-ntwrk/compact-runtime');
const modOf = name => import(pathToFileURL(join(build, name, 'contract/index.js')).href);
const ledgerMod = await modOf('sherlock-ledger');
const consumerMod = await modOf('claim-consumer');

const V = Object.fromEntries(readFileSync(join(repo, 'research/sherlock-enc-fp3-20261112/fp3-closure/VALUES.txt'), 'utf8')
  .split('\n').filter(l => l && !l.startsWith('#')).map(l => l.split(' = ')).map(([k, v]) => [k, BigInt(v)]));
const run = k => ['c_p', 'input_pub', 'outcome_pub'].map(f => V[`coll.run[${k}].${f}`]);
const th = xs => rt.transientHash(new rt.CompactTypeVector(xs.length, rt.CompactTypeField), xs);
const ANCHOR = 0x65f6874b0be5aaa5a42b2e3b6b94ee9adf10dad60d7eff1211c65e3331ff2994n;
const REG = V['coll.reg_pub'];
const tag = s => { let v = 0n; [...Buffer.from(s, 'ascii')].reverse().forEach(b => { v = (v << 8n) | BigInt(b); }); return v; };
const SHK_CLAIM_FGN = tag('SHK_CLAIM_FGN');
const LEDGER = '02'.repeat(32), CONSUMER = '03'.repeat(32), COIN = '00'.repeat(32);

// ---- the claims tree of W-2: depth 20, node H2(l, r), index bit 1 = running value on the right
export const D_C = 20;
const ZERO = [0n];
for (let i = 0; i < D_C; i++) ZERO.push(th([ZERO[i], ZERO[i]]));
export function claimsTree(leaves) {
  const levels = [leaves.slice()];
  for (let d = 0; d < D_C; d++) {
    const cur = levels[d], nxt = [];
    for (let i = 0; i < Math.max(1, Math.ceil(cur.length / 2)); i++) {
      const l = cur[2 * i] ?? ZERO[d], r = cur[2 * i + 1] ?? ZERO[d];
      nxt.push(th([l, r]));
    }
    levels.push(nxt);
  }
  return {root: levels[D_C][0], path: idx => {
    const sibs = [], bits = [];
    let i = idx;
    for (let d = 0; d < D_C; d++) { sibs.push(levels[d][i ^ 1] ?? ZERO[d]); bits.push((i & 1) === 1); i >>= 1; }
    return {sibs, bits};
  }};
}
const claimPub = k => th(run(k));
export const foreignLeaf = fclaimPub => th([SHK_CLAIM_FGN, fclaimPub]);

// ---- deploy the ledger with the compiled keys of its circuits (the read entries are native-keyed)
const ledgerKeys = Object.fromEntries(['acceptRun', 'acceptCollection', 'rootAccepted', 'runAccepted']
  .map(c => [c, Uint8Array.from(readFileSync(join(build, 'sherlock-ledger/keys', `${c}.verifier`)))]));
function withOps(charged, keys = ledgerKeys) {
  const s = new rt.ContractState(); s.data = charged;
  for (const [name, vk] of Object.entries(keys)) { const op = new rt.ContractOperation(); op.verifierKey = vk; s.setOperation(name, op); }
  return s;
}
async function newWorld(consumer = consumerMod) {
  const lc = new ledgerMod.Contract({});
  const li = await lc.initialState(rt.createConstructorContext({}, COIN), ANCHOR, REG);
  const w = {lc, ledger: withOps(li.currentContractState.data), path: {sibs: Array(D_C).fill(0n), bits: Array(D_C).fill(false)}};
  w.cc = new consumer.Contract({
    claimSiblings: ({privateState}) => [privateState, w.path.sibs],
    claimIndexBits: ({privateState}) => [privateState, w.path.bits],
  });
  const ci = await w.cc.initialState(rt.createConstructorContext({}, COIN), {bytes: Uint8Array.from(Buffer.from(LEDGER, 'hex'))});
  w.consumer = ci.currentContractState;
  w.consumerMod = consumer;
  return w;
}
async function ledgerCall(w, entry, ...args) {
  const ctx = rt.createCircuitContext({circuitId: entry, contractAddress: LEDGER, coinPublicKeyOrZswapState: COIN,
    contractState: w.ledger, privateState: {}});
  const r = await w.lc.circuits[entry](ctx, ...args);
  w.ledger = withOps(r.context.callContext.currentQueryContext.state);
  return r;
}
async function consumerCall(w, entry, path, ...args) {
  w.path = path;
  const crossContract = {
    stateProvider: {getContractState: async (_h, a) => (a === LEDGER ? w.ledger : undefined)},
    moduleProvider: {resolve: a => (a === LEDGER ? async () => ledgerMod : undefined)},
  };
  const ledgerBefore = Buffer.from(w.ledger.serialize()).toString('hex');
  const ctx = rt.createCircuitContext({circuitId: entry, contractAddress: CONSUMER, coinPublicKeyOrZswapState: COIN,
    contractState: w.consumer, privateState: {}, parentBlockHash: '00'.repeat(32), crossContract});
  const r = await w.cc.circuits[entry](ctx, ...args);
  w.consumer = withOps(r.context.callContext.currentQueryContext.state, {});
  const lq = r.context.queryContexts?.[LEDGER];
  const ledgerAfter = lq ? Buffer.from(withOps(new rt.ChargedState(lq.state.state ?? lq.state)).serialize()).toString('hex') : ledgerBefore;
  return {r, ledgerUnchanged: ledgerAfter === ledgerBefore, calls: r.context.callProofDataTrace.length};
}
const ledgerView = w => ledgerMod.ledger(w.ledger.data);
const usedSize = w => w.consumerMod.ledger(w.consumer.data).used.size();

// ---- W-2 tests (fix 2)
test('W-2 entries: no openClaim, no per-claim entry; the ledger keeps runs and claim_roots only', () => {
  const circuits = Object.keys(new ledgerMod.Contract({}).circuits).sort();
  assert.deepEqual(circuits, ['acceptCollection', 'acceptRun', 'rootAccepted', 'runAccepted']);
});

test('W-2 collection: one claims_root per epoch, no claim write; replay and other registry refused', async () => {
  const w = await newWorld();
  const t = claimsTree([claimPub(0), claimPub(1), claimPub(2)]);
  await ledgerCall(w, 'acceptCollection', REG, t.root);
  const l = ledgerView(w);
  assert.equal(l.claim_roots.size(), 1n);
  assert.equal(l.claim_roots.member(t.root), true);
  assert.equal(l.runs.size(), 0n, 'no claim written by acceptCollection');
  await assert.rejects(ledgerCall(w, 'acceptCollection', REG, t.root), /collection already accepted/);
  await assert.rejects(ledgerCall(w, 'acceptCollection', REG + 1n, claimsTree([claimPub(1)]).root), /other registry root/);
});

test('W-2 consumer: a member of an accepted root is used once; the ledger is not written', async () => {
  const w = await newWorld();
  const t = claimsTree([claimPub(0), claimPub(1), claimPub(2)]);
  await ledgerCall(w, 'acceptCollection', REG, t.root);
  const res = await consumerCall(w, 'useRunClaim', t.path(1), ...run(1));
  assert.equal(res.ledgerUnchanged, true, 'rootAccepted writes nothing');
  assert.equal(res.calls, 2, 'the callee read and the consumer call');
  assert.equal(usedSize(w), 1n);
  await assert.rejects(consumerCall(w, 'useRunClaim', t.path(1), ...run(1)), /claim already used/);
  // another member of the same root
  await consumerCall(w, 'useRunClaim', t.path(2), ...run(2));
  assert.equal(usedSize(w), 2n);
});

test('W-2 refusals: unaccepted root, forged outcome, wrong index, other path', async () => {
  const w = await newWorld();
  const t = claimsTree([claimPub(0), claimPub(1)]);
  const u = claimsTree([claimPub(2)]);                       // never accepted
  await ledgerCall(w, 'acceptCollection', REG, t.root);
  await assert.rejects(consumerCall(w, 'useRunClaim', u.path(0), ...run(2)), /root not accepted/, 'unaccepted root');
  const [c, i, o] = run(0);
  await assert.rejects(consumerCall(w, 'useRunClaim', t.path(0), c, i, o + 1n), /root not accepted/, 'forged outcome');
  await assert.rejects(consumerCall(w, 'useRunClaim', t.path(1), ...run(0)), /root not accepted/, 'path of another slot');
  const p = t.path(0); const bad = {sibs: p.sibs.slice(), bits: p.bits.slice()}; bad.bits[0] = true;
  await assert.rejects(consumerCall(w, 'useRunClaim', bad, ...run(0)), /root not accepted/, 'flipped index bit');
  assert.equal(usedSize(w), 0n);
});

test('W-2 one use across sources: a claim in two epochs and in runs is used once', async () => {
  const w = await newWorld();
  const t1 = claimsTree([claimPub(0), claimPub(1)]);
  const t2 = claimsTree([claimPub(1), claimPub(0)]);          // the same claims in a later epoch (diamond, re-proof)
  await ledgerCall(w, 'acceptCollection', REG, t1.root);
  await ledgerCall(w, 'acceptCollection', REG, t2.root);
  await ledgerCall(w, 'acceptRun', ...run(0));               // and its user also settled it
  await consumerCall(w, 'useRunClaim', t1.path(0), ...run(0));
  await assert.rejects(consumerCall(w, 'useRunClaim', t2.path(1), ...run(0)), /claim already used/, 'second root');
  await assert.rejects(consumerCall(w, 'useSettledRun', t1.path(0), ...run(0)), /claim already used/, 'runs source');
  await consumerCall(w, 'useSettledRun', t1.path(0), ...run(1)).then(() => assert.fail('run 1 is not in runs'),
    e => assert.match(String(e.message), /run not accepted/));
});

test('W-2 controls: a root-keyed nullifier double-uses; a consumer without the root read takes a forged path', async () => {
  const ctlA = await modOf('consumer-control-root-nullifier');
  const a = await newWorld(ctlA);
  const t1 = claimsTree([claimPub(0)]), t2 = claimsTree([claimPub(1), claimPub(0)]);
  await ledgerCall(a, 'acceptCollection', REG, t1.root);
  await ledgerCall(a, 'acceptCollection', REG, t2.root);
  await consumerCall(a, 'useRunClaim', t1.path(0), ...run(0));
  await consumerCall(a, 'useRunClaim', t2.path(1), ...run(0));  // accepted: the defect the design refuses
  assert.equal(usedSize(a), 2n);
  const ctlB = await modOf('consumer-control-no-root-check');
  const b = await newWorld(ctlB);
  await consumerCall(b, 'useRunClaim', claimsTree([claimPub(2)]).path(0), ...run(2)); // accepted with no root on chain
  assert.equal(usedSize(b), 1n);
});

test('W-2 read entry key: a callee whose rootAccepted holds another key is not resolved', async () => {
  const w = await newWorld();
  const t = claimsTree([claimPub(0)]);
  await ledgerCall(w, 'acceptCollection', REG, t.root);
  const wrapLike = {...ledgerKeys, rootAccepted: ledgerKeys.acceptRun};   // stand-in for a non-native key
  w.ledger = withOps(w.ledger.data, wrapLike);
  await assert.rejects(consumerCall(w, 'useRunClaim', t.path(0), ...run(0)), /mismatch|Implementation/i);
});

// ---- W-3 tests (fix 3): foreign claims by membership, no foreign map
const FCLAIM = th([0x53484b5f46434c41494dn, 7n]); // a stand-in fclaim_pub (any Fr value; the leaf form is what is tested)
test('W-3 foreign claim: used once by membership; the ledger has no foreign state', async () => {
  const w = await newWorld();
  const t = claimsTree([claimPub(0), foreignLeaf(FCLAIM), claimPub(1)]);
  await ledgerCall(w, 'acceptCollection', REG, t.root);
  assert.equal('foreign' in ledgerView(w), false, 'no foreign map');
  const res = await consumerCall(w, 'useForeignClaim', t.path(1), FCLAIM);
  assert.equal(res.ledgerUnchanged, true);
  await assert.rejects(consumerCall(w, 'useForeignClaim', t.path(1), FCLAIM), /claim already used/);
  await consumerCall(w, 'useRunClaim', t.path(0), ...run(0));   // the run members stay usable
  assert.equal(usedSize(w), 2n);
});

test('W-3 kind separation: a run leaf is not a foreign claim (the control without the tag accepts it)', async () => {
  const w = await newWorld();
  const t = claimsTree([claimPub(0), claimPub(1)]);
  await ledgerCall(w, 'acceptCollection', REG, t.root);
  await assert.rejects(consumerCall(w, 'useForeignClaim', t.path(0), claimPub(0)), /root not accepted/);
  const ctl = await modOf('consumer-control-no-kind-tag');
  const c = await newWorld(ctl);
  await ledgerCall(c, 'acceptCollection', REG, t.root);
  await consumerCall(c, 'useForeignClaim', t.path(0), claimPub(0));   // accepted: the defect the design refuses
  assert.equal(usedSize(c), 1n);
});
