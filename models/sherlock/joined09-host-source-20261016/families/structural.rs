//! Pure structural-family row effects. PROPOSAL, not installed or adopted.
//! Rows and deltas are from exact adopted HOST R4; owner selects before calling.
//! Root binds this sibling module to `super::super::shared` and frozen `raw`.
//! Uses provisional shared v3a reads and its owner-resolved foreign-rule mapping.
//! Uncompiled; Root owns installation and FamilyPlans method forwarding.
use super::super::shared::*;
use RuleIdC as RuleId;

fn wrong_variant() -> HostViolation {
    HostViolation { kind: HostViolationKind::WrongVariant, clause: None,
        rule: None, locator: None }
}
fn foreign_rule() -> HostViolation {
    HostViolation { kind: HostViolationKind::WrongVariant, clause: None,
        rule: None, locator: Some("foreign rule for family".to_string()) }
}
fn increment(n: usize) -> Result<usize, HostViolation> {
    n.checked_add(1).ok_or(HostViolation {
        kind: HostViolationKind::ArithmeticUndefined, clause: None,
        rule: None, locator: None })
}
fn helper_copy(pre: &dyn PreImageView) -> Result<HelperControlC, HostViolation> {
    pre.helper().copied().ok_or_else(wrong_variant)
}
fn task_copy(pre: &dyn PreImageView) -> Result<HelperTaskC, HostViolation> {
    pre.top_task().copied().ok_or_else(wrong_variant)
}
fn plan(writes: PlannedWrites, delta_assert: PlanDelta) -> EffectPlan {
    EffectPlan { writes, delta_assert }
}

// Non-entry Validate effects. The public wrapper adds the checked entry read.
fn plan_validate_nonentry(sel: &Selected, pre: &dyn PreImageView,
    guards: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> {
    use RuleId::*;
    if !matches!(&sel.rule, BHelperValidateOccurrence | BHelperValidateField |
        BHelperValidateEmpty | BHelperValidateInvalid | BHelperValidateExit |
        BHelperValidateFailure) { return Err(foreign_rule()); }
    let mut helper = helper_copy(pre)?;
    let HelperControlC::Validate { status, .. } = &mut helper
        else { return Err(wrong_variant()); };
    let mut w = PlannedWrites::default();
    let mut d = PlanDelta::default();
    match &sel.rule {
        BHelperValidateOccurrence => {
            let HelperTaskC::ValidateTask { r, .. } = task_copy(pre)?
                else { return Err(wrong_variant()); };
            if guards.eval_guard(&GuardId::GValidateOccLeaf, pre)? {
                w.task_op = TaskOp::Pop; d.task_live = -1;
            } else {
                w.task_rewrite_top = Some(HelperTaskC::ValidateTask {
                    r, visit: TaskVisitC::Fields, cursor: 0 });
            }
        }
        BHelperValidateField => {
            let HelperTaskC::ValidateTask { r, cursor, .. } = task_copy(pre)?
                else { return Err(wrong_variant()); };
            let child = pre.field_at(&r, &cursor)?;
            let child_task = HelperTaskC::ValidateTask {
                r: child, visit: TaskVisitC::Unvisited, cursor: 0 };
            if guards.eval_guard(&GuardId::GValidateFieldNontail, pre)? {
                w.task_rewrite_top = Some(HelperTaskC::ValidateTask {
                    r, visit: TaskVisitC::Fields, cursor: increment(cursor)? });
                w.task_op = TaskOp::Push(child_task); d.task_live = 1;
            } else { w.task_rewrite_top = Some(child_task); }
        }
        BHelperValidateEmpty => {
            *status = TriStatus::Positive; w.helper = Write::Set(Some(helper));
        }
        BHelperValidateInvalid => {
            w.task_op = TaskOp::Pop; d.task_live = -1;
            *status = TriStatus::Negative; w.helper = Write::Set(Some(helper));
        }
        BHelperValidateExit | BHelperValidateFailure => {
            w.result = Write::Set(Some(HelperResultC::Validated {
                ok: matches!(&sel.rule, BHelperValidateExit) }));
            w.helper = Write::Set(None);
        }
        _ => return Err(foreign_rule()),
    }
    Ok(plan(w, d))
}

// Non-entry Equal effects. Guard meaning is owner-owned.
fn plan_equal_nonentry(sel: &Selected, pre: &dyn PreImageView,
    guards: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> {
    use RuleId::*;
    if !matches!(&sel.rule, BHelperEqualPair | BHelperEqualField |
        BHelperEqualEmpty | BHelperEqualDifferent | BHelperEqualExit) {
        return Err(foreign_rule());
    }
    let mut helper = helper_copy(pre)?;
    let HelperControlC::Equal { status, .. } = &mut helper
        else { return Err(wrong_variant()); };
    let mut w = PlannedWrites::default();
    let mut d = PlanDelta::default();
    match &sel.rule {
        BHelperEqualPair => {
            let HelperTaskC::EqualTask { left, right, .. } = task_copy(pre)?
                else { return Err(wrong_variant()); };
            if guards.eval_guard(&GuardId::GEqualPairLeaf, pre)? {
                w.task_op = TaskOp::Pop; d.task_live = -1;
            } else {
                w.task_rewrite_top = Some(HelperTaskC::EqualTask {
                    left, right, visit: TaskVisitC::Fields, cursor: 0 });
            }
        }
        BHelperEqualField => {
            let HelperTaskC::EqualTask { left, right, cursor, .. } = task_copy(pre)?
                else { return Err(wrong_variant()); };
            let lc = pre.field_at(&left, &cursor)?;
            let rc = pre.field_at(&right, &cursor)?;
            let child = HelperTaskC::EqualTask {
                left: lc, right: rc, visit: TaskVisitC::Unvisited, cursor: 0 };
            if guards.eval_guard(&GuardId::GEqualFieldNontail, pre)? {
                w.task_rewrite_top = Some(HelperTaskC::EqualTask {
                    left, right, visit: TaskVisitC::Fields, cursor: increment(cursor)? });
                w.task_op = TaskOp::Push(child); d.task_live = 1;
            } else { w.task_rewrite_top = Some(child); }
        }
        BHelperEqualEmpty => {
            *status = TriStatus::Positive; w.helper = Write::Set(Some(helper));
        }
        BHelperEqualDifferent => {
            w.task_op = TaskOp::Pop; d.task_live = -1;
            *status = TriStatus::Negative; w.helper = Write::Set(Some(helper));
        }
        BHelperEqualExit => {
            w.alloc_data = Some(AllocDataWrite::Bool { b: *status == TriStatus::Positive });
            d.alloc_data_nodes = 1;
            w.result = Write::Set(Some(HelperResultC::EqualDone {
                value: RefC::Data(pre.active().data) }));
            w.helper = Write::Set(None);
        }
        _ => return Err(foreign_rule()),
    }
    Ok(plan(w, d))
}

// Post-arm Match effects. The public wrapper reads the checked arm/pattern.
// Pattern-kind mapping is the row payload computation; owner selection is unchanged.
fn plan_match_postarm(sel: &Selected, pre: &dyn PreImageView,
    _guards: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> {
    use RuleId::*;
    if !matches!(&sel.rule, BHelperMatchOccurrence | BHelperMatchField |
        BHelperMatchBinding | BHelperMatchDistinct | BHelperMatchNext |
        BHelperMatchSelected | BHelperMatchExit | BHelperMatchFailure) {
        return Err(foreign_rule());
    }
    let mut helper = helper_copy(pre)?;
    let HelperControlC::Match { scrutinee, partial, pat_tag, x, y,
        field_idx, staged, phase, status, .. } = &mut helper
        else { return Err(wrong_variant()); };
    let mut w = PlannedWrites::default();
    let mut d = PlanDelta::default();
    match &sel.rule {
        BHelperMatchOccurrence => {
            let expected = match *pat_tag {
                PatternTag::PNil => LogicalKind::Nil,
                PatternTag::PCons => LogicalKind::Cons,
                PatternTag::PLeaf => LogicalKind::Leaf,
                PatternTag::PBranch => LogicalKind::Branch,
                _ => return Err(wrong_variant()),
            };
            if pre.kind_of(scrutinee)? != expected { *phase = MatchPhase::NoMatchReady; }
            else { match *pat_tag {
                PatternTag::PNil => *phase = MatchPhase::Ready,
                PatternTag::PLeaf => { *field_idx = 0; *phase = MatchPhase::Field; }
                _ => *phase = MatchPhase::Distinct,
            } }
        }
        BHelperMatchField => {
            *staged = Some(pre.field_at(scrutinee, &usize::from(*field_idx))?);
            *phase = MatchPhase::Bind;
        }
        BHelperMatchBinding => {
            let binder = if *pat_tag == PatternTag::PVar || *field_idx == 0 {
                *x } else { *y };
            let value = (*staged).ok_or_else(wrong_variant)?;
            w.alloc_env = Some(EnvCellC { binder, value, previous: *partial });
            d.alloc_environment_cells = 1;
            *partial = Some(pre.active().env); *staged = None;
            if matches!(*pat_tag, PatternTag::PVar | PatternTag::PLeaf) || *field_idx == 1 {
                *phase = MatchPhase::Ready;
            } else { *field_idx = 1; *phase = MatchPhase::Field; }
        }
        BHelperMatchDistinct => {
            if *x == *y { *phase = MatchPhase::NoMatchReady; }
            else { *field_idx = 0; *phase = MatchPhase::Field; }
        }
        BHelperMatchNext => { *status = TriStatus::Negative; *phase = MatchPhase::Done; }
        BHelperMatchSelected => { *status = TriStatus::Positive; *phase = MatchPhase::Done; }
        BHelperMatchExit => {
            w.result = Write::Set(Some(HelperResultC::MatchMatched { env: *partial }));
            w.helper = Write::Set(None); return Ok(plan(w, d));
        }
        BHelperMatchFailure => {
            w.result = Write::Set(Some(HelperResultC::MatchNoMatch));
            w.helper = Write::Set(None); return Ok(plan(w, d));
        }
        _ => return Err(foreign_rule()),
    }
    w.helper = Write::Set(Some(helper));
    Ok(plan(w, d))
}

// Bind exit/failure payloads; the public wrapper handles all ten Enabled rows.
fn plan_bind_terminal(sel: &Selected, pre: &dyn PreImageView,
    _guards: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> {
    use RuleId::*;
    if !matches!(&sel.rule, BHelperBindExit | BHelperBindFailure) {
        return Err(foreign_rule());
    }
    let HelperControlC::Bind { partial, .. } = helper_copy(pre)?
        else { return Err(wrong_variant()); };
    let mut w = PlannedWrites::default();
    w.result = Write::Set(Some(if matches!(&sel.rule, BHelperBindExit) {
        HelperResultC::BindOk { env: partial }
    } else { HelperResultC::BindBad }));
    w.helper = Write::Set(None);
    Ok(plan(w, PlanDelta::default()))
}

fn bind_unwind_payload(sel: &Selected, pre: &dyn PreImageView,
    guards: &dyn GuardEvaluator, task_live: usize) -> Result<EffectPlan, HostViolation> {
    use RuleIdC::*;
    if !matches!(&sel.rule, BHelperBindEmpty | BHelperBindArity | BHelperBindType |
        BHelperBindDuplicate | BHelperBindInsert | BHelperBindPropagate) {
        return Err(foreign_rule());
    }
    let mut helper = pre.helper().copied().ok_or_else(wrong_variant)?;
    let HelperControlC::Bind { partial, dup_cursor, phase, status, .. } = &mut helper
        else { return Err(wrong_variant()); };
    let mut w = PlannedWrites::default();
    let mut d = PlanDelta::default();
    match &sel.rule {
        BHelperBindEmpty => {
            *phase = if task_live > 0 { BindPhase::UnwindType } else { BindPhase::Finished };
        }
        BHelperBindArity => {
            *status = BindStatus::Bad;
            *phase = if task_live > 0 { BindPhase::Propagate } else { BindPhase::Finished };
        }
        BHelperBindType => {
            // This named owner guard is precisely the row's type-reject branch.
            if guards.eval_guard(&GuardId::GBindTypeReject, pre)? {
                w.task_op = TaskOp::Pop; d.task_live = -1;
                *status = BindStatus::Bad;
                *phase = if task_live > 1 { BindPhase::Propagate } else { BindPhase::Finished };
            } else {
                *dup_cursor = *partial;
                *phase = if partial.is_none() { BindPhase::UnwindInsert } else { BindPhase::UnwindDup };
            }
        }
        BHelperBindDuplicate => {
            let cell_id = (*dup_cursor).ok_or_else(wrong_variant)?;
            let cell = pre.env_cell(&cell_id)?;
            if guards.eval_guard(&GuardId::GBindDupHit, pre)? {
                w.task_op = TaskOp::Pop; d.task_live = -1;
                *status = BindStatus::Bad; *dup_cursor = None;
                *phase = if task_live > 1 { BindPhase::Propagate } else { BindPhase::Finished };
            } else {
                *dup_cursor = cell.previous;
                *phase = if cell.previous.is_none() { BindPhase::UnwindInsert } else { BindPhase::UnwindDup };
            }
        }
        BHelperBindInsert => {
            let HelperTaskC::PairTask { binder, arg, .. } =
                pre.top_task().copied().ok_or_else(wrong_variant)?
                else { return Err(wrong_variant()); };
            w.alloc_env = Some(EnvCellC { binder, value: arg, previous: *partial });
            d.alloc_environment_cells = 1;
            w.task_op = TaskOp::Pop; d.task_live = -1;
            *partial = Some(pre.active().env);
            *phase = if task_live > 1 { BindPhase::UnwindType } else { BindPhase::Finished };
        }
        BHelperBindPropagate => {
            let HelperTaskC::PairTask { .. } =
                pre.top_task().copied().ok_or_else(wrong_variant)?
                else { return Err(wrong_variant()); };
            w.task_op = TaskOp::Pop; d.task_live = -1;
            *phase = if task_live > 1 { BindPhase::Propagate } else { BindPhase::Finished };
        }
        _ => return Err(foreign_rule()),
    }
    w.helper = Write::Set(Some(helper));
    Ok(EffectPlan { writes: w, delta_assert: d })
}

fn common(sel: &Selected, origin_reference_cursor: u128) -> Result<HelperCommon, HostViolation> {
    Ok(HelperCommon { site: sel.site.ok_or_else(wrong_variant)?,
        origin_reference_cursor, parent_resume: None })
}
fn empty_plan(helper: HelperControlC) -> EffectPlan {
    let mut writes = PlannedWrites::default(); writes.helper = Write::Set(Some(helper));
    EffectPlan { writes, delta_assert: PlanDelta::default() }
}
fn bind_entry_payload(sel: &Selected, pre: &dyn PreImageView,
    origin_reference_cursor: u128, params: SliceC) -> Result<EffectPlan, HostViolation> {
    let ControlC::Enter { function: Some(function), args } = *pre.control()
        else { return Err(wrong_variant()); };
    Ok(empty_plan(HelperControlC::Bind { common: common(sel, origin_reference_cursor)?,
        function, params, args, p_cursor: 0, a_cursor: 0, partial: None,
        dup_cursor: None, phase: BindPhase::Pairing, status: BindStatus::Ok }))
}
fn validate_entry_payload(sel: &Selected, pre: &dyn PreImageView,
    origin_reference_cursor: u128) -> Result<EffectPlan, HostViolation> {
    let ControlC::Eval { expr } = *pre.control() else { return Err(wrong_variant()); };
    let ExpressionRowC::Literal { constant } = *pre.expr(&expr)?
        else { return Err(wrong_variant()); };
    let root = RefC::Const(constant);
    let mut p = empty_plan(HelperControlC::Validate {
        common: common(sel, origin_reference_cursor)?, root, status: TriStatus::Pending });
    p.writes.task_op = TaskOp::Push(HelperTaskC::ValidateTask {
        r: root, visit: TaskVisitC::Unvisited, cursor: 0 }); p.delta_assert.task_live = 1;
    Ok(p)
}
fn equal_entry_payload(sel: &Selected, pre: &dyn PreImageView,
    origin_reference_cursor: u128) -> Result<EffectPlan, HostViolation> {
    let ControlC::Return { value: right } = *pre.control()
        else { return Err(wrong_variant()); };
    let FrameRowC::RightF { left, .. } = *pre.top_frame().ok_or_else(wrong_variant)?
        else { return Err(wrong_variant()); };
    let mut p = empty_plan(HelperControlC::Equal {
        common: common(sel, origin_reference_cursor)?, left, right, status: TriStatus::Pending });
    p.writes.task_op = TaskOp::Push(HelperTaskC::EqualTask {
        left, right, visit: TaskVisitC::Unvisited, cursor: 0 }); p.delta_assert.task_live = 1;
    Ok(p)
}
fn match_entry_payload(sel: &Selected, pre: &dyn PreImageView,
    origin_reference_cursor: u128) -> Result<EffectPlan, HostViolation> {
    let ControlC::Cases { value: scrutinee, arms, saved_env } = *pre.control()
        else { return Err(wrong_variant()); };
    Ok(empty_plan(HelperControlC::Match { common: common(sel, origin_reference_cursor)?,
        scrutinee, arm: arms.start, saved_env, partial: saved_env,
        pat_tag: PatternTag::PVar, x: 0, y: 0, field_idx: 0, staged: None,
        phase: MatchPhase::Arm, status: TriStatus::Pending }))
}

// pr comes from the public wrapper's checked parameter read. The value-slot
// read remains here, in normative parameter-before-argument order.
fn bind_pair_payload(_sel: &Selected, pre: &dyn PreImageView,
    pr: (usize, SortTag)) -> Result<EffectPlan, HostViolation> {
    let mut helper = pre.helper().copied().ok_or_else(wrong_variant)?;
    let HelperControlC::Bind { args, p_cursor, a_cursor, .. } = &mut helper
        else { return Err(wrong_variant()); };
    let pos = args.start.checked_add(*a_cursor).ok_or(HostViolation {
        kind: HostViolationKind::ArithmeticUndefined, clause: None,
        rule: None, locator: None })?;
    let arg = pre.value_slot(&pos)?;
    *p_cursor = p_cursor.checked_add(1).ok_or(HostViolation {
        kind: HostViolationKind::ArithmeticUndefined, clause: None,
        rule: None, locator: None })?;
    *a_cursor = a_cursor.checked_add(1).ok_or(HostViolation {
        kind: HostViolationKind::ArithmeticUndefined, clause: None,
        rule: None, locator: None })?;
    let mut writes = PlannedWrites::default();
    writes.task_op = TaskOp::Push(HelperTaskC::PairTask { binder: pr.0, sort: pr.1, arg });
    writes.helper = Write::Set(Some(helper));
    let mut delta_assert = PlanDelta::default(); delta_assert.task_live = 1;
    Ok(EffectPlan { writes, delta_assert })
}

// p is a private tuple projected from the wrapper's exact checked pattern row.
fn match_arm_payload(_sel: &Selected, pre: &dyn PreImageView,
    p: (PatternTag, usize, usize)) -> Result<EffectPlan, HostViolation> {
    let mut helper = pre.helper().copied().ok_or_else(wrong_variant)?;
    let HelperControlC::Match { scrutinee, pat_tag, x, y, staged, phase, .. } = &mut helper
        else { return Err(wrong_variant()); };
    *pat_tag = p.0;
    *x = if p.0 == PatternTag::PNil { 0 } else { p.1 };
    *y = if matches!(p.0, PatternTag::PCons | PatternTag::PBranch) { p.2 } else { 0 };
    if p.0 == PatternTag::PVar { *staged = Some(*scrutinee); *phase = MatchPhase::Bind; }
    else { *phase = MatchPhase::Tag; }
    let mut writes = PlannedWrites::default(); writes.helper = Write::Set(Some(helper));
    Ok(EffectPlan { writes, delta_assert: PlanDelta::default() })
}

// Root forwards the four corresponding FamilyPlans methods to these pure
// functions. No fallback body for another family is supplied here.
pub fn plan_bind(sel: &Selected, pre: &dyn PreImageView,
    guards: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> {
    use RuleIdC::*;
    match sel.rule {
        BHelperBindEntry => {
            let ControlC::Enter { function: Some(function), .. } = *pre.control()
                else { return Err(wrong_variant()); };
            let params = pre.function_row(&function)?.params;
            bind_entry_payload(sel, pre, pre.reference_cursor(), params)
        }
        BHelperBindPair => {
            let HelperControlC::Bind { params, p_cursor, .. } = helper_copy(pre)?
                else { return Err(wrong_variant()); };
            let pos = params.start.checked_add(p_cursor).ok_or(HostViolation {
                kind: HostViolationKind::ArithmeticUndefined, clause: None,
                rule: None, locator: None })?;
            let pr = pre.parameter(&pos)?;
            bind_pair_payload(sel, pre, (pr.binder, pr.sort))
        }
        BHelperBindEmpty | BHelperBindArity | BHelperBindType | BHelperBindDuplicate |
        BHelperBindInsert | BHelperBindPropagate => {
            bind_unwind_payload(sel, pre, guards, pre.task_live())
        }
        BHelperBindExit | BHelperBindFailure => plan_bind_terminal(sel, pre, guards),
        _ => Err(foreign_rule()),
    }
}

pub fn plan_validate(sel: &Selected, pre: &dyn PreImageView,
    guards: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> {
    use RuleIdC::*;
    match sel.rule {
        BHelperValidateEntry => validate_entry_payload(sel, pre, pre.reference_cursor()),
        BHelperValidateOccurrence | BHelperValidateField | BHelperValidateEmpty |
        BHelperValidateInvalid | BHelperValidateExit | BHelperValidateFailure => {
            plan_validate_nonentry(sel, pre, guards)
        }
        _ => Err(foreign_rule()),
    }
}

pub fn plan_equal(sel: &Selected, pre: &dyn PreImageView,
    guards: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> {
    use RuleIdC::*;
    match sel.rule {
        BHelperEqualEntry => equal_entry_payload(sel, pre, pre.reference_cursor()),
        BHelperEqualPair | BHelperEqualField | BHelperEqualEmpty |
        BHelperEqualDifferent | BHelperEqualExit => plan_equal_nonentry(sel, pre, guards),
        _ => Err(foreign_rule()),
    }
}

pub fn plan_match(sel: &Selected, pre: &dyn PreImageView,
    guards: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> {
    use RuleIdC::*;
    match sel.rule {
        BHelperMatchEntry => match_entry_payload(sel, pre, pre.reference_cursor()),
        BHelperMatchArm => {
            let HelperControlC::Match { arm, .. } = helper_copy(pre)?
                else { return Err(wrong_variant()); };
            let arm_row = pre.arm(&arm)?;
            let p = match *pre.pattern(&arm_row.pattern)? {
                PatternRowC::PVar { x } => (PatternTag::PVar, x, 0),
                PatternRowC::PNil => (PatternTag::PNil, 0, 0),
                PatternRowC::PCons { x, y } => (PatternTag::PCons, x, y),
                PatternRowC::PLeaf { x } => (PatternTag::PLeaf, x, 0),
                PatternRowC::PBranch { x, y } => (PatternTag::PBranch, x, y),
            };
            match_arm_payload(sel, pre, p)
        }
        BHelperMatchOccurrence | BHelperMatchField | BHelperMatchBinding |
        BHelperMatchDistinct | BHelperMatchNext | BHelperMatchSelected |
        BHelperMatchExit | BHelperMatchFailure => plan_match_postarm(sel, pre, guards),
        _ => Err(foreign_rule()),
    }
}
