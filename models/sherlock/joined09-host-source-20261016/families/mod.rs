// Sole FamilyPlans forwarding implementation. The owner selects and validates.
// This module adds no guard, selector, counter update, allocation or mutation.
use crate::shared::*;
mod legacy;
mod scalar;
mod environment;
mod structural;

pub struct HostFamilyPlans;

impl FamilyPlans for HostFamilyPlans {
    fn plan_legacy(&self, sel: &Selected, pre: &dyn PreImageView, guards: &dyn GuardEvaluator)
        -> Result<EffectPlan, HostViolation>
    {
        legacy::dispatch(sel, pre, guards)
    }
    fn plan_scalar(&self, sel: &Selected, pre: &dyn PreImageView, guards: &dyn GuardEvaluator)
        -> Result<EffectPlan, HostViolation>
    {
        scalar::dispatch(sel, pre, guards)
    }
    fn plan_env_get(&self, sel: &Selected, pre: &dyn PreImageView, guards: &dyn GuardEvaluator)
        -> Result<EffectPlan, HostViolation>
    {
        environment::plan_env_get(sel, pre, guards)
    }
    fn plan_env_put(&self, sel: &Selected, pre: &dyn PreImageView, guards: &dyn GuardEvaluator)
        -> Result<EffectPlan, HostViolation>
    {
        environment::plan_env_put(sel, pre, guards)
    }
    fn plan_find(&self, sel: &Selected, pre: &dyn PreImageView, guards: &dyn GuardEvaluator)
        -> Result<EffectPlan, HostViolation>
    {
        environment::plan_find(sel, pre, guards)
    }
    fn plan_bind(&self, sel: &Selected, pre: &dyn PreImageView, guards: &dyn GuardEvaluator)
        -> Result<EffectPlan, HostViolation>
    {
        structural::plan_bind(sel, pre, guards)
    }
    fn plan_append(&self, sel: &Selected, pre: &dyn PreImageView, guards: &dyn GuardEvaluator)
        -> Result<EffectPlan, HostViolation>
    {
        environment::plan_append(sel, pre, guards)
    }
    fn plan_validate(&self, sel: &Selected, pre: &dyn PreImageView, guards: &dyn GuardEvaluator)
        -> Result<EffectPlan, HostViolation>
    {
        structural::plan_validate(sel, pre, guards)
    }
    fn plan_equal(&self, sel: &Selected, pre: &dyn PreImageView, guards: &dyn GuardEvaluator)
        -> Result<EffectPlan, HostViolation>
    {
        structural::plan_equal(sel, pre, guards)
    }
    fn plan_match(&self, sel: &Selected, pre: &dyn PreImageView, guards: &dyn GuardEvaluator)
        -> Result<EffectPlan, HostViolation>
    {
        structural::plan_match(sel, pre, guards)
    }
    fn plan_bookkeeping_release(&self, sel: &Selected, pre: &dyn PreImageView, guards: &dyn GuardEvaluator)
        -> Result<EffectPlan, HostViolation>
    {
        environment::plan_bookkeeping_release(sel, pre, guards)
    }
}
