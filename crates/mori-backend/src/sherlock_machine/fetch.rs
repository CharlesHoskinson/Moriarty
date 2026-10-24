use super::*;

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

/// Emit reusable typed fetch over caller-supplied Native operands.
///
/// Variable inputs must already start with `%`, as the pinned SDK's serialized
/// operand format requires. `scope` is inserted literally after the generated
/// `%`; the caller normally supplies a name without that prefix.
/// Reserve the entire final `%{scope}.temp.` namespace for this one block and
/// bind every input before appending the fragment. The composer must allocate
/// distinct, nonoverlapping final namespaces: neither reserved prefix may be a
/// prefix of another (for example, scopes `a` and `a.temp.1` overlap).
/// Input collisions are rejected here; collisions with unrelated enclosing
/// instructions or another fragment remain the composing caller's obligation.
/// No witness value or expected result is an argument to this constructor.
pub fn emit_fetch_block(
    shape: &TableShape,
    inputs: &FetchInputs,
    scope: &str,
) -> Result<FetchBlock, ConstructionError> {
    shape.check_rows(&inputs.rows)?;
    if scope.is_empty() {
        return Err(ConstructionError::EmptyScope);
    }
    // Includes all generated selector, sum, comparison and cell temporaries.
    shape
        .capacity
        .checked_mul(
            shape
                .columns
                .len()
                .checked_add(2)
                .ok_or(ConstructionError::SizeOverflow)?,
        )
        .and_then(|n| n.checked_add(2))
        .ok_or(ConstructionError::SizeOverflow)?;
    let namespace = format!("%{scope}.temp.");
    for operand in [&inputs.active_rows, &inputs.index]
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
    constrain(&mut b.instructions, inputs.active_rows.clone(), Type::UInt);
    constrain(&mut b.instructions, inputs.index.clone(), Type::UInt);
    for row in &inputs.rows {
        for (cell, column) in row.iter().zip(&shape.columns) {
            constrain(&mut b.instructions, cell.clone(), column.kind);
        }
    }
    // Both operands of each comparison are in the UInt128 domain. The first
    // constrains the active prefix; the second requires an index in that prefix.
    let exceeds_capacity = b.less_than(
        immediate(shape.capacity as u128),
        inputs.active_rows.clone(),
    );
    b.instructions.push(Instruction::ConstrainEq {
        a: exceeds_capacity,
        b: immediate(0),
    });
    let in_prefix = b.less_than(inputs.index.clone(), inputs.active_rows.clone());
    b.instructions.push(Instruction::Assert { cond: in_prefix });

    let mut selectors = Vec::with_capacity(shape.capacity);
    let mut sum = immediate(0);
    for row in 0..shape.capacity {
        let output = b.fresh();
        b.instructions.push(Instruction::TestEq {
            a: inputs.index.clone(),
            b: immediate(row as u128),
            output: output.clone(),
        });
        let selector = Operand::Variable(output);
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
        b: immediate(1),
    });

    let mut selected = Vec::with_capacity(shape.columns.len());
    for (column, col) in shape.columns.iter().enumerate() {
        let mut value = immediate(0);
        for (row, selector) in selectors.iter().enumerate() {
            let output = b.fresh();
            b.instructions.push(Instruction::CondSelect {
                bit: selector.clone(),
                a: inputs.rows[row][column].clone(),
                b: value,
                output: output.clone(),
            });
            value = Operand::Variable(output);
        }
        constrain(&mut b.instructions, value.clone(), col.kind);
        selected.push(TypedOperand {
            kind: col.kind,
            value,
        });
    }
    Ok(FetchBlock {
        instructions: b.instructions,
        selected,
        preserved: inputs.clone(),
    })
}
