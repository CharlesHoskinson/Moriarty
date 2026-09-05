module installment_adversarial_test {
  import effects.* from "./effects"
  import consumption.* from "./consumption"
  import observations.* from "./observations"
  import policies.* from "./policies"
  import policies_harness.* from "./policies_harness"
  import authorization.* from "./authorization"
  import execution.* from "./execution"
  import installment_fixtures.* from "./installment_fixtures"
  import installment_harness.* from "./installment_harness"

  // This module checks lifecycle guards. EvidenceValid remains a trusted
  // external-verifier abstraction; these checks do not interpret candidates.
  val recoveryNonceUnused = recoveryKey.nonce == 1
    and installment.authority.context.registry.get(recoveryKey) == AuthorityUnused
  val losingFillCannotCommit = not(canCommit(installment, FirstFillAttempt))
  val losingCancellationCannotCommit = not(canCommit(installment, CancelAttempt))
  val recoveryCannotReplay = not(canProposeRecovery(installment, profile))
    and not(canCommit(installment, RecoveryAttempt))

  // Constructed mutations below are guard checks, not lifecycle traces.
  pure def withAttempt(state: InstallmentExecution,
                       other: PreparedAttempt[str,str,str,str]): InstallmentExecution =
    {...state, attempts: state.attempts.put(other.id, ProposedAttempt(other))}
  val recoveryMissingProofRejected = match installment.attempts.get(RecoveryAttempt) {
    | ProposedAttempt(attempt) => not(canVerify(installment, RecoveryAttempt,
        {...evidenceFor(attempt, recoveryPolicy(installment, profile)), signatures: Map()}))
    | _ => false
  }
  val recoveryInvalidProofRejected = match installment.attempts.get(RecoveryAttempt) {
    | ProposedAttempt(attempt) => {
        val evidence = evidenceFor(attempt, recoveryPolicy(installment, profile))
        not(canVerify(installment, RecoveryAttempt,
          {...evidence, effect: {...evidence.effect, disposition: EvidenceInvalid}}))
      }
    | _ => false
  }
  val recoveryWrongAmountRejected = match installment.attempts.get(RecoveryAttempt) {
    | ProposedAttempt(attempt) => {
        val other = {...attempt, observation: {...attempt.observation,
          effects: List({...aliceRefund, quantity: 1})}}
        not(canVerify(withAttempt(installment, other), RecoveryAttempt,
          evidenceFor(other, recoveryPolicy(installment, profile))))
      }
    | _ => false
  }
  val recoveryWrongRecipientRejected = match installment.attempts.get(RecoveryAttempt) {
    | ProposedAttempt(attempt) => {
        val other = {...attempt, observation: {...attempt.observation,
          effects: List({...attempt.observation.effects.head(), destination: Wallet(Bob)})}}
        not(canVerify(withAttempt(installment, other), RecoveryAttempt,
          evidenceFor(other, recoveryPolicy(installment, profile))))
      }
    | _ => false
  }

  run cancellationWinnerAfterGuardTest = initAfter.then(prepareParent).then(signParent)
    .then(proposeFirstFill).then(proposeInitialCancellation)
    .then(verifyFirstFill).then(verifyInitialCancellation).then(commitInitialCancellation)
    .expect(initialCancellationCommitted and losingFillCannotCommit
      and balance(installment.authority.context.ledger, aliceEscrow, TokenA) == 10
      and not(recoveryCommitted))
  run cancellationWinnerBeforeGuardTest = initBefore.then(prepareParent).then(signParent)
    .then(proposeInitialCancellation).then(proposeFirstFill)
    .then(verifyInitialCancellation).then(verifyFirstFill).then(commitInitialCancellation)
    .expect(initialCancellationCommitted and losingFillCannotCommit
      and balance(installment.authority.context.ledger, aliceEscrow, TokenA) == 10
      and not(recoveryCommitted))

  run fillWinnerAfterGuardTest = initAfter.then(prepareParent).then(signParent)
    .then(proposeFirstFill).then(proposeInitialCancellation)
    .then(verifyFirstFill).then(verifyInitialCancellation).then(commitFirstFill)
    .then(rejectInitialCancellation)
    .expect(firstFillCommitted and losingCancellationCannotCommit and canProposeSecondFill(installment)
      and not(canProposeFirstFill(installment, profile)))
  run fillWinnerBeforeGuardTest = initBefore.then(prepareParent).then(signParent)
    .then(proposeInitialCancellation).then(proposeFirstFill)
    .then(verifyInitialCancellation).then(verifyFirstFill).then(commitFirstFill)
    .then(rejectInitialCancellation)
    .expect(firstFillCommitted and losingCancellationCannotCommit and canProposeSecondFill(installment)
      and not(canProposeFirstFill(installment, profile)))

  run recoveryUnavailableBeforeCancellationAfterTest = initAfter.then(prepareParent).then(signParent)
    .expect(recoveryNonceUnused and not(canPrepareRecovery(installment, profile))
      and not(canSignRecovery(installment, profile)) and not(canProposeRecovery(installment, profile)))
  run recoveryUnavailableBeforeCancellationBeforeTest = initBefore.then(prepareParent).then(signParent)
    .expect(recoveryNonceUnused and not(canPrepareRecovery(installment, profile))
      and not(canSignRecovery(installment, profile)) and not(canProposeRecovery(installment, profile)))

  run recoveryNonceUnusedUntilSigningTest = initAfter.then(prepareParent).then(signParent)
    .then(proposeFirstFill).then(proposeInitialCancellation)
    .then(verifyFirstFill).then(verifyInitialCancellation).then(commitInitialCancellation)
    .then(rejectFirstFill).then(prepareRecovery)
    .expect(recoveryChecked and recoveryNonceUnused and not(recoveryRegistered))

  run recoveryProofGuardsTest = initBefore.then(prepareParent).then(signParent)
    .then(proposeInitialCancellation).then(proposeFirstFill)
    .then(verifyInitialCancellation).then(verifyFirstFill).then(commitInitialCancellation)
    .then(rejectFirstFill).then(prepareRecovery).then(signRecovery).then(proposeRecovery)
    .expect(recoveryProposed and recoveryMissingProofRejected and recoveryInvalidProofRejected
      and recoveryWrongAmountRejected and recoveryWrongRecipientRejected)

  run recoveredAuthorityCannotReplayTest = initAfter.then(prepareParent).then(signParent)
    .then(proposeFirstFill).then(proposeInitialCancellation)
    .then(verifyFirstFill).then(verifyInitialCancellation).then(commitInitialCancellation)
    .then(rejectFirstFill).then(prepareRecovery).then(signRecovery).then(proposeRecovery)
    .then(verifyRecovery).then(commitRecovery)
    .expect(recoveryCommitted and recoveryCannotReplay
      and installment.authority.context.registry.get(recoveryKey) != AuthorityUnused)
}
