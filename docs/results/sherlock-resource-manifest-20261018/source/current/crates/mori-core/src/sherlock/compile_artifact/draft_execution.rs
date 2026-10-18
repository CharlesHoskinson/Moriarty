//! Public dedicated-source execution through the existing fixed B host owner.
//! SOURCE_ONLY / UNRUN. No CLI, grammar, guard, policy, budget or native change.
//!
//! Parser/typechecker/copy extents come unchanged from the existing draft config.
//! H1's process-wide time/heap/stack guarantees, source-AST cleanup, owner/report
//! allocations and caller-owned sink retention remain separate premises. This
//! in-process diagnostic API is not an untrusted-input sandbox or a proof path.
use serde_json::json;

use crate::sherlock::helper_machine::core_action_consumer::{
    CompletedRun, ConsumerError, EventForwarding,
};
use crate::sherlock::helper_machine::resource_report::ResourceReport;
use crate::sherlock::helper_machine::shared::owner_host::{host_owner_engine, HostObservation};
use crate::sherlock::helper_machine::shared::{OwnerEngine, StepEventC};

use super::draft_contract::DraftConfigIdentity;
use super::draft_report::{self, DraftCompileArtifact};
use super::recursive_source::{self, AdmittedDraftSource, DraftPreparationBudget};
use super::wire::OwnerWire;
use super::{parse_draft_config, CompileFailure, TargetDescriptor};

/// Unstepped preparation snapshot and the actual completed run of that pair.
///
/// `preparation` deliberately keeps its original genesis-only observation and
/// unproved correspondence fields; it is not relabeled as runtime evidence.
/// Read the actual post-run image/summary through `completed` and `observe()`.
/// An Ok wrapper means owner run returned a summary, including real Rejected or
/// Exhausted outcomes. It does not mean the program halted successfully.
pub struct DraftHostRun {
    pub preparation: DraftCompileArtifact,
    pub completed: CompletedRun,
}

impl DraftHostRun {
    /// Borrow the actual owner-bound post-run pair. An observation refusal keeps
    /// the real completed summary and forwarded-event count in ConsumerError.
    pub fn observe(&self) -> Result<HostObservation<'_>, ConsumerError> {
        self.completed.observe()
    }

    /// Actual owner counters/storage only; no A/B equal-fuel or physical-cost
    /// inference. Existing observation errors are propagated unchanged.
    pub fn resource_report(&self) -> Result<ResourceReport, ConsumerError> {
        let observation = self.observe()?;
        Ok(ResourceReport::from_owner(&observation, &self.completed.summary))
    }

    /// Preserve completed-run metadata on a later publication/serialization
    /// failure. `completed` includes Rejected/Exhausted, not only Halted.
    /// The preparation snapshot's failure method describes preprocessing only.
    pub fn failure(&self, stage: &'static str, message: impl Into<String>) -> CompileFailure {
        CompileFailure {
            stage,
            actual_error: json!({"message": message.into()}),
            evaluation_performed: true,
            events_forwarded: self.completed.events_forwarded.to_string(),
            completed: true,
        }
    }
}

/// Bounded source preparation, unstepped report, then actual B-owner execution.
///
/// No compile-only API is called and no second admission is performed. Every
/// Call/constructor/match remains in the prepared owner program. The sink sees
/// each full actual StepEventC exactly once in owner order; this leaf retains
/// no event vector. Run HostViolation maps through the existing ConsumerError
/// and CompileFailure contract with a partial-prefix count and completed=false.
/// Sink panics follow the existing scalar consumer contract and are not caught.
pub fn run_draft_host(
    source: &str,
    config_source: &str,
    target_source: &str,
    sink: &mut dyn FnMut(&StepEventC),
) -> Result<DraftHostRun, CompileFailure> {
    let config = parse_draft_config(config_source)?;
    let target_descriptor = crate::json::parse(target_source)
        .map_err(|error| CompileFailure::core("config", error))?;
    let budget = DraftPreparationBudget {
        parse: config.preprocessing.parser_budget(),
        max_type_depth: config.preprocessing.max_type_depth.0,
        copy: config.preprocessing.copy_budget(),
    };
    let bounds = config.bounds.owner();
    let layout = config.layout.owner(bounds);
    let fuel = config.fuel.owner();
    let stack = config.stack.owner();
    let checked = recursive_source::prepare_checked_draft_bounded(
        source, budget.parse, budget.max_type_depth,
    ).map_err(CompileFailure::draft_source)?;
    let pair = recursive_source::admit_checked_draft_bounded(
        checked, &budget.copy, &bounds, &layout, &fuel, &stack,
    ).map_err(CompileFailure::draft_source)?;

    // Own the genesis snapshot of precisely the same pair that is moved below.
    // Projection borrows it; it cannot supply a different admission or image.
    let preparation = draft_report::project_admitted_preparation(
        &pair,
        crate::intent::hash_bytes(source.as_bytes()),
        DraftConfigIdentity {
            fields: config,
            file_digest: crate::intent::hash_bytes(config_source.as_bytes()),
        },
        TargetDescriptor {
            descriptor: target_descriptor,
            file_digest: crate::intent::hash_bytes(target_source.as_bytes()),
        },
    ).map_err(|error| CompileFailure {
        stage: "observation", actual_error: error.owner_wire(),
        evaluation_performed: false, events_forwarded: "0".into(), completed: false,
    })?;

    let AdmittedDraftSource { preparation: _, admitted, mut image } = pair;
    let engine = host_owner_engine();
    let mut events = EventForwarding::default();
    let result = {
        let mut forwarding_sink = |event: &StepEventC| {
            events.note();
            sink(event);
        };
        engine.run(&admitted, &mut image, &mut forwarding_sink)
    };
    let summary = result.map_err(|violation| CompileFailure::consumer(ConsumerError::Run {
        violation,
        events_forwarded: events.forwarded,
    }))?;
    Ok(DraftHostRun {
        preparation,
        completed: CompletedRun {
            admitted,
            image,
            summary,
            events_forwarded: events.forwarded,
        },
    })
}

// The selected public draft run exposes the same owner-bound reporter.
impl DraftHostRun {
    pub fn resource_report_with_manifest(&self)
        -> Result<crate::sherlock::helper_machine::resource_report::ManifestResourceReport, ConsumerError>
    {
        let observation = self.observe()?;
        Ok(ResourceReport::from_owner_with_manifest(&observation, &self.completed.summary))
    }
}
