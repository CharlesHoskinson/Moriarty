//! Positive documented Boolean source forms through real public Core checking.
//! Full Program and CLI expectations were manually frozen before execution;
//! the author's implementation-source exposure is explicit in the fixtures.
use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};

const HUMAN_STDERR: &str = "    Checking boolean.mori\n    Finished bounded core check\n";

fn fixture(name: &str) -> Vec<u8> {
    fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../fixtures/core-boolean-check")
            .join(name),
    )
    .expect("read frozen Boolean fixture")
}

struct Project {
    root: PathBuf,
    temporary: bool,
}

impl Project {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let (root, temporary) = match std::env::var_os("MORI_BOOLEAN_CHECK_OBSERVATIONS") {
            Some(path) => {
                let path = PathBuf::from(path);
                assert!(path.is_absolute(), "use an absolute capture directory");
                (path, false)
            }
            None => (
                std::env::temp_dir().join(format!(
                    "mori-boolean-check-{}-{}",
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
        fs::write(dir.join("boolean.mori"), source).unwrap();
        let mut args = vec!["--color", "never", "check", "boolean.mori"];
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
fn documented_boolean_programs_reach_public_core_check() {
    let project = Project::new();
    // All four public CLI calls are captured before any product assertion.
    let outputs: Vec<_> = [("core1", false), ("core2", true)]
        .into_iter()
        .map(|(id, staged)| {
            let source = fixture(&format!("{id}.mori"));
            let json = project.run(id, "json", &source);
            let human = project.run(id, "human", &source);
            (id, staged, source, json, human)
        })
        .collect();
    let captured: Vec<_> = outputs
        .into_iter()
        .map(|(id, staged, source, json, human)| {
            let text = std::str::from_utf8(&source).expect("UTF-8 source fixture");
            // Actual exported consumers, with no evaluator, synthetic AST or lowering.
            let program = if staged {
                mori_core::stage::load(text).map(|program| program.program().clone())
            } else {
                mori_core::parse(text)
            };
            fs::write(
                project
                    .root
                    .join(format!("{id}.program-result.actual.json")),
                serde_json::to_vec_pretty(&program).unwrap(),
            )
            .unwrap();
            (id, source, json, human, program)
        })
        .collect();
    for (id, source, json, human, program) in captured {
        let description = format!(
            "{id} complete source:\n{}",
            String::from_utf8_lossy(&source)
        );
        let actual =
            serde_json::to_value(program.expect("admit frozen documented source")).unwrap();
        let expected: Value =
            serde_json::from_slice(&fixture(&format!("{id}.program.expected.json"))).unwrap();
        assert_eq!(actual, expected, "{description}: complete Program");
        assert_eq!(json.status.code(), Some(0), "{description}");
        assert_eq!(
            json.stdout,
            fixture(&format!("{id}.check.stdout.expected.json")),
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
}
