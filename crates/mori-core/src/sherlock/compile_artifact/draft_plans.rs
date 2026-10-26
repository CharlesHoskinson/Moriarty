//! UIREQ-031 export plans and UIREQ-033 bounded static residualization over the
//! real `sherlock-k-a/1` reference machine (the same run `--reference-slices`
//! groups). Pure description and host observation: nothing here proves, emits
//! a circuit, or claims a Midnight/ZKIR exporter.
//!
//! Residualization folds only the immutable rule inventory (K rule location,
//! charge, source-occurrence id) into blocks. Control, data, environments,
//! frames, fuel/work, capacity checks and outcomes stay dynamic because the
//! residual realization is executed by the same unmetered host Machine and
//! each step is dispatched through the residual block table; rules without a
//! block (family mask, per-program mask or the size limit) use the generic
//! dispatch path. The executed stream is the existing reference-slices stream
//! at budget 1, so equality with `--reference-slices 1` is checkable.
use std::path::Path;

use serde_json::{Value, json};

use super::CompileFailure;
use crate::sherlock::reference_display::{KDisplay, environment_to_k};
use crate::sherlock::{Control, Expr, Frame, Machine, Outcome, PROFILE, Snapshot, StepRecord};

pub const RESIDUAL_FORMAT: &str = "moriarty-sherlock-residual-step/1";
pub const EXPORT_FORMAT: &str = "sherlock-export-plans/1";
const LAYOUT: &str = "sherlock-layout/1";
const FIELD: &str = "bls12-381-scalar";

/// (K rule line, name, family, kind, charge). Families: Schedule, Scalar,
/// CallBind, FrameReturn, DataMatch. Kinds: decode (static dispatch of an
/// expression/frame constructor), check (dynamic capacity/type/fuel check),
/// publish (administrative outcome).
const RULES: &[(usize, &str, &str, &str, u8)] = &[
    (80, "PublishDone", "Schedule", "publish", 0),
    (82, "PublishError", "Schedule", "publish", 0),
    (84, "PublishExhausted", "Schedule", "publish", 0),
    (102, "EvalLiteral", "Scalar", "decode", 1),
    (108, "EvalVar", "Scalar", "decode", 1),
    (111, "EvalVarUnbound", "Scalar", "check", 1),
    (114, "EvalLet", "Scalar", "decode", 1),
    (117, "EvalBinary", "Scalar", "decode", 1),
    (120, "EvalUnary", "Scalar", "decode", 1),
    (123, "EvalIf", "Schedule", "decode", 1),
    (126, "EvalMatch", "DataMatch", "decode", 1),
    (129, "EvalCallNoArgs", "CallBind", "decode", 1),
    (131, "EvalCallArgs", "CallBind", "decode", 1),
    (135, "PushFrame", "FrameReturn", "check", 1),
    (140, "PushFrameCap", "FrameReturn", "check", 1),
    (145, "ReturnLet", "Scalar", "decode", 1),
    (149, "ReturnLeft", "Scalar", "decode", 1),
    (153, "ReturnRight", "Scalar", "decode", 1),
    (157, "ReturnUnary", "Scalar", "decode", 1),
    (161, "ReturnIfTrue", "Schedule", "decode", 1),
    (165, "ReturnIfFalse", "Schedule", "decode", 1),
    (169, "ReturnIfNotBool", "Schedule", "check", 1),
    (173, "ReturnMatch", "DataMatch", "decode", 1),
    (177, "ReturnArgNext", "CallBind", "decode", 1),
    (181, "ReturnArgLast", "CallBind", "decode", 1),
    (185, "ReturnTyped", "FrameReturn", "check", 1),
    (190, "ReturnTypeMismatch", "FrameReturn", "check", 1),
    (195, "ReturnScope", "FrameReturn", "decode", 1),
    (200, "Invoke", "CallBind", "decode", 1),
    (202, "EnterMissing", "CallBind", "check", 1),
    (204, "EnterBind", "CallBind", "decode", 1),
    (206, "BoundFailed", "CallBind", "check", 1),
    (208, "BoundPush", "CallBind", "decode", 1),
    (212, "CasesEmpty", "DataMatch", "check", 1),
    (214, "CasesTry", "DataMatch", "decode", 1),
    (217, "CaseMiss", "DataMatch", "decode", 1),
    (220, "CaseHit", "DataMatch", "decode", 1),
    (223, "PrimitiveOk", "Scalar", "decode", 1),
    (225, "PrimitiveErr", "Scalar", "check", 1),
];
const FAMILIES: &[&str] = &["Schedule", "Scalar", "CallBind", "FrameReturn", "DataMatch"];
const DYNAMIC: &[&str] = &[
    "control",
    "data",
    "environment",
    "frames",
    "depth",
    "fuel",
    "work",
    "helper-cursor",
    "capacity-checks",
    "outcome",
];

/// Fixed block partition of the whole K-A rule inventory. Every block is
/// homogeneous in one rule family, so a family mask folds only its own block.
/// The partition never depends on the program, fuel, stack or recursion
/// depth; only `program_image_id` and the folded bindings differ between
/// realizations, which keeps the universal/family IRs program-independent.
struct BlockGroup {
    id: &'static str,
    kind: &'static str,
    family: &'static str,
    selector: Option<&'static str>,
    rules: &'static [usize],
}
const GROUPS: &[BlockGroup] = &[
    BlockGroup {
        id: "publish",
        kind: "return",
        family: "Schedule",
        selector: None,
        rules: &[80, 82, 84],
    },
    BlockGroup {
        id: "scalar",
        kind: "static-dispatch",
        family: "Scalar",
        selector: None,
        rules: &[102, 108, 114, 117, 120, 145, 149, 153, 157, 223],
    },
    BlockGroup {
        id: "scalar-check",
        kind: "step",
        family: "Scalar",
        selector: None,
        rules: &[111, 225],
    },
    BlockGroup {
        id: "branch",
        kind: "dynamic-branch",
        family: "Schedule",
        selector: Some("control"),
        rules: &[123, 161, 165, 169],
    },
    BlockGroup {
        id: "match",
        kind: "dynamic-branch",
        family: "DataMatch",
        selector: Some("data"),
        rules: &[126, 173, 212, 214, 217, 220],
    },
    BlockGroup {
        id: "call",
        kind: "runtime-call",
        family: "CallBind",
        selector: None,
        rules: &[129, 131, 177, 181, 200, 202, 204, 206, 208],
    },
    BlockGroup {
        id: "frame",
        kind: "return",
        family: "FrameReturn",
        selector: None,
        rules: &[135, 140, 185, 190, 195],
    },
];
const GENERIC_ID: &str = "generic-dispatch";

fn rule(line: usize) -> Option<&'static (usize, &'static str, &'static str, &'static str, u8)> {
    RULES.iter().find(|r| r.0 == line)
}

fn run_records(
    source: &str,
    config_source: &str,
    target_source: &str,
) -> Result<Vec<StepRecord>, CompileFailure> {
    let config = super::parse_draft_config(config_source)?;
    let fuel = match config.fuel.owner() {
        crate::sherlock::helper_machine::shared::RunIntC::Nat(v) => v,
        _ => {
            return Err(CompileFailure::input(
                "config",
                "fuel must be a natural u128",
            ));
        }
    };
    let stack = match config.stack.owner() {
        crate::sherlock::helper_machine::shared::RunIntC::Nat(v) => usize::try_from(v)
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
    let mut out = Vec::new();
    let cap = fuel.saturating_add(1);
    while let Some(record) = machine.step() {
        out.push(record);
        if out.len() as u128 > cap {
            return Err(CompileFailure::input(
                "evaluation",
                "reference run exceeded fuel + 1 events",
            ));
        }
    }
    Ok(out)
}

fn digest(value: &Value) -> String {
    crate::intent::hash_bytes(value.to_string().as_bytes())
}

fn folded_kinds(mode: &str, family: Option<&str>, group_family: &str) -> Vec<&'static str> {
    match mode {
        "family" if family == Some(group_family) => vec!["layout", "family"],
        "per-program" => vec!["code", "layout", "source-occurrence-ids"],
        _ => vec!["layout"],
    }
}

fn block_value(
    mode: &str,
    family: Option<&str>,
    group: &BlockGroup,
    rules: &[usize],
    successors: &[&str],
) -> Value {
    let mut block = json!({
        "block_id": group.id,
        "kind": group.kind,
        "rules": rules.iter().map(|line| line.to_string()).collect::<Vec<_>>(),
        "folded": folded_kinds(mode, family, group.family),
        "successors": successors,
    });
    if let Some(selector) = group.selector {
        block["selector"] = json!(selector);
    }
    if group.kind == "runtime-call" {
        block["unrolled"] = json!(false);
        block["frame_check"] = json!("dynamic");
    }
    block
}

/// Select the residual block that must produce the next step, from the
/// complete pre-step state only. This is the dispatch half of the residual
/// executor: the selected block then performs the step, and the executor
/// rejects the run if the produced rule is not one of the block's rules.
fn dispatch_group(snapshot: &Snapshot) -> &'static str {
    let Some(control) = &snapshot.control else {
        return "publish";
    };
    match control {
        Control::Return(_) if snapshot.frames.is_empty() => "publish",
        Control::Error(_) => "publish",
        _ if snapshot.remaining_fuel == 0 => "publish",
        Control::Eval(expr) => match expr.as_ref() {
            Expr::If(..) => "branch",
            Expr::Match(..) => "match",
            Expr::Call(..) => "call",
            Expr::Var(name) if !snapshot.environment.contains_key(name) => "scalar-check",
            _ => "scalar",
        },
        Control::Push(..) => "frame",
        Control::Return(_) => match snapshot.frames.first() {
            Some(Frame::If(..)) => "branch",
            Some(Frame::Match(..)) => "match",
            Some(Frame::Args(..)) => "call",
            Some(Frame::Return(..) | Frame::Scope(_)) => "frame",
            _ => "scalar",
        },
        Control::Invoke(..) | Control::Enter(..) | Control::Bound(..) => "call",
        Control::Cases(..) | Control::CaseResult(..) => "match",
        Control::PrimitiveResult(Err(_)) => "scalar-check",
        Control::PrimitiveResult(Ok(_)) => "scalar",
    }
}

fn outcome_value(outcome: &Outcome) -> Value {
    match outcome {
        Outcome::Pending => json!({"kind": "Pending"}),
        Outcome::Done(value) => json!({"kind": "Done", "value": value.to_k()}),
        Outcome::Rejected(code) => json!({"kind": "Rejected", "code": code.to_string()}),
        Outcome::Exhausted => json!({"kind": "Exhausted"}),
    }
}

fn snapshot_value(snapshot: &Snapshot) -> Value {
    json!({
        "control": snapshot.control.as_ref().map(KDisplay::to_k),
        "environment": environment_to_k(&snapshot.environment),
        "frames": snapshot.frames.iter().map(KDisplay::to_k).collect::<Vec<_>>(),
        "logical_depth": snapshot.frames.len().to_string(),
        "stack_limit": snapshot.stack_limit.to_string(),
        "remaining_fuel": snapshot.remaining_fuel.to_string(),
        "initial_fuel": snapshot.initial_fuel.to_string(),
        "outcome": outcome_value(&snapshot.outcome),
    })
}

/// Execute the residual IR: repeatedly select a block from the current
/// state, let that block perform exactly one reference step, and group the
/// produced records into `reference-slices/1` slices of budget 1. Each
/// record carries the `block_id` of the block that produced it.
fn execute_residual(
    source: &str,
    config_source: &str,
    target_source: &str,
    assignment: &std::collections::BTreeMap<usize, &'static str>,
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
    let mut stepped: Vec<(StepRecord, &'static str)> = Vec::new();
    let cap = fuel.saturating_add(1);
    loop {
        let snapshot = machine.snapshot();
        if snapshot.outcome != Outcome::Pending {
            break;
        }
        let group = dispatch_group(&snapshot);
        let block_id = if assignment.values().any(|block| *block == group) {
            group
        } else {
            GENERIC_ID
        };
        let Some(record) = machine.step() else {
            break;
        };
        let realized = assignment
            .get(&record.rule_start_line)
            .copied()
            .unwrap_or(GENERIC_ID);
        if realized != block_id {
            return Err(CompileFailure::input(
                "evaluation",
                format!(
                    "residual block {block_id} produced rule {}, which it does not realize",
                    record.rule_start_line
                ),
            ));
        }
        stepped.push((record, block_id));
        if stepped.len() as u128 > cap {
            return Err(CompileFailure::input(
                "evaluation",
                "residual run exceeded fuel + 1 events",
            ));
        }
    }
    let last = stepped
        .last()
        .ok_or_else(|| CompileFailure::input("evaluation", "residual run made no progress"))?;
    let mut cumulative: u128 = 0;
    let mut slices: Vec<Value> = Vec::with_capacity(stepped.len());
    for (index, (record, block_id)) in stepped.iter().enumerate() {
        cumulative += u128::from(record.charge);
        let terminal = index + 1 == stepped.len();
        slices.push(json!({
            "index": index.to_string(),
            "profile_id": PROFILE,
            "start_event": index.to_string(),
            "records": [{
                "internal_position": "1",
                "position": (index + 1).to_string(),
                "rule_start_line": record.rule_start_line.to_string(),
                "source_control": record.before.control.as_ref().map(KDisplay::to_k),
                "charge": record.charge.to_string(),
                "prefix_work": record.charge.to_string(),
                "cumulative_work": cumulative.to_string(),
                "frame_check": {
                    "depth_before": record.before.frames.len().to_string(),
                    "depth_after": record.after.frames.len().to_string(),
                    "stack_limit": record.after.stack_limit.to_string(),
                    "within_limit": record.after.frames.len() <= record.after.stack_limit,
                },
                "first_outcome": record.after.outcome != Outcome::Pending,
                "outcome_after": outcome_value(&record.after.outcome),
                "block_id": block_id,
            }],
            "prefix_work": ["0", record.charge.to_string()],
            "boundaries": [{
                "position": "1",
                "kind": if terminal { "terminal" } else { "technical-cut" },
            }],
            "stop": if terminal {
                json!({"kind": "outcome", "outcome": outcome_value(&record.after.outcome)})
            } else {
                json!({"kind": "technical-cut", "financial_commit": "none"})
            },
            "start": snapshot_value(&record.before),
            "end": snapshot_value(&record.after),
            "padding": {"reference_records": "0", "charge": "0"},
        }));
    }
    Ok(json!({
        "format": super::draft_reference_slices::FORMAT,
        "qualification": "HostOnly sherlock-k-a/1 residual block execution; unmetered host machine; no circuit, proof or B-image claim",
        "reference_schedule_id": PROFILE,
        "group_budget": "1",
        "fuel": fuel.to_string(),
        "stack_limit": stack.to_string(),
        "events": stepped.len().to_string(),
        "reference_work": cumulative.to_string(),
        "technical_cuts": (stepped.len() - 1).to_string(),
        "financial_commits": "0",
        "outcome": outcome_value(&last.0.after.outcome),
        "slices": slices,
        "cumulative": Value::Null,
    }))
}

/// ResidualStepIR plus the executed residual realization, in the
/// `moriarty-sherlock-residual-step/1` shape of the U033 contract.
pub fn residualize(
    source: &str,
    config_source: &str,
    target_source: &str,
    mode: &str,
    family: Option<&str>,
    limit: usize,
) -> Result<Value, CompileFailure> {
    if limit == 0 {
        return Err(CompileFailure::input(
            "config",
            "--residual-size-limit must be greater than zero",
        ));
    }
    let mask = match (mode, family) {
        ("universal", None) => "empty",
        ("family", Some(f)) if FAMILIES.contains(&f) => "family",
        ("per-program", None) => "program-image",
        ("family", _) => {
            return Err(CompileFailure::input(
                "config",
                "--residualize family needs --residual-family Schedule|Scalar|CallBind|FrameReturn|DataMatch",
            ));
        }
        ("universal" | "per-program", Some(_)) => {
            return Err(CompileFailure::input(
                "config",
                "--residual-family is only valid with --residualize family",
            ));
        }
        _ => {
            return Err(CompileFailure::input(
                "config",
                "--residualize must be universal, family or per-program",
            ));
        }
    };

    // Bounded worklist: keep at most `limit` blocks in total, counting the
    // shared generic-dispatch block. Surplus rule groups fall back to it.
    let mut fallbacks: Vec<Value> = Vec::new();
    let mut assignment: std::collections::BTreeMap<usize, &'static str> =
        std::collections::BTreeMap::new();
    let kept: &[BlockGroup] = if limit >= GROUPS.len() {
        GROUPS
    } else {
        &GROUPS[..limit - 1]
    };
    let mut generic_rules: Vec<usize> = Vec::new();
    for group in GROUPS.iter().skip(kept.len()) {
        generic_rules.extend_from_slice(group.rules);
    }
    generic_rules.sort_unstable();
    for group in kept {
        for &line in group.rules {
            assignment.insert(line, group.id);
        }
    }
    for &line in &generic_rules {
        assignment.insert(line, GENERIC_ID);
        fallbacks.push(json!({
            "key": format!("rule:{line}"),
            "rule_start_line": line.to_string(),
            "reason": "residual-size-limit",
        }));
    }
    // Dense CFG: conservative overapproximation of the partition's control
    // flow. Every live block names all present blocks as successors
    // (terminal publish: none).
    let mut present: Vec<&'static str> = kept.iter().map(|group| group.id).collect();
    if !generic_rules.is_empty() {
        present.push(GENERIC_ID);
    }
    let mut blocks: Vec<Value> = kept
        .iter()
        .map(|group| {
            let successors: &[&str] = if group.id == "publish" { &[] } else { &present };
            block_value(mode, family, group, group.rules, successors)
        })
        .collect();
    if !generic_rules.is_empty() {
        blocks.push(json!({
            "block_id": GENERIC_ID,
            "kind": "generic-dispatch",
            "rules": generic_rules.iter().map(|line| line.to_string()).collect::<Vec<_>>(),
            "folded": ["layout"],
            "successors": present,
        }));
    }

    let rule_boundary_map: Vec<Value> = RULES
        .iter()
        .map(|&(line, _, _, _, charge)| {
            json!({
                "rule_start_line": line.to_string(),
                "block_id": assignment.get(&line).copied().unwrap_or(GENERIC_ID),
                "charge": charge.to_string(),
            })
        })
        .collect();
    let source_occurrence_map: Vec<Value> = RULES
        .iter()
        .map(|&(line, name, _, _, _)| {
            json!({
                "rule_start_line": line.to_string(),
                "rule": name,
                "source_occurrence": format!("occ:{line}"),
                "block_id": assignment.get(&line).copied().unwrap_or(GENERIC_ID),
            })
        })
        .collect();
    let fallback_used = !fallbacks.is_empty();
    let image_id = (mode == "per-program").then(|| crate::intent::hash_bytes(source.as_bytes()));
    let static_bindings = match mode {
        "family" => json!({"kinds": ["layout", "family"], "family": family}),
        "per-program" => json!({
            "kinds": ["code", "declarations", "layout", "source-occurrence-ids", "operand-slices"],
            "program_image_id": image_id,
        }),
        _ => json!({"kinds": ["layout"]}),
    };
    let mut ir = json!({
        "reference_definition_id": "sherlock-k-a/1",
        "profile_id": PROFILE,
        "program_image_id": image_id,
        "layout_version": LAYOUT,
        "blocks": blocks,
        "dynamic_inputs": DYNAMIC,
        "static_bindings": static_bindings,
        "rule_boundary_map": rule_boundary_map,
        "source_occurrence_map": source_occurrence_map,
        "generic_fallbacks": fallbacks,
        "omissions": [],
    });
    let canonical = crate::intent::canonical(&ir)
        .map_err(|error| CompileFailure::input("serialization", error.to_string()))?;
    ir["ir_id"] = Value::String(crate::intent::hash_bytes(&canonical));

    // Execute this IR through its own block dispatch (see execute_residual);
    // every dynamic boundary stays in the machine state, so the produced
    // stream is the reference stream at slice budget 1.
    let execution = execute_residual(source, config_source, target_source, &assignment)?;
    Ok(json!({
        "format": RESIDUAL_FORMAT,
        "realization": mode,
        "config": {
            "static_mask": mask,
            "family": family,
            "residual_size_limit": limit.to_string(),
        },
        "ir": ir,
        "worklist": {"fallback_used": fallback_used},
        "execution": execution,
    }))
}

fn value_type(v: &crate::sherlock::Value) -> &'static str {
    match v {
        crate::sherlock::Value::UInt(_) => "UInt128",
        crate::sherlock::Value::Bool(_) => "Bool",
        crate::sherlock::Value::List(_) => "List",
        crate::sherlock::Value::Tree(_) => "Tree",
    }
}

/// Operation ids for one reference record (coverage names, not realization claims).
fn operations(r: &StepRecord) -> Vec<(&'static str, &'static str)> {
    let mut ops: Vec<(&'static str, &'static str)> = Vec::new();
    match r.rule_start_line {
        108 | 111 => ops.push(("ArenaRead", "env")),
        153 => {
            let op = match r.before.frames.first() {
                Some(Frame::Right(op, _)) => Some(*op),
                _ => None,
            };
            ops.push(match op {
                Some(crate::sherlock::BinaryOp::Add) => ("CheckedU128Add", "u128"),
                Some(crate::sherlock::BinaryOp::Sub) => ("CheckedU128Sub", "u128"),
                Some(crate::sherlock::BinaryOp::Mul) => ("CheckedU128Mul", "u128"),
                Some(crate::sherlock::BinaryOp::Eq | crate::sherlock::BinaryOp::Lt) => {
                    ("Select", "bool")
                }
                _ => ("DataConstruct", "arena"),
            });
        }
        135 | 140 | 145 | 149 | 185 | 190 | 195 | 208 => ops.push(("FrameRestore", "frame")),
        161 | 165 | 169 => ops.push(("Select", "bool")),
        173 | 212 | 214 | 217 | 220 => ops.push(("MatchSelect", "arena")),
        _ => ops.push(("DecodeStep", "control")),
    }
    if r.charge > 0 {
        ops.push(("CheckedU128Sub", "u128")); // fuel decrement
    }
    ops
}

fn io_types(r: &StepRecord) -> (Vec<Value>, Vec<Value>) {
    let ty = |c: &Option<crate::sherlock::Control>| -> Value {
        match c {
            Some(crate::sherlock::Control::Return(v)) => {
                json!({"name": "control", "type": value_type(v)})
            }
            Some(c) => json!({"name": "control", "type": "Control", "k": c.to_k()}),
            None => json!({"name": "control", "type": "Published"}),
        }
    };
    let ins = vec![
        ty(&r.before.control),
        json!({"name": "remaining_fuel", "type": "UInt128", "value": r.before.remaining_fuel.to_string()}),
        json!({"name": "frame_depth", "type": "UInt128", "value": r.before.frames.len().to_string()}),
    ];
    let outs = vec![
        ty(&r.after.control),
        json!({"name": "remaining_fuel", "type": "UInt128", "value": r.after.remaining_fuel.to_string()}),
        json!({"name": "frame_depth", "type": "UInt128", "value": r.after.frames.len().to_string()}),
        json!({"name": "outcome", "type": match &r.after.outcome {
            Outcome::Pending => "Pending", _ => "Published" }}),
    ];
    (ins, outs)
}

const REQUESTS: &[&str] = &[
    "constraint-json",
    "structured-witness",
    "native-witness-closure",
    "lookup",
    "interaction",
    "custom-gate",
];

/// Separate ConstraintPlan / WitnessPlan for reference record `position` (1-based).
pub fn export_plans(
    source: &str,
    config_source: &str,
    target_source: &str,
    position: usize,
    requests: &[String],
    export_dir: Option<&Path>,
) -> Result<Value, CompileFailure> {
    for q in requests {
        if !REQUESTS.contains(&q.as_str()) {
            return Err(CompileFailure::input(
                "config",
                format!("unknown --export-request {q}"),
            ));
        }
    }
    let records = run_records(source, config_source, target_source)?;
    let r = position
        .checked_sub(1)
        .and_then(|i| records.get(i))
        .ok_or_else(|| {
            CompileFailure::input(
                "config",
                "--export-plan position is outside the reference run",
            )
        })?;
    let (ins, outs) = io_types(r);
    let ops = operations(r);
    let known = rule(r.rule_start_line).map_or("Unknown", |k| k.1);
    let mut operations_v = Vec::new();
    for (i, (id, domain)) in ops.iter().enumerate() {
        let comp = matches!(
            *id,
            "CheckedU128Add"
                | "CheckedU128Sub"
                | "CheckedU128Mul"
                | "ArenaRead"
                | "FrameRestore"
                | "Select"
        )
        .then(|| format!("component:{id}"));
        operations_v.push(json!({"operation_id": id, "domain": domain, "ordering": i,
            "source_locators": [format!("k-a:{PROFILE}:rule@{}:{known}", r.rule_start_line), format!("record:{position}")],
            "witness_dependencies": if i == 0 { vec![] } else { vec![format!("op{}", i - 1)] },
            "constraint_component_locator": comp,
            "witness_program_kind": "structured",
            "supported_status": if comp.is_some() { "Specified" } else { "Unsupported" }}));
    }
    let mut manifest = json!({"reference_definition_id": "stepReference", "profile_id": PROFILE,
        "layout_version": LAYOUT, "operations": operations_v,
        "typed_operations": ops.iter().map(|o| json!({"operation_id": o.0, "domain": o.1})).collect::<Vec<_>>(),
        "ordered_inputs": ins, "ordered_outputs": outs,
        "source_membership": {"rule_start_line": r.rule_start_line, "rule": known, "record": position},
        "component_membership": operations_v.iter().filter_map(|o| o["constraint_component_locator"].as_str()).collect::<Vec<_>>(),
        "omissions": operations_v.iter().filter(|o| o["supported_status"] == "Unsupported")
            .map(|o| o["operation_id"].clone()).collect::<Vec<_>>()});
    manifest["manifest_id"] = Value::String(digest(&manifest));
    let mid = manifest["manifest_id"].clone();
    let mut cplan = json!({"operation_manifest_id": mid, "layout_version": LAYOUT, "field_id": FIELD,
        "ordered_public_inputs": ["remaining_fuel"], "ordered_private_inputs": ["control", "frame_depth"],
        "ordered_outputs": manifest["ordered_outputs"],
        "components": operations_v.iter().filter(|o| o["constraint_component_locator"].is_string()).map(|o| json!({
            "component_id": o["constraint_component_locator"], "operation_id": o["operation_id"],
            "source_locator": o["source_locators"][0], "parameters": {}})).collect::<Vec<_>>(),
        "required_capabilities": ["constraint-json"], "omissions": manifest["omissions"]});
    cplan["plan_id"] = Value::String(digest(&cplan));
    let mut wplan = json!({"operation_manifest_id": mid, "layout_version": LAYOUT,
        "input_bindings": manifest["ordered_inputs"],
        "nodes": ops.iter().enumerate().map(|(i, o)| json!({"node_id": format!("n{i}"), "operation_id": o.0,
            "typed_inputs": manifest["ordered_inputs"], "typed_outputs": manifest["ordered_outputs"],
            "dependencies": if i == 0 { vec![] } else { vec![format!("n{}", i - 1)] },
            "last_use": format!("n{i}")})).collect::<Vec<_>>(),
        "stable_topological_order": "declaration", "output_bindings": ["control", "remaining_fuel", "frame_depth", "outcome"],
        "slot_counts": {"nodes": ops.len()}, "program_kind": "structured-witness"});
    wplan["plan_id"] = Value::String(digest(&wplan));
    let caps = json!({"backend_id": "sherlock-tooling/1", "source_membership": "reference-step",
        "field_id": FIELD, "formats": ["constraint-json", "structured-witness"],
        "constraint_operations": ["CheckedU128Add", "CheckedU128Sub", "CheckedU128Mul", "ArenaRead", "FrameRestore", "Select"],
        "witness_operations": ["CheckedU128Add", "CheckedU128Sub", "CheckedU128Mul", "ArenaRead", "FrameRestore", "Select", "MatchSelect", "DecodeStep", "DataConstruct"],
        "lookup": false, "interaction": false, "custom_gates": false,
        "structured_witness_ir": true, "native_closures": false, "midnight_exporter": false});
    let mut exports = Vec::new();
    for q in requests {
        let located = |feature: &str| {
            json!({"request": q, "status": "Unsupported", "tooling": "Unsupported",
            "feature": feature, "locator": format!("record:{position}/rule@{}", r.rule_start_line),
            "reason": format!("backend sherlock-tooling/1 has no {feature} support"), "artifact": null})
        };
        match q.as_str() {
            "constraint-json" | "structured-witness" => {
                let (plan, file) = if q == "constraint-json" {
                    (&cplan, "constraint_plan.json")
                } else {
                    (&wplan, "witness_plan.json")
                };
                let dir = export_dir.ok_or_else(|| {
                    CompileFailure::input("config", "--export-request needs --export-dir")
                })?;
                let bytes = serde_json::to_vec_pretty(plan)
                    .map_err(|e| CompileFailure::input("serialization", e.to_string()))?;
                std::fs::create_dir_all(dir)
                    .map_err(|e| CompileFailure::input("config", e.to_string()))?;
                std::fs::write(dir.join(file), &bytes)
                    .map_err(|e| CompileFailure::input("config", e.to_string()))?;
                exports.push(json!({"request": q, "status": "Emitted", "artifact": {"file": file,
                    "sha256": crate::intent::hash_bytes(&bytes), "bytes": bytes.len()},
                    "scope": if q == "constraint-json" { "ConstraintPlan descriptor by component locator; not a witness program and not a constraint system" } else { "WitnessPlan descriptor; structured witness program kind" }}));
            }
            "native-witness-closure" => exports.push(located("native-witness-closure")),
            "lookup" => exports.push(located("lookup")),
            "interaction" => exports.push(located("interaction")),
            _ => exports.push(located("custom-gate")),
        }
    }
    let any_unsupported = exports.iter().any(|e| e["status"] == "Unsupported");
    Ok(
        json!({"format": EXPORT_FORMAT, "record_position": position.to_string(),
        "operation_manifest": manifest, "constraint_plan": cplan, "witness_plan": wplan,
        "backend_capabilities": caps,
        "disposition": {
            "constraint_export": {"status": "Supported", "format": "constraint-json"},
            "witness_program_export": {"status": "Supported", "format": "structured-witness"},
            "lookup": {"status": "Unsupported"}, "interact": {"status": "Unsupported"},
            "custom_gates": {"status": "Unsupported"}, "native_closures": {"status": "Unsupported"},
            "field_layout": {"status": "Supported", "field_id": FIELD, "layout_version": LAYOUT}},
        "exports": exports,
        "tooling_status": if any_unsupported { "Unsupported" } else { "Supported" },
        "claims": {"constraint_json_supplies_witness_computation": false,
            "non_midnight_format_is_midnight_exporter": false,
            "midnight_zkir_exporter": "not-implemented", "execution": "none", "acceptance": "none"}}),
    )
}
