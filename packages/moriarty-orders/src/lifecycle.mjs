import {readAcceptedOrder} from './accepted-state.mjs';
import {verifyEnrollment} from './authorization.mjs';
export {readAcceptedOrder} from './accepted-state.mjs';
export {signOrder,issueEnrollment,verifyEnrollment} from './authorization.mjs';
export {acceptFill,prepareFill} from './acceptance.mjs';

function accepted(ledger,instance,deployment) {
  if (instance.address !== deployment.address) throw Error('Wrong order contract');
  const head = readAcceptedOrder({network:ledger.network,index:address=>ledger.state.index(address)},
    ledger.runtime,instance.contract.generated,deployment);
  verifyEnrollment(ledger.L,ledger.runtime,deployment,head,deployment.enrollment);
  return head;
}

export async function fundOrder(ledger,instance,deployment,payer) {
  const head = accepted(ledger,instance,deployment);
  if (head.funded) throw Error('ALREADY_FUNDED');
  if (payer.address !== ledger.runtime.decodeUserAddress(head.terms.account.bytes)) throw Error('Wrong owner funding wallet');
  return ledger.call(instance,'fund',[],'fund-order',payer);
}

// Cancellation and return of the entire remaining A occur atomically in this policy.
export async function cancelOrder(ledger,instance,deployment,{revision,signature},payer) {
  const head = accepted(ledger,instance,deployment);
  if (!head.funded || head.revision !== revision || head.kernelState.f3 !== 0n) throw Error('ORDER_REVISION');
  return ledger.call(instance,'cancel',[revision,signature],'cancel-order',payer);
}
