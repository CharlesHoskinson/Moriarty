module execution_test {
  import effects.* from "./effects"
  import consumption.* from "./consumption"
  import observations.* from "./observations"
  import policies.* from "./policies"
  import policies_harness.* from "./policies_harness"
  import authorization.* from "./authorization"
  import execution.* from "./execution"
  import execution_harness.* from "./execution_harness"

  run proposeSettlementTest = init.then(proposeSettlement).expect(settlementProposed)
  run cannotVerifyWithoutAttemptTest = assert(not(canVerify(signedAfter, DispositionAttempt,
    settlementEvidence(signedAfter, aliceAfter, bobAfter))))
  run cannotCommitWithoutVerificationTest = assert(not(canCommit(signedAfter, DispositionAttempt)))
  run verifySettlementTest = init.then(proposeSettlement).then(verifySettlement).expect(settlementVerified)
  pure val proposedAfter = applyPropose(signedAfter, DispositionAttempt, OpSettle, settleObservation, Bob)
  pure val proposedBefore = applyPropose(signedBefore, DispositionAttempt, OpSettle, settleObservation, Bob)
  run verifyBeforeResolutionTest = assert(canVerify(proposedBefore, DispositionAttempt,
    settlementEvidence(signedBefore, aliceBefore, bobBefore)))
}
