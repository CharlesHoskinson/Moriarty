// Private adapter for the unchanged source fragment. No row semantics here.
use crate::sherlock::helper_machine::shared::*;
include!("../fragments/legacy.proposed.rs");
pub(super) fn dispatch(
    sel: &Selected,
    pre: &dyn PreImageView,
    guards: &dyn GuardEvaluator,
) -> Result<EffectPlan, HostViolation> {
    match sel.rule {
        RuleIdC::XAdtEvalConstructNullary
        | RuleIdC::XAdtEvalConstructFields
        | RuleIdC::XAdtReturnCtorFieldsTyped
        | RuleIdC::XAdtReturnCtorFieldsMistyped
        | RuleIdC::XAdtReturnCtorFieldsNext
        | RuleIdC::XAdtReturnCtorFieldsFinal
        | RuleIdC::XAdtMaterializeDone
        | RuleIdC::XAdtCasesPlanHit
        | RuleIdC::XAdtCasesPlanMiss => super::adt_constructor::dispatch(sel, pre),
        _ => plan_legacy(sel, pre, guards),
    }
}
