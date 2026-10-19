// Target: crates/mori-backend/src/sherlock_machine/b_admin_publication.rs

//! Handwritten B-core `midnight_zkir_v3::Instruction` circuit emitters for
//! administrative terminal publication (`P1` in `schema-host09.json`:
//! `k.admin.publish.value` and `k.admin.publish.error`, template `T-ADMIN`)
//! and the composed 5-stage `BinaryOp::Add` execution chain
//! (`b.helper.Scalar.arithmetic` -> `b.helper.Scalar.exit` / `b.helper.Scalar.failure`
//! -> `k.machine.return.right` -> `k.machine.primitive.value` / `k.machine.primitive.error`
//! -> `P1` `k.admin.publish.value` / `k.admin.publish.error`).
//!
//! Normative B-schema `P1` (`A2-double-prime R2`, `/precedence/steps/2`):
//! - Preconditions (`enable == 1`):
//!   * `pre.outcome == Outcome::Running` (`tag = 0`, all payload columns `0`).
//!   * `pre.helper` absent (present `0` and every payload column `0`).
//!   * `pre.result` absent (present `0` and every payload column `0`).
//!   * `pre.counters.task_live == 0` (`SI-07`, `SI-18`).
//!   * Branch guard:
//!     - `k.admin.publish.value`: `pre.control` is `ControlRow::Return { value }`
//!       (`tag = CONTROL_TAG_RETURN = 1`, inactive union columns `0`),
//!       `pre.depth == 0` (all physical `img.frames` slots `0..frames_cap`
//!       equal to `zeroRow(FrameRow)`), and `value` is a valid `ValueRef`
//!       (`space in {0, 1}`, `index < active_rows(space)` per `SI-13`).
//!     - `k.admin.publish.error`: `pre.control` is `ControlRow::Error { code }`
//!       (`tag = CONTROL_TAG_ERROR = 9`, inactive union columns `0`),
//!       `code in 1..=11` (4-bit `ErrorCode` domain per `/primitive_types/ErrorCode`;
//!       `0` and `>= 12` are rejected), and `0 <= pre.depth <= frames_cap`
//!       (active frames `< pre.depth` preserved, inactive frames `>= pre.depth` zero).
//! - Effects (`enable == 1`):
//!   * `post.outcome`: `Outcome::Halted { value }` (`tag = 1`, other columns `0`)
//!     on `publish.value`, or `Outcome::Rejected { code }` (`tag = 2`, other
//!     columns `0`) on `publish.error`.
//!   * Cost `0` (`ADMIN_PUBLISH_CHARGE = 0`): fires before `P2` (`remainingWork = 0`),
//!     so `remainingWork == 0` is permitted and all work counters
//!     (`remainingWork`, `consumedWork`, `helperWork`, `helperVisits`),
//!     `referenceCursor` (`D-09R`), allocation counters, `depth`, `controlOrigin`,
//!     `reg.control` (including `Return.value` / `Error.code` payload — never
//!     cleared to zero as in superseded `LiteralImage`/L4 `A80`/`A82`), `img.data`,
//!     and `img.frames` are preserved unchanged.
//! - Disabled behavior (`enable == 0`):
//!   * Constrains full state stutter (`post == pre`) so disabled `P1` fragments
//!     in a composed trace cannot forge state transitions.

use super::b_return_right_completion::{
    emit_k_machine_primitive_error_block, emit_k_machine_primitive_value_block,
    emit_k_machine_return_right_add_block, BControlRow, BFrameRow, BLegacyCompletionState,
    BPostSiteContract, BReturnRightCompletionEmitError, CONTROL_TAG_ERROR, CONTROL_TAG_RETURN,
    FRAME_TAG_CONSTRUCTOR_FIELDS_EXTENSION_RESERVED, FRAME_TAG_RIGHT_F, SITE_PRIM,
};
use super::b_scalar_arithmetic::{
    emit_b_scalar_arithmetic_add, BChargedCounters, BScalarArithmeticAddBlock,
    BScalarArithmeticAddInputs, BScalarArithmeticConstructionError, BScalarArithmeticRegister,
    BU128Lanes, BValueRef, BValueTableRead, HELPER_PRESENT, OPT_ABSENT, OPT_PRESENT,
    SCALAR_STAGED_TAG_PRIM_ERROR, SCALAR_STAGED_TAG_U128_VALUE,
};
use super::b_scalar_completion::{
    emit_b_scalar_exit_add_u128, emit_b_scalar_failure_add, BDataRowLanes, BDataTableLanes,
    BHelperResultLanes, BOutcomeLanes, BScalarCompletionConstructionError,
    BScalarCompletionCounters, BScalarExitAddU128Block, BScalarExitAddU128Inputs,
    BScalarFailureAddBlock, BScalarFailureAddInputs, OUTCOME_TAG_HALTED, OUTCOME_TAG_REJECTED,
    OUTCOME_TAG_RUNNING, VALUE_SPACE_CONST, VALUE_SPACE_DATA,
};
use super::b_return_right_subtraction::emit_k_machine_return_right_sub_block;
use super::b_scalar_subtraction::{
    emit_b_scalar_arithmetic_sub,
    BScalarArithmeticConstructionError as BSubArithmeticConstructionError,
    BScalarArithmeticSubBlock, BScalarArithmeticSubInputs, BValueRowLanes as BSubValueRowLanes,
    BValueTableRead as BSubValueTableRead,
};
use super::b_scalar_subtraction_completion::{
    emit_b_scalar_exit_sub_u128, emit_b_scalar_failure_sub, emit_b_sub_completion_complement,
    BScalarExitSubU128Block, BScalarExitSubU128Inputs, BScalarFailureSubBlock,
    BScalarFailureSubInputs, BScalarSubCompletionConstructionError, BSubPublishState,
};
use super::*;

/// Minimum valid B-schema `ErrorCode` (`Nat in 1..11`, `/primitive_types/ErrorCode`).
pub const ERROR_CODE_MIN: u128 = 1;

/// Maximum valid B-schema `ErrorCode` (`Nat in 1..11`, `/primitive_types/ErrorCode`).
pub const ERROR_CODE_MAX: u128 = 11;

/// Administrative `P1` step charge (`0`, `/templates/T-ADMIN` & `/precedence/steps/2`).
pub const ADMIN_PUBLISH_CHARGE: u128 = 0;

/// Full logical machine-state view for B-schema `P1` administrative publication.
///
/// Carries both scalar `counters.reference_cursor` (used by `b_scalar_completion`)
/// and 16-byte LE `reference_cursor_bytes` (used by `b_return_right_completion`),
/// which are constrained in-circuit to represent the same 128-bit integer.
#[derive(Clone, Debug, PartialEq)]
pub struct BAdminPublishState {
    pub site: Operand,
    pub outcome: BOutcomeLanes,
    pub control: BControlRow,
    pub control_origin: Operand,
    pub helper: BScalarArithmeticRegister,
    pub result: BHelperResultLanes,
    pub data: BDataTableLanes,
    pub frames: Vec<BFrameRow>,
    pub counters: BScalarCompletionCounters,
    pub reference_cursor_bytes: BU128Lanes,
}

/// Caller-supplied witness inputs for `k.admin.publish.value`,
/// `k.admin.publish.error`, and unified `P1` `emit_b_admin_publish_p1`.
#[derive(Clone, Debug, PartialEq)]
pub struct BAdminPublishInputs {
    pub enable: Operand,
    pub const_active_rows: Operand,
    pub pre: BAdminPublishState,
    pub post: BAdminPublishState,
}

/// Composable instruction fragment for B-schema `P1` publication.
#[derive(Clone, Debug, PartialEq)]
pub struct BAdminPublishBlock {
    pub instructions: Vec<Instruction>,
    pub publish_value: Operand,
    pub publish_error: Operand,
    pub published_value: BValueRef,
    pub published_error_code: Operand,
    pub preserved: BAdminPublishInputs,
}

/// Caller-supplied witness inputs for the 5-stage `BinaryOp::Add` chain:
/// - Stage 1 (`state0 -> state1`): `b.helper.Scalar.arithmetic` (`RetRight/Add/Second -> Done`)
/// - Stage 2 (`state1 -> state2`): `b.helper.Scalar.exit` (`Applied` vs `RefusedCapCharged(DataNodes)`)
///   or `b.helper.Scalar.failure` (`PrimError(10 | 11)`)
/// - Stage 3 (`state2 -> state3`): `k.machine.return.right` (enabled iff Stage 2 did not cap-refuse)
/// - Stage 4 (`state3 -> state4`): `k.machine.primitive.value` (via `prim_val_post`) or
///   `k.machine.primitive.error` (via `prim_err_post`)
/// - Stage 5 (`state4 -> state5`): `P1` `k.admin.publish.value` or `k.admin.publish.error`
///   (enabled iff Stage 2 did not cap-refuse and `state4` is `Error` or `Return` at `depth == 0`).
#[derive(Clone, Debug, PartialEq)]
pub struct BAddToPublicationChainInputs {
    pub enable: Operand,
    pub bounds_data_nodes: Operand,
    pub constants: BValueTableRead,
    pub state0: BAdminPublishState,
    pub state1: BAdminPublishState,
    pub state2: BAdminPublishState,
    /// Stage-2 per-branch posts (`exit`, `failure`): a disabled row's post must equal `state1`;
    /// `state2` is their exactly-one mux.
    pub s2_exit_post: BAdminPublishState,
    pub s2_fail_post: BAdminPublishState,
    pub state3: BAdminPublishState,
    pub prim_val_post: BLegacyCompletionState,
    pub prim_err_post: BLegacyCompletionState,
    pub state4: BAdminPublishState,
    pub state5: BAdminPublishState,
}

/// Composable instruction fragment for the 5-stage `BinaryOp::Add` chain.
#[derive(Clone, Debug, PartialEq)]
pub struct BAddToPublicationChainBlock {
    pub instructions: Vec<Instruction>,
    pub arithmetic_block: BScalarArithmeticAddBlock,
    pub exit_block: BScalarExitAddU128Block,
    pub failure_block: BScalarFailureAddBlock,
    pub p1_block: BAdminPublishBlock,
    pub exit_enable: Operand,
    pub failure_enable: Operand,
    pub legacy_enable: Operand,
    pub prim_value_enable: Operand,
    pub prim_error_enable: Operand,
    /// `primitive.value` at `depth > 0`: `post.site` is bound from the top frame.
    pub return_dispatch: Operand,
    pub p1_enable: Operand,
}

/// Caller-supplied witness inputs for the 5-stage `BinaryOp::Sub` chain, stage for stage the
/// `BAddToPublicationChainInputs` layout. Every state is a `BSubPublishState` (actual
/// `b_scalar_subtraction` helper and work lanes, `mode = 1`); stages 3-5 consume it only through
/// the lossless `to_legacy_handoff().state` / `to_admin_publish_state()` maps. `constants` is the
/// Sub arithmetic Const table read.
#[derive(Clone, Debug, PartialEq)]
pub struct BSubToPublicationChainInputs {
    pub enable: Operand,
    pub bounds_data_nodes: Operand,
    pub constants: BSubValueTableRead,
    pub state0: BSubPublishState,
    pub state1: BSubPublishState,
    pub state2: BSubPublishState,
    /// Stage-2 per-branch posts (`exit`, `failure`): a disabled row's post must equal `state1`;
    /// `state2` is their exactly-one mux.
    pub s2_exit_post: BSubPublishState,
    pub s2_fail_post: BSubPublishState,
    pub state3: BSubPublishState,
    pub prim_val_post: BLegacyCompletionState,
    pub prim_err_post: BLegacyCompletionState,
    pub state4: BSubPublishState,
    pub state5: BSubPublishState,
}

/// Composable instruction fragment for the 5-stage `BinaryOp::Sub` chain.
#[derive(Clone, Debug, PartialEq)]
pub struct BSubToPublicationChainBlock {
    pub instructions: Vec<Instruction>,
    pub arithmetic_block: BScalarArithmeticSubBlock,
    pub exit_block: BScalarExitSubU128Block,
    pub failure_block: BScalarFailureSubBlock,
    pub p1_block: BAdminPublishBlock,
    pub exit_enable: Operand,
    pub failure_enable: Operand,
    pub legacy_enable: Operand,
    pub prim_value_enable: Operand,
    pub prim_error_enable: Operand,
    /// `primitive.value` at `depth > 0`: `post.site` is bound from the top frame.
    pub return_dispatch: Operand,
    pub p1_enable: Operand,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BAdminPublicationError {
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
    DataRowCount,
    DataRowRestLaneCount,
    HelperRestLaneCount,
    ResultRestLaneCount,
    CounterRestLaneCount,
    Arithmetic(BScalarArithmeticConstructionError),
    Completion(BScalarCompletionConstructionError),
    ReturnRight(BReturnRightCompletionEmitError),
    PostSiteContract {
        expected: BPostSiteContract,
        actual: BPostSiteContract,
    },
    Fetch(ConstructionError),
    SubArithmetic(BSubArithmeticConstructionError),
    SubCompletion(BScalarSubCompletionConstructionError),
}

impl From<ConstructionError> for BAdminPublicationError {
    fn from(e: ConstructionError) -> Self {
        Self::Fetch(e)
    }
}

impl From<BScalarArithmeticConstructionError> for BAdminPublicationError {
    fn from(e: BScalarArithmeticConstructionError) -> Self {
        Self::Arithmetic(e)
    }
}

impl From<BScalarCompletionConstructionError> for BAdminPublicationError {
    fn from(e: BScalarCompletionConstructionError) -> Self {
        Self::Completion(e)
    }
}

impl From<BReturnRightCompletionEmitError> for BAdminPublicationError {
    fn from(e: BReturnRightCompletionEmitError) -> Self {
        Self::ReturnRight(e)
    }
}

impl From<BSubArithmeticConstructionError> for BAdminPublicationError {
    fn from(e: BSubArithmeticConstructionError) -> Self {
        Self::SubArithmetic(e)
    }
}

impl From<BScalarSubCompletionConstructionError> for BAdminPublicationError {
    fn from(e: BScalarSubCompletionConstructionError) -> Self {
        Self::SubCompletion(e)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PublishMode {
    ValueOnly,
    ErrorOnly,
    UnifiedP1,
}

/// Emit the in-circuit relation for `k.admin.publish.value` (`P1` value branch:
/// `pre.control` is `Return { value }` with `pre.depth == 0` -> `Outcome::Halted { value }`).
pub fn emit_k_admin_publish_value(
    input: &BAdminPublishInputs,
    scope: &str,
) -> Result<BAdminPublishBlock, BAdminPublicationError> {
    emit_admin_publish_with_mode(input, scope, PublishMode::ValueOnly)
}

/// Emit the in-circuit relation for `k.admin.publish.error` (`P1` error branch:
/// `pre.control` is `Error { code }` with `code in 1..=11` -> `Outcome::Rejected { code }`).
pub fn emit_k_admin_publish_error(
    input: &BAdminPublishInputs,
    scope: &str,
) -> Result<BAdminPublishBlock, BAdminPublicationError> {
    emit_admin_publish_with_mode(input, scope, PublishMode::ErrorOnly)
}

/// Emit the unified B-schema `P1` administrative publication relation
/// covering both `k.admin.publish.value` (`Return` at `depth == 0`) and
/// `k.admin.publish.error` (`Error(code in 1..=11)` at `0 <= depth <= frames_cap`).
pub fn emit_b_admin_publish_p1(
    input: &BAdminPublishInputs,
    scope: &str,
) -> Result<BAdminPublishBlock, BAdminPublicationError> {
    emit_admin_publish_with_mode(input, scope, PublishMode::UnifiedP1)
}

fn emit_admin_publish_with_mode(
    input: &BAdminPublishInputs,
    scope: &str,
    mode: PublishMode,
) -> Result<BAdminPublishBlock, BAdminPublicationError> {
    validate_publish_state_pair(scope, &input.pre, &input.post)?;
    let temp_prefix = format!("%{scope}.local.temp.");
    for op in [&input.enable, &input.const_active_rows]
        .into_iter()
        .chain(publish_state_operands(&input.pre))
        .chain(publish_state_operands(&input.post))
    {
        check_variable(op, &temp_prefix)?;
    }

    let mut b = Emitter {
        scope: scope.into(),
        serial: 0,
        code: Vec::new(),
    };
    let enable = &input.enable;
    b.boolean(enable.clone());

    // P1.1: Pre-state common guards: Outcome::Running, helper absent, result absent, taskLive == 0.
    constrain_outcome_running(&mut b, enable, &input.pre.outcome);
    constrain_helper_absent(&mut b, enable, &input.pre.helper);
    constrain_result_absent(&mut b, enable, &input.pre.result);
    b.gated_eq(enable, input.pre.counters.task_live.clone(), immediate(0));

    // P1.2: Branch discrimination between k.admin.publish.value and k.admin.publish.error.
    let is_ctrl_return = b.test_eq(input.pre.control.tag.clone(), immediate(CONTROL_TAG_RETURN));
    let is_ctrl_error = b.test_eq(input.pre.control.tag.clone(), immediate(CONTROL_TAG_ERROR));
    let is_depth_zero = b.test_eq(input.pre.counters.depth.clone(), immediate(0));
    let raw_pub_val = b.mul(is_ctrl_return, is_depth_zero);
    b.boolean(raw_pub_val.clone());

    let (publish_value, publish_error) = match mode {
        PublishMode::ValueOnly => {
            let pv = b.select(enable.clone(), raw_pub_val, immediate(0));
            b.boolean(pv.clone());
            b.eq(pv.clone(), enable.clone());
            (pv, immediate(0))
        }
        PublishMode::ErrorOnly => {
            let pe = b.select(enable.clone(), is_ctrl_error, immediate(0));
            b.boolean(pe.clone());
            b.eq(pe.clone(), enable.clone());
            (immediate(0), pe)
        }
        PublishMode::UnifiedP1 => {
            let pv = b.select(enable.clone(), raw_pub_val, immediate(0));
            b.boolean(pv.clone());
            let pe = b.select(enable.clone(), is_ctrl_error, immediate(0));
            b.boolean(pe.clone());
            let sum = b.add(pv.clone(), pe.clone());
            b.eq(sum, enable.clone());
            (pv, pe)
        }
    };

    // P1.3: Value branch (`publish_value == 1`):
    // - pre.control is Return { value }, all inactive ControlRow union columns 0.
    // - value.space in {0 (Const), 1 (Data)}, value.index < active_rows(space) (SI-13).
    // - pre.depth == 0 (already in raw_pub_val; also enforced via gated_eq).
    b.gated_eq(
        &publish_value,
        input.pre.control.tag.clone(),
        immediate(CONTROL_TAG_RETURN),
    );
    b.gated_eq(&publish_value, input.pre.counters.depth.clone(), immediate(0));
    for inactive_val_ctrl in [
        &input.pre.control.prim_outcome_tag,
        &input.pre.control.prim_value.space,
        &input.pre.control.prim_value.index,
        &input.pre.control.prim_error_code,
        &input.pre.control.error_code,
    ]
    .into_iter()
    .chain(input.pre.control.rest.iter())
    {
        b.gated_eq(&publish_value, inactive_val_ctrl.clone(), immediate(0));
    }

    let eff_rv_space = b.select(
        publish_value.clone(),
        input.pre.control.return_value.space.clone(),
        immediate(VALUE_SPACE_CONST),
    );
    b.boolean(eff_rv_space.clone());
    let eff_rv_index = eff_uint128(
        &mut b,
        &publish_value,
        &input.pre.control.return_value.index,
    );
    let eff_const_active = eff_uint128(&mut b, &publish_value, &input.const_active_rows);
    let eff_data_active = eff_uint128(&mut b, enable, &input.pre.data.active_rows);
    let selected_space_active = b.select(
        eff_rv_space.clone(),
        eff_data_active.clone(),
        eff_const_active,
    );
    let rv_in_active_bound = b.less_than(eff_rv_index.clone(), selected_space_active);
    let guarded_rv_in_bound = b.select(publish_value.clone(), rv_in_active_bound, immediate(1));
    b.assert(guarded_rv_in_bound);

    // P1.4: Error branch (`publish_error == 1`):
    // - pre.control is Error { code }, all inactive ControlRow union columns 0.
    // - code in 1..=11 (4-bit B-schema ErrorCode domain).
    b.gated_eq(
        &publish_error,
        input.pre.control.tag.clone(),
        immediate(CONTROL_TAG_ERROR),
    );
    for inactive_err_ctrl in [
        &input.pre.control.return_value.space,
        &input.pre.control.return_value.index,
        &input.pre.control.prim_outcome_tag,
        &input.pre.control.prim_value.space,
        &input.pre.control.prim_value.index,
        &input.pre.control.prim_error_code,
    ]
    .into_iter()
    .chain(input.pre.control.rest.iter())
    {
        b.gated_eq(&publish_error, inactive_err_ctrl.clone(), immediate(0));
    }

    let eff_err_code = b.select(
        publish_error.clone(),
        input.pre.control.error_code.clone(),
        immediate(ERROR_CODE_MIN),
    );
    b.bits(eff_err_code.clone(), 4);
    let mut err_domain_sum = immediate(0);
    for valid_code in ERROR_CODE_MIN..=ERROR_CODE_MAX {
        let eq_c = b.test_eq(eff_err_code.clone(), immediate(valid_code));
        err_domain_sum = b.add(err_domain_sum, eq_c);
    }
    b.eq(err_domain_sum, immediate(1));

    // P1.5: Post-state Outcome (`Halted(value)` on publish_value, `Rejected(code)` on publish_error,
    // stutter `post.outcome == pre.outcome` when `enable == 0`).
    let published_value = BValueRef {
        space: b.select(
            publish_value.clone(),
            input.pre.control.return_value.space.clone(),
            immediate(0),
        ),
        index: b.select(publish_value.clone(), eff_rv_index, immediate(0)),
    };
    let published_error_code = b.select(publish_error.clone(), eff_err_code, immediate(0));
    b.bits(published_error_code.clone(), 4);

    let exp_out_tag_enabled = b.select(
        publish_value.clone(),
        immediate(OUTCOME_TAG_HALTED),
        immediate(OUTCOME_TAG_REJECTED),
    );
    let exp_out_tag = b.select(
        enable.clone(),
        exp_out_tag_enabled,
        input.pre.outcome.tag.clone(),
    );
    b.eq(input.post.outcome.tag.clone(), exp_out_tag);

    let exp_halt_sp = b.select(
        enable.clone(),
        published_value.space.clone(),
        input.pre.outcome.halted_value.space.clone(),
    );
    b.eq(input.post.outcome.halted_value.space.clone(), exp_halt_sp);
    let exp_halt_ix = b.select(
        enable.clone(),
        published_value.index.clone(),
        input.pre.outcome.halted_value.index.clone(),
    );
    b.eq(input.post.outcome.halted_value.index.clone(), exp_halt_ix);

    let exp_rej_code = b.select(
        enable.clone(),
        published_error_code.clone(),
        input.pre.outcome.rejected_code.clone(),
    );
    b.eq(input.post.outcome.rejected_code.clone(), exp_rej_code);

    let exp_exh_reason = b.select(
        enable.clone(),
        immediate(0),
        input.pre.outcome.exhausted_reason.clone(),
    );
    b.eq(input.post.outcome.exhausted_reason.clone(), exp_exh_reason);

    // P1.6: Full preservation of `site`, `control` (NEVER cleared in B-schema P1!),
    // `control_origin`, `helper` (absent when enabled), `result` (absent when enabled),
    // `img.data`, `img.frames` (with active-prefix check `0 <= depth <= frames_cap`
    // and inactive slots >= depth zeroed when enabled), and all counters (0 charge!).
    b.eq(input.post.site.clone(), input.pre.site.clone());
    let _eff_ctrl_orig = eff_uint128(&mut b, enable, &input.pre.control_origin);
    b.eq(
        input.post.control_origin.clone(),
        input.pre.control_origin.clone(),
    );
    constrain_control_eq(&mut b, &input.pre.control, &input.post.control);
    constrain_helper_eq(&mut b, &input.pre.helper, &input.post.helper);
    constrain_result_eq(&mut b, &input.pre.result, &input.post.result);
    constrain_data_table_preserved(&mut b, enable, &input.pre.data, &input.post.data);
    constrain_frames_preserved_with_prefix(
        &mut b,
        enable,
        &input.pre.counters.depth,
        &input.post.counters.depth,
        &input.pre.frames,
        &input.post.frames,
    );
    constrain_zero_charge_counters_and_ref_cursor(&mut b, enable, &input.pre, &input.post);

    Ok(BAdminPublishBlock {
        instructions: b.code,
        publish_value,
        publish_error,
        published_value,
        published_error_code,
        preserved: input.clone(),
    })
}

/// Emit the composed 5-stage `BinaryOp::Add` execution chain from
/// `b.helper.Scalar.arithmetic` (`RetRight/Add/Second -> Done`) through
/// `b.helper.Scalar.exit` / `b.helper.Scalar.failure`, `k.machine.return.right`,
/// `k.machine.primitive.value` / `k.machine.primitive.error`, and B-schema `P1`
/// `k.admin.publish.value` / `k.admin.publish.error`.
pub fn emit_b_add_to_publication_chain(
    input: &BAddToPublicationChainInputs,
    scope: &str,
) -> Result<BAddToPublicationChainBlock, BAdminPublicationError> {
    if scope.is_empty() {
        return Err(ConstructionError::EmptyScope.into());
    }
    for (pre, post) in [
        (&input.state0, &input.state1),
        (&input.state1, &input.s2_exit_post),
        (&input.state1, &input.s2_fail_post),
        (&input.state1, &input.state2),
        (&input.state2, &input.state3),
        (&input.state3, &input.state4),
        (&input.state4, &input.state5),
    ] {
        validate_publish_state_pair(scope, pre, post)?;
    }

    // M-2: every caller operand, `constants` included, is checked against every temporary
    // namespace this chain derives (its own, Add's local and enabled-fetch blocks, exit, failure,
    // the exact-once legacy stage scopes `%{scope}_s3_rr__`, `%{scope}_s4_pv__` and
    // `%{scope}_s4_pe__`, and P1), since each sub-emitter checks only the fields it receives.
    // Every emitted name is `%`-prefixed, as SDK serde `Operand` decoding requires. `scope` must be
    // unique per `IrSource`, and no other block there may own a namespace derived here.
    let temp_prefixes = [
        format!("%{scope}_chain.local.temp."),
        format!("%{scope}_s1_arith.local.temp."),
        format!("%{scope}_s1_arith.const_fetch.temp."),
        format!("%{scope}_s1_arith.data_fetch.temp."),
        format!("%{scope}_s2_exit.local.temp."),
        format!("%{scope}_s2_fail.local.temp."),
        format!("%{scope}_s3_rr__"),
        format!("%{scope}_s4_pv__"),
        format!("%{scope}_s4_pe__"),
        format!("%{scope}_s5_p1.local.temp."),
    ];
    let states = [
        &input.state0,
        &input.state1,
        &input.s2_exit_post,
        &input.s2_fail_post,
        &input.state2,
        &input.state3,
        &input.state4,
        &input.state5,
    ];
    for op in [&input.enable, &input.bounds_data_nodes, &input.constants.active_rows]
        .into_iter()
        .chain(
            input
                .constants
                .rows
                .iter()
                .flat_map(|row| std::iter::once(&row.tag).chain(row.n.iter())),
        )
        .chain(states.into_iter().flat_map(publish_state_operands))
        .chain(
            [&input.prim_val_post, &input.prim_err_post]
                .into_iter()
                .flat_map(legacy_state_operands),
        )
    {
        for prefix in &temp_prefixes {
            check_variable(op, prefix)?;
        }
    }

    let mut b = Emitter {
        scope: format!("{scope}_chain"),
        serial: 0,
        code: Vec::new(),
    };
    let enable = &input.enable;
    b.boolean(enable.clone());

    // Bind `reference_cursor` scalar and 16-byte LE views at every stage `state0..=state5`.
    for st in [
        &input.state0,
        &input.state1,
        &input.state2,
        &input.state3,
        &input.state4,
        &input.state5,
    ] {
        constrain_ref_cursor_scalar_matches_bytes(
            &mut b,
            &st.counters.reference_cursor,
            &st.reference_cursor_bytes,
        );
    }

    // -----------------------------------------------------------------------
    // Stage 1: `b.helper.Scalar.arithmetic` (`state0 -> state1`)
    // -----------------------------------------------------------------------
    let arith_data = BValueTableRead {
        capacity: input.state0.data.capacity,
        active_rows: input.state0.data.active_rows.clone(),
        rows: input
            .state0
            .data
            .rows
            .iter()
            .map(BDataRowLanes::to_value_row_lanes)
            .collect(),
    };
    let arith_inputs = BScalarArithmeticAddInputs {
        enable: enable.clone(),
        constants: input.constants.clone(),
        data: arith_data,
        pre_helper: input.state0.helper.clone(),
        post_helper: input.state1.helper.clone(),
        pre_counters: input.state0.counters.work.clone(),
        post_counters: input.state1.counters.work.clone(),
    };
    let arithmetic_block =
        emit_b_scalar_arithmetic_add(&arith_inputs, &format!("{scope}_s1_arith"))?;
    b.code.extend(arithmetic_block.instructions.clone());

    // Stage 1 complement C preservation (`state0 -> state1`):
    // `site`, `outcome = Running`, `control`, `control_origin`, `result = absent`,
    // `data` table, `frames` table, and all non-work counters (`depth`, `reference_cursor`,
    // `alloc_*`, `task_live == 0`, `active_*`, `rest`) are preserved unchanged.
    constrain_outcome_running(&mut b, enable, &input.state0.outcome);
    constrain_outcome_eq(&mut b, &input.state0.outcome, &input.state1.outcome);
    constrain_result_absent(&mut b, enable, &input.state0.result);
    constrain_result_eq(&mut b, &input.state0.result, &input.state1.result);
    b.eq(input.state1.site.clone(), input.state0.site.clone());
    b.eq(
        input.state1.control_origin.clone(),
        input.state0.control_origin.clone(),
    );
    constrain_control_eq(&mut b, &input.state0.control, &input.state1.control);
    constrain_data_table_preserved(&mut b, enable, &input.state0.data, &input.state1.data);
    constrain_frames_preserved_with_prefix(
        &mut b,
        enable,
        &input.state0.counters.depth,
        &input.state1.counters.depth,
        &input.state0.frames,
        &input.state1.frames,
    );
    b.gated_eq(enable, input.state0.counters.task_live.clone(), immediate(0));
    constrain_non_work_counters_eq(&mut b, &input.state0.counters, &input.state1.counters);
    // M-1: a disabled Add row leaves its helper and work posts free, so a disabled chain pins all
    // of `state1` to `state0` (a no-op when `enable == 1`).
    constrain_stutter_unless(&mut b, enable, &input.state0, &input.state1);

    // -----------------------------------------------------------------------
    // Stage 2: `b.helper.Scalar.exit` or `b.helper.Scalar.failure` (`state1 -> state2`)
    // -----------------------------------------------------------------------
    let raw_is_u128_staged = b.test_eq(
        input.state1.helper.staged.tag.clone(),
        immediate(SCALAR_STAGED_TAG_U128_VALUE),
    );
    let raw_is_err_staged = b.test_eq(
        input.state1.helper.staged.tag.clone(),
        immediate(SCALAR_STAGED_TAG_PRIM_ERROR),
    );
    let exit_enable = b.select(enable.clone(), raw_is_u128_staged, immediate(0));
    b.boolean(exit_enable.clone());
    let failure_enable = b.select(enable.clone(), raw_is_err_staged, immediate(0));
    b.boolean(failure_enable.clone());
    let s2_branch_sum = b.add(exit_enable.clone(), failure_enable.clone());
    b.eq(s2_branch_sum, enable.clone());

    let exit_inputs = BScalarExitAddU128Inputs {
        enable: exit_enable.clone(),
        bounds_data_nodes: input.bounds_data_nodes.clone(),
        pre_data: input.state1.data.clone(),
        post_data: input.s2_exit_post.data.clone(),
        pre_helper: input.state1.helper.clone(),
        post_helper: input.s2_exit_post.helper.clone(),
        pre_result: input.state1.result.clone(),
        post_result: input.s2_exit_post.result.clone(),
        pre_outcome: input.state1.outcome.clone(),
        post_outcome: input.s2_exit_post.outcome.clone(),
        pre_counters: input.state1.counters.clone(),
        post_counters: input.s2_exit_post.counters.clone(),
    };
    let exit_block = emit_b_scalar_exit_add_u128(&exit_inputs, &format!("{scope}_s2_exit"))?;
    b.code.extend(exit_block.instructions.clone());

    let failure_inputs = BScalarFailureAddInputs {
        enable: failure_enable.clone(),
        pre_data: input.state1.data.clone(),
        post_data: input.s2_fail_post.data.clone(),
        pre_helper: input.state1.helper.clone(),
        post_helper: input.s2_fail_post.helper.clone(),
        pre_result: input.state1.result.clone(),
        post_result: input.s2_fail_post.result.clone(),
        pre_outcome: input.state1.outcome.clone(),
        post_outcome: input.s2_fail_post.outcome.clone(),
        pre_counters: input.state1.counters.clone(),
        post_counters: input.s2_fail_post.counters.clone(),
    };
    let failure_block = emit_b_scalar_failure_add(&failure_inputs, &format!("{scope}_s2_fail"))?;
    b.code.extend(failure_block.instructions.clone());
    constrain_stutter_unless(&mut b, &exit_enable, &input.state1, &input.s2_exit_post);
    constrain_stutter_unless(&mut b, &failure_enable, &input.state1, &input.s2_fail_post);
    constrain_publish_mux(
        &mut b,
        &exit_enable,
        &input.s2_exit_post,
        &failure_enable,
        &input.s2_fail_post,
        &input.state1,
        &input.state2,
    );

    // Stage 2 complement C preservation (`state1 -> state2`):
    // `site`, `control`, `control_origin`, and `frames` are preserved unchanged.
    b.eq(input.state2.site.clone(), input.state1.site.clone());
    b.eq(
        input.state2.control_origin.clone(),
        input.state1.control_origin.clone(),
    );
    constrain_control_eq(&mut b, &input.state1.control, &input.state2.control);
    constrain_frames_preserved_with_prefix(
        &mut b,
        enable,
        &input.state1.counters.depth,
        &input.state2.counters.depth,
        &input.state1.frames,
        &input.state2.frames,
    );

    // -----------------------------------------------------------------------
    // Stage 3: `k.machine.return.right` (`state2 -> state3`)
    // Enabled iff Stage 2 did NOT refuse at P6 DataNodes (`legacy_enable = applied + failure_enable`).
    // -----------------------------------------------------------------------
    let legacy_enable = b.add(exit_block.applied.clone(), failure_enable.clone());
    b.boolean(legacy_enable.clone());
    let total_s2_outcomes = b.add(legacy_enable.clone(), exit_block.cap_refused.clone());
    b.eq(total_s2_outcomes, enable.clone());

    let leg_state2 = to_legacy_completion_state(&input.state2);
    let leg_state3 = to_legacy_completion_state(&input.state3);
    // T-LEGACY / SI-09: the consumed result implies `taskLive == 0`. SI-21R freshness on this
    // chain's carrier: an applied `ScalarValue(Data(j))` has `j + 1 == active.data`.
    b.gated_eq(&legacy_enable, input.state2.counters.task_live.clone(), immediate(0));
    let fresh_next = b.add(input.state2.result.scalar_value.index.clone(), immediate(1));
    b.gated_eq(&exit_block.applied, fresh_next, input.state2.data.active_rows.clone());
    let s3 = emit_k_machine_return_right_add_block(
        legacy_enable.clone(),
        &leg_state2,
        &leg_state3,
        &format!("{scope}_s3_rr"),
    )?;
    expect_post_site(s3.post_site, BPostSiteContract::Bound(SITE_PRIM))?;
    b.code.extend(s3.instructions);

    // Stage 3 complement C preservation (`state2 -> state3`):
    // `outcome` payload columns, `data` arena, and `alloc_*`/`task_live`/`active_*` preserved.
    constrain_outcome_eq(&mut b, &input.state2.outcome, &input.state3.outcome);
    constrain_data_table_preserved(&mut b, enable, &input.state2.data, &input.state3.data);

    // -----------------------------------------------------------------------
    // Stage 4: `k.machine.primitive.value` / `k.machine.primitive.error` (`state3 -> state4`)
    // -----------------------------------------------------------------------
    let prim_value_enable = exit_block.applied.clone();
    let prim_error_enable = failure_enable.clone();

    let s4_val = emit_k_machine_primitive_value_block(
        prim_value_enable.clone(),
        &leg_state3,
        &input.prim_val_post,
        &format!("{scope}_s4_pv"),
    )?;
    expect_post_site(s4_val.post_site, BPostSiteContract::ReturnDispatch)?;
    b.code.extend(s4_val.instructions);

    let s4_err = emit_k_machine_primitive_error_block(
        prim_error_enable.clone(),
        &leg_state3,
        &input.prim_err_post,
        &format!("{scope}_s4_pe"),
    )?;
    expect_post_site(s4_err.post_site, BPostSiteContract::TerminalP1)?;
    b.code.extend(s4_err.instructions);

    // `ReturnDispatch` (`/dispatch/site`): at `depth > 0`, `post.site == top.tag + (RetRight - RightF)`
    // (`LetF..=ScopeF` -> `RetLet..=RetScope`; the reserved frame tag is C-INV). At `depth == 0`, and
    // for `TerminalP1`, the successor is P1, which has no SiteTag: the derived site lane is pinned
    // to `state3.site` only for witness determinism and feeds no dispatch.
    let pv = &input.prim_val_post;
    let pv_depth_zero = b.test_eq(pv.depth.clone(), immediate(0));
    let pv_to_p1 = b.mul(prim_value_enable.clone(), pv_depth_zero);
    b.boolean(pv_to_p1.clone());
    let return_dispatch = b.select(pv_to_p1.clone(), immediate(0), prim_value_enable.clone());
    let mut top_tag = immediate(0);
    for (i, frame) in pv.frames.iter().enumerate() {
        let is_top = b.test_eq(pv.depth.clone(), immediate(i as u128 + 1));
        let tag = b.select(is_top, frame.tag.clone(), immediate(0));
        top_tag = b.add(top_tag, tag);
    }
    let eff_top_tag = b.select(return_dispatch.clone(), top_tag, immediate(0));
    b.bits(eff_top_tag.clone(), 4);
    let top_tag_core = b.less_than(
        eff_top_tag.clone(),
        immediate(FRAME_TAG_CONSTRUCTOR_FIELDS_EXTENSION_RESERVED),
    );
    b.assert(top_tag_core);
    let dispatched_site = b.add(eff_top_tag, immediate(SITE_RET_RIGHT - FRAME_TAG_RIGHT_F));
    b.gated_eq(&return_dispatch, pv.site.clone(), dispatched_site);
    b.gated_eq(&pv_to_p1, pv.site.clone(), input.state3.site.clone());
    b.gated_eq(
        &prim_error_enable,
        input.prim_err_post.site.clone(),
        input.state3.site.clone(),
    );

    // Bind `state4`'s legacy view to `prim_val_post` when `prim_value_enable == 1`,
    // `prim_err_post` when `prim_error_enable == 1`, and `leg_state3` when `legacy_enable == 0`.
    let leg_state4 = to_legacy_completion_state(&input.state4);
    constrain_legacy_mux(
        &mut b,
        &prim_value_enable,
        &prim_error_enable,
        &input.prim_val_post,
        &input.prim_err_post,
        &leg_state3,
        &leg_state4,
    );
    constrain_outcome_eq(&mut b, &input.state3.outcome, &input.state4.outcome);
    constrain_data_table_preserved(&mut b, enable, &input.state3.data, &input.state4.data);

    // -----------------------------------------------------------------------
    // Stage 5: `P1` `k.admin.publish.value` / `k.admin.publish.error` (`state4 -> state5`)
    // Enabled when `prim_error_enable == 1` OR (`prim_value_enable == 1` and `state4.depth == 0`).
    // When `cap_refused == 1`, `p1_enable == 0` so `state5 == state4 == state2` (`Exhausted(DataNodes)`).
    // -----------------------------------------------------------------------
    let s4_depth_zero = b.test_eq(input.state4.counters.depth.clone(), immediate(0));
    let p1_val_ready = b.mul(prim_value_enable.clone(), s4_depth_zero);
    b.boolean(p1_val_ready.clone());
    let p1_enable = b.add(p1_val_ready, prim_error_enable.clone());
    b.boolean(p1_enable.clone());

    let p1_inputs = BAdminPublishInputs {
        enable: p1_enable.clone(),
        const_active_rows: input.constants.active_rows.clone(),
        pre: input.state4.clone(),
        post: input.state5.clone(),
    };
    let p1_block = emit_b_admin_publish_p1(&p1_inputs, &format!("{scope}_s5_p1"))?;
    b.code.extend(p1_block.instructions.clone());

    Ok(BAddToPublicationChainBlock {
        instructions: b.code,
        arithmetic_block,
        exit_block,
        failure_block,
        p1_block,
        exit_enable,
        failure_enable,
        legacy_enable,
        prim_value_enable,
        prim_error_enable,
        return_dispatch,
        p1_enable,
    })
}

/// Emit the composed 5-stage `BinaryOp::Sub` chain with the Add chain's stage relation:
/// `b.helper.Scalar.arithmetic` (`RetRight/Sub/Second -> Done`, checked `UInt128` subtraction),
/// Sub `b.helper.Scalar.exit` / `b.helper.Scalar.failure` (each with its own
/// `emit_b_sub_completion_complement`), `k.machine.return.right` (top `RightF.op == Sub`),
/// `k.machine.primitive.value` / `k.machine.primitive.error`, and `P1`. `helper.mode` stays `1`
/// and `RightF.op` stays `1` until popped; no lane is relabelled.
///
/// Namespace ABI: `%{scope}_chain.local.temp.`, `%{scope}_s1_arith.local.temp.`,
/// `%{scope}_s1_arith.{const,data}_fetch.temp.`, `%{scope}_s2_{exit,fail}.local.temp.`,
/// `%{scope}_s2_{exit,fail}_cmp.local.temp.`, `%{scope}_s3_rr__`, `%{scope}_s4_{pv,pe}__` and
/// `%{scope}_s5_p1.local.temp.`. Every caller operand, `constants` included, must be
/// `%`-prefixed and outside each of them (checked). `scope` must also pass the return-right scope
/// rule and be unique per `IrSource`. No other producer there may own a namespace that is a
/// prefix of, or is prefixed by, one of these (caller premise, no registry).
pub fn emit_b_sub_to_publication_chain(
    input: &BSubToPublicationChainInputs,
    scope: &str,
) -> Result<BSubToPublicationChainBlock, BAdminPublicationError> {
    if scope.is_empty() {
        return Err(ConstructionError::EmptyScope.into());
    }
    // Lane-for-lane chain/`P1` carrier views of the Sub states: no lane is re-derived.
    let a0 = input.state0.to_admin_publish_state();
    let a1 = input.state1.to_admin_publish_state();
    let a_exit = input.s2_exit_post.to_admin_publish_state();
    let a_fail = input.s2_fail_post.to_admin_publish_state();
    let a2 = input.state2.to_admin_publish_state();
    let a3 = input.state3.to_admin_publish_state();
    let a4 = input.state4.to_admin_publish_state();
    let a5 = input.state5.to_admin_publish_state();
    for (pre, post) in [
        (&a0, &a1),
        (&a1, &a_exit),
        (&a1, &a_fail),
        (&a1, &a2),
        (&a2, &a3),
        (&a3, &a4),
        (&a4, &a5),
    ] {
        validate_publish_state_pair(scope, pre, post)?;
    }

    // Every caller operand, `constants` included, against every namespace this chain derives.
    let temp_prefixes = [
        format!("%{scope}_chain.local.temp."),
        format!("%{scope}_s1_arith.local.temp."),
        format!("%{scope}_s1_arith.const_fetch.temp."),
        format!("%{scope}_s1_arith.data_fetch.temp."),
        format!("%{scope}_s2_exit.local.temp."),
        format!("%{scope}_s2_fail.local.temp."),
        format!("%{scope}_s2_exit_cmp.local.temp."),
        format!("%{scope}_s2_fail_cmp.local.temp."),
        format!("%{scope}_s3_rr__"),
        format!("%{scope}_s4_pv__"),
        format!("%{scope}_s4_pe__"),
        format!("%{scope}_s5_p1.local.temp."),
    ];
    let views = [&a0, &a1, &a_exit, &a_fail, &a2, &a3, &a4, &a5];
    for op in [&input.enable, &input.bounds_data_nodes, &input.constants.active_rows]
        .into_iter()
        .chain(
            input
                .constants
                .rows
                .iter()
                .flat_map(|row| std::iter::once(&row.tag).chain(row.n.iter())),
        )
        .chain(views.into_iter().flat_map(publish_state_operands))
        .chain(
            [&input.prim_val_post, &input.prim_err_post]
                .into_iter()
                .flat_map(legacy_state_operands),
        )
    {
        for prefix in &temp_prefixes {
            check_variable(op, prefix)?;
        }
    }

    let mut b = Emitter {
        scope: format!("{scope}_chain"),
        serial: 0,
        code: Vec::new(),
    };
    let enable = &input.enable;
    b.boolean(enable.clone());

    // Unconditional scalar/16-byte LE `reference_cursor` binding at every main stage and both
    // branch posts. The completion complement's own binding is only `enable`-gated.
    for st in views {
        constrain_ref_cursor_scalar_matches_bytes(
            &mut b,
            &st.counters.reference_cursor,
            &st.reference_cursor_bytes,
        );
    }

    // Stage 1: Sub `b.helper.Scalar.arithmetic` (`state0 -> state1`) over the states' own lanes.
    let arith_inputs = BScalarArithmeticSubInputs {
        enable: enable.clone(),
        constants: input.constants.clone(),
        data: sub_value_table_from_data(&input.state0.data),
        pre_helper: input.state0.helper.clone(),
        post_helper: input.state1.helper.clone(),
        pre_counters: input.state0.counters.work.clone(),
        post_counters: input.state1.counters.work.clone(),
    };
    let arithmetic_block =
        emit_b_scalar_arithmetic_sub(&arith_inputs, &format!("{scope}_s1_arith"))?;
    b.code.extend(arithmetic_block.instructions.clone());
    constrain_outcome_running(&mut b, enable, &a0.outcome);
    constrain_outcome_eq(&mut b, &a0.outcome, &a1.outcome);
    constrain_result_absent(&mut b, enable, &a0.result);
    constrain_result_eq(&mut b, &a0.result, &a1.result);
    b.eq(a1.site.clone(), a0.site.clone());
    b.eq(a1.control_origin.clone(), a0.control_origin.clone());
    constrain_control_eq(&mut b, &a0.control, &a1.control);
    constrain_data_table_preserved(&mut b, enable, &a0.data, &a1.data);
    constrain_frames_preserved_with_prefix(
        &mut b,
        enable,
        &a0.counters.depth,
        &a1.counters.depth,
        &a0.frames,
        &a1.frames,
    );
    b.gated_eq(enable, a0.counters.task_live.clone(), immediate(0));
    constrain_non_work_counters_eq(&mut b, &a0.counters, &a1.counters);
    // A disabled Sub row leaves its helper and work posts free, so pin all of `state1` to `state0`.
    constrain_stutter_unless(&mut b, enable, &a0, &a1);

    // Stage 2: Sub `exit` / `failure` (`state1 -> state2`), selected by the actual staged kind.
    let raw_is_u128_staged = b.test_eq(
        input.state1.helper.staged.tag.clone(),
        immediate(SCALAR_STAGED_TAG_U128_VALUE),
    );
    let raw_is_err_staged = b.test_eq(
        input.state1.helper.staged.tag.clone(),
        immediate(SCALAR_STAGED_TAG_PRIM_ERROR),
    );
    let exit_enable = b.select(enable.clone(), raw_is_u128_staged, immediate(0));
    b.boolean(exit_enable.clone());
    let failure_enable = b.select(enable.clone(), raw_is_err_staged, immediate(0));
    b.boolean(failure_enable.clone());
    let s2_branch_sum = b.add(exit_enable.clone(), failure_enable.clone());
    b.eq(s2_branch_sum, enable.clone());

    let exit_inputs = BScalarExitSubU128Inputs::from_states(
        exit_enable.clone(),
        input.bounds_data_nodes.clone(),
        &input.state1,
        &input.s2_exit_post,
    );
    let exit_block = emit_b_scalar_exit_sub_u128(&exit_inputs, &format!("{scope}_s2_exit"))?;
    b.code.extend(exit_block.instructions.clone());
    b.code.extend(emit_b_sub_completion_complement(
        &exit_enable,
        &input.state1,
        &input.s2_exit_post,
        &format!("{scope}_s2_exit_cmp"),
    )?);
    let failure_inputs = BScalarFailureSubInputs::from_states(
        failure_enable.clone(),
        &input.state1,
        &input.s2_fail_post,
    );
    let failure_block = emit_b_scalar_failure_sub(&failure_inputs, &format!("{scope}_s2_fail"))?;
    b.code.extend(failure_block.instructions.clone());
    b.code.extend(emit_b_sub_completion_complement(
        &failure_enable,
        &input.state1,
        &input.s2_fail_post,
        &format!("{scope}_s2_fail_cmp"),
    )?);
    constrain_stutter_unless(&mut b, &exit_enable, &a1, &a_exit);
    constrain_stutter_unless(&mut b, &failure_enable, &a1, &a_fail);
    constrain_publish_mux(&mut b, &exit_enable, &a_exit, &failure_enable, &a_fail, &a1, &a2);
    b.eq(a2.site.clone(), a1.site.clone());
    b.eq(a2.control_origin.clone(), a1.control_origin.clone());
    constrain_control_eq(&mut b, &a1.control, &a2.control);
    constrain_frames_preserved_with_prefix(
        &mut b,
        enable,
        &a1.counters.depth,
        &a2.counters.depth,
        &a1.frames,
        &a2.frames,
    );

    // Stage 3: Sub `k.machine.return.right` (`state2 -> state3`), enabled iff stage 2 did not
    // refuse at P6 DataNodes. A refusal stutters through the end with `Exhausted(DataNodes)`.
    let legacy_enable = b.add(exit_block.applied.clone(), failure_enable.clone());
    b.boolean(legacy_enable.clone());
    let total_s2_outcomes = b.add(legacy_enable.clone(), exit_block.cap_refused.clone());
    b.eq(total_s2_outcomes, enable.clone());
    let leg_state2 = input.state2.to_legacy_handoff().state;
    let leg_state3 = input.state3.to_legacy_handoff().state;
    // T-LEGACY / SI-09: `taskLive == 0` at the ReturnRight pre-state. SI-21R on this carrier: an
    // applied `ScalarValue(Data(j))` has `j + 1 == active.data`, and the row written is `U128`.
    b.gated_eq(&legacy_enable, a2.counters.task_live.clone(), immediate(0));
    let fresh_next = b.add(a2.result.scalar_value.index.clone(), immediate(1));
    b.gated_eq(&exit_block.applied, fresh_next, a2.data.active_rows.clone());
    for (write_sel, row) in exit_block.write_selectors.iter().zip(a2.data.rows.iter()) {
        b.gated_eq(write_sel, row.tag.clone(), immediate(DATA_TAG_U128));
    }
    let s3 = emit_k_machine_return_right_sub_block(
        legacy_enable.clone(),
        &leg_state2,
        &leg_state3,
        &format!("{scope}_s3_rr"),
    )?;
    expect_post_site(s3.post_site, BPostSiteContract::Bound(SITE_PRIM))?;
    b.code.extend(s3.instructions);
    constrain_outcome_eq(&mut b, &a2.outcome, &a3.outcome);
    constrain_data_table_preserved(&mut b, enable, &a2.data, &a3.data);

    // Stage 4: `k.machine.primitive.value` / `k.machine.primitive.error` (`state3 -> state4`).
    let prim_value_enable = exit_block.applied.clone();
    let prim_error_enable = failure_enable.clone();
    let s4_val = emit_k_machine_primitive_value_block(
        prim_value_enable.clone(),
        &leg_state3,
        &input.prim_val_post,
        &format!("{scope}_s4_pv"),
    )?;
    expect_post_site(s4_val.post_site, BPostSiteContract::ReturnDispatch)?;
    b.code.extend(s4_val.instructions);
    let s4_err = emit_k_machine_primitive_error_block(
        prim_error_enable.clone(),
        &leg_state3,
        &input.prim_err_post,
        &format!("{scope}_s4_pe"),
    )?;
    expect_post_site(s4_err.post_site, BPostSiteContract::TerminalP1)?;
    b.code.extend(s4_err.instructions);

    // `ReturnDispatch` at `depth > 0`: `post.site == top.tag + (RetRight - RightF)`. The reserved
    // frame tag is refused. At `depth == 0`, and for `TerminalP1`, the successor is P1 (no SiteTag),
    // and the site lane is pinned to `state3.site` only for witness determinism.
    let pv = &input.prim_val_post;
    let pv_depth_zero = b.test_eq(pv.depth.clone(), immediate(0));
    let pv_to_p1 = b.mul(prim_value_enable.clone(), pv_depth_zero);
    b.boolean(pv_to_p1.clone());
    let return_dispatch = b.select(pv_to_p1.clone(), immediate(0), prim_value_enable.clone());
    let mut top_tag = immediate(0);
    for (i, frame) in pv.frames.iter().enumerate() {
        let is_top = b.test_eq(pv.depth.clone(), immediate(i as u128 + 1));
        let tag = b.select(is_top, frame.tag.clone(), immediate(0));
        top_tag = b.add(top_tag, tag);
    }
    let eff_top_tag = b.select(return_dispatch.clone(), top_tag, immediate(0));
    b.bits(eff_top_tag.clone(), 4);
    let top_tag_core = b.less_than(
        eff_top_tag.clone(),
        immediate(FRAME_TAG_CONSTRUCTOR_FIELDS_EXTENSION_RESERVED),
    );
    b.assert(top_tag_core);
    let dispatched_site = b.add(eff_top_tag, immediate(SITE_RET_RIGHT - FRAME_TAG_RIGHT_F));
    b.gated_eq(&return_dispatch, pv.site.clone(), dispatched_site);
    b.gated_eq(&pv_to_p1, pv.site.clone(), a3.site.clone());
    b.gated_eq(&prim_error_enable, input.prim_err_post.site.clone(), a3.site.clone());
    let leg_state4 = input.state4.to_legacy_handoff().state;
    constrain_legacy_mux(
        &mut b,
        &prim_value_enable,
        &prim_error_enable,
        &input.prim_val_post,
        &input.prim_err_post,
        &leg_state3,
        &leg_state4,
    );
    constrain_outcome_eq(&mut b, &a3.outcome, &a4.outcome);
    constrain_data_table_preserved(&mut b, enable, &a3.data, &a4.data);

    // Stage 5: `P1` (`state4 -> state5`) when `prim_error_enable == 1` or a value at `depth == 0`.
    let s4_depth_zero = b.test_eq(a4.counters.depth.clone(), immediate(0));
    let p1_val_ready = b.mul(prim_value_enable.clone(), s4_depth_zero);
    b.boolean(p1_val_ready.clone());
    let p1_enable = b.add(p1_val_ready, prim_error_enable.clone());
    b.boolean(p1_enable.clone());
    let p1_inputs = BAdminPublishInputs {
        enable: p1_enable.clone(),
        const_active_rows: input.constants.active_rows.clone(),
        pre: a4,
        post: a5,
    };
    let p1_block = emit_b_admin_publish_p1(&p1_inputs, &format!("{scope}_s5_p1"))?;
    b.code.extend(p1_block.instructions.clone());

    Ok(BSubToPublicationChainBlock {
        instructions: b.code,
        arithmetic_block,
        exit_block,
        failure_block,
        p1_block,
        exit_enable,
        failure_enable,
        legacy_enable,
        prim_value_enable,
        prim_error_enable,
        return_dispatch,
        p1_enable,
    })
}

/// `img.data` read view for Sub `Scalar.arithmetic`. It moves `capacity`, `active_rows` and each
/// row's `tag` and `n` lanes (the lanes of `BDataRowLanes::to_value_row_lanes`) unchanged into
/// the Sub carrier. The other row lanes stay constrained by the chain itself.
fn sub_value_table_from_data(data: &BDataTableLanes) -> BSubValueTableRead {
    BSubValueTableRead {
        capacity: data.capacity,
        active_rows: data.active_rows.clone(),
        rows: data
            .rows
            .iter()
            .map(|row| BSubValueRowLanes {
                tag: row.tag.clone(),
                n: row.n.clone(),
            })
            .collect(),
    }
}

/// Typed B -> legacy bridge for stages 3-4: `outcome := outcome.tag`, `reference_cursor :=
/// reference_cursor_bytes`, `rest := [alloc_data_nodes, alloc_data_fields,
/// alloc_environment_cells, task_live, active_data_fields, active_env, active_value_slots] ++
/// counters.rest`. The chain constrains its complement explicitly: outcome payload
/// (`constrain_outcome_eq`), the scalar cursor (`constrain_ref_cursor_scalar_matches_bytes`),
/// `data` (`constrain_data_table_preserved`) and `task_live == 0` at the ReturnRight pre-state.
fn to_legacy_completion_state(st: &BAdminPublishState) -> BLegacyCompletionState {
    let mut rest = vec![
        st.counters.alloc_data_nodes.clone(),
        st.counters.alloc_data_fields.clone(),
        st.counters.alloc_environment_cells.clone(),
        st.counters.task_live.clone(),
        st.counters.active_data_fields.clone(),
        st.counters.active_env.clone(),
        st.counters.active_value_slots.clone(),
    ];
    rest.extend(st.counters.rest.iter().cloned());
    BLegacyCompletionState {
        site: st.site.clone(),
        outcome: st.outcome.tag.clone(),
        control: st.control.clone(),
        control_origin: st.control_origin.clone(),
        helper: st.helper.clone(),
        result: st.result.clone(),
        frames: st.frames.clone(),
        depth: st.counters.depth.clone(),
        remaining_work: st.counters.work.remaining_work.clone(),
        consumed_work: st.counters.work.consumed_work.clone(),
        helper_work: st.counters.work.helper_work.clone(),
        helper_visits: st.counters.work.helper_visits.clone(),
        reference_cursor: st.reference_cursor_bytes.clone(),
        rest,
    }
}

fn constrain_legacy_mux(
    b: &mut Emitter,
    sel_val: &Operand,
    sel_err: &Operand,
    val_st: &BLegacyCompletionState,
    err_st: &BLegacyCompletionState,
    default_st: &BLegacyCompletionState,
    target_st: &BLegacyCompletionState,
) {
    let mux_op = |b: &mut Emitter, v: &Operand, e: &Operand, d: &Operand, t: &Operand| {
        let err_or_def = b.select(sel_err.clone(), e.clone(), d.clone());
        let chosen = b.select(sel_val.clone(), v.clone(), err_or_def);
        b.eq(t.clone(), chosen);
    };
    mux_op(b, &val_st.site, &err_st.site, &default_st.site, &target_st.site);
    mux_op(
        b,
        &val_st.outcome,
        &err_st.outcome,
        &default_st.outcome,
        &target_st.outcome,
    );
    mux_op(
        b,
        &val_st.control_origin,
        &err_st.control_origin,
        &default_st.control_origin,
        &target_st.control_origin,
    );
    mux_op(
        b,
        &val_st.depth,
        &err_st.depth,
        &default_st.depth,
        &target_st.depth,
    );
    for ((v, e), (d, t)) in control_operands(&val_st.control)
        .zip(control_operands(&err_st.control))
        .zip(control_operands(&default_st.control).zip(control_operands(&target_st.control)))
    {
        mux_op(b, v, e, d, t);
    }
    for ((v, e), (d, t)) in helper_operands(&val_st.helper)
        .zip(helper_operands(&err_st.helper))
        .zip(helper_operands(&default_st.helper).zip(helper_operands(&target_st.helper)))
    {
        mux_op(b, v, e, d, t);
    }
    for ((v, e), (d, t)) in result_operands(&val_st.result)
        .zip(result_operands(&err_st.result))
        .zip(result_operands(&default_st.result).zip(result_operands(&target_st.result)))
    {
        mux_op(b, v, e, d, t);
    }
    for (((vf, ef), df), tf) in val_st
        .frames
        .iter()
        .zip(err_st.frames.iter())
        .zip(default_st.frames.iter())
        .zip(target_st.frames.iter())
    {
        for ((v, e), (d, t)) in frame_operands(vf)
            .zip(frame_operands(ef))
            .zip(frame_operands(df).zip(frame_operands(tf)))
        {
            mux_op(b, v, e, d, t);
        }
    }
    for i in 0..16 {
        mux_op(
            b,
            &val_st.remaining_work[i],
            &err_st.remaining_work[i],
            &default_st.remaining_work[i],
            &target_st.remaining_work[i],
        );
        mux_op(
            b,
            &val_st.consumed_work[i],
            &err_st.consumed_work[i],
            &default_st.consumed_work[i],
            &target_st.consumed_work[i],
        );
        mux_op(
            b,
            &val_st.helper_work[i],
            &err_st.helper_work[i],
            &default_st.helper_work[i],
            &target_st.helper_work[i],
        );
        mux_op(
            b,
            &val_st.helper_visits[i],
            &err_st.helper_visits[i],
            &default_st.helper_visits[i],
            &target_st.helper_visits[i],
        );
        mux_op(
            b,
            &val_st.reference_cursor[i],
            &err_st.reference_cursor[i],
            &default_st.reference_cursor[i],
            &target_st.reference_cursor[i],
        );
    }
    for (((v, e), d), t) in val_st
        .rest
        .iter()
        .zip(err_st.rest.iter())
        .zip(default_st.rest.iter())
        .zip(target_st.rest.iter())
    {
        mux_op(b, v, e, d, t);
    }
}

fn expect_post_site(
    actual: BPostSiteContract,
    expected: BPostSiteContract,
) -> Result<(), BAdminPublicationError> {
    if actual == expected {
        Ok(())
    } else {
        Err(BAdminPublicationError::PostSiteContract { expected, actual })
    }
}

/// Pins a sibling row's own post to `pre` unless that row is enabled.
fn constrain_stutter_unless(
    b: &mut Emitter,
    enable: &Operand,
    pre: &BAdminPublishState,
    post: &BAdminPublishState,
) {
    let disabled = b.select(enable.clone(), immediate(0), immediate(1));
    for (p, q) in publish_state_operands(pre).zip(publish_state_operands(post)) {
        b.gated_eq(&disabled, q.clone(), p.clone());
    }
}

/// `target := a` if `sel_a`, else `alt` if `sel_alt`, else `default` (callers make the selectors
/// exclusive booleans).
fn constrain_publish_mux(
    b: &mut Emitter,
    sel_a: &Operand,
    a: &BAdminPublishState,
    sel_alt: &Operand,
    alt: &BAdminPublishState,
    default: &BAdminPublishState,
    target: &BAdminPublishState,
) {
    for (((x, y), d), t) in publish_state_operands(a)
        .zip(publish_state_operands(alt))
        .zip(publish_state_operands(default))
        .zip(publish_state_operands(target))
    {
        let alt_or_default = b.select(sel_alt.clone(), y.clone(), d.clone());
        let chosen = b.select(sel_a.clone(), x.clone(), alt_or_default);
        b.eq(t.clone(), chosen);
    }
}

fn legacy_state_operands(st: &BLegacyCompletionState) -> impl Iterator<Item = &Operand> {
    [&st.site, &st.outcome, &st.control_origin, &st.depth]
        .into_iter()
        .chain(control_operands(&st.control))
        .chain(helper_operands(&st.helper))
        .chain(result_operands(&st.result))
        .chain(st.frames.iter().flat_map(frame_operands))
        .chain(
            [
                &st.remaining_work,
                &st.consumed_work,
                &st.helper_work,
                &st.helper_visits,
                &st.reference_cursor,
            ]
            .into_iter()
            .flatten(),
        )
        .chain(st.rest.iter())
}

fn validate_publish_state_pair(
    scope: &str,
    pre: &BAdminPublishState,
    post: &BAdminPublishState,
) -> Result<(), BAdminPublicationError> {
    if scope.is_empty() {
        return Err(ConstructionError::EmptyScope.into());
    }
    // F4: `bounds.frames == 0` is admitted (P1 at `depth == 0`); only a pre/post mismatch is refused.
    if pre.frames.len() != post.frames.len() {
        return Err(BAdminPublicationError::FrameCapacityMismatch {
            pre_len: pre.frames.len(),
            post_len: post.frames.len(),
        });
    }
    let expected_frame_rest = pre.frames.first().map_or(0, |f| f.rest.len());
    for (slot, (fp, fq)) in pre.frames.iter().zip(post.frames.iter()).enumerate() {
        if fp.rest.len() != expected_frame_rest {
            return Err(BAdminPublicationError::FrameRestLengthMismatch {
                slot,
                expected: expected_frame_rest,
                actual: fp.rest.len(),
            });
        }
        if fq.rest.len() != expected_frame_rest {
            return Err(BAdminPublicationError::FrameRestLengthMismatch {
                slot,
                expected: expected_frame_rest,
                actual: fq.rest.len(),
            });
        }
    }
    if pre.control.rest.len() != post.control.rest.len() {
        return Err(BAdminPublicationError::ControlRestLengthMismatch {
            pre_len: pre.control.rest.len(),
            post_len: post.control.rest.len(),
        });
    }
    if pre.data.rows.len() != pre.data.capacity
        || post.data.rows.len() != post.data.capacity
        || pre.data.capacity != post.data.capacity
    {
        return Err(BAdminPublicationError::DataRowCount);
    }
    if let Some(first) = pre.data.rows.first() {
        let expected_data_rest = first.rest.len();
        for (pr, qr) in pre.data.rows.iter().zip(post.data.rows.iter()) {
            if pr.rest.len() != expected_data_rest || qr.rest.len() != expected_data_rest {
                return Err(BAdminPublicationError::DataRowRestLaneCount);
            }
        }
    }
    if pre.helper.rest.len() != post.helper.rest.len() {
        return Err(BAdminPublicationError::HelperRestLaneCount);
    }
    if pre.result.rest.len() != post.result.rest.len() {
        return Err(BAdminPublicationError::ResultRestLaneCount);
    }
    if pre.counters.rest.len() != post.counters.rest.len() {
        return Err(BAdminPublicationError::CounterRestLaneCount);
    }
    Ok(())
}

fn check_variable(operand: &Operand, temp_prefix: &str) -> Result<(), BAdminPublicationError> {
    if let Operand::Variable(id) = operand {
        if !id.0.starts_with('%') {
            return Err(ConstructionError::UnprefixedVariable.into());
        }
        if id.0.starts_with(temp_prefix) {
            return Err(ConstructionError::TemporaryNameCollision.into());
        }
    }
    Ok(())
}

fn constrain_outcome_running(b: &mut Emitter, enable: &Operand, outcome: &BOutcomeLanes) {
    b.gated_eq(enable, outcome.tag.clone(), immediate(OUTCOME_TAG_RUNNING));
    b.gated_eq(enable, outcome.halted_value.space.clone(), immediate(0));
    b.gated_eq(enable, outcome.halted_value.index.clone(), immediate(0));
    b.gated_eq(enable, outcome.rejected_code.clone(), immediate(0));
    b.gated_eq(enable, outcome.exhausted_reason.clone(), immediate(0));
}

fn constrain_outcome_eq(b: &mut Emitter, pre: &BOutcomeLanes, post: &BOutcomeLanes) {
    for (p, q) in outcome_operands(pre).zip(outcome_operands(post)) {
        b.eq(q.clone(), p.clone());
    }
}

fn constrain_helper_absent(
    b: &mut Emitter,
    enable: &Operand,
    helper: &BScalarArithmeticRegister,
) {
    for col in helper_operands(helper) {
        b.gated_eq(enable, col.clone(), immediate(0));
    }
}

fn constrain_helper_eq(
    b: &mut Emitter,
    pre: &BScalarArithmeticRegister,
    post: &BScalarArithmeticRegister,
) {
    for (p, q) in helper_operands(pre).zip(helper_operands(post)) {
        b.eq(q.clone(), p.clone());
    }
}

fn constrain_result_absent(b: &mut Emitter, enable: &Operand, result: &BHelperResultLanes) {
    for col in result_operands(result) {
        b.gated_eq(enable, col.clone(), immediate(0));
    }
}

fn constrain_result_eq(b: &mut Emitter, pre: &BHelperResultLanes, post: &BHelperResultLanes) {
    for (p, q) in result_operands(pre).zip(result_operands(post)) {
        b.eq(q.clone(), p.clone());
    }
}

fn constrain_control_eq(b: &mut Emitter, pre: &BControlRow, post: &BControlRow) {
    for (p, q) in control_operands(pre).zip(control_operands(post)) {
        b.eq(q.clone(), p.clone());
    }
}

fn constrain_data_table_preserved(
    b: &mut Emitter,
    enable: &Operand,
    pre: &BDataTableLanes,
    post: &BDataTableLanes,
) {
    let cap_u128 = immediate(pre.capacity as u128);
    let eff_pre_active = eff_uint128(b, enable, &pre.active_rows);
    let pre_exceeds_cap = b.less_than(cap_u128, eff_pre_active.clone());
    b.eq(pre_exceeds_cap, immediate(0));
    b.eq(post.active_rows.clone(), pre.active_rows.clone());

    for row in 0..pre.capacity {
        let row_idx = immediate(row as u128);
        let pre_row = &pre.rows[row];
        let post_row = &post.rows[row];

        let in_pre_prefix = b.less_than(row_idx, eff_pre_active.clone());
        let not_in_pre_prefix = b.select(in_pre_prefix, immediate(0), immediate(1));
        b.boolean(not_in_pre_prefix.clone());
        let row_inactive = b.select(enable.clone(), not_in_pre_prefix, immediate(0));
        b.boolean(row_inactive.clone());

        for col in data_row_operands(pre_row) {
            b.gated_eq(&row_inactive, col.clone(), immediate(0));
        }
        for i in 0..16 {
            let pre_byte = b.select(enable.clone(), pre_row.n[i].clone(), immediate(0));
            b.bits(pre_byte, 8);
        }
        for (p, q) in data_row_operands(pre_row).zip(data_row_operands(post_row)) {
            b.eq(q.clone(), p.clone());
        }
    }
}

fn constrain_frames_preserved_with_prefix(
    b: &mut Emitter,
    enable: &Operand,
    pre_depth: &Operand,
    post_depth: &Operand,
    pre_frames: &[BFrameRow],
    post_frames: &[BFrameRow],
) {
    let cap = pre_frames.len();
    let eff_depth = eff_uint128(b, enable, pre_depth);
    b.eq(post_depth.clone(), pre_depth.clone());

    let raw_d0 = b.test_eq(eff_depth.clone(), immediate(0));
    let mut reached_depth = b.select(enable.clone(), raw_d0, immediate(0));
    b.boolean(reached_depth.clone());

    for i in 0..cap {
        for (p, q) in frame_operands(&pre_frames[i]).zip(frame_operands(&post_frames[i])) {
            b.eq(q.clone(), p.clone());
        }
        for col in frame_operands(&pre_frames[i]) {
            b.gated_eq(&reached_depth, col.clone(), immediate(0));
        }
        let next_raw = b.test_eq(eff_depth.clone(), immediate((i + 1) as u128));
        let next_en = b.select(enable.clone(), next_raw, immediate(0));
        b.boolean(next_en.clone());
        reached_depth = b.add(reached_depth, next_en);
    }
    b.eq(reached_depth, enable.clone());
}

fn constrain_ref_cursor_scalar_matches_bytes(
    b: &mut Emitter,
    scalar: &Operand,
    bytes: &BU128Lanes,
) {
    for lane in bytes {
        b.bits(lane.clone(), 8);
    }
    let mut acc = bytes[15].clone();
    for i in (0..15).rev() {
        let shifted = b.mul(acc, immediate(256));
        acc = b.add(shifted, bytes[i].clone());
    }
    b.eq(scalar.clone(), acc);
}

fn constrain_non_work_counters_eq(
    b: &mut Emitter,
    pre: &BScalarCompletionCounters,
    post: &BScalarCompletionCounters,
) {
    for (p, q) in [
        (&pre.reference_cursor, &post.reference_cursor),
        (&pre.alloc_data_nodes, &post.alloc_data_nodes),
        (&pre.alloc_data_fields, &post.alloc_data_fields),
        (&pre.alloc_environment_cells, &post.alloc_environment_cells),
        (&pre.depth, &post.depth),
        (&pre.task_live, &post.task_live),
        (&pre.active_data_fields, &post.active_data_fields),
        (&pre.active_env, &post.active_env),
        (&pre.active_value_slots, &post.active_value_slots),
    ]
    .into_iter()
    .chain(pre.rest.iter().zip(post.rest.iter()))
    {
        b.eq(q.clone(), p.clone());
    }
}

fn constrain_zero_charge_counters_and_ref_cursor(
    b: &mut Emitter,
    enable: &Operand,
    pre: &BAdminPublishState,
    post: &BAdminPublishState,
) {
    let _ = eff_bytes(b, enable, &pre.counters.work.remaining_work);
    let _ = eff_bytes(b, enable, &pre.counters.work.consumed_work);
    let _ = eff_bytes(b, enable, &pre.counters.work.helper_work);
    let _ = eff_bytes(b, enable, &pre.counters.work.helper_visits);

    for i in 0..16 {
        b.eq(
            post.counters.work.remaining_work[i].clone(),
            pre.counters.work.remaining_work[i].clone(),
        );
        b.eq(
            post.counters.work.consumed_work[i].clone(),
            pre.counters.work.consumed_work[i].clone(),
        );
        b.eq(
            post.counters.work.helper_work[i].clone(),
            pre.counters.work.helper_work[i].clone(),
        );
        b.eq(
            post.counters.work.helper_visits[i].clone(),
            pre.counters.work.helper_visits[i].clone(),
        );
        b.eq(
            post.reference_cursor_bytes[i].clone(),
            pre.reference_cursor_bytes[i].clone(),
        );
    }

    constrain_ref_cursor_scalar_matches_bytes(
        b,
        &pre.counters.reference_cursor,
        &pre.reference_cursor_bytes,
    );
    constrain_ref_cursor_scalar_matches_bytes(
        b,
        &post.counters.reference_cursor,
        &post.reference_cursor_bytes,
    );
    constrain_non_work_counters_eq(b, &pre.counters, &post.counters);
}

fn data_row_operands(row: &BDataRowLanes) -> impl Iterator<Item = &Operand> {
    std::iter::once(&row.tag)
        .chain(row.n.iter())
        .chain([&row.bool_b, &row.fields_start, &row.fields_len])
        .chain(row.rest.iter())
}

fn data_table_operands(table: &BDataTableLanes) -> impl Iterator<Item = &Operand> {
    std::iter::once(&table.active_rows).chain(table.rows.iter().flat_map(data_row_operands))
}

fn frame_operands(frame: &BFrameRow) -> impl Iterator<Item = &Operand> {
    [
        &frame.tag,
        &frame.origin,
        &frame.right_op,
        &frame.right_left.space,
        &frame.right_left.index,
    ]
    .into_iter()
    .chain(frame.rest.iter())
}

fn control_operands(ctrl: &BControlRow) -> impl Iterator<Item = &Operand> {
    [
        &ctrl.tag,
        &ctrl.return_value.space,
        &ctrl.return_value.index,
        &ctrl.prim_outcome_tag,
        &ctrl.prim_value.space,
        &ctrl.prim_value.index,
        &ctrl.prim_error_code,
        &ctrl.error_code,
    ]
    .into_iter()
    .chain(ctrl.rest.iter())
}

fn helper_operands(helper: &BScalarArithmeticRegister) -> impl Iterator<Item = &Operand> {
    [
        &helper.present,
        &helper.kind,
        &helper.site,
        &helper.mode,
        &helper.phase,
        &helper.operand_a.space,
        &helper.operand_a.index,
        &helper.operand_b.present,
        &helper.operand_b.value.space,
        &helper.operand_b.value.index,
    ]
    .into_iter()
    .chain(helper.reg_a.iter())
    .chain(std::iter::once(&helper.staged.tag))
    .chain(helper.staged.u128_value.iter())
    .chain([
        &helper.staged.bool_value,
        &helper.staged.pending_node_tag,
        &helper.staged.prim_error_code,
        &helper.staged.selector,
        &helper.staged.type_test_ok,
    ])
    .chain(helper.rest.iter())
}

fn result_operands(result: &BHelperResultLanes) -> impl Iterator<Item = &Operand> {
    [
        &result.present,
        &result.tag,
        &result.scalar_value.space,
        &result.scalar_value.index,
        &result.scalar_error_code,
        &result.scalar_selector,
        &result.scalar_type_test_ok,
    ]
    .into_iter()
    .chain(result.rest.iter())
}

fn outcome_operands(outcome: &BOutcomeLanes) -> impl Iterator<Item = &Operand> {
    [
        &outcome.tag,
        &outcome.halted_value.space,
        &outcome.halted_value.index,
        &outcome.rejected_code,
        &outcome.exhausted_reason,
    ]
    .into_iter()
}

fn completion_counter_operands(
    counters: &BScalarCompletionCounters,
) -> impl Iterator<Item = &Operand> {
    [
        &counters.work.remaining_work,
        &counters.work.consumed_work,
        &counters.work.helper_work,
        &counters.work.helper_visits,
    ]
    .into_iter()
    .flatten()
    .chain([
        &counters.reference_cursor,
        &counters.alloc_data_nodes,
        &counters.alloc_data_fields,
        &counters.alloc_environment_cells,
        &counters.depth,
        &counters.task_live,
        &counters.active_data_fields,
        &counters.active_env,
        &counters.active_value_slots,
    ])
    .chain(counters.rest.iter())
}

fn publish_state_operands(state: &BAdminPublishState) -> impl Iterator<Item = &Operand> {
    [&state.site, &state.control_origin]
        .into_iter()
        .chain(outcome_operands(&state.outcome))
        .chain(control_operands(&state.control))
        .chain(helper_operands(&state.helper))
        .chain(result_operands(&state.result))
        .chain(data_table_operands(&state.data))
        .chain(state.frames.iter().flat_map(frame_operands))
        .chain(completion_counter_operands(&state.counters))
        .chain(state.reference_cursor_bytes.iter())
}

struct Emitter {
    scope: String,
    serial: usize,
    code: Vec<Instruction>,
}

impl Emitter {
    fn fresh(&mut self) -> Identifier {
        let id = Identifier(format!("%{}.local.temp.{}", self.scope, self.serial));
        self.serial += 1;
        id
    }

    fn select(&mut self, bit: Operand, a: Operand, b: Operand) -> Operand {
        let output = self.fresh();
        self.code.push(Instruction::CondSelect {
            bit,
            a,
            b,
            output: output.clone(),
        });
        Operand::Variable(output)
    }

    fn eq(&mut self, a: Operand, b: Operand) {
        self.code.push(Instruction::ConstrainEq { a, b });
    }

    fn gated_eq(&mut self, enable: &Operand, a: Operand, b: Operand) {
        let guarded = self.select(enable.clone(), a, b.clone());
        self.eq(guarded, b);
    }

    fn boolean(&mut self, val: Operand) {
        self.code.push(Instruction::ConstrainToBoolean { val });
    }

    fn bits(&mut self, val: Operand, bits: u32) {
        self.code.push(Instruction::ConstrainBits { val, bits });
    }

    fn assert(&mut self, cond: Operand) {
        self.code.push(Instruction::Assert { cond });
    }

    fn test_eq(&mut self, a: Operand, other: Operand) -> Operand {
        let output = self.fresh();
        self.code.push(Instruction::TestEq {
            a,
            b: other,
            output: output.clone(),
        });
        let value = Operand::Variable(output);
        self.boolean(value.clone());
        value
    }

    fn less_than(&mut self, a: Operand, b: Operand) -> Operand {
        let output = self.fresh();
        self.code.push(Instruction::LessThan {
            a,
            b,
            bits: 128,
            output: output.clone(),
        });
        let value = Operand::Variable(output);
        self.boolean(value.clone());
        value
    }

    fn add(&mut self, a: Operand, b: Operand) -> Operand {
        let output = self.fresh();
        self.code.push(Instruction::Add {
            a,
            b,
            output: output.clone(),
        });
        Operand::Variable(output)
    }

    fn mul(&mut self, a: Operand, b: Operand) -> Operand {
        let output = self.fresh();
        self.code.push(Instruction::Mul {
            a,
            b,
            output: output.clone(),
        });
        Operand::Variable(output)
    }
}

fn eff_uint128(b: &mut Emitter, enable: &Operand, val: &Operand) -> Operand {
    let eff = b.select(enable.clone(), val.clone(), immediate(0));
    b.bits(eff.clone(), 128);
    eff
}

fn eff_bytes(b: &mut Emitter, enable: &Operand, lanes: &BU128Lanes) -> BU128Lanes {
    std::array::from_fn(|i| {
        let value = b.select(enable.clone(), lanes[i].clone(), immediate(0));
        b.bits(value.clone(), 8);
        value
    })
}
