//! Symbolic source-only tests for `error_publication_step`.
//! Inspects emitted ZKIR instructions over normative `LiteralSchema` symbolic
//! and boundary-immediate operands without SDK execution, witness evaluation,
//! or host-oracle cell values.

use super::error_publication_step::{
    ERROR_PUBLICATION_RULE_CHARGE, ERROR_PUBLICATION_RULE_ID, ERROR_PUBLICATION_RULE_START_LINE,
    ERROR_PUBLICATION_RULE_TAG, ErrorPublicationStepInputs, emit_error_publication_step,
    is_reserved_error_publication_variable,
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

fn symbolic_inputs(schema: &LiteralSchema, prefix: &str) -> ErrorPublicationStepInputs {
    ErrorPublicationStepInputs {
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

fn has_constrain_bits(instructions: &[Instruction], val: &Operand, bits: u32) -> bool {
    instructions.iter().any(|inst| {
        matches!(
            inst,
            Instruction::ConstrainBits { val: v, bits: b } if v == val && *b == bits
        )
    })
}

fn has_less_than_on(instructions: &[Instruction], operand: &Operand) -> bool {
    instructions.iter().any(|inst| {
        matches!(
            inst,
            Instruction::LessThan { a, b, .. } if a == operand || b == operand
        )
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
fn error_publication_step_emits_exact_guards_u8_wiring_and_preservations() {
    assert_eq!(ERROR_PUBLICATION_RULE_ID, "k.admin.publish.error");
    assert_eq!(ERROR_PUBLICATION_RULE_START_LINE, 82);
    assert_eq!(ERROR_PUBLICATION_RULE_TAG, 1);
    assert_eq!(ERROR_PUBLICATION_RULE_CHARGE, 0);

    let schema = literal_schema(&sample_capacities()).expect("valid schema");
    let inputs = symbolic_inputs(&schema, "s82");
    let block = emit_error_publication_step(&schema, &inputs, "err_pub_step")
        .expect("symbolic error publication emission");

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
    let two = immediate(2);
    let nine = immediate(9);

    // Envelope: wire tag 1 (line 82) and charge 0.
    assert!(has_gated_eq(
        &block.instructions,
        enable,
        &inputs.envelope.rule_tag,
        &one
    ));
    assert!(has_gated_eq(
        &block.instructions,
        enable,
        &inputs.envelope.charge,
        &zero
    ));

    // Pre-state outcome Pending(0) with zeroed inactive outcome lanes and controlLength 1.
    for name in [
        "outcomeTag",
        "outcomeValue.space",
        "outcomeValue.index",
        "outcomeError",
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

    // Unlike A80 Return, pre depth and FrameSlots are NOT forced to 0.
    let pre_depth = root_op(&schema.state_root, &inputs.pre, "depth");
    assert!(!has_gated_eq(
        &block.instructions,
        enable,
        &pre_depth,
        &zero
    ));
    for row in &inputs.pre.tables[4] {
        for cell in row {
            assert!(!has_gated_eq(&block.instructions, enable, cell, &zero));
        }
    }

    // Pre-state ControlRows[0].tag == 9 (Error), errorCode is an unwhitelisted 8-bit u8,
    // every other column of row 0 (including all pendingFrame columns) is gated to 0,
    // and all columns of inactive rows (row >= 1) are gated to 0.
    let cr0_tag = table_op(&schema.state_tables, &inputs.pre, 5, 0, "tag");
    let cr0_error = table_op(&schema.state_tables, &inputs.pre, 5, 0, "errorCode");
    assert!(has_gated_eq(&block.instructions, enable, &cr0_tag, &nine));
    assert!(!has_gated_eq(
        &block.instructions,
        enable,
        &cr0_error,
        &zero
    ));
    assert!(has_constrain_bits(&block.instructions, &cr0_error, 8));
    assert!(!has_less_than_on(&block.instructions, &cr0_error));

    for (lane, cell) in schema.state_tables[5]
        .lanes
        .iter()
        .zip(&inputs.pre.tables[5][0])
    {
        if lane.name != "tag" && lane.name != "errorCode" {
            assert!(has_gated_eq(&block.instructions, enable, cell, &zero));
        }
    }
    for cell in &inputs.pre.tables[5][1] {
        assert!(has_gated_eq(&block.instructions, enable, cell, &zero));
    }

    // Post-state Root: controlLength = 0, outcomeTag = 2 (Rejected),
    // outcomeValue.{space,index} = 0, outcomeError = cr0.errorCode.
    let post_outcome_error = root_op(&schema.state_root, &inputs.post, "outcomeError");
    assert!(has_constrain_bits(
        &block.instructions,
        &post_outcome_error,
        8
    ));
    assert!(!has_less_than_on(&block.instructions, &post_outcome_error));
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
        &two
    ));
    assert!(has_gated_eq(
        &block.instructions,
        enable,
        &root_op(&schema.state_root, &inputs.post, "outcomeValue.space"),
        &zero
    ));
    assert!(has_gated_eq(
        &block.instructions,
        enable,
        &root_op(&schema.state_root, &inputs.post, "outcomeValue.index"),
        &zero
    ));
    assert!(has_gated_eq(
        &block.instructions,
        enable,
        &post_outcome_error,
        &cr0_error
    ));

    // All other Root fields (active counts, depth, environment, stackLimit, and all 16 LE
    // remainingFuel and initialFuel bytes) are preserved post == pre.
    for name in [
        "activeData",
        "activeDataFields",
        "activeEnvCells",
        "activeValueSlots",
        "depth",
        "environment.present",
        "environment.index",
        "stackLimit",
    ] {
        assert!(has_gated_eq(
            &block.instructions,
            enable,
            &root_op(&schema.state_root, &inputs.post, name),
            &root_op(&schema.state_root, &inputs.pre, name),
        ));
    }
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

    // All five arenas (tables 0..5, including FrameSlots at table 4) preserved;
    // post ControlRows (table 5) completely zeroed.
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
fn error_publication_step_supports_error_0_and_255_nonempty_frames_and_zero_fuel() {
    let schema = literal_schema(&sample_capacities()).expect("valid schema");
    let depth_idx = schema
        .state_root
        .iter()
        .position(|l| l.name == "depth")
        .expect("depth column");
    let err_col = schema.state_tables[5]
        .lanes
        .iter()
        .position(|l| l.name == "errorCode")
        .expect("errorCode column");

    for code_byte in [0u128, 255u128] {
        let mut inputs = symbolic_inputs(&schema, &format!("case_{code_byte}"));
        inputs.pre.root[depth_idx] = immediate(2);
        inputs.pre.tables[4][0][0] = immediate(7);
        inputs.pre.tables[4][1][0] = immediate(8);
        inputs.pre.tables[5][0][err_col] = immediate(code_byte);
        for byte in 0..16 {
            let rem_idx = schema
                .state_root
                .iter()
                .position(|l| l.name == format!("remainingFuel.{byte}"))
                .expect("remainingFuel byte");
            inputs.pre.root[rem_idx] = immediate(0);
        }

        let block = emit_error_publication_step(&schema, &inputs, "err_boundary")
            .expect("boundary error publication emission");
        let enable = &inputs.enable;
        let code_imm = immediate(code_byte);
        let post_err = root_op(&schema.state_root, &inputs.post, "outcomeError");
        let post_depth = root_op(&schema.state_root, &inputs.post, "depth");

        assert!(has_constrain_bits(&block.instructions, &code_imm, 8));
        assert!(!has_less_than_on(&block.instructions, &code_imm));
        assert!(has_gated_eq(
            &block.instructions,
            enable,
            &post_err,
            &code_imm
        ));
        assert!(has_gated_eq(
            &block.instructions,
            enable,
            &post_depth,
            &immediate(2)
        ));
        assert!(has_gated_eq(
            &block.instructions,
            enable,
            &inputs.post.tables[4][0][0],
            &immediate(7)
        ));
        assert!(has_gated_eq(
            &block.instructions,
            enable,
            &inputs.post.tables[4][1][0],
            &immediate(8)
        ));
        for byte in 0..16 {
            let rem_post = root_op(
                &schema.state_root,
                &inputs.post,
                &format!("remainingFuel.{byte}"),
            );
            assert!(has_gated_eq(
                &block.instructions,
                enable,
                &rem_post,
                &immediate(0)
            ));
        }
        assert_eq!(
            ErrorPublicationStepInputs::from(LiteralStepInputs::from(inputs.clone())),
            inputs
        );
    }
}

#[test]
fn error_publication_step_rejects_invalid_scope_variables_and_shapes() {
    let schema = literal_schema(&sample_capacities()).expect("valid schema");
    let base = symbolic_inputs(&schema, "err");

    assert_eq!(
        emit_error_publication_step(&schema, &base, ""),
        Err(ConstructionError::EmptyScope.into())
    );

    let mut unprefixed = base.clone();
    unprefixed.enable = Operand::Variable(Identifier("bad_enable".into()));
    assert_eq!(
        emit_error_publication_step(&schema, &unprefixed, "err_step"),
        Err(ConstructionError::UnprefixedVariable.into())
    );

    assert!(is_reserved_error_publication_variable(
        "%err_step.local.temp.0",
        "err_step"
    ));
    let mut colliding = base.clone();
    colliding.envelope.charge = Operand::Variable(Identifier("%err_step.local.temp.0".into()));
    assert_eq!(
        emit_error_publication_step(&schema, &colliding, "err_step"),
        Err(ConstructionError::TemporaryNameCollision.into())
    );

    let mut bad_schema = schema.clone();
    bad_schema.state_root.pop();
    assert_eq!(
        emit_error_publication_step(&bad_schema, &base, "err_step"),
        Err(LiteralConstructionError::SchemaMismatch)
    );

    let mut bad_post = base;
    bad_post.post.tables[5][0].pop();
    assert_eq!(
        emit_error_publication_step(&schema, &bad_post, "err_step"),
        Err(LiteralConstructionError::ImageShape {
            block: "post",
            table: Some(5),
            row: Some(0),
        })
    );
}
