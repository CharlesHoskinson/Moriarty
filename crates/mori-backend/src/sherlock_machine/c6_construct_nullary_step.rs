//! Full-carrier XAdtEvalConstructNullary transition fragment. Source only.
//! Actual PROG expression/type/constructor/constant reads, exact Return Const,
//! checked work/reference charge and complete framing. No host success flags.
//! Full admission/StepInv, authenticated provenance/program/state commitments,
//! multi-family range dispatch, recursion and native verification remain caller
//! obligations. No financial support or circuit/proof acceptance is claimed.
use midnight_transient_crypto::curve::{Fr, FR_BITS};
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::{Identifier, Instruction};
use mori_core::sherlock::helper_machine::c6_phys::{
    C6PhysCaps, C6_PHYS_MAGIC, C6_PHYS_VERSION, L_ACTIVE, L_CONSUMED_WORK,
    L_CONTROL, L_CONTROL_ORIGIN, L_ENV_ROOT, L_FRAME_COUNT, L_OUTCOME,
    L_REFERENCE_CURSOR, L_REMAINING_WORK, L_STACK_LIMIT,
};
use mori_core::sherlock::helper_machine::c6_phys_helper::{L_HELPER_RECORD, L_RESULT_RECORD};
use mori_core::sherlock::helper_machine::c6_phys_host::{CTX_LANES, C6_HOST_SCHEMA};
use thiserror::Error;
use super::c6_data_canonical::{C6DataCanonicalEmitter, C6DataCanonicalInputs};
use super::c6_helper_canonical::{C6HelperCanonicalEmitter, C6HelperCanonicalInputs};
use super::c6_meta_canonical::{C6MetaBoundCanonicalInputs, C6MetaCanonicalEmitter};
use super::c6_program_context_canonical::{
    C6ProgramContextCanonicalEmitter, C6ProgramContextCanonicalInputs,
    C6ProgramContextCtxBindingInputs, C6ProgramContextExpressionCaptureInputs,
    C6ProgramContextConstCaptureInputs, C6ProgramContextCheckedSortCaptureInputs,
};
use super::c6_push_step::{
    C6PaddedCarrierGeometry, C6PushFamilyParams, C6PushStepError, C6_PADDED_CARRIER_VERSION,
};
const LANE_BITS: u32 = 128;

#[derive(Debug, Error)]
pub enum C6ConstructNullaryStepError {
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

fn be<E: std::fmt::Display>(e: E) -> C6ConstructNullaryStepError {
    C6ConstructNullaryStepError::Backend(e.to_string())
}

const LOCAL_RESERVED_SEGMENT: &str = ".local.temp.";
const COMPOSED_RESERVED_SEGMENTS: [&str; 8] = [
    LOCAL_RESERVED_SEGMENT, ".c6data.temp.", ".c6helper.temp.", ".c6meta.temp.", ".c6progctx.temp.",
    ".nullaryexpr.c6progctx.temp.", ".nullarynom.c6progctx.temp.", ".nullaryconst.c6progctx.temp.",
];

fn validate_parent_scope(scope: &str) -> Result<(), C6ConstructNullaryStepError> {
    if scope.trim().is_empty() { return Err(C6ConstructNullaryStepError::EmptyScope); }
    if scope.contains('%') || COMPOSED_RESERVED_SEGMENTS.iter().any(|s| scope.contains(s)) {
        return Err(C6ConstructNullaryStepError::InvalidScope);
    }
    Ok(())
}

fn reject_reserved_operand(scope: &str, op: &Operand) -> Result<(), C6ConstructNullaryStepError> {
    if let Operand::Variable(id) = op {
        for seg in COMPOSED_RESERVED_SEGMENTS {
            if id.0.starts_with(&format!("%{}{}", scope, seg)) {
                return Err(C6ConstructNullaryStepError::ReservedOperand { identifier: id.0.clone() });
            }
        }
    }
    Ok(())
}

fn reject_all_reserved(
    scope: &str, inputs: &C6ConstructNullaryStepInputs,
) -> Result<(), C6ConstructNullaryStepError> {
    for op in [&inputs.enable, &inputs.host_schema, &inputs.carrier_version] {
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

/// Retained-serial composition API for several ConstructNullary blocks under
/// one scope in one circuit. One emitter owns the parent
/// `%{scope}.local.temp.*` serial and one retained serial for each composed
/// canonical reader namespace (DATA, META, PROGCTX, HELPER and the
/// distinct expression/nominal/constant subscopes), so repeated `emit` calls
/// never rebind a temporary minted by an earlier call. The free function
/// `emit_c6_construct_nullary_step_block` is single-shot: it constructs one
/// emitter and calls it exactly once, so a given scope may be used by that
/// function at most once per circuit unless the caller can prove no other
/// block uses the same reserved prefixes. Standalone (non-composed) use
/// follows the same contract: one block per scope per circuit, or this
/// retained emitter for composition.
pub struct C6ConstructNullaryStepEmitter {
    scope: String,
    next_temp: usize,
    data_em: C6DataCanonicalEmitter,
    meta_em: C6MetaCanonicalEmitter,
    progctx_em: C6ProgramContextCanonicalEmitter,
    expression_em: C6ProgramContextCanonicalEmitter,
    nominal_em: C6ProgramContextCanonicalEmitter,
    constant_em: C6ProgramContextCanonicalEmitter,
    helper_em: C6HelperCanonicalEmitter,
}

impl C6ConstructNullaryStepEmitter {
    pub fn new(scope: &str) -> Result<Self, C6ConstructNullaryStepError> {
        validate_parent_scope(scope)?;
        Ok(Self {
            scope: scope.to_string(),
            next_temp: 0,
            data_em: C6DataCanonicalEmitter::new(scope).map_err(be)?,
            meta_em: C6MetaCanonicalEmitter::new(scope).map_err(be)?,
            progctx_em: C6ProgramContextCanonicalEmitter::new(scope).map_err(be)?,
            expression_em: C6ProgramContextCanonicalEmitter::new(&format!("{scope}.nullaryexpr")).map_err(be)?,
            nominal_em: C6ProgramContextCanonicalEmitter::new(&format!("{scope}.nullarynom")).map_err(be)?,
            constant_em: C6ProgramContextCanonicalEmitter::new(&format!("{scope}.nullaryconst")).map_err(be)?,
            helper_em: C6HelperCanonicalEmitter::new(scope).map_err(be)?,
        })
    }
    pub fn scope(&self) -> &str { &self.scope }
    pub fn next_temp(&self) -> usize { self.next_temp }
}

#[derive(Clone, Debug)]
pub struct C6ConstructNullaryStepInputs {
    pub enable: Operand,
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
pub struct C6ConstructNullaryStepBlock {
    pub instructions: Vec<Instruction>,
}

struct Em<'a> { scope: &'a str, next: usize, ins: Vec<Instruction> }
impl<'a> Em<'a> {
    fn t(&mut self) -> Identifier {
        let id = Identifier(format!("%{}.local.temp.{}", self.scope, self.next));
        self.next += 1; id
    }
    fn imm(v: u128) -> Operand { Operand::Immediate(Fr::from(v)) }
    fn cbool(&mut self, v: Operand) { self.ins.push(Instruction::ConstrainToBoolean { val: v }); }
    fn cbits(&mut self, v: Operand) { self.ins.push(Instruction::ConstrainBits { val: v, bits: LANE_BITS }); }
    fn ceq(&mut self, a: Operand, b: Operand) { self.ins.push(Instruction::ConstrainEq { a, b }); }
    fn teq(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.t(); self.ins.push(Instruction::TestEq { a, b, output: o.clone() });
        Operand::Variable(o)
    }
    fn lt(&mut self, a: Operand, b: Operand, bits: u32) -> Operand {
        let o = self.t(); self.ins.push(Instruction::LessThan { a, b, bits, output: o.clone() });
        Operand::Variable(o)
    }
    fn lt128(&mut self, a: Operand, b: Operand) -> Operand { self.lt(a, b, LANE_BITS) }
    fn add(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.t(); self.ins.push(Instruction::Add { a, b, output: o.clone() });
        Operand::Variable(o)
    }
    fn mul(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.t(); self.ins.push(Instruction::Mul { a, b, output: o.clone() });
        Operand::Variable(o)
    }
    fn not(&mut self, a: Operand) -> Operand {
        let o = self.t(); self.ins.push(Instruction::Not { a, output: o.clone() });
        Operand::Variable(o)
    }
    fn sel(&mut self, bit: Operand, a: Operand, b: Operand) -> Operand {
        let o = self.t(); self.ins.push(Instruction::CondSelect { bit, a, b, output: o.clone() });
        Operand::Variable(o)
    }
    fn assert_t(&mut self, c: Operand) { self.ins.push(Instruction::Assert { cond: c }); }
    fn geq(&mut self, g: Operand, a: Operand, b: Operand) {
        let s = self.sel(g, a, b.clone()); self.ceq(s, b);
    }
    fn gassert(&mut self, g: Operand, c: Operand) {
        let e = self.sel(g, c, Self::imm(1)); self.assert_t(e);
    }
    fn gzero(&mut self, g: Operand, v: Operand) { self.geq(g, v, Self::imm(0)); }
    fn gbool(&mut self, g: Operand, v: Operand) {
        let c = self.sel(g, v, Self::imm(0)); self.cbool(c);
    }
    fn glt_i(&mut self, g: Operand, v: Operand, cap: usize) {
        let l = self.lt128(v, Self::imm(cap as u128)); self.gassert(g, l);
    }
    fn glt_c(&mut self, g: Operand, v: Operand, cap: usize) {
        let l = self.lt128(v, Self::imm((cap as u128) + 1)); self.gassert(g, l);
    }
    fn glt_end(&mut self, g: Operand, end: Operand, cap: usize) {
        let l = self.lt(end, Self::imm((cap as u128) + 1), 129); self.gassert(g, l);
    }
    fn tag_bits(&mut self, g: &Operand, lane: Operand, n: u128) -> Vec<Operand> {
        let mut bits = Vec::new(); let mut sum = Self::imm(0);
        for t in 0..n {
            let b = self.teq(lane.clone(), Self::imm(t));
            sum = self.add(sum, b.clone()); bits.push(b);
        }
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
            for (t, u) in USED.iter().enumerate() {
                if lane >= *u { mz = self.add(mz, bits[t].clone()); any = true; }
            }
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
            // Whole-record padding against Core decode_control/encode_control:
            // USED[t] is the used-lane count for tag t. Tag 8 (PrimResult)
            // is NOT exempt: USED[8] = 4 covers the shared 4..16 tail of
            // both sub-tags, the Value ref occupies lanes 2..4 (so lanes
            // 1..3 are untouched by this rule), and the Error sub-tag's
            // lane 3 is separately gated zero by `gzero(eg, k[3])` above.
            let mut mz = Self::imm(0); let mut any = false;
            for (t, u) in USED.iter().enumerate() {
                if lane >= *u { mz = self.add(mz, bits[t].clone()); any = true; }
            }
            if any { let pg = self.mul(gate.clone(), mz); self.gzero(pg, k[lane].clone()); }
        }
    }
}

/// Single-shot block at parent serial zero; see
/// `C6ConstructNullaryStepEmitter` for the composition contract. Repeated
/// blocks under one scope in one circuit must go through one retained
/// `C6ConstructNullaryStepEmitter`.
pub fn emit_c6_construct_nullary_step_block(
    inputs: &C6ConstructNullaryStepInputs, scope: &str,
) -> Result<C6ConstructNullaryStepBlock, C6ConstructNullaryStepError> {
    C6ConstructNullaryStepEmitter::new(scope)?.emit(inputs)
}

impl C6ConstructNullaryStepEmitter {
pub fn emit(
    &mut self, inputs: &C6ConstructNullaryStepInputs,
) -> Result<C6ConstructNullaryStepBlock, C6ConstructNullaryStepError> {
    let scope: &str = &self.scope;
    validate_parent_scope(scope)?;
    reject_all_reserved(scope, inputs)?;
    if (LANE_BITS as usize) >= FR_BITS { return Err(C6ConstructNullaryStepError::InvalidBitWidth { bits: LANE_BITS }); }
    inputs.source_params.validate_geometry(inputs.geometry)?;
    let g = &inputs.geometry;
    let caps = &g.caps; let p = &caps.phys;
    for cap in [p.max_frames, p.max_expressions, p.max_names, p.max_arms, p.max_children,
        p.max_value_slots, p.max_constants, p.max_data_nodes, p.max_env_cells, p.max_functions,
        p.max_arity, p.max_types, p.max_constructors] {
        if cap == usize::MAX { return Err(C6ConstructNullaryStepError::GeometryOverflow { section: "caps" }); }
    }
    for (s, a, e) in [("image", inputs.pre_image.len(), g.image_total_lanes),
        ("image", inputs.post_image.len(), g.image_total_lanes),
        ("data", inputs.pre_data.len(), g.data_total_lanes),
        ("data", inputs.post_data.len(), g.data_total_lanes),
        ("helper", inputs.pre_helper.len(), g.helper_total_lanes),
        ("helper", inputs.post_helper.len(), g.helper_total_lanes)] {
        if a != e { return Err(C6ConstructNullaryStepError::LengthMismatch { section: s, expected: e, actual: a }); }
    }
    if inputs.pre_meta.len() < 3 || inputs.pre_meta.len() != inputs.post_meta.len()
        || inputs.pre_prog.len() != inputs.post_prog.len()
        || inputs.meta_bounds_frames_offset >= inputs.pre_meta.len() {
        return Err(C6ConstructNullaryStepError::Backend("host prefix length/offset".into()));
    }

    let mut em = Em { scope, next: self.next_temp, ins: Vec::new() };
    let en = inputs.enable.clone();
    em.cbool(en.clone());
    let disabled = em.not(en.clone());
    em.ceq(inputs.host_schema.clone(), Em::imm(C6_HOST_SCHEMA));
    em.ceq(inputs.carrier_version.clone(), Em::imm(C6_PADDED_CARRIER_VERSION));

    // Host prefix: unconditional ranges + pre == post; gated headers.
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
    // State lanes: unconditional ranges and complete DATA/HELPER equality.
    for l in inputs.pre_image.iter().chain(inputs.post_image.iter())
        .chain(inputs.pre_data.iter()).chain(inputs.post_data.iter()) {
        em.cbits(l.clone());
    }
    for (a, b) in inputs.pre_data.iter().zip(inputs.post_data.iter()) {
        em.ceq(b.clone(), a.clone());
    }

    let pre_h = &inputs.pre_helper; let post_h = &inputs.post_helper;
    let pre_i = &inputs.pre_image; let post_i = &inputs.post_image;

    // Pre image root/control/frames structural canonicality (all 11 controls, 10 frames).
    em.geq(en.clone(), pre_i[0].clone(), Em::imm(C6_PHYS_MAGIC));
    em.geq(en.clone(), pre_i[1].clone(), Em::imm(C6_PHYS_VERSION));
    { let ctl: Vec<Operand> = pre_i[L_CONTROL..L_CONTROL + 17].to_vec(); em.vcontrol(en.clone(), &ctl, p); }
    em.glt_i(en.clone(), pre_i[L_CONTROL_ORIGIN].clone(), p.max_expressions);
    em.venv(en.clone(), pre_i[L_ENV_ROOT].clone(), pre_i[L_ENV_ROOT + 1].clone(), p.max_env_cells);
    em.glt_c(en.clone(), pre_i[L_STACK_LIMIT].clone(), p.max_frames);
    for off in 0..3 { em.gzero(en.clone(), pre_i[L_OUTCOME + off].clone()); }
    let has_fuel = em.lt128(Em::imm(0), pre_i[L_REMAINING_WORK].clone());
    em.gassert(en.clone(), has_fuel);
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

    // Actual dispatch domain: ordinary Running Eval, with no staged
    // helper/result or tasks. P3 is irrelevant to this LegacyComplete row.
    em.gzero(en.clone(), pre_i[L_CONTROL].clone());
    em.gzero(en.clone(), pre_h[L_HELPER_RECORD].clone());
    em.gzero(en.clone(), pre_h[L_RESULT_RECORD].clone());
    em.gzero(en.clone(), pre_h[3].clone());
    for off in 0..4 {
        em.geq(en.clone(), pre_i[L_ACTIVE + off].clone(), inputs.pre_data[3 + off].clone());
    }

    // Each dependent reader retains a distinct private namespace. Its output
    // is legal input to the next reader without relaxing any reserved guard.
    // Read the actual Eval-selected expression through the complete parser.
    // No caller-supplied Construct/nullary/checked-sort success bit exists.
    let expression = self.expression_em.emit_prog_expression_capture(
        &C6ProgramContextExpressionCaptureInputs {
            gate: en.clone(), pre_prog: inputs.pre_prog.clone(), caps: *caps,
            selected_index: pre_i[L_CONTROL + 1].clone(),
        }).map_err(be)?;
    em.ins.extend(expression.instructions);
    em.geq(en.clone(), expression.selected_found, Em::imm(1));
    let x = expression.row;
    em.geq(en.clone(), x[0].clone(), Em::imm(8)); // Construct
    em.gzero(en.clone(), x[4].clone()); // fields.len
    em.geq(en.clone(), x[5].clone(), Em::imm(1)); // Some constant

    let nominal = self.nominal_em.emit_prog_checked_sort_capture(
        &C6ProgramContextCheckedSortCaptureInputs {
            gate: en.clone(), pre_prog: inputs.pre_prog.clone(), caps: *caps,
            selected_type_id: x[1].clone(), selected_ctor_id: x[2].clone(),
        }).map_err(be)?;
    em.ins.extend(nominal.instructions);
    let dedicated = em.lt128(x[1].clone(), Em::imm(4));
    em.gzero(en.clone(), dedicated);
    em.geq(en.clone(), nominal.type_found, Em::imm(1));
    em.geq(en.clone(), nominal.ctor_found, Em::imm(1));
    em.geq(en.clone(), nominal.owning_type, x[1].clone());
    // Use 129 bits for sums of UInt128 lanes, including disabled input.
    // The interval is equivalent to Core's checked subtraction test and
    // does not invent a usize overflow refusal for type.ctor_start+len.
    let ctor_end = em.add(nominal.ctor_start.clone(), nominal.ctor_len);
    let below_start = em.lt(x[2].clone(), nominal.ctor_start, 129);
    em.gzero(en.clone(), below_start);
    let below_end = em.lt(x[2].clone(), ctor_end, 129);
    em.gassert(en.clone(), below_end);
    em.gzero(en.clone(), nominal.field_len.clone());
    let field_end = em.add(nominal.field_start, nominal.field_len);
    let field_outside = em.lt(nominal.ctor_field_count, field_end, 129);
    em.gzero(en.clone(), field_outside);

    let constant = self.constant_em.emit_prog_const_capture(
        &C6ProgramContextConstCaptureInputs {
            gate: en.clone(), pre_prog: inputs.pre_prog.clone(), caps: *caps,
            selected_index: x[6].clone(),
        }).map_err(be)?;
    em.ins.extend(constant.instructions);
    em.geq(en.clone(), constant.selected_found, Em::imm(1));
    let c = constant.row;
    em.geq(en.clone(), c[0].clone(), Em::imm(6)); // Constructor
    em.geq(en.clone(), c[1].clone(), Em::imm(1)); // marker_free
    em.geq(en.clone(), c[2].clone(), x[1].clone());
    em.geq(en.clone(), c[3].clone(), x[2].clone());
    em.gzero(en.clone(), c[5].clone()); // constant fields.len
    // Neither zero-length expression nor constant slice starts are forced
    // to zero: the source reader does not impose that extra condition.

    // Exact Return{Const(cid)} control, including all unused union lanes.
    for off in 0..17 {
        let expected = match off {
            0 => Em::imm(1), 2 => x[6].clone(), _ => Em::imm(0),
        };
        em.geq(en.clone(), post_i[L_CONTROL + off].clone(), expected);
    }
    let uncharged_predecessor = em.add(post_i[L_REMAINING_WORK].clone(), Em::imm(1));
    em.geq(en.clone(), uncharged_predecessor, pre_i[L_REMAINING_WORK].clone());
    for lane in [L_CONSUMED_WORK, L_REFERENCE_CURSOR] {
        let no_overflow = em.lt128(pre_i[lane].clone(), Em::imm(u128::MAX));
        em.gassert(en.clone(), no_overflow);
        let incremented = em.add(pre_i[lane].clone(), Em::imm(1));
        em.geq(en.clone(), post_i[lane].clone(), incremented);
    }
    for lane in 0..g.image_total_lanes {
        if !(L_CONTROL..L_CONTROL + 17).contains(&lane)
            && ![L_REMAINING_WORK, L_CONSUMED_WORK, L_REFERENCE_CURSOR].contains(&lane) {
            em.geq(en.clone(), post_i[lane].clone(), pre_i[lane].clone());
        }
    }
    for (a, b) in pre_h.iter().zip(post_h.iter()) { em.ceq(b.clone(), a.clone()); }

    // Composed canonical blocks (pre-only parsers: DATA and prefix post are equal, image/helper framing is explicit).
    // Each composed emitter retains its serial inside `self`, so a second
    // block under this scope continues every composed namespace.
    let data_block = self.data_em
        .emit(&C6DataCanonicalInputs { gate: en.clone(), data: inputs.pre_data.clone(), caps: *caps })
        .map_err(be)?;
    em.ins.extend(data_block.instructions);
    let meta_block = self.meta_em.emit_bound(&C6MetaBoundCanonicalInputs {
        gate: en.clone(), pre_meta: inputs.pre_meta.clone(), post_meta: inputs.post_meta.clone(),
        limits: inputs.source_params.limits(),
        meta_bounds_frames_offset: inputs.meta_bounds_frames_offset,
        binding: inputs.source_params.meta_binding(),
    }).map_err(be)?;
    em.ins.extend(meta_block.instructions);
    let pc_block = self.progctx_em.emit(&C6ProgramContextCanonicalInputs {
        gate: en.clone(), pre_prog: inputs.pre_prog.clone(), pre_ctx: inputs.pre_ctx.clone(), caps: *caps,
    }).map_err(be)?;
    em.ins.extend(pc_block.instructions);
    let ctx_block = self.progctx_em.emit_ctx_binding(&C6ProgramContextCtxBindingInputs {
        gate: en.clone(), pre_ctx: inputs.pre_ctx.clone(),
        expected_ctx: inputs.source_params.expected_ctx(),
    }).map_err(be)?;
    em.ins.extend(ctx_block.instructions);
    // HELPER canonicality on BOTH pre and post under one retained serial.
    let hb1 = self.helper_em.emit(&C6HelperCanonicalInputs {
        gate: en.clone(), helper: inputs.pre_helper.clone(), caps: *caps,
    }).map_err(be)?;
    em.ins.extend(hb1.instructions);
    let hb2 = self.helper_em.emit(&C6HelperCanonicalInputs {
        gate: en.clone(), helper: inputs.post_helper.clone(), caps: *caps,
    }).map_err(be)?;
    em.ins.extend(hb2.instructions);

    // Disabled: full equality in every section.
    for (a, b) in inputs.pre_image.iter().zip(inputs.post_image.iter())
        .chain(inputs.pre_helper.iter().zip(inputs.post_helper.iter())) {
        em.geq(disabled.clone(), b.clone(), a.clone());
    }
    self.next_temp = em.next;
    Ok(C6ConstructNullaryStepBlock { instructions: em.ins })
}
}

