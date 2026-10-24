//! Ordinary Build helper effects on the SAME data/data-field arenas. One field
//! event at a time; the last field and constructor node form one owner-applied
//! EffectPlan. Guard, caps, charge/refusal, freshness and application are owned.
use super::super::shared::adt_runtime::BuildReadC;
use super::super::shared::*;

pub(super) fn dispatch(
    sel: &Selected,
    pre: &dyn PreImageView,
    guards: &dyn GuardEvaluator,
) -> Result<EffectPlan, HostViolation> {
    let reads = pre.adt_reads();
    let mut writes = PlannedWrites::default();
    let mut delta = PlanDelta::default();
    match sel.rule {
        RuleIdC::XAdtHelperBuildEntry => {
            let x = reads.build_entry(sel)?;
            let state = BuildReadC {
                common: x.common,
                type_id: x.type_id,
                ctor_id: x.ctor_id,
                src: x.src,
                src_cursor: 0,
                field_start: x.field_start,
                staged: None,
                node: None,
                phase: BuildPhase::Copy,
            };
            writes.helper = Write::Set(Some(state.helper()));
        }
        RuleIdC::XAdtHelperBuildCell => {
            let x = reads.build_cell(sel)?;
            let mut state = x.state;
            state.staged = Some(x.value);
            writes.helper = Write::Set(Some(state.helper()));
        }
        RuleIdC::XAdtHelperBuildField => {
            let x = reads.build_field(sel)?;
            let mut state = x.state;
            writes.alloc_data_field = Some(x.staged);
            delta.alloc_data_fields = 1;
            state.src_cursor = x.next_cursor;
            state.staged = None;
            if guards.eval_guard(&GuardId::GBuildLast, pre)? {
                writes.alloc_constructor = Some(ConstructorNodeWriteC {
                    type_id: state.type_id,
                    ctor_id: state.ctor_id,
                    fields: SliceC::normalized(state.field_start, state.src.len),
                });
                delta.alloc_data_nodes = 1;
                state.node = Some(x.fresh_node);
                state.phase = BuildPhase::Done;
            }
            writes.helper = Write::Set(Some(state.helper()));
        }
        RuleIdC::XAdtHelperBuildExit => {
            writes.result = Write::Set(Some(HelperResultC::BuildDone {
                value: reads.build_done(sel)?,
            }));
            writes.helper = Write::Set(None);
        }
        _ => return Err(reads.foreign_rule(sel)),
    }
    Ok(EffectPlan {
        writes,
        delta_assert: delta,
    })
}
