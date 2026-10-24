//! Internal H1 observation adapters. No evaluator, continuity or proof checker.
//! Metadata is preserved as declared; this module does not authenticate it.
use super::limits::Locator;
use super::rows::{PreparedA, ProgramBlock};
use super::state::{AImage, DecodeMode, StateBlock, StateCapacities, StateFailure, StateResult};
use super::state_decode::decode_observed_a;
use super::state_encode::encode_observed_a;
use crate::sherlock::compare::{TraceDifference, TraceIdentity, TraceView, first_difference};
use crate::sherlock::{Outcome, Program, Snapshot, StepRecord};

// Exact crosswalk rule_line_tags order. Array index is the separate enum(39)
// tag; each tuple is (source line, exact charge). No inclusive ranges implied.
const RULE_LINES: [(usize, u8); 39] = [
    (80, 0),
    (82, 0),
    (84, 0),
    (102, 1),
    (108, 1),
    (111, 1),
    (114, 1),
    (117, 1),
    (120, 1),
    (123, 1),
    (126, 1),
    (129, 1),
    (131, 1),
    (135, 1),
    (140, 1),
    (145, 1),
    (149, 1),
    (153, 1),
    (157, 1),
    (161, 1),
    (165, 1),
    (169, 1),
    (173, 1),
    (177, 1),
    (181, 1),
    (185, 1),
    (190, 1),
    (195, 1),
    (200, 1),
    (202, 1),
    (204, 1),
    (206, 1),
    (208, 1),
    (212, 1),
    (214, 1),
    (217, 1),
    (220, 1),
    (223, 1),
    (225, 1),
];

/// Shared borrowed image context, containing the existing AImage program fields.
/// Both record sides and the initial state use this one complete ProgramBlock.
/// Each real decode checks full_program structurally and every program row.
#[derive(Clone, Copy)]
pub(super) struct ObservationProgramA<'a> {
    pub profile: &'a str,
    pub full_program: &'a Program,
    pub program: &'a ProgramBlock,
}

impl ObservationProgramA<'_> {
    fn image<'a>(&'a self, state: &'a StateBlock) -> AImage<'a> {
        AImage {
            profile: self.profile,
            full_program: self.full_program,
            program: self.program,
            state,
        }
    }
}

/// Host envelope outside StateBlock. rule_start_line is the actual source line
/// (for example 102), never its wire enum tag (3). rule_tag returns the L4 tag
/// after O4 membership/charge checks; no envelope byte serializer is added here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct RecordEnvelopeA {
    pub rule_start_line: usize,
    pub charge: u8,
}

impl RecordEnvelopeA {
    pub(super) fn rule_tag(self) -> StateResult<u8> {
        let (tag, &(_, charge)) = RULE_LINES
            .iter()
            .enumerate()
            .find(|(_, (line, _))| *line == self.rule_start_line)
            .ok_or(StateFailure::OutsideObservationDomain {
                condition: "O4: ruleStartLine must be listed in rule_line_tags",
                locator: Locator::field("envelope.ruleStartLine"),
            })?;
        if self.charge != charge {
            return Err(StateFailure::OutsideObservationDomain {
                condition: "O4: charge must equal the listed source rule charge",
                locator: Locator::field("envelope.charge"),
            });
        }
        Ok(tag as u8)
    }
}

/// Complete images, not deltas. Their shared program context is a separate
/// materialization argument, so a record cannot silently choose another program.
#[derive(Clone, Copy)]
pub(super) struct ObservedRecordA<'a> {
    pub envelope: RecordEnvelopeA,
    pub before_image: &'a StateBlock,
    pub after_image: &'a StateBlock,
}

/// Owns complete materialized observations and borrows caller-declared identity.
/// No layer-presence guess, producer provenance or source identity is synthesized.
pub(super) struct MaterializedTraceA<'a> {
    pub identity: TraceIdentity<'a>,
    pub initial: Snapshot,
    pub records: Vec<StepRecord>,
}

impl MaterializedTraceA<'_> {
    pub(super) fn view(&self) -> TraceView<'_> {
        TraceView {
            identity: self.identity,
            initial: &self.initial,
            records: &self.records,
        }
    }
}

/// Explicit canonical boundary projection. This reconstructs the same typed
/// observation from a Step representation and re-encodes it; it takes no step.
pub(super) fn project_a(
    prepared: &PreparedA,
    capacities: &StateCapacities,
    image: &AImage<'_>,
) -> StateResult<StateBlock> {
    let snapshot = decode_observed_a(prepared, capacities, image, DecodeMode::Step)?;
    encode_observed_a(prepared, capacities, &snapshot)
}

fn decode_side(
    prepared: &PreparedA,
    capacities: &StateCapacities,
    shared: &ObservationProgramA<'_>,
    state: &StateBlock,
    side: &'static str,
    record: usize,
) -> StateResult<Snapshot> {
    decode_observed_a(prepared, capacities, &shared.image(state), DecodeMode::Step).map_err(
        |failure| match failure {
            StateFailure::ProgramMismatch { .. } => StateFailure::ProgramMismatch {
                locator: Locator {
                    item: record,
                    ..Locator::field(side)
                },
            },
            other => other,
        },
    )
}

fn materialize_record_at(
    prepared: &PreparedA,
    capacities: &StateCapacities,
    shared: &ObservationProgramA<'_>,
    record: &ObservedRecordA<'_>,
    ordinal: usize,
) -> StateResult<StepRecord> {
    record
        .envelope
        .rule_tag()
        .map_err(|failure| match failure {
            StateFailure::OutsideObservationDomain {
                condition,
                mut locator,
            } => {
                locator.item = ordinal;
                StateFailure::OutsideObservationDomain { condition, locator }
            }
            other => other,
        })?;
    let before = decode_side(
        prepared,
        capacities,
        shared,
        record.before_image,
        "record.before.program",
        ordinal,
    )?;
    if !matches!(&before.outcome, Outcome::Pending) {
        return Err(StateFailure::OutsideObservationDomain {
            condition: "O4: a record before.outcome must be Pending",
            locator: Locator {
                item: ordinal,
                ..Locator::field("record.before.outcome")
            },
        });
    }
    let after = decode_side(
        prepared,
        capacities,
        shared,
        record.after_image,
        "record.after.program",
        ordinal,
    )?;
    Ok(StepRecord {
        rule_start_line: record.envelope.rule_start_line,
        charge: record.envelope.charge,
        before,
        after,
    })
}

/// Materialize both sides using the real Step decoder. O1-O3 are decoded on
/// each side and O4 is checked here. This does not evaluate the declared rule,
/// require a reachable state, or check that before transitions to after.
pub(super) fn materialize_record_a(
    prepared: &PreparedA,
    capacities: &StateCapacities,
    shared: &ObservationProgramA<'_>,
    record: &ObservedRecordA<'_>,
) -> StateResult<StepRecord> {
    materialize_record_at(prepared, capacities, shared, record, 0)
}

/// Keep the supplied initial state and every full ordered record. Identity is
/// passed through exactly as comparison metadata, independently of the checked
/// AImage profile; this is not authentication of any declared source/profile.
/// An empty records slice remains a present zero-step observation. Continuity,
/// transition validity and source reachability are not checked by this adapter.
pub(super) fn materialize_trace_a<'a>(
    prepared: &PreparedA,
    capacities: &StateCapacities,
    shared: &ObservationProgramA<'_>,
    identity: TraceIdentity<'a>,
    initial_image: &StateBlock,
    records: &[ObservedRecordA<'_>],
) -> StateResult<MaterializedTraceA<'a>> {
    let initial = decode_side(
        prepared,
        capacities,
        shared,
        initial_image,
        "initial.program",
        0,
    )?;
    let mut materialized = Vec::new();
    for (ordinal, record) in records.iter().enumerate() {
        // No record-capacity default is invented. Reserve incrementally before
        // materializing the next record and retain the existing located class.
        materialized
            .try_reserve(1)
            .map_err(|_| StateFailure::HostUnrepresentable {
                field: "trace.records",
                locator: Locator {
                    item: ordinal,
                    ..Locator::field("trace.records")
                },
            })?;
        let record = materialize_record_at(prepared, capacities, shared, record, ordinal)?;
        materialized.push(record);
    }
    Ok(MaterializedTraceA {
        identity,
        initial,
        records: materialized,
    })
}

/// Use the existing comparator directly. Equality is not trace validity,
/// continuity, authenticated identity, circuit acceptance or a proof result.
pub(super) fn compare_a(
    left: &MaterializedTraceA<'_>,
    right: &MaterializedTraceA<'_>,
) -> Option<TraceDifference> {
    first_difference(left.view(), right.view())
}
