//! SOURCE_ONLY / UNRUN. Ordinary field-projection expectations, written first.
use super::*;
use crate::sherlock::Type;
use crate::sherlock_draft::{
    DraftBudgetExceeded, SourceLocation as ActualLocation, SourceSpan as ActualSpan,
};
use crate::sherlock_draft_types::{DraftTypeBudgetExceeded, DraftTypeBudgetResource};

#[test]
fn typed_depth_budget_preserves_parent_child_and_attempted_count() {
    let actual = types(DraftTypeError {
        source: ActualLocation {
            function_index: Some(2),
            child_path: vec![1],
        },
        kind: DraftTypeErrorKind::Budget(DraftTypeBudgetExceeded {
            resource: DraftTypeBudgetResource::TypeDepth,
            limit: 2,
            observed: Some(3),
            child_index: Some(0),
        }),
    });
    assert_eq!(
        actual,
        json!({"source":{"function_index":"2","child_path":["1"]},
        "kind":{"Budget":{"resource":"TypeDepth","limit":"2","observed":"3","child_index":"0"}}})
    );
}

#[test]
fn typed_depth_budget_retains_explicit_null_dispositions() {
    let actual = types(DraftTypeError {
        source: ActualLocation {
            function_index: None,
            child_path: vec![],
        },
        kind: DraftTypeErrorKind::Budget(DraftTypeBudgetExceeded {
            resource: DraftTypeBudgetResource::TypeDepth,
            limit: 0,
            observed: Some(1),
            child_index: None,
        }),
    });
    assert_eq!(
        actual,
        json!({"source":{"function_index":null,"child_path":[]},
        "kind":{"Budget":{"resource":"TypeDepth","limit":"0","observed":"1","child_index":null}}})
    );
    let actual = types(DraftTypeError {
        source: ActualLocation {
            function_index: Some(0),
            child_path: vec![3],
        },
        kind: DraftTypeErrorKind::Budget(DraftTypeBudgetExceeded {
            resource: DraftTypeBudgetResource::TypeDepth,
            limit: usize::MAX,
            observed: None,
            child_index: Some(1),
        }),
    });
    assert_eq!(
        actual,
        json!({"source":{"function_index":"0","child_path":["3"]},
        "kind":{"Budget":{"resource":"TypeDepth","limit":usize::MAX.to_string(),"observed":null,"child_index":"1"}}})
    );
}

#[test]
fn all_ordinary_type_diagnostic_forms_remain_exact() {
    let cases = [
        (
            DraftTypeErrorKind::UnknownVariable("x".into()),
            json!({"UnknownVariable":"x"}),
        ),
        (
            DraftTypeErrorKind::UnknownFunction("f".into()),
            json!({"UnknownFunction":"f"}),
        ),
        (
            DraftTypeErrorKind::Arity {
                name: "f".into(),
                expected: 2,
                actual: 1,
            },
            json!({"Arity":{"name":"f","expected":"2","actual":"1"}}),
        ),
        (
            DraftTypeErrorKind::Expected {
                expected: Type::Bool,
                actual: Type::UInt,
            },
            json!({"Expected":{"expected":"Bool","actual":"UInt"}}),
        ),
        (
            DraftTypeErrorKind::BranchTypes {
                yes: Type::List,
                no: Type::Tree,
            },
            json!({"BranchTypes":{"yes":"List","no":"Tree"}}),
        ),
        (
            DraftTypeErrorKind::ArmTypes {
                expected: Type::UInt,
                actual: Type::Tree,
            },
            json!({"ArmTypes":{"expected":"UInt","actual":"Tree"}}),
        ),
        (
            DraftTypeErrorKind::DuplicateParameter {
                name: "p".into(),
                index: 1,
            },
            json!({"DuplicateParameter":{"name":"p","index":"1"}}),
        ),
        (
            DraftTypeErrorKind::DuplicatePatternBinder("same".into()),
            json!({"DuplicatePatternBinder":"same"}),
        ),
        (DraftTypeErrorKind::EmptyMatch, json!("EmptyMatch")),
    ];
    for (kind, expected) in cases {
        assert_eq!(
            types(DraftTypeError {
                source: ActualLocation {
                    function_index: Some(3),
                    child_path: vec![2, 1],
                },
                kind
            }),
            json!({"source":{"function_index":"3","child_path":["2","1"]},"kind":expected})
        );
    }
}

#[test]
fn all_ordinary_source_diagnostic_forms_remain_exact() {
    assert_eq!(
        source(DraftError::Lex("lex message".into())),
        json!({"Lex":"lex message"})
    );
    assert_eq!(
        source(DraftError::Syntax {
            span: ActualSpan { start: 4, end: 7 },
            expected: "entry",
            found: "name".into()
        }),
        json!({"Syntax":{"span":{"start":4,"end":7},"expected":"entry","found":"name"}})
    );
    assert_eq!(
        source(DraftError::UIntLiteral {
            span: ActualSpan { start: 8, end: 12 },
            text: "literal".into()
        }),
        json!({"UIntLiteral":{"span":{"start":8,"end":12},"text":"literal"}})
    );
}

#[test]
fn all_existing_parser_budget_resources_keep_typed_names_and_counts() {
    for (resource, name) in [
        (DraftBudgetResource::SourceBytes, "SourceBytes"),
        (DraftBudgetResource::Tokens, "Tokens"),
        (DraftBudgetResource::Functions, "Functions"),
        (
            DraftBudgetResource::ParametersPerFunction,
            "ParametersPerFunction",
        ),
        (DraftBudgetResource::TotalParameters, "TotalParameters"),
        (DraftBudgetResource::AstOccurrences, "AstOccurrences"),
        (
            DraftBudgetResource::ParserRecursionDepth,
            "ParserRecursionDepth",
        ),
        (
            DraftBudgetResource::StructuralAstDepth,
            "StructuralAstDepth",
        ),
    ] {
        assert_eq!(
            source(DraftError::Budget(DraftBudgetExceeded {
                resource,
                limit: 7,
                observed: 8
            })),
            json!({"Budget":{"resource":name,"limit":"7","observed":"8"}})
        );
    }
}

#[test]
fn every_copy_resource_and_optional_location_is_preserved() {
    use crate::sherlock::helper_machine::source_input_bridge::{
        SourceCopyBudgetExceeded, SourceCopyError, SourceCopyResource,
    };
    for (resource, name) in [
        (SourceCopyResource::Functions, "Functions"),
        (
            SourceCopyResource::ParametersPerFunction,
            "ParametersPerFunction",
        ),
        (SourceCopyResource::TotalParameters, "TotalParameters"),
        (SourceCopyResource::ExprNodes, "ExprNodes"),
        (SourceCopyResource::CopyDepth, "CopyDepth"),
        (SourceCopyResource::MatchArms, "MatchArms"),
        (SourceCopyResource::NameBytes, "NameBytes"),
        (SourceCopyResource::ListElements, "ListElements"),
        (SourceCopyResource::TreeNodes, "TreeNodes"),
        (SourceCopyResource::TreeDepth, "TreeDepth"),
    ] {
        assert_eq!(
            copy(SourceCopyError::Budget(SourceCopyBudgetExceeded {
                resource,
                limit: 7,
                observed: Some(8),
                function_index: Some(2),
                child_index: Some(3),
            })),
            json!({"Budget":{"resource":name,"limit":"7","observed":"8","function_index":"2","child_index":"3"}})
        );
    }
    assert_eq!(
        copy(SourceCopyError::Budget(SourceCopyBudgetExceeded {
            resource: SourceCopyResource::NameBytes,
            limit: usize::MAX,
            observed: None,
            function_index: None,
            child_index: None,
        })),
        json!({"Budget":{"resource":"NameBytes","limit":usize::MAX.to_string(),"observed":null,"function_index":null,"child_index":null}})
    );
}
