// Offline public-transcript check. No wallet, proving, network or preimage files.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
const [build, runtimeEntry, ledgerEntry, templatePath] = process.argv.slice(2);
assert.ok(build && runtimeEntry && ledgerEntry && templatePath,
  'usage: node capture-a4-template.mjs BUILD RUNTIME_ENTRY LEDGER_ENTRY TEMPLATE');
const rt = await import(pathToFileURL(resolve(runtimeEntry)).href);
const ledger = await import(pathToFileURL(resolve(ledgerEntry)).href);
const mod = await import(pathToFileURL(resolve(build, 'contract/index.js')).href);
const expected = JSON.parse(readFileSync(templatePath));
const le = x => Buffer.from(x.toString(16).padStart(64, '0'), 'hex').reverse().toString('hex');
const modulus = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001n;

// The ledger serializes counts and Fr values with SCALE, not fixed-width integers.
function transcript(raw) {
  const bytes = Buffer.from(raw), tag = Buffer.from('midnight:proof-preimage:');
  assert.ok(bytes.subarray(0, tag.length).equals(tag));
  let at = tag.length;
  const integer = () => {
    assert.ok(at < bytes.length);
    const first = bytes[at++], mode = first & 3;
    if (mode === 0) return BigInt(first >>> 2);
    const count = mode === 3 ? (first >>> 2) + 4 : [0, 2, 4][mode];
    assert.ok(count <= 32 && at + count - Number(mode !== 3) <= bytes.length);
    let value = mode === 3 ? 0n : BigInt(first);
    for (let i = Number(mode !== 3); i < count; i++) value |= BigInt(bytes[at++]) << BigInt(8 * i);
    return mode === 3 ? value : value >> 2n;
  };
  const vector = () => {
    const count = Number(integer());
    assert.ok(count <= 4096);
    return Array.from({length: count}, () => {
      const field = integer();
      assert.ok(field < modulus);
      return le(field);
    });
  };
  vector(); // inputs stay in memory
  vector(); // private transcript stays in memory
  return vector();
}

const captures = [];
for (const statement of [42n, 43n]) {
  const contract = new mod.Contract({});
  const initial = await contract.initialState(rt.createConstructorContext({}, '00'.repeat(32)),
    1n, 1n, 2n, 3n, 4n, 5n, 1n, 1n, 0n, 0n);
  const context = rt.createCircuitContext('acceptValidity', '01'.repeat(32), '00'.repeat(32),
    initial.currentContractState, {});
  const result = await contract.provableCircuits.acceptValidity(context, statement);
  const data = result.context.callProofDataTrace[0];
  const fields = transcript(ledger.proofDataIntoSerializedPreimage(data.input, data.output,
    data.publicTranscript, data.privateTranscriptOutputs, 'sherlock/acceptValidity'));
  assert.equal(fields.length, expected.slots.length - 2);
  for (let i = 2; i < expected.slots.length; i++) {
    const slot = expected.slots[i];
    assert.equal(fields[i - 2], slot.kind === 'stmt' ? le(statement) : slot.value, `slot ${i}`);
  }
  captures.push(fields);
}
assert.notEqual(captures[0][8], captures[1][8]);
console.log('PASS: both SherlockAnchor transcripts match all 12 template constants and statement slot 10');
