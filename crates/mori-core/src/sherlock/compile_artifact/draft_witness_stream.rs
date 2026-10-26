//! G5 host witness stream over the real `sherlock-k-a/1` reference machine.
//!
//! Emits, for every executed machine step, the rule that fired and the
//! complete pre/post states in the same canonical snapshot encoding the
//! reference-slices report uses, each with a host digest (`pre_d`/`post_d`)
//! of its canonical JSON. The final entry carries the terminal outcome and
//! result. This is a host observation stream for the C6 leaf consumer: the
//! digests are SHA-256 over canonical snapshot JSON, not the Poseidon lane
//! digests the circuit side pins; mapping lanes to field digests stays with
//! the leaf producer. Nothing here proves, emits a circuit, or claims a
//! Midnight/ZKIR artifact. See the H1 unmetered-host warning on
//! [`Machine::visit_steps`](crate::sherlock::Machine::visit_steps).
use serde_json::{Value, json};

use super::CompileFailure;
use crate::sherlock::reference_display::{KDisplay, environment_to_k};
use crate::sherlock::{Machine, Outcome, PROFILE, Snapshot};

pub const WITNESS_STREAM_FORMAT: &str = "moriarty-sherlock-witness-stream/1";

/// (K rule line, name) for every rule in the frozen K-A inventory; the line
/// numbers refer only to `K_SOURCE_SHA256`, as in the residual rule map.
const RULE_NAMES: &[(usize, &str)] = &[
    (80, "PublishDone"),
    (82, "PublishError"),
    (84, "PublishExhausted"),
    (102, "EvalLiteral"),
    (108, "EvalVar"),
    (111, "EvalVarUnbound"),
    (114, "EvalLet"),
    (117, "EvalBinary"),
    (120, "EvalUnary"),
    (123, "EvalIf"),
    (126, "EvalMatch"),
    (129, "EvalCallNoArgs"),
    (131, "EvalCallArgs"),
    (135, "PushFrame"),
    (140, "PushFrameCap"),
    (145, "ReturnLet"),
    (149, "ReturnLeft"),
    (153, "ReturnRight"),
    (157, "ReturnUnary"),
    (161, "ReturnIfTrue"),
    (165, "ReturnIfFalse"),
    (169, "ReturnIfNotBool"),
    (173, "ReturnMatch"),
    (177, "ReturnArgNext"),
    (181, "ReturnArgLast"),
    (185, "ReturnTyped"),
    (190, "ReturnTypeMismatch"),
    (195, "ReturnScope"),
    (200, "Invoke"),
    (202, "EnterMissing"),
    (204, "EnterBind"),
    (206, "BoundFailed"),
    (208, "BoundPush"),
    (212, "CasesEmpty"),
    (214, "CasesTry"),
    (217, "CaseMiss"),
    (220, "CaseHit"),
    (223, "PrimitiveOk"),
    (225, "PrimitiveErr"),
];

fn rule_name(line: usize) -> &'static str {
    RULE_NAMES
        .iter()
        .find_map(|&(candidate, name)| (candidate == line).then_some(name))
        .unwrap_or("UnknownRule")
}

fn outcome_value(outcome: &Outcome) -> Value {
    match outcome {
        Outcome::Pending => json!({"kind": "Pending"}),
        Outcome::Done(value) => json!({"kind": "Done", "value": value.to_k()}),
        Outcome::Rejected(code) => json!({"kind": "Rejected", "code": code.to_string()}),
        Outcome::Exhausted => json!({"kind": "Exhausted"}),
    }
}

/// Canonical snapshot encoding plus its host digest. The digest covers the
/// canonical JSON of the encoding without the digest field itself.
fn snapshot_value(snapshot: &Snapshot) -> Result<Value, CompileFailure> {
    let mut value = json!({
        "control": snapshot.control.as_ref().map(KDisplay::to_k),
        "environment": environment_to_k(&snapshot.environment),
        "frames": snapshot.frames.iter().map(KDisplay::to_k).collect::<Vec<_>>(),
        "logical_depth": snapshot.frames.len().to_string(),
        "stack_limit": snapshot.stack_limit.to_string(),
        "remaining_fuel": snapshot.remaining_fuel.to_string(),
        "initial_fuel": snapshot.initial_fuel.to_string(),
        "outcome": outcome_value(&snapshot.outcome),
    });
    let canonical = crate::intent::canonical(&value)
        .map_err(|error| CompileFailure::input("serialization", error.to_string()))?;
    value["digest"] = Value::String(crate::intent::hash_bytes(&canonical));
    Ok(value)
}

/// Run the draft program under its host config and emit the per-step
/// witness stream described in the module docs.
pub fn witness_stream(
    source: &str,
    config_source: &str,
    target_source: &str,
) -> Result<Value, CompileFailure> {
    let config = super::parse_draft_config(config_source)?;
    let fuel = match config.fuel.owner() {
        crate::sherlock::helper_machine::shared::RunIntC::Nat(value) => value,
        _ => {
            return Err(CompileFailure::input(
                "config",
                "fuel must be a natural u128",
            ));
        }
    };
    let stack = match config.stack.owner() {
        crate::sherlock::helper_machine::shared::RunIntC::Nat(value) => usize::try_from(value)
            .map_err(|_| CompileFailure::input("config", "stack exceeds usize"))?,
        _ => {
            return Err(CompileFailure::input(
                "config",
                "stack must be a natural u128",
            ));
        }
    };
    let (pair, _, _) = super::prepare_draft_pair(source, config_source, target_source)?;
    let mut machine = Machine::new(pair.preparation.draft.program.clone(), fuel, stack);
    // Each event charges one fuel unit except the single free publication,
    // so the run has at most fuel + 1 events; the cap is defensive only.
    let max_events = fuel.saturating_add(1);
    let mut steps: Vec<Value> = Vec::new();
    let mut rules_exercised: Vec<&'static str> = Vec::new();
    let mut last_post_d = String::new();
    while let Some(record) = machine.step() {
        if steps.len() as u128 > max_events {
            return Err(CompileFailure::input(
                "evaluation",
                "witness run exceeded fuel + 1 events",
            ));
        }
        let name = rule_name(record.rule_start_line);
        if !rules_exercised.contains(&name) {
            rules_exercised.push(name);
        }
        let pre = snapshot_value(&record.before)?;
        let post = snapshot_value(&record.after)?;
        last_post_d = post["digest"].as_str().unwrap_or_default().to_owned();
        let terminal = record.after.outcome != Outcome::Pending;
        steps.push(json!({
            "index": steps.len().to_string(),
            "rule": name,
            "rule_start_line": record.rule_start_line.to_string(),
            "charge": record.charge.to_string(),
            "pre_d": pre["digest"].clone(),
            "post_d": post["digest"].clone(),
            "h": record.after.frames.len().to_string(),
            "pre": pre,
            "post": post,
        }));
        if terminal {
            break;
        }
    }
    let last = steps
        .last()
        .ok_or_else(|| CompileFailure::input("evaluation", "witness run made no progress"))?;
    let final_outcome = last["post"]["outcome"].clone();
    let result = match &final_outcome["kind"] {
        Value::String(kind) if kind == "Done" => final_outcome["value"].clone(),
        _ => Value::Null,
    };
    rules_exercised.sort_unstable();
    Ok(json!({
        "format": WITNESS_STREAM_FORMAT,
        "qualification": "HostOnly sherlock-k-a/1 witness stream; unmetered host machine; digests are SHA-256 of canonical snapshot JSON, not Poseidon lane digests; no circuit, proof or B-image claim",
        "reference_schedule_id": PROFILE,
        "fuel": fuel.to_string(),
        "stack_limit": stack.to_string(),
        "events": steps.len().to_string(),
        "steps": steps,
        "rules_exercised": rules_exercised,
        "final": {
            "outcome": final_outcome,
            "result": result,
            "post_d": last_post_d,
        },
    }))
}

#[cfg(test)]
#[path = "draft_witness_stream_tests.rs"]
mod tests;
