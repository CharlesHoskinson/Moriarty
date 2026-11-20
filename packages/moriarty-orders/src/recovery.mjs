import assert from 'node:assert/strict';
import {readAcceptedOrder} from './accepted-state.mjs';

const fields = ['owner A','second owner A','executor A','recipient B','order A','second order A',
  'remaining A','spent A','refunded A','closed','revision'];

export function checkRecoveryReadback(actual, expected) {
  assert.equal(actual.length, fields.length);
  assert.equal(expected.length, fields.length);
  for (const [index, name] of fields.entries()) assert.equal(actual[index], expected[index], name);
}

// Read both order instances at the same accepted head. Wallets expose distinct
// signed roles, even where the first recipient also owns the second order.
export function recoveryReadback(ledger, contract, first, second) {
  const reader = {network:ledger.network, index:address => ledger.state.index(address)};
  assert.notEqual(first.address, second.address);
  const read = deployment => readAcceptedOrder(reader, ledger.runtime, contract.generated, deployment);
  const order = read(first);
  const other = read(second);
  const {decodeUserAddress:address} = ledger.runtime;
  const assetA = Buffer.from(order.terms.assetA).toString('hex');
  const assetB = Buffer.from(order.terms.assetB).toString('hex');
  assert.deepEqual(order.terms.assetA, other.terms.assetA);
  assert.notEqual(address(order.terms.account.bytes), address(other.terms.account.bytes));
  return [ledger.balance(address(order.terms.account.bytes), assetA),
    ledger.balance(address(other.terms.account.bytes), assetA),
    ledger.balance(address(order.terms.executor.bytes), assetA),
    ledger.balance(address(order.terms.recipient.bytes), assetB),
    order.custody.get(assetA) ?? 0n, other.custody.get(assetA) ?? 0n,
    order.kernelState.f0, order.kernelState.f1, order.kernelState.f2, order.kernelState.f3,
    order.revision].map(value => {
      const number = Number(value);
      if (!Number.isSafeInteger(number)) throw Error('Recovery fixture exceeds safe integer range');
      return number;
    });
}
