//! Exact pairwise comparison of supplied complete B observations.
//!
//! Initial state and every before/raw-event/after value use their existing
//! typed equality, including admission metadata and raw arena/event fields.
//! This same-profile correspondence predicate does not check continuity,
//! transition validity, source authenticity or terminality. No owner is called.

use super::complete_observation::{CompleteStepRecord, HostSnapshot};
use super::shared::StepEventC;

/// The first unequal location, borrowing the original supplied values.
#[derive(Debug)]
pub enum TraceDifference<'a> {
    Initial {
        expected: &'a HostSnapshot,
        observed: &'a HostSnapshot,
    },
    RecordBefore {
        index: usize,
        expected: &'a HostSnapshot,
        observed: &'a HostSnapshot,
    },
    RecordEvent {
        index: usize,
        expected: &'a StepEventC,
        observed: &'a StepEventC,
    },
    RecordAfter {
        index: usize,
        expected: &'a HostSnapshot,
        observed: &'a HostSnapshot,
    },
    RecordLength {
        common_records: usize,
        expected_records: usize,
        observed_records: usize,
    },
}

/// Compare two present complete traces, retaining profile-specific raw values.
///
/// Order: initial; ascending common record index, before/event/after; length.
/// A strict prefix differs at length after its equal common records. An empty
/// slice is a present zero-event trace; the caller represents missing evidence.
/// Equal traces need not be contiguous or terminal. No returned-state value is
/// inferred; compare returned state separately and use `first_discontinuity`
/// for initial/before/after/returned boundary continuity.
pub fn first_difference<'a>(
    expected_initial: &'a HostSnapshot,
    expected_records: &'a [CompleteStepRecord],
    observed_initial: &'a HostSnapshot,
    observed_records: &'a [CompleteStepRecord],
) -> Option<TraceDifference<'a>> {
    if expected_initial != observed_initial {
        return Some(TraceDifference::Initial {
            expected: expected_initial,
            observed: observed_initial,
        });
    }
    for (index, (expected, observed)) in expected_records.iter().zip(observed_records).enumerate() {
        if expected.before != observed.before {
            return Some(TraceDifference::RecordBefore {
                index,
                expected: &expected.before,
                observed: &observed.before,
            });
        }
        if expected.event != observed.event {
            return Some(TraceDifference::RecordEvent {
                index,
                expected: &expected.event,
                observed: &observed.event,
            });
        }
        if expected.after != observed.after {
            return Some(TraceDifference::RecordAfter {
                index,
                expected: &expected.after,
                observed: &observed.after,
            });
        }
    }
    (expected_records.len() != observed_records.len()).then_some(TraceDifference::RecordLength {
        common_records: expected_records.len().min(observed_records.len()),
        expected_records: expected_records.len(),
        observed_records: observed_records.len(),
    })
}
