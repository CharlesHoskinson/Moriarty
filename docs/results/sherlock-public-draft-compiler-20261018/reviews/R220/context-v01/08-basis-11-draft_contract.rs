//! Ordinary source/ABI proposal, unregistered and unexecuted.
//! No preprocessing policy, guard implementation or numerical defaults live here.
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use super::config::{Bounds, Finance, Grammar, Layout, Profile, RunInput, UsizeDecimal};

pub const SOURCE_PROFILE: &str = "moriarty-sherlock-draft/1";
// Root-selected dedicated semantics identity; never a Core/1 alias.
pub const SOURCE_SEMANTICS: &str = "moriarty-sherlock-dedicated/1";
pub const ARTIFACT_FORMAT: &str = "sherlock-host-artifact/2";

/// Selection on the already-selected Sherlock compile route only.
/// Omitted source profile keeps Core action selection and never tries draft parsing.
pub enum SourceSelection<'a> { CoreAction(&'a str), DraftEntry }
#[derive(Debug, PartialEq, Eq)]
pub enum SelectionError { UnknownProfile, MissingAction, MissingEntry, ConflictingSelection }
pub fn select_source<'a>(profile: Option<&str>, action: Option<&'a str>, entry: bool)
    -> Result<SourceSelection<'a>, SelectionError>
{
    match profile {
        None | Some("moriarty/1") => {
            if entry { return Err(SelectionError::ConflictingSelection); }
            action.map(SourceSelection::CoreAction).ok_or(SelectionError::MissingAction)
        }
        Some(SOURCE_PROFILE) => {
            if action.is_some() { return Err(SelectionError::ConflictingSelection); }
            if !entry { return Err(SelectionError::MissingEntry); }
            Ok(SourceSelection::DraftEntry)
        }
        Some(_) => Err(SelectionError::UnknownProfile),
    }
}

#[derive(Clone, Copy)]
pub enum DraftConfigFormat { V1 }
impl Serialize for DraftConfigFormat {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str("sherlock-draft-host-config/1")
    }
}
impl<'de> Deserialize<'de> for DraftConfigFormat {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        if text == "sherlock-draft-host-config/1" { Ok(Self::V1) }
        else { Err(serde::de::Error::custom("expected sherlock-draft-host-config/1")) }
    }
}

/// Explicit extents; the parser projection below chooses no numerical values.
/// Bounded-checker wiring consumes the exact max_type_depth.0.
/// Copy/report/drop admission and the public constructor remain separate work.
#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DraftPreprocessingExtents {
    pub max_source_bytes: UsizeDecimal,
    pub max_tokens: UsizeDecimal,
    pub max_functions: UsizeDecimal,
    pub max_parameters_per_function: UsizeDecimal,
    pub max_total_parameters: UsizeDecimal,
    /// Syntactic Expr + Pattern occurrences, including repeated source uses.
    /// Maps to DraftParseBudget::max_ast_occurrences, without pointer deduplication.
    pub max_ast_nodes: UsizeDecimal,
    /// Active parser descent bodies; maps to max_parser_recursion_depth.
    /// Parentheses consume this extent even though they create no Expr node.
    pub max_parse_depth: UsizeDecimal,
    /// Longest Expr root-to-leaf path, leaf depth one; distinct from parser,
    /// checker and copy depth. No default or alias to another extent.
    pub max_structural_ast_depth: UsizeDecimal,
    /// Active structural Expr inference depth: each body/entry root is1.
    /// Consumed unchanged by check_program_types_bounded; no parser/copy alias.
    pub max_type_depth: UsizeDecimal,
    /// Reserved for the designated bounded copy constructor, not silently ignored
    /// by an alleged complete public compile path. No copy policy is added here.
    pub max_copy_depth: UsizeDecimal,
}

impl DraftPreprocessingExtents {
    /// Exact parser projection only; no checker/copy/drop/evaluator admission.
    pub fn parser_budget(self) -> crate::sherlock_draft::DraftParseBudget {
        crate::sherlock_draft::DraftParseBudget {
            max_source_bytes: self.max_source_bytes.0,
            max_tokens: self.max_tokens.0,
            max_functions: self.max_functions.0,
            max_parameters_per_function: self.max_parameters_per_function.0,
            max_total_parameters: self.max_total_parameters.0,
            max_ast_occurrences: self.max_ast_nodes.0,
            max_parser_recursion_depth: self.max_parse_depth.0,
            max_structural_ast_depth: self.max_structural_ast_depth.0,
        }
    }
}

/// Separate schema: Core HostBounds action fields are never silently renamed
/// to draft function/parameter limits. Existing artifact/config /1 are untouched.
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DraftExecutionConfig {
    pub format: DraftConfigFormat,
    pub profile: Profile,
    pub grammar: Grammar,
    pub finance: Finance,
    pub bounds: Bounds,
    pub layout: Layout,
    pub fuel: RunInput,
    pub stack: RunInput,
    pub preprocessing: DraftPreprocessingExtents,
}
#[derive(Serialize)]
pub struct DraftConfigIdentity {
    #[serde(flatten)] pub fields: DraftExecutionConfig,
    pub file_digest: String,
}

#[derive(Clone, Copy, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum EntrySelection { Entry }

#[derive(Serialize)]
pub struct DraftSourceIdentity {
    pub language: &'static str,
    pub semantics: &'static str,
    pub program: String,
    pub selection: EntrySelection,
    pub source_digest: String,
}

#[cfg(test)]
#[path = "draft_contract_tests.rs"]
mod tests;
