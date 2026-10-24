// Sole FamilyPlans forwarding implementation. The owner selects and validates.
// This module adds no guard, selector, counter update, allocation or mutation.
use crate::sherlock::helper_machine::shared::*;
mod adt_build;
mod adt_constructor;
mod adt_plan;
mod environment;
mod legacy;
mod scalar;
mod structural;

pub struct HostFamilyPlans;

impl FamilyPlans for HostFamilyPlans {
    fn plan_legacy(
        &self,
        sel: &Selected,
        pre: &dyn PreImageView,
        guards: &dyn GuardEvaluator,
    ) -> Result<EffectPlan, HostViolation> {
        legacy::dispatch(sel, pre, guards)
    }
    fn plan_scalar(
        &self,
        sel: &Selected,
        pre: &dyn PreImageView,
        guards: &dyn GuardEvaluator,
    ) -> Result<EffectPlan, HostViolation> {
        scalar::dispatch(sel, pre, guards)
    }
    fn plan_env_get(
        &self,
        sel: &Selected,
        pre: &dyn PreImageView,
        guards: &dyn GuardEvaluator,
    ) -> Result<EffectPlan, HostViolation> {
        environment::plan_env_get(sel, pre, guards)
    }
    fn plan_env_put(
        &self,
        sel: &Selected,
        pre: &dyn PreImageView,
        guards: &dyn GuardEvaluator,
    ) -> Result<EffectPlan, HostViolation> {
        environment::plan_env_put(sel, pre, guards)
    }
    fn plan_find(
        &self,
        sel: &Selected,
        pre: &dyn PreImageView,
        guards: &dyn GuardEvaluator,
    ) -> Result<EffectPlan, HostViolation> {
        environment::plan_find(sel, pre, guards)
    }
    fn plan_bind(
        &self,
        sel: &Selected,
        pre: &dyn PreImageView,
        guards: &dyn GuardEvaluator,
    ) -> Result<EffectPlan, HostViolation> {
        structural::plan_bind(sel, pre, guards)
    }
    fn plan_append(
        &self,
        sel: &Selected,
        pre: &dyn PreImageView,
        guards: &dyn GuardEvaluator,
    ) -> Result<EffectPlan, HostViolation> {
        environment::plan_append(sel, pre, guards)
    }
    fn plan_validate(
        &self,
        sel: &Selected,
        pre: &dyn PreImageView,
        guards: &dyn GuardEvaluator,
    ) -> Result<EffectPlan, HostViolation> {
        structural::plan_validate(sel, pre, guards)
    }
    fn plan_equal(
        &self,
        sel: &Selected,
        pre: &dyn PreImageView,
        guards: &dyn GuardEvaluator,
    ) -> Result<EffectPlan, HostViolation> {
        structural::plan_equal(sel, pre, guards)
    }
    fn plan_match(
        &self,
        sel: &Selected,
        pre: &dyn PreImageView,
        guards: &dyn GuardEvaluator,
    ) -> Result<EffectPlan, HostViolation> {
        structural::plan_match(sel, pre, guards)
    }
    fn plan_build(
        &self,
        sel: &Selected,
        pre: &dyn PreImageView,
        guards: &dyn GuardEvaluator,
    ) -> Result<EffectPlan, HostViolation> {
        adt_build::dispatch(sel, pre, guards)
    }
    fn plan_plan(
        &self,
        sel: &Selected,
        pre: &dyn PreImageView,
        _guards: &dyn GuardEvaluator,
    ) -> Result<EffectPlan, HostViolation> {
        adt_plan::dispatch(sel, pre)
    }
    fn plan_bookkeeping_release(
        &self,
        sel: &Selected,
        pre: &dyn PreImageView,
        guards: &dyn GuardEvaluator,
    ) -> Result<EffectPlan, HostViolation> {
        environment::plan_bookkeeping_release(sel, pre, guards)
    }
}

// Closed selected realization. The owner constructs this fixed implementation.
pub(crate) struct WordFamilyPlans {
    layout: crate::sherlock_word_lowering::WordLayout,
}
impl WordFamilyPlans {
    pub(crate) fn new(layout: crate::sherlock_word_lowering::WordLayout) -> Self {
        Self { layout }
    }
}

impl FamilyPlans for WordFamilyPlans {
    fn plan_legacy(
        &self,
        sel: &Selected,
        pre: &dyn PreImageView,
        guards: &dyn GuardEvaluator,
    ) -> Result<EffectPlan, HostViolation> {
        legacy::dispatch(sel, pre, guards)
    }
    fn plan_scalar(
        &self,
        sel: &Selected,
        pre: &dyn PreImageView,
        guards: &dyn GuardEvaluator,
    ) -> Result<EffectPlan, HostViolation> {
        scalar::dispatch_word(sel, pre, guards, self.layout)
    }
    fn plan_env_get(
        &self,
        sel: &Selected,
        pre: &dyn PreImageView,
        guards: &dyn GuardEvaluator,
    ) -> Result<EffectPlan, HostViolation> {
        environment::plan_env_get(sel, pre, guards)
    }
    fn plan_env_put(
        &self,
        sel: &Selected,
        pre: &dyn PreImageView,
        guards: &dyn GuardEvaluator,
    ) -> Result<EffectPlan, HostViolation> {
        environment::plan_env_put(sel, pre, guards)
    }
    fn plan_find(
        &self,
        sel: &Selected,
        pre: &dyn PreImageView,
        guards: &dyn GuardEvaluator,
    ) -> Result<EffectPlan, HostViolation> {
        environment::plan_find(sel, pre, guards)
    }
    fn plan_bind(
        &self,
        sel: &Selected,
        pre: &dyn PreImageView,
        guards: &dyn GuardEvaluator,
    ) -> Result<EffectPlan, HostViolation> {
        structural::plan_bind(sel, pre, guards)
    }
    fn plan_append(
        &self,
        sel: &Selected,
        pre: &dyn PreImageView,
        guards: &dyn GuardEvaluator,
    ) -> Result<EffectPlan, HostViolation> {
        environment::plan_append(sel, pre, guards)
    }
    fn plan_validate(
        &self,
        sel: &Selected,
        pre: &dyn PreImageView,
        guards: &dyn GuardEvaluator,
    ) -> Result<EffectPlan, HostViolation> {
        structural::plan_validate(sel, pre, guards)
    }
    fn plan_equal(
        &self,
        sel: &Selected,
        pre: &dyn PreImageView,
        guards: &dyn GuardEvaluator,
    ) -> Result<EffectPlan, HostViolation> {
        structural::plan_equal(sel, pre, guards)
    }
    fn plan_match(
        &self,
        sel: &Selected,
        pre: &dyn PreImageView,
        guards: &dyn GuardEvaluator,
    ) -> Result<EffectPlan, HostViolation> {
        structural::plan_match(sel, pre, guards)
    }
    fn plan_build(
        &self,
        sel: &Selected,
        pre: &dyn PreImageView,
        guards: &dyn GuardEvaluator,
    ) -> Result<EffectPlan, HostViolation> {
        adt_build::dispatch(sel, pre, guards)
    }
    fn plan_plan(
        &self,
        sel: &Selected,
        pre: &dyn PreImageView,
        _guards: &dyn GuardEvaluator,
    ) -> Result<EffectPlan, HostViolation> {
        adt_plan::dispatch(sel, pre)
    }
    fn plan_bookkeeping_release(
        &self,
        sel: &Selected,
        pre: &dyn PreImageView,
        guards: &dyn GuardEvaluator,
    ) -> Result<EffectPlan, HostViolation> {
        environment::plan_bookkeeping_release(sel, pre, guards)
    }
}
