//! Staged `moriarty/2` compilation to `/4` artifacts: N + 1 successful-segment
//! circuits per action with N awaits, through the ordinary `/3` lowerer.
//! `/4` is a distinct type with no conversion to the `/3` Artifact, so proving,
//! signed and native consumers cannot accept it. Unsigned only: there is no
//! event authentication, stage authority or consumption, and all ten gates
//! remain unperformed.
mod artifact;
pub(crate) mod context;
pub(crate) mod frame;
mod lower;

pub use artifact::{
    ActionLogical, LOGICAL_ENFORCEMENT, MAX_STAGED_CIRCUITS, PendingPositions,
    STAGE_CONTEXT_FORMAT, STAGE_GATES, STAGED_ABI_VERSION, STAGED_ARTIFACT_FORMAT,
    STAGED_RELATION_VERSION, SegmentLogical, StagedArtifact, StagedCircuit, StagedResources,
};
pub use lower::compile_staged;

use crate::{Error, error};
use mori_core::Program;

/// Explicit `/1` entry refusal for legacy backend consumers, with the existing
/// `mori::backend::program` wrapping of Core's profile error.
pub(crate) fn require_core1(program: &Program) -> Result<(), Error> {
    mori_core::stage::require_core1(program).map_err(|e| error("program", e.to_string()))
}

/// Typed refusal for an Await reaching a `/1`-only statement match.
pub(crate) fn core1_refusal(program: &Program) -> Error {
    match require_core1(program) {
        Err(e) => e,
        Ok(()) => error(
            "program",
            "mori::core::profile: Await statements require the moriarty/2 staged compiler.",
        ),
    }
}
