// owner_runtime.proposed.rs — PROPOSED, source-only, uncompiled, unadopted.
// Root placement: shared::owner_runtime (child of `shared`). No OwnerEngine
// trait impl here; Root wires these free functions into its sole impl.
// Cross-lane dependencies are explicit parameters: `families: &dyn FamilyPlans`
// (pure row effects, other work already supplied), `guards: &OwnerGuards`,
// and, for SI-24 only, the prepare lane's `check_program_view` function.
use super::*;
use super::super::raw::{
    ConstRow as RawConstRow, CtorId as RawCtorId, DataRow as RawDataRow, RuleId as RawRuleId,
    TypeId as RawTypeId,
};
mod owner_adt_runtime;
use owner_adt_runtime::*;
pub(super) use owner_adt_runtime::check_generic_program_view;
mod owner_si21_bind_match;
use owner_si21_bind_match::{si21_bind_helper_ok, si21_bind_result_ok, si21_match_helper_ok, si21_match_result_ok};
mod owner_si21_closure;
use owner_si21_closure::{si21_helper_ok, si21_result_ok, si22_helper_ok};

fn hv(kind: HostViolationKind, clause: Option<StepInvClauseId>, rule: Option<RuleIdC>, loc: &str) -> HostViolation {
    HostViolation { kind, clause, rule, locator: Some(loc.to_string()) }
}
fn wrong(loc: &str) -> HostViolation { hv(HostViolationKind::WrongVariant, None, None, loc) }
fn oob(loc: &str) -> HostViolation { hv(HostViolationKind::ReadOutOfBounds, None, None, loc) }
fn arith(loc: &str) -> HostViolation { hv(HostViolationKind::ArithmeticUndefined, None, None, loc) }
fn invalid_plan(_loc: &str) -> HostViolation { hv(HostViolationKind::WrongVariant, None, None, "invalid plan") }
fn siv(clause: StepInvClauseId, loc: &str, detail: &str) -> StepInvViolation {
    StepInvViolation { clause, locator: Some(loc.to_string()), detail: detail.to_string() }
}
fn is_disabled(r: &RuleIdC) -> bool {
    matches!(r, RuleIdC::BHelperEnvPutCell | RuleIdC::BHelperEnvPutEmpty | RuleIdC::BHelperEnvPutFailure
        | RuleIdC::BHelperAppendFailure | RuleIdC::BHelperEqualFailure | RuleIdC::BHelperMatchEmpty
        | RuleIdC::BBookkeepingTaskEnqueue)
}
fn is_extension(r: &RuleIdC) -> bool {
    matches!(r, RuleIdC::BHelperValidateDeclaration | RuleIdC::BHelperMatchDiscard
        | RuleIdC::BBookkeepingResumeStore | RuleIdC::BBookkeepingResumeRestore)
}
fn check_enabled(r: &RuleIdC) -> Result<(), HostViolation> {
    if is_disabled(r) { Err(hv(HostViolationKind::DisabledRuleReached, None, Some(*r), "disabled rule")) }
    else if is_extension(r) { Err(hv(HostViolationKind::ExtensionRuleReached, None, Some(*r), "extension rule")) }
    else { Ok(()) }
}

// ================= PreImageView over const/data/program tables =================
fn const_fields(r: &RawConstRow) -> (usize, usize) {
    match r {
        RawConstRow::Cons { fields, .. }
        | RawConstRow::Branch { fields, .. }
        | RawConstRow::Constructor { fields, .. } => (fields.start.0, fields.len),
        RawConstRow::Leaf { fields, .. } => (fields.start.0, fields.len),
        _ => (0, 0),
    }
}
fn data_fields(r: &RawDataRow) -> (usize, usize) {
    match r {
        RawDataRow::Cons { fields }
        | RawDataRow::Branch { fields }
        | RawDataRow::Constructor { fields, .. } => (fields.start.0, fields.len),
        RawDataRow::Leaf { fields } => (fields.start.0, fields.len),
        _ => (0, 0),
    }
}
fn raw_marker_free(r: &RawConstRow) -> bool {
    match r {
        RawConstRow::U128 { marker_free, .. } | RawConstRow::Bool { marker_free, .. }
        | RawConstRow::Nil { marker_free } | RawConstRow::Cons { marker_free, .. }
        | RawConstRow::Leaf { marker_free, .. } | RawConstRow::Branch { marker_free, .. }
        | RawConstRow::Constructor { marker_free, .. }
        | RawConstRow::U128OutOfRange { marker_free } => *marker_free,
    }
}
impl<'a> PreImageView for PreImage<'a> {
    fn adt_reads(&self) -> &dyn adt_runtime::AdtRuntimeReads { self }
    fn control(&self) -> &ControlC { &self.img.control }
    fn control_origin(&self) -> usize { self.img.control_origin }
    fn env_root(&self) -> EnvRootC { self.img.env_root }
    fn depth(&self) -> usize { self.img.frames.len() }
    fn task_live(&self) -> usize { self.img.tasks.len() }
    fn stack_limit(&self) -> usize { self.img.stack_limit }
    fn outcome(&self) -> &OutcomeC { &self.img.outcome }
    fn initial_fuel(&self) -> u128 { self.img.initial_fuel }
    fn remaining_work(&self) -> u128 { self.img.remaining_work }
    fn consumed_work(&self) -> u128 { self.img.consumed_work }
    fn helper_work(&self) -> u128 { self.img.helper_work }
    fn helper_visits(&self) -> u128 { self.img.helper_visits }
    fn reference_cursor(&self) -> u128 { self.img.reference_cursor }
    fn helper(&self) -> Option<&HelperControlC> { self.img.helper.as_ref() }
    fn result(&self) -> Option<&HelperResultC> { self.img.result.as_ref() }
    fn top_frame(&self) -> Option<&FrameRowC> { self.img.frames.last() }
    fn top_task(&self) -> Option<&HelperTaskC> { self.img.tasks.last() }
    fn active(&self) -> ActiveLengths { self.img.active }
    fn alloc(&self) -> AllocCounters { self.img.alloc }
    fn expr(&self, id: &usize) -> Result<&ExpressionRowC, HostViolation> {
        self.admitted.body.view.expressions.get(*id).ok_or_else(|| oob("prog.expressions"))
    }
    fn child(&self, pos: &usize) -> Result<usize, HostViolation> {
        self.admitted.body.view.children.get(*pos).copied().ok_or_else(|| oob("prog.children"))
    }
    fn function_row(&self, id: &usize) -> Result<&FunctionRowC, HostViolation> {
        self.admitted.body.view.functions.get(*id).ok_or_else(|| oob("prog.functions"))
    }
    fn admitted_function_count(&self) -> usize { self.admitted.body.view.functions.len() }
    fn parameter(&self, pos: &usize) -> Result<&ParameterRowC, HostViolation> {
        self.admitted.body.view.parameters.get(*pos).ok_or_else(|| oob("prog.parameters"))
    }
    fn arm(&self, id: &usize) -> Result<&ArmRowC, HostViolation> {
        self.admitted.body.view.arms.get(*id).ok_or_else(|| oob("prog.arms"))
    }
    fn pattern(&self, id: &usize) -> Result<&PatternRowC, HostViolation> {
        self.admitted.body.view.patterns.get(*id).ok_or_else(|| oob("prog.patterns"))
    }
    fn occurrence(&self, id: &usize) -> Result<&OccurrenceRowC, HostViolation> {
        self.admitted.body.view.occurrences.get(*id).ok_or_else(|| oob("prog.occurrences"))
    }
    fn occurrence_path(&self, s: &SliceC) -> Result<&[usize], HostViolation> {
        if s.len == 0 { if s.start == 0 { return Ok(&[]); } return Err(oob("prog.occurrencePaths")); }
        let end = s.start.checked_add(s.len).ok_or_else(|| arith("occurrence path end"))?;
        self.admitted.body.view.occurrence_paths.get(s.start..end).ok_or_else(|| oob("prog.occurrencePaths"))
    }
    fn constant_borrowed(&self, id: &usize) -> Result<pure_leaves::BorrowedValueRow<'_>, HostViolation> {
        self.admitted.body.view.constants.get(*id).map(pure_leaves::BorrowedValueRow::Const).ok_or_else(|| oob("prog.constants"))
    }
    fn constant_field(&self, id: &usize) -> Result<usize, HostViolation> {
        self.admitted.body.view.constant_fields.get(*id).copied().ok_or_else(|| oob("prog.constantFields"))
    }
    fn marker_free(&self, id: &usize) -> Result<bool, HostViolation> {
        self.admitted.body.view.constants.get(*id).map(raw_marker_free).ok_or_else(|| oob("prog.constants markerFree"))
    }
    fn deref_borrowed(&self, r: &RefC) -> Result<pure_leaves::BorrowedValueRow<'_>, HostViolation> {
        match r {
            RefC::Const(id) => self.constant_borrowed(id),
            RefC::Data(id) => self.img.data.get(*id).map(pure_leaves::BorrowedValueRow::Data).ok_or_else(|| oob("img.data")),
        }
    }
    fn field_at(&self, r: &RefC, i: &usize) -> Result<RefC, HostViolation> {
        match r {
            RefC::Const(id) => {
                let row = self.admitted.body.view.constants.get(*id).ok_or_else(|| oob("prog.constants"))?;
                let (start, len) = const_fields(row);
                if *i >= len { return Err(oob("const field index")); }
                let pos = start.checked_add(*i).ok_or_else(|| arith("const field pos"))?;
                Ok(RefC::Const(self.constant_field(&pos)?))
            }
            RefC::Data(id) => {
                let row = self.img.data.get(*id).ok_or_else(|| oob("img.data"))?;
                let (start, len) = data_fields(row);
                if *i >= len { return Err(oob("data field index")); }
                let pos = start.checked_add(*i).ok_or_else(|| arith("data field pos"))?;
                self.img.data_fields.get(pos).copied().ok_or_else(|| oob("img.dataFields"))
            }
        }
    }
    fn env_cell(&self, id: &usize) -> Result<&EnvCellC, HostViolation> {
        self.img.env.get(*id).ok_or_else(|| oob("img.env"))
    }
    fn value_slot(&self, pos: &usize) -> Result<RefC, HostViolation> {
        self.img.value_slots.get(*pos).copied().ok_or_else(|| oob("img.valueSlots"))
    }
    fn kind_of(&self, r: &RefC) -> Result<LogicalKind, HostViolation> {
        Ok(pure_leaves::borrowed_kind(self.deref_borrowed(r)?))
    }
    fn u128_of(&self, r: &RefC) -> Result<u128, HostViolation> {
        pure_leaves::borrowed_u128(self.deref_borrowed(r)?).ok_or_else(|| arith("u128 payload"))
    }
    fn bool_of(&self, r: &RefC) -> Result<bool, HostViolation> {
        pure_leaves::borrowed_bool(self.deref_borrowed(r)?).ok_or_else(|| arith("bool payload"))
    }
    fn checked_arity(&self, k: &LogicalKind) -> Result<usize, HostViolation> {
        match *k {
            LogicalKind::Constructor { type_id, ctor_id } =>
                adt_checked_ctor_arity(&self.admitted.body.view, type_id, ctor_id),
            _ => pure_leaves::arity_of(*k).ok_or_else(|| arith("arity undefined")),
        }
    }
    fn checked_sort(&self, k: &LogicalKind) -> Result<TypeIdC, HostViolation> {
        match *k {
            LogicalKind::Constructor { type_id, ctor_id } =>
                adt_checked_ctor_sort(&self.admitted.body.view, type_id, ctor_id),
            _ => pure_leaves::sort_of_kind(*k).ok_or_else(|| arith("sort undefined")),
        }
    }
    fn scalar_test_type_ok(&self, r: &RefC, expected: &TypeIdC) -> Result<bool, HostViolation> {
        let k = self.kind_of(r)?;
        if matches!(k, LogicalKind::Marker | LogicalKind::ConstructorExtensionReserved) { return Ok(false); }
        Ok(self.checked_sort(&k)? == *expected)
    }
}

// ================= OwnerGuards: nine guards + select_when + reads-r =================
fn helper_common(h: &HelperControlC) -> HelperCommon {
    match h { HelperControlC::Scalar { common, .. } | HelperControlC::EnvGet { common, .. }
        | HelperControlC::EnvPut { common, .. } | HelperControlC::Find { common, .. }
        | HelperControlC::Bind { common, .. } | HelperControlC::Append { common, .. }
        | HelperControlC::Validate { common, .. } | HelperControlC::Equal { common, .. }
        | HelperControlC::Match { common, .. } | HelperControlC::Build { common, .. }
        | HelperControlC::Plan { common, .. } => *common }
}
fn helper_kind_of(h: &HelperControlC) -> HelperKind {
    match h { HelperControlC::Scalar { .. } => HelperKind::Scalar, HelperControlC::EnvGet { .. } => HelperKind::EnvGet,
        HelperControlC::EnvPut { .. } => HelperKind::EnvPut, HelperControlC::Find { .. } => HelperKind::Find,
        HelperControlC::Bind { .. } => HelperKind::Bind, HelperControlC::Append { .. } => HelperKind::Append,
        HelperControlC::Validate { .. } => HelperKind::Validate, HelperControlC::Equal { .. } => HelperKind::Equal,
        HelperControlC::Match { .. } => HelperKind::Match, HelperControlC::Build { .. } => HelperKind::Build,
        HelperControlC::Plan { .. } => HelperKind::Plan }
}
impl GuardEvaluator for OwnerGuards {
    fn eval_guard(&self, g: &GuardId, pre: &dyn PreImageView) -> Result<bool, HostViolation> {
        match g {
            GuardId::GScalarAllocNode => Ok(matches!(pre.helper(),
                Some(HelperControlC::Scalar { staged: ScalarStaged::U128Value { .. } | ScalarStaged::BoolValue { .. } | ScalarStaged::PendingNode { .. }, .. }))),
            GuardId::GScalarFields1 => Ok(matches!(pre.helper(),
                Some(HelperControlC::Scalar { staged: ScalarStaged::PendingNode { tag: NodeKind::Leaf }, .. }))),
            GuardId::GScalarFields2 => Ok(matches!(pre.helper(),
                Some(HelperControlC::Scalar { staged: ScalarStaged::PendingNode { tag: NodeKind::Cons | NodeKind::Branch }, .. }))),
            GuardId::GValidateOccLeaf => match pre.top_task() {
                Some(HelperTaskC::ValidateTask { r, .. }) => Ok(pre.checked_arity(&pre.kind_of(r)?)? == 0),
                _ => Err(wrong("GValidateOccLeaf task")) },
            GuardId::GValidateFieldNontail => match pre.top_task() {
                Some(HelperTaskC::ValidateTask { r, cursor, .. }) =>
                    Ok(cursor.checked_add(1).ok_or_else(|| arith("validate cursor+1"))? < pre.checked_arity(&pre.kind_of(r)?)?),
                _ => Err(wrong("GValidateFieldNontail task")) },
            GuardId::GEqualPairLeaf => match pre.top_task() {
                Some(HelperTaskC::EqualTask { left, .. }) => Ok(pre.checked_arity(&pre.kind_of(left)?)? == 0),
                _ => Err(wrong("GEqualPairLeaf task")) },
            GuardId::GEqualFieldNontail => match pre.top_task() {
                Some(HelperTaskC::EqualTask { left, cursor, .. }) =>
                    Ok(cursor.checked_add(1).ok_or_else(|| arith("equal cursor+1"))? < pre.checked_arity(&pre.kind_of(left)?)?),
                _ => Err(wrong("GEqualFieldNontail task")) },
            GuardId::GBindTypeReject => match pre.top_task() {
                Some(HelperTaskC::PairTask { sort, arg, .. }) => Ok(pre.checked_sort(&pre.kind_of(arg)?)? != *sort),
                _ => Err(wrong("GBindTypeReject task")) },
            GuardId::GBindDupHit => {
                let (dup, binder) = match (pre.helper(), pre.top_task()) {
                    (Some(HelperControlC::Bind { dup_cursor, .. }), Some(HelperTaskC::PairTask { binder, .. })) =>
                        (dup_cursor.ok_or_else(|| wrong("GBindDupHit dupCursor"))?, *binder),
                    _ => return Err(wrong("GBindDupHit state")) };
                Ok(pre.env_cell(&dup)?.binder == binder) }
            GuardId::GBuildLast => match pre.helper() {
                Some(HelperControlC::Build { src, src_cursor, phase: BuildPhase::Copy, .. }) =>
                    Ok(src_cursor.checked_add(1).ok_or_else(|| arith("build cursor+1"))? == src.len),
                _ => Err(wrong("GBuildLast state")) },
        }
    }
    fn eval_select_when(&self, rule: &RuleIdC, pre: &dyn PreImageView) -> Result<bool, HostViolation> {
        owner_select_when(rule, pre, self)
    }
    fn reads_result_root(&self, rule: &RuleIdC) -> bool {
        // AST Root{r} membership, fixed per row (never a substring test).
        matches!(rule, RuleIdC::KMachineEvalLiteralValid | RuleIdC::KMachineEvalLiteralInvalid
            | RuleIdC::KMachineEvalVariablePresent | RuleIdC::KMachineEvalVariableAbsent
            | RuleIdC::KMachineReturnLet | RuleIdC::KMachineReturnRight | RuleIdC::KMachineReturnUnary
            | RuleIdC::KMachineReturnIfTrue | RuleIdC::KMachineReturnIfFalse | RuleIdC::KMachineReturnIfInvalid
            | RuleIdC::KMachineReturnArgumentsNext | RuleIdC::KMachineReturnArgumentsFinal
            | RuleIdC::KMachineReturnCallValid | RuleIdC::KMachineReturnCallInvalid
            | RuleIdC::KMachineInvokeFind | RuleIdC::KMachineEnterBind | RuleIdC::KMachineCasesTry
            | RuleIdC::XAdtReturnCtorFieldsTyped | RuleIdC::XAdtReturnCtorFieldsMistyped
            | RuleIdC::XAdtReturnCtorFieldsNext | RuleIdC::XAdtReturnCtorFieldsFinal
            | RuleIdC::XAdtMaterializeDone | RuleIdC::XAdtCasesPlanHit | RuleIdC::XAdtCasesPlanMiss)
    }
}
fn res(pre: &dyn PreImageView) -> Option<HelperResultC> { pre.result().copied() }
fn owner_select_when(rule: &RuleIdC, pre: &dyn PreImageView, _g: &OwnerGuards) -> Result<bool, HostViolation> {
    use RuleIdC::*;
    Ok(match rule {
        // Legacy (37): select_when verbatim from rows-supplement legacy instances.
        KMachineEvalLiteralValid => matches!(res(pre), Some(HelperResultC::Validated { ok: true })),
        KMachineEvalLiteralInvalid => matches!(res(pre), Some(HelperResultC::Validated { ok: false })),
        KMachineEvalVariablePresent => matches!(res(pre), Some(HelperResultC::EnvFound { .. })),
        KMachineEvalVariableAbsent => matches!(res(pre), Some(HelperResultC::EnvMissing)),
        KMachineEvalLet | KMachineEvalBinary | KMachineEvalUnary | KMachineEvalIf | KMachineEvalMatch
        | KMachineReturnLeft | KMachineReturnMatch | KMachineReturnScope => true,
        KMachineEvalCallEmpty => match pre.control() { ControlC::Eval { expr } =>
            matches!(pre.expr(expr)?, ExpressionRowC::Call { args, .. } if args.len == 0), _ => false },
        KMachineEvalCallArguments => match pre.control() { ControlC::Eval { expr } =>
            matches!(pre.expr(expr)?, ExpressionRowC::Call { args, .. } if args.len > 0), _ => false },
        KMachinePushAccept => pre.depth() < pre.stack_limit(),
        KMachinePushRefuse => pre.depth() >= pre.stack_limit(),
        KMachineReturnLet => matches!(res(pre), Some(HelperResultC::EnvUpdated { .. })),
        KMachineReturnRight => matches!(res(pre), Some(HelperResultC::ScalarValue { .. } | HelperResultC::ScalarError { .. } | HelperResultC::EqualDone { .. })),
        KMachineReturnUnary => matches!(res(pre), Some(HelperResultC::ScalarValue { .. } | HelperResultC::ScalarError { .. })),
        KMachineReturnIfTrue => matches!(res(pre), Some(HelperResultC::ScalarSelector { sel: BoolSelector::True })),
        KMachineReturnIfFalse => matches!(res(pre), Some(HelperResultC::ScalarSelector { sel: BoolSelector::False })),
        KMachineReturnIfInvalid => matches!(res(pre), Some(HelperResultC::ScalarSelector { sel: BoolSelector::NotBool })),
        KMachineReturnArgumentsNext => matches!(res(pre), Some(HelperResultC::AppendDone { .. }))
            && matches!(pre.top_frame(), Some(FrameRowC::ArgsF { rest, .. }) if rest.len > 0),
        KMachineReturnArgumentsFinal => matches!(res(pre), Some(HelperResultC::AppendDone { .. }))
            && matches!(pre.top_frame(), Some(FrameRowC::ArgsF { rest, .. }) if rest.len == 0),
        KMachineReturnCallValid => matches!(res(pre), Some(HelperResultC::ScalarTypeTest { ok: true })),
        KMachineReturnCallInvalid => matches!(res(pre), Some(HelperResultC::ScalarTypeTest { ok: false })),
        KMachineInvokeFind => matches!(res(pre), Some(HelperResultC::FindFound { .. } | HelperResultC::FindMissing)),
        KMachineEnterMissing => matches!(pre.control(), ControlC::Enter { function: None, .. }),
        KMachineEnterBind => matches!(res(pre), Some(HelperResultC::BindOk { .. } | HelperResultC::BindBad)),
        KMachineBoundInvalid => matches!(pre.control(), ControlC::Bound { binding: BoundBinding::Bad, .. }),
        KMachineBoundValid => matches!(pre.control(), ControlC::Bound { binding: BoundBinding::Ok { .. }, .. }),
        KMachineCasesEmpty => matches!(pre.control(), ControlC::Cases { arms, .. } if arms.len == 0)
            && matches!(pre.expr(&pre.control_origin())?, ExpressionRowC::Match { plan: None, .. }),
        KMachineCasesTry => matches!(pre.expr(&pre.control_origin())?, ExpressionRowC::Match { plan: None, .. })
            && matches!(res(pre), Some(HelperResultC::MatchMatched { .. } | HelperResultC::MatchNoMatch)),
        KMachineCasesMiss => matches!(pre.control(), ControlC::CaseResult { matching: CaseMatching::NoMatch, .. }),
        KMachineCasesHit => matches!(pre.control(), ControlC::CaseResult { matching: CaseMatching::Matched { .. }, .. }),
        KMachinePrimitiveValue => matches!(pre.control(), ControlC::PrimResult { result: PrimOutcome::Value { .. } }),
        KMachinePrimitiveError => matches!(pre.control(), ControlC::PrimResult { result: PrimOutcome::Error { .. } }),
        XAdtEvalConstructNullary => match pre.control() { ControlC::Eval { expr } =>
            matches!(pre.expr(expr)?, ExpressionRowC::Construct { fields, constant: Some(_), .. } if fields.len == 0), _ => false },
        XAdtEvalConstructFields => match pre.control() { ControlC::Eval { expr } =>
            matches!(pre.expr(expr)?, ExpressionRowC::Construct { fields, constant: None, .. } if fields.len > 0), _ => false },
        XAdtReturnCtorFieldsTyped => matches!(pre.top_frame(), Some(FrameRowC::ConstructorFieldsF { checked: false, .. }))
            && matches!(res(pre), Some(HelperResultC::ScalarTypeTest { ok: true })),
        XAdtReturnCtorFieldsMistyped => matches!(pre.top_frame(), Some(FrameRowC::ConstructorFieldsF { checked: false, .. }))
            && matches!(res(pre), Some(HelperResultC::ScalarTypeTest { ok: false })),
        XAdtReturnCtorFieldsNext => matches!(pre.top_frame(), Some(FrameRowC::ConstructorFieldsF { checked: true, rest, .. }) if rest.len > 0)
            && matches!(res(pre), Some(HelperResultC::AppendDone { .. })),
        XAdtReturnCtorFieldsFinal => matches!(pre.top_frame(), Some(FrameRowC::ConstructorFieldsF { checked: true, rest, .. }) if rest.len == 0)
            && matches!(res(pre), Some(HelperResultC::AppendDone { .. })),
        XAdtMaterializeDone => matches!(pre.control(), ControlC::Materialize { .. })
            && matches!(res(pre), Some(HelperResultC::BuildDone { .. })),
        XAdtCasesPlanHit => matches!(pre.control(), ControlC::Cases { .. })
            && matches!(pre.expr(&pre.control_origin())?, ExpressionRowC::Match { plan: Some(_), .. })
            && matches!(res(pre), Some(HelperResultC::PlanMatched { .. })),
        XAdtCasesPlanMiss => matches!(pre.control(), ControlC::Cases { .. })
            && matches!(pre.expr(&pre.control_origin())?, ExpressionRowC::Match { plan: Some(_), .. })
            && matches!(res(pre), Some(HelperResultC::PlanNoMatch)),
        // Helper entries: selected only by dispatch step 3 (helperPlan), never by group scan.
        BHelperScalarEntry | BHelperEnvGetEntry | BHelperEnvPutEntry | BHelperFindEntry | BHelperBindEntry
        | BHelperAppendEntry | BHelperValidateEntry | BHelperEqualEntry | BHelperMatchEntry
        | XAdtHelperBuildEntry | XAdtHelperPlanEntry => false,
        BHelperScalarTag => matches!(pre.helper(), Some(HelperControlC::Scalar { phase: ScalarPhase::First, mode, .. })
            if !matches!(mode, ScalarMode::Leaf | ScalarMode::Not)),
        BHelperScalarArithmetic => matches!(pre.helper(), Some(HelperControlC::Scalar { phase, mode, .. })
            if (*phase == ScalarPhase::First && *mode == ScalarMode::Not)
            || (*phase == ScalarPhase::Second && matches!(mode, ScalarMode::Add | ScalarMode::Sub | ScalarMode::Mul | ScalarMode::Lt))),
        BHelperScalarConstruct => matches!(pre.helper(), Some(HelperControlC::Scalar { phase, mode, .. })
            if (*phase == ScalarPhase::First && *mode == ScalarMode::Leaf)
            || (*phase == ScalarPhase::Second && matches!(mode, ScalarMode::Cons | ScalarMode::Branch))),
        BHelperScalarExit => matches!(pre.helper(), Some(HelperControlC::Scalar { phase: ScalarPhase::Done, staged, .. })
            if !matches!(staged, ScalarStaged::PrimError { .. })),
        BHelperScalarFailure => matches!(pre.helper(), Some(HelperControlC::Scalar { phase: ScalarPhase::Done, staged: ScalarStaged::PrimError { .. }, .. })),
        BHelperEnvGetCell => matches!(pre.helper(), Some(HelperControlC::EnvGet { status: TriStatus::Pending, cursor: Some(_), .. })),
        BHelperEnvGetEmpty => matches!(pre.helper(), Some(HelperControlC::EnvGet { status: TriStatus::Pending, cursor: None, .. })),
        BHelperEnvGetExit => matches!(pre.helper(), Some(HelperControlC::EnvGet { status: TriStatus::Positive, .. })),
        BHelperEnvGetFailure => matches!(pre.helper(), Some(HelperControlC::EnvGet { status: TriStatus::Negative, .. })),
        BHelperEnvPutInsert => matches!(pre.helper(), Some(HelperControlC::EnvPut { status: TriStatus::Pending, .. })),
        BHelperEnvPutExit => matches!(pre.helper(), Some(HelperControlC::EnvPut { status: TriStatus::Positive, .. })),
        BHelperFindRow => match pre.helper() { Some(HelperControlC::Find { status: TriStatus::Pending, row_cursor, .. }) =>
            *row_cursor < pre.admitted_function_count(), _ => false },
        BHelperFindEmpty => match pre.helper() { Some(HelperControlC::Find { status: TriStatus::Pending, row_cursor, .. }) =>
            *row_cursor == pre.admitted_function_count(), _ => false },
        BHelperFindExit => matches!(pre.helper(), Some(HelperControlC::Find { status: TriStatus::Positive, .. })),
        BHelperFindFailure => matches!(pre.helper(), Some(HelperControlC::Find { status: TriStatus::Negative, .. })),
        BHelperBindPair => matches!(pre.helper(), Some(HelperControlC::Bind { phase: BindPhase::Pairing, params, args, p_cursor, a_cursor, .. })
            if *p_cursor < params.len && *a_cursor < args.len),
        BHelperBindEmpty => matches!(pre.helper(), Some(HelperControlC::Bind { phase: BindPhase::Pairing, params, args, p_cursor, a_cursor, .. })
            if *p_cursor == params.len && *a_cursor == args.len),
        BHelperBindArity => matches!(pre.helper(), Some(HelperControlC::Bind { phase: BindPhase::Pairing, params, args, p_cursor, a_cursor, .. })
            if (*p_cursor == params.len) != (*a_cursor == args.len)),
        BHelperBindType => matches!(pre.helper(), Some(HelperControlC::Bind { phase: BindPhase::UnwindType, .. })),
        BHelperBindDuplicate => matches!(pre.helper(), Some(HelperControlC::Bind { phase: BindPhase::UnwindDup, .. })),
        BHelperBindInsert => matches!(pre.helper(), Some(HelperControlC::Bind { phase: BindPhase::UnwindInsert, .. })),
        BHelperBindPropagate => matches!(pre.helper(), Some(HelperControlC::Bind { phase: BindPhase::Propagate, .. })),
        BHelperBindExit => matches!(pre.helper(), Some(HelperControlC::Bind { phase: BindPhase::Finished, status: BindStatus::Ok, .. })),
        BHelperBindFailure => matches!(pre.helper(), Some(HelperControlC::Bind { phase: BindPhase::Finished, status: BindStatus::Bad, .. })),
        BHelperAppendCopy => matches!(pre.helper(), Some(HelperControlC::Append { phase: AppendPhase::Copy, staged: Some(_), .. })),
        BHelperAppendCell => matches!(pre.helper(), Some(HelperControlC::Append { phase: AppendPhase::Copy, staged: None, src, src_cursor, .. })
            if *src_cursor < src.len),
        BHelperAppendEmpty => matches!(pre.helper(), Some(HelperControlC::Append { phase: AppendPhase::Copy, staged: None, src, src_cursor, .. })
            if *src_cursor == src.len),
        BHelperAppendTail => matches!(pre.helper(), Some(HelperControlC::Append { phase: AppendPhase::Tail, .. })),
        BHelperAppendExit => matches!(pre.helper(), Some(HelperControlC::Append { phase: AppendPhase::Done, .. })),
        BHelperValidateOccurrence => matches!(pre.helper(), Some(HelperControlC::Validate { status: TriStatus::Pending, .. }))
            && pre.task_live() > 0 && matches!(pre.top_task(), Some(HelperTaskC::ValidateTask { r, visit: TaskVisitC::Unvisited, .. })
                if pre.kind_of(r)? != LogicalKind::Marker),
        BHelperValidateField => matches!(pre.helper(), Some(HelperControlC::Validate { status: TriStatus::Pending, .. }))
            && matches!(pre.top_task(), Some(HelperTaskC::ValidateTask { visit: TaskVisitC::Fields, .. })),
        BHelperValidateEmpty => matches!(pre.helper(), Some(HelperControlC::Validate { status: TriStatus::Pending, .. })) && pre.task_live() == 0,
        BHelperValidateInvalid => matches!(pre.helper(), Some(HelperControlC::Validate { status: TriStatus::Pending, .. }))
            && matches!(pre.top_task(), Some(HelperTaskC::ValidateTask { r, visit: TaskVisitC::Unvisited, .. })
                if pre.kind_of(r)? == LogicalKind::Marker),
        BHelperValidateExit => matches!(pre.helper(), Some(HelperControlC::Validate { status: TriStatus::Positive, .. })),
        BHelperValidateFailure => matches!(pre.helper(), Some(HelperControlC::Validate { status: TriStatus::Negative, .. })) && pre.task_live() == 0,
        BHelperEqualPair => matches!(pre.helper(), Some(HelperControlC::Equal { status: TriStatus::Pending, .. }))
            && match pre.top_task() { Some(HelperTaskC::EqualTask { left, right, visit: TaskVisitC::Unvisited, .. }) =>
                !pure_leaves::shallow_differ(pre.deref_borrowed(left)?, pre.deref_borrowed(right)?), _ => false },
        BHelperEqualField => matches!(pre.helper(), Some(HelperControlC::Equal { status: TriStatus::Pending, .. }))
            && matches!(pre.top_task(), Some(HelperTaskC::EqualTask { visit: TaskVisitC::Fields, .. })),
        BHelperEqualEmpty => matches!(pre.helper(), Some(HelperControlC::Equal { status: TriStatus::Pending, .. })) && pre.task_live() == 0,
        BHelperEqualDifferent => matches!(pre.helper(), Some(HelperControlC::Equal { status: TriStatus::Pending, .. }))
            && match pre.top_task() { Some(HelperTaskC::EqualTask { left, right, visit: TaskVisitC::Unvisited, .. }) =>
                pure_leaves::shallow_differ(pre.deref_borrowed(left)?, pre.deref_borrowed(right)?), _ => false },
        BHelperEqualExit => matches!(pre.helper(), Some(HelperControlC::Equal { status, .. })
            if *status == TriStatus::Positive || (*status == TriStatus::Negative && pre.task_live() == 0)),
        BHelperMatchArm => matches!(pre.helper(), Some(HelperControlC::Match { phase: MatchPhase::Arm, .. })),
        BHelperMatchOccurrence => matches!(pre.helper(), Some(HelperControlC::Match { phase: MatchPhase::Tag, .. })),
        BHelperMatchField => matches!(pre.helper(), Some(HelperControlC::Match { phase: MatchPhase::Field, .. })),
        BHelperMatchBinding => matches!(pre.helper(), Some(HelperControlC::Match { phase: MatchPhase::Bind, .. })),
        BHelperMatchDistinct => matches!(pre.helper(), Some(HelperControlC::Match { phase: MatchPhase::Distinct, .. })),
        BHelperMatchNext => matches!(pre.helper(), Some(HelperControlC::Match { phase: MatchPhase::NoMatchReady, .. })),
        BHelperMatchSelected => matches!(pre.helper(), Some(HelperControlC::Match { phase: MatchPhase::Ready, .. })),
        BHelperMatchExit => matches!(pre.helper(), Some(HelperControlC::Match { phase: MatchPhase::Done, status: TriStatus::Positive, .. })),
        BHelperMatchFailure => matches!(pre.helper(), Some(HelperControlC::Match { phase: MatchPhase::Done, status: TriStatus::Negative, .. })),
        XAdtHelperBuildCell => matches!(pre.helper(), Some(HelperControlC::Build { phase: BuildPhase::Copy, staged: None, src, src_cursor, .. })
            if *src_cursor < src.len),
        XAdtHelperBuildField => matches!(pre.helper(), Some(HelperControlC::Build { phase: BuildPhase::Copy, staged: Some(_), src, src_cursor, .. })
            if *src_cursor < src.len),
        XAdtHelperBuildExit => matches!(pre.helper(), Some(HelperControlC::Build { phase: BuildPhase::Done, node: Some(_), .. })),
        XAdtHelperPlanWalk => matches!(pre.helper(), Some(HelperControlC::Plan { phase: PlanPhase::Walk, path, path_cursor, .. })
            if *path_cursor < path.len),
        XAdtHelperPlanSwitch => matches!(pre.helper(), Some(HelperControlC::Plan { phase: PlanPhase::Test, after_walk: PlanAfterWalk::Test, path, path_cursor, .. })
            if *path_cursor == path.len),
        XAdtHelperPlanBind => matches!(pre.helper(), Some(HelperControlC::Plan { phase: PlanPhase::Bind, after_walk: PlanAfterWalk::Bind, path, path_cursor, .. })
            if *path_cursor == path.len),
        XAdtHelperPlanSelect => matches!(pre.helper(), Some(HelperControlC::Plan { phase: PlanPhase::Ready, status: TriStatus::Pending, arm: Some(_), .. })),
        XAdtHelperPlanNoMatch => matches!(pre.helper(), Some(HelperControlC::Plan { phase: PlanPhase::NoMatchReady, status: TriStatus::Pending, arm: None, .. })),
        XAdtHelperPlanExit => matches!(pre.helper(), Some(HelperControlC::Plan { phase: PlanPhase::Done, status: TriStatus::Positive, arm: Some(_), .. })),
        XAdtHelperPlanFailure => matches!(pre.helper(), Some(HelperControlC::Plan { phase: PlanPhase::Done, status: TriStatus::Negative, .. })),
        BBookkeepingTaskRelease => matches!(pre.helper(),
            Some(HelperControlC::Validate { status: TriStatus::Negative, .. } | HelperControlC::Equal { status: TriStatus::Negative, .. }))
            && pre.task_live() > 0,
        _ => return Err(hv(HostViolationKind::DisabledRuleReached, None, Some(*rule), "non-enabled rule in select_when")),
    })
}
// Function-count seam is the shared PreImageView::admitted_function_count above; no ViewLen trait and no dyn fallback exists.

// ================= Site / helperPlan / selector =================
pub fn owner_site(pre: &PreImage) -> Result<SiteTag, HostViolation> {
    match pre.img.control {
        ControlC::Eval { expr } => Ok(match pre.expr(&expr)? {
            ExpressionRowC::Literal { .. } => SiteTag::EvalLiteral, ExpressionRowC::Var { .. } => SiteTag::EvalVar,
            ExpressionRowC::Let { .. } => SiteTag::EvalLet, ExpressionRowC::Binary { .. } => SiteTag::EvalBinary,
            ExpressionRowC::Unary { .. } => SiteTag::EvalUnary, ExpressionRowC::If { .. } => SiteTag::EvalIf,
            ExpressionRowC::Call { .. } => SiteTag::EvalCall, ExpressionRowC::Match { .. } => SiteTag::EvalMatch,
            ExpressionRowC::Construct { .. } => SiteTag::EvalConstruct }),
        ControlC::ScheduleNext { .. } => Ok(SiteTag::Push),
        ControlC::Return { .. } => {
            if pre.depth() == 0 { return Err(hv(HostViolationKind::SelectUndefined, None, None, "Return at depth 0")); }
            Ok(match pre.top_frame().ok_or_else(|| wrong("Return top frame"))? {
                FrameRowC::LetF { .. } => SiteTag::RetLet, FrameRowC::LeftF { .. } => SiteTag::RetLeft,
                FrameRowC::RightF { .. } => SiteTag::RetRight, FrameRowC::UnaryF { .. } => SiteTag::RetUnary,
                FrameRowC::IfF { .. } => SiteTag::RetIf, FrameRowC::MatchF { .. } => SiteTag::RetMatch,
                FrameRowC::ArgsF { .. } => SiteTag::RetArgs, FrameRowC::ReturnF { .. } => SiteTag::RetCall,
                FrameRowC::ScopeF { .. } => SiteTag::RetScope,
                FrameRowC::ConstructorFieldsF { .. } => SiteTag::RetCtorFields })
        }
        ControlC::Invoke { .. } => Ok(SiteTag::Invoke), ControlC::Enter { .. } => Ok(SiteTag::Enter),
        ControlC::Bound { .. } => Ok(SiteTag::Bound), ControlC::Cases { .. } => Ok(SiteTag::Cases),
        ControlC::CaseResult { .. } => Ok(SiteTag::CaseResult), ControlC::PrimResult { .. } => Ok(SiteTag::Prim),
        ControlC::Materialize { .. } => Ok(SiteTag::Materialize),
        ControlC::Error { .. } => Err(hv(HostViolationKind::SelectUndefined, None, None, "Error control in select")),
    }
}
pub fn owner_helper_plan(site: SiteTag, pre: &PreImage) -> Option<HelperKind> {
    match site {
        SiteTag::EvalLiteral => Some(HelperKind::Validate), SiteTag::EvalVar => Some(HelperKind::EnvGet),
        SiteTag::RetLet => Some(HelperKind::EnvPut),
        SiteTag::RetRight => match pre.top_frame() { Some(FrameRowC::RightF { op, .. }) =>
            Some(if *op == BinaryOp::Eq { HelperKind::Equal } else { HelperKind::Scalar }), _ => None },
        SiteTag::RetUnary | SiteTag::RetIf | SiteTag::RetCall => Some(HelperKind::Scalar),
        SiteTag::RetArgs => Some(HelperKind::Append),
        SiteTag::RetCtorFields => match pre.top_frame() {
            Some(FrameRowC::ConstructorFieldsF { checked, .. }) =>
                Some(if *checked { HelperKind::Append } else { HelperKind::Scalar }),
            _ => None },
        SiteTag::Materialize => Some(HelperKind::Build),
        SiteTag::Invoke => Some(HelperKind::Find),
        SiteTag::Enter => match pre.img.control { ControlC::Enter { function: Some(_), .. } => Some(HelperKind::Bind), _ => None },
        SiteTag::Cases => match (pre.img.control, pre.expr(&pre.img.control_origin).ok()) {
            (ControlC::Cases { .. }, Some(ExpressionRowC::Match { plan: Some(_), .. })) => Some(HelperKind::Plan),
            (ControlC::Cases { arms, .. }, Some(ExpressionRowC::Match { plan: None, .. })) if arms.len > 0 => Some(HelperKind::Match),
            _ => None },
        _ => None,
    }
}
fn entry_rule(k: HelperKind) -> RuleIdC { match k { HelperKind::Scalar => RuleIdC::BHelperScalarEntry,
    HelperKind::EnvGet => RuleIdC::BHelperEnvGetEntry, HelperKind::EnvPut => RuleIdC::BHelperEnvPutEntry,
    HelperKind::Find => RuleIdC::BHelperFindEntry, HelperKind::Bind => RuleIdC::BHelperBindEntry,
    HelperKind::Append => RuleIdC::BHelperAppendEntry, HelperKind::Validate => RuleIdC::BHelperValidateEntry,
    HelperKind::Equal => RuleIdC::BHelperEqualEntry, HelperKind::Match => RuleIdC::BHelperMatchEntry,
    HelperKind::Plan => RuleIdC::XAdtHelperPlanEntry, HelperKind::Build => RuleIdC::XAdtHelperBuildEntry } }
fn event_kind_of(r: &RuleIdC) -> EventKind {
    use RuleIdC::*;
    match r {
        BHelperScalarEntry | BHelperEnvGetEntry | BHelperEnvPutEntry | BHelperFindEntry | BHelperBindEntry | BHelperAppendEntry | BHelperValidateEntry | BHelperEqualEntry | BHelperMatchEntry | XAdtHelperBuildEntry | XAdtHelperPlanEntry => EventKind::HelperEnter,
        BHelperScalarExit | BHelperEnvGetExit | BHelperEnvPutExit | BHelperFindExit | BHelperBindExit | BHelperAppendExit | BHelperValidateExit | BHelperEqualExit | BHelperMatchExit | XAdtHelperBuildExit | XAdtHelperPlanExit => EventKind::HelperExit,
        BHelperScalarFailure | BHelperEnvGetFailure | BHelperFindFailure | BHelperBindFailure | BHelperValidateFailure | BHelperMatchFailure | XAdtHelperPlanFailure => EventKind::HelperFail,
        BBookkeepingTaskRelease => EventKind::Bookkeeping,
        KAdminPublishValue | KAdminPublishError | KAdminPublishExhausted => EventKind::Publish,
        KAdminStartupValid | KAdminStartupInvalid => EventKind::Startup,
        KMachineEvalLiteralValid | KMachineEvalLiteralInvalid | KMachineEvalVariablePresent | KMachineEvalVariableAbsent | KMachineEvalLet | KMachineEvalBinary | KMachineEvalUnary | KMachineEvalIf | KMachineEvalMatch | KMachineEvalCallEmpty | KMachineEvalCallArguments | KMachinePushAccept | KMachinePushRefuse | KMachineReturnLet | KMachineReturnLeft | KMachineReturnRight | KMachineReturnUnary | KMachineReturnIfTrue | KMachineReturnIfFalse | KMachineReturnIfInvalid | KMachineReturnMatch | KMachineReturnArgumentsNext | KMachineReturnArgumentsFinal | KMachineReturnCallValid | KMachineReturnCallInvalid | KMachineReturnScope | KMachineInvokeFind | KMachineEnterMissing | KMachineEnterBind | KMachineBoundInvalid | KMachineBoundValid | KMachineCasesEmpty | KMachineCasesTry | KMachineCasesMiss | KMachineCasesHit | KMachinePrimitiveValue | KMachinePrimitiveError | XAdtEvalConstructNullary | XAdtEvalConstructFields | XAdtReturnCtorFieldsTyped | XAdtReturnCtorFieldsMistyped | XAdtReturnCtorFieldsNext | XAdtReturnCtorFieldsFinal | XAdtMaterializeDone | XAdtCasesPlanHit | XAdtCasesPlanMiss => EventKind::LegacyComplete,
        BHelperScalarTag | BHelperScalarArithmetic | BHelperScalarConstruct | BHelperEnvGetCell | BHelperEnvGetEmpty | BHelperEnvPutInsert | BHelperFindRow | BHelperFindEmpty | BHelperBindPair | BHelperBindEmpty | BHelperBindArity | BHelperBindType | BHelperBindDuplicate | BHelperBindInsert | BHelperBindPropagate | BHelperAppendCell | BHelperAppendEmpty | BHelperAppendTail | BHelperAppendCopy | BHelperValidateOccurrence | BHelperValidateField | BHelperValidateEmpty | BHelperValidateInvalid | BHelperEqualPair | BHelperEqualField | BHelperEqualEmpty | BHelperEqualDifferent | BHelperMatchArm | BHelperMatchOccurrence | BHelperMatchField | BHelperMatchBinding | BHelperMatchDistinct | BHelperMatchNext | BHelperMatchSelected | XAdtHelperBuildCell | XAdtHelperBuildField | XAdtHelperPlanWalk | XAdtHelperPlanSwitch | XAdtHelperPlanBind | XAdtHelperPlanSelect | XAdtHelperPlanNoMatch => EventKind::HelperVisit,
        BHelperEnvPutCell | BHelperEnvPutEmpty | BHelperEnvPutFailure | BHelperAppendFailure | BHelperEqualFailure | BHelperMatchEmpty | BHelperValidateDeclaration | BHelperMatchDiscard | BBookkeepingTaskEnqueue | BBookkeepingResumeStore | BBookkeepingResumeRestore => EventKind::HelperVisit
    }
}
fn visits_of(r: &RuleIdC) -> u8 {
    use RuleIdC::*;
    match r { BHelperScalarTag | BHelperScalarArithmetic | BHelperScalarConstruct | BHelperEnvGetCell
        | BHelperEnvPutInsert | BHelperFindRow | BHelperBindPair | BHelperBindType | BHelperBindDuplicate
        | BHelperBindInsert | BHelperBindPropagate | BHelperAppendCell | BHelperAppendTail | BHelperAppendCopy
        | BHelperValidateOccurrence | BHelperValidateField | BHelperValidateInvalid | BHelperEqualPair
        | BHelperEqualField | BHelperEqualDifferent | BHelperMatchArm | BHelperMatchOccurrence
        | BHelperMatchField | BHelperMatchBinding | BHelperMatchDistinct
        | XAdtHelperBuildCell | XAdtHelperBuildField | XAdtHelperPlanWalk | XAdtHelperPlanSwitch
        | XAdtHelperPlanBind | XAdtHelperPlanSelect | XAdtHelperPlanNoMatch => 1, _ => 0 }
}
fn family_of_rule(r: &RuleIdC) -> FamilyId {
    use RuleIdC::*;
    match r { BHelperScalarEntry | BHelperScalarTag | BHelperScalarArithmetic | BHelperScalarConstruct
        | BHelperScalarExit | BHelperScalarFailure => FamilyId::Scalar,
        BHelperEnvGetEntry | BHelperEnvGetCell | BHelperEnvGetEmpty | BHelperEnvGetExit | BHelperEnvGetFailure => FamilyId::EnvGet,
        BHelperEnvPutEntry | BHelperEnvPutInsert | BHelperEnvPutExit => FamilyId::EnvPut,
        BHelperFindEntry | BHelperFindRow | BHelperFindEmpty | BHelperFindExit | BHelperFindFailure => FamilyId::Find,
        BHelperBindEntry | BHelperBindPair | BHelperBindEmpty | BHelperBindArity | BHelperBindType
        | BHelperBindDuplicate | BHelperBindInsert | BHelperBindPropagate | BHelperBindExit | BHelperBindFailure => FamilyId::Bind,
        BHelperAppendEntry | BHelperAppendCell | BHelperAppendEmpty | BHelperAppendTail | BHelperAppendCopy | BHelperAppendExit => FamilyId::Append,
        BHelperValidateEntry | BHelperValidateOccurrence | BHelperValidateField | BHelperValidateEmpty
        | BHelperValidateInvalid | BHelperValidateExit | BHelperValidateFailure => FamilyId::Validate,
        BHelperEqualEntry | BHelperEqualPair | BHelperEqualField | BHelperEqualEmpty | BHelperEqualDifferent | BHelperEqualExit => FamilyId::Equal,
        BHelperMatchEntry | BHelperMatchArm | BHelperMatchOccurrence | BHelperMatchField | BHelperMatchBinding
        | BHelperMatchDistinct | BHelperMatchNext | BHelperMatchSelected | BHelperMatchExit | BHelperMatchFailure => FamilyId::Match,
        XAdtHelperBuildEntry | XAdtHelperBuildCell | XAdtHelperBuildField | XAdtHelperBuildExit => FamilyId::Build,
        XAdtHelperPlanEntry | XAdtHelperPlanWalk | XAdtHelperPlanSwitch | XAdtHelperPlanBind
        | XAdtHelperPlanSelect | XAdtHelperPlanNoMatch | XAdtHelperPlanExit | XAdtHelperPlanFailure => FamilyId::Plan,
        BBookkeepingTaskRelease => FamilyId::BookkeepingRelease, _ => FamilyId::Legacy }
}
fn selected_for(pre: &PreImage, rule: RuleIdC) -> Result<Selected, HostViolation> {
    check_enabled(&rule)?;
    let site = if pre.helper().is_some() || pre.result().is_some() { Some(owner_site(pre)?) }
        else { Some(owner_site(pre)?) };
    let hk = match event_kind_of(&rule) { EventKind::LegacyComplete => None,
        EventKind::Bookkeeping => match pre.helper() { Some(h) => Some(helper_kind_of(h)), None => None },
        _ => Some(match family_of_rule(&rule) { FamilyId::Scalar => HelperKind::Scalar, FamilyId::EnvGet => HelperKind::EnvGet,
            FamilyId::EnvPut => HelperKind::EnvPut, FamilyId::Find => HelperKind::Find, FamilyId::Bind => HelperKind::Bind,
            FamilyId::Append => HelperKind::Append, FamilyId::Validate => HelperKind::Validate,
            FamilyId::Equal => HelperKind::Equal, FamilyId::Match => HelperKind::Match,
            FamilyId::Build => HelperKind::Build, FamilyId::Plan => HelperKind::Plan,
            FamilyId::BookkeepingRelease => HelperKind::Validate, FamilyId::Legacy => HelperKind::Scalar }) };
    let hk = if event_kind_of(&rule) == EventKind::Bookkeeping {
        pre.helper().map(helper_kind_of) } else { hk };
    Ok(Selected { rule, event_kind: event_kind_of(&rule), site,
        helper_kind: if event_kind_of(&rule) == EventKind::LegacyComplete { None } else { hk },
        helper_eligible: event_kind_of(&rule) != EventKind::LegacyComplete, helper_visits: visits_of(&rule) })
}
fn helper_group(k: HelperKind) -> &'static [RuleIdC] {
    use RuleIdC::*;
    match k { HelperKind::Scalar => &[BHelperScalarConstruct, BHelperScalarArithmetic, BHelperScalarTag, BHelperScalarFailure, BHelperScalarExit],
        HelperKind::EnvGet => &[BHelperEnvGetExit, BHelperEnvGetFailure, BHelperEnvGetCell, BHelperEnvGetEmpty],
        HelperKind::EnvPut => &[BHelperEnvPutInsert, BHelperEnvPutExit],
        HelperKind::Find => &[BHelperFindExit, BHelperFindFailure, BHelperFindRow, BHelperFindEmpty],
        HelperKind::Bind => &[BHelperBindExit, BHelperBindFailure, BHelperBindPair, BHelperBindEmpty, BHelperBindArity, BHelperBindType, BHelperBindDuplicate, BHelperBindInsert, BHelperBindPropagate],
        HelperKind::Append => &[BHelperAppendCopy, BHelperAppendCell, BHelperAppendEmpty, BHelperAppendTail, BHelperAppendExit],
        HelperKind::Validate => &[BBookkeepingTaskRelease, BHelperValidateFailure, BHelperValidateExit, BHelperValidateEmpty, BHelperValidateInvalid, BHelperValidateOccurrence, BHelperValidateField],
        HelperKind::Equal => &[BBookkeepingTaskRelease, BHelperEqualExit, BHelperEqualEmpty, BHelperEqualDifferent, BHelperEqualPair, BHelperEqualField],
        HelperKind::Match => &[BHelperMatchExit, BHelperMatchFailure, BHelperMatchArm, BHelperMatchOccurrence, BHelperMatchDistinct, BHelperMatchField, BHelperMatchBinding, BHelperMatchSelected, BHelperMatchNext],
        HelperKind::Build => &[XAdtHelperBuildField, XAdtHelperBuildCell, XAdtHelperBuildExit],
        HelperKind::Plan => &[XAdtHelperPlanExit, XAdtHelperPlanFailure, XAdtHelperPlanWalk, XAdtHelperPlanSwitch, XAdtHelperPlanBind, XAdtHelperPlanSelect, XAdtHelperPlanNoMatch] }
}
fn legacy_group(site: SiteTag) -> &'static [RuleIdC] {
    use RuleIdC::*; use SiteTag::*;
    match site { EvalLiteral => &[KMachineEvalLiteralValid, KMachineEvalLiteralInvalid],
        EvalVar => &[KMachineEvalVariablePresent, KMachineEvalVariableAbsent], EvalLet => &[KMachineEvalLet],
        EvalBinary => &[KMachineEvalBinary], EvalUnary => &[KMachineEvalUnary], EvalIf => &[KMachineEvalIf],
        EvalMatch => &[KMachineEvalMatch], EvalCall => &[KMachineEvalCallEmpty, KMachineEvalCallArguments],
        EvalConstruct => &[XAdtEvalConstructNullary, XAdtEvalConstructFields],
        Push => &[KMachinePushAccept, KMachinePushRefuse], RetLet => &[KMachineReturnLet], RetLeft => &[KMachineReturnLeft],
        RetRight => &[KMachineReturnRight], RetUnary => &[KMachineReturnUnary],
        RetIf => &[KMachineReturnIfTrue, KMachineReturnIfFalse, KMachineReturnIfInvalid], RetMatch => &[KMachineReturnMatch],
        RetArgs => &[KMachineReturnArgumentsNext, KMachineReturnArgumentsFinal],
        RetCall => &[KMachineReturnCallValid, KMachineReturnCallInvalid], RetScope => &[KMachineReturnScope],
        RetCtorFields => &[XAdtReturnCtorFieldsTyped, XAdtReturnCtorFieldsMistyped, XAdtReturnCtorFieldsNext, XAdtReturnCtorFieldsFinal],
        Materialize => &[XAdtMaterializeDone],
        Invoke => &[KMachineInvokeFind], Enter => &[KMachineEnterMissing, KMachineEnterBind],
        Bound => &[KMachineBoundInvalid, KMachineBoundValid],
        Cases => &[KMachineCasesEmpty, KMachineCasesTry, XAdtCasesPlanHit, XAdtCasesPlanMiss],
        CaseResult => &[KMachineCasesMiss, KMachineCasesHit], Prim => &[KMachinePrimitiveValue, KMachinePrimitiveError] }
}
/// AB is already enforced by `PreImage::bind`; a non-Running image fails SI-18 here.
pub fn owner_select(pre: &PreImage, guards: &OwnerGuards) -> Result<Selected, HostViolation> {
    if pre.img.outcome != OutcomeC::Running {
        return Err(hv(HostViolationKind::StepInvViolated, Some(StepInvClauseId::SI18), None, "select on non-Running image")); }
    if let Some(h) = pre.helper() {
        for r in helper_group(helper_kind_of(h)) {
            if guards.eval_select_when(r, pre)? { return selected_for(pre, *r); } }
        return Err(hv(HostViolationKind::SelectUndefined, None, None, "helper group exhausted"));
    }
    let site = owner_site(pre)?;
    if pre.result().is_some() {
        for r in legacy_group(site) {
            if guards.reads_result_root(r) && guards.eval_select_when(r, pre)? { return selected_for(pre, *r); } }
        return Err(hv(HostViolationKind::SelectUndefined, None, None, "stale result"));
    }
    if let Some(k) = owner_helper_plan(site, pre) { return selected_for(pre, entry_rule(k)); }
    for r in legacy_group(site) {
        if !guards.reads_result_root(r) && guards.eval_select_when(r, pre)? { return selected_for(pre, *r); } }
    Err(hv(HostViolationKind::SelectUndefined, None, None, "legacy group exhausted"))
}
pub fn owner_family_of(raw: &RawRuleId) -> Option<FamilyId> {
    let r = owner_rule_id_of(raw);
    if is_disabled(&r) || is_extension(&r) || matches!(r, RuleIdC::KAdminStartupValid | RuleIdC::KAdminStartupInvalid
        | RuleIdC::KAdminPublishValue | RuleIdC::KAdminPublishError | RuleIdC::KAdminPublishExhausted) { None }
    else { Some(family_of_rule(&r)) }
}
pub fn owner_rule_id_of(raw: &RawRuleId) -> RuleIdC {
    use RawRuleId as R; use RuleIdC as C;
    match raw { R::KMachineEvalLiteralValid => C::KMachineEvalLiteralValid, R::KMachineEvalLiteralInvalid => C::KMachineEvalLiteralInvalid,
        R::KMachineEvalVariablePresent => C::KMachineEvalVariablePresent, R::KMachineEvalVariableAbsent => C::KMachineEvalVariableAbsent,
        R::KMachineEvalLet => C::KMachineEvalLet, R::KMachineEvalBinary => C::KMachineEvalBinary, R::KMachineEvalUnary => C::KMachineEvalUnary,
        R::KMachineEvalIf => C::KMachineEvalIf, R::KMachineEvalMatch => C::KMachineEvalMatch, R::KMachineEvalCallEmpty => C::KMachineEvalCallEmpty,
        R::KMachineEvalCallArguments => C::KMachineEvalCallArguments, R::KMachinePushAccept => C::KMachinePushAccept, R::KMachinePushRefuse => C::KMachinePushRefuse,
        R::KMachineReturnLet => C::KMachineReturnLet, R::KMachineReturnLeft => C::KMachineReturnLeft, R::KMachineReturnRight => C::KMachineReturnRight,
        R::KMachineReturnUnary => C::KMachineReturnUnary, R::KMachineReturnIfTrue => C::KMachineReturnIfTrue, R::KMachineReturnIfFalse => C::KMachineReturnIfFalse,
        R::KMachineReturnIfInvalid => C::KMachineReturnIfInvalid, R::KMachineReturnMatch => C::KMachineReturnMatch,
        R::KMachineReturnArgumentsNext => C::KMachineReturnArgumentsNext, R::KMachineReturnArgumentsFinal => C::KMachineReturnArgumentsFinal,
        R::KMachineReturnCallValid => C::KMachineReturnCallValid, R::KMachineReturnCallInvalid => C::KMachineReturnCallInvalid,
        R::KMachineReturnScope => C::KMachineReturnScope, R::KMachineInvokeFind => C::KMachineInvokeFind, R::KMachineEnterMissing => C::KMachineEnterMissing,
        R::KMachineEnterBind => C::KMachineEnterBind, R::KMachineBoundInvalid => C::KMachineBoundInvalid, R::KMachineBoundValid => C::KMachineBoundValid,
        R::KMachineCasesEmpty => C::KMachineCasesEmpty, R::KMachineCasesTry => C::KMachineCasesTry, R::KMachineCasesMiss => C::KMachineCasesMiss,
        R::KMachineCasesHit => C::KMachineCasesHit, R::KMachinePrimitiveValue => C::KMachinePrimitiveValue, R::KMachinePrimitiveError => C::KMachinePrimitiveError,
        R::BHelperScalarEntry => C::BHelperScalarEntry, R::BHelperScalarTag => C::BHelperScalarTag, R::BHelperScalarArithmetic => C::BHelperScalarArithmetic,
        R::BHelperScalarConstruct => C::BHelperScalarConstruct, R::BHelperScalarExit => C::BHelperScalarExit, R::BHelperScalarFailure => C::BHelperScalarFailure,
        R::BHelperEnvGetEntry => C::BHelperEnvGetEntry, R::BHelperEnvGetCell => C::BHelperEnvGetCell, R::BHelperEnvGetEmpty => C::BHelperEnvGetEmpty,
        R::BHelperEnvGetExit => C::BHelperEnvGetExit, R::BHelperEnvGetFailure => C::BHelperEnvGetFailure, R::BHelperEnvPutEntry => C::BHelperEnvPutEntry,
        R::BHelperEnvPutCell => C::BHelperEnvPutCell, R::BHelperEnvPutInsert => C::BHelperEnvPutInsert, R::BHelperEnvPutEmpty => C::BHelperEnvPutEmpty,
        R::BHelperEnvPutExit => C::BHelperEnvPutExit, R::BHelperEnvPutFailure => C::BHelperEnvPutFailure, R::BHelperFindEntry => C::BHelperFindEntry,
        R::BHelperFindRow => C::BHelperFindRow, R::BHelperFindEmpty => C::BHelperFindEmpty, R::BHelperFindExit => C::BHelperFindExit, R::BHelperFindFailure => C::BHelperFindFailure,
        R::BHelperBindEntry => C::BHelperBindEntry, R::BHelperBindPair => C::BHelperBindPair, R::BHelperBindEmpty => C::BHelperBindEmpty,
        R::BHelperBindArity => C::BHelperBindArity, R::BHelperBindType => C::BHelperBindType, R::BHelperBindDuplicate => C::BHelperBindDuplicate,
        R::BHelperBindInsert => C::BHelperBindInsert, R::BHelperBindPropagate => C::BHelperBindPropagate, R::BHelperBindExit => C::BHelperBindExit, R::BHelperBindFailure => C::BHelperBindFailure,
        R::BHelperAppendEntry => C::BHelperAppendEntry, R::BHelperAppendCell => C::BHelperAppendCell, R::BHelperAppendEmpty => C::BHelperAppendEmpty,
        R::BHelperAppendTail => C::BHelperAppendTail, R::BHelperAppendCopy => C::BHelperAppendCopy, R::BHelperAppendExit => C::BHelperAppendExit, R::BHelperAppendFailure => C::BHelperAppendFailure,
        R::BHelperValidateEntry => C::BHelperValidateEntry, R::BHelperValidateOccurrence => C::BHelperValidateOccurrence, R::BHelperValidateField => C::BHelperValidateField,
        R::BHelperValidateDeclaration => C::BHelperValidateDeclaration, R::BHelperValidateEmpty => C::BHelperValidateEmpty, R::BHelperValidateInvalid => C::BHelperValidateInvalid,
        R::BHelperValidateExit => C::BHelperValidateExit, R::BHelperValidateFailure => C::BHelperValidateFailure, R::BHelperEqualEntry => C::BHelperEqualEntry,
        R::BHelperEqualPair => C::BHelperEqualPair, R::BHelperEqualField => C::BHelperEqualField, R::BHelperEqualEmpty => C::BHelperEqualEmpty,
        R::BHelperEqualDifferent => C::BHelperEqualDifferent, R::BHelperEqualExit => C::BHelperEqualExit, R::BHelperEqualFailure => C::BHelperEqualFailure,
        R::BHelperMatchEntry => C::BHelperMatchEntry, R::BHelperMatchArm => C::BHelperMatchArm, R::BHelperMatchOccurrence => C::BHelperMatchOccurrence,
        R::BHelperMatchField => C::BHelperMatchField, R::BHelperMatchBinding => C::BHelperMatchBinding, R::BHelperMatchDistinct => C::BHelperMatchDistinct,
        R::BHelperMatchDiscard => C::BHelperMatchDiscard, R::BHelperMatchNext => C::BHelperMatchNext, R::BHelperMatchSelected => C::BHelperMatchSelected,
        R::BHelperMatchEmpty => C::BHelperMatchEmpty, R::BHelperMatchExit => C::BHelperMatchExit, R::BHelperMatchFailure => C::BHelperMatchFailure,
        R::BBookkeepingTaskEnqueue => C::BBookkeepingTaskEnqueue, R::BBookkeepingTaskRelease => C::BBookkeepingTaskRelease,
        R::BBookkeepingResumeStore => C::BBookkeepingResumeStore, R::BBookkeepingResumeRestore => C::BBookkeepingResumeRestore,
        R::KAdminStartupValid => C::KAdminStartupValid, R::KAdminStartupInvalid => C::KAdminStartupInvalid, R::KAdminPublishValue => C::KAdminPublishValue,
        R::KAdminPublishError => C::KAdminPublishError, R::KAdminPublishExhausted => C::KAdminPublishExhausted,
        R::XAdtEvalConstructNullary => C::XAdtEvalConstructNullary, R::XAdtEvalConstructFields => C::XAdtEvalConstructFields,
        R::XAdtReturnCtorFieldsTyped => C::XAdtReturnCtorFieldsTyped, R::XAdtReturnCtorFieldsMistyped => C::XAdtReturnCtorFieldsMistyped,
        R::XAdtReturnCtorFieldsNext => C::XAdtReturnCtorFieldsNext, R::XAdtReturnCtorFieldsFinal => C::XAdtReturnCtorFieldsFinal,
        R::XAdtMaterializeDone => C::XAdtMaterializeDone, R::XAdtCasesPlanHit => C::XAdtCasesPlanHit, R::XAdtCasesPlanMiss => C::XAdtCasesPlanMiss,
        R::XAdtHelperBuildEntry => C::XAdtHelperBuildEntry, R::XAdtHelperBuildCell => C::XAdtHelperBuildCell,
        R::XAdtHelperBuildField => C::XAdtHelperBuildField, R::XAdtHelperBuildExit => C::XAdtHelperBuildExit,
        R::XAdtHelperPlanEntry => C::XAdtHelperPlanEntry, R::XAdtHelperPlanWalk => C::XAdtHelperPlanWalk,
        R::XAdtHelperPlanSwitch => C::XAdtHelperPlanSwitch, R::XAdtHelperPlanBind => C::XAdtHelperPlanBind,
        R::XAdtHelperPlanSelect => C::XAdtHelperPlanSelect, R::XAdtHelperPlanNoMatch => C::XAdtHelperPlanNoMatch,
        R::XAdtHelperPlanExit => C::XAdtHelperPlanExit, R::XAdtHelperPlanFailure => C::XAdtHelperPlanFailure }
}
pub fn owner_raw_rule_id(rule: &RuleIdC) -> RawRuleId {
    use RawRuleId as R; use RuleIdC as C;
    match rule { C::KMachineEvalLiteralValid => R::KMachineEvalLiteralValid, C::KMachineEvalLiteralInvalid => R::KMachineEvalLiteralInvalid,
        C::KMachineEvalVariablePresent => R::KMachineEvalVariablePresent, C::KMachineEvalVariableAbsent => R::KMachineEvalVariableAbsent,
        C::KMachineEvalLet => R::KMachineEvalLet, C::KMachineEvalBinary => R::KMachineEvalBinary, C::KMachineEvalUnary => R::KMachineEvalUnary,
        C::KMachineEvalIf => R::KMachineEvalIf, C::KMachineEvalMatch => R::KMachineEvalMatch, C::KMachineEvalCallEmpty => R::KMachineEvalCallEmpty,
        C::KMachineEvalCallArguments => R::KMachineEvalCallArguments, C::KMachinePushAccept => R::KMachinePushAccept, C::KMachinePushRefuse => R::KMachinePushRefuse,
        C::KMachineReturnLet => R::KMachineReturnLet, C::KMachineReturnLeft => R::KMachineReturnLeft, C::KMachineReturnRight => R::KMachineReturnRight,
        C::KMachineReturnUnary => R::KMachineReturnUnary, C::KMachineReturnIfTrue => R::KMachineReturnIfTrue, C::KMachineReturnIfFalse => R::KMachineReturnIfFalse,
        C::KMachineReturnIfInvalid => R::KMachineReturnIfInvalid, C::KMachineReturnMatch => R::KMachineReturnMatch,
        C::KMachineReturnArgumentsNext => R::KMachineReturnArgumentsNext, C::KMachineReturnArgumentsFinal => R::KMachineReturnArgumentsFinal,
        C::KMachineReturnCallValid => R::KMachineReturnCallValid, C::KMachineReturnCallInvalid => R::KMachineReturnCallInvalid,
        C::KMachineReturnScope => R::KMachineReturnScope, C::KMachineInvokeFind => R::KMachineInvokeFind, C::KMachineEnterMissing => R::KMachineEnterMissing,
        C::KMachineEnterBind => R::KMachineEnterBind, C::KMachineBoundInvalid => R::KMachineBoundInvalid, C::KMachineBoundValid => R::KMachineBoundValid,
        C::KMachineCasesEmpty => R::KMachineCasesEmpty, C::KMachineCasesTry => R::KMachineCasesTry, C::KMachineCasesMiss => R::KMachineCasesMiss,
        C::KMachineCasesHit => R::KMachineCasesHit, C::KMachinePrimitiveValue => R::KMachinePrimitiveValue, C::KMachinePrimitiveError => R::KMachinePrimitiveError,
        C::BHelperScalarEntry => R::BHelperScalarEntry, C::BHelperScalarTag => R::BHelperScalarTag, C::BHelperScalarArithmetic => R::BHelperScalarArithmetic,
        C::BHelperScalarConstruct => R::BHelperScalarConstruct, C::BHelperScalarExit => R::BHelperScalarExit, C::BHelperScalarFailure => R::BHelperScalarFailure,
        C::BHelperEnvGetEntry => R::BHelperEnvGetEntry, C::BHelperEnvGetCell => R::BHelperEnvGetCell, C::BHelperEnvGetEmpty => R::BHelperEnvGetEmpty,
        C::BHelperEnvGetExit => R::BHelperEnvGetExit, C::BHelperEnvGetFailure => R::BHelperEnvGetFailure, C::BHelperEnvPutEntry => R::BHelperEnvPutEntry,
        C::BHelperEnvPutCell => R::BHelperEnvPutCell, C::BHelperEnvPutInsert => R::BHelperEnvPutInsert, C::BHelperEnvPutEmpty => R::BHelperEnvPutEmpty,
        C::BHelperEnvPutExit => R::BHelperEnvPutExit, C::BHelperEnvPutFailure => R::BHelperEnvPutFailure, C::BHelperFindEntry => R::BHelperFindEntry,
        C::BHelperFindRow => R::BHelperFindRow, C::BHelperFindEmpty => R::BHelperFindEmpty, C::BHelperFindExit => R::BHelperFindExit, C::BHelperFindFailure => R::BHelperFindFailure,
        C::BHelperBindEntry => R::BHelperBindEntry, C::BHelperBindPair => R::BHelperBindPair, C::BHelperBindEmpty => R::BHelperBindEmpty,
        C::BHelperBindArity => R::BHelperBindArity, C::BHelperBindType => R::BHelperBindType, C::BHelperBindDuplicate => R::BHelperBindDuplicate,
        C::BHelperBindInsert => R::BHelperBindInsert, C::BHelperBindPropagate => R::BHelperBindPropagate, C::BHelperBindExit => R::BHelperBindExit, C::BHelperBindFailure => R::BHelperBindFailure,
        C::BHelperAppendEntry => R::BHelperAppendEntry, C::BHelperAppendCell => R::BHelperAppendCell, C::BHelperAppendEmpty => R::BHelperAppendEmpty,
        C::BHelperAppendTail => R::BHelperAppendTail, C::BHelperAppendCopy => R::BHelperAppendCopy, C::BHelperAppendExit => R::BHelperAppendExit, C::BHelperAppendFailure => R::BHelperAppendFailure,
        C::BHelperValidateEntry => R::BHelperValidateEntry, C::BHelperValidateOccurrence => R::BHelperValidateOccurrence, C::BHelperValidateField => R::BHelperValidateField,
        C::BHelperValidateDeclaration => R::BHelperValidateDeclaration, C::BHelperValidateEmpty => R::BHelperValidateEmpty, C::BHelperValidateInvalid => R::BHelperValidateInvalid,
        C::BHelperValidateExit => R::BHelperValidateExit, C::BHelperValidateFailure => R::BHelperValidateFailure, C::BHelperEqualEntry => R::BHelperEqualEntry,
        C::BHelperEqualPair => R::BHelperEqualPair, C::BHelperEqualField => R::BHelperEqualField, C::BHelperEqualEmpty => R::BHelperEqualEmpty,
        C::BHelperEqualDifferent => R::BHelperEqualDifferent, C::BHelperEqualExit => R::BHelperEqualExit, C::BHelperEqualFailure => R::BHelperEqualFailure,
        C::BHelperMatchEntry => R::BHelperMatchEntry, C::BHelperMatchArm => R::BHelperMatchArm, C::BHelperMatchOccurrence => R::BHelperMatchOccurrence,
        C::BHelperMatchField => R::BHelperMatchField, C::BHelperMatchBinding => R::BHelperMatchBinding, C::BHelperMatchDistinct => R::BHelperMatchDistinct,
        C::BHelperMatchDiscard => R::BHelperMatchDiscard, C::BHelperMatchNext => R::BHelperMatchNext, C::BHelperMatchSelected => R::BHelperMatchSelected,
        C::BHelperMatchEmpty => R::BHelperMatchEmpty, C::BHelperMatchExit => R::BHelperMatchExit, C::BHelperMatchFailure => R::BHelperMatchFailure,
        C::BBookkeepingTaskEnqueue => R::BBookkeepingTaskEnqueue, C::BBookkeepingTaskRelease => R::BBookkeepingTaskRelease,
        C::BBookkeepingResumeStore => R::BBookkeepingResumeStore, C::BBookkeepingResumeRestore => R::BBookkeepingResumeRestore,
        C::KAdminStartupValid => R::KAdminStartupValid, C::KAdminStartupInvalid => R::KAdminStartupInvalid, C::KAdminPublishValue => R::KAdminPublishValue,
        C::KAdminPublishError => R::KAdminPublishError, C::KAdminPublishExhausted => R::KAdminPublishExhausted,
        C::XAdtEvalConstructNullary => R::XAdtEvalConstructNullary, C::XAdtEvalConstructFields => R::XAdtEvalConstructFields,
        C::XAdtReturnCtorFieldsTyped => R::XAdtReturnCtorFieldsTyped, C::XAdtReturnCtorFieldsMistyped => R::XAdtReturnCtorFieldsMistyped,
        C::XAdtReturnCtorFieldsNext => R::XAdtReturnCtorFieldsNext, C::XAdtReturnCtorFieldsFinal => R::XAdtReturnCtorFieldsFinal,
        C::XAdtMaterializeDone => R::XAdtMaterializeDone, C::XAdtCasesPlanHit => R::XAdtCasesPlanHit, C::XAdtCasesPlanMiss => R::XAdtCasesPlanMiss,
        C::XAdtHelperBuildEntry => R::XAdtHelperBuildEntry, C::XAdtHelperBuildCell => R::XAdtHelperBuildCell,
        C::XAdtHelperBuildField => R::XAdtHelperBuildField, C::XAdtHelperBuildExit => R::XAdtHelperBuildExit,
        C::XAdtHelperPlanEntry => R::XAdtHelperPlanEntry, C::XAdtHelperPlanWalk => R::XAdtHelperPlanWalk,
        C::XAdtHelperPlanSwitch => R::XAdtHelperPlanSwitch, C::XAdtHelperPlanBind => R::XAdtHelperPlanBind,
        C::XAdtHelperPlanSelect => R::XAdtHelperPlanSelect, C::XAdtHelperPlanNoMatch => R::XAdtHelperPlanNoMatch,
        C::XAdtHelperPlanExit => R::XAdtHelperPlanExit, C::XAdtHelperPlanFailure => R::XAdtHelperPlanFailure }
}

// Owner-observed SC-07 read provenance for one PS family-plan call.
struct RecordingPreImageView<'a> { inner: &'a PreImage<'a>, reads: std::cell::RefCell<Vec<RefC>>, env_ids: std::cell::RefCell<Vec<usize>>, slot_slices: std::cell::RefCell<Vec<SliceC>> }
impl<'a> RecordingPreImageView<'a> {
    fn new(inner: &'a PreImage<'a>) -> Self { Self { inner, reads: std::cell::RefCell::new(Vec::new()), env_ids: std::cell::RefCell::new(Vec::new()), slot_slices: std::cell::RefCell::new(Vec::new()) } }
    fn recorded(&self) -> Vec<RefC> { self.reads.borrow().clone() }
    fn recorded_envs(&self) -> Vec<usize> { self.env_ids.borrow().clone() }
    fn record(&self, r: RefC) { let mut v = self.reads.borrow_mut(); if !v.contains(&r) { v.push(r); } }
    fn record_env(&self, e: EnvRootC) { if let Some(id) = e { let mut v = self.env_ids.borrow_mut(); if !v.contains(&id) { v.push(id); } } }
    fn record_slice(&self, s: SliceC) { if s.len > 0 { let mut v = self.slot_slices.borrow_mut(); if !v.contains(&s) { v.push(s); } } }
    fn record_control(&self, c: &ControlC) { let mut refs = Vec::new(); control_refs(c, &mut refs); for r in refs { self.record(r); } match c { ControlC::Invoke { args, .. } | ControlC::Enter { args, .. } | ControlC::Materialize { fields: args, .. } => self.record_slice(*args), ControlC::ScheduleNext { frame, env, .. } => { self.record_frame(frame); self.record_env(*env); } ControlC::Bound { binding: BoundBinding::Ok { env }, .. } => self.record_env(*env), ControlC::Cases { saved_env, .. } => self.record_env(*saved_env), ControlC::CaseResult { matching, saved_env, .. } => { self.record_env(*saved_env); if let CaseMatching::Matched { env } = matching { self.record_env(*env); } } _ => {} } }
    fn record_frame(&self, f: &FrameRowC) { let mut refs = Vec::new(); frame_refs(f, &mut refs); for r in refs { self.record(r); } self.record_env(frame_env(f)); if let FrameRowC::ArgsF { completed, .. } | FrameRowC::ConstructorFieldsF { completed, .. } = f { self.record_slice(*completed); } }
    fn record_helper(&self, h: &HelperControlC) { let mut refs = Vec::new(); let mut exempt = Vec::new(); helper_refs(h, &mut refs, &mut exempt); for r in refs.into_iter().chain(exempt) { self.record(r); } match h { HelperControlC::EnvGet { cursor, .. } => self.record_env(*cursor), HelperControlC::EnvPut { base, new_root, .. } => { self.record_env(*base); self.record_env(*new_root); } HelperControlC::Bind { args, partial, dup_cursor, .. } => { self.record_slice(*args); self.record_env(*partial); self.record_env(*dup_cursor); } HelperControlC::Append { src, .. } | HelperControlC::Build { src, .. } => self.record_slice(*src), HelperControlC::Match { saved_env, partial, .. } | HelperControlC::Plan { saved_env, partial, .. } => { self.record_env(*saved_env); self.record_env(*partial); } _ => {} } }
    fn record_result(&self, r: &HelperResultC) { let mut refs = Vec::new(); result_refs(r, &mut refs); for x in refs { self.record(x); } match r { HelperResultC::EnvUpdated { cell } => self.record_env(Some(*cell)), HelperResultC::BindOk { env } | HelperResultC::MatchMatched { env } | HelperResultC::PlanMatched { env, .. } => self.record_env(*env), HelperResultC::AppendDone { seq } => self.record_slice(*seq), _ => {} } }
    fn record_task(&self, t: &HelperTaskC) { let mut refs = Vec::new(); let mut exempt = Vec::new(); task_refs(t, &mut refs, &mut exempt); for r in refs.into_iter().chain(exempt) { self.record(r); } }
    fn slot_contains(&self, pos: &usize) -> bool { self.slot_slices.borrow().iter().any(|s| *pos >= s.start && s.start.checked_add(s.len).map_or(false, |e| *pos < e)) }
}
impl<'a> PreImageView for RecordingPreImageView<'a> {
    fn adt_reads(&self) -> &dyn adt_runtime::AdtRuntimeReads { self }
    fn control(&self) -> &ControlC { let c = self.inner.control(); self.record_control(c); c }
    fn control_origin(&self) -> usize { self.inner.control_origin() }
    fn env_root(&self) -> EnvRootC { let e = self.inner.env_root(); self.record_env(e); e }
    fn depth(&self) -> usize { self.inner.depth() }
    fn task_live(&self) -> usize { self.inner.task_live() }
    fn stack_limit(&self) -> usize { self.inner.stack_limit() }
    fn outcome(&self) -> &OutcomeC { self.inner.outcome() }
    fn initial_fuel(&self) -> u128 { self.inner.initial_fuel() }
    fn remaining_work(&self) -> u128 { self.inner.remaining_work() }
    fn consumed_work(&self) -> u128 { self.inner.consumed_work() }
    fn helper_work(&self) -> u128 { self.inner.helper_work() }
    fn helper_visits(&self) -> u128 { self.inner.helper_visits() }
    fn reference_cursor(&self) -> u128 { self.inner.reference_cursor() }
    fn helper(&self) -> Option<&HelperControlC> { let h = self.inner.helper(); if let Some(h) = h { self.record_helper(h); } h }
    fn result(&self) -> Option<&HelperResultC> { let r = self.inner.result(); if let Some(r) = r { self.record_result(r); } r }
    fn top_frame(&self) -> Option<&FrameRowC> { let f = self.inner.top_frame(); if let Some(f) = f { self.record_frame(f); } f }
    fn top_task(&self) -> Option<&HelperTaskC> { let t = self.inner.top_task(); if let Some(t) = t { self.record_task(t); } t }
    fn active(&self) -> ActiveLengths { self.inner.active() }
    fn alloc(&self) -> AllocCounters { self.inner.alloc() }
    fn expr(&self, id: &usize) -> Result<&ExpressionRowC, HostViolation> { let row = self.inner.expr(id)?; if let ExpressionRowC::Literal { constant } = row { if *id == self.inner.control_origin() { self.record(RefC::Const(*constant)); } } Ok(row) }
    fn child(&self, pos: &usize) -> Result<usize, HostViolation> { self.inner.child(pos) }
    fn function_row(&self, id: &usize) -> Result<&FunctionRowC, HostViolation> { self.inner.function_row(id) }
    fn admitted_function_count(&self) -> usize { self.inner.admitted_function_count() }
    fn parameter(&self, pos: &usize) -> Result<&ParameterRowC, HostViolation> { self.inner.parameter(pos) }
    fn arm(&self, id: &usize) -> Result<&ArmRowC, HostViolation> { self.inner.arm(id) }
    fn pattern(&self, id: &usize) -> Result<&PatternRowC, HostViolation> { self.inner.pattern(id) }
    fn occurrence(&self, id: &usize) -> Result<&OccurrenceRowC, HostViolation> { self.inner.occurrence(id) }
    fn occurrence_path(&self, s: &SliceC) -> Result<&[usize], HostViolation> { self.inner.occurrence_path(s) }
    fn constant_borrowed(&self, id: &usize) -> Result<pure_leaves::BorrowedValueRow<'_>, HostViolation> { self.inner.constant_borrowed(id) }
    fn constant_field(&self, id: &usize) -> Result<usize, HostViolation> { self.inner.constant_field(id) }
    fn marker_free(&self, id: &usize) -> Result<bool, HostViolation> { self.inner.marker_free(id) }
    fn deref_borrowed(&self, r: &RefC) -> Result<pure_leaves::BorrowedValueRow<'_>, HostViolation> { self.inner.deref_borrowed(r) }
    fn field_at(&self, r: &RefC, i: &usize) -> Result<RefC, HostViolation> { let out = self.inner.field_at(r, i)?; if self.reads.borrow().contains(r) { self.record(out); } Ok(out) }
    fn env_cell(&self, id: &usize) -> Result<&EnvCellC, HostViolation> { let cell = self.inner.env_cell(id)?; if self.env_ids.borrow().contains(id) { self.record(cell.value); self.record_env(cell.previous); } Ok(cell) }
    fn value_slot(&self, pos: &usize) -> Result<RefC, HostViolation> { let out = self.inner.value_slot(pos)?; if self.slot_contains(pos) { self.record(out); } Ok(out) }
    fn kind_of(&self, r: &RefC) -> Result<LogicalKind, HostViolation> { self.inner.kind_of(r) }
    fn u128_of(&self, r: &RefC) -> Result<u128, HostViolation> { self.inner.u128_of(r) }
    fn bool_of(&self, r: &RefC) -> Result<bool, HostViolation> { self.inner.bool_of(r) }
    fn checked_arity(&self, k: &LogicalKind) -> Result<usize, HostViolation> { self.inner.checked_arity(k) }
    fn checked_sort(&self, k: &LogicalKind) -> Result<TypeIdC, HostViolation> { self.inner.checked_sort(k) }
    fn scalar_test_type_ok(&self, r: &RefC, expected: &TypeIdC) -> Result<bool, HostViolation> { self.inner.scalar_test_type_ok(r, expected) }
}
fn check_copied_refs(pre: &PreImage, w: &PlannedWrites, reads: &[RefC]) -> Result<(), HostViolation> {
    let fresh = if w.alloc_data.is_some() || w.alloc_constructor.is_some() { Some(RefC::Data(pre.img.active.data)) } else { None };
    let mut written: Vec<RefC> = Vec::new();
    if let Write::Set(c) = &w.control { control_refs(c, &mut written); }
    if let Some(f) = &w.frame { match f { FrameWrite::Push(x) | FrameWrite::RewriteTop(x) => frame_refs(x, &mut written), FrameWrite::Pop => {} } }
    if let Write::Set(Some(h)) = &w.helper { let mut exempt = Vec::new(); helper_refs(h, &mut written, &mut exempt); written.extend(exempt); }
    if let Write::Set(Some(r)) = &w.result { result_refs(r, &mut written); }
    if let Some(t) = &w.task_rewrite_top { let mut exempt = Vec::new(); task_refs(t, &mut written, &mut exempt); written.extend(exempt); }
    if let TaskOp::Push(t) = &w.task_op { let mut exempt = Vec::new(); task_refs(t, &mut written, &mut exempt); written.extend(exempt); }
    if let Some(a) = &w.alloc_data { for f in match a { AllocDataWrite::Cons { fields } | AllocDataWrite::Branch { fields } => fields.as_slice(), AllocDataWrite::Leaf { field } => std::slice::from_ref(field), _ => &[] } { written.push(*f); } }
    if let Some(f) = &w.alloc_data_field { if !reads.contains(f) { return Err(invalid_plan("SC-07 copied reference provenance")); } }
    if let Some(cell) = &w.alloc_env { written.push(cell.value); }
    if let Some(slot) = &w.alloc_value_slot { written.push(*slot); }
    for r in written { if Some(r) != fresh && !reads.contains(&r) { return Err(invalid_plan("SC-07 copied reference provenance")); } }
    Ok(())
}
// SC-07 for EnvId references: a written EnvId is a recorded pre-image read below active.env, or the fresh
// pre.active.env under alloc_env. alloc_env.previous is never fresh (own index is pre.active.env).
fn check_copied_envs(pre: &PreImage, w: &PlannedWrites, envs: &[usize]) -> Result<(), HostViolation> {
    let fresh = if w.alloc_env.is_some() { Some(pre.img.active.env) } else { None };
    let mut written: Vec<EnvRootC> = Vec::new();
    if let Write::Set(c) = &w.control { match c {
        ControlC::ScheduleNext { frame, env, .. } => { written.push(frame_env(frame)); written.push(*env); }
        ControlC::Bound { binding: BoundBinding::Ok { env }, .. } => written.push(*env),
        ControlC::Cases { saved_env, .. } => written.push(*saved_env),
        ControlC::CaseResult { matching, saved_env, .. } => { written.push(*saved_env); if let CaseMatching::Matched { env } = matching { written.push(*env); } }
        _ => {} } }
    if let Write::Set(e) = &w.env_root { written.push(*e); }
    if let Some(f) = &w.frame { match f { FrameWrite::Push(x) | FrameWrite::RewriteTop(x) => written.push(frame_env(x)), FrameWrite::Pop => {} } }
    if let Write::Set(Some(h)) = &w.helper { match h {
        HelperControlC::EnvGet { cursor, .. } => written.push(*cursor),
        HelperControlC::EnvPut { base, new_root, .. } => { written.push(*base); written.push(*new_root); }
        HelperControlC::Bind { partial, dup_cursor, .. } => { written.push(*partial); written.push(*dup_cursor); }
        HelperControlC::Match { saved_env, partial, .. } | HelperControlC::Plan { saved_env, partial, .. } => { written.push(*saved_env); written.push(*partial); }
        _ => {} } }
    if let Write::Set(Some(r)) = &w.result { match r {
        HelperResultC::EnvUpdated { cell } => written.push(Some(*cell)),
        HelperResultC::BindOk { env } | HelperResultC::MatchMatched { env } | HelperResultC::PlanMatched { env, .. } => written.push(*env),
        _ => {} } }
    let copied = |id: usize| id < pre.img.active.env && envs.contains(&id);
    for id in written.into_iter().flatten() { if Some(id) != fresh && !copied(id) { return Err(invalid_plan("SC-07 copied environment provenance")); } }
    if let Some(p) = w.alloc_env.and_then(|cell| cell.previous) { if !copied(p) { return Err(invalid_plan("SC-07 alloc_env.previous provenance")); } }
    Ok(())
}
// SC-05 for code ids: every written ExprId, ArmId, FunctionRowId, ConstId and NameId is below its admitted program
// table (control_origin keeps its own check). Match x/y are NameIds only where pat_tag is active outside phase Arm;
// their inactive zero columns are SI-22, not range-checked here. Data refs and EnvIds stay with SC-07/SI-13.
fn frame_code_ids(f: &FrameRowC, exprs: &mut Vec<usize>, names: &mut Vec<usize>) {
    exprs.push(frame_origin(f));
    match f { FrameRowC::LetF { binder, body, .. } => { names.push(*binder); exprs.push(*body); }
        FrameRowC::LeftF { right, .. } => exprs.push(*right),
        FrameRowC::IfF { then_expr, else_expr, .. } => { exprs.push(*then_expr); exprs.push(*else_expr); }
        FrameRowC::ArgsF { function_name, .. } => names.push(*function_name),
        FrameRowC::RightF { .. } | FrameRowC::UnaryF { .. } | FrameRowC::MatchF { .. }
        | FrameRowC::ConstructorFieldsF { .. } | FrameRowC::ReturnF { .. } | FrameRowC::ScopeF { .. } => {} }
}
fn check_written_frame_adt_ids(view: &ProgramView, f: &FrameRowC) -> Result<(), HostViolation> {
    if let FrameRowC::ConstructorFieldsF { type_id, ctor_id, field_type, .. } = f {
        if adt_checked_ctor_arity(view, *type_id, *ctor_id).is_err() || field_type.0 >= view.types.len() {
            return Err(invalid_plan("SC-05 written code id"));
        }
    }
    Ok(())
}
fn check_written_code_ids(pre: &PreImage, w: &PlannedWrites) -> Result<(), HostViolation> {
    let view = &pre.admitted.body.view;
    let mut exprs: Vec<usize> = Vec::new(); let mut fns: Vec<usize> = Vec::new(); let mut names: Vec<usize> = Vec::new();
    let mut arms: Vec<usize> = Vec::new(); let mut refs: Vec<RefC> = Vec::new();
    if let Write::Set(c) = &w.control { control_refs(c, &mut refs); match c {
        ControlC::Eval { expr } => exprs.push(*expr),
        ControlC::ScheduleNext { frame, next, .. } => {
            check_written_frame_adt_ids(view, frame)?;
            frame_code_ids(frame, &mut exprs, &mut names);
            exprs.push(*next);
        }
        ControlC::Invoke { function_name, .. } => names.push(*function_name),
        ControlC::Enter { function, .. } => fns.extend(*function),
        ControlC::Materialize { type_id, ctor_id, .. } => {
            if adt_checked_ctor_arity(view, *type_id, *ctor_id).is_err() {
                return Err(invalid_plan("SC-05 written code id"));
            }
        }
        ControlC::Bound { body, .. } | ControlC::CaseResult { body, .. } => exprs.push(*body),
        ControlC::Return { .. } | ControlC::Cases { .. } | ControlC::PrimResult { .. } | ControlC::Error { .. } => {} } }
    if let Some(f) = &w.frame { match f { FrameWrite::Push(x) | FrameWrite::RewriteTop(x) => {
        check_written_frame_adt_ids(view, x)?;
        frame_refs(x, &mut refs);
        frame_code_ids(x, &mut exprs, &mut names);
    } FrameWrite::Pop => {} } }
    if let Write::Set(Some(h)) = &w.helper { let mut exempt = Vec::new(); helper_refs(h, &mut refs, &mut exempt); refs.extend(exempt); match h {
        HelperControlC::EnvGet { binder, .. } | HelperControlC::EnvPut { binder, .. } => names.push(*binder),
        HelperControlC::Find { function_name, found, .. } => { names.push(*function_name); fns.extend(*found); }
        HelperControlC::Bind { function, .. } => fns.push(*function),
        HelperControlC::Match { arm, pat_tag, x, y, phase, .. } => { arms.push(*arm); if *phase != MatchPhase::Arm { match pat_tag {
            PatternTag::PVar | PatternTag::PLeaf => names.push(*x),
            PatternTag::PCons | PatternTag::PBranch => { names.push(*x); names.push(*y); }
            PatternTag::PNil | PatternTag::WildcardExtensionReserved | PatternTag::ConstructorExtensionReserved => {} } } }
        HelperControlC::Plan { source_match, node, arm, .. } => {
            exprs.push(*source_match);
            if *node >= view.plan_nodes.len() { return Err(invalid_plan("SC-05 written code id")); }
            arms.extend(*arm);
        }
        HelperControlC::Build { type_id, ctor_id, .. } => {
            if adt_checked_ctor_arity(view, *type_id, *ctor_id).is_err() {
                return Err(invalid_plan("SC-05 written code id"));
            }
        }
        HelperControlC::Scalar { .. } | HelperControlC::Append { .. } | HelperControlC::Validate { .. } | HelperControlC::Equal { .. } => {} } }
    if let Write::Set(Some(r)) = &w.result {
        result_refs(r, &mut refs);
        match r {
            HelperResultC::FindFound { function } => fns.push(*function),
            HelperResultC::PlanMatched { arm, .. } => arms.push(*arm),
            _ => {}
        }
    }
    let pushed = match &w.task_op { TaskOp::Push(t) => Some(t), TaskOp::None | TaskOp::Pop => None };
    for t in w.task_rewrite_top.iter().chain(pushed) { let mut exempt = Vec::new(); task_refs(t, &mut refs, &mut exempt); refs.extend(exempt);
        if let HelperTaskC::PairTask { binder, .. } = t { names.push(*binder); } }
    if let Some(a) = &w.alloc_data { for f in match a { AllocDataWrite::Cons { fields } | AllocDataWrite::Branch { fields } => fields.as_slice(), AllocDataWrite::Leaf { field } => std::slice::from_ref(field), _ => &[] } { refs.push(*f); } }
    if let Some(f) = &w.alloc_data_field { refs.push(*f); }
    if let Some(c) = &w.alloc_constructor {
        if adt_checked_ctor_arity(view, c.type_id, c.ctor_id).is_err() {
            return Err(invalid_plan("SC-05 written code id"));
        }
    }
    if let Some(cell) = &w.alloc_env { names.push(cell.binder); refs.push(cell.value); }
    if let Some(slot) = &w.alloc_value_slot { refs.push(*slot); }
    let n_const = view.constants.len();
    if exprs.iter().any(|e| *e >= view.expressions.len()) || fns.iter().any(|f| *f >= view.functions.len())
        || names.iter().any(|n| *n >= view.names.len()) || arms.iter().any(|a| *a >= view.arms.len())
        || refs.iter().any(|r| matches!(r, RefC::Const(id) if *id >= n_const)) {
        return Err(invalid_plan("SC-05 written code id")); }
    Ok(())
}
// ================= validate_plan / cap / physical =================
fn slice_ok(s: SliceC, active: usize) -> bool {
    if s.len == 0 { s.start == 0 } else { s.start.checked_add(s.len).map_or(false, |e| e <= active) }
}
fn derived_delta(w: &PlannedWrites) -> PlanDelta {
    PlanDelta { alloc_data_nodes: if w.alloc_data.is_some() || w.alloc_constructor.is_some() { 1 } else { 0 },
        alloc_data_fields: w.alloc_data.map_or(0, |a| a.field_len() as i64) + if w.alloc_data_field.is_some() { 1 } else { 0 },
        alloc_environment_cells: if w.alloc_env.is_some() { 1 } else { 0 },
        depth: match w.frame { Some(FrameWrite::Push(_)) => 1, Some(FrameWrite::Pop) => -1, _ => 0 },
        task_live: match w.task_op { TaskOp::Push(_) => 1, TaskOp::Pop => -1, TaskOp::None => 0 },
        active_value_slots: if w.alloc_value_slot.is_some() { 1 } else { 0 } }
}
fn check_slice(s: SliceC, active: usize) -> Result<(), HostViolation> {
    if slice_ok(s, active) { Ok(()) } else { Err(invalid_plan("slice normalization")) }
}
fn check_ref(pre: &PreImage, r: &RefC, allow_new_data: bool) -> Result<(), HostViolation> {
    match r { RefC::Const(id) => if *id < pre.admitted.body.view.constants.len() { Ok(()) } else { Err(invalid_plan("const ref")) },
        RefC::Data(id) => if *id < pre.img.active.data || (allow_new_data && *id == pre.img.active.data) { Ok(()) } else { Err(invalid_plan("data ref")) } }
}
fn check_any_ref(pre: &PreImage, r: &RefC) -> Result<(), HostViolation> { check_ref(pre, r, true) }
fn post_slot_len(pre: &PreImage, w: &PlannedWrites) -> Result<usize, HostViolation> {
    if w.alloc_value_slot.is_some() {
        pre.img.active.value_slots.checked_add(1).ok_or_else(|| invalid_plan("slice normalization"))
    } else {
        Ok(pre.img.active.value_slots)
    }
}
fn validate_slices_in_control(pre: &PreImage, w: &PlannedWrites, c: &ControlC) -> Result<(), HostViolation> {
    match c { ControlC::Invoke { args, .. } | ControlC::Enter { args, .. } => check_slice(*args, pre.img.active.value_slots),
        ControlC::Materialize { fields, .. } => check_slice(*fields, post_slot_len(pre, w)?),
        ControlC::Cases { arms, .. } => check_slice(*arms, pre.admitted.body.view.arms.len()),
        ControlC::CaseResult { rest, .. } => check_slice(*rest, pre.admitted.body.view.arms.len()),
        ControlC::ScheduleNext { frame, .. } => validate_slices_in_frame(pre, w, frame), _ => Ok(()) }
}
fn validate_slices_in_frame(pre: &PreImage, w: &PlannedWrites, f: &FrameRowC) -> Result<(), HostViolation> {
    match f { FrameRowC::MatchF { arms, .. } => check_slice(*arms, pre.admitted.body.view.arms.len()),
        FrameRowC::ArgsF { completed, rest, .. } => { check_slice(*completed, pre.img.active.value_slots)?; check_slice(*rest, pre.admitted.body.view.children.len()) }
        FrameRowC::ConstructorFieldsF { completed, rest, .. } => { check_slice(*completed, post_slot_len(pre, w)?)?; check_slice(*rest, pre.admitted.body.view.children.len()) }
        _ => Ok(()) }
}
fn validate_slices_in_helper(pre: &PreImage, h: &HelperControlC) -> Result<(), HostViolation> {
    match h { HelperControlC::Bind { params, args, .. } => { check_slice(*params, pre.admitted.body.view.parameters.len())?; check_slice(*args, pre.img.active.value_slots) }
        HelperControlC::Append { src, .. } | HelperControlC::Build { src, .. } => check_slice(*src, pre.img.active.value_slots),
        HelperControlC::Plan { path, .. } => check_slice(*path, pre.admitted.body.view.field_paths.len()),
        _ => Ok(()) }
}
pub fn owner_validate_plan(pre: &PreImage, sel: &Selected, plan: EffectPlan, read_provenance: &[RefC], read_env_provenance: &[usize]) -> Result<ValidatedPlan, HostViolation> {
    check_enabled(&sel.rule)?;
    if sel.event_kind != event_kind_of(&sel.rule) || sel.helper_visits != visits_of(&sel.rule)
        || sel.helper_eligible != (event_kind_of(&sel.rule) != EventKind::LegacyComplete) {
        return Err(invalid_plan("Selected header")); }
    let w = &plan.writes;
    if derived_delta(w) != plan.delta_assert { return Err(invalid_plan("delta vs writes")); }
    check_copied_refs(pre, w, read_provenance)?;
    check_copied_envs(pre, w, read_env_provenance)?;
    // Frame/task existence before charge.
    match w.frame { Some(FrameWrite::RewriteTop(_)) | Some(FrameWrite::Pop) if pre.depth() == 0 =>
        return Err(invalid_plan("frame existence")), _ => {} }
    if (w.task_rewrite_top.is_some() || matches!(w.task_op, TaskOp::Pop)) && pre.task_live() == 0 {
        return Err(invalid_plan("task existence")); }
    // Slice normalization everywhere a slice is written.
    if let Write::Set(c) = &w.control { validate_slices_in_control(pre, w, c)?; }
    if let Some(f) = &w.frame { match f { FrameWrite::Push(x) | FrameWrite::RewriteTop(x) => validate_slices_in_frame(pre, w, x)?, FrameWrite::Pop => {} } }
    if let Write::Set(Some(h)) = &w.helper {
        validate_slices_in_helper(pre, h)?;
        if helper_common(h).parent_resume.is_some() {
            return Err(hv(HostViolationKind::StepInvViolated, Some(StepInvClauseId::SI08), None, "parentResume present")); }
        if let Some(want) = sel.helper_kind { if helper_kind_of(h) != want && event_kind_of(&sel.rule) != EventKind::Bookkeeping {
            return Err(invalid_plan("helper kind")); } }
    }
    if let Write::Set(Some(HelperResultC::AppendDone { seq })) = &w.result { check_slice(*seq, pre.img.active.value_slots.checked_add(1).ok_or_else(|| invalid_plan("slice normalization"))?)?; }
    // New identities are exactly the pre-image active lengths; split constructor writes obey the Build schedule.
    if w.alloc_data.is_some() && (w.alloc_data_field.is_some() || w.alloc_constructor.is_some()) {
        return Err(invalid_plan("conflicting data alloc"));
    }
    if let Some(a) = &w.alloc_data {
        for f in match a { AllocDataWrite::Cons { fields } | AllocDataWrite::Branch { fields } => fields.as_slice(),
            AllocDataWrite::Leaf { field } => std::slice::from_ref(field), _ => &[] } { check_ref(pre, f, false)?; }
    }
    if let Some(df) = &w.alloc_data_field {
        check_ref(pre, df, false)?;
        if sel.rule != RuleIdC::XAdtHelperBuildField { return Err(invalid_plan("alloc_data_field rule")); }
        match pre.helper() {
            Some(HelperControlC::Build { src, src_cursor, field_start, staged: Some(st), node: None, phase: BuildPhase::Copy, .. })
                if *st == *df && *src_cursor < src.len
                    && field_start.checked_add(*src_cursor) == Some(pre.img.active.data_fields)
                    && ((*src_cursor + 1 == src.len) == w.alloc_constructor.is_some()) => {}
            _ => return Err(invalid_plan("alloc_data_field state")),
        }
    }
    if let Some(cw) = &w.alloc_constructor {
        match (&sel.rule, pre.helper()) {
            (RuleIdC::XAdtHelperBuildField, Some(HelperControlC::Build {
                type_id, ctor_id, src, src_cursor, field_start, staged: Some(_), node: None, phase: BuildPhase::Copy, ..
            })) if w.alloc_data_field.is_some() && src_cursor.checked_add(1) == Some(src.len)
                && cw.type_id == *type_id && cw.ctor_id == *ctor_id
                && cw.fields == SliceC::normalized(*field_start, src.len) => {}
            _ => return Err(invalid_plan("alloc_constructor state")),
        }
    }
    if let Write::Set(Some(r)) = &w.result {
        match r { HelperResultC::ScalarValue { value } | HelperResultC::EqualDone { value } if w.alloc_data.is_some() =>
                if *value != RefC::Data(pre.img.active.data) { return Err(invalid_plan("new data id")); },
            HelperResultC::EnvUpdated { cell } if w.alloc_env.is_some() =>
                if *cell != pre.img.active.env { return Err(invalid_plan("new env id")); },
            _ => {} }
    }
    if let Write::Set(Some(h)) = &w.helper {
        match h { HelperControlC::EnvPut { new_root: Some(n), .. } | HelperControlC::Bind { partial: Some(n), .. }
            | HelperControlC::Match { partial: Some(n), .. } | HelperControlC::Plan { partial: Some(n), .. }
            if w.alloc_env.is_some() && *n != pre.img.active.env =>
            return Err(invalid_plan("new env id in helper")),
            HelperControlC::Build { node: Some(n), .. } if w.alloc_constructor.is_some() && *n != RefC::Data(pre.img.active.data) =>
            return Err(invalid_plan("new constructor data id in helper")),
            _ => {} }
    }
    // Error-code ranges carried by writes.
    if let Write::Set(ControlC::Error { code }) = &w.control { if !(1..=11).contains(code) { return Err(invalid_plan("error code")); } }
    if let Write::Set(ControlC::PrimResult { result: PrimOutcome::Error { code } }) = &w.control {
        if !matches!(code, 10 | 11) { return Err(invalid_plan("prim error code")); } }
    if let Write::Set(c) = &w.control_origin { if *c >= pre.admitted.body.view.expressions.len() { return Err(invalid_plan("control origin")); } }
    // SC-05: every other written code id (ExprId, ArmId, FunctionRowId, ConstId, NameId) against its program table.
    check_written_code_ids(pre, w)?;
    // Result/helper membership for exit, failure, entry and legacy-consuming rows.
    let ek = event_kind_of(&sel.rule);
    if ek == EventKind::HelperEnter && !matches!(&w.helper, Write::Set(Some(_))) { return Err(invalid_plan("entry helper")); }
    if matches!(ek, EventKind::HelperExit | EventKind::HelperFail) {
        if !matches!(&w.helper, Write::Set(None)) || !matches!(&w.result, Write::Set(Some(_))) {
            return Err(invalid_plan("exit result")); } }
    if ek == EventKind::LegacyComplete {
        // A row whose select_when reads r must clear the staged result.
        // (Root's GuardEvaluator is not a parameter of the trait-shaped
        // validate_plan, so the fixed AST table in OwnerGuards is mirrored
        // by this literal set, identical to reads_result_root above.)
        let reads_r = matches!(sel.rule, RuleIdC::KMachineEvalLiteralValid | RuleIdC::KMachineEvalLiteralInvalid
            | RuleIdC::KMachineEvalVariablePresent | RuleIdC::KMachineEvalVariableAbsent | RuleIdC::KMachineReturnLet
            | RuleIdC::KMachineReturnRight | RuleIdC::KMachineReturnUnary | RuleIdC::KMachineReturnIfTrue
            | RuleIdC::KMachineReturnIfFalse | RuleIdC::KMachineReturnIfInvalid | RuleIdC::KMachineReturnArgumentsNext
            | RuleIdC::KMachineReturnArgumentsFinal | RuleIdC::KMachineReturnCallValid | RuleIdC::KMachineReturnCallInvalid
            | RuleIdC::KMachineInvokeFind | RuleIdC::KMachineEnterBind | RuleIdC::KMachineCasesTry
            | RuleIdC::XAdtReturnCtorFieldsTyped | RuleIdC::XAdtReturnCtorFieldsMistyped
            | RuleIdC::XAdtReturnCtorFieldsNext | RuleIdC::XAdtReturnCtorFieldsFinal
            | RuleIdC::XAdtMaterializeDone | RuleIdC::XAdtCasesPlanHit | RuleIdC::XAdtCasesPlanMiss);
        if reads_r && !matches!(&w.result, Write::Set(None)) { return Err(invalid_plan("result not cleared")); }
        if pre.helper().is_some() { return Err(invalid_plan("legacy with helper")); }
    }
    // P6 cap_refusal, fixed order, computed purely in PS from pre counters.
    let b = &pre.admitted.body.bounds; let d = &plan.delta_assert;
    let cap_first = if d.alloc_data_nodes > 0 && pre.img.alloc.data_nodes.checked_add(d.alloc_data_nodes as usize).map_or(true, |n| n > b.data_nodes) { Some(CapKind::DataNodes) }
        else if d.alloc_data_fields > 0 && pre.img.alloc.data_fields.checked_add(d.alloc_data_fields as usize).map_or(true, |n| n > b.data_field_slots) { Some(CapKind::DataFields) }
        else if d.alloc_environment_cells > 0 && pre.img.alloc.environment_cells.checked_add(d.alloc_environment_cells as usize).map_or(true, |n| n > b.environment_cells) { Some(CapKind::EnvironmentCells) }
        else if d.task_live > 0 && pre.task_live().checked_add(d.task_live as usize).map_or(true, |n| n > b.helper_tasks) { Some(CapKind::HelperTasks) }
        else { None };
    Ok(ValidatedPlan::from_validated(plan.writes, plan.delta_assert, cap_first, sel.rule))
}
pub fn owner_cap_refusal(v: &ValidatedPlan) -> Option<CapKind> { v.cap_first }
pub fn owner_physical_check(pre: &PreImage, v: &ValidatedPlan) -> Result<(), HostViolation> {
    if v.cap_first.is_some() { return Ok(()); } // P6 refusal: no physical check applies.
    let b = &pre.admitted.body.bounds; let w = &v.writes;
    let bad = ((w.alloc_data.is_some() || w.alloc_constructor.is_some()) && pre.img.active.data >= b.data_nodes)
        || (v.delta.alloc_data_fields > 0 && pre.img.active.data_fields.checked_add(v.delta.alloc_data_fields as usize).map_or(true, |n| n > b.data_field_slots))
        || (w.alloc_env.is_some() && pre.img.active.env >= b.environment_cells)
        || (w.alloc_value_slot.is_some() && pre.img.active.value_slots >= pre.admitted.body.layout.capacities.value_slots)
        || (matches!(w.frame, Some(FrameWrite::Push(_))) && pre.depth() >= b.frames)
        || (matches!(w.task_op, TaskOp::Push(_)) && pre.task_live() >= b.helper_tasks);
    if bad { Err(hv(HostViolationKind::PhysicalCapacityExceeded, None, Some(v.rule()), "physical capacity")) } else { Ok(()) }
}

// ================= Event construction (owner-resolution candidate) =================
#[derive(Clone)]
struct Snap { consumed: u128, cursor: u128, origin: usize, helper: Option<HelperControlC>,
    result: Option<HelperResultC>, result_plan: Option<HelperKind>, depth: usize, tasks: usize,
    source_occurrence: LocatorC }
// result_plan = helperPlan(site(pre), pre) for a staged pre.result (adopted helperKind branch 3), computed purely before any write.
fn snap(pre: &PreImage) -> Result<Snap, HostViolation> {
    let img = pre.img;
    let result_plan = if img.result.is_some() { owner_helper_plan(owner_site(pre)?, pre) } else { None };
    let occurrence = pre.occurrence(&img.control_origin)?;
    let source_occurrence = LocatorC::Occurrence { owner_is_function: occurrence.owner_is_function,
        function: occurrence.function, ast_path: pre.occurrence_path(&occurrence.path)?.to_vec() };
    Ok(Snap { consumed: img.consumed_work, cursor: img.reference_cursor,
        origin: img.control_origin, helper: img.helper, result: img.result, result_plan, depth: img.frames.len(), tasks: img.tasks.len(), source_occurrence }) }
fn phase_of(h: &HelperControlC) -> HelperPhaseSummaryC { match h {
    HelperControlC::Scalar { phase, .. } => HelperPhaseSummaryC::Scalar(*phase),
    HelperControlC::Bind { phase, .. } => HelperPhaseSummaryC::Bind(*phase),
    HelperControlC::Append { phase, .. } => HelperPhaseSummaryC::Append(*phase),
    HelperControlC::Match { phase, .. } => HelperPhaseSummaryC::Match(*phase),
    HelperControlC::Plan { phase, .. } => HelperPhaseSummaryC::Plan(*phase),
    HelperControlC::Build { phase, .. } => HelperPhaseSummaryC::Build(*phase),
    HelperControlC::EnvGet { status, .. } | HelperControlC::EnvPut { status, .. } | HelperControlC::Find { status, .. }
    | HelperControlC::Validate { status, .. } | HelperControlC::Equal { status, .. } => HelperPhaseSummaryC::Tri(*status) } }
fn cursor_of(h: &HelperControlC, tasks: usize) -> Option<usize> { match h {
    HelperControlC::Scalar { phase, .. } => Some(match phase { ScalarPhase::First => 0, ScalarPhase::Second => 1, ScalarPhase::Done => 2 }),
    HelperControlC::EnvGet { cursor, .. } => *cursor, HelperControlC::EnvPut { new_root, .. } => *new_root,
    HelperControlC::Find { row_cursor, .. } => Some(*row_cursor), HelperControlC::Bind { p_cursor, .. } => Some(*p_cursor),
    HelperControlC::Append { src_cursor, .. } | HelperControlC::Build { src_cursor, .. } => Some(*src_cursor),
    HelperControlC::Validate { .. } | HelperControlC::Equal { .. } => Some(tasks),
    HelperControlC::Match { field_idx, .. } => Some(usize::from(*field_idx)),
    HelperControlC::Plan { node, .. } => Some(*node) } }
fn result_tag(r: &HelperResultC) -> HelperResultTagC { match r { HelperResultC::Validated { .. } => HelperResultTagC::Validated,
    HelperResultC::EnvFound { .. } => HelperResultTagC::EnvFound, HelperResultC::EnvMissing => HelperResultTagC::EnvMissing,
    HelperResultC::EnvUpdated { .. } => HelperResultTagC::EnvUpdated, HelperResultC::FindFound { .. } => HelperResultTagC::FindFound,
    HelperResultC::FindMissing => HelperResultTagC::FindMissing, HelperResultC::BindOk { .. } => HelperResultTagC::BindOk,
    HelperResultC::BindBad => HelperResultTagC::BindBad, HelperResultC::AppendDone { .. } => HelperResultTagC::AppendDone,
    HelperResultC::ScalarValue { .. } => HelperResultTagC::ScalarValue, HelperResultC::ScalarError { .. } => HelperResultTagC::ScalarError,
    HelperResultC::ScalarSelector { .. } => HelperResultTagC::ScalarSelector, HelperResultC::ScalarTypeTest { .. } => HelperResultTagC::ScalarTypeTest,
    HelperResultC::EqualDone { .. } => HelperResultTagC::EqualDone, HelperResultC::MatchMatched { .. } => HelperResultTagC::MatchMatched,
    HelperResultC::MatchNoMatch => HelperResultTagC::MatchNoMatch, HelperResultC::PlanMatched { .. } => HelperResultTagC::PlanMatched,
    HelperResultC::PlanNoMatch => HelperResultTagC::PlanNoMatch, HelperResultC::BuildDone { .. } => HelperResultTagC::BuildDone } }
fn outcome_tag(o: &OutcomeC) -> OutcomeTagC { match o { OutcomeC::Running => OutcomeTagC::Running,
    OutcomeC::Halted { .. } => OutcomeTagC::Halted, OutcomeC::Rejected { .. } => OutcomeTagC::Rejected,
    OutcomeC::Exhausted { .. } => OutcomeTagC::Exhausted } }
fn build_event(_a: &Admitted, pre: &Snap, img: &MachineImage, ordinal: u128, rule: RuleIdC, kind: EventKind,
    site: Option<SiteTag>, hk: Option<HelperKind>, disp: StepDisposition, work: u8, hwork: u8, visits: u8,
    alloc: AllocDeltasC, slots: u8, exhausted: Option<ExhaustReason>) -> StepEventC {
    let post_h = img.helper.as_ref();
    // Adopted helperKind (0abca...): pre.helper, post.helper, helperPlan(site(pre)) for a staged pre.result, then Selected.helperKind for a helper/bookkeeping row under Applied or a P3/P6 refusal; otherwise absent.
    let helper_row = matches!(kind, EventKind::HelperEnter | EventKind::HelperVisit | EventKind::HelperExit | EventKind::HelperFail | EventKind::Bookkeeping);
    let helper_disp = matches!(disp, StepDisposition::Applied | StepDisposition::RefusedHelperWorkUncharged | StepDisposition::RefusedCapCharged);
    let helper_kind = pre.helper.as_ref().map(helper_kind_of).or_else(|| post_h.map(helper_kind_of))
        .or_else(|| if pre.result.is_some() { pre.result_plan } else { None })
        .or_else(|| if helper_row && helper_disp { hk } else { None });
    // Adopted originReferenceCursor: pre.helper, post.helper, then pre.referenceCursor for a staged pre.result or a .entry row refused at P3/P6 with no helper installed.
    let entry_refused = kind == EventKind::HelperEnter && post_h.is_none()
        && matches!(disp, StepDisposition::RefusedHelperWorkUncharged | StepDisposition::RefusedCapCharged);
    let origin_cursor = pre.helper.as_ref().map(|h| helper_common(h).origin_reference_cursor)
        .or_else(|| post_h.map(|h| helper_common(h).origin_reference_cursor))
        .or_else(|| if pre.result.is_some() || entry_refused { Some(pre.cursor) } else { None });
    let source_occurrence = pre.source_occurrence.clone();
    // Adopted resultTag (f59d...): staged by an Applied exit/failure, consumed by an Applied LegacyComplete, or retained pre.result on P2 Publish; absent otherwise.
    let result_tag_src = match (kind, disp) {
        (EventKind::HelperExit | EventKind::HelperFail, StepDisposition::Applied) => img.result.as_ref(),
        (EventKind::LegacyComplete, StepDisposition::Applied) => pre.result.as_ref(),
        (EventKind::Publish, StepDisposition::ExhaustedFuelUncharged) => pre.result.as_ref(),
        _ => None };
    StepEventC { step_id: ExpandedStepIdC { run_id: std::sync::Arc::clone(&img.run_id), ordinal },
        rule_id: rule, event_kind: kind, site, helper_kind, disposition: disp, work_delta: work,
        helper_work_delta: hwork, helper_visits_delta: visits,
        reference_cursor_delta: if disp == StepDisposition::Applied && kind == EventKind::LegacyComplete { 1 } else { 0 },
        alloc_deltas: alloc, value_slot_writes: slots, logical_depth_before: pre.depth,
        logical_depth_after: img.frames.len(), task_cursor_before: pre.tasks, task_cursor_after: img.tasks.len(),
        helper_phase_before: pre.helper.as_ref().map(phase_of), helper_phase_after: post_h.map(phase_of),
        helper_cursor_before: pre.helper.as_ref().and_then(|h| cursor_of(h, pre.tasks)),
        helper_cursor_after: post_h.and_then(|h| cursor_of(h, img.tasks.len())),
        result_tag: result_tag_src.map(result_tag), origin_reference_cursor: origin_cursor,
        source_expr_id: pre.origin, source_occurrence, exhausted_kind: exhausted,
        outcome_tag: outcome_tag(&img.outcome), outcome_payload: img.outcome,
        boundary: if img.outcome == OutcomeC::Running { BoundaryKind::None } else { BoundaryKind::Terminal } }
}

// ================= apply_validated (infallible, after P4 in owner_step) =================
pub fn owner_apply_validated(img: &mut MachineImage, v: ValidatedPlan, sel: &Selected) -> Result<StepResult, HostViolation> {
    if v.rule() != sel.rule { return Err(invalid_plan("ValidatedPlan rule != Selected rule")); }
    let adm = img.admission.clone();
    let pre = snap(&PreImage::bind(&adm, img)?)?; // before any write; on the owner_step path site(pre) already succeeded in PS select
    let ordinal = pre.consumed.checked_sub(1).ok_or_else(|| arith("event ordinal consumedWork - 1"))?;
    let next_alloc_data_nodes = if v.delta.alloc_data_nodes > 0 { img.alloc.data_nodes.checked_add(v.delta.alloc_data_nodes as usize).ok_or_else(|| arith("P7 alloc.dataNodes + 1"))? } else { img.alloc.data_nodes };
    let next_alloc_data_fields = if v.delta.alloc_data_fields > 0 { img.alloc.data_fields.checked_add(v.delta.alloc_data_fields as usize).ok_or_else(|| arith("P7 alloc.dataFields + fieldLen"))? } else { img.alloc.data_fields };
    let next_alloc_environment_cells = if v.writes.alloc_env.is_some() { img.alloc.environment_cells.checked_add(1).ok_or_else(|| arith("P7 alloc.environmentCells + 1"))? } else { img.alloc.environment_cells };
    let next_helper_visits = img.helper_visits.checked_add(u128::from(sel.helper_visits)).ok_or_else(|| arith("P7 helperVisits + visits"))?;
    let next_reference_cursor = if sel.event_kind == EventKind::LegacyComplete { img.reference_cursor.checked_add(1).ok_or_else(|| arith("P7 referenceCursor + 1"))? } else { img.reference_cursor };
    let w = v.writes;
    let data_start = img.active.data; let fields_start = img.active.data_fields; let env_start = img.active.env;
    // Register field writes first (control, origin, env root, result, helper).
    if let Write::Set(c) = w.control { img.control = c; }
    if let Write::Set(c) = w.control_origin { img.control_origin = c; }
    if let Write::Set(e) = w.env_root { img.env_root = e; }
    if let Write::Set(r) = w.result { img.result = r; }
    if let Write::Set(h) = w.helper { img.helper = h; }
    // Data field block before its node (G1), then env cell, then value slot.
    if let Some(a) = w.alloc_data {
        let fields: &[RefC] = match &a { AllocDataWrite::Cons { fields } | AllocDataWrite::Branch { fields } => fields,
            AllocDataWrite::Leaf { field } => std::slice::from_ref(field), _ => &[] };
        for f in fields { img.data_fields.push(*f); }
        let slice = crate::sherlock::helper_machine::shared::SliceC::normalized(fields_start, a.field_len());
        let row = match a { AllocDataWrite::U128 { n } => RawDataRow::U128 { n }, AllocDataWrite::Bool { b } => RawDataRow::Bool { b },
            AllocDataWrite::Nil => RawDataRow::Nil,
            AllocDataWrite::Cons { .. } => RawDataRow::Cons { fields: super::super::raw::FieldSlice {
                start: super::super::raw::FieldPos(slice.start), len: slice.len } },
            AllocDataWrite::Leaf { .. } => RawDataRow::Leaf { fields: super::super::raw::FieldSlice {
                start: super::super::raw::FieldPos(slice.start), len: slice.len } },
            AllocDataWrite::Branch { .. } => RawDataRow::Branch { fields: super::super::raw::FieldSlice {
                start: super::super::raw::FieldPos(slice.start), len: slice.len } } };
        img.data.push(row); let _ = data_start; let _ = env_start;
    }
    if let Some(df) = w.alloc_data_field { img.data_fields.push(df); }
    if let Some(cw) = w.alloc_constructor {
        img.data.push(RawDataRow::Constructor {
            type_id: RawTypeId(cw.type_id.0),
            ctor_id: RawCtorId(cw.ctor_id.0),
            fields: super::super::raw::FieldSlice {
                start: super::super::raw::FieldPos(cw.fields.start),
                len: cw.fields.len,
            },
        });
    }
    if let Some(cell) = w.alloc_env { img.env.push(cell); }
    if let Some(slot) = w.alloc_value_slot { img.value_slots.push(slot); }
    // Task rewrite of the pre-image top, before any push/pop.
    if let Some(t) = w.task_rewrite_top { if let Some(top) = img.tasks.last_mut() { *top = t; } }
    match w.task_op { TaskOp::Push(t) => img.tasks.push(t), TaskOp::Pop => { img.tasks.pop(); }, TaskOp::None => {} }
    // Frame push / rewrite / pop (Vec pop removes the slot; no stale row remains).
    match w.frame { Some(FrameWrite::Push(f)) => img.frames.push(f),
        Some(FrameWrite::RewriteTop(f)) => { if let Some(top) = img.frames.last_mut() { *top = f; } }
        Some(FrameWrite::Pop) => { img.frames.pop(); }, None => {} }
    // Counters and active lengths move exactly once, derived from writes + Selected.
    img.alloc.data_nodes = next_alloc_data_nodes; img.alloc.data_fields = next_alloc_data_fields;
    img.alloc.environment_cells = next_alloc_environment_cells;
    img.active.data = img.data.len(); img.active.data_fields = img.data_fields.len();
    img.active.env = img.env.len(); img.active.value_slots = img.value_slots.len();
    img.helper_visits = next_helper_visits;
    img.reference_cursor = next_reference_cursor;
    let admitted = img.admission.clone();
    let alloc = AllocDeltasC { data_nodes: v.delta.alloc_data_nodes, data_fields: v.delta.alloc_data_fields,
        environment_cells: v.delta.alloc_environment_cells };
    let slots = if w.alloc_value_slot.is_some() { 1 } else { 0 };
    let hk = if sel.event_kind == EventKind::LegacyComplete { None } else { sel.helper_kind };
    Ok(StepResult::Event { event: build_event(&admitted, &pre, img, ordinal, sel.rule,
        sel.event_kind, sel.site, hk, StepDisposition::Applied, 1, if sel.helper_eligible { 1 } else { 0 },
        sel.helper_visits, alloc, slots, None) })
}

// ================= step / run =================
fn call_family(families: &dyn FamilyPlans, sel: &Selected, pre: &PreImage, g: &dyn GuardEvaluator)
    -> Result<(EffectPlan, Vec<RefC>, Vec<usize>), HostViolation> {
    check_enabled(&sel.rule)?;
    let recording = RecordingPreImageView::new(pre);
    let plan = match family_of_rule(&sel.rule) { FamilyId::Legacy => families.plan_legacy(sel, &recording, g),
        FamilyId::Scalar => families.plan_scalar(sel, &recording, g), FamilyId::EnvGet => families.plan_env_get(sel, &recording, g),
        FamilyId::EnvPut => families.plan_env_put(sel, &recording, g), FamilyId::Find => families.plan_find(sel, &recording, g),
        FamilyId::Bind => families.plan_bind(sel, &recording, g), FamilyId::Append => families.plan_append(sel, &recording, g),
        FamilyId::Validate => families.plan_validate(sel, &recording, g), FamilyId::Equal => families.plan_equal(sel, &recording, g),
        FamilyId::Match => families.plan_match(sel, &recording, g), FamilyId::Build => families.plan_build(sel, &recording, g),
        FamilyId::Plan => families.plan_plan(sel, &recording, g), FamilyId::BookkeepingRelease => families.plan_bookkeeping_release(sel, &recording, g) }?;
    Ok((plan, recording.recorded(), recording.recorded_envs()))
}
fn publish_event(a: &Admitted, pre: &Snap, img: &MachineImage, rule: RuleIdC, disp: StepDisposition,
    exhausted: Option<ExhaustReason>, hk: Option<HelperKind>) -> StepResult {
    StepResult::Event { event: build_event(a, pre, img, pre.consumed, rule, EventKind::Publish, None, hk, disp, 0, 0, 0,
        AllocDeltasC { data_nodes: 0, data_fields: 0, environment_cells: 0 }, 0, exhausted) }
}
/// Full AB/P0/P1/P2/PS/P3/P4/P6/P7 driver. `families` is the passed FamilyPlans dependency.
pub fn owner_step(a: &Admitted, img: &mut MachineImage, families: &dyn FamilyPlans, guards: &OwnerGuards)
    -> Result<StepResult, HostViolation> {
    PreImage::bind(a, img)?; // AB first, before any table read.
    if img.outcome != OutcomeC::Running { return Ok(StepResult::Terminal); } // P0
    let pre_snap = snap(&PreImage::bind(a, img)?)?; // pure pre-image event snapshot; no write precedes it
    // P1 publish (helper and result absent only).
    if img.helper.is_none() && img.result.is_none() {
        match img.control { ControlC::Error { code } => { img.outcome = OutcomeC::Rejected { code };
                return Ok(publish_event(a, &pre_snap, img, RuleIdC::KAdminPublishError, StepDisposition::Published, None, None)); }
            ControlC::Return { value } if img.frames.is_empty() => { img.outcome = OutcomeC::Halted { value };
                return Ok(publish_event(a, &pre_snap, img, RuleIdC::KAdminPublishValue, StepDisposition::Published, None, None)); }
            _ => {} }
    }
    if img.remaining_work == 0 { img.outcome = OutcomeC::Exhausted { reason: ExhaustReason::Fuel }; // P2
        return Ok(publish_event(a, &pre_snap, img, RuleIdC::KAdminPublishExhausted,
            StepDisposition::ExhaustedFuelUncharged, Some(ExhaustReason::Fuel), pre_snap.helper.as_ref().map(helper_kind_of))); }
    // PS: select + plan + validate + physical, all pure over the pre-image.
    let (sel, validated) = {
        let pre = PreImage::bind(a, img)?;
        let sel = owner_select(&pre, guards)?;
        let (plan, read_provenance, read_env_provenance) = call_family(families, &sel, &pre, guards)?;
        let validated = owner_validate_plan(&pre, &sel, plan, &read_provenance, &read_env_provenance)?;
        owner_physical_check(&pre, &validated)?;
        (sel, validated) };
    let cap = owner_cap_refusal(&validated);
    // P3 helper-work refusal, uncharged.
    if sel.helper_eligible && img.helper_work == a.body.bounds.max_helper_work {
        img.outcome = OutcomeC::Exhausted { reason: ExhaustReason::HelperWork };
        return Ok(StepResult::Event { event: build_event(a, &pre_snap, img, pre_snap.consumed, sel.rule, sel.event_kind,
            sel.site, sel.helper_kind, StepDisposition::RefusedHelperWorkUncharged, 0, 0, 0,
            AllocDeltasC { data_nodes: 0, data_fields: 0, environment_cells: 0 }, 0, Some(ExhaustReason::HelperWork)) });
    }
    // P4 charge exactly once.
    let remaining_work = img.remaining_work.checked_sub(1).ok_or_else(|| arith("P4 remainingWork - 1"))?;
    let consumed_work = img.consumed_work.checked_add(1).ok_or_else(|| arith("P4 consumedWork + 1"))?;
    let helper_work = if sel.helper_eligible { Some(img.helper_work.checked_add(1).ok_or_else(|| arith("P4 helperWork + 1"))?) } else { None };
    img.remaining_work = remaining_work; img.consumed_work = consumed_work;
    if let Some(helper_work) = helper_work { img.helper_work = helper_work; }
    // P6 logical cap refusal, charged; only P4 persists.
    if let Some(kind) = cap {
        let reason = match kind { CapKind::DataNodes => ExhaustReason::DataNodes, CapKind::DataFields => ExhaustReason::DataFields,
            CapKind::EnvironmentCells => ExhaustReason::EnvironmentCells, CapKind::HelperTasks => ExhaustReason::HelperTasks };
        img.outcome = OutcomeC::Exhausted { reason };
        return Ok(StepResult::Event { event: build_event(a, &pre_snap, img, pre_snap.consumed, sel.rule, sel.event_kind,
            sel.site, sel.helper_kind, StepDisposition::RefusedCapCharged, 1, if sel.helper_eligible { 1 } else { 0 }, 0,
            AllocDeltasC { data_nodes: 0, data_fields: 0, environment_cells: 0 }, 0, Some(reason)) });
    }
    owner_apply_validated(img, validated, &sel) // P7
}
pub fn owner_run(a: &Admitted, img: &mut MachineImage, families: &dyn FamilyPlans, guards: &OwnerGuards,
    sink: &mut dyn FnMut(&StepEventC)) -> Result<RunSummary, HostViolation> {
    PreImage::bind(a, img)?; // AB: no step or sink call on mismatch.
    let mut events: u128 = 0;
    while img.outcome == OutcomeC::Running {
        match owner_step(a, img, families, guards)? {
            StepResult::Terminal => break,
            StepResult::Event { event } => { let next_events = events.checked_add(1).ok_or_else(|| arith("run events + 1"))?; sink(&event); events = next_events; } }
    }
    Ok(RunSummary { outcome: img.outcome, events, consumed_work: img.consumed_work, helper_work: img.helper_work,
        helper_visits: img.helper_visits, reference_cursor: img.reference_cursor })
}

// ================= check_step_inv SI01-SI24 (diagnostic, explicit worklists) =================
fn structurally_equal(pre: &PreImage, a0: RefC, b0: RefC) -> Result<bool, HostViolation> {
    // Explicit pair worklist; visited pairs keep shared DAGs polynomial.
    let mut stack: Vec<(RefC, RefC)> = vec![(a0, b0)];
    let mut visited: Vec<(RefC, RefC)> = Vec::new();
    while let Some((x, y)) = stack.pop() {
        if visited.contains(&(x, y)) { continue; }
        visited.push((x, y));
        if pure_leaves::shallow_differ(pre.deref_borrowed(&x)?, pre.deref_borrowed(&y)?) { return Ok(false); }
        let arity = pre.checked_arity(&pre.kind_of(&x)?)?;
        let mut i = 0usize;
        while i < arity { stack.push((pre.field_at(&x, &i)?, pre.field_at(&y, &i)?)); i += 1; }
    }
    Ok(true)
}
fn const_descendants(pre: &PreImage, root: usize) -> Result<Vec<usize>, HostViolation> {
    let mut seen: Vec<usize> = Vec::new(); let mut stack: Vec<usize> = vec![root];
    while let Some(id) = stack.pop() {
        if seen.contains(&id) { continue; }
        seen.push(id);
        let row = pre.admitted.body.view.constants.get(id).ok_or_else(|| oob("prog.constants"))?;
        let (start, len) = const_fields(row);
        let mut i = 0usize;
        while i < len { let pos = start.checked_add(i).ok_or_else(|| arith("const descendant pos"))?;
            stack.push(pre.constant_field(&pos)?); i += 1; }
    }
    Ok(seen)
}
fn ref_in_bounds(pre: &PreImage, r: &RefC) -> bool {
    match r { RefC::Const(id) => *id < pre.admitted.body.view.constants.len(), RefC::Data(id) => *id < pre.img.active.data }
}
// ---- check_step_inv private diagnostic helpers: read-only, explicit loops/worklists, no native recursion. A read Err is a violation, never a pass.
fn frame_origin(f: &FrameRowC) -> usize {
    match f { FrameRowC::LetF { origin, .. } | FrameRowC::LeftF { origin, .. } | FrameRowC::RightF { origin, .. }
        | FrameRowC::UnaryF { origin, .. } | FrameRowC::IfF { origin, .. } | FrameRowC::MatchF { origin, .. }
        | FrameRowC::ArgsF { origin, .. } | FrameRowC::ConstructorFieldsF { origin, .. }
        | FrameRowC::ReturnF { origin, .. } | FrameRowC::ScopeF { origin, .. } => *origin }
}
fn frame_env(f: &FrameRowC) -> EnvRootC {
    match f { FrameRowC::LetF { saved_env, .. } | FrameRowC::LeftF { saved_env, .. } | FrameRowC::IfF { saved_env, .. }
        | FrameRowC::MatchF { saved_env, .. } | FrameRowC::ArgsF { saved_env, .. }
        | FrameRowC::ConstructorFieldsF { saved_env, .. } | FrameRowC::ReturnF { saved_env, .. }
        | FrameRowC::ScopeF { saved_env, .. } => *saved_env,
        FrameRowC::RightF { .. } | FrameRowC::UnaryF { .. } => None }
}
// SI-13/SI-17 ValueRef holders. `exempt` receives only the SI-17 positional exemptions (helper Validate.root,
// ValidateTask.ref); every other holder goes to `refs`.
fn frame_refs(f: &FrameRowC, refs: &mut Vec<RefC>) {
    if let FrameRowC::RightF { left, .. } = f { refs.push(*left); }
}
fn control_refs(c: &ControlC, refs: &mut Vec<RefC>) {
    match c { ControlC::Return { value } => refs.push(*value), ControlC::Cases { value, .. } => refs.push(*value),
        ControlC::CaseResult { value, .. } => refs.push(*value),
        ControlC::PrimResult { result: PrimOutcome::Value { value } } => refs.push(*value),
        ControlC::ScheduleNext { frame, .. } => frame_refs(frame, refs), _ => {} }
}
fn helper_refs(h: &HelperControlC, refs: &mut Vec<RefC>, exempt: &mut Vec<RefC>) {
    match h {
        HelperControlC::Scalar { operand_a, operand_b, .. } => { refs.push(*operand_a); if let Some(b) = operand_b { refs.push(*b); } }
        HelperControlC::EnvGet { found, .. } => { if let Some(f) = found { refs.push(*f); } }
        HelperControlC::EnvPut { value, .. } => refs.push(*value),
        HelperControlC::Find { .. } | HelperControlC::Bind { .. } => {}
        HelperControlC::Append { value, staged, .. } => { refs.push(*value); if let Some(s) = staged { refs.push(*s); } }
        HelperControlC::Build { staged, node, .. } => { if let Some(s) = staged { refs.push(*s); } if let Some(n) = node { refs.push(*n); } }
        HelperControlC::Validate { root, .. } => exempt.push(*root),
        HelperControlC::Equal { left, right, .. } => { refs.push(*left); refs.push(*right); }
        HelperControlC::Match { scrutinee, staged, .. } => { refs.push(*scrutinee); if let Some(s) = staged { refs.push(*s); } }
        HelperControlC::Plan { scrutinee, cur, .. } => { refs.push(*scrutinee); refs.push(*cur); }
    }
}
fn result_refs(r: &HelperResultC, refs: &mut Vec<RefC>) {
    match r { HelperResultC::EnvFound { value } | HelperResultC::ScalarValue { value }
        | HelperResultC::EqualDone { value } | HelperResultC::BuildDone { value } => refs.push(*value),
        _ => {} }
}
fn task_refs(t: &HelperTaskC, refs: &mut Vec<RefC>, exempt: &mut Vec<RefC>) {
    match t { HelperTaskC::ValidateTask { r, .. } => exempt.push(*r),
        HelperTaskC::EqualTask { left, right, .. } => { refs.push(*left); refs.push(*right); }
        HelperTaskC::PairTask { arg, .. } => refs.push(*arg) }
}
// SI-13 EnvId-valued holders (env cell `previous` is SI-14).
fn env_holders(img: &MachineImage, out: &mut Vec<EnvRootC>) {
    out.push(img.env_root);
    for f in &img.frames { out.push(frame_env(f)); }
    match &img.control {
        ControlC::ScheduleNext { frame, env, .. } => { out.push(frame_env(frame)); out.push(*env); }
        ControlC::Bound { binding: BoundBinding::Ok { env }, .. } => out.push(*env),
        ControlC::Cases { saved_env, .. } => out.push(*saved_env),
        ControlC::CaseResult { matching, saved_env, .. } => {
            out.push(*saved_env); if let CaseMatching::Matched { env } = matching { out.push(*env); } }
        _ => {} }
    match &img.helper {
        Some(HelperControlC::EnvGet { cursor, .. }) => out.push(*cursor),
        Some(HelperControlC::EnvPut { base, new_root, .. }) => { out.push(*base); out.push(*new_root); }
        Some(HelperControlC::Bind { partial, dup_cursor, .. }) => { out.push(*partial); out.push(*dup_cursor); }
        Some(HelperControlC::Match { saved_env, partial, .. })
        | Some(HelperControlC::Plan { saved_env, partial, .. }) => { out.push(*saved_env); out.push(*partial); }
        _ => {} }
    match &img.result {
        Some(HelperResultC::EnvUpdated { cell }) => out.push(Some(*cell)),
        Some(HelperResultC::BindOk { env }) | Some(HelperResultC::MatchMatched { env })
        | Some(HelperResultC::PlanMatched { env, .. }) => out.push(*env),
        _ => {} }
}
// MCL F2 suffix rule over normalized slices: the empty slice (0,0) is a suffix of every slice.
fn is_suffix(inner: SliceC, outer: SliceC) -> bool {
    if inner.len == 0 { return inner.start == 0; }
    match (inner.start.checked_add(inner.len), outer.start.checked_add(outer.len)) {
        (Some(ie), Some(oe)) => inner.start >= outer.start && ie == oe, _ => false }
}
// SI-11 for data row j: arity(kind) = field-slice length, slice below active.dataFields, records.DataRow.field_kinds over kind().
fn si11_row_ok(pre: &PreImage, j: usize, row: &RawDataRow) -> Result<bool, HostViolation> {
    let k = pure_leaves::borrowed_kind(pure_leaves::BorrowedValueRow::Data(row));
    let (start, len) = data_fields(row);
    if !slice_ok(SliceC { start, len }, pre.img.active.data_fields) { return Ok(false); }
    let r = RefC::Data(j);
    let kind_at = |i: usize| -> Result<LogicalKind, HostViolation> { pre.kind_of(&pre.field_at(&r, &i)?) };
    match k {
        LogicalKind::UInt | LogicalKind::Bool | LogicalKind::Nil => Ok(len == 0),
        LogicalKind::Leaf => Ok(len == 1 && kind_at(0)? == LogicalKind::UInt),
        LogicalKind::Cons => Ok(len == 2 && kind_at(0)? == LogicalKind::UInt && matches!(kind_at(1)?, LogicalKind::Nil | LogicalKind::Cons)),
        LogicalKind::Branch => Ok(len == 2 && matches!(kind_at(0)?, LogicalKind::Leaf | LogicalKind::Branch)
            && matches!(kind_at(1)?, LogicalKind::Leaf | LogicalKind::Branch)),
        LogicalKind::Constructor { type_id, ctor_id } => {
            let view = &pre.admitted.body.view;
            if type_id.0 < 4 { return Ok(false); }
            let Some(trow) = view.types.get(type_id.0) else { return Ok(false); };
            if ctor_id.0 < trow.ctor_start.0 || ctor_id.0 - trow.ctor_start.0 >= trow.ctor_len { return Ok(false); }
            let Some(crow) = view.constructors.get(ctor_id.0) else { return Ok(false); };
            if crow.owning_type != type_id || crow.field_len != len { return Ok(false); }
            let Some(f_end) = crow.field_start.checked_add(len) else { return Ok(false); };
            if f_end > view.ctor_fields.len() { return Ok(false); }
            let mut i = 0usize;
            while i < len {
                let expected_ty = view.ctor_fields[crow.field_start + i].type_id;
                if expected_ty.0 >= view.types.len() { return Ok(false); }
                if pure_leaves::sort_of_kind(kind_at(i)?) != Some(expected_ty) { return Ok(false); }
                i += 1;
            }
            Ok(true)
        }
        LogicalKind::Marker | LogicalKind::ConstructorExtensionReserved => Ok(false),
    }
}
// SI-15 for one frame (also SI-16 ScheduleNext.frame): origin and frame ExprIds in range, FrameRow origin_tag,
// MatchF.arms = expr(origin).arms, ArgsF.rest suffix of expr(origin).args with completed.len + rest.len + 1 = args.len,
// and every frame slice inside its table.
fn si15_frame_ok(pre: &PreImage, f: &FrameRowC) -> Result<bool, HostViolation> {
    let view = &pre.admitted.body.view;
    let n = view.expressions.len();
    let origin = frame_origin(f);
    if origin >= n { return Ok(false); }
    let e = pre.expr(&origin)?;
    Ok(match f {
        FrameRowC::LetF { body, .. } => *body < n && matches!(e, ExpressionRowC::Let { .. }),
        FrameRowC::LeftF { right, .. } => *right < n && matches!(e, ExpressionRowC::Binary { .. }),
        FrameRowC::RightF { .. } => matches!(e, ExpressionRowC::Binary { .. }),
        FrameRowC::UnaryF { .. } => matches!(e, ExpressionRowC::Unary { .. }),
        FrameRowC::IfF { then_expr, else_expr, .. } => *then_expr < n && *else_expr < n && matches!(e, ExpressionRowC::If { .. }),
        FrameRowC::MatchF { arms, .. } => slice_ok(*arms, view.arms.len())
            && matches!(e, ExpressionRowC::Match { arms: ea, .. } if ea == arms),
        FrameRowC::ArgsF { completed, rest, .. } => slice_ok(*completed, pre.img.active.value_slots)
            && slice_ok(*rest, view.children.len()) && match e {
                ExpressionRowC::Call { args, .. } => is_suffix(*rest, *args)
                    && completed.len.checked_add(rest.len).and_then(|s| s.checked_add(1)) == Some(args.len),
                _ => false },
        FrameRowC::ConstructorFieldsF { type_id, ctor_id, index, field_type, completed, rest, .. } => {
            if !slice_ok(*completed, pre.img.active.value_slots) || !slice_ok(*rest, view.children.len()) || type_id.0 < 4 {
                return Ok(false);
            }
            let ExpressionRowC::Construct { type_id: et, ctor_id: ec, fields, constant: None } = e else {
                return Ok(false);
            };
            if et != type_id || ec != ctor_id || *index != completed.len || !is_suffix(*rest, *fields)
                || completed.len.checked_add(rest.len).and_then(|s| s.checked_add(1)) != Some(fields.len)
            {
                return Ok(false);
            }
            let Some(trow) = view.types.get(type_id.0) else { return Ok(false); };
            if ctor_id.0 < trow.ctor_start.0 || ctor_id.0 - trow.ctor_start.0 >= trow.ctor_len { return Ok(false); }
            let Some(crow) = view.constructors.get(ctor_id.0) else { return Ok(false); };
            let Some(f_end) = crow.field_start.checked_add(crow.field_len) else { return Ok(false); };
            if crow.owning_type != *type_id || crow.field_len != fields.len || *index >= crow.field_len || f_end > view.ctor_fields.len() {
                return Ok(false);
            }
            if view.ctor_fields[crow.field_start + *index].type_id != *field_type || field_type.0 >= view.types.len() {
                return Ok(false);
            }
            let mut k = 0usize;
            while k < completed.len {
                let vref = pre.value_slot(&(completed.start + k))?;
                let exp_ty = view.ctor_fields[crow.field_start + k].type_id;
                if pure_leaves::sort_of_kind(pre.kind_of(&vref)?) != Some(exp_ty) { return Ok(false); }
                k += 1;
            }
            true
        }
        FrameRowC::ReturnF { return_type, .. } => return_type.0 < view.types.len().max(4) && matches!(e, ExpressionRowC::Call { .. }),
        FrameRowC::ScopeF { .. } => matches!(e, ExpressionRowC::Let { .. } | ExpressionRowC::Match { .. }),
    })
}
// SI-20 Return at depth > 0: seed expectedChild(top) (all candidates for ReturnF and ScopeF-over-Match), then
// tailClosure through nested If branches with an explicit heap worklist and a visited set over ExprIds.
fn si20_return_ok(pre: &PreImage, top: &FrameRowC, origin: usize) -> Result<bool, HostViolation> {
    let mut work: Vec<usize> = Vec::new();
    let o = frame_origin(top);
    match (top, pre.expr(&o)?) {
        (FrameRowC::LetF { .. }, ExpressionRowC::Let { value, .. }) => work.push(*value),
        (FrameRowC::LeftF { .. }, ExpressionRowC::Binary { left, .. }) => work.push(*left),
        (FrameRowC::RightF { .. }, ExpressionRowC::Binary { right, .. }) => work.push(*right),
        (FrameRowC::UnaryF { .. }, ExpressionRowC::Unary { operand, .. }) => work.push(*operand),
        (FrameRowC::IfF { .. }, ExpressionRowC::If { cond, .. }) => work.push(*cond),
        (FrameRowC::MatchF { .. }, ExpressionRowC::Match { scrutinee, .. }) => work.push(*scrutinee),
        (FrameRowC::ArgsF { completed, .. }, ExpressionRowC::Call { args, .. }) => {
            if completed.len >= args.len { return Ok(false); }
            let pos = args.start.checked_add(completed.len).ok_or_else(|| arith("SI-20 ArgsF child pos"))?;
            work.push(pre.child(&pos)?);
        }
        (FrameRowC::ConstructorFieldsF { index, completed, .. }, ExpressionRowC::Construct { fields, .. }) => {
            if *index != completed.len || completed.len >= fields.len { return Ok(false); }
            let pos = fields.start.checked_add(completed.len).ok_or_else(|| arith("SI-20 ConstructorFieldsF child pos"))?;
            work.push(pre.child(&pos)?);
        }
        (FrameRowC::ReturnF { .. }, ExpressionRowC::Call { function_name, .. }) => {
            let n = pre.admitted_function_count(); let mut i = 0usize;
            while i < n { let f = *pre.function_row(&i)?; if f.function_name == *function_name { work.push(f.body); } i += 1; }
        }
        (FrameRowC::ScopeF { .. }, ExpressionRowC::Let { body, .. }) => work.push(*body),
        (FrameRowC::ScopeF { .. }, ExpressionRowC::Match { arms, .. }) => {
            let mut k = 0usize;
            while k < arms.len { let aid = arms.start.checked_add(k).ok_or_else(|| arith("SI-20 ScopeF arm"))?;
                work.push(pre.arm(&aid)?.body); k += 1; }
        }
        _ => return Ok(false),
    }
    let mut seen: Vec<bool> = vec![false; pre.admitted.body.view.expressions.len()];
    while let Some(e) = work.pop() {
        let slot = seen.get_mut(e).ok_or_else(|| oob("SI-20 tailClosure ExprId"))?;
        if *slot { continue; }
        *slot = true;
        if e == origin { return Ok(true); }
        if let ExpressionRowC::If { then_expr, else_expr, .. } = pre.expr(&e)? { work.push(*then_expr); work.push(*else_expr); }
    }
    Ok(false)
}
// SI-23 live ValSlices (control, frames incl. a ScheduleNext frame, helper, result) inside [0, active.valueSlots), len <= maxArity.
fn si23_live_slices_ok(img: &MachineImage, max_arity: usize) -> bool {
    let live = |s: &SliceC| slice_ok(*s, img.active.value_slots) && s.len <= max_arity;
    let control_ok = match &img.control {
        ControlC::Invoke { args, .. } | ControlC::Enter { args, .. } | ControlC::Materialize { fields: args, .. } => live(args),
        ControlC::ScheduleNext { frame: FrameRowC::ArgsF { completed, .. } | FrameRowC::ConstructorFieldsF { completed, .. }, .. } => live(completed),
        _ => true };
    let frames_ok = img.frames.iter().all(|f| match f {
        FrameRowC::ArgsF { completed, .. } | FrameRowC::ConstructorFieldsF { completed, .. } => live(completed),
        _ => true
    });
    let helper_ok = match &img.helper {
        Some(HelperControlC::Bind { args, .. }) => live(args),
        Some(HelperControlC::Append { src, .. }) | Some(HelperControlC::Build { src, .. }) => live(src),
        _ => true };
    let result_ok = match &img.result { Some(HelperResultC::AppendDone { seq }) => live(seq), _ => true };
    control_ok && frames_ok && helper_ok && result_ok
}
/// `check_view` is the prepare lane's check_program_view, supplied by Root.
pub fn owner_check_step_inv(a: &Admitted, img: &MachineImage,
    check_view: &dyn Fn(&ProgramView, &BoundsC, &LayoutSchemaC) -> Result<(), ProgramViewViolation>)
    -> Result<(), Vec<StepInvViolation>> {
    if !std::sync::Arc::ptr_eq(&a.body, &img.admission.body) {
        return Err(vec![siv(StepInvClauseId::SI01, "admission binding", "admitted instance mismatch")]); }
    let pre = match PreImage::bind(a, img) { Ok(p) => p, Err(_) =>
        return Err(vec![siv(StepInvClauseId::SI01, "admission binding", "bind failed")]) };
    let b = &a.body.bounds; let view = &a.body.view; let mut v: Vec<StepInvViolation> = Vec::new();
    let mut push = |c: StepInvClauseId, l: &str, d: &str, bad: bool, vv: &mut Vec<StepInvViolation>| {
        if bad { vv.push(siv(c, l, d)); } };
    let has_ext = !view.types.is_empty() || !view.constructors.is_empty() || !view.plan_nodes.is_empty()
        || !view.pattern_children.is_empty() || view.expressions.iter().any(|e| matches!(e, ExpressionRowC::Construct { .. } | ExpressionRowC::Match { plan: Some(_), .. }));
    // SI-01 identity profile/domain/finance.
    let si01_ok = if has_ext {
        a.body.identity.profile_id == 2
    } else {
        a.body.identity.profile_id == 2 && a.body.identity.grammar_domain == GrammarDomain::DedicatedSorts
            && a.body.identity.finance == FinanceInput::Absent
    };
    push(StepInvClauseId::SI01, "identity", "profile/domain/finance", !si01_ok, &mut v);
    // SI-02 outcome payload.
    let si02 = match img.outcome { OutcomeC::Halted { value } => !ref_in_bounds(&pre, &value),
        OutcomeC::Rejected { code } => !(1..=11).contains(&code),
        OutcomeC::Exhausted { reason } => reason == ExhaustReason::SerializationBytes, OutcomeC::Running => false };
    push(StepInvClauseId::SI02, "reg.outcome", "payload", si02, &mut v);
    // SI-03 / SI-04 counters.
    push(StepInvClauseId::SI03, "work counters", "counter relation",
        !(img.remaining_work.checked_add(img.consumed_work) == Some(img.initial_fuel)
            && img.initial_fuel <= b.max_reference_work && img.helper_work <= b.max_helper_work
            && img.helper_work <= img.consumed_work && img.helper_visits <= img.helper_work
            && img.initial_fuel <= a.body.layout.capacities.value_slots as u128), &mut v);
    push(StepInvClauseId::SI04, "reg.referenceCursor", "cursor = consumed - helper",
        img.consumed_work.checked_sub(img.helper_work) != Some(img.reference_cursor), &mut v);
    // SI-05 active == alloc.
    push(StepInvClauseId::SI05, "alloc/active", "active != alloc",
        !(img.active.data == img.alloc.data_nodes && img.active.data_fields == img.alloc.data_fields
            && img.active.env == img.alloc.environment_cells && img.alloc.data_nodes <= b.data_nodes
            && img.alloc.data_fields <= b.data_field_slots && img.alloc.environment_cells <= b.environment_cells
            && img.active.data == img.data.len() && img.active.data_fields == img.data_fields.len()
            && img.active.env == img.env.len() && img.active.value_slots == img.value_slots.len()), &mut v);
    // SI-06 depth.
    push(StepInvClauseId::SI06, "reg.depth", "depth/stackLimit", !(img.frames.len() <= img.stack_limit && img.stack_limit <= b.frames), &mut v);
    // SI-07 tasks.
    let si07 = img.tasks.len() > b.helper_tasks || (!img.tasks.is_empty() && img.helper.is_none())
        || img.tasks.iter().any(|t| match (t, &img.helper) {
            (HelperTaskC::ValidateTask { .. }, Some(HelperControlC::Validate { .. }))
            | (HelperTaskC::EqualTask { .. }, Some(HelperControlC::Equal { .. }))
            | (HelperTaskC::PairTask { .. }, Some(HelperControlC::Bind { .. })) => false, _ => img.helper.is_some() && !img.tasks.is_empty() });
    push(StepInvClauseId::SI07, "img.tasks", "task kind", si07, &mut v);
    // SI-08 helper coherence (site/kind/origin/parentResume).
    if let Some(h) = &img.helper {
        let site_ok = owner_site(&pre).map_or(false, |s| helper_common(h).site == s
            && owner_helper_plan(s, &pre) == Some(helper_kind_of(h)));
        push(StepInvClauseId::SI08, "reg.helper", "helper coherence",
            !(img.result.is_none() && site_ok && helper_common(h).origin_reference_cursor == img.reference_cursor
                && helper_common(h).parent_resume.is_none()
                && !matches!(img.control, ControlC::Error { .. })
                && !(matches!(img.control, ControlC::Return { .. }) && img.frames.is_empty())), &mut v);
    }
    // SI-09 result membership by site.
    if let Some(r) = &img.result {
        let allowed = owner_site(&pre).map_or(false, |s| {
            let top_op = match pre.top_frame() { Some(FrameRowC::RightF { op, .. }) => Some(*op), _ => None };
            let ctor_checked = match pre.top_frame() { Some(FrameRowC::ConstructorFieldsF { checked, .. }) => Some(*checked), _ => None };
            let has_plan = matches!(pre.expr(&img.control_origin), Ok(ExpressionRowC::Match { plan: Some(_), .. }));
            match (s, r) { (SiteTag::EvalLiteral, HelperResultC::Validated { .. }) => true,
                (SiteTag::EvalVar, HelperResultC::EnvFound { .. } | HelperResultC::EnvMissing) => true,
                (SiteTag::RetLet, HelperResultC::EnvUpdated { .. }) => true,
                (SiteTag::RetRight, HelperResultC::EqualDone { .. }) => top_op == Some(BinaryOp::Eq),
                (SiteTag::RetRight, HelperResultC::ScalarValue { .. } | HelperResultC::ScalarError { .. }) => top_op != Some(BinaryOp::Eq),
                (SiteTag::RetUnary, HelperResultC::ScalarValue { .. } | HelperResultC::ScalarError { .. }) => true,
                (SiteTag::RetIf, HelperResultC::ScalarSelector { .. }) => true,
                (SiteTag::RetArgs, HelperResultC::AppendDone { .. }) => true,
                (SiteTag::RetCtorFields, HelperResultC::ScalarTypeTest { .. }) => ctor_checked == Some(false),
                (SiteTag::RetCtorFields, HelperResultC::AppendDone { .. }) => ctor_checked == Some(true),
                (SiteTag::RetCall, HelperResultC::ScalarTypeTest { .. }) => true,
                (SiteTag::Invoke, HelperResultC::FindFound { .. } | HelperResultC::FindMissing) => true,
                (SiteTag::Enter, HelperResultC::BindOk { .. } | HelperResultC::BindBad) => true,
                (SiteTag::Materialize, HelperResultC::BuildDone { .. }) => true,
                (SiteTag::Cases, HelperResultC::MatchMatched { .. } | HelperResultC::MatchNoMatch) => !has_plan,
                (SiteTag::Cases, HelperResultC::PlanMatched { .. } | HelperResultC::PlanNoMatch) => has_plan,
                _ => false } });
        push(StepInvClauseId::SI09, "reg.result", "result membership",
            !(img.helper.is_none() && img.tasks.is_empty() && allowed), &mut v);
    }
    // SI-10: Vec lengths are the sole active lengths.
    push(StepInvClauseId::SI10, "zero storage", "active length mismatch",
        !(img.active.data == img.data.len() && img.active.value_slots == img.value_slots.len()), &mut v);
    // SI-11 data typing.
    let mut si11 = false;
    for (j, row) in img.data.iter().enumerate() {
        if !matches!(si11_row_ok(&pre, j, row), Ok(true)) { si11 = true; } }
    push(StepInvClauseId::SI11, "img.data", "data typing", si11, &mut v);
    // SI-12 field-block ownership (G1), accounting for pending Build field slice prior to final Constructor node.
    let mut si12 = false; let mut field_cursor = 0usize;
    for row in &img.data {
        let (start, len) = data_fields(row);
        if len > 0 {
            if start != field_cursor { si12 = true; }
            match field_cursor.checked_add(len) { Some(c) => field_cursor = c, None => { si12 = true; break; } } } }
    let pending_build_fields = match &img.helper {
        Some(HelperControlC::Build { src, src_cursor, field_start, node: None, phase: BuildPhase::Copy, .. }) if *src_cursor > 0 => {
            if field_cursor != *field_start || *src_cursor >= src.len { si12 = true; }
            *src_cursor
        }
        _ => 0,
    };
    if field_cursor.checked_add(pending_build_fields) != Some(img.data_fields.len()) { si12 = true; }
    push(StepInvClauseId::SI12, "img.dataFields", "field ownership", si12, &mut v);
    // SI-13 references.
    let mut si13 = false;
    for (owner, row) in img.data.iter().enumerate() {
        let (start, len) = data_fields(row); let mut i = 0usize;
        while i < len {
            match start.checked_add(i).and_then(|p| img.data_fields.get(p)) {
                Some(RefC::Data(id)) if *id >= owner => si13 = true,
                Some(_) => {}
                None => si13 = true }
            i += 1; } }
    if img.data_fields.iter().any(|r| !ref_in_bounds(&pre, r)) { si13 = true; }
    let mut held: Vec<RefC> = Vec::new(); let mut validate_held: Vec<RefC> = Vec::new();
    control_refs(&img.control, &mut held);
    for f in &img.frames { frame_refs(f, &mut held); }
    if let Some(h) = &img.helper { helper_refs(h, &mut held, &mut validate_held); }
    if let Some(r) = &img.result { result_refs(r, &mut held); }
    for c in &img.env { held.push(c.value); }
    for s in &img.value_slots { held.push(*s); }
    for t in &img.tasks { task_refs(t, &mut held, &mut validate_held); }
    let outcome_ref: Option<RefC> = match img.outcome { OutcomeC::Halted { value } => Some(value), _ => None };
    if held.iter().chain(validate_held.iter()).chain(outcome_ref.iter()).any(|r| !ref_in_bounds(&pre, r)) { si13 = true; }
    let mut env_held: Vec<EnvRootC> = Vec::new(); env_holders(img, &mut env_held);
    if env_held.iter().any(|e| e.map_or(false, |id| id >= img.active.env)) { si13 = true; }
    push(StepInvClauseId::SI13, "references", "reference and EnvId bound", si13, &mut v);
    // SI-14 env cells.
    push(StepInvClauseId::SI14, "img.env", "env cell",
        img.env.iter().enumerate().any(|(i, c)| !(c.binder < view.names.len()
            && c.previous.map_or(true, |p| p < i) && ref_in_bounds(&pre, &c.value))), &mut v);
    // SI-15 frames and code ids.
    let n_expr = view.expressions.len(); let n_fn = view.functions.len();
    let mut si15 = img.control_origin >= n_expr
        || img.frames.iter().any(|f| !matches!(si15_frame_ok(&pre, f), Ok(true)));
    si15 |= match &img.control {
        ControlC::Eval { expr } => *expr >= n_expr,
        ControlC::ScheduleNext { next, .. } => *next >= n_expr,
        ControlC::Enter { function: Some(fid), .. } => *fid >= n_fn,
        ControlC::Bound { return_type, body, .. } => *body >= n_expr || return_type.0 >= view.types.len().max(4),
        ControlC::CaseResult { body, .. } => *body >= n_expr,
        ControlC::Materialize { type_id, ctor_id, .. } => adt_checked_ctor_arity(view, *type_id, *ctor_id).is_err(),
        _ => false };
    si15 |= match &img.helper {
        Some(HelperControlC::Bind { function, params, .. }) => *function >= n_fn || !slice_ok(*params, view.parameters.len()),
        Some(HelperControlC::Find { found: Some(fid), .. }) => *fid >= n_fn,
        Some(HelperControlC::Plan { source_match, node, path, arm, .. }) =>
            *source_match >= n_expr || *node >= view.plan_nodes.len() || !slice_ok(*path, view.field_paths.len())
                || arm.map_or(false, |a| a >= view.arms.len()),
        Some(HelperControlC::Build { type_id, ctor_id, .. }) => adt_checked_ctor_arity(view, *type_id, *ctor_id).is_err(),
        _ => false };
    si15 |= match &img.result {
        Some(HelperResultC::FindFound { function }) => *function >= n_fn,
        Some(HelperResultC::PlanMatched { arm, .. }) => *arm >= view.arms.len(),
        _ => false };
    // SI-20 controlOrigin coherence.
    let si20 = match &img.control {
        ControlC::Eval { expr } => img.control_origin != *expr,
        ControlC::ScheduleNext { frame, .. } => img.control_origin != frame_origin(frame),
        ControlC::Invoke { .. } | ControlC::Enter { .. } | ControlC::Bound { .. } =>
            !matches!(pre.expr(&img.control_origin), Ok(ExpressionRowC::Call { .. })),
        ControlC::Cases { .. } | ControlC::CaseResult { .. } =>
            !matches!(pre.expr(&img.control_origin), Ok(ExpressionRowC::Match { .. })),
        ControlC::Materialize { type_id, ctor_id, .. } =>
            !matches!(pre.expr(&img.control_origin), Ok(ExpressionRowC::Construct { type_id: et, ctor_id: ec, constant: None, .. }) if et == type_id && ec == ctor_id),
        ControlC::PrimResult { .. } =>
            !matches!(pre.expr(&img.control_origin), Ok(ExpressionRowC::Binary { .. } | ExpressionRowC::Unary { .. })),
        ControlC::Return { .. } => match img.frames.last() {
            Some(top) => !matches!(si20_return_ok(&pre, top, img.control_origin), Ok(true)),
            None => false },
        ControlC::Error { .. } => false };
    push(StepInvClauseId::SI15, "frames", "frame code ids", si15, &mut v);
    push(StepInvClauseId::SI20, "reg.controlOrigin", "origin coherence", si20, &mut v);
    // SI-16 control payload.
    let n_arms = view.arms.len();
    let origin_arms_suffix = |s: &SliceC| slice_ok(*s, n_arms)
        && matches!(pre.expr(&img.control_origin), Ok(ExpressionRowC::Match { arms, .. }) if is_suffix(*s, *arms));
    let si16 = match &img.control {
        ControlC::Invoke { function_name, args } =>
            *function_name >= view.names.len() || !slice_ok(*args, img.active.value_slots) || args.len > b.max_arity,
        ControlC::Enter { args, .. } => !slice_ok(*args, img.active.value_slots) || args.len > b.max_arity,
        ControlC::Materialize { type_id, ctor_id, fields } => {
            if !slice_ok(*fields, img.active.value_slots) || fields.len == 0 || fields.len > b.max_arity {
                true
            } else if !matches!(adt_checked_ctor_arity(view, *type_id, *ctor_id), Ok(arity) if arity == fields.len) {
                true
            } else {
                let crow = &view.constructors[ctor_id.0];
                (0..fields.len).any(|k| {
                    let exp_ty = view.ctor_fields[crow.field_start + k].type_id;
                    match pre.value_slot(&(fields.start + k)).and_then(|r| pre.kind_of(&r)) {
                        Ok(knd) => pure_leaves::sort_of_kind(knd) != Some(exp_ty),
                        Err(_) => true,
                    }
                })
            }
        }
        ControlC::ScheduleNext { frame, .. } => !matches!(si15_frame_ok(&pre, frame), Ok(true)),
        ControlC::Cases { arms, .. } => !origin_arms_suffix(arms),
        ControlC::CaseResult { rest, .. } => !origin_arms_suffix(rest),
        ControlC::PrimResult { result: PrimOutcome::Error { code } } => !matches!(code, 10 | 11),
        ControlC::Error { code } => !(1..=11).contains(code),
        ControlC::Eval { .. } | ControlC::Return { .. } | ControlC::Bound { .. }
        | ControlC::PrimResult { result: PrimOutcome::Value { .. } } => false };
    push(StepInvClauseId::SI16, "reg.control", "control payload", si16, &mut v);
    // SI-17 marker placement.
    let si17 = held.iter().chain(img.data_fields.iter()).any(|r| match r {
        RefC::Const(id) => !matches!(pre.marker_free(id), Ok(true)), RefC::Data(_) => false });
    push(StepInvClauseId::SI17, "markers", "marker placement", si17, &mut v);
    // SI-18 terminal coherence.
    push(StepInvClauseId::SI18, "terminal", "terminal coherence",
        matches!(img.outcome, OutcomeC::Halted { .. } | OutcomeC::Rejected { .. })
            && !(img.helper.is_none() && img.result.is_none() && img.tasks.is_empty()), &mut v);
    // SI-21 per-kind helper coherence.
    if let Some(h) = &img.helper {
        let si21 = match h { HelperControlC::Bind { .. } => si21_bind_helper_ok(&pre), HelperControlC::Match { .. } => si21_match_helper_ok(&pre), _ => si21_helper_ok(&pre) };
        push(StepInvClauseId::SI21, "helper coherence", "SI-21", !matches!(si21, Ok(true)), &mut v);
    }
    if let Some(r) = &img.result {
        let si21r = match r { HelperResultC::BindOk { .. } | HelperResultC::BindBad => si21_bind_result_ok(&pre, r),
            HelperResultC::MatchMatched { .. } | HelperResultC::MatchNoMatch => si21_match_result_ok(&pre, r), _ => si21_result_ok(&pre, r) };
        push(StepInvClauseId::SI21, "result coherence", "SI-21R", !matches!(si21r, Ok(true)), &mut v);
    }
    // SI-22 helper_state_space.
    if img.helper.is_some() {
        push(StepInvClauseId::SI22, "helper state space", "SI-22", !matches!(si22_helper_ok(&pre), Ok(true)), &mut v);
    }
    // SI-23 value slots.
    push(StepInvClauseId::SI23, "img.valueSlots", "value slots",
        !(img.active.value_slots as u128 <= img.consumed_work
            && img.active.value_slots <= a.body.layout.capacities.value_slots
            && img.value_slots.iter().all(|r| ref_in_bounds(&pre, r))
            && si23_live_slices_ok(img, b.max_arity)), &mut v);
    // SI-24 dispatched by the admitted grammar domain: GenericNominal takes the
    // V1..V17 checker under the admitted ExtDomain; DedicatedSorts must carry no
    // extension rows and pass the prepare lane's R4 checker.
    let si24 = match a.body.identity.grammar_domain {
        GrammarDomain::GenericNominalExtensionReserved => super::owner_ext_admit::check_program_view_ext(
            view, None,
            super::owner_ext_source_program::ExtDomain {
                grammar_domain: a.body.identity.grammar_domain, finance: a.body.identity.finance },
            b, &a.body.layout).is_err(),
        GrammarDomain::DedicatedSorts => has_ext || check_view(view, b, &a.body.layout).is_err(),
    };
    push(StepInvClauseId::SI24, "program view", "check_program_view failed", si24, &mut v);
    if v.is_empty() { Ok(()) } else { Err(v) }
}

// ================= CheckedOwnerEngine: Root wiring only (finding 5) =================
// Prepare-lane bodies are supplied exactly once as fn pointers by Root from the other lane's owner_prepare module. No default, mock, fallback, prepare/admit/Drop body, or caller FamilyPlans injection into admit exists here; CheckedOwnerEngine cannot be constructed without every dependency.
pub struct PrepareDeps {
    pub prepare_core: fn(&CoreProgram, &BoundsC, &LayoutSchemaC) -> Result<ProgramView, Tooling>,
    pub check_program_view: fn(&ProgramView, &BoundsC, &LayoutSchemaC) -> Result<(), ProgramViewViolation>,
    pub admit: fn(&CoreProgram, &BoundsC, &LayoutSchemaC) -> Result<Admitted, Tooling>,
    pub admit_run: fn(&Admitted, &RunIntC, &RunIntC, &FinanceInput) -> Result<MachineImage, Tooling>,
}
pub struct CheckedOwnerEngine<F> { families: F, guards: OwnerGuards, prepare: PrepareDeps }
impl<F> CheckedOwnerEngine<F> {
    pub(super) fn new_reviewed(families: F, prepare: PrepareDeps) -> Self { Self { families, guards: OwnerGuards::new_owner(), prepare } }
    pub(super) fn checked_guards(&self) -> &OwnerGuards { &self.guards }
    pub(super) fn checked_pre_image<'a>(&self, a: &'a Admitted, img: &'a MachineImage) -> Result<PreImage<'a>, HostViolation> { PreImage::bind(a, img) }
}
impl<F: FamilyPlans> OwnerEngine for CheckedOwnerEngine<F> {
    fn prepare_core(&self, p: &CoreProgram, b: &BoundsC, l: &LayoutSchemaC) -> Result<ProgramView, Tooling> { (self.prepare.prepare_core)(p, b, l) }
    fn check_program_view(&self, v: &ProgramView, b: &BoundsC, l: &LayoutSchemaC) -> Result<(), ProgramViewViolation> { (self.prepare.check_program_view)(v, b, l) }
    fn admit(&self, p: &CoreProgram, b: &BoundsC, l: &LayoutSchemaC) -> Result<Admitted, Tooling> { (self.prepare.admit)(p, b, l) }
    fn admit_run(&self, a: &Admitted, fuel: &RunIntC, stack_limit: &RunIntC, finance: &FinanceInput) -> Result<MachineImage, Tooling> { (self.prepare.admit_run)(a, fuel, stack_limit, finance) }
    fn rule_id_of(&self, raw: &RawRuleId) -> RuleIdC { owner_rule_id_of(raw) }
    fn raw_rule_id(&self, rule: &RuleIdC) -> RawRuleId { owner_raw_rule_id(rule) }
    fn select(&self, pre: &PreImage<'_>) -> Result<Selected, HostViolation> { owner_select(pre, &self.guards) }
    fn family_of(&self, rule: &RawRuleId) -> Option<FamilyId> { owner_family_of(rule) }
    fn step(&self, a: &Admitted, img: &mut MachineImage) -> Result<StepResult, HostViolation> { owner_step(a, img, &self.families, &self.guards) }
    fn run(&self, a: &Admitted, img: &mut MachineImage, sink: &mut dyn FnMut(&StepEventC)) -> Result<RunSummary, HostViolation> { owner_run(a, img, &self.families, &self.guards, sink) }
    fn check_step_inv(&self, pre: &PreImage<'_>) -> Result<(), Vec<StepInvViolation>> { owner_check_step_inv(pre.admitted, pre.img, &self.prepare.check_program_view) }
}

// ============================================================================
// SC05 + R6 owner regression tests, repaired candidate02. SECURITY/hostile
// regression repair author: fresh Muse Wonder 1.0 HIGH under the recorded
// fallback (canonical Opus author call terminated on a session limit with
// no code; no Opus authorship claimed). SOURCE ONLY: UNCOMPILED,
// UNEXECUTED. Direct cells call the real owner_validate_plan with
// test-constructed Selected values and are direct-validation only, never
// selector or end-to-end coverage. Stepped cells call the real owner_step
// via CheckedOwnerEngine::new_reviewed. Hostile plans delegate to the real
// HostFamilyPlans and then mutate one written field; they never fake
// selection, a view, a guard, or provenance.
// ============================================================================
#[cfg(test)]
mod sc05_r6_owner_regression_tests {
    use super::*;
    use crate::sherlock::helper_machine::families::HostFamilyPlans;
    use crate::sherlock::helper_machine::shared::i128_class::KIntC;
    use crate::sherlock::helper_machine::shared::*;

    // ---------------- fixtures: real admission through owner_prepare --------
    fn bounds() -> BoundsC {
        BoundsC { code_rows: 64, function_rows: 8, code_child_slots: 32,
            parameter_slots: 16, pattern_rows: 16, match_plan_rows: 0,
            match_arm_rows: 16, type_rows: 0, constructor_rows: 0,
            constructor_field_slots: 0, max_arity: 4, constant_nodes: 64,
            constant_field_slots: 64, data_nodes: 64, data_field_slots: 64,
            environment_cells: 64, frames: 64, helper_tasks: 64,
            serialization_bytes: 0, max_reference_work: 1000,
            max_helper_work: 1000, step_cap: 1 }
    }
    fn layout(b: &BoundsC) -> LayoutSchemaC {
        LayoutSchemaC { layout_version: ("owner-regression".to_string(), "sc05-r6".to_string()),
            field_id: None, widths: None, scalar_layout_id: None, bounds: *b,
            capacities: LayoutCapacitiesC { name_rows: 16, name_bytes: 128,
                occurrence_path_slots: 128, value_slots: 1000 },
            ext_capacities: None,
            boundary_capacities_present: false }
    }
    fn deps() -> PrepareDeps {
        PrepareDeps { prepare_core: super::super::owner_prepare::prepare_core,
            check_program_view: super::super::owner_prepare::check_program_view,
            admit: super::super::owner_prepare::admit,
            admit_run: super::super::owner_prepare::admit_run }
    }
    fn engine<F: FamilyPlans>(families: F) -> CheckedOwnerEngine<F> {
        CheckedOwnerEngine::new_reviewed(families, deps())
    }
    fn admitted_image(p: &CoreProgram, fuel: u128) -> (Admitted, MachineImage) {
        let b = bounds(); let l = layout(&b);
        let admitted = super::super::owner_prepare::admit(p, &b, &l)
            .unwrap_or_else(|_| panic!("fixture admission refused"));
        let img = super::super::owner_prepare::admit_run(&admitted,
            &RunIntC::Nat(fuel), &RunIntC::Nat(64), &FinanceInput::Absent)
            .unwrap_or_else(|_| panic!("fixture admit_run refused"));
        (admitted, img)
    }
    fn bind_pre<'a>(a: &'a Admitted, img: &'a MachineImage) -> PreImage<'a> {
        match PreImage::bind(a, img) {
            Ok(p) => p,
            Err(e) => panic!("bind failed: {:?} {:?}", e.kind, e.locator),
        }
    }
    fn lit(n: u128) -> CoreExpr { CoreExpr::Literal { value: CoreValue::UInt { n: KIntC::InRange(n) } } }
    fn prog(entry: CoreExpr) -> CoreProgram { CoreProgram { functions: Vec::new(), entry } }
    fn prog_literal() -> CoreProgram { prog(lit(7)) }
    fn prog_list12() -> CoreProgram {
        prog(CoreExpr::Literal { value: CoreValue::List {
            elements: vec![KIntC::InRange(1), KIntC::InRange(2)] } })
    }
    fn prog_let() -> CoreProgram {
        prog(CoreExpr::Let { name: "x".to_string(), value: Box::new(lit(1)),
            body: Box::new(CoreExpr::Var { name: "x".to_string() }) })
    }
    fn prog_match(scrutinee: CoreExpr, pattern: CorePattern, body: CoreExpr) -> CoreProgram {
        prog(CoreExpr::Match { scrutinee: Box::new(scrutinee),
            arms: vec![CoreArm { pattern, body }] })
    }
    fn prog_nameless_nil() -> CoreProgram {
        prog_match(CoreExpr::Literal { value: CoreValue::List { elements: Vec::new() } },
            CorePattern::PNil, lit(7))
    }
    fn prog_with_function() -> CoreProgram {
        CoreProgram { functions: vec![CoreFunction { name: "identity".to_string(),
                parameters: vec![CoreParameter { name: "x".to_string(), sort: SortTag::UInt }],
                return_type: SortTag::UInt,
                body: CoreExpr::Var { name: "x".to_string() } }],
            entry: CoreExpr::Call { name: "identity".to_string(), args: vec![lit(2)] } }
    }

    // ---------- full MachineImage field snapshot for preservation ---------
    #[derive(Debug, PartialEq)]
    enum SnapDataRow { U128(u128), Bool(bool), Nil,
        Cons(usize, usize), Leaf(usize, usize), Branch(usize, usize),
        Constructor(usize, usize, usize, usize) }
    fn snap_data_row(r: &crate::sherlock::helper_machine::raw::DataRow) -> SnapDataRow {
        match r {
            crate::sherlock::helper_machine::raw::DataRow::U128 { n } => SnapDataRow::U128(*n),
            crate::sherlock::helper_machine::raw::DataRow::Bool { b } => SnapDataRow::Bool(*b),
            crate::sherlock::helper_machine::raw::DataRow::Nil => SnapDataRow::Nil,
            crate::sherlock::helper_machine::raw::DataRow::Cons { fields } => SnapDataRow::Cons(fields.start.0, fields.len),
            crate::sherlock::helper_machine::raw::DataRow::Leaf { fields } => SnapDataRow::Leaf(fields.start.0, fields.len),
            crate::sherlock::helper_machine::raw::DataRow::Branch { fields } => SnapDataRow::Branch(fields.start.0, fields.len),
            crate::sherlock::helper_machine::raw::DataRow::Constructor { type_id, ctor_id, fields } => SnapDataRow::Constructor(type_id.0, ctor_id.0, fields.start.0, fields.len),
        }
    }
    #[derive(Debug, PartialEq)]
    struct FullImageSnap { admission_body: usize, run_id: RunIdC,
        control: ControlC, control_origin: usize, env_root: EnvRootC,
        stack_limit: usize, outcome: OutcomeC, initial_fuel: u128,
        remaining: u128, consumed: u128, helper_work: u128,
        helper_visits: u128, cursor: u128, alloc: AllocCounters,
        active: ActiveLengths, helper: Option<HelperControlC>,
        result: Option<HelperResultC>, data: Vec<SnapDataRow>,
        data_fields: Vec<RefC>, env: Vec<EnvCellC>, frames: Vec<FrameRowC>,
        tasks: Vec<HelperTaskC>, value_slots: Vec<RefC> }
    fn snapshot(img: &MachineImage) -> FullImageSnap {
        FullImageSnap {
            admission_body: std::sync::Arc::as_ptr(&img.admission.body) as usize,
            run_id: (*img.run_id).clone(),
            control: img.control, control_origin: img.control_origin,
            env_root: img.env_root, stack_limit: img.stack_limit,
            outcome: img.outcome, initial_fuel: img.initial_fuel,
            remaining: img.remaining_work, consumed: img.consumed_work,
            helper_work: img.helper_work, helper_visits: img.helper_visits,
            cursor: img.reference_cursor, alloc: img.alloc, active: img.active,
            helper: img.helper, result: img.result,
            data: img.data.iter().map(snap_data_row).collect(),
            data_fields: img.data_fields.clone(), env: img.env.clone(),
            frames: img.frames.clone(), tasks: img.tasks.clone(),
            value_slots: img.value_slots.clone() }
    }

    // ---------------- direct owner_validate_plan helpers --------------------
    // These construct Selected as a test fixture for owner_validate_plan
    // only. They are not owner_select and are not end-to-end coverage.
    fn plan_with(w: PlannedWrites) -> EffectPlan {
        let mut d = PlanDelta::default();
        d.alloc_data_nodes = if w.alloc_data.is_some() { 1 } else { 0 };
        d.alloc_data_fields = w.alloc_data.map_or(0, |a| a.field_len() as i64);
        d.alloc_environment_cells = if w.alloc_env.is_some() { 1 } else { 0 };
        d.depth = match w.frame { Some(FrameWrite::Push(_)) => 1, Some(FrameWrite::Pop) => -1, _ => 0 };
        d.task_live = match w.task_op { TaskOp::Push(_) => 1, TaskOp::Pop => -1, TaskOp::None => 0 };
        d.active_value_slots = if w.alloc_value_slot.is_some() { 1 } else { 0 };
        EffectPlan { delta_assert: d, writes: w }
    }
    fn fixture_selected(rule: RuleIdC, hk: Option<HelperKind>) -> Selected {
        let ek = super::event_kind_of(&rule);
        Selected { rule, event_kind: ek, site: None, helper_kind: hk,
            helper_eligible: ek != EventKind::LegacyComplete,
            helper_visits: super::visits_of(&rule) }
    }
    fn validate(pre: &PreImage, sel: &Selected, w: PlannedWrites,
        refs: &[RefC], envs: &[usize]) -> Result<ValidatedPlan, HostViolation> {
        owner_validate_plan(pre, sel, plan_with(w), refs, envs)
    }
    fn must_err_invalid(r: Result<ValidatedPlan, HostViolation>, msg: &str) -> HostViolation {
        match r { Ok(_) => panic!("{}", msg), Err(e) => e }
    }
    fn common(site: SiteTag) -> HelperCommon {
        HelperCommon { site, origin_reference_cursor: 0, parent_resume: None } }
    fn assert_invalid_plan(e: &HostViolation) {
        assert_eq!(e.kind, HostViolationKind::WrongVariant);
        assert_eq!(e.clause, None); assert_eq!(e.rule, None);
        assert_eq!(e.locator.as_deref(), Some("invalid plan"));
    }
    fn match_helper(phase: MatchPhase, tag: PatternTag, x: usize, y: usize) -> HelperControlC {
        HelperControlC::Match { common: common(SiteTag::Cases), scrutinee: RefC::Const(0),
            arm: 0, saved_env: None, partial: None, pat_tag: tag, x, y, field_idx: 0,
            staged: None, phase, status: TriStatus::Pending }
    }

    // ---------------- real run through CheckedOwnerEngine -------------------
    fn run_checked<F: FamilyPlans>(eng: &CheckedOwnerEngine<F>, a: &Admitted,
        img: &mut MachineImage) -> OutcomeC {
        for _ in 0..10000 {
            if img.outcome != OutcomeC::Running { return img.outcome; }
            match eng.step(a, img) {
                Ok(StepResult::Event { .. }) => {
                    let pre = bind_pre(a, img);
                    assert!(eng.check_step_inv(&pre).is_ok(), "N07: StepInv after step");
                }
                Ok(StepResult::Terminal) => break,
                Err(e) => panic!("normal run HostViolation {:?} {:?}", e.kind, e.locator),
            }
        }
        panic!("run did not terminate");
    }

    // ============ SC05: Match active / inactive columns, nameless case ======
    #[test]
    fn sc05_inactive_match_columns_phase_arm_accepted_without_names() {
        let (a, img) = admitted_image(&prog_nameless_nil(), 100);
        let pre = bind_pre(&a, &img);
        assert_eq!(a.body.view.names.len(), 0);
        assert_eq!(a.body.view.arms.len(), 1);
        let mut w = PlannedWrites::default();
        w.helper = Write::Set(Some(match_helper(MatchPhase::Arm, PatternTag::PVar, 0, 0)));
        let sel = fixture_selected(RuleIdC::BHelperMatchEntry, Some(HelperKind::Match));
        assert!(validate(&pre, &sel, w, &[RefC::Const(0)], &[]).is_ok(),
            "inactive zero columns in phase Arm rejected");
    }
    #[test]
    fn sc05_h04_active_match_xy_at_names_len_rejected() {
        let (a, img) = admitted_image(&prog_nameless_nil(), 100);
        let pre = bind_pre(&a, &img);
        let mut w = PlannedWrites::default();
        w.helper = Write::Set(Some(match_helper(MatchPhase::Tag, PatternTag::PVar, 0, 0)));
        let sel = fixture_selected(RuleIdC::BHelperMatchOccurrence, Some(HelperKind::Match));
        let e = must_err_invalid(validate(&pre, &sel, w, &[RefC::Const(0)], &[]),
            "active x at names len accepted");
        assert_invalid_plan(&e);
        let mut h = match_helper(MatchPhase::Arm, PatternTag::PVar, 0, 0);
        if let HelperControlC::Match { arm, .. } = &mut h { *arm = a.body.view.arms.len(); }
        let mut w2 = PlannedWrites::default(); w2.helper = Write::Set(Some(h));
        let sel2 = fixture_selected(RuleIdC::BHelperMatchOccurrence, Some(HelperKind::Match));
        let e2 = must_err_invalid(validate(&pre, &sel2, w2, &[RefC::Const(0)], &[]),
            "arm at arms len accepted");
        assert_invalid_plan(&e2);
    }
    #[test]
    fn sc05_n03_active_match_patterns_with_names_accepted() {
        // One real arm exists, so Match.arm 0 is in range and cannot mask
        // the x/y decision. Names are exactly a and b from the PCons arm.
        let p = prog_match(lit(3),
            CorePattern::PCons { x: "a".to_string(), y: "b".to_string() }, lit(1));
        let (a, img) = admitted_image(&p, 100);
        let pre = bind_pre(&a, &img);
        assert_eq!(a.body.view.names.len(), 2);
        assert_eq!(a.body.view.arms.len(), 1);
        for (tag, x, y) in [(PatternTag::PVar, 1usize, 0usize),
            (PatternTag::PLeaf, 0, 0), (PatternTag::PCons, 0, 1),
            (PatternTag::PBranch, 1, 0)] {
            let mut w = PlannedWrites::default();
            w.helper = Write::Set(Some(match_helper(MatchPhase::Tag, tag, x, y)));
            let sel = fixture_selected(RuleIdC::BHelperMatchOccurrence, Some(HelperKind::Match));
            assert!(validate(&pre, &sel, w, &[RefC::Const(0)], &[]).is_ok(),
                "active pattern x y rejected");
        }
    }
    #[test]
    fn sc05_n02_nameless_pnil_match_halts_and_miss_rejects_9() {
        let eng = engine(HostFamilyPlans);
        let (a, mut img) = admitted_image(&prog_nameless_nil(), 200);
        match run_checked(&eng, &a, &mut img) {
            OutcomeC::Halted { .. } => {}
            other => panic!("nameless nil match outcome {:?}", other), }
        let p = prog_match(CoreExpr::Literal { value: CoreValue::List {
                elements: vec![KIntC::InRange(1)] } }, CorePattern::PNil, lit(0));
        let (a2, mut img2) = admitted_image(&p, 400);
        match run_checked(&eng, &a2, &mut img2) {
            OutcomeC::Rejected { code: 9 } => {}
            other => panic!("nil miss outcome {:?}", other), }
    }

    // ============ SC05: H01 ExprId / H02 FunctionRowId / H03 NameId =========
    #[test]
    fn sc05_h01_expr_id_at_len_rejected_last_accepted() {
        let (a, img) = admitted_image(&prog_literal(), 100);
        let pre = bind_pre(&a, &img);
        let n = a.body.view.expressions.len();
        let sel = fixture_selected(RuleIdC::KMachineEvalLet, None);
        for (expr, ok) in [(n, false), (usize::MAX, false), (n - 1, true)] {
            let mut w = PlannedWrites::default();
            w.control = Write::Set(ControlC::Eval { expr });
            w.result = Write::Set(None);
            match validate(&pre, &sel, w, &[], &[]) {
                Ok(_) => assert!(ok, "Eval expr accepted"),
                Err(e) => { assert!(!ok); assert_invalid_plan(&e); } }
        }
        let frame = FrameRowC::ScopeF { saved_env: None, origin: n };
        let mut w = PlannedWrites::default();
        w.control = Write::Set(ControlC::ScheduleNext { frame, next: 0, env: None });
        w.result = Write::Set(None);
        let e = must_err_invalid(validate(&pre, &sel, w, &[], &[]),
            "frame origin at expressions len accepted");
        assert_invalid_plan(&e);
    }
    #[test]
    fn sc05_h02_function_id_at_len_rejected_last_accepted() {
        let (a, img) = admitted_image(&prog_with_function(), 100);
        let pre = bind_pre(&a, &img);
        assert_eq!(a.body.view.functions.len(), 1);
        let sel = fixture_selected(RuleIdC::KMachineEnterMissing, None);
        for (f, ok) in [(Some(1usize), false), (Some(usize::MAX), false), (Some(0usize), true)] {
            let mut w = PlannedWrites::default();
            w.control = Write::Set(ControlC::Enter { function: f, args: SliceC::empty() });
            w.result = Write::Set(None);
            match validate(&pre, &sel, w, &[], &[]) {
                Ok(_) => assert!(ok, "Enter function accepted"),
                Err(e) => { assert!(!ok); assert_invalid_plan(&e); } }
        }
    }
    #[test]
    fn sc05_h03_name_id_invoke_and_alloc_env_binder() {
        let (a, img) = admitted_image(&prog_let(), 100);
        let pre = bind_pre(&a, &img);
        assert_eq!(a.body.view.names.len(), 1);
        let sel = fixture_selected(RuleIdC::KMachineInvokeFind, None);
        for (name, ok) in [(1usize, false), (usize::MAX, false), (0usize, true)] {
            let mut w = PlannedWrites::default();
            w.control = Write::Set(ControlC::Invoke { function_name: name, args: SliceC::empty() });
            w.result = Write::Set(None);
            match validate(&pre, &sel, w, &[], &[]) {
                Ok(_) => assert!(ok, "Invoke name accepted"),
                Err(e) => { assert!(!ok); assert_invalid_plan(&e); } }
        }
        let sel2 = fixture_selected(RuleIdC::BHelperEnvPutInsert, Some(HelperKind::EnvPut));
        let mut w = PlannedWrites::default();
        w.alloc_env = Some(EnvCellC { binder: 1, value: RefC::Const(0), previous: None });
        let e = must_err_invalid(validate(&pre, &sel2, w, &[RefC::Const(0)], &[]),
            "alloc_env binder at names len accepted");
        assert_invalid_plan(&e);
    }
    #[test]
    fn sc05_h05_const_id_provenance_ok_but_range_rejected() {
        let (a, img) = admitted_image(&prog_literal(), 100);
        let pre = bind_pre(&a, &img);
        let n = a.body.view.constants.len(); assert!(n >= 1);
        let sel = fixture_selected(RuleIdC::KMachineEvalLet, None);
        let bad = RefC::Const(n);
        let mut w = PlannedWrites::default();
        w.control = Write::Set(ControlC::Return { value: bad }); w.result = Write::Set(None);
        let e = must_err_invalid(validate(&pre, &sel, w, &[bad], &[]),
            "Const at constants len accepted");
        assert_invalid_plan(&e);
        let good = RefC::Const(n - 1);
        let mut w2 = PlannedWrites::default();
        w2.control = Write::Set(ControlC::Return { value: good }); w2.result = Write::Set(None);
        assert!(validate(&pre, &sel, w2, &[good], &[]).is_ok());
    }
    #[test]
    fn sc05_h05_const_id_other_holders_rejected() {
        let (a, img) = admitted_image(&prog_let(), 100);
        let pre = bind_pre(&a, &img);
        let n = a.body.view.constants.len(); assert!(n >= 1);
        let bad = RefC::Const(n);
        // Validate helper root holder, root supplied in provenance.
        let mut w = PlannedWrites::default();
        w.helper = Write::Set(Some(HelperControlC::Validate {
            common: common(SiteTag::EvalLiteral), root: bad, status: TriStatus::Pending }));
        let sel = fixture_selected(RuleIdC::BHelperValidateOccurrence, Some(HelperKind::Validate));
        assert_invalid_plan(&must_err_invalid(validate(&pre, &sel, w, &[bad], &[]),
            "Validate root Const at len accepted"));
        // Pushed ValidateTask holder.
        let mut w = PlannedWrites::default();
        w.task_op = TaskOp::Push(HelperTaskC::ValidateTask {
            r: bad, visit: TaskVisitC::Unvisited, cursor: 0 });
        let sel = fixture_selected(RuleIdC::KMachineEvalLet, None);
        assert_invalid_plan(&must_err_invalid(validate(&pre, &sel, w, &[bad], &[]),
            "ValidateTask Const at len accepted"));
        // alloc_value_slot holder.
        let mut w = PlannedWrites::default();
        w.alloc_value_slot = Some(bad);
        let sel = fixture_selected(RuleIdC::KMachineEvalLet, None);
        assert_invalid_plan(&must_err_invalid(validate(&pre, &sel, w, &[bad], &[]),
            "alloc_value_slot Const at len accepted"));
        // alloc_env value holder, binder in range.
        let mut w = PlannedWrites::default();
        w.alloc_env = Some(EnvCellC { binder: 0, value: bad, previous: None });
        let sel = fixture_selected(RuleIdC::BHelperEnvPutInsert, Some(HelperKind::EnvPut));
        assert_invalid_plan(&must_err_invalid(validate(&pre, &sel, w, &[bad], &[]),
            "alloc_env value Const at len accepted"));
        // Pushed RightF frame left holder, origin in range.
        let mut w = PlannedWrites::default();
        w.frame = Some(FrameWrite::Push(FrameRowC::RightF {
            op: BinaryOp::Add, left: bad, origin: 0 }));
        w.result = Write::Set(None);
        let sel = fixture_selected(RuleIdC::KMachineEvalLet, None);
        assert_invalid_plan(&must_err_invalid(validate(&pre, &sel, w, &[bad], &[]),
            "frame left Const at len accepted"));
    }
    #[test]
    fn sc05_n05_boundary_last_ids_accepted() {
        let (a, img) = admitted_image(&prog_with_function(), 100);
        let pre = bind_pre(&a, &img);
        let last_expr = a.body.view.expressions.len() - 1;
        let last_const = a.body.view.constants.len() - 1;
        let last_fn = a.body.view.functions.len() - 1;
        let last_name = a.body.view.names.len() - 1;
        let sel = fixture_selected(RuleIdC::KMachineEvalLet, None);
        let mut w = PlannedWrites::default();
        w.control = Write::Set(ControlC::Eval { expr: last_expr }); w.result = Write::Set(None);
        assert!(validate(&pre, &sel, w, &[], &[]).is_ok());
        let mut w = PlannedWrites::default();
        w.control = Write::Set(ControlC::Enter { function: Some(last_fn), args: SliceC::empty() });
        w.result = Write::Set(None);
        assert!(validate(&pre, &sel, w, &[], &[]).is_ok());
        let mut w = PlannedWrites::default();
        w.control = Write::Set(ControlC::Invoke { function_name: last_name, args: SliceC::empty() });
        w.result = Write::Set(None);
        assert!(validate(&pre, &fixture_selected(RuleIdC::KMachineInvokeFind, None), w, &[], &[]).is_ok());
        let mut w = PlannedWrites::default();
        w.control = Write::Set(ControlC::Return { value: RefC::Const(last_const) });
        w.result = Write::Set(None);
        assert!(validate(&pre, &sel, w, &[RefC::Const(last_const)], &[]).is_ok());
    }

    // ============ SC05: H06 precedence, partial as labelled =================
    #[test]
    fn sc05_h06_parent_resume_si08_precedes_sc05() {
        let (a, img) = admitted_image(&prog_literal(), 100);
        let pre = bind_pre(&a, &img);
        let vh = HelperControlC::Validate { common: HelperCommon { site: SiteTag::EvalLiteral,
            origin_reference_cursor: 0, parent_resume: Some(1) },
            root: RefC::Const(0), status: TriStatus::Pending };
        let mut w = PlannedWrites::default();
        w.helper = Write::Set(Some(vh));
        w.control = Write::Set(ControlC::Eval { expr: usize::MAX });
        let sel = fixture_selected(RuleIdC::BHelperValidateOccurrence, Some(HelperKind::Validate));
        let e = must_err_invalid(validate(&pre, &sel, w, &[RefC::Const(0)], &[]),
            "parentResume plus bad ExprId accepted");
        assert_eq!(e.kind, HostViolationKind::StepInvViolated);
        assert_eq!(e.clause, Some(StepInvClauseId::SI08));
    }
    #[test]
    fn sc05_h06_disabled_rule_still_disabled_not_invalid_plan() {
        let (a, img) = admitted_image(&prog_literal(), 100);
        let pre = bind_pre(&a, &img);
        let sel = fixture_selected(RuleIdC::BHelperEnvPutCell, Some(HelperKind::EnvPut));
        let e = must_err_invalid(validate(&pre, &sel, PlannedWrites::default(), &[], &[]),
            "disabled rule validated");
        assert_eq!(e.kind, HostViolationKind::DisabledRuleReached);
        let selx = fixture_selected(RuleIdC::BHelperValidateDeclaration, Some(HelperKind::Validate));
        let ex = must_err_invalid(validate(&pre, &selx, PlannedWrites::default(), &[], &[]),
            "extension rule validated");
        assert_eq!(ex.kind, HostViolationKind::ExtensionRuleReached);
    }

    // ============ hostile FamilyPlans through the real owner_step ==========
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Mutation { UnreadValidateRoot, UnreadLetEnv }
    struct HostilePlans { inner: HostFamilyPlans, mutation: Mutation }
    impl HostilePlans {
        fn mutate(&self, plan: &mut EffectPlan) {
            match self.mutation {
                Mutation::UnreadValidateRoot => {
                    if let Write::Set(Some(HelperControlC::Validate { root, .. })) = &mut plan.writes.helper {
                        if *root != RefC::Const(0) { *root = RefC::Const(0); } }
                }
                Mutation::UnreadLetEnv => {
                    if let Write::Set(ControlC::ScheduleNext { frame, env, .. }) = &mut plan.writes.control {
                        if let FrameRowC::LetF { saved_env, .. } = frame { *saved_env = Some(0); }
                        *env = Some(0); }
                }
            }
        }
        fn finish(&self, r: Result<EffectPlan, HostViolation>) -> Result<EffectPlan, HostViolation> {
            let mut p = r?; self.mutate(&mut p); Ok(p) }
    }
    impl FamilyPlans for HostilePlans {
        fn plan_legacy(&self, s: &Selected, p: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> { self.finish(self.inner.plan_legacy(s, p, g)) }
        fn plan_scalar(&self, s: &Selected, p: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> { self.finish(self.inner.plan_scalar(s, p, g)) }
        fn plan_env_get(&self, s: &Selected, p: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> { self.finish(self.inner.plan_env_get(s, p, g)) }
        fn plan_env_put(&self, s: &Selected, p: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> { self.finish(self.inner.plan_env_put(s, p, g)) }
        fn plan_find(&self, s: &Selected, p: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> { self.finish(self.inner.plan_find(s, p, g)) }
        fn plan_bind(&self, s: &Selected, p: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> { self.finish(self.inner.plan_bind(s, p, g)) }
        fn plan_append(&self, s: &Selected, p: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> { self.finish(self.inner.plan_append(s, p, g)) }
        fn plan_validate(&self, s: &Selected, p: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> { self.finish(self.inner.plan_validate(s, p, g)) }
        fn plan_equal(&self, s: &Selected, p: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> { self.finish(self.inner.plan_equal(s, p, g)) }
        fn plan_match(&self, s: &Selected, p: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> { self.finish(self.inner.plan_match(s, p, g)) }
        fn plan_build(&self, s: &Selected, p: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> { self.finish(self.inner.plan_build(s, p, g)) }
        fn plan_plan(&self, s: &Selected, p: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> { self.finish(self.inner.plan_plan(s, p, g)) }
        fn plan_bookkeeping_release(&self, s: &Selected, p: &dyn PreImageView, g: &dyn GuardEvaluator) -> Result<EffectPlan, HostViolation> { self.finish(self.inner.plan_bookkeeping_release(s, p, g)) }
    }
    #[test]
    fn r6_refc_unread_written_ref_rejected_step_unmutated() {
        let (a, mut img) = admitted_image(&prog_list12(), 200);
        let eng = engine(HostilePlans { inner: HostFamilyPlans, mutation: Mutation::UnreadValidateRoot });
        let before = snapshot(&img);
        let e = match eng.step(&a, &mut img) {
            Ok(_) => panic!("unread RefC write applied"), Err(e) => e };
        assert_invalid_plan(&e);
        assert_eq!(snapshot(&img), before, "image mutated on SC-07 refusal");
        assert_eq!(img.outcome, OutcomeC::Running);
    }
    #[test]
    fn r6_env_unread_env_id_rejected_step_unmutated() {
        let (a, mut img) = admitted_image(&prog_let(), 200);
        let eng = engine(HostilePlans { inner: HostFamilyPlans, mutation: Mutation::UnreadLetEnv });
        let before = snapshot(&img);
        let e = match eng.step(&a, &mut img) {
            Ok(_) => panic!("unread EnvId write applied"), Err(e) => e };
        assert_invalid_plan(&e);
        assert_eq!(snapshot(&img), before);
    }
    #[test]
    fn sc05_r6_h06_hostile_step_precedes_p3_helper_work_refusal() {
        // Repaired fixture: the Validate root for the list literal is the
        // Cons head constant, not Const(0), so the mutation is effective.
        let (a, mut img) = admitted_image(&prog_list12(), 200);
        img.helper_work = bounds().max_helper_work;
        let eng = engine(HostilePlans { inner: HostFamilyPlans, mutation: Mutation::UnreadValidateRoot });
        let before = snapshot(&img);
        match eng.step(&a, &mut img) {
            Err(e) => assert_invalid_plan(&e),
            Ok(_) => panic!("hostile plan reached P3 or outcome instead of PS refusal"), }
        assert_eq!(snapshot(&img), before);
        assert_eq!(img.outcome, OutcomeC::Running, "Exhausted HelperWork published");
    }

    // ============ R6 EnvId: fresh id, previous, below-active =================
    #[test]
    fn r6_env_alloc_previous_is_never_fresh_self_parent() {
        let (a, img) = admitted_image(&prog_let(), 100);
        let pre = bind_pre(&a, &img);
        let sel = fixture_selected(RuleIdC::BHelperEnvPutInsert, Some(HelperKind::EnvPut));
        let mut w = PlannedWrites::default();
        w.alloc_env = Some(EnvCellC { binder: 0, value: RefC::Const(0), previous: Some(0) });
        let e = must_err_invalid(validate(&pre, &sel, w, &[RefC::Const(0)], &[]),
            "alloc_env previous equal to fresh id accepted");
        assert_invalid_plan(&e);
        let mut w2 = PlannedWrites::default();
        w2.alloc_env = Some(EnvCellC { binder: 0, value: RefC::Const(0), previous: None });
        assert!(validate(&pre, &sel, w2, &[RefC::Const(0)], &[]).is_ok());
    }
    #[test]
    fn r6_env_fresh_id_only_under_alloc_env_and_below_active_rule() {
        let (a, img) = admitted_image(&prog_let(), 100);
        let pre = bind_pre(&a, &img);
        let sel = fixture_selected(RuleIdC::BHelperEnvPutInsert, Some(HelperKind::EnvPut));
        let mut w = PlannedWrites::default(); w.env_root = Write::Set(Some(0));
        let e = must_err_invalid(validate(&pre, &sel, w, &[], &[]),
            "would-be-next EnvId minted without alloc_env");
        assert_invalid_plan(&e);
        let mut w2 = PlannedWrites::default();
        w2.alloc_env = Some(EnvCellC { binder: 0, value: RefC::Const(0), previous: None });
        w2.env_root = Write::Set(Some(0));
        assert!(validate(&pre, &sel, w2, &[RefC::Const(0)], &[]).is_ok());
    }
    #[test]
    fn r6_refc_fresh_data_exception_and_unread_data_rejected() {
        let (a, img) = admitted_image(&prog_literal(), 100);
        let pre = bind_pre(&a, &img);
        let sel = fixture_selected(RuleIdC::BHelperScalarExit, Some(HelperKind::Scalar));
        let mut w = PlannedWrites::default();
        w.alloc_data = Some(AllocDataWrite::U128 { n: 7 });
        w.helper = Write::Set(None);
        w.result = Write::Set(Some(HelperResultC::ScalarValue { value: RefC::Data(0) }));
        assert!(validate(&pre, &sel, w, &[], &[]).is_ok(), "fresh Data exception refused");
        let mut w2 = PlannedWrites::default();
        w2.helper = Write::Set(None);
        w2.result = Write::Set(Some(HelperResultC::ScalarValue { value: RefC::Data(0) }));
        let e = must_err_invalid(validate(&pre, &sel, w2, &[], &[]),
            "unread Data result accepted");
        assert_invalid_plan(&e);
    }

    // ============ N04 / N06 / N07 normal cells ==============================
    #[test]
    fn sc05_n04_call_to_missing_function_rejected_7() {
        let p = prog(CoreExpr::Call { name: "missing".to_string(), args: Vec::new() });
        let eng = engine(HostFamilyPlans);
        let (a, mut img) = admitted_image(&p, 200);
        assert_eq!(a.body.view.functions.len(), 0);
        match run_checked(&eng, &a, &mut img) {
            OutcomeC::Rejected { code: 7 } => {}
            other => panic!("missing function outcome {:?}", other), }
    }
    #[test]
    fn sc05_n06_genesis_invalid_publish_unchanged() {
        let eng = engine(HostFamilyPlans);
        let b = bounds(); let l = layout(&b);
        let a = match super::super::owner_prepare::admit(&prog_literal(), &b, &l) {
            Ok(a) => a, Err(_) => panic!("admit refused") };
        let mut img = match super::super::owner_prepare::admit_run(&a, &RunIntC::Negative,
            &RunIntC::Nat(4), &FinanceInput::Absent) {
            Ok(i) => i, Err(_) => panic!("admit_run refused") };
        assert_eq!(img.control, ControlC::Error { code: 1 });
        match eng.step(&a, &mut img) {
            Ok(StepResult::Event { .. }) => {}
            Ok(StepResult::Terminal) => panic!("genesis publish returned Terminal"),
            Err(e) => panic!("genesis publish HostViolation {:?} {:?}", e.kind, e.locator), }
        assert_eq!(img.outcome, OutcomeC::Rejected { code: 1 });
        assert_eq!(img.consumed_work, 0);
        assert!(matches!(eng.step(&a, &mut img), Ok(StepResult::Terminal)));
    }
    #[test]
    fn sc05_n07_step_inv_ok_throughout_literal_run() {
        let eng = engine(HostFamilyPlans);
        let (a, mut img) = admitted_image(&prog_literal(), 200);
        let pre = bind_pre(&a, &img);
        assert!(eng.check_step_inv(&pre).is_ok());
        match run_checked(&eng, &a, &mut img) {
            OutcomeC::Halted { .. } => {}
            other => panic!("literal outcome {:?}", other), }
    }

    // ============ N01: mandatory full96 differential, UNIMPLEMENTED ========
    // Focused cells above do NOT substitute for this contract. It stays UNRUN
    // until an independently authored candidate04 differential corpus exists.
    #[test]
    #[ignore = "UNIMPLEMENTED: full 96-Enabled positive differential corpus (N01) not supplied; focused regressions do not close it"]
    fn n01_full96_enabled_differential_unimplemented() {
        unimplemented!("UNIMPLEMENTED: all-96-Enabled event-stream and RunSummary differential vs candidate04")
    }
}
