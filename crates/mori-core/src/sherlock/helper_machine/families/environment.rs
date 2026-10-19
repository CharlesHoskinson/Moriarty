// PROVISIONAL source fragment; Root binds the sibling shared/raw modules.
// Pure row effects only. No selection, select_when, charges, caps, admission,
// validation, StepInv or image mutation. Rust compilation/execution is UNRUN.
// Private row helpers below are composed by the public plan_* functions.
use super::super::shared::{
    AppendPhase, ControlC, EffectPlan, EnvCellC, ExpressionRowC, FrameRowC,
    HelperCommon, HelperControlC, HelperResultC, HostViolation,
    HostViolationKind, GuardEvaluator, PlanDelta, PlannedWrites, PreImageView, RuleIdC,
    Selected, SiteTag, SliceC, TaskOp, TriStatus, Write,
};

fn effect_error(kind: HostViolationKind, locator: &str) -> HostViolation {
    HostViolation { kind, clause: None, rule: None,
        locator: Some(locator.to_string()) }
}
fn wrong(locator: &str) -> HostViolation {
    effect_error(HostViolationKind::WrongVariant, locator)
}
fn arithmetic(locator: &str) -> HostViolation {
    effect_error(HostViolationKind::ArithmeticUndefined, locator)
}
// Exact designated-owner Q3 rendering; no foreign RuleId is copied.
fn foreign() -> HostViolation {
    HostViolation { kind: HostViolationKind::WrongVariant, clause: None,
        rule: None, locator: Some("foreign rule for family".to_string()) }
}
// Every effect not named by a source row is preserved. This explicit starting
// value adds no Default implementation or implicit state transition.
fn preserved_effects() -> EffectPlan {
    EffectPlan {
        delta_assert: PlanDelta { alloc_data_nodes: 0, alloc_data_fields: 0,
            alloc_environment_cells: 0, depth: 0, task_live: 0,
            active_value_slots: 0 },
        writes: PlannedWrites { control: Write::Keep, control_origin: Write::Keep,
            env_root: Write::Keep, result: Write::Keep, helper: Write::Keep,
            frame: None, task_rewrite_top: None, task_op: TaskOp::None,
            alloc_data: None, alloc_env: None, alloc_value_slot: None,
            alloc_data_field: None, alloc_constructor: None },
    }
}
fn helper_copy(pre: &dyn PreImageView) -> Result<HelperControlC, HostViolation> {
    pre.helper().copied().ok_or_else(|| wrong("reg.helper absent"))
}

// rows-supplement.json#/helper_events/7/effects
fn env_get_cell(pre: &dyn PreImageView) -> Result<EffectPlan, HostViolation> {
    let mut h = helper_copy(pre)?;
    let HelperControlC::EnvGet { binder, cursor, found, status, .. } = &mut h
        else { return Err(wrong("EnvGet.cell helper")); };
    let id = cursor.ok_or_else(|| wrong("EnvGet.cell cursor absent"))?;
    let cell = pre.env_cell(&id)?;
    if cell.binder == *binder {
        *found = Some(cell.value);
        *status = TriStatus::Positive;
    } else {
        *cursor = cell.previous;
    }
    let mut p = preserved_effects();
    p.writes.helper = Write::Set(Some(h));
    Ok(p)
}
// rows-supplement.json#/helper_events/8/effects
fn env_get_empty(pre: &dyn PreImageView) -> Result<EffectPlan, HostViolation> {
    let mut h = helper_copy(pre)?;
    let HelperControlC::EnvGet { status, .. } = &mut h
        else { return Err(wrong("EnvGet.empty helper")); };
    *status = TriStatus::Negative;
    let mut p = preserved_effects();
    p.writes.helper = Write::Set(Some(h));
    Ok(p)
}
// rows-supplement.json#/helper_events/9/effects
fn env_get_exit(pre: &dyn PreImageView) -> Result<EffectPlan, HostViolation> {
    let HelperControlC::EnvGet { found, .. } = helper_copy(pre)?
        else { return Err(wrong("EnvGet.exit helper")); };
    let value = found.ok_or_else(|| wrong("EnvGet.exit found absent"))?;
    let mut p = preserved_effects();
    p.writes.result = Write::Set(Some(HelperResultC::EnvFound { value }));
    p.writes.helper = Write::Set(None);
    Ok(p)
}
// rows-supplement.json#/helper_events/10/effects
fn env_get_failure(pre: &dyn PreImageView) -> Result<EffectPlan, HostViolation> {
    let HelperControlC::EnvGet { .. } = helper_copy(pre)?
        else { return Err(wrong("EnvGet.failure helper")); };
    let mut p = preserved_effects();
    p.writes.result = Write::Set(Some(HelperResultC::EnvMissing));
    p.writes.helper = Write::Set(None);
    Ok(p)
}

// rows-supplement.json#/helper_events/13/effects
fn env_put_insert(pre: &dyn PreImageView) -> Result<EffectPlan, HostViolation> {
    let mut h = helper_copy(pre)?;
    let HelperControlC::EnvPut { binder, value, base, new_root, status, .. } = &mut h
        else { return Err(wrong("EnvPut.insert helper")); };
    let cell = EnvCellC { binder: *binder, value: *value, previous: *base };
    // allocEnv binds its identity at the pre-image physical active length.
    *new_root = Some(pre.active().env);
    *status = TriStatus::Positive;
    let mut p = preserved_effects();
    p.writes.alloc_env = Some(cell);
    p.writes.helper = Write::Set(Some(h));
    p.delta_assert.alloc_environment_cells = 1;
    Ok(p)
}
// rows-supplement.json#/helper_events/15/effects
fn env_put_exit(pre: &dyn PreImageView) -> Result<EffectPlan, HostViolation> {
    let HelperControlC::EnvPut { new_root, .. } = helper_copy(pre)?
        else { return Err(wrong("EnvPut.exit helper")); };
    let cell = new_root.ok_or_else(|| wrong("EnvPut.exit newRoot absent"))?;
    let mut p = preserved_effects();
    p.writes.result = Write::Set(Some(HelperResultC::EnvUpdated { cell }));
    p.writes.helper = Write::Set(None);
    Ok(p)
}

// rows-supplement.json#/helper_events/19/effects
fn find_empty(pre: &dyn PreImageView) -> Result<EffectPlan, HostViolation> {
    let mut h = helper_copy(pre)?;
    let HelperControlC::Find { status, .. } = &mut h
        else { return Err(wrong("Find.empty helper")); };
    *status = TriStatus::Negative;
    let mut p = preserved_effects();
    p.writes.helper = Write::Set(Some(h));
    Ok(p)
}
// rows-supplement.json#/helper_events/20/effects
fn find_exit(pre: &dyn PreImageView) -> Result<EffectPlan, HostViolation> {
    let HelperControlC::Find { found, .. } = helper_copy(pre)?
        else { return Err(wrong("Find.exit helper")); };
    let function = found.ok_or_else(|| wrong("Find.exit found absent"))?;
    let mut p = preserved_effects();
    p.writes.result = Write::Set(Some(HelperResultC::FindFound { function }));
    p.writes.helper = Write::Set(None);
    Ok(p)
}
// rows-supplement.json#/helper_events/21/effects
fn find_failure(pre: &dyn PreImageView) -> Result<EffectPlan, HostViolation> {
    let HelperControlC::Find { .. } = helper_copy(pre)?
        else { return Err(wrong("Find.failure helper")); };
    let mut p = preserved_effects();
    p.writes.result = Write::Set(Some(HelperResultC::FindMissing));
    p.writes.helper = Write::Set(None);
    Ok(p)
}

// rows-supplement.json#/helper_events/33/effects
fn append_cell(pre: &dyn PreImageView) -> Result<EffectPlan, HostViolation> {
    let mut h = helper_copy(pre)?;
    let HelperControlC::Append { src, src_cursor, staged, .. } = &mut h
        else { return Err(wrong("Append.cell helper")); };
    let pos = src.start.checked_add(*src_cursor)
        .ok_or_else(|| arithmetic("Append.cell src.start + srcCursor"))?;
    *staged = Some(pre.value_slot(&pos)?);
    let mut p = preserved_effects();
    p.writes.helper = Write::Set(Some(h));
    Ok(p)
}
// rows-supplement.json#/helper_events/34/effects
fn append_empty(pre: &dyn PreImageView) -> Result<EffectPlan, HostViolation> {
    let mut h = helper_copy(pre)?;
    let HelperControlC::Append { phase, .. } = &mut h
        else { return Err(wrong("Append.empty helper")); };
    *phase = AppendPhase::Tail;
    let mut p = preserved_effects();
    p.writes.helper = Write::Set(Some(h));
    Ok(p)
}
// rows-supplement.json#/helper_events/35/effects
fn append_tail(pre: &dyn PreImageView) -> Result<EffectPlan, HostViolation> {
    let mut h = helper_copy(pre)?;
    let HelperControlC::Append { value, phase, .. } = &mut h
        else { return Err(wrong("Append.tail helper")); };
    let value_to_append = *value;
    *phase = AppendPhase::Done;
    let mut p = preserved_effects();
    p.writes.alloc_value_slot = Some(value_to_append);
    p.writes.helper = Write::Set(Some(h));
    p.delta_assert.active_value_slots = 1;
    Ok(p)
}
// rows-supplement.json#/helper_events/36/effects
fn append_copy(pre: &dyn PreImageView) -> Result<EffectPlan, HostViolation> {
    let mut h = helper_copy(pre)?;
    let HelperControlC::Append { src_cursor, staged, .. } = &mut h
        else { return Err(wrong("Append.copy helper")); };
    let value_to_append = staged.ok_or_else(|| wrong("Append.copy staged absent"))?;
    *src_cursor = src_cursor.checked_add(1)
        .ok_or_else(|| arithmetic("Append.copy srcCursor + 1"))?;
    *staged = None;
    let mut p = preserved_effects();
    p.writes.alloc_value_slot = Some(value_to_append);
    p.writes.helper = Write::Set(Some(h));
    p.delta_assert.active_value_slots = 1;
    Ok(p)
}
// rows-supplement.json#/helper_events/37/effects
fn append_exit(pre: &dyn PreImageView) -> Result<EffectPlan, HostViolation> {
    let HelperControlC::Append { src, dest_start, .. } = helper_copy(pre)?
        else { return Err(wrong("Append.exit helper")); };
    let len = src.len.checked_add(1)
        .ok_or_else(|| arithmetic("Append.exit src.len + 1"))?;
    let mut p = preserved_effects();
    p.writes.result = Write::Set(Some(HelperResultC::AppendDone {
        seq: SliceC::normalized(dest_start, len) }));
    p.writes.helper = Write::Set(None);
    Ok(p)
}

// rows-supplement.json#/bookkeeping_events/1/effects
pub fn plan_bookkeeping_release(sel: &Selected, pre: &dyn PreImageView,
    _g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> {
    match &sel.rule {
        RuleIdC::BBookkeepingTaskRelease => {
            // The row records one top-task scratch read and exactly one pop.
            let _task = pre.top_task()
                .ok_or_else(|| wrong("task.release top task absent"))?;
            let mut p = preserved_effects();
            p.writes.task_op = TaskOp::Pop;
            p.delta_assert.task_live = -1;
            Ok(p)
        }
        _ => Err(foreign()),
    }
}

// The entry common record is passed from the authoritative family wrapper.
// rows-supplement.json#/helper_events/6/effects
fn env_get_entry(pre: &dyn PreImageView, common: HelperCommon)
    -> Result<EffectPlan, HostViolation> {
    let ControlC::Eval { expr } = pre.control()
        else { return Err(wrong("EnvGet.entry control")); };
    let ExpressionRowC::Var { binder } = pre.expr(expr)?
        else { return Err(wrong("EnvGet.entry expression")); };
    let mut p = preserved_effects();
    p.writes.helper = Write::Set(Some(HelperControlC::EnvGet {
        common, binder: *binder, cursor: pre.env_root(), found: None,
        status: TriStatus::Pending }));
    Ok(p)
}
// rows-supplement.json#/helper_events/11/effects
fn env_put_entry(pre: &dyn PreImageView, common: HelperCommon)
    -> Result<EffectPlan, HostViolation> {
    let ControlC::Return { value } = pre.control()
        else { return Err(wrong("EnvPut.entry control")); };
    let Some(FrameRowC::LetF { binder, saved_env, .. }) = pre.top_frame()
        else { return Err(wrong("EnvPut.entry top frame")); };
    let mut p = preserved_effects();
    p.writes.helper = Write::Set(Some(HelperControlC::EnvPut {
        common, binder: *binder, value: *value, base: *saved_env,
        new_root: None, status: TriStatus::Pending }));
    Ok(p)
}
// rows-supplement.json#/helper_events/17/effects
fn find_entry(pre: &dyn PreImageView, common: HelperCommon)
    -> Result<EffectPlan, HostViolation> {
    let ControlC::Invoke { function_name, .. } = pre.control()
        else { return Err(wrong("Find.entry control")); };
    let mut p = preserved_effects();
    p.writes.helper = Write::Set(Some(HelperControlC::Find {
        common, function_name: *function_name, row_cursor: 0,
        found: None, status: TriStatus::Pending }));
    Ok(p)
}
// rows-supplement.json#/helper_events/18/effects
// The wrapper supplies the checked name read at the pre-helper row_cursor.
fn find_row(pre: &dyn PreImageView, inspected_function_name: usize)
    -> Result<EffectPlan, HostViolation> {
    let mut h = helper_copy(pre)?;
    let HelperControlC::Find { function_name, row_cursor, found, status, .. } = &mut h
        else { return Err(wrong("Find.row helper")); };
    if inspected_function_name == *function_name {
        *found = Some(*row_cursor);
        *status = TriStatus::Positive;
    } else {
        *row_cursor = row_cursor.checked_add(1)
            .ok_or_else(|| arithmetic("Find.row rowCursor + 1"))?;
    }
    let mut p = preserved_effects();
    p.writes.helper = Write::Set(Some(h));
    Ok(p)
}
// rows-supplement.json#/helper_events/32/effects
fn append_entry(pre: &dyn PreImageView, common: HelperCommon)
    -> Result<EffectPlan, HostViolation> {
    let ControlC::Return { value } = pre.control()
        else { return Err(wrong("Append.entry control")); };
    let Some(FrameRowC::ArgsF { completed, .. }) = pre.top_frame()
        else { return Err(wrong("Append.entry top frame")); };
    let mut p = preserved_effects();
    p.writes.helper = Write::Set(Some(HelperControlC::Append {
        common, src: *completed, value: *value, src_cursor: 0,
        dest_start: pre.active().value_slots, staged: None,
        phase: AppendPhase::Copy }));
    Ok(p)
}


// FamilyPlans delegates may call these free functions from Root's one shared
// implementation. Selected is authoritative; no select_when is evaluated.
fn entry_common(sel: &Selected, pre: &dyn PreImageView)
    -> Result<HelperCommon, HostViolation> {
    let site = sel.site.ok_or_else(|| wrong("selected entry site absent"))?;
    Ok(HelperCommon { site, origin_reference_cursor: pre.reference_cursor(),
        parent_resume: None })
}

pub fn plan_env_get(sel: &Selected, pre: &dyn PreImageView,
    _g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> {
    match &sel.rule {
        RuleIdC::BHelperEnvGetEntry => env_get_entry(pre, entry_common(sel, pre)?),
        RuleIdC::BHelperEnvGetCell => env_get_cell(pre),
        RuleIdC::BHelperEnvGetEmpty => env_get_empty(pre),
        RuleIdC::BHelperEnvGetExit => env_get_exit(pre),
        RuleIdC::BHelperEnvGetFailure => env_get_failure(pre),
        _ => Err(foreign()),
    }
}

pub fn plan_env_put(sel: &Selected, pre: &dyn PreImageView,
    _g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> {
    match &sel.rule {
        RuleIdC::BHelperEnvPutEntry => env_put_entry(pre, entry_common(sel, pre)?),
        RuleIdC::BHelperEnvPutInsert => env_put_insert(pre),
        RuleIdC::BHelperEnvPutExit => env_put_exit(pre),
        _ => Err(foreign()),
    }
}

pub fn plan_find(sel: &Selected, pre: &dyn PreImageView,
    _g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> {
    match &sel.rule {
        RuleIdC::BHelperFindEntry => find_entry(pre, entry_common(sel, pre)?),
        RuleIdC::BHelperFindRow => {
            let HelperControlC::Find { row_cursor, .. } = helper_copy(pre)?
                else { return Err(wrong("Find.row helper")); };
            let function_name = pre.function_row(&row_cursor)?.function_name;
            find_row(pre, function_name)
        }
        RuleIdC::BHelperFindEmpty => find_empty(pre),
        RuleIdC::BHelperFindExit => find_exit(pre),
        RuleIdC::BHelperFindFailure => find_failure(pre),
        _ => Err(foreign()),
    }
}

pub fn plan_append(sel: &Selected, pre: &dyn PreImageView,
    _g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> {
    match (sel.rule, sel.site) {
        (RuleIdC::BHelperAppendEntry, Some(SiteTag::RetCtorFields)) =>
            return super::adt_constructor::append_entry(sel, pre),
        _ => {}
    }
    match &sel.rule {
        RuleIdC::BHelperAppendEntry => append_entry(pre, entry_common(sel, pre)?),
        RuleIdC::BHelperAppendCell => append_cell(pre),
        RuleIdC::BHelperAppendEmpty => append_empty(pre),
        RuleIdC::BHelperAppendTail => append_tail(pre),
        RuleIdC::BHelperAppendCopy => append_copy(pre),
        RuleIdC::BHelperAppendExit => append_exit(pre),
        _ => Err(foreign()),
    }
}
