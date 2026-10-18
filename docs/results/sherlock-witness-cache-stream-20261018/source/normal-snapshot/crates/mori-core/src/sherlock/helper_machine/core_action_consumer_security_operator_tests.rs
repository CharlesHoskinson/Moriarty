// Security-owned unit tests for the Core/1 scalar operator occurrence precount
// and generated-AST guard invariants in `core_action_consumer_security`.
//
// SOURCE_ONLY / UNRUN. These tests exercise only `validate_config`,
// `preflight_source`, `prepare_scalar_action`, `check_generated`, `count_ast`,
// and `check_combined_generated_ast_nodes`. They do not invoke Machine A,
// the Core evaluator, or the B engine, and they assert no expected runtime
// values, states, or work.

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::sherlock::helper_machine::core_action_consumer_security::{
    check_combined_generated_ast_nodes, check_generated, preflight_source, ConsumerConfig,
    ConsumerError, ConsumerProfile, HostGuardError, HostMeasure, SourceHostBounds,
};
use crate::sherlock::helper_machine::shared::{
    BoundsC, FinanceInput, GrammarDomain, LayoutCapacitiesC, LayoutSchemaC, RunIntC,
};
use crate::sherlock::{Expr as SherlockExpr, Value as SherlockValue};
use crate::sherlock_source::prepare_scalar_action;
use crate::{
    Action, BinaryOp, Expr, InputDecl, Program, Statement, Type, UInt, Value, LANGUAGE, SEMANTICS,
};

fn proposed_operator_config() -> ConsumerConfig {
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
        boundary_capacities_present: false,
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

fn two_input_map(left: u128, right: u128) -> BTreeMap<String, Value> {
    let mut map = BTreeMap::new();
    map.insert("left".to_string(), Value::UInt(UInt(left)));
    map.insert("right".to_string(), Value::UInt(UInt(right)));
    map
}

fn single_binary_program(op: BinaryOp, input_ty: Type) -> Program {
    Program {
        language: LANGUAGE.to_string(),
        semantics: SEMANTICS.to_string(),
        name: "SingleBinary".to_string(),
        states: Vec::new(),
        assets: Vec::new(),
        expressions: vec![
            Expr::Name { name: "left".to_string() },
            Expr::Name { name: "right".to_string() },
            Expr::Binary { op, lhs: 0, rhs: 1 },
            Expr::Name { name: "out".to_string() },
        ],
        actions: vec![Action {
            name: "inspect".to_string(),
            inputs: vec![
                InputDecl { name: "left".to_string(), ty: input_ty },
                InputDecl { name: "right".to_string(), ty: input_ty },
            ],
            statements: vec![
                Statement::Let { name: "out".to_string(), value: 2 },
                Statement::Emit { name: "obs".to_string(), value: 3 },
            ],
        }],
    }
}

#[test]
fn precount_matches_generated_occurrences_across_all_11_binary_operators() {
    let cases: [(BinaryOp, Type, u128); 11] = [
        (BinaryOp::Add, Type::UInt, 1 + 3 + 1),
        (BinaryOp::Sub, Type::UInt, 1 + 3 + 1),
        (BinaryOp::Mul, Type::UInt, 1 + 3 + 1),
        (BinaryOp::Eq, Type::UInt, 1 + 3 + 1),
        (BinaryOp::Lt, Type::UInt, 1 + 3 + 1),
        (BinaryOp::Ne, Type::UInt, 1 + 4 + 1),
        (BinaryOp::Ge, Type::UInt, 1 + 4 + 1),
        (BinaryOp::Gt, Type::UInt, 1 + 7 + 1),
        (BinaryOp::Le, Type::UInt, 1 + 8 + 1),
        (BinaryOp::And, Type::Bool, 1 + 8 + 1),
        (BinaryOp::Or, Type::Bool, 1 + 8 + 1),
    ];
    let config = proposed_operator_config();
    for (op, ty, expected_occurrences) in cases {
        let program = single_binary_program(op, ty);
        let mut supplied = BTreeMap::new();
        let sample = match ty {
            Type::UInt => Value::UInt(UInt(5)),
            Type::Bool => Value::Bool(true),
        };
        supplied.insert("left".to_string(), sample.clone());
        supplied.insert("right".to_string(), sample);

        let Ok(preflight) = preflight_source(&program, 0, &supplied, &config) else {
            panic!("preflight should succeed for single binary program");
        };
        assert_eq!(preflight.projected_occurrences, expected_occurrences);

        let mapped = prepare_scalar_action(&program, 0)
            .expect("prepare_scalar_action should succeed for single binary program");
        assert_eq!(mapped.report.occurrences.len() as u128, expected_occurrences);

        let Ok((body_nodes, _)) = check_generated(&mapped, &preflight, &config.host) else {
            panic!("check_generated should accept exact precount");
        };
        assert_eq!(body_nodes as u128, expected_occurrences);

        let mut boundary_config = config.clone();
        boundary_config.host.max_projection_occurrences = (expected_occurrences - 1) as usize;
        match preflight_source(&program, 0, &supplied, &boundary_config) {
            Err(ConsumerError::HostGuard(HostGuardError::Exceeded {
                measure: HostMeasure::ProjectionOccurrences,
                required: Some(req),
                bound,
            })) => {
                assert_eq!(req, expected_occurrences);
                assert_eq!(bound, expected_occurrences - 1);
            }
            other => panic!("expected ProjectionOccurrences Exceeded, got {:?}", other.is_err()),
        }
    }
}

#[test]
fn precount_catches_lowered_dag_amplification_before_allocation() {
    // Exercises only Exceeded; the CountOverflow path is defensive and not exercised here.
    // Build a validated flat DAG of shared `Le` nodes: e0 = Name(left), e1 = Name(right),
    // e2 = Le(e0, e1) [size 8], and for k >= 3: ek = And(e_{k-1}, e_{k-1}) [size 6 + 2 * size(e_{k-1})].
    // At depth 40, source expression count is only 41 (3 + 38 And), well within max_source_expressions = 256,
    // and depth is 40 <= 64, so crate::validate passes.
    let mut expressions = vec![
        Expr::Name { name: "left".to_string() },
        Expr::Name { name: "right".to_string() },
        Expr::Binary { op: BinaryOp::Le, lhs: 0, rhs: 1 },
    ];
    for prev in 2..40u16 {
        expressions.push(Expr::Binary { op: BinaryOp::And, lhs: prev, rhs: prev });
    }
    let root_id = (expressions.len() - 1) as u16;
    let program = Program {
        language: LANGUAGE.to_string(),
        semantics: SEMANTICS.to_string(),
        name: "DagAmplification".to_string(),
        states: Vec::new(),
        assets: Vec::new(),
        expressions,
        actions: vec![Action {
            name: "inspect".to_string(),
            inputs: vec![
                InputDecl { name: "left".to_string(), ty: Type::UInt },
                InputDecl { name: "right".to_string(), ty: Type::UInt },
            ],
            statements: vec![Statement::Emit { name: "obs".to_string(), value: root_id }],
        }],
    };
    let supplied = two_input_map(7, 11);
    let config = proposed_operator_config();
    match preflight_source(&program, 0, &supplied, &config) {
        Err(ConsumerError::HostGuard(HostGuardError::Exceeded {
            measure: HostMeasure::ProjectionOccurrences,
            required: Some(req),
            bound: 512,
        })) => {
            // Closed form: S(2) = 8; S(k) = 6 + 2*S(k-1) => S(2 + d) = 14 * 2^d - 6.
            let expected = 14u128 * (1u128 << 38) - 6;
            assert_eq!(req, expected);
        }
        other => panic!("expected preflight Exceeded on lowered DAG, got {:?}", other.is_err()),
    }
}

#[test]
fn public_revision02_ascending_and_equal_shapes_match_exact_static_derivations() {
    let ascending = Program {
        language: LANGUAGE.to_string(),
        semantics: SEMANTICS.to_string(),
        name: "AscendingComparisons".to_string(),
        states: Vec::new(),
        assets: Vec::new(),
        expressions: vec![
            Expr::Name { name: "left".to_string() },
            Expr::Name { name: "right".to_string() },
            Expr::Binary { op: BinaryOp::Ne, lhs: 0, rhs: 1 },
            Expr::Name { name: "left".to_string() },
            Expr::Name { name: "right".to_string() },
            Expr::Binary { op: BinaryOp::Le, lhs: 3, rhs: 4 },
            Expr::Name { name: "right".to_string() },
            Expr::Name { name: "left".to_string() },
            Expr::Binary { op: BinaryOp::Gt, lhs: 6, rhs: 7 },
            Expr::Name { name: "left".to_string() },
            Expr::Name { name: "right".to_string() },
            Expr::Binary { op: BinaryOp::Ge, lhs: 9, rhs: 10 },
            Expr::Name { name: "different".to_string() },
            Expr::Name { name: "low".to_string() },
            Expr::Binary { op: BinaryOp::And, lhs: 12, rhs: 13 },
            Expr::Name { name: "high".to_string() },
            Expr::Name { name: "inclusive".to_string() },
            Expr::Binary { op: BinaryOp::Or, lhs: 15, rhs: 16 },
            Expr::Name { name: "both".to_string() },
            Expr::Name { name: "either".to_string() },
            Expr::Binary { op: BinaryOp::And, lhs: 18, rhs: 19 },
            Expr::Name { name: "result".to_string() },
        ],
        actions: vec![Action {
            name: "inspect".to_string(),
            inputs: vec![
                InputDecl { name: "left".to_string(), ty: Type::UInt },
                InputDecl { name: "right".to_string(), ty: Type::UInt },
            ],
            statements: vec![
                Statement::Let { name: "different".to_string(), value: 2 },
                Statement::Let { name: "low".to_string(), value: 5 },
                Statement::Let { name: "high".to_string(), value: 8 },
                Statement::Let { name: "inclusive".to_string(), value: 11 },
                Statement::Let { name: "both".to_string(), value: 14 },
                Statement::Let { name: "either".to_string(), value: 17 },
                Statement::Let { name: "result".to_string(), value: 20 },
                Statement::Emit { name: "ascending".to_string(), value: 21 },
            ],
        }],
    };

    let equal = Program {
        language: LANGUAGE.to_string(),
        semantics: SEMANTICS.to_string(),
        name: "EqualComparisons".to_string(),
        states: Vec::new(),
        assets: Vec::new(),
        expressions: vec![
            Expr::Name { name: "left".to_string() },
            Expr::Name { name: "right".to_string() },
            Expr::Binary { op: BinaryOp::Ne, lhs: 0, rhs: 1 },
            Expr::Name { name: "left".to_string() },
            Expr::Name { name: "right".to_string() },
            Expr::Binary { op: BinaryOp::Le, lhs: 3, rhs: 4 },
            Expr::Name { name: "left".to_string() },
            Expr::Name { name: "right".to_string() },
            Expr::Binary { op: BinaryOp::Gt, lhs: 6, rhs: 7 },
            Expr::Name { name: "left".to_string() },
            Expr::Name { name: "right".to_string() },
            Expr::Binary { op: BinaryOp::Ge, lhs: 9, rhs: 10 },
            Expr::Name { name: "different".to_string() },
            Expr::Name { name: "low".to_string() },
            Expr::Binary { op: BinaryOp::And, lhs: 12, rhs: 13 },
            Expr::Name { name: "high".to_string() },
            Expr::Name { name: "inclusive".to_string() },
            Expr::Binary { op: BinaryOp::Or, lhs: 15, rhs: 16 },
            Expr::Name { name: "both".to_string() },
            Expr::Name { name: "high".to_string() },
            Expr::Name { name: "inclusive".to_string() },
            Expr::Binary { op: BinaryOp::And, lhs: 19, rhs: 20 },
            Expr::Binary { op: BinaryOp::Or, lhs: 18, rhs: 21 },
            Expr::Name { name: "result".to_string() },
        ],
        actions: vec![Action {
            name: "inspect".to_string(),
            inputs: vec![
                InputDecl { name: "left".to_string(), ty: Type::UInt },
                InputDecl { name: "right".to_string(), ty: Type::UInt },
            ],
            statements: vec![
                Statement::Let { name: "different".to_string(), value: 2 },
                Statement::Let { name: "low".to_string(), value: 5 },
                Statement::Let { name: "high".to_string(), value: 8 },
                Statement::Let { name: "inclusive".to_string(), value: 11 },
                Statement::Let { name: "both".to_string(), value: 14 },
                Statement::Let { name: "either".to_string(), value: 17 },
                Statement::Let { name: "result".to_string(), value: 22 },
                Statement::Emit { name: "equal".to_string(), value: 23 },
            ],
        }],
    };

    let config = proposed_operator_config();

    // Ascending verification: 55 body occurrences/nodes, 313 body path slots, 58 combined nodes, 315 total path slots.
    let asc_inputs = two_input_map(7, 11);
    let Ok(asc_preflight) = preflight_source(&ascending, 0, &asc_inputs, &config) else {
        panic!("ascending preflight should succeed");
    };
    assert_eq!(asc_preflight.projected_occurrences, 55);
    let asc_mapped = prepare_scalar_action(&ascending, 0).unwrap();
    assert_eq!(asc_mapped.report.occurrences.len(), 55);
    let asc_body_path_slots: usize = asc_mapped
        .report
        .occurrences
        .iter()
        .map(|site| site.body_path.len())
        .sum();
    assert_eq!(asc_body_path_slots, 313);
    let Ok((asc_body_nodes, asc_nesting)) = check_generated(&asc_mapped, &asc_preflight, &config.host) else {
        panic!("ascending check_generated should succeed");
    };
    assert_eq!(asc_body_nodes, 55);
    assert_eq!(asc_nesting, 11);
    let asc_entry = Arc::new(SherlockExpr::Call(
        "inspect".to_string(),
        vec![
            Arc::new(SherlockExpr::Literal(SherlockValue::UInt(7))),
            Arc::new(SherlockExpr::Literal(SherlockValue::UInt(11))),
        ],
    ));
    let Ok(asc_total_nodes) =
        check_combined_generated_ast_nodes(asc_body_nodes, &asc_entry, &config.host)
    else {
        panic!("ascending combined generated node check should succeed");
    };
    assert_eq!(asc_total_nodes, 58);

    // Equal verification: 62 body occurrences/nodes, 391 body path slots, 65 combined nodes, 393 total path slots.
    let eq_inputs = two_input_map(13, 13);
    let Ok(eq_preflight) = preflight_source(&equal, 0, &eq_inputs, &config) else {
        panic!("equal preflight should succeed");
    };
    assert_eq!(eq_preflight.projected_occurrences, 62);
    let eq_mapped = prepare_scalar_action(&equal, 0).unwrap();
    assert_eq!(eq_mapped.report.occurrences.len(), 62);
    let eq_body_path_slots: usize = eq_mapped
        .report
        .occurrences
        .iter()
        .map(|site| site.body_path.len())
        .sum();
    assert_eq!(eq_body_path_slots, 391);
    let Ok((eq_body_nodes, eq_nesting)) = check_generated(&eq_mapped, &eq_preflight, &config.host) else {
        panic!("equal check_generated should succeed");
    };
    assert_eq!(eq_body_nodes, 62);
    assert_eq!(eq_nesting, 13);
    let eq_entry = Arc::new(SherlockExpr::Call(
        "inspect".to_string(),
        vec![
            Arc::new(SherlockExpr::Literal(SherlockValue::UInt(13))),
            Arc::new(SherlockExpr::Literal(SherlockValue::UInt(13))),
        ],
    ));
    let Ok(eq_total_nodes) =
        check_combined_generated_ast_nodes(eq_body_nodes, &eq_entry, &config.host)
    else {
        panic!("equal combined generated node check should succeed");
    };
    assert_eq!(eq_total_nodes, 65);
}
