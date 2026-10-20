//! In-circuit canonicality for the padded `sherlock-c6-phys/1` HELPER section.
//!
//! Accepted domain is exactly the supplied Core `c6_phys_helper` codec domain
//! placed in the fixed padded carrier: header4, helper record 23 lanes at 4,
//! result record 4 lanes at 27, task records 7 lanes each at 31, inactive
//! tasks exactly zero. Tags/offsets are the explicit literal encode/decode
//! maps in Unit01, not enum declaration order. This is structural
//! representation canonicality/caps only, not StepInv, admission, membership,
//! or a proof/native claim.
//!
//! Gate policy: `gate` is boolean-constrained and every lane is
//! 128-bit-range-constrained unconditionally. Gate 1 enforces all below;
//! gate 0 enforces no header/record/task canonicality. Host shape refusal
//! (checked geometry, exact padded length, `usize::MAX` cap refusal) is
//! ungated and precedes emission. Slice ends and dynamic cursor bounds use
//! 129-bit `LessThan`: two 128-bit lanes sum to at most `2^129-2`, below the
//! BLS12-381 modulus, so field addition is exact integer addition.
//!
//! Namespace: intermediates are `%{scope}.c6helper.temp.{serial}` only.
//! That prefix is reserved; caller gate/helper operands using it, or a scope
//! containing `%` or `.c6helper.temp.`, are refused. One
//! `C6HelperCanonicalEmitter` retains its serial across `emit` calls under
//! one scope; the single-shot function starts at 0 and may be used once per
//! scope per circuit unless the caller proves disjointness. This prefix is
//! disjoint from the parent `%{scope}.local.temp.*` and DATA
//! `%{scope}.c6data.temp.*`.

use midnight_transient_crypto::curve::{Fr, FR_BITS};
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::{Identifier, Instruction};
use mori_core::sherlock::helper_machine::c6_phys::{C6_PHYS_MAGIC, C6_PHYS_VERSION};
use mori_core::sherlock::helper_machine::c6_phys_data::C6StateCaps;
use mori_core::sherlock::helper_machine::c6_phys_helper::{
    HELPER_FIXED_LANES, HELPER_SECTION_TAG, TASK_LANES,
};
use thiserror::Error;

const LANE_BITS: u32 = 128;
const END_BITS: u32 = 129;
const RESERVED_SEGMENT: &str = ".c6helper.temp.";

#[derive(Debug, Error, PartialEq, Eq)]
pub enum C6HelperCanonicalError {
    #[error("invalid scope identifier: must not be empty")]
    EmptyScope,
    #[error("invalid scope identifier: scope must not contain '%' or the reserved segment '.c6helper.temp.'")]
    InvalidScope,
    #[error("lane bit width {bits} is invalid for the BLS12-381 scalar field")]
    InvalidBitWidth { bits: u32 },
    #[error("geometry arithmetic overflow in HELPER section")]
    GeometryOverflow,
    #[error("padded HELPER length mismatch: expected {expected}, got {actual}")]
    LengthMismatch { expected: usize, actual: usize },
    #[error("operand uses reserved HELPER temporary identifier: {identifier}")]
    ReservedOperand { identifier: String },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct C6HelperGeometry {
    pub helper_base: usize,
    pub result_base: usize,
    pub tasks_base: usize,
    pub total: usize,
}

pub fn c6_helper_geometry(caps: &C6StateCaps) -> Result<C6HelperGeometry, C6HelperCanonicalError> {
    let ov = || C6HelperCanonicalError::GeometryOverflow;
    let total = caps.max_helper_tasks.checked_mul(TASK_LANES).and_then(|n| HELPER_FIXED_LANES.checked_add(n)).ok_or_else(ov)?;
    Ok(C6HelperGeometry { helper_base: 4, result_base: 27, tasks_base: HELPER_FIXED_LANES, total })
}

#[derive(Clone, Debug)]
pub struct C6HelperCanonicalInputs {
    pub gate: Operand,
    pub helper: Vec<Operand>,
    pub caps: C6StateCaps,
}

#[derive(Clone, Debug)]
pub struct C6HelperCanonicalBlock { pub instructions: Vec<Instruction> }

fn validate_scope(scope: &str) -> Result<(), C6HelperCanonicalError> {
    if scope.trim().is_empty() { return Err(C6HelperCanonicalError::EmptyScope); }
    if scope.contains('%') || scope.contains(RESERVED_SEGMENT) { return Err(C6HelperCanonicalError::InvalidScope); }
    Ok(())
}
fn reserved_prefix(scope: &str) -> String { format!("%{}.c6helper.temp.", scope) }
fn reject_reserved(scope: &str, op: &Operand) -> Result<(), C6HelperCanonicalError> {
    if let Operand::Variable(id) = op { if id.0.starts_with(&reserved_prefix(scope)) { return Err(C6HelperCanonicalError::ReservedOperand { identifier: id.0.clone() }); } }
    Ok(())
}

struct Emitter<'a> { scope: &'a str, next_temp: usize, instructions: Vec<Instruction> }
impl<'a> Emitter<'a> {
    fn new(scope: &'a str, n: usize) -> Self { Self { scope, next_temp: n, instructions: Vec::new() } }
    fn temp(&mut self) -> Identifier { let id = Identifier(format!("%{}.c6helper.temp.{}", self.scope, self.next_temp)); self.next_temp += 1; id }
    fn imm(v: u128) -> Operand { Operand::Immediate(Fr::from(v)) }
    fn cbool(&mut self, v: Operand) { self.instructions.push(Instruction::ConstrainToBoolean { val: v }); }
    fn cbits(&mut self, v: Operand, b: u32) { self.instructions.push(Instruction::ConstrainBits { val: v, bits: b }); }
    fn ceq(&mut self, a: Operand, b: Operand) { self.instructions.push(Instruction::ConstrainEq { a, b }); }
    fn teq(&mut self, a: Operand, b: Operand) -> Operand { let o = self.temp(); self.instructions.push(Instruction::TestEq { a, b, output: o.clone() }); Operand::Variable(o) }
    fn lt(&mut self, a: Operand, b: Operand, bits: u32) -> Operand { let o = self.temp(); self.instructions.push(Instruction::LessThan { a, b, bits, output: o.clone() }); Operand::Variable(o) }
    fn lt128(&mut self, a: Operand, b: Operand) -> Operand { self.lt(a, b, LANE_BITS) }
    fn lt129(&mut self, a: Operand, b: Operand) -> Operand { self.lt(a, b, END_BITS) }
    fn add(&mut self, a: Operand, b: Operand) -> Operand { let o = self.temp(); self.instructions.push(Instruction::Add { a, b, output: o.clone() }); Operand::Variable(o) }
    fn mul(&mut self, a: Operand, b: Operand) -> Operand { let o = self.temp(); self.instructions.push(Instruction::Mul { a, b, output: o.clone() }); Operand::Variable(o) }
    fn not(&mut self, a: Operand) -> Operand { let o = self.temp(); self.instructions.push(Instruction::Not { a, output: o.clone() }); Operand::Variable(o) }
    fn sel(&mut self, bit: Operand, a: Operand, b: Operand) -> Operand { let o = self.temp(); self.instructions.push(Instruction::CondSelect { bit, a, b, output: o.clone() }); Operand::Variable(o) }
    fn assert_true(&mut self, c: Operand) { self.instructions.push(Instruction::Assert { cond: c }); }
    fn geq(&mut self, g: Operand, a: Operand, b: Operand) { let s = self.sel(g, a, b.clone()); self.ceq(s, b); }
    fn gassert(&mut self, g: Operand, c: Operand) { let e = self.sel(g, c, Self::imm(1)); self.assert_true(e); }
    fn gzero(&mut self, g: Operand, v: Operand) { self.geq(g, v, Self::imm(0)); }
    fn gbool(&mut self, g: Operand, v: Operand) { let c = self.sel(g, v, Self::imm(0)); self.cbool(c); }
    /// Strict bound `v < cap`.
    fn glt_index(&mut self, g: Operand, v: Operand, cap: usize) { let lt = self.lt128(v, Self::imm(cap as u128)); self.gassert(g, lt); }
    /// Inclusive bound `v <= cap`, i.e. `v < cap+1`; host refused cap==MAX.
    fn glt_count(&mut self, g: Operand, v: Operand, cap: usize) { let lt = self.lt128(v, Self::imm((cap as u128) + 1)); self.gassert(g, lt); }
    fn glt_end(&mut self, g: Operand, end: Operand, cap: usize) { let lt = self.lt129(end, Self::imm((cap as u128) + 1)); self.gassert(g, lt); }
    fn tag_bits(&mut self, g: &Operand, tag_lane: Operand, count: u128) -> Vec<Operand> {
        let mut bits = Vec::new(); let mut sum = Self::imm(0);
        for t in 0..count { let b = self.teq(tag_lane.clone(), Self::imm(t)); sum = self.add(sum, b.clone()); bits.push(b); }
        self.geq(g.clone(), sum, Self::imm(1)); bits
    }
    fn gref(&mut self, g: Operand, kind: Operand, idx: Operand, caps: &C6StateCaps) {
        self.gbool(g.clone(), kind.clone());
        let is_data = self.teq(kind, Self::imm(1));
        let cap = self.sel(is_data, Self::imm(caps.phys.max_data_nodes as u128), Self::imm(caps.phys.max_constants as u128));
        let lt = self.lt128(idx, cap); self.gassert(g, lt);
    }
    fn gopt_index(&mut self, g: Operand, present: Operand, idx: Operand, cap: Option<usize>) {
        self.gbool(g.clone(), present.clone());
        let absent = self.teq(present.clone(), Self::imm(0));
        let ag = self.mul(g.clone(), absent); self.geq(ag, idx.clone(), Self::imm(0));
        if let Some(cap) = cap { let present_b = self.teq(present, Self::imm(1)); let pg = self.mul(g, present_b); self.glt_index(pg, idx, cap); }
    }
    fn gopt_ref(&mut self, g: Operand, lanes: &[Operand], at: usize, caps: &C6StateCaps) {
        self.gbool(g.clone(), lanes[at].clone());
        let absent = self.teq(lanes[at].clone(), Self::imm(0));
        let ag = self.mul(g.clone(), absent);
        self.gzero(ag.clone(), lanes[at + 1].clone()); self.gzero(ag, lanes[at + 2].clone());
        let present_b = self.teq(lanes[at].clone(), Self::imm(1));
        let pg = self.mul(g, present_b); self.gref(pg, lanes[at + 1].clone(), lanes[at + 2].clone(), caps);
    }
    fn gslice(&mut self, g: Operand, start: Operand, len: Operand, table: usize, max_len: Option<usize>) {
        if let Some(m) = max_len { self.glt_count(g.clone(), len.clone(), m); }
        let end = self.add(start, len); self.glt_end(g, end, table);
    }
    fn gcursor_le_len(&mut self, g: Operand, cursor: Operand, len: Operand) {
        let lp1 = self.add(len, Self::imm(1)); let lt = self.lt129(cursor, lp1); self.gassert(g, lt);
    }
    fn gstaged(&mut self, g: Operand, tag: Operand, payload: Operand) {
        let bits = self.tag_bits(&g.clone(), tag, 7);
        let vg = |em: &mut Self, i: usize| em.mul(g.clone(), bits[i].clone());
        let v0 = vg(self, 0); self.gzero(v0, payload.clone());
        // 1 U128Value: payload unrestricted.
        let v2 = vg(self, 2); self.gbool(v2, payload.clone());
        let v3 = vg(self, 3); self.glt_index(v3, payload.clone(), 3);
        let v4 = vg(self, 4); self.glt_index(v4, payload.clone(), 256);
        let v5 = vg(self, 5); self.glt_index(v5, payload.clone(), 3);
        let v6 = vg(self, 6); self.gbool(v6, payload);
    }
    fn slot_gates(&mut self, gate: &Operand, count: &Operand, slot: usize) -> (Operand, Operand) {
        let active = self.lt128(Self::imm(slot as u128), count.clone());
        let inactive = self.not(active.clone());
        (self.mul(gate.clone(), active), self.mul(gate.clone(), inactive))
    }
}

pub struct C6HelperCanonicalEmitter { scope: String, next_temp: usize }
impl C6HelperCanonicalEmitter {
    pub fn new(scope: &str) -> Result<Self, C6HelperCanonicalError> { validate_scope(scope)?; Ok(Self { scope: scope.to_string(), next_temp: 0 }) }
    pub fn scope(&self) -> &str { &self.scope }
    pub fn next_temp(&self) -> usize { self.next_temp }
    pub fn emit(&mut self, inputs: &C6HelperCanonicalInputs) -> Result<C6HelperCanonicalBlock, C6HelperCanonicalError> {
        if (LANE_BITS as usize) >= FR_BITS || (END_BITS as usize) >= FR_BITS { return Err(C6HelperCanonicalError::InvalidBitWidth { bits: END_BITS }); }
        reject_reserved(&self.scope, &inputs.gate)?;
        for l in &inputs.helper { reject_reserved(&self.scope, l)?; }
        let caps = &inputs.caps; let p = &caps.phys;
        for cap in [p.max_frames, p.max_expressions, p.max_names, p.max_arms, p.max_children, p.max_value_slots, p.max_constants, p.max_data_nodes, p.max_env_cells, p.max_functions, p.max_arity, p.max_types, p.max_occurrences, p.max_occurrence_paths, p.max_parameters, p.max_patterns, p.max_pattern_children, p.max_constructors, p.max_ctor_fields, p.max_plan_nodes, p.max_field_paths, caps.max_data_field_slots, caps.max_helper_tasks] { if cap == usize::MAX { return Err(C6HelperCanonicalError::GeometryOverflow); } }
        let geom = c6_helper_geometry(caps)?;
        if inputs.helper.len() != geom.total { return Err(C6HelperCanonicalError::LengthMismatch { expected: geom.total, actual: inputs.helper.len() }); }
        let builtin = p.max_types.max(4);
        let mut em = Emitter::new(&self.scope, self.next_temp);
        let gate = inputs.gate.clone(); let h = &inputs.helper;
        em.cbool(gate.clone());
        for l in h.iter() { em.cbits(l.clone(), LANE_BITS); }
        // Header4.
        em.geq(gate.clone(), h[0].clone(), Emitter::imm(C6_PHYS_MAGIC));
        em.geq(gate.clone(), h[1].clone(), Emitter::imm(C6_PHYS_VERSION));
        em.geq(gate.clone(), h[2].clone(), Emitter::imm(HELPER_SECTION_TAG));
        em.glt_count(gate.clone(), h[3].clone(), caps.max_helper_tasks);
        // ---- helper record, lanes 4..27 ----
        let r = &h[4..27];
        let bits = em.tag_bits(&gate, r[0].clone(), 12);
        let none_gate = em.mul(gate.clone(), bits[0].clone());
        for l in r.iter().skip(1) { em.gzero(none_gate.clone(), l.clone()); }
        let present = em.not(bits[0].clone());
        let pg_common = em.mul(gate.clone(), present);
        // Common: site 0..=26 at 1, origin at 2 unrestricted, parent_resume opt at 3..5 no cap.
        em.glt_index(pg_common.clone(), r[1].clone(), 27);
        em.gopt_index(pg_common.clone(), r[3].clone(), r[4].clone(), None);
        let vg = |em: &mut Emitter<'_>, i: usize| em.mul(gate.clone(), bits[i].clone());
        // 1 Scalar: mode5, refA6-7, optRefB8-10, opt expected11-12 builtin, phase13, regA14 free, staged15-16; used17.
        { let g = vg(&mut em, 1); em.glt_index(g.clone(), r[5].clone(), 10); em.gref(g.clone(), r[6].clone(), r[7].clone(), caps); em.gopt_ref(g.clone(), r, 8, caps); em.gopt_index(g.clone(), r[11].clone(), r[12].clone(), Some(builtin)); em.glt_index(g.clone(), r[13].clone(), 3); em.gstaged(g.clone(), r[15].clone(), r[16].clone()); for i in 17..23 { em.gzero(g.clone(), r[i].clone()); } }
        // 2 EnvGet: binder5, cursor opt6-7 env, found optRef8-10, status11; used12.
        { let g = vg(&mut em, 2); em.glt_index(g.clone(), r[5].clone(), p.max_names); em.gopt_index(g.clone(), r[6].clone(), r[7].clone(), Some(p.max_env_cells)); em.gopt_ref(g.clone(), r, 8, caps); em.glt_index(g.clone(), r[11].clone(), 3); for i in 12..23 { em.gzero(g.clone(), r[i].clone()); } }
        // 3 EnvPut: binder5, value6-7, base8-9 env, new_root10-11 env-index, status12; used13.
        { let g = vg(&mut em, 3); em.glt_index(g.clone(), r[5].clone(), p.max_names); em.gref(g.clone(), r[6].clone(), r[7].clone(), caps); em.gopt_index(g.clone(), r[8].clone(), r[9].clone(), Some(p.max_env_cells)); em.gopt_index(g.clone(), r[10].clone(), r[11].clone(), Some(p.max_env_cells)); em.glt_index(g.clone(), r[12].clone(), 3); for i in 13..23 { em.gzero(g.clone(), r[i].clone()); } }
        // 4 Find: name5, row_cursor6 count<=functions, found opt7-8 functions, status9; used10.
        { let g = vg(&mut em, 4); em.glt_index(g.clone(), r[5].clone(), p.max_names); em.glt_count(g.clone(), r[6].clone(), p.max_functions); em.gopt_index(g.clone(), r[7].clone(), r[8].clone(), Some(p.max_functions)); em.glt_index(g.clone(), r[9].clone(), 3); for i in 10..23 { em.gzero(g.clone(), r[i].clone()); } }
        // 5 Bind: fn5, params6-7, args8-9, pcur10, acur11, partial12-13 env, dup14-15 opt env, phase16, status17; used18.
        { let g = vg(&mut em, 5); em.glt_index(g.clone(), r[5].clone(), p.max_functions); em.gslice(g.clone(), r[6].clone(), r[7].clone(), p.max_parameters, Some(p.max_arity)); em.gslice(g.clone(), r[8].clone(), r[9].clone(), p.max_value_slots, Some(p.max_arity)); em.gcursor_le_len(g.clone(), r[10].clone(), r[7].clone()); em.gcursor_le_len(g.clone(), r[11].clone(), r[9].clone()); em.gopt_index(g.clone(), r[12].clone(), r[13].clone(), Some(p.max_env_cells)); em.gopt_index(g.clone(), r[14].clone(), r[15].clone(), Some(p.max_env_cells)); em.glt_index(g.clone(), r[16].clone(), 6); em.glt_index(g.clone(), r[17].clone(), 2); for i in 18..23 { em.gzero(g.clone(), r[i].clone()); } }
        // 6 Append: src5-6, value7-8, src_cursor9, dest_start10 count<=value_slots, staged11-13, phase14; used15.
        { let g = vg(&mut em, 6); em.gslice(g.clone(), r[5].clone(), r[6].clone(), p.max_value_slots, Some(p.max_arity)); em.gref(g.clone(), r[7].clone(), r[8].clone(), caps); em.gcursor_le_len(g.clone(), r[9].clone(), r[6].clone()); em.glt_count(g.clone(), r[10].clone(), p.max_value_slots); em.gopt_ref(g.clone(), r, 11, caps); em.glt_index(g.clone(), r[14].clone(), 3); for i in 15..23 { em.gzero(g.clone(), r[i].clone()); } }
        // 7 Validate: root5-6, status7; used8.
        { let g = vg(&mut em, 7); em.gref(g.clone(), r[5].clone(), r[6].clone(), caps); em.glt_index(g.clone(), r[7].clone(), 3); for i in 8..23 { em.gzero(g.clone(), r[i].clone()); } }
        // 8 Equal: left5-6, right7-8, status9; used10.
        { let g = vg(&mut em, 8); em.gref(g.clone(), r[5].clone(), r[6].clone(), caps); em.gref(g.clone(), r[7].clone(), r[8].clone(), caps); em.glt_index(g.clone(), r[9].clone(), 3); for i in 10..23 { em.gzero(g.clone(), r[i].clone()); } }
        // 9 Match: scrut5-6, arm7, saved8-9, partial10-11, pat12, x13, y14, field_idx15 u8, staged16-18, phase19, status20; used21.
        { let g = vg(&mut em, 9); em.gref(g.clone(), r[5].clone(), r[6].clone(), caps); em.glt_index(g.clone(), r[7].clone(), p.max_arms); em.gopt_index(g.clone(), r[8].clone(), r[9].clone(), Some(p.max_env_cells)); em.gopt_index(g.clone(), r[10].clone(), r[11].clone(), Some(p.max_env_cells)); em.glt_index(g.clone(), r[12].clone(), 7); em.glt_index(g.clone(), r[15].clone(), 256);
          // Field-dependent x/y: only when phase != Arm(0). PVar/PLeaf bind x; PCons/PBranch bind x,y; others none. Raw placeholders otherwise.
          let phase_arm = em.teq(r[19].clone(), Emitter::imm(0)); let not_arm = em.not(phase_arm);
          let pat = em.tag_bits(&g.clone(), r[12].clone(), 7);
          // tag_bits already gated exactly-one under g; reuse its bits for binding gates without new sum (bits returned).
          let x_sum = em.add(pat[0].clone(), pat[2].clone()); let x_sum = em.add(x_sum, pat[3].clone()); let x_sum = em.add(x_sum, pat[4].clone());
          let xg0 = em.mul(g.clone(), not_arm.clone()); let xg = em.mul(xg0, x_sum); em.glt_index(xg, r[13].clone(), p.max_names);
          let y_sum = em.add(pat[2].clone(), pat[4].clone()); let yg0 = em.mul(g.clone(), not_arm); let yg = em.mul(yg0, y_sum); em.glt_index(yg, r[14].clone(), p.max_names);
          em.gopt_ref(g.clone(), r, 16, caps); em.glt_index(g.clone(), r[19].clone(), 8); em.glt_index(g.clone(), r[20].clone(), 3); for i in 21..23 { em.gzero(g.clone(), r[i].clone()); } }
        // 10 Plan: source5, scrut6-7, saved8-9, partial10-11, node12, phase13, after14, cur15-16, path17-18, pathcur19, arm opt20-21, status22; used23.
        { let g = vg(&mut em, 10); em.glt_index(g.clone(), r[5].clone(), p.max_expressions); em.gref(g.clone(), r[6].clone(), r[7].clone(), caps); em.gopt_index(g.clone(), r[8].clone(), r[9].clone(), Some(p.max_env_cells)); em.gopt_index(g.clone(), r[10].clone(), r[11].clone(), Some(p.max_env_cells)); em.glt_index(g.clone(), r[12].clone(), p.max_plan_nodes); em.glt_index(g.clone(), r[13].clone(), 6); em.glt_index(g.clone(), r[14].clone(), 2); em.gref(g.clone(), r[15].clone(), r[16].clone(), caps); em.gslice(g.clone(), r[17].clone(), r[18].clone(), p.max_field_paths, None); em.gcursor_le_len(g.clone(), r[19].clone(), r[18].clone()); em.gopt_index(g.clone(), r[20].clone(), r[21].clone(), Some(p.max_arms)); em.glt_index(g.clone(), r[22].clone(), 3); }
        // 11 Build: type5 plain max_types, ctor6, src7-8, src_cursor9, field_start10 count<=data_field_slots, staged11-13, node14-16, phase17; used18.
        { let g = vg(&mut em, 11); em.glt_index(g.clone(), r[5].clone(), p.max_types); em.glt_index(g.clone(), r[6].clone(), p.max_constructors); em.gslice(g.clone(), r[7].clone(), r[8].clone(), p.max_value_slots, Some(p.max_arity)); em.gcursor_le_len(g.clone(), r[9].clone(), r[8].clone()); em.glt_count(g.clone(), r[10].clone(), caps.max_data_field_slots); em.gopt_ref(g.clone(), r, 11, caps); em.gopt_ref(g.clone(), r, 14, caps); em.glt_index(g.clone(), r[17].clone(), 2); for i in 18..23 { em.gzero(g.clone(), r[i].clone()); } }
        // ---- result record, lanes 27..31 ----
        let s = &h[27..31];
        let rb = em.tag_bits(&gate, s[0].clone(), 20);
        let rg = |em: &mut Emitter<'_>, i: usize| em.mul(gate.clone(), rb[i].clone());
        { let g = rg(&mut em, 0); for i in 1..4 { em.gzero(g.clone(), s[i].clone()); } }
        { let g = rg(&mut em, 1); em.gbool(g.clone(), s[1].clone()); em.gzero(g.clone(), s[2].clone()); em.gzero(g, s[3].clone()); }
        { let g = rg(&mut em, 2); em.gref(g.clone(), s[1].clone(), s[2].clone(), caps); em.gzero(g, s[3].clone()); }
        { let g = rg(&mut em, 3); for i in 1..4 { em.gzero(g.clone(), s[i].clone()); } }
        { let g = rg(&mut em, 4); em.glt_index(g.clone(), s[1].clone(), p.max_env_cells); em.gzero(g.clone(), s[2].clone()); em.gzero(g, s[3].clone()); }
        { let g = rg(&mut em, 5); em.glt_index(g.clone(), s[1].clone(), p.max_functions); em.gzero(g.clone(), s[2].clone()); em.gzero(g, s[3].clone()); }
        { let g = rg(&mut em, 6); for i in 1..4 { em.gzero(g.clone(), s[i].clone()); } }
        { let g = rg(&mut em, 7); em.gopt_index(g.clone(), s[1].clone(), s[2].clone(), Some(p.max_env_cells)); em.gzero(g, s[3].clone()); }
        { let g = rg(&mut em, 8); for i in 1..4 { em.gzero(g.clone(), s[i].clone()); } }
        { let g = rg(&mut em, 9); em.gslice(g, s[1].clone(), s[2].clone(), p.max_value_slots, Some(p.max_arity)); let g2 = rg(&mut em, 9); em.gzero(g2, s[3].clone()); }
        // Correct AppendDone padding under its own gate (recomputed above is same gate product).
        { let g = rg(&mut em, 10); em.gref(g.clone(), s[1].clone(), s[2].clone(), caps); em.gzero(g, s[3].clone()); }
        { let g = rg(&mut em, 11); em.glt_index(g.clone(), s[1].clone(), 256); em.gzero(g.clone(), s[2].clone()); em.gzero(g, s[3].clone()); }
        { let g = rg(&mut em, 12); em.glt_index(g.clone(), s[1].clone(), 3); em.gzero(g.clone(), s[2].clone()); em.gzero(g, s[3].clone()); }
        { let g = rg(&mut em, 13); em.gbool(g.clone(), s[1].clone()); em.gzero(g.clone(), s[2].clone()); em.gzero(g, s[3].clone()); }
        { let g = rg(&mut em, 14); em.gref(g.clone(), s[1].clone(), s[2].clone(), caps); em.gzero(g, s[3].clone()); }
        { let g = rg(&mut em, 15); em.gopt_index(g.clone(), s[1].clone(), s[2].clone(), Some(p.max_env_cells)); em.gzero(g, s[3].clone()); }
        { let g = rg(&mut em, 16); for i in 1..4 { em.gzero(g.clone(), s[i].clone()); } }
        { let g = rg(&mut em, 17); em.glt_index(g.clone(), s[1].clone(), p.max_arms); em.gopt_index(g, s[2].clone(), s[3].clone(), Some(p.max_env_cells)); }
        { let g = rg(&mut em, 18); for i in 1..4 { em.gzero(g.clone(), s[i].clone()); } }
        { let g = rg(&mut em, 19); em.gref(g.clone(), s[1].clone(), s[2].clone(), caps); em.gzero(g, s[3].clone()); }
        // ---- tasks ----
        for slot in 0..caps.max_helper_tasks {
            let base = geom.tasks_base + slot * TASK_LANES;
            let t = &h[base..base + TASK_LANES];
            let (row_gate, inactive_gate) = em.slot_gates(&gate, &h[3], slot);
            for l in t.iter() { em.gzero(inactive_gate.clone(), l.clone()); }
            let tb = em.tag_bits(&row_gate, t[0].clone(), 3);
            let tg = |em: &mut Emitter<'_>, i: usize| em.mul(row_gate.clone(), tb[i].clone());
            { let g = tg(&mut em, 0); em.gref(g.clone(), t[1].clone(), t[2].clone(), caps); em.glt_index(g.clone(), t[3].clone(), 2); em.glt_count(g.clone(), t[4].clone(), p.max_arity); em.gzero(g.clone(), t[5].clone()); em.gzero(g, t[6].clone()); }
            { let g = tg(&mut em, 1); em.gref(g.clone(), t[1].clone(), t[2].clone(), caps); em.gref(g.clone(), t[3].clone(), t[4].clone(), caps); em.glt_index(g.clone(), t[5].clone(), 2); em.glt_count(g, t[6].clone(), p.max_arity); }
            { let g = tg(&mut em, 2); em.glt_index(g.clone(), t[1].clone(), p.max_names); em.glt_index(g.clone(), t[2].clone(), builtin); em.gref(g.clone(), t[3].clone(), t[4].clone(), caps); em.gzero(g.clone(), t[5].clone()); em.gzero(g, t[6].clone()); }
        }
        self.next_temp = em.next_temp;
        Ok(C6HelperCanonicalBlock { instructions: em.instructions })
    }
}

pub fn emit_c6_helper_canonical_block(inputs: &C6HelperCanonicalInputs, scope: &str) -> Result<C6HelperCanonicalBlock, C6HelperCanonicalError> {
    let mut emitter = C6HelperCanonicalEmitter::new(scope)?;
    emitter.emit(inputs)
}
