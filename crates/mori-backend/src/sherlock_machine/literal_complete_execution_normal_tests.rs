use super::literal_complete_execution::{
    emit_literal_complete_execution, is_reserved_literal_complete_execution_variable,
    LiteralCompleteExecutionInputs, COMPLETE_LITERAL_EXECUTION_STEP1_RULE_CHARGE,
    COMPLETE_LITERAL_EXECUTION_STEP1_RULE_TAG, COMPLETE_LITERAL_EXECUTION_STEP2_RULE_CHARGE,
    COMPLETE_LITERAL_EXECUTION_STEP2_RULE_TAG, COMPLETE_LITERAL_EXECUTION_TOTAL_CHARGE,
};
use super::return_publication_step::{
    emit_return_publication_step, ReturnPublicationStepInputs,
};
use super::*;

fn var(name: &str) -> Operand {
    Operand::Variable(Identifier(name.to_string()))
}

fn sample_capacities() -> LiteralCapacities {
    // Finite test specimen dimensions only, not product resource bounds.
    LiteralCapacities {
        program: [1, 2, 1, 1, 2, 1, 0, 0, 0, 0, 4, 4, 5, 3, 2],
        state: [2, 2, 1, 1, 1, 2],
        stack_limit: 8,
    }
}

fn build_image(prefix: &str, root: &[LiteralLane], tables: &[LiteralTable]) -> LiteralImage {
    let root_ops = root
        .iter()
        .map(|lane| var(&format!("%{prefix}.root.{}", lane.name)))
        .collect();
    let table_ops = tables
        .iter()
        .map(|table| {
            (0..table.capacity)
                .map(|r| {
                    table
                        .lanes
                        .iter()
                        .map(|lane| var(&format!("%{prefix}.{}.{r}.{}", table.name, lane.name)))
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

fn sample_inputs(schema: &LiteralSchema) -> LiteralCompleteExecutionInputs {
    LiteralCompleteExecutionInputs {
        program: build_image("prog", &schema.program_header, &schema.program_tables),
        init: build_image("init", &schema.state_root, &schema.state_tables),
        intermediate: build_image("mid", &schema.state_root, &schema.state_tables),
        done: build_image("done", &schema.state_root, &schema.state_tables),
        literal_envelope: LiteralEnvelope {
            rule_tag: var("%env.step1.rule_tag"),
            charge: var("%env.step1.charge"),
        },
        return_envelope: LiteralEnvelope {
            rule_tag: var("%env.step2.rule_tag"),
            charge: var("%env.step2.charge"),
        },
    }
}

fn root_op(schema: &LiteralSchema, image: &LiteralImage, name: &str) -> Operand {
    let idx = schema
        .state_root
        .iter()
        .position(|lane| lane.name == name)
        .expect("state_root lane");
    image.root[idx].clone()
}

#[test]
fn emits_both_real_child_vectors_in_a102_then_a80_order_with_aliased_intermediate() {
    let schema = literal_schema(&sample_capacities()).expect("valid schema");
    let inputs = sample_inputs(&schema);
    let block = emit_literal_complete_execution(&schema, &inputs, "complete_exec")
        .expect("complete execution emission");

    let expected_a102 = emit_literal_step(
        &schema,
        &LiteralStepInputs {
            enable: immediate(1),
            program: inputs.program.clone(),
            pre: inputs.init.clone(),
            post: inputs.intermediate.clone(),
            envelope: inputs.literal_envelope.clone(),
        },
        "complete_exec.literal_step",
    )
    .expect("direct A102 emission");
    let expected_a80 = emit_return_publication_step(
        &schema,
        &ReturnPublicationStepInputs {
            enable: immediate(1),
            program: inputs.program.clone(),
            pre: inputs.intermediate.clone(),
            post: inputs.done.clone(),
            envelope: inputs.return_envelope.clone(),
        },
        "complete_exec.return_publication_step",
    )
    .expect("direct A80 emission");

    assert_eq!(block.literal_step_instructions, expected_a102.instructions);
    assert_eq!(
        block.return_publication_step_instructions,
        expected_a80.instructions
    );
    assert_eq!(
        block.step_instruction_vectors(),
        [
            &expected_a102.instructions[..],
            &expected_a80.instructions[..],
        ]
    );
    assert_eq!(block.selected_expression, expected_a102.selected_expression);

    // Shared immutable ProgramBlock and exact aliased intermediate LiteralImage across both calls.
    assert_eq!(block.literal_step_preserved.enable, immediate(1));
    assert_eq!(block.return_publication_step_preserved.enable, immediate(1));
    assert_eq!(block.literal_step_preserved.program, inputs.program);
    assert_eq!(
        block.return_publication_step_preserved.program,
        inputs.program
    );
    assert_eq!(block.literal_step_preserved.pre, inputs.init);
    assert_eq!(block.literal_step_preserved.post, inputs.intermediate);
    assert_eq!(
        block.return_publication_step_preserved.pre,
        inputs.intermediate
    );
    assert_eq!(
        block.literal_step_preserved.post,
        block.return_publication_step_preserved.pre
    );
    assert_eq!(block.return_publication_step_preserved.post, inputs.done);
    assert_eq!(block.preserved, inputs);

    let mut expected_all = block.init_instructions.clone();
    expected_all.extend(expected_a102.instructions);
    expected_all.extend(expected_a80.instructions);
    assert_eq!(block.instructions, expected_all);
    assert!(!block.instructions.iter().any(|ix| matches!(
        ix,
        Instruction::PublicInput { .. } | Instruction::Output { .. }
    )));
}

#[test]
fn grounds_machine_new_init_equations_final_writes_and_16_byte_fuel_work_wiring() {
    let schema = literal_schema(&sample_capacities()).expect("valid schema");
    let inputs = sample_inputs(&schema);
    let block = emit_literal_complete_execution(&schema, &inputs, "complete_exec")
        .expect("complete execution emission");

    let has_eq = |slice: &[Instruction], a: Operand, b: Operand| {
        slice.contains(&Instruction::ConstrainEq { a, b })
    };

    // Fixed step envelopes and circuit-computed total work == 1.
    assert!(has_eq(
        &block.init_instructions,
        inputs.literal_envelope.rule_tag.clone(),
        immediate(COMPLETE_LITERAL_EXECUTION_STEP1_RULE_TAG),
    ));
    assert!(has_eq(
        &block.init_instructions,
        inputs.literal_envelope.charge.clone(),
        immediate(COMPLETE_LITERAL_EXECUTION_STEP1_RULE_CHARGE),
    ));
    assert!(has_eq(
        &block.init_instructions,
        inputs.return_envelope.rule_tag.clone(),
        immediate(COMPLETE_LITERAL_EXECUTION_STEP2_RULE_TAG),
    ));
    assert!(has_eq(
        &block.init_instructions,
        inputs.return_envelope.charge.clone(),
        immediate(COMPLETE_LITERAL_EXECUTION_STEP2_RULE_CHARGE),
    ));
    assert_eq!(block.charged_work, var("%complete_exec.local.temp.0"));
    assert!(block.init_instructions.contains(&Instruction::Add {
        a: inputs.literal_envelope.charge.clone(),
        b: inputs.return_envelope.charge.clone(),
        output: Identifier("%complete_exec.local.temp.0".into()),
    }));
    assert!(has_eq(
        &block.init_instructions,
        block.charged_work.clone(),
        immediate(COMPLETE_LITERAL_EXECUTION_TOTAL_CHARGE),
    ));

    // Entry expression and Machine::new Init root/table equations.
    let entry_expr = inputs.program.root[0].clone();
    let init_expr0 = inputs.init.tables[5][0][1].clone();
    assert!(has_eq(&block.init_instructions, entry_expr.clone(), immediate(0)));
    assert!(has_eq(&block.init_instructions, init_expr0.clone(), entry_expr));
    assert!(has_eq(&block.init_instructions, init_expr0, immediate(0)));

    for zero_lane in [
        "activeData",
        "activeDataFields",
        "activeEnvCells",
        "activeValueSlots",
        "depth",
        "environment.present",
        "environment.index",
        "outcomeTag",
        "outcomeValue.space",
        "outcomeValue.index",
        "outcomeError",
    ] {
        assert!(has_eq(
            &block.init_instructions,
            root_op(&schema, &inputs.init, zero_lane),
            immediate(0),
        ));
    }
    assert!(has_eq(
        &block.init_instructions,
        root_op(&schema, &inputs.init, "controlLength"),
        immediate(1),
    ));
    for table in 0..6 {
        for cell in inputs.init.tables[table].iter().flatten() {
            assert!(has_eq(&block.init_instructions, cell.clone(), immediate(0)));
        }
    }

    // 16-byte LE fuel equality at Init and preservation of initialFuel and post-decrement remainingFuel.
    for byte in 0..16 {
        let rem = format!("remainingFuel.{byte}");
        let ini = format!("initialFuel.{byte}");
        assert!(has_eq(
            &block.init_instructions,
            root_op(&schema, &inputs.init, &rem),
            root_op(&schema, &inputs.init, &ini),
        ));
        assert!(has_eq(
            &block.init_instructions,
            root_op(&schema, &inputs.intermediate, &ini),
            root_op(&schema, &inputs.init, &ini),
        ));
        assert!(has_eq(
            &block.init_instructions,
            root_op(&schema, &inputs.done, &ini),
            root_op(&schema, &inputs.intermediate, &ini),
        ));
        assert!(has_eq(
            &block.init_instructions,
            root_op(&schema, &inputs.done, &rem),
            root_op(&schema, &inputs.intermediate, &rem),
        ));
    }

    // Intermediate Return(V) write in A102 and terminal Done(V) + ControlRows release in A80.
    let mid_ret_space = inputs.intermediate.tables[5][0][2].clone();
    let mid_ret_index = inputs.intermediate.tables[5][0][3].clone();
    let done_out_tag = root_op(&schema, &inputs.done, "outcomeTag");
    let done_out_space = root_op(&schema, &inputs.done, "outcomeValue.space");
    let done_out_index = root_op(&schema, &inputs.done, "outcomeValue.index");
    assert!(block.return_publication_step_instructions.iter().any(|ix| {
        matches!(
            ix,
            Instruction::CondSelect { bit, a, b, .. }
                if *bit == immediate(1) && *a == done_out_tag && *b == immediate(1)
        )
    }));
    assert!(block.return_publication_step_instructions.iter().any(|ix| {
        matches!(
            ix,
            Instruction::CondSelect { bit, a, b, .. }
                if *bit == immediate(1) && *a == done_out_space && *b == mid_ret_space
        )
    }));
    assert!(block.return_publication_step_instructions.iter().any(|ix| {
        matches!(
            ix,
            Instruction::CondSelect { bit, a, b, .. }
                if *bit == immediate(1) && *a == done_out_index && *b == mid_ret_index
        )
    }));
}

#[test]
fn rejects_invalid_shapes_scopes_unprefixed_vars_and_cross_callee_namespace_collisions() {
    let schema = literal_schema(&sample_capacities()).expect("valid schema");
    let base = LiteralCompleteExecutionInputs::new(
        build_image("prog", &schema.program_header, &schema.program_tables),
        build_image("init", &schema.state_root, &schema.state_tables),
        build_image("mid", &schema.state_root, &schema.state_tables),
        build_image("done", &schema.state_root, &schema.state_tables),
    );

    let mut tampered_schema = schema.clone();
    tampered_schema.state_root.pop();
    assert_eq!(
        emit_literal_complete_execution(&tampered_schema, &base, "s").unwrap_err(),
        LiteralConstructionError::SchemaMismatch
    );
    assert_eq!(
        emit_literal_complete_execution(&schema, &base, "").unwrap_err(),
        LiteralConstructionError::Fetch(ConstructionError::EmptyScope)
    );

    let mut bad_done = base.clone();
    bad_done.done.tables[5].pop();
    assert_eq!(
        emit_literal_complete_execution(&schema, &bad_done, "s").unwrap_err(),
        LiteralConstructionError::ImageShape {
            block: "done",
            table: Some(5),
            row: None,
        }
    );

    let mut unprefixed = base.clone();
    unprefixed.done.root[0] = var("done.root.activeData");
    assert_eq!(
        emit_literal_complete_execution(&schema, &unprefixed, "s").unwrap_err(),
        LiteralConstructionError::Fetch(ConstructionError::UnprefixedVariable)
    );

    // Cross-callee collisions: `done` colliding with A102/ui_valid namespaces, and `init` colliding with A80.
    for colliding in [
        "%s.local.temp.0",
        "%s.literal_step.local.temp.0",
        "%s.literal_step.expression_fetch.temp.0",
        "%s.literal_step.ui_valid.const_0.type_fetch.temp.0",
        "%s.return_publication_step.local.temp.0",
    ] {
        assert!(is_reserved_literal_complete_execution_variable(
            colliding, "s", 3, 2
        ));
        let mut in_done = base.clone();
        in_done.done.root[0] = var(colliding);
        assert_eq!(
            emit_literal_complete_execution(&schema, &in_done, "s").unwrap_err(),
            LiteralConstructionError::Fetch(ConstructionError::TemporaryNameCollision)
        );
        let mut in_init = base.clone();
        in_init.init.root[0] = var(colliding);
        assert_eq!(
            emit_literal_complete_execution(&schema, &in_init, "s").unwrap_err(),
            LiteralConstructionError::Fetch(ConstructionError::TemporaryNameCollision)
        );
    }
}
