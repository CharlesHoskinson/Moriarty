//! In-circuit canonicality for the variable-length `sherlock-c6-phys/1`
//! META section (`decode_metadata` in `c6_phys_host`).
//!
//! The accepted domain is exactly the supplied Host4 codec domain placed
//! in the caller-supplied pre/post META lane vectors: fixed header,
//! two complete `IdentityC` values, both distinct 22-lane `BoundsC`
//! blocks, layout version/field/widths/scalar fields, four capacities,
//! boundary bool, and both `RunIntC` values, ending exactly at the last
//! lane. Text uses the Rust `String::from_utf8` domain (non-ASCII and
//! NUL are valid when the byte sequence is valid UTF-8); digest bytes
//! are opaque blobs and are never UTF-8 checked. Digest bytes are not
//! commitments and no ctx_key, hash, profile allowlist, or cross-field
//! equality beyond `decode_metadata` is invented here.
//!
//! Limits authority: `limits` / `meta_limits` is an UNAUTHENTICATED
//! CIRCUIT PARAMETER. The value chosen by the circuit builder defines
//! only the structural relation emitted by this module. It is not
//! authenticated against, derived from, or verified against any
//! retained host, `C6HostConfig`, serialized-circuit, or verifier
//! authority by this module, and neither this module nor the Push
//! `check_c6_push_step_circuit` helper may be described as verifying
//! original retained-limit authenticity. Full Push verifier
//! configuration/geometry/serialized-circuit authentication remains
//! REQUIRED_OPEN. Opaque `DigestC` bytes do not create a trusted
//! ctx_key or hash binding.
//!
//! Gate policy: `gate` is boolean-constrained and every META lane is
//! 128-bit-range-constrained unconditionally. When `gate == 1` every
//! canonicality constraint below is enforced. When `gate == 0` no
//! header, tag, string, count, UTF-8, width, bounds, or exact-end
//! constraint is enforced. Host shape admission (checked lengths,
//! offset range, scope and reserved-operand refusal) happens before
//! any constraint is emitted and is not gated.
//!
//! Bound mode (`C6MetaConfigBinding`): a caller may additionally retain
//! one admission `BoundsC` and one complete original layout. Bound
//! emission runs the identical parser and, gated by the same gate,
//! compares every admission-bounds lane, every layout-bounds lane, the
//! complete remaining layout segment (version pair, optional field_id,
//! raw ordered width lists with duplicates, optional scalar id, four
//! capacities, boundary bool) against those retained values on both
//! passes. This is a deliberately stronger relation than the codec
//! domain above and is a caller-selected family parameter, not codec
//! or admission authentication. Admission identity, run identity, both
//! `RunIntC` payloads, all PROG and all state remain runtime. The
//! binding type's private fields claim no native authority.
//! Parsing: META is variable-length, so lane roles are not fixed
//! offsets. This module emits a lane-by-lane parser whose phase,
//! sub-phase, remaining-byte, remaining-entry, width-list, optional
//! tag, UTF-8, and cumulative-limit registers are field operands
//! advanced once per lane. No host allocation is sized by a lane
//! value; emitted size is linear in the supplied lane-vector lengths.
//! The parser entry for the lane at `meta_bounds_frames_offset` is
//! additionally constrained to be admission-`BoundsC` index 16
//! (`frames`), tying the Push `bounds.frames` operand to that field.
//!
//! Operand-bound discipline (SDK `Instruction::LessThan` is UB if an
//! operand exceeds its declared bit width): a `LessThan` is never fed
//! a raw field difference or an unbounded accumulator.
//! - Every lane is `ConstrainBits(128)`-constrained before `step`, so
//!   every direct lane operand of a 128-bit `LessThan` is `< 2^128` and
//!   every immediate compared at 128 bits below is `< 2^128`;
//!   `usize` bounds use `(usize::MAX as u128) + 1 <= 2^64` on the
//!   supported 64-bit host, and limit-plus-one values are `<= 2^64`.
//! - UTF-8: the continuation offset fed to codepoint arithmetic is
//!   `sel(cont_ok, lane - 0x80, 0)`, hence in `[0, 64)` as an integer
//!   whenever it is selected, and `0` otherwise; the raw field
//!   difference `lane - 0x80` (which is `p - 128` for lane `0`) is
//!   never a `LessThan` operand and never reaches a register. The
//!   candidate codepoint is additionally capped by a 128-bit
//!   comparison against `CP_CAP = 0x200000` (both operands
//!   `< 2^128`) and `sel` to at most `CP_CAP` before it is stored in
//!   `utf_cp` or compared at minimum/maximum/surrogate bounds. Hence
//!   every stored `utf_cp` and every compared codepoint is
//!   `<= CP_CAP < 2^128`, on lead, continuation, completing, invalid,
//!   gate-0, gate-1, accepting, rejecting, and dead/unselected
//!   `utf8_step` evaluations alike. `utf_min` is selected only from
//!   `{0, 0x80, 0x800, 0x10000}` or its previous value, so it is
//!   `< 2^128`. `utf_pending` is selected only from `{0,1,2,3}`, a
//!   previous-minus-one selected only when the previous value is
//!   non-zero by the same `sel`, or its previous value.
//! - Cumulative byte/width sums are saturating bounded accumulators,
//!   not raw sums: `cum_next_bounded` first caps the lane contribution
//!   with a 128-bit `LessThan` against `limit + 1 <= 2^64`, adds it to
//!   a register invariantly in `[0, limit + 1]`, so the raw sum is
//!   `< 2^65 < 2^128`; it then compares that sum at 128 bits and
//!   stores `sel(ok, raw, limit + 1)`. The stored cumulative register
//!   therefore always stays in `[0, limit + 1]`, however many length
//!   or width-count lanes a rejecting or gate-0 trace supplies, and
//!   no 129-bit comparison on an unbounded sum is emitted. The gated
//!   assertion uses the same `ok`: for a lane contribution already
//!   above the limit the capped contribution makes `ok` false, so
//!   gate-1 acceptance is exactly `cum + lane <= limit` as before.
//! - Parser control registers only feed `TestEq`, except `sub`,
//!   which feeds one 128-bit `LessThan` against immediate `19` in the
//!   BoundsC block. Register transition mapping, by induction from
//!   the all-zero initial registers: `phase` is only ever an initial
//!   `0`, a small immediate, its previous value, or `previous + 1`
//!   selected when the previous value equals a specific phase
//!   `< 32`, so `phase in [0, 32]`; `sub` likewise is only `0`, an
//!   immediate in `[0, 6]`, its previous value, or `previous + 1`
//!   selected under a phase/sub condition entered with `sub = 0`
//!   (BoundsC increments `0..=21` then resets to `0` while advancing
//!   `phase`; capacities increment `0..=3`; digest/optional/width
//!   sub increments originate from subs in `{1,2,3,4,5}`), so
//!   `sub in [0, 22]`; `list in [0, 2]`, `hold` is a lane or previous
//!   value compared only by `TestEq`, `rem`/`cnt` are a lane
//!   (`< 2^128`), an immediate, or a minus-one value selected only
//!   on paths entered with a non-zero length/count, and are compared
//!   only by `TestEq`. No other register reaches a `LessThan`.
//! - All remaining `LessThan` sites take either a 128-bit lane, the
//!   bounded `sub`, the bounded codepoint, a saturated cumulative
//!   sum, or a boolean/`TestEq` product, against an immediate
//!   `< 2^128` (or the checked `usize` bound above). `Add`/`Mul`
//!   intermediates that are not selected (raw lead offsets,
//!   `pending - 1` at pending `0`, shifted codepoints before the
//!   cap) are field operations with no width precondition and are
//!   discarded by the bounding `CondSelect` before any comparison
//!   or register store.
//!
//! Temporary-namespace composition contract:
//! - All intermediates have identifiers `%{scope}.c6meta.temp.{n}`.
//! - That whole prefix is reserved; caller gate/META operands using
//!   it are rejected with `ReservedOperand`.
//! - A scope containing `%` or `.c6meta.temp.` is rejected.
//! - `emit_c6_meta_canonical_block` is single-shot at serial zero.
//!   For several META blocks under one scope in one circuit, create
//!   one `C6MetaCanonicalEmitter` and call `emit` repeatedly; its
//!   serial never resets. The parent Push module uses
//!   `%{scope}.local.temp.*`, DATA uses `%{scope}.c6data.temp.*`, and
//!   this module uses only `%{scope}.c6meta.temp.*`, so the three
//!   namespaces are disjoint.

use midnight_transient_crypto::curve::{Fr, FR_BITS};
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::{Identifier, Instruction};
use mori_core::sherlock::helper_machine::c6_phys::{C6_PHYS_MAGIC, C6_PHYS_VERSION};
use mori_core::sherlock::helper_machine::c6_phys_host::{
    C6HostConfig, C6HostLimits, META_HEADER_LANES, META_SECTION_TAG,
};
use mori_core::sherlock::helper_machine::complete_observation::LayoutSnapshot;
use mori_core::sherlock::helper_machine::shared::BoundsC;
use thiserror::Error;

const LANE_BITS: u32 = 128;
const RESERVED_SEGMENT: &str = ".c6meta.temp.";
/// Stored/compared codepoints are capped at this value. It exceeds
/// every valid Unicode scalar value (`< 0x110000`), so capping never
/// changes a valid codepoint; an over-cap candidate fails the
/// `< 0x110000` assertion under gate 1 exactly as before.
const CP_CAP: u128 = 0x20_0000;

const PH_MAGIC: u128 = 0;
const PH_VERSION: u128 = 1;
const PH_TAG: u128 = 2;
const PH_ADM_BOUNDS: u128 = 12;
const PH_LAYOUT_BOUNDS: u128 = 18;
const PH_LAYOUT_CAPS: u128 = 19;
const PH_LAYOUT_BOUNDARY: u128 = 20;
const PH_LAYOUT_VERSION: u128 = 13;
const PH_LAYOUT_FIELD_ID: u128 = 15;
const PH_LAYOUT_WIDTHS: u128 = 16;
const PH_LAYOUT_SCALAR: u128 = 17;
const PH_RUN_FUEL: u128 = 30;
const PH_RUN_STACK: u128 = 31;
const PH_END: u128 = 32;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum C6MetaCanonicalError {
    #[error("invalid scope identifier: must not be empty")]
    EmptyScope,
    #[error("invalid scope identifier: scope must not contain '%' or the reserved segment '.c6meta.temp.'")]
    InvalidScope,
    #[error("lane bit width {bits} is invalid for the BLS12-381 scalar field")]
    InvalidBitWidth { bits: u32 },
    #[error("META length mismatch in {section}: expected {expected}, got {actual}")]
    LengthMismatch { section: &'static str, expected: usize, actual: usize },
    #[error("metadata bounds frames offset {offset} out of range for {len} lanes")]
    OffsetOutOfRange { offset: usize, len: usize },
    #[error("operand uses reserved META temporary identifier: {identifier}")]
    ReservedOperand { identifier: String },
    #[error("bound META limits do not equal the retained binding limits")]
    BindingLimitsMismatch,
    #[error("invalid META config binding value in {field}")]
    InvalidBinding { field: &'static str },
}

#[derive(Clone, Debug)]
pub struct C6MetaCanonicalInputs {
    pub gate: Operand,
    pub pre_meta: Vec<Operand>,
    pub post_meta: Vec<Operand>,
    /// UNAUTHENTICATED CIRCUIT PARAMETER: chosen by the circuit
    /// builder, defines only this structural relation. Not verified
    /// against any retained host or verifier authority here.
    pub limits: C6HostLimits,
    pub meta_bounds_frames_offset: usize,
}

/// Caller-selected full META configuration binding. Private fields:
/// this type is retained source data, not native authority and not a
/// provenance guard. Constructed only by the validated constructors
/// below, usable by a future same-crate `SourceSpec`; there is no
/// dependency on any absent type.
#[derive(Clone, Debug)]
pub struct C6MetaConfigBinding {
    admission_bounds: BoundsC,
    layout: LayoutSnapshot,
    limits: C6HostLimits,
    admission_values: [u128; 22],
    layout_values: [u128; 22],
    capacity_values: [u128; 4],
    layout_variable_expected: Vec<u128>,
    layout_total_lanes: usize,
}

fn bounds_values(b: &BoundsC) -> [u128; 22] {
    [
        b.code_rows as u128, b.function_rows as u128, b.code_child_slots as u128,
        b.parameter_slots as u128, b.pattern_rows as u128, b.match_plan_rows as u128,
        b.match_arm_rows as u128, b.type_rows as u128, b.constructor_rows as u128,
        b.constructor_field_slots as u128, b.max_arity as u128, b.constant_nodes as u128,
        b.constant_field_slots as u128, b.data_nodes as u128, b.data_field_slots as u128,
        b.environment_cells as u128, b.frames as u128, b.helper_tasks as u128,
        b.serialization_bytes as u128, b.max_reference_work, b.max_helper_work, b.step_cap,
    ]
}

fn push_text_expected(out: &mut Vec<u128>, s: &str) {
    out.push(s.len() as u128);
    out.extend(s.as_bytes().iter().map(|&b| u128::from(b)));
}

fn push_pair_expected(out: &mut Vec<u128>, p: &(String, String)) {
    push_text_expected(out, &p.0);
    push_text_expected(out, &p.1);
}

fn push_opt_pair_expected(out: &mut Vec<u128>, p: &Option<(String, String)>) {
    match p {
        None => out.push(0),
        Some(pair) => { out.push(1); push_pair_expected(out, pair); }
    }
}

fn binding_text_ok(s: &str, limits: &C6HostLimits, cum: &mut usize) -> Result<(), C6MetaCanonicalError> {
    if s.len() > limits.max_string_bytes {
        return Err(C6MetaCanonicalError::InvalidBinding { field: "layout.string" });
    }
    *cum = cum.checked_add(s.len())
        .ok_or(C6MetaCanonicalError::InvalidBinding { field: "layout.bytes" })?;
    if *cum > limits.max_metadata_bytes {
        return Err(C6MetaCanonicalError::InvalidBinding { field: "layout.bytes" });
    }
    Ok(())
}

impl C6MetaConfigBinding {
    /// Validated constructor from actual public Core types. Refuses a
    /// retained layout that no metadata under `limits` could carry:
    /// any layout string above `max_string_bytes`, layout text bytes
    /// above `max_metadata_bytes`, or width entries above
    /// `max_width_entries`. Admission bounds are retained separately
    /// and are never equated with `layout.bounds`.
    pub(crate) fn new(
        admission_bounds: BoundsC,
        layout: LayoutSnapshot,
        limits: C6HostLimits,
    ) -> Result<Self, C6MetaCanonicalError> {
        let mut cum = 0usize;
        binding_text_ok(&layout.layout_version.0, &limits, &mut cum)?;
        binding_text_ok(&layout.layout_version.1, &limits, &mut cum)?;
        if let Some(p) = &layout.field_id {
            binding_text_ok(&p.0, &limits, &mut cum)?;
            binding_text_ok(&p.1, &limits, &mut cum)?;
        }
        let mut width_entries = 0usize;
        if let Some(w) = &layout.widths {
            for list in [&w.index_widths, &w.counter_widths, &w.record_widths] {
                width_entries = width_entries.checked_add(list.len())
                    .ok_or(C6MetaCanonicalError::InvalidBinding { field: "layout.width_entries" })?;
                if width_entries > limits.max_width_entries {
                    return Err(C6MetaCanonicalError::InvalidBinding { field: "layout.width_entries" });
                }
                for (name, _) in list { binding_text_ok(name, &limits, &mut cum)?; }
            }
        }
        if let Some(p) = &layout.scalar_layout_id {
            binding_text_ok(&p.0, &limits, &mut cum)?;
            binding_text_ok(&p.1, &limits, &mut cum)?;
        }
        let mut layout_variable_expected = Vec::new();
        push_pair_expected(&mut layout_variable_expected, &layout.layout_version);
        push_opt_pair_expected(&mut layout_variable_expected, &layout.field_id);
        match &layout.widths {
            None => layout_variable_expected.push(0),
            Some(w) => {
                layout_variable_expected.push(1);
                for list in [&w.index_widths, &w.counter_widths, &w.record_widths] {
                    layout_variable_expected.push(list.len() as u128);
                    for (name, width) in list {
                        push_text_expected(&mut layout_variable_expected, name);
                        layout_variable_expected.push(u128::from(*width));
                    }
                }
            }
        }
        push_opt_pair_expected(&mut layout_variable_expected, &layout.scalar_layout_id);
        let layout_total_lanes = layout_variable_expected.len()
            .checked_add(22).and_then(|n| n.checked_add(4)).and_then(|n| n.checked_add(1))
            .ok_or(C6MetaCanonicalError::InvalidBinding { field: "layout.total" })?;
        Ok(Self {
            admission_values: bounds_values(&admission_bounds),
            layout_values: bounds_values(&layout.bounds),
            capacity_values: [
                layout.capacities.name_rows as u128, layout.capacities.name_bytes as u128,
                layout.capacities.occurrence_path_slots as u128, layout.capacities.value_slots as u128,
            ],
            admission_bounds,
            layout,
            limits,
            layout_variable_expected,
            layout_total_lanes,
        })
    }

    /// Same validated binding taken from an original retained host
    /// config. This module emits nothing outside META: any expected
    /// CTX, derived caps, or source-checker integration remains the
    /// separate mandatory Push/P1 integration, not this constructor.
    pub(crate) fn from_host_config(
        admission_bounds: BoundsC,
        cfg: &C6HostConfig,
        limits: C6HostLimits,
    ) -> Result<Self, C6MetaCanonicalError> {
        Self::new(admission_bounds, cfg.layout.clone(), limits)
    }

    pub(crate) fn admission_bounds(&self) -> BoundsC { self.admission_bounds }
    pub(crate) fn layout(&self) -> &LayoutSnapshot { &self.layout }
    pub(crate) fn limits(&self) -> C6HostLimits { self.limits }
}

#[derive(Clone, Debug)]
pub struct C6MetaBoundCanonicalInputs {
    pub gate: Operand,
    pub pre_meta: Vec<Operand>,
    pub post_meta: Vec<Operand>,
    /// Must equal the binding's retained limits, else host refusal
    /// before any emission. Never raised from raw lanes.
    pub limits: C6HostLimits,
    pub meta_bounds_frames_offset: usize,
    /// Explicit and non-optional: bound mode cannot be selected
    /// accidentally or omitted by the bound entry points.
    pub binding: C6MetaConfigBinding,
}

#[derive(Clone, Debug)]
pub struct C6MetaCanonicalBlock {
    pub instructions: Vec<Instruction>,
}

fn validate_scope(scope: &str) -> Result<(), C6MetaCanonicalError> {
    if scope.trim().is_empty() { return Err(C6MetaCanonicalError::EmptyScope); }
    if scope.contains('%') || scope.contains(RESERVED_SEGMENT) {
        return Err(C6MetaCanonicalError::InvalidScope);
    }
    Ok(())
}

fn reserved_prefix(scope: &str) -> String { format!("%{}.c6meta.temp.", scope) }

fn reject_reserved(scope: &str, operand: &Operand) -> Result<(), C6MetaCanonicalError> {
    if let Operand::Variable(id) = operand {
        if id.0.starts_with(&reserved_prefix(scope)) {
            return Err(C6MetaCanonicalError::ReservedOperand { identifier: id.0.clone() });
        }
    }
    Ok(())
}

#[derive(Clone)]
struct Regs {
    phase: Operand,
    sub: Operand,
    rem: Operand,
    cnt: Operand,
    list: Operand,
    hold: Operand,
    utf_pending: Operand,
    utf_cp: Operand,
    utf_min: Operand,
    cum_bytes: Operand,
    cum_widths: Operand,
    bind_pos: Operand,
}

struct MetaEmitter<'a> {
    scope: &'a str,
    next_temp: usize,
    instructions: Vec<Instruction>,
}

impl<'a> MetaEmitter<'a> {
    fn new(scope: &'a str, next_temp: usize) -> Self { Self { scope, next_temp, instructions: Vec::new() } }
    fn temp(&mut self) -> Identifier {
        let id = Identifier(format!("%{}.c6meta.temp.{}", self.scope, self.next_temp));
        self.next_temp += 1;
        id
    }
    fn imm(v: u128) -> Operand { Operand::Immediate(Fr::from(v)) }
    fn constrain_boolean(&mut self, val: Operand) { self.instructions.push(Instruction::ConstrainToBoolean { val }); }
    fn constrain_bits(&mut self, val: Operand, bits: u32) { self.instructions.push(Instruction::ConstrainBits { val, bits }); }
    fn constrain_eq(&mut self, a: Operand, b: Operand) { self.instructions.push(Instruction::ConstrainEq { a, b }); }
    fn assert_true(&mut self, cond: Operand) { self.instructions.push(Instruction::Assert { cond }); }
    fn test_eq(&mut self, a: Operand, b: Operand) -> Operand {
        let out = self.temp();
        self.instructions.push(Instruction::TestEq { a, b, output: out.clone() });
        Operand::Variable(out)
    }
    fn eqv(&mut self, a: Operand, v: u128) -> Operand { self.test_eq(a, Self::imm(v)) }
    fn lt(&mut self, a: Operand, b: Operand, bits: u32) -> Operand {
        let out = self.temp();
        self.instructions.push(Instruction::LessThan { a, b, bits, output: out.clone() });
        Operand::Variable(out)
    }
    fn lt128(&mut self, a: Operand, v: u128) -> Operand { self.lt(a, Self::imm(v), LANE_BITS) }
    fn add(&mut self, a: Operand, b: Operand) -> Operand {
        let out = self.temp();
        self.instructions.push(Instruction::Add { a, b, output: out.clone() });
        Operand::Variable(out)
    }
    fn add_i(&mut self, a: Operand, d: i128) -> Operand { self.add(a, Operand::Immediate(Fr::from(d))) }
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
    fn sel(&mut self, bit: Operand, a: Operand, b: Operand) -> Operand {
        let out = self.temp();
        self.instructions.push(Instruction::CondSelect { bit, a, b, output: out.clone() });
        Operand::Variable(out)
    }
    fn and(&mut self, a: Operand, b: Operand) -> Operand { self.mul(a, b) }
    fn or(&mut self, a: Operand, b: Operand) -> Operand {
        let na = self.not(a);
        let nb = self.not(b);
        let both = self.mul(na, nb);
        self.not(both)
    }
    fn or_many(&mut self, mut xs: Vec<Operand>) -> Operand {
        let mut acc = xs.remove(0);
        for x in xs { acc = self.or(acc, x); }
        acc
    }
    fn in_range(&mut self, a: Operand, lo: u128, hi: u128) -> Operand {
        let below_lo = self.lt128(a.clone(), lo);
        let ge = self.not(below_lo);
        let lt_hi = self.lt128(a, hi);
        self.and(ge, lt_hi)
    }
    fn gated_assert(&mut self, gate: Operand, cond: Operand) {
        let effective = self.sel(gate, cond, Self::imm(1));
        self.assert_true(effective);
    }
    fn gated_eq(&mut self, gate: Operand, a: Operand, b: Operand) {
        let selected = self.sel(gate, a, b.clone());
        self.constrain_eq(selected, b);
    }
    fn gated_bool(&mut self, gate: Operand, val: Operand) {
        let checked = self.sel(gate, val, Self::imm(0));
        self.constrain_boolean(checked);
    }
    fn check_le(&mut self, active: Operand, lane: Operand, max_plus_one: u128) {
        let ok = self.lt128(lane, max_plus_one);
        self.gated_assert(active, ok);
    }
    fn check_usize(&mut self, active: Operand, lane: Operand) {
        let limit = (usize::MAX as u128) + 1;
        let ok = self.lt128(lane, limit);
        self.gated_assert(active, ok);
    }
    /// Saturating bounded cumulative step. `cum` must already be in
    /// `[0, limit_plus_one]` (initially `0`, thereafter only this
    /// function's stored output). Returns `(stored_next, ok)` where
    /// `ok` is exactly `cum + lane < limit_plus_one` over the
    /// integers, and `stored_next` stays in `[0, limit_plus_one]`.
    /// Every `LessThan` here is 128-bit with both operands `< 2^128`:
    /// `lane < 2^128` by the unconditional lane range, the capped
    /// contribution and `cum` are `<= limit_plus_one <= 2^64`, and
    /// their sum is `< 2^65`.
    fn cum_next_bounded(&mut self, cum: Operand, lane: Operand, limit_plus_one: u128) -> (Operand, Operand) {
        let lane_within = self.lt128(lane.clone(), limit_plus_one);
        let contribution = self.sel(lane_within, lane, Self::imm(limit_plus_one));
        let raw = self.add(cum, contribution);
        let ok = self.lt128(raw.clone(), limit_plus_one);
        let stored = self.sel(ok.clone(), raw, Self::imm(limit_plus_one));
        (stored, ok)
    }
    fn check_len(&mut self, active: Operand, lane: Operand, regs: &Regs, limits: &C6HostLimits) -> Operand {
        self.check_le(active.clone(), lane.clone(), (limits.max_string_bytes as u128) + 1);
        let (stored, ok) = self.cum_next_bounded(regs.cum_bytes.clone(), lane, (limits.max_metadata_bytes as u128) + 1);
        self.gated_assert(active, ok);
        stored
    }
    /// Phase-keyed fixed-field binding for one lane. Selectors are the
    /// actual parser `phase`/`sub` registers and the `bind_pos`
    /// register advanced only across layout phases 13..=20; no host
    /// lane index selects a fixed field. Every comparison is gated by
    /// `gate AND selector`. Only `TestEq` is used, so no operand
    /// beyond the unconditionally 128-bit-ranged lane and immediates
    /// `< 2^128` reaches any comparison.
    fn bind_lane(&mut self, gate: &Operand, lane: Operand, regs: &Regs, binding: &C6MetaConfigBinding) {
        for (phase_const, values) in [
            (PH_ADM_BOUNDS, binding.admission_values),
            (PH_LAYOUT_BOUNDS, binding.layout_values),
        ] {
            let is_phase = self.eqv(regs.phase.clone(), phase_const);
            for (index, expected) in values.iter().enumerate() {
                let is_sub = self.eqv(regs.sub.clone(), index as u128);
                let phase_sub = self.and(is_phase.clone(), is_sub);
                let active = self.and(gate.clone(), phase_sub);
                self.gated_eq(active, lane.clone(), Self::imm(*expected));
            }
        }
        {
            let is_phase = self.eqv(regs.phase.clone(), PH_LAYOUT_CAPS);
            for (index, expected) in binding.capacity_values.iter().enumerate() {
                let is_sub = self.eqv(regs.sub.clone(), index as u128);
                let phase_sub = self.and(is_phase.clone(), is_sub);
                let active = self.and(gate.clone(), phase_sub);
                self.gated_eq(active, lane.clone(), Self::imm(*expected));
            }
        }
        {
            let is_phase = self.eqv(regs.phase.clone(), PH_LAYOUT_BOUNDARY);
            let active = self.and(gate.clone(), is_phase);
            let expected = u128::from(binding.layout.boundary_capacities_present);
            self.gated_eq(active, lane.clone(), Self::imm(expected));
            // The boundary lane is the last layout lane: the cursor
            // must stand exactly at the retained layout total, so a
            // different valid layout shape cannot end the variable
            // segment early or late and avoid the bindings above.
            let last = (binding.layout_total_lanes - 1) as u128;
            let at_last = self.eqv(regs.bind_pos.clone(), last);
            let is_phase2 = self.eqv(regs.phase.clone(), PH_LAYOUT_BOUNDARY);
            let active2 = self.and(gate.clone(), is_phase2);
            self.gated_assert(active2, at_last);
        }
        {
            let v13 = self.eqv(regs.phase.clone(), PH_LAYOUT_VERSION);
            let v14 = self.eqv(regs.phase.clone(), 14);
            let v15 = self.eqv(regs.phase.clone(), PH_LAYOUT_FIELD_ID);
            let v16 = self.eqv(regs.phase.clone(), PH_LAYOUT_WIDTHS);
            let v17 = self.eqv(regs.phase.clone(), PH_LAYOUT_SCALAR);
            let in_variable = self.or_many(vec![v13, v14, v15, v16, v17]);
            for (pos, expected) in binding.layout_variable_expected.iter().enumerate() {
                let at_pos = self.eqv(regs.bind_pos.clone(), pos as u128);
                let pos_sel = self.and(in_variable.clone(), at_pos);
                let active = self.and(gate.clone(), pos_sel);
                self.gated_eq(active, lane.clone(), Self::imm(*expected));
            }
        }
    }

    /// One strict-UTF-8 byte step. Returns (pending, codepoint, minimum).
    /// Rejects lone/truncated continuations, overlong forms (including
    /// C0/C1 leads), surrogates, and codepoints above U+10FFFF, matching
    /// `String::from_utf8`; ASCII, NUL, and valid non-ASCII pass.
    /// All `LessThan` operands are bounded on every evaluation, as
    /// documented at module level: lane operands are 128-bit ranged,
    /// the continuation offset is selected to `[0, 64)`, and the
    /// codepoint is capped to `CP_CAP` before any comparison or store.
    fn utf8_step(&mut self, active: Operand, lane: Operand, regs: &Regs) -> (Operand, Operand, Operand) {
        let b = lane;
        let is_p0 = self.eqv(regs.utf_pending.clone(), 0);
        let active_p0 = self.and(active.clone(), is_p0.clone());
        let not_p0 = self.not(is_p0.clone());
        let active_cont = self.and(active.clone(), not_p0);
        let ascii = self.lt128(b.clone(), 0x80);
        let lead2 = self.in_range(b.clone(), 0xC2, 0xE0);
        let lead3 = self.in_range(b.clone(), 0xE0, 0xF0);
        let lead4 = self.in_range(b.clone(), 0xF0, 0xF5);
        let valid_start = self.or_many(vec![ascii.clone(), lead2.clone(), lead3.clone(), lead4.clone()]);
        self.gated_assert(active_p0, valid_start);
        let cont_ok = self.in_range(b.clone(), 0x80, 0xC0);
        self.gated_assert(active_cont, cont_ok.clone());
        let mut start_pending = Self::imm(0);
        start_pending = self.sel(lead2.clone(), Self::imm(1), start_pending);
        start_pending = self.sel(lead3.clone(), Self::imm(2), start_pending);
        start_pending = self.sel(lead4.clone(), Self::imm(3), start_pending);
        let cont_pending = self.add_i(regs.utf_pending.clone(), -1);
        let pending_next = self.sel(is_p0.clone(), start_pending, cont_pending);
        let mut start_cp = Self::imm(0);
        let cp2 = self.add_i(b.clone(), -0xC0);
        start_cp = self.sel(lead2.clone(), cp2, start_cp);
        let cp3 = self.add_i(b.clone(), -0xE0);
        start_cp = self.sel(lead3.clone(), cp3, start_cp);
        let cp4 = self.add_i(b.clone(), -0xF0);
        start_cp = self.sel(lead4, cp4, start_cp);
        // Bounded continuation offset: selected only when the byte is
        // in [0x80, 0xC0), where `b - 0x80` is the integer in [0, 64).
        // The raw field difference is never compared or stored.
        let low_raw = self.add_i(b.clone(), -0x80);
        let safe_low = self.sel(cont_ok, low_raw, Self::imm(0));
        let shifted = self.mul(regs.utf_cp.clone(), Self::imm(64));
        let cont_cp = self.add(shifted, safe_low);
        let cp_candidate = self.sel(is_p0.clone(), start_cp, cont_cp);
        // Mechanically cap the stored/compared codepoint. Both
        // operands of this comparison are `< 2^128`: the candidate is
        // a selected start offset (`< 0x20` when its lead holds, else
        // a previous bounded value), a shifted previously-capped
        // codepoint (`<= CP_CAP * 64 + 63 < 2^128`), or `0`.
        let cp_within = self.lt128(cp_candidate.clone(), CP_CAP);
        let cp_next = self.sel(cp_within, cp_candidate, Self::imm(CP_CAP));
        let mut start_min = Self::imm(0);
        start_min = self.sel(lead2, Self::imm(0x80), start_min);
        start_min = self.sel(lead3, Self::imm(0x800), start_min);
        let lead4_min = self.in_range(b.clone(), 0xF0, 0xF5);
        start_min = self.sel(lead4_min, Self::imm(0x10000), start_min);
        let min_next = self.sel(is_p0, start_min, regs.utf_min.clone());
        let completing = self.eqv(regs.utf_pending.clone(), 1);
        let active_done = self.and(active, completing);
        let min_now = min_next.clone();
        let below = self.lt(cp_next.clone(), min_now, LANE_BITS);
        let ge_min = self.not(below);
        self.gated_assert(active_done.clone(), ge_min);
        let le_max = self.lt128(cp_next.clone(), 0x110000);
        self.gated_assert(active_done.clone(), le_max);
        let surr = self.in_range(cp_next.clone(), 0xD800, 0xE000);
        let not_surr = self.not(surr);
        self.gated_assert(active_done, not_surr);
        (pending_next, cp_next, min_next)
    }

    #[allow(clippy::too_many_arguments)]
    fn step(&mut self, gate: &Operand, lane: Operand, regs: Regs, limits: &C6HostLimits, at_bounds_frames: bool, binding: Option<&C6MetaConfigBinding>) -> Regs {
        let phase = regs.phase.clone();
        let sub = regs.sub.clone();
        let ph = |this: &mut Self, v: u128| this.eqv(phase.clone(), v);
        let sb = |this: &mut Self, v: u128| this.eqv(sub.clone(), v);
        // Phase classes, in codec field order.
        let pstr_6 = ph(self, 6);
        let pstr_7 = ph(self, 7);
        let pstr_13 = ph(self, 13);
        let pstr_14 = ph(self, 14);
        let pstr_24 = ph(self, 24);
        let pstr_25 = ph(self, 25);
        let p_str = self.or_many(vec![pstr_6, pstr_7, pstr_13, pstr_14, pstr_24, pstr_25]);
        let pdig_8 = ph(self, 8);
        let pdig_9 = ph(self, 9);
        let pdig_11 = ph(self, 11);
        let pdig_26 = ph(self, 26);
        let pdig_27 = ph(self, 27);
        let pdig_29 = ph(self, 29);
        let p_dig = self.or_many(vec![pdig_8, pdig_9, pdig_11, pdig_26, pdig_27, pdig_29]);
        let popt_10 = ph(self, 10);
        let popt_15 = ph(self, 15);
        let popt_17 = ph(self, 17);
        let popt_28 = ph(self, 28);
        let p_opt = self.or_many(vec![popt_10, popt_15, popt_17, popt_28]);
        let p_widths = ph(self, 16);
        let pbounds_adm = ph(self, PH_ADM_BOUNDS);
        let pbounds_layout = ph(self, PH_LAYOUT_BOUNDS);
        let p_bounds = self.or_many(vec![pbounds_adm, pbounds_layout]);
        let p_caps = ph(self, PH_LAYOUT_CAPS);
        let prun_fuel = ph(self, PH_RUN_FUEL);
        let prun_stack = ph(self, PH_RUN_STACK);
        let p_runint = self.or_many(vec![prun_fuel, prun_stack]);
        let s0 = sb(self, 0);
        let s1 = sb(self, 1);
        let s2 = sb(self, 2);
        let s3 = sb(self, 3);
        let s4 = sb(self, 4);
        let s5 = sb(self, 5);
        let s6 = sb(self, 6);
        if at_bounds_frames {
            let is_adm = ph(self, PH_ADM_BOUNDS);
            let is_frames = sb(self, 16);
            let tie = self.and(is_adm, is_frames);
            self.gated_assert(gate.clone(), tie);
        }
        if let Some(binding) = binding {
            self.bind_lane(gate, lane.clone(), &regs, binding);
        }
        let mut next_phase = regs.phase.clone();
        let mut next_sub = regs.sub.clone();
        let mut next_rem = regs.rem.clone();
        let mut next_cnt = regs.cnt.clone();
        let mut next_list = regs.list.clone();
        let mut next_hold = regs.hold.clone();
        let mut next_utf_p = regs.utf_pending.clone();
        let mut next_utf_cp = regs.utf_cp.clone();
        let mut next_utf_min = regs.utf_min.clone();
        let mut next_cum_bytes = regs.cum_bytes.clone();
        let mut next_cum_widths = regs.cum_widths.clone();
        let next_phase_inc = self.add_i(regs.phase.clone(), 1);
        let mut next_bind_pos = regs.bind_pos.clone();
        if binding.is_some() {
            let b13 = self.eqv(regs.phase.clone(), PH_LAYOUT_VERSION);
            let b14 = self.eqv(regs.phase.clone(), 14);
            let b15 = self.eqv(regs.phase.clone(), PH_LAYOUT_FIELD_ID);
            let b16 = self.eqv(regs.phase.clone(), PH_LAYOUT_WIDTHS);
            let b17 = self.eqv(regs.phase.clone(), PH_LAYOUT_SCALAR);
            let b18 = self.eqv(regs.phase.clone(), PH_LAYOUT_BOUNDS);
            let b19 = self.eqv(regs.phase.clone(), PH_LAYOUT_CAPS);
            let b20 = self.eqv(regs.phase.clone(), PH_LAYOUT_BOUNDARY);
            let in_layout = self.or_many(vec![b13, b14, b15, b16, b17, b18, b19, b20]);
            let bind_inc = self.add_i(regs.bind_pos.clone(), 1);
            next_bind_pos = self.sel(in_layout, bind_inc, next_bind_pos);
        }
        // Fixed header.
        for (p, value) in [(PH_MAGIC, C6_PHYS_MAGIC), (PH_VERSION, C6_PHYS_VERSION), (PH_TAG, META_SECTION_TAG)] {
            let is_p = ph(self, p);
            let active = self.and(gate.clone(), is_p.clone());
            self.gated_eq(active, lane.clone(), Self::imm(value));
            next_phase = self.sel(is_p, next_phase_inc.clone(), next_phase);
        }
        // Profile (u8), grammar and finance (tags <= 1), in both identities.
        let pprofile_3 = ph(self, 3);
        let pprofile_21 = ph(self, 21);
        let p_profile = self.or_many(vec![pprofile_3, pprofile_21]);
        let active = self.and(gate.clone(), p_profile.clone());
        self.check_le(active, lane.clone(), 256);
        next_phase = self.sel(p_profile, next_phase_inc.clone(), next_phase);
        let ptag1_4 = ph(self, 4);
        let ptag1_5 = ph(self, 5);
        let ptag1_22 = ph(self, 22);
        let ptag1_23 = ph(self, 23);
        let p_tag1 = self.or_many(vec![ptag1_4, ptag1_5, ptag1_22, ptag1_23]);
        let active = self.and(gate.clone(), p_tag1.clone());
        self.check_le(active, lane.clone(), 2);
        next_phase = self.sel(p_tag1, next_phase_inc.clone(), next_phase);
        // Plain strings: sub 0 = length, sub 1 = UTF-8 bytes.
        let cond_len = self.and(p_str.clone(), s0.clone());
        let cond_byte = self.and(p_str.clone(), s1.clone());
        {
            let active = self.and(gate.clone(), cond_len.clone());
            let cum_next = self.check_len(active, lane.clone(), &regs, limits);
            next_cum_bytes = self.sel(cond_len.clone(), cum_next, next_cum_bytes);
            next_utf_p = self.sel(cond_len.clone(), Self::imm(0), next_utf_p);
            next_utf_cp = self.sel(cond_len.clone(), Self::imm(0), next_utf_cp);
            next_utf_min = self.sel(cond_len.clone(), Self::imm(0), next_utf_min);
            let is_zero = self.eqv(lane.clone(), 0);
            let not_zero = self.not(is_zero.clone());
            let go_next = self.and(cond_len.clone(), is_zero);
            next_phase = self.sel(go_next.clone(), next_phase_inc.clone(), next_phase);
            let go_bytes = self.and(cond_len.clone(), not_zero);
            next_sub = self.sel(go_bytes.clone(), Self::imm(1), next_sub);
            next_sub = self.sel(go_next, Self::imm(0), next_sub);
            next_rem = self.sel(go_bytes, lane.clone(), next_rem);
        }
        {
            let active = self.and(gate.clone(), cond_byte.clone());
            self.check_le(active.clone(), lane.clone(), 256);
            let (pend, cp, min) = self.utf8_step(active.clone(), lane.clone(), &regs);
            next_utf_p = self.sel(cond_byte.clone(), pend.clone(), next_utf_p);
            next_utf_cp = self.sel(cond_byte.clone(), cp, next_utf_cp);
            next_utf_min = self.sel(cond_byte.clone(), min, next_utf_min);
            let is_last = self.eqv(regs.rem.clone(), 1);
            let done = self.and(cond_byte.clone(), is_last.clone());
            let pend_zero = self.eqv(pend, 0);
            let active_done = self.and(active, is_last);
            self.gated_assert(active_done, pend_zero);
            let rem_dec_plain_byte = self.add_i(regs.rem.clone(), -1);
            next_rem = self.sel(cond_byte.clone(), rem_dec_plain_byte, next_rem);
            next_phase = self.sel(done.clone(), next_phase_inc.clone(), next_phase);
            next_sub = self.sel(done, Self::imm(0), next_sub);
        }
        // Digests: 0 tag, 1/2 format string 0, 3/4 format string 1,
        // 5/6 opaque bytes blob. Unselected advances immediately.
        {
            let cond_tag = self.and(p_dig.clone(), s0.clone());
            let active = self.and(gate.clone(), cond_tag.clone());
            self.check_le(active, lane.clone(), 2);
            next_hold = self.sel(cond_tag.clone(), lane.clone(), next_hold);
            let is_zero = self.eqv(lane.clone(), 0);
            let is_one = self.eqv(lane.clone(), 1);
            let adv = self.and(cond_tag.clone(), is_zero);
            next_phase = self.sel(adv, next_phase_inc.clone(), next_phase);
            let go = self.and(cond_tag, is_one);
            next_sub = self.sel(go, Self::imm(1), next_sub);
        }
        {
            let is_len_sub = self.or_many(vec![s1.clone(), s3.clone(), s5.clone()]);
            let cond_len = self.and(p_dig.clone(), is_len_sub);
            let active = self.and(gate.clone(), cond_len.clone());
            let cum_next = self.check_len(active.clone(), lane.clone(), &regs, limits);
            let is_text_len = self.or_many(vec![s1.clone(), s3.clone()]);
            let text_len = self.and(cond_len.clone(), is_text_len);
            next_cum_bytes = self.sel(cond_len.clone(), cum_next, next_cum_bytes);
            next_utf_p = self.sel(text_len.clone(), Self::imm(0), next_utf_p);
            next_utf_cp = self.sel(text_len.clone(), Self::imm(0), next_utf_cp);
            next_utf_min = self.sel(text_len.clone(), Self::imm(0), next_utf_min);
            let is_zero = self.eqv(lane.clone(), 0);
            let not_zero = self.not(is_zero.clone());
            // Zero length: 1->3, 3->5, 5->next phase. Non-zero: +1 sub.
            let s1_and_zero_digest_len = self.and(s1.clone(), is_zero.clone());
            let z1 = self.and(cond_len.clone(), s1_and_zero_digest_len);
            next_sub = self.sel(z1, Self::imm(3), next_sub);
            let s3_and_zero_digest_len = self.and(s3.clone(), is_zero.clone());
            let z3 = self.and(cond_len.clone(), s3_and_zero_digest_len);
            next_sub = self.sel(z3, Self::imm(5), next_sub);
            let s5_and_zero_digest_len = self.and(s5.clone(), is_zero.clone());
            let z5 = self.and(cond_len.clone(), s5_and_zero_digest_len);
            next_phase = self.sel(z5.clone(), next_phase_inc.clone(), next_phase);
            next_sub = self.sel(z5, Self::imm(0), next_sub);
            let nz = self.and(cond_len, not_zero);
            let sub_inc = self.add_i(regs.sub.clone(), 1);
            next_sub = self.sel(nz.clone(), sub_inc, next_sub);
            next_rem = self.sel(nz, lane.clone(), next_rem);
        }
        {
            let is_byte_sub = self.or_many(vec![s2.clone(), s4.clone(), s6.clone()]);
            let cond_byte = self.and(p_dig.clone(), is_byte_sub);
            let is_text_byte = self.or_many(vec![s2.clone(), s4.clone()]);
            let text_byte = self.and(cond_byte.clone(), is_text_byte);
            let active = self.and(gate.clone(), cond_byte.clone());
            self.check_le(active.clone(), lane.clone(), 256);
            let active_text = self.and(gate.clone(), text_byte.clone());
            let (pend, cp, min) = self.utf8_step(active_text, lane.clone(), &regs);
            next_utf_p = self.sel(text_byte.clone(), pend.clone(), next_utf_p);
            next_utf_cp = self.sel(text_byte.clone(), cp, next_utf_cp);
            next_utf_min = self.sel(text_byte, min, next_utf_min);
            let is_last = self.eqv(regs.rem.clone(), 1);
            let done = self.and(cond_byte.clone(), is_last.clone());
            let pend_zero = self.eqv(pend, 0);
            let text_byte_sub_digest = self.or_many(vec![s2.clone(), s4.clone()]);
            let text_byte_and_last_digest = self.and(text_byte_sub_digest, is_last.clone());
            let active_text_done = self.and(active.clone(), text_byte_and_last_digest);
            self.gated_assert(active_text_done, pend_zero);
            let rem_dec_digest_byte = self.add_i(regs.rem.clone(), -1);
            next_rem = self.sel(cond_byte.clone(), rem_dec_digest_byte, next_rem);
            let sub_inc = self.add_i(regs.sub.clone(), 1);
            let not_s6_digest = self.not(s6.clone());
            let done_not_blob = self.and(done.clone(), not_s6_digest);
            next_sub = self.sel(done_not_blob, sub_inc, next_sub);
            let done_blob = self.and(done, s6.clone());
            next_phase = self.sel(done_blob.clone(), next_phase_inc.clone(), next_phase);
            next_sub = self.sel(done_blob, Self::imm(0), next_sub);
        }
        // Optional text pairs: 0 tag, 1/2 string 0, 3/4 string 1.
        {
            let cond_tag = self.and(p_opt.clone(), s0.clone());
            let active = self.and(gate.clone(), cond_tag.clone());
            self.check_le(active, lane.clone(), 2);
            next_hold = self.sel(cond_tag.clone(), lane.clone(), next_hold);
            let is_zero = self.eqv(lane.clone(), 0);
            let is_one = self.eqv(lane.clone(), 1);
            let adv = self.and(cond_tag.clone(), is_zero);
            next_phase = self.sel(adv, next_phase_inc.clone(), next_phase);
            let go = self.and(cond_tag, is_one);
            next_sub = self.sel(go, Self::imm(1), next_sub);
        }
        {
            let is_len_sub = self.or_many(vec![s1.clone(), s3.clone()]);
            let cond_len = self.and(p_opt.clone(), is_len_sub);
            let active = self.and(gate.clone(), cond_len.clone());
            let cum_next = self.check_len(active, lane.clone(), &regs, limits);
            next_cum_bytes = self.sel(cond_len.clone(), cum_next, next_cum_bytes);
            next_utf_p = self.sel(cond_len.clone(), Self::imm(0), next_utf_p);
            next_utf_cp = self.sel(cond_len.clone(), Self::imm(0), next_utf_cp);
            next_utf_min = self.sel(cond_len.clone(), Self::imm(0), next_utf_min);
            let is_zero = self.eqv(lane.clone(), 0);
            let not_zero = self.not(is_zero.clone());
            let s1_and_zero_opt_len = self.and(s1.clone(), is_zero.clone());
            let z1 = self.and(cond_len.clone(), s1_and_zero_opt_len);
            next_sub = self.sel(z1, Self::imm(3), next_sub);
            let s3_and_zero_opt_len = self.and(s3.clone(), is_zero.clone());
            let z3 = self.and(cond_len.clone(), s3_and_zero_opt_len);
            next_phase = self.sel(z3.clone(), next_phase_inc.clone(), next_phase);
            next_sub = self.sel(z3, Self::imm(0), next_sub);
            let nz = self.and(cond_len, not_zero);
            let sub_inc = self.add_i(regs.sub.clone(), 1);
            next_sub = self.sel(nz.clone(), sub_inc, next_sub);
            next_rem = self.sel(nz, lane.clone(), next_rem);
        }
        {
            let is_byte_sub = self.or_many(vec![s2.clone(), s4.clone()]);
            let cond_byte = self.and(p_opt.clone(), is_byte_sub);
            let active = self.and(gate.clone(), cond_byte.clone());
            self.check_le(active.clone(), lane.clone(), 256);
            let (pend, cp, min) = self.utf8_step(active.clone(), lane.clone(), &regs);
            next_utf_p = self.sel(cond_byte.clone(), pend.clone(), next_utf_p);
            next_utf_cp = self.sel(cond_byte.clone(), cp, next_utf_cp);
            next_utf_min = self.sel(cond_byte.clone(), min, next_utf_min);
            let is_last = self.eqv(regs.rem.clone(), 1);
            let done = self.and(cond_byte.clone(), is_last.clone());
            let pend_zero = self.eqv(pend, 0);
            let active_done = self.and(active, is_last.clone());
            self.gated_assert(active_done, pend_zero);
            let rem_dec_opt_byte = self.add_i(regs.rem.clone(), -1);
            next_rem = self.sel(cond_byte.clone(), rem_dec_opt_byte, next_rem);
            let d2 = self.and(done.clone(), s2.clone());
            next_sub = self.sel(d2, Self::imm(3), next_sub);
            let d4 = self.and(done, s4.clone());
            next_phase = self.sel(d4.clone(), next_phase_inc.clone(), next_phase);
            next_sub = self.sel(d4, Self::imm(0), next_sub);
        }
        // Widths: 0 tag; 1 list count; 2 name length; 3 name bytes;
        // 4 u32 width. Three raw lists in order, duplicates and raw
        // order preserved, cumulative entries capped.
        {
            let cond_tag = self.and(p_widths.clone(), s0.clone());
            let active = self.and(gate.clone(), cond_tag.clone());
            self.check_le(active, lane.clone(), 2);
            next_hold = self.sel(cond_tag.clone(), lane.clone(), next_hold);
            let is_zero = self.eqv(lane.clone(), 0);
            let is_one = self.eqv(lane.clone(), 1);
            let adv = self.and(cond_tag.clone(), is_zero);
            next_phase = self.sel(adv, next_phase_inc.clone(), next_phase);
            let go = self.and(cond_tag, is_one);
            next_sub = self.sel(go.clone(), Self::imm(1), next_sub);
            next_list = self.sel(go, Self::imm(0), next_list);
        }
        {
            let cond_count = self.and(p_widths.clone(), s1.clone());
            let active = self.and(gate.clone(), cond_count.clone());
            self.check_usize(active.clone(), lane.clone());
            let (cum_next, ok) = self.cum_next_bounded(regs.cum_widths.clone(), lane.clone(), (limits.max_width_entries as u128) + 1);
            self.gated_assert(active, ok);
            next_cum_widths = self.sel(cond_count.clone(), cum_next, next_cum_widths);
            next_cnt = self.sel(cond_count.clone(), lane.clone(), next_cnt);
            let is_zero = self.eqv(lane.clone(), 0);
            let not_zero = self.not(is_zero.clone());
            let is_last_list = self.eqv(regs.list.clone(), 2);
            let zero_and_last_list_count = self.and(is_zero.clone(), is_last_list.clone());
            let z_last = self.and(cond_count.clone(), zero_and_last_list_count);
            next_phase = self.sel(z_last.clone(), next_phase_inc.clone(), next_phase);
            next_sub = self.sel(z_last, Self::imm(0), next_sub);
            let not_last_list_count = self.not(is_last_list);
            let zero_and_not_last_count = self.and(is_zero, not_last_list_count);
            let z_more = self.and(cond_count.clone(), zero_and_not_last_count);
            let list_inc_count = self.add_i(regs.list.clone(), 1);
            next_list = self.sel(z_more, list_inc_count, next_list);
            let nz = self.and(cond_count, not_zero);
            next_sub = self.sel(nz, Self::imm(2), next_sub);
        }
        {
            let cond_len = self.and(p_widths.clone(), s2.clone());
            let active = self.and(gate.clone(), cond_len.clone());
            let cum_next = self.check_len(active, lane.clone(), &regs, limits);
            next_cum_bytes = self.sel(cond_len.clone(), cum_next, next_cum_bytes);
            next_utf_p = self.sel(cond_len.clone(), Self::imm(0), next_utf_p);
            next_utf_cp = self.sel(cond_len.clone(), Self::imm(0), next_utf_cp);
            next_utf_min = self.sel(cond_len.clone(), Self::imm(0), next_utf_min);
            let is_zero = self.eqv(lane.clone(), 0);
            let not_zero = self.not(is_zero.clone());
            let z = self.and(cond_len.clone(), is_zero);
            next_sub = self.sel(z, Self::imm(4), next_sub);
            let nz = self.and(cond_len, not_zero);
            next_sub = self.sel(nz.clone(), Self::imm(3), next_sub);
            next_rem = self.sel(nz, lane.clone(), next_rem);
        }
        {
            let cond_byte = self.and(p_widths.clone(), s3.clone());
            let active = self.and(gate.clone(), cond_byte.clone());
            self.check_le(active.clone(), lane.clone(), 256);
            let (pend, cp, min) = self.utf8_step(active.clone(), lane.clone(), &regs);
            next_utf_p = self.sel(cond_byte.clone(), pend.clone(), next_utf_p);
            next_utf_cp = self.sel(cond_byte.clone(), cp, next_utf_cp);
            next_utf_min = self.sel(cond_byte.clone(), min, next_utf_min);
            let is_last = self.eqv(regs.rem.clone(), 1);
            let done = self.and(cond_byte.clone(), is_last.clone());
            let pend_zero = self.eqv(pend, 0);
            let active_done = self.and(active, is_last);
            self.gated_assert(active_done, pend_zero);
            let rem_dec_width_byte = self.add_i(regs.rem.clone(), -1);
            next_rem = self.sel(cond_byte.clone(), rem_dec_width_byte, next_rem);
            next_sub = self.sel(done, Self::imm(4), next_sub);
        }
        {
            let cond_width = self.and(p_widths.clone(), s4.clone());
            let active = self.and(gate.clone(), cond_width.clone());
            self.check_le(active, lane.clone(), 4294967296);
            let cnt_is_one = self.eqv(regs.cnt.clone(), 1);
            let not_one = self.not(cnt_is_one.clone());
            let more = self.and(cond_width.clone(), not_one);
            let cnt_dec_width = self.add_i(regs.cnt.clone(), -1);
            next_cnt = self.sel(more.clone(), cnt_dec_width, next_cnt);
            next_sub = self.sel(more, Self::imm(2), next_sub);
            let is_last_list = self.eqv(regs.list.clone(), 2);
            let one_and_last_list_width = self.and(cnt_is_one.clone(), is_last_list.clone());
            let done_last = self.and(cond_width.clone(), one_and_last_list_width);
            next_phase = self.sel(done_last.clone(), next_phase_inc.clone(), next_phase);
            next_sub = self.sel(done_last.clone(), Self::imm(0), next_sub);
            next_cnt = self.sel(done_last.clone(), Self::imm(0), next_cnt);
            let not_last_list_width = self.not(is_last_list);
            let one_and_not_last_width = self.and(cnt_is_one, not_last_list_width);
            let done_more = self.and(cond_width, one_and_not_last_width);
            let list_inc_width_done = self.add_i(regs.list.clone(), 1);
            next_list = self.sel(done_more.clone(), list_inc_width_done, next_list);
            next_sub = self.sel(done_more.clone(), Self::imm(1), next_sub);
            next_cnt = self.sel(done_more, Self::imm(0), next_cnt);
        }
        // Both 22-lane BoundsC blocks: first 19 usize, last 3 raw u128.
        {
            let cond = p_bounds.clone();
            let is_index = self.lt(sub.clone(), Self::imm(19), LANE_BITS);
            let cond_and_index_bounds = self.and(cond.clone(), is_index);
            let active_index = self.and(gate.clone(), cond_and_index_bounds);
            self.check_usize(active_index, lane.clone());
            let is_last = self.eqv(sub.clone(), 21);
            let not_last = self.not(is_last.clone());
            let adv = self.and(cond.clone(), is_last.clone());
            next_phase = self.sel(adv.clone(), next_phase_inc.clone(), next_phase);
            let stay = self.and(cond.clone(), not_last);
            let sub_inc = self.add_i(regs.sub.clone(), 1);
            next_sub = self.sel(stay, sub_inc, next_sub);
            next_sub = self.sel(adv, Self::imm(0), next_sub);
        }
        // Four layout capacities, all usize.
        {
            let cond = p_caps.clone();
            let active = self.and(gate.clone(), cond.clone());
            self.check_usize(active, lane.clone());
            let is_last = self.eqv(sub.clone(), 3);
            let adv = self.and(cond.clone(), is_last.clone());
            next_phase = self.sel(adv.clone(), next_phase_inc.clone(), next_phase);
            let not_last_caps = self.not(is_last.clone());
            let stay = self.and(cond, not_last_caps);
            let sub_inc = self.add_i(regs.sub.clone(), 1);
            next_sub = self.sel(stay, sub_inc, next_sub);
            next_sub = self.sel(adv, Self::imm(0), next_sub);
        }
        // Boundary bool.
        {
            let cond = ph(self, PH_LAYOUT_BOUNDARY);
            let active = self.and(gate.clone(), cond.clone());
            self.gated_bool(active, lane.clone());
            next_phase = self.sel(cond, next_phase_inc.clone(), next_phase);
        }
        // RunIntC: tag <= 2, then one lane; non-Nat tags require zero.
        {
            let cond_tag = self.and(p_runint.clone(), s0.clone());
            let active = self.and(gate.clone(), cond_tag.clone());
            self.check_le(active, lane.clone(), 3);
            next_hold = self.sel(cond_tag.clone(), lane.clone(), next_hold);
            next_sub = self.sel(cond_tag, Self::imm(1), next_sub);
            let cond_val = self.and(p_runint.clone(), s1.clone());
            let is_nat = self.eqv(regs.hold.clone(), 1);
            let not_nat = self.not(is_nat);
            let pad_cond = self.and(cond_val.clone(), not_nat);
            let active_pad = self.and(gate.clone(), pad_cond);
            self.gated_eq(active_pad, lane.clone(), Self::imm(0));
            next_phase = self.sel(cond_val.clone(), next_phase_inc.clone(), next_phase);
            next_sub = self.sel(cond_val, Self::imm(0), next_sub);
        }
        // No lane may be consumed in the END phase (trailing lanes).
        {
            let cond_end = ph(self, PH_END);
            let active = self.and(gate.clone(), cond_end);
            self.gated_assert(active, Self::imm(0));
        }
        Regs { phase: next_phase, sub: next_sub, rem: next_rem, cnt: next_cnt, list: next_list, hold: next_hold, utf_pending: next_utf_p, utf_cp: next_utf_cp, utf_min: next_utf_min, cum_bytes: next_cum_bytes, cum_widths: next_cum_widths, bind_pos: next_bind_pos }
    }

    fn emit_one(&mut self, gate: &Operand, lanes: &[Operand], limits: &C6HostLimits, offset: usize, section: &'static str, binding: Option<&C6MetaConfigBinding>) -> Result<(), C6MetaCanonicalError> {
        let _ = section;
        let mut regs = Regs { phase: Self::imm(0), sub: Self::imm(0), rem: Self::imm(0), cnt: Self::imm(0), list: Self::imm(0), hold: Self::imm(0), utf_pending: Self::imm(0), utf_cp: Self::imm(0), utf_min: Self::imm(0), cum_bytes: Self::imm(0), cum_widths: Self::imm(0), bind_pos: Self::imm(0) };
        for (index, lane) in lanes.iter().enumerate() {
            self.constrain_bits(lane.clone(), LANE_BITS);
            regs = self.step(gate, lane.clone(), regs, limits, index == offset, binding);
        }
        self.gated_eq(gate.clone(), regs.phase, Self::imm(PH_END));
        Ok(())
    }
}

pub struct C6MetaCanonicalEmitter {
    scope: String,
    next_temp: usize,
}

impl C6MetaCanonicalEmitter {
    pub fn new(scope: &str) -> Result<Self, C6MetaCanonicalError> {
        validate_scope(scope)?;
        Ok(Self { scope: scope.to_string(), next_temp: 0 })
    }
    pub fn scope(&self) -> &str { &self.scope }
    pub fn next_temp(&self) -> usize { self.next_temp }
    pub fn emit(&mut self, inputs: &C6MetaCanonicalInputs) -> Result<C6MetaCanonicalBlock, C6MetaCanonicalError> {
        if (LANE_BITS as usize) >= FR_BITS {
            return Err(C6MetaCanonicalError::InvalidBitWidth { bits: LANE_BITS });
        }
        reject_reserved(&self.scope, &inputs.gate)?;
        for lane in inputs.pre_meta.iter().chain(inputs.post_meta.iter()) { reject_reserved(&self.scope, lane)?; }
        if inputs.pre_meta.len() < META_HEADER_LANES {
            return Err(C6MetaCanonicalError::LengthMismatch { section: "meta.pre", expected: META_HEADER_LANES, actual: inputs.pre_meta.len() });
        }
        if inputs.pre_meta.len() != inputs.post_meta.len() {
            return Err(C6MetaCanonicalError::LengthMismatch { section: "meta", expected: inputs.pre_meta.len(), actual: inputs.post_meta.len() });
        }
        if inputs.meta_bounds_frames_offset >= inputs.pre_meta.len() {
            return Err(C6MetaCanonicalError::OffsetOutOfRange { offset: inputs.meta_bounds_frames_offset, len: inputs.pre_meta.len() });
        }
        let mut em = MetaEmitter::new(&self.scope, self.next_temp);
        em.constrain_boolean(inputs.gate.clone());
        em.emit_one(&inputs.gate, &inputs.pre_meta, &inputs.limits, inputs.meta_bounds_frames_offset, "meta.pre", None)?;
        em.emit_one(&inputs.gate, &inputs.post_meta, &inputs.limits, inputs.meta_bounds_frames_offset, "meta.post", None)?;
        self.next_temp = em.next_temp;
        Ok(C6MetaCanonicalBlock { instructions: em.instructions })
    }
}

pub fn emit_c6_meta_canonical_block(inputs: &C6MetaCanonicalInputs, scope: &str) -> Result<C6MetaCanonicalBlock, C6MetaCanonicalError> {
    let mut emitter = C6MetaCanonicalEmitter::new(scope)?;
    emitter.emit(inputs)
}

impl C6MetaCanonicalEmitter {
    /// Bound emission in this emitter's retained
    /// `%{scope}.c6meta.temp.` serial namespace, disjoint from the
    /// parent `.local.temp.` and DATA `.c6data.temp.` namespaces.
    /// Repeated calls share the serial and never reuse a temp; the
    /// standalone function below is single-shot at serial zero, and
    /// reusing one scope for two standalone blocks in one circuit is
    /// a caller collision, not a supported composition.
    pub fn emit_bound(&mut self, inputs: &C6MetaBoundCanonicalInputs) -> Result<C6MetaCanonicalBlock, C6MetaCanonicalError> {
        if (LANE_BITS as usize) >= FR_BITS {
            return Err(C6MetaCanonicalError::InvalidBitWidth { bits: LANE_BITS });
        }
        if inputs.limits != inputs.binding.limits() {
            return Err(C6MetaCanonicalError::BindingLimitsMismatch);
        }
        reject_reserved(&self.scope, &inputs.gate)?;
        for lane in inputs.pre_meta.iter().chain(inputs.post_meta.iter()) { reject_reserved(&self.scope, lane)?; }
        if inputs.pre_meta.len() < META_HEADER_LANES {
            return Err(C6MetaCanonicalError::LengthMismatch { section: "meta.pre", expected: META_HEADER_LANES, actual: inputs.pre_meta.len() });
        }
        if inputs.pre_meta.len() != inputs.post_meta.len() {
            return Err(C6MetaCanonicalError::LengthMismatch { section: "meta", expected: inputs.pre_meta.len(), actual: inputs.post_meta.len() });
        }
        if inputs.meta_bounds_frames_offset >= inputs.pre_meta.len() {
            return Err(C6MetaCanonicalError::OffsetOutOfRange { offset: inputs.meta_bounds_frames_offset, len: inputs.pre_meta.len() });
        }
        let mut em = MetaEmitter::new(&self.scope, self.next_temp);
        em.constrain_boolean(inputs.gate.clone());
        em.emit_one(&inputs.gate, &inputs.pre_meta, &inputs.limits, inputs.meta_bounds_frames_offset, "meta.pre", Some(&inputs.binding))?;
        em.emit_one(&inputs.gate, &inputs.post_meta, &inputs.limits, inputs.meta_bounds_frames_offset, "meta.post", Some(&inputs.binding))?;
        self.next_temp = em.next_temp;
        Ok(C6MetaCanonicalBlock { instructions: em.instructions })
    }
}

/// Single-shot bound block at serial zero. For several bound blocks
/// under one scope in one circuit, use one `C6MetaCanonicalEmitter`
/// and call `emit_bound` repeatedly. Existing unbound
/// `emit_c6_meta_canonical_block` remains exactly usable unchanged.
pub fn emit_c6_meta_bound_canonical_block(inputs: &C6MetaBoundCanonicalInputs, scope: &str) -> Result<C6MetaCanonicalBlock, C6MetaCanonicalError> {
    let mut emitter = C6MetaCanonicalEmitter::new(scope)?;
    emitter.emit_bound(inputs)
}

#[cfg(test)]
#[path = "c6_meta_canonical_bound_tests.rs"]
mod c6_meta_canonical_bound_tests;
