// Builds proof-server request bodies for the compiled Moriarty loan circuit
// `initialize`, fully offline: runs the circuit in compact-runtime 0.16.0 against a
// locally constructed contract state, then serialises the proof preimage with the
// ledger-v8 8.1.0 helpers. No wallet, node, indexer or network access.
import {readFileSync, writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import * as runtime from '@midnight-ntwrk/compact-runtime';
import * as ledger from '@midnight-ntwrk/ledger-v8';
import * as loan from './loan/contract/index.js';

const dir = new URL('.', import.meta.url).pathname;
const h = n => n.toString(16).padStart(64, '0');
const bytes = x => Uint8Array.from(Buffer.from(x, 'hex'));
const sha = b => createHash('sha256').update(b).digest('hex');
// Same fixed synthetic values as ledger/compiled-comparison.test.mjs.
const firstSecret = bytes(h(8291)), secondSecret = bytes(h(9373));
const firstAddress = h(48271), secondAddress = h(58273);
const address = h(78191), networkTag = h(99299), TIME = 1700000000n;
const program = bytes('95b46e39a9039e19063bb3d618128aec6cbd9ee656b6e635b3587e7f3f5235b2');
const net = bytes(networkTag);

const contract = new loan.Contract({});
const deployed = contract.initialState(
  runtime.createConstructorContext({}, '00'.repeat(32)),
  firstSecret, secondSecret, {bytes: bytes(firstAddress)}, {bytes: bytes(secondAddress)}, program, net);
const ctx = runtime.createCircuitContext(address, '00'.repeat(32), deployed.currentContractState, {}, undefined, undefined, Number(TIME));
ctx.currentQueryContext.block = {...ctx.currentQueryContext.block, secondsSinceEpoch: TIME, secondsSinceEpochErr: 0, lastBlockTime: TIME, parentBlockHash: '00'.repeat(32)};

const t0 = performance.now();
const result = contract.impureCircuits.initialize(ctx, firstSecret, program, net, 2n, TIME);
const execMs = performance.now() - t0;
const pd = result.proofData;
const preimage = ledger.proofDataIntoSerializedPreimage(pd.input, pd.output, pd.publicTranscript, pd.privateTranscriptOutputs, 'initialize');
const ir = readFileSync(dir + 'loan/zkir/initialize.bzkir');
const keyMaterial = {
  proverKey: readFileSync(dir + 'loan/keys/initialize.prover'),
  verifierKey: readFileSync(dir + 'loan/keys/initialize.verifier'),
  ir,
};
const prove = ledger.createProvingPayload(preimage, undefined, keyMaterial);
const check = ledger.createCheckPayload(preimage, ir);
writeFileSync(dir + 'initialize.preimage', preimage);
writeFileSync(dir + 'initialize.prove.bin', prove);
writeFileSync(dir + 'initialize.check.bin', check);
writeFileSync(dir + 'initialize.k.bin', ir);
console.log(JSON.stringify({
  circuit: 'initialize', circuitExecMs: +execMs.toFixed(2),
  publicTranscriptOps: pd.publicTranscript.length, privateTranscriptOutputs: pd.privateTranscriptOutputs.length,
  preimageBytes: preimage.length, provePayloadBytes: prove.length, checkPayloadBytes: check.length,
  provePayloadSha256: sha(prove), preimageSha256: sha(preimage),
}));
