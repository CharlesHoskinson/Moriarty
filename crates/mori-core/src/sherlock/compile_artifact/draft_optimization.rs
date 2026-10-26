//! UIREQ-024 replay of the `sherlock-a-intern-equal-tree-nodes/1` candidate on
//! a dedicated draft source.
//!
//! The source is prepared exactly as `draft_reference_slices` does and its
//! retained checked program runs twice on the host K-A `Machine` with the
//! config's fuel and stack: once unchanged (reference) and once realized by
//! the candidate (tree literals interned; every boundary state re-interned
//! before the machine resumes from it). `check_preservation` must accept the
//! pair over all six declared observation planes before anything is reported;
//! a divergence is a failure, never a report. The B owner execution of the
//! same command is not changed by this candidate. Host-only diagnostic
//! observation: no circuit, proof, native or acceptance claim.
use serde_json::{Value, json};
use std::sync::Arc;

use super::CompileFailure;
use crate::sherlock::helper_machine::shared::RunIntC;
use crate::sherlock::optimization_intern::{
    self as intern, Observation, TreeInterner, check_preservation,
};
use crate::sherlock::reference_display::KDisplay;
use crate::sherlock::{Outcome, PROFILE, Value as KValue};

pub const RUN_FORMAT: &str = "moriarty-sherlock-optimization-run/1";
pub const CANDIDATE: &str = intern::CANDIDATE_ID;

fn run_int(value: RunIntC, field: &'static str) -> Result<u128, CompileFailure> {
    match value {
        RunIntC::Nat(v) => Ok(v),
        _ => Err(CompileFailure::input(
            "config",
            format!("{field} must be a natural u128 for the K-A reference machine"),
        )),
    }
}

fn outcome(o: &Outcome) -> String {
    match o {
        Outcome::Pending => "Pending".into(),
        Outcome::Done(KValue::UInt(n)) => format!("Done({n})"),
        Outcome::Done(v) => format!("Done({})", v.to_k()),
        Outcome::Rejected(c) => format!("Rejected({c})"),
        Outcome::Exhausted => "Exhausted".into(),
    }
}

fn pair<T: ToString>(reference: T, candidate: T) -> Value {
    json!({"reference": reference.to_string(), "candidate": candidate.to_string()})
}

fn work(o: &Observation) -> u128 {
    o.records.iter().map(|r| u128::from(r.charge)).sum()
}

fn max_depth(o: &Observation) -> usize {
    o.records
        .iter()
        .map(|r| r.after.frames.len())
        .chain(std::iter::once(o.initial.frames.len()))
        .max()
        .unwrap_or(0)
}

/// Replay `candidate` against the K-A reference run of `source`.
pub fn optimization_report(
    source: &str,
    config_source: &str,
    target_source: &str,
    candidate: &str,
) -> Result<Value, CompileFailure> {
    if candidate != CANDIDATE {
        return Err(CompileFailure::input(
            "config",
            format!("unknown optimization candidate {candidate:?}; supported: {CANDIDATE}"),
        ));
    }
    let config = super::parse_draft_config(config_source)?;
    let fuel = run_int(config.fuel.owner(), "fuel")?;
    let stack = usize::try_from(run_int(config.stack.owner(), "stack")?)
        .map_err(|_| CompileFailure::input("config", "stack exceeds usize"))?;
    let (pair_, _, _) = super::prepare_draft_pair(source, config_source, target_source)?;
    let program = pair_.preparation.draft.program.clone();
    // At most fuel + 1 events (one free publication); a defensive bound only.
    let max_events = usize::try_from(fuel.saturating_add(1)).unwrap_or(usize::MAX);
    let reference = intern::observe_reference(Arc::clone(&program), fuel, stack, max_events);
    let mut interner = TreeInterner::new();
    let realized = intern::observe_candidate(&program, fuel, stack, max_events, &mut interner);
    let kept = check_preservation(&reference, &realized).map_err(|d| {
        CompileFailure::input(
            "observation",
            format!(
                "candidate {CANDIDATE} diverged on {} at event {:?}: {}",
                d.clause.name(),
                d.event,
                d.detail
            ),
        )
    })?;
    let (rc, cc) = (reference.peak_census(), realized.peak_census());
    Ok(json!({
        "format": RUN_FORMAT,
        "candidate": CANDIDATE,
        "realization": format!("{PROFILE} host reference run of the same draft source; B owner execution unchanged"),
        "qualification": "HostOnly unmetered K-A machine; no circuit, proof, native or acceptance claim",
        "applied": interner.distinct_nodes() > 0,
        "interned_nodes": interner.distinct_nodes().to_string(),
        "preservation": {
            "entry": "check_preservation",
            "status": "preserved",
            "clauses": intern::Clause::ALL.iter().map(|c| c.name()).collect::<Vec<_>>(),
            "duties": if kept.duties_vacuous { "vacuous-by-profile" } else { "compared" },
        },
        "logical": {
            "events": pair(reference.records.len(), realized.records.len()),
            "work": pair(work(&reference), work(&realized)),
            "outcome": pair(outcome(&reference.final_state.outcome), outcome(&realized.final_state.outcome)),
            "remaining_fuel": pair(reference.final_state.remaining_fuel, realized.final_state.remaining_fuel),
            "max_depth": pair(max_depth(&reference), max_depth(&realized)),
            "peak_tree_child_occurrences": pair(rc.structural_children, cc.structural_children),
        },
        "physical": {
            "peak_tree_child_allocations": pair(rc.physical_children, cc.physical_children),
        },
    }))
}
