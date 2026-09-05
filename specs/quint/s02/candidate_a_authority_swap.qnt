module candidate_a_authority_swap {
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

  type CommandS = PrepareS({id: AttemptId, principal: Principal}) | SignS({id: AttemptId, principal: Principal})
    | ProposeS(AttemptId) | VerifyS(AttemptId) | CommitS(AttemptId) | AdvanceS(ATime)
    | RejectProposedS(AttemptId) | RejectVerifiedS(AttemptId)
  pure def preparePairS(id: AttemptId, principal: Principal): List[CommandS] =
    List(PrepareS({id: id, principal: principal}), SignS({id: id, principal: principal}))
  pure def pipelineS(id: AttemptId): List[CommandS] = List(ProposeS(id), VerifyS(id), CommitS(id))
  pure def fundingRouteS(s: ScenarioS): List[CommandS] =
    (if (s.funded >= 1) preparePairS(FundingOneAttempt, Alice).concat(pipelineS(FundingOneAttempt)) else List())
    .concat(if (s.funded == 2) List(AdvanceS(Time2)).concat(preparePairS(FundingTwoAttempt, Bob)).concat(pipelineS(FundingTwoAttempt)) else List())
  pure def routeS(s: ScenarioS): List[CommandS] = {
    val clock = if (Set(SettleS, RefundS).contains(s.mode)) List() else List(AdvanceS(timeS(s.mode)))
    val signing = if (rejectedS(s.mode) or s.funded == 0) List()
      else preparePairS(DispositionAttempt, Alice).concat(if (s.funded == 2) preparePairS(DispositionAttempt, Bob) else List())
    val disposition = if (rejectedS(s.mode) or s.funded == 0)
      List(ProposeS(DispositionAttempt), RejectProposedS(DispositionAttempt)) else pipelineS(DispositionAttempt)
    fundingRouteS(s).concat(clock).concat(signing).concat(disposition)
  }
  pure def evidenceS(attempt: AAuthorityAttempt, s: ScenarioS, p: SigningProfile): AAuthorityEvidence = {
    effect: {attempt: attempt, disposition: EvidenceValid},
    signatures: expectedKeysS(attempt.id, s).mapBy(key => {attempt: attempt, disposition: EvidenceValid,
      signed: {policy: policyS(attempt.id, s, key.principal, p), signer: key.principal, token: 0}})}
  pure def canCommandS(state: AAuthorityExecution, s: ScenarioS, p: SigningProfile, cmd: CommandS): bool = match cmd {
    | PrepareS(c) => canPrepareSigningAuthorityA(state.authority, policyS(c.id, s, c.principal, p), c.principal)
    | SignS(c) => canSignAuthorityA(state.authority, policyS(c.id, s, c.principal, p), c.principal, 0)
    | AdvanceS(t) => validAuthorityExecutionA(state) and timeValue(t) > state.authority.context.environment.physicalTime
    | ProposeS(id) => match actualS(id, s) {
        | AuthorityAdaptedA(obs) => validAuthorityExecutionA(state) and canPropose(state, id, plannedS(id, s).operation, obs,
            if (id == FundingOneAttempt) Alice else Bob)
        | _ => false }
    | VerifyS(id) => match state.attempts.get(id) {
        | ProposedAttempt(a) => canVerifyAuthorityA(state, id, evidenceS(a, s, p)) | _ => false }
    | CommitS(id) => canCommitAuthorityA(state, id)
    | RejectProposedS(id) => match state.attempts.get(id) {
        | ProposedAttempt(a) => canRejectProposedAuthorityA(state, id, evidenceS(a, s, p)) | _ => false }
    | RejectVerifiedS(id) => canRejectVerifiedAuthorityA(state, id)
  }
  pure def applyCommandS(state: AAuthorityExecution, s: ScenarioS, p: SigningProfile, cmd: CommandS): AAuthorityExecution = match cmd {
    | PrepareS(c) => {...state, authority: applyPrepareSigning(state.authority, policyS(c.id, s, c.principal, p), c.principal)}
    | SignS(c) => {...state, authority: applySign(state.authority, policyS(c.id, s, c.principal, p), c.principal, 0)}
    | AdvanceS(t) => {...state, authority: {...state.authority, context: {...state.authority.context, environment: environmentS(timeValue(t))}}}
    | ProposeS(id) => match actualS(id, s) {
        | AuthorityAdaptedA(obs) => applyPropose(state, id, plannedS(id, s).operation, obs, if (id == FundingOneAttempt) Alice else Bob)
        | _ => state }
    | VerifyS(id) => match state.attempts.get(id) {
        | ProposedAttempt(a) => applyVerify(state, id, evidenceS(a, s, p)) | _ => state }
    | CommitS(id) => applyCommit(state, id)
    | RejectProposedS(id) => match state.attempts.get(id) {
        | ProposedAttempt(a) => applyRejectProposedAuthorityA(state, id, evidenceS(a, s, p)) | _ => state }
    | RejectVerifiedS(id) => applyRejectVerifiedAuthorityA(state, id)
  }
  pure def noPendingS(state: AAuthorityExecution): bool = ATTEMPT_IDS.forall(id => match state.attempts.get(id) {
    | ProposedAttempt(_) => false | VerifiedOperation(_) => false | _ => true })
    and AUTHORITY_KEYS.forall(key => match state.authority.signing.get(key) { | PreparedSigning(_) => false | _ => true })
  pure def financialTerminalS(state: AAuthorityExecution): bool = noPendingS(state)
    and state.authority.context.candidate.state.continuation == N0
    and CORE_ACCOUNTS.forall(a => balance(state.authority.context.ledger, Escrow(a), a.asset) == 0)
  pure def safetyS(state: AAuthorityExecution): bool = validAuthorityExecutionA(state)
    and totalAsset(state.authority.context.ledger, TokenA) == 10 and totalAsset(state.authority.context.ledger, TokenB) == 20
    and state.authority.context.parents == unsignedS.authority.context.parents
  pure def nonCommitS(before: AAuthorityExecution, after: AAuthorityExecution, cmd: CommandS): bool = match cmd {
    | CommitS(id) => after == applyCommit(before, id)
    | AdvanceS(_) => after.authority.context.ledger == before.authority.context.ledger
        and after.authority.context.candidate == before.authority.context.candidate and after.attempts == before.attempts
        and after.authority.context.registry == before.authority.context.registry
    | SignS(_) => after.authority.context.candidate == before.authority.context.candidate
        and after.authority.context.ledger == before.authority.context.ledger and after.attempts == before.attempts
    | _ => after.authority.context == before.authority.context }
  type PrefixS = {state: AAuthorityExecution, ok: bool, visited: List[CommandS]}
  pure def executeListS(s: ScenarioS, p: SigningProfile, commands: List[CommandS]): PrefixS =
    commands.foldl({state: unsignedS, ok: safetyS(unsignedS), visited: List()}, (acc, cmd) => {
      val commandReady = canCommandS(acc.state, s, p, cmd)
      val after = if (commandReady) applyCommandS(acc.state, s, p, cmd) else acc.state
      {state: after, ok: acc.ok and commandReady and safetyS(after) and nonCommitS(acc.state, after, cmd),
        visited: if (commandReady) acc.visited.append(cmd) else acc.visited}
    })
  pure def prefixS(s: ScenarioS, p: SigningProfile, count: int): PrefixS = executeListS(s, p, routeS(s).slice(0, count))
  pure def expectedFinalLedgerS(s: ScenarioS): Ledger = if (s.mode == SettleS)
    LEDGER_KEYS.mapBy(_ => 0).put((Wallet(Bob), TokenA), 10).put((Wallet(Alice), TokenB), 20)
    else if (rejectedS(s.mode) or s.funded == 0) ledgerS(s.funded) else ledgerS(0)
  pure val staleScenarioS: ScenarioS = {funded: 2, mode: SettleS}
  pure val staleRouteS: List[CommandS] = fundingRouteS(staleScenarioS)
    .concat(preparePairS(DispositionAttempt, Alice)).concat(preparePairS(DispositionAttempt, Bob))
    .concat(List(ProposeS(DispositionAttempt), VerifyS(DispositionAttempt), AdvanceS(Time100), RejectVerifiedS(DispositionAttempt)))
}
