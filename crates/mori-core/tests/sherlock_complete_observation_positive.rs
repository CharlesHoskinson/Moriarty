//! Source-only ordinary observer/continuity regression proposal.
//! Uses actual fixed-owner admission/steps; no independent full semantic table
//! is populated or claimed here. Root must classify this new execution closure.
use std::sync::Arc;
use mori_core::sherlock::{Expr, Program, Value};
use mori_core::sherlock::helper_machine::complete_observation::{
    self as complete, CompleteStepRecord, ContinuityBoundary, ObservedStep,
};
use mori_core::sherlock::helper_machine::shared::{
    Admitted, BoundsC, FinanceInput, LayoutCapacitiesC, LayoutSchemaC, MachineImage,
    OwnerEngine, RunIntC,
};
use mori_core::sherlock::helper_machine::shared::owner_host::host_owner_engine;
use mori_core::sherlock::helper_machine::source_input_bridge;

fn literal_pair() -> (Admitted, MachineImage) {
    let program = Program { functions:vec![], entry:Arc::new(Expr::Literal(Value::UInt(7))) };
    let source = source_input_bridge::from_program(&program);
    let bounds = BoundsC {
        code_rows:64, function_rows:4, code_child_slots:16, parameter_slots:8,
        pattern_rows:8, match_plan_rows:8, match_arm_rows:8, type_rows:8,
        constructor_rows:8, constructor_field_slots:16, max_arity:8,
        constant_nodes:16, constant_field_slots:16, data_nodes:64,
        data_field_slots:64, environment_cells:64, frames:64, helper_tasks:64,
        serialization_bytes:65536, max_reference_work:4096, max_helper_work:4096, step_cap:4096,
    };
    let layout = LayoutSchemaC {
        layout_version:("moriarty".into(), "sherlock-helper-microstep-hostonly/1".into()),
        field_id:None, widths:None, scalar_layout_id:None, bounds,
        capacities:LayoutCapacitiesC { name_rows:32, name_bytes:256, occurrence_path_slots:256, value_slots:1024 },
        ext_capacities: None, boundary_capacities_present:false,
    };
    let owner = host_owner_engine();
    let admitted = owner.admit(&source, &bounds, &layout)
        .unwrap_or_else(|_| panic!("fixed ordinary literal admission"));
    let image = owner.admit_run(&admitted, &RunIntC::Nat(64), &RunIntC::Nat(8), &FinanceInput::Absent)
        .unwrap_or_else(|_| panic!("fixed ordinary genesis"));
    (admitted, image)
}
fn event(admitted: &Admitted, image: &mut MachineImage) -> CompleteStepRecord {
    match complete::record_step(admitted, image) {
        Ok(ObservedStep::Event(record)) => record,
        Ok(ObservedStep::Terminal { .. }) => panic!("expected an ordinary prefix event"),
        Err(_) => panic!("complete fixed-owner prefix observation"),
    }
}

#[test]
fn actual_pre_is_owned_before_step_and_post_matches_returned_image() {
    let (admitted, mut image) = literal_pair();
    let initial = complete::capture(&admitted, &image).unwrap_or_else(|_| panic!("initial copy"));
    let record = event(&admitted, &mut image);
    let post = complete::capture(&admitted, &image).unwrap_or_else(|_| panic!("post copy"));
    assert_eq!(record.before, initial);
    assert_eq!(record.after, post);
    let saved_before = record.before.clone();
    let _next = event(&admitted, &mut image);
    assert_eq!(record.before, saved_before);
    assert_eq!(record.before.image.depth(), record.before.image.frames.len());
    assert_eq!(record.before.image.task_live(), record.before.image.tasks.len());
}

#[test]
fn present_empty_records_compare_initial_to_returned_without_terminality_claim() {
    let (admitted, image) = literal_pair();
    let initial = complete::capture(&admitted, &image).unwrap_or_else(|_| panic!("initial copy"));
    assert!(complete::first_discontinuity(&initial, &[], &initial).is_none());
}

#[test]
fn two_actual_complete_prefix_records_are_contiguous_in_every_field() {
    let (admitted, mut image) = literal_pair();
    let initial = complete::capture(&admitted, &image).unwrap_or_else(|_| panic!("initial copy"));
    let records = vec![event(&admitted, &mut image), event(&admitted, &mut image)];
    let returned = complete::capture(&admitted, &image).unwrap_or_else(|_| panic!("returned copy"));
    assert_eq!(records[0].after, records[1].before);
    assert!(complete::first_discontinuity(&initial, &records, &returned).is_none());
}

#[test]
fn different_owned_returned_metadata_reports_the_final_boundary() {
    let (admitted, mut image) = literal_pair();
    let initial = complete::capture(&admitted, &image).unwrap_or_else(|_| panic!("initial copy"));
    let records = vec![event(&admitted, &mut image)];
    let mut returned = complete::capture(&admitted, &image).unwrap_or_else(|_| panic!("returned copy"));
    // Ordinary supplied-record contrast only; no actual owner image is mutated.
    returned.admission.layout.layout_version.1.push_str("-other");
    let difference = complete::first_discontinuity(&initial, &records, &returned)
        .expect("changed full observation metadata must differ");
    assert_eq!(difference.boundary, ContinuityBoundary::FinalReturned { final_record:0 });
    assert_eq!(difference.expected, &records[0].after);
    assert_eq!(difference.observed, &returned);
}
