//! Source-only bounded scalar fetch, composable A102 Literal instructions, and
//! trusted-source A102 witness construction.
//!
//! The `fetch`, `enabled_fetch`, and `literal_step` constructors emit pinned SDK
//! instructions over caller-supplied table shapes and operands without loading,
//! evaluating, compiling, or proving them. The child module `literal_bridge`
//! adds statement binding and local native and ledger entry points around the
//! unmodified `literal_step` fragment; it is uncompiled and unobserved. The child
//! module `literal_witness` is a host diagnostic producer (H1, zero native
//! authority) that invokes `mori_core::sherlock::layout_a::prepare_a`,
//! `mori_core::sherlock::Machine::new(...).step()` for the first A102 `StepRecord`,
//! `encode_observed_a` for the canonical pre-state, and an isolated Step-mode
//! Constant-space post-state constructor without calling `decode_observed_a` on
//! constructed Step-mode rows. Full `StepInv` (`O1`..`O4`), `uiValid`, `F4`,
//! `OBL.G4` canonical byte binding, `projectA`, and exactly-one dispatch remain
//! open caller obligations.

use midnight_transient_crypto::curve::Fr;
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::ir_types::IrType;
use midnight_zkir_v3::{Identifier, Instruction, IrSource};
use mori_core::{Type, Value};
use std::collections::BTreeSet;
use std::sync::Arc;

mod fetch;
pub use fetch::emit_fetch_block;

mod enabled_fetch;
pub use enabled_fetch::{
    emit_enabled_fetch_block, EnabledFetchBlock, EnabledFetchInputs, EnabledTypedOperand,
};

pub mod literal_host;
mod literal_step;
mod literal_ui_valid;
pub mod literal_bridge;
pub mod literal_witness;
pub mod return_publication_step;
pub mod return_publication_witness;
pub mod literal_complete_execution;
pub mod literal_complete_bridge;
pub mod error_publication_step;
pub mod fuel_exhaustion_step;
pub use literal_step::{
    emit_literal_step, literal_schema, LiteralCapacities, LiteralConstructionError,
    LiteralEnvelope, LiteralImage, LiteralLane, LiteralSchema, LiteralStepBlock,
    LiteralStepInputs, LiteralTable,
};
pub use literal_ui_valid::{
    emit_literal_ui_valid, emit_literal_ui_valid_with_capacities, is_reserved_ui_valid_variable,
    schema_capacities, LiteralUiValidBlock, LiteralUiValidInputs,
};
pub use literal_witness::{
    build_checked_literal_transition_request, build_literal_transition_request,
    core_program_capacities, core_state_capacities, encode_a102_step_post_state,
    produce_a102_literal_transition, produce_a102_literal_transition_from_step,
    program_block_to_image_values, state_block_to_image_values, validate_literal_schema,
    ProducedLiteralTransition,
};
pub use fuel_exhaustion_step::{
    emit_fuel_exhaustion_step, is_reserved_fuel_exhaustion_variable, FuelExhaustionStepBlock,
    FuelExhaustionStepInputs, FUEL_EXHAUSTION_RULE_CHARGE, FUEL_EXHAUSTION_RULE_ID,
    FUEL_EXHAUSTION_RULE_START_LINE, FUEL_EXHAUSTION_RULE_TAG,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Column {
    pub name: String,
    pub kind: Type,
}

/// Host construction shape, not a machine encoding or resource admission.
/// No fixed maximum is chosen here. A later owner must admit concrete bounds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableShape {
    capacity: usize,
    columns: Vec<Column>,
    cells: usize,
}

pub use literal_host::ConstructionError;

impl TableShape {
    pub fn new(capacity: usize, columns: Vec<Column>) -> Result<Self, ConstructionError> {
        if capacity == 0 || columns.is_empty() {
            return Err(ConstructionError::EmptyShape);
        }
        let mut names = BTreeSet::new();
        for column in &columns {
            if column.name.is_empty() || !names.insert(column.name.clone()) {
                return Err(ConstructionError::EmptyOrRepeatedColumnName);
            }
        }
        let cells = capacity.checked_mul(columns.len()).ok_or(ConstructionError::SizeOverflow)?;
        cells.checked_add(columns.len()).and_then(|n| n.checked_add(2))
            .ok_or(ConstructionError::SizeOverflow)?;
        u128::try_from(capacity).map_err(|_| ConstructionError::SizeOverflow)?;
        Ok(Self { capacity, columns, cells })
    }

    pub fn capacity(&self) -> usize { self.capacity }
    pub fn columns(&self) -> &[Column] { &self.columns }

    fn check_rows<T>(&self, rows: &[Vec<T>]) -> Result<(), ConstructionError> {
        if rows.len() != self.capacity { return Err(ConstructionError::RowCount); }
        for (row, cells) in rows.iter().enumerate() {
            if cells.len() != self.columns.len() {
                return Err(ConstructionError::ColumnCount { row });
            }
        }
        Ok(())
    }
}

/// Caller-owned Native operands. The block constrains count/index as UInt128
/// and every cell according to TableShape; all rows, including inactive rows,
/// are supplied and preserved. No canonical padding convention is imposed.
#[derive(Clone, Debug, PartialEq)]
pub struct FetchInputs {
    pub active_rows: Operand,
    pub index: Operand,
    pub rows: Vec<Vec<Operand>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypedOperand {
    pub kind: Type,
    pub value: Operand,
}

/// An instruction fragment, without public bindings or an Output terminator.
/// Preserved operands are exact aliases, not separately supplied after-state.
pub struct FetchBlock {
    pub instructions: Vec<Instruction>,
    pub selected: Vec<TypedOperand>,
    pub preserved: FetchInputs,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FieldRole {
    ActiveRows,
    Index,
    Cell { row: usize, column: usize },
    Selected { column: usize },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub role: FieldRole,
    pub kind: Type,
}

/// Standalone construction for review and future normal comparison. No SDK
/// execution occurs. Inputs follow PublicInput encounter order; outputs follow
/// the Output operand list. See MAPPING.md for the complete positional map.
pub struct EmittedFetch {
    pub ir: IrSource,
    pub input_fields: Vec<Field>,
    pub output_fields: Vec<Field>,
    shape: TableShape,
}

pub struct FetchWitness {
    pub active_rows: u128,
    pub index: u128,
    pub rows: Vec<Vec<Value>>,
}

impl EmittedFetch {
    /// Encode supplied scalars in public-input order. Only host shape/type is
    /// checked. In particular, this does not select a row, compute an expected
    /// result, or decide index membership/constraint acceptance.
    pub fn pack_witness(&self, witness: &FetchWitness) -> Result<Vec<Fr>, ConstructionError> {
        self.shape.check_rows(&witness.rows)?;
        let mut packed = Vec::with_capacity(self.shape.cells + 2);
        packed.push(Fr::from(witness.active_rows));
        packed.push(Fr::from(witness.index));
        for (row, cells) in witness.rows.iter().enumerate() {
            for (column, value) in cells.iter().enumerate() {
                if value.ty() != self.shape.columns[column].kind {
                    return Err(ConstructionError::CellType { row, column });
                }
                packed.push(Fr::from(match value {
                    Value::UInt(n) => n.0,
                    Value::Bool(b) => u128::from(*b),
                }));
            }
        }
        Ok(packed)
    }
}

fn immediate(n: u128) -> Operand { Operand::Immediate(Fr::from(n)) }

fn constrain(instructions: &mut Vec<Instruction>, value: Operand, kind: Type) {
    instructions.push(match kind {
        Type::UInt => Instruction::ConstrainBits { val: value, bits: 128 },
        Type::Bool => Instruction::ConstrainToBoolean { val: value },
    });
}

/// Construct a public-input wrapper using the existing backend's
/// PublicInput/range/Impact convention. Binding/security review is separate.
pub fn emit_public_fetch(shape: &TableShape) -> Result<EmittedFetch, ConstructionError> {
    let mut instructions = Vec::new();
    let mut input_fields = vec![
        Field { role: FieldRole::ActiveRows, kind: Type::UInt },
        Field { role: FieldRole::Index, kind: Type::UInt },
    ];
    for row in 0..shape.capacity {
        for (column, col) in shape.columns.iter().enumerate() {
            input_fields.push(Field { role: FieldRole::Cell { row, column }, kind: col.kind });
        }
    }
    let mut operands = Vec::with_capacity(input_fields.len());
    for (position, field) in input_fields.iter().enumerate() {
        let id = Identifier(format!("%row_fetch.input.{position}"));
        instructions.push(Instruction::PublicInput {
            guard: None, val_t: IrType::Native, output: id.clone(),
        });
        let value = Operand::Variable(id);
        constrain(&mut instructions, value.clone(), field.kind);
        instructions.push(Instruction::Impact {
            guard: immediate(1), inputs: vec![value.clone()],
        });
        operands.push(value);
    }
    let inputs = FetchInputs {
        active_rows: operands[0].clone(),
        index: operands[1].clone(),
        rows: operands[2..].chunks(shape.columns.len()).map(|row| row.to_vec()).collect(),
    };
    let block = emit_fetch_block(shape, &inputs, "row_fetch")?;
    instructions.extend(block.instructions);
    let mut outputs: Vec<_> = block.selected.into_iter().map(|cell| cell.value).collect();
    outputs.push(block.preserved.active_rows);
    outputs.push(block.preserved.index);
    outputs.extend(block.preserved.rows.into_iter().flatten());
    let mut output_fields: Vec<_> = shape.columns.iter().enumerate().map(|(column, col)| {
        Field { role: FieldRole::Selected { column }, kind: col.kind }
    }).collect();
    output_fields.extend(input_fields.iter().cloned());
    instructions.push(Instruction::Output { vals: outputs });
    let ir = IrSource {
        version: Default::default(),
        inputs: vec![],
        outputs: vec![IrType::Native; output_fields.len()],
        do_communications_commitment: false,
        instructions: Arc::new(instructions),
    };
    Ok(EmittedFetch { ir, input_fields, output_fields, shape: shape.clone() })
}

#[cfg(test)]
mod normal_tests;

#[cfg(test)]
mod identifier_rejection_tests;

#[cfg(test)]
mod enabled_fetch_tests;

#[cfg(test)]
mod literal_step_normal_tests;

#[cfg(test)]
mod literal_witness_normal_tests;

#[cfg(test)]
mod literal_witness_native_tests;

#[cfg(test)]
mod return_publication_step_normal_tests;

#[cfg(test)]
mod return_publication_witness_normal_tests;

#[cfg(test)]
mod literal_complete_execution_normal_tests;

#[cfg(test)]
mod literal_complete_bridge_normal_tests;

#[cfg(test)]
mod error_publication_step_normal_tests;

#[cfg(test)]
mod fuel_exhaustion_step_normal_tests;

pub mod b_scalar_arithmetic;
pub use b_scalar_arithmetic::{
    emit_b_scalar_arithmetic_add, emit_public_b_scalar_arithmetic_add, u128_to_le_immediates,
    BChargedCounters, BOptValueRef, BScalarArithmeticAddBlock, BScalarArithmeticAddInputs,
    BScalarArithmeticConstructionError, BScalarArithmeticField, BScalarArithmeticFieldRole,
    BScalarArithmeticRegister, BScalarArithmeticWitness, BScalarHelperRestLanes,
    BScalarStagedLanes, BU128Lanes, BValueRef, BValueRowLanes, BValueSpace, BValueTableRead,
    EmittedBScalarArithmeticAdd, CONST_TAG_U128, DATA_TAG_U128, HELPER_KIND_SCALAR,
    HELPER_PRESENT, OPT_ABSENT, OPT_PRESENT, PRIM_ERROR_ARITH_OVERFLOW, PRIM_ERROR_SORT_MISMATCH,
    SCALAR_MODE_ADD, SCALAR_PHASE_DONE, SCALAR_PHASE_FIRST, SCALAR_PHASE_SECOND,
    SCALAR_STAGED_TAG_BOOL_VALUE, SCALAR_STAGED_TAG_NONE, SCALAR_STAGED_TAG_PENDING_NODE,
    SCALAR_STAGED_TAG_PRIM_ERROR, SCALAR_STAGED_TAG_SELECTOR, SCALAR_STAGED_TAG_TYPE_TEST,
    SCALAR_STAGED_TAG_U128_VALUE, SITE_RET_RIGHT, VALID_VALUE_TAG_COUNT, VALUE_TAG_BOOL,
    VALUE_TAG_BRANCH, VALUE_TAG_CONS, VALUE_TAG_LEAF, VALUE_TAG_NIL, VALUE_TAG_U128,
};

#[cfg(test)]
mod b_scalar_arithmetic_normal_tests;

pub mod b_scalar_subtraction;
pub use b_scalar_subtraction::{
    emit_b_scalar_arithmetic_sub, emit_public_b_scalar_arithmetic_sub,
    BScalarArithmeticSubBlock, BScalarArithmeticSubInputs, EmittedBScalarArithmeticSub,
    SCALAR_MODE_SUB,
};

pub mod b_scalar_completion;
pub use b_scalar_completion::{
    emit_b_scalar_exit_add_u128, emit_b_scalar_failure_add, BDataRowLanes, BDataTableLanes,
    BHelperResultLanes, BOutcomeLanes, BScalarCompletionConstructionError,
    BScalarCompletionCounters, BScalarExitAddU128Block, BScalarExitAddU128Inputs,
    BScalarFailureAddBlock, BScalarFailureAddInputs, EXHAUST_REASON_DATA_FIELDS,
    EXHAUST_REASON_DATA_NODES, EXHAUST_REASON_ENVIRONMENT_CELLS, EXHAUST_REASON_FUEL,
    EXHAUST_REASON_HELPER_TASKS, EXHAUST_REASON_HELPER_WORK,
    EXHAUST_REASON_SERIALIZATION_BYTES, HELPER_RESULT_TAG_APPEND_DONE,
    HELPER_RESULT_TAG_BIND_BAD, HELPER_RESULT_TAG_BIND_OK, HELPER_RESULT_TAG_ENV_FOUND,
    HELPER_RESULT_TAG_ENV_MISSING, HELPER_RESULT_TAG_ENV_UPDATED, HELPER_RESULT_TAG_EQUAL_DONE,
    HELPER_RESULT_TAG_FIND_FOUND, HELPER_RESULT_TAG_FIND_MISSING,
    HELPER_RESULT_TAG_MATCH_MATCHED, HELPER_RESULT_TAG_MATCH_NO_MATCH,
    HELPER_RESULT_TAG_SCALAR_ERROR, HELPER_RESULT_TAG_SCALAR_SELECTOR,
    HELPER_RESULT_TAG_SCALAR_TYPE_TEST, HELPER_RESULT_TAG_SCALAR_VALUE,
    HELPER_RESULT_TAG_VALIDATED, OUTCOME_TAG_EXHAUSTED, OUTCOME_TAG_HALTED,
    OUTCOME_TAG_REJECTED, OUTCOME_TAG_RUNNING, VALUE_SPACE_CONST, VALUE_SPACE_DATA,
};
pub mod b_scalar_subtraction_completion;
pub use b_scalar_subtraction_completion::{
    emit_b_scalar_exit_sub_u128, emit_b_scalar_failure_sub, BScalarExitSubU128Block,
    BScalarExitSubU128Inputs, BScalarFailureSubBlock, BScalarFailureSubInputs,
    BScalarSubCompletionConstructionError, BScalarSubCompletionCounters, BSubPublishState,
};
pub mod b_return_right_completion;
pub use b_return_right_completion::{
    emit_k_machine_primitive_error, emit_k_machine_primitive_value,
    emit_k_machine_return_right_add, BControlRow, BFrameRow, BLegacyCompletionState,
    BReturnRightCompletionEmitError, BINARY_OP_ADD, BINARY_OP_BRANCH, BINARY_OP_CONS,
    BINARY_OP_EQ, BINARY_OP_LT, BINARY_OP_MUL, BINARY_OP_SUB, CONTROL_TAG_BOUND,
    CONTROL_TAG_CASES, CONTROL_TAG_CASE_RESULT, CONTROL_TAG_ENTER, CONTROL_TAG_ERROR,
    CONTROL_TAG_EVAL, CONTROL_TAG_INVOKE, CONTROL_TAG_PRIM_RESULT, CONTROL_TAG_RETURN,
    CONTROL_TAG_SCHEDULE_NEXT, FRAME_TAG_ARGS_F,
    FRAME_TAG_CONSTRUCTOR_FIELDS_EXTENSION_RESERVED, FRAME_TAG_IF_F, FRAME_TAG_LEFT_F,
    FRAME_TAG_LET_F, FRAME_TAG_MATCH_F, FRAME_TAG_RETURN_F, FRAME_TAG_RIGHT_F,
    FRAME_TAG_SCOPE_F, FRAME_TAG_UNARY_F, PRIM_OUTCOME_TAG_ERROR, PRIM_OUTCOME_TAG_VALUE,
    SITE_PRIM,
};
pub mod b_return_right_subtraction;
pub use b_return_right_subtraction::{
    emit_k_machine_return_right_sub, emit_k_machine_return_right_sub_block,
};
pub mod b_admin_publication;
pub use b_admin_publication::{
    emit_b_add_to_publication_chain, emit_b_admin_publish_p1, emit_b_sub_to_publication_chain,
    emit_k_admin_publish_error, emit_k_admin_publish_value, BAddToPublicationChainBlock,
    BAddToPublicationChainInputs, BAdminPublicationError, BAdminPublishBlock,
    BAdminPublishInputs, BAdminPublishState, BSubToPublicationChainBlock,
    BSubToPublicationChainInputs, ADMIN_PUBLISH_CHARGE, ERROR_CODE_MAX, ERROR_CODE_MIN,
};

#[cfg(test)]
mod b_scalar_completion_normal_tests;
#[cfg(test)]
mod b_return_right_completion_normal_tests;
#[cfg(test)]
mod b_admin_publication_normal_tests;

pub mod c6_a135_push;
pub mod c6_data_canonical;
pub mod c6_meta_canonical;
pub mod c6_program_context_canonical;
pub mod c6_push_step;
pub use c6_a135_push::{
    c6_lanes_to_immediates, c6_push_family, emit_c6_push_observed_operands, C6PushOperands,
    C6_A135_FAMILY, C6_A140_FAMILY,
};
pub use c6_data_canonical::{
    c6_data_geometry, emit_c6_data_canonical_block, C6DataCanonicalBlock,
    C6DataCanonicalEmitter, C6DataCanonicalError, C6DataCanonicalInputs, C6DataGeometry,
};
pub use c6_meta_canonical::{
    emit_c6_meta_bound_canonical_block, emit_c6_meta_canonical_block, C6MetaBoundCanonicalInputs,
    C6MetaCanonicalBlock, C6MetaCanonicalEmitter, C6MetaCanonicalError, C6MetaCanonicalInputs,
    C6MetaConfigBinding,
};
pub use c6_program_context_canonical::{
    emit_c6_program_context_canonical_block, emit_c6_program_context_ctx_binding_block,
    C6ProgramContextCanonicalBlock, C6ProgramContextCanonicalEmitter,
    C6ProgramContextCanonicalError, C6ProgramContextCanonicalInputs,
    C6ProgramContextCtxBindingInputs,
};
pub use c6_push_step::{
    check_c6_push_step_circuit, decode_padded_state_to_owned,
    emit_c6_push_step_block, encode_owned_state_to_padded, meta_admission_bounds_frames_offset,
    witness_from_host_pair, witness_from_owner_step, C6PaddedCarrierGeometry,
    C6PaddedStateLanes, C6PushFamilyParams, C6PushHostPrefixLanes, C6PushStatementBindingMode,
    C6PushStepBlock, C6PushStepCircuit, C6PushStepError, C6PushStepInputs,
    C6PushStepWitness, C6_PADDED_CARRIER_VERSION,
};
