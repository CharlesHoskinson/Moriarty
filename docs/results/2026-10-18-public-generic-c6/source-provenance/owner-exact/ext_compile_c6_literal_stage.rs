//! Public compile_ext_host regressions for the C6 Literal-slot stage split in
//! owner_ext_c_expression.rs. Valid source only; no expected tables, constant
//! IDs or runtime results are asserted. Owner admission (V11/V12/V14) runs
//! inside compile_ext_host before Ok is returned.
use mori_core::sherlock::compile_artifact::ext_compile::{
    compile_ext_host, ExtCompileArtifact, ExtCompileFailure,
};
use mori_core::sherlock::helper_machine::shared::i128_class::KIntC;
use mori_core::sherlock::helper_machine::shared::owner_ext_source_program::ExtProgram;
use mori_core::sherlock::helper_machine::shared::owner_ext_t3_uses::{
    ExtArm, ExtExpr, ExtFunction, ExtParamSrc, ExtPattern, ExtValue,
};
use mori_core::sherlock::helper_machine::shared::owner_ext_type_builder::{
    CtorDeclSrc, CtorFieldSrc, TypeDeclSrc, TypeExprSrc,
};
use mori_core::sherlock::helper_machine::shared::{BinaryOp, CorePattern, CoreValue};

#[path = "support/c6_stage_safe_config.rs"]
mod safe_config;

#[path = "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/independent-public-generic-runtime-oracle-20261018/inputs/independent_public_generic_inputs.rs"]
mod independent_inputs;

fn named(name: &str) -> TypeExprSrc {
    TypeExprSrc::Named { name: name.into() }
}

fn uint_value(n: u128) -> ExtValue {
    ExtValue::Core { value: CoreValue::UInt { n: KIntC::InRange(n) } }
}

fn uint_literal(n: u128) -> ExtExpr {
    ExtExpr::Literal { value: uint_value(n) }
}

fn nullary(ty: &str, ctor: &str) -> ExtExpr {
    ExtExpr::Construct { ty: named(ty), ctor: ctor.into(), fields: vec![] }
}

fn compile_admitted(program: ExtProgram, case: &str) -> ExtCompileArtifact {
    match compile_ext_host(program, safe_config::safe_public_config(), None) {
        Ok(artifact) => artifact,
        Err(ExtCompileFailure::Preparation(_)) => {
            panic!("{}: compile_ext_host preparation/admission refused valid source", case)
        }
        Err(ExtCompileFailure::Genesis(_)) => {
            panic!("{}: preparation admitted but owner genesis refused", case)
        }
    }
}

fn assert_bound_unevaluated(artifact: &ExtCompileArtifact, case: &str) {
    assert!(artifact.observe().is_ok(), "{}: admitted/genesis pair not bound", case);
    match artifact.report() {
        Ok(report) => assert!(!report.evaluation_performed, "{}: report claims evaluation", case),
        Err(_) => panic!("{}: report projection refused bound pair", case),
    }
}

fn flag_decl() -> TypeDeclSrc {
    TypeDeclSrc {
        name: "Flag".into(),
        parameters: vec![],
        ctors: vec![
            CtorDeclSrc { name: "Off".into(), fields: vec![] },
            CtorDeclSrc { name: "On".into(), fields: vec![] },
        ],
    }
}

/// Nullary Construct (preorder ExprId 1) precedes both Literal occurrences.
/// The first Literal is a Constructor value with a nested nullary Constructor
/// field; the second is a Core UInt inside a planned match arm body.
fn nullary_before_later_literals() -> ExtProgram {
    ExtProgram {
        types: vec![
            flag_decl(),
            TypeDeclSrc {
                name: "Tagged".into(),
                parameters: vec![],
                ctors: vec![CtorDeclSrc {
                    name: "Tagged".into(),
                    fields: vec![
                        CtorFieldSrc { name: "tagged_flag".into(), ty: named("Flag") },
                        CtorFieldSrc { name: "tagged_amount".into(), ty: TypeExprSrc::UInt },
                    ],
                }],
            },
        ],
        assets: vec![],
        states: vec![],
        inputs: vec![],
        functions: vec![],
        entry: ExtExpr::Let {
            name: "v10".into(),
            value: Box::new(nullary("Flag", "On")),
            body: Box::new(ExtExpr::Let {
                name: "v20".into(),
                value: Box::new(ExtExpr::Literal {
                    value: ExtValue::Constructor {
                        ty: named("Tagged"),
                        ctor: "Tagged".into(),
                        fields: vec![
                            ExtValue::Constructor {
                                ty: named("Flag"),
                                ctor: "Off".into(),
                                fields: vec![],
                            },
                            uint_value(5),
                        ],
                    },
                }),
                body: Box::new(ExtExpr::Match {
                    scrutinee: Box::new(ExtExpr::Var { name: "v20".into() }),
                    arms: vec![ExtArm {
                        pattern: ExtPattern::Ctor {
                            ty: named("Tagged"),
                            ctor: "Tagged".into(),
                            fields: vec![
                                ExtPattern::Wildcard,
                                ExtPattern::Dedicated {
                                    pattern: CorePattern::PVar { x: "v11".into() },
                                },
                            ],
                        },
                        body: ExtExpr::Construct {
                            ty: named("Tagged"),
                            ctor: "Tagged".into(),
                            fields: vec![
                                ExtExpr::Var { name: "v10".into() },
                                ExtExpr::Binary {
                                    op: BinaryOp::Add,
                                    left: Box::new(ExtExpr::Var { name: "v11".into() }),
                                    right: Box::new(uint_literal(9)),
                                },
                            ],
                        },
                    }],
                }),
            }),
        },
    }
}

/// No Literal anywhere: only nullary Construct emission and a planned match.
fn nullary_only_without_literals() -> ExtProgram {
    ExtProgram {
        types: vec![flag_decl()],
        assets: vec![],
        states: vec![],
        inputs: vec![],
        functions: vec![ExtFunction {
            name: "flip".into(),
            parameters: vec![ExtParamSrc { name: "v10".into(), ty: named("Flag") }],
            return_type: named("Flag"),
            body: ExtExpr::Match {
                scrutinee: Box::new(ExtExpr::Var { name: "v10".into() }),
                arms: vec![
                    ExtArm {
                        pattern: ExtPattern::Ctor {
                            ty: named("Flag"),
                            ctor: "Off".into(),
                            fields: vec![],
                        },
                        body: nullary("Flag", "On"),
                    },
                    ExtArm {
                        pattern: ExtPattern::Ctor {
                            ty: named("Flag"),
                            ctor: "On".into(),
                            fields: vec![],
                        },
                        body: nullary("Flag", "Off"),
                    },
                ],
            },
        }],
        entry: ExtExpr::Call { name: "flip".into(), args: vec![nullary("Flag", "On")] },
    }
}

#[test]
fn recursive_schedule_multiple_literals_and_nested_constructor_constants_compile() {
    let case = "public_recursive_ordered_schedule";
    let artifact = compile_admitted(independent_inputs::public_recursive_ordered_schedule(), case);
    assert_bound_unevaluated(&artifact, case);
}

#[test]
fn mutual_postcaller_literals_then_nullary_constructs_compile() {
    let case = "public_mutual_three_argument_postcaller";
    let artifact =
        compile_admitted(independent_inputs::public_mutual_three_argument_postcaller(), case);
    assert_bound_unevaluated(&artifact, case);
}

#[test]
fn nullary_construct_before_later_literals_compiles() {
    let case = "nullary_before_later_literals";
    let artifact = compile_admitted(nullary_before_later_literals(), case);
    assert_bound_unevaluated(&artifact, case);
}

#[test]
fn nullary_only_program_without_literals_compiles() {
    let case = "nullary_only_without_literals";
    let artifact = compile_admitted(nullary_only_without_literals(), case);
    assert_bound_unevaluated(&artifact, case);
}
