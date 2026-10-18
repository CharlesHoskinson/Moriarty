// Declarations only, copied from the historical unregistered prior adapter.
use crate::stage::{Continuation, PendingFrame};
use crate::{Effect, Error, Liability, Snapshot, Step, UInt};
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

