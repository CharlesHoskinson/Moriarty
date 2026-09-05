module candidate_a_authority_installment_harness_f {
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
import candidate_a_authority_installment_fixtures_f.* from "./candidate_a_authority_installment_fixtures_f"
import candidate_a_authority_installment_f.* from "./candidate_a_authority_installment_f"

var state: AAuthorityExecution
var scenario: ScenarioI
var profile: SigningProfile
var cursor: int
var history: List[CommandI]
var lastAtomicI: bool
action init = {
  nondet chosenScenario = SCENARIOS_I.oneOf()
  nondet chosenProfile = PROFILES_I.oneOf()
  all { state' = unsignedI, scenario' = chosenScenario, profile' = chosenProfile, cursor' = 0, history' = List(), lastAtomicI' = true }
}
action performI(cmd: CommandI): bool = all {
  cursor < routeI(scenario).length(), routeI(scenario).nth(cursor) == cmd,
  canCommandI(state, scenario, profile, cmd),
  state' = applyCommandI(state, scenario, profile, cmd), scenario' = scenario, profile' = profile,
  cursor' = cursor + 1, history' = history.append(cmd),
  lastAtomicI' = transitionExactI(state, applyCommandI(state, scenario, profile, cmd), cmd),
}
action step = if (cursor < routeI(scenario).length()) performI(routeI(scenario).nth(cursor))
  else all { false, state' = state, scenario' = scenario, profile' = profile, cursor' = cursor, history' = history, lastAtomicI' = lastAtomicI }
def seenI(cmd: CommandI): bool = history.foldl(false, (found, actual) => found or actual == cmd)
val installmentSafetyI = safetyI(state) and lastAtomicI and cursor == history.length() and cursor <= routeI(scenario).length()
val actionWitnessI = history.length() > 0
val allCasesWitnessI = cursor == routeI(scenario).length() and cursor > 0
val parentPreparedI = seenI(PrepareParentI)
val parentSignedI = seenI(SignParentI)
val firstProposedI = seenI(ProposeI(FirstFillAttempt))
val cancelProposedI = seenI(ProposeI(CancelAttempt))
val firstVerifiedI = seenI(VerifyI(FirstFillAttempt))
val cancelVerifiedI = seenI(VerifyI(CancelAttempt))
val firstCommittedI = seenI(CommitI(FirstFillAttempt))
val cancelCommittedI = seenI(CommitI(CancelAttempt))
val firstRejectedI = seenI(RejectStaleI(FirstFillAttempt))
val cancelRejectedI = seenI(RejectStaleI(CancelAttempt))
val secondProposedI = seenI(ProposeI(SecondFillAttempt))
val secondVerifiedI = seenI(VerifyI(SecondFillAttempt))
val secondCommittedI = seenI(CommitI(SecondFillAttempt))
val freshProposedI = seenI(ProposeI(FreshCancelAttempt))
val freshVerifiedI = seenI(VerifyI(FreshCancelAttempt))
val freshCommittedI = seenI(CommitI(FreshCancelAttempt))
val clockAdvancedI = seenI(AdvanceI)
val recoveryPreparedI = seenI(PrepareRecoveryI)
val recoverySignedI = seenI(SignRecoveryI)
val recoveryProposedI = seenI(ProposeI(RecoveryAttempt))
val recoveryVerifiedI = seenI(VerifyI(RecoveryAttempt))
val recoveryCommittedI = seenI(CommitI(RecoveryAttempt))
val recoveryRejectedI = seenI(RejectRecoveryI)
val afterProfileCompletedI = allCasesWitnessI and profile == SignAfterResolve
val beforeProfileCompletedI = allCasesWitnessI and profile == SignBeforeResolve
}
