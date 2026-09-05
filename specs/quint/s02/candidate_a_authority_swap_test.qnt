module candidate_a_authority_swap_test {
  import effects.* from "./effects"
  import consumption.* from "./consumption"
  import observations.* from "./observations"
  import policies.* from "./policies"
  import authorization.* from "./authorization"
  import execution.* from "./execution"
  import candidate_a_types.* from "./candidate_a_types"
  import candidate_a_programs.* from "./candidate_a_programs"
  import candidate_a_core.* from "./candidate_a_core"
  import candidate_a_projection.* from "./candidate_a_projection"
  import candidate_a_authority_adapter.* from "./candidate_a_authority_adapter"
  import candidate_a_authority_boundary.* from "./candidate_a_authority_boundary"
  import candidate_a_authority_swap_fixtures.* from "./candidate_a_authority_swap_fixtures"
  import candidate_a_authority_swap.* from "./candidate_a_authority_swap"

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
}
