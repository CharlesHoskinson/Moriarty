//! Explicit ordinary draft-source routing; no resource/security policy here.
//! H1 and owner/report allocation premises remain; no trace or native dispatch.
use std::process::ExitCode;
use mori_core::sherlock_compile_artifact::{self as host, CompileFailure};
use mori_core::sherlock::helper_machine::shared::{OutcomeC, StepEventC};
use super::Args;
use super::super::core_io;

pub(super) use crate::cli_args::simulate::selection;

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
