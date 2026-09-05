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
  var selectedProfile: SigningProfile
  action initSignedAfter = all {executionState' = signedAfter, selectedProfile' = SignAfterResolve}
  action initAfter = all {executionState' = executionInitial, selectedProfile' = SignAfterResolve}
  action initBefore = all {executionState' = executionInitial, selectedProfile' = SignBeforeResolve}
  action init = any {initAfter, initBefore}
  val activeAlice = if (selectedProfile == SignAfterResolve) aliceAfter else aliceBefore
  val activeBob = if (selectedProfile == SignAfterResolve) bobAfter else bobBefore
  val signaturesReady = executionState.authority.context.registry.get(aliceKey)
      == AuthorityRegistered({policy: activeAlice, signer: Alice, token: 0})
    and executionState.authority.context.registry.get(bobKey)
      == AuthorityRegistered({policy: activeBob, signer: Bob, token: 0})
  action prepareAlice = all {
    canPrepareSigning(executionState.authority, activeAlice, Alice),
    executionState' = {...executionState, authority: applyPrepareSigning(executionState.authority, activeAlice, Alice)},
    selectedProfile' = selectedProfile,
  }
  action prepareBob = all {
    canPrepareSigning(executionState.authority, activeBob, Bob),
    executionState' = {...executionState, authority: applyPrepareSigning(executionState.authority, activeBob, Bob)},
    selectedProfile' = selectedProfile,
  }
  action signAlice = all {
    canSign(executionState.authority, activeAlice, Alice, 0),
    executionState' = {...executionState, authority: applySign(executionState.authority, activeAlice, Alice, 0)},
    selectedProfile' = selectedProfile,
  }
  action signBob = all {
    canSign(executionState.authority, activeBob, Bob, 0),
    executionState' = {...executionState, authority: applySign(executionState.authority, activeBob, Bob, 0)},
    selectedProfile' = selectedProfile,
  }
  action proposeSettlement = all {
    signaturesReady,
    canPropose(executionState, DispositionAttempt, OpSettle, settleObservation, Bob),
    executionState' = applyPropose(executionState, DispositionAttempt, OpSettle, settleObservation, Bob),
    selectedProfile' = selectedProfile,
  }
  action verifySettlement = all {
    canVerify(executionState, DispositionAttempt, settlementEvidence(executionState, activeAlice, activeBob)),
    executionState' = applyVerify(executionState, DispositionAttempt, settlementEvidence(executionState, activeAlice, activeBob)),
    selectedProfile' = selectedProfile,
  }
  action commitSettlement = all {
    canCommit(executionState, DispositionAttempt),
    executionState' = applyCommit(executionState, DispositionAttempt),
    selectedProfile' = selectedProfile,
  }
  action step = any {prepareAlice, prepareBob, signAlice, signBob, proposeSettlement, verifySettlement, commitSettlement}
  val aliceChecked = match executionState.authority.signing.get(aliceKey) {
    | PreparedSigning(_) => true
    | _ => false
  }
  val bobChecked = match executionState.authority.signing.get(bobKey) {
    | PreparedSigning(_) => true
    | _ => false
  }
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
  val settlementAuthorityConsumed = executionState.authority.context.registry.get(aliceKey)
      == AuthorityConsumed({signed: {policy: activeAlice, signer: Alice, token: 0}, revision: 1})
    and executionState.authority.context.registry.get(bobKey)
      == AuthorityConsumed({signed: {policy: activeBob, signer: Bob, token: 0}, revision: 1})
  val completeAfter = selectedProfile == SignAfterResolve and settlementCommitted
  val completeBefore = selectedProfile == SignBeforeResolve and settlementCommitted
  val noOutstandingAttempt = ATTEMPT_IDS.forall(id => match executionState.attempts.get(id) {
    | NoAttempt => true
    | ExecutedOperation(_) => true
    | _ => false
  })
  val executionEnabled = canPrepareSigning(executionState.authority, activeAlice, Alice)
    or canPrepareSigning(executionState.authority, activeBob, Bob)
    or canSign(executionState.authority, activeAlice, Alice, 0)
    or canSign(executionState.authority, activeBob, Bob, 0)
    or (signaturesReady and canPropose(executionState, DispositionAttempt, OpSettle, settleObservation, Bob))
    or canVerify(executionState, DispositionAttempt, settlementEvidence(executionState, activeAlice, activeBob))
    or canCommit(executionState, DispositionAttempt)
  val executionSafety = validExecutionState(executionState)
    and ASSETS.forall(asset => totalAsset(executionState.authority.context.ledger, asset) == totalAsset(fundedLedger, asset))
    and (if (settlementCommitted) settlementMoney and settlementAuthorityConsumed
      and executionState.authority.context.candidate == "closed" and noOutstandingAttempt and not(executionEnabled)
      else proposalPreservesMoney and executionEnabled)
}
