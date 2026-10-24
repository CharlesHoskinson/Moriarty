use std::path::Path;
use std::process::ExitCode;

use crate::shell::Shell;

#[path = "simulate_sherlock.rs"]
mod sherlock;

pub use crate::cli_args::simulate::Args;

impl Args {
    /// Legacy start inputs. Clap requires both outside resume or explicit Sherlock selection.
    fn legacy(&self) -> super::Result<(&str, &Path)> {
        match (self.action.as_deref(), self.scenario.as_deref()) {
            (Some(action), Some(scenario)) => Ok((action, scenario)),
            _ => Err(super::core_io::operation(
                "--action and --scenario are required without --resume",
            )),
        }
    }
}

pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    if args.execution_profile.is_some()
        || args.source_profile.is_some()
        || args.entry
        || args.execution_config.is_some()
    {
        return sherlock::run(&args);
    }
    // Clap has already refused every --resume/--event conflict, so a
    // malformed invocation reads no file.
    if let Some(resume) = &args.resume {
        return run_resume(&args, resume);
    }
    let (action, scenario_path) = args.legacy()?;
    if args.trace {
        let path = args
            .target
            .as_ref()
            .ok_or_else(|| super::core_io::operation("--trace requires --target"))?;
        #[cfg(feature = "native")]
        let target = super::core_io::read_json(path)?;
        #[cfg(feature = "native")]
        let authorization = args.authorization.load(&target)?;
        #[cfg(feature = "native")]
        let source = super::core_io::read_source(&args.file)?;
        #[cfg(feature = "native")]
        let scenario = super::core_io::read_json(scenario_path)?;
        #[cfg(feature = "native")]
        return match mori_backend::replay::generate(
            &source,
            action,
            &scenario,
            &target,
            authorization.as_input(),
        ) {
            Ok(record) => {
                super::core_io::print(&record)?;
                Ok(ExitCode::SUCCESS)
            }
            Err(error) => {
                super::core_io::print(
                    &serde_json::json!({"status":"rejected","qualification":"local-reference-replay","error":error}),
                )?;
                Ok(ExitCode::FAILURE)
            }
        };
        #[cfg(not(feature = "native"))]
        {
            let _ = path;
            return super::native_unavailable();
        }
    }
    if args.target.is_some() || args.authorization.present() {
        return Err(super::core_io::operation(
            "--target and signed replay arguments require --trace",
        ));
    }
    let source = super::core_io::read_source(&args.file)?;
    // Hook: /2 routing. A /2 start keeps every core failure labelled.
    let language = mori_core::language(&source).map_err(super::core_io::operation)?;
    if language == mori_core::stage::LANGUAGE_2 {
        return run_stage_start(&args, &source, action, scenario_path);
    }
    let program = mori_core::parse(&source).map_err(super::core_io::operation)?;
    let scenario = super::core_io::read_json(scenario_path)?;
    if let Some(cap) = args.logical_cap {
        return match mori_core::simulate_metered(&program, action, &scenario, cap) {
            Ok(metered) => {
                super::core_io::print(&serde_json::json!({
                    "status": "accepted", "qualification": "local-reference-execution",
                    "execution": metered.execution, "profile": metered.cost.profile,
                    "logical": {"cap": metered.cost.cap, "used": metered.cost.used}
                }))?;
                Ok(ExitCode::SUCCESS)
            }
            Err(refused) => {
                super::core_io::print(&serde_json::json!({
                    "status": "rejected", "error": refused.error,
                    "profile": refused.cost.profile,
                    "logical": {"cap": refused.cost.cap, "used": refused.cost.used}
                }))?;
                Ok(ExitCode::FAILURE)
            }
        };
    }
    match mori_core::simulate(&program, action, &scenario) {
        Ok(execution) => {
            super::core_io::print(
                &serde_json::json!({"status":"accepted", "qualification":"local-reference-execution", "execution":execution}),
            )?;
            Ok(ExitCode::SUCCESS)
        }
        Err(error) => {
            super::core_io::print(&serde_json::json!({"status":"rejected", "error":error}))?;
            Ok(ExitCode::FAILURE)
        }
    }
}

/// Render the same explicit qualification on every staged public object.
pub(super) fn print_stage_report(mut report: serde_json::Value) -> super::Result<()> {
    let object = report
        .as_object_mut()
        .expect("stage reports are JSON objects");
    for (name, value) in mori_core::stage::STAGE_LABELS {
        object.insert(name.into(), serde_json::Value::String(value.into()));
    }
    super::core_io::print(&report)
}

fn print_stage_result(
    result: Result<mori_core::stage::StageOutcome, mori_core::stage::StageRejection>,
) -> super::Result<ExitCode> {
    let exit = if result.is_ok() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    };
    super::core_io::print(&mori_core::stage::outcome_report(result))?;
    Ok(exit)
}

/// The owning call retains its exact financial inputs and result. Ordinary
/// Core/2 stdout uses the adopted outcome envelope; the richer Sherlock
/// publication remains a separate API, not an implicit public profile change.
fn print_produced_stage_result(
    produced: &mori_core::sherlock_financial_projection::ProducedInvocation<'_>,
) -> super::Result<ExitCode> {
    print_stage_result(produced.raw_result().clone())
}

/// `--resume`: reads the source, then the continuation, then the event.
/// Bounded-read, strict-decode, load and admission failures are labelled stage
/// rejections; a source the filesystem cannot read keeps the legacy error.
/// Nothing is recorded, so the same file can be resumed again (untracked).
fn run_resume(args: &Args, resume: &Path) -> super::Result<ExitCode> {
    use mori_core::stage::{self, StageRejection, decode};
    let event_path = args
        .event
        .as_deref()
        .ok_or_else(|| super::core_io::operation("--resume requires --event"))?;
    let source = super::core_io::read_source(&args.file)?;
    let loaded = (|| -> Result<(stage::StagedProgram, stage::Continuation, stage::StageEvent), StageRejection> {
        let program = stage::load(&source).map_err(no_work)?;
        let continuation = decode::read_bounded(
            resume,
            decode::MAX_CONTINUATION_BYTES,
            decode::CONTINUATION_TOO_LARGE,
        )
        .and_then(|text| decode::continuation(&text))
        .map_err(no_work)?;
        let event =
            decode::read_bounded(event_path, decode::MAX_EVENT_BYTES, decode::EVENT_TOO_LARGE)
                .and_then(|text| decode::event(&text))
                .map_err(no_work)?;
        Ok((program, continuation, event))
    })();
    match loaded {
        Ok((program, continuation, event)) => {
            let produced = mori_core::sherlock_financial_projection::resume_observed(
                &program,
                continuation,
                event,
                args.action.clone(),
                args.logical_cap,
            );
            print_produced_stage_result(&produced)
        }
        Err(rejected) => print_stage_result(Err(rejected)),
    }
}

/// A `/2` start. Load and scenario failures are labelled stage rejections; the
/// scenario keeps the legacy 1 MiB strict reader.
fn run_stage_start(
    args: &Args,
    source: &str,
    action: &str,
    scenario: &Path,
) -> super::Result<ExitCode> {
    use mori_core::stage::{self, StageRejection, decode};
    let loaded = (|| -> Result<(stage::StagedProgram, mori_core::Scenario), StageRejection> {
        let program = stage::load(source).map_err(no_work)?;
        let text = decode::read_bounded(scenario, 1_048_576, "Scenario JSON exceeds 1 MiB")
            .map_err(no_work)?;
        let value = mori_core::json::parse(&text).map_err(no_work)?;
        let scenario = serde_json::from_value(value).map_err(|error| {
            no_work(mori_core::Error {
                code: "mori::core::json".into(),
                message: error.to_string(),
                offset: None,
                step: None,
            })
        })?;
        Ok((program, scenario))
    })();
    match loaded {
        Ok((program, scenario)) => {
            let produced = mori_core::sherlock_financial_projection::start_observed(
                &program,
                action.to_owned(),
                scenario,
                args.logical_cap,
            );
            print_produced_stage_result(&produced)
        }
        Err(rejected) => print_stage_result(Err(rejected)),
    }
}

/// A refusal before any segment evaluation: no work attempted or committed.
fn no_work(error: mori_core::Error) -> mori_core::stage::StageRejection {
    mori_core::stage::StageRejection {
        error,
        attempted: mori_core::UInt(0),
        committed: mori_core::UInt(0),
    }
}

#[cfg(test)]
#[path = "simulate_sherlock_tests.rs"]
mod sherlock_tests;
