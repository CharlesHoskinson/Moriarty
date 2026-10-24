//! Public Core/1 complete diagnostic records around the unchanged owner API.
//! Separates per-projection capacity (`admission.bounds.serialization_bytes`,
//! where `0` is a real zero bound) from the cumulative trace/envelope ceiling
//! (`derive_trace_serialization_budget`, bounded by `crate::json`'s 1 MiB /
//! 65,536-node / 32-depth limits via `crate::json::parse_limited`) without
//! retaining duplicate `CompleteStepRecord` vectors alongside the byte buffer.
use super::{
    CompileFailure,
    complete_trace_wire::{
        BoundedJsonWriter, BoundedWireError, TRACE_FORMAT, TRACE_SERIALIZATION_BUDGET,
    },
};
use crate::sherlock::helper_machine::complete_observation::{
    CaptureStepError, CompleteStepRecord, HostSnapshot, ObservedStep, capture, record_step,
};
use crate::sherlock::helper_machine::core_action_consumer::{
    CompletedRun, CoreActionPrepared, CoreActionRun,
};
use crate::sherlock::helper_machine::shared::{
    BoundsC, ExpandedStepIdC, HostViolation, RuleIdC, RunSummary, StepDisposition,
};
use serde_json::{Value, json};

/// Canonical `crate::json` document byte ceiling (`1024 * 1024` = 1 MiB).
pub(super) const PUBLIC_JSON_MAX_BYTES: usize = 1024 * 1024;

/// Derive the cumulative multi-step trace byte ceiling from the admitted
/// `BoundsC` (`serialization_bytes` per projection across `step_cap` steps plus
/// initial/returned projections) capped by `PUBLIC_JSON_MAX_BYTES`.
pub(super) fn derive_trace_serialization_budget(bounds: &BoundsC) -> usize {
    let projections = usize::try_from(bounds.step_cap)
        .unwrap_or(usize::MAX)
        .saturating_add(2);
    bounds
        .serialization_bytes
        .saturating_mul(projections)
        .min(PUBLIC_JSON_MAX_BYTES)
}

const TRACE_OPEN_BEFORE_INITIAL: &[u8] =
    br#"{"format":"sherlock-core1-complete-trace/2","initial":"#;
const TRACE_AFTER_INITIAL_OPEN_RECORDS: &[u8] = br#","qualification":"actual complete owner records; no transition/native/target acceptance","records":["#;
const TRACE_PREFIX_CLOSE: &[u8] =
    br#"],"serialization_budget":"per_projection:admission.bounds.serialization_bytes;whole_trace:derive_trace_serialization_budget(admission.bounds);step_boundaries:image-only-shared-admission"}"#;
const TRACE_CLOSE_RECORDS_OPEN_RETURNED: &[u8] = br#"],"returned":"#;
const TRACE_AFTER_RETURNED_SERIALIZATION_BUDGET: &[u8] =
    br#","serialization_budget":"per_projection:admission.bounds.serialization_bytes;whole_trace:derive_trace_serialization_budget(admission.bounds);step_boundaries:image-only-shared-admission""#;
const TRACE_OPEN_TERMINAL: &[u8] = br#","terminal":"#;
const TRACE_CLOSE_ROOT: &[u8] = b"}";

fn wire_error_json(error: &BoundedWireError) -> Value {
    match error {
        BoundedWireError::SerializationLimitExceeded { limit } => json!({
            "kind": "SerializationLimitExceeded",
            "limit": limit.to_string(),
        }),
        BoundedWireError::OutputNotUtf8 => json!({
            "kind": "OutputNotUtf8",
        }),
        BoundedWireError::Serialize { field, message } => json!({
            "kind": "Serialize",
            "field": field,
            "message": message,
        }),
    }
}

fn bounded_wire_value<T: super::wire::OwnerWire>(
    value: &T,
    limit: usize,
    field: &'static str,
) -> Value {
    let mut writer = BoundedJsonWriter::new(limit);
    match writer
        .write_owner_wire(value, field)
        .and_then(|()| writer.into_value(field))
    {
        Ok(encoded) => encoded,
        Err(error) => json!({ "serialization_error": wire_error_json(&error) }),
    }
}

#[derive(Debug)]
pub(super) enum BoundedFailureKind {
    Initial,
    Record {
        step_id: ExpandedStepIdC,
        rule_id: RuleIdC,
        disposition: StepDisposition,
    },
    Returned,
    Terminal {
        returned_checkpoint: usize,
    },
    CaptureStepBoundary {
        stage: &'static str,
        boundary_kind: &'static str,
        violation: HostViolation,
        executed_event: Option<(ExpandedStepIdC, RuleIdC, StepDisposition)>,
    },
}

/// Typed per-projection or whole-trace serialization failure retaining the
/// bounded UTF-8 buffer and record checkpoints without unread snapshot fields.
#[derive(Debug)]
pub(super) struct BoundedTraceFailure {
    writer: BoundedJsonWriter,
    record_checkpoints: Vec<usize>,
    per_projection_limit: usize,
    trace_limit: usize,
    error: BoundedWireError,
    events_forwarded: u128,
    evaluation_performed: bool,
    completed: bool,
    kind: BoundedFailureKind,
}

impl BoundedTraceFailure {
    fn write_envelope_with_trace(
        limit: usize,
        trace_slice: Option<&[u8]>,
        failure_boundary: &Value,
        serialization_error: Option<&Value>,
        violation: Option<&HostViolation>,
    ) -> Result<Value, BoundedWireError> {
        let mut env = BoundedJsonWriter::new(limit);
        env.write_raw(b"{\"complete_trace\":")?;
        match trace_slice {
            Some(bytes) => env.write_raw(bytes)?,
            None => env.write_raw(b"null")?,
        }
        env.write_raw(b",\"failure_boundary\":")?;
        env.write_serialize(failure_boundary, "failure_boundary")?;
        if let Some(ser_err) = serialization_error {
            env.write_raw(b",\"serialization_error\":")?;
            env.write_serialize(ser_err, "serialization_error")?;
        }
        if let Some(v) = violation {
            env.write_raw(b",\"violation\":")?;
            env.write_owner_wire(v, "violation")?;
        }
        env.write_raw(b"}")?;
        env.into_value("actual_error")
    }

    pub(super) fn into_compile_failure(self) -> CompileFailure {
        let Self {
            mut writer,
            record_checkpoints,
            per_projection_limit,
            trace_limit,
            error,
            events_forwarded,
            evaluation_performed,
            completed,
            kind,
        } = self;
        let events_str = events_forwarded.to_string();
        let exceeded_limit = match &error {
            BoundedWireError::SerializationLimitExceeded { limit } => *limit,
            _ => per_projection_limit.min(trace_limit),
        };
        let field_limit = per_projection_limit.min(trace_limit);
        let ser_err = wire_error_json(&error);

        if matches!(kind, BoundedFailureKind::Initial)
            || per_projection_limit == 0
            || trace_limit == 0
        {
            let boundary = json!({
                "kind": "initial-serialization-limit-exceeded",
                "limit": exceeded_limit.to_string(),
            });
            let actual_error =
                Self::write_envelope_with_trace(trace_limit, None, &boundary, Some(&ser_err), None)
                    .unwrap_or_else(|_| {
                        json!({
                            "complete_trace": Value::Null,
                            "failure_boundary": boundary,
                            "serialization_error": ser_err,
                        })
                    });
            return CompileFailure {
                stage: "observation",
                actual_error,
                evaluation_performed,
                events_forwarded: events_str,
                completed,
            };
        }

        let stage = match &kind {
            BoundedFailureKind::CaptureStepBoundary { stage, .. } => *stage,
            BoundedFailureKind::Initial
            | BoundedFailureKind::Record { .. }
            | BoundedFailureKind::Returned
            | BoundedFailureKind::Terminal { .. } => "observation",
        };

        let try_candidate = |preserved_records: usize,
                             trace_slice: Option<&[u8]>|
         -> Result<Value, BoundedWireError> {
            let boundary = match &kind {
                BoundedFailureKind::Initial => unreachable!(),
                BoundedFailureKind::Record {
                    step_id,
                    rule_id,
                    disposition,
                } => json!({
                    "executed_records": &events_str,
                    "failed_disposition": bounded_wire_value(disposition, field_limit, "failed_disposition"),
                    "failed_rule_id": bounded_wire_value(rule_id, field_limit, "failed_rule_id"),
                    "failed_step_id": bounded_wire_value(step_id, field_limit, "failed_step_id"),
                    "kind": "record-serialization-limit-exceeded",
                    "limit": exceeded_limit.to_string(),
                    "preserved_records": preserved_records.to_string(),
                }),
                BoundedFailureKind::Returned => json!({
                    "executed_records": &events_str,
                    "kind": "returned-serialization-limit-exceeded",
                    "limit": exceeded_limit.to_string(),
                    "preserved_records": preserved_records.to_string(),
                }),
                BoundedFailureKind::Terminal { .. } => json!({
                    "executed_records": &events_str,
                    "kind": "terminal-serialization-limit-exceeded",
                    "limit": exceeded_limit.to_string(),
                    "preserved_records": preserved_records.to_string(),
                }),
                BoundedFailureKind::CaptureStepBoundary {
                    boundary_kind,
                    executed_event,
                    ..
                } => match executed_event {
                    Some((step_id, rule_id, disposition)) => json!({
                        "event_present": true,
                        "executed_records": &events_str,
                        "failed_disposition": bounded_wire_value(disposition, field_limit, "failed_disposition"),
                        "failed_rule_id": bounded_wire_value(rule_id, field_limit, "failed_rule_id"),
                        "failed_step_id": bounded_wire_value(step_id, field_limit, "failed_step_id"),
                        "kind": boundary_kind,
                        "limit": exceeded_limit.to_string(),
                        "preserved_records": preserved_records.to_string(),
                        "serialization_error": &ser_err,
                    }),
                    None => json!({
                        "event_present": false,
                        "executed_records": &events_str,
                        "kind": boundary_kind,
                        "limit": exceeded_limit.to_string(),
                        "preserved_records": preserved_records.to_string(),
                        "serialization_error": &ser_err,
                    }),
                },
            };

            match &kind {
                BoundedFailureKind::CaptureStepBoundary { violation, .. } => {
                    Self::write_envelope_with_trace(
                        trace_limit,
                        trace_slice,
                        &boundary,
                        None,
                        Some(violation),
                    )
                }
                _ => Self::write_envelope_with_trace(
                    trace_limit,
                    trace_slice,
                    &boundary,
                    Some(&ser_err),
                    None,
                ),
            }
        };

        if let BoundedFailureKind::Terminal {
            returned_checkpoint,
        } = kind
        {
            if returned_checkpoint > 0 {
                let all_recs = record_checkpoints.len().saturating_sub(1);
                writer.rollback(returned_checkpoint);
                if writer.write_raw(TRACE_CLOSE_ROOT).is_ok() {
                    if let Ok(actual_error) = try_candidate(all_recs, Some(writer.as_bytes())) {
                        return CompileFailure {
                            stage,
                            actual_error,
                            evaluation_performed,
                            events_forwarded: events_str,
                            completed,
                        };
                    }
                }
            }
        }

        for (idx, &cp) in record_checkpoints.iter().enumerate().rev() {
            writer.rollback(cp);
            if writer.write_raw(TRACE_PREFIX_CLOSE).is_ok() {
                if let Ok(actual_error) = try_candidate(idx, Some(writer.as_bytes())) {
                    return CompileFailure {
                        stage,
                        actual_error,
                        evaluation_performed,
                        events_forwarded: events_str,
                        completed,
                    };
                }
            }
        }

        if let Ok(actual_error) = try_candidate(0, None) {
            return CompileFailure {
                stage,
                actual_error,
                evaluation_performed,
                events_forwarded: events_str,
                completed,
            };
        }

        let fallback_error = match &kind {
            BoundedFailureKind::CaptureStepBoundary {
                boundary_kind,
                violation,
                executed_event,
                ..
            } => {
                let boundary = match executed_event {
                    Some((step_id, rule_id, disposition)) => json!({
                        "event_present": true,
                        "executed_records": &events_str,
                        "failed_disposition": bounded_wire_value(disposition, field_limit, "failed_disposition"),
                        "failed_rule_id": bounded_wire_value(rule_id, field_limit, "failed_rule_id"),
                        "failed_step_id": bounded_wire_value(step_id, field_limit, "failed_step_id"),
                        "kind": boundary_kind,
                        "limit": exceeded_limit.to_string(),
                        "preserved_records": "0",
                        "serialization_error": &ser_err,
                    }),
                    None => json!({
                        "event_present": false,
                        "executed_records": &events_str,
                        "kind": boundary_kind,
                        "limit": exceeded_limit.to_string(),
                        "preserved_records": "0",
                        "serialization_error": &ser_err,
                    }),
                };
                json!({
                    "complete_trace": Value::Null,
                    "failure_boundary": boundary,
                    "violation": bounded_wire_value(violation, field_limit, "violation"),
                })
            }
            BoundedFailureKind::Record {
                step_id,
                rule_id,
                disposition,
            } => json!({
                "complete_trace": Value::Null,
                "failure_boundary": {
                    "executed_records": &events_str,
                    "failed_disposition": bounded_wire_value(disposition, field_limit, "failed_disposition"),
                    "failed_rule_id": bounded_wire_value(rule_id, field_limit, "failed_rule_id"),
                    "failed_step_id": bounded_wire_value(step_id, field_limit, "failed_step_id"),
                    "kind": "record-serialization-limit-exceeded",
                    "limit": exceeded_limit.to_string(),
                    "preserved_records": "0",
                },
                "serialization_error": ser_err,
            }),
            BoundedFailureKind::Returned => json!({
                "complete_trace": Value::Null,
                "failure_boundary": {
                    "executed_records": &events_str,
                    "kind": "returned-serialization-limit-exceeded",
                    "limit": exceeded_limit.to_string(),
                    "preserved_records": "0",
                },
                "serialization_error": ser_err,
            }),
            BoundedFailureKind::Terminal { .. } => json!({
                "complete_trace": Value::Null,
                "failure_boundary": {
                    "executed_records": &events_str,
                    "kind": "terminal-serialization-limit-exceeded",
                    "limit": exceeded_limit.to_string(),
                    "preserved_records": "0",
                },
                "serialization_error": ser_err,
            }),
            BoundedFailureKind::Initial => unreachable!(),
        };

        CompileFailure {
            stage,
            actual_error: fallback_error,
            evaluation_performed,
            events_forwarded: events_str,
            completed,
        }
    }
}

pub(super) struct Trace {
    writer: BoundedJsonWriter,
    per_projection_limit: usize,
    trace_limit: usize,
    checkpoint: usize,
    record_checkpoints: Vec<usize>,
}

impl Trace {
    pub(super) fn begin(initial: HostSnapshot) -> Result<Self, BoundedTraceFailure> {
        let per_projection_limit = initial.admission.bounds.serialization_bytes;
        let trace_limit = derive_trace_serialization_budget(&initial.admission.bounds);
        Self::begin_with_limits(initial, per_projection_limit, trace_limit)
    }

    pub(super) fn begin_with_limits(
        initial: HostSnapshot,
        per_projection_limit: usize,
        trace_limit: usize,
    ) -> Result<Self, BoundedTraceFailure> {
        let mut writer = BoundedJsonWriter::new(trace_limit);
        let write_result = (|| -> Result<usize, BoundedWireError> {
            if per_projection_limit == 0 {
                return Err(BoundedWireError::SerializationLimitExceeded { limit: 0 });
            }
            writer.write_raw(TRACE_OPEN_BEFORE_INITIAL)?;
            writer.with_projection_budget(per_projection_limit, |w| {
                w.write_host_snapshot(&initial)
            })?;
            writer.write_raw(TRACE_AFTER_INITIAL_OPEN_RECORDS)?;
            let checkpoint = writer.checkpoint();
            writer.write_raw(TRACE_PREFIX_CLOSE)?;
            writer.rollback(checkpoint);
            Ok(checkpoint)
        })();
        drop(initial);
        match write_result {
            Ok(checkpoint) => Ok(Self {
                writer,
                per_projection_limit,
                trace_limit,
                checkpoint,
                record_checkpoints: vec![checkpoint],
            }),
            Err(error) => Err(BoundedTraceFailure {
                writer,
                record_checkpoints: Vec::new(),
                per_projection_limit,
                trace_limit,
                error,
                events_forwarded: 0,
                evaluation_performed: false,
                completed: false,
                kind: BoundedFailureKind::Initial,
            }),
        }
    }

    pub(super) fn record_count(&self) -> usize {
        self.record_checkpoints.len().saturating_sub(1)
    }

    /// Exact UTF-8 byte length of the closed prefix (`initial` + `records` + envelope).
    #[cfg(test)]
    pub(super) fn prefix_byte_len(&self) -> usize {
        self.checkpoint + TRACE_PREFIX_CLOSE.len()
    }

    #[cfg(test)]
    pub(super) fn prefix_bytes(&self) -> Vec<u8> {
        let mut copy = self.writer.clone();
        copy.rollback(self.checkpoint);
        let _ = copy.write_raw(TRACE_PREFIX_CLOSE);
        copy.as_bytes().to_vec()
    }

    #[cfg(test)]
    pub(super) fn prefix(&self) -> Value {
        let bytes = self.prefix_bytes();
        super::complete_trace_wire::parse_bounded_slice(
            &bytes,
            self.writer.limit(),
            "complete_trace",
        )
        .unwrap_or(Value::Null)
    }

    pub(super) fn push_record(
        &mut self,
        record: CompleteStepRecord,
    ) -> Result<(), BoundedTraceFailure> {
        let prior_records = self.record_count();
        let per_projection_limit = self.per_projection_limit;
        let trace_limit = self.trace_limit;
        self.writer.rollback(self.checkpoint);
        let write_result = (|| -> Result<usize, BoundedWireError> {
            if prior_records > 0 {
                self.writer.write_raw(b",")?;
            }
            self.writer
                .with_projection_budget(per_projection_limit, |w| w.write_step_record(&record))?;
            let next_checkpoint = self.writer.checkpoint();
            self.writer.write_raw(TRACE_PREFIX_CLOSE)?;
            self.writer.rollback(next_checkpoint);
            Ok(next_checkpoint)
        })();
        match write_result {
            Ok(next_checkpoint) => {
                drop(record);
                self.checkpoint = next_checkpoint;
                self.record_checkpoints.push(next_checkpoint);
                Ok(())
            }
            Err(error) => {
                let CompleteStepRecord {
                    before,
                    event,
                    after,
                } = record;
                drop(before);
                drop(after);
                let step_id = event.step_id;
                let rule_id = event.rule_id;
                let disposition = event.disposition;
                let mut writer = BoundedJsonWriter::new(trace_limit);
                std::mem::swap(&mut self.writer, &mut writer);
                let record_checkpoints = std::mem::take(&mut self.record_checkpoints);
                Err(BoundedTraceFailure {
                    writer,
                    record_checkpoints,
                    per_projection_limit,
                    trace_limit,
                    error,
                    events_forwarded: prior_records as u128 + 1,
                    evaluation_performed: true,
                    completed: false,
                    kind: BoundedFailureKind::Record {
                        step_id,
                        rule_id,
                        disposition,
                    },
                })
            }
        }
    }

    pub(super) fn finish_terminal(
        mut self,
        before: HostSnapshot,
        after: HostSnapshot,
    ) -> Result<Value, BoundedTraceFailure> {
        let per_projection_limit = self.per_projection_limit;
        let trace_limit = self.trace_limit;
        let events = self.record_count() as u128;
        self.writer.rollback(self.checkpoint);
        let returned_checkpoint = match (|| -> Result<usize, BoundedWireError> {
            self.writer.write_raw(TRACE_CLOSE_RECORDS_OPEN_RETURNED)?;
            self.writer
                .with_projection_budget(per_projection_limit, |w| w.write_host_snapshot(&after))?;
            self.writer
                .write_raw(TRACE_AFTER_RETURNED_SERIALIZATION_BUDGET)?;
            let after_returned = self.writer.checkpoint();
            self.writer.write_raw(TRACE_CLOSE_ROOT)?;
            self.writer.rollback(after_returned);
            Ok(after_returned)
        })() {
            Ok(cp) => cp,
            Err(error) => {
                drop(before);
                drop(after);
                return Err(BoundedTraceFailure {
                    writer: self.writer,
                    record_checkpoints: self.record_checkpoints,
                    per_projection_limit,
                    trace_limit,
                    error,
                    events_forwarded: events,
                    evaluation_performed: true,
                    completed: true,
                    kind: BoundedFailureKind::Returned,
                });
            }
        };

        let terminal_write = (|| -> Result<(), BoundedWireError> {
            self.writer.write_raw(TRACE_OPEN_TERMINAL)?;
            self.writer
                .with_projection_budget(per_projection_limit, |w| {
                    w.write_terminal_boundary(&before, &after)
                })?;
            self.writer.write_raw(TRACE_CLOSE_ROOT)
        })();
        drop(before);
        drop(after);

        match terminal_write {
            Ok(()) => match self.writer.parse_value("complete_trace") {
                Ok(value) => Ok(value),
                Err(error) => Err(BoundedTraceFailure {
                    writer: self.writer,
                    record_checkpoints: self.record_checkpoints,
                    per_projection_limit,
                    trace_limit,
                    error,
                    events_forwarded: events,
                    evaluation_performed: true,
                    completed: true,
                    kind: BoundedFailureKind::Terminal {
                        returned_checkpoint,
                    },
                }),
            },
            Err(error) => Err(BoundedTraceFailure {
                writer: self.writer,
                record_checkpoints: self.record_checkpoints,
                per_projection_limit,
                trace_limit,
                error,
                events_forwarded: events,
                evaluation_performed: true,
                completed: true,
                kind: BoundedFailureKind::Terminal {
                    returned_checkpoint,
                },
            }),
        }
    }

    pub(super) fn failure_checked(
        mut self,
        error: CaptureStepError,
    ) -> Result<CompileFailure, BoundedTraceFailure> {
        let per_projection_limit = self.per_projection_limit;
        let trace_limit = self.trace_limit;
        let prior_records = self.record_count();

        let (stage, additional_event) = match &error {
            CaptureStepError::BeforeObservation(_) => ("observation", 0u128),
            CaptureStepError::Step { .. } => ("run", 0u128),
            CaptureStepError::AfterObservation { event, .. } => {
                ("observation", u128::from(event.is_some()))
            }
        };
        let events = prior_records as u128 + additional_event;
        let evaluation_performed = prior_records > 0 || stage == "run" || additional_event != 0;

        let mut env = BoundedJsonWriter::new(trace_limit);
        let full_write = (|| -> Result<Value, BoundedWireError> {
            self.writer.rollback(self.checkpoint);
            self.writer.write_raw(TRACE_PREFIX_CLOSE)?;
            env.write_raw(b"{\"complete_trace\":")?;
            env.write_raw(self.writer.as_bytes())?;
            env.write_raw(b",\"failure_boundary\":")?;
            match &error {
                CaptureStepError::BeforeObservation(_) => {
                    env.write_raw(b"{\"kind\":\"before-observation\"}")?;
                }
                CaptureStepError::Step { before, after, .. } => {
                    env.write_raw(b"{\"after\":")?;
                    match after {
                        Ok(snapshot) => {
                            env.write_raw(b"{\"Ok\":")?;
                            env.with_projection_budget(per_projection_limit, |w| {
                                w.write_host_snapshot(snapshot)
                            })?;
                            env.write_raw(b"}")?;
                        }
                        Err(violation) => {
                            env.write_raw(b"{\"Err\":")?;
                            env.write_owner_wire(violation, "after.Err")?;
                            env.write_raw(b"}")?;
                        }
                    }
                    env.write_raw(b",\"before\":")?;
                    env.with_projection_budget(per_projection_limit, |w| {
                        w.write_host_snapshot(before)
                    })?;
                    env.write_raw(b",\"kind\":\"step\"}")?;
                }
                CaptureStepError::AfterObservation { before, event, .. } => {
                    env.write_raw(b"{\"before\":")?;
                    env.with_projection_budget(per_projection_limit, |w| {
                        w.write_host_snapshot(before)
                    })?;
                    env.write_raw(b",\"event\":")?;
                    env.write_owner_wire(event, "failure_boundary.event")?;
                    env.write_raw(b",\"kind\":\"after-observation\"}")?;
                }
            }
            env.write_raw(b",\"violation\":")?;
            match &error {
                CaptureStepError::BeforeObservation(violation)
                | CaptureStepError::Step { violation, .. }
                | CaptureStepError::AfterObservation { violation, .. } => {
                    env.write_owner_wire(violation, "violation")?;
                }
            }
            env.write_raw(b"}")?;
            env.into_value("actual_error")
        })();

        match full_write {
            Ok(actual_error) => Ok(CompileFailure {
                stage,
                actual_error,
                evaluation_performed,
                events_forwarded: events.to_string(),
                completed: false,
            }),
            Err(wire_error) => {
                let (boundary_kind, violation, executed_event) = match error {
                    CaptureStepError::BeforeObservation(v) => ("before-observation", v, None),
                    CaptureStepError::Step { violation: v, .. } => ("step", v, None),
                    CaptureStepError::AfterObservation {
                        violation: v,
                        event,
                        ..
                    } => (
                        "after-observation",
                        v,
                        event.map(|ev| (ev.step_id, ev.rule_id, ev.disposition)),
                    ),
                };
                Err(BoundedTraceFailure {
                    writer: self.writer,
                    record_checkpoints: self.record_checkpoints,
                    per_projection_limit,
                    trace_limit,
                    error: wire_error,
                    events_forwarded: events,
                    evaluation_performed,
                    completed: false,
                    kind: BoundedFailureKind::CaptureStepBoundary {
                        stage,
                        boundary_kind,
                        violation,
                        executed_event,
                    },
                })
            }
        }
    }

    pub(super) fn failure(self, error: CaptureStepError) -> CompileFailure {
        self.failure_checked(error)
            .unwrap_or_else(BoundedTraceFailure::into_compile_failure)
    }
}

fn initial_failure(violation: HostViolation, limit: usize) -> CompileFailure {
    let violation_wire = bounded_wire_value(&violation, limit, "violation");
    let complete_trace = if limit == 0 {
        Value::Null
    } else {
        json!({
            "format": TRACE_FORMAT,
            "initial": Value::Null,
            "records": [],
        })
    };
    CompileFailure {
        stage: "observation",
        actual_error: json!({
            "violation": violation_wire,
            "complete_trace": complete_trace,
            "failure_boundary": {
                "kind": "initial-observation",
            },
        }),
        evaluation_performed: false,
        events_forwarded: "0".into(),
        completed: false,
    }
}

/// Consume exactly one existing prepared pair; do not admit or prepare again.
/// record_step alone calls the existing owner step. No alternate engine or
/// inferred transition is used. Terminal has no fabricated event/charge.
pub(super) fn run(prepared: CoreActionPrepared) -> Result<(CoreActionRun, Value), CompileFailure> {
    let bounds = *prepared
        .observe()
        .map_err(|v| initial_failure(v, PUBLIC_JSON_MAX_BYTES))?
        .admission
        .bounds;
    let per_projection_limit = bounds.serialization_bytes;
    let trace_limit = derive_trace_serialization_budget(&bounds);
    run_with_limits(prepared, per_projection_limit, trace_limit)
}

pub(super) fn run_with_limits(
    prepared: CoreActionPrepared,
    per_projection_limit: usize,
    trace_limit: usize,
) -> Result<(CoreActionRun, Value), CompileFailure> {
    let CoreActionPrepared {
        preparation,
        admitted,
        mut image,
    } = prepared;
    let initial = capture(&admitted, &image)
        .map_err(|v| initial_failure(v, per_projection_limit.min(trace_limit)))?;
    let mut trace = Trace::begin_with_limits(initial, per_projection_limit, trace_limit)
        .map_err(BoundedTraceFailure::into_compile_failure)?;
    loop {
        match record_step(&admitted, &mut image) {
            Ok(ObservedStep::Event(record)) => {
                trace
                    .push_record(record)
                    .map_err(BoundedTraceFailure::into_compile_failure)?;
            }
            Ok(ObservedStep::Terminal { before, after }) => {
                // These are the actual final registers, not A work or source
                // counters. Events count exactly the records just delivered.
                let events = trace.record_count() as u128;
                let summary = RunSummary {
                    outcome: after.image.outcome,
                    events,
                    consumed_work: after.image.consumed_work,
                    helper_work: after.image.helper_work,
                    helper_visits: after.image.helper_visits,
                    reference_cursor: after.image.reference_cursor,
                };
                let output = trace
                    .finish_terminal(before, after)
                    .map_err(BoundedTraceFailure::into_compile_failure)?;
                return Ok((
                    CoreActionRun {
                        preparation,
                        completed: CompletedRun {
                            admitted,
                            image,
                            summary,
                            events_forwarded: events,
                        },
                    },
                    output,
                ));
            }
            Err(error) => return Err(trace.failure(error)),
        }
    }
}

/// Preserve the already captured complete run on a later ordinary reporter error.
pub(super) fn attach_completed_trace(mut failure: CompileFailure, trace: &Value) -> CompileFailure {
    failure.actual_error = json!({"error":failure.actual_error,"complete_trace":trace});
    failure
}

#[cfg(test)]
mod tests {
    use super::super::complete_trace_wire::reconstruct_step_host_snapshot;
    use super::super::wire::OwnerWire;
    use super::*;
    use crate::sherlock::helper_machine::core_action_consumer::prepare_scalar_action_host;
    use crate::sherlock::helper_machine::shared::OwnerEngine;
    use crate::sherlock::helper_machine::shared::owner_host::host_owner_engine;
    use std::collections::BTreeMap;
    const CONFIG: &str =
        include_str!("../../../tests/sherlock_compile_artifact_positive/config.json");
    const TRACE_QUALIFICATION: &str =
        "actual complete owner records; no transition/native/target acceptance";
    fn prepared_with_limit(name: &str, serialization_bytes: usize) -> CoreActionPrepared {
        let source = format!(
            "language \"moriarty/1\"; agreement {name} {{ action q() {{ let x = 19 * 3; emit \"q\"(x + 2); }} }}"
        );
        let program = crate::parse(&source).unwrap();
        let mut config = super::super::parse_config(CONFIG)
            .ok()
            .unwrap()
            .consumer()
            .unwrap();
        config.bounds.serialization_bytes = serialization_bytes;
        config.layout.bounds.serialization_bytes = serialization_bytes;
        prepare_scalar_action_host(&program, 0, &BTreeMap::new(), &config)
            .ok()
            .unwrap()
    }
    fn prepared_with_fuel_and_limit(
        name: &str,
        fuel: u128,
        serialization_bytes: usize,
    ) -> CoreActionPrepared {
        let source = format!(
            "language \"moriarty/1\"; agreement {name} {{ action q() {{ let x = 19 * 3; emit \"q\"(x + 2); }} }}"
        );
        let program = crate::parse(&source).unwrap();
        let mut config = super::super::parse_config(CONFIG)
            .ok()
            .unwrap()
            .consumer()
            .unwrap();
        config.fuel = crate::sherlock::helper_machine::shared::RunIntC::Nat(fuel);
        config.bounds.serialization_bytes = serialization_bytes;
        config.layout.bounds.serialization_bytes = serialization_bytes;
        prepare_scalar_action_host(&program, 0, &BTreeMap::new(), &config)
            .ok()
            .unwrap()
    }
    fn prepared(name: &str) -> CoreActionPrepared {
        let source = format!(
            "language \"moriarty/1\"; agreement {name} {{ action q() {{ let x = 19 * 3; emit \"q\"(x + 2); }} }}"
        );
        let program = crate::parse(&source).unwrap();
        let config = super::super::parse_config(CONFIG)
            .ok()
            .unwrap()
            .consumer()
            .unwrap();
        prepare_scalar_action_host(&program, 0, &BTreeMap::new(), &config)
            .ok()
            .unwrap()
    }
    fn two_records() -> (CoreActionPrepared, Trace) {
        let mut pair = prepared("TraceBoundary");
        let initial = capture(&pair.admitted, &pair.image).unwrap();
        let mut trace = Trace::begin(initial).unwrap();
        for _ in 0..2 {
            match record_step(&pair.admitted, &mut pair.image) {
                Ok(ObservedStep::Event(record)) => trace.push_record(record).unwrap(),
                _ => panic!("real initial owner events required"),
            }
        }
        (pair, trace)
    }
    #[test]
    fn actual_ab_failure_retains_full_earlier_prefix_without_replay() {
        let (mut pair, trace) = two_records();
        let foreign = prepared("DifferentAdmission");
        let before = capture(&pair.admitted, &pair.image).unwrap();
        let prefix = trace.prefix();
        let error = match record_step(&foreign.admitted, &mut pair.image) {
            Err(error) => error,
            Ok(_) => panic!("foreign admission must fail AB binding"),
        };
        let failure = trace.failure(error);
        assert_eq!(failure.stage, "observation");
        assert_eq!(failure.events_forwarded, "2");
        assert!(failure.evaluation_performed);
        assert!(!failure.completed);
        assert_eq!(failure.actual_error["complete_trace"], prefix);
        assert_eq!(
            failure.actual_error["failure_boundary"],
            json!({"kind":"before-observation"})
        );
        assert!(failure.actual_error["violation"].is_object());
        assert_eq!(capture(&pair.admitted, &pair.image).unwrap(), before);
    }
    #[test]
    fn explicit_step_failure_projection_retains_both_available_boundaries() {
        // Projection branch control, not a claim this injected Step error is
        // reachable through an immutable well-formed public source pair.
        let (pair, trace) = two_records();
        let foreign = prepared("ForeignErrorValue");
        let violation = capture(&foreign.admitted, &pair.image).err().unwrap();
        let exact_error = violation.owner_wire();
        let before = capture(&pair.admitted, &pair.image).unwrap();
        let prefix = trace.prefix();
        let failure = trace.failure(CaptureStepError::Step {
            violation,
            before: Box::new(before.clone()),
            after: Ok(Box::new(before.clone())),
        });
        assert_eq!(failure.actual_error["violation"], exact_error);
        assert_eq!(failure.actual_error["complete_trace"], prefix);
        assert_eq!(
            failure.actual_error["failure_boundary"],
            json!({"kind":"step",
            "before":before.owner_wire(),"after":{"Ok":before.owner_wire()}})
        );
        assert_eq!(failure.events_forwarded, "2");
    }
    #[test]
    fn post_capture_failure_preserves_actual_event_outside_complete_prefix() {
        // Constructed projection branch using a genuine next owner event.
        // It does not establish a naturally occurring post-capture failure.
        let (mut pair, trace) = two_records();
        let before = capture(&pair.admitted, &pair.image).unwrap();
        let event = match host_owner_engine()
            .step(&pair.admitted, &mut pair.image)
            .unwrap()
        {
            crate::sherlock::helper_machine::shared::StepResult::Event { event } => event,
            _ => panic!("next real owner event required"),
        };
        let foreign = prepared("ForeignAfterError");
        let violation = capture(&foreign.admitted, &pair.image).err().unwrap();
        let exact_event = event.owner_wire();
        let failure = trace.failure(CaptureStepError::AfterObservation {
            violation,
            before: Box::new(before.clone()),
            event: Some(event),
        });
        assert_eq!(failure.events_forwarded, "3");
        assert_eq!(
            failure.actual_error["complete_trace"]["records"]
                .as_array()
                .unwrap()
                .len(),
            2
        );
        assert_eq!(
            failure.actual_error["failure_boundary"]["before"],
            before.owner_wire()
        );
        assert_eq!(
            failure.actual_error["failure_boundary"]["event"],
            exact_event
        );
        assert!(!failure.completed);
    }
    #[test]
    fn unavailable_failure_sides_and_initial_failure_are_explicit_not_success_defaults() {
        // Constructed projection controls only; no claim that the unchanged
        // well-formed public producer naturally reaches all defensive branches.
        let (pair, trace) = two_records();
        let foreign = prepared("ForeignMissingAfter");
        let violation = capture(&foreign.admitted, &pair.image).err().unwrap();
        let after_error = capture(&foreign.admitted, &pair.image).err().unwrap();
        let after_wire = after_error.owner_wire();
        let before = capture(&pair.admitted, &pair.image).unwrap();
        let failure = trace.failure(CaptureStepError::Step {
            violation,
            before: Box::new(before),
            after: Err(after_error),
        });
        assert_eq!(
            failure.actual_error["failure_boundary"]["after"],
            json!({"Err":after_wire})
        );
        assert!(
            failure.actual_error["failure_boundary"]["after"]
                .get("Ok")
                .is_none()
        );
        let (pair, trace) = two_records();
        let violation = capture(&foreign.admitted, &pair.image).err().unwrap();
        let before = capture(&pair.admitted, &pair.image).unwrap();
        let failure = trace.failure(CaptureStepError::AfterObservation {
            violation,
            before: Box::new(before),
            event: None,
        });
        assert_eq!(
            failure.actual_error["failure_boundary"]["event"],
            Value::Null
        );
        assert_eq!(failure.events_forwarded, "2");
        let violation = capture(&foreign.admitted, &pair.image).err().unwrap();
        let failure = initial_failure(
            violation,
            pair.observe().unwrap().admission.bounds.serialization_bytes,
        );
        assert_eq!(
            failure.actual_error["complete_trace"]["initial"],
            Value::Null
        );
        assert_eq!(failure.actual_error["complete_trace"]["records"], json!([]));
        assert!(!failure.evaluation_performed);
        assert!(!failure.completed);
        assert_eq!(failure.events_forwarded, "0");
    }
    #[test]
    fn bounded_writer_matches_owner_wire_bytes_and_enforces_zero_and_initial_limits() {
        let mut ref_pair = prepared("TraceBoundary");
        let ref_initial = capture(&ref_pair.admitted, &ref_pair.image).unwrap();
        let mut ref_records = Vec::new();
        for _ in 0..2 {
            if let Ok(ObservedStep::Event(record)) =
                record_step(&ref_pair.admitted, &mut ref_pair.image)
            {
                ref_records.push(record);
            }
        }
        let (_, trace) = two_records();
        let expected_prefix = json!({
            "format": TRACE_FORMAT,
            "initial": ref_initial.owner_wire(),
            "qualification": TRACE_QUALIFICATION,
            "records": ref_records.owner_wire(),
            "serialization_budget": TRACE_SERIALIZATION_BUDGET,
        });
        let expected_bytes = serde_json::to_vec(&expected_prefix).unwrap();
        assert_eq!(trace.prefix_bytes(), expected_bytes);
        assert_eq!(trace.prefix(), expected_prefix);
        assert_eq!(
            reconstruct_step_host_snapshot(
                &expected_prefix["initial"]["admission"],
                &expected_prefix["records"][0]["before"],
            ),
            ref_initial.owner_wire()
        );

        // Zero is a real zero bound, not unlimited: emits Null complete_trace (0 trace bytes).
        let zero_pair = prepared_with_limit("ZeroBudget", 0);
        let zero_failure = match run(zero_pair) {
            Err(failure) => failure,
            Ok(_) => panic!("zero serialization_bytes must refuse initial trace"),
        };
        assert_eq!(zero_failure.stage, "observation");
        assert!(!zero_failure.evaluation_performed);
        assert!(!zero_failure.completed);
        assert_eq!(zero_failure.events_forwarded, "0");
        assert_eq!(zero_failure.actual_error["complete_trace"], Value::Null);
        assert_eq!(
            zero_failure.actual_error["failure_boundary"]["kind"],
            "initial-serialization-limit-exceeded"
        );
        assert_eq!(
            zero_failure.actual_error["serialization_error"]["limit"],
            "0"
        );

        // Per-projection initial and returned boundaries via `run(prepared_with_fuel_and_limit(...))`.
        let (_, baseline_fuel0) = run(prepared_with_fuel_and_limit("InitialProbe", 0, 65_536))
            .ok()
            .unwrap();
        let init_bytes_65536 = serde_json::to_vec(&baseline_fuel0["initial"])
            .unwrap()
            .len();
        let base_init_snap = init_bytes_65536 - 2 * "65536".len();
        let exact_init_snap = (1usize..=65_536)
            .find(|c| *c == base_init_snap + 2 * c.to_string().len())
            .unwrap();
        let under_init_snap = match run(prepared_with_fuel_and_limit(
            "InitialProbe",
            0,
            exact_init_snap - 1,
        )) {
            Err(failure) => failure,
            Ok(_) => panic!("exact_init_snap - 1 must fail initial snapshot projection"),
        };
        assert!(!under_init_snap.evaluation_performed);
        assert!(!under_init_snap.completed);
        assert_eq!(under_init_snap.events_forwarded, "0");
        assert_eq!(under_init_snap.actual_error["complete_trace"], Value::Null);
        assert_eq!(
            under_init_snap.actual_error["failure_boundary"]["limit"],
            (exact_init_snap - 1).to_string()
        );

        // Cumulative trace_limit initial boundary (`exact_initial_trace - 1`): fails before step 1.
        let probe_pair = prepared("InitialProbe");
        let probe_initial = capture(&probe_pair.admitted, &probe_pair.image).unwrap();
        let probe_trace = Trace::begin(probe_initial).unwrap();
        let exact_initial_trace = probe_trace.prefix_byte_len();

        let under_initial =
            match run_with_limits(prepared("InitialProbe"), 65_536, exact_initial_trace - 1) {
                Err(failure) => failure,
                Ok(_) => panic!("exact_initial_trace - 1 must fail during initial serialization"),
            };
        assert!(!under_initial.evaluation_performed);
        assert_eq!(under_initial.events_forwarded, "0");
        assert_eq!(under_initial.actual_error["complete_trace"], Value::Null);
        assert!(
            serde_json::to_vec(&under_initial.actual_error)
                .unwrap()
                .len()
                <= exact_initial_trace - 1
        );

        // Exact whole-envelope boundary `L_0_env` (where `actual_error` with `initial` + 0 records fits).
        let probe_step1 = match run_with_limits(prepared("InitialProbe"), 65_536, 7_000) {
            Err(failure) => failure,
            Ok(_) => panic!("7_000 must fail on step 1 record serialization"),
        };
        let probe_env_len = serde_json::to_vec(&probe_step1.actual_error).unwrap().len();
        let base_env0 = probe_env_len - 2 * "7000".len();
        let exact_initial_env = (1usize..=65_536)
            .find(|c| *c == base_env0 + 2 * c.to_string().len())
            .unwrap();

        // At `exact_initial_env - 1`: step 1 executes, and `actual_error` rolls back `complete_trace` to Null
        // so the entire `actual_error` envelope stays `<= exact_initial_env - 1`.
        let env_minus_one =
            match run_with_limits(prepared("InitialProbe"), 65_536, exact_initial_env - 1) {
                Err(failure) => failure,
                Ok(_) => panic!("exact_initial_env - 1 must fail"),
            };
        assert!(env_minus_one.evaluation_performed);
        assert_eq!(env_minus_one.events_forwarded, "1");
        assert_eq!(env_minus_one.actual_error["complete_trace"], Value::Null);
        assert!(
            serde_json::to_vec(&env_minus_one.actual_error)
                .unwrap()
                .len()
                <= exact_initial_env - 1
        );

        // At `exact_initial_env` and `exact_initial_env + 1`: `initial` + 0 records fits inside `actual_error`.
        for limit in [exact_initial_env, exact_initial_env + 1] {
            let at_env = match run_with_limits(prepared("InitialProbe"), 65_536, limit) {
                Err(failure) => failure,
                Ok(_) => panic!("limit must fail on first step record serialization"),
            };
            assert!(at_env.evaluation_performed);
            assert!(!at_env.completed);
            assert_eq!(at_env.events_forwarded, "1");
            assert!(at_env.actual_error["complete_trace"]["initial"].is_object());
            assert_eq!(
                at_env.actual_error["complete_trace"]["records"]
                    .as_array()
                    .unwrap()
                    .len(),
                0
            );
            assert_eq!(
                serde_json::to_vec(&at_env.actual_error).unwrap().len(),
                exact_initial_env
            );
        }
    }
    #[test]
    fn record_returned_terminal_and_failure_boundary_limits_preserve_exact_prefix_and_counts() {
        // Probe 2-record whole-envelope boundary `L_2_env` (2 limit occurrences in `actual_error`).
        let (_, two_rec_trace) = two_records();
        let probe_two_limit = two_rec_trace.prefix_byte_len() + 1000;
        assert!(probe_two_limit <= 65_536);
        let probe_two_failure =
            match run_with_limits(prepared("TraceBoundary"), 65_536, probe_two_limit) {
                Err(failure) => failure,
                Ok(_) => panic!("probe_two_limit must fail on 3rd record"),
            };
        assert_eq!(
            probe_two_failure.actual_error["failure_boundary"]["preserved_records"],
            "2"
        );
        let probe_two_env_len = serde_json::to_vec(&probe_two_failure.actual_error)
            .unwrap()
            .len();
        let base_two_env = probe_two_env_len - 2 * probe_two_limit.to_string().len();
        let exact_two_env = (1usize..=65_536)
            .find(|c| *c == base_two_env + 2 * c.to_string().len())
            .unwrap();

        // `L_2_env - 1` rolls back to 1 preserved record so `actual_error` stays `<= L_2_env - 1`.
        let minus_one_two =
            match run_with_limits(prepared("TraceBoundary"), 65_536, exact_two_env - 1) {
                Err(failure) => failure,
                Ok(_) => panic!("exact_two_env - 1 must fail on 3rd record"),
            };
        assert_eq!(minus_one_two.events_forwarded, "3");
        assert_eq!(
            minus_one_two.actual_error["failure_boundary"]["preserved_records"],
            "1"
        );
        assert_eq!(
            minus_one_two.actual_error["complete_trace"]["records"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
        assert!(
            serde_json::to_vec(&minus_one_two.actual_error)
                .unwrap()
                .len()
                <= exact_two_env - 1
        );

        // `L_2_env` and `L_2_env + 1` preserve 2 records with exact whole-envelope byte length `L_2_env`.
        for limit in [exact_two_env, exact_two_env + 1] {
            let fail_on_third = match run_with_limits(prepared("TraceBoundary"), 65_536, limit) {
                Err(failure) => failure,
                Ok(_) => panic!("must fail on 3rd record"),
            };
            assert!(fail_on_third.evaluation_performed);
            assert!(!fail_on_third.completed);
            assert_eq!(fail_on_third.events_forwarded, "3");
            assert_eq!(
                fail_on_third.actual_error["complete_trace"]["records"]
                    .as_array()
                    .unwrap()
                    .len(),
                2
            );
            assert_eq!(
                fail_on_third.actual_error["failure_boundary"]["preserved_records"],
                "2"
            );
            assert!(fail_on_third.actual_error["failure_boundary"]["failed_step_id"].is_object());
            assert!(
                fail_on_third.actual_error["failure_boundary"]["failed_step_id"]["ordinal"]
                    .is_string()
            );
            assert!(fail_on_third.actual_error["failure_boundary"]["failed_rule_id"].is_string());
            assert!(
                fail_on_third.actual_error["failure_boundary"]["failed_disposition"].is_string()
            );
            assert_eq!(
                serde_json::to_vec(&fail_on_third.actual_error)
                    .unwrap()
                    .len(),
                exact_two_env
            );
        }

        // CaptureStepError::AfterObservation boundary overflow preserves executed event identity
        // (`failed_step_id`, `failed_rule_id`, `failed_disposition`) and bounds `actual_error <= limit`.
        let mut pair_step_err = prepared("TraceBoundary");
        let init_step_err = capture(&pair_step_err.admitted, &pair_step_err.image).unwrap();
        let mut trace_step_err =
            Trace::begin_with_limits(init_step_err, 65_536, exact_two_env + 64).unwrap();
        for _ in 0..2 {
            if let Ok(ObservedStep::Event(r)) =
                record_step(&pair_step_err.admitted, &mut pair_step_err.image)
            {
                trace_step_err.push_record(r).unwrap();
            }
        }
        let before_snap = capture(&pair_step_err.admitted, &pair_step_err.image).unwrap();
        let event_next = match host_owner_engine()
            .step(&pair_step_err.admitted, &mut pair_step_err.image)
            .unwrap()
        {
            crate::sherlock::helper_machine::shared::StepResult::Event { event } => event,
            _ => panic!("expected event"),
        };
        let expected_step_id = event_next.step_id.owner_wire();
        let expected_rule_id = event_next.rule_id.owner_wire();
        let expected_disposition = event_next.disposition.owner_wire();
        let foreign = prepared("ForeignBoundaryOverflow");
        let violation = capture(&foreign.admitted, &pair_step_err.image)
            .err()
            .unwrap();
        let boundary_overflow = trace_step_err.failure(CaptureStepError::AfterObservation {
            violation,
            before: Box::new(before_snap),
            event: Some(event_next),
        });
        assert_eq!(boundary_overflow.events_forwarded, "3");
        assert!(boundary_overflow.evaluation_performed);
        assert!(!boundary_overflow.completed);
        assert_eq!(
            boundary_overflow.actual_error["failure_boundary"]["kind"],
            "after-observation"
        );
        assert_eq!(
            boundary_overflow.actual_error["failure_boundary"]["event_present"],
            true
        );
        assert_eq!(
            boundary_overflow.actual_error["failure_boundary"]["failed_step_id"],
            expected_step_id
        );
        assert_eq!(
            boundary_overflow.actual_error["failure_boundary"]["failed_rule_id"],
            expected_rule_id
        );
        assert_eq!(
            boundary_overflow.actual_error["failure_boundary"]["failed_disposition"],
            expected_disposition
        );
        assert!(
            serde_json::to_vec(&boundary_overflow.actual_error)
                .unwrap()
                .len()
                <= exact_two_env + 64
        );

        // Full terminal completion boundary triplet (`L_full - 1`, `L_full`, `L_full + 1`)
        // using fuel = 0 (`1` record, normalized trace ~17 KB <= 65,536).
        let working_pair = prepared_with_fuel_and_limit("TraceBoundary", 0, 65_536);
        let (completed_run, working_trace) = run(working_pair).ok().expect("65536 must succeed");
        let n_records = completed_run.completed.events_forwarded as usize;
        assert_eq!(n_records, 1);
        let exact_full = serde_json::to_vec(&working_trace).unwrap().len();
        assert!(exact_full <= 65_536);

        for limit in [exact_full, exact_full + 1] {
            let (_, ok_trace) = run_with_limits(
                prepared_with_fuel_and_limit("TraceBoundary", 0, 65_536),
                65_536,
                limit,
            )
            .ok()
            .expect("exact_full and exact_full + 1 must succeed");
            assert_eq!(serde_json::to_vec(&ok_trace).unwrap().len(), exact_full);
        }

        let minus_one_failure = match run_with_limits(
            prepared_with_fuel_and_limit("TraceBoundary", 0, 65_536),
            65_536,
            exact_full - 1,
        ) {
            Err(failure) => failure,
            Ok(_) => panic!("exact_full - 1 must fail on terminal serialization"),
        };
        assert!(minus_one_failure.evaluation_performed);
        assert!(minus_one_failure.completed);
        assert_eq!(minus_one_failure.events_forwarded, n_records.to_string());
        assert_eq!(
            minus_one_failure.actual_error["failure_boundary"]["kind"],
            "terminal-serialization-limit-exceeded"
        );
        assert_eq!(
            minus_one_failure.actual_error["complete_trace"]["records"]
                .as_array()
                .unwrap()
                .len(),
            n_records
        );
        assert!(
            serde_json::to_vec(&minus_one_failure.actual_error)
                .unwrap()
                .len()
                <= exact_full - 1
        );

        // Verify lazy in-place checkpoint rollback on populated `writer` when `returned_checkpoint`
        // succeeds in `finish_terminal` (`returned_only_len <= tight_terminal_limit`) but `terminal`
        // fails AND `returned` + `actual_error` envelope exceeds `tight_terminal_limit`: rolls back
        // in-place from `returned_checkpoint` to `record_checkpoints[1]` (`1` preserved record),
        // never dropping `complete_trace` to `null`.
        let mut returned_only = working_trace.as_object().unwrap().clone();
        returned_only.remove("terminal");
        let returned_only_len = serde_json::to_vec(&Value::Object(returned_only))
            .unwrap()
            .len();
        let tight_terminal_limit = returned_only_len + 32;
        let rollback_from_terminal = match run_with_limits(
            prepared_with_fuel_and_limit("TraceBoundary", 0, 65_536),
            65_536,
            tight_terminal_limit,
        ) {
            Err(failure) => failure,
            Ok(_) => {
                panic!("tight_terminal_limit must fail on terminal write and roll back returned")
            }
        };
        assert!(rollback_from_terminal.evaluation_performed);
        assert!(rollback_from_terminal.completed);
        assert_eq!(rollback_from_terminal.events_forwarded, "1");
        assert_eq!(
            rollback_from_terminal.actual_error["failure_boundary"]["kind"],
            "terminal-serialization-limit-exceeded"
        );
        assert_eq!(
            rollback_from_terminal.actual_error["failure_boundary"]["preserved_records"],
            "1"
        );
        assert!(
            rollback_from_terminal.actual_error["complete_trace"]
                .get("returned")
                .is_none()
        );
        assert_eq!(
            rollback_from_terminal.actual_error["complete_trace"]["records"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
        assert!(
            serde_json::to_vec(&rollback_from_terminal.actual_error)
                .unwrap()
                .len()
                <= tight_terminal_limit
        );
    }
}
