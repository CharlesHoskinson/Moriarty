//! Handwritten Bool completion for RetRight / Lt(mode3) / Done.
//! BoolValue appends a DataRow::Bool through the normal Scalar.exit P6 branch,
//! or preserves the helper on DataNodes cap refusal. Scalar.failure transfers
//! only defined sort error11; Lt has no overflow10. Logical allocation and
//! physical active-data capacity remain distinct, with the inherited P4 work
//! charges and preserved-counter/complement relations.
//!
//! Public carriers are nominal Lt copies with explicit conversion to existing
//! common publication/legacy carriers. This does not connect the Add/Lt/Mul
//! return-right or P1 composers: a genuine op4 Lt return-right and consumed Bool
//! publication chain, parent registration and full stage input shapes remain
//! future work. No cast or nominal admission is implied.
//!
//! Disabled row post lanes remain free; compositional stutter/mux and global
//! per-IrSource namespace/admission/StepInv/physical/source binding are caller
//! duties. This new Sol source is unregistered, uncompiled and unrun and requires
//! fresh independent Opus correctness/security review before any execution.

use super::b_admin_publication::BAdminPublishState;
use super::b_return_right_completion::{BControlRow, BFrameRow, BLegacyCompletionState};
use super::b_scalar_arithmetic::{
    BChargedCounters as BCommonChargedCounters, BOptValueRef,
    BScalarArithmeticRegister as BCommonArithmeticRegister, BScalarStagedLanes, BValueRef,
};
use super::b_scalar_completion::{
    BDataRowLanes, BDataTableLanes, BHelperResultLanes, BOutcomeLanes,
    BScalarCompletionCounters as BCommonCompletionCounters,
};
use super::b_scalar_less_than::{
    BChargedCounters, BOptValueRef as BLtOptValueRef, BScalarArithmeticRegister,
    BScalarStagedLanes as BLtStagedLanes, BValueRef as BLtValueRef, SCALAR_MODE_LT,
};
use super::*;

pub const DATA_TAG_BOOL: u128 = 1; // schema DataTag[1] Bool

pub const VALUE_SPACE_CONST: u128 = 0; // records.ValueRef space bit 0 = Const
pub const VALUE_SPACE_DATA: u128 = 1; // records.ValueRef space bit 1 = Data

pub const HELPER_RESULT_TAG_VALIDATED: u128 = 0; // enums.HelperResultTag[0]
pub const HELPER_RESULT_TAG_ENV_FOUND: u128 = 1; // enums.HelperResultTag[1]
pub const HELPER_RESULT_TAG_ENV_MISSING: u128 = 2; // enums.HelperResultTag[2]
pub const HELPER_RESULT_TAG_ENV_UPDATED: u128 = 3; // enums.HelperResultTag[3]
pub const HELPER_RESULT_TAG_FIND_FOUND: u128 = 4; // enums.HelperResultTag[4]
pub const HELPER_RESULT_TAG_FIND_MISSING: u128 = 5; // enums.HelperResultTag[5]
pub const HELPER_RESULT_TAG_BIND_OK: u128 = 6; // enums.HelperResultTag[6]
pub const HELPER_RESULT_TAG_BIND_BAD: u128 = 7; // enums.HelperResultTag[7]
pub const HELPER_RESULT_TAG_APPEND_DONE: u128 = 8; // enums.HelperResultTag[8]
pub const HELPER_RESULT_TAG_SCALAR_VALUE: u128 = 9; // enums.HelperResultTag[9]
pub const HELPER_RESULT_TAG_SCALAR_ERROR: u128 = 10; // enums.HelperResultTag[10]
pub const HELPER_RESULT_TAG_SCALAR_SELECTOR: u128 = 11; // enums.HelperResultTag[11]
pub const HELPER_RESULT_TAG_SCALAR_TYPE_TEST: u128 = 12; // enums.HelperResultTag[12]
pub const HELPER_RESULT_TAG_EQUAL_DONE: u128 = 13; // enums.HelperResultTag[13]
pub const HELPER_RESULT_TAG_MATCH_MATCHED: u128 = 14; // enums.HelperResultTag[14]
pub const HELPER_RESULT_TAG_MATCH_NO_MATCH: u128 = 15; // enums.HelperResultTag[15]

pub const OUTCOME_TAG_RUNNING: u128 = 0; // enums.OutcomeTag[0] = "Running"
pub const OUTCOME_TAG_HALTED: u128 = 1; // enums.OutcomeTag[1] = "Halted"
pub const OUTCOME_TAG_REJECTED: u128 = 2; // enums.OutcomeTag[2] = "Rejected"
pub const OUTCOME_TAG_EXHAUSTED: u128 = 3; // enums.OutcomeTag[3] = "Exhausted"

pub const EXHAUST_REASON_FUEL: u128 = 0; // enums.ExhaustReason[0] = "Fuel"
pub const EXHAUST_REASON_DATA_NODES: u128 = 1; // enums.ExhaustReason[1] = "DataNodes"
pub const EXHAUST_REASON_DATA_FIELDS: u128 = 2; // enums.ExhaustReason[2] = "DataFields"
pub const EXHAUST_REASON_ENVIRONMENT_CELLS: u128 = 3; // enums.ExhaustReason[3] = "EnvironmentCells"
pub const EXHAUST_REASON_HELPER_TASKS: u128 = 4; // enums.ExhaustReason[4] = "HelperTasks"
pub const EXHAUST_REASON_HELPER_WORK: u128 = 5; // enums.ExhaustReason[5] = "HelperWork"
pub const EXHAUST_REASON_SERIALIZATION_BYTES: u128 = 6; // enums.ExhaustReason[6] = "SerializationBytes"

/// Complete step counter/cursor view covering `/delta_order` and active arena
/// lengths outside `img.data.active_rows`:
/// - `work`: `[remainingWork, consumedWork, helperWork, helperVisits]` as 16-byte LE lanes
/// - `reference_cursor`: `reg.referenceCursor`
/// - `alloc_data_nodes`: `reg.alloc.dataNodes` (logical monotone counter compared against `bounds.dataNodes` at P6)
/// - `alloc_data_fields`: `reg.alloc.dataFields`
/// - `alloc_environment_cells`: `reg.alloc.environmentCells`
/// - `depth`: `reg.depth`
/// - `task_live`: `reg.taskLive`
/// - `active_data_fields`: `reg.active.dataFields`
/// - `active_env`: `reg.active.env`
/// - `active_value_slots`: `reg.active.valueSlots`
/// - `rest`: any remaining register/counter lanes in identical pre/post order.
#[derive(Clone, Debug, PartialEq)]
pub struct BScalarLtCompletionCounters {
    pub work: BChargedCounters,
    pub reference_cursor: Operand,
    pub alloc_data_nodes: Operand,
    pub alloc_data_fields: Operand,
    pub alloc_environment_cells: Operand,
    pub depth: Operand,
    pub task_live: Operand,
    pub active_data_fields: Operand,
    pub active_env: Operand,
    pub active_value_slots: Operand,
    pub rest: Vec<Operand>,
}

/// Caller-supplied witness inputs for `b.helper.Scalar.exit` on branch
/// `RetRight / Lt / Done / BoolValue`.
#[derive(Clone, Debug, PartialEq)]
pub struct BScalarExitLtBoolInputs {
    pub enable: Operand,
    pub bounds_data_nodes: Operand,
    pub pre_data: BDataTableLanes,
    pub post_data: BDataTableLanes,
    pub pre_helper: BScalarArithmeticRegister,
    pub post_helper: BScalarArithmeticRegister,
    pub pre_result: BHelperResultLanes,
    pub post_result: BHelperResultLanes,
    pub pre_outcome: BOutcomeLanes,
    pub post_outcome: BOutcomeLanes,
    pub pre_counters: BScalarLtCompletionCounters,
    pub post_counters: BScalarLtCompletionCounters,
}

/// Composable instruction fragment for `b.helper.Scalar.exit` (`RetRight/Lt/Done/BoolValue`).
#[derive(Clone, Debug, PartialEq)]
pub struct BScalarExitLtBoolBlock {
    pub instructions: Vec<Instruction>,
    pub applied: Operand,
    pub cap_refused: Operand,
    pub allocated_index: Operand,
    pub allocated_bool: Operand,
    pub write_selectors: Vec<Operand>,
    pub preserved: BScalarExitLtBoolInputs,
}

/// Caller-supplied witness inputs for `b.helper.Scalar.failure` on branch
/// `RetRight / Lt / Done / PrimError(11)`.
#[derive(Clone, Debug, PartialEq)]
pub struct BScalarFailureLtInputs {
    pub enable: Operand,
    pub pre_data: BDataTableLanes,
    pub post_data: BDataTableLanes,
    pub pre_helper: BScalarArithmeticRegister,
    pub post_helper: BScalarArithmeticRegister,
    pub pre_result: BHelperResultLanes,
    pub post_result: BHelperResultLanes,
    pub pre_outcome: BOutcomeLanes,
    pub post_outcome: BOutcomeLanes,
    pub pre_counters: BScalarLtCompletionCounters,
    pub post_counters: BScalarLtCompletionCounters,
}

/// Composable instruction fragment for `b.helper.Scalar.failure` (`RetRight/Lt/Done/PrimError`).
#[derive(Clone, Debug, PartialEq)]
pub struct BScalarFailureLtBlock {
    pub instructions: Vec<Instruction>,
    pub error_code: Operand,
    pub is_sort_mismatch_11: Operand,
    pub preserved: BScalarFailureLtInputs,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BScalarLtCompletionConstructionError {
    DataRowCount,
    DataRowRestLaneCount,
    HelperRestLaneCount,
    ResultRestLaneCount,
    CounterRestLaneCount,
    FrameRowCount,
    FrameRestLaneCount,
    ControlRestLaneCount,
    LegacyRestLaneCount,
    Fetch(ConstructionError),
}

impl From<ConstructionError> for BScalarLtCompletionConstructionError {
    fn from(e: ConstructionError) -> Self {
        Self::Fetch(e)
    }
}

/// Emit the in-circuit relation for `b.helper.Scalar.exit` on branch
/// `site = RetRight`, `mode = Lt`, `phase = Done`, `staged = BoolValue`,
/// including P6 logical `DataNodes` cap check (`Applied` vs `RefusedCapCharged(DataNodes)`).
pub fn emit_b_scalar_exit_lt_bool(
    input: &BScalarExitLtBoolInputs,
    scope: &str,
) -> Result<BScalarExitLtBoolBlock, BScalarLtCompletionConstructionError> {
    validate_common_shapes(
        scope,
        &input.pre_data,
        &input.post_data,
        &input.pre_helper,
        &input.post_helper,
        &input.pre_result,
        &input.post_result,
        &input.pre_counters,
        &input.post_counters,
    )?;
    let temp_prefix = format!("%{scope}.local.temp.");
    for operand in [&input.enable, &input.bounds_data_nodes]
        .into_iter()
        .chain(data_table_operands(&input.pre_data))
        .chain(data_table_operands(&input.post_data))
        .chain(helper_operands(&input.pre_helper))
        .chain(helper_operands(&input.post_helper))
        .chain(result_operands(&input.pre_result))
        .chain(result_operands(&input.post_result))
        .chain(outcome_operands(&input.pre_outcome))
        .chain(outcome_operands(&input.post_outcome))
        .chain(completion_counter_operands(&input.pre_counters))
        .chain(completion_counter_operands(&input.post_counters))
    {
        check_variable(operand, &temp_prefix)?;
    }

    let mut b = Emitter {
        scope: scope.into(),
        serial: 0,
        code: Vec::new(),
    };
    let enable = &input.enable;
    b.boolean(enable.clone());

    // E01: Pre-state branch guards for RetRight / Lt / Done / BoolValue,
    // Running outcome, and absent pre.result.
    for (pre, expected) in [
        (&input.pre_helper.present, HELPER_PRESENT),
        (&input.pre_helper.kind, HELPER_KIND_SCALAR),
        (&input.pre_helper.site, SITE_RET_RIGHT),
        (&input.pre_helper.mode, SCALAR_MODE_LT),
        (&input.pre_helper.phase, SCALAR_PHASE_DONE),
        (&input.pre_helper.operand_b.present, OPT_PRESENT),
        (&input.pre_helper.staged.tag, SCALAR_STAGED_TAG_BOOL_VALUE),
        (&input.pre_helper.staged.pending_node_tag, 0),
        (&input.pre_helper.staged.prim_error_code, 0),
        (&input.pre_helper.staged.selector, 0),
        (&input.pre_helper.staged.type_test_ok, 0),
    ] {
        b.gated_eq(enable, pre.clone(), immediate(expected));
    }
    constrain_outcome_running(&mut b, enable, &input.pre_outcome);
    constrain_result_absent(&mut b, enable, &input.pre_result);

    let eff_reg_a = eff_bytes(&mut b, enable, &input.pre_helper.reg_a);
    let eff_staged_u128 = eff_bytes(&mut b, enable, &input.pre_helper.staged.u128_value);
    for byte in &eff_staged_u128 { b.eq(byte.clone(), immediate(0)); }
    let eff_staged_bool = b.select(enable.clone(), input.pre_helper.staged.bool_value.clone(), immediate(0));
    b.boolean(eff_staged_bool.clone());

    // E02: P6 logical DataNodes cap check:
    // applied = 1 iff enable == 1 and pre.alloc.dataNodes < bounds.dataNodes.
    // cap_refused = 1 iff enable == 1 and pre.alloc.dataNodes >= bounds.dataNodes.
    let eff_pre_alloc_nodes = eff_uint128(&mut b, enable, &input.pre_counters.alloc_data_nodes);
    let eff_post_alloc_nodes = eff_uint128(&mut b, enable, &input.post_counters.alloc_data_nodes);
    let eff_bounds_nodes = eff_uint128(&mut b, enable, &input.bounds_data_nodes);

    let can_alloc = b.less_than(eff_pre_alloc_nodes.clone(), eff_bounds_nodes);
    let applied = b.select(enable.clone(), can_alloc.clone(), immediate(0));
    b.boolean(applied.clone());
    let not_can_alloc = b.select(can_alloc, immediate(0), immediate(1));
    b.boolean(not_can_alloc.clone());
    let cap_refused = b.select(enable.clone(), not_can_alloc, immediate(0));
    b.boolean(cap_refused.clone());
    let branch_sum = b.add(applied.clone(), cap_refused.clone());
    b.eq(branch_sum, enable.clone());

    // E03: Logical alloc.dataNodes delta (+1 on Applied, +0 on RefusedCapCharged).
    let alloc_nodes_plus_inv = b.add(eff_pre_alloc_nodes, applied.clone());
    b.bits(alloc_nodes_plus_inv.clone(), 128);
    b.gated_eq(enable, eff_post_alloc_nodes, alloc_nodes_plus_inv);

    // E04: Physical img.data capacity & active.data delta (+1 on Applied, +0 on RefusedCapCharged).
    // By PS (R2-T1 D-33), when capRefusal is some (cap_refused == 1), P6 writes nothing
    // and no physical capacity check applies; when capRefusal is none (applied == 1),
    // pre.active.data must be strictly below img.data physical capacity.
    let cap_u128 = immediate(input.pre_data.capacity as u128);
    let eff_pre_active_data = eff_uint128(&mut b, enable, &input.pre_data.active_rows);
    let eff_post_active_data = eff_uint128(&mut b, enable, &input.post_data.active_rows);
    let pre_exceeds_cap = b.less_than(cap_u128.clone(), eff_pre_active_data.clone());
    b.eq(pre_exceeds_cap, immediate(0));
    let post_exceeds_cap = b.less_than(cap_u128.clone(), eff_post_active_data.clone());
    b.eq(post_exceeds_cap, immediate(0));

    let in_physical_cap = b.less_than(eff_pre_active_data.clone(), cap_u128);
    let guarded_physical_cap = b.select(applied.clone(), in_physical_cap, immediate(1));
    b.assert(guarded_physical_cap);

    let active_data_plus = b.add(eff_pre_active_data.clone(), applied.clone());
    b.bits(active_data_plus.clone(), 128);
    b.gated_eq(enable, eff_post_active_data, active_data_plus);

    // E05: One-hot write selector at slot r == pre.active.data under applied,
    // prefix preservation for r < pre.active.data, and inactive-row zeroing for r >= active.
    let mut write_selectors = Vec::with_capacity(input.pre_data.capacity);
    let mut write_sum = immediate(0);
    for row in 0..input.pre_data.capacity {
        let row_idx = immediate(row as u128);
        let is_target = b.test_eq(eff_pre_active_data.clone(), row_idx.clone());
        let write_sel = b.select(applied.clone(), is_target, immediate(0));
        b.boolean(write_sel.clone());
        write_sum = b.add(write_sum, write_sel.clone());

        let pre_row = &input.pre_data.rows[row];
        let post_row = &input.post_data.rows[row];

        // Inactive rows in pre_data (row >= pre.active.data) must have all columns == 0.
        let in_pre_prefix = b.less_than(row_idx, eff_pre_active_data.clone());
        let not_in_pre_prefix = b.select(in_pre_prefix, immediate(0), immediate(1));
        b.boolean(not_in_pre_prefix.clone());
        let pre_row_inactive = b.select(enable.clone(), not_in_pre_prefix, immediate(0));
        b.boolean(pre_row_inactive.clone());
        constrain_data_row_zero_when(&mut b, &pre_row_inactive, pre_row);

        // Post row equals newly appended DataRow::Bool{b: staged.bool_value} when write_sel == 1,
        // and equals pre_row otherwise (covering prefix rows, trailing rows, and cap_refused).
        let exp_tag = b.select(write_sel.clone(), immediate(DATA_TAG_BOOL), pre_row.tag.clone());
        b.gated_eq(enable, post_row.tag.clone(), exp_tag);
        for i in 0..16 {
            let pre_byte = b.select(enable.clone(), pre_row.n[i].clone(), immediate(0));
            b.bits(pre_byte.clone(), 8);
            let post_byte = b.select(enable.clone(), post_row.n[i].clone(), immediate(0));
            b.bits(post_byte.clone(), 8);
            let exp_byte = b.select(write_sel.clone(), immediate(0), pre_byte);
            b.eq(post_byte, exp_byte);
        }
        let exp_bool = b.select(write_sel.clone(), eff_staged_bool.clone(), pre_row.bool_b.clone());
        b.gated_eq(enable, post_row.bool_b.clone(), exp_bool);
        let exp_f_start = b.select(
            write_sel.clone(),
            immediate(0),
            pre_row.fields_start.clone(),
        );
        b.gated_eq(enable, post_row.fields_start.clone(), exp_f_start);
        let exp_f_len = b.select(write_sel.clone(), immediate(0), pre_row.fields_len.clone());
        b.gated_eq(enable, post_row.fields_len.clone(), exp_f_len);
        for (pre_rest, post_rest) in pre_row.rest.iter().zip(post_row.rest.iter()) {
            let exp_rest = b.select(write_sel.clone(), immediate(0), pre_rest.clone());
            b.gated_eq(enable, post_rest.clone(), exp_rest);
        }

        write_selectors.push(write_sel);
    }
    b.eq(write_sum, applied.clone());

    // E06: Post reg.result:
    // - Applied: Some(ScalarValue { value: Data(pre.active.data) }), inactive columns 0
    // - RefusedCapCharged: absent (all columns 0, matching pre_result)
    let exp_res_present = b.select(applied.clone(), immediate(OPT_PRESENT), immediate(OPT_ABSENT));
    b.gated_eq(enable, input.post_result.present.clone(), exp_res_present);
    let exp_res_tag = b.select(
        applied.clone(),
        immediate(HELPER_RESULT_TAG_SCALAR_VALUE),
        immediate(0),
    );
    b.gated_eq(enable, input.post_result.tag.clone(), exp_res_tag);
    let exp_res_space = b.select(
        applied.clone(),
        immediate(VALUE_SPACE_DATA),
        immediate(0),
    );
    b.gated_eq(
        enable,
        input.post_result.scalar_value.space.clone(),
        exp_res_space,
    );
    let allocated_index = b.select(applied.clone(), eff_pre_active_data, immediate(0));
    b.gated_eq(
        enable,
        input.post_result.scalar_value.index.clone(),
        allocated_index.clone(),
    );
    for inactive_res in [
        &input.post_result.scalar_error_code,
        &input.post_result.scalar_selector,
        &input.post_result.scalar_type_test_ok,
    ]
    .into_iter()
    .chain(input.post_result.rest.iter())
    {
        b.gated_eq(enable, inactive_res.clone(), immediate(0));
    }

    // E07: Post reg.helper:
    // - Applied: clearHelper (absent + every payload column 0)
    // - RefusedCapCharged: retained identical to pre_helper
    for (pre_lane, post_lane) in [
        (&input.pre_helper.present, &input.post_helper.present),
        (&input.pre_helper.kind, &input.post_helper.kind),
        (&input.pre_helper.site, &input.post_helper.site),
        (&input.pre_helper.mode, &input.post_helper.mode),
        (&input.pre_helper.phase, &input.post_helper.phase),
        (
            &input.pre_helper.operand_a.space,
            &input.post_helper.operand_a.space,
        ),
        (
            &input.pre_helper.operand_a.index,
            &input.post_helper.operand_a.index,
        ),
        (
            &input.pre_helper.operand_b.present,
            &input.post_helper.operand_b.present,
        ),
        (
            &input.pre_helper.operand_b.value.space,
            &input.post_helper.operand_b.value.space,
        ),
        (
            &input.pre_helper.operand_b.value.index,
            &input.post_helper.operand_b.value.index,
        ),
        (&input.pre_helper.staged.tag, &input.post_helper.staged.tag),
        (
            &input.pre_helper.staged.bool_value,
            &input.post_helper.staged.bool_value,
        ),
        (
            &input.pre_helper.staged.pending_node_tag,
            &input.post_helper.staged.pending_node_tag,
        ),
        (
            &input.pre_helper.staged.prim_error_code,
            &input.post_helper.staged.prim_error_code,
        ),
        (
            &input.pre_helper.staged.selector,
            &input.post_helper.staged.selector,
        ),
        (
            &input.pre_helper.staged.type_test_ok,
            &input.post_helper.staged.type_test_ok,
        ),
    ]
    .into_iter()
    .chain(eff_reg_a.iter().zip(input.post_helper.reg_a.iter()))
    .chain(
        eff_staged_u128
            .iter()
            .zip(input.post_helper.staged.u128_value.iter()),
    )
    .chain(
        input
            .pre_helper
            .rest
            .iter()
            .zip(input.post_helper.rest.iter()),
    ) {
        let exp_helper = b.select(applied.clone(), immediate(0), pre_lane.clone());
        b.gated_eq(enable, post_lane.clone(), exp_helper);
    }

    // E08: Post reg.outcome:
    // - Applied: Running (tag = 0, all payload columns 0)
    // - RefusedCapCharged: Exhausted(DataNodes) (tag = 3, exhausted_reason = 1, other columns 0)
    let exp_outcome_tag = b.select(
        cap_refused.clone(),
        immediate(OUTCOME_TAG_EXHAUSTED),
        immediate(OUTCOME_TAG_RUNNING),
    );
    b.gated_eq(enable, input.post_outcome.tag.clone(), exp_outcome_tag);
    let exp_exhaust_reason = b.select(
        cap_refused.clone(),
        immediate(EXHAUST_REASON_DATA_NODES),
        immediate(0),
    );
    b.gated_eq(
        enable,
        input.post_outcome.exhausted_reason.clone(),
        exp_exhaust_reason,
    );
    b.gated_eq(
        enable,
        input.post_outcome.halted_value.space.clone(),
        immediate(0),
    );
    b.gated_eq(
        enable,
        input.post_outcome.halted_value.index.clone(),
        immediate(0),
    );
    b.gated_eq(
        enable,
        input.post_outcome.rejected_code.clone(),
        immediate(0),
    );

    // E09: P4 work charges [-1, +1, +1, 0] and non-data counter preservation.
    constrain_completion_work_and_preserved_counters(
        &mut b,
        enable,
        &input.pre_counters,
        &input.post_counters,
    );

    let allocated_bool = b.select(applied.clone(), eff_staged_bool, immediate(0));
    b.boolean(allocated_bool.clone());

    Ok(BScalarExitLtBoolBlock {
        instructions: b.code,
        applied,
        cap_refused,
        allocated_index,
        allocated_bool,
        write_selectors,
        preserved: input.clone(),
    })
}

/// Emit the in-circuit relation for `b.helper.Scalar.failure` on branch
/// `site = RetRight`, `mode = Lt`, `phase = Done`, `staged = PrimError(11)`.
pub fn emit_b_scalar_failure_lt(
    input: &BScalarFailureLtInputs,
    scope: &str,
) -> Result<BScalarFailureLtBlock, BScalarLtCompletionConstructionError> {
    validate_common_shapes(
        scope,
        &input.pre_data,
        &input.post_data,
        &input.pre_helper,
        &input.post_helper,
        &input.pre_result,
        &input.post_result,
        &input.pre_counters,
        &input.post_counters,
    )?;
    let temp_prefix = format!("%{scope}.local.temp.");
    for operand in [&input.enable]
        .into_iter()
        .chain(data_table_operands(&input.pre_data))
        .chain(data_table_operands(&input.post_data))
        .chain(helper_operands(&input.pre_helper))
        .chain(helper_operands(&input.post_helper))
        .chain(result_operands(&input.pre_result))
        .chain(result_operands(&input.post_result))
        .chain(outcome_operands(&input.pre_outcome))
        .chain(outcome_operands(&input.post_outcome))
        .chain(completion_counter_operands(&input.pre_counters))
        .chain(completion_counter_operands(&input.post_counters))
    {
        check_variable(operand, &temp_prefix)?;
    }

    let mut b = Emitter {
        scope: scope.into(),
        serial: 0,
        code: Vec::new(),
    };
    let enable = &input.enable;
    b.boolean(enable.clone());

    // F01: Pre-state branch guards for RetRight / Lt / Done / PrimError(11),
    // Running outcome, absent pre.result, and zero inactive staged columns.
    for (pre, expected) in [
        (&input.pre_helper.present, HELPER_PRESENT),
        (&input.pre_helper.kind, HELPER_KIND_SCALAR),
        (&input.pre_helper.site, SITE_RET_RIGHT),
        (&input.pre_helper.mode, SCALAR_MODE_LT),
        (&input.pre_helper.phase, SCALAR_PHASE_DONE),
        (&input.pre_helper.operand_b.present, OPT_PRESENT),
        (&input.pre_helper.staged.tag, SCALAR_STAGED_TAG_PRIM_ERROR),
        (&input.pre_helper.staged.bool_value, 0),
        (&input.pre_helper.staged.pending_node_tag, 0),
        (&input.pre_helper.staged.selector, 0),
        (&input.pre_helper.staged.type_test_ok, 0),
    ] {
        b.gated_eq(enable, pre.clone(), immediate(expected));
    }
    for byte_lane in &input.pre_helper.staged.u128_value {
        b.gated_eq(enable, byte_lane.clone(), immediate(0));
    }
    let _eff_reg_a = eff_bytes(&mut b, enable, &input.pre_helper.reg_a);
    constrain_outcome_running(&mut b, enable, &input.pre_outcome);
    constrain_result_absent(&mut b, enable, &input.pre_result);

    // F02: Lt's sole defined primitive failure is SortMismatch(11).
    b.gated_eq(enable, input.pre_helper.staged.prim_error_code.clone(), immediate(PRIM_ERROR_SORT_MISMATCH));
    let error_code = b.select(enable.clone(), immediate(PRIM_ERROR_SORT_MISMATCH), immediate(0));
    b.bits(error_code.clone(), 4);
    let is_sort_mismatch_11 = enable.clone();

    // F03: Post reg.result = Some(ScalarError { code }) with all inactive columns 0.
    b.gated_eq(
        enable,
        input.post_result.present.clone(),
        immediate(OPT_PRESENT),
    );
    b.gated_eq(
        enable,
        input.post_result.tag.clone(),
        immediate(HELPER_RESULT_TAG_SCALAR_ERROR),
    );
    b.gated_eq(
        enable,
        input.post_result.scalar_error_code.clone(),
        error_code.clone(),
    );
    for inactive_res in [
        &input.post_result.scalar_value.space,
        &input.post_result.scalar_value.index,
        &input.post_result.scalar_selector,
        &input.post_result.scalar_type_test_ok,
    ]
    .into_iter()
    .chain(input.post_result.rest.iter())
    {
        b.gated_eq(enable, inactive_res.clone(), immediate(0));
    }

    // F04: clearHelper (absent + every payload column 0) and outcome Running preserved.
    for post_lane in helper_operands(&input.post_helper) {
        b.gated_eq(enable, post_lane.clone(), immediate(0));
    }
    constrain_outcome_running(&mut b, enable, &input.post_outcome);

    // F05: No Data allocation: active.data, alloc.dataNodes, and all img.data rows preserved.
    let cap_u128 = immediate(input.pre_data.capacity as u128);
    let eff_pre_active_data = eff_uint128(&mut b, enable, &input.pre_data.active_rows);
    let eff_post_active_data = eff_uint128(&mut b, enable, &input.post_data.active_rows);
    let pre_exceeds_cap = b.less_than(cap_u128, eff_pre_active_data.clone());
    b.eq(pre_exceeds_cap, immediate(0));
    b.gated_eq(enable, eff_post_active_data, eff_pre_active_data.clone());

    let eff_pre_alloc_nodes = eff_uint128(&mut b, enable, &input.pre_counters.alloc_data_nodes);
    let eff_post_alloc_nodes = eff_uint128(&mut b, enable, &input.post_counters.alloc_data_nodes);
    b.gated_eq(enable, eff_post_alloc_nodes, eff_pre_alloc_nodes);

    for row in 0..input.pre_data.capacity {
        let row_idx = immediate(row as u128);
        let pre_row = &input.pre_data.rows[row];
        let post_row = &input.post_data.rows[row];

        let in_pre_prefix = b.less_than(row_idx, eff_pre_active_data.clone());
        let not_in_pre_prefix = b.select(in_pre_prefix, immediate(0), immediate(1));
        b.boolean(not_in_pre_prefix.clone());
        let pre_row_inactive = b.select(enable.clone(), not_in_pre_prefix, immediate(0));
        b.boolean(pre_row_inactive.clone());
        constrain_data_row_zero_when(&mut b, &pre_row_inactive, pre_row);

        b.gated_eq(enable, post_row.tag.clone(), pre_row.tag.clone());
        for i in 0..16 {
            let pre_byte = b.select(enable.clone(), pre_row.n[i].clone(), immediate(0));
            b.bits(pre_byte.clone(), 8);
            let post_byte = b.select(enable.clone(), post_row.n[i].clone(), immediate(0));
            b.bits(post_byte.clone(), 8);
            b.eq(post_byte, pre_byte);
        }
        b.gated_eq(enable, post_row.bool_b.clone(), pre_row.bool_b.clone());
        b.gated_eq(
            enable,
            post_row.fields_start.clone(),
            pre_row.fields_start.clone(),
        );
        b.gated_eq(
            enable,
            post_row.fields_len.clone(),
            pre_row.fields_len.clone(),
        );
        for (pre_rest, post_rest) in pre_row.rest.iter().zip(post_row.rest.iter()) {
            b.gated_eq(enable, post_rest.clone(), pre_rest.clone());
        }
    }

    // F06: P4 work charges [-1, +1, +1, 0] and non-data counter preservation.
    constrain_completion_work_and_preserved_counters(
        &mut b,
        enable,
        &input.pre_counters,
        &input.post_counters,
    );

    Ok(BScalarFailureLtBlock {
        instructions: b.code,
        error_code,
        is_sort_mismatch_11,
        preserved: input.clone(),
    })
}

/// Lt-carrier mirror of `BAdminPublishState`, in the same field order: `helper` is the
/// actual `b_scalar_less_than` register and `counters.work` its work counters; every
/// other lane is the existing common carrier.
#[derive(Clone, Debug, PartialEq)]
pub struct BLtPublishState {
    pub site: Operand,
    pub outcome: BOutcomeLanes,
    pub control: BControlRow,
    pub control_origin: Operand,
    pub helper: BScalarArithmeticRegister,
    pub result: BHelperResultLanes,
    pub data: BDataTableLanes,
    pub frames: Vec<BFrameRow>,
    pub counters: BScalarLtCompletionCounters,
    pub reference_cursor_bytes: BU128Lanes,
}

/// `BLegacyCompletionState` plus the `BAdminPublishState` lanes it does not carry
/// (outcome payload, `img.data`, scalar `referenceCursor`), so the handoff inverts to
/// `BAdminPublishState` only; there is no map back to `BLtPublishState`.
/// `state.rest` is `[alloc_data_nodes, alloc_data_fields, alloc_environment_cells,
/// task_live, active_data_fields, active_env, active_value_slots] ++ counters.rest`,
/// the order of the `b_admin_publication` chain bridge.
#[derive(Clone, Debug, PartialEq)]
pub struct BLtLegacyHandoff {
    pub state: BLegacyCompletionState,
    pub outcome_halted_value: BValueRef,
    pub outcome_rejected_code: Operand,
    pub outcome_exhausted_reason: Operand,
    pub data: BDataTableLanes,
    pub reference_cursor: Operand,
}

/// Moves every Lt helper lane (exhaustive destructuring) into the common register
/// type. No lane is re-derived: `mode` keeps the Lt ordinal and `rest` its order.
pub fn lt_helper_to_common(helper: &BScalarArithmeticRegister) -> BCommonArithmeticRegister {
    let BScalarArithmeticRegister {
        present,
        kind,
        site,
        mode,
        phase,
        operand_a: BLtValueRef { space: a_space, index: a_index },
        operand_b:
            BLtOptValueRef {
                present: b_present,
                value: BLtValueRef { space: b_space, index: b_index },
            },
        reg_a,
        staged:
            BLtStagedLanes {
                tag,
                u128_value,
                bool_value,
                pending_node_tag,
                prim_error_code,
                selector,
                type_test_ok,
            },
        rest,
    } = helper;
    BCommonArithmeticRegister {
        present: present.clone(),
        kind: kind.clone(),
        site: site.clone(),
        mode: mode.clone(),
        phase: phase.clone(),
        operand_a: BValueRef {
            space: a_space.clone(),
            index: a_index.clone(),
        },
        operand_b: BOptValueRef {
            present: b_present.clone(),
            value: BValueRef {
                space: b_space.clone(),
                index: b_index.clone(),
            },
        },
        reg_a: reg_a.clone(),
        staged: BScalarStagedLanes {
            tag: tag.clone(),
            u128_value: u128_value.clone(),
            bool_value: bool_value.clone(),
            pending_node_tag: pending_node_tag.clone(),
            prim_error_code: prim_error_code.clone(),
            selector: selector.clone(),
            type_test_ok: type_test_ok.clone(),
        },
        rest: rest.clone(),
    }
}

/// Moves the four Lt work counters into the common carrier type.
pub fn lt_work_to_common(work: &BChargedCounters) -> BCommonChargedCounters {
    let BChargedCounters {
        remaining_work,
        consumed_work,
        helper_work,
        helper_visits,
    } = work;
    BCommonChargedCounters {
        remaining_work: remaining_work.clone(),
        consumed_work: consumed_work.clone(),
        helper_work: helper_work.clone(),
        helper_visits: helper_visits.clone(),
    }
}

/// Moves every completion counter lane into the common completion counter type.
pub fn lt_completion_counters_to_common(
    counters: &BScalarLtCompletionCounters,
) -> BCommonCompletionCounters {
    let BScalarLtCompletionCounters {
        work,
        reference_cursor,
        alloc_data_nodes,
        alloc_data_fields,
        alloc_environment_cells,
        depth,
        task_live,
        active_data_fields,
        active_env,
        active_value_slots,
        rest,
    } = counters;
    BCommonCompletionCounters {
        work: lt_work_to_common(work),
        reference_cursor: reference_cursor.clone(),
        alloc_data_nodes: alloc_data_nodes.clone(),
        alloc_data_fields: alloc_data_fields.clone(),
        alloc_environment_cells: alloc_environment_cells.clone(),
        depth: depth.clone(),
        task_live: task_live.clone(),
        active_data_fields: active_data_fields.clone(),
        active_env: active_env.clone(),
        active_value_slots: active_value_slots.clone(),
        rest: rest.clone(),
    }
}

impl BLtPublishState {
    /// Every lane, unchanged, in the existing `P1` / chain carrier.
    pub fn to_admin_publish_state(&self) -> BAdminPublishState {
        let Self {
            site,
            outcome,
            control,
            control_origin,
            helper,
            result,
            data,
            frames,
            counters,
            reference_cursor_bytes,
        } = self;
        BAdminPublishState {
            site: site.clone(),
            outcome: outcome.clone(),
            control: control.clone(),
            control_origin: control_origin.clone(),
            helper: lt_helper_to_common(helper),
            result: result.clone(),
            data: data.clone(),
            frames: frames.clone(),
            counters: lt_completion_counters_to_common(counters),
            reference_cursor_bytes: reference_cursor_bytes.clone(),
        }
    }

    /// Every lane, unchanged, in the existing legacy carrier plus its complement.
    pub fn to_legacy_handoff(&self) -> BLtLegacyHandoff {
        let BAdminPublishState {
            site,
            outcome:
                BOutcomeLanes {
                    tag,
                    halted_value,
                    rejected_code,
                    exhausted_reason,
                },
            control,
            control_origin,
            helper,
            result,
            data,
            frames,
            counters:
                BCommonCompletionCounters {
                    work:
                        BCommonChargedCounters {
                            remaining_work,
                            consumed_work,
                            helper_work,
                            helper_visits,
                        },
                    reference_cursor,
                    alloc_data_nodes,
                    alloc_data_fields,
                    alloc_environment_cells,
                    depth,
                    task_live,
                    active_data_fields,
                    active_env,
                    active_value_slots,
                    rest: counter_rest,
                },
            reference_cursor_bytes,
        } = self.to_admin_publish_state();
        let mut rest = vec![
            alloc_data_nodes,
            alloc_data_fields,
            alloc_environment_cells,
            task_live,
            active_data_fields,
            active_env,
            active_value_slots,
        ];
        rest.extend(counter_rest);
        BLtLegacyHandoff {
            state: BLegacyCompletionState {
                site,
                outcome: tag,
                control,
                control_origin,
                helper,
                result,
                frames,
                depth,
                remaining_work,
                consumed_work,
                helper_work,
                helper_visits,
                reference_cursor: reference_cursor_bytes,
                rest,
            },
            outcome_halted_value: halted_value,
            outcome_rejected_code: rejected_code,
            outcome_exhausted_reason: exhausted_reason,
            data,
            reference_cursor,
        }
    }
}

impl BLtLegacyHandoff {
    /// Exact inverse of `BLtPublishState::to_legacy_handoff` into the common
    /// `BAdminPublishState`; refuses a `state.rest` shorter than its 7 named lanes.
    pub fn to_admin_publish_state(
        &self,
    ) -> Result<BAdminPublishState, BScalarLtCompletionConstructionError> {
        let Self {
            state:
                BLegacyCompletionState {
                    site,
                    outcome,
                    control,
                    control_origin,
                    helper,
                    result,
                    frames,
                    depth,
                    remaining_work,
                    consumed_work,
                    helper_work,
                    helper_visits,
                    reference_cursor: reference_cursor_bytes,
                    rest,
                },
            outcome_halted_value,
            outcome_rejected_code,
            outcome_exhausted_reason,
            data,
            reference_cursor,
        } = self;
        let [alloc_data_nodes, alloc_data_fields, alloc_environment_cells, task_live, active_data_fields, active_env, active_value_slots, counter_rest @ ..] =
            rest.as_slice()
        else {
            return Err(BScalarLtCompletionConstructionError::LegacyRestLaneCount);
        };
        Ok(BAdminPublishState {
            site: site.clone(),
            outcome: BOutcomeLanes {
                tag: outcome.clone(),
                halted_value: outcome_halted_value.clone(),
                rejected_code: outcome_rejected_code.clone(),
                exhausted_reason: outcome_exhausted_reason.clone(),
            },
            control: control.clone(),
            control_origin: control_origin.clone(),
            helper: helper.clone(),
            result: result.clone(),
            data: data.clone(),
            frames: frames.clone(),
            counters: BCommonCompletionCounters {
                work: BCommonChargedCounters {
                    remaining_work: remaining_work.clone(),
                    consumed_work: consumed_work.clone(),
                    helper_work: helper_work.clone(),
                    helper_visits: helper_visits.clone(),
                },
                reference_cursor: reference_cursor.clone(),
                alloc_data_nodes: alloc_data_nodes.clone(),
                alloc_data_fields: alloc_data_fields.clone(),
                alloc_environment_cells: alloc_environment_cells.clone(),
                depth: depth.clone(),
                task_live: task_live.clone(),
                active_data_fields: active_data_fields.clone(),
                active_env: active_env.clone(),
                active_value_slots: active_value_slots.clone(),
                rest: counter_rest.to_vec(),
            },
            reference_cursor_bytes: reference_cursor_bytes.clone(),
        })
    }
}

impl BScalarExitLtBoolInputs {
    /// Exit inputs over the lanes of two Lt states that `Scalar.exit` reads or writes;
    /// the remaining state lanes belong to `emit_b_lt_completion_complement`.
    pub fn from_states(
        enable: Operand,
        bounds_data_nodes: Operand,
        pre: &BLtPublishState,
        post: &BLtPublishState,
    ) -> Self {
        Self {
            enable,
            bounds_data_nodes,
            pre_data: pre.data.clone(),
            post_data: post.data.clone(),
            pre_helper: pre.helper.clone(),
            post_helper: post.helper.clone(),
            pre_result: pre.result.clone(),
            post_result: post.result.clone(),
            pre_outcome: pre.outcome.clone(),
            post_outcome: post.outcome.clone(),
            pre_counters: pre.counters.clone(),
            post_counters: post.counters.clone(),
        }
    }
}

impl BScalarFailureLtInputs {
    /// Failure inputs over the lanes of two Lt states that `Scalar.failure` reads or
    /// writes; the remaining state lanes belong to `emit_b_lt_completion_complement`.
    pub fn from_states(enable: Operand, pre: &BLtPublishState, post: &BLtPublishState) -> Self {
        Self {
            enable,
            pre_data: pre.data.clone(),
            post_data: post.data.clone(),
            pre_helper: pre.helper.clone(),
            post_helper: post.helper.clone(),
            pre_result: pre.result.clone(),
            post_result: post.result.clone(),
            pre_outcome: pre.outcome.clone(),
            post_outcome: post.outcome.clone(),
            pre_counters: pre.counters.clone(),
            post_counters: post.counters.clone(),
        }
    }
}

/// Emit the `enable`-gated `T-HELPER` preservation of the state lanes that
/// `Scalar.exit` / `Scalar.failure` never write (`site`, `control_origin`, every
/// `control` lane, every `frames` lane including `RightF.op`, and
/// `reference_cursor_bytes`), plus the 16-byte LE binding of `pre` and `post`
/// `reference_cursor_bytes` to `counters.reference_cursor`. That binding is `enable`-gated, so
/// a composer must bind the cursor unconditionally at every stage. `%{scope}.local.temp.` must be
/// owned by this block alone across the whole `IrSource`: no other producer's namespace may be a
/// prefix of it or prefixed by it (caller premise, unchecked).
pub fn emit_b_lt_completion_complement(
    enable: &Operand,
    pre: &BLtPublishState,
    post: &BLtPublishState,
    scope: &str,
) -> Result<Vec<Instruction>, BScalarLtCompletionConstructionError> {
    if scope.is_empty() {
        return Err(ConstructionError::EmptyScope.into());
    }
    if pre.frames.len() != post.frames.len() {
        return Err(BScalarLtCompletionConstructionError::FrameRowCount);
    }
    let frame_rest = pre.frames.first().map_or(0, |f| f.rest.len());
    if pre.frames.iter().chain(&post.frames).any(|f| f.rest.len() != frame_rest) {
        return Err(BScalarLtCompletionConstructionError::FrameRestLaneCount);
    }
    if pre.control.rest.len() != post.control.rest.len() {
        return Err(BScalarLtCompletionConstructionError::ControlRestLaneCount);
    }
    let temp_prefix = format!("%{scope}.local.temp.");
    for operand in std::iter::once(enable)
        .chain(complement_operands(pre))
        .chain(complement_operands(post))
        .chain([&pre.counters.reference_cursor, &post.counters.reference_cursor])
    {
        check_variable(operand, &temp_prefix)?;
    }

    let mut b = Emitter {
        scope: scope.into(),
        serial: 0,
        code: Vec::new(),
    };
    b.boolean(enable.clone());

    // C01: post == pre on every state lane the completion rows never write.
    for (pre_lane, post_lane) in complement_operands(pre).zip(complement_operands(post)) {
        b.gated_eq(enable, post_lane.clone(), pre_lane.clone());
    }

    // C02: referenceCursor byte lanes are the 16-byte LE image of the scalar lane.
    for st in [pre, post] {
        let bytes = eff_bytes(&mut b, enable, &st.reference_cursor_bytes);
        let mut acc = bytes[15].clone();
        for i in (0..15).rev() {
            let shifted = b.mul(acc, immediate(256));
            acc = b.add(shifted, bytes[i].clone());
        }
        let scalar = eff_uint128(&mut b, enable, &st.counters.reference_cursor);
        b.eq(scalar, acc);
    }

    Ok(b.code)
}

fn complement_operands(st: &BLtPublishState) -> impl Iterator<Item = &Operand> {
    let c = &st.control;
    [
        &st.site,
        &st.control_origin,
        &c.tag,
        &c.return_value.space,
        &c.return_value.index,
        &c.prim_outcome_tag,
        &c.prim_value.space,
        &c.prim_value.index,
        &c.prim_error_code,
        &c.error_code,
    ]
    .into_iter()
    .chain(c.rest.iter())
    .chain(st.frames.iter().flat_map(|f| {
        [&f.tag, &f.origin, &f.right_op, &f.right_left.space, &f.right_left.index]
            .into_iter()
            .chain(f.rest.iter())
    }))
    .chain(st.reference_cursor_bytes.iter())
}

fn validate_common_shapes(
    scope: &str,
    pre_data: &BDataTableLanes,
    post_data: &BDataTableLanes,
    pre_helper: &BScalarArithmeticRegister,
    post_helper: &BScalarArithmeticRegister,
    pre_result: &BHelperResultLanes,
    post_result: &BHelperResultLanes,
    pre_counters: &BScalarLtCompletionCounters,
    post_counters: &BScalarLtCompletionCounters,
) -> Result<(), BScalarLtCompletionConstructionError> {
    if scope.is_empty() {
        return Err(ConstructionError::EmptyScope.into());
    }
    if pre_data.rows.len() != pre_data.capacity
        || post_data.rows.len() != post_data.capacity
        || pre_data.capacity != post_data.capacity
    {
        return Err(BScalarLtCompletionConstructionError::DataRowCount);
    }
    if let Some(first) = pre_data.rows.first() {
        let expected_row_rest = first.rest.len();
        for (pre_r, post_r) in pre_data.rows.iter().zip(post_data.rows.iter()) {
            if pre_r.rest.len() != expected_row_rest || post_r.rest.len() != expected_row_rest {
                return Err(BScalarLtCompletionConstructionError::DataRowRestLaneCount);
            }
        }
    }
    if pre_helper.rest.len() != post_helper.rest.len() {
        return Err(BScalarLtCompletionConstructionError::HelperRestLaneCount);
    }
    if pre_result.rest.len() != post_result.rest.len() {
        return Err(BScalarLtCompletionConstructionError::ResultRestLaneCount);
    }
    if pre_counters.rest.len() != post_counters.rest.len() {
        return Err(BScalarLtCompletionConstructionError::CounterRestLaneCount);
    }
    Ok(())
}

fn check_variable(
    operand: &Operand,
    temp_prefix: &str,
) -> Result<(), BScalarLtCompletionConstructionError> {
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

fn constrain_result_absent(b: &mut Emitter, enable: &Operand, result: &BHelperResultLanes) {
    for col in result_operands(result) {
        b.gated_eq(enable, col.clone(), immediate(0));
    }
}

fn constrain_data_row_zero_when(b: &mut Emitter, cond: &Operand, row: &BDataRowLanes) {
    for col in std::iter::once(&row.tag)
        .chain(row.n.iter())
        .chain([&row.bool_b, &row.fields_start, &row.fields_len])
        .chain(row.rest.iter())
    {
        b.gated_eq(cond, col.clone(), immediate(0));
    }
}

fn constrain_completion_work_and_preserved_counters(
    b: &mut Emitter,
    enable: &Operand,
    pre: &BScalarLtCompletionCounters,
    post: &BScalarLtCompletionCounters,
) {
    let pre_remaining = eff_bytes(b, enable, &pre.work.remaining_work);
    let pre_consumed = eff_bytes(b, enable, &pre.work.consumed_work);
    let pre_helper_work = eff_bytes(b, enable, &pre.work.helper_work);
    let pre_visits = eff_bytes(b, enable, &pre.work.helper_visits);
    let post_remaining = eff_bytes(b, enable, &post.work.remaining_work);
    let post_consumed = eff_bytes(b, enable, &post.work.consumed_work);
    let post_helper_work = eff_bytes(b, enable, &post.work.helper_work);
    let post_visits = eff_bytes(b, enable, &post.work.helper_visits);

    decrement(b, enable, &pre_remaining, &post_remaining);
    increment(b, enable, &pre_consumed, &post_consumed);
    increment(b, enable, &pre_helper_work, &post_helper_work);
    // helperVisits is 0-delta (preserved) on both Scalar.exit and Scalar.failure.
    for i in 0..16 {
        b.eq(post_visits[i].clone(), pre_visits[i].clone());
    }

    for (pre_c, post_c) in [
        (&pre.reference_cursor, &post.reference_cursor),
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
        b.gated_eq(enable, post_c.clone(), pre_c.clone());
    }
}

fn data_table_operands(table: &BDataTableLanes) -> impl Iterator<Item = &Operand> {
    std::iter::once(&table.active_rows).chain(table.rows.iter().flat_map(|row| {
        std::iter::once(&row.tag)
            .chain(row.n.iter())
            .chain([&row.bool_b, &row.fields_start, &row.fields_len])
            .chain(row.rest.iter())
    }))
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
    counters: &BScalarLtCompletionCounters,
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

fn decrement(b: &mut Emitter, enable: &Operand, pre: &BU128Lanes, post: &BU128Lanes) {
    let mut borrow = enable.clone();
    for i in 0..16 {
        let zero = b.test_eq(pre[i].clone(), immediate(0));
        let next = b.select(borrow.clone(), zero, immediate(0));
        b.boolean(next.clone());
        let carry = b.mul(immediate(256), next.clone());
        b.bits(carry.clone(), 9);
        let left = b.add(pre[i].clone(), carry);
        b.bits(left.clone(), 9);
        let right = b.add(post[i].clone(), borrow);
        b.bits(right.clone(), 9);
        b.eq(left, right);
        borrow = next;
    }
    b.eq(borrow, immediate(0));
}

fn increment(b: &mut Emitter, enable: &Operand, pre: &BU128Lanes, post: &BU128Lanes) {
    let mut carry = enable.clone();
    for i in 0..16 {
        let full = b.test_eq(pre[i].clone(), immediate(255));
        let next = b.select(carry.clone(), full, immediate(0));
        b.boolean(next.clone());
        let overflow = b.mul(immediate(256), next.clone());
        b.bits(overflow.clone(), 9);
        let left = b.add(post[i].clone(), overflow);
        b.bits(left.clone(), 9);
        let right = b.add(pre[i].clone(), carry);
        b.bits(right.clone(), 9);
        b.eq(left, right);
        carry = next;
    }
    b.eq(carry, immediate(0));
}
