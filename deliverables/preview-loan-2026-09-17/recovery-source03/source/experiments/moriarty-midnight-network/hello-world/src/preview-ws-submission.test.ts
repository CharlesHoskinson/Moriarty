import { mock, test } from 'node:test';
import assert from 'node:assert/strict';
import type { FinalizedTransaction } from '@midnight-ntwrk/midnight-js-protocol/ledger';
import type { NetworkConfig } from './network';

const calls: string[] = [];
let connectionError = false;
let delayedOpen = false;
let openSocket: () => void;

class FakeWsProvider {
  isReady = delayedOpen ? new Promise<void>(resolve => { openSocket = resolve; }) : Promise.resolve();
  constructor(url: string, reconnect: false) { assert.equal(url, 'wss://rpc.preview.midnight.network/'); assert.equal(reconnect, false); calls.push('ws'); }
  async connect() { calls.push('connect'); if (connectionError) throw Error('private provider error'); }
  async send(method: string) { calls.push(method); return 'Midnight Preview'; }
  async disconnect() { calls.push('disconnect'); }
}
class FakeHttpProvider { constructor() { throw Error('Unexpected HTTP transport'); } }
class FakeApi {
  isReadyOrError = Promise.resolve();
  provider: FakeWsProvider;
  constructor({ provider }: { provider: FakeWsProvider }) { this.provider = provider; calls.push('metadata'); }
  tx = { midnight: { sendMnTransaction: (hex: string) => {
    assert.equal(hex, '0x0102'); return { send: async () => { calls.push('submit'); return { toHex: () => '0x' + '12'.repeat(32) }; } };
  } } };
  async disconnect() { await this.provider.disconnect(); }
}
(mock as any).module('@polkadot/api', { namedExports: { ApiPromise: FakeApi, HttpProvider: FakeHttpProvider, WsProvider: FakeWsProvider } });
const { previewSubmissionFactory, previewHttpSubmissionService } = await import('./preview-http-submission');
const config = { networkId: 'preview', node: 'https://rpc.preview.midnight.network', indexer: 'https://indexer.preview.midnight.network/api/v4/graphql' } as NetworkConfig;
const tx = { identifiers: () => ['00' + 'ab'.repeat(32), '00' + 'cd'.repeat(32)], transactionHash: () => 'ef'.repeat(32), serialize: () => new Uint8Array([1, 2]) } as unknown as FinalizedTransaction;

test('selected WS production transport connects once and submits exact bytes without HTTP', async () => {
  calls.length = 0; connectionError = false;
  const service = previewSubmissionFactory(config, 'preview-ws')!();
  const result = await service.submitTransaction(tx, 'Submitted');
  assert.equal(result._tag, 'Submitted');
  assert.deepEqual(calls, ['ws', 'connect', 'system_chain', 'metadata', 'submit', 'disconnect']);
});
test('WS connection failure never constructs an extrinsic or reconnects', async () => {
  calls.length = 0; connectionError = true;
  const service = previewSubmissionFactory(config, 'preview-ws')!();
  await assert.rejects(service.submitTransaction(tx, 'Submitted'), { message: 'PREVIEW_HTTP_SUBMISSION_OR_OBSERVATION_FAILED' });
  assert.deepEqual(calls, ['ws', 'connect', 'disconnect']);
});

test('WS waits for socket open after connect resolves', async () => {
  calls.length = 0; connectionError = false; delayedOpen = true;
  const service = previewSubmissionFactory(config, 'preview-ws')!();
  const pending = service.submitTransaction(tx, 'Submitted');
  await new Promise(resolve => setTimeout(resolve, 10));
  assert.deepEqual(calls, ['ws', 'connect']);
  openSocket();
  assert.equal((await pending)._tag, 'Submitted');
  assert.equal(calls.filter(c => c === 'submit').length, 1);
  delayedOpen = false;
});
test('late socket open after timeout cannot submit', async () => {
  calls.length = 0; connectionError = false; delayedOpen = true;
  const service = previewHttpSubmissionService(config, {mode: 'ws', timeoutMs: 10});
  await assert.rejects(service.submitTransaction(tx, 'Submitted'), {message: 'PREVIEW_HTTP_UNKNOWN_FINALITY'});
  openSocket();
  await new Promise(resolve => setTimeout(resolve, 10));
  assert.equal(calls.includes('submit'), false);
  assert.equal(calls.includes('metadata'), false);
  assert.equal(calls.filter(c => c === 'connect').length, 1);
  delayedOpen = false;
});
