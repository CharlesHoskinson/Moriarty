//! Proposed source-derived A102 witness producer and L4 logical-lane adapter.
//!
//! Maps complete `ProgramBlock` and `StateBlock` structures into `ImageValues`
//! in `literal_schema` lane order, and builds candidate02 `LiteralTransitionRequest`
//! values from a real `Arc<Program>`, `prepare_a`, `Machine::new(...).step()`, and
//! `encode_observed_a` for the pre-state.
//!
//! Trust and boundary qualifications:
//! - Host diagnostic / witness-construction source only (H1). Native authority is 0.
//! - Unchecked H1 recursion and allocation hazard: this module and `mori_core::sherlock`
//!   rely on derived recursive `PartialEq`, `Clone`, and `Drop` over `Program`, `Expr`,
//!   `Value`, `Tree`, `Environment`, and `Frame`, as well as full capacity-sized table
//!   allocations in `prepare_a`, `encode_observed_a`, and `*_block_to_image_values`.
//!   Valid deep ASTs or runtime trees can overflow the native call stack and abort the
//!   process; there is no guaranteed `Err` return for untrusted or deep inputs.
//! - `decode_observed_a` is NEVER called on constructed Step-mode `post` rows:
//!   under the `layout_a` trusted-diagnostic contract, `decode_observed_a` requires
//!   every decoded `StateBlock` to come from `encode_observed_a` over a trusted
//!   `Snapshot`. Because A102's post-image is a Step-mode mid-segment state whose
//!   `CR0 = Return` holds a Constant-space `ValueRef` and preserves all five
//!   arenas from `pre`, it is constructed via the isolated seam
//!   [`encode_a102_step_post_state`], not `encode_observed_a`.
//! - Raw-helper vs end-to-end provenance: [`produce_a102_literal_transition`] computes
//!   its own fresh `PreparedA`, entry `StepRecord`, and `pre_state`, whereas the public
//!   raw helpers [`encode_a102_step_post_state`] and [`produce_a102_literal_transition_from_step`]
//!   trust caller-supplied `PreparedA`, `pre_state`, and `StepRecord` provenance.
//! - Relation-hash / source-closure note (C2/C3): `LiteralLayout::new`
//!   (`moriarty-sherlock-a102-relation/1`) in `literal_host.rs` now hashes eight
//!   `include_str!` sources (`mod.rs`, `fetch.rs`, `enabled_fetch.rs`, `literal_host.rs`,
//!   `literal_step.rs`, `literal_bridge.rs`, `literal_witness.rs`, and
//!   `../../../../models/sherlock/literal-step/MAPPING.md`) rather than the prior
//!   six-file list with `literal_witness.rs` outside, and `mori-ledger` `source_closure()`
//!   also includes `literal_host.rs` and `literal_witness.rs`. This explicitly creates a
//!   new source, `relation_hash`, `context`/`context_hash`, build, and key identity with
//!   no old-key or descriptor inheritance; any edit to these eight files (including
//!   comments in `literal_witness.rs`) changes `relation_hash` and `context_hash`.
//! - Does not discharge `StepInv`, `uiValid`, `F4`, `OBL.G4`, `projectA`,
//!   source-digest authentication, ledger/native verification, or full goal.

use super::literal_host::{
    Error, ImageValues, LiteralCapacities, LiteralConstructionError, LiteralLane, LiteralLayout,
    LiteralSchema, LiteralSourceIdentity, LiteralTable, LiteralTransitionRequest, LiteralValues,
    RULE_CHARGE, RULE_START_LINE, RULE_TAG, error, literal_schema,
};
use mori_core::sherlock::layout_a::{
    Capacities, ConstantNode, ControlRow, EnvCell, EnvRoot, FrameRow, Locator, PreparedA,
    ProgramBlock, Slice, StateBlock, StateCapacities, StateFailure, StateResult, Table, ValueRef,
    encode_observed_a, prepare_a,
};
use mori_core::sherlock::{
    Control, Expr, Machine, Outcome, Program, Snapshot, StepRecord, Tree, Value,
};
use std::sync::Arc;

/// Complete source-derived A102 transition bundle retaining the unmodified
/// `PreparedA`, the semantic `StepRecord` from `Machine::step`, the canonical
/// `pre_state`, the Step-mode `post_state`, selected metadata, and the
/// resulting `LiteralTransitionRequest`.
#[derive(Clone, Debug)]
pub struct ProducedLiteralTransition {
    pub prepared: PreparedA,
    pub step_record: StepRecord,
    pub pre_state: StateBlock,
    pub post_state: StateBlock,
    pub selected_expr_id: usize,
    pub selected_constant_ref: ValueRef,
    pub request: LiteralTransitionRequest,
}

fn wrong_rep(
    block: &'static str,
    table: &'static str,
    row: usize,
    column: &'static str,
    definition: &'static str,
) -> StateFailure {
    StateFailure::WrongRepresentation {
        block,
        table,
        row,
        column,
        definition,
    }
}

fn host_err(field: &'static str, item: usize) -> StateFailure {
    StateFailure::HostUnrepresentable {
        field,
        locator: Locator {
            expression: None,
            function: None,
            field,
            item,
        },
    }
}

fn usize_to_u128(value: usize, field: &'static str, item: usize) -> StateResult<u128> {
    u128::try_from(value).map_err(|_| host_err(field, item))
}

/// Convert `LiteralCapacities` into `layout_a::Capacities` without clamping or defaults.
pub fn core_program_capacities(capacities: &LiteralCapacities) -> StateResult<Capacities> {
    let mut tables = [0u128; 15];
    for (i, &cap) in capacities.program.iter().enumerate() {
        tables[i] = usize_to_u128(cap, "LiteralCapacities.program", i)?;
    }
    let data = usize_to_u128(capacities.state[0], "LiteralCapacities.state.D", 0)?;
    Ok(Capacities { tables, data })
}

/// Convert `LiteralCapacities` into `layout_a::StateCapacities` without clamping or defaults.
pub fn core_state_capacities(capacities: &LiteralCapacities) -> StateResult<StateCapacities> {
    Ok(StateCapacities {
        data: usize_to_u128(capacities.state[0], "LiteralCapacities.state.D", 0)?,
        data_fields: usize_to_u128(capacities.state[1], "LiteralCapacities.state.DF", 1)?,
        env_cells: usize_to_u128(capacities.state[2], "LiteralCapacities.state.E", 2)?,
        value_slots: usize_to_u128(capacities.state[3], "LiteralCapacities.state.V", 3)?,
        frame_slots: usize_to_u128(capacities.state[4], "LiteralCapacities.state.S", 4)?,
        control_rows: usize_to_u128(capacities.state[5], "LiteralCapacities.state.C", 5)?,
        stack_limit: capacities.stack_limit,
    })
}

/// Validate `LiteralCapacities` against `literal_schema` and map construction
/// failures to existing `StateFailure` refusal classes.
pub fn validate_literal_schema(capacities: &LiteralCapacities) -> StateResult<LiteralSchema> {
    literal_schema(capacities).map_err(|err| match err {
        LiteralConstructionError::MinimumShape => wrong_rep(
            "ProgramBlock",
            "ProgramHeader",
            0,
            "capacities",
            "A102 schema requires X>=1, K>=1, C>=1, T>=4, Q>=4, QF>=5",
        ),
        LiteralConstructionError::SizeOverflow => host_err("LiteralSchema", 0),
        LiteralConstructionError::SchemaMismatch => wrong_rep(
            "ProgramBlock",
            "ProgramHeader",
            0,
            "schema",
            "schema mismatch against capacities",
        ),
        LiteralConstructionError::ImageShape { block, .. } => wrong_rep(
            block,
            "Root",
            0,
            "shape",
            "image shape differs from L4 schema",
        ),
        LiteralConstructionError::Fetch(_) => wrong_rep(
            "ProgramBlock",
            "Expressions",
            0,
            "fetchShape",
            "invalid expression fetch table shape",
        ),
    })
}

fn check_lane_bounds(
    block: &'static str,
    table: &'static str,
    row: usize,
    lanes: &[LiteralLane],
    values: &[u128],
) -> StateResult<()> {
    if values.len() != lanes.len() {
        return Err(wrong_rep(
            block,
            table,
            row,
            "laneCount",
            "row lane count must equal declared L4 schema lane count",
        ));
    }
    for (lane, &value) in lanes.iter().zip(values) {
        if lane.boolean {
            if value > 1 {
                return Err(wrong_rep(
                    block,
                    table,
                    row,
                    "boolean",
                    "boolean lane must be 0 or 1",
                ));
            }
        } else if lane.bits < 128 && (value >> lane.bits) != 0 {
            return Err(wrong_rep(
                block,
                table,
                row,
                "bits",
                "unsigned lane exceeds declared L4 bit width",
            ));
        }
        if lane.maximum.is_some_and(|max| value > max) {
            return Err(wrong_rep(
                block,
                table,
                row,
                "maximum",
                "lane value exceeds declared L4 maximum",
            ));
        }
    }
    Ok(())
}

fn check_program_table<T: Default + PartialEq>(
    table_name: &'static str,
    table: &Table<T>,
    header_active: usize,
    capacity: usize,
) -> StateResult<()> {
    if table.active != header_active {
        return Err(wrong_rep(
            "ProgramBlock",
            table_name,
            0,
            "activeLength",
            "table active length must equal ProgramHeader active_lengths entry",
        ));
    }
    let active_u128 = usize_to_u128(table.active, table_name, 0)?;
    let cap_u128 = usize_to_u128(capacity, table_name, 0)?;
    if active_u128 > cap_u128 {
        return Err(StateFailure::Capacity {
            table: table_name,
            needed: active_u128,
            limit: cap_u128,
            locator: Locator::field(table_name),
        });
    }
    if table.rows.len() != capacity {
        return Err(wrong_rep(
            "ProgramBlock",
            table_name,
            0,
            "fixedLength",
            "padded program table length must equal declared capacity",
        ));
    }
    let zero = T::default();
    for (idx, row) in table.rows.iter().enumerate().skip(table.active) {
        if row != &zero {
            return Err(wrong_rep(
                "ProgramBlock",
                table_name,
                idx,
                "padding",
                "every inactive program row must be zero",
            ));
        }
    }
    Ok(())
}

fn check_state_table<T: Default + PartialEq>(
    table_name: &'static str,
    rows: &[T],
    active: usize,
    capacity: usize,
) -> StateResult<()> {
    let active_u128 = usize_to_u128(active, table_name, 0)?;
    let cap_u128 = usize_to_u128(capacity, table_name, 0)?;
    if active_u128 > cap_u128 {
        return Err(StateFailure::Capacity {
            table: table_name,
            needed: active_u128,
            limit: cap_u128,
            locator: Locator::field(table_name),
        });
    }
    if rows.len() != capacity {
        return Err(wrong_rep(
            "StateBlock",
            table_name,
            0,
            "length",
            "fixed state vector length must equal declared capacity",
        ));
    }
    let zero = T::default();
    for (idx, row) in rows.iter().enumerate().skip(active) {
        if row != &zero {
            return Err(wrong_rep(
                "StateBlock",
                table_name,
                idx,
                "padding",
                "every inactive state row must be zero",
            ));
        }
    }
    Ok(())
}

fn slice_lanes(slice: Slice, field: &'static str, row: usize) -> StateResult<[u128; 2]> {
    Ok([
        usize_to_u128(slice.start, field, row)?,
        usize_to_u128(slice.len, field, row)?,
    ])
}

fn value_ref_lanes(value: ValueRef, field: &'static str, row: usize) -> StateResult<[u128; 2]> {
    Ok([
        u128::from(value.space),
        usize_to_u128(value.index, field, row)?,
    ])
}

fn env_root_lanes(root: EnvRoot, field: &'static str, row: usize) -> StateResult<[u128; 2]> {
    Ok([
        u128::from(root.present),
        usize_to_u128(root.index, field, row)?,
    ])
}

fn constant_or_data_node_lanes(
    node: &ConstantNode,
    table: &'static str,
    row: usize,
) -> StateResult<Vec<u128>> {
    let mut out = Vec::with_capacity(22);
    out.push(u128::from(node.kind));
    out.push(u128::from(node.type_id));
    out.push(usize_to_u128(node.ctor_id, table, row)?);
    for &byte in &node.uint_bytes {
        out.push(u128::from(byte));
    }
    out.push(u128::from(node.bool_value));
    out.extend(slice_lanes(node.fields, table, row)?);
    Ok(out)
}

fn frame_row_lanes(frame: &FrameRow, table: &'static str, row: usize) -> StateResult<Vec<u128>> {
    let mut out = Vec::with_capacity(17);
    out.push(u128::from(frame.tag));
    out.push(usize_to_u128(frame.name_id, table, row)?);
    out.push(u128::from(frame.binary_op));
    out.push(u128::from(frame.unary_op));
    out.push(u128::from(frame.type_id));
    out.push(usize_to_u128(frame.expr0, table, row)?);
    out.push(usize_to_u128(frame.expr1, table, row)?);
    out.extend(env_root_lanes(frame.saved_env, table, row)?);
    out.extend(value_ref_lanes(frame.value, table, row)?);
    out.extend(slice_lanes(frame.completed, table, row)?);
    out.extend(slice_lanes(frame.remaining_exprs, table, row)?);
    out.extend(slice_lanes(frame.arms, table, row)?);
    Ok(out)
}

fn control_row_lanes(
    control: &ControlRow,
    table: &'static str,
    row: usize,
) -> StateResult<Vec<u128>> {
    let mut out = Vec::with_capacity(36);
    out.push(u128::from(control.tag));
    out.push(usize_to_u128(control.expr_id, table, row)?);
    out.extend(value_ref_lanes(control.value, table, row)?);
    out.extend(frame_row_lanes(&control.pending_frame, table, row)?);
    out.extend(env_root_lanes(control.environment, table, row)?);
    out.push(usize_to_u128(control.name_id, table, row)?);
    out.extend(slice_lanes(control.values, table, row)?);
    out.push(u128::from(control.function_present));
    out.push(usize_to_u128(control.function_id, table, row)?);
    out.push(u128::from(control.bindings_present));
    out.extend(env_root_lanes(control.bindings, table, row)?);
    out.push(u128::from(control.type_id));
    out.extend(slice_lanes(control.arms, table, row)?);
    out.push(u128::from(control.primitive_is_ok));
    out.push(u128::from(control.error_code));
    Ok(out)
}

fn validate_mapped_tables(
    block_name: &'static str,
    root_lanes: &[LiteralLane],
    table_schemas: &[LiteralTable],
    image: &ImageValues,
) -> StateResult<()> {
    check_lane_bounds(block_name, "Root", 0, root_lanes, &image.root)?;
    if image.tables.len() != table_schemas.len() {
        return Err(wrong_rep(
            block_name,
            "Root",
            0,
            "tableCount",
            "table count differs from L4 schema",
        ));
    }
    for (table_schema, rows) in table_schemas.iter().zip(&image.tables) {
        if rows.len() != table_schema.capacity {
            return Err(wrong_rep(
                block_name,
                table_schema.name,
                0,
                "capacity",
                "table row count differs from declared capacity",
            ));
        }
        for (row_idx, row_values) in rows.iter().enumerate() {
            check_lane_bounds(
                block_name,
                table_schema.name,
                row_idx,
                &table_schema.lanes,
                row_values,
            )?;
        }
    }
    Ok(())
}

/// Complete `ProgramBlock -> ImageValues` logical mapping in exact `literal_schema` / L4 lane order.
///
/// Maps the 16-lane `ProgramHeader` and all 15 padded program tables, preserving
/// active counts, inactive zero rows, and full capacities. Checks active lengths,
/// padding, builtin type/constructor declarations, occurrence/function indices,
/// and all schema lane bit-widths and maximums.
pub fn program_block_to_image_values(
    block: &ProgramBlock,
    capacities: &LiteralCapacities,
) -> StateResult<ImageValues> {
    let schema = validate_literal_schema(capacities)?;
    let h = &block.header;

    check_program_table(
        "Names",
        &block.names,
        h.active_lengths[0],
        capacities.program[0],
    )?;
    check_program_table(
        "NameBytes",
        &block.name_bytes,
        h.active_lengths[1],
        capacities.program[1],
    )?;
    check_program_table(
        "Occurrences",
        &block.occurrences,
        h.active_lengths[2],
        capacities.program[2],
    )?;
    check_program_table(
        "OccurrencePaths",
        &block.occurrence_paths,
        h.active_lengths[3],
        capacities.program[3],
    )?;
    check_program_table(
        "Expressions",
        &block.expressions,
        h.active_lengths[4],
        capacities.program[4],
    )?;
    check_program_table(
        "ExprChildren",
        &block.expr_children,
        h.active_lengths[5],
        capacities.program[5],
    )?;
    check_program_table(
        "Functions",
        &block.functions,
        h.active_lengths[6],
        capacities.program[6],
    )?;
    check_program_table(
        "Parameters",
        &block.parameters,
        h.active_lengths[7],
        capacities.program[7],
    )?;
    check_program_table(
        "Patterns",
        &block.patterns,
        h.active_lengths[8],
        capacities.program[8],
    )?;
    check_program_table(
        "Arms",
        &block.arms,
        h.active_lengths[9],
        capacities.program[9],
    )?;
    check_program_table(
        "Types",
        &block.types,
        h.active_lengths[10],
        capacities.program[10],
    )?;
    check_program_table(
        "Constructors",
        &block.constructors,
        h.active_lengths[11],
        capacities.program[11],
    )?;
    check_program_table(
        "ConstructorFieldTypes",
        &block.constructor_field_types,
        h.active_lengths[12],
        capacities.program[12],
    )?;
    check_program_table(
        "Constants",
        &block.constants,
        h.active_lengths[13],
        capacities.program[13],
    )?;
    check_program_table(
        "ConstantFields",
        &block.constant_fields,
        h.active_lengths[14],
        capacities.program[14],
    )?;

    if h.entry_expr_id != 0 || block.expressions.active == 0 {
        return Err(wrong_rep(
            "ProgramBlock",
            "ProgramHeader",
            0,
            "entryExprId",
            "ProgramHeader requires entryExprId=0 and activeExpressions>=1",
        ));
    }

    for (idx, row) in block
        .expressions
        .rows
        .iter()
        .take(block.expressions.active)
        .enumerate()
    {
        if row.occurrence_id != idx {
            return Err(wrong_rep(
                "ProgramBlock",
                "Expressions",
                idx,
                "occurrenceId",
                "Expressions[i].occurrenceId must equal row index i",
            ));
        }
    }
    for (idx, row) in block
        .functions
        .rows
        .iter()
        .take(block.functions.active)
        .enumerate()
    {
        if row.source_table_index != idx {
            return Err(wrong_rep(
                "ProgramBlock",
                "Functions",
                idx,
                "sourceTableIndex",
                "Functions[i].sourceTableIndex must equal row index i",
            ));
        }
    }

    let mut root = Vec::with_capacity(16);
    root.push(usize_to_u128(
        h.entry_expr_id,
        "ProgramHeader.entryExprId",
        0,
    )?);
    for (i, &len) in h.active_lengths.iter().enumerate() {
        root.push(usize_to_u128(len, "ProgramHeader.activeLengths", i)?);
    }

    let names = block
        .names
        .rows
        .iter()
        .enumerate()
        .map(|(i, r)| Ok(slice_lanes(r.bytes, "Names", i)?.to_vec()))
        .collect::<StateResult<Vec<_>>>()?;

    let name_bytes = block
        .name_bytes
        .rows
        .iter()
        .map(|&b| vec![u128::from(b)])
        .collect::<Vec<_>>();

    let occurrences = block
        .occurrences
        .rows
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let [start, len] = slice_lanes(r.path, "Occurrences", i)?;
            Ok(vec![
                u128::from(r.owner_tag),
                usize_to_u128(r.function_index, "Occurrences.functionIndex", i)?,
                start,
                len,
            ])
        })
        .collect::<StateResult<Vec<_>>>()?;

    let occurrence_paths = block
        .occurrence_paths
        .rows
        .iter()
        .enumerate()
        .map(|(i, &p)| Ok(vec![usize_to_u128(p, "OccurrencePaths", i)?]))
        .collect::<StateResult<Vec<_>>>()?;

    let expressions = block
        .expressions
        .rows
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let [val_space, val_index] = value_ref_lanes(r.value, "Expressions.valueRef", i)?;
            let [child_start, child_len] = slice_lanes(r.children, "Expressions.childSlice", i)?;
            let [arm_start, arm_len] = slice_lanes(r.arms, "Expressions.armSlice", i)?;
            Ok(vec![
                u128::from(r.tag),
                usize_to_u128(r.occurrence_id, "Expressions.occurrenceId", i)?,
                val_space,
                val_index,
                usize_to_u128(r.name_id, "Expressions.nameId", i)?,
                u128::from(r.binary_op),
                u128::from(r.unary_op),
                child_start,
                child_len,
                arm_start,
                arm_len,
            ])
        })
        .collect::<StateResult<Vec<_>>>()?;

    let expr_children = block
        .expr_children
        .rows
        .iter()
        .enumerate()
        .map(|(i, &c)| Ok(vec![usize_to_u128(c, "ExprChildren", i)?]))
        .collect::<StateResult<Vec<_>>>()?;

    let functions = block
        .functions
        .rows
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let [param_start, param_len] =
                slice_lanes(r.parameters, "Functions.parameterSlice", i)?;
            Ok(vec![
                usize_to_u128(r.source_table_index, "Functions.sourceTableIndex", i)?,
                usize_to_u128(r.name_id, "Functions.nameId", i)?,
                param_start,
                param_len,
                u128::from(r.return_type),
                usize_to_u128(r.body_expr_id, "Functions.bodyExprId", i)?,
            ])
        })
        .collect::<StateResult<Vec<_>>>()?;

    let parameters = block
        .parameters
        .rows
        .iter()
        .enumerate()
        .map(|(i, r)| {
            Ok(vec![
                usize_to_u128(r.name_id, "Parameters.nameId", i)?,
                u128::from(r.type_id),
            ])
        })
        .collect::<StateResult<Vec<_>>>()?;

    let patterns = block
        .patterns
        .rows
        .iter()
        .enumerate()
        .map(|(i, r)| {
            Ok(vec![
                u128::from(r.tag),
                usize_to_u128(r.name0, "Patterns.name0", i)?,
                usize_to_u128(r.name1, "Patterns.name1", i)?,
            ])
        })
        .collect::<StateResult<Vec<_>>>()?;

    let arms = block
        .arms
        .rows
        .iter()
        .enumerate()
        .map(|(i, r)| {
            Ok(vec![
                usize_to_u128(r.pattern_id, "Arms.patternId", i)?,
                usize_to_u128(r.body_expr_id, "Arms.bodyExprId", i)?,
            ])
        })
        .collect::<StateResult<Vec<_>>>()?;

    let types = block
        .types
        .rows
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let [start, len] = slice_lanes(r.constructors, "Types.constructors", i)?;
            Ok(vec![u128::from(r.kind), start, len])
        })
        .collect::<StateResult<Vec<_>>>()?;

    let constructors = block
        .constructors
        .rows
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let [start, len] = slice_lanes(r.fields, "Constructors.fields", i)?;
            Ok(vec![
                u128::from(r.owning_type),
                usize_to_u128(r.source_order, "Constructors.sourceOrder", i)?,
                start,
                len,
            ])
        })
        .collect::<StateResult<Vec<_>>>()?;

    let constructor_field_types = block
        .constructor_field_types
        .rows
        .iter()
        .map(|&t| vec![u128::from(t)])
        .collect::<Vec<_>>();

    let constants = block
        .constants
        .rows
        .iter()
        .enumerate()
        .map(|(i, r)| constant_or_data_node_lanes(r, "Constants", i))
        .collect::<StateResult<Vec<_>>>()?;

    let constant_fields = block
        .constant_fields
        .rows
        .iter()
        .enumerate()
        .map(|(i, &r)| Ok(value_ref_lanes(r, "ConstantFields", i)?.to_vec()))
        .collect::<StateResult<Vec<_>>>()?;

    let expected_types = [[0u128, 0, 0], [1, 0, 0], [2, 0, 2], [3, 2, 2]];
    if block.types.active != 4
        || types.len() < 4
        || types[..4]
            .iter()
            .zip(&expected_types)
            .any(|(row, want)| row.as_slice() != want)
    {
        return Err(wrong_rep(
            "ProgramBlock",
            "Types",
            0,
            "builtin",
            "Types table must match fixed A builtin declarations",
        ));
    }
    let expected_ctors = [[2u128, 0, 0, 0], [2, 1, 0, 2], [3, 0, 2, 1], [3, 1, 3, 2]];
    if block.constructors.active != 4
        || constructors.len() < 4
        || constructors[..4]
            .iter()
            .zip(&expected_ctors)
            .any(|(row, want)| row.as_slice() != want)
    {
        return Err(wrong_rep(
            "ProgramBlock",
            "Constructors",
            0,
            "builtin",
            "Constructors table must match fixed A builtin declarations",
        ));
    }
    let expected_cft = [[0u128], [2], [0], [3], [3]];
    if block.constructor_field_types.active != 5
        || constructor_field_types.len() < 5
        || constructor_field_types[..5]
            .iter()
            .zip(&expected_cft)
            .any(|(row, want)| row.as_slice() != want)
    {
        return Err(wrong_rep(
            "ProgramBlock",
            "ConstructorFieldTypes",
            0,
            "builtin",
            "ConstructorFieldTypes table must match fixed A builtin declarations",
        ));
    }

    let image = ImageValues {
        root,
        tables: vec![
            names,
            name_bytes,
            occurrences,
            occurrence_paths,
            expressions,
            expr_children,
            functions,
            parameters,
            patterns,
            arms,
            types,
            constructors,
            constructor_field_types,
            constants,
            constant_fields,
        ],
    };

    validate_mapped_tables(
        "ProgramBlock",
        &schema.program_header,
        &schema.program_tables,
        &image,
    )?;
    Ok(image)
}

/// Complete `StateBlock -> ImageValues` logical mapping in exact `literal_schema` / L4 lane order.
///
/// Maps the 45-lane `StateRoot` and all 6 padded state tables (`Data`, `DataFields`,
/// `EnvCells`, `ValueSlots`, `FrameSlots`, `ControlRows`), preserving root counts,
/// inactive zero rows, full capacities, and supplied stack limit and 16-byte fuels.
pub fn state_block_to_image_values(
    state: &StateBlock,
    capacities: &LiteralCapacities,
) -> StateResult<ImageValues> {
    let schema = validate_literal_schema(capacities)?;
    let r = &state.root;

    check_state_table("Data", &state.data, r.active_data, capacities.state[0])?;
    check_state_table(
        "DataFields",
        &state.data_fields,
        r.active_data_fields,
        capacities.state[1],
    )?;
    check_state_table(
        "EnvCells",
        &state.env_cells,
        r.active_env_cells,
        capacities.state[2],
    )?;
    check_state_table(
        "ValueSlots",
        &state.value_slots,
        r.active_value_slots,
        capacities.state[3],
    )?;
    check_state_table(
        "FrameSlots",
        &state.frame_slots,
        r.depth,
        capacities.state[4],
    )?;
    check_state_table(
        "ControlRows",
        &state.control_rows,
        r.control_length,
        capacities.state[5],
    )?;

    if r.stack_limit > capacities.stack_limit {
        return Err(StateFailure::Capacity {
            table: "Root.stackLimit",
            needed: r.stack_limit,
            limit: capacities.stack_limit,
            locator: Locator::field("Root.stackLimit"),
        });
    }

    let mut root = Vec::with_capacity(45);
    root.push(usize_to_u128(r.active_data, "Root.activeData", 0)?);
    root.push(usize_to_u128(
        r.active_data_fields,
        "Root.activeDataFields",
        0,
    )?);
    root.push(usize_to_u128(r.active_env_cells, "Root.activeEnvCells", 0)?);
    root.push(usize_to_u128(
        r.active_value_slots,
        "Root.activeValueSlots",
        0,
    )?);
    root.push(usize_to_u128(r.depth, "Root.depth", 0)?);
    root.push(usize_to_u128(r.control_length, "Root.controlLength", 0)?);
    root.extend(env_root_lanes(r.environment, "Root.environment", 0)?);
    root.push(r.stack_limit);
    for &b in &r.remaining_fuel {
        root.push(u128::from(b));
    }
    for &b in &r.initial_fuel {
        root.push(u128::from(b));
    }
    root.push(u128::from(r.outcome_tag));
    root.extend(value_ref_lanes(r.outcome_value, "Root.outcomeValue", 0)?);
    root.push(u128::from(r.outcome_error));

    let data = state
        .data
        .iter()
        .enumerate()
        .map(|(i, node)| constant_or_data_node_lanes(node, "Data", i))
        .collect::<StateResult<Vec<_>>>()?;

    let data_fields = state
        .data_fields
        .iter()
        .enumerate()
        .map(|(i, &v)| Ok(value_ref_lanes(v, "DataFields", i)?.to_vec()))
        .collect::<StateResult<Vec<_>>>()?;

    let env_cells = state
        .env_cells
        .iter()
        .enumerate()
        .map(|(i, cell): (usize, &EnvCell)| {
            let [val_space, val_index] = value_ref_lanes(cell.value, "EnvCells.valueRef", i)?;
            let [parent_present, parent_index] = env_root_lanes(cell.parent, "EnvCells.parent", i)?;
            Ok(vec![
                usize_to_u128(cell.name_id, "EnvCells.nameId", i)?,
                val_space,
                val_index,
                parent_present,
                parent_index,
            ])
        })
        .collect::<StateResult<Vec<_>>>()?;

    let value_slots = state
        .value_slots
        .iter()
        .enumerate()
        .map(|(i, &v)| Ok(value_ref_lanes(v, "ValueSlots", i)?.to_vec()))
        .collect::<StateResult<Vec<_>>>()?;

    let frame_slots = state
        .frame_slots
        .iter()
        .enumerate()
        .map(|(i, f)| frame_row_lanes(f, "FrameSlots", i))
        .collect::<StateResult<Vec<_>>>()?;

    let control_rows = state
        .control_rows
        .iter()
        .enumerate()
        .map(|(i, c)| control_row_lanes(c, "ControlRows", i))
        .collect::<StateResult<Vec<_>>>()?;

    let image = ImageValues {
        root,
        tables: vec![
            data,
            data_fields,
            env_cells,
            value_slots,
            frame_slots,
            control_rows,
        ],
    };

    validate_mapped_tables(
        "StateBlock",
        &schema.state_root,
        &schema.state_tables,
        &image,
    )?;
    Ok(image)
}

fn check_constant_root_matches_value(
    prepared: &PreparedA,
    value_ref: ValueRef,
    value: &Value,
    expr_id: usize,
) -> StateResult<()> {
    if value_ref.space {
        return Err(wrong_rep(
            "ProgramBlock",
            "Expressions",
            expr_id,
            "valueRef.space",
            "Literal expression valueRef must be in Constant space",
        ));
    }
    if value_ref.index >= prepared.block.constants.active {
        return Err(wrong_rep(
            "ProgramBlock",
            "Expressions",
            expr_id,
            "valueRef.index",
            "Literal expression valueRef index must be below activeConstants",
        ));
    }
    let node = prepared
        .block
        .constants
        .rows
        .get(value_ref.index)
        .ok_or_else(|| {
            wrong_rep(
                "ProgramBlock",
                "Constants",
                value_ref.index,
                "node",
                "selected ConstantNode is missing",
            )
        })?;

    match value {
        Value::UInt(n) => {
            let expected = ConstantNode {
                kind: 0,
                type_id: 0,
                ctor_id: 0,
                uint_bytes: n.to_le_bytes(),
                bool_value: false,
                fields: Slice::ZERO,
            };
            if node != &expected {
                return Err(wrong_rep(
                    "ProgramBlock",
                    "Constants",
                    value_ref.index,
                    "UInt",
                    "selected ConstantNode does not match Literal UInt value",
                ));
            }
        }
        Value::Bool(b) => {
            let expected = ConstantNode {
                kind: 1,
                type_id: 1,
                ctor_id: 0,
                uint_bytes: [0u8; 16],
                bool_value: *b,
                fields: Slice::ZERO,
            };
            if node != &expected {
                return Err(wrong_rep(
                    "ProgramBlock",
                    "Constants",
                    value_ref.index,
                    "Bool",
                    "selected ConstantNode does not match Literal Bool value",
                ));
            }
        }
        Value::List(items) => {
            let (want_ctor, want_len) = if items.is_empty() { (0, 0) } else { (1, 2) };
            if node.kind != 2
                || node.type_id != 2
                || node.ctor_id != want_ctor
                || node.uint_bytes != [0u8; 16]
                || node.bool_value
                || node.fields.len != want_len
            {
                return Err(wrong_rep(
                    "ProgramBlock",
                    "Constants",
                    value_ref.index,
                    "List",
                    "selected ConstantNode does not match Literal List root constructor",
                ));
            }
        }
        Value::Tree(tree) => {
            let (want_ctor, want_len) = match tree {
                Tree::Leaf(_) => (2, 1),
                Tree::Branch(_, _) => (3, 2),
            };
            if node.kind != 2
                || node.type_id != 3
                || node.ctor_id != want_ctor
                || node.uint_bytes != [0u8; 16]
                || node.bool_value
                || node.fields.len != want_len
            {
                return Err(wrong_rep(
                    "ProgramBlock",
                    "Constants",
                    value_ref.index,
                    "Tree",
                    "selected ConstantNode does not match Literal Tree root constructor",
                ));
            }
        }
    }
    Ok(())
}

fn check_snapshot_observation_domain(snapshot: &Snapshot, side: &'static str) -> StateResult<()> {
    if snapshot.remaining_fuel > snapshot.initial_fuel {
        return Err(StateFailure::OutsideObservationDomain {
            condition: "O1: remaining_fuel <= initial_fuel",
            locator: Locator::field(side),
        });
    }
    if snapshot.frames.len() > snapshot.stack_limit {
        return Err(StateFailure::OutsideObservationDomain {
            condition: "O2: frames.len() <= stack_limit",
            locator: Locator::field(side),
        });
    }
    if matches!(snapshot.outcome, Outcome::Pending) != snapshot.control.is_some() {
        return Err(StateFailure::OutsideObservationDomain {
            condition: "O3: outcome == Pending iff control.is_some()",
            locator: Locator::field(side),
        });
    }
    Ok(())
}

/// Isolated proposed seam constructing the A102 Step-mode post `StateBlock` from
/// an unmodified `PreparedA`, an `encode_observed_a`-built `pre_state`, and the
/// semantic `StepRecord` from `Machine::step()`.
///
/// Why this seam is separate from `encode_observed_a` and `decode_observed_a`:
/// - `encode_observed_a` constructs Canonical-boundary `StateBlock`s whose
///   `CR0 = Return` value is allocated in `Data` space (`space = true`), whereas
///   the A102 fragment requires a Step-mode mid-segment post image whose
///   `CR0 = Return` holds the selected `Constant`-space `ValueRef` (`space = false`)
///   and preserves all five runtime arenas from `pre_state`.
/// - Under the `layout_a` trusted-diagnostic exposure contract, `decode_observed_a`
///   requires every input `StateBlock` to come from `encode_observed_a` over a
///   trusted `Snapshot`; it must not be called on constructed Step-mode rows.
///
/// # Unchecked Preconditions (H1 & Trusted Provenance)
/// - **H1 recursive equality and drop hazard**: This function uses derived `!=`/`==` on
///   `Program`, `Value`, `Environment`, and `Vec<Frame>` (and `Clone`/`Drop` on
///   recursive AST/tree structures). It is for trusted host diagnostics only; deep
///   inputs can overflow the native stack and abort the process without returning `Err`.
/// - **Unmodified `PreparedA`**: `prepared` must be the exact, unmodified value returned
///   by `prepare_a`. For `Value::List` and `Value::Tree`, `check_constant_root_matches_value`
///   inspects only the root constructor `ConstantNode` (`kind`, `type_id`, `ctor_id`,
///   zero scalar lanes, `fields.len`) and does not walk child `Constants`/`ConstantFields`;
///   it relies on intact `prepare_a` metadata, `Constants`, and `ConstantFields`.
/// - **Exact `encode_observed_a` `pre_state` provenance**: `pre_state` must be the exact
///   output of `encode_observed_a(prepared, state_capacities, &record.before)` over the
///   same trusted `record.before` and capacities, not unrelated or mutated caller rows.
///   This helper checks `pre_state` fuel, Pending outcome, and `ControlRows`, then clones
///   `pre_state`'s five arenas and unchanged Root lanes without re-encoding `record.before`
///   or performing hand-built Step-decoder validation.
/// - **Trusted `StepRecord`**: `record` must be an actual `Machine::step()` record
///   (`StepRecord` is a plain struct, not an authenticated capability).
pub fn encode_a102_step_post_state(
    prepared: &PreparedA,
    state_capacities: &StateCapacities,
    pre_state: &StateBlock,
    record: &StepRecord,
) -> StateResult<(StateBlock, usize, ValueRef)> {
    if state_capacities.data != prepared.capacities.data {
        return Err(wrong_rep(
            "StateBlock",
            "Root",
            0,
            "D",
            "StateCapacities.data must equal PreparedA.capacities.data",
        ));
    }
    if record.before.program.as_ref() != prepared.full_program.as_ref() {
        return Err(StateFailure::ProgramMismatch {
            locator: Locator::field("StepRecord.before.program"),
        });
    }
    if record.after.program.as_ref() != prepared.full_program.as_ref() {
        return Err(StateFailure::ProgramMismatch {
            locator: Locator::field("StepRecord.after.program"),
        });
    }

    check_snapshot_observation_domain(&record.before, "StepRecord.before")?;
    check_snapshot_observation_domain(&record.after, "StepRecord.after")?;

    if record.before.outcome != Outcome::Pending {
        return Err(StateFailure::OutsideObservationDomain {
            condition: "O4: StepRecord.before.outcome must be Pending",
            locator: Locator::field("StepRecord.before.outcome"),
        });
    }
    if record.rule_start_line as u128 != RULE_START_LINE || u128::from(record.charge) != RULE_CHARGE
    {
        return Err(StateFailure::OutsideObservationDomain {
            condition: "A102 requires StepRecord rule_start_line=102 and charge=1",
            locator: Locator::field("StepRecord.rule_start_line"),
        });
    }

    let eval_expr = match &record.before.control {
        Some(Control::Eval(expr)) => expr,
        _ => {
            return Err(StateFailure::OutsideObservationDomain {
                condition: "A102 requires StepRecord.before.control to be Some(Control::Eval(..))",
                locator: Locator::field("StepRecord.before.control"),
            });
        }
    };
    let literal_value = match eval_expr.as_ref() {
        Expr::Literal(val) => val,
        _ => {
            return Err(StateFailure::OutsideObservationDomain {
                condition: "A102 requires evaluated expression to be Expr::Literal",
                locator: Locator::field("StepRecord.before.control"),
            });
        }
    };
    let returned_value = match &record.after.control {
        Some(Control::Return(val)) => val,
        _ => {
            return Err(StateFailure::OutsideObservationDomain {
                condition: "A102 requires StepRecord.after.control to be Some(Control::Return(..))",
                locator: Locator::field("StepRecord.after.control"),
            });
        }
    };
    if literal_value != returned_value
        || record.after.outcome != Outcome::Pending
        || record.before.environment != record.after.environment
        || record.before.frames != record.after.frames
        || record.before.stack_limit != record.after.stack_limit
        || record.before.initial_fuel != record.after.initial_fuel
    {
        return Err(StateFailure::OutsideObservationDomain {
            condition: "A102 StepRecord before/after framing mismatch",
            locator: Locator::field("StepRecord.after"),
        });
    }

    let expected_after_fuel = record
        .before
        .remaining_fuel
        .checked_sub(u128::from(record.charge))
        .ok_or(StateFailure::OutsideObservationDomain {
            condition: "A102 requires remaining_fuel >= charge (1)",
            locator: Locator::field("StepRecord.before.remaining_fuel"),
        })?;
    if record.after.remaining_fuel != expected_after_fuel {
        return Err(StateFailure::OutsideObservationDomain {
            condition: "A102 StepRecord.after.remaining_fuel must equal before.remaining_fuel - 1",
            locator: Locator::field("StepRecord.after.remaining_fuel"),
        });
    }

    let pre_fuel = u128::from_le_bytes(pre_state.root.remaining_fuel);
    let pre_initial_fuel = u128::from_le_bytes(pre_state.root.initial_fuel);
    if pre_fuel != record.before.remaining_fuel || pre_initial_fuel != record.before.initial_fuel {
        return Err(wrong_rep(
            "StateBlock",
            "Root",
            0,
            "remainingFuel",
            "pre_state fuel bytes must equal StepRecord.before fuel values",
        ));
    }
    if pre_state.root.outcome_tag != 0
        || pre_state.root.outcome_value != ValueRef::default()
        || pre_state.root.outcome_error != 0
        || pre_state.root.control_length != 1
    {
        return Err(wrong_rep(
            "StateBlock",
            "Root",
            0,
            "controlLength",
            "A102 pre_state requires Pending outcome with zero payload and controlLength=1",
        ));
    }

    let control_cap =
        usize::try_from(state_capacities.control_rows).map_err(|_| host_err("ControlRows", 0))?;
    if control_cap == 0 || pre_state.control_rows.len() != control_cap {
        return Err(wrong_rep(
            "StateBlock",
            "ControlRows",
            0,
            "length",
            "pre_state ControlRows length must equal nonzero declared capacity",
        ));
    }
    let cr0 = &pre_state.control_rows[0];
    let selected_expr_id = cr0.expr_id;
    let expected_cr0 = ControlRow {
        tag: 0,
        expr_id: selected_expr_id,
        ..ControlRow::default()
    };
    if cr0 != &expected_cr0 {
        return Err(wrong_rep(
            "StateBlock",
            "ControlRows",
            0,
            "tag",
            "A102 pre_state CR0 must be Eval with all unused columns zero",
        ));
    }
    for (row_idx, row) in pre_state.control_rows.iter().enumerate().skip(1) {
        if row != &ControlRow::default() {
            return Err(wrong_rep(
                "StateBlock",
                "ControlRows",
                row_idx,
                "padding",
                "A102 pre_state control rows >= 1 must be zero",
            ));
        }
    }

    if selected_expr_id >= prepared.block.expressions.active {
        return Err(wrong_rep(
            "StateBlock",
            "ControlRows",
            0,
            "exprId",
            "selected exprId must be below activeExpressions",
        ));
    }
    let occurrence = prepared.occurrences.get(selected_expr_id).ok_or_else(|| {
        wrong_rep(
            "ProgramBlock",
            "Occurrences",
            selected_expr_id,
            "metadata",
            "PreparedA occurrence metadata missing for selected exprId",
        )
    })?;
    match occurrence.source.as_ref() {
        Expr::Literal(occ_val) if occ_val == literal_value => {}
        _ => {
            return Err(wrong_rep(
                "ProgramBlock",
                "Expressions",
                selected_expr_id,
                "source",
                "PreparedA occurrence at selected exprId does not match StepRecord Literal",
            ));
        }
    }

    let expr_row = prepared
        .block
        .expressions
        .rows
        .get(selected_expr_id)
        .ok_or_else(|| {
            wrong_rep(
                "ProgramBlock",
                "Expressions",
                selected_expr_id,
                "row",
                "ExpressionRow missing for selected exprId",
            )
        })?;
    let selected_constant_ref = expr_row.value;
    if expr_row.tag != 0
        || expr_row.occurrence_id != selected_expr_id
        || expr_row.name_id != 0
        || expr_row.binary_op != 0
        || expr_row.unary_op != 0
        || expr_row.children != Slice::ZERO
        || expr_row.arms != Slice::ZERO
    {
        return Err(wrong_rep(
            "ProgramBlock",
            "Expressions",
            selected_expr_id,
            "tag",
            "selected ExpressionRow must be a Literal row with zero unused columns",
        ));
    }
    check_constant_root_matches_value(
        prepared,
        selected_constant_ref,
        literal_value,
        selected_expr_id,
    )?;

    let mut post_state = pre_state.clone();
    post_state.root.remaining_fuel = expected_after_fuel.to_le_bytes();
    post_state.control_rows[0] = ControlRow {
        tag: 1,
        value: selected_constant_ref,
        ..ControlRow::default()
    };

    Ok((post_state, selected_expr_id, selected_constant_ref))
}

/// Build the pre/post `StateBlock`s and `LiteralTransitionRequest` from an
/// unmodified `PreparedA` and an actual A102 `StepRecord`.
///
/// # Unchecked Preconditions (H1 & Trusted Provenance)
/// - `prepared` must be the wholly unmodified result of `prepare_a`, and `record`
///   must be an actual trusted `Machine::step()` observation (not authenticated by type).
/// - Unlike [`produce_a102_literal_transition`], which computes its own fresh
///   `PreparedA` and entry `StepRecord`, this raw helper does not verify that `record`
///   was produced by `Machine::step()` or disambiguate duplicate literal occurrences
///   beyond `encode_observed_a`'s least-equal occurrence selection.
/// - Subject to the H1 recursive equality/clone/drop preconditions documented on
///   [`encode_a102_step_post_state`].
pub fn produce_a102_literal_transition_from_step(
    prepared: &PreparedA,
    capacities: &LiteralCapacities,
    record: &StepRecord,
    identity: &LiteralSourceIdentity,
) -> StateResult<(
    StateBlock,
    StateBlock,
    usize,
    ValueRef,
    LiteralTransitionRequest,
)> {
    validate_literal_schema(capacities)?;
    let state_caps = core_state_capacities(capacities)?;
    let pre_state = encode_observed_a(prepared, &state_caps, &record.before)?;
    let (post_state, selected_expr_id, selected_constant_ref) =
        encode_a102_step_post_state(prepared, &state_caps, &pre_state, record)?;

    let program_values = program_block_to_image_values(&prepared.block, capacities)?;
    let pre_values = state_block_to_image_values(&pre_state, capacities)?;
    let post_values = state_block_to_image_values(&post_state, capacities)?;

    let request = LiteralTransitionRequest {
        capacities: capacities.clone(),
        identity: identity.clone(),
        values: LiteralValues {
            program: program_values,
            pre: pre_values,
            post: post_values,
            rule_tag: RULE_TAG,
            charge: RULE_CHARGE,
        },
    };

    Ok((
        pre_state,
        post_state,
        selected_expr_id,
        selected_constant_ref,
        request,
    ))
}

/// Source-derived A102 transition producer from a complete `Arc<Program>`,
/// explicit `LiteralCapacities`, `initial_fuel`, `stack_limit`, and `LiteralSourceIdentity`.
///
/// Sequencing and failure precedence (C1):
/// 1. Validates `literal_schema(capacities)` and converts `core_program_capacities`.
/// 2. Runs the cheap $O(1)$ entry/stack/fuel checks (`Expr::Literal` entry,
///    `stack_limit <= capacities.stack_limit`, `initial_fuel > 0`) *before* calling
///    `prepare_a`, avoiding padded program-table allocations on those refusals.
///    Consequently, on combined-invalid inputs that also exceed program table
///    capacities in `prepare_a`, these entry/stack/fuel refusals take precedence
///    over `prepare_a` `Capacity`/`HostUnrepresentable` errors (unlike candidate02).
/// 3. Calls `prepare_a` over the complete program, runs `Machine::new(...).step()`,
///    encodes `pre_state` via `encode_observed_a`, constructs the Step-mode
///    `post_state` via `encode_a102_step_post_state`, and maps all tables.
///
/// Trusted host diagnostic only (H1): valid deep ASTs or trees can still overflow
/// the native call stack during recursive drop/clone/equality.
pub fn produce_a102_literal_transition(
    program: Arc<Program>,
    capacities: &LiteralCapacities,
    initial_fuel: u128,
    stack_limit: usize,
    identity: &LiteralSourceIdentity,
) -> StateResult<ProducedLiteralTransition> {
    validate_literal_schema(capacities)?;
    let prog_caps = core_program_capacities(capacities)?;

    if !matches!(program.entry.as_ref(), Expr::Literal(_)) {
        return Err(StateFailure::OutsideObservationDomain {
            condition: "A102 entry transition requires Program.entry to be Expr::Literal",
            locator: Locator {
                expression: Some(0),
                function: None,
                field: "Program.entry",
                item: 0,
            },
        });
    }

    let stack_limit_u128 = usize_to_u128(stack_limit, "Snapshot.stack_limit", 0)?;
    if stack_limit_u128 > capacities.stack_limit {
        return Err(StateFailure::Capacity {
            table: "Root.stackLimit",
            needed: stack_limit_u128,
            limit: capacities.stack_limit,
            locator: Locator::field("Snapshot.stack_limit"),
        });
    }

    if initial_fuel == 0 {
        return Err(StateFailure::OutsideObservationDomain {
            condition: "A102 requires nonzero initial_fuel for charged rule 102",
            locator: Locator::field("Snapshot.remaining_fuel"),
        });
    }

    let prepared = prepare_a(program.clone(), prog_caps)?;

    let mut machine = Machine::new(program, initial_fuel, stack_limit);
    let step_record = machine
        .step()
        .ok_or(StateFailure::OutsideObservationDomain {
            condition: "Machine::step returned None on initial state",
            locator: Locator::field("Machine.step"),
        })?;

    let (pre_state, post_state, selected_expr_id, selected_constant_ref, request) =
        produce_a102_literal_transition_from_step(&prepared, capacities, &step_record, identity)?;

    Ok(ProducedLiteralTransition {
        prepared,
        step_record,
        pre_state,
        post_state,
        selected_expr_id,
        selected_constant_ref,
        request,
    })
}

/// Build a candidate02 `LiteralTransitionRequest` directly from a complete
/// `Arc<Program>`, explicit capacities, fuel, stack limit, and source identity.
pub fn build_literal_transition_request(
    program: Arc<Program>,
    capacities: &LiteralCapacities,
    initial_fuel: u128,
    stack_limit: usize,
    identity: &LiteralSourceIdentity,
) -> StateResult<LiteralTransitionRequest> {
    Ok(
        produce_a102_literal_transition(program, capacities, initial_fuel, stack_limit, identity)?
            .request,
    )
}

/// Build a `ProducedLiteralTransition` and verify its public statement layout
/// and C6 host representation checks via `LiteralLayout::new` and `LiteralLayout::statement`,
/// without invoking any SDK keygen, prover, or verifier.
///
/// Admits `identity`, `literal_schema(capacities)`, and `max_statement_lanes`
/// via `LiteralLayout::new` before calling `produce_a102_literal_transition`,
/// so statement-lane bounds are enforced prior to `prepare_a`, `Machine::step`,
/// `encode_observed_a`, and table/image allocations.
pub fn build_checked_literal_transition_request(
    program: Arc<Program>,
    capacities: &LiteralCapacities,
    initial_fuel: u128,
    stack_limit: usize,
    identity: &LiteralSourceIdentity,
    max_statement_lanes: usize,
) -> Result<(ProducedLiteralTransition, Vec<u128>), Error> {
    let layout = LiteralLayout::new(capacities, identity, max_statement_lanes)?;
    let produced =
        produce_a102_literal_transition(program, capacities, initial_fuel, stack_limit, identity)
            .map_err(|e| error("sherlock-producer", format!("{e:?}")))?;
    let statement = layout.statement(&produced.request.values)?;
    Ok((produced, statement))
}
