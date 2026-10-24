//! Explicit external regression; Root selects this exact --test source.
//! One retained dedicated A case through the real owner, complete observation,
//! witness and reference projection path, plus explicit typed refusals for
//! generic nominal forms admitted by the actual public compile_ext_host.
//! Owner-authored expectations only: no populated oracle, generic projection,
//! full-B trace or static-to-runtime correspondence claim.
use a::helper_machine::shared::i128_class::KIntC;
use a::helper_machine::shared::owner_ext_source_program::{ExtDomain, ExtProgram};
use a::helper_machine::shared::owner_ext_t3_uses::{ExtArm, ExtExpr, ExtPattern, ExtValue};
use a::helper_machine::shared::owner_ext_type_builder::{
    CtorDeclSrc, CtorFieldSrc, TypeDeclSrc, TypeExprSrc,
};
use a::helper_machine::shared::owner_host::host_owner_engine;
use a::helper_machine::shared::*;
use a::helper_machine::{
    complete_observation as co, reference_projection as rp, source_input_bridge as bridge,
    witness_plan as wp,
};
use mori_core::sherlock as a;
use mori_core::sherlock_compile_artifact::ext_compile::{ExtCompileConfig, compile_ext_host};
use std::sync::Arc;

fn ex(e: a::Expr) -> a::ExprRef {
    Arc::new(e)
}
fn nominal(name: &str) -> TypeExprSrc {
    TypeExprSrc::Named { name: name.into() }
}
fn uint(n: u128) -> ExtExpr {
    ExtExpr::Literal {
        value: ExtValue::Core {
            value: CoreValue::UInt {
                n: KIntC::InRange(n),
            },
        },
    }
}
fn named<'a>(fields: &'a [wp::NamedValue], name: &str) -> &'a wp::SlotValue {
    &fields
        .iter()
        .find(|f| f.field == name)
        .unwrap_or_else(|| panic!("decoded field {name}"))
        .value
}
fn outcome_value(decoded: &wp::DecodedSnapshot) -> &wp::DecodedValue {
    let wp::SlotValue::Payload(outcome) = named(&decoded.fields, "outcome") else {
        panic!("decoded outcome payload")
    };
    let wp::SlotValue::Value(value) = named(&outcome.fields, "value") else {
        panic!("decoded halted value")
    };
    value
}

// Same input-only dedicated layout/run conventions as the retained 035 consumers.
fn dedicated_layout() -> (BoundsC, LayoutSchemaC) {
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
    (
        bounds,
        LayoutSchemaC {
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
                value_slots: 4096,
            },
            ext_capacities: None,
            boundary_capacities_present: false,
        },
    )
}
fn list_sum() -> Arc<a::Program> {
    let var = |n: &str| ex(a::Expr::Var(n.into()));
    let recurse = ex(a::Expr::Call("sum_list".into(), vec![var("t")]));
    let body = ex(a::Expr::Match(
        var("xs"),
        vec![
            a::Arm {
                pattern: a::Pattern::Nil,
                body: ex(a::Expr::Literal(a::Value::UInt(0))),
            },
            a::Arm {
                pattern: a::Pattern::Cons("h".into(), "t".into()),
                body: ex(a::Expr::Binary(a::BinaryOp::Add, var("h"), recurse)),
            },
        ],
    ));
    Arc::new(a::Program {
        functions: vec![a::Function {
            name: "sum_list".into(),
            parameters: vec![a::Parameter {
                name: "xs".into(),
                ty: a::Type::List,
            }],
            return_type: a::Type::UInt,
            body,
        }],
        entry: ex(a::Expr::Call(
            "sum_list".into(),
            vec![ex(a::Expr::Literal(a::Value::List(vec![2, 5])))],
        )),
    })
}

#[test]
fn dedicated_list_sum_reference_projection_retained() {
    let p = list_sum();
    let budget = bridge::SourceCopyBudget {
        max_functions: 4,
        max_parameters_per_function: 8,
        max_total_parameters: 8,
        max_expr_nodes: 128,
        max_copy_depth: 32,
        max_match_arms: 16,
        max_name_bytes: 512,
        max_list_elements: 64,
        max_tree_nodes: 64,
        max_tree_depth: 32,
    };
    let core = bridge::from_program_bounded(&p, &budget)
        .unwrap_or_else(|_| panic!("bounded dedicated source copy"));
    let (bounds, schema) = dedicated_layout();
    let owner = host_owner_engine();
    let admitted = owner
        .admit(&core, &bounds, &schema)
        .unwrap_or_else(|_| panic!("dedicated admission"));
    let mut image = owner
        .admit_run(
            &admitted,
            &RunIntC::Nat(4096),
            &RunIntC::Nat(64),
            &FinanceInput::Absent,
        )
        .unwrap_or_else(|_| panic!("dedicated genesis"));
    let initial =
        co::capture(&admitted, &image).unwrap_or_else(|_| panic!("dedicated initial capture"));
    assert!(
        initial.admission.view.expressions.iter().all(|e| !matches!(
            e,
            co::ExpressionSnapshot::Construct { .. }
                | co::ExpressionSnapshot::Match { plan: Some(_), .. }
        )),
        "dedicated rows only"
    );
    let mut projection = rp::ReferencePrefixProjection::new(p, &initial, 4096)
        .expect("dedicated initial projection");
    loop {
        match co::record_step(&admitted, &mut image) {
            Ok(co::ObservedStep::Event(record)) => {
                projection
                    .push(&record)
                    .expect("dedicated saved record projection");
            }
            Ok(co::ObservedStep::Terminal { before, after }) => {
                assert_eq!(before, after);
                break;
            }
            Err(_) => panic!("dedicated owner step"),
        }
    }
    assert!(!projection.records().is_empty());
    assert_eq!(
        projection
            .finish()
            .expect("no trailing helper interval")
            .outcome,
        a::Outcome::Done(a::Value::UInt(7))
    );
    let last = co::capture(&admitted, &image).unwrap_or_else(|_| panic!("dedicated final capture"));
    let witness = wp::materialize(&last).expect("dedicated final witness");
    assert_eq!(
        rp::reference_value(outcome_value(&witness.evaluation.decoded)),
        Ok(a::Value::UInt(7))
    );
}

// Exactly the existing SAFE public run input; no release defaults.
fn generic_config() -> ExtCompileConfig {
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
fn run_generic(program: ExtProgram) -> (co::HostSnapshot, co::HostSnapshot) {
    let artifact = compile_ext_host(program, generic_config(), None)
        .unwrap_or_else(|_| panic!("actual public generic compile"));
    let (admitted, mut image, _) = artifact.into_parts();
    let initial =
        co::capture(&admitted, &image).unwrap_or_else(|_| panic!("generic genesis capture"));
    loop {
        match co::record_step(&admitted, &mut image) {
            Ok(co::ObservedStep::Event(_)) => {}
            Ok(co::ObservedStep::Terminal { .. }) => break,
            Err(_) => panic!("generic owner step"),
        }
    }
    let last = co::capture(&admitted, &image).unwrap_or_else(|_| panic!("generic final capture"));
    assert!(
        matches!(last.image.outcome, OutcomeC::Halted { .. }),
        "generic run halts with a value"
    );
    (initial, last)
}
fn source_refusal(initial: &co::HostSnapshot) -> rp::ProjectionGap {
    // No A syntax exists for a generic program; any caller source is refused first.
    let unrelated = Arc::new(a::Program {
        functions: vec![],
        entry: ex(a::Expr::Literal(a::Value::UInt(0))),
    });
    let gap = rp::ReferenceSource::new(unrelated.clone(), initial)
        .err()
        .expect("generic source refusal");
    let prefix = rp::ReferencePrefixProjection::new(unrelated, initial, 4096)
        .err()
        .expect("generic prefix refusal");
    assert_eq!(gap, prefix);
    gap
}
fn pair_program() -> ExtProgram {
    let fields = vec![
        CtorFieldSrc {
            name: "first".into(),
            ty: TypeExprSrc::UInt,
        },
        CtorFieldSrc {
            name: "second".into(),
            ty: TypeExprSrc::UInt,
        },
    ];
    ExtProgram {
        types: vec![TypeDeclSrc {
            name: "Pair".into(),
            parameters: vec![],
            ctors: vec![CtorDeclSrc {
                name: "P".into(),
                fields,
            }],
        }],
        assets: vec![],
        states: vec![],
        inputs: vec![],
        functions: vec![],
        entry: ExtExpr::Construct {
            ty: nominal("Pair"),
            ctor: "P".into(),
            fields: vec![uint(1), uint(2)],
        },
    }
}
fn flag_program() -> ExtProgram {
    let ctor = |name: &str| ExtPattern::Ctor {
        ty: nominal("Flag"),
        ctor: name.into(),
        fields: vec![],
    };
    let on = ExtExpr::Literal {
        value: ExtValue::Constructor {
            ty: nominal("Flag"),
            ctor: "On".into(),
            fields: vec![],
        },
    };
    ExtProgram {
        types: vec![TypeDeclSrc {
            name: "Flag".into(),
            parameters: vec![],
            ctors: vec![
                CtorDeclSrc {
                    name: "On".into(),
                    fields: vec![],
                },
                CtorDeclSrc {
                    name: "Off".into(),
                    fields: vec![],
                },
            ],
        }],
        assets: vec![],
        states: vec![],
        inputs: vec![],
        functions: vec![],
        entry: ExtExpr::Match {
            scrutinee: Box::new(on),
            arms: vec![
                ExtArm {
                    pattern: ctor("On"),
                    body: uint(1),
                },
                ExtArm {
                    pattern: ctor("Off"),
                    body: uint(0),
                },
            ],
        },
    }
}

#[test]
fn generic_construct_refused_with_retained_nominal_identity() {
    let (initial, last) = run_generic(pair_program());
    let (row, type_id, ctor_id) = match source_refusal(&initial) {
        rp::ProjectionGap::UnsupportedGeneric(rp::GenericGap::ConstructExpression {
            row,
            type_id,
            ctor_id,
        }) => (row, type_id, ctor_id),
        other => panic!("expected explicit ConstructExpression refusal, got {other:?}"),
    };
    let view = &initial.admission.view;
    assert!(
        matches!(view.expressions[row],co::ExpressionSnapshot::Construct{type_id:t,ctor_id:c,..} if t==type_id&&c==ctor_id)
    );
    assert!(view.expressions[..row].iter().all(|e| !matches!(
        e,
        co::ExpressionSnapshot::Construct { .. }
            | co::ExpressionSnapshot::Match { plan: Some(_), .. }
    )));
    assert!(type_id.0 >= 4, "user nominal type, not a dedicated sort");
    let witness = wp::materialize(&last).expect("generic final witness");
    let value = outcome_value(&witness.evaluation.decoded);
    let wp::ValueNode::Constructor {
        type_id: vt,
        ctor_id: vc,
        fields,
    } = &value.nodes[value.root]
    else {
        panic!("nominal final value")
    };
    assert_eq!((*vt, *vc, fields.len()), (type_id, ctor_id, 2));
    for (&i, n) in fields.iter().zip([1u128, 2]) {
        assert_eq!(value.nodes[i], wp::ValueNode::UInt(n));
    }
    assert_eq!(
        rp::reference_value(value),
        Err(rp::ProjectionGap::UnsupportedGeneric(
            rp::GenericGap::ConstructorValue {
                type_id,
                ctor_id,
                arity: 2
            }
        ))
    );
}

#[test]
fn generic_planned_match_refused_and_dedicated_result_converts() {
    let (initial, last) = run_generic(flag_program());
    let (row, plan) = match source_refusal(&initial) {
        rp::ProjectionGap::UnsupportedGeneric(rp::GenericGap::PlannedMatch { row, plan }) => {
            (row, plan)
        }
        other => panic!("expected explicit PlannedMatch refusal, got {other:?}"),
    };
    let view = &initial.admission.view;
    assert!(
        matches!(view.expressions[row],co::ExpressionSnapshot::Match{plan:Some(p),..} if p==plan)
    );
    assert!(
        view.expressions
            .iter()
            .all(|e| !matches!(e, co::ExpressionSnapshot::Construct { .. }))
    );
    assert!(
        view.expressions[..row]
            .iter()
            .all(|e| !matches!(e, co::ExpressionSnapshot::Match { plan: Some(_), .. }))
    );
    let witness = wp::materialize(&last).expect("generic final witness");
    assert_eq!(
        rp::reference_value(outcome_value(&witness.evaluation.decoded)),
        Ok(a::Value::UInt(1))
    );
}

// Declared-but-unused nominal type and a dedicated-only literal entry; the A
// source matches that entry, so no source mismatch can substitute for the guard.
fn unused_decl_program() -> ExtProgram {
    ExtProgram {
        types: vec![TypeDeclSrc {
            name: "Unused".into(),
            parameters: vec![],
            ctors: vec![CtorDeclSrc {
                name: "U".into(),
                fields: vec![],
            }],
        }],
        assets: vec![],
        states: vec![],
        inputs: vec![],
        functions: vec![],
        entry: uint(5),
    }
}

#[test]
fn generic_profile_unused_nominal_declaration_refused_before_association() {
    let artifact = compile_ext_host(unused_decl_program(), generic_config(), None)
        .unwrap_or_else(|_| panic!("actual public generic compile"));
    let (admitted, image, _) = artifact.into_parts();
    let initial =
        co::capture(&admitted, &image).unwrap_or_else(|_| panic!("generic genesis capture"));
    let (identity, view) = (&initial.admission.identity, &initial.admission.view);
    assert_eq!(
        identity.grammar_domain,
        GrammarDomain::GenericNominalExtensionReserved
    );
    assert!(
        matches!(
            view.expressions.get(view.entry),
            Some(co::ExpressionSnapshot::Literal { .. })
        ) && view
            .expressions
            .iter()
            .all(|e| matches!(e, co::ExpressionSnapshot::Literal { .. })),
        "dedicated-only literal rows"
    );
    assert!(
        view.patterns
            .iter()
            .all(|p| !matches!(p, PatternRowC::Wildcard | PatternRowC::Constructor { .. })),
        "no generic pattern rows"
    );
    assert!(
        view.constants
            .iter()
            .all(|c| !matches!(c, co::ConstantSnapshot::Constructor { .. })),
        "no constructor constants"
    );
    assert!(
        view.functions.is_empty() && view.parameters.is_empty(),
        "no function/parameter TypeIds"
    );
    assert!(
        !view.decl_templates.is_empty(),
        "unused declaration retained in admitted tables"
    );
    let expected = rp::ProjectionGap::UnsupportedGeneric(rp::GenericGap::GenericProfile {
        grammar_domain: identity.grammar_domain,
        types: view.types.len(),
        type_args: view.type_args.len(),
        constructors: view.constructors.len(),
        ctor_fields: view.ctor_fields.len(),
        decl_param_counts: view.decl_param_counts.len(),
        decl_ctor_counts: view.decl_ctor_counts.len(),
        decl_templates: view.decl_templates.len(),
        plan_nodes: view.plan_nodes.len(),
        plan_cases: view.plan_cases.len(),
        field_paths: view.field_paths.len(),
        pattern_children: view.pattern_children.len(),
    });
    let matching = Arc::new(a::Program {
        functions: vec![],
        entry: ex(a::Expr::Literal(a::Value::UInt(5))),
    });
    assert_eq!(
        rp::ReferenceSource::new(matching.clone(), &initial).err(),
        Some(expected.clone())
    );
    assert_eq!(
        rp::ReferencePrefixProjection::new(matching, &initial, 4096).err(),
        Some(expected)
    );
}
