// Produce real ProofPreimages for the seven Moriarty artifacts by driving the
// compiled contracts (output/contract/index.js) through the Compact runtime.
//
// Each circuit is executed on a consistent ledger state reached by executing the
// previous circuits of the same scenario, with deterministic witnesses and a
// block time chosen so the assertions hold. The runtime's proof data (inputs,
// public transcript, private transcript outputs, output) is turned into a
// tagged-serialized ProofPreimage by the onchain runtime itself
// (proofDataIntoSerializedPreimage) and decoded to the harness's JSON shape by
// the `preimage-json` binary built in the ledger-92e8bdd3 workspace.
//
// Usage: node moriarty_preimages.mjs --out DIR [--decoder PATH] [--runtime DIR]
//   COMPACT_RUNTIME may name the @midnight-ntwrk/compact-runtime package directory.
import { registerHooks } from 'node:module';
import { pathToFileURL, fileURLToPath } from 'node:url';
import { readdirSync, writeFileSync, mkdirSync, existsSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import path from 'node:path';

const HERE = path.dirname(fileURLToPath(import.meta.url));
const REPO = path.resolve(HERE, '..', '..', '..');
const args = Object.fromEntries(process.argv.slice(2).map((a, i, xs) => a.startsWith('--') ? [a.slice(2), xs[i + 1]] : []).filter(x => x.length));
const OUT = args.out ?? path.join(REPO, 'experiments', 'zkir-k', 'corpus', 'moriarty-contexts');
const DECODER = args.decoder ?? path.join(process.env.HOME, 'Moriarty/repos/_build/ledger-92e8bdd3/target/release/preimage-json');

function findRuntime() {
  if (args.runtime) return args.runtime;
  if (process.env.COMPACT_RUNTIME) return process.env.COMPACT_RUNTIME;
  const store = '/nix/store';
  const hits = readdirSync(store).filter(d => d.includes('midnight-ntwrk-compact-runtime_0.19.100'));
  if (!hits.length) throw new Error('no @midnight-ntwrk/compact-runtime 0.19.100 found; set COMPACT_RUNTIME');
  return path.join(store, hits[0], 'lib/node_modules/@midnight-ntwrk/compact-runtime');
}
const RUNTIME = findRuntime();
const runtimeUrl = pathToFileURL(path.join(RUNTIME, 'dist', 'index.js')).href;
registerHooks({
  resolve(specifier, context, next) {
    if (specifier === '@midnight-ntwrk/compact-runtime') return { url: runtimeUrl, shortCircuit: true };
    return next(specifier, context);
  },
});

const rt = await import('@midnight-ntwrk/compact-runtime');
const swapMod = await import(pathToFileURL(path.join(REPO, 'experiments/moriarty-core-swap/output/contract/index.js')).href);
const escrowMod = await import(pathToFileURL(path.join(REPO, 'experiments/moriarty-compact-escrow/output/contract/index.js')).href);

const bytes32 = (fill) => new Uint8Array(32).fill(fill);
const hex32 = (fill) => Buffer.from(bytes32(fill)).toString('hex');
const B32 = new rt.CompactTypeBytes(32);
const authority = (secret) => rt.persistentHash(B32, secret);
const COIN_PK = hex32(0x77);
const ADDRESS = rt.dummyContractAddress();

mkdirSync(OUT, { recursive: true });
const manifest = {
  produced: new Date().toISOString().slice(0, 10),
  method: 'compact-runtime',
  how: 'tools/moriarty_preimages.mjs drives output/contract/index.js through @midnight-ntwrk/compact-runtime 0.19.100 '
     + '(onchain-runtime-v4 4.0.0-rc.3, the same onchain-runtime version as midnight-ledger 92e8bdd3); the proof data of each '
     + 'call is serialised by proofDataIntoSerializedPreimage and decoded by the preimage-json binary '
     + '(experiments/zkir-k/tools/preimage-json, built in the ledger-92e8bdd3 workspace).',
  fixed_fields: 'binding_input is 0 and the communications-commitment randomness is 0: the onchain runtime sets both to 0 '
     + 'and the ledger overwrites them at transaction assembly, which is not performed here. key_location (dummy) is dropped.',
  runtime: RUNTIME, decoder: DECODER,
  contract_address: ADDRESS, coin_public_key: COIN_PK,
  artifacts: {},
};

function decode(pd) {
  const buf = rt.proofDataIntoSerializedPreimage(pd.input, pd.output, pd.publicTranscript, pd.privateTranscriptOutputs, 'dummy');
  const json = execFileSync(DECODER, ['-'], { input: Buffer.from(buf) }).toString();
  return JSON.parse(json);
}

async function runCircuit(name, contract, circuit, state, privateState, time, inputs, scenario) {
  const ctx = rt.createCircuitContext(circuit, ADDRESS, COIN_PK, state, privateState, undefined, undefined, undefined, time);
  const res = await contract.circuits[circuit](ctx, ...inputs);
  const trace = res.context.callProofDataTrace;
  const pd = trace[trace.length - 1];
  if (pd.circuitId !== circuit) throw new Error(`trace tail is ${pd.circuitId}, expected ${circuit}`);
  const pre = decode(pd);
  const { key_location, ...harness } = pre;
  const file = path.join(OUT, `${name}.pre.json`);
  writeFileSync(file, JSON.stringify(harness, null, 1) + '\n');
  manifest.artifacts[name] = {
    circuit, scenario, time, inputs: inputs.map(String),
    sizes: { inputs: harness.inputs.length, private_transcript: harness.private_transcript.length,
             public_transcript_inputs: harness.public_transcript_inputs.length, public_transcript_outputs: harness.public_transcript_outputs.length },
  };
  console.log(`${name}: wrote ${file}`, manifest.artifacts[name].sizes);
  return res.context.callContext.currentQueryContext.state;
}

// --- swap ------------------------------------------------------------------------
{
  const aliceSecret = bytes32(0xa1), bobSecret = bytes32(0xb0);
  const witnesses = {
    aliceSecret: (wc) => [wc.privateState, aliceSecret],
    bobSecret: (wc) => [wc.privateState, bobSecret],
  };
  const contract = new swapMod.Contract(witnesses);
  const priv = { role: 'test' };
  const deadline = 1_800_000_000n;   // 2027-01-15, seconds since the epoch
  const before = 1_700_000_000, after = 1_900_000_000;
  const init = await contract.initialState(
    rt.createConstructorContext(priv, COIN_PK),
    { bytes: bytes32(0x0a) }, { bytes: bytes32(0x0b) },
    authority(aliceSecret), authority(bobSecret),
    bytes32(0x1a), bytes32(0x1b),
    1000n, 2000n, deadline);
  const s0 = init.currentContractState;
  const s1 = await runCircuit('swap-fundAlice', contract, 'fundAlice', s0, priv, before, [], 'fresh contract (WaitingAlice), block time before the deadline');
  const s2 = await runCircuit('swap-fundBob', contract, 'fundBob', s1, priv, before, [], 'after fundAlice (WaitingBob), block time before the deadline');
  await runCircuit('swap-decide', contract, 'decide', s2, priv, before, [1n], 'after fundBob (WaitingDecision), decision=1 (settle), block time before the deadline');
  await runCircuit('swap-expire', contract, 'expire', s2, priv, after, [], 'after fundBob (WaitingDecision), block time past the deadline; refunds both deposits');
}

// --- escrow ----------------------------------------------------------------------
{
  const partySecret = bytes32(0xe5);
  const witnesses = { partySecret: (wc) => [wc.privateState, partySecret] };
  const contract = new escrowMod.Contract(witnesses);
  const priv = { role: 'buyer' };
  const deadline = 1_800_000_000n;
  const before = 1_700_000_000, after = 1_900_000_000;
  const init = await contract.initialState(
    rt.createConstructorContext(priv, COIN_PK),
    { bytes: bytes32(0x0c) }, { bytes: bytes32(0x0d) },
    authority(partySecret), bytes32(0x1c), 500n, deadline);
  const s0 = init.currentContractState;
  const s1 = await runCircuit('escrow-fund', contract, 'fund', s0, priv, before, [], 'fresh contract (Waiting); buyer witness matches buyerAuthority');
  await runCircuit('escrow-release', contract, 'release', s1, priv, before, [], 'after fund (Funded); buyer witness matches buyerAuthority');
  await runCircuit('escrow-refundAfterTimeout', contract, 'refundAfterTimeout', s1, priv, after, [], 'after fund (Funded), block time past the deadline');
}

writeFileSync(path.join(OUT, 'manifest.json'), JSON.stringify(manifest, null, 1) + '\n');
console.log('manifest written');
