import {verifyEnrollment} from './authorization.mjs';
import {readAcceptedOrder} from './accepted-state.mjs';

// "ledger" implements the SDK's call builder and owns the current strict LedgerState.
// Prepare against the installed key and state. Ledger application rechecks the transcript.
export function prepareFill(ledger,instance,deployment,request) {
  if (instance.address !== deployment.address) throw Error('Wrong order contract');
  const reader = {network:ledger.network,index:address=>ledger.state.index(address)};
  const head = readAcceptedOrder(reader,ledger.runtime,instance.contract.generated,deployment);
  verifyEnrollment(ledger.L,ledger.runtime,deployment,head,deployment.enrollment);
  if (!head.funded || head.revision !== request.revision || head.kernelState.f3 !== 0n) throw Error('ORDER_REVISION');
  return ledger.prepareCall(instance,'fill',[request.revision,request.input,request.delivery,request.fee,
    request.now,request.signature]);
}

export async function acceptFill(ledger,instance,deployment,request,payer) {
  const prepared = await prepareFill(ledger,instance,deployment,request);
  const tx = await ledger.fundAndProve(ledger.assemble([prepared]),instance.contract,payer);
  const receipt = ledger.apply(tx,'accept-order');
  return {tx,receipt};
}
