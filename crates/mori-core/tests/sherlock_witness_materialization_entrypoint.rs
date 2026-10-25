//! UIREQ-035 public entrypoint: WitnessPlan materialization of chunk boundaries
//! under uncached, last-use, cold-cache and warm-cache schedules.
//!
//! Expectations are authored from the program text: 10 + inner(3) returns 4
//! from inner while inner's own return frame (n = 3) and the caller's pending
//! addition with left operand 10 remain; id([1, 2]) is [1, 2]; fact(3) is 6;
//! fold(branch(leaf(4), leaf(5))) is 9. None is read from the DAG metadata.
use mori_core::sherlock_compile_artifact::draft_chunked::{
    ChunkEvents, ChunkPlan, ChunkRequest, Representation,
};
use mori_core::sherlock_compile_artifact::draft_execution::run_draft_host_chunked;
use serde_json::{Value, json};

const CONFIG: &str = include_str!("fixtures/technical-chunks/config.json");
const TARGET: &str = include_str!("fixtures/technical-chunks/target.json");

const INNER: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn inner(n: UInt): UInt { n + 1 }\n  entry 10 + inner(3);\n}\n";
const LIST: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn id(xs: List): List { xs }\n  entry id(cons(1, cons(2, nil)));\n}\n";
const FACT: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn fact(n: UInt): UInt { if n == 0 { 1 } else { n * fact(n - 1) } }\n  entry fact(3);\n}\n";
const FOLD: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn fold(t: Tree): UInt { match t { leaf(x) => x, branch(l, r) => fold(l) + fold(r) } }\n  entry fold(branch(leaf(4), leaf(5)));\n}\n";

fn chunks(source: &str, plan: ChunkPlan, witness: bool) -> Value {
    let request = ChunkRequest {
        plan,
        events: ChunkEvents::NotRetained,
        represent: Representation::Off,
        witness,
        images: true,
    };
    run_draft_host_chunked(source, CONFIG, TARGET, &request, &mut |_| {})
        .unwrap_or_else(|f| panic!("run failed: {}", serde_json::to_string(&f).unwrap()))
        .1
}

fn n(v: &Value) -> u128 {
    v.as_str().unwrap().parse().unwrap()
}

fn field<'a>(fields: &'a Value, name: &str) -> &'a Value {
    fields["fields"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["field"] == name)
        .unwrap_or_else(|| panic!("no field {name} in {fields}"))
}

/// Schedules agree, and caches change only diagnostics.
fn check_schedules(chunk: &Value) -> &Value {
    let w = &chunk["witness"];
    assert_eq!(w["status"], "materialized", "{w}");
    assert_eq!(w["schedules_agree"], true);
    assert_eq!(w["plan"]["stable_order_is_identity"], true);
    let d = &w["diagnostics"];
    assert_eq!(d["uncached"]["evaluated_nodes"], w["plan"]["nodes"]);
    assert!(n(&d["cold_cache"]["scheduling"]["cache_misses"]) > 0);
    assert_eq!(d["warm_cache"]["scheduling"]["cache_misses"], "0");
    assert!(n(&d["warm_cache"]["scheduling"]["cache_hits"]) > 0);
    // The decoded counters agree with the owner's own boundary observation.
    for counter in [
        "consumed_work",
        "remaining_work",
        "helper_work",
        "helper_visits",
        "reference_cursor",
    ] {
        assert_eq!(
            w["decoded"][counter], chunk["end_counters"][counter],
            "{counter}"
        );
    }
    &w["decoded"]["fields"]
}

fn final_outcome(source: &str) -> Value {
    let c = chunks(source, ChunkPlan::Every(100_000), true);
    let last = c["chunks"].as_array().unwrap().last().unwrap().clone();
    let fields = check_schedules(&last).clone();
    let outcome = &fields["outcome"];
    assert_eq!(outcome["payload"], "Outcome");
    assert!(outcome["raw"].as_str().unwrap().contains("Halted"));
    field(outcome, "value")["value"]["value"].clone()
}

#[test]
fn halted_states_materialize_identically_under_every_schedule() {
    assert_eq!(final_outcome(LIST), json!(["1", "2"]));
    assert_eq!(final_outcome(FACT), json!("6"));
    assert_eq!(final_outcome(FOLD), json!("9"));
    assert_eq!(final_outcome(INNER), json!("14"));
}

#[test]
fn pending_caller_state_materializes_its_frames_and_environment() {
    // Locate the cut where inner(3) has produced 4 by the owner's own image.
    let all = chunks(INNER, ChunkPlan::Every(1), false);
    let position = all["chunks"]
        .as_array()
        .unwrap()
        .iter()
        .find_map(|c| {
            let image = &c["boundary"]["image"];
            let index: usize = image["control"]["Return"]["value"]["Data"]
                .as_str()?
                .parse()
                .ok()?;
            let four = image["data"][index]["U128"]["n"] == "4";
            (four && !image["frames"].as_array()?.is_empty()).then(|| c["end_event"].clone())
        })
        .expect("a pending return of 4");
    let cut = chunks(INNER, ChunkPlan::At(vec![n(&position)]), true);
    let first = &cut["chunks"][0];
    assert_eq!(first["end"], "technical-cut");
    let fields = check_schedules(first);
    assert_eq!(field(&fields["control"], "value")["value"]["value"], "4");
    let frames = fields["frames"]["items"].as_array().unwrap();
    assert_eq!(frames.len(), 2, "{frames:?}");
    assert!(frames[0]["raw"].as_str().unwrap().contains("ReturnF"));
    let pending = &frames[1];
    assert!(
        pending["raw"]
            .as_str()
            .unwrap()
            .contains("RightF { op: Add")
    );
    assert_eq!(field(pending, "left")["value"]["value"], "10");
    assert_eq!(
        fields["current_environment"]["environment"],
        json!([{"binder": "1", "name": "n", "value": "3"}])
    );
}
