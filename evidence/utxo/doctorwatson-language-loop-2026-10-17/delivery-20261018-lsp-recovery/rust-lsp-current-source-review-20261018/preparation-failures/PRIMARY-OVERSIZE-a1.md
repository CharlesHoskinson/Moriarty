# Full current Rust LSP source-review primary

SOURCE REVIEW ONLY. Candidate HEAD adff55f33942310123dc32f272cd6291e5b5c4da; tree fe345ec28bd5444ea8c7204a4eb48d643962fe8e; integrated over actual main e4d7a2b5709caa18a94f5256d03a0302364b8235. All eight changed files are physically complete under candidate/, including complete Cargo.lock (118092 bytes). This inline primary is deliberately bounded: it includes all three candidate Rust files, both complete author normal-session fixtures, both changed manifests, full actual lsp-server Message implementation and full actual public check router, complete 37-row API table, independent L12 oracle/custody/inputs, both original approved design WHOLE-object arrays and root adoption, and actual metadata preparation result records. Cargo.lock, full generated dependency definitions, complete parser/checker/diagnostic/core source trees, full design and all public failed preparations are physically present, not falsely described as inline or already read.

Current graph evidence is REAL Cargo metadata, 456 resolved nodes via explicit 428-package local directory source replacement, selecting lsp-server0.10.0, gen-lsp-types0.11.0 and crossbeam-channel0.5.17. No compilation/server/protocol result exists. Historical design records saying UNKNOWN graph or LSP stub describe their old exact tree, not this candidate. Preserve those records. Source-only API table likewise predates metadata resolution; it is not compiler evidence. Author normal fixture is exposed and not an independent oracle. Independent L12 has separate inputs/messages and expressly separates optional adapter fields/help text from semantic message; preserve this distinction.

Both current reviewers must review the complete eight-file candidate and full design/adoption, actual Message read/write, actual check routing/diagnostic dependencies and full normal evidence; cite exact file/ranges and disclose uninspected closure honestly. No blanket full-tree read claim follows from the primary. Read additional physical public files as necessary using read-only tools; do not execute any command, tests, model, server, package, native/SDK/ledger action, write/modify a file, research the web or spawn any agent. Return exact source change requests or a qualified source opinion. No review grants an unspecified runtime plan. Existing user authority lets root independently select inspected bounded ordinary native-OFF normal tooling tests; this review adds no extra approval gate. Such a normal pass cannot imply protocol safety/security, production qualification, proofs, target/ledger or product closure. Security/source/lifecycle/resource judgments are assigned only to the designated security fallback review; the ordinary review must state its boundary.

Physical must-read guides: design/DESIGN.md, design/EARS-PROPOSAL.md; candidate/Cargo.lock; source/crates/mori/src/{main,cli}.rs and commands/{mod,check,core_io}.rs; source/crates/mori-{lexer,parser,checker,ast,span,diagnostics,core}/; public-dependencies/lsp-server-0.10.0/src/{lib,msg}.rs and gen-lsp-types-0.11.0/src/; api-compatibility/API-COMPATIBILITY.json; independent-L12/PARSER-CHECKER-BASIS.json and LSP-NORMATIVE-EXCERPTS.json. These are complete physical files; report read scope rather than assuming all read. Current source is pinned by CURRENT-CANDIDATE and REPOSITORY-SOURCE-PINS. Full failed metadata preparations and original failed a1 design preparations are retained separately.

## candidate/Cargo.toml (COMPLETE FILE)

```text
[workspace]
resolver = "3"
members = ["crates/*"]
# Standalone experiment and evidence crates keep their own builds, and
# crates/fixtures holds shared test programs, not a crate.
exclude = ["experiments", "raw", "wiki-llm", "deliverables", "crates/fixtures"]

[workspace.package]
edition = "2024"
rust-version = "1.99"
license = "Apache-2.0"
repository = "https://github.com/CharlesHoskinson/Moriarty"

[workspace.dependencies]
mori-ast = { path = "crates/mori-ast" }
mori-backend = { path = "crates/mori-backend" }
mori-checker = { path = "crates/mori-checker" }
mori-core = { path = "crates/mori-core" }
mori-diagnostics = { path = "crates/mori-diagnostics" }
mori-fmt = { path = "crates/mori-fmt" }
mori-ledger = { path = "crates/mori-ledger" }
mori-lexer = { path = "crates/mori-lexer" }
mori-parser = { path = "crates/mori-parser" }
mori-source6 = { path = "crates/mori-source6" }
mori-span = { path = "crates/mori-span" }

# Midnight ledger crates, all from one pinned revision.
midnight-base-crypto = { git = "https://github.com/midnightntwrk/midnight-ledger", rev = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14" }
midnight-coin-structure = { git = "https://github.com/midnightntwrk/midnight-ledger", rev = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14" }
midnight-ledger = { git = "https://github.com/midnightntwrk/midnight-ledger", rev = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14", package = "midnight-ledger-v9" }
midnight-onchain-runtime = { git = "https://github.com/midnightntwrk/midnight-ledger", rev = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14" }
midnight-onchain-state = { git = "https://github.com/midnightntwrk/midnight-ledger", rev = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14" }
midnight-onchain-vm = { git = "https://github.com/midnightntwrk/midnight-ledger", rev = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14" }
midnight-serialize = { git = "https://github.com/midnightntwrk/midnight-ledger", rev = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14" }
midnight-storage = { git = "https://github.com/midnightntwrk/midnight-ledger", rev = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14" }
midnight-transient-crypto = { git = "https://github.com/midnightntwrk/midnight-ledger", rev = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14" }
midnight-zkir-v3 = { git = "https://github.com/midnightntwrk/midnight-ledger", rev = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14" }
# Published crates that must match the versions the ledger revision uses.
midnight-curves = "=0.3.1"
midnight-proofs = "=0.8.3"

anstream = "1.0.0"
anstyle = "1.0.14"
clap = { version = "4.6.7", features = ["derive"] }
futures = "=0.3.34"
ignore = "0.4.33"
indoc = "2.0.7"
insta = "1.49.0"
k256 = { version = "=0.13.4", features = ["schnorr", "ecdsa"] }
libc = "=0.2.190"
miette = { version = "7.6.0", default-features = false }
lsp-server = "=0.10.0"
lsp-types = { package = "gen-lsp-types", version = "=0.11.0", default-features = false }
proptest = "1.11.0"
rand = "=0.8.8"
rayon = "1.12.0"
serde = { version = "1.0.229", features = ["derive"] }
serde_json = "1.0.151"
sha2 = "0.11.0"
similar = "2.7.0"
soa-rs = "1.0.1"
terminal_size = "0.4.4"
thiserror = "2.0.21"

# Faster snapshot diffing, as recommended by insta.
[profile.dev.package]
insta.opt-level = 3
similar.opt-level = 3

# The pinned ledger's v2 compatibility path requires unpublished crypto 2.2.
# Mirror its own workspace patches with an immutable retained release commit;
# direct v3 dependencies remain pinned to 04c9c5d. No mutable local paths.
[patch.crates-io]
midnight-transient-crypto-old = { git = "https://github.com/midnightntwrk/midnight-ledger", rev = "a01a1ea0270d2e8a1f9a58f0e3cf5ceb029283e1", package = "midnight-transient-crypto" }
midnight-serialize = { git = "https://github.com/midnightntwrk/midnight-ledger", rev = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14" }
midnight-base-crypto = { git = "https://github.com/midnightntwrk/midnight-ledger", rev = "04c9c5d9bcebb8d4427d8589fb54d58a55599c14" }

```

## candidate/crates/mori/Cargo.toml (COMPLETE FILE)

```text
[package]
name = "mori"
version = "0.1.0"
description = "Command-line interface for the Moriarty toolchain"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
repository.workspace = true

[features]
core2-disjoint-draft = ["mori-core/core2-disjoint-draft"]
default = ["native"]
source6-expansion = ["dep:mori-source6", "mori-source6/source6-transfer-slice"]
native = [
    "dep:mori-backend",
    "dep:mori-ledger",
    "dep:futures",
    "dep:rand",
    "dep:midnight-ledger",
    "dep:midnight-base-crypto",
    "dep:midnight-coin-structure",
    "dep:midnight-onchain-state",
    "dep:midnight-onchain-runtime",
    "dep:midnight-storage",
    "dep:midnight-transient-crypto",
    "dep:midnight-serialize",
]

[dependencies]
anstream.workspace = true
anstyle.workspace = true
clap.workspace = true
ignore.workspace = true
lsp-server.workspace = true
lsp-types.workspace = true
mori-ast.workspace = true
mori-checker.workspace = true
mori-core.workspace = true
mori-backend = { workspace = true, optional = true }
mori-ledger = { workspace = true, optional = true }
mori-diagnostics = { workspace = true, features = ["fancy"] }
mori-fmt.workspace = true
mori-lexer.workspace = true
mori-parser.workspace = true
mori-source6 = { workspace = true, optional = true }
mori-span.workspace = true
rayon.workspace = true
futures = { workspace = true, optional = true }
rand = { workspace = true, optional = true }
serde.workspace = true
serde_json.workspace = true
sha2.workspace = true
terminal_size.workspace = true
thiserror.workspace = true

midnight-ledger = { workspace = true, optional = true }
midnight-base-crypto = { workspace = true, optional = true }
midnight-coin-structure = { workspace = true, optional = true }
midnight-onchain-state = { workspace = true, optional = true }
midnight-onchain-runtime = { workspace = true, optional = true }
midnight-storage = { workspace = true, optional = true }
midnight-transient-crypto = { workspace = true, optional = true }
midnight-serialize = { workspace = true, optional = true }

[dev-dependencies]
indoc.workspace = true
insta.workspace = true

[[test]]
name = "check_compile_io"
required-features = ["native"]

[[test]]
name = "check_static_compile_io"
required-features = ["native"]

[[test]]
name = "prepare_call"
required-features = ["native"]

[[test]]
name = "installed_call"
required-features = ["native"]

[[test]]
name = "signed_installed_call"
required-features = ["native"]

[[test]]
name = "native_proofs"
required-features = ["native"]

[[test]]
name = "signed_intents"
required-features = ["native"]

[[test]]
name = "replay"
required-features = ["native"]

[[test]]
name = "replay_challenge"
required-features = ["native"]

[[test]]
name = "enforcement_report"
required-features = ["native"]

[[test]]
name = "mapped_role_static_io"
required-features = ["native"]

[[test]]
name = "source6_expand"
required-features = ["source6-expansion"]

```

## candidate/crates/mori/src/commands/lsp.rs (COMPLETE FILE)

```text
use std::collections::HashMap;
use std::io::{Write, stdin, stdout};
use std::process::ExitCode;

use lsp_server::{ErrorCode, Message, Notification, Request, Response};
use lsp_types::{
    Diagnostic, DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    InitializeParams, InitializeResult, PositionEncodingKind, PublishDiagnosticsParams,
    ServerCapabilities, ServerInfo, TextDocumentContentChangeEvent, TextDocumentSyncKind,
    TextDocumentSyncOptions, Uri,
};
use serde_json::json;

use crate::shell::Shell;

mod diagnostics;

#[derive(Debug, clap::Args)]
pub struct Args {}

pub fn run(_args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    let stdin = stdin();
    let stdout = stdout();
    let mut input = stdin.lock();
    let mut output = stdout.lock();
    let mut server = Server::default();
    while let Some(message) = Message::read(&mut input).map_err(super::core_io::operation)? {
        if let Some(code) = server.dispatch(message, &mut output)? {
            return Ok(code);
        }
    }
    // A normal LSP session ends with shutdown followed by exit, not just EOF.
    Ok(ExitCode::FAILURE)
}

#[derive(Default, PartialEq, Eq)]
enum Phase {
    #[default]
    New,
    Initializing,
    Running,
    Shutdown,
}

struct Document {
    version: i32,
    text: String,
}

#[derive(Default)]
struct Server {
    phase: Phase,
    documents: HashMap<Uri, Document>,
    versioned_diagnostics: bool,
    related_information: bool,
}

impl Server {
    fn dispatch(
        &mut self,
        message: Message,
        output: &mut impl Write,
    ) -> super::Result<Option<ExitCode>> {
        match message {
            Message::Request(request) => {
                let response = self.request(request);
                Message::Response(response)
                    .write(output)
                    .map_err(super::core_io::operation)?;
            }
            Message::Notification(notification) => return self.notification(notification, output),
            // This server sends no requests, so there are no pending responses.
            Message::Response(_) => {}
        }
        Ok(None)
    }

    fn request(&mut self, request: Request) -> Response {
        if request.method == "initialize" {
            if self.phase != Phase::New {
                return Response::new_err(
                    request.id,
                    ErrorCode::InvalidRequest as i32,
                    "Server already initialized.".into(),
                );
            }
            let Ok(params) = serde_json::from_value::<InitializeParams>(request.params) else {
                return Response::new_err(
                    request.id,
                    ErrorCode::InvalidParams as i32,
                    "Invalid initialize parameters.".into(),
                );
            };
            let diagnostics = params
                .capabilities
                .text_document
                .as_ref()
                .and_then(|document| document.publish_diagnostics.as_ref());
            self.versioned_diagnostics = diagnostics
                .and_then(|capabilities| capabilities.version_support)
                .unwrap_or(false);
            self.related_information = diagnostics
                .and_then(|capabilities| capabilities.diagnostics_capabilities.related_information)
                .unwrap_or(false);
            self.phase = Phase::Initializing;
            return Response::new_ok(request.id, initialize_result());
        }
        let error = match self.phase {
            Phase::New | Phase::Initializing => {
                Some((ErrorCode::ServerNotInitialized, "Server not initialized."))
            }
            Phase::Shutdown => Some((ErrorCode::InvalidRequest, "Server has shut down.")),
            Phase::Running => None,
        };
        if let Some((code, message)) = error {
            return Response::new_err(request.id, code as i32, message.into());
        }
        if request.method == "shutdown" {
            if !request.params.is_null() {
                return Response::new_err(
                    request.id,
                    ErrorCode::InvalidParams as i32,
                    "Shutdown has no parameters.".into(),
                );
            }
            self.phase = Phase::Shutdown;
            self.documents.clear();
            return Response::new_ok(request.id, ());
        }
        Response::new_err(
            request.id,
            ErrorCode::MethodNotFound as i32,
            "Method not supported by this diagnostics server.".into(),
        )
    }

    fn notification(
        &mut self,
        notification: Notification,
        output: &mut impl Write,
    ) -> super::Result<Option<ExitCode>> {
        if notification.method == "exit" {
            return Ok(Some(if self.phase == Phase::Shutdown {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }));
        }
        if notification.method == "initialized" && self.phase == Phase::Initializing {
            self.phase = Phase::Running;
            return Ok(None);
        }
        if self.phase != Phase::Running {
            return Ok(None);
        }
        match notification.method.as_str() {
            "textDocument/didOpen" => {
                let Ok(params) =
                    serde_json::from_value::<DidOpenTextDocumentParams>(notification.params)
                else {
                    return Ok(None);
                };
                let document = params.text_document;
                if self.documents.contains_key(&document.uri) {
                    return Ok(None);
                }
                let found =
                    diagnostics::check(&document.uri, &document.text, self.related_information);
                let version = self.versioned_diagnostics.then_some(document.version);
                let uri = document.uri;
                self.documents.insert(
                    uri.clone(),
                    Document {
                        version: document.version,
                        text: document.text,
                    },
                );
                publish(output, uri, version, found)?;
            }
            "textDocument/didChange" => {
                let Ok(params) =
                    serde_json::from_value::<DidChangeTextDocumentParams>(notification.params)
                else {
                    return Ok(None);
                };
                let mut text = None;
                for change in params.content_changes {
                    let TextDocumentContentChangeEvent::TextDocumentContentChangeWholeDocument(
                        change,
                    ) = change
                    else {
                        return Ok(None);
                    };
                    // Full changes replace the whole document in their supplied order.
                    text = Some(change.text);
                }
                let Some(text) = text else {
                    return Ok(None);
                };
                let uri = params.text_document.text_document_identifier.uri;
                let Some(document) = self.documents.get_mut(&uri) else {
                    return Ok(None);
                };
                if params.text_document.version <= document.version {
                    return Ok(None);
                }
                document.version = params.text_document.version;
                document.text = text;
                let found = diagnostics::check(&uri, &document.text, self.related_information);
                let version = self.versioned_diagnostics.then_some(document.version);
                publish(output, uri, version, found)?;
            }
            "textDocument/didClose" => {
                let Ok(params) =
                    serde_json::from_value::<DidCloseTextDocumentParams>(notification.params)
                else {
                    return Ok(None);
                };
                let uri = params.text_document.uri;
                if self.documents.remove(&uri).is_some() {
                    publish(output, uri, None, Vec::new())?;
                }
            }
            _ => {}
        }
        Ok(None)
    }
}

fn initialize_result() -> InitializeResult {
    InitializeResult {
        capabilities: ServerCapabilities {
            position_encoding: Some(PositionEncodingKind::UTF16),
            text_document_sync: Some(
                TextDocumentSyncOptions {
                    open_close: Some(true),
                    change: Some(TextDocumentSyncKind::Full),
                    ..Default::default()
                }
                .into(),
            ),
            experimental: Some(json!({
                "moriarty": {
                    "analysis": "local-check-only",
                    "sourceProfiles": ["moriarty-beta/1", "moriarty/1", "moriarty/2"]
                }
            })),
            ..Default::default()
        },
        server_info: Some(ServerInfo {
            name: "moriarty-rust".into(),
            version: Some(env!("CARGO_PKG_VERSION").into()),
        }),
    }
}

fn publish(
    output: &mut impl Write,
    uri: Uri,
    version: Option<i32>,
    diagnostics: Vec<Diagnostic>,
) -> super::Result<()> {
    Message::Notification(Notification::new(
        "textDocument/publishDiagnostics".into(),
        PublishDiagnosticsParams::new(uri, version, diagnostics),
    ))
    .write(output)
    .map_err(super::core_io::operation)
}

```

## candidate/crates/mori/src/commands/lsp/diagnostics.rs (COMPLETE FILE)

```text
use lsp_types::{
    Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, Position, Range, Uri,
};
use mori_diagnostics::{Diagnostic as _, MoriDiagnostic, Severity};

/// Match the public check command's source-profile dispatch without running an action.
pub(super) fn check(uri: &Uri, source: &str, related: bool) -> Vec<Diagnostic> {
    if mori_lexer::first_token_text(source).is_ok_and(|token| token == "language") {
        let staged = mori_core::language(source).is_ok_and(|language| language == "moriarty/2");
        let error = if staged {
            mori_core::stage::load(source).err()
        } else {
            mori_core::parse(source).err()
        };
        let profile = if staged { "moriarty/2" } else { "moriarty/1" };
        return error
            .map(|error| core(source, profile, error))
            .into_iter()
            .collect();
    }
    let diagnostics = match mori_parser::parse(source) {
        Err(error) => vec![error],
        Ok(ast) => mori_checker::check(&ast).diagnostics,
    };
    diagnostics
        .iter()
        .map(|diagnostic| legacy(uri, source, diagnostic, related))
        .collect()
}

fn core(source: &str, profile: &str, error: mori_core::Error) -> Diagnostic {
    let at = position(source, error.offset.unwrap_or(0) as usize);
    Diagnostic::new(
        Range::new(at, at),
        Some(DiagnosticSeverity::Error),
        Some(error.code.into()),
        None,
        Some(format!("moriarty-rust ({profile})")),
        error.message.into(),
        None,
        None,
        None,
    )
}

fn legacy(uri: &Uri, source: &str, diagnostic: &MoriDiagnostic, related: bool) -> Diagnostic {
    let labels: Vec<_> = diagnostic.labels().into_iter().flatten().collect();
    let primary = labels.iter().position(|label| label.primary()).unwrap_or(0);
    let range = labels.get(primary).map_or_else(
        || Range::new(Position::new(0, 0), Position::new(0, 0)),
        |label| label_range(source, label.offset(), label.len()),
    );
    let information: Vec<_> = if related {
        labels
            .iter()
            .enumerate()
            .filter(|(index, _)| *index != primary)
            .filter_map(|(_, label)| {
                label.label().map(|text| {
                    DiagnosticRelatedInformation::new(
                        Location::new(
                            uri.clone(),
                            label_range(source, label.offset(), label.len()),
                        ),
                        text.to_owned(),
                    )
                })
            })
            .collect()
    } else {
        Vec::new()
    };
    let mut message = diagnostic.to_string();
    if let Some(help) = diagnostic.help() {
        message.push_str("\n\n");
        message.push_str(&help.to_string());
    }
    let severity = match diagnostic.severity() {
        Some(Severity::Warning) => DiagnosticSeverity::Warning,
        Some(Severity::Advice) => DiagnosticSeverity::Information,
        _ => DiagnosticSeverity::Error,
    };
    Diagnostic::new(
        range,
        Some(severity),
        diagnostic.code().map(|code| code.to_string().into()),
        None,
        Some("moriarty-rust (moriarty-beta/1)".into()),
        message.into(),
        None,
        (!information.is_empty()).then_some(information),
        None,
    )
}

fn label_range(source: &str, offset: usize, length: usize) -> Range {
    Range::new(
        position(source, offset),
        position(source, offset.saturating_add(length)),
    )
}

/// LSP positions count UTF-16 units and treat CRLF as one line break.
fn position(source: &str, byte: usize) -> Position {
    let mut byte = byte.min(source.len());
    while !source.is_char_boundary(byte) {
        byte -= 1;
    }
    let mut line = 0;
    let mut character = 0;
    let mut chars = source.char_indices().peekable();
    while let Some((offset, ch)) = chars.next() {
        if offset >= byte {
            break;
        }
        match ch {
            '\r' => {
                if let Some(&(next, '\n')) = chars.peek() {
                    // A byte position between CR and LF denotes the preceding line end.
                    if byte <= next {
                        break;
                    }
                    chars.next();
                }
                line += 1;
                character = 0;
            }
            '\n' => {
                line += 1;
                character = 0;
            }
            _ => character += ch.len_utf16() as u32,
        }
    }
    Position::new(line, character)
}

```

## candidate/crates/mori/tests/lsp.rs (COMPLETE FILE)

```text
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

```

## candidate/crates/mori/tests/fixtures/lsp/input-session.json (COMPLETE FILE)

```text
[
  {
    "jsonrpc": "2.0",
    "id": 1,
    "method": "initialize",
    "params": {
      "processId": null,
      "rootUri": null,
      "capabilities": {
        "general": {
          "positionEncodings": [
            "utf-16"
          ]
        },
        "textDocument": {
          "publishDiagnostics": {
            "versionSupport": true
          }
        }
      }
    }
  },
  {
    "jsonrpc": "2.0",
    "method": "initialized",
    "params": {}
  },
  {
    "jsonrpc": "2.0",
    "method": "textDocument/didOpen",
    "params": {
      "textDocument": {
        "uri": "untitled:invoice.mori",
        "languageId": "moriarty",
        "version": 1,
        "text": "profile \"moriarty-beta/1\";\nagreement Invoice {\n  /* 🧪 */ const total = shipping + 1;\n}\n"
      }
    }
  },
  {
    "jsonrpc": "2.0",
    "method": "textDocument/didChange",
    "params": {
      "textDocument": {
        "uri": "untitled:invoice.mori",
        "version": 2
      },
      "contentChanges": [
        {
          "text": "profile \"moriarty-beta/1\";\nagreement Invoice {\n  /* 🧪 */ const total = 4 + 1;\n}\n"
        }
      ]
    }
  },
  {
    "jsonrpc": "2.0",
    "method": "textDocument/didClose",
    "params": {
      "textDocument": {
        "uri": "untitled:invoice.mori"
      }
    }
  },
  {
    "jsonrpc": "2.0",
    "method": "textDocument/didOpen",
    "params": {
      "textDocument": {
        "uri": "untitled:core1.mori",
        "languageId": "moriarty",
        "version": 1,
        "text": "language \"moriarty/1\"; agreement Minimal { action tick() {} }\n"
      }
    }
  },
  {
    "jsonrpc": "2.0",
    "method": "textDocument/didClose",
    "params": {
      "textDocument": {
        "uri": "untitled:core1.mori"
      }
    }
  },
  {
    "jsonrpc": "2.0",
    "method": "textDocument/didOpen",
    "params": {
      "textDocument": {
        "uri": "untitled:core2.mori",
        "languageId": "moriarty",
        "version": 1,
        "text": "language \"moriarty/2\"; agreement Minimal { action tick() {} }\n"
      }
    }
  },
  {
    "jsonrpc": "2.0",
    "method": "textDocument/didClose",
    "params": {
      "textDocument": {
        "uri": "untitled:core2.mori"
      }
    }
  },
  {
    "jsonrpc": "2.0",
    "id": 2,
    "method": "shutdown",
    "params": null
  },
  {
    "jsonrpc": "2.0",
    "method": "exit",
    "params": null
  }
]

```

## candidate/crates/mori/tests/fixtures/lsp/expected-session.json (COMPLETE FILE)

```text
{
  "status": "Hand-derived proposed normal protocol oracle, UNRUN, NOT independently custodial or adopted",
  "orderedMessages": [
    {
      "jsonrpc": "2.0",
      "id": 1,
      "result": {
        "capabilities": {
          "positionEncoding": "utf-16",
          "textDocumentSync": {
            "openClose": true,
            "change": 1
          },
          "experimental": {
            "moriarty": {
              "analysis": "local-check-only",
              "sourceProfiles": [
                "moriarty-beta/1",
                "moriarty/1",
                "moriarty/2"
              ]
            }
          }
        },
        "serverInfo": {
          "name": "moriarty-rust",
          "version": "0.1.0"
        }
      }
    },
    {
      "jsonrpc": "2.0",
      "method": "textDocument/publishDiagnostics",
      "params": {
        "uri": "untitled:invoice.mori",
        "diagnostics": [
          {
            "range": {
              "start": {
                "line": 2,
                "character": 25
              },
              "end": {
                "line": 2,
                "character": 33
              }
            },
            "severity": 1,
            "code": "mori::check::unknown_name",
            "source": "moriarty-rust (moriarty-beta/1)",
            "message": "I cannot find a declaration named `shipping`.\n\nDeclare `shipping` above this line. A declaration can only use the names declared above it."
          }
        ],
        "version": 1
      }
    },
    {
      "jsonrpc": "2.0",
      "method": "textDocument/publishDiagnostics",
      "params": {
        "uri": "untitled:invoice.mori",
        "diagnostics": [],
        "version": 2
      }
    },
    {
      "jsonrpc": "2.0",
      "method": "textDocument/publishDiagnostics",
      "params": {
        "uri": "untitled:invoice.mori",
        "diagnostics": []
      }
    },
    {
      "jsonrpc": "2.0",
      "method": "textDocument/publishDiagnostics",
      "params": {
        "uri": "untitled:core1.mori",
        "diagnostics": [],
        "version": 1
      }
    },
    {
      "jsonrpc": "2.0",
      "method": "textDocument/publishDiagnostics",
      "params": {
        "uri": "untitled:core1.mori",
        "diagnostics": []
      }
    },
    {
      "jsonrpc": "2.0",
      "method": "textDocument/publishDiagnostics",
      "params": {
        "uri": "untitled:core2.mori",
        "diagnostics": [],
        "version": 1
      }
    },
    {
      "jsonrpc": "2.0",
      "method": "textDocument/publishDiagnostics",
      "params": {
        "uri": "untitled:core2.mori",
        "diagnostics": []
      }
    },
    {
      "jsonrpc": "2.0",
      "id": 2,
      "result": null
    }
  ],
  "processExit": 0,
  "stderrUtf8": "",
  "stdoutRule": "Exactly nine Content-Length framed UTF-8 JSON objects in this order; complete semantic JSON equality, no extra messages/status bytes; byte serialization/member order not independently frozen."
}

```

## public-dependencies/lsp-server-0.10.0/src/msg.rs (COMPLETE FILE)

```text
use std::{
    fmt,
    io::{self, BufRead, Write},
};

use serde::de::DeserializeOwned;
use serde_derive::{Deserialize, Serialize};

use crate::error::ExtractError;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Message {
    Request(Request),
    Response(Response),
    Notification(Notification),
}

impl From<Request> for Message {
    fn from(request: Request) -> Message {
        Message::Request(request)
    }
}

impl From<Response> for Message {
    fn from(response: Response) -> Message {
        Message::Response(response)
    }
}

impl From<Notification> for Message {
    fn from(notification: Notification) -> Message {
        Message::Notification(notification)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(transparent)]
pub struct RequestId(IdRepr);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(untagged)]
enum IdRepr {
    I32(i32),
    String(String),
}

impl From<i32> for RequestId {
    fn from(id: i32) -> RequestId {
        RequestId(IdRepr::I32(id))
    }
}

impl From<String> for RequestId {
    fn from(id: String) -> RequestId {
        RequestId(IdRepr::String(id))
    }
}

impl fmt::Display for RequestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            IdRepr::I32(it) => fmt::Display::fmt(it, f),
            // Use debug here, to make it clear that `92` and `"92"` are
            // different, and to reduce WTF factor if the sever uses `" "` as an
            // ID.
            IdRepr::String(it) => fmt::Debug::fmt(it, f),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Request {
    pub id: RequestId,
    pub method: String,
    #[serde(default = "serde_json::Value::default")]
    #[serde(skip_serializing_if = "serde_json::Value::is_null")]
    pub params: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    // JSON-RPC allows this to be null if we can't find or parse the
    // request id. We fail deserialization in that case, so we just
    // make this field mandatory.
    pub id: RequestId,
    #[serde(flatten, with = "ResponseResult")]
    pub response_result: Result<serde_json::Value, ResponseError>,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Result<serde_json::Value, ResponseError>")]
enum ResponseResult {
    #[serde(rename = "result")]
    Ok(serde_json::Value),
    #[serde(rename = "error")]
    Err(ResponseError),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub data: Option<serde_json::Value>,
}

#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum ErrorCode {
    // Defined by JSON RPC:
    ParseError = -32700,
    InvalidRequest = -32600,
    MethodNotFound = -32601,
    InvalidParams = -32602,
    InternalError = -32603,
    ServerErrorStart = -32099,
    ServerErrorEnd = -32000,

    /// Error code indicating that a server received a notification or
    /// request before the server has received the `initialize` request.
    ServerNotInitialized = -32002,
    UnknownErrorCode = -32001,

    // Defined by the protocol:
    /// The client has canceled a request and a server has detected
    /// the cancel.
    RequestCanceled = -32800,

    /// The server detected that the content of a document got
    /// modified outside normal conditions. A server should
    /// NOT send this error code if it detects a content change
    /// in it unprocessed messages. The result even computed
    /// on an older state might still be useful for the client.
    ///
    /// If a client decides that a result is not of any use anymore
    /// the client should cancel the request.
    ContentModified = -32801,

    /// The server cancelled the request. This error code should
    /// only be used for requests that explicitly support being
    /// server cancellable.
    ///
    /// @since 3.17.0
    ServerCancelled = -32802,

    /// A request failed but it was syntactically correct, e.g the
    /// method name was known and the parameters were valid. The error
    /// message should contain human readable information about why
    /// the request failed.
    ///
    /// @since 3.17.0
    RequestFailed = -32803,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Notification {
    pub method: String,
    #[serde(default = "serde_json::Value::default")]
    #[serde(skip_serializing_if = "serde_json::Value::is_null")]
    pub params: serde_json::Value,
}

fn invalid_data(error: impl Into<Box<dyn std::error::Error + Send + Sync>>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error)
}

macro_rules! invalid_data {
    ($($tt:tt)*) => (invalid_data(format!($($tt)*)))
}

impl Message {
    pub fn read(r: &mut impl BufRead) -> io::Result<Option<Message>> {
        Message::_read(r)
    }
    fn _read(r: &mut dyn BufRead) -> io::Result<Option<Message>> {
        let text = match read_msg_text(r)? {
            None => return Ok(None),
            Some(text) => text,
        };

        let msg = match serde_json::from_str(&text) {
            Ok(msg) => msg,
            Err(e) => {
                return Err(invalid_data!("malformed LSP payload `{e:?}`: {text:?}"));
            }
        };

        Ok(Some(msg))
    }
    pub fn write(&self, w: &mut impl Write) -> io::Result<()> {
        self._write(w)
    }
    fn _write(&self, w: &mut dyn Write) -> io::Result<()> {
        #[derive(Serialize)]
        struct JsonRpc<'a> {
            jsonrpc: &'static str,
            #[serde(flatten)]
            msg: &'a Message,
        }
        let text = serde_json::to_string(&JsonRpc { jsonrpc: "2.0", msg: self })?;
        write_msg_text(w, &text)
    }
}

impl Response {
    pub fn new_ok<R: serde::Serialize>(id: RequestId, result: R) -> Response {
        Response { id, response_result: Ok(serde_json::to_value(result).unwrap()) }
    }
    pub fn new_err(id: RequestId, code: i32, message: String) -> Response {
        let error = ResponseError { code, message, data: None };
        Response { id, response_result: Err(error) }
    }
}

impl Request {
    pub fn new<P: serde::Serialize>(id: RequestId, method: String, params: P) -> Request {
        Request { id, method, params: serde_json::to_value(params).unwrap() }
    }
    pub fn extract<P: DeserializeOwned>(
        self,
        method: &str,
    ) -> Result<(RequestId, P), ExtractError<Request>> {
        if self.method != method {
            return Err(ExtractError::MethodMismatch(self));
        }
        match serde_json::from_value(self.params) {
            Ok(params) => Ok((self.id, params)),
            Err(error) => Err(ExtractError::JsonError { method: self.method, error }),
        }
    }

    pub(crate) fn is_shutdown(&self) -> bool {
        self.method == "shutdown"
    }
    pub(crate) fn is_initialize(&self) -> bool {
        self.method == "initialize"
    }
}

impl Notification {
    pub fn new(method: String, params: impl serde::Serialize) -> Notification {
        Notification { method, params: serde_json::to_value(params).unwrap() }
    }
    pub fn extract<P: DeserializeOwned>(
        self,
        method: &str,
    ) -> Result<P, ExtractError<Notification>> {
        if self.method != method {
            return Err(ExtractError::MethodMismatch(self));
        }
        match serde_json::from_value(self.params) {
            Ok(params) => Ok(params),
            Err(error) => Err(ExtractError::JsonError { method: self.method, error }),
        }
    }
    pub(crate) fn is_exit(&self) -> bool {
        self.method == "exit"
    }
    pub(crate) fn is_initialized(&self) -> bool {
        self.method == "initialized"
    }
}

fn read_msg_text(inp: &mut dyn BufRead) -> io::Result<Option<String>> {
    let mut size = None;
    let mut buf = String::new();
    loop {
        buf.clear();
        if inp.read_line(&mut buf)? == 0 {
            return Ok(None);
        }
        if !buf.ends_with("\r\n") {
            return Err(invalid_data!("malformed header: {:?}", buf));
        }
        let buf = &buf[..buf.len() - 2];
        if buf.is_empty() {
            break;
        }
        let mut parts = buf.splitn(2, ": ");
        let header_name = parts.next().unwrap();
        let header_value =
            parts.next().ok_or_else(|| invalid_data!("malformed header: {:?}", buf))?;
        if header_name.eq_ignore_ascii_case("Content-Length") {
            size = Some(header_value.parse::<usize>().map_err(invalid_data)?);
        }
    }
    let size: usize = size.ok_or_else(|| invalid_data!("no Content-Length"))?;
    let mut buf = buf.into_bytes();
    buf.resize(size, 0);
    inp.read_exact(&mut buf)?;
    let buf = String::from_utf8(buf).map_err(invalid_data)?;
    log::debug!("< {buf}");
    Ok(Some(buf))
}

fn write_msg_text(out: &mut dyn Write, msg: &str) -> io::Result<()> {
    log::debug!("> {msg}");
    write!(out, "Content-Length: {}\r\n\r\n", msg.len())?;
    out.write_all(msg.as_bytes())?;
    out.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Message, Notification, Request, RequestId, Response};

    #[test]
    fn shutdown_with_explicit_null() {
        let text = r#"{"jsonrpc": "2.0","id": 3,"method": "shutdown", "params": null }"#;
        let msg: Message = serde_json::from_str(text).unwrap();

        assert!(
            matches!(msg, Message::Request(req) if req.id == 3.into() && req.method == "shutdown")
        );
    }

    #[test]
    fn shutdown_with_no_params() {
        let text = r#"{"jsonrpc": "2.0","id": 3,"method": "shutdown"}"#;
        let msg: Message = serde_json::from_str(text).unwrap();

        assert!(
            matches!(msg, Message::Request(req) if req.id == 3.into() && req.method == "shutdown")
        );
    }

    #[test]
    fn notification_with_explicit_null() {
        let text = r#"{"jsonrpc": "2.0","method": "exit", "params": null }"#;
        let msg: Message = serde_json::from_str(text).unwrap();

        assert!(matches!(msg, Message::Notification(not) if not.method == "exit"));
    }

    #[test]
    fn notification_with_no_params() {
        let text = r#"{"jsonrpc": "2.0","method": "exit"}"#;
        let msg: Message = serde_json::from_str(text).unwrap();

        assert!(matches!(msg, Message::Notification(not) if not.method == "exit"));
    }

    #[test]
    fn serialize_request_with_null_params() {
        let msg = Message::Request(Request {
            id: RequestId::from(3),
            method: "shutdown".into(),
            params: serde_json::Value::Null,
        });
        let serialized = serde_json::to_string(&msg).unwrap();

        assert_eq!(r#"{"id":3,"method":"shutdown"}"#, serialized);
    }

    #[test]
    fn serialize_notification_with_null_params() {
        let msg = Message::Notification(Notification {
            method: "exit".into(),
            params: serde_json::Value::Null,
        });
        let serialized = serde_json::to_string(&msg).unwrap();

        assert_eq!(r#"{"method":"exit"}"#, serialized);
    }

    #[test]
    fn serialize_ok_response() {
        let msg = Message::Response(Response::new_ok(RequestId::from(3), "success"));
        let serialized = serde_json::to_string(&msg).unwrap();

        assert_eq!(r#"{"id":3,"result":"success"}"#, serialized);
    }

    #[test]
    fn serialize_err_response() {
        let msg = Message::Response(Response::new_err(
            RequestId::from(3),
            -32600,
            String::from("bad response message"),
        ));
        let serialized = serde_json::to_string(&msg).unwrap();

        assert_eq!(
            r#"{"id":3,"error":{"code":-32600,"message":"bad response message"}}"#,
            serialized
        );
    }
}

```

## source/crates/mori/src/commands/check.rs (COMPLETE FILE)

```text
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use mori_diagnostics::{MoriDiagnostic, NamedSource};

use crate::json::CheckReport;
use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Source file
    pub file: PathBuf,

    /// Print the result as JSON
    #[arg(long)]
    pub json: bool,
}

/// Checks syntax, names, values, types, declarations, calls and intents.
pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    let name = args.file.display().to_string();
    let source = super::core_io::read_source(&args.file)?;

    if !args.json {
        shell.status("Checking", &name);
    }
    let started = Instant::now();
    if mori_lexer::first_token_text(&source).is_ok_and(|token| token == "language") {
        let staged = mori_core::language(&source).is_ok_and(|language| language == "moriarty/2");
        let result = if staged {
            mori_core::stage::load(&source).map(|program| program.program().clone())
        } else {
            mori_core::parse(&source)
        };
        if args.json {
            let report = match &result {
                Ok(program) => {
                    serde_json::json!({"status":"accepted", "file":name,"source_hash":mori_core::intent::hash_bytes(source.as_bytes()),"language":program.language,"agreement":program.name,"actions":program.actions,"diagnostics":[]})
                }
                Err(error) => {
                    serde_json::json!({"status":"rejected","file":name,"diagnostics":[error]})
                }
            };
            if staged {
                super::simulate::print_stage_report(report)?;
            } else {
                super::core_io::print(&report)?;
            }
        } else {
            match &result {
                Ok(_) => shell.status("Finished", "bounded core check"),
                Err(error) => shell.error(error),
            }
        }
        return Ok(if result.is_ok() {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        });
    }
    let (accepted, report) = match mori_parser::parse(&source) {
        Err(error) => {
            let diagnostics = vec![error];
            let report = args
                .json
                .then(|| CheckReport::new(name.clone(), &source, None, &diagnostics));
            (
                finish(shell, &name, &source, diagnostics, args.json),
                report,
            )
        }
        Ok(ast) => {
            if !args.json {
                shell.verbose_status(
                    "Parsed",
                    format!("{} tokens, {} nodes", ast.tokens.len(), ast.nodes.len()),
                );
            }
            let result = mori_checker::check(&ast);
            let report = args.json.then(|| {
                CheckReport::new(
                    name.clone(),
                    &source,
                    Some(&result.checked),
                    &result.diagnostics,
                )
            });
            (
                finish(shell, &name, &source, result.diagnostics, args.json),
                report,
            )
        }
    };

    if let Some(report) = report {
        super::core_io::print(&report)?;
    } else if accepted {
        shell.status(
            "Finished",
            format!("check in {:.2}s", started.elapsed().as_secs_f64()),
        );
    }
    Ok(if accepted {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

/// Renders the diagnostics, unless the report is JSON. True when there are none.
fn finish(
    shell: &mut Shell,
    name: &str,
    source: &str,
    diagnostics: Vec<MoriDiagnostic>,
    json: bool,
) -> bool {
    if diagnostics.is_empty() {
        return true;
    }
    if !json {
        let count = diagnostics.len();
        for diagnostic in diagnostics {
            shell.report(&diagnostic.with_source_code(NamedSource::new(
                crate::display::text(name),
                source.to_owned(),
            )));
        }
        let plural = if count == 1 { "" } else { "s" };
        shell.error(format!(
            "could not check `{name}` due to {count} previous error{plural}"
        ));
    }
    false
}

```

## api-compatibility/API-COMPATIBILITY.md (COMPLETE FILE)

```text
# Source-only API compatibility

No API-definition or signature mismatch found in the inspected snapshot. Source-only, not a compiler result.

Seven current I candidate paths were snapshotted with repeated byte checks. Full adopted package text and original custody pins are preserved. This table binds only those exact bytes; it is not a compile-pass claim.

| API | Candidate use | Exact public definition | Source conclusion |
|---|---|---|---|
| API-01 lsp_server root imports | lsp.rs:5 | lsp-server-0.10.0/src/lib.rs:19–28 | ErrorCode, Message, Notification, Request and Response are public crate-root re-exports of msg types. |
| API-02 Message variants | lsp.rs:65; lsp.rs:71,263; lsp.rs:67,73 | lsp-server-0.10.0/src/msg.rs:11–17 | The public enum has precisely the Request(Request), Response(Response) and Notification(Notification) variants used by dispatch. |
| API-03 Message::read | lsp.rs:27 | lsp-server-0.10.0/src/msg.rs:172–175 | Signature is read(&mut impl BufRead) -> io::Result<Option<Message>>. The candidate passes &mut StdinLock and handles Result then Option. |
| API-04 Message::write | lsp.rs:68,267 | lsp-server-0.10.0/src/msg.rs:191–203 | Signature is write(&self, &mut impl Write) -> io::Result<()>, matching each candidate output helper and error mapping. |
| API-05 Request public fields | lsp.rs:79,118; lsp.rs:87,119; lsp.rs:82,89,106,116,121,128,131 | lsp-server-0.10.0/src/msg.rs:72–80 | method is String; params is serde_json::Value; id is RequestId. Response constructors accept that same RequestId. |
| API-06 Response::new_ok | lsp.rs:106,128 | lsp-server-0.10.0/src/msg.rs:206–209 | new_ok<R: serde::Serialize>(RequestId, R) -> Response accepts InitializeResult (derived Serialize) and unit (). |
| API-07 Response::new_err | lsp.rs:81,88,116,120,130 | lsp-server-0.10.0/src/msg.rs:210–214 | new_err(RequestId, i32, String) -> Response matches the moved id, cast enum discriminant and inferred String conversion. |
| API-08 ErrorCode variants/casts | lsp.rs:83,112; lsp.rs:90,122; lsp.rs:110; lsp.rs:132 | lsp-server-0.10.0/src/msg.rs:108–126 | All four names are real unit enum variants; integer casts target the required i32 code parameter. The non_exhaustive marker does not prevent naming these variants. |
| API-09 Notification public fields and constructor | lsp.rs:142,149,156; lsp.rs:159,182,215; lsp.rs:263 | lsp-server-0.10.0/src/msg.rs:156–163; lsp-server-0.10.0/src/msg.rs:241–244 | method is String, params serde_json::Value, and new(String, impl serde::Serialize) accepts the typed publish parameters. |
| API-10 gen-lsp-types crate-root exports | lsp.rs:6; diagnostics.rs:1 | gen-lsp-types-0.11.0/src/lib.rs:1–4; gen-lsp-types-0.11.0/src/generated.rs:19–33 | All generated structures/enums/common types are public root re-exports. Workspace alias lsp-types maps to package gen-lsp-types, whose explicit lib target is gen_lsp_types; Cargo dependency renaming supplies the lsp_types external name. Condition: Cargo alias selection has not been executed; manifests are preserved exactly. |
| API-11 Uri and HashMap trait bounds | lsp.rs:53; lsp.rs:172; diagnostics.rs:62 | gen-lsp-types-0.11.0/src/generated/common.rs:25–40 | Under the adopted no-URI-feature configuration Uri(pub String) derives Clone, Eq and Hash, satisfying clone and HashMap key use; it also derives Serialize/Deserialize. Condition: The exact source cfg is not(url) and not(fluent-uri). No Cargo feature-unification result is asserted. |
| API-12 InitializeParams + capabilities | lsp.rs:87; lsp.rs:95 | gen-lsp-types-0.11.0/src/generated/structures.rs:2093–2095; gen-lsp-types-0.11.0/src/generated/structures.rs:2131–2144 | InitializeParams derives Deserialize and exposes public capabilities: ClientCapabilities. Candidate does not construct deprecated initialize fields. |
| API-13 Client capability traversal | lsp.rs:96,163,200,204,207,219; lsp.rs:98 | gen-lsp-types-0.11.0/src/generated/structures.rs:7820–7829; gen-lsp-types-0.11.0/src/generated/structures.rs:8341–8347; gen-lsp-types-0.11.0/src/generated/structures.rs:8424–8428 | ClientCapabilities.text_document is Option<TextDocumentClientCapabilities>; that type has public publish_diagnostics: Option<PublishDiagnosticsClientCapabilities>. The as_ref/and_then chain matches these types. |
| API-14 Publish diagnostics client flags | lsp.rs:100; lsp.rs:103 | gen-lsp-types-0.11.0/src/generated/structures.rs:9949–9961; gen-lsp-types-0.11.0/src/generated/structures.rs:10812–10818 | version_support is Option<bool>; diagnostics_capabilities is a flattened DiagnosticsCapabilities containing related_information: Option<bool>. Candidate uses the real nested Rust field rather than assuming serde flatten creates a direct Rust field. |
| API-15 DidOpenTextDocumentParams | lsp.rs:159; lsp.rs:163 | gen-lsp-types-0.11.0/src/generated/structures.rs:2329–2335 | The type derives Deserialize and has text_document: TextDocumentItem. |
| API-16 TextDocumentItem fields | lsp.rs:164,168,170,219; lsp.rs:168,175,200,208,209; lsp.rs:169,174,204,207,210 | gen-lsp-types-0.11.0/src/generated/structures.rs:5648–5662 | uri is Uri, text String, version i32; these match diagnostics borrow, owned HashMap key/document fields and Option<i32> diagnostic versions. language_id exists but is not read by this candidate. |
| API-17 DidChangeTextDocumentParams | lsp.rs:182; lsp.rs:187 | gen-lsp-types-0.11.0/src/generated/structures.rs:2358–2377 | The type derives Deserialize; content_changes is Vec<TextDocumentContentChangeEvent>, and text_document is VersionedTextDocumentIdentifier. |
| API-18 VersionedTextDocumentIdentifier nesting | lsp.rs:200; lsp.rs:204,207 | gen-lsp-types-0.11.0/src/generated/structures.rs:6191–6199; gen-lsp-types-0.11.0/src/generated/structures.rs:4644–4650 | version is i32 and the flattened Rust member is text_document_identifier: TextDocumentIdentifier, whose public uri: Uri matches candidate access. |
| API-19 TextDocumentContentChangeEvent full variant | lsp.rs:188 | gen-lsp-types-0.11.0/src/generated/enum_ors.rs:1688–1705 | The exact named tuple variant exists alongside a partial variant. The let-else pattern binds its whole-document payload. |
| API-20 Whole-document change payload | lsp.rs:195 | gen-lsp-types-0.11.0/src/generated/structures.rs:7970–7982 | TextDocumentContentChangeWholeDocument.text is String, matching replacement document text. |
| API-21 DidCloseTextDocumentParams | lsp.rs:215; lsp.rs:219 | gen-lsp-types-0.11.0/src/generated/structures.rs:2414–2425; gen-lsp-types-0.11.0/src/generated/structures.rs:4644–4650 | The type derives Deserialize and has text_document: TextDocumentIdentifier with uri: Uri. |
| API-22 InitializeResult literal | lsp.rs:230; lsp.rs:230,231 | gen-lsp-types-0.11.0/src/generated/structures.rs:2174–2194 | InitializeResult derives Serialize and its two public fields are capabilities: ServerCapabilities and server_info: Option<ServerInfo>; the candidate initializes both. |
| API-23 ServerCapabilities fields/Default | lsp.rs:232; lsp.rs:233; lsp.rs:234; lsp.rs:242 | gen-lsp-types-0.11.0/src/generated/structures.rs:5942–5962; gen-lsp-types-0.11.0/src/generated/structures.rs:6083–6089; gen-lsp-types-0.11.0/src/generated/type_aliases.rs:15–21 | ServerCapabilities derives Default; the three explicitly set fields have the expected Option types. LspAny aliases serde_json::Value, matching json!(...). Remaining fields come from Default. |
| API-24 PositionEncodingKind::UTF16 | lsp.rs:233 | gen-lsp-types-0.11.0/src/generated/enumerations.rs:1931–1943; gen-lsp-types-0.11.0/src/generated/enumerations.rs:1954–1961 | UTF16 is a real enum variant; its Serialize conversion maps to the string utf-16. No dependency constant named UTF_16 is assumed. |
| API-25 TextDocumentSyncOptions fields/Default | lsp.rs:235; lsp.rs:236; lsp.rs:237 | gen-lsp-types-0.11.0/src/generated/structures.rs:7868–7891 | Options derives Default and Copy; open_close: Option<bool>, change: Option<TextDocumentSyncKind> match candidate fields. Other fields default to None. |
| API-26 TextDocumentSyncOptions -> TextDocumentSync conversion | lsp.rs:84,91,123,133,240,251,264 | gen-lsp-types-0.11.0/src/generated/enum_ors.rs:1746–1760; gen-lsp-types-0.11.0/src/generated/structures.rs:5961–5962 | Explicit From<TextDocumentSyncOptions> for TextDocumentSync makes the .into() value fit ServerCapabilities.text_document_sync. The same line anchor also sees unrelated ordinary string conversions; this row addresses the options conversion only. |
| API-27 TextDocumentSyncKind::Full | lsp.rs:237 | gen-lsp-types-0.11.0/src/generated/enumerations.rs:888–913 | Full exists and its Serialize conversion is numeric 1, matching the author-frozen initialize expectation. |
| API-28 ServerInfo literal | lsp.rs:250 | gen-lsp-types-0.11.0/src/generated/structures.rs:6171–6188 | Public name: String and version: Option<String> are all fields. Its type name was added in the 3.18 metamodel but the candidate emits only the existing name/version wire fields. |
| API-29 PublishDiagnosticsParams::new | lsp.rs:265 | gen-lsp-types-0.11.0/src/generated/structures.rs:2536–2558 | The exact parameter order is (uri: Uri, version: Option<i32>, diagnostics: Vec<Diagnostic>) -> Self, which matches the candidate call and implements Serialize for Notification::new. |
| API-30 Diagnostic::new nine-argument constructor | diagnostics.rs:33,83 | gen-lsp-types-0.11.0/src/generated/structures.rs:6265–6311; gen-lsp-types-0.11.0/src/generated/structures.rs:6312–6337 | Both core and legacy calls supply exactly (Range, Option<DiagnosticSeverity>, Option<Code>, Option<CodeDescription>, Option<String>, Message, Option<Vec<DiagnosticTag>>, Option<Vec<DiagnosticRelatedInformation>>, Option<LspAny>). This is the real generated signature, not the older original-lsp-types constructor. |
| API-31 Diagnostic code string conversions | diagnostics.rs:36; diagnostics.rs:86 | gen-lsp-types-0.11.0/src/generated/enum_ors.rs:135–154 | Code has From<String> and From<&str>. Actual mori_core::Error.code is String, and the legacy conversion also produces String; both .into() expressions match Code. |
| API-32 Diagnostic message string conversion | diagnostics.rs:39; diagnostics.rs:39,89 | gen-lsp-types-0.11.0/src/generated/enum_ors.rs:1182–1196 | The generated diagnostic Message is an untagged String/MarkupContent enum with From<String>; both candidate calls select the String conversion. This enum is distinct from lsp_server::Message and is inferred from Diagnostic::new, so no import-name collision occurs. |
| API-33 DiagnosticSeverity variants | diagnostics.rs:35,81; diagnostics.rs:79; diagnostics.rs:80 | gen-lsp-types-0.11.0/src/generated/enumerations.rs:2078–2102 | The three named variants exist; Serialize maps Error=1, Warning=2, Information=3. |
| API-34 DiagnosticRelatedInformation::new | diagnostics.rs:60 | gen-lsp-types-0.11.0/src/generated/structures.rs:8000–8016 | Signature (Location, String) -> Self matches the location and text.to_owned() arguments. |
| API-35 Location::new | diagnostics.rs:61 | gen-lsp-types-0.11.0/src/generated/structures.rs:32–45 | Signature (Uri, Range) -> Self matches cloned URI and label range. |
| API-36 Range::new | diagnostics.rs:34,50,97 | gen-lsp-types-0.11.0/src/generated/structures.rs:4552–4563 | Signature (Position, Position) -> Self matches both zero/default span construction and label endpoints. |
| API-37 Position::new and Copy | diagnostics.rs:50,135; diagnostics.rs:34 | gen-lsp-types-0.11.0/src/generated/structures.rs:4744–4772 | Position has public line/character: u32 and new(u32,u32). It derives Copy, so the same local at can be passed twice without a move error. |

No source patch is justified by this bounded API comparison. Exact consumer anchors, complete excerpts and all limitations are in `API-COMPATIBILITY.json`; complete source bytes and original paths/hashes are in `INPUT-SOURCE-PINS.json`.

No Cargo/resolver/lock/cache/worktree changes, acquired-source execution, server execution, dependency version selection, provider dispatch or security/resource judgments occurred. Actual compiler checking and resolved feature selection remain separate root steps.

```

## independent-L12/CUSTODY.json (COMPLETE FILE)

```text
{
  "status": "frozen before any consumer execution by this lane",
  "lane": "L12 normal LSP diagnostics",
  "source_commit": "74a909d69639df8823a527a4d608a7516ccd69a2",
  "prior_exposure": "Same custodian previously accidentally read starter DevEx generated financial expected literals in init.rs at 9a140f7; preserved separate contamination record. That packet is unrelated to these newly authored LSP inputs. No independent financial oracle claim is made.",
  "this_lane_exclusions": "No candidate lsp.rs diagnostics implementation, candidate tests or expected-session fixture were read; no protocol/server/parser/compiler/SDK execution; no provider dispatch; no security or hostile test judgments.",
  "allowed_actions": "Read-only git source inspection, public LSP3.17 fetch through Scrapling, Python standard-library input/position/hash serialization, writes confined to this new directory.",
  "expected_position_method": "Manually derived token prefix columns 16 and 45 were cross-checked with standard-library UTF16 byte counting; no Moriarty parser or checker imported or executed."
}

```

## independent-L12/INDEPENDENT-EXPECTATIONS.json (COMPLETE FILE)

```text
{
  "status": "independently derived normal diagnostic oracle; NOT execution evidence",
  "frozen_at": "2026-10-18T11:31:30.599824+00:00",
  "source_commit": "74a909d69639df8823a527a4d608a7516ccd69a2",
  "position_premise": "LSP3.17 utf-16 position encoding, either explicitly negotiated or default; zero-based lines/characters; CRLF counts as one line break; half-open ranges.",
  "diagnostic_assertion_scope": "Exact semantic checker message, stable code, severity Error=1 and complete label-token range. Optional LSP fields, help text placement, source identifier and framing are not inferred from the candidate. If an adapter appends help into message, compare separately and record divergence rather than silently changing this frozen expected message.",
  "cases": [
    {
      "input": "beta-valid.mori",
      "input_sha256": "2e379e38d3b1c172a30d0aa66128342c26af1f86b7337aa30e2a86ea20884f8c",
      "profile": "moriarty-beta/1",
      "expected_diagnostics": [],
      "semantic_basis": "Valid beta arithmetic with an earlier scalar declaration."
    },
    {
      "input": "beta-unknown-name.mori",
      "input_sha256": "7e43d653b12d142d0aec7dfc1dbcfa7803f1ffc1007f0521332cdeb3f296f586",
      "profile": "moriarty-beta/1",
      "expected_diagnostics": [
        {
          "message": "I cannot find a declaration named `missing`.",
          "code": "mori::check::unknown_name",
          "severity": 1,
          "range": {
            "start": {
              "line": 3,
              "character": 16
            },
            "end": {
              "line": 3,
              "character": 23
            }
          }
        }
      ],
      "source_byte_span": {
        "start": 87,
        "end_exclusive": 94,
        "token": "missing"
      },
      "position_derivation": {
        "line_prefix": "  const twice = ",
        "prefix_unicode_codepoints": 16,
        "prefix_utf8_bytes": 16,
        "prefix_utf16_units": 16,
        "range_end_exclusive": true
      },
      "semantic_basis": "Names collection finds no declaration missing; ordered checker resolves missing once, adds unknown_name once; failed const does not add a second diagnostic. Source label is the complete identifier token."
    },
    {
      "input": "beta-unicode-crlf-unknown-name.mori",
      "input_sha256": "14e478bfa4ae4a44afec89b356640dbb8d44f365f034fe408651c9e8dd44abee",
      "profile": "moriarty-beta/1",
      "expected_diagnostics": [
        {
          "message": "I cannot find a declaration named `missing`.",
          "code": "mori::check::unknown_name",
          "severity": 1,
          "range": {
            "start": {
              "line": 2,
              "character": 45
            },
            "end": {
              "line": 2,
              "character": 52
            }
          }
        }
      ],
      "source_byte_span": {
        "start": 97,
        "end_exclusive": 104,
        "token": "missing"
      },
      "position_derivation": {
        "line_prefix": "  const amount = 10; /* \ud83d\ude00\u00e9 */ const twice = ",
        "prefix_unicode_codepoints": 44,
        "prefix_utf8_bytes": 48,
        "prefix_utf16_units": 45,
        "range_end_exclusive": true
      },
      "semantic_basis": "Names collection finds no declaration missing; ordered checker resolves missing once, adds unknown_name once; failed const does not add a second diagnostic. Source label is the complete identifier token."
    },
    {
      "input": "beta-unicode-crlf-valid.mori",
      "input_sha256": "82710fbbed702a5682f02a3a4c8eaf52f44038f1a0cc50e78a7d7e8d46776ec6",
      "profile": "moriarty-beta/1",
      "expected_diagnostics": [],
      "semantic_basis": "Same normal beta arithmetic, with a valid Unicode block comment and CRLF line breaks."
    },
    {
      "input": "core1-valid.mori",
      "input_sha256": "d729658a1531a7102320e7de5e78d0e9a400ab9b67028699022f36e112a0a9d4",
      "profile": "moriarty/1",
      "expected_diagnostics": [],
      "semantic_basis": "Core1 parser and finite validator admit declared UInt state/input, UInt arithmetic, Bool predicate and matching state assignment."
    },
    {
      "input": "core2-valid.mori",
      "input_sha256": "a4e3ea45c3a3f85e4ee53218ffb6e0410380a551b7c0750891cbff5347e0d301",
      "profile": "moriarty/2",
      "expected_diagnostics": [],
      "semantic_basis": "Core2 parser and staged validator additionally admit unique approved await with nonshadowing Bool binder ok, Bool guard ok, then preserved UInt state/input assignment."
    }
  ],
  "unasserted": [
    "Transport request IDs, byte framing, server capability shape and notification count/timing/order across different URIs",
    "Optional diagnostic source, relatedInformation, tags, data, codeDescription and version presence",
    "Non-UTF16 negotiated positions",
    "Whether the candidate currently routes Core1/Core2; these are expectations for a consumer claiming the same supported checks as public check.rs",
    "Compilation, simulation, proofs, ledger outcomes or security judgments"
  ]
}

```

## independent-L12/NORMAL-EVENT-EXPECTATIONS.json (COMPLETE FILE)

```text
{
  "status": "conceptual event predicates only; no protocol was run",
  "premises": [
    "Client uses UTF16 positions and full document changes.",
    "Each event is checked after its normal diagnostic recomputation has completed; no millisecond threshold is asserted.",
    "The close-clearing predicate applies to a single-file/open-document diagnostics service. LSP3.17 explicitly distinguishes project-system diagnostics, which may persist after close; this packet makes no universal close-clearing claim for project-wide servers.",
    "For a stale error followed by valid content, server must publish an empty list; prior client list is replaced rather than merged."
  ],
  "sequences": [
    {
      "name": "normal-change-clears-former-error",
      "uri": "file:///independent-normal/beta.mori",
      "steps": [
        {
          "event": "didOpen",
          "version": 1,
          "text_file": "beta-unknown-name.mori",
          "expected_case": "beta-unknown-name.mori"
        },
        {
          "event": "didChange",
          "version": 2,
          "change_kind": "full-document",
          "text_file": "beta-valid.mori",
          "expected_diagnostics": []
        },
        {
          "event": "didClose",
          "expected_diagnostics": []
        }
      ]
    },
    {
      "name": "unicode-crlf-change-clears-former-error",
      "uri": "file:///independent-normal/unicode.mori",
      "steps": [
        {
          "event": "didOpen",
          "version": 1,
          "text_file": "beta-unicode-crlf-unknown-name.mori",
          "expected_case": "beta-unicode-crlf-unknown-name.mori"
        },
        {
          "event": "didChange",
          "version": 2,
          "change_kind": "full-document",
          "text_file": "beta-unicode-crlf-valid.mori",
          "expected_diagnostics": []
        },
        {
          "event": "didClose",
          "expected_diagnostics": []
        }
      ]
    },
    {
      "name": "normal-close-clears-open-file-error",
      "uri": "file:///independent-normal/close.mori",
      "steps": [
        {
          "event": "didOpen",
          "version": 1,
          "text_file": "beta-unknown-name.mori",
          "expected_case": "beta-unknown-name.mori"
        },
        {
          "event": "didClose",
          "expected_diagnostics": []
        }
      ]
    }
  ]
}

```

## independent-L12/beta-unicode-crlf-unknown-name.mori (COMPLETE FILE)

```text
profile "moriarty-beta/1";
agreement Unicode {
  const amount = 10; /* 😀é */ const twice = missing;
}

```

## independent-L12/beta-unicode-crlf-valid.mori (COMPLETE FILE)

```text
profile "moriarty-beta/1";
agreement Unicode {
  const amount = 10; /* 😀é */ const twice = amount * 2;
}

```

## independent-L12/beta-unknown-name.mori (COMPLETE FILE)

```text
profile "moriarty-beta/1";
agreement Arithmetic {
  const amount = 10;
  const twice = missing;
}

```

## independent-L12/beta-valid.mori (COMPLETE FILE)

```text
profile "moriarty-beta/1";
agreement Arithmetic {
  const amount = 10;
  const twice = amount * 2;
}

```

## independent-L12/core1-valid.mori (COMPLETE FILE)

```text
language "moriarty/1";
agreement CoreOne {
  state count: UInt = 0;
  action add(amount: UInt) {
    requires amount > 0;
    next count = count + amount;
  }
}

```

## independent-L12/core2-valid.mori (COMPLETE FILE)

```text
language "moriarty/2";
agreement CoreTwo {
  state count: UInt = 0;
  action add(amount: UInt) {
    requires amount > 0;
    await approved(ok: Bool) when ok;
    next count = count + amount;
  }
}

```

## design-reviews/doctorwatson-rust-lsp-design-protocol-ordinary-20261018-a2/WHOLE-SCHEMA-VALID-CANDIDATES.json (COMPLETE FILE)

```text
[
  {
    "actualSourceFacts": {
      "currentCargoLockVersions": {
        "crossbeam-channel": [],
        "crossbeam-utils": [
          "0.8.23"
        ],
        "gen-lsp-types": [],
        "log": [
          "0.4.34"
        ],
        "lsp-server": [],
        "lsp-types": [],
        "serde": [
          "1.0.229"
        ],
        "serde_derive": [
          "1.0.229"
        ],
        "serde_json": [
          "1.0.151"
        ]
      },
      "existingConsumers": {
        "routes": [
          "language first token + moriarty/2 -> mori_core::stage::load",
          "other language first token -> mori_core::parse (includes legacy validate)",
          "otherwise -> mori_parser::parse then mori_checker::check"
        ],
        "rustCheck": "source/crates/mori/src/commands/check.rs",
        "rustLsp": "source/crates/mori/src/commands/lsp.rs run -> todo!",
        "rustMcp": "source/crates/mori/src/commands/mcp.rs run -> todo!"
      },
      "literalTokenDeclaration": {
        "coreCaller": "source/crates/mori-core/src/parser.rs -> mori_lexer::lex",
        "file": "source/crates/mori-lexer/src/lib.rs",
        "name": "MAX_TOKENS",
        "value": 8192
      },
      "normalPredicate": "Actual mori lsp binary,11 input messages ->9 ordered complete semantic JSON messages, beta unknown-name UTF16 line2 chars25..33, full edit/close clear, ordinary Core1/Core2 open/close, shutdown/exit0 and empty stderr. Proposed hand-derived UNRUN expectation; not independent finance or execution evidence.",
      "normativeEarsPaths": [
        "source/openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md",
        "source/openspec/changes/consolidated-language-kernel/traceability.md"
      ],
      "ordinaryEars": {
        "ids": [
          "LSP-N01",
          "LSP-N02",
          "LSP-N03",
          "LSP-N04",
          "LSP-N05"
        ],
        "path": "EARS-PROPOSAL.md",
        "status": "Proposed child clauses, unadopted"
      },
      "originalInputManifestSha256": "a337f8a473c885c188957d1199fbe1a3a8ca669818e43251afa8383700e10028",
      "proposedDirectVersionsNotCurrentLock": {
        "lsp-server": "=0.10.0",
        "lsp-types alias package gen-lsp-types": "=0.11.0, default-features=false, no features"
      },
      "publishedManifestRangesNotResolvedVersions": {
        "gen-lsp-types0.11.0": {
          "optionalFeaturesNotProposed": [
            "url",
            "fluent-uri"
          ],
          "serde": "1.0.228 with derive",
          "serde_json": "1.0.150"
        },
        "lsp-server0.10.0": {
          "crossbeam-channel": "0.5.15",
          "log": "0.4.27",
          "serde": "1.0.219",
          "serde_derive": "1.0.219",
          "serde_json": "1.0.140"
        }
      },
      "repoCommit": "588d811dbbd9e88d22d3ebe90b0544672870caa0",
      "resolvedNewGraph": "UNKNOWN. Manifest ranges are not exact new selected identities. No claim that current log/serde/json pins must be downgraded or replaced.",
      "status": "NOT EXECUTED; NO Cargo resolution; Rust LSP/MCP not implemented in observed input",
      "typeProtocolQualification": "gen-lsp-types0.11.0 generated from3.18; proposed serialization emits only specified3.17 subset, Diagnostic.message plain String. Uri is String wrapper with no optional URI feature."
    },
    "filesRead": [
      "ACTUAL-SOURCE-FACTS.json",
      "PROPOSED-INPUT-SESSION.json",
      "PROPOSED-EXPECTED-SESSION.json",
      "EARS-PROPOSAL.md",
      "source/crates/mori/src/commands/check.rs",
      "source/crates/mori/src/commands/lsp.rs",
      "public-dependencies/source/lsp-server-0.10.0/src/msg.rs",
      "public-dependencies/source/lsp-server-0.10.0/src/stdio.rs",
      "public-dependencies/source/lsp-server-0.10.0/src/lib.rs",
      "public-dependencies/source/gen-lsp-types-0.11.0/src/generated/common.rs",
      "public-dependencies/source/gen-lsp-types-0.11.0/src/generated/enum_ors.rs",
      "public-dependencies/source/gen-lsp-types-0.11.0/src/generated/structures.rs",
      "source/crates/mori-core/src/parser.rs",
      "source/crates/mori-core/src/stage/mod.rs",
      "source/crates/mori-core/src/ir.rs",
      "source/openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md"
    ],
    "findings": [
      "D1 (Transport): In public-dependencies/source/lsp-server-0.10.0/src/msg.rs, Message::read and Message::write (via read_msg_text and write_msg_text) provide complete Content-Length framing and JSON-RPC 2.0 envelope handling directly over locked stdin/stdout in a deterministic synchronous loop for source/crates/mori/src/commands/lsp.rs::run, avoiding the three background threads (LspServerReader, LspServerWriter, LspMessageDropper) and bounded channel handoffs spawned by stdio::stdio_transport in Connection::stdio (public-dependencies/source/lsp-server-0.10.0/src/stdio.rs) as well as bespoke manual framing.",
      "D2 (Types): In public-dependencies/source/gen-lsp-types-0.11.0/src/generated/common.rs, disabling default and optional features defines pub struct Uri(pub String), preserving arbitrary client-supplied document identifiers (such as untitled: URIs in PROPOSED-INPUT-SESSION.json) without external URL parsing, while Diagnostic.message in public-dependencies/source/gen-lsp-types-0.11.0/src/generated/structures.rs and Message::String in public-dependencies/source/gen-lsp-types-0.11.0/src/generated/enum_ors.rs serialize plain JSON strings via #[serde(untagged)] for the specified 3.17 subset.",
      "D3 (Profiles): Reusing the exact dispatch of source/crates/mori/src/commands/check.rs::run (inspecting mori_lexer::first_token_text and mori_core::language to route moriarty/2 to mori_core::stage::load, other language-header inputs to mori_core::parse, and remaining inputs to mori_parser::parse followed by mori_checker::check) keeps mori lsp aligned with mori check per LSP-N02 and UNI-018, mapping MoriDiagnostic labels and mori_core::ir::Error optional offset fields (with explicit zero-width fallback per LSP-N03) under local-check-only capabilities."
    ],
    "limitations": [
      "Review covers ordinary protocol and library design fit only across the supplied static files and bound actualSourceFacts; Cargo dependency resolution, lockfile generation, and binary execution are not performed.",
      "Proposed session transcripts (PROPOSED-INPUT-SESSION.json and PROPOSED-EXPECTED-SESSION.json) are unrun hand-derived protocol predictions, and source/crates/mori/src/commands/lsp.rs::run remains a todo! stub.",
      "Hostile-input, resource, and security requirements remain outside the scope of this ordinary review."
    ],
    "reviewToken": "doctorwatson-rust-lsp-design-protocol-ordinary-20261018-a2-whole-review",
    "reviewedDesignSubjectSha256": "32bf1810058935515e05b4efcabd97b3356f5166ebd7e083979f665cec62c490",
    "selectedProfiles": "same-routing-as-current-Rust-check",
    "selectedTransport": "lsp-server-0.10.0-Message-read-write",
    "selectedTypes": "gen-lsp-types-0.11.0-no-features-3.17-subset",
    "summary": "Approve the proposed ordinary Rust LSP design choices: synchronous stdio framing via lsp-server Message::read and Message::write (D1), gen-lsp-types with no optional features emitting the 3.17 subset and plain String diagnostic messages (D2), and the same three-way profile routing as mori check (D3).",
    "toolAction": "Completing ordinary LSP design review",
    "toolSummary": "Complete ordinary LSP design review",
    "verdict": "approve-proposed-design"
  },
  {
    "actualSourceFacts": {
      "currentCargoLockVersions": {
        "crossbeam-channel": [],
        "crossbeam-utils": [
          "0.8.23"
        ],
        "gen-lsp-types": [],
        "log": [
          "0.4.34"
        ],
        "lsp-server": [],
        "lsp-types": [],
        "serde": [
          "1.0.229"
        ],
        "serde_derive": [
          "1.0.229"
        ],
        "serde_json": [
          "1.0.151"
        ]
      },
      "existingConsumers": {
        "routes": [
          "language first token + moriarty/2 -> mori_core::stage::load",
          "other language first token -> mori_core::parse (includes legacy validate)",
          "otherwise -> mori_parser::parse then mori_checker::check"
        ],
        "rustCheck": "source/crates/mori/src/commands/check.rs",
        "rustLsp": "source/crates/mori/src/commands/lsp.rs run -> todo!",
        "rustMcp": "source/crates/mori/src/commands/mcp.rs run -> todo!"
      },
      "literalTokenDeclaration": {
        "coreCaller": "source/crates/mori-core/src/parser.rs -> mori_lexer::lex",
        "file": "source/crates/mori-lexer/src/lib.rs",
        "name": "MAX_TOKENS",
        "value": 8192
      },
      "normalPredicate": "Actual mori lsp binary,11 input messages ->9 ordered complete semantic JSON messages, beta unknown-name UTF16 line2 chars25..33, full edit/close clear, ordinary Core1/Core2 open/close, shutdown/exit0 and empty stderr. Proposed hand-derived UNRUN expectation; not independent finance or execution evidence.",
      "normativeEarsPaths": [
        "source/openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md",
        "source/openspec/changes/consolidated-language-kernel/traceability.md"
      ],
      "ordinaryEars": {
        "ids": [
          "LSP-N01",
          "LSP-N02",
          "LSP-N03",
          "LSP-N04",
          "LSP-N05"
        ],
        "path": "EARS-PROPOSAL.md",
        "status": "Proposed child clauses, unadopted"
      },
      "originalInputManifestSha256": "a337f8a473c885c188957d1199fbe1a3a8ca669818e43251afa8383700e10028",
      "proposedDirectVersionsNotCurrentLock": {
        "lsp-server": "=0.10.0",
        "lsp-types alias package gen-lsp-types": "=0.11.0, default-features=false, no features"
      },
      "publishedManifestRangesNotResolvedVersions": {
        "gen-lsp-types0.11.0": {
          "optionalFeaturesNotProposed": [
            "url",
            "fluent-uri"
          ],
          "serde": "1.0.228 with derive",
          "serde_json": "1.0.150"
        },
        "lsp-server0.10.0": {
          "crossbeam-channel": "0.5.15",
          "log": "0.4.27",
          "serde": "1.0.219",
          "serde_derive": "1.0.219",
          "serde_json": "1.0.140"
        }
      },
      "repoCommit": "588d811dbbd9e88d22d3ebe90b0544672870caa0",
      "resolvedNewGraph": "UNKNOWN. Manifest ranges are not exact new selected identities. No claim that current log/serde/json pins must be downgraded or replaced.",
      "status": "NOT EXECUTED; NO Cargo resolution; Rust LSP/MCP not implemented in observed input",
      "typeProtocolQualification": "gen-lsp-types0.11.0 generated from3.18; proposed serialization emits only specified3.17 subset, Diagnostic.message plain String. Uri is String wrapper with no optional URI feature."
    },
    "filesRead": [
      "ACTUAL-SOURCE-FACTS.json",
      "PROPOSED-INPUT-SESSION.json",
      "PROPOSED-EXPECTED-SESSION.json",
      "EARS-PROPOSAL.md",
      "source/crates/mori/src/commands/check.rs",
      "source/crates/mori/src/commands/lsp.rs",
      "public-dependencies/source/lsp-server-0.10.0/src/msg.rs",
      "public-dependencies/source/lsp-server-0.10.0/src/stdio.rs",
      "public-dependencies/source/lsp-server-0.10.0/src/lib.rs",
      "public-dependencies/source/gen-lsp-types-0.11.0/src/generated/common.rs",
      "public-dependencies/source/gen-lsp-types-0.11.0/src/generated/enum_ors.rs",
      "public-dependencies/source/gen-lsp-types-0.11.0/src/generated/structures.rs",
      "source/crates/mori-core/src/parser.rs",
      "source/crates/mori-core/src/stage/mod.rs",
      "source/crates/mori-core/src/ir.rs",
      "source/openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md"
    ],
    "findings": [
      "D1 (Transport): In public-dependencies/source/lsp-server-0.10.0/src/msg.rs, Message::read and Message::write (via read_msg_text and write_msg_text) provide complete Content-Length framing and JSON-RPC 2.0 envelope handling directly over locked stdin/stdout in a deterministic synchronous loop for source/crates/mori/src/commands/lsp.rs::run, avoiding the three background threads (LspServerReader, LspServerWriter, LspMessageDropper) and bounded channel handoffs spawned by stdio::stdio_transport in Connection::stdio (public-dependencies/source/lsp-server-0.10.0/src/stdio.rs) as well as bespoke manual framing.",
      "D2 (Types): In public-dependencies/source/gen-lsp-types-0.11.0/src/generated/common.rs, disabling default and optional features defines pub struct Uri(pub String), preserving arbitrary client-supplied document identifiers (such as untitled: URIs in PROPOSED-INPUT-SESSION.json) without external URL parsing, while Diagnostic.message in public-dependencies/source/gen-lsp-types-0.11.0/src/generated/structures.rs and Message::String in public-dependencies/source/gen-lsp-types-0.11.0/src/generated/enum_ors.rs serialize plain JSON strings via #[serde(untagged)] for the specified 3.17 subset.",
      "D3 (Profiles): Reusing the exact dispatch of source/crates/mori/src/commands/check.rs::run (inspecting mori_lexer::first_token_text and mori_core::language to route moriarty/2 to mori_core::stage::load, other language-header inputs to mori_core::parse, and remaining inputs to mori_parser::parse followed by mori_checker::check) keeps mori lsp aligned with mori check per LSP-N02 and UNI-018, mapping MoriDiagnostic labels and mori_core::ir::Error optional offset fields (with explicit zero-width fallback per LSP-N03) under local-check-only capabilities."
    ],
    "limitations": [
      "Review covers ordinary protocol and library design fit only across the supplied static files and bound actualSourceFacts; Cargo dependency resolution, lockfile generation, and binary execution are not performed.",
      "Proposed session transcripts (PROPOSED-INPUT-SESSION.json and PROPOSED-EXPECTED-SESSION.json) are unrun hand-derived protocol predictions, and source/crates/mori/src/commands/lsp.rs::run remains a todo! stub.",
      "Hostile-input, resource, and security requirements remain outside the scope of this ordinary review."
    ],
    "reviewToken": "doctorwatson-rust-lsp-design-protocol-ordinary-20261018-a2-whole-review",
    "reviewedDesignSubjectSha256": "32bf1810058935515e05b4efcabd97b3356f5166ebd7e083979f665cec62c490",
    "selectedProfiles": "same-routing-as-current-Rust-check",
    "selectedTransport": "lsp-server-0.10.0-Message-read-write",
    "selectedTypes": "gen-lsp-types-0.11.0-no-features-3.17-subset",
    "summary": "Approve the proposed ordinary Rust LSP design choices: synchronous stdio framing via lsp-server Message::read and Message::write (D1), gen-lsp-types with no optional features emitting the 3.17 subset and plain String diagnostic messages (D2), and the same three-way profile routing as mori check (D3).",
    "verdict": "approve-proposed-design"
  }
]

```

## design-reviews/doctorwatson-rust-lsp-design-toolchain-ordinary-20261018-a2/WHOLE-SCHEMA-VALID-CANDIDATES.json (COMPLETE FILE)

```text
[
  {
    "actualSourceFacts": {
      "currentCargoLockVersions": {
        "crossbeam-channel": [],
        "crossbeam-utils": [
          "0.8.23"
        ],
        "gen-lsp-types": [],
        "log": [
          "0.4.34"
        ],
        "lsp-server": [],
        "lsp-types": [],
        "serde": [
          "1.0.229"
        ],
        "serde_derive": [
          "1.0.229"
        ],
        "serde_json": [
          "1.0.151"
        ]
      },
      "existingConsumers": {
        "routes": [
          "language first token + moriarty/2 -> mori_core::stage::load",
          "other language first token -> mori_core::parse (includes legacy validate)",
          "otherwise -> mori_parser::parse then mori_checker::check"
        ],
        "rustCheck": "source/crates/mori/src/commands/check.rs",
        "rustLsp": "source/crates/mori/src/commands/lsp.rs run -> todo!",
        "rustMcp": "source/crates/mori/src/commands/mcp.rs run -> todo!"
      },
      "literalTokenDeclaration": {
        "coreCaller": "source/crates/mori-core/src/parser.rs -> mori_lexer::lex",
        "file": "source/crates/mori-lexer/src/lib.rs",
        "name": "MAX_TOKENS",
        "value": 8192
      },
      "normalPredicate": "Actual mori lsp binary,11 input messages ->9 ordered complete semantic JSON messages, beta unknown-name UTF16 line2 chars25..33, full edit/close clear, ordinary Core1/Core2 open/close, shutdown/exit0 and empty stderr. Proposed hand-derived UNRUN expectation; not independent finance or execution evidence.",
      "normativeEarsPaths": [
        "source/openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md",
        "source/openspec/changes/consolidated-language-kernel/traceability.md"
      ],
      "ordinaryEars": {
        "ids": [
          "LSP-N01",
          "LSP-N02",
          "LSP-N03",
          "LSP-N04",
          "LSP-N05"
        ],
        "path": "EARS-PROPOSAL.md",
        "status": "Proposed child clauses, unadopted"
      },
      "originalInputManifestSha256": "a337f8a473c885c188957d1199fbe1a3a8ca669818e43251afa8383700e10028",
      "proposedDirectVersionsNotCurrentLock": {
        "lsp-server": "=0.10.0",
        "lsp-types alias package gen-lsp-types": "=0.11.0, default-features=false, no features"
      },
      "publishedManifestRangesNotResolvedVersions": {
        "gen-lsp-types0.11.0": {
          "optionalFeaturesNotProposed": [
            "url",
            "fluent-uri"
          ],
          "serde": "1.0.228 with derive",
          "serde_json": "1.0.150"
        },
        "lsp-server0.10.0": {
          "crossbeam-channel": "0.5.15",
          "log": "0.4.27",
          "serde": "1.0.219",
          "serde_derive": "1.0.219",
          "serde_json": "1.0.140"
        }
      },
      "repoCommit": "588d811dbbd9e88d22d3ebe90b0544672870caa0",
      "resolvedNewGraph": "UNKNOWN. Manifest ranges are not exact new selected identities. No claim that current log/serde/json pins must be downgraded or replaced.",
      "status": "NOT EXECUTED; NO Cargo resolution; Rust LSP/MCP not implemented in observed input",
      "typeProtocolQualification": "gen-lsp-types0.11.0 generated from3.18; proposed serialization emits only specified3.17 subset, Diagnostic.message plain String. Uri is String wrapper with no optional URI feature."
    },
    "filesRead": [
      "ACTUAL-SOURCE-FACTS.json",
      "PROPOSED-INPUT-SESSION.json",
      "PROPOSED-EXPECTED-SESSION.json",
      "EARS-PROPOSAL.md",
      "source/crates/mori/src/commands/lsp.rs",
      "source/crates/mori/src/commands/mcp.rs",
      "source/crates/mori/src/commands/check.rs",
      "source/crates/mori-parser/src/lib.rs",
      "source/crates/mori-checker/src/lib.rs",
      "source/crates/mori-core/src/parser.rs",
      "source/crates/mori-core/src/stage/mod.rs",
      "source/crates/mori-core/src/ir.rs",
      "source/crates/mori-lexer/src/lib.rs",
      "public-dependencies/source/lsp-server-0.10.0/src/msg.rs",
      "public-dependencies/source/lsp-server-0.10.0/src/stdio.rs",
      "public-dependencies/source/gen-lsp-types-0.11.0/src/generated/common.rs",
      "public-dependencies/source/gen-lsp-types-0.11.0/src/generated/enum_ors.rs",
      "public-dependencies/source/gen-lsp-types-0.11.0/src/generated/structures.rs",
      "source/openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md"
    ],
    "findings": [
      "D1 (selectedTransport = lsp-server-0.10.0-Message-read-write): Using Message::read and Message::write in public-dependencies/source/lsp-server-0.10.0/src/msg.rs over locked stdin/stdout inside source/crates/mori/src/commands/lsp.rs::run reuses standard Content-Length framing (read_msg_text / write_msg_text) and JSON-RPC 2.0 envelope serialization without spawning the three background OS threads (LspServerWriter, LspMessageDropper, LspServerReader) and bounded channels in public-dependencies/source/lsp-server-0.10.0/src/stdio.rs::stdio_transport or reimplementing manual framing.",
      "D2 (selectedTypes = gen-lsp-types-0.11.0-no-features-3.17-subset): Aliasing gen-lsp-types as lsp-types with default-features=false and no optional features (per actualSourceFacts) keeps Uri in public-dependencies/source/gen-lsp-types-0.11.0/src/generated/common.rs as an opaque String newtype (pub struct Uri(pub String)) that preserves client-supplied URIs such as untitled:* without URL parser dependencies or filesystem reads, while Diagnostic.message in public-dependencies/source/gen-lsp-types-0.11.0/src/generated/structures.rs and enum_ors.rs::Message (an #[serde(untagged)] enum) serializes Message::String as a plain JSON string compatible with the specified LSP 3.17 subset.",
      "D3 (selectedProfiles = same-routing-as-current-Rust-check): Routing in-memory document text through the same three-way dispatch as source/crates/mori/src/commands/check.rs::run\u2014mori_lexer::first_token_text and mori_core::language in source/crates/mori-core/src/parser.rs selecting mori_core::stage::load (source/crates/mori-core/src/stage/mod.rs) for moriarty/2, mori_core::parse for moriarty/1, and mori_parser::parse (source/crates/mori-parser/src/lib.rs) plus mori_checker::check (source/crates/mori-checker/src/lib.rs) for moriarty-beta/1\u2014maintains local-check parity with mori check, satisfies UNI-018 and LSP-N01..LSP-N05, and maps mori_core::ir::Error optional byte offsets to explicit zero-width UTF-16 ranges without inventing spans."
    ],
    "limitations": [
      "Rust LSP (source/crates/mori/src/commands/lsp.rs::run) and MCP (source/crates/mori/src/commands/mcp.rs::run) remain unimplemented todo! stubs in the observed repository state.",
      "No Cargo dependency resolution, lockfile generation, compilation, or test execution has been performed; actualSourceFacts.resolvedNewGraph remains UNKNOWN.",
      "PROPOSED-INPUT-SESSION.json and PROPOSED-EXPECTED-SESSION.json are unrun hand-derived protocol expectations rather than observed binary outputs.",
      "Hostile-input, resource-bound, and security evaluations (such as Content-Length allocation in read_msg_text) are outside the scope of this ordinary design review and remain with their designated owner."
    ],
    "reviewToken": "doctorwatson-rust-lsp-design-toolchain-ordinary-20261018-a2-whole-review",
    "reviewedDesignSubjectSha256": "32bf1810058935515e05b4efcabd97b3356f5166ebd7e083979f665cec62c490",
    "selectedProfiles": "same-routing-as-current-Rust-check",
    "selectedTransport": "lsp-server-0.10.0-Message-read-write",
    "selectedTypes": "gen-lsp-types-0.11.0-no-features-3.17-subset",
    "summary": "Approve the proposed ordinary Rust LSP design across all three finite choices: (D1) synchronous stdio loop using lsp-server Message::read and Message::write on locked stdin/stdout, (D2) gen-lsp-types aliased as lsp-types with default-features=false and no features emitting the specified 3.17 subset and plain String diagnostic messages, and (D3) identical three-profile routing (moriarty-beta/1, moriarty/1, moriarty/2) as existing mori check with explicit local-check-only metadata and zero-width fallback ranges for Core errors.",
    "toolAction": "Completing ordinary design review",
    "toolSummary": "Complete ordinary design review",
    "verdict": "approve-proposed-design"
  },
  {
    "actualSourceFacts": {
      "currentCargoLockVersions": {
        "crossbeam-channel": [],
        "crossbeam-utils": [
          "0.8.23"
        ],
        "gen-lsp-types": [],
        "log": [
          "0.4.34"
        ],
        "lsp-server": [],
        "lsp-types": [],
        "serde": [
          "1.0.229"
        ],
        "serde_derive": [
          "1.0.229"
        ],
        "serde_json": [
          "1.0.151"
        ]
      },
      "existingConsumers": {
        "routes": [
          "language first token + moriarty/2 -> mori_core::stage::load",
          "other language first token -> mori_core::parse (includes legacy validate)",
          "otherwise -> mori_parser::parse then mori_checker::check"
        ],
        "rustCheck": "source/crates/mori/src/commands/check.rs",
        "rustLsp": "source/crates/mori/src/commands/lsp.rs run -> todo!",
        "rustMcp": "source/crates/mori/src/commands/mcp.rs run -> todo!"
      },
      "literalTokenDeclaration": {
        "coreCaller": "source/crates/mori-core/src/parser.rs -> mori_lexer::lex",
        "file": "source/crates/mori-lexer/src/lib.rs",
        "name": "MAX_TOKENS",
        "value": 8192
      },
      "normalPredicate": "Actual mori lsp binary,11 input messages ->9 ordered complete semantic JSON messages, beta unknown-name UTF16 line2 chars25..33, full edit/close clear, ordinary Core1/Core2 open/close, shutdown/exit0 and empty stderr. Proposed hand-derived UNRUN expectation; not independent finance or execution evidence.",
      "normativeEarsPaths": [
        "source/openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md",
        "source/openspec/changes/consolidated-language-kernel/traceability.md"
      ],
      "ordinaryEars": {
        "ids": [
          "LSP-N01",
          "LSP-N02",
          "LSP-N03",
          "LSP-N04",
          "LSP-N05"
        ],
        "path": "EARS-PROPOSAL.md",
        "status": "Proposed child clauses, unadopted"
      },
      "originalInputManifestSha256": "a337f8a473c885c188957d1199fbe1a3a8ca669818e43251afa8383700e10028",
      "proposedDirectVersionsNotCurrentLock": {
        "lsp-server": "=0.10.0",
        "lsp-types alias package gen-lsp-types": "=0.11.0, default-features=false, no features"
      },
      "publishedManifestRangesNotResolvedVersions": {
        "gen-lsp-types0.11.0": {
          "optionalFeaturesNotProposed": [
            "url",
            "fluent-uri"
          ],
          "serde": "1.0.228 with derive",
          "serde_json": "1.0.150"
        },
        "lsp-server0.10.0": {
          "crossbeam-channel": "0.5.15",
          "log": "0.4.27",
          "serde": "1.0.219",
          "serde_derive": "1.0.219",
          "serde_json": "1.0.140"
        }
      },
      "repoCommit": "588d811dbbd9e88d22d3ebe90b0544672870caa0",
      "resolvedNewGraph": "UNKNOWN. Manifest ranges are not exact new selected identities. No claim that current log/serde/json pins must be downgraded or replaced.",
      "status": "NOT EXECUTED; NO Cargo resolution; Rust LSP/MCP not implemented in observed input",
      "typeProtocolQualification": "gen-lsp-types0.11.0 generated from3.18; proposed serialization emits only specified3.17 subset, Diagnostic.message plain String. Uri is String wrapper with no optional URI feature."
    },
    "filesRead": [
      "ACTUAL-SOURCE-FACTS.json",
      "PROPOSED-INPUT-SESSION.json",
      "PROPOSED-EXPECTED-SESSION.json",
      "EARS-PROPOSAL.md",
      "source/crates/mori/src/commands/lsp.rs",
      "source/crates/mori/src/commands/mcp.rs",
      "source/crates/mori/src/commands/check.rs",
      "source/crates/mori-parser/src/lib.rs",
      "source/crates/mori-checker/src/lib.rs",
      "source/crates/mori-core/src/parser.rs",
      "source/crates/mori-core/src/stage/mod.rs",
      "source/crates/mori-core/src/ir.rs",
      "source/crates/mori-lexer/src/lib.rs",
      "public-dependencies/source/lsp-server-0.10.0/src/msg.rs",
      "public-dependencies/source/lsp-server-0.10.0/src/stdio.rs",
      "public-dependencies/source/gen-lsp-types-0.11.0/src/generated/common.rs",
      "public-dependencies/source/gen-lsp-types-0.11.0/src/generated/enum_ors.rs",
      "public-dependencies/source/gen-lsp-types-0.11.0/src/generated/structures.rs",
      "source/openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md"
    ],
    "findings": [
      "D1 (selectedTransport = lsp-server-0.10.0-Message-read-write): Using Message::read and Message::write in public-dependencies/source/lsp-server-0.10.0/src/msg.rs over locked stdin/stdout inside source/crates/mori/src/commands/lsp.rs::run reuses standard Content-Length framing (read_msg_text / write_msg_text) and JSON-RPC 2.0 envelope serialization without spawning the three background OS threads (LspServerWriter, LspMessageDropper, LspServerReader) and bounded channels in public-dependencies/source/lsp-server-0.10.0/src/stdio.rs::stdio_transport or reimplementing manual framing.",
      "D2 (selectedTypes = gen-lsp-types-0.11.0-no-features-3.17-subset): Aliasing gen-lsp-types as lsp-types with default-features=false and no optional features (per actualSourceFacts) keeps Uri in public-dependencies/source/gen-lsp-types-0.11.0/src/generated/common.rs as an opaque String newtype (pub struct Uri(pub String)) that preserves client-supplied URIs such as untitled:* without URL parser dependencies or filesystem reads, while Diagnostic.message in public-dependencies/source/gen-lsp-types-0.11.0/src/generated/structures.rs and enum_ors.rs::Message (an #[serde(untagged)] enum) serializes Message::String as a plain JSON string compatible with the specified LSP 3.17 subset.",
      "D3 (selectedProfiles = same-routing-as-current-Rust-check): Routing in-memory document text through the same three-way dispatch as source/crates/mori/src/commands/check.rs::run\u2014mori_lexer::first_token_text and mori_core::language in source/crates/mori-core/src/parser.rs selecting mori_core::stage::load (source/crates/mori-core/src/stage/mod.rs) for moriarty/2, mori_core::parse for moriarty/1, and mori_parser::parse (source/crates/mori-parser/src/lib.rs) plus mori_checker::check (source/crates/mori-checker/src/lib.rs) for moriarty-beta/1\u2014maintains local-check parity with mori check, satisfies UNI-018 and LSP-N01..LSP-N05, and maps mori_core::ir::Error optional byte offsets to explicit zero-width UTF-16 ranges without inventing spans."
    ],
    "limitations": [
      "Rust LSP (source/crates/mori/src/commands/lsp.rs::run) and MCP (source/crates/mori/src/commands/mcp.rs::run) remain unimplemented todo! stubs in the observed repository state.",
      "No Cargo dependency resolution, lockfile generation, compilation, or test execution has been performed; actualSourceFacts.resolvedNewGraph remains UNKNOWN.",
      "PROPOSED-INPUT-SESSION.json and PROPOSED-EXPECTED-SESSION.json are unrun hand-derived protocol expectations rather than observed binary outputs.",
      "Hostile-input, resource-bound, and security evaluations (such as Content-Length allocation in read_msg_text) are outside the scope of this ordinary design review and remain with their designated owner."
    ],
    "reviewToken": "doctorwatson-rust-lsp-design-toolchain-ordinary-20261018-a2-whole-review",
    "reviewedDesignSubjectSha256": "32bf1810058935515e05b4efcabd97b3356f5166ebd7e083979f665cec62c490",
    "selectedProfiles": "same-routing-as-current-Rust-check",
    "selectedTransport": "lsp-server-0.10.0-Message-read-write",
    "selectedTypes": "gen-lsp-types-0.11.0-no-features-3.17-subset",
    "summary": "Approve the proposed ordinary Rust LSP design across all three finite choices: (D1) synchronous stdio loop using lsp-server Message::read and Message::write on locked stdin/stdout, (D2) gen-lsp-types aliased as lsp-types with default-features=false and no features emitting the specified 3.17 subset and plain String diagnostic messages, and (D3) identical three-profile routing (moriarty-beta/1, moriarty/1, moriarty/2) as existing mori check with explicit local-check-only metadata and zero-width fallback ranges for Core errors.",
    "verdict": "approve-proposed-design"
  }
]

```

## design/DESIGN-ADOPTION.json (COMPLETE FILE)

```text
{
  "recordedAt": "2026-10-18T11:05:02.621516+00:00",
  "status": "ADOPT-D1-D2-D3-FOR-UNAPPLIED-SOURCE-AUTHORING",
  "contexts": [
    {
      "recordedAt": "2026-10-18T11:05:02.621195+00:00",
      "status": "ADOPT-THREE-ORDINARY-DESIGN-CHOICES-ONLY",
      "job": "doctorwatson-rust-lsp-design-protocol-ordinary-20261018-a2",
      "wholeSha256": "11b5ac8b92b068607dcff992331b515d361f7a54511c4d9157c620dd94417c66",
      "wholeFormsRetained": 2,
      "substantiveFieldsExactlyEqual": true,
      "votesCounted": 1,
      "canonicalProviderIdentity": false,
      "requestedModel": "gemini-4-argon-high",
      "choiceFields": {
        "selectedTransport": "lsp-server-0.10.0-Message-read-write",
        "selectedTypes": "gen-lsp-types-0.11.0-no-features-3.17-subset",
        "selectedProfiles": "same-routing-as-current-Rust-check"
      },
      "qualification": "No implementation/lock-resolution/resource/security/hostile-input/execution approval. lsp-server::Message read allocation still owner-unassessed. New dependencies proposed exact versions only, no resolved graph.",
      "sourceNotationCorrection": "Reviews refer to defining mori_core::ir::Error; Rust implementation must import public mori_core::Error, not private module.",
      "limitations": [
        "Review covers ordinary protocol and library design fit only across the supplied static files and bound actualSourceFacts; Cargo dependency resolution, lockfile generation, and binary execution are not performed.",
        "Proposed session transcripts (PROPOSED-INPUT-SESSION.json and PROPOSED-EXPECTED-SESSION.json) are unrun hand-derived protocol predictions, and source/crates/mori/src/commands/lsp.rs::run remains a todo! stub.",
        "Hostile-input, resource, and security requirements remain outside the scope of this ordinary review."
      ],
      "originalA1RejectedObjectsPreserved": true
    },
    {
      "recordedAt": "2026-10-18T11:05:02.621414+00:00",
      "status": "ADOPT-THREE-ORDINARY-DESIGN-CHOICES-ONLY",
      "job": "doctorwatson-rust-lsp-design-toolchain-ordinary-20261018-a2",
      "wholeSha256": "6bfaf0c40a49f0f124827ca226f204d39ee50771b83343a22fddfd20560fc6d4",
      "wholeFormsRetained": 2,
      "substantiveFieldsExactlyEqual": true,
      "votesCounted": 1,
      "canonicalProviderIdentity": false,
      "requestedModel": "gemini-4-argon-high",
      "choiceFields": {
        "selectedTransport": "lsp-server-0.10.0-Message-read-write",
        "selectedTypes": "gen-lsp-types-0.11.0-no-features-3.17-subset",
        "selectedProfiles": "same-routing-as-current-Rust-check"
      },
      "qualification": "No implementation/lock-resolution/resource/security/hostile-input/execution approval. lsp-server::Message read allocation still owner-unassessed. New dependencies proposed exact versions only, no resolved graph.",
      "sourceNotationCorrection": "Reviews refer to defining mori_core::ir::Error; Rust implementation must import public mori_core::Error, not private module.",
      "limitations": [
        "Rust LSP (source/crates/mori/src/commands/lsp.rs::run) and MCP (source/crates/mori/src/commands/mcp.rs::run) remain unimplemented todo! stubs in the observed repository state.",
        "No Cargo dependency resolution, lockfile generation, compilation, or test execution has been performed; actualSourceFacts.resolvedNewGraph remains UNKNOWN.",
        "PROPOSED-INPUT-SESSION.json and PROPOSED-EXPECTED-SESSION.json are unrun hand-derived protocol expectations rather than observed binary outputs.",
        "Hostile-input, resource-bound, and security evaluations (such as Content-Length allocation in read_msg_text) are outside the scope of this ordinary design review and remain with their designated owner."
      ],
      "originalA1RejectedObjectsPreserved": true
    }
  ],
  "agreeingNonAuthorContexts": 2,
  "productPermissionRuleAdded": false,
  "executionNotGranted": true,
  "sourceBase": "588d811 static packet; actual main7396 evolved through unrelated Core2/init/Source6; author must recheck full actual source and retain separate identities.",
  "requiredNext": "Unapplied complete LSP code/test candidate, then actual dependency identities and full candidate owner/resource review before any selected protocol test. No speculative lock or fabricated output."
}

```

## dependency-preparations/dependency-resolution-vendor-a2/RESULT.json (COMPLETE FILE)

```text
{
  "at": "2026-10-18T11:37:34.666958+00:00",
  "exitCode": 0,
  "wallSeconds": 0.19517772900144337,
  "lockBefore": "f47f1f3e04111a50134656dce0e9d278e19a6c162aaf06cdd5f79fceead68ff3",
  "lockAfter": "7abb05e03815d2ec56e4419f00a1b3c360c33ed8e2ee94acbf1404a1dd704d08",
  "lockChanged": true,
  "sessionMembers": [],
  "sourceSetPackages": 428,
  "scope": "Cargo metadata graph via explicit local replacement only; not a fresh network registry resolution or compilation/protocol/product result.",
  "resolvedNodes": 456,
  "selectedNewPackages": [
    {
      "name": "crossbeam-channel",
      "version": "0.5.17",
      "source": "registry+https://github.com/rust-lang/crates.io-index"
    },
    {
      "name": "gen-lsp-types",
      "version": "0.11.0",
      "source": "registry+https://github.com/rust-lang/crates.io-index"
    },
    {
      "name": "lsp-server",
      "version": "0.10.0",
      "source": "registry+https://github.com/rust-lang/crates.io-index"
    }
  ],
  "removedPackageIdentities": [],
  "addedPackageIdentities": [
    [
      "gen-lsp-types",
      "0.11.0",
      "registry+https://github.com/rust-lang/crates.io-index"
    ],
    [
      "crossbeam-channel",
      "0.5.17",
      "registry+https://github.com/rust-lang/crates.io-index"
    ],
    [
      "lsp-server",
      "0.10.0",
      "registry+https://github.com/rust-lang/crates.io-index"
    ]
  ],
  "modifiedExistingPackageEntries": [
    [
      "mori",
      "0.1.0",
      null
    ]
  ],
  "ownedTempRemoved": true
}

```

## dependency-preparations/dependency-resolution-offline-a1/RESULT.json (COMPLETE FILE)

```text
{
  "at": "2026-10-18T11:33:02.002520+00:00",
  "exitCode": 101,
  "wallSeconds": 0.06421369499730645,
  "lockAfter": "f47f1f3e04111a50134656dce0e9d278e19a6c162aaf06cdd5f79fceead68ff3",
  "lockChanged": false,
  "sessionMembers": [],
  "ownedTempRemoved": true,
  "scope": "Metadata resolution only; no build/server or product execution."
}

```

## dependency-preparations/dependency-resolution-vendor-a1/RESULT.json (COMPLETE FILE)

```text
{
  "at": "2026-10-18T11:34:40.746218+00:00",
  "exitCode": 101,
  "wallSeconds": 0.05363027499697637,
  "lockBefore": "f47f1f3e04111a50134656dce0e9d278e19a6c162aaf06cdd5f79fceead68ff3",
  "lockAfter": "f47f1f3e04111a50134656dce0e9d278e19a6c162aaf06cdd5f79fceead68ff3",
  "lockChanged": false,
  "sessionMembers": [],
  "ownedTempRemoved": true,
  "scope": "Metadata graph only via explicit local source replacement; no compilation/protocol/SDK/product result."
}

```

## dependency-preparations/dependency-metadata-a1/RESULT.json (COMPLETE FILE)

```text
{
  "finishedAt": "2026-10-18T11:24:36.554512+00:00",
  "exitCode": 0,
  "wallSeconds": 0.014765687003091443,
  "CargoLockAfterSha256": "f47f1f3e04111a50134656dce0e9d278e19a6c162aaf06cdd5f79fceead68ff3",
  "lockChanged": false,
  "ownedSession": 637224,
  "remainingMembers": [],
  "scope": "Metadata only, not compilation/protocol/product verification."
}

```
