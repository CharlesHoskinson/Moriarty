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
    compile_nominal_source_host_bounded, compile_nominal_source_host_bounded_with_nominal_inputs,
    NominalBindingRequest, NominalBodyCompileError,
    NominalBodyCompileFailure, ADOPTED_NOMINAL_JUDGMENTS, NOMINAL_BODY_TYPED_PROFILE,
};
use crate::sherlock::nominal_external_inputs::{NominalBindingProvenance, NominalInputError};
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
fn nominal_input_error(error: NominalInputError) -> Value {
    use NominalInputError as E;
    match error {
        E::LimitsInvalid { field } => json!({"NominalInput": {"variant": "LimitsInvalid", "field": field}}),
        E::CountOverflow => json!({"NominalInput": {"variant": "CountOverflow"}}),
        E::InputDepthExceeded { depth, limit } => json!({"NominalInput": {"variant": "InputDepthExceeded", "depth": depth, "limit": limit}}),
        E::CombinedAstBudgetExceeded { existing, synthesized, limit } => json!({"NominalInput": {"variant": "CombinedAstBudgetExceeded", "existing": existing, "synthesized": synthesized, "limit": limit}}),
        E::StructuralDepthExceeded { depth, limit } => json!({"NominalInput": {"variant": "StructuralDepthExceeded", "depth": depth, "limit": limit}}),
        E::MissingJsonField { object, field } => json!({"NominalInput": {"variant": "MissingJsonField", "object": object, "field": field}}),
        E::ExtraJsonField { object, field } => json!({"NominalInput": {"variant": "ExtraJsonField", "object": object, "field": field}}),
        E::JsonTypeMismatch { object, field, expected } => json!({"NominalInput": {"variant": "JsonTypeMismatch", "object": object, "field": field, "expected": expected}}),
        E::FormatMismatch { actual } => json!({"NominalInput": {"variant": "FormatMismatch", "actual": actual}}),
        E::UnsupportedValueKind { kind } => json!({"NominalInput": {"variant": "UnsupportedValueKind", "kind": kind}}),
        E::UnsupportedTypeKind { kind } => json!({"NominalInput": {"variant": "UnsupportedTypeKind", "kind": kind}}),
        E::EmptyName { context } => json!({"NominalInput": {"variant": "EmptyName", "context": context}}),
        E::InvalidUInt { argument_index, path, text } => json!({"NominalInput": {"variant": "InvalidUInt", "argument_index": argument_index, "path": path, "text": text}}),
        E::UIntOutOfRange { argument_index, path, text } => json!({"NominalInput": {"variant": "UIntOutOfRange", "argument_index": argument_index, "path": path, "text": text}}),
        E::UnknownFunction { name } => json!({"NominalInput": {"variant": "UnknownFunction", "name": name}}),
        E::AmbiguousFunction { name, count } => json!({"NominalInput": {"variant": "AmbiguousFunction", "name": name, "count": count}}),
        E::DuplicateArgument { name, index } => json!({"NominalInput": {"variant": "DuplicateArgument", "name": name, "index": index}}),
        E::ArgumentNameMismatch { index, expected, actual } => json!({"NominalInput": {"variant": "ArgumentNameMismatch", "index": index, "expected": expected, "actual": actual}}),
        E::MissingArgument { index, name } => json!({"NominalInput": {"variant": "MissingArgument", "index": index, "name": name}}),
        E::ExtraArgument { index, name } => json!({"NominalInput": {"variant": "ExtraArgument", "index": index, "name": name}}),
        E::UnknownType { name } => json!({"NominalInput": {"variant": "UnknownType", "name": name}}),
        E::AmbiguousType { name, count } => json!({"NominalInput": {"variant": "AmbiguousType", "name": name, "count": count}}),
        E::UnknownConstructor { type_name, ctor } => json!({"NominalInput": {"variant": "UnknownConstructor", "type_name": type_name, "ctor": ctor}}),
        E::AmbiguousConstructor { type_name, ctor, count } => json!({"NominalInput": {"variant": "AmbiguousConstructor", "type_name": type_name, "ctor": ctor, "count": count}}),
        E::TypeArityMismatch { type_name, expected, actual } => json!({"NominalInput": {"variant": "TypeArityMismatch", "type_name": type_name, "expected": expected, "actual": actual}}),
        E::UnsupportedExpectedType { argument_index, path, expected } => json!({"NominalInput": {"variant": "UnsupportedExpectedType", "argument_index": argument_index, "path": path, "expected": expected}}),
        E::ValueKindMismatch { argument_index, path, expected, actual } => json!({"NominalInput": {"variant": "ValueKindMismatch", "argument_index": argument_index, "path": path, "expected": expected, "actual": actual}}),
        E::TypeMismatch { argument_index, path, expected, actual } => json!({"NominalInput": {"variant": "TypeMismatch", "argument_index": argument_index, "path": path, "expected": expected, "actual": actual}}),
        E::MissingField { type_name, ctor, field_index, field_name } => json!({"NominalInput": {"variant": "MissingField", "type_name": type_name, "ctor": ctor, "field_index": field_index, "field_name": field_name}}),
        E::ExtraField { type_name, ctor, field_index, actual_name } => json!({"NominalInput": {"variant": "ExtraField", "type_name": type_name, "ctor": ctor, "field_index": field_index, "actual_name": actual_name}}),
        E::FieldNameMismatch { type_name, ctor, field_index, expected, actual } => json!({"NominalInput": {"variant": "FieldNameMismatch", "type_name": type_name, "ctor": ctor, "field_index": field_index, "expected": expected, "actual": actual}}),
    }
}
fn nominal_binding_value(binding: &NominalBindingProvenance, nominal_source: Option<&str>) -> Value {
    json!({
        "format": binding.format,
        "function_index": binding.function_index,
        "function_name": binding.function_name,
        "input_digest": nominal_source.map(|text| crate::intent::hash_bytes(text.as_bytes())),
        "arguments": binding.arguments.iter().map(|argument| json!({
            "index": argument.index,
            "name": argument.name,
            "declared_type": argument.declared_type,
            "value_kind": argument.value_kind,
            "bound_value": argument.bound_value,
            "value_nodes": argument.value_nodes,
            "type_nodes": argument.type_nodes,
        })).collect::<Vec<Value>>(),
        "synthesized_expr_nodes": binding.synthesized_expr_nodes,
        "synthesized_value_nodes": binding.synthesized_value_nodes,
        "synthesized_type_nodes": binding.synthesized_type_nodes,
        "synthesized_structural_depth": binding.synthesized_structural_depth,
        "combined_ast_occurrences": binding.combined_ast_occurrences,
        "entry_provenance": {
            "synthesized_entry_span": null,
            "retained_entry_provenance": "stale-original-entry-source-only",
            "source_spans_repurposed": false,
        },
    })
}
fn program_failure(failure: NominalBodyCompileFailure) -> CompileFailure {
    let status = if failure.body_check_completed() { "completed" }
        else if matches!(&failure.error, NominalBodyCompileError::BodyType(_)) { "failed" }
        else { "not-performed" };
    let span = failure.span.map(SourceSpan::from);
    let (stage, error) = match failure.error {
        NominalBodyCompileError::Parse(error) => ("parse", super::draft_diagnostics::source(error)),
        NominalBodyCompileError::NominalInput(error) => ("nominal-input", nominal_input_error(error)),
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
    project_preparation(&artifact, config, target, source, config_source, target_source, None)
}

/// Compile the selected explicit nominal body-typed profile with nominal
/// external inputs bound to one named function before the first T3.
pub fn compile_with_nominal_inputs(
    source: &str, config_source: &str, target_source: &str,
    nominal_source: &str, nominal_function: &str,
) -> Result<Value, CompileFailure>
{
    let (artifact, config, target) = prepare_nominal(source, config_source, target_source, nominal_source, nominal_function)?;
    project_preparation(&artifact, config, target, source, config_source, target_source, Some(nominal_source))
}

fn prepare(source: &str, config_source: &str, target_source: &str)
    -> Result<(NominalBodyCompileArtifact, NominalConfig, Value), CompileFailure>
{
    prepare_inner(source, config_source, target_source, None)
}

fn prepare_nominal(
    source: &str, config_source: &str, target_source: &str,
    nominal_source: &str, nominal_function: &str,
) -> Result<(NominalBodyCompileArtifact, NominalConfig, Value), CompileFailure>
{
    prepare_inner(source, config_source, target_source, Some((nominal_source, nominal_function)))
}

fn prepare_inner(
    source: &str, config_source: &str, target_source: &str,
    nominal: Option<(&str, &str)>,
) -> Result<(NominalBodyCompileArtifact, NominalConfig, Value), CompileFailure>
{
    let config_json = crate::json::parse(config_source).map_err(|e| CompileFailure::core("config", e))?;
    let config: NominalConfig = serde_json::from_value(config_json)
        .map_err(|e| CompileFailure::input("config", e.to_string()))?;
    let target = crate::json::parse(target_source).map_err(|e| CompileFailure::core("config", e))?;
    let nominal_request = match nominal {
        Some((nominal_source, nominal_function)) => Some(NominalBindingRequest {
            function_name: nominal_function.to_string(),
            input: crate::json::parse(nominal_source)
                .map_err(|e| CompileFailure::core("nominal-input", e))?,
        }),
        None => None,
    };
    let artifact = match nominal_request {
        Some(request) => compile_nominal_source_host_bounded_with_nominal_inputs(
            source, config.preprocessing.parser_budget(), config.owner(), None, request,
        ),
        None => compile_nominal_source_host_bounded(
            source, config.preprocessing.parser_budget(), config.owner(), None,
        ),
    }.map_err(program_failure)?;
    Ok((artifact, config, target))
}

fn project_preparation(
    artifact: &NominalBodyCompileArtifact, config: NominalConfig, target: Value,
    source: &str, config_source: &str, target_source: &str, nominal_source: Option<&str>,
) -> Result<Value, CompileFailure> {
    let report = artifact.report()
        .map_err(|e| diagnostic("observation", e.owner_wire(), None, "completed"))?;
    let mut projected = json!({
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
    });
    if let Some(binding) = artifact.binding() {
        projected["nominal_binding"] = nominal_binding_value(binding, nominal_source);
    }
    Ok(projected)
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
    let preparation = project_preparation(&artifact, config, target, source, config_source, target_source, None)?;
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

/// General nominal source execution with external inputs bound to one named
/// function through the same actual checked pair as observe.
pub fn observe_with_nominal_inputs(
    source: &str, config_source: &str, target_source: &str,
    nominal_source: &str, nominal_function: &str,
) -> Result<NominalHostObservation, CompileFailure>
{
    let (artifact, config, target) = prepare_nominal(source, config_source, target_source, nominal_source, nominal_function)?;
    let preparation = project_preparation(&artifact, config, target, source, config_source, target_source, Some(nominal_source))?;
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

/// Typed nominal compiler-to-owner handoff with the complete unstepped source
/// preparation and the actual post-run admitted/image/summary pair. Events are
/// forwarded to the caller, not retained here. This is HostOnly execution, not
/// an importer, proof, native export, or a process-wide H1 guarantee.
///
/// Private fields preserve the producing sequence. Borrow the pair through
/// `completed`/`observe`, or transfer it through `into_parts` to an existing
/// typed consumer. No program/image is reconstructed from the JSON snapshot.
pub struct NominalHostRun {
    preparation: Value,
    completed: CompletedRun,
}

impl NominalHostRun {
    pub fn preparation(&self) -> &Value { &self.preparation }
    pub fn completed(&self) -> &CompletedRun { &self.completed }

    /// Borrow every actual owner table/image field with the existing AB check.
    /// The original ConsumerError and completed summary/count are preserved.
    pub fn observe(&self) -> Result<
        crate::sherlock::helper_machine::shared::owner_host::HostObservation<'_>, ConsumerError,
    > {
        self.completed.observe()
    }

    /// Transfer the same preparation and completed pair, without a new parse,
    /// admission, genesis, run, or JSON-to-program conversion.
    pub fn into_parts(self) -> (Value, CompletedRun) {
        (self.preparation, self.completed)
    }

    /// Consume the typed handoff into the existing complete observation wire.
    /// The caller's sink may have retained events; this object itself has not.
    /// No additional owner step is performed and no wire format is introduced.
    pub fn into_observation(self) -> Result<NominalHostObservation, CompileFailure> {
        let actual = self.completed.observe()
            .map_err(|error| runtime_failure(CompileFailure::consumer(error)))?;
        let execution = ExecutionReport {
            evaluation_performed: true,
            completed: true,
            summary: self.completed.summary.owner_wire(),
            events_forwarded: self.completed.events_forwarded.to_string(),
            event_capture: "not-retained",
            image: wire::image(&actual.image),
            resource_report: ResourceReport::from_owner(&actual, &self.completed.summary),
        };
        Ok(NominalHostObservation {
            format: "sherlock-nominal-host-observation/1",
            qualification: "HostOnly nominal body-typed owner execution; no proof/native/target qualification",
            preparation: self.preparation,
            execution,
        })
    }
}

/// Prepare the real body-typed closed nominal source once, then run its same
/// admitted pair with every full StepEventC forwarded once in owner order.
///
/// Declarations, recursive functions, generic type instances and match plans
/// use the existing bounded compiler, owner and capacities. No callback can
/// select a guard/plan or alter that pair. Sink allocation/retention is caller
/// owned; sink panics are not caught, as in run_draft_host/run_scalar_action.
/// A returned Rejected/Exhausted summary remains a completed real owner run.
pub fn run(
    source: &str, config_source: &str, target_source: &str,
    sink: &mut dyn FnMut(&StepEventC),
) -> Result<NominalHostRun, CompileFailure> {
    let (artifact, config, target) = prepare(source, config_source, target_source)?;
    let preparation = project_preparation(
        &artifact, config, target, source, config_source, target_source, None,
    )?;
    run_prepared_nominal(artifact, preparation, sink)
}

/// The same typed/event handoff with the existing nominal input binding before
/// first T3/body checking. Full binding metadata and original-source provenance
/// remain in preparation. The arguments are visible program constants, not
/// private witnesses or a fixed-circuit input path.
pub fn run_with_nominal_inputs(
    source: &str, config_source: &str, target_source: &str,
    nominal_source: &str, nominal_function: &str,
    sink: &mut dyn FnMut(&StepEventC),
) -> Result<NominalHostRun, CompileFailure> {
    let (artifact, config, target) = prepare_nominal(
        source, config_source, target_source, nominal_source, nominal_function,
    )?;
    let preparation = project_preparation(
        &artifact, config, target, source, config_source, target_source, Some(nominal_source),
    )?;
    run_prepared_nominal(artifact, preparation, sink)
}

fn run_prepared_nominal(
    artifact: NominalBodyCompileArtifact, preparation: Value,
    sink: &mut dyn FnMut(&StepEventC),
) -> Result<NominalHostRun, CompileFailure> {
    let (_, _, owner) = artifact.into_parts();
    let (admitted, mut image, _) = owner.into_parts();
    let mut events = EventForwarding::default();
    let result = {
        let mut forwarding_sink = |event: &StepEventC| {
            events.note();
            sink(event);
        };
        host_owner_engine().run(&admitted, &mut image, &mut forwarding_sink)
    };
    let summary = result.map_err(|violation| runtime_failure(CompileFailure::consumer(
        ConsumerError::Run { violation, events_forwarded: events.forwarded },
    )))?;
    Ok(NominalHostRun {
        preparation,
        completed: CompletedRun { admitted, image, summary, events_forwarded: events.forwarded },
    })
}
