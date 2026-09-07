import { readFile } from 'node:fs/promises';

const indexerUrl = process.env.MN_INDEXER_URL ?? 'http://127.0.0.1:18088/api/v4/graphql';
const nodeUrl = process.env.MN_NODE_URL ?? 'http://127.0.0.1:19944';
const receiptsPath = new URL('../../evidence/moriarty-midnight-network-2026-09-07/local-receipts.json', import.meta.url);
const receipts = JSON.parse(await readFile(receiptsPath, 'utf8'));

const graphql = async (query) => {
  const response = await fetch(indexerUrl, {
    method: 'POST', headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ query }), signal: AbortSignal.timeout(10_000),
  });
  if (!response.ok) throw new Error(`Indexer HTTP ${response.status}: ${await response.text()}`);
  const body = await response.json();
  if (body.errors?.length) throw new Error(`Indexer GraphQL errors: ${JSON.stringify(body.errors)}`);
  if (!body.data) throw new Error(`Indexer response has no data: ${JSON.stringify(body)}`);
  return body.data;
};

const rpc = async (method, params = []) => {
  const response = await fetch(nodeUrl, {
    method: 'POST', headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ jsonrpc: '2.0', id: 1, method, params }),
    signal: AbortSignal.timeout(10_000),
  });
  if (!response.ok) throw new Error(`Node RPC HTTP ${response.status}: ${await response.text()}`);
  const body = await response.json();
  if (body.error) throw new Error(`Node RPC ${method} error: ${JSON.stringify(body.error)}`);
  if (body.result === undefined || body.result === null) throw new Error(`Node RPC ${method} missing result`);
  return body.result;
};

const normalizeHash = (hash) => hash.toLowerCase().replace(/^0x/, '');
const verified = [];
const checks = ['nightTransfer', 'dustRegistration', 'compactCall', 'dedicatedWalletCompactCall'];

for (const name of checks) {
  const expected = receipts[name];
  const query = `{ transactions(offset: { identifier: "${expected.walletTxIdentifier}" }) { hash block { height hash } ... on RegularTransaction { transactionResult { status } } } }`;
  const data = await graphql(query);
  const actual = data.transactions?.[0];
  if (data.transactions?.length !== 1 || actual?.hash !== expected.transactionHash ||
      actual?.block?.height !== expected.blockHeight || actual?.block?.hash !== expected.blockHash ||
      actual?.transactionResult?.status !== 'SUCCESS') {
    throw new Error(`${name} receipt mismatch: ${JSON.stringify(data)}`);
  }
  const nodeHash = await rpc('chain_getBlockHash', [actual.block.height]);
  if (normalizeHash(nodeHash) !== normalizeHash(actual.block.hash)) {
    throw new Error(`${name} node/indexer block hash mismatch: node=${nodeHash} indexer=${actual.block.hash}`);
  }
  verified.push({ name, transactionHash: actual.hash, blockHeight: actual.block.height,
    blockHash: actual.block.hash, indexerStatus: 'SUCCESS', nodeHashMatch: true });
}

for (const deployName of ['compactDeploy', 'dedicatedWalletCompactDeploy']) {
const deploy = receipts[deployName];
const deployQuery = `{ block(offset: { height: ${deploy.blockHeight} }) { hash transactions { hash ... on RegularTransaction { transactionResult { status } contractActions { __typename address } } } } }`;
const deployData = await graphql(deployQuery);
const deployTx = deployData.block?.transactions?.find((tx) =>
  tx.hash === deploy.transactionHash && tx.contractActions?.some((action) =>
    action.__typename === 'ContractDeploy' && action.address === deploy.contractAddress));
if (deployData.block?.hash !== deploy.blockHash || deployTx?.transactionResult?.status !== 'SUCCESS') {
  throw new Error(`${deployName} receipt mismatch: ${JSON.stringify(deployData)}`);
}
const deployNodeHash = await rpc('chain_getBlockHash', [deploy.blockHeight]);
if (normalizeHash(deployNodeHash) !== normalizeHash(deploy.blockHash)) {
  throw new Error(`${deployName} node/indexer block hash mismatch: node=${deployNodeHash} indexer=${deploy.blockHash}`);
}
verified.push({ name: deployName, transactionHash: deploy.transactionHash,
  blockHeight: deploy.blockHeight, blockHash: deploy.blockHash,
  indexerStatus: 'SUCCESS', nodeHashMatch: true });
}

const finalizedHash = await rpc('chain_getFinalizedHead');
const finalizedHeader = await rpc('chain_getHeader', [finalizedHash]);
const finalizedHeight = Number.parseInt(finalizedHeader.number, 16);
if (!Number.isSafeInteger(finalizedHeight)) throw new Error(`Invalid finalized height: ${finalizedHeader.number}`);
const highestReceipt = Math.max(...verified.map(({ blockHeight }) => blockHeight));
if (finalizedHeight < highestReceipt) throw new Error(`Finalized height ${finalizedHeight} is below receipt height ${highestReceipt}`);

console.log(JSON.stringify({
  verifiedAt: new Date().toISOString(), network: receipts.network, indexerUrl, nodeUrl,
  transactions: verified,
  finality: { finalizedHeight, finalizedHash, highestReceipt, coversAllReceipts: true },
}, null, 2));
