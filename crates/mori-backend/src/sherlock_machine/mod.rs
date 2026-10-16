//! Reusable source-only bounded scalar row fetch, pending integration/review.
//!
//! These constructors emit pinned SDK instructions. They do not load, check,
//! evaluate, compile or prove them. A caller supplies the table shape and all
//! witness cells; no program or machine transition is evaluated by this module.
//! This is not a Sherlock machine layout, codec, dispatch table or profile.

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstructionError {
    EmptyShape,
    EmptyOrRepeatedColumnName,
    SizeOverflow,
    RowCount,
    ColumnCount { row: usize },
    CellType { row: usize, column: usize },
    EmptyScope,
    UnprefixedVariable,
    TemporaryNameCollision,
}

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
