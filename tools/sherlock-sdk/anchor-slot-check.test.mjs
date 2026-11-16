// T23 /check at slot 15 (Preview rehearsal 2026-11-15): the proof server's /check runs the stock compactc zkir of
// acceptRun, which refuses every slot-15 transcript ("Public transcript input mismatch for input 4"). t10a sends
// /check the slot-0 preimage of the same circuit run instead, after assertSlot15CheckEquivalent. This test builds both
// preimages offline, as t10a does, and checks them with the zkir-v3 `check` of the pinned SDK (the proof server's
// check): the stock zkir accepts slot 0 and refuses slot 15 at input 4; a key-15 zkir (the stock JSON IR with the
// prologue key 0 -> 15, nothing else) accepts slot 15 with the same skips as the stock check of slot 0. Negative
// controls: a second changed word and a changed op are refused by assertSlot15CheckEquivalent.
// No network, no proofs. Usage: [SHERLOCK_SDK_ROOT=~/.cache/xc9/sdk] [T17_BUILD=~/.cache/t17/contract/build]
//                               node --test anchor-slot-check.test.mjs
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync, existsSync, mkdirSync, symlinkSync} from 'node:fs';
import {join, resolve, dirname} from 'node:path';
import {homedir} from 'node:os';
import {pathToFileURL, fileURLToPath} from 'node:url';
import {createHash} from 'node:crypto';
import test from 'node:test';
import {ANCHOR_SLOT, ledgerDeployState, prologueToSlot15, assertSlot15CheckEquivalent, PROLOGUE_KEY_INPUT} from './anchor-slot.mjs';
import {decodePreimage, encodePreimage} from './e6-padding.mjs';

const H = homedir();
const here = dirname(fileURLToPath(import.meta.url));
const sdk = resolve(process.env.SHERLOCK_SDK_ROOT ?? join(H, '.cache/xc9/sdk'));
const build = resolve(process.env.T17_BUILD ?? join(H, '.cache/t17/contract/build'), 'sherlock-ledger');
assert.ok(existsSync(join(build, 'zkir/acceptRun.zkir')), `compactc build with zkir at ${build}`);
const shims = join(here, '../../target/anchor-slot-check-test');
mkdirSync(shims, {recursive: true});
if (!existsSync(join(shims, 'node_modules'))) symlinkSync(join(sdk, 'node_modules'), join(shims, 'node_modules'));
const load = async name => {
  const f = join(shims, createHash('sha256').update(name).digest('hex') + '.mjs');
  writeFileSync(f, `import * as m from ${JSON.stringify(name)}; export default m;\n`);
  return (await import(pathToFileURL(f).href)).default;
};
const ledger = await load('@midnight-ntwrk/midnight-js-protocol/v9');
const rt = await load('@midnight-ntwrk/compact-runtime');
const zkir = await load('@midnightntwrk/zkir-v3');
const mod = await import(pathToFileURL(join(build, 'contract/index.js')).href);
const ir = readFileSync(join(build, 'zkir/acceptRun.bzkir'));
const irJson = JSON.parse(readFileSync(join(build, 'zkir/acceptRun.zkir'), 'utf8'));
const ANCHOR = 0x65f6874b0be5aaa5a42b2e3b6b94ee9adf10dad60d7eff1211c65e3331ff2994n;

// The key-15 zkir: the one impact that writes the prologue `idx` (opcode 0x50) with the constant key 0 gets key 15.
const isPrologueIdx = x => x.op === 'impact' && x.inputs[0] === '0x50' && x.inputs.at(-1) === '0x00';
const irKey15 = () => {
  const j = structuredClone(irJson);
  const at = j.instructions.map((x, i) => isPrologueIdx(x) ? i : -1).filter(i => i >= 0);
  assert.equal(at.length, 1, 'one prologue idx impact with key 0 in acceptRun.zkir');
  j.instructions[at[0]].inputs = [...j.instructions[at[0]].inputs.slice(0, -1), '0x' + ANCHOR_SLOT.toString(16).padStart(2, '0')];
  return {bytes: zkir.jsonIrToBinary(JSON.stringify(j)), json: j, at: at[0]};
};
const km = irBytes => ({lookupKey: async () => ({proverKey: new Uint8Array(), verifierKey: new Uint8Array(), ir: irBytes}),
  getParams: async () => { throw new Error('check needs no params'); }});
const check = (raw, irBytes) => zkir.check(raw, km(irBytes));

// Both builds of one acceptRun circuit run, as t10a-ledger-run.mjs callTx builds them (same randomness and TTL).
const builds = async () => {
  const initial = await new mod.Contract({}).initialState(rt.createConstructorContext({}, '00'.repeat(32)), ANCHOR, 0x1234n);
  const state = ledger.ContractState.deserialize(initial.currentContractState.serialize());
  state.data = new ledger.ChargedState(ledger.StateValue.decode(ledgerDeployState(rt, ANCHOR, 0x1234n)));
  const op = new ledger.ContractOperation(); op.verifierKey = readFileSync(join(build, 'keys/acceptRun.verifier'));
  state.setOperation('acceptRun', op);
  const target = new ledger.ContractDeploy(state).address;
  const ctx = rt.createCircuitContext({circuitId: 'acceptRun', contractAddress: target, coinPublicKeyOrZswapState: '00'.repeat(32),
    contractState: rt.ContractState.deserialize(state.serialize()), privateState: {}, time: 1});
  const res = await new mod.Contract({}).provableCircuits.acceptRun(ctx, 11n, 12n, 13n);
  const pd = res.context.callProofDataTrace[0];
  const params = ledger.LedgerParameters.initialParameters();
  const qc = new ledger.QueryContext(state.data, target); qc.block = ctx.callContext.currentQueryContext.block;
  const rand = ledger.communicationCommitmentRandomness(), ttl = new Date(Date.now() + 600_000);
  const one = async ops => {
    const [g, f] = ledger.partitionTranscripts([new ledger.PreTranscript(qc, ops)], params)[0];
    assert.ok(g && !f);
    let raw;
    const tx = ledger.Transaction.fromParts('undeployed', undefined, undefined, ledger.Intent.new(ttl).addCall(new ledger.ContractCallPrototype(
      target, 'acceptRun', state.operation('acceptRun'), g, undefined, pd.privateTranscriptOutputs, pd.input, pd.output, rand, 'sherlock/acceptRun')));
    await tx.prove({check: async r => { raw = new Uint8Array(r); return Array(g.program.length).fill(undefined); },
      lookupKey: async () => ({proverKey: new Uint8Array(), verifierKey: new Uint8Array(), ir}),
      prove: async () => { throw new Error('STOP'); }}, params.transactionCostModel.runtimeCostModel)
      .then(() => assert.fail('prove must stop'), e => { if (!String(e?.message ?? e).includes('STOP')) throw e; });
    return {g, raw, pre: decodePreimage(raw)};
  };
  return {s0: await one(pd.publicTranscript), s15: await one(prologueToSlot15(rt, pd.publicTranscript))};
};
const B = await builds();

test('stock zkir: /check accepts the slot-0 preimage and refuses the slot-15 preimage at input 4 (the rehearsal failure)', async () => {
  assert.equal((await check(B.s0.raw, ir)).length, B.s0.g.program.length);
  await assert.rejects(check(B.s15.raw, ir), new RegExp(`Public transcript input mismatch for input ${PROLOGUE_KEY_INPUT}`));
});

test('key-15 zkir differs from the stock zkir in the prologue key only; every impact guard is the constant 1', () => {
  assert.ok(Buffer.from(zkir.jsonIrToBinary(JSON.stringify(irJson))).equals(ir), 'acceptRun.zkir round-trips to acceptRun.bzkir');
  const k = irKey15();
  const d = irJson.instructions.map((x, i) => JSON.stringify(x) === JSON.stringify(k.json.instructions[i]) ? null : i).filter(i => i !== null);
  assert.deepEqual(d, [k.at]);
  // impact words before the key: 0x30 (dup) then 0x50, 1, 1 -> the key is public transcript input 4
  const before = irJson.instructions.slice(0, k.at).filter(x => x.op === 'impact').reduce((n, x) => n + x.inputs.length, 0);
  assert.equal(before + irJson.instructions[k.at].inputs.length - 1, PROLOGUE_KEY_INPUT);
  assert.deepEqual([...new Set(irJson.instructions.filter(x => x.op === 'impact').map(x => x.guard))], ['0x01']);
});

test('slot-0 /check skips equal a correct (key-15 zkir) /check of the slot-15 preimage; the two differ only in input 4', async () => {
  const d = assertSlot15CheckEquivalent(B.s0.pre, B.s15.pre, B.s0.g, B.s15.g);
  assert.deepEqual(d, {input: PROLOGUE_KEY_INPUT, from: 0n, to: BigInt(ANCHOR_SLOT)});
  const k15 = irKey15().bytes;
  const skips0 = await check(B.s0.raw, ir), skips15 = await check(B.s15.raw, k15);
  assert.deepEqual(skips15, skips0);
  assert.ok(skips0.every(s => s === undefined));
  await assert.rejects(check(B.s0.raw, k15), /mismatch for input 4/);
});

test('assertSlot15CheckEquivalent refuses any other difference', () => {
  const pre = (p, f) => { const c = structuredClone(p); f(c); return decodePreimage(encodePreimage(c)); };
  const moreWords = pre(B.s15.pre, c => { c.publicTranscriptInputs[7] += 1n; });
  assert.throws(() => assertSlot15CheckEquivalent(B.s0.pre, moreWords, B.s0.g, B.s15.g), /differ at \[4,7\]/);
  const otherKey = pre(B.s15.pre, c => { c.publicTranscriptInputs[4] = 14n; });
  assert.throws(() => assertSlot15CheckEquivalent(B.s0.pre, otherKey, B.s0.g, B.s15.g), /not only input 4/);
  const otherOut = pre(B.s15.pre, c => { c.publicTranscriptOutputs[0] += 1n; });
  assert.throws(() => assertSlot15CheckEquivalent(B.s0.pre, otherOut, B.s0.g, B.s15.g), /publicTranscriptOutputs differs/);
  const otherBind = pre(B.s15.pre, c => { c.bindingInput += 1n; });
  assert.throws(() => assertSlot15CheckEquivalent(B.s0.pre, otherBind, B.s0.g, B.s15.g), /bindingInput differs/);
  const g = {...B.s15.g, program: [...B.s15.g.program.slice(0, -1), {noop: {n: 3}}]};
  assert.throws(() => assertSlot15CheckEquivalent(B.s0.pre, B.s15.pre, B.s0.g, g), /programs differ at ops \[1,12\]/);
  assert.throws(() => assertSlot15CheckEquivalent(B.s0.pre, B.s15.pre, B.s0.g, {...B.s15.g, gas: {...B.s15.g.gas, readTime: 1n}}), /gas differs/);
  assert.throws(() => assertSlot15CheckEquivalent(B.s0.pre, B.s0.pre, B.s0.g, B.s0.g), /not only input 4/, 'two slot-0 builds');
});
