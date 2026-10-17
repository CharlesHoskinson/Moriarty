//! SOURCE_ONLY / UNRUN: ordinary monomorphic typing proposal for the actual AST.
//! No execution, source profile adoption, H1 admission or host resource guarantee.
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
    Arity { name: Name, expected: usize, actual: usize },
    Expected { expected: Type, actual: Type },
    BranchTypes { yes: Type, no: Type },
    ArmTypes { expected: Type, actual: Type },
    DuplicateParameter { name: Name, index: usize },
    DuplicatePatternBinder(Name),
    EmptyMatch,
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
    let mut function_returns = Vec::new();
    for (index, function) in program.functions.iter().enumerate() {
        let source = SourceLocation { function_index: Some(index), child_path: vec![] };
        let mut environment = Types::new();
        for (parameter_index, parameter) in function.parameters.iter().enumerate() {
            if environment.insert(parameter.name.clone(), parameter.ty).is_some() {
                return Err(error(&source, DraftTypeErrorKind::DuplicateParameter {
                    name: parameter.name.clone(), index: parameter_index,
                }));
            }
        }
        let actual = infer(program, &function.body, &environment, &source)?;
        require(actual, function.return_type, &source)?;
        function_returns.push(actual);
    }
    let source = SourceLocation { function_index: None, child_path: vec![] };
    let entry_type = infer(program, &program.entry, &Types::new(), &source)?;
    Ok(DraftTypeSummary { function_returns, entry_type })
}

fn error(source: &SourceLocation, kind: DraftTypeErrorKind) -> DraftTypeError {
    DraftTypeError { source: source.clone(), kind }
}

fn child(source: &SourceLocation, index: usize) -> SourceLocation {
    let mut source = source.clone();
    source.child_path.push(index);
    source
}

fn require(actual: Type, expected: Type, source: &SourceLocation) -> Result<(), DraftTypeError> {
    if actual == expected { Ok(()) }
    else { Err(error(source, DraftTypeErrorKind::Expected { expected, actual })) }
}

fn infer(program: &Program, expression: &Expr, environment: &Types,
         source: &SourceLocation) -> Result<Type, DraftTypeError> {
    match expression {
        Expr::Literal(value) => Ok(match value {
            Value::UInt(_) => Type::UInt, Value::Bool(_) => Type::Bool,
            Value::List(_) => Type::List, Value::Tree(_) => Type::Tree,
        }),
        Expr::Var(name) => environment.get(name).copied()
            .ok_or_else(|| error(source, DraftTypeErrorKind::UnknownVariable(name.clone()))),
        Expr::Let(name, value, body) => {
            let value_type = infer(program, value, environment, &child(source, 0))?;
            let mut inner = environment.clone();
            inner.insert(name.clone(), value_type);
            infer(program, body, &inner, &child(source, 1))
        }
        Expr::Binary(op, left, right) => {
            let left_source = child(source, 0);
            let right_source = child(source, 1);
            let left = infer(program, left, environment, &left_source)?;
            let right = infer(program, right, environment, &right_source)?;
            // Frozen A equality is structural across all existing value sorts,
            // including unequal sorts. Do not silently copy Core's Eq rule.
            if *op == BinaryOp::Eq { return Ok(Type::Bool); }
            let (left_expected, right_expected, result) = match op {
                BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul => (Type::UInt, Type::UInt, Type::UInt),
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
            let operand_source = child(source, 0);
            let actual = infer(program, operand, environment, &operand_source)?;
            let (expected, result) = match op {
                UnaryOp::Leaf => (Type::UInt, Type::Tree),
                UnaryOp::Not => (Type::Bool, Type::Bool),
            };
            require(actual, expected, &operand_source)?;
            Ok(result)
        }
        Expr::If(condition, yes, no) => {
            let condition_source = child(source, 0);
            let condition = infer(program, condition, environment, &condition_source)?;
            require(condition, Type::Bool, &condition_source)?;
            let yes = infer(program, yes, environment, &child(source, 1))?;
            let no = infer(program, no, environment, &child(source, 2))?;
            if yes == no { Ok(yes) }
            else { Err(error(source, DraftTypeErrorKind::BranchTypes { yes, no })) }
        }
        Expr::Call(name, arguments) => {
            // Same first matching declaration rule as actual A lookup. Checking
            // all bodies above does not reorder/deduplicate this original table.
            let function: &Function = program.functions.iter().find(|function| &function.name == name)
                .ok_or_else(|| error(source, DraftTypeErrorKind::UnknownFunction(name.clone())))?;
            if function.parameters.len() != arguments.len() {
                return Err(error(source, DraftTypeErrorKind::Arity { name: name.clone(),
                    expected: function.parameters.len(), actual: arguments.len() }));
            }
            for (index, (argument, parameter)) in arguments.iter().zip(&function.parameters).enumerate() {
                let source = child(source, index);
                let actual = infer(program, argument, environment, &source)?;
                require(actual, parameter.ty, &source)?;
            }
            Ok(function.return_type)
        }
        Expr::Match(scrutinee, arms) => {
            let scrutinee_type = infer(program, scrutinee, environment, &child(source, 0))?;
            let mut result = None;
            for (index, arm) in arms.iter().enumerate() {
                let body_source = child(source, index + 1);
                let inner = pattern_types(&arm.pattern, scrutinee_type, environment, &body_source)?;
                let actual = infer(program, &arm.body, &inner, &body_source)?;
                if let Some(expected) = result {
                    if actual != expected {
                        return Err(error(&body_source, DraftTypeErrorKind::ArmTypes { expected, actual }));
                    }
                } else { result = Some(actual); }
            }
            result.ok_or_else(|| error(source, DraftTypeErrorKind::EmptyMatch))
        }
    }
}

fn pattern_types(pattern: &Pattern, scrutinee: Type, saved: &Types,
                 source: &SourceLocation) -> Result<Types, DraftTypeError> {
    let mut environment = saved.clone();
    match pattern {
        Pattern::Var(name) => { environment.insert(name.clone(), scrutinee); }
        Pattern::Nil => { require(scrutinee, Type::List, source)?; }
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
    if left == right { Err(error(source, DraftTypeErrorKind::DuplicatePatternBinder(left.into()))) }
    else { Ok(()) }
}

#[cfg(test)]
#[path = "draft_typecheck_tests.rs"]
mod tests;
