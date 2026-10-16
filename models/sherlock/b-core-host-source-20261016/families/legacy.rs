// Private adapter for the unchanged source fragment. No row semantics here.
use crate::shared::*;
include!("../fragments/legacy.proposed.rs");
pub(super) fn dispatch(sel: &Selected, pre: &dyn PreImageView, guards: &dyn GuardEvaluator)
    -> Result<EffectPlan, HostViolation>
{
    plan_legacy(sel, pre, guards)
}
