//! Actual forwarded-event diagnostics, SOURCE_ONLY / UNCOMPILED / UNRUN.
//!
//! This observes the existing source caller; it never evaluates, replays,
//! validates, charges, retries or mutates its engine. Sums are not RunSummary
//! assertions. Maximum depth covers only observed event before/after fields.
use std::collections::BTreeMap;
use serde::Serialize;

use super::core_action_consumer::{run_scalar_action, ConsumerConfig, ConsumerError, CoreActionRun};
use super::resource_report::Counter;
use super::shared::StepEventC;

pub const FORMAT: &str = "moriarty-sherlock-b-event-accounting/1";

/// Overflow retains the last exact prefix BEFORE the failed addition. It is
/// explicitly not the full-stream total, a saturated total or a refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "status", rename_all = "kebab-case")]
pub enum CheckedTotal {
    Exact { value: Counter },
    Overflow { exact_prefix: Counter },
}
impl CheckedTotal {
    fn zero() -> Self { Self::Exact { value: Counter(0) } }
    fn add(&mut self, delta: u128) {
        if let Self::Exact { value } = *self {
            *self = match value.0.checked_add(delta) {
                Some(sum) => Self::Exact { value: Counter(sum) },
                None => Self::Overflow { exact_prefix: value },
            };
        }
    }
    fn is_exact(self) -> bool { matches!(self, Self::Exact { .. }) }
}

/// Rust caller status only: ReturnedOk is not application acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PrefixEnd { Open, ReturnedOk, ReturnedErr }
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ArithmeticStatus { Exact, IncompleteOverflow }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct EventAccountingReport {
    pub format: &'static str,
    pub prefix_end: PrefixEnd,
    pub arithmetic_status: ArithmeticStatus,
    pub event_count: CheckedTotal,
    pub work_delta_sum: CheckedTotal,
    pub helper_work_delta_sum: CheckedTotal,
    pub helper_visits_delta_sum: CheckedTotal,
    pub reference_cursor_delta_sum: CheckedTotal,
    pub maximum_observed_logical_depth: Option<usize>,
}

/// Fixed-size aggregate state. There is no event vector, rule-count map,
/// sampling, policy extent or resource input. Empty-prefix zero counts are
/// exact observations; empty-prefix depth remains None rather than inferred0.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EventAccounting {
    event_count: CheckedTotal,
    work_delta_sum: CheckedTotal,
    helper_work_delta_sum: CheckedTotal,
    helper_visits_delta_sum: CheckedTotal,
    reference_cursor_delta_sum: CheckedTotal,
    maximum_observed_logical_depth: Option<usize>,
}
impl EventAccounting {
    pub fn new() -> Self {
        Self { event_count: CheckedTotal::zero(), work_delta_sum: CheckedTotal::zero(),
            helper_work_delta_sum: CheckedTotal::zero(), helper_visits_delta_sum: CheckedTotal::zero(),
            reference_cursor_delta_sum: CheckedTotal::zero(), maximum_observed_logical_depth: None }
    }

    /// Account every event before passing it to the caller's sink. Arithmetic
    /// overflow is diagnostic state only; every other column/depth continues.
    pub fn observe(&mut self, event: &StepEventC) {
        self.event_count.add(1);
        self.work_delta_sum.add(u128::from(event.work_delta));
        self.helper_work_delta_sum.add(u128::from(event.helper_work_delta));
        self.helper_visits_delta_sum.add(u128::from(event.helper_visits_delta));
        self.reference_cursor_delta_sum.add(u128::from(event.reference_cursor_delta));
        let event_max = event.logical_depth_before.max(event.logical_depth_after);
        self.maximum_observed_logical_depth = Some(match self.maximum_observed_logical_depth {
            Some(previous) => previous.max(event_max), None => event_max,
        });
    }

    /// A snapshot of this observed prefix, not an owner invariant/validation.
    /// The wrapper derives prefix_end from the unchanged actual Result.
    pub fn snapshot(&self, prefix_end: PrefixEnd) -> EventAccountingReport {
        let exact = [self.event_count, self.work_delta_sum, self.helper_work_delta_sum,
            self.helper_visits_delta_sum, self.reference_cursor_delta_sum]
            .into_iter().all(CheckedTotal::is_exact);
        EventAccountingReport { format: FORMAT, prefix_end,
            arithmetic_status: if exact { ArithmeticStatus::Exact } else { ArithmeticStatus::IncompleteOverflow },
            event_count: self.event_count, work_delta_sum: self.work_delta_sum,
            helper_work_delta_sum: self.helper_work_delta_sum,
            helper_visits_delta_sum: self.helper_visits_delta_sum,
            reference_cursor_delta_sum: self.reference_cursor_delta_sum,
            maximum_observed_logical_depth: self.maximum_observed_logical_depth }
    }
}

/// Original source result and separate observed-prefix accounting. On Err,
/// the original ConsumerError (including its own prefix metadata) is retained
/// untouched. On Ok, the actual RunSummary remains in run_result; no sum is
/// compared to, substituted for or asserted against that summary here.
pub struct AccountedScalarRun {
    pub run_result: Result<CoreActionRun, ConsumerError>,
    pub event_accounting: EventAccountingReport,
}

/// Additive wrapper: actual existing caller once, same sink order/delivery.
/// Neither sink panics nor engine panics are caught or translated; as before,
/// unwinding does not promise a returned report. Wrapper/accounting costs are
/// not owner work or measured physical/circuit/financial/preparation costs.
pub fn run_scalar_action_with_event_accounting(
    source: &crate::Program,
    action_index: usize,
    inputs: &BTreeMap<String, crate::Value>,
    config: &ConsumerConfig,
    sink: &mut dyn FnMut(&StepEventC),
) -> AccountedScalarRun {
    let (run_result, event_accounting) = collect_forwarded(sink, |forward| {
        run_scalar_action(source, action_index, inputs, config, forward)
    });
    AccountedScalarRun { run_result, event_accounting }
}

// One small forwarding function shared by the actual wrapper and literal-event
// unit tests. It is private: no alternative executor is registered or exposed.
fn collect_forwarded<T, E>(sink: &mut dyn FnMut(&StepEventC),
    operation: impl FnOnce(&mut dyn FnMut(&StepEventC)) -> Result<T, E>)
    -> (Result<T, E>, EventAccountingReport)
{
    let mut accounting = EventAccounting::new();
    let result = {
        let mut forward = |event: &StepEventC| {
            accounting.observe(event);
            sink(event);
        };
        operation(&mut forward)
    };
    let end = if result.is_ok() { PrefixEnd::ReturnedOk } else { PrefixEnd::ReturnedErr };
    (result, accounting.snapshot(end))
}

#[cfg(test)]
#[path = "resource_event_accounting_tests.rs"]
mod tests;
