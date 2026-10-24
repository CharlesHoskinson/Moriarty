//! C6 BHelperEqualEmpty over complete padded machine sections.
//!
//! Core-selected premises: Running, positive fuel, helper Equal/Pending,
//! zero live tasks, and valid helper/result exclusivity. Applied: status
//! becomes Positive, remaining_work -= 1, consumed_work += 1 and
//! helper_work += 1. HelperVisits delta is ZERO, despite HelperVisit event.
//! All other image, DATA, helper and result lanes remain exactly equal.
//! At the original separately retained admission helper-work maximum,
//! P3 changes only outcome to Exhausted(HelperWork), uncharged; status
//! remains Pending. Earlier P0/P1/P2 branches are outside this relation.
//! At enable=0 every state section is identity; enable/disposition are
//! constrained Booleans and all statement lanes remain UInt128.
//!
//! This is a composable transition fragment, NOT proof of equality's
//! previous walk: EqualEntry/Pair/Field/release/Exit, complete StepInv,
//! helper/control/site/entry coherence, actual program membership and
//! authoritative compiler/owner/child provenance are caller obligations.
//! Canonical framing is composed from the existing DATA/META/PROGCTX/
//! HELPER emitters; original refs are preserved, parent_resume=None is
//! enforced by the actual PS rule, and active lengths bind DATA counts. No
//! Boolean saying values are equal is accepted as an oracle. No financial
//! execution, native crypto, source admission, native proof, key issuer
//! or complete range dispatch is added. Financial codec refusals remain.
//!
//! Statement order is unchanged C6 convention: enable, refused-helper-work,
//! host_schema, carrier_version, pre(META,PROG,CTX,image,DATA,helper),
//! post(META,PROG,CTX,image,DATA,helper). LocalPublic and LedgerCall bind
//! this exact statement; LedgerCall returns the actual constrained full
//! post flat state. ProofPreimage::check is preprocessing ONLY, not a proof.
//! One retained emitter owns each parent/composed temporary namespace;
//! reuse it for multiple blocks or use distinct scopes. The free emitter
//! function is single-shot per scope, as in the existing C6 family.

use std::sync::Arc;

use midnight_transient_crypto::curve::{Fr, FR_BITS};
use midnight_transient_crypto::hash::transient_commit;
use midnight_transient_crypto::proofs::{KeyLocation, ProofPreimage, Zkir};
use midnight_zkir_v3::ir::{IrMinorVersion, Operand, TypedIdentifier};
use midnight_zkir_v3::ir_types::IrType;
use midnight_zkir_v3::{Identifier, Instruction, IrSource};
use mori_core::sherlock::helper_machine::c6_phys::{
    C6PhysCaps, C6_PHYS_MAGIC, C6_PHYS_VERSION, L_ACTIVE, L_CONSUMED_WORK, L_CONTROL, L_CONTROL_ORIGIN,
    L_ENV_ROOT, L_FRAME_COUNT, L_HELPER_WORK, L_OUTCOME, L_REMAINING_WORK, L_STACK_LIMIT,
};
use mori_core::sherlock::helper_machine::c6_phys_data::C6StateCaps;
use mori_core::sherlock::helper_machine::c6_phys_helper::{
    L_HELPER_RECORD, L_RESULT_RECORD,
};
use mori_core::sherlock::helper_machine::c6_phys_host::{C6HostConfig, C6HostLimits, CTX_LANES};
use mori_core::sherlock::helper_machine::c6_phys_state::{C6OwnedState, C6StateLimits};
use mori_core::sherlock::helper_machine::complete_observation::HostSnapshot;
use serde_json::json;
use thiserror::Error;

use super::c6_data_canonical::{C6DataCanonicalEmitter, C6DataCanonicalInputs};
use super::c6_helper_canonical::{C6HelperCanonicalEmitter, C6HelperCanonicalInputs};
use super::c6_meta_canonical::{C6MetaBoundCanonicalInputs, C6MetaCanonicalEmitter};
use super::c6_program_context_canonical::{
    C6ProgramContextCanonicalEmitter, C6ProgramContextCanonicalInputs,
    C6ProgramContextCtxBindingInputs,
};
use super::c6_push_step::{
    C6PaddedCarrierGeometry, C6PushFamilyParams, C6PushHostPrefixLanes,
    C6PushStatementBindingMode, C6PushStepError, C6_PADDED_CARRIER_VERSION,
};
use mori_core::sherlock::helper_machine::c6_phys_host::C6_HOST_SCHEMA;

const LANE_BITS: u32 = 128;
const CHECK_KEY_LOCATION: &str = "sherlock_c6_equal_empty_step_check";

#[derive(Debug, Error)]
pub enum C6EqualEmptyStepError {
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
    #[error("host prefix section {section} mismatch between pre and post state")]
    HostPrefixMismatch { section: &'static str },
    #[error("core C6 physical/push error: {0}")]
    Push(#[from] C6PushStepError),
    #[error("composed canonical emitter or SDK backend error: {0}")]
    Backend(String),
}

fn be<E: std::fmt::Display>(e: E) -> C6EqualEmptyStepError {
    C6EqualEmptyStepError::Backend(e.to_string())
}

const LOCAL_RESERVED_SEGMENT: &str = ".local.temp.";
const COMPOSED_RESERVED_SEGMENTS: [&str; 5] = [
    LOCAL_RESERVED_SEGMENT, ".c6data.temp.", ".c6helper.temp.", ".c6meta.temp.", ".c6progctx.temp.",
];

fn validate_parent_scope(scope: &str) -> Result<(), C6EqualEmptyStepError> {
    if scope.trim().is_empty() { return Err(C6EqualEmptyStepError::EmptyScope); }
    if scope.contains('%') || COMPOSED_RESERVED_SEGMENTS.iter().any(|s| scope.contains(s)) {
        return Err(C6EqualEmptyStepError::InvalidScope);
    }
    Ok(())
}

fn reject_reserved_operand(scope: &str, op: &Operand) -> Result<(), C6EqualEmptyStepError> {
    if let Operand::Variable(id) = op {
        for seg in COMPOSED_RESERVED_SEGMENTS {
            if id.0.starts_with(&format!("%{}{}", scope, seg)) {
                return Err(C6EqualEmptyStepError::ReservedOperand { identifier: id.0.clone() });
            }
        }
    }
    Ok(())
}

fn reject_all_reserved(
    scope: &str, inputs: &C6EqualEmptyStepInputs,
) -> Result<(), C6EqualEmptyStepError> {
    for op in [&inputs.enable, &inputs.disposition_is_refused_helper_work,
        &inputs.host_schema, &inputs.carrier_version] {
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

/// Retained-serial composition API for several EqualEmpty blocks under
/// one scope in one circuit. One emitter owns the parent
/// `%{scope}.local.temp.*` serial and one retained serial for each composed
/// canonical block (DATA, META, PROGCTX, HELPER), so repeated `emit` calls
/// never rebind a temporary minted by an earlier call. The free function
/// `emit_c6_equal_empty_step_block` is single-shot: it constructs one
/// emitter and calls it exactly once, so a given scope may be used by that
/// function at most once per circuit unless the caller can prove no other
/// block uses the same reserved prefixes. Standalone (non-composed) use
/// follows the same contract: one block per scope per circuit, or this
/// retained emitter for composition.
pub struct C6EqualEmptyStepEmitter {
    scope: String,
    next_temp: usize,
    data_em: C6DataCanonicalEmitter,
    meta_em: C6MetaCanonicalEmitter,
    progctx_em: C6ProgramContextCanonicalEmitter,
    helper_em: C6HelperCanonicalEmitter,
}

impl C6EqualEmptyStepEmitter {
    pub fn new(scope: &str) -> Result<Self, C6EqualEmptyStepError> {
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
pub struct C6EqualEmptyStepInputs {
    pub enable: Operand,
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
pub struct C6EqualEmptyStepBlock {
    pub is_helper_work_max: Operand,
    pub instructions: Vec<Instruction>,
}

struct Em<'a> { scope: &'a str, next: usize, ins: Vec<Instruction> }
impl<'a> Em<'a> {
    fn new(scope: &'a str) -> Self { Self { scope, next: 0, ins: Vec::new() } }
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
/// `C6EqualEmptyStepEmitter` for the composition contract. Repeated
/// blocks under one scope in one circuit must go through one retained
/// `C6EqualEmptyStepEmitter`.
pub fn emit_c6_equal_empty_step_block(
    inputs: &C6EqualEmptyStepInputs, scope: &str,
) -> Result<C6EqualEmptyStepBlock, C6EqualEmptyStepError> {
    C6EqualEmptyStepEmitter::new(scope)?.emit(inputs)
}

impl C6EqualEmptyStepEmitter {
pub fn emit(
    &mut self, inputs: &C6EqualEmptyStepInputs,
) -> Result<C6EqualEmptyStepBlock, C6EqualEmptyStepError> {
    let scope: &str = &self.scope;
    validate_parent_scope(scope)?;
    reject_all_reserved(scope, inputs)?;
    if (LANE_BITS as usize) >= FR_BITS { return Err(C6EqualEmptyStepError::InvalidBitWidth { bits: LANE_BITS }); }
    inputs.source_params.validate_geometry(inputs.geometry)?;
    let g = &inputs.geometry;
    let caps = &g.caps; let p = &caps.phys;
    for cap in [p.max_frames, p.max_expressions, p.max_names, p.max_arms, p.max_children,
        p.max_value_slots, p.max_constants, p.max_data_nodes, p.max_env_cells, p.max_functions,
        p.max_arity, p.max_types, p.max_constructors] {
        if cap == usize::MAX { return Err(C6EqualEmptyStepError::GeometryOverflow { section: "caps" }); }
    }
    for (s, a, e) in [("image", inputs.pre_image.len(), g.image_total_lanes),
        ("image", inputs.post_image.len(), g.image_total_lanes),
        ("data", inputs.pre_data.len(), g.data_total_lanes),
        ("data", inputs.post_data.len(), g.data_total_lanes),
        ("helper", inputs.pre_helper.len(), g.helper_total_lanes),
        ("helper", inputs.post_helper.len(), g.helper_total_lanes)] {
        if a != e { return Err(C6EqualEmptyStepError::LengthMismatch { section: s, expected: e, actual: a }); }
    }
    if inputs.pre_meta.len() < 3 || inputs.pre_meta.len() != inputs.post_meta.len()
        || inputs.pre_prog.len() != inputs.post_prog.len()
        || inputs.meta_bounds_frames_offset >= inputs.pre_meta.len() {
        return Err(C6EqualEmptyStepError::Backend("host prefix length/offset".into()));
    }

    let max_hw = inputs.source_params.admission_bounds().max_helper_work;
    let mut em = Em { scope, next: self.next_temp, ins: Vec::new() };
    let en = inputs.enable.clone();
    let dp = inputs.disposition_is_refused_helper_work.clone();
    em.cbool(en.clone()); em.cbool(dp.clone());
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
    // State lanes: unconditional ranges; DATA/HELPER equality is branch-specific below.
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

    // Core eval_select_when: Equal + Pending + zero live tasks.
    // PS owner_validate_plan rejects rewritten helper parent_resume=Some,
    // before P3. Preserve the B-core None domain, not just codec acceptance.
    // This rule does not dereference original left/right: the prior walk
    // and its provenance must be constrained separately by the caller.
    em.geq(en.clone(), pre_h[L_HELPER_RECORD].clone(), Em::imm(8));
    em.gzero(en.clone(), pre_h[L_HELPER_RECORD + 9].clone()); // Pending
    em.gzero(en.clone(), pre_h[3].clone()); // no live tasks
    em.gzero(en.clone(), pre_h[L_HELPER_RECORD + 3].clone()); // PS parentResume None
    // P7 recomputes stored active lengths from unchanged DATA vectors.
    // Bind the pre-image active counters to the actual physical row counts.
    for off in 0..4 {
        em.geq(en.clone(), pre_i[L_ACTIVE + off].clone(), inputs.pre_data[3 + off].clone());
    }
    // Valid helper domain: no simultaneously staged result.
    em.gzero(en.clone(), pre_h[L_RESULT_RECORD].clone());

    // Disposition: P3 iff helper_work == retained admission max_helper_work.
    let is_p3 = em.teq(pre_i[L_HELPER_WORK].clone(), Em::imm(max_hw));
    em.geq(en.clone(), dp.clone(), is_p3.clone());
    let not_p3 = em.not(is_p3.clone());
    let applied_gate = em.mul(en.clone(), not_p3.clone());

    // Post outcome: Applied Running [0,0,0]; P3 Exhausted{HelperWork} [3,5,0].
    let exp_o0 = em.sel(is_p3.clone(), Em::imm(3), Em::imm(0));
    em.geq(en.clone(), post_i[L_OUTCOME].clone(), exp_o0);
    let exp_o1 = em.sel(is_p3.clone(), Em::imm(5), Em::imm(0));
    em.geq(en.clone(), post_i[L_OUTCOME + 1].clone(), exp_o1);
    em.gzero(en.clone(), post_i[L_OUTCOME + 2].clone());
    // Counters.
    { let s = em.add(post_i[L_REMAINING_WORK].clone(), Em::imm(1));
      em.geq(applied_gate.clone(), s, pre_i[L_REMAINING_WORK].clone());
      let pg = em.mul(en.clone(), is_p3.clone()); em.geq(pg, post_i[L_REMAINING_WORK].clone(), pre_i[L_REMAINING_WORK].clone()); }
    for lane in [L_CONSUMED_WORK, L_HELPER_WORK] {
        let noov = em.lt128(pre_i[lane].clone(), Em::imm(u128::MAX));
        em.gassert(applied_gate.clone(), noov);
        let exp = em.add(pre_i[lane].clone(), Em::imm(1));
        let exp2 = em.sel(is_p3.clone(), pre_i[lane].clone(), exp);
        em.geq(en.clone(), post_i[lane].clone(), exp2);
    }
    // All other image lanes preserved (incl. frames, control, origin, env root,
    // initial fuel, helper visits, reference cursor, alloc/active/seg bases).
    let mutable = [L_OUTCOME, L_OUTCOME + 1, L_OUTCOME + 2, L_REMAINING_WORK, L_CONSUMED_WORK, L_HELPER_WORK];
    for lane in 0..g.image_total_lanes {
        if !mutable.contains(&lane) {
            em.geq(en.clone(), post_i[lane].clone(), pre_i[lane].clone());
        }
    }
    // EqualEmpty changes only status Pending -> Positive when applied.
    // Original refs, common/site/origin/parent_resume, result=None,
    // empty task table and every other padded helper lane are preserved.
    for lane in 0..g.helper_total_lanes {
        let expected = if lane == L_HELPER_RECORD + 9 {
            em.sel(is_p3.clone(), pre_h[lane].clone(), Em::imm(1))
        } else { pre_h[lane].clone() };
        em.geq(en.clone(), post_h[lane].clone(), expected);
    }

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
    Ok(C6EqualEmptyStepBlock { is_helper_work_max: is_p3, instructions: em.ins })
}
}

#[derive(Clone, Debug, PartialEq)]
pub struct C6EqualEmptyStepWitness {
    enable: bool,
    disposition_is_refused_helper_work: bool,
    source_params: C6PushFamilyParams,
    host_prefix: C6PushHostPrefixLanes,
    pre_image: Vec<u128>, pre_data: Vec<u128>, pre_helper: Vec<u128>,
    post_image: Vec<u128>, post_data: Vec<u128>, post_helper: Vec<u128>,
}
impl C6EqualEmptyStepWitness {
    pub fn source_params(&self) -> &C6PushFamilyParams { &self.source_params }
    pub fn to_statement_frs(&self) -> Vec<Fr> {
        let mut out = Vec::new();
        out.push(Fr::from(self.enable));
        out.push(Fr::from(self.disposition_is_refused_helper_work));
        out.push(Fr::from(C6_HOST_SCHEMA));
        out.push(Fr::from(C6_PADDED_CARRIER_VERSION));
        for l in self.host_prefix.meta.iter().chain(&self.host_prefix.prog)
            .chain(self.host_prefix.ctx.iter())
            .chain(&self.pre_image).chain(&self.pre_data).chain(&self.pre_helper) {
            out.push(Fr::from(*l));
        }
        for l in self.host_prefix.meta.iter().chain(&self.host_prefix.prog)
            .chain(self.host_prefix.ctx.iter())
            .chain(&self.post_image).chain(&self.post_data).chain(&self.post_helper) {
            out.push(Fr::from(*l));
        }
        out
    }
    fn post_flat(&self) -> Vec<u128> {
        let mut v = self.post_image.clone();
        v.extend(&self.post_data); v.extend(&self.post_helper); v
    }
}

fn split_state(
    state: &mori_core::sherlock::helper_machine::complete_observation::ImageSnapshot,
    caps: C6StateCaps, limits: C6HostLimits, geom: C6PaddedCarrierGeometry,
) -> Result<(Vec<u128>, Vec<u128>, Vec<u128>), C6EqualEmptyStepError> {
    let padded = super::c6_push_step::encode_owned_state_to_padded(
        &C6OwnedState::from_image(state), caps,
        C6StateLimits { max_total_lanes: limits.max_state_lanes })?;
    let flat = padded.to_flat_lanes();
    let i = geom.image_total_lanes; let d = i + geom.data_total_lanes;
    Ok((flat[..i].to_vec(), flat[i..d].to_vec(), flat[d..].to_vec()))
}

/// Independent-post route: no recorder runs; wrong post reaches the circuit.
pub fn witness_from_host_pair(
    enable: bool, pre: &HostSnapshot, post: &HostSnapshot,
    cfg: &C6HostConfig, limits: &C6HostLimits,
) -> Result<C6EqualEmptyStepWitness, C6EqualEmptyStepError> {
    let params = C6PushFamilyParams::new(cfg.clone(), pre.admission.bounds, *limits)?;
    let geom = params.geometry(); let caps = params.caps();
    let pp = C6PushHostPrefixLanes::from_host_snapshot(pre, cfg, limits)?;
    let qp = C6PushHostPrefixLanes::from_host_snapshot(post, cfg, limits)?;
    if pp.meta != qp.meta { return Err(C6EqualEmptyStepError::HostPrefixMismatch { section: "meta" }); }
    if pp.prog != qp.prog { return Err(C6EqualEmptyStepError::HostPrefixMismatch { section: "prog" }); }
    if pp.ctx != qp.ctx { return Err(C6EqualEmptyStepError::HostPrefixMismatch { section: "ctx" }); }
    let (pre_image, pre_data, pre_helper) = split_state(&pre.image, caps, *limits, geom)?;
    let (post_image, post_data, post_helper) = split_state(&post.image, caps, *limits, geom)?;
    Ok(C6EqualEmptyStepWitness {
        enable,
        disposition_is_refused_helper_work:
            pre.image.helper_work == params.admission_bounds().max_helper_work,
        source_params: params, host_prefix: pp,
        pre_image, pre_data, pre_helper, post_image, post_data, post_helper,
    })
}

#[derive(Clone, Debug)]
pub struct C6EqualEmptyStepCircuit {
    mode: C6PushStatementBindingMode,
    geometry: C6PaddedCarrierGeometry,
    meta_len: usize, prog_len: usize, meta_bounds_frames_offset: usize,
    source_params: C6PushFamilyParams, statement_lane_count: usize,
    ir: IrSource, serialized_zkir: String,
}
impl C6EqualEmptyStepCircuit {
    pub fn source_params(&self) -> &C6PushFamilyParams { &self.source_params }
    pub fn geometry(&self) -> C6PaddedCarrierGeometry { self.geometry }
}

fn make_typed(id: Identifier) -> Result<TypedIdentifier, C6EqualEmptyStepError> {
    serde_json::from_value(json!({ "name": id, "type": IrType::Native })).map_err(be)
}

#[allow(clippy::too_many_arguments)]
fn alloc_lane(mode: C6PushStatementBindingMode, scope: &str, label: &str, idx: usize,
    inputs: &mut Vec<TypedIdentifier>, ins: &mut Vec<Instruction>, tr: &mut Vec<Operand>)
    -> Result<Operand, C6EqualEmptyStepError> {
    let prefix = match mode { C6PushStatementBindingMode::LocalPublic => "public", C6PushStatementBindingMode::LedgerCall => "input" };
    let id = Identifier(format!("%{scope}.{prefix}.{label}.{idx}"));
    match mode {
        C6PushStatementBindingMode::LocalPublic =>
            ins.push(Instruction::PublicInput { guard: None, val_t: IrType::Native, output: id.clone() }),
        C6PushStatementBindingMode::LedgerCall => inputs.push(make_typed(id.clone())?),
    }
    let op = Operand::Variable(id); tr.push(op.clone()); Ok(op)
}

pub fn build_c6_equal_empty_step_circuit(
    source_params: C6PushFamilyParams, meta_len: usize, prog_len: usize,
    meta_bounds_frames_offset: usize, mode: C6PushStatementBindingMode, scope: &str,
) -> Result<C6EqualEmptyStepCircuit, C6EqualEmptyStepError> {
    if scope.trim().is_empty() { return Err(C6EqualEmptyStepError::EmptyScope); }
    let geometry = source_params.geometry();
    let mut call_inputs = Vec::new(); let mut ins = Vec::new(); let mut tr = Vec::new();
    let mut alloc = |label: &str, idx: usize, ci: &mut Vec<TypedIdentifier>,
        ii: &mut Vec<Instruction>, tt: &mut Vec<Operand>| alloc_lane(mode, scope, label, idx, ci, ii, tt);
    let enable = alloc("enable", 0, &mut call_inputs, &mut ins, &mut tr)?;
    let disp = alloc("disposition_is_refused", 0, &mut call_inputs, &mut ins, &mut tr)?;
    let schema = alloc("host_schema", 0, &mut call_inputs, &mut ins, &mut tr)?;
    let carrier = alloc("carrier_version", 0, &mut call_inputs, &mut ins, &mut tr)?;
    let mut vec_of = |label: &str, n: usize, ci: &mut Vec<TypedIdentifier>,
        ii: &mut Vec<Instruction>, tt: &mut Vec<Operand>| -> Result<Vec<Operand>, C6EqualEmptyStepError> {
        let mut o = Vec::with_capacity(n);
        for i in 0..n { o.push(alloc(label, i, ci, ii, tt)?); } Ok(o)
    };
    let pre_meta = vec_of("pre_meta", meta_len, &mut call_inputs, &mut ins, &mut tr)?;
    let pre_prog = vec_of("pre_prog", prog_len, &mut call_inputs, &mut ins, &mut tr)?;
    let pre_ctx_v = vec_of("pre_ctx", CTX_LANES, &mut call_inputs, &mut ins, &mut tr)?;
    let pre_ctx: [Operand; CTX_LANES] = pre_ctx_v.try_into().map_err(|_| C6EqualEmptyStepError::Backend("pre_ctx".into()))?;
    let pre_image = vec_of("pre_image", geometry.image_total_lanes, &mut call_inputs, &mut ins, &mut tr)?;
    let pre_data = vec_of("pre_data", geometry.data_total_lanes, &mut call_inputs, &mut ins, &mut tr)?;
    let pre_helper = vec_of("pre_helper", geometry.helper_total_lanes, &mut call_inputs, &mut ins, &mut tr)?;
    let post_meta = vec_of("post_meta", meta_len, &mut call_inputs, &mut ins, &mut tr)?;
    let post_prog = vec_of("post_prog", prog_len, &mut call_inputs, &mut ins, &mut tr)?;
    let post_ctx_v = vec_of("post_ctx", CTX_LANES, &mut call_inputs, &mut ins, &mut tr)?;
    let post_ctx: [Operand; CTX_LANES] = post_ctx_v.try_into().map_err(|_| C6EqualEmptyStepError::Backend("post_ctx".into()))?;
    let post_image = vec_of("post_image", geometry.image_total_lanes, &mut call_inputs, &mut ins, &mut tr)?;
    let post_data = vec_of("post_data", geometry.data_total_lanes, &mut call_inputs, &mut ins, &mut tr)?;
    let post_helper = vec_of("post_helper", geometry.helper_total_lanes, &mut call_inputs, &mut ins, &mut tr)?;
    let block = emit_c6_equal_empty_step_block(&C6EqualEmptyStepInputs {
        enable, disposition_is_refused_helper_work: disp, host_schema: schema, carrier_version: carrier,
        geometry, meta_bounds_frames_offset, source_params: source_params.clone(),
        pre_meta, pre_prog, pre_ctx, pre_image, pre_data, pre_helper,
        post_meta, post_prog, post_ctx,
        post_image: post_image.clone(), post_data: post_data.clone(), post_helper: post_helper.clone(),
    }, scope)?;
    ins.extend(block.instructions);
    let statement_lane_count = tr.len();
    ins.push(Instruction::Impact { guard: Operand::Immediate(Fr::from(1u64)), inputs: tr });
    let outputs = match mode {
        C6PushStatementBindingMode::LocalPublic => Vec::new(),
        C6PushStatementBindingMode::LedgerCall => {
            let mut vals = Vec::with_capacity(geometry.state_total_lanes);
            vals.extend(post_image); vals.extend(post_data); vals.extend(post_helper);
            ins.push(Instruction::Output { vals });
            vec![IrType::Native; geometry.state_total_lanes]
        }
    };
    let ir0 = IrSource { version: IrMinorVersion::V0, inputs: call_inputs, outputs,
        do_communications_commitment: matches!(mode, C6PushStatementBindingMode::LedgerCall),
        instructions: Arc::new(ins) };
    let mut disk = serde_json::to_value(&ir0).map_err(be)?;
    disk["version"] = json!({ "major": 3, "minor": 0 });
    let serialized_zkir = serde_json::to_string(&disk).map_err(be)?;
    let ir = IrSource::load(serialized_zkir.as_bytes()).map_err(be)?;
    // No instruction-count guard is emitted here. An instruction count is
    // not the circuit degree `k`: the pinned SDK derives `k` from its
    // cost model (`IrSource::k` / `optimal_k`), and `VERIFIER_MAX_DEGREE`
    // is a standard-verifier-key parameter, not a bound this builder may
    // present as a capacity proof. This builder therefore refuses nothing
    // on size grounds; any future coarse host resource limit must be a
    // distinctly named, separately reviewed policy, not a degree claim.
    // No size, `k`, or performance qualification is made by this module.
    Ok(C6EqualEmptyStepCircuit { mode, geometry, meta_len, prog_len,
        meta_bounds_frames_offset, source_params, statement_lane_count, ir, serialized_zkir })
}

/// `ProofPreimage::check` preprocessing only; not synthesis/proving/native.
pub fn check_c6_equal_empty_step_circuit(
    circuit: &C6EqualEmptyStepCircuit, witness: &C6EqualEmptyStepWitness,
) -> Result<Vec<Option<usize>>, C6EqualEmptyStepError> {
    if witness.source_params.geometry() != circuit.geometry {
        return Err(C6EqualEmptyStepError::Backend("witness geometry does not match circuit".into()));
    }
    if witness.host_prefix.meta.len() != circuit.meta_len {
        return Err(C6EqualEmptyStepError::LengthMismatch { section: "meta", expected: circuit.meta_len, actual: witness.host_prefix.meta.len() });
    }
    if witness.host_prefix.prog.len() != circuit.prog_len {
        return Err(C6EqualEmptyStepError::LengthMismatch { section: "prog", expected: circuit.prog_len, actual: witness.host_prefix.prog.len() });
    }
    if witness.host_prefix.meta_bounds_frames_offset != circuit.meta_bounds_frames_offset {
        return Err(C6EqualEmptyStepError::Backend("meta bounds frames offset mismatch".into()));
    }
    let statement = witness.to_statement_frs();
    if statement.len() != circuit.statement_lane_count {
        return Err(C6EqualEmptyStepError::LengthMismatch { section: "statement", expected: circuit.statement_lane_count, actual: statement.len() });
    }
    let preimage = match circuit.mode {
        C6PushStatementBindingMode::LocalPublic => ProofPreimage {
            inputs: Vec::new(), private_transcript: Vec::new(),
            public_transcript_inputs: statement.clone(), public_transcript_outputs: statement,
            binding_input: Fr::from(0u64), communications_commitment: None,
            key_location: KeyLocation(CHECK_KEY_LOCATION.into()) },
        C6PushStatementBindingMode::LedgerCall => {
            let randomness = Fr::from(0u64);
            let mut comm = statement.clone();
            comm.extend(witness.post_flat().iter().map(|&l| Fr::from(l)));
            ProofPreimage {
                inputs: statement.clone(), private_transcript: Vec::new(),
                public_transcript_inputs: statement, public_transcript_outputs: Vec::new(),
                binding_input: Fr::from(0u64),
                communications_commitment: Some((transient_commit(&comm, randomness), randomness)),
                key_location: KeyLocation(CHECK_KEY_LOCATION.into()) }
        }
    };
    preimage.check(&circuit.ir)
        .map_err(|e| C6EqualEmptyStepError::Backend(format!("SDK ProofPreimage::check failed: {e}")))
}

#[cfg(test)]
#[path = "c6_equal_empty_step_tests.rs"]
mod tests;
