//! Pure, SDK-free presentation of the complete typed observation.
//! Provisional source preparation, uncompiled and unadopted.
//! Every enum is an object with a `$variant` plus every declared payload field.
//! Options use JSON null; arrays retain order and multiplicity. Integers are
//! decimal JSON numbers, including all u128 values without a float conversion.
//! No canonical codec, validation, admission, or bounded encoding is asserted.
use std::fmt::{self, Write};
use std::sync::Arc;
use super::{raw, shared};

/// A lossless presentation writer for one already supplied typed value.
pub trait JsonValue {
    fn write_json<W: Write + ?Sized>(&self, out: &mut W) -> fmt::Result;
}

/// Write one complete JSON value to a caller-owned text sink.
pub fn write_json<W: Write + ?Sized, T: JsonValue + ?Sized>(out: &mut W, value: &T) -> fmt::Result {
    value.write_json(out)
}

/// Build one complete JSON value, preserving sink failure as a Result.
pub fn to_json<T: JsonValue + ?Sized>(value: &T) -> Result<String, fmt::Error> {
    let mut out = String::new();
    value.write_json(&mut out)?;
    Ok(out)
}

fn string<W: Write + ?Sized>(out: &mut W, value: &str) -> fmt::Result {
    out.write_char('"')?;
    for c in value.chars() {
        match c {
            '"' => out.write_str("\\\"")?,
            '\\' => out.write_str("\\\\")?,
            '\u{08}' => out.write_str("\\b")?,
            '\u{0c}' => out.write_str("\\f")?,
            '\n' => out.write_str("\\n")?,
            '\r' => out.write_str("\\r")?,
            '\t' => out.write_str("\\t")?,
            '\u{00}'..='\u{1f}' => write!(out, "\\u{:04x}", c as u32)?,
            c => out.write_char(c)?,
        }
    }
    out.write_char('"')
}
impl JsonValue for str {
    fn write_json<W: Write + ?Sized>(&self, out: &mut W) -> fmt::Result { string(out, self) }
}
impl JsonValue for String {
    fn write_json<W: Write + ?Sized>(&self, out: &mut W) -> fmt::Result { string(out, self) }
}
impl<T: JsonValue + ?Sized> JsonValue for &T {
    fn write_json<W: Write + ?Sized>(&self, out: &mut W) -> fmt::Result { (**self).write_json(out) }
}
impl<T: JsonValue + ?Sized> JsonValue for Arc<T> {
    fn write_json<W: Write + ?Sized>(&self, out: &mut W) -> fmt::Result { (**self).write_json(out) }
}
impl JsonValue for bool {
    fn write_json<W: Write + ?Sized>(&self, out: &mut W) -> fmt::Result {
        out.write_str(if *self { "true" } else { "false" })
    }
}
macro_rules! integer_json {
    ($($ty:ty),* $(,)?) => {$(impl JsonValue for $ty {
        fn write_json<W: Write + ?Sized>(&self, out: &mut W) -> fmt::Result { write!(out, "{}", self) }
    })*};
}
integer_json!(u8, u32, usize, u128, i64);
impl<T: JsonValue> JsonValue for Option<T> {
    fn write_json<W: Write + ?Sized>(&self, out: &mut W) -> fmt::Result {
        match self { Some(value) => value.write_json(out), None => out.write_str("null") }
    }
}
impl<T: JsonValue> JsonValue for [T] {
    fn write_json<W: Write + ?Sized>(&self, out: &mut W) -> fmt::Result {
        out.write_char('[')?;
        for (index, value) in self.iter().enumerate() {
            if index != 0 { out.write_char(',')?; }
            value.write_json(out)?;
        }
        out.write_char(']')
    }
}
impl<T: JsonValue> JsonValue for Vec<T> {
    fn write_json<W: Write + ?Sized>(&self, out: &mut W) -> fmt::Result { self.as_slice().write_json(out) }
}
impl<A: JsonValue, B: JsonValue> JsonValue for (A, B) {
    fn write_json<W: Write + ?Sized>(&self, out: &mut W) -> fmt::Result {
        out.write_char('[')?; self.0.write_json(out)?; out.write_char(',')?;
        self.1.write_json(out)?; out.write_char(']')
    }
}
fn field<W: Write + ?Sized, T: JsonValue + ?Sized>(out: &mut W, first: &mut bool, key: &str, value: &T) -> fmt::Result {
    if !*first { out.write_char(',')?; }
    *first = false;
    string(out, key)?; out.write_char(':')?; value.write_json(out)
}
macro_rules! record_json {
    ($ty:path; $($name:ident),* $(,)?) => {
        impl JsonValue for $ty {
            fn write_json<W: Write + ?Sized>(&self, out: &mut W) -> fmt::Result {
                out.write_char('{')?;
                let mut first = true;
                $(field(out, &mut first, stringify!($name), &self.$name)?;)*
                out.write_char('}')
            }
        }
    };
}
macro_rules! variant_json {
    ($ty:path; $($variant:ident $( { $($name:ident),* $(,)? } )?),* $(,)?) => {
        impl JsonValue for $ty {
            fn write_json<W: Write + ?Sized>(&self, out: &mut W) -> fmt::Result {
                match self {
                    $(Self::$variant $( { $($name),* } )? => {
                        out.write_char('{')?;
                        let mut first = true;
                        field(out, &mut first, "$variant", stringify!($variant))?;
                        $($(field(out, &mut first, stringify!($name), $name)?;)*)?
                        out.write_char('}')
                    }),*
                }
            }
        }
    };
}

record_json!(shared::IdentityC; profile_id, grammar_domain, finance, layout_version, layout_digest, bounds_digest, field_id, program_membership_digest);

record_json!(shared::RunIdC; identity, fuel_input, stack_limit_input);

record_json!(shared::ExpandedStepIdC; run_id, ordinal);

record_json!(shared::AllocDeltasC; data_nodes, data_fields, environment_cells);

record_json!(shared::StepEventC; step_id, rule_id, event_kind, site, helper_kind, disposition, work_delta, helper_work_delta, helper_visits_delta, reference_cursor_delta, alloc_deltas, value_slot_writes, logical_depth_before, logical_depth_after, task_cursor_before, task_cursor_after, helper_phase_before, helper_phase_after, helper_cursor_before, helper_cursor_after, result_tag, origin_reference_cursor, source_expr_id, source_occurrence, exhausted_kind, outcome_tag, outcome_payload, boundary);

record_json!(shared::BoundsC; code_rows, function_rows, code_child_slots, parameter_slots, pattern_rows, match_plan_rows, match_arm_rows, type_rows, constructor_rows, constructor_field_slots, max_arity, constant_nodes, constant_field_slots, data_nodes, data_field_slots, environment_cells, frames, helper_tasks, serialization_bytes, max_reference_work, max_helper_work, step_cap);

record_json!(shared::LayoutCapacitiesC; name_rows, name_bytes, occurrence_path_slots, value_slots);

record_json!(shared::SliceC; start, len);

record_json!(shared::AllocCounters; data_nodes, data_fields, environment_cells);

record_json!(shared::ActiveLengths; data, data_fields, env, value_slots);

record_json!(shared::HelperCommon; site, origin_reference_cursor, parent_resume);

record_json!(shared::EnvCellC; binder, value, previous);

record_json!(shared::FunctionRowC; function_name, params, return_type, body);

record_json!(shared::ParameterRowC; binder, sort);

record_json!(shared::ArmRowC; pattern, body);

record_json!(shared::OccurrenceRowC; owner_is_function, function, path);

record_json!(shared::ProgramView; entry, expressions, occurrences, occurrence_paths, children, functions, parameters, arms, patterns, constants, constant_fields, names);

record_json!(shared::RunSummary; outcome, events, consumed_work, helper_work, helper_visits, reference_cursor);

variant_json!(shared::GrammarDomain;
    DedicatedSorts,
    GenericNominalExtensionReserved
);

variant_json!(shared::FinanceInput;
    Absent,
    TypedExtensionReserved
);

variant_json!(shared::SortTag;
    UInt,
    Bool,
    List,
    Tree
);

variant_json!(shared::BinaryOp;
    Add,
    Sub,
    Mul,
    Eq,
    Lt,
    Cons,
    Branch
);

variant_json!(shared::UnaryOp;
    Leaf,
    Not
);

variant_json!(shared::PatternTag;
    PVar,
    PNil,
    PCons,
    PLeaf,
    PBranch,
    WildcardExtensionReserved,
    ConstructorExtensionReserved
);

variant_json!(shared::BoundaryKind;
    None,
    Terminal,
    TechnicalCut,
    SourceSuspensionExtensionReserved
);

variant_json!(shared::LogicalKind;
    UInt,
    Bool,
    Nil,
    Cons,
    Leaf,
    Branch,
    Marker,
    ConstructorExtensionReserved
);

variant_json!(shared::ScalarMode;
    Add,
    Sub,
    Mul,
    Lt,
    Cons,
    Branch,
    Leaf,
    Not,
    TestBool,
    TestType
);

variant_json!(shared::ScalarPhase;
    First,
    Second,
    Done
);

variant_json!(shared::NodeKind;
    Cons,
    Leaf,
    Branch
);

variant_json!(shared::BoolSelector;
    True,
    False,
    NotBool
);

variant_json!(shared::TriStatus;
    Pending,
    Positive,
    Negative
);

variant_json!(shared::BindPhase;
    Pairing,
    UnwindType,
    UnwindDup,
    UnwindInsert,
    Propagate,
    Finished
);

variant_json!(shared::BindStatus;
    Ok,
    Bad
);

variant_json!(shared::AppendPhase;
    Copy,
    Tail,
    Done
);

variant_json!(shared::MatchPhase;
    Arm,
    Tag,
    Distinct,
    Field,
    Bind,
    Ready,
    NoMatchReady,
    Done
);

variant_json!(shared::HelperKind;
    Scalar,
    EnvGet,
    EnvPut,
    Find,
    Bind,
    Append,
    Validate,
    Equal,
    Match
);

variant_json!(shared::FamilyId;
    Legacy,
    Scalar,
    EnvGet,
    EnvPut,
    Find,
    Bind,
    Append,
    Validate,
    Equal,
    Match,
    BookkeepingRelease
);

variant_json!(shared::EventKind;
    LegacyComplete,
    HelperEnter,
    HelperVisit,
    HelperExit,
    HelperFail,
    Bookkeeping,
    Publish,
    Startup
);

variant_json!(shared::SiteTag;
    EvalLiteral,
    EvalVar,
    EvalLet,
    EvalBinary,
    EvalUnary,
    EvalIf,
    EvalMatch,
    EvalCall,
    Push,
    RetLet,
    RetLeft,
    RetRight,
    RetUnary,
    RetIf,
    RetMatch,
    RetArgs,
    RetCall,
    RetScope,
    Invoke,
    Enter,
    Bound,
    Cases,
    CaseResult,
    Prim
);

variant_json!(shared::ExhaustReason;
    Fuel,
    DataNodes,
    DataFields,
    EnvironmentCells,
    HelperTasks,
    HelperWork,
    SerializationBytes
);

variant_json!(shared::StepDisposition;
    Applied,
    Published,
    ExhaustedFuelUncharged,
    RefusedHelperWorkUncharged,
    RefusedCapCharged
);

variant_json!(shared::TaskVisitC;
    Unvisited,
    Fields
);

variant_json!(shared::RuleIdC;
    KMachineEvalLiteralValid,
    KMachineEvalLiteralInvalid,
    KMachineEvalVariablePresent,
    KMachineEvalVariableAbsent,
    KMachineEvalLet,
    KMachineEvalBinary,
    KMachineEvalUnary,
    KMachineEvalIf,
    KMachineEvalMatch,
    KMachineEvalCallEmpty,
    KMachineEvalCallArguments,
    KMachinePushAccept,
    KMachinePushRefuse,
    KMachineReturnLet,
    KMachineReturnLeft,
    KMachineReturnRight,
    KMachineReturnUnary,
    KMachineReturnIfTrue,
    KMachineReturnIfFalse,
    KMachineReturnIfInvalid,
    KMachineReturnMatch,
    KMachineReturnArgumentsNext,
    KMachineReturnArgumentsFinal,
    KMachineReturnCallValid,
    KMachineReturnCallInvalid,
    KMachineReturnScope,
    KMachineInvokeFind,
    KMachineEnterMissing,
    KMachineEnterBind,
    KMachineBoundInvalid,
    KMachineBoundValid,
    KMachineCasesEmpty,
    KMachineCasesTry,
    KMachineCasesMiss,
    KMachineCasesHit,
    KMachinePrimitiveValue,
    KMachinePrimitiveError,
    BHelperScalarEntry,
    BHelperScalarTag,
    BHelperScalarArithmetic,
    BHelperScalarConstruct,
    BHelperScalarExit,
    BHelperScalarFailure,
    BHelperEnvGetEntry,
    BHelperEnvGetCell,
    BHelperEnvGetEmpty,
    BHelperEnvGetExit,
    BHelperEnvGetFailure,
    BHelperEnvPutEntry,
    BHelperEnvPutCell,
    BHelperEnvPutInsert,
    BHelperEnvPutEmpty,
    BHelperEnvPutExit,
    BHelperEnvPutFailure,
    BHelperFindEntry,
    BHelperFindRow,
    BHelperFindEmpty,
    BHelperFindExit,
    BHelperFindFailure,
    BHelperBindEntry,
    BHelperBindPair,
    BHelperBindEmpty,
    BHelperBindArity,
    BHelperBindType,
    BHelperBindDuplicate,
    BHelperBindInsert,
    BHelperBindPropagate,
    BHelperBindExit,
    BHelperBindFailure,
    BHelperAppendEntry,
    BHelperAppendCell,
    BHelperAppendEmpty,
    BHelperAppendTail,
    BHelperAppendCopy,
    BHelperAppendExit,
    BHelperAppendFailure,
    BHelperValidateEntry,
    BHelperValidateOccurrence,
    BHelperValidateField,
    BHelperValidateDeclaration,
    BHelperValidateEmpty,
    BHelperValidateInvalid,
    BHelperValidateExit,
    BHelperValidateFailure,
    BHelperEqualEntry,
    BHelperEqualPair,
    BHelperEqualField,
    BHelperEqualEmpty,
    BHelperEqualDifferent,
    BHelperEqualExit,
    BHelperEqualFailure,
    BHelperMatchEntry,
    BHelperMatchArm,
    BHelperMatchOccurrence,
    BHelperMatchField,
    BHelperMatchBinding,
    BHelperMatchDistinct,
    BHelperMatchDiscard,
    BHelperMatchNext,
    BHelperMatchSelected,
    BHelperMatchEmpty,
    BHelperMatchExit,
    BHelperMatchFailure,
    BBookkeepingTaskEnqueue,
    BBookkeepingTaskRelease,
    BBookkeepingResumeStore,
    BBookkeepingResumeRestore,
    KAdminStartupValid,
    KAdminStartupInvalid,
    KAdminPublishValue,
    KAdminPublishError,
    KAdminPublishExhausted
);

variant_json!(shared::HelperResultTagC;
    Validated,
    EnvFound,
    EnvMissing,
    EnvUpdated,
    FindFound,
    FindMissing,
    BindOk,
    BindBad,
    AppendDone,
    ScalarValue,
    ScalarError,
    ScalarSelector,
    ScalarTypeTest,
    EqualDone,
    MatchMatched,
    MatchNoMatch
);

variant_json!(shared::OutcomeTagC;
    Running,
    Halted,
    Rejected,
    Exhausted
);

variant_json!(shared::ScalarStaged;
    None,
    U128Value { n },
    BoolValue { b },
    PendingNode { tag },
    PrimError { code },
    Selector { sel },
    TypeTest { ok }
);

variant_json!(shared::HelperControlC;
    Scalar { common, mode, operand_a, operand_b, expected, phase, reg_a, staged },
    EnvGet { common, binder, cursor, found, status },
    EnvPut { common, binder, value, base, new_root, status },
    Find { common, function_name, row_cursor, found, status },
    Bind { common, function, params, args, p_cursor, a_cursor, partial, dup_cursor, phase, status },
    Append { common, src, value, src_cursor, dest_start, staged, phase },
    Validate { common, root, status },
    Equal { common, left, right, status },
    Match { common, scrutinee, arm, saved_env, partial, pat_tag, x, y, field_idx, staged, phase, status }
);

variant_json!(shared::FrameRowC;
    LetF { binder, body, saved_env, origin },
    LeftF { op, right, saved_env, origin },
    RightF { op, left, origin },
    UnaryF { op, origin },
    IfF { then_expr, else_expr, saved_env, origin },
    MatchF { arms, saved_env, origin },
    ArgsF { function_name, completed, rest, saved_env, origin },
    ReturnF { return_type, saved_env, origin },
    ScopeF { saved_env, origin }
);

variant_json!(shared::BoundBinding;
    Bad,
    Ok { env }
);

variant_json!(shared::CaseMatching;
    NoMatch,
    Matched { env }
);

variant_json!(shared::PrimOutcome;
    Value { value },
    Error { code }
);

variant_json!(shared::ControlC;
    Eval { expr },
    Return { value },
    ScheduleNext { frame, next, env },
    Invoke { function_name, args },
    Enter { function, args },
    Bound { binding, return_type, body },
    Cases { value, arms, saved_env },
    CaseResult { matching, value, body, rest, saved_env },
    PrimResult { result },
    Error { code }
);

variant_json!(shared::HelperTaskC;
    ValidateTask { r, visit, cursor },
    EqualTask { left, right, visit, cursor },
    PairTask { binder, sort, arg }
);

variant_json!(shared::HelperResultC;
    Validated { ok },
    EnvFound { value },
    EnvMissing,
    EnvUpdated { cell },
    FindFound { function },
    FindMissing,
    BindOk { env },
    BindBad,
    AppendDone { seq },
    ScalarValue { value },
    ScalarError { code },
    ScalarSelector { sel },
    ScalarTypeTest { ok },
    EqualDone { value },
    MatchMatched { env },
    MatchNoMatch
);

variant_json!(shared::OutcomeC;
    Running,
    Halted { value },
    Rejected { code },
    Exhausted { reason }
);

variant_json!(shared::ExpressionRowC;
    Literal { constant },
    Var { binder },
    Let { binder, value, body },
    Binary { op, left, right },
    Unary { op, operand },
    If { cond, then_expr, else_expr },
    Call { function_name, args },
    Match { scrutinee, arms }
);

variant_json!(shared::PatternRowC;
    PVar { x },
    PNil,
    PCons { x, y },
    PLeaf { x },
    PBranch { x, y }
);

variant_json!(shared::LocatorC;
    Occurrence { owner_is_function, function, ast_path },
    Table { table, row },
    Field { name }
);

variant_json!(shared::DigestC;
    Unselected,
    Selected { format, bytes }
);

impl JsonValue for shared::RefC {
    fn write_json<W: Write + ?Sized>(&self, out: &mut W) -> fmt::Result {
        match self {
            Self::Const(value) => {
                out.write_char('{')?;
                let mut first = true;
                field(out, &mut first, "$variant", "Const")?;
                field(out, &mut first, "id", value)?;
                out.write_char('}')
            },
            Self::Data(value) => {
                out.write_char('{')?;
                let mut first = true;
                field(out, &mut first, "$variant", "Data")?;
                field(out, &mut first, "id", value)?;
                out.write_char('}')
            },
        }
    }
}

impl JsonValue for shared::RunIntC {
    fn write_json<W: Write + ?Sized>(&self, out: &mut W) -> fmt::Result {
        match self {
            Self::Negative => {
                out.write_char('{')?;
                let mut first = true;
                field(out, &mut first, "$variant", "Negative")?;
                out.write_char('}')
            },
            Self::Nat(value) => {
                out.write_char('{')?;
                let mut first = true;
                field(out, &mut first, "$variant", "Nat")?;
                field(out, &mut first, "value", value)?;
                out.write_char('}')
            },
            Self::NatAboveU128 => {
                out.write_char('{')?;
                let mut first = true;
                field(out, &mut first, "$variant", "NatAboveU128")?;
                out.write_char('}')
            },
        }
    }
}

impl JsonValue for shared::HelperPhaseSummaryC {
    fn write_json<W: Write + ?Sized>(&self, out: &mut W) -> fmt::Result {
        match self {
            Self::Scalar(value) => {
                out.write_char('{')?;
                let mut first = true;
                field(out, &mut first, "$variant", "Scalar")?;
                field(out, &mut first, "phase", value)?;
                out.write_char('}')
            },
            Self::Tri(value) => {
                out.write_char('{')?;
                let mut first = true;
                field(out, &mut first, "$variant", "Tri")?;
                field(out, &mut first, "phase", value)?;
                out.write_char('}')
            },
            Self::Bind(value) => {
                out.write_char('{')?;
                let mut first = true;
                field(out, &mut first, "$variant", "Bind")?;
                field(out, &mut first, "phase", value)?;
                out.write_char('}')
            },
            Self::Append(value) => {
                out.write_char('{')?;
                let mut first = true;
                field(out, &mut first, "$variant", "Append")?;
                field(out, &mut first, "phase", value)?;
                out.write_char('}')
            },
            Self::Match(value) => {
                out.write_char('{')?;
                let mut first = true;
                field(out, &mut first, "$variant", "Match")?;
                field(out, &mut first, "phase", value)?;
                out.write_char('}')
            },
        }
    }
}

record_json!(raw::ConstFieldSlice; start, len);

record_json!(raw::FieldSlice; start, len);

impl JsonValue for raw::ConstFieldPos {
    fn write_json<W: Write + ?Sized>(&self, out: &mut W) -> fmt::Result { self.0.write_json(out) }
}

impl JsonValue for raw::FieldPos {
    fn write_json<W: Write + ?Sized>(&self, out: &mut W) -> fmt::Result { self.0.write_json(out) }
}

variant_json!(raw::ConstRow;
    U128 { marker_free, n },
    Bool { marker_free, b },
    Nil { marker_free },
    Cons { marker_free, fields },
    Leaf { marker_free, fields },
    Branch { marker_free, fields },
    U128OutOfRange { marker_free }
);

variant_json!(raw::DataRow;
    U128 { n },
    Bool { b },
    Nil,
    Cons { fields },
    Leaf { fields },
    Branch { fields }
);

/// Present an already emitted complete event without changing its semantics.
pub fn write_step_event_json<W: Write + ?Sized>(out: &mut W, event: &shared::StepEventC) -> fmt::Result {
    event.write_json(out)
}

/// Present every admitted program table and every row payload in order.
pub fn write_program_json<W: Write + ?Sized>(out: &mut W, program: &shared::ProgramView) -> fmt::Result {
    program.write_json(out)
}

// Owner-authored readonly observer declarations, candidate06.
// Private unit seals contain no represented state and are not accessible here.
record_json!(shared::owner_host::HostObservation<'_>; admission, image);
record_json!(shared::owner_host::AdmissionObservation<'_>; identity, bounds, view, layout);
record_json!(shared::owner_host::LayoutObservation<'_>; layout_version, field_id, widths, scalar_layout_id, bounds, capacities, boundary_capacities_present);
record_json!(shared::owner_host::WidthTableObservation<'_>; index_widths, counter_widths, record_widths);
impl JsonValue for shared::owner_host::ImageObservation<'_> {
    fn write_json<W: Write + ?Sized>(&self, out: &mut W) -> fmt::Result {
        out.write_char('{')?;
        let mut first = true;
        field(out, &mut first, "run_id", &self.run_id)?;
        field(out, &mut first, "control", &self.control)?;
        field(out, &mut first, "control_origin", &self.control_origin)?;
        field(out, &mut first, "env_root", &self.env_root)?;
        field(out, &mut first, "stack_limit", &self.stack_limit)?;
        field(out, &mut first, "outcome", &self.outcome)?;
        field(out, &mut first, "initial_fuel", &self.initial_fuel)?;
        field(out, &mut first, "remaining_work", &self.remaining_work)?;
        field(out, &mut first, "consumed_work", &self.consumed_work)?;
        field(out, &mut first, "helper_work", &self.helper_work)?;
        field(out, &mut first, "helper_visits", &self.helper_visits)?;
        field(out, &mut first, "reference_cursor", &self.reference_cursor)?;
        field(out, &mut first, "alloc", &self.alloc)?;
        field(out, &mut first, "active", &self.active)?;
        field(out, &mut first, "helper", &self.helper)?;
        field(out, &mut first, "result", &self.result)?;
        field(out, &mut first, "seg_base", &self.seg_base)?;
        field(out, &mut first, "data", &self.data)?;
        field(out, &mut first, "data_fields", &self.data_fields)?;
        field(out, &mut first, "env", &self.env)?;
        field(out, &mut first, "frames", &self.frames)?;
        field(out, &mut first, "tasks", &self.tasks)?;
        field(out, &mut first, "value_slots", &self.value_slots)?;
        field(out, &mut first, "depth", &self.depth())?;
        field(out, &mut first, "taskLive", &self.task_live())?;
        out.write_char('}')
    }
}

/// Present all fields of the owner's supplied bound observation.
pub fn write_host_observation_json<W: Write + ?Sized>(
    out: &mut W, observation: &shared::owner_host::HostObservation<'_>,
) -> fmt::Result {
    observation.write_json(out)
}

/// Present the owner's typed segment-prefix value through its readonly getters.
impl JsonValue for shared::host_implied_prefix::SegBase {
    fn write_json<W: Write + ?Sized>(&self, out: &mut W) -> fmt::Result {
        out.write_char('{')?;
        let mut first = true;
        field(out, &mut first, "active", &self.active())?;
        field(out, &mut first, "alloc", &self.alloc())?;
        field(out, &mut first, "consumed_work", &self.consumed_work())?;
        out.write_char('}')
    }
}
