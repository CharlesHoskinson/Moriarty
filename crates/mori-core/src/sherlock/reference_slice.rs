//! Private host grouping proposal for the existing frozen K-A reference events.
//!
//! This ordinary host baseline groups genuine Machine::step records; it does
//! not implement a new transition relation, helper clock or optimized machine.
//! Full snapshots and record order remain observable. A technical cut records
//! a grouping boundary only: it creates no financial commit, effect, suspension
//! or new machine Outcome. Preparation and full observation cloning are unmetered
//! host work. This is no bounded B schema, native payload or capacity claim.

#![cfg(test)]

use super::reference_types::*;
use super::Machine;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SliceBoundaryKind {
    Terminal,
    TechnicalCut,
}

/// Position counts retained records, so zero names the slice's start state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct SliceBoundary {
    pub(super) position: usize,
    pub(super) kind: SliceBoundaryKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum SliceStop {
    Outcome(Outcome),
    TechnicalCut,
}

/// Host-only subset proposal of interfaces.md's ReferenceSlice. It carries only
/// fields available in the existing K-A machine; source suspension and proposed
/// B identities/layouts/helpers remain future work. It freezes no shared API.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ReferenceSlice {
    pub(super) profile_id: &'static str,
    pub(super) start: Snapshot,
    pub(super) records: Vec<StepRecord>,
    /// Length records.len()+1, starts zero, accumulates each actual record.charge.
    pub(super) prefix_work: Vec<u128>,
    pub(super) boundaries: Vec<SliceBoundary>,
    pub(super) end: Snapshot,
    pub(super) stop: SliceStop,
}

/// Retain at most m reference events, including free publication as an event.
/// Normal comparison budgets are 1/2/4/8. Other host usize budgets merely choose
/// grouping length; no native width or resource policy is selected here.
///
/// Zero is an explicit observation-only cut: pending input yields identical
/// start/end, no records, prefix_work=[0], and a TechnicalCut at position zero.
/// Already published outcomes, including with m=0, yield an empty terminal
/// slice with the unchanged Outcome and a Terminal boundary at position zero.
///
/// Each next step is attempted in order on the live ordinary Machine, with its
/// own fuel/cap/publication behavior. There is no stepping ahead or replay.
/// Reaching a pending terminal control at the event budget leaves publication
/// for the next group; when the budget includes publication, its zero charge
/// is retained and the group stops immediately at that first published outcome.
pub(super) fn advance_m(machine: &mut Machine, m: usize) -> ReferenceSlice {
    let start = machine.snapshot();
    let mut records = Vec::new();
    let mut prefix_work = vec![0];

    if start.outcome == Outcome::Pending {
        for _ in 0..m {
            let Some(record) = machine.step() else {
                break;
            };
            let cumulative = prefix_work.last().copied().expect("initial zero work")
                + u128::from(record.charge);
            let terminal = record.after.outcome != Outcome::Pending;
            prefix_work.push(cumulative);
            records.push(record);
            if terminal {
                break;
            }
        }
    }

    let end = machine.snapshot();
    let (stop, kind) = if end.outcome == Outcome::Pending {
        (SliceStop::TechnicalCut, SliceBoundaryKind::TechnicalCut)
    } else {
        (SliceStop::Outcome(end.outcome.clone()), SliceBoundaryKind::Terminal)
    };
    let boundary = SliceBoundary { position: records.len(), kind };
    ReferenceSlice {
        profile_id: PROFILE,
        start,
        records,
        prefix_work,
        boundaries: vec![boundary],
        end,
        stop,
    }
}

#[cfg(test)]
#[path = "reference_slice_tests.rs"]
mod tests;
