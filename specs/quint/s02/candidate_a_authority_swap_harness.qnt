module candidate_a_authority_swap_harness {
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

  var state: AAuthorityExecution
  var scenario: ScenarioS
  var profile: SigningProfile
  var stale: bool
  var cursor: int
  var history: List[CommandS]
  var lastAtomicS: bool
  val selectedRouteS: List[CommandS] = if (stale) staleRouteS else routeS(scenario)
  action init = {
    nondet selectedScenario = SCENARIOS_S.oneOf()
    nondet selectedProfile = PROFILES_S.oneOf()
    nondet selectedStale = Set(false, true).oneOf()
    all { state' = unsignedS, scenario' = if (selectedStale) staleScenarioS else selectedScenario,
      profile' = selectedProfile, stale' = selectedStale, cursor' = 0, history' = List(), lastAtomicS' = true }
  }
  action performS(cmd: CommandS): bool = all {
    cursor < selectedRouteS.length(), selectedRouteS.nth(cursor) == cmd,
    canCommandS(state, scenario, profile, cmd),
    state' = applyCommandS(state, scenario, profile, cmd), scenario' = scenario, profile' = profile, stale' = stale,
    cursor' = cursor + 1, history' = history.append(cmd),
    lastAtomicS' = nonCommitS(state, applyCommandS(state, scenario, profile, cmd), cmd),
  }
  action step = if (cursor < selectedRouteS.length()) performS(selectedRouteS.nth(cursor))
    else all { false, state' = state, scenario' = scenario, profile' = profile, stale' = stale, cursor' = cursor, history' = history, lastAtomicS' = lastAtomicS }
  def seenS(cmd: CommandS): bool = history.foldl(false, (found, actual) => found or actual == cmd)
  val swapSafetyS = safetyS(state) and lastAtomicS and cursor == history.length() and cursor <= selectedRouteS.length()
  val completedS = cursor == selectedRouteS.length() and cursor > 0
  val alicePreparedS = seenS(PrepareS({id: FundingOneAttempt, principal: Alice}))
  val aliceSignedS = seenS(SignS({id: FundingOneAttempt, principal: Alice}))
  val aliceProposedS = seenS(ProposeS(FundingOneAttempt))
  val aliceVerifiedS = seenS(VerifyS(FundingOneAttempt))
  val aliceCommittedS = seenS(CommitS(FundingOneAttempt))
  val bobPreparedS = seenS(PrepareS({id: FundingTwoAttempt, principal: Bob}))
  val bobSignedS = seenS(SignS({id: FundingTwoAttempt, principal: Bob}))
  val bobProposedS = seenS(ProposeS(FundingTwoAttempt))
  val bobVerifiedS = seenS(VerifyS(FundingTwoAttempt))
  val bobCommittedS = seenS(CommitS(FundingTwoAttempt))
  val dispositionAlicePreparedS = seenS(PrepareS({id: DispositionAttempt, principal: Alice}))
  val dispositionAliceSignedS = seenS(SignS({id: DispositionAttempt, principal: Alice}))
  val dispositionBobPreparedS = seenS(PrepareS({id: DispositionAttempt, principal: Bob}))
  val dispositionBobSignedS = seenS(SignS({id: DispositionAttempt, principal: Bob}))
  val dispositionProposedS = seenS(ProposeS(DispositionAttempt))
  val dispositionVerifiedS = seenS(VerifyS(DispositionAttempt))
  val dispositionCommittedS = seenS(CommitS(DispositionAttempt))
  val proposedRejectedS = seenS(RejectProposedS(DispositionAttempt))
  val verifiedRejectedS = seenS(RejectVerifiedS(DispositionAttempt))
  val advanced2S = seenS(AdvanceS(Time2))
  val advanced100S = seenS(AdvanceS(Time100))
  val advanced101S = seenS(AdvanceS(Time101))
  val afterCompletedS = completedS and profile == SignAfterResolve
  val beforeCompletedS = completedS and profile == SignBeforeResolve
}
