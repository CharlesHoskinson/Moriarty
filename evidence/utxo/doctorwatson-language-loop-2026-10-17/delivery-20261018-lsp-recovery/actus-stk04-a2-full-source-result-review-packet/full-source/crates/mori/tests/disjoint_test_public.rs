#![cfg(feature = "core2-disjoint-draft")]
use serde_json::Value;
use std::{
    fs,
    path::PathBuf,
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Project(PathBuf);
impl Project {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mori-disjoint-test-public-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed),
        ));
        fs::create_dir(&path).unwrap();
        let fixtures =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../fixtures/core-disjoint-product");
        for entry in fs::read_dir(fixtures).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() {
                fs::copy(entry.path(), path.join(entry.file_name())).unwrap();
            }
        }
        Self(path)
    }
}
impl Drop for Project {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn expand(actual: &Value, tables: &Value) -> Value {
    let mut actual = actual.clone();
    assert_eq!(
        actual["format"],
        "moriarty-local-disjoint-referenced-draft/1"
    );
    for name in ["history", "effects"] {
        assert_eq!(actual[name]["table"], name);
        let length = usize::try_from(actual[name]["prefix_len"].as_u64().unwrap()).unwrap();
        let table = tables[name].as_array().unwrap();
        actual[name] = Value::Array(table.get(..length).unwrap().to_vec());
    }
    let format = actual
        .as_object_mut()
        .unwrap()
        .remove("expanded_format")
        .unwrap();
    actual["format"] = format;
    actual
}

#[test]
fn public_consumer_observes_both_disjoint_domains_for_two_explicit_schedules() {
    let project = Project::new();
    // The aggregate and all ten stage reports are independent literal inputs.
    let expected = fs::read(project.0.join("public-report.json")).unwrap();
    let expected_json: Value = serde_json::from_slice(&expected).unwrap();
    let manifest: Value =
        serde_json::from_slice(&fs::read(project.0.join("mori-tests.json")).unwrap()).unwrap();
    let expected_stages: Vec<Vec<Value>> = manifest["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|case| {
            let mut paths = vec![case["expected"].as_str().unwrap()];
            paths.extend(
                case["events"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|event| event["expected"].as_str().unwrap()),
            );
            paths
                .into_iter()
                .map(|path| {
                    serde_json::from_slice(&fs::read(project.0.join(path)).unwrap()).unwrap()
                })
                .collect()
        })
        .collect();
    let output = Command::new(env!("CARGO_BIN_EXE_mori"))
        .args(["--color", "never", "test"])
        .arg(&project.0)
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let actual: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(actual, expected_json);
    assert_eq!(output.stdout, expected);
    assert!(String::from_utf8_lossy(&output.stderr).contains("2 passed; 0 failed"));
    // Expand each explicit prefix reference and compare the original whole
    // independent report. Every frame, duty, original envelope and effect stays
    // observable; the shared tables do not normalize or truncate expectations.
    for (case, expected_stages) in actual["cases"]
        .as_array()
        .unwrap()
        .iter()
        .zip(expected_stages)
    {
        assert_eq!(
            case["format"],
            "moriarty-disjoint-case-observations-draft/1"
        );
        assert_eq!(
            case["stages"].as_array().unwrap().len(),
            expected_stages.len()
        );
        for (stage, expected_stage) in case["stages"]
            .as_array()
            .unwrap()
            .iter()
            .zip(expected_stages)
        {
            assert_eq!(expand(&stage["actual"], &case["tables"]), expected_stage);
        }
    }
    // Equality here supplements the independently literal complete reports.
    // It does not replace their separate ordered histories or member effects.
    assert_eq!(
        actual["cases"][0]["stages"][4]["actual"]["post"],
        actual["cases"][1]["stages"][4]["actual"]["post"],
    );
}
