//! Ordinary raw-observation projection tests; no admission or execution claim.
use super::super::complete_observation::{
    ConstFieldsSnapshot, DataFieldsSnapshot, ImageSnapshot, LayoutSnapshot, ProgramSnapshot,
};
use super::*;

fn snapshot() -> HostSnapshot {
    let bounds = BoundsC {
        code_rows: 32,
        function_rows: 8,
        code_child_slots: 32,
        parameter_slots: 8,
        pattern_rows: 32,
        match_plan_rows: 32,
        match_arm_rows: 8,
        type_rows: 16,
        constructor_rows: 16,
        constructor_field_slots: 32,
        max_arity: 8,
        constant_nodes: 32,
        constant_field_slots: 32,
        data_nodes: 32,
        data_field_slots: 32,
        environment_cells: 16,
        frames: 16,
        helper_tasks: 16,
        serialization_bytes: 4096,
        max_reference_work: 1024,
        max_helper_work: 1024,
        step_cap: 1024,
    };
    let identity = IdentityC {
        profile_id: 2,
        grammar_domain: GrammarDomain::GenericNominalExtensionReserved,
        finance: FinanceInput::Absent,
        layout_version: ("projection".into(), "1".into()),
        layout_digest: DigestC::Unselected,
        bounds_digest: DigestC::Unselected,
        field_id: None,
        program_membership_digest: DigestC::Unselected,
    };
    let view = ProgramSnapshot {
        entry: 0,
        expressions: vec![ExpressionSnapshot::Literal { constant: 0 }],
        occurrences: vec![OccurrenceRowC {
            owner_is_function: false,
            function: 0,
            path: SliceC::empty(),
        }],
        occurrence_paths: vec![],
        children: vec![],
        functions: vec![],
        parameters: vec![],
        arms: vec![],
        patterns: vec![],
        pattern_children: vec![],
        types: vec![],
        type_args: vec![],
        constructors: vec![],
        ctor_fields: vec![],
        decl_param_counts: vec![],
        decl_ctor_counts: vec![],
        decl_templates: vec![],
        plan_nodes: vec![],
        plan_cases: vec![],
        field_paths: vec![],
        constants: vec![ConstantSnapshot::U128 {
            marker_free: true,
            n: 17,
        }],
        constant_fields: vec![],
        names: vec!["x".into()],
    };
    HostSnapshot {
        admission: AdmissionSnapshot {
            identity: identity.clone(),
            bounds: bounds.clone(),
            view,
            layout: LayoutSnapshot {
                layout_version: ("projection".into(), "1".into()),
                field_id: None,
                widths: None,
                scalar_layout_id: None,
                bounds,
                capacities: LayoutCapacitiesC::default(),
                boundary_capacities_present: false,
            },
        },
        image: ImageSnapshot {
            run_id: RunIdC {
                identity,
                fuel_input: RunIntC::Nat(1024),
                stack_limit_input: RunIntC::Nat(16),
            },
            control: ControlC::Return {
                value: RefC::Const(0),
            },
            control_origin: 0,
            env_root: None,
            stack_limit: 16,
            outcome: OutcomeC::Running,
            initial_fuel: 1024,
            remaining_work: 1024,
            consumed_work: 0,
            helper_work: 0,
            helper_visits: 0,
            reference_cursor: 0,
            alloc: AllocCounters::default(),
            active: ActiveLengths::default(),
            helper: None,
            result: None,
            seg_base: SegBase::genesis_uncut(),
            data: vec![],
            data_fields: vec![],
            env: vec![],
            frames: vec![],
            tasks: vec![],
            value_slots: vec![],
        },
    }
}
fn sequence<'a>(decoded: &'a DecodedSnapshot, field: &str) -> &'a [SlotValue] {
    let value = &decoded
        .fields
        .iter()
        .find(|v| v.field == field)
        .expect("output field")
        .value;
    let SlotValue::Sequence { items, .. } = value else {
        panic!("sequence output")
    };
    items
}
fn value(item: &SlotValue) -> &DecodedValue {
    let SlotValue::Value(value) = item else {
        panic!("decoded value")
    };
    value
}
fn constructor(node: &ValueNode) -> (TypeIdC, adt::CtorIdC, &[usize]) {
    let ValueNode::Constructor {
        type_id,
        ctor_id,
        fields,
    } = node
    else {
        panic!("constructor")
    };
    (*type_id, *ctor_id, fields)
}
#[test]
fn nested_nominal_values_keep_ids_order_and_repeated_occurrences() {
    let mut input = snapshot();
    input.admission.view.constants.extend([
        ConstantSnapshot::Constructor {
            marker_free: true,
            type_id: TypeIdC(4),
            ctor_id: adt::CtorIdC(8),
            fields: ConstFieldsSnapshot {
                start: usize::MAX,
                len: 0,
            },
        },
        ConstantSnapshot::Constructor {
            marker_free: true,
            type_id: TypeIdC(5),
            ctor_id: adt::CtorIdC(9),
            fields: ConstFieldsSnapshot { start: 0, len: 3 },
        },
    ]);
    input.admission.view.constant_fields = vec![1, 0, 1];
    input.image.data = vec![DataSnapshot::Constructor {
        type_id: TypeIdC(6),
        ctor_id: adt::CtorIdC(10),
        fields: DataFieldsSnapshot { start: 0, len: 2 },
    }];
    input.image.data_fields = vec![RefC::Const(2), RefC::Const(0)];
    let plan = construct_plan(&input).expect("raw nominal projection");
    let result = evaluate_uncached(&plan, &input).expect("evaluate projection");
    let v = value(&sequence(&result.decoded, "constants")[2]);
    let (ty, ctor, fields) = constructor(&v.nodes[v.root]);
    assert_eq!((ty, ctor), (TypeIdC(5), adt::CtorIdC(9)));
    assert_eq!(fields, &[0, 1, 2]);
    assert_ne!(fields[0], fields[2]);
    assert_eq!(v.nodes[fields[0]], v.nodes[fields[2]]);
    assert_eq!(v.nodes[fields[1]], ValueNode::UInt(17));
    let data = value(&sequence(&result.decoded, "data")[0]);
    let (_, _, fields) = constructor(&data.nodes[data.root]);
    let (_, _, inner) = constructor(&data.nodes[fields[0]]);
    assert_eq!(inner.len(), 3);
    assert_eq!(data.nodes[fields[1]], ValueNode::UInt(17));
}
#[test]
fn all_generic_tables_are_typed_immutable_inputs_and_outputs() {
    let mut input = snapshot();
    let p = &mut input.admission.view;
    p.pattern_children = vec![3];
    p.types = vec![adt::TypeRowC {
        name_id: 0,
        decl: Some(0),
        args: SliceC { start: 0, len: 1 },
        ctor_start: adt::CtorIdC(0),
        ctor_len: 1,
    }];
    p.type_args = vec![TypeIdC(19)];
    p.constructors = vec![adt_storage::CtorRowExt {
        owning_type: TypeIdC(0),
        name_id: 0,
        source_order: 0,
        field_start: 0,
        field_len: 1,
    }];
    p.ctor_fields = vec![adt_storage::CtorFieldRowExt {
        name_id: 0,
        type_id: TypeIdC(19),
    }];
    p.decl_param_counts = vec![1];
    p.decl_ctor_counts = vec![1];
    p.decl_templates = vec![owner_ext_type_builder::TypeDeclSrc {
        name: "Box".into(),
        parameters: vec!["a".into()],
        ctors: vec![],
    }];
    p.plan_nodes = vec![PlanNodeC::NoMatch { source_match: 0 }];
    p.plan_cases = vec![Some(0), None];
    p.field_paths = vec![2, 1];
    let plan = construct_plan(&input).expect("table projection");
    let result = evaluate_uncached(&plan, &input).expect("table evaluation");
    for (name, len) in [
        ("pattern_children", 1),
        ("types", 1),
        ("type_args", 1),
        ("constructors", 1),
        ("ctor_fields", 1),
        ("decl_param_counts", 1),
        ("decl_ctor_counts", 1),
        ("decl_templates", 1),
        ("plan_nodes", 1),
        ("plan_cases", 2),
        ("field_paths", 2),
    ] {
        assert_eq!(sequence(&result.decoded, name).len(), len, "{name}");
    }
    assert_eq!(
        sequence(&result.decoded, "type_args"),
        &[SlotValue::Source(InputValue::TypeId(TypeIdC(19)))]
    );
    input.admission.view.plan_cases[1] = Some(0);
    assert!(matches!(
        evaluate_uncached(&plan, &input),
        Err(WitnessFailure {
            kind: FailureKind::InputChanged,
            ..
        })
    ));
}
#[test]
fn new_payload_variants_follow_existing_value_expression_and_environment_paths() {
    let mut input = snapshot();
    let ty = TypeIdC(0);
    let ctor = adt::CtorIdC(0);
    input.admission.view.types = vec![adt::TypeRowC {
        name_id: 0,
        decl: None,
        args: SliceC::empty(),
        ctor_start: ctor,
        ctor_len: 1,
    }];
    input.admission.view.constructors = vec![adt_storage::CtorRowExt {
        owning_type: ty,
        name_id: 0,
        source_order: 0,
        field_start: 0,
        field_len: 0,
    }];
    input.admission.view.plan_nodes = vec![PlanNodeC::Select {
        source_match: 0,
        arm: 0,
    }];
    input.admission.view.arms = vec![ArmRowC {
        pattern: 0,
        body: 0,
    }];
    input.admission.view.children = vec![0];
    input.image.value_slots = vec![RefC::Const(0)];
    input.image.frames = vec![FrameRowC::ConstructorFieldsF {
        type_id: ty,
        ctor_id: ctor,
        index: 0,
        field_type: ty,
        checked: true,
        completed: SliceC { start: 0, len: 1 },
        rest: SliceC { start: 0, len: 1 },
        saved_env: None,
        origin: 0,
    }];
    input.image.control = ControlC::Materialize {
        type_id: ty,
        ctor_id: ctor,
        fields: SliceC { start: 0, len: 1 },
    };
    let common = HelperCommon {
        site: SiteTag::Materialize,
        origin_reference_cursor: 0,
        parent_resume: None,
    };
    let helpers = [
        HelperControlC::Build {
            common,
            type_id: ty,
            ctor_id: ctor,
            src: SliceC { start: 0, len: 1 },
            src_cursor: 0,
            field_start: 0,
            staged: Some(RefC::Const(0)),
            node: None,
            phase: BuildPhase::Copy,
        },
        HelperControlC::Plan {
            common,
            source_match: 0,
            scrutinee: RefC::Const(0),
            saved_env: None,
            partial: None,
            node: 0,
            phase: PlanPhase::Ready,
            after_walk: PlanAfterWalk::Test,
            cur: RefC::Const(0),
            path: SliceC {
                start: usize::MAX,
                len: 0,
            },
            path_cursor: 0,
            arm: Some(0),
            status: TriStatus::Pending,
        },
    ];
    for helper in helpers {
        input.image.helper = Some(helper);
        for result in [
            HelperResultC::BuildDone {
                value: RefC::Const(0),
            },
            HelperResultC::PlanMatched { arm: 0, env: None },
            HelperResultC::PlanNoMatch,
        ] {
            input.image.result = Some(result);
            let projection = materialize(&input).expect("generic payload projection");
            assert_eq!(projection.raw, input);
            let field = projection
                .evaluation
                .decoded
                .fields
                .iter()
                .find(|f| f.field == "helper")
                .unwrap();
            let SlotValue::Payload(payload) = &field.value else {
                panic!("helper payload")
            };
            assert!(!payload.fields.is_empty());
        }
    }
}
#[test]
fn dedicated_values_and_nominal_children_share_existing_occurrence_reconstruction() {
    let mut input = snapshot();
    input.admission.view.constants.extend([
        ConstantSnapshot::Nil { marker_free: true },
        ConstantSnapshot::Cons {
            marker_free: true,
            fields: ConstFieldsSnapshot { start: 0, len: 2 },
        },
        ConstantSnapshot::Leaf {
            marker_free: true,
            fields: ConstFieldsSnapshot { start: 2, len: 1 },
        },
        ConstantSnapshot::Branch {
            marker_free: true,
            fields: ConstFieldsSnapshot { start: 3, len: 2 },
        },
        ConstantSnapshot::Constructor {
            marker_free: true,
            type_id: TypeIdC(41),
            ctor_id: adt::CtorIdC(73),
            fields: ConstFieldsSnapshot { start: 5, len: 2 },
        },
    ]);
    input.admission.view.constant_fields = vec![0, 1, 0, 3, 3, 2, 4];
    let plan = construct_plan(&input).expect("mixed dedicated and nominal rows");
    let plain = evaluate_uncached(&plan, &input).expect("baseline DAG evaluation");
    assert_eq!(
        value(&sequence(&plain.decoded, "constants")[2]).nodes,
        vec![
            ValueNode::UInt(17),
            ValueNode::Nil,
            ValueNode::Cons { head: 0, tail: 1 }
        ]
    );
    let mixed = value(&sequence(&plain.decoded, "constants")[5]);
    let (ty, ctor, fields) = constructor(&mixed.nodes[mixed.root]);
    assert_eq!((ty, ctor), (TypeIdC(41), adt::CtorIdC(73)));
    assert!(matches!(&mixed.nodes[fields[0]], ValueNode::Cons { .. }));
    assert!(matches!(&mixed.nodes[fields[1]], ValueNode::Branch { .. }));
    assert_eq!(
        evaluate_last_use(&plan, &input).unwrap().witness.decoded,
        plain.decoded
    );
    let mut cache = WitnessCache::default();
    for storage in [SlotStorage::RetainAll, SlotStorage::LastUse] {
        assert_eq!(
            evaluate_memoized(&plan, &input, &mut cache, storage)
                .unwrap()
                .witness
                .decoded,
            plain.decoded
        );
    }
}
