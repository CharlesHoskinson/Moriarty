//! The complete selected Sherlock compile handler, shared with direct consumers.
//! Source is read by the caller before selection, as in the existing public path.
use super::{publish_io, source_io as core_io};
use crate::cli_args::compile::Args;
use std::process::ExitCode;

/// Selected SDK-free host artifact path. Observation is explicit and retains
/// its separate H1/command-closure dependency; flags confer no internal authority.
pub(super) fn run(
    args: &Args,
    source: &str,
    mut status: impl FnMut(&str, String),
) -> Result<ExitCode, mori_diagnostics::Report> {
    use host::draft_contract::{SourceSelection, select_source};
    use mori_core::sherlock_compile_artifact::{self as host, CompileFailure};
    let execute = || -> Result<(SelectedSherlockArtifact, String), CompileFailure> {
        let profile = args
            .execution_profile
            .as_deref()
            .ok_or_else(|| CompileFailure::input("select", "missing --execution-profile"))?;
        if profile != host::EXECUTION_PROFILE {
            return Err(CompileFailure::input(
                "select",
                format!("unsupported execution profile: {profile}"),
            ));
        }
        let body_typed_nominal = args.source_profile.as_deref()
            == Some(host::nominal_body_compile::NOMINAL_BODY_TYPED_PROFILE);
        let selection = select_source(
            args.source_profile.as_deref(),
            args.action.as_deref(),
            args.entry,
        )
        .map_err(|error| match error {
            host::draft_contract::SelectionError::MissingAction => {
                CompileFailure::input("select", "selected Sherlock compilation requires --action")
            }
            other => CompileFailure {
                stage: "select",
                actual_error: serde_json::json!({
                    "selection_error": format!("{other:?}"),
                    "source_profile": args.source_profile.as_deref(),
                    "entry": args.entry, "action": args.action.as_deref(),
                }),
                evaluation_performed: false,
                events_forwarded: "0".into(),
                completed: false,
            },
        })?;
        if args.execution_inputs.is_some() && !matches!(&selection, SourceSelection::CoreAction(_))
        {
            return Err(CompileFailure::input(
                "select",
                "--execution-inputs is supported only for Core/1 action compilation",
            ));
        }
        if args.nominal_inputs.is_some() || args.nominal_function.is_some() {
            if args.nominal_inputs.is_none() || args.nominal_function.is_none() {
                return Err(CompileFailure::input(
                    "select",
                    "--nominal-inputs and --nominal-function must be supplied together",
                ));
            }
            if !body_typed_nominal || !matches!(&selection, SourceSelection::NominalEntry) {
                return Err(CompileFailure::input(
                    "select",
                    "--nominal-inputs is supported only for nominal-body-typed/1 entry compilation",
                ));
            }
        }
        if matches!(&selection, SourceSelection::DraftEntry) && args.observe {
            return Err(CompileFailure::input(
                "select",
                "draft preparation is compile-only; --observe is unsupported",
            ));
        }
        if matches!(&selection, SourceSelection::NominalEntry)
            && args.observe
            && !body_typed_nominal
        {
            return Err(CompileFailure::input(
                "select",
                "nominal preparation is compile-only; --observe is unsupported",
            ));
        }
        let config_path = args.execution_config.as_deref().ok_or_else(|| {
            CompileFailure::input(
                "config",
                "selected Sherlock compilation requires --execution-config",
            )
        })?;
        let config = core_io::read_source(config_path)
            .map_err(|error| CompileFailure::input("config", error.to_string()))?;
        // Target bytes are an explicit descriptor and digest, not native support.
        let target = core_io::read_source(&args.target)
            .map_err(|error| CompileFailure::input("config", error.to_string()))?;
        let inputs = args
            .execution_inputs
            .as_deref()
            .map(core_io::read_source)
            .transpose()
            .map_err(|error| CompileFailure::input("inputs", error.to_string()))?;
        let nominal_inputs = args
            .nominal_inputs
            .as_deref()
            .map(core_io::read_source)
            .transpose()
            .map_err(|error| CompileFailure::input("inputs", error.to_string()))?;
        let artifact = match selection {
            SourceSelection::CoreAction(action) => {
                SelectedSherlockArtifact::Core(if let Some(inputs) = inputs.as_deref() {
                    host::compile_host_with_inputs(
                        source,
                        action,
                        &config,
                        &target,
                        inputs,
                        args.observe,
                    )?
                } else {
                    host::compile_host(source, action, &config, &target, args.observe)?
                })
            }
            SourceSelection::DraftEntry => {
                SelectedSherlockArtifact::Draft(host::compile_draft_host(source, &config, &target)?)
            }
            SourceSelection::NominalEntry => {
                if body_typed_nominal {
                    if args.observe {
                        SelectedSherlockArtifact::NominalObserved(
                            match (nominal_inputs.as_deref(), args.nominal_function.as_deref()) {
                                (Some(inputs), Some(function)) => {
                                    host::nominal_body_command::observe_with_nominal_inputs(
                                        source, &config, &target, inputs, function,
                                    )?
                                }
                                (None, None) => {
                                    host::nominal_body_command::observe(source, &config, &target)?
                                }
                                _ => {
                                    return Err(CompileFailure::input(
                                        "select",
                                        "--nominal-inputs and --nominal-function must be supplied together",
                                    ));
                                }
                            },
                        )
                    } else {
                        SelectedSherlockArtifact::NominalBodyTyped(
                            match (nominal_inputs.as_deref(), args.nominal_function.as_deref()) {
                                (Some(inputs), Some(function)) => {
                                    host::nominal_body_command::compile_with_nominal_inputs(
                                        source, &config, &target, inputs, function,
                                    )?
                                }
                                (None, None) => {
                                    host::nominal_body_command::compile(source, &config, &target)?
                                }
                                _ => {
                                    return Err(CompileFailure::input(
                                        "select",
                                        "--nominal-inputs and --nominal-function must be supplied together",
                                    ));
                                }
                            },
                        )
                    }
                } else {
                    SelectedSherlockArtifact::Nominal(host::nominal_command::compile(
                        source, &config, &target,
                    )?)
                }
            }
        };
        let manifest = artifact.manifest()?;
        Ok((artifact, manifest))
    };
    let (artifact, manifest) = match execute() {
        Ok(result) => result,
        Err(failure) => {
            return Err(report_sherlock_failure(&failure));
        }
    };
    // Complete preparation/optional observation and serialization before reserving
    // NEWDIR. Existing publish_new never replaces a concurrently created leaf.
    std::fs::create_dir(&args.out).map_err(|source| {
        let actual = publish_io::WriteError {
            path: args.out.clone(),
            source,
        };
        report_sherlock_failure(&artifact.failure("publish", actual.to_string()))
    })?;
    publish_io::publish_new(
        &args.out.join("sherlock-artifact.json"),
        manifest.as_bytes(),
    )
    .map_err(|error| report_sherlock_failure(&artifact.failure("publish", error.to_string())))?;
    status(
        "Prepared",
        format!("Sherlock HostOnly artifact into {}", args.out.display()),
    );
    Ok(ExitCode::SUCCESS)
}

// Serialize the selected artifact directly: no new envelope around artifact/1.
enum SelectedSherlockArtifact {
    Core(mori_core::sherlock_compile_artifact::CompileArtifact),
    Draft(mori_core::sherlock_compile_artifact::draft_report::DraftCompileArtifact),
    Nominal(serde_json::Value),
    NominalBodyTyped(serde_json::Value),
    NominalObserved(
        mori_core::sherlock_compile_artifact::nominal_body_command::NominalHostObservation,
    ),
}
impl SelectedSherlockArtifact {
    fn failure(
        &self,
        stage: &'static str,
        message: impl Into<String>,
    ) -> mori_core::sherlock_compile_artifact::CompileFailure {
        match self {
            Self::Core(value) => value.failure(stage, message),
            Self::Draft(value) => value.failure(stage, message),
            Self::NominalObserved(value) => value.failure(stage, message),
            Self::Nominal(_) => {
                mori_core::sherlock_compile_artifact::CompileFailure::input(stage, message)
            }
            // This private variant is constructed only after the actual typed
            // command succeeds. Retain its observed body disposition on later
            // serialization/publication errors; runtime completion stays false.
            Self::NominalBodyTyped(value) => mori_core::sherlock_compile_artifact::CompileFailure {
                stage,
                actual_error: serde_json::json!({"message": message.into(),
                    "source_profile": value["source_profile"],
                    "body_typechecking": value["body_typechecking"]}),
                evaluation_performed: false,
                events_forwarded: "0".into(),
                completed: false,
            },
        }
    }
    fn manifest(&self) -> Result<String, mori_core::sherlock_compile_artifact::CompileFailure> {
        match self {
            Self::Core(value) => serde_json::to_string_pretty(value)
                .map_err(|error| value.failure("observation", error.to_string())),
            Self::Draft(value) => serde_json::to_string_pretty(value)
                .map_err(|error| value.failure("serialization", error.to_string())),
            Self::NominalObserved(value) => serde_json::to_string_pretty(value)
                .map_err(|error| value.failure("serialization", error.to_string())),
            Self::Nominal(value) | Self::NominalBodyTyped(value) => {
                serde_json::to_string_pretty(value)
                    .map_err(|error| self.failure("serialization", error.to_string()))
            }
        }
    }
}

fn report_sherlock_failure(
    failure: &mori_core::sherlock_compile_artifact::CompileFailure,
) -> mori_diagnostics::Report {
    match mori_core::json::export(failure) {
        Ok(diagnostic) => core_io::operation(diagnostic),
        Err(error) => core_io::operation(error),
    }
}
