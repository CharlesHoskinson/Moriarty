//! UIREQ-024 public replay: `optimization_report` runs the
//! intern-equal-tree-nodes candidate against the unchanged `sherlock-k-a/1`
//! reference run of the same draft source and reports equal logical counters
//! with the physical tree-allocation counters of both realizations.
//!
//! Hand-authored expectations: fold(branch(leaf(3), leaf(3))) is 6 and builds
//! two equal leaf children at runtime, so the reference holds 2 child
//! allocations where the candidate holds 1; fact(2) is 2 and holds no tree, so
//! the candidate is not applied; a fuel of 3 exhausts both realizations.
use mori_core::sherlock_compile_artifact::draft_optimization::{
    CANDIDATE, RUN_FORMAT, optimization_report,
};
use mori_core::sherlock_compile_artifact::draft_reference_slices::{
    reference_slices, reference_slices_with_candidate,
};
use serde_json::Value;

const CONFIG: &str = include_str!("fixtures/technical-chunks/config.json");
const TARGET: &str = include_str!("fixtures/technical-chunks/target.json");
const EQUAL: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn fold(t: Tree): UInt { match t { leaf(x) => x, branch(l, r) => fold(l) + fold(r) } }\n  entry fold(branch(leaf(3), leaf(3)));\n}\n";
const FACT: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn fact(n: UInt): UInt { if n == 0 { 1 } else { n * fact(n - 1) } }\n  entry fact(2);\n}\n";

fn config(fuel: Option<u128>) -> String {
    let mut c: Value = serde_json::from_str(CONFIG).unwrap();
    if let Some(f) = fuel {
        c["fuel"]["value"] = Value::String(f.to_string());
    }
    c.to_string()
}

fn report(source: &str, fuel: Option<u128>) -> Value {
    optimization_report(source, &config(fuel), TARGET, CANDIDATE)
        .unwrap_or_else(|f| panic!("{}", serde_json::to_string(&f).unwrap()))
}

fn logical_equal(r: &Value) {
    let lg = r["logical"].as_object().unwrap();
    assert!(lg.contains_key("events") && lg.contains_key("work") && lg.contains_key("outcome"));
    for (k, v) in lg {
        assert_eq!(v["reference"], v["candidate"], "logical {k}");
    }
}

#[test]
fn replay_shares_runtime_equal_leaves() {
    let r = report(EQUAL, None);
    assert_eq!(r["format"], RUN_FORMAT);
    assert_eq!(r["candidate"], "sherlock-a-intern-equal-tree-nodes/1");
    assert_eq!(r["applied"], true);
    logical_equal(&r);
    assert_eq!(r["logical"]["outcome"]["reference"], "Done(6)");
    let p = &r["physical"]["peak_tree_child_allocations"];
    assert_eq!(p["reference"], "2");
    assert_eq!(p["candidate"], "1");
}

#[test]
fn replay_without_trees_is_not_applied() {
    let r = report(FACT, None);
    assert_eq!(r["applied"], false);
    logical_equal(&r);
    assert_eq!(r["logical"]["outcome"]["candidate"], "Done(2)");
}

#[test]
fn replay_preserves_exhaustion() {
    let r = report(EQUAL, Some(3));
    logical_equal(&r);
    assert_eq!(r["logical"]["outcome"]["reference"], "Exhausted");
}

#[test]
fn replay_rejects_unknown_candidate() {
    let e = optimization_report(EQUAL, &config(None), TARGET, "sherlock-a-other/1");
    assert!(e.is_err(), "an unknown candidate must be refused");
}

#[test]
fn candidate_slices_keep_the_reference_stream() {
    for m in [1, 2, 4] {
        let a =
            reference_slices(EQUAL, &config(None), TARGET, m).unwrap_or_else(|_| panic!("slices"));
        let b = reference_slices_with_candidate(EQUAL, &config(None), TARGET, m)
            .unwrap_or_else(|_| panic!("slices"));
        assert_eq!(
            a["slices"].as_array().unwrap().len(),
            b["slices"].as_array().unwrap().len()
        );
        assert_eq!(a, b, "M={m}: interning changed the reference slices");
    }
}
