//! Literal SDK-free source fixtures for the combined B engine consumer.
//! Input packet SHA-256: 9affd94479e9db92274eb7b90086f3d680e2d2d69066f0361368228f01badcf4.
//! No evaluator, prepared state, expected result, fuel or stack selection.
//! Include as a sibling of the immutable `shared` module; Root owns registration.
use super::shared::{BinaryOp, BoundsC, CoreArm, CoreExpr, CoreFunction,
    CoreParameter, CorePattern, CoreProgram, CoreTree, CoreValue,
    LayoutCapacitiesC, LayoutSchemaC, SortTag};
use super::shared::i128_class::KIntC;

pub const CASE_IDS: [&str; 13] = ["D01", "D02", "D03", "D04", "D05", "D21", "D14", "D16", "D10", "D12", "D09", "D13", "TREE-EQ-REPEATED"];

pub fn program(case_id: &str) -> Option<CoreProgram> {
    match case_id {
        "D01" => Some(CoreProgram {
            functions: vec![
            ],
            entry: CoreExpr::Literal {
                value: CoreValue::UInt {
                    n: KIntC::InRange(7), // input /programs/0/program/entry/value/n/tag
                },
            },
        }),
        "D02" => Some(CoreProgram {
            functions: vec![
            ],
            entry: CoreExpr::Let {
                name: "x".to_string(), // input /programs/1/program/entry/name
                value: Box::new(CoreExpr::Literal {
                    value: CoreValue::UInt {
                        n: KIntC::InRange(2), // input /programs/1/program/entry/value/value/n/tag
                    },
                }),
                body: Box::new(CoreExpr::Binary {
                    op: BinaryOp::Add, // input /programs/1/program/entry/body/op
                    left: Box::new(CoreExpr::Var {
                        name: "x".to_string(), // input /programs/1/program/entry/body/left/name
                    }),
                    right: Box::new(CoreExpr::Literal {
                        value: CoreValue::UInt {
                            n: KIntC::InRange(3), // input /programs/1/program/entry/body/right/value/n/tag
                        },
                    }),
                }),
            },
        }),
        "D03" => Some(CoreProgram {
            functions: vec![
            ],
            entry: CoreExpr::If {
                cond: Box::new(CoreExpr::Literal {
                    value: CoreValue::Bool {
                        b: true, // input /programs/2/program/entry/cond/value/b
                    },
                }),
                then_expr: Box::new(CoreExpr::Literal {
                    value: CoreValue::UInt {
                        n: KIntC::InRange(9), // input /programs/2/program/entry/thenExpr/value/n/tag
                    },
                }),
                else_expr: Box::new(CoreExpr::Literal {
                    value: CoreValue::UInt {
                        n: KIntC::InRange(4), // input /programs/2/program/entry/elseExpr/value/n/tag
                    },
                }),
            },
        }),
        "D04" => Some(CoreProgram {
            functions: vec![
            ],
            entry: CoreExpr::Binary {
                op: BinaryOp::Add, // input /programs/3/program/entry/op
                left: Box::new(CoreExpr::Binary {
                    op: BinaryOp::Add, // input /programs/3/program/entry/left/op
                    left: Box::new(CoreExpr::Literal {
                        value: CoreValue::UInt {
                            n: KIntC::InRange(1), // input /programs/3/program/entry/left/left/value/n/tag
                        },
                    }),
                    right: Box::new(CoreExpr::Literal {
                        value: CoreValue::UInt {
                            n: KIntC::InRange(2), // input /programs/3/program/entry/left/right/value/n/tag
                        },
                    }),
                }),
                right: Box::new(CoreExpr::Literal {
                    value: CoreValue::UInt {
                        n: KIntC::InRange(3), // input /programs/3/program/entry/right/value/n/tag
                    },
                }),
            },
        }),
        "D05" => Some(CoreProgram {
            functions: vec![
                CoreFunction {
                    name: "identity".to_string(), // input /programs/4/program/functions/0/name
                    parameters: vec![
                        CoreParameter {
                            name: "x".to_string(), // input /programs/4/program/functions/0/parameters/0/name
                            sort: SortTag::UInt, // input /programs/4/program/functions/0/parameters/0/sort
                        },
                    ],
                    return_type: SortTag::UInt, // input /programs/4/program/functions/0/returnType
                    body: CoreExpr::Var {
                        name: "x".to_string(), // input /programs/4/program/functions/0/body/name
                    },
                },
            ],
            entry: CoreExpr::Let {
                name: "x".to_string(), // input /programs/4/program/entry/name
                value: Box::new(CoreExpr::Literal {
                    value: CoreValue::UInt {
                        n: KIntC::InRange(7), // input /programs/4/program/entry/value/value/n/tag
                    },
                }),
                body: Box::new(CoreExpr::Binary {
                    op: BinaryOp::Add, // input /programs/4/program/entry/body/op
                    left: Box::new(CoreExpr::Call {
                        name: "identity".to_string(), // input /programs/4/program/entry/body/left/name
                        args: vec![
                            CoreExpr::Literal {
                                value: CoreValue::UInt {
                                    n: KIntC::InRange(2), // input /programs/4/program/entry/body/left/args/0/value/n/tag
                                },
                            },
                        ],
                    }),
                    right: Box::new(CoreExpr::Var {
                        name: "x".to_string(), // input /programs/4/program/entry/body/right/name
                    }),
                }),
            },
        }),
        "D21" => Some(CoreProgram {
            functions: vec![
                CoreFunction {
                    name: "combine".to_string(), // input /programs/5/program/functions/0/name
                    parameters: vec![
                        CoreParameter {
                            name: "a".to_string(), // input /programs/5/program/functions/0/parameters/0/name
                            sort: SortTag::UInt, // input /programs/5/program/functions/0/parameters/0/sort
                        },
                        CoreParameter {
                            name: "b".to_string(), // input /programs/5/program/functions/0/parameters/1/name
                            sort: SortTag::UInt, // input /programs/5/program/functions/0/parameters/1/sort
                        },
                        CoreParameter {
                            name: "c".to_string(), // input /programs/5/program/functions/0/parameters/2/name
                            sort: SortTag::UInt, // input /programs/5/program/functions/0/parameters/2/sort
                        },
                    ],
                    return_type: SortTag::UInt, // input /programs/5/program/functions/0/returnType
                    body: CoreExpr::Binary {
                        op: BinaryOp::Add, // input /programs/5/program/functions/0/body/op
                        left: Box::new(CoreExpr::Binary {
                            op: BinaryOp::Add, // input /programs/5/program/functions/0/body/left/op
                            left: Box::new(CoreExpr::Var {
                                name: "a".to_string(), // input /programs/5/program/functions/0/body/left/left/name
                            }),
                            right: Box::new(CoreExpr::Binary {
                                op: BinaryOp::Mul, // input /programs/5/program/functions/0/body/left/right/op
                                left: Box::new(CoreExpr::Literal {
                                    value: CoreValue::UInt {
                                        n: KIntC::InRange(10), // input /programs/5/program/functions/0/body/left/right/left/value/n/tag
                                    },
                                }),
                                right: Box::new(CoreExpr::Var {
                                    name: "b".to_string(), // input /programs/5/program/functions/0/body/left/right/right/name
                                }),
                            }),
                        }),
                        right: Box::new(CoreExpr::Binary {
                            op: BinaryOp::Mul, // input /programs/5/program/functions/0/body/right/op
                            left: Box::new(CoreExpr::Literal {
                                value: CoreValue::UInt {
                                    n: KIntC::InRange(100), // input /programs/5/program/functions/0/body/right/left/value/n/tag
                                },
                            }),
                            right: Box::new(CoreExpr::Var {
                                name: "c".to_string(), // input /programs/5/program/functions/0/body/right/right/name
                            }),
                        }),
                    },
                },
            ],
            entry: CoreExpr::Call {
                name: "combine".to_string(), // input /programs/5/program/entry/name
                args: vec![
                    CoreExpr::Literal {
                        value: CoreValue::UInt {
                            n: KIntC::InRange(1), // input /programs/5/program/entry/args/0/value/n/tag
                        },
                    },
                    CoreExpr::Literal {
                        value: CoreValue::UInt {
                            n: KIntC::InRange(2), // input /programs/5/program/entry/args/1/value/n/tag
                        },
                    },
                    CoreExpr::Literal {
                        value: CoreValue::UInt {
                            n: KIntC::InRange(3), // input /programs/5/program/entry/args/2/value/n/tag
                        },
                    },
                ],
            },
        }),
        "D14" => Some(CoreProgram {
            functions: vec![
                CoreFunction {
                    name: "3".to_string(), // input /programs/6/program/functions/0/name
                    parameters: vec![
                        CoreParameter {
                            name: "0".to_string(), // input /programs/6/program/functions/0/parameters/0/name
                            sort: SortTag::List, // input /programs/6/program/functions/0/parameters/0/sort
                        },
                    ],
                    return_type: SortTag::UInt, // input /programs/6/program/functions/0/returnType
                    body: CoreExpr::Match {
                        scrutinee: Box::new(CoreExpr::Var {
                            name: "0".to_string(), // input /programs/6/program/functions/0/body/scrutinee/name
                        }),
                        arms: vec![
                            CoreArm {
                                pattern: CorePattern::PNil,
                                body: CoreExpr::Literal {
                                    value: CoreValue::UInt {
                                        n: KIntC::InRange(0), // input /programs/6/program/functions/0/body/arms/0/body/value/n/tag
                                    },
                                },
                            },
                            CoreArm {
                                pattern: CorePattern::PCons {
                                    x: "1".to_string(), // input /programs/6/program/functions/0/body/arms/1/pattern/x
                                    y: "2".to_string(), // input /programs/6/program/functions/0/body/arms/1/pattern/y
                                },
                                body: CoreExpr::Binary {
                                    op: BinaryOp::Add, // input /programs/6/program/functions/0/body/arms/1/body/op
                                    left: Box::new(CoreExpr::Var {
                                        name: "1".to_string(), // input /programs/6/program/functions/0/body/arms/1/body/left/name
                                    }),
                                    right: Box::new(CoreExpr::Call {
                                        name: "3".to_string(), // input /programs/6/program/functions/0/body/arms/1/body/right/name
                                        args: vec![
                                            CoreExpr::Var {
                                                name: "2".to_string(), // input /programs/6/program/functions/0/body/arms/1/body/right/args/0/name
                                            },
                                        ],
                                    }),
                                },
                            },
                        ],
                    },
                },
            ],
            entry: CoreExpr::Call {
                name: "3".to_string(), // input /programs/6/program/entry/name
                args: vec![
                    CoreExpr::Literal {
                        value: CoreValue::List {
                            elements: vec![
                                KIntC::InRange(1), // input /programs/6/program/entry/args/0/value/elements/0/tag
                                KIntC::InRange(2), // input /programs/6/program/entry/args/0/value/elements/1/tag
                                KIntC::InRange(3), // input /programs/6/program/entry/args/0/value/elements/2/tag
                            ],
                        },
                    },
                ],
            },
        }),
        "D16" => Some(CoreProgram {
            functions: vec![
                CoreFunction {
                    name: "4".to_string(), // input /programs/7/program/functions/0/name
                    parameters: vec![
                        CoreParameter {
                            name: "0".to_string(), // input /programs/7/program/functions/0/parameters/0/name
                            sort: SortTag::Tree, // input /programs/7/program/functions/0/parameters/0/sort
                        },
                    ],
                    return_type: SortTag::UInt, // input /programs/7/program/functions/0/returnType
                    body: CoreExpr::Match {
                        scrutinee: Box::new(CoreExpr::Var {
                            name: "0".to_string(), // input /programs/7/program/functions/0/body/scrutinee/name
                        }),
                        arms: vec![
                            CoreArm {
                                pattern: CorePattern::PLeaf {
                                    x: "1".to_string(), // input /programs/7/program/functions/0/body/arms/0/pattern/x
                                },
                                body: CoreExpr::Var {
                                    name: "1".to_string(), // input /programs/7/program/functions/0/body/arms/0/body/name
                                },
                            },
                            CoreArm {
                                pattern: CorePattern::PBranch {
                                    x: "1".to_string(), // input /programs/7/program/functions/0/body/arms/1/pattern/x
                                    y: "2".to_string(), // input /programs/7/program/functions/0/body/arms/1/pattern/y
                                },
                                body: CoreExpr::Binary {
                                    op: BinaryOp::Add, // input /programs/7/program/functions/0/body/arms/1/body/op
                                    left: Box::new(CoreExpr::Call {
                                        name: "4".to_string(), // input /programs/7/program/functions/0/body/arms/1/body/left/name
                                        args: vec![
                                            CoreExpr::Var {
                                                name: "1".to_string(), // input /programs/7/program/functions/0/body/arms/1/body/left/args/0/name
                                            },
                                        ],
                                    }),
                                    right: Box::new(CoreExpr::Call {
                                        name: "4".to_string(), // input /programs/7/program/functions/0/body/arms/1/body/right/name
                                        args: vec![
                                            CoreExpr::Var {
                                                name: "2".to_string(), // input /programs/7/program/functions/0/body/arms/1/body/right/args/0/name
                                            },
                                        ],
                                    }),
                                },
                            },
                        ],
                    },
                },
            ],
            entry: CoreExpr::Call {
                name: "4".to_string(), // input /programs/7/program/entry/name
                args: vec![
                    CoreExpr::Literal {
                        value: CoreValue::Tree {
                            tree: CoreTree::Branch {
                                left: Box::new(CoreTree::Leaf {
                                    n: KIntC::InRange(4), // input /programs/7/program/entry/args/0/value/tree/left/n/tag
                                }),
                                right: Box::new(CoreTree::Leaf {
                                    n: KIntC::InRange(5), // input /programs/7/program/entry/args/0/value/tree/right/n/tag
                                }),
                            },
                        },
                    },
                ],
            },
        }),
        "D10" => Some(CoreProgram {
            functions: vec![
                CoreFunction {
                    name: "1".to_string(), // input /programs/8/program/functions/0/name
                    parameters: vec![
                        CoreParameter {
                            name: "0".to_string(), // input /programs/8/program/functions/0/parameters/0/name
                            sort: SortTag::UInt, // input /programs/8/program/functions/0/parameters/0/sort
                        },
                    ],
                    return_type: SortTag::UInt, // input /programs/8/program/functions/0/returnType
                    body: CoreExpr::If {
                        cond: Box::new(CoreExpr::Binary {
                            op: BinaryOp::Eq, // input /programs/8/program/functions/0/body/cond/op
                            left: Box::new(CoreExpr::Var {
                                name: "0".to_string(), // input /programs/8/program/functions/0/body/cond/left/name
                            }),
                            right: Box::new(CoreExpr::Literal {
                                value: CoreValue::UInt {
                                    n: KIntC::InRange(0), // input /programs/8/program/functions/0/body/cond/right/value/n/tag
                                },
                            }),
                        }),
                        then_expr: Box::new(CoreExpr::Literal {
                            value: CoreValue::UInt {
                                n: KIntC::InRange(1), // input /programs/8/program/functions/0/body/thenExpr/value/n/tag
                            },
                        }),
                        else_expr: Box::new(CoreExpr::Binary {
                            op: BinaryOp::Mul, // input /programs/8/program/functions/0/body/elseExpr/op
                            left: Box::new(CoreExpr::Var {
                                name: "0".to_string(), // input /programs/8/program/functions/0/body/elseExpr/left/name
                            }),
                            right: Box::new(CoreExpr::Call {
                                name: "1".to_string(), // input /programs/8/program/functions/0/body/elseExpr/right/name
                                args: vec![
                                    CoreExpr::Binary {
                                        op: BinaryOp::Sub, // input /programs/8/program/functions/0/body/elseExpr/right/args/0/op
                                        left: Box::new(CoreExpr::Var {
                                            name: "0".to_string(), // input /programs/8/program/functions/0/body/elseExpr/right/args/0/left/name
                                        }),
                                        right: Box::new(CoreExpr::Literal {
                                            value: CoreValue::UInt {
                                                n: KIntC::InRange(1), // input /programs/8/program/functions/0/body/elseExpr/right/args/0/right/value/n/tag
                                            },
                                        }),
                                    },
                                ],
                            }),
                        }),
                    },
                },
            ],
            entry: CoreExpr::Call {
                name: "1".to_string(), // input /programs/8/program/entry/name
                args: vec![
                    CoreExpr::Literal {
                        value: CoreValue::UInt {
                            n: KIntC::InRange(3), // input /programs/8/program/entry/args/0/value/n/tag
                        },
                    },
                ],
            },
        }),
        "D12" => Some(CoreProgram {
            functions: vec![
                CoreFunction {
                    name: "6".to_string(), // input /programs/9/program/functions/0/name
                    parameters: vec![
                        CoreParameter {
                            name: "0".to_string(), // input /programs/9/program/functions/0/parameters/0/name
                            sort: SortTag::UInt, // input /programs/9/program/functions/0/parameters/0/sort
                        },
                    ],
                    return_type: SortTag::Bool, // input /programs/9/program/functions/0/returnType
                    body: CoreExpr::If {
                        cond: Box::new(CoreExpr::Binary {
                            op: BinaryOp::Eq, // input /programs/9/program/functions/0/body/cond/op
                            left: Box::new(CoreExpr::Var {
                                name: "0".to_string(), // input /programs/9/program/functions/0/body/cond/left/name
                            }),
                            right: Box::new(CoreExpr::Literal {
                                value: CoreValue::UInt {
                                    n: KIntC::InRange(0), // input /programs/9/program/functions/0/body/cond/right/value/n/tag
                                },
                            }),
                        }),
                        then_expr: Box::new(CoreExpr::Literal {
                            value: CoreValue::Bool {
                                b: true, // input /programs/9/program/functions/0/body/thenExpr/value/b
                            },
                        }),
                        else_expr: Box::new(CoreExpr::Call {
                            name: "7".to_string(), // input /programs/9/program/functions/0/body/elseExpr/name
                            args: vec![
                                CoreExpr::Binary {
                                    op: BinaryOp::Sub, // input /programs/9/program/functions/0/body/elseExpr/args/0/op
                                    left: Box::new(CoreExpr::Var {
                                        name: "0".to_string(), // input /programs/9/program/functions/0/body/elseExpr/args/0/left/name
                                    }),
                                    right: Box::new(CoreExpr::Literal {
                                        value: CoreValue::UInt {
                                            n: KIntC::InRange(1), // input /programs/9/program/functions/0/body/elseExpr/args/0/right/value/n/tag
                                        },
                                    }),
                                },
                            ],
                        }),
                    },
                },
                CoreFunction {
                    name: "7".to_string(), // input /programs/9/program/functions/1/name
                    parameters: vec![
                        CoreParameter {
                            name: "0".to_string(), // input /programs/9/program/functions/1/parameters/0/name
                            sort: SortTag::UInt, // input /programs/9/program/functions/1/parameters/0/sort
                        },
                    ],
                    return_type: SortTag::Bool, // input /programs/9/program/functions/1/returnType
                    body: CoreExpr::If {
                        cond: Box::new(CoreExpr::Binary {
                            op: BinaryOp::Eq, // input /programs/9/program/functions/1/body/cond/op
                            left: Box::new(CoreExpr::Var {
                                name: "0".to_string(), // input /programs/9/program/functions/1/body/cond/left/name
                            }),
                            right: Box::new(CoreExpr::Literal {
                                value: CoreValue::UInt {
                                    n: KIntC::InRange(0), // input /programs/9/program/functions/1/body/cond/right/value/n/tag
                                },
                            }),
                        }),
                        then_expr: Box::new(CoreExpr::Literal {
                            value: CoreValue::Bool {
                                b: false, // input /programs/9/program/functions/1/body/thenExpr/value/b
                            },
                        }),
                        else_expr: Box::new(CoreExpr::Call {
                            name: "6".to_string(), // input /programs/9/program/functions/1/body/elseExpr/name
                            args: vec![
                                CoreExpr::Binary {
                                    op: BinaryOp::Sub, // input /programs/9/program/functions/1/body/elseExpr/args/0/op
                                    left: Box::new(CoreExpr::Var {
                                        name: "0".to_string(), // input /programs/9/program/functions/1/body/elseExpr/args/0/left/name
                                    }),
                                    right: Box::new(CoreExpr::Literal {
                                        value: CoreValue::UInt {
                                            n: KIntC::InRange(1), // input /programs/9/program/functions/1/body/elseExpr/args/0/right/value/n/tag
                                        },
                                    }),
                                },
                            ],
                        }),
                    },
                },
            ],
            entry: CoreExpr::Call {
                name: "6".to_string(), // input /programs/9/program/entry/name
                args: vec![
                    CoreExpr::Literal {
                        value: CoreValue::UInt {
                            n: KIntC::InRange(4), // input /programs/9/program/entry/args/0/value/n/tag
                        },
                    },
                ],
            },
        }),
        "D09" => Some(CoreProgram {
            functions: vec![
            ],
            entry: CoreExpr::Binary {
                op: BinaryOp::Eq, // structural input /0/sourceAST/entry/op
                left: Box::new(CoreExpr::Literal {
                    value: CoreValue::UInt {
                        n: KIntC::InRange(3), // structural input /0/sourceAST/entry/left/value/n/tag
                    },
                }),
                right: Box::new(CoreExpr::Literal {
                    value: CoreValue::UInt {
                        n: KIntC::InRange(3), // structural input /0/sourceAST/entry/right/value/n/tag
                    },
                }),
            },
        }),
        "D13" => Some(CoreProgram {
            functions: vec![
                CoreFunction {
                    name: "sum".to_string(), // structural input /1/sourceAST/functions/0/name
                    parameters: vec![
                        CoreParameter {
                            name: "xs".to_string(), // structural input /1/sourceAST/functions/0/parameters/0/name
                            sort: SortTag::List, // structural input /1/sourceAST/functions/0/parameters/0/sort
                        },
                    ],
                    return_type: SortTag::UInt, // structural input /1/sourceAST/functions/0/returnType
                    body: CoreExpr::Match {
                        scrutinee: Box::new(CoreExpr::Var {
                            name: "xs".to_string(), // structural input /1/sourceAST/functions/0/body/scrutinee/name
                        }),
                        arms: vec![
                            CoreArm {
                                pattern: CorePattern::PNil,
                                body: CoreExpr::Literal {
                                    value: CoreValue::UInt {
                                        n: KIntC::InRange(0), // structural input /1/sourceAST/functions/0/body/arms/0/body/value/n/tag
                                    },
                                },
                            },
                            CoreArm {
                                pattern: CorePattern::PCons {
                                    x: "h".to_string(), // structural input /1/sourceAST/functions/0/body/arms/1/pattern/x
                                    y: "t".to_string(), // structural input /1/sourceAST/functions/0/body/arms/1/pattern/y
                                },
                                body: CoreExpr::Binary {
                                    op: BinaryOp::Add, // structural input /1/sourceAST/functions/0/body/arms/1/body/op
                                    left: Box::new(CoreExpr::Var {
                                        name: "h".to_string(), // structural input /1/sourceAST/functions/0/body/arms/1/body/left/name
                                    }),
                                    right: Box::new(CoreExpr::Call {
                                        name: "sum".to_string(), // structural input /1/sourceAST/functions/0/body/arms/1/body/right/name
                                        args: vec![
                                            CoreExpr::Var {
                                                name: "t".to_string(), // structural input /1/sourceAST/functions/0/body/arms/1/body/right/args/0/name
                                            },
                                        ],
                                    }),
                                },
                            },
                        ],
                    },
                },
            ],
            entry: CoreExpr::Call {
                name: "sum".to_string(), // structural input /1/sourceAST/entry/name
                args: vec![
                    CoreExpr::Literal {
                        value: CoreValue::List {
                            elements: vec![],
                        },
                    },
                ],
            },
        }),
        "TREE-EQ-REPEATED" => Some(CoreProgram {
            functions: vec![
            ],
            entry: CoreExpr::Binary {
                op: BinaryOp::Eq, // structural input /2/sourceAST/entry/op
                left: Box::new(CoreExpr::Literal {
                    value: CoreValue::Tree {
                        tree: CoreTree::Branch {
                            left: Box::new(CoreTree::Leaf {
                                n: KIntC::InRange(4), // structural input /2/sourceAST/entry/left/value/tree/left/n/tag
                            }),
                            right: Box::new(CoreTree::Leaf {
                                n: KIntC::InRange(4), // structural input /2/sourceAST/entry/left/value/tree/right/n/tag
                            }),
                        },
                    },
                }),
                right: Box::new(CoreExpr::Literal {
                    value: CoreValue::Tree {
                        tree: CoreTree::Branch {
                            left: Box::new(CoreTree::Leaf {
                                n: KIntC::InRange(4), // structural input /2/sourceAST/entry/right/value/tree/left/n/tag
                            }),
                            right: Box::new(CoreTree::Leaf {
                                n: KIntC::InRange(4), // structural input /2/sourceAST/entry/right/value/tree/right/n/tag
                            }),
                        },
                    },
                }),
            },
        }),
        _ => None,
    }
}

pub fn bounds(case_id: &str) -> Option<BoundsC> {
    match case_id {
        "D01" => Some(BoundsC {
            code_rows: 8, // input /programs/0/bounds/codeRows
            function_rows: 0, // input /programs/0/bounds/functionRows
            code_child_slots: 0, // input /programs/0/bounds/codeChildSlots
            parameter_slots: 0, // input /programs/0/bounds/parameterSlots
            pattern_rows: 0, // input /programs/0/bounds/patternRows
            match_plan_rows: 0, // input /programs/0/bounds/matchPlanRows
            match_arm_rows: 0, // input /programs/0/bounds/matchArmRows
            type_rows: 0, // input /programs/0/bounds/typeRows
            constructor_rows: 0, // input /programs/0/bounds/constructorRows
            constructor_field_slots: 0, // input /programs/0/bounds/constructorFieldSlots
            max_arity: 2, // input /programs/0/bounds/maxArity
            constant_nodes: 4, // input /programs/0/bounds/constantNodes
            constant_field_slots: 0, // input /programs/0/bounds/constantFieldSlots
            data_nodes: 4, // input /programs/0/bounds/dataNodes
            data_field_slots: 0, // input /programs/0/bounds/dataFieldSlots
            environment_cells: 2, // input /programs/0/bounds/environmentCells
            frames: 4, // input /programs/0/bounds/frames
            helper_tasks: 2, // input /programs/0/bounds/helperTasks
            serialization_bytes: 0, // input /programs/0/bounds/serializationBytes
            max_reference_work: 64, // input /programs/0/bounds/maxReferenceWork
            max_helper_work: 64, // input /programs/0/bounds/maxHelperWork
            step_cap: 1, // input /programs/0/bounds/stepCap
        }),
        "D02" => Some(BoundsC {
            code_rows: 8, // input /programs/1/bounds/codeRows
            function_rows: 0, // input /programs/1/bounds/functionRows
            code_child_slots: 0, // input /programs/1/bounds/codeChildSlots
            parameter_slots: 0, // input /programs/1/bounds/parameterSlots
            pattern_rows: 0, // input /programs/1/bounds/patternRows
            match_plan_rows: 0, // input /programs/1/bounds/matchPlanRows
            match_arm_rows: 0, // input /programs/1/bounds/matchArmRows
            type_rows: 0, // input /programs/1/bounds/typeRows
            constructor_rows: 0, // input /programs/1/bounds/constructorRows
            constructor_field_slots: 0, // input /programs/1/bounds/constructorFieldSlots
            max_arity: 2, // input /programs/1/bounds/maxArity
            constant_nodes: 4, // input /programs/1/bounds/constantNodes
            constant_field_slots: 0, // input /programs/1/bounds/constantFieldSlots
            data_nodes: 4, // input /programs/1/bounds/dataNodes
            data_field_slots: 0, // input /programs/1/bounds/dataFieldSlots
            environment_cells: 2, // input /programs/1/bounds/environmentCells
            frames: 4, // input /programs/1/bounds/frames
            helper_tasks: 2, // input /programs/1/bounds/helperTasks
            serialization_bytes: 0, // input /programs/1/bounds/serializationBytes
            max_reference_work: 64, // input /programs/1/bounds/maxReferenceWork
            max_helper_work: 64, // input /programs/1/bounds/maxHelperWork
            step_cap: 1, // input /programs/1/bounds/stepCap
        }),
        "D03" => Some(BoundsC {
            code_rows: 8, // input /programs/2/bounds/codeRows
            function_rows: 0, // input /programs/2/bounds/functionRows
            code_child_slots: 0, // input /programs/2/bounds/codeChildSlots
            parameter_slots: 0, // input /programs/2/bounds/parameterSlots
            pattern_rows: 0, // input /programs/2/bounds/patternRows
            match_plan_rows: 0, // input /programs/2/bounds/matchPlanRows
            match_arm_rows: 0, // input /programs/2/bounds/matchArmRows
            type_rows: 0, // input /programs/2/bounds/typeRows
            constructor_rows: 0, // input /programs/2/bounds/constructorRows
            constructor_field_slots: 0, // input /programs/2/bounds/constructorFieldSlots
            max_arity: 2, // input /programs/2/bounds/maxArity
            constant_nodes: 4, // input /programs/2/bounds/constantNodes
            constant_field_slots: 0, // input /programs/2/bounds/constantFieldSlots
            data_nodes: 4, // input /programs/2/bounds/dataNodes
            data_field_slots: 0, // input /programs/2/bounds/dataFieldSlots
            environment_cells: 2, // input /programs/2/bounds/environmentCells
            frames: 4, // input /programs/2/bounds/frames
            helper_tasks: 2, // input /programs/2/bounds/helperTasks
            serialization_bytes: 0, // input /programs/2/bounds/serializationBytes
            max_reference_work: 64, // input /programs/2/bounds/maxReferenceWork
            max_helper_work: 64, // input /programs/2/bounds/maxHelperWork
            step_cap: 1, // input /programs/2/bounds/stepCap
        }),
        "D04" => Some(BoundsC {
            code_rows: 8, // input /programs/3/bounds/codeRows
            function_rows: 0, // input /programs/3/bounds/functionRows
            code_child_slots: 0, // input /programs/3/bounds/codeChildSlots
            parameter_slots: 0, // input /programs/3/bounds/parameterSlots
            pattern_rows: 0, // input /programs/3/bounds/patternRows
            match_plan_rows: 0, // input /programs/3/bounds/matchPlanRows
            match_arm_rows: 0, // input /programs/3/bounds/matchArmRows
            type_rows: 0, // input /programs/3/bounds/typeRows
            constructor_rows: 0, // input /programs/3/bounds/constructorRows
            constructor_field_slots: 0, // input /programs/3/bounds/constructorFieldSlots
            max_arity: 2, // input /programs/3/bounds/maxArity
            constant_nodes: 4, // input /programs/3/bounds/constantNodes
            constant_field_slots: 0, // input /programs/3/bounds/constantFieldSlots
            data_nodes: 4, // input /programs/3/bounds/dataNodes
            data_field_slots: 0, // input /programs/3/bounds/dataFieldSlots
            environment_cells: 2, // input /programs/3/bounds/environmentCells
            frames: 4, // input /programs/3/bounds/frames
            helper_tasks: 2, // input /programs/3/bounds/helperTasks
            serialization_bytes: 0, // input /programs/3/bounds/serializationBytes
            max_reference_work: 64, // input /programs/3/bounds/maxReferenceWork
            max_helper_work: 64, // input /programs/3/bounds/maxHelperWork
            step_cap: 1, // input /programs/3/bounds/stepCap
        }),
        "D05" => Some(BoundsC {
            code_rows: 32, // input /programs/4/bounds/codeRows
            function_rows: 4, // input /programs/4/bounds/functionRows
            code_child_slots: 8, // input /programs/4/bounds/codeChildSlots
            parameter_slots: 8, // input /programs/4/bounds/parameterSlots
            pattern_rows: 0, // input /programs/4/bounds/patternRows
            match_plan_rows: 0, // input /programs/4/bounds/matchPlanRows
            match_arm_rows: 0, // input /programs/4/bounds/matchArmRows
            type_rows: 0, // input /programs/4/bounds/typeRows
            constructor_rows: 0, // input /programs/4/bounds/constructorRows
            constructor_field_slots: 0, // input /programs/4/bounds/constructorFieldSlots
            max_arity: 4, // input /programs/4/bounds/maxArity
            constant_nodes: 16, // input /programs/4/bounds/constantNodes
            constant_field_slots: 0, // input /programs/4/bounds/constantFieldSlots
            data_nodes: 16, // input /programs/4/bounds/dataNodes
            data_field_slots: 0, // input /programs/4/bounds/dataFieldSlots
            environment_cells: 16, // input /programs/4/bounds/environmentCells
            frames: 8, // input /programs/4/bounds/frames
            helper_tasks: 8, // input /programs/4/bounds/helperTasks
            serialization_bytes: 0, // input /programs/4/bounds/serializationBytes
            max_reference_work: 512, // input /programs/4/bounds/maxReferenceWork
            max_helper_work: 512, // input /programs/4/bounds/maxHelperWork
            step_cap: 1, // input /programs/4/bounds/stepCap
        }),
        "D21" => Some(BoundsC {
            code_rows: 32, // input /programs/5/bounds/codeRows
            function_rows: 4, // input /programs/5/bounds/functionRows
            code_child_slots: 8, // input /programs/5/bounds/codeChildSlots
            parameter_slots: 8, // input /programs/5/bounds/parameterSlots
            pattern_rows: 0, // input /programs/5/bounds/patternRows
            match_plan_rows: 0, // input /programs/5/bounds/matchPlanRows
            match_arm_rows: 0, // input /programs/5/bounds/matchArmRows
            type_rows: 0, // input /programs/5/bounds/typeRows
            constructor_rows: 0, // input /programs/5/bounds/constructorRows
            constructor_field_slots: 0, // input /programs/5/bounds/constructorFieldSlots
            max_arity: 4, // input /programs/5/bounds/maxArity
            constant_nodes: 16, // input /programs/5/bounds/constantNodes
            constant_field_slots: 0, // input /programs/5/bounds/constantFieldSlots
            data_nodes: 16, // input /programs/5/bounds/dataNodes
            data_field_slots: 0, // input /programs/5/bounds/dataFieldSlots
            environment_cells: 16, // input /programs/5/bounds/environmentCells
            frames: 8, // input /programs/5/bounds/frames
            helper_tasks: 8, // input /programs/5/bounds/helperTasks
            serialization_bytes: 0, // input /programs/5/bounds/serializationBytes
            max_reference_work: 512, // input /programs/5/bounds/maxReferenceWork
            max_helper_work: 512, // input /programs/5/bounds/maxHelperWork
            step_cap: 1, // input /programs/5/bounds/stepCap
        }),
        "D14" => Some(BoundsC {
            code_rows: 64, // input /programs/6/bounds/codeRows
            function_rows: 8, // input /programs/6/bounds/functionRows
            code_child_slots: 32, // input /programs/6/bounds/codeChildSlots
            parameter_slots: 16, // input /programs/6/bounds/parameterSlots
            pattern_rows: 16, // input /programs/6/bounds/patternRows
            match_plan_rows: 0, // input /programs/6/bounds/matchPlanRows
            match_arm_rows: 16, // input /programs/6/bounds/matchArmRows
            type_rows: 0, // input /programs/6/bounds/typeRows
            constructor_rows: 0, // input /programs/6/bounds/constructorRows
            constructor_field_slots: 0, // input /programs/6/bounds/constructorFieldSlots
            max_arity: 4, // input /programs/6/bounds/maxArity
            constant_nodes: 64, // input /programs/6/bounds/constantNodes
            constant_field_slots: 64, // input /programs/6/bounds/constantFieldSlots
            data_nodes: 64, // input /programs/6/bounds/dataNodes
            data_field_slots: 64, // input /programs/6/bounds/dataFieldSlots
            environment_cells: 64, // input /programs/6/bounds/environmentCells
            frames: 64, // input /programs/6/bounds/frames
            helper_tasks: 64, // input /programs/6/bounds/helperTasks
            serialization_bytes: 0, // input /programs/6/bounds/serializationBytes
            max_reference_work: 1000, // input /programs/6/bounds/maxReferenceWork
            max_helper_work: 1000, // input /programs/6/bounds/maxHelperWork
            step_cap: 1, // input /programs/6/bounds/stepCap
        }),
        "D16" => Some(BoundsC {
            code_rows: 64, // input /programs/7/bounds/codeRows
            function_rows: 8, // input /programs/7/bounds/functionRows
            code_child_slots: 32, // input /programs/7/bounds/codeChildSlots
            parameter_slots: 16, // input /programs/7/bounds/parameterSlots
            pattern_rows: 16, // input /programs/7/bounds/patternRows
            match_plan_rows: 0, // input /programs/7/bounds/matchPlanRows
            match_arm_rows: 16, // input /programs/7/bounds/matchArmRows
            type_rows: 0, // input /programs/7/bounds/typeRows
            constructor_rows: 0, // input /programs/7/bounds/constructorRows
            constructor_field_slots: 0, // input /programs/7/bounds/constructorFieldSlots
            max_arity: 4, // input /programs/7/bounds/maxArity
            constant_nodes: 64, // input /programs/7/bounds/constantNodes
            constant_field_slots: 64, // input /programs/7/bounds/constantFieldSlots
            data_nodes: 64, // input /programs/7/bounds/dataNodes
            data_field_slots: 64, // input /programs/7/bounds/dataFieldSlots
            environment_cells: 64, // input /programs/7/bounds/environmentCells
            frames: 64, // input /programs/7/bounds/frames
            helper_tasks: 64, // input /programs/7/bounds/helperTasks
            serialization_bytes: 0, // input /programs/7/bounds/serializationBytes
            max_reference_work: 1000, // input /programs/7/bounds/maxReferenceWork
            max_helper_work: 1000, // input /programs/7/bounds/maxHelperWork
            step_cap: 1, // input /programs/7/bounds/stepCap
        }),
        "D10" => Some(BoundsC {
            code_rows: 64, // input /programs/8/bounds/codeRows
            function_rows: 8, // input /programs/8/bounds/functionRows
            code_child_slots: 16, // input /programs/8/bounds/codeChildSlots
            parameter_slots: 16, // input /programs/8/bounds/parameterSlots
            pattern_rows: 0, // input /programs/8/bounds/patternRows
            match_plan_rows: 0, // input /programs/8/bounds/matchPlanRows
            match_arm_rows: 0, // input /programs/8/bounds/matchArmRows
            type_rows: 0, // input /programs/8/bounds/typeRows
            constructor_rows: 0, // input /programs/8/bounds/constructorRows
            constructor_field_slots: 0, // input /programs/8/bounds/constructorFieldSlots
            max_arity: 2, // input /programs/8/bounds/maxArity
            constant_nodes: 32, // input /programs/8/bounds/constantNodes
            constant_field_slots: 0, // input /programs/8/bounds/constantFieldSlots
            data_nodes: 64, // input /programs/8/bounds/dataNodes
            data_field_slots: 0, // input /programs/8/bounds/dataFieldSlots
            environment_cells: 32, // input /programs/8/bounds/environmentCells
            frames: 64, // input /programs/8/bounds/frames
            helper_tasks: 8, // input /programs/8/bounds/helperTasks
            serialization_bytes: 1048576, // input /programs/8/bounds/serializationBytes
            max_reference_work: 1000, // input /programs/8/bounds/maxReferenceWork
            max_helper_work: 1000, // input /programs/8/bounds/maxHelperWork
            step_cap: 1, // input /programs/8/bounds/stepCap
        }),
        "D12" => Some(BoundsC {
            code_rows: 64, // input /programs/9/bounds/codeRows
            function_rows: 8, // input /programs/9/bounds/functionRows
            code_child_slots: 16, // input /programs/9/bounds/codeChildSlots
            parameter_slots: 16, // input /programs/9/bounds/parameterSlots
            pattern_rows: 0, // input /programs/9/bounds/patternRows
            match_plan_rows: 0, // input /programs/9/bounds/matchPlanRows
            match_arm_rows: 0, // input /programs/9/bounds/matchArmRows
            type_rows: 0, // input /programs/9/bounds/typeRows
            constructor_rows: 0, // input /programs/9/bounds/constructorRows
            constructor_field_slots: 0, // input /programs/9/bounds/constructorFieldSlots
            max_arity: 2, // input /programs/9/bounds/maxArity
            constant_nodes: 32, // input /programs/9/bounds/constantNodes
            constant_field_slots: 0, // input /programs/9/bounds/constantFieldSlots
            data_nodes: 64, // input /programs/9/bounds/dataNodes
            data_field_slots: 0, // input /programs/9/bounds/dataFieldSlots
            environment_cells: 32, // input /programs/9/bounds/environmentCells
            frames: 64, // input /programs/9/bounds/frames
            helper_tasks: 8, // input /programs/9/bounds/helperTasks
            serialization_bytes: 1048576, // input /programs/9/bounds/serializationBytes
            max_reference_work: 1000, // input /programs/9/bounds/maxReferenceWork
            max_helper_work: 1000, // input /programs/9/bounds/maxHelperWork
            step_cap: 1, // input /programs/9/bounds/stepCap
        }),
        "D09" => Some(BoundsC {
            code_rows: 64, // structural input /0/bounds/codeRows
            function_rows: 8, // structural input /0/bounds/functionRows
            code_child_slots: 32, // structural input /0/bounds/codeChildSlots
            parameter_slots: 16, // structural input /0/bounds/parameterSlots
            pattern_rows: 16, // structural input /0/bounds/patternRows
            match_plan_rows: 0, // structural input /0/bounds/matchPlanRows
            match_arm_rows: 16, // structural input /0/bounds/matchArmRows
            type_rows: 0, // structural input /0/bounds/typeRows
            constructor_rows: 0, // structural input /0/bounds/constructorRows
            constructor_field_slots: 0, // structural input /0/bounds/constructorFieldSlots
            max_arity: 2, // structural input /0/bounds/maxArity
            constant_nodes: 32, // structural input /0/bounds/constantNodes
            constant_field_slots: 32, // structural input /0/bounds/constantFieldSlots
            data_nodes: 8, // structural input /0/bounds/dataNodes
            data_field_slots: 16, // structural input /0/bounds/dataFieldSlots
            environment_cells: 16, // structural input /0/bounds/environmentCells
            frames: 8, // structural input /0/bounds/frames
            helper_tasks: 8, // structural input /0/bounds/helperTasks
            serialization_bytes: 0, // structural input /0/bounds/serializationBytes
            max_reference_work: 128, // structural input /0/bounds/maxReferenceWork
            max_helper_work: 128, // structural input /0/bounds/maxHelperWork
            step_cap: 1, // structural input /0/bounds/stepCap
        }),
        "D13" => Some(BoundsC {
            code_rows: 64, // structural input /1/bounds/codeRows
            function_rows: 8, // structural input /1/bounds/functionRows
            code_child_slots: 32, // structural input /1/bounds/codeChildSlots
            parameter_slots: 16, // structural input /1/bounds/parameterSlots
            pattern_rows: 16, // structural input /1/bounds/patternRows
            match_plan_rows: 0, // structural input /1/bounds/matchPlanRows
            match_arm_rows: 16, // structural input /1/bounds/matchArmRows
            type_rows: 0, // structural input /1/bounds/typeRows
            constructor_rows: 0, // structural input /1/bounds/constructorRows
            constructor_field_slots: 0, // structural input /1/bounds/constructorFieldSlots
            max_arity: 2, // structural input /1/bounds/maxArity
            constant_nodes: 32, // structural input /1/bounds/constantNodes
            constant_field_slots: 32, // structural input /1/bounds/constantFieldSlots
            data_nodes: 8, // structural input /1/bounds/dataNodes
            data_field_slots: 16, // structural input /1/bounds/dataFieldSlots
            environment_cells: 16, // structural input /1/bounds/environmentCells
            frames: 8, // structural input /1/bounds/frames
            helper_tasks: 8, // structural input /1/bounds/helperTasks
            serialization_bytes: 0, // structural input /1/bounds/serializationBytes
            max_reference_work: 128, // structural input /1/bounds/maxReferenceWork
            max_helper_work: 128, // structural input /1/bounds/maxHelperWork
            step_cap: 1, // structural input /1/bounds/stepCap
        }),
        "TREE-EQ-REPEATED" => Some(BoundsC {
            code_rows: 64, // structural input /2/bounds/codeRows
            function_rows: 8, // structural input /2/bounds/functionRows
            code_child_slots: 32, // structural input /2/bounds/codeChildSlots
            parameter_slots: 16, // structural input /2/bounds/parameterSlots
            pattern_rows: 16, // structural input /2/bounds/patternRows
            match_plan_rows: 0, // structural input /2/bounds/matchPlanRows
            match_arm_rows: 16, // structural input /2/bounds/matchArmRows
            type_rows: 0, // structural input /2/bounds/typeRows
            constructor_rows: 0, // structural input /2/bounds/constructorRows
            constructor_field_slots: 0, // structural input /2/bounds/constructorFieldSlots
            max_arity: 2, // structural input /2/bounds/maxArity
            constant_nodes: 32, // structural input /2/bounds/constantNodes
            constant_field_slots: 32, // structural input /2/bounds/constantFieldSlots
            data_nodes: 8, // structural input /2/bounds/dataNodes
            data_field_slots: 16, // structural input /2/bounds/dataFieldSlots
            environment_cells: 16, // structural input /2/bounds/environmentCells
            frames: 8, // structural input /2/bounds/frames
            helper_tasks: 8, // structural input /2/bounds/helperTasks
            serialization_bytes: 0, // structural input /2/bounds/serializationBytes
            max_reference_work: 128, // structural input /2/bounds/maxReferenceWork
            max_helper_work: 128, // structural input /2/bounds/maxHelperWork
            step_cap: 1, // structural input /2/bounds/stepCap
        }),
        _ => None,
    }
}

pub fn layout(case_id: &str) -> Option<LayoutSchemaC> {
    match case_id {
        "D01" => Some(LayoutSchemaC {
            layout_version: (
                "oracle.scalar-env".to_string(), // input /programs/0/layout/layoutVersion/namespace
                "host-literal-1".to_string(), // input /programs/0/layout/layoutVersion/name
            ),
            field_id: None, // input /programs/0/layout/fieldId
            widths: None, // input /programs/0/layout/widths
            scalar_layout_id: None, // input /programs/0/layout/scalarLayoutId
            bounds: bounds(case_id)?, // exact duplicate of this case layout.bounds
            capacities: LayoutCapacitiesC {
                name_rows: 1, // input /programs/0/layout/capacities/nameRows
                name_bytes: 1, // input /programs/0/layout/capacities/nameBytes
                occurrence_path_slots: 8, // input /programs/0/layout/capacities/occurrencePathSlots
                value_slots: 64, // input /programs/0/layout/capacities/valueSlots
            },
            boundary_capacities_present: false, // input /programs/0/layout/boundaryCapacities
        }),
        "D02" => Some(LayoutSchemaC {
            layout_version: (
                "oracle.scalar-env".to_string(), // input /programs/1/layout/layoutVersion/namespace
                "host-literal-1".to_string(), // input /programs/1/layout/layoutVersion/name
            ),
            field_id: None, // input /programs/1/layout/fieldId
            widths: None, // input /programs/1/layout/widths
            scalar_layout_id: None, // input /programs/1/layout/scalarLayoutId
            bounds: bounds(case_id)?, // exact duplicate of this case layout.bounds
            capacities: LayoutCapacitiesC {
                name_rows: 1, // input /programs/1/layout/capacities/nameRows
                name_bytes: 1, // input /programs/1/layout/capacities/nameBytes
                occurrence_path_slots: 8, // input /programs/1/layout/capacities/occurrencePathSlots
                value_slots: 64, // input /programs/1/layout/capacities/valueSlots
            },
            boundary_capacities_present: false, // input /programs/1/layout/boundaryCapacities
        }),
        "D03" => Some(LayoutSchemaC {
            layout_version: (
                "oracle.scalar-env".to_string(), // input /programs/2/layout/layoutVersion/namespace
                "host-literal-1".to_string(), // input /programs/2/layout/layoutVersion/name
            ),
            field_id: None, // input /programs/2/layout/fieldId
            widths: None, // input /programs/2/layout/widths
            scalar_layout_id: None, // input /programs/2/layout/scalarLayoutId
            bounds: bounds(case_id)?, // exact duplicate of this case layout.bounds
            capacities: LayoutCapacitiesC {
                name_rows: 1, // input /programs/2/layout/capacities/nameRows
                name_bytes: 1, // input /programs/2/layout/capacities/nameBytes
                occurrence_path_slots: 8, // input /programs/2/layout/capacities/occurrencePathSlots
                value_slots: 64, // input /programs/2/layout/capacities/valueSlots
            },
            boundary_capacities_present: false, // input /programs/2/layout/boundaryCapacities
        }),
        "D04" => Some(LayoutSchemaC {
            layout_version: (
                "oracle.scalar-env".to_string(), // input /programs/3/layout/layoutVersion/namespace
                "host-literal-1".to_string(), // input /programs/3/layout/layoutVersion/name
            ),
            field_id: None, // input /programs/3/layout/fieldId
            widths: None, // input /programs/3/layout/widths
            scalar_layout_id: None, // input /programs/3/layout/scalarLayoutId
            bounds: bounds(case_id)?, // exact duplicate of this case layout.bounds
            capacities: LayoutCapacitiesC {
                name_rows: 1, // input /programs/3/layout/capacities/nameRows
                name_bytes: 1, // input /programs/3/layout/capacities/nameBytes
                occurrence_path_slots: 8, // input /programs/3/layout/capacities/occurrencePathSlots
                value_slots: 64, // input /programs/3/layout/capacities/valueSlots
            },
            boundary_capacities_present: false, // input /programs/3/layout/boundaryCapacities
        }),
        "D05" => Some(LayoutSchemaC {
            layout_version: (
                "independent-normal-host-oracle".to_string(), // input /programs/4/layout/layoutVersion/namespace
                "call-bind-01".to_string(), // input /programs/4/layout/layoutVersion/name
            ),
            field_id: None, // input /programs/4/layout/fieldId
            widths: None, // input /programs/4/layout/widths
            scalar_layout_id: None, // input /programs/4/layout/scalarLayoutId
            bounds: bounds(case_id)?, // exact duplicate of this case layout.bounds
            capacities: LayoutCapacitiesC {
                name_rows: 8, // input /programs/4/layout/capacities/nameRows
                name_bytes: 64, // input /programs/4/layout/capacities/nameBytes
                occurrence_path_slots: 128, // input /programs/4/layout/capacities/occurrencePathSlots
                value_slots: 512, // input /programs/4/layout/capacities/valueSlots
            },
            boundary_capacities_present: false, // input /programs/4/layout/boundaryCapacities
        }),
        "D21" => Some(LayoutSchemaC {
            layout_version: (
                "independent-normal-host-oracle".to_string(), // input /programs/5/layout/layoutVersion/namespace
                "call-bind-01".to_string(), // input /programs/5/layout/layoutVersion/name
            ),
            field_id: None, // input /programs/5/layout/fieldId
            widths: None, // input /programs/5/layout/widths
            scalar_layout_id: None, // input /programs/5/layout/scalarLayoutId
            bounds: bounds(case_id)?, // exact duplicate of this case layout.bounds
            capacities: LayoutCapacitiesC {
                name_rows: 8, // input /programs/5/layout/capacities/nameRows
                name_bytes: 64, // input /programs/5/layout/capacities/nameBytes
                occurrence_path_slots: 128, // input /programs/5/layout/capacities/occurrencePathSlots
                value_slots: 512, // input /programs/5/layout/capacities/valueSlots
            },
            boundary_capacities_present: false, // input /programs/5/layout/boundaryCapacities
        }),
        "D14" => Some(LayoutSchemaC {
            layout_version: (
                "sherlock-normal-oracle".to_string(), // input /programs/6/layout/layoutVersion/namespace
                "host-recursive-data-01".to_string(), // input /programs/6/layout/layoutVersion/name
            ),
            field_id: None, // input /programs/6/layout/fieldId
            widths: None, // input /programs/6/layout/widths
            scalar_layout_id: None, // input /programs/6/layout/scalarLayoutId
            bounds: bounds(case_id)?, // exact duplicate of this case layout.bounds
            capacities: LayoutCapacitiesC {
                name_rows: 16, // input /programs/6/layout/capacities/nameRows
                name_bytes: 128, // input /programs/6/layout/capacities/nameBytes
                occurrence_path_slots: 128, // input /programs/6/layout/capacities/occurrencePathSlots
                value_slots: 1000, // input /programs/6/layout/capacities/valueSlots
            },
            boundary_capacities_present: false, // input /programs/6/layout/boundaryCapacities
        }),
        "D16" => Some(LayoutSchemaC {
            layout_version: (
                "sherlock-normal-oracle".to_string(), // input /programs/7/layout/layoutVersion/namespace
                "host-recursive-data-01".to_string(), // input /programs/7/layout/layoutVersion/name
            ),
            field_id: None, // input /programs/7/layout/fieldId
            widths: None, // input /programs/7/layout/widths
            scalar_layout_id: None, // input /programs/7/layout/scalarLayoutId
            bounds: bounds(case_id)?, // exact duplicate of this case layout.bounds
            capacities: LayoutCapacitiesC {
                name_rows: 16, // input /programs/7/layout/capacities/nameRows
                name_bytes: 128, // input /programs/7/layout/capacities/nameBytes
                occurrence_path_slots: 128, // input /programs/7/layout/capacities/occurrencePathSlots
                value_slots: 1000, // input /programs/7/layout/capacities/valueSlots
            },
            boundary_capacities_present: false, // input /programs/7/layout/boundaryCapacities
        }),
        "D10" => Some(LayoutSchemaC {
            layout_version: (
                "sherlock-oracle".to_string(), // input /programs/8/layout/layoutVersion/namespace
                "recursive-host-literal-01".to_string(), // input /programs/8/layout/layoutVersion/name
            ),
            field_id: None, // input /programs/8/layout/fieldId
            widths: None, // input /programs/8/layout/widths
            scalar_layout_id: None, // input /programs/8/layout/scalarLayoutId
            bounds: bounds(case_id)?, // exact duplicate of this case layout.bounds
            capacities: LayoutCapacitiesC {
                name_rows: 8, // input /programs/8/layout/capacities/nameRows
                name_bytes: 32, // input /programs/8/layout/capacities/nameBytes
                occurrence_path_slots: 128, // input /programs/8/layout/capacities/occurrencePathSlots
                value_slots: 1000, // input /programs/8/layout/capacities/valueSlots
            },
            boundary_capacities_present: false, // input /programs/8/layout/boundaryCapacities
        }),
        "D12" => Some(LayoutSchemaC {
            layout_version: (
                "sherlock-oracle".to_string(), // input /programs/9/layout/layoutVersion/namespace
                "recursive-host-literal-01".to_string(), // input /programs/9/layout/layoutVersion/name
            ),
            field_id: None, // input /programs/9/layout/fieldId
            widths: None, // input /programs/9/layout/widths
            scalar_layout_id: None, // input /programs/9/layout/scalarLayoutId
            bounds: bounds(case_id)?, // exact duplicate of this case layout.bounds
            capacities: LayoutCapacitiesC {
                name_rows: 8, // input /programs/9/layout/capacities/nameRows
                name_bytes: 32, // input /programs/9/layout/capacities/nameBytes
                occurrence_path_slots: 128, // input /programs/9/layout/capacities/occurrencePathSlots
                value_slots: 1000, // input /programs/9/layout/capacities/valueSlots
            },
            boundary_capacities_present: false, // input /programs/9/layout/boundaryCapacities
        }),
        "D09" => Some(LayoutSchemaC {
            layout_version: (
                "sherlock-oracle".to_string(), // structural input /0/layout/layoutVersion/namespace
                "structural-host-01".to_string(), // structural input /0/layout/layoutVersion/name
            ),
            field_id: None, // structural input /0/layout/fieldId
            widths: None, // structural input /0/layout/widths
            scalar_layout_id: None, // structural input /0/layout/scalarLayoutId
            bounds: bounds(case_id)?, // exact duplicate of this structural case layout.bounds
            capacities: LayoutCapacitiesC {
                name_rows: 16, // structural input /0/layout/capacities/nameRows
                name_bytes: 128, // structural input /0/layout/capacities/nameBytes
                occurrence_path_slots: 128, // structural input /0/layout/capacities/occurrencePathSlots
                value_slots: 128, // structural input /0/layout/capacities/valueSlots
            },
            boundary_capacities_present: false, // structural input /0/layout/boundaryCapacities
        }),
        "D13" => Some(LayoutSchemaC {
            layout_version: (
                "sherlock-oracle".to_string(), // structural input /1/layout/layoutVersion/namespace
                "structural-host-01".to_string(), // structural input /1/layout/layoutVersion/name
            ),
            field_id: None, // structural input /1/layout/fieldId
            widths: None, // structural input /1/layout/widths
            scalar_layout_id: None, // structural input /1/layout/scalarLayoutId
            bounds: bounds(case_id)?, // exact duplicate of this structural case layout.bounds
            capacities: LayoutCapacitiesC {
                name_rows: 16, // structural input /1/layout/capacities/nameRows
                name_bytes: 128, // structural input /1/layout/capacities/nameBytes
                occurrence_path_slots: 128, // structural input /1/layout/capacities/occurrencePathSlots
                value_slots: 128, // structural input /1/layout/capacities/valueSlots
            },
            boundary_capacities_present: false, // structural input /1/layout/boundaryCapacities
        }),
        "TREE-EQ-REPEATED" => Some(LayoutSchemaC {
            layout_version: (
                "sherlock-oracle".to_string(), // structural input /2/layout/layoutVersion/namespace
                "structural-host-01".to_string(), // structural input /2/layout/layoutVersion/name
            ),
            field_id: None, // structural input /2/layout/fieldId
            widths: None, // structural input /2/layout/widths
            scalar_layout_id: None, // structural input /2/layout/scalarLayoutId
            bounds: bounds(case_id)?, // exact duplicate of this structural case layout.bounds
            capacities: LayoutCapacitiesC {
                name_rows: 16, // structural input /2/layout/capacities/nameRows
                name_bytes: 128, // structural input /2/layout/capacities/nameBytes
                occurrence_path_slots: 128, // structural input /2/layout/capacities/occurrencePathSlots
                value_slots: 128, // structural input /2/layout/capacities/valueSlots
            },
            boundary_capacities_present: false, // structural input /2/layout/boundaryCapacities
        }),
        _ => None,
    }
}
