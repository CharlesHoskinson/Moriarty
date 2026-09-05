module execution_test {
  import effects.* from "./effects"
  import consumption.* from "./consumption"
  import observations.* from "./observations"
  import policies.* from "./policies"
  import policies_harness.* from "./policies_harness"
  import authorization.* from "./authorization"
  import execution.* from "./execution"
  import execution_harness.* from "./execution_harness"

  run proposeSettlementTest = initSignedAfter.then(proposeSettlement).expect(settlementProposed)
  run cannotVerifyWithoutAttemptTest = assert(not(canVerify(signedAfter, DispositionAttempt,
    settlementEvidence(signedAfter, aliceAfter, bobAfter))))
  run cannotCommitWithoutVerificationTest = assert(not(canCommit(signedAfter, DispositionAttempt)))
  run verifySettlementTest = initSignedAfter.then(proposeSettlement).then(verifySettlement).expect(settlementVerified)
  pure val proposedAfter = applyPropose(signedAfter, DispositionAttempt, OpSettle, settleObservation, Bob)
  pure val proposedBefore = applyPropose(signedBefore, DispositionAttempt, OpSettle, settleObservation, Bob)
  run verifyBeforeResolutionTest = assert(canVerify(proposedBefore, DispositionAttempt,
    settlementEvidence(signedBefore, aliceBefore, bobBefore)))
  run commitSettlementTest = initSignedAfter.then(proposeSettlement).then(verifySettlement).then(commitSettlement)
    .expect(settlementCommitted and settlementMoney)
  run fullAfterResolutionPipelineTest = initAfter.then(prepareAlice).then(prepareBob).then(signAlice).then(signBob)
    .then(proposeSettlement).then(verifySettlement).then(commitSettlement).expect(completeAfter and executionSafety)
  run fullBeforeResolutionPipelineTest = initBefore.then(prepareBob).then(prepareAlice).then(signBob).then(signAlice)
    .then(proposeSettlement).then(verifySettlement).then(commitSettlement).expect(completeBefore and executionSafety)
}
