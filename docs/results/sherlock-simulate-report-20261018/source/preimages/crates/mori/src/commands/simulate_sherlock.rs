//! Explicit ordinary draft-source routing; no resource/security policy here.
//! H1 and owner/report allocation premises remain; no trace or native dispatch.
use std::{path::Path, process::ExitCode};
use mori_core::sherlock_compile_artifact::{self as host, CompileFailure};
use mori_core::sherlock::helper_machine::shared::{OutcomeC, StepEventC};
use super::Args;
use super::super::core_io;

/// Pure command selection before input reads. Clap enforces the same conflicts
/// for public invocations; the explicit check also keeps direct callers honest.
pub(super) fn selection(args: &Args) -> Result<(&Path, &Path), CompileFailure> {
    if args.action.is_some() || args.scenario.is_some() || args.resume.is_some()
        || args.event.is_some() || args.trace || args.logical_cap.is_some()
        || args.authorization.present()
    {
        return Err(CompileFailure::input("select",
            "selected Sherlock draft simulation does not accept action/scenario, resume/event, trace, logical-cap or signed replay arguments"));
    }
    let execution_profile = args.execution_profile.as_deref().ok_or_else(||
        CompileFailure::input("select", "missing --execution-profile"))?;
    if execution_profile != host::EXECUTION_PROFILE {
        return Err(CompileFailure::input("select",
            format!("unsupported execution profile: {execution_profile}")));
    }
    let source_profile = args.source_profile.as_deref().ok_or_else(||
        CompileFailure::input("select", "selected Sherlock simulation requires explicit --source-profile"))?;
    match host::draft_contract::select_source(Some(source_profile), None, args.entry) {
        Ok(host::draft_contract::SourceSelection::DraftEntry) => {}
        Ok(host::draft_contract::SourceSelection::CoreAction(_)) => {
            return Err(CompileFailure::input("select", "selected Sherlock simulation supports the explicit dedicated draft entry"));
        }
        Err(error) => {
            return Err(CompileFailure {
                stage: "select", actual_error: serde_json::json!({
                    "selection_error": format!("{error:?}"),
                    "source_profile": source_profile, "entry": args.entry,
                }), evaluation_performed: false, events_forwarded: "0".into(), completed: false,
            });
        }
    }
    let config = args.execution_config.as_deref().ok_or_else(||
        CompileFailure::input("config", "selected Sherlock simulation requires --execution-config"))?;
    let target = args.target.as_deref().ok_or_else(||
        CompileFailure::input("config", "selected Sherlock simulation requires --target"))?;
    Ok((config, target))
}

fn print_failure(failure: &CompileFailure) -> super::super::Result<ExitCode> {
    core_io::print(failure)?;
    Ok(ExitCode::FAILURE)
}

pub(super) fn run(args: &Args) -> super::super::Result<ExitCode> {
    let input = (|| -> Result<(String, String, String), CompileFailure> {
        let (config, target) = selection(args)?;
        let source = core_io::read_source(&args.file)
            .map_err(|error| CompileFailure::input("source", error.to_string()))?;
        let config = core_io::read_source(config)
            .map_err(|error| CompileFailure::input("config", error.to_string()))?;
        let target = core_io::read_source(target)
            .map_err(|error| CompileFailure::input("config", error.to_string()))?;
        Ok((source, config, target))
    })();
    let (source, config, target) = match input {
        Ok(input) => input,
        Err(failure) => return print_failure(&failure),
    };
    // Retain no event vector. R258 still forwards every actual full event once
    // and reports its existing actual count; this command does not claim a trace.
    let mut sink = |_event: &StepEventC| {};
    let run = match host::draft_execution::run_draft_host(&source, &config, &target, &mut sink) {
        Ok(run) => run,
        Err(failure) => return print_failure(&failure),
    };
    let report = match host::draft_simulation_report::report(&run) {
        Ok(report) => report,
        Err(failure) => return print_failure(&failure),
    };
    if let Err(error) = core_io::print(&report) {
        return print_failure(&run.failure("serialization", error.to_string()));
    }
    // Preserve the actual runtime outcome. Completed owner tooling is not a
    // fabricated accepted result, and any unexpected Running stays non-success.
    Ok(match run.completed.summary.outcome {
        OutcomeC::Halted { .. } => ExitCode::SUCCESS,
        OutcomeC::Rejected { .. } | OutcomeC::Exhausted { .. } | OutcomeC::Running => ExitCode::FAILURE,
    })
}
