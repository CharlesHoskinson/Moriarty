//! Public Core header errors preserve the routing reader's complete diagnostics.

use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

struct CaseDirectory(PathBuf);

impl CaseDirectory {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("current time is after the Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "mori-header-{}-{nonce}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).expect("create isolated header directory");
        Self(path)
    }
}

impl Drop for CaseDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn check_header(source: &str, expected_stdout: &str, expected_stderr: &str) {
    let directory = CaseDirectory::new();
    std::fs::write(directory.0.join("header.mori"), source).expect("write source");
    let json = Command::new(env!("CARGO_BIN_EXE_mori"))
        .current_dir(&directory.0)
        .args(["--color", "never", "check", "header.mori", "--json"])
        .output()
        .expect("run public JSON check");
    assert_eq!(json.status.code(), Some(1), "source:\n{source}");
    assert_eq!(json.stdout, expected_stdout.as_bytes(), "source:\n{source}");
    assert!(
        json.stderr.is_empty(),
        "source:\n{source}: {:?}",
        json.stderr
    );

    let human = Command::new(env!("CARGO_BIN_EXE_mori"))
        .current_dir(&directory.0)
        .args(["--color", "never", "--quiet", "check", "header.mori"])
        .output()
        .expect("run public quiet human check");
    assert_eq!(human.status.code(), Some(1), "source:\n{source}");
    assert!(human.stdout.is_empty(), "source:\n{source}");
    assert_eq!(
        human.stderr,
        expected_stderr.as_bytes(),
        "source:\n{source}"
    );

    let normal = Command::new(env!("CARGO_BIN_EXE_mori"))
        .current_dir(&directory.0)
        .args(["--color", "never", "check", "header.mori"])
        .output()
        .expect("run public human check");
    assert_eq!(normal.status.code(), Some(1), "source:\n{source}");
    assert!(normal.stdout.is_empty(), "source:\n{source}");
    let normal_stderr = format!("    Checking header.mori\n{expected_stderr}");
    assert_eq!(normal.stderr, normal_stderr.as_bytes(), "source:\n{source}");
}

#[test]
fn unknown_profile_reaches_public_check() {
    check_header(
        r##"language "other";"##,
        r##"{
  "diagnostics": [
    {
      "code": "mori::core::profile",
      "message": "Expected language \"moriarty/1\" or \"moriarty/2\"; found \"other\".",
      "offset": 16,
      "step": null
    }
  ],
  "file": "header.mori",
  "status": "rejected"
}
"##,
        r##"error: mori::core::profile: Expected language "moriarty/1" or "moriarty/2"; found "other".
"##,
    );
}

#[test]
fn missing_profile_string_reaches_public_check() {
    check_header(
        r##"language ;"##,
        r##"{
  "diagnostics": [
    {
      "code": "mori::core::syntax",
      "message": "Expected language \"moriarty/1\" or \"moriarty/2\". Expected a quoted string.",
      "offset": 9,
      "step": null
    }
  ],
  "file": "header.mori",
  "status": "rejected"
}
"##,
        r##"error: mori::core::syntax: Expected language "moriarty/1" or "moriarty/2". Expected a quoted string.
"##,
    );
}

#[test]
fn commented_unknown_profile_reaches_public_check() {
    check_header(
        r##"// café
language "other";"##,
        r##"{
  "diagnostics": [
    {
      "code": "mori::core::profile",
      "message": "Expected language \"moriarty/1\" or \"moriarty/2\"; found \"other\".",
      "offset": 25,
      "step": null
    }
  ],
  "file": "header.mori",
  "status": "rejected"
}
"##,
        r##"error: mori::core::profile: Expected language "moriarty/1" or "moriarty/2"; found "other".
"##,
    );
}
