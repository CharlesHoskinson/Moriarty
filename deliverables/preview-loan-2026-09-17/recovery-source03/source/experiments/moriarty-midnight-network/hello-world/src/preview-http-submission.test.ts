import test from 'node:test';
import assert from 'node:assert/strict';
import { previewHttpSubmissionService, previewSubmissionFactory, type PreviewHttpTransport } from './preview-http-submission';
import { NETWORK_CONFIGS } from './network';
import type { FinalizedTransaction } from '@midnight-ntwrk/midnight-js-protocol/ledger';

const id = '00' + 'ab'.repeat(32), txHash = 'cd'.repeat(32), block = 'ef'.repeat(32), extrinsic = '12'.repeat(32);
function fixture() {
  const calls: string[] = [];
  const row = { hash: txHash, identifiers: [id], transactionResult: { status: 'SUCCESS' }, block: { height: 10, hash: block } };
  const tx = { identifiers: () => [id], transactionHash: () => txHash, serialize: () => new Uint8Array([1, 2]) } as unknown as FinalizedTransaction;
  const channel: PreviewHttpTransport = {
    async submit(bytes) { calls.push('submit'); assert.deepEqual([...bytes], [1, 2]); return '0x' + extrinsic; },
    async rpc(method) { calls.push(method); return method === 'system_chain' ? 'Midnight Preview' : method === 'chain_getHeader' ? { number: '0xa' } : '0x' + block; },
    async query(identifier) { calls.push('query'); assert.equal(identifier, id); return [row]; },
    async close() { calls.push('close'); },
  };
  const service = (timeoutMs = 100) => previewHttpSubmissionService(NETWORK_CONFIGS.preview, { transport: () => channel, timeoutMs, pollMs: 1 });
  return { calls, row, tx, channel, service };
}
for (const status of ['Submitted', 'InBlock', 'Finalized'] as const) test(`returns ${status} only after its required observations`, async () => {
  const f = fixture(); const event = await f.service().submitTransaction(f.tx, status);
  assert.equal(event._tag, status); assert.equal(event.txHash, '0x' + extrinsic);
  assert.equal(f.calls.filter(x => x === 'submit').length, 1);
  assert.equal(f.calls.includes('query'), status !== 'Submitted');
  assert.equal(f.calls.includes('chain_getFinalizedHead'), status === 'Finalized');
  assert.equal(f.calls.at(-1), 'close');
});
test('finality lag polls observations without resubmitting', async () => {
  const f = fixture(); const rpc = f.channel.rpc; let reads = 0;
  f.channel.rpc = async (m, p) => m === 'chain_getHeader' ? { number: ++reads === 1 ? '0x9' : '0xa' } : rpc(m, p);
  assert.equal((await f.service().submitTransaction(f.tx, 'Finalized'))._tag, 'Finalized');
  assert.equal(reads, 2); assert.equal(f.calls.filter(x => x === 'submit').length, 1);
});
test('multi-intent finality requires every native identifier', async () => {
  const other = '00' + '34'.repeat(32);
  const f = fixture();
  f.tx.identifiers = () => [id, other];
  f.row.identifiers = [id, other];
  assert.equal((await f.service().submitTransaction(f.tx, 'Finalized'))._tag, 'Finalized');
  assert.equal(f.calls.filter(x => x === 'submit').length, 1);
  const missing = fixture();
  missing.tx.identifiers = () => [id, other];
  await assert.rejects(missing.service().submitTransaction(missing.tx, 'Finalized'), /IDENTITY/);
});
for (const status of ['FAILURE', 'PARTIAL_SUCCESS', 'unknown']) test(`rejects indexed ${status}`, async () => {
  const f = fixture(); f.row.transactionResult.status = status;
  await assert.rejects(f.service().submitTransaction(f.tx, 'Finalized'), /LEDGER_REJECTED/);
});
test('does not confuse another indexed transaction with the submitted one', async () => {
  const f = fixture(); f.row.hash = 'aa'.repeat(32);
  await assert.rejects(f.service(10).submitTransaction(f.tx, 'Finalized'), /UNKNOWN_FINALITY/);
  assert.equal(f.calls.filter(x => x === 'submit').length, 1);
});
test('canonical mismatch fails even after indexed SUCCESS', async () => {
  const f = fixture(); f.row.block.hash = 'aa'.repeat(32);
  await assert.rejects(f.service().submitTransaction(f.tx, 'Finalized'), /NONCANONICAL/);
});
test('canonical recheck rejects a block replaced across the finality observation', async () => {
  const f = fixture(); const rpc = f.channel.rpc; let checks = 0;
  f.channel.rpc = async (m, p) => m === 'chain_getBlockHash' && ++checks === 2 ? '0x' + 'aa'.repeat(32) : rpc(m, p);
  await assert.rejects(f.service().submitTransaction(f.tx, 'Finalized'), /NONCANONICAL/);
});
test('submission errors strip transaction bytes and do not retry', async () => {
  const f = fixture(); let submissions = 0;
  f.channel.submit = async () => { submissions++; throw Error('secret raw transaction witness'); };
  await assert.rejects(f.service().submitTransaction(f.tx), { message: 'PREVIEW_HTTP_SUBMISSION_OR_OBSERVATION_FAILED' });
  assert.equal(submissions, 1);
});
test('hanging submit is bounded and never reported as finalized', async () => {
  const f = fixture(); f.channel.submit = () => new Promise(() => {});
  await assert.rejects(f.service(10).submitTransaction(f.tx, 'Finalized'), /UNKNOWN_FINALITY/);
  assert.equal(f.calls.at(-1), 'close');
});
test('close aborts observation and prevents future submissions', async () => {
  const f = fixture(); let queried!: () => void;
  const ready = new Promise<void>(resolve => { queried = resolve; });
  f.channel.query = async () => { queried(); return new Promise(() => {}); };
  const service = f.service(); const pending = service.submitTransaction(f.tx, 'Finalized');
  await ready; await service.close();
  await assert.rejects(pending, /UNKNOWN_FINALITY/);
  await assert.rejects(service.submitTransaction(f.tx), /CLOSED/);
});
test('explicit network selection preserves historical SDK defaults', async () => {
  assert.equal(previewSubmissionFactory(NETWORK_CONFIGS.undeployed, 'default'), undefined);
  assert.equal(previewSubmissionFactory(NETWORK_CONFIGS.preview, ''), undefined);
  assert.throws(() => previewSubmissionFactory(NETWORK_CONFIGS.preview, 'typo'), /Unsupported/);
  assert.throws(() => previewHttpSubmissionService(NETWORK_CONFIGS.preprod), /NETWORK/);
  const f = fixture(); f.channel.rpc = async () => 'Other Network';
  await assert.rejects(f.service().submitTransaction(f.tx), /NETWORK/);
  assert.equal(f.calls.includes('submit'), false);
});
