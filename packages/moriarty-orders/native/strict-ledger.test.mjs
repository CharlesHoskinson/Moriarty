import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {join, resolve} from 'node:path';
import {test} from 'node:test';
import {pathToFileURL} from 'node:url';

const modules = process.env.MORIARTY_ORDER_MODULES ?? resolve('tools/sherlock-sdk/node_modules');
const ledger = await import(pathToFileURL(join(modules, '@midnightntwrk/ledger-v9/midnight_ledger_wasm_v9_fs.js')));
const binary = process.env.MORIARTY_ORDER_VALIDATOR ?? resolve('target/debug/examples/validate_call');
const hash = data => createHash('sha256').update(data).digest('hex');

test('strict native reward replay agrees with the independently serialized SDK poststate', () => {
  const key = ledger.signingKeyFromBip340(new Uint8Array(32).fill(31));
  const vk = ledger.signatureVerifyingKey(key);
  const amount = 1_000_000_000_000n;
  const date = new Date(1000);
  const state = ledger.LedgerState.blank('undeployed').testingDistributeNight(ledger.addressFromKey(vk), amount, date);
  let claim = ledger.ClaimRewardsTransaction.new('undeployed', amount, vk, '19'.repeat(32), 'Reward');
  claim = claim.addSignature(ledger.signData(key, claim.dataToSign));
  const transaction = ledger.Transaction.fromRewards(claim);
  const verified = transaction.wellFormed(state, new ledger.WellFormedStrictness(), date);
  const parentBlockHash = '00'.repeat(32);
  const block = {secondsSinceEpoch: 1n, lastBlockTime: 1n, secondsSinceEpochErr: 0, parentBlockHash};
  const [post, sdkResult] = state.apply(verified, new ledger.TransactionContext(state, block));
  assert.equal(sdkResult.type, 'success');
  const input = {
    state: Buffer.from(state.serialize()).toString('hex'),
    transaction: Buffer.from(transaction.serialize()).toString('hex'),
    block_seconds: 1, block_context: {uncertainty_seconds: 0, last_block_seconds: 1, parent_block_hash: parentBlockHash},
  };
  const receipt = JSON.parse(execFileSync(binary, ['--strict'], {input: JSON.stringify(input)}));
  assert.equal(receipt.schema, 'moriarty-strict-ledger-replay/1');
  assert.equal(receipt.outcome, 'success');
  assert.deepEqual(receipt.blockContext, input.block_context);
  assert.equal(receipt.blockSeconds, 1);
  assert.equal(receipt.preStateSHA256, hash(state.serialize()));
  assert.equal(receipt.transactionSHA256, hash(transaction.serialize()));
  assert.equal(receipt.postStateSHA256, hash(post.serialize()));
  assert.equal(receipt.afterBlockStateSHA256, hash(post.postBlockUpdate(date).serialize()));
  assert.deepEqual(receipt.contractCalls, []);
});
