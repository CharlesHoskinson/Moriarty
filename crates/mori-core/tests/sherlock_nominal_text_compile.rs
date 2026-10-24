//! Ordinary source-to-admission tests. No runtime/native operation is performed.
//! Author table expectations are distinct from independent semantic evidence.
#[path = "support/nominal_text_compile_expected.rs"]
mod expected;

use mori_core::sherlock::helper_machine::shared::{
    owner_ext_source_program::ExtDomain, owner_ext_type_builder::ExtTypeBuildError,
};
use mori_core::sherlock::helper_machine::{complete_observation, shared::*};
use mori_core::sherlock_compile_artifact::ext_compile::{
    ExtCompileConfig, ExtCompileFailure, NominalCompileFailure, compile_nominal_host,
};
use mori_core::sherlock_nominal_source::{
    DraftBudgetExceeded, DraftBudgetResource, DraftError, DraftParseBudget,
};

const SOURCE: &str = include_str!("fixtures/nominal-text-compile/wrapped-countdown.mori");

fn parser_budget() -> DraftParseBudget {
    DraftParseBudget {
        max_source_bytes: 4096,
        max_tokens: 1024,
        max_functions: 8,
        max_parameters_per_function: 8,
        max_total_parameters: 32,
        max_ast_occurrences: 256,
        max_parser_recursion_depth: 64,
        max_structural_ast_depth: 32,
    }
}

// Explicit positive inputs, using the existing generic-C6 SAFE configuration.
// These values are test inputs, not defaults or an admission policy.
fn configuration() -> ExtCompileConfig {
    let bounds = BoundsC {
        code_rows: 256,
        function_rows: 8,
        code_child_slots: 256,
        parameter_slots: 16,
        pattern_rows: 128,
        match_plan_rows: 64,
        match_arm_rows: 32,
        type_rows: 64,
        constructor_rows: 64,
        constructor_field_slots: 128,
        max_arity: 4,
        constant_nodes: 128,
        constant_field_slots: 128,
        data_nodes: 64,
        data_field_slots: 128,
        environment_cells: 64,
        frames: 64,
        helper_tasks: 64,
        serialization_bytes: 1048576,
        max_reference_work: 100000,
        max_helper_work: 100000,
        step_cap: 1,
    };
    ExtCompileConfig {
        bounds,
        layout: LayoutSchemaC {
            layout_version: ("independent-public-generic-runtime".into(), "1".into()),
            field_id: None,
            widths: None,
            scalar_layout_id: None,
            bounds,
            capacities: LayoutCapacitiesC {
                name_rows: 64,
                name_bytes: 1024,
                occurrence_path_slots: 1024,
                value_slots: 4096,
            },
            ext_capacities: Some(ExtCapacitiesC {
                plan_case_slots: 256,
                field_path_slots: 512,
                pattern_child_slots: 256,
                input_rows: 0,
                asset_rows: 0,
                state_rows: 0,
                account_rows: 0,
                liability_rows: 0,
                await_site_rows: 0,
                await_binder_slots: 0,
                balance_cells: 0,
                journal_rows: 0,
                pending_op_rows: 0,
                observations_per_invocation: 0,
                max_lifetime_work: 0,
                resume_rows: 0,
            }),
            boundary_capacities_present: false,
        },
        domain: ExtDomain::GENERIC_ABSENT,
        fuel: RunIntC::Nat(4096),
        stack: RunIntC::Nat(64),
    }
}

#[test]
fn bounded_text_compiles_generic_recursive_body_to_complete_admitted_tables() {
    let artifact = compile_nominal_host(SOURCE, parser_budget(), configuration(), None)
        .unwrap_or_else(|_| panic!("normal bounded source must reach actual admission"));
    let observation = artifact
        .observe()
        .unwrap_or_else(|_| panic!("actual bound genesis"));
    let snapshot = complete_observation::copy_observation(observation);
    assert_eq!(snapshot.admission.view, expected::wrapped_countdown());
    assert_eq!(snapshot.image.control, ControlC::Eval { expr: 0 });
    assert_eq!(snapshot.image.control_origin, 0);
    assert_eq!(snapshot.image.outcome, OutcomeC::Running);
    assert_eq!(snapshot.image.initial_fuel, 4096);
    assert_eq!(snapshot.image.remaining_work, 4096);
    assert_eq!(snapshot.image.stack_limit, 64);
    assert_eq!(
        (
            snapshot.image.consumed_work,
            snapshot.image.helper_work,
            snapshot.image.helper_visits,
            snapshot.image.reference_cursor
        ),
        (0, 0, 0, 0)
    );
    let report = artifact
        .report()
        .unwrap_or_else(|_| panic!("actual compile report"));
    assert!(!report.evaluation_performed);
    assert_eq!(
        complete_observation::copy_observation(report.owner),
        snapshot
    );
}

#[test]
fn bounded_text_preserves_parser_failure_payload() {
    let mut budget = parser_budget();
    budget.max_source_bytes = SOURCE.len() - 1;
    match compile_nominal_host(SOURCE, budget, configuration(), None) {
        Err(NominalCompileFailure::Parse(error)) => assert_eq!(
            error,
            DraftError::Budget(DraftBudgetExceeded {
                resource: DraftBudgetResource::SourceBytes,
                limit: SOURCE.len() - 1,
                observed: SOURCE.len()
            })
        ),
        _ => panic!("preserve the existing bounded parser payload before owner compilation"),
    }
}

#[test]
fn bounded_text_preserves_owner_admission_failure_payload() {
    let mut config = configuration();
    config.layout.bounds.code_rows -= 1;
    match compile_nominal_host(SOURCE, parser_budget(), config, None) {
        Err(NominalCompileFailure::Compile(ExtCompileFailure::Preparation(
            ExtTypeBuildError::CoreTooling(Tooling::AdmissionRejected {
                reason,
                detail,
                locator,
            }),
        ))) => {
            assert_eq!(reason, AdmissionReason::LayoutBoundsMismatch);
            assert_eq!(detail, "layout.bounds != bounds");
            assert_eq!(
                locator,
                Some(LocatorC::Field {
                    name: "layout.bounds".into()
                })
            );
        }
        _ => panic!("preserve the existing owner admission payload"),
    }
}
