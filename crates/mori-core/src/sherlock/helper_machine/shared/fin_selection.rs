//! Proposed MU2 financial receiving selection. Not adopted.
//!
//! This is a separate typed namespace. It does not reuse RuleIdC, raw-104
//! discriminants, FamilyPlans, Selected, or StepEventC. A selection is a
//! receiving-validation header for the statement at the actual financial
//! cursor. It is not an applied Step, an event publication, a segment
//! commit, or evidence that MU5 exists.

use super::fin_carrier::FinPreparedStagedSource;
use super::fin_exec_state::{FinExecInvariantError, check_fin_exec_invariant};
use super::{BoundaryKind, HostViolation, OutcomeC, PreImage};
use crate::{Expr, Statement, UInt, Value};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FinStatementKind {
    Let,
    Liability,
    Transfer,
    Fee,
    Discharge,
    Next,
    Emit,
    Requires,
    Ensures,
    Await,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FinRuleId {
    LetApply,
    LiabilityApply,
    TransferApply,
    FeeApply,
    DischargeApply,
    NextApply,
    EmitApply,
    RequiresApply,
    EnsuresApply,
    AwaitEnter,
}

impl FinRuleId {
    /// Retained requested XFin identity, as a separate-namespace wire name.
    /// This is not a RawRuleId conversion and must not be fed to rule_id_of,
    /// raw_rule_id, family_of, event_kind_of, or visits_of.
    pub fn retained_name(self) -> &'static str {
        match self {
            Self::LetApply => "XFinLetApply",
            Self::LiabilityApply => "XFinLiabilityApply",
            Self::TransferApply => "XFinTransferApply",
            Self::FeeApply => "XFinFeeApply",
            Self::DischargeApply => "XFinDischargeApply",
            Self::NextApply => "XFinNextApply",
            Self::EmitApply => "XFinEmitApply",
            Self::RequiresApply => "XFinRequiresApply",
            Self::EnsuresApply => "XFinEnsuresApply",
            Self::AwaitEnter => "XFinAwaitEnter",
        }
    }

    pub fn statement_kind(self) -> FinStatementKind {
        match self {
            Self::LetApply => FinStatementKind::Let,
            Self::LiabilityApply => FinStatementKind::Liability,
            Self::TransferApply => FinStatementKind::Transfer,
            Self::FeeApply => FinStatementKind::Fee,
            Self::DischargeApply => FinStatementKind::Discharge,
            Self::NextApply => FinStatementKind::Next,
            Self::EmitApply => FinStatementKind::Emit,
            Self::RequiresApply => FinStatementKind::Requires,
            Self::EnsuresApply => FinStatementKind::Ensures,
            Self::AwaitEnter => FinStatementKind::Await,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FinSelectedStatement {
    Let {
        name: String,
        value_expr: crate::ExprId,
        value_literal: Option<u128>,
        value_name: Option<String>,
    },
    Liability {
        name: String,
        asset: String,
        debtor: String,
        creditor: String,
        amount_expr: crate::ExprId,
        amount_literal: Option<u128>,
        amount_name: Option<String>,
    },
    Transfer {
        asset: String,
        from: String,
        to: String,
        amount_expr: crate::ExprId,
        amount_literal: Option<u128>,
        amount_name: Option<String>,
    },
    Fee {
        asset: String,
        from: String,
        to: String,
        amount_expr: crate::ExprId,
        amount_literal: Option<u128>,
        amount_name: Option<String>,
    },
    Discharge {
        name: String,
        amount_expr: crate::ExprId,
        amount_literal: Option<u128>,
        amount_name: Option<String>,
    },
    Next {
        state: String,
        value_expr: crate::ExprId,
        value_literal: Option<u128>,
        value_name: Option<String>,
    },
    Emit {
        name: String,
        value_expr: crate::ExprId,
        value_literal: Option<u128>,
        value_name: Option<String>,
    },
    Requires {
        predicate_expr: crate::ExprId,
    },
    Ensures {
        predicate_expr: crate::ExprId,
    },
    Await {
        label: String,
        binder_names: Vec<String>,
        guard_expr: crate::ExprId,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FinSelectionHeader {
    pub format: &'static str,
    pub rule: FinRuleId,
    pub action_index: usize,
    pub action_name: String,
    pub statement_cursor: usize,
    pub segment_index: usize,
    pub statement_kind: FinStatementKind,
    pub committed_before: u128,
    pub segment_src_cost: u128,
    pub plan_total: u128,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FinSelection {
    pub header: FinSelectionHeader,
    pub statement: FinSelectedStatement,
    /// Exact source root expression for Let/Liability/Transfer/Fee/
    /// Discharge/Next/Emit statement payloads; the predicate or guard
    /// expression for Requires/Ensures/Await. This is a checked read, not
    /// an evaluation.
    pub root_expression: Expr,
}

impl FinSelection {
    pub fn event_header(&self) -> FinEventHeader {
        FinEventHeader {
            format: "financial-event-header/1",
            rule: self.header.rule,
            action_index: self.header.action_index,
            action_name: self.header.action_name.clone(),
            statement_cursor: self.header.statement_cursor,
            segment_index: self.header.segment_index,
            outcome: OutcomeC::Running,
            boundary: BoundaryKind::None,
        }
    }
}

/// Header-shaped consumer record only. It is deliberately not StepEventC:
/// no B work, helper work, reference-cursor, allocation, ordinal, or
/// financial application has occurred. Its `outcome` is the prospective
/// declared label `OutcomeC::Running`; `select_fin` enforces that label's
/// bounded domain by refusing every non-Running image, so a header can
/// only be derived from a selection taken on a live image. It is never
/// a report of an observed post-step outcome.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FinEventHeader {
    pub format: &'static str,
    pub rule: FinRuleId,
    pub action_index: usize,
    pub action_name: String,
    pub statement_cursor: usize,
    pub segment_index: usize,
    pub outcome: OutcomeC,
    pub boundary: BoundaryKind,
}

#[derive(Debug)]
pub enum FinSelectionError {
    Binding(HostViolation),
    NotFinancialAdmission,
    MissingFinancialState,
    NonRunningOutcome {
        outcome: OutcomeC,
    },
    Invariant(FinExecInvariantError),
    PendingHelperOrResult,
    ActionOutOfRange {
        action_index: usize,
        action_count: usize,
    },
    StatementOutOfRange {
        cursor: usize,
        statement_count: usize,
    },
    ExpressionOutOfRange {
        expr_id: crate::ExprId,
    },
}

impl From<FinExecInvariantError> for FinSelectionError {
    fn from(error: FinExecInvariantError) -> Self {
        Self::Invariant(error)
    }
}

fn literal_of(expr: &Expr) -> Option<u128> {
    match expr {
        Expr::Literal {
            value: Value::UInt(UInt(n)),
        } => Some(*n),
        _ => None,
    }
}

fn name_of(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Name { name } => Some(name.clone()),
        _ => None,
    }
}

fn checked_expr(
    prepared: &FinPreparedStagedSource,
    id: crate::ExprId,
) -> Result<Expr, FinSelectionError> {
    prepared
        .expression(id)
        .cloned()
        .ok_or(FinSelectionError::ExpressionOutOfRange { expr_id: id })
}

fn rule_for(statement: &Statement) -> FinRuleId {
    match statement {
        Statement::Let { .. } => FinRuleId::LetApply,
        Statement::Liability { .. } => FinRuleId::LiabilityApply,
        Statement::Transfer { .. } => FinRuleId::TransferApply,
        Statement::Fee { .. } => FinRuleId::FeeApply,
        Statement::Discharge { .. } => FinRuleId::DischargeApply,
        Statement::Next { .. } => FinRuleId::NextApply,
        Statement::Emit { .. } => FinRuleId::EmitApply,
        Statement::Requires { .. } => FinRuleId::RequiresApply,
        Statement::Ensures { .. } => FinRuleId::EnsuresApply,
        Statement::Await { .. } => FinRuleId::AwaitEnter,
    }
}

/// AB-bound receiving selection. `pre` must already come from
/// `PreImage::bind`; this function performs no admission, no bind bypass,
/// no mutation, no evaluation, and no hidden Core/2 segment execution.
pub fn select_fin(pre: &PreImage<'_>) -> Result<FinSelection, FinSelectionError> {
    let Some(prepared) = pre.admitted.body.fin.as_ref() else {
        return Err(FinSelectionError::NotFinancialAdmission);
    };
    let Some(state) = pre.img.fin.as_ref() else {
        return Err(FinSelectionError::MissingFinancialState);
    };

    check_fin_exec_invariant(state, prepared)?;
    // Bounded domain of the receiving header: only a live Running image
    // may be selected. The OutcomeC::Running written by
    // FinSelection::event_header is a prospective declared label for a
    // selection produced inside this domain; it is not an observed
    // outcome, an applied event, or a StepInv claim. A non-Running
    // image is refused here, before any pending-result competition or
    // statement read, mirroring the pure owner selector's refusal to
    // select on a non-Running image. This function takes &PreImage and
    // never mutates the image.
    if pre.img.outcome != OutcomeC::Running {
        return Err(FinSelectionError::NonRunningOutcome {
            outcome: pre.img.outcome,
        });
    }

    // MU2 does not consume a staged B helper/result/task. Such a pair is a
    // competing premise (CE11 scope for this unit), not a statement fallthrough.
    if pre.img.helper.is_some() || pre.img.result.is_some() || !pre.img.tasks.is_empty() {
        return Err(FinSelectionError::PendingHelperOrResult);
    }

    let action_count = prepared.program().actions.len();
    let Some(source_action) = prepared.program().actions.get(state.action_index) else {
        return Err(FinSelectionError::ActionOutOfRange {
            action_index: state.action_index,
            action_count,
        });
    };
    let Some(fin_action) = prepared.fin_plan().actions.get(state.action_index) else {
        return Err(FinSelectionError::ActionOutOfRange {
            action_index: state.action_index,
            action_count,
        });
    };

    let cursor = state.exec.statement_cursor;
    let Some(statement) = source_action.statements.get(cursor) else {
        return Err(FinSelectionError::StatementOutOfRange {
            cursor,
            statement_count: source_action.statements.len(),
        });
    };

    let (selected, root_id) = match statement {
        Statement::Let { name, value } => {
            let root = checked_expr(prepared, *value)?;
            (
                FinSelectedStatement::Let {
                    name: name.clone(),
                    value_expr: *value,
                    value_literal: literal_of(&root),
                    value_name: name_of(&root),
                },
                *value,
            )
        }
        Statement::Liability {
            name,
            asset,
            debtor,
            creditor,
            amount,
        } => {
            let root = checked_expr(prepared, *amount)?;
            (
                FinSelectedStatement::Liability {
                    name: name.clone(),
                    asset: asset.clone(),
                    debtor: debtor.clone(),
                    creditor: creditor.clone(),
                    amount_expr: *amount,
                    amount_literal: literal_of(&root),
                    amount_name: name_of(&root),
                },
                *amount,
            )
        }
        Statement::Transfer {
            asset,
            from,
            to,
            amount,
        } => {
            let root = checked_expr(prepared, *amount)?;
            (
                FinSelectedStatement::Transfer {
                    asset: asset.clone(),
                    from: from.clone(),
                    to: to.clone(),
                    amount_expr: *amount,
                    amount_literal: literal_of(&root),
                    amount_name: name_of(&root),
                },
                *amount,
            )
        }
        Statement::Fee {
            asset,
            from,
            to,
            amount,
        } => {
            let root = checked_expr(prepared, *amount)?;
            (
                FinSelectedStatement::Fee {
                    asset: asset.clone(),
                    from: from.clone(),
                    to: to.clone(),
                    amount_expr: *amount,
                    amount_literal: literal_of(&root),
                    amount_name: name_of(&root),
                },
                *amount,
            )
        }
        Statement::Discharge { name, amount } => {
            let root = checked_expr(prepared, *amount)?;
            (
                FinSelectedStatement::Discharge {
                    name: name.clone(),
                    amount_expr: *amount,
                    amount_literal: literal_of(&root),
                    amount_name: name_of(&root),
                },
                *amount,
            )
        }
        Statement::Next { state, value } => {
            let root = checked_expr(prepared, *value)?;
            (
                FinSelectedStatement::Next {
                    state: state.clone(),
                    value_expr: *value,
                    value_literal: literal_of(&root),
                    value_name: name_of(&root),
                },
                *value,
            )
        }
        Statement::Emit { name, value } => {
            let root = checked_expr(prepared, *value)?;
            (
                FinSelectedStatement::Emit {
                    name: name.clone(),
                    value_expr: *value,
                    value_literal: literal_of(&root),
                    value_name: name_of(&root),
                },
                *value,
            )
        }
        Statement::Requires { predicate } => (
            FinSelectedStatement::Requires {
                predicate_expr: *predicate,
            },
            *predicate,
        ),
        Statement::Ensures { predicate } => (
            FinSelectedStatement::Ensures {
                predicate_expr: *predicate,
            },
            *predicate,
        ),
        Statement::Await {
            label,
            binders,
            guard,
        } => (
            FinSelectedStatement::Await {
                label: label.clone(),
                binder_names: binders.iter().map(|b| b.name.clone()).collect(),
                guard_expr: *guard,
            },
            *guard,
        ),
    };
    let root_expression = checked_expr(prepared, root_id)?;

    let segment_index = state.exec.segment_index;
    let committed_before = fin_action
        .committed_before
        .get(segment_index)
        .copied()
        .ok_or(FinExecInvariantError::SegmentOutOfRange {
            segment_index,
            segment_count: fin_action.segment_src_cost.len(),
        })?;
    let segment_src_cost = fin_action
        .segment_src_cost
        .get(segment_index)
        .copied()
        .ok_or(FinExecInvariantError::SegmentOutOfRange {
            segment_index,
            segment_count: fin_action.segment_src_cost.len(),
        })?;

    let rule = rule_for(statement);
    Ok(FinSelection {
        header: FinSelectionHeader {
            format: "financial-selection/1",
            rule,
            action_index: state.action_index,
            action_name: source_action.name.clone(),
            statement_cursor: cursor,
            segment_index,
            statement_kind: rule.statement_kind(),
            committed_before,
            segment_src_cost,
            plan_total: fin_action.plan_total,
        },
        statement: selected,
        root_expression,
    })
}
