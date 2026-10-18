//! Full positive Core/1 financial Programs and public CLI reports.
use serde_json::Value;
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures/financial-report-contract")
}

fn json_fixture(root: &Path, name: &str) -> Value {
    serde_json::from_slice(&fs::read(root.join(name)).unwrap()).unwrap()
}

static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Project {
    root: PathBuf,
}

impl Project {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "mori-financial-report-contract-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        Self { root }
    }

    fn assert_case(&self, fixtures: &Path, case: &Value) -> Value {
        let name = case["name"].as_str().unwrap();
        for key in ["source", "scenario"] {
            let file = case[key].as_str().unwrap();
            fs::copy(fixtures.join(file), self.root.join(file)).unwrap();
        }
        let argv = case["argv_template"]
            .as_array()
            .unwrap()
            .iter()
            .map(|arg| arg.as_str().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(argv[0], "mori");
        let output = Command::new(env!("CARGO_BIN_EXE_mori"))
            .current_dir(&self.root)
            .args(&argv[1..])
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(0),
            "{name}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stderr = case["stderr"].as_str().unwrap();
        let expected_stderr = fs::read(fixtures.join(stderr)).unwrap();
        assert_eq!(output.stderr, expected_stderr, "{name}: exact empty stderr");
        let stdout = case["stdout"].as_str().unwrap();
        let expected_stdout = fs::read(fixtures.join(stdout)).unwrap();
        let actual: Value = serde_json::from_slice(&output.stdout).unwrap();
        let expected: Value = serde_json::from_slice(&expected_stdout).unwrap();
        assert_eq!(actual, expected, "{name}: complete JSON report");
        assert_eq!(output.stdout, expected_stdout, "{name}: exact stdout bytes");
        actual
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn financial_sources_match_complete_independent_programs() {
    let root = fixtures();
    for (source, expected) in [
        (
            "installment-invoice.mori",
            "installment-invoice.program.expected.json",
        ),
        (
            "conditional-reserve.mori",
            "conditional-reserve.program.expected.json",
        ),
    ] {
        let source_text = fs::read_to_string(root.join(source)).unwrap();
        let program = mori_core::parse(&source_text).expect("financial source must parse");
        assert_eq!(
            serde_json::to_value(program).unwrap(),
            json_fixture(&root, expected),
            "{source}: complete Program"
        );
    }
}

#[test]
fn financial_cli_reports_match_complete_oracles_and_invoice_link() {
    let root = fixtures();
    let cases = json_fixture(&root, "CASES.json");
    let cases = cases["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 6);
    let project = Project::new();
    let mut reports = BTreeMap::new();
    for case in cases {
        let actual = project.assert_case(&root, case);
        reports.insert(case["name"].as_str().unwrap().to_owned(), actual);
    }
    for (first, final_call) in [
        ("invoice-first-ordinary", "invoice-final-ordinary"),
        ("invoice-first-cap42", "invoice-final-cap42"),
    ] {
        assert_eq!(
            reports[first]["execution"]["post"], reports[final_call]["execution"]["pre"],
            "{first} post must equal {final_call} pre in full"
        );
    }
}
