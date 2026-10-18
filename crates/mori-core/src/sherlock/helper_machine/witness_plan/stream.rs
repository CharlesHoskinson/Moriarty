//! Complete record streaming around the SAME fixed-owner observation interface.
//!
//! The library retains no record history. The callback sees a complete borrowed
//! record, then its raw/materialized buffers are dropped before the next step.
//! A retaining caller/sink can still retain everything; no measured memory/time
//! improvement or semantic-prefix/clock theorem is implied by this interface.
use super::{
    construct_plan_for, evaluate_with_storage, HostSnapshot, LifetimeUnit,
    SlotStorage, StrategyEvaluation, WitnessCache, WitnessFailure, WitnessPlan,
};
use crate::sherlock::helper_machine::complete_observation::{
    self as observation, CaptureStepError, ObservedStep, WitnessSide,
};
use crate::sherlock::helper_machine::shared::{Admitted, MachineImage};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StrategyMaterializedSnapshot {
    pub raw: HostSnapshot,
    pub plan: WitnessPlan,
    pub evaluation: StrategyEvaluation,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StrategyObservedStep {
    pub observed: ObservedStep,
    pub before: StrategyMaterializedSnapshot,
    pub after: StrategyMaterializedSnapshot,
}
pub enum StrategyStepError {
    Owner(CaptureStepError),
    Materialization {
        side: WitnessSide,
        failure: WitnessFailure,
        observed: Box<ObservedStep>,
        completed_before: Option<Box<StrategyMaterializedSnapshot>>,
    },
}
/// Separate before/after lifetime namespaces retain one graph each, not history.
#[derive(Default)]
pub struct WitnessRecordCache { before: WitnessCache, after: WitnessCache }
impl WitnessRecordCache {
    pub fn clear(&mut self) { self.before.clear(); self.after.clear(); }
}

pub fn materialize_with_storage(input: &HostSnapshot, lifetime: LifetimeUnit,
    storage: SlotStorage, cache: Option<&mut WitnessCache>)
    -> Result<StrategyMaterializedSnapshot, WitnessFailure>
{
    let plan = construct_plan_for(input, lifetime)?;
    let evaluation = evaluate_with_storage(&plan, input, storage, cache)?;
    Ok(StrategyMaterializedSnapshot { raw: input.clone(), plan, evaluation })
}

/// One real owner step, followed solely by reconstruction of its saved raw
/// boundaries. On any tooling failure, the actual step and available boundaries
/// are returned unchanged; no rollback, retry or extra transition is performed.
pub fn record_step_with_storage(admitted: &Admitted, image: &mut MachineImage,
    storage: SlotStorage, mut cache: Option<&mut WitnessRecordCache>)
    -> Result<StrategyObservedStep, StrategyStepError>
{
    let observed = observation::record_step(admitted, image).map_err(StrategyStepError::Owner)?;
    let (before_raw, after_raw) = match &observed {
        ObservedStep::Event(record) => (&record.before, &record.after),
        ObservedStep::Terminal { before, after } => (before, after),
    };
    let before_cache = cache.as_deref_mut().map(|memo| &mut memo.before);
    let before = match materialize_with_storage(before_raw, LifetimeUnit::RecordBefore, storage, before_cache) {
        Ok(value) => value,
        Err(failure) => return Err(StrategyStepError::Materialization {
            side: WitnessSide::Before, failure, observed: Box::new(observed),
            completed_before: None,
        }),
    };
    let after_cache = cache.as_deref_mut().map(|memo| &mut memo.after);
    let after = match materialize_with_storage(after_raw, LifetimeUnit::RecordAfter, storage, after_cache) {
        Ok(value) => value,
        Err(failure) => return Err(StrategyStepError::Materialization {
            side: WitnessSide::After, failure, observed: Box::new(observed),
            completed_before: Some(Box::new(before)),
        }),
    };
    Ok(StrategyObservedStep { observed, before, after })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StreamAction { Continue, Stop }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StreamStop { Sink, Terminal }

/// Yield every complete observed record/terminal boundary in order. The sink
/// may stop after a delivered boundary, preserving that exact caller-owned
/// MachineImage. No inferred work/event, hidden step, image substitution or
/// application exhaustion is introduced. Execution classification remains the
/// existing caller's obligation, independently of this source-only interface.
pub fn stream_owner_steps<F>(admitted: &Admitted, image: &mut MachineImage,
    mut cache: Option<&mut WitnessRecordCache>, mut sink: F)
    -> Result<StreamStop, StrategyStepError>
where F: FnMut(&StrategyObservedStep) -> StreamAction
{
    loop {
        let step = record_step_with_storage(admitted, image, SlotStorage::LastUse, cache.as_deref_mut())?;
        let terminal = matches!(&step.observed, ObservedStep::Terminal { .. });
        let action = sink(&step);
        // No decoded/raw record history is held across loop iterations.
        drop(step);
        if terminal { return Ok(StreamStop::Terminal); }
        if action == StreamAction::Stop { return Ok(StreamStop::Sink); }
    }
}
