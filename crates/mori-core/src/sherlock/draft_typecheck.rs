//! Monomorphic type checking for the draft Sherlock source AST. Not an adopted
//! source profile; it gives no H1 admission or host resource guarantee.
use crate::sherlock::{BinaryOp, Expr, Function, Name, Pattern, Program, Type, UnaryOp, Value};
use crate::sherlock_draft::SourceLocation;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DraftTypeSummary {
    pub function_returns: Vec<Type>,
    pub entry_type: Type,
}

/// Local preparation diagnostics, not runtime Outcomes or existing error codes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DraftTypeError {
    pub source: SourceLocation,
    pub kind: DraftTypeErrorKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DraftTypeErrorKind {
    UnknownVariable(Name),
    UnknownFunction(Name),
    Arity {
        name: Name,
        expected: usize,
        actual: usize,
    },
    Expected {
        expected: Type,
        actual: Type,
    },
    BranchTypes {
        yes: Type,
        no: Type,
    },
    ArmTypes {
        expected: Type,
        actual: Type,
    },
    DuplicateParameter {
        name: Name,
        index: usize,
    },
    DuplicatePatternBinder(Name),
    EmptyMatch,
    Budget(DraftTypeBudgetExceeded),
}

/// Distinct preparation resource, never an evaluator Outcome/error code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftTypeBudgetResource {
    TypeDepth,
}

/// `source` on the containing DraftTypeError names the admitted parent.
/// `child_index` names its refused child, or None for a refused body/entry root.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DraftTypeBudgetExceeded {
    pub resource: DraftTypeBudgetResource,
    pub limit: usize,
    /// First attempted depth; None means checked depth arithmetic overflowed.
    pub observed: Option<usize>,
    pub child_index: Option<usize>,
}

#[derive(Clone, Copy)]
struct DepthBudget {
    limit: Option<usize>,
}

impl DepthBudget {
    fn next_depth(
        self,
        parent_depth: usize,
        source: &SourceLocation,
        child_index: Option<usize>,
    ) -> Result<usize, DraftTypeError> {
        let Some(limit) = self.limit else {
            // Legacy traversal has no hidden cap and needs no depth arithmetic.
            return Ok(0);
        };
        let observed = parent_depth.checked_add(1);
        if let Some(next) = observed {
            if next <= limit {
                return Ok(next);
            }
        }
        // Copy only the already-admitted parent path, never the refused child.
        Err(error(
            source,
            DraftTypeErrorKind::Budget(DraftTypeBudgetExceeded {
                resource: DraftTypeBudgetResource::TypeDepth,
                limit,
                observed,
                child_index,
            }),
        ))
    }
}

type Types = BTreeMap<Name, Type>;

/// Inspect every body under fresh parameter types, then entry under an empty
/// environment. Named calls consult declarations, never recurse into callees.
/// Thus mutual recursion requires no source unrolling or inferred recursion cap.
/// This checker is a review-pending ordinary typed subset, not admission policy.
///
/// # Security Warning (H1 — Unmetered Host Evaluator)
/// This machine is a host diagnostic reference with zero native authority and no host time,
/// heap, or native stack admission bounds. Do not expose it to untrusted inputs.
pub fn check_program_types(program: &Program) -> Result<DraftTypeSummary, DraftTypeError> {
    check_program_types_inner(program, DepthBudget { limit: None })
}

/// Bound active structural Expr inference depth with an explicit inclusive limit.
/// Roots have depth1; each Expr child adds1; every body and entry resets the root.
/// Calls inspect signatures/arguments without descending into named callees.
/// Checks precede child-path copies, descent and Let/Match environment copies.
/// A budget error records the admitted parent source plus attempted child index;
/// refused roots record their root source with no child index. This permits a
/// precise location without first constructing the over-budget child path.
/// Zero refuses the first root before parameter environment construction.
/// No default, arbitrary cap, global work/heap or native-stack guarantee is given;
/// H1 and parser/copy/report/Drop/public-consumer obligations remain separate.
pub fn check_program_types_bounded(
    program: &Program,
    max_type_depth: usize,
) -> Result<DraftTypeSummary, DraftTypeError> {
    check_program_types_inner(
        program,
        DepthBudget {
            limit: Some(max_type_depth),
        },
    )
}

fn check_program_types_inner(
    program: &Program,
    budget: DepthBudget,
) -> Result<DraftTypeSummary, DraftTypeError> {
    let mut function_returns = Vec::new();
    for (index, function) in program.functions.iter().enumerate() {
        let source = SourceLocation {
            function_index: Some(index),
            child_path: vec![],
        };
        let depth = budget.next_depth(0, &source, None)?;
        let mut environment = Types::new();
        for (parameter_index, parameter) in function.parameters.iter().enumerate() {
            if environment
                .insert(parameter.name.clone(), parameter.ty)
                .is_some()
            {
                return Err(error(
                    &source,
                    DraftTypeErrorKind::DuplicateParameter {
                        name: parameter.name.clone(),
                        index: parameter_index,
                    },
                ));
            }
        }
        let actual = infer(
            program,
            &function.body,
            &environment,
            &source,
            depth,
            budget,
        )?;
        require(actual, function.return_type, &source)?;
        function_returns.push(actual);
    }
    let source = SourceLocation {
        function_index: None,
        child_path: vec![],
    };
    let depth = budget.next_depth(0, &source, None)?;
    let entry_type = infer(
        program,
        &program.entry,
        &Types::new(),
        &source,
        depth,
        budget,
    )?;
    Ok(DraftTypeSummary {
        function_returns,
        entry_type,
    })
}

fn error(source: &SourceLocation, kind: DraftTypeErrorKind) -> DraftTypeError {
    DraftTypeError {
        source: source.clone(),
        kind,
    }
}

fn child(
    source: &SourceLocation,
    index: usize,
    depth: usize,
    budget: DepthBudget,
) -> Result<(SourceLocation, usize), DraftTypeError> {
    let child_depth = budget.next_depth(depth, source, Some(index))?;
    let mut source = source.clone();
    source.child_path.push(index);
    Ok((source, child_depth))
}

fn require(actual: Type, expected: Type, source: &SourceLocation) -> Result<(), DraftTypeError> {
    if actual == expected {
        Ok(())
    } else {
        Err(error(
            source,
            DraftTypeErrorKind::Expected { expected, actual },
        ))
    }
}

fn infer(
    program: &Program,
    expression: &Expr,
    environment: &Types,
    source: &SourceLocation,
    depth: usize,
    budget: DepthBudget,
) -> Result<Type, DraftTypeError> {
    match expression {
        Expr::Literal(value) => Ok(match value {
            Value::UInt(_) => Type::UInt,
            Value::Bool(_) => Type::Bool,
            Value::List(_) => Type::List,
            Value::Tree(_) => Type::Tree,
        }),
        Expr::Var(name) => environment
            .get(name)
            .copied()
            .ok_or_else(|| error(source, DraftTypeErrorKind::UnknownVariable(name.clone()))),
        Expr::Let(name, value, body) => {
            let (value_source, value_depth) = child(source, 0, depth, budget)?;
            let value_type = infer(
                program,
                value,
                environment,
                &value_source,
                value_depth,
                budget,
            )?;
            let (body_source, body_depth) = child(source, 1, depth, budget)?;
            let mut inner = environment.clone();
            inner.insert(name.clone(), value_type);
            infer(program, body, &inner, &body_source, body_depth, budget)
        }
        Expr::Binary(op, left, right) => {
            let (left_source, left_depth) = child(source, 0, depth, budget)?;
            let left = infer(program, left, environment, &left_source, left_depth, budget)?;
            let (right_source, right_depth) = child(source, 1, depth, budget)?;
            let right = infer(
                program,
                right,
                environment,
                &right_source,
                right_depth,
                budget,
            )?;
            // Frozen A equality is structural across all existing value sorts,
            // including unequal sorts. Do not silently copy Core's Eq rule.
            if *op == BinaryOp::Eq {
                return Ok(Type::Bool);
            }
            let (left_expected, right_expected, result) = match op {
                BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul => {
                    (Type::UInt, Type::UInt, Type::UInt)
                }
                BinaryOp::Lt => (Type::UInt, Type::UInt, Type::Bool),
                BinaryOp::Cons => (Type::UInt, Type::List, Type::List),
                BinaryOp::Branch => (Type::Tree, Type::Tree, Type::Tree),
                BinaryOp::Eq => unreachable!("handled without imposing equal operand sorts"),
            };
            require(left, left_expected, &left_source)?;
            require(right, right_expected, &right_source)?;
            Ok(result)
        }
        Expr::Unary(op, operand) => {
            let (operand_source, operand_depth) = child(source, 0, depth, budget)?;
            let actual = infer(
                program,
                operand,
                environment,
                &operand_source,
                operand_depth,
                budget,
            )?;
            let (expected, result) = match op {
                UnaryOp::Leaf => (Type::UInt, Type::Tree),
                UnaryOp::Not => (Type::Bool, Type::Bool),
            };
            require(actual, expected, &operand_source)?;
            Ok(result)
        }
        Expr::If(condition, yes, no) => {
            let (condition_source, condition_depth) = child(source, 0, depth, budget)?;
            let condition = infer(
                program,
                condition,
                environment,
                &condition_source,
                condition_depth,
                budget,
            )?;
            require(condition, Type::Bool, &condition_source)?;
            let (yes_source, yes_depth) = child(source, 1, depth, budget)?;
            let yes = infer(program, yes, environment, &yes_source, yes_depth, budget)?;
            let (no_source, no_depth) = child(source, 2, depth, budget)?;
            let no = infer(program, no, environment, &no_source, no_depth, budget)?;
            if yes == no {
                Ok(yes)
            } else {
                Err(error(source, DraftTypeErrorKind::BranchTypes { yes, no }))
            }
        }
        Expr::Call(name, arguments) => {
            // Same first matching declaration rule as actual A lookup. Checking
            // all bodies above does not reorder/deduplicate this original table.
            let function: &Function = program
                .functions
                .iter()
                .find(|function| &function.name == name)
                .ok_or_else(|| error(source, DraftTypeErrorKind::UnknownFunction(name.clone())))?;
            if function.parameters.len() != arguments.len() {
                return Err(error(
                    source,
                    DraftTypeErrorKind::Arity {
                        name: name.clone(),
                        expected: function.parameters.len(),
                        actual: arguments.len(),
                    },
                ));
            }
            for (index, (argument, parameter)) in
                arguments.iter().zip(&function.parameters).enumerate()
            {
                let (source, argument_depth) = child(source, index, depth, budget)?;
                let actual = infer(
                    program,
                    argument,
                    environment,
                    &source,
                    argument_depth,
                    budget,
                )?;
                require(actual, parameter.ty, &source)?;
            }
            Ok(function.return_type)
        }
        Expr::Match(scrutinee, arms) => {
            let (scrutinee_source, scrutinee_depth) = child(source, 0, depth, budget)?;
            let scrutinee_type = infer(
                program,
                scrutinee,
                environment,
                &scrutinee_source,
                scrutinee_depth,
                budget,
            )?;
            let mut result = None;
            for (index, arm) in arms.iter().enumerate() {
                let (body_source, body_depth) = child(source, index + 1, depth, budget)?;
                let inner = pattern_types(&arm.pattern, scrutinee_type, environment, &body_source)?;
                let actual = infer(program, &arm.body, &inner, &body_source, body_depth, budget)?;
                if let Some(expected) = result {
                    if actual != expected {
                        return Err(error(
                            &body_source,
                            DraftTypeErrorKind::ArmTypes { expected, actual },
                        ));
                    }
                } else {
                    result = Some(actual);
                }
            }
            result.ok_or_else(|| error(source, DraftTypeErrorKind::EmptyMatch))
        }
    }
}

fn pattern_types(
    pattern: &Pattern,
    scrutinee: Type,
    saved: &Types,
    source: &SourceLocation,
) -> Result<Types, DraftTypeError> {
    let mut environment = saved.clone();
    match pattern {
        Pattern::Var(name) => {
            environment.insert(name.clone(), scrutinee);
        }
        Pattern::Nil => {
            require(scrutinee, Type::List, source)?;
        }
        Pattern::Cons(head, tail) => {
            require(scrutinee, Type::List, source)?;
            distinct(head, tail, source)?;
            environment.insert(head.clone(), Type::UInt);
            environment.insert(tail.clone(), Type::List);
        }
        Pattern::Leaf(name) => {
            require(scrutinee, Type::Tree, source)?;
            environment.insert(name.clone(), Type::UInt);
        }
        Pattern::Branch(left, right) => {
            require(scrutinee, Type::Tree, source)?;
            distinct(left, right, source)?;
            environment.insert(left.clone(), Type::Tree);
            environment.insert(right.clone(), Type::Tree);
        }
    }
    Ok(environment)
}

fn distinct(left: &str, right: &str, source: &SourceLocation) -> Result<(), DraftTypeError> {
    if left == right {
        Err(error(
            source,
            DraftTypeErrorKind::DuplicatePatternBinder(left.into()),
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
#[path = "draft_typecheck_tests.rs"]
mod tests;
