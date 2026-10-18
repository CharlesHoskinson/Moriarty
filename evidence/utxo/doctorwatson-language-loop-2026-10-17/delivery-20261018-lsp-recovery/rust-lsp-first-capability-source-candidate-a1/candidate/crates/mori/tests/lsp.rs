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
        let body = wire.get(body_start..body_end).expect("complete message body");
        messages.push(serde_json::from_slice(body).unwrap());
        wire = &wire[body_end..];
    }
    messages
}
