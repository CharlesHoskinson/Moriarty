module installment_harness {
  import effects.* from "./effects"
  import consumption.* from "./consumption"
  import observations.* from "./observations"
  import policies.* from "./policies"
  import policies_harness.* from "./policies_harness"
  import authorization.* from "./authorization"
  import execution.* from "./execution"
  import installment_fixtures.* from "./installment_fixtures"
  var installment: InstallmentExecution
  var profile: SigningProfile
  action initAfter = all {installment' = unsignedInstallment, profile' = SignAfterResolve}
  action initBefore = all {installment' = unsignedInstallment, profile' = SignBeforeResolve}
  action init = any {initAfter, initBefore}
  action prepareParent = all {
    canPrepareParent(installment, profile),
    installment' = {...installment, authority: applyPrepareSigning(installment.authority, parentPolicy(profile), Alice)},
    profile' = profile,
  }
  action signParent = all {
    canSignParent(installment, profile),
    installment' = {...installment, authority: applySign(installment.authority, parentPolicy(profile), Alice, 0)},
    profile' = profile,
  }
  action proposeFirstFill = all {
    canProposeFirstFill(installment, profile),
    installment' = applyPropose(installment, FirstFillAttempt, fillOne, fillObservation, Bob),
    profile' = profile,
  }
  action proposeInitialCancellation = all {
    canProposeInitialCancellation(installment, profile),
    installment' = applyPropose(installment, CancelAttempt, cancel, cancelObservation, Alice),
    profile' = profile,
  }
  action verifyFirstFill = all {
    canVerifyWith(installment, FirstFillAttempt, parentPolicy(profile)),
    installment' = applyVerifyWith(installment, FirstFillAttempt, parentPolicy(profile)),
    profile' = profile,
  }
  action verifyInitialCancellation = all {
    canVerifyWith(installment, CancelAttempt, parentPolicy(profile)),
    installment' = applyVerifyWith(installment, CancelAttempt, parentPolicy(profile)),
    profile' = profile,
  }
  action commitFirstFill = all {
    canCommitFirstFill(installment), installment' = applyCommit(installment, FirstFillAttempt), profile' = profile,
  }
  action commitInitialCancellation = all {
    canCommitInitialCancellation(installment), installment' = applyCommit(installment, CancelAttempt), profile' = profile,
  }
  action rejectFirstFill = all {
    canRejectFirstFill(installment), installment' = applyRejectVerified(installment, FirstFillAttempt), profile' = profile,
  }
  action rejectInitialCancellation = all {
    canRejectInitialCancellation(installment), installment' = applyRejectVerified(installment, CancelAttempt), profile' = profile,
  }
  action proposeSecondFill = all {
    canProposeSecondFill(installment),
    installment' = applyPropose(installment, SecondFillAttempt, fillTwo, secondFillObservation, Bob), profile' = profile,
  }
  action verifySecondFill = all {
    canVerifyWith(installment, SecondFillAttempt, parentPolicy(profile)),
    installment' = applyVerifyWith(installment, SecondFillAttempt, parentPolicy(profile)), profile' = profile,
  }
  action commitSecondFill = all {
    canCommit(installment, SecondFillAttempt), installment' = applyCommit(installment, SecondFillAttempt), profile' = profile,
  }
  action proposeFreshCancellation = all {
    canProposeFreshCancellation(installment),
    installment' = applyPropose(installment, FreshCancelAttempt, cancel, freshCancellationObservation, Alice), profile' = profile,
  }
  action verifyFreshCancellation = all {
    canVerifyWith(installment, FreshCancelAttempt, parentPolicy(profile)),
    installment' = applyVerifyWith(installment, FreshCancelAttempt, parentPolicy(profile)), profile' = profile,
  }
  action commitFreshCancellation = all {
    canCommit(installment, FreshCancelAttempt), installment' = applyCommit(installment, FreshCancelAttempt), profile' = profile,
  }
  action prepareRecovery = all {
    canPrepareRecovery(installment, profile),
    installment' = {...installment, authority: applyPrepareSigning(installment.authority, recoveryPolicy(installment, profile), Alice)},
    profile' = profile,
  }
  action signRecovery = all {
    canSignRecovery(installment, profile),
    installment' = {...installment, authority: applySign(installment.authority, recoveryPolicy(installment, profile), Alice, 0)},
    profile' = profile,
  }
  action proposeRecovery = all {
    canProposeRecovery(installment, profile),
    installment' = applyPropose(installment, RecoveryAttempt, recover, actualRecoveryObservation(installment), Alice),
    profile' = profile,
  }
  action verifyRecovery = all {
    canVerifyWith(installment, RecoveryAttempt, recoveryPolicy(installment, profile)),
    installment' = applyVerifyWith(installment, RecoveryAttempt, recoveryPolicy(installment, profile)), profile' = profile,
  }
  action commitRecovery = all {
    canCommit(installment, RecoveryAttempt), installment' = applyCommit(installment, RecoveryAttempt), profile' = profile,
  }
  action step = any {prepareParent, signParent, proposeFirstFill, proposeInitialCancellation,
    verifyFirstFill, verifyInitialCancellation, commitFirstFill, commitInitialCancellation,
    rejectFirstFill, rejectInitialCancellation, proposeSecondFill, verifySecondFill, commitSecondFill,
    proposeFreshCancellation, verifyFreshCancellation, commitFreshCancellation,
    prepareRecovery, signRecovery, proposeRecovery, verifyRecovery, commitRecovery}
  val parentChecked = match installment.authority.signing.get(parentKey) {
    | PreparedSigning(_) => true
    | _ => false
  }
  val parentRegistered = registeredParent(installment, profile)
  val firstFillProposed = isProposed(installment, FirstFillAttempt)
  val initialCancellationProposed = isProposed(installment, CancelAttempt)
  val firstFillVerified = isVerified(installment, FirstFillAttempt)
  val initialCancellationVerified = isVerified(installment, CancelAttempt)
  val racePrepared = preparedRace(installment)
  val firstFillCommitted = isExecuted(installment, FirstFillAttempt)
  val initialCancellationCommitted = isExecuted(installment, CancelAttempt)
  val firstFillRejected = isRaceRejected(installment, FirstFillAttempt)
  val initialCancellationRejected = isRaceRejected(installment, CancelAttempt)
  val secondFillProposed = isProposed(installment, SecondFillAttempt)
  val secondFillVerified = isVerified(installment, SecondFillAttempt)
  val secondFillCommitted = isExecuted(installment, SecondFillAttempt)
  val freshCancellationProposed = isProposed(installment, FreshCancelAttempt)
  val freshCancellationVerified = isVerified(installment, FreshCancelAttempt)
  val freshCancellationCommitted = isExecuted(installment, FreshCancelAttempt)
  val recoveryChecked = match installment.authority.signing.get(recoveryKey) {
    | PreparedSigning(_) => true
    | _ => false
  }
  val recoveryRegistered = registeredRecovery(installment, profile)
  val recoveryProposed = isProposed(installment, RecoveryAttempt)
  val recoveryVerified = isVerified(installment, RecoveryAttempt)
  val recoveryCommitted = isExecuted(installment, RecoveryAttempt)
  val terminal = terminalState(installment)
  val installmentEnabled = lifecycleEnabled(installment, profile)
  val completedPaymentAfter = profile == SignAfterResolve and terminal and paymentCompleted(installment)
  val completedPaymentBefore = profile == SignBeforeResolve and terminal and paymentCompleted(installment)
  val recoveryTenAfter = profile == SignAfterResolve and terminal and recoveryCompleted(installment) and parentPaid(installment) == 0
  val recoveryTenBefore = profile == SignBeforeResolve and terminal and recoveryCompleted(installment) and parentPaid(installment) == 0
  val recoveryFiveAfter = profile == SignAfterResolve and terminal and recoveryCompleted(installment) and parentPaid(installment) == 5
  val recoveryFiveBefore = profile == SignBeforeResolve and terminal and recoveryCompleted(installment) and parentPaid(installment) == 5
  val installmentSafety = lifecycleSafety(installment, profile)
}
