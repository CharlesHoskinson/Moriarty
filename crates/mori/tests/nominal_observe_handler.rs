//! Source-derived tests frozen before observation implementation. ALL UNRUN.
//! Real production Args/handler/I/O and full normal external Core; no fake owner.
//! BranchScore values are author predictions, NOT an independent semantic oracle.
#[path = "../src/cli_args/compile.rs"]
pub(crate) mod compile_args;
mod cli_args {
    pub(crate) use crate::compile_args as compile;
}
#[path = "../src/display.rs"]
mod display;
#[path = "../src/commands/publish_io.rs"]
mod publish_io;
#[path = "../src/commands/compile_sherlock.rs"]
mod selected_compile;
#[path = "../src/commands/source_io.rs"]
mod source_io;
use clap::{Args as ClapArgs, FromArgMatches};
use mori_core::sherlock_compile_artifact::nominal_body_command as command;
use serde_json::{Value, json};
use std::{
    collections::BTreeSet,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};

const TYPED: &str = "moriarty-sherlock-nominal-body-typed/1";
const QUALIFICATION: &str =
    "HostOnly nominal body-typed owner execution; no proof/native/target qualification";
const TWO: &str = include_str!("fixtures/nominal-observe/branch-score-two.mori");
const ZERO: &str = include_str!("fixtures/nominal-observe/branch-score-zero.mori");
const JUDGMENTS: &[&str] = &[
    "eq-any",
    "unique-parameters-every-declaration",
    "unique-binders-every-arm",
];
static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mori-nominal-observe-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn config() -> Value {
    serde_json::from_str(include_str!("fixtures/nominal-compile-handler/config.json")).unwrap()
}
fn text(entry: &str) -> String {
    format!("language \"moriarty-sherlock-nominal/1\"; program Observe {{ entry {entry}; }}")
}
struct Outcome {
    artifact: Option<Value>,
    error: Option<String>,
    published: bool,
}
fn invoke(
    source: &str,
    cfg: &Value,
    profile: &str,
    observe: bool,
    readable: bool,
    existing_out: bool,
) -> Outcome {
    let temp = Temp::new();
    let file = temp.0.join("program.mori");
    let conf = temp.0.join("config.json");
    let target = temp.0.join("target.json");
    let out = temp.0.join("out");
    std::fs::write(&file, source).unwrap();
    if readable {
        std::fs::write(&conf, cfg.to_string()).unwrap();
    }
    std::fs::write(&target, "{}").unwrap();
    if existing_out {
        std::fs::create_dir(&out).unwrap();
    }
    let mut argv = vec![
        "compile".to_owned(),
        file.display().to_string(),
        "--target".into(),
        target.display().to_string(),
        "--out".into(),
        out.display().to_string(),
        "--execution-profile".into(),
        "sherlock-helper-microstep/1".into(),
        "--source-profile".into(),
        profile.into(),
        "--entry".into(),
        "--execution-config".into(),
        conf.display().to_string(),
    ];
    if observe {
        argv.push("--observe".into());
    }
    let parsed = compile_args::Args::augment_args(clap::Command::new("compile"))
        .try_get_matches_from(argv)
        .unwrap();
    let args = compile_args::Args::from_arg_matches(&parsed).unwrap();
    let actual_source = source_io::read_source(&args.file).unwrap();
    let mut statuses = Vec::new();
    let result = selected_compile::run(&args, &actual_source, |verb, msg| {
        statuses.push((verb.to_owned(), msg))
    });
    match result {
        Ok(exit) => {
            assert_eq!(exit, std::process::ExitCode::SUCCESS);
            assert_eq!(statuses.len(), 1);
            assert_eq!(statuses[0].0, "Prepared");
            assert_eq!(std::fs::read_dir(&out).unwrap().count(), 1);
            Outcome {
                artifact: Some(
                    serde_json::from_str(
                        &std::fs::read_to_string(out.join("sherlock-artifact.json")).unwrap(),
                    )
                    .unwrap(),
                ),
                error: None,
                published: true,
            }
        }
        Err(error) => {
            assert!(statuses.is_empty());
            if existing_out {
                assert_eq!(std::fs::read_dir(&out).unwrap().count(), 0);
            }
            Outcome {
                artifact: None,
                error: Some(error.to_string()),
                published: out.exists(),
            }
        }
    }
}
fn observed(source: &str, cfg: &Value) -> Value {
    let out = invoke(source, cfg, TYPED, true, true, false);
    assert!(out.error.is_none(), "{:?}", out.error);
    assert!(out.published);
    out.artifact.unwrap()
}
fn keys(value: &Value) -> BTreeSet<&str> {
    value
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect()
}
fn literal_keys<'a>(items: &[&'a str]) -> BTreeSet<&'a str> {
    items.iter().copied().collect()
}
fn assert_report(a: &Value) {
    assert_eq!(
        keys(a),
        literal_keys(&["format", "qualification", "preparation", "execution"])
    );
    assert_eq!(a["format"], "sherlock-nominal-host-observation/1");
    assert_eq!(a["qualification"], QUALIFICATION);
    let p = &a["preparation"];
    assert_eq!(p["format"], "sherlock-nominal-body-typed-host-artifact/1");
    assert_eq!(
        p["body_typechecking"],
        json!({"status":"completed", "judgments":JUDGMENTS})
    );
    assert_eq!(p["evaluation_performed"], false);
    assert_eq!(p["semantic_correspondence"], "unproved");
    assert_eq!(p["plans"]["witness"]["status"], "missing");
    assert_eq!(p["plans"]["constraint"]["status"], "missing");
    assert_eq!(p["plans"]["export"]["status"], "unsupported");
    assert_eq!(p["evidence"]["native"], "unrun");
    assert_eq!(
        keys(&p["admission"]["program_view"]),
        literal_keys(&[
            "entry",
            "expressions",
            "occurrences",
            "occurrence_paths",
            "children",
            "functions",
            "parameters",
            "arms",
            "patterns",
            "pattern_children",
            "types",
            "type_args",
            "constructors",
            "ctor_fields",
            "decl_param_counts",
            "decl_ctor_counts",
            "decl_templates",
            "plan_nodes",
            "plan_cases",
            "field_paths",
            "constants",
            "constant_fields",
            "names"
        ])
    );
    let e = &a["execution"];
    assert_eq!(
        keys(e),
        literal_keys(&[
            "evaluation_performed",
            "completed",
            "summary",
            "events_forwarded",
            "event_capture",
            "image",
            "resource_report"
        ])
    );
    assert_eq!(e["evaluation_performed"], true);
    assert_eq!(e["completed"], true);
    assert_eq!(e["event_capture"], "not-retained");
    assert_eq!(e["events_forwarded"], e["summary"]["events"]);
    assert_eq!(e["summary"]["outcome"], e["image"]["outcome"]);
    assert_eq!(
        keys(&e["image"]),
        literal_keys(&[
            "run_id",
            "control",
            "control_origin",
            "env_root",
            "stack_limit",
            "outcome",
            "initial_fuel",
            "remaining_work",
            "consumed_work",
            "helper_work",
            "helper_visits",
            "reference_cursor",
            "alloc",
            "active",
            "helper",
            "result",
            "data",
            "data_fields",
            "env",
            "frames",
            "tasks",
            "value_slots",
            "seg_base"
        ])
    );
}
fn halted_uint(a: &Value) -> &str {
    let r = &a["execution"]["summary"]["outcome"]["Halted"]["value"];
    let row = if let Some(n) = r.get("Const") {
        &a["preparation"]["admission"]["program_view"]["constants"]
            [n.as_str().unwrap().parse::<usize>().unwrap()]
    } else {
        &a["execution"]["image"]["data"][r["Data"].as_str().unwrap().parse::<usize>().unwrap()]
    };
    row["U128"]["n"].as_str().unwrap()
}

#[test]
fn selected_recursive_generic_branch_score_is_twelve() {
    let a = observed(TWO, &config());
    assert_report(&a);
    assert_eq!(halted_uint(&a), "12");
    assert_eq!(
        a["preparation"]["admission"]["program_view"]["functions"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert!(
        !a["preparation"]["admission"]["program_view"]["plan_nodes"]
            .as_array()
            .unwrap()
            .is_empty()
    );
}
#[test]
fn contrasting_zero_branch_score_is_nine() {
    let a = observed(ZERO, &config());
    assert_report(&a);
    assert_eq!(halted_uint(&a), "9");
}
#[test]
fn complete_nested_preparation_matches_compile_only_projection() {
    let cfg = config();
    let a = observed(TWO, &cfg);
    // Shared-producer equivalence baseline, NOT an independent semantic oracle.
    let out = invoke(TWO, &cfg, TYPED, false, true, false);
    assert!(out.error.is_none());
    let original = out.artifact.unwrap();
    assert_eq!(a["preparation"], original);
    assert_eq!(original["evaluation_performed"], false);
    assert!(original.get("execution").is_none());
}
#[test]
fn real_zero_fuel_is_completed_exhaustion_not_halted() {
    let mut cfg = config();
    cfg["fuel"]["value"] = "0".into();
    let a = observed(&text("0"), &cfg);
    assert_report(&a);
    // Existing owner P1/P2: initial Expr is not publishable, P2 publishes Fuel
    // exhaustion uncharged, one actual event, no charged work/reference cursor.
    assert_eq!(
        a["execution"]["summary"],
        json!({"outcome":{"Exhausted":{"reason":"Fuel"}},"events":"1",
        "consumed_work":"0","helper_work":"0","helper_visits":"0","reference_cursor":"0"})
    );
    assert_eq!(a["execution"]["image"]["remaining_work"], "0");
}
#[test]
fn real_checked_subtraction_error_is_completed_rejection() {
    let a = observed(&text("0 - 1"), &config());
    assert_report(&a);
    assert_eq!(
        a["execution"]["summary"]["outcome"],
        json!({"Rejected":{"code":10}})
    );
}
#[test]
fn all_preexecution_stages_keep_compile_error_payload_and_flags() {
    let ordinary = config();
    let mut parse = config();
    parse["preprocessing"]["max_source_bytes"] = "1".into();
    let mut owner = config();
    owner["bounds"]["step_cap"] = "0".into();
    let mut genesis = config();
    genesis["fuel"]["value"] = "4097".into();
    for (source, cfg, stage) in [
        (text("0"), parse, "parse"),
        (
            text("construct List::Cons(1,nil)"),
            ordinary.clone(),
            "preparation",
        ),
        (text("true + 1"), ordinary, "body-type"),
        (text("0"), owner, "owner-admission"),
        (text("0"), genesis, "genesis"),
    ] {
        let before = invoke(&source, &cfg, TYPED, false, true, false);
        let after = invoke(&source, &cfg, TYPED, true, true, false);
        assert_eq!(after.error, before.error, "stage {stage}");
        assert!(
            after
                .error
                .unwrap()
                .contains(&format!("\"stage\": \"{stage}\""))
        );
        assert!(!after.published);
        assert!(!before.published);
    }
}
#[test]
fn structural_and_draft_observe_still_refuse_before_config_io() {
    for (profile, message) in [
        (
            "moriarty-sherlock-nominal/1",
            "nominal preparation is compile-only",
        ),
        (
            "moriarty-sherlock-draft/1",
            "draft preparation is compile-only",
        ),
    ] {
        let o = invoke(&text("0"), &config(), profile, true, false, false);
        assert!(o.error.unwrap().contains(message));
        assert!(!o.published);
    }
}
#[test]
fn body_typed_observe_now_reads_exact_config_before_source_work() {
    let o = invoke(&text("0"), &config(), TYPED, true, false, false);
    assert!(o.error.unwrap().contains("\"stage\": \"config\""));
    assert!(!o.published);
}
#[test]
fn new_observer_runtime_report_failure_retains_actual_count_and_body_completion() {
    let cfg = config().to_string();
    let report = command::observe(ZERO, &cfg, "{}")
        .unwrap_or_else(|e| panic!("{}", serde_json::to_string(&e).unwrap()));
    let a = serde_json::to_value(&report).unwrap();
    struct RefusingWriter;
    impl std::io::Write for RefusingWriter {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other("retained output unavailable"))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    // A genuine caller serialization I/O failure, not an injected owner failure.
    // CLI to_string_pretty has no known reachable error for the current fields.
    let error = serde_json::to_writer(RefusingWriter, &report).unwrap_err();
    let failure = serde_json::to_value(report.failure("serialization", error.to_string())).unwrap();
    assert_eq!(failure["stage"], "serialization");
    assert_eq!(failure["evaluation_performed"], true);
    assert_eq!(failure["completed"], true);
    assert_eq!(
        failure["events_forwarded"],
        a["execution"]["events_forwarded"]
    );
    assert_eq!(
        keys(&failure["actual_error"]),
        literal_keys(&["message", "source_profile", "body_typechecking"])
    );
    assert_eq!(failure["actual_error"]["source_profile"], TYPED);
    assert_eq!(
        failure["actual_error"]["body_typechecking"],
        json!({"status":"completed", "judgments":JUDGMENTS})
    );
}
#[test]
fn selected_publication_refusal_retains_completed_observation_metadata() {
    let cfg = config();
    let reference = observed(ZERO, &cfg);
    let o = invoke(ZERO, &cfg, TYPED, true, true, true);
    let error = o.error.unwrap();
    assert!(o.published);
    for fragment in [
        "\"stage\": \"publish\"",
        "\"evaluation_performed\": true",
        "\"completed\": true",
        "\"status\": \"completed\"",
    ] {
        assert!(error.contains(fragment), "{error}");
    }
    let count = reference["execution"]["events_forwarded"].as_str().unwrap();
    assert!(error.contains(&format!("\"events_forwarded\": \"{count}\"")));
    assert!(error.contains(TYPED));
}
