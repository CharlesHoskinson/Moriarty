//! Assembled physical state section of sherlock-c6-phys/1 (C6P1), Chunk 3C.
//!
//! One outer section over the three existing codecs, used as-is: Chunk 1 root+frames
//! (`encode_root_and_frames`/`decode_image_section`), 3A data tables and 3B
//! helper/result/task tables. It admits nothing, constructs no `Admitted`,
//! `MachineImage`, `RunIdC`, `IdentityC` or `AdmissionSnapshot`, and steps nothing.
//!
//! Layout `sherlock-c6-phys/1` state section, version `C6_PHYS_VERSION`:
//! - 0 magic, 1 version, 2 `STATE_SECTION_TAG`, 3 child count (exactly 3);
//! - 4..=9 directory: (tag, exact lane length) for image (`IMAGE_CHILD_TAG`),
//!   data (`DATA_SECTION_TAG`), helper (`HELPER_SECTION_TAG`), in that fixed order;
//! - 10 ext-capacity presence (0 absent, 1 present; caps use the effective value);
//! - 11 reserved, zero;
//! - 12..=40 capacity descriptor: the 27 `C6PhysCaps` fields in declaration order,
//!   then `max_data_field_slots`, `max_helper_tasks`;
//! - then the three children contiguously; nothing between or after them.
//! The order is fixed, so reordered or duplicated children are tag errors.
//! Lengths are full usize; every offset sum is checked.
//!
//! `decode_state` first-error order:
//! 1. input longer than the caller lane limit: `LimitExceeded` (nothing read);
//! 2. fewer than 41 lanes: `Truncated`; 3. `BadMagic`; 4. `BadVersion`; 5. `BadTag`;
//! 6. lanes 3..=40 ascending: count (`BadSectionCount`); per child tag
//!    (`BadChildTag`), length (`UsizeOverflow`) and its minimum header
//!    (`ChildLengthRange`); ext lane (`BadBool`, then `ContextMismatch`);
//!    reserved lane (`NonZeroPadding`); descriptor lanes (`ContextMismatch`);
//! 7. offsets (`UsizeOverflow`), then `Truncated`, then `TrailingLanes`
//!    (no allocation before);
//! 8. image, data, helper children on exact slices, each with its own frozen
//!    first-error order, wrapped as `Child { child, offset, error }`.
//! `encode_state` order: length plan (`UsizeOverflow`), then the lane limit, then
//! the image, data and helper encoders (their caps); the output is the unique
//! canonical section. Both sides accept exactly the same canonical domain.
//!
//! Representation only: stored `alloc`/`active`/segment-base counters are carried
//! independently of vector lengths, and the derived depth/task_live are not stored.
//! Cross-field consistency (SI-05/SI-10, membership, StepInv) is future owner
//! admission (Chunk 4); no normalization happens here. `max_types` rules are the
//! existing codecs' rules, unchanged.
//!
//! The uncached witness comparison checks preservation under the existing
//! `construct_plan_for`/`evaluate_uncached` implementation only. Lanes,
//! codec equality and plan equality are not constraints, proofs or native evidence.

use super::c6_phys::{
    decode_image_section, encode_root_and_frames, C6CodecBudgets, C6PhysError, C6Root, C6_PHYS_MAGIC,
    C6_PHYS_VERSION, FRAME_LANES, IMAGE_SECTION_HEADER_LANES,
};
use super::c6_phys_data::{
    decode_data_tables, encode_data_tables, rd, rd_bool, u, zero, C6DataTables, C6StateCaps,
    DATA_HEADER_LANES, DATA_ROW_LANES, DATA_SECTION_TAG, ENV_ROW_LANES, REF_LANES,
};
use super::c6_phys_helper::{
    decode_helper_tables, encode_helper_tables, C6HelperTables, HELPER_FIXED_LANES, HELPER_SECTION_TAG,
    TASK_LANES,
};
use super::complete_observation::{HostSnapshot, ImageSnapshot};
use super::shared::host_implied_prefix::SegBase;
use super::shared::{BoundsC, ExtCapacitiesC, FrameRowC, LayoutCapacitiesC, LayoutSchemaC};
use super::witness_plan::{
    construct_plan_for, evaluate_uncached, LifetimeUnit, NodeId, Operation, WitnessEvaluation,
    WitnessFailure, WitnessPlan,
};

pub const STATE_SECTION_TAG: u128 = 0x5354_4154;
/// Directory-only tag: the Chunk 1 image section carries no tag lane itself.
pub const IMAGE_CHILD_TAG: u128 = 0x494D_4147;
pub const STATE_CHILD_COUNT: u128 = 3;
pub const L_SECTION_COUNT: usize = 3;
pub const L_DIRECTORY: usize = 4;
pub const L_EXT_PRESENT: usize = 10;
pub const L_RESERVED: usize = 11;
pub const L_CAPS: usize = 12;
pub const CAPS_DESCRIPTOR_LANES: usize = 29;
pub const STATE_HEADER_LANES: usize = L_CAPS + CAPS_DESCRIPTOR_LANES;
const STATE_S: &str = "state";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C6StateChild { Image, Data, Helper }

impl C6StateChild {
    pub const ALL: [Self; 3] = [Self::Image, Self::Data, Self::Helper];
    const fn index(self) -> usize { match self { Self::Image => 0, Self::Data => 1, Self::Helper => 2 } }
    pub const fn tag(self) -> u128 {
        match self { Self::Image => IMAGE_CHILD_TAG, Self::Data => DATA_SECTION_TAG, Self::Helper => HELPER_SECTION_TAG }
    }
    pub const fn minimum_lanes(self) -> usize {
        match self { Self::Image => IMAGE_SECTION_HEADER_LANES, Self::Data => DATA_HEADER_LANES, Self::Helper => HELPER_FIXED_LANES }
    }
}

/// Local wrapper; parent `C6PhysError` values are preserved unchanged.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum C6StateError {
    /// An outer-header error, reported against section "state".
    Outer(C6PhysError),
    LimitExceeded { needed: usize, limit: usize },
    BadSectionCount { actual: u128 },
    BadChildTag { child: C6StateChild, expected: u128, actual: u128 },
    ChildLengthRange { child: C6StateChild, declared: usize, minimum: usize },
    /// A header lane disagrees with the caller's supplied context.
    ContextMismatch { lane: usize, expected: u128, actual: u128 },
    /// A child codec error; `offset` is the child's first lane in the section.
    Child { child: C6StateChild, offset: usize, error: C6PhysError },
    /// Encoder self-check: the predicted child length differs from the child output.
    ChildLengthDrift { child: C6StateChild, predicted: usize, actual: usize },
    RetainedLayoutMismatch { field: &'static str },
}
pub type C6StateResult<T> = Result<T, C6StateError>;

fn outer(error: C6PhysError) -> C6StateError { C6StateError::Outer(error) }
fn child_error(child: C6StateChild, offset: usize, error: C6PhysError) -> C6StateError {
    C6StateError::Child { child, offset, error }
}

/// Caller-required bound on the whole section, checked before decoding or assembly.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct C6StateLimits { pub max_total_lanes: usize }

/// Caller-retained capacity context. It is an independent caller assertion and is
/// not bound to `host.admission`; `LayoutSnapshot` has no ext capacities.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct C6StateContext { caps: C6StateCaps, ext_present: bool, limits: C6StateLimits }

impl C6StateContext {
    /// From the ORIGINAL admitting layout the caller retained.
    pub fn from_layout(layout: &LayoutSchemaC, budgets: C6CodecBudgets, limits: C6StateLimits) -> Self {
        Self { caps: C6StateCaps::from_layout(layout, budgets), ext_present: layout.ext_capacities.is_some(), limits }
    }
    /// Same derivation as `from_layout`; `ext` keeps absent distinct from present zero.
    pub fn from_parts(bounds: &BoundsC, capacities: &LayoutCapacitiesC, ext: Option<ExtCapacitiesC>,
        budgets: C6CodecBudgets, limits: C6StateLimits) -> Self {
        let effective = ExtCapacitiesC::effective_for_extended_domain(ext);
        Self { caps: C6StateCaps::from_parts(bounds, capacities, &effective, budgets), ext_present: ext.is_some(), limits }
    }
    /// Explicit caller assertion over already-derived caps (tooling and synthetic vectors).
    pub fn from_caps_assertion(caps: C6StateCaps, ext_present: bool, limits: C6StateLimits) -> Self {
        Self { caps, ext_present, limits }
    }
    pub fn caps(&self) -> &C6StateCaps { &self.caps }
    pub fn ext_present(&self) -> bool { self.ext_present }
    pub fn limits(&self) -> C6StateLimits { self.limits }
    pub fn descriptor(&self) -> [u128; CAPS_DESCRIPTOR_LANES] {
        let (c, p) = (&self.caps, &self.caps.phys);
        [p.max_frames, p.max_expressions, p.max_names, p.max_arms, p.max_children, p.max_value_slots,
            p.max_constants, p.max_data_nodes, p.max_env_cells, p.max_functions, p.max_arity, p.max_types,
            p.max_occurrences, p.max_occurrence_paths, p.max_parameters, p.max_patterns,
            p.max_pattern_children, p.max_constructors, p.max_ctor_fields, p.max_decl_templates,
            p.max_plan_nodes, p.max_plan_cases, p.max_field_paths, p.max_constant_fields, p.max_name_bytes,
            p.max_type_expr_nodes, p.max_template_bytes, c.max_data_field_slots, c.max_helper_tasks].map(u)
    }
    fn admit_total(&self, needed: usize) -> C6StateResult<()> {
        let limit = self.limits.max_total_lanes;
        if needed > limit { Err(C6StateError::LimitExceeded { needed, limit }) } else { Ok(()) }
    }
}

/// Checks that the caller's retained layout equals every layout field the host
/// snapshot carries. Ext capacities have no host counterpart.
pub fn retained_layout_agrees_with_host(layout: &LayoutSchemaC, host: &HostSnapshot) -> C6StateResult<()> {
    let h = &host.admission.layout;
    let widths = match (&h.widths, &layout.widths) {
        (None, None) => true,
        (Some(s), Some(w)) => s.index_widths.as_slice() == w.index_width_entries()
            && s.counter_widths.as_slice() == w.counter_width_entries()
            && s.record_widths.as_slice() == w.record_width_entries(),
        _ => false,
    };
    for (same, field) in [
        (h.layout_version == layout.layout_version, "layout_version"),
        (h.field_id == layout.field_id, "field_id"),
        (widths, "widths"),
        (h.scalar_layout_id == layout.scalar_layout_id, "scalar_layout_id"),
        (h.bounds == layout.bounds && host.admission.bounds == layout.bounds, "bounds"),
        (h.capacities == layout.capacities, "capacities"),
        (h.boundary_capacities_present == layout.boundary_capacities_present, "boundary_capacities_present"),
    ] {
        if !same { return Err(C6StateError::RetainedLayoutMismatch { field }); }
    }
    Ok(())
}

/// Every encoded `ImageSnapshot` field: root registers (including the segment
/// base), raw frames bottom-to-top, and all seven tables. `run_id` is external.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct C6OwnedState {
    pub root: C6Root,
    pub frames: Vec<FrameRowC>,
    pub data: C6DataTables,
    pub helper: C6HelperTables,
}

impl C6OwnedState {
    pub fn from_image(image: &ImageSnapshot) -> Self {
        Self { root: C6Root::from_image(image), frames: image.frames.clone(),
            data: C6DataTables::from_image(image), helper: C6HelperTables::from_image(image) }
    }
    /// Diagnostic: overwrite every encoded field of a caller-owned image (normally
    /// a clone); `run_id` stays external. Exhaustive destructuring makes a new
    /// image or root field a compile error until it is accounted for.
    pub fn replace_state_of(self, image: &mut ImageSnapshot) {
        let C6OwnedState { root, frames, data, helper } = self;
        let C6Root { control, control_origin, env_root, stack_limit, outcome, initial_fuel, remaining_work,
            consumed_work, helper_work, helper_visits, reference_cursor, alloc, active, seg_active, seg_alloc,
            seg_consumed_work } = root;
        let C6DataTables { data, data_fields, env, value_slots } = data;
        let C6HelperTables { helper, result, tasks } = helper;
        let ImageSnapshot { run_id: _, control: i_control, control_origin: i_origin, env_root: i_env_root,
            stack_limit: i_stack, outcome: i_outcome, initial_fuel: i_fuel, remaining_work: i_remaining,
            consumed_work: i_consumed, helper_work: i_hwork, helper_visits: i_visits,
            reference_cursor: i_cursor, alloc: i_alloc, active: i_active, helper: i_helper, result: i_result,
            seg_base: i_seg, data: i_data, data_fields: i_fields, env: i_env, frames: i_frames,
            tasks: i_tasks, value_slots: i_slots } = image;
        *i_control = control; *i_origin = control_origin; *i_env_root = env_root; *i_stack = stack_limit;
        *i_outcome = outcome; *i_fuel = initial_fuel; *i_remaining = remaining_work;
        *i_consumed = consumed_work; *i_hwork = helper_work; *i_visits = helper_visits;
        *i_cursor = reference_cursor; *i_alloc = alloc; *i_active = active;
        *i_seg = SegBase::from_parts(seg_active, seg_alloc, seg_consumed_work);
        *i_helper = helper; *i_result = result; *i_tasks = tasks;
        *i_data = data; *i_fields = data_fields; *i_env = env; *i_slots = value_slots; *i_frames = frames;
    }
}

/// Exact child lengths. Each child decoder checks its own exact length again.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct C6StateDirectory { pub image_lanes: usize, pub data_lanes: usize, pub helper_lanes: usize }

impl C6StateDirectory {
    pub fn len_of(&self, child: C6StateChild) -> usize {
        match child { C6StateChild::Image => self.image_lanes, C6StateChild::Data => self.data_lanes,
            C6StateChild::Helper => self.helper_lanes }
    }
    /// Image, data and helper start offsets, then the total length; all checked.
    pub fn offsets(&self) -> C6StateResult<[usize; 4]> {
        let mut o = [STATE_HEADER_LANES; 4];
        for child in C6StateChild::ALL {
            let (i, len) = (child.index(), self.len_of(child));
            o[i + 1] = o[i].checked_add(len)
                .ok_or(outer(C6PhysError::UsizeOverflow { section: STATE_S, value: u(len) }))?;
        }
        Ok(o)
    }
}

/// Child lengths from counts and the public row widths (no child-private offsets).
pub fn plan_state_lengths(state: &C6OwnedState) -> C6StateResult<C6StateDirectory> {
    fn rows(acc: usize, count: usize, width: usize) -> C6StateResult<usize> {
        count.checked_mul(width).and_then(|n| acc.checked_add(n))
            .ok_or(outer(C6PhysError::UsizeOverflow { section: STATE_S, value: u(count) }))
    }
    let d = &state.data;
    let data = rows(DATA_HEADER_LANES, d.data.len(), DATA_ROW_LANES)?;
    let data = rows(data, d.data_fields.len(), REF_LANES)?;
    let data = rows(data, d.env.len(), ENV_ROW_LANES)?;
    Ok(C6StateDirectory {
        image_lanes: rows(IMAGE_SECTION_HEADER_LANES, state.frames.len(), FRAME_LANES)?,
        data_lanes: rows(data, d.value_slots.len(), REF_LANES)?,
        helper_lanes: rows(HELPER_FIXED_LANES, state.helper.tasks.len(), TASK_LANES)?,
    })
}

fn state_header(dir: &C6StateDirectory, ctx: &C6StateContext) -> [u128; STATE_HEADER_LANES] {
    let mut h = [0u128; STATE_HEADER_LANES];
    h[..L_DIRECTORY].copy_from_slice(&[C6_PHYS_MAGIC, C6_PHYS_VERSION, STATE_SECTION_TAG, STATE_CHILD_COUNT]);
    for child in C6StateChild::ALL {
        let at = L_DIRECTORY + 2 * child.index();
        h[at] = child.tag();
        h[at + 1] = u(dir.len_of(child));
    }
    h[L_EXT_PRESENT] = u128::from(ctx.ext_present);
    h[L_CAPS..].copy_from_slice(&ctx.descriptor());
    h
}

/// Canonical encoder: length plan, lane limit, then the three existing encoders.
pub fn encode_state(state: &C6OwnedState, ctx: &C6StateContext) -> C6StateResult<Vec<u128>> {
    use C6StateChild::*;
    let dir = plan_state_lengths(state)?;
    let o = dir.offsets()?;
    ctx.admit_total(o[3])?;
    let image = encode_root_and_frames(&state.root, &state.frames, &ctx.caps.phys)
        .map_err(|e| child_error(Image, o[0], e))?;
    let data = encode_data_tables(&state.data, &ctx.caps).map_err(|e| child_error(Data, o[1], e))?;
    let helper = encode_helper_tables(&state.helper, &ctx.caps).map_err(|e| child_error(Helper, o[2], e))?;
    let mut out = Vec::with_capacity(o[3]);
    out.extend(state_header(&dir, ctx));
    for (child, lanes) in [(Image, image), (Data, data), (Helper, helper)] {
        let predicted = dir.len_of(child);
        if lanes.len() != predicted {
            return Err(C6StateError::ChildLengthDrift { child, predicted, actual: lanes.len() });
        }
        out.extend(lanes);
    }
    Ok(out)
}

/// Decoder; see the module documentation for the exact first-error order.
pub fn decode_state(lanes: &[u128], ctx: &C6StateContext) -> C6StateResult<C6OwnedState> {
    use C6StateChild::*;
    ctx.admit_total(lanes.len())?;
    if lanes.len() < STATE_HEADER_LANES {
        return Err(outer(C6PhysError::Truncated { section: STATE_S, needed: STATE_HEADER_LANES, actual: lanes.len() }));
    }
    if lanes[0] != C6_PHYS_MAGIC { return Err(outer(C6PhysError::BadMagic { actual: lanes[0] })); }
    if lanes[1] != C6_PHYS_VERSION { return Err(outer(C6PhysError::BadVersion { actual: lanes[1] })); }
    if lanes[2] != STATE_SECTION_TAG { return Err(outer(C6PhysError::BadTag { section: STATE_S, tag: lanes[2] })); }
    if lanes[L_SECTION_COUNT] != STATE_CHILD_COUNT {
        return Err(C6StateError::BadSectionCount { actual: lanes[L_SECTION_COUNT] });
    }
    let mut lengths = [0usize; 3];
    for child in C6StateChild::ALL {
        let at = L_DIRECTORY + 2 * child.index();
        if lanes[at] != child.tag() {
            return Err(C6StateError::BadChildTag { child, expected: child.tag(), actual: lanes[at] });
        }
        let declared = rd(STATE_S, lanes[at + 1]).map_err(outer)?;
        if declared < child.minimum_lanes() {
            return Err(C6StateError::ChildLengthRange { child, declared, minimum: child.minimum_lanes() });
        }
        lengths[child.index()] = declared;
    }
    if rd_bool(STATE_S, lanes[L_EXT_PRESENT]).map_err(outer)? != ctx.ext_present {
        return Err(C6StateError::ContextMismatch { lane: L_EXT_PRESENT,
            expected: u128::from(ctx.ext_present), actual: lanes[L_EXT_PRESENT] });
    }
    zero(STATE_S, lanes, L_RESERVED, L_RESERVED + 1).map_err(outer)?;
    for (k, expected) in ctx.descriptor().into_iter().enumerate() {
        let lane = L_CAPS + k;
        if lanes[lane] != expected { return Err(C6StateError::ContextMismatch { lane, expected, actual: lanes[lane] }); }
    }
    let dir = C6StateDirectory { image_lanes: lengths[0], data_lanes: lengths[1], helper_lanes: lengths[2] };
    let o = dir.offsets()?;
    if lanes.len() < o[3] {
        return Err(outer(C6PhysError::Truncated { section: STATE_S, needed: o[3], actual: lanes.len() }));
    }
    if lanes.len() > o[3] {
        return Err(outer(C6PhysError::TrailingLanes { section: STATE_S, extra: lanes.len() - o[3] }));
    }
    let (root, frames) = decode_image_section(&lanes[o[0]..o[1]], &ctx.caps.phys)
        .map_err(|e| child_error(Image, o[0], e))?;
    let data = decode_data_tables(&lanes[o[1]..o[2]], &ctx.caps).map_err(|e| child_error(Data, o[1], e))?;
    let helper = decode_helper_tables(&lanes[o[2]..o[3]], &ctx.caps).map_err(|e| child_error(Helper, o[2], e))?;
    Ok(C6OwnedState { root, frames, data, helper })
}

pub fn state_lanes_from_host(host: &HostSnapshot, ctx: &C6StateContext) -> C6StateResult<Vec<u128>> {
    encode_state(&C6OwnedState::from_image(&host.image), ctx)
}

pub fn roundtrip_state_owned(state: &C6OwnedState, ctx: &C6StateContext) -> C6StateResult<C6OwnedState> {
    decode_state(&encode_state(state, ctx)?, ctx)
}

/// Conditional diagnostic inverse: a clone of the caller's host with every encoded
/// field replaced. `admission` and `image.run_id` are retained from the clone. This
/// is not import, admission or an owner-authorized transition.
pub fn host_with_decoded_state(host: &HostSnapshot, lanes: &[u128], ctx: &C6StateContext)
    -> C6StateResult<HostSnapshot> {
    let state = decode_state(lanes, ctx)?;
    let mut out = host.clone();
    state.replace_state_of(&mut out.image);
    Ok(out)
}

/// First differing field of two owned host snapshots, if any.
fn first_host_difference(a: &HostSnapshot, b: &HostSnapshot) -> Option<&'static str> {
    let (x, y) = (&a.image, &b.image);
    [
        (a.admission == b.admission, "admission"), (x.run_id == y.run_id, "image.run_id"),
        (x.control == y.control, "image.control"), (x.control_origin == y.control_origin, "image.control_origin"),
        (x.env_root == y.env_root, "image.env_root"), (x.stack_limit == y.stack_limit, "image.stack_limit"),
        (x.outcome == y.outcome, "image.outcome"), (x.initial_fuel == y.initial_fuel, "image.initial_fuel"),
        (x.remaining_work == y.remaining_work, "image.remaining_work"),
        (x.consumed_work == y.consumed_work, "image.consumed_work"), (x.helper_work == y.helper_work, "image.helper_work"),
        (x.helper_visits == y.helper_visits, "image.helper_visits"),
        (x.reference_cursor == y.reference_cursor, "image.reference_cursor"), (x.alloc == y.alloc, "image.alloc"),
        (x.active == y.active, "image.active"), (x.helper == y.helper, "image.helper"),
        (x.result == y.result, "image.result"), (x.seg_base == y.seg_base, "image.seg_base"),
        (x.data == y.data, "image.data"), (x.data_fields == y.data_fields, "image.data_fields"),
        (x.env == y.env, "image.env"), (x.frames == y.frames, "image.frames"), (x.tasks == y.tasks, "image.tasks"),
        (x.value_slots == y.value_slots, "image.value_slots"),
    ].into_iter().find(|&(same, _)| !same).map(|(_, field)| field)
        .or_else(|| (a != b).then_some("snapshot"))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C6WitnessSide { Decoded, Cross }

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum C6WitnessError {
    State(C6StateError),
    HostMismatch { field: &'static str },
    OriginalPlan(WitnessFailure),
    DecodedPlan(WitnessFailure),
    PlanMismatch { field: &'static str, index: Option<usize> },
    LiveOutMismatch { node: NodeId, field: &'static str },
    OriginalEvaluation(WitnessFailure),
    DecodedEvaluation(WitnessFailure),
    /// The ORIGINAL plan, with all its immutable bindings, evaluated over the rebuilt host.
    CrossEvaluation(WitnessFailure),
    EvaluationMismatch { against: C6WitnessSide, field: &'static str },
}

/// Preservation under the existing uncached implementation only.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct C6WitnessPreservation {
    pub lifetime: LifetimeUnit,
    pub state_lanes: Vec<u128>,
    pub plan: WitnessPlan,
    pub evaluation: WitnessEvaluation,
    /// Named snapshot fields of the output node; live_outs = these plus the output.
    pub live_out_fields: usize,
}

fn first_plan_difference(a: &WitnessPlan, b: &WitnessPlan) -> Option<(&'static str, Option<usize>)> {
    fn first<T: PartialEq>(x: &[T], y: &[T]) -> Option<usize> {
        (0..x.len().max(y.len())).find(|&i| x.get(i) != y.get(i))
    }
    if a.format != b.format { return Some(("format", None)); }
    if a.lifetime != b.lifetime { return Some(("lifetime", None)); }
    for (field, index) in [("inputs", first(&a.inputs, &b.inputs)), ("nodes", first(&a.nodes, &b.nodes)),
        ("stable_topological_order", first(&a.stable_topological_order, &b.stable_topological_order)),
        ("definitions", first(&a.definitions, &b.definitions)), ("live_outs", first(&a.live_outs, &b.live_outs))] {
        if index.is_some() { return Some((field, index)); }
    }
    if a.output != b.output { return Some(("output", None)); }
    (a != b).then_some(("plan", None))
}

/// Live-out structure of the actual plan: output fields, flags, lifetime, producer, last use.
fn check_live_outs(plan: &WitnessPlan, lifetime: LifetimeUnit) -> Result<usize, C6WitnessError> {
    let bad = |node, field| C6WitnessError::LiveOutMismatch { node, field };
    let Some(Operation::Snapshot { fields }) = plan.nodes.get(plan.output.0).map(|n| &n.operation) else {
        return Err(bad(plan.output, "output"));
    };
    let mut expected: Vec<NodeId> = fields.iter().map(|f| f.dependency.node).collect();
    expected.push(plan.output);
    if plan.live_outs != expected { return Err(bad(plan.output, "live_outs")); }
    let publication = plan.stable_topological_order.len();
    for &id in &plan.live_outs {
        let d = plan.definitions.get(id.0).ok_or(bad(id, "definition"))?;
        if d.value != id || d.producer != id { return Err(bad(id, "definition")); }
        if !d.live_out { return Err(bad(id, "live_out")); }
        if d.lifetime != lifetime { return Err(bad(id, "lifetime")); }
        if d.last_use_order != publication { return Err(bad(id, "last_use_order")); }
    }
    let distinct: std::collections::BTreeSet<NodeId> = plan.live_outs.iter().copied().collect();
    if plan.definitions.iter().filter(|d| d.live_out).count() != distinct.len() {
        return Err(bad(plan.output, "live_out_count"));
    }
    Ok(fields.len())
}

fn first_evaluation_difference(a: &WitnessEvaluation, b: &WitnessEvaluation) -> Option<&'static str> {
    if a.decoded.metadata != b.decoded.metadata { return Some("decoded.metadata"); }
    if a.decoded.fields.len() != b.decoded.fields.len() { return Some("decoded.fields"); }
    for (x, y) in a.decoded.fields.iter().zip(&b.decoded.fields) {
        if x.field != y.field || x.value != y.value { return Some(x.field); }
    }
    if a.diagnostics != b.diagnostics { return Some("diagnostics"); }
    (a != b).then_some("evaluation")
}

/// Encode/decode a host's state, clone-replace it, build both uncached plans for
/// `lifetime`, evaluate them, and require equality of the whole raw host, the
/// exact plans, the live-out structure, the full decoded snapshot and the
/// diagnostics. The baseline is the actual host and the existing implementation.
pub fn compare_uncached_witness(host: &HostSnapshot, ctx: &C6StateContext, lifetime: LifetimeUnit)
    -> Result<C6WitnessPreservation, C6WitnessError> {
    use C6WitnessError as E;
    let state_lanes = state_lanes_from_host(host, ctx).map_err(E::State)?;
    let rebuilt = host_with_decoded_state(host, &state_lanes, ctx).map_err(E::State)?;
    if let Some(field) = first_host_difference(host, &rebuilt) { return Err(E::HostMismatch { field }); }
    let plan = construct_plan_for(host, lifetime).map_err(E::OriginalPlan)?;
    let decoded_plan = construct_plan_for(&rebuilt, lifetime).map_err(E::DecodedPlan)?;
    if let Some((field, index)) = first_plan_difference(&plan, &decoded_plan) {
        return Err(E::PlanMismatch { field, index });
    }
    let live_out_fields = check_live_outs(&plan, lifetime)?;
    check_live_outs(&decoded_plan, lifetime)?;
    let evaluation = evaluate_uncached(&plan, host).map_err(E::OriginalEvaluation)?;
    let decoded = evaluate_uncached(&decoded_plan, &rebuilt).map_err(E::DecodedEvaluation)?;
    let cross = evaluate_uncached(&plan, &rebuilt).map_err(E::CrossEvaluation)?;
    for (against, other) in [(C6WitnessSide::Decoded, &decoded), (C6WitnessSide::Cross, &cross)] {
        if let Some(field) = first_evaluation_difference(&evaluation, other) {
            return Err(E::EvaluationMismatch { against, field });
        }
    }
    Ok(C6WitnessPreservation { lifetime, state_lanes, plan, evaluation, live_out_fields })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sherlock::helper_machine::c6_phys::{encode_frame, fixtures, L_INITIAL_FUEL};
    use crate::sherlock::helper_machine::complete_observation::{
        capture, record_step, DataFieldsSnapshot, DataSnapshot, ObservedStep,
    };
    use crate::sherlock::helper_machine::shared::adt::{CtorIdC, TypeIdC};
    use crate::sherlock::helper_machine::shared::i128_class::KIntC;
    use crate::sherlock::helper_machine::shared::owner_ext_source_program::{ExtDomain, ExtProgram};
    use crate::sherlock::helper_machine::shared::owner_ext_t3_uses::{ExtExpr, ExtFunction, ExtParamSrc, ExtValue};
    use crate::sherlock::helper_machine::shared::owner_ext_type_builder::{
        CtorDeclSrc, CtorFieldSrc, TypeDeclSrc, TypeExprSrc,
    };
    use crate::sherlock::helper_machine::shared::owner_host::host_owner_engine;
    use crate::sherlock::helper_machine::shared::{
        admit_ext_program, ActiveLengths, Admitted, AllocCounters, ControlC, CoreValue, EnvCellC, FinanceInput,
        HelperCommon, HelperControlC, HelperResultC, HelperTaskC, MachineImage, OutcomeC, OwnerEngine, RefC,
        RunIntC, SiteTag, SliceC, TaskVisitC, TriStatus,
    };
    use C6StateChild::{Data, Helper, Image};

    fn test_ctx() -> C6StateContext {
        C6StateContext::from_caps_assertion(C6StateCaps::test_caps(), true, C6StateLimits { max_total_lanes: 4096 })
    }

    /// SYNTHETIC, unadmitted: stored counters disagree with table lengths, the segment
    /// base is nonzero and several counters are u128::MAX. No owner step reaches this.
    fn synthetic() -> C6OwnedState {
        C6OwnedState {
            root: C6Root {
                control: ControlC::Eval { expr: 5 }, control_origin: 5, env_root: Some(0), stack_limit: 64,
                outcome: OutcomeC::Running, initial_fuel: u128::MAX, remaining_work: u128::MAX - 10,
                consumed_work: 10, helper_work: 3, helper_visits: u128::MAX, reference_cursor: 7,
                alloc: AllocCounters { data_nodes: 11, data_fields: 12, environment_cells: 13 },
                active: ActiveLengths { data: 21, data_fields: 22, env: 23, value_slots: 24 },
                seg_active: ActiveLengths { data: 1, data_fields: 2, env: 3, value_slots: 4 },
                seg_alloc: AllocCounters { data_nodes: 5, data_fields: 6, environment_cells: 7 },
                seg_consumed_work: u128::MAX,
            },
            frames: vec![
                FrameRowC::ScopeF { saved_env: None, origin: 1 },
                FrameRowC::ConstructorFieldsF { type_id: TypeIdC(5), ctor_id: CtorIdC(6), index: 1,
                    field_type: TypeIdC(4), checked: true, completed: SliceC { start: 2, len: 1 },
                    rest: SliceC { start: 7, len: 0 }, saved_env: Some(2), origin: 16 },
            ],
            data: C6DataTables {
                data: vec![DataSnapshot::U128 { n: u128::MAX },
                    DataSnapshot::Constructor { type_id: TypeIdC(5), ctor_id: CtorIdC(6),
                        fields: DataFieldsSnapshot { start: 0, len: 2 } },
                    DataSnapshot::Nil],
                data_fields: vec![RefC::Const(3), RefC::Data(0)],
                env: vec![EnvCellC { binder: 2, value: RefC::Data(1), previous: None }],
                value_slots: vec![RefC::Data(1), RefC::Const(0)],
            },
            helper: C6HelperTables {
                helper: Some(HelperControlC::Validate { common: HelperCommon { site: SiteTag::EvalLiteral,
                    origin_reference_cursor: u128::MAX, parent_resume: None }, root: RefC::Const(4),
                    status: TriStatus::Pending }),
                result: Some(HelperResultC::BindOk { env: Some(0) }),
                tasks: vec![HelperTaskC::ValidateTask { r: RefC::Const(4), visit: TaskVisitC::Unvisited, cursor: 0 },
                    HelperTaskC::PairTask { binder: 4, sort: TypeIdC(3), arg: RefC::Data(2) }],
            },
        }
    }

    #[test]
    fn outer_framing_lane_map_inverse_and_first_error_order() {
        let ctx = test_ctx();
        let s = synthetic();
        let lanes = encode_state(&s, &ctx).unwrap();
        assert_eq!(plan_state_lengths(&s), Ok(C6StateDirectory { image_lanes: 74, data_lanes: 35, helper_lanes: 45 }));
        assert_eq!(lanes.len(), 195);
        assert_eq!(lanes[..L_CAPS], [C6_PHYS_MAGIC, C6_PHYS_VERSION, STATE_SECTION_TAG, 3, IMAGE_CHILD_TAG, 74,
            DATA_SECTION_TAG, 35, HELPER_SECTION_TAG, 45, 1, 0]);
        assert_eq!(lanes[L_CAPS..STATE_HEADER_LANES], ctx.descriptor());
        assert_eq!((lanes[L_CAPS], lanes[L_CAPS + 10], lanes[39], lanes[40]), (64, 8, 1024, 64));
        assert_eq!(lanes[41..115], encode_root_and_frames(&s.root, &s.frames, &ctx.caps.phys).unwrap()[..]);
        assert_eq!(lanes[41 + L_INITIAL_FUEL], u128::MAX);
        assert_eq!(lanes[102..115], encode_frame(&s.frames[1]), "the last raw frame is the top");
        assert_eq!(lanes[115..150], encode_data_tables(&s.data, &ctx.caps).unwrap()[..]);
        assert_eq!(lanes[150..], encode_helper_tables(&s.helper, &ctx.caps).unwrap()[..]);
        let decoded = decode_state(&lanes, &ctx).unwrap();
        assert_eq!(decoded, s);
        assert_eq!(encode_state(&decoded, &ctx).unwrap(), lanes);
        assert_eq!(roundtrip_state_owned(&s, &ctx), Ok(s.clone()));

        let dec = |l: &[u128]| decode_state(l, &ctx);
        let set = |pairs: &[(usize, u128)]| { let mut m = lanes.clone(); for &(k, v) in pairs { m[k] = v; } m };
        let phys = |e: C6PhysError| C6StateError::Outer(e);
        let mut c = ctx;
        c.limits.max_total_lanes = 194;
        assert_eq!(decode_state(&lanes, &c), Err(C6StateError::LimitExceeded { needed: 195, limit: 194 }));
        assert_eq!(encode_state(&s, &c), Err(C6StateError::LimitExceeded { needed: 195, limit: 194 }));
        c.limits.max_total_lanes = 195;
        assert_eq!(decode_state(&lanes, &c), Ok(s.clone()));
        assert_eq!(dec(&lanes[..40]), Err(phys(C6PhysError::Truncated { section: "state", needed: 41, actual: 40 })));
        assert_eq!(dec(&set(&[(0, 0), (2, 0)])), Err(phys(C6PhysError::BadMagic { actual: 0 })));
        assert_eq!(dec(&set(&[(1, 2)])), Err(phys(C6PhysError::BadVersion { actual: 2 })));
        assert_eq!(dec(&set(&[(2, DATA_SECTION_TAG)])),
            Err(phys(C6PhysError::BadTag { section: "state", tag: DATA_SECTION_TAG })));
        assert_eq!(dec(&set(&[(3, 2), (4, 0)])), Err(C6StateError::BadSectionCount { actual: 2 }));
        // Reordered and duplicated children are fixed-position tag errors.
        assert_eq!(dec(&set(&[(4, DATA_SECTION_TAG), (5, 35), (6, IMAGE_CHILD_TAG), (7, 74)])),
            Err(C6StateError::BadChildTag { child: Image, expected: IMAGE_CHILD_TAG, actual: DATA_SECTION_TAG }));
        assert_eq!(dec(&set(&[(8, DATA_SECTION_TAG)])),
            Err(C6StateError::BadChildTag { child: Helper, expected: HELPER_SECTION_TAG, actual: DATA_SECTION_TAG }));
        assert_eq!(dec(&set(&[(7, u128::MAX)])),
            Err(phys(C6PhysError::UsizeOverflow { section: "state", value: u128::MAX })));
        assert_eq!(dec(&set(&[(7, 6)])), Err(C6StateError::ChildLengthRange { child: Data, declared: 6, minimum: 7 }));
        assert_eq!(dec(&set(&[(10, 2), (11, 1)])), Err(phys(C6PhysError::BadBool { section: "state", value: 2 })));
        assert_eq!(dec(&set(&[(11, 1)])),
            Err(phys(C6PhysError::NonZeroPadding { section: "state", lane: 11, value: 1 })));
        let mut absent = ctx;
        absent.ext_present = false;
        assert_eq!(decode_state(&set(&[(11, 1)]), &absent),
            Err(C6StateError::ContextMismatch { lane: 10, expected: 0, actual: 1 }));
        let mut narrow = ctx;
        narrow.caps.phys.max_frames = 63;
        assert_eq!(decode_state(&lanes, &narrow), Err(C6StateError::ContextMismatch { lane: 12, expected: 63, actual: 64 }));
        let mut tight = ctx;
        tight.caps.max_data_field_slots = 1;
        assert_eq!(decode_state(&lanes, &tight), Err(C6StateError::ContextMismatch { lane: 39, expected: 1, actual: 1024 }));
        assert_eq!(encode_state(&s, &tight), Err(C6StateError::Child { child: Data, offset: 115,
            error: C6PhysError::CapacityExceeded { section: "state.data_fields", needed: 2, limit: 1 } }));
        assert_eq!(dec(&set(&[(5, usize::MAX as u128)])),
            Err(phys(C6PhysError::UsizeOverflow { section: "state", value: usize::MAX as u128 })));
        // Outer length precedes every child body error.
        assert_eq!(dec(&set(&[(5, 75), (89, 10)])),
            Err(phys(C6PhysError::Truncated { section: "state", needed: 196, actual: 195 })));
        let mut trailing = lanes.clone();
        trailing.push(0);
        assert_eq!(dec(&trailing), Err(phys(C6PhysError::TrailingLanes { section: "state", extra: 1 })));
        // Declared boundaries are used as declared; the child's exact-length check refuses.
        assert_eq!(dec(&set(&[(5, 87), (7, 22)])), Err(C6StateError::Child { child: Image, offset: 41,
            error: C6PhysError::TrailingLanes { section: "image_section", extra: 13 } }));
        assert_eq!(dec(&set(&[(5, 61), (7, 48)])), Err(C6StateError::Child { child: Image, offset: 41,
            error: C6PhysError::Truncated { section: "image_section", needed: 74, actual: 61 } }));
        assert_eq!(dec(&set(&[(89, 10), (122, 7)])), Err(C6StateError::Child { child: Image, offset: 41,
            error: C6PhysError::BadTag { section: "frame", tag: 10 } }));
        assert_eq!(dec(&set(&[(122, 7)])), Err(C6StateError::Child { child: Data, offset: 115,
            error: C6PhysError::BadTag { section: "state.data", tag: 7 } }));
        assert_eq!(dec(&set(&[(117, HELPER_SECTION_TAG)])), Err(C6StateError::Child { child: Data, offset: 115,
            error: C6PhysError::BadTag { section: "state_data", tag: HELPER_SECTION_TAG } }));
        assert_eq!(dec(&set(&[(154, 12)])), Err(C6StateError::Child { child: Helper, offset: 150,
            error: C6PhysError::BadTag { section: "state.helper", tag: 12 } }));
        let mut deep = s.clone();
        deep.root.stack_limit = 65;
        assert_eq!(encode_state(&deep, &ctx), Err(C6StateError::Child { child: Image, offset: 41,
            error: C6PhysError::CapacityExceeded { section: "root.stack_limit", needed: 65, limit: 64 } }));
    }

    #[test]
    fn every_owned_field_is_significant_and_external_fields_are_retained() {
        let ctx = test_ctx();
        let base_state = synthetic();
        let base = encode_state(&base_state, &ctx).unwrap();
        let mutations: [(&str, fn(&mut C6OwnedState)); 29] = [
            ("control", |s| s.root.control = ControlC::Eval { expr: 6 }),
            ("control_origin", |s| s.root.control_origin = 6),
            ("env_root", |s| s.root.env_root = None),
            ("stack_limit", |s| s.root.stack_limit = 63),
            ("outcome", |s| s.root.outcome = OutcomeC::Halted { value: RefC::Data(0) }),
            ("initial_fuel", |s| s.root.initial_fuel ^= 1),
            ("remaining_work", |s| s.root.remaining_work ^= 1),
            ("consumed_work", |s| s.root.consumed_work += 1),
            ("helper_work", |s| s.root.helper_work += 1),
            ("helper_visits", |s| s.root.helper_visits ^= 1),
            ("reference_cursor", |s| s.root.reference_cursor += 1),
            ("alloc", |s| s.root.alloc.data_fields += 1),
            ("active", |s| s.root.active.value_slots += 1),
            ("seg_active", |s| s.root.seg_active.env += 1),
            ("seg_alloc", |s| s.root.seg_alloc.data_nodes += 1),
            ("seg_consumed_work", |s| s.root.seg_consumed_work ^= 1),
            ("frame order", |s| s.frames.swap(0, 1)),
            ("frame count", |s| { s.frames.pop(); }),
            ("raw empty rest start", |s| if let FrameRowC::ConstructorFieldsF { rest, .. } = &mut s.frames[1] { rest.start = 8; }),
            ("data", |s| s.data.data[0] = DataSnapshot::U128 { n: u128::MAX - 1 }),
            ("data_fields order", |s| s.data.data_fields.swap(0, 1)),
            ("env previous", |s| s.data.env[0].previous = Some(0)),
            ("value_slots", |s| s.data.value_slots.push(RefC::Const(0))),
            ("helper presence", |s| s.helper.helper = None),
            ("helper origin", |s| if let Some(HelperControlC::Validate { common, .. }) = &mut s.helper.helper { common.origin_reference_cursor ^= 1; }),
            ("result presence", |s| s.helper.result = None),
            ("result env", |s| s.helper.result = Some(HelperResultC::BindOk { env: None })),
            ("task order", |s| s.helper.tasks.swap(0, 1)),
            ("task multiplicity", |s| s.helper.tasks.push(s.helper.tasks[0])),
        ];
        for (name, mutate) in mutations {
            let mut m = base_state.clone();
            mutate(&mut m);
            let lanes = encode_state(&m, &ctx).unwrap();
            assert_ne!(lanes, base, "{name} was ignored");
            assert_eq!(decode_state(&lanes, &ctx), Ok(m), "{name}");
        }
        // Stored counters are carried, never normalized to table lengths.
        let decoded = decode_state(&base, &ctx).unwrap();
        assert_eq!((decoded.root.active.data, decoded.data.data.len()), (21, 3));
        // Empty tables, absent helper/result, and env None versus present Some(0).
        let mut empty = synthetic();
        empty.frames.clear();
        empty.data = C6DataTables { data: vec![], data_fields: vec![], env: vec![], value_slots: vec![] };
        empty.helper = C6HelperTables { helper: None, result: None, tasks: vec![] };
        let some_zero = encode_state(&empty, &ctx).unwrap();
        assert_eq!(some_zero.len(), 41 + 48 + 7 + 31);
        assert_eq!(decode_state(&some_zero, &ctx), Ok(empty.clone()));
        empty.root.env_root = None;
        let none = encode_state(&empty, &ctx).unwrap();
        assert_ne!(none, some_zero);
        assert_eq!(decode_state(&none, &ctx), Ok(empty));
        // Absent and present-zero ext give equal caps but distinct framed bytes.
        let (b, lc, budgets, limits) = (BoundsC::default(), LayoutCapacitiesC::default(),
            C6CodecBudgets { type_expr_nodes: 0, template_bytes: 0 }, C6StateLimits { max_total_lanes: 4096 });
        let absent = C6StateContext::from_parts(&b, &lc, None, budgets, limits);
        let zero_ext = C6StateContext::from_parts(&b, &lc, Some(ExtCapacitiesC::all_zero()), budgets, limits);
        assert_eq!((absent.caps(), absent.ext_present(), zero_ext.ext_present()), (zero_ext.caps(), false, true));
        // Clone-replace on a REAL captured host: only encoded fields change; the
        // synthetic nonzero segment base is materialized via SegBase::from_parts.
        let (admitted, image, _layout) = admitted_with_layout();
        let host = fixtures::must(capture(&admitted, &image), "capture");
        let out = host_with_decoded_state(&host, &base, &ctx).unwrap();
        assert_eq!((&out.admission, &out.image.run_id), (&host.admission, &host.image.run_id));
        assert_eq!(C6OwnedState::from_image(&out.image), base_state);
        let r = &base_state.root;
        assert_eq!(out.image.seg_base, SegBase::from_parts(r.seg_active, r.seg_alloc, r.seg_consumed_work));
        assert_ne!(out.image.seg_base, SegBase::genesis_uncut());
        assert_eq!(first_host_difference(&host, &out), Some("image.control"));
        let mut back = out.clone();
        C6OwnedState::from_image(&host.image).replace_state_of(&mut back.image);
        assert_eq!(back, host);
    }

    /// Local fixture retaining its ORIGINAL admitting layout. Two-argument call,
    /// nested nominal constructors; real generic admission and the owner's admit_run.
    fn admitted_with_layout() -> (Admitted, MachineImage, LayoutSchemaC) {
        let bounds = BoundsC { code_rows: 128, function_rows: 16, code_child_slots: 128, parameter_slots: 32,
            pattern_rows: 32, match_plan_rows: 32, match_arm_rows: 32, type_rows: 16, constructor_rows: 16,
            constructor_field_slots: 32, max_arity: 8, constant_nodes: 128, constant_field_slots: 128,
            data_nodes: 128, data_field_slots: 128, environment_cells: 128, frames: 64, helper_tasks: 64,
            serialization_bytes: 1_000_000, max_reference_work: 1_000_000, max_helper_work: 1_000_000, step_cap: 10_000 };
        let layout = LayoutSchemaC { layout_version: ("c6-phys-state-test".into(), "1".into()), field_id: None,
            widths: None, scalar_layout_id: None, bounds,
            capacities: LayoutCapacitiesC { name_rows: 128, name_bytes: 4096, occurrence_path_slots: 4096, value_slots: 4096 },
            ext_capacities: Some(ExtCapacitiesC { plan_case_slots: 64, field_path_slots: 64, pattern_child_slots: 64,
                ..ExtCapacitiesC::all_zero() }),
            boundary_capacities_present: false };
        let field = |name: &str, ty: TypeExprSrc| CtorFieldSrc { name: name.into(), ty };
        let var = |name: &str| ExtExpr::Var { name: name.into() };
        let lit = |n: u128| ExtExpr::Literal { value: ExtValue::Core { value: CoreValue::UInt { n: KIntC::InRange(n) } } };
        let pair = || TypeExprSrc::Named { name: "Pair".into() };
        let box_uint = || TypeExprSrc::Apply { name: "Box".into(), args: vec![TypeExprSrc::UInt] };
        let param = |name: &str| ExtParamSrc { name: name.into(), ty: TypeExprSrc::UInt };
        // `type Box<A> = MkBox { item: A }`, `type Pair = MkPair { fst: UInt, snd: Box<UInt> }`,
        // `fn mk(x: UInt, y: UInt) -> Pair { MkPair(x, MkBox(y)) }`, entry `let p = mk(7, 8) in p`.
        let mk = ExtFunction { name: "mk".into(), parameters: vec![param("x"), param("y")], return_type: pair(),
            body: ExtExpr::Construct { ty: pair(), ctor: "MkPair".into(), fields: vec![var("x"),
                ExtExpr::Construct { ty: box_uint(), ctor: "MkBox".into(), fields: vec![var("y")] }] } };
        let program = ExtProgram {
            types: vec![
                TypeDeclSrc { name: "Box".into(), parameters: vec!["A".into()], ctors: vec![CtorDeclSrc {
                    name: "MkBox".into(), fields: vec![field("item", TypeExprSrc::Param { name: "A".into() })] }] },
                TypeDeclSrc { name: "Pair".into(), parameters: Vec::new(), ctors: vec![CtorDeclSrc {
                    name: "MkPair".into(), fields: vec![field("fst", TypeExprSrc::UInt), field("snd", box_uint())] }] },
            ],
            assets: Vec::new(), states: Vec::new(), inputs: Vec::new(), functions: vec![mk],
            entry: ExtExpr::Let { name: "p".into(),
                value: Box::new(ExtExpr::Call { name: "mk".into(), args: vec![lit(7), lit(8)] }), body: Box::new(var("p")) },
        };
        let (admitted, _report) = fixtures::must(admit_ext_program(program, ExtDomain::GENERIC_ABSENT, &bounds,
            &layout, None), "generic nominal admission");
        let image = fixtures::must(host_owner_engine().admit_run(&admitted, &RunIntC::Nat(fixtures::FUEL),
            &RunIntC::Nat(64), &FinanceInput::Absent), "admit run");
        (admitted, image, layout)
    }

    fn real_ctx(layout: &LayoutSchemaC) -> C6StateContext {
        let budgets = C6CodecBudgets { type_expr_nodes: 0, template_bytes: 0 };
        let limits = C6StateLimits { max_total_lanes: 1 << 20 };
        let ctx = C6StateContext::from_layout(layout, budgets, limits);
        assert_eq!(ctx, C6StateContext::from_parts(&layout.bounds, &layout.capacities, layout.ext_capacities, budgets, limits));
        ctx
    }

    #[derive(Default)]
    struct Seen { args_completed: bool, return_f: bool, ctor_completed: bool, bind_pairs: bool, append: bool,
        build: bool, scalar_expected: bool, result: bool, env: bool, constructor_data: bool }

    fn observe(seen: &mut Seen, img: &ImageSnapshot) {
        for frame in &img.frames {
            match frame {
                FrameRowC::ArgsF { completed, .. } if completed.len > 0 => seen.args_completed = true,
                FrameRowC::ReturnF { .. } => seen.return_f = true,
                FrameRowC::ConstructorFieldsF { completed, .. } if completed.len > 0 => seen.ctor_completed = true,
                _ => {}
            }
        }
        match img.helper {
            Some(HelperControlC::Bind { .. }) if img.tasks.iter().any(|t| matches!(t, HelperTaskC::PairTask { .. })) =>
                seen.bind_pairs = true,
            Some(HelperControlC::Append { .. }) => seen.append = true,
            Some(HelperControlC::Build { .. }) => seen.build = true,
            Some(HelperControlC::Scalar { expected: Some(_), .. }) => seen.scalar_expected = true,
            _ => {}
        }
        seen.result |= img.result.is_some();
        seen.env |= img.env_root.is_some() && !img.env.is_empty();
        seen.constructor_data |= img.data.iter().any(|d| matches!(d, DataSnapshot::Constructor { .. }));
    }

    #[test]
    fn owner_trace_states_roundtrip_and_clone_replace_whole_host() {
        // OWNER-REACHABLE: real admission, real capture/record_step; ctx from the retained layout.
        let (admitted, mut image, layout) = admitted_with_layout();
        let ctx = real_ctx(&layout);
        let first = fixtures::must(capture(&admitted, &image), "capture");
        assert_eq!(retained_layout_agrees_with_host(&layout, &first), Ok(()));
        let mut wrong = layout.clone();
        wrong.bounds.frames += 1;
        assert_eq!(retained_layout_agrees_with_host(&wrong, &first), Err(C6StateError::RetainedLayoutMismatch { field: "bounds" }));
        let mut tampered = first.clone();
        tampered.image.run_id.fuel_input = RunIntC::Nat(1);
        assert_eq!(first_host_difference(&first, &tampered), Some("image.run_id"));
        let check = |host: &HostSnapshot| -> Vec<u128> {
            let lanes = state_lanes_from_host(host, &ctx).unwrap();
            let decoded = decode_state(&lanes, &ctx).unwrap();
            assert_eq!(decoded, C6OwnedState::from_image(&host.image));
            assert_eq!(encode_state(&decoded, &ctx).unwrap(), lanes);
            assert_eq!(host_with_decoded_state(host, &lanes, &ctx).unwrap(), *host, "whole clone-replaced host");
            lanes
        };
        let mut seen = Seen::default();
        let mut terminal = None;
        for step in 0..4096 {
            match record_step(&admitted, &mut image) {
                Ok(ObservedStep::Event(record)) => {
                    let (before, after) = (check(&record.before), check(&record.after));
                    assert_eq!(before != after, record.before != record.after, "step {step}");
                    observe(&mut seen, &record.before.image);
                }
                Ok(ObservedStep::Terminal { before, after }) => {
                    check(&before);
                    check(&after);
                    terminal = Some(after);
                    break;
                }
                Err(_) => panic!("owner step {step}"),
            }
        }
        let last = terminal.expect("terminal within fuel and 4096 steps");
        assert!(matches!(last.image.outcome, OutcomeC::Halted { .. }));
        assert_eq!(retained_layout_agrees_with_host(&layout, &last), Ok(()));
        assert!(seen.args_completed, "ArgsF with a nonempty completed prefix");
        assert!(seen.return_f && seen.ctor_completed && seen.bind_pairs && seen.append && seen.build);
        assert!(seen.scalar_expected && seen.result && seen.env && seen.constructor_data);
    }

    #[test]
    fn uncached_witness_plans_and_decodes_are_preserved_over_the_owner_trace() {
        let (admitted, mut image, layout) = admitted_with_layout();
        let ctx = real_ctx(&layout);
        let (mut compared, mut helpers, mut tasks, mut results, mut ctor_frames) = (0usize, 0usize, 0usize, 0usize, 0usize);
        let mut verify = |host: &HostSnapshot, lifetime: LifetimeUnit, step: usize| {
            let report = compare_uncached_witness(host, &ctx, lifetime)
                .unwrap_or_else(|e| panic!("step {step} {lifetime:?}: {e:?}"));
            assert_eq!(report.lifetime, lifetime);
            assert_eq!(report.live_out_fields, 33);
            assert_eq!(report.plan.live_outs.len(), 34);
            assert_eq!(report.state_lanes, state_lanes_from_host(host, &ctx).unwrap());
            assert_eq!(report.evaluation.decoded.metadata.seg_base, host.image.seg_base);
            compared += 1;
            helpers += usize::from(host.image.helper.is_some());
            tasks += usize::from(!host.image.tasks.is_empty());
            results += usize::from(host.image.result.is_some());
            ctor_frames += usize::from(host.image.frames.iter().any(|f| matches!(f, FrameRowC::ConstructorFieldsF { .. })));
        };
        let mut terminal = false;
        for step in 0..4096 {
            if step % 8 == 0 {
                verify(&fixtures::must(capture(&admitted, &image), "capture"), LifetimeUnit::Snapshot, step);
            }
            match record_step(&admitted, &mut image) {
                Ok(ObservedStep::Event(record)) => {
                    verify(&record.before, LifetimeUnit::RecordBefore, step);
                    verify(&record.after, LifetimeUnit::RecordAfter, step);
                }
                Ok(ObservedStep::Terminal { before, after }) => {
                    verify(&before, LifetimeUnit::RecordBefore, step);
                    verify(&after, LifetimeUnit::RecordAfter, step);
                    verify(&after, LifetimeUnit::Snapshot, step);
                    terminal = true;
                    break;
                }
                Err(_) => panic!("owner step {step}"),
            }
        }
        assert!(terminal && compared > 0);
        assert!(helpers > 0 && tasks > 0 && results > 0 && ctor_frames > 0);
    }
}
