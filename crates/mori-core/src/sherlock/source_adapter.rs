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

/// Maintainer preparation budget: the maximum number of mapping occurrences a
/// single report (summed across its actions) will materialize. Preparation
/// expands the validated Core expression DAG once per traversed edge with no
/// sharing, so a DAG of linear size can demand exponentially many occurrences
/// (and matching generated AST nodes) even within the existing depth-64 and
/// 8192-expression Core bounds. This ceiling is a representation-preparation
/// limit only: it is not a Core schema rule, resource/circuit cost, or any
/// change to the compiler's supported subset. Exhaustion is per-site
/// `Unsupported` metadata, never a compiler refusal. Synthetic scalar-lowering
/// nodes consume this same existing ceiling; it is not increased for lowering.
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
/// Every synthetic internal lowering node also receives one occurrence, marked
/// by a `lowering-` node_kind. Its expression/source_expression identify the
/// original binary requiring it, rather than another traversed source edge.
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
    let actions: Vec<ActionProjection> = program
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
    let lowered = actions.iter().any(|projection| match projection {
        ActionProjection::Prepared { report } => report
            .occurrences
            .iter()
            .any(|site| site.node_kind.starts_with("lowering-")),
        ActionProjection::Unsupported { .. } => false,
    });
    SourceProjectionReport {
        format: FORMAT.into(), source_language: program.language.clone(),
        source_semantics: program.semantics.clone(), agreement: program.name.clone(),
        reference_profile: PROFILE.into(), qualification: "source-static-scalar-projection-only".into(),
        source_function_support: false, source_branch_support: false,
        source_recursion_support: false, source_data_support: false,
        evaluation_performed: false, source_expression_count: program.expressions.len(),
        source_action_count: program.actions.len(), source_state_count: program.states.len(),
        source_asset_count: program.assets.len(),
        coverage: if lowered {
            "all Core/1 scalar binary operators via existing primitives and hygienic eager lowering; stateless actions; ordered lets and one final emit value; source/generated body-preorder mapping; no effects execution or unvisited-site mapping"
        } else {
            "direct scalar nodes; stateless actions; ordered lets and one final emit value; first unsupported site per action; no effects execution or unvisited-site mapping"
        }.into(),
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
    let mut symbols: Option<LoweringSymbols> = None;
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
            &mut symbols,
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
        &mut symbols,
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
    symbols: &mut Option<LoweringSymbols>,
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
        CoreExpr::Literal { .. } => "literal",
        CoreExpr::Name { .. } => "name",
        CoreExpr::Binary { .. } => "binary",
        CoreExpr::Not { .. } => "not",
        CoreExpr::LiabilityPresent { .. } => "liability-present",
        CoreExpr::LiabilityAmount { .. } => "liability-amount",
    };
    let ordinal = occurrences.len();
    charge(remaining, action, Some(statement), Some(id))?;
    occurrences.push(SourceOccurrence {
        action_index: action,
        statement_index: statement,
        expression: Some(id),
        body_path: path.clone(),
        node_kind: kind.into(),
        source_expression: Some(source.clone()),
    });
    let child = |id,
                 position,
                 occurrences: &mut Vec<SourceOccurrence>,
                 symbols: &mut Option<LoweringSymbols>,
                 remaining: &mut usize| {
        let mut child_path = path.clone();
        child_path.push(position);
        map_expression(
            program,
            action,
            statement,
            id,
            child_path,
            occurrences,
            symbols,
            remaining,
        )
    };
    let mapped = match source {
        CoreExpr::Literal { value } => Expr::Literal(match value {
            crate::Value::UInt(value) => Value::UInt(value.0),
            crate::Value::Bool(value) => Value::Bool(*value),
        }),
        CoreExpr::Name { name } => Expr::Var(name.clone()),
        CoreExpr::Not { value } => Expr::Unary(
            UnaryOp::Not,
            child(*value, 0, occurrences, symbols, remaining)?,
        ),
        CoreExpr::Binary { op, lhs, rhs } => {
            match op {
                CoreOp::Add | CoreOp::Sub | CoreOp::Mul | CoreOp::Eq | CoreOp::Lt => {
                    let primitive = match op {
                        CoreOp::Add => BinaryOp::Add,
                        CoreOp::Sub => BinaryOp::Sub,
                        CoreOp::Mul => BinaryOp::Mul,
                        CoreOp::Eq => BinaryOp::Eq,
                        CoreOp::Lt => BinaryOp::Lt,
                        _ => unreachable!("direct scalar operator selected above"),
                    };
                    let left = child(*lhs, 0, occurrences, symbols, remaining)?;
                    let right = child(*rhs, 1, occurrences, symbols, remaining)?;
                    Expr::Binary(primitive, left, right)
                }
                CoreOp::Ne | CoreOp::Ge => {
                    // Both existing primitives are already eager left-to-right.
                    // The added Not wraps their result without reversing operands.
                    let (primitive, kind) = if *op == CoreOp::Ne {
                        (BinaryOp::Eq, "lowering-eq")
                    } else {
                        (BinaryOp::Lt, "lowering-lt")
                    };
                    let inner_path = generated_occurrence(
                        action,
                        statement,
                        id,
                        source,
                        &path,
                        &[0],
                        kind,
                        occurrences,
                        remaining,
                    )?;
                    let left = map_expression(
                        program,
                        action,
                        statement,
                        *lhs,
                        child_path(&inner_path, 0),
                        occurrences,
                        symbols,
                        remaining,
                    )?;
                    let right = map_expression(
                        program,
                        action,
                        statement,
                        *rhs,
                        child_path(&inner_path, 1),
                        occurrences,
                        symbols,
                        remaining,
                    )?;
                    Expr::Unary(UnaryOp::Not, Arc::new(Expr::Binary(primitive, left, right)))
                }
                CoreOp::Gt | CoreOp::Le | CoreOp::And | CoreOp::Or => {
                    return lower_eager_binary(
                        program,
                        action,
                        statement,
                        id,
                        source,
                        *op,
                        *lhs,
                        *rhs,
                        &path,
                        ordinal,
                        occurrences,
                        symbols,
                        remaining,
                    );
                }
            }
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

/// Lazy, action-local fresh symbols. Provenance lives in report occurrences,
/// so generated strings need carry only their unique preorder ordinal/side.
struct LoweringSymbols {
    occupied: std::collections::BTreeSet<String>,
}

impl LoweringSymbols {
    fn new(program: &Program) -> Self {
        let mut occupied = std::collections::BTreeSet::new();
        occupied.insert(program.name.clone());
        for declaration in &program.states {
            occupied.insert(declaration.name.clone());
        }
        for declaration in &program.assets {
            occupied.insert(declaration.name.clone());
        }
        for action in &program.actions {
            occupied.insert(action.name.clone());
            for input in &action.inputs {
                occupied.insert(input.name.clone());
            }
            for statement in &action.statements {
                if let Statement::Let { name, .. } = statement {
                    occupied.insert(name.clone());
                }
            }
        }
        for expression in &program.expressions {
            if let CoreExpr::Name { name } = expression {
                occupied.insert(name.clone());
            }
        }
        Self { occupied }
    }

    fn fresh(&mut self, ordinal: usize, side: &str) -> String {
        let mut candidate = format!("${ordinal}{side}");
        while self.occupied.contains(&candidate) {
            candidate.push('$');
        }
        self.occupied.insert(candidate.clone());
        candidate
    }
}

fn child_path(path: &[usize], position: usize) -> Vec<usize> {
    let mut result = path.to_vec();
    result.push(position);
    result
}

/// Record a real internal AST node against the unchanged report budget before
/// allocating its path/record. The origin is the source binary requiring it.
fn generated_occurrence(
    action: usize,
    statement: usize,
    id: ExprId,
    source: &CoreExpr,
    parent: &[usize],
    positions: &[usize],
    kind: &str,
    occurrences: &mut Vec<SourceOccurrence>,
    remaining: &mut usize,
) -> Result<Vec<usize>, ProjectionDiagnostic> {
    charge(remaining, action, Some(statement), Some(id))?;
    let mut path = parent.to_vec();
    path.extend_from_slice(positions);
    occurrences.push(SourceOccurrence {
        action_index: action,
        statement_index: statement,
        expression: Some(id),
        body_path: path.clone(),
        node_kind: kind.into(),
        source_expression: Some(source.clone()),
    });
    Ok(path)
}

/// Existing Let sequencing forces both operands before a reversed comparison
/// or conditional. `$` is outside validated Core identifiers. The preorder
/// ordinal distinguishes each unshared source occurrence, including DAG edges.
fn lower_eager_binary(
    program: &Program,
    action: usize,
    statement: usize,
    id: ExprId,
    source: &CoreExpr,
    op: CoreOp,
    lhs: ExprId,
    rhs: ExprId,
    path: &[usize],
    ordinal: usize,
    occurrences: &mut Vec<SourceOccurrence>,
    symbols: &mut Option<LoweringSymbols>,
    remaining: &mut usize,
) -> Result<ExprRef, ProjectionDiagnostic> {
    let (left_name, right_name) = {
        let supply = symbols.get_or_insert_with(|| LoweringSymbols::new(program));
        (supply.fresh(ordinal, "l"), supply.fresh(ordinal, "r"))
    };
    // The outer Let's source-root occurrence was recorded by map_expression.
    let left = map_expression(
        program,
        action,
        statement,
        lhs,
        child_path(path, 0),
        occurrences,
        symbols,
        remaining,
    )?;
    let right_let_path = generated_occurrence(
        action,
        statement,
        id,
        source,
        path,
        &[1],
        "lowering-let",
        occurrences,
        remaining,
    )?;
    let right = map_expression(
        program,
        action,
        statement,
        rhs,
        child_path(&right_let_path, 0),
        occurrences,
        symbols,
        remaining,
    )?;
    let body = match op {
        CoreOp::Gt | CoreOp::Le => {
            let comparison_path = if op == CoreOp::Le {
                let not_path = generated_occurrence(
                    action,
                    statement,
                    id,
                    source,
                    &right_let_path,
                    &[1],
                    "lowering-not",
                    occurrences,
                    remaining,
                )?;
                generated_occurrence(
                    action,
                    statement,
                    id,
                    source,
                    &not_path,
                    &[0],
                    "lowering-lt",
                    occurrences,
                    remaining,
                )?
            } else {
                generated_occurrence(
                    action,
                    statement,
                    id,
                    source,
                    &right_let_path,
                    &[1],
                    "lowering-lt",
                    occurrences,
                    remaining,
                )?
            };
            generated_occurrence(
                action,
                statement,
                id,
                source,
                &comparison_path,
                &[0],
                "lowering-name",
                occurrences,
                remaining,
            )?;
            generated_occurrence(
                action,
                statement,
                id,
                source,
                &comparison_path,
                &[1],
                "lowering-name",
                occurrences,
                remaining,
            )?;
            let comparison = Arc::new(Expr::Binary(
                BinaryOp::Lt,
                Arc::new(Expr::Var(right_name.clone())),
                Arc::new(Expr::Var(left_name.clone())),
            ));
            if op == CoreOp::Le {
                Arc::new(Expr::Unary(UnaryOp::Not, comparison))
            } else {
                comparison
            }
        }
        CoreOp::And | CoreOp::Or => {
            let conditional_path = generated_occurrence(
                action,
                statement,
                id,
                source,
                &right_let_path,
                &[1],
                "lowering-if",
                occurrences,
                remaining,
            )?;
            generated_occurrence(
                action,
                statement,
                id,
                source,
                &conditional_path,
                &[0],
                "lowering-name",
                occurrences,
                remaining,
            )?;
            let (yes, no) = if op == CoreOp::And {
                generated_occurrence(
                    action,
                    statement,
                    id,
                    source,
                    &conditional_path,
                    &[1],
                    "lowering-name",
                    occurrences,
                    remaining,
                )?;
                generated_occurrence(
                    action,
                    statement,
                    id,
                    source,
                    &conditional_path,
                    &[2],
                    "lowering-literal",
                    occurrences,
                    remaining,
                )?;
                (
                    Arc::new(Expr::Var(right_name.clone())),
                    Arc::new(Expr::Literal(Value::Bool(false))),
                )
            } else {
                generated_occurrence(
                    action,
                    statement,
                    id,
                    source,
                    &conditional_path,
                    &[1],
                    "lowering-literal",
                    occurrences,
                    remaining,
                )?;
                generated_occurrence(
                    action,
                    statement,
                    id,
                    source,
                    &conditional_path,
                    &[2],
                    "lowering-name",
                    occurrences,
                    remaining,
                )?;
                (
                    Arc::new(Expr::Literal(Value::Bool(true))),
                    Arc::new(Expr::Var(right_name.clone())),
                )
            };
            Arc::new(Expr::If(Arc::new(Expr::Var(left_name.clone())), yes, no))
        }
        _ => unreachable!("only eager derived operators call lower_eager_binary"),
    };
    Ok(Arc::new(Expr::Let(
        left_name,
        left,
        Arc::new(Expr::Let(right_name, right, body)),
    )))
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

#[cfg(test)]
#[path = "source_adapter_operator_tests.rs"]
mod operator_tests;
