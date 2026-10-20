//! Real public manifests compare complete supplied ordinary reports.
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Project(PathBuf);
impl Project {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "mori-core3-manifest-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        let fixture =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures/core3-public-consumers");
        for name in [
            "empty.mori",
            "empty-scenario.json",
            "empty-complete.expected.json",
            "mori-tests.json",
        ] {
            fs::copy(fixture.join(name), root.join(name)).unwrap();
        }
        Self(root)
    }
    fn read(&self, name: &str) -> Value {
        serde_json::from_slice(&fs::read(self.0.join(name)).unwrap()).unwrap()
    }
    fn write(&self, name: &str, value: &Value) {
        fs::write(self.0.join(name), serde_json::to_vec(value).unwrap()).unwrap();
    }
    fn run(&self) -> Output {
        Command::new(env!("CARGO_BIN_EXE_mori"))
            .current_dir(&self.0)
            .env("PATH", "")
            .stdin(Stdio::null())
            .args(["--color", "never", "test", "."])
            .output()
            .unwrap()
    }
}
impl Drop for Project {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn core3_manifest_compares_the_complete_supplied_empty_outcome() {
    let p = Project::new();
    let expected = p.read("empty-complete.expected.json");
    let output = p.run();
    assert_eq!(output.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&output.stderr).contains("1 passed; 0 failed"));
    assert!(output.stdout.ends_with(b"\n"));
    assert_eq!(
        serde_json::from_slice::<Value>(&output.stdout).unwrap(),
        json!({
            "status":"passed", "cases":[{
                "name":"empty-core3", "passed":true, "stages":[{"index":0,"passed":true,"actual":expected}]
            }]
        })
    );
}

#[test]
fn ordinary_expected_status_typo_fails_without_replacing_actual_or_omitting_later_case() {
    let p = Project::new();
    let correct = p.read("empty-complete.expected.json");
    let mut wrong = correct.clone();
    wrong["status"] = json!("waiting");
    p.write("wrong.expected.json", &wrong);
    let mut manifest = p.read("mori-tests.json");
    let mut second = manifest["cases"][0].clone();
    second["name"] = json!("later-correct");
    manifest["cases"][0]["expected"] = json!("wrong.expected.json");
    manifest["cases"].as_array_mut().unwrap().push(second);
    p.write("mori-tests.json", &manifest);
    let output = p.run();
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("1 passed; 1 failed"));
    assert_eq!(
        serde_json::from_slice::<Value>(&output.stdout).unwrap(),
        json!({
            "status":"failed", "cases":[
                {"name":"empty-core3","passed":false,"stages":[{"index":0,"passed":false,"actual":correct}]},
                {"name":"later-correct","passed":true,"stages":[{"index":0,"passed":true,"actual":correct}]}
            ]
        })
    );
}
