module policies_harness {
  import effects.* from "./effects"
  import consumption.* from "./consumption"
  import observations.* from "./observations"
  import policies.* from "./policies"

  // Concrete fixture aliases, not an executable A–D candidate or signing trace.
  type TestObservation = CandidateObservation[str,str,str,str]
  type TestPolicy = Policy[str,str,str,str]
  pure val aliceKey = {domain: SwapDomain, principal: Alice, nonce: 1}
  pure val bobKey = {domain: SwapDomain, principal: Bob, nonce: 1}
  pure val parentKey = {domain: InstallmentDomain, principal: Alice, nonce: 0}
  pure val recoveryKey = {domain: InstallmentDomain, principal: Alice, nonce: 1}
  pure val aliceEscrow = Escrow({owner: Alice, asset: TokenA})
  pure val bobEscrow = Escrow({owner: Bob, asset: TokenB})
  pure val aliceLeg = {source: aliceEscrow, destination: Wallet(Bob), asset: TokenA, quantity: 10}
  pure val bobLeg = {source: bobEscrow, destination: Wallet(Alice), asset: TokenB, quantity: 20}
  pure val aliceRefund = {...aliceLeg, destination: Wallet(Alice)}
  pure val bobRefund = {...bobLeg, destination: Wallet(Bob)}
  pure val settleInput = ChoiceLike({id: "settle", chooser: Bob, chosen: 1})
  pure val refundInput = ChoiceLike({id: "settle", chooser: Bob, chosen: 0})
  pure val environment: Environment = {physicalTime: 2, anchor: 0, implementationVersion: 0, enforcementMechanism: 0}
  pure val fundedLedger = initialLedger(0, 0).put((aliceEscrow, TokenA), 10).put((bobEscrow, TokenB), 20)
  pure val facts: PolicyFacts = {ledger: fundedLedger, environment: environment,
    parents: AUTHORITY_KEYS.mapBy(_ => ParentAbsent)}
  pure val settleClause: Clause = {operation: OpSettle, conditions: List(InputIs(settleInput), BeforeTime(100)),
    requiredEffects: List(aliceLeg, bobLeg), allowedEffects: List(aliceLeg, bobLeg), effectOrder: AnyOrder}
  pure val refundClause: Clause = {operation: OpVoluntaryRefund, conditions: List(InputIs(refundInput), BeforeTime(100)),
    requiredEffects: List(aliceRefund, bobRefund), allowedEffects: List(aliceRefund, bobRefund), effectOrder: AnyOrder}
  pure val deadlineClause: Clause = {...refundClause, operation: OpDeadlineRefund,
    conditions: List(InputIs(NoInput), AtOrAfterTime(100))}
  pure val aliceBody: PolicyBody = {key: aliceKey, clauses: List(settleClause, refundClause, deadlineClause),
    debitLocations: Set(aliceEscrow), capabilities: Set(DisposeCapability), disclosures: Set("public"),
    validFrom: 1, validUntil: 101, implementationVersion: 0, enforcementMechanism: 0}
  pure val bobBody = {...aliceBody, key: bobKey, debitLocations: Set(bobEscrow)}
  pure val settleObservation: TestObservation = {predecessor: "funded", proposedSuccessor: "closed",
    artifactAndCall: "settle-call", resolvedPlan: "settle-plan", transactionTime: 2, input: settleInput,
    effects: List(aliceLeg, bobLeg), outcome: Settlement, coreProjection: NoCoreProjection,
    effectEvidence: EvidenceValid, display: PublicDisplay}
  pure val settleOperation: PlannedOperation[str,str,str] = {predecessor: "funded", proposedSuccessor: "closed",
    artifactAndCall: "settle-call", input: settleInput, operation: OpSettle, transactionTime: 2,
    effects: List(aliceLeg, bobLeg), coreProjection: NoCoreProjection, predecessorFacts: facts}
  pure val aliceAfter: TestPolicy = {body: aliceBody, profile: SignAfterResolve,
    binding: AfterResolution({identity: "settle-plan", operations: List(settleOperation)})}
  pure val bobAfter: TestPolicy = {...aliceAfter, body: bobBody}
  pure val aliceBefore: TestPolicy = {body: aliceBody, profile: SignBeforeResolve,
    binding: BeforeResolution(AnyArtifactUnderMechanism)}
  pure val bobBefore: TestPolicy = {...aliceBefore, body: bobBody}

  pure val parentInitial = ParentPresent({cancelled: false, usedSlots: Set(), paid: 0, remainingAllowance: 10, revision: 0})
  pure val parentAfterFill = ParentPresent({cancelled: false, usedSlots: Set(1), paid: 5, remainingAllowance: 5, revision: 1})
  pure val parentCancelled = ParentPresent({cancelled: true, usedSlots: Set(), paid: 0, remainingAllowance: 10, revision: 1})
  pure val parentCancelledAfterFill = ParentPresent({cancelled: true, usedSlots: Set(1), paid: 5, remainingAllowance: 5, revision: 2})
  pure val installmentFacts: PolicyFacts = {...facts,
    ledger: initialLedger(0, 0).put((aliceEscrow, TokenA), 10),
    parents: facts.parents.put(parentKey, parentInitial)}
  pure val installmentLeg = {...aliceLeg, quantity: 5}
  pure val afterFillFacts = {...installmentFacts,
    ledger: applyTransfers(installmentFacts.ledger, List(installmentLeg)),
    parents: installmentFacts.parents.put(parentKey, parentAfterFill)}
  pure val cancelledFacts = {...installmentFacts, parents: installmentFacts.parents.put(parentKey, parentCancelled)}
  pure val cancelledAfterFillFacts = {...afterFillFacts, parents: afterFillFacts.parents.put(parentKey, parentCancelledAfterFill)}
  pure val fillOne = OpFillSlot({parent: parentKey, slot: 1})
  pure val fillTwo = OpFillSlot({parent: parentKey, slot: 2})
  pure val cancel = OpCancelParent(parentKey)
  pure val recover = OpRecover({parent: parentKey, recovery: recoveryKey})
  pure val fillOneClause: Clause = {operation: fillOne,
    conditions: List(InputIs(NoInput), BeforeTime(100), ParentMatches({key: parentKey, expected: parentInitial})),
    requiredEffects: List(installmentLeg), allowedEffects: List(installmentLeg), effectOrder: ExactOrder}
  pure val fillTwoClause = {...fillOneClause, operation: fillTwo,
    conditions: List(InputIs(NoInput), BeforeTime(100), ParentMatches({key: parentKey, expected: parentAfterFill}))}
  pure val cancelInitialClause: Clause = {operation: cancel,
    conditions: List(InputIs(NoInput), ParentMatches({key: parentKey, expected: parentInitial})),
    requiredEffects: List(), allowedEffects: List(), effectOrder: ExactOrder}
  pure val cancelAfterFillClause = {...cancelInitialClause,
    conditions: List(InputIs(NoInput), ParentMatches({key: parentKey, expected: parentAfterFill}))}
  pure val parentBody: PolicyBody = {...aliceBody, key: parentKey,
    clauses: List(fillOneClause, fillTwoClause, cancelInitialClause, cancelAfterFillClause),
    capabilities: Set(FirstFillCapability, SecondFillCapability, CancelCapability)}
  pure val parentBefore: TestPolicy = {...aliceBefore, body: parentBody}
  pure val fillObservation: TestObservation = {...settleObservation, input: NoInput,
    predecessor: "unfilled", proposedSuccessor: "first-filled", artifactAndCall: "fill-one-call",
    resolvedPlan: "installment-plan", effects: List(installmentLeg), outcome: FirstInstallment}
  pure val cancelObservation: TestObservation = {...fillObservation, proposedSuccessor: "cancelled",
    artifactAndCall: "cancel-call", effects: List(), outcome: Cancellation}
  pure val fillOneOperation: PlannedOperation[str,str,str] = {predecessor: "unfilled", proposedSuccessor: "first-filled",
    artifactAndCall: "fill-one-call", input: NoInput, operation: fillOne, transactionTime: 2,
    effects: List(installmentLeg), coreProjection: NoCoreProjection, predecessorFacts: installmentFacts}
  pure val fillTwoOperation = {...fillOneOperation, predecessor: "first-filled", proposedSuccessor: "completed",
    artifactAndCall: "fill-two-call", operation: fillTwo, predecessorFacts: afterFillFacts}
  pure val parentAfter: TestPolicy = {...parentBefore, profile: SignAfterResolve,
    binding: AfterResolution({identity: "installment-plan", operations: List(fillOneOperation, fillTwoOperation)})}
  pure val recoveryTenClause: Clause = {operation: recover,
    conditions: List(InputIs(NoInput), ParentMatches({key: parentKey, expected: parentCancelled}),
      SourceBalanceIs({location: aliceEscrow, asset: TokenA, quantity: 10})),
    requiredEffects: List(aliceRefund), allowedEffects: List(aliceRefund), effectOrder: ExactOrder}
  pure val recoveryFiveClause: Clause = {...recoveryTenClause,
    conditions: List(InputIs(NoInput), ParentMatches({key: parentKey, expected: parentCancelledAfterFill}),
      SourceBalanceIs({location: aliceEscrow, asset: TokenA, quantity: 5})),
    requiredEffects: List({...aliceRefund, quantity: 5}), allowedEffects: List({...aliceRefund, quantity: 5})}
  pure val recoveryBody: PolicyBody = {...aliceBody, key: recoveryKey,
    clauses: List(recoveryTenClause, recoveryFiveClause), capabilities: Set(RecoveryCapability)}
  pure val recoveryBefore: TestPolicy = {...aliceBefore, body: recoveryBody}
  pure val recoveryObservation: TestObservation = {...cancelObservation,
    predecessor: "cancelled", proposedSuccessor: "recovered", artifactAndCall: "recovery-call",
    effects: List(aliceRefund), outcome: Recovery}
  pure val depositEffect = {source: Wallet(Alice), destination: aliceEscrow, asset: TokenA, quantity: 10}
  pure val depositInput = DepositLike({location: aliceEscrow, depositor: Alice, asset: TokenA, quantity: 10})
  pure val depositFacts = {...facts, ledger: initialLedger(10, 20)}
  pure val depositClause: Clause = {operation: OpFund, conditions: List(InputIs(depositInput), BeforeTime(100)),
    requiredEffects: List(depositEffect), allowedEffects: List(depositEffect), effectOrder: ExactOrder}
  pure val depositBody: PolicyBody = {...aliceBody, key: {...aliceKey, nonce: 0}, clauses: List(depositClause),
    debitLocations: Set(Wallet(Alice)), capabilities: Set(FundCapability)}
  pure val depositOperation: PlannedOperation[str,str,str] = {...settleOperation, predecessor: "empty", proposedSuccessor: "alice-funded",
    artifactAndCall: "deposit-call", input: depositInput, operation: OpFund,
    effects: List(depositEffect), predecessorFacts: depositFacts}
  pure val depositAfter: TestPolicy = {...aliceAfter, body: depositBody,
    binding: AfterResolution({identity: "deposit-plan", operations: List(depositOperation)})}
}
