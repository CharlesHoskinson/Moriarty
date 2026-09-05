module candidate_a_funding_pilot {
  import effects.* from "../effects"
  import consumption.* from "../consumption"
  import observations.* from "../observations"
  import policies.* from "../policies"
  import authorization.* from "../authorization"
  import execution.* from "../execution"
  import candidate_a_types.* from "../candidate_a_types"
  import candidate_a_programs.* from "../candidate_a_programs"
  import candidate_a_authority_adapter.* from "../candidate_a_authority_adapter"
  import candidate_a_authority_swap_fixtures as F from "../candidate_a_authority_swap_fixtures"
  import candidate_a_authority_swap as S from "../candidate_a_authority_swap"
  import candidate_a_authority_boundary as B from "../candidate_a_authority_boundary"
  import candidate_a_core as C from "../candidate_a_core"
  type PilotState = {execution: AAuthorityExecution, profile: SigningProfile, forged: bool,
    cursor: int, history: List[AAuthorityExecution], raw: List[ATransactionEvaluation]}
  pure val scenario = {funded: 2, mode: F::SettleS}
  pure def initial(p: SigningProfile, bad: bool): PilotState = {
    execution: F::unsignedS, profile: p, forged: bad, cursor: 0, history: List(), raw: List()}
  pure def pilotLength(p: PilotState): int = if (p.forged) 4 else 5
  pure def command(i: int): S::CommandS =
    if (i == 0) S::PrepareS({id: FundingOneAttempt, principal: Alice})
    else if (i == 1) S::SignS({id: FundingOneAttempt, principal: Alice})
    else if (i == 3) S::VerifyS(FundingOneAttempt) else S::CommitS(FundingOneAttempt)
  pure def observation(obs: AAuthorityObservation, bad: bool): AAuthorityObservation =
    if (not(bad)) obs else {...obs, proposedSuccessor: {...obs.proposedSuccessor,
      program: {...obs.proposedSuccessor.program, nodes: obs.proposedSuccessor.program.nodes.put(N15,
        PayA({account: aliceA, payee: Mallory, amount: ConstantA(1), continuation: N0}))}}}
  pure def ready(p: PilotState): bool =
    if (p.cursor < 0 or p.cursor >= pilotLength(p)) false
    else if (p.cursor == 2) match F::actualS(FundingOneAttempt,scenario) {
      | AuthorityAdaptedA(obs) => canPropose(p.execution,FundingOneAttempt,OpFund,observation(obs,p.forged),Alice)
      | _ => false }
    else if (p.cursor == 3 and p.forged) match p.execution.attempts.get(FundingOneAttempt) {
      | ProposedAttempt(a) => B::canRejectProposedAuthorityA(p.execution,FundingOneAttempt,S::evidenceS(a,scenario,p.profile))
      | _ => false }
    else S::canCommandS(p.execution,scenario,p.profile,command(p.cursor))
  pure def nextExecution(p: PilotState): AAuthorityExecution =
    if (p.cursor == 2) match F::actualS(FundingOneAttempt,scenario) {
      | AuthorityAdaptedA(obs) => applyPropose(p.execution,FundingOneAttempt,OpFund,observation(obs,p.forged),Alice)
      | _ => p.execution }
    else if (p.cursor == 3 and p.forged) match p.execution.attempts.get(FundingOneAttempt) {
      | ProposedAttempt(a) => B::applyRejectProposedAuthorityA(p.execution,FundingOneAttempt,S::evidenceS(a,scenario,p.profile))
      | _ => p.execution }
    else S::applyCommandS(p.execution,scenario,p.profile,command(p.cursor))
  pure def advance(p: PilotState): PilotState = {
    val after = nextExecution(p)
    val raw = if (p.cursor != 2) p.raw else p.raw.append(C::computeTransaction(F::programS,F::beforeS(0).state,
      PresentAInput(DepositInputA({account: aliceA, depositor: Alice, quantity: 10})),Time1))
    {...p, execution: after, cursor: p.cursor + 1, history: p.history.append(after), raw: raw}
  }
  pure def terminalExact(p: PilotState): bool =
    if (p.cursor != pilotLength(p)) true
    else if (p.forged) match p.execution.attempts.get(FundingOneAttempt) {
      | RejectedOperation(r) => {
          val registered = p.history.nth(1)
          p.execution == {...registered, attempts: registered.attempts.put(FundingOneAttempt,
            RejectedOperation({attempt: r.attempt, evidence: S::evidenceS(r.attempt,scenario,p.profile),
              observedContext: registered.authority.context, reason: UnauthorizedEffect, stage: VerificationBoundary}))}
            and r.attempt == {id: FundingOneAttempt, actor: Alice, operation: OpFund,
              observation: observation(F::expectedS(FundingOneAttempt,scenario),true), context: registered.authority.context}
            and not(B::canVerifyAuthorityA(p.history.nth(2),FundingOneAttempt,r.evidence))
        }
      | _ => false }
    else p.execution.authority.context.candidate == F::beforeS(1)
      and p.execution.authority.context.ledger == F::ledgerS(1)
      and p.execution.authority.context.registry.get(F::keyS(Alice,0)) ==
        AuthorityConsumed({signed: {policy: F::policyS(FundingOneAttempt,scenario,Alice,p.profile), signer: Alice, token: 0}, revision: 1})
      and match p.execution.attempts.get(FundingOneAttempt) { | ExecutedOperation(_) => true | _ => false }
  pure def safety(p: PilotState): bool = S::safetyS(p.execution)
    and p.cursor == p.history.length() and p.cursor >= 0 and p.cursor <= pilotLength(p)
    and (if (p.cursor < 3) p.raw == List() else p.raw == List(TransactionComputedA({
      accepted: true, state: F::beforeS(1).state, error: NoCoreError, payments: List(), warnings: List(), reductions: 0})))
    and (if (p.cursor == pilotLength(p)) not(ready(p)) else ready(p))
    and terminalExact(p)
    and (if (p.cursor == 0) p.execution == F::unsignedS else p.execution == p.history.nth(p.cursor - 1))
    and (if (p.cursor < 5) p.execution.authority.context.candidate == F::beforeS(0)
      and p.execution.authority.context.ledger == F::ledgerS(0) else true)
  var pilot: PilotState
  action init = {
    nondet profile = F::PROFILES_S.oneOf()
    nondet forged = Set(false,true).oneOf()
    pilot' = initial(profile,forged)
  }
  action step = all { ready(pilot), pilot' = advance(pilot) }
  val pilotSafety = safety(pilot)
  val neverPrepared = pilot.cursor == 0
  val prepared = pilot.cursor >= 1
  val signed = pilot.cursor >= 2
  val proposed = pilot.cursor >= 3
  val verified = not(pilot.forged) and pilot.cursor >= 4
  val committed = not(pilot.forged) and pilot.cursor == 5
  val rejected = pilot.forged and pilot.cursor == 4
  val afterCompleted = pilot.cursor == pilotLength(pilot) and pilot.profile == SignAfterResolve
  val beforeCompleted = pilot.cursor == pilotLength(pilot) and pilot.profile == SignBeforeResolve
}
