//! Literal complete observation specimens, authored before implementation. UNRUN.
use std::sync::Arc;
use crate::sherlock::{Control, Environment, Expr, Function, Outcome, Parameter,
    Program, Snapshot, StepRecord, Type, Value, PROFILE, K_SOURCE_SHA256};
use crate::sherlock::compare::{first_difference, TraceIdentity, TraceView};
use super::prepare::prepare_a;
use super::rows::{Capacities, ConstantNode, ValueRef};
use super::state::*;
use super::state_decode::decode_observed_a;
use super::observation::*;

fn program() -> Arc<Program> {
    Arc::new(Program {
        entry: Arc::new(Expr::Literal(Value::UInt(7))),
        functions: vec![Function { name: "unused".into(),
            parameters: vec![Parameter { name: "ignored".into(), ty: Type::UInt }],
            return_type: Type::UInt, body: Arc::new(Expr::Literal(Value::UInt(99))) }],
    })
}

fn capacities() -> StateCapacities {
    StateCapacities { data: 1, data_fields: 0, env_cells: 0, value_slots: 0,
        frame_slots: 0, control_rows: 1, stack_limit: 3 }
}

fn snapshot(program: &Arc<Program>, control: Option<Control>, fuel: u128,
    outcome: Outcome) -> Snapshot {
    Snapshot { program: program.clone(), control, environment: Environment::new(),
        frames: vec![], stack_limit: 3, remaining_fuel: fuel, initial_fuel: 2, outcome }
}

// Explicit literal image, including one inactive Data row and one active Eval.
fn eval_image(expr_id: usize) -> StateBlock {
    StateBlock {
        root: StateRoot { control_length: 1, stack_limit: 3,
            remaining_fuel: 2u128.to_le_bytes(), initial_fuel: 2u128.to_le_bytes(),
            ..StateRoot::default() },
        data: vec![ConstantNode::default()], data_fields: vec![], env_cells: vec![],
        value_slots: vec![], frame_slots: vec![],
        control_rows: vec![ControlRow { expr_id, ..ControlRow::default() }],
    }
}

fn return_image() -> StateBlock {
    StateBlock {
        root: StateRoot { active_data: 1, control_length: 1, stack_limit: 3,
            remaining_fuel: 1u128.to_le_bytes(), initial_fuel: 2u128.to_le_bytes(),
            ..StateRoot::default() },
        data: vec![ConstantNode { uint_bytes: 7u128.to_le_bytes(), ..ConstantNode::default() }],
        data_fields: vec![], env_cells: vec![], value_slots: vec![], frame_slots: vec![],
        control_rows: vec![ControlRow { tag: 1, value: ValueRef { space: true, index: 0 },
            ..ControlRow::default() }],
    }
}

fn done_image() -> StateBlock {
    StateBlock {
        root: StateRoot { active_data: 1, stack_limit: 3,
            remaining_fuel: 1u128.to_le_bytes(), initial_fuel: 2u128.to_le_bytes(),
            outcome_tag: 1, outcome_value: ValueRef { space: true, index: 0 },
            ..StateRoot::default() },
        data: vec![ConstantNode { uint_bytes: 7u128.to_le_bytes(), ..ConstantNode::default() }],
        data_fields: vec![], env_cells: vec![], value_slots: vec![], frame_slots: vec![],
        control_rows: vec![ControlRow::default()],
    }
}

#[test]
fn two_complete_records_retain_unused_program_and_compare_through_real_api() {
    let program = program();
    let prepared = prepare_a(program.clone(), Capacities {
        tables: [2, 13, 2, 0, 2, 0, 1, 1, 0, 0, 4, 4, 5, 2, 0], data: 1,
    }).unwrap();
    let shared = ObservationProgramA { profile: PROFILE, full_program: &program,
        program: &prepared.block };
    let initial_image = eval_image(0); let returned_image = return_image(); let done_image = done_image();
    let records = [
        ObservedRecordA { envelope: RecordEnvelopeA { rule_start_line: 102, charge: 1 },
            before_image: &initial_image, after_image: &returned_image },
        ObservedRecordA { envelope: RecordEnvelopeA { rule_start_line: 80, charge: 0 },
            before_image: &returned_image, after_image: &done_image },
    ];
    let identity = TraceIdentity { profile: PROFILE, source_sha256: K_SOURCE_SHA256 };
    let actual = materialize_trace_a(&prepared, &capacities(), &shared, identity,
        &initial_image, &records).unwrap();
    // Independent literal typed snapshots: no Machine, encoder or candidate result.
    let initial = snapshot(&program, Some(Control::Eval(Arc::new(Expr::Literal(Value::UInt(7))))), 2, Outcome::Pending);
    let returned = snapshot(&program, Some(Control::Return(Value::UInt(7))), 1, Outcome::Pending);
    let done = snapshot(&program, None, 1, Outcome::Done(Value::UInt(7)));
    let expected = [StepRecord { rule_start_line: 102, charge: 1, before: initial.clone(), after: returned.clone() },
        StepRecord { rule_start_line: 80, charge: 0, before: returned, after: done }];
    assert_eq!(actual.identity, identity);
    assert_eq!(first_difference(actual.view(), TraceView { identity, initial: &initial, records: &expected }), None);
    assert_eq!(actual.records, expected);
    assert_eq!(actual.initial.program.as_ref(), program.as_ref());
    assert_eq!(materialize_record_a(&prepared, &capacities(), &shared, &records[0]).unwrap(), expected[0]);
    assert_eq!(compare_a(&actual, &actual), None);
}

#[test]
fn supplied_initial_with_no_records_is_a_present_trace_with_declared_metadata() {
    let program = program();
    let prepared = prepare_a(program.clone(), Capacities {
        tables: [2, 13, 2, 0, 2, 0, 1, 1, 0, 0, 4, 4, 5, 2, 0], data: 1,
    }).unwrap();
    let shared = ObservationProgramA { profile: PROFILE, full_program: &program, program: &prepared.block };
    let image = eval_image(0);
    let identity = TraceIdentity { profile: PROFILE, source_sha256: K_SOURCE_SHA256 };
    let actual = materialize_trace_a(&prepared, &capacities(), &shared, identity, &image, &[]).unwrap();
    let expected = snapshot(&program, Some(Control::Eval(Arc::new(Expr::Literal(Value::UInt(7))))), 2, Outcome::Pending);
    assert_eq!(actual.identity, identity);
    assert!(actual.records.is_empty());
    assert_eq!(first_difference(actual.view(), TraceView { identity, initial: &expected, records: &[] }), None);
}

#[test]
fn projection_reconstructs_equal_occurrence_two_and_chooses_least_one() {
    let program = Arc::new(Program { functions: vec![], entry: Arc::new(Expr::Let("x".into(),
        Arc::new(Expr::Literal(Value::UInt(7))), Arc::new(Expr::Literal(Value::UInt(7))))) });
    let prepared = prepare_a(program.clone(), Capacities {
        tables: [1, 1, 3, 2, 3, 2, 0, 0, 0, 0, 4, 4, 5, 2, 0], data: 1,
    }).unwrap();
    let step = eval_image(2);
    let supplied = AImage { profile: PROFILE, full_program: &program, program: &prepared.block, state: &step };
    let actual = project_a(&prepared, &capacities(), &supplied).unwrap();
    assert_eq!(actual, eval_image(1));
    let projected = AImage { profile: PROFILE, full_program: &program, program: &prepared.block, state: &actual };
    let expected = snapshot(&program, Some(Control::Eval(Arc::new(Expr::Literal(Value::UInt(7))))), 2, Outcome::Pending);
    assert_eq!(decode_observed_a(&prepared, &capacities(), &projected, DecodeMode::Canonical).unwrap(), expected);
    assert_eq!(step.control_rows[0].expr_id, 2);
}

#[test]
fn all_thirty_nine_source_lines_have_the_declared_separate_enum_tags() {
    let expected = [(80,0),(82,0),(84,0),(102,1),(108,1),(111,1),(114,1),(117,1),
        (120,1),(123,1),(126,1),(129,1),(131,1),(135,1),(140,1),(145,1),(149,1),
        (153,1),(157,1),(161,1),(165,1),(169,1),(173,1),(177,1),(181,1),(185,1),
        (190,1),(195,1),(200,1),(202,1),(204,1),(206,1),(208,1),(212,1),(214,1),
        (217,1),(220,1),(223,1),(225,1)];
    for (tag, (rule_start_line, charge)) in expected.into_iter().enumerate() {
        let envelope = RecordEnvelopeA { rule_start_line, charge };
        assert_eq!(envelope.rule_tag().unwrap(), tag as u8);
        assert_eq!(envelope.rule_start_line, rule_start_line);
        assert_eq!(envelope.charge, charge);
    }
}
