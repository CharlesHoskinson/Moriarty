//! Versioned physical lane view of the generic C6 machine root and frame stack.
//!
//! Profile `sherlock-c6-phys/1` (C6P1), Chunk 1: one root+frames image section,
//! plus owned recorders around the existing fixed owner step for A135
//! (`KMachinePushAccept`) and A140 (`KMachinePushRefuse`).
//!
//! This module is a codec and an owned-record consumer. It does not admit
//! programs, construct `Admitted`/`MachineImage`, select rules, evaluate, or step
//! twice. The section carries no program binding, helper, result, task, data,
//! env-cell or value-slot table; those are Chunk 3/4 and remain OPEN. No native,
//! proof, encoder-conformance or full-state physical POST claim is made.
//!
//! Layout (lane offsets are the `L_*` constants): magic, version, 17-lane
//! control, control origin, 2-lane env root, stack limit, 3-lane outcome,
//! six u128 counters, alloc(3), active(4), seg active(4), seg alloc(3), seg
//! consumed work, frame count; then `frame_count` 13-lane frames bottom-to-top.
//!
//! `decode_image_section` first-error precedence:
//! 1. fewer than 48 lanes: `Truncated`; 2. `BadMagic`; 3. `BadVersion`;
//! 4. control record, structurally, ascending lanes (record-relative lane
//!    numbers; an embedded frame reports section "frame");
//! 5. root lanes 19..=47 structurally in ascending lane order (usize/u8
//!    representability, env/outcome/ref tags, padding; section-absolute lanes);
//! 6. representation caps: control, then root fields in lane order, then the
//!    frame count against `max_frames`;
//! 7. exact length: `Truncated`, then `TrailingLanes` (no allocation before);
//! 8. frames bottom-to-top, each structurally then against caps.
//! Indices must be strictly below their table cap; counts at most their cap;
//! a slice is checked for end overflow, then arity, then end <= table cap.
//! Builtin TypeId exception: the five TypeId index fields (ReturnF.return_type,
//! ConstructorFieldsF.type_id and .field_type, Bound.return_type,
//! Materialize.type_id) are checked against the effective cap
//! `max(max_types, 4)`, because builtin TypeIds 0..=3 exist even when
//! `type_rows < 4` (owner SI-15 `types.len().max(4)`; dedicated
//! `check_program_view` uses 4 types). `max_types` itself stays the declared
//! `type_rows` and still caps the program `types`/`type_args` table counts.
//! `AllocCounters::data_fields`/`ActiveLengths::data_fields` have no caps field
//! and are carried exactly. Membership of nominal IDs in admitted tables is
//! cross-section work (Chunk 4) and is not claimed.

use super::complete_observation::{
    capture, record_step, CaptureStepError, CompleteStepRecord, HostSnapshot, ImageSnapshot,
    ObservedStep,
};
use super::shared::adt::{CtorIdC, TypeIdC};
use super::shared::*;

pub const C6_PHYS_MAGIC: u128 = 0x4336_5048_5953_31;
pub const C6_PHYS_VERSION: u128 = 1;
pub const FRAME_LANES: usize = 13;
pub const CONTROL_LANES: usize = 17;
pub const IMAGE_SECTION_HEADER_LANES: usize = 48;
/// A140 writes `ControlC::Error { code: 4 }`; publication is a later step.
pub const PUSH_REFUSE_ERROR_CODE: u8 = 4;

pub const L_MAGIC: usize = 0;
pub const L_VERSION: usize = 1;
pub const L_CONTROL: usize = 2;
pub const L_CONTROL_ORIGIN: usize = 19;
pub const L_ENV_ROOT: usize = 20;
pub const L_STACK_LIMIT: usize = 22;
pub const L_OUTCOME: usize = 23;
pub const L_INITIAL_FUEL: usize = 26;
pub const L_REMAINING_WORK: usize = 27;
pub const L_CONSUMED_WORK: usize = 28;
pub const L_HELPER_WORK: usize = 29;
pub const L_HELPER_VISITS: usize = 30;
pub const L_REFERENCE_CURSOR: usize = 31;
pub const L_ALLOC: usize = 32;
pub const L_ACTIVE: usize = 35;
pub const L_SEG_ACTIVE: usize = 39;
pub const L_SEG_ALLOC: usize = 43;
pub const L_SEG_CONSUMED_WORK: usize = 46;
pub const L_FRAME_COUNT: usize = 47;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PushSelection { Accept, Refuse }

impl PushSelection {
    pub fn rule(self) -> RuleIdC {
        match self {
            Self::Accept => RuleIdC::KMachinePushAccept,
            Self::Refuse => RuleIdC::KMachinePushRefuse,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum C6PhysError {
    BadMagic { actual: u128 },
    BadVersion { actual: u128 },
    Truncated { section: &'static str, needed: usize, actual: usize },
    TrailingLanes { section: &'static str, extra: usize },
    BadTag { section: &'static str, tag: u128 },
    BadBool { section: &'static str, value: u128 },
    NonZeroPadding { section: &'static str, lane: usize, value: u128 },
    UsizeOverflow { section: &'static str, value: u128 },
    CapacityExceeded { section: &'static str, needed: usize, limit: usize },
    WrongControl { section: &'static str },
    /// The owner would not select a Push row here (terminal, publication, fuel,
    /// helper or stale result), in owner priority order.
    NotPushSelected { section: &'static str },
    /// The state is outside the step-invariant domain this layer accepts.
    StepInvDomain { section: &'static str },
    /// Logical Accept, but the owner's physical frame check would refuse.
    PhysicalFrameCapacity { depth: usize, frames: usize },
    UnexpectedPushSelection { expected: PushSelection, actual: PushSelection },
    EventMismatch { section: &'static str },
    AfterMismatch { section: &'static str },
    CounterOverflow { section: &'static str },
}

pub type C6Result<T> = Result<T, C6PhysError>;

/// Representation caps. Counts are inclusive limits; indices are strictly below.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct C6PhysCaps {
    pub max_frames: usize,
    pub max_expressions: usize,
    pub max_names: usize,
    pub max_arms: usize,
    pub max_children: usize,
    pub max_value_slots: usize,
    pub max_constants: usize,
    pub max_data_nodes: usize,
    pub max_env_cells: usize,
    pub max_functions: usize,
    pub max_arity: usize,
    pub max_types: usize,
    pub max_occurrences: usize,
    pub max_occurrence_paths: usize,
    pub max_parameters: usize,
    pub max_patterns: usize,
    pub max_pattern_children: usize,
    pub max_constructors: usize,
    pub max_ctor_fields: usize,
    pub max_decl_templates: usize,
    pub max_plan_nodes: usize,
    pub max_plan_cases: usize,
    pub max_field_paths: usize,
    pub max_constant_fields: usize,
    pub max_name_bytes: usize,
    /// Codec work budget, not an admitted limit: type-expression nodes decoded.
    pub max_type_expr_nodes: usize,
    /// Codec work budget, not an admitted limit: declaration-template text bytes.
    pub max_template_bytes: usize,
}

/// Codec work budgets that have no admitted counterpart.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct C6CodecBudgets {
    pub type_expr_nodes: usize,
    pub template_bytes: usize,
}

impl C6PhysCaps {
    /// Exact admitted limits; never raised. Declarations are `type_rows - 4`.
    /// `max_types` is the declared `type_rows` and caps table counts; the five
    /// TypeId index fields use `max(max_types, 4)` so builtin TypeIds 0..=3
    /// remain representable when `type_rows < 4`.
    pub fn from_parts(bounds: &BoundsC, capacities: &LayoutCapacitiesC, ext: &ExtCapacitiesC,
        budgets: C6CodecBudgets) -> Self {
        Self {
            max_frames: bounds.frames,
            max_expressions: bounds.code_rows,
            max_names: capacities.name_rows,
            max_arms: bounds.match_arm_rows,
            max_children: bounds.code_child_slots,
            max_value_slots: capacities.value_slots,
            max_constants: bounds.constant_nodes,
            max_data_nodes: bounds.data_nodes,
            max_env_cells: bounds.environment_cells,
            max_functions: bounds.function_rows,
            max_arity: bounds.max_arity,
            max_types: bounds.type_rows,
            max_occurrences: bounds.code_rows,
            max_occurrence_paths: capacities.occurrence_path_slots,
            max_parameters: bounds.parameter_slots,
            max_patterns: bounds.pattern_rows,
            max_pattern_children: ext.pattern_child_slots,
            max_constructors: bounds.constructor_rows,
            max_ctor_fields: bounds.constructor_field_slots,
            max_decl_templates: bounds.type_rows.saturating_sub(4),
            max_plan_nodes: bounds.match_plan_rows,
            max_plan_cases: ext.plan_case_slots,
            max_field_paths: ext.field_path_slots,
            max_constant_fields: bounds.constant_field_slots,
            max_name_bytes: capacities.name_bytes,
            max_type_expr_nodes: budgets.type_expr_nodes,
            max_template_bytes: budgets.template_bytes,
        }
    }

    pub fn from_layout(layout: &LayoutSchemaC, budgets: C6CodecBudgets) -> Self {
        let ext = ExtCapacitiesC::effective_for_extended_domain(layout.ext_capacities);
        Self::from_parts(&layout.bounds, &layout.capacities, &ext, budgets)
    }

    /// Convenience caps for tests and tooling. Not admitted limits.
    pub const fn test_caps() -> Self {
        Self {
            max_frames: 64, max_expressions: 256, max_names: 256, max_arms: 64,
            max_children: 256, max_value_slots: 1024, max_constants: 256,
            max_data_nodes: 256, max_env_cells: 256, max_functions: 64, max_arity: 8,
            max_types: 64, max_occurrences: 256, max_occurrence_paths: 4096,
            max_parameters: 256, max_patterns: 256, max_pattern_children: 256,
            max_constructors: 256, max_ctor_fields: 256, max_decl_templates: 64,
            max_plan_nodes: 256, max_plan_cases: 256, max_field_paths: 4096,
            max_constant_fields: 256, max_name_bytes: 8192, max_type_expr_nodes: 4096,
            max_template_bytes: 8192,
        }
    }
}

// ---------- lane primitives ----------

/// usize -> u128 is lossless on every supported target (usize <= 64 bits).
const fn u(v: usize) -> u128 { v as u128 }

fn rd(section: &'static str, value: u128) -> C6Result<usize> {
    usize::try_from(value).map_err(|_| C6PhysError::UsizeOverflow { section, value })
}
fn rd_u8(section: &'static str, value: u128) -> C6Result<u8> {
    u8::try_from(value).map_err(|_| C6PhysError::UsizeOverflow { section, value })
}
fn rd_bool(section: &'static str, value: u128) -> C6Result<bool> {
    match value { 0 => Ok(false), 1 => Ok(true), _ => Err(C6PhysError::BadBool { section, value }) }
}
/// Lanes `from..to` of `lanes` must be zero; `lane` is an index into `lanes`.
fn zero(section: &'static str, lanes: &[u128], from: usize, to: usize) -> C6Result<()> {
    for (lane, &value) in lanes.iter().enumerate().take(to).skip(from) {
        if value != 0 { return Err(C6PhysError::NonZeroPadding { section, lane, value }); }
    }
    Ok(())
}
fn put_env(out: &mut [u128], at: usize, env: EnvRootC) {
    if let Some(index) = env { out[at] = 1; out[at + 1] = u(index); }
}
fn get_env(section: &'static str, lanes: &[u128], at: usize) -> C6Result<EnvRootC> {
    match lanes[at] {
        0 => { zero(section, lanes, at + 1, at + 2)?; Ok(None) }
        1 => Ok(Some(rd(section, lanes[at + 1])?)),
        tag => Err(C6PhysError::BadTag { section, tag }),
    }
}
fn put_ref(out: &mut [u128], at: usize, value: RefC) {
    match value {
        RefC::Const(index) => out[at + 1] = u(index),
        RefC::Data(index) => { out[at] = 1; out[at + 1] = u(index); }
    }
}
fn get_ref(section: &'static str, lanes: &[u128], at: usize) -> C6Result<RefC> {
    match lanes[at] {
        0 => Ok(RefC::Const(rd(section, lanes[at + 1])?)),
        1 => Ok(RefC::Data(rd(section, lanes[at + 1])?)),
        tag => Err(C6PhysError::BadTag { section, tag }),
    }
}
fn put_slice(out: &mut [u128], at: usize, value: SliceC) {
    out[at] = u(value.start);
    out[at + 1] = u(value.len);
}
fn get_slice(section: &'static str, lanes: &[u128], at: usize) -> C6Result<SliceC> {
    Ok(SliceC { start: rd(section, lanes[at])?, len: rd(section, lanes[at + 1])? })
}
fn binary_tag(op: BinaryOp) -> u128 {
    match op { BinaryOp::Add => 0, BinaryOp::Sub => 1, BinaryOp::Mul => 2, BinaryOp::Eq => 3,
        BinaryOp::Lt => 4, BinaryOp::Cons => 5, BinaryOp::Branch => 6 }
}
fn binary_from(section: &'static str, tag: u128) -> C6Result<BinaryOp> {
    Ok(match tag { 0 => BinaryOp::Add, 1 => BinaryOp::Sub, 2 => BinaryOp::Mul, 3 => BinaryOp::Eq,
        4 => BinaryOp::Lt, 5 => BinaryOp::Cons, 6 => BinaryOp::Branch,
        _ => return Err(C6PhysError::BadTag { section, tag }) })
}
fn unary_tag(op: UnaryOp) -> u128 { match op { UnaryOp::Leaf => 0, UnaryOp::Not => 1 } }
fn unary_from(section: &'static str, tag: u128) -> C6Result<UnaryOp> {
    match tag { 0 => Ok(UnaryOp::Leaf), 1 => Ok(UnaryOp::Not), _ => Err(C6PhysError::BadTag { section, tag }) }
}
fn exhaust_tag(reason: ExhaustReason) -> u128 {
    match reason { ExhaustReason::Fuel => 0, ExhaustReason::DataNodes => 1, ExhaustReason::DataFields => 2,
        ExhaustReason::EnvironmentCells => 3, ExhaustReason::HelperTasks => 4,
        ExhaustReason::HelperWork => 5, ExhaustReason::SerializationBytes => 6 }
}
fn exhaust_from(section: &'static str, tag: u128) -> C6Result<ExhaustReason> {
    Ok(match tag { 0 => ExhaustReason::Fuel, 1 => ExhaustReason::DataNodes, 2 => ExhaustReason::DataFields,
        3 => ExhaustReason::EnvironmentCells, 4 => ExhaustReason::HelperTasks,
        5 => ExhaustReason::HelperWork, 6 => ExhaustReason::SerializationBytes,
        _ => return Err(C6PhysError::BadTag { section, tag }) })
}

// ---------- representation caps ----------

fn cap_count(section: &'static str, needed: usize, limit: usize) -> C6Result<()> {
    if needed > limit { Err(C6PhysError::CapacityExceeded { section, needed, limit }) } else { Ok(()) }
}
fn cap_index(section: &'static str, index: usize, limit: usize) -> C6Result<()> {
    let needed = index.checked_add(1).ok_or(C6PhysError::UsizeOverflow { section, value: u(index) })?;
    cap_count(section, needed, limit)
}
fn cap_env(section: &'static str, env: EnvRootC, caps: &C6PhysCaps) -> C6Result<()> {
    match env { Some(index) => cap_index(section, index, caps.max_env_cells), None => Ok(()) }
}
/// Const indexes the program constant table; Data indexes the data arena.
fn cap_ref(section: &'static str, value: RefC, caps: &C6PhysCaps) -> C6Result<()> {
    match value {
        RefC::Const(index) => cap_index(section, index, caps.max_constants),
        RefC::Data(index) => cap_index(section, index, caps.max_data_nodes),
    }
}
fn cap_slice(section: &'static str, s: SliceC, table: usize, max_len: Option<usize>) -> C6Result<()> {
    let end = s.start.checked_add(s.len).ok_or(C6PhysError::UsizeOverflow { section, value: u(s.start) })?;
    if let Some(max_len) = max_len { cap_count(section, s.len, max_len)?; }
    cap_count(section, end, table)
}
fn cap_vals(section: &'static str, s: SliceC, caps: &C6PhysCaps) -> C6Result<()> {
    cap_slice(section, s, caps.max_value_slots, Some(caps.max_arity))
}
fn cap_kids(section: &'static str, s: SliceC, caps: &C6PhysCaps) -> C6Result<()> {
    cap_slice(section, s, caps.max_children, Some(caps.max_arity))
}
/// TypeId index cap: builtin TypeIds 0..=3 exist independently of `type_rows`
/// (owner SI-15 `types.len().max(4)`); larger nominal caps apply unchanged.
fn type_index_cap(c: &C6PhysCaps) -> usize { c.max_types.max(4) }

fn frame_caps(frame: &FrameRowC, c: &C6PhysCaps) -> C6Result<()> {
    use FrameRowC as F;
    let x = c.max_expressions;
    match *frame {
        F::LetF { binder, body, saved_env, origin } => {
            cap_index("frame.binder", binder, c.max_names)?;
            cap_index("frame.body", body, x)?;
            cap_env("frame.saved_env", saved_env, c)?;
            cap_index("frame.origin", origin, x)
        }
        F::LeftF { right, saved_env, origin, .. } => {
            cap_index("frame.right", right, x)?;
            cap_env("frame.saved_env", saved_env, c)?;
            cap_index("frame.origin", origin, x)
        }
        F::RightF { left, origin, .. } => {
            cap_ref("frame.left", left, c)?;
            cap_index("frame.origin", origin, x)
        }
        F::UnaryF { origin, .. } => cap_index("frame.origin", origin, x),
        F::IfF { then_expr, else_expr, saved_env, origin } => {
            cap_index("frame.then_expr", then_expr, x)?;
            cap_index("frame.else_expr", else_expr, x)?;
            cap_env("frame.saved_env", saved_env, c)?;
            cap_index("frame.origin", origin, x)
        }
        F::MatchF { arms, saved_env, origin } => {
            cap_slice("frame.arms", arms, c.max_arms, None)?;
            cap_env("frame.saved_env", saved_env, c)?;
            cap_index("frame.origin", origin, x)
        }
        F::ArgsF { function_name, completed, rest, saved_env, origin } => {
            cap_index("frame.function_name", function_name, c.max_names)?;
            cap_vals("frame.completed", completed, c)?;
            cap_kids("frame.rest", rest, c)?;
            cap_env("frame.saved_env", saved_env, c)?;
            cap_index("frame.origin", origin, x)
        }
        F::ReturnF { return_type, saved_env, origin } => {
            cap_index("frame.return_type", return_type.0, type_index_cap(c))?;
            cap_env("frame.saved_env", saved_env, c)?;
            cap_index("frame.origin", origin, x)
        }
        F::ScopeF { saved_env, origin } => {
            cap_env("frame.saved_env", saved_env, c)?;
            cap_index("frame.origin", origin, x)
        }
        F::ConstructorFieldsF { type_id, ctor_id, index, field_type, completed, rest, saved_env, origin, .. } => {
            cap_index("frame.type_id", type_id.0, type_index_cap(c))?;
            cap_index("frame.ctor_id", ctor_id.0, c.max_constructors)?;
            cap_index("frame.index", index, c.max_arity)?;
            cap_index("frame.field_type", field_type.0, type_index_cap(c))?;
            cap_vals("frame.completed", completed, c)?;
            cap_kids("frame.rest", rest, c)?;
            cap_env("frame.saved_env", saved_env, c)?;
            cap_index("frame.origin", origin, x)
        }
    }
}

fn control_caps(control: &ControlC, c: &C6PhysCaps) -> C6Result<()> {
    use ControlC as K;
    let x = c.max_expressions;
    match *control {
        K::Eval { expr } => cap_index("control.expr", expr, x),
        K::Return { value } => cap_ref("control.value", value, c),
        K::ScheduleNext { frame, next, env } => {
            frame_caps(&frame, c)?;
            cap_index("control.next", next, x)?;
            cap_env("control.env", env, c)
        }
        K::Invoke { function_name, args } => {
            cap_index("control.function_name", function_name, c.max_names)?;
            cap_vals("control.args", args, c)
        }
        K::Enter { function, args } => {
            if let Some(function) = function { cap_index("control.function", function, c.max_functions)?; }
            cap_vals("control.args", args, c)
        }
        K::Bound { binding, return_type, body } => {
            if let BoundBinding::Ok { env } = binding { cap_env("control.env", env, c)?; }
            cap_index("control.return_type", return_type.0, type_index_cap(c))?;
            cap_index("control.body", body, x)
        }
        K::Cases { value, arms, saved_env } => {
            cap_ref("control.value", value, c)?;
            cap_slice("control.arms", arms, c.max_arms, None)?;
            cap_env("control.saved_env", saved_env, c)
        }
        K::CaseResult { matching, value, body, rest, saved_env } => {
            if let CaseMatching::Matched { env } = matching { cap_env("control.env", env, c)?; }
            cap_ref("control.value", value, c)?;
            cap_index("control.body", body, x)?;
            cap_slice("control.rest", rest, c.max_arms, None)?;
            cap_env("control.saved_env", saved_env, c)
        }
        K::PrimResult { result: PrimOutcome::Value { value } } => cap_ref("control.value", value, c),
        K::PrimResult { result: PrimOutcome::Error { .. } } | K::Error { .. } => Ok(()),
        K::Materialize { type_id, ctor_id, fields } => {
            cap_index("control.type_id", type_id.0, type_index_cap(c))?;
            cap_index("control.ctor_id", ctor_id.0, c.max_constructors)?;
            cap_vals("control.fields", fields, c)
        }
    }
}

// ---------- fixed-width records ----------

/// Infallible 13-lane frame encoder. Caps are applied by the section layer.
pub fn encode_frame(frame: &FrameRowC) -> [u128; FRAME_LANES] {
    use FrameRowC as F;
    let mut o = [0u128; FRAME_LANES];
    match *frame {
        F::LetF { binder, body, saved_env, origin } => {
            o[1] = u(binder); o[2] = u(body); put_env(&mut o, 3, saved_env); o[5] = u(origin);
        }
        F::LeftF { op, right, saved_env, origin } => {
            o[0] = 1; o[1] = binary_tag(op); o[2] = u(right); put_env(&mut o, 3, saved_env); o[5] = u(origin);
        }
        F::RightF { op, left, origin } => {
            o[0] = 2; o[1] = binary_tag(op); put_ref(&mut o, 2, left); o[4] = u(origin);
        }
        F::UnaryF { op, origin } => { o[0] = 3; o[1] = unary_tag(op); o[2] = u(origin); }
        F::IfF { then_expr, else_expr, saved_env, origin } => {
            o[0] = 4; o[1] = u(then_expr); o[2] = u(else_expr); put_env(&mut o, 3, saved_env); o[5] = u(origin);
        }
        F::MatchF { arms, saved_env, origin } => {
            o[0] = 5; put_slice(&mut o, 1, arms); put_env(&mut o, 3, saved_env); o[5] = u(origin);
        }
        F::ArgsF { function_name, completed, rest, saved_env, origin } => {
            o[0] = 6; o[1] = u(function_name); put_slice(&mut o, 2, completed); put_slice(&mut o, 4, rest);
            put_env(&mut o, 6, saved_env); o[8] = u(origin);
        }
        F::ReturnF { return_type, saved_env, origin } => {
            o[0] = 7; o[1] = u(return_type.0); put_env(&mut o, 2, saved_env); o[4] = u(origin);
        }
        F::ScopeF { saved_env, origin } => { o[0] = 8; put_env(&mut o, 1, saved_env); o[3] = u(origin); }
        F::ConstructorFieldsF { type_id, ctor_id, index, field_type, checked, completed, rest, saved_env, origin } => {
            o[0] = 9; o[1] = u(type_id.0); o[2] = u(ctor_id.0); o[3] = u(index); o[4] = u(field_type.0);
            o[5] = u128::from(checked); put_slice(&mut o, 6, completed); put_slice(&mut o, 8, rest);
            put_env(&mut o, 10, saved_env); o[12] = u(origin);
        }
    }
    o
}

/// Structural frame decode: tags, representability and padding, ascending lanes.
pub fn decode_frame(l: &[u128; FRAME_LANES]) -> C6Result<FrameRowC> {
    use FrameRowC as F;
    const S: &str = "frame";
    let (frame, used) = match l[0] {
        0 => (F::LetF { binder: rd(S, l[1])?, body: rd(S, l[2])?, saved_env: get_env(S, l, 3)?,
            origin: rd(S, l[5])? }, 6),
        1 => (F::LeftF { op: binary_from(S, l[1])?, right: rd(S, l[2])?, saved_env: get_env(S, l, 3)?,
            origin: rd(S, l[5])? }, 6),
        2 => (F::RightF { op: binary_from(S, l[1])?, left: get_ref(S, l, 2)?, origin: rd(S, l[4])? }, 5),
        3 => (F::UnaryF { op: unary_from(S, l[1])?, origin: rd(S, l[2])? }, 3),
        4 => (F::IfF { then_expr: rd(S, l[1])?, else_expr: rd(S, l[2])?, saved_env: get_env(S, l, 3)?,
            origin: rd(S, l[5])? }, 6),
        5 => (F::MatchF { arms: get_slice(S, l, 1)?, saved_env: get_env(S, l, 3)?, origin: rd(S, l[5])? }, 6),
        6 => (F::ArgsF { function_name: rd(S, l[1])?, completed: get_slice(S, l, 2)?, rest: get_slice(S, l, 4)?,
            saved_env: get_env(S, l, 6)?, origin: rd(S, l[8])? }, 9),
        7 => (F::ReturnF { return_type: TypeIdC(rd(S, l[1])?), saved_env: get_env(S, l, 2)?,
            origin: rd(S, l[4])? }, 5),
        8 => (F::ScopeF { saved_env: get_env(S, l, 1)?, origin: rd(S, l[3])? }, 4),
        9 => (F::ConstructorFieldsF { type_id: TypeIdC(rd(S, l[1])?), ctor_id: CtorIdC(rd(S, l[2])?),
            index: rd(S, l[3])?, field_type: TypeIdC(rd(S, l[4])?), checked: rd_bool(S, l[5])?,
            completed: get_slice(S, l, 6)?, rest: get_slice(S, l, 8)?, saved_env: get_env(S, l, 10)?,
            origin: rd(S, l[12])? }, FRAME_LANES),
        tag => return Err(C6PhysError::BadTag { section: S, tag }),
    };
    zero(S, l, used, FRAME_LANES)?;
    Ok(frame)
}

pub fn decode_frame_checked(lanes: &[u128; FRAME_LANES], caps: &C6PhysCaps) -> C6Result<FrameRowC> {
    let frame = decode_frame(lanes)?;
    frame_caps(&frame, caps)?;
    Ok(frame)
}

/// Infallible 17-lane control encoder.
pub fn encode_control(control: &ControlC) -> [u128; CONTROL_LANES] {
    use ControlC as K;
    let mut o = [0u128; CONTROL_LANES];
    match *control {
        K::Eval { expr } => o[1] = u(expr),
        K::Return { value } => { o[0] = 1; put_ref(&mut o, 1, value); }
        K::ScheduleNext { frame, next, env } => {
            o[0] = 2; o[1..1 + FRAME_LANES].copy_from_slice(&encode_frame(&frame));
            o[14] = u(next); put_env(&mut o, 15, env);
        }
        K::Invoke { function_name, args } => { o[0] = 3; o[1] = u(function_name); put_slice(&mut o, 2, args); }
        K::Enter { function, args } => {
            o[0] = 4;
            if let Some(function) = function { o[1] = 1; o[2] = u(function); }
            put_slice(&mut o, 3, args);
        }
        K::Bound { binding, return_type, body } => {
            o[0] = 5;
            if let BoundBinding::Ok { env } = binding { o[1] = 1; put_env(&mut o, 2, env); }
            o[4] = u(return_type.0); o[5] = u(body);
        }
        K::Cases { value, arms, saved_env } => {
            o[0] = 6; put_ref(&mut o, 1, value); put_slice(&mut o, 3, arms); put_env(&mut o, 5, saved_env);
        }
        K::CaseResult { matching, value, body, rest, saved_env } => {
            o[0] = 7;
            if let CaseMatching::Matched { env } = matching { o[1] = 1; put_env(&mut o, 2, env); }
            put_ref(&mut o, 4, value); o[6] = u(body); put_slice(&mut o, 7, rest); put_env(&mut o, 9, saved_env);
        }
        K::PrimResult { result } => {
            o[0] = 8;
            match result {
                PrimOutcome::Value { value } => put_ref(&mut o, 2, value),
                PrimOutcome::Error { code } => { o[1] = 1; o[2] = u128::from(code); }
            }
        }
        K::Error { code } => { o[0] = 9; o[1] = u128::from(code); }
        K::Materialize { type_id, ctor_id, fields } => {
            o[0] = 10; o[1] = u(type_id.0); o[2] = u(ctor_id.0); put_slice(&mut o, 3, fields);
        }
    }
    o
}

/// Structural control decode; every unused lane of every union arm must be zero.
pub fn decode_control(l: &[u128; CONTROL_LANES]) -> C6Result<ControlC> {
    use ControlC as K;
    const S: &str = "control";
    let (control, used) = match l[0] {
        0 => (K::Eval { expr: rd(S, l[1])? }, 2),
        1 => (K::Return { value: get_ref(S, l, 1)? }, 3),
        2 => {
            let fl: &[u128; FRAME_LANES] = (&l[1..1 + FRAME_LANES]).try_into()
                .map_err(|_| C6PhysError::Truncated { section: S, needed: CONTROL_LANES, actual: l.len() })?;
            (K::ScheduleNext { frame: decode_frame(fl)?, next: rd(S, l[14])?, env: get_env(S, l, 15)? }, CONTROL_LANES)
        }
        3 => (K::Invoke { function_name: rd(S, l[1])?, args: get_slice(S, l, 2)? }, 4),
        4 => {
            let function = match l[1] {
                0 => { zero(S, l, 2, 3)?; None }
                1 => Some(rd(S, l[2])?),
                tag => return Err(C6PhysError::BadTag { section: S, tag }),
            };
            (K::Enter { function, args: get_slice(S, l, 3)? }, 5)
        }
        5 => {
            let binding = match l[1] {
                0 => { zero(S, l, 2, 4)?; BoundBinding::Bad }
                1 => BoundBinding::Ok { env: get_env(S, l, 2)? },
                tag => return Err(C6PhysError::BadTag { section: S, tag }),
            };
            (K::Bound { binding, return_type: TypeIdC(rd(S, l[4])?), body: rd(S, l[5])? }, 6)
        }
        6 => (K::Cases { value: get_ref(S, l, 1)?, arms: get_slice(S, l, 3)?, saved_env: get_env(S, l, 5)? }, 7),
        7 => {
            let matching = match l[1] {
                0 => { zero(S, l, 2, 4)?; CaseMatching::NoMatch }
                1 => CaseMatching::Matched { env: get_env(S, l, 2)? },
                tag => return Err(C6PhysError::BadTag { section: S, tag }),
            };
            (K::CaseResult { matching, value: get_ref(S, l, 4)?, body: rd(S, l[6])?, rest: get_slice(S, l, 7)?,
                saved_env: get_env(S, l, 9)? }, 11)
        }
        8 => match l[1] {
            0 => (K::PrimResult { result: PrimOutcome::Value { value: get_ref(S, l, 2)? } }, 4),
            1 => (K::PrimResult { result: PrimOutcome::Error { code: rd_u8(S, l[2])? } }, 3),
            tag => return Err(C6PhysError::BadTag { section: S, tag }),
        },
        9 => (K::Error { code: rd_u8(S, l[1])? }, 2),
        10 => (K::Materialize { type_id: TypeIdC(rd(S, l[1])?), ctor_id: CtorIdC(rd(S, l[2])?),
            fields: get_slice(S, l, 3)? }, 5),
        tag => return Err(C6PhysError::BadTag { section: S, tag }),
    };
    zero(S, l, used, CONTROL_LANES)?;
    Ok(control)
}

pub fn decode_control_checked(lanes: &[u128; CONTROL_LANES], caps: &C6PhysCaps) -> C6Result<ControlC> {
    let control = decode_control(lanes)?;
    control_caps(&control, caps)?;
    Ok(control)
}

// ---------- root + frames section ----------

/// Root registers of the section. Counters are carried exactly.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct C6Root {
    pub control: ControlC,
    pub control_origin: usize,
    pub env_root: EnvRootC,
    pub stack_limit: usize,
    pub outcome: OutcomeC,
    pub initial_fuel: u128,
    pub remaining_work: u128,
    pub consumed_work: u128,
    pub helper_work: u128,
    pub helper_visits: u128,
    pub reference_cursor: u128,
    pub alloc: AllocCounters,
    pub active: ActiveLengths,
    pub seg_active: ActiveLengths,
    pub seg_alloc: AllocCounters,
    pub seg_consumed_work: u128,
}

impl C6Root {
    /// Projection of the owned image registers; no field is invented.
    pub fn from_image(image: &ImageSnapshot) -> Self {
        Self {
            control: image.control, control_origin: image.control_origin, env_root: image.env_root,
            stack_limit: image.stack_limit, outcome: image.outcome, initial_fuel: image.initial_fuel,
            remaining_work: image.remaining_work, consumed_work: image.consumed_work,
            helper_work: image.helper_work, helper_visits: image.helper_visits,
            reference_cursor: image.reference_cursor, alloc: image.alloc, active: image.active,
            seg_active: image.seg_base.active(), seg_alloc: image.seg_base.alloc(),
            seg_consumed_work: image.seg_base.consumed_work(),
        }
    }
}

fn root_caps(root: &C6Root, c: &C6PhysCaps) -> C6Result<()> {
    control_caps(&root.control, c)?;
    cap_index("root.control_origin", root.control_origin, c.max_expressions)?;
    cap_env("root.env_root", root.env_root, c)?;
    cap_count("root.stack_limit", root.stack_limit, c.max_frames)?;
    if let OutcomeC::Halted { value } = root.outcome { cap_ref("root.outcome", value, c)?; }
    for (section, needed, limit) in [
        ("root.alloc.data_nodes", root.alloc.data_nodes, c.max_data_nodes),
        ("root.alloc.environment_cells", root.alloc.environment_cells, c.max_env_cells),
        ("root.active.data", root.active.data, c.max_data_nodes),
        ("root.active.env", root.active.env, c.max_env_cells),
        ("root.active.value_slots", root.active.value_slots, c.max_value_slots),
        ("root.seg_active.data", root.seg_active.data, c.max_data_nodes),
        ("root.seg_active.env", root.seg_active.env, c.max_env_cells),
        ("root.seg_active.value_slots", root.seg_active.value_slots, c.max_value_slots),
        ("root.seg_alloc.data_nodes", root.seg_alloc.data_nodes, c.max_data_nodes),
        ("root.seg_alloc.environment_cells", root.seg_alloc.environment_cells, c.max_env_cells),
    ] {
        cap_count(section, needed, limit)?;
    }
    Ok(())
}

fn section_len(frame_count: usize) -> C6Result<usize> {
    frame_count.checked_mul(FRAME_LANES).and_then(|n| n.checked_add(IMAGE_SECTION_HEADER_LANES))
        .ok_or(C6PhysError::UsizeOverflow { section: "image_section", value: u(frame_count) })
}
fn put_alloc(out: &mut [u128], at: usize, a: AllocCounters) {
    out[at] = u(a.data_nodes); out[at + 1] = u(a.data_fields); out[at + 2] = u(a.environment_cells);
}
fn put_active(out: &mut [u128], at: usize, a: ActiveLengths) {
    out[at] = u(a.data); out[at + 1] = u(a.data_fields); out[at + 2] = u(a.env); out[at + 3] = u(a.value_slots);
}
fn get_alloc(s: &'static str, l: &[u128], at: usize) -> C6Result<AllocCounters> {
    Ok(AllocCounters { data_nodes: rd(s, l[at])?, data_fields: rd(s, l[at + 1])?, environment_cells: rd(s, l[at + 2])? })
}
fn get_active(s: &'static str, l: &[u128], at: usize) -> C6Result<ActiveLengths> {
    Ok(ActiveLengths { data: rd(s, l[at])?, data_fields: rd(s, l[at + 1])?, env: rd(s, l[at + 2])?,
        value_slots: rd(s, l[at + 3])? })
}
fn put_outcome(out: &mut [u128], at: usize, outcome: OutcomeC) {
    match outcome {
        OutcomeC::Running => {}
        OutcomeC::Halted { value } => { out[at] = 1; put_ref(out, at + 1, value); }
        OutcomeC::Rejected { code } => { out[at] = 2; out[at + 1] = u128::from(code); }
        OutcomeC::Exhausted { reason } => { out[at] = 3; out[at + 1] = exhaust_tag(reason); }
    }
}
fn get_outcome(s: &'static str, l: &[u128], at: usize) -> C6Result<OutcomeC> {
    match l[at] {
        0 => { zero(s, l, at + 1, at + 3)?; Ok(OutcomeC::Running) }
        1 => Ok(OutcomeC::Halted { value: get_ref(s, l, at + 1)? }),
        2 => { let code = rd_u8(s, l[at + 1])?; zero(s, l, at + 2, at + 3)?; Ok(OutcomeC::Rejected { code }) }
        3 => { let reason = exhaust_from(s, l[at + 1])?; zero(s, l, at + 2, at + 3)?; Ok(OutcomeC::Exhausted { reason }) }
        tag => Err(C6PhysError::BadTag { section: s, tag }),
    }
}

/// Root+frames encoder. Caps are checked (root, frame count, frames) before
/// any allocation; the output is the unique canonical section.
pub fn encode_root_and_frames(root: &C6Root, frames: &[FrameRowC], caps: &C6PhysCaps) -> C6Result<Vec<u128>> {
    root_caps(root, caps)?;
    cap_count("frames", frames.len(), caps.max_frames)?;
    for frame in frames { frame_caps(frame, caps)?; }
    let mut out = vec![0u128; section_len(frames.len())?];
    out[L_MAGIC] = C6_PHYS_MAGIC;
    out[L_VERSION] = C6_PHYS_VERSION;
    out[L_CONTROL..L_CONTROL + CONTROL_LANES].copy_from_slice(&encode_control(&root.control));
    out[L_CONTROL_ORIGIN] = u(root.control_origin);
    put_env(&mut out, L_ENV_ROOT, root.env_root);
    out[L_STACK_LIMIT] = u(root.stack_limit);
    put_outcome(&mut out, L_OUTCOME, root.outcome);
    out[L_INITIAL_FUEL] = root.initial_fuel;
    out[L_REMAINING_WORK] = root.remaining_work;
    out[L_CONSUMED_WORK] = root.consumed_work;
    out[L_HELPER_WORK] = root.helper_work;
    out[L_HELPER_VISITS] = root.helper_visits;
    out[L_REFERENCE_CURSOR] = root.reference_cursor;
    put_alloc(&mut out, L_ALLOC, root.alloc);
    put_active(&mut out, L_ACTIVE, root.active);
    put_active(&mut out, L_SEG_ACTIVE, root.seg_active);
    put_alloc(&mut out, L_SEG_ALLOC, root.seg_alloc);
    out[L_SEG_CONSUMED_WORK] = root.seg_consumed_work;
    out[L_FRAME_COUNT] = u(frames.len());
    for (chunk, frame) in out[IMAGE_SECTION_HEADER_LANES..].chunks_exact_mut(FRAME_LANES).zip(frames) {
        chunk.copy_from_slice(&encode_frame(frame));
    }
    Ok(out)
}

pub fn encode_image_section(image: &ImageSnapshot, caps: &C6PhysCaps) -> C6Result<Vec<u128>> {
    encode_root_and_frames(&C6Root::from_image(image), &image.frames, caps)
}

/// Decoder; see the module documentation for the exact first-error order.
pub fn decode_image_section(lanes: &[u128], caps: &C6PhysCaps) -> C6Result<(C6Root, Vec<FrameRowC>)> {
    const S: &str = "image_section";
    if lanes.len() < IMAGE_SECTION_HEADER_LANES {
        return Err(C6PhysError::Truncated { section: S, needed: IMAGE_SECTION_HEADER_LANES, actual: lanes.len() });
    }
    if lanes[L_MAGIC] != C6_PHYS_MAGIC { return Err(C6PhysError::BadMagic { actual: lanes[L_MAGIC] }); }
    if lanes[L_VERSION] != C6_PHYS_VERSION { return Err(C6PhysError::BadVersion { actual: lanes[L_VERSION] }); }
    let control_lanes: &[u128; CONTROL_LANES] = (&lanes[L_CONTROL..L_CONTROL + CONTROL_LANES]).try_into()
        .map_err(|_| C6PhysError::Truncated { section: S, needed: IMAGE_SECTION_HEADER_LANES, actual: lanes.len() })?;
    let root = C6Root {
        control: decode_control(control_lanes)?,
        control_origin: rd(S, lanes[L_CONTROL_ORIGIN])?,
        env_root: get_env(S, lanes, L_ENV_ROOT)?,
        stack_limit: rd(S, lanes[L_STACK_LIMIT])?,
        outcome: get_outcome(S, lanes, L_OUTCOME)?,
        initial_fuel: lanes[L_INITIAL_FUEL],
        remaining_work: lanes[L_REMAINING_WORK],
        consumed_work: lanes[L_CONSUMED_WORK],
        helper_work: lanes[L_HELPER_WORK],
        helper_visits: lanes[L_HELPER_VISITS],
        reference_cursor: lanes[L_REFERENCE_CURSOR],
        alloc: get_alloc(S, lanes, L_ALLOC)?,
        active: get_active(S, lanes, L_ACTIVE)?,
        seg_active: get_active(S, lanes, L_SEG_ACTIVE)?,
        seg_alloc: get_alloc(S, lanes, L_SEG_ALLOC)?,
        seg_consumed_work: lanes[L_SEG_CONSUMED_WORK],
    };
    let frame_count = rd(S, lanes[L_FRAME_COUNT])?;
    root_caps(&root, caps)?;
    cap_count("frames", frame_count, caps.max_frames)?;
    let needed = section_len(frame_count)?;
    if lanes.len() < needed { return Err(C6PhysError::Truncated { section: S, needed, actual: lanes.len() }); }
    if lanes.len() > needed { return Err(C6PhysError::TrailingLanes { section: S, extra: lanes.len() - needed }); }
    let mut frames = Vec::with_capacity(frame_count);
    for chunk in lanes[IMAGE_SECTION_HEADER_LANES..].chunks_exact(FRAME_LANES) {
        let record: &[u128; FRAME_LANES] = chunk.try_into()
            .map_err(|_| C6PhysError::Truncated { section: S, needed, actual: lanes.len() })?;
        frames.push(decode_frame_checked(record, caps)?);
    }
    Ok((root, frames))
}

// ---------- owner Push selection over owned evidence ----------

/// Owner order: P0 Running; P1 publication (helper and result absent: Error,
/// or Return at depth 0); P2 positive fuel; PS helper absent, result absent
/// (Push rows read no result); SI-07 tasks empty; ScheduleNext control; the
/// logical stack-limit row; for Accept, the separate physical frame check.
pub fn push_selection(host: &HostSnapshot) -> C6Result<PushSelection> {
    let img = &host.image;
    if img.outcome != OutcomeC::Running {
        return Err(C6PhysError::NotPushSelected { section: "push.p0_terminal" });
    }
    if img.helper.is_none() && img.result.is_none() {
        let publishes = match img.control {
            ControlC::Error { .. } => true,
            ControlC::Return { .. } => img.frames.is_empty(),
            _ => false,
        };
        if publishes { return Err(C6PhysError::NotPushSelected { section: "push.p1_publish" }); }
    }
    if img.remaining_work == 0 { return Err(C6PhysError::NotPushSelected { section: "push.p2_fuel" }); }
    if img.helper.is_some() { return Err(C6PhysError::NotPushSelected { section: "push.ps_helper" }); }
    if img.result.is_some() { return Err(C6PhysError::NotPushSelected { section: "push.ps_result" }); }
    if !img.tasks.is_empty() { return Err(C6PhysError::StepInvDomain { section: "push.si07_tasks" }); }
    if !matches!(img.control, ControlC::ScheduleNext { .. }) {
        return Err(C6PhysError::WrongControl { section: "push.control" });
    }
    let depth = img.frames.len();
    if depth >= img.stack_limit { return Ok(PushSelection::Refuse); }
    let frames = host.admission.bounds.frames;
    if depth >= frames { return Err(C6PhysError::PhysicalFrameCapacity { depth, frames }); }
    Ok(PushSelection::Accept)
}

/// Structural effect only: append the pending frame (Accept) or set Error 4
/// (Refuse). Counters are NOT charged here.
fn apply_push_structural(selection: PushSelection, root: &mut C6Root, frames: &mut Vec<FrameRowC>) -> C6Result<()> {
    let ControlC::ScheduleNext { frame, next, env } = root.control else {
        return Err(C6PhysError::WrongControl { section: "push.control" });
    };
    match selection {
        PushSelection::Accept => {
            frames.push(frame);
            root.env_root = env;
            root.control = ControlC::Eval { expr: next };
            root.control_origin = next;
        }
        PushSelection::Refuse => root.control = ControlC::Error { code: PUSH_REFUSE_ERROR_CODE },
    }
    Ok(())
}

fn project_push_structural(pre: &HostSnapshot, expected: PushSelection) -> C6Result<HostSnapshot> {
    let actual = push_selection(pre)?;
    if actual != expected { return Err(C6PhysError::UnexpectedPushSelection { expected, actual }); }
    let mut post = pre.clone();
    let mut root = C6Root::from_image(&post.image);
    apply_push_structural(actual, &mut root, &mut post.image.frames)?;
    post.image.control = root.control;
    post.image.env_root = root.env_root;
    post.image.control_origin = root.control_origin;
    Ok(post)
}

/// STRUCTURAL A135 projection of an owned snapshot: counters unchanged. Not
/// the owner POST; see `record_a135_push_step` for the charged, checked step.
pub fn project_a135_push_structural(pre: &HostSnapshot) -> C6Result<HostSnapshot> {
    project_push_structural(pre, PushSelection::Accept)
}

/// STRUCTURAL A140 projection: only control becomes Error 4; counters unchanged.
pub fn project_a140_refusal_structural(pre: &HostSnapshot) -> C6Result<HostSnapshot> {
    project_push_structural(pre, PushSelection::Refuse)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SectionProvenance {
    /// Projected from a caller-owned `HostSnapshot`; not admitted evidence.
    CallerSnapshot,
    /// Projected from a `HostSnapshot` this module captured from an AB-bound pair.
    OwnerCapture,
}

/// Root+frames lanes plus the Push verdict computed from the SAME owned
/// snapshot (including helper/result/tasks and admitted frame bounds, which
/// the lanes omit). Carries no program binding in either provenance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct C6BoundImageSection {
    lanes: Vec<u128>,
    provenance: SectionProvenance,
    push: C6Result<PushSelection>,
}

pub enum C6SectionCaptureError {
    Observation(HostViolation),
    Codec(C6PhysError),
}

impl C6BoundImageSection {
    pub fn from_snapshot(host: &HostSnapshot, caps: &C6PhysCaps) -> C6Result<Self> {
        Self::from_owned(host, caps, SectionProvenance::CallerSnapshot)
    }
    pub fn capture(admitted: &Admitted, image: &MachineImage, caps: &C6PhysCaps)
        -> Result<Self, C6SectionCaptureError> {
        let host = capture(admitted, image).map_err(C6SectionCaptureError::Observation)?;
        Self::from_owned(&host, caps, SectionProvenance::OwnerCapture).map_err(C6SectionCaptureError::Codec)
    }
    fn from_owned(host: &HostSnapshot, caps: &C6PhysCaps, provenance: SectionProvenance) -> C6Result<Self> {
        Ok(Self { lanes: encode_image_section(&host.image, caps)?, provenance, push: push_selection(host) })
    }
    pub fn lanes(&self) -> &[u128] { &self.lanes }
    pub fn provenance(&self) -> SectionProvenance { self.provenance }
    pub fn push_verdict(&self) -> C6Result<PushSelection> { self.push.clone() }
}

/// STRUCTURAL lane transform for the selected Push row; counters unchanged.
pub fn push_structural_lanes(section: &C6BoundImageSection, caps: &C6PhysCaps)
    -> C6Result<(PushSelection, Vec<u128>)> {
    let selection = section.push.clone()?;
    let (mut root, mut frames) = decode_image_section(&section.lanes, caps)?;
    if root.outcome != OutcomeC::Running { return Err(C6PhysError::NotPushSelected { section: "push.p0_terminal" }); }
    if root.remaining_work == 0 { return Err(C6PhysError::NotPushSelected { section: "push.p2_fuel" }); }
    let lane = if frames.len() < root.stack_limit { PushSelection::Accept } else { PushSelection::Refuse };
    if lane != selection { return Err(C6PhysError::UnexpectedPushSelection { expected: selection, actual: lane }); }
    apply_push_structural(selection, &mut root, &mut frames)?;
    Ok((selection, encode_root_and_frames(&root, &frames, caps)?))
}

// ---------- owner event charge ----------

fn ensure(ok: bool, section: &'static str) -> C6Result<()> {
    if ok { Ok(()) } else { Err(C6PhysError::EventMismatch { section }) }
}

/// A Push charge: the selection plus the event's work and reference-cursor
/// deltas. `from_event` validates a caller-supplied `StepEventC` against a
/// caller-supplied `HostSnapshot`. Both are publicly constructible, so this
/// value carries no owner provenance by itself. Owner provenance exists only
/// in `C6PushObserved`, which the strict recorders build around one actual
/// `record_step`. Applying a charge uses the validated event deltas.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PushOwnerCharge {
    selection: PushSelection,
    work: u8,
    reference_cursor: u8,
}

impl PushOwnerCharge {
    /// Consistency check of `event` against `before` and `selection` only; it
    /// does not establish that the owner emitted `event`.
    pub fn from_event(selection: PushSelection, event: &StepEventC, before: &HostSnapshot) -> C6Result<Self> {
        let img = &before.image;
        let depth = img.frames.len();
        let depth_after = match selection {
            PushSelection::Accept => depth.checked_add(1).ok_or(C6PhysError::CounterOverflow { section: "event.depth" })?,
            PushSelection::Refuse => depth,
        };
        ensure(event.rule_id == selection.rule(), "event.rule_id")?;
        ensure(event.event_kind == EventKind::LegacyComplete, "event.event_kind")?;
        ensure(event.site == Some(SiteTag::Push), "event.site")?;
        ensure(event.helper_kind.is_none(), "event.helper_kind")?;
        ensure(event.disposition == StepDisposition::Applied, "event.disposition")?;
        ensure(event.work_delta == 1 && event.helper_work_delta == 0 && event.helper_visits_delta == 0
            && event.reference_cursor_delta == 1, "event.charge")?;
        ensure(event.alloc_deltas == AllocDeltasC { data_nodes: 0, data_fields: 0, environment_cells: 0 }
            && event.value_slot_writes == 0, "event.alloc")?;
        ensure(event.logical_depth_before == depth && event.logical_depth_after == depth_after, "event.depth")?;
        ensure(event.task_cursor_before == img.tasks.len() && event.task_cursor_after == img.tasks.len(), "event.tasks")?;
        ensure(event.helper_phase_before.is_none() && event.helper_phase_after.is_none()
            && event.helper_cursor_before.is_none() && event.helper_cursor_after.is_none()
            && event.result_tag.is_none() && event.origin_reference_cursor.is_none(), "event.helper")?;
        ensure(event.exhausted_kind.is_none() && event.outcome_tag == OutcomeTagC::Running
            && event.outcome_payload == OutcomeC::Running && event.boundary == BoundaryKind::None, "event.outcome")?;
        ensure(event.source_expr_id == img.control_origin, "event.source_expr_id")?;
        ensure(event.step_id.ordinal == img.consumed_work && *event.step_id.run_id == img.run_id, "event.step_id")?;
        ensure(img.remaining_work >= u128::from(event.work_delta), "event.fuel")?;
        Ok(Self { selection, work: event.work_delta, reference_cursor: event.reference_cursor_delta })
    }
    pub fn selection(&self) -> PushSelection { self.selection }
    fn charged(&self, remaining: u128, consumed: u128, cursor: u128) -> C6Result<(u128, u128, u128)> {
        let work = u128::from(self.work);
        Ok((
            remaining.checked_sub(work).ok_or(C6PhysError::CounterOverflow { section: "charge.remaining_work" })?,
            consumed.checked_add(work).ok_or(C6PhysError::CounterOverflow { section: "charge.consumed_work" })?,
            cursor.checked_add(u128::from(self.reference_cursor))
                .ok_or(C6PhysError::CounterOverflow { section: "charge.reference_cursor" })?,
        ))
    }
    pub fn apply_to_root(&self, root: &C6Root) -> C6Result<C6Root> {
        let (remaining_work, consumed_work, reference_cursor) =
            self.charged(root.remaining_work, root.consumed_work, root.reference_cursor)?;
        Ok(C6Root { remaining_work, consumed_work, reference_cursor, ..*root })
    }
    pub fn apply_to_lanes(&self, structural: &[u128], caps: &C6PhysCaps) -> C6Result<Vec<u128>> {
        let (root, frames) = decode_image_section(structural, caps)?;
        encode_root_and_frames(&self.apply_to_root(&root)?, &frames, caps)
    }
    fn apply_to_image(&self, image: &mut ImageSnapshot) -> C6Result<()> {
        let (r, c, k) = self.charged(image.remaining_work, image.consumed_work, image.reference_cursor)?;
        image.remaining_work = r;
        image.consumed_work = c;
        image.reference_cursor = k;
        Ok(())
    }
}

// ---------- recorders around the single existing owner step ----------

pub enum C6RecordError {
    /// Pre-step capture failed; the owner was not called.
    Capture(HostViolation),
    /// Refused before the owner was called; the image is untouched.
    Precondition { error: C6PhysError, before: Box<HostSnapshot> },
    /// The unchanged owner/record failure.
    Owner(CaptureStepError),
    /// The owner stepped once, then verification failed.
    Mismatch { error: C6PhysError, observed: Box<ObservedStep> },
}

impl C6RecordError {
    /// Whether the image changed, from actual snapshots; `None` when unknown.
    pub fn image_advanced(&self) -> Option<bool> {
        match self {
            Self::Capture(_) | Self::Precondition { .. } => Some(false),
            Self::Owner(CaptureStepError::BeforeObservation(_)) => Some(false),
            Self::Owner(CaptureStepError::Step { before, after: Ok(after), .. }) => Some(before != after),
            Self::Owner(CaptureStepError::Step { after: Err(_), .. })
            | Self::Owner(CaptureStepError::AfterObservation { .. }) => None,
            Self::Mismatch { observed, .. } => Some(match observed.as_ref() {
                ObservedStep::Event(record) => record.before != record.after,
                ObservedStep::Terminal { before, after } => before != after,
            }),
        }
    }
    pub fn phys_error(&self) -> Option<&C6PhysError> {
        match self {
            Self::Precondition { error, .. } | Self::Mismatch { error, .. } => Some(error),
            Self::Capture(_) | Self::Owner(_) => None,
        }
    }
}

fn refused(error: C6PhysError, before: &HostSnapshot) -> C6RecordError {
    C6RecordError::Precondition { error, before: Box::new(before.clone()) }
}

/// Generic owner step with encoded before/after sections. Makes no rule claim.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct C6ObservedStepLanes {
    observed: ObservedStep,
    before_lanes: Vec<u128>,
    after_lanes: Vec<u128>,
}

impl C6ObservedStepLanes {
    pub fn observed(&self) -> &ObservedStep { &self.observed }
    pub fn before_lanes(&self) -> &[u128] { &self.before_lanes }
    pub fn after_lanes(&self) -> &[u128] { &self.after_lanes }
}

pub fn record_owner_step_lanes(admitted: &Admitted, image: &mut MachineImage, caps: &C6PhysCaps)
    -> Result<C6ObservedStepLanes, C6RecordError> {
    let before = capture(admitted, image).map_err(C6RecordError::Capture)?;
    let before_lanes = encode_image_section(&before.image, caps).map_err(|error| refused(error, &before))?;
    let observed = record_step(admitted, image).map_err(C6RecordError::Owner)?;
    let after = {
        let (b, a) = match &observed {
            ObservedStep::Event(record) => (&record.before, &record.after),
            ObservedStep::Terminal { before: b, after: a } => (b, a),
        };
        if *b != before { Err(C6PhysError::EventMismatch { section: "record.before" }) }
        else { encode_image_section(&a.image, caps) }
    };
    match after {
        Ok(after_lanes) => Ok(C6ObservedStepLanes { observed, before_lanes, after_lanes }),
        Err(error) => Err(C6RecordError::Mismatch { error, observed: Box::new(observed) }),
    }
}

/// One owner-checked Push step. Fields are private; only the strict recorders
/// construct it, after checking the actual event and the complete after state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct C6PushObserved {
    selection: PushSelection,
    record: CompleteStepRecord,
    before_lanes: Vec<u128>,
    structural_lanes: Vec<u128>,
    after_lanes: Vec<u128>,
    charge: PushOwnerCharge,
}

impl C6PushObserved {
    pub fn selection(&self) -> PushSelection { self.selection }
    pub fn record(&self) -> &CompleteStepRecord { &self.record }
    pub fn before_lanes(&self) -> &[u128] { &self.before_lanes }
    /// Selected structural effect with uncharged counters (diagnostic only).
    pub fn structural_lanes(&self) -> &[u128] { &self.structural_lanes }
    /// Encoding of the actual owner after-image.
    pub fn after_lanes(&self) -> &[u128] { &self.after_lanes }
    pub fn charge(&self) -> PushOwnerCharge { self.charge }
}

/// First differing field of the complete owned snapshots, if any.
fn first_host_difference(expected: &HostSnapshot, actual: &HostSnapshot) -> Option<&'static str> {
    let (x, y) = (&expected.image, &actual.image);
    let checks = [
        (expected.admission == actual.admission, "after.admission"),
        (x.run_id == y.run_id, "after.run_id"),
        (x.control == y.control, "after.control"),
        (x.control_origin == y.control_origin, "after.control_origin"),
        (x.env_root == y.env_root, "after.env_root"),
        (x.stack_limit == y.stack_limit, "after.stack_limit"),
        (x.outcome == y.outcome, "after.outcome"),
        (x.initial_fuel == y.initial_fuel, "after.initial_fuel"),
        (x.remaining_work == y.remaining_work, "after.remaining_work"),
        (x.consumed_work == y.consumed_work, "after.consumed_work"),
        (x.helper_work == y.helper_work, "after.helper_work"),
        (x.helper_visits == y.helper_visits, "after.helper_visits"),
        (x.reference_cursor == y.reference_cursor, "after.reference_cursor"),
        (x.alloc == y.alloc, "after.alloc"),
        (x.active == y.active, "after.active"),
        (x.helper == y.helper, "after.helper"),
        (x.result == y.result, "after.result"),
        (x.seg_base == y.seg_base, "after.seg_base"),
        (x.data == y.data, "after.data"),
        (x.data_fields == y.data_fields, "after.data_fields"),
        (x.env == y.env, "after.env"),
        (x.frames == y.frames, "after.frames"),
        (x.tasks == y.tasks, "after.tasks"),
        (x.value_slots == y.value_slots, "after.value_slots"),
    ];
    let first = checks.into_iter().find(|&(same, _)| !same).map(|(_, field)| field);
    first.or(if expected == actual { None } else { Some("after.snapshot") })
}

fn check_push_record(before: &HostSnapshot, selection: PushSelection, structural: &[u128],
    observed: &ObservedStep, caps: &C6PhysCaps) -> C6Result<(PushOwnerCharge, Vec<u128>)> {
    let ObservedStep::Event(record) = observed else {
        return Err(C6PhysError::EventMismatch { section: "event.terminal" });
    };
    ensure(record.before == *before, "record.before")?;
    let charge = PushOwnerCharge::from_event(selection, &record.event, &record.before)?;
    let mut expected = project_push_structural(&record.before, selection)?;
    charge.apply_to_image(&mut expected.image)?;
    if let Some(section) = first_host_difference(&expected, &record.after) {
        return Err(C6PhysError::AfterMismatch { section });
    }
    let after_lanes = encode_image_section(&record.after.image, caps)?;
    if charge.apply_to_lanes(structural, caps)? != after_lanes {
        return Err(C6PhysError::AfterMismatch { section: "push.after_lanes" });
    }
    Ok((charge, after_lanes))
}

fn record_push_step(admitted: &Admitted, image: &mut MachineImage, caps: &C6PhysCaps, expected: PushSelection)
    -> Result<C6PushObserved, C6RecordError> {
    let before = capture(admitted, image).map_err(C6RecordError::Capture)?;
    let section = C6BoundImageSection::from_owned(&before, caps, SectionProvenance::OwnerCapture)
        .map_err(|error| refused(error, &before))?;
    let selection = section.push_verdict().map_err(|error| refused(error, &before))?;
    if selection != expected {
        return Err(refused(C6PhysError::UnexpectedPushSelection { expected, actual: selection }, &before));
    }
    let (_, structural) = push_structural_lanes(&section, caps).map_err(|error| refused(error, &before))?;
    let observed = record_step(admitted, image).map_err(C6RecordError::Owner)?;
    let checked = check_push_record(&before, selection, &structural, &observed, caps);
    match (checked, observed) {
        (Ok((charge, after_lanes)), ObservedStep::Event(record)) => Ok(C6PushObserved {
            selection, record, before_lanes: section.lanes, structural_lanes: structural, after_lanes, charge,
        }),
        (Ok(_), observed) => Err(C6RecordError::Mismatch {
            error: C6PhysError::EventMismatch { section: "event.terminal" }, observed: Box::new(observed) }),
        (Err(error), observed) => Err(C6RecordError::Mismatch { error, observed: Box::new(observed) }),
    }
}

/// Strict A135 recorder: refuses (untouched) unless the owner would select
/// `KMachinePushAccept`; then steps once and checks event and full POST.
pub fn record_a135_push_step(admitted: &Admitted, image: &mut MachineImage, caps: &C6PhysCaps)
    -> Result<C6PushObserved, C6RecordError> {
    record_push_step(admitted, image, caps, PushSelection::Accept)
}

/// Strict A140 recorder for `KMachinePushRefuse` (Error 4, charged). The
/// later publication (`KAdminPublishError`) is a separate step, not A140.
pub fn record_a140_refusal_step(admitted: &Admitted, image: &mut MachineImage, caps: &C6PhysCaps)
    -> Result<C6PushObserved, C6RecordError> {
    record_push_step(admitted, image, caps, PushSelection::Refuse)
}

/// Owner-reachable fixtures shared with `c6_phys_program` tests.
#[cfg(test)]
pub(crate) mod fixtures {
    use super::*;
    use crate::sherlock::helper_machine::shared::i128_class::KIntC;
    use crate::sherlock::helper_machine::shared::owner_ext_source_program::{ExtDomain, ExtProgram};
    use crate::sherlock::helper_machine::shared::owner_ext_t3_uses::{
        ExtExpr, ExtFunction, ExtParamSrc, ExtValue,
    };
    use crate::sherlock::helper_machine::shared::owner_ext_type_builder::{
        CtorDeclSrc, CtorFieldSrc, TypeDeclSrc, TypeExprSrc,
    };
    use crate::sherlock::helper_machine::shared::owner_host::host_owner_engine;

    /// Initial fuel; kept <= layout value_slots and max_reference_work.
    pub(crate) const FUEL: u128 = 4096;

    pub(crate) fn caps() -> C6PhysCaps { C6PhysCaps::test_caps() }

    pub(crate) fn must<T, E>(result: Result<T, E>, message: &str) -> T {
        match result { Ok(value) => value, Err(_) => panic!("{message}") }
    }

    fn lit(n: u128) -> ExtExpr {
        ExtExpr::Literal { value: ExtValue::Core { value: CoreValue::UInt { n: KIntC::InRange(n) } } }
    }
    fn box_uint() -> TypeExprSrc {
        TypeExprSrc::Apply { name: "Box".into(), args: vec![TypeExprSrc::UInt] }
    }

    /// `type Box<A> = MkBox { item: A }`, `type Pair = MkPair { fst: UInt, snd: Box<UInt> }`,
    /// `fn mk(x: UInt) -> Pair { MkPair(x, MkBox(x)) }`, entry `let p = mk(7) in p`.
    /// The call gives genuine ArgsF and nominal ReturnF pushes. `KMachineBoundValid`
    /// schedules the ReturnF with the callee environment, so that push changes `env_root`.
    fn nominal_program() -> ExtProgram {
        let field = |name: &str, ty: TypeExprSrc| CtorFieldSrc { name: name.into(), ty };
        let var = |name: &str| ExtExpr::Var { name: name.into() };
        let pair = || TypeExprSrc::Named { name: "Pair".into() };
        let mk = ExtFunction {
            name: "mk".into(),
            parameters: vec![ExtParamSrc { name: "x".into(), ty: TypeExprSrc::UInt }],
            return_type: pair(),
            body: ExtExpr::Construct {
                ty: pair(), ctor: "MkPair".into(),
                fields: vec![var("x"), ExtExpr::Construct { ty: box_uint(), ctor: "MkBox".into(), fields: vec![var("x")] }],
            },
        };
        ExtProgram {
            types: vec![
                TypeDeclSrc { name: "Box".into(), parameters: vec!["A".into()], ctors: vec![CtorDeclSrc {
                    name: "MkBox".into(), fields: vec![field("item", TypeExprSrc::Param { name: "A".into() })] }] },
                TypeDeclSrc { name: "Pair".into(), parameters: Vec::new(), ctors: vec![CtorDeclSrc {
                    name: "MkPair".into(),
                    fields: vec![field("fst", TypeExprSrc::UInt), field("snd", box_uint())] }] },
            ],
            assets: Vec::new(), states: Vec::new(), inputs: Vec::new(), functions: vec![mk],
            entry: ExtExpr::Let {
                name: "p".into(),
                value: Box::new(ExtExpr::Call { name: "mk".into(), args: vec![lit(7)] }),
                body: Box::new(var("p")),
            },
        }
    }

    /// Real generic nominal admission and the owner's real `admit_run`.
    pub(crate) fn admitted_run(stack_limit: usize) -> (Admitted, MachineImage) {
        let bounds = BoundsC {
            code_rows: 128, function_rows: 16, code_child_slots: 128, parameter_slots: 32,
            pattern_rows: 32, match_plan_rows: 32, match_arm_rows: 32, type_rows: 16,
            constructor_rows: 16, constructor_field_slots: 32, max_arity: 8, constant_nodes: 128,
            constant_field_slots: 128, data_nodes: 128, data_field_slots: 128, environment_cells: 128,
            frames: 64, helper_tasks: 64, serialization_bytes: 1_000_000, max_reference_work: 1_000_000,
            max_helper_work: 1_000_000, step_cap: 10_000,
        };
        let layout = LayoutSchemaC {
            layout_version: ("c6-phys-test".into(), "1".into()), field_id: None, widths: None,
            scalar_layout_id: None, bounds,
            capacities: LayoutCapacitiesC { name_rows: 128, name_bytes: 4096, occurrence_path_slots: 4096,
                value_slots: 4096 },
            ext_capacities: Some(ExtCapacitiesC { plan_case_slots: 64, field_path_slots: 64,
                pattern_child_slots: 64, ..ExtCapacitiesC::all_zero() }),
            boundary_capacities_present: false,
        };
        let (admitted, _report) = must(admit_ext_program(nominal_program(), ExtDomain::GENERIC_ABSENT,
            &bounds, &layout, None), "generic nominal admission");
        let image = must(host_owner_engine().admit_run(&admitted, &RunIntC::Nat(FUEL),
            &RunIntC::Nat(stack_limit as u128), &FinanceInput::Absent), "admit run");
        (admitted, image)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::fixtures::must;

    fn caps() -> C6PhysCaps { fixtures::caps() }

    fn all_frames() -> [FrameRowC; 10] {
        [
            FrameRowC::LetF { binder: 3, body: 9, saved_env: Some(2), origin: 7 },
            FrameRowC::LeftF { op: BinaryOp::Lt, right: 11, saved_env: None, origin: 8 },
            FrameRowC::RightF { op: BinaryOp::Eq, left: RefC::Data(4), origin: 9 },
            FrameRowC::UnaryF { op: UnaryOp::Not, origin: 10 },
            FrameRowC::IfF { then_expr: 12, else_expr: 13, saved_env: Some(5), origin: 11 },
            FrameRowC::MatchF { arms: SliceC { start: 5, len: 2 }, saved_env: Some(1), origin: 12 },
            FrameRowC::ArgsF { function_name: 4, completed: SliceC { start: 2, len: 2 },
                rest: SliceC { start: 3, len: 1 }, saved_env: Some(6), origin: 13 },
            FrameRowC::ReturnF { return_type: TypeIdC(5), saved_env: Some(1), origin: 14 },
            FrameRowC::ScopeF { saved_env: Some(3), origin: 15 },
            FrameRowC::ConstructorFieldsF { type_id: TypeIdC(5), ctor_id: CtorIdC(6), index: 1,
                field_type: TypeIdC(4), checked: true, completed: SliceC { start: 2, len: 1 },
                rest: SliceC { start: 7, len: 1 }, saved_env: Some(2), origin: 16 },
        ]
    }

    fn all_controls() -> Vec<ControlC> {
        vec![
            ControlC::Eval { expr: 5 },
            ControlC::Return { value: RefC::Const(2) },
            ControlC::ScheduleNext { frame: all_frames()[9], next: 6, env: Some(4) },
            ControlC::Invoke { function_name: 2, args: SliceC { start: 1, len: 2 } },
            ControlC::Enter { function: None, args: SliceC { start: 3, len: 1 } },
            ControlC::Enter { function: Some(3), args: SliceC { start: 1, len: 2 } },
            ControlC::Bound { binding: BoundBinding::Bad, return_type: TypeIdC(4), body: 8 },
            ControlC::Bound { binding: BoundBinding::Ok { env: Some(1) }, return_type: TypeIdC(5), body: 8 },
            ControlC::Cases { value: RefC::Data(3), arms: SliceC { start: 2, len: 1 }, saved_env: Some(2) },
            ControlC::CaseResult { matching: CaseMatching::NoMatch, value: RefC::Const(1), body: 9,
                rest: SliceC { start: 4, len: 1 }, saved_env: Some(2) },
            ControlC::CaseResult { matching: CaseMatching::Matched { env: Some(3) }, value: RefC::Data(1),
                body: 9, rest: SliceC { start: 4, len: 1 }, saved_env: None },
            ControlC::PrimResult { result: PrimOutcome::Value { value: RefC::Data(2) } },
            ControlC::PrimResult { result: PrimOutcome::Error { code: 11 } },
            ControlC::Error { code: 4 },
            ControlC::Materialize { type_id: TypeIdC(5), ctor_id: CtorIdC(6), fields: SliceC { start: 0, len: 2 } },
        ]
    }

    /// Changing any single lane must refuse or decode to a different value.
    fn every_lane_significant<const N: usize, T: PartialEq>(lanes: [u128; N], value: &T,
        decode: fn(&[u128; N]) -> C6Result<T>) {
        for at in 0..N {
            let mut changed = lanes;
            changed[at] ^= 0x55;
            assert!(decode(&changed).map_or(true, |v| &v != value), "lane {at} was ignored");
        }
    }

    #[test]
    fn all_ten_frames_and_eleven_controls_are_exact_and_canonical() {
        // SYNTHETIC codec vectors over every variant and union arm; not admitted images.
        for frame in all_frames() {
            let lanes = encode_frame(&frame);
            assert_eq!(decode_frame(&lanes), Ok(frame));
            assert_eq!(decode_frame_checked(&lanes, &caps()), Ok(frame));
            every_lane_significant(lanes, &frame, decode_frame);
            let scheduled = ControlC::ScheduleNext { frame, next: 3, env: None };
            assert_eq!(decode_control_checked(&encode_control(&scheduled), &caps()), Ok(scheduled));
        }
        assert_eq!(encode_frame(&all_frames()[9]), [9, 5, 6, 1, 4, 1, 2, 1, 7, 1, 1, 2, 16]);
        for control in all_controls() {
            let lanes = encode_control(&control);
            assert_eq!(decode_control(&lanes), Ok(control));
            assert_eq!(decode_control_checked(&lanes, &caps()), Ok(control));
            every_lane_significant(lanes, &control, decode_control);
        }
    }

    #[test]
    fn ref_env_slice_union_and_bool_distinctions_are_exact() {
        let right = |left| FrameRowC::RightF { op: BinaryOp::Add, left, origin: 1 };
        assert_ne!(encode_frame(&right(RefC::Const(0))), encode_frame(&right(RefC::Data(0))));
        let scope = |saved_env| FrameRowC::ScopeF { saved_env, origin: 1 };
        assert_ne!(encode_frame(&scope(None)), encode_frame(&scope(Some(0))));
        let raw = FrameRowC::MatchF { arms: SliceC { start: 3, len: 0 }, saved_env: None, origin: 1 };
        let empty = FrameRowC::MatchF { arms: SliceC::empty(), saved_env: None, origin: 1 };
        assert_ne!(encode_frame(&raw), encode_frame(&empty));
        assert_eq!(decode_frame(&encode_frame(&raw)), Ok(raw), "raw empty slices are not normalized");
        let bound = |binding| ControlC::Bound { binding, return_type: TypeIdC(4), body: 2 };
        assert_ne!(encode_control(&bound(BoundBinding::Bad)), encode_control(&bound(BoundBinding::Ok { env: None })));
        let enter = |function| ControlC::Enter { function, args: SliceC::empty() };
        assert_ne!(encode_control(&enter(None)), encode_control(&enter(Some(0))));
        let mut padded = encode_control(&enter(None));
        padded[2] = 1;
        assert_eq!(decode_control(&padded), Err(C6PhysError::NonZeroPadding { section: "control", lane: 2, value: 1 }));
        let mut nomatch = encode_control(&all_controls()[9]);
        nomatch[3] = 2;
        assert_eq!(decode_control(&nomatch), Err(C6PhysError::NonZeroPadding { section: "control", lane: 3, value: 2 }));
        let mut checked = encode_frame(&all_frames()[9]);
        checked[5] = 2;
        assert_eq!(decode_frame(&checked), Err(C6PhysError::BadBool { section: "frame", value: 2 }));
    }

    #[test]
    fn checked_caps_use_the_right_table_and_strict_index_bounds() {
        let mut c = caps();
        c.max_data_nodes = 8;
        c.max_constants = 32;
        c.max_value_slots = 64;
        let right = |left| encode_frame(&FrameRowC::RightF { op: BinaryOp::Add, left, origin: 1 });
        assert!(decode_frame_checked(&right(RefC::Data(7)), &c).is_ok());
        assert_eq!(decode_frame_checked(&right(RefC::Data(8)), &c),
            Err(C6PhysError::CapacityExceeded { section: "frame.left", needed: 9, limit: 8 }));
        assert!(decode_frame_checked(&right(RefC::Const(8)), &c).is_ok(), "Const is not a data index");
        let let_f = |binder| encode_frame(&FrameRowC::LetF { binder, body: 0, saved_env: None, origin: 0 });
        assert!(decode_frame_checked(&let_f(c.max_names - 1), &c).is_ok());
        assert_eq!(decode_frame_checked(&let_f(c.max_names), &c),
            Err(C6PhysError::CapacityExceeded { section: "frame.binder", needed: c.max_names + 1, limit: c.max_names }));
        let args = |completed| encode_frame(&FrameRowC::ArgsF { function_name: 0, completed,
            rest: SliceC::empty(), saved_env: None, origin: 0 });
        assert_eq!(decode_frame_checked(&args(SliceC { start: usize::MAX, len: 1 }), &c),
            Err(C6PhysError::UsizeOverflow { section: "frame.completed", value: usize::MAX as u128 }));
        assert_eq!(decode_frame_checked(&args(SliceC { start: 0, len: 9 }), &c),
            Err(C6PhysError::CapacityExceeded { section: "frame.completed", needed: 9, limit: 8 }));
        assert_eq!(decode_frame_checked(&args(SliceC { start: 60, len: 5 }), &c),
            Err(C6PhysError::CapacityExceeded { section: "frame.completed", needed: 65, limit: 64 }));
        let enter = encode_control(&ControlC::Enter { function: Some(c.max_functions), args: SliceC::empty() });
        assert!(decode_control(&enter).is_ok(), "structural decode applies no caps");
        assert_eq!(decode_control_checked(&enter, &c), Err(C6PhysError::CapacityExceeded {
            section: "control.function", needed: c.max_functions + 1, limit: c.max_functions }));
        let budgets = C6CodecBudgets { type_expr_nodes: 7, template_bytes: 9 };
        let bounds = BoundsC { type_rows: 6, frames: 3, data_nodes: 5, ..BoundsC::default() };
        let from = C6PhysCaps::from_parts(&bounds, &LayoutCapacitiesC::default(), &ExtCapacitiesC::all_zero(), budgets);
        assert_eq!((from.max_decl_templates, from.max_types, from.max_frames, from.max_data_nodes), (2, 6, 3, 5));
        assert_eq!((from.max_type_expr_nodes, from.max_template_bytes), (7, 9));
    }

    #[test]
    fn builtin_type_ids_use_max_types_or_four_in_all_five_type_id_fields() {
        // from_parts with type_rows 0: builtin TypeIds 0..=3 stay representable in
        // all five TypeId fields; TypeId 4 refuses with needed 5, limit 4.
        let budgets = C6CodecBudgets { type_expr_nodes: 0, template_bytes: 0 };
        let at = |type_rows| C6PhysCaps::from_parts(
            &BoundsC { type_rows, code_rows: 1, constructor_rows: 1, max_arity: 1, ..BoundsC::default() },
            &LayoutCapacitiesC::default(), &ExtCapacitiesC::all_zero(), budgets);
        let exceeded = |section: &'static str, needed: usize, limit: usize|
            Err(C6PhysError::CapacityExceeded { section, needed, limit });
        let ret = |t| encode_frame(&FrameRowC::ReturnF { return_type: TypeIdC(t), saved_env: None, origin: 0 });
        let ctor = |t, f| encode_frame(&FrameRowC::ConstructorFieldsF { type_id: TypeIdC(t), ctor_id: CtorIdC(0),
            index: 0, field_type: TypeIdC(f), checked: false, completed: SliceC::empty(), rest: SliceC::empty(),
            saved_env: None, origin: 0 });
        let bound = |t| encode_control(&ControlC::Bound { binding: BoundBinding::Bad, return_type: TypeIdC(t), body: 0 });
        let mat = |t| encode_control(&ControlC::Materialize { type_id: TypeIdC(t), ctor_id: CtorIdC(0),
            fields: SliceC::empty() });
        let c = at(0);
        assert_eq!((c.max_types, c.max_decl_templates), (0, 0), "declared table caps are unchanged");
        assert!(decode_frame_checked(&ret(3), &c).is_ok());
        assert_eq!(decode_frame_checked(&ret(4), &c).map(|_| ()), exceeded("frame.return_type", 5, 4));
        assert!(decode_frame_checked(&ctor(3, 3), &c).is_ok());
        assert_eq!(decode_frame_checked(&ctor(4, 0), &c).map(|_| ()), exceeded("frame.type_id", 5, 4));
        assert_eq!(decode_frame_checked(&ctor(0, 4), &c).map(|_| ()), exceeded("frame.field_type", 5, 4));
        assert!(decode_control_checked(&bound(3), &c).is_ok());
        assert_eq!(decode_control_checked(&bound(4), &c).map(|_| ()), exceeded("control.return_type", 5, 4));
        assert!(decode_control_checked(&mat(3), &c).is_ok());
        assert_eq!(decode_control_checked(&mat(4), &c).map(|_| ()), exceeded("control.type_id", 5, 4));
        // A larger declared nominal cap is used as is.
        let c6 = at(6);
        assert_eq!(c6.max_types, 6);
        assert!(decode_frame_checked(&ret(5), &c6).is_ok());
        assert_eq!(decode_frame_checked(&ret(6), &c6).map(|_| ()), exceeded("frame.return_type", 7, 6));
        assert_eq!(decode_control_checked(&mat(6), &c6).map(|_| ()), exceeded("control.type_id", 7, 6));
    }

    fn synthetic_root(outcome: OutcomeC) -> C6Root {
        C6Root {
            control: ControlC::Eval { expr: 5 }, control_origin: 5, env_root: Some(0), stack_limit: 64,
            outcome, initial_fuel: u128::MAX, remaining_work: u128::MAX - 10, consumed_work: 10,
            helper_work: 3, helper_visits: 2, reference_cursor: 7,
            alloc: AllocCounters { data_nodes: 11, data_fields: 12, environment_cells: 13 },
            active: ActiveLengths { data: 21, data_fields: 22, env: 23, value_slots: 24 },
            seg_active: ActiveLengths { data: 1, data_fields: 2, env: 3, value_slots: 4 },
            seg_alloc: AllocCounters { data_nodes: 5, data_fields: 6, environment_cells: 7 },
            seg_consumed_work: 9,
        }
    }

    #[test]
    fn root_and_frames_inverse_offsets_and_order() {
        // SYNTHETIC section vectors with nonzero counters; not admitted images.
        let frames = vec![FrameRowC::ScopeF { saved_env: None, origin: 1 },
            FrameRowC::LetF { binder: 2, body: 3, saved_env: Some(0), origin: 4 }];
        let mut outcomes = vec![OutcomeC::Running, OutcomeC::Halted { value: RefC::Const(3) },
            OutcomeC::Halted { value: RefC::Data(0) }, OutcomeC::Rejected { code: 4 }];
        for reason in [ExhaustReason::Fuel, ExhaustReason::DataNodes, ExhaustReason::DataFields,
            ExhaustReason::EnvironmentCells, ExhaustReason::HelperTasks, ExhaustReason::HelperWork,
            ExhaustReason::SerializationBytes] {
            outcomes.push(OutcomeC::Exhausted { reason });
        }
        for outcome in outcomes {
            let root = synthetic_root(outcome);
            let lanes = encode_root_and_frames(&root, &frames, &caps()).unwrap();
            assert_eq!(lanes.len(), IMAGE_SECTION_HEADER_LANES + 2 * FRAME_LANES);
            let (root2, frames2) = decode_image_section(&lanes, &caps()).unwrap();
            assert_eq!(root2, root);
            assert_eq!(frames2, frames);
            assert_eq!(encode_root_and_frames(&root2, &frames2, &caps()).unwrap(), lanes);
        }
        // Declared lane offsets and bottom-to-top frame order.
        let root = synthetic_root(OutcomeC::Running);
        let lanes = encode_root_and_frames(&root, &frames, &caps()).unwrap();
        assert_eq!(lanes[L_MAGIC], C6_PHYS_MAGIC);
        assert_eq!(lanes[L_VERSION], C6_PHYS_VERSION);
        assert_eq!(lanes[L_CONTROL..L_CONTROL + CONTROL_LANES], encode_control(&root.control));
        assert_eq!(lanes[L_CONTROL_ORIGIN], 5);
        assert_eq!(lanes[L_ENV_ROOT..L_ENV_ROOT + 2], [1, 0]);
        assert_eq!(lanes[L_STACK_LIMIT], 64);
        assert_eq!(lanes[L_INITIAL_FUEL], u128::MAX);
        assert_eq!(lanes[L_REMAINING_WORK], u128::MAX - 10);
        assert_eq!(lanes[L_CONSUMED_WORK], 10);
        assert_eq!(lanes[L_HELPER_WORK], 3);
        assert_eq!(lanes[L_HELPER_VISITS], 2);
        assert_eq!(lanes[L_REFERENCE_CURSOR], 7);
        assert_eq!(lanes[L_ALLOC..L_ALLOC + 3], [11, 12, 13]);
        assert_eq!(lanes[L_ACTIVE..L_ACTIVE + 4], [21, 22, 23, 24]);
        assert_eq!(lanes[L_SEG_ACTIVE..L_SEG_ACTIVE + 4], [1, 2, 3, 4]);
        assert_eq!(lanes[L_SEG_ALLOC..L_SEG_ALLOC + 3], [5, 6, 7]);
        assert_eq!(lanes[L_SEG_CONSUMED_WORK], 9);
        assert_eq!(lanes[L_FRAME_COUNT], 2);
        assert_eq!(lanes[48..61], encode_frame(&frames[0]));
        assert_eq!(lanes[61..74], encode_frame(&frames[1]));
    }

    #[test]
    fn image_section_refusals_follow_the_frozen_first_error_order() {
        let frames = vec![FrameRowC::ScopeF { saved_env: None, origin: 1 }];
        let lanes = encode_root_and_frames(&synthetic_root(OutcomeC::Running), &frames, &caps()).unwrap();
        let decode = |l: &[u128]| decode_image_section(l, &caps());
        assert_eq!(decode(&lanes[..47]), Err(C6PhysError::Truncated { section: "image_section", needed: 48, actual: 47 }));
        let mut bad_magic = lanes.clone();
        bad_magic[L_MAGIC] = 0;
        bad_magic[L_CONTROL] = 99;
        assert_eq!(decode(&bad_magic), Err(C6PhysError::BadMagic { actual: 0 }));
        let mut bad_version = lanes.clone();
        bad_version[L_VERSION] = 2;
        assert_eq!(decode(&bad_version), Err(C6PhysError::BadVersion { actual: 2 }));
        let mut bad_control = lanes.clone();
        bad_control[L_CONTROL] = 99;
        assert_eq!(decode(&bad_control), Err(C6PhysError::BadTag { section: "control", tag: 99 }));
        // Structural root errors precede caps: an over-cap origin loses to a bad outcome tag.
        let mut structural_first = lanes.clone();
        structural_first[L_CONTROL_ORIGIN] = 300;
        structural_first[L_OUTCOME] = 9;
        assert_eq!(decode(&structural_first), Err(C6PhysError::BadTag { section: "image_section", tag: 9 }));
        let mut over_cap_origin = lanes.clone();
        over_cap_origin[L_CONTROL_ORIGIN] = 300;
        assert_eq!(decode(&over_cap_origin),
            Err(C6PhysError::CapacityExceeded { section: "root.control_origin", needed: 301, limit: 256 }));
        let mut running_pad = lanes.clone();
        running_pad[L_OUTCOME + 1] = 9;
        assert_eq!(decode(&running_pad),
            Err(C6PhysError::NonZeroPadding { section: "image_section", lane: L_OUTCOME + 1, value: 9 }));
        let rejected = encode_root_and_frames(&synthetic_root(OutcomeC::Rejected { code: 4 }), &frames, &caps()).unwrap();
        let mut rejected_pad = rejected.clone();
        rejected_pad[L_OUTCOME + 2] = 1;
        assert_eq!(decode(&rejected_pad),
            Err(C6PhysError::NonZeroPadding { section: "image_section", lane: L_OUTCOME + 2, value: 1 }));
        let mut exhaust = encode_root_and_frames(&synthetic_root(OutcomeC::Exhausted { reason: ExhaustReason::Fuel }),
            &frames, &caps()).unwrap();
        exhaust[L_OUTCOME + 1] = 7;
        assert_eq!(decode(&exhaust), Err(C6PhysError::BadTag { section: "image_section", tag: 7 }));
        let mut env_pad = lanes.clone();
        env_pad[L_ENV_ROOT] = 0;
        env_pad[L_ENV_ROOT + 1] = 1;
        assert_eq!(decode(&env_pad),
            Err(C6PhysError::NonZeroPadding { section: "image_section", lane: L_ENV_ROOT + 1, value: 1 }));
        // Caps precede length: an over-cap frame count with too few lanes is CapacityExceeded.
        let mut too_many = lanes.clone();
        too_many[L_FRAME_COUNT] = 65;
        assert_eq!(decode(&too_many), Err(C6PhysError::CapacityExceeded { section: "frames", needed: 65, limit: 64 }));
        let mut short = lanes.clone();
        short[L_FRAME_COUNT] = 2;
        assert_eq!(decode(&short), Err(C6PhysError::Truncated { section: "image_section", needed: 74, actual: 61 }));
        let mut trailing = lanes.clone();
        trailing.push(0);
        assert_eq!(decode(&trailing), Err(C6PhysError::TrailingLanes { section: "image_section", extra: 1 }));
        let mut huge = lanes.clone();
        huge[L_FRAME_COUNT] = u128::MAX;
        assert_eq!(decode(&huge), Err(C6PhysError::UsizeOverflow { section: "image_section", value: u128::MAX }));
        let mut bad_frame = lanes.clone();
        bad_frame[48] = 10;
        assert_eq!(decode(&bad_frame), Err(C6PhysError::BadTag { section: "frame", tag: 10 }));
        // Encoder applies the same root caps.
        let mut deep = synthetic_root(OutcomeC::Running);
        deep.stack_limit = 65;
        assert_eq!(encode_root_and_frames(&deep, &frames, &caps()),
            Err(C6PhysError::CapacityExceeded { section: "root.stack_limit", needed: 65, limit: 64 }));
        assert_eq!(encode_root_and_frames(&synthetic_root(OutcomeC::Running), &vec![frames[0]; 65], &caps()),
            Err(C6PhysError::CapacityExceeded { section: "frames", needed: 65, limit: 64 }));
    }

    fn caller_snapshot_refusals(pre: &HostSnapshot, caps: &C6PhysCaps) {
        let bind = |host: &HostSnapshot| C6BoundImageSection::from_snapshot(host, caps).unwrap();
        let section = bind(pre);
        assert_eq!(section.provenance(), SectionProvenance::CallerSnapshot);
        assert_eq!(push_structural_lanes(&section, caps).unwrap().0, PushSelection::Accept);
        let mut with_result = pre.clone();
        with_result.image.result = Some(HelperResultC::EnvMissing);
        assert_eq!(push_structural_lanes(&bind(&with_result), caps),
            Err(C6PhysError::NotPushSelected { section: "push.ps_result" }));
        let mut with_helper = pre.clone();
        with_helper.image.helper = Some(HelperControlC::Validate { common: HelperCommon { site: SiteTag::EvalLiteral,
            origin_reference_cursor: 0, parent_resume: None }, root: RefC::Const(0), status: TriStatus::Pending });
        assert_eq!(push_structural_lanes(&bind(&with_helper), caps),
            Err(C6PhysError::NotPushSelected { section: "push.ps_helper" }));
        let mut with_tasks = pre.clone();
        with_tasks.image.tasks = vec![HelperTaskC::ValidateTask { r: RefC::Const(0), visit: TaskVisitC::Unvisited, cursor: 0 }];
        assert_eq!(push_structural_lanes(&bind(&with_tasks), caps),
            Err(C6PhysError::StepInvDomain { section: "push.si07_tasks" }));
        let mut no_fuel = pre.clone();
        no_fuel.image.remaining_work = 0;
        assert_eq!(push_structural_lanes(&bind(&no_fuel), caps),
            Err(C6PhysError::NotPushSelected { section: "push.p2_fuel" }));
        let mut physical = pre.clone();
        physical.admission.bounds.frames = pre.image.frames.len();
        assert_eq!(push_structural_lanes(&bind(&physical), caps),
            Err(C6PhysError::PhysicalFrameCapacity { depth: pre.image.frames.len(), frames: pre.image.frames.len() }));
        // At stack_limit the selected row is the refusal: only control changes.
        let mut at_limit = pre.clone();
        at_limit.image.stack_limit = at_limit.image.frames.len();
        let (selection, lanes) = push_structural_lanes(&bind(&at_limit), caps).unwrap();
        assert_eq!(selection, PushSelection::Refuse);
        let (root, frames) = decode_image_section(&lanes, caps).unwrap();
        assert_eq!(root.control, ControlC::Error { code: PUSH_REFUSE_ERROR_CODE });
        assert_eq!(frames, at_limit.image.frames);
        let refused = project_a140_refusal_structural(&at_limit).unwrap();
        assert_eq!(refused.image.control, ControlC::Error { code: PUSH_REFUSE_ERROR_CODE });
        assert_eq!(refused.image.frames, at_limit.image.frames);
        assert_eq!(refused.image.env_root, at_limit.image.env_root);
        assert_eq!(refused.image.control_origin, at_limit.image.control_origin);
        assert_eq!(refused.image.remaining_work, at_limit.image.remaining_work, "structural: counters unchanged");
        assert_eq!(project_a135_push_structural(&at_limit),
            Err(C6PhysError::UnexpectedPushSelection { expected: PushSelection::Accept, actual: PushSelection::Refuse }));
    }

    #[test]
    fn admitted_nominal_run_strict_push_recorder_and_codec() {
        // OWNER-REACHABLE: real generic nominal admission, real fixed-owner steps.
        let caps = fixtures::caps();
        let (admitted, mut image) = fixtures::admitted_run(64);

        // Non-A135: the entry Eval is refused before any owner step; image untouched.
        let entry = must(capture(&admitted, &image), "capture entry");
        match record_a135_push_step(&admitted, &mut image, &caps) {
            Err(error) => {
                assert_eq!(error.image_advanced(), Some(false));
                assert!(matches!(error, C6RecordError::Precondition { error: C6PhysError::WrongControl { .. }, .. }));
            }
            Ok(_) => panic!("an Eval control is not a selected KMachinePushAccept"),
        }
        assert_eq!(must(capture(&admitted, &image), "capture after refusal"), entry);

        // Generic recorder: Eval(Let) selects KMachineEvalLet (legacy_group(EvalLet)).
        let generic = must(record_owner_step_lanes(&admitted, &mut image, &caps), "generic owner step");
        let ObservedStep::Event(first) = generic.observed() else { panic!("first step is an event") };
        assert_eq!(first.event.rule_id, RuleIdC::KMachineEvalLet);
        assert_eq!(generic.before_lanes(), &encode_image_section(&first.before.image, &caps).unwrap()[..]);
        assert_eq!(generic.after_lanes(), &encode_image_section(&first.after.image, &caps).unwrap()[..]);

        let mut pushes = 0usize;
        let mut nested_ctor_push = false;
        let mut args_origin: Option<usize> = None;
        let mut return_push = false;
        let mut env_changing_push = false;
        for _ in 0..512 {
            let pre = must(capture(&admitted, &image), "capture");
            if pre.image.outcome != OutcomeC::Running { break; }
            let ControlC::ScheduleNext { frame: pending, next, env } = pre.image.control else {
                match record_step(&admitted, &mut image) {
                    Ok(ObservedStep::Event(_)) => continue,
                    Ok(ObservedStep::Terminal { .. }) => break,
                    Err(_) => panic!("owner step"),
                }
            };
            if pre.image.remaining_work == 0 { break; }
            assert_eq!(push_selection(&pre), Ok(PushSelection::Accept));
            let observed = must(record_a135_push_step(&admitted, &mut image, &caps), "strict A135 recorder");
            pushes += 1;
            let record = observed.record();
            let (before, after) = (&record.before.image, &record.after.image);
            assert_eq!(record.before, pre);
            assert_eq!(observed.selection(), PushSelection::Accept);
            assert_eq!(record.event.rule_id, RuleIdC::KMachinePushAccept);
            // Exact append; older frames unchanged, bottom to top.
            assert_eq!(after.frames.len(), before.frames.len() + 1);
            assert_eq!(after.frames[..before.frames.len()], before.frames[..]);
            assert_eq!(after.frames.last(), Some(&pending));
            assert_eq!(after.env_root, env);
            assert_eq!(after.control, ControlC::Eval { expr: next });
            assert_eq!(after.control_origin, next);
            // Accounting from the actual owner event.
            assert_eq!(after.remaining_work + 1, before.remaining_work);
            assert_eq!(after.consumed_work, before.consumed_work + 1);
            assert_eq!(after.reference_cursor, before.reference_cursor + 1);
            assert_eq!(after.helper_work, before.helper_work);
            assert_eq!(after.helper_visits, before.helper_visits);
            assert_eq!((after.alloc, after.active, after.seg_base), (before.alloc, before.active, before.seg_base));
            assert_eq!((&after.data, &after.data_fields, &after.env), (&before.data, &before.data_fields, &before.env));
            assert_eq!((&after.value_slots, &after.tasks), (&before.value_slots, &before.tasks));
            assert_eq!((after.helper, after.result), (None, None));
            assert_eq!(record.after.admission, record.before.admission);
            // Structural and actual lanes differ exactly at the charged counters.
            let (structural, after_lanes) = (observed.structural_lanes(), observed.after_lanes());
            assert_eq!(structural.len(), after_lanes.len());
            for (at, (s, a)) in structural.iter().zip(after_lanes).enumerate() {
                if [L_REMAINING_WORK, L_CONSUMED_WORK, L_REFERENCE_CURSOR].contains(&at) { assert_ne!(s, a) }
                else { assert_eq!(s, a) }
            }
            assert_eq!(after_lanes, &encode_image_section(after, &caps).unwrap()[..]);
            // The A135 env write, observed where the owner actually changes env_root.
            if before.env_root != env {
                env_changing_push = true;
                assert_ne!(after.env_root, before.env_root);
                assert_ne!(observed.before_lanes()[L_ENV_ROOT..L_ENV_ROOT + 2], after_lanes[L_ENV_ROOT..L_ENV_ROOT + 2]);
            }
            let view = &record.before.admission.view;
            match pending {
                FrameRowC::ArgsF { function_name, completed, saved_env, origin, .. } => {
                    // KMachineEvalCallArguments: empty prefix, caller env kept.
                    assert_eq!(view.names[function_name], "mk");
                    assert_eq!(completed.len, 0);
                    assert_eq!((saved_env, env), (before.env_root, before.env_root));
                    assert!(matches!(before.frames.last(), Some(FrameRowC::LetF { .. })));
                    args_origin = Some(origin);
                }
                FrameRowC::ReturnF { return_type, saved_env, origin } => {
                    // KMachineBoundValid: nominal Pair return, callee env replaces caller env.
                    let decl = view.types[return_type.0].decl.expect("nominal return type");
                    assert_eq!(view.decl_templates[decl].name, "Pair");
                    assert_eq!(view.functions[0].return_type, return_type);
                    assert_eq!(saved_env, before.env_root);
                    assert_ne!(env, saved_env);
                    assert_eq!(Some(origin), args_origin);
                    assert!(matches!(before.frames.last(), Some(FrameRowC::LetF { .. })));
                    return_push = true;
                }
                _ => {}
            }
            let older_ctor = before.frames.iter().any(|f| matches!(f, FrameRowC::ConstructorFieldsF { .. }));
            if matches!(pending, FrameRowC::ConstructorFieldsF { .. }) && older_ctor {
                nested_ctor_push = true;
                let (root, frames) = decode_image_section(after_lanes, &caps).unwrap();
                assert_eq!(root, C6Root::from_image(after));
                assert_eq!(frames, after.frames);
                assert_eq!(encode_root_and_frames(&root, &frames, &caps).unwrap(), after_lanes);
                let owned = must(C6BoundImageSection::capture(&admitted, &image, &caps), "owned section");
                assert_eq!(owned.provenance(), SectionProvenance::OwnerCapture);
                assert_eq!(owned.lanes(), after_lanes);
                caller_snapshot_refusals(&record.before, &caps);
                let mut wrong_rule = record.event.clone();
                wrong_rule.rule_id = RuleIdC::KMachinePushRefuse;
                assert_eq!(PushOwnerCharge::from_event(PushSelection::Accept, &wrong_rule, &record.before),
                    Err(C6PhysError::EventMismatch { section: "event.rule_id" }));
                let mut wrong_charge = record.event.clone();
                wrong_charge.work_delta = 2;
                assert_eq!(PushOwnerCharge::from_event(PushSelection::Accept, &wrong_charge, &record.before),
                    Err(C6PhysError::EventMismatch { section: "event.charge" }));
                assert_eq!(observed.charge().apply_to_root(&C6Root { remaining_work: 0, ..root }),
                    Err(C6PhysError::CounterOverflow { section: "charge.remaining_work" }));
            }
        }
        assert!(pushes >= 5, "Let, ArgsF, ReturnF, MkPair and MkBox pushes are owner-reachable");
        assert!(args_origin.is_some(), "an ArgsF push for the call mk(7)");
        assert!(return_push, "a nominal ReturnF push scheduled by KMachineBoundValid");
        assert!(env_changing_push, "an Accept whose ScheduleNext.env differs from the before env_root");
        assert!(nested_ctor_push, "a ConstructorFieldsF push over an older ConstructorFieldsF frame");
    }

    #[test]
    fn admitted_genuine_a140_refusal_is_distinct_from_accept_and_publication() {
        // OWNER-REACHABLE: stack_limit 1 makes the call's ArgsF Push (depth 1) a refusal.
        let caps = fixtures::caps();
        let (admitted, mut image) = fixtures::admitted_run(1);
        let mut refused = None;
        for _ in 0..512 {
            let pre = must(capture(&admitted, &image), "capture");
            if pre.image.outcome != OutcomeC::Running { break; }
            if let ControlC::ScheduleNext { .. } = pre.image.control {
                if pre.image.frames.len() >= pre.image.stack_limit { refused = Some(pre); break; }
                must(record_a135_push_step(&admitted, &mut image, &caps), "accept below stack_limit");
                continue;
            }
            match record_step(&admitted, &mut image) {
                Ok(ObservedStep::Event(_)) => {}
                Ok(ObservedStep::Terminal { .. }) => break,
                Err(_) => panic!("owner step"),
            }
        }
        let pre = refused.expect("depth reaches stack_limit 1 at the call's ArgsF Push");
        // The Let push is accepted at depth 0; the call's argument frame is refused at depth 1.
        assert!(matches!(pre.image.control, ControlC::ScheduleNext { frame: FrameRowC::ArgsF { .. }, .. }));
        assert_eq!(push_selection(&pre), Ok(PushSelection::Refuse));
        match record_a135_push_step(&admitted, &mut image, &caps) {
            Err(C6RecordError::Precondition { error, .. }) => assert_eq!(error,
                C6PhysError::UnexpectedPushSelection { expected: PushSelection::Accept, actual: PushSelection::Refuse }),
            _ => panic!("A135 recorder must refuse a selected KMachinePushRefuse without stepping"),
        }
        assert_eq!(must(capture(&admitted, &image), "capture after refusal"), pre);
        let observed = must(record_a140_refusal_step(&admitted, &mut image, &caps), "strict A140 recorder");
        let record = observed.record();
        assert_eq!(record.event.rule_id, RuleIdC::KMachinePushRefuse);
        assert_eq!(record.event.disposition, StepDisposition::Applied);
        let (before, after) = (&record.before.image, &record.after.image);
        assert_eq!(after.control, ControlC::Error { code: PUSH_REFUSE_ERROR_CODE });
        assert_eq!(after.frames, before.frames);
        assert_eq!(after.env_root, before.env_root);
        assert_eq!(after.control_origin, before.control_origin);
        assert_eq!(after.outcome, OutcomeC::Running);
        assert_eq!(after.remaining_work + 1, before.remaining_work);
        assert_eq!(after.reference_cursor, before.reference_cursor + 1);
        // Publication 82 is the next, separate step; both recorders refuse it untouched.
        match record_a140_refusal_step(&admitted, &mut image, &caps) {
            Err(error) => {
                assert_eq!(error.image_advanced(), Some(false));
                assert!(matches!(error, C6RecordError::Precondition {
                    error: C6PhysError::NotPushSelected { section: "push.p1_publish" }, .. }));
            }
            Ok(_) => panic!("Error control publishes; it is not a Push"),
        }
        let publish = match record_step(&admitted, &mut image) {
            Ok(ObservedStep::Event(record)) => record,
            _ => panic!("publication event"),
        };
        assert_eq!(publish.event.rule_id, RuleIdC::KAdminPublishError);
        assert_eq!(publish.event.disposition, StepDisposition::Published);
        assert_eq!(publish.event.work_delta, 0);
        assert_eq!(publish.after.image.outcome, OutcomeC::Rejected { code: PUSH_REFUSE_ERROR_CODE });
    }
}
