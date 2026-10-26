module financialSegmentsTest {
  import financialSegments.* from "./financialSegments"
  run firstPaidPrefixTest = initCase(firstPrefix, AllOrNothing, 12, true)
    .then(statementStep).then(statementStep).then(statementStep)
    .expect(image.committed == genesis and image.tentative.reserve == 68)
    .then(commitStep)
    .expect(paidPrefix and allInvariants and image.committedEffects ==
      List(Payment(30), FeePaid(2), Discharged(30)))
  run secondPaidPrefixTest = initCase(firstPrefix, AllOrNothing, 12, true)
    .then(statementStep).then(statementStep).then(statementStep).then(commitStep)
    .then(resumeWith(secondPrefix, false))
    .then(statementStep).then(statementStep).then(statementStep).then(commitStep)
    .expect(allInvariants and image.outcome == Halted and image.committed ==
      {reserve: 57, recipient: 40, feePool: 3, liability: 0})
  run secondSegmentRollbackTest = initCase(firstPrefix, AllOrNothing, 12, true)
    .then(statementStep).then(statementStep).then(statementStep).then(commitStep)
    .then(resumeWith(retainedFailure, false))
    .then(statementStep).then(statementStep).then(statementStep).then(statementStep)
    .expect(allInvariants and image.outcome == Rejected and image.committed ==
      {reserve: 68, recipient: 30, feePool: 2, liability: 10})
    .expect(image.discarded == List(FeePaid(1), Payment(10), Discharged(10))
      and image.consumed > image.committedEvents and image.retained == List())
  run sealedFeeRetainedTest = initCase(retainedFailure, RetainFees, 10, false)
    .then(statementStep).then(statementStep).then(statementStep).then(statementStep)
    .expect(allInvariants and image.outcome == Rejected and image.committed ==
      {reserve: 99, recipient: 0, feePool: 1, liability: 40})
    .expect(image.retained == List(FeePaid(1)) and image.discarded ==
      List(Payment(10), Discharged(10)))
  run partialFeePhaseDiscardedTest = initCase(List(Fee(10), Fee(100)), RetainFees, 10, false)
    .then(statementStep).expect(image.committed == genesis)
    .then(statementStep)
    .expect(allInvariants and image.outcome == Rejected and image.committed == genesis
      and image.retained == List() and image.discarded == List(FeePaid(10)))
  run unsealedExhaustionTest = initCase(List(Fee(10), Fee(1)), RetainFees, 1, false)
    .then(statementStep).then(statementStep)
    .expect(allInvariants and image.outcome == Exhausted and image.committed == genesis)
  run sealedExhaustionTest = initCase(List(Fee(10), Pay(1)), RetainFees, 1, false)
    .then(statementStep).then(statementStep)
    .expect(allInvariants and image.outcome == Exhausted and image.committed.reserve == 90
      and image.retained == List(FeePaid(10)))
  run finalFeeExhaustionTest = initCase(List(Fee(10)), RetainFees, 1, false)
    .then(statementStep).then(commitStep)
    .expect(allInvariants and image.outcome == Exhausted and image.committed.reserve == 90)
  run defaultExhaustionTest = initCase(List(Fee(10)), AllOrNothing, 1, false)
    .then(statementStep).then(commitStep)
    .expect(allInvariants and image.outcome == Exhausted and image.committed == genesis)
  run technicalCutTest = initCase(firstPrefix, AllOrNothing, 12, true)
    .then(statementStep).then(cutStep)
    .expect(allInvariants and image.committed == genesis and image.consumed == 1
      and image.tentative.reserve == 70 and image.cuts == 1)
  run notReadyPreservesImageTest = initCase(firstPrefix, AllOrNothing, 12, true)
    .then(statementStep).then(statementStep).then(statementStep).then(commitStep)
    .then(notReadyStep)
    .expect(allInvariants and paidPrefix and image.remaining == 8 and image.consumed == 4
      and image.notReady == 1)
  run feeOrderingAdmissionTest = init
    .expect(not(policyAdmits(firstPrefix, RetainFees))
      and policyAdmits(retainedFailure, RetainFees)
      and policyAdmits(firstPrefix, AllOrNothing))
  run requiresBetweenFeesDiscardsPhaseTest =
    initCase(List(Fee(10), Requires(false), Fee(1)), RetainFees, 10, false)
      .then(statementStep).then(statementStep)
      .expect(allInvariants and image.outcome == Rejected and not(image.feeSealed)
        and image.committed == genesis and image.retained == List()
        and image.discarded == List(FeePaid(10)))
  run requiresAfterFeesRetainsPhaseTest =
    initCase(List(Fee(10), Requires(false)), RetainFees, 10, false)
      .then(statementStep).then(statementStep)
      .expect(allInvariants and image.outcome == Rejected and image.feeSealed
        and image.committed.reserve == 90 and image.retained == List(FeePaid(10))
        and image.discarded == List() and image.committed.liability == 40)
  run requiresBeforeFeesDiscardsNothingTest =
    initCase(List(Requires(false), Fee(10)), RetainFees, 10, false)
      .then(statementStep)
      .expect(allInvariants and image.outcome == Rejected and image.committed == genesis
        and image.retained == List() and image.discarded == List())
  run wholeActionFeeOrderingTest = init
    .expect(not(actionAdmits(List(List(Fee(1)), List(Pay(1), Fee(1))), RetainFees))
      and actionAdmits(List(List(Fee(1)), List(Fee(1), Pay(1))), RetainFees))
  run defaultPolicyIgnoresFeeOrderingTest = init
    .expect(actionAdmits(List(List(Fee(1)), List(Pay(1), Fee(1))), AllOrNothing))
  // SI-29 await-entry check (sherlock2.rs:319-325): fuel=4 exhausts at await
  // commit instead of publishing a zero-fuel Waiting image, rolling back
  // tentative via fail(...) rather than committing tentative per E2x.
  run zeroFuelWaitingExhaustsAtAwaitEntryTest = initCase(firstPrefix, AllOrNothing, 4, true)
    .then(statementStep).then(statementStep).then(statementStep).then(commitStep)
    .expect(allInvariants and waitingResumableInv and image.outcome == Exhausted
      and image.consumed == 4 and image.remaining == 0 and image.starvedAwait
      and image.committed == genesis and not(awaitStarvedCommitsPrefixInv))
}
