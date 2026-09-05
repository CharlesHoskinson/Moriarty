module installment_fixtures {
  import effects.* from "./effects"
  import consumption.* from "./consumption"
  import observations.* from "./observations"
  import policies.* from "./policies"
  import policies_harness.* from "./policies_harness"
  import authorization.* from "./authorization"
  import execution.* from "./execution"

  // Finite neutral fixtures; no candidate interpreter or generated proof.
  type InstallmentExecution = ExecutionState[str,str,str,str]
  pure val unsignedInstallment: InstallmentExecution = {
    authority: {context: {candidate: "unfilled", ledger: installmentFacts.ledger, environment: environment,
      registry: AUTHORITY_KEYS.mapBy(_ => AuthorityUnused), parents: AUTHORITY_KEYS.mapBy(_ => ParentVacant)},
      signing: AUTHORITY_KEYS.mapBy(_ => NoSigningCheck)},
    attempts: ATTEMPT_IDS.mapBy(_ => NoAttempt)}
  pure val initialCancellationPlan = {...fillOneOperation, operation: cancel,
    proposedSuccessor: "cancelled", artifactAndCall: "cancel-call", effects: List()}
  pure val freshCancellationPlan = {...initialCancellationPlan, predecessor: "first-filled",
    predecessorFacts: afterFillFacts}
  pure val completeParentAfter: TestPolicy = {...parentAfter,
    binding: AfterResolution({identity: "installment-plan", operations: List(
      fillOneOperation, fillTwoOperation, initialCancellationPlan, freshCancellationPlan)})}
  pure def parentPolicy(profile: SigningProfile): TestPolicy =
    if (profile == SignAfterResolve) completeParentAfter else parentBefore
  pure def registeredParent(state: InstallmentExecution, profile: SigningProfile): bool =
    state.authority.context.registry.get(parentKey)
      == AuthorityRegistered({policy: parentPolicy(profile), signer: Alice, token: 0})
  pure def isProposed(state: InstallmentExecution, id: AttemptId): bool = match state.attempts.get(id) {
    | ProposedAttempt(_) => true
    | _ => false
  }
  pure def isVerified(state: InstallmentExecution, id: AttemptId): bool = match state.attempts.get(id) {
    | VerifiedOperation(_) => true
    | _ => false
  }
  pure def isExecuted(state: InstallmentExecution, id: AttemptId): bool = match state.attempts.get(id) {
    | ExecutedOperation(_) => true
    | _ => false
  }
  pure def isRaceRejected(state: InstallmentExecution, id: AttemptId): bool = match state.attempts.get(id) {
    | RejectedOperation(rejected) => rejected.stage == CommitBoundary and rejected.reason == StaleBindings
    | _ => false
  }
  // This constructor models a fresh external verifier response for the actual
  // stored proposal and exact signed policy. EvidenceValid is an assumption.
  pure def evidenceFor(attempt: PreparedAttempt[str,str,str,str], policy: TestPolicy): EvidenceBundle[str,str,str,str] = {
    effect: {attempt: attempt, disposition: EvidenceValid},
    signatures: Map(policy.body.key -> {attempt: attempt,
      signed: {policy: policy, signer: Alice, token: 0}, disposition: EvidenceValid})}
  pure def canVerifyWith(state: InstallmentExecution, id: AttemptId, policy: TestPolicy): bool =
    match state.attempts.get(id) {
      | ProposedAttempt(attempt) => canVerify(state, id, evidenceFor(attempt, policy))
      | _ => false
    }
  pure def applyVerifyWith(state: InstallmentExecution, id: AttemptId, policy: TestPolicy): InstallmentExecution =
    match state.attempts.get(id) {
      | ProposedAttempt(attempt) => applyVerify(state, id, evidenceFor(attempt, policy))
      | _ => state
    }
  pure def canPrepareParent(state: InstallmentExecution, profile: SigningProfile): bool =
    canPrepareSigning(state.authority, parentPolicy(profile), Alice)
  pure def canSignParent(state: InstallmentExecution, profile: SigningProfile): bool =
    canSign(state.authority, parentPolicy(profile), Alice, 0)
  pure def canProposeFirstFill(state: InstallmentExecution, profile: SigningProfile): bool =
    registeredParent(state, profile) and state.authority.context.candidate == "unfilled"
      and canPropose(state, FirstFillAttempt, fillOne, fillObservation, Bob)
  pure def canProposeInitialCancellation(state: InstallmentExecution, profile: SigningProfile): bool =
    registeredParent(state, profile) and state.authority.context.candidate == "unfilled"
      and canPropose(state, CancelAttempt, cancel, cancelObservation, Alice)
  pure def preparedRace(state: InstallmentExecution): bool =
    match state.attempts.get(FirstFillAttempt) {
      | VerifiedOperation(fill) => match state.attempts.get(CancelAttempt) {
          | VerifiedOperation(cancellation) => fill.attempt.context == cancellation.attempt.context
              and fill.attempt.context == state.authority.context
              and parentFacts(state.authority.context.parents.get(parentKey)) == parentInitial
          | _ => false
        }
      | _ => false
    }

  pure val secondFillObservation = {...fillObservation, predecessor: "first-filled", proposedSuccessor: "completed",
    artifactAndCall: "fill-two-call", outcome: SecondInstallment}
  pure val freshCancellationObservation = {...cancelObservation, predecessor: "first-filled"}
  pure val recoveryTenPlan: PlannedOperation[str,str,str] = {...initialCancellationPlan,
    predecessor: "cancelled", proposedSuccessor: "recovered", artifactAndCall: "recovery-call",
    operation: recover, effects: List(aliceRefund), predecessorFacts: cancelledFacts}
  pure val recoveryFivePlan = {...recoveryTenPlan, effects: List({...aliceRefund, quantity: 5}),
    predecessorFacts: cancelledAfterFillFacts}
  pure val recoveryAfterTen: TestPolicy = {...recoveryBefore, profile: SignAfterResolve,
    binding: AfterResolution({identity: "recovery-plan", operations: List(recoveryTenPlan)})}
  pure val recoveryAfterFive: TestPolicy = {...recoveryAfterTen,
    binding: AfterResolution({identity: "recovery-plan", operations: List(recoveryFivePlan)})}
  pure def parentPaid(state: InstallmentExecution): int = match state.authority.context.parents.get(parentKey) {
    | ParentLive(live) => live.entry.paid
    | ParentVacant => 0
  }
  // Retained parent accounting selects the fixed expected recovery plan. Actual
  // refund effects separately read the current ledger; verification must match both.
  pure def recoveryPolicy(state: InstallmentExecution, profile: SigningProfile): TestPolicy =
    if (profile == SignBeforeResolve) recoveryBefore
    else if (parentPaid(state) == 0) recoveryAfterTen else recoveryAfterFive
  pure def actualRecoveryObservation(state: InstallmentExecution): TestObservation = {
    ...recoveryObservation, resolvedPlan: "recovery-plan",
    effects: List({...aliceRefund, quantity: balance(state.authority.context.ledger, aliceEscrow, TokenA)}),
  }
  pure def firstFillWon(state: InstallmentExecution): bool = isExecuted(state, FirstFillAttempt)
    and isRaceRejected(state, CancelAttempt)
  pure def cancellationWon(state: InstallmentExecution): bool = isExecuted(state, CancelAttempt)
    and isRaceRejected(state, FirstFillAttempt)
  pure def cancellationReadyForRecovery(state: InstallmentExecution): bool =
    state.authority.context.candidate == "cancelled"
      and (cancellationWon(state) or (firstFillWon(state) and isExecuted(state, FreshCancelAttempt)))
  pure def canCommitFirstFill(state: InstallmentExecution): bool =
    preparedRace(state) and canCommit(state, FirstFillAttempt)
  pure def canCommitInitialCancellation(state: InstallmentExecution): bool =
    preparedRace(state) and canCommit(state, CancelAttempt)
  pure def canRejectFirstFill(state: InstallmentExecution): bool =
    isExecuted(state, CancelAttempt) and canRejectVerified(state, FirstFillAttempt)
  pure def canRejectInitialCancellation(state: InstallmentExecution): bool =
    isExecuted(state, FirstFillAttempt) and canRejectVerified(state, CancelAttempt)
  // A single successor branch is selected by the first proposal. This models
  // the declared initial fill/cancel race, not an additional slot-two race.
  pure def canProposeSecondFill(state: InstallmentExecution): bool = firstFillWon(state)
    and state.authority.context.candidate == "first-filled" and state.attempts.get(FreshCancelAttempt) == NoAttempt
    and canPropose(state, SecondFillAttempt, fillTwo, secondFillObservation, Bob)
  pure def canProposeFreshCancellation(state: InstallmentExecution): bool = firstFillWon(state)
    and state.authority.context.candidate == "first-filled" and state.attempts.get(SecondFillAttempt) == NoAttempt
    and canPropose(state, FreshCancelAttempt, cancel, freshCancellationObservation, Alice)
  pure def canPrepareRecovery(state: InstallmentExecution, profile: SigningProfile): bool =
    cancellationReadyForRecovery(state) and canPrepareSigning(state.authority, recoveryPolicy(state, profile), Alice)
  pure def canSignRecovery(state: InstallmentExecution, profile: SigningProfile): bool =
    cancellationReadyForRecovery(state) and canSign(state.authority, recoveryPolicy(state, profile), Alice, 0)
  pure def registeredRecovery(state: InstallmentExecution, profile: SigningProfile): bool =
    state.authority.context.registry.get(recoveryKey)
      == AuthorityRegistered({policy: recoveryPolicy(state, profile), signer: Alice, token: 0})
  pure def canProposeRecovery(state: InstallmentExecution, profile: SigningProfile): bool =
    cancellationReadyForRecovery(state) and registeredRecovery(state, profile)
      and canPropose(state, RecoveryAttempt, recover, actualRecoveryObservation(state), Alice)

  pure def noPendingAttempts(state: InstallmentExecution): bool = ATTEMPT_IDS.forall(id =>
    match state.attempts.get(id) {
      | ProposedAttempt(_) => false
      | VerifiedOperation(_) => false
      | _ => true
    })
  pure def noPendingSigning(state: InstallmentExecution): bool = AUTHORITY_KEYS.forall(key =>
    match state.authority.signing.get(key) {
      | PreparedSigning(_) => false
      | _ => true
    })
  pure def paymentCompleted(state: InstallmentExecution): bool = isExecuted(state, SecondFillAttempt)
    and state.authority.context.candidate == "completed" and firstFillWon(state)
    and balance(state.authority.context.ledger, Wallet(Bob), TokenA) == 10
    and balance(state.authority.context.ledger, Wallet(Alice), TokenA) == 0
    and balance(state.authority.context.ledger, aliceEscrow, TokenA) == 0
  pure def recoveryCompleted(state: InstallmentExecution): bool = isExecuted(state, RecoveryAttempt)
    and state.authority.context.candidate == "recovered"
    and balance(state.authority.context.ledger, aliceEscrow, TokenA) == 0
  pure def terminalState(state: InstallmentExecution): bool = (paymentCompleted(state) or recoveryCompleted(state))
    and noPendingAttempts(state) and noPendingSigning(state)
  pure def lifecycleEnabled(state: InstallmentExecution, profile: SigningProfile): bool =
    canPrepareParent(state, profile) or canSignParent(state, profile)
      or canProposeFirstFill(state, profile) or canProposeInitialCancellation(state, profile)
      or canVerifyWith(state, FirstFillAttempt, parentPolicy(profile))
      or canVerifyWith(state, CancelAttempt, parentPolicy(profile))
      or canCommitFirstFill(state) or canCommitInitialCancellation(state)
      or canRejectFirstFill(state) or canRejectInitialCancellation(state)
      or canProposeSecondFill(state) or canVerifyWith(state, SecondFillAttempt, parentPolicy(profile))
      or canCommit(state, SecondFillAttempt)
      or canProposeFreshCancellation(state) or canVerifyWith(state, FreshCancelAttempt, parentPolicy(profile))
      or canCommit(state, FreshCancelAttempt)
      or canPrepareRecovery(state, profile) or canSignRecovery(state, profile) or canProposeRecovery(state, profile)
      or canVerifyWith(state, RecoveryAttempt, recoveryPolicy(state, profile)) or canCommit(state, RecoveryAttempt)
  pure def parentMoneyCoupled(state: InstallmentExecution, profile: SigningProfile): bool =
    match state.authority.context.parents.get(parentKey) {
      | ParentVacant => state.authority.context.ledger == installmentFacts.ledger
      | ParentLive(live) => live.parent.policy == {policy: parentPolicy(profile), signer: Alice, token: 0}
          and balance(state.authority.context.ledger, Wallet(Bob), TokenA) == live.entry.paid
          and (if (isExecuted(state, RecoveryAttempt)) live.entry.cancelled
            and balance(state.authority.context.ledger, aliceEscrow, TokenA) == 0
            and balance(state.authority.context.ledger, Wallet(Alice), TokenA) == live.entry.remainingAllowance
          else balance(state.authority.context.ledger, aliceEscrow, TokenA) == live.entry.remainingAllowance
            and balance(state.authority.context.ledger, Wallet(Alice), TokenA) == 0)
    }
  pure def recoveryPreservesParent(state: InstallmentExecution): bool = match state.attempts.get(RecoveryAttempt) {
    | ExecutedOperation(done) => state.authority.context.parents == done.attempt.context.parents
        and state.authority.context.registry.get(parentKey) == done.attempt.context.registry.get(parentKey)
        and state.authority.context.registry.get(recoveryKey)
          == AuthorityConsumed({signed: done.evidence.signatures.get(recoveryKey).signed, revision: 1})
    | _ => true
  }
  pure def raceHistoryPreserved(state: InstallmentExecution): bool = ATTEMPT_IDS.forall(id =>
    match state.attempts.get(id) {
      | RejectedOperation(rejected) => Set(FirstFillAttempt, CancelAttempt).contains(id)
          and rejected.attempt.id == id and rejected.stage == CommitBoundary and rejected.reason == StaleBindings
          and rejected.attempt.context != rejected.observedContext
          and rejected.evidence.effect.attempt == rejected.attempt
          and rejected.evidence.signatures.keys() == Set(parentKey)
          and rejected.evidence.signatures.get(parentKey).attempt == rejected.attempt
          and verificationValid(rejected.attempt.context, rejected.attempt, rejected.evidence)
      | _ => true
    })
  pure def lifecycleSafety(state: InstallmentExecution, profile: SigningProfile): bool =
    validExecutionState(state)
      and ASSETS.forall(asset => totalAsset(state.authority.context.ledger, asset)
        == totalAsset(installmentFacts.ledger, asset))
      and parentMoneyCoupled(state, profile) and recoveryPreservesParent(state) and raceHistoryPreserved(state)
      and (if (terminalState(state)) not(lifecycleEnabled(state, profile)) else lifecycleEnabled(state, profile))
}
