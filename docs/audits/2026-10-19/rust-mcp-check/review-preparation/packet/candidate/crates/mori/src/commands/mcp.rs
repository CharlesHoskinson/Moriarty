//! Read-only MCP source checking over newline-delimited JSON-RPC stdio.
//!
//! This slice supports MCP 2025-06-18 and advertises only the implemented
//! `check` tool. The source/report consumer is shared with `mori check`;
//! no scenario, evaluator, target, filesystem source path or proof is accepted.

use std::io::{self, BufRead, Read, Write};
use std::process::ExitCode;

use mori_diagnostics::MoriDiagnostic;
use serde_json::{Map, Value, json};

use crate::shell::Shell;

use super::check_source::{self, SourceCheckConsumer};

const PROTOCOL_VERSION: &str = "2025-06-18";
const MAX_FRAME_BYTES: usize = 262_144;
const MAX_RESPONSE_BYTES: usize = 524_288;
const MAX_SOURCE_BYTES: usize = 65_536;
const MAX_DISPLAY_BYTES: usize = 256;

#[derive(Debug, clap::Args)]
pub struct Args {}

pub fn run(_args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    serve(io::stdin().lock(), io::stdout().lock()).map_err(super::core_io::operation)?;
    Ok(ExitCode::SUCCESS)
}

/// Process one request at a time; EOF terminates the stdio connection.
fn serve(mut input: impl BufRead, mut output: impl Write) -> io::Result<()> {
    let mut session = Session::default();
    let mut line = Vec::new();
    loop {
        line.clear();
        let count = (&mut input)
            .take((MAX_FRAME_BYTES + 1) as u64)
            .read_until(b'\n', &mut line)?;
        if count == 0 {
            return Ok(());
        }
        if line.len() > MAX_FRAME_BYTES || line.last() != Some(&b'\n') {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "MCP frame exceeds 262144 bytes or is missing its final newline",
            ));
        }
        let reply = match serde_json::from_slice::<Value>(&line) {
            Ok(request) => session.receive(request),
            Err(_) => Some(error(Value::Null, -32700, "Parse error")),
        };
        if let Some(reply) = reply {
            send(&mut output, &reply)?;
        }
    }
}

/// Buffer a complete bounded message before writing any bytes to stdout.
struct ResponseBuffer(Vec<u8>);

impl Write for ResponseBuffer {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > MAX_RESPONSE_BYTES - 1 - self.0.len() {
            return Err(io::Error::other("MCP response byte limit"));
        }
        self.0.extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn send(output: &mut impl Write, reply: &Value) -> io::Result<()> {
    let mut buffer = ResponseBuffer(Vec::new());
    if serde_json::to_writer(&mut buffer, reply).is_err() {
        buffer.0.clear();
        let refusal = error(
            reply["id"].clone(),
            -32603,
            "Response exceeds 524288-byte limit",
        );
        serde_json::to_writer(&mut buffer, &refusal).map_err(io::Error::other)?;
    }
    buffer.0.push(b'\n');
    output.write_all(&buffer.0)?;
    output.flush()
}

#[derive(Default)]
struct Session {
    initialized: bool,
    ready: bool,
}

#[derive(Clone, Copy)]
struct RpcError(i32, &'static str);

type RpcResult<T> = Result<T, RpcError>;

fn error(id: Value, code: i32, message: &str) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "error": {"code": code, "message": message}})
}

fn object(value: Option<&Value>) -> RpcResult<&Map<String, Value>> {
    value
        .and_then(Value::as_object)
        .ok_or(RpcError(-32602, "Invalid params"))
}

fn empty_params(params: Option<&Value>) -> bool {
    params.is_none_or(|p| p.as_object().is_some_and(Map::is_empty))
}

impl Session {
    fn receive(&mut self, request: Value) -> Option<Value> {
        let Some(request) = request.as_object() else {
            return Some(error(Value::Null, -32600, "Invalid Request"));
        };
        let method = request.get("method").and_then(Value::as_str);
        let id = request.get("id");
        let valid_id = id.is_none_or(|id| id.is_string() || id.is_i64() || id.is_u64());
        if request.get("jsonrpc").and_then(Value::as_str) != Some("2.0")
            || method.is_none()
            || !valid_id
        {
            return Some(error(Value::Null, -32600, "Invalid Request"));
        }
        let method = method.unwrap();
        let params = request.get("params");
        let Some(id) = id else {
            // Notifications never get responses or execute a tool call.
            if method == "notifications/initialized" && self.initialized && empty_params(params) {
                self.ready = true;
            }
            return None;
        };
        let result = self.request(method, params);
        Some(match result {
            Ok(result) => json!({"jsonrpc": "2.0", "id": id, "result": result}),
            Err(RpcError(code, message)) => error(id.clone(), code, message),
        })
    }

    fn request(&mut self, method: &str, params: Option<&Value>) -> RpcResult<Value> {
        // MCP permits pings during initialization as well as normal operation.
        if method == "ping" {
            return if empty_params(params) {
                Ok(json!({}))
            } else {
                Err(RpcError(-32602, "Invalid params"))
            };
        }
        if method == "initialize" {
            if self.initialized {
                return Err(RpcError(-32600, "Server already initialized"));
            }
            let p = object(params)?;
            let valid_version = p
                .get("protocolVersion")
                .and_then(Value::as_str)
                .is_some_and(|v| !v.is_empty() && v.len() <= 32);
            let info = p.get("clientInfo").and_then(Value::as_object);
            let valid_info = info.is_some_and(|info| {
                ["name", "version"].iter().all(|key| {
                    info.get(*key)
                        .and_then(Value::as_str)
                        .is_some_and(|v| !v.is_empty())
                })
            });
            if !valid_version || !valid_info || !p.get("capabilities").is_some_and(Value::is_object)
            {
                return Err(RpcError(-32602, "Invalid initialize parameters"));
            }
            self.initialized = true;
            // A different requested version gets this server's supported version;
            // the client must decide whether to continue or disconnect.
            return Ok(json!({
                "protocolVersion": PROTOCOL_VERSION,
                "capabilities": {"tools": {"listChanged": false}},
                "serverInfo": {"name": "moriarty-rust", "version": env!("CARGO_PKG_VERSION")},
                "instructions": "Read-only source checking only; no execution, proof or ledger qualification."
            }));
        }
        if !self.initialized {
            return Err(RpcError(-32002, "Server not initialized"));
        }
        if !self.ready {
            return Err(RpcError(-32002, "Client initialization is incomplete"));
        }
        match method {
            "tools/list" if empty_params(params) => Ok(tools()),
            "tools/list" => Err(RpcError(-32602, "Invalid params")),
            "tools/call" => call_tool(params),
            _ => Err(RpcError(-32601, "Method not found")),
        }
    }
}

fn tools() -> Value {
    json!({"tools": [{
        "name": "check",
        "description": "Check in-memory source with the existing Rust CLI parser/checker; no execution or proof qualification.",
        "inputSchema": {
            "type": "object", "properties": {
                "source": {"type": "string", "maxLength": MAX_SOURCE_BYTES},
                "displayName": {"type": "string", "maxLength": MAX_DISPLAY_BYTES}
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

fn call_tool(params: Option<&Value>) -> RpcResult<Value> {
    let params = object(params)?;
    if params.get("name").and_then(Value::as_str) != Some("check") {
        return Err(RpcError(-32602, "Unknown tool"));
    }
    let args = object(params.get("arguments"))?;
    if !args.contains_key("source") || args.keys().any(|k| k != "source" && k != "displayName") {
        return Err(RpcError(
            -32602,
            "Tool arguments must match the closed text schema",
        ));
    }
    let source = args
        .get("source")
        .and_then(Value::as_str)
        .ok_or(RpcError(-32602, "Source must be a string"))?;
    if source.len() > MAX_SOURCE_BYTES {
        return Err(RpcError(-32602, "Source exceeds 65536 UTF-8 bytes"));
    }
    let display_name = match args.get("displayName") {
        None => "<mcp>",
        Some(Value::String(name)) if name.len() <= MAX_DISPLAY_BYTES => name,
        _ => return Err(RpcError(-32602, "Invalid displayName")),
    };
    let result = check_source::dispatch(
        source,
        CheckConsumer {
            source,
            display_name,
        },
    )
    .map_err(|_| RpcError(-32603, "Could not serialize check report"))?;
    let text = serde_json::to_string(&result)
        .map_err(|_| RpcError(-32603, "Could not serialize tool result"))?;
    Ok(json!({
        "isError": result["report"]["status"] != "accepted",
        "content": [{"type": "text", "text": text}], "structuredContent": result
    }))
}

struct CheckConsumer<'a> {
    source: &'a str,
    display_name: &'a str,
}

fn qualified(source_profile: Option<&str>, core_profile: Option<&str>, report: Value) -> Value {
    json!({
        "consumer": "rust-mori-check-source", "sourceProfile": source_profile,
        "coreProfile": core_profile, "qualification": "source-check-only", "report": report
    })
}

impl SourceCheckConsumer for CheckConsumer<'_> {
    type Output = Result<Value, serde_json::Error>;

    fn beta(
        self,
        checked: Option<&mori_checker::Checked<'_>>,
        diagnostics: Vec<MoriDiagnostic>,
    ) -> Self::Output {
        let report = check_source::beta_report(
            self.display_name.to_owned(),
            self.source,
            checked,
            &diagnostics,
        );
        Ok(qualified(
            Some(mori_parser::PROFILE),
            None,
            serde_json::to_value(report)?,
        ))
    }

    fn core(
        self,
        result: Result<mori_core::Program, mori_core::Error>,
        staged: bool,
    ) -> Self::Output {
        let (source_profile, core_profile) = match &result {
            Ok(program) => (
                Some(program.language.clone()),
                Some(program.semantics.clone()),
            ),
            Err(_) => (
                mori_core::language(self.source).ok().map(str::to_owned),
                None,
            ),
        };
        let report = check_source::core_report(self.display_name, self.source, &result, staged);
        Ok(qualified(
            source_profile.as_deref(),
            core_profile.as_deref(),
            report,
        ))
    }
}
