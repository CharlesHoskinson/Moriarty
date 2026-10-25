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
  // Current-code characterization, not endorsement of design SI-29.
  run zeroFuelWaitingCounterexampleTest = initCase(firstPrefix, AllOrNothing, 4, true)
    .then(statementStep).then(statementStep).then(statementStep).then(commitStep)
    .expect(allInvariants and paidPrefix and not(waitingResumableInv))
    .then(resumeWith(secondPrefix, false))
    .expect(allInvariants and image.outcome == Exhausted and image.consumed == 4)
}
