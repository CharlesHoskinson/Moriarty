//! In-circuit canonicality for PROG (`decode_program`) and CTX
//! (`decode_context`), pre-state only. Gate is boolean; every lane is
//! 128-bit ranged unconditionally. Gate 1 is the exact codec domain.
//! Gate 0 imposes no canonicality beyond those ranges. `Reader::slice`
//! reads only two `usize` lanes: no end/table/arity/admission check is
//! added for program slices. Standalone use makes no post claim;
//! composed Push must retain unconditional post ranges and post==pre.
use midnight_transient_crypto::curve::{FR_BITS, Fr};
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::{Identifier, Instruction};
use mori_core::sherlock::helper_machine::c6_phys::{C6_PHYS_MAGIC, C6_PHYS_VERSION, C6PhysCaps};
use mori_core::sherlock::helper_machine::c6_phys_data::C6StateCaps;
use mori_core::sherlock::helper_machine::c6_phys_host::{CTX_LANES, CTX_SECTION_TAG};
use mori_core::sherlock::helper_machine::c6_phys_program::{
    PROGRAM_HEADER_LANES, PROGRAM_SECTION_TAG,
};
use thiserror::Error;
const LANE_BITS: u32 = 128;
const RESERVED_SEGMENT: &str = ".c6progctx.temp.";
const CP_CAP: u128 = 0x20_0000;
const PH_MAGIC: u128 = 0;
const PH_VERSION: u128 = 1;
const PH_TAG: u128 = 2;
const PH_ENTRY: u128 = 3;
const PH_EXPR_COUNT: u128 = 4;
const PH_EXPR_ROW: u128 = 5;
const PH_OCC_COUNT: u128 = 6;
const PH_OCC_ROW: u128 = 7;
const PH_OCCPATH_COUNT: u128 = 8;
const PH_OCCPATH_ROW: u128 = 9;
const PH_CHILD_COUNT: u128 = 10;
const PH_CHILD_ROW: u128 = 11;
const PH_FUNC_COUNT: u128 = 12;
const PH_FUNC_ROW: u128 = 13;
const PH_PARAM_COUNT: u128 = 14;
const PH_PARAM_ROW: u128 = 15;
const PH_ARM_COUNT: u128 = 16;
const PH_ARM_ROW: u128 = 17;
const PH_PAT_COUNT: u128 = 18;
const PH_PAT_ROW: u128 = 19;
const PH_PATCHILD_COUNT: u128 = 20;
const PH_PATCHILD_ROW: u128 = 21;
const PH_TYPE_COUNT: u128 = 22;
const PH_TYPE_ROW: u128 = 23;
const PH_TYPEARG_COUNT: u128 = 24;
const PH_TYPEARG_ROW: u128 = 25;
const PH_CTOR_COUNT: u128 = 26;
const PH_CTOR_ROW: u128 = 27;
const PH_CTORFIELD_COUNT: u128 = 28;
const PH_CTORFIELD_ROW: u128 = 29;
const PH_DECPARAMCNT_COUNT: u128 = 30;
const PH_DECPARAMCNT_ROW: u128 = 31;
const PH_DECCTORCNT_COUNT: u128 = 32;
const PH_DECCTORCNT_ROW: u128 = 33;
const PH_DECL_COUNT: u128 = 34;
const PH_DECL_NAME: u128 = 35;
const PH_DECL_PARAMCNT: u128 = 36;
const PH_DECL_PARAM: u128 = 37;
const PH_DECL_CTORCNT: u128 = 38;
const PH_DECL_CTORNAME: u128 = 39;
const PH_DECL_FIELDCNT: u128 = 40;
const PH_DECL_FIELDNAME: u128 = 41;
const PH_TYPEEXPR: u128 = 42;
const PH_PLAN_COUNT: u128 = 43;
const PH_PLAN_ROW: u128 = 44;
const PH_PLANCASE_COUNT: u128 = 45;
const PH_PLANCASE_ROW: u128 = 46;
const PH_FIELDPATH_COUNT: u128 = 47;
const PH_FIELDPATH_ROW: u128 = 48;
const PH_CONST_COUNT: u128 = 49;
const PH_CONST_ROW: u128 = 50;
const PH_CONSTFIELD_COUNT: u128 = 51;
const PH_CONSTFIELD_ROW: u128 = 52;
const PH_NAMES_COUNT: u128 = 53;
const PH_NAMES_STR: u128 = 54;
const PH_END: u128 = 55;
#[derive(Debug, Error, PartialEq, Eq)]
pub enum C6ProgramContextCanonicalError {
    #[error("invalid scope identifier: must not be empty")]
    EmptyScope,
    #[error(
        "invalid scope identifier: scope must not contain '%' or the reserved segment '.c6progctx.temp.'"
    )]
    InvalidScope,
    #[error("lane bit width {bits} is invalid for the BLS12-381 scalar field")]
    InvalidBitWidth { bits: u32 },
    #[error("geometry arithmetic overflow in PROG/CTX section")]
    GeometryOverflow,
    #[error("PROG length mismatch: expected at least {expected}, got {actual}")]
    LengthMismatch { expected: usize, actual: usize },
    #[error("operand uses reserved PROG/CTX temporary identifier: {identifier}")]
    ReservedOperand { identifier: String },
}
#[derive(Clone, Debug)]
pub struct C6ProgramContextCanonicalInputs {
    pub gate: Operand,
    pub pre_prog: Vec<Operand>,
    pub pre_ctx: [Operand; CTX_LANES],
    pub caps: C6StateCaps,
}
#[derive(Clone, Debug)]
pub struct C6ProgramContextCanonicalBlock {
    pub instructions: Vec<Instruction>,
}

/// Inputs for one extra gated Prog walk whose only added outputs are the
/// parsed constants count and the six lanes of the constant row whose
/// index equals `selected_index`. Row lanes are codec lanes:
/// `[tag, marker_free, lane2, lane3, lane4, lane5]`, variable used prefix
/// by tag exactly as the parser's `PH_CONST_ROW` width table
/// `[3,3,2,4,4,4,6,2]` encodes. Unused tail lanes stay zeroed by capture
/// initialisation, never by synthesised payload.
#[derive(Clone, Debug)]
pub struct C6ProgramContextConstCaptureInputs {
    pub gate: Operand,
    pub pre_prog: Vec<Operand>,
    pub caps: C6StateCaps,
    pub selected_index: Operand,
}

#[derive(Clone, Debug)]
pub struct C6ProgramContextConstCaptureBlock {
    pub instructions: Vec<Instruction>,
    pub const_count: Operand,
    pub selected_found: Operand,
    pub row: [Operand; 6],
}

/// Inputs for one extra gated Prog walk that additionally captures the
/// type row whose index equals `selected_type_id` (ctor_start/ctor_len),
/// the constructor row whose index equals `selected_ctor_id`
/// (owning_type/field_start/field_len), the parsed type count, and the
/// parsed constructor-field count. This is the constrained table read
/// behind Core `adt_checked_ctor_arity`; validity itself is computed by
/// the consumer from these captured lanes, never by a host flag.
#[derive(Clone, Debug)]
pub struct C6ProgramContextCheckedSortCaptureInputs {
    pub gate: Operand,
    pub pre_prog: Vec<Operand>,
    pub caps: C6StateCaps,
    pub selected_type_id: Operand,
    pub selected_ctor_id: Operand,
}

#[derive(Clone, Debug)]
pub struct C6ProgramContextCheckedSortCaptureBlock {
    pub instructions: Vec<Instruction>,
    pub type_count: Operand,
    pub type_found: Operand,
    pub ctor_found: Operand,
    pub owning_type: Operand,
    pub ctor_start: Operand,
    pub ctor_len: Operand,
    pub field_start: Operand,
    pub field_len: Operand,
    pub ctor_field_count: Operand,
}

/// Actual selected expression row, captured while the complete canonical
/// PROG parser walks its variable-width expression table. Construct codec
/// lanes are [8, type_id, ctor_id, fields.start, fields.len, Some, cid].
/// Other tags retain their exact used prefix and a zero unused tail.
#[derive(Clone, Debug)]
pub struct C6ProgramContextExpressionCaptureInputs {
    pub gate: Operand,
    pub pre_prog: Vec<Operand>,
    pub caps: C6StateCaps,
    pub selected_index: Operand,
}
#[derive(Clone, Debug)]
pub struct C6ProgramContextExpressionCaptureBlock {
    pub instructions: Vec<Instruction>,
    pub expressions_count: Operand,
    pub selected_found: Operand,
    pub row: [Operand; 7],
}

#[derive(Clone, Debug)]
pub struct C6ProgramContextCtxBindingInputs {
    pub gate: Operand,
    pub pre_ctx: [Operand; CTX_LANES],
    pub expected_ctx: [u128; CTX_LANES],
}
fn validate_scope(scope: &str) -> Result<(), C6ProgramContextCanonicalError> {
    if scope.trim().is_empty() {
        return Err(C6ProgramContextCanonicalError::EmptyScope);
    }
    if scope.contains('%') || scope.contains(RESERVED_SEGMENT) {
        return Err(C6ProgramContextCanonicalError::InvalidScope);
    }
    Ok(())
}
fn reserved_prefix(scope: &str) -> String {
    format!("%{}.c6progctx.temp.", scope)
}
fn reject_reserved(scope: &str, operand: &Operand) -> Result<(), C6ProgramContextCanonicalError> {
    if let Operand::Variable(id) = operand {
        if id.0.starts_with(&reserved_prefix(scope)) {
            return Err(C6ProgramContextCanonicalError::ReservedOperand {
                identifier: id.0.clone(),
            });
        }
    }
    Ok(())
}
#[derive(Clone)]
struct Regs {
    phase: Operand,
    sub: Operand,
    hold: Operand,
    opt: Operand,
    rem: Operand,
    cnt: Operand,
    cnt2: Operand,
    cnt3: Operand,
    te_need: Operand,
    srem: Operand,
    utf_pending: Operand,
    utf_cp: Operand,
    utf_min: Operand,
    cum_name: Operand,
    cum_tmpl: Operand,
    cum_nodes: Operand,
    const_count: Operand,
    const_row_index: Operand,
    const_capture_active: Operand,
    const_found: Operand,
    const_cap_0: Operand,
    const_cap_1: Operand,
    const_cap_2: Operand,
    const_cap_3: Operand,
    const_cap_4: Operand,
    const_cap_5: Operand,
    types_count: Operand,
    type_row_index: Operand,
    type_capture_active: Operand,
    type_found: Operand,
    type_ctor_start: Operand,
    type_ctor_len: Operand,
    ctor_row_index: Operand,
    ctor_capture_active: Operand,
    ctor_found: Operand,
    ctor_owning_type: Operand,
    ctor_field_start: Operand,
    ctor_field_len: Operand,
    ctor_fields_count: Operand,
}
struct Emitter<'a> {
    scope: &'a str,
    next_temp: usize,
    instructions: Vec<Instruction>,
}
impl<'a> Emitter<'a> {
    fn new(scope: &'a str, next_temp: usize) -> Self {
        Self {
            scope,
            next_temp,
            instructions: Vec::new(),
        }
    }
    fn temp(&mut self) -> Identifier {
        let id = Identifier(format!("%{}.c6progctx.temp.{}", self.scope, self.next_temp));
        self.next_temp += 1;
        id
    }
    fn imm(v: u128) -> Operand {
        Operand::Immediate(Fr::from(v))
    }
    fn cbool(&mut self, v: Operand) {
        self.instructions
            .push(Instruction::ConstrainToBoolean { val: v });
    }
    fn cbits(&mut self, v: Operand, b: u32) {
        self.instructions
            .push(Instruction::ConstrainBits { val: v, bits: b });
    }
    fn ceq(&mut self, a: Operand, b: Operand) {
        self.instructions.push(Instruction::ConstrainEq { a, b });
    }
    fn assert_true(&mut self, c: Operand) {
        self.instructions.push(Instruction::Assert { cond: c });
    }
    fn teq(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.temp();
        self.instructions.push(Instruction::TestEq {
            a,
            b,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn eqv(&mut self, a: Operand, v: u128) -> Operand {
        self.teq(a, Self::imm(v))
    }
    fn lt128(&mut self, a: Operand, v: u128) -> Operand {
        let o = self.temp();
        self.instructions.push(Instruction::LessThan {
            a,
            b: Self::imm(v),
            bits: LANE_BITS,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn lt_op(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.temp();
        self.instructions.push(Instruction::LessThan {
            a,
            b,
            bits: LANE_BITS,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn add(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.temp();
        self.instructions.push(Instruction::Add {
            a,
            b,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn add_i(&mut self, a: Operand, d: i128) -> Operand {
        self.add(a, Operand::Immediate(Fr::from(d)))
    }
    fn mul(&mut self, a: Operand, b: Operand) -> Operand {
        let o = self.temp();
        self.instructions.push(Instruction::Mul {
            a,
            b,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn not(&mut self, a: Operand) -> Operand {
        let o = self.temp();
        self.instructions.push(Instruction::Not {
            a,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn sel(&mut self, bit: Operand, a: Operand, b: Operand) -> Operand {
        let o = self.temp();
        self.instructions.push(Instruction::CondSelect {
            bit,
            a,
            b,
            output: o.clone(),
        });
        Operand::Variable(o)
    }
    fn and(&mut self, a: Operand, b: Operand) -> Operand {
        self.mul(a, b)
    }
    fn or(&mut self, a: Operand, b: Operand) -> Operand {
        let na = self.not(a);
        let nb = self.not(b);
        let both = self.mul(na, nb);
        self.not(both)
    }
    fn or_many(&mut self, mut xs: Vec<Operand>) -> Operand {
        let mut acc = xs.remove(0);
        for x in xs {
            acc = self.or(acc, x);
        }
        acc
    }
    fn in_range(&mut self, a: Operand, lo: u128, hi: u128) -> Operand {
        let below = self.lt128(a.clone(), lo);
        let ge = self.not(below);
        let lt = self.lt128(a, hi);
        self.and(ge, lt)
    }
    fn gated_assert(&mut self, gate: Operand, cond: Operand) {
        let e = self.sel(gate, cond, Self::imm(1));
        self.assert_true(e);
    }
    fn gated_eq(&mut self, gate: Operand, a: Operand, b: Operand) {
        let s = self.sel(gate, a, b.clone());
        self.ceq(s, b);
    }
    fn gated_zero(&mut self, gate: Operand, v: Operand) {
        self.gated_eq(gate, v, Self::imm(0));
    }
    fn gated_bool(&mut self, gate: Operand, v: Operand) {
        let c = self.sel(gate, v, Self::imm(0));
        self.cbool(c);
    }
    fn check_lt(&mut self, active: Operand, lane: Operand, bound: u128) {
        let ok = self.lt128(lane, bound);
        self.gated_assert(active, ok);
    }
    fn check_usize(&mut self, active: Operand, lane: Operand) {
        self.check_lt(active, lane, (usize::MAX as u128) + 1);
    }
    fn check_count(&mut self, active: Operand, lane: Operand, cap: usize) {
        self.check_lt(active, lane, (cap as u128) + 1);
    }
    fn cum_next_bounded(
        &mut self,
        cum: Operand,
        lane: Operand,
        limit_plus_one: u128,
    ) -> (Operand, Operand) {
        let within = self.lt128(lane.clone(), limit_plus_one);
        let contrib = self.sel(within, lane, Self::imm(limit_plus_one));
        let raw = self.add(cum, contrib);
        let ok = self.lt128(raw.clone(), limit_plus_one);
        let stored = self.sel(ok.clone(), raw, Self::imm(limit_plus_one));
        (stored, ok)
    }
    fn utf8_step(
        &mut self,
        active: Operand,
        lane: Operand,
        org: &Regs,
    ) -> (Operand, Operand, Operand) {
        let b = lane;
        let is_p0 = self.eqv(org.utf_pending.clone(), 0);
        let active_p0 = self.and(active.clone(), is_p0.clone());
        let not_p0 = self.not(is_p0.clone());
        let active_cont = self.and(active.clone(), not_p0);
        let ascii = self.lt128(b.clone(), 0x80);
        let lead2 = self.in_range(b.clone(), 0xC2, 0xE0);
        let lead3 = self.in_range(b.clone(), 0xE0, 0xF0);
        let lead4 = self.in_range(b.clone(), 0xF0, 0xF5);
        let valid_start = self.or_many(vec![
            ascii.clone(),
            lead2.clone(),
            lead3.clone(),
            lead4.clone(),
        ]);
        self.gated_assert(active_p0, valid_start);
        let cont_ok = self.in_range(b.clone(), 0x80, 0xC0);
        self.gated_assert(active_cont, cont_ok.clone());
        let mut start_pending = Self::imm(0);
        start_pending = self.sel(lead2.clone(), Self::imm(1), start_pending);
        start_pending = self.sel(lead3.clone(), Self::imm(2), start_pending);
        start_pending = self.sel(lead4.clone(), Self::imm(3), start_pending);
        let cont_pending = self.add_i(org.utf_pending.clone(), -1);
        let pending_next = self.sel(is_p0.clone(), start_pending, cont_pending);
        let mut start_cp = Self::imm(0);
        let cp2 = self.add_i(b.clone(), -0xC0);
        start_cp = self.sel(lead2.clone(), cp2, start_cp);
        let cp3 = self.add_i(b.clone(), -0xE0);
        start_cp = self.sel(lead3.clone(), cp3, start_cp);
        let cp4 = self.add_i(b.clone(), -0xF0);
        start_cp = self.sel(lead4, cp4, start_cp);
        let low_raw = self.add_i(b.clone(), -0x80);
        let safe_low = self.sel(cont_ok, low_raw, Self::imm(0));
        let shifted = self.mul(org.utf_cp.clone(), Self::imm(64));
        let cont_cp = self.add(shifted, safe_low);
        let candidate = self.sel(is_p0.clone(), start_cp, cont_cp);
        let within = self.lt128(candidate.clone(), CP_CAP);
        let cp_next = self.sel(within, candidate, Self::imm(CP_CAP));
        let mut start_min = Self::imm(0);
        start_min = self.sel(lead2, Self::imm(0x80), start_min);
        start_min = self.sel(lead3, Self::imm(0x800), start_min);
        let lead4_min = self.in_range(b.clone(), 0xF0, 0xF5);
        start_min = self.sel(lead4_min, Self::imm(0x10000), start_min);
        let min_next = self.sel(is_p0, start_min, org.utf_min.clone());
        let completing = self.eqv(org.utf_pending.clone(), 1);
        let active_done = self.and(active, completing);
        let lt_min = self.lt_op(cp_next.clone(), min_next.clone());
        let ge_min = self.not(lt_min);
        self.gated_assert(active_done.clone(), ge_min);
        let le_max = self.lt128(cp_next.clone(), 0x110000);
        self.gated_assert(active_done.clone(), le_max);
        let surr = self.in_range(cp_next.clone(), 0xD800, 0xE000);
        let not_surr = self.not(surr);
        self.gated_assert(active_done, not_surr);
        (pending_next, cp_next, min_next)
    }
    fn count_step(
        &mut self,
        gate: &Operand,
        lane: &Operand,
        org: &Regs,
        nxt: &mut Regs,
        cur: u128,
        cap: usize,
        row: u128,
        zero_next: u128,
    ) {
        let cond = self.eqv(org.phase.clone(), cur);
        let active = self.and(gate.clone(), cond.clone());
        self.check_count(active, lane.clone(), cap);
        let is_zero = self.eqv(lane.clone(), 0);
        let chosen = self.sel(is_zero, Self::imm(zero_next), Self::imm(row));
        nxt.phase = self.sel(cond.clone(), chosen, nxt.phase.clone());
        nxt.rem = self.sel(cond.clone(), lane.clone(), nxt.rem.clone());
        nxt.sub = self.sel(cond, Self::imm(0), nxt.sub.clone());
    }
    fn fixed_step(&mut self, org: &Regs, nxt: &mut Regs, cur: u128, width: u128, next_table: u128) {
        let cond = self.eqv(org.phase.clone(), cur);
        let wm1 = width - 1;
        let is_last = self.eqv(org.sub.clone(), wm1);
        let done = self.and(cond.clone(), is_last.clone());
        let dec = self.add_i(org.rem.clone(), -1);
        let dec_zero = self.eqv(dec.clone(), 0);
        let chosen = self.sel(dec_zero, Self::imm(next_table), Self::imm(cur));
        nxt.phase = self.sel(done.clone(), chosen, nxt.phase.clone());
        let inc = self.add_i(org.sub.clone(), 1);
        let sub_row = self.sel(is_last, Self::imm(0), inc);
        nxt.sub = self.sel(cond, sub_row, nxt.sub.clone());
        nxt.rem = self.sel(done, dec, nxt.rem.clone());
    }
    fn var_step(&mut self, org: &Regs, nxt: &mut Regs, cur: u128, used: Operand, next_table: u128) {
        let cond = self.eqv(org.phase.clone(), cur);
        let used_m1 = self.add_i(used, -1);
        let is_last = self.teq(org.sub.clone(), used_m1);
        let done = self.and(cond.clone(), is_last.clone());
        let dec = self.add_i(org.rem.clone(), -1);
        let dec_zero = self.eqv(dec.clone(), 0);
        let chosen = self.sel(dec_zero, Self::imm(next_table), Self::imm(cur));
        nxt.phase = self.sel(done.clone(), chosen, nxt.phase.clone());
        let inc = self.add_i(org.sub.clone(), 1);
        let sub_row = self.sel(is_last, Self::imm(0), inc);
        nxt.sub = self.sel(cond, sub_row, nxt.sub.clone());
        nxt.rem = self.sel(done, dec, nxt.rem.clone());
    }
    fn used_from_hold(&mut self, org: &Regs, table: &[u128]) -> Operand {
        let mut used = Self::imm(table[0]);
        for (tag, width) in table.iter().enumerate().skip(1) {
            let is_tag = self.eqv(org.hold.clone(), tag as u128);
            used = self.sel(is_tag, Self::imm(*width), used);
        }
        used
    }
    fn string_step(
        &mut self,
        gate: &Operand,
        lane: Operand,
        org: &Regs,
        nxt: &mut Regs,
        cur: u128,
        names_budget: bool,
        caps: &C6PhysCaps,
    ) -> Operand {
        let cond = self.eqv(org.phase.clone(), cur);
        let is_len = self.eqv(org.sub.clone(), 0);
        let is_byte = self.eqv(org.sub.clone(), 1);
        let cond_len = self.and(cond.clone(), is_len);
        let cond_byte = self.and(cond.clone(), is_byte);
        let active_len = self.and(gate.clone(), cond_len.clone());
        let (limit, cum) = if names_budget {
            (caps.max_name_bytes, org.cum_name.clone())
        } else {
            (caps.max_template_bytes, org.cum_tmpl.clone())
        };
        let (stored, ok) = self.cum_next_bounded(cum, lane.clone(), (limit as u128) + 1);
        self.gated_assert(active_len, ok);
        if names_budget {
            nxt.cum_name = self.sel(cond_len.clone(), stored, nxt.cum_name.clone());
        } else {
            nxt.cum_tmpl = self.sel(cond_len.clone(), stored, nxt.cum_tmpl.clone());
        }
        nxt.utf_pending = self.sel(cond_len.clone(), Self::imm(0), nxt.utf_pending.clone());
        nxt.utf_cp = self.sel(cond_len.clone(), Self::imm(0), nxt.utf_cp.clone());
        nxt.utf_min = self.sel(cond_len.clone(), Self::imm(0), nxt.utf_min.clone());
        nxt.srem = self.sel(cond_len.clone(), lane.clone(), nxt.srem.clone());
        let lane_zero = self.eqv(lane.clone(), 0);
        let len_done = self.and(cond_len.clone(), lane_zero);
        let is_zero = self.eqv(lane.clone(), 0);
        let not_zero = self.not(is_zero);
        let go_bytes = self.and(cond_len.clone(), not_zero);
        let mut next_sub = nxt.sub.clone();
        next_sub = self.sel(go_bytes, Self::imm(1), next_sub);
        next_sub = self.sel(len_done.clone(), Self::imm(0), next_sub);
        let active_byte = self.and(gate.clone(), cond_byte.clone());
        self.check_lt(active_byte.clone(), lane.clone(), 256);
        let (pend, cp, min) = self.utf8_step(active_byte.clone(), lane.clone(), org);
        nxt.utf_pending = self.sel(cond_byte.clone(), pend.clone(), nxt.utf_pending.clone());
        nxt.utf_cp = self.sel(cond_byte.clone(), cp, nxt.utf_cp.clone());
        nxt.utf_min = self.sel(cond_byte.clone(), min, nxt.utf_min.clone());
        let is_last = self.eqv(org.srem.clone(), 1);
        let byte_done = self.and(cond_byte.clone(), is_last.clone());
        let pend_zero = self.eqv(pend, 0);
        let active_done = self.and(active_byte, is_last);
        self.gated_assert(active_done, pend_zero);
        let dec = self.add_i(org.srem.clone(), -1);
        nxt.srem = self.sel(cond_byte.clone(), dec, nxt.srem.clone());
        next_sub = self.sel(byte_done.clone(), Self::imm(0), next_sub);
        nxt.sub = self.sel(cond, next_sub, nxt.sub.clone());
        self.or(len_done, byte_done)
    }
    fn after_field_phase(&mut self, org: &Regs) -> Operand {
        let d3 = self.add_i(org.cnt3.clone(), -1);
        let z3 = self.eqv(d3, 0);
        let d2 = self.add_i(org.cnt2.clone(), -1);
        let z2 = self.eqv(d2, 0);
        let dr = self.add_i(org.rem.clone(), -1);
        let zr = self.eqv(dr, 0);
        let after_decl = self.sel(zr, Self::imm(PH_PLAN_COUNT), Self::imm(PH_DECL_NAME));
        let after_ctor = self.sel(z2, after_decl, Self::imm(PH_DECL_CTORNAME));
        self.sel(z3, after_ctor, Self::imm(PH_DECL_FIELDNAME))
    }
    fn apply_after_field(&mut self, org: &Regs, nxt: &mut Regs, event: Operand) {
        let p = self.after_field_phase(org);
        nxt.phase = self.sel(event.clone(), p, nxt.phase.clone());
        let d3 = self.add_i(org.cnt3.clone(), -1);
        let z3 = self.eqv(d3.clone(), 0);
        nxt.cnt3 = self.sel(event.clone(), d3, nxt.cnt3.clone());
        let ev2 = self.and(event.clone(), z3);
        let d2 = self.add_i(org.cnt2.clone(), -1);
        let z2 = self.eqv(d2.clone(), 0);
        nxt.cnt2 = self.sel(ev2.clone(), d2, nxt.cnt2.clone());
        let evr = self.and(ev2, z2);
        let dr = self.add_i(org.rem.clone(), -1);
        nxt.rem = self.sel(evr, dr, nxt.rem.clone());
    }
    fn after_ctor_phase(&mut self, org: &Regs) -> Operand {
        let d2 = self.add_i(org.cnt2.clone(), -1);
        let z2 = self.eqv(d2, 0);
        let dr = self.add_i(org.rem.clone(), -1);
        let zr = self.eqv(dr, 0);
        let after_decl = self.sel(zr, Self::imm(PH_PLAN_COUNT), Self::imm(PH_DECL_NAME));
        self.sel(z2, after_decl, Self::imm(PH_DECL_CTORNAME))
    }
    fn apply_after_ctor(&mut self, org: &Regs, nxt: &mut Regs, event: Operand) {
        let p = self.after_ctor_phase(org);
        nxt.phase = self.sel(event.clone(), p, nxt.phase.clone());
        let d2 = self.add_i(org.cnt2.clone(), -1);
        let z2 = self.eqv(d2.clone(), 0);
        nxt.cnt2 = self.sel(event.clone(), d2, nxt.cnt2.clone());
        let evr = self.and(event, z2);
        let dr = self.add_i(org.rem.clone(), -1);
        nxt.rem = self.sel(evr, dr, nxt.rem.clone());
    }
    fn after_decl_phase(&mut self, org: &Regs) -> Operand {
        let dr = self.add_i(org.rem.clone(), -1);
        let zr = self.eqv(dr, 0);
        self.sel(zr, Self::imm(PH_PLAN_COUNT), Self::imm(PH_DECL_NAME))
    }
    fn apply_after_decl(&mut self, org: &Regs, nxt: &mut Regs, event: Operand) {
        let p = self.after_decl_phase(org);
        nxt.phase = self.sel(event.clone(), p, nxt.phase.clone());
        let dr = self.add_i(org.rem.clone(), -1);
        nxt.rem = self.sel(event, dr, nxt.rem.clone());
    }
    fn step(
        &mut self,
        gate: &Operand,
        lane: Operand,
        org: Regs,
        caps: &C6PhysCaps,
        selected_const: Option<&Operand>,
        selected_ctor: Option<(&Operand, &Operand)>,
    ) -> Regs {
        let mut nxt = org.clone();
        let p_magic = self.eqv(org.phase.clone(), PH_MAGIC);
        let a = self.and(gate.clone(), p_magic);
        self.gated_eq(a, lane.clone(), Self::imm(C6_PHYS_MAGIC));
        let p_version = self.eqv(org.phase.clone(), PH_VERSION);
        let a = self.and(gate.clone(), p_version);
        self.gated_eq(a, lane.clone(), Self::imm(C6_PHYS_VERSION));
        let p_tag = self.eqv(org.phase.clone(), PH_TAG);
        let a = self.and(gate.clone(), p_tag);
        self.gated_eq(a, lane.clone(), Self::imm(PROGRAM_SECTION_TAG));
        let p_entry = self.eqv(org.phase.clone(), PH_ENTRY);
        let a = self.and(gate.clone(), p_entry);
        self.check_usize(a, lane.clone());
        {
            let row = self.eqv(org.phase.clone(), PH_EXPR_ROW);
            let s0 = self.eqv(org.sub.clone(), 0);
            let c = self.and(row.clone(), s0.clone());
            let a = self.and(gate.clone(), c);
            self.check_lt(a, lane.clone(), 9);
            let ns0 = self.not(s0);
            let body = self.and(row.clone(), ns0);
            let a = self.and(gate.clone(), body.clone());
            self.check_usize(a, lane.clone());
            let h3 = self.eqv(org.hold.clone(), 3);
            let s1 = self.eqv(org.sub.clone(), 1);
            let hs = self.and(h3, s1.clone());
            let c = self.and(body.clone(), hs);
            let a = self.and(gate.clone(), c);
            self.check_lt(a, lane.clone(), 7);
            let h4 = self.eqv(org.hold.clone(), 4);
            let hs = self.and(h4, s1);
            let c = self.and(body.clone(), hs);
            let a = self.and(gate.clone(), c);
            self.check_lt(a, lane.clone(), 2);
            let h7 = self.eqv(org.hold.clone(), 7);
            let h8 = self.eqv(org.hold.clone(), 8);
            let s4 = self.eqv(org.sub.clone(), 4);
            let s5 = self.eqv(org.sub.clone(), 5);
            let s6 = self.eqv(org.sub.clone(), 6);
            let oa = self.and(h7.clone(), s4.clone());
            let ob = self.and(h8.clone(), s5.clone());
            let oc = self.or(oa, ob);
            let rc = self.and(row.clone(), oc);
            let a = self.and(gate.clone(), rc);
            self.gated_bool(a, lane.clone());
            let va = self.and(h7, s5);
            let vb = self.and(h8, s6);
            let vc = self.or(va, vb);
            let rv = self.and(row, vc);
            let absent = self.eqv(org.opt.clone(), 0);
            let ac = self.and(rv, absent);
            let a = self.and(gate.clone(), ac);
            self.gated_zero(a, lane.clone());
        }
        {
            let row = self.eqv(org.phase.clone(), PH_OCC_ROW);
            let s0 = self.eqv(org.sub.clone(), 0);
            let c = self.and(row, s0);
            let a = self.and(gate.clone(), c);
            self.gated_bool(a, lane.clone());
        }
        {
            let row = self.eqv(org.phase.clone(), PH_TYPE_ROW);
            let s1 = self.eqv(org.sub.clone(), 1);
            let c = self.and(row.clone(), s1);
            let a = self.and(gate.clone(), c);
            self.gated_bool(a, lane.clone());
            let s2 = self.eqv(org.sub.clone(), 2);
            let c = self.and(row, s2);
            let absent = self.eqv(org.opt.clone(), 0);
            let ac = self.and(c, absent);
            let a = self.and(gate.clone(), ac);
            self.gated_zero(a, lane.clone());
        }
        {
            let row = self.eqv(org.phase.clone(), PH_PLANCASE_ROW);
            let s0 = self.eqv(org.sub.clone(), 0);
            let c = self.and(row.clone(), s0);
            let a = self.and(gate.clone(), c);
            self.gated_bool(a, lane.clone());
            let s1 = self.eqv(org.sub.clone(), 1);
            let c = self.and(row, s1);
            let absent = self.eqv(org.opt.clone(), 0);
            let ac = self.and(c, absent);
            let a = self.and(gate.clone(), ac);
            self.gated_zero(a, lane.clone());
        }
        {
            let row = self.eqv(org.phase.clone(), PH_PAT_ROW);
            let s0 = self.eqv(org.sub.clone(), 0);
            let c = self.and(row, s0);
            let a = self.and(gate.clone(), c);
            self.check_lt(a, lane.clone(), 7);
        }
        {
            let row = self.eqv(org.phase.clone(), PH_PLAN_ROW);
            let s0 = self.eqv(org.sub.clone(), 0);
            let c = self.and(row, s0);
            let a = self.and(gate.clone(), c);
            self.check_lt(a, lane.clone(), 4);
        }
        {
            let row = self.eqv(org.phase.clone(), PH_CONST_ROW);
            let s0 = self.eqv(org.sub.clone(), 0);
            let c = self.and(row.clone(), s0.clone());
            let a = self.and(gate.clone(), c);
            self.check_lt(a, lane.clone(), 8);
            let s1 = self.eqv(org.sub.clone(), 1);
            let c = self.and(row.clone(), s1);
            let a = self.and(gate.clone(), c);
            self.gated_bool(a, lane.clone());
            let h1 = self.eqv(org.hold.clone(), 1);
            let s2 = self.eqv(org.sub.clone(), 2);
            let hs = self.and(h1, s2);
            let c = self.and(row, hs);
            let a = self.and(gate.clone(), c);
            self.gated_bool(a, lane.clone());
        }
        {
            let mut phases: Vec<Operand> = Vec::new();
            for ph in [
                PH_OCC_ROW,
                PH_OCCPATH_ROW,
                PH_CHILD_ROW,
                PH_FUNC_ROW,
                PH_PARAM_ROW,
                PH_ARM_ROW,
                PH_PAT_ROW,
                PH_PATCHILD_ROW,
                PH_TYPE_ROW,
                PH_TYPEARG_ROW,
                PH_CTOR_ROW,
                PH_CTORFIELD_ROW,
                PH_DECPARAMCNT_ROW,
                PH_DECCTORCNT_ROW,
                PH_PLAN_ROW,
                PH_FIELDPATH_ROW,
                PH_CONSTFIELD_ROW,
            ] {
                let e = self.eqv(org.phase.clone(), ph);
                phases.push(e);
            }
            let in_row = self.or_many(phases);
            let s0 = self.eqv(org.sub.clone(), 0);
            let not_tag = self.not(s0);
            let payload = self.and(in_row, not_tag);
            let mut ones: Vec<Operand> = Vec::new();
            for ph in [
                PH_OCCPATH_ROW,
                PH_CHILD_ROW,
                PH_PATCHILD_ROW,
                PH_TYPEARG_ROW,
                PH_DECPARAMCNT_ROW,
                PH_DECCTORCNT_ROW,
                PH_FIELDPATH_ROW,
                PH_CONSTFIELD_ROW,
            ] {
                let e = self.eqv(org.phase.clone(), ph);
                ones.push(e);
            }
            let one_lane = self.or_many(ones);
            let payload = self.or(payload, one_lane);
            let crow = self.eqv(org.phase.clone(), PH_CONST_ROW);
            let cs0 = self.eqv(org.sub.clone(), 0);
            let cs1 = self.eqv(org.sub.clone(), 1);
            let cs2 = self.eqv(org.sub.clone(), 2);
            let h0 = self.eqv(org.hold.clone(), 0);
            let h1 = self.eqv(org.hold.clone(), 1);
            let u128pay = self.and(h0, cs2.clone());
            let boolpay = self.and(h1, cs2);
            let mut excl = self.or(cs0, cs1);
            excl = self.or(excl, u128pay);
            excl = self.or(excl, boolpay);
            let not_excl = self.not(excl);
            let const_payload = self.and(crow, not_excl);
            let payload = self.or(payload, const_payload);
            let a = self.and(gate.clone(), payload);
            self.check_usize(a, lane.clone());
        }
        for cur in [PH_MAGIC, PH_VERSION, PH_TAG, PH_ENTRY] {
            let cond = self.eqv(org.phase.clone(), cur);
            nxt.phase = self.sel(cond, Self::imm(cur + 1), nxt.phase.clone());
        }
        self.count_step(
            gate,
            &lane,
            &org,
            &mut nxt,
            PH_EXPR_COUNT,
            caps.max_expressions,
            PH_EXPR_ROW,
            PH_OCC_COUNT,
        );
        self.count_step(
            gate,
            &lane,
            &org,
            &mut nxt,
            PH_OCC_COUNT,
            caps.max_occurrences,
            PH_OCC_ROW,
            PH_OCCPATH_COUNT,
        );
        self.count_step(
            gate,
            &lane,
            &org,
            &mut nxt,
            PH_OCCPATH_COUNT,
            caps.max_occurrence_paths,
            PH_OCCPATH_ROW,
            PH_CHILD_COUNT,
        );
        self.count_step(
            gate,
            &lane,
            &org,
            &mut nxt,
            PH_CHILD_COUNT,
            caps.max_children,
            PH_CHILD_ROW,
            PH_FUNC_COUNT,
        );
        self.count_step(
            gate,
            &lane,
            &org,
            &mut nxt,
            PH_FUNC_COUNT,
            caps.max_functions,
            PH_FUNC_ROW,
            PH_PARAM_COUNT,
        );
        self.count_step(
            gate,
            &lane,
            &org,
            &mut nxt,
            PH_PARAM_COUNT,
            caps.max_parameters,
            PH_PARAM_ROW,
            PH_ARM_COUNT,
        );
        self.count_step(
            gate,
            &lane,
            &org,
            &mut nxt,
            PH_ARM_COUNT,
            caps.max_arms,
            PH_ARM_ROW,
            PH_PAT_COUNT,
        );
        self.count_step(
            gate,
            &lane,
            &org,
            &mut nxt,
            PH_PAT_COUNT,
            caps.max_patterns,
            PH_PAT_ROW,
            PH_PATCHILD_COUNT,
        );
        self.count_step(
            gate,
            &lane,
            &org,
            &mut nxt,
            PH_PATCHILD_COUNT,
            caps.max_pattern_children,
            PH_PATCHILD_ROW,
            PH_TYPE_COUNT,
        );
        self.count_step(
            gate,
            &lane,
            &org,
            &mut nxt,
            PH_TYPE_COUNT,
            caps.max_types,
            PH_TYPE_ROW,
            PH_TYPEARG_COUNT,
        );
        self.count_step(
            gate,
            &lane,
            &org,
            &mut nxt,
            PH_TYPEARG_COUNT,
            caps.max_types,
            PH_TYPEARG_ROW,
            PH_CTOR_COUNT,
        );
        self.count_step(
            gate,
            &lane,
            &org,
            &mut nxt,
            PH_CTOR_COUNT,
            caps.max_constructors,
            PH_CTOR_ROW,
            PH_CTORFIELD_COUNT,
        );
        self.count_step(
            gate,
            &lane,
            &org,
            &mut nxt,
            PH_CTORFIELD_COUNT,
            caps.max_ctor_fields,
            PH_CTORFIELD_ROW,
            PH_DECPARAMCNT_COUNT,
        );
        self.count_step(
            gate,
            &lane,
            &org,
            &mut nxt,
            PH_DECPARAMCNT_COUNT,
            caps.max_decl_templates,
            PH_DECPARAMCNT_ROW,
            PH_DECCTORCNT_COUNT,
        );
        self.count_step(
            gate,
            &lane,
            &org,
            &mut nxt,
            PH_DECCTORCNT_COUNT,
            caps.max_decl_templates,
            PH_DECCTORCNT_ROW,
            PH_DECL_COUNT,
        );
        self.count_step(
            gate,
            &lane,
            &org,
            &mut nxt,
            PH_DECL_COUNT,
            caps.max_decl_templates,
            PH_DECL_NAME,
            PH_PLAN_COUNT,
        );
        self.count_step(
            gate,
            &lane,
            &org,
            &mut nxt,
            PH_PLAN_COUNT,
            caps.max_plan_nodes,
            PH_PLAN_ROW,
            PH_PLANCASE_COUNT,
        );
        self.count_step(
            gate,
            &lane,
            &org,
            &mut nxt,
            PH_PLANCASE_COUNT,
            caps.max_plan_cases,
            PH_PLANCASE_ROW,
            PH_FIELDPATH_COUNT,
        );
        self.count_step(
            gate,
            &lane,
            &org,
            &mut nxt,
            PH_FIELDPATH_COUNT,
            caps.max_field_paths,
            PH_FIELDPATH_ROW,
            PH_CONST_COUNT,
        );
        self.count_step(
            gate,
            &lane,
            &org,
            &mut nxt,
            PH_CONST_COUNT,
            caps.max_constants,
            PH_CONST_ROW,
            PH_CONSTFIELD_COUNT,
        );
        self.count_step(
            gate,
            &lane,
            &org,
            &mut nxt,
            PH_CONSTFIELD_COUNT,
            caps.max_constant_fields,
            PH_CONSTFIELD_ROW,
            PH_NAMES_COUNT,
        );
        self.count_step(
            gate,
            &lane,
            &org,
            &mut nxt,
            PH_NAMES_COUNT,
            caps.max_names,
            PH_NAMES_STR,
            PH_END,
        );
        for cur in [PH_EXPR_ROW, PH_PAT_ROW, PH_PLAN_ROW, PH_CONST_ROW] {
            let row = self.eqv(org.phase.clone(), cur);
            let s0 = self.eqv(org.sub.clone(), 0);
            let cond = self.and(row, s0);
            nxt.hold = self.sel(cond, lane.clone(), nxt.hold.clone());
        }
        {
            let row = self.eqv(org.phase.clone(), PH_EXPR_ROW);
            let h7 = self.eqv(org.hold.clone(), 7);
            let h8 = self.eqv(org.hold.clone(), 8);
            let s4 = self.eqv(org.sub.clone(), 4);
            let s5 = self.eqv(org.sub.clone(), 5);
            let ca = self.and(h7, s4);
            let ca = self.and(row.clone(), ca);
            let cb = self.and(h8, s5);
            let cb = self.and(row, cb);
            let cond = self.or(ca, cb);
            nxt.opt = self.sel(cond, lane.clone(), nxt.opt.clone());
        }
        {
            let row = self.eqv(org.phase.clone(), PH_TYPE_ROW);
            let s1 = self.eqv(org.sub.clone(), 1);
            let cond = self.and(row, s1);
            nxt.opt = self.sel(cond, lane.clone(), nxt.opt.clone());
        }
        {
            let row = self.eqv(org.phase.clone(), PH_PLANCASE_ROW);
            let s0 = self.eqv(org.sub.clone(), 0);
            let cond = self.and(row, s0);
            nxt.opt = self.sel(cond, lane.clone(), nxt.opt.clone());
        }
        let used = self.used_from_hold(&org, &[2, 2, 4, 4, 3, 4, 4, 6, 7]);
        self.var_step(&org, &mut nxt, PH_EXPR_ROW, used, PH_OCC_COUNT);
        let used = self.used_from_hold(&org, &[2, 1, 3, 2, 3, 1, 5]);
        self.var_step(&org, &mut nxt, PH_PAT_ROW, used, PH_PATCHILD_COUNT);
        let used = self.used_from_hold(&org, &[8, 6, 3, 2]);
        self.var_step(&org, &mut nxt, PH_PLAN_ROW, used, PH_PLANCASE_COUNT);
        let used = self.used_from_hold(&org, &[3, 3, 2, 4, 4, 4, 6, 2]);
        self.var_step(&org, &mut nxt, PH_CONST_ROW, used, PH_CONSTFIELD_COUNT);
        self.fixed_step(&org, &mut nxt, PH_OCC_ROW, 4, PH_OCCPATH_COUNT);
        self.fixed_step(&org, &mut nxt, PH_OCCPATH_ROW, 1, PH_CHILD_COUNT);
        self.fixed_step(&org, &mut nxt, PH_CHILD_ROW, 1, PH_FUNC_COUNT);
        self.fixed_step(&org, &mut nxt, PH_FUNC_ROW, 5, PH_PARAM_COUNT);
        self.fixed_step(&org, &mut nxt, PH_PARAM_ROW, 2, PH_ARM_COUNT);
        self.fixed_step(&org, &mut nxt, PH_ARM_ROW, 2, PH_PAT_COUNT);
        self.fixed_step(&org, &mut nxt, PH_PATCHILD_ROW, 1, PH_TYPE_COUNT);
        self.fixed_step(&org, &mut nxt, PH_TYPE_ROW, 7, PH_TYPEARG_COUNT);
        self.fixed_step(&org, &mut nxt, PH_TYPEARG_ROW, 1, PH_CTOR_COUNT);
        self.fixed_step(&org, &mut nxt, PH_CTOR_ROW, 5, PH_CTORFIELD_COUNT);
        self.fixed_step(&org, &mut nxt, PH_CTORFIELD_ROW, 2, PH_DECPARAMCNT_COUNT);
        self.fixed_step(&org, &mut nxt, PH_DECPARAMCNT_ROW, 1, PH_DECCTORCNT_COUNT);
        self.fixed_step(&org, &mut nxt, PH_DECCTORCNT_ROW, 1, PH_DECL_COUNT);
        self.fixed_step(&org, &mut nxt, PH_PLANCASE_ROW, 2, PH_FIELDPATH_COUNT);
        self.fixed_step(&org, &mut nxt, PH_FIELDPATH_ROW, 1, PH_CONST_COUNT);
        self.fixed_step(&org, &mut nxt, PH_CONSTFIELD_ROW, 1, PH_NAMES_COUNT);
        let done = self.string_step(
            gate,
            lane.clone(),
            &org,
            &mut nxt,
            PH_DECL_NAME,
            false,
            caps,
        );
        nxt.phase = self.sel(done, Self::imm(PH_DECL_PARAMCNT), nxt.phase.clone());
        {
            let cond = self.eqv(org.phase.clone(), PH_DECL_PARAMCNT);
            let a = self.and(gate.clone(), cond.clone());
            self.check_count(a, lane.clone(), caps.max_arity);
            nxt.cnt = self.sel(cond.clone(), lane.clone(), nxt.cnt.clone());
            let z = self.eqv(lane.clone(), 0);
            let chosen = self.sel(z, Self::imm(PH_DECL_CTORCNT), Self::imm(PH_DECL_PARAM));
            nxt.phase = self.sel(cond, chosen, nxt.phase.clone());
        }
        let done = self.string_step(
            gate,
            lane.clone(),
            &org,
            &mut nxt,
            PH_DECL_PARAM,
            false,
            caps,
        );
        {
            let dec = self.add_i(org.cnt.clone(), -1);
            let z = self.eqv(dec.clone(), 0);
            let chosen = self.sel(z, Self::imm(PH_DECL_CTORCNT), Self::imm(PH_DECL_PARAM));
            nxt.phase = self.sel(done.clone(), chosen, nxt.phase.clone());
            nxt.cnt = self.sel(done, dec, nxt.cnt.clone());
        }
        {
            let cond = self.eqv(org.phase.clone(), PH_DECL_CTORCNT);
            let a = self.and(gate.clone(), cond.clone());
            self.check_count(a, lane.clone(), caps.max_names);
            nxt.cnt2 = self.sel(cond.clone(), lane.clone(), nxt.cnt2.clone());
            let z = self.eqv(lane.clone(), 0);
            let nz = self.not(z);
            let go = self.and(cond.clone(), nz);
            nxt.phase = self.sel(go, Self::imm(PH_DECL_CTORNAME), nxt.phase.clone());
            let z2 = self.eqv(lane.clone(), 0);
            let ev = self.and(cond, z2);
            self.apply_after_decl(&org, &mut nxt, ev);
        }
        let done = self.string_step(
            gate,
            lane.clone(),
            &org,
            &mut nxt,
            PH_DECL_CTORNAME,
            false,
            caps,
        );
        nxt.phase = self.sel(done, Self::imm(PH_DECL_FIELDCNT), nxt.phase.clone());
        {
            let cond = self.eqv(org.phase.clone(), PH_DECL_FIELDCNT);
            let a = self.and(gate.clone(), cond.clone());
            self.check_count(a, lane.clone(), caps.max_arity);
            nxt.cnt3 = self.sel(cond.clone(), lane.clone(), nxt.cnt3.clone());
            let z = self.eqv(lane.clone(), 0);
            let nz = self.not(z);
            let go = self.and(cond.clone(), nz);
            nxt.phase = self.sel(go, Self::imm(PH_DECL_FIELDNAME), nxt.phase.clone());
            let z2 = self.eqv(lane.clone(), 0);
            let ev = self.and(cond, z2);
            self.apply_after_ctor(&org, &mut nxt, ev);
        }
        let done = self.string_step(
            gate,
            lane.clone(),
            &org,
            &mut nxt,
            PH_DECL_FIELDNAME,
            false,
            caps,
        );
        nxt.phase = self.sel(done.clone(), Self::imm(PH_TYPEEXPR), nxt.phase.clone());
        nxt.te_need = self.sel(done.clone(), Self::imm(1), nxt.te_need.clone());
        nxt.sub = self.sel(done, Self::imm(0), nxt.sub.clone());
        {
            let cond = self.eqv(org.phase.clone(), PH_TYPEEXPR);
            let s0 = self.eqv(org.sub.clone(), 0);
            let tag_cond = self.and(cond.clone(), s0);
            let tag_active = self.and(gate.clone(), tag_cond.clone());
            self.check_lt(tag_active.clone(), lane.clone(), 7);
            let (stored, ok) = self.cum_next_bounded(
                org.cum_nodes.clone(),
                Self::imm(1),
                (caps.max_type_expr_nodes as u128) + 1,
            );
            self.gated_assert(tag_active, ok);
            nxt.cum_nodes = self.sel(tag_cond.clone(), stored, nxt.cum_nodes.clone());
            nxt.hold = self.sel(tag_cond.clone(), lane.clone(), nxt.hold.clone());
            let is_leaf = self.lt128(lane.clone(), 4);
            let t4 = self.eqv(lane.clone(), 4);
            let t5 = self.eqv(lane.clone(), 5);
            let named = self.or(t4, t5);
            let apply = self.eqv(lane.clone(), 6);
            let need_dec = self.add_i(org.te_need.clone(), -1);
            let leaf_zero = self.eqv(need_dec.clone(), 0);
            let leaf_mid = self.and(is_leaf.clone(), leaf_zero);
            let leaf_event = self.and(tag_cond.clone(), leaf_mid);
            let go_str = self.or(named.clone(), apply.clone());
            let go_cond = self.and(tag_cond.clone(), go_str);
            nxt.sub = self.sel(go_cond, Self::imm(1), nxt.sub.clone());
            let leaf_cond = self.and(tag_cond.clone(), is_leaf);
            nxt.te_need = self.sel(leaf_cond, need_dec.clone(), nxt.te_need.clone());
            self.apply_after_field(&org, &mut nxt, leaf_event);
            let es1 = self.eqv(org.sub.clone(), 1);
            let len_cond = self.and(cond.clone(), es1);
            let active_len = self.and(gate.clone(), len_cond.clone());
            let (stored_t, ok_t) = self.cum_next_bounded(
                org.cum_tmpl.clone(),
                lane.clone(),
                (caps.max_template_bytes as u128) + 1,
            );
            self.gated_assert(active_len, ok_t);
            nxt.cum_tmpl = self.sel(len_cond.clone(), stored_t, nxt.cum_tmpl.clone());
            nxt.utf_pending = self.sel(len_cond.clone(), Self::imm(0), nxt.utf_pending.clone());
            nxt.utf_cp = self.sel(len_cond.clone(), Self::imm(0), nxt.utf_cp.clone());
            nxt.utf_min = self.sel(len_cond.clone(), Self::imm(0), nxt.utf_min.clone());
            nxt.srem = self.sel(len_cond.clone(), lane.clone(), nxt.srem.clone());
            let lz = self.eqv(lane.clone(), 0);
            let lnz = self.not(lz.clone());
            let go_bytes = self.and(len_cond.clone(), lnz);
            nxt.sub = self.sel(go_bytes, Self::imm(2), nxt.sub.clone());
            let lz2 = self.eqv(lane.clone(), 0);
            let str_done_now = self.and(len_cond, lz2);
            let es2 = self.eqv(org.sub.clone(), 2);
            let byte_cond = self.and(cond.clone(), es2);
            let active_byte = self.and(gate.clone(), byte_cond.clone());
            self.check_lt(active_byte.clone(), lane.clone(), 256);
            let (pend, cp, min) = self.utf8_step(active_byte.clone(), lane.clone(), &org);
            nxt.utf_pending = self.sel(byte_cond.clone(), pend.clone(), nxt.utf_pending.clone());
            nxt.utf_cp = self.sel(byte_cond.clone(), cp, nxt.utf_cp.clone());
            nxt.utf_min = self.sel(byte_cond.clone(), min, nxt.utf_min.clone());
            let lastb = self.eqv(org.srem.clone(), 1);
            let byte_done = self.and(byte_cond.clone(), lastb.clone());
            let pz = self.eqv(pend, 0);
            let adone = self.and(active_byte, lastb);
            self.gated_assert(adone, pz);
            let sdec = self.add_i(org.srem.clone(), -1);
            nxt.srem = self.sel(byte_cond, sdec, nxt.srem.clone());
            let any_done = self.or(str_done_now, byte_done);
            let hold_apply = self.eqv(org.hold.clone(), 6);
            let to_arity = self.and(any_done.clone(), hold_apply.clone());
            nxt.sub = self.sel(to_arity, Self::imm(3), nxt.sub.clone());
            let not_apply = self.not(hold_apply);
            let named_done = self.and(any_done.clone(), not_apply);
            let nd = self.add_i(org.te_need.clone(), -1);
            let nz2 = self.eqv(nd.clone(), 0);
            let named_event = self.and(named_done.clone(), nz2);
            nxt.te_need = self.sel(named_done.clone(), nd, nxt.te_need.clone());
            nxt.sub = self.sel(named_done, Self::imm(0), nxt.sub.clone());
            self.apply_after_field(&org, &mut nxt, named_event);
            let es3 = self.eqv(org.sub.clone(), 3);
            let arity_cond = self.and(cond, es3);
            let arity_active = self.and(gate.clone(), arity_cond.clone());
            self.check_count(arity_active, lane.clone(), caps.max_arity);
            let bound = (caps.max_type_expr_nodes as u128) + 1;
            let within = self.lt128(lane.clone(), bound);
            let capped = self.sel(within, lane.clone(), Self::imm(bound));
            let base = self.add_i(org.te_need.clone(), -1);
            let raw_need = self.add(base, capped);
            let te_zero = self.eqv(org.te_need.clone(), 0);
            let safe_raw = self.sel(te_zero, Self::imm(bound), raw_need.clone());
            let rwithin = self.lt128(safe_raw, bound);
            let new_need = self.sel(rwithin, raw_need, Self::imm(bound));
            let az = self.eqv(new_need.clone(), 0);
            let arity_event = self.and(arity_cond.clone(), az);
            nxt.te_need = self.sel(arity_cond.clone(), new_need, nxt.te_need.clone());
            nxt.sub = self.sel(arity_cond, Self::imm(0), nxt.sub.clone());
            self.apply_after_field(&org, &mut nxt, arity_event);
        }
        let done = self.string_step(gate, lane.clone(), &org, &mut nxt, PH_NAMES_STR, true, caps);
        {
            let dec = self.add_i(org.rem.clone(), -1);
            let z = self.eqv(dec.clone(), 0);
            let chosen = self.sel(z, Self::imm(PH_END), Self::imm(PH_NAMES_STR));
            nxt.phase = self.sel(done.clone(), chosen, nxt.phase.clone());
            nxt.rem = self.sel(done, dec, nxt.rem.clone());
        }
        {
            let cond_end = self.eqv(org.phase.clone(), PH_END);
            let a = self.and(gate.clone(), cond_end);
            self.gated_assert(a, Self::imm(0));
        }
        if let Some(selected) = selected_const {
            let is_ccount = self.eqv(org.phase.clone(), PH_CONST_COUNT);
            nxt.const_count = self.sel(is_ccount, lane.clone(), nxt.const_count.clone());
            let is_crow = self.eqv(org.phase.clone(), PH_CONST_ROW);
            let is_start = self.eqv(org.sub.clone(), 0);
            let row_start = self.and(is_crow.clone(), is_start);
            let idx_eq = self.teq(org.const_row_index.clone(), selected.clone());
            let active_here = self.sel(row_start, idx_eq, org.const_capture_active.clone());
            let lane_active = self.and(is_crow.clone(), active_here.clone());
            nxt.const_capture_active = self.sel(
                is_crow.clone(),
                active_here,
                nxt.const_capture_active.clone(),
            );
            nxt.const_found = self.sel(lane_active.clone(), Self::imm(1), nxt.const_found.clone());
            let s0 = self.eqv(org.sub.clone(), 0);
            let h0 = self.and(lane_active.clone(), s0);
            nxt.const_cap_0 = self.sel(h0, lane.clone(), nxt.const_cap_0.clone());
            let s1 = self.eqv(org.sub.clone(), 1);
            let h1 = self.and(lane_active.clone(), s1);
            nxt.const_cap_1 = self.sel(h1, lane.clone(), nxt.const_cap_1.clone());
            let s2 = self.eqv(org.sub.clone(), 2);
            let h2 = self.and(lane_active.clone(), s2);
            nxt.const_cap_2 = self.sel(h2, lane.clone(), nxt.const_cap_2.clone());
            let s3 = self.eqv(org.sub.clone(), 3);
            let h3 = self.and(lane_active.clone(), s3);
            nxt.const_cap_3 = self.sel(h3, lane.clone(), nxt.const_cap_3.clone());
            let s4 = self.eqv(org.sub.clone(), 4);
            let h4 = self.and(lane_active.clone(), s4);
            nxt.const_cap_4 = self.sel(h4, lane.clone(), nxt.const_cap_4.clone());
            let s5 = self.eqv(org.sub.clone(), 5);
            let h5 = self.and(lane_active, s5);
            nxt.const_cap_5 = self.sel(h5, lane.clone(), nxt.const_cap_5.clone());
            let used = self.used_from_hold(&org, &[3, 3, 2, 4, 4, 4, 6, 2]);
            let used_m1 = self.add_i(used, -1);
            let is_last = self.teq(org.sub.clone(), used_m1);
            let row_done = self.and(is_crow, is_last);
            let next_idx = self.add_i(org.const_row_index.clone(), 1);
            nxt.const_row_index = self.sel(row_done, next_idx, nxt.const_row_index.clone());
        }
        if let Some((sel_type, sel_ctor)) = selected_ctor {
            let is_tcount = self.eqv(org.phase.clone(), PH_TYPE_COUNT);
            nxt.types_count = self.sel(is_tcount, lane.clone(), nxt.types_count.clone());
            let is_trow = self.eqv(org.phase.clone(), PH_TYPE_ROW);
            let is_s0 = self.eqv(org.sub.clone(), 0);
            let row_start = self.and(is_trow.clone(), is_s0);
            let idx_eq = self.teq(org.type_row_index.clone(), (*sel_type).clone());
            let active_here = self.sel(row_start, idx_eq, org.type_capture_active.clone());
            let lane_active = self.and(is_trow.clone(), active_here.clone());
            nxt.type_capture_active = self.sel(
                is_trow.clone(),
                active_here,
                nxt.type_capture_active.clone(),
            );
            nxt.type_found = self.sel(lane_active.clone(), Self::imm(1), nxt.type_found.clone());
            let s5 = self.eqv(org.sub.clone(), 5);
            let h5 = self.and(lane_active.clone(), s5);
            nxt.type_ctor_start = self.sel(h5, lane.clone(), nxt.type_ctor_start.clone());
            let s6 = self.eqv(org.sub.clone(), 6);
            let h6 = self.and(lane_active.clone(), s6);
            nxt.type_ctor_len = self.sel(h6, lane.clone(), nxt.type_ctor_len.clone());
            let is_last = self.eqv(org.sub.clone(), 6);
            let row_done = self.and(is_trow, is_last);
            let next_idx = self.add_i(org.type_row_index.clone(), 1);
            nxt.type_row_index = self.sel(row_done, next_idx, nxt.type_row_index.clone());
            let is_fcount = self.eqv(org.phase.clone(), PH_CTORFIELD_COUNT);
            nxt.ctor_fields_count =
                self.sel(is_fcount, lane.clone(), nxt.ctor_fields_count.clone());
            let is_crow = self.eqv(org.phase.clone(), PH_CTOR_ROW);
            let is_cs0 = self.eqv(org.sub.clone(), 0);
            let crow_start = self.and(is_crow.clone(), is_cs0);
            let cidx_eq = self.teq(org.ctor_row_index.clone(), (*sel_ctor).clone());
            let cactive_here = self.sel(crow_start, cidx_eq, org.ctor_capture_active.clone());
            let clane_active = self.and(is_crow.clone(), cactive_here.clone());
            nxt.ctor_capture_active = self.sel(
                is_crow.clone(),
                cactive_here,
                nxt.ctor_capture_active.clone(),
            );
            nxt.ctor_found = self.sel(clane_active.clone(), Self::imm(1), nxt.ctor_found.clone());
            let cs0 = self.eqv(org.sub.clone(), 0);
            let ch0 = self.and(clane_active.clone(), cs0);
            nxt.ctor_owning_type = self.sel(ch0, lane.clone(), nxt.ctor_owning_type.clone());
            let cs3 = self.eqv(org.sub.clone(), 3);
            let ch3 = self.and(clane_active.clone(), cs3);
            nxt.ctor_field_start = self.sel(ch3, lane.clone(), nxt.ctor_field_start.clone());
            let cs4 = self.eqv(org.sub.clone(), 4);
            let ch4 = self.and(clane_active, cs4);
            nxt.ctor_field_len = self.sel(ch4, lane.clone(), nxt.ctor_field_len.clone());
            let is_clast = self.eqv(org.sub.clone(), 4);
            let crow_done = self.and(is_crow, is_clast);
            let cnext_idx = self.add_i(org.ctor_row_index.clone(), 1);
            nxt.ctor_row_index = self.sel(crow_done, cnext_idx, nxt.ctor_row_index.clone());
        }
        nxt
    }
}
pub struct C6ProgramContextCanonicalEmitter {
    scope: String,
    next_temp: usize,
}
impl C6ProgramContextCanonicalEmitter {
    pub fn new(scope: &str) -> Result<Self, C6ProgramContextCanonicalError> {
        validate_scope(scope)?;
        Ok(Self {
            scope: scope.to_string(),
            next_temp: 0,
        })
    }
    pub fn scope(&self) -> &str {
        &self.scope
    }
    pub fn next_temp(&self) -> usize {
        self.next_temp
    }
    pub fn emit(
        &mut self,
        inputs: &C6ProgramContextCanonicalInputs,
    ) -> Result<C6ProgramContextCanonicalBlock, C6ProgramContextCanonicalError> {
        if (LANE_BITS as usize) >= FR_BITS {
            return Err(C6ProgramContextCanonicalError::InvalidBitWidth { bits: LANE_BITS });
        }
        reject_reserved(&self.scope, &inputs.gate)?;
        for lane in &inputs.pre_prog {
            reject_reserved(&self.scope, lane)?;
        }
        for lane in &inputs.pre_ctx {
            reject_reserved(&self.scope, lane)?;
        }
        if inputs.pre_prog.len() < PROGRAM_HEADER_LANES {
            return Err(C6ProgramContextCanonicalError::LengthMismatch {
                expected: PROGRAM_HEADER_LANES,
                actual: inputs.pre_prog.len(),
            });
        }
        let p = &inputs.caps.phys;
        for cap in [
            p.max_expressions,
            p.max_names,
            p.max_arms,
            p.max_children,
            p.max_functions,
            p.max_arity,
            p.max_types,
            p.max_occurrences,
            p.max_occurrence_paths,
            p.max_parameters,
            p.max_patterns,
            p.max_pattern_children,
            p.max_constructors,
            p.max_ctor_fields,
            p.max_decl_templates,
            p.max_plan_nodes,
            p.max_plan_cases,
            p.max_field_paths,
            p.max_constants,
            p.max_constant_fields,
            p.max_name_bytes,
            p.max_type_expr_nodes,
            p.max_template_bytes,
        ] {
            if cap == usize::MAX {
                return Err(C6ProgramContextCanonicalError::GeometryOverflow);
            }
        }
        let mut em = Emitter::new(&self.scope, self.next_temp);
        let gate = inputs.gate.clone();
        em.cbool(gate.clone());
        let mut regs = Regs {
            phase: Emitter::imm(0),
            sub: Emitter::imm(0),
            hold: Emitter::imm(0),
            opt: Emitter::imm(0),
            rem: Emitter::imm(0),
            cnt: Emitter::imm(0),
            cnt2: Emitter::imm(0),
            cnt3: Emitter::imm(0),
            te_need: Emitter::imm(0),
            srem: Emitter::imm(0),
            utf_pending: Emitter::imm(0),
            utf_cp: Emitter::imm(0),
            utf_min: Emitter::imm(0),
            cum_name: Emitter::imm(0),
            cum_tmpl: Emitter::imm(0),
            cum_nodes: Emitter::imm(0),
            const_count: Emitter::imm(0),
            const_row_index: Emitter::imm(0),
            const_capture_active: Emitter::imm(0),
            const_found: Emitter::imm(0),
            const_cap_0: Emitter::imm(0),
            const_cap_1: Emitter::imm(0),
            const_cap_2: Emitter::imm(0),
            const_cap_3: Emitter::imm(0),
            const_cap_4: Emitter::imm(0),
            const_cap_5: Emitter::imm(0),
            types_count: Emitter::imm(0),
            type_row_index: Emitter::imm(0),
            type_capture_active: Emitter::imm(0),
            type_found: Emitter::imm(0),
            type_ctor_start: Emitter::imm(0),
            type_ctor_len: Emitter::imm(0),
            ctor_row_index: Emitter::imm(0),
            ctor_capture_active: Emitter::imm(0),
            ctor_found: Emitter::imm(0),
            ctor_owning_type: Emitter::imm(0),
            ctor_field_start: Emitter::imm(0),
            ctor_field_len: Emitter::imm(0),
            ctor_fields_count: Emitter::imm(0),
        };
        for lane in &inputs.pre_prog {
            em.cbits(lane.clone(), LANE_BITS);
            regs = em.step(&gate, lane.clone(), regs, &inputs.caps.phys, None, None);
        }
        em.gated_eq(gate.clone(), regs.phase, Emitter::imm(PH_END));
        let ctx = &inputs.pre_ctx;
        for lane in ctx.iter() {
            em.cbits(lane.clone(), LANE_BITS);
        }
        em.gated_eq(gate.clone(), ctx[0].clone(), Emitter::imm(C6_PHYS_MAGIC));
        em.gated_eq(gate.clone(), ctx[1].clone(), Emitter::imm(C6_PHYS_VERSION));
        em.gated_eq(gate.clone(), ctx[2].clone(), Emitter::imm(CTX_SECTION_TAG));
        em.gated_bool(gate.clone(), ctx[3].clone());
        let is_present = em.teq(ctx[3].clone(), Emitter::imm(1));
        let is_absent = em.teq(ctx[3].clone(), Emitter::imm(0));
        let absent_gate = em.mul(gate.clone(), is_absent);
        for lane in ctx.iter().take(20).skip(4) {
            em.gated_zero(absent_gate.clone(), lane.clone());
        }
        let present_gate = em.mul(gate.clone(), is_present);
        for lane in ctx.iter().take(18).skip(4) {
            em.check_usize(present_gate.clone(), lane.clone());
        }
        em.check_usize(present_gate.clone(), ctx[19].clone());
        em.check_usize(gate.clone(), ctx[20].clone());
        em.check_usize(gate.clone(), ctx[21].clone());
        self.next_temp = em.next_temp;
        Ok(C6ProgramContextCanonicalBlock {
            instructions: em.instructions,
        })
    }
    pub fn emit_prog_const_capture(
        &mut self,
        inputs: &C6ProgramContextConstCaptureInputs,
    ) -> Result<C6ProgramContextConstCaptureBlock, C6ProgramContextCanonicalError> {
        if (LANE_BITS as usize) >= FR_BITS {
            return Err(C6ProgramContextCanonicalError::InvalidBitWidth { bits: LANE_BITS });
        }
        reject_reserved(&self.scope, &inputs.gate)?;
        reject_reserved(&self.scope, &inputs.selected_index)?;
        for lane in &inputs.pre_prog {
            reject_reserved(&self.scope, lane)?;
        }
        if inputs.pre_prog.len() < PROGRAM_HEADER_LANES {
            return Err(C6ProgramContextCanonicalError::LengthMismatch {
                expected: PROGRAM_HEADER_LANES,
                actual: inputs.pre_prog.len(),
            });
        }
        let p = &inputs.caps.phys;
        for cap in [
            p.max_expressions,
            p.max_names,
            p.max_arms,
            p.max_children,
            p.max_functions,
            p.max_arity,
            p.max_types,
            p.max_occurrences,
            p.max_occurrence_paths,
            p.max_parameters,
            p.max_patterns,
            p.max_pattern_children,
            p.max_constructors,
            p.max_ctor_fields,
            p.max_decl_templates,
            p.max_plan_nodes,
            p.max_plan_cases,
            p.max_field_paths,
            p.max_constants,
            p.max_constant_fields,
            p.max_name_bytes,
            p.max_type_expr_nodes,
            p.max_template_bytes,
        ] {
            if cap == usize::MAX {
                return Err(C6ProgramContextCanonicalError::GeometryOverflow);
            }
        }
        let mut em = Emitter::new(&self.scope, self.next_temp);
        let gate = inputs.gate.clone();
        em.cbool(gate.clone());
        let mut regs = Regs {
            phase: Emitter::imm(0),
            sub: Emitter::imm(0),
            hold: Emitter::imm(0),
            opt: Emitter::imm(0),
            rem: Emitter::imm(0),
            cnt: Emitter::imm(0),
            cnt2: Emitter::imm(0),
            cnt3: Emitter::imm(0),
            te_need: Emitter::imm(0),
            srem: Emitter::imm(0),
            utf_pending: Emitter::imm(0),
            utf_cp: Emitter::imm(0),
            utf_min: Emitter::imm(0),
            cum_name: Emitter::imm(0),
            cum_tmpl: Emitter::imm(0),
            cum_nodes: Emitter::imm(0),
            const_count: Emitter::imm(0),
            const_row_index: Emitter::imm(0),
            const_capture_active: Emitter::imm(0),
            const_found: Emitter::imm(0),
            const_cap_0: Emitter::imm(0),
            const_cap_1: Emitter::imm(0),
            const_cap_2: Emitter::imm(0),
            const_cap_3: Emitter::imm(0),
            const_cap_4: Emitter::imm(0),
            const_cap_5: Emitter::imm(0),
            types_count: Emitter::imm(0),
            type_row_index: Emitter::imm(0),
            type_capture_active: Emitter::imm(0),
            type_found: Emitter::imm(0),
            type_ctor_start: Emitter::imm(0),
            type_ctor_len: Emitter::imm(0),
            ctor_row_index: Emitter::imm(0),
            ctor_capture_active: Emitter::imm(0),
            ctor_found: Emitter::imm(0),
            ctor_owning_type: Emitter::imm(0),
            ctor_field_start: Emitter::imm(0),
            ctor_field_len: Emitter::imm(0),
            ctor_fields_count: Emitter::imm(0),
        };
        for lane in &inputs.pre_prog {
            em.cbits(lane.clone(), LANE_BITS);
            regs = em.step(
                &gate,
                lane.clone(),
                regs,
                &inputs.caps.phys,
                Some(&inputs.selected_index),
                None,
            );
        }
        em.gated_eq(gate.clone(), regs.phase.clone(), Emitter::imm(PH_END));
        self.next_temp = em.next_temp;
        Ok(C6ProgramContextConstCaptureBlock {
            instructions: em.instructions,
            const_count: regs.const_count.clone(),
            selected_found: regs.const_found.clone(),
            row: [
                regs.const_cap_0.clone(),
                regs.const_cap_1.clone(),
                regs.const_cap_2.clone(),
                regs.const_cap_3.clone(),
                regs.const_cap_4.clone(),
                regs.const_cap_5.clone(),
            ],
        })
    }
    pub fn emit_prog_expression_capture(
        &mut self,
        inputs: &C6ProgramContextExpressionCaptureInputs,
    ) -> Result<C6ProgramContextExpressionCaptureBlock, C6ProgramContextCanonicalError> {
        if (LANE_BITS as usize) >= FR_BITS {
            return Err(C6ProgramContextCanonicalError::InvalidBitWidth { bits: LANE_BITS });
        }
        reject_reserved(&self.scope, &inputs.gate)?;
        reject_reserved(&self.scope, &inputs.selected_index)?;
        for lane in &inputs.pre_prog {
            reject_reserved(&self.scope, lane)?;
        }
        if inputs.pre_prog.len() < PROGRAM_HEADER_LANES {
            return Err(C6ProgramContextCanonicalError::LengthMismatch {
                expected: PROGRAM_HEADER_LANES,
                actual: inputs.pre_prog.len(),
            });
        }
        let p = &inputs.caps.phys;
        for cap in [
            p.max_expressions,
            p.max_names,
            p.max_arms,
            p.max_children,
            p.max_functions,
            p.max_arity,
            p.max_types,
            p.max_occurrences,
            p.max_occurrence_paths,
            p.max_parameters,
            p.max_patterns,
            p.max_pattern_children,
            p.max_constructors,
            p.max_ctor_fields,
            p.max_decl_templates,
            p.max_plan_nodes,
            p.max_plan_cases,
            p.max_field_paths,
            p.max_constants,
            p.max_constant_fields,
            p.max_name_bytes,
            p.max_type_expr_nodes,
            p.max_template_bytes,
        ] {
            if cap == usize::MAX {
                return Err(C6ProgramContextCanonicalError::GeometryOverflow);
            }
        }
        let mut em = Emitter::new(&self.scope, self.next_temp);
        let gate = inputs.gate.clone();
        em.cbool(gate.clone());
        let mut regs = Regs {
            phase: Emitter::imm(0),
            sub: Emitter::imm(0),
            hold: Emitter::imm(0),
            opt: Emitter::imm(0),
            rem: Emitter::imm(0),
            cnt: Emitter::imm(0),
            cnt2: Emitter::imm(0),
            cnt3: Emitter::imm(0),
            te_need: Emitter::imm(0),
            srem: Emitter::imm(0),
            utf_pending: Emitter::imm(0),
            utf_cp: Emitter::imm(0),
            utf_min: Emitter::imm(0),
            cum_name: Emitter::imm(0),
            cum_tmpl: Emitter::imm(0),
            cum_nodes: Emitter::imm(0),
            const_count: Emitter::imm(0),
            const_row_index: Emitter::imm(0),
            const_capture_active: Emitter::imm(0),
            const_found: Emitter::imm(0),
            const_cap_0: Emitter::imm(0),
            const_cap_1: Emitter::imm(0),
            const_cap_2: Emitter::imm(0),
            const_cap_3: Emitter::imm(0),
            const_cap_4: Emitter::imm(0),
            const_cap_5: Emitter::imm(0),
            types_count: Emitter::imm(0),
            type_row_index: Emitter::imm(0),
            type_capture_active: Emitter::imm(0),
            type_found: Emitter::imm(0),
            type_ctor_start: Emitter::imm(0),
            type_ctor_len: Emitter::imm(0),
            ctor_row_index: Emitter::imm(0),
            ctor_capture_active: Emitter::imm(0),
            ctor_found: Emitter::imm(0),
            ctor_owning_type: Emitter::imm(0),
            ctor_field_start: Emitter::imm(0),
            ctor_field_len: Emitter::imm(0),
            ctor_fields_count: Emitter::imm(0),
        };
        em.cbits(inputs.selected_index.clone(), LANE_BITS);
        let mut expressions_count = Emitter::imm(0);
        let mut expression_index = Emitter::imm(0);
        let mut selected_found = Emitter::imm(0);
        let mut row: [Operand; 7] = std::array::from_fn(|_| Emitter::imm(0));
        for lane in &inputs.pre_prog {
            em.cbits(lane.clone(), LANE_BITS);
            let count_phase = em.eqv(regs.phase.clone(), PH_EXPR_COUNT);
            expressions_count = em.sel(count_phase, lane.clone(), expressions_count);
            let row_phase = em.eqv(regs.phase.clone(), PH_EXPR_ROW);
            let index_eq = em.teq(expression_index.clone(), inputs.selected_index.clone());
            let selected = em.and(row_phase.clone(), index_eq);
            selected_found = em.sel(selected.clone(), Emitter::imm(1), selected_found);
            for (column, value) in row.iter_mut().enumerate() {
                let at_column = em.eqv(regs.sub.clone(), column as u128);
                let capture = em.and(selected.clone(), at_column);
                *value = em.sel(capture, lane.clone(), value.clone());
            }
            // Every expression uses at least two lanes, so a tag lane cannot
            // complete a row using the preceding row's retained tag.
            let used = em.used_from_hold(&regs, &[2, 2, 4, 4, 3, 4, 4, 6, 7]);
            let last_sub = em.add_i(used, -1);
            let at_end = em.teq(regs.sub.clone(), last_sub);
            let row_done = em.and(row_phase, at_end);
            let incremented = em.add_i(expression_index.clone(), 1);
            expression_index = em.sel(row_done, incremented, expression_index);
            regs = em.step(&gate, lane.clone(), regs, &inputs.caps.phys, None, None);
        }
        em.gated_eq(gate, regs.phase, Emitter::imm(PH_END));
        self.next_temp = em.next_temp;
        Ok(C6ProgramContextExpressionCaptureBlock {
            instructions: em.instructions,
            expressions_count,
            selected_found,
            row,
        })
    }
    pub fn emit_prog_checked_sort_capture(
        &mut self,
        inputs: &C6ProgramContextCheckedSortCaptureInputs,
    ) -> Result<C6ProgramContextCheckedSortCaptureBlock, C6ProgramContextCanonicalError> {
        if (LANE_BITS as usize) >= FR_BITS {
            return Err(C6ProgramContextCanonicalError::InvalidBitWidth { bits: LANE_BITS });
        }
        reject_reserved(&self.scope, &inputs.gate)?;
        reject_reserved(&self.scope, &inputs.selected_type_id)?;
        reject_reserved(&self.scope, &inputs.selected_ctor_id)?;
        for lane in &inputs.pre_prog {
            reject_reserved(&self.scope, lane)?;
        }
        if inputs.pre_prog.len() < PROGRAM_HEADER_LANES {
            return Err(C6ProgramContextCanonicalError::LengthMismatch {
                expected: PROGRAM_HEADER_LANES,
                actual: inputs.pre_prog.len(),
            });
        }
        let p = &inputs.caps.phys;
        for cap in [
            p.max_expressions,
            p.max_names,
            p.max_arms,
            p.max_children,
            p.max_functions,
            p.max_arity,
            p.max_types,
            p.max_occurrences,
            p.max_occurrence_paths,
            p.max_parameters,
            p.max_patterns,
            p.max_pattern_children,
            p.max_constructors,
            p.max_ctor_fields,
            p.max_decl_templates,
            p.max_plan_nodes,
            p.max_plan_cases,
            p.max_field_paths,
            p.max_constants,
            p.max_constant_fields,
            p.max_name_bytes,
            p.max_type_expr_nodes,
            p.max_template_bytes,
        ] {
            if cap == usize::MAX {
                return Err(C6ProgramContextCanonicalError::GeometryOverflow);
            }
        }
        let mut em = Emitter::new(&self.scope, self.next_temp);
        let gate = inputs.gate.clone();
        em.cbool(gate.clone());
        let mut regs = Regs {
            phase: Emitter::imm(0),
            sub: Emitter::imm(0),
            hold: Emitter::imm(0),
            opt: Emitter::imm(0),
            rem: Emitter::imm(0),
            cnt: Emitter::imm(0),
            cnt2: Emitter::imm(0),
            cnt3: Emitter::imm(0),
            te_need: Emitter::imm(0),
            srem: Emitter::imm(0),
            utf_pending: Emitter::imm(0),
            utf_cp: Emitter::imm(0),
            utf_min: Emitter::imm(0),
            cum_name: Emitter::imm(0),
            cum_tmpl: Emitter::imm(0),
            cum_nodes: Emitter::imm(0),
            const_count: Emitter::imm(0),
            const_row_index: Emitter::imm(0),
            const_capture_active: Emitter::imm(0),
            const_found: Emitter::imm(0),
            const_cap_0: Emitter::imm(0),
            const_cap_1: Emitter::imm(0),
            const_cap_2: Emitter::imm(0),
            const_cap_3: Emitter::imm(0),
            const_cap_4: Emitter::imm(0),
            const_cap_5: Emitter::imm(0),
            types_count: Emitter::imm(0),
            type_row_index: Emitter::imm(0),
            type_capture_active: Emitter::imm(0),
            type_found: Emitter::imm(0),
            type_ctor_start: Emitter::imm(0),
            type_ctor_len: Emitter::imm(0),
            ctor_row_index: Emitter::imm(0),
            ctor_capture_active: Emitter::imm(0),
            ctor_found: Emitter::imm(0),
            ctor_owning_type: Emitter::imm(0),
            ctor_field_start: Emitter::imm(0),
            ctor_field_len: Emitter::imm(0),
            ctor_fields_count: Emitter::imm(0),
        };
        for lane in &inputs.pre_prog {
            em.cbits(lane.clone(), LANE_BITS);
            regs = em.step(
                &gate,
                lane.clone(),
                regs,
                &inputs.caps.phys,
                None,
                Some((&inputs.selected_type_id, &inputs.selected_ctor_id)),
            );
        }
        em.gated_eq(gate.clone(), regs.phase.clone(), Emitter::imm(PH_END));
        self.next_temp = em.next_temp;
        Ok(C6ProgramContextCheckedSortCaptureBlock {
            instructions: em.instructions,
            type_count: regs.types_count.clone(),
            type_found: regs.type_found.clone(),
            ctor_found: regs.ctor_found.clone(),
            owning_type: regs.ctor_owning_type.clone(),
            ctor_start: regs.type_ctor_start.clone(),
            ctor_len: regs.type_ctor_len.clone(),
            field_start: regs.ctor_field_start.clone(),
            field_len: regs.ctor_field_len.clone(),
            ctor_field_count: regs.ctor_fields_count.clone(),
        })
    }
    pub fn emit_ctx_binding(
        &mut self,
        inputs: &C6ProgramContextCtxBindingInputs,
    ) -> Result<C6ProgramContextCanonicalBlock, C6ProgramContextCanonicalError> {
        if (LANE_BITS as usize) >= FR_BITS {
            return Err(C6ProgramContextCanonicalError::InvalidBitWidth { bits: LANE_BITS });
        }
        reject_reserved(&self.scope, &inputs.gate)?;
        for lane in &inputs.pre_ctx {
            reject_reserved(&self.scope, lane)?;
        }
        let mut em = Emitter::new(&self.scope, self.next_temp);
        em.cbool(inputs.gate.clone());
        for (lane, expected) in inputs.pre_ctx.iter().zip(inputs.expected_ctx.iter()) {
            em.gated_eq(inputs.gate.clone(), lane.clone(), Emitter::imm(*expected));
        }
        self.next_temp = em.next_temp;
        Ok(C6ProgramContextCanonicalBlock {
            instructions: em.instructions,
        })
    }
}
pub fn emit_c6_program_context_canonical_block(
    inputs: &C6ProgramContextCanonicalInputs,
    scope: &str,
) -> Result<C6ProgramContextCanonicalBlock, C6ProgramContextCanonicalError> {
    let mut e = C6ProgramContextCanonicalEmitter::new(scope)?;
    e.emit(inputs)
}
pub fn emit_c6_program_context_ctx_binding_block(
    inputs: &C6ProgramContextCtxBindingInputs,
    scope: &str,
) -> Result<C6ProgramContextCanonicalBlock, C6ProgramContextCanonicalError> {
    let mut e = C6ProgramContextCanonicalEmitter::new(scope)?;
    e.emit_ctx_binding(inputs)
}
