//! Unregistered host AST bridge for the current dedicated-sort source carriers.
//!
//! Root owns registration and the existing cfg(test)-only / no Stage-2 seam.
//! This copies AST shape, not execution state, profiles, budgets or provenance.
//! A and B have distinct helper/fuel schedules; no behavioral qualification is
//! implied. Legacy [`from_program`] remains available to its existing guarded
//! scalar and fixture callers. [`from_program_bounded`] enforces explicit
//! caller-owned [`SourceCopyBudget`] extents before every affected bridge
//! worklist, child slot, arm, string, list, or tree allocation.
//!
//! Security boundary: [`from_program_bounded`] bounds only the allocations
//! performed while constructing [`b::CoreProgram`] in this bridge. It does not
//! bound transient replacement-box/worklist allocations inside owner-provided
//! iterative `Drop` for `CoreProgram`/`CoreExpr`/`CoreTree`, recursive `Arc`
//! destruction of `a::Program`, owner `prepare_core`/`admit`/`admit_run`
//! allocations, downstream provenance/report/JSON serialization, or global
//! host heap/stack/time admission (H1).

use crate::sherlock as a;
use super::shared as b;
use b::i128_class::KIntC;

/// Explicit, inclusive, caller-owned limits for [`from_program_bounded`].
///
/// There is deliberately no `Default` and no profile-supplied cap: every caller
/// must state all ten extents explicitly. Neither parser extents
/// (`max_parse_depth`, `max_structural_ast_depth`, `max_ast_occurrences`) nor
/// checker `max_type_depth` may be silently substituted for copy policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceCopyBudget {
    pub max_functions: usize,
    pub max_parameters_per_function: usize,
    pub max_total_parameters: usize,
    /// Total `CoreExpr` occurrences copied across `entry` and all function
    /// bodies, counting each structural occurrence of shared `Arc<a::Expr>`.
    pub max_expr_nodes: usize,
    /// Structural `CoreExpr` copy depth on any root-to-leaf path (`entry` and
    /// each function body root start at depth 1; each child `Expr` adds 1).
    pub max_copy_depth: usize,
    /// Total `CoreArm`s (and their `CorePattern`s) copied across `Match` nodes.
    pub max_match_arms: usize,
    /// Cumulative UTF-8 bytes cloned across all `Name` strings in the copied
    /// `CoreProgram` (function names, parameter names, `Var`, `Let` binders,
    /// `Call` targets, and `Pattern` binders).
    pub max_name_bytes: usize,
    /// Cumulative `u128` (`KIntC`) elements copied across `Value::List` nodes.
    pub max_list_elements: usize,
    /// Cumulative `CoreTree` nodes (`Leaf` + `Branch`, per structural
    /// occurrence) copied across `Value::Tree` literals.
    pub max_tree_nodes: usize,
    /// Structural `CoreTree` depth within any `Value::Tree` literal (root
    /// `Tree` is depth 1; each `Tree::Branch` child adds 1).
    pub max_tree_depth: usize,
}

/// Distinct copy-stage resource identifier, never an evaluator `Outcome` code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceCopyResource {
    Functions,
    ParametersPerFunction,
    TotalParameters,
    ExprNodes,
    CopyDepth,
    MatchArms,
    NameBytes,
    ListElements,
    TreeNodes,
    TreeDepth,
}

/// Exact zero-allocation diagnostic payload for a refused copy allocation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceCopyBudgetExceeded {
    pub resource: SourceCopyResource,
    pub limit: usize,
    /// First attempted count, depth, or byte total above `limit`;
    /// `None` means checked `usize` addition overflowed.
    pub observed: Option<usize>,
    /// `Some(i)` when copying function `i` (header or body); `None` at the
    /// top-level function-table check or inside `entry`.
    pub function_index: Option<usize>,
    /// Offending parameter index, child expression index (`0` for `Unary` or
    /// `Match` scrutinee, `0..2` for `Let`/`Binary`, `0..3` for `If`,
    /// `0..args.len()` for `Call`, `1 + arm_idx` for `Match` arm pattern/body),
    /// or `Tree::Branch` child index (`0` = left, `1` = right); `None` at
    /// root or node-own checks.
    pub child_index: Option<usize>,
}

/// Preparation copy failure, distinct from parser, typechecker, and owner `Tooling` errors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceCopyError {
    Budget(SourceCopyBudgetExceeded),
}

#[derive(Default)]
struct CopyCounters {
    total_parameters: usize,
    expr_nodes: usize,
    match_arms: usize,
    name_bytes: usize,
    list_elements: usize,
    tree_nodes: usize,
}

fn check_limit(
    resource: SourceCopyResource,
    limit: usize,
    observed: Option<usize>,
    function_index: Option<usize>,
    child_index: Option<usize>,
) -> Result<usize, SourceCopyError> {
    if let Some(value) = observed {
        if value <= limit {
            return Ok(value);
        }
    }
    Err(SourceCopyError::Budget(SourceCopyBudgetExceeded {
        resource,
        limit,
        observed,
        function_index,
        child_index,
    }))
}

fn check_add(
    resource: SourceCopyResource,
    current: &mut usize,
    delta: usize,
    limit: usize,
    function_index: Option<usize>,
    child_index: Option<usize>,
) -> Result<(), SourceCopyError> {
    let next = check_limit(
        resource,
        limit,
        current.checked_add(delta),
        function_index,
        child_index,
    )?;
    *current = next;
    Ok(())
}

fn check_pattern_name_bytes(
    input: &a::Pattern,
    budget: &SourceCopyBudget,
    counters: &mut CopyCounters,
    function_index: Option<usize>,
    child_index: Option<usize>,
) -> Result<(), SourceCopyError> {
    match input {
        a::Pattern::Nil => Ok(()),
        a::Pattern::Var(x) | a::Pattern::Leaf(x) => check_add(
            SourceCopyResource::NameBytes,
            &mut counters.name_bytes,
            x.len(),
            budget.max_name_bytes,
            function_index,
            child_index,
        ),
        a::Pattern::Cons(x, y) | a::Pattern::Branch(x, y) => {
            check_add(
                SourceCopyResource::NameBytes,
                &mut counters.name_bytes,
                x.len(),
                budget.max_name_bytes,
                function_index,
                child_index,
            )?;
            check_add(
                SourceCopyResource::NameBytes,
                &mut counters.name_bytes,
                y.len(),
                budget.max_name_bytes,
                function_index,
                child_index,
            )
        }
    }
}

/// Copy every current A AST constructor into its B dedicated-sort carrier
/// under explicit caller-supplied [`SourceCopyBudget`] extents.
///
/// Every extent is checked before the affected vector, box, string, or
/// worklist allocation. Traversal order and output structure are identical to
/// [`from_program`] whenever the budget admits the program.
///
/// # Security Note
/// If this function returns `Err`, any partially constructed `b::CoreProgram`
/// is dropped via the owner's iterative `Drop` implementation in
/// `shared::owner_prepare`, which avoids call-stack recursion but allocates
/// transient worklist/replacement boxes for already-admitted nodes.
pub fn from_program_bounded(
    source: &a::Program,
    budget: &SourceCopyBudget,
) -> Result<b::CoreProgram, SourceCopyError> {
    check_limit(
        SourceCopyResource::Functions,
        budget.max_functions,
        Some(source.functions.len()),
        None,
        None,
    )?;

    let mut counters = CopyCounters::default();
    let mut functions = Vec::with_capacity(source.functions.len());

    for (fn_idx, function) in source.functions.iter().enumerate() {
        let fn_loc = Some(fn_idx);
        check_add(
            SourceCopyResource::NameBytes,
            &mut counters.name_bytes,
            function.name.len(),
            budget.max_name_bytes,
            fn_loc,
            None,
        )?;
        check_limit(
            SourceCopyResource::ParametersPerFunction,
            budget.max_parameters_per_function,
            Some(function.parameters.len()),
            fn_loc,
            None,
        )?;
        check_add(
            SourceCopyResource::TotalParameters,
            &mut counters.total_parameters,
            function.parameters.len(),
            budget.max_total_parameters,
            fn_loc,
            None,
        )?;

        let mut parameters = Vec::with_capacity(function.parameters.len());
        for (param_idx, parameter) in function.parameters.iter().enumerate() {
            check_add(
                SourceCopyResource::NameBytes,
                &mut counters.name_bytes,
                parameter.name.len(),
                budget.max_name_bytes,
                fn_loc,
                Some(param_idx),
            )?;
            parameters.push(b::CoreParameter {
                name: parameter.name.clone(),
                sort: sort(parameter.ty),
            });
        }

        functions.push(b::CoreFunction {
            name: function.name.clone(),
            parameters,
            return_type: sort(function.return_type),
            body: expression_slot(),
        });
    }

    let mut destination = b::CoreProgram {
        functions,
        entry: expression_slot(),
    };

    // Copy entry in preorder first, then each function body in source table
    // order, matching `from_program` while keeping only one active root on the
    // expression worklist at a time.
    copy_root_expression(
        source.entry.as_ref(),
        &mut destination.entry,
        None,
        budget,
        &mut counters,
    )?;

    for (fn_idx, (input_fn, output_fn)) in source
        .functions
        .iter()
        .zip(destination.functions.iter_mut())
        .enumerate()
    {
        copy_root_expression(
            input_fn.body.as_ref(),
            &mut output_fn.body,
            Some(fn_idx),
            budget,
            &mut counters,
        )?;
    }

    Ok(destination)
}

fn copy_root_expression<'source>(
    root_input: &'source a::Expr,
    root_output: &mut b::CoreExpr,
    function_index: Option<usize>,
    budget: &SourceCopyBudget,
    counters: &mut CopyCounters,
) -> Result<(), SourceCopyError> {
    check_limit(
        SourceCopyResource::CopyDepth,
        budget.max_copy_depth,
        Some(1),
        function_index,
        None,
    )?;
    check_add(
        SourceCopyResource::ExprNodes,
        &mut counters.expr_nodes,
        1,
        budget.max_expr_nodes,
        function_index,
        None,
    )?;

    let mut pending = vec![BoundedExpressionCopy {
        input: root_input,
        output: root_output,
        depth: 1,
    }];

    while let Some(BoundedExpressionCopy { input, output, depth }) = pending.pop() {
        match input {
            a::Expr::Literal(input_val) => {
                *output = b::CoreExpr::Literal {
                    value: value_bounded(input_val, budget, counters, function_index)?,
                };
            }
            a::Expr::Var(name) => {
                check_add(
                    SourceCopyResource::NameBytes,
                    &mut counters.name_bytes,
                    name.len(),
                    budget.max_name_bytes,
                    function_index,
                    None,
                )?;
                *output = b::CoreExpr::Var { name: name.clone() };
            }
            a::Expr::Let(name, input_value, input_body) => {
                check_add(
                    SourceCopyResource::NameBytes,
                    &mut counters.name_bytes,
                    name.len(),
                    budget.max_name_bytes,
                    function_index,
                    None,
                )?;
                let next_depth = check_limit(
                    SourceCopyResource::CopyDepth,
                    budget.max_copy_depth,
                    depth.checked_add(1),
                    function_index,
                    Some(0),
                )?;
                check_add(
                    SourceCopyResource::ExprNodes,
                    &mut counters.expr_nodes,
                    1,
                    budget.max_expr_nodes,
                    function_index,
                    Some(0),
                )?;
                check_add(
                    SourceCopyResource::ExprNodes,
                    &mut counters.expr_nodes,
                    1,
                    budget.max_expr_nodes,
                    function_index,
                    Some(1),
                )?;
                *output = b::CoreExpr::Let {
                    name: name.clone(),
                    value: Box::new(expression_slot()),
                    body: Box::new(expression_slot()),
                };
                match output {
                    b::CoreExpr::Let { value, body, .. } => {
                        pending.push(BoundedExpressionCopy {
                            input: input_body.as_ref(),
                            output: body.as_mut(),
                            depth: next_depth,
                        });
                        pending.push(BoundedExpressionCopy {
                            input: input_value.as_ref(),
                            output: value.as_mut(),
                            depth: next_depth,
                        });
                    }
                    _ => unreachable!("fresh Let construction"),
                }
            }
            a::Expr::Binary(op, input_left, input_right) => {
                let next_depth = check_limit(
                    SourceCopyResource::CopyDepth,
                    budget.max_copy_depth,
                    depth.checked_add(1),
                    function_index,
                    Some(0),
                )?;
                check_add(
                    SourceCopyResource::ExprNodes,
                    &mut counters.expr_nodes,
                    1,
                    budget.max_expr_nodes,
                    function_index,
                    Some(0),
                )?;
                check_add(
                    SourceCopyResource::ExprNodes,
                    &mut counters.expr_nodes,
                    1,
                    budget.max_expr_nodes,
                    function_index,
                    Some(1),
                )?;
                *output = b::CoreExpr::Binary {
                    op: binary(*op),
                    left: Box::new(expression_slot()),
                    right: Box::new(expression_slot()),
                };
                match output {
                    b::CoreExpr::Binary { left, right, .. } => {
                        pending.push(BoundedExpressionCopy {
                            input: input_right.as_ref(),
                            output: right.as_mut(),
                            depth: next_depth,
                        });
                        pending.push(BoundedExpressionCopy {
                            input: input_left.as_ref(),
                            output: left.as_mut(),
                            depth: next_depth,
                        });
                    }
                    _ => unreachable!("fresh Binary construction"),
                }
            }
            a::Expr::Unary(op, input_operand) => {
                let next_depth = check_limit(
                    SourceCopyResource::CopyDepth,
                    budget.max_copy_depth,
                    depth.checked_add(1),
                    function_index,
                    Some(0),
                )?;
                check_add(
                    SourceCopyResource::ExprNodes,
                    &mut counters.expr_nodes,
                    1,
                    budget.max_expr_nodes,
                    function_index,
                    Some(0),
                )?;
                *output = b::CoreExpr::Unary {
                    op: unary(*op),
                    operand: Box::new(expression_slot()),
                };
                match output {
                    b::CoreExpr::Unary { operand, .. } => {
                        pending.push(BoundedExpressionCopy {
                            input: input_operand.as_ref(),
                            output: operand.as_mut(),
                            depth: next_depth,
                        });
                    }
                    _ => unreachable!("fresh Unary construction"),
                }
            }
            a::Expr::If(input_cond, input_then, input_else) => {
                let next_depth = check_limit(
                    SourceCopyResource::CopyDepth,
                    budget.max_copy_depth,
                    depth.checked_add(1),
                    function_index,
                    Some(0),
                )?;
                for child_idx in 0..3 {
                    check_add(
                        SourceCopyResource::ExprNodes,
                        &mut counters.expr_nodes,
                        1,
                        budget.max_expr_nodes,
                        function_index,
                        Some(child_idx),
                    )?;
                }
                *output = b::CoreExpr::If {
                    cond: Box::new(expression_slot()),
                    then_expr: Box::new(expression_slot()),
                    else_expr: Box::new(expression_slot()),
                };
                match output {
                    b::CoreExpr::If { cond, then_expr, else_expr } => {
                        pending.push(BoundedExpressionCopy {
                            input: input_else.as_ref(),
                            output: else_expr.as_mut(),
                            depth: next_depth,
                        });
                        pending.push(BoundedExpressionCopy {
                            input: input_then.as_ref(),
                            output: then_expr.as_mut(),
                            depth: next_depth,
                        });
                        pending.push(BoundedExpressionCopy {
                            input: input_cond.as_ref(),
                            output: cond.as_mut(),
                            depth: next_depth,
                        });
                    }
                    _ => unreachable!("fresh If construction"),
                }
            }
            a::Expr::Call(name, input_args) => {
                check_add(
                    SourceCopyResource::NameBytes,
                    &mut counters.name_bytes,
                    name.len(),
                    budget.max_name_bytes,
                    function_index,
                    None,
                )?;
                let next_depth = if input_args.is_empty() {
                    depth
                } else {
                    let nd = check_limit(
                        SourceCopyResource::CopyDepth,
                        budget.max_copy_depth,
                        depth.checked_add(1),
                        function_index,
                        Some(0),
                    )?;
                    for arg_idx in 0..input_args.len() {
                        check_add(
                            SourceCopyResource::ExprNodes,
                            &mut counters.expr_nodes,
                            1,
                            budget.max_expr_nodes,
                            function_index,
                            Some(arg_idx),
                        )?;
                    }
                    nd
                };
                let mut args = Vec::with_capacity(input_args.len());
                for _ in 0..input_args.len() {
                    args.push(expression_slot());
                }
                *output = b::CoreExpr::Call {
                    name: name.clone(),
                    args,
                };
                match output {
                    b::CoreExpr::Call { args, .. } => {
                        for (arg_in, arg_out) in input_args.iter().zip(args.iter_mut()).rev() {
                            pending.push(BoundedExpressionCopy {
                                input: arg_in.as_ref(),
                                output: arg_out,
                                depth: next_depth,
                            });
                        }
                    }
                    _ => unreachable!("fresh Call construction"),
                }
            }
            a::Expr::Match(input_scrutinee, input_arms) => {
                let next_depth = check_limit(
                    SourceCopyResource::CopyDepth,
                    budget.max_copy_depth,
                    depth.checked_add(1),
                    function_index,
                    Some(0),
                )?;
                check_add(
                    SourceCopyResource::MatchArms,
                    &mut counters.match_arms,
                    input_arms.len(),
                    budget.max_match_arms,
                    function_index,
                    None,
                )?;
                check_add(
                    SourceCopyResource::ExprNodes,
                    &mut counters.expr_nodes,
                    1,
                    budget.max_expr_nodes,
                    function_index,
                    Some(0),
                )?;
                for (arm_idx, arm) in input_arms.iter().enumerate() {
                    let child_idx = arm_idx.checked_add(1);
                    check_pattern_name_bytes(
                        &arm.pattern,
                        budget,
                        counters,
                        function_index,
                        child_idx,
                    )?;
                    check_add(
                        SourceCopyResource::ExprNodes,
                        &mut counters.expr_nodes,
                        1,
                        budget.max_expr_nodes,
                        function_index,
                        child_idx,
                    )?;
                }
                let mut arms = Vec::with_capacity(input_arms.len());
                for arm in input_arms {
                    arms.push(b::CoreArm {
                        pattern: pattern(&arm.pattern),
                        body: expression_slot(),
                    });
                }
                *output = b::CoreExpr::Match {
                    scrutinee: Box::new(expression_slot()),
                    arms,
                };
                match output {
                    b::CoreExpr::Match { scrutinee, arms } => {
                        for (arm_in, arm_out) in input_arms.iter().zip(arms.iter_mut()).rev() {
                            pending.push(BoundedExpressionCopy {
                                input: arm_in.body.as_ref(),
                                output: &mut arm_out.body,
                                depth: next_depth,
                            });
                        }
                        pending.push(BoundedExpressionCopy {
                            input: input_scrutinee.as_ref(),
                            output: scrutinee.as_mut(),
                            depth: next_depth,
                        });
                    }
                    _ => unreachable!("fresh Match construction"),
                }
            }
        }
    }
    drop(pending);
    Ok(())
}

fn value_bounded(
    input: &a::Value,
    budget: &SourceCopyBudget,
    counters: &mut CopyCounters,
    function_index: Option<usize>,
) -> Result<b::CoreValue, SourceCopyError> {
    match input {
        a::Value::UInt(n) => Ok(b::CoreValue::UInt { n: KIntC::InRange(*n) }),
        a::Value::Bool(input) => Ok(b::CoreValue::Bool { b: *input }),
        a::Value::List(elements) => {
            check_add(
                SourceCopyResource::ListElements,
                &mut counters.list_elements,
                elements.len(),
                budget.max_list_elements,
                function_index,
                None,
            )?;
            Ok(b::CoreValue::List {
                elements: elements.iter().copied().map(KIntC::InRange).collect(),
            })
        }
        a::Value::Tree(input) => Ok(b::CoreValue::Tree {
            tree: tree_bounded(input, budget, counters, function_index)?,
        }),
    }
}

fn tree_bounded(
    input: &a::Tree,
    budget: &SourceCopyBudget,
    counters: &mut CopyCounters,
    function_index: Option<usize>,
) -> Result<b::CoreTree, SourceCopyError> {
    check_limit(
        SourceCopyResource::TreeDepth,
        budget.max_tree_depth,
        Some(1),
        function_index,
        None,
    )?;
    check_add(
        SourceCopyResource::TreeNodes,
        &mut counters.tree_nodes,
        1,
        budget.max_tree_nodes,
        function_index,
        None,
    )?;

    let mut destination = tree_slot();
    let mut pending = vec![BoundedTreeCopy {
        input,
        output: &mut destination,
        depth: 1,
    }];
    while let Some(BoundedTreeCopy { input, output, depth }) = pending.pop() {
        match input {
            a::Tree::Leaf(n) => {
                *output = b::CoreTree::Leaf { n: KIntC::InRange(*n) };
            }
            a::Tree::Branch(input_left, input_right) => {
                let next_depth = check_limit(
                    SourceCopyResource::TreeDepth,
                    budget.max_tree_depth,
                    depth.checked_add(1),
                    function_index,
                    Some(0),
                )?;
                check_add(
                    SourceCopyResource::TreeNodes,
                    &mut counters.tree_nodes,
                    1,
                    budget.max_tree_nodes,
                    function_index,
                    Some(0),
                )?;
                check_add(
                    SourceCopyResource::TreeNodes,
                    &mut counters.tree_nodes,
                    1,
                    budget.max_tree_nodes,
                    function_index,
                    Some(1),
                )?;
                *output = b::CoreTree::Branch {
                    left: Box::new(tree_slot()),
                    right: Box::new(tree_slot()),
                };
                match output {
                    b::CoreTree::Branch { left, right } => {
                        pending.push(BoundedTreeCopy {
                            input: input_right.as_ref(),
                            output: right.as_mut(),
                            depth: next_depth,
                        });
                        pending.push(BoundedTreeCopy {
                            input: input_left.as_ref(),
                            output: left.as_mut(),
                            depth: next_depth,
                        });
                    }
                    _ => unreachable!("fresh tree Branch construction"),
                }
            }
        }
    }
    drop(pending);
    Ok(destination)
}

struct BoundedExpressionCopy<'source, 'destination> {
    input: &'source a::Expr,
    output: &'destination mut b::CoreExpr,
    depth: usize,
}

struct BoundedTreeCopy<'source, 'destination> {
    input: &'source a::Tree,
    output: &'destination mut b::CoreTree,
    depth: usize,
}

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

#[cfg(test)]
#[path = "source_input_bridge_bounded_tests.rs"]
mod tests;
