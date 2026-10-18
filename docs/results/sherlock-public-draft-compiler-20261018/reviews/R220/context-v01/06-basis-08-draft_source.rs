//! Unregistered draft syntax preparation using the real Moriarty lexer.
//! Integration proposal: a Core lib.rs path module outside Sherlock's standalone
//! pure-Std graph. No current export, profile registration or execution.
use crate::sherlock::{Arm, BinaryOp, Expr, ExprRef, Function, Name, Parameter,
    Pattern, Program, Type, UnaryOp, Value};
use mori_lexer::{TokenIdx, TokenKind, Tokens};
use std::sync::Arc;

pub const DRAFT_LANGUAGE: &str = "moriarty-sherlock-draft/1";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceSpan { pub start: u32, pub end: u32 }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BinderSite { pub name: Name, pub span: SourceSpan }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParameterSite { pub binder: BinderSite, pub type_span: SourceSpan }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionSite {
    pub index: usize,
    pub span: SourceSpan,
    pub name_span: SourceSpan,
    pub parameters: Vec<ParameterSite>,
    pub return_type_span: SourceSpan,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceLocation {
    /// None is the entry; Some(i) is body i in the unchanged function table.
    pub function_index: Option<usize>,
    /// Existing AST child order, independent of storage/pointer sharing.
    pub child_path: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExpressionSite {
    pub source: SourceLocation,
    pub span: SourceSpan,
    pub kind: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PatternSite {
    pub match_source: SourceLocation,
    pub arm_index: usize,
    pub span: SourceSpan,
    pub binders: Vec<BinderSite>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceProvenance {
    pub language_span: SourceSpan,
    pub program_name_span: SourceSpan,
    pub functions: Vec<FunctionSite>,
    // Image order: entry preorder, then bodies in original function order.
    pub expressions: Vec<ExpressionSite>,
    pub patterns: Vec<PatternSite>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParsedDraft {
    pub name: Name,
    pub program: Arc<Program>,
    pub provenance: SourceProvenance,
}

/// Preparation diagnostics only, never an existing machine Outcome/error code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DraftError {
    Lex(String),
    Syntax { span: SourceSpan, expected: &'static str, found: String },
    UIntLiteral { span: SourceSpan, text: String },
    Budget(DraftBudgetExceeded),
}

/// Inclusive, caller-owned parser limits. No defaults or evaluator admission.
/// AST occurrences count every Expr and Pattern, including repeated source uses;
/// functions and parameters have their own separate limits. Structural depth
/// counts Expr nodes on the longest root-to-leaf path (a leaf has depth one).
/// Parser recursion counts active block/expression/atom/arguments/match/pattern
/// descent entries; parentheses count here even though they add no Expr node.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftParseBudget {
    pub max_source_bytes: usize,
    /// Includes EOF; comments do not count as tokens.
    pub max_tokens: usize,
    pub max_functions: usize,
    pub max_parameters_per_function: usize,
    pub max_total_parameters: usize,
    pub max_ast_occurrences: usize,
    pub max_parser_recursion_depth: usize,
    pub max_structural_ast_depth: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftBudgetResource {
    SourceBytes, Tokens, Functions, ParametersPerFunction, TotalParameters,
    AstOccurrences, ParserRecursionDepth, StructuralAstDepth,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftBudgetExceeded {
    pub resource: DraftBudgetResource,
    pub limit: usize,
    /// Actual source size, or first attempted count/depth above the limit.
    pub observed: usize,
}

fn within(resource: DraftBudgetResource, limit: usize, observed: usize)
    -> Result<(), DraftError>
{
    if observed > limit {
        Err(DraftError::Budget(DraftBudgetExceeded { resource, limit, observed }))
    } else { Ok(()) }
}

///
/// # Security Warning (H1 — Unmetered Host Evaluator)
/// This machine is a host diagnostic reference with zero native authority and no host time,
/// heap, or native stack admission bounds. Do not expose it to untrusted inputs.
pub fn parse_draft_source(source: &str) -> Result<ParsedDraft, DraftError> {
    let tokens = mori_lexer::lex(source).map_err(|error| DraftError::Lex(error.to_string()))?;
    Parser::new(source, tokens, None).program()
}

/// Parse with explicit preparation limits, checked before the affected work.
/// Preserves the legacy grammar and provenance order. The lexer hard limits
/// remain independent. This does not admit the checker, evaluator, general
/// cloning/reporting, or recursive destruction of returned/partial ASTs (H1).
pub fn parse_draft_source_bounded(source: &str, budget: DraftParseBudget)
    -> Result<ParsedDraft, DraftError>
{
    within(DraftBudgetResource::SourceBytes, budget.max_source_bytes, source.len())?;
    let tokens = mori_lexer::lex_with_token_limit(source, budget.max_tokens)
        .map_err(|error| match error {
            mori_lexer::TokenLimitedLexError::Lex(error) => DraftError::Lex(error.to_string()),
            mori_lexer::TokenLimitedLexError::TokenLimit { limit, observed } =>
                DraftError::Budget(DraftBudgetExceeded {
                    resource: DraftBudgetResource::Tokens, limit, observed,
                }),
        })?;
    Parser::new(source, tokens, Some(budget)).program()
}

