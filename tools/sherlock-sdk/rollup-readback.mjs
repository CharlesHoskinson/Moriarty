// Independent read-only historical action/current-state checks. No wallet or proof generation.
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync} from 'node:fs';
import {resolve, join} from 'node:path';
import {pathToFileURL} from 'node:url';
import {createHash} from 'node:crypto';
import {sdkLoader, endpoints} from './stagenet-session.mjs';
import {anchorView, bindBatch, assertPredecessor, assertSettled} from './rollup-statement.mjs';
import {validateIndexedCall} from './rollup-validate-indexed.mjs';
const [runArg, output] = process.argv.slice(2);
assert.ok(output);
const run = resolve(runArg), read = path => JSON.parse(readFileSync(path));
const sha = bytes => createHash('sha256').update(bytes).digest('hex');
const ready = read(join(run, 'ready.json'));
assert.equal(ready.schema, 'sherlock-b4-local-preparation/1');
assert.equal(ready.decider_key_bytes_match_b3, true);
assert.equal(sha(readFileSync(ready.native_validator)), ready.native_validator_sha256);
const load = sdkLoader(ready.sdk, join(run, 'readback-sdk-imports'));
const rt = await load('@midnight-ntwrk/compact-runtime');
const ledger = await load('@midnight-ntwrk/midnight-js-protocol/ledger');
const mod = await import(pathToFileURL(resolve(ready.build, 'contract/index.js')).href);
const keyManifest = read(join(ready.key, 'manifest.json'));
const bound = bindBatch(read(join(ready.b3, 'leaf.instance.json')), read(join(ready.b3, 'parent.instance.json')), keyManifest, rt);
const receipts = Object.fromEntries(['deploy', 'acceptValidity', 'settleValidity'].map(name => [name, read(join(run, name, 'receipt.json'))]));
const deployment = receipts.deploy;
assert.equal(deployment.schema, 'sherlock-rollup-stagenet-deployment/1');
assert.match(deployment.address, /^[0-9a-f]{64}$/);
assert.equal(deployment.sourceSHA256, sha(readFileSync(new URL('./rollup-anchor.compact', import.meta.url))));
assert.equal(deployment.keys.acceptValidity.sha256, keyManifest.tagged_vk_sha256);
const actions = {};
const indexedTransactions = {}, nativeReadback = {};
const queryIndexer = async query => {
  const response = await fetch(endpoints.indexer, {method: 'POST', signal: AbortSignal.timeout(30000),
    headers: {'content-type': 'application/json'}, body: JSON.stringify({query})});
  assert.ok(response.ok); const body = await response.json(); assert.ok(!body.errors); return body.data;
};
for (const [name, receipt] of Object.entries(receipts)) {
  assert.equal(receipt.address, deployment.address); assert.equal(receipt.network, 'stagenet');
  assert.equal(receipt.status, 'SucceedEntirely'); assert.equal(receipt.completeEffectsMatch, true);
  assert.match(receipt.txHash, /^[0-9a-f]{64}$/);
  if (name !== 'deploy') {
    assert.equal(receipt.schema, 'sherlock-rollup-stagenet-call/1');
    assert.equal(receipt.circuit, name); assert.equal(receipt.batch, String(bound.batch));
    assert.equal(receipt.stmt, keyManifest.statement);
    assert.deepEqual(receipt.keys, deployment.keys);
    const native = read(join(run, name, 'native-final-check.json'));
    assert.equal(native.proofVerification, 'native/proof-verifying');
    assert.equal(native.contractProofsSignaturesLimitsAccepted, true); assert.equal(native.applied, true);
    assert.equal(native.contractCalls.length, 1);
    assert.equal(native.contractCalls[0].address, deployment.address);
    assert.equal(native.contractCalls[0].entryPoint, name); assert.equal(native.contractCalls[0].proofVersion, 'V3');
    if (name === 'acceptValidity') {
      assert.equal(native.contractCalls[0].proofSHA256, sha(readFileSync(join(run, name, 'proof/terminal.proof'))));
      assert.deepEqual(native.contractCalls[0].publicInputs, read(join(run, name, 'proof/terminal.public.json')));
    }
  }
  // SDK transaction IDs may carry a 0x prefix. The chain hash is canonical and
  // the hash-offset query was independently exercised against a finalized call.
  const rawQuery = `transactions(offset:{hash:"${receipt.txHash}"}){hash raw block{timestamp} ... on RegularTransaction{transactionResult{status}}}`;
  const body = await queryIndexer(`{contractAction(address:"${deployment.address}",offset:{transactionOffset:{hash:"${receipt.txHash}"}}){__typename state transaction{hash block{height hash}} ... on ContractCall{entryPoint}} ${rawQuery}}`);
  const action = body.contractAction; assert.ok(action);
  const transaction = body.transactions.find(tx => tx.hash === receipt.txHash); assert.ok(transaction);
  assert.equal(transaction.transactionResult.status, 'SUCCESS');
  indexedTransactions[name] = transaction;
  assert.equal(action.__typename, name === 'deploy' ? 'ContractDeploy' : 'ContractCall');
  if (name !== 'deploy') assert.equal(action.entryPoint, name);
  assert.equal(action.transaction.hash, receipt.txHash);
  assert.equal(action.transaction.block.height, receipt.blockHeight);
  assert.equal(action.transaction.block.hash, receipt.blockHash);
  const bytes = Buffer.from(action.state, 'hex'); assert.equal(sha(bytes), receipt.postStateSHA256);
  const state = ledger.ContractState.deserialize(bytes), view = anchorView(rt, mod, state);
  for (const [entry, pin] of Object.entries(deployment.keys)) assert.equal(sha(state.operation(entry).verifierKey), pin.sha256);
  if (name === 'settleValidity') assertSettled(view, bound);
  else {assertPredecessor(view, bound); assert.equal(view.validity_ok.member(bound.statement), name === 'acceptValidity');}
  if (name !== 'deploy') {
    assert.equal(sha(Buffer.from(transaction.raw, 'hex')), read(join(run, name, 'native-final-check.json')).transactionSHA256,
      'indexed transaction differs from the exact native-verified proof/statement artifact');
    // A fresh process verifies the indexed proof bytes against the actual
    // historical pre-state. No submitter receipt is the verification oracle.
    const history = await queryIndexer(`{contract(address:"${deployment.address}",offset:{height:${receipt.blockHeight - 1}}){state}
      block(offset:{height:${receipt.blockHeight}}){transactions{hash contractActions{address}}}}`);
    const here = history.block.transactions.flatMap(tx => tx.contractActions
      .filter(a => a.address === deployment.address).map(() => tx.hash));
    assert.deepEqual(here, [receipt.txHash], 'ambiguous same-block contract pre-state');
    assert.ok(history.contract); const pre = Buffer.from(history.contract.state, 'hex');
    assert.equal(sha(pre), receipt.preStateSHA256);
    const fresh = await validateIndexedCall({ledger, validator: ready.native_validator,
      deploymentRaw: Buffer.from(indexedTransactions.deploy.raw, 'hex'), preStateRaw: pre,
      transactionRaw: Buffer.from(transaction.raw, 'hex'), postStateRaw: bytes,
      blockSeconds: Math.floor(new Date(transaction.block.timestamp).getTime() / 1000),
      address: deployment.address, entry: name, logPath: join(run, name, 'native-readback.log')});
    const archived = read(join(run, name, 'native-final-check.json'));
    assert.deepEqual(fresh.contractCalls, archived.contractCalls);
    nativeReadback[name] = fresh;
  }
  actions[name] = action;
}
assert.equal(receipts.acceptValidity.preStateSHA256, deployment.postStateSHA256);
assert.equal(receipts.settleValidity.preStateSHA256, receipts.acceptValidity.postStateSHA256);
assert.notEqual(receipts.acceptValidity.txHash, receipts.settleValidity.txHash);
// Confirm the current index head also retains the complete settled state.
const latest = await queryIndexer(`{contractAction(address:"${deployment.address}"){state}}`);
assert.ok(latest.contractAction);
const current = ledger.ContractState.deserialize(Buffer.from(latest.contractAction.state, 'hex'));
assert.equal(sha(current.serialize()), receipts.settleValidity.postStateSHA256);
assertSettled(anchorView(rt, mod, current), bound);
const result = {schema: 'sherlock-b4-readback/1', network: 'stagenet', address: deployment.address,
  batch: String(bound.batch), transactions: 16, headers: 50, currentStateSHA256: sha(current.serialize()),
  completeEffectsMatch: true, nativeProofsVerified: true, receipts, actions, nativeReadback,
  nativeVerificationScope: 'fresh native contract proofs, signatures, limits and effects; isolated ledger waives fees and native token proofs'};
writeFileSync(output, JSON.stringify(result, null, 2) + '\n');
console.log('PASS B4 readback: exact public batch, both finalized calls, all settled fields and verifier keys');
