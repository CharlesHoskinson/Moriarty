// Private adapter for the unchanged source fragment. No row semantics here.
use crate::sherlock::helper_machine::shared::*;
use crate::sherlock_word_lowering::{ArithmeticLowering, UInt128Operation, WordLayout};
include!("../fragments/scalar.proposed.rs");
pub(super) fn dispatch(sel: &Selected, pre: &dyn PreImageView, guards: &dyn GuardEvaluator)
    -> Result<EffectPlan, HostViolation>
{
    plan_scalar(sel, pre, guards)
}

pub(super) fn dispatch_word(sel: &Selected, pre: &dyn PreImageView, guards: &dyn GuardEvaluator,
    layout: WordLayout) -> Result<EffectPlan, HostViolation>
{
    plan_scalar_with_word_layout(sel, pre, guards, layout)
}
