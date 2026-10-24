//! UNRUN normal symbolic instruction-wiring tests. No SDK consumer is called.
//! Specimen dimensions are construction data, not resource admission.
use super::literal_step::*;
use super::*;

fn var(name: &str) -> Operand {
    Operand::Variable(Identifier(name.into()))
}
fn specimen() -> (LiteralSchema, LiteralStepInputs) {
    let dimensions = LiteralCapacities {
        program: [2, 3, 2, 2, 2, 3, 1, 2, 1, 1, 4, 4, 5, 2, 2],
        state: [2, 2, 2, 2, 2, 2],
        stack_limit: 5,
    };
    let schema = literal_schema(&dimensions).unwrap();
    let row = |prefix: &str, lanes: &[LiteralLane]| {
        lanes
            .iter()
            .map(|lane| var(&format!("%input.{prefix}.{}", lane.name)))
            .collect::<Vec<_>>()
    };
    let tables = |prefix: &str, shapes: &[LiteralTable]| {
        shapes
            .iter()
            .map(|table| {
                (0..table.capacity)
                    .map(|i| row(&format!("{prefix}.{}.{i}", table.name), &table.lanes))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    };
    let inputs = LiteralStepInputs {
        enable: var("%dispatch.a102"),
        program: LiteralImage {
            root: row("program.header", &schema.program_header),
            tables: tables("program", &schema.program_tables),
        },
        pre: LiteralImage {
            root: row("pre.root", &schema.state_root),
            tables: tables("pre", &schema.state_tables),
        },
        post: LiteralImage {
            root: row("post.root", &schema.state_root),
            tables: tables("post", &schema.state_tables),
        },
        envelope: LiteralEnvelope {
            rule_tag: var("%event.rule_tag"),
            charge: var("%event.charge"),
        },
    };
    (schema, inputs)
}
fn cell(lanes: &[LiteralLane], row: &[Operand], name: &str) -> Operand {
    row[lanes.iter().position(|lane| lane.name == name).unwrap()].clone()
}
fn guarded_eq(code: &[Instruction], enable: &Operand, left: &Operand, right: &Operand) -> bool {
    code.iter().any(|i| match i {
        Instruction::CondSelect { bit, a, b, output }
            if bit == enable && a == left && b == right =>
        {
            code.iter().any(|j| {
                matches!(j, Instruction::ConstrainEq { a, b }
                if a == &Operand::Variable(output.clone()) && b == right)
            })
        }
        _ => false,
    })
}

#[test]
fn complete_l4_column_membership_and_both_fuel_arrays_are_retained() {
    let (schema, _) = specimen();
    assert_eq!(schema.program_header.len(), 16);
    assert_eq!(
        schema
            .program_tables
            .iter()
            .map(|t| t.lanes.len())
            .collect::<Vec<_>>(),
        vec![2, 1, 4, 1, 11, 1, 6, 2, 3, 2, 3, 4, 1, 22, 2]
    );
    assert_eq!(schema.state_root.len(), 45);
    assert_eq!(
        schema
            .state_tables
            .iter()
            .map(|t| t.lanes.len())
            .collect::<Vec<_>>(),
        vec![22, 2, 5, 2, 17, 36]
    );
    for prefix in ["remainingFuel", "initialFuel"] {
        for byte in 0..16 {
            let lane = schema
                .state_root
                .iter()
                .find(|l| l.name == format!("{prefix}.{byte}"))
                .unwrap();
            assert_eq!(lane.bits, 8);
        }
    }
}

#[test]
fn real_enabled_fetch_instructions_feed_the_exact_return_reference() {
    let (schema, inputs) = specimen();
    let block = emit_literal_step(&schema, &inputs, "literal").unwrap();
    let expression = &schema.program_tables[4];
    let fetch_shape = TableShape::new(
        expression.capacity,
        expression
            .lanes
            .iter()
            .map(|l| Column {
                name: l.name.clone(),
                kind: if l.boolean { Type::Bool } else { Type::UInt },
            })
            .collect(),
    )
    .unwrap();
    let fetch = emit_enabled_fetch_block(
        &fetch_shape,
        &EnabledFetchInputs {
            enable: inputs.enable.clone(),
            active_rows: inputs.program.root[5].clone(),
            index: cell(
                &schema.state_tables[5].lanes,
                &inputs.pre.tables[5][0],
                "exprId",
            ),
            rows: inputs.program.tables[4].clone(),
        },
        "literal.expression_fetch",
    )
    .unwrap();
    assert!(
        block
            .instructions
            .windows(fetch.instructions.len())
            .any(|w| w == fetch.instructions.as_slice())
    );
    let selected: Vec<_> = fetch.selected.iter().map(|v| v.value.clone()).collect();
    assert_eq!(block.selected_expression, selected);
    for name in ["valueRef.space", "valueRef.index"] {
        let expected = cell(&expression.lanes, &selected, name);
        let actual = cell(
            &schema.state_tables[5].lanes,
            &inputs.post.tables[5][0],
            name,
        );
        assert!(guarded_eq(
            &block.instructions,
            &inputs.enable,
            &actual,
            &expected
        ));
    }
    assert_eq!(block.preserved, inputs);
}

#[test]
fn every_unchanged_root_and_arena_cell_has_a_post_to_pre_equation() {
    let (schema, inputs) = specimen();
    let block = emit_literal_step(&schema, &inputs, "preservation").unwrap();
    for (i, lane) in schema.state_root.iter().enumerate() {
        if lane.name == "controlLength" || lane.name.starts_with("remainingFuel.") {
            continue;
        }
        assert!(guarded_eq(
            &block.instructions,
            &inputs.enable,
            &inputs.post.root[i],
            &inputs.pre.root[i]
        ));
    }
    for table in 0..5 {
        for row in 0..schema.state_tables[table].capacity {
            for column in 0..schema.state_tables[table].lanes.len() {
                assert!(guarded_eq(
                    &block.instructions,
                    &inputs.enable,
                    &inputs.post.tables[table][row][column],
                    &inputs.pre.tables[table][row][column]
                ));
            }
        }
    }
}

#[test]
fn return_padding_and_wire_envelope_have_explicit_guarded_equations() {
    let (schema, inputs) = specimen();
    let block = emit_literal_step(&schema, &inputs, "return").unwrap();
    assert!(guarded_eq(
        &block.instructions,
        &inputs.enable,
        &inputs.envelope.rule_tag,
        &immediate(3)
    ));
    assert!(guarded_eq(
        &block.instructions,
        &inputs.enable,
        &inputs.envelope.charge,
        &immediate(1)
    ));
    for row in 0..schema.state_tables[5].capacity {
        for (column, lane) in schema.state_tables[5].lanes.iter().enumerate() {
            if row == 0 && (lane.name == "valueRef.space" || lane.name == "valueRef.index") {
                continue;
            }
            let value = if row == 0 && lane.name == "tag" { 1 } else { 0 };
            assert!(guarded_eq(
                &block.instructions,
                &inputs.enable,
                &inputs.post.tables[5][row][column],
                &immediate(value)
            ));
        }
    }
    assert!(!block.instructions.iter().any(|i| matches!(
        i,
        Instruction::PublicInput { .. } | Instruction::Output { .. }
    )));
}

#[test]
fn decrement_wiring_propagates_a_boolean_borrow_through_all_sixteen_bytes() {
    let (schema, inputs) = specimen();
    let block = emit_literal_step(&schema, &inputs, "fuel").unwrap();
    let code = &block.instructions;
    let mut borrow = inputs.enable.clone();
    for byte in 0..16 {
        let pre = cell(
            &schema.state_root,
            &inputs.pre.root,
            &format!("remainingFuel.{byte}"),
        );
        let post = cell(
            &schema.state_root,
            &inputs.post.root,
            &format!("remainingFuel.{byte}"),
        );
        for lane in [&pre, &post] {
            assert!(
                code.iter().any(
                    |i| matches!(i, Instruction::ConstrainBits { val, bits: 8 } if val == lane)
                )
            );
        }
        let is_zero = code
            .iter()
            .find_map(|i| match i {
                Instruction::TestEq { a, b, output } if a == &pre && b == &immediate(0) => {
                    Some(Operand::Variable(output.clone()))
                }
                _ => None,
            })
            .unwrap();
        let next = code
            .iter()
            .find_map(|i| match i {
                Instruction::CondSelect { bit, a, b, output }
                    if bit == &borrow && a == &is_zero && b == &immediate(0) =>
                {
                    Some(Operand::Variable(output.clone()))
                }
                _ => None,
            })
            .unwrap();
        assert!(
            code.iter()
                .any(|i| matches!(i, Instruction::ConstrainToBoolean { val } if val == &next))
        );
        let carry = code
            .iter()
            .find_map(|i| match i {
                Instruction::Mul { a, b, output } if a == &immediate(256) && b == &next => {
                    Some(Operand::Variable(output.clone()))
                }
                _ => None,
            })
            .unwrap();
        let left = code
            .iter()
            .find_map(|i| match i {
                Instruction::Add { a, b, output } if a == &pre && b == &carry => {
                    Some(Operand::Variable(output.clone()))
                }
                _ => None,
            })
            .unwrap();
        let right = code
            .iter()
            .find_map(|i| match i {
                Instruction::Add { a, b, output } if a == &post && b == &borrow => {
                    Some(Operand::Variable(output.clone()))
                }
                _ => None,
            })
            .unwrap();
        assert!(guarded_eq(code, &inputs.enable, &left, &right));
        borrow = next;
    }
    assert!(code.iter().any(
        |i| matches!(i, Instruction::ConstrainEq { a, b } if a == &borrow && b == &immediate(0))
    ));
}
