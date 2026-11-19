import {createHash} from 'node:crypto';
import {isDeepStrictEqual} from 'node:util';

const sha = bytes => createHash('sha256').update(bytes).digest('hex');
const hex = bytes => Buffer.from(bytes).toString('hex');
const fields = ['predecessor','eventId','custody','orderId','program','asset','amount','budget'];
function identity(capture) {
  const bytes32 = bytes => {
    if (!(bytes instanceof Uint8Array) || bytes.length !== 32) throw Error('Invalid continuation digest');
    return hex(bytes);
  };
  if (!capture || Object.keys(capture).sort().join() !== [...fields].sort().join() ||
      typeof capture.amount !== 'bigint' || typeof capture.budget !== 'bigint') throw Error('Invalid continuation schema');
  return [bytes32(capture.predecessor),bytes32(capture.eventId),bytes32(capture.custody?.bytes),
    bytes32(capture.orderId),bytes32(capture.program),bytes32(capture.asset),capture.amount,capture.budget];
}

export function checkContinuation(accepted, submitted) {
  if (!isDeepStrictEqual(identity(accepted),identity(submitted))) throw Error('Changed continuation capture');
}

export function readContinuation(ledger, runtime, compiled, deployment) {
  if (ledger.network !== deployment.network) throw Error('Wrong continuation network');
  const contract = ledger.index(deployment.address);
  if (!contract) throw Error('Missing continuation contract');
  for (const operation of ['fund','first','enable','resume','refund']) {
    const key = contract.operation(operation)?.verifierKey;
    if (!key || sha(key) !== deployment.verifiers[operation]) throw Error('Wrong continuation verifier '+operation);
  }
  const state = runtime.ContractState.deserialize(contract.serialize());
  const head = compiled.ledger(state.data);
  if (hex(head.terms.program) !== deployment.programHash || hex(head.terms.network) !== sha(ledger.network)) {
    throw Error('Wrong continuation domain');
  }
  return head;
}

export function prepareTerminal(ledger, instance, deployment, request) {
  if (instance.address !== deployment.address) throw Error('Wrong continuation custody');
  const reader = {network:ledger.network,index:address=>ledger.state.index(address)};
  const head = readContinuation(reader,ledger.runtime,instance.contract.generated,deployment);
  if (head.terminal !== 0n || head.current.f2 !== 1n) throw Error('Continuation has been consumed');
  checkContinuation(head.pending,request.capture);
  if (request.action === 'refund') return ledger.prepareCall(instance,'refund',[request.capture,request.signature]);
  if (request.action === 'resume') {
    return ledger.prepareCall(instance,'resume',[request.capture,request.delivery,request.now,request.signature]);
  }
  throw Error('Unknown continuation action');
}

export function checkO9Balances(observed, expected) {
  if (!isDeepStrictEqual(observed,expected)) throw Error('Unexpected staged exchange balances: '+JSON.stringify(observed));
}
