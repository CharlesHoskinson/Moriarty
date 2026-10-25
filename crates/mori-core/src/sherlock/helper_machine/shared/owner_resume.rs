//! Technical-cut resume of a complete host checkpoint (child of `shared`).
//!
//! A technical chunk boundary stops the owner between two events and continues
//! from a complete `HostSnapshot` instead of the live image. This module is the
//! only owner-side constructor of such an image. It rebuilds every
//! `MachineImage` field from the snapshot, binds it to the caller's existing
//! `Admitted` (the same `Arc`, so AB holds by construction), then refuses
//! unless:
//! - the admission is finance `Absent` (a financial image has no complete
//!   capture beyond genesis, so it has no checkpoint to resume);
//! - the snapshot's `seg_base` is the uncut genesis value (a technical cut
//!   creates no financial segment or commit; SI-05 still applies);
//! - the owner step invariants SI01..SI24 hold on the rebuilt image; and
//! - capturing the rebuilt pair reproduces the snapshot exactly, admission and
//!   image, so no field was dropped, normalized or invented.
//!
//! This is host diagnostic tooling for checkpoints the caller produced from
//! its own run. It is not a trust boundary for adversarial checkpoints, a
//! canonical encoding, a circuit relation or the future suspension extension
//! (a waiting callee resumed by an external observation).
use super::super::complete_observation::{DataSnapshot, HostSnapshot, capture};
use super::super::raw::{CtorId, DataRow as RawDataRow, FieldPos, FieldSlice, TypeId};
use super::host_implied_prefix::SegBase;
use super::owner_host::host_owner_engine;
use super::*;

/// Why a checkpoint was not resumed. Every variant leaves the caller's live
/// image untouched; nothing is partially resumed.
#[derive(Debug)]
pub enum ResumeError {
    /// Financial admissions have no complete checkpoint beyond genesis.
    FinancialUnsupported,
    /// A technical cut never stores a nonzero segment base.
    SegmentBaseNotGenesis,
    /// AB binding or SI01..SI24 refused the rebuilt image.
    StepInvariant(Vec<String>),
    /// The rebuilt pair could not be observed.
    Capture(String),
    /// The rebuilt pair's capture differs from the checkpoint.
    RoundTripMismatch { field: &'static str },
}

impl std::fmt::Display for ResumeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FinancialUnsupported => f.write_str("financial checkpoint resume is unsupported"),
            Self::SegmentBaseNotGenesis => {
                f.write_str("technical checkpoint carries a nonzero segment base")
            }
            Self::StepInvariant(violations) => {
                write!(
                    f,
                    "step invariants refused the resumed image: {violations:?}"
                )
            }
            Self::Capture(message) => write!(f, "resumed image capture failed: {message}"),
            Self::RoundTripMismatch { field } => {
                write!(f, "resumed image differs from the checkpoint at {field}")
            }
        }
    }
}

fn raw_data(row: &DataSnapshot) -> RawDataRow {
    let slice = |start: usize, len: usize| FieldSlice {
        start: FieldPos(start),
        len,
    };
    match row {
        DataSnapshot::U128 { n } => RawDataRow::U128 { n: *n },
        DataSnapshot::Bool { b } => RawDataRow::Bool { b: *b },
        DataSnapshot::Nil => RawDataRow::Nil,
        DataSnapshot::Cons { fields } => RawDataRow::Cons {
            fields: slice(fields.start, fields.len),
        },
        DataSnapshot::Leaf { fields } => RawDataRow::Leaf {
            fields: slice(fields.start, fields.len),
        },
        DataSnapshot::Branch { fields } => RawDataRow::Branch {
            fields: slice(fields.start, fields.len),
        },
        DataSnapshot::Constructor {
            type_id,
            ctor_id,
            fields,
        } => RawDataRow::Constructor {
            type_id: TypeId(type_id.0),
            ctor_id: CtorId(ctor_id.0),
            fields: slice(fields.start, fields.len),
        },
    }
}

/// Rebuild the complete resumable image of `checkpoint` on `admitted`.
pub fn resume_technical_checkpoint(
    admitted: &Admitted,
    checkpoint: &HostSnapshot,
) -> Result<MachineImage, ResumeError> {
    let body = &admitted.body;
    if body.fin.is_some() || body.identity.finance != FinanceInput::Absent {
        return Err(ResumeError::FinancialUnsupported);
    }
    let s = &checkpoint.image;
    if s.seg_base != SegBase::genesis_uncut() {
        return Err(ResumeError::SegmentBaseNotGenesis);
    }
    let image = MachineImage {
        admission: admitted.clone(),
        run_id: Arc::new(s.run_id.clone()),
        control: s.control,
        control_origin: s.control_origin,
        env_root: s.env_root,
        stack_limit: s.stack_limit,
        outcome: s.outcome,
        initial_fuel: s.initial_fuel,
        remaining_work: s.remaining_work,
        consumed_work: s.consumed_work,
        helper_work: s.helper_work,
        helper_visits: s.helper_visits,
        reference_cursor: s.reference_cursor,
        alloc: s.alloc,
        active: s.active,
        seg_base: s.seg_base,
        helper: s.helper,
        result: s.result,
        fin: None,
        data: s.data.iter().map(raw_data).collect(),
        data_fields: s.data_fields.clone(),
        env: s.env.clone(),
        frames: s.frames.clone(),
        tasks: s.tasks.clone(),
        value_slots: s.value_slots.clone(),
    };
    let pre = PreImage::bind(admitted, &image)
        .map_err(|violation| ResumeError::StepInvariant(vec![format!("{violation:?}")]))?;
    host_owner_engine()
        .check_step_inv(&pre)
        .map_err(|violations| {
            ResumeError::StepInvariant(
                violations
                    .iter()
                    .map(|v| format!("{:?} {:?} {}", v.clause, v.locator, v.detail))
                    .collect(),
            )
        })?;
    let observed = capture(admitted, &image)
        .map_err(|violation| ResumeError::Capture(format!("{violation:?}")))?;
    if observed.admission != checkpoint.admission {
        return Err(ResumeError::RoundTripMismatch { field: "admission" });
    }
    if observed.image != checkpoint.image {
        return Err(ResumeError::RoundTripMismatch { field: "image" });
    }
    Ok(image)
}
