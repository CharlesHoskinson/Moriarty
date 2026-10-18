use super::*;

fn default_bounds_and_layout() -> (BoundsC, LayoutSchemaC) {
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
        boundary_capacities_present: false,
    };
    (b, l)
}

fn uint_lit(n: u128) -> CoreExpr {
    CoreExpr::Literal {
        value: CoreValue::UInt {
            n: i128_class::KIntC::InRange(n),
        },
    }
}

fn var_expr(name: &str) -> CoreExpr {
    CoreExpr::Var {
        name: name.to_string(),
    }
}

fn provenance_twins_program() -> CoreProgram {
    CoreProgram {
        functions: vec![
            CoreFunction {
                name: "unused_before".to_string(),
                parameters: vec![],
                return_type: SortTag::UInt,
                body: uint_lit(71),
            },
            CoreFunction {
                name: "down".to_string(),
                parameters: vec![CoreParameter {
                    name: "n".to_string(),
                    sort: SortTag::UInt,
                }],
                return_type: SortTag::UInt,
                body: CoreExpr::If {
                    cond: Box::new(CoreExpr::Binary {
                        op: BinaryOp::Eq,
                        left: Box::new(var_expr("n")),
                        right: Box::new(uint_lit(0)),
                    }),
                    then_expr: Box::new(uint_lit(1)),
                    else_expr: Box::new(CoreExpr::Call {
                        name: "down".to_string(),
                        args: vec![CoreExpr::Binary {
                            op: BinaryOp::Sub,
                            left: Box::new(var_expr("n")),
                            right: Box::new(uint_lit(1)),
                        }],
                    }),
                },
            },
            CoreFunction {
                name: "unused_after".to_string(),
                parameters: vec![],
                return_type: SortTag::UInt,
                body: uint_lit(93),
            },
        ],
        entry: CoreExpr::Binary {
            op: BinaryOp::Add,
            left: Box::new(CoreExpr::Call {
                name: "down".to_string(),
                args: vec![uint_lit(1)],
            }),
            right: Box::new(CoreExpr::Call {
                name: "down".to_string(),
                args: vec![uint_lit(1)],
            }),
        },
    }
}

fn program_with_arities(arities: &[usize]) -> CoreProgram {
    let mut functions = Vec::with_capacity(arities.len());
    for (fi, &arity) in arities.iter().enumerate() {
        let mut parameters = Vec::with_capacity(arity);
        let mut body = uint_lit((fi as u128) + 1);
        for pi in 0..arity {
            let pname = format!("p_{fi}_{pi}");
            parameters.push(CoreParameter {
                name: pname.clone(),
                sort: SortTag::UInt,
            });
            body = CoreExpr::Binary {
                op: BinaryOp::Add,
                left: Box::new(body),
                right: Box::new(var_expr(&pname)),
            };
        }
        functions.push(CoreFunction {
            name: format!("fn_{fi}"),
            parameters,
            return_type: SortTag::UInt,
            body,
        });
    }
    CoreProgram {
        functions,
        entry: uint_lit(0),
    }
}

fn expect_ok<T>(res: Result<T, Tooling>, msg: &str) -> T {
    match res {
        Ok(v) => v,
        Err(Tooling::AdmissionRejected { reason, detail, locator }) => {
            panic!("{msg}: AdmissionRejected({reason:?}, {detail:?}, {locator:?})")
        }
        Err(Tooling::CapacityNotAdmitted { kind, needed, limit, locator }) => {
            panic!("{msg}: CapacityNotAdmitted({kind:?}, {needed:?}, {limit}, {locator:?})")
        }
        Err(Tooling::UnsupportedFeature { feature, locator }) => {
            panic!("{msg}: UnsupportedFeature({feature:?}, {locator:?})")
        }
    }
}

fn unwrap_ok<T>(res: Result<T, Tooling>) -> T {
    expect_ok(res, "unexpected Tooling error")
}

#[test]
fn provenance_twins_zero_one_zero_params_admits_and_preserves_declaration_order() {
    let (b, l) = default_bounds_and_layout();
    let prog = provenance_twins_program();
    let view = expect_ok(prepare_core(&prog, &b, &l), "prepare_core should succeed");
    assert_eq!(
        view.names,
        vec!["down", "n", "unused_after", "unused_before"]
    );
    assert_eq!(
        view.functions,
        vec![
            FunctionRowC {
                function_name: 3,
                params: SliceC { start: 0, len: 0 },
                return_type: SortTag::UInt,
                body: 5,
            },
            FunctionRowC {
                function_name: 0,
                params: SliceC { start: 0, len: 1 },
                return_type: SortTag::UInt,
                body: 6,
            },
            FunctionRowC {
                function_name: 2,
                params: SliceC { start: 0, len: 0 },
                return_type: SortTag::UInt,
                body: 15,
            },
        ]
    );
    assert_eq!(
        view.parameters,
        vec![ParameterRowC {
            binder: 1,
            sort: SortTag::UInt,
        }]
    );
    assert!(check_program_view(&view, &b, &l).is_ok());
    let admitted = expect_ok(
        admit(&prog, &b, &l),
        "admit should accept [0, 1, 0] parameter slices",
    );
    let img = expect_ok(
        admit_run(
            &admitted,
            &RunIntC::Nat(4096),
            &RunIntC::Nat(64),
            &FinanceInput::Absent,
        ),
        "admit_run should succeed",
    );
    assert_eq!(img.initial_fuel, 4096);
    assert_eq!(img.stack_limit, 64);
}

#[test]
fn empty_and_nonempty_parameter_boundary_patterns_admit() {
    let (b, l) = default_bounds_and_layout();

    // All zero-parameter functions: [0, 0, 0]
    let prog_all_empty = program_with_arities(&[0, 0, 0]);
    let view_all_empty = unwrap_ok(prepare_core(&prog_all_empty, &b, &l));
    assert!(view_all_empty.parameters.is_empty());
    for f in &view_all_empty.functions {
        assert_eq!(f.params, SliceC { start: 0, len: 0 });
    }
    assert!(check_program_view(&view_all_empty, &b, &l).is_ok());
    assert!(admit(&prog_all_empty, &b, &l).is_ok());

    // Alternating non-empty and empty: [1, 0, 2, 0]
    let prog_alt = program_with_arities(&[1, 0, 2, 0]);
    let view_alt = unwrap_ok(prepare_core(&prog_alt, &b, &l));
    assert_eq!(view_alt.parameters.len(), 3);
    assert_eq!(view_alt.functions[0].params, SliceC { start: 0, len: 1 });
    assert_eq!(view_alt.functions[1].params, SliceC { start: 0, len: 0 });
    assert_eq!(view_alt.functions[2].params, SliceC { start: 1, len: 2 });
    assert_eq!(view_alt.functions[3].params, SliceC { start: 0, len: 0 });
    assert!(check_program_view(&view_alt, &b, &l).is_ok());
    assert!(admit(&prog_alt, &b, &l).is_ok());

    // Leading empty then multiple non-empty: [0, 2, 1]
    let prog_lead = program_with_arities(&[0, 2, 1]);
    let view_lead = unwrap_ok(prepare_core(&prog_lead, &b, &l));
    assert_eq!(view_lead.parameters.len(), 3);
    assert_eq!(view_lead.functions[0].params, SliceC { start: 0, len: 0 });
    assert_eq!(view_lead.functions[1].params, SliceC { start: 0, len: 2 });
    assert_eq!(view_lead.functions[2].params, SliceC { start: 2, len: 1 });
    assert!(check_program_view(&view_lead, &b, &l).is_ok());
    assert!(admit(&prog_lead, &b, &l).is_ok());
}

#[test]
fn v4_rejects_noncanonical_empty_param_slices() {
    let (b, l) = default_bounds_and_layout();
    let prog = provenance_twins_program();

    // Trailing empty function with start == expect_start (1) instead of canonical 0
    let mut v_trailing = unwrap_ok(prepare_core(&prog, &b, &l));
    v_trailing.functions[2].params = SliceC { start: 1, len: 0 };
    let err_trailing = check_program_view(&v_trailing, &b, &l).unwrap_err();
    assert_eq!(err_trailing.check, ViewCheck::V4);
    assert_eq!(err_trailing.table, "functions");
    assert_eq!(err_trailing.row, Some(2));
    assert_eq!(err_trailing.detail, "params not contiguous");

    // Leading empty function with non-zero start
    let mut v_leading = unwrap_ok(prepare_core(&prog, &b, &l));
    v_leading.functions[0].params = SliceC { start: 1, len: 0 };
    let err_leading = check_program_view(&v_leading, &b, &l).unwrap_err();
    assert_eq!(err_leading.check, ViewCheck::V4);
    assert_eq!(err_leading.table, "functions");
    assert_eq!(err_leading.row, Some(0));
    assert_eq!(err_leading.detail, "params not contiguous");
}

#[test]
fn v4_rejects_nonempty_gaps_overlaps_out_of_range_and_trailing_parameters() {
    let (b, l) = default_bounds_and_layout();
    let prog = program_with_arities(&[1, 0, 2, 0]);

    // Overlap: function 2 starts at 0 instead of cumulative expect_start 1
    let mut v_overlap = unwrap_ok(prepare_core(&prog, &b, &l));
    v_overlap.functions[2].params = SliceC { start: 0, len: 2 };
    let err_overlap = check_program_view(&v_overlap, &b, &l).unwrap_err();
    assert_eq!(err_overlap.check, ViewCheck::V4);
    assert_eq!(err_overlap.table, "functions");
    assert_eq!(err_overlap.row, Some(2));
    assert_eq!(err_overlap.detail, "params not contiguous");

    // Gap: function 2 starts at 2 instead of cumulative expect_start 1
    let mut v_gap = unwrap_ok(prepare_core(&prog, &b, &l));
    v_gap.parameters.push(ParameterRowC {
        binder: 0,
        sort: SortTag::UInt,
    });
    v_gap.functions[2].params = SliceC { start: 2, len: 2 };
    let err_gap = check_program_view(&v_gap, &b, &l).unwrap_err();
    assert_eq!(err_gap.check, ViewCheck::V4);
    assert_eq!(err_gap.table, "functions");
    assert_eq!(err_gap.row, Some(2));
    assert_eq!(err_gap.detail, "params not contiguous");

    // Out-of-range slice end (start + len > parameters.len())
    let mut v_oob = unwrap_ok(prepare_core(&prog, &b, &l));
    v_oob.functions[2].params = SliceC { start: 1, len: 3 };
    let err_oob = check_program_view(&v_oob, &b, &l).unwrap_err();
    assert_eq!(err_oob.check, ViewCheck::V4);
    assert_eq!(err_oob.table, "functions");
    assert_eq!(err_oob.row, Some(2));
    assert_eq!(err_oob.detail, "params not contiguous");

    // Arithmetic overflow in slice_inside (start + len overflows usize)
    let mut v_ovf = unwrap_ok(prepare_core(&prog, &b, &l));
    v_ovf.functions[2].params = SliceC {
        start: usize::MAX,
        len: 1,
    };
    let err_ovf = check_program_view(&v_ovf, &b, &l).unwrap_err();
    assert_eq!(err_ovf.check, ViewCheck::V4);
    assert_eq!(err_ovf.table, "functions");
    assert_eq!(err_ovf.row, Some(2));
    assert_eq!(err_ovf.detail, "params not contiguous");

    // Trailing unowned parameter after [1, 0, 2, 0]
    let mut v_trail = unwrap_ok(prepare_core(&prog, &b, &l));
    v_trail.parameters.push(ParameterRowC {
        binder: 0,
        sort: SortTag::UInt,
    });
    let err_trail = check_program_view(&v_trail, &b, &l).unwrap_err();
    assert_eq!(err_trail.check, ViewCheck::V4);
    assert_eq!(err_trail.table, "parameters");
    assert_eq!(err_trail.row, None);
    assert_eq!(err_trail.detail, "trailing parameters");

    // Trailing unowned parameter when all functions have 0 parameters
    let prog_empty = program_with_arities(&[0, 0]);
    let mut v_empty_trail = unwrap_ok(prepare_core(&prog_empty, &b, &l));
    v_empty_trail.parameters.push(ParameterRowC {
        binder: 0,
        sort: SortTag::UInt,
    });
    let err_empty_trail = check_program_view(&v_empty_trail, &b, &l).unwrap_err();
    assert_eq!(err_empty_trail.check, ViewCheck::V4);
    assert_eq!(err_empty_trail.table, "parameters");
    assert_eq!(err_empty_trail.row, None);
    assert_eq!(err_empty_trail.detail, "trailing parameters");
}

#[test]
fn v4_rejects_invalid_function_name_body_root_and_parameter_binder_and_preserves_c5_caps() {
    let (b, l) = default_bounds_and_layout();
    let prog = provenance_twins_program();

    // V4 function_name out of range
    let mut v_name = unwrap_ok(prepare_core(&prog, &b, &l));
    v_name.functions[1].function_name = v_name.names.len();
    let err_name = check_program_view(&v_name, &b, &l).unwrap_err();
    assert_eq!(err_name.check, ViewCheck::V4);
    assert_eq!(err_name.table, "functions");
    assert_eq!(err_name.row, Some(1));
    assert_eq!(err_name.detail, "name");

    // V4 parameter binder out of range
    let mut v_binder = unwrap_ok(prepare_core(&prog, &b, &l));
    v_binder.parameters[0].binder = v_binder.names.len();
    let err_binder = check_program_view(&v_binder, &b, &l).unwrap_err();
    assert_eq!(err_binder.check, ViewCheck::V4);
    assert_eq!(err_binder.table, "parameters");
    assert_eq!(err_binder.row, Some(0));
    assert_eq!(err_binder.detail, "binder");

    // V4 body not function root (pointing to a non-root child expression)
    let entry_let_prog = CoreProgram {
        functions: vec![],
        entry: CoreExpr::Let {
            name: "f".to_string(),
            value: Box::new(uint_lit(1)),
            body: Box::new(var_expr("f")),
        },
    };
    let mut v_body = unwrap_ok(prepare_core(&entry_let_prog, &b, &l));
    v_body.functions.push(FunctionRowC {
        function_name: 0,
        params: SliceC::empty(),
        return_type: SortTag::UInt,
        body: 1,
    });
    let err_body = check_program_view(&v_body, &b, &l).unwrap_err();
    assert_eq!(err_body.check, ViewCheck::V4);
    assert_eq!(err_body.table, "functions");
    assert_eq!(err_body.row, Some(0));
    assert_eq!(err_body.detail, "body not function root");

    // C5 capacity checks: function_rows, max_arity, parameter_slots
    let mut b_fn_cap = b;
    b_fn_cap.function_rows = 2;
    let mut l_fn_cap = l.clone();
    l_fn_cap.bounds = b_fn_cap;
    assert!(matches!(
        prepare_core(&prog, &b_fn_cap, &l_fn_cap),
        Err(Tooling::CapacityNotAdmitted {
            kind: CapacityKind::FunctionRows,
            needed: NeededNatC::Exact(3),
            limit: 2,
            locator: None,
        })
    ));

    let prog_arity3 = program_with_arities(&[3]);
    let mut b_arity_cap = b;
    b_arity_cap.max_arity = 2;
    let mut l_arity_cap = l.clone();
    l_arity_cap.bounds = b_arity_cap;
    assert!(matches!(
        prepare_core(&prog_arity3, &b_arity_cap, &l_arity_cap),
        Err(Tooling::CapacityNotAdmitted {
            kind: CapacityKind::MaxArity,
            needed: NeededNatC::Exact(3),
            limit: 2,
            locator: None,
        })
    ));

    let mut b_param_cap = b;
    b_param_cap.parameter_slots = 1;
    let mut l_param_cap = l.clone();
    l_param_cap.bounds = b_param_cap;
    assert!(matches!(
        prepare_core(&prog_arity3, &b_param_cap, &l_param_cap),
        Err(Tooling::CapacityNotAdmitted {
            kind: CapacityKind::ParameterSlots,
            needed: NeededNatC::Exact(2),
            limit: 1,
            locator: None,
        })
    ));
}
