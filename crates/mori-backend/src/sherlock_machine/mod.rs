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
pub mod literal_bridge;
pub mod literal_witness;
pub use literal_step::{
    emit_literal_step, literal_schema, LiteralCapacities, LiteralConstructionError,
    LiteralEnvelope, LiteralImage, LiteralLane, LiteralSchema, LiteralStepBlock,
    LiteralStepInputs, LiteralTable,
};
pub use literal_witness::{
    build_checked_literal_transition_request, build_literal_transition_request,
    core_program_capacities, core_state_capacities, encode_a102_step_post_state,
    produce_a102_literal_transition, produce_a102_literal_transition_from_step,
    program_block_to_image_values, state_block_to_image_values, validate_literal_schema,
    ProducedLiteralTransition,
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
