#![cfg(feature = "core2-disjoint-draft")]
use serde_json::Value;
use std::{fs, path::PathBuf, process::Command, sync::atomic::{AtomicUsize, Ordering}};

static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Project(PathBuf);
impl Project {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mori-disjoint-test-public-{}-{}", std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed),
        ));
        fs::create_dir(&path).unwrap();
        let fixtures = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../fixtures/core-disjoint-product");
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
    fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); }
}

#[test]
fn public_consumer_observes_both_disjoint_domains_for_two_explicit_schedules() {
    let project = Project::new();
    // The aggregate and all ten stage reports are independent literal inputs.
    let expected = fs::read(project.0.join("public-report.json")).unwrap();
    let expected_json: Value = serde_json::from_slice(&expected).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_mori"))
        .args(["--color", "never", "test"])
        .arg(&project.0).output().unwrap();
    assert_eq!(output.status.code(), Some(0), "{}", String::from_utf8_lossy(&output.stderr));
    let actual: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(actual, expected_json);
    assert_eq!(output.stdout, expected);
    assert!(String::from_utf8_lossy(&output.stderr).contains("2 passed; 0 failed"));
    // Equality here supplements the independently literal complete reports.
    // It does not replace their separate ordered histories or member effects.
    assert_eq!(
        actual["cases"][0]["stages"][4]["actual"]["post"],
        actual["cases"][1]["stages"][4]["actual"]["post"],
    );
}
