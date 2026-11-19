import {createHash} from 'node:crypto';

const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const hex = bytes => Buffer.from(bytes).toString('hex');

// The caller supplies a trusted native-ledger connection and an independently accepted deployment.
// Read through that connection on every invocation. A request never supplies its own state.
export function readAcceptedOrder(reader,runtime,compiled,deployment) {
  if (reader.network !== deployment.network) throw Error('Wrong ledger network');
  const native = reader.index(deployment.address);
  if (!native) throw Error('Order deployment absent from accepted head');
  for (const op of ['fund','fill','cancel']) {
    const key = native.operation(op)?.verifierKey;
    if (!key || digest(key) !== deployment.verifiers[op]) throw Error(op+' verifier mismatch');
  }
  const bytes = native.serialize();
  const state = runtime.ContractState.deserialize(bytes);
  const order = compiled.ledger(state.data);
  if (hex(order.terms.program) !== deployment.programHash) throw Error('Wrong accepted order program');
  if (hex(order.terms.network) !== digest(deployment.network)) throw Error('Wrong signed network domain');
  const custody = new Map();
  for (const [token,amount] of state.balance) {
    if (token.tag === 'unshielded') custody.set(token.raw,amount);
  }
  return {terms:order.terms,owner:order.ownerKey,kernelState:order.kernelState,revision:order.revision,
    remaining:order.remaining,funded:order.funded,custody,contractStateSha256:digest(bytes)};
}
