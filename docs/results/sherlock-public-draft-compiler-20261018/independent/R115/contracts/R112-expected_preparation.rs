//! Independent-oracle input interface only; not an expected-table producer.
//! Separate oracle author supplies every field before any subject execution.
//! This code imports artifact types solely to compare actual output. It never
//! imports parser/checker/mapper/evaluator, generates tables, or fills defaults.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use mori_core::sherlock_compile_artifact::draft_report::DraftCompileArtifact;

#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Case { ChainTotal, BranchTotal }

/// Every ProgramView field is mandatory, including empty vectors. Preserve
/// OwnerWire variants, decimal strings, array order and original owner context.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExpectedProgramView {
    pub entry: Value,
    pub expressions: Value,
    pub occurrences: Value,
    pub occurrence_paths: Value,
    pub children: Value,
    pub functions: Value,
    pub parameters: Value,
    pub arms: Value,
    pub patterns: Value,
    pub constants: Value,
    pub constant_fields: Value,
    pub names: Value,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExpectedPreparation {
    pub source_digest: String,
    pub source: Value,
    pub preparation: Value,
    pub program_view: ExpectedProgramView,
}
/// Both programs must be supplied; there is no default, optional or wildcard
/// expectation. Independent authorship/provenance lives in the frozen receipt.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TwoProgramExpectations {
    pub chain_total: ExpectedPreparation,
    pub branch_total: ExpectedPreparation,
}
#[derive(Debug)]
pub enum ComparisonError {
    Serialization(serde_json::Error),
    MissingActualProgramView,
}
pub struct Difference { pub field: &'static str, pub expected: Value, pub actual: Value }

pub fn first_preparation_difference(
    case: Case,
    expected: &TwoProgramExpectations,
    actual: &DraftCompileArtifact,
) -> Result<Option<Difference>, ComparisonError> {
    let expected = match case { Case::ChainTotal => &expected.chain_total,
        Case::BranchTotal => &expected.branch_total };
    let actual_source = serde_json::to_value(&actual.source).map_err(ComparisonError::Serialization)?;
    let actual_preparation = serde_json::to_value(&actual.preparation).map_err(ComparisonError::Serialization)?;
    let actual_view = actual.admission.get("program_view")
        .ok_or(ComparisonError::MissingActualProgramView)?;
    let expected_view = serde_json::to_value(&expected.program_view).map_err(ComparisonError::Serialization)?;
    // Equality preserves complete nested objects/arrays/variants. No context
    // stripping, numeric coercion, sort or producer-derived expectation occurs.
    for (field, expected, actual) in [
        ("source_digest", Value::String(expected.source_digest.clone()),
            Value::String(actual.source.source_digest.clone())),
        ("source", expected.source.clone(), actual_source),
        ("preparation", expected.preparation.clone(), actual_preparation),
        ("admission.program_view", expected_view, actual_view.clone()),
    ] {
        if expected != actual { return Ok(Some(Difference { field, expected, actual })); }
    }
    Ok(None)
}
