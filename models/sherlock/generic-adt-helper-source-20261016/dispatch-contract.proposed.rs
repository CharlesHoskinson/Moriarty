//! families::adt_helper -- PROPOSED Sol module (sibling of `shared`). Pure effect
//! construction for the 12 x.adt.helper rows; no admission, selection, charge, cap,
//! physical check, apply, Outcome, StepEvent or StepInv code.
use crate::sherlock::helper_machine::shared::{HostViolation, HostViolationKind};
use crate::sherlock::helper_machine::shared::adt::{AdtEffectPlan, AdtFamilyError,
    AdtGuardEvaluator, AdtHelperFamilyPlans, AdtHelperRuleC, AdtPreImageView, SelectedAdt};

pub struct AdtHelperFamilies;

fn foreign_rule() -> AdtFamilyError {
    AdtFamilyError::Host(HostViolation { kind: HostViolationKind::WrongVariant, clause: None,
        rule: None, locator: Some("foreign rule for family".to_string()) })
}

impl AdtHelperFamilyPlans for AdtHelperFamilies {
    fn plan_adt_build(&self, sel: &SelectedAdt, pre: &dyn AdtPreImageView,
        g: &dyn AdtGuardEvaluator) -> Result<AdtEffectPlan, AdtFamilyError> {
        match sel.rule {
            AdtHelperRuleC::BuildEntry => build_entry(pre),
            AdtHelperRuleC::BuildCell => build_cell(pre),
            AdtHelperRuleC::BuildField => build_field(pre, g),
            AdtHelperRuleC::BuildExit => build_exit(pre),
            AdtHelperRuleC::PlanEntry | AdtHelperRuleC::PlanWalk | AdtHelperRuleC::PlanSwitch
            | AdtHelperRuleC::PlanBind | AdtHelperRuleC::PlanSelect | AdtHelperRuleC::PlanNoMatch
            | AdtHelperRuleC::PlanExit | AdtHelperRuleC::PlanFailure => Err(foreign_rule()),
        }
    }
    fn plan_adt_plan(&self, sel: &SelectedAdt, pre: &dyn AdtPreImageView)
        -> Result<AdtEffectPlan, AdtFamilyError> {
        match sel.rule {
            AdtHelperRuleC::PlanEntry => plan_entry(pre),
            AdtHelperRuleC::PlanWalk => plan_walk(pre),
            AdtHelperRuleC::PlanSwitch => plan_switch(pre),
            AdtHelperRuleC::PlanBind => plan_bind(pre),
            AdtHelperRuleC::PlanSelect => plan_select(pre),
            AdtHelperRuleC::PlanNoMatch => plan_nomatch(pre),
            AdtHelperRuleC::PlanExit => plan_exit(pre),
            AdtHelperRuleC::PlanFailure => plan_failure(pre),
            AdtHelperRuleC::BuildEntry | AdtHelperRuleC::BuildCell
            | AdtHelperRuleC::BuildField | AdtHelperRuleC::BuildExit => Err(foreign_rule()),
        }
    }
}
