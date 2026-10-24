//! Ordinary source/ABI proposal, unregistered and unexecuted.
//! No preprocessing policy, guard implementation or numerical defaults live here.
use super::config::{Bounds, Finance, Grammar, Layout, Profile, RunInput, UsizeDecimal};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub const SOURCE_PROFILE: &str = "moriarty-sherlock-draft/1";
// Root-selected dedicated semantics identity; never a Core/1 alias.
pub const SOURCE_SEMANTICS: &str = "moriarty-sherlock-dedicated/1";
pub const ARTIFACT_FORMAT: &str = "sherlock-host-artifact/2";

/// Selection on the already-selected Sherlock compile route only.
/// Omitted source profile keeps Core action selection and never tries draft parsing.
// NominalEntry selects grammar; the caller retains the explicit source profile
// to distinguish structural preparation from body-typed preparation.
pub enum SourceSelection<'a> {
    CoreAction(&'a str),
    DraftEntry,
    NominalEntry,
}
#[derive(Debug, PartialEq, Eq)]
pub enum SelectionError {
    UnknownProfile,
    MissingAction,
    MissingEntry,
    ConflictingSelection,
}
pub fn select_source<'a>(
    profile: Option<&str>,
    action: Option<&'a str>,
    entry: bool,
) -> Result<SourceSelection<'a>, SelectionError> {
    match profile {
        None | Some("moriarty/1") => {
            if entry {
                return Err(SelectionError::ConflictingSelection);
            }
            action
                .map(SourceSelection::CoreAction)
                .ok_or(SelectionError::MissingAction)
        }
        Some(SOURCE_PROFILE) => {
            if action.is_some() {
                return Err(SelectionError::ConflictingSelection);
            }
            if !entry {
                return Err(SelectionError::MissingEntry);
            }
            Ok(SourceSelection::DraftEntry)
        }
        Some(crate::sherlock_nominal_source::NOMINAL_LANGUAGE)
        | Some(super::nominal_body_compile::NOMINAL_BODY_TYPED_PROFILE) => {
            if action.is_some() {
                return Err(SelectionError::ConflictingSelection);
            }
            if !entry {
                return Err(SelectionError::MissingEntry);
            }
            Ok(SourceSelection::NominalEntry)
        }
        Some(_) => Err(SelectionError::UnknownProfile),
    }
}

#[derive(Clone, Copy)]
pub enum DraftConfigFormat {
    V1,
}
impl Serialize for DraftConfigFormat {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str("sherlock-draft-host-config/1")
    }
}
impl<'de> Deserialize<'de> for DraftConfigFormat {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        if text == "sherlock-draft-host-config/1" {
            Ok(Self::V1)
        } else {
            Err(serde::de::Error::custom(
                "expected sherlock-draft-host-config/1",
            ))
        }
    }
}

/// Explicit extents; the parser projection below chooses no numerical values.
/// Bounded-checker wiring consumes the exact max_type_depth.0.
/// Copy fields below project the exact returned R208 ABI; no numerical policy.
/// Reporting/cleanup and global H1 guarantees remain separate work.
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
    /// Structural Expr copy depth, independent of parser and checker depth.
    pub max_copy_depth: UsizeDecimal,
    pub max_copy_expr_nodes: UsizeDecimal,
    pub max_copy_match_arms: UsizeDecimal,
    pub max_copy_name_bytes: UsizeDecimal,
    pub max_copy_list_elements: UsizeDecimal,
    pub max_copy_tree_nodes: UsizeDecimal,
    pub max_copy_tree_depth: UsizeDecimal,
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

impl DraftPreprocessingExtents {
    pub fn copy_budget(
        self,
    ) -> crate::sherlock::helper_machine::source_input_bridge::SourceCopyBudget {
        crate::sherlock::helper_machine::source_input_bridge::SourceCopyBudget {
            max_functions: self.max_functions.0,
            max_parameters_per_function: self.max_parameters_per_function.0,
            max_total_parameters: self.max_total_parameters.0,
            max_expr_nodes: self.max_copy_expr_nodes.0,
            max_copy_depth: self.max_copy_depth.0,
            max_match_arms: self.max_copy_match_arms.0,
            max_name_bytes: self.max_copy_name_bytes.0,
            max_list_elements: self.max_copy_list_elements.0,
            max_tree_nodes: self.max_copy_tree_nodes.0,
            max_tree_depth: self.max_copy_tree_depth.0,
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
    #[serde(flatten)]
    pub fields: DraftExecutionConfig,
    pub file_digest: String,
}

#[derive(Clone, Copy, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum EntrySelection {
    Entry,
}

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
