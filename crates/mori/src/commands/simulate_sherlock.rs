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
    /// A draft run with technical chunks and/or K-A reference slices attached.
    Extended(
        host::draft_execution::DraftHostRun,
        Option<serde_json::Value>,
        serde_json::Value,
    ),
    Nominal(host::nominal_body_command::NominalHostRun),
}

fn chunk_request(args: &Args) -> Option<host::draft_chunked::ChunkRequest> {
    use host::draft_chunked::{ChunkEvents, ChunkPlan, ChunkRequest, Representation};
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
        represent: if args.chunk_represent {
            Representation::Bounded
        } else {
            Representation::Off
        },
        witness: args.chunk_witness,
        images: args.chunk_images,
    })
}

/// The optional K-A reference grouping of the same draft source and config.
fn reference_slices(args: &Args) -> Result<Option<serde_json::Value>, CompileFailure> {
    let Some(m) = args.reference_slices else {
        return Ok(None);
    };
    if args.source_profile.as_deref()
        == Some(host::nominal_body_compile::NOMINAL_BODY_TYPED_PROFILE)
    {
        return Err(CompileFailure::input(
            "config",
            "reference slices are supported on the dedicated draft route only",
        ));
    }
    let (config, target) = selection(args)?;
    let read = |path: &std::path::Path, stage: &'static str| {
        core_io::read_source(path).map_err(|error| CompileFailure::input(stage, error.to_string()))
    };
    let source = read(&args.file, "source")?;
    let config = read(config, "config")?;
    let target = read(target, "config")?;
    host::draft_reference_slices::reference_slices(&source, &config, &target, m).map(Some)
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
    let mut tally = host::native_inventory::RuleTally::default();
    let mut sink = |event: &StepEventC| tally.observe(event);
    let slices = match reference_slices(args) {
        Ok(slices) => slices,
        Err(failure) => return print_failure(&failure),
    };
    let selected = match execute(args, &mut sink) {
        Ok(run) => run,
        Err(failure) => return print_failure(&failure),
    };
    let inventory = if args.native_inventory {
        if matches!(selected, SelectedRun::Nominal(_)) {
            return print_failure(&CompileFailure::input(
                "config",
                "the native inventory is supported on the dedicated draft route only",
            ));
        }
        Some(host::native_inventory::report(
            &args.file.display().to_string(),
            &tally,
        ))
    } else {
        None
    };
    let selected = match (selected, slices) {
        (SelectedRun::Draft(run), Some(slices)) => SelectedRun::Extended(run, None, slices),
        (SelectedRun::Draft(run), None) if inventory.is_some() => {
            SelectedRun::Extended(run, None, serde_json::Value::Null)
        }
        (SelectedRun::Chunked(run, chunks), Some(slices)) => {
            SelectedRun::Extended(run, Some(chunks), slices)
        }
        (SelectedRun::Chunked(run, chunks), None) => {
            SelectedRun::Extended(run, Some(chunks), serde_json::Value::Null)
        }
        (other, _) => other,
    };
    match selected {
        SelectedRun::Chunked(run, chunks) => {
            print_extended(&run, Some(chunks), serde_json::Value::Null, inventory)
        }
        SelectedRun::Extended(run, chunks, slices) => {
            print_extended(&run, chunks, slices, inventory)
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
                // Profile /2 Waiting is a resumable success (design note §3.5).
                OutcomeC::Halted { .. } | OutcomeC::Waiting { .. } => ExitCode::SUCCESS,
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
                // Profile /2 Waiting is a resumable success (design note §3.5).
                OutcomeC::Halted { .. } | OutcomeC::Waiting { .. } => ExitCode::SUCCESS,
                OutcomeC::Rejected { .. } | OutcomeC::Exhausted { .. } | OutcomeC::Running => {
                    ExitCode::FAILURE
                }
            })
        }
    }
}

/// The ordinary draft report plus the requested chunk and slice sections.
fn print_extended(
    run: &host::draft_execution::DraftHostRun,
    chunks: Option<serde_json::Value>,
    slices: serde_json::Value,
    inventory: Option<serde_json::Value>,
) -> super::super::Result<ExitCode> {
    let report = match host::draft_simulation_report::report(run) {
        Ok(report) => report,
        Err(failure) => return print_failure(&failure),
    };
    let mut value = match serde_json::to_value(&report) {
        Ok(value) => value,
        Err(error) => return print_failure(&run.failure("serialization", error.to_string())),
    };
    if let Some(chunks) = chunks {
        value["execution"]["technical_chunks"] = chunks;
    }
    if !slices.is_null() {
        value["reference_slices"] = slices;
    }
    if let Some(inventory) = inventory {
        value["native_inventory"] = inventory;
    }
    if let Err(error) = core_io::print(&value) {
        return print_failure(&run.failure("serialization", error.to_string()));
    }
    Ok(exit_for(run.completed.summary.outcome))
}

fn exit_for(outcome: OutcomeC) -> ExitCode {
    match outcome {
        // Profile /2 Waiting is a resumable success (design note §3.5).
        OutcomeC::Halted { .. } | OutcomeC::Waiting { .. } => ExitCode::SUCCESS,
        OutcomeC::Rejected { .. } | OutcomeC::Exhausted { .. } | OutcomeC::Running => {
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
#[path = "simulate_nominal_tests.rs"]
mod nominal_tests;
