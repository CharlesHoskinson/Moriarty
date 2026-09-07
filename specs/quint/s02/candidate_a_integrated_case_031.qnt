module candidate_a_integrated_case_031 {
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

pure val CASE_A4: A4Case = {descriptor: {caseId: "installment/two-fills/unused-successor/SignBeforeResolve", lifecycle: "installment", profile: "SignBeforeResolve", scenario: "two-fills", control: "unused-successor"}, scenario: InstallmentScenarioA4(TwoFillsI), profile: SignBeforeResolve, steps: List({kind: "transition", instruction: LifecycleIA4(PrepareParentI)},
{kind: "transition", instruction: LifecycleIA4(SignParentI)},
{kind: "transition", instruction: LifecycleIA4(ProposeI(FirstFillAttempt))},
{kind: "transition", instruction: LifecycleIA4(ProposeI(CancelAttempt))},
{kind: "adversarial-derivation", instruction: MutateA4(ProposedDerivationA4)},
{kind: "denied-probe", instruction: VerifyEvidenceA4({id: FirstFillAttempt, mode: NormalEvidenceA4})},
{kind: "transition", instruction: RejectEvidenceA4({id: FirstFillAttempt, mode: NormalEvidenceA4})},
{kind: "adversarial-derivation", instruction: MutateA4(VerifiedDerivationA4)},
{kind: "denied-probe", instruction: ReplayCommitA4(FirstFillAttempt)},
{kind: "transition", instruction: RejectConstructedA4(FirstFillAttempt)})}
pure val CASE_INDEX_A4: int = 31
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
