//! Explicit ordinary draft/nominal source routing; no resource/security policy here.
//! H1 and owner/report allocation premises remain; no trace or native dispatch.
use super::super::source_io as core_io;
use super::Args;
use mori_core::sherlock::helper_machine::shared::{OutcomeC, StepEventC};
use mori_core::sherlock_compile_artifact::{self as host, CompileFailure};
use std::process::ExitCode;

pub(super) use crate::cli_args::simulate::selection;

fn print_failure(failure: &CompileFailure) -> super::super::Result<ExitCode> {
    core_io::print(failure)?;
    Ok(ExitCode::FAILURE)
}

// Own the actual selected runtime product until the public handler projects it.
// This is private command plumbing, not an artifact importer or a new Core API.
enum SelectedRun {
    Draft(host::draft_execution::DraftHostRun),
    Chunked(host::draft_execution::DraftHostRun, serde_json::Value),
    Nominal(host::nominal_body_command::NominalHostRun),
}

fn chunk_request(args: &Args) -> Option<host::draft_chunked::ChunkRequest> {
    use host::draft_chunked::{ChunkEvents, ChunkPlan, ChunkRequest};
    let plan = match (args.chunk, &args.chunk_at) {
        (Some(n), _) => ChunkPlan::Every(n),
        (None, Some(points)) => ChunkPlan::At(points.clone()),
        (None, None) => return None,
    };
    Some(ChunkRequest {
        plan,
        events: if args.chunk_events {
            ChunkEvents::Retained
        } else {
            ChunkEvents::NotRetained
        },
    })
}

fn execute(args: &Args, sink: &mut dyn FnMut(&StepEventC)) -> Result<SelectedRun, CompileFailure> {
    let (config, target) = selection(args)?;
    let source = core_io::read_source(&args.file)
        .map_err(|error| CompileFailure::input("source", error.to_string()))?;
    let config = core_io::read_source(config)
        .map_err(|error| CompileFailure::input("config", error.to_string()))?;
    let target = core_io::read_source(target)
        .map_err(|error| CompileFailure::input("config", error.to_string()))?;
    let nominal = args.source_profile.as_deref()
        == Some(host::nominal_body_compile::NOMINAL_BODY_TYPED_PROFILE);
    if let Some(request) = chunk_request(args) {
        if nominal {
            return Err(CompileFailure::input(
                "config",
                "technical chunks are supported on the dedicated draft route only",
            ));
        }
        return host::draft_execution::run_draft_host_chunked(
            &source, &config, &target, &request, sink,
        )
        .map(|(run, report)| SelectedRun::Chunked(run, report));
    }
    if nominal {
        host::nominal_body_command::run(&source, &config, &target, sink).map(SelectedRun::Nominal)
    } else {
        host::draft_execution::run_draft_host(&source, &config, &target, sink)
            .map(SelectedRun::Draft)
    }
}

pub(super) fn run(args: &Args) -> super::super::Result<ExitCode> {
    // Retain no event vector. Both existing consumers forward every actual full
    // event once and report its actual count; this command does not claim a trace.
    let mut sink = |_event: &StepEventC| {};
    let selected = match execute(args, &mut sink) {
        Ok(run) => run,
        Err(failure) => return print_failure(&failure),
    };
    match selected {
        SelectedRun::Chunked(run, chunks) => {
            let report = match host::draft_simulation_report::report(&run) {
                Ok(report) => report,
                Err(failure) => return print_failure(&failure),
            };
            let mut value = match serde_json::to_value(&report) {
                Ok(value) => value,
                Err(error) => {
                    return print_failure(&run.failure("serialization", error.to_string()));
                }
            };
            value["execution"]["technical_chunks"] = chunks;
            if let Err(error) = core_io::print(&value) {
                return print_failure(&run.failure("serialization", error.to_string()));
            }
            Ok(exit_for(run.completed.summary.outcome))
        }
        SelectedRun::Draft(run) => {
            let report = match host::draft_simulation_report::report(&run) {
                Ok(report) => report,
                Err(failure) => return print_failure(&failure),
            };
            if let Err(error) = core_io::print(&report) {
                return print_failure(&run.failure("serialization", error.to_string()));
            }
            // Preserve the actual runtime outcome. Completed owner tooling is not
            // a fabricated accepted result; unexpected Running stays non-success.
            Ok(match run.completed.summary.outcome {
                OutcomeC::Halted { .. } => ExitCode::SUCCESS,
                OutcomeC::Rejected { .. } | OutcomeC::Exhausted { .. } | OutcomeC::Running => {
                    ExitCode::FAILURE
                }
            })
        }
        SelectedRun::Nominal(run) => {
            let outcome = run.completed().summary.outcome;
            // Consume the SAME actual admitted/image/summary pair. This existing
            // projection performs no parse, admission, genesis or owner step.
            let report = match run.into_observation() {
                Ok(report) => report,
                Err(failure) => return print_failure(&failure),
            };
            if let Err(error) = core_io::print(&report) {
                return print_failure(&report.failure("serialization", error.to_string()));
            }
            Ok(match outcome {
                OutcomeC::Halted { .. } => ExitCode::SUCCESS,
                OutcomeC::Rejected { .. } | OutcomeC::Exhausted { .. } | OutcomeC::Running => {
                    ExitCode::FAILURE
                }
            })
        }
    }
}

fn exit_for(outcome: OutcomeC) -> ExitCode {
    match outcome {
        OutcomeC::Halted { .. } => ExitCode::SUCCESS,
        OutcomeC::Rejected { .. } | OutcomeC::Exhausted { .. } | OutcomeC::Running => {
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
#[path = "simulate_nominal_tests.rs"]
mod nominal_tests;
