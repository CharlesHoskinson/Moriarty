use super::super::adt::TypeIdC;
use super::super::owner_ext_type_builder::{
    CtorDeclSrc, CtorFieldSrc, ExtTypeBuildError, ExtTypeCapKind, ExtTypeRejectReason,
    TypeDeclSrc, TypeExprSrc,
};
use super::super::{
    BoundsC, CapacityKind, FeatureKind, GrammarDomain, LayoutCapacitiesC, LayoutSchemaC, Tooling,
};
use super::{prepare_nominal_declarations, PreparedDeclarationScope};

fn test_layout() -> LayoutSchemaC {
    LayoutSchemaC {
        layout_version: ("1".to_string(), "0".to_string()),
        field_id: None,
        widths: None,
        scalar_layout_id: None,
        bounds: BoundsC {
            type_rows: 16,
            constructor_rows: 16,
            constructor_field_slots: 32,
            max_arity: 4,
            ..BoundsC::default()
        },
        capacities: LayoutCapacitiesC {
            name_rows: 32,
            name_bytes: 256,
            occurrence_path_slots: 32,
            value_slots: 32,
        },
        boundary_capacities_present: false,
    }
}

#[test]
fn mutual_recursive_and_parameterized_t1_t2_order() {
    let decls = vec![
        TypeDeclSrc {
            name: "Boxed".to_string(),
            parameters: vec!["T".to_string()],
            ctors: vec![CtorDeclSrc {
                name: "Wrap".to_string(),
                fields: vec![CtorFieldSrc {
                    name: "item".to_string(),
                    ty: TypeExprSrc::Param {
                        name: "T".to_string(),
                    },
                }],
            }],
        },
        TypeDeclSrc {
            name: "Even".to_string(),
            parameters: vec![],
            ctors: vec![
                CtorDeclSrc {
                    name: "Zero".to_string(),
                    fields: vec![],
                },
                CtorDeclSrc {
                    name: "SuccE".to_string(),
                    fields: vec![CtorFieldSrc {
                        name: "pred".to_string(),
                        ty: TypeExprSrc::Named {
                            name: "Odd".to_string(),
                        },
                    }],
                },
            ],
        },
        TypeDeclSrc {
            name: "Odd".to_string(),
            parameters: vec![],
            ctors: vec![CtorDeclSrc {
                name: "SuccO".to_string(),
                fields: vec![
                    CtorFieldSrc {
                        name: "pred".to_string(),
                        ty: TypeExprSrc::Named {
                            name: "Even".to_string(),
                        },
                    },
                    CtorFieldSrc {
                        name: "flag".to_string(),
                        ty: TypeExprSrc::Apply {
                            name: "Boxed".to_string(),
                            args: vec![TypeExprSrc::Bool],
                        },
                    },
                ],
            }],
        },
    ];
    let out = match prepare_nominal_declarations(
        &decls,
        &test_layout(),
        GrammarDomain::GenericNominalExtensionReserved,
        None,
    ) {
        Ok(v) => v,
        Err(_) => panic!("expected mutual + parameterized declarations to succeed"),
    };
    assert_eq!(out.scope, PreparedDeclarationScope::DeclarationOnly);
    // 4 builtins (0..3) + Even(4) + Odd(5) + Boxed<Bool>(6) = 7 types
    assert_eq!(out.tables.types.len(), 7);
    assert_eq!(out.tables.types[4].decl, Some(1));
    assert_eq!(out.tables.types[5].decl, Some(2));
    assert_eq!(out.tables.types[6].decl, Some(0));
    assert_eq!(out.tables.type_args, vec![TypeIdC(1)]);
}

#[test]
fn rejects_dedicated_sorts_and_uninstantiated_ill_formed_decl() {
    let decls = vec![TypeDeclSrc {
        name: "UnusedParam".to_string(),
        parameters: vec!["A".to_string()],
        ctors: vec![CtorDeclSrc {
            name: "Mk".to_string(),
            fields: vec![CtorFieldSrc {
                name: "bad".to_string(),
                ty: TypeExprSrc::Param {
                    name: "Undeclared".to_string(),
                },
            }],
        }],
    }];
    let dedicated =
        prepare_nominal_declarations(&decls, &test_layout(), GrammarDomain::DedicatedSorts, None);
    assert!(matches!(
        dedicated,
        Err(ExtTypeBuildError::CoreTooling(
            Tooling::UnsupportedFeature {
                feature: FeatureKind::GenericAdt,
                ..
            }
        ))
    ));

    let bad_param = prepare_nominal_declarations(
        &decls,
        &test_layout(),
        GrammarDomain::GenericNominalExtensionReserved,
        None,
    );
    assert!(matches!(
        bad_param,
        Err(ExtTypeBuildError::ExtAdmissionRejected {
            reason: ExtTypeRejectReason::UnknownType,
            ..
        })
    ));

    let mut tiny_names = test_layout();
    tiny_names.capacities.name_rows = 2;
    let cap_fail = prepare_nominal_declarations(
        &decls,
        &tiny_names,
        GrammarDomain::GenericNominalExtensionReserved,
        None,
    );
    assert!(matches!(
        cap_fail,
        Err(ExtTypeBuildError::CoreTooling(
            Tooling::CapacityNotAdmitted {
                kind: CapacityKind::NameRows,
                ..
            }
        ))
    ));

    let mut tiny_types = test_layout();
    tiny_types.bounds.type_rows = 4;
    let type_cap_fail = prepare_nominal_declarations(
        &decls,
        &tiny_types,
        GrammarDomain::GenericNominalExtensionReserved,
        None,
    );
    assert!(matches!(
        type_cap_fail,
        Err(ExtTypeBuildError::ExtCapacityNotAdmitted {
            kind: ExtTypeCapKind::TypeRows,
            ..
        })
    ));
}
