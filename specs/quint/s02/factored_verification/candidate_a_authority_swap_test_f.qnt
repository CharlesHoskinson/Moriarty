module candidate_a_authority_swap_test_f {
  import effects.* from "../effects"
  import consumption.* from "../consumption"
  import observations.* from "../observations"
  import policies.* from "../policies"
  import authorization.* from "../authorization"
  import execution.* from "../execution"
  import candidate_a_types.* from "../candidate_a_types"
  import candidate_a_programs.* from "../candidate_a_programs"
  import candidate_a_core_f.* from "./candidate_a_core_f"
  import candidate_a_projection_f.* from "./candidate_a_projection_f"
  import candidate_a_authority_adapter_f.* from "./candidate_a_authority_adapter_f"
  import candidate_a_authority_boundary_f.* from "./candidate_a_authority_boundary_f"
  import candidate_a_authority_swap_fixtures_f.* from "./candidate_a_authority_swap_fixtures_f"
  import candidate_a_authority_swap_f.* from "./candidate_a_authority_swap_f"
  import candidate_a_authority_swap_harness_f as H from "./candidate_a_authority_swap_harness_f"

  run literalFidelityTest = assert(programS == canonicalSwap and beforeS(0).state.minimumTime == Time0
    and unsignedS.authority.context.environment.physicalTime == 1 and SCENARIOS_S.forall(s =>
      Set(FundingOneAttempt, FundingTwoAttempt, DispositionAttempt).forall(id =>
        actualS(id, s) == AuthorityAdaptedA(expectedS(id, s))
        and authorityPlanMatchesA(planS(id, s))
        and requiredKeys(plannedS(id, s).operation, plannedS(id, s).effects) == expectedKeysS(id, s))))
  run authorityTimeZeroTest = assert(adaptAuthorityA({before: beforeS(0), input: NoAInput, now: Time0},
    OpDeadlineRefund, planS(DispositionAttempt, {funded: 0, mode: TimeoutS(Time100)}), PublicDisplay) == AuthorityInvalidRequestA
    and validState(programS, beforeS(0).state))

  run fundingTest = assert(PROFILES_S.forall(p => {
    val s = {funded: 2, mode: SettleS}
    val signed = prefixS(s, p, 2)
    val alice = prefixS(s, p, 5)
    val bob = prefixS(s, p, 11)
    signed.ok and alice.ok and bob.ok
      and signed.state.authority.context.candidate == beforeS(0) and signed.state.authority.context.ledger == ledgerS(0)
      and alice.state.authority.context.candidate == beforeS(1) and alice.state.authority.context.ledger == ledgerS(1)
      and bob.state.authority.context.candidate == beforeS(2) and bob.state.authority.context.ledger == ledgerS(2)
      and Set(Alice, Bob).forall(owner => bob.state.authority.context.registry.get(keyS(owner, 0)) ==
        AuthorityConsumed({signed: {policy: policyS(if (owner == Alice) FundingOneAttempt else FundingTwoAttempt, s, owner, p),
          signer: owner, token: 0}, revision: 1}))
  }))
  run dispositionMatrixTest = assert(PROFILES_S.forall(p => SCENARIOS_S.forall(s => {
    val result = prefixS(s, p, routeS(s).length())
    val registered = prefixS(s, p, routeS(s).length() - (if (rejectedS(s.mode) or s.funded == 0) 2 else 3))
    val planned = dispositionPlannedS(s)
    result.ok and registered.ok and result.state.authority.context.ledger == expectedFinalLedgerS(s)
      and match result.state.attempts.get(DispositionAttempt) {
        | ExecutedOperation(done) => s.funded > 0 and not(rejectedS(s.mode))
            and financialTerminalS(result.state) and done.attempt.observation == expectedS(DispositionAttempt, s)
            and result.state.authority.context.candidate == planned.proposedSuccessor
            and done.evidence.signatures.keys() == expectedKeysS(DispositionAttempt, s)
            and expectedKeysS(DispositionAttempt, s).forall(key => result.state.authority.context.registry.get(key)
              == AuthorityConsumed({signed: done.evidence.signatures.get(key).signed, revision: 1}))
        | RejectedOperation(r) => {
            val attempt: AAuthorityAttempt = {id: DispositionAttempt, actor: Bob, operation: planned.operation,
              observation: expectedS(DispositionAttempt, s), context: registered.state.authority.context}
            val evidence: AAuthorityEvidence = {effect: {attempt: attempt, disposition: EvidenceValid}, signatures: Map()}
            not(financialTerminalS(result.state)) and noPendingS(result.state)
              and result.state.authority.context.candidate == beforeS(s.funded)
              and result.state == {...registered.state, attempts: registered.state.attempts.put(DispositionAttempt, RejectedOperation({
                attempt: attempt, evidence: evidence, observedContext: registered.state.authority.context,
                reason: if (rejectedS(s.mode)) CoreRejected(CoreErrorCode("contract_closed")) else UnauthorizedEffect,
                stage: VerificationBoundary}))}
          }
        | _ => false }
  })))
  run replayTest = assert(PROFILES_S.forall(p => SCENARIOS_S.forall(s => {
    val result = prefixS(s, p, routeS(s).length())
    result.ok and not(canCommitAuthorityA(result.state, DispositionAttempt))
      and not(canPropose(result.state, DispositionAttempt, dispositionPlannedS(s).operation, expectedS(DispositionAttempt, s), Bob))
  })))
  pure def rejectionPreservedS(state: AAuthorityExecution, id: AttemptId, e: AAuthorityEvidence,
    expectedReason: RejectionReason): bool = canRejectProposedAuthorityA(state, id, e) and match state.attempts.get(id) {
      | ProposedAttempt(attempt) => applyRejectProposedAuthorityA(state, id, e) == {...state,
          attempts: state.attempts.put(id, RejectedOperation({attempt: attempt, evidence: e,
            observedContext: state.authority.context, reason: expectedReason, stage: VerificationBoundary}))}
      | _ => false }
  pure def changedAttemptS(state: AAuthorityExecution, original: AAuthorityAttempt, changed: AAuthorityObservation): AAuthorityExecution =
    {...state, attempts: state.attempts.put(original.id, ProposedAttempt({...original, observation: changed}))}
  run staleSigningTest = assert(PROFILES_S.forall(p => {
    val s = staleScenarioS
    val funded = prefixS(s, p, 11)
    val policy = policyS(DispositionAttempt, s, Alice, p)
    val prepared = applyPrepareSigning(funded.state.authority, policy, Alice)
    val advanced = {...prepared, context: {...prepared.context, environment: environmentS(100)}}
    funded.ok and canPrepareSigningAuthorityA(funded.state.authority, policy, Alice)
      and canSignAuthorityA(prepared, policy, Alice, 0)
      and not(canSignAuthorityA(advanced, policy, Alice, 0))
      and advanced.signing == prepared.signing and advanced.context.candidate == prepared.context.candidate
      and advanced.context.ledger == prepared.context.ledger and advanced.context.registry == prepared.context.registry
  }))
  run staleVerifiedTest = assert(PROFILES_S.forall(p => {
    val ready = executeListS(staleScenarioS, p, staleRouteS.slice(0, 17))
    val advanced = executeListS(staleScenarioS, p, staleRouteS.slice(0, 18))
    val rejected = executeListS(staleScenarioS, p, staleRouteS)
    ready.ok and advanced.ok and rejected.ok and not(financialTerminalS(rejected.state))
      and canCommitAuthorityA(ready.state, DispositionAttempt)
      and not(canCommitAuthorityA(advanced.state, DispositionAttempt))
      and advanced.state.attempts == ready.state.attempts and rejected.state.authority == advanced.state.authority
      and match ready.state.attempts.get(DispositionAttempt) {
        | VerifiedOperation(v) => rejected.state.attempts.get(DispositionAttempt) == RejectedOperation({
            attempt: v.attempt, evidence: v.evidence, observedContext: advanced.state.authority.context,
            reason: StaleBindings, stage: CommitBoundary})
        | _ => false }
  }))
  pure def mutationsS(obs: AAuthorityObservation): Set[AAuthorityObservation] = {
    val changedProgram = {...programS, nodes: programS.nodes.put(N15,
      PayA({account: aliceA, payee: Bob, amount: ConstantA(5), continuation: N0}))}
    val wrongResult = match obs.coreProjection {
      | CoreProjected(r) => CoreProjected({...r, reductions: r.reductions + 1})
      | NoCoreProjection => NoCoreProjection }
    Set({...obs, proposedSuccessor: {...obs.proposedSuccessor, program: changedProgram}},
      {...obs, effects: List(obs.effects.nth(1), obs.effects.nth(0))},
      {...obs, coreProjection: wrongResult},
      {...obs, input: ChoiceLike({id: "settle", chooser: Alice, chosen: 1})})
  }
  run mutationTest = assert(PROFILES_S.forall(p => {
    val s = staleScenarioS
    val proposed = prefixS(s, p, 16)
    proposed.ok and match proposed.state.attempts.get(DispositionAttempt) {
      | ProposedAttempt(a) => mutationsS(a.observation).forall(obs => {
          val changed = {...a, observation: obs}
          val bad = changedAttemptS(proposed.state, a, obs)
          val e = evidenceS(changed, s, p)
          val verified = {...bad, attempts: bad.attempts.put(DispositionAttempt, VerifiedOperation({attempt: changed, evidence: e}))}
          not(canCommandS(bad, s, p, VerifyS(DispositionAttempt)))
            and not(canCommitAuthorityA(verified, DispositionAttempt))
            and authorityRejectionReasonA(bad, changed, e) == UnauthorizedEffect
            and rejectionPreservedS(bad, DispositionAttempt, e, UnauthorizedEffect)
        })
      | _ => false }
  }))
  run identityBindingTest = assert(PROFILES_S.forall(p => {
    val s = staleScenarioS
    val original = fundingPlannedS(Alice)
    val second = {...original, proposedSuccessor: beforeS(0)}
    val plan = {identity: SwapPlanA, operations: List(original, second)}
    val policy = policyS(FundingOneAttempt, s, Alice, p)
    val withPlan = {...policy, binding: if (p == SignAfterResolve) AfterResolution({identity: plan, operations: plan.operations})
      else BeforeResolution(AnyArtifactUnderMechanism)}
    val obs = {...expectedS(FundingOneAttempt, s), resolvedPlan: plan}
    val signed = {...unsignedS, authority: applySign(applyPrepareSigning(unsignedS.authority, policy, Alice), policy, Alice, 0)}
    val proposed = applyPropose(signed, FundingOneAttempt, OpFund, obs, Alice)
    not(authorityPlanMatchesA(plan))
      and (if (p == SignAfterResolve) not(canPrepareSigningAuthorityA(unsignedS.authority, withPlan, Alice))
        else canPrepareSigningAuthorityA(unsignedS.authority, withPlan, Alice))
      and canPrepareSigningAuthorityA(unsignedS.authority, policy, Alice)
      and canSignAuthorityA(applyPrepareSigning(unsignedS.authority, policy, Alice), policy, Alice, 0)
      and canPropose(signed, FundingOneAttempt, OpFund, obs, Alice)
      and match proposed.attempts.get(FundingOneAttempt) {
        | ProposedAttempt(a) => not(canVerifyAuthorityA(proposed, FundingOneAttempt, evidenceS(a, s, p)))
        | _ => false }
  }))
  run wrongActorTest = assert(PROFILES_S.forall(p => {
    val s = staleScenarioS
    val proposed = prefixS(s, p, 16)
    val wrongCore = computeTransaction(programS, beforeS(2).state,
      PresentAInput(ChoiceInputA({id: SettleId, chooser: Alice, chosen: 1})), Time2)
    proposed.ok and wrongCore == TransactionComputedA({accepted: false, state: beforeS(2).state,
      error: CoreErrorCode("no_matching_input"), payments: List(), warnings: List(), reductions: 0})
      and match proposed.state.attempts.get(DispositionAttempt) {
        | ProposedAttempt(a) => {
            val e = evidenceS(a, s, p)
            val aliceKey = keyS(Alice, 1)
            val wrongSigner = {...e, signatures: e.signatures.put(aliceKey, {...e.signatures.get(aliceKey),
              signed: {...e.signatures.get(aliceKey).signed, signer: Mallory}})}
            val wrongNonce = {...e, signatures: e.signatures.keys().exclude(Set(aliceKey)).mapBy(k => e.signatures.get(k))
              .put(keyS(Alice, 0), {...e.signatures.get(aliceKey), signed: {
                policy: policyS(FundingOneAttempt, s, Alice, p), signer: Alice, token: 0}})}
            not(canVerifyAuthorityA(proposed.state, DispositionAttempt, wrongSigner))
              and not(canVerifyAuthorityA(proposed.state, DispositionAttempt, wrongNonce))
              and rejectionPreservedS(proposed.state, DispositionAttempt, wrongSigner, UnauthorizedEffect)
              and rejectionPreservedS(proposed.state, DispositionAttempt, wrongNonce, EvidenceMissing)
          } | _ => false }
  }))

  run staleFactsTest = assert(PROFILES_S.forall(p => {
    val s = staleScenarioS
    val obs = expectedS(DispositionAttempt, s)
    val original = dispositionPlannedS(s)
    val changed = {...original, predecessorFacts: {...original.predecessorFacts, environment: environmentS(100)}}
    val plan = {identity: SwapPlanA, operations: List(changed)}
    val proposed = prefixS(s, p, 16)
    proposed.ok and not(authorityPlanMatchesA(plan)) and match proposed.state.attempts.get(DispositionAttempt) {
      | ProposedAttempt(a) => {
          val bad = {...a, observation: {...obs, resolvedPlan: plan}}
          val badState = {...proposed.state, attempts: proposed.state.attempts.put(DispositionAttempt, ProposedAttempt(bad))}
          not(canVerifyAuthorityA(badState, DispositionAttempt, evidenceS(bad, s, p)))
        } | _ => false }
  }))

  action initCaseS(s: ScenarioS, p: SigningProfile, isStale: bool): bool = all {
    H::state' = unsignedS, H::scenario' = s, H::profile' = p, H::stale' = isStale, H::cursor' = 0, H::history' = List(), H::lastAtomicS' = true }
  run settleActionTest = initCaseS(staleScenarioS, SignAfterResolve, false)
    .then(18.reps(_ => H::step)).expect(H::completedS and H::swapSafetyS and financialTerminalS(H::state))
  run refundActionTest = initCaseS({funded: 2, mode: RefundS}, SignBeforeResolve, false)
    .then(18.reps(_ => H::step)).expect(H::completedS and H::swapSafetyS and financialTerminalS(H::state))
  run staleActionTest = initCaseS(staleScenarioS, SignAfterResolve, true)
    .then(19.reps(_ => H::step)).expect(H::completedS and H::verifiedRejectedS and not(financialTerminalS(H::state)))
  run emptyTimeoutActionTest = initCaseS({funded: 0, mode: TimeoutS(Time100)}, SignBeforeResolve, false)
    .then(3.reps(_ => H::step)).expect(H::completedS and H::proposedRejectedS and not(financialTerminalS(H::state)))
  run initialWitnessesFalseTest = H::init.expect(not(H::completedS or H::alicePreparedS or H::proposedRejectedS))
  run boundsTest = assert(SCENARIOS_S.size() == 12 and SCENARIOS_S.forall(s => routeS(s).length() <= 19)
    and routeS({funded: 2, mode: TimeoutS(Time101)}).length() == 19 and staleRouteS.length() == 19)
}
