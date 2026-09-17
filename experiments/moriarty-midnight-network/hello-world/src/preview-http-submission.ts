import { ApiPromise, HttpProvider } from '@polkadot/api';
import type { FinalizedTransaction } from '@midnight-ntwrk/midnight-js-protocol/ledger';
import type { SubmissionService, SubmissionEvent, SubmitTransactionMethod } from '@midnight-ntwrk/wallet-sdk/capabilities/submission';
import type { NetworkConfig } from './network';

const NODE = 'https://rpc.preview.midnight.network';
const INDEXER = 'https://indexer.preview.midnight.network/api/v4/graphql';
const hash = (value: unknown): string => {
  if (typeof value !== 'string' || !/^(0x)?[0-9a-fA-F]{64}$/.test(value)) throw Error('PREVIEW_HTTP_INVALID_HASH');
  return value.replace(/^0x/, '').toLowerCase();
};

/** Injected transports are for offline tests; production always uses the official Preview endpoints. */
export interface PreviewHttpTransport {
  submit(bytes: Uint8Array, signal: AbortSignal): Promise<string>;
  rpc(method: string, params: unknown[]): Promise<any>;
  query(identifier: string, signal: AbortSignal): Promise<any[]>;
  close(): Promise<void>;
}

function transport(): PreviewHttpTransport {
  const provider = new HttpProvider(NODE);
  let api: ApiPromise | undefined;
  return {
    async submit(bytes, signal) {
      api = new ApiPromise({ provider });
      await api.isReadyOrError;
      signal.throwIfAborted();
      return (await api.tx.midnight.sendMnTransaction('0x' + Buffer.from(bytes).toString('hex')).send()).toHex();
    },
    rpc: (method, params) => provider.send(method, params),
    async query(identifier, signal) {
      const response = await fetch(INDEXER, {
        method: 'POST', redirect: 'error', signal,
        headers: { 'content-type': 'application/json' },
        body: JSON.stringify({ query: `query { transactions(offset: { identifier: "${identifier}" }) { hash block { height hash } ... on RegularTransaction { identifiers transactionResult { status } } } }` }),
      });
      if (!response.ok) throw Error('PREVIEW_HTTP_INDEXER');
      const body = await response.json();
      if (body.errors || !Array.isArray(body.data?.transactions)) throw Error('PREVIEW_HTTP_INDEXER');
      return body.data.transactions;
    },
    async close() { if (api) await api.disconnect(); else await provider.disconnect(); },
  };
}

/** Opt-in repair for Preview HTTP submission. Pool acceptance alone is never finality. */
export function previewHttpSubmissionService(
  config: NetworkConfig,
  options: { timeoutMs?: number; pollMs?: number; transport?: () => PreviewHttpTransport } = {},
): SubmissionService<FinalizedTransaction> {
  if (config.networkId !== 'preview' || config.node.replace(/\/$/, '') !== NODE || config.indexer !== INDEXER) {
    throw Error('PREVIEW_HTTP_NETWORK');
  }
  const timeoutMs = options.timeoutMs ?? 180_000;
  const pollMs = options.pollMs ?? 2_000;
  if (!Number.isSafeInteger(timeoutMs) || timeoutMs < 1 || timeoutMs > 600_000 ||
      !Number.isSafeInteger(pollMs) || pollMs < 1 || pollMs > 10_000) throw Error('PREVIEW_HTTP_BOUNDS');
  const active = new Set<AbortController>();
  let closed = false;
  const submit = async (transaction: FinalizedTransaction, waitFor: 'Submitted' | 'InBlock' | 'Finalized' = 'InBlock'): Promise<SubmissionEvent> => {
    if (closed) throw Error('PREVIEW_HTTP_CLOSED');
    if (!['Submitted', 'InBlock', 'Finalized'].includes(waitFor)) throw Error('PREVIEW_HTTP_STATUS');
    const controller = new AbortController();
    active.add(controller);
    const timer = setTimeout(() => controller.abort(), timeoutMs);
    let channel: PreviewHttpTransport | undefined;
    // A transport timeout is ambiguous: callers must observe the transaction before retrying.
    const stopped = new Promise<never>((_, reject) => controller.signal.addEventListener('abort', () => reject(Error('PREVIEW_HTTP_UNKNOWN_FINALITY')), { once: true }));
    void stopped.catch(() => {});
    const within = <T>(operation: Promise<T>): Promise<T> => Promise.race([operation, stopped]);
    try {
      const identifiers = [...transaction.identifiers()];
      if (!identifiers.length || !identifiers.every(id => /^(?:[0-9a-f]{64}|[0-9a-f]{66})$/.test(id))) throw Error('PREVIEW_HTTP_IDENTITY');
      const transactionHash = hash(transaction.transactionHash());
      const tx = transaction.serialize() as SubmissionEvent['tx'];
      channel = (options.transport ?? transport)();
      if (await within(channel.rpc('system_chain', [])) !== 'Midnight Preview') throw Error('PREVIEW_HTTP_NETWORK');
      const txHash = '0x' + hash(await within(channel.submit(tx, controller.signal)));
      if (waitFor === 'Submitted') return { _tag: 'Submitted', tx, txHash };
      for (;;) {
        const rows = await within(channel.query(identifiers[0], controller.signal));
        const matching = rows.filter(row => hash(row.hash) === transactionHash);
        if (matching.length > 1) throw Error('PREVIEW_HTTP_INDEXER');
        const row = matching[0];
        if (row) {
          if (!Array.isArray(row.identifiers) || !identifiers.every(id => row.identifiers.includes(id))) throw Error('PREVIEW_HTTP_IDENTITY');
          if (row.transactionResult?.status !== 'SUCCESS') throw Error('PREVIEW_HTTP_LEDGER_REJECTED');
          const height = row.block?.height;
          const blockHash = '0x' + hash(row.block?.hash);
          if (!Number.isSafeInteger(height) || height < 0) throw Error('PREVIEW_HTTP_BLOCK');
          if (hash(await within(channel.rpc('chain_getBlockHash', [height]))) !== hash(blockHash)) throw Error('PREVIEW_HTTP_NONCANONICAL');
          if (waitFor === 'InBlock') return { _tag: 'InBlock', tx, txHash, blockHash, blockHeight: BigInt(height) };
          const finalHash = '0x' + hash(await within(channel.rpc('chain_getFinalizedHead', [])));
          const header = await within(channel.rpc('chain_getHeader', [finalHash]));
          if (typeof header?.number !== 'string' || !/^0x[0-9a-f]+$/i.test(header.number)) throw Error('PREVIEW_HTTP_BLOCK');
          if (BigInt(header.number) >= BigInt(height)) {
            // Recheck after the finality observation to avoid accepting a replaced indexed block.
            if (hash(await within(channel.rpc('chain_getBlockHash', [height]))) !== hash(blockHash)) throw Error('PREVIEW_HTTP_NONCANONICAL');
            return { _tag: 'Finalized', tx, txHash, blockHash, blockHeight: BigInt(height) };
          }
        }
        await within(new Promise(resolve => setTimeout(resolve, pollMs)));
      }
    } catch (error) {
      const code = error instanceof Error && /^PREVIEW_HTTP_[A-Z_]+$/.test(error.message) ? error.message : 'PREVIEW_HTTP_SUBMISSION_OR_OBSERVATION_FAILED';
      // Never retain provider causes: they can contain full transaction bytes or witness data.
      throw Error(code);
    } finally {
      clearTimeout(timer);
      active.delete(controller);
      controller.abort();
      // Cleanup must not extend the operation deadline or expose provider errors.
      if (channel) void channel.close().catch(() => {});
    }
  };
  return {
    submitTransaction: submit as SubmitTransactionMethod<FinalizedTransaction>,
    async close() { closed = true; for (const controller of active) controller.abort(); },
  };
}

export function previewSubmissionFactory(config: NetworkConfig, selection = process.env.MIDNIGHT_SUBMISSION_TRANSPORT) {
  if (selection === undefined || selection === '' || selection === 'default') return undefined;
  if (selection !== 'preview-http') throw Error('Unsupported MIDNIGHT_SUBMISSION_TRANSPORT');
  return () => previewHttpSubmissionService(config);
}
