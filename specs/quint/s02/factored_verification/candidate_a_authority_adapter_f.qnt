module candidate_a_authority_adapter_f {
  import candidate_a_joint_f as J from "./candidate_a_joint_f"
  import effects.* from "../effects"
  import observations.* from "../observations"
  import policies.* from "../policies"
  import execution.* from "../execution"
  import candidate_a_types.* from "../candidate_a_types"
  import candidate_a_core_f.* from "./candidate_a_core_f"
  import candidate_a_projection_f.* from "./candidate_a_projection_f"

  type AAuthorityRequest = {before: APredecessor, input: AInput, now: ATime}
  type ACall = AgreementCallA(AAuthorityRequest)
    | CancellationCallA({before: APredecessor, now: ATime})
  type APlanId = SwapPlanA | InstallmentPlanA | RecoveryPlanA
  type AResolvedPlan = {identity: APlanId,
    operations: List[PlannedOperation[APredecessor,AContinuation,ACall]]}
  type AAuthorityObservation = CandidateObservation[APredecessor,AContinuation,ACall,AResolvedPlan]
  type AAuthorityPolicy = Policy[APredecessor,AContinuation,ACall,AResolvedPlan]
  type AAuthorityExecution = ExecutionState[APredecessor,AContinuation,ACall,AResolvedPlan]
  type AAuthorityEvidence = EvidenceBundle[APredecessor,AContinuation,ACall,AResolvedPlan]
  type AAuthorityAdaptation = AuthorityAdaptedA(AAuthorityObservation)
    | AuthorityComputationDiagnosticA(ATransactionEvaluation)
    | AuthorityExtractionDiagnosticA(AEffectExtraction)
    | AuthorityInvalidRequestA

  pure def acceptedOutcomeA(op: Operation): SemanticOutcome = match op {
    | OpFund => FundingAccepted
    | OpSettle => Settlement
    | OpVoluntaryRefund => VoluntaryRefund
    | OpDeadlineRefund => DeadlineRefund
    | OpFillSlot(fill) => if (fill.slot == 1) FirstInstallment else SecondInstallment
    | OpRecover(_) => Recovery
    | OpCancelParent(_) => Cancellation
  }

  pure def adaptAuthorityA(request: AAuthorityRequest, op: Operation, plan: AResolvedPlan, display: DisplayProjection): AAuthorityAdaptation =
    J::adaptAuthorityF(request, op, plan, display)
  pure def authorityObservationMatchesA(op: Operation, obs: AAuthorityObservation): bool =
    match obs.artifactAndCall {
      | AgreementCallA(request) => adaptAuthorityA(request, op, obs.resolvedPlan, obs.display)
          == AuthorityAdaptedA(obs)
      | CancellationCallA(call) => adaptAuthorityA({before: call.before, input: NoAInput,
          now: call.now}, op, obs.resolvedPlan, obs.display) == AuthorityAdaptedA(obs)
    }
}
