//! C6 call-return completion: actual KMachineReturnCallValid / Invalid (A185/A190).
//! Source candidate, UNCOMPILED_UNRUN. Enabled input is Return + top ReturnF +
//! ScalarTypeTest result, no active helper, Running and positive work. Valid restores
//! exact saved env/origin, pops precisely the raw bottom-to-top top frame and zeros
//! its padded row. Invalid writes Error6, keeps every frame/env/origin. Both clear
//! result, charge work1 and cursor1, and never charge helper_work/visits (no P3).
//! All10 frame variants are canonicalized for lower frames; raw lanes are unsigned128
//! unconditionally and slice ends use129 bits. Bound META, generic PROG/CTX, DATA,
//! HELPER reuse current actual modules and original caller-retained family config.
//! Disabled imposes pre==post, not selected shape. No ungated receipt is exported.
//! PRE ScalarTypeTest.ok is a staged input, not a newly proved return-type judgment:
//! predecessor checked-reader/ScalarTag/Exit and SI08/15/17/20/21/22/StepInv remain
//! external composition obligations. No host flag substitutes for this circuit.
//! Program identity/auth/issuer, full39, grouping/residual, finance/persistence and
//! recursive/stagenet/native acceptance remain open. No per-program IR template.
//! Composition reserves all five existing temporary prefixes and retains serials.

use midnight_transient_crypto::curve::{Fr, FR_BITS};
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::{Identifier, Instruction};
use mori_core::sherlock::helper_machine::c6_phys::{
    C6_PHYS_MAGIC, C6_PHYS_VERSION, FRAME_LANES, IMAGE_SECTION_HEADER_LANES,
    L_CONSUMED_WORK, L_CONTROL, L_FRAME_COUNT, L_HELPER_WORK, L_OUTCOME,
    L_REFERENCE_CURSOR, L_REMAINING_WORK, L_CONTROL_ORIGIN, L_ENV_ROOT, L_STACK_LIMIT,
    L_MAGIC, L_VERSION, L_ACTIVE, L_ALLOC, L_SEG_ACTIVE, L_SEG_ALLOC, CONTROL_LANES, C6PhysCaps,
};
use mori_core::sherlock::helper_machine::c6_phys_helper::{L_HELPER_RECORD, L_RESULT_RECORD};
use mori_core::sherlock::helper_machine::c6_phys_host::{CTX_LANES, C6_HOST_SCHEMA};
use thiserror::Error;

use super::c6_data_canonical::{C6DataCanonicalEmitter, C6DataCanonicalInputs};
use super::c6_helper_canonical::{C6HelperCanonicalEmitter, C6HelperCanonicalInputs};
use super::c6_meta_canonical::{C6MetaBoundCanonicalInputs, C6MetaCanonicalEmitter};
use super::c6_program_context_canonical::{
    C6ProgramContextCanonicalEmitter, C6ProgramContextCanonicalInputs,
    C6ProgramContextCtxBindingInputs,
};
use super::c6_push_step::{
    C6PaddedCarrierGeometry, C6PushFamilyParams, C6PushStepError, C6_PADDED_CARRIER_VERSION,
};

const LANE_BITS: u32 = 128;
// Legacy completion preserves helper_work and helper_visits, even at the helper limit.

#[derive(Debug, Error)]
pub enum C6ReturnCallStepError {
    #[error("invalid scope identifier: must not be empty")]
    EmptyScope,
    #[error("invalid scope identifier: scope must not contain '%' or a reserved composed segment")]
    InvalidScope,
    #[error("operand uses reserved parent temporary identifier: {identifier}")]
    ReservedOperand { identifier: String },
    #[error("lane bit width {bits} is invalid for the BLS12-381 scalar field")]
    InvalidBitWidth { bits: u32 },
    #[error("geometry arithmetic overflow ({section})")]
    GeometryOverflow { section: &'static str },
    #[error("length mismatch in {section}: expected {expected}, got {actual}")]
    LengthMismatch { section: &'static str, expected: usize, actual: usize },
    #[error("core C6 physical/push error: {0}")]
    Push(#[from] C6PushStepError),
    #[error("composed canonical emitter or SDK backend error: {0}")]
    Backend(String),
}

fn be<E: std::fmt::Display>(e: E) -> C6ReturnCallStepError {
    C6ReturnCallStepError::Backend(e.to_string())
}

const COMPOSED_RESERVED_SEGMENTS: [&str; 5] = [
    ".local.temp.", ".c6data.temp.", ".c6helper.temp.", ".c6meta.temp.", ".c6progctx.temp.",
];

fn validate_parent_scope(scope: &str) -> Result<(), C6ReturnCallStepError> {
    if scope.trim().is_empty() { return Err(C6ReturnCallStepError::EmptyScope); }
    if scope.contains('%') || COMPOSED_RESERVED_SEGMENTS.iter().any(|s| scope.contains(s)) {
        return Err(C6ReturnCallStepError::InvalidScope);
    }
    Ok(())
}

fn reject_reserved_operand(scope: &str, op: &Operand) -> Result<(), C6ReturnCallStepError> {
    if let Operand::Variable(id) = op {
        for seg in COMPOSED_RESERVED_SEGMENTS {
            if id.0.starts_with(&format!("%{}{}", scope, seg)) {
                return Err(C6ReturnCallStepError::ReservedOperand { identifier: id.0.clone() });
            }
        }
    }
    Ok(())
}

#[derive(Clone, Debug)]
pub struct C6ReturnCallStepInputs {
    pub enable: Operand,
    /// Statement selector equals the actual staged ScalarTypeTest.ok lane under enable.
    pub selector_is_valid: Operand,
    pub host_schema: Operand,
    pub carrier_version: Operand,
    pub geometry: C6PaddedCarrierGeometry,
    pub meta_bounds_frames_offset: usize,
    pub source_params: C6PushFamilyParams,
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

/// The block exposes actual instructions only; no ungated semantic receipts.
#[derive(Clone, Debug)]
pub struct C6ReturnCallStepBlock { pub instructions: Vec<Instruction> }

struct Em<'a> { scope: &'a str, next: usize, ins: Vec<Instruction> }
impl<'a> Em<'a> {
    fn imm(v: u128) -> Operand { Operand::Immediate(Fr::from(v)) }
    fn t(&mut self) -> Identifier {
        let id = Identifier(format!("%{}.local.temp.{}", self.scope, self.next));
        self.next += 1; id
    }
    fn cbool(&mut self, v: Operand) { self.ins.push(Instruction::ConstrainToBoolean { val: v }); }
    fn cbits(&mut self, v: Operand) { self.ins.push(Instruction::ConstrainBits { val: v, bits: LANE_BITS }); }
    fn ceq(&mut self, a: Operand, b: Operand) { self.ins.push(Instruction::ConstrainEq { a, b }); }
    fn teq(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.t(); self.ins.push(Instruction::TestEq { a, b, output: o.clone() }); Operand::Variable(o)
    }
    fn lt128(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.t(); self.ins.push(Instruction::LessThan { a, b, bits: LANE_BITS, output: o.clone() }); Operand::Variable(o)
    }
    fn add(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.t(); self.ins.push(Instruction::Add { a, b, output: o.clone() }); Operand::Variable(o)
    }
    fn mul(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.t(); self.ins.push(Instruction::Mul { a, b, output: o.clone() }); Operand::Variable(o)
    }
    fn not(&mut self, a: Operand) -> Operand {
        let o = self.t(); self.ins.push(Instruction::Not { a, output: o.clone() }); Operand::Variable(o)
    }
    fn sel(&mut self, bit: Operand, a: Operand, b: Operand) -> Operand {
        let o = self.t(); self.ins.push(Instruction::CondSelect { bit, a, b, output: o.clone() }); Operand::Variable(o)
    }
    fn assert_t(&mut self, c: Operand) { self.ins.push(Instruction::Assert { cond: c }); }
    fn geq(&mut self, g: Operand, a: Operand, b: Operand) { let s = self.sel(g, a, b.clone()); self.ceq(s, b); }
    fn gassert(&mut self, g: Operand, c: Operand) { let e = self.sel(g, c, Self::imm(1)); self.assert_t(e); }
    fn gzero(&mut self, g: Operand, v: Operand) { self.geq(g, v, Self::imm(0)); }
    fn and(&mut self, a: Operand, b: Operand) -> Operand { self.mul(a, b) }
    fn or(&mut self, a: Operand, b: Operand) -> Operand {
        let na = self.not(a); let nb = self.not(b); let both = self.mul(na, nb); self.not(both)
    }
    /// Strict index bound: `value < cap`.
    fn gated_lt_index(&mut self, gate: Operand, val: Operand, cap: usize) {
        let lt = self.lt128(val, Self::imm(cap as u128));
        self.gassert(gate, lt);
    }
    /// Inclusive count/length/end bound: `value < cap + 1`, checked host-side.
    fn gated_lt_count(&mut self, gate: Operand, val: Operand, cap: usize) {
        debug_assert!(cap < usize::MAX);
        let lt = self.lt128(val, Self::imm((cap as u128) + 1));
        self.gassert(gate, lt);
    }
    fn gated_boolean(&mut self, gate: Operand, val: Operand) {
        let checked = self.sel(gate, val, Self::imm(0));
        self.cbool(checked);
    }
    /// `EnvRootC` lanes: present is boolean; absent => index 0; present => index < env cap.
    fn gated_validate_env(&mut self, gate: Operand, present: Operand, index: Operand, env_cap: usize) {
        self.gated_boolean(gate.clone(), present.clone());
        let is_absent = self.teq(present.clone(), Self::imm(0));
        let absent_gate = self.mul(gate.clone(), is_absent);
        self.geq(absent_gate, index.clone(), Self::imm(0));
        let is_present = self.teq(present, Self::imm(1));
        let present_gate = self.mul(gate, is_present);
        self.gated_lt_index(present_gate, index, env_cap);
    }
    fn gated_validate_ref(&mut self, gate: Operand, lanes: &[Operand], at: usize, caps: &C6PhysCaps) {
        self.gated_boolean(gate.clone(), lanes[at].clone());
        let is_data = self.teq(lanes[at].clone(), Self::imm(1));
        let cap = self.sel(is_data, Self::imm(caps.max_data_nodes as u128), Self::imm(caps.max_constants as u128));
        let lt = self.lt128(lanes[at + 1].clone(), cap);
        self.gassert(gate, lt);
    }
    fn less_than_129(&mut self, a: Operand, b: Operand) -> Operand {
        let out = self.t();
        self.ins.push(Instruction::LessThan { a, b, bits: 129, output: out.clone() });
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
        self.gassert(gate, lt);
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
        let mut tag_bits = Vec::with_capacity(10);
        let mut tag_sum = Self::imm(0);
        for tag in 0..10 {
            let is_tag = self.teq(frame[0].clone(), Self::imm(tag as u128));
            tag_sum = self.add(tag_sum, is_tag.clone());
            tag_bits.push(is_tag);
        }
        self.geq(gate.clone(), tag_sum, Self::imm(1));
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
        const USED: [usize; 10] = [6, 6, 5, 3, 6, 6, 9, 5, 4, 13];
        for lane in 1..FRAME_LANES {
            let mut must_zero = Self::imm(0);
            let mut any = false;
            for (tag, used) in USED.iter().enumerate() {
                if lane >= *used { must_zero = self.add(must_zero, tag_bits[tag].clone()); any = true; }
            }
            if any {
                let pad_gate = self.mul(gate.clone(), must_zero);
                self.geq(pad_gate, frame[lane].clone(), Self::imm(0));
            }
        }
    }
}

/// Retained-serial composition API; see module docs. The free
/// function `emit_c6_return_call_step_block` is single-shot per
/// scope per circuit.
pub struct C6ReturnCallStepEmitter {
    scope: String,
    next_temp: usize,
    data_em: C6DataCanonicalEmitter,
    meta_em: C6MetaCanonicalEmitter,
    progctx_em: C6ProgramContextCanonicalEmitter,
    helper_em: C6HelperCanonicalEmitter,
}

impl C6ReturnCallStepEmitter {
    pub fn new(scope: &str) -> Result<Self, C6ReturnCallStepError> {
        validate_parent_scope(scope)?;
        Ok(Self {
            scope: scope.to_string(),
            next_temp: 0,
            data_em: C6DataCanonicalEmitter::new(scope).map_err(be)?,
            meta_em: C6MetaCanonicalEmitter::new(scope).map_err(be)?,
            progctx_em: C6ProgramContextCanonicalEmitter::new(scope).map_err(be)?,
            helper_em: C6HelperCanonicalEmitter::new(scope).map_err(be)?,
        })
    }
    pub fn scope(&self) -> &str { &self.scope }
    pub fn next_temp(&self) -> usize { self.next_temp }

    pub fn emit(&mut self, inputs: &C6ReturnCallStepInputs) -> Result<C6ReturnCallStepBlock, C6ReturnCallStepError> {
        let scope: &str = &self.scope;
        validate_parent_scope(scope)?;
        if 129usize >= FR_BITS { return Err(C6ReturnCallStepError::InvalidBitWidth { bits: 129 }); }
        for op in [&inputs.enable, &inputs.selector_is_valid, &inputs.host_schema, &inputs.carrier_version] {
            reject_reserved_operand(scope, op)?;
        }
        for op in inputs.pre_meta.iter().chain(inputs.post_meta.iter())
            .chain(inputs.pre_prog.iter()).chain(inputs.post_prog.iter())
            .chain(inputs.pre_ctx.iter()).chain(inputs.post_ctx.iter())
            .chain(inputs.pre_image.iter()).chain(inputs.post_image.iter())
            .chain(inputs.pre_data.iter()).chain(inputs.post_data.iter())
            .chain(inputs.pre_helper.iter()).chain(inputs.post_helper.iter()) {
            reject_reserved_operand(scope, op)?;
        }
        inputs.source_params.validate_geometry(inputs.geometry)?;
        let g = &inputs.geometry;
        let caps = &g.caps; let p = &caps.phys;
        for cap in [p.max_frames, p.max_expressions, p.max_names, p.max_arms, p.max_children,
            p.max_value_slots, p.max_constants, p.max_data_nodes, p.max_env_cells, p.max_functions,
            p.max_arity, p.max_types, p.max_constructors, p.max_ctor_fields, caps.max_data_field_slots,
            caps.max_helper_tasks] {
            if cap == usize::MAX { return Err(C6ReturnCallStepError::GeometryOverflow { section: "caps" }); }
        }
        for (s, a, e) in [("image", inputs.pre_image.len(), g.image_total_lanes),
            ("image", inputs.post_image.len(), g.image_total_lanes),
            ("data", inputs.pre_data.len(), g.data_total_lanes),
            ("data", inputs.post_data.len(), g.data_total_lanes),
            ("helper", inputs.pre_helper.len(), g.helper_total_lanes),
            ("helper", inputs.post_helper.len(), g.helper_total_lanes)] {
            if a != e { return Err(C6ReturnCallStepError::LengthMismatch { section: s, expected: e, actual: a }); }
        }
        if inputs.pre_meta.len() < 3 || inputs.pre_meta.len() != inputs.post_meta.len()
            || inputs.pre_prog.len() != inputs.post_prog.len()
            || inputs.meta_bounds_frames_offset >= inputs.pre_meta.len() {
            return Err(C6ReturnCallStepError::Backend("host prefix length/offset".into()));
        }
        let rec = L_HELPER_RECORD; let res = L_RESULT_RECORD;

        let mut em = Em { scope, next: self.next_temp, ins: Vec::new() };
        let en = inputs.enable.clone();
        let valid = inputs.selector_is_valid.clone();
        em.cbool(en.clone()); em.cbool(valid.clone());
        let disabled = em.not(en.clone());
        em.ceq(inputs.host_schema.clone(), Em::imm(C6_HOST_SCHEMA));
        em.ceq(inputs.carrier_version.clone(), Em::imm(C6_PADDED_CARRIER_VERSION));
        for (a, b) in inputs.pre_meta.iter().zip(inputs.post_meta.iter())
            .chain(inputs.pre_prog.iter().zip(inputs.post_prog.iter())) {
            em.cbits(a.clone()); em.cbits(b.clone()); em.ceq(b.clone(), a.clone());
        }
        for (a, b) in inputs.pre_ctx.iter().zip(inputs.post_ctx.iter()) {
            em.cbits(a.clone()); em.cbits(b.clone()); em.ceq(b.clone(), a.clone());
        }
        em.geq(en.clone(), inputs.pre_meta[0].clone(), Em::imm(C6_PHYS_MAGIC));
        em.geq(en.clone(), inputs.pre_meta[1].clone(), Em::imm(C6_PHYS_VERSION));
        em.geq(en.clone(), inputs.pre_ctx[0].clone(), Em::imm(C6_PHYS_MAGIC));
        em.geq(en.clone(), inputs.pre_ctx[1].clone(), Em::imm(C6_PHYS_VERSION));
        for l in inputs.pre_image.iter().chain(inputs.post_image.iter())
            .chain(inputs.pre_data.iter()).chain(inputs.post_data.iter())
            .chain(inputs.pre_helper.iter()).chain(inputs.post_helper.iter()) {
            em.cbits(l.clone());
        }
        let pre_i = &inputs.pre_image; let post_i = &inputs.post_image;
        let pre_h = &inputs.pre_helper; let post_h = &inputs.post_helper;

        // PS: exact selected source state. This is LegacyComplete, not helper-eligible.
        em.gzero(en.clone(), pre_h[rec].clone());
        em.geq(en.clone(), pre_h[res].clone(), Em::imm(13)); // ScalarTypeTest
        em.geq(en.clone(), valid.clone(), pre_h[res + 1].clone());
        for off in 0..3 { em.gzero(en.clone(), pre_i[L_OUTCOME + off].clone()); }
        let fuel = em.lt128(Em::imm(0), pre_i[L_REMAINING_WORK].clone());
        em.gassert(en.clone(), fuel);
        em.geq(en.clone(), pre_i[L_MAGIC].clone(), Em::imm(C6_PHYS_MAGIC));
        em.geq(en.clone(), pre_i[L_VERSION].clone(), Em::imm(C6_PHYS_VERSION));
        em.geq(en.clone(), pre_i[L_CONTROL].clone(), Em::imm(1)); // Return
        em.gated_validate_ref(en.clone(), pre_i, L_CONTROL + 1, p);
        for off in 3..CONTROL_LANES { em.gzero(en.clone(), pre_i[L_CONTROL + off].clone()); }
        em.gated_lt_index(en.clone(), pre_i[L_CONTROL_ORIGIN].clone(), p.max_expressions);
        em.gated_validate_env(en.clone(), pre_i[L_ENV_ROOT].clone(), pre_i[L_ENV_ROOT + 1].clone(), p.max_env_cells);
        em.gated_lt_count(en.clone(), pre_i[L_STACK_LIMIT].clone(), p.max_frames);
        // Representation root counts, plus exact DATA active table lengths.
        for (at,cap) in [
            (L_ALLOC,p.max_data_nodes),(L_ALLOC+1,caps.max_data_field_slots),(L_ALLOC+2,p.max_env_cells),
            (L_SEG_ALLOC,p.max_data_nodes),(L_SEG_ALLOC+1,caps.max_data_field_slots),(L_SEG_ALLOC+2,p.max_env_cells),
            (L_SEG_ACTIVE,p.max_data_nodes),(L_SEG_ACTIVE+1,caps.max_data_field_slots),
            (L_SEG_ACTIVE+2,p.max_env_cells),(L_SEG_ACTIVE+3,p.max_value_slots)] {
            em.gated_lt_count(en.clone(),pre_i[at].clone(),cap);
        }
        for off in 0..4 {
            em.geq(en.clone(),pre_i[L_ACTIVE+off].clone(),inputs.pre_data[3+off].clone());
        }
        let depth=pre_i[L_FRAME_COUNT].clone();
        let nonempty=em.lt128(Em::imm(0),depth.clone());em.gassert(en.clone(),nonempty);
        em.gated_lt_count(en.clone(),depth.clone(),p.max_frames);
        let fits_limit=em.lt128(pre_i[L_STACK_LIMIT].clone(),depth.clone());
        let fits_limit=em.not(fits_limit);em.gassert(en.clone(),fits_limit);
        let mut top=vec![Em::imm(0);FRAME_LANES];
        for slot in 0..p.max_frames {
            let base=IMAGE_SECTION_HEADER_LANES+slot*FRAME_LANES;
            let active=em.lt128(Em::imm(slot as u128),depth.clone());
            let ga=em.and(en.clone(),active.clone());
            em.gated_validate_frame(ga,&pre_i[base..base+FRAME_LANES],p);
            let inactive=em.not(active);let gi=em.and(en.clone(),inactive);
            let hit=em.teq(Em::imm(slot as u128+1),depth.clone());
            for lane in 0..FRAME_LANES {
                em.gzero(gi.clone(),pre_i[base+lane].clone());
                top[lane]=em.sel(hit.clone(),pre_i[base+lane].clone(),top[lane].clone());
            }
        }
        em.geq(en.clone(),top[0].clone(),Em::imm(7)); // exact ReturnF
        // TypeId source fact: primitive IDs 0..3 exist even with type_rows=0.
        // Full top validation above uses max(max_types,4), not a count bound.
        let apply_valid=em.and(en.clone(),valid.clone());
        // Both LegacyComplete branches charge exactly work1/refcursor1, helper0/visits0.
        let dec=em.add(post_i[L_REMAINING_WORK].clone(),Em::imm(1));
        em.geq(en.clone(),dec,pre_i[L_REMAINING_WORK].clone());
        for lane in [L_CONSUMED_WORK,L_REFERENCE_CURSOR] {
            let no_overflow=em.lt128(pre_i[lane].clone(),Em::imm(u128::MAX));
            em.gassert(en.clone(),no_overflow);
            let inc=em.add(pre_i[lane].clone(),Em::imm(1));
            em.geq(en.clone(),post_i[lane].clone(),inc);
        }
        // depth_post + valid = depth_pre; all lanes unsigned128, no field wrap.
        let depth_plus=em.add(post_i[L_FRAME_COUNT].clone(),valid.clone());
        em.geq(en.clone(),depth_plus,depth.clone());
        for off in 0..CONTROL_LANES {
            let invalid=if off==0 { Em::imm(9) } else if off==1 { Em::imm(6) } else { Em::imm(0) };
            let want=em.sel(valid.clone(),pre_i[L_CONTROL+off].clone(),invalid);
            em.geq(en.clone(),post_i[L_CONTROL+off].clone(),want);
        }
        for (lane,restore) in [(L_ENV_ROOT,top[2].clone()),(L_ENV_ROOT+1,top[3].clone()),
            (L_CONTROL_ORIGIN,top[4].clone())] {
            let want=em.sel(valid.clone(),restore,pre_i[lane].clone());
            em.geq(en.clone(),post_i[lane].clone(),want);
        }
        for slot in 0..p.max_frames {
            let base=IMAGE_SECTION_HEADER_LANES+slot*FRAME_LANES;
            let hit=em.teq(Em::imm(slot as u128+1),depth.clone());
            let popped=em.and(apply_valid.clone(),hit);
            for lane in 0..FRAME_LANES {
                let want=em.sel(popped.clone(),Em::imm(0),pre_i[base+lane].clone());
                em.geq(en.clone(),post_i[base+lane].clone(),want);
            }
        }
        for lane in 0..IMAGE_SECTION_HEADER_LANES {
            let mutable=(L_CONTROL..L_CONTROL+CONTROL_LANES).contains(&lane)
                || [L_ENV_ROOT,L_ENV_ROOT+1,L_CONTROL_ORIGIN,L_FRAME_COUNT,
                    L_REMAINING_WORK,L_CONSUMED_WORK,L_REFERENCE_CURSOR].contains(&lane);
            if !mutable { em.geq(en.clone(),post_i[lane].clone(),pre_i[lane].clone()); }
        }
        // Neither branch allocates, rewrites data, tasks or helper. Only result clears.
        for (a,b) in inputs.pre_data.iter().zip(inputs.post_data.iter()) { em.ceq(b.clone(),a.clone()); }
        for lane in 0..g.helper_total_lanes {
            let want=if (res..res+4).contains(&lane) { Em::imm(0) } else { pre_h[lane].clone() };
            em.geq(en.clone(),post_h[lane].clone(),want);
        }

        // ---- Composed canonical blocks (retained serials) ----
        let mb_block = self.meta_em.emit_bound(&C6MetaBoundCanonicalInputs {
            gate: en.clone(), pre_meta: inputs.pre_meta.clone(), post_meta: inputs.post_meta.clone(),
            limits: inputs.source_params.limits(),
            meta_bounds_frames_offset: inputs.meta_bounds_frames_offset,
            binding: inputs.source_params.meta_binding(),
        }).map_err(be)?;
        em.ins.extend(mb_block.instructions);
        let pc = self.progctx_em.emit(&C6ProgramContextCanonicalInputs {
            gate: en.clone(), pre_prog: inputs.pre_prog.clone(), pre_ctx: inputs.pre_ctx.clone(), caps: *caps,
        }).map_err(be)?;
        em.ins.extend(pc.instructions);
        let cx = self.progctx_em.emit_ctx_binding(&C6ProgramContextCtxBindingInputs {
            gate: en.clone(), pre_ctx: inputs.pre_ctx.clone(),
            expected_ctx: inputs.source_params.expected_ctx(),
        }).map_err(be)?;
        em.ins.extend(cx.instructions);
        let db = self.data_em.emit(&C6DataCanonicalInputs {
            gate: en.clone(), data: inputs.pre_data.clone(), caps: *caps }).map_err(be)?;
        em.ins.extend(db.instructions);
        let hb1 = self.helper_em.emit(&C6HelperCanonicalInputs {
            gate: en.clone(), helper: inputs.pre_helper.clone(), caps: *caps }).map_err(be)?;
        em.ins.extend(hb1.instructions);
        let hb2 = self.helper_em.emit(&C6HelperCanonicalInputs {
            gate: en.clone(), helper: inputs.post_helper.clone(), caps: *caps }).map_err(be)?;
        em.ins.extend(hb2.instructions);

        // Disabled: full equality in image and helper (DATA/META/
        // PROG/CTX equality above is already unconditional).
        for (a, b) in inputs.pre_image.iter().zip(inputs.post_image.iter())
            .chain(inputs.pre_helper.iter().zip(inputs.post_helper.iter())) {
            em.geq(disabled.clone(), b.clone(), a.clone());
        }
        self.next_temp = em.next;
        Ok(C6ReturnCallStepBlock { instructions: em.ins })
    }
}

/// Single-shot block at parent serial zero; see
/// `C6ReturnCallStepEmitter` for the composition contract.
pub fn emit_c6_return_call_step_block(
    inputs: &C6ReturnCallStepInputs, scope: &str,
) -> Result<C6ReturnCallStepBlock, C6ReturnCallStepError> {
    C6ReturnCallStepEmitter::new(scope)?.emit(inputs)
}
