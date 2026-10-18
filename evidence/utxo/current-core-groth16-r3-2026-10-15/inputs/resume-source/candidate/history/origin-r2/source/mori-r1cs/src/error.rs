//! Compile-time refusals and witness-mapping errors.
//!
//! A [`CompileError`] means no relation was produced. A [`MappingError`] means no
//! assignment was proposed; it is never an acceptance decision, which belongs to the
//! constraint system alone.
use crate::adapter::AdapterError;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompileError {
    /// Actual Core lexing, parsing, validation or stage admission refused the input.
    Core(mori_core::Error),
    /// A profile that the current Core routes but this compiler does not.
    Profile(String),
    /// `moriarty/2` was admitted by `stage::load`; the first slice does not lower it.
    StagedNotLowered(Box<crate::staged::StagedLoweringPlan>),
    UnknownAction(String),
    /// A construct that is admitted by Core but has no constraint lowering yet.
    Unsupported { construct: &'static str, reason: String },
    /// A Core bound that the represented transition would violate.
    Bound(String),
    /// The action cannot succeed for any input (a constant subexpression or predicate
    /// always refuses), so no success relation is produced.
    StaticFailure { scope: String, reason: String },
    /// Invalid identity input (for example an all-zero compiler build digest).
    Identity(String),
    /// The scalar field is too small for the exact-integer gadgets, or a circuit was
    /// requested over a field other than the one the relation was probed in.
    Field(String),
    /// Invalid caller option (frame cell, cap binding).
    Option(String),
    /// The selected statement adapter or target policy refuses the program.
    Adapter(AdapterError),
    /// Constraint synthesis failed during the compile-time probe.
    Synthesis(String),
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Core(e) => write!(f, "core refusal: {e}"),
            Self::Profile(p) => write!(f, "unsupported profile: {p}"),
            Self::StagedNotLowered(plan) => write!(
                f,
                "moriarty/2 admitted by stage::load but not lowered in this slice ({} actions planned)",
                plan.actions.len()
            ),
            Self::UnknownAction(a) => write!(f, "unknown action {a:?}"),
            Self::Unsupported { construct, reason } => write!(f, "unsupported {construct}: {reason}"),
            Self::Bound(b) => write!(f, "bound: {b}"),
            Self::StaticFailure { scope, reason } => write!(f, "action can never succeed at {scope}: {reason}"),
            Self::Identity(i) => write!(f, "identity: {i}"),
            Self::Field(m) => write!(f, "field: {m}"),
            Self::Option(m) => write!(f, "option: {m}"),
            Self::Adapter(a) => write!(f, "adapter: {a}"),
            Self::Synthesis(m) => write!(f, "synthesis: {m}"),
        }
    }
}

impl std::error::Error for CompileError {}

/// Failure to turn Core records into a typed assignment for one relation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MappingError {
    /// The reference refused the scenario; there is no success witness to propose.
    ReferenceRefused(mori_core::Error),
    /// The record does not have the relation's shape (count, kind, name, type or a
    /// non-canonical absent duty).
    Shape(String),
    /// A row outside the represented cells changed between pre and post.
    FrameChanged(String),
    /// Cap value presence differs from the relation's cap binding.
    Cap(String),
    /// The statement adapter cannot encode this relation's public inputs.
    Adapter(AdapterError),
    /// A circuit was requested over a different field than the relation was probed in.
    Field(String),
}

impl fmt::Display for MappingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReferenceRefused(e) => write!(f, "reference refused: {e}"),
            Self::Shape(m) => write!(f, "shape: {m}"),
            Self::FrameChanged(m) => write!(f, "frame changed: {m}"),
            Self::Cap(m) => write!(f, "cap: {m}"),
            Self::Adapter(a) => write!(f, "adapter: {a}"),
            Self::Field(m) => write!(f, "field: {m}"),
        }
    }
}

impl std::error::Error for MappingError {}
