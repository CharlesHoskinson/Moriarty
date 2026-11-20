// The signing runtime and compiled module must match the deployed verifier.
export function signFeedUpdate(runtime, pureCircuits, domain, observation, controllerSecret) {
  const message = pureCircuits.feedMessage(domain.network, domain.address, observation);
  const type = new runtime.CompactTypeVector(1, runtime.CompactTypeField);
  return runtime.jubjubSchnorrSign(type, [message], controllerSecret);
}
