//! Ordinary effects of the existing specified ADT legacy rows and RetCtorFields
//! Scalar/Append entry sites. Selection/checked reads/guards/apply stay owner-only.
use super::super::shared::*;
use super::super::shared::adt_runtime::ConstructorFieldsReadC;

pub(super) fn dispatch(sel: &Selected, pre: &dyn PreImageView)
    -> Result<EffectPlan, HostViolation> {
    let reads = pre.adt_reads();
    let mut writes = PlannedWrites::default();
    let mut delta = PlanDelta::default();
    match sel.rule {
        RuleIdC::XAdtEvalConstructNullary => {
            writes.control = Write::Set(ControlC::Return { value: reads.nullary_construct(sel)? });
        }
        RuleIdC::XAdtEvalConstructFields => {
            let x = reads.construct_fields(sel)?;
            let frame = ConstructorFieldsReadC { type_id: x.type_id, ctor_id: x.ctor_id,
                index: 0, field_type: x.field_type, checked: false,
                completed: SliceC::empty(), rest: x.rest, saved_env: x.saved_env, origin: x.origin };
            writes.control = Write::Set(ControlC::ScheduleNext {
                frame: frame.frame(), next: x.first, env: x.saved_env });
        }
        RuleIdC::XAdtReturnCtorFieldsTyped => {
            let mut frame = reads.constructor_frame(sel)?;
            frame.checked = true;
            writes.frame = Some(FrameWrite::RewriteTop(frame.frame()));
            writes.result = Write::Set(None);
        }
        RuleIdC::XAdtReturnCtorFieldsMistyped => {
            writes.control = Write::Set(ControlC::Error { code: reads.application_error(sel)? });
            writes.result = Write::Set(None);
        }
        RuleIdC::XAdtReturnCtorFieldsNext => {
            let x = reads.constructor_next(sel)?;
            let frame = ConstructorFieldsReadC { index: x.next_index,
                field_type: x.field_type, checked: false, completed: x.completed,
                rest: x.rest, ..x.frame };
            writes.frame = Some(FrameWrite::RewriteTop(frame.frame()));
            writes.env_root = Write::Set(frame.saved_env);
            writes.control = Write::Set(ControlC::Eval { expr: x.next });
            writes.control_origin = Write::Set(x.next);
            writes.result = Write::Set(None);
        }
        RuleIdC::XAdtReturnCtorFieldsFinal => {
            let x = reads.constructor_final(sel)?;
            writes.frame = Some(FrameWrite::Pop);
            writes.env_root = Write::Set(x.frame.saved_env);
            writes.control = Write::Set(ControlC::Materialize {
                type_id: x.frame.type_id, ctor_id: x.frame.ctor_id, fields: x.completed });
            writes.control_origin = Write::Set(x.frame.origin);
            writes.result = Write::Set(None);
            delta.depth = -1;
        }
        RuleIdC::XAdtMaterializeDone => {
            writes.control = Write::Set(ControlC::Return { value: reads.materialized_value(sel)? });
            writes.result = Write::Set(None);
        }
        RuleIdC::XAdtCasesPlanHit => {
            let x = reads.plan_hit(sel)?;
            writes.control = Write::Set(ControlC::ScheduleNext {
                frame: FrameRowC::ScopeF { saved_env: x.saved_env, origin: x.origin },
                next: x.body, env: x.arm_env });
            writes.result = Write::Set(None);
        }
        RuleIdC::XAdtCasesPlanMiss => {
            writes.control = Write::Set(ControlC::Error { code: reads.application_error(sel)? });
            writes.result = Write::Set(None);
        }
        _ => return Err(reads.foreign_rule(sel)),
    }
    Ok(EffectPlan { writes, delta_assert: delta })
}

/// Expected type is copied only from the checked frame's field_type. The
/// existing Scalar TestType/tag/exit path remains the actual type-test engine.
pub(super) fn scalar_entry(sel: &Selected, pre: &dyn PreImageView)
    -> Result<EffectPlan, HostViolation> {
    let x = pre.adt_reads().constructor_scalar(sel)?;
    let mut writes = PlannedWrites::default();
    writes.helper = Write::Set(Some(HelperControlC::Scalar {
        common: x.common, mode: ScalarMode::TestType, operand_a: x.value,
        operand_b: None, expected: Some(x.expected), phase: ScalarPhase::First,
        reg_a: 0, staged: ScalarStaged::None }));
    Ok(EffectPlan { writes, delta_assert: PlanDelta::default() })
}

/// Reuse the installed Append sequence microsteps for the evaluated prefix;
/// allocation still happens only after every field has returned and type-tested.
pub(super) fn append_entry(sel: &Selected, pre: &dyn PreImageView)
    -> Result<EffectPlan, HostViolation> {
    let x = pre.adt_reads().constructor_append(sel)?;
    let mut writes = PlannedWrites::default();
    writes.helper = Write::Set(Some(HelperControlC::Append {
        common: x.common, src: x.completed, value: x.value, src_cursor: 0,
        dest_start: x.dest_start, staged: None, phase: AppendPhase::Copy }));
    Ok(EffectPlan { writes, delta_assert: PlanDelta::default() })
}
