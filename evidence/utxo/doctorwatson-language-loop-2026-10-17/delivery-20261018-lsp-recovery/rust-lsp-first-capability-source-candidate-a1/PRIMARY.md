# Complete unapplied Rust LSP source candidate

Confirmed remote-main source basis and all limits are in BEFORE-PINS.json and SOURCE-NOTE.md. Initial stale local870 inspection is retained separately and excluded from current source basis. Full source and frozen normal data below are not a build/test result.

===== SOURCE-NOTE.md COMPLETE =====
# Complete unapplied first Rust LSP source candidate

The candidate is based on confirmed GitHub main7396d7d316ab72957bca9709483f4638daa0d5cb, not stale local refs/heads/main. Root explicitly authorized read-only git ls-remote origin refs/heads/main from the docs worktree; its actual result confirmed7396. The initial mistaken870f local-ref inspection is preserved separately under stale-local-870f-inspection with its correction record; none of those stale files defines the candidate basis. BEFORE-PINS.json pins64 exact current files and compares them with the adopted588 design snapshot. Actual LSP/check/parser/checker/Core error and stage entry APIs remain byte-identical; existing Source6 feature/manifest/guideline changes are retained.

Root adopted D1/D2/D3 from two independently authored ordinary a2 contexts. Full adoption, reviews, roots and their no-execution qualifications are physical. Original failed a1 design reviews remain preserved in the earlier packets and remain ineligible. This source candidate is new implementation, not implementation approval by those design votes.

The complete7-path patch proposes exact workspace dependencies, preserves default native and source6-expansion feature declarations and every old dependency/test target, replaces the existing public lsp::run stub, adds its direct diagnostic adapter, and tests the actual Rust CLI process using the frozen normal transcript. MCP remains untouched and unimplemented. No current check/parse/compiler semantics are changed and no new public permission gate is introduced.

The test was written before production. Its exact11-message input and9-message expected objects predate implementation and are copied byte-identically. They are author-derived normal expectations, not independently custodial financial/work evidence. The test's independent frame reader observes the actual child process; it does not substitute a helper or mock server. It compares all ordered semantic JSON outputs, exit0 and empty stderr; exact serialized JSON member order/byte rendering has no separate independent golden. RED/GREEN are NOT RUN under explicit root source-only instructions; no TDD pass or observed failure is claimed.

D1 uses public lsp_server::Message::read/write on inherited locked stdin/stdout; it does not call Connection::stdio or socket APIs. D2 uses gen-lsp-types0.11.0 alias with no optional URI features and emits the specified3.17 subset including plain-string diagnostic messages. D3 mirrors the current Rust check branch: lexer first-token/profile detection, legacy parser+checker, Core1 parse with validation, Core2 load with admission/plan/cost/digests. No simulate, compile, SDK/proof/model/ledger/wallet consumer is selected by the new server. This is a static proposed call graph, not a full transitive no-I/O/security claim.

The adapter retains codes/message/help, primary/fallback ranges and client-supported related information. It reads gen0.11's actual nested diagnostics_capabilities.related_information and VersionedTextDocumentIdentifier.text_document_identifier.uri fields, corrected during static API inspection before freeze. It imports public mori_core::Error. UTF16 conversion handles source byte boundaries and LF/CRLF/CR. Full change events are applied in supplied order (last whole-document text is the closing version); incremental events are outside the advertised Full capability. Diagnostic version/related information are conditioned on the corresponding client capabilities. These are ordinary protocol implementation details, not security judgments.

Package versions are exact proposals verified against public registry metadata/archives in the design packet, not entries in current Cargo.lock. Cargo.lock is copied only as unchanged before evidence; no candidate lock or generated lock entry is supplied. New resolved dependency identities/features/build/proc-macro graph remain UNKNOWN until root's separate authorized resolution/inspection. Default normal builds retain existing native dependencies; the proposed selected protocol test disables defaults explicitly. No Cargo, npm, rustc, rustfmt, Clippy, Rust/Moriarty parser execution, product server, SDK, model, VM/proof/key or wallet process ran. No new package download occurred during source authoring; the one root-authorized public ls-remote is recorded. Shared worktrees/Git were not written.

Important remaining boundaries: Message::read framing allocation/error/log behavior and retained-document/header/body/output/protocol exposure need designated-owner assessment before any actual protocol test. The new application imposes no newly adopted resource cap or allocation contract. Malformed/unsupported notifications, partial edits and unrecognized messages are not covered by the normal oracle; the code's returns/retained state are visible for owner review, not certified. EOF/operational I/O handling, parent-process lifecycle, cancellation, slow peers, multiple normal Full changes and CRLF/CR have no actual result evidence. Only the frozen normal11-to9 transcript is the proposed acceptance predicate. No security/hostile tests or judgment are authored here. No full LSP/editor-platform/DevEx release, U1/U2/U7, M10, MC08/SP02/SP12, G01–G24 or original language/proof/ledger exit is closed. Existing failed contexts, native allocations, caps and C4/k17 remain unchanged.

===== CANDIDATE-FILES.json COMPLETE =====
{
  "status": "Complete unapplied normal source/test proposal, NOT compiled or executed",
  "baseCommit": "7396d7d316ab72957bca9709483f4638daa0d5cb",
  "baseTree": "7e7a6fc1980b67c06ba9ceda78a6f11eaf682227",
  "changes": [
    {
      "path": "Cargo.toml",
      "oldSha256": "ffa6cd7e538f3162a722dd657aea0e68d80c6ace3eddfcab0810ccbc03d37404",
      "newSha256": "8d38687057ca8cbc6511ce9994ba0c3051b559ef2df768e521800d07af01320f",
      "bytes": 3938,
      "lines": 76
    },
    {
      "path": "crates/mori/Cargo.toml",
      "oldSha256": "317bf14a6f3b4b26f223ac9dc93823e7d185333a161bd3ad3f0cf1e0fb6dd05f",
      "newSha256": "35a35a661178dbff712795040f4c3eb0833a44622052cd438189222a27522642",
      "bytes": 2880,
      "lines": 114
    },
    {
      "path": "crates/mori/src/commands/lsp.rs",
      "oldSha256": "8187ef893f48b6a5a54a5d48e61e120edde48ddb57b64d21b46100970d8df478",
      "newSha256": "1f9dcb182be9f8cbfe043f9127edc2e1f998fc0a4ac2a641ec4ae66943ad1b78",
      "bytes": 9655,
      "lines": 273
    },
    {
      "path": "crates/mori/src/commands/lsp/diagnostics.rs",
      "oldSha256": null,
      "newSha256": "dbac1e66f7087e1686b4f73e65dd34a8576885fc6324d975613aa6c50c6537f8",
      "bytes": 4612,
      "lines": 136
    },
    {
      "path": "crates/mori/tests/fixtures/lsp/expected-session.json",
      "oldSha256": null,
      "newSha256": "72151b6233fcb02d473f02730a19d93b51cfe42c47d9fe9151566c7e2939113c",
      "bytes": 2993,
      "lines": 117
    },
    {
      "path": "crates/mori/tests/fixtures/lsp/input-session.json",
      "oldSha256": null,
      "newSha256": "c89a0d8a38e197ea9cdee58e14b36f9a032df50a32d299043b32b20c5a918392",
      "bytes": 2402,
      "lines": 117
    },
    {
      "path": "crates/mori/tests/lsp.rs",
      "oldSha256": null,
      "newSha256": "de8a913f78e8b39d5016988998e4a4263d22029038a92b5ad23e0fb8514981eb",
      "bytes": 2255,
      "lines": 61
    }
  ],
  "patchSha256": "ca063d08f23533fd7a3d2d1d63e1fbcda9c794e9812b75933b4885d2c9e5c372",
  "CargoLockChanged": false,
  "beforeCargoLockSha256": "f47f1f3e04111a50134656dce0e9d278e19a6c162aaf06cdd5f79fceead68ff3",
  "defaultNativeAndSource6FeaturesPreserved": true,
  "existingDependencyDeclarationsAndExplicitTestTargetsPreserved": true,
  "normalOracleByteIdentityPreserved": true
}

===== UNRUN-CHECKS.json COMPLETE =====
{
  "status": "No product/build/parser/formatter/test execution",
  "sourcePredicate": "Actual public mori lsp binary consumes the frozen11-message stdio session and emits exactly9 complete ordered framed semantic JSON messages; normal beta error UTF16 line2 chars25..33, full edit clears, close clears, ordinary Core1/Core2 documents empty diagnostics, shutdown/exit0 and stderr empty.",
  "proposedSelector": [
    "cargo",
    "test",
    "--manifest-path",
    "<actual-repo>/Cargo.toml",
    "--frozen",
    "-j1",
    "-p",
    "mori",
    "--no-default-features",
    "--test",
    "lsp",
    "full_sync_diagnostics_follow_public_checker_and_clear_after_edit",
    "--",
    "--exact",
    "--test-threads=1"
  ],
  "selectorStatus": "NOT RUN; current lock lacks proposed packages, so root must separately resolve/review actual dependency identities and obtain complete-candidate/owner/resource review first.",
  "otherRequiredButUnrun": [
    "Appropriate Cargo compilation/check of actual selected source graph",
    "rustfmt source formatting check",
    "Scoped Clippy -D warnings",
    "Fresh full-candidate ordinary and designated-owner reviews",
    "Actual selected normal protocol test with launch-time source/tool/env/resource/cleanup conditions"
  ],
  "staticOnlyVerification": [
    "Parsed proposed TOML as text using Python tomllib, not Cargo",
    "Compared current before source hashes and native/source6 features/existing dependency and explicit test declarations",
    "Compared proposed normal fixture bytes against pre-implementation frozen copies",
    "Read actual downloaded Rust dependency API definitions and corrected two nested generated field accesses",
    "Generated complete unified diff with Python difflib; no Git apply/parser execution or Git mutations"
  ],
  "executionAuthority": false
}

===== CALL-GRAPH-AND-EXPOSURE.json COMPLETE =====
{
  "status": "Mechanical proposed selected call graph and I/O references, NOT execution/security/resource classification",
  "entry": [
    "crates/mori/src/main.rs main",
    "commands::run -> Command::Lsp -> commands/lsp.rs run",
    "stdin.lock/stdout.lock -> lsp_server::Message::read -> Server::dispatch -> Message::write"
  ],
  "requests": [
    "initialize -> gen InitializeParams serde deserialization -> client capability fields -> InitializeResult/ServerCapabilities/ServerInfo",
    "shutdown -> phase transition + in-memory document clear -> response result null",
    "other requests -> ErrorCode response; no outgoing server requests"
  ],
  "notifications": [
    "initialized -> running phase",
    "didOpen -> typed text document + in-memory URI map + diagnostics::check + PublishDiagnosticsParams",
    "didChange -> typed whole-document changes in order, newer version -> diagnostics::check -> PublishDiagnosticsParams",
    "didClose -> remove in-memory document + empty diagnostic notification without version",
    "exit -> return success only after shutdown; EOF returns failure"
  ],
  "analysisRoutes": [
    "mori_lexer::first_token_text; mori_core::language",
    "legacy -> mori_parser::parse -> mori_checker::check -> MoriDiagnostic code/labels/help/severity",
    "Core1 -> public mori_core::parse -> parser::parse_profile -> legacy validate -> public mori_core::Error",
    "Core2 -> mori_core::stage::load -> parse_v2/profile/validate/plan/cost/digest -> public mori_core::Error"
  ],
  "outputs": [
    "Message::write produces Content-Length UTF8 JSON frames and flushes stdout",
    "Existing core_io::operation wraps operational I/O errors; main/Shell reports those on inherited stderr",
    "Library framing parse errors and debug log calls are retained source exposure for owner inspection"
  ],
  "selectedTest": [
    "crates/mori/tests/lsp.rs spawns env!(CARGO_BIN_EXE_mori) --color never lsp with3stdio pipes",
    "Writes frozen11-message sequence with byte Content-Length, closes stdin, waits and captures stdout/stderr",
    "Independent test-only frame reader compares exactly9 complete semantic values, exit0, empty stderr"
  ],
  "ownerInputs": [
    "public-dependencies/source/lsp-server-0.10.0/src/msg.rs Message::read/read_msg_text/write_msg_text and message serde",
    "public-dependencies/source/gen-lsp-types-0.11.0 generated parameter/URI/diagnostic structs and serde",
    "New application document storage, protocol state, notification early returns and lifecycle in candidate/crates/mori/src/commands/lsp.rs",
    "Actual resolved graph/feature/build/proc-macro source unknown until separate root resolution"
  ],
  "notSelectedByNewEntry": [
    "URI-to-file/workspace/watch/socket APIs",
    "mori_core::simulate or replay",
    "native compiler/SDK/VM/proof/key/ledger/wallet consumers"
  ],
  "qualification": "Not-selected list describes direct implemented call selection, not a certificate of all dependency or host effects. No resource cap/command admission or safety judgment supplied."
}

===== STATIC-REVIEW.json COMPLETE =====
{
  "status": "Bounded source-only API/ownership/import/manifest/fixture review; not compiler/formatter or test success",
  "resolvedFindings": [
    {
      "source": "gen-lsp-types0.11 PublishDiagnosticsClientCapabilities",
      "finding": "related_information is inside flattened diagnostics_capabilities, not directly on publish_diagnostics",
      "candidate": "request reads capabilities.diagnostics_capabilities.related_information"
    },
    {
      "source": "gen-lsp-types0.11 VersionedTextDocumentIdentifier",
      "finding": "URI is inside flattened text_document_identifier, not directly beside version",
      "candidate": "didChange reads params.text_document.text_document_identifier.uri"
    },
    {
      "source": "gen-lsp-types0.11 TextDocumentContentChangeEvent",
      "finding": "WholeDocument variant uses generated full name; list entries describe ordered whole-document replacements",
      "candidate": "matches TextDocumentContentChangeWholeDocument and keeps final supplied Full text before one analysis/publication"
    }
  ],
  "remainingConcreteSourceCompileFindings": [],
  "qualification": "No compiler run. Empty remaining list is bounded manual-source review only and cannot certify compilation or warning cleanliness. Full independent source review/actual dependency resolution remain root steps.",
  "inspectedPublicApis": [
    "lsp_server Message/Request/Response/Notification and ErrorCode",
    "gen InitializeParams/Result, ServerCapabilities/Info, nested client diagnostic capabilities, Uri",
    "gen DidOpen/Change/Close and nested VersionedTextDocumentIdentifier; Full content change variant",
    "gen Position/Range/Diagnostic/Code/Message/Severity/RelatedInformation/Location constructors and serde fields",
    "current MoriDiagnostic traits/labels and public mori_core::Error; unchanged legacy/Core1/Core2 APIs"
  ],
  "noBlanketAllowOrPathMountAdded": true,
  "noProductExecutionOrLockGeneration": true
}

===== candidate/Cargo.toml COMPLETE =====
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

===== candidate/crates/mori/Cargo.toml COMPLETE =====
[package]
name = "mori"
version = "0.1.0"
description = "Command-line interface for the Moriarty toolchain"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
repository.workspace = true

[features]
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

===== candidate/crates/mori/src/commands/lsp.rs COMPLETE =====
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

===== candidate/crates/mori/src/commands/lsp/diagnostics.rs COMPLETE =====
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

===== candidate/crates/mori/tests/fixtures/lsp/expected-session.json COMPLETE =====
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

===== candidate/crates/mori/tests/fixtures/lsp/input-session.json COMPLETE =====
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

===== candidate/crates/mori/tests/lsp.rs COMPLETE =====
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
