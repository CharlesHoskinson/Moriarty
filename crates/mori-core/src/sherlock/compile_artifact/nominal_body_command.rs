//! Explicit body-typed nominal command over the actual public owner wrapper.
//! Structural nominal compilation remains its separate unchanged command.
use serde::Serialize;
use serde_json::{json, Value};
use super::draft_simulation_report::ExecutionReport;
use super::nominal_body_compile::NominalBodyCompileArtifact;
use crate::sherlock::helper_machine::core_action_consumer::{CompletedRun, ConsumerError, EventForwarding};
use crate::sherlock::helper_machine::resource_report::ResourceReport;
use crate::sherlock::helper_machine::shared::{OwnerEngine, StepEventC};
use crate::sherlock::helper_machine::shared::owner_host::host_owner_engine;
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
    let (artifact, config, target) = prepare(source, config_source, target_source)?;
    project_preparation(&artifact, config, target, source, config_source, target_source)
}

fn prepare(source: &str, config_source: &str, target_source: &str)
    -> Result<(NominalBodyCompileArtifact, NominalConfig, Value), CompileFailure>
{
    let config_json = crate::json::parse(config_source).map_err(|e| CompileFailure::core("config", e))?;
    let config: NominalConfig = serde_json::from_value(config_json)
        .map_err(|e| CompileFailure::input("config", e.to_string()))?;
    let target = crate::json::parse(target_source).map_err(|e| CompileFailure::core("config", e))?;
    let artifact = compile_nominal_source_host_bounded(
        source, config.preprocessing.parser_budget(), config.owner(), None,
    ).map_err(program_failure)?;
    Ok((artifact, config, target))
}

fn project_preparation(
    artifact: &NominalBodyCompileArtifact, config: NominalConfig, target: Value,
    source: &str, config_source: &str, target_source: &str,
) -> Result<Value, CompileFailure> {
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


/// Output-only complete preparation and actual owner execution. Private fields
/// prevent an unchecked public constructor; this is not an import/run codec.
#[derive(Serialize)]
pub struct NominalHostObservation {
    format: &'static str,
    qualification: &'static str,
    preparation: Value,
    execution: ExecutionReport,
}

impl NominalHostObservation {
    /// A caller's later serialization/publication error retains actual runtime
    /// completion/count plus completed body checking. No shared report is changed.
    pub fn failure(&self, stage: &'static str, message: impl Into<String>) -> CompileFailure {
        CompileFailure {
            stage,
            actual_error: json!({"message": message.into(),
                "source_profile": NOMINAL_BODY_TYPED_PROFILE,
                "body_typechecking": {"status": "completed", "judgments": ADOPTED_NOMINAL_JUDGMENTS}}),
            evaluation_performed: self.execution.evaluation_performed,
            events_forwarded: self.execution.events_forwarded.clone(),
            completed: self.execution.completed,
        }
    }
}

// Preserve the full existing consumer payload/flags and add only separately
// completed source typing. Runtime errors have no source-local span.
fn runtime_failure(mut failure: CompileFailure) -> CompileFailure {
    failure.actual_error = json!({"source_profile": NOMINAL_BODY_TYPED_PROFILE,
        "error": failure.actual_error, "span": null,
        "body_typechecking": {"status": "completed", "judgments": ADOPTED_NOMINAL_JUDGMENTS}});
    failure
}

/// General closed nominal source execution through the same actual checked pair.
///
/// Preparation/typing/admission/genesis and the complete compile-only projection
/// happen once before moving that exact pair to the existing owner. No reparse,
/// second admission, owner guard/default change, literal replacement or event
/// vector. A returned Rejected/Exhausted summary is a real completed owner run.
/// H1, serialization allocation, correspondence/native/target evidence remain
/// separate; the target descriptor invokes no backend. Public use is permissionless.
pub fn observe(source: &str, config_source: &str, target_source: &str)
    -> Result<NominalHostObservation, CompileFailure>
{
    let (artifact, config, target) = prepare(source, config_source, target_source)?;
    let preparation = project_preparation(&artifact, config, target, source, config_source, target_source)?;
    // The projection owns its JSON values, not a cloned admitted pair. Its report
    // borrow has ended before consuming these existing ownership-transfer APIs.
    let (_, _, owner) = artifact.into_parts();
    let (admitted, mut image, _) = owner.into_parts();
    let mut events = EventForwarding::default();
    let result = {
        let mut sink = |_event: &StepEventC| { events.note(); };
        host_owner_engine().run(&admitted, &mut image, &mut sink)
    };
    let summary = result.map_err(|violation| runtime_failure(CompileFailure::consumer(
        ConsumerError::Run { violation, events_forwarded: events.forwarded },
    )))?;
    let completed = CompletedRun { admitted, image, summary, events_forwarded: events.forwarded };
    let actual = completed.observe().map_err(|error| runtime_failure(CompileFailure::consumer(error)))?;
    let execution = ExecutionReport {
        evaluation_performed: true,
        completed: true,
        summary: completed.summary.owner_wire(),
        events_forwarded: completed.events_forwarded.to_string(),
        event_capture: "not-retained",
        image: wire::image(&actual.image),
        resource_report: ResourceReport::from_owner(&actual, &completed.summary),
    };
    Ok(NominalHostObservation {
        format: "sherlock-nominal-host-observation/1",
        qualification: "HostOnly nominal body-typed owner execution; no proof/native/target qualification",
        preparation,
        execution,
    })
}
