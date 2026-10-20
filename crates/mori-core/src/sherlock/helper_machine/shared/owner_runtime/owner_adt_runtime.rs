// owner_adt_runtime.rs — Coherent generic ADT runtime owner support:
// - All 22 AdtRuntimeReads methods connected to both PreImage and RecordingPreImageView
// - Checked constructor arity/sort/field/plan-node validation against ProgramView
// - SI-11/SI-12/SI-15/SI-16/SI-20/SI-21/SI-22/SI-23/SI-24 and SI-X01..SI-X10 checks
use super::*;
use super::adt::CtorIdC;
use super::adt_runtime::{
    AdtRuntimeReads, BuildCellReadC, BuildEntryReadC, BuildFieldReadC, BuildReadC,
    ConstructFieldsReadC, ConstructorAppendReadC, ConstructorFieldsReadC, ConstructorFinalReadC,
    ConstructorNextReadC, ConstructorScalarReadC, PlanBindReadC, PlanEntryReadC, PlanHitReadC,
    PlanReadC, PlanSwitchReadC, PlanWalkReadC,
};

pub(super) trait AdtPreImageAccess: PreImageView {
    fn program_view(&self) -> Result<&ProgramView, HostViolation>;
    fn record_adt_ref(&self, _r: RefC) {}
    fn record_adt_env(&self, _e: EnvRootC) {}
    fn record_adt_slots(&self, _s: SliceC) {}
}

impl<'a> AdtPreImageAccess for PreImage<'a> {
    fn program_view(&self) -> Result<&ProgramView, HostViolation> { super::pure_view(self) }
}

impl<'a> AdtPreImageAccess for RecordingPreImageView<'a> {
    fn program_view(&self) -> Result<&ProgramView, HostViolation> { super::pure_view(self.inner) }
    fn record_adt_ref(&self, r: RefC) {
        self.record(r);
    }
    fn record_adt_env(&self, e: EnvRootC) {
        self.record_env(e);
    }
    fn record_adt_slots(&self, s: SliceC) {
        self.record_slice(s);
    }
}

pub(super) fn adt_checked_ctor_arity(
    view: &ProgramView,
    type_id: TypeIdC,
    ctor_id: CtorIdC,
) -> Result<usize, HostViolation> {
    if type_id.0 < 4 {
        return Err(arith("dedicated type_id in constructor kind"));
    }
    let t_row = view.types.get(type_id.0).ok_or_else(|| oob("prog.types"))?;
    let c_row = view
        .constructors
        .get(ctor_id.0)
        .ok_or_else(|| oob("prog.constructors"))?;
    if c_row.owning_type != type_id {
        return Err(wrong("constructor owning_type mismatch"));
    }
    if ctor_id.0 < t_row.ctor_start.0 || ctor_id.0 - t_row.ctor_start.0 >= t_row.ctor_len {
        return Err(wrong("constructor outside owning type range"));
    }
    let end = c_row
        .field_start
        .checked_add(c_row.field_len)
        .ok_or_else(|| arith("ctor field slice overflow"))?;
    if end > view.ctor_fields.len() {
        return Err(oob("prog.ctorFields"));
    }
    Ok(c_row.field_len)
}

pub(super) fn adt_checked_ctor_sort(
    view: &ProgramView,
    type_id: TypeIdC,
    ctor_id: CtorIdC,
) -> Result<TypeIdC, HostViolation> {
    let _ = adt_checked_ctor_arity(view, type_id, ctor_id)?;
    Ok(type_id)
}

pub(super) fn plan_node_source_match(row: &PlanNodeC) -> usize {
    match *row {
        PlanNodeC::Switch { source_match, .. }
        | PlanNodeC::Bind { source_match, .. }
        | PlanNodeC::Select { source_match, .. }
        | PlanNodeC::NoMatch { source_match } => source_match,
    }
}

fn record_build_state<T: AdtPreImageAccess + ?Sized>(pre: &T, s: &BuildReadC) {
    pre.record_adt_slots(s.src);
    if let Some(st) = s.staged {
        pre.record_adt_ref(st);
    }
    if let Some(nd) = s.node {
        pre.record_adt_ref(nd);
    }
}

fn record_plan_state<T: AdtPreImageAccess + ?Sized>(pre: &T, s: &PlanReadC) {
    pre.record_adt_ref(s.scrutinee);
    pre.record_adt_ref(s.cur);
    pre.record_adt_env(s.saved_env);
    pre.record_adt_env(s.partial);
}

fn extract_plan_state<T: AdtPreImageAccess + ?Sized>(pre: &T) -> Result<PlanReadC, HostViolation> {
    let Some(HelperControlC::Plan {
        common,
        source_match,
        node,
        scrutinee,
        cur,
        path,
        path_cursor,
        after_walk,
        saved_env,
        partial,
        phase,
        status,
        arm,
    }) = pre.helper()
    else {
        return Err(wrong("expected Plan helper"));
    };
    let s = PlanReadC {
        common: *common,
        source_match: *source_match,
        node: *node,
        scrutinee: *scrutinee,
        cur: *cur,
        path: *path,
        path_cursor: *path_cursor,
        after_walk: *after_walk,
        saved_env: *saved_env,
        partial: *partial,
        phase: *phase,
        status: *status,
        arm: *arm,
    };
    record_plan_state(pre, &s);
    Ok(s)
}

fn kind_ctor_class(
    view: &ProgramView,
    k: LogicalKind,
) -> Result<Option<(TypeIdC, CtorIdC)>, HostViolation> {
    match k {
        LogicalKind::UInt | LogicalKind::Bool => Ok(None),
        LogicalKind::Nil => Ok(Some((TypeIdC::List, CtorIdC(0)))),
        LogicalKind::Cons => Ok(Some((TypeIdC::List, CtorIdC(1)))),
        LogicalKind::Leaf => Ok(Some((TypeIdC::Tree, CtorIdC(2)))),
        LogicalKind::Branch => Ok(Some((TypeIdC::Tree, CtorIdC(3)))),
        LogicalKind::Constructor { type_id, ctor_id } => {
            let _ = adt_checked_ctor_arity(view, type_id, ctor_id)?;
            Ok(Some((type_id, ctor_id)))
        }
        LogicalKind::Marker | LogicalKind::ConstructorExtensionReserved => {
            Err(arith("invalid kind in plan_switch"))
        }
    }
}

fn impl_foreign_rule() -> HostViolation {
    hv(
        HostViolationKind::WrongVariant,
        None,
        None,
        "foreign rule for family",
    )
}

fn impl_nullary_construct<T: AdtPreImageAccess + ?Sized>(
    pre: &T,
    selected: &Selected,
) -> Result<RefC, HostViolation> {
    if selected.rule != RuleIdC::XAdtEvalConstructNullary {
        return Err(wrong("nullary_construct rule"));
    }
    let ControlC::Eval { expr } = *pre.control() else {
        return Err(wrong("nullary_construct control"));
    };
    let ExpressionRowC::Construct {
        type_id,
        ctor_id,
        fields,
        constant: Some(cid),
    } = *pre.expr(&expr)?
    else {
        return Err(wrong("nullary_construct expr"));
    };
    if fields.len != 0 {
        return Err(wrong("nullary_construct non-empty fields"));
    }
    let view = pre.program_view()?;
    let arity = adt_checked_ctor_arity(view, type_id, ctor_id)?;
    if arity != 0 {
        return Err(wrong("nullary_construct arity != 0"));
    }
    let c_row = view
        .constants
        .get(cid)
        .ok_or_else(|| oob("prog.constants"))?;
    match c_row {
        RawConstRow::Constructor {
            marker_free,
            type_id: ct,
            ctor_id: cc,
            fields: cf,
        } if *marker_free && ct.0 == type_id.0 && cc.0 == ctor_id.0 && cf.len == 0 => {
            let r = RefC::Const(cid);
            pre.record_adt_ref(r);
            Ok(r)
        }
        _ => Err(wrong("nullary_construct constant row mismatch")),
    }
}

fn impl_construct_fields<T: AdtPreImageAccess + ?Sized>(
    pre: &T,
    selected: &Selected,
) -> Result<ConstructFieldsReadC, HostViolation> {
    if selected.rule != RuleIdC::XAdtEvalConstructFields {
        return Err(wrong("construct_fields rule"));
    }
    let ControlC::Eval { expr } = *pre.control() else {
        return Err(wrong("construct_fields control"));
    };
    let ExpressionRowC::Construct {
        type_id,
        ctor_id,
        fields,
        constant: None,
    } = *pre.expr(&expr)?
    else {
        return Err(wrong("construct_fields expr"));
    };
    if fields.len == 0 {
        return Err(wrong("construct_fields empty fields"));
    }
    let view = pre.program_view()?;
    let arity = adt_checked_ctor_arity(view, type_id, ctor_id)?;
    if arity != fields.len {
        return Err(wrong("construct_fields arity mismatch"));
    }
    let first = pre.child(&fields.start)?;
    let c_row = &view.constructors[ctor_id.0];
    let cf_row = view
        .ctor_fields
        .get(c_row.field_start)
        .ok_or_else(|| oob("prog.ctorFields"))?;
    if cf_row.type_id.0 >= view.types.len() {
        return Err(oob("prog.types"));
    }
    let rest_start = fields
        .start
        .checked_add(1)
        .ok_or_else(|| arith("fields.start + 1"))?;
    let rest = SliceC::normalized(rest_start, fields.len - 1);
    let saved_env = pre.env_root();
    pre.record_adt_env(saved_env);
    Ok(ConstructFieldsReadC {
        type_id,
        ctor_id,
        field_type: cf_row.type_id,
        first,
        rest,
        saved_env,
        origin: expr,
    })
}

fn impl_constructor_frame<T: AdtPreImageAccess + ?Sized>(
    pre: &T,
    selected: &Selected,
) -> Result<ConstructorFieldsReadC, HostViolation> {
    if selected.rule != RuleIdC::XAdtReturnCtorFieldsTyped {
        return Err(wrong("constructor_frame rule"));
    }
    if !matches!(pre.control(), ControlC::Return { .. }) {
        return Err(wrong("constructor_frame control"));
    }
    if !matches!(
        pre.result(),
        Some(HelperResultC::ScalarTypeTest { ok: true })
    ) {
        return Err(wrong("constructor_frame result"));
    }
    let Some(FrameRowC::ConstructorFieldsF {
        type_id,
        ctor_id,
        index,
        field_type,
        rest,
        saved_env,
        completed,
        checked: false,
        origin,
    }) = pre.top_frame()
    else {
        return Err(wrong("constructor_frame top_frame"));
    };
    pre.record_adt_env(*saved_env);
    pre.record_adt_slots(*completed);
    Ok(ConstructorFieldsReadC {
        type_id: *type_id,
        ctor_id: *ctor_id,
        index: *index,
        field_type: *field_type,
        rest: *rest,
        saved_env: *saved_env,
        completed: *completed,
        checked: false,
        origin: *origin,
    })
}

fn impl_constructor_next<T: AdtPreImageAccess + ?Sized>(
    pre: &T,
    selected: &Selected,
) -> Result<ConstructorNextReadC, HostViolation> {
    if selected.rule != RuleIdC::XAdtReturnCtorFieldsNext {
        return Err(wrong("constructor_next rule"));
    }
    if !matches!(pre.control(), ControlC::Return { .. }) {
        return Err(wrong("constructor_next control"));
    }
    let Some(HelperResultC::AppendDone { seq }) = pre.result() else {
        return Err(wrong("constructor_next result"));
    };
    let Some(FrameRowC::ConstructorFieldsF {
        type_id,
        ctor_id,
        index,
        field_type,
        rest,
        saved_env,
        completed,
        checked: true,
        origin,
    }) = pre.top_frame()
    else {
        return Err(wrong("constructor_next top_frame"));
    };
    if rest.len == 0 {
        return Err(wrong("constructor_next empty rest"));
    }
    let next = pre.child(&rest.start)?;
    let next_index = index
        .checked_add(1)
        .ok_or_else(|| arith("constructor_next index + 1"))?;
    let view = pre.program_view()?;
    let arity = adt_checked_ctor_arity(view, *type_id, *ctor_id)?;
    if next_index >= arity {
        return Err(oob("prog.ctorFields"));
    }
    let c_row = &view.constructors[ctor_id.0];
    let cf_pos = c_row
        .field_start
        .checked_add(next_index)
        .ok_or_else(|| arith("field_start + next_index"))?;
    let cf_row = view
        .ctor_fields
        .get(cf_pos)
        .ok_or_else(|| oob("prog.ctorFields"))?;
    if cf_row.type_id.0 >= view.types.len() {
        return Err(oob("prog.types"));
    }
    let next_rest_start = rest
        .start
        .checked_add(1)
        .ok_or_else(|| arith("rest.start + 1"))?;
    let next_rest = SliceC::normalized(next_rest_start, rest.len - 1);
    pre.record_adt_env(*saved_env);
    pre.record_adt_slots(*completed);
    pre.record_adt_slots(*seq);
    Ok(ConstructorNextReadC {
        frame: ConstructorFieldsReadC {
            type_id: *type_id,
            ctor_id: *ctor_id,
            index: *index,
            field_type: *field_type,
            rest: *rest,
            saved_env: *saved_env,
            completed: *completed,
            checked: true,
            origin: *origin,
        },
        completed: *seq,
        next_index,
        field_type: cf_row.type_id,
        next,
        rest: next_rest,
    })
}

fn impl_constructor_final<T: AdtPreImageAccess + ?Sized>(
    pre: &T,
    selected: &Selected,
) -> Result<ConstructorFinalReadC, HostViolation> {
    if selected.rule != RuleIdC::XAdtReturnCtorFieldsFinal {
        return Err(wrong("constructor_final rule"));
    }
    if !matches!(pre.control(), ControlC::Return { .. }) {
        return Err(wrong("constructor_final control"));
    }
    let Some(HelperResultC::AppendDone { seq }) = pre.result() else {
        return Err(wrong("constructor_final result"));
    };
    let Some(FrameRowC::ConstructorFieldsF {
        type_id,
        ctor_id,
        index,
        field_type,
        rest,
        saved_env,
        completed,
        checked: true,
        origin,
    }) = pre.top_frame()
    else {
        return Err(wrong("constructor_final top_frame"));
    };
    if rest.len != 0 {
        return Err(wrong("constructor_final non-empty rest"));
    }
    let view = pre.program_view()?;
    let arity = adt_checked_ctor_arity(view, *type_id, *ctor_id)?;
    if seq.len != arity {
        return Err(wrong("constructor_final seq.len != arity"));
    }
    pre.record_adt_env(*saved_env);
    pre.record_adt_slots(*completed);
    pre.record_adt_slots(*seq);
    Ok(ConstructorFinalReadC {
        frame: ConstructorFieldsReadC {
            type_id: *type_id,
            ctor_id: *ctor_id,
            index: *index,
            field_type: *field_type,
            rest: *rest,
            saved_env: *saved_env,
            completed: *completed,
            checked: true,
            origin: *origin,
        },
        completed: *seq,
    })
}

fn impl_constructor_scalar<T: AdtPreImageAccess + ?Sized>(
    pre: &T,
    selected: &Selected,
) -> Result<ConstructorScalarReadC, HostViolation> {
    if selected.rule != RuleIdC::BHelperScalarEntry
        || selected.site != Some(SiteTag::RetCtorFields)
    {
        return Err(wrong("constructor_scalar rule/site"));
    }
    if pre.helper().is_some() || pre.result().is_some() {
        return Err(wrong("constructor_scalar helper/result busy"));
    }
    let ControlC::Return { value } = *pre.control() else {
        return Err(wrong("constructor_scalar control"));
    };
    let Some(FrameRowC::ConstructorFieldsF {
        field_type,
        checked: false,
        ..
    }) = pre.top_frame()
    else {
        return Err(wrong("constructor_scalar frame"));
    };
    pre.record_adt_ref(value);
    Ok(ConstructorScalarReadC {
        common: HelperCommon {
            site: SiteTag::RetCtorFields,
            origin_reference_cursor: pre.reference_cursor(),
            parent_resume: None,
        },
        value,
        expected: *field_type,
    })
}

fn impl_constructor_append<T: AdtPreImageAccess + ?Sized>(
    pre: &T,
    selected: &Selected,
) -> Result<ConstructorAppendReadC, HostViolation> {
    if selected.rule != RuleIdC::BHelperAppendEntry
        || selected.site != Some(SiteTag::RetCtorFields)
    {
        return Err(wrong("constructor_append rule/site"));
    }
    if pre.helper().is_some() || pre.result().is_some() {
        return Err(wrong("constructor_append helper/result busy"));
    }
    let ControlC::Return { value } = *pre.control() else {
        return Err(wrong("constructor_append control"));
    };
    let Some(FrameRowC::ConstructorFieldsF {
        completed,
        checked: true,
        ..
    }) = pre.top_frame()
    else {
        return Err(wrong("constructor_append frame"));
    };
    pre.record_adt_ref(value);
    pre.record_adt_slots(*completed);
    Ok(ConstructorAppendReadC {
        common: HelperCommon {
            site: SiteTag::RetCtorFields,
            origin_reference_cursor: pre.reference_cursor(),
            parent_resume: None,
        },
        value,
        completed: *completed,
        dest_start: pre.active().value_slots,
    })
}

fn impl_application_error<T: AdtPreImageAccess + ?Sized>(
    pre: &T,
    selected: &Selected,
) -> Result<u8, HostViolation> {
    match selected.rule {
        RuleIdC::XAdtReturnCtorFieldsMistyped => {
            if !matches!(pre.control(), ControlC::Return { .. })
                || !matches!(
                    pre.top_frame(),
                    Some(FrameRowC::ConstructorFieldsF { checked: false, .. })
                )
                || !matches!(
                    pre.result(),
                    Some(HelperResultC::ScalarTypeTest { ok: false })
                )
            {
                return Err(wrong("XAdtReturnCtorFieldsMistyped state"));
            }
            Ok(11)
        }
        RuleIdC::XAdtCasesPlanMiss => {
            if !matches!(pre.control(), ControlC::Cases { .. })
                || !matches!(
                    pre.expr(&pre.control_origin())?,
                    ExpressionRowC::Match { plan: Some(_), .. }
                )
                || !matches!(pre.result(), Some(HelperResultC::PlanNoMatch))
            {
                return Err(wrong("XAdtCasesPlanMiss state"));
            }
            Ok(9)
        }
        _ => Err(wrong("application_error rule")),
    }
}

fn impl_materialized_value<T: AdtPreImageAccess + ?Sized>(
    pre: &T,
    selected: &Selected,
) -> Result<RefC, HostViolation> {
    if selected.rule != RuleIdC::XAdtMaterializeDone {
        return Err(wrong("materialized_value rule"));
    }
    if !matches!(pre.control(), ControlC::Materialize { .. }) {
        return Err(wrong("materialized_value control"));
    }
    let Some(HelperResultC::BuildDone { value }) = pre.result() else {
        return Err(wrong("materialized_value result"));
    };
    pre.record_adt_ref(*value);
    Ok(*value)
}

fn impl_plan_hit<T: AdtPreImageAccess + ?Sized>(
    pre: &T,
    selected: &Selected,
) -> Result<PlanHitReadC, HostViolation> {
    if selected.rule != RuleIdC::XAdtCasesPlanHit {
        return Err(wrong("plan_hit rule"));
    }
    let ControlC::Cases { saved_env, .. } = *pre.control() else {
        return Err(wrong("plan_hit control"));
    };
    let origin = pre.control_origin();
    let ExpressionRowC::Match {
        arms,
        plan: Some(_),
        ..
    } = *pre.expr(&origin)?
    else {
        return Err(wrong("plan_hit origin match"));
    };
    let Some(HelperResultC::PlanMatched { arm, env }) = pre.result() else {
        return Err(wrong("plan_hit result"));
    };
    if *arm < arms.start || *arm - arms.start >= arms.len {
        return Err(oob("prog.arms"));
    }
    let arm_row = pre.arm(arm)?;
    pre.record_adt_env(saved_env);
    pre.record_adt_env(*env);
    Ok(PlanHitReadC {
        body: arm_row.body,
        saved_env,
        arm_env: *env,
        origin,
    })
}

fn impl_build_entry<T: AdtPreImageAccess + ?Sized>(
    pre: &T,
    selected: &Selected,
) -> Result<BuildEntryReadC, HostViolation> {
    if selected.rule != RuleIdC::XAdtHelperBuildEntry {
        return Err(wrong("build_entry rule"));
    }
    if pre.helper().is_none() || pre.result().is_some() {
        if pre.result().is_some() {
            return Err(wrong("build_entry result busy"));
        }
    }
    if pre.helper().is_some() {
        return Err(wrong("build_entry helper busy"));
    }
    let ControlC::Materialize {
        type_id,
        ctor_id,
        fields,
    } = *pre.control()
    else {
        return Err(wrong("build_entry control"));
    };
    let arity = adt_checked_ctor_arity(pre.program_view()?, type_id, ctor_id)?;
    if fields.len == 0 || fields.len != arity {
        return Err(wrong("build_entry fields arity"));
    }
    pre.record_adt_slots(fields);
    Ok(BuildEntryReadC {
        common: HelperCommon {
            site: SiteTag::Materialize,
            origin_reference_cursor: pre.reference_cursor(),
            parent_resume: None,
        },
        type_id,
        ctor_id,
        src: fields,
        field_start: pre.active().data_fields,
    })
}

fn impl_build_cell<T: AdtPreImageAccess + ?Sized>(
    pre: &T,
    selected: &Selected,
) -> Result<BuildCellReadC, HostViolation> {
    if selected.rule != RuleIdC::XAdtHelperBuildCell {
        return Err(wrong("build_cell rule"));
    }
    let Some(HelperControlC::Build {
        common,
        type_id,
        ctor_id,
        src,
        field_start,
        src_cursor,
        staged: None,
        node: None,
        phase: BuildPhase::Copy,
    }) = pre.helper()
    else {
        return Err(wrong("build_cell helper"));
    };
    if *src_cursor >= src.len {
        return Err(wrong("build_cell cursor"));
    }
    let pos = src
        .start
        .checked_add(*src_cursor)
        .ok_or_else(|| arith("build_cell pos"))?;
    let value = pre.value_slot(&pos)?;
    let state = BuildReadC {
        common: *common,
        type_id: *type_id,
        ctor_id: *ctor_id,
        src: *src,
        field_start: *field_start,
        src_cursor: *src_cursor,
        staged: None,
        node: None,
        phase: BuildPhase::Copy,
    };
    record_build_state(pre, &state);
    pre.record_adt_ref(value);
    Ok(BuildCellReadC { state, value })
}

fn impl_build_field<T: AdtPreImageAccess + ?Sized>(
    pre: &T,
    selected: &Selected,
) -> Result<BuildFieldReadC, HostViolation> {
    if selected.rule != RuleIdC::XAdtHelperBuildField {
        return Err(wrong("build_field rule"));
    }
    let Some(HelperControlC::Build {
        common,
        type_id,
        ctor_id,
        src,
        field_start,
        src_cursor,
        staged: Some(staged_ref),
        node: None,
        phase: BuildPhase::Copy,
    }) = pre.helper()
    else {
        return Err(wrong("build_field helper"));
    };
    if *src_cursor >= src.len {
        return Err(wrong("build_field cursor"));
    }
    let next_cursor = src_cursor
        .checked_add(1)
        .ok_or_else(|| arith("build_field next_cursor"))?;
    let fresh_node = RefC::Data(pre.active().data);
    let state = BuildReadC {
        common: *common,
        type_id: *type_id,
        ctor_id: *ctor_id,
        src: *src,
        field_start: *field_start,
        src_cursor: *src_cursor,
        staged: Some(*staged_ref),
        node: None,
        phase: BuildPhase::Copy,
    };
    record_build_state(pre, &state);
    Ok(BuildFieldReadC {
        state,
        staged: *staged_ref,
        next_cursor,
        fresh_node,
    })
}

fn impl_build_done<T: AdtPreImageAccess + ?Sized>(
    pre: &T,
    selected: &Selected,
) -> Result<RefC, HostViolation> {
    if selected.rule != RuleIdC::XAdtHelperBuildExit {
        return Err(wrong("build_done rule"));
    }
    let Some(HelperControlC::Build {
        node: Some(value),
        phase: BuildPhase::Done,
        ..
    }) = pre.helper()
    else {
        return Err(wrong("build_done helper"));
    };
    pre.record_adt_ref(*value);
    Ok(*value)
}

fn impl_plan_entry<T: AdtPreImageAccess + ?Sized>(
    pre: &T,
    selected: &Selected,
) -> Result<PlanEntryReadC, HostViolation> {
    if selected.rule != RuleIdC::XAdtHelperPlanEntry {
        return Err(wrong("plan_entry rule"));
    }
    if pre.helper().is_some() || pre.result().is_some() {
        return Err(wrong("plan_entry helper/result busy"));
    }
    let ControlC::Cases {
        value, saved_env, ..
    } = *pre.control()
    else {
        return Err(wrong("plan_entry control"));
    };
    let source_match = pre.control_origin();
    let ExpressionRowC::Match {
        plan: Some(root), ..
    } = *pre.expr(&source_match)?
    else {
        return Err(wrong("plan_entry match expr"));
    };
    let row = impl_plan_node(pre, selected, &root)?;
    pre.record_adt_ref(value);
    pre.record_adt_env(saved_env);
    Ok(PlanEntryReadC {
        common: HelperCommon {
            site: SiteTag::Cases,
            origin_reference_cursor: pre.reference_cursor(),
            parent_resume: None,
        },
        source_match,
        scrutinee: value,
        saved_env,
        root,
        row,
    })
}

fn impl_plan_node<T: AdtPreImageAccess + ?Sized>(
    pre: &T,
    _selected: &Selected,
    id: &usize,
) -> Result<PlanNodeC, HostViolation> {
    let view = pre.program_view()?;
    let row = *view
        .plan_nodes
        .get(*id)
        .ok_or_else(|| oob("prog.planNodes"))?;
    let expected_match = match pre.helper() {
        Some(HelperControlC::Plan { source_match, .. }) => *source_match,
        None if matches!(pre.control(), ControlC::Cases { .. }) => pre.control_origin(),
        _ => return Err(wrong("plan_node context")),
    };
    if plan_node_source_match(&row) != expected_match {
        return Err(wrong("plan_node source_match mismatch"));
    }
    let ExpressionRowC::Match {
        arms,
        plan: Some(root),
        ..
    } = *pre.expr(&expected_match)?
    else {
        return Err(wrong("plan_node match row"));
    };
    if *id < root {
        return Err(wrong("plan_node before root"));
    }
    match row {
        PlanNodeC::Switch {
            path,
            type_id,
            cases,
            default,
            ..
        } => {
            let p_end = path
                .start
                .checked_add(path.len)
                .ok_or_else(|| arith("path slice"))?;
            if p_end > view.field_paths.len() {
                return Err(oob("prog.fieldPaths"));
            }
            let t_row = view.types.get(type_id.0).ok_or_else(|| oob("prog.types"))?;
            let c_end = cases
                .start
                .checked_add(cases.len)
                .ok_or_else(|| arith("cases slice"))?;
            if c_end > view.plan_cases.len() || cases.len != t_row.ctor_len {
                return Err(oob("prog.planCases"));
            }
            if default >= view.plan_nodes.len() {
                return Err(oob("prog.planNodes"));
            }
        }
        PlanNodeC::Bind {
            path, binder, next, ..
        } => {
            let p_end = path
                .start
                .checked_add(path.len)
                .ok_or_else(|| arith("path slice"))?;
            if p_end > view.field_paths.len() {
                return Err(oob("prog.fieldPaths"));
            }
            if binder >= view.names.len() {
                return Err(oob("prog.names"));
            }
            if next >= view.plan_nodes.len() {
                return Err(oob("prog.planNodes"));
            }
        }
        PlanNodeC::Select { arm, .. } => {
            if arm >= view.arms.len() || arm < arms.start || arm - arms.start >= arms.len {
                return Err(oob("prog.arms"));
            }
        }
        PlanNodeC::NoMatch { .. } => {}
    }
    Ok(row)
}

fn impl_plan_walk<T: AdtPreImageAccess + ?Sized>(
    pre: &T,
    selected: &Selected,
) -> Result<PlanWalkReadC, HostViolation> {
    if selected.rule != RuleIdC::XAdtHelperPlanWalk {
        return Err(wrong("plan_walk rule"));
    }
    let state = extract_plan_state(pre)?;
    if state.phase != PlanPhase::Walk || state.path_cursor >= state.path.len {
        return Err(wrong("plan_walk phase/cursor"));
    }
    let view = pre.program_view()?;
    let fp_pos = state
        .path
        .start
        .checked_add(state.path_cursor)
        .ok_or_else(|| arith("plan_walk fp_pos"))?;
    let field_idx = *view
        .field_paths
        .get(fp_pos)
        .ok_or_else(|| oob("prog.fieldPaths"))?;
    let value = pre.field_at(&state.cur, &field_idx)?;
    pre.record_adt_ref(value);
    let next_cursor = state
        .path_cursor
        .checked_add(1)
        .ok_or_else(|| arith("plan_walk next_cursor"))?;
    let next_phase = if next_cursor == state.path.len {
        match state.after_walk {
            PlanAfterWalk::Test => PlanPhase::Test,
            PlanAfterWalk::Bind => PlanPhase::Bind,
        }
    } else {
        PlanPhase::Walk
    };
    Ok(PlanWalkReadC {
        state,
        value,
        next_cursor,
        next_phase,
    })
}

fn impl_plan_switch<T: AdtPreImageAccess + ?Sized>(
    pre: &T,
    selected: &Selected,
) -> Result<PlanSwitchReadC, HostViolation> {
    if selected.rule != RuleIdC::XAdtHelperPlanSwitch {
        return Err(wrong("plan_switch rule"));
    }
    let state = extract_plan_state(pre)?;
    if state.phase != PlanPhase::Test
        || state.after_walk != PlanAfterWalk::Test
        || state.path_cursor != state.path.len
    {
        return Err(wrong("plan_switch phase"));
    }
    let PlanNodeC::Switch {
        type_id,
        cases,
        default,
        ..
    } = impl_plan_node(pre, selected, &state.node)?
    else {
        return Err(wrong("plan_switch node"));
    };
    let view = pre.program_view()?;
    let t_row = view.types.get(type_id.0).ok_or_else(|| oob("prog.types"))?;
    if cases.len != t_row.ctor_len {
        return Err(oob("prog.planCases"));
    }
    if default >= view.plan_nodes.len() {
        return Err(oob("prog.planNodes"));
    }
    let k = pre.kind_of(&state.cur)?;
    let case = match kind_ctor_class(view, k)? {
        Some((kt, kc)) if kt == type_id => {
            if kc.0 < t_row.ctor_start.0 || kc.0 - t_row.ctor_start.0 >= t_row.ctor_len {
                return Err(oob("prog.constructors"));
            }
            let offset = kc.0 - t_row.ctor_start.0;
            let pos = cases
                .start
                .checked_add(offset)
                .ok_or_else(|| arith("cases.start + offset"))?;
            let target = *view
                .plan_cases
                .get(pos)
                .ok_or_else(|| oob("prog.planCases"))?;
            if let Some(t) = target {
                if t >= view.plan_nodes.len() {
                    return Err(oob("prog.planNodes"));
                }
            }
            target
        }
        _ => None,
    };
    Ok(PlanSwitchReadC {
        state,
        case,
        default,
    })
}

fn impl_plan_bind<T: AdtPreImageAccess + ?Sized>(
    pre: &T,
    selected: &Selected,
) -> Result<PlanBindReadC, HostViolation> {
    if selected.rule != RuleIdC::XAdtHelperPlanBind {
        return Err(wrong("plan_bind rule"));
    }
    let state = extract_plan_state(pre)?;
    if state.phase != PlanPhase::Bind
        || state.after_walk != PlanAfterWalk::Bind
        || state.path_cursor != state.path.len
    {
        return Err(wrong("plan_bind phase"));
    }
    let PlanNodeC::Bind { binder, next, .. } = impl_plan_node(pre, selected, &state.node)? else {
        return Err(wrong("plan_bind node"));
    };
    Ok(PlanBindReadC {
        state,
        binder,
        next,
        fresh_env: pre.active().env,
    })
}

fn impl_plan_state<T: AdtPreImageAccess + ?Sized>(
    pre: &T,
    selected: &Selected,
) -> Result<PlanReadC, HostViolation> {
    let state = extract_plan_state(pre)?;
    match selected.rule {
        RuleIdC::XAdtHelperPlanSelect => {
            if state.phase != PlanPhase::Ready
                || state.status != TriStatus::Pending
                || state.arm.is_none()
            {
                return Err(wrong("plan_state select phase"));
            }
            let PlanNodeC::Select { arm, .. } = impl_plan_node(pre, selected, &state.node)? else {
                return Err(wrong("plan_state select node"));
            };
            if state.arm != Some(arm) {
                return Err(wrong("plan_state select arm"));
            }
            Ok(state)
        }
        RuleIdC::XAdtHelperPlanNoMatch => {
            if state.phase != PlanPhase::NoMatchReady
                || state.status != TriStatus::Pending
                || state.arm.is_some()
            {
                return Err(wrong("plan_state nomatch phase"));
            }
            let PlanNodeC::NoMatch { .. } = impl_plan_node(pre, selected, &state.node)? else {
                return Err(wrong("plan_state nomatch node"));
            };
            Ok(state)
        }
        _ => Err(wrong("plan_state rule")),
    }
}

fn impl_plan_matched<T: AdtPreImageAccess + ?Sized>(
    pre: &T,
    selected: &Selected,
) -> Result<(usize, EnvRootC), HostViolation> {
    if selected.rule != RuleIdC::XAdtHelperPlanExit {
        return Err(wrong("plan_matched rule"));
    }
    let state = extract_plan_state(pre)?;
    if state.phase != PlanPhase::Done || state.status != TriStatus::Positive {
        return Err(wrong("plan_matched phase/status"));
    }
    let arm = state.arm.ok_or_else(|| wrong("plan_matched arm"))?;
    Ok((arm, state.partial))
}

macro_rules! impl_adt_runtime_reads {
    ($ty:ty) => {
        impl<'a> AdtRuntimeReads for $ty {
            fn foreign_rule(&self, _selected: &Selected) -> HostViolation {
                impl_foreign_rule()
            }
            fn nullary_construct(&self, selected: &Selected) -> Result<RefC, HostViolation> {
                impl_nullary_construct(self, selected)
            }
            fn construct_fields(
                &self,
                selected: &Selected,
            ) -> Result<ConstructFieldsReadC, HostViolation> {
                impl_construct_fields(self, selected)
            }
            fn constructor_frame(
                &self,
                selected: &Selected,
            ) -> Result<ConstructorFieldsReadC, HostViolation> {
                impl_constructor_frame(self, selected)
            }
            fn constructor_next(
                &self,
                selected: &Selected,
            ) -> Result<ConstructorNextReadC, HostViolation> {
                impl_constructor_next(self, selected)
            }
            fn constructor_final(
                &self,
                selected: &Selected,
            ) -> Result<ConstructorFinalReadC, HostViolation> {
                impl_constructor_final(self, selected)
            }
            fn constructor_scalar(
                &self,
                selected: &Selected,
            ) -> Result<ConstructorScalarReadC, HostViolation> {
                impl_constructor_scalar(self, selected)
            }
            fn constructor_append(
                &self,
                selected: &Selected,
            ) -> Result<ConstructorAppendReadC, HostViolation> {
                impl_constructor_append(self, selected)
            }
            fn application_error(&self, selected: &Selected) -> Result<u8, HostViolation> {
                impl_application_error(self, selected)
            }
            fn materialized_value(&self, selected: &Selected) -> Result<RefC, HostViolation> {
                impl_materialized_value(self, selected)
            }
            fn plan_hit(&self, selected: &Selected) -> Result<PlanHitReadC, HostViolation> {
                impl_plan_hit(self, selected)
            }
            fn build_entry(&self, selected: &Selected) -> Result<BuildEntryReadC, HostViolation> {
                impl_build_entry(self, selected)
            }
            fn build_cell(&self, selected: &Selected) -> Result<BuildCellReadC, HostViolation> {
                impl_build_cell(self, selected)
            }
            fn build_field(&self, selected: &Selected) -> Result<BuildFieldReadC, HostViolation> {
                impl_build_field(self, selected)
            }
            fn build_done(&self, selected: &Selected) -> Result<RefC, HostViolation> {
                impl_build_done(self, selected)
            }
            fn plan_entry(&self, selected: &Selected) -> Result<PlanEntryReadC, HostViolation> {
                impl_plan_entry(self, selected)
            }
            fn plan_walk(&self, selected: &Selected) -> Result<PlanWalkReadC, HostViolation> {
                impl_plan_walk(self, selected)
            }
            fn plan_switch(&self, selected: &Selected) -> Result<PlanSwitchReadC, HostViolation> {
                impl_plan_switch(self, selected)
            }
            fn plan_bind(&self, selected: &Selected) -> Result<PlanBindReadC, HostViolation> {
                impl_plan_bind(self, selected)
            }
            fn plan_state(&self, selected: &Selected) -> Result<PlanReadC, HostViolation> {
                impl_plan_state(self, selected)
            }
            fn plan_node(&self, selected: &Selected, id: &usize) -> Result<PlanNodeC, HostViolation> {
                impl_plan_node(self, selected, id)
            }
            fn plan_matched(&self, selected: &Selected) -> Result<(usize, EnvRootC), HostViolation> {
                impl_plan_matched(self, selected)
            }
        }
    };
}

impl_adt_runtime_reads!(PreImage<'a>);
impl_adt_runtime_reads!(RecordingPreImageView<'a>);

// ================= StepInv & Denotational Closure Helpers for Generic ADTs =================

pub(super) fn adt_dedicated_domain_clean(img: &MachineImage) -> bool {
        let v = match &img.admission.body.view {
            Some(view) => view,
            None => return false,
        };
    if !v.pattern_children.is_empty()
        || !v.types.is_empty()
        || !v.type_args.is_empty()
        || !v.constructors.is_empty()
        || !v.ctor_fields.is_empty()
        || !v.decl_param_counts.is_empty()
        || !v.decl_ctor_counts.is_empty()
        || !v.decl_templates.is_empty()
        || !v.plan_nodes.is_empty()
        || !v.plan_cases.is_empty()
        || !v.field_paths.is_empty()
    {
        return false;
    }
    if v.constants
        .iter()
        .any(|c| matches!(c, RawConstRow::Constructor { .. }))
    {
        return false;
    }
    if v.expressions.iter().any(|e| {
        matches!(
            e,
            ExpressionRowC::Construct { .. } | ExpressionRowC::Match { plan: Some(_), .. }
        )
    }) {
        return false;
    }
    if v.patterns
        .iter()
        .any(|p| matches!(p, PatternRowC::Wildcard | PatternRowC::Constructor { .. }))
    {
        return false;
    }
    if img
        .data
        .iter()
        .any(|d| matches!(d, RawDataRow::Constructor { .. }))
    {
        return false;
    }
    if img
        .frames
        .iter()
        .any(|f| matches!(f, FrameRowC::ConstructorFieldsF { .. }))
    {
        return false;
    }
    if matches!(img.control, ControlC::Materialize { .. }) {
        return false;
    }
    if matches!(
        img.helper,
        Some(HelperControlC::Build { .. } | HelperControlC::Plan { .. })
    ) {
        return false;
    }
    if matches!(
        img.result,
        Some(
            HelperResultC::BuildDone { .. }
                | HelperResultC::PlanMatched { .. }
                | HelperResultC::PlanNoMatch
        )
    ) {
        return false;
    }
    true
}

pub(super) fn adt_si11_ctor_row_ok(
    pre: &PreImage,
    row_idx: usize,
    type_id: RawTypeId,
    ctor_id: RawCtorId,
    fields: &super::super::super::raw::FieldSlice,
) -> Result<bool, HostViolation> {
    if pre.admitted.body.identity.grammar_domain
        != GrammarDomain::GenericNominalExtensionReserved
    {
        return Ok(false);
    }
    let view = pre.program_view()?;
    let tid = TypeIdC(type_id.0);
    let cid = CtorIdC(ctor_id.0);
    let Ok(arity) = adt_checked_ctor_arity(view, tid, cid) else {
        return Ok(false);
    };
    if fields.len == 0 || fields.len != arity {
        return Ok(false);
    }
    let Some(end) = fields.start.0.checked_add(fields.len) else {
        return Ok(false);
    };
    if end > pre.img.data_fields.len() {
        return Ok(false);
    }
    let c_row = &view.constructors[cid.0];
    for k in 0..fields.len {
        let r = pre.img.data_fields[fields.start.0 + k];
        if let RefC::Data(d) = r {
            if d >= row_idx {
                return Ok(false);
            }
        }
        let want = view.ctor_fields[c_row.field_start + k].type_id;
        if !pre.scalar_test_type_ok(&r, &want)? {
            return Ok(false);
        }
    }
    Ok(true)
}

pub(super) fn adt_si15_ctor_frame_ok(
    pre: &PreImage,
    f: &FrameRowC,
    is_top: bool,
) -> Result<bool, HostViolation> {
    let FrameRowC::ConstructorFieldsF {
        type_id,
        ctor_id,
        index,
        field_type,
        rest,
        saved_env,
        completed,
        checked,
        origin,
    } = f
    else {
        return Ok(false);
    };
    if pre.admitted.body.identity.grammar_domain
        != GrammarDomain::GenericNominalExtensionReserved
    {
        return Ok(false);
    }
    let view = pre.program_view()?;
    let Ok(ExpressionRowC::Construct {
        type_id: et,
        ctor_id: ec,
        fields: ef,
        constant: None,
    }) = pre.expr(origin)
    else {
        return Ok(false);
    };
    if et != type_id || ec != ctor_id || ef.len == 0 {
        return Ok(false);
    }
    let Ok(arity) = adt_checked_ctor_arity(view, *type_id, *ctor_id) else {
        return Ok(false);
    };
    if arity != ef.len || *index >= ef.len {
        return Ok(false);
    }
    let c_row = &view.constructors[ctor_id.0];
    if view.ctor_fields[c_row.field_start + *index].type_id != *field_type {
        return Ok(false);
    }
    let expected_rest = SliceC::normalized(ef.start + *index + 1, ef.len - 1 - *index);
    if *rest != expected_rest {
        return Ok(false);
    }
    if let Some(eid) = saved_env {
        if *eid >= pre.img.env.len() {
            return Ok(false);
        }
    }
    if *checked && !is_top {
        return Ok(false);
    }
    if completed.len != *index {
        return Ok(false);
    }
    let Some(c_end) = completed.start.checked_add(completed.len) else {
        return Ok(false);
    };
    if c_end > pre.img.value_slots.len() {
        return Ok(false);
    }
    for k in 0..completed.len {
        let r = pre.img.value_slots[completed.start + k];
        let want = view.ctor_fields[c_row.field_start + k].type_id;
        if !pre.scalar_test_type_ok(&r, &want)? {
            return Ok(false);
        }
    }
    Ok(true)
}

pub(super) fn adt_si16_materialize_ok(
    pre: &PreImage,
    type_id: TypeIdC,
    ctor_id: CtorIdC,
    fields: SliceC,
) -> Result<bool, HostViolation> {
    if pre.admitted.body.identity.grammar_domain
        != GrammarDomain::GenericNominalExtensionReserved
    {
        return Ok(false);
    }
    let view = pre.program_view()?;
    let Ok(ExpressionRowC::Construct {
        type_id: et,
        ctor_id: ec,
        fields: ef,
        constant: None,
    }) = pre.expr(&pre.img.control_origin)
    else {
        return Ok(false);
    };
    if *et != type_id || *ec != ctor_id || ef.len == 0 || ef.len != fields.len {
        return Ok(false);
    }
    let Ok(arity) = adt_checked_ctor_arity(view, type_id, ctor_id) else {
        return Ok(false);
    };
    if arity != fields.len {
        return Ok(false);
    }
    let Some(f_end) = fields.start.checked_add(fields.len) else {
        return Ok(false);
    };
    if f_end > pre.img.value_slots.len() {
        return Ok(false);
    }
    let c_row = &view.constructors[ctor_id.0];
    for k in 0..fields.len {
        let r = pre.img.value_slots[fields.start + k];
        let want = view.ctor_fields[c_row.field_start + k].type_id;
        if !pre.scalar_test_type_ok(&r, &want)? {
            return Ok(false);
        }
    }
    Ok(true)
}

fn env_chain_bindings(
    pre: &PreImage,
    head: EnvRootC,
    base: EnvRootC,
) -> Result<Option<Vec<(usize, RefC)>>, HostViolation> {
    let mut cells: Vec<(usize, RefC)> = Vec::new();
    let mut cur = head;
    while cur != base {
        let Some(id) = cur else {
            return Ok(None);
        };
        if cells.len() >= pre.active().env {
            return Ok(None);
        }
        let c = pre.env_cell(&id)?;
        cells.push((c.binder, c.value));
        cur = c.previous;
    }
    cells.reverse();
    Ok(Some(cells))
}

fn walk_path_from(
    pre: &PreImage,
    start_val: RefC,
    path: SliceC,
    steps: usize,
) -> Result<Option<RefC>, HostViolation> {
    let view = pre.program_view()?;
    let Some(end) = path.start.checked_add(path.len) else {
        return Ok(None);
    };
    if end > view.field_paths.len() || steps > path.len {
        return Ok(None);
    }
    let mut cur = start_val;
    for i in 0..steps {
        let fidx = view.field_paths[path.start + i];
        let k = pre.kind_of(&cur)?;
        let arity = pre.checked_arity(&k)?;
        if fidx >= arity {
            return Ok(None);
        }
        cur = pre.field_at(&cur, &fidx)?;
    }
    Ok(Some(cur))
}

enum EvalPlanOutcome {
    Matched {
        arm: usize,
        bindings: Vec<(usize, RefC)>,
    },
    NoMatch,
}

fn evaluate_plan_dag(
    pre: &PreImage,
    source_match: usize,
    root: usize,
    scrutinee: RefC,
) -> Result< Option<EvalPlanOutcome>, HostViolation> {
    let view = pre.program_view()?;
    let mut node_id = root;
    let mut bindings: Vec<(usize, RefC)> = Vec::new();
    let mut budget = view.plan_nodes.len().saturating_add(1);
    while budget > 0 {
        budget -= 1;
        let Some(row) = view.plan_nodes.get(node_id).copied() else {
            return Ok(None);
        };
        if plan_node_source_match(&row) != source_match || node_id < root {
            return Ok(None);
        }
        match row {
            PlanNodeC::Switch {
                path,
                type_id,
                cases,
                default,
                ..
            } => {
                let Some(cur) = walk_path_from(pre, scrutinee, path, path.len)? else {
                    return Ok(None);
                };
                let Some(t_row) = view.types.get(type_id.0) else {
                    return Ok(None);
                };
                if cases.len != t_row.ctor_len {
                    return Ok(None);
                }
                let k = pre.kind_of(&cur)?;
                let case = match kind_ctor_class(view, k)? {
                    Some((kt, kc)) if kt == type_id => {
                        if kc.0 < t_row.ctor_start.0
                            || kc.0 - t_row.ctor_start.0 >= t_row.ctor_len
                        {
                            return Ok(None);
                        }
                        let pos = cases.start + (kc.0 - t_row.ctor_start.0);
                        view.plan_cases.get(pos).copied().flatten()
                    }
                    _ => None,
                };
                let next = case.unwrap_or(default);
                if next <= node_id {
                    return Ok(None);
                }
                node_id = next;
            }
            PlanNodeC::Bind {
                path, binder, next, ..
            } => {
                let Some(cur) = walk_path_from(pre, scrutinee, path, path.len)? else {
                    return Ok(None);
                };
                bindings.push((binder, cur));
                if next <= node_id {
                    return Ok(None);
                }
                node_id = next;
            }
            PlanNodeC::Select { arm, .. } => {
                return Ok(Some(EvalPlanOutcome::Matched { arm, bindings }));
            }
            PlanNodeC::NoMatch { .. } => {
                return Ok(Some(EvalPlanOutcome::NoMatch));
            }
        }
    }
    Ok(None)
}

pub(super) fn si21_adt_helper_ok(
    pre: &PreImage,
    h: &HelperControlC,
) -> Result<bool, HostViolation> {
    if pre.admitted.body.identity.grammar_domain
        != GrammarDomain::GenericNominalExtensionReserved
    {
        return Ok(false);
    }
    let view = pre.program_view()?;
    match h {
        HelperControlC::Build {
            common,
            type_id,
            ctor_id,
            src,
            field_start,
            src_cursor,
            staged,
            node,
            phase,
        } => {
            if common.site != SiteTag::Materialize || common.parent_resume.is_some() {
                return Ok(false);
            }
            let ControlC::Materialize {
                type_id: ct,
                ctor_id: cc,
                fields: cf,
            } = pre.img.control
            else {
                return Ok(false);
            };
            if *type_id != ct || *ctor_id != cc || *src != cf {
                return Ok(false);
            }
            let Ok(arity) = adt_checked_ctor_arity(view, *type_id, *ctor_id) else {
                return Ok(false);
            };
            if src.len == 0 || src.len != arity {
                return Ok(false);
            }
            match phase {
                BuildPhase::Copy
 => {
                    if *src_cursor >= src.len || node.is_some() {
                        return Ok(false);
                    }
                    if field_start.checked_add(*src_cursor) != Some(pre.active().data_fields) {
                        return Ok(false);
                    }
                    let Some(s_end) = src.start.checked_add(src.len) else {
                        return Ok(false);
                    };
                    if s_end > pre.active().value_slots {
                        return Ok(false);
                    }
                    let c_row = &view.constructors[ctor_id.0];
                    for k in 0..src.len {
                        let r = pre.value_slot(&(src.start + k))?;
                        let want = view.ctor_fields[c_row.field_start + k].type_id;
                        if !pre.scalar_test_type_ok(&r, &want)? {
                            return Ok(false);
                        }
                        if k < *src_cursor && pre.img.data_fields[*field_start + k] != r {
                            return Ok(false);
                        }
                    }
                    if let Some(st) = staged {
                        let cur_slot = pre.value_slot(&(src.start + *src_cursor))?;
                        if *st != cur_slot {
                            return Ok(false);
                        }
                    }
                    Ok(true)
                }
                BuildPhase::Done => {
                    if *src_cursor != src.len || staged.is_some() {
                        return Ok(false);
                    }
                    let Some(RefC::Data(nid)) = *node else {
                        return Ok(false);
                    };
                    if nid.checked_add(1) != Some(pre.active().data) {
                        return Ok(false);
                    }
                    if field_start.checked_add(src.len) != Some(pre.active().data_fields) {
                        return Ok(false);
                    }
                    let Some(RawDataRow::Constructor {
                        type_id: dt,
                        ctor_id: dc,
                        fields: df,
                    }) = pre.img.data.get(nid)
                    else {
                        return Ok(false);
                    };
                    if dt.0 != type_id.0
                        || dc.0 != ctor_id.0
                        || df.start.0 != *field_start
                        || df.len != src.len
                    {
                        return Ok(false);
                    }
                    for k in 0..src.len {
                        let r = pre.value_slot(&(src.start + k))?;
                        if pre.img.data_fields[*field_start + k] != r {
                            return Ok(false);
                        }
                    }
                    Ok(true)
                }
            }
        }
        HelperControlC::Plan {
            common,
            source_match,
            node,
            scrutinee,
            cur,
            path,
            path_cursor,
            after_walk,
            saved_env,
            partial,
            phase,
            status,
            arm,
        } => {
            if common.site != SiteTag::Cases || common.parent_resume.is_some() {
                return Ok(false);
            }
            let ControlC::Cases {
                value: cv,
                saved_env: cse,
                ..
            } = pre.img.control
            else {
                return Ok(false);
            };
            if *source_match != pre.img.control_origin
                || *scrutinee != cv
                || *saved_env != cse
            {
                return Ok(false);
            }
            let Ok(ExpressionRowC::Match {
                arms,
                plan: Some(root),
                ..
            }) = pre.expr(source_match)
            else {
                return Ok(false);
            };
            if *node < *root || *node >= view.plan_nodes.len() {
                return Ok(false);
            }
            let node_row = view.plan_nodes[*node];
            if plan_node_source_match(&node_row) != *source_match {
                return Ok(false);
            }
            if env_chain_bindings(pre, *partial, *saved_env)?.is_none() {
                return Ok(false);
            }
            match node_row {
                PlanNodeC::Switch { path: np, .. } => {
                    if *path != np || *after_walk != PlanAfterWalk::Test || *path_cursor > path.len
                    {
                        return Ok(false);
                    }
                    let Some(expected_cur) =
                        walk_path_from(pre, *scrutinee, *path, *path_cursor)?
                    else {
                        return Ok(false);
                    };
                    if *cur != expected_cur {
                        return Ok(false);
                    }
                    if *path_cursor < path.len {
                        Ok(*phase == PlanPhase::Walk
                            && *status == TriStatus::Pending
                            && arm.is_none())
                    } else {
                        Ok((*phase == PlanPhase::Test
                            && *status == TriStatus::Pending
                            && arm.is_none())
                            || (*phase == PlanPhase::Done
                                && *status == TriStatus::Negative
                                && arm.is_none()))
                    }
                }
                PlanNodeC::Bind { path: np, .. } => {
                    if *path != np || *after_walk != PlanAfterWalk::Bind || *path_cursor > path.len
                    {
                        return Ok(false);
                    }
                    let Some(expected_cur) =
                        walk_path_from(pre, *scrutinee, *path, *path_cursor)?
                    else {
                        return Ok(false);
                    };
                    if *cur != expected_cur {
                        return Ok(false);
                    }
                    if *path_cursor < path.len {
                        Ok(*phase == PlanPhase::Walk
                            && *status == TriStatus::Pending
                            && arm.is_none())
                    } else {
                        Ok(*phase == PlanPhase::Bind
                            && *status == TriStatus::Pending
                            && arm.is_none())
                    }
                }
                PlanNodeC::Select { arm: na, .. } => {
                    if path.len != 0
                        || *path_cursor != 0
                        || *cur != *scrutinee
                        || *arm != Some(na)
                        || na < arms.start
                        || na - arms.start >= arms.len
                    {
                        return Ok(false);
                    }
                    Ok((*phase == PlanPhase::Ready && *status == TriStatus::Pending)
                        || (*phase == PlanPhase::Done && *status == TriStatus::Positive))
                }
                PlanNodeC::NoMatch { .. } => {
                    if path.len != 0 || *path_cursor != 0 || *cur != *scrutinee || arm.is_some() {
                        return Ok(false);
                    }
                    Ok((*phase == PlanPhase::NoMatchReady && *status == TriStatus::Pending)
                        || (*phase == PlanPhase::Done && *status == TriStatus::Negative))
                }
            }
        }
        _ => Ok(false),
    }
}

pub(super) fn si21_adt_result_ok(
    pre: &PreImage,
    r: &HelperResultC,
) -> Result<bool, HostViolation> {
    if pre.admitted.body.identity.grammar_domain
        != GrammarDomain::GenericNominalExtensionReserved
    {
        return Ok(false);
    }
    match r {
        HelperResultC::BuildDone { value } => {
            let ControlC::Materialize {
                type_id,
                ctor_id,
                fields,
            } = pre.img.control
            else {
                return Ok(false);
            };
            let RefC::Data(nid) = *value else {
                return Ok(false);
            };
            if nid.checked_add(1) != Some(pre.active().data) {
                return Ok(false);
            }
            let Some(RawDataRow::Constructor {
                type_id: dt,
                ctor_id: dc,
                fields: df,
            }) = pre.img.data.get(nid)
            else {
                return Ok(false);
            };
            if dt.0 != type_id.0 || dc.0 != ctor_id.0 || df.len != fields.len {
                return Ok(false);
            }
            if df.start.0.checked_add(df.len) != Some(pre.active().data_fields) {
                return Ok(false);
            }
            for k in 0..fields.len {
                let slot_val = pre.value_slot(&(fields.start + k))?;
                if pre.img.data_fields[df.start.0 + k] != slot_val {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        HelperResultC::PlanMatched { arm, env } => {
            let ControlC::Cases {
                value, saved_env, ..
            } = pre.img.control
            else {
                return Ok(false);
            };
            let source_match = pre.img.control_origin;
            let Ok(ExpressionRowC::Match {
                arms,
                plan: Some(root),
                ..
            }) = pre.expr(&source_match)
            else {
                return Ok(false);
            };
            if *arm < arms.start || *arm - arms.start >= arms.len {
                return Ok(false);
            }
            let Some(actual_bindings) = env_chain_bindings(pre, *env, saved_env)? else {
                return Ok(false);
            };
            let Some(EvalPlanOutcome::Matched {
                arm: expected_arm,
                bindings: expected_bindings,
            }) = evaluate_plan_dag(pre, source_match, *root, value)?
            else {
                return Ok(false);
            };
            Ok(expected_arm == *arm && actual_bindings == expected_bindings)
        }
        HelperResultC::PlanNoMatch => {
            let ControlC::Cases { value, .. } = pre.img.control else {
                return Ok(false);
            };
            let source_match = pre.img.control_origin;
            let Ok(ExpressionRowC::Match {
                plan: Some(root), ..
            }) = pre.expr(&source_match)
            else {
                return Ok(false);
            };
            let Some(EvalPlanOutcome::NoMatch) =
                evaluate_plan_dag(pre, source_match, *root, value)?
            else {
                return Ok(false);
            };
            Ok(true)
        }
        _ => Ok(false),
    }
}

pub(super) fn si22_adt_helper_ok(
    pre: &PreImage,
    h: &HelperControlC,
) -> Result<bool, HostViolation> {
    if pre.task_live() != 0 {
        return Ok(false);
    }
    si21_adt_helper_ok(pre, h)
}

pub(super) fn si23_adt_collect_live_slices(img: &MachineImage, out: &mut Vec<SliceC>) {
    for f in &img.frames {
        if let FrameRowC::ConstructorFieldsF { completed, .. } = f {
            if completed.len > 0 {
                out.push(*completed);
            }
        }
    }
    if let ControlC::Materialize { fields, .. } = img.control {
        if fields.len > 0 {
            out.push(fields);
        }
    }
    if let Some(HelperControlC::Build { src, .. }) = &img.helper {
        if src.len > 0 {
            out.push(*src);
        }
    }
}

pub(crate) fn check_generic_program_view(v: &ProgramView) -> Result<(), HostViolation> {
    let si24 = |loc: &str| HostViolation {
        kind: HostViolationKind::StepInvViolated,
        clause: Some(StepInvClauseId::SI24),
        rule: None,
        locator: Some(loc.to_string()),
    };
    if v.expressions.is_empty()
        || v.entry != 0
        || v.occurrences.len() != v.expressions.len()
        || v.types.len() < 4
        || v.constructors.len() < 4
        || v.decl_param_counts.len() != v.decl_ctor_counts.len()
        || v.decl_param_counts.len() != v.decl_templates.len()
    {
        return Err(si24("generic view header/base counts"));
    }
    // Rows 0..3 (UInt, Bool, List, Tree) are fixed by position; their nameId is
    // padding 0 (builtin_rows) and is never resolved against prog.names.
    let expected_builtins = [
        (0usize, 0usize, 0usize),
        (1usize, 0usize, 0usize),
        (2usize, 0usize, 2usize),
        (3usize, 2usize, 2usize),
    ];
    for (ti, cstart, clen) in expected_builtins {
        let trow = &v.types[ti];
        if trow.decl.is_some()
            || trow.args != SliceC::empty()
            || trow.name_id != 0
            || trow.ctor_start.0 != cstart
            || trow.ctor_len != clen
        {
            return Err(si24("builtin type row"));
        }
    }
    for (ti, trow) in v.types.iter().enumerate().skip(4) {
        let Some(decl_id) = trow.decl else {
            return Err(si24("user type decl None"));
        };
        if trow.name_id >= v.names.len()
            || decl_id >= v.decl_param_counts.len()
            || trow.args.len != v.decl_param_counts[decl_id]
            || trow.ctor_len != v.decl_ctor_counts[decl_id]
            || trow.ctor_len == 0
        {
            return Err(si24("user type decl counts"));
        }
        let Some(a_end) = trow.args.start.checked_add(trow.args.len) else {
            return Err(si24("type_args overflow"));
        };
        if a_end > v.type_args.len() {
            return Err(si24("type_args slice"));
        }
        for pos in trow.args.start..a_end {
            if v.type_args[pos].0 >= v.types.len() {
                return Err(si24("type_args target"));
            }
        }
        let Some(c_end) = trow.ctor_start.0.checked_add(trow.ctor_len) else {
            return Err(si24("constructors overflow"));
        };
        if c_end > v.constructors.len() {
            return Err(si24("constructors slice"));
        }
        for (ord, cid) in (trow.ctor_start.0..c_end).enumerate() {
            let crow = &v.constructors[cid];
            if crow.owning_type != TypeIdC(ti)
                || crow.name_id >= v.names.len()
                || crow.source_order != ord
            {
                return Err(si24("constructor owning_type/name/order"));
            }
            let Some(f_end) = crow.field_start.checked_add(crow.field_len) else {
                return Err(si24("ctor_fields overflow"));
            };
            if f_end > v.ctor_fields.len() {
                return Err(si24("ctor_fields slice"));
            }
            for fpos in crow.field_start..f_end {
                let cf = &v.ctor_fields[fpos];
                if cf.name_id >= v.names.len() || cf.type_id.0 >= v.types.len() {
                    return Err(si24("ctor_field name/type_id"));
                }
            }
        }
    }
    for (pi, pat) in v.patterns.iter().enumerate() {
        match pat {
            PatternRowC::PVar { x } | PatternRowC::PLeaf { x } => {
                if *x >= v.names.len() {
                    return Err(si24("pattern binder"));
                }
            }
            PatternRowC::PCons { x, y } | PatternRowC::PBranch { x, y } => {
                if *x >= v.names.len() || *y >= v.names.len() {
                    return Err(si24("pattern binder"));
                }
            }
            PatternRowC::PNil | PatternRowC::Wildcard => {}
            PatternRowC::Constructor {
                type_id,
                ctor_id,
                children,
            } => {
                let arity = match (*type_id, ctor_id.0) {
                    (TypeIdC::List, 0) => 0,
                    (TypeIdC::List, 1) => 2,
                    (TypeIdC::Tree, 2) => 1,
                    (TypeIdC::Tree, 3) => 2,
                    _ => adt_checked_ctor_arity(v, *type_id, *ctor_id)
                        .map_err(|_| si24("pattern ctor arity"))?,
                };
                if children.len != arity {
                    return Err(si24("pattern children len"));
                }
                let Some(pc_end) = children.start.checked_add(children.len) else {
                    return Err(si24("pattern_children overflow"));
                };
                if pc_end > v.pattern_children.len() {
                    return Err(si24("pattern_children slice"));
                }
                for pos in children.start..pc_end {
                    let child_pat = v.pattern_children[pos];
                    if child_pat >= pi {
                        return Err(si24("pattern child order"));
                    }
                }
            }
        }
    }
    let n_expr = v.expressions.len();
    let mut parent: Vec<Option<usize>> = vec![None; n_expr];
    let mut child_pos: Vec<usize> = vec![0; n_expr];
    for (ei, erow) in v.expressions.iter().enumerate() {
        let mut kids: Vec<(usize, usize)> = Vec::new();
        match erow {
            ExpressionRowC::Literal { constant } => {
                if *constant >= v.constants.len() {
                    return Err(si24("literal constant"));
                }
            }
            ExpressionRowC::Var { binder } => {
                if *binder >= v.names.len() {
                    return Err(si24("var binder"));
                }
            }
            ExpressionRowC::Let {
                binder,
                value,
                body,
            } => {
                if *binder >= v.names.len() {
                    return Err(si24("let binder"));
                }
                kids = vec![(*value, 0), (*body, 1)];
            }
            ExpressionRowC::Binary { left, right, .. } => {
                kids = vec![(*left, 0), (*right, 1)];
            }
            ExpressionRowC::Unary { operand, .. } => {
                kids = vec![(*operand, 0)];
            }
            ExpressionRowC::If {
                cond,
                then_expr,
                else_expr,
            } => {
                kids = vec![(*cond, 0), (*then_expr, 1), (*else_expr, 2)];
            }
            ExpressionRowC::Call {
                function_name,
                args,
            } => {
                if *function_name >= v.names.len() {
                    return Err(si24("call name"));
                }
                let Some(a_end) = args.start.checked_add(args.len) else {
                    return Err(si24("call args overflow"));
                };
                if a_end > v.children.len() {
                    return Err(si24("call args slice"));
                }
                for (k, pos) in (args.start..a_end).enumerate() {
                    kids.push((v.children[pos], k));
                }
            }
            ExpressionRowC::Construct {
                type_id,
                ctor_id,
                fields,
                constant,
            } => {
                let arity = adt_checked_ctor_arity(v, *type_id, *ctor_id)
                    .map_err(|_| si24("construct arity"))?;
                if fields.len != arity {
                    return Err(si24("construct fields.len"));
                }
                let Some(f_end) = fields.start.checked_add(fields.len) else {
                    return Err(si24("construct children overflow"));
                };
                if f_end > v.children.len() {
                    return Err(si24("construct children slice"));
                }
                if (fields.len == 0) != constant.is_some() {
                    return Err(si24("construct nullary constant"));
                }
                if let Some(cid) = constant {
                    let Some(RawConstRow::Constructor {
                        marker_free: true,
                        type_id: ct,
                        ctor_id: cc,
                        fields: cf,
                    }) = v.constants.get(*cid)
                    else {
                        return Err(si24("construct nullary const row"));
                    };
                    if ct.0 != type_id.0 || cc.0 != ctor_id.0 || cf.len != 0 {
                        return Err(si24("construct nullary const match"));
                    }
                }
                for (k, pos) in (fields.start..f_end).enumerate() {
                    kids.push((v.children[pos], k));
                }
            }
            ExpressionRowC::Match {
                scrutinee,
                arms,
                plan,
            } => {
                let Some(a_end) = arms.start.checked_add(arms.len) else {
                    return Err(si24("match arms overflow"));
                };
                if a_end > v.arms.len() {
                    return Err(si24("match arms slice"));
                }
                kids.push((*scrutinee, 0));
                for (k, aid) in (arms.start..a_end).enumerate() {
                    kids.push((v.arms[aid].body, k + 1));
                }
                if let Some(root) = plan {
                    if *root >= v.plan_nodes.len() {
                        return Err(si24("match plan root"));
                    }
                    let mut idx = *root;
                    while idx < v.plan_nodes.len()
                        && plan_node_source_match(&v.plan_nodes[idx]) == ei
                    {
                        match v.plan_nodes[idx] {
                            PlanNodeC::Switch {
                                path,
                                type_id,
                                cases,
                                default,
                                ..
                            } => {
                                let Some(p_end) = path.start.checked_add(path.len) else {
                                    return Err(si24("switch path overflow"));
                                };
                                let Some(c_end) = cases.start.checked_add(cases.len) else {
                                    return Err(si24("switch cases overflow"));
                                };
                                let Some(trow) = v.types.get(type_id.0) else {
                                    return Err(si24("switch type_id"));
                                };
                                if p_end > v.field_paths.len()
                                    || c_end > v.plan_cases.len()
                                    || cases.len != trow.ctor_len
                                    || default <= idx
                                    || default >= v.plan_nodes.len()
                                    || plan_node_source_match(&v.plan_nodes[default]) != ei
                                {
                                    return Err(si24("switch node targets"));
                                }
                                for cpos in cases.start..c_end {
                                    if let Some(target) = v.plan_cases[cpos] {
                                        if target <= idx
                                            || target >= v.plan_nodes.len()
                                            || plan_node_source_match(&v.plan_nodes[target]) != ei
                                        {
                                            return Err(si24("switch case target"));
                                        }
                                    }
                                }
                            }
                            PlanNodeC::Bind {
                                path, binder, next, ..
                            } => {
                                let Some(p_end) = path.start.checked_add(path.len) else {
                                    return Err(si24("bind path overflow"));
                                };
                                if p_end > v.field_paths.len()
                                    || binder >= v.names.len()
                                    || next <= idx
                                    || next >= v.plan_nodes.len()
                                    || plan_node_source_match(&v.plan_nodes[next]) != ei
                                {
                                    return Err(si24("bind node target"));
                                }
                            }
                            PlanNodeC::Select { arm, .. } => {
                                if arm < arms.start
                                    || arm - arms.start >= arms.len
                                    || arm >= v.arms.len()
                                {
                                    return Err(si24("select arm"));
                                }
                            }
                            PlanNodeC::NoMatch { .. } => {}
                        }
                        idx += 1;
                    }
                }
            }
        }
        for (c, pos) in kids {
            if c <= ei || c >= n_expr || parent[c].is_some() {
                return Err(si24("expression child tree"));
            }
            parent[c] = Some(ei);
            child_pos[c] = pos;
        }
    }
    Ok(())
}
