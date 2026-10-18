//! Output-only view of an actual R258 owner run; no execution or policy here.
//! Genesis preparation and post-run state remain distinct. Public JSON size,
//! serialization cost and host allocation remain unmeasured, without a new cap.
use serde::Serialize;
use serde_json::Value;

use crate::sherlock::helper_machine::resource_report::ResourceReport;
use super::draft_execution::DraftHostRun;
use super::draft_report::DraftCompileArtifact;
use super::wire::{self, OwnerWire};
use super::CompileFailure;

#[derive(Serialize)]
pub struct DraftSimulationReport<'a> {
    pub format: &'static str,
    pub qualification: &'static str,
    pub preparation: &'a DraftCompileArtifact,
    pub execution: ExecutionReport,
}

#[derive(Serialize)]
pub struct ExecutionReport {
    pub evaluation_performed: bool,
    pub completed: bool,
    pub summary: Value,
    pub events_forwarded: String,
    pub event_capture: &'static str,
    pub image: Value,
    pub resource_report: ResourceReport,
}

/// Borrow the run's bound post-run pair once and use the existing complete
/// owner projections. This does not infer Halted from successful tooling.
/// The caller must map later serialization errors through `run.failure(...)`.
pub fn report(run: &DraftHostRun) -> Result<DraftSimulationReport<'_>, CompileFailure> {
    let observation = run.observe().map_err(CompileFailure::consumer)?;
    Ok(DraftSimulationReport {
        format: "sherlock-host-simulation/1",
        qualification: "HostOnly dedicated draft owner execution; no proof/native/target qualification",
        preparation: &run.preparation,
        execution: ExecutionReport {
            evaluation_performed: true,
            completed: true,
            summary: run.completed.summary.owner_wire(),
            events_forwarded: run.completed.events_forwarded.to_string(),
            event_capture: "not-retained",
            image: wire::image(&observation.image),
            resource_report: ResourceReport::from_owner(&observation, &run.completed.summary),
        },
    })
}
