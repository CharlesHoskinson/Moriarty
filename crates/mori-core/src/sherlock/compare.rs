//! First-difference and continuity positions for complete typed A observations.
//!
//! Pairwise `first_difference` compares supplied observations. The separate
//! `first_continuity_difference` checks complete state edges and total charge.
//! Neither evaluates rules, authenticates source metadata, or validates proofs.
//! For pairwise comparison the caller supplies two present traces on the declared
//! `sherlock-k-a/1` common domain. An empty record slice is a present zero-step
//! observation; it is not an absent K, Quint, Lean, Clean, or handwritten layer.
//! Missing adapters and layers must be reported separately by their consumer.
//!
//! No K execution or cross-profile clock alignment is provided. Pairwise
//! comparison reports different profile/source declarations before states. Metadata
//! equality alone does not establish that a declaration is true. This API does
//! not implement Candidate B or claim other profiles have the same semantics.
//! Structural comparison inherits A's unbounded host time and recursive data
//! traversal; no new host-resource bound or untrusted consumer is introduced.

use super::reference_types::{Snapshot, StepRecord};

/// Declared semantic clock and pinned source identity, not producer identity.
/// For the frozen K A observations use `PROFILE` and `K_SOURCE_SHA256`.
/// This comparator neither hashes sources nor parses diagnostic display text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TraceIdentity<'a> {
    pub profile: &'a str,
    pub source_sha256: &'a str,
}

/// Borrowed complete observations, retaining every original typed value.
/// Both an initial state and a record slice are required; absence is not a match.
#[derive(Clone, Copy, Debug)]
pub struct TraceView<'a> {
    pub identity: TraceIdentity<'a>,
    pub initial: &'a Snapshot,
    pub records: &'a [StepRecord],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdentityField {
    Profile,
    SourceSha256,
}

/// Complete snapshot fields, in deterministic comparison order.
/// Program equality includes entry, every function/parameter/body, and ordering;
/// frames include their complete saved environments and values, top first.
/// Depth is represented by the complete ordered frames, not a separate counter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnapshotField {
    Program,
    Control,
    Environment,
    Frames,
    StackLimit,
    RemainingFuel,
    InitialFuel,
    Outcome,
}

/// Record fields, in deterministic comparison order. Before/after fields follow
/// the ordering of `SnapshotField`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecordField {
    RuleStartLine,
    Charge,
    Before(SnapshotField),
    After(SnapshotField),
}

/// Position of the earliest unequal observation. Indices are zero based.
/// Retrieve raw values through the original `TraceView` at this position; no
/// display string, serialized codec, copied state, or erased outcome is returned.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraceDifference {
    Identity(IdentityField),
    Initial(SnapshotField),
    Record {
        index: usize,
        field: RecordField,
    },
    /// The shorter slice ended after this many equal complete records.
    Length {
        common_records: usize,
        left_records: usize,
        right_records: usize,
    },
}

/// Compare identity (profile, source), initial snapshot, then common records in
/// ascending index order (rule, charge, before, after), then record lengths.
/// Snapshot field ordering is documented on `SnapshotField`. Equality is typed
/// structural equality, including through `Arc`, rather than pointer identity.
///
/// `None` means only that both supplied observations agree completely. It says
/// nothing about validity, source authenticity, missing layers, or proofs.
#[must_use]
pub fn first_difference(left: TraceView<'_>, right: TraceView<'_>) -> Option<TraceDifference> {
    if left.identity.profile != right.identity.profile {
        return Some(TraceDifference::Identity(IdentityField::Profile));
    }
    if left.identity.source_sha256 != right.identity.source_sha256 {
        return Some(TraceDifference::Identity(IdentityField::SourceSha256));
    }
    if let Some(field) = snapshot_difference(left.initial, right.initial) {
        return Some(TraceDifference::Initial(field));
    }
    for (index, (left_record, right_record)) in left.records.iter().zip(right.records).enumerate() {
        let field = if left_record.rule_start_line != right_record.rule_start_line {
            Some(RecordField::RuleStartLine)
        } else if left_record.charge != right_record.charge {
            Some(RecordField::Charge)
        } else if let Some(field) = snapshot_difference(&left_record.before, &right_record.before) {
            Some(RecordField::Before(field))
        } else {
            snapshot_difference(&left_record.after, &right_record.after).map(RecordField::After)
        };
        if let Some(field) = field {
            return Some(TraceDifference::Record { index, field });
        }
    }
    if left.records.len() != right.records.len() {
        return Some(TraceDifference::Length {
            common_records: left.records.len().min(right.records.len()),
            left_records: left.records.len(),
            right_records: right.records.len(),
        });
    }
    None
}

fn snapshot_difference(left: &Snapshot, right: &Snapshot) -> Option<SnapshotField> {
    if left.program.as_ref() != right.program.as_ref() {
        Some(SnapshotField::Program)
    } else if left.control != right.control {
        Some(SnapshotField::Control)
    } else if left.environment != right.environment {
        Some(SnapshotField::Environment)
    } else if left.frames != right.frames {
        Some(SnapshotField::Frames)
    } else if left.stack_limit != right.stack_limit {
        Some(SnapshotField::StackLimit)
    } else if left.remaining_fuel != right.remaining_fuel {
        Some(SnapshotField::RemainingFuel)
    } else if left.initial_fuel != right.initial_fuel {
        Some(SnapshotField::InitialFuel)
    } else if left.outcome != right.outcome {
        Some(SnapshotField::Outcome)
    } else {
        None
    }
}

/// Complete-state continuity edge. Record indices are zero based.
/// Retrieve both raw states from the original trace and returned snapshot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContinuityBoundary {
    /// `trace.initial` versus `trace.records[0].before`.
    InitialToFirst,
    /// `records[previous].after` versus `records[next].before`.
    BetweenRecords { previous: usize, next: usize },
    /// `records[last].after` versus the independently supplied returned state.
    LastToReturned { last: usize },
    /// A present empty record slice: `trace.initial` versus returned state.
    EmptyToReturned,
}

/// First complete-state boundary or aggregate charge-accounting mismatch.
/// Raw observations remain caller-owned; no values are erased or normalized.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContinuityDifference {
    State {
        boundary: ContinuityBoundary,
        field: SnapshotField,
    },
    /// Returned remaining fuel exceeds initial remaining fuel.
    UndefinedFuelDelta,
    /// Checked addition of this record's charge would exceed u128.
    ChargeOverflow { index: usize },
    ChargeSumMismatch {
        recorded: u128,
        endpoint_delta: u128,
    },
}

/// Check initial/first, adjacent after/before edges in ascending record order,
/// then last/returned. Empty records compare initial/returned, without inferring
/// terminality. All eight complete fields use the existing `SnapshotField` order.
/// State differences precede aggregate accounting: checked endpoint subtraction,
/// checked charge summation, then exact total equality. No counters are clamped.
///
/// `None` means these supplied boundaries and aggregate counters agree only.
/// It does not validate a transition, rule line, individual rule charge, outcome,
/// profile declaration, source authenticity, missing layer, or proof. In
/// particular, a matching pending prefix is permitted. Structural comparison
/// retains the A profile's unbounded host traversal; no new resource policy or
/// public untrusted-input admission is provided.
#[must_use]
pub fn first_continuity_difference(
    trace: TraceView<'_>,
    returned: &Snapshot,
) -> Option<ContinuityDifference> {
    if let Some((first, rest)) = trace.records.split_first() {
        if let Some(field) = snapshot_difference(trace.initial, &first.before) {
            return Some(ContinuityDifference::State {
                boundary: ContinuityBoundary::InitialToFirst,
                field,
            });
        }
        let mut previous = first;
        for (index, next) in rest.iter().enumerate() {
            if let Some(field) = snapshot_difference(&previous.after, &next.before) {
                return Some(ContinuityDifference::State {
                    boundary: ContinuityBoundary::BetweenRecords {
                        previous: index,
                        next: index + 1,
                    },
                    field,
                });
            }
            previous = next;
        }
        if let Some(field) = snapshot_difference(&previous.after, returned) {
            return Some(ContinuityDifference::State {
                boundary: ContinuityBoundary::LastToReturned {
                    last: trace.records.len() - 1,
                },
                field,
            });
        }
    } else if let Some(field) = snapshot_difference(trace.initial, returned) {
        return Some(ContinuityDifference::State {
            boundary: ContinuityBoundary::EmptyToReturned,
            field,
        });
    }

    let Some(endpoint_delta) = trace
        .initial
        .remaining_fuel
        .checked_sub(returned.remaining_fuel)
    else {
        return Some(ContinuityDifference::UndefinedFuelDelta);
    };
    let mut recorded = 0u128;
    for (index, record) in trace.records.iter().enumerate() {
        let Some(sum) = recorded.checked_add(u128::from(record.charge)) else {
            return Some(ContinuityDifference::ChargeOverflow { index });
        };
        recorded = sum;
    }
    if recorded != endpoint_delta {
        return Some(ContinuityDifference::ChargeSumMismatch {
            recorded,
            endpoint_delta,
        });
    }
    None
}

#[cfg(test)]
#[path = "trace_continuity_tests.rs"]
mod trace_continuity_tests;
