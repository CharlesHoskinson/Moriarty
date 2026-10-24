//! Exact ordinary source/type diagnostic field projections. These are not
//! runtime Outcomes and do not interpret designated-owner guard diagnostics.
use serde_json::{json, Value};
use crate::sherlock_draft::DraftError;
use crate::sherlock_draft_types::{DraftTypeError, DraftTypeErrorKind};
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
    };
    json!({"source": SourceLocation::from(&error.source), "kind": kind})
}
