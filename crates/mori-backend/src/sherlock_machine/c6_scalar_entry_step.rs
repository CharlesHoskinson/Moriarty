//! C6 `BHelperScalarEntry`: no active helper/result, `ControlC::Return`,
//! and a nonempty frame stack whose actual TOP frame (bottom-to-top
//! index `frame_count - 1`) is one of:
//!
//! * `RightF{op:Add|Sub|Mul|Lt|Cons|Branch}` -> Scalar modes
//!   `Add|Sub|Mul|Lt|Cons|Branch`, `operandA = left`,
//!   `operandB = Some(Return value)`, `expected = None`;
//! * `RightF{op:Eq}` is NOT this rule (it selects Equal);
//! * `UnaryF{op:Leaf|Not}` -> `Leaf|Not`, `operandA = Return value`,
//!   `operandB = None`, `expected = None`;
//! * `IfF` -> `TestBool`, operands as Unary;
//! * `ReturnF{return_type}` -> `TestType`,
//!   `expected = Some(return_type)` copied exactly (builtin `0..=3`
//!   and nominal `>= 4` alike; no UInt-only substitution);
//! * `ConstructorFieldsF{checked:false, field_type}` -> `TestType`,
//!   `expected = Some(field_type)` copied exactly. This is the fifth
//!   site reached through Core
//!   `adt_constructor::scalar_entry -> impl_constructor_scalar`, whose
//!   actual read checks rule/site, absent helper/result, `Return`
//!   control and `checked == false`, and copies `field_type`; it does
//!   NOT call a constructor-arity checker, and neither does this
//!   emitter invent one. `checked:true` selects Append, not Entry.
//!
//! The installed helper is exactly `Scalar{ common:{ site derived
//! from the actual top frame, origin_reference_cursor =
//! pre.reference_cursor, parent_resume: None }, phase: First,
//! reg_a: 0, staged: None }`, with the mode/operand/expected lanes
//! above. The full top frame (type_id/ctor_id/index/completed/rest/
//! saved_env/origin for `ConstructorFieldsF`) is preserved untouched:
//! no frame is popped and control, env, slots, tasks, DATA and the
//! program are unmutated.
//!
//! New enforced connection discharged here, replacing the premise
//! retained by ScalarTag/ScalarArithmetic: the dynamic top-frame row
//! and the `Return` Ref lanes themselves determine site, mode and the
//! exact Ref space/index and TypeId copies installed in the
//! post-helper, so an encodable wrong top frame, left Ref, return
//! Ref, expected TypeId or site cannot satisfy this relation.
//! Retained predecessor premises NOT discharged here: SI08/SI21
//! entry coherence beyond this installation step, SI15/SI20
//! frame/program/origin coherence of the pre-state, SI17 marker
//! placement, full StepInv, program membership / cryptographic
//! provenance, and native issuance. No program snapshot, fixture or
//! per-program specialization is baked into the IR; PROG is the
//! canonical generic runtime carrier, PRE == POST.
//!
//! Driver order mirrors Core: pre-state validity (Running, positive
//! fuel, absent helper/result/tasks, Return control, selected top
//! frame) is PS, before P3. Entry is helper-eligible, so P3 uses the
//! ORIGINAL separate admission `max_helper_work` (from the retained
//! `source_params`, never layout bounds): uncharged
//! `Exhausted{HelperWork}` preserving every lane including the absent
//! helper. Otherwise P4 charges remaining-1 / consumed+1 /
//! helper_work+1 with checked 128-bit no-wrap assertions; P7 adds
//! exactly ZERO helper visits for Entry (`visits_of(entry) == 0`)
//! and the reference cursor is unchanged. Fuel 0 is
//! `KAdminPublishExhausted`, not Entry; there is no allocation delta
//! and therefore no fictional P6 branch.
//!
//! Canonicality composed from actual existing emitters: bound META
//! (retained cfg + separate admission bounds, current C6
//! `C6PushFamilyParams` reconstruction), PROG/CTX canonicality plus
//! expected-CTX binding, and full pre/post DATA and HELPER
//! canonicality. Disabled (`enable == 0`) imposes full pre == post
//! equality for image/DATA/helper (META/PROG/CTX equality is already
//! unconditional) and no selected-shape constraint.
//!
//! Namespace composition: this emitter owns `%{scope}.local.temp.*`
//! plus one retained serial per composed block under `scope`
//! (`.c6data.temp.`, `.c6helper.temp.`, `.c6meta.temp.`,
//! `.c6progctx.temp.`). Caller operands under any composed prefix are
//! refused. The retained emitter advances every serial across `emit`
//! calls; the free function is single-shot per scope.

use midnight_transient_crypto::curve::{Fr, FR_BITS};
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::{Identifier, Instruction};
use mori_core::sherlock::helper_machine::c6_phys::{
    C6_PHYS_MAGIC, C6_PHYS_VERSION, FRAME_LANES, IMAGE_SECTION_HEADER_LANES,
    L_CONSUMED_WORK, L_CONTROL, L_FRAME_COUNT, L_HELPER_WORK, L_OUTCOME,
    L_REFERENCE_CURSOR, L_REMAINING_WORK,
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
// L_HELPER_VISITS is deliberately not mutable for Entry.

#[derive(Debug, Error)]
pub enum C6ScalarEntryStepError {
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

fn be<E: std::fmt::Display>(e: E) -> C6ScalarEntryStepError {
    C6ScalarEntryStepError::Backend(e.to_string())
}

const COMPOSED_RESERVED_SEGMENTS: [&str; 5] = [
    ".local.temp.", ".c6data.temp.", ".c6helper.temp.", ".c6meta.temp.", ".c6progctx.temp.",
];

fn validate_parent_scope(scope: &str) -> Result<(), C6ScalarEntryStepError> {
    if scope.trim().is_empty() { return Err(C6ScalarEntryStepError::EmptyScope); }
    if scope.contains('%') || COMPOSED_RESERVED_SEGMENTS.iter().any(|s| scope.contains(s)) {
        return Err(C6ScalarEntryStepError::InvalidScope);
    }
    Ok(())
}

fn reject_reserved_operand(scope: &str, op: &Operand) -> Result<(), C6ScalarEntryStepError> {
    if let Operand::Variable(id) = op {
        for seg in COMPOSED_RESERVED_SEGMENTS {
            if id.0.starts_with(&format!("%{}{}", scope, seg)) {
                return Err(C6ScalarEntryStepError::ReservedOperand { identifier: id.0.clone() });
            }
        }
    }
    Ok(())
}

#[derive(Clone, Debug)]
pub struct C6ScalarEntryStepInputs {
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

/// Denotation receipt for future consumers. Every `post_*` operand is
/// an UNGATED derived value (it names the applied installation even
/// under P3 or `enable == 0`); only the constrained post-helper lanes
/// in the emitted instructions are the relation's post-state.
#[derive(Clone, Debug)]
pub struct C6ScalarEntryStepBlock {
    pub instructions: Vec<Instruction>,
    pub is_helper_work_max: Operand,
    pub top_frame_tag: Operand,
    pub post_site: Operand,
    pub post_mode: Operand,
    pub post_operand_a_is_data: Operand,
    pub post_operand_a_index: Operand,
    pub post_operand_b_present: Operand,
    pub post_operand_b_is_data: Operand,
    pub post_operand_b_index: Operand,
    pub post_expected_present: Operand,
    pub post_expected: Operand,
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
    fn and(&mut self, a: Operand, b: Operand) -> Operand { self.mul(a, b) }
    fn or(&mut self, a: Operand, b: Operand) -> Operand {
        let na = self.not(a); let nb = self.not(b); let both = self.mul(na, nb); self.not(both)
    }
}

/// Retained-serial composition API; see module docs. The free
/// function `emit_c6_scalar_entry_step_block` is single-shot per
/// scope per circuit.
pub struct C6ScalarEntryStepEmitter {
    scope: String,
    next_temp: usize,
    data_em: C6DataCanonicalEmitter,
    meta_em: C6MetaCanonicalEmitter,
    progctx_em: C6ProgramContextCanonicalEmitter,
    helper_em: C6HelperCanonicalEmitter,
}

impl C6ScalarEntryStepEmitter {
    pub fn new(scope: &str) -> Result<Self, C6ScalarEntryStepError> {
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

    pub fn emit(&mut self, inputs: &C6ScalarEntryStepInputs) -> Result<C6ScalarEntryStepBlock, C6ScalarEntryStepError> {
        let scope: &str = &self.scope;
        validate_parent_scope(scope)?;
        if (LANE_BITS as usize) >= FR_BITS { return Err(C6ScalarEntryStepError::InvalidBitWidth { bits: LANE_BITS }); }
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
            if cap == usize::MAX { return Err(C6ScalarEntryStepError::GeometryOverflow { section: "caps" }); }
        }
        for (s, a, e) in [("image", inputs.pre_image.len(), g.image_total_lanes),
            ("image", inputs.post_image.len(), g.image_total_lanes),
            ("data", inputs.pre_data.len(), g.data_total_lanes),
            ("data", inputs.post_data.len(), g.data_total_lanes),
            ("helper", inputs.pre_helper.len(), g.helper_total_lanes),
            ("helper", inputs.post_helper.len(), g.helper_total_lanes)] {
            if a != e { return Err(C6ScalarEntryStepError::LengthMismatch { section: s, expected: e, actual: a }); }
        }
        if inputs.pre_meta.len() < 3 || inputs.pre_meta.len() != inputs.post_meta.len()
            || inputs.pre_prog.len() != inputs.post_prog.len()
            || inputs.meta_bounds_frames_offset >= inputs.pre_meta.len() {
            return Err(C6ScalarEntryStepError::Backend("host prefix length/offset".into()));
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

        // ---- Selected pre-state (PS, before P3) ----
        em.gzero(en.clone(), pre_h[rec].clone()); // helper None
        em.gzero(en.clone(), pre_h[res].clone()); // result None
        em.gzero(en.clone(), pre_h[3].clone()); // task count 0
        for off in 0..3 { em.gzero(en.clone(), pre_i[L_OUTCOME + off].clone()); } // Running
        { let hf = em.lt128(Em::imm(0), pre_i[L_REMAINING_WORK].clone()); em.gassert(en.clone(), hf); }
        em.geq(en.clone(), pre_i[L_CONTROL].clone(), Em::imm(1)); // ControlC::Return

        // ---- Dynamic TOP frame: slot `depth - 1`, bottom-to-top ----
        let depth = pre_i[L_FRAME_COUNT].clone();
        { let pos = em.lt128(Em::imm(0), depth.clone()); em.gassert(en.clone(), pos); }
        { let le = em.lt128(depth.clone(), Em::imm((p.max_frames as u128) + 1)); em.gassert(en.clone(), le); }
        let mut top: Vec<Operand> = vec![Em::imm(0); FRAME_LANES];
        for slot in 0..p.max_frames {
            let base = IMAGE_SECTION_HEADER_LANES + slot * FRAME_LANES;
            let hit = em.teq(Em::imm((slot as u128) + 1), depth.clone());
            for j in 0..FRAME_LANES {
                top[j] = em.sel(hit.clone(), pre_i[base + j].clone(), top[j].clone());
            }
        }
        // Return value Ref lanes (control-relative Ref at 1..3).
        let ret_kind = pre_i[L_CONTROL + 1].clone();
        let ret_index = pre_i[L_CONTROL + 2].clone();

        // Frame tags: Let0 Left1 Right2 Unary3 If4 Match5 Args6 Return7 Scope8 Ctor9.
        let mut fb: Vec<Operand> = Vec::new();
        for v in 0..10u128 { fb.push(em.teq(top[0].clone(), Em::imm(v))); }
        let is_right = fb[2].clone();
        let is_unary = fb[3].clone();
        let is_if = fb[4].clone();
        let is_return = fb[7].clone();
        let is_ctor = fb[9].clone();
        // BinaryOp lanes: Add0 Sub1 Mul2 Eq3 Lt4 Cons5 Branch6.
        let mut ob: Vec<Operand> = Vec::new();
        for v in 0..7u128 { ob.push(em.teq(top[1].clone(), Em::imm(v))); }
        let right_op_ok = { let mut s = ob[0].clone();
            for b in [&ob[1], &ob[2], &ob[4], &ob[5], &ob[6]] { s = em.add(s, b.clone()); } s };
        let right_valid = em.and(is_right.clone(), right_op_ok);
        let unary_op_ok = em.or(ob[0].clone(), ob[1].clone()); // Leaf0 | Not1 on top[1]
        let unary_valid = em.and(is_unary.clone(), unary_op_ok);
        let ctor_unchecked = em.teq(top[5].clone(), Em::imm(0));
        let ctor_valid = em.and(is_ctor.clone(), ctor_unchecked);
        let selected = { let mut s = right_valid.clone();
            for b in [unary_valid.clone(), is_if.clone(), is_return.clone(), ctor_valid.clone()] {
                s = em.add(s, b); } s };
        em.geq(en.clone(), selected, Em::imm(1)); // exactly one Entry shape

        // ---- Installation derivation from the actual rows ----
        // ScalarMode: Add0 Sub1 Mul2 Lt3 Cons4 Branch5 Leaf6 Not7 TestBool8 TestType9.
        let mode_right = { let mut m = Em::imm(0);
            m = em.sel(ob[1].clone(), Em::imm(1), m);
            m = em.sel(ob[2].clone(), Em::imm(2), m);
            m = em.sel(ob[4].clone(), Em::imm(3), m);
            m = em.sel(ob[5].clone(), Em::imm(4), m);
            m = em.sel(ob[6].clone(), Em::imm(5), m); m };
        let mode_unary = em.sel(ob[1].clone(), Em::imm(7), Em::imm(6)); // Not else Leaf
        let post_mode = { let mut m = Em::imm(9); // TestType (Return/Ctor)
            m = em.sel(is_if.clone(), Em::imm(8), m);
            m = em.sel(unary_valid.clone(), mode_unary, m);
            em.sel(right_valid.clone(), mode_right, m) };
        // SiteTag: RetRight11 RetUnary12 RetIf13 RetCall16 RetCtorFields25.
        let post_site = { let mut s = Em::imm(25);
            s = em.sel(is_return.clone(), Em::imm(16), s);
            s = em.sel(is_if.clone(), Em::imm(13), s);
            s = em.sel(unary_valid.clone(), Em::imm(12), s);
            em.sel(right_valid.clone(), Em::imm(11), s) };
        let post_operand_a_is_data = em.sel(right_valid.clone(), top[2].clone(), ret_kind.clone());
        let post_operand_a_index = em.sel(right_valid.clone(), top[3].clone(), ret_index.clone());
        let post_operand_b_present = right_valid.clone();
        let post_operand_b_is_data = em.sel(right_valid.clone(), ret_kind.clone(), Em::imm(0));
        let post_operand_b_index = em.sel(right_valid.clone(), ret_index.clone(), Em::imm(0));
        let post_expected_present = em.or(is_return.clone(), ctor_valid.clone());
        let post_expected = { let v = em.sel(ctor_valid.clone(), top[4].clone(), Em::imm(0));
            em.sel(is_return.clone(), top[1].clone(), v) };
        let origin_cursor = pre_i[L_REFERENCE_CURSOR].clone();

        // ---- P3 / P4 driver (P7 installs helper, 0 visits) ----
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
        // Every other image lane preserved: helper_visits and
        // reference_cursor included; frames/control/env untouched.
        let mutable = [L_OUTCOME, L_OUTCOME + 1, L_OUTCOME + 2, L_REMAINING_WORK,
            L_CONSUMED_WORK, L_HELPER_WORK];
        for lane in 0..g.image_total_lanes {
            if !mutable.contains(&lane) {
                em.geq(en.clone(), post_i[lane].clone(), pre_i[lane].clone());
            }
        }
        // DATA unchanged (pre canonicality composed below).
        for (a, b) in inputs.pre_data.iter().zip(inputs.post_data.iter()) {
            em.ceq(b.clone(), a.clone());
        }
        // HELPER post: only the Scalar record is installed, only when applied.
        for lane in 0..g.helper_total_lanes {
            let applied_expected = if lane == rec { Em::imm(1) } // Scalar
                else if lane == rec + 1 { post_site.clone() }
                else if lane == rec + 2 { origin_cursor.clone() }
                else if lane == rec + 5 { post_mode.clone() }
                else if lane == rec + 6 { post_operand_a_is_data.clone() }
                else if lane == rec + 7 { post_operand_a_index.clone() }
                else if lane == rec + 8 { post_operand_b_present.clone() }
                else if lane == rec + 9 { post_operand_b_is_data.clone() }
                else if lane == rec + 10 { post_operand_b_index.clone() }
                else if lane == rec + 11 { post_expected_present.clone() }
                else if lane == rec + 12 { post_expected.clone() }
                else if lane == rec + 3 || lane == rec + 4 || lane == rec + 13
                    || lane == rec + 14 || lane == rec + 15 || lane == rec + 16 { Em::imm(0) }
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
        Ok(C6ScalarEntryStepBlock {
            instructions: em.ins,
            is_helper_work_max: is_p3,
            top_frame_tag: top[0].clone(),
            post_site, post_mode,
            post_operand_a_is_data, post_operand_a_index,
            post_operand_b_present, post_operand_b_is_data, post_operand_b_index,
            post_expected_present, post_expected,
        })
    }
}

/// Single-shot block at parent serial zero; see
/// `C6ScalarEntryStepEmitter` for the composition contract.
pub fn emit_c6_scalar_entry_step_block(
    inputs: &C6ScalarEntryStepInputs, scope: &str,
) -> Result<C6ScalarEntryStepBlock, C6ScalarEntryStepError> {
    C6ScalarEntryStepEmitter::new(scope)?.emit(inputs)
}
