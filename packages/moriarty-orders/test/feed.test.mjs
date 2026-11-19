import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {mkdtempSync, readFileSync, rmSync, symlinkSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {dirname, join, resolve} from 'node:path';
import test, {after, before} from 'node:test';
import {fileURLToPath, pathToFileURL} from 'node:url';
import {signFeedUpdate} from '../src/feed.mjs';

const here = dirname(fileURLToPath(import.meta.url));
const dependencies = resolve(process.env.MORIARTY_ORDER_MODULES || resolve(here, '../../../tools/sherlock-sdk/node_modules'));
const rt = await import(pathToFileURL(join(dependencies, '@midnight-ntwrk/compact-runtime/dist/index.js')));
const compileDir = mkdtempSync(join(tmpdir(), 'moriarty-feed-'));
const ZERO_KEY = '00'.repeat(32);
const feedId = (value) => Uint8Array.from(Buffer.from(value.padEnd(32, '\0')));
const FEED = feedId('feed-AB');
const NETWORK = feedId('test-network');
const ADDRESS = rt.sampleContractAddress();
let compiled;
before(async () => {
  const source = resolve(here, '../contracts/feed.compact');
  if (process.env.COMPACTC) execFileSync(process.env.COMPACTC, ['--skip-zk', source, compileDir]);
  else execFileSync('compact', ['compile', '+0.36.0', '--skip-zk', source, compileDir]);
  symlinkSync(dependencies, join(compileDir, 'node_modules'), 'dir');
  compiled = await import(pathToFileURL(join(compileDir, 'contract/index.js')));
});
after(() => rmSync(compileDir, {recursive: true, force: true}));

function context(state, time, address = ADDRESS, circuitId = 'checkFeed') {
  const ctx = rt.createCircuitContext({circuitId, contractAddress: address, coinPublicKeyOrZswapState: ZERO_KEY, contractState: state, privateState: {}, time: Number(time)});
  ctx.callContext.currentQueryContext.block = {...ctx.callContext.currentQueryContext.block,
    secondsSinceEpoch: time, secondsSinceEpochErr: 0, lastBlockTime: time};
  return ctx;
}

async function fixture() {
  const secret = 42n; // Public test vector only.
  const contract = new compiled.Contract({});
  const initial = await contract.initialState(rt.createConstructorContext({}, ZERO_KEY), rt.ecMulGenerator(secret), NETWORK);
  let state = initial.currentContractState;
  function prepare(time, sample = {}, key = secret, network = NETWORK) {
    const observation = {feed: FEED, price: 2n, observedAt: time, ...sample};
    const domain = {network, address: {bytes: rt.encodeContractAddress(ADDRESS)}};
    const signature = signFeedUpdate(rt, compiled.pureCircuits, domain, observation, key);
    return {observation, signature};
  }
  async function publish(time, {observation, signature}, address = ADDRESS) {
    const ctx = context(state, time, address, 'publishFeedUpdate');
    const result = await contract.circuits.publishFeedUpdate(ctx, observation, signature);
    state = replay(result, time, address).state;
  }
  function check(time, id = FEED) {
    return contract.circuits.checkFeed(context(state, time), id);
  }
  function replay(result, time, address = ADDRESS) {
    const ctx = context(state, time, address);
    const {initialQueryContext, publicTranscript} = result.context.callProofDataTrace[0];
    const gatherProgram = publicTranscript.map(op => op.popeq ? {popeq: {...op.popeq, result: null}} : op);
    const {gasCost} = initialQueryContext.query(gatherProgram, ctx.costModel);
    return ctx.callContext.currentQueryContext.runTranscript({
      program: result.context.callProofDataTrace[0].publicTranscript,
      effects: result.context.callContext.currentQueryContext.effects,
      gas: gasCost,
    }, ctx.costModel);
  }
  function ledger() { return compiled.ledger(state.data ?? state); }
  return {prepare, publish, check, replay, ledger};
}

test('both feed entry points compile to ZKIRv3', () => {
  for (const circuit of ['publishFeedUpdate', 'checkFeed']) {
    const artifact = JSON.parse(readFileSync(join(compileDir, 'zkir', circuit + '.zkir'), 'utf8'));
    assert.equal(artifact.version.major, 3, circuit);
  }
});

test('signed feed-AB updates support both inclusive freshness endpoints', async () => {
  const feed = await fixture();
  await feed.publish(100n, feed.prepare(100n));
  assert.equal(feed.ledger().feedPrice, 2n);
  assert.equal(feed.ledger().feedObservedAt, 100n);
  for (const time of [100n, 130n]) feed.replay(await feed.check(time), time);
});

test('outage, future and stale samples refuse on the executing ledger clock', async () => {
  const feed = await fixture();
  await assert.rejects(() => feed.check(100n), /FEED_UNAVAILABLE/);
  await assert.rejects(() => feed.publish(100n, feed.prepare(101n)), /FEED_FUTURE/);
  await feed.publish(100n, feed.prepare(100n));
  await assert.rejects(() => feed.check(99n), /FEED_FUTURE/);
  await assert.rejects(() => feed.check(131n), /FEED_STALE/);
  const lastGood = await feed.check(130n);
  assert.throws(() => feed.replay(lastGood, 131n), /mismatch between expected .* and actual .* read/);
});

test('substitution, caller flags, replay and altered signed data cannot update state', async () => {
  const feed = await fixture();
  const update = feed.prepare(100n);
  await assert.rejects(() => feed.publish(100n, {...update, signature: {authentic: true}}));
  await assert.rejects(() => feed.publish(100n, feed.prepare(100n, {}, 43n)), /FEED_SIGNATURE/);
  await assert.rejects(() => feed.publish(100n, feed.prepare(100n, {feed: feedId('feed-BA')})), /FEED_ID/);
  await assert.rejects(() => feed.publish(100n, update, rt.sampleContractAddress()), /FEED_SIGNATURE/);
  const otherNetwork = feed.prepare(100n, {}, 42n, feedId('other-network'));
  await assert.rejects(() => feed.publish(100n, otherNetwork), /FEED_SIGNATURE/);
  for (const change of [{price: 1n}, {observedAt: 99n}]) {
    const changed = {...update, observation: {...update.observation, ...change}};
    await assert.rejects(() => feed.publish(100n, changed), /FEED_SIGNATURE/);
  }
  assert.equal(feed.ledger().feedAvailable, false);
  await feed.publish(100n, update);
  await assert.rejects(() => feed.publish(100n, update), /FEED_SEQUENCE/);
  await assert.rejects(() => feed.check(100n, feedId('feed-BA')), /FEED_ID/);
  assert.equal(feed.ledger().feedObservedAt, 100n);
});

test('a valid prepared trigger fails transcript replay after a signed price change', async () => {
  const feed = await fixture();
  await feed.publish(100n, feed.prepare(100n));
  const prepared = await feed.check(100n);
  feed.replay(prepared, 100n);
  await feed.publish(101n, feed.prepare(101n, {price: 3n}));
  assert.equal(feed.ledger().feedPrice, 3n);
  assert.throws(() => feed.replay(prepared, 101n), /mismatch between expected .* and actual .* read/);
  await assert.rejects(() => feed.check(101n), /FEED_THRESHOLD/);
});
