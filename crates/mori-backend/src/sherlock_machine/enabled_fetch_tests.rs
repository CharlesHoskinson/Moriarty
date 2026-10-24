//! UNRUN constructor wiring tests prescribed by the Opus enabled-fetch owner.
//! No SDK load, check, evaluation, unsatisfiability or proof operation is called.
//! These tests inspect instruction edges and aliases, not actual selected values.

use super::*;

fn variable(name: &str) -> Operand {
    Operand::Variable(Identifier(name.into()))
}
fn temp(n: usize) -> Operand {
    variable(&format!("%enabled_fetch.temp.{n}"))
}

fn supplied() -> (TableShape, EnabledFetchInputs) {
    let shape = TableShape::new(
        3,
        vec![
            Column {
                name: "amount".into(),
                kind: Type::UInt,
            },
            Column {
                name: "enabled".into(),
                kind: Type::Bool,
            },
        ],
    )
    .unwrap();
    let inputs = EnabledFetchInputs {
        enable: variable("%dispatch.selected"),
        active_rows: variable("%state.active_rows"),
        index: variable("%branch.raw_index"),
        rows: (0..3)
            .map(|row| {
                vec![
                    variable(&format!("%state.row.{row}.amount")),
                    variable(&format!("%state.row.{row}.enabled")),
                ]
            })
            .collect(),
    };
    (shape, inputs)
}

fn has_kind(instructions: &[Instruction], operand: &Operand, kind: Type) -> bool {
    instructions
        .iter()
        .any(|instruction| match (instruction, kind) {
            (Instruction::ConstrainBits { val, bits: 128 }, Type::UInt) => val == operand,
            (Instruction::ConstrainToBoolean { val }, Type::Bool) => val == operand,
            _ => false,
        })
}

#[test]
fn owner_wiring_uses_effective_index_guarded_prefix_and_gated_selectors() {
    let (shape, inputs) = supplied();
    let block = emit_enabled_fetch_block(&shape, &inputs, "enabled_fetch").unwrap();
    let source = &block.instructions;
    let selects: Vec<_> = source
        .iter()
        .filter_map(|i| match i {
            Instruction::CondSelect { bit, a, b, output } => Some((
                bit.clone(),
                a.clone(),
                b.clone(),
                Operand::Variable(output.clone()),
            )),
            _ => None,
        })
        .collect();
    assert_eq!(selects.len(), 2 + 3 + 3 * 2);
    assert_eq!(
        selects[0],
        (
            inputs.enable.clone(),
            inputs.index.clone(),
            immediate(0),
            temp(0)
        )
    );
    assert_eq!(
        selects[1],
        (inputs.enable.clone(), temp(2), immediate(1), temp(3))
    );
    assert!(
        source
            .iter()
            .any(|i| matches!(i, Instruction::Assert { cond } if cond == &temp(3)))
    );
    assert_eq!(
        source
            .iter()
            .filter(|i| matches!(i, Instruction::Assert { .. }))
            .count(),
        1
    );
    let comparisons: Vec<_> = source
        .iter()
        .filter_map(|i| match i {
            Instruction::LessThan { a, b, bits, output } => Some((
                a.clone(),
                b.clone(),
                *bits,
                Operand::Variable(output.clone()),
            )),
            _ => None,
        })
        .collect();
    assert_eq!(
        comparisons,
        vec![
            (immediate(3), inputs.active_rows.clone(), 128, temp(1)),
            (temp(0), inputs.active_rows.clone(), 128, temp(2)),
        ]
    );
    assert!(
        source
            .iter()
            .any(|i| matches!(i, Instruction::ConstrainEq { a, b }
        if a == &temp(1) && b == &immediate(0)))
    );
    assert!(has_kind(source, &temp(1), Type::Bool));
    assert!(has_kind(source, &temp(2), Type::Bool));
    let equalities: Vec<_> = source
        .iter()
        .filter_map(|i| match i {
            Instruction::TestEq { a, b, output } => {
                Some((a.clone(), b.clone(), Operand::Variable(output.clone())))
            }
            _ => None,
        })
        .collect();
    let sums: Vec<_> = source
        .iter()
        .filter_map(|i| match i {
            Instruction::Add { a, b, output } => {
                Some((a.clone(), b.clone(), Operand::Variable(output.clone())))
            }
            _ => None,
        })
        .collect();
    assert_eq!(equalities.len(), 3);
    assert_eq!(sums.len(), 3);
    let mut previous_sum = immediate(0);
    for row in 0..3 {
        let eq = temp(4 + 3 * row);
        let selector = temp(5 + 3 * row);
        let sum = temp(6 + 3 * row);
        assert_eq!(
            equalities[row],
            (temp(0), immediate(row as u128), eq.clone())
        );
        assert_eq!(
            selects[2 + row],
            (
                inputs.enable.clone(),
                eq.clone(),
                immediate(0),
                selector.clone()
            )
        );
        assert!(has_kind(source, &eq, Type::Bool));
        assert!(has_kind(source, &selector, Type::Bool));
        assert_eq!(sums[row], (previous_sum, selector, sum.clone()));
        previous_sum = sum;
    }
    assert!(
        source
            .iter()
            .any(|i| matches!(i, Instruction::ConstrainEq { a, b }
        if a == &previous_sum && b == &inputs.enable))
    );
    assert_eq!(
        source
            .iter()
            .filter(|i| matches!(i, Instruction::ConstrainEq { .. }))
            .count(),
        2
    );
    for column in 0..2 {
        let mut previous_value = immediate(0);
        for row in 0..3 {
            let output = temp(13 + 3 * column + row);
            assert_eq!(
                selects[5 + 3 * column + row],
                (
                    temp(5 + 3 * row),
                    inputs.rows[row][column].clone(),
                    previous_value,
                    output.clone(),
                )
            );
            previous_value = output;
        }
        assert_eq!(block.selected[column].value, previous_value);
        assert_eq!(block.selected[column].kind, shape.columns()[column].kind);
        assert_eq!(block.selected[column].enable, inputs.enable);
        assert!(has_kind(
            source,
            &previous_value,
            shape.columns()[column].kind
        ));
    }
    assert_eq!(block.preserved, inputs);
    let identifiers: Vec<_> = source
        .iter()
        .filter_map(|i| match i {
            Instruction::LessThan { output, .. }
            | Instruction::TestEq { output, .. }
            | Instruction::Add { output, .. }
            | Instruction::CondSelect { output, .. } => Some(output.0.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(
        identifiers,
        (0..19)
            .map(|n| format!("%enabled_fetch.temp.{n}"))
            .collect::<Vec<_>>()
    );
    assert_eq!(source.len(), 12 + 2 * 3 * 2 + 5 * 3 + 2);
    assert!(!source.iter().any(|i| matches!(
        i,
        Instruction::PublicInput { .. } | Instruction::Output { .. }
    )));
}

#[test]
fn disabled_construction_keeps_unconditional_checks_and_raw_index_alias() {
    let (shape, mut inputs) = supplied();
    inputs.enable = immediate(0);
    inputs.active_rows = immediate(0);
    // This variable may be bound to the fixture's 2^128 by a future admitted
    // consumer. The constructor neither packs nor narrows that supplied value.
    inputs.index = variable("%caller.raw_out_of_uint128_index");
    let block = emit_enabled_fetch_block(&shape, &inputs, "enabled_fetch").unwrap();
    let source = &block.instructions;
    assert!(has_kind(source, &inputs.enable, Type::Bool));
    assert!(has_kind(source, &inputs.active_rows, Type::UInt));
    assert!(has_kind(source, &temp(0), Type::UInt));
    assert!(
        !source
            .iter()
            .any(|i| matches!(i, Instruction::ConstrainBits { val, .. }
        | Instruction::ConstrainToBoolean { val } if val == &inputs.index))
    );
    for row in &inputs.rows {
        for (cell, column) in row.iter().zip(shape.columns()) {
            assert!(has_kind(source, cell, column.kind));
        }
    }
    assert!(source.iter().any(
        |i| matches!(i, Instruction::LessThan { a, b, bits: 128, .. }
        if a == &immediate(3) && b == &inputs.active_rows)
    ));
    assert!(
        source
            .iter()
            .any(|i| matches!(i, Instruction::ConstrainEq { a, b }
        if a == &temp(1) && b == &immediate(0)))
    );
    assert_eq!(block.preserved, inputs);
    assert!(
        block
            .selected
            .iter()
            .all(|cell| cell.enable == inputs.enable)
    );
    assert_eq!(source.len(), 41);
}

#[test]
fn owner_rejects_unprefixed_inputs_including_enable() {
    let (shape, supplied) = supplied();
    for position in 0..4 {
        let mut inputs = supplied.clone();
        let operand = match position {
            0 => &mut inputs.enable,
            1 => &mut inputs.active_rows,
            2 => &mut inputs.index,
            _ => &mut inputs.rows[0][0],
        };
        *operand = variable("caller.unprefixed");
        assert!(matches!(
            emit_enabled_fetch_block(&shape, &inputs, "enabled_fetch"),
            Err(ConstructionError::UnprefixedVariable)
        ));
    }
}

#[test]
fn owner_rejects_input_collisions_including_enable() {
    let (shape, supplied) = supplied();
    for position in 0..4 {
        let mut inputs = supplied.clone();
        let operand = match position {
            0 => &mut inputs.enable,
            1 => &mut inputs.active_rows,
            2 => &mut inputs.index,
            _ => &mut inputs.rows[0][0],
        };
        *operand = variable("%enabled_fetch.temp.0");
        assert!(matches!(
            emit_enabled_fetch_block(&shape, &inputs, "enabled_fetch"),
            Err(ConstructionError::TemporaryNameCollision)
        ));
    }
}

#[test]
fn independent_fixture_remains_expected_only_with_raw_index_text_intact() {
    // No fixture field is passed to the emitter. These are metadata assertions,
    // not comparisons of actual outputs and not SDK satisfiability tests.
    let expected: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../models/sherlock/enabled-fetch/expected.json"
    ))
    .unwrap();
    assert_eq!(expected["actual"], "UNRUN");
    assert_eq!(expected["cases"].as_array().unwrap().len(), 4);
    assert_eq!(
        expected["cases"][2]["raw_index"],
        "340282366920938463463374607431768211456"
    );
    assert_eq!(
        expected["cases"][2]["expected_selected"],
        serde_json::json!(["0", false])
    );
    assert_eq!(
        expected["cases"][3]["expected_selected"],
        serde_json::json!(["0", false])
    );
}
