//! Ordinary API/format test SOURCE ONLY / UNRUN; no independent O is imported.
//! Engine calls below are proposed tests, not executed or newly authorized.
use super::*;
use crate::sherlock::helper_machine::core_action_consumer::{
    ConsumerConfig, ConsumerProfile, SourceHostBounds, run_scalar_action,
};
use crate::sherlock::helper_machine::shared::{LayoutCapacitiesC, LayoutSchemaC, StepEventC};
use std::collections::BTreeMap;

#[test]
fn raw_summary_counter_columns_and_u128_serialization_stay_distinct() {
    // R01's public hand-stated Summary input, not an observed engine result.
    let summary = RunSummary {
        outcome: OutcomeC::Halted {
            value: RefC::Const(0),
        },
        events: 40,
        consumed_work: 40,
        helper_work: 18,
        helper_visits: 12,
        reference_cursor: 22,
    };
    let report = Summary::from(&summary);
    assert_eq!(report.consumed_work, Counter(40));
    assert_eq!(report.helper_work, Counter(18));
    assert_eq!(report.helper_visits, Counter(12));
    assert_eq!(report.reference_cursor, Counter(22));
    let json = serde_json::to_value(report).unwrap();
    assert_eq!(json["consumed_work"], "40");
    assert_eq!(
        serde_json::to_value(Counter(u128::MAX)).unwrap(),
        u128::MAX.to_string()
    );
}

#[test]
fn snapshot_only_gaps_remain_explicit_instead_of_zero_estimates() {
    let json = serde_json::to_value(Unmeasured::from_snapshot_only()).unwrap();
    for group in json.as_object().unwrap().values() {
        assert_eq!(group["status"], "unmeasured");
        assert!(!group["reason"].as_str().unwrap().is_empty());
        assert!(!group["metrics"].as_array().unwrap().is_empty());
        assert!(group.get("value").is_none());
    }
    assert!(
        json["overhead"]["metrics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|metric| metric == "end-to-end-total")
    );
}

// Exact existing caller-specific normal configuration, reused as test input;
// this supplies no new default, cap, profile, guard or execution authority.
fn existing_normal_config() -> ConsumerConfig {
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

fn assert_real_source_report(source_text: &str, inputs: &[(&str, u128)]) {
    let source = crate::parse(source_text).unwrap();
    let inputs: BTreeMap<_, _> = inputs
        .iter()
        .map(|(name, value)| ((*name).into(), crate::Value::UInt(crate::UInt(*value))))
        .collect();
    let config = existing_normal_config();
    let mut sink = |_event: &StepEventC| {};
    let actual = run_scalar_action(&source, 0, &inputs, &config, &mut sink)
        .unwrap_or_else(|_| panic!("existing normal source did not complete"));
    let observation = actual
        .observe()
        .unwrap_or_else(|_| panic!("actual bound observation unavailable"));
    let report = actual
        .resource_report()
        .unwrap_or_else(|_| panic!("source report did not preserve owner observation"));
    assert_eq!(report.format, FORMAT);
    assert_eq!(
        report.observed.consumed_work,
        Counter(observation.image.consumed_work)
    );
    assert_eq!(
        report.observed.remaining_work,
        Counter(observation.image.remaining_work)
    );
    assert_eq!(
        report.observed.reference_cursor,
        Counter(observation.image.reference_cursor)
    );
    assert_eq!(
        report.run_summary.events,
        Counter(actual.completed.summary.events)
    );
    assert_eq!(report.observed.logical_depth, observation.image.depth());
    assert_eq!(report.observed.helper_tasks, observation.image.task_live());
    assert_eq!(report.stored_active.data, observation.image.active.data);
    assert_eq!(report.current_slices.data, observation.image.data.len());
    assert_eq!(
        report.allocation_counters.data_nodes,
        observation.image.alloc.data_nodes
    );
    assert_eq!(
        report.supplied.admission_bounds.data_nodes,
        config.bounds.data_nodes
    );
    assert_eq!(
        report.supplied.layout_capacities.value_slots,
        config.layout.capacities.value_slots
    );
    let json = serde_json::to_value(report).unwrap();
    assert_eq!(json["unmeasured"]["peaks"]["status"], "unmeasured");
    assert_eq!(
        json["unmeasured"]["financial_target"]["status"],
        "unmeasured"
    );
}

#[test]
fn actual_weighted_core_action_exposes_a_typed_resource_report() {
    assert_real_source_report(
        include_str!("../source_adapter_fixtures/weighted-total.mori"),
        &[("units", 7), ("price", 11)],
    );
}

#[test]
fn actual_threshold_core_action_exposes_a_typed_resource_report() {
    assert_real_source_report(
        include_str!("../source_adapter_fixtures/threshold-check.mori"),
        &[("observed", 8), ("threshold", 13)],
    );
}
