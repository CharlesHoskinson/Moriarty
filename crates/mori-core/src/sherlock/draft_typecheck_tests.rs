//! Typed-subset boundary checks. Expectations are handwritten from the draft
//! type rules only.
use super::{DraftTypeError, DraftTypeErrorKind, DraftTypeSummary, check_program_types};
use crate::sherlock::Type;
use crate::sherlock_draft::{DraftError, SourceLocation, parse_draft_source};

fn source(items: &str) -> String {
    format!("language \"moriarty-sherlock-draft/1\";\nprogram T {{\n{items}\n}}\n")
}

fn check(items: &str) -> Result<DraftTypeSummary, DraftTypeError> {
    let parsed = parse_draft_source(&source(items)).expect("well-formed syntax");
    check_program_types(&parsed.program)
}

fn rejected(
    function_index: Option<usize>,
    path: &[usize],
    kind: DraftTypeErrorKind,
) -> Result<DraftTypeSummary, DraftTypeError> {
    Err(DraftTypeError {
        source: SourceLocation {
            function_index,
            child_path: path.to_vec(),
        },
        kind,
    })
}

#[test]
fn unknown_variable_and_arity_are_rejected_before_any_run() {
    assert_eq!(
        check("entry y;"),
        rejected(None, &[], DraftTypeErrorKind::UnknownVariable("y".into()))
    );
    assert_eq!(
        check("fn f(x: UInt): UInt { x } entry f();"),
        rejected(
            None,
            &[],
            DraftTypeErrorKind::Arity {
                name: "f".into(),
                expected: 1,
                actual: 0
            }
        )
    );
}

#[test]
fn declared_return_type_and_duplicate_binders_are_rejected() {
    assert_eq!(
        check("fn f(x: UInt): Bool { x } entry f(1);"),
        rejected(
            Some(0),
            &[],
            DraftTypeErrorKind::Expected {
                expected: Type::Bool,
                actual: Type::UInt
            }
        )
    );
    assert_eq!(
        check("fn f(x: UInt, x: UInt): UInt { x } entry f(1, 2);"),
        rejected(
            Some(0),
            &[],
            DraftTypeErrorKind::DuplicateParameter {
                name: "x".into(),
                index: 1
            }
        )
    );
    assert_eq!(
        check("fn f(xs: List): UInt { match xs { cons(h, h) => h, all => 0 } } entry f(nil);"),
        rejected(
            Some(0),
            &[1],
            DraftTypeErrorKind::DuplicatePatternBinder("h".into())
        )
    );
}

#[test]
fn shadowed_duplicate_function_body_is_still_checked() {
    assert_eq!(
        check("fn f(): UInt { 1 } fn f(): UInt { true } entry f();"),
        rejected(
            Some(1),
            &[],
            DraftTypeErrorKind::Expected {
                expected: Type::UInt,
                actual: Type::Bool
            }
        )
    );
}

#[test]
fn pinned_choices_cross_sort_eq_and_nonexhaustive_match_are_accepted() {
    // DRAFT-TYPE-EQUALITY as proposed: constant false when run under A.
    assert_eq!(
        check("entry 1 == true;"),
        Ok(DraftTypeSummary {
            function_returns: vec![],
            entry_type: Type::Bool
        })
    );
    // DRAFT-TYPE-PARTIAL: accepted here; A would publish error 9 for this entry.
    assert_eq!(
        check("fn f(xs: List): UInt { match xs { nil => 0 } } entry f(cons(1, nil));"),
        Ok(DraftTypeSummary {
            function_returns: vec![Type::UInt],
            entry_type: Type::UInt
        })
    );
}

#[test]
fn uint_literal_overflow_fails_closed_without_wrapping() {
    // Assumes the unchanged lexer has no digit-count limit (number.rs not shown).
    assert!(matches!(
        parse_draft_source(&source("entry 340282366920938463463374607431768211456;")),
        Err(DraftError::UIntLiteral { .. })
    ));
}
