// Private adapter for the unchanged source fragment. No row semantics here.
use crate::sherlock::helper_machine::shared::*;
include!("../fragments/scalar.proposed.rs");
pub(super) fn dispatch(sel: &Selected, pre: &dyn PreImageView, guards: &dyn GuardEvaluator)
    -> Result<EffectPlan, HostViolation>
{
    plan_scalar(sel, pre, guards)
}
