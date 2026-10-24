//! Symbolic source-only tests for `return_publication_step`.
//! Inspects emitted ZKIR instructions over normative `LiteralSchema` symbolic
//! operands without SDK execution, witness evaluation, or host-oracle cell values.

use super::return_publication_step::{
    RETURN_PUBLICATION_RULE_CHARGE, RETURN_PUBLICATION_RULE_ID, RETURN_PUBLICATION_RULE_START_LINE,
    RETURN_PUBLICATION_RULE_TAG, ReturnPublicationStepInputs, emit_return_publication_step,
    is_reserved_return_publication_variable,
};
use super::*;

fn sample_capacities() -> LiteralCapacities {
    LiteralCapacities {
        program: [1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 4, 4, 5, 2, 1],
        state: [2, 1, 1, 1, 2, 2],
        stack_limit: 4,
    }
}

fn symbolic_image(prefix: &str, root: &[LiteralLane], tables: &[LiteralTable]) -> LiteralImage {
    let root_ops = root
        .iter()
        .map(|lane| Operand::Variable(Identifier(format!("%{prefix}.root.{}", lane.name))))
        .collect();
    let table_ops = tables
        .iter()
        .map(|table| {
            (0..table.capacity)
                .map(|row| {
                    table
                        .lanes
                        .iter()
                        .map(|lane| {
                            Operand::Variable(Identifier(format!(
                                "%{prefix}.{}.{row}.{}",
                                table.name, lane.name
                            )))
                        })
                        .collect()
                })
                .collect()
        })
        .collect();
    LiteralImage {
        root: root_ops,
        tables: table_ops,
    }
}

fn symbolic_inputs(schema: &LiteralSchema, prefix: &str) -> ReturnPublicationStepInputs {
    ReturnPublicationStepInputs {
        enable: Operand::Variable(Identifier(format!("%{prefix}.enable"))),
        program: symbolic_image(
            &format!("{prefix}.program"),
            &schema.program_header,
            &schema.program_tables,
        ),
        pre: symbolic_image(
            &format!("{prefix}.pre"),
            &schema.state_root,
            &schema.state_tables,
        ),
        post: symbolic_image(
            &format!("{prefix}.post"),
            &schema.state_root,
            &schema.state_tables,
        ),
        envelope: LiteralEnvelope {
            rule_tag: Operand::Variable(Identifier(format!("%{prefix}.envelope.rule_tag"))),
            charge: Operand::Variable(Identifier(format!("%{prefix}.envelope.charge"))),
        },
    }
}

fn has_gated_eq(instructions: &[Instruction], enable: &Operand, a: &Operand, b: &Operand) -> bool {
    instructions
        .windows(2)
        .any(|pair| match (&pair[0], &pair[1]) {
            (
                Instruction::CondSelect {
                    bit,
                    a: sel_a,
                    b: sel_b,
                    output,
                },
                Instruction::ConstrainEq { a: eq_a, b: eq_b },
            ) => {
                bit == enable
                    && sel_a == a
                    && sel_b == b
                    && eq_a == &Operand::Variable(output.clone())
                    && eq_b == b
            }
            _ => false,
        })
}

fn root_op(lanes: &[LiteralLane], image: &LiteralImage, name: &str) -> Operand {
    let idx = lanes
        .iter()
        .position(|l| l.name == name)
        .expect("root lane");
    image.root[idx].clone()
}

fn table_op(
    tables: &[LiteralTable],
    image: &LiteralImage,
    table_idx: usize,
    row: usize,
    name: &str,
) -> Operand {
    let col = tables[table_idx]
        .lanes
        .iter()
        .position(|l| l.name == name)
        .expect("table column");
    image.tables[table_idx][row][col].clone()
}

#[test]
fn return_publication_step_emits_exact_guards_and_preservations() {
    assert_eq!(RETURN_PUBLICATION_RULE_ID, "k.admin.publish.value");
    assert_eq!(RETURN_PUBLICATION_RULE_START_LINE, 80);
    assert_eq!(RETURN_PUBLICATION_RULE_TAG, 0);
    assert_eq!(RETURN_PUBLICATION_RULE_CHARGE, 0);

    let schema = literal_schema(&sample_capacities()).expect("valid schema");
    let inputs = symbolic_inputs(&schema, "s80");
    let block = emit_return_publication_step(&schema, &inputs, "pub_step")
        .expect("symbolic return publication emission");

    assert_eq!(block.preserved, inputs);
    assert!(block.instructions.iter().all(|inst| matches!(
        inst,
        Instruction::ConstrainToBoolean { .. }
            | Instruction::ConstrainBits { .. }
            | Instruction::LessThan { .. }
            | Instruction::CondSelect { .. }
            | Instruction::ConstrainEq { .. }
    )));

    let enable = &inputs.enable;
    let zero = immediate(0);
    let one = immediate(1);

    // Envelope: wire tag 0 (line 80) and charge 0.
    assert!(has_gated_eq(
        &block.instructions,
        enable,
        &inputs.envelope.rule_tag,
        &zero
    ));
    assert!(has_gated_eq(
        &block.instructions,
        enable,
        &inputs.envelope.charge,
        &zero
    ));

    // Pre-state outcome Pending(0), depth 0, controlLength 1, and FrameSlots all 0.
    for name in [
        "outcomeTag",
        "outcomeValue.space",
        "outcomeValue.index",
        "outcomeError",
        "depth",
    ] {
        let pre_op = root_op(&schema.state_root, &inputs.pre, name);
        assert!(has_gated_eq(&block.instructions, enable, &pre_op, &zero));
    }
    let pre_ctrl_len = root_op(&schema.state_root, &inputs.pre, "controlLength");
    assert!(has_gated_eq(
        &block.instructions,
        enable,
        &pre_ctrl_len,
        &one
    ));
    for row in &inputs.pre.tables[4] {
        for cell in row {
            assert!(has_gated_eq(&block.instructions, enable, cell, &zero));
        }
    }

    // Pre-state ControlRows[0].tag == 1 (Return), valueRef unconstrained to 0, all other cells 0.
    let cr0_tag = table_op(&schema.state_tables, &inputs.pre, 5, 0, "tag");
    let cr0_space = table_op(&schema.state_tables, &inputs.pre, 5, 0, "valueRef.space");
    let cr0_index = table_op(&schema.state_tables, &inputs.pre, 5, 0, "valueRef.index");
    assert!(has_gated_eq(&block.instructions, enable, &cr0_tag, &one));
    assert!(!has_gated_eq(
        &block.instructions,
        enable,
        &cr0_space,
        &zero
    ));
    assert!(!has_gated_eq(
        &block.instructions,
        enable,
        &cr0_index,
        &zero
    ));
    for cell in &inputs.pre.tables[5][1] {
        assert!(has_gated_eq(&block.instructions, enable, cell, &zero));
    }

    // Active-prefix space selector: CondSelect(cr0.valueRef.space, pre.activeData, program.activeConstants).
    let active_constants = root_op(&schema.program_header, &inputs.program, "activeConstants");
    let active_data = root_op(&schema.state_root, &inputs.pre, "activeData");
    let space_select_out = block
        .instructions
        .iter()
        .find_map(|inst| match inst {
            Instruction::CondSelect { bit, a, b, output }
                if bit == &cr0_space && a == &active_data && b == &active_constants =>
            {
                Some(Operand::Variable(output.clone()))
            }
            _ => None,
        })
        .expect("space-dependent active length CondSelect");
    let less_out = block
        .instructions
        .iter()
        .find_map(|inst| match inst {
            Instruction::LessThan {
                a,
                b,
                bits: 128,
                output,
            } if a == &cr0_index && b == &space_select_out => {
                Some(Operand::Variable(output.clone()))
            }
            _ => None,
        })
        .expect("valueRef.index < selected_active LessThan");
    assert!(has_gated_eq(&block.instructions, enable, &less_out, &one));

    // Post-state Root: controlLength = 0, outcomeTag = 1 (Done), outcomeValue = cr0.valueRef, outcomeError = 0.
    assert!(has_gated_eq(
        &block.instructions,
        enable,
        &root_op(&schema.state_root, &inputs.post, "controlLength"),
        &zero
    ));
    assert!(has_gated_eq(
        &block.instructions,
        enable,
        &root_op(&schema.state_root, &inputs.post, "outcomeTag"),
        &one
    ));
    assert!(has_gated_eq(
        &block.instructions,
        enable,
        &root_op(&schema.state_root, &inputs.post, "outcomeValue.space"),
        &cr0_space
    ));
    assert!(has_gated_eq(
        &block.instructions,
        enable,
        &root_op(&schema.state_root, &inputs.post, "outcomeValue.index"),
        &cr0_index
    ));
    assert!(has_gated_eq(
        &block.instructions,
        enable,
        &root_op(&schema.state_root, &inputs.post, "outcomeError"),
        &zero
    ));

    // All 16 LE remainingFuel bytes and 16 LE initialFuel bytes are preserved without decrement.
    for byte in 0..16 {
        let rem_name = format!("remainingFuel.{byte}");
        let init_name = format!("initialFuel.{byte}");
        assert!(has_gated_eq(
            &block.instructions,
            enable,
            &root_op(&schema.state_root, &inputs.post, &rem_name),
            &root_op(&schema.state_root, &inputs.pre, &rem_name),
        ));
        assert!(has_gated_eq(
            &block.instructions,
            enable,
            &root_op(&schema.state_root, &inputs.post, &init_name),
            &root_op(&schema.state_root, &inputs.pre, &init_name),
        ));
    }

    // All five arenas (tables 0..5) preserved; post ControlRows (table 5) completely zeroed.
    for table in 0..5 {
        for (pre, post) in inputs.pre.tables[table]
            .iter()
            .flatten()
            .zip(inputs.post.tables[table].iter().flatten())
        {
            assert!(has_gated_eq(&block.instructions, enable, post, pre));
        }
    }
    for cell in inputs.post.tables[5].iter().flatten() {
        assert!(has_gated_eq(&block.instructions, enable, cell, &zero));
    }
}

#[test]
fn return_publication_step_composes_after_a102_literal_step_and_supports_zero_arenas() {
    let caps = LiteralCapacities {
        program: [0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 4, 4, 5, 1, 0],
        state: [0, 0, 0, 0, 0, 1],
        stack_limit: 0,
    };
    let schema = literal_schema(&caps).expect("minimal zero-arena schema");
    let program = symbolic_image(
        "trace.program",
        &schema.program_header,
        &schema.program_tables,
    );
    let state0 = symbolic_image("trace.state0", &schema.state_root, &schema.state_tables);
    let state1 = symbolic_image("trace.state1", &schema.state_root, &schema.state_tables);
    let state2 = symbolic_image("trace.state2", &schema.state_root, &schema.state_tables);

    let step0_inputs = LiteralStepInputs {
        enable: Operand::Variable(Identifier("%trace.step0.enable".into())),
        program: program.clone(),
        pre: state0,
        post: state1.clone(),
        envelope: LiteralEnvelope {
            rule_tag: Operand::Variable(Identifier("%trace.step0.rule_tag".into())),
            charge: Operand::Variable(Identifier("%trace.step0.charge".into())),
        },
    };
    let step0 = emit_literal_step(&schema, &step0_inputs, "step0_a102").expect("A102 emission");

    let step1_inputs = ReturnPublicationStepInputs {
        enable: Operand::Variable(Identifier("%trace.step1.enable".into())),
        program: step0.preserved.program.clone(),
        pre: step0.preserved.post.clone(),
        post: state2,
        envelope: LiteralEnvelope {
            rule_tag: Operand::Variable(Identifier("%trace.step1.rule_tag".into())),
            charge: Operand::Variable(Identifier("%trace.step1.charge".into())),
        },
    };
    let step1 = emit_return_publication_step(&schema, &step1_inputs, "step1_a80")
        .expect("A80 emission after A102");

    assert_eq!(step1.preserved.program, step0.preserved.program);
    assert_eq!(step1.preserved.pre, step0.preserved.post);
    assert_eq!(
        ReturnPublicationStepInputs::from(LiteralStepInputs::from(step1_inputs.clone())),
        step1_inputs
    );
}

#[test]
fn return_publication_step_rejects_invalid_scope_variables_and_shapes() {
    let schema = literal_schema(&sample_capacities()).expect("valid schema");
    let base = symbolic_inputs(&schema, "err");

    assert_eq!(
        emit_return_publication_step(&schema, &base, ""),
        Err(ConstructionError::EmptyScope.into())
    );

    let mut unprefixed = base.clone();
    unprefixed.enable = Operand::Variable(Identifier("bad_enable".into()));
    assert_eq!(
        emit_return_publication_step(&schema, &unprefixed, "pub_step"),
        Err(ConstructionError::UnprefixedVariable.into())
    );

    assert!(is_reserved_return_publication_variable(
        "%pub_step.local.temp.0",
        "pub_step"
    ));
    let mut colliding = base.clone();
    colliding.envelope.charge = Operand::Variable(Identifier("%pub_step.local.temp.0".into()));
    assert_eq!(
        emit_return_publication_step(&schema, &colliding, "pub_step"),
        Err(ConstructionError::TemporaryNameCollision.into())
    );

    let mut bad_schema = schema.clone();
    bad_schema.state_root.pop();
    assert_eq!(
        emit_return_publication_step(&bad_schema, &base, "pub_step"),
        Err(LiteralConstructionError::SchemaMismatch)
    );

    let mut bad_post = base;
    bad_post.post.tables[5][0].pop();
    assert_eq!(
        emit_return_publication_step(&schema, &bad_post, "pub_step"),
        Err(LiteralConstructionError::ImageShape {
            block: "post",
            table: Some(5),
            row: Some(0),
        })
    );
}
