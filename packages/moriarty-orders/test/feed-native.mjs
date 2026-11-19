import assert from 'node:assert/strict';
import {execFileSync} from 'node:child_process';
import {mkdirSync, readFileSync, writeFileSync} from 'node:fs';
import {join, resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {signFeedUpdate} from '../src/feed.mjs';
import {generatorRecord, retainedTransactions} from '../native/record.mjs';
import {retainedRefusals} from '../native/refusals.mjs';
import {NativeLedger, hash} from './native-ledger.mjs';

const feed = Uint8Array.from(Buffer.from('feed-AB'.padEnd(32, '\0')));
const sha = path => hash(readFileSync(path));
const json = value => JSON.stringify(value, (_k, v) => typeof v === 'bigint' ? v.toString() : v, 2) + '\n';

export async function runNativeFeed(h, contract, observed) {
  const secret = 42n; // Public local fixture.
  const network = Uint8Array.from(Buffer.from(hash(h.network), 'hex'));
  const controller = h.runtime.ecMulGenerator(secret);
  const instance = await h.deploy(contract, [controller, network], 'deploy-feed-AB', {maintainers: [h.actors.owner]});
  observed.maintenance = instance.maintenance;
  const address = {bytes: h.runtime.encodeContractAddress(instance.address)};
  const date = BigInt(h.time);
  observed.contractAddress = instance.address;
  observed.network = h.network;
  observed.publicController = controller;
  observed.deploymentNetworkTag = Buffer.from(network).toString('hex');
  function update(price, observedAt, name = feed, key = secret) {
    const sample = {feed: name, price, observedAt};
    const signature = signFeedUpdate(h.runtime, contract.generated.pureCircuits, {network, address}, sample, key);
    return [sample, signature];
  }
  function read() {
    const native = h.state.index(instance.address);
    const archived = h.runtime.ContractState.deserialize(native.serialize());
    const current = contract.generated.ledger(archived.data);
    return {available: current.feedAvailable, observedAt: current.feedObservedAt, price: current.feedPrice};
  }
  async function refuses(label, circuit, args, expectedError) {
    const row = await h.refuseCall(instance, circuit, args, expectedError, label);
    observed.refusals.push({label, stage: row.boundary, error: row.observedError, ledgerSha256: row.stateSha256});
  }
  await refuses('missing', 'checkFeed', [feed], /FEED_UNAVAILABLE/);
  await refuses('wrong controller', 'publishFeedUpdate', update(2n, date, feed, 43n), /FEED_SIGNATURE/);
  const substituted = Uint8Array.from(Buffer.from('feed-BA'.padEnd(32, '\0')));
  await refuses('substitution', 'publishFeedUpdate', update(2n, date, substituted), /FEED_ID/);
  await refuses('future', 'publishFeedUpdate', update(2n, date + 1n), /FEED_FUTURE/);
  await h.call(instance, 'publishFeedUpdate', update(2n, date), 'publish-price-2');
  assert.deepEqual(read(), {available: true, observedAt: date, price: 2n});
  observed.afterPublication = read();
  h.time = Number(date);
  const prepared = await h.prepareCall(instance, 'checkFeed', [feed]);
  const original = await h.fundAndProve(h.assemble([prepared]), contract);
  const snapshot = h.snapshot();
  // Reuse the exact signed transaction on each isolated fork from this snapshot.
  h.apply(original, 'fresh-age-0');
  h.restore(snapshot);
  h.time += 30;
  h.apply(original, 'fresh-age-30');
  h.restore(snapshot);
  h.time += 31;
  await refuses('stale', 'checkFeed', [feed], /FEED_STALE/);
  function refusePrepared(label) {
    const result = h.apply(original, label, {allowFailure: true});
    assert.notEqual(result.status, 'success');
    assert.match(json(result.error), /mismatch.*read/is);
  }
  refusePrepared('unchanged-transaction-age-31');
  h.restore(snapshot);
  h.reserveFunding(original);
  try {
    h.time++;
    await h.call(instance, 'publishFeedUpdate', update(3n, date + 1n), 'publish-price-3');
  } finally {
    h.clearReservations();
  }
  h.time = Number(date) + 1;
  const before = read();
  assert.deepEqual(before, {available: true, observedAt: date + 1n, price: 3n});
  await refuses('threshold equality', 'checkFeed', [feed], /FEED_THRESHOLD/);
  refusePrepared('unchanged-transaction-new-price');
  assert.deepEqual(read(), before);
  observed.afterChangedCondition = read();
  // Control: a newer sample at the same price invalidates the unchanged transaction too, while a
  // fresh check at that second passes. Only the price refuses a fresh check.
  h.restore(snapshot);
  h.reserveFunding(original);
  try {
    h.time++;
    await h.call(instance, 'publishFeedUpdate', update(2n, date + 1n), 'publish-newer-price-2');
  } finally {
    h.clearReservations();
  }
  h.time = Number(date) + 1;
  refusePrepared('unchanged-transaction-newer-same-price');
  const fresh = await h.prepareCall(instance, 'checkFeed', [feed]);
  h.apply(await h.fundAndProve(h.assemble([fresh]), contract), 'refreshed-check-same-price');
  observed.afterSameSampleControl = read();
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const [build, output, proofServer] = process.argv.slice(2);
  if (!build || !output || !proofServer || !process.env.MORIARTY_ORDER_MODULES) {
    throw Error('usage: MORIARTY_ORDER_MODULES=MODULES node feed-native.mjs BUILD OUTPUT PROOF_SERVER');
  }
  mkdirSync(output, {recursive: true});
  const observed = {requirement: 'II.5', factoryPredicate: 'F28', target: 'Midnight strict native-local ledger',
    command: ['node', 'packages/moriarty-orders/test/feed-native.mjs', build, output, proofServer],
    generator: generatorRecord(['node', 'packages/moriarty-orders/test/feed-native.mjs', build, output, proofServer]),
    node: process.version, assessment: 'independent review pending', records: [], refusals: [],
    sources: Object.fromEntries(['contracts/feed.compact','contracts/feed-policy.compact','src/feed.mjs',
      'test/feed-native.mjs','test/native-ledger.mjs','test/feed.test.mjs'].map(name => [name, sha(join('packages/moriarty-orders',name))])),
    artifacts: Object.fromEntries(['publishFeedUpdate','checkFeed'].map(name => [name, {
      zkir: sha(join(build,'zkir',name+'.zkir')), verifier: sha(join(build,'keys',name+'.verifier'))
    }]))};
  const writeTransaction = retainedTransactions(output);
  function retain(record) {
    writeTransaction(record);
    observed.records.push({label: record.label, status: record.status, transactionHash: record.transactionHash,
      transactionSha256: record.serializedTransactionSha256});
  }
  try {
    const ledger = await NativeLedger.create({modules: process.env.MORIARTY_ORDER_MODULES, proofServer, retain, refusals: retainedRefusals(output)});
    observed.compiler = JSON.parse(readFileSync(join(build,'compiler/contract-info.json'),'utf8'))['compiler-version'];
    observed.runtime = JSON.parse(readFileSync(join(process.env.MORIARTY_ORDER_MODULES,'@midnight-ntwrk/compact-runtime/package.json'),'utf8')).version;
    observed.proofServerVersion = (await (await fetch(new URL('/version',proofServer))).text()).trim();
    assert.equal(observed.proofServerVersion, '9.0.1');
    observed.assumptions = ['Controller price, signing time and availability', 'Ledger block time and ordering', 'Deployment and verifier governance'];
    observed.boundary = 'Signed and proved transactions on the V9 WASM LedgerState. Run native/check-feed.mjs II.5 for the independent Rust validation, application and readback.';
    const contract = await ledger.loadContract(build);
    await runNativeFeed(ledger, contract, observed);
    observed.status = 'passed';
    observed.receipts = {file:'receipts.json',sha256:sha(join(output,'receipts.json'))};
  } catch (error) {
    observed.status = 'failed';
    observed.error = String(error?.message ?? error);
    throw error;
  } finally {
    writeFileSync(join(output,'result.json'), json(observed));
  }
}
