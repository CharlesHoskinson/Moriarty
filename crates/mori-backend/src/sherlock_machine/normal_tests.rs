//! Normal construction tests only. No SDK load, check, evaluation or proving.

use super::*;
use mori_core::UInt;

fn shape() -> TableShape {
    TableShape::new(
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
    .unwrap()
}

// Supplied inputs from the independent problem statement, not expected outputs.
fn witness(index: u128) -> FetchWitness {
    FetchWitness {
        active_rows: 3,
        index,
        rows: vec![
            vec![Value::UInt(UInt(7)), Value::Bool(true)],
            vec![Value::UInt(UInt(9)), Value::Bool(false)],
            vec![Value::UInt(UInt(12)), Value::Bool(true)],
        ],
    }
}

#[test]
fn supplied_normal_inputs_have_one_fixed_construction_and_complete_order() {
    let emitted = emit_public_fetch(&shape()).unwrap();
    assert_eq!(emitted.input_fields.len(), 8);
    assert_eq!(emitted.output_fields.len(), 10);
    assert_eq!(emitted.input_fields[0].role, FieldRole::ActiveRows);
    assert_eq!(emitted.input_fields[1].role, FieldRole::Index);
    for index in 0..3 {
        let supplied = emitted.pack_witness(&witness(index)).unwrap();
        assert_eq!(
            supplied,
            [3, index, 7, 1, 9, 0, 12, 1]
                .into_iter()
                .map(Fr::from)
                .collect::<Vec<_>>()
        );
    }
    // Construction does not take a witness and cannot bake these values in.
    assert_eq!(emitted.ir, emit_public_fetch(&shape()).unwrap().ir);
    let identifiers: Vec<_> = emitted
        .ir
        .instructions
        .iter()
        .filter_map(|instruction| match instruction {
            Instruction::PublicInput { output, .. }
            | Instruction::LessThan { output, .. }
            | Instruction::TestEq { output, .. }
            | Instruction::Add { output, .. }
            | Instruction::CondSelect { output, .. } => Some(output.0.as_str()),
            _ => None,
        })
        .collect();
    let expected_names: Vec<_> = (0..8)
        .map(|n| format!("%row_fetch.input.{n}"))
        .chain((0..14).map(|n| format!("%row_fetch.temp.{n}")))
        .collect();
    assert_eq!(
        identifiers,
        expected_names
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    );
}

#[test]
fn selection_uses_all_supplied_cells_and_preservation_returns_the_same_operands() {
    let emitted = emit_public_fetch(&shape()).unwrap();
    let source = emitted.ir.instructions.as_ref();
    let inputs: Vec<_> = source
        .iter()
        .filter_map(|i| match i {
            Instruction::PublicInput { output, .. } => Some(Operand::Variable(output.clone())),
            _ => None,
        })
        .collect();
    let outputs = match source.last().unwrap() {
        Instruction::Output { vals } => vals,
        _ => panic!("expected Output terminator"),
    };
    // Two selected cells, followed by unchanged active_rows, index and all rows.
    assert_eq!(&outputs[2..], inputs.as_slice());
    let selectors: Vec<_> = source
        .iter()
        .filter_map(|i| match i {
            Instruction::TestEq { a, b, output } => Some((a, b, output)),
            _ => None,
        })
        .collect();
    let sums: Vec<_> = source
        .iter()
        .filter_map(|i| match i {
            Instruction::Add { a, b, output } => Some((a, b, output)),
            _ => None,
        })
        .collect();
    assert_eq!(selectors.len(), 3);
    assert_eq!(sums.len(), 3);
    let mut previous_sum = immediate(0);
    for row in 0..3 {
        assert_eq!(selectors[row].0, &inputs[1]);
        assert_eq!(selectors[row].1, &immediate(row as u128));
        assert_eq!(sums[row].0, &previous_sum);
        assert_eq!(sums[row].1, &Operand::Variable(selectors[row].2.clone()));
        previous_sum = Operand::Variable(sums[row].2.clone());
    }
    assert!(
        source
            .iter()
            .any(|i| matches!(i, Instruction::ConstrainEq { a, b }
        if a == &previous_sum && b == &immediate(1)))
    );
    let selects: Vec<_> = source
        .iter()
        .filter_map(|i| match i {
            Instruction::CondSelect { bit, a, b, output } => Some((bit, a, b, output)),
            _ => None,
        })
        .collect();
    assert_eq!(selects.len(), 6);
    for column in 0..2 {
        let mut accumulator = immediate(0);
        for row in 0..3 {
            let (bit, a, b, output) = selects[column * 3 + row];
            assert_eq!(bit, &Operand::Variable(selectors[row].2.clone()));
            assert_eq!(a, &inputs[2 + row * 2 + column]);
            assert_eq!(b, &accumulator);
            accumulator = Operand::Variable(output.clone());
        }
        assert_eq!(outputs[column], accumulator);
    }
    assert_eq!(
        source
            .iter()
            .filter(|i| matches!(i, Instruction::LessThan { bits: 128, .. }))
            .count(),
        2
    );
    assert_eq!(
        source
            .iter()
            .filter(|i| matches!(i, Instruction::TestEq { .. }))
            .count(),
        3
    );
    assert_eq!(
        source
            .iter()
            .filter(|i| matches!(i, Instruction::Impact { .. }))
            .count(),
        8
    );
    let comparisons: Vec<_> = source
        .iter()
        .filter_map(|i| match i {
            Instruction::LessThan { a, b, output, .. } => Some((a, b, output)),
            _ => None,
        })
        .collect();
    assert_eq!(comparisons[0].0, &immediate(3));
    assert_eq!(comparisons[0].1, &inputs[0]);
    assert!(
        source
            .iter()
            .any(|i| matches!(i, Instruction::ConstrainEq { a, b }
        if a == &Operand::Variable(comparisons[0].2.clone()) && b == &immediate(0)))
    );
    assert_eq!(comparisons[1].0, &inputs[1]);
    assert_eq!(comparisons[1].1, &inputs[0]);
    assert!(
        source
            .iter()
            .any(|i| matches!(i, Instruction::Assert { cond }
        if cond == &Operand::Variable(comparisons[1].2.clone())))
    );
}

#[test]
fn embedded_fragment_accepts_another_ordered_shape_and_preserves_all_bindings() {
    let shape = TableShape::new(
        2,
        vec![
            Column {
                name: "ready".into(),
                kind: Type::Bool,
            },
            Column {
                name: "left".into(),
                kind: Type::UInt,
            },
            Column {
                name: "right".into(),
                kind: Type::UInt,
            },
        ],
    )
    .unwrap();
    let variable = |name: &str| Operand::Variable(Identifier(name.into()));
    let supplied = FetchInputs {
        active_rows: variable("%caller.count"),
        index: variable("%caller.index"),
        rows: vec![
            vec![
                variable("%a.ready"),
                variable("%a.left"),
                variable("%a.right"),
            ],
            vec![
                variable("%b.ready"),
                variable("%b.left"),
                variable("%b.right"),
            ],
        ],
    };
    let block = emit_fetch_block(&shape, &supplied, "embedded_fetch").unwrap();
    assert_eq!(block.preserved, supplied);
    assert_eq!(
        block
            .selected
            .iter()
            .map(|cell| cell.kind)
            .collect::<Vec<_>>(),
        vec![Type::Bool, Type::UInt, Type::UInt]
    );
    assert!(!block.instructions.iter().any(|i| matches!(
        i,
        Instruction::PublicInput { .. } | Instruction::Output { .. }
    )));
}

#[test]
fn frozen_normal_expectations_are_kept_for_future_actual_comparison() {
    // This verifies fixture provenance/shape only. No expected value is supplied
    // to the emitter, and this is not an actual selected-output comparison.
    let expected: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../models/sherlock/machine-circuit/expected.json"
    ))
    .unwrap();
    assert_eq!(expected["cases"].as_array().unwrap().len(), 3);
    assert_eq!(
        expected["cases"][0]["selected"],
        serde_json::json!(["7", true])
    );
    assert_eq!(
        expected["cases"][1]["selected"],
        serde_json::json!(["9", false])
    );
    assert_eq!(
        expected["cases"][2]["selected"],
        serde_json::json!(["12", true])
    );
    assert_eq!(expected["rows"], expected["preserved_input_rows"]);
}

// Nonproving public-source capture is a child of this existing test-only module.
// Keep all production relation/context source-closure members unchanged.
#[path = "literal_complete_public_source_capture_normal_tests.rs"]
mod literal_complete_public_source_capture_normal_tests;
