//! C6 `BHelperScalarConstruct`: active Scalar helper in exactly the
//! Core-selected shapes `(First, Leaf)`, `(Second, Cons)`, or
//! `(Second, Branch)`. Add/Sub/Mul/Lt/Not are selected by
//! ScalarArithmetic, the Tag modes by ScalarTag, and TestBool/TestType
//! terminate at Tag; none of them is selected here.
//!
//! Branch denotation follows Core `plan_scalar` for
//! `BHelperScalarConstruct`:
//! * `Leaf` reads the actual pre-helper `operandA`. A UInt operand
//!   stages `PendingNode { Leaf }`; every other logical kind stages
//!   `PrimError { 11 }`.
//! * `Cons` reads the actual pre-helper `operandB`, never
//!   `operandA`. A Nil or Cons operand stages
//!   `PendingNode { Cons }`; every other kind stages
//!   `PrimError { 11 }`.
//! * `Branch` reads the actual pre-helper `operandB`, never
//!   `operandA`. A Leaf or Branch operand stages
//!   `PendingNode { Branch }`; every other kind stages
//!   `PrimError { 11 }`.
//! * Every applied branch stages phase `Done`, preserving the full
//!   common fields, mode, operandA, operandB, expected, and `regA`.
//!   No node is allocated here; node allocation is ScalarExit. No
//!   result is published and the helper is not cleared.
//!
//! Both references are read in-circuit with the complete
//! `C6CheckedValueReadEmitter`. Reader A is gated by `enable`:
//! operandA is the effect operand for `Leaf` and the SI22 domain
//! operand for `Cons`/`Branch`. Reader B is gated by
//! `enable && (Second, Cons|Branch)`: operandB is absent for
//! `Leaf`, and its lanes are not interpreted as a reference in that
//! mode. The complete reader blocks, including `selected_found`,
//! `is_data`, `marker_free_present`, raw `marker_free`, and the
//! original Ref space/index lanes, are retained in the block.
//!
//! Premises imposed here beyond the bare selector: helper kind
//! Scalar, the exact phase/mode pairing above, staged `None` with
//! payload 0, `regA == 0`, `operandB` present iff `Cons|Branch`,
//! `expected` absent, result `None`, task count 0, `parent_resume`
//! absent, origin cursor equal to the image reference cursor, and
//! the valid SI22 Second-state operand-A domain (`Cons`: A is UInt;
//! `Branch`: A is Leaf or Branch). These are enforced, not assumed
//! silently, and make this relation stronger than the bare Core
//! selector plus row effect, which trusts an installed helper.
//!
//! Driver order mirrors Core: premises and read walks are PS
//! (before P3), even when P3 is chosen. P3
//! (`pre.helper_work == admission.max_helper_work`, the original
//! separate admission bounds) is uncharged
//! `Exhausted{HelperWork}` with all state preserved. Otherwise P4
//! charges remaining-1/consumed+1/helper_work+1 and P7 adds exactly
//! one helper visit; the reference cursor is unchanged. No
//! allocation, env, frame, task, value-slot, field, slot, control,
//! program, or DATA delta exists for this rule, so there is no P6
//! cap refusal. The post helper is still present; there is no
//! HelperExit result or clearing.
//!
//! Retained caller premises, not discharged here: SI08/SI21 entry
//! coherence, namely that ScalarEntry and, for the binary shapes,
//! ScalarTag installed this helper from the actual control/top
//! frame with the site lane agreeing with that frame; SI17 marker
//! placement; SI21/SI22/StepInv beyond the lanes enforced here;
//! program membership; and native issuance. No site value is
//! guessed from the helper lanes alone.
//!
//! Namespace composition: this emitter owns
//! `%{scope}.local.temp.*` plus one retained serial per composed
//! block under `scope`. The readers are composed under the child
//! scope `{scope}.constructread`, because both reader and parent
//! mint `.local.temp.`-family serials; no same-scope serial is
//! initialised twice. Caller operands under any composed prefix,
//! parent or child, are refused. The retained emitter advances
//! every serial across `emit` calls; the free function is
//! single-shot per scope.

use midnight_transient_crypto::curve::{Fr, FR_BITS};
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::{Identifier, Instruction};
use mori_core::sherlock::helper_machine::c6_phys::{
    C6_PHYS_MAGIC, C6_PHYS_VERSION, L_CONSUMED_WORK, L_HELPER_VISITS,
    L_HELPER_WORK, L_OUTCOME, L_REFERENCE_CURSOR, L_REMAINING_WORK,
};
use mori_core::sherlock::helper_machine::c6_phys_helper::{L_HELPER_RECORD, L_RESULT_RECORD};
use mori_core::sherlock::helper_machine::c6_phys_host::{CTX_LANES, C6_HOST_SCHEMA};
use thiserror::Error;

use super::c6_checked_value_read::{
    C6CheckedValueReadBlock, C6CheckedValueReadEmitter, C6CheckedValueReadInputs,
};
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

#[derive(Debug, Error)]
pub enum C6ScalarConstructStepError {
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

fn be<E: std::fmt::Display>(e: E) -> C6ScalarConstructStepError {
    C6ScalarConstructStepError::Backend(e.to_string())
}

const COMPOSED_RESERVED_SEGMENTS: [&str; 8] = [
    ".local.temp.", ".c6data.temp.", ".c6helper.temp.", ".c6meta.temp.", ".c6progctx.temp.",
    ".constructread.local.temp.", ".constructread.c6progctx.temp.", ".constructread.c6data.temp.",
];

fn validate_parent_scope(scope: &str) -> Result<(), C6ScalarConstructStepError> {
    if scope.trim().is_empty() { return Err(C6ScalarConstructStepError::EmptyScope); }
    if scope.contains('%') || COMPOSED_RESERVED_SEGMENTS.iter().any(|s| scope.contains(s)) {
        return Err(C6ScalarConstructStepError::InvalidScope);
    }
    Ok(())
}

fn reject_reserved_operand(scope: &str, op: &Operand) -> Result<(), C6ScalarConstructStepError> {
    if let Operand::Variable(id) = op {
        for seg in COMPOSED_RESERVED_SEGMENTS {
            if id.0.starts_with(&format!("%{}{}", scope, seg)) {
                return Err(C6ScalarConstructStepError::ReservedOperand { identifier: id.0.clone() });
            }
        }
    }
    Ok(())
}

#[derive(Clone, Debug)]
pub struct C6ScalarConstructStepInputs {
    pub enable: Operand,
    /// Statement lane constrained in-circuit to exactly P3
    /// (`pre.helper_work == retained admission max_helper_work`).
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

/// The denotation receipt for future consumers: both complete
/// reader blocks (full instructions and every exported projection
/// and raw marker lane), the original Ref lanes taken from the
/// pre-helper itself, and the derived post-helper Scalar lanes.
/// The `post_*` operands are ungated derived receipts; only the
/// constrained post-helper lanes in `instructions` are the
/// relation's post-state.
/// `is_helper_work_max` is likewise an ungated derived receipt:
/// it is the raw `pre.helper_work == max_helper_work` equality
/// (`is_p3`), computed without `enable`. When `enable == 0` and
/// `pre.helper_work == max_helper_work`, it exports `1`. In that
/// disabled case the disposition lane is not constrained to P3
/// and no charged step occurs; only the disabled identity
/// relation applies. The constrained post-state in
/// `instructions` is the identity post-state (full pre/post
/// equality for image/helper and the already-unconditional
/// META/PROG/CTX/DATA equality), not a P3 or Applied post-state.
/// The receipt does not alter that identity constraint.
/// It must never be treated as enabled, constrained-disposition,
/// or post-state truth; only the constrained post-helper lanes
/// in `instructions` are the relation's post-state.
#[derive(Clone, Debug)]
pub struct C6ScalarConstructStepBlock {
    pub instructions: Vec<Instruction>,
    pub reader_a: C6CheckedValueReadBlock,
    pub reader_b: C6CheckedValueReadBlock,
    pub original_ref_a_is_data: Operand,
    pub original_ref_a_index: Operand,
    pub operand_b_present: Operand,
    pub original_ref_b_is_data: Operand,
    pub original_ref_b_index: Operand,
    /// Ungated raw `is_p3` receipt; see `C6ScalarConstructStepBlock`.
    pub is_helper_work_max: Operand,
    pub post_phase: Operand,
    pub post_reg_a: Operand,
    pub post_staged_tag: Operand,
    pub post_staged_payload: Operand,
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
    fn lt(&mut self, a: Operand, b: Operand, bits: u32) -> Operand {
        let o = self.t(); self.ins.push(Instruction::LessThan { a, b, bits, output: o.clone() }); Operand::Variable(o)
    }
    fn lt128(&mut self, a: Operand, b: Operand) -> Operand { self.lt(a, b, LANE_BITS) }
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
}

/// Retained-serial composition API; see module docs. The free
/// function `emit_c6_scalar_construct_step_block` is single-shot
/// per scope per circuit.
pub struct C6ScalarConstructStepEmitter {
    scope: String,
    next_temp: usize,
    reader_em: C6CheckedValueReadEmitter,
    meta_em: C6MetaCanonicalEmitter,
    progctx_em: C6ProgramContextCanonicalEmitter,
    helper_em: C6HelperCanonicalEmitter,
}

impl C6ScalarConstructStepEmitter {
    pub fn new(scope: &str) -> Result<Self, C6ScalarConstructStepError> {
        validate_parent_scope(scope)?;
        Ok(Self {
            scope: scope.to_string(),
            next_temp: 0,
            reader_em: C6CheckedValueReadEmitter::new(&format!("{scope}.constructread")).map_err(be)?,
            meta_em: C6MetaCanonicalEmitter::new(scope).map_err(be)?,
            progctx_em: C6ProgramContextCanonicalEmitter::new(scope).map_err(be)?,
            helper_em: C6HelperCanonicalEmitter::new(scope).map_err(be)?,
        })
    }
    pub fn scope(&self) -> &str { &self.scope }
    pub fn next_temp(&self) -> usize { self.next_temp }

    pub fn emit(&mut self, inputs: &C6ScalarConstructStepInputs) -> Result<C6ScalarConstructStepBlock, C6ScalarConstructStepError> {
        let scope: &str = &self.scope;
        validate_parent_scope(scope)?;
        if (LANE_BITS as usize) >= FR_BITS { return Err(C6ScalarConstructStepError::InvalidBitWidth { bits: LANE_BITS }); }
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
        inputs.source_params.validate_geometry(inputs.geometry)?;
        let g = &inputs.geometry;
        let caps = &g.caps; let p = &caps.phys;
        for cap in [p.max_frames, p.max_expressions, p.max_names, p.max_arms, p.max_children,
            p.max_value_slots, p.max_constants, p.max_data_nodes, p.max_env_cells, p.max_functions,
            p.max_arity, p.max_types, p.max_constructors, p.max_ctor_fields, caps.max_data_field_slots,
            caps.max_helper_tasks] {
            if cap == usize::MAX { return Err(C6ScalarConstructStepError::GeometryOverflow { section: "caps" }); }
        }
        for (s, a, e) in [("image", inputs.pre_image.len(), g.image_total_lanes),
            ("image", inputs.post_image.len(), g.image_total_lanes),
            ("data", inputs.pre_data.len(), g.data_total_lanes),
            ("data", inputs.post_data.len(), g.data_total_lanes),
            ("helper", inputs.pre_helper.len(), g.helper_total_lanes),
            ("helper", inputs.post_helper.len(), g.helper_total_lanes)] {
            if a != e { return Err(C6ScalarConstructStepError::LengthMismatch { section: s, expected: e, actual: a }); }
        }
        if inputs.pre_meta.len() < 3 || inputs.pre_meta.len() != inputs.post_meta.len()
            || inputs.pre_prog.len() != inputs.post_prog.len()
            || inputs.meta_bounds_frames_offset >= inputs.pre_meta.len() {
            return Err(C6ScalarConstructStepError::Backend("host prefix length/offset".into()));
        }
        let adm = inputs.source_params.admission_bounds();
        let max_hw = adm.max_helper_work;
        let rec = L_HELPER_RECORD; let res = L_RESULT_RECORD;

        let mut em = Em { scope, next: self.next_temp, ins: Vec::new() };
        let en = inputs.enable.clone();
        let dp = inputs.disposition_is_refused_helper_work.clone();
        em.cbool(en.clone()); em.cbool(dp.clone());
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

        // Mode bits: Add0 Sub1 Mul2 Lt3 Cons4 Branch5 Leaf6 Not7 TestBool8 TestType9.
        // Phase lanes: First0 Second1 Done2.
        let mode = pre_h[rec + 5].clone();
        let mut mb: Vec<Operand> = Vec::new();
        for v in 0..10u128 { mb.push(em.teq(mode.clone(), Em::imm(v))); }
        let is_cons = mb[4].clone();
        let is_branch = mb[5].clone();
        let is_leaf = mb[6].clone();
        let is_first = em.teq(pre_h[rec + 13].clone(), Em::imm(0));
        let is_second = em.teq(pre_h[rec + 13].clone(), Em::imm(1));
        let cons_or_branch = em.or(is_cons.clone(), is_branch.clone());
        let second_construct = em.and(is_second.clone(), cons_or_branch.clone());
        let binary_gate = em.and(en.clone(), second_construct.clone());

        // ---- PS read A: checked operand-A read over the actual
        // pre-helper Ref lanes. The complete reader instructions
        // are composed, not re-derived. This is the Leaf effect
        // read and the SI22 Second-state domain read.
        let reader_a = self.reader_em.emit(&C6CheckedValueReadInputs {
            gate: en.clone(),
            ref_kind_is_data: pre_h[rec + 6].clone(),
            ref_index: pre_h[rec + 7].clone(),
            pre_prog: inputs.pre_prog.clone(),
            pre_data: inputs.pre_data.clone(),
            caps: *caps,
        }).map_err(be)?;
        em.ins.extend(reader_a.instructions.clone());

        // ---- PS read B: checked operand-B read, gated off for
        // `Leaf`, whose absent operandB lanes are not a reference.
        let reader_b = self.reader_em.emit(&C6CheckedValueReadInputs {
            gate: binary_gate.clone(),
            ref_kind_is_data: pre_h[rec + 9].clone(),
            ref_index: pre_h[rec + 10].clone(),
            pre_prog: inputs.pre_prog.clone(),
            pre_data: inputs.pre_data.clone(),
            caps: *caps,
        }).map_err(be)?;
        em.ins.extend(reader_b.instructions.clone());

        // ---- Selected pre-state premises (see module docs) ----
        em.geq(en.clone(), pre_h[rec].clone(), Em::imm(1)); // Scalar
        let pair_leaf = em.and(is_first, is_leaf.clone());
        let pair_second = em.and(is_second.clone(), cons_or_branch.clone());
        let pair = em.add(pair_leaf, pair_second);
        em.geq(en.clone(), pair, Em::imm(1)); // (First,Leaf) or (Second,Cons|Branch)
        em.gzero(en.clone(), pre_h[rec + 15].clone()); // staged None tag
        em.gzero(en.clone(), pre_h[rec + 16].clone()); // staged None payload
        em.gzero(en.clone(), pre_h[rec + 14].clone()); // regA 0 in all three shapes
        em.gzero(en.clone(), pre_h[res].clone()); // result None
        em.geq(en.clone(), pre_h[3].clone(), Em::imm(0)); // task count 0
        em.gzero(en.clone(), pre_h[rec + 3].clone()); // parent_resume absent
        em.geq(en.clone(), pre_h[rec + 2].clone(), pre_i[L_REFERENCE_CURSOR].clone());
        for off in 0..3 { em.gzero(en.clone(), pre_i[L_OUTCOME + off].clone()); } // Running
        { let hf = em.lt128(Em::imm(0), pre_i[L_REMAINING_WORK].clone()); em.gassert(en.clone(), hf); }
        em.geq(en.clone(), pre_h[rec + 8].clone(), second_construct.clone()); // operand_b present iff Cons|Branch
        em.gzero(en.clone(), pre_h[rec + 11].clone()); // expected absent
        em.geq(en.clone(), reader_a.selected_found.clone(), Em::imm(1));
        em.gassert(binary_gate.clone(), reader_b.selected_found.clone());
        // Valid SI22 Second-state operand-A domain. The staged
        // result below is derived only from reader_b for these
        // modes; this gate only excludes pre-states that ScalarTag
        // could not have produced at Second.
        { let second_and_cons = em.and(is_second.clone(), is_cons.clone());
          let g = em.and(en.clone(), second_and_cons);
          em.gassert(g, reader_a.is_uint.clone()); }
        { let second_and_branch = em.and(is_second.clone(), is_branch.clone());
          let g = em.and(en.clone(), second_and_branch);
          let leaf_or_branch = em.or(reader_a.is_leaf.clone(), reader_a.is_branch.clone());
          em.gassert(g, leaf_or_branch); }

        // ---- Core branch staging on the actual referenced rows ----
        let cons_ok = em.or(reader_b.is_nil.clone(), reader_b.is_cons.clone());
        let branch_ok = em.or(reader_b.is_leaf.clone(), reader_b.is_branch.clone());
        let pending_leaf = em.and(is_leaf.clone(), reader_a.is_uint.clone());
        let pending_cons = em.and(is_cons.clone(), cons_ok);
        let pending_branch = em.and(is_branch.clone(), branch_ok);
        let is_pending = { let s = em.or(pending_leaf.clone(), pending_cons.clone()); em.or(s, pending_branch.clone()) };
        let post_staged_tag = em.sel(is_pending, Em::imm(3), Em::imm(4)); // PendingNode else PrimError
        let post_staged_payload = {
            let mut pl = Em::imm(11); // PrimError { 11 }
            pl = em.sel(pending_cons, Em::imm(0), pl); // NodeKind::Cons
            pl = em.sel(pending_leaf, Em::imm(1), pl); // NodeKind::Leaf
            em.sel(pending_branch, Em::imm(2), pl) // NodeKind::Branch
        };
        let post_phase = Em::imm(2); // Done
        let post_reg_a = pre_h[rec + 14].clone();

        // ---- P3 / P4 / P7 driver ----
        let is_p3 = em.teq(pre_i[L_HELPER_WORK].clone(), Em::imm(max_hw));
        em.geq(en.clone(), dp.clone(), is_p3.clone());
        let not_p3 = em.not(is_p3.clone());
        let charged_gate = em.and(en.clone(), not_p3.clone());
        let exp_o0 = em.sel(is_p3.clone(), Em::imm(3), Em::imm(0));
        em.geq(en.clone(), post_i[L_OUTCOME].clone(), exp_o0);
        let exp_o1 = em.sel(is_p3.clone(), Em::imm(5), Em::imm(0));
        em.geq(en.clone(), post_i[L_OUTCOME + 1].clone(), exp_o1);
        em.gzero(en.clone(), post_i[L_OUTCOME + 2].clone());
        { let s = em.add(post_i[L_REMAINING_WORK].clone(), Em::imm(1));
          em.geq(charged_gate.clone(), s, pre_i[L_REMAINING_WORK].clone());
          let pg = em.and(en.clone(), is_p3.clone());
          em.geq(pg, post_i[L_REMAINING_WORK].clone(), pre_i[L_REMAINING_WORK].clone()); }
        for lane in [L_CONSUMED_WORK, L_HELPER_WORK] {
            let noov = em.lt128(pre_i[lane].clone(), Em::imm(u128::MAX));
            em.gassert(charged_gate.clone(), noov);
            let exp = em.add(pre_i[lane].clone(), Em::imm(1));
            let exp2 = em.sel(is_p3.clone(), pre_i[lane].clone(), exp);
            em.geq(en.clone(), post_i[lane].clone(), exp2);
        }
        { let noov = em.lt128(pre_i[L_HELPER_VISITS].clone(), Em::imm(u128::MAX));
          em.gassert(charged_gate.clone(), noov);
          let exp = em.add(pre_i[L_HELPER_VISITS].clone(), Em::imm(1));
          let exp2 = em.sel(is_p3.clone(), pre_i[L_HELPER_VISITS].clone(), exp);
          em.geq(en.clone(), post_i[L_HELPER_VISITS].clone(), exp2); }
        // Every other image lane preserved, reference cursor included.
        let mutable = [L_OUTCOME, L_OUTCOME + 1, L_OUTCOME + 2, L_REMAINING_WORK,
            L_CONSUMED_WORK, L_HELPER_WORK, L_HELPER_VISITS];
        for lane in 0..g.image_total_lanes {
            if !mutable.contains(&lane) {
                em.geq(en.clone(), post_i[lane].clone(), pre_i[lane].clone());
            }
        }
        // DATA unchanged (pre canonicality is the readers' composed blocks).
        for (a, b) in inputs.pre_data.iter().zip(inputs.post_data.iter()) {
            em.ceq(b.clone(), a.clone());
        }
        // HELPER post: only phase/staged change, only when applied.
        for lane in 0..g.helper_total_lanes {
            let applied_expected = if lane == rec + 13 { post_phase.clone() }
                else if lane == rec + 15 { post_staged_tag.clone() }
                else if lane == rec + 16 { post_staged_payload.clone() }
                else { pre_h[lane].clone() };
            let exp = em.sel(is_p3.clone(), pre_h[lane].clone(), applied_expected);
            em.geq(en.clone(), post_h[lane].clone(), exp);
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
        let hb1 = self.helper_em.emit(&C6HelperCanonicalInputs {
            gate: en.clone(), helper: inputs.pre_helper.clone(), caps: *caps }).map_err(be)?;
        em.ins.extend(hb1.instructions);
        let hb2 = self.helper_em.emit(&C6HelperCanonicalInputs {
            gate: en.clone(), helper: inputs.post_helper.clone(), caps: *caps }).map_err(be)?;
        em.ins.extend(hb2.instructions);

        // Disabled: full equality in image and helper (DATA/META/PROG/CTX
        // equality above is already unconditional).
        for (a, b) in inputs.pre_image.iter().zip(inputs.post_image.iter())
            .chain(inputs.pre_helper.iter().zip(inputs.post_helper.iter())) {
            em.geq(disabled.clone(), b.clone(), a.clone());
        }
        self.next_temp = em.next;
        Ok(C6ScalarConstructStepBlock {
            instructions: em.ins,
            reader_a,
            reader_b,
            original_ref_a_is_data: pre_h[rec + 6].clone(),
            original_ref_a_index: pre_h[rec + 7].clone(),
            operand_b_present: pre_h[rec + 8].clone(),
            original_ref_b_is_data: pre_h[rec + 9].clone(),
            original_ref_b_index: pre_h[rec + 10].clone(),
            is_helper_work_max: is_p3,
            post_phase, post_reg_a, post_staged_tag, post_staged_payload,
        })
    }
}

/// Single-shot block at parent serial zero; see
/// `C6ScalarConstructStepEmitter` for the composition contract.
pub fn emit_c6_scalar_construct_step_block(
    inputs: &C6ScalarConstructStepInputs, scope: &str,
) -> Result<C6ScalarConstructStepBlock, C6ScalarConstructStepError> {
    C6ScalarConstructStepEmitter::new(scope)?.emit(inputs)
}
