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
        ext_capacities: None,
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
        Err(Tooling::AdmissionRejected {
            reason,
            detail,
            locator,
        }) => {
            panic!("{msg}: AdmissionRejected({reason:?}, {detail:?}, {locator:?})")
        }
        Err(Tooling::CapacityNotAdmitted {
            kind,
            needed,
            limit,
            locator,
        }) => {
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
                return_type: SortTag::UInt.into(),
                body: 5,
            },
            FunctionRowC {
                function_name: 0,
                params: SliceC { start: 0, len: 1 },
                return_type: SortTag::UInt.into(),
                body: 6,
            },
            FunctionRowC {
                function_name: 2,
                params: SliceC { start: 0, len: 0 },
                return_type: SortTag::UInt.into(),
                body: 15,
            },
        ]
    );
    assert_eq!(
        view.parameters,
        vec![ParameterRowC {
            binder: 1,
            sort: SortTag::UInt.into(),
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
        sort: SortTag::UInt.into(),
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
        sort: SortTag::UInt.into(),
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
        sort: SortTag::UInt.into(),
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
        return_type: SortTag::UInt.into(),
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

fn assert_pv_ok(res: Result<(), ProgramViewViolation>) {
    if let Err(err) = res {
        panic!(
            "expected check_program_view Ok(()), got Err({:?}, {:?}, {:?}, {:?})",
            err.check, err.table, err.row, err.detail
        );
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
        Ok(()) => panic!(
            "expected ProgramViewViolation({expected_check:?}, {expected_table:?}, {expected_row:?}, {expected_detail:?}), got Ok(())"
        ),
        Err(err) => {
            assert_eq!(err.check, expected_check);
            assert_eq!(err.table, expected_table);
            assert_eq!(err.row, expected_row);
            assert_eq!(err.detail, expected_detail);
        }
    }
}

fn all_builtin_sorts_program() -> CoreProgram {
    CoreProgram {
        functions: vec![
            CoreFunction {
                name: "fn_0_uint".to_string(),
                parameters: vec![
                    CoreParameter {
                        name: "p0_uint".to_string(),
                        sort: SortTag::UInt,
                    },
                    CoreParameter {
                        name: "p1_bool".to_string(),
                        sort: SortTag::Bool,
                    },
                ],
                return_type: SortTag::UInt,
                body: CoreExpr::If {
                    cond: Box::new(var_expr("p1_bool")),
                    then_expr: Box::new(var_expr("p0_uint")),
                    else_expr: Box::new(uint_lit(10)),
                },
            },
            CoreFunction {
                name: "fn_1_bool".to_string(),
                parameters: vec![
                    CoreParameter {
                        name: "p2_list".to_string(),
                        sort: SortTag::List,
                    },
                    CoreParameter {
                        name: "p3_tree".to_string(),
                        sort: SortTag::Tree,
                    },
                ],
                return_type: SortTag::Bool,
                body: CoreExpr::If {
                    cond: Box::new(CoreExpr::Literal {
                        value: CoreValue::Bool { b: true },
                    }),
                    then_expr: Box::new(var_expr("p2_list")),
                    else_expr: Box::new(var_expr("p3_tree")),
                },
            },
            CoreFunction {
                name: "fn_2_list".to_string(),
                parameters: vec![],
                return_type: SortTag::List,
                body: CoreExpr::Literal {
                    value: CoreValue::List {
                        elements: vec![
                            i128_class::KIntC::InRange(1),
                            i128_class::KIntC::InRange(2),
                        ],
                    },
                },
            },
            CoreFunction {
                name: "fn_3_tree".to_string(),
                parameters: vec![
                    CoreParameter {
                        name: "p4_tree".to_string(),
                        sort: SortTag::Tree,
                    },
                    CoreParameter {
                        name: "p5_list".to_string(),
                        sort: SortTag::List,
                    },
                    CoreParameter {
                        name: "p6_bool".to_string(),
                        sort: SortTag::Bool,
                    },
                    CoreParameter {
                        name: "p7_uint".to_string(),
                        sort: SortTag::UInt,
                    },
                ],
                return_type: SortTag::Tree,
                body: CoreExpr::If {
                    cond: Box::new(var_expr("p6_bool")),
                    then_expr: Box::new(var_expr("p4_tree")),
                    else_expr: Box::new(CoreExpr::If {
                        cond: Box::new(CoreExpr::Binary {
                            op: BinaryOp::Eq,
                            left: Box::new(var_expr("p7_uint")),
                            right: Box::new(uint_lit(0)),
                        }),
                        then_expr: Box::new(var_expr("p5_list")),
                        else_expr: Box::new(CoreExpr::Literal {
                            value: CoreValue::Tree {
                                tree: CoreTree::Branch {
                                    left: Box::new(CoreTree::Leaf {
                                        n: i128_class::KIntC::InRange(3),
                                    }),
                                    right: Box::new(CoreTree::Leaf {
                                        n: i128_class::KIntC::InRange(4),
                                    }),
                                },
                            },
                        }),
                    }),
                },
            },
        ],
        entry: uint_lit(0),
    }
}

#[test]
fn v8_accepts_all_builtin_parameter_and_return_sorts_roundtrip() {
    let (b, l) = default_bounds_and_layout();
    let prog = all_builtin_sorts_program();
    let view = expect_ok(
        prepare_core(&prog, &b, &l),
        "prepare_core should succeed for all builtin sorts",
    );

    let expected_param_sorts = [
        TypeIdC::UInt,
        TypeIdC::Bool,
        TypeIdC::List,
        TypeIdC::Tree,
        TypeIdC::Tree,
        TypeIdC::List,
        TypeIdC::Bool,
        TypeIdC::UInt,
    ];
    assert_eq!(view.parameters.len(), expected_param_sorts.len());
    for (pid, &expected_sort) in expected_param_sorts.iter().enumerate() {
        assert_eq!(view.parameters[pid].sort, expected_sort);
    }

    let expected_return_types = [TypeIdC::UInt, TypeIdC::Bool, TypeIdC::List, TypeIdC::Tree];
    assert_eq!(view.functions.len(), expected_return_types.len());
    for (fi, &expected_ret) in expected_return_types.iter().enumerate() {
        assert_eq!(view.functions[fi].return_type, expected_ret);
    }

    assert_pv_ok(check_program_view(&view, &b, &l));
    let _admitted = expect_ok(
        admit(&prog, &b, &l),
        "admit should accept all builtin parameter and return sorts",
    );

    // Builtin-to-builtin mutations on the view also round-trip through V8 decode + prepare_core.
    for &builtin_id in &[TypeIdC::UInt, TypeIdC::Bool, TypeIdC::List, TypeIdC::Tree] {
        for pid in 0..view.parameters.len() {
            let mut v_param = unwrap_ok(prepare_core(&prog, &b, &l));
            v_param.parameters[pid].sort = builtin_id;
            assert_pv_ok(check_program_view(&v_param, &b, &l));
        }
        for fi in 0..view.functions.len() {
            let mut v_ret = unwrap_ok(prepare_core(&prog, &b, &l));
            v_ret.functions[fi].return_type = builtin_id;
            assert_pv_ok(check_program_view(&v_ret, &b, &l));
        }
    }
}

#[test]
fn v8_rejects_nominal_parameter_sorts_at_exact_parameter_row() {
    let (b, l) = default_bounds_and_layout();
    let prog = all_builtin_sorts_program();
    let nominal_ids = [
        TypeIdC(4),
        TypeIdC(5),
        TypeIdC(7),
        TypeIdC(64),
        TypeIdC(usize::MAX),
    ];

    for pid in 0..8 {
        for &nominal_id in &nominal_ids {
            let mut view = unwrap_ok(prepare_core(&prog, &b, &l));
            view.parameters[pid].sort = nominal_id;
            assert_pv_err(
                check_program_view(&view, &b, &l),
                ViewCheck::V8,
                "parameters",
                Some(pid),
                "non-builtin parameter sort",
            );
        }
    }
}

#[test]
fn v8_rejects_nominal_function_return_types_at_exact_function_row() {
    let (b, l) = default_bounds_and_layout();
    let prog = all_builtin_sorts_program();
    let nominal_ids = [
        TypeIdC(4),
        TypeIdC(5),
        TypeIdC(8),
        TypeIdC(128),
        TypeIdC(usize::MAX),
    ];

    for fi in 0..4 {
        for &nominal_id in &nominal_ids {
            let mut view = unwrap_ok(prepare_core(&prog, &b, &l));
            view.functions[fi].return_type = nominal_id;
            assert_pv_err(
                check_program_view(&view, &b, &l),
                ViewCheck::V8,
                "functions",
                Some(fi),
                "non-builtin return type",
            );
        }
    }
}

#[test]
fn v8_enforces_intra_function_body_parameter_and_return_type_precedence() {
    let (b, l) = default_bounds_and_layout();
    let prog = all_builtin_sorts_program();

    // 1. Within function 0: parameter refusal at pid=0 fires before function 0 return_type refusal.
    let mut v_fn0_both = unwrap_ok(prepare_core(&prog, &b, &l));
    v_fn0_both.parameters[0].sort = TypeIdC(4);
    v_fn0_both.functions[0].return_type = TypeIdC(5);
    assert_pv_err(
        check_program_view(&v_fn0_both, &b, &l),
        ViewCheck::V8,
        "parameters",
        Some(0),
        "non-builtin parameter sort",
    );

    // 2. Within function 3 (params 4..8): multiple nominal parameters refuse at lowest pid first.
    let mut v_fn3_multi_param = unwrap_ok(prepare_core(&prog, &b, &l));
    v_fn3_multi_param.parameters[5].sort = TypeIdC(9);
    v_fn3_multi_param.parameters[7].sort = TypeIdC(4);
    v_fn3_multi_param.functions[3].return_type = TypeIdC(6);
    assert_pv_err(
        check_program_view(&v_fn3_multi_param, &b, &l),
        ViewCheck::V8,
        "parameters",
        Some(5),
        "non-builtin parameter sort",
    );

    // 3. Within function 3: last parameter pid=7 still fires before function 3 return_type.
    let mut v_fn3_last_param = unwrap_ok(prepare_core(&prog, &b, &l));
    v_fn3_last_param.parameters[7].sort = TypeIdC(4);
    v_fn3_last_param.functions[3].return_type = TypeIdC(5);
    assert_pv_err(
        check_program_view(&v_fn3_last_param, &b, &l),
        ViewCheck::V8,
        "parameters",
        Some(7),
        "non-builtin parameter sort",
    );

    // 4. Within function 0: decoded[f.body].take() ("function body taken") fires before parameter/return_type conversion.
    let single_name_prog = CoreProgram {
        functions: vec![CoreFunction {
            name: "x".to_string(),
            parameters: vec![CoreParameter {
                name: "x".to_string(),
                sort: SortTag::UInt,
            }],
            return_type: SortTag::UInt,
            body: var_expr("x"),
        }],
        entry: var_expr("x"),
    };
    let mut v_body_taken = unwrap_ok(prepare_core(&single_name_prog, &b, &l));
    v_body_taken.expressions.pop();
    v_body_taken.occurrences.pop();
    v_body_taken.functions[0].body = 0;
    v_body_taken.parameters[0].sort = TypeIdC(4);
    v_body_taken.functions[0].return_type = TypeIdC(5);
    assert_pv_err(
        check_program_view(&v_body_taken, &b, &l),
        ViewCheck::V8,
        "functions",
        None,
        "function body taken",
    );
}

#[test]
fn v8_enforces_inter_function_order_and_v1_through_v8_stage_precedence() {
    let (b, l) = default_bounds_and_layout();
    let prog = all_builtin_sorts_program();

    // Inter-function order: function 0 return_type refusal fires before function 1 parameter refusal.
    let mut v_fn0_ret_before_fn1_param = unwrap_ok(prepare_core(&prog, &b, &l));
    v_fn0_ret_before_fn1_param.functions[0].return_type = TypeIdC(4);
    v_fn0_ret_before_fn1_param.parameters[2].sort = TypeIdC(5);
    v_fn0_ret_before_fn1_param.functions[1].return_type = TypeIdC(6);
    assert_pv_err(
        check_program_view(&v_fn0_ret_before_fn1_param, &b, &l),
        ViewCheck::V8,
        "functions",
        Some(0),
        "non-builtin return type",
    );

    // Inter-function order: function 0 parameter pid=1 refusal fires before function 1 parameter pid=2 refusal.
    let mut v_fn0_param_before_fn1_param = unwrap_ok(prepare_core(&prog, &b, &l));
    v_fn0_param_before_fn1_param.parameters[1].sort = TypeIdC(4);
    v_fn0_param_before_fn1_param.parameters[2].sort = TypeIdC(5);
    assert_pv_err(
        check_program_view(&v_fn0_param_before_fn1_param, &b, &l),
        ViewCheck::V8,
        "parameters",
        Some(1),
        "non-builtin parameter sort",
    );

    // Inter-function order: empty-param function 2 return_type refusal fires before function 3 parameter pid=4 refusal.
    let mut v_fn2_ret_before_fn3_param = unwrap_ok(prepare_core(&prog, &b, &l));
    v_fn2_ret_before_fn3_param.functions[2].return_type = TypeIdC(4);
    v_fn2_ret_before_fn3_param.parameters[4].sort = TypeIdC(5);
    assert_pv_err(
        check_program_view(&v_fn2_ret_before_fn3_param, &b, &l),
        ViewCheck::V8,
        "functions",
        Some(2),
        "non-builtin return type",
    );

    // V1 precedes V8 parameter/return_type refusal.
    let mut v_v1 = unwrap_ok(prepare_core(&prog, &b, &l));
    v_v1.entry = 1;
    v_v1.parameters[0].sort = TypeIdC(4);
    v_v1.functions[0].return_type = TypeIdC(5);
    assert_pv_err(
        check_program_view(&v_v1, &b, &l),
        ViewCheck::V1,
        "header",
        None,
        "entry != 0",
    );

    // V2 precedes V8 parameter/return_type refusal.
    let mut v_v2 = unwrap_ok(prepare_core(&prog, &b, &l));
    v_v2.expressions[0] = ExpressionRowC::Var {
        binder: v_v2.names.len(),
    };
    v_v2.parameters[0].sort = TypeIdC(4);
    v_v2.functions[0].return_type = TypeIdC(5);
    assert_pv_err(
        check_program_view(&v_v2, &b, &l),
        ViewCheck::V2,
        "expressions",
        Some(0),
        "binder out of range",
    );

    // V3 precedes V8 parameter/return_type refusal.
    let mut v_v3 = unwrap_ok(prepare_core(&prog, &b, &l));
    v_v3.occurrences[0].owner_is_function = true;
    v_v3.parameters[0].sort = TypeIdC(4);
    v_v3.functions[0].return_type = TypeIdC(5);
    assert_pv_err(
        check_program_view(&v_v3, &b, &l),
        ViewCheck::V3,
        "occurrences",
        Some(0),
        "owner mismatch",
    );

    // V4 precedes V8 parameter/return_type refusal.
    let mut v_v4 = unwrap_ok(prepare_core(&prog, &b, &l));
    v_v4.parameters[0].binder = v_v4.names.len();
    v_v4.parameters[0].sort = TypeIdC(4);
    v_v4.functions[0].return_type = TypeIdC(5);
    assert_pv_err(
        check_program_view(&v_v4, &b, &l),
        ViewCheck::V4,
        "parameters",
        Some(0),
        "binder",
    );

    // V5 precedes V8 parameter/return_type refusal.
    let mut v_v5 = unwrap_ok(prepare_core(&prog, &b, &l));
    v_v5.patterns.push(PatternRowC::PNil);
    v_v5.parameters[0].sort = TypeIdC(4);
    v_v5.functions[0].return_type = TypeIdC(5);
    assert_pv_err(
        check_program_view(&v_v5, &b, &l),
        ViewCheck::V5,
        "patterns",
        None,
        "count != arms",
    );

    // V6 precedes V8 parameter/return_type refusal.
    let mut v_v6 = unwrap_ok(prepare_core(&prog, &b, &l));
    let first_composite_const = v_v6
        .constants
        .iter()
        .position(|r| const_fields_of(r).len > 0)
        .expect("fixture has composite constants");
    v_v6.constant_fields.clear();
    v_v6.parameters[0].sort = TypeIdC(4);
    v_v6.functions[0].return_type = TypeIdC(5);
    assert_pv_err(
        check_program_view(&v_v6, &b, &l),
        ViewCheck::V6,
        "constants",
        Some(first_composite_const),
        "field slice",
    );

    // V7 precedes V8 parameter/return_type refusal.
    let mut v_v7 = unwrap_ok(prepare_core(&prog, &b, &l));
    v_v7.names.swap(0, 1);
    v_v7.parameters[0].sort = TypeIdC(4);
    v_v7.functions[0].return_type = TypeIdC(5);
    assert_pv_err(
        check_program_view(&v_v7, &b, &l),
        ViewCheck::V7,
        "names",
        Some(1),
        "not ascending",
    );

    // V8 expression/constant decode precedes V8 parameter/return_type conversion,
    // which in turn precedes V8 re-prepare/views_equal.
    let dag_prog = CoreProgram {
        functions: vec![CoreFunction {
            name: "f".to_string(),
            parameters: vec![CoreParameter {
                name: "p".to_string(),
                sort: SortTag::UInt,
            }],
            return_type: SortTag::UInt,
            body: var_expr("p"),
        }],
        entry: CoreExpr::Literal {
            value: CoreValue::Tree {
                tree: CoreTree::Branch {
                    left: Box::new(CoreTree::Branch {
                        left: Box::new(CoreTree::Leaf {
                            n: i128_class::KIntC::InRange(1),
                        }),
                        right: Box::new(CoreTree::Leaf {
                            n: i128_class::KIntC::InRange(2),
                        }),
                    }),
                    right: Box::new(CoreTree::Leaf {
                        n: i128_class::KIntC::InRange(3),
                    }),
                },
            },
        },
    };
    let mut v_dag = unwrap_ok(prepare_core(&dag_prog, &b, &l));
    let root_const = v_dag.constants.len() - 1;
    let root_fs = const_fields_of(&v_dag.constants[root_const]);
    assert_eq!(root_fs.len, 2);
    // Share the left Branch subtree in both fields of root Branch: V1..V7 remain valid,
    // active constants.len() is 8, but unfolding visits 11 constant nodes.
    v_dag.constant_fields[root_fs.start + 1] = v_dag.constant_fields[root_fs.start];
    v_dag.parameters[0].sort = TypeIdC(4);
    v_dag.functions[0].return_type = TypeIdC(5);

    let mut b_const_cap = b;
    b_const_cap.constant_nodes = v_dag.constants.len();
    let mut l_const_cap = l.clone();
    l_const_cap.bounds = b_const_cap;
    assert_pv_err(
        check_program_view(&v_dag, &b_const_cap, &l_const_cap),
        ViewCheck::V8,
        "constants",
        Some(0),
        "constant expansion exceeds bounds.constantNodes",
    );

    // When constant_nodes allows unfolding (16 >= 11), V8 parameter refusal fires before views_equal.
    assert_pv_err(
        check_program_view(&v_dag, &b, &l),
        ViewCheck::V8,
        "parameters",
        Some(0),
        "non-builtin parameter sort",
    );

    // When parameter and return sorts are restored to builtin, V8 proceeds to views_equal and refuses the shared DAG view.
    v_dag.parameters[0].sort = TypeIdC::UInt;
    v_dag.functions[0].return_type = TypeIdC::UInt;
    assert_pv_err(
        check_program_view(&v_dag, &b, &l),
        ViewCheck::V8,
        "program",
        None,
        "view != prepare_core(decode(view))",
    );
}
