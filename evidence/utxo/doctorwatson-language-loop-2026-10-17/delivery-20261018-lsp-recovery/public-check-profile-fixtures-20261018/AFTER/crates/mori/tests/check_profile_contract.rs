//! Complete normal public check reports for beta, Core/1 and staged Core/2.
use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures/public-check-profile")
}

struct Project(PathBuf);

impl Project {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "mori-check-profile-contract-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        Self(root)
    }

    fn check(&self, stem: &str) {
        let source_name = format!("{stem}.mori");
        let expected_name = format!("{stem}.check.expected.json");
        fs::copy(fixtures().join(&source_name), self.0.join(&source_name)).unwrap();
        let expected: Value = serde_json::from_slice(
            &fs::read(fixtures().join(expected_name)).unwrap(),
        )
        .unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_mori"))
            .current_dir(&self.0)
            .args(["--color", "never", "check", &source_name, "--json"])
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(0),
            "{stem}: stdout={} stderr={}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(output.stderr.is_empty(), "{stem}: JSON check writes no status");
        let actual: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(actual, expected, "{stem}: complete independently declared report");
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn supported_profiles_keep_complete_normal_financial_check_reports() {
    let project = Project::new();
    for stem in [
        "beta-invoice",
        "beta-repayment",
        "core1-payment",
        "core1-loan",
        "core2-escrow",
        "core2-rebate",
    ] {
        project.check(stem);
    }
}
