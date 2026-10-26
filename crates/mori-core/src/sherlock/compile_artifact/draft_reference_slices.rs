//! Earliest-stop `sherlock-k-a/1` ReferenceSlices of a dedicated draft source.
//!
//! The source is prepared exactly as `compile_draft_with_reference` does; the
//! retained checked reference program runs on the host K-A `Machine` with the
//! config's fuel and stack values. Reference events are grouped by the
//! existing `advance_m` (at most `m` per slice, stopping at the first published
//! outcome). Every technical cut continues on a fresh `Machine` rebuilt from
//! the previous slice's complete end `Snapshot`, so a cut really resumes.
//!
//! Each record keeps its internal and global position, K rule line, charge,
//! slice-prefix and cumulative work, logical frame check and whether it is the
//! slice's first outcome. Host grouping adds no padding, no reference record,
//! no charge and no financial commit. The host K-A machine is unmetered (H1);
//! this is diagnostic observation, not a circuit, proof or B-image claim.
use serde_json::{Value, json};

use super::CompileFailure;
use crate::sherlock::helper_machine::shared::RunIntC;
use crate::sherlock::optimization_intern::TreeInterner;
use crate::sherlock::reference_display::{KDisplay, environment_to_k};
use crate::sherlock::reference_slice::{
    ReferenceSlice, SliceBoundaryKind, SliceStop, advance_m,
    cumulative_resource_report_with_manifest,
};
use crate::sherlock::{Machine, Outcome, PROFILE, Snapshot};

pub const FORMAT: &str = "moriarty-sherlock-reference-slices/1";

fn outcome(o: &Outcome) -> Value {
    match o {
        Outcome::Pending => json!({"kind": "Pending"}),
        Outcome::Done(v) => json!({"kind": "Done", "value": v.to_k()}),
        Outcome::Rejected(code) => json!({"kind": "Rejected", "code": code.to_string()}),
        Outcome::Exhausted => json!({"kind": "Exhausted"}),
    }
}

fn snapshot(s: &Snapshot) -> Value {
    json!({
        "control": s.control.as_ref().map(KDisplay::to_k),
        "environment": environment_to_k(&s.environment),
        "frames": s.frames.iter().map(KDisplay::to_k).collect::<Vec<_>>(),
        "logical_depth": s.frames.len().to_string(),
        "stack_limit": s.stack_limit.to_string(),
        "remaining_fuel": s.remaining_fuel.to_string(),
        "initial_fuel": s.initial_fuel.to_string(),
        "outcome": outcome(&s.outcome),
    })
}

fn run_int(value: RunIntC, field: &'static str) -> Result<u128, CompileFailure> {
    match value {
        RunIntC::Nat(v) => Ok(v),
        _ => Err(CompileFailure::input(
            "config",
            format!("{field} must be a natural u128 for the K-A reference machine"),
        )),
    }
}

fn slice_wire(
    index: usize,
    first_event: u128,
    cumulative_before: u128,
    s: &ReferenceSlice,
) -> Value {
    let mut first_outcome_seen = false;
    let records: Vec<Value> = s
        .records
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let published = r.after.outcome != Outcome::Pending;
            let first_outcome = published && !first_outcome_seen;
            first_outcome_seen |= published;
            json!({
                "internal_position": (i + 1).to_string(),
                "position": (first_event + i as u128 + 1).to_string(),
                "rule_start_line": r.rule_start_line.to_string(),
                "source_control": r.before.control.as_ref().map(KDisplay::to_k),
                "charge": r.charge.to_string(),
                "prefix_work": s.prefix_work[i + 1].to_string(),
                "cumulative_work": (cumulative_before + s.prefix_work[i + 1]).to_string(),
                "frame_check": {
                    "depth_before": r.before.frames.len().to_string(),
                    "depth_after": r.after.frames.len().to_string(),
                    "stack_limit": r.after.stack_limit.to_string(),
                    "within_limit": r.after.frames.len() <= r.after.stack_limit,
                },
                "first_outcome": first_outcome,
                "outcome_after": outcome(&r.after.outcome),
            })
        })
        .collect();
    json!({
        "index": index.to_string(),
        "profile_id": s.profile_id,
        "start_event": first_event.to_string(),
        "records": records,
        "prefix_work": s.prefix_work.iter().map(u128::to_string).collect::<Vec<_>>(),
        "boundaries": s.boundaries.iter().map(|b| json!({
            "position": b.position.to_string(),
            "kind": match b.kind {
                SliceBoundaryKind::Terminal => "terminal",
                SliceBoundaryKind::TechnicalCut => "technical-cut",
            },
        })).collect::<Vec<_>>(),
        "stop": match &s.stop {
            SliceStop::Outcome(o) => json!({"kind": "outcome", "outcome": outcome(o)}),
            SliceStop::TechnicalCut => json!({"kind": "technical-cut", "financial_commit": "none"}),
        },
        "start": snapshot(&s.start),
        "end": snapshot(&s.end),
        "padding": {"reference_records": "0", "charge": "0"},
    })
}

/// Group the K-A reference run of `source` into slices of at most `m` events.
pub fn reference_slices(
    source: &str,
    config_source: &str,
    target_source: &str,
    m: usize,
) -> Result<Value, CompileFailure> {
    slices_with(source, config_source, target_source, m, None)
}

/// The same grouping realized by the UIREQ-024 intern-equal-tree-nodes
/// candidate: the program's tree literals are interned and every technical cut
/// resumes from the re-interned end snapshot. The report must equal
/// [`reference_slices`] (interning preserves structural equality).
pub fn reference_slices_with_candidate(
    source: &str,
    config_source: &str,
    target_source: &str,
    m: usize,
) -> Result<Value, CompileFailure> {
    let mut interner = TreeInterner::new();
    slices_with(source, config_source, target_source, m, Some(&mut interner))
}

fn slices_with(
    source: &str,
    config_source: &str,
    target_source: &str,
    m: usize,
    mut interner: Option<&mut TreeInterner>,
) -> Result<Value, CompileFailure> {
    if m == 0 {
        return Err(CompileFailure::input(
            "config",
            "--reference-slices must be at least 1",
        ));
    }
    let config = super::parse_draft_config(config_source)?;
    let fuel = run_int(config.fuel.owner(), "fuel")?;
    let stack = usize::try_from(run_int(config.stack.owner(), "stack")?)
        .map_err(|_| CompileFailure::input("config", "stack exceeds usize"))?;
    let (pair, _, _) = super::prepare_draft_pair(source, config_source, target_source)?;
    let mut program = pair.preparation.draft.program.clone();
    if let Some(i) = interner.as_deref_mut() {
        program = std::sync::Arc::new(i.intern_program(&program));
    }
    // Each event charges one fuel unit except the single free publication, so
    // the run has at most fuel + 1 events; the cap is a defensive bound only.
    let max_events = fuel.saturating_add(1);
    let mut machine = Machine::new(program, fuel, stack);
    let mut slices: Vec<ReferenceSlice> = Vec::new();
    let mut events: u128 = 0;
    let mut resumed_cuts: u128 = 0;
    loop {
        let slice = advance_m(&mut machine, m);
        let produced = slice.records.len() as u128;
        if produced == 0 {
            return Err(CompileFailure::input(
                "evaluation",
                "reference slice made no progress",
            ));
        }
        events += produced;
        if events > max_events {
            return Err(CompileFailure::input(
                "evaluation",
                "reference run exceeded fuel + 1 events",
            ));
        }
        let cut = slice.stop == SliceStop::TechnicalCut;
        if cut {
            // Resume the next slice from the complete end snapshot only.
            machine = Machine::from_snapshot(match interner.as_deref_mut() {
                Some(i) => i.intern_snapshot(&slice.end),
                None => slice.end.clone(),
            });
            resumed_cuts += 1;
        }
        slices.push(slice);
        if !cut {
            break;
        }
    }
    let refs: Vec<&ReferenceSlice> = slices.iter().collect();
    let cumulative = cumulative_resource_report_with_manifest(&refs).map_err(|error| {
        CompileFailure::input("observation", format!("cumulative report: {error:?}"))
    })?;
    let mut wires = Vec::with_capacity(slices.len());
    let mut first_event = 0u128;
    let mut work = 0u128;
    for (i, s) in slices.iter().enumerate() {
        wires.push(slice_wire(i, first_event, work, s));
        first_event += s.records.len() as u128;
        work += s.prefix_work.last().copied().unwrap_or(0);
    }
    let last = slices.last().expect("at least one slice");
    Ok(json!({
        "format": FORMAT,
        "qualification": "HostOnly sherlock-k-a/1 reference grouping; unmetered host machine; no circuit, proof or B-image claim",
        "reference_schedule_id": PROFILE,
        "group_budget": m.to_string(),
        "fuel": fuel.to_string(),
        "stack_limit": stack.to_string(),
        "events": events.to_string(),
        "reference_work": work.to_string(),
        "technical_cuts": resumed_cuts.to_string(),
        "financial_commits": "0",
        "outcome": outcome(&last.end.outcome),
        "slices": wires,
        "cumulative": serde_json::to_value(&cumulative).unwrap_or(Value::Null),
    }))
}
