import {createHash} from 'node:crypto';
import {isDeepStrictEqual} from 'node:util';
import {encodeValue} from './values.mjs';

const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const hex = bytes => Buffer.from(bytes).toString('hex');
const name = operation => typeof operation === 'string' ? operation : Buffer.from(operation).toString('utf8');

/**
 * Accept only the pinned installation: exactly the enrolled entry points and verifier keys,
 * the enrolled maintenance committee and threshold, and no maintenance update since deployment.
 */
export function verifyInstalledContract(native, deployment) {
  const pinned = Object.keys(deployment.verifiers ?? {}).sort();
  const installed = native.operations().map(name).sort();
  if (!pinned.length || !isDeepStrictEqual(installed, pinned)) throw Error('Unexpected contract entry points');
  for (const operation of pinned) {
    const key = native.operation(operation)?.verifierKey;
    if (!key || digest(key) !== deployment.verifiers[operation]) throw Error(operation+' verifier mismatch');
  }
  const authority = native.maintenanceAuthority, pin = deployment.maintenance;
  if (!pin) throw Error('Missing pinned maintenance authority');
  if (authority.counter !== 0n) throw Error('Contract maintenance has been exercised');
  if (authority.threshold !== pin.threshold || !isDeepStrictEqual([...authority.committee], pin.committee)) {
    throw Error('Maintenance authority mismatch');
  }
}

// The caller supplies a trusted native-ledger connection and an independently accepted deployment.
// Read through that connection on every invocation. A request never supplies its own state.
export function readAcceptedOrder(reader,runtime,compiled,deployment) {
  if (reader.network !== deployment.network) throw Error('Wrong ledger network');
  const native = reader.index(deployment.address);
  if (!native) throw Error('Order deployment absent from accepted head');
  verifyInstalledContract(native, deployment);
  const bytes = native.serialize();
  const state = runtime.ContractState.deserialize(bytes);
  const order = compiled.ledger(state.data);
  if (hex(order.terms.program) !== deployment.programHash) throw Error('Wrong accepted order program');
  if (hex(order.terms.network) !== digest(deployment.network)) throw Error('Wrong signed network domain');
  for (const [field, value] of Object.entries(deployment.pins ?? {})) {
    if (!isDeepStrictEqual(encodeValue(order[field]), value)) throw Error('Pinned '+field+' mismatch');
  }
  const custody = new Map();
  for (const [token,amount] of state.balance) {
    if (token.tag === 'unshielded') custody.set(token.raw,amount);
  }
  return {terms:order.terms,owner:order.ownerKey,kernelState:order.kernelState,revision:order.revision,
    remaining:order.remaining,funded:order.funded,custody,contractStateSha256:digest(bytes)};
}
