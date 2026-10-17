//! Ordinary positive source tests; NOT RUN. Internal host consistency only.
//! No runtime independent expectation outputs, SDK or native operations.
use super::literal_host::{LiteralCapacities, LiteralSourceIdentity};
use super::literal_witness::{program_block_to_image_values, state_block_to_image_values};
use super::return_publication_witness::produce_a102_literal_return_publication;
use mori_core::sherlock::{Control, Expr, Function, Outcome, Program, Tree, Type, Value, K_SOURCE_SHA256};
use mori_core::sherlock::layout_a::{ControlRow, ValueRef};
use std::sync::Arc;

fn identity() -> LiteralSourceIdentity {
    LiteralSourceIdentity {
        k_source_sha256: K_SOURCE_SHA256.into(),
        reference_source_sha256: "11".repeat(32),
        program_source_sha256: "22".repeat(32),
    }
}
fn capacities() -> LiteralCapacities {
    LiteralCapacities {
        program: [2, 16, 4, 4, 4, 4, 2, 2, 4, 2, 4, 4, 5, 16, 16],
        state: [8, 8, 4, 4, 3, 3],
        stack_limit: 8,
    }
}
fn program(value: Value) -> Arc<Program> {
    Arc::new(Program {
        functions: vec![],
        entry: Arc::new(Expr::Literal(value)),
    })
}

#[test]
fn zero_remaining_fuel_publishes_actual_done_and_keeps_inherited_constant_alias() {
    let value = Value::UInt(0xfedc_ba98_7654_3210_0123_4567_89ab_cdef);
    let result = produce_a102_literal_return_publication(
        program(value.clone()), &capacities(), 1, 2, &identity(),
    ).expect("two actual records and complete Step carriers");
    assert_eq!(result.step_record.before, result.literal_transition.step_record.after);
    assert_eq!(result.step_record.before.control, Some(Control::Return(value.clone())));
    assert_eq!(result.step_record.after.control, None);
    assert_eq!(result.step_record.after.outcome, Outcome::Done(value));
    assert_eq!(result.step_record.rule_start_line, 80);
    assert_eq!(result.step_record.charge, 0);
    assert_eq!(result.step_record.before.remaining_fuel, 0);
    assert_eq!(result.step_record.after.remaining_fuel, 0);
    let pre = result.pre_state();
    assert_eq!(pre, &result.literal_transition.post_state);
    assert_eq!(pre.control_rows[0].value, result.literal_transition.selected_constant_ref);
    assert!(!pre.control_rows[0].value.space);
    assert_eq!(result.post_state.root.outcome_value, pre.control_rows[0].value);
    assert_eq!(result.values.pre, result.literal_transition.request.values.post);
    assert_eq!(result.values.rule_tag, 0);
    assert_eq!(result.values.charge, 0);
}

#[test]
fn contrasting_literal_values_preserve_all_five_complete_arenas_and_fuel_bytes() {
    let cases = [
        Value::Bool(true),
        Value::List(vec![17, u128::MAX - 3]),
        Value::Tree(Tree::Branch(Arc::new(Tree::Leaf(7)), Arc::new(Tree::Leaf(11)))),
    ];
    let caps = capacities();
    let fuel = (1u128 << 120) + (1u128 << 64) + 257;
    for value in cases {
        let result = produce_a102_literal_return_publication(
            program(value.clone()), &caps, fuel, 3, &identity(),
        ).expect("normal literal publication");
        let pre = result.pre_state();
        let post = &result.post_state;
        assert_eq!(result.step_record.after.outcome, Outcome::Done(value));
        assert_eq!(pre.data, post.data);
        assert_eq!(pre.data_fields, post.data_fields);
        assert_eq!(pre.env_cells, post.env_cells);
        assert_eq!(pre.value_slots, post.value_slots);
        assert_eq!(pre.frame_slots, post.frame_slots);
        assert_eq!(post.control_rows.len(), caps.state[5]);
        assert!(post.control_rows.iter().all(|row| row == &ControlRow::default()));
        let mut expected_root = pre.root.clone();
        expected_root.control_length = 0;
        expected_root.outcome_tag = 1;
        expected_root.outcome_value = pre.control_rows[0].value;
        expected_root.outcome_error = 0;
        assert_eq!(post.root, expected_root);
        assert_eq!(post.root.remaining_fuel, (fuel - 1).to_le_bytes());
        assert_eq!(post.root.initial_fuel, fuel.to_le_bytes());
        assert_eq!(result.values.program, result.literal_transition.request.values.program);
        assert_eq!(result.values.pre, state_block_to_image_values(pre, &caps).unwrap());
        assert_eq!(result.values.post, state_block_to_image_values(post, &caps).unwrap());
        assert_eq!(result.values.pre.tables.len(), 6);
        assert_eq!(result.values.post.tables.len(), 6);
        assert_eq!(result.values.pre.root.len(), 45);
        assert_eq!(result.values.post.root.len(), 45);
    }
}

#[test]
fn complete_unused_function_program_tables_are_carried_with_publication() {
    let caps = capacities();
    let source = Arc::new(Program {
        functions: vec![Function {
            name: "unused".into(), parameters: vec![], return_type: Type::Bool,
            body: Arc::new(Expr::Literal(Value::Bool(false))),
        }],
        entry: Arc::new(Expr::Literal(Value::UInt(23))),
    });
    let result = produce_a102_literal_return_publication(
        source.clone(), &caps, 2, 0, &identity(),
    ).expect("complete source including unused function");
    assert_eq!(result.literal_transition.prepared.full_program, source);
    assert_eq!(result.values.program, program_block_to_image_values(
        &result.literal_transition.prepared.block, &caps,
    ).unwrap());
    assert_eq!(result.values.program.tables.len(), 15);
    assert_eq!(result.literal_transition.prepared.block.functions.active, 1);
    assert_eq!(result.post_state.root.outcome_value,
        result.literal_transition.selected_constant_ref);
    assert_ne!(result.post_state.root.outcome_value, ValueRef { space: true, index: 0 });
}
