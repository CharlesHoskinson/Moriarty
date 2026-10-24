//! Unregistered host AST bridge for the current dedicated-sort source carriers.
//!
//! Root owns registration and the existing cfg(test)-only / no Stage-2 seam.
//! This copies AST shape, not execution state, profiles, budgets or provenance.
//! A and B have distinct helper/fuel schedules; no behavioral qualification is
//! implied. Construction is iterative but host allocation is not admitted or
//! metered here. Owner-provided CoreExpr/CoreTree destruction remains separate.

use crate::sherlock as a;
use super::shared as b;
use b::i128_class::KIntC;

/// Copy every current A AST constructor into its B dedicated-sort carrier.
///
/// Pass `parsed.program.as_ref()` for an existing `ParsedDraft`. Draft name,
/// source spans and provenance stay with that caller: CoreProgram has no such
/// fields. No parser, evaluator, admission, or machine rejection is involved.
/// All current A variants are representable, so this conversion is total over
/// those finite source ASTs and has no invented Unsupported/error outcome.
/// Shared source Arcs are visited once per structural occurrence, never by
/// pointer identity. Calls remain named Call nodes, including recursive calls.
pub fn from_program(source: &a::Program) -> b::CoreProgram {
    let mut destination = b::CoreProgram {
        functions: source.functions.iter().map(|function| b::CoreFunction {
            name: function.name.clone(),
            parameters: function.parameters.iter().map(|parameter| b::CoreParameter {
                name: parameter.name.clone(),
                sort: sort(parameter.ty),
            }).collect(),
            return_type: sort(function.return_type),
            body: expression_slot(),
        }).collect(),
        entry: expression_slot(),
    };

    // Each queued item owns an exclusive destination slot. Empty slots are
    // replaced before publication; no unfinished AST escapes this function.
    let mut pending = Vec::new();
    for (input, output) in source.functions.iter()
        .zip(destination.functions.iter_mut()).rev()
    {
        pending.push(ExpressionCopy { input: input.body.as_ref(), output: &mut output.body });
    }
    pending.push(ExpressionCopy { input: source.entry.as_ref(), output: &mut destination.entry });

    // LIFO: entry preorder, followed by bodies in source function-table order.
    while let Some(ExpressionCopy { input, output }) = pending.pop() {
        match input {
            a::Expr::Literal(input) => {
                *output = b::CoreExpr::Literal { value: value(input) };
            }
            a::Expr::Var(name) => {
                *output = b::CoreExpr::Var { name: name.clone() };
            }
            a::Expr::Let(name, input_value, input_body) => {
                *output = b::CoreExpr::Let {
                    name: name.clone(),
                    value: Box::new(expression_slot()),
                    body: Box::new(expression_slot()),
                };
                match output {
                    b::CoreExpr::Let { value, body, .. } => {
                        pending.push(ExpressionCopy { input: input_body.as_ref(), output: body.as_mut() });
                        pending.push(ExpressionCopy { input: input_value.as_ref(), output: value.as_mut() });
                    }
                    _ => unreachable!("fresh Let construction"),
                }
            }
            a::Expr::Binary(op, input_left, input_right) => {
                *output = b::CoreExpr::Binary {
                    op: binary(*op),
                    left: Box::new(expression_slot()),
                    right: Box::new(expression_slot()),
                };
                match output {
                    b::CoreExpr::Binary { left, right, .. } => {
                        pending.push(ExpressionCopy { input: input_right.as_ref(), output: right.as_mut() });
                        pending.push(ExpressionCopy { input: input_left.as_ref(), output: left.as_mut() });
                    }
                    _ => unreachable!("fresh Binary construction"),
                }
            }
            a::Expr::Unary(op, input_operand) => {
                *output = b::CoreExpr::Unary {
                    op: unary(*op),
                    operand: Box::new(expression_slot()),
                };
                match output {
                    b::CoreExpr::Unary { operand, .. } => {
                        pending.push(ExpressionCopy { input: input_operand.as_ref(), output: operand.as_mut() });
                    }
                    _ => unreachable!("fresh Unary construction"),
                }
            }
            a::Expr::If(input_cond, input_then, input_else) => {
                *output = b::CoreExpr::If {
                    cond: Box::new(expression_slot()),
                    then_expr: Box::new(expression_slot()),
                    else_expr: Box::new(expression_slot()),
                };
                match output {
                    b::CoreExpr::If { cond, then_expr, else_expr } => {
                        pending.push(ExpressionCopy { input: input_else.as_ref(), output: else_expr.as_mut() });
                        pending.push(ExpressionCopy { input: input_then.as_ref(), output: then_expr.as_mut() });
                        pending.push(ExpressionCopy { input: input_cond.as_ref(), output: cond.as_mut() });
                    }
                    _ => unreachable!("fresh If construction"),
                }
            }
            a::Expr::Call(name, input_args) => {
                *output = b::CoreExpr::Call {
                    name: name.clone(),
                    args: input_args.iter().map(|_| expression_slot()).collect(),
                };
                match output {
                    b::CoreExpr::Call { args, .. } => {
                        for (input, output) in input_args.iter().zip(args.iter_mut()).rev() {
                            pending.push(ExpressionCopy { input: input.as_ref(), output });
                        }
                    }
                    _ => unreachable!("fresh Call construction"),
                }
            }
            a::Expr::Match(input_scrutinee, input_arms) => {
                *output = b::CoreExpr::Match {
                    scrutinee: Box::new(expression_slot()),
                    arms: input_arms.iter().map(|arm| b::CoreArm {
                        pattern: pattern(&arm.pattern),
                        body: expression_slot(),
                    }).collect(),
                };
                match output {
                    b::CoreExpr::Match { scrutinee, arms } => {
                        for (input, output) in input_arms.iter().zip(arms.iter_mut()).rev() {
                            pending.push(ExpressionCopy { input: input.body.as_ref(), output: &mut output.body });
                        }
                        pending.push(ExpressionCopy { input: input_scrutinee.as_ref(), output: scrutinee.as_mut() });
                    }
                    _ => unreachable!("fresh Match construction"),
                }
            }
        }
    }
    drop(pending);
    destination
}

struct ExpressionCopy<'source, 'destination> {
    input: &'source a::Expr,
    output: &'destination mut b::CoreExpr,
}

// Construction-only empty slots have no children and never leave the bridge.
fn expression_slot() -> b::CoreExpr {
    b::CoreExpr::Literal { value: b::CoreValue::Bool { b: false } }
}

fn sort(input: a::Type) -> b::SortTag {
    match input {
        a::Type::UInt => b::SortTag::UInt,
        a::Type::Bool => b::SortTag::Bool,
        a::Type::List => b::SortTag::List,
        a::Type::Tree => b::SortTag::Tree,
    }
}

fn binary(input: a::BinaryOp) -> b::BinaryOp {
    match input {
        a::BinaryOp::Add => b::BinaryOp::Add,
        a::BinaryOp::Sub => b::BinaryOp::Sub,
        a::BinaryOp::Mul => b::BinaryOp::Mul,
        a::BinaryOp::Eq => b::BinaryOp::Eq,
        a::BinaryOp::Lt => b::BinaryOp::Lt,
        a::BinaryOp::Cons => b::BinaryOp::Cons,
        a::BinaryOp::Branch => b::BinaryOp::Branch,
    }
}

fn unary(input: a::UnaryOp) -> b::UnaryOp {
    match input {
        a::UnaryOp::Leaf => b::UnaryOp::Leaf,
        a::UnaryOp::Not => b::UnaryOp::Not,
    }
}

fn pattern(input: &a::Pattern) -> b::CorePattern {
    match input {
        a::Pattern::Var(x) => b::CorePattern::PVar { x: x.clone() },
        a::Pattern::Nil => b::CorePattern::PNil,
        a::Pattern::Cons(x, y) => b::CorePattern::PCons { x: x.clone(), y: y.clone() },
        a::Pattern::Leaf(x) => b::CorePattern::PLeaf { x: x.clone() },
        a::Pattern::Branch(x, y) => b::CorePattern::PBranch { x: x.clone(), y: y.clone() },
    }
}

fn value(input: &a::Value) -> b::CoreValue {
    match input {
        a::Value::UInt(n) => b::CoreValue::UInt { n: KIntC::InRange(*n) },
        a::Value::Bool(input) => b::CoreValue::Bool { b: *input },
        a::Value::List(elements) => b::CoreValue::List {
            elements: elements.iter().copied().map(KIntC::InRange).collect(),
        },
        a::Value::Tree(input) => b::CoreValue::Tree { tree: tree(input) },
    }
}

fn tree(input: &a::Tree) -> b::CoreTree {
    let mut destination = tree_slot();
    let mut pending = vec![TreeCopy { input, output: &mut destination }];
    while let Some(TreeCopy { input, output }) = pending.pop() {
        match input {
            a::Tree::Leaf(n) => {
                *output = b::CoreTree::Leaf { n: KIntC::InRange(*n) };
            }
            a::Tree::Branch(input_left, input_right) => {
                *output = b::CoreTree::Branch {
                    left: Box::new(tree_slot()),
                    right: Box::new(tree_slot()),
                };
                match output {
                    b::CoreTree::Branch { left, right } => {
                        pending.push(TreeCopy { input: input_right.as_ref(), output: right.as_mut() });
                        pending.push(TreeCopy { input: input_left.as_ref(), output: left.as_mut() });
                    }
                    _ => unreachable!("fresh tree Branch construction"),
                }
            }
        }
    }
    drop(pending);
    destination
}

struct TreeCopy<'source, 'destination> {
    input: &'source a::Tree,
    output: &'destination mut b::CoreTree,
}

fn tree_slot() -> b::CoreTree {
    b::CoreTree::Leaf { n: KIntC::InRange(0) }
}
