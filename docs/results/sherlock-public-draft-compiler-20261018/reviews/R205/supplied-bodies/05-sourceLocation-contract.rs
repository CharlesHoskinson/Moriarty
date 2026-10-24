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

