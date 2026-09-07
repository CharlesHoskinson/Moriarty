// Read-only verification of the Preview hello-world deployment and latest call.
import fs from 'node:fs/promises';
import path from 'node:path';
import { createRequire } from 'node:module';
import { fileURLToPath, pathToFileURL } from 'node:url';
const repo = fileURLToPath(new URL('../..', import.meta.url));
const runtime = path.join(repo, '.worktrees/r3-native/experiments/moriarty-midnight-network/hello-world');
const require = createRequire(path.join(runtime, 'package.json'));
globalThis.WebSocket = require('ws').WebSocket;
const { indexerPublicDataProvider } = await import(pathToFileURL(path.join(runtime, 'node_modules/@midnight-ntwrk/midnight-js-indexer-public-data-provider/dist/index.mjs')).href);
const { ledger } = await import(pathToFileURL(path.join(runtime, 'contracts/managed/hello-world/contract/index.js')).href);
const address = process.argv[2];
if (!/^[0-9a-f]{64}$/.test(address ?? '')) throw Error('Provide the deployed 64-hex Preview contract address');
const expectedMessage = 'Moriarty Preview settlement test';
const indexer = 'https://indexer.preview.midnight.network/api/v4/graphql';
const node = 'https://rpc.preview.midnight.network';
async function request(url, body) {
  const response = await fetch(url, { method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify(body), signal: AbortSignal.timeout(20_000) });
  if (!response.ok) throw Error(`HTTP ${response.status}`);
  const result = await response.json();
  if (result.errors || result.error) throw Error(JSON.stringify(result));
  return result;
}
const rpc = async (method, params = []) => (await request(node, { jsonrpc: '2.0', id: 1, method, params })).result;
if (await rpc('system_chain') !== 'Midnight Preview') throw Error('Unexpected network');
const txFields = 'hash block { height hash } ... on RegularTransaction { identifiers transactionResult { status } }';
const query = `query($address: HexEncoded!) { contractAction(address: $address) { __typename ... on ContractCall { address transaction { ${txFields} } deploy { address transaction { ${txFields} } } } } }`;
const action = (await request(indexer, { query, variables: { address } })).data?.contractAction;
if (action?.__typename !== 'ContractCall' || action.address !== address || action.deploy?.address !== address) throw Error('Expected deployed contract with a settled call');
const finalizedHash = await rpc('chain_getFinalizedHead');
const header = await rpc('chain_getHeader', [finalizedHash]);
const finalizedHeight = Number.parseInt(header?.number, 16);
if (!Number.isSafeInteger(finalizedHeight)) throw Error('Invalid finality response');
const transactions = [];
for (const [kind, tx] of [['deploy', action.deploy.transaction], ['call', action.transaction]]) {
  if (tx.transactionResult?.status !== 'SUCCESS') throw Error(`${kind} indexer result is not SUCCESS`);
  const canonical = await rpc('chain_getBlockHash', [tx.block.height]);
  if (canonical?.replace(/^0x/, '') !== tx.block.hash || tx.block.height > finalizedHeight) throw Error(`${kind} block not verified on finalized canonical chain`);
  transactions.push({ kind, ...tx, canonicalNodeHash: canonical, finalized: true });
}
const provider = indexerPublicDataProvider(indexer, 'wss://indexer.preview.midnight.network/api/v4/graphql/ws');
const state = await provider.queryContractState(address);
if (!state) throw Error('Contract state absent');
const message = Buffer.from(ledger(state.data).message).toString();
if (message !== expectedMessage) throw Error(`Unexpected ledger message: ${message}`);
const receipt = { checkedUtc: new Date().toISOString(), network: 'preview', contractAddress: address, transactions, finality: { finalizedHeight, finalizedHash }, readBack: message, result: 'PASS', scope: 'Public Preview Compact hello-world deploy/call and state readback; not Moriarty PCD or native financial proof' };
const name = `settlement-${receipt.checkedUtc.replace(/[:.]/g, '-')}.json`;
await fs.writeFile(path.join(repo, 'evidence/midnight-preview-2026-09-07', name), JSON.stringify(receipt, null, 2) + '\n', { flag: 'wx' });
console.log(JSON.stringify(receipt, null, 2));
process.exit(0);
