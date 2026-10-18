use super::{Machine, Outcome, StepRecord};
use std::ops::ControlFlow;

/// Why a host step-observation call stopped; this does not change machine outcome or fuel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VisitStop {
    /// The requested number of events was observed without another stop condition.
    Budget,
    /// The machine published a terminal outcome, or had no next step.
    Terminal,
    /// The observer requested a stop after a nonterminal event.
    Observer,
}

/// Host observation metadata for one [`Machine::visit_steps`] call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VisitResult {
    pub events: usize,
    pub stop: VisitStop,
}

impl Machine {
    /// Visits at most `max_events` successive [`Self::step`] records from the current state.
    ///
    /// Each callback borrows the actual complete before/after snapshots, source-rule line
    /// and charge returned by `step()`. This method drops that record before advancing again
    /// and retains no history. The observer may return `ControlFlow::Break(())` to stop after
    /// an event. Terminal publication takes precedence over that request and counts as one
    /// event. A zero budget returns [`VisitStop::Budget`] without advancing or calling the
    /// observer, including when the machine is already terminal. Later calls resume this
    /// same machine, including any pending publication.
    ///
    /// # Security Warning (H1 — Unmetered Host Evaluator)
    /// This is a host observation grouping limit, not a fuel or host resource policy. The
    /// callback, snapshot cloning, pure helpers and host allocations remain unmetered in
    /// `sherlock-k-a/1`. This method does not make the reference evaluator safe for untrusted
    /// inputs, implement the B helper schedule, or supply circuit/native evidence.
    pub fn visit_steps(
        &mut self,
        max_events: usize,
        mut visitor: impl FnMut(&StepRecord) -> ControlFlow<()>,
    ) -> VisitResult {
        let mut events = 0;
        for _ in 0..max_events {
            let Some(record) = self.step() else {
                return VisitResult {
                    events,
                    stop: VisitStop::Terminal,
                };
            };
            let terminal = record.after.outcome != Outcome::Pending;
            let observed = visitor(&record);
            drop(record);
            events += 1;
            if terminal {
                return VisitResult {
                    events,
                    stop: VisitStop::Terminal,
                };
            }
            if observed.is_break() {
                return VisitResult {
                    events,
                    stop: VisitStop::Observer,
                };
            }
        }
        VisitResult {
            events,
            stop: VisitStop::Budget,
        }
    }
}
