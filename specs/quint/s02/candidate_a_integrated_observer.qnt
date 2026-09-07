module candidate_a_integrated_observer {
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

type A4Signed = SignedPolicy[APredecessor,AContinuation,ACall,AResolvedPlan]
type A4Parent = Parent[A4Signed]
type A4Entry = Entry[A4Signed]
type A4Scenario = InstallmentScenarioA4(ScenarioI) | SwapScenarioA4(ScenarioS)
type A4CaseDescriptor = {caseId: str, lifecycle: str, profile: str,
  scenario: str, control: str}
type A4DerivationStage = ProposedDerivationA4 | VerifiedDerivationA4
type A4Diagnostic = ObserverIssueA4(str)
  | AdapterIssueA4({call: ACall, adaptation: AAuthorityAdaptation})
type A4Command = PrepareA4({policy: AAuthorityPolicy, signer: Principal})
  | SignA4({policy: AAuthorityPolicy, signer: Principal, token: int})
  | ProposeA4({id: AttemptId, operation: Operation,
      observation: AAuthorityObservation, actor: Principal})
  | VerifyA4({id: AttemptId, evidence: AAuthorityEvidence})
  | CommitA4({id: AttemptId})
  | RejectProposedA4({id: AttemptId, evidence: AAuthorityEvidence})
  | RejectVerifiedA4({id: AttemptId})
  | AdvanceA4({environment: Environment})
  | CancelParentProbeA4({parent: A4Parent, entry: A4Entry, prepared: A4Entry})
  | ConsumeSlotProbeA4({parent: A4Parent, entry: A4Entry, slot: int, prepared: A4Entry})
  | FinancialProbeA4({operation: Operation, effects: List[Transfer]})
  | CoreAcceptedProbeA4({request: AAuthorityRequest})
  | PlanMatchesProbeA4({plan: AResolvedPlan})
  | DeriveA4({mutationId: str, baseCaseId: str, baseSequence: int,
      attempt: AAuthorityAttempt, evidence: AAuthorityEvidence, stage: A4DerivationStage})
  | CaseStartA4(A4CaseDescriptor)
  | CaseEndA4({status: str})
  | DiagnosticA4(A4Diagnostic)
type A4Guard = InstallmentCommandGuardA4(CommandI) | SwapCommandGuardA4(CommandS)
  | PrepareGuardA4 | SignGuardA4 | ProposeGuardA4 | VerifyGuardA4 | CommitGuardA4
  | RejectProposedGuardA4 | RejectVerifiedGuardA4 | AdvanceGuardA4
  | CancelParentGuardA4 | ConsumeSlotGuardA4 | FinancialGuardA4
  | CoreAcceptedGuardA4 | PlanMatchesGuardA4 | NoGuardA4
type A4Arguments = {guard: A4Guard, command: A4Command}
type A4Extraction = ExtractionObservedA4(AEffectExtraction) | ExtractionNotApplicableA4
type A4Computation = {request: AAuthorityRequest, evaluation: ATransactionEvaluation,
  extraction: A4Extraction}
type A4Event = {caseId: str, profile: str, sequence: int, kind: str,
  arguments: A4Arguments, observedGuard: bool, computations: List[A4Computation]}

type A4AttemptView = MissingAttemptA4 | PresentAttemptA4(AAuthorityAttempt)
pure def attemptA4(state: AAuthorityExecution, id: AttemptId): A4AttemptView = match state.attempts.get(id) {
  | NoAttempt => MissingAttemptA4
  | ProposedAttempt(a) => PresentAttemptA4(a)
  | VerifiedOperation(v) => PresentAttemptA4(v.attempt)
  | ExecutedOperation(v) => PresentAttemptA4(v.attempt)
  | RejectedOperation(r) => PresentAttemptA4(r.attempt)
}
pure def computeA4(request: AAuthorityRequest): A4Computation = {
  val evaluation = computeTransaction(request.before.program, request.before.state, request.input, request.now)
  {request: request, evaluation: evaluation, extraction: match evaluation {
    | TransactionComputedA(raw) => ExtractionObservedA4(extractCommittedEffects(request.before.program,
        request.before.state, request.input, request.now, raw))
    | _ => ExtractionNotApplicableA4 }}
}
pure def callComputationsA4(call: ACall): List[A4Computation] = match call {
  | CancellationCallA(_) => List()
  | AgreementCallA(request) => List(computeA4(request))
}
pure def planComputationsA4(plan: AResolvedPlan): List[A4Computation] =
  plan.operations.foldl(List(), (out, op) => out.concat(callComputationsA4(op.artifactAndCall)))
pure def policyComputationsA4(policy: AAuthorityPolicy): List[A4Computation] = match policy.binding {
  | BeforeResolution(_) => List()
  | AfterResolution(view) => planComputationsA4(view.identity)
}
pure def retainedComputationsA4(state: AAuthorityExecution, id: AttemptId): List[A4Computation] =
  match attemptA4(state, id) {
    | MissingAttemptA4 => List()
    | PresentAttemptA4(a) => callComputationsA4(a.observation.artifactAndCall)
  }
pure def computationsA4(state: AAuthorityExecution, command: A4Command): List[A4Computation] = match command {
  | PrepareA4(c) => policyComputationsA4(c.policy)
  | SignA4(c) => policyComputationsA4(c.policy)
  | ProposeA4(c) => callComputationsA4(c.observation.artifactAndCall)
  | VerifyA4(c) => retainedComputationsA4(state, c.id)
  | CommitA4(c) => retainedComputationsA4(state, c.id)
  | RejectProposedA4(c) => retainedComputationsA4(state, c.id)
  | RejectVerifiedA4(c) => retainedComputationsA4(state, c.id)
  | DeriveA4(c) => callComputationsA4(c.attempt.observation.artifactAndCall)
  | CoreAcceptedProbeA4(c) => List(computeA4(c.request))
  | PlanMatchesProbeA4(c) => planComputationsA4(c.plan)
  | DiagnosticA4(detail) => match detail {
      | ObserverIssueA4(_) => List() | AdapterIssueA4(issue) => callComputationsA4(issue.call) }
  | _ => List()
}
pure def computedA4(c: A4Computation): bool = match c.evaluation {
  | TransactionComputedA(_) => match c.extraction {
      | ExtractionObservedA4(extraction) => match extraction {
          | EffectsExtractedA(_) => true | _ => false }
      | _ => false }
  | _ => false
}
pure def directTagA4(command: A4Command): A4Guard = match command {
  | PrepareA4(_) => PrepareGuardA4 | SignA4(_) => SignGuardA4
  | ProposeA4(_) => ProposeGuardA4 | VerifyA4(_) => VerifyGuardA4
  | CommitA4(_) => CommitGuardA4 | RejectProposedA4(_) => RejectProposedGuardA4
  | RejectVerifiedA4(_) => RejectVerifiedGuardA4 | AdvanceA4(_) => AdvanceGuardA4
  | CancelParentProbeA4(_) => CancelParentGuardA4 | ConsumeSlotProbeA4(_) => ConsumeSlotGuardA4
  | FinancialProbeA4(_) => FinancialGuardA4 | CoreAcceptedProbeA4(_) => CoreAcceptedGuardA4
  | PlanMatchesProbeA4(_) => PlanMatchesGuardA4 | _ => NoGuardA4
}
pure def directA4(command: A4Command): A4Arguments = {guard: directTagA4(command), command: command}
pure def directGuardA4(state: AAuthorityExecution, command: A4Command): bool = match command {
  | PrepareA4(c) => canPrepareSigningAuthorityA(state.authority, c.policy, c.signer)
  | SignA4(c) => canSignAuthorityA(state.authority, c.policy, c.signer, c.token)
  | ProposeA4(c) => canPropose(state, c.id, c.operation, c.observation, c.actor)
  | VerifyA4(c) => canVerifyAuthorityA(state, c.id, c.evidence)
  | CommitA4(c) => canCommitAuthorityA(state, c.id)
  | RejectProposedA4(c) => canRejectProposedAuthorityA(state, c.id, c.evidence)
  | RejectVerifiedA4(c) => canRejectVerifiedAuthorityA(state, c.id)
  | AdvanceA4(c) => validAuthorityExecutionA(state) and validEnvironment(c.environment)
      and c.environment.physicalTime > state.authority.context.environment.physicalTime
  | CancelParentProbeA4(c) => canCancelParent(c.parent, c.entry, c.prepared)
  | ConsumeSlotProbeA4(c) => canConsumeSlot(c.parent, c.entry, c.slot, c.prepared)
  | FinancialProbeA4(c) => operationFinancialGuard(state.authority.context, c.operation, c.effects)
  | PlanMatchesProbeA4(c) => authorityPlanMatchesA(c.plan)
  | CoreAcceptedProbeA4(c) => match computeA4(c.request).evaluation {
      | TransactionComputedA(r) => r.accepted | _ => false }
  | DeriveA4(c) => c.attempt.id == (if (c.baseSequence == 3) FirstFillAttempt else DispositionAttempt)
      and match state.attempts.get(c.attempt.id) {
        | ProposedAttempt(_) => c.stage == ProposedDerivationA4
        | RejectedOperation(r) => c.stage == VerifiedDerivationA4
            and r.stage == VerificationBoundary and r.attempt == c.attempt and r.evidence == c.evidence
        | _ => false }
  | CaseStartA4(_) => true | CaseEndA4(_) => true | DiagnosticA4(_) => false
}
pure def directApplyA4(state: AAuthorityExecution, command: A4Command): AAuthorityExecution = match command {
  | PrepareA4(c) => {...state, authority: applyPrepareSigning(state.authority, c.policy, c.signer)}
  | SignA4(c) => {...state, authority: applySign(state.authority, c.policy, c.signer, c.token)}
  | ProposeA4(c) => applyPropose(state, c.id, c.operation, c.observation, c.actor)
  | VerifyA4(c) => applyVerify(state, c.id, c.evidence)
  | CommitA4(c) => applyCommit(state, c.id)
  | RejectProposedA4(c) => applyRejectProposedAuthorityA(state, c.id, c.evidence)
  | RejectVerifiedA4(c) => applyRejectVerifiedAuthorityA(state, c.id)
  | AdvanceA4(c) => {...state, authority: {...state.authority,
      context: {...state.authority.context, environment: c.environment}}}
  | DeriveA4(c) => {...state, attempts: state.attempts.put(c.attempt.id,
      if (c.stage == ProposedDerivationA4) ProposedAttempt(c.attempt)
      else VerifiedOperation({attempt: c.attempt, evidence: c.evidence}))}
  | _ => state
}
}
