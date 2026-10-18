//! Public reference-stage consumers against literals frozen before implementation.
//! No test in this file reaches native, proof, replay, or ledger execution; the
//! H16/H19 `--trace` and `compile-call` requests are refused at the profile first.
use serde_json::{Value, json};
use std::{
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);
const DELIVERY: &str = include_str!("../../../examples/core/continuations/delivery.mori");
const RESERVE: &str = include_str!("../../../examples/core/continuations/reserve.mori");
const LEGACY: &str = include_str!("../../../examples/core/continuations/atomic-compatibility.mori");
fn oracle() -> Value {
    serde_json::from_str(include_str!(
        "../../../examples/core/continuations/normal-oracle.json"
    ))
    .unwrap()
}
struct Files(PathBuf);
impl Files {
    fn new(source: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "mori-core-continuations-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        let f = Self(path);
        f.write("source.mori", source.as_bytes());
        f
    }
    fn path(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
    fn write(&self, name: &str, bytes: impl AsRef<[u8]>) {
        std::fs::write(self.path(name), bytes).unwrap();
    }
    fn json(&self, name: &str, value: &Value) {
        self.write(name, serde_json::to_vec(value).unwrap());
    }
    fn command(&self, command: &str) -> Command {
        let mut c = Command::new(env!("CARGO_BIN_EXE_mori"));
        c.args(["--color", "never", command])
            .arg(self.path("source.mori"));
        c
    }
    fn start(&self, action: &str, opening: &Value, cap: Option<&str>) -> Output {
        self.json("scenario.json", opening);
        let mut c = self.command("simulate");
        c.args(["--action", action, "--scenario"])
            .arg(self.path("scenario.json"));
        if let Some(cap) = cap {
            c.args(["--logical-cap", cap]);
        }
        c.output().unwrap()
    }
    fn resume(&self, continuation: &Value, event: &Value, action: Option<&str>) -> Output {
        self.json("continuation.json", continuation);
        self.json("event.json", event);
        let mut c = self.command("simulate");
        c.arg("--resume")
            .arg(self.path("continuation.json"))
            .arg("--event")
            .arg(self.path("event.json"));
        if let Some(action) = action {
            c.args(["--action", action]);
        }
        c.output().unwrap()
    }
}
impl Drop for Files {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn report(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout).unwrap_or_else(|e| {
        panic!(
            "{e}: stdout={} stderr={}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    })
}
fn accepted(output: &Output) -> Value {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let r = report(output);
    for (key, expected) in oracle()["normal_output_labels_from_opus"]
        .as_object()
        .unwrap()
    {
        assert_eq!(&r[key], expected, "{key}");
    }
    r
}
