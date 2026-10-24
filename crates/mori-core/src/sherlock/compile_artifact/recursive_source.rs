//! Internal source-preparation/admission proposal for the existing draft dialect.
//! No registered public dispatcher, Core grammar change or evaluator is added.
//! Root must resolve the source-profile/report and preprocessing-classification
//! prerequisites before connecting this leaf to any public source route.
//!
//! All three preprocessing phases (parser R181, typechecker R200, and A-to-B
//! AST copy R208) require explicit caller-supplied budgets; no unbounded
//! parser, checker, or `source_input_bridge::from_program` fallback remains.
use crate::sherlock::helper_machine::shared::owner_host::{
    HostObservation, host_owner_engine, observe_bound_image,
};
use crate::sherlock::helper_machine::shared::{
    Admitted, BoundsC, FinanceInput, HostViolation, LayoutSchemaC, MachineImage, OwnerEngine,
    RunIntC, Tooling,
};
use crate::sherlock::helper_machine::source_input_bridge::{
    self, SourceCopyBudget, SourceCopyError,
};
use crate::sherlock_draft::{
    DraftError, DraftParseBudget, ParsedDraft, parse_draft_source_bounded,
};
use crate::sherlock_draft_types::{DraftTypeError, DraftTypeSummary, check_program_types_bounded};

// Existing grammar identity, not moriarty/1 or moriarty-core/1.
pub(crate) const SOURCE_LANGUAGE: &str = crate::sherlock_draft::DRAFT_LANGUAGE;

/// Caller-owned preparation budgets for bounded parse, typecheck, and A-to-B copy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct DraftPreparationBudget {
    pub(crate) parse: DraftParseBudget,
    pub(crate) max_type_depth: usize,
    pub(crate) copy: SourceCopyBudget,
}

pub(crate) struct PreparedDraftSource {
    pub(crate) draft: ParsedDraft,
    pub(crate) types: DraftTypeSummary,
}

pub(crate) enum DraftSourceFailure {
    Source(DraftError),
    Types(DraftTypeError),
    Copy(SourceCopyError),
    Admit(Tooling),
    AdmitRun(Tooling),
}

/// Bounded parse and typecheck only. All bodies are checked without recursive
/// call unrolling under explicit caller-owned parser and type-depth budgets.
/// This is not a public/untrusted-input admission or a global host-resource
/// guarantee (H1).
pub(crate) fn prepare_checked_draft(
    source: &str,
    parse_budget: DraftParseBudget,
    max_type_depth: usize,
) -> Result<PreparedDraftSource, DraftSourceFailure> {
    let draft =
        parse_draft_source_bounded(source, parse_budget).map_err(DraftSourceFailure::Source)?;
    let types = check_program_types_bounded(draft.program.as_ref(), max_type_depth)
        .map_err(DraftSourceFailure::Types)?;
    Ok(PreparedDraftSource { draft, types })
}

/// Explicit bounded alias for [`prepare_checked_draft`].
pub(crate) fn prepare_checked_draft_bounded(
    source: &str,
    parse_budget: DraftParseBudget,
    max_type_depth: usize,
) -> Result<PreparedDraftSource, DraftSourceFailure> {
    prepare_checked_draft(source, parse_budget, max_type_depth)
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

/// Copy the complete checked AST into the dedicated-sort B carrier under the
/// caller's explicit [`SourceCopyBudget`], then invoke the fixed owner
/// admission and genesis operations. Calls remain `Call` nodes; list/tree
/// constructors and ordered arms are retained. No expression is evaluated into
/// a result literal, and unbounded `from_program` is never called.
pub(crate) fn admit_checked_draft(
    preparation: PreparedDraftSource,
    copy_budget: &SourceCopyBudget,
    bounds: &BoundsC,
    layout: &LayoutSchemaC,
    fuel: &RunIntC,
    stack: &RunIntC,
) -> Result<AdmittedDraftSource, DraftSourceFailure> {
    let owner_source =
        source_input_bridge::from_program_bounded(preparation.draft.program.as_ref(), copy_budget)
            .map_err(DraftSourceFailure::Copy)?;
    let owner = host_owner_engine();
    let admitted = owner
        .admit(&owner_source, bounds, layout)
        .map_err(DraftSourceFailure::Admit)?;
    let image = owner
        .admit_run(&admitted, fuel, stack, &FinanceInput::Absent)
        .map_err(DraftSourceFailure::AdmitRun)?;
    Ok(AdmittedDraftSource {
        preparation,
        admitted,
        image,
    })
}

/// Explicit bounded alias for [`admit_checked_draft`].
pub(crate) fn admit_checked_draft_bounded(
    preparation: PreparedDraftSource,
    copy_budget: &SourceCopyBudget,
    bounds: &BoundsC,
    layout: &LayoutSchemaC,
    fuel: &RunIntC,
    stack: &RunIntC,
) -> Result<AdmittedDraftSource, DraftSourceFailure> {
    admit_checked_draft(preparation, copy_budget, bounds, layout, fuel, stack)
}
