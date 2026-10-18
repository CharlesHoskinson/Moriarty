//! Source-static projection from current Rust Core to Sherlock's typed scalar AST.
//!
//! Preparation evaluates no expression and calls no reference machine. A missing
//! projection is report metadata, not a refusal by the existing Core compiler.
//! Only stateless `/1` actions with ordered lets and one final emit are mapped.
//! The emit value is projected; its effect remains with the Core consumer.
use crate::sherlock::{
    BinaryOp, Expr, ExprRef, Function, PROFILE, Parameter, Type, UnaryOp, Value,
};
use crate::{BinaryOp as CoreOp, Expr as CoreExpr, ExprId, Program, Statement};
use serde::Serialize;
use std::sync::Arc;

pub const FORMAT: &str = "moriarty-sherlock-scalar-source-projection/1";

/// Maintainer preparation budget: the maximum number of source occurrences a
/// single report (summed across its actions) will materialize. Preparation
/// expands the validated Core expression DAG once per traversed edge with no
/// sharing, so a DAG of linear size can demand exponentially many occurrences
/// (and matching generated AST nodes) even within the existing depth-64 and
/// 8192-expression Core bounds. This ceiling is a representation-preparation
/// limit only: it is not a Core schema rule, resource/circuit cost, or any
/// change to the compiler's supported subset. Exhaustion is per-site
/// `Unsupported` metadata, never a compiler refusal.
pub const MAX_PROJECTION_OCCURRENCES: usize = crate::MAX_EXPRESSIONS + crate::MAX_STATEMENTS;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ProjectionDiagnostic {
    pub code: String,
    pub construct: String,
    pub action_index: usize,
    pub statement_index: Option<usize>,
    pub expression: Option<ExprId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct InputSite {
    pub index: usize,
    pub name: String,
    pub ty: crate::Type,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct LetSite {
    pub statement_index: usize,
    pub name: String,
    pub expression: ExprId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct EmitSite {
    pub statement_index: usize,
    pub name: String,
    pub expression: ExprId,
    pub role: String,
}

/// Path is relative to the generated function body, not text or image row IDs.
/// Every traversed source edge receives a separate occurrence, even for a DAG.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SourceOccurrence {
    pub action_index: usize,
    pub statement_index: usize,
    pub expression: Option<ExprId>,
    pub body_path: Vec<usize>,
    pub node_kind: String,
    pub source_expression: Option<CoreExpr>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ScalarActionReport {
    pub action_index: usize,
    pub action: String,
    pub inputs: Vec<InputSite>,
    pub lets: Vec<LetSite>,
    pub emit: EmitSite,
    /// Body preorder, with child positions matching ProgramImage's AST locators.
    pub occurrences: Vec<SourceOccurrence>,
    pub return_type: crate::Type,
    pub generated_function_name: String,
    pub symbol_domain: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScalarActionMap {
    pub function: Function,
    pub report: ScalarActionReport,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "status", rename_all = "kebab-case")]
pub enum ActionProjection {
    Prepared {
        report: ScalarActionReport,
    },
    Unsupported {
        action_index: usize,
        action: String,
        diagnostic: ProjectionDiagnostic,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SourceProjectionReport {
    pub format: String,
    pub source_language: String,
    pub source_semantics: String,
    pub agreement: String,
    pub reference_profile: String,
    pub qualification: String,
    pub source_function_support: bool,
    pub source_branch_support: bool,
    pub source_recursion_support: bool,
    pub source_data_support: bool,
    pub evaluation_performed: bool,
    pub source_expression_count: usize,
    pub source_action_count: usize,
    pub source_state_count: usize,
    pub source_asset_count: usize,
    pub coverage: String,
    pub actions: Vec<ActionProjection>,
}

/// Report every action without changing the real compiler's supported subset.
pub fn source_projection_report(program: &Program) -> SourceProjectionReport {
    let validation = crate::validate(program);
    let mut remaining = MAX_PROJECTION_OCCURRENCES;
    let actions = program
        .actions
        .iter()
        .enumerate()
        .map(|(index, action)| {
            let result = match &validation {
                Ok(()) => prepare_validated_action(program, index, &mut remaining),
                Err(error) => Err(diagnostic(
                    index,
                    None,
                    None,
                    "core-validation",
                    &error.code,
                )),
            };
            match result {
                Ok(map) => ActionProjection::Prepared { report: map.report },
                Err(diagnostic) => ActionProjection::Unsupported {
                    action_index: index,
                    action: action.name.clone(),
                    diagnostic,
                },
            }
        })
        .collect();
    SourceProjectionReport {
        format: FORMAT.into(), source_language: program.language.clone(),
        source_semantics: program.semantics.clone(), agreement: program.name.clone(),
        reference_profile: PROFILE.into(), qualification: "source-static-scalar-projection-only".into(),
        source_function_support: false, source_branch_support: false,
        source_recursion_support: false, source_data_support: false,
        evaluation_performed: false, source_expression_count: program.expressions.len(),
        source_action_count: program.actions.len(), source_state_count: program.states.len(),
        source_asset_count: program.assets.len(),
        coverage: "direct scalar nodes; stateless actions; ordered lets and one final emit value; first unsupported site per action; no effects execution or unvisited-site mapping".into(),
        actions,
    }
}

/// Compile a typed scalar body with dynamic ordered parameters. The generated
/// function is a representation wrapper; it is not a source function feature.
pub fn prepare_scalar_action(
    program: &Program,
    action_index: usize,
) -> Result<ScalarActionMap, ProjectionDiagnostic> {
    crate::validate(program)
        .map_err(|error| diagnostic(action_index, None, None, "core-validation", &error.code))?;
    let mut remaining = MAX_PROJECTION_OCCURRENCES;
    prepare_validated_action(program, action_index, &mut remaining)
}

fn diagnostic(
    action_index: usize,
    statement_index: Option<usize>,
    expression: Option<ExprId>,
    construct: &str,
    code: &str,
) -> ProjectionDiagnostic {
    ProjectionDiagnostic {
        action_index,
        statement_index,
        expression,
        construct: construct.into(),
        code: code.into(),
    }
}

/// Account one occurrence against the report budget BEFORE any occurrence/path/
/// AST allocation, preserving the failing site's identity.
fn charge(
    remaining: &mut usize,
    action: usize,
    statement: Option<usize>,
    expression: Option<ExprId>,
) -> Result<(), ProjectionDiagnostic> {
    *remaining = remaining.checked_sub(1).ok_or_else(|| {
        diagnostic(
            action,
            statement,
            expression,
            "projection-occurrence-budget",
            "projection-occurrence-budget-exhausted",
        )
    })?;
    Ok(())
}

fn scalar_type(ty: crate::Type) -> Type {
    match ty {
        crate::Type::UInt => Type::UInt,
        crate::Type::Bool => Type::Bool,
    }
}

fn prepare_validated_action(
    program: &Program,
    index: usize,
    remaining: &mut usize,
) -> Result<ScalarActionMap, ProjectionDiagnostic> {
    let action = program
        .actions
        .get(index)
        .ok_or_else(|| diagnostic(index, None, None, "action-index", "no-such-action"))?;
    if program.language != crate::LANGUAGE || program.semantics != crate::SEMANTICS {
        return Err(diagnostic(
            index,
            None,
            None,
            "source-profile",
            "unsupported-projection",
        ));
    }
    if !program.states.is_empty() || !program.assets.is_empty() {
        return Err(diagnostic(
            index,
            None,
            None,
            "state-or-asset-declaration",
            "unsupported-projection",
        ));
    }
    let mut lets = Vec::new();
    let mut emit = None;
    for (statement_index, statement) in action.statements.iter().enumerate() {
        match statement {
            Statement::Let { name, value } if emit.is_none() => lets.push(LetSite {
                statement_index,
                name: name.clone(),
                expression: *value,
            }),
            Statement::Emit { name, value } if statement_index + 1 == action.statements.len() => {
                emit = Some(EmitSite {
                    statement_index,
                    name: name.clone(),
                    expression: *value,
                    role: "value-projection; ordered emit effect remains in Core".into(),
                });
            }
            other => {
                return Err(diagnostic(
                    index,
                    Some(statement_index),
                    None,
                    statement_kind(other),
                    "unsupported-projection",
                ));
            }
        }
    }
    let emit = emit.ok_or_else(|| {
        diagnostic(
            index,
            None,
            None,
            "final-emit",
            "missing-projection-observation",
        )
    })?;
    let mut types: std::collections::BTreeMap<String, crate::Type> = action
        .inputs
        .iter()
        .map(|input| (input.name.clone(), input.ty))
        .collect();
    let mut occurrences = Vec::new();
    let mut mapped_lets = Vec::new();
    let mut path = Vec::new();
    for site in &lets {
        charge(remaining, index, Some(site.statement_index), None)?;
        occurrences.push(SourceOccurrence {
            action_index: index,
            statement_index: site.statement_index,
            expression: None,
            body_path: path.clone(),
            node_kind: "let".into(),
            source_expression: None,
        });
        let mut value_path = path.clone();
        value_path.push(0);
        let value = map_expression(
            program,
            index,
            site.statement_index,
            site.expression,
            value_path,
            &mut occurrences,
            remaining,
        )?;
        let ty =
            crate::runtime::expr_type(program, site.expression, &types, 0).map_err(|error| {
                diagnostic(
                    index,
                    Some(site.statement_index),
                    Some(site.expression),
                    "expression-type",
                    &error.code,
                )
            })?;
        types.insert(site.name.clone(), ty);
        mapped_lets.push((site.name.clone(), value));
        path.push(1);
    }
    let mut body = map_expression(
        program,
        index,
        emit.statement_index,
        emit.expression,
        path,
        &mut occurrences,
        remaining,
    )?;
    let return_type =
        crate::runtime::expr_type(program, emit.expression, &types, 0).map_err(|error| {
            diagnostic(
                index,
                Some(emit.statement_index),
                Some(emit.expression),
                "expression-type",
                &error.code,
            )
        })?;
    for (name, value) in mapped_lets.into_iter().rev() {
        body = Arc::new(Expr::Let(name, value, body));
    }
    let inputs: Vec<_> = action
        .inputs
        .iter()
        .enumerate()
        .map(|(index, input)| InputSite {
            index,
            name: input.name.clone(),
            ty: input.ty,
        })
        .collect();
    let function = Function {
        name: action.name.clone(),
        parameters: inputs
            .iter()
            .map(|input| Parameter {
                name: input.name.clone(),
                ty: scalar_type(input.ty),
            })
            .collect(),
        return_type: scalar_type(return_type),
        body,
    };
    let report = ScalarActionReport {
        action_index: index,
        action: action.name.clone(),
        inputs,
        lets,
        emit,
        occurrences,
        return_type,
        generated_function_name: function.name.clone(),
        symbol_domain: "typed-host-symbols; no K numeric-name correspondence claimed".into(),
    };
    Ok(ScalarActionMap { function, report })
}

fn map_expression(
    program: &Program,
    action: usize,
    statement: usize,
    id: ExprId,
    path: Vec<usize>,
    occurrences: &mut Vec<SourceOccurrence>,
    remaining: &mut usize,
) -> Result<ExprRef, ProjectionDiagnostic> {
    let source = program.expressions.get(usize::from(id)).ok_or_else(|| {
        diagnostic(
            action,
            Some(statement),
            Some(id),
            "expression-index",
            "no-such-expression",
        )
    })?;
    let kind = match source {
        #[cfg(feature = "core-floor-div-draft")]
        CoreExpr::FloorDiv { .. } => "floor-div",
        CoreExpr::Literal { .. } => "literal",
        CoreExpr::Name { .. } => "name",
        CoreExpr::Binary { .. } => "binary",
        CoreExpr::Not { .. } => "not",
        CoreExpr::LiabilityPresent { .. } => "liability-present",
        CoreExpr::LiabilityAmount { .. } => "liability-amount",
    };
    charge(remaining, action, Some(statement), Some(id))?;
    occurrences.push(SourceOccurrence {
        action_index: action,
        statement_index: statement,
        expression: Some(id),
        body_path: path.clone(),
        node_kind: kind.into(),
        source_expression: Some(source.clone()),
    });
    let child = |id, position, occurrences: &mut Vec<SourceOccurrence>, remaining: &mut usize| {
        let mut child_path = path.clone();
        child_path.push(position);
        map_expression(
            program,
            action,
            statement,
            id,
            child_path,
            occurrences,
            remaining,
        )
    };
    let mapped = match source {
        #[cfg(feature = "core-floor-div-draft")]
        CoreExpr::FloorDiv { .. } => {
            return Err(diagnostic(action, Some(statement), Some(id), kind, "unsupported-projection"));
        }
        CoreExpr::Literal { value } => Expr::Literal(match value {
            crate::Value::UInt(value) => Value::UInt(value.0),
            crate::Value::Bool(value) => Value::Bool(*value),
        }),
        CoreExpr::Name { name } => Expr::Var(name.clone()),
        CoreExpr::Not { value } => {
            Expr::Unary(UnaryOp::Not, child(*value, 0, occurrences, remaining)?)
        }
        CoreExpr::Binary { op, lhs, rhs } => {
            let op = match op {
                CoreOp::Add => BinaryOp::Add,
                CoreOp::Sub => BinaryOp::Sub,
                CoreOp::Mul => BinaryOp::Mul,
                CoreOp::Eq => BinaryOp::Eq,
                CoreOp::Lt => BinaryOp::Lt,
                other => {
                    return Err(diagnostic(
                        action,
                        Some(statement),
                        Some(id),
                        &format!("binary-{other:?}"),
                        "unsupported-projection",
                    ));
                }
            };
            let left = child(*lhs, 0, occurrences, remaining)?;
            let right = child(*rhs, 1, occurrences, remaining)?;
            Expr::Binary(op, left, right)
        }
        CoreExpr::LiabilityPresent { .. } | CoreExpr::LiabilityAmount { .. } => {
            return Err(diagnostic(
                action,
                Some(statement),
                Some(id),
                kind,
                "unsupported-projection",
            ));
        }
    };
    Ok(Arc::new(mapped))
}

fn statement_kind(statement: &Statement) -> &'static str {
    match statement {
        Statement::Await { .. } => "await",
        Statement::Let { .. } => "let",
        Statement::Requires { .. } => "requires",
        Statement::Ensures { .. } => "ensures",
        Statement::Next { .. } => "next",
        Statement::Emit { .. } => "nonfinal-emit",
        Statement::Transfer { .. } => "transfer",
        Statement::Fee { .. } => "fee",
        Statement::Liability { .. } => "liability",
        Statement::Discharge { .. } => "discharge",
    }
}

#[cfg(test)]
#[path = "source_adapter_tests.rs"]
mod tests;
