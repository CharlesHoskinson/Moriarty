//! C6 `BHelperScalarTag`: active Scalar helper at phase `First` in one
//! of the eight Tag modes (Add/Sub/Mul/Lt/Cons/Branch/TestBool/TestType;
//! Leaf/Not are selected by ScalarConstruct/ScalarArithmetic, not here).
//!
//! The transition consumes the complete `C6CheckedValueReadBlock`
//! instructions over the same runtime PROG and padded DATA operands.
//! The reader's reference operands are the pre-helper `operandA` lanes
//! themselves, so the selected kind/payload is derived from the actual
//! referenced row in the encoded pre-helper's own space; no host kind,
//! sort, or found flag is accepted. The full reader block is retained
//! in the emitted block (`selected_found`, `is_data`,
//! `marker_free_present`, raw `marker_free`, original Ref space/index)
//! for future SI17/identity consumers.
//!
//! TestType follows Core `scalar_test_type_ok`: Marker (and any
//! reserved kind, which the reader cannot produce) yields `ok=false`;
//! builtin kinds use `sort_of_kind` (UInt 0, Bool 1, Nil/Cons 2,
//! Leaf/Branch 3) even when `max_types == 0`; in TestType mode only,
//! Constructor uses the nominal capture hook
//! (`adt_checked_ctor_arity`): a Constructor row that fails nominal
//! validation rejects in-circuit, exactly as the Core read errors
//! before any plan exists. In every other mode a Constructor operand
//! follows Core's kind-only branch (numeric/Cons/Branch mismatch
//! `PrimError{11}`, TestBool `Selector{NotBool}`) with no nominal
//! assertion. Generic nominal induction
//! beyond this one capture (SI08/SI17/SI21/SI22, StepInv, program
//! membership, native issuance) remains REQUIRED_OPEN; this module is
//! one selected relation under the stated selected pre-state premises,
//! not full Tag acceptance.
//!
//! Driver order mirrors Core: the selected premises and both read
//! walks are PS (before P3), even when P3 is chosen. P3
//! (`pre.helper_work == admission.max_helper_work`, the ORIGINAL
//! separate admission bounds) is uncharged `Exhausted{HelperWork}` with
//! all state preserved. Otherwise P4 charges remaining-1/consumed+1/
//! helper_work+1 and P7 adds exactly one helper visit; the reference
//! cursor is unchanged. No allocation, env, frame, task, or value-slot
//! delta exists for this rule, so there is no P6 cap refusal. Post
//! helper is still present; there is no HelperExit result or clearing.
//!
//! Premises imposed here beyond the bare selector (spelled out, per
//! the partial SI22 First shape): phase `First`, `regA == 0`, staged
//! tag and payload `0`, `operand_b` present iff mode is binary
//! (Add..Branch), `expected` present iff TestType, result `None`, task
//! count `0`, `parent_resume` absent, origin cursor equal to the image
//! reference cursor. These are enforced, not assumed silently.
//!
//! Namespace composition: this emitter owns `%{scope}.local.temp.*`
//! plus one retained serial per composed block under `scope`
//! (`.c6data.temp.` is used only inside the reader child). The reader
//! is composed under the child scope `{scope}.tagread` and the nominal
//! capture under `{scope}.tagnom`, because both reader and parent mint
//! `.local.temp.`-family serials; no same-scope serial is initialised
//! twice. Caller operands under any composed prefix, parent or child,
//! are refused. The retained emitter advances every serial across
//! `emit` calls; the free function is single-shot per scope.

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
    C6ProgramContextCheckedSortCaptureInputs, C6ProgramContextCtxBindingInputs,
};
use super::c6_push_step::{
    C6PaddedCarrierGeometry, C6PushFamilyParams, C6PushStepError, C6_PADDED_CARRIER_VERSION,
};

const LANE_BITS: u32 = 128;

#[derive(Debug, Error)]
pub enum C6ScalarTagStepError {
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

fn be<E: std::fmt::Display>(e: E) -> C6ScalarTagStepError {
    C6ScalarTagStepError::Backend(e.to_string())
}

const COMPOSED_RESERVED_SEGMENTS: [&str; 9] = [
    ".local.temp.", ".c6data.temp.", ".c6helper.temp.", ".c6meta.temp.", ".c6progctx.temp.",
    ".tagread.local.temp.", ".tagread.c6progctx.temp.", ".tagread.c6data.temp.",
    ".tagnom.c6progctx.temp.",
];

fn validate_parent_scope(scope: &str) -> Result<(), C6ScalarTagStepError> {
    if scope.trim().is_empty() { return Err(C6ScalarTagStepError::EmptyScope); }
    if scope.contains('%') || COMPOSED_RESERVED_SEGMENTS.iter().any(|s| scope.contains(s)) {
        return Err(C6ScalarTagStepError::InvalidScope);
    }
    Ok(())
}

fn reject_reserved_operand(scope: &str, op: &Operand) -> Result<(), C6ScalarTagStepError> {
    if let Operand::Variable(id) = op {
        for seg in COMPOSED_RESERVED_SEGMENTS {
            if id.0.starts_with(&format!("%{}{}", scope, seg)) {
                return Err(C6ScalarTagStepError::ReservedOperand { identifier: id.0.clone() });
            }
        }
    }
    Ok(())
}

#[derive(Clone, Debug)]
pub struct C6ScalarTagStepInputs {
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

/// The denotation receipt for future consumers: the complete reader
/// block (full instructions, `selected_found`, `is_data`,
/// `marker_free_present`, raw `marker_free`), the original RefC
/// space/index operands taken from the pre-helper itself, the gated
/// nominal-validity and checked-sort operands, and the derived
/// post-helper Scalar lanes. `nominal_valid` and `checked_sort_ok` are
/// exported as exact `0` when `enable == 0`.
#[derive(Clone, Debug)]
pub struct C6ScalarTagStepBlock {
    pub instructions: Vec<Instruction>,
    pub reader: C6CheckedValueReadBlock,
    pub original_ref_is_data: Operand,
    pub original_ref_index: Operand,
    pub nominal_valid: Operand,
    pub checked_sort_ok: Operand,
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
    fn lt129(&mut self, a: Operand, b: Operand) -> Operand { self.lt(a, b, 129) }
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

/// Retained-serial composition API; see module docs. The free function
/// `emit_c6_scalar_tag_step_block` is single-shot per scope per circuit.
pub struct C6ScalarTagStepEmitter {
    scope: String,
    next_temp: usize,
    reader_em: C6CheckedValueReadEmitter,
    nominal_em: C6ProgramContextCanonicalEmitter,
    meta_em: C6MetaCanonicalEmitter,
    progctx_em: C6ProgramContextCanonicalEmitter,
    helper_em: C6HelperCanonicalEmitter,
}

impl C6ScalarTagStepEmitter {
    pub fn new(scope: &str) -> Result<Self, C6ScalarTagStepError> {
        validate_parent_scope(scope)?;
        Ok(Self {
            scope: scope.to_string(),
            next_temp: 0,
            reader_em: C6CheckedValueReadEmitter::new(&format!("{scope}.tagread")).map_err(be)?,
            nominal_em: C6ProgramContextCanonicalEmitter::new(&format!("{scope}.tagnom")).map_err(be)?,
            meta_em: C6MetaCanonicalEmitter::new(scope).map_err(be)?,
            progctx_em: C6ProgramContextCanonicalEmitter::new(scope).map_err(be)?,
            helper_em: C6HelperCanonicalEmitter::new(scope).map_err(be)?,
        })
    }
    pub fn scope(&self) -> &str { &self.scope }
    pub fn next_temp(&self) -> usize { self.next_temp }

    pub fn emit(&mut self, inputs: &C6ScalarTagStepInputs) -> Result<C6ScalarTagStepBlock, C6ScalarTagStepError> {
        let scope: &str = &self.scope;
        validate_parent_scope(scope)?;
        if (LANE_BITS as usize) >= FR_BITS { return Err(C6ScalarTagStepError::InvalidBitWidth { bits: LANE_BITS }); }
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
            if cap == usize::MAX { return Err(C6ScalarTagStepError::GeometryOverflow { section: "caps" }); }
        }
        for (s, a, e) in [("image", inputs.pre_image.len(), g.image_total_lanes),
            ("image", inputs.post_image.len(), g.image_total_lanes),
            ("data", inputs.pre_data.len(), g.data_total_lanes),
            ("data", inputs.post_data.len(), g.data_total_lanes),
            ("helper", inputs.pre_helper.len(), g.helper_total_lanes),
            ("helper", inputs.post_helper.len(), g.helper_total_lanes)] {
            if a != e { return Err(C6ScalarTagStepError::LengthMismatch { section: s, expected: e, actual: a }); }
        }
        if inputs.pre_meta.len() < 3 || inputs.pre_meta.len() != inputs.post_meta.len()
            || inputs.pre_prog.len() != inputs.post_prog.len()
            || inputs.meta_bounds_frames_offset >= inputs.pre_meta.len() {
            return Err(C6ScalarTagStepError::Backend("host prefix length/offset".into()));
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

        // ---- PS read 1: checked operand-A read. The reference operands
        // are the pre-helper operandA lanes themselves; the complete
        // reader instructions are composed, not re-derived.
        let reader = self.reader_em.emit(&C6CheckedValueReadInputs {
            gate: en.clone(),
            ref_kind_is_data: pre_h[rec + 6].clone(),
            ref_index: pre_h[rec + 7].clone(),
            pre_prog: inputs.pre_prog.clone(),
            pre_data: inputs.pre_data.clone(),
            caps: *caps,
        }).map_err(be)?;
        em.ins.extend(reader.instructions.clone());

        // ---- PS read 2: nominal Constructor capture over the same PROG.
        let nominal = self.nominal_em.emit_prog_checked_sort_capture(
            &C6ProgramContextCheckedSortCaptureInputs {
                gate: en.clone(),
                pre_prog: inputs.pre_prog.clone(),
                caps: *caps,
                selected_type_id: reader.type_id.clone(),
                selected_ctor_id: reader.ctor_id.clone(),
            }).map_err(be)?;
        em.ins.extend(nominal.instructions);

        // Nominal validity, Core `adt_checked_ctor_arity` order:
        // user type (>= 4), actual type row, actual ctor row, owning
        // type, ctor range inside the type row, checked field end.
        // 129-bit comparisons: starts/lengths/counts are 128-bit lanes.
        let nominal_valid = {
            let lt4 = em.lt128(reader.type_id.clone(), Em::imm(4));
            let ge4 = em.not(lt4);
            let owning = em.teq(nominal.owning_type.clone(), reader.type_id.clone());
            let end = em.add(nominal.ctor_start.clone(), nominal.ctor_len.clone());
            let below_end = em.lt129(reader.ctor_id.clone(), end);
            let below_start = em.lt129(reader.ctor_id.clone(), nominal.ctor_start.clone());
            let at_least_start = em.not(below_start);
            let fend = em.add(nominal.field_start.clone(), nominal.field_len.clone());
            let count_below = em.lt129(nominal.ctor_field_count.clone(), fend);
            let field_ok = em.not(count_below);
            let v = em.and(ge4, nominal.type_found.clone());
            let v = em.and(v, nominal.ctor_found.clone());
            let v = em.and(v, owning);
            let v = em.and(v, at_least_start);
            let v = em.and(v, below_end);
            em.and(v, field_ok)
        };

        // Checked sort and TestType ok. Builtins ignore `max_types`.
        let checked_sort_ok = {
            let mut sort = Em::imm(0);
            sort = em.sel(reader.is_bool.clone(), Em::imm(1), sort);
            let nil_cons = em.or(reader.is_nil.clone(), reader.is_cons.clone());
            sort = em.sel(nil_cons, Em::imm(2), sort);
            let leaf_branch = em.or(reader.is_leaf.clone(), reader.is_branch.clone());
            sort = em.sel(leaf_branch, Em::imm(3), sort);
            sort = em.sel(reader.is_constructor.clone(), reader.type_id.clone(), sort);
            let raw_ok = em.teq(sort, pre_h[rec + 12].clone());
            let not_marker = em.not(reader.is_marker.clone());
            em.and(not_marker, raw_ok)
        };

        // ---- Selected pre-state premises (see module docs) ----
        em.geq(en.clone(), pre_h[rec].clone(), Em::imm(1)); // Scalar
        em.gzero(en.clone(), pre_h[rec + 13].clone()); // phase First
        em.gzero(en.clone(), pre_h[rec + 14].clone()); // regA 0 (partial SI22)
        em.gzero(en.clone(), pre_h[rec + 15].clone()); // staged None tag
        em.gzero(en.clone(), pre_h[rec + 16].clone()); // staged None payload
        em.gzero(en.clone(), pre_h[res].clone()); // result None
        em.geq(en.clone(), pre_h[3].clone(), Em::imm(0)); // task count 0
        em.gzero(en.clone(), pre_h[rec + 3].clone()); // parent_resume absent
        em.geq(en.clone(), pre_h[rec + 2].clone(), pre_i[L_REFERENCE_CURSOR].clone());
        for off in 0..3 { em.gzero(en.clone(), pre_i[L_OUTCOME + off].clone()); } // Running
        { let hf = em.lt128(Em::imm(0), pre_i[L_REMAINING_WORK].clone()); em.gassert(en.clone(), hf); }
        em.geq(en.clone(), reader.selected_found.clone(), Em::imm(1));

        // Mode bits: Add0 Sub1 Mul2 Lt3 Cons4 Branch5 Leaf6 Not7 TestBool8 TestType9.
        let mode = pre_h[rec + 5].clone();
        let mut mb: Vec<Operand> = Vec::new();
        for v in 0..10u128 { mb.push(em.teq(mode.clone(), Em::imm(v))); }
        let allowed = { let mut s = mb[0].clone(); for b in mb[1..6].iter().chain(mb[8..10].iter()) { s = em.add(s, b.clone()); } s };
        em.geq(en.clone(), allowed, Em::imm(1)); // Leaf/Not excluded here
        let is_binary = { let mut s = mb[0].clone(); for b in mb[1..6].iter() { s = em.add(s, b.clone()); } s };
        em.geq(en.clone(), pre_h[rec + 8].clone(), is_binary); // operand_b present iff binary
        em.geq(en.clone(), pre_h[rec + 11].clone(), mb[9].clone()); // expected iff TestType

        // Core invokes `checked_sort`/`adt_checked_ctor_arity` only
        // through TestType; only there does a nominally invalid
        // Constructor reject instead of producing the kind-only result.
        let ctor_gate = {
            let g = em.and(en.clone(), reader.is_constructor.clone());
            em.and(g, mb[9].clone())
        };
        em.gassert(ctor_gate, nominal_valid.clone());

        // ---- Core Tag branch on the actual referenced row ----
        let is_num = { let o = em.or(mb[0].clone(), mb[1].clone()); let o = em.or(o, mb[2].clone()); em.or(o, mb[3].clone()) };
        let num_ok = em.and(is_num.clone(), reader.is_uint.clone());
        let cons_ok = em.and(mb[4].clone(), reader.is_uint.clone());
        let leaf_or_branch = em.or(reader.is_leaf.clone(), reader.is_branch.clone());
        let branch_ok = em.and(mb[5].clone(), leaf_or_branch);
        let to_second = { let o = em.or(num_ok.clone(), cons_ok); em.or(o, branch_ok.clone()) };
        let binary_modes = { let o = em.or(is_num, mb[4].clone()); em.or(o, mb[5].clone()) };
        let not_second = em.not(to_second.clone());
        let is_err = em.and(binary_modes, not_second);
        let post_phase = em.sel(to_second, Em::imm(1), Em::imm(2)); // Second else Done
        let post_reg_a = em.sel(num_ok, reader.uint_value.clone(), pre_h[rec + 14].clone());
        let selector_code = {
            let inner = em.sel(reader.bool_value.clone(), Em::imm(0), Em::imm(1));
            em.sel(reader.is_bool.clone(), inner, Em::imm(2))
        };
        let post_staged_tag = {
            let t = em.sel(is_err.clone(), Em::imm(4), pre_h[rec + 15].clone());
            let t = em.sel(mb[8].clone(), Em::imm(5), t);
            em.sel(mb[9].clone(), Em::imm(6), t)
        };
        let post_staged_payload = {
            let pl = em.sel(is_err, Em::imm(11), pre_h[rec + 16].clone());
            let pl = em.sel(mb[8].clone(), selector_code, pl);
            em.sel(mb[9].clone(), checked_sort_ok.clone(), pl)
        };

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
        // DATA unchanged (pre canonicality is the reader's composed block).
        for (a, b) in inputs.pre_data.iter().zip(inputs.post_data.iter()) {
            em.ceq(b.clone(), a.clone());
        }
        // HELPER post: only phase/regA/staged change, only when applied.
        for lane in 0..g.helper_total_lanes {
            let applied_expected = if lane == rec + 13 { post_phase.clone() }
                else if lane == rec + 14 { post_reg_a.clone() }
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
        let exported_nominal_valid = em.sel(en.clone(), nominal_valid, Em::imm(0));
        let exported_checked_sort_ok = em.sel(en, checked_sort_ok, Em::imm(0));
        self.next_temp = em.next;
        Ok(C6ScalarTagStepBlock {
            instructions: em.ins,
            original_ref_is_data: pre_h[rec + 6].clone(),
            original_ref_index: pre_h[rec + 7].clone(),
            nominal_valid: exported_nominal_valid,
            checked_sort_ok: exported_checked_sort_ok,
            is_helper_work_max: is_p3,
            post_phase, post_reg_a, post_staged_tag, post_staged_payload,
            reader,
        })
    }
}

/// Single-shot block at parent serial zero; see `C6ScalarTagStepEmitter`
/// for the composition contract.
pub fn emit_c6_scalar_tag_step_block(
    inputs: &C6ScalarTagStepInputs, scope: &str,
) -> Result<C6ScalarTagStepBlock, C6ScalarTagStepError> {
    C6ScalarTagStepEmitter::new(scope)?.emit(inputs)
}
