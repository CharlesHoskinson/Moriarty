// FP-3 contract-side closure tests on the compiled ledger contract (compactc 0.36.0 --skip-zk,
// compact-runtime 0.21.0). Values come from VALUES.txt (the Rust reference, fp3_closure.rs).
// Run: FP3_BUILD=<dir with sherlock-ledger/ and control-two-maps/> SHERLOCK_SDK_ROOT=<sdk> node --test fp3.test.mjs
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createRequire} from 'node:module';
import {resolve, join, dirname} from 'node:path';
import {pathToFileURL, fileURLToPath} from 'node:url';
import test from 'node:test';

const here = dirname(fileURLToPath(import.meta.url));
const sdk = process.env.SHERLOCK_SDK_ROOT;
assert.ok(sdk, 'SHERLOCK_SDK_ROOT must name the SDK package directory');
const require = createRequire(resolve(sdk, 'package.json'));
const rt = await import(pathToFileURL(require.resolve('@midnight-ntwrk/compact-runtime')).href);
const build = resolve(process.env.FP3_BUILD ?? 'build');
const main = await import(pathToFileURL(join(build, 'sherlock-ledger/contract/index.js')).href);
const ctrl = await import(pathToFileURL(join(build, 'control-two-maps/contract/index.js')).href);

const V = Object.fromEntries(readFileSync(join(here, 'VALUES.txt'), 'utf8').split('\n')
  .filter(l => l && !l.startsWith('#')).map(l => l.split(' = ')).map(([k, v]) => [k, BigInt(v)]));
const run = k => ['c_p', 'input_pub', 'outcome_pub'].map(f => V[`coll.run[${k}].${f}`]);
const th = xs => rt.transientHash(new rt.CompactTypeVector(xs.length, rt.CompactTypeField), xs);

async function deploy(mod, reg = V['coll.reg_pub']) {
  const contract = new mod.Contract({});
  const st = await contract.initialState(rt.createConstructorContext({}, '00'.repeat(32)), reg);
  return {mod, contract, data: st.currentContractState.data, transcripts: []};
}
async function call(s, entry, ...args) {
  const ctx = rt.createCircuitContext({circuitId: entry, contractAddress: '01'.repeat(32),
    coinPublicKeyOrZswapState: '00'.repeat(32), contractState: s.data, privateState: {}});
  const r = await s.contract.provableCircuits[entry](ctx, ...args);
  s.data = r.context.callContext.currentQueryContext.state;
  s.transcripts.push({entry, proof: r.context.callProofDataTrace});
  return r;
}

test('XC-3b parity: transientHash equals the u = 0 Midnight sponge of the Rust reference', () => {
  for (const n of [0, 1, 2, 3, 5, 16]) {
    const xs = Array.from({length: n}, (_, i) => BigInt(i + 1));
    assert.equal(th(xs), V[`parity.u0[${n}]`], `n = ${n}`);
  }
  for (const k of [0, 1, 2]) assert.equal(th(run(k)), V[`coll.run[${k}].claim_pub`], `claim_pub run ${k}`);
});

test('fp3_cross_entry_replay: one settled-claim map refuses a claim settled through the other entry', async () => {
  // acceptRun first, then openClaim of the same claim.
  let s = await deploy(main);
  await call(s, 'acceptRun', ...run(0));
  await call(s, 'acceptCollection', V['coll.reg_pub'], V['coll.set_pub']);
  await assert.rejects(call(s, 'openClaim', V['coll.set_pub'], ...run(0)), /claim already settled/);
  // openClaim first, then acceptRun of the same claim.
  await call(s, 'openClaim', V['coll.set_pub'], ...run(1));
  await assert.rejects(call(s, 'acceptRun', ...run(1)), /claim already settled/);
  // The same entry twice.
  await assert.rejects(call(s, 'acceptRun', ...run(0)), /claim already settled/);
  await assert.rejects(call(s, 'openClaim', V['coll.set_pub'], ...run(1)), /claim already settled/);
  // The keys are the Rust claim_pub values.
  const runs = main.ledger(s.data).runs;
  for (const k of [0, 1]) assert.equal(runs.lookup(V[`coll.run[${k}].claim_pub`]), V[`coll.run[${k}].outcome_pub`]);
  assert.equal(runs.member(V['coll.run[2].claim_pub']), false);
  // A third run of the collection opens once.
  await call(s, 'openClaim', V['coll.set_pub'], ...run(2));
  assert.equal(main.ledger(s.data).runs.size(), 3n);
});

test('fp3_cross_entry_replay: the two-map control accepts both replays', async () => {
  const s = await deploy(ctrl);
  await call(s, 'acceptRun', ...run(0));
  await call(s, 'acceptCollection', V['coll.reg_pub'], V['coll.set_pub']);
  await call(s, 'openClaim', V['coll.set_pub'], ...run(0));
  await call(s, 'openClaim', V['coll.set_pub'], ...run(1));
  await call(s, 'acceptRun', ...run(1));
  const l = ctrl.ledger(s.data);
  assert.equal(l.runs.size(), 2n);
  assert.equal(l.opened.size(), 2n);
});

test('openClaim needs an accepted collection of the registered template', async () => {
  const s = await deploy(main);
  await assert.rejects(call(s, 'openClaim', V['coll.set_pub'], ...run(0)), /collection not accepted/);
  await assert.rejects(call(s, 'acceptCollection', V['coll.reg_pub'] + 1n, V['coll.set_pub']), /other template/);
  // A run's claim_pub offered as a collection root does not make its run openable: openClaim
  // keys on (set_pub, claim) and the root of a SET is a DIGEST(SHK_SET) output, which the wrap
  // lane map binds; here the contract alone is checked.
  await call(s, 'acceptCollection', V['coll.reg_pub'], V['coll.set_pub']);
  await assert.rejects(call(s, 'acceptCollection', V['coll.reg_pub'], V['coll.set_pub']), /collection already accepted/);
});

// Every field element in a public transcript, as a BigInt (transcript cells are LE hex).
function transcriptFields(proofs) {
  const out = [];
  const walk = v => {
    if (Array.isArray(v)) return v.forEach(walk);
    if (v && typeof v === 'object') {
      if (Array.isArray(v.value) && Array.isArray(v.alignment)) {
        v.value.forEach((x, i) => {
          const a = v.alignment[i];
          if (x instanceof Uint8Array || typeof x === 'string') {
            const hex = typeof x === 'string' ? x : Buffer.from(x).toString('hex');
            if (hex.length) out.push(BigInt('0x' + Buffer.from(hex, 'hex').reverse().toString('hex')));
          }
          void a;
        });
      }
      return Object.values(v).forEach(walk);
    }
  };
  for (const p of proofs) walk(p.publicTranscript);
  return out;
}

test('fp3_collection_private: no step count, claim digest or SET node in the public transcripts', async () => {
  const s = await deploy(main);
  await call(s, 'acceptCollection', V['coll.reg_pub'], V['coll.set_pub']);
  for (const k of [0, 1, 2]) await call(s, 'openClaim', V['coll.set_pub'], ...run(k));
  const proofs = s.transcripts.flatMap(t => t.proof);
  assert.equal(proofs.length, 4);
  const seen = new Set(transcriptFields(proofs));
  assert.ok(seen.has(V['coll.set_pub']) && seen.has(V['coll.run[2].claim_pub']), 'the walk reads the public values');
  const priv = [V['coll.set_root_pack']];
  for (const k of [0, 1, 2]) {
    priv.push(V[`coll.run[${k}].n_steps`], V[`coll.run[${k}].claim_d_pack`], V[`coll.run[${k}].head_pack`]);
  }
  for (const x of priv) assert.equal(seen.has(x), false, `private value 0x${x.toString(16)} in a transcript`);
  // Inputs of the calls: only the public digests.
  for (const p of proofs) for (const x of transcriptFields([{publicTranscript: p.input}])) {
    assert.ok(!priv.includes(x));
  }
});

test('lanes.json is the lane map of the compiled entries', () => {
  const lanes = JSON.parse(readFileSync(join(here, 'lanes.json'), 'utf8'));
  assert.deepEqual(lanes.acceptRun.carried['transientHash(c_p,input_pub,outcome_pub)'], [11, 25]);
  assert.deepEqual(lanes.acceptRun.carried.outcome_pub, [30]);
  assert.deepEqual(lanes.openClaim.carried.set_pub, [11]);
  assert.deepEqual(lanes.openClaim.carried['transientHash(c_p,input_pub,outcome_pub)'], [26, 40]);
  assert.deepEqual(lanes.openClaim.carried.outcome_pub, [45]);
  assert.deepEqual(lanes.acceptCollection.carried.set_pub, [20, 34]);
});
