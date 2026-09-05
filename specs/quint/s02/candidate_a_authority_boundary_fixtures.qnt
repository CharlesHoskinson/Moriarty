module candidate_a_authority_boundary_fixtures {
  import effects.* from "./effects"
  import consumption.* from "./consumption"
  import observations.* from "./observations"
  import policies.* from "./policies"
  import authorization.* from "./authorization"
  import execution.* from "./execution"
  import candidate_a_types.* from "./candidate_a_types"
  import candidate_a_programs.* from "./candidate_a_programs"
  import candidate_a_authority_adapter.* from "./candidate_a_authority_adapter"
  import candidate_a_authority_boundary.* from "./candidate_a_authority_boundary"

  // Independently written literal funding expectation, never copied from an adapter result.
  pure val fundingKeyA: AuthorityKey = {domain: SwapDomain, principal: Alice, nonce: 0}
  pure val fundingBeforeA: APredecessor = {program: canonicalSwap, state: {...emptyAState(N6), minimumTime: Time1}}
  pure val fundingNextA: APredecessor = {program: canonicalSwap, state: {
    accounts: CORE_ACCOUNTS.mapBy(_ => 0).put(aliceA, 10),
    choices: A_CHOICE_IDS.mapBy(_ => NoInt), continuation: N5, minimumTime: Time2}}
  pure val fundingLedgerA = initialLedger(10, 0)
  pure val fundingEffectA: Transfer = {source: Wallet(Alice), destination: Escrow(aliceA), asset: TokenA, quantity: 10}
  pure val fundingInputA: NeutralInput = DepositLike({location: Escrow(aliceA), depositor: Alice, asset: TokenA, quantity: 10})
  pure val fundingRequestA: AAuthorityRequest = {before: fundingBeforeA,
    input: PresentAInput(DepositInputA({account: aliceA, depositor: Alice, quantity: 10})), now: Time2}
  pure val fundingFactsA: PolicyFacts = {ledger: fundingLedgerA,
    environment: {physicalTime: 2, anchor: 0, implementationVersion: 0, enforcementMechanism: 0},
    parents: AUTHORITY_KEYS.mapBy(_ => ParentAbsent)}
  pure val fundingProjectionA: CoreProjection[AContinuation] = CoreProjected({
    accepted: true, error: NoCoreError, payments: List(), warnings: List(), reductions: 0,
    state: {accounts: CORE_ACCOUNTS.mapBy(_ => 0).put(aliceA, 10),
      choices: Map("settle" -> NoInt, "fill1" -> NoInt, "fill2" -> NoInt, "recover" -> NoInt, "other" -> NoInt),
      continuation: {program: canonicalSwap, node: N5}, minimumTime: 2}})
  pure val fundingPlannedA: AAuthorityPlanned = {predecessor: fundingBeforeA, proposedSuccessor: fundingNextA,
    artifactAndCall: AgreementCallA(fundingRequestA), input: fundingInputA, operation: OpFund,
    transactionTime: 2, effects: List(fundingEffectA), coreProjection: fundingProjectionA, predecessorFacts: fundingFactsA}
  pure val fundingPlanA: AResolvedPlan = {identity: SwapPlanA, operations: List(fundingPlannedA)}
  pure val fundingExpectedA: AAuthorityObservation = {predecessor: fundingBeforeA, proposedSuccessor: fundingNextA,
    artifactAndCall: AgreementCallA(fundingRequestA), resolvedPlan: fundingPlanA, input: fundingInputA,
    transactionTime: 2, effects: List(fundingEffectA), coreProjection: fundingProjectionA,
    outcome: FundingAccepted, effectEvidence: EvidenceValid, display: PublicDisplay}
  pure val fundingAdaptationA = adaptAuthorityA(fundingRequestA, OpFund, fundingPlanA, PublicDisplay)
  pure val fundingFixtureValidA = fundingAdaptationA == AuthorityAdaptedA(fundingExpectedA)
    and validState(canonicalSwap, fundingBeforeA.state) and fundingBeforeA.state.minimumTime == Time1
    and validPolicyFacts(fundingFactsA)
  pure val fundingBodyA: PolicyBody = {key: fundingKeyA, debitLocations: Set(Wallet(Alice)),
    capabilities: Set(FundCapability), disclosures: Set(), validFrom: 1, validUntil: 99,
    implementationVersion: 0, enforcementMechanism: 0,
    clauses: List({operation: OpFund, conditions: List(InputIs(fundingInputA), BeforeTime(100),
      SourceBalanceIs({location: Wallet(Alice), asset: TokenA, quantity: 10})),
      requiredEffects: List(fundingEffectA), allowedEffects: List(fundingEffectA), effectOrder: ExactOrder})}
  pure def fundingPolicyA(profile: SigningProfile): AAuthorityPolicy = {body: fundingBodyA, profile: profile,
    binding: if (profile == SignAfterResolve) AfterResolution({identity: fundingPlanA, operations: fundingPlanA.operations})
      else BeforeResolution(AnyArtifactUnderMechanism)}
  pure val fundingContextA: AAuthorityContext = {candidate: fundingBeforeA, ledger: fundingLedgerA,
    environment: fundingFactsA.environment, registry: AUTHORITY_KEYS.mapBy(_ => AuthorityUnused),
    parents: AUTHORITY_KEYS.mapBy(_ => ParentVacant)}
  pure val fundingUnsignedA: AAuthorityExecution = {authority: {context: fundingContextA,
    signing: AUTHORITY_KEYS.mapBy(_ => NoSigningCheck)}, attempts: ATTEMPT_IDS.mapBy(_ => NoAttempt)}
  pure def fundingPreparedA(profile: SigningProfile): AAuthorityExecution = {...fundingUnsignedA,
    authority: applyPrepareSigning(fundingUnsignedA.authority, fundingPolicyA(profile), Alice)}
  pure def fundingSignedA(profile: SigningProfile): AAuthorityExecution = {...fundingPreparedA(profile),
    authority: applySign(fundingPreparedA(profile).authority, fundingPolicyA(profile), Alice, 0)}
  // Fixture updates are used only with separate guard assertions in tests/actions.
  pure def fundingProposedA(profile: SigningProfile): AAuthorityExecution = match fundingAdaptationA {
    | AuthorityAdaptedA(obs) => applyPropose(fundingSignedA(profile), FundingOneAttempt, OpFund, obs, Alice)
    | _ => fundingSignedA(profile)
  }
  pure def fundingAttemptA(profile: SigningProfile): AAuthorityAttempt = {
    id: FundingOneAttempt, actor: Alice, operation: OpFund, observation: fundingExpectedA,
    context: fundingSignedA(profile).authority.context}
  pure def fundingEvidenceForA(attempt: AAuthorityAttempt, profile: SigningProfile): AAuthorityEvidence = {
    effect: {attempt: attempt, disposition: EvidenceValid},
    signatures: Map(fundingKeyA -> {attempt: attempt, disposition: EvidenceValid,
      signed: {policy: fundingPolicyA(profile), signer: Alice, token: 0}})}
  pure def fundingEvidenceA(profile: SigningProfile): AAuthorityEvidence = fundingEvidenceForA(fundingAttemptA(profile), profile)
  pure def planWithSecondA(second: AAuthorityPlanned): AResolvedPlan =
    {...fundingPlanA, operations: List(fundingPlannedA, second)}

  pure def fundingVerifiedA(profile: SigningProfile): AAuthorityExecution =
    applyVerify(fundingProposedA(profile), FundingOneAttempt, fundingEvidenceA(profile))
  pure def fundingCommittedA(profile: SigningProfile): AAuthorityExecution =
    applyCommit(fundingVerifiedA(profile), FundingOneAttempt)
  pure def replaceProposedA(state: AAuthorityExecution, attempt: AAuthorityAttempt): AAuthorityExecution =
    {...state, attempts: state.attempts.put(FundingOneAttempt, ProposedAttempt(attempt))}
  pure def replaceVerifiedA(state: AAuthorityExecution, attempt: AAuthorityAttempt,
    evidence: AAuthorityEvidence): AAuthorityExecution =
    {...state, attempts: state.attempts.put(FundingOneAttempt, VerifiedOperation({attempt: attempt, evidence: evidence}))}
  pure val forgedFundingAttemptA: AAuthorityAttempt = {...fundingAttemptA(SignBeforeResolve),
    observation: {...fundingExpectedA, proposedSuccessor: fundingBeforeA}}
  pure val forgedFundingEvidenceA = fundingEvidenceForA(forgedFundingAttemptA, SignBeforeResolve)
  pure val forgedFundingProposedA = replaceProposedA(fundingProposedA(SignBeforeResolve), forgedFundingAttemptA)
  pure val forgedFundingVerifiedA = replaceVerifiedA(fundingVerifiedA(SignBeforeResolve),
    forgedFundingAttemptA, forgedFundingEvidenceA)
  pure def expectedFundingCommitA(profile: SigningProfile): AAuthorityExecution = {
    val verified = {attempt: fundingAttemptA(profile), evidence: fundingEvidenceA(profile)}
    val signed = {policy: fundingPolicyA(profile), signer: Alice, token: 0}
    {...fundingSignedA(profile), authority: {...fundingSignedA(profile).authority,
      context: {...fundingSignedA(profile).authority.context, candidate: fundingNextA,
        ledger: fundingLedgerA.put((Wallet(Alice), TokenA), 0).put((Escrow(aliceA), TokenA), 10),
        registry: fundingSignedA(profile).authority.context.registry
          .put(fundingKeyA, AuthorityConsumed({signed: signed, revision: 1}))}},
      attempts: fundingUnsignedA.attempts.put(FundingOneAttempt, ExecutedOperation(verified))}
  }
  pure def expectedRejectionA(state: AAuthorityExecution, attempt: AAuthorityAttempt,
    evidence: AAuthorityEvidence, reason: RejectionReason, stage: RejectionStage): AAuthorityExecution =
    {...state, attempts: state.attempts.put(FundingOneAttempt, RejectedOperation({
      attempt: attempt, evidence: evidence, observedContext: state.authority.context, reason: reason, stage: stage}))}
}
