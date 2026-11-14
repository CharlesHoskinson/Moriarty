// T10a compiled-contract tests of SherlockLedger (sherlock-ledger.compact, kit LEDGER CONTRACT, FP-3 facts 1 to 3,
// anchor prologue). compactc 0.36.0, compact-runtime 0.21.0, ledger 9.1.0.1 (for the call-vector words).
// Build first: tools/sherlock-sdk/sherlock-ledger-build.sh [DIR]
// Run: LEDGER_BUILD=DIR [SHERLOCK_SDK_ROOT=~/.cache/xc9/sdk] [LANES_OUT=FILE] [T10A_FORBID=FILE] node --test sherlock-ledger.test.mjs
//   LEDGER_BUILD  build dir with sherlock-ledger/, control-two-maps/, control-shape/, sherlock-run/, lanes-zkir.json
//   LANES_OUT     write the measured lane map (lanes.json) there
//   T10A_FORBID   extra forbidden words for the privacy scan: JSON list of decimal strings or {label, value}
// Values: research/sherlock-enc-fp3-20261112/fp3-closure/VALUES.txt (the Rust reference fp3_closure.rs).
// The call-vector words are the ledger's own: partitionTranscripts, then proofDataIntoSerializedPreimage, then
// the tagged preimage's public transcript inputs, which are lanes 2.. of [binding_input, comm] ++ inputs. Lanes
// 0 and 1 exist only in a transaction (binding, commitment randomness); the devnet dump reads them.
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync, mkdirSync, existsSync, symlinkSync} from 'node:fs';
import {resolve, join, dirname} from 'node:path';
import {homedir} from 'node:os';
import {pathToFileURL, fileURLToPath} from 'node:url';
import {createHash} from 'node:crypto';
import test from 'node:test';
import {decodePreimage, FR_MODULUS} from './e6-padding.mjs';
import {alignedWords, nestedWords, contractStateWords, scan, packWords} from './sherlock-ledger-scan.mjs';

const here = dirname(fileURLToPath(import.meta.url));
const repo = resolve(here, '../..');
const H = homedir();
const sdk = resolve(process.env.SHERLOCK_SDK_ROOT ?? join(H, '.cache/xc9/sdk'));
const build = resolve(process.env.LEDGER_BUILD ?? join(H, '.cache/t9/contract/build'));
// ESM-only SDK packages load through shim modules beside a node_modules link (outside the worktree).
const shimDir = resolve(process.env.T9_SHIM_DIR ?? join(H, '.cache/t9/contract/shims'));
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
const L = await load('@midnight-ntwrk/midnight-js-protocol/v9');
const contractOf = async name => import(pathToFileURL(join(build, name, 'contract/index.js')).href);
const main = await contractOf('sherlock-ledger');
const ctrl = await contractOf('control-two-maps');
const shape = await contractOf('control-shape');
const runCtl = await contractOf('sherlock-run');
const zkirOf = (name, entry) => JSON.parse(readFileSync(join(build, name, 'zkir', `${entry}.zkir`), 'utf8'));
const params = L.LedgerParameters.initialParameters();

const V = Object.fromEntries(readFileSync(join(repo, 'research/sherlock-enc-fp3-20261112/fp3-closure/VALUES.txt'), 'utf8')
  .split('\n').filter(l => l && !l.startsWith('#')).map(l => l.split(' = ')).map(([k, v]) => [k, BigInt(v)]));
const run = k => ['c_p', 'input_pub', 'outcome_pub'].map(f => V[`coll.run[${k}].${f}`]);
const th = xs => rt.transientHash(new rt.CompactTypeVector(xs.length, rt.CompactTypeField), xs);
const ANCHOR = 0x65f6874b0be5aaa5a42b2e3b6b94ee9adf10dad60d7eff1211c65e3331ff2994n; // enc1.C_P (LEAD pick)
const ANCHOR2 = 0x37adc0f165135af5f670e5ba16a4d473c4a6ec42847cb53833acaecdb08e82a2n; // enc1.A_1 (second deploy)
const REG = V['coll.reg_pub'], SET = V['coll.set_pub'];
const ADDR = '01'.repeat(32);

// ---- deploy and call, keeping a compact-runtime ContractState and the ledger's call-vector words
async function deploy(mod, ...args) {
  const contract = new mod.Contract({});
  const st = await contract.initialState(rt.createConstructorContext({}, '00'.repeat(32)), ...args);
  return {mod, contract, cs: st.currentContractState, calls: []};
}
async function call(s, entry, ...args) {
  const before = L.ContractState.deserialize(s.cs.serialize());
  const ctx = rt.createCircuitContext({circuitId: entry, contractAddress: ADDR, coinPublicKeyOrZswapState: '00'.repeat(32),
    contractState: s.cs, privateState: {}});
  const r = await s.contract.provableCircuits[entry](ctx, ...args);
  const pds = r.context.callProofDataTrace;
  assert.equal(pds.length, 1, 'one call per entry');
  const pd = pds[0];
  const qc = new L.QueryContext(before.data, ADDR);
  qc.block = ctx.callContext.currentQueryContext.block;
  const [guaranteed, fallible] = L.partitionTranscripts([new L.PreTranscript(qc, pd.publicTranscript)], params)[0];
  assert.ok(guaranteed && !fallible, `${entry} is entirely guaranteed`);
  const pre = decodePreimage(L.proofDataIntoSerializedPreimage(pd.input, pd.output, guaranteed.program,
    pd.privateTranscriptOutputs, `sherlock/${entry}`));
  assert.equal(pre.privateTranscript.length, 0);
  s.cs.data = r.context.callContext.currentQueryContext.state;
  const rec = {entry, args, pd, pre, words: pre.publicTranscriptInputs, outputs: pre.publicTranscriptOutputs,
    stateAfter: contractStateWords(s.cs)};
  s.calls.push(rec);
  return rec;
}
const lanesOf = rec => ({total: 2 + rec.words.length, lane: i => rec.words[i - 2]});

test('(a) XC-3b parity: transientHash equals the u = 0 sponge and the Rust claim_pub values', () => {
  for (const n of [0, 1, 2, 3, 5, 16]) {
    const xs = Array.from({length: n}, (_, i) => BigInt(i + 1));
    assert.equal(th(xs), V[`parity.u0[${n}]`], `n = ${n}`);
  }
  for (const k of [0, 1, 2]) assert.equal(th(run(k)), V[`coll.run[${k}].claim_pub`], `claim_pub run ${k}`);
});

test('(b) fp3_cross_entry_replay: one runs map refuses a claim settled through either entry', async () => {
  const s = await deploy(main, ANCHOR, REG);
  await call(s, 'acceptRun', ...run(0));
  await call(s, 'acceptCollection', REG, SET);
  await assert.rejects(call(s, 'openClaim', SET, ...run(0)), /claim already settled/, 'acceptRun then openClaim');
  await call(s, 'openClaim', SET, ...run(1));
  await assert.rejects(call(s, 'acceptRun', ...run(1)), /claim already settled/, 'openClaim then acceptRun');
  await assert.rejects(call(s, 'acceptRun', ...run(0)), /claim already settled/, 'acceptRun twice');
  await assert.rejects(call(s, 'openClaim', SET, ...run(1)), /claim already settled/, 'openClaim twice');
  const l = main.ledger(s.cs.data);
  for (const k of [0, 1]) assert.equal(l.runs.lookup(V[`coll.run[${k}].claim_pub`]), V[`coll.run[${k}].outcome_pub`]);
  assert.equal(l.runs.member(V['coll.run[2].claim_pub']), false);
  await call(s, 'openClaim', SET, ...run(2));
  assert.equal(main.ledger(s.cs.data).runs.size(), 3n);
  assert.equal(main.ledger(s.cs.data).anchor, ANCHOR, 'no entry writes the anchor');
});

test('(b) control: the two-map contract with the same prologue accepts both cross-entry replays', async () => {
  const s = await deploy(ctrl, ANCHOR, REG);
  await call(s, 'acceptRun', ...run(0));
  await call(s, 'acceptCollection', REG, SET);
  await call(s, 'openClaim', SET, ...run(0));       // refused by the main contract
  await call(s, 'openClaim', SET, ...run(1));
  await call(s, 'acceptRun', ...run(1));            // refused by the main contract
  const l = ctrl.ledger(s.cs.data);
  assert.equal(l.runs.size(), 2n); assert.equal(l.opened.size(), 2n);
  // the same entry twice is still refused inside each map
  await assert.rejects(call(s, 'acceptRun', ...run(0)), /claim already settled/);
});

test('(c, g) openClaim preconditions and refusals: unaccepted set, wrong reg_pub, collection twice, zero anchor', async () => {
  const s = await deploy(main, ANCHOR, REG);
  await assert.rejects(call(s, 'openClaim', SET, ...run(0)), /collection not accepted/);
  await assert.rejects(call(s, 'acceptCollection', REG + 1n, SET), /other registry root/);
  await assert.rejects(call(s, 'acceptCollection', V['coll.run[0].claim_pub'], SET), /other registry root/);
  await assert.rejects(call(s, 'openClaim', SET, ...run(0)), /collection not accepted/, 'a refused acceptCollection leaves the set unaccepted');
  await call(s, 'acceptCollection', REG, SET);
  await assert.rejects(call(s, 'acceptCollection', REG, SET), /collection already accepted/);
  await assert.rejects(call(s, 'openClaim', SET + 1n, ...run(0)), /collection not accepted/, 'another set_pub');
  await call(s, 'openClaim', SET, ...run(0));
  assert.equal(main.ledger(s.cs.data).verified_roots.size(), 1n);
  await assert.rejects(deploy(main, 0n, REG), /zero anchor/);
});

// ---- (d) privacy scan
const STANDIN_PROG = [520957427n, 984145394n, 1989647505n, 623846375n, 1856043986n, 928030188n, 771931842n, 1420568668n];
function forbidList() {
  const f = [];
  // STAND-IN prog_d words: the reference prog_d = P2(5, [k, 1]) (fp3_closure.rs collection()) needs Poseidon2 over
  // BabyBear, which this test does not have. SherlockLedger has no argument or field that a prog_d could enter
  // through; the control below shows the scanner finds a program digest where a contract does take one.
  STANDIN_PROG.forEach((w, i) => f.push({label: `prog_d stand-in word ${i}`, value: w}));
  f.push({label: 'pack(prog_d stand-in)', value: packWords(STANDIN_PROG)});
  f.push({label: 'set_root pack', value: V['coll.set_root_pack']});
  for (const k of [0, 1, 2]) {
    f.push({label: `run ${k} n_steps`, value: V[`coll.run[${k}].n_steps`]});
    f.push({label: `run ${k} claim_d pack`, value: V[`coll.run[${k}].claim_d_pack`]});
    f.push({label: `run ${k} SET head pack`, value: V[`coll.run[${k}].head_pack`]});
  }
  if (process.env.T10A_FORBID) for (const x of JSON.parse(readFileSync(process.env.T10A_FORBID, 'utf8')))
    f.push(typeof x === 'object' ? {label: x.label, value: BigInt(x.value)} : {label: `T10A_FORBID ${x}`, value: BigInt(x)});
  return f;
}
const surfacesOf = rec => ({callVector: rec.words, transcriptOps: nestedWords(rec.pd.publicTranscript),
  callInputs: [...alignedWords(rec.pd.input), ...alignedWords(rec.pd.output)], state: rec.stateAfter});

test('(d) fp3_collection_private and run-side privacy: no forbidden word on any public surface', async () => {
  const s = await deploy(main, ANCHOR, REG);
  const deployWords = contractStateWords(s.cs);
  await call(s, 'acceptRun', ...run(0));
  await call(s, 'acceptCollection', REG, SET);
  await call(s, 'openClaim', SET, ...run(1));
  await call(s, 'openClaim', SET, ...run(2));
  const forbid = forbidList();
  for (const row of scan({deployState: deployWords}, forbid)) assert.equal(row.found, false, `${row.label} in the deploy state`);
  for (const rec of s.calls) {
    const sf = surfacesOf(rec);
    for (const n of Object.keys(sf)) assert.ok(sf[n].length > 0, `${rec.entry}: surface ${n} is empty`);
    for (const row of scan(sf, forbid)) assert.equal(row.found, false, `${rec.entry}: ${row.label} in ${row.foundIn}`);
  }
  // The scanner reads the public values on the surfaces it scans (non-vacuous on this contract too).
  const [ar, ac, oc1] = s.calls;
  const must = (rec, label, value, where) => {
    const [row] = scan(surfacesOf(rec), [{label, value}]);
    for (const w of where) assert.ok(row.foundIn.includes(w), `${rec.entry}: ${label} not seen in ${w} (seen in ${row.foundIn})`);
  };
  must(ar, 'claim_pub', V['coll.run[0].claim_pub'], ['callVector', 'transcriptOps', 'state']);
  must(ar, 'outcome_pub', V['coll.run[0].outcome_pub'], ['callVector', 'transcriptOps', 'callInputs', 'state']);
  must(ar, 'anchor', ANCHOR, ['callVector', 'transcriptOps', 'state']);
  must(ac, 'set_pub', SET, ['callVector', 'transcriptOps', 'callInputs', 'state']);
  must(oc1, 'claim_pub', V['coll.run[1].claim_pub'], ['callVector', 'state']);
});

test('(d) control: the same scanner FINDS program_d and steps in a sherlock-run.compact call', async () => {
  const prog = packWords(STANDIN_PROG), init = 0x1234567n, n = V['coll.run[0].n_steps'];
  const be16 = v => new Uint8Array(Buffer.from(v.toString(16).padStart(32, '0'), 'hex'));
  const s = await deploy(runCtl, prog, init, be16(1n));
  const rec = await call(s, 'acceptRun', 0xabcdefn, prog, init, n, 0x77n, be16(42n));
  const rows = scan(surfacesOf(rec), forbidList());
  const byLabel = Object.fromEntries(rows.map(r => [r.label, r]));
  for (const w of ['callVector', 'transcriptOps', 'callInputs', 'state']) {
    assert.ok(byLabel['pack(prog_d stand-in)'].foundIn.includes(w), `program_d not found in ${w}`);
    assert.ok(byLabel['run 0 n_steps'].foundIn.includes(w), `steps not found in ${w}`);
  }
});

// ---- (e) LW-009: acceptCollection takes [reg_pub, set_pub] only and writes nothing keyed by shape_d.
async function lw009(mod, name, extra = []) {
  const failures = [];
  const ins = zkirOf(name, 'acceptCollection').inputs.map(i => i.name.split('.')[0].replace(/^%/, ''));
  if (JSON.stringify(ins) !== JSON.stringify(['reg_pub', 'set_pub'])) failures.push(`acceptCollection inputs ${ins}`);
  const s = await deploy(mod, ANCHOR, REG);
  await call(s, 'acceptRun', ...run(0));
  const names = Object.keys(Object.getOwnPropertyDescriptors(mod.ledger(s.cs.data)));
  if (names.some(k => /shape/i.test(k))) failures.push(`state field ${names.filter(k => /shape/i.test(k))}`);
  const cells = s.cs.data.state.asArray();
  const before = cells.map(c => contractStateWords({data: {state: c}}).map(String).sort().join());
  await call(s, 'acceptCollection', REG, SET, ...extra);
  const after = s.cs.data.state.asArray().map(c => contractStateWords({data: {state: c}}).map(String).sort().join());
  const changed = after.map((w, i) => w === before[i] ? null : i).filter(i => i !== null);
  const vrIdx = 2; // field order: anchor, runs, verified_roots, reg_root (sherlock-ledger.compact)
  if (cells.length !== 4) failures.push(`${cells.length} state fields`);
  if (JSON.stringify(changed) !== JSON.stringify([vrIdx])) failures.push(`fields changed by acceptCollection: ${changed}`);
  const added = new Set(after[vrIdx].split(',')); for (const w of before[vrIdx].split(',')) added.delete(w);
  added.delete('');
  if (JSON.stringify([...added]) !== JSON.stringify([SET.toString()])) failures.push(`words added to verified_roots: ${[...added]}`);
  return {fields: names, inputs: ins, changed, failures};
}
test('(e) LW-009: acceptCollection takes [reg_pub, set_pub], writes only set_pub into verified_roots', async () => {
  const r = await lw009(main, 'sherlock-ledger');
  assert.deepEqual(r.failures, []);
  assert.deepEqual(r.fields.sort(), ['anchor', 'reg_root', 'runs', 'verified_roots']);
});
test('(e) control: the LW-009 checks flag a contract with a shape_d argument and a write keyed by it', async () => {
  const r = await lw009(shape, 'control-shape', [0xfeedn]);
  assert.ok(r.failures.some(f => f.startsWith('acceptCollection inputs')), 'argument check fires');
  assert.ok(r.failures.some(f => f.startsWith('state field')), 'field-name check fires');
  assert.ok(r.failures.some(f => f.startsWith('5 state fields') || f.startsWith('fields changed')), 'write check fires');
});

// ---- (f) the anchor prologue in the call-vector words, against the zkir impact split
const frOf = c => { const v = BigInt(c.replace(/^-/, '')); return c.startsWith('-') ? (FR_MODULUS - v) % FR_MODULUS : v; };
// Expected lanes from the zkir: every impact input in order from lane 2, with circuit inputs and transientHash
// outputs computed here and public_input values taken from the preimage's transcript outputs, in order.
function zkirLanes(zk, args, outputs) {
  const val = {}, label = {};
  zk.inputs.forEach((x, i) => { val[x.name] = BigInt(args[i]); label[x.name] = x.name.split('.')[0].replace(/^%/, ''); });
  let reads = 0;
  const lanes = [];
  for (const ins of zk.instructions) {
    if (ins.op === 'transient_hash') {
      val[ins.output] = th(ins.inputs.map(x => val[x] ?? frOf(x)));
      label[ins.output] = `transientHash(${ins.inputs.map(x => label[x] ?? x).join(',')})`;
    }
    if (ins.op === 'public_input') { val[ins.output] = outputs[reads]; label[ins.output] = `read${reads}`; reads++; }
    if (ins.op === 'impact') {
      assert.equal(ins.guard, '0x01', 'unguarded impact');
      for (const x of ins.inputs) {
        if (x.startsWith('%')) { assert.ok(x in val, `zkir symbol ${x}`); lanes.push({word: val[x], what: label[x]}); }
        else lanes.push({word: frOf(x), what: `const ${x}`});
      }
    }
  }
  assert.equal(reads, outputs.length, 'one transcript output per read');
  return lanes;
}
const LEAD = { // the lead's lanes.py numbers (MEASURED on the zkir by the lead)
  acceptRun: {lanes: 42, claim_pub: [20, 34], outcome_pub: [39]},
  acceptCollection: {lanes: 48, reg_root_read: [19], set_pub: [29, 43]},
  openClaim: {lanes: 57, set_pub: [20], claim_pub: [35, 49], outcome_pub: [54]},
};
const lanesWith = (words, v) => words.map((w, i) => w === v ? i + 2 : null).filter(i => i !== null);

test('(f) prologue: every entry starts with the same template words and reads the anchor at lane 10', async () => {
  const s = await deploy(main, ANCHOR, REG);
  const s2 = await deploy(main, ANCHOR2, REG);
  const recs = {}, recs2 = {};
  for (const [st, out] of [[s, recs], [s2, recs2]]) {
    out.acceptRun = await call(st, 'acceptRun', ...run(0));
    out.acceptCollection = await call(st, 'acceptCollection', REG, SET);
    out.openClaim = await call(st, 'openClaim', SET, ...run(1));
  }
  const template = recs.acceptRun.words.slice(0, 8); // lanes 2 to 9
  const report = {schema: 't10a-ledger-lanes/1', anchor: ANCHOR.toString(), compactc: '0.36.0', runtime: 'compact-runtime 0.21.0',
    ledger: 'ledger-v9 (proofDataIntoSerializedPreimage)', lanesNote: 'lane i = word i of [binding_input, comm] ++ public transcript inputs; lanes 0 and 1 need a transaction (devnet dump)',
    prologueTemplateLanes2to10: {}, entries: {}};
  for (const [entry, rec] of Object.entries(recs)) {
    const {lane, total} = lanesOf(rec);
    assert.deepEqual(rec.words.slice(0, 8), template, `${entry}: lanes 2 to 9 are the template`);
    assert.equal(lane(10), ANCHOR, `${entry}: lane 10 is the anchor`);
    // second anchor: only lane 10 changes, in the whole vector
    const w2 = recs2[entry].words;
    const diff = rec.words.map((w, i) => w === w2[i] ? null : i + 2).filter(i => i !== null);
    assert.equal(w2.length, rec.words.length);
    assert.deepEqual(diff, [10], `${entry}: lanes that change with the anchor`);
    assert.equal(w2[8], ANCHOR2);
    // the runtime words equal the zkir impact split, word for word
    const zl = zkirLanes(zkirOf('sherlock-ledger', entry), rec.args, rec.outputs);
    assert.equal(zl.length, rec.words.length, `${entry}: zkir lanes ${zl.length + 2}, runtime ${total}`);
    const mismatch = zl.map((z, i) => z.word === rec.words[i] ? null : i + 2).filter(i => i !== null);
    assert.deepEqual(mismatch, [], `${entry}: runtime words differ from the zkir impact split at lanes ${mismatch}`);
    // lane numbers against the lead's lanes.py numbers
    assert.equal(total, LEAD[entry].lanes, `${entry} lanes`);
    const at = {anchor_read: lanesWith(rec.words, ANCHOR)};
    if (entry !== 'acceptCollection') {
      const k = rec.entry === 'acceptRun' ? 0 : 1;
      at.claim_pub = lanesWith(rec.words, V[`coll.run[${k}].claim_pub`]);
      at.outcome_pub = lanesWith(rec.words, V[`coll.run[${k}].outcome_pub`]);
      assert.deepEqual(at.claim_pub, LEAD[entry].claim_pub); assert.deepEqual(at.outcome_pub, LEAD[entry].outcome_pub);
      at.c_p = lanesWith(rec.words, rec.args.at(-3)); at.input_pub = lanesWith(rec.words, rec.args.at(-2));
      assert.deepEqual(at.c_p, []); assert.deepEqual(at.input_pub, []);
    }
    if (entry !== 'acceptRun') { at.set_pub = lanesWith(rec.words, SET); assert.deepEqual(at.set_pub, LEAD[entry].set_pub); }
    if (entry === 'acceptCollection') { at.reg_root_read = lanesWith(rec.words, REG); assert.deepEqual(at.reg_root_read, LEAD[entry].reg_root_read); }
    assert.deepEqual(at.anchor_read, [10]);
    report.entries[entry] = {lanes: total, transcriptWords: rec.words.length, valueLanes: at,
      runtimeEqualsZkirImpactSplit: true, onlyLaneChangingWithAnchor: diff,
      words: zl.map((z, i) => ({lane: i + 2, word: rec.words[i].toString(), what: z.what}))};
  }
  for (let i = 0; i < 9; i++) report.prologueTemplateLanes2to10[`lane${i + 2}`] =
    {word: recs.acceptRun.words[i].toString(), what: zkirLanes(zkirOf('sherlock-ledger', 'acceptRun'), recs.acceptRun.args, recs.acceptRun.outputs)[i].what};
  report.lanesPy = JSON.parse(readFileSync(join(build, 'lanes-zkir.json'), 'utf8'));
  if (process.env.LANES_OUT) { mkdirSync(dirname(resolve(process.env.LANES_OUT)), {recursive: true}); writeFileSync(resolve(process.env.LANES_OUT), JSON.stringify(report, null, 1) + '\n'); }
});
