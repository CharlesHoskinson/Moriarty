// Circuit refusals at call preparation produce no transaction. Record the public
// call, its accepted predecessor state and the observed assertion, so a checker
// can re-execute the same compiled circuit instead of trusting a recorded label.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync, readFileSync, writeFileSync} from 'node:fs';
import {join} from 'node:path';
import {decodeValue} from '../src/values.mjs';

export {encodeValue, decodeValue} from '../src/values.mjs';

const sha = bytes => createHash('sha256').update(bytes).digest('hex');

/** Run one provable circuit against the installed contract state with the harness's block context. */
export async function executeCircuit({runtime, contract, ledgerState, address, circuit, args, time}) {
  const compactState = runtime.ContractState.deserialize(ledgerState.serialize());
  const ctx = runtime.createCircuitContext({circuitId: circuit, contractAddress: address,
    coinPublicKeyOrZswapState: '00'.repeat(32), contractState: compactState, privateState: {}, time});
  const initial = ctx.callContext.currentQueryContext;
  initial.block = {...initial.block, secondsSinceEpoch: BigInt(time), secondsSinceEpochErr: 0, lastBlockTime: BigInt(time)};
  const executed = await contract.provableCircuits[circuit](ctx, ...args);
  return {executed, initial};
}

/** Persist each refusal and the exact ledger state it was evaluated against. */
export function retainedRefusals(directory) {
  const rows = [];
  return (row, stateBytes) => {
    const file = 'refusal-state-' + row.stateSha256.slice(0, 16) + '.bin';
    if (!existsSync(join(directory, file))) writeFileSync(join(directory, file), stateBytes);
    rows.push({...row, state: {file, sha256: row.stateSha256}});
    writeFileSync(join(directory, 'refusals.json'), JSON.stringify(rows, null, 2) + '\n');
  };
}

/** Re-execute every recorded refusal. The same assertion must refuse the same call. */
export async function rerunRefusals({L, runtime, compiled, directory, rows}) {
  assert.ok(Array.isArray(rows) && rows.length > 0, 'missing recorded refusals');
  const contract = new compiled.Contract({});
  const results = [];
  for (const row of rows) {
    assert.equal(row.boundary, 'compiled-circuit-preparation', row.label);
    const bytes = readFileSync(join(directory, row.state.file));
    assert.equal(sha(bytes), row.state.sha256, row.label + ' state digest');
    const state = L.LedgerState.deserialize(bytes);
    const ledgerState = state.index(row.address);
    assert.ok(ledgerState, row.label + ' contract absent from recorded state');
    let error;
    try {
      await executeCircuit({runtime, contract, ledgerState, address: row.address, circuit: row.circuit,
        args: decodeValue(row.args), time: row.time});
    } catch (caught) {
      error = caught;
    }
    assert.ok(error, row.label + ' was accepted when re-executed');
    const message = String(error?.message ?? error);
    assert.match(message, new RegExp(row.expectedError), row.label);
    assert.equal(message, row.observedError, row.label + ' refused for a different reason');
    results.push({label: row.label, circuit: row.circuit, error: message, stateSha256: row.state.sha256});
  }
  return results;
}
