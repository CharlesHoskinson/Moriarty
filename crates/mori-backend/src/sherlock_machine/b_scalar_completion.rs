//! Handwritten B-core `midnight_zkir_v3` instruction emitters for Scalar helper
//! completion following `RetRight / Add / Second -> Done`:
//! 1. `b.helper.Scalar.exit` (`/helper_events/4` in `rows-r4.json`) on branch
//!    `site = RetRight`, `mode = Add`, `phase = Done`, `staged = U128Value`,
//!    covering both `Applied` (`pre.alloc.dataNodes < bounds.dataNodes`:
//!    appends `DataRow::U128 { n: pre.helper.staged.n }` at `pre.active.data`,
//!    increments `active.data` and `alloc.dataNodes` by `+1`, sets
//!    `reg.result = Some(ScalarValue(Data(pre.active.data)))`, clears
//!    `reg.helper` to absent/all-zero, and keeps `reg.outcome = Running`) and
//!    `RefusedCapCharged(DataNodes)` (`pre.alloc.dataNodes >= bounds.dataNodes`:
//!    P6 logical cap refusal with P4 work charges only, `reg.outcome =
//!    Exhausted(DataNodes)`, no arena write or allocation/active delta, and
//!    `reg.helper` and `reg.result` retained as in pre-state).
//! 2. `b.helper.Scalar.failure` (`/helper_events/5` in `rows-r4.json`) on branch
//!    `site = RetRight`, `mode = Add`, `phase = Done`, `staged = PrimError(code)`
//!    with `code in {10, 11}`, setting `reg.result = Some(ScalarError { code })`,
//!    clearing `reg.helper` to absent/all-zero, preserving `img.data`,
//!    `active.data`, `alloc.dataNodes`, `helperVisits`, and `reg.outcome = Running`,
//!    and applying P4 work charges `[-1, +1, +1, 0]`.
//!
//! Source only: emits pinned SDK instructions; performs no SDK load, check,
//! evaluation, compilation, or proof. Subsequent legacy `RetRight` frame pop,
//! `primitive.error`, and `P1` publication remain separate relations.

use super::*;

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

/// Full `records.DataRow` lane view for `img.data` arena append and preservation:
/// variant `tag`, `U128.n` 16 LE byte lanes, `Bool.b` lane, `FieldSlice.{start,len}`
/// lanes, and `rest` for any additional flattened row columns.
/// By `/notation/inactive`, inactive variant columns and rows at `index >= active.data`
/// are zero.
#[derive(Clone, Debug, PartialEq)]
pub struct BDataRowLanes {
    pub tag: Operand,
    pub n: BU128Lanes,
    pub bool_b: Operand,
    pub fields_start: Operand,
    pub fields_len: Operand,
    pub rest: Vec<Operand>,
}

impl BDataRowLanes {
    pub fn to_value_row_lanes(&self) -> BValueRowLanes {
        BValueRowLanes {
            tag: self.tag.clone(),
            n: self.n.clone(),
        }
    }
}

/// Physical `img.data` arena carrier with `reg.active.data` (`active_rows`) and
/// all physical slots `0..capacity` (`capacity` may be 0).
#[derive(Clone, Debug, PartialEq)]
pub struct BDataTableLanes {
    pub capacity: usize,
    pub active_rows: Operand,
    pub rows: Vec<BDataRowLanes>,
}

/// `reg.result` (`Opt<HelperResult>`) lane view covering `present`, `tag`
/// (`HelperResultTag`), `ScalarValue.value` (`BValueRef`), `ScalarError.code`,
/// `ScalarSelector.sel`, `ScalarTypeTest.ok`, and `rest` for non-Scalar union
/// columns. By `/notation/inactive`, all payload columns are zero when `present = 0`
/// and all inactive union columns are zero when `present = 1`.
#[derive(Clone, Debug, PartialEq)]
pub struct BHelperResultLanes {
    pub present: Operand,
    pub tag: Operand,
    pub scalar_value: BValueRef,
    pub scalar_error_code: Operand,
    pub scalar_selector: Operand,
    pub scalar_type_test_ok: Operand,
    pub rest: Vec<Operand>,
}

/// `reg.outcome` (`records.Outcome`) lane view: `tag` (`OutcomeTag`),
/// `Halted.value` (`BValueRef`), `Rejected.code`, and `Exhausted.reason`.
/// By `/notation/inactive`, inactive variant payload columns are zero.
#[derive(Clone, Debug, PartialEq)]
pub struct BOutcomeLanes {
    pub tag: Operand,
    pub halted_value: BValueRef,
    pub rejected_code: Operand,
    pub exhausted_reason: Operand,
}

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
pub struct BScalarCompletionCounters {
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
/// `RetRight / Add / Done / U128Value`.
#[derive(Clone, Debug, PartialEq)]
pub struct BScalarExitAddU128Inputs {
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
    pub pre_counters: BScalarCompletionCounters,
    pub post_counters: BScalarCompletionCounters,
}

/// Composable instruction fragment for `b.helper.Scalar.exit` (`RetRight/Add/Done/U128Value`).
#[derive(Clone, Debug, PartialEq)]
pub struct BScalarExitAddU128Block {
    pub instructions: Vec<Instruction>,
    pub applied: Operand,
    pub cap_refused: Operand,
    pub allocated_index: Operand,
    pub allocated_n: BU128Lanes,
    pub write_selectors: Vec<Operand>,
    pub preserved: BScalarExitAddU128Inputs,
}

/// Caller-supplied witness inputs for `b.helper.Scalar.failure` on branch
/// `RetRight / Add / Done / PrimError(code in {10, 11})`.
#[derive(Clone, Debug, PartialEq)]
pub struct BScalarFailureAddInputs {
    pub enable: Operand,
    pub pre_data: BDataTableLanes,
    pub post_data: BDataTableLanes,
    pub pre_helper: BScalarArithmeticRegister,
    pub post_helper: BScalarArithmeticRegister,
    pub pre_result: BHelperResultLanes,
    pub post_result: BHelperResultLanes,
    pub pre_outcome: BOutcomeLanes,
    pub post_outcome: BOutcomeLanes,
    pub pre_counters: BScalarCompletionCounters,
    pub post_counters: BScalarCompletionCounters,
}

/// Composable instruction fragment for `b.helper.Scalar.failure` (`RetRight/Add/Done/PrimError`).
#[derive(Clone, Debug, PartialEq)]
pub struct BScalarFailureAddBlock {
    pub instructions: Vec<Instruction>,
    pub error_code: Operand,
    pub is_overflow_10: Operand,
    pub is_sort_mismatch_11: Operand,
    pub preserved: BScalarFailureAddInputs,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BScalarCompletionConstructionError {
    DataRowCount,
    DataRowRestLaneCount,
    HelperRestLaneCount,
    ResultRestLaneCount,
    CounterRestLaneCount,
    Fetch(ConstructionError),
}

impl From<ConstructionError> for BScalarCompletionConstructionError {
    fn from(e: ConstructionError) -> Self {
        Self::Fetch(e)
    }
}

/// Emit the in-circuit relation for `b.helper.Scalar.exit` on branch
/// `site = RetRight`, `mode = Add`, `phase = Done`, `staged = U128Value`,
/// including P6 logical `DataNodes` cap check (`Applied` vs `RefusedCapCharged(DataNodes)`).
pub fn emit_b_scalar_exit_add_u128(
    input: &BScalarExitAddU128Inputs,
    scope: &str,
) -> Result<BScalarExitAddU128Block, BScalarCompletionConstructionError> {
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

    // E01: Pre-state branch guards for RetRight / Add / Done / U128Value,
    // Running outcome, and absent pre.result.
    for (pre, expected) in [
        (&input.pre_helper.present, HELPER_PRESENT),
        (&input.pre_helper.kind, HELPER_KIND_SCALAR),
        (&input.pre_helper.site, SITE_RET_RIGHT),
        (&input.pre_helper.mode, SCALAR_MODE_ADD),
        (&input.pre_helper.phase, SCALAR_PHASE_DONE),
        (&input.pre_helper.operand_b.present, OPT_PRESENT),
        (&input.pre_helper.staged.tag, SCALAR_STAGED_TAG_U128_VALUE),
        (&input.pre_helper.staged.bool_value, 0),
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

        // Post row equals newly appended DataRow::U128{n: staged.u128_value} when write_sel == 1,
        // and equals pre_row otherwise (covering prefix rows, trailing rows, and cap_refused).
        let exp_tag = b.select(
            write_sel.clone(),
            immediate(DATA_TAG_U128),
            pre_row.tag.clone(),
        );
        b.gated_eq(enable, post_row.tag.clone(), exp_tag);
        for i in 0..16 {
            let pre_byte = b.select(enable.clone(), pre_row.n[i].clone(), immediate(0));
            b.bits(pre_byte.clone(), 8);
            let post_byte = b.select(enable.clone(), post_row.n[i].clone(), immediate(0));
            b.bits(post_byte.clone(), 8);
            let exp_byte = b.select(write_sel.clone(), eff_staged_u128[i].clone(), pre_byte);
            b.eq(post_byte, exp_byte);
        }
        let exp_bool = b.select(write_sel.clone(), immediate(0), pre_row.bool_b.clone());
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
    let exp_res_present = b.select(
        applied.clone(),
        immediate(OPT_PRESENT),
        immediate(OPT_ABSENT),
    );
    b.gated_eq(enable, input.post_result.present.clone(), exp_res_present);
    let exp_res_tag = b.select(
        applied.clone(),
        immediate(HELPER_RESULT_TAG_SCALAR_VALUE),
        immediate(0),
    );
    b.gated_eq(enable, input.post_result.tag.clone(), exp_res_tag);
    let exp_res_space = b.select(applied.clone(), immediate(VALUE_SPACE_DATA), immediate(0));
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

    let allocated_n = std::array::from_fn(|i| {
        b.select(applied.clone(), eff_staged_u128[i].clone(), immediate(0))
    });

    Ok(BScalarExitAddU128Block {
        instructions: b.code,
        applied,
        cap_refused,
        allocated_index,
        allocated_n,
        write_selectors,
        preserved: input.clone(),
    })
}

/// Emit the in-circuit relation for `b.helper.Scalar.failure` on branch
/// `site = RetRight`, `mode = Add`, `phase = Done`, `staged = PrimError(code in {10, 11})`.
pub fn emit_b_scalar_failure_add(
    input: &BScalarFailureAddInputs,
    scope: &str,
) -> Result<BScalarFailureAddBlock, BScalarCompletionConstructionError> {
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

    // F01: Pre-state branch guards for RetRight / Add / Done / PrimError(code in {10, 11}),
    // Running outcome, absent pre.result, and zero inactive staged columns.
    for (pre, expected) in [
        (&input.pre_helper.present, HELPER_PRESENT),
        (&input.pre_helper.kind, HELPER_KIND_SCALAR),
        (&input.pre_helper.site, SITE_RET_RIGHT),
        (&input.pre_helper.mode, SCALAR_MODE_ADD),
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

    // F02: Constrain pre.helper.staged.prim_error_code in {10, 11} under enable.
    let raw_code = b.select(
        enable.clone(),
        input.pre_helper.staged.prim_error_code.clone(),
        immediate(PRIM_ERROR_ARITH_OVERFLOW),
    );
    b.bits(raw_code.clone(), 4);
    let eq_10 = b.test_eq(raw_code.clone(), immediate(PRIM_ERROR_ARITH_OVERFLOW));
    let eq_11 = b.test_eq(raw_code.clone(), immediate(PRIM_ERROR_SORT_MISMATCH));
    let valid_code = b.add(eq_10.clone(), eq_11.clone());
    b.eq(valid_code, immediate(1));

    let error_code = b.select(enable.clone(), raw_code, immediate(0));
    b.bits(error_code.clone(), 4);
    let is_overflow_10 = b.select(enable.clone(), eq_10, immediate(0));
    b.boolean(is_overflow_10.clone());
    let is_sort_mismatch_11 = b.select(enable.clone(), eq_11, immediate(0));
    b.boolean(is_sort_mismatch_11.clone());

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

    Ok(BScalarFailureAddBlock {
        instructions: b.code,
        error_code,
        is_overflow_10,
        is_sort_mismatch_11,
        preserved: input.clone(),
    })
}

fn validate_common_shapes(
    scope: &str,
    pre_data: &BDataTableLanes,
    post_data: &BDataTableLanes,
    pre_helper: &BScalarArithmeticRegister,
    post_helper: &BScalarArithmeticRegister,
    pre_result: &BHelperResultLanes,
    post_result: &BHelperResultLanes,
    pre_counters: &BScalarCompletionCounters,
    post_counters: &BScalarCompletionCounters,
) -> Result<(), BScalarCompletionConstructionError> {
    if scope.is_empty() {
        return Err(ConstructionError::EmptyScope.into());
    }
    if pre_data.rows.len() != pre_data.capacity
        || post_data.rows.len() != post_data.capacity
        || pre_data.capacity != post_data.capacity
    {
        return Err(BScalarCompletionConstructionError::DataRowCount);
    }
    if let Some(first) = pre_data.rows.first() {
        let expected_row_rest = first.rest.len();
        for (pre_r, post_r) in pre_data.rows.iter().zip(post_data.rows.iter()) {
            if pre_r.rest.len() != expected_row_rest || post_r.rest.len() != expected_row_rest {
                return Err(BScalarCompletionConstructionError::DataRowRestLaneCount);
            }
        }
    }
    if pre_helper.rest.len() != post_helper.rest.len() {
        return Err(BScalarCompletionConstructionError::HelperRestLaneCount);
    }
    if pre_result.rest.len() != post_result.rest.len() {
        return Err(BScalarCompletionConstructionError::ResultRestLaneCount);
    }
    if pre_counters.rest.len() != post_counters.rest.len() {
        return Err(BScalarCompletionConstructionError::CounterRestLaneCount);
    }
    Ok(())
}

fn check_variable(
    operand: &Operand,
    temp_prefix: &str,
) -> Result<(), BScalarCompletionConstructionError> {
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
    pre: &BScalarCompletionCounters,
    post: &BScalarCompletionCounters,
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
