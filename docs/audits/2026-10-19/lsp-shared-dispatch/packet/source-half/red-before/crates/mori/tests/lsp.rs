//! The public Rust stdio diagnostic session with frozen normal expectations.
use std::io::Write;
use std::process::{Command, Stdio};

use serde_json::Value;

#[test]
fn full_sync_diagnostics_follow_public_checker_and_clear_after_edit() {
    let input: Vec<Value> =
        serde_json::from_str(include_str!("fixtures/lsp/input-session.json")).unwrap();
    let expected: Value =
        serde_json::from_str(include_str!("fixtures/lsp/expected-session.json")).unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_mori"))
        .args(["--color", "never", "lsp"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    {
        let mut stdin = child.stdin.take().unwrap();
        for message in input {
            let body = serde_json::to_vec(&message).unwrap();
            write!(stdin, "Content-Length: {}\r\n\r\n", body.len()).unwrap();
            stdin.write_all(&body).unwrap();
        }
    }
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(0), "{:?}", output);
    assert_eq!(
        output.stderr.as_slice(),
        expected["stderrUtf8"].as_str().unwrap().as_bytes()
    );
    assert_eq!(
        Value::Array(read_frames(&output.stdout)),
        expected["orderedMessages"]
    );
}

/// Read the wire independently of the server's lsp-server framing helpers.
fn read_frames(mut wire: &[u8]) -> Vec<Value> {
    let mut messages = Vec::new();
    while !wire.is_empty() {
        let header_end = wire
            .windows(4)
            .position(|bytes| bytes == b"\r\n\r\n")
            .expect("stdout contains only complete framed messages");
        let header = std::str::from_utf8(&wire[..header_end]).unwrap();
        let length: usize = header
            .strip_prefix("Content-Length: ")
            .expect("the server emits one Content-Length header")
            .parse()
            .unwrap();
        let body_start = header_end + 4;
        let body_end = body_start + length;
        let body = wire
            .get(body_start..body_end)
            .expect("complete message body");
        messages.push(serde_json::from_slice(body).unwrap());
        wire = &wire[body_end..];
    }
    messages
}

struct ParityDirectory(std::path::PathBuf);

impl ParityDirectory {
    fn new() -> Self {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("current time is after the Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "mori-lsp-parity-{}-{nonce}-{}",
            std::process::id(),
            NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).expect("create isolated LSP parity directory");
        Self(path)
    }
}

impl Drop for ParityDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn parity_session(input: &[Value]) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_mori"))
        .args(["--color", "never", "lsp"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("run public LSP parity session");
    {
        let mut stdin = child.stdin.take().unwrap();
        for message in input {
            let body = serde_json::to_vec(message).unwrap();
            write!(stdin, "Content-Length: {}\r\n\r\n", body.len()).unwrap();
            stdin.write_all(&body).unwrap();
        }
    }
    child
        .wait_with_output()
        .expect("collect LSP parity session")
}

#[test]
fn shared_dispatch_preserves_unknown_header_and_supported_profile_parity() {
    let input: Vec<Value> = serde_json::from_str(include_str!(
        "../../fixtures/lsp-shared-dispatch/input-session.json"
    ))
    .unwrap();
    let expected: Value = serde_json::from_str(include_str!(
        "../../fixtures/lsp-shared-dispatch/expected-session.json"
    ))
    .unwrap();
    let output = parity_session(&input);
    let documents: Vec<_> = input
        .iter()
        .filter(|message| message["method"] == "textDocument/didOpen")
        .map(|message| &message["params"]["textDocument"])
        .collect();
    let checks = expected["checks"].as_array().unwrap();
    assert_eq!(documents.len(), checks.len());
    let directory = ParityDirectory::new();
    // Collect each actual public check before comparing either consumer.
    let observed_checks: Vec<_> = documents
        .iter()
        .map(|document| {
            std::fs::write(
                directory.0.join("parity.mori"),
                document["text"].as_str().unwrap(),
            )
            .unwrap();
            Command::new(env!("CARGO_BIN_EXE_mori"))
                .current_dir(&directory.0)
                .args([
                    "--color",
                    "never",
                    "--quiet",
                    "check",
                    "parity.mori",
                    "--json",
                ])
                .output()
                .expect("run public parity check")
        })
        .collect();
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert_eq!(
        output.stderr.as_slice(),
        expected["stderrUtf8"].as_str().unwrap().as_bytes()
    );
    let messages = read_frames(&output.stdout);
    assert_eq!(Value::Array(messages.clone()), expected["orderedMessages"]);
    for (index, ((document, expected_check), actual)) in documents
        .iter()
        .zip(checks)
        .zip(observed_checks)
        .enumerate()
    {
        assert_eq!(document["uri"], expected_check["uri"]);
        assert_eq!(
            actual.status.code().map(i64::from),
            expected_check["exit"].as_i64(),
            "{actual:?}"
        );
        assert_eq!(
            actual.stderr.as_slice(),
            expected_check["stderrUtf8"].as_str().unwrap().as_bytes()
        );
        let report: Value = serde_json::from_slice(&actual.stdout).unwrap();
        assert_eq!(report["status"], expected_check["status"]);
        assert_eq!(report["diagnostics"], expected_check["diagnostics"]);
        let published = &messages[1 + 2 * index]["params"];
        assert_eq!(published["uri"], document["uri"]);
        let lsp_diagnostics = published["diagnostics"].as_array().unwrap();
        let check_diagnostics = report["diagnostics"].as_array().unwrap();
        assert_eq!(lsp_diagnostics.len(), check_diagnostics.len());
        for (lsp, check) in lsp_diagnostics.iter().zip(check_diagnostics) {
            assert_eq!(lsp["code"], check["code"]);
            assert_eq!(lsp["message"], check["message"]);
        }
    }
}
