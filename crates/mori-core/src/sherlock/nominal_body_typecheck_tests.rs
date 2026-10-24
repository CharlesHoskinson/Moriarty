use super::*;
use crate::sherlock::helper_machine::shared::owner_ext_source_program::ExtDomain;
use crate::sherlock::helper_machine::shared::{BoundsC, LayoutCapacitiesC, LayoutSchemaC};
use crate::sherlock_nominal_source::prepare_nominal_source_types;

fn layout() -> LayoutSchemaC {
    LayoutSchemaC {
        layout_version: ("1".into(), "0".into()),
        field_id: None,
        widths: None,
        scalar_layout_id: None,
        bounds: BoundsC {
            type_rows: 64,
            constructor_rows: 128,
            constructor_field_slots: 256,
            max_arity: 8,
            ..BoundsC::default()
        },
        capacities: LayoutCapacitiesC {
            name_rows: 256,
            name_bytes: 4096,
            occurrence_path_slots: 1024,
            value_slots: 256,
        },
        ext_capacities: None,
        boundary_capacities_present: false,
    }
}

fn check(body: &str) -> std::result::Result<NominalBodyTypeSummary, NominalBodyTypeError> {
    let source = format!("language \"moriarty-sherlock-nominal/1\"; program Example {{ {body} }}");
    let prepared =
        match prepare_nominal_source_types(&source, &layout(), ExtDomain::GENERIC_ABSENT, None) {
            Ok(p) => p,
            Err(_) => panic!("functional fixture must parse and structurally prepare"),
        };
    check_nominal_program_bodies(&prepared.prepared)
}

#[test]
fn unused_body_must_match_its_return_signature() {
    let error = check("fn f(n: UInt): Bool { n } entry true;").unwrap_err();
    assert_eq!(error.source.function_index, Some(0));
    assert!(matches!(
        error.kind,
        NominalBodyTypeErrorKind::Expected {
            expected: TypeIdC::Bool,
            actual: TypeIdC::UInt
        }
    ));
}

#[test]
fn bodies_and_entry_have_separate_lexical_environments() {
    for source in [
        "fn f(n: UInt): UInt { missing } entry 0;",
        "fn f(n: UInt): UInt { n } entry n;",
        "entry let x = x in x;",
    ] {
        assert!(matches!(
            check(source).unwrap_err().kind,
            NominalBodyTypeErrorKind::UnknownVariable(_)
        ));
    }
}

#[test]
fn let_shadowing_restores_the_outer_type() {
    assert_eq!(
        check("entry let x = 7 in let y = (let x = true in not(x)) in x + 1;")
            .unwrap()
            .entry_type,
        TypeIdC::UInt
    );
}

#[test]
fn primitive_operands_and_condition_are_typed() {
    for source in [
        "entry true + 1;",
        "entry 1 * false;",
        "entry not(1);",
        "entry if 1 { true } else { false };",
        "entry cons(true, nil);",
        "entry branch(leaf(1), false);",
    ] {
        assert!(matches!(
            check(source).unwrap_err().kind,
            NominalBodyTypeErrorKind::Expected { .. }
        ));
    }
    assert_eq!(
        check("entry not((1 + 2) < (3 * 4));").unwrap().entry_type,
        TypeIdC::Bool
    );
}

#[test]
fn branch_and_arm_results_must_agree() {
    assert!(matches!(
        check("entry if true { 1 } else { false };")
            .unwrap_err()
            .kind,
        NominalBodyTypeErrorKind::BranchTypes { .. }
    ));
    assert!(matches!(
        check("entry match nil { nil => 1, cons(h,t) => false };")
            .unwrap_err()
            .kind,
        NominalBodyTypeErrorKind::ArmTypes { .. }
    ));
}

#[test]
fn named_calls_check_lookup_arity_and_arguments() {
    assert!(matches!(
        check("entry missing(1);").unwrap_err().kind,
        NominalBodyTypeErrorKind::UnknownFunction(_)
    ));
    assert!(matches!(
        check("fn f(n: UInt): UInt { n } entry f();")
            .unwrap_err()
            .kind,
        NominalBodyTypeErrorKind::Arity { .. }
    ));
    assert!(matches!(
        check("fn f(n: UInt): UInt { n } entry f(true);")
            .unwrap_err()
            .kind,
        NominalBodyTypeErrorKind::Expected { .. }
    ));
}

#[test]
fn recursive_calls_check_signatures_without_unfolding() {
    assert_eq!(
        check("fn f(n: UInt): UInt { f(n) } entry f(1);")
            .unwrap()
            .entry_type,
        TypeIdC::UInt
    );
    assert_eq!(
        check("fn even(n: UInt): Bool { odd(n) } fn odd(n: UInt): Bool { even(n) } entry even(1);")
            .unwrap()
            .entry_type,
        TypeIdC::Bool
    );
    assert!(matches!(
        check("fn f(n: UInt): UInt { f(false) } entry 0;")
            .unwrap_err()
            .kind,
        NominalBodyTypeErrorKind::Expected { .. }
    ));
}

const BOX: &str = "type Box<T> { Wrap(item: T); }";

#[test]
fn generic_constructor_fields_use_the_instantiated_type() {
    assert!(check(&format!("{BOX} entry construct Box<UInt>::Wrap(7);")).is_ok());
    assert!(matches!(
        check(&format!("{BOX} entry construct Box<UInt>::Wrap(true);"))
            .unwrap_err()
            .kind,
        NominalBodyTypeErrorKind::Expected {
            expected: TypeIdC::UInt,
            actual: TypeIdC::Bool
        }
    ));
    assert!(matches!(
        check(&format!(
            "{BOX} fn f(x: Box<UInt>): Box<Bool> {{ x }} entry 0;"
        ))
        .unwrap_err()
        .kind,
        NominalBodyTypeErrorKind::Expected { .. }
    ));
}

#[test]
fn nested_patterns_bind_instantiated_fields_in_declaration_order() {
    let prefix = format!("{BOX} type Pair<A,B> {{ Both(first: A, second: B); }}");
    let expression = "match construct Pair<Bool,Box<UInt>>::Both(true, construct Box<UInt>::Wrap(8)) { Pair<Bool,Box<UInt>>::Both(flag,Box<UInt>::Wrap(n)) => if flag { n + 1 } else { 0 } }";
    assert_eq!(
        check(&format!("{prefix} entry {expression};"))
            .unwrap()
            .entry_type,
        TypeIdC::UInt
    );
    assert!(matches!(
        check(&format!(
            "{prefix} entry {} ;",
            expression.replace("n + 1", "flag + 1")
        ))
        .unwrap_err()
        .kind,
        NominalBodyTypeErrorKind::Expected { .. }
    ));
}

#[test]
fn nominal_patterns_match_the_scrutinee_instantiation() {
    assert!(matches!(
        check(&format!(
            "{BOX} entry match construct Box<UInt>::Wrap(1) {{ Box<Bool>::Wrap(x) => x }};"
        ))
        .unwrap_err()
        .kind,
        NominalBodyTypeErrorKind::Expected { .. }
    ));
}

#[test]
fn binders_are_local_to_each_arm() {
    assert!(matches!(
        check("entry match nil { cons(h,t) => h, nil => h };")
            .unwrap_err()
            .kind,
        NominalBodyTypeErrorKind::UnknownVariable(_)
    ));
}

#[test]
fn nested_duplicate_binders_follow_existing_g4() {
    assert!(
        matches!(check(&format!("{BOX} type Pair {{ Both(left: Box<UInt>, right: UInt); }} entry match construct Pair::Both(construct Box<UInt>::Wrap(1),2) {{ Pair::Both(Box<UInt>::Wrap(x),x) => x }};")).unwrap_err().kind, NominalBodyTypeErrorKind::DuplicatePatternBinder(name) if name == "x")
    );
}

#[test]
fn recursive_nominal_references_remain_finite() {
    assert_eq!(check("type Even { Zero; Step(pred: Odd); } type Odd { Step(pred: Even); } fn f(x: Even): Even { x } entry f(construct Even::Zero());").unwrap().function_returns.len(), 1);
}

#[test]
fn dedicated_pattern_fields_keep_their_types() {
    assert_eq!(
        check("entry match cons(1,nil) { cons(h,t) => cons(h,t), nil => nil };")
            .unwrap()
            .entry_type,
        TypeIdC::List
    );
    assert_eq!(check("entry match branch(leaf(1),leaf(2)) { branch(l,r) => branch(l,r), leaf(n) => leaf(n) };").unwrap().entry_type, TypeIdC::Tree);
}

#[test]
fn proposed_eq_any_rule_accepts_individually_typed_operands() {
    assert_eq!(check("entry 1 == true;").unwrap().entry_type, TypeIdC::Bool);
}

#[test]
fn calls_retain_first_matching_function_table_order() {
    assert_eq!(
        check("fn f(n: UInt): UInt { n } fn f(b: Bool): Bool { b } entry f(4);")
            .unwrap()
            .entry_type,
        TypeIdC::UInt
    );
    assert!(matches!(
        check("fn f(n: UInt): UInt { n } fn f(b: Bool): Bool { b } entry f(true);")
            .unwrap_err()
            .kind,
        NominalBodyTypeErrorKind::Expected { .. }
    ));
}

#[test]
fn pattern_shadowing_restores_the_surrounding_environment() {
    assert_eq!(check("entry let x = true in let y = (match cons(3,nil) { cons(x,t) => x + 1, nil => 0 }) in not(x);").unwrap().entry_type, TypeIdC::Bool);
}

#[test]
fn proposed_static_rule_rejects_even_unused_duplicate_parameters() {
    assert!(matches!(
        check("fn f(x: UInt,x: UInt): UInt { x } entry 0;")
            .unwrap_err()
            .kind,
        NominalBodyTypeErrorKind::DuplicateParameter { index: 1, .. }
    ));
}

#[test]
fn owned_nested_constructor_literals_check_all_field_types() {
    use crate::sherlock::helper_machine::shared::i128_class::KIntC;
    use crate::sherlock::helper_machine::shared::prepare_ext_program_types;
    use crate::sherlock_nominal_source::parse_nominal_source;
    for good in [true, false] {
        let source = format!(
            "language \"moriarty-sherlock-nominal/1\"; program Literal {{ {BOX} type Outer {{ Hold(inner: Box<UInt>); }} entry 0; }}"
        );
        let mut parsed = match parse_nominal_source(&source) {
            Ok(p) => p,
            Err(_) => panic!("literal fixture syntax"),
        };
        parsed.program.entry = ExtExpr::Literal {
            value: ExtValue::Constructor {
                ty: TypeExprSrc::Named {
                    name: "Outer".into(),
                },
                ctor: "Hold".into(),
                fields: vec![ExtValue::Constructor {
                    ty: TypeExprSrc::Apply {
                        name: "Box".into(),
                        args: vec![TypeExprSrc::UInt],
                    },
                    ctor: "Wrap".into(),
                    fields: vec![ExtValue::Core {
                        value: if good {
                            CoreValue::UInt {
                                n: KIntC::InRange(9),
                            }
                        } else {
                            CoreValue::Bool { b: true }
                        },
                    }],
                }],
            },
        };
        let prepared = match prepare_ext_program_types(
            parsed.program,
            &layout(),
            ExtDomain::GENERIC_ABSENT,
            None,
        ) {
            Ok(p) => p,
            Err(_) => panic!("literal fixture must structurally prepare"),
        };
        let result = check_nominal_program_bodies(&prepared);
        if good {
            assert!(result.unwrap().entry_type.0 >= 4);
        } else {
            let error = result.unwrap_err();
            assert_eq!(error.source.child_path, vec![0, 0]);
            assert!(matches!(
                error.kind,
                NominalBodyTypeErrorKind::Expected {
                    expected: TypeIdC::UInt,
                    actual: TypeIdC::Bool
                }
            ));
        }
    }
}

// Review M1 discriminator: frozen before changing Checker::fields. Builtin
// constructor rows have fixed ordinals and no interned constructor-name binding.
#[test]
fn builtin_qualified_patterns_follow_owner_fixed_constructor_ordinals() {
    for source in [
        "entry match cons(1,nil) { List::Cons(h,t) => h, List::Nil() => 0 };",
        "entry match branch(leaf(3),leaf(4)) { Tree::Branch(Tree::Leaf(n),r) => n, Tree::Leaf(n) => n };",
        "entry match cons(1,nil) { List::Cons(h,List::Nil()) => h, wildcard => 0 };",
    ] {
        assert_eq!(check(source).unwrap().entry_type, TypeIdC::UInt);
    }
    assert!(matches!(
        check("entry match nil { Tree::Leaf(n) => n };")
            .unwrap_err()
            .kind,
        NominalBodyTypeErrorKind::Expected {
            expected: TypeIdC::Tree,
            actual: TypeIdC::List
        }
    ));
}

// Revision02 review R1(a): source-derived expectations, frozen without changing
// the production checker. Both constructor use kinds share the owner resolver.
#[test]
fn owner_preparation_refuses_builtin_construct_and_literal_constructor_values() {
    use crate::sherlock::helper_machine::shared::i128_class::KIntC;
    use crate::sherlock::helper_machine::shared::owner_ext_type_builder::{
        ExtTypeBuildError, ExtTypeRejectReason,
    };
    use crate::sherlock::helper_machine::shared::prepare_ext_program_types;
    use crate::sherlock_nominal_source::{NominalSourceError, parse_nominal_source};
    for expression in ["construct List::Cons(1,nil)", "construct Tree::Leaf(1)"] {
        let source = format!(
            "language \"moriarty-sherlock-nominal/1\"; program Builtin {{ entry {expression}; }}"
        );
        assert!(matches!(
            prepare_nominal_source_types(&source, &layout(), ExtDomain::GENERIC_ABSENT, None),
            Err(NominalSourceError::Preparation(
                ExtTypeBuildError::ExtAdmissionRejected {
                    reason: ExtTypeRejectReason::BuiltinConstruct,
                    ..
                }
            ))
        ));
    }
    for list in [true, false] {
        let mut parsed = match parse_nominal_source(
            "language \"moriarty-sherlock-nominal/1\"; program Builtin { entry 0; }",
        ) {
            Ok(parsed) => parsed,
            Err(_) => panic!("builtin literal fixture syntax"),
        };
        let one = ExtValue::Core {
            value: CoreValue::UInt {
                n: KIntC::InRange(1),
            },
        };
        let (ty, ctor, fields) = if list {
            (
                TypeExprSrc::List,
                "Cons",
                vec![
                    one,
                    ExtValue::Core {
                        value: CoreValue::List { elements: vec![] },
                    },
                ],
            )
        } else {
            (TypeExprSrc::Tree, "Leaf", vec![one])
        };
        parsed.program.entry = ExtExpr::Literal {
            value: ExtValue::Constructor {
                ty,
                ctor: ctor.into(),
                fields,
            },
        };
        assert!(matches!(
            prepare_ext_program_types(parsed.program, &layout(), ExtDomain::GENERIC_ABSENT, None),
            Err(ExtTypeBuildError::ExtAdmissionRejected {
                reason: ExtTypeRejectReason::BuiltinConstruct,
                ..
            })
        ));
    }
}
