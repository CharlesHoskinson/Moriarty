//! Complete public checking and Program shape for Core/2 duty-read expressions.
use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};

const HUMAN_STDERR: &str = "    Checking duty.mori\n    Finished bounded core check\n";

fn fixture(name: &str) -> Vec<u8> {
    fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../fixtures/core-duty-read-check")
            .join(name),
    )
    .expect("read frozen duty-read fixture")
}

struct Project {
    root: PathBuf,
    temporary: bool,
}

impl Project {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let (root, temporary) = match std::env::var_os("MORI_DUTY_READ_CHECK_OBSERVATIONS") {
            Some(path) => {
                let path = PathBuf::from(path);
                assert!(path.is_absolute(), "use an absolute capture directory");
                (path, false)
            }
            None => (
                std::env::temp_dir().join(format!(
                    "mori-duty-read-check-{}-{}",
                    std::process::id(),
                    NEXT.fetch_add(1, Ordering::Relaxed)
                )),
                true,
            ),
        };
        fs::create_dir(&root).expect("use a fresh capture directory");
        Self { root, temporary }
    }

    fn run(&self, id: &str, mode: &str, source: &[u8]) -> Output {
        let dir = self.root.join(format!("{id}-{mode}"));
        fs::create_dir(&dir).unwrap();
        fs::write(dir.join("duty.mori"), source).unwrap();
        let mut args = vec!["--color", "never", "check", "duty.mori"];
        if mode == "json" {
            args.push("--json");
        }
        let captured = Command::new(env!("CARGO_BIN_EXE_mori"))
            .current_dir(&dir)
            .args(&args)
            .output();
        if let Err(error) = &captured {
            fs::write(
                dir.join("process.json"),
                serde_json::to_vec_pretty(&serde_json::json!({
                    "executable": env!("CARGO_BIN_EXE_mori"),
                    "args": args, "launchError": error.to_string(),
                }))
                .unwrap(),
            )
            .unwrap();
        }
        let output = captured.expect("run actual public check");
        // Retain complete actual process bytes and exit before assertions.
        fs::write(dir.join("stdout.bin"), &output.stdout).unwrap();
        fs::write(dir.join("stderr.bin"), &output.stderr).unwrap();
        fs::write(
            dir.join("process.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "executable": env!("CARGO_BIN_EXE_mori"),
                "args": args, "status": output.status.code(),
            }))
            .unwrap(),
        )
        .unwrap();
        output
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        if self.temporary && !std::thread::panicking() {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
}

#[test]
fn documented_duty_reads_reach_public_core2_check() {
    let project = Project::new();
    let source = fixture("duty-observer.mori");
    // Keep both actual public processes before any product assertion.
    let json = project.run("duty", "json", &source);
    let human = project.run("duty", "human", &source);
    let text = std::str::from_utf8(&source).expect("UTF-8 source fixture");
    let program = mori_core::stage::load(text).map(|program| program.program().clone());
    fs::write(
        project.root.join("program-result.actual.json"),
        serde_json::to_vec_pretty(&program).unwrap(),
    )
    .unwrap();
    let description = format!("complete source:\n{text}");
    let actual = serde_json::to_value(program.expect("admit documented duty-read source")).unwrap();
    let expected: Value = serde_json::from_slice(&fixture("duty-observer.program.expected.json"))
        .expect("decode frozen complete Program");
    assert_eq!(actual, expected, "{description}: complete Program");
    assert_eq!(json.status.code(), Some(0), "{description}");
    assert_eq!(
        json.stdout,
        fixture("duty-observer.check.stdout.expected.json"),
        "{description}: complete check stdout"
    );
    assert!(json.stderr.is_empty(), "{description}: {:?}", json.stderr);
    assert_eq!(human.status.code(), Some(0), "{description}");
    assert!(human.stdout.is_empty(), "{description}");
    assert_eq!(
        human.stderr,
        HUMAN_STDERR.as_bytes(),
        "{description}: complete human stream"
    );
}
