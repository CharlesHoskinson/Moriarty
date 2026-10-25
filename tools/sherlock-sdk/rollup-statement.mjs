// Public correspondence checks shared by offline deployment and call preparation.
import assert from 'node:assert/strict';
// Each WASM module owns distinct classes; cross the public serialization boundary.
export const anchorView = (runtime, contract, state) =>
  contract.ledger(runtime.ContractState.deserialize(state.serialize()).data);
const modulus = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001n;
export const fromLe = value => {
  assert.match(value, /^[0-9a-f]{64}$/);
  const field = BigInt('0x' + Buffer.from(value, 'hex').reverse().toString('hex'));
  assert.ok(field < modulus, 'noncanonical public field');
  return field;
};
export function bindBatch(leaf, parent, manifest, runtime) {
  assert.equal(leaf.length, 17); assert.equal(parent.length, 29);
  assert.deepEqual(leaf, parent.slice(0, 17), 'leaf/parent statement mismatch');
  const fields = leaf.map(fromLe), id = fromLe(manifest.instance_id);
  const batch = fromLe(manifest.batch_no);
  assert.ok(id > 0n && batch > 0n);
  assert.equal(fields[15], (batch - 1n) * 50n + 1n);
  assert.equal(fields[16], batch * 50n);
  const domain = 0x544d54535f4d4f44n;
  const projection = [0, 1, 15, 16, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
  const statement = runtime.transientHash(new runtime.CompactTypeVector(17, runtime.CompactTypeField),
    [domain, id, batch, ...projection.map(i => fields[i])]);
  assert.equal(statement, fromLe(manifest.statement), 'prepared terminal statement mismatch');
  return {fields, id, batch, statement};
}
export function assertPredecessor(view, {fields, id, batch}) {
  for (const [name, value] of Object.entries({instance_id: id, root: fields[0],
    height: fields[15] - 1n, batch_no: batch - 1n, dep_consumed: fields[5], dep_acc: fields[7]}))
    assert.equal(view[name], value, `anchor predecessor ${name}`);
}
