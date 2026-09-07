module candidate_a_integrated_case_043 {
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
import candidate_a_authority_installment_fixtures.* from "./candidate_a_authority_installment_fixtures"
import candidate_a_authority_installment.* from "./candidate_a_authority_installment"
import candidate_a_authority_swap_fixtures.* from "./candidate_a_authority_swap_fixtures"
import candidate_a_authority_swap.* from "./candidate_a_authority_swap"
import candidate_a_integrated_observer.* from "./candidate_a_integrated_observer"
import candidate_a_integrated_lowering.* from "./candidate_a_integrated_lowering"

pure val CASE_A4: A4Case = {descriptor: {caseId: "swap/funded1-timeout101/ordinary/SignBeforeResolve", lifecycle: "swap", profile: "SignBeforeResolve", scenario: "funded1-timeout101", control: "ordinary"}, scenario: SwapScenarioA4({funded: 1, mode: TimeoutS(Time101)}), profile: SignBeforeResolve, steps: List({kind: "transition", instruction: LifecycleSA4(PrepareS({id: FundingOneAttempt, principal: Alice}))},
{kind: "transition", instruction: LifecycleSA4(SignS({id: FundingOneAttempt, principal: Alice}))},
{kind: "transition", instruction: LifecycleSA4(ProposeS(FundingOneAttempt))},
{kind: "transition", instruction: LifecycleSA4(VerifyS(FundingOneAttempt))},
{kind: "transition", instruction: LifecycleSA4(CommitS(FundingOneAttempt))},
{kind: "transition", instruction: LifecycleSA4(AdvanceS(Time101))},
{kind: "transition", instruction: LifecycleSA4(PrepareS({id: DispositionAttempt, principal: Alice}))},
{kind: "transition", instruction: LifecycleSA4(SignS({id: DispositionAttempt, principal: Alice}))},
{kind: "transition", instruction: LifecycleSA4(ProposeS(DispositionAttempt))},
{kind: "transition", instruction: LifecycleSA4(VerifyS(DispositionAttempt))},
{kind: "transition", instruction: LifecycleSA4(CommitS(DispositionAttempt))},
{kind: "denied-probe", instruction: ReplayProposalA4(FundingOneAttempt)},
{kind: "denied-probe", instruction: ReplayCommitA4(FundingOneAttempt)},
{kind: "denied-probe", instruction: ReplayProposalA4(DispositionAttempt)},
{kind: "denied-probe", instruction: ReplayCommitA4(DispositionAttempt)})}
pure val CASE_INDEX_A4: int = 43
var authorityState: AAuthorityExecution
var latestEvent: A4Event
var caseIndex: int
var cursor: int
action init = all {
  authorityState' = initialA4(CASE_A4), latestEvent' = startA4(CASE_A4),
  caseIndex' = CASE_INDEX_A4, cursor' = 0
}
action step = {
  if (latestEvent.kind == "diagnostic") all {
    false, authorityState' = authorityState, latestEvent' = latestEvent,
    caseIndex' = caseIndex, cursor' = cursor
  } else if (cursor < CASE_A4.steps.length()) {
    val event = observeA4(authorityState, CASE_A4, cursor + 1, CASE_A4.steps.nth(cursor))
    all {
      latestEvent' = event,
      authorityState' = if (Set("transition", "adversarial-derivation").contains(event.kind))
        applyA4(authorityState, CASE_A4, event.arguments) else authorityState,
      cursor' = cursor + 1, caseIndex' = caseIndex
    }
  } else if (cursor == CASE_A4.steps.length()) all {
    latestEvent' = endA4(authorityState, CASE_A4, latestEvent), authorityState' = authorityState,
    cursor' = cursor + 1, caseIndex' = caseIndex
  } else all {
    false, authorityState' = authorityState, latestEvent' = latestEvent,
    caseIndex' = caseIndex, cursor' = cursor
  }
}
val noDiagnosticA4 = latestEvent.kind != "diagnostic"
val sourceInvariantA4 = caseIndex == CASE_INDEX_A4 and
  (if (CASE_A4.descriptor.lifecycle == "installment") safetyI(authorityState) else safetyS(authorityState))
val completeA4 = caseIndex == CASE_INDEX_A4 and latestEvent.kind == "case-end"
}
