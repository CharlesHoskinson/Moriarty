//! Public reference-stage consumers against literals frozen before implementation.
//! No test in this file reaches native, proof, replay, or ledger execution; the
//! H16/H19 `--trace` and `compile-call` requests are refused at the profile first.
use serde_json::{Value, json};
use std::{
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);
const DELIVERY: &str = include_str!("../../../examples/core/continuations/delivery.mori");
const RESERVE: &str = include_str!("../../../examples/core/continuations/reserve.mori");
const LEGACY: &str = include_str!("../../../examples/core/continuations/atomic-compatibility.mori");
fn oracle() -> Value {
    serde_json::from_str(include_str!(
        "../../../examples/core/continuations/normal-oracle.json"
    ))
    .unwrap()
}
struct Files(PathBuf);
impl Files {
    fn new(source: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "mori-core-continuations-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        let f = Self(path);
        f.write("source.mori", source.as_bytes());
        f
    }
    fn path(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
    fn write(&self, name: &str, bytes: impl AsRef<[u8]>) {
        std::fs::write(self.path(name), bytes).unwrap();
    }
    fn json(&self, name: &str, value: &Value) {
        self.write(name, serde_json::to_vec(value).unwrap());
    }
    fn command(&self, command: &str) -> Command {
        let mut c = Command::new(env!("CARGO_BIN_EXE_mori"));
        c.args(["--color", "never", command])
            .arg(self.path("source.mori"));
        c
    }
    fn start(&self, action: &str, opening: &Value, cap: Option<&str>) -> Output {
        self.json("scenario.json", opening);
        let mut c = self.command("simulate");
        c.args(["--action", action, "--scenario"])
            .arg(self.path("scenario.json"));
        if let Some(cap) = cap {
            c.args(["--logical-cap", cap]);
        }
        c.output().unwrap()
    }
    fn resume(&self, continuation: &Value, event: &Value, action: Option<&str>) -> Output {
        self.json("continuation.json", continuation);
        self.json("event.json", event);
        let mut c = self.command("simulate");
        c.arg("--resume")
            .arg(self.path("continuation.json"))
            .arg("--event")
            .arg(self.path("event.json"));
        if let Some(action) = action {
            c.args(["--action", action]);
        }
        c.output().unwrap()
    }
}
impl Drop for Files {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn report(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout).unwrap_or_else(|e| {
        panic!(
            "{e}: stdout={} stderr={}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    })
}
fn accepted(output: &Output) -> Value {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let r = report(output);
    for (key, expected) in oracle()["normal_output_labels_from_opus"]
        .as_object()
        .unwrap()
    {
        assert_eq!(&r[key], expected, "{key}");
    }
    r
}
fn segment(actual: &Value, expected: &Value) {
    let execution = &actual["execution"];
    assert_eq!(execution["semantics"], "moriarty-core/2");
    for key in ["pre", "post", "effects"] {
        assert_eq!(execution[key], expected[key], "{key}");
    }
    let steps = execution["steps"].as_array().unwrap();
    let wanted = expected["steps"].as_array().unwrap();
    assert_eq!(steps.len(), wanted.len());
    for (step, want) in steps.iter().zip(wanted) {
        for key in ["index", "value", "snapshot", "effects"] {
            assert_eq!(step[key], want[key], "{key}");
        }
    }
}
fn waiting(r: &Value, expected: &Value) {
    assert_eq!(r["status"], "waiting");
    assert_eq!(r["advanced"], expected["advanced"]);
    assert_eq!(
        r["attempted"],
        expected["attempted"].as_u64().unwrap().to_string()
    );
    assert_eq!(
        r["committed"],
        expected["committed"].as_u64().unwrap().to_string()
    );
    let c = &r["continuation"];
    assert_eq!(c["site"], expected["site"]);
    assert_eq!(c["captures"], expected["captures"]);
    assert_eq!(c["boundary"], expected["segment"]["post"]);
    assert_eq!(c["logical"]["committed"], r["committed"]);
    assert_eq!(r["residual"], expected["residual"]);
    assert_eq!(
        r["segment"]["frame_post"],
        json!({"site":expected["site"],"captures":expected["captures"]})
    );
    segment(&r["segment"], &expected["segment"]);
}
fn complete(r: &Value, expected: &Value) {
    assert_eq!(r["status"], "complete");
    assert_eq!(
        r["attempted"],
        expected["attempted"].as_u64().unwrap().to_string()
    );
    assert_eq!(
        r["committed"],
        expected["committed"].as_u64().unwrap().to_string()
    );
    assert_eq!(r["residual"], expected["residual"]);
    assert_eq!(r["segment"]["frame_post"], json!({"site":0,"captures":[]}));
    segment(&r["segment"], &expected["segment"]);
}
#[test]
fn delivery_false_observation_preserves_boundary_then_completes_at_eleven() {
    let o = oracle();
    let e = &o["escrow"];
    let f = Files::new(DELIVERY);
    let started = accepted(&f.start("deliver", &e["opening"], Some("11")));
    waiting(&started, &e["prefix_waiting"]);
    assert_eq!(started["continuation"]["logical"]["cap"], "11");
    let c = &started["continuation"];
    let observed = accepted(&f.resume(c, &e["false_resume"]["event"], None));
    assert_eq!(observed["status"], "waiting");
    assert_eq!(observed["advanced"], false);
    assert_eq!(observed["segment"], Value::Null);
    assert_eq!(observed["attempted"], "2");
    assert_eq!(observed["continuation"], *c);
    assert_eq!(observed["residual"], started["residual"]);
    let done = accepted(&f.resume(c, &e["true_resume_complete"]["event"], Some("deliver")));
    complete(&done, &e["true_resume_complete"]);
    assert_eq!(done["residual"], json!([]));
    assert_eq!(
        done["segment"]["frame_post"],
        json!({"site":0,"captures":[]})
    );
}
#[test]
fn reserve_preserves_let_and_event_captures_and_finishes_with_residual_five() {
    let o = oracle();
    let e = &o["reserve"];
    let f = Files::new(RESERVE);
    let started = accepted(&f.start("allocate", &e["opening"], Some("36")));
    waiting(&started, &e["prefix_waiting"]);
    let c1 = &started["continuation"];
    let observed = accepted(&f.resume(c1, &e["first_false_resume"]["event"], None));
    assert_eq!(observed["attempted"], "4");
    assert_eq!(observed["continuation"], *c1);
    assert_eq!(observed["segment"], Value::Null);
    let next = accepted(&f.resume(c1, &e["first_true_second_boundary"]["event"], None));
    waiting(&next, &e["first_true_second_boundary"]);
    let c2 = &next["continuation"];
    let observed = accepted(&f.resume(c2, &e["second_false_resume"]["event"], None));
    assert_eq!(observed["attempted"], "2");
    assert_eq!(observed["continuation"], *c2);
    let done = accepted(&f.resume(c2, &e["second_true_complete"]["event"], None));
    complete(&done, &e["second_true_complete"]);
    let residual = done["residual"].as_array().unwrap();
    assert_eq!(residual.len(), 1);
    assert_eq!(residual[0]["name"], "allocation");
    assert_eq!(residual[0]["amount"], "5");
}
#[test]
fn ordinary_reserve_business_predicate_refusal_publishes_no_suffix() {
    let o = oracle();
    let e = &o["reserve"];
    let refusal = &e["normal_business_suffix_rejection"];
    let f = Files::new(RESERVE);
    let started = accepted(&f.start("allocate", &e["opening"], Some("36")));
    let next = accepted(&f.resume(
        &started["continuation"],
        &refusal["first_accepted_event"],
        None,
    ));
    let prior = next["continuation"].clone();
    let output = f.resume(&prior, &refusal["resume_event"], None);
    assert_eq!(output.status.code(), Some(1));
    let rejected = report(&output);
    assert_eq!(rejected["status"], "rejected");
    assert_eq!(rejected["attempted"], "6");
    assert_eq!(rejected["committed"], "26");
    assert_eq!(rejected["error"]["code"], refusal["error_code"]);
    assert!(rejected.get("segment").is_none());
    assert!(rejected.get("continuation").is_none());
    assert_eq!(
        serde_json::from_slice::<Value>(&std::fs::read(f.path("continuation.json")).unwrap())
            .unwrap(),
        prior
    );
}
#[test]
fn check_and_source_inspect_route_core_two_without_legacy_consumers() {
    let f = Files::new(RESERVE);
    let output = f.command("check").arg("--json").output().unwrap();
    let checked = accepted(&output);
    assert_eq!(checked["language"], "moriarty/2");
    let inspected = accepted(&f.command("inspect").output().unwrap());
    assert_eq!(inspected["semantics"], "moriarty-core/2");
    assert_eq!(
        inspected["stage_plan"]["actions"][0]["segment_cost"],
        json!(["11", "15", "10"])
    );
}
#[cfg(feature = "native")]
#[test]
fn stage_compile_publishes_and_reopens_actual_ordered_segment_artifacts() {
    for (source, action, costs) in [
        (DELIVERY, "deliver", vec!["5", "6"]),
        (RESERVE, "allocate", vec!["11", "15", "10"]),
    ] {
        let f = Files::new(source);
        f.write(
            "target.json",
            serde_json::to_vec(&mori_backend::Target::ledger9("local", "continuations")).unwrap(),
        );
        let output = f
            .command("compile")
            .arg("--target")
            .arg(f.path("target.json"))
            .arg("--out")
            .arg(f.path("out"))
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let bytes = std::fs::read(f.path("out/artifact.json")).unwrap();
        let artifact: mori_backend::stage::StagedArtifact = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(artifact.format, "moriarty-zkir-artifact/4");
        assert_eq!(artifact.circuits.len(), costs.len());
        assert_eq!(serde_json::to_vec_pretty(&artifact).unwrap(), bytes);
        assert_eq!(
            std::fs::read_dir(f.path("out")).unwrap().count(),
            costs.len() + 1
        );
        for (j, circuit) in artifact.circuits.iter().enumerate() {
            assert_eq!(circuit.action, action);
            assert_eq!(circuit.segment as usize, j);
            assert_eq!(circuit.entry_site as usize, j);
            assert_eq!(
                circuit.exit_site as usize,
                if j + 1 == costs.len() { 0 } else { j + 1 }
            );
            assert_eq!(
                std::fs::read(f.path(&format!("out/{j:03}-{action}.s{j}.zkir"))).unwrap(),
                circuit.zkir.as_bytes()
            );
            assert!(!circuit.zkir.is_empty());
        }
        let manifest: Value = serde_json::from_slice(&bytes).unwrap();
        let logical = manifest["resources"]["logical"][0]["segments"]
            .as_array()
            .unwrap();
        for (j, cost) in costs.iter().enumerate() {
            assert_eq!(logical[j]["segment_logical"], *cost);
        }
    }
}
#[test]
fn zero_await_core_two_action_completes_without_pending_frame_or_stored_cap() {
    let source = r#"language "moriarty/2"; agreement Immediate { state count: UInt = 0; action run() { next count = count + 1; } }"#;
    let f = Files::new(source);
    let done = accepted(&f.start(
        "run",
        &json!({"state":{},"inputs":{},"balances":[],"liabilities":[]}),
        None,
    ));
    assert_eq!(done["status"], "complete");
    assert_eq!(done["attempted"], "4");
    assert_eq!(done["committed"], "4");
    for field in ["frame_pre", "frame_post"] {
        assert_eq!(done["segment"][field], json!({"site":0,"captures":[]}));
    }
    assert_eq!(
        done["segment"]["execution"]["post"]["state"]["count"],
        json!({"type":"UInt","value":"1"})
    );
    assert_eq!(done["residual"], json!([]));
    let o = oracle();
    let e = &o["escrow"];
    let f = Files::new(DELIVERY);
    let waiting = accepted(&f.start("deliver", &e["opening"], None));
    assert_eq!(waiting["continuation"]["logical"]["cap"], Value::Null);
    assert!(waiting["continuation"]["logical"].get("cap").is_some());
}
#[test]
fn core_one_keeps_atomic_behavior_and_existing_stdout_envelope() {
    let o = oracle();
    let e = &o["compatibility_v1"];
    let f = Files::new(LEGACY);
    let rejected = f.start("deliver", &e["false_scenario"], Some("10"));
    assert_eq!(rejected.status.code(), Some(1));
    let r = report(&rejected);
    assert_eq!(r["logical"]["used"], "6");
    assert_eq!(r["error"]["step"], 2);
    assert_eq!(r["error"]["code"], e["false_expected"]["code"]);
    let ordinary = f.start("deliver", &e["true_scenario"], None);
    assert!(ordinary.status.success());
    let r = report(&ordinary);
    assert_eq!(r.as_object().unwrap().len(), 3);
    assert_eq!(r["qualification"], "local-reference-execution");
    let expected = &e["true_expected"];
    for key in ["pre", "post"] {
        assert_eq!(r["execution"][key], expected[key]);
    }
    assert_eq!(r["execution"]["effects"], expected["ordered_effects"]);
    assert_eq!(
        ordinary.stdout,
        f.start("deliver", &e["true_scenario"], None).stdout
    );
}
// H16/H19 profile refusals. Expected identities are the frozen typed errors,
// written out here rather than read from the callee.
const STAGE_LABELS: [(&str, &str); 3] = [
    ("qualification", "local-reference-stage"),
    ("authentication", "none"),
    ("consumption", "untracked"),
];
/// Rendered stderr without the diagnostic gutter glyphs or the 80-column wrap.
#[cfg(feature = "native")]
fn diagnostic(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr)
        .split_whitespace()
        .filter(|w| !matches!(*w, "×" | "│"))
        .collect::<Vec<_>>()
        .join(" ")
}
#[test]
fn h16_core_one_resume_is_labelled_profile_rejection_before_continuation_or_event_read() {
    let f = Files::new(LEGACY);
    let (continuation, event) = (
        f.path("absent-continuation.json"),
        f.path("absent-event.json"),
    );
    let output = f
        .command("simulate")
        .arg("--resume")
        .arg(&continuation)
        .arg("--event")
        .arg(&event)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(
        output.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let r = report(&output);
    let keys: std::collections::BTreeSet<&str> =
        r.as_object().unwrap().keys().map(String::as_str).collect();
    assert_eq!(
        keys,
        [
            "attempted",
            "authentication",
            "committed",
            "consumption",
            "error",
            "qualification",
            "status"
        ]
        .into()
    );
    assert_eq!(r["status"], "rejected");
    assert_eq!(r["attempted"], "0");
    assert_eq!(r["committed"], "0");
    assert_eq!(
        r["error"],
        json!({"code":"mori::core::profile","message":"Expected language \"moriarty/2\"; found \"moriarty/1\".","offset":21,"step":null})
    );
    for (key, value) in STAGE_LABELS {
        assert_eq!(r[key], value, "{key}");
    }
    assert!(!continuation.exists() && !event.exists());
}
#[cfg(feature = "native")]
#[test]
fn h16_core_two_trace_target_prints_backend_program_profile_rejection_before_reconstruction() {
    let o = oracle();
    let f = Files::new(DELIVERY);
    f.write(
        "target.json",
        serde_json::to_vec(&mori_backend::Target::ledger9("local", "continuations")).unwrap(),
    );
    f.json("scenario.json", &o["escrow"]["opening"]);
    let output = f
        .command("simulate")
        .args(["--action", "deliver", "--scenario"])
        .arg(f.path("scenario.json"))
        .arg("--trace")
        .arg("--target")
        .arg(f.path("target.json"))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(
        output.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        report(&output),
        json!({
            "status":"rejected", "qualification":"local-reference-replay",
            "error":{"code":"mori::backend::program","message":"mori::core::profile: Expected language \"moriarty/1\"; found \"moriarty/2\"."}
        })
    );
    assert_eq!(std::fs::read_dir(&f.0).unwrap().count(), 3);
}
#[cfg(feature = "native")]
#[test]
fn h16_core_two_inspect_target_refuses_with_core_profile_before_target_read() {
    let f = Files::new(DELIVERY);
    let target = f.path("absent-target.json");
    let output = f
        .command("inspect")
        .arg("--target")
        .arg(&target)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert_eq!(
        diagnostic(&output),
        "mori::command: mori::core::profile: Unsupported language or semantics identity."
    );
    assert!(!target.exists());
}
#[cfg(feature = "native")]
#[test]
fn h19_core_two_compile_call_refuses_before_deployment_read_or_emission() {
    let f = Files::new(DELIVERY);
    let (deployment, out) = (f.path("absent-deployment.json"), f.path("out"));
    let output = f
        .command("compile-call")
        .args(["--action", "deliver", "--deployment"])
        .arg(&deployment)
        .arg("--out")
        .arg(&out)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert_eq!(
        diagnostic(&output),
        "mori::command: mori::core::profile: Expected language \"moriarty/1\"; found \"moriarty/2\"."
    );
    assert!(!out.exists() && !deployment.exists());
}
// H18: byte-identical copy of the public `target_canonical_utf8` in
// examples/core/native-owner-intent/golden-vector.json (the backend
// `signed_target()` fixture); only the Target data, no key material.
#[cfg(feature = "native")]
const SIGNED_TARGET: &str = r#"{"anchor":"native-liability-qa","ledger_revision":"04c9c5d9bcebb8d4427d8589fb54d58a55599c14","max_artifact_bytes":16777216,"max_instructions":100000,"network":"local","profile":"ledger-9","signed_policy":{"beneficiary":"bob","owner":"alice","program_address":"local/DutySettlement"},"zkir_version":"3.0.0-rc.2"}"#;
#[cfg(feature = "native")]
#[test]
fn h18_core_two_compile_signed_target_is_unsupported_before_output_directory() {
    assert!(
        mori_backend::Target::from_json(SIGNED_TARGET)
            .unwrap()
            .signed_policy
            .is_some()
    );
    let f = Files::new(DELIVERY);
    f.write("target.json", SIGNED_TARGET);
    let out = f.path("out");
    let output = f
        .command("compile")
        .arg("--target")
        .arg(f.path("target.json"))
        .arg("--out")
        .arg(&out)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert_eq!(
        diagnostic(&output),
        "mori::command: mori::backend::unsupported: Staged moriarty/2 relations are unsigned; a signed Target is refused."
    );
    assert!(!out.exists());
    assert_eq!(std::fs::read_dir(&f.0).unwrap().count(), 2);
}
