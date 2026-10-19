// Target: crates/mori-backend/src/sherlock_machine/b_return_right_completion.rs

//! Handwritten `midnight_zkir_v3::Instruction` circuit emitters for the
//! legacy completion transitions immediately following `b.helper.Scalar`
//! exit/failure on the `BinaryOp::Add` path:
//!
//! 1. `k.machine.return.right` (`rows-r4.json` `/legacy_completion_instances/15`,
//!    `T-LEGACY`):
//!    - Selected at `site = RetRight` (`11`), `pre.outcome = Running` (`0`),
//!      `pre.helper` absent, `pre.control = ControlRow::Return { value }`,
//!      `pre.depth >= 1`, top frame `pre.frames[pre.depth - 1]` equal to
//!      `FrameRow::RightF { op: BinaryOp::Add, left, origin }`, and `pre.result`
//!      present with `ScalarValue(value)` or `ScalarError(10 | 11)`.
//!    - Effects:
//!      * `framePop`: decrements `depth` (`post.depth = pre.depth - 1`),
//!        preserves active lower frames `0 .. post.depth`, zeroes the popped
//!        slot `post.depth`, and enforces `zeroRow(FrameRow)` on all inactive
//!        slots `>= pre.depth` (and `>= post.depth` in `post`).
//!      * `reg.control`: writes `ControlRow::PrimResult` with
//!        `PrimOutcome::Value { value: r.value }` when `r` is `ScalarValue` or
//!        `PrimOutcome::Error { code: r.code }` when `r` is `ScalarError(10 | 11)`,
//!        zeroing all inactive `ControlRow` union columns.
//!      * `reg.controlOrigin`: sets `post.control_origin = top.origin`.
//!      * `clearResult`: zeroes `post.result.present` and every `ResultRow`
//!        payload column.
//!      * `T-LEGACY` counter deltas `[-1, +1, 0, 0, +1, 0, 0, 0, -1, 0, 0]`:
//!        `remaining_work -= 1`, `consumed_work += 1`, `reference_cursor += 1`,
//!        `depth -= 1`; preserves `helper_work`, `helper_visits`, `outcome`,
//!        `helper` absence, and all row-complement state columns (`rest`).
//!
//! 2. `k.machine.primitive.value` (`rows-r4.json` `/legacy_completion_instances/35`,
//!    `T-LEGACY`):
//!    - Selected at `site = Prim` (`23`), `pre.outcome = Running` (`0`),
//!      `pre.helper` absent, `pre.result` absent, and
//!      `pre.control = ControlRow::PrimResult { result: PrimOutcome::Value { value } }`.
//!    - Effects:
//!      * `reg.control`: writes `ControlRow::Return { value }`, zeroing all
//!        `PrimResult` and other inactive `ControlRow` union columns.
//!      * Preserves `control_origin`, `depth`, active frames `< pre.depth`
//!        (with inactive slots `>= pre.depth` constrained to `zeroRow(FrameRow)`),
//!        `outcome = Running`, `helper` absence, `result` absence, `helper_work`,
//!        `helper_visits`, and all row-complement state columns (`rest`).
//!      * `T-LEGACY` counter deltas `[-1, +1, 0, 0, +1, 0, 0, 0, 0, 0, 0]`:
//!        `remaining_work -= 1`, `consumed_work += 1`, `reference_cursor += 1`,
//!        `depth` unchanged.
//!
//! 3. `k.machine.primitive.error` (`rows-r4.json` `/legacy_completion_instances/36`,
//!    `T-LEGACY`):
//!    - Selected at `site = Prim` (`23`), `pre.outcome = Running` (`0`),
//!      `pre.helper` absent, `pre.result` absent, and
//!      `pre.control = ControlRow::PrimResult { result: PrimOutcome::Error { code } }`
//!      with `code in {10, 11}`.
//!    - Effects:
//!      * `reg.control`: writes `ControlRow::Error { code }`, zeroing all
//!        `PrimResult` and other inactive `ControlRow` union columns.
//!      * Preserves `control_origin`, `depth`, active frames `< pre.depth`
//!        (with inactive slots `>= pre.depth` constrained to `zeroRow(FrameRow)`),
//!        `outcome = Running` (terminal `Outcome::Rejected` publication belongs
//!        to separate 0-charge `P1` row `k.admin.publish.error`), `helper`
//!        absence, `result` absence, `helper_work`, `helper_visits`, and all
//!        row-complement state columns (`rest`).
//!      * `T-LEGACY` counter deltas `[-1, +1, 0, 0, +1, 0, 0, 0, 0, 0, 0]`:
//!        `remaining_work -= 1`, `consumed_work += 1`, `reference_cursor += 1`,
//!        `depth` unchanged.

use super::b_scalar_arithmetic::{
    BScalarArithmeticRegister, BU128Lanes, BValueRef, PRIM_ERROR_ARITH_OVERFLOW,
    PRIM_ERROR_SORT_MISMATCH, SITE_RET_RIGHT,
};
use super::b_scalar_completion::{
    BHelperResultLanes, HELPER_RESULT_TAG_SCALAR_ERROR, HELPER_RESULT_TAG_SCALAR_VALUE,
    OUTCOME_TAG_RUNNING, VALUE_SPACE_CONST, VALUE_SPACE_DATA,
};
use super::*;

// ---------------------------------------------------------------------------
// Schema enum ordinals (0-based in NEXT-SCHEMA.compact.json declaration order)
// ---------------------------------------------------------------------------

/// `SiteTag::Prim` (`enums.SiteTag[23] == "Prim"`).
pub const SITE_PRIM: u128 = 23;

/// `FrameTag` 0-based ordinals (`enums.FrameTag`).
pub const FRAME_TAG_LET_F: u128 = 0;
pub const FRAME_TAG_LEFT_F: u128 = 1;
pub const FRAME_TAG_RIGHT_F: u128 = 2;
pub const FRAME_TAG_UNARY_F: u128 = 3;
pub const FRAME_TAG_IF_F: u128 = 4;
pub const FRAME_TAG_MATCH_F: u128 = 5;
pub const FRAME_TAG_ARGS_F: u128 = 6;
pub const FRAME_TAG_RETURN_F: u128 = 7;
pub const FRAME_TAG_SCOPE_F: u128 = 8;
pub const FRAME_TAG_CONSTRUCTOR_FIELDS_EXTENSION_RESERVED: u128 = 9;

/// `BinaryOp` 0-based ordinals (`enums.BinaryOp`).
pub const BINARY_OP_ADD: u128 = 0;
pub const BINARY_OP_SUB: u128 = 1;
pub const BINARY_OP_MUL: u128 = 2;
pub const BINARY_OP_EQ: u128 = 3;
pub const BINARY_OP_LT: u128 = 4;
pub const BINARY_OP_CONS: u128 = 5;
pub const BINARY_OP_BRANCH: u128 = 6;

/// `ControlTag` 0-based ordinals (`enums.ControlTag`).
pub const CONTROL_TAG_EVAL: u128 = 0;
pub const CONTROL_TAG_RETURN: u128 = 1;
pub const CONTROL_TAG_SCHEDULE_NEXT: u128 = 2;
pub const CONTROL_TAG_INVOKE: u128 = 3;
pub const CONTROL_TAG_ENTER: u128 = 4;
pub const CONTROL_TAG_BOUND: u128 = 5;
pub const CONTROL_TAG_CASES: u128 = 6;
pub const CONTROL_TAG_CASE_RESULT: u128 = 7;
pub const CONTROL_TAG_PRIM_RESULT: u128 = 8;
pub const CONTROL_TAG_ERROR: u128 = 9;

/// `PrimOutcomeTag` 0-based ordinals (`enums.PrimOutcomeTag`: `Value = 0, Error = 1`).
pub const PRIM_OUTCOME_TAG_VALUE: u128 = 0;
pub const PRIM_OUTCOME_TAG_ERROR: u128 = 1;

// ---------------------------------------------------------------------------
// Carriers for FrameRow, ControlRow, and Legacy Completion State
// ---------------------------------------------------------------------------

/// Logical carrier for one `img.frames[i]` (`FrameRow`) entry.
///
/// Under `zeroRow(FrameRow)`, `tag`, `origin`, `right_op`, `right_left`, and
/// every column in `rest` are `0`. Under `FrameRow::RightF { op, left, origin }`,
/// `tag == FRAME_TAG_RIGHT_F`, `right_op == op`, `right_left == left`,
/// `origin == origin`, and all `rest` columns are `0`.
#[derive(Clone, Debug, PartialEq)]
pub struct BFrameRow {
    pub tag: Operand,
    pub origin: Operand,
    pub right_op: Operand,
    pub right_left: BValueRef,
    pub rest: Vec<Operand>,
}

/// Logical carrier for `reg.control` (`ControlRow`).
///
/// Active-union rules (`NEXT-SCHEMA.compact.json`):
/// - `ControlRow::Return { value }`:
///   `tag == CONTROL_TAG_RETURN`, `return_value == value`, and
///   `prim_outcome_tag`, `prim_value`, `prim_error_code`, `error_code`, `rest` all `0`.
/// - `ControlRow::PrimResult { result: PrimOutcome::Value { value } }`:
///   `tag == CONTROL_TAG_PRIM_RESULT`, `prim_outcome_tag == PRIM_OUTCOME_TAG_VALUE`,
///   `prim_value == value`, and `return_value`, `prim_error_code`, `error_code`, `rest` all `0`.
/// - `ControlRow::PrimResult { result: PrimOutcome::Error { code } }`:
///   `tag == CONTROL_TAG_PRIM_RESULT`, `prim_outcome_tag == PRIM_OUTCOME_TAG_ERROR`,
///   `prim_error_code == code`, and `return_value`, `prim_value`, `error_code`, `rest` all `0`.
/// - `ControlRow::Error { code }`:
///   `tag == CONTROL_TAG_ERROR`, `error_code == code`, and
///   `return_value`, `prim_outcome_tag`, `prim_value`, `prim_error_code`, `rest` all `0`.
#[derive(Clone, Debug, PartialEq)]
pub struct BControlRow {
    pub tag: Operand,
    pub return_value: BValueRef,
    pub prim_outcome_tag: Operand,
    pub prim_value: BValueRef,
    pub prim_error_code: Operand,
    pub error_code: Operand,
    pub rest: Vec<Operand>,
}

/// Logical pre/post state carrier for `k.machine.return.right`,
/// `k.machine.primitive.value`, and `k.machine.primitive.error`.
#[derive(Clone, Debug, PartialEq)]
pub struct BLegacyCompletionState {
    pub site: Operand,
    pub outcome: Operand,
    pub control: BControlRow,
    pub control_origin: Operand,
    pub helper: BScalarArithmeticRegister,
    pub result: BHelperResultLanes,
    pub frames: Vec<BFrameRow>,
    pub depth: Operand,
    pub remaining_work: BU128Lanes,
    pub consumed_work: BU128Lanes,
    pub helper_work: BU128Lanes,
    pub helper_visits: BU128Lanes,
    pub reference_cursor: BU128Lanes,
    pub rest: Vec<Operand>,
}

/// Structural validation errors before emitting ZKIR instructions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BReturnRightCompletionEmitError {
    EmptyFrameCapacity,
    FrameCapacityMismatch {
        pre_len: usize,
        post_len: usize,
    },
    FrameRestLengthMismatch {
        slot: usize,
        expected: usize,
        actual: usize,
    },
    ControlRestLengthMismatch {
        pre_len: usize,
        post_len: usize,
    },
    HelperRestLengthMismatch {
        pre_len: usize,
        post_len: usize,
    },
    ResultRestLengthMismatch {
        pre_len: usize,
        post_len: usize,
    },
    StateRestLengthMismatch {
        pre_len: usize,
        post_len: usize,
    },
    /// Scope is empty, non `[A-Za-z0-9_]`, contains `__`, or starts/ends with `_`.
    InvalidScope,
    /// A caller `Operand::Variable` lacks the leading `%` that SDK serde `Operand` decoding requires.
    UnprefixedVariable,
    /// A caller `Operand::Variable` lies in this block's reserved `%{scope}__` output namespace.
    TemporaryNameCollision,
}

/// Enabled-row successor contract for `post.site`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BPostSiteContract {
    /// `post.site == tag` constrained in-row when `enable == 1`.
    Bound(u128),
    /// Post control `Return`: `post.site` unconstrained in-row; caller dispatch must bind it
    /// by `/dispatch/site` on the top frame if `post.depth > 0`, else select P1 (no SiteTag).
    ReturnDispatch,
    /// Post control `Error`: successor is P1 `k.admin.publish.error` (no SiteTag);
    /// `post.site` unconstrained in-row and must not feed site dispatch.
    TerminalP1,
}

/// Scoped fragment: all outputs are `%{scope}__*` (SDK serde variable form); every caller
/// `Variable` must be `%`-prefixed and outside `%{scope}__` (checked). `scope` must be unique per
/// `IrSource` and no other block there may emit into `%{scope}__*` (caller premise, unchecked).
/// `enable == 0` still forces `post == pre` on every lane, so sibling rows MUST NOT share
/// one `post`: each row takes its own post (a disabled row may pass `pre`) and the caller
/// muxes the exactly-one enabled row's post.
#[derive(Clone, Debug, PartialEq)]
pub struct BLegacyCompletionBlock {
    pub instructions: Vec<Instruction>,
    pub post_site: BPostSiteContract,
}

fn validate_scope(scope: &str) -> Result<(), BReturnRightCompletionEmitError> {
    let ok = !scope.is_empty()
        && !scope.starts_with('_')
        && !scope.ends_with('_')
        && !scope.contains("__")
        && scope.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_');
    ok.then_some(()).ok_or(BReturnRightCompletionEmitError::InvalidScope)
}

/// Reserved-name predicate for outputs of a block emitted under `scope`: exactly `%{scope}__*`.
pub fn is_legacy_completion_scope_variable(scope: &str, name: &str) -> bool {
    name.strip_prefix('%')
        .and_then(|r| r.strip_prefix(scope))
        .is_some_and(|r| r.starts_with("__"))
}

// Standalone forms use a fixed scope: at most one call each per `IrSource`; they carry
// the same `post_site` contract as their `_block` form.

pub fn emit_k_machine_return_right_add(
    enable: Operand,
    pre: &BLegacyCompletionState,
    post: &BLegacyCompletionState,
) -> Result<Vec<Instruction>, BReturnRightCompletionEmitError> {
    emit_return_right_add_scoped(enable, pre, post, "k_ret_right_add")
}

pub fn emit_k_machine_return_right_add_block(
    enable: Operand,
    pre: &BLegacyCompletionState,
    post: &BLegacyCompletionState,
    scope: &str,
) -> Result<BLegacyCompletionBlock, BReturnRightCompletionEmitError> {
    let instructions = emit_return_right_add_scoped(enable, pre, post, scope)?;
    Ok(BLegacyCompletionBlock { instructions, post_site: BPostSiteContract::Bound(SITE_PRIM) })
}

pub fn emit_k_machine_primitive_value(
    enable: Operand,
    pre: &BLegacyCompletionState,
    post: &BLegacyCompletionState,
) -> Result<Vec<Instruction>, BReturnRightCompletionEmitError> {
    emit_primitive_value_scoped(enable, pre, post, "k_prim_val")
}

pub fn emit_k_machine_primitive_value_block(
    enable: Operand,
    pre: &BLegacyCompletionState,
    post: &BLegacyCompletionState,
    scope: &str,
) -> Result<BLegacyCompletionBlock, BReturnRightCompletionEmitError> {
    let instructions = emit_primitive_value_scoped(enable, pre, post, scope)?;
    Ok(BLegacyCompletionBlock { instructions, post_site: BPostSiteContract::ReturnDispatch })
}

pub fn emit_k_machine_primitive_error(
    enable: Operand,
    pre: &BLegacyCompletionState,
    post: &BLegacyCompletionState,
) -> Result<Vec<Instruction>, BReturnRightCompletionEmitError> {
    emit_primitive_error_scoped(enable, pre, post, "k_prim_err")
}

pub fn emit_k_machine_primitive_error_block(
    enable: Operand,
    pre: &BLegacyCompletionState,
    post: &BLegacyCompletionState,
    scope: &str,
) -> Result<BLegacyCompletionBlock, BReturnRightCompletionEmitError> {
    let instructions = emit_primitive_error_scoped(enable, pre, post, scope)?;
    Ok(BLegacyCompletionBlock { instructions, post_site: BPostSiteContract::TerminalP1 })
}

// ---------------------------------------------------------------------------
// Internal ZKIR emitter builder
// ---------------------------------------------------------------------------

struct Emitter {
    prefix: String,
    counter: usize,
    instructions: Vec<Instruction>,
}

impl Emitter {
    fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
            counter: 0,
            instructions: Vec::new(),
        }
    }

    fn fresh(&mut self, tag: &str) -> Identifier {
        let id = format!("%{}__{}_{}", self.prefix, tag, self.counter);
        self.counter += 1;
        Identifier(id)
    }

    fn load_imm_u64(&mut self, value: u128, _tag: &str) -> Operand {
        Operand::Immediate(Fr::from(value))
    }

    fn add(&mut self, a: Operand, b: Operand, tag: &str) -> Operand {
        let out = self.fresh(tag);
        self.instructions.push(Instruction::Add {
            a,
            b,
            output: out.clone(),
        });
        Operand::Variable(out)
    }

    fn mul(&mut self, a: Operand, b: Operand, tag: &str) -> Operand {
        let out = self.fresh(tag);
        self.instructions.push(Instruction::Mul {
            a,
            b,
            output: out.clone(),
        });
        Operand::Variable(out)
    }

    fn is_equal(&mut self, a: Operand, b: Operand, tag: &str) -> Operand {
        let out = self.fresh(tag);
        self.instructions.push(Instruction::TestEq {
            a,
            b,
            output: out.clone(),
        });
        Operand::Variable(out)
    }

    fn div_mod_pow2(&mut self, val: Operand, bits: u32, tag: &str) -> (Operand, Operand) {
        let q = self.fresh(&format!("{}_q", tag));
        let r = self.fresh(&format!("{}_r", tag));
        self.instructions.push(Instruction::DivModPowerOfTwo {
            val,
            bits,
            outputs: vec![q.clone(), r.clone()],
        });
        (Operand::Variable(q), Operand::Variable(r))
    }

    fn range_check(&mut self, val: Operand, bits: u32) {
        self.instructions
            .push(Instruction::ConstrainBits { val, bits });
    }

    fn constrain_eq(&mut self, a: Operand, b: Operand) {
        self.instructions.push(Instruction::ConstrainEq { a, b });
    }

    fn assert_bit(&mut self, bit: Operand, _one: Operand, _zero: Operand, _tag: &str) {
        self.instructions
            .push(Instruction::ConstrainToBoolean { val: bit });
    }

    fn assert_eq_always(&mut self, a: Operand, b: Operand, _tag: &str) {
        self.constrain_eq(a, b);
    }

    fn assert_eq_when(
        &mut self,
        gate: Operand,
        a: Operand,
        b: Operand,
        _zero: Operand,
        tag: &str,
    ) {
        let lhs = self.mul(gate.clone(), a, &format!("{}_lhs", tag));
        let rhs = self.mul(gate, b, &format!("{}_rhs", tag));
        self.constrain_eq(lhs, rhs);
    }

    fn assert_zero_when(&mut self, gate: Operand, val: Operand, zero: Operand, tag: &str) {
        let gated = self.mul(gate, val, &format!("{}_gated", tag));
        self.constrain_eq(gated, zero);
    }

    fn select(
        &mut self,
        cond: Operand,
        when_true: Operand,
        when_false: Operand,
        tag: &str,
    ) -> Operand {
        let out = self.fresh(tag);
        self.instructions.push(Instruction::CondSelect {
            bit: cond,
            a: when_true,
            b: when_false,
            output: out.clone(),
        });
        Operand::Variable(out)
    }

    fn assert_u128_when(&mut self, gate: Operand, val: Operand, zero: Operand, tag: &str) {
        let gated = self.select(gate, val, zero, &format!("{}_u128_eff", tag));
        self.range_check(gated, 128);
    }

    fn assert_value_space_when(
        &mut self,
        gate: Operand,
        space: Operand,
        c_const: Operand,
        c_data: Operand,
        zero: Operand,
        tag: &str,
    ) {
        let one = self.load_imm_u64(1, &format!("{}_one", tag));
        let is_const = self.is_equal(space.clone(), c_const, &format!("{}_isc", tag));
        let is_data = self.is_equal(space, c_data, &format!("{}_isd", tag));
        let sum = self.add(is_const, is_data, &format!("{}_sum", tag));
        self.assert_eq_when(gate, sum, one, zero, &format!("{}_sp", tag));
    }

    fn assert_valid_value_ref_when(
        &mut self,
        gate: Operand,
        vref: &BValueRef,
        c_const: Operand,
        c_data: Operand,
        zero: Operand,
        tag: &str,
    ) {
        self.assert_value_space_when(
            gate.clone(),
            vref.space.clone(),
            c_const,
            c_data,
            zero.clone(),
            &format!("{}_space", tag),
        );
        self.assert_u128_when(gate, vref.index.clone(), zero, &format!("{}_index", tag));
    }

    fn assert_value_ref_zero_when(
        &mut self,
        gate: Operand,
        vref: &BValueRef,
        zero: Operand,
        tag: &str,
    ) {
        self.assert_zero_when(
            gate.clone(),
            vref.space.clone(),
            zero.clone(),
            &format!("{}_space_z", tag),
        );
        self.assert_zero_when(gate, vref.index.clone(), zero, &format!("{}_index_z", tag));
    }

    fn assert_value_ref_eq_when(
        &mut self,
        gate: Operand,
        a: &BValueRef,
        b: &BValueRef,
        zero: Operand,
        tag: &str,
    ) {
        self.assert_eq_when(
            gate.clone(),
            a.space.clone(),
            b.space.clone(),
            zero.clone(),
            &format!("{}_space_eq", tag),
        );
        self.assert_eq_when(
            gate,
            a.index.clone(),
            b.index.clone(),
            zero,
            &format!("{}_index_eq", tag),
        );
    }

    fn assert_error_code_10_or_11_when(
        &mut self,
        gate: Operand,
        code: Operand,
        c10: Operand,
        c11: Operand,
        zero: Operand,
        tag: &str,
    ) {
        let one = self.load_imm_u64(1, &format!("{}_one", tag));
        let is_10 = self.is_equal(code.clone(), c10, &format!("{}_is10", tag));
        let is_11 = self.is_equal(code, c11, &format!("{}_is11", tag));
        let sum = self.add(is_10, is_11, &format!("{}_sum", tag));
        self.assert_eq_when(gate, sum, one, zero, &format!("{}_chk", tag));
    }

    fn assert_helper_absent(
        &mut self,
        gate: Operand,
        helper: &BScalarArithmeticRegister,
        zero: Operand,
        tag: &str,
    ) {
        for (name, op) in [
            ("pres", helper.present.clone()),
            ("kind", helper.kind.clone()),
            ("site", helper.site.clone()),
            ("mode", helper.mode.clone()),
            ("ph", helper.phase.clone()),
            ("oa_sp", helper.operand_a.space.clone()),
            ("oa_ix", helper.operand_a.index.clone()),
            ("ob_pr", helper.operand_b.present.clone()),
            ("ob_sp", helper.operand_b.value.space.clone()),
            ("ob_ix", helper.operand_b.value.index.clone()),
            ("st_tag", helper.staged.tag.clone()),
            ("st_bv", helper.staged.bool_value.clone()),
            ("st_pnt", helper.staged.pending_node_tag.clone()),
            ("st_pec", helper.staged.prim_error_code.clone()),
            ("st_sel", helper.staged.selector.clone()),
            ("st_tto", helper.staged.type_test_ok.clone()),
        ] {
            self.assert_zero_when(gate.clone(), op, zero.clone(), &format!("{}_{}", tag, name));
        }
        for i in 0..16 {
            self.assert_zero_when(
                gate.clone(),
                helper.reg_a[i].clone(),
                zero.clone(),
                &format!("{}_ra_b_{}", tag, i),
            );
            self.assert_zero_when(
                gate.clone(),
                helper.staged.u128_value[i].clone(),
                zero.clone(),
                &format!("{}_st_b_{}", tag, i),
            );
        }
        for (i, col) in helper.rest.iter().enumerate() {
            self.assert_zero_when(
                gate.clone(),
                col.clone(),
                zero.clone(),
                &format!("{}_h_rest_{}", tag, i),
            );
        }
    }

    fn assert_helper_eq_when(
        &mut self,
        gate: Operand,
        a: &BScalarArithmeticRegister,
        b: &BScalarArithmeticRegister,
        zero: Operand,
        tag: &str,
    ) {
        for (name, x, y) in [
            ("pres", a.present.clone(), b.present.clone()),
            ("kind", a.kind.clone(), b.kind.clone()),
            ("site", a.site.clone(), b.site.clone()),
            ("mode", a.mode.clone(), b.mode.clone()),
            ("ph", a.phase.clone(), b.phase.clone()),
            ("oa_sp", a.operand_a.space.clone(), b.operand_a.space.clone()),
            ("oa_ix", a.operand_a.index.clone(), b.operand_a.index.clone()),
            ("ob_pr", a.operand_b.present.clone(), b.operand_b.present.clone()),
            ("ob_sp", a.operand_b.value.space.clone(), b.operand_b.value.space.clone()),
            ("ob_ix", a.operand_b.value.index.clone(), b.operand_b.value.index.clone()),
            ("st_tag", a.staged.tag.clone(), b.staged.tag.clone()),
            ("st_bv", a.staged.bool_value.clone(), b.staged.bool_value.clone()),
            ("st_pnt", a.staged.pending_node_tag.clone(), b.staged.pending_node_tag.clone()),
            ("st_pec", a.staged.prim_error_code.clone(), b.staged.prim_error_code.clone()),
            ("st_sel", a.staged.selector.clone(), b.staged.selector.clone()),
            ("st_tto", a.staged.type_test_ok.clone(), b.staged.type_test_ok.clone()),
        ] {
            self.assert_eq_when(gate.clone(), x, y, zero.clone(), &format!("{}_{}", tag, name));
        }
        for i in 0..16 {
            self.assert_eq_when(
                gate.clone(),
                a.reg_a[i].clone(),
                b.reg_a[i].clone(),
                zero.clone(),
                &format!("{}_ra_b_{}", tag, i),
            );
            self.assert_eq_when(
                gate.clone(),
                a.staged.u128_value[i].clone(),
                b.staged.u128_value[i].clone(),
                zero.clone(),
                &format!("{}_st_b_{}", tag, i),
            );
        }
        for (i, (x, y)) in a.rest.iter().zip(b.rest.iter()).enumerate() {
            self.assert_eq_when(
                gate.clone(),
                x.clone(),
                y.clone(),
                zero.clone(),
                &format!("{}_h_rest_{}", tag, i),
            );
        }
    }

    fn assert_result_absent(
        &mut self,
        gate: Operand,
        res: &BHelperResultLanes,
        zero: Operand,
        tag: &str,
    ) {
        self.assert_zero_when(
            gate.clone(),
            res.present.clone(),
            zero.clone(),
            &format!("{}_pres", tag),
        );
        self.assert_zero_when(
            gate.clone(),
            res.tag.clone(),
            zero.clone(),
            &format!("{}_tag", tag),
        );
        self.assert_value_ref_zero_when(
            gate.clone(),
            &res.scalar_value,
            zero.clone(),
            &format!("{}_val", tag),
        );
        self.assert_zero_when(
            gate.clone(),
            res.scalar_error_code.clone(),
            zero.clone(),
            &format!("{}_err", tag),
        );
        self.assert_zero_when(
            gate.clone(),
            res.scalar_selector.clone(),
            zero.clone(),
            &format!("{}_sel", tag),
        );
        self.assert_zero_when(
            gate.clone(),
            res.scalar_type_test_ok.clone(),
            zero.clone(),
            &format!("{}_tt", tag),
        );
        for (i, col) in res.rest.iter().enumerate() {
            self.assert_zero_when(
                gate.clone(),
                col.clone(),
                zero.clone(),
                &format!("{}_rest_{}", tag, i),
            );
        }
    }

    fn assert_result_eq_when(
        &mut self,
        gate: Operand,
        a: &BHelperResultLanes,
        b: &BHelperResultLanes,
        zero: Operand,
        tag: &str,
    ) {
        self.assert_eq_when(
            gate.clone(),
            a.present.clone(),
            b.present.clone(),
            zero.clone(),
            &format!("{}_pres", tag),
        );
        self.assert_eq_when(
            gate.clone(),
            a.tag.clone(),
            b.tag.clone(),
            zero.clone(),
            &format!("{}_tag", tag),
        );
        self.assert_value_ref_eq_when(
            gate.clone(),
            &a.scalar_value,
            &b.scalar_value,
            zero.clone(),
            &format!("{}_val", tag),
        );
        self.assert_eq_when(
            gate.clone(),
            a.scalar_error_code.clone(),
            b.scalar_error_code.clone(),
            zero.clone(),
            &format!("{}_err", tag),
        );
        self.assert_eq_when(
            gate.clone(),
            a.scalar_selector.clone(),
            b.scalar_selector.clone(),
            zero.clone(),
            &format!("{}_sel", tag),
        );
        self.assert_eq_when(
            gate.clone(),
            a.scalar_type_test_ok.clone(),
            b.scalar_type_test_ok.clone(),
            zero.clone(),
            &format!("{}_tt", tag),
        );
        for (i, (x, y)) in a.rest.iter().zip(b.rest.iter()).enumerate() {
            self.assert_eq_when(
                gate.clone(),
                x.clone(),
                y.clone(),
                zero.clone(),
                &format!("{}_rest_{}", tag, i),
            );
        }
    }

    fn assert_frame_zero_when(
        &mut self,
        gate: Operand,
        frame: &BFrameRow,
        zero: Operand,
        tag: &str,
    ) {
        self.assert_zero_when(
            gate.clone(),
            frame.tag.clone(),
            zero.clone(),
            &format!("{}_tag", tag),
        );
        self.assert_zero_when(
            gate.clone(),
            frame.origin.clone(),
            zero.clone(),
            &format!("{}_orig", tag),
        );
        self.assert_zero_when(
            gate.clone(),
            frame.right_op.clone(),
            zero.clone(),
            &format!("{}_rop", tag),
        );
        self.assert_value_ref_zero_when(
            gate.clone(),
            &frame.right_left,
            zero.clone(),
            &format!("{}_rleft", tag),
        );
        for (i, col) in frame.rest.iter().enumerate() {
            self.assert_zero_when(
                gate.clone(),
                col.clone(),
                zero.clone(),
                &format!("{}_rest_{}", tag, i),
            );
        }
    }

    fn assert_frame_eq_when(
        &mut self,
        gate: Operand,
        a: &BFrameRow,
        b: &BFrameRow,
        zero: Operand,
        tag: &str,
    ) {
        self.assert_eq_when(
            gate.clone(),
            a.tag.clone(),
            b.tag.clone(),
            zero.clone(),
            &format!("{}_tag", tag),
        );
        self.assert_eq_when(
            gate.clone(),
            a.origin.clone(),
            b.origin.clone(),
            zero.clone(),
            &format!("{}_orig", tag),
        );
        self.assert_eq_when(
            gate.clone(),
            a.right_op.clone(),
            b.right_op.clone(),
            zero.clone(),
            &format!("{}_rop", tag),
        );
        self.assert_value_ref_eq_when(
            gate.clone(),
            &a.right_left,
            &b.right_left,
            zero.clone(),
            &format!("{}_rleft", tag),
        );
        for (i, (x, y)) in a.rest.iter().zip(b.rest.iter()).enumerate() {
            self.assert_eq_when(
                gate.clone(),
                x.clone(),
                y.clone(),
                zero.clone(),
                &format!("{}_rest_{}", tag, i),
            );
        }
    }

    fn assert_control_eq_when(
        &mut self,
        gate: Operand,
        a: &BControlRow,
        b: &BControlRow,
        zero: Operand,
        tag: &str,
    ) {
        self.assert_eq_when(
            gate.clone(),
            a.tag.clone(),
            b.tag.clone(),
            zero.clone(),
            &format!("{}_tag", tag),
        );
        self.assert_value_ref_eq_when(
            gate.clone(),
            &a.return_value,
            &b.return_value,
            zero.clone(),
            &format!("{}_rv", tag),
        );
        self.assert_eq_when(
            gate.clone(),
            a.prim_outcome_tag.clone(),
            b.prim_outcome_tag.clone(),
            zero.clone(),
            &format!("{}_pot", tag),
        );
        self.assert_value_ref_eq_when(
            gate.clone(),
            &a.prim_value,
            &b.prim_value,
            zero.clone(),
            &format!("{}_pv", tag),
        );
        self.assert_eq_when(
            gate.clone(),
            a.prim_error_code.clone(),
            b.prim_error_code.clone(),
            zero.clone(),
            &format!("{}_pec", tag),
        );
        self.assert_eq_when(
            gate.clone(),
            a.error_code.clone(),
            b.error_code.clone(),
            zero.clone(),
            &format!("{}_ec", tag),
        );
        for (i, (x, y)) in a.rest.iter().zip(b.rest.iter()).enumerate() {
            self.assert_eq_when(
                gate.clone(),
                x.clone(),
                y.clone(),
                zero.clone(),
                &format!("{}_rest_{}", tag, i),
            );
        }
    }

    fn assert_u128_lanes_bytes(&mut self, lanes: &BU128Lanes) {
        for i in 0..16 {
            self.range_check(lanes[i].clone(), 8);
        }
    }

    fn assert_u128_lanes_eq_always(&mut self, a: &BU128Lanes, b: &BU128Lanes, tag: &str) {
        self.assert_u128_lanes_bytes(a);
        self.assert_u128_lanes_bytes(b);
        for i in 0..16 {
            self.assert_eq_always(a[i].clone(), b[i].clone(), &format!("{}_b_{}", tag, i));
        }
    }

    /// Constrains `post == pre + enable` on 16-byte LE lanes with no u128 overflow.
    fn assert_u128_inc_by_gate(
        &mut self,
        gate: Operand,
        pre: &BU128Lanes,
        post: &BU128Lanes,
        zero: Operand,
        tag: &str,
    ) {
        self.assert_u128_lanes_bytes(pre);
        self.assert_u128_lanes_bytes(post);
        let mut carry = gate;
        for i in 0..16 {
            let sum = self.add(pre[i].clone(), carry, &format!("{}_sum_{}", tag, i));
            let (q, r) = self.div_mod_pow2(sum, 8, &format!("{}_dm_{}", tag, i));
            self.assert_eq_always(r, post[i].clone(), &format!("{}_rem_{}", tag, i));
            carry = q;
        }
        self.assert_eq_always(carry, zero, &format!("{}_no_ovf", tag));
    }

    /// Constrains `pre == post + enable` (i.e. `post == pre - enable` without underflow).
    fn assert_u128_dec_by_gate(
        &mut self,
        gate: Operand,
        pre: &BU128Lanes,
        post: &BU128Lanes,
        zero: Operand,
        tag: &str,
    ) {
        self.assert_u128_inc_by_gate(gate, post, pre, zero, tag);
    }

    /// Constrains `T-LEGACY` common counter deltas (`remaining_work -= enable`,
    /// `consumed_work += enable`, `helper_work` preserved, `helper_visits`
    /// preserved, `reference_cursor += enable`, and `state.rest` preserved).
    fn emit_t_legacy_common_counters_and_rest(
        &mut self,
        enable: Operand,
        pre: &BLegacyCompletionState,
        post: &BLegacyCompletionState,
        zero: Operand,
    ) {
        self.assert_u128_dec_by_gate(
            enable.clone(),
            &pre.remaining_work,
            &post.remaining_work,
            zero.clone(),
            "rem_work",
        );
        self.assert_u128_inc_by_gate(
            enable.clone(),
            &pre.consumed_work,
            &post.consumed_work,
            zero.clone(),
            "cons_work",
        );
        self.assert_u128_lanes_eq_always(&pre.helper_work, &post.helper_work, "hlp_work");
        self.assert_u128_lanes_eq_always(&pre.helper_visits, &post.helper_visits, "hlp_visits");
        self.assert_u128_inc_by_gate(
            enable,
            &pre.reference_cursor,
            &post.reference_cursor,
            zero,
            "ref_cursor",
        );
        for (i, (x, y)) in pre.rest.iter().zip(post.rest.iter()).enumerate() {
            self.assert_eq_always(x.clone(), y.clone(), &format!("state_rest_{}", i));
        }
    }
}

// ---------------------------------------------------------------------------
// Structural validation
// ---------------------------------------------------------------------------

fn validate_state_pair(
    pre: &BLegacyCompletionState,
    post: &BLegacyCompletionState,
) -> Result<usize, BReturnRightCompletionEmitError> {
    let cap = pre.frames.len();
    // F4: `cap == 0` is admitted for `primitive.value` / `primitive.error` at `depth == 0`.
    if post.frames.len() != cap {
        return Err(BReturnRightCompletionEmitError::FrameCapacityMismatch {
            pre_len: cap,
            post_len: post.frames.len(),
        });
    }
    let expected_frame_rest = pre.frames.first().map_or(0, |f| f.rest.len());
    for (slot, (f_pre, f_post)) in pre.frames.iter().zip(post.frames.iter()).enumerate() {
        if f_pre.rest.len() != expected_frame_rest {
            return Err(BReturnRightCompletionEmitError::FrameRestLengthMismatch {
                slot,
                expected: expected_frame_rest,
                actual: f_pre.rest.len(),
            });
        }
        if f_post.rest.len() != expected_frame_rest {
            return Err(BReturnRightCompletionEmitError::FrameRestLengthMismatch {
                slot,
                expected: expected_frame_rest,
                actual: f_post.rest.len(),
            });
        }
    }
    if pre.control.rest.len() != post.control.rest.len() {
        return Err(BReturnRightCompletionEmitError::ControlRestLengthMismatch {
            pre_len: pre.control.rest.len(),
            post_len: post.control.rest.len(),
        });
    }
    if pre.helper.rest.len() != post.helper.rest.len() {
        return Err(BReturnRightCompletionEmitError::HelperRestLengthMismatch {
            pre_len: pre.helper.rest.len(),
            post_len: post.helper.rest.len(),
        });
    }
    if pre.result.rest.len() != post.result.rest.len() {
        return Err(BReturnRightCompletionEmitError::ResultRestLengthMismatch {
            pre_len: pre.result.rest.len(),
            post_len: post.result.rest.len(),
        });
    }
    if pre.rest.len() != post.rest.len() {
        return Err(BReturnRightCompletionEmitError::StateRestLengthMismatch {
            pre_len: pre.rest.len(),
            post_len: post.rest.len(),
        });
    }
    Ok(cap)
}

/// Every caller `Variable` (`enable` and every `pre`/`post` lane, inactive frames included) must
/// carry the leading `%` that SDK serde `Operand` decoding requires and lie outside this block's
/// `%{scope}__` output namespace. First offending operand in this order wins.
fn check_caller_variables(
    enable: &Operand,
    pre: &BLegacyCompletionState,
    post: &BLegacyCompletionState,
    scope: &str,
) -> Result<(), BReturnRightCompletionEmitError> {
    let mut ops = vec![enable];
    for st in [pre, post] {
        let (c, h, r) = (&st.control, &st.helper, &st.result);
        ops.extend([&st.site, &st.outcome, &st.control_origin, &st.depth]);
        ops.extend([&c.tag, &c.return_value.space, &c.return_value.index, &c.prim_outcome_tag]);
        ops.extend([&c.prim_value.space, &c.prim_value.index, &c.prim_error_code, &c.error_code]);
        ops.extend([&h.present, &h.kind, &h.site, &h.mode, &h.phase, &h.operand_a.space]);
        ops.extend([&h.operand_a.index, &h.operand_b.present, &h.operand_b.value.space]);
        ops.extend([&h.operand_b.value.index, &h.staged.tag, &h.staged.bool_value]);
        ops.extend([&h.staged.pending_node_tag, &h.staged.prim_error_code, &h.staged.selector]);
        ops.extend([&h.staged.type_test_ok, &r.present, &r.tag, &r.scalar_value.space]);
        ops.extend([&r.scalar_value.index, &r.scalar_error_code, &r.scalar_selector]);
        ops.push(&r.scalar_type_test_ok);
        ops.extend(c.rest.iter().chain(&h.reg_a).chain(&h.staged.u128_value).chain(&h.rest));
        ops.extend(r.rest.iter().chain(&st.rest));
        for f in &st.frames {
            ops.extend([&f.tag, &f.origin, &f.right_op, &f.right_left.space, &f.right_left.index]);
            ops.extend(&f.rest);
        }
        for lanes in [
            &st.remaining_work,
            &st.consumed_work,
            &st.helper_work,
            &st.helper_visits,
            &st.reference_cursor,
        ] {
            ops.extend(lanes);
        }
    }
    let reserved = format!("%{scope}__");
    for op in ops {
        if let Operand::Variable(id) = op {
            if !id.0.starts_with('%') {
                return Err(BReturnRightCompletionEmitError::UnprefixedVariable);
            }
            if id.0.starts_with(&reserved) {
                return Err(BReturnRightCompletionEmitError::TemporaryNameCollision);
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// 1. k.machine.return.right (BinaryOp::Add, ScalarValue or ScalarError 10|11)
// ---------------------------------------------------------------------------

/// Emits the `midnight_zkir_v3::Instruction` stream for `k.machine.return.right`
/// (`/legacy_completion_instances/15`, `T-LEGACY`) on the `BinaryOp::Add` path.
fn emit_return_right_add_scoped(
    enable: Operand,
    pre: &BLegacyCompletionState,
    post: &BLegacyCompletionState,
    scope: &str,
) -> Result<Vec<Instruction>, BReturnRightCompletionEmitError> {
    validate_scope(scope)?;
    // An active ReturnRight pops a real top frame (`pre.depth >= 1`), so it refuses `cap == 0`.
    if pre.frames.is_empty() {
        return Err(BReturnRightCompletionEmitError::EmptyFrameCapacity);
    }
    let cap = validate_state_pair(pre, post)?;
    check_caller_variables(&enable, pre, post, scope)?;
    let mut em = Emitter::new(scope);

    let zero = em.load_imm_u64(0, "zero");
    let one = em.load_imm_u64(1, "one");
    let c_site_ret_right = em.load_imm_u64(SITE_RET_RIGHT, "site_ret_right");
    let c_outcome_running = em.load_imm_u64(OUTCOME_TAG_RUNNING, "outcome_running");
    let c_ctrl_return = em.load_imm_u64(CONTROL_TAG_RETURN, "ctrl_return");
    let c_ctrl_prim_result = em.load_imm_u64(CONTROL_TAG_PRIM_RESULT, "ctrl_prim_result");
    let c_frame_right_f = em.load_imm_u64(FRAME_TAG_RIGHT_F, "frame_right_f");
    let c_binop_add = em.load_imm_u64(BINARY_OP_ADD, "binop_add");
    let c_res_scalar_val = em.load_imm_u64(HELPER_RESULT_TAG_SCALAR_VALUE, "res_scalar_val");
    let c_res_scalar_err = em.load_imm_u64(HELPER_RESULT_TAG_SCALAR_ERROR, "res_scalar_err");
    let c_err_10 = em.load_imm_u64(PRIM_ERROR_ARITH_OVERFLOW, "err_10");
    let c_err_11 = em.load_imm_u64(PRIM_ERROR_SORT_MISMATCH, "err_11");
    let c_space_const = em.load_imm_u64(VALUE_SPACE_CONST, "space_const");
    let c_space_data = em.load_imm_u64(VALUE_SPACE_DATA, "space_data");

    em.assert_bit(enable.clone(), one.clone(), zero.clone(), "enable_bit");
    let not_enable = em.select(enable.clone(), zero.clone(), one.clone(), "not_enable");

    // Site and Outcome preconditions/preservation.
    em.assert_eq_when(
        enable.clone(),
        pre.site.clone(),
        c_site_ret_right,
        zero.clone(),
        "pre_site",
    );
    em.assert_eq_when(
        not_enable.clone(),
        post.site.clone(),
        pre.site.clone(),
        zero.clone(),
        "stutter_site",
    );
    em.assert_eq_when(
        enable.clone(),
        pre.outcome.clone(),
        c_outcome_running.clone(),
        zero.clone(),
        "pre_outcome",
    );
    // `/dispatch/site`: post.control = PrimResult selects site Prim.
    let c_site_prim = em.load_imm_u64(SITE_PRIM, "site_prim");
    em.assert_eq_when(
        enable.clone(),
        post.site.clone(),
        c_site_prim,
        zero.clone(),
        "post_site_prim",
    );
    em.assert_eq_when(
        enable.clone(),
        post.outcome.clone(),
        c_outcome_running,
        zero.clone(),
        "post_outcome",
    );
    em.assert_eq_when(
        not_enable.clone(),
        post.outcome.clone(),
        pre.outcome.clone(),
        zero.clone(),
        "stutter_outcome",
    );

    // Helper must be absent in both pre and post when enabled; stutter when disabled.
    em.assert_helper_absent(enable.clone(), &pre.helper, zero.clone(), "pre_hlp_abs");
    em.assert_helper_absent(enable.clone(), &post.helper, zero.clone(), "post_hlp_abs");
    em.assert_helper_eq_when(
        not_enable.clone(),
        &post.helper,
        &pre.helper,
        zero.clone(),
        "stutter_hlp",
    );

    // Pre-control must be ControlRow::Return { value } when enabled (`site == RetRight`).
    em.assert_eq_when(
        enable.clone(),
        pre.control.tag.clone(),
        c_ctrl_return,
        zero.clone(),
        "pre_ctrl_tag",
    );
    em.assert_valid_value_ref_when(
        enable.clone(),
        &pre.control.return_value,
        c_space_const.clone(),
        c_space_data.clone(),
        zero.clone(),
        "pre_ctrl_rv",
    );
    em.assert_zero_when(
        enable.clone(),
        pre.control.prim_outcome_tag.clone(),
        zero.clone(),
        "pre_ctrl_pot_z",
    );
    em.assert_value_ref_zero_when(
        enable.clone(),
        &pre.control.prim_value,
        zero.clone(),
        "pre_ctrl_pv_z",
    );
    em.assert_zero_when(
        enable.clone(),
        pre.control.prim_error_code.clone(),
        zero.clone(),
        "pre_ctrl_pec_z",
    );
    em.assert_zero_when(
        enable.clone(),
        pre.control.error_code.clone(),
        zero.clone(),
        "pre_ctrl_ec_z",
    );
    for (i, col) in pre.control.rest.iter().enumerate() {
        em.assert_zero_when(
            enable.clone(),
            col.clone(),
            zero.clone(),
            &format!("pre_ctrl_rest_z_{}", i),
        );
    }
    em.assert_u128_when(
        enable.clone(),
        pre.control_origin.clone(),
        zero.clone(),
        "pre_ctrl_orig_u128",
    );

    // Pre-result must be present with either ScalarValue(9) or ScalarError(10) when enabled.
    em.assert_eq_when(
        enable.clone(),
        pre.result.present.clone(),
        one.clone(),
        zero.clone(),
        "pre_res_pres",
    );
    let raw_is_val = em.is_equal(
        pre.result.tag.clone(),
        c_res_scalar_val,
        "raw_is_scalar_val",
    );
    let raw_is_err = em.is_equal(
        pre.result.tag.clone(),
        c_res_scalar_err,
        "raw_is_scalar_err",
    );
    let is_val = em.mul(enable.clone(), raw_is_val, "en_is_val");
    let is_err = em.mul(enable.clone(), raw_is_err, "en_is_err");
    let branch_sum = em.add(is_val.clone(), is_err.clone(), "res_branch_sum");
    em.assert_eq_always(branch_sum, enable.clone(), "res_branch_partition");

    // Pre-result active-union validation on ScalarValue vs ScalarError(10 | 11).
    // SI-21R: ScalarValue is Data-space only. Freshness (`index == active.data - 1`)
    // needs an `active.data` carrier and remains a caller StepInv premise.
    em.assert_eq_when(
        is_val.clone(),
        pre.result.scalar_value.space.clone(),
        c_space_data.clone(),
        zero.clone(),
        "pre_res_val_data_space",
    );
    em.assert_u128_when(
        is_val.clone(),
        pre.result.scalar_value.index.clone(),
        zero.clone(),
        "pre_res_val_index",
    );
    em.assert_zero_when(
        is_val.clone(),
        pre.result.scalar_error_code.clone(),
        zero.clone(),
        "pre_res_val_ec_z",
    );
    em.assert_value_ref_zero_when(
        is_err.clone(),
        &pre.result.scalar_value,
        zero.clone(),
        "pre_res_err_val_z",
    );
    em.assert_error_code_10_or_11_when(
        is_err.clone(),
        pre.result.scalar_error_code.clone(),
        c_err_10,
        c_err_11,
        zero.clone(),
        "pre_res_err_10_11",
    );
    em.assert_zero_when(
        enable.clone(),
        pre.result.scalar_selector.clone(),
        zero.clone(),
        "pre_res_sel_z",
    );
    em.assert_zero_when(
        enable.clone(),
        pre.result.scalar_type_test_ok.clone(),
        zero.clone(),
        "pre_res_tt_z",
    );
    for (i, col) in pre.result.rest.iter().enumerate() {
        em.assert_zero_when(
            enable.clone(),
            col.clone(),
            zero.clone(),
            &format!("pre_res_rest_z_{}", i),
        );
    }

    // Post-result: clearResult when enabled; stutter when disabled.
    em.assert_result_absent(enable.clone(), &post.result, zero.clone(), "post_res_clear");
    em.assert_result_eq_when(
        not_enable.clone(),
        &post.result,
        &pre.result,
        zero.clone(),
        "stutter_res",
    );

    // Post-control: PrimResult(Value | Error) when enabled; stutter when disabled.
    // Note: PRIM_OUTCOME_TAG_VALUE == 0 and PRIM_OUTCOME_TAG_ERROR == 1, so
    // `prim_outcome_tag == is_err` when `enable == 1`.
    em.assert_eq_when(
        enable.clone(),
        post.control.tag.clone(),
        c_ctrl_prim_result,
        zero.clone(),
        "post_ctrl_tag",
    );
    em.assert_value_ref_zero_when(
        enable.clone(),
        &post.control.return_value,
        zero.clone(),
        "post_ctrl_rv_z",
    );
    em.assert_eq_when(
        enable.clone(),
        post.control.prim_outcome_tag.clone(),
        is_err.clone(),
        zero.clone(),
        "post_ctrl_pot",
    );
    em.assert_value_ref_eq_when(
        is_val.clone(),
        &post.control.prim_value,
        &pre.result.scalar_value,
        zero.clone(),
        "post_ctrl_pv_eq",
    );
    em.assert_value_ref_zero_when(
        is_err.clone(),
        &post.control.prim_value,
        zero.clone(),
        "post_ctrl_pv_z_on_err",
    );
    em.assert_zero_when(
        is_val,
        post.control.prim_error_code.clone(),
        zero.clone(),
        "post_ctrl_pec_z_on_val",
    );
    em.assert_eq_when(
        is_err,
        post.control.prim_error_code.clone(),
        pre.result.scalar_error_code.clone(),
        zero.clone(),
        "post_ctrl_pec_eq",
    );
    em.assert_zero_when(
        enable.clone(),
        post.control.error_code.clone(),
        zero.clone(),
        "post_ctrl_ec_z",
    );
    for (i, col) in post.control.rest.iter().enumerate() {
        em.assert_zero_when(
            enable.clone(),
            col.clone(),
            zero.clone(),
            &format!("post_ctrl_rest_z_{}", i),
        );
    }
    em.assert_control_eq_when(
        not_enable.clone(),
        &post.control,
        &pre.control,
        zero.clone(),
        "stutter_ctrl",
    );

    // Frame stack pop (`framePop`):
    // `pre.depth == post.depth + enable`, and when `enable == 1`, `post.depth`
    // must match exactly one slot `i in 0..cap` (which is `pre.depth - 1`).
    let expected_pre_depth = em.add(post.depth.clone(), enable.clone(), "exp_pre_depth");
    em.assert_eq_always(pre.depth.clone(), expected_pre_depth, "depth_dec");

    let mut seen_top = zero.clone();
    for i in 0..cap {
        let idx_imm = em.load_imm_u64(i as u128, &format!("f_idx_{}", i));
        let raw_top_i = em.is_equal(post.depth.clone(), idx_imm, &format!("raw_top_{}", i));
        let is_popped_i = em.mul(enable.clone(), raw_top_i, &format!("is_popped_{}", i));
        let is_above_pre_i = seen_top.clone();
        let next_seen_top = em.add(seen_top, is_popped_i.clone(), &format!("seen_top_{}", i));
        let one_minus_seen = em.select(
            next_seen_top.clone(),
            zero.clone(),
            one.clone(),
            &format!("oms_{}", i),
        );
        let is_below_top_i = em.mul(enable.clone(), one_minus_seen, &format!("below_top_{}", i));
        let preserve_slot_i = em.add(
            not_enable.clone(),
            is_below_top_i,
            &format!("pres_slot_{}", i),
        );
        let zero_post_slot_i = em.add(
            is_popped_i.clone(),
            is_above_pre_i.clone(),
            &format!("zero_post_slot_{}", i),
        );

        // 1. Slots below `post.depth` (or when `enable == 0`) are preserved: `post.frames[i] == pre.frames[i]`.
        em.assert_frame_eq_when(
            preserve_slot_i,
            &post.frames[i],
            &pre.frames[i],
            zero.clone(),
            &format!("f_pres_{}", i),
        );

        // 2. Popped top slot (`i == post.depth == pre.depth - 1` when `enable == 1`):
        //    `pre.frames[i]` must be `FrameRow::RightF { op: BinaryOp::Add, left, origin }`
        //    and `post.control_origin == pre.frames[i].origin`.
        em.assert_eq_when(
            is_popped_i.clone(),
            pre.frames[i].tag.clone(),
            c_frame_right_f.clone(),
            zero.clone(),
            &format!("f_top_tag_{}", i),
        );
        em.assert_eq_when(
            is_popped_i.clone(),
            pre.frames[i].right_op.clone(),
            c_binop_add.clone(),
            zero.clone(),
            &format!("f_top_op_{}", i),
        );
        em.assert_valid_value_ref_when(
            is_popped_i.clone(),
            &pre.frames[i].right_left,
            c_space_const.clone(),
            c_space_data.clone(),
            zero.clone(),
            &format!("f_top_left_{}", i),
        );
        em.assert_u128_when(
            is_popped_i.clone(),
            pre.frames[i].origin.clone(),
            zero.clone(),
            &format!("f_top_orig_u128_{}", i),
        );
        em.assert_eq_when(
            is_popped_i.clone(),
            post.control_origin.clone(),
            pre.frames[i].origin.clone(),
            zero.clone(),
            &format!("f_top_ctrl_orig_{}", i),
        );
        for (j, col) in pre.frames[i].rest.iter().enumerate() {
            em.assert_zero_when(
                is_popped_i.clone(),
                col.clone(),
                zero.clone(),
                &format!("f_top_rest_z_{}_{}", i, j),
            );
        }

        // 3. Slots `>= pre.depth` (`is_above_pre_i == 1`) must be `zeroRow(FrameRow)` in `pre`,
        //    and slots `>= post.depth` (`zero_post_slot_i == 1`) must be `zeroRow(FrameRow)` in `post`.
        em.assert_frame_zero_when(
            is_above_pre_i,
            &pre.frames[i],
            zero.clone(),
            &format!("f_pre_above_z_{}", i),
        );
        em.assert_frame_zero_when(
            zero_post_slot_i,
            &post.frames[i],
            zero.clone(),
            &format!("f_post_ge_z_{}", i),
        );

        seen_top = next_seen_top;
    }

    // Exactly one slot `i in 0..cap` must match `post.depth` when `enable == 1`
    // (proving `0 <= post.depth < cap` and `1 <= pre.depth <= cap`).
    em.assert_eq_always(seen_top, enable.clone(), "valid_popped_frame_slot");

    em.assert_eq_when(
        not_enable,
        post.control_origin.clone(),
        pre.control_origin.clone(),
        zero.clone(),
        "stutter_ctrl_orig",
    );

    // T-LEGACY base counters and row-complement `state.rest` preservation.
    em.emit_t_legacy_common_counters_and_rest(enable, pre, post, zero);

    Ok(em.instructions)
}

// ---------------------------------------------------------------------------
// Shared helper for k.machine.primitive.value and k.machine.primitive.error
// ---------------------------------------------------------------------------

fn emit_primitive_common(
    em: &mut Emitter,
    enable: Operand,
    pre: &BLegacyCompletionState,
    post: &BLegacyCompletionState,
    cap: usize,
    zero: Operand,
    one: Operand,
) -> Operand {
    let c_site_prim = em.load_imm_u64(SITE_PRIM, "site_prim");
    let c_outcome_running = em.load_imm_u64(OUTCOME_TAG_RUNNING, "outcome_running");
    let c_ctrl_prim_result = em.load_imm_u64(CONTROL_TAG_PRIM_RESULT, "ctrl_prim_result");

    em.assert_bit(enable.clone(), one.clone(), zero.clone(), "enable_bit");
    let not_enable = em.select(enable.clone(), zero.clone(), one, "not_enable");

    // Site and Outcome preconditions/preservation (`Outcome::Running` preserved).
    em.assert_eq_when(
        enable.clone(),
        pre.site.clone(),
        c_site_prim,
        zero.clone(),
        "pre_site_prim",
    );
    em.assert_eq_when(
        not_enable.clone(),
        post.site.clone(),
        pre.site.clone(),
        zero.clone(),
        "stutter_site",
    );
    em.assert_eq_when(
        enable.clone(),
        pre.outcome.clone(),
        c_outcome_running.clone(),
        zero.clone(),
        "pre_outcome_running",
    );
    em.assert_eq_when(
        enable.clone(),
        post.outcome.clone(),
        c_outcome_running,
        zero.clone(),
        "post_outcome_running",
    );
    em.assert_eq_when(
        not_enable.clone(),
        post.outcome.clone(),
        pre.outcome.clone(),
        zero.clone(),
        "stutter_outcome",
    );

    // Helper and Result must both be absent in pre and post when enabled; stutter when disabled.
    em.assert_helper_absent(enable.clone(), &pre.helper, zero.clone(), "pre_hlp_abs");
    em.assert_helper_absent(enable.clone(), &post.helper, zero.clone(), "post_hlp_abs");
    em.assert_helper_eq_when(
        not_enable.clone(),
        &post.helper,
        &pre.helper,
        zero.clone(),
        "stutter_hlp",
    );
    em.assert_result_absent(enable.clone(), &pre.result, zero.clone(), "pre_res_abs");
    em.assert_result_absent(enable.clone(), &post.result, zero.clone(), "post_res_abs");
    em.assert_result_eq_when(
        not_enable.clone(),
        &post.result,
        &pre.result,
        zero.clone(),
        "stutter_res",
    );

    // Common pre.control == ControlRow::PrimResult requirements.
    em.assert_eq_when(
        enable.clone(),
        pre.control.tag.clone(),
        c_ctrl_prim_result,
        zero.clone(),
        "pre_ctrl_prim_tag",
    );
    em.assert_value_ref_zero_when(
        enable.clone(),
        &pre.control.return_value,
        zero.clone(),
        "pre_ctrl_rv_z",
    );
    em.assert_zero_when(
        enable.clone(),
        pre.control.error_code.clone(),
        zero.clone(),
        "pre_ctrl_ec_z",
    );
    for (i, col) in pre.control.rest.iter().enumerate() {
        em.assert_zero_when(
            enable.clone(),
            col.clone(),
            zero.clone(),
            &format!("pre_ctrl_rest_z_{}", i),
        );
    }

    // control_origin and depth are preserved unconditionally; range-checked when enabled.
    em.assert_u128_when(
        enable.clone(),
        pre.control_origin.clone(),
        zero.clone(),
        "pre_ctrl_orig_u128",
    );
    em.assert_eq_always(
        post.control_origin.clone(),
        pre.control_origin.clone(),
        "ctrl_orig_preserved",
    );
    em.assert_eq_always(post.depth.clone(), pre.depth.clone(), "depth_preserved");

    // Frames are preserved (`post.frames[i] == pre.frames[i]`), with `0 <= pre.depth <= cap`
    // and inactive slots `i >= pre.depth` constrained to `zeroRow(FrameRow)` when enabled.
    let d0_imm = em.load_imm_u64(0, "depth_idx_0");
    let raw_d0 = em.is_equal(pre.depth.clone(), d0_imm, "raw_depth_0");
    let mut reached_depth = em.mul(enable.clone(), raw_d0, "reached_depth_0");

    for i in 0..cap {
        let one_f = em.load_imm_u64(1, &format!("one_f_{}", i));
        em.assert_frame_eq_when(
            one_f,
            &post.frames[i],
            &pre.frames[i],
            zero.clone(),
            &format!("f_pres_{}", i),
        );
        em.assert_frame_zero_when(
            reached_depth.clone(),
            &pre.frames[i],
            zero.clone(),
            &format!("f_pre_inact_z_{}", i),
        );
        em.assert_frame_zero_when(
            reached_depth.clone(),
            &post.frames[i],
            zero.clone(),
            &format!("f_post_inact_z_{}", i),
        );

        let next_idx = em.load_imm_u64((i + 1) as u128, &format!("depth_idx_{}", i + 1));
        let raw_next = em.is_equal(
            pre.depth.clone(),
            next_idx,
            &format!("raw_depth_{}", i + 1),
        );
        let en_next = em.mul(enable.clone(), raw_next, &format!("en_depth_{}", i + 1));
        reached_depth = em.add(
            reached_depth,
            en_next,
            &format!("reached_depth_{}", i + 1),
        );
    }
    em.assert_eq_always(reached_depth, enable.clone(), "depth_in_range_0_to_cap");

    // T-LEGACY base counters and row-complement `state.rest` preservation.
    em.emit_t_legacy_common_counters_and_rest(enable, pre, post, zero);

    not_enable
}

// ---------------------------------------------------------------------------
// 2. k.machine.primitive.value (/legacy_completion_instances/35, T-LEGACY)
// ---------------------------------------------------------------------------

/// Emits the `midnight_zkir_v3::Instruction` stream for `k.machine.primitive.value`
/// (`/legacy_completion_instances/35`, `T-LEGACY`).
fn emit_primitive_value_scoped(
    enable: Operand,
    pre: &BLegacyCompletionState,
    post: &BLegacyCompletionState,
    scope: &str,
) -> Result<Vec<Instruction>, BReturnRightCompletionEmitError> {
    validate_scope(scope)?;
    let cap = validate_state_pair(pre, post)?;
    check_caller_variables(&enable, pre, post, scope)?;
    let mut em = Emitter::new(scope);

    let zero = em.load_imm_u64(0, "zero");
    let one = em.load_imm_u64(1, "one");
    let c_pot_val = em.load_imm_u64(PRIM_OUTCOME_TAG_VALUE, "pot_val");
    let c_ctrl_return = em.load_imm_u64(CONTROL_TAG_RETURN, "ctrl_return");
    let c_space_const = em.load_imm_u64(VALUE_SPACE_CONST, "space_const");
    let c_space_data = em.load_imm_u64(VALUE_SPACE_DATA, "space_data");

    let not_enable = emit_primitive_common(
        &mut em,
        enable.clone(),
        pre,
        post,
        cap,
        zero.clone(),
        one,
    );

    // Pre-control PrimResult must carry PrimOutcome::Value { value }.
    em.assert_eq_when(
        enable.clone(),
        pre.control.prim_outcome_tag.clone(),
        c_pot_val,
        zero.clone(),
        "pre_ctrl_pot_val",
    );
    em.assert_valid_value_ref_when(
        enable.clone(),
        &pre.control.prim_value,
        c_space_const,
        c_space_data,
        zero.clone(),
        "pre_ctrl_pv_valid",
    );
    em.assert_zero_when(
        enable.clone(),
        pre.control.prim_error_code.clone(),
        zero.clone(),
        "pre_ctrl_pec_z",
    );

    // Post-control must be ControlRow::Return { value: pre.control.prim_value }.
    em.assert_eq_when(
        enable.clone(),
        post.control.tag.clone(),
        c_ctrl_return,
        zero.clone(),
        "post_ctrl_return_tag",
    );
    em.assert_value_ref_eq_when(
        enable.clone(),
        &post.control.return_value,
        &pre.control.prim_value,
        zero.clone(),
        "post_ctrl_rv_eq_pv",
    );
    em.assert_zero_when(
        enable.clone(),
        post.control.prim_outcome_tag.clone(),
        zero.clone(),
        "post_ctrl_pot_z",
    );
    em.assert_value_ref_zero_when(
        enable.clone(),
        &post.control.prim_value,
        zero.clone(),
        "post_ctrl_pv_z",
    );
    em.assert_zero_when(
        enable.clone(),
        post.control.prim_error_code.clone(),
        zero.clone(),
        "post_ctrl_pec_z",
    );
    em.assert_zero_when(
        enable.clone(),
        post.control.error_code.clone(),
        zero.clone(),
        "post_ctrl_ec_z",
    );
    for (i, col) in post.control.rest.iter().enumerate() {
        em.assert_zero_when(
            enable.clone(),
            col.clone(),
            zero.clone(),
            &format!("post_ctrl_rest_z_{}", i),
        );
    }

    em.assert_control_eq_when(
        not_enable,
        &post.control,
        &pre.control,
        zero,
        "stutter_ctrl",
    );

    Ok(em.instructions)
}

// ---------------------------------------------------------------------------
// 3. k.machine.primitive.error (/legacy_completion_instances/36, T-LEGACY)
// ---------------------------------------------------------------------------

/// Emits the `midnight_zkir_v3::Instruction` stream for `k.machine.primitive.error`
/// (`/legacy_completion_instances/36`, `T-LEGACY`).
fn emit_primitive_error_scoped(
    enable: Operand,
    pre: &BLegacyCompletionState,
    post: &BLegacyCompletionState,
    scope: &str,
) -> Result<Vec<Instruction>, BReturnRightCompletionEmitError> {
    validate_scope(scope)?;
    let cap = validate_state_pair(pre, post)?;
    check_caller_variables(&enable, pre, post, scope)?;
    let mut em = Emitter::new(scope);

    let zero = em.load_imm_u64(0, "zero");
    let one = em.load_imm_u64(1, "one");
    let c_pot_err = em.load_imm_u64(PRIM_OUTCOME_TAG_ERROR, "pot_err");
    let c_ctrl_error = em.load_imm_u64(CONTROL_TAG_ERROR, "ctrl_error");
    let c_err_10 = em.load_imm_u64(PRIM_ERROR_ARITH_OVERFLOW, "err_10");
    let c_err_11 = em.load_imm_u64(PRIM_ERROR_SORT_MISMATCH, "err_11");

    let not_enable = emit_primitive_common(
        &mut em,
        enable.clone(),
        pre,
        post,
        cap,
        zero.clone(),
        one,
    );

    // Pre-control PrimResult must carry PrimOutcome::Error { code } with code in {10, 11}.
    em.assert_eq_when(
        enable.clone(),
        pre.control.prim_outcome_tag.clone(),
        c_pot_err,
        zero.clone(),
        "pre_ctrl_pot_err",
    );
    em.assert_value_ref_zero_when(
        enable.clone(),
        &pre.control.prim_value,
        zero.clone(),
        "pre_ctrl_pv_z",
    );
    em.assert_error_code_10_or_11_when(
        enable.clone(),
        pre.control.prim_error_code.clone(),
        c_err_10,
        c_err_11,
        zero.clone(),
        "pre_ctrl_pec_10_11",
    );

    // Post-control must be ControlRow::Error { code: pre.control.prim_error_code }.
    em.assert_eq_when(
        enable.clone(),
        post.control.tag.clone(),
        c_ctrl_error,
        zero.clone(),
        "post_ctrl_error_tag",
    );
    em.assert_eq_when(
        enable.clone(),
        post.control.error_code.clone(),
        pre.control.prim_error_code.clone(),
        zero.clone(),
        "post_ctrl_ec_eq_pec",
    );
    em.assert_value_ref_zero_when(
        enable.clone(),
        &post.control.return_value,
        zero.clone(),
        "post_ctrl_rv_z",
    );
    em.assert_zero_when(
        enable.clone(),
        post.control.prim_outcome_tag.clone(),
        zero.clone(),
        "post_ctrl_pot_z",
    );
    em.assert_value_ref_zero_when(
        enable.clone(),
        &post.control.prim_value,
        zero.clone(),
        "post_ctrl_pv_z",
    );
    em.assert_zero_when(
        enable.clone(),
        post.control.prim_error_code.clone(),
        zero.clone(),
        "post_ctrl_pec_z",
    );
    for (i, col) in post.control.rest.iter().enumerate() {
        em.assert_zero_when(
            enable.clone(),
            col.clone(),
            zero.clone(),
            &format!("post_ctrl_rest_z_{}", i),
        );
    }

    em.assert_control_eq_when(
        not_enable,
        &post.control,
        &pre.control,
        zero,
        "stutter_ctrl",
    );

    Ok(em.instructions)
}
