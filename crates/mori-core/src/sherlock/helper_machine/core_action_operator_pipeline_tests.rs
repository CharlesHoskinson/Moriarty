//! Normal actual-operator pipeline tests, SOURCE_ONLY / UNCOMPILED / UNRUN.
//! Same actual owner wrapper, bound observation and reports as production.
//! Core-local differential results are ordinary cross-path checks; no private
//! frozen O, expected B state/work, captured result or second interpreter.
use std::collections::BTreeMap;
use super::core_action_consumer::{ConsumerConfig, ConsumerProfile, SourceHostBounds};
use super::raw::{ConstRow, DataRow};
use super::resource_event_accounting::{run_scalar_action_with_event_accounting,
    AccountedScalarRun, ArithmeticStatus, CheckedTotal, PrefixEnd};
use super::resource_report::Counter;
use super::shared::owner_host::HostObservation;
use super::shared::{BoundsC, FinanceInput, GrammarDomain, LayoutCapacitiesC,
    LayoutSchemaC, OutcomeC, RefC, RunIntC, StepEventC};
use crate::BinaryOp as CoreOp;

// Exact designated-owner proposed configuration values. The function name is
// local to these tests; no production default, policy, cap or guard changes.
// Static 128-code/512-path fit votes are not runtime adequacy or a new-case vote.
fn operator_config() -> ConsumerConfig {
    let bounds = BoundsC {
        code_rows: 128,
        function_rows: 4,
        code_child_slots: 16,
        parameter_slots: 8,
        pattern_rows: 8,
        match_plan_rows: 8,
        match_arm_rows: 8,
        type_rows: 8,
        constructor_rows: 8,
        constructor_field_slots: 16,
        max_arity: 8,
        constant_nodes: 16,
        constant_field_slots: 16,
        data_nodes: 64,
        data_field_slots: 64,
        environment_cells: 64,
        frames: 64,
        helper_tasks: 64,
        serialization_bytes: 65536,
        max_reference_work: 4096,
        max_helper_work: 4096,
        step_cap: 4096,
    };
    let capacities = LayoutCapacitiesC {
        name_rows: 32,
        name_bytes: 256,
        occurrence_path_slots: 512,
        value_slots: 1024,
    };
    let layout = LayoutSchemaC {
        layout_version: (
            "moriarty".to_string(),
            "sherlock-helper-microstep-hostonly/1".to_string(),
        ),
        field_id: None,
        widths: None,
        scalar_layout_id: None,
        bounds,
        capacities,
        ext_capacities: None, boundary_capacities_present: false,
    };
    ConsumerConfig {
        bounds,
        layout,
        fuel: RunIntC::Nat(1000),
        stack: RunIntC::Nat(32),
        profile: ConsumerProfile::SherlockHelperMicrostep1,
        grammar: GrammarDomain::DedicatedSorts,
        finance: FinanceInput::Absent,
        host: SourceHostBounds {
            max_source_actions: 4,
            max_source_expressions: 256,
            max_source_statements: 64,
            max_action_inputs: 8,
            max_supplied_inputs: 8,
            max_projection_occurrences: 512,
            max_generated_ast_nodes: 512,
            max_ast_nesting: 128,
            max_input_name_bytes: 64,
            max_total_input_name_bytes: 512,
        },
    }
}

// Test-only fixed-size observations of events actually forwarded to the sink.
// Counter arithmetic is checked; this adds no owner charge or admission rule.
#[derive(Default)]
struct SinkFacts {
    events: u128,
    work: u128,
    helper_work: u128,
    helper_visits: u128,
    reference_cursor: u128,
    maximum_depth: Option<usize>,
}
impl SinkFacts {
    fn note(&mut self, event: &StepEventC) {
        self.events = self.events.checked_add(1).expect("normal sink event count overflow");
        self.work = self.work.checked_add(u128::from(event.work_delta))
            .expect("normal sink work overflow");
        self.helper_work = self.helper_work.checked_add(u128::from(event.helper_work_delta))
            .expect("normal sink helper work overflow");
        self.helper_visits = self.helper_visits.checked_add(u128::from(event.helper_visits_delta))
            .expect("normal sink helper visits overflow");
        self.reference_cursor = self.reference_cursor.checked_add(u128::from(event.reference_cursor_delta))
            .expect("normal sink reference cursor overflow");
        let current = event.logical_depth_before.max(event.logical_depth_after);
        self.maximum_depth = Some(self.maximum_depth.map_or(current, |previous| previous.max(current)));
    }
}

// Actual public raw observation ABI, scalar variants only. This is test-side
// decoding of an already owner-bound observation, not another evaluator,
// production codec, admission decision or manufactured fallback value.
fn halted_scalar(observation: &HostObservation<'_>) -> crate::Value {
    let value = match *observation.image.outcome {
        OutcomeC::Halted { value } => value,
        other => panic!("normal operator run did not halt: {other:?}"),
    };
    match value {
        RefC::Const(index) => match observation.admission.view.constants.get(index) {
            Some(ConstRow::U128 { marker_free, n }) => {
                assert!(*marker_free);
                crate::Value::UInt(crate::UInt(*n))
            }
            Some(ConstRow::Bool { marker_free, b }) => {
                assert!(*marker_free);
                crate::Value::Bool(*b)
            }
            _ => panic!("normal operator halted with a non-scalar constant reference"),
        },
        RefC::Data(index) => match observation.image.data.get(index) {
            Some(DataRow::U128 { n }) => crate::Value::UInt(crate::UInt(*n)),
            Some(DataRow::Bool { b }) => crate::Value::Bool(*b),
            _ => panic!("normal operator halted with a non-scalar data reference"),
        },
    }
}

fn assert_actual_pipeline(case: &str, source_text: &str, scenario_text: &str,
    emit_name: &str, covered: &[CoreOp])
{
    let source = crate::parse(source_text).expect("frozen public normal source did not parse");
    let scenario: crate::Scenario = serde_json::from_str(scenario_text)
        .expect("public normal scenario did not decode");
    for op in covered {
        assert!(source.expressions.iter().any(|expression|
            matches!(expression, crate::Expr::Binary { op: actual, .. } if actual == op)),
            "declared public source operator missing: {op:?}");
    }
    let action = &source.actions[0];
    let inputs: &BTreeMap<String, crate::Value> = &scenario.inputs;
    let mut sink = SinkFacts::default();
    let AccountedScalarRun { run_result, event_accounting } =
        run_scalar_action_with_event_accounting(&source, 0, inputs, &operator_config(),
            &mut |event| sink.note(event));
    let actual = run_result.unwrap_or_else(|_| panic!("{case}: actual owner pipeline refused"));
    let observation = actual.observe()
        .unwrap_or_else(|_| panic!("{case}: actual bound observation unavailable"));
    let report = actual.resource_report()
        .unwrap_or_else(|_| panic!("{case}: actual resource report unavailable"));

    // Publish actual observations even if the ordinary Halted/value assertion
    // below finds runtime exhaustion/refusal. No computed work total is fixed.
    println!("{}", serde_json::to_string(&(case, &report, &event_accounting))
        .expect("actual scalar reports did not serialize"));
    let scalar = halted_scalar(&observation);
    let core = crate::simulate(&source, &action.name, &scenario)
        .expect("public Core scalar simulation failed");
    let core_value = match core.effects.as_slice() {
        [crate::Effect::Emit { name, value }] => {
            assert_eq!(name, emit_name);
            value
        }
        _ => panic!("public scalar source did not retain its one Core emit effect"),
    };
    assert_eq!(&scalar, core_value, "{case}: actual Core/B scalar differential");
    assert_eq!(actual.preparation.emit.name, emit_name);
    assert_eq!(actual.preparation.generated_function_name, action.name);
    assert_eq!(actual.preparation.inputs.iter().map(|input| input.name.as_str()).collect::<Vec<_>>(),
        action.inputs.iter().map(|input| input.name.as_str()).collect::<Vec<_>>());
    assert_eq!(event_accounting.prefix_end, PrefixEnd::ReturnedOk);
    assert_eq!(event_accounting.arithmetic_status, ArithmeticStatus::Exact);
    for (actual_total, raw_forwarded_sum) in [
        (event_accounting.event_count, sink.events),
        (event_accounting.work_delta_sum, sink.work),
        (event_accounting.helper_work_delta_sum, sink.helper_work),
        (event_accounting.helper_visits_delta_sum, sink.helper_visits),
        (event_accounting.reference_cursor_delta_sum, sink.reference_cursor),
    ] {
        assert_eq!(actual_total, CheckedTotal::Exact { value: Counter(raw_forwarded_sum) });
    }
    assert_eq!(event_accounting.maximum_observed_logical_depth, sink.maximum_depth);
    assert_eq!(actual.completed.events_forwarded, sink.events);
    assert_eq!(report.run_summary.events, Counter(actual.completed.summary.events));
    assert_eq!(report.observed.consumed_work, Counter(observation.image.consumed_work));
    assert_eq!(report.observed.remaining_work, Counter(observation.image.remaining_work));
    assert_eq!(report.observed.helper_work, Counter(observation.image.helper_work));
    assert_eq!(report.observed.helper_visits, Counter(observation.image.helper_visits));
    assert_eq!(report.observed.reference_cursor, Counter(observation.image.reference_cursor));
    assert_eq!(report.observed.logical_depth, observation.image.depth());
    assert_eq!(report.observed.helper_tasks, observation.image.task_live());
    assert_eq!(report.stored_active.data, observation.image.active.data);
    assert_eq!(report.current_slices.data, observation.image.data.len());
    assert_eq!(report.allocation_counters.data_nodes, observation.image.alloc.data_nodes);
    assert_eq!(report.supplied.admission_bounds.code_rows, operator_config().bounds.code_rows);
    assert_eq!(report.supplied.layout_capacities.occurrence_path_slots,
        operator_config().layout.capacities.occurrence_path_slots);
}

#[test]
fn ascending_operator_fixture_and_sub_eq_companion_use_actual_owner_reports() {
    assert_actual_pipeline("ascending-comparisons",
        include_str!("source_operator_fixtures/ascending-comparisons.mori"),
        include_str!("source_operator_fixtures/ascending-comparisons.scenario.json"),
        "ascending", &[CoreOp::Ne, CoreOp::Le, CoreOp::Gt, CoreOp::Ge, CoreOp::And, CoreOp::Or]);
    assert_actual_pipeline("subtract-positive",
        include_str!("source_operator_fixtures/subtract-and-compare.mori"),
        include_str!("source_operator_fixtures/subtract-positive.scenario.json"),
        "differenceIsZero", &[CoreOp::Sub, CoreOp::Eq]);
}

#[test]
fn equal_operator_fixture_and_sub_eq_companion_use_actual_owner_reports() {
    assert_actual_pipeline("equal-comparisons",
        include_str!("source_operator_fixtures/equal-comparisons.mori"),
        include_str!("source_operator_fixtures/equal-comparisons.scenario.json"),
        "equal", &[CoreOp::Ne, CoreOp::Le, CoreOp::Gt, CoreOp::Ge, CoreOp::And, CoreOp::Or]);
    assert_actual_pipeline("subtract-zero",
        include_str!("source_operator_fixtures/subtract-and-compare.mori"),
        include_str!("source_operator_fixtures/subtract-zero.scenario.json"),
        "differenceIsZero", &[CoreOp::Sub, CoreOp::Eq]);
}
