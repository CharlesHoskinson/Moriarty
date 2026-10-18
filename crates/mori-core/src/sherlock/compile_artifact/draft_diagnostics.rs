//! Exact ordinary source/type diagnostic field projections. These are not
//! runtime Outcomes. Budget errors retain their typed resources and counts.
//! R218 preparation: SOURCE_ONLY / UNRUN; no reporting resource guard is added.
use serde_json::{json, Value};
use crate::sherlock_draft::{DraftBudgetResource, DraftError};
use crate::sherlock_draft_types::{DraftTypeBudgetResource, DraftTypeError, DraftTypeErrorKind};
use super::draft_report::{SourceLocation, SourceSort, SourceSpan};

pub(super) fn source(error: DraftError) -> Value {
    match error {
        DraftError::Lex(message) => json!({"Lex": message}),
        DraftError::Syntax { span, expected, found } => json!({"Syntax": {
            "span": SourceSpan::from(span), "expected": expected, "found": found,
        }}),
        DraftError::UIntLiteral { span, text } => json!({"UIntLiteral": {
            "span": SourceSpan::from(span), "text": text,
        }}),
        DraftError::Budget(exceeded) => {
            let resource = match exceeded.resource {
                DraftBudgetResource::SourceBytes => "SourceBytes",
                DraftBudgetResource::Tokens => "Tokens",
                DraftBudgetResource::Functions => "Functions",
                DraftBudgetResource::ParametersPerFunction => "ParametersPerFunction",
                DraftBudgetResource::TotalParameters => "TotalParameters",
                DraftBudgetResource::AstOccurrences => "AstOccurrences",
                DraftBudgetResource::ParserRecursionDepth => "ParserRecursionDepth",
                DraftBudgetResource::StructuralAstDepth => "StructuralAstDepth",
            };
            json!({"Budget": {
                "resource": resource,
                "limit": exceeded.limit.to_string(),
                "observed": exceeded.observed.to_string(),
            }})
        },
    }
}

pub(super) fn types(error: DraftTypeError) -> Value {
    let kind = match error.kind {
        DraftTypeErrorKind::UnknownVariable(name) => json!({"UnknownVariable": name}),
        DraftTypeErrorKind::UnknownFunction(name) => json!({"UnknownFunction": name}),
        DraftTypeErrorKind::Arity { name, expected, actual } => json!({"Arity": {
            "name": name, "expected": expected.to_string(), "actual": actual.to_string(),
        }}),
        DraftTypeErrorKind::Expected { expected, actual } => json!({"Expected": {
            "expected": SourceSort::from(expected), "actual": SourceSort::from(actual),
        }}),
        DraftTypeErrorKind::BranchTypes { yes, no } => json!({"BranchTypes": {
            "yes": SourceSort::from(yes), "no": SourceSort::from(no),
        }}),
        DraftTypeErrorKind::ArmTypes { expected, actual } => json!({"ArmTypes": {
            "expected": SourceSort::from(expected), "actual": SourceSort::from(actual),
        }}),
        DraftTypeErrorKind::DuplicateParameter { name, index } => json!({"DuplicateParameter": {
            "name": name, "index": index.to_string(),
        }}),
        DraftTypeErrorKind::DuplicatePatternBinder(name) => json!({"DuplicatePatternBinder": name}),
        DraftTypeErrorKind::EmptyMatch => json!("EmptyMatch"),
        DraftTypeErrorKind::Budget(exceeded) => {
            let resource = match exceeded.resource {
                DraftTypeBudgetResource::TypeDepth => "TypeDepth",
            };
            json!({"Budget": {
                "resource": resource,
                "limit": exceeded.limit.to_string(),
                "observed": exceeded.observed.map(|n| n.to_string()),
                "child_index": exceeded.child_index.map(|n| n.to_string()),
            }})
        },
    };
    json!({"source": SourceLocation::from(&error.source), "kind": kind})
}

pub(super) fn copy(
    error: crate::sherlock::helper_machine::source_input_bridge::SourceCopyError,
) -> Value {
    use crate::sherlock::helper_machine::source_input_bridge::{
        SourceCopyError, SourceCopyResource,
    };
    match error {
        SourceCopyError::Budget(exceeded) => {
            let resource = match exceeded.resource {
                SourceCopyResource::Functions => "Functions",
                SourceCopyResource::ParametersPerFunction => "ParametersPerFunction",
                SourceCopyResource::TotalParameters => "TotalParameters",
                SourceCopyResource::ExprNodes => "ExprNodes",
                SourceCopyResource::CopyDepth => "CopyDepth",
                SourceCopyResource::MatchArms => "MatchArms",
                SourceCopyResource::NameBytes => "NameBytes",
                SourceCopyResource::ListElements => "ListElements",
                SourceCopyResource::TreeNodes => "TreeNodes",
                SourceCopyResource::TreeDepth => "TreeDepth",
            };
            json!({
                "Budget": {
                    "resource": resource,
                    "limit": exceeded.limit.to_string(),
                    "observed": exceeded.observed.map(|n| n.to_string()),
                    "function_index": exceeded.function_index.map(|n| n.to_string()),
                    "child_index": exceeded.child_index.map(|n| n.to_string()),
                }
            })
        }
    }
}

#[cfg(test)]
#[path = "draft_diagnostics_tests.rs"]
mod tests;
