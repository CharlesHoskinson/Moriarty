//! Complete normal stdio transcripts for the public Rust `mori mcp` consumer.
//! Source expectations are literal declarations; these tests invoke no evaluator.

use std::io::Write;
use std::process::{Command, Stdio};

use serde_json::{Value, json};

const CORE1: &str = "language \"moriarty/1\";\nagreement Example { action go() {} }\n";
const CORE2: &str = "language \"moriarty/2\";\nagreement Example { action go() {} }\n";
const BETA: &str = "profile \"moriarty-beta/1\";\nagreement Example {}\n";

fn request(id: Value, method: &str, params: Value) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params})
}

fn initialize() -> Value {
    request(
        json!(1),
        "initialize",
        json!({
            "protocolVersion": "2025-06-18", "capabilities": {},
            "clientInfo": {"name": "mori-protocol-test", "version": "1"}
        }),
    )
}

fn ready() -> Value {
    json!({"jsonrpc": "2.0", "method": "notifications/initialized"})
}

fn response(id: Value, result: Value) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "result": result})
}

fn error(id: Value, code: i32, message: &str) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "error": {"code": code, "message": message}})
}

fn initialized() -> Value {
    response(
        json!(1),
        json!({
            "protocolVersion": "2025-06-18",
            "capabilities": {"tools": {"listChanged": false}},
            "serverInfo": {"name": "moriarty-rust", "version": "0.1.0"},
            "instructions": "Read-only source checking only; no execution, proof or ledger qualification."
        }),
    )
}

fn tool_list() -> Value {
    json!({"tools": [{
        "name": "check",
        "description": "Check in-memory source with the existing Rust CLI parser/checker; no execution or proof qualification.",
        "inputSchema": {
            "type": "object", "properties": {
                "source": {"type": "string", "maxLength": 65536},
                "displayName": {"type": "string", "maxLength": 256}
            }, "required": ["source"], "additionalProperties": false
        },
        "outputSchema": {
            "type": "object", "properties": {
                "consumer": {"const": "rust-mori-check-source"},
                "sourceProfile": {"type": ["string", "null"]},
                "coreProfile": {"type": ["string", "null"]},
                "qualification": {"const": "source-check-only"},
                "report": {"type": "object"}
            }, "required": ["consumer", "sourceProfile", "coreProfile", "qualification", "report"],
            "additionalProperties": false
        },
        "annotations": {"readOnlyHint": true, "destructiveHint": false,
                        "idempotentHint": true, "openWorldHint": false}
    }]})
}

fn tool_result(source_profile: Value, core_profile: Value, report: Value) -> Value {
    let is_error = report["status"] != "accepted";
    let structured = json!({
        "consumer": "rust-mori-check-source", "sourceProfile": source_profile,
        "coreProfile": core_profile, "qualification": "source-check-only", "report": report
    });
    json!({"content": [{"type": "text", "text": serde_json::to_string(&structured).unwrap()}],
           "structuredContent": structured, "isError": is_error})
}

fn accepted_core(profile: &str, hash: &str, file: &str) -> Value {
    json!({"status": "accepted", "file": file, "source_hash": hash,
           "language": profile, "agreement": "Example", "diagnostics": [],
           "actions": [{"name": "go", "inputs": [], "statements": []}]})
}

fn transcript(input: &[Value], expected: &[Value], sources: &str) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_mori"))
        .args(["--color", "never", "mcp"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn public mori mcp");
    let mut stdin = child.stdin.take().unwrap();
    for value in input {
        serde_json::to_writer(&mut stdin, value).unwrap();
        stdin.write_all(b"\n").unwrap();
    }
    drop(stdin); // Finite input: MCP stdio shutdown is EOF, not a shutdown RPC.
    let output = child
        .wait_with_output()
        .expect("collect complete MCP transcript");
    let mut expected_stdout = Vec::new();
    for value in expected {
        serde_json::to_writer(&mut expected_stdout, value).unwrap();
        expected_stdout.push(b'\n');
    }
    assert_eq!(
        output.status.code(),
        Some(0),
        "sources:\n{sources}\n{:?}",
        output.stderr
    );
    assert!(
        output.stderr.is_empty(),
        "sources:\n{sources}\n{:?}",
        output.stderr
    );
    assert_eq!(output.stdout, expected_stdout, "sources:\n{sources}");
}

#[test]
fn mcp_negotiates_lists_only_check_and_preserves_string_request_id() {
    let mut init = initialize();
    init["params"]["protocolVersion"] = json!("2024-11-05");
    transcript(
        &[
            request(json!("early-ping"), "ping", json!({})),
            init,
            ready(),
            request(json!("tool-list"), "tools/list", json!({})),
            json!({"jsonrpc": "2.0", "method": "notifications/unrecognized"}),
        ],
        &[
            response(json!("early-ping"), json!({})),
            initialized(),
            response(json!("tool-list"), tool_list()),
        ],
        "no source supplied",
    );
}

#[test]
fn mcp_checks_core1_and_core2_through_the_public_stdio_consumer() {
    let core1 = accepted_core(
        "moriarty/1",
        "d49f4eba1ed6e246b579ff734b0b27f2ad7369eb4ac7b14119cecffd10fd27ce",
        "<mcp>",
    );
    let mut core2 = accepted_core(
        "moriarty/2",
        "1d67f9e645473a930e03da7331b0749b26b269630c171f44a0e96877406e79e7",
        "editor:two",
    );
    core2["qualification"] = json!("local-reference-stage");
    core2["authentication"] = json!("none");
    core2["consumption"] = json!("untracked");
    transcript(
        &[
            initialize(),
            ready(),
            request(
                json!(2),
                "tools/call",
                json!({"name": "check", "arguments": {"source": CORE1}}),
            ),
            request(
                json!(3),
                "tools/call",
                json!({"name": "check", "arguments": {"source": CORE2, "displayName": "editor:two"}}),
            ),
        ],
        &[
            initialized(),
            response(
                json!(2),
                tool_result(json!("moriarty/1"), json!("moriarty-core/1"), core1),
            ),
            response(
                json!(3),
                tool_result(json!("moriarty/2"), json!("moriarty-core/2"), core2),
            ),
        ],
        &format!("{CORE1}\n{CORE2}"),
    );
}

#[test]
fn mcp_preserves_beta_report_and_returns_core_refusal_as_tool_error() {
    let refused = "language \"moriarty/1\";\nagreement Empty {}\n";
    let beta = json!({"status": "accepted", "file": "<mcp>",
        "source_hash": "35652fcdbd57acefc7f65a6085ca762afaee11cba9e7c98f0c7ca71edc9e9656",
        "agreement": {"name": "Example", "span": {"start": 37, "end": 44}},
        "declarations": [], "actions": [], "diagnostics": []});
    let rejection = json!({"status": "rejected", "file": "<mcp>", "diagnostics": [{
        "code": "mori::core::program", "message": "Program needs at least one action.",
        "offset": null, "step": null
    }]});
    transcript(
        &[
            initialize(),
            ready(),
            request(
                json!(2),
                "tools/call",
                json!({"name": "check", "arguments": {"source": BETA}}),
            ),
            request(
                json!(3),
                "tools/call",
                json!({"name": "check", "arguments": {"source": refused}}),
            ),
        ],
        &[
            initialized(),
            response(
                json!(2),
                tool_result(json!("moriarty-beta/1"), Value::Null, beta),
            ),
            response(
                json!(3),
                tool_result(json!("moriarty/1"), Value::Null, rejection),
            ),
        ],
        &format!("{BETA}\n{refused}"),
    );
}

#[test]
fn mcp_observes_lifecycle_and_closed_check_arguments() {
    transcript(
        &[
            request(json!(1), "tools/list", json!({})),
            initialize(),
            request(json!(2), "tools/list", json!({})),
            ready(),
            request(json!(3), "inspect", json!({})),
            request(
                json!(4),
                "tools/call",
                json!({"name": "check", "arguments": {"source": CORE1, "path": "unused.mori"}}),
            ),
            request(
                json!(5),
                "tools/call",
                json!({"name": "inspect", "arguments": {"source": CORE1}}),
            ),
        ],
        &[
            error(json!(1), -32002, "Server not initialized"),
            initialized(),
            error(json!(2), -32002, "Client initialization is incomplete"),
            error(json!(3), -32601, "Method not found"),
            error(
                json!(4),
                -32602,
                "Tool arguments must match the closed text schema",
            ),
            error(json!(5), -32602, "Unknown tool"),
        ],
        CORE1,
    );
}

#[test]
fn mcp_accepts_opaque_metadata_during_initialization_ping_and_tool_listing() {
    transcript(
        &[
            initialize(),
            json!({
                "jsonrpc": "2.0", "method": "notifications/initialized",
                "params": {"_meta": {"example.org/client": "mori-normal-metadata-case"}}
            }),
            request(
                json!(2),
                "ping",
                json!({"_meta": {"example.org/sequence": 2}}),
            ),
            request(
                json!(3),
                "tools/list",
                json!({
                    "_meta": {"example.org/context": {"editor": "test"}}
                }),
            ),
        ],
        &[
            initialized(),
            response(json!(2), json!({})),
            response(json!(3), tool_list()),
        ],
        "no source supplied; optional metadata remains opaque",
    );
}
