import {createHash} from 'node:crypto';
import {readAcceptedOrder} from './accepted-state.mjs';
import {verifyEnrollment} from './authorization.mjs';

export const FEED_AB = Uint8Array.from(Buffer.from('feed-AB'.padEnd(32, '\0')));

export function signProviderQuote(runtime, generated, grant, providerSecret) {
  const message = generated.pureCircuits.quoteMessage(grant.terms, grant.contract, grant.revision, grant.input, grant.delivery);
  return runtime.jubjubSchnorrSign(new runtime.CompactTypeVector(1, runtime.CompactTypeField), [message], providerSecret);
}

export function prepareO5Fill(ledger, instance, deployment, request) {
  if (deployment.address !== instance.address) throw Error('Wrong order contract');
  // The owner enrolls the feed controller and quote provider; the reader compares both with the ledger.
  if (!deployment.pins?.feedController || !deployment.pins?.providerKey) throw Error('O5 deployment must pin its feed controller and provider');
  const reader = {network: ledger.network, index: address => ledger.state.index(address)};
  const head = readAcceptedOrder(reader, ledger.runtime, instance.contract.generated, deployment);
  verifyEnrollment(ledger.L, ledger.runtime, deployment, head, deployment.enrollment);
  const installed = ledger.state.index(instance.address).operation('publishFeedUpdate')?.verifierKey;
  if (!installed || createHash('sha256').update(installed).digest('hex') !== deployment.verifiers.publishFeedUpdate) {
    throw Error('Installed feed verifier mismatch');
  }
  if (!head.funded || head.revision !== request.revision || head.kernelState.f3 !== 0n) throw Error('ORDER_REVISION');
  return ledger.prepareCall(instance, 'fill', [request.revision, request.input, request.delivery, request.fee,
    request.now, request.signature, request.feed, request.quote]);
}
