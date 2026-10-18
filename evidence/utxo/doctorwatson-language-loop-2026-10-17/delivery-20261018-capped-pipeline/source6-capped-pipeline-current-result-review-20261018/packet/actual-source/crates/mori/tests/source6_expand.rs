#![cfg(feature = "source6-expansion")]

use serde_json::Value;
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};

macro_rules! fixture {
    ($name:literal) => {
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/source6-expansion/",
            $name
        ))
        .as_slice()
    };
}

static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Files(PathBuf);

impl Files {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "mori-source6-expand-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed),
        ));
        fs::create_dir(&root).unwrap();
        fs::write(root.join("invoice.mori"), fixture!("invoice.mori")).unwrap();
        fs::write(
            root.join("scenario.json"),
            fixture!("invoice.scenario.json"),
        )
        .unwrap();
        fs::write(root.join("core.mori"), fixture!("core.mori")).unwrap();
        fs::write(root.join("core.json"), fixture!("core.scenario.json")).unwrap();
        fs::write(root.join("repayment.mori"), fixture!("repayment.mori")).unwrap();
        fs::write(root.join("oracle.mori"), fixture!("oracle.mori")).unwrap();
        Self(root)
    }

    fn invoke(&self, arguments: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_mori"))
            .current_dir(&self.0)
            .args(["--color", "never"])
            .args(arguments)
            .output()
            .unwrap()
    }
}

impl Drop for Files {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn source6_arguments() -> [&'static str; 7] {
    [
        "expand",
        "invoice.mori",
        "--action",
        "pay",
        "--scenario",
        "scenario.json",
        "--emit-source6-transfer",
    ]
}

#[test]
fn checked_beta_reaches_real_expansion_library_and_raw_stdout() {
    let files = Files::new();
    let check = files.invoke(&["check", "invoice.mori", "--json"]);
    assert_eq!(check.status.code(), Some(0));
    assert!(check.stderr.is_empty());
    let report: Value = serde_json::from_slice(&check.stdout).unwrap();
    assert_eq!(report["status"], "accepted");
    assert_eq!(report["actions"][0]["name"], "pay");
    assert_eq!(report["actions"][0]["support"], "local_s0");

    let expansion = files.invoke(&source6_arguments());
    assert_eq!(expansion.status.code(), Some(0));
    assert_eq!(expansion.stdout.as_slice(), fixture!("invoice.source6"));
    assert_eq!(expansion.stderr.as_slice(), fixture!("source6.stderr.txt"));
}

#[test]
fn explicit_source6_quiet_output_is_raw_source6_text() {
    let files = Files::new();
    let mut arguments = vec!["--quiet"];
    arguments.extend(source6_arguments());
    let expansion = files.invoke(&arguments);
    assert_eq!(expansion.status.code(), Some(0));
    assert_eq!(expansion.stdout.as_slice(), fixture!("invoice.source6"));
    assert!(expansion.stderr.is_empty());
}

#[test]
fn core_default_retains_complete_program_bytes() {
    let files = Files::new();
    let output = files.invoke(&[
        "expand",
        "core.mori",
        "--action",
        "noop",
        "--scenario",
        "core.json",
    ]);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stdout.as_slice(), fixture!("core.stdout.json"));
    assert!(output.stderr.is_empty());
}

#[test]
fn beta_default_retains_core_parser_refusal_boundary() {
    let files = Files::new();
    let output = files.invoke(&[
        "expand",
        "invoice.mori",
        "--action",
        "pay",
        "--scenario",
        "scenario.json",
    ]);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("mori::core::syntax"));
}

#[test]
fn source_action_selection_precedes_scenario_file_read() {
    let files = Files::new();
    for (source, action, expected) in [
        (
            "invoice.mori",
            "absent_action",
            "mori::source6::unknown_action",
        ),
        (
            "repayment.mori",
            "repay_loan",
            "mori::source6::unsupported_operation",
        ),
        (
            "oracle.mori",
            "read_fixing",
            "mori::source6::unsupported_operation",
        ),
    ] {
        let output = files.invoke(&[
            "expand",
            source,
            "--action",
            action,
            "--scenario",
            "absent.json",
            "--emit-source6-transfer",
        ]);
        assert_eq!(output.status.code(), Some(1));
        assert!(output.stdout.is_empty());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains(expected), "{stderr}");
        assert!(!stderr.contains("could not read"));
    }
}

#[test]
fn scenario_profile_and_subset_have_explicit_consumer_diagnostics() {
    let files = Files::new();
    let base: Value = serde_json::from_slice(fixture!("invoice.scenario.json")).unwrap();
    let foreign: Value = serde_json::from_slice(fixture!("core.scenario.json")).unwrap();
    let mut with_override = base.clone();
    with_override["candidate_effects"] = serde_json::json!([]);
    let mut with_obligation = base.clone();
    with_obligation["obligation"] = serde_json::json!({});
    let mut missing_cell = base;
    missing_cell["balances"].as_array_mut().unwrap().pop();
    for (scenario, expected) in [
        (foreign, "mori::command::source6-scenario-profile"),
        (with_override, "mori::command::source6-scenario-unsupported"),
        (
            with_obligation,
            "mori::command::source6-scenario-unsupported",
        ),
        (missing_cell, "mori::command::source6-scenario-cells"),
    ] {
        fs::write(
            files.0.join("scenario.json"),
            serde_json::to_vec(&scenario).unwrap(),
        )
        .unwrap();
        let output = files.invoke(&source6_arguments());
        assert_eq!(output.status.code(), Some(1));
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8_lossy(&output.stderr).contains(expected));
    }
}

#[test]
fn diagnostic_presentation_escapes_filename_action_and_scenario_shape_errors() {
    let files = Files::new();
    let hostile_file = "invoice\u{1b}\u{202e}.mori";
    fs::write(files.0.join(hostile_file), fixture!("invoice.mori")).unwrap();

    let unknown = files.invoke(&[
        "expand",
        hostile_file,
        "--action",
        "pay\n\u{1b}\u{202e}",
        "--scenario",
        "absent.json",
        "--emit-source6-transfer",
    ]);
    assert_eq!(unknown.status.code(), Some(1));
    assert!(unknown.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&unknown.stderr);
    assert!(stderr.contains("mori::source6::unknown_action"), "{stderr}");
    assert!(stderr.contains(r"invoice\u{1b}\u{202e}.mori"), "{stderr}");
    assert!(
        stderr.contains(r"No checked action named `pay\n\u{1b}\u{202e}` was found."),
        "{stderr}"
    );
    assert!(!stderr.contains(['\u{1b}', '\u{202e}']), "{stderr}");

    let mut bad_serde: Value = serde_json::from_slice(fixture!("invoice.scenario.json")).unwrap();
    bad_serde["replay"] = serde_json::json!("bad\u{1b}\u{202e}");
    fs::write(
        files.0.join("scenario.json"),
        serde_json::to_vec(&bad_serde).unwrap(),
    )
    .unwrap();
    let shape = files.invoke(&source6_arguments());
    assert_eq!(shape.status.code(), Some(1));
    assert!(shape.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&shape.stderr);
    assert!(
        stderr.contains("mori::command::source6-scenario-shape"),
        "{stderr}"
    );
    assert!(stderr.contains(r"bad\u{1b}\u{202e}"), "{stderr}");
    assert!(!stderr.contains(['\u{1b}', '\u{202e}']), "{stderr}");

    let mut bad_domain: Value = serde_json::from_slice(fixture!("invoice.scenario.json")).unwrap();
    bad_domain["domain"] = serde_json::json!("Other");
    fs::write(
        files.0.join("scenario.json"),
        serde_json::to_vec(&bad_domain).unwrap(),
    )
    .unwrap();
    let mismatch = files.invoke(&[
        "expand",
        hostile_file,
        "--action",
        "pay",
        "--scenario",
        "scenario.json",
        "--emit-source6-transfer",
    ]);
    assert_eq!(mismatch.status.code(), Some(1));
    assert!(mismatch.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&mismatch.stderr);
    assert!(stderr.contains("mori::source6::scenario_shape"), "{stderr}");
    assert!(stderr.contains(r"invoice\u{1b}\u{202e}.mori"), "{stderr}");
    assert!(!stderr.contains(['\u{1b}', '\u{202e}']), "{stderr}");
}

// Public consumer coverage for already admitted nominal arithmetic. Expected
// bytes are hand-derived fixtures, not output from the checker or expander.
#[test]
fn quantity_min_selects_both_operands_through_public_source6_expansion() {
    let files = Files::new();
    fs::write(
        files.0.join("capped.mori"),
        fixture!("capped-remittance.mori"),
    )
    .unwrap();
    fs::write(
        files.0.join("capped.json"),
        fixture!("capped-remittance.scenario.json"),
    )
    .unwrap();

    let checked = files.invoke(&["check", "capped.mori", "--json"]);
    assert_eq!(checked.status.code(), Some(0));
    assert!(checked.stderr.is_empty());
    let report: Value = serde_json::from_slice(&checked.stdout).unwrap();
    assert_eq!(report["status"], "accepted");
    let actions = report["actions"].as_array().unwrap();
    assert_eq!(actions.len(), 2);
    for (action, name) in actions.iter().zip(["pay_tight", "pay_loose"]) {
        assert_eq!(action["name"], name);
        assert_eq!(action["support"], "local_s0");
    }

    // 75 * 3 = 225 atoms. The tight minimum chooses b=210; the loose minimum
    // chooses a=225. The atoms(...,0) fee retains its input cell but adds no
    // fee Credit proposal. Both are complete Source/6 text comparisons.
    for (action, expected) in [
        ("pay_tight", fixture!("capped-tight.source6")),
        ("pay_loose", fixture!("capped-loose.source6")),
    ] {
        let output = files.invoke(&[
            "--quiet",
            "expand",
            "capped.mori",
            "--action",
            action,
            "--scenario",
            "capped.json",
            "--emit-source6-transfer",
        ]);
        assert_eq!(output.status.code(), Some(0));
        assert!(output.stderr.is_empty());
        assert_eq!(output.stdout.as_slice(), expected);
    }
}
