import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {test} from 'node:test';
import {lowerCompact} from '../../../experiments/moriarty-language/src/lower-compact.ts';
import {checkRecoveryReadback} from '../src/recovery.mjs';

const mapping = file => lowerCompact(readFileSync(new URL('../orders/' + file, import.meta.url)),
  readFileSync(new URL('../../../experiments/moriarty-language/spec/bounds.json', import.meta.url)));

// Independent native readbacks after the winning transaction. Each vector records
// owner A, other owner A, executor A, recipient B, first custody A, second custody A,
// tracked remainder, prior A spent, A refunded, closed flag, revision.
const expected = JSON.parse(readFileSync(new URL('./fixtures/recovery.json', import.meta.url)));
test('partial recovery uses a separate bounded program and leaves the one-fill O1 unchanged', () => {
  assert.notEqual(mapping('partial-recovery.mori').metadata.programHash, mapping('o1.mori').metadata.programHash);
  assert.deepEqual(mapping('partial-recovery.mori').metadata.textTable, mapping('o1.mori').metadata.textTable);
});
test('refund reconciliation distinguishes spent principal, unassigned surplus and the other order', () => {
  checkRecoveryReadback(expected.partialRefund, expected.partialRefund);
  for (const [i, value] of [[0, 100], [2, 0], [3, 0], [4, 0], [5, 40], [6, 60], [8, 100], [9, 0], [10, 1]]) {
    const changed = [...expected.partialRefund];
    changed[i] = value;
    assert.throws(() => checkRecoveryReadback(changed, expected.partialRefund));
  }
});
