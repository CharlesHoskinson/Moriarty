module candidate_a_authority_installment_test {
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

run fourOperationParentTest = assert(parentPlanI.operations ==
  List(fill1PlannedI, fill2PlannedI, cancel0PlannedI, cancel1PlannedI)
  and PROFILES_I.forall(p => validAuthorityPolicyA(parentPolicyI(p))))
run literalFidelityTest = assert(programI == installmentProgram and SCENARIOS_I.forall(s =>
  Set(FirstFillAttempt, SecondFillAttempt, CancelAttempt, FreshCancelAttempt, RecoveryAttempt).forall(id =>
    actualI(id, s) == AuthorityAdaptedA(expectedObservationI(id, s))))
  and SCENARIOS_I.forall(s => authorityPlanMatchesA(recoveryPlanI(s))))
}
