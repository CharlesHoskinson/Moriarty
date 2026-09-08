import {test} from 'node:test';
import assert from 'node:assert/strict';
import {runLocalFinancialCase} from '/home/charl/Moriarty/.worktrees/sp05-ledger-integration-grok/experiments/moriarty-midnight-financial/ledger/run-local.mjs';
import {validatePublicFinancialReceipt, decodePublicFinancialReceipt} from '/home/charl/Moriarty/.worktrees/sp05-ledger-integration-grok/experiments/moriarty-midnight-financial/ledger/decode-receipt.mjs';

// Offline diagnostic under the candidate's accepted input contract. These
// objects grant no operational admission and provide no blockchain evidence.
test('driver reaches deployment transport for input accepted by its own contract', async () => {
  const calls = [];
  const options = {
    case: 'loan',
    networkAdmission: {logicalTag: 'local', bound: true, observedProtocol: 'offline-test-only'},
    sourceManifest: {bindings: {time: {horizon: 2000000000}}},
    provenAssetManifest: {proven: false},
    walletContext: {facade: {}},
    roleCapabilities: {borrower: new Uint8Array(32), lender: new Uint8Array(32)},
    recipientAddresses: {borrower: '11'.repeat(32), lender: '22'.repeat(32)},
    blockTime: 1700000000,
    limits: {attempts: 1, deadlineMs: 1000, spend: 1n},
    operationalAdmission: {allowLocalExecution: true, reviewed: true,
      networkConfig: {indexer: 'mock://indexer', prover: 'mock://proof'}},
    privateStateLocation: '/tmp/moriarty-offline-unused',
    eventSink: () => {},
    adapters: {
      CompiledContract: {make: () => ({pipe() {return this;}})},
      NodeZkConfigProvider: class {},
      httpClientProofProvider: () => ({}),
      indexerPublicDataProvider: () => ({}),
      levelPrivateStateProvider: () => ({}),
      deployContract: async () => {calls.push('deploy'); throw new Error('TRANSPORT_REACHED_DEPLOY');},
    },
  };
  let result;
  try {result = await runLocalFinancialCase(options);}
  catch (error) {assert.match(error.message, /TRANSPORT_REACHED_DEPLOY/);}
  assert.deepEqual(calls, ['deploy'], `driver returned ${JSON.stringify(result)} without reaching deployment`);
});

test('a success flag without status or transaction evidence is rejected', () => {
  const forged = {contractAddress: 'aa'.repeat(32), blockHash: 'bb'.repeat(32), acceptedStage: true};
  assert.equal(validatePublicFinancialReceipt(forged).ok, false);
});

test('public state cannot overwrite finalized failure status', async () => {
  const receipt = await decodePublicFinancialReceipt({
    provenance: 'offline-transport-test',
    contractAddress: 'aa'.repeat(32),
    finalizedTxData: {status: 'FailEntirely', blockHash: 'bb'.repeat(32)},
    publicState: {txStatus: 'SucceedEntirely', acceptedStage: true},
  });
  assert.equal(receipt.acceptedStage, false);
  assert.equal(receipt.txStatus, 'FailEntirely');
});
