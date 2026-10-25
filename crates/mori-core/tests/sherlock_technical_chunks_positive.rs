//! Technical chunks of a real dedicated-draft owner run (UIREQ-013, UIREQ-016).
//!
//! Expected results are authored by hand from the program text (10 + inner(3)
//! is 14, fact(3) is 6, fold(branch(leaf(4), leaf(5))) is 9). Chunked runs are
//! compared with an unchunked run of the same source, never with values derived
//! from the chunk driver itself.
use mori_core::sherlock::helper_machine::complete_observation::capture;
use mori_core::sherlock::helper_machine::shared::host_implied_prefix::SegBase;
use mori_core::sherlock::helper_machine::shared::owner_resume::{
    ResumeError, resume_technical_checkpoint,
};
use mori_core::sherlock::helper_machine::shared::{ActiveLengths, AllocCounters};
use mori_core::sherlock_compile_artifact::draft_chunked::{
    ChunkEvents, ChunkPlan, ChunkRequest, Representation,
};
use mori_core::sherlock_compile_artifact::draft_execution::{
    run_draft_host, run_draft_host_chunked,
};
use mori_core::sherlock_compile_artifact::draft_simulation_report::report;
use serde_json::Value;

const CONFIG: &str = include_str!("fixtures/technical-chunks/config.json");
const TARGET: &str = include_str!("fixtures/technical-chunks/target.json");

const INNER: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn inner(n: UInt): UInt { n + 1 }\n  entry 10 + inner(3);\n}\n";
const FACT: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn fact(n: UInt): UInt { if n == 0 { 1 } else { n * fact(n - 1) } }\n  entry fact(3);\n}\n";
const FOLD: &str = "language \"moriarty-sherlock-draft/1\";\nprogram P {\n  fn fold(t: Tree): UInt { match t { leaf(x) => x, branch(l, r) => fold(l) + fold(r) } }\n  entry fold(branch(leaf(4), leaf(5)));\n}\n";

fn plain(source: &str) -> Value {
    let run = run_draft_host(source, CONFIG, TARGET, &mut |_| {})
        .unwrap_or_else(|f| panic!("plain run failed: {}", serde_json::to_string(&f).unwrap()));
    serde_json::to_value(report(&run).unwrap_or_else(|_| panic!("plain report failed"))).unwrap()
}

fn chunked(source: &str, plan: ChunkPlan, events: ChunkEvents) -> (Value, Value) {
    let request = ChunkRequest {
        plan,
        events,
        represent: Representation::Off,
        witness: false,
        images: true,
    };
    let (run, chunks) = run_draft_host_chunked(source, CONFIG, TARGET, &request, &mut |_| {})
        .unwrap_or_else(|f| panic!("chunked run failed: {}", serde_json::to_string(&f).unwrap()));
    let full =
        serde_json::to_value(report(&run).unwrap_or_else(|_| panic!("chunked report failed")))
            .unwrap();
    (full, chunks)
}

fn n(v: &Value) -> u128 {
    v.as_str()
        .unwrap_or_else(|| panic!("not a decimal string: {v}"))
        .parse()
        .unwrap()
}

fn exact(v: &Value) -> u128 {
    assert_eq!(v["status"], "exact", "inexact total {v}");
    n(&v["value"])
}

/// Decode a scalar UInt reference in a report image.
fn scalar(image: &Value, reference: &Value) -> u128 {
    let index: usize = reference["Data"]
        .as_str()
        .expect("data reference")
        .parse()
        .unwrap();
    n(&image["data"][index]["U128"]["n"])
}

fn halted_value(full: &Value) -> u128 {
    let execution = &full["execution"];
    scalar(
        &execution["image"],
        &execution["summary"]["outcome"]["Halted"]["value"],
    )
}

fn assert_transparent(source: &str, expected: u128, plan: ChunkPlan) {
    let one = plain(source);
    assert_eq!(halted_value(&one), expected, "plain result");
    let (full, chunks) = chunked(source, plan.clone(), ChunkEvents::NotRetained);
    // Same result, final complete image and owner counters as one trace.
    assert_eq!(halted_value(&full), expected, "{plan:?}");
    assert_eq!(
        full["execution"]["summary"], one["execution"]["summary"],
        "{plan:?}"
    );
    assert_eq!(
        full["execution"]["image"], one["execution"]["image"],
        "{plan:?}"
    );
    assert_eq!(
        full["execution"]["resource_report"], one["execution"]["resource_report"],
        "{plan:?}"
    );
    let summary = &one["execution"]["summary"];
    let list = chunks["chunks"].as_array().unwrap();
    assert_eq!(n(&chunks["chunk_count"]), list.len() as u128);
    // Chunks are contiguous, and each chunk's semantic charges sum to the trace.
    let mut at = 0u128;
    let mut work = 0u128;
    let mut helper = 0u128;
    let mut cursor = 0u128;
    let mut events = 0u128;
    let mut cuts = 0u128;
    let mut previous_consumed = 0u128;
    for (i, c) in list.iter().enumerate() {
        assert_eq!(n(&c["start_event"]), at, "{plan:?} chunk {i}");
        let end = n(&c["end_event"]);
        let acc = &c["charges"];
        assert_eq!(n(&acc["events"]), end - at);
        work += n(&acc["work"]);
        helper += n(&acc["helper_work"]);
        cursor += n(&acc["reference_cursor"]);
        events += n(&acc["events"]);
        let delta = n(&c["end_counters"]["consumed_work"]) - previous_consumed;
        assert_eq!(delta, n(&acc["work"]), "{plan:?} chunk {i}");
        previous_consumed = n(&c["end_counters"]["consumed_work"]);
        if c["end"] == "technical-cut" {
            cuts += 1;
            let cp = &c["checkpoint"];
            assert_eq!(cp["format"], "C6RAWHS1");
            assert_eq!(cp["resumed_from_archive"], true);
            assert_eq!(cp["financial_commit"], "none");
            assert_eq!(cp["segment_base"], "genesis-uncut");
            assert!(n(&cp["bytes"]) > 8);
            assert_eq!(c["boundary"]["outcome"], "Running");
        } else {
            assert_eq!(i + 1, list.len(), "only the last chunk is terminal");
            assert!(c["checkpoint"].is_null());
        }
        at = end;
    }
    assert_eq!(n(&chunks["technical_cuts"]), cuts);
    assert_eq!(events, n(&summary["events"]));
    assert_eq!(work, n(&summary["consumed_work"]));
    assert_eq!(helper, n(&summary["helper_work"]));
    assert_eq!(cursor, n(&summary["reference_cursor"]));
    assert_eq!(exact(&chunks["cumulative"]["work_delta_sum"]), work);
    assert_eq!(chunks["financial_commits"], "0");
    // Separate fields stay separate; nothing physical is estimated.
    let manifest = &chunks["resource_manifest"];
    assert_eq!(
        manifest["reference_schedule_id"],
        "sherlock-helper-microstep/1"
    );
    assert_eq!(manifest["same_fuel_equivalence"], "not-claimed");
    assert_eq!(manifest["measured_cost_winner"], "none");
    let retained = manifest["retained_profiles"].as_array().unwrap();
    let charge = |id: &str| {
        retained
            .iter()
            .find(|p| p["reference_schedule_id"] == id)
            .unwrap()["terminal_return_charge"]
            .clone()
    };
    assert_ne!(charge("sherlock-k-a/1"), charge("sherlock-quint-a/1"));
    let unmeasured = &chunks["separate_fields"]["unmeasured"];
    for group in ["physical", "circuit", "proof", "host", "financial_target"] {
        assert_eq!(unmeasured[group]["status"], "unmeasured", "{group}");
    }
}

#[test]
fn chunked_runs_match_one_trace_for_every_cut_plan() {
    for (source, expected) in [(INNER, 14), (FACT, 6), (FOLD, 9)] {
        for k in [1, 2, 3, 5, 8, 100_000] {
            assert_transparent(source, expected, ChunkPlan::Every(k));
        }
        assert_transparent(source, expected, ChunkPlan::At(vec![1, 7, 20]));
        assert_transparent(source, expected, ChunkPlan::At(vec![100_000]));
    }
}

#[test]
fn two_technical_chunks_sum_to_the_one_trace_work() {
    let one = plain(FACT);
    let total = n(&one["execution"]["summary"]["events"]);
    let (_, chunks) = chunked(
        FACT,
        ChunkPlan::At(vec![total / 2]),
        ChunkEvents::NotRetained,
    );
    let list = chunks["chunks"].as_array().unwrap();
    assert_eq!(list.len(), 2);
    let w: u128 = list.iter().map(|c| n(&c["charges"]["work"])).sum();
    assert_eq!(w, n(&one["execution"]["summary"]["consumed_work"]));
}

/// UIREQ-016: a cut after inner(3) returns 4 and before the caller's addition.
#[test]
fn boundary_between_callee_return_and_caller_addition_is_transparent() {
    let (full, chunks) = chunked(INNER, ChunkPlan::Every(1), ChunkEvents::Retained);
    assert_eq!(halted_value(&full), 14);
    let list = chunks["chunks"].as_array().unwrap();
    // Use only the always-published boundary view (no full image needed).
    let found = list.iter().find(|c| {
        let view = &c["boundary"]["view"];
        c["end"] == "technical-cut"
            && view["control"].get("Return").is_some()
            && view["returned_data_row"]["U128"]["n"] == "4"
            && !view["frames"].as_array().unwrap().is_empty()
    });
    let cut = found.expect("a cut with the callee result 4 returning into a pending caller frame");
    // The pending caller frame is the left-evaluated addition of 10.
    let frames = cut["boundary"]["view"]["frames"].as_array().unwrap();
    assert!(
        frames.iter().any(|f| f.to_string().contains("Add")),
        "pending caller addition frame retained: {frames:?}"
    );
    // The full image agrees with the view.
    assert_eq!(
        cut["boundary"]["image"]["frames"],
        cut["boundary"]["view"]["frames"]
    );
    assert_eq!(cut["checkpoint"]["resumed_capture"], "equal");
    // Every chunk retains its own owner events, in order and without gaps.
    let mut position = 0u128;
    for c in list {
        for e in c["events"].as_array().unwrap() {
            position += 1;
            assert_eq!(n(&e["position"]), position);
        }
    }
    assert_eq!(position, n(&full["execution"]["summary"]["events"]));
}

#[test]
fn invalid_chunk_plans_are_refused_before_execution() {
    for plan in [
        ChunkPlan::Every(0),
        ChunkPlan::At(vec![]),
        ChunkPlan::At(vec![0, 2]),
        ChunkPlan::At(vec![3, 3]),
        ChunkPlan::At(vec![5, 2]),
    ] {
        let request = ChunkRequest {
            plan: plan.clone(),
            events: ChunkEvents::NotRetained,
            represent: Representation::Off,
            witness: false,
            images: true,
        };
        let mut seen = 0;
        let refused = run_draft_host_chunked(INNER, CONFIG, TARGET, &request, &mut |_| seen += 1);
        assert!(refused.is_err(), "{plan:?} accepted");
        assert_eq!(seen, 0, "{plan:?} executed");
    }
}

#[test]
fn resume_reproduces_the_checkpoint_and_refuses_inconsistent_ones() {
    let run = run_draft_host(FACT, CONFIG, TARGET, &mut |_| {}).unwrap_or_else(|_| panic!("run"));
    let admitted = &run.completed.admitted;
    let snapshot = capture(admitted, &run.completed.image).unwrap_or_else(|_| panic!("capture"));
    let resumed = resume_technical_checkpoint(admitted, &snapshot).expect("resume");
    assert_eq!(
        capture(admitted, &resumed).unwrap_or_else(|_| panic!("recapture")),
        snapshot
    );

    let mut cut = snapshot.clone();
    cut.image.seg_base = SegBase::from_parts(
        ActiveLengths {
            data: 1,
            data_fields: 0,
            env: 0,
            value_slots: 0,
        },
        AllocCounters::default(),
        1,
    );
    assert!(matches!(
        resume_technical_checkpoint(admitted, &cut),
        Err(ResumeError::SegmentBaseNotGenesis)
    ));

    let mut work = snapshot.clone();
    work.image.remaining_work += 1;
    assert!(matches!(
        resume_technical_checkpoint(admitted, &work),
        Err(ResumeError::StepInvariant(_))
    ));

    let mut truncated = snapshot.clone();
    truncated.image.data.pop();
    assert!(matches!(
        resume_technical_checkpoint(admitted, &truncated),
        Err(ResumeError::StepInvariant(_))
    ));
}

fn represented(source: &str, config: &str, plan: ChunkPlan) -> Value {
    let request = ChunkRequest {
        plan,
        events: ChunkEvents::NotRetained,
        represent: Representation::Bounded,
        witness: false,
        images: true,
    };
    let (_, chunks) = run_draft_host_chunked(source, config, TARGET, &request, &mut |_| {})
        .unwrap_or_else(|f| panic!("chunked run failed: {}", serde_json::to_string(&f).unwrap()));
    chunks
}

/// UIREQ-029: a pending factorial state with a caller environment, pending
/// multiplication and a live helper cursor is in-domain for the bounded
/// C6P1 layout, and its decoded image equals the owner's.
#[test]
fn pending_factorial_states_have_a_bounded_representation() {
    let chunks = represented(FACT, CONFIG, ChunkPlan::Every(1));
    let mut pending_helper = 0;
    for c in chunks["chunks"].as_array().unwrap() {
        let r = &c["representation"];
        assert_eq!(r["status"], "in-domain", "chunk {}: {r}", c["index"]);
        assert_eq!(r["decoded_equals_owner_image"], true);
        assert_eq!(r["decoded_equals_capture"], true);
        assert_eq!(r["represented_image_claim"], true);
        assert_eq!(r["encode_decode_correspondence"], "unproved");
        assert!(n(&r["lanes"]) > 0);
        let image = &c["boundary"]["image"];
        let frames = image["frames"].to_string();
        if !image["helper"].is_null()
            && frames.contains("Mul")
            && !image["env"].as_array().unwrap().is_empty()
        {
            pending_helper += 1;
        }
    }
    assert!(
        pending_helper > 0,
        "no pending multiplication with a helper cursor"
    );
}

/// A state beyond the representation capacities has no represented-image claim.
#[test]
fn states_beyond_the_lane_capacity_are_explicitly_out_of_domain() {
    let mut config: Value = serde_json::from_str(CONFIG).unwrap();
    config["bounds"]["serialization_bytes"] = Value::String("4096".into());
    let chunks = represented(FACT, &config.to_string(), ChunkPlan::Every(100_000));
    let last = chunks["chunks"].as_array().unwrap().last().unwrap();
    let r = &last["representation"];
    assert_eq!(r["status"], "out-of-domain", "{r}");
    assert_eq!(r["refused_by"], "encode");
    assert_eq!(r["represented_image_claim"], false);
    assert!(r.get("lanes").is_none());
}
