//! SOURCE_ONLY / UNRUN normal API wiring checks.
//!
//! Inputs and configuration are the frozen PUBLIC-NORMAL-ENVELOPE.json.
//! These tests check declaration order, owner binding and event/summary
//! consistency. They assert no computed scalar value, work total, event total
//! or independently expected B state. O/QA owns those separate comparisons.

use std::collections::BTreeMap;

use super::core_action_consumer::{
    run_scalar_action, ConsumerConfig, ConsumerProfile, SourceHostBounds,
};
use super::core_action_consumer_security::EventForwarding;
use super::raw::ConstRow;
use super::shared::{
    BoundsC, ExpressionRowC, FinanceInput, GrammarDomain, LayoutCapacitiesC, LayoutSchemaC,
    RunIntC, StepEventC,
};

const WEIGHTED: &str = include_str!("../source_adapter_fixtures/weighted-total.mori");
const THRESHOLD: &str = include_str!("../source_adapter_fixtures/threshold-check.mori");

// Caller-specific operational inputs from the security-owned public envelope.
// This test helper supplies no production default or new policy profile.
fn normal_config() -> ConsumerConfig {
    let bounds = BoundsC {
        code_rows: 64, function_rows: 4, code_child_slots: 16, parameter_slots: 8,
        pattern_rows: 8, match_plan_rows: 8, match_arm_rows: 8, type_rows: 8,
        constructor_rows: 8, constructor_field_slots: 16, max_arity: 8,
        constant_nodes: 16, constant_field_slots: 16, data_nodes: 64,
        data_field_slots: 64, environment_cells: 64, frames: 64, helper_tasks: 64,
        serialization_bytes: 65536, max_reference_work: 4096, max_helper_work: 4096,
        step_cap: 4096,
    };
    ConsumerConfig {
        bounds,
        layout: LayoutSchemaC {
            layout_version: ("moriarty".into(), "sherlock-helper-microstep-hostonly/1".into()),
            field_id: None, widths: None, scalar_layout_id: None, bounds,
            capacities: LayoutCapacitiesC {
                name_rows: 32, name_bytes: 256, occurrence_path_slots: 256, value_slots: 1024,
            },
            boundary_capacities_present: false,
        },
        fuel: RunIntC::Nat(1000),
        stack: RunIntC::Nat(32),
        profile: ConsumerProfile::SherlockHelperMicrostep1,
        grammar: GrammarDomain::DedicatedSorts,
        finance: FinanceInput::Absent,
        host: SourceHostBounds {
            max_source_actions: 4, max_source_expressions: 256, max_source_statements: 64,
            max_action_inputs: 8, max_supplied_inputs: 8, max_projection_occurrences: 512,
            max_generated_ast_nodes: 512, max_ast_nesting: 128,
            max_input_name_bytes: 64, max_total_input_name_bytes: 512,
        },
    }
}

fn assert_owner_wiring(source: &str, action: &str, declared: &[(&str, u128)]) {
    let parsed = crate::parse(source).unwrap_or_else(|_| panic!("public Core fixture did not parse"));
    let inputs: BTreeMap<_, _> = declared.iter().rev().map(|(name, value)| {
        ((*name).to_string(), crate::Value::UInt(crate::UInt(*value)))
    }).collect();
    let config = normal_config();
    let mut seen = EventForwarding::default();
    let mut sink = |_event: &StepEventC| seen.note();
    let actual = run_scalar_action(&parsed, 0, &inputs, &config, &mut sink)
        .unwrap_or_else(|_| panic!("normal source consumer did not return a completed owner run"));
    let observation = actual.observe()
        .unwrap_or_else(|_| panic!("normal owner-bound observation was refused"));

    assert_eq!(actual.preparation.action, action);
    assert_eq!(actual.preparation.generated_function_name, action);
    assert_eq!(actual.preparation.inputs.iter().map(|input| input.name.as_str()).collect::<Vec<_>>(),
        declared.iter().map(|(name, _)| *name).collect::<Vec<_>>());
    assert_eq!(actual.preparation.emit.statement_index, 2);
    assert_eq!(actual.preparation.emit.role, "value-projection; ordered emit effect remains in Core");

    assert_eq!(observation.admission.identity.profile_id, 2);
    assert_eq!(observation.admission.identity.grammar_domain, GrammarDomain::DedicatedSorts);
    assert_eq!(observation.admission.identity.finance, FinanceInput::Absent);
    assert_eq!(observation.admission.bounds, &config.bounds);
    assert_eq!(observation.image.initial_fuel, 1000);
    assert_eq!(observation.image.stack_limit, 32);
    assert_eq!(*observation.image.outcome, actual.completed.summary.outcome);
    assert_eq!(observation.image.consumed_work, actual.completed.summary.consumed_work);
    assert_eq!(observation.image.helper_work, actual.completed.summary.helper_work);
    assert_eq!(observation.image.helper_visits, actual.completed.summary.helper_visits);
    assert_eq!(observation.image.reference_cursor, actual.completed.summary.reference_cursor);
    assert_eq!(actual.completed.events_forwarded, seen.forwarded);
    assert_eq!(actual.completed.summary.events, seen.forwarded);

    // Inspect the actual admitted function and literal call, not a reconstructed AST.
    let view = observation.admission.view;
    assert_eq!(view.functions.len(), 1);
    let function = &view.functions[0];
    assert_eq!(view.names[function.function_name], action);
    let parameters = &view.parameters[function.params.start..function.params.start + function.params.len];
    assert_eq!(parameters.iter().map(|parameter| view.names[parameter.binder].as_str()).collect::<Vec<_>>(),
        declared.iter().map(|(name, _)| *name).collect::<Vec<_>>());
    let (function_name, args) = match &view.expressions[view.entry] {
        ExpressionRowC::Call { function_name, args } => (*function_name, *args),
        _ => panic!("admitted entry is not the explicit generated call"),
    };
    assert_eq!(view.names[function_name], action);
    assert_eq!(args.len, declared.len());
    for (position, (_, input)) in declared.iter().enumerate() {
        let expression = view.children[args.start + position];
        let constant = match &view.expressions[expression] {
            ExpressionRowC::Literal { constant } => *constant,
            _ => panic!("admitted entry argument is not an input literal"),
        };
        match &view.constants[constant] {
            ConstRow::U128 { marker_free, n } => {
                assert!(*marker_free);
                assert_eq!(n, input);
            }
            _ => panic!("admitted input literal is not UInt"),
        }
    }
}

#[test]
fn weighted_source_reaches_the_actual_owner_bound_shape() {
    assert_owner_wiring(WEIGHTED, "quote", &[("units", 7), ("price", 11)]);
}

#[test]
fn threshold_source_reaches_the_actual_owner_bound_shape() {
    assert_owner_wiring(THRESHOLD, "inspect", &[("observed", 8), ("threshold", 13)]);
}
