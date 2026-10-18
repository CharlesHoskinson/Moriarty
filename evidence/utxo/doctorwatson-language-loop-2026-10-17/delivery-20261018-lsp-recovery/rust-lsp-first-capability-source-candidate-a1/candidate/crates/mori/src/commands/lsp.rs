use std::collections::HashMap;
use std::io::{Write, stdin, stdout};
use std::process::ExitCode;

use lsp_server::{ErrorCode, Message, Notification, Request, Response};
use lsp_types::{
    Diagnostic, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, InitializeParams, InitializeResult, PositionEncodingKind,
    PublishDiagnosticsParams, ServerCapabilities, ServerInfo, TextDocumentContentChangeEvent,
    TextDocumentSyncKind, TextDocumentSyncOptions, Uri,
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
            Phase::New | Phase::Initializing => Some((
                ErrorCode::ServerNotInitialized,
                "Server not initialized.",
            )),
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
                let found = diagnostics::check(
                    &document.uri,
                    &document.text,
                    self.related_information,
                );
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
