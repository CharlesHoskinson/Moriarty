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

/// Required extents, not a working admission implementation. No values or
/// relationships are chosen here. Designated owner owns their final schema,
/// meanings, checks, error representation and source-depth/drop disposition.
#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DraftPreprocessingExtents {
    pub max_source_bytes: UsizeDecimal,
    pub max_tokens: UsizeDecimal,
    pub max_functions: UsizeDecimal,
    pub max_parameters_per_function: UsizeDecimal,
    pub max_total_parameters: UsizeDecimal,
    pub max_ast_nodes: UsizeDecimal,
    pub max_parse_depth: UsizeDecimal,
    pub max_type_depth: UsizeDecimal,
    pub max_copy_depth: UsizeDecimal,
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
