use super::*;
use super::super::owner_ext_source_program::{ExtDomain, ExtProgram};
use super::super::owner_ext_t3_uses::{ExtExpr, ExtFunction, ExtParamSrc, ExtValue};
use super::super::owner_ext_type_builder::{
    CtorDeclSrc, CtorFieldSrc, TypeDeclSrc, TypeExprSrc,
};

fn bounds_and_layout() -> (BoundsC, LayoutSchemaC) {
    let b = BoundsC {
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
    let l = LayoutSchemaC {
        layout_version: (
            "moriarty".to_string(),
            "sherlock-helper-microstep-hostonly/1".to_string(),
        ),
        field_id: None,
        widths: None,
        scalar_layout_id: None,
        bounds: b,
        capacities: LayoutCapacitiesC {
            name_rows: 32,
            name_bytes: 256,
            occurrence_path_slots: 256,
            value_slots: 4096,
        },
        ext_capacities: None,
        boundary_capacities_present: false,
    };
    (b, l)
}

fn token_decl() -> TypeDeclSrc {
    TypeDeclSrc {
        name: "Token".to_string(),
        parameters: vec![],
        ctors: vec![CtorDeclSrc {
            name: "MkToken".to_string(),
            fields: vec![CtorFieldSrc {
                name: "value".to_string(),
                ty: TypeExprSrc::UInt,
            }],
        }],
    }
}

fn generic_view() -> (ProgramView, BoundsC, LayoutSchemaC) {
    let (b, l) = bounds_and_layout();
    let program = ExtProgram {
        types: vec![token_decl()],
        assets: Vec::new(),
        states: Vec::new(),
        inputs: Vec::new(),
        functions: vec![ExtFunction {
            name: "id_token".to_string(),
            parameters: vec![ExtParamSrc {
                name: "tok".to_string(),
                ty: TypeExprSrc::Named {
                    name: "Token".to_string(),
                },
            }],
            return_type: TypeExprSrc::Named {
                name: "Token".to_string(),
            },
            body: ExtExpr::Var {
                name: "tok".to_string(),
            },
        }],
        entry: ExtExpr::Literal {
            value: ExtValue::Core {
                value: CoreValue::UInt {
                    n: i128_class::KIntC::InRange(0),
                },
            },
        },
    };
    match prepare_ext_program(program, ExtDomain::GENERIC_ABSENT, &b, &l, None) {
        Ok((view, _report)) => (view, b, l),
        Err(_) => panic!("prepare_ext_program should succeed for Token program"),
    }
}

fn assert_pv_err(
    res: Result<(), ProgramViewViolation>,
    expected_check: ViewCheck,
    expected_table: &str,
    expected_row: Option<usize>,
    expected_detail: &str,
) {
    match res {
        Ok(()) => panic!("expected ProgramViewViolation, got Ok(())"),
        Err(err) => {
            assert_eq!(err.check, expected_check);
            assert_eq!(err.table, expected_table);
            assert_eq!(err.row, expected_row);
            assert_eq!(err.detail, expected_detail);
        }
    }
}

#[test]
fn valid_generic_nominal_signature_stays_on_ext_route() {
    let (view, b, l) = generic_view();
    // Token is the first non-parameterized declaration: TypeIdC(4).
    assert_eq!(view.types.len(), 5);
    assert_eq!(view.parameters.len(), 1);
    assert_eq!(view.parameters[0].sort, TypeIdC(4));
    assert_eq!(view.functions[0].return_type, TypeIdC(4));
    if let Err(err) = check_program_view(&view, &b, &l) {
        panic!(
            "valid generic view refused: {:?} {:?} {:?} {:?}",
            err.check, err.table, err.row, err.detail
        );
    }
}

#[test]
fn malformed_generic_signature_with_ext_tables_stays_ext_refusal() {
    let (mut view, b, l) = generic_view();
    view.parameters[0].sort = TypeIdC(999);
    // Ext tables are present, so this must not become the dedicated
    // (V8, parameters, Some(0), "non-builtin parameter sort") refusal.
    assert_pv_err(
        check_program_view(&view, &b, &l),
        ViewCheck::V8,
        "program",
        None,
        "V4",
    );
}

#[test]
fn unused_decl_template_table_alone_is_generic_evidence() {
    let (b, l) = bounds_and_layout();
    let prog = CoreProgram {
        functions: vec![],
        entry: CoreExpr::Literal {
            value: CoreValue::UInt {
                n: i128_class::KIntC::InRange(0),
            },
        },
    };
    let mut view = match prepare_core(&prog, &b, &l) {
        Ok(v) => v,
        Err(_) => panic!("prepare_core should succeed"),
    };
    assert!(view.types.is_empty());
    view.decl_templates.push(token_decl());
    // An unused declaration table is genuine generic evidence and must
    // not be hidden as a dedicated view with trusted nominal IDs.
    assert_pv_err(
        check_program_view(&view, &b, &l),
        ViewCheck::V8,
        "program",
        None,
        "V1",
    );
}
