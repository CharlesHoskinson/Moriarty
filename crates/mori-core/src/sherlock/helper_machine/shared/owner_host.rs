// owner_host.proposed.rs — shared::owner_host (child of `shared`)
// PROPOSED, source-only, uncompiled, unadopted; no execution authority.
// Fixed host engine constructor plus read-only observation of a bound pair.
// Plain std: no unsafe, I/O, allocation, value decoding or traversal (C8).
// host_owner_engine takes no argument: FamilyPlans is the Root-reviewed
// crate::sherlock::helper_machine::families::HostFamilyPlans, OwnerGuards is sealed inside
// owner_runtime::CheckedOwnerEngine::new_reviewed, and PrepareDeps is exactly
// owner_prepare's four bodies. No caller plan, guard or fn pointer enters.
// observe_bound_image runs the existing AB binding (PreImage::bind) before
// any field read, then borrows every admission and MachineImage field.
// It is ordinary host diagnostic observation, never project, decodeCanonical,
// a codec, ChunkStore, Stage 2 or native authority. Admitted values come only
// from admit (HostOnly: widths refused at AD-2b, boundaryCapacities at AD-6)
// and images from admit_run (finance Absent, one uncut segment), so no
// encoding or boundary field exists to map and none is invented. reg.segBase
// has no MachineImage field: ImageObservation.seg_base comes only from
// host_implied_prefix::implied_seg_base on the AB-bound pair, and its Err
// refuses the whole observation (no literal zero, default or omission).
use super::*;
use super::super::raw::DataRow as RawDataRow;
use super::owner_prepare;
use super::owner_runtime::{CheckedOwnerEngine, PrepareDeps};
use super::host_implied_prefix::{implied_seg_base, SegBase};
use crate::sherlock::helper_machine::families::HostFamilyPlans;

use super::fin_carrier::{FinCapacitiesC, FinPreparedStagedSource, FinStagedPrepareError};

/// Callable owner-host source preparation (source-only; no genesis, no
/// execution, no `simulate`/`start`/`resume` substitution). The producing
/// seam is the real `stage::load`; the receiving constructor is
/// `FinPreparedStagedSource::prepare`.
pub fn prepare_fin_staged_program(
    staged: crate::stage::StagedProgram,
    layout: &LayoutSchemaC,
    caps: &FinCapacitiesC,
) -> Result<FinPreparedStagedSource, FinStagedPrepareError> {
    FinPreparedStagedSource::prepare(staged, layout, caps)
}

pub fn prepare_fin_staged_source(
    src: &str,
    layout: &LayoutSchemaC,
    caps: &FinCapacitiesC,
) -> Result<FinPreparedStagedSource, FinStagedPrepareError> {
    let staged = crate::stage::load(src).map_err(FinStagedPrepareError::Stage)?;
    prepare_fin_staged_program(staged, layout, caps)
}

pub fn host_owner_engine() -> impl OwnerEngine {
    CheckedOwnerEngine::new_reviewed(HostFamilyPlans, PrepareDeps {
        prepare_core: owner_prepare::prepare_core,
        check_program_view: owner_prepare::check_program_view,
        admit: owner_prepare::admit,
        admit_run: owner_prepare::admit_run,
    })
}

pub fn host_owner_engine_with_word_layout(
    layout: crate::sherlock_word_lowering::WordLayout,
) -> impl OwnerEngine {
    CheckedOwnerEngine::new_reviewed(
        crate::sherlock::helper_machine::families::WordFamilyPlans::new(layout),
        PrepareDeps {
            prepare_core: owner_prepare::prepare_core,
            check_program_view: owner_prepare::check_program_view,
            admit: owner_prepare::admit,
            admit_run: owner_prepare::admit_run,
        },
    )
}

// Records are built only by observe_bound_image (private _sealed). Every
// field borrows the bound pair, so the image stays immutable while observed.
#[derive(Clone, Copy)]
pub struct HostObservation<'a> {
    pub admission: AdmissionObservation<'a>,
    pub image: ImageObservation<'a>,
    _sealed: (),
}
#[derive(Clone, Copy)]
pub struct AdmissionObservation<'a> {
    pub identity: &'a IdentityC,
    pub bounds: &'a BoundsC,
    pub view: &'a ProgramView,
    pub layout: LayoutObservation<'a>,
    _sealed: (),
}
#[derive(Clone, Copy)]
pub struct LayoutObservation<'a> {
    pub layout_version: &'a (String, String),
    pub field_id: Option<&'a (String, String)>,
    pub widths: Option<WidthTableObservation<'a>>,
    pub scalar_layout_id: Option<&'a (String, String)>,
    pub bounds: &'a BoundsC,
    pub capacities: &'a LayoutCapacitiesC,
    pub boundary_capacities_present: bool,
    _sealed: (),
}
#[derive(Clone, Copy)]
pub struct WidthTableObservation<'a> {
    pub index_widths: &'a [(String, u32)],
    pub counter_widths: &'a [(String, u32)],
    pub record_widths: &'a [(String, u32)],
    _sealed: (),
}
#[derive(Clone, Copy)]
pub struct ImageObservation<'a> {
    pub run_id: &'a RunIdC,
    pub control: &'a ControlC,
    pub control_origin: usize,
    pub env_root: EnvRootC,
    pub stack_limit: usize,
    pub outcome: &'a OutcomeC,
    pub initial_fuel: u128,
    pub remaining_work: u128,
    pub consumed_work: u128,
    pub helper_work: u128,
    pub helper_visits: u128,
    pub reference_cursor: u128,
    pub alloc: &'a AllocCounters,
    pub active: &'a ActiveLengths,
    pub helper: Option<&'a HelperControlC>,
    pub result: Option<&'a HelperResultC>,
    pub seg_base: SegBase,
    pub data: &'a [RawDataRow],
    pub data_fields: &'a [RefC],
    pub env: &'a [EnvCellC],
    pub frames: &'a [FrameRowC],
    pub tasks: &'a [HelperTaskC],
    pub value_slots: &'a [RefC],
    _sealed: (),
}
// Normative registers depth/taskLive, derived exactly as the owner's
// MachineImage::depth/task_live (shared.rs): frames/tasks lengths are the
// sole active authority. No stored count, guard or policy is added; the
// slices borrow img.frames/img.tasks whole, so the values are equal.
impl ImageObservation<'_> {
    pub fn depth(&self) -> usize { self.frames.len() }
    pub fn task_live(&self) -> usize { self.tasks.len() }
}

pub fn observe_bound_image<'a>(admitted: &'a Admitted, img: &'a MachineImage)
    -> Result<HostObservation<'a>, HostViolation> {
    // AB first: a mismatch returns the SI-01 HostViolation before any read below.
    let pre = PreImage::bind(admitted, img)?;
    // reg.segBase: only the owner helper over the bound pair; Err refuses the record.
    let seg_base = implied_seg_base(&pre)?;
    let PreImage { admitted, img } = pre;
    let body: &'a AdmittedBody = &admitted.body;
    let l = &body.layout;
    let widths = l.widths.as_ref().map(|w| WidthTableObservation {
        index_widths: w.index_widths.as_slice(), counter_widths: w.counter_widths.as_slice(),
        record_widths: w.record_widths.as_slice(), _sealed: () });
    let layout = LayoutObservation { layout_version: &l.layout_version,
        field_id: l.field_id.as_ref(), widths, scalar_layout_id: l.scalar_layout_id.as_ref(),
        bounds: &l.bounds, capacities: &l.capacities,
        boundary_capacities_present: l.boundary_capacities_present, _sealed: () };
    // img.admission is the same body as `admitted` (Arc::ptr_eq above); the
    // handle itself is a host address and is not observed.
    let image = ImageObservation { run_id: &*img.run_id, control: &img.control,
        control_origin: img.control_origin, env_root: img.env_root,
        stack_limit: img.stack_limit, outcome: &img.outcome,
        initial_fuel: img.initial_fuel, remaining_work: img.remaining_work,
        consumed_work: img.consumed_work, helper_work: img.helper_work,
        helper_visits: img.helper_visits, reference_cursor: img.reference_cursor,
        alloc: &img.alloc, active: &img.active,
        helper: img.helper.as_ref(), result: img.result.as_ref(), seg_base,
        data: img.data.as_slice(), data_fields: img.data_fields.as_slice(),
        env: img.env.as_slice(), frames: img.frames.as_slice(),
        tasks: img.tasks.as_slice(), value_slots: img.value_slots.as_slice(), _sealed: () };
    Ok(HostObservation { admission: AdmissionObservation { identity: &body.identity,
        bounds: &body.bounds, view: &body.view, layout, _sealed: () }, image, _sealed: () })
}
