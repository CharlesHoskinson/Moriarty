//! C6 `BHelperScalarExit`: active Scalar helper at Done with a non-error,
//! non-None staged payload (U128Value, BoolValue, PendingNode, Selector,
//! TypeTest). This is the full-C6 relation, not the narrow byte-carrier
//! `b_scalar_completion` / `b_scalar_subtraction_completion` U128 paths.
//!
//! Driver order mirrored from Core: PS selection/plan/validate/physical are
//! pre-image premises; P3 (pre.helper_work == retained admission
//! max_helper_work) is uncharged Exhausted{HelperWork} with all state
//! preserved and wins over P6; P4 charges remaining-1 / consumed+1 /
//! helper_work+1 exactly once; P6 compares logical alloc counters plus the
//! DELTA-dependent node/field deltas against the ORIGINAL SEPARATE admission
//! bounds (DataNodes before DataFields, checked against those bounds, not
//! physical-cap aliases) and on refusal only P4 persists; otherwise P7
//! appends fields BEFORE the node at the ACTIVE physical cursor, records the
//! field slice start/len from pre.active.data_fields, sets the exact result,
//! and clears the helper. Selector/TypeTest allocate nothing and cannot P6.
//!
//! DATA changes here, so unlike ScalarFailure this module canonicalises BOTH
//! pre and post DATA and constrains every affected row/field, every
//! preserved row/field/env/value-slot, both header count pairs, and the
//! image alloc/active counters explicitly. HELPER is canonicalised pre and
//! post; META is the retained bound relation; PROG/CTX keep the unconditional
//! pre/post equalities plus the immediate expected-CTX binding from the
//! retained family params. Full StepInv, SI08/SI21/SI22 denotation, program
//! membership and native P1 issuance remain required unproved bridges; no
//! emitted check below is claimed to prove them.
//!
//! Namespace contract mirrors ScalarFailure: one retained emitter owns the
//! parent `%{scope}.local.temp.*` serial and one retained serial per composed
//! canonical block (DATA, HELPER, META, PROGCTX), so repeated emit calls
//! under one scope never rebind a temporary. The free function is
//! single-shot per scope per circuit.

use midnight_transient_crypto::curve::{Fr, FR_BITS};
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::{Identifier, Instruction};
use mori_core::sherlock::helper_machine::c6_phys::{
    C6PhysCaps, C6_PHYS_MAGIC, C6_PHYS_VERSION, L_ACTIVE, L_ALLOC, L_CONSUMED_WORK,
    L_CONTROL, L_CONTROL_ORIGIN, L_ENV_ROOT, L_FRAME_COUNT, L_HELPER_WORK, L_OUTCOME,
    L_REFERENCE_CURSOR, L_REMAINING_WORK, L_STACK_LIMIT,
};
use mori_core::sherlock::helper_machine::c6_phys_data::DATA_HEADER_LANES;
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
use super::c6_push_step::{C6PaddedCarrierGeometry, C6PushFamilyParams, C6PushStepError, C6_PADDED_CARRIER_VERSION};

const LANE_BITS: u32 = 128;
const CHECK_HELPER_KIND_SCALAR: u128 = 1;
const CHECK_PHASE_DONE: u128 = 2;

#[derive(Debug, Error)]
pub enum C6ScalarExitStepError {
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

fn be<E: std::fmt::Display>(e: E) -> C6ScalarExitStepError { C6ScalarExitStepError::Backend(e.to_string()) }

const LOCAL_RESERVED_SEGMENT: &str = ".local.temp.";
const COMPOSED_RESERVED_SEGMENTS: [&str; 5] = [
    LOCAL_RESERVED_SEGMENT, ".c6data.temp.", ".c6helper.temp.", ".c6meta.temp.", ".c6progctx.temp.",
];

fn validate_parent_scope(scope: &str) -> Result<(), C6ScalarExitStepError> {
    if scope.trim().is_empty() { return Err(C6ScalarExitStepError::EmptyScope); }
    if scope.contains('%') || COMPOSED_RESERVED_SEGMENTS.iter().any(|s| scope.contains(s)) {
        return Err(C6ScalarExitStepError::InvalidScope);
    }
    Ok(())
}

fn reject_reserved_operand(scope: &str, op: &Operand) -> Result<(), C6ScalarExitStepError> {
    if let Operand::Variable(id) = op {
        for seg in COMPOSED_RESERVED_SEGMENTS {
            if id.0.starts_with(&format!("%{}{}", scope, seg)) {
                return Err(C6ScalarExitStepError::ReservedOperand { identifier: id.0.clone() });
            }
        }
    }
    Ok(())
}

fn reject_all_reserved(scope: &str, inputs: &C6ScalarExitStepInputs) -> Result<(), C6ScalarExitStepError> {
    for op in [&inputs.enable, &inputs.disposition_is_refused_helper_work, &inputs.host_schema, &inputs.carrier_version] {
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
    Ok(())
}

/// Retained-serial composition API; see module docs. The free function
/// `emit_c6_scalar_exit_step_block` is single-shot per scope per circuit.
pub struct C6ScalarExitStepEmitter {
    scope: String,
    next_temp: usize,
    data_em: C6DataCanonicalEmitter,
    meta_em: C6MetaCanonicalEmitter,
    progctx_em: C6ProgramContextCanonicalEmitter,
    helper_em: C6HelperCanonicalEmitter,
}

impl C6ScalarExitStepEmitter {
    pub fn new(scope: &str) -> Result<Self, C6ScalarExitStepError> {
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
}

#[derive(Clone, Debug)]
pub struct C6ScalarExitStepInputs {
    pub enable: Operand,
    /// Host statement lane constrained in-circuit to exactly P3
    /// (`pre.helper_work == retained admission max_helper_work`). P6 is
    /// derived in-circuit from alloc counters, deltas and admission bounds;
    /// it is not a caller flag.
    pub disposition_is_refused_helper_work: Operand,
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

#[derive(Clone, Debug)]
pub struct C6ScalarExitStepBlock {
    pub is_helper_work_max: Operand,
    pub is_cap_refused: Operand,
    pub instructions: Vec<Instruction>,
}

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
    fn glt_c(&mut self, g: Operand, v: Operand, cap: usize) {
        let l = self.lt128(v, Self::imm((cap as u128) + 1)); self.gassert(g, l);
    }
    fn lt(&mut self, a: Operand, b: Operand, bits: u32) -> Operand {
        let o = self.t(); self.ins.push(Instruction::LessThan { a, b, bits, output: o.clone() }); Operand::Variable(o)
    }
    fn lt129(&mut self, a: Operand, b: Operand) -> Operand { self.lt(a, b, 129) }
    fn gbool(&mut self, g: Operand, v: Operand) { let c = self.sel(g, v, Self::imm(0)); self.cbool(c); }
    fn glt_i(&mut self, g: Operand, v: Operand, cap: usize) {
        let l = self.lt128(v, Self::imm(cap as u128)); self.gassert(g, l);
    }
    fn glt_end(&mut self, g: Operand, end: Operand, cap: usize) {
        let l = self.lt(end, Self::imm((cap as u128) + 1), 129); self.gassert(g, l);
    }
    fn tag_bits(&mut self, g: &Operand, lane: Operand, n: u128) -> Vec<Operand> {
        let mut bits = Vec::new(); let mut sum = Self::imm(0);
        for t in 0..n { let b = self.teq(lane.clone(), Self::imm(t)); sum = self.add(sum, b.clone()); bits.push(b); }
        self.geq(g.clone(), sum, Self::imm(1)); bits
    }
    fn vref(&mut self, g: Operand, lanes: &[Operand], at: usize, c: &C6PhysCaps) {
        self.gbool(g.clone(), lanes[at].clone());
        let d = self.teq(lanes[at].clone(), Self::imm(1));
        let cap = self.sel(d, Self::imm(c.max_data_nodes as u128), Self::imm(c.max_constants as u128));
        let l = self.lt128(lanes[at + 1].clone(), cap); self.gassert(g, l);
    }
    fn venv(&mut self, g: Operand, present: Operand, idx: Operand, cap: usize) {
        self.gbool(g.clone(), present.clone());
        let a = self.teq(present.clone(), Self::imm(0));
        let ag = self.mul(g.clone(), a); self.gzero(ag, idx.clone());
        let p = self.teq(present, Self::imm(1));
        let pg = self.mul(g, p); self.glt_i(pg, idx, cap);
    }
    fn vslice(&mut self, g: Operand, lanes: &[Operand], at: usize, table: usize, max_len: Option<usize>) {
        if let Some(m) = max_len { self.glt_c(g.clone(), lanes[at + 1].clone(), m); }
        let end = self.add(lanes[at].clone(), lanes[at + 1].clone());
        self.glt_end(g, end, table);
    }
    fn vframe(&mut self, gate: Operand, f: &[Operand], c: &C6PhysCaps) {
        debug_assert_eq!(f.len(), 13);
        let bt = c.max_types.max(4);
        let bits = self.tag_bits(&gate, f[0].clone(), 10);
        let vg = |em: &mut Self, i: usize| em.mul(gate.clone(), bits[i].clone());
        { let g = vg(self, 0); self.glt_i(g.clone(), f[1].clone(), c.max_names); self.glt_i(g.clone(), f[2].clone(), c.max_expressions); self.venv(g.clone(), f[3].clone(), f[4].clone(), c.max_env_cells); self.glt_i(g, f[5].clone(), c.max_expressions); }
        { let g = vg(self, 1); self.glt_i(g.clone(), f[1].clone(), 7); self.glt_i(g.clone(), f[2].clone(), c.max_expressions); self.venv(g.clone(), f[3].clone(), f[4].clone(), c.max_env_cells); self.glt_i(g, f[5].clone(), c.max_expressions); }
        { let g = vg(self, 2); self.glt_i(g.clone(), f[1].clone(), 7); self.vref(g.clone(), f, 2, c); self.glt_i(g, f[4].clone(), c.max_expressions); }
        { let g = vg(self, 3); self.glt_i(g.clone(), f[1].clone(), 2); self.glt_i(g, f[2].clone(), c.max_expressions); }
        { let g = vg(self, 4); self.glt_i(g.clone(), f[1].clone(), c.max_expressions); self.glt_i(g.clone(), f[2].clone(), c.max_expressions); self.venv(g.clone(), f[3].clone(), f[4].clone(), c.max_env_cells); self.glt_i(g, f[5].clone(), c.max_expressions); }
        { let g = vg(self, 5); self.vslice(g.clone(), f, 1, c.max_arms, None); self.venv(g.clone(), f[3].clone(), f[4].clone(), c.max_env_cells); self.glt_i(g, f[5].clone(), c.max_expressions); }
        { let g = vg(self, 6); self.glt_i(g.clone(), f[1].clone(), c.max_names); self.vslice(g.clone(), f, 2, c.max_value_slots, Some(c.max_arity)); self.vslice(g.clone(), f, 4, c.max_children, Some(c.max_arity)); self.venv(g.clone(), f[6].clone(), f[7].clone(), c.max_env_cells); self.glt_i(g, f[8].clone(), c.max_expressions); }
        { let g = vg(self, 7); self.glt_i(g.clone(), f[1].clone(), bt); self.venv(g.clone(), f[2].clone(), f[3].clone(), c.max_env_cells); self.glt_i(g, f[4].clone(), c.max_expressions); }
        { let g = vg(self, 8); self.venv(g.clone(), f[1].clone(), f[2].clone(), c.max_env_cells); self.glt_i(g, f[3].clone(), c.max_expressions); }
        { let g = vg(self, 9); self.glt_i(g.clone(), f[1].clone(), bt); self.glt_i(g.clone(), f[2].clone(), c.max_constructors); self.glt_i(g.clone(), f[3].clone(), c.max_arity); self.glt_i(g.clone(), f[4].clone(), bt); self.gbool(g.clone(), f[5].clone()); self.vslice(g.clone(), f, 6, c.max_value_slots, Some(c.max_arity)); self.vslice(g.clone(), f, 8, c.max_children, Some(c.max_arity)); self.venv(g.clone(), f[10].clone(), f[11].clone(), c.max_env_cells); self.glt_i(g, f[12].clone(), c.max_expressions); }
        const USED: [usize; 10] = [6, 6, 5, 3, 6, 6, 9, 5, 4, 13];
        for lane in 1..13usize {
            let mut mz = Self::imm(0); let mut any = false;
            for (t, u) in USED.iter().enumerate() { if lane >= *u { mz = self.add(mz, bits[t].clone()); any = true; } }
            if any { let pg = self.mul(gate.clone(), mz); self.gzero(pg, f[lane].clone()); }
        }
    }
    fn vcontrol(&mut self, gate: Operand, k: &[Operand], c: &C6PhysCaps) {
        debug_assert_eq!(k.len(), 17);
        let bt = c.max_types.max(4);
        let bits = self.tag_bits(&gate, k[0].clone(), 11);
        let vg = |em: &mut Self, i: usize| em.mul(gate.clone(), bits[i].clone());
        { let g = vg(self, 0); self.glt_i(g, k[1].clone(), c.max_expressions); }
        { let g = vg(self, 1); self.vref(g, k, 1, c); }
        { let g = vg(self, 2); let fr: Vec<Operand> = k[1..14].to_vec(); self.vframe(g.clone(), &fr, c); self.glt_i(g.clone(), k[14].clone(), c.max_expressions); self.venv(g, k[15].clone(), k[16].clone(), c.max_env_cells); }
        { let g = vg(self, 3); self.glt_i(g.clone(), k[1].clone(), c.max_names); self.vslice(g, k, 2, c.max_value_slots, Some(c.max_arity)); }
        { let g = vg(self, 4); self.gbool(g.clone(), k[1].clone()); let a = self.teq(k[1].clone(), Self::imm(0)); let ag = self.mul(g.clone(), a); self.gzero(ag, k[2].clone()); let p = self.teq(k[1].clone(), Self::imm(1)); let pg = self.mul(g.clone(), p); self.glt_i(pg, k[2].clone(), c.max_functions); self.vslice(g, k, 3, c.max_value_slots, Some(c.max_arity)); }
        { let g = vg(self, 5); self.gbool(g.clone(), k[1].clone()); let a = self.teq(k[1].clone(), Self::imm(0)); let ag = self.mul(g.clone(), a); self.gzero(ag.clone(), k[2].clone()); self.gzero(ag, k[3].clone()); self.venv(g.clone(), k[2].clone(), k[3].clone(), c.max_env_cells); self.glt_i(g.clone(), k[4].clone(), bt); self.glt_i(g, k[5].clone(), c.max_expressions); }
        { let g = vg(self, 6); self.vref(g.clone(), k, 1, c); self.vslice(g.clone(), k, 3, c.max_arms, None); self.venv(g, k[5].clone(), k[6].clone(), c.max_env_cells); }
        { let g = vg(self, 7); self.gbool(g.clone(), k[1].clone()); let a = self.teq(k[1].clone(), Self::imm(0)); let ag = self.mul(g.clone(), a); self.gzero(ag.clone(), k[2].clone()); self.gzero(ag, k[3].clone()); self.venv(g.clone(), k[2].clone(), k[3].clone(), c.max_env_cells); self.vref(g.clone(), k, 4, c); self.glt_i(g.clone(), k[6].clone(), c.max_expressions); self.vslice(g.clone(), k, 7, c.max_arms, None); self.venv(g, k[9].clone(), k[10].clone(), c.max_env_cells); }
        { let g = vg(self, 8); self.gbool(g.clone(), k[1].clone()); let e = self.teq(k[1].clone(), Self::imm(1)); let eg = self.mul(g.clone(), e.clone()); self.glt_i(eg.clone(), k[2].clone(), 256); self.gzero(eg, k[3].clone()); let v = self.not(e); let vlg = self.mul(g, v); self.vref(vlg, k, 2, c); }
        { let g = vg(self, 9); self.glt_i(g, k[1].clone(), 256); }
        { let g = vg(self, 10); self.glt_i(g.clone(), k[1].clone(), bt); self.glt_i(g.clone(), k[2].clone(), c.max_constructors); self.vslice(g, k, 3, c.max_value_slots, Some(c.max_arity)); }
        const USED: [usize; 11] = [2, 3, 17, 4, 5, 6, 7, 11, 4, 2, 5];
        for lane in 1..17usize {
            let mut mz = Self::imm(0); let mut any = false;
            for (t, u) in USED.iter().enumerate() { if lane >= *u { mz = self.add(mz, bits[t].clone()); any = true; } }
            if any { let pg = self.mul(gate.clone(), mz); self.gzero(pg, k[lane].clone()); }
        }
    }
}

/// Single-shot block at parent serial zero; see `C6ScalarExitStepEmitter`
/// for the composition contract.
pub fn emit_c6_scalar_exit_step_block(
    inputs: &C6ScalarExitStepInputs, scope: &str,
) -> Result<C6ScalarExitStepBlock, C6ScalarExitStepError> {
    C6ScalarExitStepEmitter::new(scope)?.emit(inputs)
}

impl C6ScalarExitStepEmitter {
    pub fn emit(&mut self, inputs: &C6ScalarExitStepInputs) -> Result<C6ScalarExitStepBlock, C6ScalarExitStepError> {
        let scope: &str = &self.scope;
        validate_parent_scope(scope)?;
        reject_all_reserved(scope, inputs)?;
        if (LANE_BITS as usize) >= FR_BITS { return Err(C6ScalarExitStepError::InvalidBitWidth { bits: LANE_BITS }); }
        inputs.source_params.validate_geometry(inputs.geometry)?;
        let g = &inputs.geometry;
        let caps = &g.caps; let p = &caps.phys;
        for cap in [p.max_frames, p.max_expressions, p.max_names, p.max_arms, p.max_children,
            p.max_value_slots, p.max_constants, p.max_data_nodes, p.max_env_cells, p.max_functions,
            p.max_arity, p.max_types, p.max_constructors, caps.max_data_field_slots, caps.max_helper_tasks] {
            if cap == usize::MAX { return Err(C6ScalarExitStepError::GeometryOverflow { section: "caps" }); }
        }
        for (s, a, e) in [("image", inputs.pre_image.len(), g.image_total_lanes),
            ("image", inputs.post_image.len(), g.image_total_lanes),
            ("data", inputs.pre_data.len(), g.data_total_lanes),
            ("data", inputs.post_data.len(), g.data_total_lanes),
            ("helper", inputs.pre_helper.len(), g.helper_total_lanes),
            ("helper", inputs.post_helper.len(), g.helper_total_lanes)] {
            if a != e { return Err(C6ScalarExitStepError::LengthMismatch { section: s, expected: e, actual: a }); }
        }
        if inputs.pre_meta.len() < 3 || inputs.pre_meta.len() != inputs.post_meta.len()
            || inputs.pre_prog.len() != inputs.post_prog.len()
            || inputs.meta_bounds_frames_offset >= inputs.pre_meta.len() {
            return Err(C6ScalarExitStepError::Backend("host prefix length/offset".into()));
        }
        let adm = inputs.source_params.admission_bounds();
        if adm.data_nodes == usize::MAX || adm.data_field_slots == usize::MAX {
            return Err(C6ScalarExitStepError::GeometryOverflow { section: "admission bounds" });
        }
        let max_hw = adm.max_helper_work;

        let mut em = Em { scope, next: self.next_temp, ins: Vec::new() };
        let en = inputs.enable.clone();
        let dp = inputs.disposition_is_refused_helper_work.clone();
        em.cbool(en.clone()); em.cbool(dp.clone());
        let disabled = em.not(en.clone());
        em.ceq(inputs.host_schema.clone(), Em::imm(C6_HOST_SCHEMA));
        em.ceq(inputs.carrier_version.clone(), Em::imm(C6_PADDED_CARRIER_VERSION));

        // Host prefix: unconditional ranges and pre == post (META/PROG/CTX
        // canonicality and bindings are composed below).
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
        let pre_d = &inputs.pre_data; let post_d = &inputs.post_data;
        let pre_h = &inputs.pre_helper; let post_h = &inputs.post_helper;
        let rec = L_HELPER_RECORD; let res = L_RESULT_RECORD;

        // ---- Selected pre-state (Scalar / Done / non-error staged) ----
        em.geq(en.clone(), pre_h[rec].clone(), Em::imm(CHECK_HELPER_KIND_SCALAR));
        em.geq(en.clone(), pre_h[rec + 13].clone(), Em::imm(CHECK_PHASE_DONE));
        em.gzero(en.clone(), pre_h[res].clone()); // pre result None
        em.geq(en.clone(), pre_h[3].clone(), Em::imm(0)); // task count 0
        em.gzero(en.clone(), pre_h[rec + 3].clone()); // parent_resume absent
        em.geq(en.clone(), pre_h[rec + 2].clone(), pre_i[L_REFERENCE_CURSOR].clone());
        { // site in {RetRight 11, RetUnary 12, RetIf 13, RetCall 16, RetCtorFields 25}
            let mut s = Em::imm(0);
            for v in [11u128, 12, 13, 16, 25] {
                let b = em.teq(pre_h[rec + 1].clone(), Em::imm(v)); s = em.add(s, b);
            }
            em.geq(en.clone(), s, Em::imm(1));
        }
        { // operand_b present iff mode in 0..=5; expected present iff mode == 9
            let mut bin = Em::imm(0);
            for v in 0..6u128 { let b = em.teq(pre_h[rec + 5].clone(), Em::imm(v)); bin = em.add(bin, b); }
            em.geq(en.clone(), pre_h[rec + 8].clone(), bin);
            let tt = em.teq(pre_h[rec + 5].clone(), Em::imm(9));
            em.geq(en.clone(), pre_h[rec + 11].clone(), tt);
        }
        // Staged tag bits: 1 U128, 2 Bool, 3 PendingNode, 5 Selector, 6 TypeTest.
        // Tag 0 (None, plan-rejected) and 4 (PrimError, ScalarFailure) excluded.
        let stag = pre_h[rec + 15].clone();
        let spay = pre_h[rec + 16].clone();
        let t_u128 = em.teq(stag.clone(), Em::imm(1));
        let t_bool = em.teq(stag.clone(), Em::imm(2));
        let t_pend = em.teq(stag.clone(), Em::imm(3));
        let t_sel  = em.teq(stag.clone(), Em::imm(5));
        let t_test = em.teq(stag.clone(), Em::imm(6));
        // Mode/staged coherence (SI22 Done shape): U128 <- Add/Sub/Mul;
        // Bool <- Lt/Not; Pending <- Cons/Branch/Leaf with matching node tag
        // (Cons 0, Leaf 1, Branch 2); Selector <- TestBool; TypeTest <- TestType.
        let mode = pre_h[rec + 5].clone();
        let coh = {
            let mut sum = Em::imm(0);
            for v in 0..3u128 { let mb = em.teq(mode.clone(), Em::imm(v)); let c = em.mul(t_u128.clone(), mb); sum = em.add(sum, c); }
            for v in [3u128, 7] { let mb = em.teq(mode.clone(), Em::imm(v)); let c = em.mul(t_bool.clone(), mb); sum = em.add(sum, c); }
            for (m, n) in [(4u128, 0u128), (5, 2), (6, 1)] {
                let mb = em.teq(mode.clone(), Em::imm(m)); let nb = em.teq(spay.clone(), Em::imm(n));
                let mn = em.mul(mb, nb); let c = em.mul(t_pend.clone(), mn); sum = em.add(sum, c);
            }
            let mb = em.teq(mode.clone(), Em::imm(8)); let c = em.mul(t_sel.clone(), mb); sum = em.add(sum, c);
            let mb = em.teq(mode.clone(), Em::imm(9)); let c = em.mul(t_test.clone(), mb); sum = em.add(sum, c);
            sum
        };
        em.geq(en.clone(), coh, Em::imm(1));

        // ---- Deltas (DELTA-dependent, from staged branch) ----
        let is_alloc = { let s = em.add(t_u128.clone(), t_bool.clone()); em.add(s, t_pend.clone()) };
        let node_delta = is_alloc.clone();
        let pay_leaf = em.teq(spay.clone(), Em::imm(1));
        let pay_cons = em.teq(spay.clone(), Em::imm(0));
        let pay_branch = em.teq(spay.clone(), Em::imm(2));
        let cb = em.add(pay_cons.clone(), pay_branch.clone());
        let field_delta = { // Leaf 1, Cons/Branch 2, else 0 (gated by t_pend via pay coherence)
            let leaf_part = em.mul(t_pend.clone(), pay_leaf.clone());
            let cb_part = { let d = em.add(cb.clone(), cb.clone()); em.mul(t_pend.clone(), d) };
            em.add(leaf_part, cb_part)
        };

        // ---- Pre-image root checks needed by this row ----
        em.geq(en.clone(), pre_i[0].clone(), Em::imm(C6_PHYS_MAGIC));
        em.geq(en.clone(), pre_i[1].clone(), Em::imm(C6_PHYS_VERSION));
        // Full physical image canonicality on the pre-image (10 frames,
        // 11 controls, env root, origin, stack limit, seg counters).
        // Post-image frames/control follow by the exact preservation of
        // every non-mutable lane below; see source-faithfulness note.
        { let ctl: Vec<Operand> = pre_i[L_CONTROL..L_CONTROL + 17].to_vec(); em.vcontrol(en.clone(), &ctl, p); }
        em.glt_i(en.clone(), pre_i[L_CONTROL_ORIGIN].clone(), p.max_expressions);
        em.venv(en.clone(), pre_i[L_ENV_ROOT].clone(), pre_i[L_ENV_ROOT + 1].clone(), p.max_env_cells);
        em.glt_c(en.clone(), pre_i[L_STACK_LIMIT].clone(), p.max_frames);
        for (lane, cap) in [(32usize, p.max_data_nodes), (34, p.max_env_cells), (35, p.max_data_nodes),
            (37, p.max_env_cells), (38, p.max_value_slots), (39, p.max_data_nodes),
            (41, p.max_env_cells), (42, p.max_value_slots),
            (43, p.max_data_nodes), (45, p.max_env_cells)] {
            em.glt_c(en.clone(), pre_i[lane].clone(), cap);
        }
        em.glt_c(en.clone(), pre_i[L_FRAME_COUNT].clone(), p.max_frames);
        for slot in 0..p.max_frames {
            let base = g.frame_slot_offset(slot);
            let active = em.lt128(Em::imm(slot as u128), pre_i[L_FRAME_COUNT].clone());
            let inact = em.not(active.clone());
            let pg = em.mul(en.clone(), inact);
            for off in 0..13 { em.gzero(pg.clone(), pre_i[base + off].clone()); }
            let rg_gate = em.mul(en.clone(), active);
            let fr: Vec<Operand> = pre_i[base..base + 13].to_vec();
            em.vframe(rg_gate, &fr, p);
        }
        for off in 0..3 { em.gzero(en.clone(), pre_i[L_OUTCOME + off].clone()); } // Running
        { let hf = em.lt128(Em::imm(0), pre_i[L_REMAINING_WORK].clone()); em.gassert(en.clone(), hf); }
        // Alloc/active counters bounded by physical caps (makes alloc+delta
        // sums below exact in the field for the P6 comparisons).
        em.glt_c(en.clone(), pre_i[L_ALLOC].clone(), p.max_data_nodes);
        em.glt_c(en.clone(), pre_i[L_ALLOC + 1].clone(), caps.max_data_field_slots);
        em.glt_c(en.clone(), pre_i[L_ACTIVE].clone(), p.max_data_nodes);
        em.glt_c(en.clone(), pre_i[L_ACTIVE + 1].clone(), caps.max_data_field_slots);
        // Tie image active/alloc counters to the DATA header active counts.
        em.geq(en.clone(), pre_i[L_ACTIVE].clone(), pre_d[3].clone());
        em.geq(en.clone(), pre_i[L_ACTIVE + 1].clone(), pre_d[4].clone());
        em.geq(en.clone(), pre_i[L_ALLOC].clone(), pre_i[L_ACTIVE].clone());
        em.geq(en.clone(), pre_i[L_ALLOC + 1].clone(), pre_i[L_ACTIVE + 1].clone());

        // ---- P3 / P6 disposition derivation ----
        let is_p3 = em.teq(pre_i[L_HELPER_WORK].clone(), Em::imm(max_hw));
        em.geq(en.clone(), dp.clone(), is_p3.clone());
        let not_p3 = em.not(is_p3.clone());
        // Logical caps: alloc counter + delta vs ORIGINAL admission bounds.
        // Pre alloc is cap-bounded above, delta <= 2, so sums are exact.
        let node_sum = em.add(pre_i[L_ALLOC].clone(), node_delta.clone());
        let node_over = em.lt129(Em::imm(adm.data_nodes as u128), node_sum);
        let p6_node_raw = em.mul(is_alloc.clone(), node_over);
        let field_sum = em.add(pre_i[L_ALLOC + 1].clone(), field_delta.clone());
        let field_over = em.lt129(Em::imm(adm.data_field_slots as u128), field_sum);
        let not_node = em.not(p6_node_raw.clone());
        let p6_field_raw = { let f = em.mul(is_alloc.clone(), field_over); em.mul(f, not_node) };
        let p6_raw = em.add(p6_node_raw.clone(), p6_field_raw.clone());
        let is_p6 = em.mul(not_p3.clone(), p6_raw); // P3 wins simultaneously
        let p6_node = em.mul(not_p3.clone(), p6_node_raw);
        let p6_field = em.mul(not_p3.clone(), p6_field_raw);
        let refused = em.add(is_p3.clone(), is_p6.clone());
        let not_refused = em.not(refused.clone());
        let not_is_p6 = em.not(is_p6.clone());
        let applied_gate = { let a = em.mul(en.clone(), not_p3.clone()); em.mul(a, not_is_p6.clone()) };
        let charged_gate = em.mul(en.clone(), not_p3.clone());
        let applied_bit = { let a = em.mul(not_p3.clone(), not_is_p6.clone()); a };

        // ---- Post outcome ----
        let exp_o0 = em.sel(refused.clone(), Em::imm(3), Em::imm(0));
        em.geq(en.clone(), post_i[L_OUTCOME].clone(), exp_o0);
        let exp_o1_inner = em.sel(p6_node.clone(), Em::imm(1), Em::imm(2));
        let exp_o1_p6 = em.sel(is_p6.clone(), exp_o1_inner, Em::imm(0));
        let exp_o1 = em.sel(is_p3.clone(), Em::imm(5), exp_o1_p6);
        em.geq(en.clone(), post_i[L_OUTCOME + 1].clone(), exp_o1);
        em.gzero(en.clone(), post_i[L_OUTCOME + 2].clone());

        // ---- P4 counters (charged = Applied or P6; P3 uncharged) ----
        { let s = em.add(post_i[L_REMAINING_WORK].clone(), Em::imm(1));
          em.geq(charged_gate.clone(), s, pre_i[L_REMAINING_WORK].clone());
          let pg = em.mul(en.clone(), is_p3.clone());
          em.geq(pg, post_i[L_REMAINING_WORK].clone(), pre_i[L_REMAINING_WORK].clone()); }
        for lane in [L_CONSUMED_WORK, L_HELPER_WORK] {
            let noov = em.lt128(pre_i[lane].clone(), Em::imm(u128::MAX));
            em.gassert(charged_gate.clone(), noov);
            let exp = em.add(pre_i[lane].clone(), Em::imm(1));
            let exp2 = em.sel(is_p3.clone(), pre_i[lane].clone(), exp);
            em.geq(en.clone(), post_i[lane].clone(), exp2);
        }
        // ---- Image alloc/active counters: advance only on Applied ----
        for (lane, delta) in [(L_ALLOC, node_delta.clone()), (L_ALLOC + 1, field_delta.clone()),
                              (L_ACTIVE, node_delta.clone()), (L_ACTIVE + 1, field_delta.clone())] {
            let adv = em.mul(applied_bit.clone(), delta);
            let exp = em.add(pre_i[lane].clone(), adv);
            em.geq(en.clone(), post_i[lane].clone(), exp);
        }
        // Every other image lane preserved (frames, control, origin, env
        // root, initial fuel, helper visits, reference cursor, seg, stack).
        let mutable = [L_OUTCOME, L_OUTCOME + 1, L_OUTCOME + 2, L_REMAINING_WORK, L_CONSUMED_WORK,
            L_HELPER_WORK, L_ALLOC, L_ALLOC + 1, L_ACTIVE, L_ACTIVE + 1];
        for lane in 0..g.image_total_lanes {
            if !mutable.contains(&lane) {
                em.geq(en.clone(), post_i[lane].clone(), pre_i[lane].clone());
            }
        }

        // ---- DATA post ----
        // Header: counts advance only on Applied; all other header lanes equal.
        for lane in [0usize, 1, 2, 5, 6] {
            em.geq(en.clone(), post_d[lane].clone(), pre_d[lane].clone());
        }
        { let adv = em.mul(applied_bit.clone(), node_delta.clone());
          let exp = em.add(pre_d[3].clone(), adv); em.geq(en.clone(), post_d[3].clone(), exp); }
        { let adv = em.mul(applied_bit.clone(), field_delta.clone());
          let exp = em.add(pre_d[4].clone(), adv); em.geq(en.clone(), post_d[4].clone(), exp); }
        // New node row at slot == pre.active.data (alloc branches only).
        let pend_tag = { // Leaf 4, Cons 3, Branch 5
            let t = em.sel(pay_leaf.clone(), Em::imm(4), Em::imm(5));
            em.sel(pay_cons.clone(), Em::imm(3), t)
        };
        let scalar_tag = em.sel(t_bool.clone(), Em::imm(1), Em::imm(0));
        let new_row0 = em.sel(t_pend.clone(), pend_tag, scalar_tag);
        let new_row1 = { let ab = em.add(t_u128.clone(), t_bool.clone()); em.sel(ab, spay.clone(), Em::imm(0)) };
        let new_row3 = em.sel(t_pend.clone(), pre_d[4].clone(), Em::imm(0));
        let new_row4 = em.sel(t_pend.clone(), field_delta.clone(), Em::imm(0));
        let new_row = [new_row0, new_row1, Em::imm(0), new_row3, new_row4];
        for slot in 0..p.max_data_nodes {
            let base = DATA_HEADER_LANES + slot * 5;
            let is_tgt = em.teq(pre_d[3].clone(), Em::imm(slot as u128));
            let w = { let a = em.mul(applied_gate.clone(), is_tgt); em.mul(a, is_alloc.clone()) };
            for off in 0..5 {
                let exp = em.sel(w.clone(), new_row[off].clone(), pre_d[base + off].clone());
                em.geq(en.clone(), post_d[base + off].clone(), exp);
            }
        }
        // Field appends at pre.active.data_fields (+1): operand_a then operand_b.
        let fields_base = g.data_fields_base();
        let second_pos = em.add(pre_d[4].clone(), Em::imm(1));
        let oa = [pre_h[rec + 6].clone(), pre_h[rec + 7].clone()];
        let ob = [pre_h[rec + 9].clone(), pre_h[rec + 10].clone()];
        let is_cons_branch = em.mul(t_pend.clone(), cb.clone());
        for slot in 0..caps.max_data_field_slots {
            let base = fields_base + slot * 2;
            let is_first = em.teq(pre_d[4].clone(), Em::imm(slot as u128));
            let is_second = em.teq(second_pos.clone(), Em::imm(slot as u128));
            let w1 = { let a = em.mul(applied_gate.clone(), is_first); em.mul(a, t_pend.clone()) };
            let w2 = { let a = em.mul(applied_gate.clone(), is_second); em.mul(a, is_cons_branch.clone()) };
            for off in 0..2 {
                let e1 = em.sel(w1.clone(), oa[off].clone(), pre_d[base + off].clone());
                let e2 = em.sel(w2.clone(), ob[off].clone(), e1);
                em.geq(en.clone(), post_d[base + off].clone(), e2);
            }
        }
        // Env rows and value slots preserved exactly.
        for lane in g.env_base()..g.data_total_lanes {
            em.geq(en.clone(), post_d[lane].clone(), pre_d[lane].clone());
        }

        // ---- HELPER post ----
        // Applied: helper record zeroed, result = ScalarValue(Data(pre.active))
        // | ScalarSelector(sel) | ScalarTypeTest(ok); refused: pre preserved.
        let result_tag = {
            let t = em.sel(t_sel.clone(), Em::imm(12), Em::imm(13));
            em.sel(is_alloc.clone(), Em::imm(10), t)
        };
        let result_a = em.sel(is_alloc.clone(), Em::imm(1), spay.clone());
        let result_b = em.sel(is_alloc.clone(), pre_d[3].clone(), Em::imm(0));
        for lane in 0..g.helper_total_lanes {
            let applied_expected = if (rec..res).contains(&lane) {
                Em::imm(0)
            } else if lane == res {
                result_tag.clone()
            } else if lane == res + 1 {
                result_a.clone()
            } else if lane == res + 2 {
                result_b.clone()
            } else if lane == res + 3 {
                Em::imm(0)
            } else { pre_h[lane].clone() };
            let exp = em.sel(refused.clone(), pre_h[lane].clone(), applied_expected);
            em.geq(en.clone(), post_h[lane].clone(), exp);
        }

        // ---- Composed canonical blocks (retained serials) ----
        // DATA canonicality on BOTH pre and post (post DATA changes here).
        let db1 = self.data_em.emit(&C6DataCanonicalInputs { gate: en.clone(), data: inputs.pre_data.clone(), caps: *caps }).map_err(be)?;
        em.ins.extend(db1.instructions);
        let db2 = self.data_em.emit(&C6DataCanonicalInputs { gate: en.clone(), data: inputs.post_data.clone(), caps: *caps }).map_err(be)?;
        em.ins.extend(db2.instructions);
        let mb = self.meta_em.emit_bound(&C6MetaBoundCanonicalInputs {
            gate: en.clone(), pre_meta: inputs.pre_meta.clone(), post_meta: inputs.post_meta.clone(),
            limits: inputs.source_params.limits(),
            meta_bounds_frames_offset: inputs.meta_bounds_frames_offset,
            binding: inputs.source_params.meta_binding(),
        }).map_err(be)?;
        em.ins.extend(mb.instructions);
        let pc = self.progctx_em.emit(&C6ProgramContextCanonicalInputs {
            gate: en.clone(), pre_prog: inputs.pre_prog.clone(), pre_ctx: inputs.pre_ctx.clone(), caps: *caps,
        }).map_err(be)?;
        em.ins.extend(pc.instructions);
        let cx = self.progctx_em.emit_ctx_binding(&C6ProgramContextCtxBindingInputs {
            gate: en.clone(), pre_ctx: inputs.pre_ctx.clone(),
            expected_ctx: inputs.source_params.expected_ctx(),
        }).map_err(be)?;
        em.ins.extend(cx.instructions);
        let hb1 = self.helper_em.emit(&C6HelperCanonicalInputs { gate: en.clone(), helper: inputs.pre_helper.clone(), caps: *caps }).map_err(be)?;
        em.ins.extend(hb1.instructions);
        let hb2 = self.helper_em.emit(&C6HelperCanonicalInputs { gate: en.clone(), helper: inputs.post_helper.clone(), caps: *caps }).map_err(be)?;
        em.ins.extend(hb2.instructions);

        // Disabled: full equality in every section.
        for (a, b) in inputs.pre_image.iter().zip(inputs.post_image.iter())
            .chain(inputs.pre_data.iter().zip(inputs.post_data.iter()))
            .chain(inputs.pre_helper.iter().zip(inputs.post_helper.iter())) {
            em.geq(disabled.clone(), b.clone(), a.clone());
        }
        self.next_temp = em.next;
        Ok(C6ScalarExitStepBlock { is_helper_work_max: is_p3, is_cap_refused: is_p6, instructions: em.ins })
    }
}
