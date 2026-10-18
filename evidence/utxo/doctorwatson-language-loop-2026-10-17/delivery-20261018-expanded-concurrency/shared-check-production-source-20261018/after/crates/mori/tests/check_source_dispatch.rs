//! Complete public `mori check` output and exits for the shared dispatch paths.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Case {
    id: String,
    source: String,
    file_label: String,
    argv: Vec<String>,
    expected_exit: i32,
    expected_stdout: String,
    human_argv: Vec<String>,
}

struct CaseDirectory(PathBuf);

impl CaseDirectory {
    fn new(id: &str) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("current time is after the Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "mori-check-{id}-{}-{nonce}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).expect("create isolated check directory");
        Self(path)
    }
}

impl Drop for CaseDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn check_case(id: &str) {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures/check-source");
    let cases: Vec<Case> = serde_json::from_slice(
        &std::fs::read(fixtures.join("cases.json")).expect("read case manifest"),
    )
    .expect("decode case manifest");
    let case = cases.iter().find(|case| case.id == id).expect("named case");
    let source = std::fs::read(fixtures.join(&case.source)).expect("read source fixture");
    let directory = CaseDirectory::new(id);
    std::fs::write(directory.0.join(&case.file_label), &source).expect("copy source bytes");
    let expected = std::fs::read(fixtures.join(&case.expected_stdout)).expect("read byte oracle");
    let output = Command::new(env!("CARGO_BIN_EXE_mori"))
        .current_dir(&directory.0)
        .args(&case.argv)
        .output()
        .expect("run public JSON check");
    let description = format!("case {id} source:\n{}", String::from_utf8_lossy(&source));
    assert_eq!(output.status.code(), Some(case.expected_exit), "{description}");
    assert_eq!(output.stdout, expected, "{description}");
    assert!(output.stderr.is_empty(), "{description}: {:?}", output.stderr);

    let human = Command::new(env!("CARGO_BIN_EXE_mori"))
        .current_dir(&directory.0)
        .args(&case.human_argv)
        .output()
        .expect("run public human check");
    assert_eq!(human.status.code(), Some(case.expected_exit), "{description}");
    assert!(human.stdout.is_empty(), "{description}");
    assert_eq!(human.stderr.is_empty(), case.expected_exit == 0, "{description}");
}

#[test]
fn beta_accepted() {
    check_case("accepted");
}

#[test]
fn beta_rejected() {
    check_case("rejected");
}

#[test]
fn beta_parse_error() {
    check_case("unparsable");
}

#[test]
fn core1_accepted() {
    check_case("core1-accepted");
}

#[test]
fn core1_rejected() {
    check_case("core1-rejected");
}

#[test]
fn core2_accepted() {
    check_case("core2-accepted");
}

#[test]
fn core2_rejected() {
    check_case("core2-rejected");
}

#[test]
fn beta_unicode_byte_offsets() {
    check_case("unicode-byte-offsets");
}
