//! Author reference expectation, frozen before adapter implementation/execution.
//! This is not the separately owned independent semantic oracle.
use mori_core::sherlock::helper_machine::shared::{
    adt::{CtorIdC, TypeIdC, TypeRowC},
    adt_storage::{CtorFieldRowExt, CtorRowExt},
    owner_ext_type_builder::{CtorDeclSrc, CtorFieldSrc, TypeDeclSrc, TypeExprSrc},
};
use mori_core::sherlock::helper_machine::{complete_observation::*, shared::*};

fn slice(start: usize, len: usize) -> SliceC {
    SliceC { start, len }
}

pub fn wrapped_countdown() -> ProgramSnapshot {
    use ExpressionSnapshot as E;
    let paths = [
        vec![],
        vec![0],
        vec![],
        vec![0],
        vec![0, 0],
        vec![0, 1],
        vec![1],
        vec![1, 0],
        vec![2],
        vec![2, 0],
        vec![2, 0, 0],
        vec![2, 0, 1],
    ];
    let mut occurrence_paths = Vec::new();
    let occurrences = paths
        .into_iter()
        .enumerate()
        .map(|(id, path)| {
            let span = if path.is_empty() {
                slice(0, 0)
            } else {
                slice(occurrence_paths.len(), path.len())
            };
            occurrence_paths.extend(path);
            OccurrenceRowC {
                owner_is_function: id >= 2,
                function: 0,
                path: span,
            }
        })
        .collect();
    ProgramSnapshot {
        entry: 0,
        expressions: vec![
            E::Call {
                function_name: 2,
                args: slice(0, 1),
            },
            E::Literal { constant: 0 },
            E::If {
                cond: 3,
                then_expr: 6,
                else_expr: 8,
            },
            E::Binary {
                op: BinaryOp::Eq,
                left: 4,
                right: 5,
            },
            E::Var { binder: 3 },
            E::Literal { constant: 1 },
            E::Construct {
                type_id: TypeIdC(4),
                ctor_id: CtorIdC(4),
                fields: slice(1, 1),
                constant: None,
            },
            E::Literal { constant: 2 },
            E::Call {
                function_name: 2,
                args: slice(2, 1),
            },
            E::Binary {
                op: BinaryOp::Sub,
                left: 10,
                right: 11,
            },
            E::Var { binder: 3 },
            E::Literal { constant: 3 },
        ],
        occurrences,
        occurrence_paths,
        children: vec![1, 7, 9],
        functions: vec![FunctionRowC {
            function_name: 2,
            params: slice(0, 1),
            return_type: TypeIdC(4),
            body: 2,
        }],
        parameters: vec![ParameterRowC {
            binder: 3,
            sort: TypeIdC::UInt,
        }],
        arms: vec![],
        patterns: vec![],
        pattern_children: vec![],
        types: vec![
            TypeRowC {
                name_id: 0,
                decl: None,
                args: slice(0, 0),
                ctor_start: CtorIdC(0),
                ctor_len: 0,
            },
            TypeRowC {
                name_id: 0,
                decl: None,
                args: slice(0, 0),
                ctor_start: CtorIdC(0),
                ctor_len: 0,
            },
            TypeRowC {
                name_id: 0,
                decl: None,
                args: slice(0, 0),
                ctor_start: CtorIdC(0),
                ctor_len: 2,
            },
            TypeRowC {
                name_id: 0,
                decl: None,
                args: slice(0, 0),
                ctor_start: CtorIdC(2),
                ctor_len: 2,
            },
            TypeRowC {
                name_id: 1,
                decl: Some(0),
                args: slice(0, 1),
                ctor_start: CtorIdC(4),
                ctor_len: 1,
            },
        ],
        type_args: vec![TypeIdC::UInt],
        constructors: vec![
            CtorRowExt {
                owning_type: TypeIdC::List,
                name_id: 0,
                source_order: 0,
                field_start: 0,
                field_len: 0,
            },
            CtorRowExt {
                owning_type: TypeIdC::List,
                name_id: 0,
                source_order: 1,
                field_start: 0,
                field_len: 2,
            },
            CtorRowExt {
                owning_type: TypeIdC::Tree,
                name_id: 0,
                source_order: 0,
                field_start: 2,
                field_len: 1,
            },
            CtorRowExt {
                owning_type: TypeIdC::Tree,
                name_id: 0,
                source_order: 1,
                field_start: 3,
                field_len: 2,
            },
            CtorRowExt {
                owning_type: TypeIdC(4),
                name_id: 0,
                source_order: 0,
                field_start: 5,
                field_len: 1,
            },
        ],
        ctor_fields: vec![
            CtorFieldRowExt {
                name_id: 0,
                type_id: TypeIdC::UInt,
            },
            CtorFieldRowExt {
                name_id: 0,
                type_id: TypeIdC::List,
            },
            CtorFieldRowExt {
                name_id: 0,
                type_id: TypeIdC::UInt,
            },
            CtorFieldRowExt {
                name_id: 0,
                type_id: TypeIdC::Tree,
            },
            CtorFieldRowExt {
                name_id: 0,
                type_id: TypeIdC::Tree,
            },
            CtorFieldRowExt {
                name_id: 4,
                type_id: TypeIdC::UInt,
            },
        ],
        decl_param_counts: vec![1],
        decl_ctor_counts: vec![1],
        decl_templates: vec![TypeDeclSrc {
            name: "Parcel".into(),
            parameters: vec!["T".into()],
            ctors: vec![CtorDeclSrc {
                name: "Pack".into(),
                fields: vec![CtorFieldSrc {
                    name: "value".into(),
                    ty: TypeExprSrc::Param { name: "T".into() },
                }],
            }],
        }],
        plan_nodes: vec![],
        plan_cases: vec![],
        field_paths: vec![],
        constants: [2, 0, 11, 1]
            .into_iter()
            .map(|n| ConstantSnapshot::U128 {
                marker_free: true,
                n,
            })
            .collect(),
        constant_fields: vec![],
        names: ["Pack", "Parcel", "descend", "n", "value"]
            .into_iter()
            .map(String::from)
            .collect(),
    }
}
