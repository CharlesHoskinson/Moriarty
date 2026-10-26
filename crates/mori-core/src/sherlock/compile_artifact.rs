//! Selected Core scalar or explicit dedicated-draft HostOnly B preparation.
//! Source candidate only: no constraint plan, native export or target execution.
//! Source routes use the existing B owner. Core scalar and body-typed nominal
//! support explicit observation; dedicated draft compilation remains compile-only.
use crate::sherlock::helper_machine::core_action_consumer::{
    ConsumerError, preflight_action_inputs, preflight_source_global, prepare_scalar_action_host,
    validate_config,
};
use crate::sherlock_source::ScalarActionReport;
use serde::Serialize;
use serde_json::{Value, json};
use std::collections::BTreeMap;

#[path = "compile_artifact/config.rs"]
pub mod config;
#[path = "compile_artifact/draft_chunked.rs"]
pub mod draft_chunked;
#[path = "compile_artifact/draft_contract.rs"]
pub mod draft_contract;
#[path = "compile_artifact/draft_execution.rs"]
pub mod draft_execution;
#[path = "compile_artifact/draft_optimization.rs"]
pub mod draft_optimization;
#[path = "compile_artifact/draft_profile.rs"]
pub mod draft_profile;
#[path = "compile_artifact/draft_reference_slices.rs"]
pub mod draft_reference_slices;
#[path = "compile_artifact/draft_report.rs"]
pub mod draft_report;
#[path = "compile_artifact/draft_witness.rs"]
pub mod draft_witness;
#[path = "compile_artifact/native_inventory.rs"]
pub mod native_inventory;
#[path = "compile_artifact/recursive_source.rs"]
pub(crate) mod recursive_source;
#[path = "compile_artifact/wire.rs"]
mod wire;
// Retained checked source and actual B genesis for explicit reference consumers.
#[path = "compile_artifact/draft_reference.rs"]
pub mod draft_reference;
// Public typed ExtProgram compiler using the actual checked owner admission ABI.
#[path = "compile_artifact/ext_compile.rs"]
pub mod ext_compile;
#[path = "compile_artifact/nominal_command.rs"]
pub mod nominal_command;
// Selected all-body nominal profile; raw nominal command remains structural.
#[path = "compile_artifact/draft_diagnostics.rs"]
mod draft_diagnostics;
#[path = "compile_artifact/draft_simulation_report.rs"]
pub mod draft_simulation_report;
#[path = "compile_artifact/nominal_body_command.rs"]
pub mod nominal_body_command;
#[path = "compile_artifact/nominal_body_compile.rs"]
pub mod nominal_body_compile;
use config::ExecutionConfig;
use wire::OwnerWire;

#[path = "compile_artifact/complete_trace_wire.rs"]
mod complete_trace_wire;
#[path = "compile_artifact/core_observation.rs"]
mod core_observation;

pub use complete_trace_wire::{
    TRACE_FORMAT, TRACE_SERIALIZATION_BUDGET, reconstruct_step_host_snapshot,
};

pub const EXECUTION_PROFILE: &str = "sherlock-helper-microstep/1";

/// Output-only JSON trees below come from typed exhaustive owner projections.
/// They are not accepted back as an executable image or as admission authority.
#[derive(Serialize)]
pub struct CompileArtifact {
    pub format: &'static str,
    pub qualification: &'static str,
    pub source: SourceIdentity,
    pub execution_profile: &'static str,
    pub input_policy: InputPolicy,
    pub config: ConfigIdentity,
    pub target: TargetDescriptor,
    pub preparation: ScalarActionReport,
    pub admission: Value,
    pub plans: Plans,
    pub observation: Observation,
    pub evidence: Evidence,
}
#[derive(Serialize)]
#[serde(tag = "status", rename_all = "kebab-case")]
pub enum Observation {
    NotRequested {
        evaluation_performed: bool,
    },
    Observed {
        evaluation_performed: bool,
        summary: Value,
        events_forwarded: String,
        image: Value,
        resource_report: crate::sherlock::helper_machine::resource_report::ResourceReport,
        /// Complete owner records under `TRACE_FORMAT` (`"sherlock-core1-complete-trace/2"`)
        /// bounded per-projection by `admission.bounds.serialization_bytes` and across
        /// the trace by `derive_trace_serialization_budget` / `crate::json::parse_limited`.
        complete_trace: Value,
    },
}
impl CompileArtifact {
    /// Preserve the real completed-run metadata on later serialization/publication failure.
    pub fn failure(&self, stage: &'static str, message: impl Into<String>) -> CompileFailure {
        let (evaluation_performed, events_forwarded, completed) = match &self.observation {
            Observation::NotRequested { .. } => (false, "0".to_owned(), false),
            Observation::Observed {
                events_forwarded, ..
            } => (true, events_forwarded.clone(), true),
        };
        CompileFailure {
            stage,
            actual_error: json!({"message":message.into()}),
            evaluation_performed,
            events_forwarded,
            completed,
        }
    }
    /// Preserve the complete observed trace for consumers of the richer manifest
    /// error envelope. Ordinary CLI publication errors retain `failure`'s shape.
    pub fn failure_with_trace(
        &self,
        stage: &'static str,
        message: impl Into<String>,
    ) -> CompileFailure {
        let mut failure = self.failure(stage, message);
        if let Observation::Observed { complete_trace, .. } = &self.observation {
            failure.actual_error["complete_trace"] = complete_trace.clone();
        }
        failure
    }
    /// Serialize the completed artifact manifest, reporting any serializer error
    /// at the `"serialization"` stage while preserving completed-run metadata
    /// and the complete observed trace.
    pub fn manifest(&self) -> Result<String, CompileFailure> {
        serde_json::to_string_pretty(self)
            .map_err(|error| self.failure_with_trace("serialization", error.to_string()))
    }
}

#[derive(Serialize)]
pub struct SourceIdentity {
    pub language: String,
    pub semantics: String,
    pub agreement: String,
    pub action: String,
    pub action_index: String,
    pub source_digest: String,
}
#[derive(Serialize)]
pub struct InputPolicy {
    pub kind: &'static str,
    pub declared: Vec<String>,
    pub supplied: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding: Option<InputBinding>,
}
/// Visible concrete entry arguments and the digest of the original input text.
/// Values bind an input-specific admitted program, not private witnesses.
#[derive(Serialize)]
pub struct InputBinding {
    pub values: BTreeMap<String, crate::Value>,
    pub file_digest: String,
}
#[derive(Serialize)]
pub struct ConfigIdentity {
    #[serde(flatten)]
    pub fields: ExecutionConfig,
    pub file_digest: String,
}
#[derive(Serialize)]
pub struct TargetDescriptor {
    pub descriptor: Value,
    pub file_digest: String,
}
#[derive(Serialize)]
pub struct Plan {
    pub status: &'static str,
    pub artifact: Option<()>,
    pub reason: &'static str,
}
#[derive(Serialize)]
pub struct Plans {
    pub witness: Plan,
    pub constraint: Plan,
    pub export: Plan,
}
#[derive(Serialize)]
pub struct Evidence {
    pub source_preparation: &'static str,
    pub host_observation: &'static str,
    pub native: &'static str,
    pub target: &'static str,
    pub acceptance: &'static str,
}

/// Structured original diagnostic payloads, never a manufactured owner outcome.
#[derive(Serialize)]
pub struct CompileFailure {
    pub stage: &'static str,
    pub actual_error: Value,
    pub evaluation_performed: bool,
    pub events_forwarded: String,
    pub completed: bool,
}
impl CompileFailure {
    pub fn input(stage: &'static str, error: impl Into<String>) -> Self {
        Self {
            stage,
            actual_error: json!({"message":error.into()}),
            evaluation_performed: false,
            events_forwarded: "0".into(),
            completed: false,
        }
    }
    /// Export this failure through `crate::json::export`. Normal normalized traces
    /// pass `crate::json::export(self)` directly; if a caller constructs an oversized
    /// diagnostic exceeding the 1 MiB / 65,536-node / 32-depth exporter bounds,
    /// preserve `stage`, `evaluation_performed`, `events_forwarded`, `completed`,
    /// all non-trace error metadata (`message`, `failure_boundary`,
    /// `serialization_error`, `violation`), and the largest fitting prefix of
    /// `complete_trace`.
    pub fn export_diagnostic(&self) -> Result<String, crate::Error> {
        let first_error = match crate::json::export(self) {
            Ok(diagnostic) => return Ok(diagnostic),
            Err(error) => error,
        };
        if let Some(trace_obj) = self
            .actual_error
            .get("complete_trace")
            .and_then(Value::as_object)
        {
            let records = trace_obj
                .get("records")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();
            let build_candidate = |keep: usize, include_snapshots: bool| -> Self {
                let mut err_map = self.actual_error.as_object().cloned().unwrap_or_default();
                err_map.insert("diagnostic_export_error".into(), json!(&first_error));
                let mut trimmed = serde_json::Map::new();
                if let Some(fmt) = trace_obj.get("format") {
                    trimmed.insert("format".into(), fmt.clone());
                }
                if include_snapshots {
                    if let Some(init) = trace_obj.get("initial") {
                        trimmed.insert("initial".into(), init.clone());
                    }
                } else {
                    trimmed.insert("initial".into(), Value::Null);
                }
                if let Some(qual) = trace_obj.get("qualification") {
                    trimmed.insert("qualification".into(), qual.clone());
                }
                trimmed.insert(
                    "records".into(),
                    Value::Array(records.iter().take(keep).cloned().collect()),
                );
                trimmed.insert("retained_records".into(), Value::String(keep.to_string()));
                if include_snapshots {
                    if let Some(ret) = trace_obj.get("returned") {
                        trimmed.insert("returned".into(), ret.clone());
                    }
                }
                if let Some(budget) = trace_obj.get("serialization_budget") {
                    trimmed.insert("serialization_budget".into(), budget.clone());
                }
                err_map.insert("complete_trace".into(), Value::Object(trimmed));
                Self {
                    stage: self.stage,
                    actual_error: Value::Object(err_map),
                    evaluation_performed: self.evaluation_performed,
                    events_forwarded: self.events_forwarded.clone(),
                    completed: self.completed,
                }
            };
            if let Ok(zero_export) = crate::json::export(&build_candidate(0, true)) {
                let mut low = 0usize;
                let mut high = records.len();
                let mut best = zero_export;
                while low < high {
                    let mid = low + (high - low + 1) / 2;
                    if let Ok(candidate_export) = crate::json::export(&build_candidate(mid, true)) {
                        low = mid;
                        best = candidate_export;
                    } else {
                        high = mid - 1;
                    }
                }
                return Ok(best);
            }
            if let Ok(meta_export) = crate::json::export(&build_candidate(0, false)) {
                return Ok(meta_export);
            }
        }
        let mut fallback_map = self.actual_error.as_object().cloned().unwrap_or_default();
        fallback_map.insert("diagnostic_export_error".into(), json!(&first_error));
        if fallback_map.contains_key("complete_trace") {
            fallback_map.insert("complete_trace".into(), Value::Null);
        }
        let without_trace = Self {
            stage: self.stage,
            actual_error: Value::Object(fallback_map.clone()),
            evaluation_performed: self.evaluation_performed,
            events_forwarded: self.events_forwarded.clone(),
            completed: self.completed,
        };
        if let Ok(exported) = crate::json::export(&without_trace) {
            return Ok(exported);
        }
        if let Some(fb) = fallback_map
            .get_mut("failure_boundary")
            .and_then(Value::as_object_mut)
        {
            fb.remove("before");
            fb.remove("after");
        }
        let compact_boundary = Self {
            stage: self.stage,
            actual_error: Value::Object(fallback_map),
            evaluation_performed: self.evaluation_performed,
            events_forwarded: self.events_forwarded.clone(),
            completed: self.completed,
        };
        crate::json::export(&compact_boundary)
    }
    fn core(stage: &'static str, error: crate::Error) -> Self {
        Self {
            stage,
            actual_error: json!(error),
            evaluation_performed: false,
            events_forwarded: "0".into(),
            completed: false,
        }
    }
    /// Preserve the bounded draft preparation stage and its complete typed error.
    fn draft_source(error: recursive_source::DraftSourceFailure) -> Self {
        use recursive_source::DraftSourceFailure;
        let (stage, actual_error) = match error {
            DraftSourceFailure::Source(error) => ("parse", draft_diagnostics::source(error)),
            DraftSourceFailure::Types(error) => ("typecheck", draft_diagnostics::types(error)),
            DraftSourceFailure::Copy(error) => ("copy", draft_diagnostics::copy(error)),
            DraftSourceFailure::Admit(error) => ("admit", error.owner_wire()),
            DraftSourceFailure::AdmitRun(error) => ("admit-run", error.owner_wire()),
        };
        Self {
            stage,
            actual_error,
            evaluation_performed: false,
            events_forwarded: "0".into(),
            completed: false,
        }
    }
    fn consumer(error: ConsumerError) -> Self {
        let (stage, actual_error, evaluation_performed, events_forwarded, completed) = match error {
            ConsumerError::HostGuard(error) => ("host-guard", error.owner_wire(), false, 0, false),
            ConsumerError::CoreValidation(error) => ("parse", json!(error), false, 0, false),
            ConsumerError::SourceProjection(error) => {
                ("source-projection", json!(error), false, 0, false)
            }
            ConsumerError::Admit(error) => ("admit", error.owner_wire(), false, 0, false),
            ConsumerError::AdmitRun(error) => ("admit-run", error.owner_wire(), false, 0, false),
            ConsumerError::Run {
                violation,
                events_forwarded,
            } => ("run", violation.owner_wire(), true, events_forwarded, false),
            ConsumerError::Observation {
                violation,
                summary,
                events_forwarded,
            } => (
                "observation",
                json!({"violation":violation.owner_wire(),
                    "summary":summary.owner_wire()}),
                true,
                events_forwarded,
                true,
            ),
        };
        Self {
            stage,
            actual_error,
            evaluation_performed,
            events_forwarded: events_forwarded.to_string(),
            completed,
        }
    }
}

pub fn parse_config(source: &str) -> Result<ExecutionConfig, CompileFailure> {
    let value =
        crate::json::parse(source).map_err(|error| CompileFailure::core("config", error))?;
    let config: ExecutionConfig = serde_json::from_value(value)
        .map_err(|error| CompileFailure::input("config", error.to_string()))?;
    // Wire-only shape constraint. Existing host admission decides numerical policy.
    config
        .consumer()
        .map_err(|error| CompileFailure::input("config", error))?;
    Ok(config)
}

/// Decode only the distinct draft wire shape. The designated-owner constructor
/// decides the supplied preprocessing/owner configuration policy before work.
pub fn parse_draft_config(
    source: &str,
) -> Result<draft_contract::DraftExecutionConfig, CompileFailure> {
    let value =
        crate::json::parse(source).map_err(|error| CompileFailure::core("config", error))?;
    serde_json::from_value(value)
        .map_err(|error| CompileFailure::input("config", error.to_string()))
}

/// SDK-free product entry point. No defaults, scenarios, fixture-name selection,
/// Core evaluation, constraint generation or native/backend dispatch occur here.
/// `observe` preserves a distinct caller execution dependency; false never runs.
pub fn compile_host(
    source: &str,
    action_name: &str,
    config_source: &str,
    target_source: &str,
    observe: bool,
) -> Result<CompileArtifact, CompileFailure> {
    compile_host_mode(
        source,
        action_name,
        config_source,
        target_source,
        InputMode::LegacyEmpty,
        observe,
    )
}

/// Bind an explicit typed map through the existing scalar prepare/run consumer.
/// Uses the same strict JSON and Value/UInt decoding as other Core inputs.
/// Typed values remain visible; no Scenario state, private witness, new guard,
/// native exporter or fixed program identity is introduced by this route.
pub fn compile_host_with_inputs(
    source: &str,
    action_name: &str,
    config_source: &str,
    target_source: &str,
    inputs_source: &str,
    observe: bool,
) -> Result<CompileArtifact, CompileFailure> {
    compile_host_mode(
        source,
        action_name,
        config_source,
        target_source,
        InputMode::ExplicitTypedMap(inputs_source),
        observe,
    )
}

enum InputMode<'a> {
    LegacyEmpty,
    ExplicitTypedMap(&'a str),
}

fn compile_host_mode(
    source: &str,
    action_name: &str,
    config_source: &str,
    target_source: &str,
    mode: InputMode<'_>,
    observe: bool,
) -> Result<CompileArtifact, CompileFailure> {
    let config = parse_config(config_source)?;
    let consumer = config
        .consumer()
        .map_err(|error| CompileFailure::input("config", error))?;
    let target_descriptor =
        crate::json::parse(target_source).map_err(|error| CompileFailure::core("config", error))?;
    validate_config(&consumer)
        .map_err(|error| CompileFailure::consumer(ConsumerError::HostGuard(error)))?;
    // New mode only: config/target/config policy retain precedence. Decode the
    // entire map before source parsing; original codec failures keep their text.
    let (inputs, binding_digest) = match mode {
        InputMode::LegacyEmpty => (BTreeMap::<String, crate::Value>::new(), None),
        InputMode::ExplicitTypedMap(text) => {
            let value =
                crate::json::parse(text).map_err(|error| CompileFailure::core("inputs", error))?;
            let inputs: BTreeMap<String, crate::Value> = serde_json::from_value(value)
                .map_err(|error| CompileFailure::input("inputs", error.to_string()))?;
            (inputs, Some(crate::intent::hash_bytes(text.as_bytes())))
        }
    };
    let program = crate::parse(source).map_err(|error| CompileFailure::core("parse", error))?;
    preflight_source_global(&program, &consumer).map_err(CompileFailure::consumer)?;
    let action_index = program
        .actions
        .iter()
        .position(|action| action.name == action_name)
        .ok_or_else(|| CompileFailure::input("select", format!("unknown action: {action_name}")))?;
    let action = preflight_action_inputs(&program, action_index, &inputs, &consumer)
        .map_err(CompileFailure::consumer)?;
    if binding_digest.is_none() && !action.inputs.is_empty() {
        return Err(CompileFailure {
            stage: "select",
            actual_error: json!({"unsupported":"parameterized-action",
                "action":action.name, "action_index":action_index.to_string(),
                "declared_inputs":action.inputs}),
            evaluation_performed: false,
            events_forwarded: "0".into(),
            completed: false,
        });
    }
    let (kind, declared, supplied) = if binding_digest.is_some() {
        (
            "explicit-typed-map",
            action
                .inputs
                .iter()
                .map(|input| input.name.clone())
                .collect(),
            inputs.keys().cloned().collect(),
        )
    } else {
        ("parameter-free", vec![], vec![])
    };
    // Each branch owns exactly one admission/image pair. Observe mode consumes
    // that same shared preparation through record_step; no second admission.
    let (preparation, admission, observation) = if observe {
        let prepared = prepare_scalar_action_host(&program, action_index, &inputs, &consumer)
            .map_err(CompileFailure::consumer)?;
        let (run, complete_trace) = core_observation::run(prepared)?;
        let actual = run.observe().map_err(|error| {
            core_observation::attach_completed_trace(
                CompileFailure::consumer(error),
                &complete_trace,
            )
        })?;
        let admission = wire::admission(&actual);
        let resource = run.resource_report().map_err(|error| {
            core_observation::attach_completed_trace(
                CompileFailure::consumer(error),
                &complete_trace,
            )
        })?;
        let observation = Observation::Observed {
            evaluation_performed: true,
            summary: run.completed.summary.owner_wire(),
            events_forwarded: run.completed.events_forwarded.to_string(),
            image: wire::image(&actual.image),
            resource_report: resource,
            complete_trace,
        };
        (run.preparation, admission, observation)
    } else {
        let prepared = prepare_scalar_action_host(&program, action_index, &inputs, &consumer)
            .map_err(CompileFailure::consumer)?;
        let actual = prepared.observe().map_err(|error| CompileFailure {
            stage: "observation",
            actual_error: error.owner_wire(),
            evaluation_performed: false,
            events_forwarded: "0".into(),
            completed: false,
        })?;
        let admission = wire::admission(&actual);
        (
            prepared.preparation,
            admission,
            Observation::NotRequested {
                evaluation_performed: false,
            },
        )
    };
    Ok(CompileArtifact {
        format: "sherlock-host-artifact/1",
        qualification: "host-only-scalar-value-projection; no effects/constraints/native/target qualification",
        source: SourceIdentity {
            language: program.language,
            semantics: program.semantics,
            agreement: program.name,
            action: action_name.to_owned(),
            action_index: action_index.to_string(),
            source_digest: crate::intent::hash_bytes(source.as_bytes()),
        },
        execution_profile: EXECUTION_PROFILE,
        input_policy: InputPolicy {
            kind,
            declared,
            supplied,
            binding: binding_digest.map(|file_digest| InputBinding {
                values: inputs,
                file_digest,
            }),
        },
        config: ConfigIdentity {
            fields: config.clone(),
            file_digest: crate::intent::hash_bytes(config_source.as_bytes()),
        },
        target: TargetDescriptor {
            descriptor: target_descriptor,
            file_digest: crate::intent::hash_bytes(target_source.as_bytes()),
        },
        preparation,
        admission,
        plans: Plans {
            witness: Plan {
                status: "not-implemented",
                artifact: None,
                reason: "no exported WitnessPlan in this batch",
            },
            constraint: Plan {
                status: "not-implemented",
                artifact: None,
                reason: "no Sherlock constraint emission in this batch",
            },
            export: Plan {
                status: "unsupported",
                artifact: None,
                reason: "HostOnly source route has no native exporter",
            },
        },
        observation,
        evidence: Evidence {
            source_preparation: "prepared-and-admitted",
            host_observation: if observe { "observed" } else { "unrun" },
            native: "unrun",
            target: "unrun",
            acceptance: "none",
        },
    })
}

/// Compile the explicit dedicated-draft source to real admitted HostOnly tables
/// and their unstepped genesis image. Every parser/type/copy extent is supplied
/// by the caller and forwarded to the actual R181/R200/R208 constructor seam.
/// No source evaluation, B step, native export or unbounded fallback occurs.
/// This is preparation only: A/B cleanup, owner/report allocations and H1's
/// process-wide heap/stack/time guarantees remain separate owner premises.
pub fn compile_draft_host(
    source: &str,
    config_source: &str,
    target_source: &str,
) -> Result<draft_report::DraftCompileArtifact, CompileFailure> {
    let (pair, config, target) = prepare_draft_pair(source, config_source, target_source)?;
    project_draft_pair(&pair, source, config, target)
}

// Both public consumers share the same bounded source/admission sequence.
// This helper introduces no alternate admission, budgets, errors or defaults.
fn prepare_draft_pair(
    source: &str,
    config_source: &str,
    target_source: &str,
) -> Result<
    (
        recursive_source::AdmittedDraftSource,
        draft_contract::DraftConfigIdentity,
        TargetDescriptor,
    ),
    CompileFailure,
> {
    let config = parse_draft_config(config_source)?;
    let target_descriptor =
        crate::json::parse(target_source).map_err(|error| CompileFailure::core("config", error))?;
    let budget = recursive_source::DraftPreparationBudget {
        parse: config.preprocessing.parser_budget(),
        max_type_depth: config.preprocessing.max_type_depth.0,
        copy: config.preprocessing.copy_budget(),
    };
    let bounds = config.bounds.owner();
    let layout = config.layout.owner(bounds);
    let fuel = config.fuel.owner();
    let stack = config.stack.owner();
    let checked = recursive_source::prepare_checked_draft_bounded(
        source,
        budget.parse,
        budget.max_type_depth,
    )
    .map_err(CompileFailure::draft_source)?;
    let admitted = recursive_source::admit_checked_draft_bounded(
        checked,
        &budget.copy,
        &bounds,
        &layout,
        &fuel,
        &stack,
    )
    .map_err(CompileFailure::draft_source)?;
    Ok((
        admitted,
        draft_contract::DraftConfigIdentity {
            fields: config,
            file_digest: crate::intent::hash_bytes(config_source.as_bytes()),
        },
        TargetDescriptor {
            descriptor: target_descriptor,
            file_digest: crate::intent::hash_bytes(target_source.as_bytes()),
        },
    ))
}

// Project this actual pair, with identical source digests and error staging.
fn project_draft_pair(
    pair: &recursive_source::AdmittedDraftSource,
    source: &str,
    config: draft_contract::DraftConfigIdentity,
    target: TargetDescriptor,
) -> Result<draft_report::DraftCompileArtifact, CompileFailure> {
    draft_report::project_admitted_preparation(
        pair,
        crate::intent::hash_bytes(source.as_bytes()),
        config,
        target,
    )
    .map_err(|error| CompileFailure {
        stage: "observation",
        actual_error: error.owner_wire(),
        evaluation_performed: false,
        events_forwarded: "0".into(),
        completed: false,
    })
}
