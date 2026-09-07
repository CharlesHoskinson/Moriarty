module candidate_a_authority_boundary_test_f {
  import effects.* from "../effects"
  import consumption.* from "../consumption"
  import observations.* from "../observations"
  import policies.* from "../policies"
  import authorization.* from "../authorization"
  import execution.* from "../execution"
  import candidate_a_types.* from "../candidate_a_types"
  import candidate_a_programs.* from "../candidate_a_programs"
  import candidate_a_authority_adapter_f.* from "./candidate_a_authority_adapter_f"
  import candidate_a_authority_boundary_f.* from "./candidate_a_authority_boundary_f"
  import candidate_a_authority_boundary_fixtures_f.* from "./candidate_a_authority_boundary_fixtures_f"
  import candidate_a_authority_boundary_harness_f.* from "./candidate_a_authority_boundary_harness_f"

  run literalFundingFixtureTest = fundingFixtureValidA and validAuthorityContext(fundingContextA)
    and fundingAdaptationA == AuthorityAdaptedA(fundingExpectedA)
  run honestPlansAndPoliciesTest = authorityPlanMatchesA(fundingPlanA)
    and authorityPlanMatchesA(planWithSecondA(fundingPlannedA))
    and validAuthorityPolicyA(fundingPolicyA(SignAfterResolve))
    and validAuthorityPolicyA(fundingPolicyA(SignBeforeResolve))
  run planCardinalityTest = not(authorityPlanMatchesA({...fundingPlanA, operations: List()}))
    and not(authorityPlanMatchesA({...fundingPlanA,
      operations: List(fundingPlannedA, fundingPlannedA, fundingPlannedA, fundingPlannedA, fundingPlannedA)}))
  run secondSuccessorAndInputTest =
    not(authorityPlanMatchesA(planWithSecondA({...fundingPlannedA, proposedSuccessor: fundingBeforeA})))
    and not(authorityPlanMatchesA(planWithSecondA({...fundingPlannedA, input: NoInput})))
  run secondArtifactAndTimeTest = {
    val changedProgram = {...canonicalSwap, nodes: canonicalSwap.nodes.put(N15,
      PayA({account: aliceA, payee: Bob, amount: ConstantA(5), continuation: N0}))}
    val changedCall = AgreementCallA({...fundingRequestA, before: {...fundingBeforeA, program: changedProgram}})
    not(authorityPlanMatchesA(planWithSecondA({...fundingPlannedA, artifactAndCall: changedCall})))
      and not(authorityPlanMatchesA(planWithSecondA({...fundingPlannedA, transactionTime: 1})))
  }
  run secondProjectionAndEffectsTest =
    not(authorityPlanMatchesA(planWithSecondA({...fundingPlannedA, coreProjection: NoCoreProjection})))
      and not(authorityPlanMatchesA(planWithSecondA({...fundingPlannedA, effects: List()})))
  run secondFactsCouplingAndClockTest = {
    val mismatchedLedger = fundingLedgerA.put((Escrow(aliceA), TokenA), 5)
    not(authorityPlanMatchesA(planWithSecondA({...fundingPlannedA,
      predecessorFacts: {...fundingFactsA, ledger: mismatchedLedger}})))
      and not(authorityPlanMatchesA(planWithSecondA({...fundingPlannedA,
        predecessorFacts: {...fundingFactsA, environment: {...fundingFactsA.environment, physicalTime: 1}}})))
  }
  run missingPlanMapsFailSafelyTest = {
    val missingAccounts = {...fundingBeforeA, state: {...fundingBeforeA.state, accounts: Map()}}
    val missingLedger = LEDGER_KEYS.exclude(Set((Escrow(aliceA), TokenA))).mapBy(k => fundingLedgerA.get(k))
    val missingParents = AUTHORITY_KEYS.exclude(Set(fundingKeyA)).mapBy(_ => ParentAbsent)
    all {
      not(authorityPredecessorLedgerMatchesA(missingAccounts, fundingLedgerA)),
      not(authorityPredecessorLedgerMatchesA(fundingBeforeA, missingLedger)),
      not(authorityPlannedOperationMatchesA({...fundingPlannedA, predecessor: missingAccounts}, fundingPlanA)),
      not(authorityPlanMatchesA(planWithSecondA({...fundingPlannedA, predecessorFacts: {...fundingFactsA, ledger: missingLedger}}))),
      not(authorityPlanMatchesA(planWithSecondA({...fundingPlannedA, predecessorFacts: {...fundingFactsA, parents: missingParents}}))),
    }
  }
  pure val mismatchedAfterPolicyA: AAuthorityPolicy = {...fundingPolicyA(SignAfterResolve),
    binding: AfterResolution({identity: fundingPlanA,
      operations: List({...fundingPlannedA, proposedSuccessor: fundingBeforeA})})}
  run afterViewMismatchPreparationTest = validPolicy(mismatchedAfterPolicyA)
    and canPrepareSigning(fundingUnsignedA.authority, mismatchedAfterPolicyA, Alice)
    and not(validAuthorityPolicyA(mismatchedAfterPolicyA))
    and not(canPrepareSigningAuthorityA(fundingUnsignedA.authority, mismatchedAfterPolicyA, Alice))
  run beforePreparationWithoutConcretePlanTest =
    canPrepareSigningAuthorityA(fundingUnsignedA.authority, fundingPolicyA(SignBeforeResolve), Alice)
  pure def signingPipelineChecksA(profile: SigningProfile): bool = {
    val policy = fundingPolicyA(profile)
    val prepared = fundingPreparedA(profile)
    val signed = fundingSignedA(profile)
    fundingFixtureValidA and validAuthoritySigningA(fundingUnsignedA.authority)
      and canPrepareSigningAuthorityA(fundingUnsignedA.authority, policy, Alice)
      and canSignAuthorityA(prepared.authority, policy, Alice, 0)
      and validAuthoritySigningA(signed.authority)
      and prepared.authority.context == fundingContextA
      and signed.authority.context == {...fundingContextA,
        registry: fundingContextA.registry.put(fundingKeyA, AuthorityRegistered({policy: policy, signer: Alice, token: 0}))}
      and signed.authority.signing == fundingUnsignedA.authority.signing
        .put(fundingKeyA, CompletedSigning({policy: policy, signer: Alice, token: 0}))
      and signed.attempts == fundingUnsignedA.attempts
  }
  run signingAfterPipelineTest = signingPipelineChecksA(SignAfterResolve)
  run signingBeforePipelineTest = signingPipelineChecksA(SignBeforeResolve)
  run invalidPlanRecheckedAtSigningTest = {
    val checked = applyPrepareSigning(fundingUnsignedA.authority, mismatchedAfterPolicyA, Alice)
    canSign(checked, mismatchedAfterPolicyA, Alice, 0)
      and not(canSignAuthorityA(checked, mismatchedAfterPolicyA, Alice, 0))
  }
  run coupledStateRecheckedAtSigningTest = {
    val malformed = {...fundingUnsignedA.authority, context: {...fundingContextA,
      ledger: fundingLedgerA.put((Escrow(aliceA), TokenA), 5)}}
    val policy = fundingPolicyA(SignBeforeResolve)
    val checked = applyPrepareSigning(malformed, policy, Alice)
    all {
      validSigningState(malformed),
      not(validAuthoritySigningA(malformed)),
      not(canPrepareSigningAuthorityA(malformed, policy, Alice)),
      canSign(checked, policy, Alice, 0),
      not(canSignAuthorityA(checked, policy, Alice, 0)),
    }
  }

  pure def fundingPipelineChecksA(profile: SigningProfile): bool = {
    val signed = fundingSignedA(profile)
    val proposed = fundingProposedA(profile)
    val verified = fundingVerifiedA(profile)
    val committed = fundingCommittedA(profile)
    signingPipelineChecksA(profile)
      and canPropose(signed, FundingOneAttempt, OpFund, fundingExpectedA, Alice)
      and proposed.attempts.get(FundingOneAttempt) == ProposedAttempt(fundingAttemptA(profile))
      and authorityAttemptMatchesA(fundingAttemptA(profile))
      and canVerifyAuthorityA(proposed, FundingOneAttempt, fundingEvidenceA(profile))
      and verified.authority == signed.authority
      and verified.attempts.get(FundingOneAttempt) == VerifiedOperation({
        attempt: fundingAttemptA(profile), evidence: fundingEvidenceA(profile)})
      and canCommitAuthorityA(verified, FundingOneAttempt)
      and committed == expectedFundingCommitA(profile)
      and validAuthorityExecutionA(committed)
      and totalAsset(committed.authority.context.ledger, TokenA) == 10
      and totalAsset(committed.authority.context.ledger, TokenB) == 0
      and not(canCommitAuthorityA(committed, FundingOneAttempt))
  }
  run actualAfterFundingPipelineTest = fundingPipelineChecksA(SignAfterResolve)
  run actualBeforeFundingPipelineTest = fundingPipelineChecksA(SignBeforeResolve)
  run forgedProposedRejectedReachablyTest = {
    val rejected = applyRejectProposedAuthorityA(forgedFundingProposedA, FundingOneAttempt, forgedFundingEvidenceA)
    all {
      fundingPipelineChecksA(SignBeforeResolve),
      canVerify(forgedFundingProposedA, FundingOneAttempt, forgedFundingEvidenceA),
      not(canRejectProposed(forgedFundingProposedA, FundingOneAttempt, forgedFundingEvidenceA)),
      not(canVerifyAuthorityA(forgedFundingProposedA, FundingOneAttempt, forgedFundingEvidenceA)),
      canRejectProposedAuthorityA(forgedFundingProposedA, FundingOneAttempt, forgedFundingEvidenceA),
      rejected == expectedRejectionA(forgedFundingProposedA, forgedFundingAttemptA,
        forgedFundingEvidenceA, UnauthorizedEffect, VerificationBoundary),
      not(canRejectProposedAuthorityA(rejected, FundingOneAttempt, forgedFundingEvidenceA)),
      not(canRejectVerifiedAuthorityA(rejected, FundingOneAttempt)),
      applyRejectProposedAuthorityA(rejected, FundingOneAttempt, forgedFundingEvidenceA) == rejected,
    }
  }
  run forgedVerifiedRejectedReachablyTest = {
    val rejected = applyRejectVerifiedAuthorityA(forgedFundingVerifiedA, FundingOneAttempt)
    all {
      canCommit(forgedFundingVerifiedA, FundingOneAttempt),
      not(canRejectVerified(forgedFundingVerifiedA, FundingOneAttempt)),
      not(canCommitAuthorityA(forgedFundingVerifiedA, FundingOneAttempt)),
      canRejectVerifiedAuthorityA(forgedFundingVerifiedA, FundingOneAttempt),
      rejected == expectedRejectionA(forgedFundingVerifiedA, forgedFundingAttemptA,
        forgedFundingEvidenceA, UnauthorizedEffect, CommitBoundary),
      not(canRejectVerifiedAuthorityA(rejected, FundingOneAttempt)),
      applyRejectVerifiedAuthorityA(rejected, FundingOneAttempt) == rejected,
    }
  }
  run forgedCoreErrorIsUnauthorizedTest = {
    match fundingProjectionA {
      | CoreProjected(raw) => {
          val attempt = {...fundingAttemptA(SignBeforeResolve), observation: {...fundingExpectedA,
            coreProjection: CoreProjected({...raw, accepted: false, error: CoreErrorCode("contract_closed")}),
            outcome: Rejected(CoreRejected(CoreErrorCode("contract_closed")))}}
          val evidence = fundingEvidenceForA(attempt, SignBeforeResolve)
          val proposed = replaceProposedA(fundingProposedA(SignBeforeResolve), attempt)
          canRejectProposedAuthorityA(proposed, FundingOneAttempt, evidence)
            and applyRejectProposedAuthorityA(proposed, FundingOneAttempt, evidence)
              == expectedRejectionA(proposed, attempt, evidence, UnauthorizedEffect, VerificationBoundary)
        }
      | _ => false
    }
  }
  run faithfulMissingEvidenceClassificationTest = {
    val profile = SignBeforeResolve
    val proposed = fundingProposedA(profile)
    val attempt = fundingAttemptA(profile)
    val evidence = fundingEvidenceA(profile)
    val noEffect = {...evidence, effect: {...evidence.effect, disposition: EvidenceUnavailable}}
    val noSignature = {...evidence, signatures: Map()}
    List(noEffect, noSignature).foldl(true, (ok, missing) => ok
      and not(canVerifyAuthorityA(proposed, FundingOneAttempt, missing))
      and canRejectProposedAuthorityA(proposed, FundingOneAttempt, missing)
      and applyRejectProposedAuthorityA(proposed, FundingOneAttempt, missing)
        == expectedRejectionA(proposed, attempt, missing, EvidenceMissing, VerificationBoundary))
  }
  run faithfulStaleVerifiedClassificationTest = {
    val verified = fundingVerifiedA(SignBeforeResolve)
    val stale = {...verified, authority: {...verified.authority, context: {...verified.authority.context,
      environment: {...verified.authority.context.environment, anchor: 1}}}}
    validAuthorityExecutionA(stale) and not(canCommitAuthorityA(stale, FundingOneAttempt))
      and canRejectVerifiedAuthorityA(stale, FundingOneAttempt)
      and applyRejectVerifiedAuthorityA(stale, FundingOneAttempt)
        == expectedRejectionA(stale, fundingAttemptA(SignBeforeResolve), fundingEvidenceA(SignBeforeResolve),
          StaleBindings, CommitBoundary)
  }
  run uncoupledStateStillRejectableTest = {
    val proposed = fundingProposedA(SignBeforeResolve)
    val context = proposed.authority.context
    val badLedger = {...context, ledger: context.ledger.put((Escrow(aliceA), TokenA), 5)}
    val badAccounts = {...context, candidate: {...context.candidate,
      state: {...context.candidate.state, accounts: context.candidate.state.accounts.put(aliceA, 5)}}}
    val absentAccounts = {...context, candidate: {...context.candidate,
      state: {...context.candidate.state, accounts: Map()}}}
    List(badLedger, badAccounts, absentAccounts).foldl(true, (ok, bad) => {
      val state = {...proposed, authority: {...proposed.authority, context: bad}}
      ok and not(validAuthorityExecutionA(state))
        and not(canVerifyAuthorityA(state, FundingOneAttempt, fundingEvidenceA(SignBeforeResolve)))
        and canRejectProposedAuthorityA(state, FundingOneAttempt, fundingEvidenceA(SignBeforeResolve))
        and applyRejectProposedAuthorityA(state, FundingOneAttempt, fundingEvidenceA(SignBeforeResolve))
          == expectedRejectionA(state, fundingAttemptA(SignBeforeResolve), fundingEvidenceA(SignBeforeResolve),
            UnauthorizedEffect, VerificationBoundary)
    })
  }
  run malformedStructuralMapsRefuseSafelyTest = {
    val proposed = forgedFundingProposedA
    val verified = forgedFundingVerifiedA
    val noAttempts = {...proposed, attempts: Map()}
    val noSigning = {...proposed, authority: {...proposed.authority, signing: Map()}}
    val noRegistry = {...proposed, authority: {...proposed.authority,
      context: {...proposed.authority.context, registry: Map()}}}
    val noParents = {...proposed, authority: {...proposed.authority,
      context: {...proposed.authority.context, parents: Map()}}}
    List(noAttempts, noSigning, noRegistry, noParents).foldl(true, (ok, bad) => ok
      and not(authorityRejectionDomainA(bad))
      and not(canVerifyAuthorityA(bad, FundingOneAttempt, forgedFundingEvidenceA))
      and not(canCommitAuthorityA(bad, FundingOneAttempt))
      and not(canRejectProposedAuthorityA(bad, FundingOneAttempt, forgedFundingEvidenceA))
      and not(canRejectVerifiedAuthorityA(bad, FundingOneAttempt))
      and applyRejectProposedAuthorityA(bad, FundingOneAttempt, forgedFundingEvidenceA) == bad
      and applyRejectVerifiedAuthorityA(bad, FundingOneAttempt) == bad)
      and not(canRejectProposedAuthorityA(verified, FundingOneAttempt, forgedFundingEvidenceA))
      and not(canRejectVerifiedAuthorityA(proposed, FundingOneAttempt))
      and not(canRejectProposedAuthorityA(fundingUnsignedA, FundingOneAttempt, forgedFundingEvidenceA))
      and not(canRejectVerifiedAuthorityA(fundingUnsignedA, FundingOneAttempt))
  }
  run retainedWrongIdRefusesRejectionTest = {
    val wrong = {...forgedFundingAttemptA, id: FundingTwoAttempt}
    val evidence = fundingEvidenceForA(wrong, SignBeforeResolve)
    val proposed = replaceProposedA(forgedFundingProposedA, wrong)
    val verified = replaceVerifiedA(forgedFundingVerifiedA, wrong, evidence)
    not(canVerifyAuthorityA(proposed, FundingOneAttempt, evidence))
      and not(canRejectProposedAuthorityA(proposed, FundingOneAttempt, evidence))
      and applyRejectProposedAuthorityA(proposed, FundingOneAttempt, evidence) == proposed
      and not(canCommitAuthorityA(verified, FundingOneAttempt))
      and not(canRejectVerifiedAuthorityA(verified, FundingOneAttempt))
      and applyRejectVerifiedAuthorityA(verified, FundingOneAttempt) == verified
  }
  run resolvedPlanRecheckedDuringExecutionTest = {
    val empty = {...fundingPlanA, operations: List()}
    val secondBad = planWithSecondA({...fundingPlannedA, proposedSuccessor: fundingBeforeA})
    List(empty, secondBad).foldl(true, (ok, plan) => {
      val attempt = {...fundingAttemptA(SignBeforeResolve), observation: {...fundingExpectedA, resolvedPlan: plan}}
      val evidence = fundingEvidenceForA(attempt, SignBeforeResolve)
      val proposed = replaceProposedA(fundingProposedA(SignBeforeResolve), attempt)
      val verified = replaceVerifiedA(fundingVerifiedA(SignBeforeResolve), attempt, evidence)
      ok and authorityObservationMatchesA(OpFund, attempt.observation)
        and canVerify(proposed, FundingOneAttempt, evidence) and canCommit(verified, FundingOneAttempt)
        and not(canVerifyAuthorityA(proposed, FundingOneAttempt, evidence))
        and not(canCommitAuthorityA(verified, FundingOneAttempt))
        and canRejectProposedAuthorityA(proposed, FundingOneAttempt, evidence)
        and canRejectVerifiedAuthorityA(verified, FundingOneAttempt)
    })
  }

  run evidenceAfterViewFidelityTest = {
    val mismatch: AAuthorityPolicy = {...fundingPolicyA(SignAfterResolve),
      binding: AfterResolution({identity: fundingPlanA,
        operations: List(fundingPlannedA, {...fundingPlannedA, proposedSuccessor: fundingBeforeA})})}
    val prepared = applyPrepareSigning(fundingUnsignedA.authority, mismatch, Alice)
    val signed = applySign(prepared, mismatch, Alice, 0)
    val attempt = {...fundingAttemptA(SignAfterResolve), context: signed.context}
    val evidence: AAuthorityEvidence = {effect: {attempt: attempt, disposition: EvidenceValid},
      signatures: Map(fundingKeyA -> {attempt: attempt, disposition: EvidenceValid,
        signed: {policy: mismatch, signer: Alice, token: 0}})}
    val proposed = replaceProposedA({...fundingUnsignedA, authority: signed}, attempt)
    val verified = replaceVerifiedA(proposed, attempt, evidence)
    all {
      validPolicy(mismatch),
      canPrepareSigning(fundingUnsignedA.authority, mismatch, Alice),
      canSign(prepared, mismatch, Alice, 0),
      authorityAttemptMatchesA(attempt),
      canVerify(proposed, FundingOneAttempt, evidence),
      canCommit(verified, FundingOneAttempt),
      not(authorityEvidencePoliciesMatchA(evidence)),
      not(canVerifyAuthorityA(proposed, FundingOneAttempt, evidence)),
      not(canCommitAuthorityA(verified, FundingOneAttempt)),
      canRejectProposedAuthorityA(proposed, FundingOneAttempt, evidence),
      canRejectVerifiedAuthorityA(verified, FundingOneAttempt),
      applyRejectProposedAuthorityA(proposed, FundingOneAttempt, evidence)
        == expectedRejectionA(proposed, attempt, evidence, UnauthorizedEffect, VerificationBoundary),
      applyRejectVerifiedAuthorityA(verified, FundingOneAttempt)
        == expectedRejectionA(verified, attempt, evidence, UnauthorizedEffect, CommitBoundary),
    }
  }
  run genuineDeadlineCoreRejectionTest = {
    val request = {...fundingRequestA, now: Time100}
    val facts = {...fundingFactsA, environment: {...fundingFactsA.environment, physicalTime: 100}}
    val projection: CoreProjection[AContinuation] = CoreProjected({
      accepted: false, error: CoreErrorCode("contract_closed"), payments: List(), warnings: List(), reductions: 0,
      state: {accounts: CORE_ACCOUNTS.mapBy(_ => 0),
        choices: Map("settle" -> NoInt, "fill1" -> NoInt, "fill2" -> NoInt, "recover" -> NoInt, "other" -> NoInt),
        continuation: {program: canonicalSwap, node: N6}, minimumTime: 1}})
    val planned: AAuthorityPlanned = {...fundingPlannedA, proposedSuccessor: fundingBeforeA,
      artifactAndCall: AgreementCallA(request), transactionTime: 100, effects: List(),
      coreProjection: projection, predecessorFacts: facts}
    val plan: AResolvedPlan = {identity: SwapPlanA, operations: List(planned)}
    val policy: AAuthorityPolicy = {...fundingPolicyA(SignAfterResolve),
      body: {...fundingBodyA, validUntil: 100, clauses: List({
        operation: OpFund, conditions: List(InputIs(fundingInputA), AtOrAfterTime(100)),
        requiredEffects: List(), allowedEffects: List(), effectOrder: ExactOrder})},
      binding: AfterResolution({identity: plan, operations: plan.operations})}
    val unsigned = {...fundingUnsignedA, authority: {...fundingUnsignedA.authority,
      context: {...fundingContextA, environment: facts.environment}}}
    match adaptAuthorityA(request, OpFund, plan, PublicDisplay) {
      | AuthorityAdaptedA(obs) => {
          val attempt: AAuthorityAttempt = {id: FundingOneAttempt, actor: Alice, operation: OpFund,
            observation: obs, context: unsigned.authority.context}
          // OpFund with no effects has NO required authority key. Do not invent one.
          val evidence: AAuthorityEvidence = {effect: {attempt: attempt, disposition: EvidenceValid}, signatures: Map()}
          val proposed = applyPropose(unsigned, FundingOneAttempt, OpFund, obs, Alice)
          all {
            obs.predecessor == fundingBeforeA, obs.proposedSuccessor == fundingBeforeA,
            obs.coreProjection == projection, obs.effects == List(),
            obs.outcome == Rejected(CoreRejected(CoreErrorCode("contract_closed"))),
            requiredKeys(OpFund, obs.effects) == Set(),
            authorityPlanMatchesA(plan), validAuthorityPolicyA(policy),
            canPrepareSigningAuthorityA(unsigned.authority, policy, Alice),
            canPropose(unsigned, FundingOneAttempt, OpFund, obs, Alice),
            authorityAttemptMatchesA(attempt), authorityEvidencePoliciesMatchA(evidence),
            not(canVerify(proposed, FundingOneAttempt, evidence)),
            not(canVerifyAuthorityA(proposed, FundingOneAttempt, evidence)),
            canRejectProposedAuthorityA(proposed, FundingOneAttempt, evidence),
            applyRejectProposedAuthorityA(proposed, FundingOneAttempt, evidence)
              == expectedRejectionA(proposed, attempt, evidence,
                CoreRejected(CoreErrorCode("contract_closed")), VerificationBoundary),
          }
        }
      | _ => false
    }
  }

  // Supplemental boundary assertions added without changing already implemented guards.
  run threeAndFourFaithfulPlanLengthsTest =
    authorityPlanMatchesA({...fundingPlanA, operations: List(fundingPlannedA, fundingPlannedA, fundingPlannedA)})
      and authorityPlanMatchesA({...fundingPlanA, operations: List(fundingPlannedA, fundingPlannedA, fundingPlannedA, fundingPlannedA)})
  run allSixEscrowsCoupledTest = CORE_ACCOUNTS.forall(account =>
    not(authorityPredecessorLedgerMatchesA(fundingBeforeA,
      fundingLedgerA.put((Escrow(account), account.asset), 1))))
  run malformedCallDiagnosticsFailPlanTest = {
    val badInput = {...fundingRequestA, input: PresentAInput(DepositInputA({
      account: aliceA, depositor: Alice, quantity: 3}))}
    val badProgram = {...fundingRequestA, before: {...fundingBeforeA, program: {...canonicalSwap, nodes: Map()}}}
    List(badInput, badProgram).foldl(true, (ok, request) => ok
      and adaptAuthorityA(request, OpFund, fundingPlanA, PublicDisplay)
        == AuthorityComputationDiagnosticA(OutsideModelDomainA)
      and not(authorityPlannedOperationMatchesA({...fundingPlannedA,
        artifactAndCall: AgreementCallA(request)}, fundingPlanA)))
  }
  run verifiedStructuralMapsRefuseSafelyTest = {
    val verified = forgedFundingVerifiedA
    val noAttempts = {...verified, attempts: Map()}
    val noSigning = {...verified, authority: {...verified.authority, signing: Map()}}
    val noRegistry = {...verified, authority: {...verified.authority,
      context: {...verified.authority.context, registry: Map()}}}
    val noParents = {...verified, authority: {...verified.authority,
      context: {...verified.authority.context, parents: Map()}}}
    List(noAttempts, noSigning, noRegistry, noParents).foldl(true, (ok, bad) => ok
      and not(authorityRejectionDomainA(bad))
      and not(canCommitAuthorityA(bad, FundingOneAttempt))
      and not(canRejectVerifiedAuthorityA(bad, FundingOneAttempt))
      and applyRejectVerifiedAuthorityA(bad, FundingOneAttempt) == bad)
  }

  run prepareFundingActionTest = initProfileA(SignAfterResolve).then(prepareFundingA)
    .expect(fundingPreparedWitnessA and boundarySafetyA)
  // These starts are constructed from previously guard-asserted pure pipelines;
  // the action under test still performs its own real update.
  run signFundingActionTest = all {
    signingPipelineChecksA(SignAfterResolve),
    boundaryProfileA' = SignAfterResolve, boundaryExecutionA' = fundingPreparedA(SignAfterResolve),
  }.then(signFundingA).expect(fundingSignedWitnessA and boundarySafetyA)
  run proposeFundingActionTest = all {
    fundingPipelineChecksA(SignBeforeResolve),
    boundaryProfileA' = SignBeforeResolve, boundaryExecutionA' = fundingSignedA(SignBeforeResolve),
  }.then(proposeFundingA).expect(fundingProposedWitnessA and boundarySafetyA)
  run verifyFundingActionTest = all {
    fundingPipelineChecksA(SignBeforeResolve),
    boundaryProfileA' = SignBeforeResolve, boundaryExecutionA' = fundingProposedA(SignBeforeResolve),
  }.then(verifyFundingA).expect(fundingVerifiedWitnessA and boundarySafetyA)
  run commitFundingActionTest = all {
    fundingPipelineChecksA(SignAfterResolve),
    boundaryProfileA' = SignAfterResolve, boundaryExecutionA' = fundingVerifiedA(SignAfterResolve),
  }.then(commitFundingA).expect(fundingCommittedWitnessA and boundarySafetyA)
  run fullAfterFundingActionsTest = initProfileA(SignAfterResolve)
    .then(prepareFundingA).then(signFundingA).then(proposeFundingA).then(verifyFundingA).then(commitFundingA)
    .expect(fundingCommittedWitnessA and boundarySafetyA and boundaryProfileA == SignAfterResolve)
  run fullBeforeFundingActionsTest = initProfileA(SignBeforeResolve)
    .then(prepareFundingA).then(signFundingA).then(proposeFundingA).then(verifyFundingA).then(commitFundingA)
    .expect(fundingCommittedWitnessA and boundarySafetyA and boundaryProfileA == SignBeforeResolve)

  // Supplemental action witnesses for the already RED/GREEN-tested rejection updates.
  // Initial tampering is constructed, not a claimed reachable honest funding action.
  run proposedRejectionActionWitnessTest = all {
    fundingPipelineChecksA(SignBeforeResolve),
    boundaryProfileA' = SignBeforeResolve, boundaryExecutionA' = forgedFundingProposedA,
  }.then(all {
    canRejectProposedAuthorityA(boundaryExecutionA, FundingOneAttempt, forgedFundingEvidenceA),
    boundaryExecutionA' = applyRejectProposedAuthorityA(boundaryExecutionA, FundingOneAttempt, forgedFundingEvidenceA),
    boundaryProfileA' = boundaryProfileA,
  }).expect(boundaryExecutionA == expectedRejectionA(forgedFundingProposedA, forgedFundingAttemptA,
    forgedFundingEvidenceA, UnauthorizedEffect, VerificationBoundary) and boundarySafetyA)
  run verifiedRejectionActionWitnessTest = all {
    fundingPipelineChecksA(SignBeforeResolve),
    boundaryProfileA' = SignBeforeResolve, boundaryExecutionA' = forgedFundingVerifiedA,
  }.then(all {
    canRejectVerifiedAuthorityA(boundaryExecutionA, FundingOneAttempt),
    boundaryExecutionA' = applyRejectVerifiedAuthorityA(boundaryExecutionA, FundingOneAttempt),
    boundaryProfileA' = boundaryProfileA,
  }).expect(boundaryExecutionA == expectedRejectionA(forgedFundingVerifiedA, forgedFundingAttemptA,
    forgedFundingEvidenceA, UnauthorizedEffect, CommitBoundary) and boundarySafetyA)
}
