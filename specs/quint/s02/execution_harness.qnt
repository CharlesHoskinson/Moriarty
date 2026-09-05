module execution_harness {
  import effects.* from "./effects"
  import consumption.* from "./consumption"
  import observations.* from "./observations"
  import policies.* from "./policies"
  import policies_harness.* from "./policies_harness"
  import authorization.* from "./authorization"
  import execution.* from "./execution"

  type FixtureExecution = ExecutionState[str,str,str,str]
  pure val signingInitial: SigningState[str,str,str,str] = {
    context: {candidate: "funded", ledger: fundedLedger, environment: environment,
      registry: AUTHORITY_KEYS.mapBy(_ => AuthorityUnused), parents: AUTHORITY_KEYS.mapBy(_ => ParentVacant)},
    signing: AUTHORITY_KEYS.mapBy(_ => NoSigningCheck)}
  pure val executionInitial: FixtureExecution = {authority: signingInitial,
    attempts: ATTEMPT_IDS.mapBy(_ => NoAttempt)}
  pure def signedFixture(alice: TestPolicy, bob: TestPolicy): FixtureExecution = {
    val checkedAlice = applyPrepareSigning(signingInitial, alice, Alice)
    val checkedBoth = applyPrepareSigning(checkedAlice, bob, Bob)
    val signedAlice = applySign(checkedBoth, alice, Alice, 0)
    {...executionInitial, authority: applySign(signedAlice, bob, Bob, 0)}
  }
  pure val signedAfter = signedFixture(aliceAfter, bobAfter)
  pure val signedBefore = signedFixture(aliceBefore, bobBefore)
  pure def settlementAttempt(state: FixtureExecution): PreparedAttempt[str,str,str,str] = {
    id: DispositionAttempt, actor: Bob, operation: OpSettle, observation: settleObservation,
    context: state.authority.context}
  pure def settlementEvidence(state: FixtureExecution, alice: TestPolicy, bob: TestPolicy): EvidenceBundle[str,str,str,str] = {
    val attempt = settlementAttempt(state)
    {effect: {attempt: attempt, disposition: EvidenceValid}, signatures: Map(
      aliceKey -> {attempt: attempt, signed: {policy: alice, signer: Alice, token: 0}, disposition: EvidenceValid},
      bobKey -> {attempt: attempt, signed: {policy: bob, signer: Bob, token: 0}, disposition: EvidenceValid})}
  }
  var executionState: FixtureExecution
  action init = executionState' = signedAfter
  action proposeSettlement = all {
    canPropose(executionState, DispositionAttempt, OpSettle, settleObservation, Bob),
    executionState' = applyPropose(executionState, DispositionAttempt, OpSettle, settleObservation, Bob),
  }
  action verifySettlement = all {
    canVerify(executionState, DispositionAttempt, settlementEvidence(executionState, aliceAfter, bobAfter)),
    executionState' = applyVerify(executionState, DispositionAttempt, settlementEvidence(executionState, aliceAfter, bobAfter)),
  }
  action commitSettlement = all {
    canCommit(executionState, DispositionAttempt),
    executionState' = applyCommit(executionState, DispositionAttempt),
  }
  action step = any {proposeSettlement, verifySettlement, commitSettlement}
  val settlementProposed = match executionState.attempts.get(DispositionAttempt) {
    | ProposedAttempt(_) => true
    | _ => false
  }
  val settlementVerified = match executionState.attempts.get(DispositionAttempt) {
    | VerifiedOperation(_) => true
    | _ => false
  }
  val proposalPreservesMoney = executionState.authority.context.ledger == fundedLedger
  val settlementCommitted = match executionState.attempts.get(DispositionAttempt) {
    | ExecutedOperation(_) => true
    | _ => false
  }
  val settlementMoney = balance(executionState.authority.context.ledger, Wallet(Bob), TokenA) == 10
    and balance(executionState.authority.context.ledger, Wallet(Alice), TokenB) == 20
    and balance(executionState.authority.context.ledger, aliceEscrow, TokenA) == 0
    and balance(executionState.authority.context.ledger, bobEscrow, TokenB) == 0
  val executionSafety = validExecutionState(executionState)
    and ASSETS.forall(asset => totalAsset(executionState.authority.context.ledger, asset) == totalAsset(fundedLedger, asset))
    and (if (settlementCommitted) settlementMoney else proposalPreservesMoney)
}
