module policies_test {
  import effects.* from "./effects"
  import consumption.* from "./consumption"
  import observations.* from "./observations"
  import policies.* from "./policies"
  import policies_harness.* from "./policies_harness"

  run settlementBranchTest = assert(bodyAllows(aliceBody, OpSettle, settleInput, 2, List(aliceLeg, bobLeg), facts))
  run refundBranchTest = assert(bodyAllows(aliceBody, OpVoluntaryRefund, refundInput, 2, List(aliceRefund, bobRefund), facts))
  run oneLegRejectedTest = assert(not(bodyAllows(aliceBody, OpSettle, settleInput, 2, List(aliceLeg), facts)))
  run afterResolveFullBindingTest = assert(policyAllowsObservation(aliceAfter, OpSettle, settleObservation, facts))
  run beforeResolveBoundedBodyTest = assert(policyAllowsObservation(aliceBefore, OpSettle, settleObservation, facts))
  run bothDebitOwnersTest = assert(allRequiredPoliciesAllow(Map(aliceKey -> aliceAfter, bobKey -> bobAfter), OpSettle, settleObservation, facts))
  run missingDebitOwnerTest = assert(not(allRequiredPoliciesAllow(Map(aliceKey -> aliceAfter), OpSettle, settleObservation, facts)))

  run reversedUnorderedEffectsTest = assert(bodyAllows(aliceBody, OpSettle, settleInput, 2, List(bobLeg, aliceLeg), facts))
  run exactOrderRejectsReversalTest = assert(not(bodyAllows({...aliceBody,
    clauses: List({...settleClause, effectOrder: ExactOrder})}, OpSettle, settleInput, 2, List(bobLeg, aliceLeg), facts)))
  run multiplicityCannotExpandTest = assert(not(bodyAllows(aliceBody, OpSettle, settleInput, 2, List(aliceLeg, bobLeg, aliceLeg),
    {...facts, ledger: fundedLedger.put((aliceEscrow, TokenA), 20)})))
  run multiplicityCannotShrinkTest = assert(not(bodyAllows({...aliceBody, clauses: List({...settleClause,
    requiredEffects: List(aliceLeg, bobLeg, aliceLeg), allowedEffects: List(aliceLeg, bobLeg, aliceLeg)})},
    OpSettle, settleInput, 2, List(aliceLeg, bobLeg), facts)))
  run wrongInputCannotRefundTest = assert(not(bodyAllows(aliceBody, OpVoluntaryRefund, settleInput, 2, List(aliceRefund, bobRefund), facts)))
  run labelOnlyRefundRejectedTest = assert(not(policyAllowsObservation(aliceBefore, OpVoluntaryRefund,
    {...settleObservation, outcome: VoluntaryRefund, effects: List(aliceRefund, bobRefund)}, facts)))
  run displayDoesNotExpandAuthorityTest = assert(not(policyAllowsObservation(aliceBefore, OpSettle,
    {...settleObservation, display: AlternateDisplay, effects: List(aliceLeg)}, facts)))
  run displayDoesNotRestrictPolicyTest = assert(policyAllowsObservation(aliceAfter, OpSettle,
    {...settleObservation, display: AlternateDisplay, outcome: VoluntaryRefund}, facts))
  run disclosuresDoNotExpandAuthorityTest = assert(not(policyAllowsObservation({...aliceBefore,
    body: {...aliceBody, disclosures: Set("everything")}}, OpSettle, {...settleObservation, effects: List(aliceLeg)}, facts)))
  run deadlineRefundTest = assert(bodyAllows(aliceBody, OpDeadlineRefund, NoInput, 100, List(aliceRefund, bobRefund),
    {...facts, environment: {...environment, physicalTime: 100}}))
  run suppliedInputAtDeadlineRejectedTest = assert(not(bodyAllows(aliceBody, OpDeadlineRefund, settleInput, 100,
    List(aliceRefund, bobRefund), {...facts, environment: {...environment, physicalTime: 100}})))
  run settlementAtDeadlineRejectedTest = assert(not(bodyAllows(aliceBody, OpSettle, settleInput, 100,
    List(aliceLeg, bobLeg), {...facts, environment: {...environment, physicalTime: 100}})))
  run backdatedExecutionRejectedTest = assert(not(bodyAllows(aliceBody, OpSettle, settleInput, 2,
    List(aliceLeg, bobLeg), {...facts, environment: {...environment, physicalTime: 100}})))
  run futureDatedExecutionRejectedTest = assert(not(bodyAllows(aliceBody, OpSettle, settleInput, 100, List(aliceLeg, bobLeg), facts)))
  run outsideFiniteTimeRejectedTest = assert(not(bodyAllows(aliceBody, OpSettle, settleInput, 3,
    List(aliceLeg, bobLeg), {...facts, environment: {...environment, physicalTime: 3}})))
  run expiredPolicyRejectedTest = assert(not(bodyAllows({...aliceBody, validUntil: 1}, OpSettle, settleInput, 2, List(aliceLeg, bobLeg), facts)))
  run notYetValidPolicyRejectedTest = assert(not(bodyAllows({...aliceBody, validFrom: 100}, OpSettle, settleInput, 2, List(aliceLeg, bobLeg), facts)))
  run implementationVersionMismatchTest = assert(not(bodyAllows(aliceBody, OpSettle, settleInput, 2,
    List(aliceLeg, bobLeg), {...facts, environment: {...environment, implementationVersion: 1}})))
  run mechanismMismatchTest = assert(not(bodyAllows(aliceBody, OpSettle, settleInput, 2,
    List(aliceLeg, bobLeg), {...facts, environment: {...environment, enforcementMechanism: 1}})))
  run foreignDebitLocationRejectedTest = assert(not(bodyAllows({...aliceBody, debitLocations: Set(aliceEscrow, bobEscrow)},
    OpSettle, settleInput, 2, List(aliceLeg, bobLeg), facts)))
  run omittedOwnDebitLocationRejectedTest = assert(not(bodyAllows({...aliceBody, debitLocations: Set()},
    OpSettle, settleInput, 2, List(aliceLeg, bobLeg), facts)))
  run policyUnderWrongMapKeyRejectedTest = assert(not(allRequiredPoliciesAllow(Map(aliceKey -> bobAfter, bobKey -> bobAfter),
    OpSettle, settleObservation, facts)))
  run emptyDebitConjunctionRejectedTest = assert(not(allRequiredPoliciesAllow(Map(), OpSettle,
    {...settleObservation, effects: List()}, facts)))
  run wrongNonceRejectedTest = assert(not(bodyAllows({...aliceBody, key: {...aliceKey, nonce: 0}},
    OpSettle, settleInput, 2, List(aliceLeg, bobLeg), facts)))
  run wrongDomainRejectedTest = assert(not(bodyAllows({...aliceBody, key: {...aliceKey, domain: InstallmentDomain}},
    OpSettle, settleInput, 2, List(aliceLeg, bobLeg), facts)))
  run missingCapabilityRejectedTest = assert(not(bodyAllows({...aliceBody, capabilities: Set()},
    OpSettle, settleInput, 2, List(aliceLeg, bobLeg), facts)))
  run unknownNonceRejectedTest = assert(not(validBody({...aliceBody, key: {...aliceKey, nonce: 2}})))
  run afterPlanIdentitySubstitutionTest = assert(not(policyAllowsObservation(aliceAfter, OpSettle,
    {...settleObservation, resolvedPlan: "different-plan"}, facts)))
  run afterArtifactSubstitutionTest = assert(not(policyAllowsObservation(aliceAfter, OpSettle,
    {...settleObservation, artifactAndCall: "different-call"}, facts)))
  run afterPredecessorSubstitutionTest = assert(not(policyAllowsObservation(aliceAfter, OpSettle,
    {...settleObservation, predecessor: "different-state"}, facts)))
  run afterSuccessorSubstitutionTest = assert(not(policyAllowsObservation(aliceAfter, OpSettle,
    {...settleObservation, proposedSuccessor: "different-result"}, facts)))
  run afterOrderedEffectsSubstitutionTest = assert(not(policyAllowsObservation(aliceAfter, OpSettle,
    {...settleObservation, effects: List(bobLeg, aliceLeg)}, facts)))
  run afterContextSubstitutionTest = assert(not(policyAllowsObservation(aliceAfter, OpSettle,
    settleObservation, {...facts, environment: {...environment, anchor: 1}})))
  run beforeSafeResolutionAlternativeTest = assert(policyAllowsObservation(aliceBefore, OpSettle,
    {...settleObservation, resolvedPlan: "safe-alternative", artifactAndCall: "another-valid-call"}, facts))
  run beforePinnedArtifactSubstitutionTest = assert(not(policyAllowsObservation({...aliceBefore,
    binding: BeforeResolution(PinnedArtifact("settle-call"))}, OpSettle,
    {...settleObservation, artifactAndCall: "different-call"}, facts)))
  run beforePinnedArtifactMatchTest = assert(policyAllowsObservation({...aliceBefore,
    binding: BeforeResolution(PinnedArtifact("settle-call"))}, OpSettle, settleObservation, facts))
  run mismatchedProfileBindingTest = assert(not(validPolicy({...aliceAfter, profile: SignBeforeResolve})))
  run emptyPlanRejectedTest = assert(not(validPolicy({...aliceAfter,
    binding: AfterResolution({identity: "settle-plan", operations: List()})})))

  run firstFillPolicyTest = assert(allRequiredPoliciesAllow(Map(parentKey -> parentBefore), fillOne, fillObservation, installmentFacts))
  run receivingBobNeedsNoPolicyTest = assert(requiredKeys(fillOne, List(installmentLeg)) == Set(parentKey))
  run secondFillPolicyTest = assert(bodyAllows(parentBody, fillTwo, NoInput, 2, List(installmentLeg), afterFillFacts))
  run secondFillWithoutFirstRejectedTest = assert(not(bodyAllows(parentBody, fillTwo, NoInput, 2, List(installmentLeg), installmentFacts)))
  run completeTwoSlotPlanTest = assert(validPolicy(parentAfter))
  run substitutedSecondSlotPlanRejectedTest = assert(not(validPolicy({...parentAfter,
    binding: AfterResolution({identity: "installment-plan", operations: List(fillOneOperation,
      {...fillTwoOperation, effects: List({...installmentLeg, quantity: 6})})})})))
  run substitutedSecondSlotDependencyRejectedTest = assert(not(validPolicy({...parentAfter,
    binding: AfterResolution({identity: "installment-plan", operations: List(fillOneOperation,
      {...fillTwoOperation, predecessorFacts: installmentFacts})})})))
  run effectsFreeCancelNeedsParentTest = assert(allRequiredPoliciesAllow(Map(parentKey -> parentBefore), cancel, cancelObservation, installmentFacts))
  run missingCancelAuthorityRejectedTest = assert(not(allRequiredPoliciesAllow(Map(), cancel, cancelObservation, installmentFacts)))
  run wrongCancelAuthorityRejectedTest = assert(not(allRequiredPoliciesAllow(Map(recoveryKey -> recoveryBefore), cancel, cancelObservation, installmentFacts)))
  run cancelWithoutCapabilityRejectedTest = assert(not(bodyAllows({...parentBody, capabilities: Set(FirstFillCapability, SecondFillCapability)},
    cancel, NoInput, 2, List(), installmentFacts)))
  run cancelAfterFirstFillPolicyTest = assert(bodyAllows(parentBody, cancel, NoInput, 2, List(), afterFillFacts))
  run cancelledParentCannotFillTest = assert(not(bodyAllows(parentBody, fillOne, NoInput, 2, List(installmentLeg), cancelledFacts)))
  run cancelledParentCannotFillSecondTest = assert(not(bodyAllows(parentBody, fillTwo, NoInput, 2, List(installmentLeg), cancelledAfterFillFacts)))
  run recoveryTenPolicyTest = assert(allRequiredPoliciesAllow(Map(recoveryKey -> recoveryBefore), recover, recoveryObservation, cancelledFacts))
  run recoveryFivePolicyTest = assert(allRequiredPoliciesAllow(Map(recoveryKey -> recoveryBefore), recover,
    {...recoveryObservation, effects: List({...aliceRefund, quantity: 5})}, cancelledAfterFillFacts))
  run recoveryBeforeCancellationRejectedTest = assert(not(allRequiredPoliciesAllow(Map(recoveryKey -> recoveryBefore), recover,
    recoveryObservation, installmentFacts)))
  run recoveryCannotReuseParentNonceTest = assert(not(allRequiredPoliciesAllow(Map(parentKey -> parentBefore), recover,
    recoveryObservation, cancelledFacts)))
  run recoveryRejectsCorruptedEscrowTest = assert(not(bodyAllows(recoveryBody, recover, NoInput, 2, List(aliceRefund),
    {...cancelledFacts, ledger: cancelledFacts.ledger.put((aliceEscrow, TokenA), 11)})))
  run partialRecoveryCannotLeaveEscrowTest = assert(not(bodyAllows(recoveryBody, recover, NoInput, 2,
    List({...aliceRefund, quantity: 5}), cancelledFacts)))
  run missingParentMapRejectedTest = assert(not(bodyAllows(parentBody, fillOne, NoInput, 2, List(installmentLeg),
    {...installmentFacts, parents: Map()})))
  run invalidSlotRejectedTest = assert(not(bodyAllows(parentBody, OpFillSlot({parent: parentKey, slot: 3}), NoInput, 2,
    List(installmentLeg), installmentFacts)))
  run depositPolicyTest = assert(validPolicy(depositAfter))
  run missingDepositDependencyRejectedTest = assert(not(validPolicy({...depositAfter,
    binding: AfterResolution({identity: "deposit-plan", operations: List({...depositOperation, effects: List()})})})))
  run mutatedDepositInputRejectedTest = assert(not(validPolicy({...depositAfter,
    binding: AfterResolution({identity: "deposit-plan", operations: List({...depositOperation,
      input: DepositLike({location: aliceEscrow, depositor: Alice, asset: TokenA, quantity: 9})})})})))
  run depositSourceInsufficientRejectedTest = assert(not(bodyAllows(depositBody, OpFund, depositInput, 2,
    List(depositEffect), {...depositFacts, ledger: initialLedger(9, 20)})))
  run afterCoreProjectionSubstitutionTest = assert(not(policyAllowsObservation(aliceAfter, OpSettle,
    {...settleObservation, coreProjection: CoreProjected({accepted: true, error: NoCoreError,
      warnings: List(), payments: List(), reductions: 0,
      state: {accounts: CORE_ACCOUNTS.mapBy(_ => 0), choices: Map(), continuation: "closed", minimumTime: 2}})}, facts)))
}
