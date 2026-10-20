//! C6P1 (`sherlock-c6-phys/1`) pinned-SDK constraint emitter and single-step
//! statement binding for `KMachinePushAccept` (`A135`) and
//! `KMachinePushRefuse` (`A140`).
//!
//! Scope and assumptions (read before use):
//! - Real pinned Midnight SDK only (`midnight-zkir-v3`,
//!   `midnight-transient-crypto`); no local `Fr`, `Instruction`, or IR type.
//! - The fixed-capacity padded carrier (`C6_PADDED_CARRIER_VERSION = 1`) is a
//!   consequential UNADOPTED adapter. It is the exact checked inverse of the
//!   canonical variable-length Core codecs (`encode_root_and_frames`,
//!   `encode_data_tables`, `encode_helper_tables` and their decoders) under
//!   caller-retained `C6StateCaps`/`C6StateLimits`; caller bounds are never
//!   raised. Inactive slots must be zero. Geometry is checked before any
//!   allocation and refused above `limits.max_total_lanes`.
//! - Circuit obligations actually emitted: 128-bit lane ranges, host-prefix
//!   (`META`/`PROG`/`CTX`) header and pre/post equality, section headers,
//!   `OutcomeC::Running`, positive fuel, absent helper/result/tasks,
//!   `ScheduleNext` control, full pending-frame structural validation for all
//!   10 `FrameRowC` variants (tags, binary/unary sub-tags, `RefC` caps split by
//!   Const/Data table, `EnvRootC` option contract, raw slices with end-overflow
//!   in field arithmetic below the BLS12-381 modulus, builtin TypeId cap
//!   `max(max_types, 4)` for the five TypeId fields, per-variant zero padding),
//!   branch selection `depth < stack_limit`, Accept-only
//!   `depth < admission.bounds.frames` read from the bound `META` lane and
//!   `depth < max_frames`, exact `Eval{next}` / `Error{4}` post control,
//!   `ScheduleNext.env` restoration into `env_root` on Accept and preservation
//!   on Refuse, checked work/reference-cursor charge (+1/+1, no wrap at
//!   `u128::MAX`), frame append at slot `depth` with every other slot and
//!   every unaffected header lane preserved, `DATA`/`HELPER` equality with
//!   count caps and zero inactive padding, and disabled-step `post == pre`.
//! - Explicit caller assumptions, NOT circuit-proved here: full canonicality
//!   of invariant `DATA`/`HELPER` active rows, `META` text/digest payloads and
//!   `PROG` tables beyond headers/equality (witnesses must be produced through
//!   `decode_padded_state_to_owned` / the host encoders in this file);
//!   `StepInv`, admission and digest authentication. `C6HostMetadata` digests
//!   stay opaque (`DigestC`), `bind_decoded_host` membership is not used as
//!   step evidence, a captured PRE is never treated as POST, and producer
//!   rejection (`C6RecordError`) is never conflated with circuit rejection:
//!   `witness_from_host_pair` feeds independently supplied post snapshots to
//!   the relation without running any recorder. `run_id` is preserved exactly
//!   through `C6HostMetadata::from_host`. `max_total_lanes` bounds only the
//!   padded carrier lanes; it does not bound DAG witness unfolding or
//!   environment traversal work. Target remains Midnight STAGENET; no native
//!   proof, ledger settlement, whole-host 39-selector dispatch, groups
//!   1/2/4/8, residual/finance/persistence constraints, or multi-family
//!   transcript chaining is delivered by this first Push module.

use std::sync::Arc;

use midnight_transient_crypto::curve::{Fr, FR_BITS};
use midnight_transient_crypto::hash::transient_commit;
use midnight_transient_crypto::proofs::{KeyLocation, ProofPreimage, VERIFIER_MAX_DEGREE, Zkir};
use midnight_zkir_v3::ir::{IrMinorVersion, Operand, TypedIdentifier};
use midnight_zkir_v3::ir_types::IrType;
use midnight_zkir_v3::{Identifier, Instruction, IrSource};
use mori_core::sherlock::helper_machine::c6_phys::{
    decode_image_section, encode_root_and_frames, push_selection, record_a135_push_step,
    record_a140_refusal_step, C6PhysCaps, C6PhysError, C6Result, PushSelection, CONTROL_LANES,
    FRAME_LANES, IMAGE_SECTION_HEADER_LANES, L_CONTROL, L_CONTROL_ORIGIN, L_ENV_ROOT,
    L_FRAME_COUNT, L_MAGIC, L_OUTCOME, L_REFERENCE_CURSOR, L_CONSUMED_WORK, L_REMAINING_WORK,
    L_STACK_LIMIT, L_VERSION, PUSH_REFUSE_ERROR_CODE,
};
use mori_core::sherlock::helper_machine::c6_phys_data::{
    decode_data_tables, encode_data_tables, C6DataTables, C6StateCaps, DATA_HEADER_LANES,
    DATA_ROW_LANES, DATA_SECTION_TAG, ENV_ROW_LANES, REF_LANES,
};
use mori_core::sherlock::helper_machine::c6_phys_helper::{
    decode_helper_tables, encode_helper_tables, C6HelperTables, HELPER_FIXED_LANES,
    HELPER_SECTION_TAG, TASK_LANES,
};
use mori_core::sherlock::helper_machine::c6_phys_host::{
    decode_context, decode_metadata, encode_context, encode_metadata,
    C6HostConfig, C6HostLimits, C6HostMetadata, CTX_LANES, CTX_SECTION_TAG,
    C6_HOST_SCHEMA, META_HEADER_LANES, META_SECTION_TAG,
};
use mori_core::sherlock::helper_machine::c6_phys_program::{
    decode_program, encode_program, PROGRAM_HEADER_LANES, PROGRAM_SECTION_TAG,
};
use mori_core::sherlock::helper_machine::c6_phys_state::{C6OwnedState, C6StateLimits};
use mori_core::sherlock::helper_machine::complete_observation::HostSnapshot;
use mori_core::sherlock::helper_machine::shared::{
    Admitted, BoundsC, DigestC, IdentityC, MachineImage,
};
use serde_json::json;
use thiserror::Error;

use super::c6_a135_push::c6_push_family;
use super::c6_data_canonical::{emit_c6_data_canonical_block, C6DataCanonicalInputs};
use super::c6_meta_canonical::{emit_c6_meta_canonical_block, C6MetaCanonicalInputs};
use super::c6_program_context_canonical::{
    emit_c6_program_context_canonical_block, C6ProgramContextCanonicalInputs,
};

/// Version of the unadopted fixed-capacity padded state carrier.
pub const C6_PADDED_CARRIER_VERSION: u128 = 1;

const CHECK_KEY_LOCATION: &str = "sherlock_c6_push_step_check";
const LANE_BITS: u32 = 128;
const CONTROL_TAG_EVAL: u128 = 0;
const CONTROL_TAG_SCHEDULE_NEXT: u128 = 2;
const CONTROL_TAG_ERROR: u128 = 9;
const FRAME_VARIANT_COUNT: usize = 10;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum C6PushStepError {
    #[error("invalid scope identifier: must not be empty")]
    EmptyScope,
    #[error("lane bit width {bits} is invalid for the BLS12-381 scalar field")]
    InvalidBitWidth { bits: u32 },
    #[error("geometry arithmetic overflow in section {section}")]
    GeometryOverflow { section: &'static str },
    #[error("padded carrier lane count {needed} exceeds state limit {limit}")]
    StateLimitExceeded { needed: usize, limit: usize },
    #[error("padded carrier length mismatch in {section}: expected {expected}, got {actual}")]
    LengthMismatch { section: &'static str, expected: usize, actual: usize },
    #[error("active count {count} in {section} exceeds capacity {limit}")]
    CapacityExceeded { section: &'static str, count: usize, limit: usize },
    #[error("non-zero inactive padding in {section} at slot {slot}, lane {lane}: {value}")]
    NonZeroInactivePadding { section: &'static str, slot: usize, lane: usize, value: u128 },
    #[error("host prefix section {section} mismatch between pre and post state")]
    HostPrefixMismatch { section: &'static str },
    #[error("metadata admission bounds frames offset {offset} out of range for {len} lanes")]
    MetadataBoundsOffsetOutOfRange { offset: usize, len: usize },
    #[error("metadata bounds frames lane {lane_val} != decoded {decoded_val}")]
    MetadataBoundsFramesDisagreement { lane_val: u128, decoded_val: u128 },
    #[error("retained host configuration mismatch in {field}")]
    ConfigMismatch { field: &'static str },
    #[error("padded carrier geometry does not equal its checked recomputation in {section}")]
    InvalidGeometry { section: &'static str },
    #[error("circuit does not match the verifier-pinned C6 circuit in {field}")]
    CircuitPinMismatch { field: &'static str },
    #[error("witness is not canonical in {section}: {detail}")]
    NonCanonicalWitness { section: &'static str, detail: String },
    #[error("coarse statement precheck refused {count} statement lanes at limit {limit}; this is not a measured degree")]
    CoarseStatementLimit { count: usize, limit: usize },
    #[error("SDK check returned unexpected public-input skips: {0:?}")]
    UnexpectedPiSkips(Vec<Option<usize>>),
    #[error("core C6 physical error: {0:?}")]
    Phys(C6PhysError),
    #[error("zkir serialization or check error: {0}")]
    Backend(String),
}

impl From<C6PhysError> for C6PushStepError {
    fn from(err: C6PhysError) -> Self { Self::Phys(err) }
}

/// Checked fixed-capacity geometry. Allocation policy: all sizes are computed
/// with `checked_mul`/`checked_add` and compared with `max_total_lanes` before
/// any `Vec` is allocated by callers of this geometry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct C6PaddedCarrierGeometry {
    pub caps: C6StateCaps,
    pub image_total_lanes: usize,
    pub data_nodes_lanes: usize,
    pub data_fields_lanes: usize,
    pub env_cells_lanes: usize,
    pub value_slots_lanes: usize,
    pub data_total_lanes: usize,
    pub helper_tasks_lanes: usize,
    pub helper_total_lanes: usize,
    pub state_total_lanes: usize,
}

impl C6PaddedCarrierGeometry {
    pub fn checked_new(caps: C6StateCaps, limits: C6StateLimits) -> Result<Self, C6PushStepError> {
        let ov = |section: &'static str| C6PushStepError::GeometryOverflow { section };
        let image_total_lanes = caps.phys.max_frames.checked_mul(FRAME_LANES)
            .and_then(|n| n.checked_add(IMAGE_SECTION_HEADER_LANES)).ok_or_else(|| ov("image"))?;
        let data_nodes_lanes = caps.phys.max_data_nodes.checked_mul(DATA_ROW_LANES).ok_or_else(|| ov("data.data"))?;
        let data_fields_lanes = caps.max_data_field_slots.checked_mul(REF_LANES).ok_or_else(|| ov("data.fields"))?;
        let env_cells_lanes = caps.phys.max_env_cells.checked_mul(ENV_ROW_LANES).ok_or_else(|| ov("data.env"))?;
        let value_slots_lanes = caps.phys.max_value_slots.checked_mul(REF_LANES).ok_or_else(|| ov("data.values"))?;
        let data_total_lanes = DATA_HEADER_LANES.checked_add(data_nodes_lanes)
            .and_then(|n| n.checked_add(data_fields_lanes))
            .and_then(|n| n.checked_add(env_cells_lanes))
            .and_then(|n| n.checked_add(value_slots_lanes)).ok_or_else(|| ov("data"))?;
        let helper_tasks_lanes = caps.max_helper_tasks.checked_mul(TASK_LANES).ok_or_else(|| ov("helper.tasks"))?;
        let helper_total_lanes = HELPER_FIXED_LANES.checked_add(helper_tasks_lanes).ok_or_else(|| ov("helper"))?;
        let state_total_lanes = image_total_lanes.checked_add(data_total_lanes)
            .and_then(|n| n.checked_add(helper_total_lanes)).ok_or_else(|| ov("state"))?;
        if state_total_lanes > limits.max_total_lanes {
            return Err(C6PushStepError::StateLimitExceeded { needed: state_total_lanes, limit: limits.max_total_lanes });
        }
        Ok(Self { caps, image_total_lanes, data_nodes_lanes, data_fields_lanes, env_cells_lanes,
            value_slots_lanes, data_total_lanes, helper_tasks_lanes, helper_total_lanes, state_total_lanes })
    }

    pub const fn frame_slot_offset(&self, slot: usize) -> usize {
        IMAGE_SECTION_HEADER_LANES + slot * FRAME_LANES
    }
    pub const fn data_fields_base(&self) -> usize { DATA_HEADER_LANES + self.data_nodes_lanes }
    pub const fn env_base(&self) -> usize { self.data_fields_base() + self.data_fields_lanes }
    pub const fn value_slots_base(&self) -> usize { self.env_base() + self.env_cells_lanes }
}

impl C6PaddedCarrierGeometry {
    /// Recompute every length from `caps`; public or mutated geometry values
    /// are never trusted for indexing.
    pub fn validate(&self) -> Result<(), C6PushStepError> {
        let recomputed = Self::checked_new(
            self.caps,
            C6StateLimits { max_total_lanes: self.state_total_lanes },
        )?;
        if recomputed != *self {
            return Err(C6PushStepError::InvalidGeometry { section: "geometry" });
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct C6PaddedStateLanes {
    geometry: C6PaddedCarrierGeometry,
    image: Vec<u128>,
    data: Vec<u128>,
    helper: Vec<u128>,
}

impl C6PaddedStateLanes {
    pub fn to_flat_lanes(&self) -> Vec<u128> {
        let mut out = Vec::with_capacity(self.geometry.state_total_lanes);
        out.extend_from_slice(&self.image);
        out.extend_from_slice(&self.data);
        out.extend_from_slice(&self.helper);
        out
    }
}

fn pad_to(canonical: Vec<u128>, total: usize, section: &'static str) -> Result<Vec<u128>, C6PushStepError> {
    if canonical.len() > total {
        return Err(C6PushStepError::LengthMismatch { section, expected: total, actual: canonical.len() });
    }
    let mut out = vec![0u128; total];
    out[..canonical.len()].copy_from_slice(&canonical);
    Ok(out)
}

/// Canonical variable-length encoding via the Core codecs, zero-padded to the
/// checked fixed capacity. Counts above capacity are refused by the Core
/// encoders themselves; nothing is truncated.
fn scatter_data_tables(
    compact: &[u128], g: &C6PaddedCarrierGeometry,
) -> Result<Vec<u128>, C6PushStepError> {
    if compact.len() < DATA_HEADER_LANES {
        return Err(C6PushStepError::LengthMismatch {
            section: "data", expected: DATA_HEADER_LANES, actual: compact.len(),
        });
    }
    let counts = [
        count_lane("data.data", compact, 3)?,
        count_lane("data.data_fields", compact, 4)?,
        count_lane("data.env", compact, 5)?,
        count_lane("data.value_slots", compact, 6)?,
    ];
    let widths = [DATA_ROW_LANES, REF_LANES, ENV_ROW_LANES, REF_LANES];
    let caps = [
        g.caps.phys.max_data_nodes, g.caps.max_data_field_slots,
        g.caps.phys.max_env_cells, g.caps.phys.max_value_slots,
    ];
    let dst_bases = [
        DATA_HEADER_LANES, g.data_fields_base(), g.env_base(), g.value_slots_base(),
    ];
    let mut out = vec![0u128; g.data_total_lanes];
    out[..DATA_HEADER_LANES].copy_from_slice(&compact[..DATA_HEADER_LANES]);
    let mut src = DATA_HEADER_LANES;
    for i in 0..4 {
        if counts[i] > caps[i] {
            return Err(C6PushStepError::CapacityExceeded {
                section: "data", count: counts[i], limit: caps[i],
            });
        }
        let len = counts[i].checked_mul(widths[i]).ok_or(
            C6PushStepError::GeometryOverflow { section: "data" })?;
        let src_end = src.checked_add(len).ok_or(
            C6PushStepError::GeometryOverflow { section: "data" })?;
        let dst_end = dst_bases[i].checked_add(len).ok_or(
            C6PushStepError::GeometryOverflow { section: "data" })?;
        out[dst_bases[i]..dst_end].copy_from_slice(
            compact.get(src..src_end).ok_or(C6PushStepError::LengthMismatch {
                section: "data", expected: src_end, actual: compact.len(),
            })?);
        src = src_end;
    }
    if src != compact.len() {
        return Err(C6PushStepError::LengthMismatch {
            section: "data", expected: src, actual: compact.len(),
        });
    }
    Ok(out)
}

pub fn encode_owned_state_to_padded(
    state: &C6OwnedState, caps: C6StateCaps, limits: C6StateLimits,
) -> Result<C6PaddedStateLanes, C6PushStepError> {
    let geometry = C6PaddedCarrierGeometry::checked_new(caps, limits)?;
    let image = pad_to(encode_root_and_frames(&state.root, &state.frames, &caps.phys)?, geometry.image_total_lanes, "image")?;
    let data = scatter_data_tables(&encode_data_tables(&state.data, &caps)?, &geometry)?;
    let helper = pad_to(encode_helper_tables(&state.helper, &caps)?, geometry.helper_total_lanes, "helper")?;
    let padded = C6PaddedStateLanes { geometry, image, data, helper };
    if decode_padded_state_to_owned(&padded)? != *state {
        return Err(C6PushStepError::Backend("padded roundtrip disagreed with owned state".into()));
    }
    Ok(padded)
}

fn verify_inactive_zero(
    section: &'static str, lanes: &[u128], base: usize, count: usize, cap: usize, width: usize,
) -> Result<(), C6PushStepError> {
    if count > cap {
        return Err(C6PushStepError::CapacityExceeded { section, count, limit: cap });
    }
    for slot in count..cap {
        for lane in 0..width {
            let value = lanes[base + slot * width + lane];
            if value != 0 {
                return Err(C6PushStepError::NonZeroInactivePadding { section, slot, lane, value });
            }
        }
    }
    Ok(())
}

fn count_lane(section: &'static str, lanes: &[u128], at: usize) -> Result<usize, C6PushStepError> {
    usize::try_from(lanes[at]).map_err(|_| C6PushStepError::Phys(C6PhysError::UsizeOverflow { section, value: lanes[at] }))
}

/// Exact inverse of `encode_owned_state_to_padded`: every inactive slot is
/// checked for zero padding before the canonical Core decoders run.
pub fn decode_padded_state_to_owned(padded: &C6PaddedStateLanes) -> Result<C6OwnedState, C6PushStepError> {
    let g = &padded.geometry;
    for (section, actual, expected) in [("image", padded.image.len(), g.image_total_lanes),
        ("data", padded.data.len(), g.data_total_lanes), ("helper", padded.helper.len(), g.helper_total_lanes)] {
        if actual != expected {
            return Err(C6PushStepError::LengthMismatch { section, expected, actual });
        }
    }
    let caps = &g.caps;
    let frame_count = count_lane("image.frames", &padded.image, L_FRAME_COUNT)?;
    verify_inactive_zero("image.frames", &padded.image, IMAGE_SECTION_HEADER_LANES, frame_count, caps.phys.max_frames, FRAME_LANES)?;
    let (root, frames) = decode_image_section(&padded.image[..IMAGE_SECTION_HEADER_LANES + frame_count * FRAME_LANES], &caps.phys)?;

    let dc = count_lane("data.data", &padded.data, 3)?;
    let dfc = count_lane("data.data_fields", &padded.data, 4)?;
    let ec = count_lane("data.env", &padded.data, 5)?;
    let vc = count_lane("data.value_slots", &padded.data, 6)?;
    verify_inactive_zero("data.data", &padded.data, DATA_HEADER_LANES, dc, caps.phys.max_data_nodes, DATA_ROW_LANES)?;
    verify_inactive_zero("data.data_fields", &padded.data, g.data_fields_base(), dfc, caps.max_data_field_slots, REF_LANES)?;
    verify_inactive_zero("data.env", &padded.data, g.env_base(), ec, caps.phys.max_env_cells, ENV_ROW_LANES)?;
    verify_inactive_zero("data.value_slots", &padded.data, g.value_slots_base(), vc, caps.phys.max_value_slots, REF_LANES)?;
    let data_len = g.value_slots_base() + vc * REF_LANES;
    // Rebuild the exact variable-length section in table order.
    let mut raw_data = Vec::with_capacity(data_len);
    raw_data.extend_from_slice(&padded.data[..DATA_HEADER_LANES]);
    raw_data.extend_from_slice(&padded.data[DATA_HEADER_LANES..DATA_HEADER_LANES + dc * DATA_ROW_LANES]);
    raw_data.extend_from_slice(&padded.data[g.data_fields_base()..g.data_fields_base() + dfc * REF_LANES]);
    raw_data.extend_from_slice(&padded.data[g.env_base()..g.env_base() + ec * ENV_ROW_LANES]);
    raw_data.extend_from_slice(&padded.data[g.value_slots_base()..g.value_slots_base() + vc * REF_LANES]);
    let data: C6DataTables = decode_data_tables(&raw_data, caps)?;

    let tc = count_lane("helper.tasks", &padded.helper, 3)?;
    verify_inactive_zero("helper.tasks", &padded.helper, HELPER_FIXED_LANES, tc, caps.max_helper_tasks, TASK_LANES)?;
    let helper: C6HelperTables = decode_helper_tables(&padded.helper[..HELPER_FIXED_LANES + tc * TASK_LANES], caps)?;
    Ok(C6OwnedState { root, frames, data, helper })
}

fn text_pair_len(pair: &(String, String)) -> usize { 2 + pair.0.len() + pair.1.len() }
fn digest_len(digest: &DigestC) -> usize {
    match digest {
        DigestC::Unselected => 1,
        DigestC::Selected { format, bytes } => 1 + text_pair_len(format) + 1 + bytes.len(),
    }
}
fn identity_encoded_len(id: &IdentityC) -> usize {
    let opt_pair = match &id.field_id { None => 1, Some(pair) => 1 + text_pair_len(pair) };
    3 + text_pair_len(&id.layout_version) + digest_len(&id.layout_digest)
        + digest_len(&id.bounds_digest) + opt_pair + digest_len(&id.program_membership_digest)
}

/// Lane index of `bounds.frames` (field 16 of the 22-lane `BoundsC`) inside an
/// encoded `META` section, verified against the decoded metadata value.
pub fn meta_admission_bounds_frames_offset(meta: &C6HostMetadata, meta_lanes: &[u128]) -> Result<usize, C6PushStepError> {
    let offset = META_HEADER_LANES + identity_encoded_len(&meta.identity) + 16;
    let lane_val = *meta_lanes.get(offset).ok_or(C6PushStepError::MetadataBoundsOffsetOutOfRange { offset, len: meta_lanes.len() })?;
    let decoded_val = meta.bounds.frames as u128;
    if lane_val != decoded_val {
        return Err(C6PushStepError::MetadataBoundsFramesDisagreement { lane_val, decoded_val });
    }
    Ok(offset)
}

/// Full invariant host prefix (`META`, `PROG`, `CTX`) under the caller's
/// original retained config. Digests remain opaque; no StepInv or admission
/// claim is made by carrying these lanes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct C6PushHostPrefixLanes {
    pub meta: Vec<u128>,
    pub prog: Vec<u128>,
    pub ctx: [u128; CTX_LANES],
    pub meta_bounds_frames_offset: usize,
}

impl C6PushHostPrefixLanes {
    pub fn from_host_snapshot(host: &HostSnapshot, cfg: &C6HostConfig, limits: &C6HostLimits) -> Result<Self, C6PushStepError> {
        if host.admission.layout != cfg.layout {
            return Err(C6PushStepError::ConfigMismatch { field: "layout" });
        }
        let ctx = cfg.state_context(limits);
        let caps = ctx.caps();
        let metadata = C6HostMetadata::from_host(host);
        let meta = encode_metadata(&metadata, limits)?;
        if decode_metadata(&meta, limits)? != metadata {
            return Err(C6PushStepError::Backend("metadata roundtrip disagreed".into()));
        }
        let offset = meta_admission_bounds_frames_offset(&metadata, &meta)?;
        let prog = encode_program(&host.admission.view, &caps.phys)?;
        if decode_program(&prog, &caps.phys)? != host.admission.view {
            return Err(C6PushStepError::Backend("program roundtrip disagreed".into()));
        }
        let ctx_lanes = encode_context(cfg.ext, cfg.budgets);
        let (ext, budgets) = decode_context(&ctx_lanes)?;
        if ext != cfg.ext || budgets != cfg.budgets {
            return Err(C6PushStepError::Backend("context roundtrip disagreed".into()));
        }
        Ok(Self { meta, prog, ctx: ctx_lanes, meta_bounds_frames_offset: offset })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct C6PushStepWitness {
    enable: bool,
    selection: PushSelection,
    host_prefix: C6PushHostPrefixLanes,
    pre_state: C6PaddedStateLanes,
    post_state: C6PaddedStateLanes,
}

impl C6PushStepWitness {
    pub const fn family(&self) -> &'static str { c6_push_family(self.selection) }

    /// Statement order: `enable, selector_is_accept, host_schema,
    /// carrier_version, pre(META,PROG,CTX,state), post(META,PROG,CTX,state)`.
    pub fn to_statement_frs(&self) -> Vec<Fr> {
        let mut out = Vec::new();
        out.push(Fr::from(self.enable));
        out.push(Fr::from(matches!(self.selection, PushSelection::Accept)));
        out.push(Fr::from(C6_HOST_SCHEMA));
        out.push(Fr::from(C6_PADDED_CARRIER_VERSION));
        for lane in self.host_prefix.meta.iter().chain(&self.host_prefix.prog)
            .chain(self.host_prefix.ctx.iter()).chain(self.pre_state.to_flat_lanes().iter()) {
            out.push(Fr::from(*lane));
        }
        for lane in self.host_prefix.meta.iter().chain(&self.host_prefix.prog)
            .chain(self.host_prefix.ctx.iter()).chain(self.post_state.to_flat_lanes().iter()) {
            out.push(Fr::from(*lane));
        }
        out
    }
}

fn geometry_from_config(cfg: &C6HostConfig, limits: &C6HostLimits) -> Result<(C6StateCaps, C6StateLimits, C6PaddedCarrierGeometry), C6PushStepError> {
    let ctx = cfg.state_context(limits);
    let caps = *ctx.caps();
    let state_limits = C6StateLimits { max_total_lanes: limits.max_state_lanes };
    let geometry = C6PaddedCarrierGeometry::checked_new(caps, state_limits)?;
    Ok((caps, state_limits, geometry))
}

/// Independent-post route: `pre` and `post` are independently supplied host
/// snapshots. No recorder runs here, so a wrong post reaches the circuit and
/// is rejected by `check_c6_push_step_circuit`, never by the producer.
pub fn witness_from_host_pair(
    enable: bool, pre: &HostSnapshot, post: &HostSnapshot, cfg: &C6HostConfig, limits: &C6HostLimits,
) -> Result<C6PushStepWitness, C6PushStepError> {
    let selection = push_selection(pre)?;
    let (caps, state_limits, _g) = geometry_from_config(cfg, limits)?;
    let pre_prefix = C6PushHostPrefixLanes::from_host_snapshot(pre, cfg, limits)?;
    let post_prefix = C6PushHostPrefixLanes::from_host_snapshot(post, cfg, limits)?;
    if pre_prefix.meta != post_prefix.meta { return Err(C6PushStepError::HostPrefixMismatch { section: "meta" }); }
    if pre_prefix.prog != post_prefix.prog { return Err(C6PushStepError::HostPrefixMismatch { section: "prog" }); }
    if pre_prefix.ctx != post_prefix.ctx { return Err(C6PushStepError::HostPrefixMismatch { section: "ctx" }); }
    let pre_state = encode_owned_state_to_padded(&C6OwnedState::from_image(&pre.image), caps, state_limits)?;
    let post_state = encode_owned_state_to_padded(&C6OwnedState::from_image(&post.image), caps, state_limits)?;
    Ok(C6PushStepWitness { enable, selection, host_prefix: pre_prefix, pre_state, post_state })
}

/// Owner-produced route: runs exactly one actual admitted+mutable recorder
/// (`record_a135_push_step` or `record_a140_refusal_step`) and builds the
/// witness from its public `record().before` / `record().after` snapshots.
/// Recorder refusal is returned as-is (producer rejection); it is never
/// converted into, or masked by, a circuit result.
pub fn witness_from_owner_step(
    admitted: &Admitted, image: &mut MachineImage, selection: PushSelection,
    cfg: &C6HostConfig, limits: &C6HostLimits,
) -> Result<C6PushStepWitness, C6PushStepError> {
    let (caps, _sl, _g) = geometry_from_config(cfg, limits)?;
    let map_rec = |e: mori_core::sherlock::helper_machine::c6_phys::C6RecordError| -> C6PushStepError {
        match e.phys_error() { Some(p) => C6PushStepError::Phys(p.clone()), None => C6PushStepError::Backend("owner recorder refused the step".into()) }
    };
    let observed = match selection {
        PushSelection::Accept => record_a135_push_step(admitted, image, &caps.phys).map_err(map_rec)?,
        PushSelection::Refuse => record_a140_refusal_step(admitted, image, &caps.phys).map_err(map_rec)?,
    };
    if observed.selection() != selection {
        return Err(C6PushStepError::Backend("recorder selection disagreed with request".into()));
    }
    let record = observed.record();
    witness_from_host_pair(true, &record.before, &record.after, cfg, limits)
}

#[derive(Clone, Debug)]
pub struct C6PushStepInputs {
    pub enable: Operand,
    pub selector_is_accept: Operand,
    pub host_schema: Operand,
    pub carrier_version: Operand,
    pub geometry: C6PaddedCarrierGeometry,
    pub meta_bounds_frames_offset: usize,
    /// UNAUTHENTICATED CIRCUIT PARAMETER: the builder-chosen limits
    /// define only the structural META relation of this circuit.
    /// They are not verified against any retained host or verifier
    /// authority by this struct, the builder, or the checker.
    pub meta_limits: C6HostLimits,
    pub pre_meta: Vec<Operand>,
    pub pre_prog: Vec<Operand>,
    pub pre_ctx: [Operand; CTX_LANES],
    pub pre_image: Vec<Operand>,
    pub pre_data: Vec<Operand>,
    pub pre_helper: Vec<Operand>,
    pub post_meta: Vec<Operand>,
    pub post_prog: Vec<Operand>,
    pub post_ctx: [Operand; CTX_LANES],
    pub post_image: Vec<Operand>,
    pub post_data: Vec<Operand>,
    pub post_helper: Vec<Operand>,
}

#[derive(Clone, Debug)]
pub struct C6PushStepBlock {
    pub lt_stack_limit: Operand,
    pub post_depth: Operand,
    pub instructions: Vec<Instruction>,
}

struct IrEmitter<'a> { scope: &'a str, next_temp: usize, instructions: Vec<Instruction> }

impl<'a> IrEmitter<'a> {
    fn new(scope: &'a str) -> Self { Self { scope, next_temp: 0, instructions: Vec::new() } }
    fn temp(&mut self) -> Identifier {
        let id = Identifier(format!("%{}.local.temp.{}", self.scope, self.next_temp));
        self.next_temp += 1;
        id
    }
    fn imm(val: u128) -> Operand { Operand::Immediate(Fr::from(val)) }
    fn constrain_boolean(&mut self, val: Operand) { self.instructions.push(Instruction::ConstrainToBoolean { val }); }
    fn constrain_bits(&mut self, val: Operand, bits: u32) { self.instructions.push(Instruction::ConstrainBits { val, bits }); }
    fn constrain_eq(&mut self, a: Operand, b: Operand) { self.instructions.push(Instruction::ConstrainEq { a, b }); }
    fn test_eq(&mut self, a: Operand, b: Operand) -> Operand {
        let out = self.temp();
        self.instructions.push(Instruction::TestEq { a, b, output: out.clone() });
        Operand::Variable(out)
    }
    fn less_than_128(&mut self, a: Operand, b: Operand) -> Operand {
        let out = self.temp();
        self.instructions.push(Instruction::LessThan { a, b, bits: LANE_BITS, output: out.clone() });
        Operand::Variable(out)
    }
    fn add(&mut self, a: Operand, b: Operand) -> Operand {
        let out = self.temp();
        self.instructions.push(Instruction::Add { a, b, output: out.clone() });
        Operand::Variable(out)
    }
    fn mul(&mut self, a: Operand, b: Operand) -> Operand {
        let out = self.temp();
        self.instructions.push(Instruction::Mul { a, b, output: out.clone() });
        Operand::Variable(out)
    }
    fn not(&mut self, a: Operand) -> Operand {
        let out = self.temp();
        self.instructions.push(Instruction::Not { a, output: out.clone() });
        Operand::Variable(out)
    }
    fn cond_select(&mut self, bit: Operand, a: Operand, b: Operand) -> Operand {
        let out = self.temp();
        self.instructions.push(Instruction::CondSelect { bit, a, b, output: out.clone() });
        Operand::Variable(out)
    }
    fn assert_true(&mut self, cond: Operand) { self.instructions.push(Instruction::Assert { cond }); }
    fn gated_constrain_eq(&mut self, gate: Operand, a: Operand, b: Operand) {
        let selected = self.cond_select(gate, a, b.clone());
        self.constrain_eq(selected, b);
    }
    fn gated_assert(&mut self, gate: Operand, cond: Operand) {
        let effective = self.cond_select(gate, cond, Self::imm(1));
        self.assert_true(effective);
    }
    /// Strict index bound: `value < cap`.
    fn gated_lt_index(&mut self, gate: Operand, val: Operand, cap: usize) {
        let lt = self.less_than_128(val, Self::imm(cap as u128));
        self.gated_assert(gate, lt);
    }
    /// Inclusive count/length/end bound: `value < cap + 1`, checked host-side.
    fn gated_lt_count(&mut self, gate: Operand, val: Operand, cap: usize) {
        debug_assert!(cap < usize::MAX);
        let lt = self.less_than_128(val, Self::imm((cap as u128) + 1));
        self.gated_assert(gate, lt);
    }
    fn gated_boolean(&mut self, gate: Operand, val: Operand) {
        let checked = self.cond_select(gate, val, Self::imm(0));
        self.constrain_boolean(checked);
    }
    /// `EnvRootC` lanes: present is boolean; absent => index 0; present => index < env cap.
    fn gated_validate_env(&mut self, gate: Operand, present: Operand, index: Operand, env_cap: usize) {
        self.gated_boolean(gate.clone(), present.clone());
        let is_absent = self.test_eq(present.clone(), Self::imm(0));
        let absent_gate = self.mul(gate.clone(), is_absent);
        self.gated_constrain_eq(absent_gate, index.clone(), Self::imm(0));
        let is_present = self.test_eq(present, Self::imm(1));
        let present_gate = self.mul(gate, is_present);
        self.gated_lt_index(present_gate, index, env_cap);
    }
    fn gated_validate_ref(&mut self, gate: Operand, lanes: &[Operand], at: usize, caps: &C6PhysCaps) {
        self.gated_boolean(gate.clone(), lanes[at].clone());
        let is_data = self.test_eq(lanes[at].clone(), Self::imm(1));
        let cap = self.cond_select(is_data, Self::imm(caps.max_data_nodes as u128), Self::imm(caps.max_constants as u128));
        let lt = self.less_than_128(lanes[at + 1].clone(), cap);
        self.gated_assert(gate, lt);
    }
    fn less_than_129(&mut self, a: Operand, b: Operand) -> Operand {
        let out = self.temp();
        self.instructions.push(Instruction::LessThan { a, b, bits: 129, output: out.clone() });
        Operand::Variable(out)
    }
    /// Inclusive slice-end bound: `end < table_cap + 1`, where `end`
    /// is the field sum of two unconditional 128-bit lanes and
    /// therefore lies in `[0, 2^129 - 2]`, below the BLS12-381
    /// modulus. The 129-bit comparator is exact; a 128-bit
    /// comparator on this derived sum would be out of width on
    /// gate-0, rejecting, and non-matching paths.
    fn gated_lt_end(&mut self, gate: Operand, end: Operand, table_cap: usize) {
        debug_assert!(table_cap < usize::MAX);
        let lt = self.less_than_129(end, Self::imm((table_cap as u128) + 1));
        self.gated_assert(gate, lt);
    }
    fn gated_validate_slice(&mut self, gate: Operand, lanes: &[Operand], at: usize, table_cap: usize, max_len: Option<usize>) {
        if let Some(max_len) = max_len { self.gated_lt_count(gate.clone(), lanes[at + 1].clone(), max_len); }
        let end = self.add(lanes[at].clone(), lanes[at + 1].clone());
        self.gated_lt_end(gate, end, table_cap);
    }
    /// Full structural validation of one 13-lane `FrameRowC` under `gate`,
    /// mirroring Core `frame_caps` field by field, including the builtin
    /// TypeId cap `max(max_types, 4)` and per-variant zero padding.
    fn gated_validate_frame(&mut self, gate: Operand, frame: &[Operand], caps: &C6PhysCaps) {
        debug_assert_eq!(frame.len(), FRAME_LANES);
        let builtin_types = caps.max_types.max(4);
        let mut tag_bits = Vec::with_capacity(FRAME_VARIANT_COUNT);
        let mut tag_sum = Self::imm(0);
        for tag in 0..FRAME_VARIANT_COUNT {
            let is_tag = self.test_eq(frame[0].clone(), Self::imm(tag as u128));
            tag_sum = self.add(tag_sum, is_tag.clone());
            tag_bits.push(is_tag);
        }
        self.gated_constrain_eq(gate.clone(), tag_sum, Self::imm(1));
        // 0 LetF
        let gt = self.mul(gate.clone(), tag_bits[0].clone());
        self.gated_lt_index(gt.clone(), frame[1].clone(), caps.max_names);
        self.gated_lt_index(gt.clone(), frame[2].clone(), caps.max_expressions);
        self.gated_validate_env(gt.clone(), frame[3].clone(), frame[4].clone(), caps.max_env_cells);
        self.gated_lt_index(gt, frame[5].clone(), caps.max_expressions);
        // 1 LeftF
        let gt = self.mul(gate.clone(), tag_bits[1].clone());
        self.gated_lt_index(gt.clone(), frame[1].clone(), 7);
        self.gated_lt_index(gt.clone(), frame[2].clone(), caps.max_expressions);
        self.gated_validate_env(gt.clone(), frame[3].clone(), frame[4].clone(), caps.max_env_cells);
        self.gated_lt_index(gt, frame[5].clone(), caps.max_expressions);
        // 2 RightF
        let gt = self.mul(gate.clone(), tag_bits[2].clone());
        self.gated_lt_index(gt.clone(), frame[1].clone(), 7);
        self.gated_validate_ref(gt.clone(), frame, 2, caps);
        self.gated_lt_index(gt, frame[4].clone(), caps.max_expressions);
        // 3 UnaryF
        let gt = self.mul(gate.clone(), tag_bits[3].clone());
        self.gated_lt_index(gt.clone(), frame[1].clone(), 2);
        self.gated_lt_index(gt, frame[2].clone(), caps.max_expressions);
        // 4 IfF
        let gt = self.mul(gate.clone(), tag_bits[4].clone());
        self.gated_lt_index(gt.clone(), frame[1].clone(), caps.max_expressions);
        self.gated_lt_index(gt.clone(), frame[2].clone(), caps.max_expressions);
        self.gated_validate_env(gt.clone(), frame[3].clone(), frame[4].clone(), caps.max_env_cells);
        self.gated_lt_index(gt, frame[5].clone(), caps.max_expressions);
        // 5 MatchF
        let gt = self.mul(gate.clone(), tag_bits[5].clone());
        self.gated_validate_slice(gt.clone(), frame, 1, caps.max_arms, None);
        self.gated_validate_env(gt.clone(), frame[3].clone(), frame[4].clone(), caps.max_env_cells);
        self.gated_lt_index(gt, frame[5].clone(), caps.max_expressions);
        // 6 ArgsF
        let gt = self.mul(gate.clone(), tag_bits[6].clone());
        self.gated_lt_index(gt.clone(), frame[1].clone(), caps.max_names);
        self.gated_validate_slice(gt.clone(), frame, 2, caps.max_value_slots, Some(caps.max_arity));
        self.gated_validate_slice(gt.clone(), frame, 4, caps.max_children, Some(caps.max_arity));
        self.gated_validate_env(gt.clone(), frame[6].clone(), frame[7].clone(), caps.max_env_cells);
        self.gated_lt_index(gt, frame[8].clone(), caps.max_expressions);
        // 7 ReturnF (builtin TypeId cap)
        let gt = self.mul(gate.clone(), tag_bits[7].clone());
        self.gated_lt_index(gt.clone(), frame[1].clone(), builtin_types);
        self.gated_validate_env(gt.clone(), frame[2].clone(), frame[3].clone(), caps.max_env_cells);
        self.gated_lt_index(gt, frame[4].clone(), caps.max_expressions);
        // 8 ScopeF
        let gt = self.mul(gate.clone(), tag_bits[8].clone());
        self.gated_validate_env(gt.clone(), frame[1].clone(), frame[2].clone(), caps.max_env_cells);
        self.gated_lt_index(gt, frame[3].clone(), caps.max_expressions);
        // 9 ConstructorFieldsF (builtin TypeId caps)
        let gt = self.mul(gate.clone(), tag_bits[9].clone());
        self.gated_lt_index(gt.clone(), frame[1].clone(), builtin_types);
        self.gated_lt_index(gt.clone(), frame[2].clone(), caps.max_constructors);
        self.gated_lt_index(gt.clone(), frame[3].clone(), caps.max_arity);
        self.gated_lt_index(gt.clone(), frame[4].clone(), builtin_types);
        self.gated_boolean(gt.clone(), frame[5].clone());
        self.gated_validate_slice(gt.clone(), frame, 6, caps.max_value_slots, Some(caps.max_arity));
        self.gated_validate_slice(gt.clone(), frame, 8, caps.max_children, Some(caps.max_arity));
        self.gated_validate_env(gt.clone(), frame[10].clone(), frame[11].clone(), caps.max_env_cells);
        self.gated_lt_index(gt, frame[12].clone(), caps.max_expressions);
        // Zero padding of unused lanes per variant.
        const USED: [usize; FRAME_VARIANT_COUNT] = [6, 6, 5, 3, 6, 6, 9, 5, 4, 13];
        for lane in 1..FRAME_LANES {
            let mut must_zero = Self::imm(0);
            let mut any = false;
            for (tag, used) in USED.iter().enumerate() {
                if lane >= *used { must_zero = self.add(must_zero, tag_bits[tag].clone()); any = true; }
            }
            if any {
                let pad_gate = self.mul(gate.clone(), must_zero);
                self.gated_constrain_eq(pad_gate, frame[lane].clone(), Self::imm(0));
            }
        }
    }
}

pub fn emit_c6_push_step_block(inputs: &C6PushStepInputs, scope: &str) -> Result<C6PushStepBlock, C6PushStepError> {
    if scope.trim().is_empty() { return Err(C6PushStepError::EmptyScope); }
    if (LANE_BITS as usize) >= FR_BITS { return Err(C6PushStepError::InvalidBitWidth { bits: LANE_BITS }); }
    let g = &inputs.geometry;
    let caps = &g.caps.phys;
    for (section, actual, expected) in [("image", inputs.pre_image.len(), g.image_total_lanes),
        ("image", inputs.post_image.len(), g.image_total_lanes), ("data", inputs.pre_data.len(), g.data_total_lanes),
        ("data", inputs.post_data.len(), g.data_total_lanes), ("helper", inputs.pre_helper.len(), g.helper_total_lanes),
        ("helper", inputs.post_helper.len(), g.helper_total_lanes)] {
        if actual != expected { return Err(C6PushStepError::LengthMismatch { section, expected, actual }); }
    }
    if inputs.pre_meta.len() < META_HEADER_LANES || inputs.pre_meta.len() != inputs.post_meta.len() {
        return Err(C6PushStepError::LengthMismatch {
            section: "meta",
            expected: META_HEADER_LANES.max(inputs.post_meta.len()),
            actual: inputs.pre_meta.len(),
        });
    }
    if inputs.pre_prog.len() < PROGRAM_HEADER_LANES || inputs.pre_prog.len() != inputs.post_prog.len() {
        return Err(C6PushStepError::LengthMismatch {
            section: "prog",
            expected: PROGRAM_HEADER_LANES.max(inputs.post_prog.len()),
            actual: inputs.pre_prog.len(),
        });
    }
    if inputs.meta_bounds_frames_offset >= inputs.pre_meta.len() {
        return Err(C6PushStepError::MetadataBoundsOffsetOutOfRange { offset: inputs.meta_bounds_frames_offset, len: inputs.pre_meta.len() });
    }

    let mut em = IrEmitter::new(scope);
    let enable = inputs.enable.clone();
    let selector = inputs.selector_is_accept.clone();
    em.constrain_boolean(enable.clone());
    em.constrain_boolean(selector.clone());
    let disabled = em.not(enable.clone());
    em.constrain_eq(inputs.host_schema.clone(), IrEmitter::imm(C6_HOST_SCHEMA));
    em.constrain_eq(inputs.carrier_version.clone(), IrEmitter::imm(C6_PADDED_CARRIER_VERSION));

    // Host prefix: headers gated, every lane range-checked and pre == post.
    for (lanes_pre, lanes_post, tag) in [(&inputs.pre_meta, &inputs.post_meta, META_SECTION_TAG),
        (&inputs.pre_prog, &inputs.post_prog, mori_core::sherlock::helper_machine::c6_phys_program::PROGRAM_SECTION_TAG)] {
        em.gated_constrain_eq(enable.clone(), lanes_pre[0].clone(), IrEmitter::imm(mori_core::sherlock::helper_machine::c6_phys::C6_PHYS_MAGIC));
        em.gated_constrain_eq(enable.clone(), lanes_pre[1].clone(), IrEmitter::imm(mori_core::sherlock::helper_machine::c6_phys::C6_PHYS_VERSION));
        em.gated_constrain_eq(enable.clone(), lanes_pre[2].clone(), IrEmitter::imm(tag));
        for (a, b) in lanes_pre.iter().zip(lanes_post.iter()) {
            em.constrain_bits(a.clone(), LANE_BITS);
            em.constrain_bits(b.clone(), LANE_BITS);
            em.constrain_eq(b.clone(), a.clone());
        }
    }
    em.gated_constrain_eq(enable.clone(), inputs.pre_ctx[0].clone(), IrEmitter::imm(mori_core::sherlock::helper_machine::c6_phys::C6_PHYS_MAGIC));
    em.gated_constrain_eq(enable.clone(), inputs.pre_ctx[1].clone(), IrEmitter::imm(mori_core::sherlock::helper_machine::c6_phys::C6_PHYS_VERSION));
    em.gated_constrain_eq(enable.clone(), inputs.pre_ctx[2].clone(), IrEmitter::imm(CTX_SECTION_TAG));
    for (a, b) in inputs.pre_ctx.iter().zip(inputs.post_ctx.iter()) {
        em.constrain_bits(a.clone(), LANE_BITS);
        em.constrain_bits(b.clone(), LANE_BITS);
        em.constrain_eq(b.clone(), a.clone());
    }

    // State lanes: ranges; DATA and HELPER fully preserved.
    for lane in inputs.pre_image.iter().chain(inputs.post_image.iter()) { em.constrain_bits(lane.clone(), LANE_BITS); }
    for (a, b) in inputs.pre_data.iter().zip(inputs.post_data.iter()) {
        em.constrain_bits(a.clone(), LANE_BITS);
        em.constrain_bits(b.clone(), LANE_BITS);
        em.constrain_eq(b.clone(), a.clone());
    }
    for (a, b) in inputs.pre_helper.iter().zip(inputs.post_helper.iter()) {
        em.constrain_bits(a.clone(), LANE_BITS);
        em.constrain_bits(b.clone(), LANE_BITS);
        em.constrain_eq(b.clone(), a.clone());
    }

    // Section headers.
    em.gated_constrain_eq(enable.clone(), inputs.pre_image[L_MAGIC].clone(), IrEmitter::imm(mori_core::sherlock::helper_machine::c6_phys::C6_PHYS_MAGIC));
    em.gated_constrain_eq(enable.clone(), inputs.pre_image[L_VERSION].clone(), IrEmitter::imm(mori_core::sherlock::helper_machine::c6_phys::C6_PHYS_VERSION));
    for lane in 0..2 {
        let expected = IrEmitter::imm(if lane == 0 {
            mori_core::sherlock::helper_machine::c6_phys::C6_PHYS_MAGIC
        } else {
            mori_core::sherlock::helper_machine::c6_phys::C6_PHYS_VERSION
        });
        em.gated_constrain_eq(enable.clone(), inputs.pre_data[lane].clone(), expected.clone());
        em.gated_constrain_eq(enable.clone(), inputs.pre_helper[lane].clone(), expected);
    }
    em.gated_constrain_eq(enable.clone(), inputs.pre_data[2].clone(), IrEmitter::imm(DATA_SECTION_TAG));
    em.gated_constrain_eq(enable.clone(), inputs.pre_helper[2].clone(), IrEmitter::imm(HELPER_SECTION_TAG));

    // Push selection preconditions in owner priority order (P1 cannot apply
    // once control is ScheduleNext; helper/result/tasks are checked before it).
    for off in 0..3 {
        em.gated_constrain_eq(enable.clone(), inputs.pre_image[L_OUTCOME + off].clone(), IrEmitter::imm(0));
    }
    let has_fuel = em.less_than_128(IrEmitter::imm(0), inputs.pre_image[L_REMAINING_WORK].clone());
    em.gated_assert(enable.clone(), has_fuel);
    em.gated_constrain_eq(enable.clone(), inputs.pre_helper[3].clone(), IrEmitter::imm(0));
    for lane in 4..HELPER_FIXED_LANES {
        em.gated_constrain_eq(enable.clone(), inputs.pre_helper[lane].clone(), IrEmitter::imm(0));
    }
    em.gated_constrain_eq(enable.clone(), inputs.pre_image[L_CONTROL].clone(), IrEmitter::imm(CONTROL_TAG_SCHEDULE_NEXT));

    let pending = &inputs.pre_image[L_CONTROL + 1..L_CONTROL + 1 + FRAME_LANES];
    let next_expr = inputs.pre_image[L_CONTROL + 14].clone();
    let sched_env_present = inputs.pre_image[L_CONTROL + 15].clone();
    let sched_env_index = inputs.pre_image[L_CONTROL + 16].clone();
    em.gated_validate_frame(enable.clone(), pending, caps);
    em.gated_lt_index(enable.clone(), next_expr.clone(), caps.max_expressions);
    em.gated_validate_env(enable.clone(), sched_env_present.clone(), sched_env_index.clone(), caps.max_env_cells);
    em.gated_validate_env(enable.clone(), inputs.pre_image[L_ENV_ROOT].clone(), inputs.pre_image[L_ENV_ROOT + 1].clone(), caps.max_env_cells);
    em.gated_lt_index(enable.clone(), inputs.pre_image[L_CONTROL_ORIGIN].clone(), caps.max_expressions);
    em.gated_lt_count(enable.clone(), inputs.pre_image[L_STACK_LIMIT].clone(), caps.max_frames);

    // Branch selection and Accept-only physical checks.
    let depth = inputs.pre_image[L_FRAME_COUNT].clone();
    em.gated_lt_count(enable.clone(), depth.clone(), caps.max_frames);
    let lt_stack_limit = em.less_than_128(depth.clone(), inputs.pre_image[L_STACK_LIMIT].clone());
    em.gated_constrain_eq(enable.clone(), selector, lt_stack_limit.clone());
    let accept_gate = em.mul(enable.clone(), lt_stack_limit.clone());
    let bounds_frames = inputs.pre_meta[inputs.meta_bounds_frames_offset].clone();
    let lt_bounds = em.less_than_128(depth.clone(), bounds_frames);
    em.gated_assert(accept_gate.clone(), lt_bounds);
    let lt_cap = em.less_than_128(depth.clone(), IrEmitter::imm(caps.max_frames as u128));
    em.gated_assert(accept_gate, lt_cap);

    // Post control / origin / env_root.
    let expected_tag = em.cond_select(lt_stack_limit.clone(), IrEmitter::imm(CONTROL_TAG_EVAL), IrEmitter::imm(CONTROL_TAG_ERROR));
    let expected_lane1 = em.cond_select(lt_stack_limit.clone(), next_expr.clone(), IrEmitter::imm(u128::from(PUSH_REFUSE_ERROR_CODE)));
    em.gated_constrain_eq(enable.clone(), inputs.post_image[L_CONTROL].clone(), expected_tag);
    em.gated_constrain_eq(enable.clone(), inputs.post_image[L_CONTROL + 1].clone(), expected_lane1);
    for off in 2..CONTROL_LANES {
        em.gated_constrain_eq(enable.clone(), inputs.post_image[L_CONTROL + off].clone(), IrEmitter::imm(0));
    }
    let expected_origin = em.cond_select(lt_stack_limit.clone(), next_expr, inputs.pre_image[L_CONTROL_ORIGIN].clone());
    em.gated_constrain_eq(enable.clone(), inputs.post_image[L_CONTROL_ORIGIN].clone(), expected_origin);
    let expected_env_present = em.cond_select(lt_stack_limit.clone(), sched_env_present, inputs.pre_image[L_ENV_ROOT].clone());
    let expected_env_index = em.cond_select(lt_stack_limit.clone(), sched_env_index, inputs.pre_image[L_ENV_ROOT + 1].clone());
    em.gated_constrain_eq(enable.clone(), inputs.post_image[L_ENV_ROOT].clone(), expected_env_present);
    em.gated_constrain_eq(enable.clone(), inputs.post_image[L_ENV_ROOT + 1].clone(), expected_env_index);

    // Checked charge: work +1 and reference cursor +1, no u128 wrap.
    let post_rem_plus_1 = em.add(inputs.post_image[L_REMAINING_WORK].clone(), IrEmitter::imm(1));
    em.gated_constrain_eq(enable.clone(), post_rem_plus_1, inputs.pre_image[L_REMAINING_WORK].clone());
    for (pre_lane, post_lane) in [(L_CONSUMED_WORK, L_CONSUMED_WORK), (L_REFERENCE_CURSOR, L_REFERENCE_CURSOR)] {
        let no_overflow = em.less_than_128(inputs.pre_image[pre_lane].clone(), IrEmitter::imm(u128::MAX));
        em.gated_assert(enable.clone(), no_overflow);
        let expected = em.add(inputs.pre_image[pre_lane].clone(), IrEmitter::imm(1));
        em.gated_constrain_eq(enable.clone(), inputs.post_image[post_lane].clone(), expected);
    }

    // Depth, frame slots, and pre inactive padding.
    let post_depth = em.add(depth.clone(), lt_stack_limit.clone());
    em.gated_constrain_eq(enable.clone(), inputs.post_image[L_FRAME_COUNT].clone(), post_depth.clone());
    for slot in 0..caps.max_frames {
        let slot_imm = IrEmitter::imm(slot as u128);
        let base = g.frame_slot_offset(slot);
        let slot_active = em.less_than_128(slot_imm.clone(), depth.clone());
        let slot_inactive = em.not(slot_active);
        let pad_gate = em.mul(enable.clone(), slot_inactive);
        for off in 0..FRAME_LANES {
            em.gated_constrain_eq(pad_gate.clone(), inputs.pre_image[base + off].clone(), IrEmitter::imm(0));
        }
        let is_target = em.test_eq(depth.clone(), slot_imm);
        let write_pending = em.mul(lt_stack_limit.clone(), is_target);
        for off in 0..FRAME_LANES {
            let expected = em.cond_select(write_pending.clone(), pending[off].clone(), inputs.pre_image[base + off].clone());
            em.gated_constrain_eq(enable.clone(), inputs.post_image[base + off].clone(), expected);
        }
    }

    // Every unaffected IMAGE header lane is preserved.
    for lane in 0..IMAGE_SECTION_HEADER_LANES {
        let mutable = (L_CONTROL..L_CONTROL + CONTROL_LANES).contains(&lane)
            || lane == L_CONTROL_ORIGIN || lane == L_ENV_ROOT || lane == L_ENV_ROOT + 1
            || lane == L_REMAINING_WORK || lane == L_CONSUMED_WORK || lane == L_REFERENCE_CURSOR
            || lane == L_FRAME_COUNT;
        if !mutable {
            em.gated_constrain_eq(enable.clone(), inputs.post_image[lane].clone(), inputs.pre_image[lane].clone());
        }
    }

    // DATA/HELPER count caps and zero inactive padding (active-row canonicality
    // of these invariant arenas is an explicit caller assumption above).
    let data_counts = [(3usize, caps.max_data_nodes, DATA_HEADER_LANES, DATA_ROW_LANES, "data"),
        (4, g.caps.max_data_field_slots, g.data_fields_base(), REF_LANES, "fields"),
        (5, caps.max_env_cells, g.env_base(), ENV_ROW_LANES, "env"),
        (6, caps.max_value_slots, g.value_slots_base(), REF_LANES, "values")];
    for (count_lane, cap, base, width, _name) in data_counts {
        let count = inputs.pre_data[count_lane].clone();
        em.gated_lt_count(enable.clone(), count.clone(), cap);
        for slot in 0..cap {
            let active = em.less_than_128(IrEmitter::imm(slot as u128), count.clone());
            let inactive = em.not(active);
            let pad_gate = em.mul(enable.clone(), inactive);
            for off in 0..width {
                em.gated_constrain_eq(pad_gate.clone(), inputs.pre_data[base + slot * width + off].clone(), IrEmitter::imm(0));
            }
        }
    }
    // DATA active-row canonicality (OPEN-1 DATA part). `pre_data` lanes are
    // already 128-bit ranged above and post lanes are constrained equal to
    // them, so this binds both copies. Gate is the same `enable`: disabled
    // steps enforce no canonicality beyond the unconditional lane ranges.
    let data_canonical = emit_c6_data_canonical_block(
        &C6DataCanonicalInputs { gate: enable.clone(), data: inputs.pre_data.clone(), caps: g.caps },
        scope,
    )
    .map_err(|e| C6PushStepError::Backend(e.to_string()))?;
    em.instructions.extend(data_canonical.instructions);
    // META canonicality (whole pre/post vectors). Parent already ranges
    // every META lane and constrains post == pre unconditionally; this
    // gated block proves the variable-length codec domain on both
    // copies and ties meta_bounds_frames_offset to admission
    // BoundsC.frames (index 16), not an arbitrary in-range lane.
    // `inputs.meta_limits` is an UNAUTHENTICATED CIRCUIT PARAMETER:
    // it defines only this structural relation and is not bound to
    // the retained host limits that produced any witness.
    let meta_canonical = emit_c6_meta_canonical_block(
        &C6MetaCanonicalInputs {
            gate: enable.clone(),
            pre_meta: inputs.pre_meta.clone(),
            post_meta: inputs.post_meta.clone(),
            limits: inputs.meta_limits,
            meta_bounds_frames_offset: inputs.meta_bounds_frames_offset,
        },
        scope,
    )
    .map_err(|e| C6PushStepError::Backend(e.to_string()))?;
    em.instructions.extend(meta_canonical.instructions);
    // PROG/CTX canonicality (pre-only). Parent already ranges every
    // PROG and CTX lane and constrains post == pre unconditionally,
    // so this binds both copies. Gate is the same `enable`: disabled
    // steps enforce no canonicality beyond the unconditional lane
    // ranges. `g.caps` is the geometry caps value already derived
    // by the circuit builder's caller-retained config chain; this
    // call does not authenticate that derivation.
    let progctx_canonical = emit_c6_program_context_canonical_block(
        &C6ProgramContextCanonicalInputs {
            gate: enable.clone(),
            pre_prog: inputs.pre_prog.clone(),
            pre_ctx: inputs.pre_ctx.clone(),
            caps: g.caps,
        },
        scope,
    )
    .map_err(|e| C6PushStepError::Backend(e.to_string()))?;
    em.instructions.extend(progctx_canonical.instructions);

    let task_count = inputs.pre_helper[3].clone();
    em.gated_lt_count(enable.clone(), task_count.clone(), g.caps.max_helper_tasks);
    for slot in 0..g.caps.max_helper_tasks {
        let active = em.less_than_128(IrEmitter::imm(slot as u128), task_count.clone());
        let inactive = em.not(active);
        let pad_gate = em.mul(enable.clone(), inactive);
        for off in 0..TASK_LANES {
            em.gated_constrain_eq(pad_gate.clone(), inputs.pre_helper[HELPER_FIXED_LANES + slot * TASK_LANES + off].clone(), IrEmitter::imm(0));
        }
    }

    // Disabled step: complete IMAGE equality (other sections already equal).
    for (a, b) in inputs.pre_image.iter().zip(inputs.post_image.iter()) {
        em.gated_constrain_eq(disabled.clone(), b.clone(), a.clone());
    }

    Ok(C6PushStepBlock { lt_stack_limit, post_depth, instructions: em.instructions })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C6PushStatementBindingMode { LocalPublic, LedgerCall }

#[derive(Clone, Debug)]
pub struct C6PushStepCircuit {
    mode: C6PushStatementBindingMode,
    geometry: C6PaddedCarrierGeometry,
    meta_len: usize,
    prog_len: usize,
    meta_bounds_frames_offset: usize,
    meta_limits: C6HostLimits,
    statement_lane_count: usize,
    ir: IrSource,
    serialized_zkir: String,
}

fn make_typed_identifier(id: Identifier) -> Result<TypedIdentifier, C6PushStepError> {
    serde_json::from_value(json!({ "name": id, "type": IrType::Native }))
        .map_err(|e| C6PushStepError::Backend(format!("TypedIdentifier serialization failed: {e}")))
}

#[allow(clippy::too_many_arguments)]
fn alloc_lane(
    mode: C6PushStatementBindingMode, scope: &str, label: &str, idx: usize,
    inputs: &mut Vec<TypedIdentifier>, instructions: &mut Vec<Instruction>, transcript: &mut Vec<Operand>,
) -> Result<Operand, C6PushStepError> {
    let prefix = match mode { C6PushStatementBindingMode::LocalPublic => "public", C6PushStatementBindingMode::LedgerCall => "input" };
    let id = Identifier(format!("%{scope}.{prefix}.{label}.{idx}"));
    match mode {
        C6PushStatementBindingMode::LocalPublic => instructions.push(Instruction::PublicInput { guard: None, val_t: IrType::Native, output: id.clone() }),
        C6PushStatementBindingMode::LedgerCall => inputs.push(make_typed_identifier(id.clone())?),
    }
    let op = Operand::Variable(id);
    transcript.push(op.clone());
    Ok(op)
}

pub fn build_c6_push_step_circuit(
    geometry: C6PaddedCarrierGeometry, meta_len: usize, prog_len: usize,
    meta_bounds_frames_offset: usize, meta_limits: C6HostLimits, mode: C6PushStatementBindingMode, scope: &str,
) -> Result<C6PushStepCircuit, C6PushStepError> {
    if scope.trim().is_empty() { return Err(C6PushStepError::EmptyScope); }
    let mut call_inputs = Vec::new();
    let mut instructions = Vec::new();
    let mut transcript = Vec::new();
    let mut alloc = |label: &str, idx: usize, call_inputs: &mut Vec<TypedIdentifier>,
                     instructions: &mut Vec<Instruction>, transcript: &mut Vec<Operand>|
     -> Result<Operand, C6PushStepError> { alloc_lane(mode, scope, label, idx, call_inputs, instructions, transcript) };

    let enable = alloc("enable", 0, &mut call_inputs, &mut instructions, &mut transcript)?;
    let selector_is_accept = alloc("selector_is_accept", 0, &mut call_inputs, &mut instructions, &mut transcript)?;
    let host_schema = alloc("host_schema", 0, &mut call_inputs, &mut instructions, &mut transcript)?;
    let carrier_version = alloc("carrier_version", 0, &mut call_inputs, &mut instructions, &mut transcript)?;
    let mut vec_of = |label: &str, n: usize, ci: &mut Vec<TypedIdentifier>, ins: &mut Vec<Instruction>, tr: &mut Vec<Operand>| -> Result<Vec<Operand>, C6PushStepError> {
        let mut out = Vec::with_capacity(n);
        for i in 0..n { out.push(alloc(label, i, ci, ins, tr)?); }
        Ok(out)
    };
    let pre_meta = vec_of("pre_meta", meta_len, &mut call_inputs, &mut instructions, &mut transcript)?;
    let pre_prog = vec_of("pre_prog", prog_len, &mut call_inputs, &mut instructions, &mut transcript)?;
    let pre_ctx_v = vec_of("pre_ctx", CTX_LANES, &mut call_inputs, &mut instructions, &mut transcript)?;
    let pre_ctx: [Operand; CTX_LANES] = pre_ctx_v.try_into().map_err(|_| C6PushStepError::Backend("pre_ctx conversion failed".into()))?;
    let pre_image = vec_of("pre_image", geometry.image_total_lanes, &mut call_inputs, &mut instructions, &mut transcript)?;
    let pre_data = vec_of("pre_data", geometry.data_total_lanes, &mut call_inputs, &mut instructions, &mut transcript)?;
    let pre_helper = vec_of("pre_helper", geometry.helper_total_lanes, &mut call_inputs, &mut instructions, &mut transcript)?;
    let post_meta = vec_of("post_meta", meta_len, &mut call_inputs, &mut instructions, &mut transcript)?;
    let post_prog = vec_of("post_prog", prog_len, &mut call_inputs, &mut instructions, &mut transcript)?;
    let post_ctx_v = vec_of("post_ctx", CTX_LANES, &mut call_inputs, &mut instructions, &mut transcript)?;
    let post_ctx: [Operand; CTX_LANES] = post_ctx_v.try_into().map_err(|_| C6PushStepError::Backend("post_ctx conversion failed".into()))?;
    let post_image = vec_of("post_image", geometry.image_total_lanes, &mut call_inputs, &mut instructions, &mut transcript)?;
    let post_data = vec_of("post_data", geometry.data_total_lanes, &mut call_inputs, &mut instructions, &mut transcript)?;
    let post_helper = vec_of("post_helper", geometry.helper_total_lanes, &mut call_inputs, &mut instructions, &mut transcript)?;

    let step_inputs = C6PushStepInputs { enable, selector_is_accept, host_schema, carrier_version, geometry,
        meta_bounds_frames_offset, meta_limits, pre_meta, pre_prog, pre_ctx, pre_image, pre_data, pre_helper,
        post_meta, post_prog, post_ctx, post_image: post_image.clone(), post_data: post_data.clone(), post_helper: post_helper.clone() };
    let block = emit_c6_push_step_block(&step_inputs, scope)?;
    instructions.extend(block.instructions);
    let statement_lane_count = transcript.len();
    instructions.push(Instruction::Impact { guard: Operand::Immediate(Fr::from(1u64)), inputs: transcript });

    let outputs = match mode {
        C6PushStatementBindingMode::LocalPublic => Vec::new(),
        C6PushStatementBindingMode::LedgerCall => {
            let mut vals = Vec::with_capacity(geometry.state_total_lanes);
            vals.extend(post_image);
            vals.extend(post_data);
            vals.extend(post_helper);
            instructions.push(Instruction::Output { vals });
            vec![IrType::Native; geometry.state_total_lanes]
        }
    };
    let ir = IrSource { version: IrMinorVersion::V0, inputs: call_inputs, outputs,
        do_communications_commitment: matches!(mode, C6PushStatementBindingMode::LedgerCall),
        instructions: Arc::new(instructions) };
    let mut disk = serde_json::to_value(&ir).map_err(|e| C6PushStepError::Backend(e.to_string()))?;
    disk["version"] = json!({ "major": 3, "minor": 0 });
    let serialized_zkir = serde_json::to_string(&disk).map_err(|e| C6PushStepError::Backend(e.to_string()))?;
    let loaded = IrSource::load(serialized_zkir.as_bytes()).map_err(|e| C6PushStepError::Backend(e.to_string()))?;
    if loaded.instructions.len() > (1usize << VERIFIER_MAX_DEGREE) {
        return Err(C6PushStepError::Backend("instruction count exceeds verifier degree bound".into()));
    }
    Ok(C6PushStepCircuit { mode, geometry, meta_len, prog_len, meta_bounds_frames_offset, meta_limits, statement_lane_count, ir: loaded, serialized_zkir })
}

/// Non-proving SDK check (`ProofPreimage::check`, i.e. `IrSource::preprocess`)
/// only. No `k()`, keygen, proving, or native claim is made. This
/// helper does not authenticate `circuit.meta_limits`, geometry, or
/// the serialized circuit against any verifier-pinned expectation;
/// `meta_limits` remains an unauthenticated circuit parameter.
pub fn check_c6_push_step_circuit(circuit: &C6PushStepCircuit, witness: &C6PushStepWitness) -> Result<Vec<Option<usize>>, C6PushStepError> {
    if witness.pre_state.geometry != circuit.geometry || witness.post_state.geometry != circuit.geometry {
        return Err(C6PushStepError::Backend("witness geometry does not match circuit".into()));
    }
    if witness.host_prefix.meta.len() != circuit.meta_len {
        return Err(C6PushStepError::LengthMismatch { section: "meta", expected: circuit.meta_len, actual: witness.host_prefix.meta.len() });
    }
    if witness.host_prefix.prog.len() != circuit.prog_len {
        return Err(C6PushStepError::LengthMismatch { section: "prog", expected: circuit.prog_len, actual: witness.host_prefix.prog.len() });
    }
    if witness.host_prefix.meta_bounds_frames_offset != circuit.meta_bounds_frames_offset {
        return Err(C6PushStepError::MetadataBoundsOffsetOutOfRange { offset: witness.host_prefix.meta_bounds_frames_offset, len: circuit.meta_len });
    }
    let statement = witness.to_statement_frs();
    if statement.len() != circuit.statement_lane_count {
        return Err(C6PushStepError::LengthMismatch { section: "statement", expected: circuit.statement_lane_count, actual: statement.len() });
    }
    let preimage = match circuit.mode {
        C6PushStatementBindingMode::LocalPublic => ProofPreimage {
            inputs: Vec::new(), private_transcript: Vec::new(),
            public_transcript_inputs: statement.clone(), public_transcript_outputs: statement,
            binding_input: Fr::from(0u64), communications_commitment: None,
            key_location: KeyLocation(CHECK_KEY_LOCATION.into()),
        },
        C6PushStatementBindingMode::LedgerCall => {
            let randomness = Fr::from(0u64);
            let mut comm_inputs = statement.clone();
            comm_inputs.extend(witness.post_state.to_flat_lanes().iter().map(|&lane| Fr::from(lane)));
            ProofPreimage {
                inputs: statement.clone(), private_transcript: Vec::new(),
                public_transcript_inputs: statement, public_transcript_outputs: Vec::new(),
                binding_input: Fr::from(0u64),
                communications_commitment: Some((transient_commit(&comm_inputs, randomness), randomness)),
                key_location: KeyLocation(CHECK_KEY_LOCATION.into()),
            }
        }
    };
    preimage.check(&circuit.ir).map_err(|e| C6PushStepError::Backend(format!("SDK ProofPreimage::check failed: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mori_core::sherlock::helper_machine::c6_phys::{C6_PHYS_MAGIC, C6PhysCaps};
    use mori_core::sherlock::helper_machine::complete_observation::{
        AdmissionSnapshot, ConstantSnapshot, ExpressionSnapshot, ImageSnapshot, LayoutSnapshot,
        ProgramSnapshot,
    };
    use mori_core::sherlock::helper_machine::shared::adt::{CtorIdC, TypeIdC};
    use mori_core::sherlock::helper_machine::shared::host_implied_prefix::SegBase;
    use mori_core::sherlock::helper_machine::shared::{
        ActiveLengths, AllocCounters, BinaryOp, ControlC, EnvCellC, FinanceInput, FrameRowC,
        GrammarDomain, IdentityC, LayoutCapacitiesC, OutcomeC, RefC, RunIdC, RunIntC, SliceC,
        UnaryOp,
    };

    fn limits() -> C6HostLimits {
        C6HostLimits { max_total_lanes: 1 << 20, max_state_lanes: 1 << 20, max_metadata_bytes: 8192, max_string_bytes: 512, max_width_entries: 64 }
    }
    fn bounds(frames: usize) -> BoundsC {
        let mut b = BoundsC { frames, ..BoundsC::default() };
        b.code_rows = 16; b.function_rows = 8; b.code_child_slots = 16; b.parameter_slots = 16;
        b.pattern_rows = 16; b.match_plan_rows = 16; b.match_arm_rows = 8; b.type_rows = 16;
        b.constructor_rows = 16; b.constructor_field_slots = 16; b.max_arity = 4;
        b.constant_nodes = 16; b.constant_field_slots = 16; b.data_nodes = 2;
        b.data_field_slots = 2; b.environment_cells = 2; b.helper_tasks = 2;
        b.serialization_bytes = 1_000_000;
        b
    }
    fn identity() -> IdentityC {
        IdentityC { profile_id: 2, grammar_domain: GrammarDomain::GenericNominalExtensionReserved,
            finance: FinanceInput::Absent, layout_version: ("c6".into(), "1".into()),
            layout_digest: DigestC::Unselected,
            bounds_digest: DigestC::Selected { format: ("f".into(), "1".into()), bytes: vec![1, 2] },
            field_id: Some(("fid".into(), "".into())), program_membership_digest: DigestC::Unselected }
    }
    fn config(frames: usize) -> C6HostConfig {
        let b = bounds(frames);
        C6HostConfig {
            layout: LayoutSnapshot { layout_version: ("lv".into(), "1".into()), field_id: None, widths: None,
                scalar_layout_id: None, bounds: b,
                capacities: LayoutCapacitiesC { name_rows: 16, name_bytes: 256, occurrence_path_slots: 32, value_slots: 2 },
                boundary_capacities_present: false },
            ext: None,
            budgets: mori_core::sherlock::helper_machine::c6_phys::C6CodecBudgets { type_expr_nodes: 0, template_bytes: 0 },
        }
    }
    fn caps(cfg: &C6HostConfig) -> C6StateCaps { *cfg.state_context(&limits()).caps() }

    fn all_frames() -> [FrameRowC; 10] {
        [
            FrameRowC::LetF { binder: 1, body: 1, saved_env: Some(0), origin: 1 },
            FrameRowC::LeftF { op: BinaryOp::Add, right: 1, saved_env: None, origin: 1 },
            FrameRowC::RightF { op: BinaryOp::Eq, left: RefC::Data(0), origin: 1 },
            FrameRowC::UnaryF { op: UnaryOp::Not, origin: 1 },
            FrameRowC::IfF { then_expr: 1, else_expr: 0, saved_env: Some(1), origin: 1 },
            FrameRowC::MatchF { arms: SliceC { start: 0, len: 1 }, saved_env: None, origin: 1 },
            FrameRowC::ArgsF { function_name: 0, completed: SliceC { start: 0, len: 1 }, rest: SliceC { start: 0, len: 1 }, saved_env: Some(0), origin: 1 },
            FrameRowC::ReturnF { return_type: TypeIdC(3), saved_env: Some(0), origin: 1 },
            FrameRowC::ScopeF { saved_env: None, origin: 1 },
            FrameRowC::ConstructorFieldsF { type_id: TypeIdC(3), ctor_id: CtorIdC(1), index: 1,
                field_type: TypeIdC(2), checked: true, completed: SliceC { start: 0, len: 1 },
                rest: SliceC { start: 1, len: 1 }, saved_env: Some(0), origin: 1 },
        ]
    }

    fn sample_host(pending: FrameRowC, stack_limit: usize) -> (HostSnapshot, C6HostConfig) {
        let cfg = config(3);
        let id = identity();
        let host = HostSnapshot {
            admission: AdmissionSnapshot { identity: id.clone(), bounds: bounds(3),
                view: ProgramSnapshot { entry: 0,
                    expressions: vec![ExpressionSnapshot::Literal { constant: 0 },
                        ExpressionSnapshot::Let { binder: 0, value: 0, body: 0 }],
                    occurrences: Vec::new(), occurrence_paths: Vec::new(), children: Vec::new(),
                    functions: Vec::new(), parameters: Vec::new(), arms: Vec::new(), patterns: Vec::new(),
                    pattern_children: Vec::new(), types: Vec::new(), type_args: Vec::new(),
                    constructors: Vec::new(), ctor_fields: Vec::new(), decl_param_counts: Vec::new(),
                    decl_ctor_counts: Vec::new(), decl_templates: Vec::new(), plan_nodes: Vec::new(),
                    plan_cases: Vec::new(), field_paths: Vec::new(),
                    constants: vec![ConstantSnapshot::U128 { marker_free: true, n: 42 }],
                    constant_fields: Vec::new(), names: vec!["x".into()] },
                layout: cfg.layout.clone() },
            image: ImageSnapshot {
                run_id: RunIdC { identity: id, fuel_input: RunIntC::Nat(64), stack_limit_input: RunIntC::Nat(stack_limit as u128) },
                control: ControlC::ScheduleNext { frame: pending, next: 1, env: Some(1) },
                control_origin: 0, env_root: Some(0), stack_limit, outcome: OutcomeC::Running,
                initial_fuel: 64, remaining_work: 10, consumed_work: 5, helper_work: 2, helper_visits: 1,
                reference_cursor: 7,
                alloc: AllocCounters { data_nodes: 1, data_fields: 1, environment_cells: 2 },
                active: ActiveLengths { data: 1, data_fields: 1, env: 2, value_slots: 1 },
                helper: None, result: None, seg_base: SegBase::genesis_uncut(),
                data: vec![mori_core::sherlock::helper_machine::complete_observation::DataSnapshot::U128 { n: 99 }],
                data_fields: vec![RefC::Const(0)],
                env: vec![EnvCellC { binder: 0, value: RefC::Const(0), previous: None },
                    EnvCellC { binder: 0, value: RefC::Data(0), previous: Some(0) }],
                frames: vec![all_frames()[0]], tasks: Vec::new(), value_slots: vec![RefC::Data(0)] },
        };
        (host, cfg)
    }

    fn expected_post(pre: &HostSnapshot, selection: PushSelection) -> HostSnapshot {
        let mut post = pre.clone();
        let ControlC::ScheduleNext { frame, next, env } = pre.image.control else { panic!("ScheduleNext expected") };
        match selection {
            PushSelection::Accept => {
                post.image.frames.push(frame);
                post.image.control = ControlC::Eval { expr: next };
                post.image.control_origin = next;
                post.image.env_root = env;
            }
            PushSelection::Refuse => post.image.control = ControlC::Error { code: PUSH_REFUSE_ERROR_CODE },
        }
        post.image.remaining_work -= 1;
        post.image.consumed_work += 1;
        post.image.reference_cursor += 1;
        post
    }

    fn circuit_for(w: &C6PushStepWitness, mode: C6PushStatementBindingMode) -> C6PushStepCircuit {
        build_c6_push_step_circuit(w.pre_state.geometry, w.host_prefix.meta.len(), w.host_prefix.prog.len(),
            w.host_prefix.meta_bounds_frames_offset, limits(), mode, "c6_push_test").unwrap()
    }

    #[test]
    fn padded_roundtrip_all_frame_variants_and_nonzero_padding_refusal() {
        let (pre, cfg) = sample_host(all_frames()[9], 2);
        let c = caps(&cfg);
        let lim = C6StateLimits { max_total_lanes: 1 << 20 };
        for frame in all_frames() {
            let mut state = C6OwnedState::from_image(&pre.image);
            state.frames = vec![frame];
            let padded = encode_owned_state_to_padded(&state, c, lim).unwrap();
            assert_eq!(decode_padded_state_to_owned(&padded).unwrap(), state);
        }
        let state = C6OwnedState::from_image(&pre.image);
        let padded = encode_owned_state_to_padded(&state, c, lim).unwrap();
        assert_eq!(padded.image[L_MAGIC], C6_PHYS_MAGIC);
        let mut bad = padded.clone();
        let off = bad.geometry.frame_slot_offset(1);
        bad.image[off + 2] = 77;
        assert!(matches!(decode_padded_state_to_owned(&bad),
            Err(C6PushStepError::NonZeroInactivePadding { section: "image.frames", slot: 1, lane: 2, value: 77 })));
        let mut bad_data = padded;
        let base = bad_data.geometry.env_base();
        bad_data.data[base + 2 * ENV_ROW_LANES] = 1;
        assert!(decode_padded_state_to_owned(&bad_data).is_err());
    }

    #[test]
    fn a135_and_a140_pass_sdk_check_in_both_modes_with_changed_resume_env() {
        // Pending frame is ConstructorFieldsF (all 13 lanes) and ScheduleNext.env
        // Some(1) differs from pre env_root Some(0): Accept must install it.
        let (pre, cfg) = sample_host(all_frames()[9], 2);
        let post = expected_post(&pre, PushSelection::Accept);
        assert_ne!(post.image.env_root, pre.image.env_root);
        assert_eq!(post.image.frames[..1], pre.image.frames[..1], "existing stack preserved");
        let w = witness_from_host_pair(true, &pre, &post, &cfg, &limits()).unwrap();
        assert_eq!(w.selection, PushSelection::Accept);
        for mode in [C6PushStatementBindingMode::LocalPublic, C6PushStatementBindingMode::LedgerCall] {
            let circuit = circuit_for(&w, mode);
            assert!(check_c6_push_step_circuit(&circuit, &w).is_ok(), "{mode:?}");
            let (pre_r, cfg_r) = sample_host(all_frames()[9], 1);
            let post_r = expected_post(&pre_r, PushSelection::Refuse);
            let wr = witness_from_host_pair(true, &pre_r, &post_r, &cfg_r, &limits()).unwrap();
            assert_eq!(wr.selection, PushSelection::Refuse);
            assert!(check_c6_push_step_circuit(&circuit, &wr).is_ok(), "refuse {mode:?}");
            let wd = witness_from_host_pair(false, &pre, &pre, &cfg, &limits()).unwrap();
            assert!(check_c6_push_step_circuit(&circuit, &wd).is_ok(), "disabled {mode:?}");
        }
    }

    #[test]
    fn independent_wrong_post_mutations_are_rejected_by_the_circuit() {
        let (pre, cfg) = sample_host(all_frames()[0], 2);
        let post = expected_post(&pre, PushSelection::Accept);
        let w = witness_from_host_pair(true, &pre, &post, &cfg, &limits()).unwrap();
        let circuit = circuit_for(&w, C6PushStatementBindingMode::LocalPublic);
        assert!(check_c6_push_step_circuit(&circuit, &w).is_ok());

        let mut bad = w.clone();
        bad.selection = PushSelection::Refuse;
        assert!(check_c6_push_step_circuit(&circuit, &bad).is_err(), "selector");

        let mut bad = w.clone();
        let off = bad.post_state.geometry.frame_slot_offset(1);
        bad.post_state.image[off + 2] ^= 1;
        assert!(check_c6_push_step_circuit(&circuit, &bad).is_err(), "appended frame payload");

        let mut bad = w.clone();
        let off = bad.post_state.geometry.frame_slot_offset(0);
        bad.post_state.image[off + 1] += 1;
        assert!(check_c6_push_step_circuit(&circuit, &bad).is_err(), "existing frame changed");

        let mut bad = w.clone();
        bad.post_state.image[L_CONTROL_ORIGIN] = 0;
        assert!(check_c6_push_step_circuit(&circuit, &bad).is_err(), "control origin");

        let mut bad = w.clone();
        bad.post_state.image[L_ENV_ROOT + 1] = 0;
        assert!(check_c6_push_step_circuit(&circuit, &bad).is_err(), "resume env");

        for lane in [L_REMAINING_WORK, L_CONSUMED_WORK, L_REFERENCE_CURSOR] {
            let mut bad = w.clone();
            bad.post_state.image[lane] = bad.pre_state.image[lane];
            assert!(check_c6_push_step_circuit(&circuit, &bad).is_err(), "counter lane {lane}");
        }

        let mut bad = w.clone();
        bad.post_state.data[DATA_HEADER_LANES + 1] += 1;
        assert!(check_c6_push_step_circuit(&circuit, &bad).is_err(), "data arena changed");

        let mut bad = w.clone();
        bad.post_state.helper[4] = 1;
        assert!(check_c6_push_step_circuit(&circuit, &bad).is_err(), "helper arena changed");

        let mut bad = w.clone();
        bad.enable = false;
        assert!(check_c6_push_step_circuit(&circuit, &bad).is_err(), "disabled with changed post");

        let mut bad = w;
        bad.pre_state.image[L_CONSUMED_WORK] = u128::MAX;
        bad.post_state.image[L_CONSUMED_WORK] = 0;
        assert!(check_c6_push_step_circuit(&circuit, &bad).is_err(), "counter overflow");
    }
}
