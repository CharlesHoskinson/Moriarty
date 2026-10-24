module b4RouteClaimTest {
  import b4RouteClaim.* from "./b4RouteClaim"

  // Shared prefixes (old and new behave identically here): route 1 claims and parks on its preimage read, then on its proofFile read.
  run toPreimage = init.then(providerProve).then(routeEnter(1))
  run toProofRead = toPreimage.then(routeAfterPreimage(1)).then(bindCallback).then(innerReturn(1))
    .expect(s.rpc.get(1) == ProofRead and s.st == Proving and s.innerCalls == 1 and not(s.captured))

  // T20 control: release with no injection -> linked, first null.
  run parkedReadControlLinkedTest = toProofRead.then(routeAfterProof(1)).then(baseReturn).then(observeSealed)
    .expect(s.st == Linked and s.first == NoErr and safety)

  // T20 injections recorded before release: STATE_CHANGED on release, original code kept, no capture, inner once, no later receipt.
  run t20ObserveInjectedTest = toProofRead.then(observeSealed).then(routeAfterProof(1)).then(baseReturn).then(observeSealed)
    .expect(s.st == Failed and s.first == SealedOutOfOrder and s.rpc.get(1) == Rej and not(s.captured) and s.innerCalls == 1 and safety)
  run t20BindInjectedTest = toProofRead.then(bindCallback).then(routeAfterProof(1)).then(baseReturn).then(observeSealed)
    .expect(s.st == Failed and s.first == CallbackOutsideRoute and s.rpc.get(1) == Rej and not(s.captured) and s.innerCalls == 1 and safety)
  run t20ReproveInjectedTest = toProofRead.then(providerProve).then(routeAfterProof(1)).then(baseReturn).then(observeSealed)
    .expect(s.st == Failed and s.first == LinkReused and s.rpc.get(1) == Rej and not(s.captured) and s.innerCalls == 1 and safety)

  // T21: the concurrent route is refused at entry; the released first route then fails at('proving') before inner.
  run t21ConcurrentRouteRefusedTest = toPreimage.then(routeEnter(2))
    .expect(s.st == Failed and s.first == RouteWithoutProvider and s.rpc.get(2) == Rej)
    .then(routeAfterPreimage(1)).then(baseReturn)
    .expect(s.first == RouteWithoutProvider and s.innerCalls == 0 and s.rpc.get(1) == Rej and s.ppc == PDone and safety)

  // EXPECTED-BAD OLD02, T20 premises: the proofFile write without at('proving') moves the link out of 'failed' with first set.
  run oldT20LeavesFailedExpectedBadTest = toProofRead.then(observeSealed).then(oldRouteAfterProof(1))
    .expect(s.st == Routed and s.first == SealedOutOfOrder and s.captured and not(failedAbsorbingInv) and not(safety))
  // EXPECTED-BAD OLD02, T21 premises: unclaimed entry admits route 2, whose read is not parked; releasing route 1 reaches inner again.
  run oldT21DoubleInnerExpectedBadTest = init.then(providerProve).then(oldRouteEnter(1)).then(oldRouteEnter(2))
    .then(oldRouteAfterPreimage(2)).then(oldRouteAfterPreimage(1))
    .expect(s.innerCalls == 2 and s.first == NoErr and not(singleInnerInv) and not(singleClaimInv) and not(safety))
}
