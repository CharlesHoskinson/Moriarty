//! Borrowed diagnostic observations of existing Core/2 stage results.
//!
//! This module preserves the complete existing values and their order. It does
//! not execute statements, lower financial control, compute account aggregates,
//! or define a serialization format. `Step` retains its source statement index,
//! statement, optional value, complete snapshot and ordered effects. Keeping
//! `Snapshot` whole also preserves zero liabilities and unaffected balances;
//! the supplied `residual` list is a separate, unchanged observation.
//!
//! A successful segment exposes its own pre/post state and effects. A waiting
//! continuation holds the current committed boundary, not an effect history.
//! Rejection exposes no tentative snapshot, effects or replacement continuation.
//! For resume diagnostics, `ResumeFinancialObservation` retains the caller's
//! continuation, event and CLI flags separately from the returned observation.
//! The staged program argument is not represented by this input view. All
//! `committed` counters here report logical work, not financial value; an early
//! resume refusal can report an unvalidated caller-supplied counter.
//!
//! These views provide no Sherlock relation/compiler correspondence or target
//! phase mapping. The existing stage consumer remains responsible for retaining
//! `crate::stage::STAGE_LABELS` on its public output.

use crate::stage::{
    Continuation, PendingFrame, StageEvent, StageOutcome, StageRejection, StageSegment,
};
use crate::{Effect, Error, Execution, Liability, Snapshot, Step, UInt};

/// Every field of one existing execution, borrowed without reduction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutionObservation<'a> {
    pub semantics: &'a str,
    pub action: &'a str,
    pub pre: &'a Snapshot,
    pub post: &'a Snapshot,
    pub steps: &'a [Step],
    pub effects: &'a [Effect],
}

/// One published successful segment, including both pending frames.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SegmentObservation<'a> {
    pub action: &'a str,
    pub frame_pre: &'a PendingFrame,
    pub frame_post: &'a PendingFrame,
    pub execution: ExecutionObservation<'a>,
}

/// The three cases already represented by the public stage result type.
///
/// No absent segment is replaced by an empty execution. In particular a false
/// await guard retains `segment: None`, while a successful zero-effect segment
/// remains `Some` and keeps its steps, frames and complete state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinancialObservation<'a> {
    Waiting {
        advanced: bool,
        segment: Option<SegmentObservation<'a>>,
        continuation: &'a Continuation,
        attempted: UInt,
        residual: &'a [Liability],
    },
    Complete {
        segment: SegmentObservation<'a>,
        committed: UInt,
        attempted: UInt,
        residual: &'a [Liability],
    },
    Rejected {
        error: &'a Error,
        attempted: UInt,
        committed: UInt,
    },
}

/// Captured continuation, event and CLI flags, separate from the returned result.
///
/// `input` is the caller-supplied continuation. This view does not establish
/// that `outcome` was produced from it. Callers capture the actual arguments
/// represented here and the returned result at the existing `stage::resume`
/// call site. The staged program argument and its actual identity are omitted;
/// the continuation's claimed hashes do not substitute for that argument.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResumeFinancialObservation<'a> {
    pub input: &'a Continuation,
    pub event: &'a StageEvent,
    pub cli_action: Option<&'a str>,
    pub cli_cap: Option<UInt>,
    pub outcome: FinancialObservation<'a>,
}

/// Projects an execution without recomputing state, effects or step values.
pub fn project_execution(execution: &Execution) -> ExecutionObservation<'_> {
    let Execution { semantics, action, pre, post, steps, effects } = execution;
    ExecutionObservation { semantics, action, pre, post, steps, effects }
}

/// Projects both frames and the complete segment execution.
pub fn project_segment(segment: &StageSegment) -> SegmentObservation<'_> {
    let StageSegment { action, frame_pre, frame_post, execution } = segment;
    SegmentObservation {
        action,
        frame_pre,
        frame_post,
        execution: project_execution(execution),
    }
}

/// Projects an existing start or resume result. No evaluator is invoked.
///
/// Exhaustive field patterns make additions to these result structures require
/// an explicit mapping update when this module is compiled with them.
pub fn project_stage_result(
    result: &Result<StageOutcome, StageRejection>,
) -> FinancialObservation<'_> {
    match result {
        Ok(StageOutcome::Waiting {
            advanced, segment, continuation, attempted, residual,
        }) => FinancialObservation::Waiting {
            advanced: *advanced,
            segment: segment.as_ref().map(project_segment),
            continuation,
            attempted: *attempted,
            residual,
        },
        Ok(StageOutcome::Complete { segment, committed, attempted, residual }) => {
            FinancialObservation::Complete {
                segment: project_segment(segment),
                committed: *committed,
                attempted: *attempted,
                residual,
            }
        }
        Err(StageRejection { error, attempted, committed }) => {
            FinancialObservation::Rejected {
                error,
                attempted: *attempted,
                committed: *committed,
            }
        }
    }
}

/// Retains the captured continuation, event and CLI flags with the full result.
pub fn project_resume_result<'a>(
    input: &'a Continuation,
    event: &'a StageEvent,
    cli_action: Option<&'a str>,
    cli_cap: Option<UInt>,
    result: &'a Result<StageOutcome, StageRejection>,
) -> ResumeFinancialObservation<'a> {
    ResumeFinancialObservation {
        input, event, cli_action, cli_cap, outcome: project_stage_result(result),
    }
}

impl<'a> FinancialObservation<'a> {
    /// The successful invocation's published boundary, if one was returned.
    /// A rejection has no returned financial boundary.
    pub fn committed_boundary(&self) -> Option<&'a Snapshot> {
        match self {
            Self::Waiting { continuation, .. } => Some(&continuation.boundary),
            Self::Complete { segment, .. } => Some(segment.execution.post),
            Self::Rejected { .. } => None,
        }
    }

    /// Only this invocation's published segment; never a prefix effect history.
    pub fn successful_segment(&self) -> Option<SegmentObservation<'a>> {
        match self {
            Self::Waiting { segment, .. } => *segment,
            Self::Complete { segment, .. } => Some(*segment),
            Self::Rejected { .. } => None,
        }
    }

    /// The exact `committed` logical work counter reported by Core/2.
    ///
    /// On early resume refusals this can be an unvalidated input echo; start
    /// refusals and CLI failures before work can report zero. This projection
    /// does not establish admission or independently validate the counter.
    pub fn committed_logical_work(&self) -> UInt {
        match self {
            Self::Waiting { continuation, .. } => continuation.logical.committed,
            Self::Complete { committed, .. } | Self::Rejected { committed, .. } => *committed,
        }
    }
}

impl<'a> ResumeFinancialObservation<'a> {
    /// The caller-supplied, unvalidated boundary, including on rejection.
    ///
    /// It is unauthenticated input and represents a locally retained prefix
    /// only when admission passed. This projection cannot infer that passage
    /// and does not substitute this input for a missing returned boundary.
    pub fn input_boundary(&self) -> &'a Snapshot {
        &self.input.boundary
    }
}

/// Full ordered diagnostic comparison against an independently frozen view.
///
/// For an independent oracle comparison, the expected view must be authored
/// independently before candidate execution. This includes statement ExprIds
/// and continuation hashes: their independent literal IR/plan derivation must
/// be recorded separately, without parser/evaluator/candidate output. Existing
/// Core/1 Step/Execution JSON decoding cannot bind an Await step; a separate
/// test-only typed literal binding remains required for Core/2 oracle fixtures.
/// Equality compares complete steps/effects/snapshots, frames, continuation
/// fields, residual rows, work counters and errors, including present zero
/// liabilities. Equality alone does not establish expected-value independence.
/// This helper does not derive or certify any expectation. The present normal
/// oracle task makes no independently specified Error-case comparison claim.
pub fn financial_observation_matches(
    actual: &Result<StageOutcome, StageRejection>,
    expected: &FinancialObservation<'_>,
) -> bool {
    project_stage_result(actual) == *expected
}

/// Full comparison of the captured resume view, including both CLI flags.
/// The staged program argument is not part of this view. Expected-value
/// independence and Core/2 typed fixture binding have the requirements above.
pub fn resume_financial_observation_matches(
    actual: &ResumeFinancialObservation<'_>,
    expected: &ResumeFinancialObservation<'_>,
) -> bool {
    actual == expected
}
