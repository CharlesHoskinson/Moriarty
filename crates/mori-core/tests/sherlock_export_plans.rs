//! UIREQ-031: separate export plans for one bounded K-A step of fact(3).
//!
//! Expectations are hand-authored from the contract: the final checked
//! multiplication must carry a `CheckedU128Mul` operation and a `ReturnF`
//! frame return a `FrameRestore`; both plans share one content-addressed
//! manifest, emitted artifacts hash to the reported bytes, and unsupported
//! capabilities stay located `Unsupported` without witness claims.
use std::path::PathBuf;

use mori_core::intent::hash_bytes;
use mori_core::sherlock_compile_artifact::draft_reference_slices::reference_slices;
use mori_core::sherlock_compile_artifact::export_plans::{ExportRequest, export_plans};
use serde_json::Value;

const CONFIG: &str = include_str!("fixtures/technical-chunks/config.json");
const TARGET: &str = include_str!("fixtures/technical-chunks/target.json");
const FACT3: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn fact(n: UInt): UInt { if n == 0 { 1 } else { n * fact(n - 1) } }\n  entry fact(3);\n}\n";

const CAPS: [ExportRequest; 6] = [
    ExportRequest::ConstraintJson,
    ExportRequest::StructuredWitness,
    ExportRequest::NativeWitnessClosure,
    ExportRequest::Lookup,
    ExportRequest::Interaction,
    ExportRequest::CustomGate,
];

fn plans(position: u64, requests: &[ExportRequest], dir: &std::path::Path) -> Value {
    plans_in(FACT3, position, requests, dir)
}

fn plans_in(
    source: &str,
    position: u64,
    requests: &[ExportRequest],
    dir: &std::path::Path,
) -> Value {
    export_plans(source, CONFIG, TARGET, position, requests, dir)
        .unwrap_or_else(|f| panic!("{}", serde_json::to_string(&f).unwrap()))
}

fn temp_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("dw-export-test-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// (1-based position, reference record) of the first slice matching `pred`.
fn select(pred: impl Fn(&Value) -> bool) -> (u64, Value) {
    select_in(FACT3, pred)
}

fn select_in(source: &str, pred: impl Fn(&Value) -> bool) -> (u64, Value) {
    let report = reference_slices(source, CONFIG, TARGET, 1)
        .unwrap_or_else(|f| panic!("{}", serde_json::to_string(&f).unwrap()));
    for (index, slice) in report["slices"].as_array().unwrap().iter().enumerate() {
        if pred(slice) {
            return ((index + 1) as u64, slice["records"][0].clone());
        }
    }
    panic!("no reference slice matches the selector");
}

fn check_step(position: u64, record: &Value, want_op: &str) {
    let dir = temp_dir(&format!("{position}"));
    let report = plans(position, &CAPS, &dir);
    assert_eq!(report["format"], "moriarty-sherlock-export-plans/1");
    let step = &report["step"];
    assert_eq!(step["position"].as_str().unwrap(), position.to_string());
    assert_eq!(step["rule_start_line"], record["rule_start_line"]);
    assert_eq!(step["source_control"], record["source_control"]);
    assert_eq!(step["profile_id"], "sherlock-k-a/1");

    let manifest = &report["operation_manifest"];
    let constraint = &report["constraint_plan"];
    let witness = &report["witness_plan"];
    let ids = [
        manifest["manifest_id"].as_str().unwrap(),
        constraint["plan_id"].as_str().unwrap(),
        witness["plan_id"].as_str().unwrap(),
    ];
    assert!(
        ids.iter()
            .all(|id| id.len() == 64 && id.bytes().all(|b| b.is_ascii_hexdigit()))
    );
    assert!(
        ids[0] != ids[1] && ids[1] != ids[2] && ids[0] != ids[2],
        "ids must be distinct: {ids:?}"
    );
    assert_eq!(
        constraint["operation_manifest_id"].as_str().unwrap(),
        ids[0]
    );
    assert_eq!(witness["operation_manifest_id"].as_str().unwrap(), ids[0]);

    let operations = manifest["operations"].as_array().unwrap();
    assert!(
        operations.iter().any(|o| o["op"] == want_op),
        "missing {want_op} in {operations:?}"
    );
    // Every state-path input must bind its actual value, and the checked
    // multiplication must bind the captured left and returned right operands.
    for operation in operations {
        for input in operation["inputs"].as_array().unwrap() {
            if input.get("source").is_some() {
                assert!(input["value"].is_string(), "unbound state input {input}");
            }
        }
    }
    if want_op == "CheckedU128Mul" {
        let mul = operations.iter().find(|o| o["op"] == want_op).unwrap();
        let values: Vec<&str> = mul["inputs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|i| i["value"].as_str().unwrap())
            .collect();
        assert_eq!(values, ["uiU(3)", "uiU(2)"], "actual mul operands");
        assert_eq!(mul["outputs"][0]["value"], "uiU(6)");
    }
    let covered: std::collections::BTreeSet<&str> = constraint["components"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["operation_id"].as_str().unwrap())
        .collect();
    assert!(
        operations
            .iter()
            .all(|o| covered.contains(o["operation_id"].as_str().unwrap()))
    );

    let nodes = witness["nodes"].as_array().unwrap();
    let order = witness["stable_topological_order"].as_array().unwrap();
    assert_eq!(order.len(), nodes.len());
    for node in nodes {
        for dep in node["dependencies"].as_array().unwrap() {
            let dep_at = order.iter().position(|n| n == dep).unwrap();
            let node_at = order.iter().position(|n| n == &node["node_id"]).unwrap();
            assert!(dep_at < node_at, "dependency must precede its node");
        }
    }

    let capabilities = &report["backend_capabilities"];
    for key in [
        "lookup",
        "interaction",
        "custom_gates",
        "structured_witness_ir",
        "native_closures",
        "field_layouts",
    ] {
        assert!(
            capabilities[key]["supported"].is_boolean(),
            "capability {key}"
        );
    }
    assert_eq!(capabilities["structured_witness_ir"]["supported"], true);
    assert_eq!(
        report["claims"],
        serde_json::json!({"constraint_json_supplies_witness": false, "midnight_zkir_exporter": false})
    );

    let exports = report["exports"].as_array().unwrap();
    assert_eq!(exports.len(), CAPS.len());
    let mut on_disk: Vec<String> = Vec::new();
    for export in exports {
        if export["disposition"] == "emitted" {
            let artifact = &export["artifact"];
            let path = artifact["path"].as_str().unwrap();
            let bytes = std::fs::read(dir.join(path)).unwrap();
            assert_eq!(hash_bytes(&bytes), artifact["sha256"].as_str().unwrap());
            assert_eq!(bytes.len() as u64, artifact["bytes"].as_u64().unwrap());
            on_disk.push(path.to_owned());
        } else {
            assert_eq!(export["disposition"], "unsupported", "{export}");
            assert!(
                export["unsupported"]["operation_id"].is_string()
                    || export["unsupported"]["source_locator"].is_string()
            );
        }
    }
    on_disk.sort();
    assert_eq!(on_disk, ["constraint-plan.json", "witness-plan.json"]);

    // The constraint artifact names the ConstraintPlan and carries no witness keys.
    let constraint_doc: Value =
        serde_json::from_slice(&std::fs::read(dir.join("constraint-plan.json")).unwrap()).unwrap();
    assert_eq!(constraint_doc["plan_id"], constraint["plan_id"]);
    assert!(
        constraint_doc.get("nodes").is_none()
            && constraint_doc.get("stable_topological_order").is_none()
    );
    let witness_doc: Value =
        serde_json::from_slice(&std::fs::read(dir.join("witness-plan.json")).unwrap()).unwrap();
    assert_eq!(witness_doc["plan_id"], witness["plan_id"]);
    assert_eq!(witness_doc["nodes"].as_array().unwrap().len(), nodes.len());

    // One capability alone gives the same disposition and bytes as together.
    let alone_dir = temp_dir(&format!("{position}-alone"));
    let alone = plans(position, &[ExportRequest::ConstraintJson], &alone_dir);
    assert_eq!(
        alone["operation_manifest"]["manifest_id"],
        manifest["manifest_id"]
    );
    assert_eq!(
        std::fs::read(alone_dir.join("constraint-plan.json")).unwrap(),
        std::fs::read(dir.join("constraint-plan.json")).unwrap()
    );
    let _ = std::fs::remove_dir_all(&dir);
    let _ = std::fs::remove_dir_all(&alone_dir);
}

#[test]
fn checked_mul_step_exports_separate_plans() {
    let (position, record) =
        select(|slice| slice["end"]["control"] == "uiPrimResult(uiPrimValue(uiU(6)))");
    check_step(position, &record, "CheckedU128Mul");
}

#[test]
fn return_frame_step_exports_frame_restore() {
    let (position, record) = select(|slice| {
        slice["start"]["control"]
            .as_str()
            .is_some_and(|c| c.starts_with("uiRet("))
            && slice["start"]["frames"][0]
                .as_str()
                .is_some_and(|f| f.starts_with("uiReturnF("))
    });
    check_step(position, &record, "FrameRestore");
}

#[test]
fn position_zero_is_refused_and_no_request_writes_nothing() {
    let dir = temp_dir("zero");
    assert!(export_plans(FACT3, CONFIG, TARGET, 0, &CAPS, &dir).is_err());
    let report = plans(1, &[], &dir);
    assert_eq!(report["exports"].as_array().unwrap().len(), 0);
    assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 0);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn scope_return_is_not_misclassified_as_pending_arithmetic() {
    // A match arm body returns through a Scope frame (rule 195) while a
    // Right(Mul) frame is pending below it: that step is a scope restore,
    // not the pending multiplication. (fact has no ScopeF; use match.)
    const SCOPED: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn value(t: Tree): UInt { match t { leaf(x) => x, branch(l, r) => value(l) } }\n  entry 2 * value(leaf(3));\n}\n";
    let (position, _record) = select_in(SCOPED, |slice| {
        slice["start"]["control"]
            .as_str()
            .is_some_and(|c| c.starts_with("uiRet("))
            && slice["start"]["frames"][0]
                .as_str()
                .is_some_and(|f| f.starts_with("uiScopeF("))
    });
    let dir = temp_dir(&format!("{position}-scope"));
    let report = plans_in(SCOPED, position, &CAPS, &dir);
    let operations = report["operation_manifest"]["operations"]
        .as_array()
        .unwrap();
    assert!(
        operations.iter().all(|o| o["op"] != "CheckedU128Mul"),
        "{operations:?}"
    );
    assert!(
        operations.iter().any(|o| o["op"] == "ScopeRestore"),
        "{operations:?}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn checked_mul_overflow_exports_error_typed_result() {
    const OVERFLOW: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn f(n: UInt): UInt { 2 * n }\n  entry f(340282366920938463463374607431768211455);\n}\n";
    let (position, _record) = select_in(OVERFLOW, |slice| {
        slice["end"]["control"]
            .as_str()
            .is_some_and(|c| c.starts_with("uiPrimResult(uiPrimError("))
    });
    let dir = temp_dir(&format!("{position}-overflow"));
    let report = plans_in(OVERFLOW, position, &CAPS, &dir);
    let operations = report["operation_manifest"]["operations"]
        .as_array()
        .unwrap();
    let mul = operations
        .iter()
        .find(|o| o["op"] == "CheckedU128Mul")
        .expect("checked mul op");
    assert_eq!(mul["outputs"][0]["type"], "CheckedU128Result", "{mul}");
    assert!(mul["outputs"][0]["error_code"].is_string(), "{mul}");
    assert!(
        mul["outputs"][0]["value"]
            .as_str()
            .unwrap()
            .contains("uiPrimError"),
        "{mul}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn bool_equality_exports_generic_value_eq() {
    const EQ: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn f(b: Bool): Bool { b == true }\n  entry f(true);\n}\n";
    let (position, _record) = select_in(EQ, |slice| {
        slice["end"]["control"] == "uiPrimResult(uiPrimValue(uiB(true)))"
    });
    let dir = temp_dir(&format!("{position}-eq"));
    let report = plans_in(EQ, position, &CAPS, &dir);
    let operations = report["operation_manifest"]["operations"]
        .as_array()
        .unwrap();
    let eq = operations
        .iter()
        .find(|o| o["op"] == "ValueEq")
        .expect("generic ValueEq for Bool operands");
    assert!(
        operations.iter().all(|o| o["op"] != "UInt128Eq"),
        "{operations:?}"
    );
    assert_eq!(eq["inputs"][0]["type"], "Bool", "{eq}");
    let _ = std::fs::remove_dir_all(&dir);
}
