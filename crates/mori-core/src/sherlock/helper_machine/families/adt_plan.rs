//! Ordinary ordered plan-helper microsteps over actual ProgramView plan rows.
//! No independent pattern interpreter, source-arm scan, occurrence cache, value
//! type judgment, selector, guard, budget or mutation of MachineImage exists here.
use super::super::shared::adt_runtime::{PlanReadC, load_plan_node};
use super::super::shared::*;

pub(super) fn dispatch(
    sel: &Selected,
    pre: &dyn PreImageView,
) -> Result<EffectPlan, HostViolation> {
    let reads = pre.adt_reads();
    let mut writes = PlannedWrites::default();
    let mut delta = PlanDelta::default();
    match sel.rule {
        RuleIdC::XAdtHelperPlanEntry => {
            let x = reads.plan_entry(sel)?;
            let state = PlanReadC {
                common: x.common,
                source_match: x.source_match,
                scrutinee: x.scrutinee,
                saved_env: x.saved_env,
                partial: x.saved_env,
                node: x.root,
                phase: PlanPhase::Walk,
                after_walk: PlanAfterWalk::Test,
                cur: x.scrutinee,
                path: SliceC::empty(),
                path_cursor: 0,
                arm: None,
                status: TriStatus::Pending,
            };
            writes.helper = Write::Set(Some(load_plan_node(state, x.root, x.row).helper()));
        }
        RuleIdC::XAdtHelperPlanWalk => {
            let x = reads.plan_walk(sel)?;
            let mut state = x.state;
            state.cur = x.value;
            state.path_cursor = x.next_cursor;
            state.phase = x.next_phase;
            writes.helper = Write::Set(Some(state.helper()));
        }
        RuleIdC::XAdtHelperPlanSwitch => {
            let x = reads.plan_switch(sel)?;
            let next = x.case.unwrap_or(x.default);
            let row = reads.plan_node(sel, &next)?;
            writes.helper = Write::Set(Some(load_plan_node(x.state, next, row).helper()));
        }
        RuleIdC::XAdtHelperPlanBind => {
            let x = reads.plan_bind(sel)?;
            let row = reads.plan_node(sel, &x.next)?;
            let mut state = x.state;
            writes.alloc_env = Some(EnvCellC {
                binder: x.binder,
                value: state.cur,
                previous: state.partial,
            });
            delta.alloc_environment_cells = 1;
            state.partial = Some(x.fresh_env);
            writes.helper = Write::Set(Some(load_plan_node(state, x.next, row).helper()));
        }
        RuleIdC::XAdtHelperPlanSelect => {
            let mut state = reads.plan_state(sel)?;
            state.status = TriStatus::Positive;
            state.phase = PlanPhase::Done;
            writes.helper = Write::Set(Some(state.helper()));
        }
        RuleIdC::XAdtHelperPlanNoMatch => {
            let mut state = reads.plan_state(sel)?;
            state.status = TriStatus::Negative;
            state.phase = PlanPhase::Done;
            writes.helper = Write::Set(Some(state.helper()));
        }
        RuleIdC::XAdtHelperPlanExit => {
            let (arm, env) = reads.plan_matched(sel)?;
            writes.result = Write::Set(Some(HelperResultC::PlanMatched { arm, env }));
            writes.helper = Write::Set(None);
        }
        RuleIdC::XAdtHelperPlanFailure => {
            writes.result = Write::Set(Some(HelperResultC::PlanNoMatch));
            writes.helper = Write::Set(None);
        }
        _ => return Err(reads.foreign_rule(sel)),
    }
    Ok(EffectPlan {
        writes,
        delta_assert: delta,
    })
}
