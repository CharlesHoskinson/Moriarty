//! Retain the actual checked source AST alongside its real B genesis pair.
//! Host diagnostic source bridge only. No A/B state conversion, step, proof,
//! source-digest authentication or global H1 admission is introduced.
use super::CompileFailure;
use super::draft_report::DraftCompileArtifact;
use super::recursive_source::AdmittedDraftSource;
use crate::sherlock::Program;
use crate::sherlock::helper_machine::shared::HostViolation;
use crate::sherlock::helper_machine::shared::owner_host::HostObservation;
use std::sync::Arc;

/// The same syntax that the existing bounded A-to-B copier consumed, plus the
/// same unstepped owner pair from which `preparation` was projected.
/// The A reference syntax is not an admitted A machine capability or B image.
pub struct DraftReferencePreparation {
    preparation: DraftCompileArtifact,
    pair: AdmittedDraftSource,
}
impl DraftReferencePreparation {
    /// Immutable report projected from this retained pair at construction.
    pub fn preparation(&self) -> &DraftCompileArtifact {
        &self.preparation
    }
    pub fn reference_program(&self) -> &Arc<Program> {
        &self.pair.preparation.draft.program
    }
    pub fn observe_b_genesis(&self) -> Result<HostObservation<'_>, HostViolation> {
        self.pair.observe()
    }
}

/// Same private bounded preparation and report projection as compile_draft_host.
/// No compiler output is evaluated into a substitute literal. All existing
/// H1/copy/Drop/report and semantic correspondence limitations remain.
pub fn compile_draft_with_reference(
    source: &str,
    config_source: &str,
    target_source: &str,
) -> Result<DraftReferencePreparation, CompileFailure> {
    let (pair, config, target) = super::prepare_draft_pair(source, config_source, target_source)?;
    let preparation = super::project_draft_pair(&pair, source, config, target)?;
    Ok(DraftReferencePreparation { preparation, pair })
}
