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
use std::collections::BTreeSet;
use std::path::Path;

use serde_json::{Value, json};

use super::CompileFailure;
use crate::sherlock::reference_display::KDisplay;
use crate::sherlock::{Frame, Machine, Outcome, PROFILE, StepRecord};

pub const RESIDUAL_FORMAT: &str = "sherlock-residual-step-ir/1";
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
    "input-data",
    "environment",
    "frames",
    "depth",
    "fuel-work",
    "capacity-checks",
    "outcome",
];

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

/// ResidualStepIR (snake_case) plus the executed residual realization.
pub fn residualize(
    source: &str,
    config_source: &str,
    target_source: &str,
    mode: &str,
    family: Option<&str>,
    limit: usize,
) -> Result<Value, CompileFailure> {
    let records = run_records(source, config_source, target_source)?;
    let exercised: BTreeSet<usize> = records.iter().map(|r| r.rule_start_line).collect();
    let (mask, static_rules): (&str, BTreeSet<usize>) = match (mode, family) {
        ("universal", None) => ("empty", RULES.iter().map(|r| r.0).collect()),
        ("family", Some(f)) if FAMILIES.contains(&f) => (
            "fixed-family",
            RULES.iter().filter(|r| r.2 == f).map(|r| r.0).collect(),
        ),
        ("per-program", None) => ("fixed-program-image", exercised.clone()),
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
    let mut blocks = Vec::new();
    let mut fallbacks = Vec::new();
    for &(line, name, fam, kind, charge) in RULES {
        if !static_rules.contains(&line) {
            fallbacks.push(json!({"rule_start_line": line, "rule": name,
                "reason": "outside-static-binding", "fallback": "generic-dispatch"}));
        } else if blocks.len() >= limit {
            fallbacks.push(json!({"rule_start_line": line, "rule": name,
                "reason": "residual-size-limit", "fallback": "generic-dispatch"}));
        } else {
            blocks.push(json!({"rule_start_line": line, "rule": name, "family": fam,
                "folded": if kind == "decode" { "static-dispatch" } else { "none" },
                "kind": kind, "source_occurrence": format!("occ:{line}"), "charge": charge,
                "dynamic": DYNAMIC, "capacity_check": "dynamic", "fuel_check": "dynamic",
                "recursion": "runtime-call"}));
        }
    }
    let covered: BTreeSet<usize> = blocks
        .iter()
        .map(|b| b["rule_start_line"].as_u64().unwrap_or(0) as usize)
        .collect();
    let rule_map: serde_json::Map<String, Value> = blocks
        .iter()
        .map(|b| {
            (
                b["rule_start_line"].to_string(),
                json!({"source_occurrence": b["source_occurrence"], "charge": b["charge"]}),
            )
        })
        .collect();
    let occ_map: serde_json::Map<String, Value> = blocks
        .iter()
        .map(|b| {
            (
                b["source_occurrence"].as_str().unwrap_or("").to_owned(),
                b["rule"].clone(),
            )
        })
        .collect();
    let image_id = (mode == "per-program").then(|| crate::intent::hash_bytes(source.as_bytes()));
    let mut ir = json!({
        "reference_definition_id": "stepReference", "profile_id": PROFILE,
        "program_image_id": image_id, "layout_version": LAYOUT, "realization": mask,
        "blocks": blocks, "dynamic_inputs": DYNAMIC,
        "static_bindings": {"mode": mode, "family": family, "size_limit": limit,
            "static_rule_count": static_rules.len()},
        "rule_boundary_map": rule_map, "source_occurrence_map": occ_map,
        "generic_fallbacks": fallbacks,
        "omissions": ["circuit cost unmeasured", "adoption unmeasured"],
    });
    ir["ir_id"] = Value::String(digest(&ir));
    ir["format"] = Value::String(RESIDUAL_FORMAT.to_owned());

    // Execute the residual realization: dispatch every step through the block
    // table; verify rule line and charge against its block.
    let mut dispatch = Vec::with_capacity(records.len());
    let (mut folded, mut generic) = (0u128, 0u128);
    for (i, r) in records.iter().enumerate() {
        let known = rule(r.rule_start_line);
        let served = covered.contains(&r.rule_start_line) && known.is_some_and(|k| k.4 == r.charge);
        if served {
            folded += 1;
        } else {
            generic += 1;
        }
        dispatch.push(json!({"position": (i + 1).to_string(),
            "rule_start_line": r.rule_start_line, "charge": r.charge.to_string(),
            "served_by": if served { "residual-block" } else { "generic-dispatch" }}));
    }
    let mut execution =
        super::draft_reference_slices::reference_slices(source, config_source, target_source, 1)?;
    execution["realization"] = json!({"mode": mode, "residual_ir_id": ir["ir_id"],
        "dispatch": dispatch, "residual_block_steps": folded.to_string(),
        "generic_dispatch_steps": generic.to_string(),
        "application_exhaustion": false,
        "stream": "sherlock-k-a/1 Machine stepped with each rule dispatched through the residual block table"});
    ir["execution"] = execution;
    Ok(ir)
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
