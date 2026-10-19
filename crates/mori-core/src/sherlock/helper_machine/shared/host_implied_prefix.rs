//! helper_machine::shared::host_implied_prefix — PROPOSED, not installed, not adopted.
//! Placement: crates/mori-core/src/sherlock/helper_machine/shared/host_implied_prefix.rs
//!   (child of `shared`; reads private PreImage/Admitted fields, constructs
//!    no PreImage, Admitted, MachineImage, OwnerGuards or ValidatedPlan).
//! Sole source of the R4 `reg.segBase` observation for qualified HOST Stage 1
//!   B-core (DedicatedSorts, finance Absent, genesis through one uncut segment).
//!   MachineImage stores no seg_base: in B-core the register is a specified
//!   constant, derived here exactly as `shared` derives depth/task_live.
//! Source (schema-supplement.json, R4):
//!   machine_image.registers reg.segBase: "BOUNDARY RESERVED: all zero in
//!     B-core (SI-05); set only by a future canonical resume";
//!   admission.genesis genesis_valid/genesis_invalid: "segBase zero";
//!   step_inv SI-05 "In B-core segBase is entirely zero", SI-23 "In B-core
//!     segBase is zero";
//!   extension_reservations reg.segBase: "nonzero or present is C-INV in B-core";
//!   row_language.effects.set: no LValue names segBase, so no row writes it;
//!   admission.admit AD-6: boundaryCapacities present is refused.
//! NOT for extension, technical cut, canonical resume or projection: a resumed
//!   image needs a STORED segBase written by that resume (boundaries.segment_base,
//!   X-10). That revision replaces this helper; it must never extend it.

use super::{ActiveLengths, AllocCounters, FinanceInput, GrammarDomain,
    HostViolation, HostViolationKind, PreImage, StepInvClauseId};

const PROFILE_HELPER_MICROSTEP: u8 = 2; // IdentityC.profile_id owner mapping in `shared`

// R4 segBase shape: {active: {data, dataFields, env, valueSlots},
//   alloc: {dataNodes, dataFields, environmentCells}, consumedWork}.
// Private fields, no Default: implied_seg_base is the only constructor.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SegBase { active: ActiveLengths, alloc: AllocCounters, consumed_work: u128 }
impl SegBase {
    pub const fn genesis_uncut() -> Self {
        Self {
            active: ActiveLengths { data: 0, data_fields: 0, env: 0, value_slots: 0 },
            alloc: AllocCounters { data_nodes: 0, data_fields: 0, environment_cells: 0 },
            consumed_work: 0,
        }
    }
    pub const fn from_parts(active: ActiveLengths, alloc: AllocCounters, consumed_work: u128) -> Self {
        Self { active, alloc, consumed_work }
    }
    pub fn active(&self) -> ActiveLengths { self.active }
    pub fn alloc(&self) -> AllocCounters { self.alloc }
    pub fn consumed_work(&self) -> u128 { self.consumed_work }
}

// Precondition: `pre` came from PreImage::bind, so AB (SI-01 host binding) passed.
// Reads the bound admission body and `pre.img.seg_base`, enforcing SI-05 prefix bounds.
pub fn implied_seg_base(pre: &PreImage<'_>) -> Result<SegBase, HostViolation> {
    let body = &pre.admitted.body;
    let domain_ok = matches!(
        body.identity.grammar_domain,
        GrammarDomain::DedicatedSorts | GrammarDomain::GenericNominalExtensionReserved
    );
    if body.identity.profile_id != PROFILE_HELPER_MICROSTEP
        || !domain_ok
        || body.identity.finance != FinanceInput::Absent
        || body.layout.boundary_capacities_present
    {
        return Err(HostViolation { kind: HostViolationKind::StepInvViolated,
            clause: Some(StepInvClauseId::SI05), rule: None,
            locator: Some("segBase implied-zero domain".to_string()) });
    }
    let sb = pre.img.seg_base;
    if sb != SegBase::genesis_uncut()
        || sb.active.data > pre.img.active.data
        || sb.active.data_fields > pre.img.active.data_fields
        || sb.active.env > pre.img.active.env
        || sb.active.value_slots > pre.img.active.value_slots
        || sb.alloc.data_nodes > pre.img.alloc.data_nodes
        || sb.alloc.data_fields > pre.img.alloc.data_fields
        || sb.alloc.environment_cells > pre.img.alloc.environment_cells
        || sb.consumed_work > pre.img.consumed_work
    {
        return Err(HostViolation { kind: HostViolationKind::StepInvViolated,
            clause: Some(StepInvClauseId::SI05), rule: None,
            locator: Some("segBase prefix invariant".to_string()) });
    }
    Ok(sb)
}
