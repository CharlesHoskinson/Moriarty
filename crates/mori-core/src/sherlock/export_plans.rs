//! Separate compilation/export plans for one bounded `sherlock-k-a/1` step.
//!
//! The selected step is the 1-based K-A reference record of the same draft
//! source and config that `--reference-slices 1` publishes. This module
//! prepares that single step into a shared `OperationManifest` with separate
//! `ConstraintPlan` and `WitnessPlan` artifacts and separately listed
//! `BackendCapabilities`. Artifact identities are the sha256 of the canonical
//! JSON of the artifact without its identity field.
//!
//! Only constraint JSON and the structured witness program are emitted by
//! this host exporter. Native witness closures, lookup, interaction and
//! custom gates are reported as located tooling `Unsupported`. No format is
//! claimed to be a Midnight/ZKIR exporter and constraint JSON is never
//! claimed to supply witness computation. Preparing a plan does not run,
//! alter or meter the application execution.
use std::path::Path;

use serde_json::{Value, json};

use super::CompileFailure;
use crate::sherlock::helper_machine::shared::RunIntC;
use crate::sherlock::reference_display::KDisplay;
use crate::sherlock::reference_slice::{SliceStop, advance_m};
use crate::sherlock::{
    BinaryOp, Control, Frame, Machine, Outcome, PROFILE, Snapshot, StepRecord, Type, UnaryOp,
    Value as KValue,
};

pub const FORMAT: &str = "moriarty-sherlock-export-plans/1";
const LAYOUT_VERSION: &str = "sherlock-layout/1";
const FIELD_ID: &str = "host-u128/1";
const WITNESS_PROGRAM_KIND: &str = "structured-witness-ir/1";
const BACKEND_ID: &str = "sherlock-host-export/1";
const CONSTRAINT_ARTIFACT: &str = "constraint-plan.json";
const WITNESS_ARTIFACT: &str = "witness-plan.json";

/// One export capability a caller may request for the prepared step.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportRequest {
    ConstraintJson,
    StructuredWitness,
    NativeWitnessClosure,
    Lookup,
    Interaction,
    CustomGate,
}

impl ExportRequest {
    /// The contract spelling of this request.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::ConstraintJson => "constraint-json",
            Self::StructuredWitness => "structured-witness",
            Self::NativeWitnessClosure => "native-witness-closure",
            Self::Lookup => "lookup",
            Self::Interaction => "interaction",
            Self::CustomGate => "custom-gate",
        }
    }

    /// Parse a contract spelling, returning `None` for any other text.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "constraint-json" => Self::ConstraintJson,
            "structured-witness" => Self::StructuredWitness,
            "native-witness-closure" => Self::NativeWitnessClosure,
            "lookup" => Self::Lookup,
            "interaction" => Self::Interaction,
            "custom-gate" => Self::CustomGate,
            _ => return None,
        })
    }

    /// Every request, in the contract's canonical order.
    const ALL: [Self; 6] = [
        Self::ConstraintJson,
        Self::StructuredWitness,
        Self::NativeWitnessClosure,
        Self::Lookup,
        Self::Interaction,
        Self::CustomGate,
    ];
}

fn content_id(value: &Value, id_key: &str) -> Result<String, CompileFailure> {
    let mut body = value.clone();
    if let Some(object) = body.as_object_mut() {
        object.remove(id_key);
    }
    let bytes = crate::intent::canonical(&body)
        .map_err(|error| CompileFailure::input("observation", error.to_string()))?;
    Ok(crate::intent::hash_bytes(&bytes))
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

fn value_type(value: &KValue) -> &'static str {
    match value {
        KValue::UInt(_) => "UInt128",
        KValue::Bool(_) => "Bool",
        KValue::List(_) => "UInt128List",
        KValue::Tree(_) => "UInt128Tree",
    }
}

fn type_name(ty: Type) -> &'static str {
    match ty {
        Type::UInt => "UInt128",
        Type::Bool => "Bool",
        Type::List => "UInt128List",
        Type::Tree => "UInt128Tree",
    }
}

fn binary_op_name(op: BinaryOp) -> &'static str {
    match op {
        BinaryOp::Add => "CheckedU128Add",
        BinaryOp::Sub => "CheckedU128Sub",
        BinaryOp::Mul => "CheckedU128Mul",
        BinaryOp::Eq => "UInt128Eq",
        BinaryOp::Lt => "UInt128Lt",
        BinaryOp::Cons => "ListCons",
        BinaryOp::Branch => "TreeBranch",
    }
}

fn typed(name: &str, ty: &str) -> Value {
    json!({"name": name, "type": ty})
}

/// A typed binding to an actual value at a named before/after state path.
fn bound(name: &str, ty: &str, source: &str, value: &KValue) -> Value {
    json!({"name": name, "type": ty, "source": source, "value": value.to_k()})
}

fn control_binding(snapshot: &Snapshot, source: &str) -> Value {
    json!({
        "name": "control",
        "type": "Control",
        "source": source,
        "value": snapshot.control.as_ref().map_or_else(|| "none".to_owned(), KDisplay::to_k),
    })
}

/// The primary operation this step performs, with its typed inputs/outputs.
///
/// Classification follows the executed K-A rule and the *top* frame only;
/// pending frames below the top are not the current operation.
fn classify(record: &StepRecord) -> (&'static str, Vec<Value>, Vec<Value>) {
    let before = &record.before;
    let after = &record.after;
    let top = before.frames.first();
    let returned = match &before.control {
        Some(Control::Return(value)) => Some(value),
        _ => None,
    };
    match (record.rule_start_line, returned, top) {
        // Rule 153: the captured left operand and the returned right operand
        // are the actual inputs of the checked binary operation.
        (153, Some(right), Some(Frame::Right(op, left))) => {
            let (result_ty, result) = match &after.control {
                Some(Control::PrimitiveResult(Ok(value))) => (
                    value_type(value),
                    bound(
                        "result",
                        value_type(value),
                        "after.control.primitive_result",
                        value,
                    ),
                ),
                _ => ("UInt128", typed("result", "UInt128")),
            };
            let _ = result_ty;
            (
                binary_op_name(*op),
                vec![
                    bound(
                        "operand_left",
                        value_type(left),
                        "before.frames[0].captured_left",
                        left,
                    ),
                    bound(
                        "operand_right",
                        value_type(right),
                        "before.control.return_value",
                        right,
                    ),
                ],
                vec![result],
            )
        }
        // Rule 185: returning through a ReturnF frame restores its environment.
        (185, Some(value), Some(Frame::Return(ty, _))) => (
            "FrameRestore",
            vec![bound(
                "return_value",
                value_type(value),
                "before.control.return_value",
                value,
            )],
            vec![bound(
                "restored_value",
                type_name(*ty),
                "after.control.return_value",
                value,
            )],
        ),
        // Rule 195: returning through a Scope frame restores only its scope.
        (195, Some(value), Some(Frame::Scope(_))) => (
            "ScopeRestore",
            vec![bound(
                "return_value",
                value_type(value),
                "before.control.return_value",
                value,
            )],
            vec![bound(
                "restored_value",
                value_type(value),
                "after.control.return_value",
                value,
            )],
        ),
        // Rule 157: a unary operation on the returned operand.
        (157, Some(value), Some(Frame::Unary(op))) => {
            let (name, ty) = match op {
                UnaryOp::Not => ("BoolNot", "Bool"),
                UnaryOp::Leaf => ("TreeLeaf", "UInt128Tree"),
            };
            (
                name,
                vec![bound(
                    "operand",
                    value_type(value),
                    "before.control.return_value",
                    value,
                )],
                vec![typed("result", ty)],
            )
        }
        // Rules 161/165: the returned condition selects a branch.
        (161 | 165, Some(value), Some(Frame::If(_, _, _))) => (
            "Select",
            vec![bound(
                "condition",
                value_type(value),
                "before.control.return_value",
                value,
            )],
            vec![control_binding(after, "after.control")],
        ),
        _ => (
            "ArenaRead",
            vec![control_binding(before, "before.control")],
            vec![control_binding(after, "after.control")],
        ),
    }
}

fn select_record(
    source: &str,
    config_source: &str,
    target_source: &str,
    position: u64,
) -> Result<StepRecord, CompileFailure> {
    let config = super::parse_draft_config(config_source)?;
    let fuel = run_int(config.fuel.owner(), "fuel")?;
    let stack = usize::try_from(run_int(config.stack.owner(), "stack")?)
        .map_err(|_| CompileFailure::input("config", "stack exceeds usize"))?;
    let (pair, _, _) = super::prepare_draft_pair(source, config_source, target_source)?;
    let program = pair.preparation.draft.program.clone();
    let max_events = fuel.saturating_add(1);
    let mut machine = Machine::new(program, fuel, stack);
    let mut seen: u64 = 0;
    loop {
        let slice = advance_m(&mut machine, 1);
        if slice.records.is_empty() {
            return Err(CompileFailure::input(
                "evaluation",
                format!(
                    "export plan position {position} exceeds the reference run ({seen} records)"
                ),
            ));
        }
        seen += 1;
        if u128::from(seen) > max_events {
            return Err(CompileFailure::input(
                "evaluation",
                "reference run exceeded fuel + 1 events",
            ));
        }
        let cut = slice.stop == SliceStop::TechnicalCut;
        let record = slice.records[0].clone();
        if seen == position {
            return Ok(record);
        }
        if !cut {
            return Err(CompileFailure::input(
                "evaluation",
                format!(
                    "export plan position {position} exceeds the reference run ({seen} records)"
                ),
            ));
        }
        machine = Machine::from_snapshot(slice.end.clone());
    }
}

fn snapshot_control(snapshot: &Snapshot) -> Value {
    snapshot
        .control
        .as_ref()
        .map_or(Value::Null, |c| json!(c.to_k()))
}

fn unsupported_capability(reason: &str) -> Value {
    json!({"supported": false, "reason": reason})
}

/// Prepare the step at 1-based `position` and emit the requested artifacts.
///
/// Position zero is refused. Only emitted artifacts are written, and only
/// into `export_dir`. The returned value is the complete `export_plans`
/// report; preparing it does not change the application run.
///
/// # Errors
///
/// Returns [`CompileFailure`] for position zero, an out-of-range position,
/// invalid draft/config/target inputs, or an artifact write failure.
pub fn export_plans(
    source: &str,
    config_source: &str,
    target_source: &str,
    position: u64,
    requests: &[ExportRequest],
    export_dir: &Path,
) -> Result<Value, CompileFailure> {
    if position == 0 {
        return Err(CompileFailure::input(
            "config",
            "--export-plan position must be at least 1 (positions are 1-based)",
        ));
    }
    let record = select_record(source, config_source, target_source, position)?;
    let locator = format!("sherlock-k-a/1:rule:{}", record.rule_start_line);
    let step_locator = format!("step:{position}");
    let (primary_op, primary_inputs, primary_outputs) = classify(&record);

    // OperationManifest: an arena read feeding the step's primary operation.
    // The state-read operation binds the complete before-state the step
    // consumes: its control and every environment entry, by state path.
    let mut env_inputs: Vec<Value> = vec![control_binding(&record.before, "before.control")];
    env_inputs.extend(record.before.environment.iter().map(|(name, value)| {
        bound(
            name,
            value_type(value),
            &format!("before.environment.{name}"),
            value,
        )
    }));
    let mut operations: Vec<Value> = Vec::new();
    let mut previous_id: Option<String> = None;
    for (ordering, (op, inputs, outputs)) in [
        ("ArenaRead", env_inputs, primary_inputs.clone()),
        (primary_op, primary_inputs.clone(), primary_outputs.clone()),
    ]
    .into_iter()
    .enumerate()
    {
        let mut operation = json!({
            "op": op,
            "ordering": ordering,
            "inputs": inputs,
            "outputs": outputs,
            "source_locators": [locator.clone(), step_locator.clone()],
            "component_membership": "source+component",
            "witness_dependencies": previous_id.as_ref().map(|id| vec![id.clone()]).unwrap_or_default(),
            "witness_program_kind": WITNESS_PROGRAM_KIND,
            "constraint_component_locator": format!("constraint-component:{ordering}"),
            "supported_status": {"kind": "Specified", "reason": Value::Null},
        });
        let operation_id = content_id(&operation, "operation_id")?;
        operation["operation_id"] = json!(operation_id);
        previous_id = Some(operation_id);
        operations.push(operation);
    }
    let operation_ids: Vec<String> = operations
        .iter()
        .map(|o| o["operation_id"].as_str().unwrap_or_default().to_owned())
        .collect();

    let mut manifest = json!({
        "reference_definition_id": PROFILE,
        "profile_id": PROFILE,
        "layout_version": LAYOUT_VERSION,
        "operations": operations,
        "omissions": [],
    });
    let manifest_id = content_id(&manifest, "manifest_id")?;
    manifest["manifest_id"] = json!(manifest_id);

    // ConstraintPlan: every operation is a located component; no omissions.
    let components: Vec<Value> = manifest["operations"]
        .as_array()
        .map(|ops| {
            ops.iter()
                .map(|o| {
                    json!({
                        "component_id": crate::intent::hash_bytes(
                            format!("constraint-component:{}", o["operation_id"].as_str().unwrap_or_default()).as_bytes(),
                        ),
                        "operation_id": o["operation_id"].clone(),
                        "source_locator": locator.clone(),
                        "parameters": {"op": o["op"].clone(), "ordering": o["ordering"].clone()},
                    })
                })
                .collect()
        })
        .unwrap_or_default();
    let mut constraint_plan = json!({
        "operation_manifest_id": manifest_id,
        "layout_version": LAYOUT_VERSION,
        "field_id": FIELD_ID,
        "ordered_public_inputs": [],
        "ordered_private_inputs": primary_inputs.clone(),
        "ordered_outputs": primary_outputs.clone(),
        "components": components,
        "required_capabilities": ["constraint-json"],
        "omissions": [],
    });
    let constraint_plan_id = content_id(&constraint_plan, "plan_id")?;
    constraint_plan["plan_id"] = json!(constraint_plan_id);

    // WitnessPlan: one node per operation, dependencies in stable order.
    let ops = manifest["operations"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let mut nodes: Vec<Value> = Vec::with_capacity(ops.len());
    let mut node_by_operation: Vec<(String, String)> = Vec::new();
    for (index, operation) in ops.iter().enumerate() {
        let dependencies: Vec<Value> = operation["witness_dependencies"]
            .as_array()
            .map(|deps| {
                deps.iter()
                    .filter_map(|dep| {
                        node_by_operation
                            .iter()
                            .find(|(op_id, _)| dep.as_str() == Some(op_id.as_str()))
                            .map(|(_, node_id)| json!(node_id))
                    })
                    .collect()
            })
            .unwrap_or_default();
        let mut node = json!({
            "operation_id": operation["operation_id"].clone(),
            "typed_inputs": operation["inputs"].clone(),
            "typed_outputs": operation["outputs"].clone(),
            "dependencies": dependencies,
            "last_use": index + 1 == ops.len(),
        });
        let node_id = content_id(&node, "node_id")?;
        node["node_id"] = json!(node_id.clone());
        node_by_operation.push((
            operation["operation_id"]
                .as_str()
                .unwrap_or_default()
                .to_owned(),
            node_id,
        ));
        nodes.push(node);
    }
    let topological_order: Vec<Value> = nodes.iter().map(|n| n["node_id"].clone()).collect();
    let node_count = nodes.len();
    let mut witness_plan = json!({
        "operation_manifest_id": manifest_id,
        "layout_version": LAYOUT_VERSION,
        "input_bindings": ops.first().map(|o| o["inputs"].clone()).unwrap_or_else(|| json!([])),
        "nodes": nodes,
        "stable_topological_order": topological_order,
        "output_bindings": primary_outputs.clone(),
        "slot_counts": {"nodes": node_count, "slots": node_count},
        "program_kind": WITNESS_PROGRAM_KIND,
    });
    let witness_plan_id = content_id(&witness_plan, "plan_id")?;
    witness_plan["plan_id"] = json!(witness_plan_id);

    let op_names: Vec<Value> = ops.iter().map(|o| o["op"].clone()).collect();
    let backend_capabilities = json!({
        "backend_id": BACKEND_ID,
        "source_membership": "source+component",
        "field_id": FIELD_ID,
        "formats": [
            {"format": "constraint-json", "midnight_zkir_exporter": false, "supplies_witness_computation": false},
            {"format": "structured-witness", "midnight_zkir_exporter": false, "supplies_witness_computation": true},
        ],
        "constraint_operations": op_names.clone(),
        "witness_operations": op_names,
        "lookup": unsupported_capability("the host exporter realizes no lookup argument"),
        "interaction": unsupported_capability("the host exporter realizes no interaction"),
        "custom_gates": unsupported_capability("the host exporter realizes no custom gates"),
        "structured_witness_ir": {"supported": true},
        "native_closures": unsupported_capability("native witness closures are not realized by the host exporter"),
        "field_layouts": unsupported_capability("no field layout is realized by the host exporter"),
    });

    // Exports: canonical request order, deduplicated; write only emitted ones.
    let requested: Vec<ExportRequest> = ExportRequest::ALL
        .into_iter()
        .filter(|r| requests.contains(r))
        .collect();
    let mut exports: Vec<Value> = Vec::with_capacity(requested.len());
    for request in requested {
        let (artifact, plan) = match request {
            ExportRequest::ConstraintJson => (Some(CONSTRAINT_ARTIFACT), Some(&constraint_plan)),
            ExportRequest::StructuredWitness => (Some(WITNESS_ARTIFACT), Some(&witness_plan)),
            _ => (None, None),
        };
        if let (Some(path), Some(plan)) = (artifact, plan) {
            let bytes = crate::intent::canonical(plan)
                .map_err(|error| CompileFailure::input("observation", error.to_string()))?;
            std::fs::create_dir_all(export_dir)
                .and_then(|()| std::fs::write(export_dir.join(path), &bytes))
                .map_err(|error| {
                    CompileFailure::input("export", format!("cannot write {path}: {error}"))
                })?;
            exports.push(json!({
                "request": request.name(),
                "disposition": "emitted",
                "artifact": {"path": path, "sha256": crate::intent::hash_bytes(&bytes), "bytes": bytes.len()},
                "unsupported": Value::Null,
            }));
        } else {
            exports.push(json!({
                "request": request.name(),
                "disposition": "unsupported",
                "artifact": Value::Null,
                "unsupported": {
                    "feature": request.name(),
                    "reason": "capability is not realized by the host exporter",
                    "operation_id": operation_ids.first().cloned().unwrap_or_default(),
                    "source_locator": locator.clone(),
                },
            }));
        }
    }

    Ok(json!({
        "format": FORMAT,
        "step": {
            "position": position.to_string(),
            "rule_start_line": record.rule_start_line.to_string(),
            "source_control": snapshot_control(&record.before),
            "profile_id": PROFILE,
        },
        "operation_manifest": manifest,
        "constraint_plan": constraint_plan,
        "witness_plan": witness_plan,
        "backend_capabilities": backend_capabilities,
        "exports": exports,
        "claims": {"constraint_json_supplies_witness": false, "midnight_zkir_exporter": false},
    }))
}
