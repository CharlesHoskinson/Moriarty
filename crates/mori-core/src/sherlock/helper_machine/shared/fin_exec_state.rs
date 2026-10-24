//! Proposed financial execution-state invariant (MU1). Not adopted.
//!
//! `FinImageState::snapshot`/`core_snapshot`/`committed` remain the committed
//! boundary. `exec.tentative_snapshot` is the only working snapshot; it equals
//! the committed snapshot at genesis and may diverge only inside a segment.
//! `committed` must always equal the selected plan's
//! `committed_before[exec.segment_index]` until a later segment-commit unit
//! advances both together.

use super::FinImageState;
use super::fin_carrier::{
    FinActionPlan, FinBoundSource, FinEffectC, FinInputBinding, FinPlan, FinPreparedStagedSource,
    FinSnapshot, check_fin_snapshot,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FinExecState {
    pub statement_cursor: usize,
    pub segment_index: usize,
    pub attempted_in_segment: u128,
    pub tentative_snapshot: FinSnapshot,
    pub tentative_effects: Vec<FinEffectC>,
    pub retained_env: Vec<FinInputBinding>,
}

pub fn fin_exec_genesis(snapshot: &FinSnapshot, inputs: &[FinInputBinding]) -> FinExecState {
    FinExecState {
        statement_cursor: 0,
        segment_index: 0,
        attempted_in_segment: 0,
        tentative_snapshot: snapshot.clone(),
        tentative_effects: Vec::new(),
        retained_env: inputs.to_vec(),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FinExecInvariantError {
    SnapshotSourceMismatch,
    SnapshotShape,
    CoreSnapshotMismatch,
    PlanActionMissing {
        action_index: usize,
    },
    PlanActionMismatch {
        action_index: usize,
        plan_name: String,
        source_name: String,
    },
    PlanLengthMismatch {
        segment_src_cost_len: usize,
        committed_before_len: usize,
    },
    PlanPrefixMismatch {
        segment_index: usize,
        expected: u128,
        actual: u128,
    },
    PlanTotalMismatch {
        expected: u128,
        actual: u128,
    },
    PlanOverflow {
        segment_index: usize,
    },
    PlanEmptySegments,
    SegmentOutOfRange {
        segment_index: usize,
        segment_count: usize,
    },
    CommittedMismatch {
        expected: u128,
        actual: u128,
    },
    AttemptedExceedsSegment {
        attempted: u128,
        segment_cost: u128,
    },
    CursorOutOfRange {
        cursor: usize,
        statement_count: usize,
    },
    EffectsAheadOfCursor,
    DuplicateRetainedName(String),
    RetainedEnvPrefixMismatch,
    GenesisTentativeNotCommitted,
    GenesisCursorNotZero {
        cursor: usize,
    },
    GenesisSegmentNotZero {
        segment_index: usize,
    },
    GenesisAttemptedNotZero {
        attempted: u128,
    },
    GenesisTentativeEffectsNotEmpty,
    GenesisRetainedEnvMismatch,
    GenesisCommittedNotZero {
        committed: u128,
    },
}

/// Total validation of one source-derived action plan. This is the exact
/// checked precondition formerly missing before `committed_before` indexing:
/// lengths are compared before either vector is indexed, prefix sums are
/// checked with checked arithmetic, and the total is compared last.
pub fn validate_fin_action_plan(action: &FinActionPlan) -> Result<(), FinExecInvariantError> {
    if action.segment_src_cost.len() != action.committed_before.len() {
        return Err(FinExecInvariantError::PlanLengthMismatch {
            segment_src_cost_len: action.segment_src_cost.len(),
            committed_before_len: action.committed_before.len(),
        });
    }
    if action.segment_src_cost.is_empty() {
        return Err(FinExecInvariantError::PlanEmptySegments);
    }
    let mut acc: u128 = 0;
    for (segment_index, cost) in action.segment_src_cost.iter().enumerate() {
        let actual = action.committed_before[segment_index];
        if actual != acc {
            return Err(FinExecInvariantError::PlanPrefixMismatch {
                segment_index,
                expected: acc,
                actual,
            });
        }
        acc = acc
            .checked_add(*cost)
            .ok_or(FinExecInvariantError::PlanOverflow { segment_index })?;
    }
    if acc != action.plan_total {
        return Err(FinExecInvariantError::PlanTotalMismatch {
            expected: acc,
            actual: action.plan_total,
        });
    }
    Ok(())
}

fn validate_whole_plan(plan: &FinPlan) -> Result<(), FinExecInvariantError> {
    for action in &plan.actions {
        validate_fin_action_plan(action)?;
    }
    Ok(())
}

/// All-state financial execution invariant.
///
/// The plan, bound source, selected source action, and statement count all
/// come from the one sealed prepared source carried by financial admission.
/// No caller-authored plan or caller-stated statement count is trusted.
/// Tentative snapshot divergence is allowed here; genesis equality is a
/// separate stronger predicate below.
pub fn check_fin_exec_invariant(
    state: &FinImageState,
    prepared: &FinPreparedStagedSource,
) -> Result<(), FinExecInvariantError> {
    let bound: &FinBoundSource = prepared.bound_source();
    let plan: &FinPlan = prepared.fin_plan();

    if state.snapshot.source() != bound || state.exec.tentative_snapshot.source() != bound {
        return Err(FinExecInvariantError::SnapshotSourceMismatch);
    }
    check_fin_snapshot(bound, &state.snapshot).map_err(|_| FinExecInvariantError::SnapshotShape)?;
    check_fin_snapshot(bound, &state.exec.tentative_snapshot)
        .map_err(|_| FinExecInvariantError::SnapshotShape)?;

    let core = state
        .snapshot
        .to_core(bound)
        .map_err(|_| FinExecInvariantError::SnapshotShape)?;
    if core != state.core_snapshot {
        return Err(FinExecInvariantError::CoreSnapshotMismatch);
    }

    validate_whole_plan(plan)?;

    let Some(source_action) = prepared.program().actions.get(state.action_index) else {
        return Err(FinExecInvariantError::PlanActionMissing {
            action_index: state.action_index,
        });
    };
    let Some(action) = plan.actions.get(state.action_index) else {
        return Err(FinExecInvariantError::PlanActionMissing {
            action_index: state.action_index,
        });
    };
    let Some(bound_action) = bound.actions().get(state.action_index) else {
        return Err(FinExecInvariantError::PlanActionMissing {
            action_index: state.action_index,
        });
    };
    if action.name != source_action.name || action.name != bound_action.text {
        return Err(FinExecInvariantError::PlanActionMismatch {
            action_index: state.action_index,
            plan_name: action.name.clone(),
            source_name: source_action.name.clone(),
        });
    }

    let statement_count = source_action.statements.len();
    let segment_count = action.segment_src_cost.len();
    let Some(expected) = action.committed_before.get(state.exec.segment_index) else {
        return Err(FinExecInvariantError::SegmentOutOfRange {
            segment_index: state.exec.segment_index,
            segment_count,
        });
    };
    if state.committed.0 != *expected {
        return Err(FinExecInvariantError::CommittedMismatch {
            expected: *expected,
            actual: state.committed.0,
        });
    }
    let Some(segment_cost) = action.segment_src_cost.get(state.exec.segment_index) else {
        return Err(FinExecInvariantError::SegmentOutOfRange {
            segment_index: state.exec.segment_index,
            segment_count,
        });
    };
    if state.exec.attempted_in_segment > *segment_cost {
        return Err(FinExecInvariantError::AttemptedExceedsSegment {
            attempted: state.exec.attempted_in_segment,
            segment_cost: *segment_cost,
        });
    }
    if state.exec.statement_cursor > statement_count {
        return Err(FinExecInvariantError::CursorOutOfRange {
            cursor: state.exec.statement_cursor,
            statement_count,
        });
    }
    if state.exec.tentative_effects.len() > state.exec.statement_cursor {
        return Err(FinExecInvariantError::EffectsAheadOfCursor);
    }

    // Genesis inputs remain the retained-environment prefix in every later
    // state delivered by this family of units. Later lets, captures, and
    // event binders may only append after them.
    if state.exec.retained_env.len() < state.inputs.len()
        || state.exec.retained_env[..state.inputs.len()] != state.inputs[..]
    {
        return Err(FinExecInvariantError::RetainedEnvPrefixMismatch);
    }
    for (i, binding) in state.exec.retained_env.iter().enumerate() {
        if state.exec.retained_env[..i]
            .iter()
            .any(|b| b.name == binding.name)
        {
            return Err(FinExecInvariantError::DuplicateRetainedName(
                binding.name.clone(),
            ));
        }
    }
    Ok(())
}

/// Genesis-only strengthening of the all-state invariant.
///
/// Genesis-only conjuncts are checked first, in the order segment,
/// cursor, attempted, committed, tentative snapshot, tentative effects,
/// retained environment, and the unchanged all-state invariant is then
/// required as well. A state is accepted only if both predicates pass;
/// checking the genesis conjuncts first changes only which exact error
/// is reported for a state that violates both. It makes every genesis
/// conjunct reachable: under the former order (all-state first, cursor
/// before effects) `GenesisTentativeEffectsNotEmpty` could never be
/// reported, because an effect at cursor 0 already failed the all-state
/// `EffectsAheadOfCursor` check and an effect at cursor >= 1 already
/// failed `GenesisCursorNotZero`.
///
/// This does not replace the all-state predicate and must not be imposed
/// on an in-segment state merely because its cursor is at a segment
/// boundary.
pub fn check_fin_exec_genesis_invariant(
    state: &FinImageState,
    prepared: &FinPreparedStagedSource,
) -> Result<(), FinExecInvariantError> {
    if state.exec.segment_index != 0 {
        return Err(FinExecInvariantError::GenesisSegmentNotZero {
            segment_index: state.exec.segment_index,
        });
    }
    if state.exec.statement_cursor != 0 {
        return Err(FinExecInvariantError::GenesisCursorNotZero {
            cursor: state.exec.statement_cursor,
        });
    }
    if state.exec.attempted_in_segment != 0 {
        return Err(FinExecInvariantError::GenesisAttemptedNotZero {
            attempted: state.exec.attempted_in_segment,
        });
    }
    if state.committed.0 != 0 {
        return Err(FinExecInvariantError::GenesisCommittedNotZero {
            committed: state.committed.0,
        });
    }
    if state.exec.tentative_snapshot != state.snapshot {
        return Err(FinExecInvariantError::GenesisTentativeNotCommitted);
    }
    if !state.exec.tentative_effects.is_empty() {
        return Err(FinExecInvariantError::GenesisTentativeEffectsNotEmpty);
    }
    if state.exec.retained_env != state.inputs {
        return Err(FinExecInvariantError::GenesisRetainedEnvMismatch);
    }
    check_fin_exec_invariant(state, prepared)
}
