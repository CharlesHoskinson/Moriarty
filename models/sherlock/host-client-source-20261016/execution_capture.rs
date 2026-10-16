//! Ordinary host client: capture owner-supplied observations and step events.
//! Source proposal only; Root owns registration, compilation and execution.
//! `Ok(())` means this client's initial/event stream reached an observed
//! non-Running outcome. It asserts no comparison, semantic completeness,
//! semantic correspondence, proof, native result or product acceptance.
//!
//! The adopted event contract says P0 and Startup emit no StepEvent.
//! admit_run returns the initial image; its full bound observation is the
//! initial record. Subsequent event records contain only actual step returns.
use std::{fmt, io};
use super::{observation_json, shared, typed_inputs};
use shared::{FinanceInput, HostViolation, OutcomeC, OwnerEngine, RunIntC,
    StepEventC, StepResult, Tooling};
use shared::owner_host::{host_owner_engine, observe_bound_image, HostObservation};

/// The exact JSONL envelope version. Each successful record ends in one LF.
pub const SCHEMA: &str = "sherlock-host-capture-v1";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FixturePart { Program, Bounds, Layout }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operation { Admit, AdmitRun, ObserveInitial, Step, ObservePostimage }

/// Errors retain the original typed owner, formatter or I/O payload.
/// No Debug implementation is required of Tooling or HostViolation.
pub enum CaptureError {
    UnknownFixture { case_id: String, missing: FixturePart },
    Tooling { operation: Operation, error: Tooling },
    HostViolation { operation: Operation, error: HostViolation },
    Render(fmt::Error),
    Io(io::Error),
    /// step returned Terminal although the preceding bound observation was
    /// Running. The client makes no further step call or image mutation.
    UnexpectedTerminalWhileRunning,
}

impl From<fmt::Error> for CaptureError {
    fn from(error: fmt::Error) -> Self { Self::Render(error) }
}
impl From<io::Error> for CaptureError {
    fn from(error: io::Error) -> Self { Self::Io(error) }
}

/// Capture one literal fixture with the caller's unchanged typed F/L inputs.
///
/// This calls only the fixed public host factory, admission, bound observer,
/// and step APIs. It supplies FinanceInput::Absent and chooses no F/L, bounds,
/// layout, step limit, expected count, charge or event metadata.
///
/// Initial record keys, in order:
/// schema, kind, case_id, fuel, stack_limit, observation.
/// Event record keys, in order:
/// schema, kind, case_id, fuel, stack_limit, event, observation.
/// F/L use observation_json's full RunIntC representation. observation and
/// event use the existing full typed writers. No CoreProgram is serialized.
///
/// Rendered lines are buffered individually before I/O. An I/O error can
/// leave a partial final line; a previous line can exist when any later error
/// is returned. The caller must retain the Result with its output. No flush
/// is imposed on the caller-owned sink.
pub fn capture_case<W: io::Write + ?Sized>(
    case_id: &str,
    fuel: &RunIntC,
    stack_limit: &RunIntC,
    out: &mut W,
) -> Result<(), CaptureError> {
    let program = typed_inputs::program(case_id).ok_or_else(||
        CaptureError::UnknownFixture {
            case_id: case_id.to_owned(), missing: FixturePart::Program,
        })?;
    let bounds = typed_inputs::bounds(case_id).ok_or_else(||
        CaptureError::UnknownFixture {
            case_id: case_id.to_owned(), missing: FixturePart::Bounds,
        })?;
    let layout = typed_inputs::layout(case_id).ok_or_else(||
        CaptureError::UnknownFixture {
            case_id: case_id.to_owned(), missing: FixturePart::Layout,
        })?;
    let engine = host_owner_engine();
    let admitted = engine.admit(&program, &bounds, &layout)
        .map_err(|error| CaptureError::Tooling { operation: Operation::Admit, error })?;
    let mut image = engine.admit_run(&admitted, fuel, stack_limit, &FinanceInput::Absent)
        .map_err(|error| CaptureError::Tooling { operation: Operation::AdmitRun, error })?;
    let running = {
        let observation = observe_bound_image(&admitted, &image)
            .map_err(|error| CaptureError::HostViolation {
                operation: Operation::ObserveInitial, error,
            })?;
        write_record(case_id, fuel, stack_limit, None, &observation, out)?;
        matches!(observation.image.outcome, OutcomeC::Running)
    };
    if !running { return Ok(()); }

    loop {
        // The preceding full observation was Running. Every Event comes
        // directly from this call; no synthetic startup or terminal event.
        let event = match engine.step(&admitted, &mut image)
            .map_err(|error| CaptureError::HostViolation {
                operation: Operation::Step, error,
            })? {
            StepResult::Event { event } => event,
            StepResult::Terminal => return Err(CaptureError::UnexpectedTerminalWhileRunning),
        };
        let observation = observe_bound_image(&admitted, &image)
            .map_err(|error| CaptureError::HostViolation {
                operation: Operation::ObservePostimage, error,
            })?;
        write_record(case_id, fuel, stack_limit, Some(&event), &observation, out)?;
        // Stop on the actual image outcome, including terminal publication.
        // No extra Terminal probe, semantic evaluator or client step cap.
        if !matches!(observation.image.outcome, OutcomeC::Running) {
            return Ok(());
        }
    }
}

fn write_record<W: io::Write + ?Sized>(
    case_id: &str,
    fuel: &RunIntC,
    stack_limit: &RunIntC,
    event: Option<&StepEventC>,
    observation: &HostObservation<'_>,
    out: &mut W,
) -> Result<(), CaptureError> {
    let mut line = String::new();
    line.push_str("{\"schema\":");
    observation_json::write_json(&mut line, SCHEMA)?;
    line.push_str(",\"kind\":");
    observation_json::write_json(&mut line, if event.is_some() { "event" } else { "initial" })?;
    line.push_str(",\"case_id\":");
    observation_json::write_json(&mut line, case_id)?;
    line.push_str(",\"fuel\":");
    observation_json::write_json(&mut line, fuel)?;
    line.push_str(",\"stack_limit\":");
    observation_json::write_json(&mut line, stack_limit)?;
    if let Some(event) = event {
        line.push_str(",\"event\":");
        observation_json::write_step_event_json(&mut line, event)?;
    }
    line.push_str(",\"observation\":");
    observation_json::write_host_observation_json(&mut line, observation)?;
    line.push_str("}\n");
    out.write_all(line.as_bytes())?;
    Ok(())
}
