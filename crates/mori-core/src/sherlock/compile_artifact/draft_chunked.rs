//! Technical chunking of an actual dedicated-draft owner run (HostOnly).
//!
//! The run is the same owner `step` loop as `OwnerEngine::run`. At each
//! requested technical cut, while the outcome is still Running, the driver
//! captures the complete `HostSnapshot`, archives it with the raw `C6RAWHS1`
//! codec, decodes the archive and continues from a fresh image rebuilt by
//! `owner_resume::resume_technical_checkpoint`. The live image is dropped, so
//! every later step really runs on the resumed checkpoint.
//!
//! A cut adds no charge, no event and no financial commit: the segment base
//! stays the uncut genesis value. Chunk accounting sums the actual events of
//! each chunk; the cumulative totals are checked against the final summary,
//! never assumed. Physical, circuit, proof, host and ledger costs remain
//! separate named unmeasured fields; checkpoint archive bytes are the only
//! serialization bytes observed here.
use serde_json::{Value, json};

use super::CompileFailure;
use super::wire::{self, OwnerWire};
use crate::sherlock::helper_machine::c6_raw_snapshot_archive::{
    RawSnapshotReadBudgets, RawSnapshotWriteBudgets, decode_raw_host_snapshot,
    encode_raw_host_snapshot,
};
use crate::sherlock::helper_machine::complete_observation::capture;
use crate::sherlock::helper_machine::resource_event_accounting::{EventAccounting, PrefixEnd};
use crate::sherlock::helper_machine::resource_report::Unmeasured;
use crate::sherlock::helper_machine::shared::owner_host::{host_owner_engine, observe_bound_image};
use crate::sherlock::helper_machine::shared::owner_resume::resume_technical_checkpoint;
use crate::sherlock::helper_machine::shared::{
    Admitted, MachineImage, OutcomeC, OwnerEngine, RunSummary, StepEventC, StepResult,
};
use crate::sherlock::reference_slice::RETAINED_RESOURCE_PROFILES;

pub const FORMAT: &str = "moriarty-sherlock-technical-chunks/1";
pub const CHECKPOINT_FORMAT: &str = "C6RAWHS1";

/// Where technical cuts fall, counted in owner events from genesis.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChunkPlan {
    /// Cut after every `n` events (`n >= 1`).
    Every(u128),
    /// Cut after exactly these event counts (strictly increasing, each >= 1).
    At(Vec<u128>),
}

impl ChunkPlan {
    /// Validate the caller's plan; no default or repair.
    pub fn validate(&self) -> Result<(), String> {
        match self {
            Self::Every(0) => Err("--chunk must be at least 1".into()),
            Self::Every(_) => Ok(()),
            Self::At(points) if points.is_empty() => Err("--chunk-at needs a position".into()),
            Self::At(points) => {
                if points[0] == 0 {
                    return Err("--chunk-at positions start at 1".into());
                }
                if points.windows(2).any(|w| w[0] >= w[1]) {
                    return Err("--chunk-at positions must be strictly increasing".into());
                }
                Ok(())
            }
        }
    }

    fn cuts_after(&self, events: u128) -> bool {
        match self {
            Self::Every(n) => events.is_multiple_of(*n),
            Self::At(points) => points.binary_search(&events).is_ok(),
        }
    }

    fn wire(&self) -> Value {
        match self {
            Self::Every(n) => json!({"kind": "every", "events": n.to_string()}),
            Self::At(points) => json!({
                "kind": "at",
                "events": points.iter().map(u128::to_string).collect::<Vec<_>>(),
            }),
        }
    }
}

/// Retain each chunk's full owner events, or only its accounting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChunkEvents {
    NotRetained,
    Retained,
}

/// Explicit request for a chunked run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChunkRequest {
    pub plan: ChunkPlan,
    pub events: ChunkEvents,
}

/// The completed chunked run: final image and summary, plus the chunk report.
pub(super) struct ChunkedOutcome {
    pub image: MachineImage,
    pub summary: RunSummary,
    pub report: Value,
}

fn counters(image: &ImageCounters) -> Value {
    json!({
        "consumed_work": image.consumed_work.to_string(),
        "remaining_work": image.remaining_work.to_string(),
        "helper_work": image.helper_work.to_string(),
        "helper_visits": image.helper_visits.to_string(),
        "reference_cursor": image.reference_cursor.to_string(),
    })
}

#[derive(Clone, Copy)]
struct ImageCounters {
    consumed_work: u128,
    remaining_work: u128,
    helper_work: u128,
    helper_visits: u128,
    reference_cursor: u128,
}

fn failure(stage: &'static str, events: u128, message: impl Into<String>) -> CompileFailure {
    CompileFailure {
        stage,
        actual_error: json!({"message": message.into()}),
        evaluation_performed: true,
        events_forwarded: events.to_string(),
        completed: false,
    }
}

fn observe(
    admitted: &Admitted,
    image: &MachineImage,
    events: u128,
) -> Result<(Value, ImageCounters, usize, OutcomeC), CompileFailure> {
    let observation = observe_bound_image(admitted, image)
        .map_err(|v| failure("observation", events, format!("{v:?}")))?;
    let i = observation.image;
    let c = ImageCounters {
        consumed_work: i.consumed_work,
        remaining_work: i.remaining_work,
        helper_work: i.helper_work,
        helper_visits: i.helper_visits,
        reference_cursor: i.reference_cursor,
    };
    Ok((wire::image(&i), c, i.frames.len(), *i.outcome))
}

fn peek(
    admitted: &Admitted,
    image: &MachineImage,
    events: u128,
) -> Result<(OutcomeC, ImageCounters), CompileFailure> {
    let observation = observe_bound_image(admitted, image)
        .map_err(|v| failure("observation", events, format!("{v:?}")))?;
    let i = observation.image;
    Ok((
        *i.outcome,
        ImageCounters {
            consumed_work: i.consumed_work,
            remaining_work: i.remaining_work,
            helper_work: i.helper_work,
            helper_visits: i.helper_visits,
            reference_cursor: i.reference_cursor,
        },
    ))
}

/// Archive, decode and resume the complete checkpoint of `image`.
fn checkpoint(
    admitted: &Admitted,
    image: &MachineImage,
    serialization_bytes: usize,
    events: u128,
) -> Result<(MachineImage, Value), CompileFailure> {
    let snapshot =
        capture(admitted, image).map_err(|v| failure("checkpoint", events, format!("{v:?}")))?;
    let limit = serialization_bytes.max(1);
    let mut bytes = Vec::new();
    encode_raw_host_snapshot(
        &snapshot,
        RawSnapshotWriteBudgets {
            max_output_bytes: limit,
            max_type_nodes: limit,
            max_type_depth: 64,
            max_walk_bytes: limit,
        },
        &mut bytes,
    )
    .map_err(|e| failure("checkpoint", events, format!("archive refused: {e:?}")))?;
    let decoded = decode_raw_host_snapshot(
        &bytes,
        RawSnapshotReadBudgets {
            max_input_bytes: limit,
            max_elements: limit,
            max_alloc_bytes: limit.saturating_mul(64),
            max_type_nodes: limit,
            max_type_depth: 64,
        },
    )
    .map_err(|e| {
        failure(
            "checkpoint",
            events,
            format!("archive decode refused: {e:?}"),
        )
    })?;
    let archive_round_trip = decoded.snapshot() == &snapshot;
    if !archive_round_trip {
        return Err(failure(
            "checkpoint",
            events,
            "decoded archive differs from the captured snapshot",
        ));
    }
    let resumed = resume_technical_checkpoint(admitted, decoded.snapshot())
        .map_err(|e| failure("checkpoint", events, e.to_string()))?;
    Ok((
        resumed,
        json!({
            "format": CHECKPOINT_FORMAT,
            "bytes": bytes.len().to_string(),
            "sha256": crate::intent::hash_bytes(&bytes),
            "archive_round_trip": "equal",
            "resumed_from_archive": true,
            "resumed_capture": "equal",
            "segment_base": "genesis-uncut",
            "financial_commit": "none",
        }),
    ))
}

fn accounting_wire(acc: &EventAccounting, end: PrefixEnd) -> Value {
    serde_json::to_value(acc.snapshot(end)).unwrap_or(Value::Null)
}

/// Drive the owner one step at a time over `image`, cutting per `request`.
pub(super) fn run_chunked(
    admitted: &Admitted,
    mut image: MachineImage,
    request: &ChunkRequest,
    serialization_bytes: usize,
    sink: &mut dyn FnMut(&StepEventC),
) -> Result<ChunkedOutcome, CompileFailure> {
    request
        .plan
        .validate()
        .map_err(|m| CompileFailure::input("config", m))?;
    let engine = host_owner_engine();
    let mut events: u128 = 0;
    let mut total = EventAccounting::new();
    let mut chunks: Vec<Value> = Vec::new();
    let (_, mut start_counters, _, _) = observe(admitted, &image, events)?;
    let mut chunk_start = events;
    let mut chunk_acc = EventAccounting::new();
    let mut chunk_events: Vec<Value> = Vec::new();
    let mut cuts: u128 = 0;
    loop {
        let terminal = if peek(admitted, &image, events)?.0 != OutcomeC::Running {
            true
        } else {
            match engine
                .step(admitted, &mut image)
                .map_err(|v| failure("run", events, format!("{v:?}")))?
            {
                StepResult::Terminal => true,
                StepResult::Event { event } => {
                    events = events
                        .checked_add(1)
                        .ok_or_else(|| failure("run", events, "event count overflow"))?;
                    total.observe(&event);
                    chunk_acc.observe(&event);
                    if request.events == ChunkEvents::Retained {
                        chunk_events.push(json!({
                            "position": events.to_string(),
                            "event": event.owner_wire(),
                        }));
                    }
                    sink(&event);
                    false
                }
            }
        };
        let running = peek(admitted, &image, events)?.0 == OutcomeC::Running;
        let cut = !terminal && running && request.plan.cuts_after(events);
        if !terminal && !cut {
            continue;
        }
        let (resumed, checkpoint) = if cut {
            let (resumed, record) = checkpoint(admitted, &image, serialization_bytes, events)?;
            (Some(resumed), record)
        } else {
            (None, Value::Null)
        };
        if let Some(resumed) = resumed {
            image = resumed;
            cuts += 1;
        }
        let (boundary_image, end_counters, depth, outcome) = observe(admitted, &image, events)?;
        let mut record = json!({
            "index": chunks.len().to_string(),
            "start_event": chunk_start.to_string(),
            "end_event": events.to_string(),
            "end": if cut { "technical-cut" } else { "terminal" },
            "start_counters": counters(&start_counters),
            "end_counters": counters(&end_counters),
            "accounting": accounting_wire(&chunk_acc, if cut { PrefixEnd::Open } else { PrefixEnd::ReturnedOk }),
            "boundary": {
                "outcome": outcome.owner_wire(),
                "logical_depth": depth.to_string(),
                "image": boundary_image,
            },
            "checkpoint": checkpoint,
        });
        if request.events == ChunkEvents::Retained {
            record["events"] = Value::Array(std::mem::take(&mut chunk_events));
        }
        chunks.push(record);
        start_counters = end_counters;
        chunk_start = events;
        chunk_acc = EventAccounting::new();
        if !cut {
            break;
        }
    }
    let (final_outcome, last) = peek(admitted, &image, events)?;
    let summary = RunSummary {
        outcome: final_outcome,
        events,
        consumed_work: last.consumed_work,
        helper_work: last.helper_work,
        helper_visits: last.helper_visits,
        reference_cursor: last.reference_cursor,
    };
    let total_report = total.snapshot(PrefixEnd::ReturnedOk);
    let exact =
        |t: crate::sherlock::helper_machine::resource_event_accounting::CheckedTotal| match t {
            crate::sherlock::helper_machine::resource_event_accounting::CheckedTotal::Exact {
                value,
            } => Some(value.0),
            _ => None,
        };
    let reconciled = exact(total_report.event_count) == Some(summary.events)
        && exact(total_report.work_delta_sum) == Some(summary.consumed_work)
        && exact(total_report.helper_work_delta_sum) == Some(summary.helper_work)
        && exact(total_report.helper_visits_delta_sum) == Some(summary.helper_visits)
        && exact(total_report.reference_cursor_delta_sum) == Some(summary.reference_cursor);
    if !reconciled {
        return Err(failure(
            "chunk-accounting",
            events,
            "chunk event sums differ from the final owner counters",
        ));
    }
    let retained: Vec<Value> = RETAINED_RESOURCE_PROFILES
        .iter()
        .map(|p| {
            json!({
                "reference_schedule_id": p.reference_schedule_id,
                "terminal_return_charge": p.terminal_return_charge.to_string(),
                "pure_helper_metering": p.pure_helper_metering,
                "qualification": p.qualification,
            })
        })
        .collect();
    let report = json!({
        "format": FORMAT,
        "qualification": "HostOnly technical chunks of one owner run; no proof, circuit or target qualification",
        "resource_manifest": {
            "reference_schedule_id": super::EXECUTION_PROFILE,
            "same_fuel_equivalence": "not-claimed",
            "measured_cost_winner": "none",
            "retained_profiles": retained,
        },
        "plan": request.plan.wire(),
        "technical_cuts": cuts.to_string(),
        "chunk_count": chunks.len().to_string(),
        "chunks": chunks,
        "cumulative": serde_json::to_value(total_report).unwrap_or(Value::Null),
        "cumulative_equals_summary": true,
        "financial_commits": "0",
        "separate_fields": {
            "observed": [
                "consumed_work", "reference_cursor", "helper_work", "helper_visits",
                "events", "logical_depth", "checkpoint_bytes"
            ],
            "unmeasured": Unmeasured::from_snapshot_only(),
        },
    });
    Ok(ChunkedOutcome {
        image,
        summary,
        report,
    })
}
