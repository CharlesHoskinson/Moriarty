//! Internal source-preparation/admission proposal for the existing draft dialect.
//! No registered public dispatcher, Core grammar change or evaluator is added.
//! Root must resolve the source-profile/report and preprocessing-classification
//! prerequisites before connecting this leaf to any public source route.
use crate::sherlock_draft::{parse_draft_source, DraftError, ParsedDraft};
use crate::sherlock_draft_types::{check_program_types, DraftTypeError, DraftTypeSummary};
use crate::sherlock::helper_machine::source_input_bridge;
use crate::sherlock::helper_machine::shared::{
    Admitted, BoundsC, FinanceInput, LayoutSchemaC, MachineImage, OwnerEngine,
    RunIntC, Tooling,
};
use crate::sherlock::helper_machine::shared::owner_host::{
    host_owner_engine, observe_bound_image, HostObservation,
};
use crate::sherlock::helper_machine::shared::HostViolation;

// Existing grammar identity, not moriarty/1 or moriarty-core/1.
pub(crate) const SOURCE_LANGUAGE: &str = crate::sherlock_draft::DRAFT_LANGUAGE;

pub(crate) struct PreparedDraftSource {
    pub(crate) draft: ParsedDraft,
    pub(crate) types: DraftTypeSummary,
}

pub(crate) enum DraftSourceFailure {
    Source(DraftError),
    Types(DraftTypeError),
    Admit(Tooling),
    AdmitRun(Tooling),
}

/// Parse/check only. All bodies are checked without recursive call unrolling.
/// Caller preprocessing classification is not supplied by owner table bounds.
/// This is not a public/untrusted-input admission or a host-resource guarantee.
pub(crate) fn prepare_checked_draft(source: &str)
    -> Result<PreparedDraftSource, DraftSourceFailure>
{
    let draft = parse_draft_source(source).map_err(DraftSourceFailure::Source)?;
    let types = check_program_types(draft.program.as_ref()).map_err(DraftSourceFailure::Types)?;
    Ok(PreparedDraftSource { draft, types })
}

pub(crate) struct AdmittedDraftSource {
    pub(crate) preparation: PreparedDraftSource,
    pub(crate) admitted: Admitted,
    pub(crate) image: MachineImage,
}

impl AdmittedDraftSource {
    /// Borrow actual admitted tables and genesis; no synthesized run summary.
    pub(crate) fn observe(&self) -> Result<HostObservation<'_>, HostViolation> {
        observe_bound_image(&self.admitted, &self.image)
    }
}

/// Copy the complete checked AST into the existing dedicated-sort B carrier,
/// then use the same fixed owner admission/genesis operations as the scalar
/// route. Calls remain Call nodes; list/tree constructors and ordered arms are
/// retained. No expression is evaluated into a result literal. Bounds/layout
/// and original three-class run inputs are supplied unchanged by the caller.
/// This does not provide draft parser/preprocessing policy or execute B steps.
pub(crate) fn admit_checked_draft(
    preparation: PreparedDraftSource,
    bounds: &BoundsC,
    layout: &LayoutSchemaC,
    fuel: &RunIntC,
    stack: &RunIntC,
) -> Result<AdmittedDraftSource, DraftSourceFailure> {
    let owner_source = source_input_bridge::from_program(preparation.draft.program.as_ref());
    let owner = host_owner_engine();
    let admitted = owner.admit(&owner_source, bounds, layout).map_err(DraftSourceFailure::Admit)?;
    let image = owner.admit_run(&admitted, fuel, stack, &FinanceInput::Absent)
        .map_err(DraftSourceFailure::AdmitRun)?;
    Ok(AdmittedDraftSource { preparation, admitted, image })
}
