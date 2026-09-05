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
}
