// T20: per-entry call-vector lanes, serialized proof-preimage bytes and contract-state bytes of a compiled
// Compact contract (compactc --skip-zk), with compact-runtime 0.21.0 and ledger-v9 (the T10a call path of
// sherlock-ledger.test.mjs). Used to give before and after write sizes for the batched ledger.
// Run: node t20-measure.mjs BUILD_DIR SPEC_JSON
//   BUILD_DIR  compactc output dir (contract/index.js)
//   SPEC_JSON  {"ctor": [args], "calls": [{"entry": name, "args": [decimal strings]}]}
// Env: SHERLOCK_SDK_ROOT (default ~/.cache/xc9/sdk), T20_SHIM_DIR (default ~/.cache/t20/shims).
import {readFileSync, writeFileSync, mkdirSync, existsSync, symlinkSync} from 'node:fs';
import {resolve, join} from 'node:path';
import {homedir} from 'node:os';
import {pathToFileURL} from 'node:url';
import {createHash} from 'node:crypto';
import {decodePreimage} from './e6-padding.mjs';

const H = homedir();
const sdk = resolve(process.env.SHERLOCK_SDK_ROOT ?? join(H, '.cache/xc9/sdk'));
const shimDir = resolve(process.env.T20_SHIM_DIR ?? join(H, '.cache/t20/shims'));
const [build, specFile] = process.argv.slice(2).map(p => resolve(p));
mkdirSync(shimDir, {recursive: true});
if (!existsSync(join(shimDir, 'node_modules'))) symlinkSync(join(sdk, 'node_modules'), join(shimDir, 'node_modules'));
if (!existsSync(join(build, 'node_modules'))) symlinkSync(join(sdk, 'node_modules'), join(build, 'node_modules'));
if (!existsSync(join(build, 'package.json'))) writeFileSync(join(build, 'package.json'), '{"type":"module"}\n');
const load = async name => {
  const file = join(shimDir, createHash('sha256').update(name).digest('hex') + '.mjs');
  if (!existsSync(file)) writeFileSync(file, `import * as m from ${JSON.stringify(name)}; export default m;\n`);
  return (await import(pathToFileURL(file).href)).default;
};
const rt = await load('@midnight-ntwrk/compact-runtime');
const L = await load('@midnight-ntwrk/midnight-js-protocol/v9');
const mod = await import(pathToFileURL(join(build, 'contract/index.js')).href);
const spec = JSON.parse(readFileSync(specFile, 'utf8'));
const big = a => Array.isArray(a) ? a.map(big) : typeof a === 'string' && /^(-?\d+|0x[0-9a-fA-F]+)$/.test(a) ? BigInt(a) : a;
const params = L.LedgerParameters.initialParameters();
const ADDR = '01'.repeat(32);

const contract = new mod.Contract(spec.witnesses ? Object.fromEntries(Object.entries(spec.witnesses)
  .map(([k, v]) => [k, ({privateState}) => [privateState, big(v)]])) : {});
const st = await contract.initialState(rt.createConstructorContext({}, '00'.repeat(32)), ...big(spec.ctor));
let cs = st.currentContractState;
const stateBytes = () => L.ContractState.deserialize(cs.serialize()).data.state.serialize?.()?.length ?? cs.serialize().length;
const out = [];
for (const c of spec.calls) {
  const before = L.ContractState.deserialize(cs.serialize());
  const sb = cs.serialize().length;
  const ctx = rt.createCircuitContext({circuitId: c.entry, contractAddress: ADDR, coinPublicKeyOrZswapState: '00'.repeat(32),
    contractState: cs, privateState: {}});
  let rec = {entry: c.entry, label: c.label ?? c.entry};
  try {
    const r = await contract.provableCircuits[c.entry](ctx, ...big(c.args));
    const pd = r.context.callProofDataTrace[0];
    const qc = new L.QueryContext(before.data, ADDR);
    qc.block = ctx.callContext.currentQueryContext.block;
    const [g, f] = L.partitionTranscripts([new L.PreTranscript(qc, pd.publicTranscript)], params)[0];
    const prog = (g ?? f).program;
    const preBytes = L.proofDataIntoSerializedPreimage(pd.input, pd.output, prog, pd.privateTranscriptOutputs, `t20/${c.entry}`);
    const pre = decodePreimage(preBytes);
    cs.data = r.context.callContext.currentQueryContext.state;
    rec = {...rec, ok: true, lanes: 2 + pre.publicTranscriptInputs.length, transcriptWords: pre.publicTranscriptInputs.length,
      preimageBytes: preBytes.length, contractStateBytesBefore: sb, contractStateBytesAfter: cs.serialize().length,
      stateDelta: cs.serialize().length - sb};
  } catch (e) {
    rec = {...rec, ok: false, error: String(e.message ?? e).slice(0, 160)};
  }
  out.push(rec);
}
console.log(JSON.stringify(out, null, 1));
