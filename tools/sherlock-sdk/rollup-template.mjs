// Capture public acceptValidity transcript slots from two offline calls.
// Private preimages remain in memory; only constants and statement indices are written.
import assert from 'node:assert/strict';
import {createRequire} from 'node:module';
import {writeFileSync} from 'node:fs';
import {resolve, join} from 'node:path';
import {pathToFileURL} from 'node:url';

const [sdkRoot, build, output] = process.argv.slice(2);
assert.ok(output, 'usage: rollup-template.mjs SDK_ROOT BUILD OUTPUT_JSON');
const require = createRequire(resolve(sdkRoot, 'package.json'));
const load = name => import(pathToFileURL(require.resolve(name)).href);
const rt = await load('@midnight-ntwrk/compact-runtime');
const ledger = await load('@midnight-ntwrk/midnight-js-protocol/ledger');
const mod = await import(pathToFileURL(resolve(build, 'contract/index.js')).href);
const le = value => Buffer.from(value.toString(16).padStart(64, '0'), 'hex').reverse().toString('hex');
const modulus = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001n;

function publicFields(raw) {
  const bytes = Buffer.from(raw), tag = Buffer.from('midnight:proof-preimage:');
  assert.ok(bytes.subarray(0, tag.length).equals(tag));
  let at = tag.length;
  const integer = () => {
    assert.ok(at < bytes.length);
    const first = bytes[at++], mode = first & 3;
    if (mode === 0) return BigInt(first >>> 2);
    const count = mode === 3 ? (first >>> 2) + 4 : [0, 2, 4][mode];
    const start = mode === 3 ? at : at - 1;
    assert.ok(start + count <= bytes.length);
    let value = 0n;
    for (let i = 0; i < count; i++) value |= BigInt(bytes[start + i]) << BigInt(8 * i);
    at = start + count;
    return mode === 3 ? value : value >> 2n;
  };
  const vector = () => {
    const count = Number(integer());
    assert.ok(count <= 4096);
    return Array.from({length: count}, () => {
      const value = integer(); assert.ok(value < modulus); return le(value);
    });
  };
  vector(); vector(); // Inputs/private transcript are deliberately not persisted.
  return vector();
}

const statements = [42n, 43n], captures = [];
for (const statement of statements) {
  const contract = new mod.Contract({});
  const initial = await contract.initialState(rt.createConstructorContext({}, '00'.repeat(32)),
    1n, 1n, 0n, 0n, 0n, 0n);
  const context = rt.createCircuitContext('acceptValidity', '01'.repeat(32), '00'.repeat(32),
    initial.currentContractState, {});
  const result = await contract.provableCircuits.acceptValidity(context, statement);
  const pd = result.context.callProofDataTrace[0];
  captures.push(publicFields(ledger.proofDataIntoSerializedPreimage(pd.input, pd.output,
    pd.publicTranscript, pd.privateTranscriptOutputs, 'sherlock/rollup/acceptValidity')));
}
assert.equal(captures[0].length, captures[1].length);
const slots = [{kind: 'binding_input'}, {kind: 'communication_commitment'}];
const stmtIndices = [], constants = {};
for (let i = 0; i < captures[0].length; i++) {
  if (captures[0][i] === captures[1][i]) {
    slots.push({kind: 'const', value: captures[0][i]}); constants[i + 2] = captures[0][i];
  } else {
    for (let j = 0; j < statements.length; j++) assert.equal(captures[j][i], le(statements[j]));
    slots.push({kind: 'stmt'}); stmtIndices.push(i + 2);
  }
}
assert.ok(stmtIndices.length > 0);
writeFileSync(output, JSON.stringify({slots, n: slots.length, free: [0, 1, ...stmtIndices],
  stmt_index: stmtIndices[0], stmt_indices: stmtIndices, constants, provisional: false,
  source: 'two compiled S0 rollup-anchor acceptValidity runtime captures; repeated statement slots constrained equal'}, null, 2) + '\n');
console.log(`PASS rollup template: ${slots.length} public slots, statement at ${stmtIndices.join(',')}`);
