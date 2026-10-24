//! SOURCE_ONLY/UNRUN actual integration API checks; no independent O imported.
//! These compare fields forwarded by this one actual pipeline, not independently
//! expected scalar values, complete B states or work totals. Test-owned retention
//! and report serialization are outside owner counters and remain unmeasured.
use super::core_action_consumer::{ConsumerConfig, ConsumerProfile, SourceHostBounds};
use super::resource_event_accounting::{
    AccountedScalarRun, ArithmeticStatus, CheckedTotal, PrefixEnd,
    run_scalar_action_with_event_accounting,
};
use super::resource_report::Counter;
use super::shared::{
    BoundsC, FinanceInput, GrammarDomain, LayoutCapacitiesC, LayoutSchemaC, RunIntC, StepEventC,
};
use std::collections::BTreeMap;

// Verbatim existing normal-config input helper; no new default or cap choice.
fn normal_config() -> ConsumerConfig {
    let bounds = BoundsC {
        code_rows: 64,
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
    ConsumerConfig {
        bounds,
        layout: LayoutSchemaC {
            layout_version: (
                "moriarty".into(),
                "sherlock-helper-microstep-hostonly/1".into(),
            ),
            field_id: None,
            widths: None,
            scalar_layout_id: None,
            bounds,
            capacities: LayoutCapacitiesC {
                name_rows: 32,
                name_bytes: 256,
                occurrence_path_slots: 256,
                value_slots: 1024,
            },
            ext_capacities: None,
            boundary_capacities_present: false,
        },
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

fn assert_report_path(source_text: &str, declared: &[(&str, u128)]) {
    let source = crate::parse(source_text)
        .unwrap_or_else(|_| panic!("existing normal source did not parse"));
    let inputs: BTreeMap<String, crate::Value> = declared
        .iter()
        .map(|(name, value)| ((*name).into(), crate::Value::UInt(crate::UInt(*value))))
        .collect();
    let config = normal_config();
    let mut delivered: Vec<StepEventC> = Vec::new();
    let AccountedScalarRun {
        run_result,
        event_accounting,
    } = run_scalar_action_with_event_accounting(&source, 0, &inputs, &config, &mut |event| {
        delivered.push(event.clone())
    });
    let actual =
        run_result.unwrap_or_else(|_| panic!("actual source caller did not return its result"));
    let observation = actual
        .observe()
        .unwrap_or_else(|_| panic!("existing owner observation refused"));
    let report = actual
        .resource_report()
        .unwrap_or_else(|_| panic!("actual owner report projection refused"));
    assert_eq!(event_accounting.prefix_end, PrefixEnd::ReturnedOk);
    assert_eq!(event_accounting.arithmetic_status, ArithmeticStatus::Exact);
    assert_eq!(
        event_accounting.event_count,
        CheckedTotal::Exact {
            value: Counter(delivered.len() as u128)
        }
    );
    // API consistency against the actual separately retained sink stream;
    // these are no independent B-semantics/work expectations.
    let sum = |field: fn(&StepEventC) -> u8| {
        delivered
            .iter()
            .try_fold(0u128, |total, event| {
                total.checked_add(u128::from(field(event)))
            })
            .expect("normal input event diagnostic exceeded u128")
    };
    for (actual_total, expected_from_forwarded_stream) in [
        (event_accounting.work_delta_sum, sum(|e| e.work_delta)),
        (
            event_accounting.helper_work_delta_sum,
            sum(|e| e.helper_work_delta),
        ),
        (
            event_accounting.helper_visits_delta_sum,
            sum(|e| e.helper_visits_delta),
        ),
        (
            event_accounting.reference_cursor_delta_sum,
            sum(|e| e.reference_cursor_delta),
        ),
    ] {
        assert_eq!(
            actual_total,
            CheckedTotal::Exact {
                value: Counter(expected_from_forwarded_stream)
            }
        );
    }
    assert_eq!(
        event_accounting.maximum_observed_logical_depth,
        delivered
            .iter()
            .map(|event| event.logical_depth_before.max(event.logical_depth_after))
            .max()
    );
    assert_eq!(actual.completed.events_forwarded, delivered.len() as u128);
    assert_eq!(
        report.observed.consumed_work,
        Counter(observation.image.consumed_work)
    );
    assert_eq!(
        report.observed.remaining_work,
        Counter(observation.image.remaining_work)
    );
    assert_eq!(report.observed.logical_depth, observation.image.depth());
    assert_eq!(
        report.run_summary.events,
        Counter(actual.completed.summary.events)
    );
    assert_eq!(
        report.run_summary.helper_work,
        Counter(actual.completed.summary.helper_work)
    );
    assert_eq!(
        report.supplied.admission_bounds.data_nodes,
        config.bounds.data_nodes
    );
    assert_eq!(
        report.profile.profile_id,
        observation.admission.identity.profile_id
    );
    assert_eq!(
        actual
            .preparation
            .inputs
            .iter()
            .map(|site| site.name.as_str())
            .collect::<Vec<_>>(),
        declared.iter().map(|(name, _)| *name).collect::<Vec<_>>()
    );
    let json = serde_json::to_value((&report, &event_accounting))
        .expect("typed diagnostics did not serialize");
    assert!(json.is_array());
}

#[test]
fn weighted_source_runs_observes_and_reports_through_registered_accounting_wrapper() {
    assert_report_path(
        include_str!("../source_adapter_fixtures/weighted-total.mori"),
        &[("units", 7), ("price", 11)],
    );
}

#[test]
fn threshold_source_runs_observes_and_reports_through_registered_accounting_wrapper() {
    assert_report_path(
        include_str!("../source_adapter_fixtures/threshold-check.mori"),
        &[("observed", 8), ("threshold", 13)],
    );
}
