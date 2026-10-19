//! Explicit body-typed nominal command over the actual public owner wrapper.
//! Structural nominal compilation remains its separate unchanged command.
use serde_json::{json, Value};
use super::{CompileFailure, nominal_command::NominalConfig, wire::{self, OwnerWire}};
use super::draft_report::{SourceLocation, SourceSpan};
use super::nominal_body_compile::{
    compile_nominal_source_host_bounded, NominalBodyCompileError,
    NominalBodyCompileFailure, ADOPTED_NOMINAL_JUDGMENTS, NOMINAL_BODY_TYPED_PROFILE,
};
use crate::sherlock_nominal_body_types::{NominalBodyTypeError, NominalBodyTypeErrorKind};
use crate::sherlock_nominal_source::NOMINAL_LANGUAGE;

pub const ARTIFACT_FORMAT: &str = "sherlock-nominal-body-typed-host-artifact/1";

/// Exact original payload projection plus separate body disposition. Existing
/// CompileFailure.completed describes runtime completion and remains false.
fn diagnostic(stage: &'static str, error: Value, span: Option<SourceSpan>, status: &'static str)
    -> CompileFailure
{
    CompileFailure {
        stage,
        actual_error: json!({
            "source_profile": NOMINAL_BODY_TYPED_PROFILE, "error": error, "span": span,
            "body_typechecking": {"status": status,
                "judgments": if status == "completed" { Some(ADOPTED_NOMINAL_JUDGMENTS) } else { None }},
        }),
        evaluation_performed: false, events_forwarded: "0".into(), completed: false,
    }
}
fn body_error(error: NominalBodyTypeError) -> Value {
    use NominalBodyTypeErrorKind as K;
    let source = SourceLocation::from(&error.source);
    let kind = match error.kind {
        K::UnknownVariable(name) => json!({"UnknownVariable": name}),
        K::UnknownFunction(name) => json!({"UnknownFunction": name}),
        K::Arity { name, expected, actual } => json!({"Arity": {
            "name": name, "expected": expected.to_string(), "actual": actual.to_string(),
        }}),
        K::Expected { expected, actual } => json!({"Expected": {
            "expected": expected.owner_wire(), "actual": actual.owner_wire(),
        }}),
        K::BranchTypes { yes, no } => json!({"BranchTypes": {"yes": yes.owner_wire(), "no": no.owner_wire()}}),
        K::ArmTypes { expected, actual } => json!({"ArmTypes": {
            "expected": expected.owner_wire(), "actual": actual.owner_wire(),
        }}),
        K::DuplicateParameter { name, index } => json!({"DuplicateParameter": {"name": name, "index": index.to_string()}}),
        K::DuplicatePatternBinder(name) => json!({"DuplicatePatternBinder": name}),
        K::EmptyMatch => json!("EmptyMatch"),
        K::UnsupportedDomain => json!("UnsupportedDomain"),
        K::UnsupportedExpression(name) => json!({"UnsupportedExpression": name}),
        K::IncompletePreparation(detail) => json!({"IncompletePreparation": detail}),
        K::Preparation(error) => json!({"Preparation": error.owner_wire()}),
    };
    json!({"source": source, "kind": kind})
}
fn program_failure(failure: NominalBodyCompileFailure) -> CompileFailure {
    let status = if failure.body_check_completed() { "completed" }
        else if matches!(&failure.error, NominalBodyCompileError::BodyType(_)) { "failed" }
        else { "not-performed" };
    let span = failure.span.map(SourceSpan::from);
    let (stage, error) = match failure.error {
        NominalBodyCompileError::Parse(error) => ("parse", super::draft_diagnostics::source(error)),
        NominalBodyCompileError::Preparation(error) => ("preparation", error.owner_wire()),
        NominalBodyCompileError::BodyType(error) => ("body-type", body_error(error)),
        NominalBodyCompileError::OwnerAdmission(error) => ("owner-admission", error.owner_wire()),
        NominalBodyCompileError::Genesis(error) => ("genesis", error.owner_wire()),
    };
    diagnostic(stage, error, span, status)
}

/// Compile the selected explicit nominal body-typed profile. The existing config
/// decoder supplies the SAME caller layout/domain/parser budget to the unchanged
/// wrapper; that wrapper owns the all-body and actual owner/genesis sequence.
/// No guard, source rewrite, evaluator step, defaults or native dispatch occur.
pub fn compile(source: &str, config_source: &str, target_source: &str)
    -> Result<Value, CompileFailure>
{
    let config_json = crate::json::parse(config_source).map_err(|e| CompileFailure::core("config", e))?;
    let config: NominalConfig = serde_json::from_value(config_json)
        .map_err(|e| CompileFailure::input("config", e.to_string()))?;
    let target = crate::json::parse(target_source).map_err(|e| CompileFailure::core("config", e))?;
    let artifact = compile_nominal_source_host_bounded(
        source, config.preprocessing.parser_budget(), config.owner(), None,
    ).map_err(program_failure)?;
    let report = artifact.report()
        .map_err(|e| diagnostic("observation", e.owner_wire(), None, "completed"))?;
    Ok(json!({
        "format": ARTIFACT_FORMAT,
        "qualification": "HostOnly nominal body typing and owner preparation; no constraints, native export or target qualification",
        "source_profile": report.source_profile,
        "source": {"language": NOMINAL_LANGUAGE, "selection": {"kind": "entry"},
            "name": artifact.name(),
            "program_name_span": SourceSpan::from(artifact.provenance().program_name_span),
            "source_digest": crate::intent::hash_bytes(source.as_bytes()),
            "semantics": null, "semantics_status": "not-separately-qualified"},
        "execution_profile": report.owner.execution_profile,
        "body_typechecking": {"status": report.body_check.status, "judgments": report.body_check.judgments},
        "semantic_correspondence": "unproved", "evaluation_performed": report.owner.evaluation_performed,
        "config": {"fields": config, "file_digest": crate::intent::hash_bytes(config_source.as_bytes())},
        "target": {"descriptor": target, "file_digest": crate::intent::hash_bytes(target_source.as_bytes())},
        "admission": wire::admission(&report.owner.owner),
        "preparation": report.owner.preparation.owner_wire(), "plans": report.owner.plans,
        "evidence": {"source_preparation": "performed", "host_observation": "admitted-tables-only",
            "native": "unrun", "target": "unrun", "acceptance": "unestablished"},
    }))
}
