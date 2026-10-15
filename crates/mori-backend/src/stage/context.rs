//! `/4` per-circuit context preimage. Domain-separated from the untouched `/3`
//! 18-element context by its own format and version strings. It commits the
//! sealed program and plan, the segment and its sites, the capture and event
//! layouts and every financial frame term. It contains no compiler identity.
use super::frame::SegmentLayout;
use super::{STAGE_CONTEXT_FORMAT, STAGED_ABI_VERSION, STAGED_ARTIFACT_FORMAT, STAGED_RELATION_VERSION};
use crate::signed_relation::{Projection, json};
use crate::{
    BalanceDescriptor, EffectDescriptor, Error, LiabilityDescriptor, LiabilityIdentity,
    MAX_LIABILITY_REGISTRY_ENTRIES, PublicBinding, Target, UNSIGNED_MODE, digest, error,
    liabilities,
};
use mori_core::LOGICAL_COST_PROFILE_2;
use mori_core::stage::{
    MAX_AWAITS_PER_ACTION, MAX_CAPTURE_SLOTS, MAX_EVENT_BINDERS, SEMANTICS_2, StagedProgram,
};

/// SHA256 of the canonical 29-element context array. As in `/3`, the
/// projection excludes every constant, so context positions 0/1 never hash
/// themselves; pending-site and padding constants are fixed by elements 18-23.
#[allow(clippy::too_many_arguments)]
pub(crate) fn stage_context_hash(
    sp: &StagedProgram,
    target: &Target,
    registry: &[LiabilityIdentity],
    l: &SegmentLayout,
    bindings: &[PublicBinding],
    effect_frame: &[EffectDescriptor],
    balance_frame: &[BalanceDescriptor],
    liability_frame: &[LiabilityDescriptor],
) -> Result<[u8; 32], Error> {
    if target.signed_policy.is_some() {
        return Err(error(
            "unsupported",
            "Staged moriarty/2 relations are unsigned; a signed Target is refused.",
        ));
    }
    let program = sp.program();
    let action = program
        .actions
        .get(l.action)
        .filter(|action| action.name == l.action_name)
        .ok_or_else(|| error("artifact", "Segment layout names no program action."))?;
    if program.semantics != SEMANTICS_2 {
        return Err(error("artifact", "Staged context requires moriarty-core/2."));
    }
    if registry != liabilities::registry(program)?.as_slice() {
        return Err(error(
            "artifact",
            "Staged context registry differs from the source registry.",
        ));
    }
    let width = u16::try_from(l.slots.len())
        .map_err(|_| error("artifact", "Capture layout exceeds the slot bound."))?;
    let projection: Vec<_> = bindings
        .iter()
        .enumerate()
        .map(|(position, b)| Projection {
            position: position as u32,
            role: &b.role,
            name: &b.name,
            ty: b.ty,
            asset: &b.asset,
        })
        .collect();
    digest(&[
        json(&STAGE_CONTEXT_FORMAT)?,
        json(&UNSIGNED_MODE)?,
        json(&STAGED_ARTIFACT_FORMAT)?,
        json(&STAGED_ABI_VERSION)?,
        json(&STAGED_RELATION_VERSION)?,
        json(&SEMANTICS_2)?,
        json(&LOGICAL_COST_PROFILE_2)?,
        json(&MAX_LIABILITY_REGISTRY_ENTRIES)?,
        json(&MAX_AWAITS_PER_ACTION)?,
        json(&MAX_EVENT_BINDERS)?,
        json(&MAX_CAPTURE_SLOTS)?,
        json(&registry)?,
        json(program)?,
        json(sp.plan())?,
        json(target)?,
        json(&action.name)?,
        json(&l.segment)?,
        json(&l.entry_site)?,
        json(&l.exit_site)?,
        json(&width)?,
        json(&l.slots)?,
        json(&l.len_in)?,
        json(&l.len_out)?,
        json(&l.event)?,
        json(&projection)?,
        json(&effect_frame)?,
        json(&balance_frame)?,
        json(&liability_frame)?,
        json(&l.replay_steps)?,
    ])
}
