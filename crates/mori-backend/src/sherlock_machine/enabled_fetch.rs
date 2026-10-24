//! Separate enabled scalar fetch implementing Opus's construction requirements.
//! Source only: no SDK load/check/evaluation, witness packing or machine dispatch.

use super::*;

/// Caller-bound Native operands. Count and every row must be well-formed in
/// every branch. The raw index is consumed only through the effective index.
/// The dispatch owner must derive enable from the actual constrained pre-state
/// selector; this fragment cannot establish that provenance.
#[derive(Clone, Debug, PartialEq)]
pub struct EnabledFetchInputs {
    pub enable: Operand,
    pub active_rows: Operand,
    pub index: Operand,
    pub rows: Vec<Vec<Operand>>,
}

/// A selected construction value paired with the exact caller enable operand.
/// Disabled-zero is not a fetched/default/padding row. Consumers must gate every
/// write or assertion using this value with enable, e.g. select(enable,new,old).
#[derive(Clone, Debug, PartialEq)]
pub struct EnabledTypedOperand {
    pub kind: Type,
    pub value: Operand,
    pub enable: Operand,
}

/// Fragment only: no public binding, Output terminator, success flag or after-state.
pub struct EnabledFetchBlock {
    pub instructions: Vec<Instruction>,
    pub selected: Vec<EnabledTypedOperand>,
    pub preserved: EnabledFetchInputs,
}

struct Builder {
    scope: String,
    serial: usize,
    instructions: Vec<Instruction>,
}

impl Builder {
    fn fresh(&mut self) -> Identifier {
        let id = Identifier(format!("%{}.temp.{}", self.scope, self.serial));
        self.serial += 1;
        id
    }

    fn select(&mut self, bit: Operand, a: Operand, b: Operand) -> Operand {
        let output = self.fresh();
        self.instructions.push(Instruction::CondSelect {
            bit,
            a,
            b,
            output: output.clone(),
        });
        Operand::Variable(output)
    }

    fn less_than(&mut self, a: Operand, b: Operand) -> Operand {
        let output = self.fresh();
        self.instructions.push(Instruction::LessThan {
            a,
            b,
            bits: 128,
            output: output.clone(),
        });
        let result = Operand::Variable(output);
        constrain(&mut self.instructions, result.clone(), Type::Bool);
        result
    }
}

/// Emit a separate enabled fragment; the unconditional G2 constructor is unchanged.
///
/// Inputs must be bound before the fragment. All Variables, including enable,
/// must start with `%` and must be outside this block's `%{scope}.temp.` prefix.
/// Scope is inserted literally after `%`, normally without its own `%` prefix.
/// The composer reserves distinct non-nested namespaces across enabled and
/// unconditional blocks and enclosing instructions. No global registry is used.
///
/// Enable must come from constrained dispatch, never a free prover witness or
/// expected data. Rows/count must be valid unconditionally; branch-specific
/// temporaries require that same unconditional validity from the caller.
pub fn emit_enabled_fetch_block(
    shape: &TableShape,
    inputs: &EnabledFetchInputs,
    scope: &str,
) -> Result<EnabledFetchBlock, ConstructionError> {
    shape.check_rows(&inputs.rows)?;
    if scope.is_empty() {
        return Err(ConstructionError::EmptyScope);
    }
    // Exactly 4 + N*(C+3) fresh identifiers: effective index, two comparisons,
    // guarded assertion; N equality/gated-selector/sum triples; N*C lane selects.
    // This checks host arithmetic only; it admits no concrete resource bound.
    shape
        .capacity
        .checked_mul(
            shape
                .columns
                .len()
                .checked_add(3)
                .ok_or(ConstructionError::SizeOverflow)?,
        )
        .and_then(|n| n.checked_add(4))
        .ok_or(ConstructionError::SizeOverflow)?;
    let namespace = format!("%{scope}.temp.");
    for operand in [&inputs.enable, &inputs.active_rows, &inputs.index]
        .into_iter()
        .chain(inputs.rows.iter().flatten())
    {
        if let Operand::Variable(id) = operand {
            if !id.0.starts_with('%') {
                return Err(ConstructionError::UnprefixedVariable);
            }
            if id.0.starts_with(&namespace) {
                return Err(ConstructionError::TemporaryNameCollision);
            }
        }
    }
    let mut b = Builder {
        scope: scope.into(),
        serial: 0,
        instructions: Vec::new(),
    };
    constrain(&mut b.instructions, inputs.enable.clone(), Type::Bool);
    constrain(&mut b.instructions, inputs.active_rows.clone(), Type::UInt);
    for row in &inputs.rows {
        for (cell, column) in row.iter().zip(&shape.columns) {
            constrain(&mut b.instructions, cell.clone(), column.kind);
        }
    }
    let index_eff = b.select(inputs.enable.clone(), inputs.index.clone(), immediate(0));
    constrain(&mut b.instructions, index_eff.clone(), Type::UInt);
    let exceeds_capacity = b.less_than(
        immediate(shape.capacity as u128),
        inputs.active_rows.clone(),
    );
    b.instructions.push(Instruction::ConstrainEq {
        a: exceeds_capacity,
        b: immediate(0),
    });
    let in_prefix = b.less_than(index_eff.clone(), inputs.active_rows.clone());
    let guarded_prefix = b.select(inputs.enable.clone(), in_prefix, immediate(1));
    b.instructions.push(Instruction::Assert {
        cond: guarded_prefix,
    });

    let mut selectors = Vec::with_capacity(shape.capacity);
    let mut sum = immediate(0);
    for row in 0..shape.capacity {
        let output = b.fresh();
        b.instructions.push(Instruction::TestEq {
            a: index_eff.clone(),
            b: immediate(row as u128),
            output: output.clone(),
        });
        let eq = Operand::Variable(output);
        constrain(&mut b.instructions, eq.clone(), Type::Bool);
        let selector = b.select(inputs.enable.clone(), eq, immediate(0));
        constrain(&mut b.instructions, selector.clone(), Type::Bool);
        let sum_output = b.fresh();
        b.instructions.push(Instruction::Add {
            a: sum,
            b: selector.clone(),
            output: sum_output.clone(),
        });
        sum = Operand::Variable(sum_output);
        selectors.push(selector);
    }
    b.instructions.push(Instruction::ConstrainEq {
        a: sum,
        b: inputs.enable.clone(),
    });

    let mut selected = Vec::with_capacity(shape.columns.len());
    for (column, col) in shape.columns.iter().enumerate() {
        let mut value = immediate(0);
        for (row, selector) in selectors.iter().enumerate() {
            value = b.select(selector.clone(), inputs.rows[row][column].clone(), value);
        }
        constrain(&mut b.instructions, value.clone(), col.kind);
        selected.push(EnabledTypedOperand {
            kind: col.kind,
            value,
            enable: inputs.enable.clone(),
        });
    }
    Ok(EnabledFetchBlock {
        instructions: b.instructions,
        selected,
        preserved: inputs.clone(),
    })
}
